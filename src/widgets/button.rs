//! # Button Widget
//!
//! Interactive button component with comprehensive styling and state management.
//!
//! The button widget provides a flexible, CSS-styled interactive element supporting
//! multiple visual types, states, and accessibility features. Buttons integrate seamlessly
//! with the theming system and support focus navigation, keyboard activation, and
//! various visual styles.
//!
//! ## Features
//!
//! - **Multiple Types**: Primary, secondary, success, warning, danger, info, ghost, link
//! - **State Management**: Normal, hover, active, focused, disabled states
//! - **CSS Integration**: Full CSS styling support with utility classes
//! - **Accessibility**: Keyboard navigation and screen reader support
//! - **Customization**: Icons, sizes, borders, and custom styling
//! - **Theme Support**: Automatic color palette integration
//!
//! ## Examples
//!
//! ### Basic Button
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let button = Button::builder("btn1", "Click Me")
//!     .button_type(ButtonType::Primary)
//!     .size(ButtonSize::Medium)
//!     .build();
//! ```
//!
//! ### Styled Button with Icon
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let submit_button = Button::builder("submit_btn", "Submit")
//!     .button_type(ButtonType::Success)
//!     .size(ButtonSize::Large)
//!     .icon('✓', IconPosition::Left)
//!     .build();
//! ```
//!
//! ### Disabled Button
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let disabled_button = Button::builder("disabled_btn", "Loading...")
//!     .button_type(ButtonType::Secondary)
//!     .state(ButtonState::Disabled)
//!     .build();
//! ```

use crate::layout::LayoutRect;
use crate::themes::{
  get_border_set, get_semantic_background, get_semantic_color, BorderStyle, ColorTheme,
  UtilityProcessor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Button types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonType {
  /// Primary action button
  Primary,
  /// Secondary action button
  Secondary,
  /// Success/confirm button
  Success,
  /// Warning/caution button
  Warning,
  /// Danger/destructive button
  Danger,
  /// Info/neutral button
  Info,
  /// Ghost/outline button
  Ghost,
  /// Link-style button
  Link,
}

/// Button states
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonState {
  /// Normal interactive state
  Normal,
  /// Mouse hovering over button
  Hover,
  /// Button is being pressed/clicked
  Active,
  /// Button has focus (keyboard navigation)
  Focused,
  /// Button is disabled
  Disabled,
  /// Button is in loading state
  Loading,
}

/// Button size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ButtonSize {
  /// Extra small button
  ExtraSmall,
  /// Small button
  Small,
  /// Medium button (default)
  #[default]
  Medium,
  /// Large button
  Large,
  /// Extra large button
  ExtraLarge,
}

/// Button border styles including pseudo-rounded option
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ButtonBorderStyle {
  /// No border
  None,
  /// Standard square border
  Square(BorderStyle),
  /// Rounded corners using standard chars
  Rounded(BorderStyle),
  /// Pseudo-rounded using subscript parentheses
  PseudoRounded,
  /// Open bracket corners (minimal border style)
  BracketCorners,
  /// Curly bracket hooks (decorative border style)
  CurlyHooks,
  /// Medium ornamental parentheses (refined style)
  OrnamentalParens,
  /// Medium angle bracket ornaments (prompt/action style)
  AngleBrackets,
  /// Heavy angle quotation marks (strong prompt style)
  AngleQuotes,
  /// Heavy angle bracket ornaments (bold action style)
  HeavyAngles,
}

/// Button styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ButtonStyle {
  /// Border style for the button
  pub border_style: ButtonBorderStyle,
  /// Padding inside the button (horizontal, vertical)
  pub padding: (u16, u16),
  /// Minimum width of the button
  pub min_width: u16,
  /// Height of the button
  pub height: u16,
  /// Button size variant
  pub size: ButtonSize,
  /// Whether to center text
  pub center_text: bool,
  /// Custom icon character
  pub icon: Option<char>,
  /// Icon position (left or right of text)
  pub icon_position: IconPosition,
}

/// Icon position within button
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum IconPosition {
  Left,
  Right,
}

/// Button widget with comprehensive styling and state management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Button {
  /// Unique identifier for the button
  pub id: String,
  /// Button text/label
  pub text: String,
  /// Button type
  pub button_type: ButtonType,
  /// Current state
  pub state: ButtonState,
  /// Styling configuration
  pub style: ButtonStyle,
  /// CSS class names
  pub css_classes: Vec<String>,
  /// Inline CSS styles
  pub inline_styles: HashMap<String, String>,
  /// Theme reference
  pub theme: Option<String>,
  /// Whether button is focusable
  pub focusable: bool,
  /// Tab index for keyboard navigation
  pub tab_index: Option<i32>,
  /// Tooltip text
  pub tooltip: Option<String>,
  /// Loading text (shown when state is Loading)
  pub loading_text: Option<String>,
}

/// Button builder for fluent API
pub struct ButtonBuilder {
  button: Button,
}

impl Button {
  /// Create a new button with default settings
  pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      text: text.into(),
      button_type: ButtonType::Primary,
      state: ButtonState::Normal,
      style: ButtonStyle::default(),
      css_classes: Vec::new(),
      inline_styles: HashMap::new(),
      theme: None,
      focusable: true,
      tab_index: None,
      tooltip: None,
      loading_text: None,
    }
  }

  /// Create a builder for fluent API
  pub fn builder(id: impl Into<String>, text: impl Into<String>) -> ButtonBuilder {
    ButtonBuilder {
      button: Self::new(id, text),
    }
  }

  /// Set the button state
  pub fn set_state(&mut self, state: ButtonState) {
    self.state = state;
  }

  /// Check if button is interactive (not disabled or loading)
  pub fn is_interactive(&self) -> bool {
    !matches!(self.state, ButtonState::Disabled | ButtonState::Loading)
  }

  /// Check if button is disabled
  pub fn is_disabled(&self) -> bool {
    matches!(self.state, ButtonState::Disabled)
  }

  /// Set button disabled state
  pub fn set_disabled(&mut self, disabled: bool) {
    if disabled {
      self.state = ButtonState::Disabled;
      self.focusable = false; // Disabled buttons should not be focusable
    } else {
      self.state = ButtonState::Normal;
      self.focusable = true; // Re-enable focus when not disabled
    }
  }

  /// Get effective button text (including loading state)
  pub fn effective_text(&self) -> &str {
    if self.state == ButtonState::Loading {
      self.loading_text.as_deref().unwrap_or("Loading...")
    } else {
      &self.text
    }
  }

  /// Render the button with utility CSS classes and theme support
  pub fn render_with_utilities(
    &self,
    layout: &LayoutRect,
    utility_processor: &UtilityProcessor,
  ) -> String {
    let mut output = String::new();

    // Process utility classes
    let class_styles = utility_processor.process_classes(&self.css_classes);
    output.push_str(&class_styles);

    // Get spacing from utility classes
    let padding_h = self
      .css_classes
      .iter()
      .find_map(|class| {
        if class.starts_with("px-") {
          utility_processor.get_spacing(class)
        } else {
          None
        }
      })
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

    // Calculate content dimensions
    let button_text = self.effective_text();
    let icon_width = if self.style.icon.is_some() { 2 } else { 0 }; // icon + space
    let text_width = button_text.chars().count() + icon_width;
    let content_width = layout.width.saturating_sub(padding_h * 2);
    let actual_width = text_width.max(content_width as usize);

    // Render based on border style
    let rendered_content = match self.style.border_style {
      ButtonBorderStyle::None => {
        self.render_borderless(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::Square(border_style) | ButtonBorderStyle::Rounded(border_style) => {
        let border_set = get_border_set(border_style);
        self.render_bordered(
          layout,
          button_text,
          padding_h,
          padding_v,
          actual_width,
          &border_set,
        )
      }
      ButtonBorderStyle::PseudoRounded => {
        self.render_pseudo_rounded(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::BracketCorners => {
        self.render_bracket_corners(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::CurlyHooks => {
        self.render_curly_hooks(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::OrnamentalParens => {
        self.render_ornamental_parens(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::AngleBrackets => {
        self.render_angle_brackets(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::AngleQuotes => {
        self.render_angle_quotes(layout, button_text, padding_h, padding_v, actual_width)
      }
      ButtonBorderStyle::HeavyAngles => {
        self.render_heavy_angles(layout, button_text, padding_h, padding_v, actual_width)
      }
    };

    output.push_str(&rendered_content);

    // Add state-based styling
    if let Some(state_style) = self.get_state_styling(utility_processor) {
      output.push_str(&state_style);
    }

    // Reset styles
    output.push_str("\x1B[0m");

    output
  }

  /// Render borderless button
  fn render_borderless(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    for row in 0..layout.height {
      if row < padding_v || row >= layout.height.saturating_sub(padding_v) {
        // Padding rows
        output.push_str(&" ".repeat(width + padding_h as usize * 2));
      } else {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));

        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      }

      if row < layout.height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with standard borders
  fn render_bordered(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    padding_v: u16,
    width: usize,
    border_set: &crate::themes::BorderSet,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;

    // Top border
    output.push(border_set.top_left);
    output.push_str(&border_set.horizontal.to_string().repeat(total_width));
    output.push(border_set.top_right);
    output.push('\n');

    // Content area
    let content_height = layout.height.saturating_sub(2);
    for row in 0..content_height {
      output.push(border_set.vertical);

      if row < padding_v || row >= content_height.saturating_sub(padding_v) {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      } else {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      }

      output.push(border_set.vertical);

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    // Bottom border
    output.push('\n');
    output.push(border_set.bottom_left);
    output.push_str(&border_set.horizontal.to_string().repeat(total_width));
    output.push(border_set.bottom_right);

    output
  }

  /// Render pseudo-rounded button using subscript parentheses
  fn render_pseudo_rounded(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    _padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;

    // Top border with pseudo-rounded corners
    output.push('₍'); // U+208D SUBSCRIPT LEFT PARENTHESIS
    output.push_str(&"─".repeat(total_width));
    output.push('₎'); // U+208E SUBSCRIPT RIGHT PARENTHESIS
    output.push('\n');

    // Content area
    let content_height = layout.height.saturating_sub(2);
    for row in 0..content_height {
      if row == 0 || row == content_height - 1 {
        // First and last content rows - use rounded sides
        output.push('₍');
      } else {
        // Middle content rows - use straight sides
        output.push('│');
      }

      // For pseudo-rounded, always show content in the middle row(s)
      if content_height == 1 || (content_height > 1 && row == content_height / 2) {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      } else {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      }

      if row == 0 || row == content_height - 1 {
        output.push('₎');
      } else {
        output.push('│');
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    // Bottom border with pseudo-rounded corners
    output.push('\n');
    output.push('₍');
    output.push_str(&"─".repeat(total_width));
    output.push('₎');

    output
  }

  /// Render button with bracket corners (minimal open style)
  fn render_bracket_corners(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 {
        output.push('⌜'); // U+231C TOP LEFT CORNER
      } else if row == content_height - 1 {
        output.push('⌞'); // U+231E BOTTOM LEFT CORNER
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if row < padding_v || row >= content_height.saturating_sub(padding_v) {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      } else {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      }

      // Last column
      if row == 0 {
        output.push('⌝'); // U+231D TOP RIGHT CORNER
      } else if row == content_height - 1 {
        output.push('⌟'); // U+231F BOTTOM RIGHT CORNER
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with curly bracket hooks (decorative style)
  fn render_curly_hooks(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 {
        output.push('⎧'); // U+23A7 LEFT CURLY BRACKET UPPER HOOK
      } else if row == content_height - 1 {
        output.push('⎩'); // U+23A9 LEFT CURLY BRACKET LOWER HOOK
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if row < padding_v || row >= content_height.saturating_sub(padding_v) {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      } else {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      }

      // Last column
      if row == 0 {
        output.push('⎫'); // U+23AB RIGHT CURLY BRACKET UPPER HOOK
      } else if row == content_height - 1 {
        output.push('⎭'); // U+23AD RIGHT CURLY BRACKET LOWER HOOK
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with ornamental parentheses (refined style)
  fn render_ornamental_parens(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    _padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 || row == content_height - 1 {
        output.push('❨'); // U+2768 MEDIUM LEFT PARENTHESIS ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if content_height == 1 || (content_height > 1 && row == content_height / 2) {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      } else {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      }

      // Last column
      if row == 0 || row == content_height - 1 {
        output.push('❩'); // U+2769 MEDIUM RIGHT PARENTHESIS ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with angle bracket ornaments (prompt/action style)
  fn render_angle_brackets(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    _padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 || row == content_height - 1 {
        output.push('❬'); // U+276C MEDIUM LEFT-POINTING ANGLE BRACKET ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if content_height == 1 || (content_height > 1 && row == content_height / 2) {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      } else {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      }

      // Last column
      if row == 0 || row == content_height - 1 {
        output.push('❭'); // U+276D MEDIUM RIGHT-POINTING ANGLE BRACKET ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with angle quotation marks (strong prompt style)
  fn render_angle_quotes(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    _padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 || row == content_height - 1 {
        output.push('❮'); // U+276E HEAVY LEFT-POINTING ANGLE QUOTATION MARK ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if content_height == 1 || (content_height > 1 && row == content_height / 2) {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      } else {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      }

      // Last column
      if row == 0 || row == content_height - 1 {
        output.push('❯'); // U+276F HEAVY RIGHT-POINTING ANGLE QUOTATION MARK ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Render button with heavy angle bracket ornaments (bold action style)
  fn render_heavy_angles(
    &self,
    layout: &LayoutRect,
    text: &str,
    padding_h: u16,
    _padding_v: u16,
    width: usize,
  ) -> String {
    let mut output = String::new();

    let total_width = width + padding_h as usize * 2;
    let content_height = layout.height;

    for row in 0..content_height {
      // First column
      if row == 0 || row == content_height - 1 {
        output.push('❰'); // U+2770 HEAVY LEFT-POINTING ANGLE BRACKET ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      // Content area
      if content_height == 1 || (content_height > 1 && row == content_height / 2) {
        // Content row
        output.push_str(&" ".repeat(padding_h as usize));
        let content = self.format_button_content(text, width);
        output.push_str(&content);
        output.push_str(&" ".repeat(padding_h as usize));
      } else {
        // Padding rows
        output.push_str(&" ".repeat(total_width));
      }

      // Last column
      if row == 0 || row == content_height - 1 {
        output.push('❱'); // U+2771 HEAVY RIGHT-POINTING ANGLE BRACKET ORNAMENT
      } else {
        output.push(' '); // Empty space for middle rows
      }

      if row < content_height - 1 {
        output.push('\n');
      }
    }

    output
  }

  /// Format button content with icon and centered text
  fn format_button_content(&self, text: &str, width: usize) -> String {
    let mut content = String::new();

    // Add icon if present
    let full_text = if let Some(icon) = self.style.icon {
      match self.style.icon_position {
        IconPosition::Left => format!("{icon} {text}"),
        IconPosition::Right => format!("{text} {icon}"),
      }
    } else {
      text.to_string()
    };

    // Center or left-align text
    if self.style.center_text {
      let padding = width.saturating_sub(full_text.chars().count()) / 2;
      content.push_str(&" ".repeat(padding));
      content.push_str(&full_text);
      let remaining = width.saturating_sub(content.chars().count());
      content.push_str(&" ".repeat(remaining));
    } else {
      content.push_str(&full_text);
      let remaining = width.saturating_sub(full_text.chars().count());
      content.push_str(&" ".repeat(remaining));
    }

    // Truncate if too long
    if content.chars().count() > width {
      content.truncate(width.saturating_sub(3));
      content.push_str("...");
    }

    content
  }

  /// Get state-specific styling
  fn get_state_styling(&self, utility_processor: &UtilityProcessor) -> Option<String> {
    match self.state {
      ButtonState::Hover => {
        // Apply hover classes if present
        let hover_classes: Vec<String> = self
          .css_classes
          .iter()
          .filter(|class| class.starts_with("hover:"))
          .cloned()
          .collect();
        if !hover_classes.is_empty() {
          Some(utility_processor.process_classes(&hover_classes))
        } else {
          None
        }
      }
      ButtonState::Active => {
        Some("\x1B[7m".to_string()) // Invert colors
      }
      ButtonState::Focused => {
        Some("\x1B[4m".to_string()) // Underline
      }
      ButtonState::Disabled => {
        Some("\x1B[2m".to_string()) // Dim
      }
      ButtonState::Loading => {
        Some("\x1B[5m".to_string()) // Blink (if supported)
      }
      _ => None,
    }
  }

  /// Render the button with theme support (legacy method)
  pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();

    // Get colors from theme based on button type
    let (bg_color, text_color, _border_color) = if let Some(theme) = theme {
      let (bg_key, text_key, border_key) = match self.button_type {
        ButtonType::Primary => ("button_background", "button_text", "button_border"),
        ButtonType::Success => ("success", "text_inverse", "success"),
        ButtonType::Warning => ("warning", "text_inverse", "warning"),
        ButtonType::Danger => ("error", "text_inverse", "error"),
        ButtonType::Info => ("info", "text_inverse", "info"),
        _ => ("surface", "text", "border"),
      };

      (
        get_semantic_background(theme, bg_key).unwrap_or_default(),
        get_semantic_color(theme, text_key).unwrap_or_default(),
        get_semantic_color(theme, border_key).unwrap_or_default(),
      )
    } else {
      (String::new(), String::new(), String::new())
    };

    // Apply colors
    output.push_str(&bg_color);
    output.push_str(&text_color);

    // Render based on border style
    let button_text = self.effective_text();
    match self.style.border_style {
      ButtonBorderStyle::PseudoRounded => {
        output.push_str(&self.render_pseudo_rounded(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::BracketCorners => {
        output.push_str(&self.render_bracket_corners(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::CurlyHooks => {
        output.push_str(&self.render_curly_hooks(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::OrnamentalParens => {
        output.push_str(&self.render_ornamental_parens(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::AngleBrackets => {
        output.push_str(&self.render_angle_brackets(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::AngleQuotes => {
        output.push_str(&self.render_angle_quotes(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      ButtonBorderStyle::HeavyAngles => {
        output.push_str(&self.render_heavy_angles(
          layout,
          button_text,
          self.style.padding.0,
          self.style.padding.1,
          button_text.len(),
        ));
      }
      _ => {
        // Use standard rendering for other border types
        output.push_str(&format!(" {button_text} "));
      }
    }

    // Add state styling
    if let Some(state_style) = self.get_legacy_state_styling() {
      output.push_str(&state_style);
    }

    // Reset
    output.push_str("\x1B[0m");

    output
  }

  /// Get legacy state styling
  fn get_legacy_state_styling(&self) -> Option<String> {
    match self.state {
      ButtonState::Active => Some("\x1B[7m".to_string()),
      ButtonState::Focused => Some("\x1B[4m".to_string()),
      ButtonState::Disabled => Some("\x1B[2m".to_string()),
      ButtonState::Loading => Some("\x1B[5m".to_string()),
      _ => None,
    }
  }

  /// Handle click events
  pub fn handle_click(&mut self) -> bool {
    if self.is_interactive() {
      self.state = ButtonState::Active;
      true
    } else {
      false
    }
  }

  /// Handle hover events
  pub fn handle_hover(&mut self, hover: bool) {
    if self.is_interactive() {
      self.state = if hover {
        ButtonState::Hover
      } else {
        ButtonState::Normal
      };
    }
  }

  /// Handle focus events
  pub fn handle_focus(&mut self, focused: bool) {
    if self.is_interactive() && self.focusable {
      self.state = if focused {
        ButtonState::Focused
      } else {
        ButtonState::Normal
      };
    }
  }
}

impl Default for ButtonStyle {
  fn default() -> Self {
    Self {
      border_style: ButtonBorderStyle::Rounded(BorderStyle::Rounded),
      padding: (2, 1),
      min_width: 10,
      height: 3,
      size: ButtonSize::Medium,
      center_text: true,
      icon: None,
      icon_position: IconPosition::Left,
    }
  }
}

// Fluent builder implementation
impl ButtonBuilder {
  pub fn button_type(mut self, button_type: ButtonType) -> Self {
    self.button.button_type = button_type;
    self
  }

  pub fn state(mut self, state: ButtonState) -> Self {
    self.button.state = state;
    self
  }

  pub fn size(mut self, size: ButtonSize) -> Self {
    self.button.style.size = size;
    self
  }

  pub fn border_style(mut self, style: ButtonBorderStyle) -> Self {
    self.button.style.border_style = style;
    self
  }

  pub fn pseudo_rounded(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::PseudoRounded;
    self
  }

  pub fn bracket_corners(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::BracketCorners;
    self
  }

  pub fn curly_hooks(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::CurlyHooks;
    self
  }

  pub fn ornamental_parens(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::OrnamentalParens;
    self
  }

  pub fn angle_brackets(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::AngleBrackets;
    self
  }

  pub fn angle_quotes(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::AngleQuotes;
    self
  }

  pub fn heavy_angles(mut self) -> Self {
    self.button.style.border_style = ButtonBorderStyle::HeavyAngles;
    self
  }

  pub fn css_class(mut self, class: impl Into<String>) -> Self {
    self.button.css_classes.push(class.into());
    self
  }

  pub fn css_classes(mut self, classes: Vec<String>) -> Self {
    self.button.css_classes = classes;
    self
  }

  pub fn icon(mut self, icon: char, position: IconPosition) -> Self {
    self.button.style.icon = Some(icon);
    self.button.style.icon_position = position;
    self
  }

  pub fn padding(mut self, horizontal: u16, vertical: u16) -> Self {
    self.button.style.padding = (horizontal, vertical);
    self
  }

  pub fn width(mut self, min_width: u16) -> Self {
    self.button.style.min_width = min_width;
    self
  }

  pub fn height(mut self, height: u16) -> Self {
    self.button.style.height = height;
    self
  }

  pub fn center_text(mut self, center: bool) -> Self {
    self.button.style.center_text = center;
    self
  }

  pub fn focusable(mut self, focusable: bool) -> Self {
    self.button.focusable = focusable;
    self
  }

  pub fn disabled(mut self, disabled: bool) -> Self {
    if disabled {
      self.button.state = ButtonState::Disabled;
      self.button.focusable = false; // Disabled buttons should not be focusable
    } else {
      self.button.state = ButtonState::Normal;
      self.button.focusable = true; // Re-enable focus when not disabled
    }
    self
  }

  pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
    self.button.tooltip = Some(tooltip.into());
    self
  }

  pub fn loading_text(mut self, loading_text: impl Into<String>) -> Self {
    self.button.loading_text = Some(loading_text.into());
    self
  }

  pub fn theme(mut self, theme: impl Into<String>) -> Self {
    self.button.theme = Some(theme.into());
    self
  }

  pub fn build(self) -> Button {
    self.button
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::themes::colors::dark_theme;
  use crate::themes::utility_css::UtilityProcessor;

  #[test]
  fn test_button_creation() {
    let button = Button::new("test-button", "Click Me");
    assert_eq!(button.id, "test-button");
    assert_eq!(button.text, "Click Me");
    assert_eq!(button.button_type, ButtonType::Primary);
    assert_eq!(button.state, ButtonState::Normal);
    assert!(button.is_interactive());
  }

  #[test]
  fn test_button_builder() {
    let button = Button::builder("builder-test", "Submit")
      .button_type(ButtonType::Success)
      .pseudo_rounded()
      .icon('✓', IconPosition::Left)
      .css_classes(vec![
        "bg-green-500".to_string(),
        "text-white".to_string(),
        "hover:bg-green-600".to_string(),
      ])
      .tooltip("Submit the form")
      .build();

    assert_eq!(button.id, "builder-test");
    assert_eq!(button.text, "Submit");
    assert_eq!(button.button_type, ButtonType::Success);
    assert!(matches!(
      button.style.border_style,
      ButtonBorderStyle::PseudoRounded
    ));
    assert_eq!(button.style.icon, Some('✓'));
    assert!(button.css_classes.contains(&"bg-green-500".to_string()));
    assert_eq!(button.tooltip, Some("Submit the form".to_string()));
  }

  #[test]
  fn test_button_states() {
    let mut button = Button::new("state-test", "Test");

    assert!(button.is_interactive());

    button.set_state(ButtonState::Disabled);
    assert!(!button.is_interactive());

    button.set_state(ButtonState::Loading);
    assert!(!button.is_interactive());
    assert_eq!(button.effective_text(), "Loading...");

    button.loading_text = Some("Please wait...".to_string());
    assert_eq!(button.effective_text(), "Please wait...");
  }

  #[test]
  fn test_button_interactions() {
    let mut button = Button::new("interaction-test", "Click");

    assert!(button.handle_click());
    assert_eq!(button.state, ButtonState::Active);

    button.handle_hover(true);
    assert_eq!(button.state, ButtonState::Hover);

    button.handle_focus(true);
    assert_eq!(button.state, ButtonState::Focused);

    button.set_state(ButtonState::Disabled);
    assert!(!button.handle_click());
  }

  #[test]
  fn test_pseudo_rounded_rendering() {
    let button = Button::builder("rounded-test", "Rounded")
      .pseudo_rounded()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain subscript parentheses
    assert!(rendered.contains('₍'));
    assert!(rendered.contains('₎'));
    assert!(rendered.contains("Rounded"));
  }

  #[test]
  fn test_button_with_icon() {
    let button = Button::builder("icon-test", "Save")
      .icon('💾', IconPosition::Left)
      .build();

    let content = button.format_button_content(button.effective_text(), 20);
    assert!(content.contains("💾 Save"));
  }

  #[test]
  fn test_bracket_corners_rendering() {
    let button = Button::builder("bracket-test", "Open Style")
      .bracket_corners()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain bracket corner characters
    assert!(rendered.contains('⌜')); // TOP LEFT CORNER
    assert!(rendered.contains('⌝')); // TOP RIGHT CORNER
    assert!(rendered.contains('⌞')); // BOTTOM LEFT CORNER
    assert!(rendered.contains('⌟')); // BOTTOM RIGHT CORNER
    assert!(rendered.contains("Open Style"));
  }

  #[test]
  fn test_curly_hooks_rendering() {
    let button = Button::builder("curly-test", "Decorative")
      .curly_hooks()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain curly bracket hook characters
    assert!(rendered.contains('⎧')); // LEFT CURLY BRACKET UPPER HOOK
    assert!(rendered.contains('⎫')); // RIGHT CURLY BRACKET UPPER HOOK
    assert!(rendered.contains('⎩')); // LEFT CURLY BRACKET LOWER HOOK
    assert!(rendered.contains('⎭')); // RIGHT CURLY BRACKET LOWER HOOK
    assert!(rendered.contains("Decorative"));
  }

  #[test]
  fn test_ornamental_parens_rendering() {
    let button = Button::builder("ornamental-test", "Refined")
      .ornamental_parens()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain ornamental parenthesis characters
    assert!(rendered.contains('❨')); // MEDIUM LEFT PARENTHESIS ORNAMENT
    assert!(rendered.contains('❩')); // MEDIUM RIGHT PARENTHESIS ORNAMENT
    assert!(rendered.contains("Refined"));
  }

  #[test]
  fn test_angle_brackets_rendering() {
    let button = Button::builder("angle-test", "Prompt")
      .angle_brackets()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain angle bracket ornament characters
    assert!(rendered.contains('❬')); // MEDIUM LEFT-POINTING ANGLE BRACKET ORNAMENT
    assert!(rendered.contains('❭')); // MEDIUM RIGHT-POINTING ANGLE BRACKET ORNAMENT
    assert!(rendered.contains("Prompt"));
  }

  #[test]
  fn test_angle_quotes_rendering() {
    let button = Button::builder("quotes-test", "Action")
      .angle_quotes()
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain angle quotation mark characters
    assert!(rendered.contains('❮')); // HEAVY LEFT-POINTING ANGLE QUOTATION MARK ORNAMENT
    assert!(rendered.contains('❯')); // HEAVY RIGHT-POINTING ANGLE QUOTATION MARK ORNAMENT
    assert!(rendered.contains("Action"));
  }

  #[test]
  fn test_heavy_angles_rendering() {
    let button = Button::builder("heavy-test", "Bold").heavy_angles().build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 3,
    };

    let utility_processor = UtilityProcessor::new();
    let rendered = button.render_with_utilities(&layout, &utility_processor);

    // Should contain heavy angle bracket characters
    assert!(rendered.contains('❰')); // HEAVY LEFT-POINTING ANGLE BRACKET ORNAMENT
    assert!(rendered.contains('❱')); // HEAVY RIGHT-POINTING ANGLE BRACKET ORNAMENT
    assert!(rendered.contains("Bold"));
  }

  #[test]
  fn test_button_theming() {
    let button = Button::builder("theme-test", "Themed")
      .button_type(ButtonType::Success)
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 15,
      height: 3,
    };

    let theme = dark_theme();
    let rendered = button.render(&layout, Some(&theme));

    // Should contain the button text
    assert!(rendered.contains("Themed"));
  }
}

impl Button {
  /// Convert to Element for integration with the component system
  /// This creates a responsive element that will be sized by the LayoutEngine
  pub fn to_element(&self) -> crate::components::Element {
    use crate::components::Element;

    let mut element = Element::with_tag("button")
      .id(&self.id)
      .content(&self.text)
      .class("button")
      .focusable(self.is_interactive())
      .attr("role", "button")
      .attr(
        "type",
        match self.button_type {
          ButtonType::Primary => "primary",
          ButtonType::Secondary => "secondary",
          ButtonType::Danger => "danger",
          ButtonType::Success => "success",
          ButtonType::Warning => "warning",
          ButtonType::Info => "info",
          ButtonType::Ghost => "ghost",
          ButtonType::Link => "link",
        },
      );

    // Add state classes for CSS styling
    match self.state {
      ButtonState::Normal => {}
      ButtonState::Hover => element = element.class("button-hover"),
      ButtonState::Active => element = element.class("button-active"),
      ButtonState::Focused => element = element.class("button-focused"),
      ButtonState::Disabled => element = element.class("button-disabled"),
      ButtonState::Loading => element = element.class("button-loading"),
    }

    // Add button type class for CSS styling
    element = element.class(format!(
      "button-{}",
      match self.button_type {
        ButtonType::Primary => "primary",
        ButtonType::Secondary => "secondary",
        ButtonType::Danger => "danger",
        ButtonType::Success => "success",
        ButtonType::Warning => "warning",
        ButtonType::Info => "info",
        ButtonType::Ghost => "ghost",
        ButtonType::Link => "link",
      }
    ));

    // Add CSS classes from the button configuration
    for class in &self.css_classes {
      element = element.class(class);
    }

    // Add inline styles as attributes for CSS processing
    for (key, value) in &self.inline_styles {
      element = element.attr(format!("style-{key}"), value);
    }

    // Add tooltip if present
    if let Some(tooltip) = &self.tooltip {
      element = element.attr("title", tooltip);
    }

    // Set tab index for keyboard navigation
    if let Some(tab_index) = self.tab_index {
      element = element.attr("tabindex", tab_index.to_string());
    }

    element.build()
  }

  /// Render the button with computed layout from LayoutEngine
  /// This method should be called by the rendering system after layout computation
  pub fn render_with_layout(
    &self,
    layout: &crate::layout::LayoutRect,
    theme: Option<&crate::themes::ColorTheme>,
  ) -> String {
    // Use the existing render method but with the computed layout
    self.render(layout, theme)
  }
}

// Implement ResponsiveWidget trait for Button
impl crate::widgets::ResponsiveWidget for Button {
  fn to_element(&self) -> crate::components::Element {
    self.to_element()
  }

  fn render_with_layout(
    &self,
    layout: &crate::layout::LayoutRect,
    theme: Option<&crate::themes::ColorTheme>,
  ) -> String {
    self.render_with_layout(layout, theme)
  }

  fn min_size(&self) -> (u16, u16) {
    // Minimum size is text length + padding + borders
    let text_width = self.text.chars().count() as u16;
    let min_width = text_width + self.style.padding.0 * 2 + 2; // padding + borders
    let min_height = 1 + self.style.padding.1 * 2 + 2; // padding + borders
    (min_width, min_height)
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    // Buttons typically don't need to grow beyond their content + reasonable padding
    let text_width = self.text.chars().count() as u16;
    let max_width = text_width + 20; // Allow some extra space for styling
    (Some(max_width), Some(5)) // Max height of 5 for buttons
  }

  fn can_grow_horizontal(&self) -> bool {
    // Buttons can grow horizontally to fill available space if needed
    true
  }

  fn can_grow_vertical(&self) -> bool {
    // Buttons typically don't need to grow vertically much
    false
  }
}
