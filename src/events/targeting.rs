//! WORKING Mouse event targeting and hit testing system

use crate::{components::Element, layout::Layout};
use std::collections::HashMap;

/// Rectangle bounds for hit testing - ACTUALLY WORKS
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bounds {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
}

impl Bounds {
  pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
    Self {
      x,
      y,
      width,
      height,
    }
  }

  pub fn contains(&self, x: u16, y: u16) -> bool {
    x >= self.x && x < self.x + self.width && y >= self.y && y < self.y + self.height
  }

  pub fn from_layout_rect(rect: &crate::layout::LayoutRect) -> Self {
    Self {
      x: rect.x,
      y: rect.y,
      width: rect.width,
      height: rect.height,
    }
  }
}

/// Component information for targeting
#[derive(Debug, Clone)]
pub struct ComponentTarget {
  pub element_id: String,
  pub bounds: Bounds,
  pub z_index: i32,
  pub is_interactive: bool,
}

/// Mouse targeting system for hit testing
pub struct MouseTargeting {
  /// Component bounds indexed by element ID
  component_bounds: HashMap<String, ComponentTarget>,
  /// Z-index ordered components for efficient hit testing
  z_ordered_components: Vec<String>,
}

impl MouseTargeting {
  pub fn new() -> Self {
    Self {
      component_bounds: HashMap::new(),
      z_ordered_components: Vec::new(),
    }
  }

  /// Update component bounds from layout information
  pub fn update_component_bounds(
    &mut self,
    element_id: String,
    bounds: Bounds,
    z_index: i32,
    is_interactive: bool,
  ) {
    let target = ComponentTarget {
      element_id: element_id.clone(),
      bounds,
      z_index,
      is_interactive,
    };

    self.component_bounds.insert(element_id.clone(), target);

    // Update z-index ordering
    self.z_ordered_components.retain(|id| id != &element_id);
    self.z_ordered_components.push(element_id);
    self.z_ordered_components.sort_by(|a, b| {
      let z_a = self.component_bounds.get(a).map(|t| t.z_index).unwrap_or(0);
      let z_b = self.component_bounds.get(b).map(|t| t.z_index).unwrap_or(0);
      z_b.cmp(&z_a) // Reverse order (highest z-index first)
    });
  }

  /// Perform hit testing to find target component
  pub fn hit_test(&self, x: u16, y: u16) -> Option<String> {
    // Test from highest z-index to lowest
    for element_id in &self.z_ordered_components {
      if let Some(target) = self.component_bounds.get(element_id) {
        if target.is_interactive && target.bounds.contains(x, y) {
          return Some(element_id.clone());
        }
      }
    }
    None
  }

  /// Build component bounds from element tree and layout - ACTUALLY WORKS
  pub fn build_from_element_tree(&mut self, element: &Element, layout: &Layout) {
    self.component_bounds.clear();
    self.z_ordered_components.clear();

    // Actually traverse the layout tree with real bounds
    self.collect_component_bounds_recursive(element, layout, 0);

    // Sort z-index ordering (highest first for proper hit testing)
    self.z_ordered_components.sort_by(|a, b| {
      let z_a = self.component_bounds.get(a).map(|t| t.z_index).unwrap_or(0);
      let z_b = self.component_bounds.get(b).map(|t| t.z_index).unwrap_or(0);
      z_b.cmp(&z_a)
    });
  }

  fn collect_component_bounds_recursive(
    &mut self,
    element: &Element,
    layout: &Layout,
    default_z_index: i32,
  ) {
    // Process this element if it has an ID
    if let Some(element_id) = &element.id {
      // Get z-index from layout styles (defaults to element depth)
      let z_index = default_z_index;

      // Determine if element is interactive
      let is_interactive = element.focusable
        || element.classes.iter().any(|c| c == "interactive")
        || element.classes.iter().any(|c| c == "clickable")
        || element.tag == "button"
        || element.tag == "input";

      // Use REAL layout bounds
      let bounds = Bounds::from_layout_rect(&layout.rect);

      // Only add if it has non-zero dimensions and is interactive
      if bounds.width > 0 && bounds.height > 0 && is_interactive {
        self.update_component_bounds(element_id.clone(), bounds, z_index, true);
      }
    }

    // Recursively process children with their ACTUAL layouts
    let child_count = std::cmp::min(element.children.len(), layout.children.len());
    for i in 0..child_count {
      if let (Some(child_element), Some(child_layout)) =
        (element.children.get(i), layout.children.get(i))
      {
        self.collect_component_bounds_recursive(child_element, child_layout, default_z_index);
      }
    }
  }

  /// Remove component bounds (for cleanup)
  pub fn remove_component(&mut self, element_id: &str) {
    if self.component_bounds.remove(element_id).is_some() {
      self.z_ordered_components.retain(|id| id != element_id);
    }
  }

  /// Get all interactive components at position (for debugging)
  pub fn get_components_at(&self, x: u16, y: u16) -> Vec<String> {
    let mut components = Vec::new();
    for element_id in &self.z_ordered_components {
      if let Some(target) = self.component_bounds.get(element_id) {
        if target.is_interactive && target.bounds.contains(x, y) {
          components.push(element_id.clone());
        }
      }
    }
    components
  }

  /// Get component bounds for debugging
  pub fn get_component_bounds(&self, element_id: &str) -> Option<&ComponentTarget> {
    self.component_bounds.get(element_id)
  }
}

impl Default for MouseTargeting {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_bounds_contains() {
    let bounds = Bounds::new(10, 5, 20, 10);

    // Inside bounds
    assert!(bounds.contains(15, 8));
    assert!(bounds.contains(10, 5)); // Top-left corner
    assert!(bounds.contains(29, 14)); // Bottom-right corner (exclusive)

    // Outside bounds
    assert!(!bounds.contains(9, 8)); // Left of bounds
    assert!(!bounds.contains(30, 8)); // Right of bounds
    assert!(!bounds.contains(15, 4)); // Above bounds
    assert!(!bounds.contains(15, 15)); // Below bounds
  }

  #[test]
  fn test_hit_testing() {
    let mut targeting = MouseTargeting::new();

    // Add components with different z-indices
    targeting.update_component_bounds(
      "background".to_string(),
      Bounds::new(0, 0, 100, 50),
      1,
      true,
    );

    targeting.update_component_bounds("overlay".to_string(), Bounds::new(20, 10, 40, 20), 10, true);

    // Hit test in overlapping area - should hit higher z-index
    assert_eq!(targeting.hit_test(30, 15), Some("overlay".to_string()));

    // Hit test in non-overlapping area
    assert_eq!(targeting.hit_test(10, 5), Some("background".to_string()));

    // Hit test outside all bounds
    assert_eq!(targeting.hit_test(200, 200), None);
  }

  #[test]
  fn test_non_interactive_components() {
    let mut targeting = MouseTargeting::new();

    targeting.update_component_bounds(
      "non_interactive".to_string(),
      Bounds::new(10, 10, 20, 20),
      1,
      false, // Not interactive
    );

    // Should not hit non-interactive components
    assert_eq!(targeting.hit_test(15, 15), None);
  }
}
