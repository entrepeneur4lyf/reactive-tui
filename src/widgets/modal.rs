/*!
 * Modal/Dialog Component - Overlay dialogs with backdrop and focus trapping
 *
 * A comprehensive modal system providing:
 * - Overlay positioning with backdrop support
 * - Focus trapping and keyboard navigation
 * - Multiple modal types (alert, confirm, prompt, custom)
 * - Customizable sizing and positioning
 * - Animation support and transitions
 * - CSS utility class integration and theming
 * - Accessibility features (ARIA roles, keyboard handling)
 */

use crate::{
  layout::LayoutRect,
  themes::{color_to_ansi, ColorDefinition, ColorTheme, UtilityProcessor},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// Modal positioning within the screen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModalPosition {
  /// Center of screen (default)
  #[default]
  Center,
  /// Top center
  TopCenter,
  /// Bottom center
  BottomCenter,
  /// Left center
  LeftCenter,
  /// Right center
  RightCenter,
  /// Top left corner
  TopLeft,
  /// Top right corner
  TopRight,
  /// Bottom left corner
  BottomLeft,
  /// Bottom right corner
  BottomRight,
  /// Custom position (x, y)
  Custom(u16, u16),
}

/// Modal size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ModalSize {
  /// Small modal
  Small,
  /// Medium modal (default)
  #[default]
  Medium,
  /// Large modal
  Large,
  /// Extra large modal
  ExtraLarge,
  /// Full screen modal
  FullScreen,
  /// Custom size (width, height)
  Custom(u16, u16),
}

/// Modal types for different use cases
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ModalType {
  /// Basic modal with custom content
  Basic,
  /// Alert dialog with message and OK button
  Alert { message: String },
  /// Confirmation dialog with message and Yes/No buttons
  Confirm {
    message: String,
    yes_label: String,
    no_label: String,
  },
  /// Prompt dialog with message and input field
  Prompt {
    message: String,
    placeholder: String,
    default_value: String,
  },
  /// Custom modal with defined buttons
  Custom { buttons: Vec<ModalButton> },
}

/// Modal button configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ModalButton {
  /// Button ID
  pub id: String,
  /// Button label
  pub label: String,
  /// Button variant (primary, secondary, danger, etc.)
  pub variant: String,
  /// Whether this button closes the modal
  pub closes_modal: bool,
  /// Whether this is the default/focused button
  pub is_default: bool,
  /// Button action when clicked
  pub action: Option<String>,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
}

/// Modal backdrop configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalBackdrop {
  /// Whether backdrop is visible
  pub visible: bool,
  /// Whether clicking backdrop closes modal
  pub click_to_close: bool,
  /// Backdrop color
  pub color: Option<ColorDefinition>,
  /// Backdrop opacity (0.0 to 1.0)
  pub opacity: f32,
  /// Backdrop character (for text-mode rendering)
  pub character: char,
}

/// Modal styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ModalStyle {
  /// Modal background color
  pub background: Option<ColorDefinition>,
  /// Modal text color
  pub text_color: Option<ColorDefinition>,
  /// Border color
  pub border_color: Option<ColorDefinition>,
  /// Border character
  pub border_char: char,
  /// Title color
  pub title_color: Option<ColorDefinition>,
  /// Button colors
  pub button_colors: HashMap<String, ColorDefinition>,
  /// Padding inside modal
  pub padding: u16,
  /// Whether to show shadow
  pub shadow: bool,
  /// Whether to round corners
  pub rounded: bool,
}

/// CSS-styled modal widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Modal {
  /// Unique identifier
  pub id: String,
  /// Modal title
  pub title: Option<String>,
  /// Modal content
  pub content: String,
  /// Modal type and configuration
  pub modal_type: ModalType,
  /// Position on screen
  pub position: ModalPosition,
  /// Modal size
  pub size: ModalSize,
  /// Backdrop configuration
  pub backdrop: ModalBackdrop,
  /// Styling configuration
  pub style: ModalStyle,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
  /// Whether modal is currently open
  pub is_open: bool,
  /// Whether modal is closeable via Escape key
  pub closeable: bool,
  /// Focus trap - elements that can receive focus
  pub focusable_elements: Vec<String>,
  /// Currently focused element index
  pub focused_element: usize,
  /// Modal result/return value
  pub result: Option<String>,
  /// Whether to animate modal transitions
  pub animate: bool,
}

/// Builder for Modal component
pub struct ModalBuilder {
  modal: Modal,
}

impl Default for ModalBackdrop {
  fn default() -> Self {
    Self {
      visible: true,
      click_to_close: true,
      color: Some(ColorDefinition { r: 0, g: 0, b: 0 }),
      opacity: 0.5,
      character: '█',
    }
  }
}

impl Default for ModalStyle {
  fn default() -> Self {
    Self {
      background: Some(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      }),
      text_color: Some(ColorDefinition { r: 0, g: 0, b: 0 }),
      border_color: Some(ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      }),
      border_char: '┌',
      title_color: Some(ColorDefinition {
        r: 59,
        g: 130,
        b: 246,
      }),
      button_colors: HashMap::new(),
      padding: 2,
      shadow: true,
      rounded: true,
    }
  }
}

impl ModalButton {
  /// Create a new modal button
  pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      variant: "secondary".to_string(),
      closes_modal: false,
      is_default: false,
      action: None,
      css_classes: Vec::new(),
    }
  }

  /// Set button variant
  pub fn variant(mut self, variant: impl Into<String>) -> Self {
    self.variant = variant.into();
    self
  }

  /// Make button close modal when clicked
  pub fn closes_modal(mut self) -> Self {
    self.closes_modal = true;
    self
  }

  /// Make this the default/focused button
  pub fn default(mut self) -> Self {
    self.is_default = true;
    self
  }

  /// Set button action
  pub fn action(mut self, action: impl Into<String>) -> Self {
    self.action = Some(action.into());
    self
  }

  /// Add CSS classes to the button
  pub fn classes(mut self, classes: Vec<String>) -> Self {
    self.css_classes.extend(classes);
    self
  }

  /// Add a single CSS class to the button
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }
}

impl Modal {
  /// Create a new modal
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      title: None,
      content: String::new(),
      modal_type: ModalType::Basic,
      position: ModalPosition::default(),
      size: ModalSize::default(),
      backdrop: ModalBackdrop::default(),
      style: ModalStyle::default(),
      css_classes: Vec::new(),
      is_open: false,
      closeable: true,
      focusable_elements: Vec::new(),
      focused_element: 0,
      result: None,
      animate: true,
    }
  }

  /// Create a builder for the modal
  pub fn builder(id: impl Into<String>) -> ModalBuilder {
    ModalBuilder {
      modal: Self::new(id),
    }
  }

  /// Open the modal
  pub fn open(&mut self) {
    self.is_open = true;
    self.result = None;
    self.focused_element = 0;
    self.update_focusable_elements();
  }

  /// Close the modal
  pub fn close(&mut self) {
    self.is_open = false;
  }

  /// Close modal with result
  pub fn close_with_result(&mut self, result: String) {
    self.result = Some(result);
    self.close();
  }

  /// Check if modal is open
  pub fn is_open(&self) -> bool {
    self.is_open
  }

  /// Get modal result
  pub fn get_result(&self) -> Option<&String> {
    self.result.as_ref()
  }

  /// Add CSS classes to the modal
  pub fn classes(mut self, classes: Vec<String>) -> Self {
    self.css_classes.extend(classes);
    self
  }

  /// Add a single CSS class to the modal
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> bool {
    if !self.is_open {
      return false;
    }

    match key {
      "Escape" if self.closeable => {
        self.close();
        true
      }
      "Tab" => {
        self.focus_next();
        true
      }
      "Shift+Tab" => {
        self.focus_previous();
        true
      }
      "Enter" => {
        self.activate_focused_element();
        true
      }
      _ => false,
    }
  }

  /// Move focus to next element
  pub fn focus_next(&mut self) {
    if !self.focusable_elements.is_empty() {
      self.focused_element = (self.focused_element + 1) % self.focusable_elements.len();
    }
  }

  /// Move focus to previous element
  pub fn focus_previous(&mut self) {
    if !self.focusable_elements.is_empty() {
      self.focused_element = if self.focused_element == 0 {
        self.focusable_elements.len() - 1
      } else {
        self.focused_element - 1
      };
    }
  }

  /// Activate currently focused element
  pub fn activate_focused_element(&mut self) {
    if let Some(element_id) = self.focusable_elements.get(self.focused_element) {
      match &self.modal_type {
        ModalType::Alert { .. } => {
          if element_id == "ok" {
            self.close_with_result("ok".to_string());
          }
        }
        ModalType::Confirm { .. } => {
          if element_id == "yes" {
            self.close_with_result("yes".to_string());
          } else if element_id == "no" {
            self.close_with_result("no".to_string());
          }
        }
        ModalType::Prompt { .. } => {
          if element_id == "ok" {
            // TODO: Get input value
            self.close_with_result("ok".to_string());
          } else if element_id == "cancel" {
            self.close_with_result("cancel".to_string());
          }
        }
        ModalType::Custom { buttons } => {
          if let Some(button) = buttons.iter().find(|b| b.id == *element_id) {
            if button.closes_modal {
              self.close_with_result(button.id.clone());
            }
            // TODO: Handle button action
          }
        }
        ModalType::Basic => {
          // Custom handling for basic modals
        }
      }
    }
  }

  /// Update focusable elements based on modal type
  fn update_focusable_elements(&mut self) {
    self.focusable_elements.clear();

    match &self.modal_type {
      ModalType::Alert { .. } => {
        self.focusable_elements.push("ok".to_string());
      }
      ModalType::Confirm { .. } => {
        self.focusable_elements.push("yes".to_string());
        self.focusable_elements.push("no".to_string());
      }
      ModalType::Prompt { .. } => {
        self.focusable_elements.push("input".to_string());
        self.focusable_elements.push("ok".to_string());
        self.focusable_elements.push("cancel".to_string());
      }
      ModalType::Custom { buttons } => {
        for button in buttons {
          self.focusable_elements.push(button.id.clone());
        }
      }
      ModalType::Basic => {
        // Custom focusable elements would be set externally
      }
    }

    // Set default focus to first element or default button
    if let ModalType::Custom { buttons } = &self.modal_type {
      if let Some(default_button) = buttons.iter().position(|b| b.is_default) {
        self.focused_element = default_button;
      }
    }
  }

  /// Calculate modal dimensions and position
  pub fn calculate_layout(&self, screen: &LayoutRect) -> LayoutRect {
    let (width, height) = match self.size {
      ModalSize::Small => (40, 15),
      ModalSize::Medium => (60, 20),
      ModalSize::Large => (80, 30),
      ModalSize::ExtraLarge => (100, 40),
      ModalSize::FullScreen => (screen.width, screen.height),
      ModalSize::Custom(w, h) => (w, h),
    };

    let (x, y) = match self.position {
      ModalPosition::Center => (
        (screen.width.saturating_sub(width)) / 2,
        (screen.height.saturating_sub(height)) / 2,
      ),
      ModalPosition::TopCenter => ((screen.width.saturating_sub(width)) / 2, screen.height / 4),
      ModalPosition::BottomCenter => (
        (screen.width.saturating_sub(width)) / 2,
        screen.height * 3 / 4 - height,
      ),
      ModalPosition::LeftCenter => (screen.width / 4, (screen.height.saturating_sub(height)) / 2),
      ModalPosition::RightCenter => (
        screen.width * 3 / 4 - width,
        (screen.height.saturating_sub(height)) / 2,
      ),
      ModalPosition::TopLeft => (screen.width / 10, screen.height / 10),
      ModalPosition::TopRight => (screen.width * 9 / 10 - width, screen.height / 10),
      ModalPosition::BottomLeft => (screen.width / 10, screen.height * 9 / 10 - height),
      ModalPosition::BottomRight => (
        screen.width * 9 / 10 - width,
        screen.height * 9 / 10 - height,
      ),
      ModalPosition::Custom(custom_x, custom_y) => (custom_x, custom_y),
    };

    LayoutRect {
      x: x.min(screen.width.saturating_sub(width)),
      y: y.min(screen.height.saturating_sub(height)),
      width: width.min(screen.width),
      height: height.min(screen.height),
    }
  }

  /// Render the modal
  pub fn render(&self, screen: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    if !self.is_open {
      return String::new();
    }

    let mut output = String::new();
    let modal_layout = self.calculate_layout(screen);

    // Render backdrop
    if self.backdrop.visible {
      writeln!(output, "{}", self.render_backdrop(screen, theme)).unwrap();
    }

    // Render modal content
    writeln!(
      output,
      "{}",
      self.render_modal_content(&modal_layout, theme)
    )
    .unwrap();

    output
  }

  /// Render backdrop
  fn render_backdrop(&self, screen: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let backdrop_color = self
      .backdrop
      .color
      .unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 });
    let color_ansi = color_to_ansi(backdrop_color, false);

    for _ in 0..screen.height {
      writeln!(
        output,
        "{}{}\x1b[0m",
        color_ansi,
        self
          .backdrop
          .character
          .to_string()
          .repeat(screen.width as usize)
      )
      .unwrap();
    }

    output
  }

  /// Render modal content
  fn render_modal_content(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();

    // Background and border colors
    let bg_color = self.style.background.unwrap_or_else(|| {
      theme.map(|t| t.palette.surface).unwrap_or(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      })
    });
    let text_color = self.style.text_color.unwrap_or_else(|| {
      theme
        .map(|t| t.palette.text)
        .unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 })
    });
    let border_color = self.style.border_color.unwrap_or_else(|| {
      theme.map(|t| t.palette.border).unwrap_or(ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      })
    });

    let bg_ansi = color_to_ansi(bg_color, false);
    let _text_ansi = color_to_ansi(text_color, true);
    let border_ansi = color_to_ansi(border_color, true);

    // Render modal box
    for row in 0..layout.height {
      let mut line = String::new();

      if row == 0 {
        // Top border
        line.push_str(&format!(
          "{}╭{}╮{}",
          border_ansi,
          "─".repeat((layout.width - 2) as usize),
          "\x1b[0m"
        ));
      } else if row == layout.height - 1 {
        // Bottom border
        line.push_str(&format!(
          "{}╰{}╯{}",
          border_ansi,
          "─".repeat((layout.width - 2) as usize),
          "\x1b[0m"
        ));
      } else {
        // Content area
        line.push_str(&format!(
          "{}│{}{}│{}",
          border_ansi,
          bg_ansi,
          self.render_content_line(row - 1, layout.width - 2),
          "\x1b[0m"
        ));
      }

      writeln!(output, "{line}").unwrap();
    }

    output
  }

  /// Render a single content line
  fn render_content_line(&self, line_index: u16, width: u16) -> String {
    let content_height = 3; // Title + content + buttons
    let padding = self.style.padding;
    let content_width = width.saturating_sub(padding * 2);

    if line_index < padding {
      // Top padding
      return " ".repeat(width as usize);
    }

    let content_line = line_index - padding;

    if content_line == 0 && self.title.is_some() {
      // Title line
      let title = self.title.as_ref().unwrap();
      let title_len = title.chars().count().min(content_width as usize);
      let title_text = title.chars().take(title_len).collect::<String>();
      let padding_right = content_width as usize - title_len;
      format!(
        "{}{}{}",
        " ".repeat(padding as usize),
        title_text,
        " ".repeat(padding_right + padding as usize)
      )
    } else if content_line == 1 || (content_line == 0 && self.title.is_none()) {
      // Content line
      let content_len = self.content.chars().count().min(content_width as usize);
      let content_text = self.content.chars().take(content_len).collect::<String>();
      let padding_right = content_width as usize - content_len;
      format!(
        "{}{}{}",
        " ".repeat(padding as usize),
        content_text,
        " ".repeat(padding_right + padding as usize)
      )
    } else if content_line == content_height - 1 {
      // Button line
      self.render_buttons_line(width)
    } else {
      // Empty line or additional content
      " ".repeat(width as usize)
    }
  }

  /// Render buttons line
  fn render_buttons_line(&self, width: u16) -> String {
    let _padding = self.style.padding;

    match &self.modal_type {
      ModalType::Alert { .. } => {
        let button_text = if self.focused_element == 0 {
          "[OK]"
        } else {
          " OK "
        };
        let button_len = button_text.len();
        let center_pos = (width as usize - button_len) / 2;
        format!(
          "{}{}{}",
          " ".repeat(center_pos),
          button_text,
          " ".repeat(width as usize - center_pos - button_len)
        )
      }
      ModalType::Confirm {
        yes_label,
        no_label,
        ..
      } => {
        let yes_text = if self.focused_element == 0 {
          format!("[{yes_label}]")
        } else {
          format!(" {yes_label} ")
        };
        let no_text = if self.focused_element == 1 {
          format!("[{no_label}]")
        } else {
          format!(" {no_label} ")
        };
        let buttons_text = format!("{yes_text}  {no_text}");
        let button_len = buttons_text.len();
        let center_pos = (width as usize - button_len) / 2;
        format!(
          "{}{}{}",
          " ".repeat(center_pos),
          buttons_text,
          " ".repeat(width as usize - center_pos - button_len)
        )
      }
      ModalType::Prompt { .. } => {
        let ok_text = if self.focused_element == 1 {
          "[OK]"
        } else {
          " OK "
        };
        let cancel_text = if self.focused_element == 2 {
          "[Cancel]"
        } else {
          " Cancel "
        };
        let buttons_text = format!("{ok_text}  {cancel_text}");
        let button_len = buttons_text.len();
        let center_pos = (width as usize - button_len) / 2;
        format!(
          "{}{}{}",
          " ".repeat(center_pos),
          buttons_text,
          " ".repeat(width as usize - center_pos - button_len)
        )
      }
      ModalType::Custom { buttons } => {
        let button_texts: Vec<String> = buttons
          .iter()
          .enumerate()
          .map(|(i, button)| {
            if i == self.focused_element {
              format!("{}]", button.label)
            } else {
              format!("{} ", button.label)
            }
          })
          .collect();
        let buttons_text = button_texts.join("  ");
        let button_len = buttons_text.len();
        let center_pos = (width as usize - button_len) / 2;
        format!(
          "{}{}{}",
          " ".repeat(center_pos),
          buttons_text,
          " ".repeat(width as usize - center_pos - button_len)
        )
      }
      ModalType::Basic => " ".repeat(width as usize),
    }
  }

  /// Render with utility CSS classes
  pub fn render_with_utilities(
    &self,
    screen: &LayoutRect,
    utility_processor: &UtilityProcessor,
  ) -> String {
    let _utility_styles = utility_processor.process_classes(&self.css_classes);
    self.render(screen, None)
  }
}

impl ModalBuilder {
  /// Set modal title
  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.modal.title = Some(title.into());
    self
  }

  /// Set modal content
  pub fn content(mut self, content: impl Into<String>) -> Self {
    self.modal.content = content.into();
    self
  }

  /// Set modal type
  pub fn modal_type(mut self, modal_type: ModalType) -> Self {
    self.modal.modal_type = modal_type;
    self
  }

  /// Set modal position
  pub fn position(mut self, position: ModalPosition) -> Self {
    self.modal.position = position;
    self
  }

  /// Set modal size
  pub fn size(mut self, size: ModalSize) -> Self {
    self.modal.size = size;
    self
  }

  /// Set backdrop configuration
  pub fn backdrop(mut self, backdrop: ModalBackdrop) -> Self {
    self.modal.backdrop = backdrop;
    self
  }

  /// Set modal style
  pub fn style(mut self, style: ModalStyle) -> Self {
    self.modal.style = style;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.modal.css_classes.push(class.into());
    self
  }

  /// Make modal not closeable
  pub fn not_closeable(mut self) -> Self {
    self.modal.closeable = false;
    self
  }

  /// Disable animations
  pub fn no_animation(mut self) -> Self {
    self.modal.animate = false;
    self
  }

  /// Build the modal widget
  pub fn build(self) -> Modal {
    self.modal
  }
}

/// Convenience functions for common modal types
/// Create an alert modal
pub fn alert_modal(
  id: impl Into<String>,
  title: impl Into<String>,
  message: impl Into<String>,
) -> Modal {
  Modal::builder(id)
    .title(title)
    .modal_type(ModalType::Alert {
      message: message.into(),
    })
    .size(ModalSize::Small)
    .build()
}

/// Create a confirmation modal
pub fn confirm_modal(
  id: impl Into<String>,
  title: impl Into<String>,
  message: impl Into<String>,
) -> Modal {
  Modal::builder(id)
    .title(title)
    .modal_type(ModalType::Confirm {
      message: message.into(),
      yes_label: "Yes".to_string(),
      no_label: "No".to_string(),
    })
    .size(ModalSize::Medium)
    .build()
}

/// Create a prompt modal
pub fn prompt_modal(
  id: impl Into<String>,
  title: impl Into<String>,
  message: impl Into<String>,
  placeholder: impl Into<String>,
) -> Modal {
  Modal::builder(id)
    .title(title)
    .modal_type(ModalType::Prompt {
      message: message.into(),
      placeholder: placeholder.into(),
      default_value: String::new(),
    })
    .size(ModalSize::Medium)
    .build()
}

/// Create a custom modal with buttons
pub fn custom_modal(
  id: impl Into<String>,
  title: impl Into<String>,
  content: impl Into<String>,
  buttons: Vec<ModalButton>,
) -> Modal {
  Modal::builder(id)
    .title(title)
    .content(content)
    .modal_type(ModalType::Custom { buttons })
    .size(ModalSize::Medium)
    .build()
}

/// Create a full screen modal
pub fn fullscreen_modal(id: impl Into<String>, content: impl Into<String>) -> Modal {
  Modal::builder(id)
    .content(content)
    .size(ModalSize::FullScreen)
    .backdrop(ModalBackdrop {
      visible: false,
      ..ModalBackdrop::default()
    })
    .build()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_modal_creation() {
    let modal = Modal::new("test-modal");
    assert_eq!(modal.id, "test-modal");
    assert_eq!(modal.position, ModalPosition::Center);
    assert_eq!(modal.size, ModalSize::Medium);
    assert!(!modal.is_open);
    assert!(modal.closeable);
  }

  #[test]
  fn test_modal_builder() {
    let modal = Modal::builder("builder-modal")
      .title("Test Modal")
      .content("This is test content")
      .position(ModalPosition::TopCenter)
      .size(ModalSize::Large)
      .not_closeable()
      .build();

    assert_eq!(modal.title, Some("Test Modal".to_string()));
    assert_eq!(modal.content, "This is test content");
    assert_eq!(modal.position, ModalPosition::TopCenter);
    assert_eq!(modal.size, ModalSize::Large);
    assert!(!modal.closeable);
  }

  #[test]
  fn test_modal_open_close() {
    let mut modal = Modal::new("test-modal");

    assert!(!modal.is_open());

    modal.open();
    assert!(modal.is_open());

    modal.close_with_result("test_result".to_string());
    assert!(!modal.is_open());
    assert_eq!(modal.get_result(), Some(&"test_result".to_string()));
  }

  #[test]
  fn test_modal_keyboard_handling() {
    let mut modal = alert_modal("alert", "Test", "Test message");
    modal.open();

    // Test escape key
    assert!(modal.handle_key("Escape"));
    assert!(!modal.is_open());

    // Reopen and test enter key
    modal.open();
    assert!(modal.handle_key("Enter"));
    assert!(!modal.is_open());
    assert_eq!(modal.get_result(), Some(&"ok".to_string()));
  }

  #[test]
  fn test_modal_button() {
    let button = ModalButton::new("save", "Save Changes")
      .variant("primary")
      .default()
      .closes_modal()
      .action("save_data");

    assert_eq!(button.id, "save");
    assert_eq!(button.label, "Save Changes");
    assert_eq!(button.variant, "primary");
    assert!(button.is_default);
    assert!(button.closes_modal);
    assert_eq!(button.action, Some("save_data".to_string()));
  }

  #[test]
  fn test_convenience_functions() {
    let alert = alert_modal("alert", "Error", "Something went wrong");
    assert!(matches!(alert.modal_type, ModalType::Alert { .. }));
    assert_eq!(alert.size, ModalSize::Small);

    let confirm = confirm_modal("confirm", "Delete File", "Are you sure?");
    assert!(matches!(confirm.modal_type, ModalType::Confirm { .. }));
    assert_eq!(confirm.size, ModalSize::Medium);

    let prompt = prompt_modal("prompt", "Enter Name", "Please enter your name:", "John");
    assert!(matches!(prompt.modal_type, ModalType::Prompt { .. }));
    assert_eq!(prompt.size, ModalSize::Medium);
  }

  #[test]
  fn test_modal_layout_calculation() {
    let screen = LayoutRect {
      x: 0,
      y: 0,
      width: 100,
      height: 30,
    };

    let modal = Modal::builder("test")
      .size(ModalSize::Medium)
      .position(ModalPosition::Center)
      .build();

    let layout = modal.calculate_layout(&screen);
    assert_eq!(layout.width, 60);
    assert_eq!(layout.height, 20);
    assert_eq!(layout.x, 20); // (100 - 60) / 2
    assert_eq!(layout.y, 5); // (30 - 20) / 2
  }

  #[test]
  fn test_focus_navigation() {
    let mut modal = confirm_modal("confirm", "Test", "Test message");
    modal.open();

    assert_eq!(modal.focused_element, 0);

    modal.focus_next();
    assert_eq!(modal.focused_element, 1);

    modal.focus_next();
    assert_eq!(modal.focused_element, 0); // Wraps around

    modal.focus_previous();
    assert_eq!(modal.focused_element, 1);
  }
}
