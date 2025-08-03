/*!
 * CSS-Styled Input Widget
 *
 * A comprehensive input field widget that integrates with the CSS styling system
 * and theme framework. Supports various input types, validation, and styling.
 */

use crate::layout::LayoutRect;
use crate::themes::{
  get_border_set, get_semantic_background, get_semantic_color, BorderStyle, ColorTheme,
  UtilityProcessor,
};
use crate::widgets::factory::WidgetConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Input field types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
  /// Single-line text input
  Text,
  /// Password input (masked)
  Password,
  /// Numeric input
  Number,
  /// Email input with validation
  Email,
  /// Search input
  Search,
  /// Multi-line text area
  TextArea,
}

/// Input validation states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationState {
  /// No validation performed yet
  None,
  /// Input is valid
  Valid,
  /// Input has validation errors
  Invalid,
  /// Currently validating (async)
  Validating,
}

/// Input field styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputStyle {
  /// Border style for the input
  pub border_style: BorderStyle,
  /// Whether to show a border
  pub show_border: bool,
  /// Padding inside the input
  pub padding: (u16, u16), // (horizontal, vertical)
  /// Minimum width of the input
  pub min_width: u16,
  /// Maximum width of the input (0 = unlimited)
  pub max_width: u16,
  /// Height for multi-line inputs
  pub height: u16,
  /// Whether to show placeholder text
  pub show_placeholder: bool,
  /// Cursor character
  pub cursor_char: char,
  /// Whether cursor blinks
  pub cursor_blink: bool,
}

/// Input field state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct InputState {
  /// Current input value
  pub value: String,
  /// Cursor position in the value
  pub cursor_position: usize,
  /// Whether the input has focus
  pub focused: bool,
  /// Current validation state
  pub validation_state: ValidationState,
  /// Validation error message
  pub validation_message: Option<String>,
  /// Whether the input is disabled
  pub disabled: bool,
  /// Whether the input is read-only
  pub readonly: bool,
  /// Selection start (for text selection)
  pub selection_start: Option<usize>,
  /// Selection end (for text selection)
  pub selection_end: Option<usize>,
  /// Scroll offset for long text
  pub scroll_offset: usize,
}

/// CSS-styled input field widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Input {
  /// Unique identifier for the input
  pub id: String,
  /// Input type
  pub input_type: InputType,
  /// Current state
  pub state: InputState,
  /// Styling configuration
  pub style: InputStyle,
  /// Placeholder text
  pub placeholder: String,
  /// CSS class names
  pub css_classes: Vec<String>,
  /// Inline CSS styles
  pub inline_styles: HashMap<String, String>,
  /// Theme reference
  pub theme: Option<String>,
  /// Validation function name/reference
  pub validator: Option<String>,
  /// Maximum length of input
  pub max_length: Option<usize>,
  /// Minimum length of input
  pub min_length: Option<usize>,
  /// Pattern for validation (regex)
  pub pattern: Option<String>,
  /// Whether input is required
  pub required: bool,
  /// Auto-complete suggestions
  pub autocomplete: Vec<String>,
}

/// Configuration for creating Input widgets through the WidgetFactory
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputConfig {
  /// Unique identifier for the input
  pub id: String,
  /// Input type
  pub input_type: InputType,
  /// Placeholder text
  pub placeholder: String,
  /// CSS class names
  pub classes: Vec<String>,
  /// HTML-like attributes
  pub attributes: HashMap<String, String>,
  /// Whether input is disabled
  pub disabled: bool,
  /// Whether input is visible
  pub visible: bool,
  /// Whether input can receive focus
  pub focusable: bool,
  /// Tab index for keyboard navigation
  pub tab_index: Option<i32>,
  /// Theme reference
  pub theme: Option<String>,
  /// Validation function name/reference
  pub validator: Option<String>,
  /// Maximum length of input
  pub max_length: Option<usize>,
  /// Minimum length of input
  pub min_length: Option<usize>,
  /// Pattern for validation (regex)
  pub pattern: Option<String>,
  /// Whether input is required
  pub required: bool,
  /// Auto-complete suggestions
  pub autocomplete: Vec<String>,
  /// Initial value
  pub value: String,
}

impl Default for InputConfig {
  fn default() -> Self {
    Self {
      id: String::new(),
      input_type: InputType::Text,
      placeholder: String::new(),
      classes: Vec::new(),
      attributes: HashMap::new(),
      disabled: false,
      visible: true,
      focusable: true,
      tab_index: None,
      theme: None,
      validator: None,
      max_length: None,
      min_length: None,
      pattern: None,
      required: false,
      autocomplete: Vec::new(),
      value: String::new(),
    }
  }
}

impl WidgetConfig for InputConfig {
  fn id(&self) -> &str {
    &self.id
  }

  fn widget_type(&self) -> &str {
    "input"
  }

  fn classes(&self) -> &[String] {
    &self.classes
  }

  fn attributes(&self) -> &HashMap<String, String> {
    &self.attributes
  }

  fn disabled(&self) -> bool {
    self.disabled
  }

  fn visible(&self) -> bool {
    self.visible
  }

  fn focusable(&self) -> bool {
    self.focusable
  }

  fn tab_index(&self) -> Option<i32> {
    self.tab_index
  }
}

impl InputConfig {
  /// Create a new input configuration
  pub fn new(id: &str) -> Self {
    Self {
      id: id.to_string(),
      ..Default::default()
    }
  }

  /// Set input type
  pub fn input_type(mut self, input_type: InputType) -> Self {
    self.input_type = input_type;
    self
  }

  /// Set placeholder text
  pub fn placeholder(mut self, placeholder: &str) -> Self {
    self.placeholder = placeholder.to_string();
    self
  }

  /// Set initial value
  pub fn value(mut self, value: &str) -> Self {
    self.value = value.to_string();
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: &str) -> Self {
    self.classes.push(class.to_string());
    self
  }

  /// Add multiple CSS classes
  pub fn classes(mut self, classes: &[&str]) -> Self {
    for class in classes {
      self.classes.push(class.to_string());
    }
    self
  }

  /// Set attribute
  pub fn attribute(mut self, key: &str, value: &str) -> Self {
    self.attributes.insert(key.to_string(), value.to_string());
    self
  }

  /// Set disabled state
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Set required state
  pub fn required(mut self, required: bool) -> Self {
    self.required = required;
    self
  }

  /// Set maximum length
  pub fn max_length(mut self, max_length: usize) -> Self {
    self.max_length = Some(max_length);
    self
  }

  /// Set minimum length
  pub fn min_length(mut self, min_length: usize) -> Self {
    self.min_length = Some(min_length);
    self
  }

  /// Set validation pattern
  pub fn pattern(mut self, pattern: &str) -> Self {
    self.pattern = Some(pattern.to_string());
    self
  }

  /// Set validator
  pub fn validator(mut self, validator: &str) -> Self {
    self.validator = Some(validator.to_string());
    self
  }

  /// Add autocomplete suggestion
  pub fn autocomplete(mut self, suggestion: &str) -> Self {
    self.autocomplete.push(suggestion.to_string());
    self
  }

  /// Set theme
  pub fn theme(mut self, theme: &str) -> Self {
    self.theme = Some(theme.to_string());
    self
  }

  /// Build an Input from this configuration
  pub fn build(self) -> Input {
    let mut state = InputState {
      value: self.value,
      cursor_position: 0,
      focused: false,
      validation_state: ValidationState::None,
      validation_message: None,
      disabled: self.disabled,
      readonly: false,
      selection_start: None,
      selection_end: None,
      scroll_offset: 0,
    };

    // Set cursor to end of initial value
    state.cursor_position = state.value.len();

    Input {
      id: self.id,
      input_type: self.input_type,
      state,
      style: InputStyle::default(),
      placeholder: self.placeholder,
      css_classes: self.classes,
      inline_styles: HashMap::new(),
      theme: self.theme,
      validator: self.validator,
      max_length: self.max_length,
      min_length: self.min_length,
      pattern: self.pattern,
      required: self.required,
      autocomplete: self.autocomplete,
    }
  }
}

/// Input field builder for fluent API
pub struct InputBuilder {
  input: Input,
}

impl Input {
  /// Create a new input with default settings
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      input_type: InputType::Text,
      state: InputState::default(),
      style: InputStyle::default(),
      placeholder: String::new(),
      css_classes: Vec::new(),
      inline_styles: HashMap::new(),
      theme: None,
      validator: None,
      max_length: None,
      min_length: None,
      pattern: None,
      required: false,
      autocomplete: Vec::new(),
    }
  }

  /// Create a builder for fluent API
  pub fn builder(id: impl Into<String>) -> InputBuilder {
    InputBuilder {
      input: Self::new(id),
    }
  }

  /// Set the input value
  pub fn set_value(&mut self, value: impl Into<String>) {
    self.state.value = value.into();
    self.state.cursor_position = self.state.value.len();
    self.validate();
  }

  /// Get the current input value
  pub fn value(&self) -> &str {
    &self.state.value
  }

  /// Set focus state
  pub fn set_focused(&mut self, focused: bool) {
    self.state.focused = focused;
  }

  /// Check if input has focus
  pub fn is_focused(&self) -> bool {
    self.state.focused
  }

  /// Insert character at cursor position
  pub fn insert_char(&mut self, ch: char) {
    if self.state.disabled || self.state.readonly {
      return;
    }

    if let Some(max_len) = self.max_length {
      if self.state.value.len() >= max_len {
        return;
      }
    }

    self.state.value.insert(self.state.cursor_position, ch);
    self.state.cursor_position += 1;
    self.validate();
  }

  /// Delete character at cursor position
  pub fn delete_char(&mut self) {
    if self.state.disabled || self.state.readonly || self.state.cursor_position == 0 {
      return;
    }

    self.state.cursor_position -= 1;
    self.state.value.remove(self.state.cursor_position);
    self.validate();
  }

  /// Move cursor left
  pub fn move_cursor_left(&mut self) {
    if self.state.cursor_position > 0 {
      self.state.cursor_position -= 1;
    }
  }

  /// Move cursor right
  pub fn move_cursor_right(&mut self) {
    if self.state.cursor_position < self.state.value.len() {
      self.state.cursor_position += 1;
    }
  }

  /// Move cursor to start
  pub fn move_cursor_home(&mut self) {
    self.state.cursor_position = 0;
  }

  /// Move cursor to end
  pub fn move_cursor_end(&mut self) {
    self.state.cursor_position = self.state.value.len();
  }

  /// Clear the input
  pub fn clear(&mut self) {
    if !self.state.disabled && !self.state.readonly {
      self.state.value.clear();
      self.state.cursor_position = 0;
      self.validate();
    }
  }

  /// Validate the current input value
  pub fn validate(&mut self) {
    if self.state.value.is_empty() {
      if self.required {
        self.state.validation_state = ValidationState::Invalid;
        self.state.validation_message = Some("This field is required".to_string());
        return;
      } else {
        self.state.validation_state = ValidationState::None;
        self.state.validation_message = None;
        return;
      }
    }

    // Length validation
    if let Some(min_len) = self.min_length {
      if self.state.value.len() < min_len {
        self.state.validation_state = ValidationState::Invalid;
        self.state.validation_message = Some(format!("Minimum length is {min_len}"));
        return;
      }
    }

    if let Some(max_len) = self.max_length {
      if self.state.value.len() > max_len {
        self.state.validation_state = ValidationState::Invalid;
        self.state.validation_message = Some(format!("Maximum length is {max_len}"));
        return;
      }
    }

    // Type-specific validation
    match self.input_type {
      InputType::Email => {
        if !self.is_valid_email(&self.state.value) {
          self.state.validation_state = ValidationState::Invalid;
          self.state.validation_message = Some("Invalid email format".to_string());
          return;
        }
      }
      InputType::Number => {
        if self.state.value.parse::<f64>().is_err() {
          self.state.validation_state = ValidationState::Invalid;
          self.state.validation_message = Some("Invalid number format".to_string());
          return;
        }
      }
      _ => {}
    }

    // Pattern validation
    if let Some(pattern) = &self.pattern {
      #[cfg(not(target_family = "wasm"))]
      use regex::Regex;

      #[cfg(target_family = "wasm")]
      use regex_lite::Regex;
      match Regex::new(pattern) {
        Ok(re) => {
          if !re.is_match(&self.state.value) {
            self.state.validation_state = ValidationState::Invalid;
            self.state.validation_message = Some("Invalid format".to_string());
            return;
          }
        }
        Err(_) => {
          // If the pattern itself is invalid, log warning but don't fail validation
          eprintln!("Warning: Invalid regex pattern in input validation: {pattern}");
        }
      }
    }

    self.state.validation_state = ValidationState::Valid;
    self.state.validation_message = None;
  }

  /// Simple email validation (basic check)
  fn is_valid_email(&self, email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
  }

  /// Render the input field with utility CSS classes and theme support
  pub fn render_with_utilities(
    &self,
    layout: &LayoutRect,
    utility_processor: &UtilityProcessor,
  ) -> String {
    let mut output = String::new();

    // Process utility classes
    let class_styles = utility_processor.process_classes(&self.css_classes);

    // Apply base styles
    output.push_str(&class_styles);

    // Get spacing from utility classes
    let padding_h = self
      .css_classes
      .iter()
      .find_map(|class| utility_processor.get_spacing(class))
      .unwrap_or(self.style.padding.0);

    let padding_v = self
      .css_classes
      .iter()
      .find_map(|class| {
        if class.starts_with("py-") {
          utility_processor.get_spacing(class)
        } else {
          None
        }
      })
      .unwrap_or(self.style.padding.1);

    // Determine border style from classes
    let show_border = self
      .css_classes
      .iter()
      .any(|class| utility_processor.is_border_class(class))
      || self.style.show_border;
    let border_set = get_border_set(self.style.border_style);

    // Calculate content area
    let content_width = layout
      .width
      .saturating_sub(if show_border { 2 } else { 0 })
      .saturating_sub(padding_h * 2);

    // Determine display text
    let display_text = if self.state.value.is_empty() && self.style.show_placeholder {
      &self.placeholder
    } else if self.input_type == InputType::Password {
      &"*".repeat(self.state.value.len())
    } else {
      &self.state.value
    };

    // Handle text overflow with scrolling
    let visible_text = if display_text.len() > content_width as usize {
      let start = self
        .state
        .scroll_offset
        .min(display_text.len().saturating_sub(content_width as usize));
      let end = (start + content_width as usize).min(display_text.len());
      &display_text[start..end]
    } else {
      display_text
    };

    // Render with Tailwind styling
    if show_border {
      // Top border
      output.push(border_set.top_left);
      output.push_str(
        &border_set
          .horizontal
          .to_string()
          .repeat(layout.width as usize - 2),
      );
      output.push(border_set.top_right);
      output.push('\n');
    }

    // Content area
    for row in 0..layout
      .height
      .saturating_sub(if show_border { 2 } else { 0 })
    {
      if show_border {
        output.push(border_set.vertical);
      }

      if row < padding_v
        || row
          >= layout
            .height
            .saturating_sub(padding_v + if show_border { 2 } else { 0 })
      {
        // Padding rows
        output.push_str(&" ".repeat(content_width as usize + padding_h as usize * 2));
      } else {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));

        // Add text with cursor
        let mut text_with_cursor = visible_text.to_string();
        if self.state.focused && !self.state.disabled {
          let cursor_pos = self
            .state
            .cursor_position
            .saturating_sub(self.state.scroll_offset);
          if cursor_pos <= text_with_cursor.len() {
            text_with_cursor.insert(cursor_pos, self.style.cursor_char);
          }
        }

        // Pad to content width
        let text_len = text_with_cursor.chars().count();
        if text_len < content_width as usize {
          text_with_cursor.push_str(&" ".repeat(content_width as usize - text_len));
        }

        output.push_str(&text_with_cursor[..(content_width as usize).min(text_with_cursor.len())]);
        output.push_str(&" ".repeat(padding_h as usize));
      }

      if show_border {
        output.push(border_set.vertical);
      }

      if row
        < layout
          .height
          .saturating_sub(if show_border { 2 } else { 0 })
          - 1
      {
        output.push('\n');
      }
    }

    if show_border {
      // Bottom border
      output.push('\n');
      output.push(border_set.bottom_left);
      output.push_str(
        &border_set
          .horizontal
          .to_string()
          .repeat(layout.width as usize - 2),
      );
      output.push(border_set.bottom_right);
    }

    // Reset styles
    output.push_str("\x1B[0m");

    // Validation message
    if let Some(message) = &self.state.validation_message {
      output.push('\n');
      // Use red color from utility processor if available
      if let Some(red_color) = utility_processor.get_color("red-500") {
        output.push_str(&format!(
          "\x1b[38;2;{};{};{}m",
          red_color.r, red_color.g, red_color.b
        ));
      }
      output.push_str("⚠ ");
      output.push_str(message);
      output.push_str("\x1B[0m");
    }

    output
  }

  /// Render the input field with CSS styling and theme support (legacy method)
  pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();

    // Get colors from theme
    let (bg_color, text_color, border_color, focus_color, error_color) = if let Some(theme) = theme
    {
      (
        get_semantic_background(theme, "input_background").unwrap_or_default(),
        get_semantic_color(theme, "input_text").unwrap_or_default(),
        get_semantic_color(theme, "input_border").unwrap_or_default(),
        get_semantic_color(theme, "input_focus").unwrap_or_default(),
        get_semantic_color(theme, "error").unwrap_or_default(),
      )
    } else {
      (
        String::new(),
        String::new(),
        String::new(),
        String::new(),
        String::new(),
      )
    };

    // Choose border color based on state
    let current_border_color = if self.state.validation_state == ValidationState::Invalid {
      &error_color
    } else if self.state.focused {
      &focus_color
    } else {
      &border_color
    };

    // Get border characters
    let border_set = get_border_set(self.style.border_style);

    // Calculate visible content area
    let content_width = layout
      .width
      .saturating_sub(if self.style.show_border { 2 } else { 0 })
      .saturating_sub(self.style.padding.0 * 2);

    // Determine what text to show
    let display_text = if self.state.value.is_empty() && self.style.show_placeholder {
      &self.placeholder
    } else if self.input_type == InputType::Password {
      &"*".repeat(self.state.value.len())
    } else {
      &self.state.value
    };

    // Calculate scroll offset for long text
    let visible_text = if display_text.len() > content_width as usize {
      let start = self
        .state
        .scroll_offset
        .min(display_text.len().saturating_sub(content_width as usize));
      let end = (start + content_width as usize).min(display_text.len());
      &display_text[start..end]
    } else {
      display_text
    };

    // Render top border
    if self.style.show_border {
      output.push_str(current_border_color);
      output.push(border_set.top_left);
      output.push_str(
        &border_set
          .horizontal
          .to_string()
          .repeat(layout.width as usize - 2),
      );
      output.push(border_set.top_right);
      output.push_str("\x1B[0m\n");
    }

    // Render content area
    let padding_v = if self.style.show_border {
      self.style.padding.1
    } else {
      0
    };

    for row in 0..layout
      .height
      .saturating_sub(if self.style.show_border { 2 } else { 0 })
    {
      if self.style.show_border {
        output.push_str(current_border_color);
        output.push(border_set.vertical);
        output.push_str("\x1B[0m");
      }

      // Background color
      output.push_str(&bg_color);

      if row < padding_v
        || row
          >= layout
            .height
            .saturating_sub(padding_v + if self.style.show_border { 2 } else { 0 })
      {
        // Padding rows
        output.push_str(&" ".repeat(content_width as usize));
      } else {
        // Content row
        output.push_str(&" ".repeat(self.style.padding.0 as usize));

        // Text color
        let text_style = if self.state.value.is_empty() && self.style.show_placeholder {
          // Placeholder styling (muted)
          get_semantic_color(
            theme.unwrap_or(&crate::themes::colors::dark_theme()),
            "text_muted",
          )
          .unwrap_or_default()
        } else {
          text_color.clone()
        };

        output.push_str(&text_style);

        // Add text content
        let mut text_with_cursor = visible_text.to_string();

        // Add cursor if focused
        if self.state.focused && !self.state.disabled {
          let cursor_pos = self
            .state
            .cursor_position
            .saturating_sub(self.state.scroll_offset);
          if cursor_pos <= text_with_cursor.len() {
            text_with_cursor.insert(cursor_pos, self.style.cursor_char);
          }
        }

        // Pad text to fill content width
        let text_len = text_with_cursor.chars().count();
        if text_len < content_width as usize {
          text_with_cursor.push_str(&" ".repeat(content_width as usize - text_len));
        }

        output.push_str(&text_with_cursor[..(content_width as usize).min(text_with_cursor.len())]);
        output.push_str("\x1B[0m");
        output.push_str(&" ".repeat(self.style.padding.0 as usize));
      }

      output.push_str("\x1B[0m");

      if self.style.show_border {
        output.push_str(current_border_color);
        output.push(border_set.vertical);
        output.push_str("\x1B[0m");
      }

      if row
        < layout
          .height
          .saturating_sub(if self.style.show_border { 2 } else { 0 })
          - 1
      {
        output.push('\n');
      }
    }

    // Render bottom border
    if self.style.show_border {
      output.push('\n');
      output.push_str(current_border_color);
      output.push(border_set.bottom_left);
      output.push_str(
        &border_set
          .horizontal
          .to_string()
          .repeat(layout.width as usize - 2),
      );
      output.push(border_set.bottom_right);
      output.push_str("\x1B[0m");
    }

    // Add validation message if present
    if let Some(message) = &self.state.validation_message {
      output.push('\n');
      output.push_str(&error_color);
      output.push_str("⚠ ");
      output.push_str(message);
      output.push_str("\x1B[0m");
    }

    output
  }

  /// Handle keyboard input events
  pub fn handle_key_event(&mut self, key: &str) -> bool {
    if self.state.disabled {
      return false;
    }

    match key {
      "Backspace" => {
        self.delete_char();
        true
      }
      "Delete" => {
        if self.state.cursor_position < self.state.value.len() {
          self.state.value.remove(self.state.cursor_position);
          self.validate();
        }
        true
      }
      "ArrowLeft" => {
        self.move_cursor_left();
        true
      }
      "ArrowRight" => {
        self.move_cursor_right();
        true
      }
      "Home" => {
        self.move_cursor_home();
        true
      }
      "End" => {
        self.move_cursor_end();
        true
      }
      "Enter" => {
        // For single-line inputs, Enter might submit the form
        // For multi-line, it adds a newline
        if self.input_type == InputType::TextArea {
          self.insert_char('\n');
        }
        true
      }
      _ => {
        // Handle regular character input
        if key.len() == 1 {
          if let Some(ch) = key.chars().next() {
            if ch.is_ascii_graphic() || ch == ' ' {
              self.insert_char(ch);
              return true;
            }
          }
        }
        false
      }
    }
  }
}

impl Default for InputState {
  fn default() -> Self {
    Self {
      value: String::new(),
      cursor_position: 0,
      focused: false,
      validation_state: ValidationState::None,
      validation_message: None,
      disabled: false,
      readonly: false,
      selection_start: None,
      selection_end: None,
      scroll_offset: 0,
    }
  }
}

impl Default for InputStyle {
  fn default() -> Self {
    Self {
      border_style: BorderStyle::Rounded,
      show_border: true,
      padding: (1, 0),
      min_width: 10,
      max_width: 0, // unlimited
      height: 1,
      show_placeholder: true,
      cursor_char: '│',
      cursor_blink: true,
    }
  }
}

// Fluent builder implementation
impl InputBuilder {
  pub fn input_type(mut self, input_type: InputType) -> Self {
    self.input.input_type = input_type;
    self
  }

  pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
    self.input.placeholder = placeholder.into();
    self
  }

  pub fn value(mut self, value: impl Into<String>) -> Self {
    self.input.set_value(value);
    self
  }

  pub fn css_class(mut self, class: impl Into<String>) -> Self {
    self.input.css_classes.push(class.into());
    self
  }

  pub fn css_classes(mut self, classes: Vec<String>) -> Self {
    self.input.css_classes = classes;
    self
  }

  pub fn inline_style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self {
    self
      .input
      .inline_styles
      .insert(property.into(), value.into());
    self
  }

  pub fn theme(mut self, theme: impl Into<String>) -> Self {
    self.input.theme = Some(theme.into());
    self
  }

  pub fn max_length(mut self, max_length: usize) -> Self {
    self.input.max_length = Some(max_length);
    self
  }

  pub fn min_length(mut self, min_length: usize) -> Self {
    self.input.min_length = Some(min_length);
    self
  }

  pub fn required(mut self, required: bool) -> Self {
    self.input.required = required;
    self
  }

  pub fn disabled(mut self, disabled: bool) -> Self {
    self.input.state.disabled = disabled;
    self
  }

  pub fn readonly(mut self, readonly: bool) -> Self {
    self.input.state.readonly = readonly;
    self
  }

  pub fn border_style(mut self, style: BorderStyle) -> Self {
    self.input.style.border_style = style;
    self
  }

  pub fn show_border(mut self, show: bool) -> Self {
    self.input.style.show_border = show;
    self
  }

  pub fn padding(mut self, horizontal: u16, vertical: u16) -> Self {
    self.input.style.padding = (horizontal, vertical);
    self
  }

  pub fn width(mut self, min: u16, max: u16) -> Self {
    self.input.style.min_width = min;
    self.input.style.max_width = max;
    self
  }

  pub fn height(mut self, height: u16) -> Self {
    self.input.style.height = height;
    self
  }

  pub fn autocomplete(mut self, suggestions: Vec<String>) -> Self {
    self.input.autocomplete = suggestions;
    self
  }

  pub fn build(self) -> Input {
    self.input
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::themes::colors::dark_theme;

  #[test]
  fn test_input_creation() {
    let input = Input::new("test-input");
    assert_eq!(input.id, "test-input");
    assert_eq!(input.input_type, InputType::Text);
    assert!(input.state.value.is_empty());
    assert!(!input.state.focused);
  }

  #[test]
  fn test_input_builder() {
    let input = Input::builder("builder-test")
      .input_type(InputType::Email)
      .placeholder("Enter email")
      .value("test@example.com")
      .max_length(50)
      .required(true)
      .css_class("form-input")
      .build();

    assert_eq!(input.id, "builder-test");
    assert_eq!(input.input_type, InputType::Email);
    assert_eq!(input.placeholder, "Enter email");
    assert_eq!(input.value(), "test@example.com");
    assert_eq!(input.max_length, Some(50));
    assert!(input.required);
    assert!(input.css_classes.contains(&"form-input".to_string()));
  }

  #[test]
  fn test_input_value_manipulation() {
    let mut input = Input::new("test");

    input.set_value("Hello");
    assert_eq!(input.value(), "Hello");
    assert_eq!(input.state.cursor_position, 5);

    input.insert_char('!');
    assert_eq!(input.value(), "Hello!");
    assert_eq!(input.state.cursor_position, 6);

    input.delete_char();
    assert_eq!(input.value(), "Hello");
    assert_eq!(input.state.cursor_position, 5);

    input.clear();
    assert_eq!(input.value(), "");
    assert_eq!(input.state.cursor_position, 0);
  }

  #[test]
  fn test_cursor_movement() {
    let mut input = Input::new("test");
    input.set_value("Hello World");

    input.move_cursor_home();
    assert_eq!(input.state.cursor_position, 0);

    input.move_cursor_end();
    assert_eq!(input.state.cursor_position, 11);

    input.move_cursor_left();
    assert_eq!(input.state.cursor_position, 10);

    input.move_cursor_right();
    assert_eq!(input.state.cursor_position, 11);
  }

  #[test]
  fn test_validation() {
    let mut input = Input::builder("email-test")
      .input_type(InputType::Email)
      .required(true)
      .build();

    // Empty required field should be invalid
    input.validate();
    assert_eq!(input.state.validation_state, ValidationState::Invalid);

    // Invalid email should be invalid
    input.set_value("invalid-email");
    assert_eq!(input.state.validation_state, ValidationState::Invalid);

    // Valid email should be valid
    input.set_value("test@example.com");
    assert_eq!(input.state.validation_state, ValidationState::Valid);
  }

  #[test]
  fn test_key_handling() {
    let mut input = Input::new("key-test");

    assert!(input.handle_key_event("a"));
    assert_eq!(input.value(), "a");

    assert!(input.handle_key_event("b"));
    assert_eq!(input.value(), "ab");

    assert!(input.handle_key_event("Backspace"));
    assert_eq!(input.value(), "a");

    assert!(input.handle_key_event("ArrowLeft"));
    assert_eq!(input.state.cursor_position, 0);

    assert!(input.handle_key_event("c"));
    assert_eq!(input.value(), "ca");
  }

  #[test]
  fn test_disabled_input() {
    let mut input = Input::builder("disabled-test").disabled(true).build();

    assert!(!input.handle_key_event("a"));
    assert_eq!(input.value(), "");

    input.insert_char('x');
    assert_eq!(input.value(), "");
  }

  #[test]
  fn test_rendering() {
    let input = Input::builder("render-test")
      .value("Hello")
      .placeholder("Enter text")
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let theme = dark_theme();
    let rendered = input.render(&layout, Some(&theme));

    // Should contain the input value
    assert!(rendered.contains("Hello"));
    // Should contain border characters
    assert!(rendered.contains("┌") || rendered.contains("╭"));
  }
}

// WidgetFactory convenience functions

/// Create an Input widget using the factory pattern
///
/// This function provides a simple way to create inputs using the WidgetFactory pattern,
/// reducing boilerplate code for common input creation scenarios.
///
/// # Arguments
///
/// * `config` - InputConfig containing all the widget configuration
///
/// # Returns
///
/// A fully configured Input widget
///
/// # Examples
///
/// ```rust,no_run
/// use reactive_tui::widgets::{Input, InputConfig, InputType, create_input};
///
/// let config = InputConfig::new("email-input")
///     .input_type(InputType::Email)
///     .placeholder("Enter your email")
///     .required(true)
///     .class("form-control");
///
/// let input = create_input(config);
/// ```
pub fn create_input(config: InputConfig) -> Input {
  config.build()
}

/// Create an Input widget with fluent configuration
///
/// This function provides a concise way to create and configure inputs using
/// a closure that operates on the InputConfig builder.
///
/// # Arguments
///
/// * `id` - Unique identifier for the input
/// * `f` - Closure that configures the InputConfig
///
/// # Returns
///
/// A fully configured Input widget
///
/// # Examples
///
/// ```rust,no_run
/// use reactive_tui::widgets::{Input, InputType, input};
///
/// let text_input = input("username", |c| {
///     c.placeholder("Enter username")
///      .required(true)
///      .max_length(50)
///      .class("form-input")
/// });
///
/// let password_input = input("password", |c| {
///     c.input_type(InputType::Password)
///      .placeholder("Enter password")
///      .required(true)
///      .class("form-input")
/// });
/// ```
pub fn input<F>(id: &str, f: F) -> Input
where
  F: FnOnce(InputConfig) -> InputConfig,
{
  let config = InputConfig::new(id);
  let configured = f(config);
  create_input(configured)
}
