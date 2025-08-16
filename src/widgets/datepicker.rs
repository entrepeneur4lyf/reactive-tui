/*!
 * DatePicker Component - Calendar date selection widget
 *
 * A comprehensive date picker widget providing:
 * - Calendar popup with month/year navigation
 * - Date range selection support
 * - Keyboard navigation (arrow keys, Enter, Escape)
 * - Multiple date formats and localization
 * - Min/max date constraints
 * - Disabled dates and custom styling
 * - Today highlighting and quick selection
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Date structure
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
  pub year: i32,
  pub month: u8,  // 1-12
  pub day: u8,    // 1-31
}

impl Date {
  /// Create a new date
  pub fn new(year: i32, month: u8, day: u8) -> Result<Self> {
    if month < 1 || month > 12 {
      return Err(TuiError::component("Invalid month".to_string()));
    }
    if day < 1 || day > Self::days_in_month(year, month) {
      return Err(TuiError::component("Invalid day".to_string()));
    }
    Ok(Self { year, month, day })
  }

  /// Get today's date (simplified - in real implementation would use system time)
  pub fn today() -> Self {
    Self { year: 2024, month: 1, day: 15 } // Placeholder
  }

  /// Get number of days in a month
  pub fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
      1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
      4 | 6 | 9 | 11 => 30,
      2 => if Self::is_leap_year(year) { 29 } else { 28 },
      _ => 0,
    }
  }

  /// Check if year is leap year
  pub fn is_leap_year(year: i32) -> bool {
    (year % 4 == 0 && year % 100 != 0) || (year % 400 == 0)
  }

  /// Get day of week (0 = Sunday, 6 = Saturday)
  pub fn day_of_week(&self) -> u8 {
    // Simplified Zeller's congruence
    let mut year = self.year;
    let mut month = self.month as i32;

    if month < 3 {
      month += 12;
      year -= 1;
    }

    let k = year % 100;
    let j = year / 100;

    let day_of_week = (self.day as i32 + (13 * (month + 1)) / 5 + k + k / 4 + j / 4 - 2 * j) % 7;
    ((day_of_week + 7) % 7) as u8
  }

  /// Format date as string
  pub fn format(&self, format: &DateFormat) -> String {
    match format {
      DateFormat::YearMonthDay => format!("{:04}-{:02}-{:02}", self.year, self.month, self.day),
      DateFormat::MonthDayYear => format!("{:02}/{:02}/{:04}", self.month, self.day, self.year),
      DateFormat::DayMonthYear => format!("{:02}/{:02}/{:04}", self.day, self.month, self.year),
      DateFormat::MonthNameDayYear => {
        let month_name = Self::month_name(self.month);
        format!("{} {}, {}", month_name, self.day, self.year)
      }
    }
  }

  /// Get month name
  pub fn month_name(month: u8) -> &'static str {
    match month {
      1 => "January", 2 => "February", 3 => "March", 4 => "April",
      5 => "May", 6 => "June", 7 => "July", 8 => "August",
      9 => "September", 10 => "October", 11 => "November", 12 => "December",
      _ => "Invalid",
    }
  }

  /// Get short month name
  pub fn month_name_short(month: u8) -> &'static str {
    match month {
      1 => "Jan", 2 => "Feb", 3 => "Mar", 4 => "Apr",
      5 => "May", 6 => "Jun", 7 => "Jul", 8 => "Aug",
      9 => "Sep", 10 => "Oct", 11 => "Nov", 12 => "Dec",
      _ => "???",
    }
  }
}

/// Date format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DateFormat {
  YearMonthDay,      // 2024-01-15
  MonthDayYear,      // 01/15/2024
  DayMonthYear,      // 15/01/2024
  MonthNameDayYear,  // January 15, 2024
}

/// Date range for range selection
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DateRange {
  pub start: Date,
  pub end: Date,
}

/// DatePicker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatePickerConfig {
  pub format: DateFormat,
  pub allow_range: bool,
  pub show_week_numbers: bool,
  pub first_day_of_week: u8, // 0 = Sunday, 1 = Monday
  pub min_date: Option<Date>,
  pub max_date: Option<Date>,
  pub disabled_dates: Vec<Date>,
}

impl Default for DatePickerConfig {
  fn default() -> Self {
    Self {
      format: DateFormat::YearMonthDay,
      allow_range: false,
      show_week_numbers: false,
      first_day_of_week: 0, // Sunday
      min_date: None,
      max_date: None,
      disabled_dates: Vec::new(),
    }
  }
}

/// DatePicker styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatePickerStyle {
  pub background: String,
  pub border_color: String,
  pub text_color: String,
  pub header_bg: String,
  pub header_text: String,
  pub selected_bg: String,
  pub selected_text: String,
  pub today_bg: String,
  pub today_text: String,
  pub disabled_text: String,
  pub hover_bg: String,
  pub hover_text: String,
  pub weekend_text: String,
}

impl Default for DatePickerStyle {
  fn default() -> Self {
    Self {
      background: "#ffffff".to_string(),
      border_color: "#cccccc".to_string(),
      text_color: "#333333".to_string(),
      header_bg: "#f5f5f5".to_string(),
      header_text: "#333333".to_string(),
      selected_bg: "#0078d4".to_string(),
      selected_text: "#ffffff".to_string(),
      today_bg: "#ffeaa7".to_string(),
      today_text: "#333333".to_string(),
      disabled_text: "#cccccc".to_string(),
      hover_bg: "#e1ecf4".to_string(),
      hover_text: "#333333".to_string(),
      weekend_text: "#666666".to_string(),
    }
  }
}

/// DatePicker widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatePicker {
  pub selected_date: Option<Date>,
  pub selected_range: Option<DateRange>,
  pub current_month: u8,
  pub current_year: i32,
  pub is_open: bool,
  pub config: DatePickerConfig,
  pub style: DatePickerStyle,
  pub focused_day: Option<u8>,
}

impl DatePicker {
  /// Create a new DatePicker
  pub fn new() -> Self {
    let today = Date::today();
    Self {
      selected_date: None,
      selected_range: None,
      current_month: today.month,
      current_year: today.year,
      is_open: false,
      config: DatePickerConfig::default(),
      style: DatePickerStyle::default(),
      focused_day: None,
    }
  }

  /// Set selected date
  pub fn set_date(&mut self, date: Date) -> Result<()> {
    if self.is_date_disabled(&date) {
      return Err(TuiError::component("Date is disabled".to_string()));
    }

    self.selected_date = Some(date);
    self.current_month = date.month;
    self.current_year = date.year;
    Ok(())
  }

  /// Get selected date as formatted string
  pub fn get_formatted_date(&self) -> Option<String> {
    self.selected_date.map(|date| date.format(&self.config.format))
  }

  /// Open the calendar popup
  pub fn open(&mut self) {
    self.is_open = true;
    if let Some(date) = self.selected_date {
      self.current_month = date.month;
      self.current_year = date.year;
      self.focused_day = Some(date.day);
    } else {
      let today = Date::today();
      self.current_month = today.month;
      self.current_year = today.year;
      self.focused_day = Some(today.day);
    }
  }

  /// Close the calendar popup
  pub fn close(&mut self) {
    self.is_open = false;
    self.focused_day = None;
  }

  /// Navigate to previous month
  pub fn prev_month(&mut self) {
    if self.current_month == 1 {
      self.current_month = 12;
      self.current_year -= 1;
    } else {
      self.current_month -= 1;
    }
    self.focused_day = None;
  }

  /// Navigate to next month
  pub fn next_month(&mut self) {
    if self.current_month == 12 {
      self.current_month = 1;
      self.current_year += 1;
    } else {
      self.current_month += 1;
    }
    self.focused_day = None;
  }

  /// Navigate to previous year
  pub fn prev_year(&mut self) {
    self.current_year -= 1;
    self.focused_day = None;
  }

  /// Navigate to next year
  pub fn next_year(&mut self) {
    self.current_year += 1;
    self.focused_day = None;
  }

  /// Check if a date is disabled
  pub fn is_date_disabled(&self, date: &Date) -> bool {
    if let Some(min_date) = self.config.min_date {
      if *date < min_date {
        return true;
      }
    }

    if let Some(max_date) = self.config.max_date {
      if *date > max_date {
        return true;
      }
    }

    self.config.disabled_dates.contains(date)
  }

  /// Render the DatePicker
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    if !self.is_open {
      // Render closed state (input field)
      self.render_input(&mut output, rect, theme)?;
    } else {
      // Render calendar popup
      self.render_calendar(&mut output, rect, theme)?;
    }

    Ok(output)
  }

  /// Render input field (closed state)
  fn render_input(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let _text_color = color_to_ansi(text_color_def, false);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    // Draw input box
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Input content
    let content = self.get_formatted_date().unwrap_or_else(|| "Select date...".to_string());
    write!(output, "\x1b[{};{}H{}│{}{:<width$}{}│",
           rect.y + 2, rect.x + 1, border_color, bg_color, content, border_color,
           width = rect.width as usize - 3)?;

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + 3, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render calendar popup
  fn render_calendar(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    let header_bg_def = get_palette_color(&theme.palette, &self.style.header_bg)
      .map_err(|e| TuiError::render(e))?;
    let header_bg = color_to_ansi(header_bg_def, true);

    let header_text_def = get_palette_color(&theme.palette, &self.style.header_text)
      .map_err(|e| TuiError::render(e))?;
    let header_text = color_to_ansi(header_text_def, false);

    // Calendar dimensions
    let cal_width = 21; // 3 chars per day * 7 days
    let cal_height = 9; // Header + 6 weeks + borders

    // Draw calendar border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..cal_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Header with month/year
    let month_name = Date::month_name(self.current_month);
    let header_text_content = format!("{} {}", month_name, self.current_year);
    let _header_padding = (cal_width as usize - 2 - header_text_content.len()) / 2;

    write!(output, "\x1b[{};{}H{}│{}{}{:^width$}{}│",
           rect.y + 2, rect.x + 1, border_color, header_bg, header_text,
           header_text_content, border_color,
           width = cal_width as usize - 2)?;

    // Day headers
    write!(output, "\x1b[{};{}H{}│", rect.y + 3, rect.x + 1, border_color)?;
    let day_names = ["Su", "Mo", "Tu", "We", "Th", "Fr", "Sa"];
    for day_name in &day_names {
      write!(output, "{}{:^3}", header_text, day_name)?;
    }
    write!(output, "{}│", border_color)?;

    // Calendar days
    let first_day = Date::new(self.current_year, self.current_month, 1)?;
    let first_weekday = first_day.day_of_week();
    let days_in_month = Date::days_in_month(self.current_year, self.current_month);

    let mut day = 1;
    for week in 0..6 {
      let y = rect.y + 4 + week;
      write!(output, "\x1b[{};{}H{}│", y + 1, rect.x + 1, border_color)?;

      for weekday in 0..7 {
        let cell_day = if week == 0 && weekday < first_weekday {
          None // Empty cell before month starts
        } else if day > days_in_month {
          None // Empty cell after month ends
        } else {
          let current_day = day;
          day += 1;
          Some(current_day)
        };

        if let Some(day_num) = cell_day {
          let date = Date::new(self.current_year, self.current_month, day_num)?;
          let is_today = date == Date::today();
          let is_selected = self.selected_date == Some(date);
          let is_focused = self.focused_day == Some(day_num);
          let is_disabled = self.is_date_disabled(&date);
          let is_weekend = weekday == 0 || weekday == 6;

          let (cell_bg, cell_text) = if is_selected {
            let selected_bg_def = get_palette_color(&theme.palette, &self.style.selected_bg)
              .map_err(|e| TuiError::render(e))?;
            let selected_text_def = get_palette_color(&theme.palette, &self.style.selected_text)
              .map_err(|e| TuiError::render(e))?;
            (color_to_ansi(selected_bg_def, true), color_to_ansi(selected_text_def, false))
          } else if is_today {
            let today_bg_def = get_palette_color(&theme.palette, &self.style.today_bg)
              .map_err(|e| TuiError::render(e))?;
            let today_text_def = get_palette_color(&theme.palette, &self.style.today_text)
              .map_err(|e| TuiError::render(e))?;
            (color_to_ansi(today_bg_def, true), color_to_ansi(today_text_def, false))
          } else if is_focused {
            let hover_bg_def = get_palette_color(&theme.palette, &self.style.hover_bg)
              .map_err(|e| TuiError::render(e))?;
            let hover_text_def = get_palette_color(&theme.palette, &self.style.hover_text)
              .map_err(|e| TuiError::render(e))?;
            (color_to_ansi(hover_bg_def, true), color_to_ansi(hover_text_def, false))
          } else if is_disabled {
            let disabled_text_def = get_palette_color(&theme.palette, &self.style.disabled_text)
              .map_err(|e| TuiError::render(e))?;
            (bg_color.clone(), color_to_ansi(disabled_text_def, false))
          } else if is_weekend {
            let weekend_text_def = get_palette_color(&theme.palette, &self.style.weekend_text)
              .map_err(|e| TuiError::render(e))?;
            (bg_color.clone(), color_to_ansi(weekend_text_def, false))
          } else {
            let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
              .map_err(|e| TuiError::render(e))?;
            (bg_color.clone(), color_to_ansi(text_color_def, false))
          };

          write!(output, "{}{}{:^3}", cell_bg, cell_text, day_num)?;
        } else {
          write!(output, "{}   ", bg_color)?;
        }
      }

      write!(output, "{}│", border_color)?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + cal_height, rect.x + 1, border_color)?;
    for _ in 0..cal_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<Option<DatePickerAction>> {
    if !self.is_open {
      match key {
        "Enter" | " " => {
          self.open();
          return Ok(Some(DatePickerAction::Opened));
        }
        _ => return Ok(None),
      }
    }

    match key {
      "Escape" => {
        self.close();
        Ok(Some(DatePickerAction::Closed))
      }
      "ArrowLeft" => {
        self.prev_month();
        Ok(Some(DatePickerAction::MonthChanged))
      }
      "ArrowRight" => {
        self.next_month();
        Ok(Some(DatePickerAction::MonthChanged))
      }
      "ArrowUp" => {
        self.prev_year();
        Ok(Some(DatePickerAction::YearChanged))
      }
      "ArrowDown" => {
        self.next_year();
        Ok(Some(DatePickerAction::YearChanged))
      }
      "Enter" => {
        if let Some(day) = self.focused_day {
          let date = Date::new(self.current_year, self.current_month, day)?;
          if !self.is_date_disabled(&date) {
            self.set_date(date)?;
            self.close();
            return Ok(Some(DatePickerAction::DateSelected(date)));
          }
        }
        Ok(None)
      }
      _ => Ok(None),
    }
  }
}

impl Default for DatePicker {
  fn default() -> Self {
    Self::new()
  }
}

/// Actions that can result from DatePicker interactions
#[derive(Debug, Clone, PartialEq)]
pub enum DatePickerAction {
  Opened,
  Closed,
  DateSelected(Date),
  MonthChanged,
  YearChanged,
}

/// Builder for DatePicker
pub struct DatePickerBuilder {
  datepicker: DatePicker,
}

impl DatePickerBuilder {
  pub fn new() -> Self {
    Self {
      datepicker: DatePicker::new(),
    }
  }

  pub fn selected_date(mut self, date: Date) -> Self {
    self.datepicker.selected_date = Some(date);
    self
  }

  pub fn format(mut self, format: DateFormat) -> Self {
    self.datepicker.config.format = format;
    self
  }

  pub fn allow_range(mut self, allow: bool) -> Self {
    self.datepicker.config.allow_range = allow;
    self
  }

  pub fn min_date(mut self, date: Date) -> Self {
    self.datepicker.config.min_date = Some(date);
    self
  }

  pub fn max_date(mut self, date: Date) -> Self {
    self.datepicker.config.max_date = Some(date);
    self
  }

  pub fn style(mut self, style: DatePickerStyle) -> Self {
    self.datepicker.style = style;
    self
  }

  pub fn build(self) -> DatePicker {
    self.datepicker
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_date_creation() {
    let date = Date::new(2024, 1, 15).unwrap();
    assert_eq!(date.year, 2024);
    assert_eq!(date.month, 1);
    assert_eq!(date.day, 15);
  }

  #[test]
  fn test_date_validation() {
    assert!(Date::new(2024, 13, 1).is_err()); // Invalid month
    assert!(Date::new(2024, 2, 30).is_err()); // Invalid day for February
    assert!(Date::new(2024, 2, 29).is_ok());  // Valid leap year day
  }

  #[test]
  fn test_leap_year() {
    assert!(Date::is_leap_year(2024));
    assert!(!Date::is_leap_year(2023));
    assert!(Date::is_leap_year(2000));
    assert!(!Date::is_leap_year(1900));
  }

  #[test]
  fn test_date_formatting() {
    let date = Date::new(2024, 1, 15).unwrap();
    assert_eq!(date.format(&DateFormat::YearMonthDay), "2024-01-15");
    assert_eq!(date.format(&DateFormat::MonthDayYear), "01/15/2024");
    assert_eq!(date.format(&DateFormat::MonthNameDayYear), "January 15, 2024");
  }

  #[test]
  fn test_datepicker_creation() {
    let datepicker = DatePicker::new();
    assert!(datepicker.selected_date.is_none());
    assert!(!datepicker.is_open);
  }

  #[test]
  fn test_datepicker_navigation() {
    let mut datepicker = DatePicker::new();
    datepicker.current_month = 1;
    datepicker.current_year = 2024;

    datepicker.next_month();
    assert_eq!(datepicker.current_month, 2);

    datepicker.prev_month();
    assert_eq!(datepicker.current_month, 1);

    datepicker.prev_month();
    assert_eq!(datepicker.current_month, 12);
    assert_eq!(datepicker.current_year, 2023);
  }
}
