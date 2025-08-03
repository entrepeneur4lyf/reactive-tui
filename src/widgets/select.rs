//! Select Dropdown Widget
//!
//! A comprehensive dropdown selection widget supporting single/multi-select modes,
//! search filtering, keyboard navigation, and customizable rendering.
//!
//! # Features
//!
//! - **Selection Modes**: Single-select and multi-select with different behaviors
//! - **Search Filtering**: Real-time option filtering as user types
//! - **Keyboard Navigation**: Arrow keys, Enter/Space selection, Escape to close
//! - **Custom Rendering**: Flexible option display with icons, descriptions, grouping
//! - **Overlay Positioning**: Smart dropdown positioning with viewport awareness
//! - **Accessibility**: Full ARIA support, screen reader compatibility
//! - **Reactive State**: Integration with reactive state management system
//!
//! # Basic Usage
//!
//! ```rust
//! use reactive_tui::widgets::{Select, SelectBuilder};
//!
//! // Simple single-select dropdown
//! let select = SelectBuilder::new("language-select")
//!     .options(vec!["Rust", "TypeScript", "Python", "Go"])
//!     .selected(Some(0))
//!     .placeholder("Choose a language...")
//!     .build();
//!
//! // Multi-select with search
//! let multi_select = SelectBuilder::new("tags-select")
//!     .options(vec!["Frontend", "Backend", "Database", "Mobile"])
//!     .multi_select(true)
//!     .searchable(true)
//!     .selected_indices(vec![0, 2])
//!     .build();
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! use reactive_tui::widgets::{SelectBuilder, SelectOption};
//!
//! // Custom options with icons and descriptions
//! let options = vec![
//!     SelectOption::new("rust", "Rust")
//!         .icon("🦀")
//!         .description("Systems programming language"),
//!     SelectOption::new("typescript", "TypeScript")
//!         .icon("📘")
//!         .description("Typed JavaScript"),
//!     SelectOption::new("python", "Python")
//!         .icon("🐍")
//!         .description("General-purpose programming"),
//! ];
//!
//! let select = SelectBuilder::new("advanced-select")
//!     .custom_options(options)
//!     .searchable(true)
//!     .max_height(10)
//!     .on_change(|selected| {
//!         println!("Selected: {:?}", selected);
//!     })
//!     .build();
//! ```

use crate::{
  components::Element,
  error::{Result, TuiError},
  reactive::Reactive,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt, sync::Arc};

// Type aliases for complex function pointer types
type OnChangeCallback = Arc<dyn Fn(&[usize]) + Send + Sync>;
type FilterCallback = Arc<dyn Fn(&SelectOption, &str) -> bool + Send + Sync>;

/// Selection mode for the dropdown
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectMode {
  /// Single selection - only one option can be selected
  Single,
  /// Multiple selection - multiple options can be selected
  Multiple,
}

/// Position preference for dropdown overlay
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DropdownPosition {
  /// Prefer to show below the select input
  Below,
  /// Prefer to show above the select input
  Above,
  /// Automatically choose based on available space
  Auto,
}

/// Represents a single option in the select dropdown
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectOption {
  /// Unique identifier for the option
  pub id: String,
  /// Display text for the option
  pub label: String,
  /// Optional icon to display with the option
  pub icon: Option<String>,
  /// Optional description text
  pub description: Option<String>,
  /// Optional group this option belongs to
  pub group: Option<String>,
  /// Whether this option is disabled
  pub disabled: bool,
  /// Custom data associated with this option
  pub data: HashMap<String, String>,
}

impl SelectOption {
  /// Create a new select option
  pub fn new<S: Into<String>>(id: S, label: S) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      icon: None,
      description: None,
      group: None,
      disabled: false,
      data: HashMap::new(),
    }
  }

  /// Set the icon for this option
  pub fn icon<S: Into<String>>(mut self, icon: S) -> Self {
    self.icon = Some(icon.into());
    self
  }

  /// Set the description for this option
  pub fn description<S: Into<String>>(mut self, description: S) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Set the group for this option
  pub fn group<S: Into<String>>(mut self, group: S) -> Self {
    self.group = Some(group.into());
    self
  }

  /// Mark this option as disabled
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Add custom data to this option
  pub fn data<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
    self.data.insert(key.into(), value.into());
    self
  }
}

/// Current state of the select dropdown
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct SelectState {
  /// Whether the dropdown is currently open
  pub open: bool,
  /// Index of the currently highlighted option (for keyboard navigation)
  pub highlighted_index: Option<usize>,
  /// Indices of selected options
  pub selected_indices: Vec<usize>,
  /// Current search query (if searchable)
  pub search_query: String,
  /// Filtered options based on search query
  pub filtered_indices: Vec<usize>,
  /// Whether the select has focus
  pub focused: bool,
  /// Current scroll position in dropdown
  pub scroll_offset: usize,
}

/// Styling configuration for the select dropdown
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SelectStyle {
  /// CSS classes for the main select container
  pub container_classes: Vec<String>,
  /// CSS classes for the select button/trigger
  pub trigger_classes: Vec<String>,
  /// CSS classes for the dropdown overlay
  pub dropdown_classes: Vec<String>,
  /// CSS classes for individual options
  pub option_classes: Vec<String>,
  /// CSS classes for selected options
  pub selected_option_classes: Vec<String>,
  /// CSS classes for highlighted option (keyboard navigation)
  pub highlighted_option_classes: Vec<String>,
  /// CSS classes for disabled options
  pub disabled_option_classes: Vec<String>,
  /// CSS classes for the search input (if searchable)
  pub search_input_classes: Vec<String>,
  /// CSS classes for option groups
  pub group_classes: Vec<String>,
  /// Character to use for dropdown arrow (default: "▼")
  pub dropdown_arrow: String,
  /// Character to use for selected items in multi-select (default: "✓")
  pub selected_marker: String,
  /// Maximum height of dropdown in rows
  pub max_height: usize,
  /// Whether to show icons in options
  pub show_icons: bool,
  /// Whether to show descriptions in options
  pub show_descriptions: bool,
}

impl Default for SelectStyle {
  fn default() -> Self {
    Self {
      container_classes: vec!["select".to_string()],
      trigger_classes: vec!["select-trigger".to_string()],
      dropdown_classes: vec!["select-dropdown".to_string()],
      option_classes: vec!["select-option".to_string()],
      selected_option_classes: vec!["select-option-selected".to_string()],
      highlighted_option_classes: vec!["select-option-highlighted".to_string()],
      disabled_option_classes: vec!["select-option-disabled".to_string()],
      search_input_classes: vec!["select-search".to_string()],
      group_classes: vec!["select-group".to_string()],
      dropdown_arrow: "▼".to_string(),
      selected_marker: "✓".to_string(),
      max_height: 8,
      show_icons: true,
      show_descriptions: true,
    }
  }
}

/// Main select dropdown widget
#[derive(Clone)]
pub struct Select {
  /// Unique identifier for the select
  pub id: String,
  /// Available options
  pub options: Vec<SelectOption>,
  /// Current state
  pub state: Reactive<SelectState>,
  /// Selection mode (single or multiple)
  pub mode: SelectMode,
  /// Whether search filtering is enabled
  pub searchable: bool,
  /// Placeholder text when no selection is made
  pub placeholder: String,
  /// Dropdown position preference
  pub position: DropdownPosition,
  /// Styling configuration
  pub style: SelectStyle,
  /// Callback for selection changes
  pub on_change: Option<OnChangeCallback>,
  /// Callback for open/close state changes
  pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
  /// Whether the select is disabled
  pub disabled: bool,
  /// Whether the select is required (for form validation)
  pub required: bool,
  /// Custom filter function for search
  pub filter_fn: Option<FilterCallback>,
}

impl Select {
  /// Create a new select widget builder
  pub fn builder<S: Into<String>>(id: S) -> SelectBuilder {
    SelectBuilder::new(id)
  }

  /// Get the currently selected options
  pub fn selected_options(&self) -> Vec<&SelectOption> {
    let state = self.state.get();
    state
      .selected_indices
      .iter()
      .filter_map(|&index| self.options.get(index))
      .collect()
  }

  /// Get the currently selected option IDs
  pub fn selected_ids(&self) -> Vec<String> {
    self
      .selected_options()
      .iter()
      .map(|option| option.id.clone())
      .collect()
  }

  /// Check if an option is selected
  pub fn is_selected(&self, index: usize) -> bool {
    let state = self.state.get();
    state.selected_indices.contains(&index)
  }

  /// Select an option by index
  pub fn select(&mut self, index: usize) -> Result<()> {
    if index >= self.options.len() {
      return Err(TuiError::component(format!("{index} out of bounds")));
    }

    if self.options[index].disabled {
      return Ok(()); // Ignore selection of disabled options
    }

    let callback = self.on_change.clone();
    let mode = self.mode.clone();

    self.state.update(|state| {
      match mode {
        SelectMode::Single => {
          state.selected_indices = vec![index];
          state.open = false; // Close dropdown after single selection
        }
        SelectMode::Multiple => {
          if !state.selected_indices.contains(&index) {
            state.selected_indices.push(index);
            state.selected_indices.sort_unstable();
          }
        }
      }
    });

    if let Some(callback) = &callback {
      let state = self.state.get();
      callback(&state.selected_indices);
    }

    Ok(())
  }

  /// Deselect an option by index
  pub fn deselect(&mut self, index: usize) -> Result<()> {
    self.state.update(|state| {
      state.selected_indices.retain(|&i| i != index);
    });

    if let Some(callback) = &self.on_change {
      let state = self.state.get();
      callback(&state.selected_indices);
    }

    Ok(())
  }

  /// Toggle selection of an option
  pub fn toggle_selection(&mut self, index: usize) -> Result<()> {
    if self.is_selected(index) {
      self.deselect(index)
    } else {
      self.select(index)
    }
  }

  /// Clear all selections
  pub fn clear_selection(&mut self) {
    self.state.update(|state| {
      state.selected_indices.clear();
    });

    if let Some(callback) = &self.on_change {
      let state = self.state.get();
      callback(&state.selected_indices);
    }
  }

  /// Open the dropdown
  pub fn open(&mut self) {
    if self.disabled {
      return;
    }

    let state = self.state.get();
    let was_open = state.open;
    drop(state);

    if !was_open {
      self.state.update(|state| {
        state.open = true;
      });

      self.update_filtered_options();

      // Set initial highlighted index
      let state = self.state.get();
      if state.highlighted_index.is_none() && !state.filtered_indices.is_empty() {
        drop(state);
        self.state.update(|state| {
          state.highlighted_index = Some(0);
        });
      }

      if let Some(callback) = &self.on_toggle {
        callback(true);
      }
    }
  }

  /// Close the dropdown
  pub fn close(&mut self) {
    let state = self.state.get();
    let was_open = state.open;
    drop(state);

    if was_open {
      self.state.update(|state| {
        state.open = false;
        state.search_query.clear();
        state.highlighted_index = None;
        state.scroll_offset = 0;
      });
      self.update_filtered_options();

      if let Some(callback) = &self.on_toggle {
        callback(false);
      }
    }
  }

  /// Toggle the dropdown open/closed state
  pub fn toggle(&mut self) {
    let state = self.state.get();
    if state.open {
      drop(state);
      self.close();
    } else {
      drop(state);
      self.open();
    }
  }

  /// Set the search query and update filtered options
  pub fn set_search_query<S: Into<String>>(&mut self, query: S) {
    if !self.searchable {
      return;
    }

    let query = query.into();
    self.state.update(|state| {
      state.search_query = query;
      state.highlighted_index = None;
      state.scroll_offset = 0;
    });

    self.update_filtered_options();
  }

  /// Update the filtered options based on current search query
  fn update_filtered_options(&mut self) {
    let state = self.state.get();
    let search_query = state.search_query.clone();
    let current_highlighted = state.highlighted_index;
    drop(state);

    let filtered_indices = if search_query.is_empty() {
      (0..self.options.len()).collect()
    } else {
      self
        .options
        .iter()
        .enumerate()
        .filter(|(_, option)| {
          if let Some(filter_fn) = &self.filter_fn {
            filter_fn(option, &search_query)
          } else {
            // Default search: case-insensitive match in label or description
            let query_lower = search_query.to_lowercase();
            option.label.to_lowercase().contains(&query_lower)
              || option
                .description
                .as_ref()
                .map(|desc| desc.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
          }
        })
        .map(|(index, _)| index)
        .collect()
    };

    self.state.update(|state| {
      state.filtered_indices = filtered_indices;

      // Reset highlighted index if current selection is no longer visible
      if let Some(highlighted) = current_highlighted {
        if !state.filtered_indices.contains(&highlighted) {
          state.highlighted_index = if !state.filtered_indices.is_empty() {
            Some(state.filtered_indices[0])
          } else {
            None
          };
        }
      }
    });
  }

  /// Navigate to the next option (keyboard navigation)
  pub fn navigate_next(&mut self) {
    let max_height = self.style.max_height;
    self.state.update(|state| {
      if state.filtered_indices.is_empty() {
        return;
      }

      match state.highlighted_index {
        None => {
          state.highlighted_index = Some(state.filtered_indices[0]);
        }
        Some(current) => {
          if let Some(current_pos) = state.filtered_indices.iter().position(|&i| i == current) {
            let next_pos = (current_pos + 1) % state.filtered_indices.len();
            state.highlighted_index = Some(state.filtered_indices[next_pos]);

            // Update scroll offset if needed
            if next_pos >= state.scroll_offset + max_height {
              state.scroll_offset = next_pos.saturating_sub(max_height - 1);
            }
          }
        }
      }
    });
  }

  /// Navigate to the previous option (keyboard navigation)
  pub fn navigate_previous(&mut self) {
    self.state.update(|state| {
      if state.filtered_indices.is_empty() {
        return;
      }

      match state.highlighted_index {
        None => {
          state.highlighted_index = Some(state.filtered_indices[state.filtered_indices.len() - 1]);
        }
        Some(current) => {
          if let Some(current_pos) = state.filtered_indices.iter().position(|&i| i == current) {
            let prev_pos = if current_pos == 0 {
              state.filtered_indices.len() - 1
            } else {
              current_pos - 1
            };
            state.highlighted_index = Some(state.filtered_indices[prev_pos]);

            // Update scroll offset if needed
            if prev_pos < state.scroll_offset {
              state.scroll_offset = prev_pos;
            }
          }
        }
      }
    });
  }

  /// Select the currently highlighted option
  pub fn select_highlighted(&mut self) -> Result<()> {
    let state = self.state.get();
    if let Some(highlighted) = state.highlighted_index {
      drop(state);
      self.toggle_selection(highlighted)
    } else {
      Ok(())
    }
  }

  /// Convert to Element for rendering
  pub fn to_element(&self) -> Element {
    let state = self.state.get();
    let mut container = Element::with_tag("div".to_string())
      .id(self.id.clone())
      .classes(self.style.container_classes.clone())
      .focusable(true);

    // Add state-based classes
    if self.disabled {
      container = container.class("select-disabled");
    }

    if state.focused {
      container = container.class("select-focused");
    }

    if state.open {
      container = container.class("select-open");
    }

    // Create trigger button
    let trigger_text = if state.selected_indices.is_empty() {
      self.placeholder.clone()
    } else {
      match self.mode {
        SelectMode::Single => {
          if let Some(&index) = state.selected_indices.first() {
            self.options[index].label.clone()
          } else {
            self.placeholder.clone()
          }
        }
        SelectMode::Multiple => {
          format!("{} selected", state.selected_indices.len())
        }
      }
    };

    let trigger = Element::with_tag("button")
      .classes(self.style.trigger_classes.clone())
      .content(format!("{}{}", trigger_text, self.style.dropdown_arrow));

    container = container.child(trigger.build());

    // Add dropdown if open
    if state.open {
      container = container.child(self.create_dropdown_element(&state));
    }

    container.build()
  }

  /// Create the dropdown overlay element
  fn create_dropdown_element(&self, state: &SelectState) -> Element {
    let mut dropdown = Element::with_tag("div").classes(self.style.dropdown_classes.clone());

    // Add search input if searchable
    if self.searchable {
      let search_input = Element::with_tag("input")
        .classes(self.style.search_input_classes.clone())
        .attr("type", "text")
        .attr("placeholder", "Search...")
        .attr("value", state.search_query.clone());

      dropdown = dropdown.child(search_input.build());
    }

    // Add options
    let visible_options: Vec<_> = state
      .filtered_indices
      .iter()
      .skip(state.scroll_offset)
      .take(self.style.max_height)
      .collect();

    for &option_index in visible_options {
      let option = &self.options[option_index];
      let mut option_element =
        Element::with_tag("div".to_string()).classes(self.style.option_classes.clone());

      // Add state classes
      if state.selected_indices.contains(&option_index) {
        option_element = option_element.classes(self.style.selected_option_classes.clone());
      }

      if state.highlighted_index == Some(option_index) {
        option_element = option_element.classes(self.style.highlighted_option_classes.clone());
      }

      if option.disabled {
        option_element = option_element.classes(self.style.disabled_option_classes.clone());
      }

      // Build option content
      let mut content_parts = Vec::new();

      // Add selection marker for multi-select
      if self.mode == SelectMode::Multiple {
        if state.selected_indices.contains(&option_index) {
          content_parts.push(self.style.selected_marker.clone());
        } else {
          content_parts.push(" ".to_string());
        }
      }

      // Add icon
      if self.style.show_icons {
        if let Some(icon) = &option.icon {
          content_parts.push(icon.clone());
        }
      }

      // Add label
      content_parts.push(option.label.clone());

      // Add description
      if self.style.show_descriptions {
        if let Some(description) = &option.description {
          content_parts.push(description.to_string());
        }
      }

      option_element = option_element.content(content_parts.join(" "));
      dropdown = dropdown.child(option_element.build());
    }

    dropdown.build()
  }

  /// Get display text for the selected value(s)
  pub fn display_text(&self) -> String {
    let state = self.state.get();

    if state.selected_indices.is_empty() {
      return self.placeholder.clone();
    }

    match self.mode {
      SelectMode::Single => {
        if let Some(&index) = state.selected_indices.first() {
          self.options[index].label.clone()
        } else {
          self.placeholder.clone()
        }
      }
      SelectMode::Multiple => {
        if state.selected_indices.len() == 1 {
          self.options[state.selected_indices[0]].label.clone()
        } else {
          format!("{} items selected", state.selected_indices.len())
        }
      }
    }
  }
}

impl Select {
  /// Handle key events for the select widget
  pub fn handle_key(&mut self, key: &str) -> Result<bool> {
    let state = self.state.get();
    let is_open = state.open;
    drop(state);

    match key {
      "Enter" | "Space" => {
        if is_open {
          self.select_highlighted()?;
        } else {
          self.open();
        }
        Ok(true)
      }
      "Escape" => {
        if is_open {
          self.close();
          Ok(true)
        } else {
          Ok(false)
        }
      }
      "ArrowDown" => {
        if is_open {
          self.navigate_next();
        } else {
          self.open();
        }
        Ok(true)
      }
      "ArrowUp" => {
        if is_open {
          self.navigate_previous();
          Ok(true)
        } else {
          Ok(false)
        }
      }
      "Tab" => {
        if is_open {
          self.close();
        }
        Ok(false) // Let tab navigation continue
      }
      key if key.len() == 1 => {
        // Handle search input for searchable selects
        if is_open && self.searchable {
          self.state.update(|state| {
            state.search_query.push_str(key);
          });
          self.update_filtered_options();
          Ok(true)
        } else {
          Ok(false)
        }
      }
      "Backspace" => {
        if is_open && self.searchable {
          self.state.update(|state| {
            state.search_query.pop();
          });
          self.update_filtered_options();
          Ok(true)
        } else {
          Ok(false)
        }
      }
      _ => Ok(false),
    }
  }
}

/// Builder for creating Select widgets
pub struct SelectBuilder {
  id: String,
  options: Vec<SelectOption>,
  mode: SelectMode,
  searchable: bool,
  placeholder: String,
  position: DropdownPosition,
  style: SelectStyle,
  selected_indices: Vec<usize>,
  on_change: Option<OnChangeCallback>,
  on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
  disabled: bool,
  required: bool,
  filter_fn: Option<FilterCallback>,
}

impl SelectBuilder {
  /// Create a new select builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      options: Vec::new(),
      mode: SelectMode::Single,
      searchable: false,
      placeholder: "Select an option...".to_string(),
      position: DropdownPosition::Auto,
      style: SelectStyle::default(),
      selected_indices: Vec::new(),
      on_change: None,
      on_toggle: None,
      disabled: false,
      required: false,
      filter_fn: None,
    }
  }

  /// Set options from string labels
  pub fn options<S: Into<String>>(mut self, labels: Vec<S>) -> Self {
    self.options = labels
      .into_iter()
      .enumerate()
      .map(|(i, label)| SelectOption::new(i.to_string(), label.into()))
      .collect();
    self
  }

  /// Set custom options
  pub fn custom_options(mut self, options: Vec<SelectOption>) -> Self {
    self.options = options;
    self
  }

  /// Enable multi-select mode
  pub fn multi_select(mut self, multi: bool) -> Self {
    self.mode = if multi {
      SelectMode::Multiple
    } else {
      SelectMode::Single
    };
    self
  }

  /// Enable search filtering
  pub fn searchable(mut self, searchable: bool) -> Self {
    self.searchable = searchable;
    self
  }

  /// Set placeholder text
  pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self {
    self.placeholder = placeholder.into();
    self
  }

  /// Set dropdown position preference
  pub fn position(mut self, position: DropdownPosition) -> Self {
    self.position = position;
    self
  }

  /// Set styling
  pub fn style(mut self, style: SelectStyle) -> Self {
    self.style = style;
    self
  }

  /// Set maximum height of dropdown
  pub fn max_height(mut self, height: usize) -> Self {
    self.style.max_height = height;
    self
  }

  /// Set initially selected option (single-select)
  pub fn selected(mut self, index: Option<usize>) -> Self {
    self.selected_indices = index.map(|i| vec![i]).unwrap_or_default();
    self
  }

  /// Set initially selected options (multi-select)
  pub fn selected_indices(mut self, indices: Vec<usize>) -> Self {
    self.selected_indices = indices;
    self
  }

  /// Set change callback
  pub fn on_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&[usize]) + Send + Sync + 'static,
  {
    self.on_change = Some(Arc::new(callback));
    self
  }

  /// Set toggle callback
  pub fn on_toggle<F>(mut self, callback: F) -> Self
  where
    F: Fn(bool) + Send + Sync + 'static,
  {
    self.on_toggle = Some(Arc::new(callback));
    self
  }

  /// Set disabled state
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Set required state
  pub fn required(mut self, required: bool) -> Self {
    self.required = required;
    self
  }

  /// Set custom filter function
  pub fn filter<F>(mut self, filter_fn: F) -> Self
  where
    F: Fn(&SelectOption, &str) -> bool + Send + Sync + 'static,
  {
    self.filter_fn = Some(Arc::new(filter_fn));
    self
  }

  /// Build the select widget
  pub fn build(self) -> Select {
    let initial_state = SelectState {
      selected_indices: self.selected_indices,
      filtered_indices: (0..self.options.len()).collect(),
      ..Default::default()
    };

    Select {
      id: self.id,
      options: self.options,
      state: Reactive::new(initial_state),
      mode: self.mode,
      searchable: self.searchable,
      placeholder: self.placeholder,
      position: self.position,
      style: self.style,
      on_change: self.on_change,
      on_toggle: self.on_toggle,
      disabled: self.disabled,
      required: self.required,
      filter_fn: self.filter_fn,
    }
  }
}

/// Convenience functions for common select patterns
impl SelectBuilder {
  /// Create a simple yes/no select
  pub fn yes_no(id: String) -> Select {
    Self::new(id)
      .options(vec!["Yes", "No"])
      .placeholder("Select...")
      .build()
  }
  /// Create a language selection dropdown
  pub fn languages(id: String) -> Select {
    let options = vec![
      SelectOption::new("rust", "Rust").icon("🦀"),
      SelectOption::new("typescript", "TypeScript").icon("📘"),
      SelectOption::new("javascript", "JavaScript").icon("📜"),
      SelectOption::new("python", "Python").icon("🐍"),
      SelectOption::new("go", "Go").icon("🐹"),
      SelectOption::new("java", "Java").icon("☕"),
      SelectOption::new("csharp", "C#").icon("🔷"),
      SelectOption::new("cpp", "C++").icon("⚡"),
    ];
    Self::new(id)
      .custom_options(options)
      .searchable(true)
      .placeholder("Choose a language...")
      .build()
  }
  /// Create a priority selection dropdown
  pub fn priority(id: String) -> Select {
    let options = vec![
      SelectOption::new("high", "High Priority").icon("🔴"),
      SelectOption::new("medium", "Medium Priority").icon("🟡"),
      SelectOption::new("low", "Low Priority").icon("🟢"),
    ];
    Self::new(id)
      .custom_options(options)
      .placeholder("Select priority...")
      .build()
  }
}
impl fmt::Display for Select {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "Select({}): {}", self.id, self.display_text())
  }
}
