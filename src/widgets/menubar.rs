/*!
 * MenuBar Component - Top-level navigation bar with keyboard shortcuts
 *
 * A comprehensive menu bar widget providing:
 * - Horizontal menu layout with dropdown submenus
 * - Full keyboard navigation (Alt+key, arrow keys)
 * - Mnemonic support for quick access
 * - Context-sensitive menu items
 * - Customizable styling and theming
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
  widgets::menu::{MenuItem, MenuItemType},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::Write;

/// MenuBar widget for top-level navigation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuBar {
  /// Menu items in the bar
  pub items: Vec<MenuBarItem>,
  /// Currently active menu index
  pub active_index: Option<usize>,
  /// Whether a submenu is currently open
  pub submenu_open: bool,
  /// Styling configuration
  pub style: MenuBarStyle,
  /// Keyboard shortcuts map
  pub shortcuts: HashMap<char, usize>,
}

/// Individual menu bar item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuBarItem {
  /// Display label
  pub label: String,
  /// Mnemonic character (underlined)
  pub mnemonic: Option<char>,
  /// Submenu items
  pub submenu: Vec<MenuItem>,
  /// Whether item is enabled
  pub enabled: bool,
  /// Custom styling
  pub style_class: Option<String>,
}

/// MenuBar styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MenuBarStyle {
  /// Background color
  pub background: String,
  /// Text color
  pub text_color: String,
  /// Active item background
  pub active_background: String,
  /// Active item text color
  pub active_text_color: String,
  /// Disabled item text color
  pub disabled_text_color: String,
  /// Mnemonic underline color
  pub mnemonic_color: String,
  /// Border style
  pub border: Option<String>,
  /// Padding
  pub padding: u16,
}

impl Default for MenuBarStyle {
  fn default() -> Self {
    Self {
      background: "#f0f0f0".to_string(),
      text_color: "#000000".to_string(),
      active_background: "#0078d4".to_string(),
      active_text_color: "#ffffff".to_string(),
      disabled_text_color: "#888888".to_string(),
      mnemonic_color: "#0078d4".to_string(),
      border: Some("1px solid #cccccc".to_string()),
      padding: 1,
    }
  }
}

impl MenuBar {
  /// Create a new MenuBar
  pub fn new() -> Self {
    Self {
      items: Vec::new(),
      active_index: None,
      submenu_open: false,
      style: MenuBarStyle::default(),
      shortcuts: HashMap::new(),
    }
  }

  /// Add a menu item to the bar
  pub fn add_item(&mut self, item: MenuBarItem) -> &mut Self {
    // Register mnemonic shortcut
    if let Some(mnemonic) = item.mnemonic {
      self.shortcuts.insert(mnemonic.to_ascii_lowercase(), self.items.len());
    }

    self.items.push(item);
    self
  }

  /// Set active menu by index
  pub fn set_active(&mut self, index: Option<usize>) -> Result<()> {
    if let Some(idx) = index {
      if idx >= self.items.len() {
        return Err(TuiError::component("Invalid menu index".to_string()));
      }
    }
    self.active_index = index;
    Ok(())
  }

  /// Navigate to next menu item
  pub fn navigate_next(&mut self) {
    if self.items.is_empty() {
      return;
    }

    self.active_index = Some(match self.active_index {
      Some(idx) => (idx + 1) % self.items.len(),
      None => 0,
    });
  }

  /// Navigate to previous menu item
  pub fn navigate_previous(&mut self) {
    if self.items.is_empty() {
      return;
    }

    self.active_index = Some(match self.active_index {
      Some(idx) => {
        if idx == 0 {
          self.items.len() - 1
        } else {
          idx - 1
        }
      }
      None => self.items.len() - 1,
    });
  }

  /// Activate menu by mnemonic key
  pub fn activate_by_mnemonic(&mut self, key: char) -> bool {
    if let Some(&index) = self.shortcuts.get(&key.to_ascii_lowercase()) {
      self.active_index = Some(index);
      self.submenu_open = true;
      true
    } else {
      false
    }
  }

  /// Toggle submenu open/closed
  pub fn toggle_submenu(&mut self) {
    self.submenu_open = !self.submenu_open;
  }

  /// Close submenu
  pub fn close_submenu(&mut self) {
    self.submenu_open = false;
    self.active_index = None;
  }

  /// Get currently active item
  pub fn get_active_item(&self) -> Option<&MenuBarItem> {
    self.active_index.and_then(|idx| self.items.get(idx))
  }

  /// Render the menu bar
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    // Apply background color
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    // Move to position and set background
    write!(output, "\x1b[{};{}H{}", rect.y + 1, rect.x + 1, bg_color)?;

    // Clear the line with background color
    write!(output, "\x1b[K")?;

    let mut x_offset = self.style.padding;

    for (index, item) in self.items.iter().enumerate() {
      let is_active = self.active_index == Some(index);

      // Set colors for this item
      let (item_bg, item_fg) = if is_active {
        let active_bg_def = get_palette_color(&theme.palette, &self.style.active_background)
          .map_err(|e| TuiError::render(e))?;
        let active_text_def = get_palette_color(&theme.palette, &self.style.active_text_color)
          .map_err(|e| TuiError::render(e))?;
        (
          color_to_ansi(active_bg_def, true),
          color_to_ansi(active_text_def, false),
        )
      } else if !item.enabled {
        let disabled_text_def = get_palette_color(&theme.palette, &self.style.disabled_text_color)
          .map_err(|e| TuiError::render(e))?;
        (
          bg_color.clone(),
          color_to_ansi(disabled_text_def, false),
        )
      } else {
        (bg_color.clone(), text_color.clone())
      };

      // Position cursor and apply colors
      write!(output, "\x1b[{};{}H{}{}", rect.y + 1, rect.x + x_offset + 1, item_bg, item_fg)?;

      // Render label with mnemonic highlighting
      if let Some(mnemonic) = item.mnemonic {
        let mnemonic_color_def = get_palette_color(&theme.palette, &self.style.mnemonic_color)
          .map_err(|e| TuiError::render(e))?;
        let mnemonic_color = color_to_ansi(mnemonic_color_def, false);
        let label = self.render_label_with_mnemonic(&item.label, mnemonic, &mnemonic_color, &item_fg)?;
        write!(output, " {} ", label)?;
      } else {
        write!(output, " {} ", item.label)?;
      }

      x_offset += item.label.len() as u16 + 2 + self.style.padding;

      // Add separator if not last item
      if index < self.items.len() - 1 {
        write!(output, "{} ", text_color)?;
        x_offset += 1;
      }
    }

    // Reset colors
    write!(output, "\x1b[0m")?;

    // Render submenu if open
    if self.submenu_open {
      if let Some(active_item) = self.get_active_item() {
        let submenu_rect = LayoutRect {
          x: rect.x,
          y: rect.y + 1,
          width: rect.width,
          height: active_item.submenu.len() as u16 + 2,
        };

        let submenu_output = self.render_submenu(&active_item.submenu, submenu_rect, theme)?;
        write!(output, "{}", submenu_output)?;
      }
    }

    Ok(output)
  }

  /// Render label with mnemonic highlighting
  fn render_label_with_mnemonic(
    &self,
    label: &str,
    mnemonic: char,
    mnemonic_color: &str,
    default_color: &str,
  ) -> Result<String> {
    let mut result = String::new();
    let mnemonic_lower = mnemonic.to_ascii_lowercase();

    for ch in label.chars() {
      if ch.to_ascii_lowercase() == mnemonic_lower {
        write!(result, "{}\x1b[4m{}\x1b[24m{}", mnemonic_color, ch, default_color)?;
      } else {
        write!(result, "{}", ch)?;
      }
    }

    Ok(result)
  }

  /// Render dropdown submenu
  fn render_submenu(
    &self,
    items: &[MenuItem],
    rect: LayoutRect,
    theme: &ColorTheme,
  ) -> Result<String> {
    let mut output = String::new();

    // Draw submenu background
    let bg_color_def = get_palette_color(&theme.palette, "background")
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, "foreground")
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let border_color_def = get_palette_color(&theme.palette, "border")
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    // Draw top border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Draw menu items
    for (index, item) in items.iter().enumerate() {
      let y = rect.y + 2 + index as u16;
      write!(output, "\x1b[{};{}H{}│{}", y + 1, rect.x + 1, border_color, bg_color)?;

      match &item.item_type {
        MenuItemType::Action { action: _ } => {
          write!(output, "{} {}", text_color, item.label)?;
        }
        MenuItemType::Submenu { items: _ } => {
          write!(output, "{} {} ►", text_color, item.label)?;
        }
        MenuItemType::Separator => {
          write!(output, "{}├", border_color)?;
          for _ in 0..rect.width - 2 {
            write!(output, "─")?;
          }
          write!(output, "┤")?;
          continue;
        }
        MenuItemType::Header => {
          write!(output, "{} {}", text_color, item.label)?;
        }
        MenuItemType::Toggle { state } => {
          let checkbox = if *state { "☑" } else { "☐" };
          write!(output, "{} {} {}", text_color, checkbox, item.label)?;
        }
        MenuItemType::Radio { group: _, selected } => {
          let radio = if *selected { "●" } else { "○" };
          write!(output, "{} {} {}", text_color, radio, item.label)?;
        }
      }

      // Pad to full width
      let content_width = item.label.len() + 2;
      for _ in content_width..rect.width as usize - 2 {
        write!(output, " ")?;
      }
      write!(output, "{}│", border_color)?;
    }

    // Draw bottom border
    let y = rect.y + 2 + items.len() as u16;
    write!(output, "\x1b[{};{}H{}└", y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(output)
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: char) -> Result<MenuBarAction> {
    match key {
      // Escape - close menu or handle arrow key sequences
      '\x1b' => {
        if self.submenu_open {
          self.close_submenu();
          Ok(MenuBarAction::MenuClosed)
        } else {
          // This would be handled by the parent for arrow key sequences
          Ok(MenuBarAction::None)
        }
      }
      // Enter - activate current menu
      '\r' | '\n' => {
        if self.active_index.is_some() {
          self.toggle_submenu();
          Ok(MenuBarAction::SubmenuToggled)
        } else {
          Ok(MenuBarAction::None)
        }
      }
      // Mnemonic activation
      c if c.is_ascii_alphabetic() => {
        if self.activate_by_mnemonic(c) {
          Ok(MenuBarAction::MenuActivated(self.active_index.unwrap()))
        } else {
          Ok(MenuBarAction::None)
        }
      }
      _ => Ok(MenuBarAction::None),
    }
  }
}

/// Actions that can result from MenuBar interactions
#[derive(Debug, Clone, PartialEq)]
pub enum MenuBarAction {
  None,
  MenuActivated(usize),
  SubmenuToggled,
  MenuClosed,
  ItemSelected(String),
}

impl Default for MenuBar {
  fn default() -> Self {
    Self::new()
  }
}

/// Builder for MenuBarItem
pub struct MenuBarItemBuilder {
  label: String,
  mnemonic: Option<char>,
  submenu: Vec<MenuItem>,
  enabled: bool,
  style_class: Option<String>,
}

impl MenuBarItemBuilder {
  pub fn new(label: impl Into<String>) -> Self {
    Self {
      label: label.into(),
      mnemonic: None,
      submenu: Vec::new(),
      enabled: true,
      style_class: None,
    }
  }

  pub fn mnemonic(mut self, mnemonic: char) -> Self {
    self.mnemonic = Some(mnemonic);
    self
  }

  pub fn submenu(mut self, items: Vec<MenuItem>) -> Self {
    self.submenu = items;
    self
  }

  pub fn enabled(mut self, enabled: bool) -> Self {
    self.enabled = enabled;
    self
  }

  pub fn style_class(mut self, class: impl Into<String>) -> Self {
    self.style_class = Some(class.into());
    self
  }

  pub fn build(self) -> MenuBarItem {
    MenuBarItem {
      label: self.label,
      mnemonic: self.mnemonic,
      submenu: self.submenu,
      enabled: self.enabled,
      style_class: self.style_class,
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_menubar_creation() {
    let menubar = MenuBar::new();
    assert!(menubar.items.is_empty());
    assert_eq!(menubar.active_index, None);
    assert!(!menubar.submenu_open);
  }

  #[test]
  fn test_menubar_navigation() {
    let mut menubar = MenuBar::new();

    // Add test items
    menubar.add_item(MenuBarItemBuilder::new("File").mnemonic('F').build());
    menubar.add_item(MenuBarItemBuilder::new("Edit").mnemonic('E').build());
    menubar.add_item(MenuBarItemBuilder::new("View").mnemonic('V').build());

    // Test navigation
    menubar.navigate_next();
    assert_eq!(menubar.active_index, Some(0));

    menubar.navigate_next();
    assert_eq!(menubar.active_index, Some(1));

    menubar.navigate_previous();
    assert_eq!(menubar.active_index, Some(0));
  }

  #[test]
  fn test_mnemonic_activation() {
    let mut menubar = MenuBar::new();

    menubar.add_item(MenuBarItemBuilder::new("File").mnemonic('F').build());
    menubar.add_item(MenuBarItemBuilder::new("Edit").mnemonic('E').build());

    assert!(menubar.activate_by_mnemonic('f'));
    assert_eq!(menubar.active_index, Some(0));
    assert!(menubar.submenu_open);

    assert!(menubar.activate_by_mnemonic('e'));
    assert_eq!(menubar.active_index, Some(1));
  }
}
