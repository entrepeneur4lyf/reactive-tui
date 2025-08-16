/*!
 * Pagination Component - Page navigation controls
 *
 * A comprehensive pagination widget providing:
 * - Page number navigation with ellipsis
 * - First/Last/Previous/Next controls
 * - Customizable page size and display options
 * - Keyboard navigation support
 * - Flexible styling and theming
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Pagination widget for navigating through pages
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pagination {
  /// Current page (1-based)
  pub current_page: usize,
  /// Total number of pages
  pub total_pages: usize,
  /// Number of page buttons to show around current page
  pub visible_pages: usize,
  /// Items per page
  pub page_size: usize,
  /// Total number of items
  pub total_items: usize,
  /// Styling configuration
  pub style: PaginationStyle,
  /// Show first/last buttons
  pub show_first_last: bool,
  /// Show previous/next buttons
  pub show_prev_next: bool,
  /// Show page info (e.g., "Page 1 of 10")
  pub show_page_info: bool,
  /// Show items info (e.g., "1-10 of 100 items")
  pub show_items_info: bool,
}

/// Pagination styling configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PaginationStyle {
  /// Background color
  pub background: String,
  /// Text color for regular buttons
  pub text_color: String,
  /// Background color for active page
  pub active_background: String,
  /// Text color for active page
  pub active_text_color: String,
  /// Background color for hover state
  pub hover_background: String,
  /// Text color for hover state
  pub hover_text_color: String,
  /// Text color for disabled buttons
  pub disabled_text_color: String,
  /// Border color
  pub border_color: String,
  /// Button padding
  pub button_padding: u16,
  /// Spacing between buttons
  pub button_spacing: u16,
}

impl Default for PaginationStyle {
  fn default() -> Self {
    Self {
      background: "transparent".to_string(),
      text_color: "#0078d4".to_string(),
      active_background: "#0078d4".to_string(),
      active_text_color: "#ffffff".to_string(),
      hover_background: "#f0f0f0".to_string(),
      hover_text_color: "#0078d4".to_string(),
      disabled_text_color: "#cccccc".to_string(),
      border_color: "#cccccc".to_string(),
      button_padding: 1,
      button_spacing: 1,
    }
  }
}

impl Pagination {
  /// Create a new Pagination widget
  pub fn new(total_items: usize, page_size: usize) -> Self {
    let total_pages = (total_items + page_size - 1) / page_size.max(1);

    Self {
      current_page: 1,
      total_pages,
      visible_pages: 5,
      page_size,
      total_items,
      style: PaginationStyle::default(),
      show_first_last: true,
      show_prev_next: true,
      show_page_info: false,
      show_items_info: true,
    }
  }

  /// Set current page
  pub fn set_page(&mut self, page: usize) -> Result<()> {
    if page == 0 || page > self.total_pages {
      return Err(TuiError::component("Invalid page number".to_string()));
    }
    self.current_page = page;
    Ok(())
  }

  /// Go to next page
  pub fn next_page(&mut self) -> Result<bool> {
    if self.current_page < self.total_pages {
      self.current_page += 1;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// Go to previous page
  pub fn prev_page(&mut self) -> Result<bool> {
    if self.current_page > 1 {
      self.current_page -= 1;
      Ok(true)
    } else {
      Ok(false)
    }
  }

  /// Go to first page
  pub fn first_page(&mut self) {
    self.current_page = 1;
  }

  /// Go to last page
  pub fn last_page(&mut self) {
    self.current_page = self.total_pages;
  }

  /// Update total items and recalculate pages
  pub fn update_total_items(&mut self, total_items: usize) {
    self.total_items = total_items;
    self.total_pages = (total_items + self.page_size - 1) / self.page_size.max(1);

    // Adjust current page if it's now out of range
    if self.current_page > self.total_pages && self.total_pages > 0 {
      self.current_page = self.total_pages;
    } else if self.total_pages == 0 {
      self.current_page = 1;
    }
  }

  /// Get the range of items for current page
  pub fn get_item_range(&self) -> (usize, usize) {
    let start = (self.current_page - 1) * self.page_size + 1;
    let end = (self.current_page * self.page_size).min(self.total_items);
    (start, end)
  }

  /// Calculate which page numbers to display
  fn calculate_visible_pages(&self) -> Vec<PageButton> {
    let mut buttons = Vec::new();

    if self.total_pages == 0 {
      return buttons;
    }

    // Add first page button if needed
    if self.show_first_last && self.current_page > self.visible_pages / 2 + 1 {
      buttons.push(PageButton::Page(1));
      if self.current_page > self.visible_pages / 2 + 2 {
        buttons.push(PageButton::Ellipsis);
      }
    }

    // Calculate start and end of visible page range
    let half_visible = self.visible_pages / 2;
    let start = (self.current_page.saturating_sub(half_visible)).max(1);
    let end = (self.current_page + half_visible).min(self.total_pages);

    // Add visible page numbers
    for page in start..=end {
      buttons.push(PageButton::Page(page));
    }

    // Add last page button if needed
    if self.show_first_last && self.current_page + half_visible < self.total_pages {
      if self.current_page + half_visible + 1 < self.total_pages {
        buttons.push(PageButton::Ellipsis);
      }
      buttons.push(PageButton::Page(self.total_pages));
    }

    buttons
  }

  /// Render the pagination widget
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    if self.total_pages <= 1 {
      return Ok(output); // Don't show pagination for single page
    }

    // Position cursor
    write!(output, "\x1b[{};{}H", rect.y + 1, rect.x + 1)?;

    let mut x_offset = 0u16;

    // Show items info if enabled
    if self.show_items_info {
      let (start, end) = self.get_item_range();
      let info_text = format!("{}-{} of {} items", start, end, self.total_items);
      let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
        .map_err(|e| TuiError::render(e))?;
      let text_color = color_to_ansi(text_color_def, false);
      write!(output, "{}{}", text_color, info_text)?;
      x_offset += info_text.len() as u16 + 2;
    }

    // Position for buttons
    write!(output, "\x1b[{};{}H", rect.y + 1, rect.x + x_offset + 1)?;

    // Previous button
    if self.show_prev_next {
      let is_disabled = self.current_page == 1;
      self.render_button(&mut output, "‹ Prev", is_disabled, false, theme)?;
      x_offset += 8 + self.style.button_spacing;
    }

    // Page number buttons
    let visible_pages = self.calculate_visible_pages();
    for button in visible_pages {
      match button {
        PageButton::Page(page) => {
          let is_active = page == self.current_page;
          let button_text = page.to_string();
          self.render_button(&mut output, &button_text, false, is_active, theme)?;
          x_offset += button_text.len() as u16 + self.style.button_padding * 2 + self.style.button_spacing;
        }
        PageButton::Ellipsis => {
          let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
            .map_err(|e| TuiError::render(e))?;
          let text_color = color_to_ansi(text_color_def, false);
          write!(output, "{}...", text_color)?;
          x_offset += 3 + self.style.button_spacing;
        }
      }
    }

    // Next button
    if self.show_prev_next {
      let is_disabled = self.current_page == self.total_pages;
      self.render_button(&mut output, "Next ›", is_disabled, false, theme)?;
    }

    // Show page info if enabled
    if self.show_page_info {
      let page_info = format!(" Page {} of {}", self.current_page, self.total_pages);
      let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
        .map_err(|e| TuiError::render(e))?;
      let text_color = color_to_ansi(text_color_def, false);
      write!(output, "{}{}", text_color, page_info)?;
    }

    // Reset colors
    write!(output, "\x1b[0m")?;

    Ok(output)
  }

  /// Render a single button
  fn render_button(
    &self,
    output: &mut String,
    text: &str,
    is_disabled: bool,
    is_active: bool,
    theme: &ColorTheme,
  ) -> Result<()> {
    let (bg_color, text_color) = if is_disabled {
      let bg_def = get_palette_color(&theme.palette, &self.style.background)
        .map_err(|e| TuiError::render(e))?;
      let disabled_text_def = get_palette_color(&theme.palette, &self.style.disabled_text_color)
        .map_err(|e| TuiError::render(e))?;
      (
        color_to_ansi(bg_def, true),
        color_to_ansi(disabled_text_def, false),
      )
    } else if is_active {
      let active_bg_def = get_palette_color(&theme.palette, &self.style.active_background)
        .map_err(|e| TuiError::render(e))?;
      let active_text_def = get_palette_color(&theme.palette, &self.style.active_text_color)
        .map_err(|e| TuiError::render(e))?;
      (
        color_to_ansi(active_bg_def, true),
        color_to_ansi(active_text_def, false),
      )
    } else {
      let bg_def = get_palette_color(&theme.palette, &self.style.background)
        .map_err(|e| TuiError::render(e))?;
      let text_def = get_palette_color(&theme.palette, &self.style.text_color)
        .map_err(|e| TuiError::render(e))?;
      (
        color_to_ansi(bg_def, true),
        color_to_ansi(text_def, false),
      )
    };

    // Render button with padding
    write!(output, "{}{}", bg_color, text_color)?;
    for _ in 0..self.style.button_padding {
      write!(output, " ")?;
    }
    write!(output, "{}", text)?;
    for _ in 0..self.style.button_padding {
      write!(output, " ")?;
    }

    // Add spacing
    for _ in 0..self.style.button_spacing {
      write!(output, " ")?;
    }

    Ok(())
  }

  /// Handle click at position
  pub fn handle_click(&mut self, x: u16, y: u16, rect: LayoutRect) -> Result<Option<PaginationAction>> {
    if y != rect.y {
      return Ok(None); // Click not on pagination line
    }

    // This is a simplified click handler - in a real implementation,
    // you'd need to track button positions more precisely
    let click_x = x.saturating_sub(rect.x);

    // For now, just handle basic navigation based on approximate positions
    if click_x < 10 && self.show_prev_next {
      // Previous button area
      if self.current_page > 1 {
        self.prev_page()?;
        return Ok(Some(PaginationAction::PageChanged(self.current_page)));
      }
    } else if click_x > rect.width - 10 && self.show_prev_next {
      // Next button area
      if self.current_page < self.total_pages {
        self.next_page()?;
        return Ok(Some(PaginationAction::PageChanged(self.current_page)));
      }
    }

    Ok(None)
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: char) -> Result<Option<PaginationAction>> {
    match key {
      // Arrow keys for navigation
      '←' | 'h' => {
        if self.prev_page()? {
          Ok(Some(PaginationAction::PageChanged(self.current_page)))
        } else {
          Ok(None)
        }
      }
      '→' | 'l' => {
        if self.next_page()? {
          Ok(Some(PaginationAction::PageChanged(self.current_page)))
        } else {
          Ok(None)
        }
      }
      // Home/End for first/last page
      '↖' => {
        self.first_page();
        Ok(Some(PaginationAction::PageChanged(self.current_page)))
      }
      '↘' => {
        self.last_page();
        Ok(Some(PaginationAction::PageChanged(self.current_page)))
      }
      // Number keys for direct page navigation
      c if c.is_ascii_digit() => {
        let digit = c.to_digit(10).unwrap() as usize;
        if digit > 0 && digit <= self.total_pages {
          self.set_page(digit)?;
          Ok(Some(PaginationAction::PageChanged(self.current_page)))
        } else {
          Ok(None)
        }
      }
      _ => Ok(None),
    }
  }
}

/// Page button types
#[derive(Debug, Clone, PartialEq)]
enum PageButton {
  Page(usize),
  Ellipsis,
}

/// Actions that can result from pagination interactions
#[derive(Debug, Clone, PartialEq)]
pub enum PaginationAction {
  PageChanged(usize),
}

/// Builder for Pagination
pub struct PaginationBuilder {
  pagination: Pagination,
}

impl PaginationBuilder {
  pub fn new(total_items: usize, page_size: usize) -> Self {
    Self {
      pagination: Pagination::new(total_items, page_size),
    }
  }

  pub fn visible_pages(mut self, count: usize) -> Self {
    self.pagination.visible_pages = count;
    self
  }

  pub fn show_first_last(mut self, show: bool) -> Self {
    self.pagination.show_first_last = show;
    self
  }

  pub fn show_prev_next(mut self, show: bool) -> Self {
    self.pagination.show_prev_next = show;
    self
  }

  pub fn show_page_info(mut self, show: bool) -> Self {
    self.pagination.show_page_info = show;
    self
  }

  pub fn show_items_info(mut self, show: bool) -> Self {
    self.pagination.show_items_info = show;
    self
  }

  pub fn style(mut self, style: PaginationStyle) -> Self {
    self.pagination.style = style;
    self
  }

  pub fn build(self) -> Pagination {
    self.pagination
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_pagination_creation() {
    let pagination = Pagination::new(100, 10);
    assert_eq!(pagination.total_pages, 10);
    assert_eq!(pagination.current_page, 1);
    assert_eq!(pagination.total_items, 100);
    assert_eq!(pagination.page_size, 10);
  }

  #[test]
  fn test_pagination_navigation() {
    let mut pagination = Pagination::new(100, 10);

    // Test next page
    assert!(pagination.next_page().unwrap());
    assert_eq!(pagination.current_page, 2);

    // Test previous page
    assert!(pagination.prev_page().unwrap());
    assert_eq!(pagination.current_page, 1);

    // Test first/last page
    pagination.last_page();
    assert_eq!(pagination.current_page, 10);

    pagination.first_page();
    assert_eq!(pagination.current_page, 1);
  }

  #[test]
  fn test_pagination_item_range() {
    let mut pagination = Pagination::new(95, 10);

    // First page
    let (start, end) = pagination.get_item_range();
    assert_eq!((start, end), (1, 10));

    // Last page (partial)
    pagination.set_page(10).unwrap();
    let (start, end) = pagination.get_item_range();
    assert_eq!((start, end), (91, 95));
  }

  #[test]
  fn test_pagination_update_total() {
    let mut pagination = Pagination::new(100, 10);
    assert_eq!(pagination.total_pages, 10);

    pagination.update_total_items(150);
    assert_eq!(pagination.total_pages, 15);
    assert_eq!(pagination.total_items, 150);
  }
}
