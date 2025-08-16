//! Switch toggle widget for boolean state control
//!
//! Provides an interactive on/off switch with customizable appearance,
//! labels, and reactive state integration.

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

/// Switch toggle state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchState {
  /// Current toggle state (on/off)
  pub enabled: bool,
  /// Whether the switch is interactive
  pub interactive: bool,
  /// Whether the switch is focused
  pub focused: bool,
}

impl Default for SwitchState {
  fn default() -> Self {
    Self {
      enabled: false,
      interactive: true,
      focused: false,
    }
  }
}

/// Switch toggle styling options
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SwitchStyle {
  /// Text to show when switch is ON
  pub on_label: String,
  /// Text to show when switch is OFF
  pub off_label: String,
  /// Character for the switch handle when ON
  pub on_handle: char,
  /// Character for the switch handle when OFF
  pub off_handle: char,
  /// Character for the track background
  pub track_char: char,
  /// Width of the switch track
  pub width: u16,
  /// Show labels inline with switch
  pub show_labels: bool,
  /// Position labels (before/after switch)
  pub label_position: LabelPosition,
}

impl Default for SwitchStyle {
  fn default() -> Self {
    Self {
      on_label: "ON".to_string(),
      off_label: "OFF".to_string(),
      on_handle: '●',
      off_handle: '○',
      track_char: '─',
      width: 8,
      show_labels: true,
      label_position: LabelPosition::After,
    }
  }
}

/// Label positioning for switch
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum LabelPosition {
  Before,
  After,
  Both,
}

/// Switch toggle widget
#[derive(Clone)]
pub struct Switch {
  /// Widget identifier
  pub id: String,
  /// Current state
  pub state: SwitchState,
  /// Visual styling
  pub style: SwitchStyle,
  /// Optional description/tooltip
  pub description: Option<String>,
  /// Reactive state for live updates
  pub reactive_state: Option<Arc<ReactiveState>>,
}

impl Switch {
  /// Create a new switch with default settings
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      state: SwitchState::default(),
      style: SwitchStyle::default(),
      description: None,
      reactive_state: None,
    }
  }

  /// Set the initial enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.state.enabled = enabled;
    self.sync_reactive_state();
    self
  }

  /// Set whether the switch is interactive
  pub fn interactive(mut self, interactive: bool) -> Self {
    self.state.interactive = interactive;
    self.sync_reactive_state();
    self
  }

  /// Set custom labels
  pub fn labels<S1: Into<String>, S2: Into<String>>(mut self, on_label: S1, off_label: S2) -> Self {
    self.style.on_label = on_label.into();
    self.style.off_label = off_label.into();
    self
  }

  /// Set custom handle characters
  pub fn handles(mut self, on_handle: char, off_handle: char) -> Self {
    self.style.on_handle = on_handle;
    self.style.off_handle = off_handle;
    self
  }

  /// Set switch width
  pub fn width(mut self, width: u16) -> Self {
    self.style.width = width;
    self
  }

  /// Set label position
  pub fn label_position(mut self, position: LabelPosition) -> Self {
    self.style.label_position = position;
    self
  }

  /// Set description/tooltip
  pub fn description<S: Into<String>>(mut self, description: S) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Connect to reactive state for live updates
  pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()> {
    // Initialize reactive fields
    state.set_field(&format!("{}.enabled", self.id), self.state.enabled);
    state.set_field(&format!("{}.interactive", self.id), self.state.interactive);
    state.set_field(&format!("{}.focused", self.id), self.state.focused);

    self.reactive_state = Some(state);
    Ok(())
  }

  /// Sync state to reactive state if connected
  fn sync_reactive_state(&self) {
    if let Some(reactive) = &self.reactive_state {
      reactive.set_field(&format!("{}.enabled", self.id), self.state.enabled);
      reactive.set_field(&format!("{}.interactive", self.id), self.state.interactive);
      reactive.set_field(&format!("{}.focused", self.id), self.state.focused);
    }
  }

  /// Toggle the switch state
  pub fn toggle(&mut self) -> Result<()> {
    if self.state.interactive {
      self.state.enabled = !self.state.enabled;
      self.sync_reactive_state();
    }
    Ok(())
  }

  /// Set the switch state explicitly
  pub fn set_enabled(&mut self, enabled: bool) -> Result<()> {
    if self.state.interactive {
      self.state.enabled = enabled;
      self.sync_reactive_state();
    }
    Ok(())
  }

  /// Get current enabled state
  pub fn is_enabled(&self) -> bool {
    self.state.enabled
  }

  /// Get current interactive state
  pub fn is_interactive(&self) -> bool {
    self.state.interactive
  }

  /// Render the switch as a string for display
  pub fn render_string(&self) -> String {
    let handle = if self.state.enabled {
      self.style.on_handle
    } else {
      self.style.off_handle
    };
    let label = if self.state.enabled {
      &self.style.on_label
    } else {
      &self.style.off_label
    };

    // Create the track
    let track_width = self.style.width.saturating_sub(1);
    let handle_pos = if self.state.enabled {
      track_width.saturating_sub(1)
    } else {
      0
    };

    let mut track = String::new();
    for i in 0..track_width {
      if i == handle_pos {
        track.push(handle);
      } else {
        track.push(self.style.track_char);
      }
    }

    // Add brackets
    let switch_display = format!("[{track}]");

    // Add labels if enabled
    if self.style.show_labels {
      match self.style.label_position {
        LabelPosition::Before => format!("{label} {switch_display}"),
        LabelPosition::After => format!("{switch_display} {label}"),
        LabelPosition::Both => {
          let other_label = if self.state.enabled {
            &self.style.off_label
          } else {
            &self.style.on_label
          };
          format!("{other_label} {switch_display} {label}")
        }
      }
    } else {
      switch_display
    }
  }

  /// Render the switch with layout and theme support
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    self.render_string()
  }

  /// Convert to Element for integration with the component system
  pub fn to_element(&self) -> Element {
    let content = self.render_string();

    let mut element = Element::with_tag("switch")
      .id(&self.id)
      .content(content)
      .class("switch")
      .focusable(self.state.interactive)
      .attr("role", "switch")
      .attr("aria-checked", self.state.enabled.to_string());

    if let Some(desc) = &self.description {
      element = element.attr("aria-label", desc);
    }

    if self.state.enabled {
      element = element.class("switch-on");
    } else {
      element = element.class("switch-off");
    }

    if !self.state.interactive {
      element = element.class("switch-disabled");
    }

    if self.state.focused {
      element = element.class("switch-focused");
    }

    // Add key bindings for interaction
    if self.state.interactive {
      element = element
        .bind_key(KeyCombination::space(), ElementAction::Toggle)
        .bind_key(KeyCombination::enter(), ElementAction::Toggle);
    }

    element.build()
  }
}

/// Builder for creating switch widgets
pub struct SwitchBuilder {
  switch: Switch,
}

impl SwitchBuilder {
  /// Create a new switch builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      switch: Switch::new(id),
    }
  }

  /// Set initial enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.switch = self.switch.enabled(enabled);
    self
  }

  /// Set interactive state
  pub fn interactive(mut self, interactive: bool) -> Self {
    self.switch = self.switch.interactive(interactive);
    self
  }

  /// Set custom labels
  pub fn labels<S1: Into<String>, S2: Into<String>>(mut self, on_label: S1, off_label: S2) -> Self {
    self.switch = self.switch.labels(on_label, off_label);
    self
  }

  /// Set custom handle characters
  pub fn handles(mut self, on_handle: char, off_handle: char) -> Self {
    self.switch = self.switch.handles(on_handle, off_handle);
    self
  }

  /// Set switch width
  pub fn width(mut self, width: u16) -> Self {
    self.switch = self.switch.width(width);
    self
  }

  /// Set label position
  pub fn label_position(mut self, position: LabelPosition) -> Self {
    self.switch = self.switch.label_position(position);
    self
  }

  /// Set description
  pub fn description<S: Into<String>>(mut self, description: S) -> Self {
    self.switch = self.switch.description(description);
    self
  }

  /// Build the switch widget
  pub fn build(self) -> Switch {
    self.switch
  }
}

/// Convenience function for creating a switch
pub fn switch<S: Into<String>>(id: S) -> SwitchBuilder {
  SwitchBuilder::new(id)
}

// ResponsiveWidget implementation for Switch
impl crate::widgets::ResponsiveWidget for Switch {
  fn to_element(&self) -> crate::components::Element {
    let mut builder = crate::components::Element::with_tag("input")
      .id(&self.id)
      .attr("type", "checkbox")
      .attr("role", "switch")
      .attr("value", if self.state.enabled { "true" } else { "false" });

    // Add state-based classes and attributes
    if self.state.enabled {
      builder = builder.class("enabled").attr("checked", "true");
    } else {
      builder = builder.class("disabled");
    }

    if !self.state.interactive {
      builder = builder.class("non-interactive").attr("disabled", "true");
    }

    // Add labels as data attributes
    builder = builder.attr("data-on-label", &self.style.on_label);
    builder = builder.attr("data-off-label", &self.style.off_label);

    // Add description if present
    if let Some(_description) = &self.description {
      builder = builder.attr("aria-describedby", &format!("{}-desc", self.id));
    }

    // Add label position class
    match self.style.label_position {
      LabelPosition::Before => builder = builder.class("label-before"),
      LabelPosition::After => builder = builder.class("label-after"),
      LabelPosition::Both => builder = builder.class("label-both"),
    }

    // Add width as style attribute
    builder = builder.attr("data-width", &self.style.width.to_string());

    // Set focusable if interactive
    if self.state.interactive {
      builder = builder.focusable(true);
    }

    builder.build()
  }

  fn render_with_layout(&self, layout: &crate::layout::LayoutRect, theme: Option<&crate::themes::ColorTheme>) -> String {
    // Use the existing render method
    self.render(layout, theme)
  }

  fn min_size(&self) -> (u16, u16) {
    let switch_width = self.style.width;

    // Calculate label widths
    let on_label_width = self.style.on_label.chars().count() as u16;
    let off_label_width = self.style.off_label.chars().count() as u16;
    let max_label_width = on_label_width.max(off_label_width);

    let (total_width, total_height) = match self.style.label_position {
      LabelPosition::Before | LabelPosition::After => {
        let width = switch_width + if max_label_width > 0 { 1 + max_label_width } else { 0 };
        (width, 1)
      }
      LabelPosition::Both => {
        let width = switch_width + on_label_width + off_label_width + 2; // spaces around switch
        (width, 1)
      }
    };

    // Add space for description if present
    let final_height = if self.description.is_some() {
      total_height + 1
    } else {
      total_height
    };

    (total_width.max(1), final_height)
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    // Switches have a natural maximum size based on their content
    let (min_width, min_height) = self.min_size();
    (Some(min_width), Some(min_height))
  }

  fn can_grow_horizontal(&self) -> bool {
    false // Switches have a fixed size based on their content
  }

  fn can_grow_vertical(&self) -> bool {
    false // Switches have a fixed height
  }
}
