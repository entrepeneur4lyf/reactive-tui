/*!
 * Drawer/Sidebar Component - Sliding panel container
 *
 * A comprehensive drawer system providing:
 * - Multiple positions (left, right, top, bottom)
 * - Slide-in/slide-out animations
 * - Overlay and push modes
 * - Resizable drawers with drag handles
 * - Collapsible sections within drawers
 * - Backdrop support with click-to-close
 * - Keyboard navigation and accessibility
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Drawer position
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DrawerPosition {
  Left,
  Right,
  Top,
  Bottom,
}

impl Default for DrawerPosition {
  fn default() -> Self {
    Self::Left
  }
}

/// Drawer display mode
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DrawerMode {
  /// Drawer overlays content
  Overlay,
  /// Drawer pushes content aside
  Push,
  /// Drawer is always visible (sidebar mode)
  Persistent,
}

impl Default for DrawerMode {
  fn default() -> Self {
    Self::Overlay
  }
}

/// Drawer state
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DrawerState {
  Closed,
  Opening,
  Open,
  Closing,
}

impl Default for DrawerState {
  fn default() -> Self {
    Self::Closed
  }
}

/// Drawer content item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawerItem {
  pub id: String,
  pub label: String,
  pub icon: Option<String>,
  pub children: Vec<DrawerItem>,
  pub is_expanded: bool,
  pub is_selected: bool,
  pub is_disabled: bool,
  pub metadata: Option<String>,
}

impl DrawerItem {
  /// Create a new drawer item
  pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      icon: None,
      children: Vec::new(),
      is_expanded: false,
      is_selected: false,
      is_disabled: false,
      metadata: None,
    }
  }

  /// Add an icon to the item
  pub fn with_icon(mut self, icon: impl Into<String>) -> Self {
    self.icon = Some(icon.into());
    self
  }

  /// Add children to the item
  pub fn with_children(mut self, children: Vec<DrawerItem>) -> Self {
    self.children = children;
    self
  }

  /// Set metadata for the item
  pub fn with_metadata(mut self, metadata: impl Into<String>) -> Self {
    self.metadata = Some(metadata.into());
    self
  }

  /// Check if item has children
  pub fn has_children(&self) -> bool {
    !self.children.is_empty()
  }

  /// Toggle expansion state
  pub fn toggle_expanded(&mut self) {
    self.is_expanded = !self.is_expanded;
  }
}

/// Drawer configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawerConfig {
  pub position: DrawerPosition,
  pub mode: DrawerMode,
  pub width: u16,  // For left/right drawers
  pub height: u16, // For top/bottom drawers
  pub min_size: u16,
  pub max_size: u16,
  pub resizable: bool,
  pub collapsible: bool,
  pub show_backdrop: bool,
  pub close_on_backdrop_click: bool,
  pub close_on_escape: bool,
  pub animation_duration: u32, // milliseconds
}

impl Default for DrawerConfig {
  fn default() -> Self {
    Self {
      position: DrawerPosition::default(),
      mode: DrawerMode::default(),
      width: 20,
      height: 10,
      min_size: 10,
      max_size: 50,
      resizable: true,
      collapsible: true,
      show_backdrop: true,
      close_on_backdrop_click: true,
      close_on_escape: true,
      animation_duration: 300,
    }
  }
}

/// Drawer styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DrawerStyle {
  pub background: String,
  pub text_color: String,
  pub border_color: String,
  pub selected_bg: String,
  pub selected_text: String,
  pub hover_bg: String,
  pub hover_text: String,
  pub disabled_text: String,
  pub backdrop_color: String,
  pub handle_color: String,
  pub separator_color: String,
}

impl Default for DrawerStyle {
  fn default() -> Self {
    Self {
      background: "#f8f9fa".to_string(),
      text_color: "#333333".to_string(),
      border_color: "#dee2e6".to_string(),
      selected_bg: "#0078d4".to_string(),
      selected_text: "#ffffff".to_string(),
      hover_bg: "#e9ecef".to_string(),
      hover_text: "#333333".to_string(),
      disabled_text: "#6c757d".to_string(),
      backdrop_color: "#00000080".to_string(),
      handle_color: "#6c757d".to_string(),
      separator_color: "#dee2e6".to_string(),
    }
  }
}

/// Drawer widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Drawer {
  pub items: Vec<DrawerItem>,
  pub state: DrawerState,
  pub config: DrawerConfig,
  pub style: DrawerStyle,
  pub selected_index: Option<usize>,
  pub scroll_offset: usize,
  pub current_size: u16,
  pub is_resizing: bool,
  pub title: Option<String>,
}

impl Drawer {
  /// Create a new Drawer
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
      state: DrawerState::default(),
      config: DrawerConfig::default(),
      style: DrawerStyle::default(),
      selected_index: None,
      scroll_offset: 0,
      current_size: 20,
      is_resizing: false,
      title: None,
    }
  }

  /// Add an item to the drawer
  pub fn add_item(&mut self, item: DrawerItem) -> &mut Self {
    self.items.push(item);
    self
  }

  /// Add multiple items to the drawer
  pub fn add_items(&mut self, items: Vec<DrawerItem>) -> &mut Self {
    self.items.extend(items);
    self
  }

  /// Set drawer title
  pub fn set_title(&mut self, title: impl Into<String>) -> &mut Self {
    self.title = Some(title.into());
    self
  }

  /// Open the drawer
  pub fn open(&mut self) {
    if self.state == DrawerState::Closed {
      self.state = DrawerState::Opening;
      // In a real implementation, this would trigger animation
      self.state = DrawerState::Open;
    }
  }

  /// Close the drawer
  pub fn close(&mut self) {
    if self.state == DrawerState::Open {
      self.state = DrawerState::Closing;
      // In a real implementation, this would trigger animation
      self.state = DrawerState::Closed;
    }
  }

  /// Toggle drawer open/closed
  pub fn toggle(&mut self) {
    match self.state {
      DrawerState::Closed => self.open(),
      DrawerState::Open => self.close(),
      _ => {} // Don't toggle during animation
    }
  }

  /// Check if drawer is visible
  pub fn is_visible(&self) -> bool {
    matches!(self.state, DrawerState::Open | DrawerState::Opening | DrawerState::Closing)
  }

  /// Get current drawer size
  pub fn get_size(&self) -> u16 {
    match self.config.position {
      DrawerPosition::Left | DrawerPosition::Right => self.current_size.min(self.config.width),
      DrawerPosition::Top | DrawerPosition::Bottom => self.current_size.min(self.config.height),
    }
  }

  /// Set drawer size (for resizing)
  pub fn set_size(&mut self, size: u16) {
    let clamped_size = size.max(self.config.min_size).min(self.config.max_size);
    self.current_size = clamped_size;

    match self.config.position {
      DrawerPosition::Left | DrawerPosition::Right => {
        self.config.width = clamped_size;
      }
      DrawerPosition::Top | DrawerPosition::Bottom => {
        self.config.height = clamped_size;
      }
    }
  }

  /// Move selection up
  pub fn select_up(&mut self) {
    let visible_items = self.get_visible_items();
    if visible_items.is_empty() {
      return;
    }

    if let Some(index) = self.selected_index {
      if index > 0 {
        self.selected_index = Some(index - 1);
      } else {
        self.selected_index = Some(visible_items.len() - 1);
      }
    } else {
      self.selected_index = Some(0);
    }

    self.ensure_visible();
  }

  /// Move selection down
  pub fn select_down(&mut self) {
    let visible_items = self.get_visible_items();
    if visible_items.is_empty() {
      return;
    }

    if let Some(index) = self.selected_index {
      if index < visible_items.len() - 1 {
        self.selected_index = Some(index + 1);
      } else {
        self.selected_index = Some(0);
      }
    } else {
      self.selected_index = Some(0);
    }

    self.ensure_visible();
  }

  /// Get flattened list of visible items
  fn get_visible_items(&self) -> Vec<&DrawerItem> {
    let mut visible = Vec::new();
    self.collect_visible_items(&self.items, &mut visible);
    visible
  }

  /// Recursively collect visible items
  fn collect_visible_items<'a>(&self, items: &'a [DrawerItem], visible: &mut Vec<&'a DrawerItem>) {
    for item in items {
      visible.push(item);
      if item.is_expanded && !item.children.is_empty() {
        self.collect_visible_items(&item.children, visible);
      }
    }
  }

  /// Ensure selected item is visible in scroll area
  fn ensure_visible(&mut self) {
    if let Some(index) = self.selected_index {
      let visible_height = self.get_content_height();

      if index < self.scroll_offset {
        self.scroll_offset = index;
      } else if index >= self.scroll_offset + visible_height {
        self.scroll_offset = index - visible_height + 1;
      }
    }
  }

  /// Get available content height
  fn get_content_height(&self) -> usize {
    let total_height = match self.config.position {
      DrawerPosition::Left | DrawerPosition::Right => 24, // Simplified screen height
      DrawerPosition::Top | DrawerPosition::Bottom => self.get_size() as usize,
    };

    let header_height = if self.title.is_some() { 2 } else { 0 };
    let border_height = 2;

    total_height.saturating_sub(header_height + border_height)
  }

  /// Toggle expansion of selected item
  pub fn toggle_selected(&mut self) -> Result<Option<DrawerAction>> {
    if let Some(index) = self.selected_index {
      let visible_items = self.get_visible_items();
      if index < visible_items.len() {
        // Find the actual item in the tree and toggle it
        if let Some(item_id) = visible_items.get(index).map(|item| item.id.clone()) {
          self.toggle_item_by_id(&item_id);
          return Ok(Some(DrawerAction::ItemToggled(item_id)));
        }
      }
    }
    Ok(None)
  }

  /// Toggle item expansion by ID
  fn toggle_item_by_id(&mut self, id: &str) {
    Self::toggle_item_in_tree_static(&mut self.items, id);
  }

  /// Recursively toggle item in tree
  #[allow(dead_code)]
  fn toggle_item_in_tree(&mut self, items: &mut [DrawerItem], id: &str) -> bool {
    Self::toggle_item_in_tree_static(items, id)
  }

  /// Static version of toggle_item_in_tree to avoid borrowing issues
  fn toggle_item_in_tree_static(items: &mut [DrawerItem], id: &str) -> bool {
    for item in items {
      if item.id == id {
        item.toggle_expanded();
        return true;
      }
      if Self::toggle_item_in_tree_static(&mut item.children, id) {
        return true;
      }
    }
    false
  }

  /// Select item by ID
  pub fn select_item(&mut self, id: &str) -> Result<Option<DrawerAction>> {
    // Clear all selections first
    Self::clear_selections_static(&mut self.items);

    // Set selection for the target item
    if Self::set_selection_by_id_static(&mut self.items, id) {
      Ok(Some(DrawerAction::ItemSelected(id.to_string())))
    } else {
      Err(TuiError::component(format!("Item with ID '{}' not found", id)))
    }
  }

  /// Clear all selections in the tree
  #[allow(dead_code)]
  fn clear_selections(&mut self, items: &mut [DrawerItem]) {
    Self::clear_selections_static(items);
  }

  /// Static version of clear_selections to avoid borrowing issues
  fn clear_selections_static(items: &mut [DrawerItem]) {
    for item in items {
      item.is_selected = false;
      Self::clear_selections_static(&mut item.children);
    }
  }

  /// Set selection by ID
  #[allow(dead_code)]
  fn set_selection_by_id(&mut self, items: &mut [DrawerItem], id: &str) -> bool {
    Self::set_selection_by_id_static(items, id)
  }

  /// Static version of set_selection_by_id to avoid borrowing issues
  fn set_selection_by_id_static(items: &mut [DrawerItem], id: &str) -> bool {
    for item in items {
      if item.id == id {
        item.is_selected = true;
        return true;
      }
      if Self::set_selection_by_id_static(&mut item.children, id) {
        return true;
      }
    }
    false
  }

  /// Render the drawer
  pub fn render(&self, screen_rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    if !self.is_visible() {
      return Ok(String::new());
    }

    let mut output = String::new();

    // Render backdrop if enabled
    if self.config.show_backdrop && self.config.mode == DrawerMode::Overlay {
      self.render_backdrop(&mut output, screen_rect, theme)?;
    }

    // Render drawer panel
    let drawer_rect = self.calculate_drawer_rect(screen_rect);
    self.render_drawer_panel(&mut output, drawer_rect, theme)?;

    write!(output, "\x1b[0m")?;
    Ok(output)
  }

  /// Calculate drawer rectangle
  fn calculate_drawer_rect(&self, screen_rect: LayoutRect) -> LayoutRect {
    let size = self.get_size();

    match self.config.position {
      DrawerPosition::Left => LayoutRect {
        x: 0,
        y: 0,
        width: size,
        height: screen_rect.height,
      },
      DrawerPosition::Right => LayoutRect {
        x: screen_rect.width - size,
        y: 0,
        width: size,
        height: screen_rect.height,
      },
      DrawerPosition::Top => LayoutRect {
        x: 0,
        y: 0,
        width: screen_rect.width,
        height: size,
      },
      DrawerPosition::Bottom => LayoutRect {
        x: 0,
        y: screen_rect.height - size,
        width: screen_rect.width,
        height: size,
      },
    }
  }

  /// Render backdrop
  fn render_backdrop(&self, output: &mut String, screen_rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let backdrop_color_def = get_palette_color(&theme.palette, &self.style.backdrop_color)
      .map_err(|e| TuiError::render(e))?;
    let backdrop_color = color_to_ansi(backdrop_color_def, true);

    for y in 0..screen_rect.height {
      write!(output, "\x1b[{};1H{}{:width$}",
             y + 1, backdrop_color, "",
             width = screen_rect.width as usize)?;
    }

    Ok(())
  }

  /// Render drawer panel
  fn render_drawer_panel(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    // Draw borders
    self.render_borders(output, rect, &border_color)?;

    // Draw title if present
    let mut content_y = rect.y + 1;
    if let Some(title) = &self.title {
      self.render_title(output, rect, title, &bg_color, &text_color, &border_color)?;
      content_y += 2;
    }

    // Draw content
    let content_rect = LayoutRect {
      x: rect.x + 1,
      y: content_y,
      width: rect.width - 2,
      height: rect.height - (content_y - rect.y) - 1,
    };

    self.render_content(output, content_rect, theme)?;

    // Draw resize handle if resizable
    if self.config.resizable {
      self.render_resize_handle(output, rect, theme)?;
    }

    Ok(())
  }

  /// Render borders
  fn render_borders(&self, output: &mut String, rect: LayoutRect, border_color: &str) -> Result<()> {
    // Top border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Side borders
    for y in 1..rect.height - 1 {
      write!(output, "\x1b[{};{}H{}│", rect.y + y + 1, rect.x + 1, border_color)?;
      write!(output, "\x1b[{};{}H{}│", rect.y + y + 1, rect.x + rect.width, border_color)?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + rect.height, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    Ok(())
  }

  /// Render title
  fn render_title(&self, output: &mut String, rect: LayoutRect, title: &str, bg_color: &str, text_color: &str, border_color: &str) -> Result<()> {
    write!(output, "\x1b[{};{}H{}│{}{}{:<width$}{}│",
           rect.y + 2, rect.x + 1, border_color, bg_color, text_color, title, border_color,
           width = rect.width as usize - 2)?;

    // Title separator
    write!(output, "\x1b[{};{}H{}├", rect.y + 3, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┤")?;

    Ok(())
  }

  /// Render content
  fn render_content(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let selected_bg_def = get_palette_color(&theme.palette, &self.style.selected_bg)
      .map_err(|e| TuiError::render(e))?;
    let selected_bg = color_to_ansi(selected_bg_def, true);

    let selected_text_def = get_palette_color(&theme.palette, &self.style.selected_text)
      .map_err(|e| TuiError::render(e))?;
    let selected_text = color_to_ansi(selected_text_def, false);

    let disabled_text_def = get_palette_color(&theme.palette, &self.style.disabled_text)
      .map_err(|e| TuiError::render(e))?;
    let disabled_text = color_to_ansi(disabled_text_def, false);

    let visible_items = self.get_visible_items();
    let end_index = (self.scroll_offset + rect.height as usize).min(visible_items.len());

    for (i, item) in visible_items[self.scroll_offset..end_index].iter().enumerate() {
      let item_index = self.scroll_offset + i;
      let y = rect.y + i as u16;
      let is_selected = self.selected_index == Some(item_index);

      let (item_bg, item_text) = if is_selected {
        (selected_bg.clone(), selected_text.clone())
      } else if item.is_disabled {
        (bg_color.clone(), disabled_text.clone())
      } else {
        (bg_color.clone(), text_color.clone())
      };

      write!(output, "\x1b[{};{}H{}{}", y + 1, rect.x + 1, item_bg, item_text)?;

      // Calculate indentation level
      let indent_level = self.calculate_indent_level(item, &visible_items);
      let indent = "  ".repeat(indent_level);

      // Expansion indicator
      let expansion_indicator = if item.has_children() {
        if item.is_expanded { "▼ " } else { "▶ " }
      } else {
        "  "
      };

      // Icon
      let icon = item.icon.as_deref().unwrap_or("");
      let icon_str = if icon.is_empty() { "" } else { &format!("{} ", icon) };

      // Render item
      let content = format!("{}{}{}{}", indent, expansion_indicator, icon_str, item.label);
      let max_width = rect.width as usize;
      let truncated_content = if content.len() > max_width {
        format!("{}...", &content[..max_width - 3])
      } else {
        content
      };

      write!(output, "{:<width$}", truncated_content, width = max_width)?;
    }

    // Fill remaining space
    for i in end_index - self.scroll_offset..rect.height as usize {
      let y = rect.y + i as u16;
      write!(output, "\x1b[{};{}H{}{:width$}",
             y + 1, rect.x + 1, bg_color, "",
             width = rect.width as usize)?;
    }

    Ok(())
  }

  /// Calculate indentation level for an item
  fn calculate_indent_level(&self, _target_item: &DrawerItem, _visible_items: &[&DrawerItem]) -> usize {
    // Simplified - in a real implementation, would track parent-child relationships
    0
  }

  /// Render resize handle
  fn render_resize_handle(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let handle_color_def = get_palette_color(&theme.palette, &self.style.handle_color)
      .map_err(|e| TuiError::render(e))?;
    let handle_color = color_to_ansi(handle_color_def, false);

    match self.config.position {
      DrawerPosition::Left => {
        // Right edge handle
        for y in 1..rect.height - 1 {
          write!(output, "\x1b[{};{}H{}│", rect.y + y + 1, rect.x + rect.width, handle_color)?;
        }
      }
      DrawerPosition::Right => {
        // Left edge handle
        for y in 1..rect.height - 1 {
          write!(output, "\x1b[{};{}H{}│", rect.y + y + 1, rect.x + 1, handle_color)?;
        }
      }
      DrawerPosition::Top => {
        // Bottom edge handle
        write!(output, "\x1b[{};{}H{}", rect.y + rect.height, rect.x + 1, handle_color)?;
        for _ in 0..rect.width - 2 {
          write!(output, "─")?;
        }
      }
      DrawerPosition::Bottom => {
        // Top edge handle
        write!(output, "\x1b[{};{}H{}", rect.y + 1, rect.x + 1, handle_color)?;
        for _ in 0..rect.width - 2 {
          write!(output, "─")?;
        }
      }
    }

    Ok(())
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<Option<DrawerAction>> {
    match key {
      "ArrowUp" => {
        self.select_up();
        Ok(Some(DrawerAction::SelectionChanged))
      }
      "ArrowDown" => {
        self.select_down();
        Ok(Some(DrawerAction::SelectionChanged))
      }
      "Enter" | " " => {
        self.toggle_selected()
      }
      "Escape" => {
        if self.config.close_on_escape {
          self.close();
          Ok(Some(DrawerAction::Closed))
        } else {
          Ok(None)
        }
      }
      _ => Ok(None),
    }
  }

  /// Handle mouse events
  pub fn handle_mouse(&mut self, x: u16, y: u16, event_type: MouseEventType) -> Result<Option<DrawerAction>> {
    if !self.is_visible() {
      return Ok(None);
    }

    let screen_rect = LayoutRect { x: 0, y: 0, width: 80, height: 24 }; // Simplified
    let drawer_rect = self.calculate_drawer_rect(screen_rect);

    // Check if click is on backdrop
    if self.config.show_backdrop && self.config.close_on_backdrop_click && event_type == MouseEventType::Click {
      if !self.is_point_in_drawer(x, y, drawer_rect) {
        self.close();
        return Ok(Some(DrawerAction::Closed));
      }
    }

    // Handle clicks within drawer
    if self.is_point_in_drawer(x, y, drawer_rect) && event_type == MouseEventType::Click {
      // Calculate which item was clicked
      let content_y = if self.title.is_some() { drawer_rect.y + 3 } else { drawer_rect.y + 1 };
      if y >= content_y && y < drawer_rect.y + drawer_rect.height - 1 {
        let item_index = (y - content_y) as usize + self.scroll_offset;
        let visible_items = self.get_visible_items();

        if item_index < visible_items.len() {
          if let Some(item_id) = visible_items.get(item_index).map(|item| item.id.clone()) {
            self.selected_index = Some(item_index);
            return Ok(Some(DrawerAction::ItemClicked(item_id)));
          }
        }
      }
    }

    Ok(None)
  }

  /// Check if point is within drawer area
  fn is_point_in_drawer(&self, x: u16, y: u16, drawer_rect: LayoutRect) -> bool {
    x >= drawer_rect.x && x < drawer_rect.x + drawer_rect.width &&
    y >= drawer_rect.y && y < drawer_rect.y + drawer_rect.height
  }
}

impl Default for Drawer {
  fn default() -> Self {
    Self::new()
  }
}

/// Mouse event types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEventType {
  Click,
  Move,
  Drag,
}

/// Actions that can result from drawer interactions
#[derive(Debug, Clone, PartialEq)]
pub enum DrawerAction {
  Opened,
  Closed,
  SelectionChanged,
  ItemSelected(String),
  ItemClicked(String),
  ItemToggled(String),
  Resized(u16),
}

/// Builder for Drawer
pub struct DrawerBuilder {
  drawer: Drawer,
}

impl DrawerBuilder {
  pub fn new() -> Self {
    Self {
      drawer: Drawer::new(),
    }
  }

  pub fn position(mut self, position: DrawerPosition) -> Self {
    self.drawer.config.position = position;
    self
  }

  pub fn mode(mut self, mode: DrawerMode) -> Self {
    self.drawer.config.mode = mode;
    self
  }

  pub fn size(mut self, size: u16) -> Self {
    match self.drawer.config.position {
      DrawerPosition::Left | DrawerPosition::Right => {
        self.drawer.config.width = size;
      }
      DrawerPosition::Top | DrawerPosition::Bottom => {
        self.drawer.config.height = size;
      }
    }
    self.drawer.current_size = size;
    self
  }

  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.drawer.title = Some(title.into());
    self
  }

  pub fn items(mut self, items: Vec<DrawerItem>) -> Self {
    self.drawer.items = items;
    self
  }

  pub fn resizable(mut self, resizable: bool) -> Self {
    self.drawer.config.resizable = resizable;
    self
  }

  pub fn style(mut self, style: DrawerStyle) -> Self {
    self.drawer.style = style;
    self
  }

  pub fn build(self) -> Drawer {
    self.drawer
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_drawer_creation() {
    let drawer = Drawer::new();
    assert_eq!(drawer.state, DrawerState::Closed);
    assert!(drawer.items.is_empty());
    assert!(!drawer.is_visible());
  }

  #[test]
  fn test_drawer_state_transitions() {
    let mut drawer = Drawer::new();

    drawer.open();
    assert_eq!(drawer.state, DrawerState::Open);
    assert!(drawer.is_visible());

    drawer.close();
    assert_eq!(drawer.state, DrawerState::Closed);
    assert!(!drawer.is_visible());

    drawer.toggle();
    assert_eq!(drawer.state, DrawerState::Open);
  }

  #[test]
  fn test_drawer_item_creation() {
    let item = DrawerItem::new("test", "Test Item")
      .with_icon("📁")
      .with_metadata("test metadata");

    assert_eq!(item.id, "test");
    assert_eq!(item.label, "Test Item");
    assert_eq!(item.icon, Some("📁".to_string()));
    assert_eq!(item.metadata, Some("test metadata".to_string()));
    assert!(!item.has_children());
  }

  #[test]
  fn test_drawer_navigation() {
    let mut drawer = Drawer::new();
    drawer.add_items(vec![
      DrawerItem::new("1", "Item 1"),
      DrawerItem::new("2", "Item 2"),
      DrawerItem::new("3", "Item 3"),
    ]);

    assert!(drawer.selected_index.is_none());

    drawer.select_down();
    assert_eq!(drawer.selected_index, Some(0));

    drawer.select_down();
    assert_eq!(drawer.selected_index, Some(1));

    drawer.select_up();
    assert_eq!(drawer.selected_index, Some(0));
  }
}
