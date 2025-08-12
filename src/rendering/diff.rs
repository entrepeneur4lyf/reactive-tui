use crate::error::Result;
use crate::layout::Layout;
use crate::rendering::{display_width, Renderer};

fn build_rows_from_grid(grid: &crate::rendering::target::CellGrid) -> Vec<Vec<u8>> {
  let mut rows: Vec<Vec<u8>> = Vec::with_capacity(grid.rows.len());
  for (y, segments) in grid.rows.iter().enumerate() {
    if segments.is_empty() {
      rows.push(Vec::new());
      continue;
    }
    let mut segs = segments.clone();
    segs.sort_by_key(|c| c.x);

    // Coalesce adjacent segments with identical styles and contiguous columns
    let mut merged: Vec<(u16, crate::rendering::RenderStyle, String)> = Vec::new();
    for cell in segs {
      if let Some((ref mut last_x, ref mut last_style, ref mut last_text)) = merged.last_mut() {
        let last_w = display_width(last_text) as u16;
        let expected_next_x = *last_x + last_w;
        if *last_style == cell.style && cell.x == expected_next_x {
          last_text.push_str(&cell.ch);
          continue;
        }
      }
      merged.push((cell.x, cell.style.clone(), cell.ch.clone()));
    }

    // Build minimal ANSI line: MoveTo + style changes only when needed
    let mut line = String::new();
    let mut cur_style = crate::rendering::RenderStyle::default();
    for (x, style, text) in merged {
      line.push_str(&format!("\u{1b}[{};{}H", y + 1, x));
      if style != cur_style {
        // Apply only differing attributes; simplified: we only toggle text attributes here.
        // Colors already come from authoritative offscreen bytes; we avoid mapping Color enums.
        // In a future pass, map RenderStyle -> ANSI exactly if needed.
        // (No-op for color diffs here to keep correctness across terminals.)
        if style.bold && !cur_style.bold {
          line.push_str("\u{1b}[1m");
        }
        if !style.bold && cur_style.bold {
          line.push_str("\u{1b}[22m");
        }
        if style.italic && !cur_style.italic {
          line.push_str("\u{1b}[3m");
        }
        if !style.italic && cur_style.italic {
          line.push_str("\u{1b}[23m");
        }
        if style.underline && !cur_style.underline {
          line.push_str("\u{1b}[4m");
        }
        if !style.underline && cur_style.underline {
          line.push_str("\u{1b}[24m");
        }
        cur_style = style;
      }
      line.push_str(&text);
    }

    rows.push(line.into_bytes());
  }
  rows
}

impl Renderer {
  pub async fn render_diff(&mut self, layout: &Layout) -> Result<Vec<u8>> {
    // Force a full repaint on the first diff frame or interval
    let need_full_repaint = self
      .last_diff_rows
      .as_ref()
      .map(|rows| rows.is_empty())
      .unwrap_or(true)
      || self
        .diff_full_repaint_interval
        .map(|n| self.diff_frames_since_full >= n)
        .unwrap_or(false);

    if need_full_repaint {
      let bytes = self.render(layout).await?;
      let rows = self.bytes_to_rows(&bytes);
      self.last_diff_rows = Some(rows);
      // If we have a raster grid from a previous offscreen render, prefer diffing on rows directly
      // Note: Stage 3 partial – future step will populate this in render_offscreen via GridTarget

      self.diff_frames_since_full = 0;
      return Ok(bytes);
    }

    // Render offscreen to get current frame without Clear/Hide/Show
    let current_bytes = if self.diff_mode_enabled {
      self.render_offscreen(layout).await?
    } else {
      self.render(layout).await?
    };
    let emit_rows = self.bytes_to_rows(&current_bytes);

    // Prefer building rows from raster grid for diff comparison when diff mode is enabled,
    // while using emit_rows as the authoritative bytes to send (preserves styles)
    let current_rows_for_diff = if self.diff_mode_enabled {
      if let Some(grid) = &self.grid_for_diff {
        build_rows_from_grid(grid)
      } else {
        emit_rows.clone()
      }
    } else {
      emit_rows.clone()
    };

    // Diff with previous
    let mut output = Vec::new();
    let prev_rows = self.last_diff_rows.as_ref().unwrap();
    let max_rows = prev_rows.len().max(current_rows_for_diff.len());

    for row in 0..max_rows {
      let prev = prev_rows.get(row).map(|v| v.as_slice()).unwrap_or(&[]);
      let curr = current_rows_for_diff
        .get(row)
        .map(|v| v.as_slice())
        .unwrap_or(&[]);
      if prev != curr {
        // Move cursor to row start and print entire row content to ensure artifacts are cleared
        output.extend_from_slice(format!("\u{1b}[{};{}H", row + 1, 1).as_bytes());
        // Emit authoritative row bytes from emit_rows (not the grid), so styles are preserved
        let authoritative = emit_rows.get(row).map(|v| v.as_slice()).unwrap_or(&[]);
        output.extend_from_slice(authoritative);

        // If the new row is shorter (by display width) than the previous, pad with spaces
        let prev_s = String::from_utf8_lossy(prev);
        let curr_s = String::from_utf8_lossy(authoritative);
        let term_w = self.width as usize;
        let prev_w = display_width(&prev_s).min(term_w);
        let curr_w = display_width(&curr_s).min(term_w);
        if curr_w < prev_w {
          let pad = " ".repeat(prev_w - curr_w);
          output.extend_from_slice(pad.as_bytes());
        }

        // ANSI reset at end of row to prevent style bleed
        output.extend_from_slice("\u{1b}[0m".as_bytes());
      }
    }

    // Update baseline and frame counter
    self.last_diff_rows = Some(current_rows_for_diff);
    self.diff_frames_since_full = self.diff_frames_since_full.saturating_add(1);

    Ok(output)
  }

  /// Convert a full-frame ANSI byte buffer into per-line rows by tracking cursor moves (CUP).
  fn bytes_to_rows(&self, bytes: &[u8]) -> Vec<Vec<u8>> {
    let mut rows: Vec<Vec<u8>> = Vec::new();
    let mut i = 0;
    let len = bytes.len();
    let mut current_row: usize = 0;

    let ensure_row = |rows: &mut Vec<Vec<u8>>, idx: usize| {
      if rows.len() <= idx {
        rows.resize_with(idx + 1, Vec::new);
      }
    };

    while i < len {
      if bytes[i] == 0x1B && i + 1 < len && bytes[i + 1] == b'[' {
        // Parse CSI sequence: ESC [ params letter
        let start = i;
        i += 2; // skip ESC[
        let mut params: Vec<usize> = Vec::new();
        let mut num: usize = 0;

        let mut has_num = false;
        let mut term: u8 = 0;
        while i < len {
          let b = bytes[i];
          if (b as char).is_ascii_digit() {
            num = num.saturating_mul(10).saturating_add((b - b'0') as usize);
            has_num = true;
          } else if b == b';' {
            if has_num {
              params.push(num);
              num = 0;
              has_num = false;
            }
          } else if (b as char).is_ascii_alphabetic() {
            if has_num {
              params.push(num);
            }
            term = b;
            i += 1; // consume terminator
            break;
          }
          i += 1;
        }
        if term == b'H' || term == b'f' {
          // CUP: row;col (1-based)
          let row1 = params.first().cloned().unwrap_or(1);
          current_row = row1.saturating_sub(1);
          ensure_row(&mut rows, current_row);
        } else {
          // Keep other CSI sequences in content of current row
          ensure_row(&mut rows, current_row);
          rows[current_row].extend_from_slice(&bytes[start..i]);
        }
      } else {
        ensure_row(&mut rows, current_row);
        rows[current_row].push(bytes[i]);
        i += 1;
      }
    }
    rows
  }
}
