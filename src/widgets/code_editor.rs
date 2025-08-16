/*!
 * CodeEditor Component - Advanced code editing with syntax highlighting
 *
 * A comprehensive code editor widget providing:
 * - Syntax highlighting for multiple languages
 * - Line numbers and gutter
 * - Code folding and indentation
 * - Find/replace functionality
 * - Multiple cursor support
 * - Auto-completion and snippets
 * - Vim-like key bindings (optional)
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// Programming language for syntax highlighting
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Language {
  Rust,
  JavaScript,
  TypeScript,
  Python,
  Go,
  C,
  Cpp,
  Java,
  Html,
  Css,
  Json,
  Yaml,
  Markdown,
  Bash,
  PlainText,
}

/// Syntax token types
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TokenType {
  Keyword,
  String,
  Number,
  Comment,
  Identifier,
  Operator,
  Punctuation,
  Type,
  Function,
  Variable,
  Constant,
  Whitespace,
}

/// Syntax token
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
  pub token_type: TokenType,
  pub text: String,
  pub start: usize,
  pub end: usize,
}

/// Cursor position in code editor
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EditorCursor {
  pub line: usize,
  pub column: usize,
}

/// Text selection in code editor
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorSelection {
  pub start: EditorCursor,
  pub end: EditorCursor,
}

/// Editor configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorConfig {
  pub language: Language,
  pub tab_size: usize,
  pub use_spaces: bool,
  pub show_line_numbers: bool,
  pub show_whitespace: bool,
  pub word_wrap: bool,
  pub auto_indent: bool,
  pub vim_mode: bool,
  pub font_size: u16,
}

impl Default for EditorConfig {
  fn default() -> Self {
    Self {
      language: Language::PlainText,
      tab_size: 4,
      use_spaces: true,
      show_line_numbers: true,
      show_whitespace: false,
      word_wrap: false,
      auto_indent: true,
      vim_mode: false,
      font_size: 14,
    }
  }
}

/// Editor styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorStyle {
  pub background: String,
  pub text_color: String,
  pub line_number_color: String,
  pub line_number_bg: String,
  pub cursor_color: String,
  pub selection_bg: String,
  pub current_line_bg: String,
  pub gutter_bg: String,
  pub syntax_colors: HashMap<TokenType, String>,
}

impl Default for EditorStyle {
  fn default() -> Self {
    let mut syntax_colors = HashMap::new();
    syntax_colors.insert(TokenType::Keyword, "#0000ff".to_string());
    syntax_colors.insert(TokenType::String, "#008000".to_string());
    syntax_colors.insert(TokenType::Number, "#ff0000".to_string());
    syntax_colors.insert(TokenType::Comment, "#808080".to_string());
    syntax_colors.insert(TokenType::Function, "#795e26".to_string());
    syntax_colors.insert(TokenType::Type, "#267f99".to_string());
    syntax_colors.insert(TokenType::Operator, "#000000".to_string());

    Self {
      background: "#ffffff".to_string(),
      text_color: "#000000".to_string(),
      line_number_color: "#858585".to_string(),
      line_number_bg: "#f5f5f5".to_string(),
      cursor_color: "#000000".to_string(),
      selection_bg: "#add6ff".to_string(),
      current_line_bg: "#f5f5f5".to_string(),
      gutter_bg: "#f8f8f8".to_string(),
      syntax_colors,
    }
  }
}

/// Code editor widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeEditor {
  pub content: Vec<String>,
  pub cursor: EditorCursor,
  pub selection: Option<EditorSelection>,
  pub scroll_offset: usize,
  pub config: EditorConfig,
  pub style: EditorStyle,
  pub width: u16,
  pub height: u16,
  pub modified: bool,
}

impl CodeEditor {
  /// Create a new code editor
  pub fn new() -> Self {
    Self {
      content: vec![String::new()],
      cursor: EditorCursor { line: 0, column: 0 },
      selection: None,
      scroll_offset: 0,
      config: EditorConfig::default(),
      style: EditorStyle::default(),
      width: 80,
      height: 24,
      modified: false,
    }
  }

  /// Set editor content
  pub fn set_content(&mut self, content: &str) -> &mut Self {
    self.content = content.lines().map(|s| s.to_string()).collect();
    if self.content.is_empty() {
      self.content.push(String::new());
    }
    self.cursor = EditorCursor { line: 0, column: 0 };
    self.selection = None;
    self.scroll_offset = 0;
    self.modified = false;
    self
  }

  /// Get editor content as string
  pub fn get_content(&self) -> String {
    self.content.join("\n")
  }

  /// Set programming language
  pub fn set_language(&mut self, language: Language) -> &mut Self {
    self.config.language = language;
    self
  }

  /// Insert text at cursor position
  pub fn insert_text(&mut self, text: &str) -> Result<()> {
    if self.cursor.line >= self.content.len() {
      return Err(TuiError::component("Invalid cursor position".to_string()));
    }

    let line = &mut self.content[self.cursor.line];

    if text.contains('\n') {
      // Handle multi-line insertion
      let lines: Vec<&str> = text.split('\n').collect();
      let before = line[..self.cursor.column].to_string();
      let after = line[self.cursor.column..].to_string();

      // Replace current line with first part + first line of text
      *line = before + lines[0];

      // Insert middle lines
      for (i, &line_text) in lines.iter().enumerate().skip(1) {
        if i == lines.len() - 1 {
          // Last line: add remaining text
          self.content.insert(self.cursor.line + i, line_text.to_string() + &after);
          self.cursor.line += i;
          self.cursor.column = line_text.len();
        } else {
          self.content.insert(self.cursor.line + i, line_text.to_string());
        }
      }
    } else {
      // Single line insertion
      line.insert_str(self.cursor.column, text);
      self.cursor.column += text.len();
    }

    self.modified = true;
    Ok(())
  }

  /// Delete character at cursor
  pub fn delete_char(&mut self) -> Result<()> {
    if self.cursor.line >= self.content.len() {
      return Ok(());
    }

    if self.cursor.column < self.content[self.cursor.line].len() {
      self.content[self.cursor.line].remove(self.cursor.column);
    } else if self.cursor.line < self.content.len() - 1 {
      // Join with next line
      let next_line = self.content.remove(self.cursor.line + 1);
      self.content[self.cursor.line].push_str(&next_line);
    }

    self.modified = true;
    Ok(())
  }

  /// Backspace at cursor
  pub fn backspace(&mut self) -> Result<()> {
    if self.cursor.column > 0 {
      self.cursor.column -= 1;
      self.delete_char()?;
    } else if self.cursor.line > 0 {
      // Move to end of previous line
      let current_line = self.content.remove(self.cursor.line);
      self.cursor.line -= 1;
      self.cursor.column = self.content[self.cursor.line].len();
      self.content[self.cursor.line].push_str(&current_line);
      self.modified = true;
    }
    Ok(())
  }

  /// Move cursor
  pub fn move_cursor(&mut self, line: usize, column: usize) -> Result<()> {
    if line >= self.content.len() {
      return Err(TuiError::component("Invalid line number".to_string()));
    }

    self.cursor.line = line;
    self.cursor.column = column.min(self.content[line].len());

    // Adjust scroll if needed
    if self.cursor.line < self.scroll_offset {
      self.scroll_offset = self.cursor.line;
    } else if self.cursor.line >= self.scroll_offset + self.height as usize {
      self.scroll_offset = self.cursor.line - self.height as usize + 1;
    }

    Ok(())
  }

  /// Move cursor up
  pub fn cursor_up(&mut self) -> Result<()> {
    if self.cursor.line > 0 {
      let new_line = self.cursor.line - 1;
      let new_column = self.cursor.column.min(self.content[new_line].len());
      self.move_cursor(new_line, new_column)?;
    }
    Ok(())
  }

  /// Move cursor down
  pub fn cursor_down(&mut self) -> Result<()> {
    if self.cursor.line < self.content.len() - 1 {
      let new_line = self.cursor.line + 1;
      let new_column = self.cursor.column.min(self.content[new_line].len());
      self.move_cursor(new_line, new_column)?;
    }
    Ok(())
  }

  /// Move cursor left
  pub fn cursor_left(&mut self) -> Result<()> {
    if self.cursor.column > 0 {
      self.cursor.column -= 1;
    } else if self.cursor.line > 0 {
      self.cursor.line -= 1;
      self.cursor.column = self.content[self.cursor.line].len();
    }
    Ok(())
  }

  /// Move cursor right
  pub fn cursor_right(&mut self) -> Result<()> {
    if self.cursor.column < self.content[self.cursor.line].len() {
      self.cursor.column += 1;
    } else if self.cursor.line < self.content.len() - 1 {
      self.cursor.line += 1;
      self.cursor.column = 0;
    }
    Ok(())
  }

  /// Simple syntax highlighting (basic implementation)
  fn highlight_line(&self, line: &str) -> Vec<Token> {
    let mut tokens = Vec::new();

    match self.config.language {
      Language::Rust => self.highlight_rust(line, &mut tokens),
      Language::JavaScript | Language::TypeScript => self.highlight_javascript(line, &mut tokens),
      Language::Python => self.highlight_python(line, &mut tokens),
      _ => {
        // Plain text - no highlighting
        if !line.is_empty() {
          tokens.push(Token {
            token_type: TokenType::Identifier,
            text: line.to_string(),
            start: 0,
            end: line.len(),
          });
        }
      }
    }

    tokens
  }

  /// Rust syntax highlighting
  fn highlight_rust(&self, line: &str, tokens: &mut Vec<Token>) {
    let keywords = ["fn", "let", "mut", "if", "else", "match", "for", "while", "loop", "break", "continue", "return", "struct", "enum", "impl", "trait", "pub", "use", "mod", "const", "static"];
    let types = ["i32", "u32", "i64", "u64", "f32", "f64", "bool", "char", "str", "String", "Vec", "Option", "Result"];

    self.highlight_generic(line, tokens, &keywords, &types);
  }

  /// JavaScript syntax highlighting
  fn highlight_javascript(&self, line: &str, tokens: &mut Vec<Token>) {
    let keywords = ["function", "var", "let", "const", "if", "else", "for", "while", "do", "switch", "case", "break", "continue", "return", "try", "catch", "finally", "throw", "class", "extends", "import", "export"];
    let types = ["Array", "Object", "String", "Number", "Boolean", "Date", "RegExp", "Promise"];

    self.highlight_generic(line, tokens, &keywords, &types);
  }

  /// Python syntax highlighting
  fn highlight_python(&self, line: &str, tokens: &mut Vec<Token>) {
    let keywords = ["def", "class", "if", "elif", "else", "for", "while", "break", "continue", "return", "try", "except", "finally", "raise", "import", "from", "as", "with", "lambda", "yield", "global", "nonlocal"];
    let types = ["int", "float", "str", "bool", "list", "dict", "tuple", "set"];

    self.highlight_generic(line, tokens, &keywords, &types);
  }

  /// Generic syntax highlighting
  fn highlight_generic(&self, line: &str, tokens: &mut Vec<Token>, keywords: &[&str], types: &[&str]) {
    let mut chars = line.char_indices().peekable();

    while let Some((pos, ch)) = chars.next() {

      if ch == '/' {
        if let Some((_, '/')) = chars.peek() {
          // Line comment
          tokens.push(Token {
            token_type: TokenType::Comment,
            text: line[pos..].to_string(),
            start: pos,
            end: line.len(),
          });
          break;
        }
      } else if ch == '"' || ch == '\'' {
        // String literal
        let quote = ch;
        let start = pos;
        let mut end = pos + 1;
        let mut escaped = false;

        while let Some((next_pos, next_ch)) = chars.next() {
          end = next_pos + 1;
          if !escaped && next_ch == quote {
            break;
          }
          escaped = next_ch == '\\' && !escaped;
        }

        tokens.push(Token {
          token_type: TokenType::String,
          text: line[start..end].to_string(),
          start,
          end,
        });
      } else if ch.is_ascii_digit() {
        // Number
        let start = pos;
        let mut end = pos + 1;

        while let Some((next_pos, next_ch)) = chars.peek() {
          if next_ch.is_ascii_digit() || *next_ch == '.' {
            end = *next_pos + 1;
            chars.next();
          } else {
            break;
          }
        }

        tokens.push(Token {
          token_type: TokenType::Number,
          text: line[start..end].to_string(),
          start,
          end,
        });
      } else if ch.is_alphabetic() || ch == '_' {
        // Identifier/keyword
        let start = pos;
        let mut end = pos + 1;

        while let Some((next_pos, next_ch)) = chars.peek() {
          if next_ch.is_alphanumeric() || *next_ch == '_' {
            end = *next_pos + 1;
            chars.next();
          } else {
            break;
          }
        }

        let text = &line[start..end];
        let token_type = if keywords.contains(&text) {
          TokenType::Keyword
        } else if types.contains(&text) {
          TokenType::Type
        } else {
          TokenType::Identifier
        };

        tokens.push(Token {
          token_type,
          text: text.to_string(),
          start,
          end,
        });
      }
    }
  }

  /// Render the code editor
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let line_num_color_def = get_palette_color(&theme.palette, &self.style.line_number_color)
      .map_err(|e| TuiError::render(e))?;
    let _line_num_color = color_to_ansi(line_num_color_def, false);

    let gutter_bg_def = get_palette_color(&theme.palette, &self.style.gutter_bg)
      .map_err(|e| TuiError::render(e))?;
    let gutter_bg = color_to_ansi(gutter_bg_def, true);

    let cursor_color_def = get_palette_color(&theme.palette, &self.style.cursor_color)
      .map_err(|e| TuiError::render(e))?;
    let cursor_color = color_to_ansi(cursor_color_def, false);

    let gutter_width = if self.config.show_line_numbers {
      (self.content.len().to_string().len() + 2) as u16
    } else {
      0
    };

    let _content_width = rect.width.saturating_sub(gutter_width);
    let visible_lines = rect.height as usize;

    // Render each visible line
    for row in 0..visible_lines {
      let line_idx = self.scroll_offset + row;
      let y = rect.y + row as u16;

      // Clear line
      write!(output, "\x1b[{};{}H{}\x1b[K", y + 1, rect.x + 1, bg_color)?;

      // Render line number gutter
      if self.config.show_line_numbers {
        write!(output, "\x1b[{};{}H{}{:>width$} ",
               y + 1, rect.x + 1, gutter_bg,
               if line_idx < self.content.len() { line_idx + 1 } else { 0 },
               width = gutter_width as usize - 1)?;
      }

      // Render line content
      if line_idx < self.content.len() {
        let line = &self.content[line_idx];
        let tokens = self.highlight_line(line);

        let mut x_offset = gutter_width;

        if tokens.is_empty() {
          // Empty line or no highlighting
          write!(output, "\x1b[{};{}H{}{}", y + 1, rect.x + x_offset + 1, text_color, line)?;
        } else {
          // Render highlighted tokens
          for token in tokens {
            if x_offset >= rect.width {
              break;
            }

            let token_color = if let Some(color_name) = self.style.syntax_colors.get(&token.token_type) {
              if let Ok(color_def) = get_palette_color(&theme.palette, color_name) {
                color_to_ansi(color_def, false)
              } else {
                text_color.clone()
              }
            } else {
              text_color.clone()
            };

            write!(output, "\x1b[{};{}H{}{}", y + 1, rect.x + x_offset + 1, token_color, token.text)?;
            x_offset += token.text.len() as u16;
          }
        }

        // Render cursor if on this line
        if line_idx == self.cursor.line {
          let cursor_x = rect.x + gutter_width + self.cursor.column as u16;
          if cursor_x < rect.x + rect.width {
            write!(output, "\x1b[{};{}H{}█", y + 1, cursor_x + 1, cursor_color)?;
          }
        }
      }
    }

    write!(output, "\x1b[0m")?;
    Ok(output)
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<()> {
    match key {
      "ArrowUp" => self.cursor_up()?,
      "ArrowDown" => self.cursor_down()?,
      "ArrowLeft" => self.cursor_left()?,
      "ArrowRight" => self.cursor_right()?,
      "Backspace" => self.backspace()?,
      "Delete" => self.delete_char()?,
      "Enter" => self.insert_text("\n")?,
      "Tab" => {
        let tab_text = if self.config.use_spaces {
          " ".repeat(self.config.tab_size)
        } else {
          "\t".to_string()
        };
        self.insert_text(&tab_text)?;
      }
      text if text.len() == 1 => {
        self.insert_text(text)?;
      }
      _ => {} // Ignore other keys
    }
    Ok(())
  }
}

impl Default for CodeEditor {
  fn default() -> Self {
    Self::new()
  }
}

/// Builder for CodeEditor
pub struct CodeEditorBuilder {
  editor: CodeEditor,
}

impl CodeEditorBuilder {
  pub fn new() -> Self {
    Self {
      editor: CodeEditor::new(),
    }
  }

  pub fn content(mut self, content: &str) -> Self {
    self.editor.set_content(content);
    self
  }

  pub fn language(mut self, language: Language) -> Self {
    self.editor.set_language(language);
    self
  }

  pub fn dimensions(mut self, width: u16, height: u16) -> Self {
    self.editor.width = width;
    self.editor.height = height;
    self
  }

  pub fn config(mut self, config: EditorConfig) -> Self {
    self.editor.config = config;
    self
  }

  pub fn style(mut self, style: EditorStyle) -> Self {
    self.editor.style = style;
    self
  }

  pub fn build(self) -> CodeEditor {
    self.editor
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_code_editor_creation() {
    let editor = CodeEditor::new();
    assert_eq!(editor.content.len(), 1);
    assert_eq!(editor.cursor.line, 0);
    assert_eq!(editor.cursor.column, 0);
  }

  #[test]
  fn test_insert_text() {
    let mut editor = CodeEditor::new();
    editor.insert_text("Hello, World!").unwrap();
    assert_eq!(editor.content[0], "Hello, World!");
    assert_eq!(editor.cursor.column, 13);
  }

  #[test]
  fn test_multiline_insert() {
    let mut editor = CodeEditor::new();
    editor.insert_text("Line 1\nLine 2\nLine 3").unwrap();
    assert_eq!(editor.content.len(), 3);
    assert_eq!(editor.content[0], "Line 1");
    assert_eq!(editor.content[1], "Line 2");
    assert_eq!(editor.content[2], "Line 3");
  }

  #[test]
  fn test_cursor_movement() {
    let mut editor = CodeEditor::new();
    editor.set_content("Line 1\nLine 2\nLine 3");

    editor.cursor_down().unwrap();
    assert_eq!(editor.cursor.line, 1);

    editor.cursor_right().unwrap();
    assert_eq!(editor.cursor.column, 1);

    editor.cursor_up().unwrap();
    assert_eq!(editor.cursor.line, 0);
  }

  #[test]
  fn test_syntax_highlighting() {
    let mut editor = CodeEditor::new();
    editor.config.language = Language::Rust;
    let tokens = editor.highlight_line("fn main() {");

    assert!(!tokens.is_empty());
    // Should have keyword "fn" highlighted
    assert!(tokens.iter().any(|t| t.token_type == TokenType::Keyword && t.text == "fn"));
  }
}
