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

pub fn prev_grapheme_boundary(s: &str, byte_idx: usize) -> usize {
  if byte_idx == 0 {
    return 0;
  }
  let mut last = 0usize;
  for (i, _) in s[..byte_idx].grapheme_indices(true) {
    last = i;
  }
  last
}

pub fn next_grapheme_boundary(s: &str, byte_idx: usize) -> usize {
  if byte_idx >= s.len() {
    return s.len();
  }
  if let Some((i, g)) = s[byte_idx..].grapheme_indices(true).next() {
    return byte_idx + i + g.len();
  }
  s.len()
}

pub fn byte_index_for_display_col(s: &str, cols: usize) -> usize {
  if cols == 0 {
    return 0;
  }
  let mut acc = 0usize;
  let mut idx = 0usize;
  let mut i = 0usize;
  while i < s.len() {
    if let Some(end) = ansi_token_end(s, i) {
      i = end;
      idx = i;
      continue;
    }
    if let Some((rel, g)) = s[i..].grapheme_indices(true).next() {
      let w = UnicodeWidthStr::width(g);
      if acc + w > cols {
        break;
      }
      acc += w;
      i = i + rel + g.len();
      idx = i;
    } else {
      break;
    }
  }
  idx
}

pub fn visible_slice_by_width(s: &str, start_cols: usize, max_cols: usize) -> (&str, usize, usize) {
  let mut acc = 0usize;
  let mut end_byte;
  let start_byte = byte_index_for_display_col(s, start_cols);
  let mut i = start_byte;
  end_byte = start_byte;
  while i < s.len() {
    if let Some(end) = ansi_token_end(s, i) {
      i = end;
      end_byte = i;
      continue;
    }
    if let Some((rel, g)) = s[i..].grapheme_indices(true).next() {
      let w = UnicodeWidthStr::width(g);
      if acc + w > max_cols {
        break;
      }
      acc += w;
      i = i + rel + g.len();
      end_byte = i;
    } else {
      break;
    }
  }
  (&s[start_byte..end_byte], start_byte, end_byte)
}
