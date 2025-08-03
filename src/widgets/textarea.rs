/*!
 * Advanced Textarea Widget
 *
 * A comprehensive multi-line text editor widget with advanced features:
 * - Multi-line text editing with insert/delete operations
 * - Undo/Redo functionality
 * - Line numbers
 * - Syntax highlighting
 * - Markdown support
 * - Cursor line highlighting
 * - Search with regular expressions
 * - Text selection
 * - Mouse wheel scrolling
 * - Yank/paste support (C-k, C-j, etc.)
 * - Multiple textarea widgets support
 *
 * Integrates with utility CSS theming system and Element/Component architecture.
 */

use crate::components::Element;
use crate::themes::{BorderStyle, ColorTheme};
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Cursor position in the textarea
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CursorPosition {
  pub row: usize,
  pub col: usize,
}

/// Text selection range
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
  pub start: CursorPosition,
  pub end: CursorPosition,
}

impl Selection {
  pub fn new(start: CursorPosition, end: CursorPosition) -> Self {
    Self { start, end }
  }

  pub fn is_empty(&self) -> bool {
    self.start == self.end
  }

  pub fn normalize(&self) -> Self {
    if self.start.row < self.end.row
      || (self.start.row == self.end.row && self.start.col <= self.end.col)
    {
      *self
    } else {
      Self {
        start: self.end,
        end: self.start,
      }
    }
  }
}

/// Viewport for scrollable text display
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
  pub scroll_row: usize,
  pub scroll_col: usize,
  pub visible_rows: usize,
  pub visible_cols: usize,
}

impl Default for Viewport {
  fn default() -> Self {
    Self {
      scroll_row: 0,
      scroll_col: 0,
      visible_rows: 20,
      visible_cols: 80,
    }
  }
}

/// Textarea styling using utility classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextareaStyle {
  pub border_style: BorderStyle,
  pub base_classes: Vec<String>,
}

impl Default for TextareaStyle {
  fn default() -> Self {
    Self {
      border_style: BorderStyle::Rounded,
      base_classes: vec![
        "bg-gray-900".to_string(),
        "text-gray-100".to_string(),
        "border".to_string(),
        "border-gray-600".to_string(),
        "rounded".to_string(),
        "p-1".to_string(),
      ],
    }
  }
}

/// Textarea state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TextareaState {
  pub lines: Vec<String>,
  pub cursor: CursorPosition,
  pub selection: Option<Selection>,
  pub viewport: Viewport,
  pub focused: bool,
  pub readonly: bool,
  pub show_line_numbers: bool,
  pub highlight_current_line: bool,
  pub syntax_highlighting: bool,
  pub language: Option<String>,
  pub search_active: bool,
  pub modified: bool,
}

impl Default for TextareaState {
  fn default() -> Self {
    Self {
      lines: vec![String::new()],
      cursor: CursorPosition::default(),
      selection: None,
      viewport: Viewport::default(),
      focused: false,
      readonly: false,
      show_line_numbers: true,
      highlight_current_line: true,
      syntax_highlighting: false,
      language: None,
      search_active: false,
      modified: false,
    }
  }
}

/// Advanced Textarea Widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Textarea {
  pub id: String,
  pub state: TextareaState,
  pub style: TextareaStyle,
  pub css_classes: Vec<String>,
  pub theme: Option<String>,
  #[serde(skip)]
  pub current_theme: Option<ColorTheme>,
  #[serde(skip)]
  pub history: History,
  #[serde(skip)]
  pub search: Search,
  #[serde(skip)]
  pub yank_buffer: YankBuffer,
}

impl Textarea {
  /// Create a new textarea with the given ID
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      state: TextareaState::default(),
      style: TextareaStyle::default(),
      css_classes: Vec::new(),
      theme: None,
      current_theme: None,
      history: History::default(),
      search: Search::default(),
      yank_buffer: YankBuffer::default(),
    }
  }

  /// Create a builder for the textarea
  pub fn builder(id: impl Into<String>) -> TextareaBuilder {
    TextareaBuilder::new(id)
  }

  /// Set the text content
  pub fn set_text(&mut self, text: &str) {
    self.state.lines = if text.is_empty() {
      vec![String::new()]
    } else {
      text.lines().map(|line| line.to_string()).collect()
    };
    self.state.cursor = CursorPosition::default();
    self.state.selection = None;
    self.state.modified = true;
    self.fix_cursor_position();
  }

  /// Get the text content
  pub fn text(&self) -> String {
    self.state.lines.join("\n")
  }

  /// Set the language for syntax highlighting
  pub fn set_language(&mut self, language: Option<String>) {
    self.state.language = language;
  }

  /// Enable or disable syntax highlighting
  pub fn set_syntax_highlighting(&mut self, enabled: bool) {
    self.state.syntax_highlighting = enabled;
  }

  /// Set the color theme for the textarea
  pub fn set_theme(&mut self, theme: ColorTheme) {
    self.current_theme = Some(theme);
  }

  /// Set focus state
  pub fn set_focused(&mut self, focused: bool) {
    self.state.focused = focused;
  }

  /// Set readonly state
  pub fn set_readonly(&mut self, readonly: bool) {
    self.state.readonly = readonly;
  }

  /// Set viewport size
  pub fn set_viewport_size(&mut self, rows: usize, cols: usize) {
    self.state.viewport.visible_rows = rows;
    self.state.viewport.visible_cols = cols;
    self.fix_scroll();
  }

  /// Insert character at cursor position
  pub fn insert_char(&mut self, ch: char) {
    if self.state.readonly {
      return;
    }

    let op = EditOperation::InsertChar {
      pos: self.state.cursor,
      ch,
    };

    self.apply_operation(op);
  }

  /// Delete character at cursor position
  pub fn delete_char(&mut self) {
    if self.state.readonly || self.state.cursor.col >= self.current_line().len() {
      return;
    }

    let ch = self
      .current_line()
      .chars()
      .nth(self.state.cursor.col)
      .unwrap_or('\0');
    let op = EditOperation::DeleteChar {
      pos: self.state.cursor,
      ch,
    };

    self.apply_operation(op);
  }

  /// Delete character before cursor (backspace)
  pub fn delete_char_before(&mut self) {
    if self.state.readonly {
      return;
    }

    if self.state.cursor.col > 0 {
      self.move_cursor_left();
      self.delete_char();
    } else if self.state.cursor.row > 0 {
      // Join with previous line
      let current_line = self.state.lines[self.state.cursor.row].clone();
      let prev_line_len = self.state.lines[self.state.cursor.row - 1].len();

      let op = EditOperation::JoinLines {
        row: self.state.cursor.row,
        text: current_line,
      };

      self.apply_operation(op);
      self.state.cursor = CursorPosition {
        row: self.state.cursor.row - 1,
        col: prev_line_len,
      };
    }
  }

  /// Insert a new line at cursor position
  pub fn insert_newline(&mut self) {
    if self.state.readonly {
      return;
    }

    let op = EditOperation::SplitLine {
      pos: self.state.cursor,
    };

    self.apply_operation(op);
  }

  /// Move cursor left
  pub fn move_cursor_left(&mut self) {
    if self.state.cursor.col > 0 {
      self.state.cursor.col -= 1;
    } else if self.state.cursor.row > 0 {
      self.state.cursor.row -= 1;
      self.state.cursor.col = self.current_line().len();
    }
    self.fix_scroll();
  }

  /// Move cursor right
  pub fn move_cursor_right(&mut self) {
    if self.state.cursor.col < self.current_line().len() {
      self.state.cursor.col += 1;
    } else if self.state.cursor.row < self.state.lines.len() - 1 {
      self.state.cursor.row += 1;
      self.state.cursor.col = 0;
    }
    self.fix_scroll();
  }

  /// Move cursor up
  pub fn move_cursor_up(&mut self) {
    if self.state.cursor.row > 0 {
      self.state.cursor.row -= 1;
      self.fix_cursor_position();
      self.fix_scroll();
    }
  }

  /// Move cursor down
  pub fn move_cursor_down(&mut self) {
    if self.state.cursor.row < self.state.lines.len() - 1 {
      self.state.cursor.row += 1;
      self.fix_cursor_position();
      self.fix_scroll();
    }
  }

  /// Move cursor to beginning of line
  pub fn move_cursor_to_line_start(&mut self) {
    self.state.cursor.col = 0;
    self.fix_scroll();
  }

  /// Move cursor to end of line
  pub fn move_cursor_to_line_end(&mut self) {
    self.state.cursor.col = self.current_line().len();
    self.fix_scroll();
  }

  /// Get current line
  fn current_line(&self) -> &String {
    &self.state.lines[self.state.cursor.row]
  }

  /// Fix cursor position to be within bounds
  fn fix_cursor_position(&mut self) {
    if self.state.cursor.row >= self.state.lines.len() {
      self.state.cursor.row = self.state.lines.len().saturating_sub(1);
    }

    let line_len = self.current_line().len();
    if self.state.cursor.col > line_len {
      self.state.cursor.col = line_len;
    }
  }

  /// Fix scroll to keep cursor visible
  fn fix_scroll(&mut self) {
    // Vertical scrolling
    if self.state.cursor.row < self.state.viewport.scroll_row {
      self.state.viewport.scroll_row = self.state.cursor.row;
    } else if self.state.cursor.row
      >= self.state.viewport.scroll_row + self.state.viewport.visible_rows
    {
      self.state.viewport.scroll_row = self.state.cursor.row - self.state.viewport.visible_rows + 1;
    }

    // Horizontal scrolling
    if self.state.cursor.col < self.state.viewport.scroll_col {
      self.state.viewport.scroll_col = self.state.cursor.col;
    } else if self.state.cursor.col
      >= self.state.viewport.scroll_col + self.state.viewport.visible_cols
    {
      self.state.viewport.scroll_col = self.state.cursor.col - self.state.viewport.visible_cols + 1;
    }
  }

  /// Apply an edit operation and add to history
  fn apply_operation(&mut self, op: EditOperation) {
    op.apply(&mut self.state.lines, &mut self.state.cursor);
    self.history.push(op);
    self.state.modified = true;
    self.fix_cursor_position();
    self.fix_scroll();
  }

  /// Undo last operation
  pub fn undo(&mut self) -> bool {
    if let Some(op) = self.history.undo() {
      let inverse = op.inverse();
      inverse.apply(&mut self.state.lines, &mut self.state.cursor);
      self.fix_cursor_position();
      self.fix_scroll();
      true
    } else {
      false
    }
  }

  /// Redo last undone operation
  pub fn redo(&mut self) -> bool {
    if let Some(op) = self.history.redo() {
      op.apply(&mut self.state.lines, &mut self.state.cursor);
      self.fix_cursor_position();
      self.fix_scroll();
      true
    } else {
      false
    }
  }

  /// Render to Element for the component system
  pub fn render(&self) -> Element {
    let mut element = Element::with_tag("textarea")
      .id(&self.id)
      .focusable(true)
      .class("textarea");

    // Apply base style utility classes
    for class in &self.style.base_classes {
      element = element.class(class);
    }

    // Apply custom CSS classes
    for class in &self.css_classes {
      element = element.class(class);
    }

    // Apply state-based utility classes
    if self.state.focused {
      element = element
        .class("focused")
        .class("border-blue-500")
        .class("ring-2")
        .class("ring-blue-300");
    }

    if self.state.readonly {
      element = element
        .class("readonly")
        .class("bg-gray-800")
        .class("text-gray-400")
        .class("cursor-not-allowed");
    }

    if self.state.highlight_current_line {
      element = element.class("highlight-current-line");
    }

    if self.state.show_line_numbers {
      element = element.class("show-line-numbers");
    }

    if self.state.syntax_highlighting && self.state.language.is_some() {
      element = element.class("syntax-highlighted");
    }

    if self.state.modified {
      element = element.class("modified");
    }

    // Add content as text
    let content = self.render_visible_content();
    element = element.content(&content);

    element.into()
  }

  /// Render the visible content within the viewport
  fn render_visible_content(&self) -> String {
    let mut output = String::new();
    let start_row = self.state.viewport.scroll_row;
    let end_row = (start_row + self.state.viewport.visible_rows).min(self.state.lines.len());

    for (line_idx, line) in self
      .state
      .lines
      .iter()
      .enumerate()
      .skip(start_row)
      .take(end_row - start_row)
    {
      // Add line number if enabled
      if self.state.show_line_numbers {
        output.push_str(&format!("{:4} ", line_idx + 1));
      }

      // Get visible portion of line
      let start_col = self.state.viewport.scroll_col;
      let end_col = start_col + self.state.viewport.visible_cols;
      let visible_line = if start_col < line.len() {
        &line[start_col..end_col.min(line.len())]
      } else {
        ""
      };

      // Apply syntax highlighting if enabled
      let highlighted_line = if self.state.syntax_highlighting {
        self.apply_syntax_highlighting(visible_line)
      } else {
        visible_line.to_string()
      };

      // Highlight current line if this is the cursor line
      if line_idx == self.state.cursor.row && self.state.highlight_current_line {
        // Apply current line highlighting via ANSI escape codes
        output.push_str("\x1b[48;5;236m"); // Dark gray background for current line
        output.push_str(&highlighted_line);
        output.push_str("\x1b[0m"); // Reset styling
      } else {
        output.push_str(&highlighted_line);
      }

      output.push('\n');
    }

    output
  }

  /// Apply basic syntax highlighting
  fn apply_syntax_highlighting(&self, line: &str) -> String {
    if let Some(ref language) = self.state.language {
      match language.as_str() {
        "rust" => self.highlight_rust_basic(line),
        "javascript" | "js" => self.highlight_javascript_basic(line),
        "python" => self.highlight_python_basic(line),
        "markdown" | "md" => self.highlight_markdown_basic(line),
        _ => line.to_string(),
      }
    } else {
      line.to_string()
    }
  }

  fn highlight_rust_basic(&self, line: &str) -> String {
    // Simple keyword highlighting for Rust
    let keywords = [
      "fn", "let", "mut", "pub", "struct", "enum", "impl", "trait", "use",
    ];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_javascript_basic(&self, line: &str) -> String {
    let keywords = ["function", "const", "let", "var", "if", "else", "return"];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_python_basic(&self, line: &str) -> String {
    let keywords = ["def", "class", "if", "else", "for", "while", "import"];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_markdown_basic(&self, line: &str) -> String {
    if line.starts_with('#') || line.starts_with("```") {
      format!("{line}</span>")
    } else {
      line.to_string()
    }
  }

  fn highlight_keywords(&self, line: &str, keywords: &[&str]) -> String {
    use crate::themes::colors::{color_to_ansi, ColorDefinition};

    let mut result = line.to_string();

    // Define highlight color (bright cyan for keywords)
    let highlight_color = ColorDefinition {
      r: 0,
      g: 255,
      b: 255,
    };
    let highlight_ansi = color_to_ansi(highlight_color, false);
    let reset_ansi = "\x1b[0m";

    for keyword in keywords {
      if result.contains(keyword) {
        // Replace all occurrences of the keyword with highlighted version
        result = result.replace(keyword, &format!("{highlight_ansi}{keyword}{reset_ansi}"));
      }
    }

    result
  }
}

impl EditOperation {
  /// Apply this operation to the text buffer
  pub fn apply(&self, lines: &mut Vec<String>, cursor: &mut CursorPosition) {
    match self {
      EditOperation::InsertChar { pos, ch } => {
        if pos.row < lines.len() {
          lines[pos.row].insert(pos.col, *ch);
          *cursor = CursorPosition {
            row: pos.row,
            col: pos.col + 1,
          };
        }
      }
      EditOperation::DeleteChar { pos, .. } => {
        if pos.row < lines.len() && pos.col < lines[pos.row].len() {
          lines[pos.row].remove(pos.col);
          *cursor = *pos;
        }
      }
      EditOperation::InsertText { pos, text } => {
        if pos.row < lines.len() {
          lines[pos.row].insert_str(pos.col, text);
          *cursor = CursorPosition {
            row: pos.row,
            col: pos.col + text.len(),
          };
        }
      }
      EditOperation::DeleteText { pos, text } => {
        if pos.row < lines.len() {
          let end_col = (pos.col + text.len()).min(lines[pos.row].len());
          lines[pos.row].drain(pos.col..end_col);
          *cursor = *pos;
        }
      }
      EditOperation::InsertLine { row, text } => {
        if *row <= lines.len() {
          lines.insert(*row, text.clone());
          *cursor = CursorPosition {
            row: *row,
            col: text.len(),
          };
        }
      }
      EditOperation::DeleteLine { row, .. } => {
        if *row < lines.len() {
          lines.remove(*row);
          *cursor = CursorPosition {
            row: (*row).saturating_sub(1),
            col: 0,
          };
        }
      }
      EditOperation::SplitLine { pos } => {
        if pos.row < lines.len() {
          let line = &lines[pos.row];
          let new_line = line[pos.col..].to_string();
          lines[pos.row].truncate(pos.col);
          lines.insert(pos.row + 1, new_line);
          *cursor = CursorPosition {
            row: pos.row + 1,
            col: 0,
          };
        }
      }
      EditOperation::JoinLines { row, .. } => {
        if *row > 0 && *row < lines.len() {
          let line = lines.remove(*row);
          let prev_len = lines[*row - 1].len();
          lines[*row - 1].push_str(&line);
          *cursor = CursorPosition {
            row: *row - 1,
            col: prev_len,
          };
        }
      }
    }
  }

  /// Get the inverse operation for undo
  pub fn inverse(&self) -> EditOperation {
    match self {
      EditOperation::InsertChar { pos, ch } => EditOperation::DeleteChar { pos: *pos, ch: *ch },
      EditOperation::DeleteChar { pos, ch } => EditOperation::InsertChar { pos: *pos, ch: *ch },
      EditOperation::InsertText { pos, text } => EditOperation::DeleteText {
        pos: *pos,
        text: text.clone(),
      },
      EditOperation::DeleteText { pos, text } => EditOperation::InsertText {
        pos: *pos,
        text: text.clone(),
      },
      EditOperation::InsertLine { row, text } => EditOperation::DeleteLine {
        row: *row,
        text: text.clone(),
      },
      EditOperation::DeleteLine { row, text } => EditOperation::InsertLine {
        row: *row,
        text: text.clone(),
      },
      EditOperation::SplitLine { pos } => EditOperation::JoinLines {
        row: pos.row + 1,
        text: String::new(),
      },
      EditOperation::JoinLines { row, text } => EditOperation::SplitLine {
        pos: CursorPosition {
          row: *row - 1,
          col: text.len(),
        },
      },
    }
  }
}

/// Edit operation for undo/redo
#[derive(Debug, Clone, PartialEq)]
pub enum EditOperation {
  InsertChar { pos: CursorPosition, ch: char },
  DeleteChar { pos: CursorPosition, ch: char },
  InsertText { pos: CursorPosition, text: String },
  DeleteText { pos: CursorPosition, text: String },
  InsertLine { row: usize, text: String },
  DeleteLine { row: usize, text: String },
  SplitLine { pos: CursorPosition },
  JoinLines { row: usize, text: String },
}

/// History manager for undo/redo
#[derive(Debug, Clone, PartialEq)]
pub struct History {
  operations: VecDeque<EditOperation>,
  current: usize,
  max_size: usize,
}

impl Default for History {
  fn default() -> Self {
    Self {
      operations: VecDeque::new(),
      current: 0,
      max_size: 1000,
    }
  }
}

impl History {
  pub fn push(&mut self, op: EditOperation) {
    // Remove any operations after current position
    self.operations.truncate(self.current);

    // Add new operation
    self.operations.push_back(op);
    self.current = self.operations.len();

    // Limit history size
    if self.operations.len() > self.max_size {
      self.operations.pop_front();
      self.current = self.operations.len();
    }
  }

  pub fn undo(&mut self) -> Option<&EditOperation> {
    if self.current > 0 {
      self.current -= 1;
      self.operations.get(self.current)
    } else {
      None
    }
  }

  pub fn redo(&mut self) -> Option<&EditOperation> {
    if self.current < self.operations.len() {
      let op = self.operations.get(self.current);
      self.current += 1;
      op
    } else {
      None
    }
  }

  pub fn can_undo(&self) -> bool {
    self.current > 0
  }

  pub fn can_redo(&self) -> bool {
    self.current < self.operations.len()
  }
}

/// Search functionality
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Search {
  pub query: String,
  pub case_sensitive: bool,
  pub whole_word: bool,
}

/// Yank buffer for copy/paste operations
#[derive(Debug, Clone, Default, PartialEq)]
pub struct YankBuffer {
  pub text: String,
  pub is_line_mode: bool,
}

impl YankBuffer {
  pub fn set_text(&mut self, text: String, is_line_mode: bool) {
    self.text = text;
    self.is_line_mode = is_line_mode;
  }

  pub fn get_text(&self) -> &str {
    &self.text
  }

  pub fn is_empty(&self) -> bool {
    self.text.is_empty()
  }
}

/// Builder for Textarea widget
pub struct TextareaBuilder {
  textarea: Textarea,
}

impl TextareaBuilder {
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      textarea: Textarea::new(id),
    }
  }

  pub fn text(mut self, text: &str) -> Self {
    self.textarea.set_text(text);
    self
  }

  pub fn language(mut self, language: &str) -> Self {
    self.textarea.set_language(Some(language.to_string()));
    self
  }

  pub fn syntax_highlighting(mut self, enabled: bool) -> Self {
    self.textarea.set_syntax_highlighting(enabled);
    self
  }

  pub fn show_line_numbers(mut self, show: bool) -> Self {
    self.textarea.state.show_line_numbers = show;
    self
  }

  pub fn highlight_current_line(mut self, highlight: bool) -> Self {
    self.textarea.state.highlight_current_line = highlight;
    self
  }

  pub fn readonly(mut self, readonly: bool) -> Self {
    self.textarea.set_readonly(readonly);
    self
  }

  pub fn viewport_size(mut self, rows: usize, cols: usize) -> Self {
    self.textarea.set_viewport_size(rows, cols);
    self
  }

  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.textarea.css_classes.push(class.into());
    self
  }

  pub fn classes(mut self, classes: &[&str]) -> Self {
    for class in classes {
      self.textarea.css_classes.push(class.to_string());
    }
    self
  }

  pub fn bg_color(mut self, color: &str) -> Self {
    self.textarea.css_classes.push(format!("bg-{color}"));
    self
  }

  pub fn text_color(mut self, color: &str) -> Self {
    self.textarea.css_classes.push(format!("text-{color}"));
    self
  }

  pub fn border_color(mut self, color: &str) -> Self {
    self.textarea.css_classes.push(format!("border-{color}"));
    self
  }

  pub fn build(self) -> Textarea {
    self.textarea
  }
}

impl Search {
  pub fn set_query(&mut self, query: String) {
    self.query = query;
  }

  pub fn find_matches(&self, text: &str) -> Vec<(usize, usize)> {
    if self.query.is_empty() {
      return Vec::new();
    }

    let search_text = if self.case_sensitive {
      text.to_string()
    } else {
      text.to_lowercase()
    };

    let search_query = if self.case_sensitive {
      self.query.clone()
    } else {
      self.query.to_lowercase()
    };

    let mut matches = Vec::new();
    let mut start = 0;

    while let Some(pos) = search_text[start..].find(&search_query) {
      let match_start = start + pos;
      let match_end = match_start + search_query.len();

      // Check whole word constraint
      if self.whole_word {
        let is_word_start = match_start == 0
          || !text
            .chars()
            .nth(match_start - 1)
            .unwrap_or(' ')
            .is_alphanumeric();
        let is_word_end =
          match_end >= text.len() || !text.chars().nth(match_end).unwrap_or(' ').is_alphanumeric();

        if is_word_start && is_word_end {
          matches.push((match_start, match_end));
        }
      } else {
        matches.push((match_start, match_end));
      }

      start = match_start + 1;
    }

    matches
  }
}

/// Syntax highlighting support
#[derive(Debug, Clone, Default)]
pub struct SyntaxHighlighter {
  pub language: Option<String>,
  pub enabled: bool,
}

impl SyntaxHighlighter {
  pub fn set_language(&mut self, language: Option<String>) {
    self.language = language;
  }

  pub fn highlight_line(&self, line: &str) -> Vec<(String, String)> {
    // Basic syntax highlighting - can be extended with tree-sitter
    if !self.enabled || self.language.is_none() {
      return vec![(line.to_string(), "text".to_string())];
    }

    // Simple keyword highlighting for demonstration
    match self.language.as_ref().unwrap().as_str() {
      "rust" => self.highlight_rust(line),
      "javascript" | "js" => self.highlight_javascript(line),
      "python" => self.highlight_python(line),
      "markdown" | "md" => self.highlight_markdown(line),
      _ => vec![(line.to_string(), "text".to_string())],
    }
  }

  fn highlight_rust(&self, line: &str) -> Vec<(String, String)> {
    // Simple Rust keyword highlighting
    let keywords = [
      "fn", "let", "mut", "pub", "struct", "enum", "impl", "trait", "use", "mod",
    ];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_javascript(&self, line: &str) -> Vec<(String, String)> {
    let keywords = [
      "function", "const", "let", "var", "if", "else", "for", "while", "return",
    ];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_python(&self, line: &str) -> Vec<(String, String)> {
    let keywords = [
      "def", "class", "if", "else", "for", "while", "import", "from", "return",
    ];
    self.highlight_keywords(line, &keywords)
  }

  fn highlight_markdown(&self, line: &str) -> Vec<(String, String)> {
    if line.starts_with('#') {
      vec![(line.to_string(), "heading".to_string())]
    } else if line.starts_with("```") {
      vec![(line.to_string(), "code_block".to_string())]
    } else {
      vec![(line.to_string(), "text".to_string())]
    }
  }

  fn highlight_keywords(&self, line: &str, keywords: &[&str]) -> Vec<(String, String)> {
    // Simple word-based highlighting
    let mut result = Vec::new();
    let mut current = String::new();
    let chars = line.chars();

    for ch in chars {
      if ch.is_alphabetic() || ch == '_' {
        current.push(ch);
      } else {
        if !current.is_empty() {
          let style = if keywords.contains(&current.as_str()) {
            "keyword"
          } else {
            "text"
          };
          result.push((current.clone(), style.to_string()));
          current.clear();
        }
        result.push((ch.to_string(), "text".to_string()));
      }
    }

    if !current.is_empty() {
      let style = if keywords.contains(&current.as_str()) {
        "keyword"
      } else {
        "text"
      };
      result.push((current, style.to_string()));
    }

    result
  }
}
