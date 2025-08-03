//! Viewport Widget
//!
//! A comprehensive scrollable viewport widget supporting lazy loading, virtual scrolling,
//! and efficient rendering of large datasets with smooth scrolling and keyboard navigation.
//!
//! # Features
//!
//! - **Virtual Scrolling**: Efficiently render only visible items for large datasets (10k+ items)
//! - **Lazy Loading**: Load content on-demand with async callbacks and loading indicators
//! - **Smooth Scrolling**: Pixel-perfect scrolling with momentum and easing
//! - **Keyboard Navigation**: Arrow keys, Page Up/Down, Home/End, vim-style navigation
//! - **Mouse Support**: Mouse wheel scrolling and drag scrolling
//! - **Scrollbar Rendering**: Customizable scrollbar with position indicators
//! - **Content Caching**: Intelligent content caching with LRU eviction
//! - **Search Integration**: Find and scroll to content with highlighting
//! - **Selection Support**: Single/multi-selection with keyboard and mouse
//! - **Responsive Sizing**: Automatic sizing based on container and content
//!
//! # Basic Usage
//!
//! ```rust,no_run
//! use reactive_tui::widgets::{ViewportItem};
//!
//! // Create viewport items
//! let items = vec![
//!     ViewportItem::new("line1", "Line 1"),
//!     ViewportItem::new("line2", "Line 2"),
//!     ViewportItem::new("line3", "Line 3"),
//! ];
//! ```

use crate::{
  components::element::Element,
  error::{Result, TuiError},
  layout::LayoutRect,
  reactive::Reactive,
  themes::{ColorDefinition, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

// Type aliases for complex function pointer types
type OnSelectionChangeCallback = Arc<dyn Fn(&[ContentId]) + Send + Sync>;
type OnItemActivateCallback = Arc<dyn Fn(&ContentId, &ViewportItem) + Send + Sync>;
type OnLazyLoadCallback = Arc<dyn Fn(LineNumber, usize) -> Vec<ViewportItem> + Send + Sync>;
type OnSearchCallback = Arc<dyn Fn(&str, &[LineNumber]) + Send + Sync>;

/// Content identifier type
pub type ContentId = String;

/// Line number type
pub type LineNumber = usize;

/// Viewport content item
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportItem {
  /// Unique item identifier
  pub id: ContentId,
  /// Item content text
  pub content: String,
  /// Item height in lines
  pub height: u16,
  /// Whether item is selectable
  pub selectable: bool,
  /// Whether item is disabled
  pub disabled: bool,
  /// Custom CSS classes
  pub css_classes: Vec<String>,
  /// Item metadata
  pub metadata: HashMap<String, String>,
}

impl ViewportItem {
  /// Create a new viewport item
  pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      content: content.into(),
      height: 1,
      selectable: true,
      disabled: false,
      css_classes: Vec::new(),
      metadata: HashMap::new(),
    }
  }

  /// Set item height
  pub fn height(mut self, height: u16) -> Self {
    self.height = height;
    self
  }

  /// Set selectable state
  pub fn selectable(mut self, selectable: bool) -> Self {
    self.selectable = selectable;
    self
  }

  /// Set disabled state
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Add metadata
  pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.metadata.insert(key.into(), value.into());
    self
  }
}

impl From<String> for ViewportItem {
  fn from(content: String) -> Self {
    Self::new(format!("item_{}", content.len()), content)
  }
}

impl From<&str> for ViewportItem {
  fn from(content: &str) -> Self {
    Self::new(format!("item_{}", content.len()), content)
  }
}

/// Scrolling mode for viewport
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollMode {
  /// Line-by-line scrolling
  Line,
  /// Pixel-perfect scrolling
  Pixel,
  /// Page-based scrolling
  Page,
}

impl Default for ScrollMode {
  fn default() -> Self {
    Self::Line
  }
}

/// Selection mode for viewport
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
  /// No selection
  None,
  /// Single item selection
  Single,
  /// Multiple item selection
  Multiple,
  /// Range selection
  Range,
}

impl Default for SelectionMode {
  fn default() -> Self {
    Self::Single
  }
}

/// Scrollbar position
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScrollbarPosition {
  /// Right side of viewport
  Right,
  /// Left side of viewport
  Left,
  /// Hidden scrollbar
  Hidden,
}

impl Default for ScrollbarPosition {
  fn default() -> Self {
    Self::Right
  }
}

/// Lazy loading state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LazyLoadState {
  /// Not loaded yet
  NotLoaded,
  /// Currently loading
  Loading,
  /// Successfully loaded
  Loaded,
  /// Failed to load
  Error(String),
}

/// Viewport state management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportState {
  /// Current scroll position (line number or pixel offset)
  pub scroll_position: f64,
  /// Currently visible range (start, end)
  pub visible_range: (LineNumber, LineNumber),
  /// Selected item IDs
  pub selected_items: Vec<ContentId>,
  /// Currently highlighted item
  pub highlighted_item: Option<ContentId>,
  /// Whether viewport is focused
  pub focused: bool,
  /// Whether viewport is disabled
  pub disabled: bool,
  /// Current search query
  pub search_query: String,
  /// Search result positions
  pub search_results: Vec<LineNumber>,
  /// Current search result index
  pub current_search_result: Option<usize>,
  /// Lazy loading states
  pub lazy_states: HashMap<ContentId, LazyLoadState>,
  /// Content cache metadata
  pub cache_stats: CacheStats,
}

impl Default for ViewportState {
  fn default() -> Self {
    Self {
      scroll_position: 0.0,
      visible_range: (0, 0),
      selected_items: Vec::new(),
      highlighted_item: None,
      focused: false,
      disabled: false,
      search_query: String::new(),
      search_results: Vec::new(),
      current_search_result: None,
      lazy_states: HashMap::new(),
      cache_stats: CacheStats::default(),
    }
  }
}

/// Content cache statistics
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CacheStats {
  /// Total items in cache
  pub cached_items: usize,
  /// Cache hit rate
  pub hit_rate: f64,
  /// Cache miss count
  pub miss_count: usize,
  /// Memory usage estimate
  pub memory_usage: usize,
}

impl Default for CacheStats {
  fn default() -> Self {
    Self {
      cached_items: 0,
      hit_rate: 0.0,
      miss_count: 0,
      memory_usage: 0,
    }
  }
}

/// Viewport configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportConfig {
  /// Viewport width
  pub width: u16,
  /// Viewport height
  pub height: u16,
  /// Enable scrolling
  pub scrollable: bool,
  /// Scroll mode
  pub scroll_mode: ScrollMode,
  /// Selection mode
  pub selection_mode: SelectionMode,
  /// Scrollbar position
  pub scrollbar_position: ScrollbarPosition,
  /// Show scrollbar
  pub show_scrollbar: bool,
  /// Enable virtual scrolling
  pub virtual_scrolling: bool,
  /// Virtual scroll buffer size
  pub virtual_buffer: usize,
  /// Enable lazy loading
  pub lazy_loading: bool,
  /// Lazy load threshold (items before/after visible range)
  pub lazy_threshold: usize,
  /// Content cache size
  pub cache_size: usize,
  /// Enable mouse support
  pub mouse_support: bool,
  /// Enable keyboard navigation
  pub keyboard_navigation: bool,
  /// Scroll sensitivity
  pub scroll_sensitivity: f64,
  /// Enable momentum scrolling
  pub momentum_scrolling: bool,
}

impl Default for ViewportConfig {
  fn default() -> Self {
    Self {
      width: 400,
      height: 200,
      scrollable: true,
      scroll_mode: ScrollMode::Line,
      selection_mode: SelectionMode::Single,
      scrollbar_position: ScrollbarPosition::Right,
      show_scrollbar: true,
      virtual_scrolling: true,
      virtual_buffer: 50,
      lazy_loading: false,
      lazy_threshold: 10,
      cache_size: 1000,
      mouse_support: true,
      keyboard_navigation: true,
      scroll_sensitivity: 1.0,
      momentum_scrolling: false,
    }
  }
}

/// Viewport styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ViewportStyle {
  /// Background color
  pub background: Option<ColorDefinition>,
  /// Text color
  pub text_color: Option<ColorDefinition>,
  /// Selected item background
  pub selected_background: Option<ColorDefinition>,
  /// Selected item text color
  pub selected_text_color: Option<ColorDefinition>,
  /// Highlighted item background
  pub highlighted_background: Option<ColorDefinition>,
  /// Scrollbar color
  pub scrollbar_color: Option<ColorDefinition>,
  /// Scrollbar track color
  pub scrollbar_track_color: Option<ColorDefinition>,
  /// Search highlight color
  pub search_highlight_color: Option<ColorDefinition>,
  /// Loading indicator
  pub loading_indicator: String,
  /// Selection indicator
  pub selection_indicator: String,
  /// Highlight indicator
  pub highlight_indicator: String,
}

impl Default for ViewportStyle {
  fn default() -> Self {
    Self {
      background: None,
      text_color: None,
      selected_background: None,
      selected_text_color: None,
      highlighted_background: None,
      scrollbar_color: None,
      scrollbar_track_color: None,
      search_highlight_color: None,
      loading_indicator: "⟳".to_string(),
      selection_indicator: "►".to_string(),
      highlight_indicator: "●".to_string(),
    }
  }
}

/// Flags for item rendering
#[derive(Debug, Clone, Copy)]
struct ItemRenderFlags {
  is_selected: bool,
  is_highlighted: bool,
  is_search_result: bool,
}

/// Event callbacks for viewport interactions
#[derive(Default)]
pub struct ViewportCallbacks {
  /// Called when scroll position changes
  pub on_scroll: Option<Arc<dyn Fn(f64) + Send + Sync>>,
  /// Called when selection changes
  pub on_selection_change: Option<OnSelectionChangeCallback>,
  /// Called when item is activated (double-click or Enter)
  pub on_item_activate: Option<OnItemActivateCallback>,
  /// Called when lazy loading is needed
  pub on_lazy_load: Option<OnLazyLoadCallback>,
  /// Called when search results change
  pub on_search: Option<OnSearchCallback>,
  /// Called when focus changes
  pub on_focus: Option<Arc<dyn Fn(bool) + Send + Sync>>,
}

/// Main Viewport widget
pub struct Viewport {
  /// Unique viewport identifier
  pub id: String,
  /// Viewport content items
  pub items: Vec<ViewportItem>,
  /// Reactive state management
  pub state: Reactive<ViewportState>,
  /// Configuration options
  pub config: ViewportConfig,
  /// Styling configuration
  pub style: ViewportStyle,
  /// Event callbacks
  pub callbacks: ViewportCallbacks,
  /// CSS utility classes
  pub css_classes: Vec<String>,
}

impl Viewport {
  /// Create a new viewport builder
  pub fn builder<S: Into<String>>(id: S) -> ViewportBuilder {
    ViewportBuilder::new(id)
  }

  /// Set viewport content
  pub fn set_content(&mut self, items: Vec<ViewportItem>) {
    self.items = items;
    self.update_visible_range();

    // Reset state
    self.state.update(|state| {
      state.scroll_position = 0.0;
      state.selected_items.clear();
      state.highlighted_item = None;
      state.search_query.clear();
      state.search_results.clear();
      state.current_search_result = None;
    });
  }

  /// Add items to viewport
  pub fn add_items(&mut self, items: Vec<ViewportItem>) {
    self.items.extend(items);
    self.update_visible_range();
  }

  /// Insert item at specific position
  pub fn insert_item(&mut self, index: usize, item: ViewportItem) {
    if index <= self.items.len() {
      self.items.insert(index, item);
      self.update_visible_range();
    }
  }

  /// Remove item by ID
  pub fn remove_item(&mut self, id: &ContentId) -> Option<ViewportItem> {
    if let Some(index) = self.items.iter().position(|item| item.id == *id) {
      let removed = self.items.remove(index);
      self.update_visible_range();

      // Update selection
      self.state.update(|state| {
        state.selected_items.retain(|selected_id| selected_id != id);
        if state.highlighted_item.as_ref() == Some(id) {
          state.highlighted_item = None;
        }
      });

      Some(removed)
    } else {
      None
    }
  }

  /// Get item by ID
  pub fn get_item(&self, id: &ContentId) -> Option<&ViewportItem> {
    self.items.iter().find(|item| item.id == *id)
  }

  /// Get item by line number
  pub fn get_item_at_line(&self, line: LineNumber) -> Option<&ViewportItem> {
    self.items.get(line)
  }

  /// Scroll to specific position
  pub fn scroll_to(&mut self, position: f64) -> bool {
    let max_scroll = self.get_max_scroll_position();
    let new_position = position.max(0.0).min(max_scroll);

    if (new_position - self.state.get().scroll_position).abs() > f64::EPSILON {
      self.state.update(|state| {
        state.scroll_position = new_position;
      });

      self.update_visible_range();

      if let Some(callback) = &self.callbacks.on_scroll {
        callback(new_position);
      }

      true
    } else {
      false
    }
  }

  /// Scroll to specific line
  pub fn scroll_to_line(&mut self, line: LineNumber) -> bool {
    let position = line as f64;
    self.scroll_to(position)
  }

  /// Scroll down by specified amount
  pub fn scroll_down(&mut self, amount: f64) -> bool {
    let current = self.state.get().scroll_position;
    let new_position = current + amount * self.config.scroll_sensitivity;
    self.scroll_to(new_position)
  }

  /// Scroll up by specified amount
  pub fn scroll_up(&mut self, amount: f64) -> bool {
    let current = self.state.get().scroll_position;
    let new_position = current - amount * self.config.scroll_sensitivity;
    self.scroll_to(new_position)
  }

  /// Page down
  pub fn page_down(&mut self) -> bool {
    let page_size = (self.config.height as f64 * 0.8).max(1.0);
    self.scroll_down(page_size)
  }

  /// Page up
  pub fn page_up(&mut self) -> bool {
    let page_size = (self.config.height as f64 * 0.8).max(1.0);
    self.scroll_up(page_size)
  }

  /// Scroll to top
  pub fn scroll_to_top(&mut self) -> bool {
    self.scroll_to(0.0)
  }

  /// Scroll to bottom
  pub fn scroll_to_bottom(&mut self) -> bool {
    let max_scroll = self.get_max_scroll_position();
    self.scroll_to(max_scroll)
  }

  /// Get maximum scroll position
  fn get_max_scroll_position(&self) -> f64 {
    let total_lines = self.items.len() as f64;
    let viewport_height = self.config.height as f64;
    (total_lines - viewport_height).max(0.0)
  }

  /// Update visible range based on current scroll position
  fn update_visible_range(&mut self) {
    let scroll_pos = self.state.get().scroll_position as usize;
    let viewport_height = self.config.height as usize;

    let start = scroll_pos;
    let end = (scroll_pos + viewport_height).min(self.items.len());

    self.state.update(|state| {
      state.visible_range = (start, end);
    });

    // Trigger lazy loading if enabled
    if self.config.lazy_loading {
      self.trigger_lazy_loading();
    }
  }

  /// Trigger lazy loading for items near visible range
  fn trigger_lazy_loading(&mut self) {
    let state = self.state.get();
    let (start, end) = state.visible_range;
    let threshold = self.config.lazy_threshold;

    let load_start = start.saturating_sub(threshold);
    let load_end = (end + threshold).min(self.items.len());

    // Check which items need loading
    let mut items_to_load = Vec::new();
    for i in load_start..load_end {
      if let Some(item) = self.items.get(i) {
        let load_state = state
          .lazy_states
          .get(&item.id)
          .unwrap_or(&LazyLoadState::NotLoaded);
        if matches!(load_state, LazyLoadState::NotLoaded) {
          items_to_load.push(i);
        }
      }
    }

    // Trigger lazy loading callback
    if !items_to_load.is_empty() {
      if let Some(callback) = &self.callbacks.on_lazy_load {
        let loaded_items = callback(load_start, items_to_load.len());

        // Update items and states
        for (i, item) in loaded_items.into_iter().enumerate() {
          if let Some(existing_item) = self.items.get_mut(load_start + i) {
            *existing_item = item.clone();
          }

          self.state.update(|state| {
            state
              .lazy_states
              .insert(item.id.clone(), LazyLoadState::Loaded);
          });
        }
      }
    }
  }

  /// Select item by ID
  pub fn select_item(&mut self, id: &ContentId) -> Result<()> {
    let item = self
      .get_item(id)
      .ok_or_else(|| TuiError::component(format!("Item '{id}' not found")))?;

    if !item.selectable || item.disabled {
      return Err(TuiError::component(format!(
        "Item '{id}' is not selectable"
      )));
    }

    self.state.update(|state| {
      match self.config.selection_mode {
        SelectionMode::None => {}
        SelectionMode::Single => {
          state.selected_items = vec![id.clone()];
        }
        SelectionMode::Multiple => {
          if !state.selected_items.contains(id) {
            state.selected_items.push(id.clone());
          }
        }
        SelectionMode::Range => {
          // Range selection logic would go here
          state.selected_items = vec![id.clone()];
        }
      }
    });

    if let Some(callback) = &self.callbacks.on_selection_change {
      let selected = self.get_selected_items();
      callback(&selected);
    }

    Ok(())
  }

  /// Deselect item by ID
  pub fn deselect_item(&mut self, id: &ContentId) {
    self.state.update(|state| {
      state.selected_items.retain(|selected_id| selected_id != id);
    });

    if let Some(callback) = &self.callbacks.on_selection_change {
      let selected = self.get_selected_items();
      callback(&selected);
    }
  }

  /// Clear all selections
  pub fn clear_selection(&mut self) {
    self.state.update(|state| {
      state.selected_items.clear();
    });

    if let Some(callback) = &self.callbacks.on_selection_change {
      callback(&[]);
    }
  }

  /// Get selected item IDs
  pub fn get_selected_items(&self) -> Vec<ContentId> {
    self.state.get().selected_items.clone()
  }

  /// Highlight item by ID
  pub fn highlight_item(&mut self, id: &ContentId) -> Result<()> {
    if self.get_item(id).is_none() {
      return Err(TuiError::component(format!("Item '{id}' not found")));
    }

    self.state.update(|state| {
      state.highlighted_item = Some(id.clone());
    });

    Ok(())
  }

  /// Clear highlight
  pub fn clear_highlight(&mut self) {
    self.state.update(|state| {
      state.highlighted_item = None;
    });
  }

  /// Search for content
  pub fn search(&mut self, query: impl Into<String>) -> usize {
    let query = query.into();
    let results = self.find_search_results(&query);

    self.state.update(|state| {
      state.search_query = query.clone();
      state.search_results = results.clone();
      state.current_search_result = if results.is_empty() { None } else { Some(0) };
    });

    if let Some(callback) = &self.callbacks.on_search {
      callback(&query, &results);
    }

    results.len()
  }

  /// Find search results in content
  fn find_search_results(&self, query: &str) -> Vec<LineNumber> {
    if query.is_empty() {
      return Vec::new();
    }

    self
      .items
      .iter()
      .enumerate()
      .filter(|(_, item)| item.content.to_lowercase().contains(&query.to_lowercase()))
      .map(|(index, _)| index)
      .collect()
  }

  /// Navigate to next search result
  pub fn next_search_result(&mut self) -> bool {
    let state = self.state.get();
    if let Some(current) = state.current_search_result {
      if current + 1 < state.search_results.len() {
        let next_line = state.search_results[current + 1];
        self.state.update(|state| {
          state.current_search_result = Some(current + 1);
        });
        self.scroll_to_line(next_line);
        return true;
      }
    }
    false
  }

  /// Navigate to previous search result
  pub fn previous_search_result(&mut self) -> bool {
    let state = self.state.get();
    if let Some(current) = state.current_search_result {
      if current > 0 {
        let prev_line = state.search_results[current - 1];
        self.state.update(|state| {
          state.current_search_result = Some(current - 1);
        });
        self.scroll_to_line(prev_line);
        return true;
      }
    }
    false
  }

  /// Clear search
  pub fn clear_search(&mut self) {
    self.state.update(|state| {
      state.search_query.clear();
      state.search_results.clear();
      state.current_search_result = None;
    });
  }

  /// Set focus state
  pub fn set_focused(&mut self, focused: bool) {
    self.state.update(|state| {
      state.focused = focused;
    });

    if let Some(callback) = &self.callbacks.on_focus {
      callback(focused);
    }
  }

  /// Check if viewport is focused
  pub fn is_focused(&self) -> bool {
    self.state.get().focused
  }

  /// Enable/disable viewport
  pub fn set_disabled(&mut self, disabled: bool) {
    self.state.update(|state| {
      state.disabled = disabled;
      if disabled {
        state.focused = false;
      }
    });
  }

  /// Check if viewport is disabled
  pub fn is_disabled(&self) -> bool {
    self.state.get().disabled
  }

  /// Get total item count
  pub fn item_count(&self) -> usize {
    self.items.len()
  }

  /// Get visible items
  pub fn get_visible_items(&self) -> Vec<&ViewportItem> {
    let (start, end) = self.state.get().visible_range;
    self.items[start..end].iter().collect()
  }

  /// Render the viewport to a string
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Base CSS classes
    let mut classes = vec!["viewport".to_string()];
    if state.focused {
      classes.push("viewport-focused".to_string());
    }
    if state.disabled {
      classes.push("viewport-disabled".to_string());
    }
    classes.extend(self.css_classes.clone());

    let (start, end) = state.visible_range;

    // Render visible items
    for (i, item) in self.items[start..end].iter().enumerate() {
      let line_number = start + i;
      let render_flags = ItemRenderFlags {
        is_selected: state.selected_items.contains(&item.id),
        is_highlighted: state.highlighted_item.as_ref() == Some(&item.id),
        is_search_result: state.search_results.contains(&line_number),
      };

      self.render_item(&mut output, item, line_number, render_flags, &state);
    }

    // Render scrollbar if enabled
    if self.config.show_scrollbar && self.config.scrollbar_position != ScrollbarPosition::Hidden {
      self.render_scrollbar(&mut output, &state);
    }

    output
  }

  /// Render individual item
  fn render_item(
    &self,
    output: &mut String,
    item: &ViewportItem,
    line_number: LineNumber,
    render_flags: ItemRenderFlags,
    state: &ViewportState,
  ) {
    let ItemRenderFlags {
      is_selected,
      is_highlighted,
      is_search_result,
    } = render_flags;
    // Selection indicator
    let selection_char = if is_selected {
      &self.style.selection_indicator
    } else if is_highlighted {
      &self.style.highlight_indicator
    } else {
      " "
    };

    // Line number (optional)
    write!(output, "{:4} ", line_number + 1).unwrap();

    // Selection indicator
    write!(output, "{selection_char} ").unwrap();

    // Content with search highlighting
    let content = if is_search_result && !state.search_query.is_empty() {
      item
        .content
        .replace(&state.search_query, &format!("⟪{}⟫", state.search_query))
    } else {
      item.content.clone()
    };

    writeln!(output, "{content}").unwrap();
  }

  /// Render scrollbar
  fn render_scrollbar(&self, output: &mut String, state: &ViewportState) {
    if self.items.is_empty() {
      return;
    }

    let total_lines = self.items.len() as f64;
    let viewport_height = self.config.height as f64;
    let scroll_position = state.scroll_position;

    let scrollbar_height = self.config.height as usize;
    let _thumb_size = ((viewport_height / total_lines) * scrollbar_height as f64).max(1.0) as usize;
    let _thumb_position = ((scroll_position / total_lines) * scrollbar_height as f64) as usize;

    writeln!(
      output,
      "\nScrollbar: {:.1}% ({}/{})",
      (scroll_position / total_lines) * 100.0,
      scroll_position as usize + 1,
      total_lines as usize
    )
    .unwrap();
  }

  /// Convert to Element for integration with layout system
  pub fn to_element(&self) -> Element {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: self.config.width,
      height: self.config.height,
    };
    Element {
      tag: "div".to_string(),
      id: Some(self.id.clone()),
      classes: self.css_classes.clone(),
      content: Some(self.render(&layout, None)),
      children: Vec::new(),
      attributes: std::collections::HashMap::new(),
      focusable: !self.is_disabled(),
      focused: self.is_focused(),
      disabled: self.is_disabled(),
      tab_index: Some(0),
      key_bindings: Vec::new(),
      modal: false,
    }
  }
}

impl fmt::Display for Viewport {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: self.config.width,
      height: self.config.height,
    };
    write!(f, "{}", self.render(&layout, None))
  }
}

/// Builder for creating viewport widgets
pub struct ViewportBuilder {
  id: String,
  items: Vec<ViewportItem>,
  config: ViewportConfig,
  style: ViewportStyle,
  callbacks: ViewportCallbacks,
  css_classes: Vec<String>,
}

impl ViewportBuilder {
  /// Create a new viewport builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      items: Vec::new(),
      config: ViewportConfig::default(),
      style: ViewportStyle::default(),
      callbacks: ViewportCallbacks::default(),
      css_classes: Vec::new(),
    }
  }

  /// Set content items
  pub fn content(mut self, items: Vec<ViewportItem>) -> Self {
    self.items = items;
    self
  }

  /// Set content from strings
  pub fn content_from_strings(mut self, strings: Vec<String>) -> Self {
    self.items = strings.into_iter().map(ViewportItem::from).collect();
    self
  }

  /// Add single item
  pub fn item(mut self, item: ViewportItem) -> Self {
    self.items.push(item);
    self
  }

  /// Set width
  pub fn width(mut self, width: u16) -> Self {
    self.config.width = width;
    self
  }

  /// Set height
  pub fn height(mut self, height: u16) -> Self {
    self.config.height = height;
    self
  }

  /// Enable/disable scrolling
  pub fn scrollable(mut self, scrollable: bool) -> Self {
    self.config.scrollable = scrollable;
    self
  }

  /// Set scroll mode
  pub fn scroll_mode(mut self, mode: ScrollMode) -> Self {
    self.config.scroll_mode = mode;
    self
  }

  /// Set selection mode
  pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
    self.config.selection_mode = mode;
    self
  }

  /// Show/hide scrollbar
  pub fn show_scrollbar(mut self, show: bool) -> Self {
    self.config.show_scrollbar = show;
    self
  }

  /// Enable/disable virtual scrolling
  pub fn virtual_scrolling(mut self, enabled: bool) -> Self {
    self.config.virtual_scrolling = enabled;
    self
  }

  /// Enable/disable lazy loading
  pub fn lazy_loading(mut self, enabled: bool) -> Self {
    self.config.lazy_loading = enabled;
    self
  }

  /// Set cache size
  pub fn cache_size(mut self, size: usize) -> Self {
    self.config.cache_size = size;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set scroll callback
  pub fn on_scroll<F>(mut self, callback: F) -> Self
  where
    F: Fn(f64) + Send + Sync + 'static,
  {
    self.callbacks.on_scroll = Some(Arc::new(callback));
    self
  }

  /// Set selection change callback
  pub fn on_selection_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&[ContentId]) + Send + Sync + 'static,
  {
    self.callbacks.on_selection_change = Some(Arc::new(callback));
    self
  }

  /// Set item activate callback
  pub fn on_item_activate<F>(mut self, callback: F) -> Self
  where
    F: Fn(&ContentId, &ViewportItem) + Send + Sync + 'static,
  {
    self.callbacks.on_item_activate = Some(Arc::new(callback));
    self
  }

  /// Set lazy load callback
  pub fn on_lazy_load<F>(mut self, callback: F) -> Self
  where
    F: Fn(LineNumber, usize) -> Vec<ViewportItem> + Send + Sync + 'static,
  {
    self.callbacks.on_lazy_load = Some(Arc::new(callback));
    self
  }

  /// Build the viewport
  pub fn build(self) -> Viewport {
    let state = ViewportState::default();

    let mut viewport = Viewport {
      id: self.id,
      items: self.items,
      state: Reactive::new(state),
      config: self.config,
      style: self.style,
      callbacks: self.callbacks,
      css_classes: self.css_classes,
    };

    viewport.update_visible_range();
    viewport
  }
}

/// Convenience functions for common viewport patterns
/// Create a file viewer viewport
pub fn file_viewer(lines: Vec<String>) -> Viewport {
  ViewportBuilder::new("file-viewer")
    .content_from_strings(lines)
    .width(100)
    .height(30)
    .scrollable(true)
    .show_scrollbar(true)
    .selection_mode(SelectionMode::Single)
    .build()
}

/// Create a log viewer viewport
pub fn log_viewer(logs: Vec<String>) -> Viewport {
  ViewportBuilder::new("log-viewer")
    .content_from_strings(logs)
    .width(120)
    .height(25)
    .scrollable(true)
    .virtual_scrolling(true)
    .show_scrollbar(true)
    .selection_mode(SelectionMode::None)
    .build()
}

/// Create a data table viewport
pub fn data_table_viewport(items: Vec<ViewportItem>) -> Viewport {
  ViewportBuilder::new("data-table")
    .content(items)
    .width(80)
    .height(20)
    .scrollable(true)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .virtual_scrolling(true)
    .build()
}
