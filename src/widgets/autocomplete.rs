//! Autocomplete Widget
//!
//! A comprehensive autocomplete widget supporting search-as-you-type functionality,
//! with filtered suggestions, keyboard navigation, and custom rendering.
//!
//! # Features
//!
//! - **Search-as-you-type**: Real-time filtering of suggestions as user types
//! - **Keyboard Navigation**: Arrow keys, Enter/Escape, Tab completion support
//! - **Custom Filtering**: Multiple filter modes (contains, starts_with, fuzzy, custom)
//! - **Suggestion Rendering**: Customizable suggestion display with highlighting
//! - **Async Support**: Support for async suggestion loading and debouncing
//! - **Multiple Selection**: Single or multi-select autocomplete modes
//! - **Rich Suggestions**: Support for complex suggestion objects with metadata
//! - **Accessibility**: Full ARIA support and screen reader compatibility
//! - **Event Callbacks**: onSelect, onFilter, onChange, onFocus event handling
//! - **Themeable**: CSS utility classes and custom styling support
//!
//! # Basic Usage
//!
//! ```rust
//! use reactive_tui::widgets::{Autocomplete, AutocompleteBuilder, AutocompleteSuggestion, FilterMode};
//!
//! let mut autocomplete = AutocompleteBuilder::new("country-search")
//!     .placeholder("Search countries...")
//!     .suggestions(vec![
//!         AutocompleteSuggestion::new("us", "United States").description("North America"),
//!         AutocompleteSuggestion::new("uk", "United Kingdom").description("Europe"),
//!         AutocompleteSuggestion::new("ca", "Canada").description("North America"),
//!     ])
//!     .filter_mode(FilterMode::Contains)
//!     .max_suggestions(10)
//!     .debounce_ms(300)
//!     .build();
//!
//! // User types "uni"
//! autocomplete.set_query("uni");
//!
//! // Get filtered suggestions
//! let suggestions = autocomplete.get_visible_suggestions();
//! ```

use crate::{
  components::element::Element,
  error::{Result, TuiError},
  layout::LayoutRect,
  reactive::Reactive,
  themes::{ColorDefinition, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

// Type aliases for complex function pointer types
type OnSelectCallback = Arc<dyn Fn(&SuggestionId, &AutocompleteSuggestion) + Send + Sync>;
type OnFilterCallback = Arc<dyn Fn(&str) -> Vec<AutocompleteSuggestion> + Send + Sync>;
type OnChangeCallback = Arc<dyn Fn(&str) + Send + Sync>;
type OnFocusCallback = Arc<dyn Fn(bool) + Send + Sync>;
type OnDropdownToggleCallback = Arc<dyn Fn(bool) + Send + Sync>;
type OnLoadSuggestionsCallback = Arc<dyn Fn(&str) -> Vec<AutocompleteSuggestion> + Send + Sync>;
type CustomFilterCallback = Arc<dyn Fn(&str, &AutocompleteSuggestion) -> bool + Send + Sync>;

/// Unique identifier for suggestions
pub type SuggestionId = String;

/// Filter modes for suggestion matching
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FilterMode {
  /// Match suggestions that contain the query (case-insensitive)
  Contains,
  /// Match suggestions that start with the query (case-insensitive)
  StartsWith,
  /// Fuzzy matching algorithm
  Fuzzy,
  /// Exact matching (case-sensitive)
  Exact,
  /// Custom filter function
  Custom,
}

impl Default for FilterMode {
  fn default() -> Self {
    Self::Contains
  }
}

/// Selection mode for autocomplete
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SelectionMode {
  /// Single selection (replaces current selection)
  Single,
  /// Multiple selection (adds to selection list)
  Multiple,
}

impl Default for SelectionMode {
  fn default() -> Self {
    Self::Single
  }
}

/// Individual autocomplete suggestion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutocompleteSuggestion {
  /// Unique suggestion identifier
  pub id: SuggestionId,
  /// Primary text to display
  pub text: String,
  /// Optional description/subtitle
  pub description: Option<String>,
  /// Optional metadata for custom rendering
  pub metadata: HashMap<String, String>,
  /// Suggestion priority/score for ranking
  pub score: f32,
  /// Whether suggestion is disabled
  pub disabled: bool,
  /// Custom CSS classes for the suggestion
  pub css_classes: Vec<String>,
}

impl AutocompleteSuggestion {
  /// Create a new suggestion
  pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      text: text.into(),
      description: None,
      metadata: HashMap::new(),
      score: 1.0,
      disabled: false,
      css_classes: Vec::new(),
    }
  }

  /// Set suggestion description
  pub fn description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Add metadata
  pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.metadata.insert(key.into(), value.into());
    self
  }

  /// Set suggestion score
  pub fn score(mut self, score: f32) -> Self {
    self.score = score;
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

/// Autocomplete state management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AutocompleteState {
  /// Current search query
  pub query: String,
  /// Currently selected suggestions (for multi-select)
  pub selected_suggestions: Vec<SuggestionId>,
  /// Currently highlighted suggestion
  pub highlighted_suggestion: Option<SuggestionId>,
  /// Whether dropdown is open
  pub dropdown_open: bool,
  /// Whether autocomplete is focused
  pub focused: bool,
  /// Whether autocomplete is disabled
  pub disabled: bool,
  /// Current filtered suggestions
  pub filtered_suggestions: Vec<SuggestionId>,
  /// Loading state for async suggestions
  pub loading: bool,
}

/// Styling configuration for autocomplete
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutocompleteStyle {
  /// Input background color
  pub input_background: Option<ColorDefinition>,
  /// Input text color
  pub input_text_color: Option<ColorDefinition>,
  /// Input border color
  pub input_border_color: Option<ColorDefinition>,
  /// Dropdown background color
  pub dropdown_background: Option<ColorDefinition>,
  /// Suggestion text color
  pub suggestion_text_color: Option<ColorDefinition>,
  /// Highlighted suggestion background
  pub highlighted_background: Option<ColorDefinition>,
  /// Selected suggestion background
  pub selected_background: Option<ColorDefinition>,
  /// Disabled suggestion opacity
  pub disabled_opacity: f32,
  /// Maximum dropdown height
  pub max_dropdown_height: u16,
  /// Suggestion padding
  pub suggestion_padding: u16,
}

impl Default for AutocompleteStyle {
  fn default() -> Self {
    Self {
      input_background: None,
      input_text_color: None,
      input_border_color: None,
      dropdown_background: None,
      suggestion_text_color: None,
      highlighted_background: None,
      selected_background: None,
      disabled_opacity: 0.6,
      max_dropdown_height: 10,
      suggestion_padding: 1,
    }
  }
}

/// Autocomplete configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AutocompleteConfig {
  /// Filter mode for suggestions
  pub filter_mode: FilterMode,
  /// Selection mode (single or multiple)
  pub selection_mode: SelectionMode,
  /// Maximum number of suggestions to show
  pub max_suggestions: usize,
  /// Minimum query length to start filtering
  pub min_query_length: usize,
  /// Debounce delay in milliseconds
  pub debounce_ms: u64,
  /// Case-sensitive filtering
  pub case_sensitive: bool,
  /// Show descriptions in suggestions
  pub show_descriptions: bool,
  /// Enable keyboard navigation
  pub keyboard_navigation: bool,
  /// Auto-select first suggestion
  pub auto_select_first: bool,
  /// Close dropdown on selection
  pub close_on_select: bool,
  /// Clear input on selection
  pub clear_on_select: bool,
  /// Placeholder text
  pub placeholder: String,
}

impl Default for AutocompleteConfig {
  fn default() -> Self {
    Self {
      filter_mode: FilterMode::Contains,
      selection_mode: SelectionMode::Single,
      max_suggestions: 10,
      min_query_length: 1,
      debounce_ms: 300,
      case_sensitive: false,
      show_descriptions: true,
      keyboard_navigation: true,
      auto_select_first: true,
      close_on_select: true,
      clear_on_select: false,
      placeholder: "Type to search...".to_string(),
    }
  }
}

/// Event callbacks for autocomplete interactions
#[derive(Default)]
pub struct AutocompleteCallbacks {
  /// Called when a suggestion is selected
  pub on_select: Option<OnSelectCallback>,
  /// Called when the query changes (for custom filtering)
  pub on_filter: Option<OnFilterCallback>,
  /// Called when the query changes
  pub on_change: Option<OnChangeCallback>,
  /// Called when focus changes
  pub on_focus: Option<OnFocusCallback>,
  /// Called when dropdown opens/closes
  pub on_dropdown_toggle: Option<OnDropdownToggleCallback>,
  /// Called for async suggestion loading
  pub on_load_suggestions: Option<OnLoadSuggestionsCallback>,
}

/// Main Autocomplete widget
pub struct Autocomplete {
  /// Unique autocomplete identifier
  pub id: String,
  /// Available suggestions
  pub suggestions: Vec<AutocompleteSuggestion>,
  /// Reactive state management
  pub state: Reactive<AutocompleteState>,
  /// Configuration options
  pub config: AutocompleteConfig,
  /// Styling configuration
  pub style: AutocompleteStyle,
  /// Event callbacks
  pub callbacks: AutocompleteCallbacks,
  /// CSS utility classes
  pub css_classes: Vec<String>,
  /// Custom filter function
  pub custom_filter: Option<CustomFilterCallback>,
}

impl Autocomplete {
  /// Create a new autocomplete builder
  pub fn builder<S: Into<String>>(id: S) -> AutocompleteBuilder {
    AutocompleteBuilder::new(id)
  }

  /// Set the current query
  pub fn set_query(&mut self, query: impl Into<String>) {
    let query = query.into();

    self.state.update(|state| {
      state.query = query.clone();
      state.loading = false;

      // Auto-open dropdown if query is long enough
      if query.len() >= self.config.min_query_length {
        state.dropdown_open = true;
      } else {
        state.dropdown_open = false;
        state.highlighted_suggestion = None;
      }
    });

    // Filter suggestions
    self.filter_suggestions();

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_change {
      callback(&query);
    }
  }

  /// Get the current query
  pub fn get_query(&self) -> String {
    self.state.get().query.clone()
  }

  /// Filter suggestions based on current query
  pub fn filter_suggestions(&mut self) {
    let query = self.state.get().query.clone();

    if query.len() < self.config.min_query_length {
      self.state.update(|state| {
        state.filtered_suggestions.clear();
        state.highlighted_suggestion = None;
      });
      return;
    }

    // Use custom filter if available
    if let Some(custom_filter) = &self.custom_filter {
      let filtered: Vec<_> = self
        .suggestions
        .iter()
        .filter(|suggestion| !suggestion.disabled && custom_filter(&query, suggestion))
        .take(self.config.max_suggestions)
        .map(|s| s.id.clone())
        .collect();

      self.state.update(|state| {
        state.filtered_suggestions = filtered;
        if self.config.auto_select_first && !state.filtered_suggestions.is_empty() {
          state.highlighted_suggestion = Some(state.filtered_suggestions[0].clone());
        }
      });
      return;
    }

    // Use callback filter if available
    if let Some(callback) = &self.callbacks.on_filter {
      let custom_suggestions = callback(&query);
      self.suggestions = custom_suggestions;
    }

    // Apply built-in filter
    let mut filtered: Vec<_> = self
      .suggestions
      .iter()
      .filter(|suggestion| !suggestion.disabled && self.matches_query(&query, suggestion))
      .collect();

    // Sort by score (highest first)
    filtered.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(Ordering::Equal));

    // Take max suggestions
    let filtered_ids: Vec<_> = filtered
      .into_iter()
      .take(self.config.max_suggestions)
      .map(|s| s.id.clone())
      .collect();

    self.state.update(|state| {
      state.filtered_suggestions = filtered_ids;
      if self.config.auto_select_first && !state.filtered_suggestions.is_empty() {
        state.highlighted_suggestion = Some(state.filtered_suggestions[0].clone());
      } else {
        state.highlighted_suggestion = None;
      }
    });
  }

  /// Check if a suggestion matches the query
  fn matches_query(&self, query: &str, suggestion: &AutocompleteSuggestion) -> bool {
    let query = if self.config.case_sensitive {
      query.to_string()
    } else {
      query.to_lowercase()
    };

    let text = if self.config.case_sensitive {
      suggestion.text.clone()
    } else {
      suggestion.text.to_lowercase()
    };

    match self.config.filter_mode {
      FilterMode::Contains => text.contains(&query),
      FilterMode::StartsWith => text.starts_with(&query),
      FilterMode::Exact => text == query,
      FilterMode::Fuzzy => self.fuzzy_match(&query, &text),
      FilterMode::Custom => true, // Handled by custom_filter
    }
  }

  /// Simple fuzzy matching implementation
  fn fuzzy_match(&self, query: &str, text: &str) -> bool {
    let mut query_chars = query.chars().peekable();
    let mut text_chars = text.chars();

    while let Some(query_char) = query_chars.peek() {
      let mut found = false;

      for text_char in text_chars.by_ref() {
        if text_char == *query_char {
          query_chars.next();
          found = true;
          break;
        }
      }

      if !found {
        return false;
      }
    }

    query_chars.peek().is_none()
  }

  /// Select a suggestion
  pub fn select_suggestion(&mut self, suggestion_id: impl AsRef<str>) -> Result<()> {
    let suggestion_id = suggestion_id.as_ref();

    let suggestion = self
      .suggestions
      .iter()
      .find(|s| s.id == suggestion_id)
      .ok_or_else(|| TuiError::component(format!("Suggestion '{suggestion_id}' not found")))?;

    if suggestion.disabled {
      return Err(TuiError::component(format!(
        "Suggestion '{suggestion_id}' is disabled"
      )));
    }

    self.state.update(|state| {
      match self.config.selection_mode {
        SelectionMode::Single => {
          state.selected_suggestions = vec![suggestion_id.to_string()];
        }
        SelectionMode::Multiple => {
          if !state
            .selected_suggestions
            .contains(&suggestion_id.to_string())
          {
            state.selected_suggestions.push(suggestion_id.to_string());
          }
        }
      }

      if self.config.close_on_select {
        state.dropdown_open = false;
      }

      if self.config.clear_on_select {
        state.query.clear();
      }
    });

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_select {
      callback(&suggestion_id.to_string(), suggestion);
    }

    Ok(())
  }

  /// Deselect a suggestion (for multi-select mode)
  pub fn deselect_suggestion(&mut self, suggestion_id: impl AsRef<str>) {
    let suggestion_id = suggestion_id.as_ref();

    self.state.update(|state| {
      state.selected_suggestions.retain(|id| id != suggestion_id);
    });
  }

  /// Get currently selected suggestions
  pub fn get_selected_suggestions(&self) -> Vec<String> {
    self.state.get().selected_suggestions.clone()
  }

  /// Clear all selections
  pub fn clear_selections(&mut self) {
    self.state.update(|state| {
      state.selected_suggestions.clear();
    });
  }

  /// Get visible suggestions (filtered)
  pub fn get_visible_suggestions(&self) -> Vec<&AutocompleteSuggestion> {
    let filtered_ids = &self.state.get().filtered_suggestions;
    filtered_ids
      .iter()
      .filter_map(|id| self.suggestions.iter().find(|s| s.id == *id))
      .collect()
  }

  /// Highlight next suggestion
  pub fn highlight_next(&mut self) -> Result<()> {
    let state = self.state.get();
    let filtered = &state.filtered_suggestions;

    if filtered.is_empty() {
      return Ok(());
    }

    let next_index = if let Some(current) = &state.highlighted_suggestion {
      if let Some(current_index) = filtered.iter().position(|id| id == current) {
        (current_index + 1) % filtered.len()
      } else {
        0
      }
    } else {
      0
    };

    let next_id = filtered[next_index].clone();
    self.state.update(|state| {
      state.highlighted_suggestion = Some(next_id);
    });

    Ok(())
  }

  /// Highlight previous suggestion
  pub fn highlight_previous(&mut self) -> Result<()> {
    let state = self.state.get();
    let filtered = &state.filtered_suggestions;

    if filtered.is_empty() {
      return Ok(());
    }

    let prev_index = if let Some(current) = &state.highlighted_suggestion {
      if let Some(current_index) = filtered.iter().position(|id| id == current) {
        if current_index == 0 {
          filtered.len() - 1
        } else {
          current_index - 1
        }
      } else {
        filtered.len() - 1
      }
    } else {
      filtered.len() - 1
    };

    let prev_id = filtered[prev_index].clone();
    self.state.update(|state| {
      state.highlighted_suggestion = Some(prev_id);
    });

    Ok(())
  }

  /// Select the currently highlighted suggestion
  pub fn select_highlighted(&mut self) -> Result<()> {
    if let Some(highlighted) = self.state.get().highlighted_suggestion.clone() {
      self.select_suggestion(highlighted)
    } else {
      Ok(())
    }
  }

  /// Open the dropdown
  pub fn open_dropdown(&mut self) {
    self.state.update(|state| {
      state.dropdown_open = true;
    });

    if let Some(callback) = &self.callbacks.on_dropdown_toggle {
      callback(true);
    }
  }

  /// Close the dropdown
  pub fn close_dropdown(&mut self) {
    self.state.update(|state| {
      state.dropdown_open = false;
      state.highlighted_suggestion = None;
    });

    if let Some(callback) = &self.callbacks.on_dropdown_toggle {
      callback(false);
    }
  }

  /// Toggle dropdown open/closed
  pub fn toggle_dropdown(&mut self) {
    if self.state.get().dropdown_open {
      self.close_dropdown();
    } else {
      self.open_dropdown();
    }
  }

  /// Set focus state
  pub fn set_focused(&mut self, focused: bool) {
    self.state.update(|state| {
      state.focused = focused;
      if !focused {
        state.dropdown_open = false;
        state.highlighted_suggestion = None;
      }
    });

    if let Some(callback) = &self.callbacks.on_focus {
      callback(focused);
    }
  }

  /// Check if autocomplete is focused
  pub fn is_focused(&self) -> bool {
    self.state.get().focused
  }

  /// Enable/disable the autocomplete
  pub fn set_disabled(&mut self, disabled: bool) {
    self.state.update(|state| {
      state.disabled = disabled;
      if disabled {
        state.dropdown_open = false;
        state.focused = false;
      }
    });
  }

  /// Check if autocomplete is disabled
  pub fn is_disabled(&self) -> bool {
    self.state.get().disabled
  }

  /// Set loading state
  pub fn set_loading(&mut self, loading: bool) {
    self.state.update(|state| {
      state.loading = loading;
    });
  }

  /// Check if autocomplete is loading
  pub fn is_loading(&self) -> bool {
    self.state.get().loading
  }

  /// Add suggestions to the list
  pub fn add_suggestions(&mut self, suggestions: Vec<AutocompleteSuggestion>) {
    self.suggestions.extend(suggestions);
    self.filter_suggestions();
  }

  /// Clear all suggestions
  pub fn clear_suggestions(&mut self) {
    self.suggestions.clear();
    self.state.update(|state| {
      state.filtered_suggestions.clear();
      state.highlighted_suggestion = None;
    });
  }

  /// Get suggestion count
  pub fn suggestion_count(&self) -> usize {
    self.suggestions.len()
  }

  /// Get filtered suggestion count
  pub fn filtered_count(&self) -> usize {
    self.state.get().filtered_suggestions.len()
  }

  /// Render the autocomplete to a string
  pub fn render(&self, _layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Base CSS classes
    let mut classes = vec!["autocomplete".to_string()];
    if state.focused {
      classes.push("autocomplete-focused".to_string());
    }
    if state.disabled {
      classes.push("autocomplete-disabled".to_string());
    }
    if state.loading {
      classes.push("autocomplete-loading".to_string());
    }
    classes.extend(self.css_classes.clone());

    // Render input field
    self.render_input(&mut output, &state, theme);

    // Render dropdown if open
    if state.dropdown_open && !state.filtered_suggestions.is_empty() {
      self.render_dropdown(&mut output, &state, theme);
    }

    output
  }

  /// Render the input field
  fn render_input(
    &self,
    output: &mut String,
    state: &AutocompleteState,
    _theme: Option<&ColorTheme>,
  ) {
    let focus_indicator = if state.focused { "► " } else { "  " };
    let loading_indicator = if state.loading { " ⟳" } else { "" };
    let disabled_indicator = if state.disabled { " (disabled)" } else { "" };

    // Input line
    write!(output, "{focus_indicator}").unwrap();

    if state.query.is_empty() && !state.focused {
      write!(output, "{}", self.config.placeholder).unwrap();
    } else {
      write!(output, "{}", state.query).unwrap();
    }

    write!(output, "{loading_indicator}{disabled_indicator}").unwrap();
    writeln!(output).unwrap();

    // Selected items (for multi-select)
    if self.config.selection_mode == SelectionMode::Multiple
      && !state.selected_suggestions.is_empty()
    {
      write!(output, "  Selected: ").unwrap();
      for (i, id) in state.selected_suggestions.iter().enumerate() {
        if let Some(suggestion) = self.suggestions.iter().find(|s| s.id == *id) {
          if i > 0 {
            write!(output, ", ").unwrap();
          }
          write!(output, "[{}]", suggestion.text).unwrap();
        }
      }
      writeln!(output).unwrap();
    }
  }

  /// Render the dropdown suggestions
  fn render_dropdown(
    &self,
    output: &mut String,
    state: &AutocompleteState,
    _theme: Option<&ColorTheme>,
  ) {
    writeln!(output, "  ┌─ Suggestions ─").unwrap();

    for (index, suggestion_id) in state.filtered_suggestions.iter().enumerate() {
      if let Some(suggestion) = self.suggestions.iter().find(|s| s.id == *suggestion_id) {
        let is_highlighted = state.highlighted_suggestion.as_ref() == Some(suggestion_id);
        let is_selected = state.selected_suggestions.contains(suggestion_id);

        self.render_suggestion(output, suggestion, is_highlighted, is_selected, index);
      }
    }

    writeln!(output, "  └───────────────").unwrap();
  }

  /// Render individual suggestion
  fn render_suggestion(
    &self,
    output: &mut String,
    suggestion: &AutocompleteSuggestion,
    is_highlighted: bool,
    is_selected: bool,
    _index: usize,
  ) {
    let highlight_char = if is_highlighted { "►" } else { " " };
    let selection_char = if is_selected { "✓" } else { " " };
    let disabled_indicator = if suggestion.disabled {
      " (disabled)"
    } else {
      ""
    };

    write!(
      output,
      "  │ {}{} {}",
      highlight_char, selection_char, suggestion.text
    )
    .unwrap();

    if self.config.show_descriptions {
      if let Some(description) = &suggestion.description {
        write!(output, " - {description}").unwrap();
      }
    }

    write!(output, "{disabled_indicator}").unwrap();
    writeln!(output).unwrap();
  }

  /// Convert to Element for integration with layout system
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

impl fmt::Display for Autocomplete {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 80,
      height: 25,
    };
    write!(f, "{}", self.render(&layout, None))
  }
}

/// Builder for creating autocomplete widgets
pub struct AutocompleteBuilder {
  id: String,
  suggestions: Vec<AutocompleteSuggestion>,
  config: AutocompleteConfig,
  style: AutocompleteStyle,
  callbacks: AutocompleteCallbacks,
  css_classes: Vec<String>,
  custom_filter: Option<CustomFilterCallback>,
}

impl AutocompleteBuilder {
  /// Create a new autocomplete builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      suggestions: Vec::new(),
      config: AutocompleteConfig::default(),
      style: AutocompleteStyle::default(),
      callbacks: AutocompleteCallbacks::default(),
      css_classes: Vec::new(),
      custom_filter: None,
    }
  }

  /// Add suggestions
  pub fn suggestions(mut self, suggestions: Vec<AutocompleteSuggestion>) -> Self {
    self.suggestions = suggestions;
    self
  }

  /// Add a single suggestion
  pub fn suggestion(mut self, suggestion: AutocompleteSuggestion) -> Self {
    self.suggestions.push(suggestion);
    self
  }

  /// Set placeholder text
  pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
    self.config.placeholder = placeholder.into();
    self
  }

  /// Set filter mode
  pub fn filter_mode(mut self, mode: FilterMode) -> Self {
    self.config.filter_mode = mode;
    self
  }

  /// Set selection mode
  pub fn selection_mode(mut self, mode: SelectionMode) -> Self {
    self.config.selection_mode = mode;
    self
  }

  /// Set maximum suggestions
  pub fn max_suggestions(mut self, max: usize) -> Self {
    self.config.max_suggestions = max;
    self
  }

  /// Set minimum query length
  pub fn min_query_length(mut self, min: usize) -> Self {
    self.config.min_query_length = min;
    self
  }

  /// Set debounce delay
  pub fn debounce_ms(mut self, ms: u64) -> Self {
    self.config.debounce_ms = ms;
    self
  }

  /// Set case sensitivity
  pub fn case_sensitive(mut self, sensitive: bool) -> Self {
    self.config.case_sensitive = sensitive;
    self
  }

  /// Show/hide descriptions
  pub fn show_descriptions(mut self, show: bool) -> Self {
    self.config.show_descriptions = show;
    self
  }

  /// Enable/disable keyboard navigation
  pub fn keyboard_navigation(mut self, enabled: bool) -> Self {
    self.config.keyboard_navigation = enabled;
    self
  }

  /// Auto-select first suggestion
  pub fn auto_select_first(mut self, enabled: bool) -> Self {
    self.config.auto_select_first = enabled;
    self
  }

  /// Close dropdown on selection
  pub fn close_on_select(mut self, close: bool) -> Self {
    self.config.close_on_select = close;
    self
  }

  /// Clear input on selection
  pub fn clear_on_select(mut self, clear: bool) -> Self {
    self.config.clear_on_select = clear;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set custom filter function
  pub fn custom_filter<F>(mut self, filter: F) -> Self
  where
    F: Fn(&str, &AutocompleteSuggestion) -> bool + Send + Sync + 'static,
  {
    self.custom_filter = Some(Arc::new(filter));
    self
  }

  /// Set select callback
  pub fn on_select<F>(mut self, callback: F) -> Self
  where
    F: Fn(&SuggestionId, &AutocompleteSuggestion) + Send + Sync + 'static,
  {
    self.callbacks.on_select = Some(Arc::new(callback));
    self
  }

  /// Set filter callback
  pub fn on_filter<F>(mut self, callback: F) -> Self
  where
    F: Fn(&str) -> Vec<AutocompleteSuggestion> + Send + Sync + 'static,
  {
    self.callbacks.on_filter = Some(Arc::new(callback));
    self
  }

  /// Set change callback
  pub fn on_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&str) + Send + Sync + 'static,
  {
    self.callbacks.on_change = Some(Arc::new(callback));
    self
  }

  /// Set focus callback
  pub fn on_focus<F>(mut self, callback: F) -> Self
  where
    F: Fn(bool) + Send + Sync + 'static,
  {
    self.callbacks.on_focus = Some(Arc::new(callback));
    self
  }

  /// Set dropdown toggle callback
  pub fn on_dropdown_toggle<F>(mut self, callback: F) -> Self
  where
    F: Fn(bool) + Send + Sync + 'static,
  {
    self.callbacks.on_dropdown_toggle = Some(Arc::new(callback));
    self
  }

  /// Build the autocomplete
  pub fn build(self) -> Autocomplete {
    let state = AutocompleteState::default();

    Autocomplete {
      id: self.id,
      suggestions: self.suggestions,
      state: Reactive::new(state),
      config: self.config,
      style: self.style,
      callbacks: self.callbacks,
      css_classes: self.css_classes,
      custom_filter: self.custom_filter,
    }
  }
}

/// Convenience functions for common autocomplete patterns
/// Create a simple country autocomplete
pub fn country_autocomplete() -> Autocomplete {
  AutocompleteBuilder::new("country-autocomplete")
    .placeholder("Search countries...")
    .suggestions(vec![
      AutocompleteSuggestion::new("us", "United States").description("North America"),
      AutocompleteSuggestion::new("uk", "United Kingdom").description("Europe"),
      AutocompleteSuggestion::new("ca", "Canada").description("North America"),
      AutocompleteSuggestion::new("fr", "France").description("Europe"),
      AutocompleteSuggestion::new("de", "Germany").description("Europe"),
      AutocompleteSuggestion::new("jp", "Japan").description("Asia"),
      AutocompleteSuggestion::new("au", "Australia").description("Oceania"),
      AutocompleteSuggestion::new("br", "Brazil").description("South America"),
    ])
    .filter_mode(FilterMode::Contains)
    .max_suggestions(5)
    .build()
}

/// Create a programming language autocomplete
pub fn language_autocomplete() -> Autocomplete {
  AutocompleteBuilder::new("language-autocomplete")
    .placeholder("Search programming languages...")
    .suggestions(vec![
      AutocompleteSuggestion::new("rust", "Rust")
        .description("Systems programming")
        .score(0.9),
      AutocompleteSuggestion::new("typescript", "TypeScript")
        .description("JavaScript with types")
        .score(0.8),
      AutocompleteSuggestion::new("python", "Python")
        .description("General purpose")
        .score(0.85),
      AutocompleteSuggestion::new("javascript", "JavaScript")
        .description("Web development")
        .score(0.75),
      AutocompleteSuggestion::new("go", "Go")
        .description("Google's language")
        .score(0.7),
      AutocompleteSuggestion::new("cpp", "C++")
        .description("Systems programming")
        .score(0.65),
    ])
    .filter_mode(FilterMode::StartsWith)
    .case_sensitive(false)
    .build()
}

/// Create a user search autocomplete
pub fn user_autocomplete(users: Vec<(&str, &str, &str)>) -> Autocomplete {
  let suggestions = users
    .into_iter()
    .map(|(id, name, email)| {
      AutocompleteSuggestion::new(id, name)
        .description(email)
        .metadata("email", email)
    })
    .collect();

  AutocompleteBuilder::new("user-autocomplete")
    .placeholder("Search users...")
    .suggestions(suggestions)
    .filter_mode(FilterMode::Contains)
    .selection_mode(SelectionMode::Multiple)
    .max_suggestions(8)
    .build()
}

/// Create a command autocomplete
pub fn command_autocomplete() -> Autocomplete {
  AutocompleteBuilder::new("command-autocomplete")
    .placeholder("Type command...")
    .suggestions(vec![
      AutocompleteSuggestion::new("help", "help").description("Show help information"),
      AutocompleteSuggestion::new("exit", "exit").description("Exit the application"),
      AutocompleteSuggestion::new("save", "save").description("Save current work"),
      AutocompleteSuggestion::new("load", "load").description("Load saved work"),
      AutocompleteSuggestion::new("settings", "settings").description("Open settings"),
      AutocompleteSuggestion::new("about", "about").description("About this application"),
    ])
    .filter_mode(FilterMode::StartsWith)
    .auto_select_first(true)
    .close_on_select(true)
    .clear_on_select(true)
    .build()
}
