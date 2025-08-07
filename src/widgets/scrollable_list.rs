//! Scrollable List Widget
//!
//! A comprehensive vertical scrolling list widget with item selection, keyboard navigation,
//! and efficient rendering for large datasets with smooth scrolling.

use crate::{
  components::element::Element,
  layout::LayoutRect,
  reactive::Reactive,
  themes::{ColorDefinition, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;
use std::sync::Arc;

// Type aliases for complex function pointer types
type OnSelectionChangeCallback = Arc<dyn Fn(&[ItemId]) + Send + Sync>;
type OnItemActivateCallback = Arc<dyn Fn(&ItemId, &ListItem) + Send + Sync>;
type OnHighlightChangeCallback = Arc<dyn Fn(Option<&ItemId>) + Send + Sync>;
type OnScrollCallback = Arc<dyn Fn(usize, usize) + Send + Sync>;
type OnSearchChangeCallback = Arc<dyn Fn(&str, usize) + Send + Sync>;
type OnFocusChangeCallback = Arc<dyn Fn(bool) + Send + Sync>;

/// Unique identifier for list items
pub type ItemId = String;

/// Individual list item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListItem {
  /// Unique item identifier
  pub id: ItemId,
  /// Primary text to display
  pub text: String,
  /// Optional subtitle/description
  pub subtitle: Option<String>,
  /// Optional icon character
  pub icon: Option<String>,
  /// Additional metadata
  pub metadata: HashMap<String, String>,
  /// Whether item is disabled
  pub disabled: bool,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
}

impl ListItem {
  /// Create a new list item
  pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      text: text.into(),
      subtitle: None,
      icon: None,
      metadata: HashMap::new(),
      disabled: false,
      css_classes: Vec::new(),
    }
  }

  /// Set item subtitle
  pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self {
    self.subtitle = Some(subtitle.into());
    self
  }

  /// Set item icon
  pub fn icon(mut self, icon: impl Into<String>) -> Self {
    self.icon = Some(icon.into());
    self
  }

  /// Add metadata
  pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.metadata.insert(key.into(), value.into());
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
}

/// Selection modes for the list
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
  /// Single item selection
  Single,
  /// Multiple item selection
  Multiple,
  /// No selection allowed
  None,
}

impl Default for SelectionMode {
  fn default() -> Self {
    Self::Single
  }
}

/// Current state of the scrollable list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct ScrollableListState {
  /// Current scroll position (top item index)
  pub scroll_position: usize,
  /// Currently highlighted item index
  pub highlighted_index: Option<usize>,
  /// Set of selected item IDs
  pub selected_items: Vec<ItemId>,
  /// Whether the list is focused
  pub is_focused: bool,
  /// Current search query
  pub search_query: String,
  /// Filtered item indices
  pub filtered_indices: Vec<usize>,
  /// Total number of items
  pub total_items: usize,
  /// Number of visible items in viewport
  pub visible_items: usize,
  /// Whether search is active
  pub search_active: bool,
}

/// Configuration for scrollable list behavior
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollableListConfig {
  /// List height in lines
  pub height: usize,
  /// Selection mode
  pub selection_mode: SelectionMode,
  /// Show scrollbar
  pub show_scrollbar: bool,
  /// Show item icons
  pub show_icons: bool,
  /// Show item subtitles
  pub show_subtitles: bool,
  /// Enable search functionality
  pub search_enabled: bool,
  /// Enable vim-style navigation
  pub vim_navigation: bool,
  /// Auto-scroll to highlighted item
  pub auto_scroll: bool,
  /// Smooth scrolling animation
  pub smooth_scrolling: bool,
  /// Number of lines to scroll at once
  pub scroll_step: usize,
  /// Height of each item in lines
  pub item_height: usize,
  /// Internal padding
  pub padding: usize,
  /// Border width
  pub border_width: usize,
}

impl Default for ScrollableListConfig {
  fn default() -> Self {
    Self {
      height: 10,
      selection_mode: SelectionMode::Single,
      show_scrollbar: true,
      show_icons: true,
      show_subtitles: true,
      search_enabled: true,
      vim_navigation: true,
      auto_scroll: true,
      smooth_scrolling: true,
      scroll_step: 3,
      item_height: 1,
      padding: 1,
      border_width: 1,
    }
  }
}

/// Visual styling for scrollable list
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ScrollableListStyle {
  /// Background color
  pub background_color: ColorDefinition,
  /// Text color
  pub text_color: ColorDefinition,
  /// Selected item background
  pub selected_background: ColorDefinition,
  /// Selected item foreground
  pub selected_foreground: ColorDefinition,
  /// Highlighted item background
  pub highlighted_background: ColorDefinition,
  /// Highlighted item foreground
  pub highlighted_foreground: ColorDefinition,
  /// Disabled item color
  pub disabled_color: ColorDefinition,
  /// Scrollbar color
  pub scrollbar_color: ColorDefinition,
  /// Scrollbar thumb color
  pub scrollbar_thumb_color: ColorDefinition,
  /// Border color
  pub border_color: ColorDefinition,
  /// Search highlight color
  pub search_highlight_color: ColorDefinition,
  /// Icon color
  pub icon_color: ColorDefinition,
  /// Subtitle color
  pub subtitle_color: ColorDefinition,
  /// CSS utility classes
  pub css_classes: Vec<String>,
}

impl Default for ScrollableListStyle {
  fn default() -> Self {
    Self {
      background_color: ColorDefinition { r: 0, g: 0, b: 0 },
      text_color: ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      },
      selected_background: ColorDefinition {
        r: 0,
        g: 123,
        b: 255,
      },
      selected_foreground: ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      },
      highlighted_background: ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      },
      highlighted_foreground: ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      },
      disabled_color: ColorDefinition {
        r: 64,
        g: 64,
        b: 64,
      },
      scrollbar_color: ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      },
      scrollbar_thumb_color: ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      },
      border_color: ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      },
      search_highlight_color: ColorDefinition {
        r: 255,
        g: 255,
        b: 0,
      },
      icon_color: ColorDefinition {
        r: 0,
        g: 255,
        b: 255,
      },
      subtitle_color: ColorDefinition {
        r: 192,
        g: 192,
        b: 192,
      },
      css_classes: Vec::new(),
    }
  }
}

/// Event callbacks for scrollable list interactions
#[derive(Default)]
pub struct ScrollableListCallbacks {
  /// Called when selection changes
  pub on_selection_change: Option<OnSelectionChangeCallback>,
  /// Called when an item is activated (Enter/double-click)
  pub on_item_activate: Option<OnItemActivateCallback>,
  /// Called when highlight changes
  pub on_highlight_change: Option<OnHighlightChangeCallback>,
  /// Called when scrolling occurs
  pub on_scroll: Option<OnScrollCallback>,
  /// Called when search query changes
  pub on_search_change: Option<OnSearchChangeCallback>,
  /// Called when focus changes
  pub on_focus_change: Option<OnFocusChangeCallback>,
}

/// Main ScrollableList widget
pub struct ScrollableList {
  /// Unique list identifier
  pub id: String,
  /// List items
  pub items: Vec<ListItem>,
  /// Reactive state management
  pub state: Reactive<ScrollableListState>,
  /// Configuration options
  pub config: ScrollableListConfig,
  /// Visual styling
  pub style: ScrollableListStyle,
  /// Event callbacks
  pub callbacks: ScrollableListCallbacks,
  /// Visible item cache for performance
  pub visible_cache: Vec<usize>,
}

impl ScrollableList {
  /// Create a new scrollable list builder
  pub fn builder<S: Into<String>>(id: S) -> ScrollableListBuilder {
    ScrollableListBuilder::new(id)
  }

  /// Set items in the list
  pub fn set_items(&mut self, items: Vec<ListItem>) {
    self.items = items;
    self.state.update(|state| {
      state.total_items = self.items.len();
      state.filtered_indices = (0..self.items.len()).collect();
      state.visible_items = self.config.height.min(self.items.len());
      state.highlighted_index = None;
      state.selected_items.clear();
    });
    self.update_visible_cache();
  }

  /// Add an item to the list
  pub fn add_item(&mut self, item: ListItem) {
    self.items.push(item);
    self.refresh_state();
  }

  /// Remove an item from the list
  pub fn remove_item(&mut self, item_id: &str) -> bool {
    if let Some(index) = self.items.iter().position(|item| item.id == item_id) {
      self.items.remove(index);
      self.refresh_state();
      true
    } else {
      false
    }
  }

  /// Get item by ID
  pub fn get_item(&self, item_id: &str) -> Option<&ListItem> {
    self.items.iter().find(|item| item.id == item_id)
  }

  /// Select an item
  pub fn select_item(&mut self, item_id: &str) -> bool {
    if let Some(index) = self.items.iter().position(|item| item.id == item_id) {
      let item = &self.items[index];
      if item.disabled {
        return false;
      }

      self.state.update(|state| match self.config.selection_mode {
        SelectionMode::Single => {
          state.selected_items = vec![item_id.to_string()];
          state.highlighted_index = Some(index);
        }
        SelectionMode::Multiple => {
          if !state.selected_items.contains(&item_id.to_string()) {
            state.selected_items.push(item_id.to_string());
          }
          state.highlighted_index = Some(index);
        }
        SelectionMode::None => {
          state.highlighted_index = Some(index);
        }
      });

      if let Some(callback) = &self.callbacks.on_selection_change {
        callback(&self.state.get().selected_items);
      }
      true
    } else {
      false
    }
  }

  /// Navigate to next item
  pub fn select_next(&mut self) -> bool {
    let state = self.state.get();
    let filtered_count = state.filtered_indices.len();

    if filtered_count == 0 {
      return false;
    }

    let new_index = if let Some(current) = state.highlighted_index {
      let current_filtered = state.filtered_indices.iter().position(|&i| i == current);
      if let Some(pos) = current_filtered {
        state.filtered_indices[(pos + 1) % filtered_count]
      } else {
        state.filtered_indices[0]
      }
    } else {
      state.filtered_indices[0]
    };

    self.highlight_item(new_index)
  }

  /// Navigate to previous item
  pub fn select_previous(&mut self) -> bool {
    let state = self.state.get();
    let filtered_count = state.filtered_indices.len();

    if filtered_count == 0 {
      return false;
    }

    let new_index = if let Some(current) = state.highlighted_index {
      let current_filtered = state.filtered_indices.iter().position(|&i| i == current);
      if let Some(pos) = current_filtered {
        if pos == 0 {
          state.filtered_indices[filtered_count - 1]
        } else {
          state.filtered_indices[pos - 1]
        }
      } else {
        state.filtered_indices[filtered_count - 1]
      }
    } else {
      state.filtered_indices[filtered_count - 1]
    };

    self.highlight_item(new_index)
  }

  /// Highlight an item by index
  fn highlight_item(&mut self, index: usize) -> bool {
    if index >= self.items.len() {
      return false;
    }

    let old_index = self.state.get().highlighted_index;
    self.state.update(|state| {
      state.highlighted_index = Some(index);
    });

    if self.config.auto_scroll {
      self.scroll_to_item(index);
    }

    if let Some(callback) = &self.callbacks.on_highlight_change {
      let item_id = &self.items[index].id;
      callback(Some(item_id));
    }

    old_index != Some(index)
  }

  /// Scroll to make item visible
  pub fn scroll_to_item(&mut self, item_index: usize) {
    let state = self.state.get();
    let filtered_index = state.filtered_indices.iter().position(|&i| i == item_index);

    if let Some(filtered_pos) = filtered_index {
      let viewport_start = state.scroll_position;
      let viewport_end = viewport_start + self.config.height - 1;

      if filtered_pos < viewport_start {
        self.set_scroll_position(filtered_pos);
      } else if filtered_pos > viewport_end {
        self.set_scroll_position(filtered_pos - self.config.height + 1);
      }
    }
  }

  /// Set scroll position
  fn set_scroll_position(&mut self, position: usize) {
    let state = self.state.get();
    let max_scroll = state
      .filtered_indices
      .len()
      .saturating_sub(self.config.height);
    let new_pos = position.min(max_scroll);

    if new_pos != state.scroll_position {
      self.state.update(|state| {
        state.scroll_position = new_pos;
      });
      self.update_visible_cache();

      if let Some(callback) = &self.callbacks.on_scroll {
        callback(new_pos, max_scroll);
      }
    }
  }

  /// Set search query
  pub fn set_search_query(&mut self, query: &str) {
    self.state.update(|state| {
      state.search_query = query.to_string();
      state.search_active = !query.is_empty();
    });
    self.filter_items();

    let results = self.state.get().filtered_indices.len();
    if let Some(callback) = &self.callbacks.on_search_change {
      callback(query, results);
    }
  }

  /// Filter items based on search query
  fn filter_items(&mut self) {
    let state = self.state.get();
    let filtered_indices = if state.search_active {
      let query = state.search_query.to_lowercase();
      self
        .items
        .iter()
        .enumerate()
        .filter(|(_, item)| {
          item.text.to_lowercase().contains(&query)
            || item
              .subtitle
              .as_ref()
              .is_some_and(|s| s.to_lowercase().contains(&query))
        })
        .map(|(i, _)| i)
        .collect()
    } else {
      (0..self.items.len()).collect()
    };

    self.state.update(|state| {
      state.filtered_indices = filtered_indices;
      let max_scroll = state
        .filtered_indices
        .len()
        .saturating_sub(self.config.height);
      if state.scroll_position > max_scroll {
        state.scroll_position = max_scroll;
      }
    });

    self.update_visible_cache();
  }

  /// Update visible cache for rendering
  fn update_visible_cache(&mut self) {
    let state = self.state.get();
    let start = state.scroll_position;
    let end = (start + self.config.height).min(state.filtered_indices.len());
    self.visible_cache = state.filtered_indices[start..end].to_vec();
  }

  /// Refresh internal state after items change
  fn refresh_state(&mut self) {
    self.state.update(|state| {
      state.total_items = self.items.len();
    });
    self.filter_items();
  }

  /// Render the scrollable list
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Render border top
    if self.config.border_width > 0 {
      let _ = writeln!(output, "┌{}┐", "─".repeat(78));
    }

    // Render visible items
    for i in 0..self.config.height {
      let mut line = String::new();

      if self.config.border_width > 0 {
        line.push_str("│ ");
      }

      if i < self.visible_cache.len() {
        let item_index = self.visible_cache[i];
        let item = &self.items[item_index];
        line.push_str(&self.render_item(item, item_index, &state));
      } else {
        line.push_str(&" ".repeat(76));
      }

      if self.config.border_width > 0 {
        line.push_str(" │");
      }

      let _ = writeln!(output, "{line}");
    }

    // Render border bottom
    if self.config.border_width > 0 {
      let _ = writeln!(output, "└{}┘", "─".repeat(78));
    }

    output
  }

  /// Render individual item
  fn render_item(&self, item: &ListItem, item_index: usize, state: &ScrollableListState) -> String {
    let mut line = String::new();
    let is_selected = state.selected_items.contains(&item.id);
    let is_highlighted = state.highlighted_index == Some(item_index);

    // Selection indicator
    if self.config.selection_mode != SelectionMode::None {
      if is_selected {
        line.push_str("✓ ");
      } else if is_highlighted {
        line.push_str("► ");
      } else {
        line.push_str("  ");
      }
    }

    // Icon
    if self.config.show_icons {
      if let Some(icon) = &item.icon {
        line.push_str(icon);
        line.push(' ');
      }
    }

    // Main text
    let mut text = item.text.clone();
    if state.search_active && !state.search_query.is_empty() {
      text = self.highlight_search_term(&text, &state.search_query);
    }

    if item.disabled {
      text = format!("{text})");
    }

    line.push_str(&text);

    // Subtitle
    if self.config.show_subtitles {
      if let Some(subtitle) = &item.subtitle {
        line.push_str(&subtitle.to_string());
      }
    }

    // Truncate if too long
    let max_width = 74;
    if line.len() > max_width {
      line.truncate(max_width - 3);
      line.push_str("...");
    }

    // Pad to full width
    while line.len() < max_width {
      line.push(' ');
    }

    line
  }

  /// Highlight search term in text
  fn highlight_search_term(&self, text: &str, query: &str) -> String {
    let lower_text = text.to_lowercase();
    let lower_query = query.to_lowercase();

    if let Some(pos) = lower_text.find(&lower_query) {
      let mut result = text.to_string();
      result.replace_range(
        pos..pos + query.len(),
        &format!("{}]", &text[pos..pos + query.len()]),
      );
      result
    } else {
      text.to_string()
    }
  }

  /// Handle key press events
  pub fn handle_key_press(&mut self, key: &str) -> bool {
    match key {
      "ArrowUp" | "k" => self.select_previous(),
      "ArrowDown" | "j" => self.select_next(),
      "Enter" | " " => self.activate_highlighted(),
      "Escape" => {
        if self.state.get().search_active {
          self.set_search_query("");
          true
        } else {
          false
        }
      }
      _ => false,
    }
  }

  /// Activate the highlighted item
  fn activate_highlighted(&mut self) -> bool {
    if let Some(index) = self.state.get().highlighted_index {
      let item_id = self.items[index].id.clone();
      let item_disabled = self.items[index].disabled;

      if !item_disabled {
        if self.config.selection_mode != SelectionMode::None {
          self.select_item(&item_id);
        }

        if let Some(callback) = &self.callbacks.on_item_activate {
          let item = &self.items[index];
          callback(&item_id, item);
        }
        return true;
      }
    }
    false
  }

  /// Convert to Element for layout integration
  pub fn to_element(&self) -> Element {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 80,
      height: 25,
    };
    Element {
      tag: "div".to_string(),
      id: Some(self.id.clone()),
      classes: self.style.css_classes.clone(),
      content: Some(self.render(&layout, None)),
      children: Vec::new(),
      attributes: HashMap::new(),
      focusable: true,
      focused: self.state.get().is_focused,
      disabled: false,
      tab_index: Some(0),
      key_bindings: Vec::new(),
      modal: false,
    }
  }
}

/// Builder for creating scrollable lists
pub struct ScrollableListBuilder {
  id: String,
  items: Vec<ListItem>,
  config: ScrollableListConfig,
  style: ScrollableListStyle,
  callbacks: ScrollableListCallbacks,
}

impl ScrollableListBuilder {
  /// Create a new scrollable list builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      items: Vec::new(),
      config: ScrollableListConfig::default(),
      style: ScrollableListStyle::default(),
      callbacks: ScrollableListCallbacks::default(),
    }
  }

  /// Set items
  pub fn items(mut self, items: Vec<ListItem>) -> Self {
    self.items = items;
    self
  }

  /// Add a single item
  pub fn item(mut self, item: ListItem) -> Self {
    self.items.push(item);
    self
  }

  /// Set height
  pub fn height(mut self, height: usize) -> Self {
    self.config.height = height;
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

  /// Show/hide icons
  pub fn show_icons(mut self, show: bool) -> Self {
    self.config.show_icons = show;
    self
  }

  /// Show/hide subtitles
  pub fn show_subtitles(mut self, show: bool) -> Self {
    self.config.show_subtitles = show;
    self
  }

  /// Enable/disable search
  pub fn search_enabled(mut self, enabled: bool) -> Self {
    self.config.search_enabled = enabled;
    self
  }

  /// Set scroll step
  pub fn scroll_step(mut self, step: usize) -> Self {
    self.config.scroll_step = step;
    self
  }

  /// Set selection change callback
  pub fn on_selection_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&[ItemId]) + Send + Sync + 'static,
  {
    self.callbacks.on_selection_change = Some(Arc::new(callback));
    self
  }

  /// Set item activate callback
  pub fn on_item_activate<F>(mut self, callback: F) -> Self
  where
    F: Fn(&ItemId, &ListItem) + Send + Sync + 'static,
  {
    self.callbacks.on_item_activate = Some(Arc::new(callback));
    self
  }

  /// Build the scrollable list
  pub fn build(self) -> ScrollableList {
    let state = ScrollableListState {
      total_items: self.items.len(),
      filtered_indices: (0..self.items.len()).collect(),
      visible_items: self.config.height.min(self.items.len()),
      ..Default::default()
    };

    let mut list = ScrollableList {
      id: self.id,
      items: self.items,
      state: Reactive::new(state),
      config: self.config,
      style: self.style,
      callbacks: self.callbacks,
      visible_cache: Vec::new(),
    };

    list.update_visible_cache();
    list
  }
}

/// Convenience functions for common scrollable list patterns
/// Create a file browser list
pub fn file_browser_list(files: Vec<(&str, &str, Option<&str>)>) -> ScrollableList {
  let items: Vec<ListItem> = files
    .into_iter()
    .enumerate()
    .map(|(i, (name, type_str, size))| {
      ListItem::new(i.to_string(), name)
        .subtitle(size.unwrap_or(type_str).to_string())
        .icon(
          if type_str == "directory" {
            "📁"
          } else {
            "📄"
          }
          .to_string(),
        )
        .metadata("type", type_str)
    })
    .collect();

  ScrollableListBuilder::new("file-browser")
    .items(items)
    .height(15)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(true)
    .build()
}

/// Create a menu list
pub fn menu_list(menu_items: Vec<(&str, &str, Option<&str>)>) -> ScrollableList {
  let items: Vec<ListItem> = menu_items
    .into_iter()
    .map(|(action, label, shortcut)| {
      ListItem::new(action, label)
        .subtitle(shortcut.unwrap_or("").to_string())
        .metadata("action", action)
    })
    .collect();

  let height = items.len().min(10);

  ScrollableListBuilder::new("menu-list")
    .items(items)
    .height(height)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(false)
    .show_icons(false)
    .show_subtitles(true)
    .search_enabled(false)
    .build()
}

/// Create a task list
pub fn task_list(tasks: Vec<(&str, &str, &str)>) -> ScrollableList {
  let items: Vec<ListItem> = tasks
    .into_iter()
    .map(|(id, title, status)| {
      let icon = match status {
        "completed" => "✅",
        "failed" => "❌",
        _ => "⏳",
      };

      ListItem::new(id, title)
        .subtitle(status.to_string())
        .icon(icon.to_string())
        .metadata("status", status)
    })
    .collect();

  ScrollableListBuilder::new("task-list")
    .items(items)
    .height(12)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(true)
    .build()
}
