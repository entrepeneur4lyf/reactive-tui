//! Overlay positioning system for floating UI elements
//!
//! Provides utilities for positioning and rendering UI elements that float
//! above the main content, such as toasts, modals, dropdowns, and tooltips.

use crate::error::Result;
use crate::layout::LayoutRect;
use serde::{Deserialize, Serialize};

/// Position for overlay elements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum OverlayPosition {
  /// Top left corner with padding
  TopLeft { padding: u16 },
  /// Top right corner with padding  
  TopRight { padding: u16 },
  /// Bottom left corner with padding
  BottomLeft { padding: u16 },
  /// Bottom right corner with padding
  BottomRight { padding: u16 },
  /// Center of the screen
  Center,
  /// Custom absolute position
  Absolute { x: u16, y: u16 },
}

impl Default for OverlayPosition {
  fn default() -> Self {
    Self::TopRight { padding: 2 }
  }
}

/// Style configuration for overlay elements
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OverlayStyle {
  /// Whether to show a border around the overlay
  pub border: bool,
  /// Border color (CSS color string)
  pub border_color: Option<String>,
  /// Background color (CSS color string)
  pub background_color: Option<String>,
  /// Shadow effect
  pub shadow: bool,
  /// Maximum width of the overlay
  pub max_width: Option<u16>,
  /// Maximum height of the overlay
  pub max_height: Option<u16>,
}

impl Default for OverlayStyle {
  fn default() -> Self {
    Self {
      border: true,
      border_color: None,
      background_color: None,
      shadow: false,
      max_width: Some(80),
      max_height: Some(40),
    }
  }
}

/// Manages overlay positioning and rendering
pub struct OverlayManager {
  viewport_width: u16,
  viewport_height: u16,
}

impl OverlayManager {
  /// Create a new overlay manager with viewport dimensions
  pub fn new(viewport_width: u16, viewport_height: u16) -> Self {
    Self {
      viewport_width,
      viewport_height,
    }
  }

  /// Update viewport dimensions (for handling terminal resize)
  pub fn update_viewport(&mut self, width: u16, height: u16) {
    self.viewport_width = width;
    self.viewport_height = height;
  }

  /// Calculate the position rectangle for an overlay
  pub fn calculate_position(
    &self,
    content_width: u16,
    content_height: u16,
    position: OverlayPosition,
    style: &OverlayStyle,
  ) -> Result<LayoutRect> {
    // Apply max width/height constraints
    let width = style
      .max_width
      .map_or(content_width, |max| content_width.min(max));
    let height = style
      .max_height
      .map_or(content_height, |max| content_height.min(max));

    // Calculate position based on strategy
    let (x, y) = match position {
      OverlayPosition::TopLeft { padding } => (padding, padding),

      OverlayPosition::TopRight { padding } => {
        let x = self.viewport_width.saturating_sub(width + padding);
        (x, padding)
      }

      OverlayPosition::BottomLeft { padding } => {
        let y = self.viewport_height.saturating_sub(height + padding);
        (padding, y)
      }

      OverlayPosition::BottomRight { padding } => {
        let x = self.viewport_width.saturating_sub(width + padding);
        let y = self.viewport_height.saturating_sub(height + padding);
        (x, y)
      }

      OverlayPosition::Center => {
        let x = (self.viewport_width.saturating_sub(width)) / 2;
        let y = (self.viewport_height.saturating_sub(height)) / 2;
        (x, y)
      }

      OverlayPosition::Absolute { x, y } => (x, y),
    };

    // Ensure position is within viewport bounds
    let x = x.min(self.viewport_width.saturating_sub(width));
    let y = y.min(self.viewport_height.saturating_sub(height));

    Ok(LayoutRect {
      x,
      y,
      width,
      height,
    })
  }

  /// Stack multiple overlays vertically
  pub fn stack_overlays(
    &self,
    overlays: &[(u16, u16, OverlayPosition, &OverlayStyle)], // (width, height, position, style)
  ) -> Result<Vec<LayoutRect>> {
    let mut positions = Vec::new();
    let mut occupied_regions = Vec::new();

    for (width, height, position, style) in overlays {
      let mut rect = self.calculate_position(*width, *height, *position, style)?;

      // Check for collisions and adjust position if needed
      while self.overlaps_any(&rect, &occupied_regions) {
        match position {
          OverlayPosition::TopRight { padding } => {
            // Move down if overlapping
            rect.y += rect.height + 1;
            if rect.y + rect.height >= self.viewport_height.saturating_sub(*padding) {
              // No more room, break
              break;
            }
          }
          _ => break, // For other positions, don't auto-adjust
        }
      }

      occupied_regions.push(rect);
      positions.push(rect);
    }

    Ok(positions)
  }

  /// Check if a rectangle overlaps with any in a list
  fn overlaps_any(&self, rect: &LayoutRect, others: &[LayoutRect]) -> bool {
    others.iter().any(|other| self.rects_overlap(rect, other))
  }

  /// Check if two rectangles overlap
  fn rects_overlap(&self, a: &LayoutRect, b: &LayoutRect) -> bool {
    !(a.x + a.width <= b.x
      || b.x + b.width <= a.x
      || a.y + a.height <= b.y
      || b.y + b.height <= a.y)
  }

  /// Get viewport dimensions
  pub fn viewport_size(&self) -> (u16, u16) {
    (self.viewport_width, self.viewport_height)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_overlay_positioning() {
    let manager = OverlayManager::new(100, 50);
    let style = OverlayStyle::default();

    // Test top-right positioning
    let rect = manager
      .calculate_position(20, 5, OverlayPosition::TopRight { padding: 2 }, &style)
      .unwrap();
    assert_eq!(rect.x, 78); // 100 - 20 - 2
    assert_eq!(rect.y, 2);
    assert_eq!(rect.width, 20);
    assert_eq!(rect.height, 5);

    // Test center positioning
    let rect = manager
      .calculate_position(20, 5, OverlayPosition::Center, &style)
      .unwrap();
    assert_eq!(rect.x, 40); // (100 - 20) / 2
    assert_eq!(rect.y, 22); // (50 - 5) / 2
  }

  #[test]
  fn test_overlay_stacking() {
    let manager = OverlayManager::new(100, 50);
    let style = OverlayStyle::default();

    let overlays = vec![
      (20, 5, OverlayPosition::TopRight { padding: 2 }, &style),
      (25, 4, OverlayPosition::TopRight { padding: 2 }, &style),
    ];

    let positions = manager.stack_overlays(&overlays).unwrap();
    assert_eq!(positions.len(), 2);

    // First overlay should be at top
    assert_eq!(positions[0].y, 2);

    // Second overlay should be stacked below (with 1 line spacing)
    assert_eq!(positions[1].y, 8); // 2 + 5 + 1
  }

  #[test]
  fn test_viewport_bounds() {
    let manager = OverlayManager::new(10, 10);
    let style = OverlayStyle::default();

    // Test that overlay doesn't exceed viewport
    let rect = manager
      .calculate_position(15, 8, OverlayPosition::TopLeft { padding: 0 }, &style)
      .unwrap();
    assert!(rect.x + rect.width <= 10);
    assert!(rect.y + rect.height <= 10);
  }
}
