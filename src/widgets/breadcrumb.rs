/*!
 * Breadcrumb Component - Navigation path display
 *
 * A breadcrumb navigation widget providing:
 * - Hierarchical path display
 * - Clickable navigation segments
 * - Customizable separators and styling
 * - Overflow handling with ellipsis
 * - Icon support for path segments
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Breadcrumb navigation widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Breadcrumb {
  /// Path segments
  pub segments: Vec<BreadcrumbSegment>,
  /// Current active segment index
  pub active_index: Option<usize>,
  /// Styling configuration
  pub style: BreadcrumbStyle,
  /// Maximum width before truncation
  pub max_width: Option<u16>,
}

/// Individual breadcrumb segment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbSegment {
  /// Display label
  pub label: String,
  /// Optional icon
  pub icon: Option<String>,
  /// Navigation target/path
  pub path: String,
  /// Whether segment is clickable
  pub clickable: bool,
  /// Custom styling
  pub style_class: Option<String>,
}

/// Breadcrumb styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreadcrumbStyle {
  /// Text color for regular segments
  pub text_color: String,
  /// Text color for active segment
  pub active_text_color: String,
  /// Text color for clickable segments
  pub link_color: String,
  /// Hover color for clickable segments
  pub hover_color: String,
  /// Separator character/string
  pub separator: String,
  /// Separator color
  pub separator_color: String,
  /// Background color
  pub background: String,
  /// Padding
  pub padding: u16,
}

impl Default for BreadcrumbStyle {
  fn default() -> Self {
    Self {
      text_color: "#666666".to_string(),
      active_text_color: "#000000".to_string(),
      link_color: "#0078d4".to_string(),
      hover_color: "#106ebe".to_string(),
      separator: " > ".to_string(),
      separator_color: "#999999".to_string(),
      background: "transparent".to_string(),
      padding: 0,
    }
  }
}

impl Breadcrumb {
  /// Create a new Breadcrumb
  pub fn new() -> Self {
    Self {
      segments: Vec::new(),
      active_index: None,
      style: BreadcrumbStyle::default(),
      max_width: None,
    }
  }

  /// Add a segment to the breadcrumb
  pub fn add_segment(&mut self, segment: BreadcrumbSegment) -> &mut Self {
    self.segments.push(segment);
    self
  }

  /// Set the active segment
  pub fn set_active(&mut self, index: usize) -> Result<()> {
    if index >= self.segments.len() {
      return Err(TuiError::component("Invalid segment index".to_string()));
    }
    self.active_index = Some(index);
    Ok(())
  }

  /// Set maximum width for truncation
  pub fn max_width(mut self, width: u16) -> Self {
    self.max_width = Some(width);
    self
  }

  /// Clear all segments
  pub fn clear(&mut self) {
    self.segments.clear();
    self.active_index = None;
  }

  /// Get segment by index
  pub fn get_segment(&self, index: usize) -> Option<&BreadcrumbSegment> {
    self.segments.get(index)
  }

  /// Navigate to a specific segment
  pub fn navigate_to(&mut self, index: usize) -> Result<String> {
    if index >= self.segments.len() {
      return Err(TuiError::component("Invalid segment index".to_string()));
    }

    self.active_index = Some(index);
    Ok(self.segments[index].path.clone())
  }

  /// Calculate total display width
  fn calculate_width(&self) -> usize {
    let mut width = 0;

    for (i, segment) in self.segments.iter().enumerate() {
      // Add icon width if present
      if segment.icon.is_some() {
        width += 2; // icon + space
      }

      // Add label width
      width += segment.label.len();

      // Add separator width (except for last segment)
      if i < self.segments.len() - 1 {
        width += self.style.separator.len();
      }
    }

    width + (self.style.padding as usize * 2)
  }

  /// Truncate segments if they exceed max width
  fn truncate_segments(&self) -> Vec<(usize, &BreadcrumbSegment)> {
    let mut result = Vec::new();

    if let Some(max_width) = self.max_width {
      let total_width = self.calculate_width();

      if total_width > max_width as usize {
        // Show first segment, ellipsis, and last few segments
        if !self.segments.is_empty() {
          result.push((0, &self.segments[0]));

          if self.segments.len() > 2 {
            // Add ellipsis indicator (we'll handle this in rendering)
            let start_from = self.segments.len().saturating_sub(2);
            for (i, segment) in self.segments.iter().enumerate().skip(start_from) {
              result.push((i, segment));
            }
          } else if self.segments.len() == 2 {
            result.push((1, &self.segments[1]));
          }
        }
      } else {
        // All segments fit
        for (i, segment) in self.segments.iter().enumerate() {
          result.push((i, segment));
        }
      }
    } else {
      // No max width, show all segments
      for (i, segment) in self.segments.iter().enumerate() {
        result.push((i, segment));
      }
    }

    result
  }

  /// Render the breadcrumb
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    if self.segments.is_empty() {
      return Ok(output);
    }

    // Apply background color if not transparent
    if self.style.background != "transparent" {
      let color_def = get_palette_color(&theme.palette, &self.style.background)
        .map_err(|e| TuiError::render(e))?;
      let bg_color = color_to_ansi(color_def, true);
      write!(output, "\x1b[{};{}H{}", rect.y + 1, rect.x + 1, bg_color)?;
    }

    // Position cursor
    write!(output, "\x1b[{};{}H", rect.y + 1, rect.x + self.style.padding + 1)?;

    let segments_to_render = self.truncate_segments();
    let mut show_ellipsis = false;

    // Check if we need to show ellipsis
    if let Some(max_width) = self.max_width {
      if self.calculate_width() > max_width as usize && segments_to_render.len() < self.segments.len() {
        show_ellipsis = true;
      }
    }

    for (display_index, (original_index, segment)) in segments_to_render.iter().enumerate() {
      // Show ellipsis after first segment if needed
      if show_ellipsis && display_index == 1 {
        let color_def = get_palette_color(&theme.palette, &self.style.separator_color)
          .map_err(|e| TuiError::render(e))?;
        let separator_color = color_to_ansi(color_def, false);
        write!(output, "{} ... {}", separator_color, self.style.separator)?;
      }

      // Determine colors for this segment
      let is_active = self.active_index == Some(*original_index);
      let is_clickable = segment.clickable;

      let text_color = if is_active {
        let color_def = get_palette_color(&theme.palette, &self.style.active_text_color)
          .map_err(|e| TuiError::render(e))?;
        color_to_ansi(color_def, false)
      } else if is_clickable {
        let color_def = get_palette_color(&theme.palette, &self.style.link_color)
          .map_err(|e| TuiError::render(e))?;
        color_to_ansi(color_def, false)
      } else {
        let color_def = get_palette_color(&theme.palette, &self.style.text_color)
          .map_err(|e| TuiError::render(e))?;
        color_to_ansi(color_def, false)
      };

      // Render icon if present
      if let Some(ref icon) = segment.icon {
        write!(output, "{}{} ", text_color, icon)?;
      }

      // Render label
      write!(output, "{}{}", text_color, segment.label)?;

      // Add separator if not the last segment
      if display_index < segments_to_render.len() - 1 {
        let color_def = get_palette_color(&theme.palette, &self.style.separator_color)
          .map_err(|e| TuiError::render(e))?;
        let separator_color = color_to_ansi(color_def, false);
        write!(output, "{}{}", separator_color, self.style.separator)?;
      }
    }

    // Reset colors
    write!(output, "\x1b[0m")?;

    Ok(output)
  }

  /// Handle click at position
  pub fn handle_click(&mut self, x: u16, y: u16, rect: LayoutRect) -> Result<Option<String>> {
    if y != rect.y {
      return Ok(None); // Click not on breadcrumb line
    }

    let click_x = x.saturating_sub(rect.x + self.style.padding);
    let mut current_x = 0u16;

    for (i, segment) in self.segments.iter().enumerate() {
      // Calculate segment width
      let segment_width = segment.icon.as_ref().map(|i| i.len() + 1).unwrap_or(0) + segment.label.len();

      if click_x >= current_x && click_x < current_x + segment_width as u16 {
        if segment.clickable {
          return self.navigate_to(i).map(Some);
        }
        return Ok(None);
      }

      current_x += segment_width as u16;

      // Add separator width
      if i < self.segments.len() - 1 {
        current_x += self.style.separator.len() as u16;
      }
    }

    Ok(None)
  }
}

impl Default for Breadcrumb {
  fn default() -> Self {
    Self::new()
  }
}

/// Builder for BreadcrumbSegment
pub struct BreadcrumbSegmentBuilder {
  label: String,
  icon: Option<String>,
  path: String,
  clickable: bool,
  style_class: Option<String>,
}

impl BreadcrumbSegmentBuilder {
  pub fn new(label: impl Into<String>, path: impl Into<String>) -> Self {
    Self {
      label: label.into(),
      icon: None,
      path: path.into(),
      clickable: true,
      style_class: None,
    }
  }

  pub fn icon(mut self, icon: impl Into<String>) -> Self {
    self.icon = Some(icon.into());
    self
  }

  pub fn clickable(mut self, clickable: bool) -> Self {
    self.clickable = clickable;
    self
  }

  pub fn style_class(mut self, class: impl Into<String>) -> Self {
    self.style_class = Some(class.into());
    self
  }

  pub fn build(self) -> BreadcrumbSegment {
    BreadcrumbSegment {
      label: self.label,
      icon: self.icon,
      path: self.path,
      clickable: self.clickable,
      style_class: self.style_class,
    }
  }
}

/// Builder for Breadcrumb
pub struct BreadcrumbBuilder {
  breadcrumb: Breadcrumb,
}

impl BreadcrumbBuilder {
  pub fn new() -> Self {
    Self {
      breadcrumb: Breadcrumb::new(),
    }
  }

  pub fn segment(mut self, segment: BreadcrumbSegment) -> Self {
    self.breadcrumb.add_segment(segment);
    self
  }

  pub fn max_width(mut self, width: u16) -> Self {
    self.breadcrumb.max_width = Some(width);
    self
  }

  pub fn style(mut self, style: BreadcrumbStyle) -> Self {
    self.breadcrumb.style = style;
    self
  }

  pub fn build(self) -> Breadcrumb {
    self.breadcrumb
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_breadcrumb_creation() {
    let breadcrumb = Breadcrumb::new();
    assert!(breadcrumb.segments.is_empty());
    assert_eq!(breadcrumb.active_index, None);
  }

  #[test]
  fn test_breadcrumb_segments() {
    let mut breadcrumb = Breadcrumb::new();

    let segment = BreadcrumbSegmentBuilder::new("Home", "/")
      .icon("🏠")
      .build();

    breadcrumb.add_segment(segment);

    assert_eq!(breadcrumb.segments.len(), 1);
    assert_eq!(breadcrumb.segments[0].label, "Home");
    assert_eq!(breadcrumb.segments[0].path, "/");
  }

  #[test]
  fn test_breadcrumb_navigation() {
    let mut breadcrumb = Breadcrumb::new();

    breadcrumb.add_segment(BreadcrumbSegmentBuilder::new("Home", "/").build());
    breadcrumb.add_segment(BreadcrumbSegmentBuilder::new("Documents", "/documents").build());
    breadcrumb.add_segment(BreadcrumbSegmentBuilder::new("Projects", "/documents/projects").build());

    let path = breadcrumb.navigate_to(1).unwrap();
    assert_eq!(path, "/documents");
    assert_eq!(breadcrumb.active_index, Some(1));
  }

  #[test]
  fn test_breadcrumb_builder() {
    let breadcrumb = BreadcrumbBuilder::new()
      .segment(BreadcrumbSegmentBuilder::new("Home", "/").icon("🏠").build())
      .segment(BreadcrumbSegmentBuilder::new("Docs", "/docs").build())
      .max_width(50)
      .build();

    assert_eq!(breadcrumb.segments.len(), 2);
    assert_eq!(breadcrumb.max_width, Some(50));
  }
}
