//! Advanced CSS-based layout system with flexbox and grid support

pub mod advanced_grid;
pub mod grid;
pub mod grid_debug;

use crate::components::Element;
use crate::error::{Result, TuiError};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Layout {
  pub rect: LayoutRect,
  pub children: Vec<Layout>,
  pub element_id: Option<String>,
  pub tag: String,
  pub content: Option<String>,
  pub styles: ComputedStyles,
  pub focused: bool,
  pub focusable: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct LayoutRect {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedStyles {
  pub display: DisplayType,
  pub position: PositionType,
  pub flex_direction: FlexDirection,
  pub justify_content: JustifyContent,
  pub align_items: AlignItems,
  pub flex_grow: f32,
  pub padding: Spacing,
  pub margin: Spacing,
  pub width: SizeValue,
  pub height: SizeValue,
  pub min_width: SizeValue,
  pub min_height: SizeValue,
  pub max_width: SizeValue,
  pub max_height: SizeValue,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DisplayType {
  Block,
  Inline,
  Flex,
  Grid,
  None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PositionType {
  Static,
  Relative,
  Absolute,
  Fixed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FlexDirection {
  Row,
  Column,
  RowReverse,
  ColumnReverse,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum JustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum AlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  Baseline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Spacing {
  pub top: u16,
  pub right: u16,
  pub bottom: u16,
  pub left: u16,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum SizeValue {
  Auto,
  Pixels(u16),
  Percent(f32),
  Fr(f32),
}

impl Default for ComputedStyles {
  fn default() -> Self {
    Self {
      display: DisplayType::Block,
      position: PositionType::Static,
      flex_direction: FlexDirection::Column,
      justify_content: JustifyContent::FlexStart,
      align_items: AlignItems::FlexStart,
      flex_grow: 0.0,
      padding: Spacing::zero(),
      margin: Spacing::zero(),
      width: SizeValue::Auto,
      height: SizeValue::Auto,
      min_width: SizeValue::Auto,
      min_height: SizeValue::Auto,
      max_width: SizeValue::Auto,
      max_height: SizeValue::Auto,
    }
  }
}

impl Spacing {
  pub fn zero() -> Self {
    Self {
      top: 0,
      right: 0,
      bottom: 0,
      left: 0,
    }
  }

  pub fn uniform(value: u16) -> Self {
    Self {
      top: value,
      right: value,
      bottom: value,
      left: value,
    }
  }

  pub fn horizontal_vertical(horizontal: u16, vertical: u16) -> Self {
    Self {
      top: vertical,
      right: horizontal,
      bottom: vertical,
      left: horizontal,
    }
  }
}

pub struct LayoutEngine {
  terminal_width: u16,
  terminal_height: u16,
}

impl LayoutEngine {
  pub fn new() -> Self {
    // Detect actual terminal size first, fallback to modern high-resolution defaults
    // Modern devices: MacBook Pro M4: 380x245, 4K Desktop: 480x270, iPad Pro: 340x255
    // Use a reasonable default that works well across modern devices
    let (width, height) = Self::detect_terminal_size().unwrap_or((400, 200));
    Self::with_dimensions(width, height)
  }

  pub fn with_dimensions(width: u16, height: u16) -> Self {
    Self {
      terminal_width: width,
      terminal_height: height,
    }
  }

  /// Detect the current terminal size using crossterm
  pub fn detect_terminal_size() -> Result<(u16, u16)> {
    crate::compat::terminal::size()
      .map_err(|e| TuiError::component(format!("Failed to detect terminal size: {e}")))
  }

  /// Update terminal dimensions (useful for handling resize events)
  pub fn update_dimensions(&mut self, width: u16, height: u16) {
    self.terminal_width = width;
    self.terminal_height = height;
  }

  /// Refresh terminal dimensions by re-detecting the current size
  pub fn refresh_dimensions(&mut self) -> Result<()> {
    let (width, height) = Self::detect_terminal_size()?;
    self.update_dimensions(width, height);
    Ok(())
  }

  /// Get current terminal dimensions
  pub fn dimensions(&self) -> (u16, u16) {
    (self.terminal_width, self.terminal_height)
  }

  pub fn compute_layout(&mut self, element: &Element) -> Result<Layout> {
    let styles = self.compute_styles(element);
    let available_rect = LayoutRect {
      x: 0,
      y: 0,
      width: self.terminal_width,
      height: self.terminal_height,
    };

    self.compute_layout_recursive(element, available_rect, &styles)
  }

  /// Compute layout using component tree with per-element styles
  pub fn compute_layout_with_component_tree(
    &mut self,
    element: &Element,
    component_tree: &crate::css::ComponentTree,
  ) -> Result<Layout> {
    let available_rect = LayoutRect {
      x: 0,
      y: 0,
      width: self.terminal_width,
      height: self.terminal_height,
    };

    self.compute_layout_with_component_tree_recursive(
      element,
      component_tree.root(),
      available_rect,
    )
  }

  /// Compute layout using pre-computed styles from CSS engine
  pub fn compute_layout_with_styles(
    &mut self,
    element: &Element,
    styles: &ComputedStyles,
  ) -> Result<Layout> {
    let available_rect = LayoutRect {
      x: 0,
      y: 0,
      width: self.terminal_width,
      height: self.terminal_height,
    };

    self.compute_layout_recursive(element, available_rect, styles)
  }

  fn compute_layout_recursive(
    &self,
    element: &Element,
    available_rect: LayoutRect,
    styles: &ComputedStyles,
  ) -> Result<Layout> {
    let rect = self.compute_element_rect(element, available_rect, styles);

    // Apply padding to content area
    let content_rect = LayoutRect {
      x: rect.x + styles.padding.left,
      y: rect.y + styles.padding.top,
      width: rect
        .width
        .saturating_sub(styles.padding.left + styles.padding.right),
      height: rect
        .height
        .saturating_sub(styles.padding.top + styles.padding.bottom),
    };

    // Compute child layouts based on display type
    let children = match styles.display {
      DisplayType::None => Vec::new(),
      DisplayType::Flex => self.compute_flex_children(element, content_rect, styles)?,
      DisplayType::Grid => self.compute_grid_children(element, content_rect)?,
      DisplayType::Block => self.compute_block_children(element, content_rect)?,
      DisplayType::Inline => self.compute_inline_children(element, content_rect)?,
    };

    Ok(Layout {
      rect,
      children,
      element_id: element.id.clone(),
      tag: element.tag.clone(),
      content: element.content.clone(),
      styles: styles.clone(),
      focused: element.focused,
      focusable: element.focusable,
    })
  }

  fn compute_element_rect(
    &self,
    _element: &Element,
    available_rect: LayoutRect,
    styles: &ComputedStyles,
  ) -> LayoutRect {
    let width = self.resolve_size_value(styles.width, available_rect.width);
    let height = self.resolve_size_value(styles.height, available_rect.height);

    // Apply constraints
    let width = width
      .max(self.resolve_size_value(styles.min_width, available_rect.width))
      .min(self.resolve_size_value(styles.max_width, available_rect.width));

    let height = height
      .max(self.resolve_size_value(styles.min_height, available_rect.height))
      .min(self.resolve_size_value(styles.max_height, available_rect.height));

    LayoutRect {
      x: available_rect.x + styles.margin.left,
      y: available_rect.y + styles.margin.top,
      width,
      height,
    }
  }

  fn resolve_size_value(&self, value: SizeValue, available: u16) -> u16 {
    match value {
      SizeValue::Auto => available,
      SizeValue::Pixels(px) => px,
      SizeValue::Percent(pct) => ((available as f32) * (pct / 100.0)) as u16,
      SizeValue::Fr(fraction) => {
        // Fr units take up a fraction of the remaining space
        // This is a simplified implementation - proper flex layout would
        // calculate total fr units and distribute space proportionally
        ((available as f32) * fraction).min(available as f32) as u16
      }
    }
  }

  fn compute_flex_children(
    &self,
    element: &Element,
    container_rect: LayoutRect,
    styles: &ComputedStyles,
  ) -> Result<Vec<Layout>> {
    let mut children = Vec::new();
    let is_row = matches!(
      styles.flex_direction,
      FlexDirection::Row | FlexDirection::RowReverse
    );
    let available_space = if is_row {
      container_rect.width
    } else {
      container_rect.height
    };

    // First pass: calculate fixed sizes and total flex grow
    let mut total_flex_grow = 0.0;
    let mut used_space = 0;
    let mut child_styles_vec = Vec::new();

    for child in &element.children {
      let child_styles = self.compute_styles(child);
      let size = if is_row {
        child_styles.width
      } else {
        child_styles.height
      };

      match size {
        SizeValue::Fr(fr) => total_flex_grow += fr,
        SizeValue::Pixels(px) => used_space += px,
        SizeValue::Percent(pct) => used_space += ((available_space as f32) * (pct / 100.0)) as u16,
        SizeValue::Auto => {
          // For auto, check if element has flex_grow
          if child_styles.flex_grow > 0.0 {
            total_flex_grow += child_styles.flex_grow;
          }
        }
      }
      child_styles_vec.push(child_styles);
    }

    let remaining_space = available_space.saturating_sub(used_space);
    let flex_unit = if total_flex_grow > 0.0 {
      remaining_space as f32 / total_flex_grow
    } else {
      0.0
    };

    // Second pass: compute actual layouts
    let mut current_pos = if is_row {
      container_rect.x
    } else {
      container_rect.y
    };

    for (i, child) in element.children.iter().enumerate() {
      let child_styles = &child_styles_vec[i];
      let size = if is_row {
        child_styles.width
      } else {
        child_styles.height
      };

      let computed_size = match size {
        SizeValue::Fr(fr) => (fr * flex_unit) as u16,
        SizeValue::Auto => {
          if child_styles.flex_grow > 0.0 {
            (child_styles.flex_grow * flex_unit) as u16
          } else if total_flex_grow == 0.0 && !element.children.is_empty() {
            remaining_space / (element.children.len() - i) as u16
          } else {
            0
          }
        }
        _ => self.resolve_size_value(size, available_space),
      };

      let child_rect = if is_row {
        LayoutRect {
          x: current_pos,
          y: container_rect.y,
          width: computed_size,
          height: container_rect.height,
        }
      } else {
        LayoutRect {
          x: container_rect.x,
          y: current_pos,
          width: container_rect.width,
          height: computed_size,
        }
      };

      let child_layout = self.compute_layout_recursive(child, child_rect, child_styles)?;
      current_pos += if is_row {
        child_layout.rect.width
      } else {
        child_layout.rect.height
      };
      children.push(child_layout);
    }

    Ok(children)
  }

  fn compute_block_children(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    let mut children = Vec::new();
    let mut current_y = container_rect.y;

    for child in &element.children {
      let child_styles = self.compute_styles(child);
      let child_rect = LayoutRect {
        x: container_rect.x,
        y: current_y,
        width: container_rect.width,
        height: container_rect
          .height
          .saturating_sub(current_y - container_rect.y),
      };

      let child_layout = self.compute_layout_recursive(child, child_rect, &child_styles)?;
      current_y += child_layout.rect.height + child_styles.margin.top + child_styles.margin.bottom;

      children.push(child_layout);
    }

    Ok(children)
  }

  fn compute_inline_children(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    let mut children = Vec::new();
    let mut current_x = container_rect.x;
    let mut current_y = container_rect.y;

    for child in &element.children {
      let child_styles = self.compute_styles(child);
      let child_width = self.resolve_size_value(child_styles.width, container_rect.width);

      // Wrap to next line if needed
      if current_x + child_width > container_rect.x + container_rect.width {
        current_x = container_rect.x;
        current_y += 1; // Simplified line height
      }

      let child_rect = LayoutRect {
        x: current_x,
        y: current_y,
        width: child_width,
        height: 1, // Simplified for inline elements
      };

      let child_layout = self.compute_layout_recursive(child, child_rect, &child_styles)?;
      current_x += child_layout.rect.width;

      children.push(child_layout);
    }

    Ok(children)
  }

  fn compute_grid_children(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    use crate::layout::advanced_grid::GridLayout as AdvancedGridLayout;

    let grid_layout = AdvancedGridLayout::new();
    let grid_result = grid_layout.compute_layout(element, container_rect)?;

    Ok(grid_result.children)
  }

  fn compute_styles(&self, element: &Element) -> ComputedStyles {
    let mut styles = ComputedStyles::default();

    // Apply default styles based on element tag
    match element.tag.as_str() {
      "div" => {
        styles.display = DisplayType::Block;
      }
      "span" => {
        styles.display = DisplayType::Inline;
      }
      "flex" | "flexbox" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Row;
      }
      "vbox" | "column" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Column;
      }
      "hbox" | "row" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Row;
      }
      "center" => {
        styles.display = DisplayType::Flex;
        styles.justify_content = JustifyContent::Center;
        styles.align_items = AlignItems::Center;
      }
      _ => {}
    }

    // Apply class-based styles
    for class in &element.classes {
      match class.as_str() {
        // Display types
        "flex" => styles.display = DisplayType::Flex,
        "grid" => styles.display = DisplayType::Grid,
        "block" => styles.display = DisplayType::Block,
        "inline" => styles.display = DisplayType::Inline,
        "hidden" => styles.display = DisplayType::None,

        // Flex styles
        "flex-row" => {
          styles.display = DisplayType::Flex;
          styles.flex_direction = FlexDirection::Row;
        }
        "flex-col" => {
          styles.display = DisplayType::Flex;
          styles.flex_direction = FlexDirection::Column;
        }
        "justify-center" => styles.justify_content = JustifyContent::Center,
        "justify-between" => styles.justify_content = JustifyContent::SpaceBetween,
        "items-center" => styles.align_items = AlignItems::Center,

        // Spacing
        "p-1" => styles.padding = Spacing::uniform(1),
        "p-2" => styles.padding = Spacing::uniform(2),
        "p-3" => styles.padding = Spacing::uniform(3),
        "p-4" => styles.padding = Spacing::uniform(4),
        "m-1" => styles.margin = Spacing::uniform(1),
        "m-2" => styles.margin = Spacing::uniform(2),
        "m-3" => styles.margin = Spacing::uniform(3),
        "m-4" => styles.margin = Spacing::uniform(4),

        // Flex grow
        "flex-1" => styles.flex_grow = 1.0,
        "flex-2" => styles.flex_grow = 2.0,
        "flex-3" => styles.flex_grow = 3.0,

        // Sizing
        "w-full" => styles.width = SizeValue::Percent(100.0),
        "h-full" => styles.height = SizeValue::Percent(100.0),
        "w-1/2" => styles.width = SizeValue::Percent(50.0),
        "w-1/3" => styles.width = SizeValue::Percent(33.33),
        "w-2/3" => styles.width = SizeValue::Percent(66.67),
        "w-1/4" => styles.width = SizeValue::Percent(25.0),
        "w-3/4" => styles.width = SizeValue::Percent(75.0),
        "h-1/2" => styles.height = SizeValue::Percent(50.0),
        "h-1/3" => styles.height = SizeValue::Percent(33.33),
        "h-2/3" => styles.height = SizeValue::Percent(66.67),
        "h-1/4" => styles.height = SizeValue::Percent(25.0),
        "h-3/4" => styles.height = SizeValue::Percent(75.0),

        _ => {}
      }
    }

    // Apply attribute-based styles
    if let Some(width) = element.get_attribute("width") {
      if let Ok(px) = width.parse::<u16>() {
        styles.width = SizeValue::Pixels(px);
      } else if width.ends_with('%') {
        if let Ok(pct) = width.trim_end_matches('%').parse::<f32>() {
          styles.width = SizeValue::Percent(pct);
        }
      }
    }

    if let Some(height) = element.get_attribute("height") {
      if let Ok(px) = height.parse::<u16>() {
        styles.height = SizeValue::Pixels(px);
      } else if height.ends_with('%') {
        if let Ok(pct) = height.trim_end_matches('%').parse::<f32>() {
          styles.height = SizeValue::Percent(pct);
        }
      }
    }

    styles
  }

  fn compute_layout_with_component_tree_recursive(
    &self,
    element: &Element,
    component_node: &crate::css::ComponentNode,
    container_rect: LayoutRect,
  ) -> Result<Layout> {
    let css_styles = &component_node.styles;
    let layout_styles = css_styles.to_layout_styles();

    // Compute this element's layout
    let mut layout = Layout {
      rect: container_rect,
      children: Vec::new(),
      element_id: element.id.clone(),
      tag: element.tag.clone(),
      content: element.content.clone(),
      styles: layout_styles.clone(),
      focused: false,
      focusable: false,
    };

    // Compute children layouts using their component tree styles
    match layout_styles.display {
      DisplayType::Flex => {
        layout.children = self.compute_flex_children_with_component_tree(
          element,
          component_node,
          container_rect,
        )?;
      }
      DisplayType::Block => {
        layout.children = self.compute_block_children_with_component_tree(
          element,
          component_node,
          container_rect,
        )?;
      }
      DisplayType::Inline => {
        layout.children = self.compute_inline_children_with_component_tree(
          element,
          component_node,
          container_rect,
        )?;
      }
      DisplayType::None => {
        // Element is hidden, no children to compute
      }
      DisplayType::Grid => {
        // Use advanced grid for component-tree-driven CSS
        let advanced = crate::layout::advanced_grid::GridLayout::new();
        // Reuse existing API to compute a grid layout for this element
        let grid_result = advanced.compute_layout(element, container_rect)?;
        layout.children = grid_result.children;
      }
    }

    Ok(layout)
  }

  fn compute_block_children_with_component_tree(
    &self,
    element: &Element,
    component_node: &crate::css::ComponentNode,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    let mut children = Vec::new();
    let mut current_y = container_rect.y;

    for (child_element, child_node) in element.children.iter().zip(component_node.children.iter()) {
      let child_styles = &child_node.styles;

      // Compute child height based on content or default to 1 line
      let child_height = if child_element.content.is_some() {
        1 // Text elements get 1 line height
      } else {
        // Container elements get height based on their children
        let child_count = child_element.children.len() as u16;
        if child_count > 0 {
          child_count
        } else {
          1
        }
      };

      let child_rect = LayoutRect {
        x: container_rect.x,
        y: current_y,
        width: container_rect.width,
        height: child_height,
      };

      let child_layout =
        self.compute_layout_with_component_tree_recursive(child_element, child_node, child_rect)?;
      current_y += child_layout.rect.height + child_styles.margin.top + child_styles.margin.bottom;

      children.push(child_layout);
    }

    Ok(children)
  }

  fn compute_flex_children_with_component_tree(
    &self,
    element: &Element,
    component_node: &crate::css::ComponentNode,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    // For now, treat flex as block layout
    self.compute_block_children_with_component_tree(element, component_node, container_rect)
  }

  fn compute_inline_children_with_component_tree(
    &self,
    element: &Element,
    component_node: &crate::css::ComponentNode,
    container_rect: LayoutRect,
  ) -> Result<Vec<Layout>> {
    // For now, treat inline as block layout
    self.compute_block_children_with_component_tree(element, component_node, container_rect)
  }
}

impl Default for LayoutEngine {
  fn default() -> Self {
    Self::new()
  }
}
