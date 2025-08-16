//! Checkbox Widget Component - Single and Group Checkboxes
//!
//! Provides both individual checkbox controls and checkbox groups for multi-selection forms.
//! Supports various checkbox styles, label positioning, and accessibility features.

use crate::{components::Element, error::Result};
use serde::{Deserialize, Serialize};

/// Checkbox visual style
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckboxStyle {
  /// Standard ballot box style: ☐ ☑
  Ballot,
  /// Square brackets style: \[ \] \[x\]
  Square,
  /// Parentheses style: ( ) (x)
  Round,
  /// Custom characters
  Custom { unchecked: String, checked: String },
}

impl Default for CheckboxStyle {
  fn default() -> Self {
    Self::Ballot
  }
}

/// Label positioning relative to checkbox
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckboxLabelPosition {
  Before,
  After,
  Above,
  Below,
  None,
}

impl Default for CheckboxLabelPosition {
  fn default() -> Self {
    Self::After
  }
}

/// Checkbox state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxState {
  pub checked: bool,
  pub enabled: bool,
  pub focused: bool,
  pub visible: bool,
  pub animation_state: CheckboxAnimationState,
  pub animation_progress: f32,          // 0.0 to 1.0
  pub last_animation_time: Option<u64>, // Timestamp in milliseconds
}

impl Default for CheckboxState {
  fn default() -> Self {
    Self {
      checked: false,
      enabled: true,
      focused: false,
      visible: true,
      animation_state: CheckboxAnimationState::default(),
      animation_progress: 0.0,
      last_animation_time: None,
    }
  }
}

/// Animation state for checkbox transitions
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckboxAnimationState {
  Idle,
  CheckingIn,  // Animating from large to normal when checking
  CheckingOut, // Animating when unchecking (optional)
}

impl Default for CheckboxAnimationState {
  fn default() -> Self {
    Self::Idle
  }
}

/// Animation configuration for checkboxes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxAnimationConfig {
  pub enabled: bool,
  pub duration_ms: u64,
  pub scale_factor: f32, // How much larger the initial checkmark is (1.5 = 50% larger)
}

impl Default for CheckboxAnimationConfig {
  fn default() -> Self {
    Self {
      enabled: true,
      duration_ms: 250,
      scale_factor: 1.5,
    }
  }
}

/// Single checkbox widget
pub struct Checkbox {
  pub id: String,
  pub label: Option<String>,
  pub style: CheckboxStyle,
  pub label_position: CheckboxLabelPosition,
  pub spacing: u16,
  pub state: CheckboxState,
  pub animation_config: CheckboxAnimationConfig,
}

impl Checkbox {
  /// Create a new checkbox
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      label: None,
      style: CheckboxStyle::default(),
      label_position: CheckboxLabelPosition::default(),
      spacing: 1,
      state: CheckboxState::default(),
      animation_config: CheckboxAnimationConfig::default(),
    }
  }

  /// Set label
  pub fn label<S: Into<String>>(mut self, label: S) -> Self {
    self.label = Some(label.into());
    self
  }

  /// Set checkbox style
  pub fn style(mut self, style: CheckboxStyle) -> Self {
    self.style = style;
    self
  }

  /// Set label position
  pub fn label_position(mut self, position: CheckboxLabelPosition) -> Self {
    self.label_position = position;
    self
  }

  /// Set spacing between checkbox and label
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.spacing = spacing;
    self
  }

  /// Set checked state
  pub fn checked(mut self, checked: bool) -> Self {
    self.state.checked = checked;
    self
  }

  /// Set enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.state.enabled = enabled;
    self
  }

  /// Set visibility
  pub fn visible(mut self, visible: bool) -> Self {
    self.state.visible = visible;
    self
  }

  /// Set animation configuration
  pub fn animation_config(mut self, config: CheckboxAnimationConfig) -> Self {
    self.animation_config = config;
    self
  }

  /// Enable/disable animations
  pub fn animated(mut self, enabled: bool) -> Self {
    self.animation_config.enabled = enabled;
    self
  }

  /// Toggle checked state
  pub fn toggle(&mut self) -> Result<()> {
    if self.state.enabled {
      let was_checked = self.state.checked;
      self.state.checked = !self.state.checked;

      // Start animation if enabled
      if self.animation_config.enabled && !was_checked && self.state.checked {
        self.start_check_animation();
      }
    }
    Ok(())
  }

  /// Check the checkbox
  pub fn check(&mut self) -> Result<()> {
    if self.state.enabled && !self.state.checked {
      self.state.checked = true;

      // Start animation if enabled
      if self.animation_config.enabled {
        self.start_check_animation();
      }
    }
    Ok(())
  }

  /// Uncheck the checkbox
  pub fn uncheck(&mut self) -> Result<()> {
    if self.state.enabled {
      self.state.checked = false;
      self.state.animation_state = CheckboxAnimationState::Idle;
    }
    Ok(())
  }

  /// Get checked state
  pub fn is_checked(&self) -> bool {
    self.state.checked
  }

  /// Get enabled state
  pub fn is_enabled(&self) -> bool {
    self.state.enabled
  }

  /// Get visibility
  pub fn is_visible(&self) -> bool {
    self.state.visible
  }

  /// Get checkbox character based on style and state (with animation support)
  pub fn checkbox_char(&self) -> String {
    // Use animated character if animation is active
    if self.animation_config.enabled && self.state.animation_state != CheckboxAnimationState::Idle {
      return self.get_animated_checkbox_char();
    }

    // Standard static character
    match &self.style {
      CheckboxStyle::Ballot => {
        if self.state.checked {
          "☑".to_string()
        } else {
          "☐".to_string()
        }
      }
      CheckboxStyle::Square => {
        if self.state.checked {
          "[x]".to_string()
        } else {
          "[ ]".to_string()
        }
      }
      CheckboxStyle::Round => {
        if self.state.checked {
          "(x)".to_string()
        } else {
          "( )".to_string()
        }
      }
      CheckboxStyle::Custom { unchecked, checked } => {
        if self.state.checked {
          checked.clone()
        } else {
          unchecked.clone()
        }
      }
    }
  }

  /// Get animated checkbox character based on animation progress
  fn get_animated_checkbox_char(&self) -> String {
    if !self.state.checked {
      return self.get_unchecked_char();
    }

    match &self.style {
      CheckboxStyle::Ballot => {
        match self.state.animation_state {
          CheckboxAnimationState::CheckingIn => {
            // Progress from large to normal
            if self.state.animation_progress < 0.3 {
              "✅".to_string() // Large heavy check mark
            } else {
              "☑".to_string() // Medium and final normal size
            }
          }
          _ => "☑".to_string(),
        }
      }
      CheckboxStyle::Square => {
        match self.state.animation_state {
          CheckboxAnimationState::CheckingIn => {
            if self.state.animation_progress < 0.4 {
              "[✓]".to_string() // Large check in square
            } else {
              "[x]".to_string() // Normal x in square
            }
          }
          _ => "[x]".to_string(),
        }
      }
      CheckboxStyle::Round => {
        match self.state.animation_state {
          CheckboxAnimationState::CheckingIn => {
            if self.state.animation_progress < 0.4 {
              "(✓)".to_string() // Large check in parentheses
            } else {
              "(x)".to_string() // Normal x in parentheses
            }
          }
          _ => "(x)".to_string(),
        }
      }
      CheckboxStyle::Custom { checked, .. } => {
        // For custom styles, just use the checked character
        checked.clone()
      }
    }
  }

  /// Get unchecked character for current style
  fn get_unchecked_char(&self) -> String {
    match &self.style {
      CheckboxStyle::Ballot => "☐".to_string(),
      CheckboxStyle::Square => "[ ]".to_string(),
      CheckboxStyle::Round => "( )".to_string(),
      CheckboxStyle::Custom { unchecked, .. } => unchecked.clone(),
    }
  }

  /// Start check animation
  fn start_check_animation(&mut self) {
    self.state.animation_state = CheckboxAnimationState::CheckingIn;
    self.state.animation_progress = 0.0;
    self.state.last_animation_time = Some(self.get_current_time_ms());
  }

  /// Update animation progress
  pub fn update_animation(&mut self) -> Result<()> {
    if !self.animation_config.enabled || self.state.animation_state == CheckboxAnimationState::Idle
    {
      return Ok(());
    }

    let current_time = self.get_current_time_ms();
    if let Some(start_time) = self.state.last_animation_time {
      let elapsed = current_time - start_time;
      let progress = (elapsed as f32) / (self.animation_config.duration_ms as f32);

      if progress >= 1.0 {
        // Animation complete
        self.state.animation_state = CheckboxAnimationState::Idle;
        self.state.animation_progress = 1.0;
      } else {
        // Update progress with easing
        self.state.animation_progress = self.ease_out_cubic(progress);
      }
    }

    Ok(())
  }

  /// Easing function for smooth animation
  fn ease_out_cubic(&self, t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
  }

  /// Get current time in milliseconds
  fn get_current_time_ms(&self) -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now()
      .duration_since(UNIX_EPOCH)
      .unwrap_or_default()
      .as_millis() as u64
  }

  /// Render checkbox as string
  pub fn render_string(&self) -> String {
    if !self.state.visible {
      return String::new();
    }

    let checkbox = self.checkbox_char();
    let spacing = " ".repeat(self.spacing as usize);

    match self.label_position {
      CheckboxLabelPosition::Before => {
        if let Some(label) = &self.label {
          format!("{label}{spacing}{checkbox}")
        } else {
          checkbox
        }
      }
      CheckboxLabelPosition::After => {
        if let Some(label) = &self.label {
          format!("{checkbox}{spacing}{label}")
        } else {
          checkbox
        }
      }
      CheckboxLabelPosition::Above => {
        if let Some(label) = &self.label {
          format!("{label}\n{checkbox}")
        } else {
          checkbox
        }
      }
      CheckboxLabelPosition::Below => {
        if let Some(label) = &self.label {
          format!("{checkbox}\n{label}")
        } else {
          checkbox
        }
      }
      CheckboxLabelPosition::None => checkbox,
    }
  }

  /// Convert to Element for integration with the component system
  pub fn to_element(&self) -> Element {
    let content = self.render_string();

    let mut element = Element::with_tag("checkbox")
      .id(&self.id)
      .content(content)
      .class("checkbox")
      .attr("role", "checkbox")
      .attr("aria-checked", self.state.checked.to_string());

    // Add state attributes
    element = element
      .attr("data-checked", self.state.checked.to_string())
      .attr("data-enabled", self.state.enabled.to_string())
      .attr("data-visible", self.state.visible.to_string());

    // Add accessibility label
    if let Some(label) = &self.label {
      element = element.attr("aria-label", label);
    }

    // Add CSS classes based on state
    if self.state.checked {
      element = element.class("checkbox-checked");
    } else {
      element = element.class("checkbox-unchecked");
    }

    if !self.state.enabled {
      element = element.class("checkbox-disabled");
    }

    if self.state.focused {
      element = element.class("checkbox-focused");
    }

    if !self.state.visible {
      element = element.class("checkbox-hidden");
    }

    element.build()
  }
}

/// Checkbox group orientation
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum CheckboxGroupOrientation {
  Vertical,
  Horizontal,
}

impl Default for CheckboxGroupOrientation {
  fn default() -> Self {
    Self::Vertical
  }
}

/// Checkbox option in a group
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxOption {
  pub id: String,
  pub label: String,
  pub value: String,
  pub checked: bool,
  pub enabled: bool,
}

impl CheckboxOption {
  /// Create a new checkbox option
  pub fn new<S: Into<String>>(id: S, label: S, value: S) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      value: value.into(),
      checked: false,
      enabled: true,
    }
  }

  /// Set checked state
  pub fn checked(mut self, checked: bool) -> Self {
    self.checked = checked;
    self
  }

  /// Set enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }
}

/// Checkbox group state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CheckboxGroupState {
  pub selected_values: Vec<String>,
  pub enabled: bool,
  pub visible: bool,
  pub focused_index: Option<usize>,
}

impl Default for CheckboxGroupState {
  fn default() -> Self {
    Self {
      selected_values: Vec::new(),
      enabled: true,
      visible: true,
      focused_index: None,
    }
  }
}

/// Checkbox group widget for multi-selection
pub struct CheckboxGroup {
  pub id: String,
  pub label: Option<String>,
  pub options: Vec<CheckboxOption>,
  pub style: CheckboxStyle,
  pub orientation: CheckboxGroupOrientation,
  pub spacing: u16,
  pub state: CheckboxGroupState,
}

impl CheckboxGroup {
  /// Create a new checkbox group
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      label: None,
      options: Vec::new(),
      style: CheckboxStyle::default(),
      orientation: CheckboxGroupOrientation::default(),
      spacing: 1,
      state: CheckboxGroupState::default(),
    }
  }

  /// Set group label
  pub fn label<S: Into<String>>(mut self, label: S) -> Self {
    self.label = Some(label.into());
    self
  }

  /// Add an option
  pub fn option(mut self, option: CheckboxOption) -> Self {
    self.options.push(option);
    self.sync_state();
    self
  }

  /// Add multiple options
  pub fn options(mut self, options: Vec<CheckboxOption>) -> Self {
    self.options.extend(options);
    self.sync_state();
    self
  }

  /// Set checkbox style
  pub fn style(mut self, style: CheckboxStyle) -> Self {
    self.style = style;
    self
  }

  /// Set orientation
  pub fn orientation(mut self, orientation: CheckboxGroupOrientation) -> Self {
    self.orientation = orientation;
    self
  }

  /// Set spacing
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.spacing = spacing;
    self
  }

  /// Set enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.state.enabled = enabled;
    self
  }

  /// Set visibility
  pub fn visible(mut self, visible: bool) -> Self {
    self.state.visible = visible;
    self
  }

  /// Toggle option by value
  pub fn toggle_option(&mut self, value: &str) -> Result<()> {
    if !self.state.enabled {
      return Ok(());
    }

    if let Some(option) = self.options.iter_mut().find(|opt| opt.value == value) {
      if option.enabled {
        option.checked = !option.checked;
        self.sync_state();
      }
    }
    Ok(())
  }

  /// Check option by value
  pub fn check_option(&mut self, value: &str) -> Result<()> {
    if !self.state.enabled {
      return Ok(());
    }

    if let Some(option) = self.options.iter_mut().find(|opt| opt.value == value) {
      if option.enabled {
        option.checked = true;
        self.sync_state();
      }
    }
    Ok(())
  }

  /// Uncheck option by value
  pub fn uncheck_option(&mut self, value: &str) -> Result<()> {
    if !self.state.enabled {
      return Ok(());
    }

    if let Some(option) = self.options.iter_mut().find(|opt| opt.value == value) {
      if option.enabled {
        option.checked = false;
        self.sync_state();
      }
    }
    Ok(())
  }

  /// Check multiple options
  pub fn check_options(&mut self, values: &[&str]) -> Result<()> {
    for value in values {
      self.check_option(value)?;
    }
    Ok(())
  }

  /// Uncheck all options
  pub fn uncheck_all(&mut self) -> Result<()> {
    if !self.state.enabled {
      return Ok(());
    }

    for option in &mut self.options {
      if option.enabled {
        option.checked = false;
      }
    }
    self.sync_state();
    Ok(())
  }

  /// Check all options
  pub fn check_all(&mut self) -> Result<()> {
    if !self.state.enabled {
      return Ok(());
    }

    for option in &mut self.options {
      if option.enabled {
        option.checked = true;
      }
    }
    self.sync_state();
    Ok(())
  }

  /// Get selected values
  pub fn selected_values(&self) -> Vec<String> {
    self.state.selected_values.clone()
  }

  /// Get checked options
  pub fn checked_options(&self) -> Vec<&CheckboxOption> {
    self.options.iter().filter(|opt| opt.checked).collect()
  }

  /// Check if option is selected
  pub fn is_selected(&self, value: &str) -> bool {
    self.state.selected_values.contains(&value.to_string())
  }

  /// Get checkbox character for option
  fn checkbox_char_for_option(&self, option: &CheckboxOption) -> String {
    match &self.style {
      CheckboxStyle::Ballot => {
        if option.checked {
          "☑".to_string()
        } else {
          "☐".to_string()
        }
      }
      CheckboxStyle::Square => {
        if option.checked {
          "[x]".to_string()
        } else {
          "[ ]".to_string()
        }
      }
      CheckboxStyle::Round => {
        if option.checked {
          "(x)".to_string()
        } else {
          "( )".to_string()
        }
      }
      CheckboxStyle::Custom { unchecked, checked } => {
        if option.checked {
          checked.clone()
        } else {
          unchecked.clone()
        }
      }
    }
  }

  /// Render checkbox group as string
  pub fn render_string(&self) -> String {
    if !self.state.visible {
      return String::new();
    }

    let mut result = String::new();

    // Add group label if present
    if let Some(label) = &self.label {
      result.push_str(label);
      result.push('\n');
    }

    let spacing = " ".repeat(self.spacing as usize);
    let separator = match self.orientation {
      CheckboxGroupOrientation::Vertical => "\n",
      CheckboxGroupOrientation::Horizontal => &spacing.to_string(),
    };

    let option_strings: Vec<String> = self
      .options
      .iter()
      .map(|option| {
        let checkbox = self.checkbox_char_for_option(option);
        format!("{}{}{}", checkbox, spacing, option.label)
      })
      .collect();

    result.push_str(&option_strings.join(separator));
    result
  }

  /// Convert to Element for integration with the component system
  pub fn to_element(&self) -> Element {
    let content = self.render_string();

    let mut element = Element::with_tag("checkbox-group")
      .id(&self.id)
      .content(content)
      .class("checkbox-group")
      .attr("role", "group");

    // Add state attributes
    element = element
      .attr(
        "data-selected-count",
        self.state.selected_values.len().to_string(),
      )
      .attr("data-total-options", self.options.len().to_string())
      .attr("data-enabled", self.state.enabled.to_string())
      .attr("data-visible", self.state.visible.to_string())
      .attr(
        "data-orientation",
        format!("{:?}", self.orientation).to_lowercase(),
      );

    // Add accessibility label
    if let Some(label) = &self.label {
      element = element.attr("aria-label", label);
    }

    // Add selected values as data attribute
    let selected_values = self.state.selected_values.join(",");
    element = element.attr("data-selected-values", selected_values);

    // Add CSS classes based on state
    if !self.state.enabled {
      element = element.class("checkbox-group-disabled");
    }

    if !self.state.visible {
      element = element.class("checkbox-group-hidden");
    }

    match self.orientation {
      CheckboxGroupOrientation::Vertical => element = element.class("checkbox-group-vertical"),
      CheckboxGroupOrientation::Horizontal => element = element.class("checkbox-group-horizontal"),
    }

    element.build()
  }

  /// Sync internal state with options
  fn sync_state(&mut self) {
    self.state.selected_values = self
      .options
      .iter()
      .filter(|opt| opt.checked)
      .map(|opt| opt.value.clone())
      .collect();
  }
}

/// Builder for creating checkboxes
pub struct CheckboxBuilder {
  checkbox: Checkbox,
}

impl CheckboxBuilder {
  /// Create a new checkbox builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      checkbox: Checkbox::new(id),
    }
  }

  /// Set label
  pub fn label<S: Into<String>>(mut self, label: S) -> Self {
    self.checkbox = self.checkbox.label(label);
    self
  }

  /// Set style
  pub fn style(mut self, style: CheckboxStyle) -> Self {
    self.checkbox = self.checkbox.style(style);
    self
  }

  /// Set label position
  pub fn label_position(mut self, position: CheckboxLabelPosition) -> Self {
    self.checkbox = self.checkbox.label_position(position);
    self
  }

  /// Set spacing
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.checkbox = self.checkbox.spacing(spacing);
    self
  }

  /// Set checked state
  pub fn checked(mut self, checked: bool) -> Self {
    self.checkbox = self.checkbox.checked(checked);
    self
  }

  /// Set enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.checkbox = self.checkbox.enabled(enabled);
    self
  }

  /// Build the checkbox
  pub fn build(self) -> Checkbox {
    self.checkbox
  }
}

/// Builder for creating checkbox groups
pub struct CheckboxGroupBuilder {
  group: CheckboxGroup,
}

impl CheckboxGroupBuilder {
  /// Create a new checkbox group builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      group: CheckboxGroup::new(id),
    }
  }

  /// Set group label
  pub fn label<S: Into<String>>(mut self, label: S) -> Self {
    self.group = self.group.label(label);
    self
  }

  /// Add an option
  pub fn option(mut self, option: CheckboxOption) -> Self {
    self.group = self.group.option(option);
    self
  }

  /// Add multiple options
  pub fn options(mut self, options: Vec<CheckboxOption>) -> Self {
    self.group = self.group.options(options);
    self
  }

  /// Set style
  pub fn style(mut self, style: CheckboxStyle) -> Self {
    self.group = self.group.style(style);
    self
  }

  /// Set orientation
  pub fn orientation(mut self, orientation: CheckboxGroupOrientation) -> Self {
    self.group = self.group.orientation(orientation);
    self
  }

  /// Set spacing
  pub fn spacing(mut self, spacing: u16) -> Self {
    self.group = self.group.spacing(spacing);
    self
  }

  /// Set enabled state
  pub fn enabled(mut self, enabled: bool) -> Self {
    self.group = self.group.enabled(enabled);
    self
  }

  /// Build the checkbox group
  pub fn build(self) -> CheckboxGroup {
    self.group
  }
}

/// Convenience function for creating a checkbox
pub fn checkbox<S: Into<String>>(id: S) -> CheckboxBuilder {
  CheckboxBuilder::new(id)
}

/// Convenience function for creating a checkbox group
pub fn checkbox_group<S: Into<String>>(id: S) -> CheckboxGroupBuilder {
  CheckboxGroupBuilder::new(id)
}

/// Create a simple checkbox with label
pub fn simple_checkbox<S: Into<String>>(id: S, label: S) -> Checkbox {
  Checkbox::new(id).label(label)
}

/// Create a checkbox group with options
pub fn simple_checkbox_group<S: Into<String>>(
  id: S,
  label: S,
  options: Vec<(&str, &str)>,
) -> CheckboxGroup {
  let id_string = id.into();
  let checkbox_options: Vec<CheckboxOption> = options
    .into_iter()
    .enumerate()
    .map(|(i, (label, value))| {
      CheckboxOption::new(
        format!("{id_string}-{i}"),
        label.to_string(),
        value.to_string(),
      )
    })
    .collect();

  CheckboxGroup::new(id_string)
    .label(label)
    .options(checkbox_options)
}

/// Create a horizontal checkbox group
pub fn horizontal_checkbox_group<S: Into<String>>(
  id: S,
  label: S,
  options: Vec<(&str, &str)>,
) -> CheckboxGroup {
  simple_checkbox_group(id, label, options).orientation(CheckboxGroupOrientation::Horizontal)
}

/// Create a custom styled checkbox
pub fn custom_checkbox<S: Into<String>>(id: S, label: S, unchecked: S, checked: S) -> Checkbox {
  Checkbox::new(id).label(label).style(CheckboxStyle::Custom {
    unchecked: unchecked.into(),
    checked: checked.into(),
  })
}

// ResponsiveWidget implementation for Checkbox
impl crate::widgets::ResponsiveWidget for Checkbox {
  fn to_element(&self) -> crate::components::Element {
    let mut builder = crate::components::Element::with_tag("input")
      .id(&self.id)
      .attr("type", "checkbox")
      .attr("value", if self.state.checked { "true" } else { "false" });

    // Add label if present
    if let Some(label) = &self.label {
      builder = builder.attr("label", label);
    }

    // Add state-based classes and attributes
    if self.state.checked {
      builder = builder.class("checked").attr("checked", "true");
    }
    if !self.state.enabled {
      builder = builder.class("disabled").attr("disabled", "true");
    }
    if self.state.focused {
      builder = builder.class("focused");
    }
    if !self.state.visible {
      builder = builder.class("hidden");
    }

    // Add animation state classes
    match self.state.animation_state {
      CheckboxAnimationState::CheckingIn => builder = builder.class("animating-in"),
      CheckboxAnimationState::CheckingOut => builder = builder.class("animating-out"),
      CheckboxAnimationState::Idle => {}
    }

    // Add style-based classes
    match &self.style {
      CheckboxStyle::Ballot => builder = builder.class("style-ballot"),
      CheckboxStyle::Square => builder = builder.class("style-square"),
      CheckboxStyle::Round => builder = builder.class("style-round"),
      CheckboxStyle::Custom { .. } => builder = builder.class("style-custom"),
    }

    // Add label position class
    match self.label_position {
      CheckboxLabelPosition::Before => builder = builder.class("label-before"),
      CheckboxLabelPosition::After => builder = builder.class("label-after"),
      CheckboxLabelPosition::Above => builder = builder.class("label-above"),
      CheckboxLabelPosition::Below => builder = builder.class("label-below"),
      CheckboxLabelPosition::None => builder = builder.class("label-none"),
    }

    // Set focusable if enabled
    if self.state.enabled {
      builder = builder.focusable(true);
    }

    builder.build()
  }

  fn render_with_layout(&self, _layout: &crate::layout::LayoutRect, _theme: Option<&crate::themes::ColorTheme>) -> String {
    // Use the existing render_string method and position it within the layout
    if !self.state.visible {
      return String::new();
    }

    let content = self.render_string();

    // For now, just return the content - in a full implementation, you'd position it within the layout bounds
    // and handle text wrapping, alignment, etc.
    content
  }

  fn min_size(&self) -> (u16, u16) {
    if !self.state.visible {
      return (0, 0);
    }

    let checkbox_width = match &self.style {
      CheckboxStyle::Ballot => 1,
      CheckboxStyle::Square => 3, // [x]
      CheckboxStyle::Round => 3,  // (x)
      CheckboxStyle::Custom { unchecked, checked } => {
        unchecked.chars().count().max(checked.chars().count()) as u16
      }
    };

    let label_width = self.label.as_ref().map(|l| l.chars().count() as u16).unwrap_or(0);
    let spacing_width = if self.label.is_some() && self.label_position != CheckboxLabelPosition::None {
      self.spacing
    } else {
      0
    };

    let total_width = checkbox_width + spacing_width + label_width;

    let height = match self.label_position {
      CheckboxLabelPosition::Above | CheckboxLabelPosition::Below => {
        if self.label.is_some() { 2 } else { 1 }
      }
      _ => 1,
    };

    (total_width.max(1), height)
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    // Checkboxes have a natural maximum size based on their content
    let (min_width, min_height) = self.min_size();
    (Some(min_width), Some(min_height))
  }

  fn can_grow_horizontal(&self) -> bool {
    false // Checkboxes have a fixed size based on their content
  }

  fn can_grow_vertical(&self) -> bool {
    false // Checkboxes have a fixed height
  }
}

// ResponsiveWidget implementation for CheckboxGroup
impl crate::widgets::ResponsiveWidget for CheckboxGroup {
  fn to_element(&self) -> crate::components::Element {
    let mut builder = crate::components::Element::with_tag("fieldset")
      .id(&self.id)
      .class("checkbox-group");

    // Add label as legend if present
    if let Some(label) = &self.label {
      builder = builder.child(
        crate::components::Element::with_tag("legend")
          .content(label)
          .build()
      );
    }

    // Add orientation class
    match self.orientation {
      CheckboxGroupOrientation::Vertical => builder = builder.class("vertical"),
      CheckboxGroupOrientation::Horizontal => builder = builder.class("horizontal"),
    }

    // Add state classes
    if !self.state.enabled {
      builder = builder.class("disabled");
    }
    if !self.state.visible {
      builder = builder.class("hidden");
    }

    // Add individual checkboxes as children
    for (index, option) in self.options.iter().enumerate() {
      let is_selected = self.state.selected_values.contains(&option.value);
      let is_focused = self.state.focused_index == Some(index);

      let checkbox_element = crate::components::Element::with_tag("input")
        .attr("type", "checkbox")
        .attr("name", &self.id)
        .attr("value", &option.value)
        .attr("id", &option.id)
        .class("checkbox-group-item")
        .focusable(option.enabled && self.state.enabled);

      let checkbox_element = if is_selected {
        checkbox_element.attr("checked", "true").class("checked")
      } else {
        checkbox_element
      };

      let checkbox_element = if is_focused {
        checkbox_element.class("focused")
      } else {
        checkbox_element
      };

      let checkbox_element = if !option.enabled || !self.state.enabled {
        checkbox_element.attr("disabled", "true").class("disabled")
      } else {
        checkbox_element
      };

      builder = builder.child(checkbox_element.build());
    }

    builder.build()
  }

  fn render_with_layout(&self, _layout: &crate::layout::LayoutRect, _theme: Option<&crate::themes::ColorTheme>) -> String {
    // Use the existing render_string method
    if !self.state.visible {
      return String::new();
    }

    self.render_string()
  }

  fn min_size(&self) -> (u16, u16) {
    if !self.state.visible || self.options.is_empty() {
      return (0, 0);
    }

    let label_height = if self.label.is_some() { 1 } else { 0 };

    match self.orientation {
      CheckboxGroupOrientation::Vertical => {
        let max_width = self.options.iter()
          .map(|opt| {
            let checkbox_width = 3; // Assume square style [x]
            let label_width = opt.label.chars().count() as u16;
            checkbox_width + 1 + label_width // checkbox + space + label
          })
          .max()
          .unwrap_or(0);

        let height = label_height + self.options.len() as u16;
        (max_width, height)
      }
      CheckboxGroupOrientation::Horizontal => {
        let total_width = self.options.iter()
          .map(|opt| {
            let checkbox_width = 3; // Assume square style [x]
            let label_width = opt.label.chars().count() as u16;
            checkbox_width + 1 + label_width // checkbox + space + label
          })
          .sum::<u16>() + (self.options.len().saturating_sub(1) as u16 * 2); // spacing between items

        let height = label_height + 1; // One row for checkboxes
        (total_width, height)
      }
    }
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    // Checkbox groups have a natural maximum size based on their content
    let (min_width, min_height) = self.min_size();
    (Some(min_width), Some(min_height))
  }

  fn can_grow_horizontal(&self) -> bool {
    false // Checkbox groups have a fixed size based on their content
  }

  fn can_grow_vertical(&self) -> bool {
    false // Checkbox groups have a fixed height
  }
}
