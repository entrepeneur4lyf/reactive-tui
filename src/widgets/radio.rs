//! Radio button widget for single selection from a group
//!
//! Provides grouped radio buttons with customizable appearance,
//! labels, and reactive state integration.

use crate::compat::KeyCode;
use crate::{
  components::Element,
  error::Result,
  events::{ElementAction, KeyCombination},
  layout::LayoutRect,
  reactive::ReactiveState,
  themes::ColorTheme,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Radio button styling options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioStyle {
  /// Character for selected radio button
  pub selected_char: char,
  /// Character for unselected radio button
  pub unselected_char: char,
  /// Spacing between radio button and label
  pub spacing: u16,
  /// Show labels with radio buttons
  pub show_labels: bool,
  /// Orientation of radio group
  pub orientation: RadioOrientation,
}

impl Default for RadioStyle {
  fn default() -> Self {
    Self {
      selected_char: '●',
      unselected_char: '○',
      spacing: 1,
      show_labels: true,
      orientation: RadioOrientation::Vertical,
    }
  }
}

/// Radio button group orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum RadioOrientation {
  Vertical,
  Horizontal,
}

/// Individual radio button option
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioOption {
  /// Unique value for this option
  pub value: String,
  /// Display label
  pub label: String,
  /// Whether this option is enabled
  pub enabled: bool,
  /// Optional description/tooltip
  pub description: Option<String>,
}

impl RadioOption {
  /// Create a new radio option
  pub fn new<V: Into<String>, L: Into<String>>(value: V, label: L) -> Self {
    Self {
      value: value.into(),
      label: label.into(),
      enabled: true,
      description: None,
    }
  }

  /// Set whether this option is enabled
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }

  /// Set description/tooltip
  pub fn description<S: Into<String>>(mut self, description: S) -> Self {
    self.description = Some(description.into());
    self
  }
}

/// Radio button group state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RadioState {
  /// Currently selected value
  pub selected: Option<String>,
  /// Whether the group is interactive
  pub interactive: bool,
  /// Currently focused option index
  pub focused_index: usize,
}

impl Default for RadioState {
  fn default() -> Self {
    Self {
      selected: None,
      interactive: true,
      focused_index: 0,
    }
  }
}

/// Radio button group widget
#[derive(Clone)]
pub struct RadioGroup {
  /// Widget identifier
  pub id: String,
  /// Available options
  pub options: Vec<RadioOption>,
  /// Current state
  pub state: RadioState,
  /// Visual styling
  pub style: RadioStyle,
  /// Reactive state for live updates
  pub reactive_state: Option<Arc<ReactiveState>>,
}

impl RadioGroup {
  /// Create a new radio group
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      options: Vec::new(),
      state: RadioState::default(),
      style: RadioStyle::default(),
      reactive_state: None,
    }
  }

  /// Add an option to the group
  pub fn option<V: Into<String>, L: Into<String>>(mut self, value: V, label: L) -> Self {
    self.options.push(RadioOption::new(value, label));
    self
  }

  /// Add multiple options at once
  pub fn options<I, V, L>(mut self, options: I) -> Self
  where
    I: IntoIterator<Item = (V, L)>,
    V: Into<String>,
    L: Into<String>,
  {
    for (value, label) in options {
      self.options.push(RadioOption::new(value, label));
    }
    self
  }

  /// Set the initially selected value
  pub fn selected<S: Into<String>>(mut self, value: S) -> Self {
    self.state.selected = Some(value.into());
    self.sync_reactive_state();
    self
  }

  /// Set whether the group is interactive
  pub fn interactive(mut self, interactive: bool) -> Self {
    self.state.interactive = interactive;
    self.sync_reactive_state();
    self
  }

  /// Set the orientation
  pub fn orientation(mut self, orientation: RadioOrientation) -> Self {
    self.style.orientation = orientation;
    self
  }

  /// Set custom characters
  pub fn chars(mut self, selected: char, unselected: char) -> Self {
    self.style.selected_char = selected;
    self.style.unselected_char = unselected;
    self
  }

  /// Set spacing between radio and label
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.style.spacing = spacing;
    self
  }

  /// Connect to reactive state for live updates
  pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()> {
    // Initialize reactive fields
    state.set_field(
      &format!("{}.selected", self.id),
      self.state.selected.clone().unwrap_or_default(),
    );
    state.set_field(&format!("{}.interactive", self.id), self.state.interactive);
    state.set_field(
      &format!("{}.focused_index", self.id),
      self.state.focused_index,
    );

    self.reactive_state = Some(state);
    Ok(())
  }

  /// Sync state to reactive state if connected
  fn sync_reactive_state(&self) {
    if let Some(reactive) = &self.reactive_state {
      reactive.set_field(
        &format!("{}.selected", self.id),
        self.state.selected.clone().unwrap_or_default(),
      );
      reactive.set_field(&format!("{}.interactive", self.id), self.state.interactive);
      reactive.set_field(
        &format!("{}.focused_index", self.id),
        self.state.focused_index,
      );
    }
  }

  /// Select an option by value
  pub fn select<S: Into<String>>(&mut self, value: S) -> Result<()> {
    let value = value.into();
    if self.state.interactive
      && self
        .options
        .iter()
        .any(|opt| opt.value == value && opt.enabled)
    {
      self.state.selected = Some(value);
      self.sync_reactive_state();
    }
    Ok(())
  }

  /// Get the currently selected value
  pub fn get_selected(&self) -> Option<&String> {
    self.state.selected.as_ref()
  }

  /// Get the currently selected option
  pub fn selected_option(&self) -> Option<&RadioOption> {
    self
      .state
      .selected
      .as_ref()
      .and_then(|value| self.options.iter().find(|opt| &opt.value == value))
  }

  /// Move focus to next option
  pub fn focus_next(&mut self) -> Result<()> {
    if self.state.interactive && !self.options.is_empty() {
      self.state.focused_index = (self.state.focused_index + 1) % self.options.len();
      self.sync_reactive_state();
    }
    Ok(())
  }

  /// Move focus to previous option
  pub fn focus_previous(&mut self) -> Result<()> {
    if self.state.interactive && !self.options.is_empty() {
      self.state.focused_index = if self.state.focused_index == 0 {
        self.options.len() - 1
      } else {
        self.state.focused_index - 1
      };
      self.sync_reactive_state();
    }
    Ok(())
  }

  /// Select the currently focused option
  pub fn select_focused(&mut self) -> Result<()> {
    if self.state.interactive && self.state.focused_index < self.options.len() {
      let option = &self.options[self.state.focused_index];
      if option.enabled {
        self.state.selected = Some(option.value.clone());
        self.sync_reactive_state();
      }
    }
    Ok(())
  }

  /// Render the radio group as a string
  pub fn render_string(&self) -> String {
    let mut result = String::new();

    for (index, option) in self.options.iter().enumerate() {
      if index > 0 {
        match self.style.orientation {
          RadioOrientation::Vertical => result.push('\n'),
          RadioOrientation::Horizontal => result.push_str("  "),
        }
      }

      let is_selected = self.state.selected.as_ref() == Some(&option.value);
      let radio_char = if is_selected {
        self.style.selected_char
      } else {
        self.style.unselected_char
      };

      let spacing = " ".repeat(self.style.spacing as usize);

      if self.style.show_labels {
        result.push_str(&format!("{}{}{}", radio_char, spacing, option.label));
      } else {
        result.push(radio_char);
      }
    }

    result
  }

  /// Render the radio group with layout and theme support
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    self.render_string()
  }

  /// Convert to Element for integration with the component system
  pub fn to_element(&self) -> Element {
    let content = self.render_string();

    let mut element = Element::with_tag("radiogroup")
      .id(&self.id)
      .content(content)
      .class("radio-group")
      .focusable(self.state.interactive)
      .attr("role", "radiogroup");

    if let Some(selected) = &self.state.selected {
      element = element.attr("aria-activedescendant", selected);
    }

    if !self.state.interactive {
      element = element.class("radio-disabled");
    }

    match self.style.orientation {
      RadioOrientation::Vertical => element = element.class("radio-vertical"),
      RadioOrientation::Horizontal => element = element.class("radio-horizontal"),
    }

    // Add key bindings for interaction
    if self.state.interactive {
      element = element
        .bind_key(
          KeyCombination::new(KeyCode::Up),
          ElementAction::Custom("focus_previous".to_string()),
        )
        .bind_key(
          KeyCombination::new(KeyCode::Down),
          ElementAction::Custom("focus_next".to_string()),
        )
        .bind_key(
          KeyCombination::new(KeyCode::Left),
          ElementAction::Custom("focus_previous".to_string()),
        )
        .bind_key(
          KeyCombination::new(KeyCode::Right),
          ElementAction::Custom("focus_next".to_string()),
        )
        .bind_key(KeyCombination::space(), ElementAction::Activate)
        .bind_key(KeyCombination::enter(), ElementAction::Activate);
    }

    element.build()
  }
}

/// Builder for creating radio groups
pub struct RadioGroupBuilder {
  radio_group: RadioGroup,
}

impl RadioGroupBuilder {
  /// Create a new radio group builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      radio_group: RadioGroup::new(id),
    }
  }

  /// Add an option
  pub fn option<V: Into<String>, L: Into<String>>(mut self, value: V, label: L) -> Self {
    self.radio_group = self.radio_group.option(value, label);
    self
  }

  /// Add multiple options
  pub fn options<I, V, L>(mut self, options: I) -> Self
  where
    I: IntoIterator<Item = (V, L)>,
    V: Into<String>,
    L: Into<String>,
  {
    self.radio_group = self.radio_group.options(options);
    self
  }

  /// Set initially selected value
  pub fn selected<S: Into<String>>(mut self, value: S) -> Self {
    self.radio_group = self.radio_group.selected(value);
    self
  }

  /// Set interactive state
  pub fn interactive(mut self, interactive: bool) -> Self {
    self.radio_group = self.radio_group.interactive(interactive);
    self
  }

  /// Set orientation
  pub fn orientation(mut self, orientation: RadioOrientation) -> Self {
    self.radio_group = self.radio_group.orientation(orientation);
    self
  }

  /// Set custom characters
  pub fn chars(mut self, selected: char, unselected: char) -> Self {
    self.radio_group = self.radio_group.chars(selected, unselected);
    self
  }

  /// Set spacing
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.radio_group = self.radio_group.spacing(spacing);
    self
  }

  /// Build the radio group
  pub fn build(self) -> RadioGroup {
    self.radio_group
  }
}

/// Convenience function for creating a radio group
pub fn radio_group<S: Into<String>>(id: S) -> RadioGroupBuilder {
  RadioGroupBuilder::new(id)
}
