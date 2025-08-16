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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum Overflow {
  Visible,
  Hidden,
  Clip,
  Scroll,
  Auto,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct OverflowBehavior {
  pub x: Overflow,
  pub y: Overflow,
}

impl Default for OverflowBehavior {
  fn default() -> Self {
    Self {
      x: Overflow::Visible,
      y: Overflow::Visible,
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct ScrollState {
  pub offset_x: u16,
  pub offset_y: u16,
  pub content_width: u16,
  pub content_height: u16,
  pub viewport_width: u16,
  pub viewport_height: u16,
}

impl Default for ScrollState {
  fn default() -> Self {
    Self {
      offset_x: 0,
      offset_y: 0,
      content_width: 0,
      content_height: 0,
      viewport_width: 0,
      viewport_height: 0,
    }
  }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComputedStyles {
  pub display: DisplayType,
  pub position: PositionType,
  pub flex_direction: FlexDirection,
  pub flex_wrap: FlexWrap,
  pub justify_content: JustifyContent,
  pub align_items: AlignItems,
  pub align_content: AlignContent,
  pub align_self: AlignSelf,
  pub flex_grow: f32,
  pub flex_shrink: f32,
  pub flex_basis: SizeValue,
  pub order: i32,
  pub z_index: i32,
  pub top: SizeValue,
  pub right: SizeValue,
  pub bottom: SizeValue,
  pub left: SizeValue,
  pub padding: Spacing,
  pub margin: Spacing,
  pub width: SizeValue,
  pub height: SizeValue,
  pub min_width: SizeValue,
  pub min_height: SizeValue,
  pub max_width: SizeValue,
  pub max_height: SizeValue,
  pub overflow: OverflowBehavior,
  pub scroll_state: ScrollState,
  pub grid_template_columns: Vec<SizeValue>,
  pub grid_template_rows: Vec<SizeValue>,
  pub grid_template_areas: Vec<Vec<String>>,
  pub grid_column_gap: u16,
  pub grid_row_gap: u16,
  pub grid_column_start: GridPosition,
  pub grid_column_end: GridPosition,
  pub grid_row_start: GridPosition,
  pub grid_row_end: GridPosition,
  pub grid_area: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Breakpoint {
  pub name: String,
  pub min_width: Option<u16>,
  pub max_width: Option<u16>,
  pub min_height: Option<u16>,
  pub max_height: Option<u16>,
}

impl Breakpoint {
  pub fn matches(&self, width: u16, height: u16) -> bool {
    let width_match = match (self.min_width, self.max_width) {
      (Some(min), Some(max)) => width >= min && width <= max,
      (Some(min), None) => width >= min,
      (None, Some(max)) => width <= max,
      (None, None) => true,
    };

    let height_match = match (self.min_height, self.max_height) {
      (Some(min), Some(max)) => height >= min && height <= max,
      (Some(min), None) => height >= min,
      (None, Some(max)) => height <= max,
      (None, None) => true,
    };

    width_match && height_match
  }
}

#[derive(Debug, Clone)]
pub struct ResponsiveStyles {
  pub base: ComputedStyles,
  pub breakpoints: Vec<(Breakpoint, ComputedStyles)>,
}

impl ResponsiveStyles {
  pub fn resolve(&self, viewport_width: u16, viewport_height: u16) -> ComputedStyles {
    let mut styles = self.base.clone();

    // Apply matching breakpoint styles in order
    for (breakpoint, breakpoint_styles) in &self.breakpoints {
      if breakpoint.matches(viewport_width, viewport_height) {
        styles = self.merge_styles(styles, breakpoint_styles.clone());
      }
    }

    styles
  }

  fn merge_styles(&self, mut base: ComputedStyles, overlay: ComputedStyles) -> ComputedStyles {
    // Only override non-default values from overlay
    if overlay.display != DisplayType::Block {
      base.display = overlay.display;
    }
    if overlay.position != PositionType::Static {
      base.position = overlay.position;
    }
    if overlay.flex_direction != FlexDirection::Column {
      base.flex_direction = overlay.flex_direction;
    }
    if overlay.justify_content != JustifyContent::FlexStart {
      base.justify_content = overlay.justify_content;
    }
    if overlay.align_items != AlignItems::FlexStart {
      base.align_items = overlay.align_items;
    }
    if overlay.flex_grow != 0.0 {
      base.flex_grow = overlay.flex_grow;
    }
    if overlay.width != SizeValue::Auto {
      base.width = overlay.width;
    }
    if overlay.height != SizeValue::Auto {
      base.height = overlay.height;
    }
    // Add more field merging as needed

    base
  }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum GridPosition {
  Auto,
  Line(i32),
  Span(u32),
  Area(String),
}

impl Default for GridPosition {
  fn default() -> Self {
    GridPosition::Auto
  }
}

#[derive(Debug, Clone)]
pub struct GridTemplate {
  pub columns: Vec<SizeValue>,
  pub rows: Vec<SizeValue>,
  pub areas: Vec<Vec<String>>,
}

#[derive(Debug, Clone)]
pub struct GridPlacement {
  pub column_start: i32,
  pub column_end: i32,
  pub row_start: i32,
  pub row_end: i32,
}

#[derive(Debug, Clone)]
pub struct GridAreaBounds {
  pub column_start: i32,
  pub column_end: i32,
  pub row_start: i32,
  pub row_end: i32,
}

#[derive(Debug, Clone)]
pub struct LayoutProfile {
  pub total_time: std::time::Duration,
  pub style_computation_time: std::time::Duration,
  pub layout_computation_time: std::time::Duration,
  pub element_count: usize,
  pub layout_depth: usize,
  pub memory_usage: usize,
}

#[derive(Debug, Clone)]
pub struct DebugRect {
  pub rect: LayoutRect,
  pub color: DebugColor,
  pub label: String,
  pub depth: usize,
}

#[derive(Debug, Clone, Copy)]
pub enum DebugColor {
  Red,
  Green,
  Blue,
  Yellow,
  White,
}

#[derive(Debug, Clone)]
pub struct LayoutWarning {
  pub element_id: String,
  pub warning_type: WarningType,
  pub message: String,
}

#[derive(Debug, Clone, Copy)]
pub enum WarningType {
  ZeroSize,
  OutsideViewport,
  Overlap,
}

#[derive(Debug, Clone)]
pub struct StackingContext {
  pub z_index: i32,
  pub layout: Layout,
  pub children: Vec<StackingContext>,
}

impl StackingContext {
  pub fn new(layout: Layout) -> Self {
    Self {
      z_index: layout.styles.z_index,
      layout,
      children: Vec::new(),
    }
  }

  /// Flatten the stacking context tree into a sorted list for rendering
  pub fn flatten_for_rendering<'a>(&'a self) -> Vec<&'a Layout> {
    let mut result = Vec::new();
    self.collect_layouts(&mut result);
    result
  }

  fn collect_layouts<'a>(&'a self, result: &mut Vec<&'a Layout>) {
    // Add this layout
    result.push(&self.layout);

    // Sort children by z-index and collect their layouts
    let mut sorted_children: Vec<_> = self.children.iter().collect();
    sorted_children.sort_by_key(|child| child.z_index);

    for child in sorted_children {
      child.collect_layouts(result);
    }
  }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DisplayType {
  Block,
  Inline,
  Flex,
  Grid,
  None,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum JustifyContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlignItems {
  FlexStart,
  FlexEnd,
  Center,
  Stretch,
  Baseline,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum FlexWrap {
  NoWrap,
  Wrap,
  WrapReverse,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlignContent {
  FlexStart,
  FlexEnd,
  Center,
  SpaceBetween,
  SpaceAround,
  SpaceEvenly,
  Stretch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AlignSelf {
  Auto,
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

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
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
      flex_wrap: FlexWrap::NoWrap,
      justify_content: JustifyContent::FlexStart,
      align_items: AlignItems::FlexStart,
      align_content: AlignContent::Stretch,
      align_self: AlignSelf::Auto,
      flex_grow: 0.0,
      flex_shrink: 1.0,
      flex_basis: SizeValue::Auto,
      order: 0,
      z_index: 0,
      top: SizeValue::Auto,
      right: SizeValue::Auto,
      bottom: SizeValue::Auto,
      left: SizeValue::Auto,
      padding: Spacing::zero(),
      margin: Spacing::zero(),
      width: SizeValue::Auto,
      height: SizeValue::Auto,
      min_width: SizeValue::Auto,
      min_height: SizeValue::Auto,
      max_width: SizeValue::Auto,
      max_height: SizeValue::Auto,
      overflow: OverflowBehavior::default(),
      scroll_state: ScrollState::default(),
      grid_template_columns: Vec::new(),
      grid_template_rows: Vec::new(),
      grid_template_areas: Vec::new(),
      grid_column_gap: 0,
      grid_row_gap: 0,
      grid_column_start: GridPosition::default(),
      grid_column_end: GridPosition::default(),
      grid_row_start: GridPosition::default(),
      grid_row_end: GridPosition::default(),
      grid_area: None,
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

    let mut layout = Layout {
      rect,
      children,
      element_id: element.id.clone(),
      tag: element.tag.clone(),
      content: element.content.clone(),
      styles: styles.clone(),
      focused: element.focused,
      focusable: element.focusable,
    };

    // Apply overflow handling
    self.apply_overflow_clipping(&mut layout);

    Ok(layout)
  }

  /// Build stacking contexts for proper z-index rendering
  pub fn build_stacking_context(&self, layout: Layout) -> StackingContext {
    let mut context = StackingContext::new(layout);

    // Group children by z-index
    let mut z_groups: std::collections::BTreeMap<i32, Vec<Layout>> = std::collections::BTreeMap::new();

    for child in context.layout.children.drain(..) {
      let z_index = child.styles.z_index;
      z_groups.entry(z_index).or_insert_with(Vec::new).push(child);
    }

    // Create stacking contexts for each z-index group
    for (_z_index, children) in z_groups {
      for child in children {
        let child_context = self.build_stacking_context(child);
        context.children.push(child_context);
      }
    }

    // Sort children by z-index
    context.children.sort_by_key(|child| child.z_index);

    context
  }

  fn compute_element_rect(
    &self,
    _element: &Element,
    available_rect: LayoutRect,
    styles: &ComputedStyles,
  ) -> LayoutRect {
    match styles.position {
      PositionType::Absolute => self.compute_absolute_rect(available_rect, styles),
      PositionType::Fixed => self.compute_fixed_rect(styles),
      PositionType::Relative => self.compute_relative_rect(available_rect, styles),
      PositionType::Static => self.compute_static_rect(available_rect, styles),
    }
  }

  fn compute_static_rect(&self, available_rect: LayoutRect, styles: &ComputedStyles) -> LayoutRect {
    let width = self.resolve_size_value(styles.width, available_rect.width);
    let height = self.resolve_size_value(styles.height, available_rect.height);

    // Apply constraints with proper min/max enforcement
    let width = self.apply_size_constraints(
      width,
      styles.min_width,
      styles.max_width,
      available_rect.width,
    );

    let height = self.apply_size_constraints(
      height,
      styles.min_height,
      styles.max_height,
      available_rect.height,
    );

    LayoutRect {
      x: available_rect.x + styles.margin.left,
      y: available_rect.y + styles.margin.top,
      width,
      height,
    }
  }

  fn compute_relative_rect(&self, available_rect: LayoutRect, styles: &ComputedStyles) -> LayoutRect {
    let mut rect = self.compute_static_rect(available_rect, styles);

    // Apply relative offsets
    if styles.left != SizeValue::Auto {
      rect.x += self.resolve_size_value(styles.left, available_rect.width);
    }
    if styles.top != SizeValue::Auto {
      rect.y += self.resolve_size_value(styles.top, available_rect.height);
    }

    rect
  }

  fn compute_absolute_rect(&self, available_rect: LayoutRect, styles: &ComputedStyles) -> LayoutRect {
    let container_width = available_rect.width;
    let container_height = available_rect.height;

    // Calculate position
    let x = if styles.left != SizeValue::Auto {
      available_rect.x + self.resolve_size_value(styles.left, container_width)
    } else if styles.right != SizeValue::Auto {
      let right_offset = self.resolve_size_value(styles.right, container_width);
      let width = self.resolve_size_value(styles.width, container_width);
      available_rect.x + container_width - right_offset - width
    } else {
      available_rect.x
    };

    let y = if styles.top != SizeValue::Auto {
      available_rect.y + self.resolve_size_value(styles.top, container_height)
    } else if styles.bottom != SizeValue::Auto {
      let bottom_offset = self.resolve_size_value(styles.bottom, container_height);
      let height = self.resolve_size_value(styles.height, container_height);
      available_rect.y + container_height - bottom_offset - height
    } else {
      available_rect.y
    };

    // Calculate size
    let width = if styles.left != SizeValue::Auto && styles.right != SizeValue::Auto {
      let left_offset = self.resolve_size_value(styles.left, container_width);
      let right_offset = self.resolve_size_value(styles.right, container_width);
      container_width - left_offset - right_offset
    } else {
      self.resolve_size_value(styles.width, container_width)
    };

    let height = if styles.top != SizeValue::Auto && styles.bottom != SizeValue::Auto {
      let top_offset = self.resolve_size_value(styles.top, container_height);
      let bottom_offset = self.resolve_size_value(styles.bottom, container_height);
      container_height - top_offset - bottom_offset
    } else {
      self.resolve_size_value(styles.height, container_height)
    };

    // Apply constraints
    let width = self.apply_size_constraints(width, styles.min_width, styles.max_width, container_width);
    let height = self.apply_size_constraints(height, styles.min_height, styles.max_height, container_height);

    LayoutRect { x, y, width, height }
  }

  fn compute_fixed_rect(&self, styles: &ComputedStyles) -> LayoutRect {
    // Fixed positioning is relative to the viewport (terminal)
    let viewport_rect = LayoutRect {
      x: 0,
      y: 0,
      width: self.terminal_width,
      height: self.terminal_height,
    };

    self.compute_absolute_rect(viewport_rect, styles)
  }

  fn apply_size_constraints(
    &self,
    size: u16,
    min_size: SizeValue,
    max_size: SizeValue,
    available: u16,
  ) -> u16 {
    let min_resolved = match min_size {
      SizeValue::Auto => 0,
      _ => self.resolve_size_value(min_size, available),
    };

    let max_resolved = match max_size {
      SizeValue::Auto => u16::MAX,
      _ => self.resolve_size_value(max_size, available),
    };

    size.max(min_resolved).min(max_resolved)
  }

  fn apply_overflow_clipping(&self, layout: &mut Layout) {
    match (layout.styles.overflow.x, layout.styles.overflow.y) {
      (Overflow::Hidden | Overflow::Clip, _) | (_, Overflow::Hidden | Overflow::Clip) => {
        self.clip_children_to_bounds(layout);
      }
      (Overflow::Scroll | Overflow::Auto, _) | (_, Overflow::Scroll | Overflow::Auto) => {
        self.setup_scrollable_content(layout);
      }
      _ => {} // Visible - no clipping
    }
  }

  fn clip_children_to_bounds(&self, layout: &mut Layout) {
    let bounds = layout.rect;

    for child in &mut layout.children {
      // Clip child to parent bounds
      if child.rect.x < bounds.x {
        let diff = bounds.x - child.rect.x;
        child.rect.x = bounds.x;
        child.rect.width = child.rect.width.saturating_sub(diff);
      }

      if child.rect.y < bounds.y {
        let diff = bounds.y - child.rect.y;
        child.rect.y = bounds.y;
        child.rect.height = child.rect.height.saturating_sub(diff);
      }

      if child.rect.x + child.rect.width > bounds.x + bounds.width {
        child.rect.width = (bounds.x + bounds.width).saturating_sub(child.rect.x);
      }

      if child.rect.y + child.rect.height > bounds.y + bounds.height {
        child.rect.height = (bounds.y + bounds.height).saturating_sub(child.rect.y);
      }

      // Recursively clip grandchildren
      self.clip_children_to_bounds(child);
    }
  }

  fn setup_scrollable_content(&self, layout: &mut Layout) {
    // Calculate content bounds
    let mut content_width = 0u16;
    let mut content_height = 0u16;

    for child in &layout.children {
      let child_right = child.rect.x + child.rect.width;
      let child_bottom = child.rect.y + child.rect.height;

      if child_right > layout.rect.x + content_width {
        content_width = child_right - layout.rect.x;
      }

      if child_bottom > layout.rect.y + content_height {
        content_height = child_bottom - layout.rect.y;
      }
    }

    // Update scroll state
    let mut scroll_state = layout.styles.scroll_state;
    scroll_state.content_width = content_width;
    scroll_state.content_height = content_height;
    scroll_state.viewport_width = layout.rect.width;
    scroll_state.viewport_height = layout.rect.height;

    // Apply scroll offset to children
    for child in &mut layout.children {
      child.rect.x = child.rect.x.saturating_sub(scroll_state.offset_x);
      child.rect.y = child.rect.y.saturating_sub(scroll_state.offset_y);
    }

    // Clip to viewport after scrolling
    self.clip_children_to_bounds(layout);
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
    if element.children.is_empty() {
      return Ok(Vec::new());
    }

    // Collect child elements with their computed styles and sort by order
    let mut flex_items: Vec<(usize, &Element, ComputedStyles)> = element
      .children
      .iter()
      .enumerate()
      .map(|(i, child)| (i, child, self.compute_styles(child)))
      .collect();

    // Sort by order property
    flex_items.sort_by_key(|(_, _, styles)| styles.order);

    let is_row = matches!(
      styles.flex_direction,
      FlexDirection::Row | FlexDirection::RowReverse
    );
    let is_reverse = matches!(
      styles.flex_direction,
      FlexDirection::RowReverse | FlexDirection::ColumnReverse
    );

    // Handle wrapping
    if styles.flex_wrap == FlexWrap::NoWrap {
      self.compute_flex_line(&flex_items, container_rect, styles, is_row, is_reverse)
    } else {
      self.compute_flex_wrap(&flex_items, container_rect, styles, is_row, is_reverse)
    }
  }

  fn compute_flex_line(
    &self,
    flex_items: &[(usize, &Element, ComputedStyles)],
    container_rect: LayoutRect,
    container_styles: &ComputedStyles,
    is_row: bool,
    is_reverse: bool,
  ) -> Result<Vec<Layout>> {
    let main_size = if is_row { container_rect.width } else { container_rect.height };
    let cross_size = if is_row { container_rect.height } else { container_rect.width };

    // Calculate flex basis and collect flex grow/shrink
    let mut flex_basis_total = 0u16;
    let mut total_flex_grow = 0.0f32;
    let mut total_flex_shrink = 0.0f32;
    let mut item_info = Vec::new();

    for (_, child, child_styles) in flex_items {
      let basis = match child_styles.flex_basis {
        SizeValue::Auto => {
          let size = if is_row { child_styles.width } else { child_styles.height };
          self.resolve_size_value(size, main_size)
        }
        _ => self.resolve_size_value(child_styles.flex_basis, main_size),
      };

      flex_basis_total += basis;
      total_flex_grow += child_styles.flex_grow;
      total_flex_shrink += child_styles.flex_shrink;

      item_info.push((child, child_styles, basis));
    }

    // Calculate free space and distribute
    let free_space = main_size as i32 - flex_basis_total as i32;
    let mut final_sizes = Vec::new();

    if free_space > 0 && total_flex_grow > 0.0 {
      // Distribute positive free space
      for (_, child_styles, basis) in &item_info {
        let grow_amount = (free_space as f32 * child_styles.flex_grow / total_flex_grow) as u16;
        final_sizes.push(basis + grow_amount);
      }
    } else if free_space < 0 && total_flex_shrink > 0.0 {
      // Distribute negative free space
      for (_, child_styles, basis) in &item_info {
        let shrink_amount = ((-free_space) as f32 * child_styles.flex_shrink / total_flex_shrink) as u16;
        final_sizes.push(basis.saturating_sub(shrink_amount));
      }
    } else {
      // No flex distribution
      final_sizes = item_info.iter().map(|(_, _, basis)| *basis).collect();
    }

    // Position items along main axis
    let mut main_positions = self.distribute_main_axis(
      &final_sizes,
      main_size,
      container_styles.justify_content,
    );

    if is_reverse {
      main_positions.reverse();
    }

    // Create layouts
    let mut layouts = Vec::new();
    for (i, ((child, child_styles, _basis), &main_pos)) in item_info.iter().zip(main_positions.iter()).enumerate() {
      let main_size = final_sizes[i];

      // Calculate cross axis size and position
      let cross_pos = self.calculate_cross_position(
        cross_size,
        child_styles,
        container_styles.align_items,
      );

      let child_rect = if is_row {
        LayoutRect {
          x: container_rect.x + main_pos,
          y: container_rect.y + cross_pos,
          width: main_size,
          height: cross_size.saturating_sub(cross_pos),
        }
      } else {
        LayoutRect {
          x: container_rect.x + cross_pos,
          y: container_rect.y + main_pos,
          width: cross_size.saturating_sub(cross_pos),
          height: main_size,
        }
      };

      let layout = self.compute_layout_recursive(child, child_rect, child_styles)?;
      layouts.push(layout);
    }

    Ok(layouts)
  }

  fn compute_flex_wrap(
    &self,
    flex_items: &[(usize, &Element, ComputedStyles)],
    container_rect: LayoutRect,
    container_styles: &ComputedStyles,
    is_row: bool,
    is_reverse: bool,
  ) -> Result<Vec<Layout>> {
    let main_size = if is_row { container_rect.width } else { container_rect.height };

    // Break items into lines
    let mut lines = Vec::new();
    let mut current_line = Vec::new();
    let mut current_line_size = 0u16;

    for item in flex_items {
      let (_, _, child_styles) = item;
      let basis = match child_styles.flex_basis {
        SizeValue::Auto => {
          let size = if is_row { child_styles.width } else { child_styles.height };
          self.resolve_size_value(size, main_size)
        }
        _ => self.resolve_size_value(child_styles.flex_basis, main_size),
      };

      if current_line_size + basis > main_size && !current_line.is_empty() {
        lines.push(std::mem::take(&mut current_line));
        current_line_size = 0;
      }

      current_line.push(item.clone());
      current_line_size += basis;
    }

    if !current_line.is_empty() {
      lines.push(current_line);
    }

    // Handle wrap-reverse
    if container_styles.flex_wrap == FlexWrap::WrapReverse {
      lines.reverse();
    }

    // Calculate line positions using align-content
    let cross_size = if is_row { container_rect.height } else { container_rect.width };
    let line_positions = self.distribute_cross_axis(&lines, cross_size, container_styles.align_content);

    // Process each line
    let mut all_layouts = Vec::new();
    for (line, &line_cross_pos) in lines.iter().zip(line_positions.iter()) {
      let line_rect = if is_row {
        LayoutRect {
          x: container_rect.x,
          y: container_rect.y + line_cross_pos,
          width: container_rect.width,
          height: cross_size / lines.len() as u16, // Simplified line height
        }
      } else {
        LayoutRect {
          x: container_rect.x + line_cross_pos,
          y: container_rect.y,
          width: cross_size / lines.len() as u16, // Simplified line width
          height: container_rect.height,
        }
      };

      let line_layouts = self.compute_flex_line(line, line_rect, container_styles, is_row, is_reverse)?;
      all_layouts.extend(line_layouts);
    }

    Ok(all_layouts)
  }

  fn distribute_main_axis(&self, sizes: &[u16], container_size: u16, justify: JustifyContent) -> Vec<u16> {
    let total_size: u16 = sizes.iter().sum();
    let free_space = container_size.saturating_sub(total_size);
    let mut positions = Vec::new();

    match justify {
      JustifyContent::FlexStart => {
        let mut pos = 0;
        for &size in sizes {
          positions.push(pos);
          pos += size;
        }
      }
      JustifyContent::FlexEnd => {
        let mut pos = free_space;
        for &size in sizes {
          positions.push(pos);
          pos += size;
        }
      }
      JustifyContent::Center => {
        let mut pos = free_space / 2;
        for &size in sizes {
          positions.push(pos);
          pos += size;
        }
      }
      JustifyContent::SpaceBetween => {
        if sizes.len() <= 1 {
          positions.push(0);
        } else {
          let gap = free_space / (sizes.len() - 1) as u16;
          let mut pos = 0;
          for (i, &size) in sizes.iter().enumerate() {
            positions.push(pos);
            pos += size + if i < sizes.len() - 1 { gap } else { 0 };
          }
        }
      }
      JustifyContent::SpaceAround => {
        let gap = free_space / sizes.len() as u16;
        let mut pos = gap / 2;
        for &size in sizes {
          positions.push(pos);
          pos += size + gap;
        }
      }
      JustifyContent::SpaceEvenly => {
        let gap = free_space / (sizes.len() + 1) as u16;
        let mut pos = gap;
        for &size in sizes {
          positions.push(pos);
          pos += size + gap;
        }
      }
    }

    positions
  }

  fn distribute_cross_axis(&self, lines: &[Vec<(usize, &Element, ComputedStyles)>], container_size: u16, align: AlignContent) -> Vec<u16> {
    let line_count = lines.len() as u16;
    let line_size = container_size / line_count; // Simplified equal distribution
    let mut positions = Vec::new();

    match align {
      AlignContent::FlexStart => {
        for i in 0..line_count {
          positions.push(i * line_size);
        }
      }
      AlignContent::FlexEnd => {
        let start = container_size - (line_count * line_size);
        for i in 0..line_count {
          positions.push(start + i * line_size);
        }
      }
      AlignContent::Center => {
        let start = (container_size - (line_count * line_size)) / 2;
        for i in 0..line_count {
          positions.push(start + i * line_size);
        }
      }
      AlignContent::SpaceBetween => {
        if line_count <= 1 {
          positions.push(0);
        } else {
          let gap = (container_size - (line_count * line_size)) / (line_count - 1);
          for i in 0..line_count {
            positions.push(i * (line_size + gap));
          }
        }
      }
      AlignContent::SpaceAround => {
        let gap = (container_size - (line_count * line_size)) / line_count;
        for i in 0..line_count {
          positions.push(gap / 2 + i * (line_size + gap));
        }
      }
      AlignContent::SpaceEvenly => {
        let gap = (container_size - (line_count * line_size)) / (line_count + 1);
        for i in 0..line_count {
          positions.push(gap + i * (line_size + gap));
        }
      }
      AlignContent::Stretch => {
        let stretched_size = container_size / line_count;
        for i in 0..line_count {
          positions.push(i * stretched_size);
        }
      }
    }

    positions
  }

  fn calculate_cross_position(&self, container_cross_size: u16, child_styles: &ComputedStyles, align_items: AlignItems) -> u16 {
    let align = if child_styles.align_self != AlignSelf::Auto {
      match child_styles.align_self {
        AlignSelf::FlexStart => AlignItems::FlexStart,
        AlignSelf::FlexEnd => AlignItems::FlexEnd,
        AlignSelf::Center => AlignItems::Center,
        AlignSelf::Stretch => AlignItems::Stretch,
        AlignSelf::Baseline => AlignItems::Baseline,
        AlignSelf::Auto => align_items,
      }
    } else {
      align_items
    };

    match align {
      AlignItems::FlexStart => 0,
      AlignItems::FlexEnd => container_cross_size,
      AlignItems::Center => container_cross_size / 2,
      AlignItems::Stretch => 0,
      AlignItems::Baseline => 0, // Simplified - would need baseline calculation
    }
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
    let container_styles = self.compute_styles(element);

    // Parse grid template
    let grid_template = self.parse_grid_template(&container_styles);

    // Place grid items
    let mut grid_items = Vec::new();
    for child in &element.children {
      let child_styles = self.compute_styles(child);
      let placement = self.resolve_grid_placement(&child_styles, &grid_template);
      grid_items.push((child, child_styles, placement));
    }

    // Calculate grid track sizes
    let column_sizes = self.calculate_grid_track_sizes(
      &container_styles.grid_template_columns,
      container_rect.width,
      container_styles.grid_column_gap,
    );

    let row_sizes = self.calculate_grid_track_sizes(
      &container_styles.grid_template_rows,
      container_rect.height,
      container_styles.grid_row_gap,
    );

    // Position grid items
    let mut layouts = Vec::new();
    for (child, child_styles, placement) in grid_items {
      let item_rect = self.calculate_grid_item_rect(
        &placement,
        &column_sizes,
        &row_sizes,
        container_rect,
        container_styles.grid_column_gap,
        container_styles.grid_row_gap,
      );

      let layout = self.compute_layout_recursive(child, item_rect, &child_styles)?;
      layouts.push(layout);
    }

    Ok(layouts)
  }

  fn parse_grid_template(&self, styles: &ComputedStyles) -> GridTemplate {
    GridTemplate {
      columns: styles.grid_template_columns.clone(),
      rows: styles.grid_template_rows.clone(),
      areas: styles.grid_template_areas.clone(),
    }
  }

  fn resolve_grid_placement(&self, styles: &ComputedStyles, template: &GridTemplate) -> GridPlacement {
    // Handle grid-area first
    if let Some(area_name) = &styles.grid_area {
      if let Some(area_bounds) = self.find_grid_area(area_name, &template.areas) {
        return GridPlacement {
          column_start: area_bounds.column_start,
          column_end: area_bounds.column_end,
          row_start: area_bounds.row_start,
          row_end: area_bounds.row_end,
        };
      }
    }

    // Handle individual grid positions
    GridPlacement {
      column_start: self.resolve_grid_position(&styles.grid_column_start, true),
      column_end: self.resolve_grid_position(&styles.grid_column_end, true),
      row_start: self.resolve_grid_position(&styles.grid_row_start, false),
      row_end: self.resolve_grid_position(&styles.grid_row_end, false),
    }
  }

  fn resolve_grid_position(&self, position: &GridPosition, _is_column: bool) -> i32 {
    match position {
      GridPosition::Auto => 1, // Auto-placement starts at line 1
      GridPosition::Line(line) => *line,
      GridPosition::Span(span) => *span as i32, // Will be handled differently in placement
      GridPosition::Area(_) => 1, // Should be resolved earlier
    }
  }

  fn find_grid_area(&self, area_name: &str, areas: &[Vec<String>]) -> Option<GridAreaBounds> {
    let mut min_row = usize::MAX;
    let mut max_row = 0;
    let mut min_col = usize::MAX;
    let mut max_col = 0;
    let mut found = false;

    for (row_idx, row) in areas.iter().enumerate() {
      for (col_idx, cell) in row.iter().enumerate() {
        if cell == area_name {
          found = true;
          min_row = min_row.min(row_idx);
          max_row = max_row.max(row_idx);
          min_col = min_col.min(col_idx);
          max_col = max_col.max(col_idx);
        }
      }
    }

    if found {
      Some(GridAreaBounds {
        column_start: (min_col + 1) as i32, // Grid lines are 1-indexed
        column_end: (max_col + 2) as i32,   // End is exclusive
        row_start: (min_row + 1) as i32,
        row_end: (max_row + 2) as i32,
      })
    } else {
      None
    }
  }

  fn calculate_grid_track_sizes(&self, tracks: &[SizeValue], available: u16, gap: u16) -> Vec<u16> {
    if tracks.is_empty() {
      return vec![available]; // Single track takes full space
    }

    let total_gap = gap * (tracks.len().saturating_sub(1)) as u16;
    let available_for_tracks = available.saturating_sub(total_gap);

    let mut sizes = Vec::new();
    let mut remaining = available_for_tracks;
    let mut fr_tracks = Vec::new();
    let mut total_fr = 0.0;

    // First pass: resolve fixed sizes and collect fr units
    for (i, track) in tracks.iter().enumerate() {
      match track {
        SizeValue::Pixels(px) => {
          sizes.push(*px);
          remaining = remaining.saturating_sub(*px);
        }
        SizeValue::Percent(pct) => {
          let size = ((available_for_tracks as f32) * (pct / 100.0)) as u16;
          sizes.push(size);
          remaining = remaining.saturating_sub(size);
        }
        SizeValue::Fr(fr) => {
          sizes.push(0); // Placeholder
          fr_tracks.push(i);
          total_fr += fr;
        }
        SizeValue::Auto => {
          sizes.push(0); // Will be calculated later
        }
      }
    }

    // Second pass: distribute remaining space to fr tracks
    if total_fr > 0.0 && remaining > 0 {
      let fr_unit = remaining as f32 / total_fr;
      for &track_idx in &fr_tracks {
        if let SizeValue::Fr(fr) = tracks[track_idx] {
          sizes[track_idx] = (fr * fr_unit) as u16;
        }
      }
    }

    sizes
  }

  fn calculate_grid_item_rect(
    &self,
    placement: &GridPlacement,
    column_sizes: &[u16],
    row_sizes: &[u16],
    container_rect: LayoutRect,
    column_gap: u16,
    row_gap: u16,
  ) -> LayoutRect {
    let col_start = (placement.column_start - 1).max(0) as usize;
    let col_end = (placement.column_end - 1).max(col_start as i32 + 1) as usize;
    let row_start = (placement.row_start - 1).max(0) as usize;
    let row_end = (placement.row_end - 1).max(row_start as i32 + 1) as usize;

    // Calculate position
    let mut x = container_rect.x;
    for i in 0..col_start.min(column_sizes.len()) {
      x += column_sizes[i] + if i > 0 { column_gap } else { 0 };
    }

    let mut y = container_rect.y;
    for i in 0..row_start.min(row_sizes.len()) {
      y += row_sizes[i] + if i > 0 { row_gap } else { 0 };
    }

    // Calculate size
    let mut width = 0u16;
    for i in col_start..col_end.min(column_sizes.len()) {
      width += column_sizes[i];
      if i > col_start {
        width += column_gap;
      }
    }

    let mut height = 0u16;
    for i in row_start..row_end.min(row_sizes.len()) {
      height += row_sizes[i];
      if i > row_start {
        height += row_gap;
      }
    }

    LayoutRect { x, y, width, height }
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
        "justify-around" => styles.justify_content = JustifyContent::SpaceAround,
        "justify-evenly" => styles.justify_content = JustifyContent::SpaceEvenly,
        "justify-start" => styles.justify_content = JustifyContent::FlexStart,
        "justify-end" => styles.justify_content = JustifyContent::FlexEnd,

        "items-center" => styles.align_items = AlignItems::Center,
        "items-start" => styles.align_items = AlignItems::FlexStart,
        "items-end" => styles.align_items = AlignItems::FlexEnd,
        "items-stretch" => styles.align_items = AlignItems::Stretch,
        "items-baseline" => styles.align_items = AlignItems::Baseline,

        "content-center" => styles.align_content = AlignContent::Center,
        "content-start" => styles.align_content = AlignContent::FlexStart,
        "content-end" => styles.align_content = AlignContent::FlexEnd,
        "content-between" => styles.align_content = AlignContent::SpaceBetween,
        "content-around" => styles.align_content = AlignContent::SpaceAround,
        "content-evenly" => styles.align_content = AlignContent::SpaceEvenly,
        "content-stretch" => styles.align_content = AlignContent::Stretch,

        "self-center" => styles.align_self = AlignSelf::Center,
        "self-start" => styles.align_self = AlignSelf::FlexStart,
        "self-end" => styles.align_self = AlignSelf::FlexEnd,
        "self-stretch" => styles.align_self = AlignSelf::Stretch,
        "self-baseline" => styles.align_self = AlignSelf::Baseline,
        "self-auto" => styles.align_self = AlignSelf::Auto,

        "flex-wrap" => styles.flex_wrap = FlexWrap::Wrap,
        "flex-nowrap" => styles.flex_wrap = FlexWrap::NoWrap,
        "flex-wrap-reverse" => styles.flex_wrap = FlexWrap::WrapReverse,

        // Grid
        "grid-cols-1" => styles.grid_template_columns = vec![SizeValue::Fr(1.0)],
        "grid-cols-2" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
        "grid-cols-3" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
        "grid-cols-4" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
        "grid-rows-1" => styles.grid_template_rows = vec![SizeValue::Fr(1.0)],
        "grid-rows-2" => styles.grid_template_rows = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
        "grid-rows-3" => styles.grid_template_rows = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
        "grid-rows-4" => styles.grid_template_rows = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],

        // Grid gaps
        "gap-0" => {
          styles.grid_column_gap = 0;
          styles.grid_row_gap = 0;
        }
        "gap-1" => {
          styles.grid_column_gap = 1;
          styles.grid_row_gap = 1;
        }
        "gap-2" => {
          styles.grid_column_gap = 2;
          styles.grid_row_gap = 2;
        }
        "gap-4" => {
          styles.grid_column_gap = 4;
          styles.grid_row_gap = 4;
        }
        "gap-x-1" => styles.grid_column_gap = 1,
        "gap-x-2" => styles.grid_column_gap = 2,
        "gap-x-4" => styles.grid_column_gap = 4,
        "gap-y-1" => styles.grid_row_gap = 1,
        "gap-y-2" => styles.grid_row_gap = 2,
        "gap-y-4" => styles.grid_row_gap = 4,

        // Grid placement
        "col-span-1" => {
          styles.grid_column_start = GridPosition::Auto;
          styles.grid_column_end = GridPosition::Span(1);
        }
        "col-span-2" => {
          styles.grid_column_start = GridPosition::Auto;
          styles.grid_column_end = GridPosition::Span(2);
        }
        "col-span-3" => {
          styles.grid_column_start = GridPosition::Auto;
          styles.grid_column_end = GridPosition::Span(3);
        }
        "col-span-4" => {
          styles.grid_column_start = GridPosition::Auto;
          styles.grid_column_end = GridPosition::Span(4);
        }
        "row-span-1" => {
          styles.grid_row_start = GridPosition::Auto;
          styles.grid_row_end = GridPosition::Span(1);
        }
        "row-span-2" => {
          styles.grid_row_start = GridPosition::Auto;
          styles.grid_row_end = GridPosition::Span(2);
        }
        "row-span-3" => {
          styles.grid_row_start = GridPosition::Auto;
          styles.grid_row_end = GridPosition::Span(3);
        }
        "row-span-4" => {
          styles.grid_row_start = GridPosition::Auto;
          styles.grid_row_end = GridPosition::Span(4);
        }

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

        // Flex shrink
        "flex-shrink-0" => styles.flex_shrink = 0.0,
        "flex-shrink" => styles.flex_shrink = 1.0,
        "flex-shrink-2" => styles.flex_shrink = 2.0,
        "flex-shrink-3" => styles.flex_shrink = 3.0,

        // Order
        "order-first" => styles.order = -9999,
        "order-last" => styles.order = 9999,
        "order-none" => styles.order = 0,
        "order-1" => styles.order = 1,
        "order-2" => styles.order = 2,
        "order-3" => styles.order = 3,
        "order-4" => styles.order = 4,
        "order-5" => styles.order = 5,

        // Z-index
        "z-0" => styles.z_index = 0,
        "z-10" => styles.z_index = 10,
        "z-20" => styles.z_index = 20,
        "z-30" => styles.z_index = 30,
        "z-40" => styles.z_index = 40,
        "z-50" => styles.z_index = 50,
        "z-auto" => styles.z_index = 0,

        // Position
        "static" => styles.position = PositionType::Static,
        "relative" => styles.position = PositionType::Relative,
        "absolute" => styles.position = PositionType::Absolute,
        "fixed" => styles.position = PositionType::Fixed,

        // Position values
        "top-0" => styles.top = SizeValue::Pixels(0),
        "top-1" => styles.top = SizeValue::Pixels(1),
        "top-2" => styles.top = SizeValue::Pixels(2),
        "top-4" => styles.top = SizeValue::Pixels(4),
        "top-8" => styles.top = SizeValue::Pixels(8),
        "right-0" => styles.right = SizeValue::Pixels(0),
        "right-1" => styles.right = SizeValue::Pixels(1),
        "right-2" => styles.right = SizeValue::Pixels(2),
        "right-4" => styles.right = SizeValue::Pixels(4),
        "right-8" => styles.right = SizeValue::Pixels(8),
        "bottom-0" => styles.bottom = SizeValue::Pixels(0),
        "bottom-1" => styles.bottom = SizeValue::Pixels(1),
        "bottom-2" => styles.bottom = SizeValue::Pixels(2),
        "bottom-4" => styles.bottom = SizeValue::Pixels(4),
        "bottom-8" => styles.bottom = SizeValue::Pixels(8),
        "left-0" => styles.left = SizeValue::Pixels(0),
        "left-1" => styles.left = SizeValue::Pixels(1),
        "left-2" => styles.left = SizeValue::Pixels(2),
        "left-4" => styles.left = SizeValue::Pixels(4),
        "left-8" => styles.left = SizeValue::Pixels(8),

        // Inset (all sides)
        "inset-0" => {
          styles.top = SizeValue::Pixels(0);
          styles.right = SizeValue::Pixels(0);
          styles.bottom = SizeValue::Pixels(0);
          styles.left = SizeValue::Pixels(0);
        }
        "inset-1" => {
          styles.top = SizeValue::Pixels(1);
          styles.right = SizeValue::Pixels(1);
          styles.bottom = SizeValue::Pixels(1);
          styles.left = SizeValue::Pixels(1);
        }
        "inset-2" => {
          styles.top = SizeValue::Pixels(2);
          styles.right = SizeValue::Pixels(2);
          styles.bottom = SizeValue::Pixels(2);
          styles.left = SizeValue::Pixels(2);
        }

        // Overflow
        "overflow-visible" => {
          styles.overflow.x = Overflow::Visible;
          styles.overflow.y = Overflow::Visible;
        }
        "overflow-hidden" => {
          styles.overflow.x = Overflow::Hidden;
          styles.overflow.y = Overflow::Hidden;
        }
        "overflow-clip" => {
          styles.overflow.x = Overflow::Clip;
          styles.overflow.y = Overflow::Clip;
        }
        "overflow-scroll" => {
          styles.overflow.x = Overflow::Scroll;
          styles.overflow.y = Overflow::Scroll;
        }
        "overflow-auto" => {
          styles.overflow.x = Overflow::Auto;
          styles.overflow.y = Overflow::Auto;
        }
        "overflow-x-visible" => styles.overflow.x = Overflow::Visible,
        "overflow-x-hidden" => styles.overflow.x = Overflow::Hidden,
        "overflow-x-clip" => styles.overflow.x = Overflow::Clip,
        "overflow-x-scroll" => styles.overflow.x = Overflow::Scroll,
        "overflow-x-auto" => styles.overflow.x = Overflow::Auto,
        "overflow-y-visible" => styles.overflow.y = Overflow::Visible,
        "overflow-y-hidden" => styles.overflow.y = Overflow::Hidden,
        "overflow-y-clip" => styles.overflow.y = Overflow::Clip,
        "overflow-y-scroll" => styles.overflow.y = Overflow::Scroll,
        "overflow-y-auto" => styles.overflow.y = Overflow::Auto,

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

    // Apply responsive styles based on terminal size
    self.apply_responsive_styles(styles, element)
  }

  fn apply_responsive_styles(&self, mut styles: ComputedStyles, element: &Element) -> ComputedStyles {
    let viewport_width = self.terminal_width;
    let _viewport_height = self.terminal_height;

    // Define standard breakpoints
    let breakpoints = [
      ("sm", 40, 80),   // Small terminals
      ("md", 80, 120),  // Medium terminals
      ("lg", 120, 160), // Large terminals
      ("xl", 160, u16::MAX), // Extra large terminals
    ];

    for class in &element.classes {
      // Check for responsive prefixes
      for (prefix, min_width, max_width) in &breakpoints {
        if let Some(base_class) = class.strip_prefix(&format!("{}:", prefix)) {
          let matches = if *max_width == u16::MAX {
            viewport_width >= *min_width
          } else {
            viewport_width >= *min_width && viewport_width < *max_width
          };

          if matches {
            styles = self.apply_responsive_class(styles, base_class);
          }
        }
      }
    }

    styles
  }

  fn apply_responsive_class(&self, mut styles: ComputedStyles, class: &str) -> ComputedStyles {
    match class {
      // Display
      "block" => styles.display = DisplayType::Block,
      "flex" => styles.display = DisplayType::Flex,
      "grid" => styles.display = DisplayType::Grid,
      "hidden" => styles.display = DisplayType::None,

      // Flex direction
      "flex-row" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Row;
      }
      "flex-col" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Column;
      }

      // Sizing
      "w-full" => styles.width = SizeValue::Percent(100.0),
      "w-1/2" => styles.width = SizeValue::Percent(50.0),
      "w-1/3" => styles.width = SizeValue::Percent(33.33),
      "w-2/3" => styles.width = SizeValue::Percent(66.67),
      "h-full" => styles.height = SizeValue::Percent(100.0),
      "h-1/2" => styles.height = SizeValue::Percent(50.0),

      // Grid
      "grid-cols-1" => styles.grid_template_columns = vec![SizeValue::Fr(1.0)],
      "grid-cols-2" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
      "grid-cols-3" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],
      "grid-cols-4" => styles.grid_template_columns = vec![SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0), SizeValue::Fr(1.0)],

      // Spacing
      "p-0" => styles.padding = Spacing::zero(),
      "p-1" => styles.padding = Spacing::uniform(1),
      "p-2" => styles.padding = Spacing::uniform(2),
      "p-4" => styles.padding = Spacing::uniform(4),
      "m-0" => styles.margin = Spacing::zero(),
      "m-1" => styles.margin = Spacing::uniform(1),
      "m-2" => styles.margin = Spacing::uniform(2),
      "m-4" => styles.margin = Spacing::uniform(4),

      _ => {} // Unknown responsive class
    }

    styles
  }

  /// Debug layout information - prints layout tree structure
  pub fn debug_layout(&self, layout: &Layout, depth: usize) {
    let indent = "  ".repeat(depth);
    println!(
      "{}Layout: {} ({}, {}) {}x{} - {} children",
      indent,
      layout.tag,
      layout.rect.x,
      layout.rect.y,
      layout.rect.width,
      layout.rect.height,
      layout.children.len()
    );

    for child in &layout.children {
      self.debug_layout(child, depth + 1);
    }
  }

  /// Generate layout performance metrics
  pub fn profile_layout(&self, element: &Element) -> LayoutProfile {
    let start_time = std::time::Instant::now();

    let styles = self.compute_styles(element);
    let style_time = start_time.elapsed();

    let layout_start = std::time::Instant::now();
    let layout = self.compute_layout_recursive(element, LayoutRect {
      x: 0,
      y: 0,
      width: self.terminal_width,
      height: self.terminal_height,
    }, &styles).unwrap();
    let layout_time = layout_start.elapsed();

    let total_time = start_time.elapsed();

    LayoutProfile {
      total_time,
      style_computation_time: style_time,
      layout_computation_time: layout_time,
      element_count: self.count_elements(element),
      layout_depth: self.calculate_layout_depth(&layout),
      memory_usage: self.estimate_memory_usage(&layout),
    }
  }

  fn count_elements(&self, element: &Element) -> usize {
    1 + element.children.iter().map(|child| self.count_elements(child)).sum::<usize>()
  }

  fn calculate_layout_depth(&self, layout: &Layout) -> usize {
    if layout.children.is_empty() {
      1
    } else {
      1 + layout.children.iter().map(|child| self.calculate_layout_depth(child)).max().unwrap_or(0)
    }
  }

  fn estimate_memory_usage(&self, layout: &Layout) -> usize {
    std::mem::size_of::<Layout>() +
    layout.children.iter().map(|child| self.estimate_memory_usage(child)).sum::<usize>()
  }

  /// Create visual debug overlay for layout bounds
  pub fn create_debug_overlay(&self, layout: &Layout) -> Vec<DebugRect> {
    let mut rects = Vec::new();
    self.collect_debug_rects(layout, &mut rects, 0);
    rects
  }

  fn collect_debug_rects(&self, layout: &Layout, rects: &mut Vec<DebugRect>, depth: usize) {
    let color = match depth % 4 {
      0 => DebugColor::Red,
      1 => DebugColor::Green,
      2 => DebugColor::Blue,
      3 => DebugColor::Yellow,
      _ => DebugColor::White,
    };

    rects.push(DebugRect {
      rect: layout.rect,
      color,
      label: format!("{} ({}x{})", layout.tag, layout.rect.width, layout.rect.height),
      depth,
    });

    for child in &layout.children {
      self.collect_debug_rects(child, rects, depth + 1);
    }
  }

  /// Validate layout for common issues
  pub fn validate_layout(&self, layout: &Layout) -> Vec<LayoutWarning> {
    let mut warnings = Vec::new();
    self.collect_layout_warnings(layout, &mut warnings);
    warnings
  }

  fn collect_layout_warnings(&self, layout: &Layout, warnings: &mut Vec<LayoutWarning>) {
    // Check for zero-sized elements
    if layout.rect.width == 0 || layout.rect.height == 0 {
      warnings.push(LayoutWarning {
        element_id: layout.element_id.clone().unwrap_or_else(|| "unknown".to_string()),
        warning_type: WarningType::ZeroSize,
        message: format!("Element has zero size: {}x{}", layout.rect.width, layout.rect.height),
      });
    }

    // Check for elements outside viewport
    if layout.rect.x >= self.terminal_width || layout.rect.y >= self.terminal_height {
      warnings.push(LayoutWarning {
        element_id: layout.element_id.clone().unwrap_or_else(|| "unknown".to_string()),
        warning_type: WarningType::OutsideViewport,
        message: "Element is positioned outside the viewport".to_string(),
      });
    }

    // Check for overlapping siblings (simplified check)
    for (i, child1) in layout.children.iter().enumerate() {
      for child2 in layout.children.iter().skip(i + 1) {
        if self.rects_overlap(child1.rect, child2.rect) {
          warnings.push(LayoutWarning {
            element_id: child1.element_id.clone().unwrap_or_else(|| "unknown".to_string()),
            warning_type: WarningType::Overlap,
            message: format!("Element overlaps with sibling: {}", child2.element_id.clone().unwrap_or_else(|| "unknown".to_string())),
          });
        }
      }
    }

    // Recursively check children
    for child in &layout.children {
      self.collect_layout_warnings(child, warnings);
    }
  }

  fn rects_overlap(&self, rect1: LayoutRect, rect2: LayoutRect) -> bool {
    !(rect1.x + rect1.width <= rect2.x ||
      rect2.x + rect2.width <= rect1.x ||
      rect1.y + rect1.height <= rect2.y ||
      rect2.y + rect2.height <= rect1.y)
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
