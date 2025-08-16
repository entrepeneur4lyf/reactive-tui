/*!
 * TimePicker Component - Time selection widget
 *
 * A comprehensive time picker widget providing:
 * - Hour, minute, and second selection
 * - 12-hour and 24-hour formats
 * - Keyboard navigation and input
 * - Time range constraints
 * - Custom time intervals (e.g., 15-minute increments)
 * - AM/PM toggle for 12-hour format
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Time structure
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct Time {
  pub hour: u8,    // 0-23
  pub minute: u8,  // 0-59
  pub second: u8,  // 0-59
}

impl Time {
  /// Create a new time
  pub fn new(hour: u8, minute: u8, second: u8) -> Result<Self> {
    if hour > 23 {
      return Err(TuiError::component("Invalid hour".to_string()));
    }
    if minute > 59 {
      return Err(TuiError::component("Invalid minute".to_string()));
    }
    if second > 59 {
      return Err(TuiError::component("Invalid second".to_string()));
    }
    Ok(Self { hour, minute, second })
  }

  /// Get current time (simplified - in real implementation would use system time)
  pub fn now() -> Self {
    Self { hour: 12, minute: 0, second: 0 } // Placeholder
  }

  /// Convert to 12-hour format
  pub fn to_12_hour(&self) -> (u8, bool) {
    let is_pm = self.hour >= 12;
    let hour_12 = if self.hour == 0 {
      12
    } else if self.hour > 12 {
      self.hour - 12
    } else {
      self.hour
    };
    (hour_12, is_pm)
  }

  /// Create from 12-hour format
  pub fn from_12_hour(hour: u8, minute: u8, second: u8, is_pm: bool) -> Result<Self> {
    if hour < 1 || hour > 12 {
      return Err(TuiError::component("Invalid 12-hour format hour".to_string()));
    }

    let hour_24 = if hour == 12 {
      if is_pm { 12 } else { 0 }
    } else {
      if is_pm { hour + 12 } else { hour }
    };

    Self::new(hour_24, minute, second)
  }

  /// Format time as string
  pub fn format(&self, format: &TimeFormat) -> String {
    match format {
      TimeFormat::Hour24Minute => format!("{:02}:{:02}", self.hour, self.minute),
      TimeFormat::Hour24MinuteSecond => format!("{:02}:{:02}:{:02}", self.hour, self.minute, self.second),
      TimeFormat::Hour12Minute => {
        let (hour_12, is_pm) = self.to_12_hour();
        format!("{:02}:{:02} {}", hour_12, self.minute, if is_pm { "PM" } else { "AM" })
      }
      TimeFormat::Hour12MinuteSecond => {
        let (hour_12, is_pm) = self.to_12_hour();
        format!("{:02}:{:02}:{:02} {}", hour_12, self.minute, self.second, if is_pm { "PM" } else { "AM" })
      }
    }
  }

  /// Parse time from string
  pub fn parse(s: &str, format: &TimeFormat) -> Result<Self> {
    match format {
      TimeFormat::Hour24Minute => {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 2 {
          return Err(TuiError::component("Invalid time format".to_string()));
        }
        let hour = parts[0].parse().map_err(|_| TuiError::component("Invalid hour".to_string()))?;
        let minute = parts[1].parse().map_err(|_| TuiError::component("Invalid minute".to_string()))?;
        Self::new(hour, minute, 0)
      }
      TimeFormat::Hour24MinuteSecond => {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() != 3 {
          return Err(TuiError::component("Invalid time format".to_string()));
        }
        let hour = parts[0].parse().map_err(|_| TuiError::component("Invalid hour".to_string()))?;
        let minute = parts[1].parse().map_err(|_| TuiError::component("Invalid minute".to_string()))?;
        let second = parts[2].parse().map_err(|_| TuiError::component("Invalid second".to_string()))?;
        Self::new(hour, minute, second)
      }
      TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond => {
        let (time_part, am_pm) = if s.contains(" AM") {
          (s.replace(" AM", ""), false)
        } else if s.contains(" PM") {
          (s.replace(" PM", ""), true)
        } else {
          return Err(TuiError::component("Missing AM/PM indicator".to_string()));
        };

        let parts: Vec<&str> = time_part.split(':').collect();
        let hour = parts[0].parse().map_err(|_| TuiError::component("Invalid hour".to_string()))?;
        let minute = parts[1].parse().map_err(|_| TuiError::component("Invalid minute".to_string()))?;
        let second = if parts.len() > 2 {
          parts[2].parse().map_err(|_| TuiError::component("Invalid second".to_string()))?
        } else {
          0
        };

        Self::from_12_hour(hour, minute, second, am_pm)
      }
    }
  }
}

/// Time format options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TimeFormat {
  Hour24Minute,        // 14:30
  Hour24MinuteSecond,  // 14:30:45
  Hour12Minute,        // 02:30 PM
  Hour12MinuteSecond,  // 02:30:45 PM
}

/// Time component being edited
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum TimeComponent {
  Hour,
  Minute,
  Second,
  AmPm,
}

/// TimePicker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePickerConfig {
  pub format: TimeFormat,
  pub show_seconds: bool,
  pub minute_interval: u8, // e.g., 15 for 15-minute increments
  pub hour_interval: u8,   // e.g., 1 for 1-hour increments
  pub min_time: Option<Time>,
  pub max_time: Option<Time>,
}

impl Default for TimePickerConfig {
  fn default() -> Self {
    Self {
      format: TimeFormat::Hour24Minute,
      show_seconds: false,
      minute_interval: 1,
      hour_interval: 1,
      min_time: None,
      max_time: None,
    }
  }
}

/// TimePicker styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePickerStyle {
  pub background: String,
  pub border_color: String,
  pub text_color: String,
  pub selected_bg: String,
  pub selected_text: String,
  pub focused_bg: String,
  pub focused_text: String,
  pub separator_color: String,
  pub button_bg: String,
  pub button_text: String,
}

impl Default for TimePickerStyle {
  fn default() -> Self {
    Self {
      background: "#ffffff".to_string(),
      border_color: "#cccccc".to_string(),
      text_color: "#333333".to_string(),
      selected_bg: "#0078d4".to_string(),
      selected_text: "#ffffff".to_string(),
      focused_bg: "#e1ecf4".to_string(),
      focused_text: "#333333".to_string(),
      separator_color: "#666666".to_string(),
      button_bg: "#f5f5f5".to_string(),
      button_text: "#333333".to_string(),
    }
  }
}

/// TimePicker widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimePicker {
  pub selected_time: Option<Time>,
  pub is_open: bool,
  pub focused_component: TimeComponent,
  pub config: TimePickerConfig,
  pub style: TimePickerStyle,
}

impl TimePicker {
  /// Create a new TimePicker
  pub fn new() -> Self {
    Self {
      selected_time: None,
      is_open: false,
      focused_component: TimeComponent::Hour,
      config: TimePickerConfig::default(),
      style: TimePickerStyle::default(),
    }
  }

  /// Set selected time
  pub fn set_time(&mut self, time: Time) -> Result<()> {
    if self.is_time_valid(&time) {
      self.selected_time = Some(time);
      Ok(())
    } else {
      Err(TuiError::component("Time is outside allowed range".to_string()))
    }
  }

  /// Get selected time as formatted string
  pub fn get_formatted_time(&self) -> Option<String> {
    self.selected_time.map(|time| time.format(&self.config.format))
  }

  /// Open the time picker
  pub fn open(&mut self) {
    self.is_open = true;
    self.focused_component = TimeComponent::Hour;
  }

  /// Close the time picker
  pub fn close(&mut self) {
    self.is_open = false;
  }

  /// Check if time is within valid range
  fn is_time_valid(&self, time: &Time) -> bool {
    if let Some(min_time) = self.config.min_time {
      if *time < min_time {
        return false;
      }
    }

    if let Some(max_time) = self.config.max_time {
      if *time > max_time {
        return false;
      }
    }

    true
  }

  /// Increment the focused time component
  pub fn increment_component(&mut self) -> Result<()> {
    let mut time = self.selected_time.unwrap_or_else(Time::now);

    match self.focused_component {
      TimeComponent::Hour => {
        time.hour = (time.hour + self.config.hour_interval) % 24;
      }
      TimeComponent::Minute => {
        time.minute = (time.minute + self.config.minute_interval) % 60;
      }
      TimeComponent::Second => {
        time.second = (time.second + 1) % 60;
      }
      TimeComponent::AmPm => {
        time.hour = if time.hour < 12 { time.hour + 12 } else { time.hour - 12 };
      }
    }

    if self.is_time_valid(&time) {
      self.selected_time = Some(time);
    }

    Ok(())
  }

  /// Decrement the focused time component
  pub fn decrement_component(&mut self) -> Result<()> {
    let mut time = self.selected_time.unwrap_or_else(Time::now);

    match self.focused_component {
      TimeComponent::Hour => {
        time.hour = if time.hour >= self.config.hour_interval {
          time.hour - self.config.hour_interval
        } else {
          24 - self.config.hour_interval
        };
      }
      TimeComponent::Minute => {
        time.minute = if time.minute >= self.config.minute_interval {
          time.minute - self.config.minute_interval
        } else {
          60 - self.config.minute_interval
        };
      }
      TimeComponent::Second => {
        time.second = if time.second > 0 { time.second - 1 } else { 59 };
      }
      TimeComponent::AmPm => {
        time.hour = if time.hour < 12 { time.hour + 12 } else { time.hour - 12 };
      }
    }

    if self.is_time_valid(&time) {
      self.selected_time = Some(time);
    }

    Ok(())
  }

  /// Move focus to next component
  pub fn next_component(&mut self) {
    self.focused_component = match self.focused_component {
      TimeComponent::Hour => TimeComponent::Minute,
      TimeComponent::Minute => {
        if self.config.show_seconds {
          TimeComponent::Second
        } else if matches!(self.config.format, TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond) {
          TimeComponent::AmPm
        } else {
          TimeComponent::Hour
        }
      }
      TimeComponent::Second => {
        if matches!(self.config.format, TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond) {
          TimeComponent::AmPm
        } else {
          TimeComponent::Hour
        }
      }
      TimeComponent::AmPm => TimeComponent::Hour,
    };
  }

  /// Move focus to previous component
  pub fn prev_component(&mut self) {
    self.focused_component = match self.focused_component {
      TimeComponent::Hour => {
        if matches!(self.config.format, TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond) {
          TimeComponent::AmPm
        } else if self.config.show_seconds {
          TimeComponent::Second
        } else {
          TimeComponent::Minute
        }
      }
      TimeComponent::Minute => TimeComponent::Hour,
      TimeComponent::Second => TimeComponent::Minute,
      TimeComponent::AmPm => {
        if self.config.show_seconds {
          TimeComponent::Second
        } else {
          TimeComponent::Minute
        }
      }
    };
  }

  /// Render the TimePicker
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    if !self.is_open {
      // Render closed state (input field)
      self.render_input(&mut output, rect, theme)?;
    } else {
      // Render time picker popup
      self.render_picker(&mut output, rect, theme)?;
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
    let content = self.get_formatted_time().unwrap_or_else(|| "Select time...".to_string());
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

  /// Render time picker popup
  fn render_picker(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let focused_bg_def = get_palette_color(&theme.palette, &self.style.focused_bg)
      .map_err(|e| TuiError::render(e))?;
    let focused_bg = color_to_ansi(focused_bg_def, true);

    let focused_text_def = get_palette_color(&theme.palette, &self.style.focused_text)
      .map_err(|e| TuiError::render(e))?;
    let focused_text = color_to_ansi(focused_text_def, false);

    let separator_color_def = get_palette_color(&theme.palette, &self.style.separator_color)
      .map_err(|e| TuiError::render(e))?;
    let separator_color = color_to_ansi(separator_color_def, false);

    let time = self.selected_time.unwrap_or_else(Time::now);
    let picker_width = 15;
    let picker_height = 5;

    // Draw picker border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..picker_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Time display
    let y = rect.y + 2;
    write!(output, "\x1b[{};{}H{}│{}", y + 1, rect.x + 1, border_color, bg_color)?;

    // Hour component
    let hour_display = match self.config.format {
      TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond => {
        let (hour_12, _) = time.to_12_hour();
        format!("{:02}", hour_12)
      }
      _ => format!("{:02}", time.hour),
    };

    if self.focused_component == TimeComponent::Hour {
      write!(output, "{}{}{}", focused_bg, focused_text, hour_display)?;
    } else {
      write!(output, "{}{}", text_color, hour_display)?;
    }

    // Separator
    write!(output, "{}:", separator_color)?;

    // Minute component
    if self.focused_component == TimeComponent::Minute {
      write!(output, "{}{}{:02}", focused_bg, focused_text, time.minute)?;
    } else {
      write!(output, "{}{:02}", text_color, time.minute)?;
    }

    // Second component (if enabled)
    if self.config.show_seconds {
      write!(output, "{}:", separator_color)?;
      if self.focused_component == TimeComponent::Second {
        write!(output, "{}{}{:02}", focused_bg, focused_text, time.second)?;
      } else {
        write!(output, "{}{:02}", text_color, time.second)?;
      }
    }

    // AM/PM component (if 12-hour format)
    if matches!(self.config.format, TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond) {
      let (_, is_pm) = time.to_12_hour();
      let am_pm = if is_pm { "PM" } else { "AM" };
      write!(output, " ")?;
      if self.focused_component == TimeComponent::AmPm {
        write!(output, "{}{}{}", focused_bg, focused_text, am_pm)?;
      } else {
        write!(output, "{}{}", text_color, am_pm)?;
      }
    }

    // Pad to full width
    let content_len = if self.config.show_seconds { 8 } else { 5 };
    let content_len = if matches!(self.config.format, TimeFormat::Hour12Minute | TimeFormat::Hour12MinuteSecond) {
      content_len + 3
    } else {
      content_len
    };

    for _ in content_len..picker_width as usize - 2 {
      write!(output, " ")?;
    }
    write!(output, "{}│", border_color)?;

    // Instructions
    write!(output, "\x1b[{};{}H{}│{}↑/↓: Change  Tab: Next{}│",
           rect.y + 3, rect.x + 1, border_color, text_color, border_color)?;

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + picker_height, rect.x + 1, border_color)?;
    for _ in 0..picker_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<Option<TimePickerAction>> {
    if !self.is_open {
      match key {
        "Enter" | " " => {
          self.open();
          return Ok(Some(TimePickerAction::Opened));
        }
        _ => return Ok(None),
      }
    }

    match key {
      "Escape" => {
        self.close();
        Ok(Some(TimePickerAction::Closed))
      }
      "ArrowUp" => {
        self.increment_component()?;
        Ok(Some(TimePickerAction::TimeChanged))
      }
      "ArrowDown" => {
        self.decrement_component()?;
        Ok(Some(TimePickerAction::TimeChanged))
      }
      "Tab" | "ArrowRight" => {
        self.next_component();
        Ok(Some(TimePickerAction::ComponentChanged))
      }
      "ArrowLeft" => {
        self.prev_component();
        Ok(Some(TimePickerAction::ComponentChanged))
      }
      "Enter" => {
        self.close();
        Ok(Some(TimePickerAction::TimeSelected))
      }
      _ => Ok(None),
    }
  }
}

impl Default for TimePicker {
  fn default() -> Self {
    Self::new()
  }
}

/// Actions that can result from TimePicker interactions
#[derive(Debug, Clone, PartialEq)]
pub enum TimePickerAction {
  Opened,
  Closed,
  TimeChanged,
  TimeSelected,
  ComponentChanged,
}

/// Builder for TimePicker
pub struct TimePickerBuilder {
  timepicker: TimePicker,
}

impl TimePickerBuilder {
  pub fn new() -> Self {
    Self {
      timepicker: TimePicker::new(),
    }
  }

  pub fn selected_time(mut self, time: Time) -> Self {
    self.timepicker.selected_time = Some(time);
    self
  }

  pub fn format(mut self, format: TimeFormat) -> Self {
    self.timepicker.config.format = format;
    self
  }

  pub fn show_seconds(mut self, show: bool) -> Self {
    self.timepicker.config.show_seconds = show;
    self
  }

  pub fn minute_interval(mut self, interval: u8) -> Self {
    self.timepicker.config.minute_interval = interval;
    self
  }

  pub fn min_time(mut self, time: Time) -> Self {
    self.timepicker.config.min_time = Some(time);
    self
  }

  pub fn max_time(mut self, time: Time) -> Self {
    self.timepicker.config.max_time = Some(time);
    self
  }

  pub fn style(mut self, style: TimePickerStyle) -> Self {
    self.timepicker.style = style;
    self
  }

  pub fn build(self) -> TimePicker {
    self.timepicker
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_time_creation() {
    let time = Time::new(14, 30, 45).unwrap();
    assert_eq!(time.hour, 14);
    assert_eq!(time.minute, 30);
    assert_eq!(time.second, 45);
  }

  #[test]
  fn test_time_validation() {
    assert!(Time::new(24, 0, 0).is_err()); // Invalid hour
    assert!(Time::new(0, 60, 0).is_err());  // Invalid minute
    assert!(Time::new(0, 0, 60).is_err());  // Invalid second
  }

  #[test]
  fn test_12_hour_conversion() {
    let time = Time::new(14, 30, 0).unwrap();
    let (hour_12, is_pm) = time.to_12_hour();
    assert_eq!(hour_12, 2);
    assert!(is_pm);

    let time = Time::new(0, 30, 0).unwrap();
    let (hour_12, is_pm) = time.to_12_hour();
    assert_eq!(hour_12, 12);
    assert!(!is_pm);
  }

  #[test]
  fn test_time_formatting() {
    let time = Time::new(14, 30, 45).unwrap();
    assert_eq!(time.format(&TimeFormat::Hour24Minute), "14:30");
    assert_eq!(time.format(&TimeFormat::Hour24MinuteSecond), "14:30:45");
    assert_eq!(time.format(&TimeFormat::Hour12Minute), "02:30 PM");
    assert_eq!(time.format(&TimeFormat::Hour12MinuteSecond), "02:30:45 PM");
  }

  #[test]
  fn test_timepicker_creation() {
    let timepicker = TimePicker::new();
    assert!(timepicker.selected_time.is_none());
    assert!(!timepicker.is_open);
    assert_eq!(timepicker.focused_component, TimeComponent::Hour);
  }
}
