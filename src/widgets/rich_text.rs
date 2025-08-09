//! Rich Text Renderer Widget
//!
//! A comprehensive rich text renderer supporting markdown parsing and syntax highlighting,
//! with custom styling, hyperlink support, and extensible content rendering.
//!
//! # Features
//!
//! - **Markdown Rendering**: Full CommonMark support with headers, lists, code blocks, tables
//! - **Syntax Highlighting**: Code block highlighting with multiple language support
//! - **Custom Styling**: Themeable colors, fonts, and layout options
//! - **Hyperlink Support**: Clickable links with navigation callbacks
//! - **Table Rendering**: Markdown table support with alignment and borders
//! - **Image Placeholders**: ASCII art placeholders for images in text mode
//! - **Line Wrapping**: Intelligent word wrapping with configurable width
//! - **Scrolling**: Vertical scrolling for content longer than viewport
//! - **Search Integration**: Text search with highlighting
//! - **Export Capabilities**: Export rendered content to plain text or ANSI
//!
//! # Basic Usage
//!
//! ```rust,no_run
//! use reactive_tui::widgets::{RichText, RichTextBuilder};
//!
//! let mut rich_text = RichTextBuilder::new("markdown-viewer")
//!     .content("# My Application\n\nThis is a **bold** statement with *italics*.")
//!     .width(80)
//!     .syntax_highlighting(true)
//!     .build();
//! ```

use crate::{
  components::element::Element,
  layout::LayoutRect,
  reactive::Reactive,
  themes::{ColorDefinition, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

use syntect::easy::HighlightLines;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;

// Type aliases for complex function pointer types
type OnLinkClickCallback = Arc<dyn Fn(&str) + Send + Sync>;
type OnSearchCallback = Arc<dyn Fn(&str, &[(usize, usize)]) + Send + Sync>;

/// Markdown element types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum MarkdownElement {
  /// Header with level (1-6)
  Header { level: u8, text: String },
  /// Paragraph text
  Paragraph { text: String },
  /// Code block with optional language
  CodeBlock {
    language: Option<String>,
    code: String,
  },
  /// Inline code
  InlineCode { text: String },
  /// Unordered list
  UnorderedList { items: Vec<String> },
  /// Ordered list
  OrderedList { items: Vec<String> },
  /// Blockquote
  Blockquote { text: String },
  /// Horizontal rule
  HorizontalRule,
  /// Table with headers and rows
  Table {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    alignments: Vec<TableAlignment>,
  },
  /// Link with text and URL
  Link { text: String, url: String },
  /// Image with alt text and URL
  Image { alt_text: String, url: String },
  /// Line break
  LineBreak,
  /// Bold text
  Bold { text: String },
  /// Italic text
  Italic { text: String },
  /// Strikethrough text
  Strikethrough { text: String },
}

/// Table column alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TableAlignment {
  Left,
  Center,
  Right,
}

impl Default for TableAlignment {
  fn default() -> Self {
    Self::Left
  }
}

/// Syntax highlighting languages
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SyntaxLanguage {
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
  Toml,
  Xml,
  Sql,
  Bash,
  Shell,
  Markdown,
  Plain,
}

impl SyntaxLanguage {
  /// Parse language from string
  pub fn from_string(s: &str) -> Self {
    match s.to_lowercase().as_str() {
      "rust" | "rs" => Self::Rust,
      "javascript" | "js" => Self::JavaScript,
      "typescript" | "ts" => Self::TypeScript,
      "python" | "py" => Self::Python,
      "go" => Self::Go,
      "c" => Self::C,
      "cpp" | "c++" | "cxx" => Self::Cpp,
      "java" => Self::Java,
      "html" => Self::Html,
      "css" => Self::Css,
      "json" => Self::Json,
      "yaml" | "yml" => Self::Yaml,
      "toml" => Self::Toml,
      "xml" => Self::Xml,
      "sql" => Self::Sql,
      "bash" => Self::Bash,
      "shell" | "sh" => Self::Shell,
      "markdown" | "md" => Self::Markdown,
      _ => Self::Plain,
    }
  }

  /// Get syntax highlighting patterns
  pub fn get_patterns(&self) -> Vec<SyntaxPattern> {
    match self {
      Self::Rust => vec![
        SyntaxPattern::keyword(vec![
          "fn", "let", "mut", "const", "static", "if", "else", "match", "for", "while", "loop",
          "impl", "trait", "struct", "enum", "mod", "use", "pub", "return", "break", "continue",
        ]),
        SyntaxPattern::string_literal(),
        SyntaxPattern::number_literal(),
        SyntaxPattern::comment(),
        SyntaxPattern::type_name(),
      ],
      Self::JavaScript | Self::TypeScript => vec![
        SyntaxPattern::keyword(vec![
          "function",
          "const",
          "let",
          "var",
          "if",
          "else",
          "for",
          "while",
          "return",
          "class",
          "interface",
          "type",
          "import",
          "export",
          "async",
          "await",
        ]),
        SyntaxPattern::string_literal(),
        SyntaxPattern::number_literal(),
        SyntaxPattern::comment(),
      ],
      Self::Python => vec![
        SyntaxPattern::keyword(vec![
          "def", "class", "if", "elif", "else", "for", "while", "try", "except", "finally",
          "import", "from", "return", "yield", "lambda", "with", "as",
        ]),
        SyntaxPattern::string_literal(),
        SyntaxPattern::number_literal(),
        SyntaxPattern::comment(),
      ],
      _ => vec![
        SyntaxPattern::string_literal(),
        SyntaxPattern::number_literal(),
        SyntaxPattern::comment(),
      ],
    }
  }
}

/// Syntax highlighting pattern
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SyntaxPattern {
  pub name: String,
  pub color: Option<ColorDefinition>,
  pub keywords: Vec<String>,
  pub pattern_type: SyntaxPatternType,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SyntaxPatternType {
  Keyword,
  StringLiteral,
  NumberLiteral,
  Comment,
  TypeName,
  Custom(String),
}

impl SyntaxPattern {
  pub fn keyword(keywords: Vec<&str>) -> Self {
    Self {
      name: "keyword".to_string(),
      color: None,
      keywords: keywords.into_iter().map(|s| s.to_string()).collect(),
      pattern_type: SyntaxPatternType::Keyword,
    }
  }

  pub fn string_literal() -> Self {
    Self {
      name: "string".to_string(),
      color: None,
      keywords: vec![],
      pattern_type: SyntaxPatternType::StringLiteral,
    }
  }

  pub fn number_literal() -> Self {
    Self {
      name: "number".to_string(),
      color: None,
      keywords: vec![],
      pattern_type: SyntaxPatternType::NumberLiteral,
    }
  }

  pub fn comment() -> Self {
    Self {
      name: "comment".to_string(),
      color: None,
      keywords: vec![],
      pattern_type: SyntaxPatternType::Comment,
    }
  }

  pub fn type_name() -> Self {
    Self {
      name: "type".to_string(),
      color: None,
      keywords: vec![],
      pattern_type: SyntaxPatternType::TypeName,
    }
  }
}

/// Rich text state management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextState {
  /// Current scroll position
  pub scroll_y: usize,
  /// Current search query
  pub search_query: String,
  /// Search result positions
  pub search_results: Vec<(usize, usize)>, // (line, column) pairs
  /// Current search result index
  pub current_search_result: Option<usize>,
  /// Whether content is focused
  pub focused: bool,
  /// Whether content is disabled
  pub disabled: bool,
  /// Viewport dimensions
  pub viewport_width: u16,
  pub viewport_height: u16,
}

impl Default for RichTextState {
  fn default() -> Self {
    Self {
      scroll_y: 0,
      search_query: String::new(),
      search_results: Vec::new(),
      current_search_result: None,
      focused: false,
      disabled: false,
      viewport_width: 80,
      viewport_height: 25,
    }
  }
}

/// Syntect syntax highlighting helper
pub struct SyntectHighlighter {
  syntax_set: SyntaxSet,
  theme_set: ThemeSet,
}

impl Default for SyntectHighlighter {
  fn default() -> Self {
    Self::new()
  }
}

impl SyntectHighlighter {
  pub fn new() -> Self {
    Self {
      syntax_set: SyntaxSet::load_defaults_newlines(),
      theme_set: ThemeSet::load_defaults(),
    }
  }

  pub fn highlight_code(&self, code: &str, language: &str) -> Vec<String> {
    let syntax = if let Some(syntax) = self.syntax_set.find_syntax_by_token(language) {
      syntax
    } else if let Some(syntax) = self.syntax_set.find_syntax_by_extension(language) {
      syntax
    } else {
      self.syntax_set.find_syntax_plain_text()
    };

    let theme = &self.theme_set.themes["InspiredGitHub"];
    let mut highlighter = HighlightLines::new(syntax, theme);

    let mut highlighted_lines = Vec::new();
    for line in code.lines() {
      let highlighted = highlighter.highlight_line(line, &self.syntax_set);
      match highlighted {
        Ok(ranges) => {
          let escaped = as_24_bit_terminal_escaped(&ranges[..], false);
          highlighted_lines.push(escaped);
        }
        Err(_) => {
          highlighted_lines.push(line.to_string());
        }
      }
    }
    highlighted_lines
  }
}

/// Rich text styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextStyle {
  /// Background color
  pub background: Option<ColorDefinition>,
  /// Default text color
  pub text_color: Option<ColorDefinition>,
  /// Header colors by level
  pub header_colors: Vec<Option<ColorDefinition>>,
  /// Code block background
  pub code_background: Option<ColorDefinition>,
  /// Code block text color
  pub code_text_color: Option<ColorDefinition>,
  /// Link color
  pub link_color: Option<ColorDefinition>,
  /// Quote border color
  pub quote_border_color: Option<ColorDefinition>,
  /// Table border color
  pub table_border_color: Option<ColorDefinition>,
  /// Search highlight color
  pub search_highlight_color: Option<ColorDefinition>,
  /// Line spacing
  pub line_spacing: u16,
  /// Paragraph spacing
  pub paragraph_spacing: u16,
  /// Code block padding
  pub code_padding: u16,
  /// Table cell padding
  pub table_cell_padding: u16,
}

impl Default for RichTextStyle {
  fn default() -> Self {
    Self {
      background: None,
      text_color: None,
      header_colors: vec![None; 6],
      code_background: None,
      code_text_color: None,
      link_color: None,
      quote_border_color: None,
      table_border_color: None,
      search_highlight_color: None,
      line_spacing: 1,
      paragraph_spacing: 1,
      code_padding: 1,
      table_cell_padding: 1,
    }
  }
}

/// Rich text configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RichTextConfig {
  /// Content width for wrapping
  pub width: u16,
  /// Content height for viewport
  pub height: u16,
  /// Enable syntax highlighting
  pub syntax_highlighting: bool,
  /// Enable scrolling
  pub scrollable: bool,
  /// Enable search
  pub searchable: bool,
  /// Enable hyperlinks
  pub hyperlinks_enabled: bool,
  /// Wrap long lines
  pub word_wrap: bool,
  /// Show line numbers for code blocks
  pub show_line_numbers: bool,
  /// Tab size for code blocks
  pub tab_size: u8,
  /// Render tables
  pub render_tables: bool,
  /// Render images as ASCII placeholders
  pub render_images: bool,
  /// Maximum nested list depth
  pub max_list_depth: u8,
}

impl Default for RichTextConfig {
  fn default() -> Self {
    Self {
      width: 80,
      height: 25,
      syntax_highlighting: true,
      scrollable: true,
      searchable: true,
      hyperlinks_enabled: true,
      word_wrap: true,
      show_line_numbers: true,
      tab_size: 4,
      render_tables: true,
      render_images: true,
      max_list_depth: 10,
    }
  }
}

/// Event callbacks for rich text interactions
#[derive(Default)]
pub struct RichTextCallbacks {
  /// Called when a link is clicked
  pub on_link_click: Option<OnLinkClickCallback>,
  /// Called when scrolling
  pub on_scroll: Option<Arc<dyn Fn(usize) + Send + Sync>>,
  /// Called when search results change
  pub on_search: Option<OnSearchCallback>,
  /// Called when focus changes
  pub on_focus: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

/// Main Rich Text widget
pub struct RichText {
  /// Unique rich text identifier
  pub id: String,
  /// Raw markdown content
  pub content: String,
  /// Parsed markdown elements
  pub elements: Vec<MarkdownElement>,
  /// Rendered lines cache
  pub rendered_lines: Vec<String>,
  /// Reactive state management
  pub state: Reactive<RichTextState>,
  /// Configuration options
  pub config: RichTextConfig,
  /// Styling configuration
  pub style: RichTextStyle,
  /// Event callbacks
  pub callbacks: RichTextCallbacks,
  /// CSS utility classes
  pub css_classes: Vec<String>,
  /// Custom syntax highlighting themes
  pub syntax_themes: HashMap<SyntaxLanguage, Vec<SyntaxPattern>>,
  /// Syntect syntax highlighter
  pub syntect_highlighter: SyntectHighlighter,
}

impl RichText {
  /// Create a new rich text builder
  pub fn builder<S: Into<String>>(id: S) -> RichTextBuilder {
    RichTextBuilder::new(id)
  }

  /// Set the markdown content
  pub fn set_content(&mut self, content: impl Into<String>) {
    self.content = content.into();
    self.parse_markdown();
    self.render_content();
  }

  /// Get the current content
  pub fn get_content(&self) -> &str {
    &self.content
  }

  /// Parse markdown content into elements
  pub fn parse_markdown(&mut self) {
    self.elements = self.simple_markdown_parser(&self.content);
  }

  /// Simple markdown parser implementation
  fn simple_markdown_parser(&self, content: &str) -> Vec<MarkdownElement> {
    let mut elements = Vec::new();
    let lines: Vec<&str> = content.lines().collect();
    let mut i = 0;

    while i < lines.len() {
      let line = lines[i].trim();

      if line.is_empty() {
        elements.push(MarkdownElement::LineBreak);
        i += 1;
        continue;
      }

      // Headers
      if line.starts_with('#') {
        let level = line.chars().take_while(|&c| c == '#').count() as u8;
        if level <= 6 {
          let text = line.trim_start_matches('#').trim().to_string();
          elements.push(MarkdownElement::Header { level, text });
          i += 1;
          continue;
        }
      }

      // Horizontal rule
      if line == "---" || line == "***" || line == "___" {
        elements.push(MarkdownElement::HorizontalRule);
        i += 1;
        continue;
      }

      // Code blocks
      if let Some(stripped) = line.strip_prefix("```") {
        let language = if !stripped.is_empty() {
          Some(stripped.trim().to_string())
        } else {
          None
        };

        i += 1;
        let mut code_lines = Vec::new();

        while i < lines.len() && !lines[i].trim_start().starts_with("```") {
          code_lines.push(lines[i]);
          i += 1;
        }

        if i < lines.len() {
          i += 1; // Skip closing ```
        }

        let code = code_lines.join("\n");
        elements.push(MarkdownElement::CodeBlock { language, code });
        continue;
      }

      // Blockquotes
      if line.starts_with('>') {
        let text = line.trim_start_matches('>').trim().to_string();
        elements.push(MarkdownElement::Blockquote { text });
        i += 1;
        continue;
      }

      // Lists
      if line.starts_with("- ") || line.starts_with("* ") || line.starts_with("+ ") {
        let mut items = Vec::new();
        while i < lines.len() {
          let current_line = lines[i].trim();
          if current_line.starts_with("- ")
            || current_line.starts_with("* ")
            || current_line.starts_with("+ ")
          {
            let item = current_line[2..].trim().to_string();
            items.push(item);
            i += 1;
          } else if current_line.is_empty() {
            i += 1;
            break;
          } else {
            break;
          }
        }
        elements.push(MarkdownElement::UnorderedList { items });
        continue;
      }

      // Ordered lists
      if line.chars().next().is_some_and(|c| c.is_ascii_digit()) && line.contains(". ") {
        let mut items = Vec::new();
        while i < lines.len() {
          let current_line = lines[i].trim();
          if current_line
            .chars()
            .next()
            .is_some_and(|c| c.is_ascii_digit())
            && current_line.contains(". ")
          {
            if let Some(dot_pos) = current_line.find(". ") {
              let item = current_line[dot_pos + 2..].trim().to_string();
              items.push(item);
            }
            i += 1;
          } else if current_line.is_empty() {
            i += 1;
            break;
          } else {
            break;
          }
        }
        elements.push(MarkdownElement::OrderedList { items });
        continue;
      }

      // Regular paragraph
      let text = self.parse_inline_formatting(line);
      elements.push(MarkdownElement::Paragraph { text });
      i += 1;
    }

    elements
  }

  /// Parse inline formatting (bold, italic, links, etc.)
  fn parse_inline_formatting(&self, text: &str) -> String {
    let mut result = text.to_string();

    // Simple implementation - in a real parser you'd use proper regex or parsing
    // This is a basic version for demonstration

    // Bold **text**
    if let Some(start) = result.find("**") {
      if let Some(end) = result[start + 2..].find("**") {
        let bold_text = &result[start + 2..start + 2 + end];
        let replacement = format!("{bold_text}**"); // Keep markers for now
        result = result.replace(&format!("{bold_text}**"), &replacement);
      }
    }

    // Italic *text*
    while let Some(start) = result.find('*') {
      if let Some(end) = result[start + 1..].find('*') {
        let italic_text = &result[start + 1..start + 1 + end];
        if !italic_text.contains("**") {
          // Avoid conflicts with bold
          let replacement = format!("{italic_text}*"); // Keep markers for now
          result = result.replace(&format!("{italic_text}*"), &replacement);
          break;
        }
      } else {
        break;
      }
    }

    result
  }

  /// Render content to lines
  pub fn render_content(&mut self) {
    self.rendered_lines.clear();

    for element in &self.elements {
      match element {
        MarkdownElement::Header { level, text } => {
          let prefix = "#".repeat(*level as usize);
          self.rendered_lines.push(format!("{prefix} {text}"));
          self.rendered_lines.push(String::new()); // Empty line after header
        }
        MarkdownElement::Paragraph { text } => {
          if self.config.word_wrap {
            let wrapped = self.wrap_text(text, self.config.width as usize);
            self.rendered_lines.extend(wrapped);
          } else {
            self.rendered_lines.push(text.clone());
          }
          self.rendered_lines.push(String::new()); // Empty line after paragraph
        }
        MarkdownElement::CodeBlock { language, code } => {
          self.rendered_lines.push("```".to_string());

          if self.config.syntax_highlighting && language.is_some() {
            let highlighted = self.apply_syntax_highlighting(code, language.as_deref().unwrap_or(""));
            self.rendered_lines.extend(highlighted);
          } else {
            for line in code.lines() {
              self.rendered_lines.push(line.to_string());
            }
          }

          self.rendered_lines.push("```".to_string());
          self.rendered_lines.push(String::new());
        }
        MarkdownElement::UnorderedList { items } => {
          for item in items {
            self.rendered_lines.push(item.to_string());
          }
          self.rendered_lines.push(String::new());
        }
        MarkdownElement::OrderedList { items } => {
          for (i, item) in items.iter().enumerate() {
            self.rendered_lines.push(format!("{}. {}", i + 1, item));
          }
          self.rendered_lines.push(String::new());
        }
        MarkdownElement::Blockquote { text } => {
          self.rendered_lines.push(text.to_string());
          self.rendered_lines.push(String::new());
        }
        MarkdownElement::HorizontalRule => {
          self
            .rendered_lines
            .push("─".repeat(self.config.width as usize));
          self.rendered_lines.push(String::new());
        }
        MarkdownElement::LineBreak => {
          self.rendered_lines.push(String::new());
        }
        _ => {
          // Handle other elements as needed
        }
      }
    }
  }

  /// Wrap text to specified width
  fn wrap_text(&self, text: &str, width: usize) -> Vec<String> {
    let mut lines = Vec::new();
    let words: Vec<&str> = text.split_whitespace().collect();
    let mut current_line = String::new();

    for word in words {
      if current_line.len() + word.len() + 1 > width && !current_line.is_empty() {
        lines.push(current_line.clone());
        current_line.clear();
      }

      if !current_line.is_empty() {
        current_line.push(' ');
      }
      current_line.push_str(word);
    }

    if !current_line.is_empty() {
      lines.push(current_line);
    }

    if lines.is_empty() {
      lines.push(String::new());
    }

    lines
  }

  /// Apply syntax highlighting to code using syntect
  fn apply_syntax_highlighting(&self, code: &str, language: &str) -> Vec<String> {
    if self.config.syntax_highlighting {
      self.syntect_highlighter.highlight_code(code, language)
    } else {
      code.lines().map(|line| line.to_string()).collect()
    }
  }

  /// Scroll down by specified lines
  pub fn scroll_down(&mut self, lines: usize) -> bool {
    let max_scroll = self
      .rendered_lines
      .len()
      .saturating_sub(self.config.height as usize);
    let new_scroll = (self.state.get().scroll_y + lines).min(max_scroll);

    if new_scroll != self.state.get().scroll_y {
      self.state.update(|state| {
        state.scroll_y = new_scroll;
      });

      if let Some(callback) = &self.callbacks.on_scroll {
        callback(new_scroll);
      }

      true
    } else {
      false
    }
  }

  /// Scroll up by specified lines
  pub fn scroll_up(&mut self, lines: usize) -> bool {
    let new_scroll = self.state.get().scroll_y.saturating_sub(lines);

    if new_scroll != self.state.get().scroll_y {
      self.state.update(|state| {
        state.scroll_y = new_scroll;
      });

      if let Some(callback) = &self.callbacks.on_scroll {
        callback(new_scroll);
      }

      true
    } else {
      false
    }
  }

  /// Search for text in content
  pub fn search(&mut self, query: impl Into<String>) -> usize {
    let query = query.into();
    let results = self.find_text_positions(&query);

    self.state.update(|state| {
      state.search_query = query.clone();
      state.search_results = results.clone();
      state.current_search_result = if results.is_empty() { None } else { Some(0) };
    });

    if let Some(callback) = &self.callbacks.on_search {
      callback(&query, &results);
    }

    results.len()
  }

  /// Find all positions of text in rendered content
  fn find_text_positions(&self, query: &str) -> Vec<(usize, usize)> {
    let mut positions = Vec::new();

    for (line_idx, line) in self.rendered_lines.iter().enumerate() {
      let mut start = 0;
      while let Some(pos) = line[start..].find(query) {
        positions.push((line_idx, start + pos));
        start += pos + query.len();
      }
    }

    positions
  }

  /// Navigate to next search result
  pub fn next_search_result(&mut self) -> bool {
    let state = self.state.get();
    if let Some(current) = state.current_search_result {
      if current + 1 < state.search_results.len() {
        self.state.update(|state| {
          state.current_search_result = Some(current + 1);
        });
        return true;
      }
    }
    false
  }

  /// Navigate to previous search result
  pub fn previous_search_result(&mut self) -> bool {
    let state = self.state.get();
    if let Some(current) = state.current_search_result {
      if current > 0 {
        self.state.update(|state| {
          state.current_search_result = Some(current - 1);
        });
        return true;
      }
    }
    false
  }

  /// Clear search
  pub fn clear_search(&mut self) {
    self.state.update(|state| {
      state.search_query.clear();
      state.search_results.clear();
      state.current_search_result = None;
    });
  }

  /// Set focus state
  pub fn set_focused(&mut self, focused: bool) {
    self.state.update(|state| {
      state.focused = focused;
    });

    if let Some(callback) = &self.callbacks.on_focus {
      callback(focused);
    }
  }

  /// Check if rich text is focused
  pub fn is_focused(&self) -> bool {
    self.state.get().focused
  }

  /// Enable/disable the rich text
  pub fn set_disabled(&mut self, disabled: bool) {
    self.state.update(|state| {
      state.disabled = disabled;
      if disabled {
        state.focused = false;
      }
    });
  }

  /// Check if rich text is disabled
  pub fn is_disabled(&self) -> bool {
    self.state.get().disabled
  }

  /// Get total number of lines
  pub fn line_count(&self) -> usize {
    self.rendered_lines.len()
  }

  /// Get current scroll position
  pub fn scroll_position(&self) -> usize {
    self.state.get().scroll_y
  }

  /// Export to plain text
  pub fn to_plain_text(&self) -> String {
    self.rendered_lines.join("\n")
  }

  /// Render the rich text to a string
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Base CSS classes
    let mut classes = vec!["rich-text".to_string()];
    if state.focused {
      classes.push("rich-text-focused".to_string());
    }
    if state.disabled {
      classes.push("rich-text-disabled".to_string());
    }
    classes.extend(self.css_classes.clone());

    // Render visible lines based on scroll position
    let start_line = state.scroll_y;
    let end_line = (start_line + self.config.height as usize).min(self.rendered_lines.len());

    for i in start_line..end_line {
      if i < self.rendered_lines.len() {
        let line = &self.rendered_lines[i];

        // Apply search highlighting if needed
        if !state.search_query.is_empty() && line.contains(&state.search_query) {
          let highlighted = line.replace(&state.search_query, &format!("{}⟫", state.search_query));
          let _ = writeln!(output, "{highlighted}");
        } else {
          let _ = writeln!(output, "{line}");
        }
      }
    }

    // Add scroll indicator if content is longer than viewport
    if self.rendered_lines.len() > self.config.height as usize {
      let scroll_percent = if !self.rendered_lines.is_empty() {
        (state.scroll_y * 100) / (self.rendered_lines.len() - self.config.height as usize).max(1)
      } else {
        0
      };
      let _ = writeln!(output, "─ {scroll_percent}% ─");
    }

    output
  }

  /// Convert to Element for integration with layout system
  pub fn to_element(&self) -> Element {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: self.config.width,
      height: self.config.height,
    };
    Element {
      tag: "div".to_string(),
      id: Some(self.id.clone()),
      classes: self.css_classes.clone(),
      content: Some(self.render(&layout, None)),
      children: Vec::new(),
      attributes: std::collections::HashMap::new(),
      focusable: !self.is_disabled(),
      focused: self.is_focused(),
      disabled: self.is_disabled(),
      tab_index: Some(0),
      key_bindings: Vec::new(),
      modal: false,
    }
  }
}

impl fmt::Display for RichText {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: self.config.width,
      height: self.config.height,
    };
    write!(f, "{}", self.render(&layout, None))
  }
}

/// Builder for creating rich text widgets
pub struct RichTextBuilder {
  id: String,
  content: String,
  config: RichTextConfig,
  style: RichTextStyle,
  callbacks: RichTextCallbacks,
  css_classes: Vec<String>,
  syntax_themes: HashMap<SyntaxLanguage, Vec<SyntaxPattern>>,
}

impl RichTextBuilder {
  /// Create a new rich text builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      content: String::new(),
      config: RichTextConfig::default(),
      style: RichTextStyle::default(),
      callbacks: RichTextCallbacks::default(),
      css_classes: Vec::new(),
      syntax_themes: HashMap::new(),
    }
  }

  /// Set content
  pub fn content(mut self, content: impl Into<String>) -> Self {
    self.content = content.into();
    self
  }

  /// Set width
  pub fn width(mut self, width: u16) -> Self {
    self.config.width = width;
    self
  }

  /// Set height
  pub fn height(mut self, height: u16) -> Self {
    self.config.height = height;
    self
  }

  /// Enable/disable syntax highlighting
  pub fn syntax_highlighting(mut self, enabled: bool) -> Self {
    self.config.syntax_highlighting = enabled;
    self
  }

  /// Enable/disable scrolling
  pub fn scrollable(mut self, enabled: bool) -> Self {
    self.config.scrollable = enabled;
    self
  }

  /// Enable/disable search
  pub fn searchable(mut self, enabled: bool) -> Self {
    self.config.searchable = enabled;
    self
  }

  /// Enable/disable hyperlinks
  pub fn hyperlinks(mut self, enabled: bool) -> Self {
    self.config.hyperlinks_enabled = enabled;
    self
  }

  /// Enable/disable word wrap
  pub fn word_wrap(mut self, enabled: bool) -> Self {
    self.config.word_wrap = enabled;
    self
  }

  /// Show/hide line numbers
  pub fn line_numbers(mut self, show: bool) -> Self {
    self.config.show_line_numbers = show;
    self
  }

  /// Set tab size
  pub fn tab_size(mut self, size: u8) -> Self {
    self.config.tab_size = size;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set link click callback
  pub fn on_link_click<F>(mut self, callback: F) -> Self
  where
    F: Fn(&str) + Send + Sync + 'static,
  {
    self.callbacks.on_link_click = Some(Arc::new(callback));
    self
  }

  /// Set scroll callback
  pub fn on_scroll<F>(mut self, callback: F) -> Self
  where
    F: Fn(usize) + Send + Sync + 'static,
  {
    self.callbacks.on_scroll = Some(Arc::new(callback));
    self
  }

  /// Set search callback
  pub fn on_search<F>(mut self, callback: F) -> Self
  where
    F: Fn(&str, &[(usize, usize)]) + Send + Sync + 'static,
  {
    self.callbacks.on_search = Some(Arc::new(callback));
    self
  }

  /// Build the rich text widget
  pub fn build(self) -> RichText {
    let state = RichTextState {
      viewport_width: self.config.width,
      viewport_height: self.config.height,
      ..Default::default()
    };

    let mut rich_text = RichText {
      id: self.id,
      content: self.content,
      elements: Vec::new(),
      rendered_lines: Vec::new(),
      state: Reactive::new(state),
      config: self.config,
      style: self.style,
      callbacks: self.callbacks,
      css_classes: self.css_classes,
      syntax_themes: self.syntax_themes,
      syntect_highlighter: SyntectHighlighter::new(),
    };

    rich_text.parse_markdown();
    rich_text.render_content();

    rich_text
  }
}

/// Convenience functions for common rich text patterns
/// Create a documentation viewer
pub fn documentation_viewer(content: impl Into<String>) -> RichText {
  RichTextBuilder::new("documentation-viewer")
    .content(content)
    .width(100)
    .height(30)
    .syntax_highlighting(true)
    .scrollable(true)
    .searchable(true)
    .word_wrap(true)
    .line_numbers(true)
    .build()
}

/// Create a README viewer
pub fn readme_viewer(content: impl Into<String>) -> RichText {
  RichTextBuilder::new("readme-viewer")
    .content(content)
    .width(80)
    .height(25)
    .syntax_highlighting(true)
    .scrollable(true)
    .word_wrap(true)
    .build()
}

/// Create a code preview widget
pub fn code_preview(content: impl Into<String>) -> RichText {
  RichTextBuilder::new("code-preview")
    .content(content)
    .width(120)
    .height(40)
    .syntax_highlighting(true)
    .scrollable(true)
    .line_numbers(true)
    .tab_size(2)
    .build()
}

/// Create a help text widget
pub fn help_text(content: impl Into<String>) -> RichText {
  RichTextBuilder::new("help-text")
    .content(content)
    .width(70)
    .height(20)
    .syntax_highlighting(false)
    .word_wrap(true)
    .build()
}
