/*!
 * Tooltip/Popover Component - Contextual information overlays
 *
 * A comprehensive tooltip and popover system providing:
 * - Smart positioning with collision detection
 * - Multiple trigger modes (hover, click, focus, manual)
 * - Rich content support (text, HTML-like markup)
 * - Customizable arrows and styling
 * - Delay and animation controls
 * - Accessibility features (ARIA attributes)
 */

use crate::{
  error::Result,
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Tooltip positioning relative to target
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TooltipPosition {
  Top,
  TopStart,
  TopEnd,
  Bottom,
  BottomStart,
  BottomEnd,
  Left,
  LeftStart,
  LeftEnd,
  Right,
  RightStart,
  RightEnd,
  Auto, // Automatically choose best position
}

impl Default for TooltipPosition {
  fn default() -> Self {
    Self::Auto
  }
}

/// Tooltip trigger modes
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TooltipTrigger {
  Hover,
  Click,
  Focus,
  Manual,
}

impl Default for TooltipTrigger {
  fn default() -> Self {
    Self::Hover
  }
}

/// Tooltip content type
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TooltipContent {
  Text(String),
  Rich(Vec<TooltipElement>),
}

/// Rich content elements for tooltips
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TooltipElement {
  Text { content: String, style: Option<String> },
  Bold(String),
  Italic(String),
  Code(String),
  Link { text: String, url: String },
  Separator,
  NewLine,
}

/// Tooltip configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipConfig {
  pub position: TooltipPosition,
  pub trigger: TooltipTrigger,
  pub show_arrow: bool,
  pub show_delay: u32,    // milliseconds
  pub hide_delay: u32,    // milliseconds
  pub max_width: Option<u16>,
  pub wrap_text: bool,
  pub close_on_click: bool,
  pub close_on_escape: bool,
}

impl Default for TooltipConfig {
  fn default() -> Self {
    Self {
      position: TooltipPosition::default(),
      trigger: TooltipTrigger::default(),
      show_arrow: true,
      show_delay: 500,
      hide_delay: 0,
      max_width: Some(40),
      wrap_text: true,
      close_on_click: true,
      close_on_escape: true,
    }
  }
}

/// Tooltip styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TooltipStyle {
  pub background: String,
  pub text_color: String,
  pub border_color: String,
  pub shadow_color: String,
  pub arrow_color: String,
  pub padding: u16,
  pub border_radius: u16,
  pub font_size: u16,
  pub z_index: u16,
}

impl Default for TooltipStyle {
  fn default() -> Self {
    Self {
      background: "#333333".to_string(),
      text_color: "#ffffff".to_string(),
      border_color: "#555555".to_string(),
      shadow_color: "#00000040".to_string(),
      arrow_color: "#333333".to_string(),
      padding: 1,
      border_radius: 0, // Terminal doesn't support rounded corners
      font_size: 12,
      z_index: 1000,
    }
  }
}

/// Calculated tooltip position and dimensions
#[derive(Debug, Clone)]
struct TooltipLayout {
  pub rect: LayoutRect,
  pub position: TooltipPosition,
  pub arrow_position: Option<(u16, u16)>,
}

/// Tooltip widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tooltip {
  pub content: TooltipContent,
  pub target_rect: LayoutRect,
  pub is_visible: bool,
  pub config: TooltipConfig,
  pub style: TooltipStyle,
  pub id: String,
}

impl Tooltip {
  /// Create a new Tooltip
  pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
    Self {
      content: TooltipContent::Text(content.into()),
      target_rect: LayoutRect { x: 0, y: 0, width: 0, height: 0 },
      is_visible: false,
      config: TooltipConfig::default(),
      style: TooltipStyle::default(),
      id: id.into(),
    }
  }

  /// Create tooltip with rich content
  pub fn with_rich_content(id: impl Into<String>, elements: Vec<TooltipElement>) -> Self {
    Self {
      content: TooltipContent::Rich(elements),
      target_rect: LayoutRect { x: 0, y: 0, width: 0, height: 0 },
      is_visible: false,
      config: TooltipConfig::default(),
      style: TooltipStyle::default(),
      id: id.into(),
    }
  }

  /// Set target element position
  pub fn set_target(&mut self, rect: LayoutRect) -> &mut Self {
    self.target_rect = rect;
    self
  }

  /// Show the tooltip
  pub fn show(&mut self) {
    self.is_visible = true;
  }

  /// Hide the tooltip
  pub fn hide(&mut self) {
    self.is_visible = false;
  }

  /// Toggle tooltip visibility
  pub fn toggle(&mut self) {
    self.is_visible = !self.is_visible;
  }

  /// Calculate content dimensions
  fn calculate_content_size(&self) -> (u16, u16) {
    match &self.content {
      TooltipContent::Text(text) => {
        let max_width = self.config.max_width.unwrap_or(80);

        if self.config.wrap_text {
          let lines = self.wrap_text(text, max_width);
          let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
          (width.min(max_width), lines.len() as u16)
        } else {
          (text.len().min(max_width as usize) as u16, 1)
        }
      }
      TooltipContent::Rich(elements) => {
        let max_width = self.config.max_width.unwrap_or(80);
        let lines = self.render_rich_content_lines(elements, max_width);
        let width = lines.iter().map(|line| line.len()).max().unwrap_or(0) as u16;
        (width.min(max_width), lines.len() as u16)
      }
    }
  }

  /// Wrap text to fit within max width
  fn wrap_text(&self, text: &str, max_width: u16) -> Vec<String> {
    let mut lines = Vec::new();
    let mut current_line = String::new();

    for word in text.split_whitespace() {
      if current_line.is_empty() {
        current_line = word.to_string();
      } else if current_line.len() + word.len() + 1 <= max_width as usize {
        current_line.push(' ');
        current_line.push_str(word);
      } else {
        lines.push(current_line);
        current_line = word.to_string();
      }
    }

    if !current_line.is_empty() {
      lines.push(current_line);
    }

    if lines.is_empty() {
      lines.push(String::new());
    }

    lines
  }

  /// Render rich content to text lines
  fn render_rich_content_lines(&self, elements: &[TooltipElement], max_width: u16) -> Vec<String> {
    let mut lines = vec![String::new()];
    let mut current_line = 0;

    for element in elements {
      match element {
        TooltipElement::Text { content, .. } => {
          for word in content.split_whitespace() {
            if lines[current_line].is_empty() {
              lines[current_line] = word.to_string();
            } else if lines[current_line].len() + word.len() + 1 <= max_width as usize {
              lines[current_line].push(' ');
              lines[current_line].push_str(word);
            } else {
              lines.push(word.to_string());
              current_line += 1;
            }
          }
        }
        TooltipElement::Bold(text) => {
          let formatted = format!("**{}**", text);
          if lines[current_line].len() + formatted.len() <= max_width as usize {
            lines[current_line].push_str(&formatted);
          } else {
            lines.push(formatted);
            current_line += 1;
          }
        }
        TooltipElement::Italic(text) => {
          let formatted = format!("*{}*", text);
          if lines[current_line].len() + formatted.len() <= max_width as usize {
            lines[current_line].push_str(&formatted);
          } else {
            lines.push(formatted);
            current_line += 1;
          }
        }
        TooltipElement::Code(text) => {
          let formatted = format!("`{}`", text);
          if lines[current_line].len() + formatted.len() <= max_width as usize {
            lines[current_line].push_str(&formatted);
          } else {
            lines.push(formatted);
            current_line += 1;
          }
        }
        TooltipElement::Link { text, .. } => {
          let formatted = format!("[{}]", text);
          if lines[current_line].len() + formatted.len() <= max_width as usize {
            lines[current_line].push_str(&formatted);
          } else {
            lines.push(formatted);
            current_line += 1;
          }
        }
        TooltipElement::Separator => {
          lines.push("─".repeat(max_width as usize));
          current_line += 1;
          lines.push(String::new());
          current_line += 1;
        }
        TooltipElement::NewLine => {
          lines.push(String::new());
          current_line += 1;
        }
      }
    }

    lines
  }

  /// Calculate optimal tooltip position
  fn calculate_position(&self, screen_rect: LayoutRect) -> TooltipLayout {
    let (content_width, content_height) = self.calculate_content_size();
    let tooltip_width = content_width + self.style.padding * 2;
    let tooltip_height = content_height + self.style.padding * 2;

    let position = if self.config.position == TooltipPosition::Auto {
      self.find_best_position(screen_rect, tooltip_width, tooltip_height)
    } else {
      self.config.position
    };

    let (x, y) = self.calculate_position_coordinates(position, tooltip_width, tooltip_height);

    // Ensure tooltip stays within screen bounds
    let final_x = x.min(screen_rect.width.saturating_sub(tooltip_width));
    let final_y = y.min(screen_rect.height.saturating_sub(tooltip_height));

    TooltipLayout {
      rect: LayoutRect {
        x: final_x,
        y: final_y,
        width: tooltip_width,
        height: tooltip_height,
      },
      position,
      arrow_position: if self.config.show_arrow {
        Some(self.calculate_arrow_position(position, final_x, final_y, tooltip_width, tooltip_height))
      } else {
        None
      },
    }
  }

  /// Find the best position that fits on screen
  fn find_best_position(&self, screen_rect: LayoutRect, width: u16, height: u16) -> TooltipPosition {
    let positions = [
      TooltipPosition::Top,
      TooltipPosition::Bottom,
      TooltipPosition::Right,
      TooltipPosition::Left,
    ];

    for &position in &positions {
      let (x, y) = self.calculate_position_coordinates(position, width, height);
      if x + width <= screen_rect.width && y + height <= screen_rect.height {
        return position;
      }
    }

    TooltipPosition::Top // Fallback
  }

  /// Calculate position coordinates based on position type
  fn calculate_position_coordinates(&self, position: TooltipPosition, width: u16, height: u16) -> (u16, u16) {
    let target = &self.target_rect;

    match position {
      TooltipPosition::Top => (
        target.x + target.width / 2 - width / 2,
        target.y.saturating_sub(height + 1),
      ),
      TooltipPosition::TopStart => (
        target.x,
        target.y.saturating_sub(height + 1),
      ),
      TooltipPosition::TopEnd => (
        target.x + target.width - width,
        target.y.saturating_sub(height + 1),
      ),
      TooltipPosition::Bottom => (
        target.x + target.width / 2 - width / 2,
        target.y + target.height + 1,
      ),
      TooltipPosition::BottomStart => (
        target.x,
        target.y + target.height + 1,
      ),
      TooltipPosition::BottomEnd => (
        target.x + target.width - width,
        target.y + target.height + 1,
      ),
      TooltipPosition::Left => (
        target.x.saturating_sub(width + 1),
        target.y + target.height / 2 - height / 2,
      ),
      TooltipPosition::LeftStart => (
        target.x.saturating_sub(width + 1),
        target.y,
      ),
      TooltipPosition::LeftEnd => (
        target.x.saturating_sub(width + 1),
        target.y + target.height - height,
      ),
      TooltipPosition::Right => (
        target.x + target.width + 1,
        target.y + target.height / 2 - height / 2,
      ),
      TooltipPosition::RightStart => (
        target.x + target.width + 1,
        target.y,
      ),
      TooltipPosition::RightEnd => (
        target.x + target.width + 1,
        target.y + target.height - height,
      ),
      TooltipPosition::Auto => (0, 0), // Should not reach here
    }
  }

  /// Calculate arrow position
  fn calculate_arrow_position(&self, position: TooltipPosition, x: u16, y: u16, width: u16, height: u16) -> (u16, u16) {
    let _target = &self.target_rect;

    match position {
      TooltipPosition::Top | TooltipPosition::TopStart | TooltipPosition::TopEnd => {
        (x + width / 2, y + height)
      }
      TooltipPosition::Bottom | TooltipPosition::BottomStart | TooltipPosition::BottomEnd => {
        (x + width / 2, y)
      }
      TooltipPosition::Left | TooltipPosition::LeftStart | TooltipPosition::LeftEnd => {
        (x + width, y + height / 2)
      }
      TooltipPosition::Right | TooltipPosition::RightStart | TooltipPosition::RightEnd => {
        (x, y + height / 2)
      }
      TooltipPosition::Auto => (x, y),
    }
  }

  /// Render the tooltip
  pub fn render(&self, screen_rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    if !self.is_visible {
      return Ok(String::new());
    }

    let mut output = String::new();
    let layout = self.calculate_position(screen_rect);

    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    // Render tooltip background and border
    self.render_tooltip_box(&mut output, &layout, &bg_color, &text_color, &border_color)?;

    // Render arrow if enabled
    if let Some(arrow_pos) = layout.arrow_position {
      self.render_arrow(&mut output, arrow_pos, layout.position, theme)?;
    }

    write!(output, "\x1b[0m")?;
    Ok(output)
  }

  /// Render tooltip box
  fn render_tooltip_box(
    &self,
    output: &mut String,
    layout: &TooltipLayout,
    bg_color: &str,
    text_color: &str,
    border_color: &str,
  ) -> Result<()> {
    let rect = &layout.rect;

    // Top border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Content lines
    let content_lines = match &self.content {
      TooltipContent::Text(text) => {
        if self.config.wrap_text {
          self.wrap_text(text, rect.width - 2 * self.style.padding)
        } else {
          vec![text.clone()]
        }
      }
      TooltipContent::Rich(elements) => {
        self.render_rich_content_lines(elements, rect.width - 2 * self.style.padding)
      }
    };

    for (i, line) in content_lines.iter().enumerate() {
      let y = rect.y + 1 + i as u16 + self.style.padding;
      write!(output, "\x1b[{};{}H{}│{}{}{:<width$}{}│",
             y + 1, rect.x + 1, border_color, bg_color, text_color, line, border_color,
             width = rect.width as usize - 2)?;
    }

    // Fill remaining content area
    let content_height = content_lines.len() as u16;
    for i in content_height..rect.height - 2 {
      let y = rect.y + 1 + i + self.style.padding;
      write!(output, "\x1b[{};{}H{}│{}{:width$}{}│",
             y + 1, rect.x + 1, border_color, bg_color, "", border_color,
             width = rect.width as usize - 2)?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + rect.height, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    Ok(())
  }

  /// Render arrow
  fn render_arrow(&self, output: &mut String, pos: (u16, u16), position: TooltipPosition, theme: &ColorTheme) -> Result<()> {
    let arrow_color_def = get_palette_color(&theme.palette, &self.style.arrow_color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let arrow_color = color_to_ansi(arrow_color_def, false);

    let arrow_char = match position {
      TooltipPosition::Top | TooltipPosition::TopStart | TooltipPosition::TopEnd => "▼",
      TooltipPosition::Bottom | TooltipPosition::BottomStart | TooltipPosition::BottomEnd => "▲",
      TooltipPosition::Left | TooltipPosition::LeftStart | TooltipPosition::LeftEnd => "▶",
      TooltipPosition::Right | TooltipPosition::RightStart | TooltipPosition::RightEnd => "◀",
      TooltipPosition::Auto => "●",
    };

    write!(output, "\x1b[{};{}H{}{}", pos.1 + 1, pos.0 + 1, arrow_color, arrow_char)?;
    Ok(())
  }

  /// Handle mouse events
  pub fn handle_mouse(&mut self, x: u16, y: u16, event_type: MouseEventType) -> Result<Option<TooltipAction>> {
    match self.config.trigger {
      TooltipTrigger::Hover => {
        if self.is_point_in_target(x, y) {
          if !self.is_visible {
            self.show();
            return Ok(Some(TooltipAction::Shown));
          }
        } else if !self.is_point_in_tooltip(x, y) {
          if self.is_visible {
            self.hide();
            return Ok(Some(TooltipAction::Hidden));
          }
        }
      }
      TooltipTrigger::Click => {
        if event_type == MouseEventType::Click && self.is_point_in_target(x, y) {
          self.toggle();
          return Ok(Some(if self.is_visible { TooltipAction::Shown } else { TooltipAction::Hidden }));
        } else if self.config.close_on_click && event_type == MouseEventType::Click && !self.is_point_in_tooltip(x, y) {
          if self.is_visible {
            self.hide();
            return Ok(Some(TooltipAction::Hidden));
          }
        }
      }
      _ => {}
    }

    Ok(None)
  }

  /// Check if point is within target area
  fn is_point_in_target(&self, x: u16, y: u16) -> bool {
    x >= self.target_rect.x && x < self.target_rect.x + self.target_rect.width &&
    y >= self.target_rect.y && y < self.target_rect.y + self.target_rect.height
  }

  /// Check if point is within tooltip area
  fn is_point_in_tooltip(&self, x: u16, y: u16) -> bool {
    if !self.is_visible {
      return false;
    }

    let screen_rect = LayoutRect { x: 0, y: 0, width: 80, height: 24 }; // Simplified
    let layout = self.calculate_position(screen_rect);

    x >= layout.rect.x && x < layout.rect.x + layout.rect.width &&
    y >= layout.rect.y && y < layout.rect.y + layout.rect.height
  }

  /// Handle keyboard events
  pub fn handle_key(&mut self, key: &str) -> Result<Option<TooltipAction>> {
    if self.config.close_on_escape && key == "Escape" && self.is_visible {
      self.hide();
      Ok(Some(TooltipAction::Hidden))
    } else {
      Ok(None)
    }
  }
}

/// Mouse event types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEventType {
  Click,
  Move,
  Enter,
  Leave,
}

/// Actions that can result from tooltip interactions
#[derive(Debug, Clone, PartialEq)]
pub enum TooltipAction {
  Shown,
  Hidden,
}

/// Builder for Tooltip
pub struct TooltipBuilder {
  tooltip: Tooltip,
}

impl TooltipBuilder {
  pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
    Self {
      tooltip: Tooltip::new(id, content),
    }
  }

  pub fn position(mut self, position: TooltipPosition) -> Self {
    self.tooltip.config.position = position;
    self
  }

  pub fn trigger(mut self, trigger: TooltipTrigger) -> Self {
    self.tooltip.config.trigger = trigger;
    self
  }

  pub fn show_arrow(mut self, show: bool) -> Self {
    self.tooltip.config.show_arrow = show;
    self
  }

  pub fn max_width(mut self, width: u16) -> Self {
    self.tooltip.config.max_width = Some(width);
    self
  }

  pub fn delays(mut self, show_delay: u32, hide_delay: u32) -> Self {
    self.tooltip.config.show_delay = show_delay;
    self.tooltip.config.hide_delay = hide_delay;
    self
  }

  pub fn style(mut self, style: TooltipStyle) -> Self {
    self.tooltip.style = style;
    self
  }

  pub fn rich_content(mut self, elements: Vec<TooltipElement>) -> Self {
    self.tooltip.content = TooltipContent::Rich(elements);
    self
  }

  pub fn build(self) -> Tooltip {
    self.tooltip
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tooltip_creation() {
    let tooltip = Tooltip::new("test", "Hello, World!");
    assert_eq!(tooltip.id, "test");
    assert!(!tooltip.is_visible);

    if let TooltipContent::Text(content) = &tooltip.content {
      assert_eq!(content, "Hello, World!");
    } else {
      panic!("Expected text content");
    }
  }

  #[test]
  fn test_tooltip_visibility() {
    let mut tooltip = Tooltip::new("test", "Hello");
    assert!(!tooltip.is_visible);

    tooltip.show();
    assert!(tooltip.is_visible);

    tooltip.hide();
    assert!(!tooltip.is_visible);

    tooltip.toggle();
    assert!(tooltip.is_visible);
  }

  #[test]
  fn test_text_wrapping() {
    let tooltip = Tooltip::new("test", "This is a very long text that should be wrapped");
    let lines = tooltip.wrap_text("This is a very long text that should be wrapped", 20);
    assert!(lines.len() > 1);
    assert!(lines.iter().all(|line| line.len() <= 20));
  }

  #[test]
  fn test_rich_content() {
    let elements = vec![
      TooltipElement::Bold("Important".to_string()),
      TooltipElement::Text { content: " information".to_string(), style: None },
      TooltipElement::NewLine,
      TooltipElement::Code("code_example()".to_string()),
    ];

    let tooltip = Tooltip::with_rich_content("test", elements);
    if let TooltipContent::Rich(content) = &tooltip.content {
      assert_eq!(content.len(), 4);
    } else {
      panic!("Expected rich content");
    }
  }
}
