use reactive_tui::widgets::textarea::Textarea;
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn ansi_token_end(s: &str, start: usize) -> Option<usize> {
  let bytes = s.as_bytes();
  if start >= bytes.len() || bytes[start] != 0x1b {
    return None;
  }
  let len = bytes.len();
  if start + 1 >= len {
    return Some(len);
  }
  match bytes[start + 1] {
    b'[' => {
      let mut j = start + 2;
      while j < len {
        let b = bytes[j];
        if (0x40..=0x7e).contains(&b) {
          return Some(j + 1);
        }
        j += 1;
      }
      Some(len)
    }
    b']' => {
      let mut j = start + 2;
      while j < len {
        if bytes[j] == 0x07 {
          return Some(j + 1);
        }
        if bytes[j] == 0x1b && j + 1 < len && bytes[j + 1] == b'\\' {
          return Some(j + 2);
        }
        j += 1;
      }
      Some(len)
    }
    _ => Some((start + 2).min(len)),
  }
}
fn visible_width(s: &str) -> usize {
  let mut i = 0;
  let mut w = 0;
  while i < s.len() {
    if let Some(end) = ansi_token_end(s, i) {
      i = end;
      continue;
    }
    if let Some((_, g)) = s[i..].grapheme_indices(true).next() {
      w += UnicodeWidthStr::width(g);
      i += g.len();
    } else {
      break;
    }
  }
  w
}

#[test]
fn textarea_grapheme_cursor_navigation() {
  let mut ta = Textarea::new("t");
  // String with multi-codepoint graphemes and wide chars
  ta.set_text("a🙂́b"); // '🙂' + combining acute over next char visually; simplified for test
                       // Move to end
  ta.move_cursor_to_line_end();
  // Move left by one grapheme: should land before last grapheme
  let col_before = ta.state.cursor.col;
  ta.move_cursor_left();
  assert!(
    ta.state.cursor.col < col_before,
    "cursor should move left by grapheme"
  );
  // Move right back
  let col_left = ta.state.cursor.col;
  ta.move_cursor_right();
  assert!(
    ta.state.cursor.col > col_left,
    "cursor should move right by grapheme"
  );
}

#[test]
fn textarea_scroll_by_display_columns() {
  let mut ta = Textarea::new("t");
  // Build a long line with some wide chars
  ta.set_text("αβγ🙂δεζη θικλμνξοπρστυφχψω");
  ta.set_viewport_size(3, 10); // 10 cols visible
  ta.move_cursor_to_line_end();
  // Ensure scroll_col tracks display width
  assert!(ta.state.viewport.scroll_col > 0);
}

#[test]
fn rich_text_wrap_ansi_unicode_width() {
  use reactive_tui::widgets::rich_text::RichTextBuilder;
  let content = "Hello \x1b[31m世界\x1b[0m emoji🙂 wrap test";
  let mut rt = RichTextBuilder::new("rt")
    .content(content)
    .width(8)
    .word_wrap(true)
    .build();
  rt.parse_markdown();
  rt.render_content();
  // We expect wrapped lines; ensure at least 2
  assert!(rt.rendered_lines.len() >= 2);
  // Visible widths (excluding ANSI) should not exceed width
  for line in &rt.rendered_lines {
    assert!(
      visible_width(line.as_str()) <= 8,
      "wrapped line exceeds width: {:?}",
      line
    );
  }
}
