use crate::error::Result;
use crate::layout::Layout;
use crate::rendering::{display_width, Renderer};

/// Simple per-line diff flush with defensive full repaint interval.
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
      self.diff_frames_since_full = 0;
      return Ok(bytes);
    }

    // Render offscreen to get current frame without Clear/Hide/Show
    let current_bytes = self.render_offscreen(layout).await?;
    let current_rows = self.bytes_to_rows(&current_bytes);

    // Diff with previous
    let mut output = Vec::new();
    let prev_rows = self.last_diff_rows.as_ref().unwrap();
    let max_rows = prev_rows.len().max(current_rows.len());

    for row in 0..max_rows {
      let prev = prev_rows.get(row).map(|v| v.as_slice()).unwrap_or(&[]);
      let curr = current_rows.get(row).map(|v| v.as_slice()).unwrap_or(&[]);
      if prev != curr {
        // Move cursor to row start and print entire row content to ensure artifacts are cleared
        output.extend_from_slice(format!("\u{1b}[{};{}H", row + 1, 1).as_bytes());
        output.extend_from_slice(curr);

        // If the new row is shorter (by display width) than the previous, pad with spaces
        let prev_s = String::from_utf8_lossy(prev);
        let curr_s = String::from_utf8_lossy(curr);
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
    self.last_diff_rows = Some(current_rows);
    self.diff_frames_since_full = self.diff_frames_since_full.saturating_add(1);

    Ok(output)
  }

  /// Convert a full-frame byte buffer into per-line rows (naive parser).
  fn bytes_to_rows(&self, bytes: &[u8]) -> Vec<Vec<u8>> {
    let s = String::from_utf8_lossy(bytes);
    let mut rows: Vec<Vec<u8>> = Vec::new();
    let mut current = Vec::new();
    for ch in s.chars() {
      if ch == '\n' {
        // treat newline as row separator
        rows.push(current);
        current = Vec::new();
      } else {
        let mut buf = [0u8; 4];
        let n = ch.encode_utf8(&mut buf).len();
        current.extend_from_slice(&buf[..n]);
      }
    }
    if !current.is_empty() {
      rows.push(current);
    }
    rows
  }
}
