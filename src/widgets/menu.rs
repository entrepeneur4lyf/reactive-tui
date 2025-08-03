/*!
 * Menu Component for Navigation and Action Selection
 *
 * A comprehensive menu widget that provides hierarchical navigation,
 * data-driven configuration from JSON/YAML, keyboard shortcuts,
 * and flexible styling for terminal user interfaces.
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, hex, ColorDefinition, ColorTheme, UtilityProcessor},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// Menu item rendering configuration
#[derive(Debug, Clone)]
pub struct MenuItemRenderConfig<'a> {
  pub width: u16,
  pub bg_color: &'a str,
  pub text_color: &'a str,
  pub hover_bg: &'a str,
  pub hover_fg: &'a str,
  pub selected_bg: &'a str,
  pub selected_fg: &'a str,
  pub disabled_fg: &'a str,
}

/// Menu item types for different use cases
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum MenuItemType {
  /// Regular action item
  Action { action: String },
  /// Submenu with nested items
  Submenu { items: Vec<MenuItem> },
  /// Separator line
  Separator,
  /// Header/label (non-interactive)
  Header,
  /// Toggle item with on/off state
  Toggle { state: bool },
  /// Radio group item
  Radio { group: String, selected: bool },
}

/// Menu item configuration
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MenuItem {
  /// Item identifier
  pub id: String,
  /// Display label
  pub label: String,
  /// Item type and configuration
  #[serde(flatten)]
  pub item_type: MenuItemType,
  /// Optional icon character
  pub icon: Option<char>,
  /// Keyboard shortcut
  pub shortcut: Option<String>,
  /// Whether item is enabled
  #[serde(default = "default_true")]
  pub enabled: bool,
  /// Whether item is visible
  #[serde(default = "default_true")]
  pub visible: bool,
  /// CSS classes for styling
  #[serde(default)]
  pub css_classes: Vec<String>,
  /// Optional tooltip text
  pub tooltip: Option<String>,
  /// Custom data for the item
  #[serde(default)]
  pub data: HashMap<String, String>,
}

fn default_true() -> bool {
  true
}

/// Menu orientation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum MenuOrientation {
  /// Vertical menu (default)
  Vertical,
  /// Horizontal menu bar
  Horizontal,
}

/// Menu item state for rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MenuItemState {
  Normal,
  Hovered,
  Selected,
  Disabled,
}

/// Menu styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MenuStyle {
  /// Background color
  pub background: Option<ColorDefinition>,
  /// Text color
  pub foreground: Option<ColorDefinition>,
  /// Hover background color
  pub hover_background: Option<ColorDefinition>,
  /// Hover text color
  pub hover_foreground: Option<ColorDefinition>,
  /// Selected background color
  pub selected_background: Option<ColorDefinition>,
  /// Selected text color
  pub selected_foreground: Option<ColorDefinition>,
  /// Disabled text color
  pub disabled_foreground: Option<ColorDefinition>,
  /// Border color
  pub border_color: Option<ColorDefinition>,
  /// Separator color
  pub separator_color: Option<ColorDefinition>,
  /// Padding around menu items
  pub padding: u16,
  /// Item height
  pub item_height: u16,
  /// Show borders
  pub show_borders: bool,
  /// Show icons
  pub show_icons: bool,
  /// Show shortcuts
  pub show_shortcuts: bool,
  /// Indent for submenus
  pub submenu_indent: u16,
}

impl Default for MenuStyle {
  fn default() -> Self {
    Self {
      background: None,
      foreground: None,
      hover_background: Some(ColorDefinition {
        r: 70,
        g: 130,
        b: 180,
      }),
      hover_foreground: Some(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      }),
      selected_background: Some(ColorDefinition {
        r: 100,
        g: 149,
        b: 237,
      }),
      selected_foreground: Some(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      }),
      disabled_foreground: Some(ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      }),
      border_color: Some(ColorDefinition {
        r: 192,
        g: 192,
        b: 192,
      }),
      separator_color: Some(ColorDefinition {
        r: 169,
        g: 169,
        b: 169,
      }),
      padding: 1,
      item_height: 1,
      show_borders: true,
      show_icons: true,
      show_shortcuts: true,
      submenu_indent: 2,
    }
  }
}

/// Menu navigation state
#[derive(Debug, Clone, Default)]
pub struct MenuState {
  /// Currently selected item index
  pub selected_index: usize,
  /// Navigation stack for submenus
  pub navigation_stack: Vec<Vec<MenuItem>>,
  /// Current menu level items
  pub current_items: Vec<MenuItem>,
  /// Expanded submenu paths
  pub expanded_paths: Vec<String>,
  /// Radio group selections
  pub radio_selections: HashMap<String, String>,
  /// Toggle states
  pub toggle_states: HashMap<String, bool>,
}

/// Main menu widget
#[derive(Debug, Clone)]
pub struct Menu {
  /// Unique identifier
  pub id: String,
  /// Menu items
  pub items: Vec<MenuItem>,
  /// Menu orientation
  pub orientation: MenuOrientation,
  /// Styling configuration
  pub style: MenuStyle,
  /// CSS classes for styling
  pub css_classes: Vec<String>,
  /// Current navigation state
  pub state: MenuState,
  /// Whether menu is visible
  pub visible: bool,
  /// Whether menu can receive focus
  pub focusable: bool,
}

/// Builder for Menu component
pub struct MenuBuilder {
  menu: Menu,
}

impl Menu {
  /// Create a new menu
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      items: Vec::new(),
      orientation: MenuOrientation::Vertical,
      style: MenuStyle::default(),
      css_classes: Vec::new(),
      state: MenuState::default(),
      visible: true,
      focusable: true,
    }
  }

  /// Create a builder for the menu
  pub fn builder(id: impl Into<String>) -> MenuBuilder {
    MenuBuilder {
      menu: Self::new(id),
    }
  }

  /// Load menu from JSON configuration
  pub fn from_json(id: impl Into<String>, json: &str) -> Result<Self> {
    let items: Vec<MenuItem> = serde_json::from_str(json)
      .map_err(|e| TuiError::component(format!("Failed to parse menu JSON: {e}")))?;

    let mut menu = Self::new(id);
    menu.items = items;
    menu.state.current_items = menu.items.clone();
    Ok(menu)
  }

  /// Load menu from YAML configuration
  pub fn from_yaml(id: impl Into<String>, yaml: &str) -> Result<Self> {
    let items: Vec<MenuItem> = serde_yaml::from_str(yaml)
      .map_err(|e| TuiError::component(format!("Failed to parse menu YAML: {e}")))?;

    let mut menu = Self::new(id);
    menu.items = items;
    menu.state.current_items = menu.items.clone();
    Ok(menu)
  }

  /// Add an item to the menu
  pub fn add_item(&mut self, item: MenuItem) {
    self.items.push(item.clone());
    self.state.current_items.push(item);
  }

  /// Remove item by ID
  pub fn remove_item(&mut self, id: &str) -> Option<MenuItem> {
    if let Some(pos) = self.items.iter().position(|item| item.id == id) {
      let removed = self.items.remove(pos);
      self.state.current_items.remove(pos);
      Some(removed)
    } else {
      None
    }
  }

  /// Get item by ID
  pub fn get_item(&self, id: &str) -> Option<&MenuItem> {
    self.items.iter().find(|item| item.id == id)
  }

  /// Get mutable item by ID
  pub fn get_item_mut(&mut self, id: &str) -> Option<&mut MenuItem> {
    self.items.iter_mut().find(|item| item.id == id)
  }

  /// Navigate to next item
  pub fn next_item(&mut self) {
    let visible_items: Vec<_> = self
      .state
      .current_items
      .iter()
      .enumerate()
      .filter(|(_, item)| item.visible && item.enabled)
      .collect();

    if !visible_items.is_empty() {
      let current_pos = visible_items
        .iter()
        .position(|(i, _)| *i == self.state.selected_index)
        .unwrap_or(0);

      let next_pos = (current_pos + 1) % visible_items.len();
      self.state.selected_index = visible_items[next_pos].0;
    }
  }

  /// Navigate to previous item
  pub fn previous_item(&mut self) {
    let visible_items: Vec<_> = self
      .state
      .current_items
      .iter()
      .enumerate()
      .filter(|(_, item)| item.visible && item.enabled)
      .collect();

    if !visible_items.is_empty() {
      let current_pos = visible_items
        .iter()
        .position(|(i, _)| *i == self.state.selected_index)
        .unwrap_or(0);

      let prev_pos = if current_pos == 0 {
        visible_items.len() - 1
      } else {
        current_pos - 1
      };
      self.state.selected_index = visible_items[prev_pos].0;
    }
  }

  /// Activate the currently selected item
  pub fn activate_selected(&mut self) -> Option<String> {
    if let Some(item) = self
      .state
      .current_items
      .get(self.state.selected_index)
      .cloned()
    {
      if !item.enabled || !item.visible {
        return None;
      }

      match &item.item_type {
        MenuItemType::Action { action } => Some(action.clone()),
        MenuItemType::Submenu { items } => {
          self.enter_submenu(items.clone());
          None
        }
        MenuItemType::Toggle { .. } => {
          self.toggle_item(&item.id);
          Some(format!("{}:{}", item.id, item.id))
        }
        MenuItemType::Radio { group, .. } => {
          self.select_radio_item(group, &item.id);
          Some(item.id.to_string())
        }
        _ => None,
      }
    } else {
      None
    }
  }

  /// Enter a submenu
  pub fn enter_submenu(&mut self, items: Vec<MenuItem>) {
    self
      .state
      .navigation_stack
      .push(self.state.current_items.clone());
    self.state.current_items = items;
    self.state.selected_index = 0;
  }

  /// Go back to parent menu
  pub fn go_back(&mut self) -> bool {
    if let Some(parent_items) = self.state.navigation_stack.pop() {
      self.state.current_items = parent_items;
      self.state.selected_index = 0;
      true
    } else {
      false
    }
  }

  /// Toggle a toggle item
  pub fn toggle_item(&mut self, item_id: &str) {
    let current_state = self
      .state
      .toggle_states
      .get(item_id)
      .copied()
      .unwrap_or(false);
    self
      .state
      .toggle_states
      .insert(item_id.to_string(), !current_state);

    // Update the item in current menu
    if let Some(item) = self
      .state
      .current_items
      .iter_mut()
      .find(|i| i.id == item_id)
    {
      if let MenuItemType::Toggle { ref mut state } = item.item_type {
        *state = !current_state;
      }
    }
  }

  /// Select a radio item
  pub fn select_radio_item(&mut self, group: &str, item_id: &str) {
    self
      .state
      .radio_selections
      .insert(group.to_string(), item_id.to_string());

    // Update all radio items in the group
    for item in &mut self.state.current_items {
      if let MenuItemType::Radio {
        group: ref item_group,
        ref mut selected,
      } = item.item_type
      {
        if item_group == group {
          *selected = item.id == item_id;
        }
      }
    }
  }

  /// Get current navigation depth
  pub fn navigation_depth(&self) -> usize {
    self.state.navigation_stack.len()
  }

  /// Get currently selected item
  pub fn selected_item(&self) -> Option<&MenuItem> {
    self.state.current_items.get(self.state.selected_index)
  }

  /// Set menu visibility
  pub fn set_visible(&mut self, visible: bool) {
    self.visible = visible;
  }

  /// Set menu focusable state
  pub fn set_focusable(&mut self, focusable: bool) {
    self.focusable = focusable;
  }

  /// Render the menu with basic theme support
  pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
    if !self.visible {
      return String::new();
    }

    let mut output = String::new();
    let (
      bg_color,
      text_color,
      hover_bg,
      hover_fg,
      selected_bg,
      selected_fg,
      disabled_fg,
      border_color,
    ) = self.get_theme_colors(theme);

    // Render border top
    if self.style.show_borders {
      let border_line = self.render_border_line(layout.width, true, &border_color);
      writeln!(output, "{border_line}").unwrap();
    }

    // Render menu items
    let content_height = if self.style.show_borders {
      layout.height.saturating_sub(2)
    } else {
      layout.height
    };

    let mut current_line = 0;
    for (index, item) in self.state.current_items.iter().enumerate() {
      if !item.visible || current_line >= content_height {
        continue;
      }

      let item_state = if !item.enabled {
        MenuItemState::Disabled
      } else if index == self.state.selected_index {
        MenuItemState::Selected
      } else {
        MenuItemState::Normal
      };

      let render_config = MenuItemRenderConfig {
        width: layout.width,
        bg_color: &bg_color,
        text_color: &text_color,
        hover_bg: &hover_bg,
        hover_fg: &hover_fg,
        selected_bg: &selected_bg,
        selected_fg: &selected_fg,
        disabled_fg: &disabled_fg,
      };

      let item_line = self.render_menu_item(item, item_state, &render_config);

      writeln!(output, "{item_line}").unwrap();
      current_line += self.style.item_height;
    }

    // Fill remaining space
    for _ in current_line..content_height {
      let empty_line = self.render_empty_line(layout.width, &bg_color);
      writeln!(output, "{empty_line}").unwrap();
    }

    // Render border bottom
    if self.style.show_borders {
      let border_line = self.render_border_line(layout.width, false, &border_color);
      writeln!(output, "{border_line}").unwrap();
    }

    output
  }

  /// Render the menu with utility CSS classes
  pub fn render_with_utilities(
    &self,
    layout: &LayoutRect,
    utility_processor: &UtilityProcessor,
  ) -> String {
    if !self.visible {
      return String::new();
    }

    // Process utility classes to get ANSI codes
    let utility_styles = utility_processor.process_classes(&self.css_classes);

    let mut output = String::new();

    // Apply utility styles as background
    let bg_style = if !utility_styles.is_empty() {
      utility_styles.clone()
    } else {
      color_to_ansi(
        self.style.background.unwrap_or(ColorDefinition {
          r: 255,
          g: 255,
          b: 255,
        }),
        false,
      )
    };

    // Use the basic render method with enhanced styling
    let basic_render = self.render(layout, None);

    // Apply utility styles to each line
    for line in basic_render.lines() {
      writeln!(output, "{bg_style}{line}\x1b[0m").unwrap();
    }

    output
  }

  /// Get theme colors with fallbacks
  fn get_theme_colors(
    &self,
    theme: Option<&ColorTheme>,
  ) -> (
    String,
    String,
    String,
    String,
    String,
    String,
    String,
    String,
  ) {
    let bg_color = if let Some(theme) = theme {
      color_to_ansi(theme.palette.background, false)
    } else {
      color_to_ansi(
        self.style.background.unwrap_or(ColorDefinition {
          r: 255,
          g: 255,
          b: 255,
        }),
        false,
      )
    };

    let text_color = color_to_ansi(
      self
        .style
        .foreground
        .unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }),
      true,
    );

    let hover_bg = color_to_ansi(
      self.style.hover_background.unwrap_or(ColorDefinition {
        r: 70,
        g: 130,
        b: 180,
      }),
      false,
    );

    let hover_fg = color_to_ansi(
      self.style.hover_foreground.unwrap_or(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      }),
      true,
    );

    let selected_bg = color_to_ansi(
      self.style.selected_background.unwrap_or(ColorDefinition {
        r: 100,
        g: 149,
        b: 237,
      }),
      false,
    );

    let selected_fg = color_to_ansi(
      self.style.selected_foreground.unwrap_or(ColorDefinition {
        r: 255,
        g: 255,
        b: 255,
      }),
      true,
    );

    let disabled_fg = color_to_ansi(
      self.style.disabled_foreground.unwrap_or(ColorDefinition {
        r: 128,
        g: 128,
        b: 128,
      }),
      true,
    );

    let border_color = color_to_ansi(
      self.style.border_color.unwrap_or(ColorDefinition {
        r: 192,
        g: 192,
        b: 192,
      }),
      true,
    );

    (
      bg_color,
      text_color,
      hover_bg,
      hover_fg,
      selected_bg,
      selected_fg,
      disabled_fg,
      border_color,
    )
  }

  /// Render border line
  fn render_border_line(&self, width: u16, is_top: bool, border_color: &str) -> String {
    let (left_corner, right_corner, horizontal) = if is_top {
      ('┌', '┐', '─')
    } else {
      ('└', '┘', '─')
    };

    let mut line = String::new();
    write!(line, "{border_color}{left_corner}").unwrap();
    for _ in 0..width.saturating_sub(2) {
      line.push(horizontal);
    }
    write!(line, "{right_corner}").unwrap();

    line
  }

  /// Render empty line
  fn render_empty_line(&self, width: u16, bg_color: &str) -> String {
    let mut line = String::new();
    write!(line, "{bg_color}").unwrap();

    if self.style.show_borders {
      line.push('│');
      for _ in 0..width.saturating_sub(2) {
        line.push(' ');
      }
      line.push('│');
    } else {
      for _ in 0..width {
        line.push(' ');
      }
    }

    line
  }

  /// Render individual menu item
  fn render_menu_item(
    &self,
    item: &MenuItem,
    state: MenuItemState,
    config: &MenuItemRenderConfig,
  ) -> String {
    let mut line = String::new();

    // Determine colors based on state
    let (item_bg, item_fg) = match state {
      MenuItemState::Selected => (config.selected_bg, config.selected_fg),
      MenuItemState::Hovered => (config.hover_bg, config.hover_fg),
      MenuItemState::Disabled => (config.bg_color, config.disabled_fg),
      MenuItemState::Normal => (config.bg_color, config.text_color),
    };

    write!(line, "{item_bg}{item_fg}").unwrap();

    // Left border
    if self.style.show_borders {
      line.push('│');
    }

    // Padding
    for _ in 0..self.style.padding {
      line.push(' ');
    }

    // Content based on item type
    match &item.item_type {
      MenuItemType::Separator => {
        let separator_width = config
          .width
          .saturating_sub(if self.style.show_borders { 2 } else { 0 } + self.style.padding * 2);
        for _ in 0..separator_width {
          line.push('─');
        }
      }
      MenuItemType::Header => {
        // Render as label in bold/different style
        let content = self.format_item_content(item, None);
        line.push_str(&content);

        // Pad to fill width
        let used_width = content.chars().count();
        let remaining_width = (config.width as usize).saturating_sub(
          used_width
            + (if self.style.show_borders { 2 } else { 0 })
            + (self.style.padding * 2) as usize,
        );
        for _ in 0..remaining_width {
          line.push(' ');
        }
      }
      _ => {
        let content = self.format_item_content(item, Some(state));
        line.push_str(&content);

        // Pad to fill width
        let used_width = content.chars().count();
        let remaining_width = (config.width as usize).saturating_sub(
          used_width
            + (if self.style.show_borders { 2 } else { 0 })
            + (self.style.padding * 2) as usize,
        );
        for _ in 0..remaining_width {
          line.push(' ');
        }
      }
    }

    // Padding
    for _ in 0..self.style.padding {
      line.push(' ');
    }

    // Right border
    if self.style.show_borders {
      line.push('│');
    }

    line
  }

  /// Format item content with icon, label, and shortcut
  fn format_item_content(&self, item: &MenuItem, _state: Option<MenuItemState>) -> String {
    let mut content = String::new();

    // Icon
    if self.style.show_icons {
      if let Some(icon) = item.icon {
        content.push(icon);
        content.push(' ');
      } else {
        // Add spacing even without icon for alignment
        content.push_str("  ");
      }
    }

    // State indicator for toggle/radio items
    match &item.item_type {
      MenuItemType::Toggle {
        state: toggle_state,
      } => {
        let indicator = if *toggle_state { "☑" } else { "☐" };
        content.push_str(indicator);
        content.push(' ');
      }
      MenuItemType::Radio { selected, .. } => {
        let indicator = if *selected { "◉" } else { "○" };
        content.push_str(indicator);
        content.push(' ');
      }
      MenuItemType::Submenu { .. } => {
        // Add submenu indicator
        content.push_str("▶ ");
      }
      _ => {}
    }

    // Label
    content.push_str(&item.label);

    // Shortcut
    if self.style.show_shortcuts {
      if let Some(ref shortcut) = item.shortcut {
        // Right-align shortcut
        let shortcut_text = format!(" [{shortcut}]");
        content.push_str(&shortcut_text);
      }
    }

    content
  }
}

impl MenuBuilder {
  /// Set menu orientation
  pub fn orientation(mut self, orientation: MenuOrientation) -> Self {
    self.menu.orientation = orientation;
    self
  }

  /// Add an item to the menu
  pub fn item(mut self, item: MenuItem) -> Self {
    self.menu.add_item(item);
    self
  }

  /// Add a simple action item
  pub fn action(
    mut self,
    id: impl Into<String>,
    label: impl Into<String>,
    action: impl Into<String>,
  ) -> Self {
    let item = MenuItem {
      id: id.into(),
      label: label.into(),
      item_type: MenuItemType::Action {
        action: action.into(),
      },
      icon: None,
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    };
    self.menu.add_item(item);
    self
  }

  /// Add a submenu item
  pub fn submenu(
    mut self,
    id: impl Into<String>,
    label: impl Into<String>,
    items: Vec<MenuItem>,
  ) -> Self {
    let item = MenuItem {
      id: id.into(),
      label: label.into(),
      item_type: MenuItemType::Submenu { items },
      icon: None,
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    };
    self.menu.add_item(item);
    self
  }

  /// Add a separator
  pub fn separator(mut self, id: impl Into<String>) -> Self {
    let item = MenuItem {
      id: id.into(),
      label: String::new(),
      item_type: MenuItemType::Separator,
      icon: None,
      shortcut: None,
      enabled: false,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    };
    self.menu.add_item(item);
    self
  }

  /// Add a header/label
  pub fn header(mut self, id: impl Into<String>, label: impl Into<String>) -> Self {
    let item = MenuItem {
      id: id.into(),
      label: label.into(),
      item_type: MenuItemType::Header,
      icon: None,
      shortcut: None,
      enabled: false,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    };
    self.menu.add_item(item);
    self
  }

  /// Add a toggle item
  pub fn toggle(
    mut self,
    id: impl Into<String>,
    label: impl Into<String>,
    initial_state: bool,
  ) -> Self {
    let item = MenuItem {
      id: id.into(),
      label: label.into(),
      item_type: MenuItemType::Toggle {
        state: initial_state,
      },
      icon: None,
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    };
    self.menu.add_item(item);
    self
  }

  /// Set menu style
  pub fn style(mut self, style: MenuStyle) -> Self {
    self.menu.style = style;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.menu.css_classes.push(class.into());
    self
  }

  /// Set background color
  pub fn background(mut self, color: ColorDefinition) -> Self {
    self.menu.style.background = Some(color);
    self
  }

  /// Set text color
  pub fn foreground(mut self, color: ColorDefinition) -> Self {
    self.menu.style.foreground = Some(color);
    self
  }

  /// Load items from JSON
  pub fn from_json(mut self, json: &str) -> Result<Self> {
    let items: Vec<MenuItem> = serde_json::from_str(json)
      .map_err(|e| TuiError::component(format!("Failed to parse menu JSON: {e}")))?;

    for item in items {
      self.menu.add_item(item);
    }

    Ok(self)
  }

  /// Load items from YAML
  pub fn from_yaml(mut self, yaml: &str) -> Result<Self> {
    let items: Vec<MenuItem> = serde_yaml::from_str(yaml)
      .map_err(|e| TuiError::component(format!("Failed to parse menu YAML: {e}")))?;

    for item in items {
      self.menu.add_item(item);
    }

    Ok(self)
  }

  /// Make menu invisible
  pub fn hidden(mut self) -> Self {
    self.menu.visible = false;
    self
  }

  /// Make menu non-focusable
  pub fn non_focusable(mut self) -> Self {
    self.menu.focusable = false;
    self
  }

  /// Build the menu
  pub fn build(self) -> Menu {
    self.menu
  }
}

/// Convenience functions for common menu types
/// Create a context menu
pub fn context_menu(id: impl Into<String>) -> MenuBuilder {
  Menu::builder(id)
    .background(hex("#F0F0F0").unwrap_or(ColorDefinition {
      r: 240,
      g: 240,
      b: 240,
    }))
    .foreground(hex("#000000").unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }))
}

/// Create a menu bar
pub fn menu_bar(id: impl Into<String>) -> MenuBuilder {
  Menu::builder(id)
    .orientation(MenuOrientation::Horizontal)
    .background(hex("#E0E0E0").unwrap_or(ColorDefinition {
      r: 224,
      g: 224,
      b: 224,
    }))
    .foreground(hex("#000000").unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }))
}

/// Create a dropdown menu
pub fn dropdown_menu(id: impl Into<String>) -> MenuBuilder {
  Menu::builder(id)
    .orientation(MenuOrientation::Vertical)
    .background(hex("#FFFFFF").unwrap_or(ColorDefinition {
      r: 255,
      g: 255,
      b: 255,
    }))
    .foreground(hex("#000000").unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }))
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_menu_creation() {
    let menu = Menu::new("test-menu");
    assert_eq!(menu.id, "test-menu");
    assert!(menu.visible);
    assert!(menu.focusable);
    assert_eq!(menu.items.len(), 0);
  }

  #[test]
  fn test_menu_builder() {
    let menu = Menu::builder("main-menu")
      .action("new", "New File", "file:new")
      .action("open", "Open File", "file:open")
      .separator("sep1")
      .action("exit", "Exit", "app:exit")
      .build();

    assert_eq!(menu.items.len(), 4);
    assert_eq!(menu.id, "main-menu");
  }

  #[test]
  fn test_menu_navigation() {
    let mut menu = Menu::builder("nav-menu")
      .action("item1", "Item 1", "action1")
      .action("item2", "Item 2", "action2")
      .action("item3", "Item 3", "action3")
      .build();

    assert_eq!(menu.state.selected_index, 0);

    menu.next_item();
    assert_eq!(menu.state.selected_index, 1);

    menu.next_item();
    assert_eq!(menu.state.selected_index, 2);

    // Should wrap around
    menu.next_item();
    assert_eq!(menu.state.selected_index, 0);

    menu.previous_item();
    assert_eq!(menu.state.selected_index, 2);
  }

  #[test]
  fn test_menu_activation() {
    let mut menu = Menu::builder("action-menu")
      .action("test", "Test Action", "test:action")
      .toggle("toggle", "Toggle Item", false)
      .build();

    // Test action activation
    let result = menu.activate_selected();
    assert_eq!(result, Some("test:action".to_string()));

    // Test toggle activation
    menu.next_item();
    let result = menu.activate_selected();
    assert_eq!(result, Some("toggle:toggle".to_string()));

    // Check toggle state changed
    assert_eq!(menu.state.toggle_states.get("toggle"), Some(&true));
  }

  #[test]
  fn test_submenu_navigation() {
    let submenu_items = vec![MenuItem {
      id: "sub1".to_string(),
      label: "Submenu Item 1".to_string(),
      item_type: MenuItemType::Action {
        action: "sub:action1".to_string(),
      },
      icon: None,
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: HashMap::new(),
    }];

    let mut menu = Menu::builder("submenu-test")
      .submenu("sub", "Submenu", submenu_items)
      .build();

    assert_eq!(menu.navigation_depth(), 0);

    // Enter submenu
    menu.activate_selected();
    assert_eq!(menu.navigation_depth(), 1);
    assert_eq!(menu.state.current_items.len(), 1);
    assert_eq!(menu.state.current_items[0].id, "sub1");

    // Go back
    let went_back = menu.go_back();
    assert!(went_back);
    assert_eq!(menu.navigation_depth(), 0);
  }

  #[test]
  fn test_json_loading() {
    let json = r#"[
            {
                "id": "file",
                "label": "File",
                "type": "Action",
                "action": "file:menu"
            },
            {
                "id": "edit",
                "label": "Edit",
                "type": "Action", 
                "action": "edit:menu"
            }
        ]"#;

    let result = Menu::from_json("json-menu", json);
    assert!(result.is_ok());

    let menu = result.unwrap();
    assert_eq!(menu.items.len(), 2);
    assert_eq!(menu.items[0].id, "file");
    assert_eq!(menu.items[1].id, "edit");
  }

  #[test]
  fn test_radio_group() {
    let mut menu = Menu::builder("radio-menu")
      .item(MenuItem {
        id: "option1".to_string(),
        label: "Option 1".to_string(),
        item_type: MenuItemType::Radio {
          group: "group1".to_string(),
          selected: true,
        },
        icon: None,
        shortcut: None,
        enabled: true,
        visible: true,
        css_classes: Vec::new(),
        tooltip: None,
        data: HashMap::new(),
      })
      .item(MenuItem {
        id: "option2".to_string(),
        label: "Option 2".to_string(),
        item_type: MenuItemType::Radio {
          group: "group1".to_string(),
          selected: false,
        },
        icon: None,
        shortcut: None,
        enabled: true,
        visible: true,
        css_classes: Vec::new(),
        tooltip: None,
        data: HashMap::new(),
      })
      .build();

    // Select second option
    menu.next_item();
    menu.activate_selected();

    // Check radio group state
    assert_eq!(
      menu.state.radio_selections.get("group1"),
      Some(&"option2".to_string())
    );

    // Check item states updated
    if let MenuItemType::Radio { selected, .. } = &menu.state.current_items[0].item_type {
      assert!(!selected);
    }
    if let MenuItemType::Radio { selected, .. } = &menu.state.current_items[1].item_type {
      assert!(*selected);
    }
  }

  #[test]
  fn test_menu_rendering() {
    let menu = Menu::builder("render-test")
      .action("item1", "First Item", "action1")
      .separator("sep")
      .action("item2", "Second Item", "action2")
      .build();

    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 30,
      height: 6,
    };
    let rendered = menu.render(&layout, None);

    assert!(!rendered.is_empty());
    assert!(rendered.contains("First Item"));
    assert!(rendered.contains("Second Item"));
  }
}
