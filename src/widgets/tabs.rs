/*!
 * Tabs Component - Dynamic tab navigation with content switching
 *
 * A comprehensive tabs widget providing flexible tab navigation with:
 * - Multiple tab positions (top, bottom, left, right)
 * - Dynamic content switching with lazy loading
 * - Closeable tabs with keyboard and mouse support
 * - Icon support and badges for tab labels
 * - Scrollable tabs for overflow handling
 * - CSS utility class integration and theming
 */

use crate::{
  layout::LayoutRect,
  themes::{color_to_ansi, ColorDefinition, ColorTheme, UtilityProcessor},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// Tab position within the tabs container
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TabPosition {
  /// Tabs at the top (default)
  #[default]
  Top,
  /// Tabs at the bottom
  Bottom,
  /// Tabs on the left side
  Left,
  /// Tabs on the right side
  Right,
}

/// Tab orientation for content layout
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TabOrientation {
  /// Horizontal tab layout
  #[default]
  Horizontal,
  /// Vertical tab layout
  Vertical,
}

/// Tab size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TabSize {
  /// Small tabs
  Small,
  /// Normal size tabs
  #[default]
  Normal,
  /// Large tabs
  Large,
  /// Custom size
  Custom(u16),
}

/// Tab border styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum TabBorderStyle {
  /// No border
  None,
  /// Simple line border
  #[default]
  Line,
  /// Box border around active tab
  Box,
  /// Underline for active tab
  Underline,
  /// Custom border character
  Custom(char),
}

/// Individual tab configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tab {
  /// Unique tab identifier
  pub id: String,
  /// Tab label text
  pub label: String,
  /// Optional icon
  pub icon: Option<char>,
  /// Optional badge text
  pub badge: Option<String>,
  /// Whether tab is closeable
  pub closeable: bool,
  /// Whether tab is disabled
  pub disabled: bool,
  /// Tab content (can be lazy loaded)
  pub content: Option<String>,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
  /// Tab-specific tooltip
  pub tooltip: Option<String>,
}

/// Tab container styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TabStyle {
  /// Active tab background color
  pub active_background: Option<ColorDefinition>,
  /// Inactive tab background color
  pub inactive_background: Option<ColorDefinition>,
  /// Active tab text color
  pub active_text: Option<ColorDefinition>,
  /// Inactive tab text color
  pub inactive_text: Option<ColorDefinition>,
  /// Border color
  pub border_color: Option<ColorDefinition>,
  /// Border style
  pub border_style: TabBorderStyle,
  /// Tab padding
  pub padding: u16,
  /// Tab spacing
  pub spacing: u16,
  /// Whether to fill available width
  pub fill_width: bool,
}

/// CSS-styled tabs widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Tabs {
  /// Unique identifier
  pub id: String,
  /// Tab position
  pub position: TabPosition,
  /// Tab orientation
  pub orientation: TabOrientation,
  /// Tab size
  pub size: TabSize,
  /// List of tabs
  pub tabs: Vec<Tab>,
  /// Currently active tab index
  pub active_tab: usize,
  /// Styling configuration
  pub style: TabStyle,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
  /// Whether tabs are visible
  pub visible: bool,
  /// Whether tabs are scrollable when overflowing
  pub scrollable: bool,
  /// Content cache for lazy loading
  pub content_cache: HashMap<String, String>,
}

/// Builder for Tabs component
pub struct TabsBuilder {
  tabs: Tabs,
}

impl Default for TabStyle {
  fn default() -> Self {
    Self {
      active_background: None,
      inactive_background: None,
      active_text: None,
      inactive_text: None,
      border_color: None,
      border_style: TabBorderStyle::Line,
      padding: 1,
      spacing: 0,
      fill_width: true,
    }
  }
}

impl Tab {
  /// Create a new tab
  pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      icon: None,
      badge: None,
      closeable: false,
      disabled: false,
      content: None,
      css_classes: Vec::new(),
      tooltip: None,
    }
  }

  /// Set tab icon
  pub fn icon(mut self, icon: char) -> Self {
    self.icon = Some(icon);
    self
  }

  /// Set tab badge
  pub fn badge(mut self, badge: impl Into<String>) -> Self {
    self.badge = Some(badge.into());
    self
  }

  /// Make tab closeable
  pub fn closeable(mut self) -> Self {
    self.closeable = true;
    self
  }

  /// Disable tab
  pub fn disabled(mut self) -> Self {
    self.disabled = true;
    self
  }

  /// Set tab content
  pub fn content(mut self, content: impl Into<String>) -> Self {
    self.content = Some(content.into());
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set tooltip
  pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
    self.tooltip = Some(tooltip.into());
    self
  }
}

impl Tabs {
  /// Create a new tabs container
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      position: TabPosition::default(),
      orientation: TabOrientation::default(),
      size: TabSize::default(),
      tabs: Vec::new(),
      active_tab: 0,
      style: TabStyle::default(),
      css_classes: Vec::new(),
      visible: true,
      scrollable: false,
      content_cache: HashMap::new(),
    }
  }

  /// Create a builder for the tabs container
  pub fn builder(id: impl Into<String>) -> TabsBuilder {
    TabsBuilder {
      tabs: Self::new(id),
    }
  }

  /// Add a tab
  pub fn add_tab(&mut self, tab: Tab) {
    self.tabs.push(tab);
  }

  /// Remove tab by ID
  pub fn remove_tab(&mut self, tab_id: &str) -> Option<Tab> {
    if let Some(pos) = self.tabs.iter().position(|tab| tab.id == tab_id) {
      let removed_tab = self.tabs.remove(pos);

      // Adjust active tab if necessary
      if pos < self.active_tab {
        self.active_tab = self.active_tab.saturating_sub(1);
      } else if pos == self.active_tab && self.active_tab >= self.tabs.len() {
        self.active_tab = self.tabs.len().saturating_sub(1);
      }

      // Remove from content cache
      self.content_cache.remove(tab_id);

      Some(removed_tab)
    } else {
      None
    }
  }

  /// Get tab by ID
  pub fn get_tab(&self, tab_id: &str) -> Option<&Tab> {
    self.tabs.iter().find(|tab| tab.id == tab_id)
  }

  /// Get mutable tab by ID
  pub fn get_tab_mut(&mut self, tab_id: &str) -> Option<&mut Tab> {
    self.tabs.iter_mut().find(|tab| tab.id == tab_id)
  }

  /// Set active tab by index
  pub fn set_active_tab(&mut self, index: usize) {
    if index < self.tabs.len() {
      self.active_tab = index;
    }
  }

  /// Set active tab by ID
  pub fn set_active_tab_by_id(&mut self, tab_id: &str) {
    if let Some(index) = self.tabs.iter().position(|tab| tab.id == tab_id) {
      self.active_tab = index;
    }
  }

  /// Get currently active tab
  pub fn get_active_tab(&self) -> Option<&Tab> {
    self.tabs.get(self.active_tab)
  }

  /// Get active tab content (with caching)
  pub fn get_active_content(&mut self) -> Option<String> {
    // Get active tab info first to avoid borrowing conflicts
    let (active_tab_id, active_tab_content) = {
      let active_tab = self.get_active_tab()?;
      (active_tab.id.clone(), active_tab.content.clone())
    };

    // Check cache first
    if let Some(cached_content) = self.content_cache.get(&active_tab_id) {
      return Some(cached_content.clone());
    }

    // Use tab content if available
    if let Some(content) = active_tab_content {
      self.content_cache.insert(active_tab_id, content.clone());
      return Some(content);
    }

    None
  }

  /// Set content for a specific tab (with caching)
  pub fn set_tab_content(&mut self, tab_id: &str, content: String) {
    self
      .content_cache
      .insert(tab_id.to_string(), content.clone());

    if let Some(tab) = self.get_tab_mut(tab_id) {
      tab.content = Some(content);
    }
  }

  /// Navigate to next tab
  pub fn next_tab(&mut self) {
    if !self.tabs.is_empty() {
      self.active_tab = (self.active_tab + 1) % self.tabs.len();
    }
  }

  /// Navigate to previous tab
  pub fn prev_tab(&mut self) {
    if !self.tabs.is_empty() {
      self.active_tab = if self.active_tab == 0 {
        self.tabs.len() - 1
      } else {
        self.active_tab - 1
      };
    }
  }

  /// Get tab header height based on position and size
  pub fn get_header_height(&self) -> u16 {
    match self.position {
      TabPosition::Top | TabPosition::Bottom => match self.size {
        TabSize::Small => 1,
        TabSize::Normal => 2,
        TabSize::Large => 3,
        TabSize::Custom(height) => height,
      },
      TabPosition::Left | TabPosition::Right => 0, // Vertical tabs don't affect height
    }
  }

  /// Get tab header width for vertical tabs
  pub fn get_header_width(&self) -> u16 {
    match self.position {
      TabPosition::Left | TabPosition::Right => {
        // Calculate max tab label width + padding + icons
        let max_label_width = self
          .tabs
          .iter()
          .map(|tab| {
            tab.label.chars().count()
              + if tab.icon.is_some() { 2 } else { 0 }
              + if tab.badge.is_some() { 4 } else { 0 }
              + if tab.closeable { 2 } else { 0 }
          })
          .max()
          .unwrap_or(0) as u16;

        max_label_width + (self.style.padding * 2)
      }
      TabPosition::Top | TabPosition::Bottom => 0, // Horizontal tabs don't affect width
    }
  }

  /// Render the tabs widget
  pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    if !self.visible || self.tabs.is_empty() {
      return String::new();
    }

    let mut output = String::new();

    match self.position {
      TabPosition::Top => {
        let _ = writeln!(output, "{}", self.render_horizontal_tabs(layout, theme));
        let _ = writeln!(output, "{}", self.render_content_area(layout, theme));
      }
      TabPosition::Bottom => {
        let _ = writeln!(output, "{}", self.render_content_area(layout, theme));
        let _ = writeln!(output, "{}", self.render_horizontal_tabs(layout, theme));
      }
      TabPosition::Left => {
        let _ = write!(
          output,
          "{}",
          self.render_vertical_layout(layout, theme, true)
        );
      }
      TabPosition::Right => {
        let _ = write!(
          output,
          "{}",
          self.render_vertical_layout(layout, theme, false)
        );
      }
    }

    output
  }

  /// Render horizontal tabs (top/bottom position)
  fn render_horizontal_tabs(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let tab_width = if self.style.fill_width {
      layout.width / self.tabs.len() as u16
    } else {
      self.calculate_tab_width()
    };

    for (index, tab) in self.tabs.iter().enumerate() {
      let is_active = index == self.active_tab;
      let tab_content = self.render_tab_header(tab, is_active, tab_width, theme);
      let _ = write!(output, "{tab_content}");
    }

    output
  }

  /// Render vertical layout (left/right position)
  fn render_vertical_layout(
    &self,
    layout: &LayoutRect,
    theme: Option<&ColorTheme>,
    tabs_on_left: bool,
  ) -> String {
    let mut output = String::new();
    let tab_width = self.get_header_width();
    let content_width = layout.width.saturating_sub(tab_width);

    for row in 0..layout.height {
      let mut line = String::new();

      if tabs_on_left {
        // Tabs on left, content on right
        if (row as usize) < self.tabs.len() {
          let tab = &self.tabs[row as usize];
          let is_active = row as usize == self.active_tab;
          line.push_str(&self.render_tab_header(tab, is_active, tab_width, theme));
        } else {
          line.push_str(&" ".repeat(tab_width as usize));
        }

        // Content area
        if row == 0 {
          line.push_str(&self.render_content_line(content_width));
        } else {
          line.push_str(&" ".repeat(content_width as usize));
        }
      } else {
        // Content on left, tabs on right
        if row == 0 {
          line.push_str(&self.render_content_line(content_width));
        } else {
          line.push_str(&" ".repeat(content_width as usize));
        }

        if (row as usize) < self.tabs.len() {
          let tab = &self.tabs[row as usize];
          let is_active = row as usize == self.active_tab;
          line.push_str(&self.render_tab_header(tab, is_active, tab_width, theme));
        } else {
          line.push_str(&" ".repeat(tab_width as usize));
        }
      }

      writeln!(output, "{line}").unwrap();
    }

    output
  }

  /// Calculate optimal tab width for non-fill mode
  fn calculate_tab_width(&self) -> u16 {
    self
      .tabs
      .iter()
      .map(|tab| {
        tab.label.chars().count() as u16
          + if tab.icon.is_some() { 2 } else { 0 }
          + if tab.badge.is_some() { 4 } else { 0 }
          + if tab.closeable { 2 } else { 0 }
          + (self.style.padding * 2)
      })
      .max()
      .unwrap_or(10)
  }

  /// Render individual tab header
  fn render_tab_header(
    &self,
    tab: &Tab,
    is_active: bool,
    width: u16,
    theme: Option<&ColorTheme>,
  ) -> String {
    let mut content = String::new();

    // Add padding
    for _ in 0..self.style.padding {
      content.push(' ');
    }

    // Add icon
    if let Some(icon) = tab.icon {
      content.push(icon);
      content.push(' ');
    }

    // Add label
    content.push_str(&tab.label);

    // Add badge
    if let Some(badge) = &tab.badge {
      content.push_str(&format!(" ({badge})"));
    }

    // Add close button
    if tab.closeable {
      content.push_str(" ✕");
    }

    // Pad to width
    let current_width = content.chars().count();
    if current_width < width as usize {
      content.push_str(&" ".repeat(width as usize - current_width));
    } else if current_width > width as usize {
      content = content.chars().take(width as usize).collect();
    }

    // Apply styling
    self.apply_tab_styling(&content, is_active, tab.disabled, theme)
  }

  /// Apply styling to tab content
  fn apply_tab_styling(
    &self,
    content: &str,
    is_active: bool,
    is_disabled: bool,
    theme: Option<&ColorTheme>,
  ) -> String {
    if is_disabled {
      return format!(
        "{}{}{}",
        color_to_ansi(
          ColorDefinition {
            r: 128,
            g: 128,
            b: 128
          },
          true
        ),
        content,
        "\x1b[0m"
      );
    }

    let (bg_color, text_color) = if is_active {
      (
        self.style.active_background.unwrap_or_else(|| {
          theme.map(|t| t.palette.primary).unwrap_or(ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
          })
        }),
        self.style.active_text.unwrap_or_else(|| {
          theme
            .map(|t| t.palette.text_inverse)
            .unwrap_or(ColorDefinition {
              r: 255,
              g: 255,
              b: 255,
            })
        }),
      )
    } else {
      (
        self.style.inactive_background.unwrap_or_else(|| {
          theme.map(|t| t.palette.surface).unwrap_or(ColorDefinition {
            r: 243,
            g: 244,
            b: 246,
          })
        }),
        self.style.inactive_text.unwrap_or_else(|| {
          theme
            .map(|t| t.palette.text_secondary)
            .unwrap_or(ColorDefinition {
              r: 107,
              g: 114,
              b: 128,
            })
        }),
      )
    };

    format!(
      "{}{}{}{}",
      color_to_ansi(bg_color, false),
      color_to_ansi(text_color, true),
      content,
      "\x1b[0m"
    )
  }

  /// Render content area
  fn render_content_area(&self, layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let content_height = layout.height.saturating_sub(self.get_header_height());
    let mut output = String::new();

    for _ in 0..content_height {
      writeln!(output, "{}", self.render_content_line(layout.width)).unwrap();
    }

    output
  }

  /// Render a single content line
  fn render_content_line(&self, width: u16) -> String {
    if let Some(active_tab) = self.get_active_tab() {
      if let Some(content) = &active_tab.content {
        // For now, just show first line of content
        let first_line = content.lines().next().unwrap_or("");
        if first_line.len() >= width as usize {
          first_line.chars().take(width as usize).collect()
        } else {
          format!(
            "{}{}",
            first_line,
            " ".repeat(width as usize - first_line.len())
          )
        }
      } else {
        format!(
          "{}{}",
          active_tab.label,
          " ".repeat(width as usize - active_tab.label.len() - 17)
        )
      }
    } else {
      " ".repeat(width as usize)
    }
  }

  /// Render with utility CSS classes
  pub fn render_with_utilities(
    &self,
    layout: &LayoutRect,
    utility_processor: &UtilityProcessor,
  ) -> String {
    // For now, delegate to standard render
    // TODO: Process utility classes and apply ANSI styles
    let _utility_styles = utility_processor.process_classes(&self.css_classes);
    self.render(layout, None)
  }
}

impl TabsBuilder {
  /// Set tab position
  pub fn position(mut self, position: TabPosition) -> Self {
    self.tabs.position = position;
    self
  }

  /// Set tab orientation
  pub fn orientation(mut self, orientation: TabOrientation) -> Self {
    self.tabs.orientation = orientation;
    self
  }

  /// Set tab size
  pub fn size(mut self, size: TabSize) -> Self {
    self.tabs.size = size;
    self
  }

  /// Add a tab
  pub fn tab(mut self, tab: Tab) -> Self {
    self.tabs.tabs.push(tab);
    self
  }

  /// Add a simple tab with label
  pub fn simple_tab(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
    self.tabs.tabs.push(Tab::new(id, label));
    self
  }

  /// Set active tab
  pub fn active(mut self, index: usize) -> Self {
    self.tabs.active_tab = index;
    self
  }

  /// Set tab style
  pub fn style(mut self, style: TabStyle) -> Self {
    self.tabs.style = style;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.tabs.css_classes.push(class.into());
    self
  }

  /// Make tabs scrollable
  pub fn scrollable(mut self) -> Self {
    self.tabs.scrollable = true;
    self
  }

  /// Hide tabs
  pub fn hidden(mut self) -> Self {
    self.tabs.visible = false;
    self
  }

  /// Build the tabs widget
  pub fn build(self) -> Tabs {
    self.tabs
  }
}

/// Convenience functions for common tab configurations
/// Create a simple horizontal tabs widget
pub fn horizontal_tabs(id: impl Into<String>) -> TabsBuilder {
  Tabs::builder(id)
    .position(TabPosition::Top)
    .orientation(TabOrientation::Horizontal)
}

/// Create vertical tabs widget
pub fn vertical_tabs(id: impl Into<String>) -> TabsBuilder {
  Tabs::builder(id)
    .position(TabPosition::Left)
    .orientation(TabOrientation::Vertical)
}

/// Create bottom tabs widget
pub fn bottom_tabs(id: impl Into<String>) -> TabsBuilder {
  Tabs::builder(id)
    .position(TabPosition::Bottom)
    .orientation(TabOrientation::Horizontal)
}

/// Create a card-style tabs widget
pub fn card_tabs(id: impl Into<String>) -> TabsBuilder {
  Tabs::builder(id)
    .position(TabPosition::Top)
    .style(TabStyle {
      border_style: TabBorderStyle::Box,
      padding: 2,
      spacing: 1,
      ..TabStyle::default()
    })
}

/// Create minimal tabs widget
pub fn minimal_tabs(id: impl Into<String>) -> TabsBuilder {
  Tabs::builder(id)
    .position(TabPosition::Top)
    .style(TabStyle {
      border_style: TabBorderStyle::None,
      padding: 1,
      spacing: 0,
      ..TabStyle::default()
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_tabs_creation() {
    let tabs = Tabs::new("test-tabs");
    assert_eq!(tabs.id, "test-tabs");
    assert_eq!(tabs.position, TabPosition::Top);
    assert_eq!(tabs.tabs.len(), 0);
    assert_eq!(tabs.active_tab, 0);
  }

  #[test]
  fn test_tabs_builder() {
    let tabs = Tabs::builder("builder-tabs")
      .simple_tab("tab1", "First Tab")
      .simple_tab("tab2", "Second Tab")
      .active(1)
      .position(TabPosition::Bottom)
      .build();

    assert_eq!(tabs.tabs.len(), 2);
    assert_eq!(tabs.active_tab, 1);
    assert_eq!(tabs.position, TabPosition::Bottom);
    assert_eq!(tabs.tabs[0].label, "First Tab");
    assert_eq!(tabs.tabs[1].label, "Second Tab");
  }

  #[test]
  fn test_tab_creation() {
    let tab = Tab::new("test-tab", "Test Label")
      .icon('📁')
      .badge("5")
      .closeable()
      .content("Tab content here");

    assert_eq!(tab.id, "test-tab");
    assert_eq!(tab.label, "Test Label");
    assert_eq!(tab.icon, Some('📁'));
    assert_eq!(tab.badge, Some("5".to_string()));
    assert!(tab.closeable);
    assert_eq!(tab.content, Some("Tab content here".to_string()));
  }

  #[test]
  fn test_tab_navigation() {
    let mut tabs = Tabs::builder("nav-tabs")
      .simple_tab("tab1", "First")
      .simple_tab("tab2", "Second")
      .simple_tab("tab3", "Third")
      .build();

    assert_eq!(tabs.active_tab, 0);

    tabs.next_tab();
    assert_eq!(tabs.active_tab, 1);

    tabs.next_tab();
    assert_eq!(tabs.active_tab, 2);

    tabs.next_tab(); // Should wrap to 0
    assert_eq!(tabs.active_tab, 0);

    tabs.prev_tab(); // Should wrap to 2
    assert_eq!(tabs.active_tab, 2);

    tabs.prev_tab();
    assert_eq!(tabs.active_tab, 1);
  }

  #[test]
  fn test_tab_management() {
    let mut tabs = Tabs::new("manage-tabs");

    // Add tabs
    tabs.add_tab(Tab::new("tab1", "First Tab"));
    tabs.add_tab(Tab::new("tab2", "Second Tab"));
    tabs.add_tab(Tab::new("tab3", "Third Tab"));

    assert_eq!(tabs.tabs.len(), 3);

    // Set active by ID
    tabs.set_active_tab_by_id("tab2");
    assert_eq!(tabs.active_tab, 1);

    // Remove tab
    let removed = tabs.remove_tab("tab2");
    assert!(removed.is_some());
    assert_eq!(removed.unwrap().label, "Second Tab");
    assert_eq!(tabs.tabs.len(), 2);
    assert_eq!(tabs.active_tab, 1); // Should adjust to tab3

    // Get tab by ID
    let tab = tabs.get_tab("tab1");
    assert!(tab.is_some());
    assert_eq!(tab.unwrap().label, "First Tab");
  }

  #[test]
  fn test_content_caching() {
    let mut tabs = Tabs::builder("cache-tabs")
      .simple_tab("tab1", "First")
      .simple_tab("tab2", "Second")
      .build();

    // Set content for a tab
    tabs.set_tab_content("tab1", "Content for first tab".to_string());

    // Retrieve content
    tabs.set_active_tab_by_id("tab1");
    let content = tabs.get_active_content();
    assert!(content.is_some());
    assert_eq!(content.unwrap(), "Content for first tab");

    // Check cache
    assert!(tabs.content_cache.contains_key("tab1"));
  }

  #[test]
  fn test_convenience_functions() {
    let horizontal = horizontal_tabs("h-tabs").build();
    assert_eq!(horizontal.position, TabPosition::Top);
    assert_eq!(horizontal.orientation, TabOrientation::Horizontal);

    let vertical = vertical_tabs("v-tabs").build();
    assert_eq!(vertical.position, TabPosition::Left);
    assert_eq!(vertical.orientation, TabOrientation::Vertical);

    let bottom = bottom_tabs("b-tabs").build();
    assert_eq!(bottom.position, TabPosition::Bottom);

    let card = card_tabs("c-tabs").build();
    assert_eq!(card.style.border_style, TabBorderStyle::Box);
    assert_eq!(card.style.padding, 2);
  }
}
