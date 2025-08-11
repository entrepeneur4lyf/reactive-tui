//! Accordion Widget
//!
//! A comprehensive accordion widget supporting expandable/collapsible sections,
//! with keyboard navigation, custom styling, and animation support.
//!
//! # Features
//!
//! - **Multiple Sections**: Support for multiple collapsible sections in a single accordion
//! - **Expand/Collapse**: Individual section expand/collapse with smooth animations
//! - **Multi-Expand Mode**: Allow multiple sections to be open simultaneously or single-expand mode
//! - **Keyboard Navigation**: Arrow keys, Enter/Space, Home/End navigation support
//! - **Custom Headers**: Customizable section headers with icons, badges, and styling
//! - **Rich Content**: Support for any content type in accordion sections
//! - **Animation Support**: Smooth expand/collapse animations with easing
//! - **Accessibility**: Full ARIA support and screen reader compatibility
//! - **Event Callbacks**: onExpand, onCollapse, onChange event handling
//! - **Themeable**: CSS utility classes and custom styling support
//!
//! # Basic Usage
//!
//! ```rust
//! use reactive_tui::widgets::{Accordion, AccordionBuilder, AccordionSection};
//!
//! let mut accordion = AccordionBuilder::new("settings-accordion")
//!     .section(AccordionSection::new("general", "General Settings")
//!         .content("General application settings go here...")
//!         .expanded(true))
//!     .section(AccordionSection::new("privacy", "Privacy & Security")
//!         .content("Privacy and security options...")
//!         .icon("🔒"))
//!     .section(AccordionSection::new("advanced", "Advanced")
//!         .content("Advanced configuration options..."))
//!     .multi_expand(true)
//!     .animated(true)
//!     .build();
//!
//! // Toggle section
//! accordion.toggle_section("privacy");
//!
//! // Expand specific section
//! accordion.expand_section("advanced");
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
type OnExpandCallback = Arc<dyn Fn(&SectionId) + Send + Sync>;
type OnCollapseCallback = Arc<dyn Fn(&SectionId) + Send + Sync>;
type OnChangeCallback = Arc<dyn Fn(&[SectionId]) + Send + Sync>;
type OnFocusCallback = Arc<dyn Fn(&SectionId) + Send + Sync>;
type OnSectionClickCallback = Arc<dyn Fn(&SectionId) + Send + Sync>;

/// Unique identifier for accordion sections
pub type SectionId = String;

/// Animation configuration for accordion sections
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccordionAnimation {
  /// Enable animation
  pub enabled: bool,
  /// Animation duration in milliseconds
  pub duration: u32,
  /// Animation easing function
  pub easing: AnimationEasing,
  /// Stagger delay between multiple sections (ms)
  pub stagger_delay: u32,
}

impl Default for AccordionAnimation {
  fn default() -> Self {
    Self {
      enabled: true,
      duration: 300,
      easing: AnimationEasing::EaseInOut,
      stagger_delay: 50,
    }
  }
}

/// Animation easing functions
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AnimationEasing {
  Linear,
  EaseIn,
  EaseOut,
  EaseInOut,
  EaseInBack,
  EaseOutBack,
  EaseInOutBack,
}

impl Default for AnimationEasing {
  fn default() -> Self {
    Self::EaseInOut
  }
}

/// Individual accordion section
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AccordionSection {
  /// Unique section identifier
  pub id: SectionId,
  /// Section header title
  pub title: String,
  /// Optional section description/subtitle
  pub description: Option<String>,
  /// Section content (can be text or Element)
  pub content: String,
  /// Whether section is expanded
  pub expanded: bool,
  /// Whether section is disabled
  pub disabled: bool,
  /// Optional header icon
  pub icon: Option<String>,
  /// Optional badge text
  pub badge: Option<String>,
  /// Custom CSS classes for the section
  pub css_classes: Vec<String>,
  /// Section-specific styling
  pub style: AccordionSectionStyle,
}

impl AccordionSection {
  /// Create a new accordion section
  pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      title: title.into(),
      description: None,
      content: String::new(),
      expanded: false,
      disabled: false,
      icon: None,
      badge: None,
      css_classes: Vec::new(),
      style: AccordionSectionStyle::default(),
    }
  }

  /// Set section content
  pub fn content(mut self, content: impl Into<String>) -> Self {
    self.content = content.into();
    self
  }

  /// Set section description
  pub fn description(mut self, description: impl Into<String>) -> Self {
    self.description = Some(description.into());
    self
  }

  /// Set expanded state
  pub fn expanded(mut self, expanded: bool) -> Self {
    self.expanded = expanded;
    self
  }

  /// Set disabled state
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Set header icon
  pub fn icon(mut self, icon: impl Into<String>) -> Self {
    self.icon = Some(icon.into());
    self
  }

  /// Set badge text
  pub fn badge(mut self, badge: impl Into<String>) -> Self {
    self.badge = Some(badge.into());
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set section style
  pub fn style(mut self, style: AccordionSectionStyle) -> Self {
    self.style = style;
    self
  }
}

/// Styling configuration for accordion sections
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccordionSectionStyle {
  /// Header background color
  pub header_background: Option<ColorDefinition>,
  /// Header text color
  pub header_text_color: Option<ColorDefinition>,
  /// Content background color
  pub content_background: Option<ColorDefinition>,
  /// Content text color
  pub content_text_color: Option<ColorDefinition>,
  /// Border color
  pub border_color: Option<ColorDefinition>,
  /// Disabled section opacity
  pub disabled_opacity: f32,
  /// Custom padding for content
  pub content_padding: u16,
  /// Header height
  pub header_height: u16,
}

impl Default for AccordionSectionStyle {
  fn default() -> Self {
    Self {
      header_background: None,
      header_text_color: None,
      content_background: None,
      content_text_color: None,
      border_color: None,
      disabled_opacity: 0.6,
      content_padding: 2,
      header_height: 3,
    }
  }
}

/// Accordion state management
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AccordionState {
  /// Currently expanded sections
  pub expanded_sections: Vec<SectionId>,
  /// Currently focused section
  pub focused_section: Option<SectionId>,
  /// Animation states for sections
  pub animation_states: HashMap<SectionId, AnimationState>,
  /// Whether accordion is disabled
  pub disabled: bool,
}

/// Animation state for individual sections
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum AnimationState {
  /// Section is collapsed
  Collapsed,
  /// Section is expanding
  Expanding,
  /// Section is expanded
  Expanded,
  /// Section is collapsing
  Collapsing,
}

/// Accordion configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AccordionConfig {
  /// Allow multiple sections to be expanded simultaneously
  pub multi_expand: bool,
  /// Enable animations
  pub animated: bool,
  /// Animation configuration
  pub animation: AccordionAnimation,
  /// Show section borders
  pub bordered: bool,
  /// Rounded corners for sections
  pub rounded: bool,
  /// Compact layout (reduced spacing)
  pub compact: bool,
  /// Collapsible sections (can all be collapsed)
  pub collapsible: bool,
  /// Show expand/collapse icons
  pub show_icons: bool,
  /// Icon to show when section is collapsed
  pub collapsed_icon: String,
  /// Icon to show when section is expanded
  pub expanded_icon: String,
}

impl Default for AccordionConfig {
  fn default() -> Self {
    Self {
      multi_expand: false,
      animated: true,
      animation: AccordionAnimation::default(),
      bordered: true,
      rounded: false,
      compact: false,
      collapsible: true,
      show_icons: true,
      collapsed_icon: "▶".to_string(), // Right arrow
      expanded_icon: "▼".to_string(),  // Down arrow
    }
  }
}

/// Event callbacks for accordion interactions
#[derive(Default)]
pub struct AccordionCallbacks {
  /// Called when a section is expanded
  pub on_expand: Option<OnExpandCallback>,
  /// Called when a section is collapsed
  pub on_collapse: Option<OnCollapseCallback>,
  /// Called when sections change (with all expanded section IDs)
  pub on_change: Option<OnChangeCallback>,
  /// Called when focus changes
  pub on_focus: Option<OnFocusCallback>,
  /// Called on section click/activation
  pub on_section_click: Option<OnSectionClickCallback>,
}

/// Main Accordion widget
pub struct Accordion {
  /// Unique accordion identifier
  pub id: String,
  /// Accordion sections
  pub sections: Vec<AccordionSection>,
  /// Reactive state management
  pub state: Reactive<AccordionState>,
  /// Configuration options
  pub config: AccordionConfig,
  /// Event callbacks
  pub callbacks: AccordionCallbacks,
  /// CSS utility classes
  pub css_classes: Vec<String>,
}

impl Accordion {
  /// Create a new accordion builder
  pub fn builder<S: Into<String>>(id: S) -> AccordionBuilder {
    AccordionBuilder::new(id)
  }

  /// Add a new section to the accordion
  pub fn add_section(&mut self, section: AccordionSection) {
    self.sections.push(section);
    self.refresh_state();
  }

  /// Remove a section from the accordion
  pub fn remove_section(&mut self, section_id: &str) -> Option<AccordionSection> {
    if let Some(index) = self.sections.iter().position(|s| s.id == section_id) {
      let removed = self.sections.remove(index);
      self.refresh_state();
      Some(removed)
    } else {
      None
    }
  }

  /// Expand a specific section
  pub fn expand_section(&mut self, section_id: impl AsRef<str>) -> Result<()> {
    let section_id = section_id.as_ref();

    // Check if section exists and is not disabled
    let section = self.sections.iter().find(|s| s.id == section_id);
    if section.is_none() {
      return Err(TuiError::component(format!(
        "Section '{section_id}' not found"
      )));
    }

    let Some(section) = section else {
      unreachable!("checked above")
    };
    if section.disabled {
      return Err(TuiError::component(format!(
        "Section '{section_id}' is disabled"
      )));
    }

    self.state.update(|state| {
      // If not multi-expand mode, collapse all other sections
      if !self.config.multi_expand {
        state.expanded_sections.clear();
        // Set all sections to collapsed in animation states
        for s in &self.sections {
          if s.id != section_id {
            state
              .animation_states
              .insert(s.id.clone(), AnimationState::Collapsed);
          }
        }
      }

      // Add section to expanded list if not already there
      if !state.expanded_sections.contains(&section_id.to_string()) {
        state.expanded_sections.push(section_id.to_string());

        // Set animation state
        if self.config.animated {
          state
            .animation_states
            .insert(section_id.to_string(), AnimationState::Expanding);
        } else {
          state
            .animation_states
            .insert(section_id.to_string(), AnimationState::Expanded);
        }
      }
    });

    // Update section expanded state
    if let Some(section) = self.sections.iter_mut().find(|s| s.id == section_id) {
      section.expanded = true;
    }

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_expand {
      callback(&section_id.to_string());
    }

    self.trigger_change_callback();
    Ok(())
  }

  /// Collapse a specific section
  pub fn collapse_section(&mut self, section_id: impl AsRef<str>) -> Result<()> {
    let section_id = section_id.as_ref();

    // Check if section exists
    if !self.sections.iter().any(|s| s.id == section_id) {
      return Err(TuiError::component(format!(
        "Section '{section_id}' not found"
      )));
    }

    self.state.update(|state| {
      // Remove section from expanded list
      state.expanded_sections.retain(|id| id != section_id);

      // Set animation state
      if self.config.animated {
        state
          .animation_states
          .insert(section_id.to_string(), AnimationState::Collapsing);
      } else {
        state
          .animation_states
          .insert(section_id.to_string(), AnimationState::Collapsed);
      }
    });

    // Update section expanded state
    if let Some(section) = self.sections.iter_mut().find(|s| s.id == section_id) {
      section.expanded = false;
    }

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_collapse {
      callback(&section_id.to_string());
    }

    self.trigger_change_callback();
    Ok(())
  }

  /// Toggle a section's expanded state
  pub fn toggle_section(&mut self, section_id: impl AsRef<str>) -> Result<()> {
    let section_id = section_id.as_ref();
    let is_expanded = self.is_section_expanded(section_id);

    if is_expanded {
      self.collapse_section(section_id)
    } else {
      self.expand_section(section_id)
    }
  }

  /// Check if a section is expanded
  pub fn is_section_expanded(&self, section_id: &str) -> bool {
    self
      .state
      .get()
      .expanded_sections
      .contains(&section_id.to_string())
  }

  /// Expand all sections (if multi-expand is enabled)
  pub fn expand_all(&mut self) -> Result<()> {
    if !self.config.multi_expand {
      return Err(TuiError::component(
        "Cannot expand all sections when multi_expand is disabled",
      ));
    }

    let section_ids: Vec<_> = self
      .sections
      .iter()
      .filter(|s| !s.disabled)
      .map(|s| s.id.clone())
      .collect();

    for section_id in section_ids {
      self.expand_section(&section_id)?;
    }
    Ok(())
  }

  /// Collapse all sections
  pub fn collapse_all(&mut self) -> Result<()> {
    let section_ids: Vec<_> = self
      .sections
      .iter()
      .filter(|s| self.is_section_expanded(&s.id))
      .map(|s| s.id.clone())
      .collect();

    for section_id in section_ids {
      self.collapse_section(&section_id)?;
    }
    Ok(())
  }

  /// Focus a specific section
  pub fn focus_section(&mut self, section_id: impl AsRef<str>) -> Result<()> {
    let section_id = section_id.as_ref();

    if !self.sections.iter().any(|s| s.id == section_id) {
      return Err(TuiError::component(format!(
        "Section '{section_id}' not found"
      )));
    }

    self.state.update(|state| {
      state.focused_section = Some(section_id.to_string());
    });

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_focus {
      callback(&section_id.to_string());
    }

    Ok(())
  }

  /// Get the currently focused section
  pub fn get_focused_section(&self) -> Option<String> {
    self.state.get().focused_section.clone()
  }

  /// Navigate to next section
  pub fn focus_next(&mut self) -> Result<()> {
    let current_focus = self.get_focused_section();
    let section_ids: Vec<_> = self.sections.iter().map(|s| s.id.clone()).collect();

    if section_ids.is_empty() {
      return Ok(());
    }

    let next_index = if let Some(current) = current_focus {
      if let Some(current_index) = section_ids.iter().position(|id| id == &current) {
        (current_index + 1) % section_ids.len()
      } else {
        0
      }
    } else {
      0
    };

    self.focus_section(&section_ids[next_index])
  }

  /// Navigate to previous section
  pub fn focus_previous(&mut self) -> Result<()> {
    let current_focus = self.get_focused_section();
    let section_ids: Vec<_> = self.sections.iter().map(|s| s.id.clone()).collect();

    if section_ids.is_empty() {
      return Ok(());
    }

    let prev_index = if let Some(current) = current_focus {
      if let Some(current_index) = section_ids.iter().position(|id| id == &current) {
        if current_index == 0 {
          section_ids.len() - 1
        } else {
          current_index - 1
        }
      } else {
        section_ids.len() - 1
      }
    } else {
      section_ids.len() - 1
    };

    self.focus_section(&section_ids[prev_index])
  }

  /// Focus first section
  pub fn focus_first(&mut self) -> Result<()> {
    if let Some(first_id) = self.sections.first().map(|s| s.id.clone()) {
      self.focus_section(&first_id)
    } else {
      Ok(())
    }
  }

  /// Focus last section
  pub fn focus_last(&mut self) -> Result<()> {
    if let Some(last_id) = self.sections.last().map(|s| s.id.clone()) {
      self.focus_section(&last_id)
    } else {
      Ok(())
    }
  }

  /// Get all expanded section IDs
  pub fn get_expanded_sections(&self) -> Vec<String> {
    self.state.get().expanded_sections.clone()
  }

  /// Get section by ID
  pub fn get_section(&self, section_id: &str) -> Option<&AccordionSection> {
    self.sections.iter().find(|s| s.id == section_id)
  }

  /// Get mutable section by ID
  pub fn get_section_mut(&mut self, section_id: &str) -> Option<&mut AccordionSection> {
    self.sections.iter_mut().find(|s| s.id == section_id)
  }

  /// Update section content
  pub fn update_section_content(
    &mut self,
    section_id: impl AsRef<str>,
    content: impl Into<String>,
  ) -> Result<()> {
    let section_id = section_id.as_ref();

    if let Some(section) = self.get_section_mut(section_id) {
      section.content = content.into();
      Ok(())
    } else {
      Err(TuiError::component(format!(
        "Section '{section_id}' not found"
      )))
    }
  }

  /// Enable/disable the entire accordion
  pub fn set_disabled(&mut self, disabled: bool) {
    self.state.update(|state| {
      state.disabled = disabled;
    });
  }

  /// Check if accordion is disabled
  pub fn is_disabled(&self) -> bool {
    self.state.get().disabled
  }

  /// Get the number of sections
  pub fn section_count(&self) -> usize {
    self.sections.len()
  }

  /// Private helper to refresh internal state
  fn refresh_state(&mut self) {
    self.state.update(|state| {
      // Remove expanded sections that no longer exist
      let valid_section_ids: Vec<_> = self.sections.iter().map(|s| s.id.clone()).collect();
      state
        .expanded_sections
        .retain(|id| valid_section_ids.contains(id));

      // Remove animation states for sections that no longer exist
      state
        .animation_states
        .retain(|id, _| valid_section_ids.contains(id));

      // Reset focus if focused section no longer exists
      if let Some(focused) = &state.focused_section {
        if !valid_section_ids.contains(focused) {
          state.focused_section = None;
        }
      }
    });
  }

  /// Private helper to trigger change callback
  fn trigger_change_callback(&self) {
    if let Some(callback) = &self.callbacks.on_change {
      let expanded = self.get_expanded_sections();
      callback(&expanded);
    }
  }

  /// Render the accordion to a string
  pub fn render(&self, _layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Base CSS classes
    let mut classes = vec!["accordion".to_string()];
    if self.config.bordered {
      classes.push("accordion-bordered".to_string());
    }
    if self.config.rounded {
      classes.push("accordion-rounded".to_string());
    }
    if self.config.compact {
      classes.push("accordion-compact".to_string());
    }
    if state.disabled {
      classes.push("accordion-disabled".to_string());
    }
    classes.extend(self.css_classes.clone());

    for (section_index, section) in self.sections.iter().enumerate() {
      let is_expanded = state.expanded_sections.contains(&section.id);
      let is_focused = state.focused_section.as_ref() == Some(&section.id);
      let animation_state =
        state
          .animation_states
          .get(&section.id)
          .copied()
          .unwrap_or(if is_expanded {
            AnimationState::Expanded
          } else {
            AnimationState::Collapsed
          });

      // Section classes
      let mut section_classes = vec!["accordion-section".to_string()];
      if is_expanded {
        section_classes.push("accordion-section-expanded".to_string());
      }
      if is_focused {
        section_classes.push("accordion-section-focused".to_string());
      }
      if section.disabled {
        section_classes.push("accordion-section-disabled".to_string());
      }
      section_classes.extend(section.css_classes.clone());

      // Render section header
      self.render_section_header(&mut output, section, is_expanded, is_focused, theme);

      // Render section content if expanded or animating
      match animation_state {
        AnimationState::Expanded | AnimationState::Expanding | AnimationState::Collapsing => {
          self.render_section_content(&mut output, section, animation_state, theme);
        }
        AnimationState::Collapsed => {
          // Content is hidden
        }
      }

      // Add spacing between sections (except last)
      if section_index < self.sections.len() - 1 && !self.config.compact {
        let _ = writeln!(output);
      }
    }

    output
  }

  /// Render section header
  fn render_section_header(
    &self,
    output: &mut String,
    section: &AccordionSection,
    is_expanded: bool,
    is_focused: bool,
    _theme: Option<&ColorTheme>,
  ) {
    let icon = if self.config.show_icons {
      if is_expanded {
        &self.config.expanded_icon
      } else {
        &self.config.collapsed_icon
      }
    } else {
      ""
    };

    let focus_indicator = if is_focused { "► " } else { "  " };
    let disabled_indicator = if section.disabled { " (disabled)" } else { "" };

    // Header line
    let _ = write!(output, "{focus_indicator}{icon}");
    if !icon.is_empty() {
      let _ = write!(output, " ");
    }

    // Section icon
    if let Some(section_icon) = &section.icon {
      let _ = write!(output, "{section_icon} ");
    }

    // Title
    let _ = write!(output, "{}", section.title);

    // Badge
    if let Some(badge) = &section.badge {
      let _ = write!(output, " [{badge}]");
    }

    let _ = write!(output, "{disabled_indicator}");
    let _ = writeln!(output);

    // Description (if present)
    if let Some(description) = &section.description {
      let _ = writeln!(output, "    {description}");
    }
  }

  /// Render section content
  fn render_section_content(
    &self,
    output: &mut String,
    section: &AccordionSection,
    animation_state: AnimationState,
    _theme: Option<&ColorTheme>,
  ) {
    let padding = " ".repeat(
      self
        .config
        .if_compact(2, section.style.content_padding as usize),
    );

    // Animation indicator
    match animation_state {
      AnimationState::Expanding => {
        let _ = writeln!(output, "{padding}[Expanding...]");
      }
      AnimationState::Collapsing => {
        let _ = writeln!(output, "{padding}[Collapsing...]");
      }
      _ => {}
    }

    // Content lines
    for line in section.content.lines() {
      let _ = writeln!(output, "{padding}{line}");
    }

    // Add bottom padding for expanded sections
    if matches!(animation_state, AnimationState::Expanded) && !self.config.compact {
      let _ = writeln!(output);
    }
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
      focusable: false,
      focused: false,
      disabled: false,
      tab_index: None,
      key_bindings: Vec::new(),
      modal: false,
    }
  }
}

impl fmt::Display for Accordion {
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

impl AccordionConfig {
  /// Helper for compact spacing
  fn if_compact<T>(&self, compact_value: T, normal_value: T) -> T {
    if self.compact {
      compact_value
    } else {
      normal_value
    }
  }
}

/// Builder for creating accordions
pub struct AccordionBuilder {
  id: String,
  sections: Vec<AccordionSection>,
  config: AccordionConfig,
  callbacks: AccordionCallbacks,
  css_classes: Vec<String>,
}

impl AccordionBuilder {
  /// Create a new accordion builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      sections: Vec::new(),
      config: AccordionConfig::default(),
      callbacks: AccordionCallbacks::default(),
      css_classes: Vec::new(),
    }
  }

  /// Add a section to the accordion
  pub fn section(mut self, section: AccordionSection) -> Self {
    self.sections.push(section);
    self
  }

  /// Enable/disable multi-expand mode
  pub fn multi_expand(mut self, enabled: bool) -> Self {
    self.config.multi_expand = enabled;
    self
  }

  /// Enable/disable animations
  pub fn animated(mut self, enabled: bool) -> Self {
    self.config.animated = enabled;
    self
  }

  /// Set animation configuration
  pub fn animation(mut self, animation: AccordionAnimation) -> Self {
    self.config.animation = animation;
    self
  }

  /// Enable/disable borders
  pub fn bordered(mut self, enabled: bool) -> Self {
    self.config.bordered = enabled;
    self
  }

  /// Enable/disable rounded corners
  pub fn rounded(mut self, enabled: bool) -> Self {
    self.config.rounded = enabled;
    self
  }

  /// Enable/disable compact layout
  pub fn compact(mut self, enabled: bool) -> Self {
    self.config.compact = enabled;
    self
  }

  /// Set custom expand/collapse icons
  pub fn icons(mut self, collapsed: impl Into<String>, expanded: impl Into<String>) -> Self {
    self.config.collapsed_icon = collapsed.into();
    self.config.expanded_icon = expanded.into();
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set expand callback
  pub fn on_expand<F>(mut self, callback: F) -> Self
  where
    F: Fn(&SectionId) + Send + Sync + 'static,
  {
    self.callbacks.on_expand = Some(Arc::new(callback));
    self
  }

  /// Set collapse callback
  pub fn on_collapse<F>(mut self, callback: F) -> Self
  where
    F: Fn(&SectionId) + Send + Sync + 'static,
  {
    self.callbacks.on_collapse = Some(Arc::new(callback));
    self
  }

  /// Set change callback
  pub fn on_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&[SectionId]) + Send + Sync + 'static,
  {
    self.callbacks.on_change = Some(Arc::new(callback));
    self
  }

  /// Set focus callback
  pub fn on_focus<F>(mut self, callback: F) -> Self
  where
    F: Fn(&SectionId) + Send + Sync + 'static,
  {
    self.callbacks.on_focus = Some(Arc::new(callback));
    self
  }

  /// Set section click callback
  pub fn on_section_click<F>(mut self, callback: F) -> Self
  where
    F: Fn(&SectionId) + Send + Sync + 'static,
  {
    self.callbacks.on_section_click = Some(Arc::new(callback));
    self
  }

  /// Build the accordion
  pub fn build(self) -> Accordion {
    let state = AccordionState {
      expanded_sections: self
        .sections
        .iter()
        .filter(|s| s.expanded)
        .map(|s| s.id.clone())
        .collect(),
      focused_section: None,
      animation_states: HashMap::new(),
      disabled: false,
    };

    Accordion {
      id: self.id,
      sections: self.sections,
      state: Reactive::new(state),
      config: self.config,
      callbacks: self.callbacks,
      css_classes: self.css_classes,
    }
  }
}

/// Convenience functions for common accordion patterns
/// Create a simple settings-style accordion
pub fn settings_accordion(sections: Vec<(&str, &str, &str)>) -> Accordion {
  let mut builder = AccordionBuilder::new("settings-accordion")
    .multi_expand(true)
    .bordered(true)
    .animated(true);

  for (id, title, content) in sections {
    builder = builder.section(AccordionSection::new(id, title).content(content));
  }

  builder.build()
}

/// Create a FAQ-style accordion (single expand)
pub fn faq_accordion(faqs: Vec<(&str, &str, &str)>) -> Accordion {
  let mut builder = AccordionBuilder::new("faq-accordion")
    .multi_expand(false)
    .bordered(true)
    .animated(true)
    .icons("❓", "✅");

  for (id, question, answer) in faqs {
    builder = builder.section(AccordionSection::new(id, question).content(answer));
  }

  builder.build()
}

/// Create a compact accordion with minimal styling
pub fn compact_accordion(sections: Vec<(&str, &str, &str)>) -> Accordion {
  let mut builder = AccordionBuilder::new("compact-accordion")
    .compact(true)
    .bordered(false)
    .animated(false)
    .icons("▸", "▾");

  for (id, title, content) in sections {
    builder = builder.section(AccordionSection::new(id, title).content(content));
  }

  builder.build()
}
