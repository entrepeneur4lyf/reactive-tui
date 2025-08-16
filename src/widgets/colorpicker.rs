/*!
 * ColorPicker Component - Color selection widget
 *
 * A comprehensive color picker widget providing:
 * - Multiple color formats (RGB, HSL, HSV, Hex)
 * - Color palette selection
 * - Hue/Saturation/Lightness sliders
 * - Color preview and comparison
 * - Recent colors history
 * - Named color presets
 * - Keyboard navigation and input
 */

use crate::{
  error::{Result, TuiError},
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Color representation
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Color {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub a: f32, // Alpha channel (0.0 - 1.0)
}

impl Color {
  /// Create a new color
  pub fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b, a: 1.0 }
  }

  /// Create a color with alpha
  pub fn with_alpha(r: u8, g: u8, b: u8, a: f32) -> Self {
    Self { r, g, b, a: a.clamp(0.0, 1.0) }
  }

  /// Create color from hex string
  pub fn from_hex(hex: &str) -> Result<Self> {
    let hex = hex.trim_start_matches('#');
    if hex.len() != 6 {
      return Err(TuiError::component("Invalid hex color format".to_string()));
    }

    let r = u8::from_str_radix(&hex[0..2], 16)
      .map_err(|_| TuiError::component("Invalid hex color".to_string()))?;
    let g = u8::from_str_radix(&hex[2..4], 16)
      .map_err(|_| TuiError::component("Invalid hex color".to_string()))?;
    let b = u8::from_str_radix(&hex[4..6], 16)
      .map_err(|_| TuiError::component("Invalid hex color".to_string()))?;

    Ok(Self::new(r, g, b))
  }

  /// Convert to hex string
  pub fn to_hex(&self) -> String {
    format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
  }

  /// Convert to HSL (Hue, Saturation, Lightness)
  pub fn to_hsl(&self) -> (f32, f32, f32) {
    let r = self.r as f32 / 255.0;
    let g = self.g as f32 / 255.0;
    let b = self.b as f32 / 255.0;

    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let delta = max - min;

    // Lightness
    let l = (max + min) / 2.0;

    if delta == 0.0 {
      return (0.0, 0.0, l); // Grayscale
    }

    // Saturation
    let s = if l < 0.5 {
      delta / (max + min)
    } else {
      delta / (2.0 - max - min)
    };

    // Hue
    let h = if max == r {
      ((g - b) / delta + if g < b { 6.0 } else { 0.0 }) / 6.0
    } else if max == g {
      ((b - r) / delta + 2.0) / 6.0
    } else {
      ((r - g) / delta + 4.0) / 6.0
    };

    (h * 360.0, s, l)
  }

  /// Create color from HSL
  pub fn from_hsl(h: f32, s: f32, l: f32) -> Self {
    let h = (h % 360.0) / 360.0;
    let s = s.clamp(0.0, 1.0);
    let l = l.clamp(0.0, 1.0);

    if s == 0.0 {
      let gray = (l * 255.0) as u8;
      return Self::new(gray, gray, gray);
    }

    let hue_to_rgb = |p: f32, q: f32, t: f32| {
      let t = if t < 0.0 { t + 1.0 } else if t > 1.0 { t - 1.0 } else { t };
      if t < 1.0 / 6.0 {
        p + (q - p) * 6.0 * t
      } else if t < 1.0 / 2.0 {
        q
      } else if t < 2.0 / 3.0 {
        p + (q - p) * (2.0 / 3.0 - t) * 6.0
      } else {
        p
      }
    };

    let q = if l < 0.5 { l * (1.0 + s) } else { l + s - l * s };
    let p = 2.0 * l - q;

    let r = (hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0) as u8;
    let g = (hue_to_rgb(p, q, h) * 255.0) as u8;
    let b = (hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0) as u8;

    Self::new(r, g, b)
  }

  /// Get color brightness (0.0 - 1.0)
  pub fn brightness(&self) -> f32 {
    (0.299 * self.r as f32 + 0.587 * self.g as f32 + 0.114 * self.b as f32) / 255.0
  }

  /// Check if color is dark (brightness < 0.5)
  pub fn is_dark(&self) -> bool {
    self.brightness() < 0.5
  }

  /// Get contrasting text color (black or white)
  pub fn contrasting_text(&self) -> Color {
    if self.is_dark() {
      Color::new(255, 255, 255) // White
    } else {
      Color::new(0, 0, 0) // Black
    }
  }
}

/// Color format for display and input
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorFormat {
  Hex,    // #FF0000
  RGB,    // rgb(255, 0, 0)
  HSL,    // hsl(0, 100%, 50%)
}

/// Color picker mode
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ColorPickerMode {
  Palette,  // Grid of predefined colors
  Sliders,  // RGB/HSL sliders
  Wheel,    // Color wheel (simplified for terminal)
  Input,    // Text input for color values
}

/// Predefined color palette
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPalette {
  pub name: String,
  pub colors: Vec<Color>,
}

impl ColorPalette {
  /// Create a basic color palette
  pub fn basic() -> Self {
    Self {
      name: "Basic".to_string(),
      colors: vec![
        Color::new(255, 0, 0),     // Red
        Color::new(0, 255, 0),     // Green
        Color::new(0, 0, 255),     // Blue
        Color::new(255, 255, 0),   // Yellow
        Color::new(255, 0, 255),   // Magenta
        Color::new(0, 255, 255),   // Cyan
        Color::new(255, 255, 255), // White
        Color::new(0, 0, 0),       // Black
        Color::new(128, 128, 128), // Gray
        Color::new(255, 128, 0),   // Orange
        Color::new(128, 0, 128),   // Purple
        Color::new(0, 128, 0),     // Dark Green
      ],
    }
  }

  /// Create a material design palette
  pub fn material() -> Self {
    Self {
      name: "Material".to_string(),
      colors: vec![
        Color::new(244, 67, 54),   // Red 500
        Color::new(233, 30, 99),   // Pink 500
        Color::new(156, 39, 176),  // Purple 500
        Color::new(103, 58, 183),  // Deep Purple 500
        Color::new(63, 81, 181),   // Indigo 500
        Color::new(33, 150, 243),  // Blue 500
        Color::new(3, 169, 244),   // Light Blue 500
        Color::new(0, 188, 212),   // Cyan 500
        Color::new(0, 150, 136),   // Teal 500
        Color::new(76, 175, 80),   // Green 500
        Color::new(139, 195, 74),  // Light Green 500
        Color::new(205, 220, 57),  // Lime 500
        Color::new(255, 235, 59),  // Yellow 500
        Color::new(255, 193, 7),   // Amber 500
        Color::new(255, 152, 0),   // Orange 500
        Color::new(255, 87, 34),   // Deep Orange 500
      ],
    }
  }
}

/// ColorPicker configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPickerConfig {
  pub mode: ColorPickerMode,
  pub format: ColorFormat,
  pub show_alpha: bool,
  pub show_preview: bool,
  pub show_recent: bool,
  pub max_recent: usize,
  pub palettes: Vec<ColorPalette>,
  pub allow_custom_input: bool,
}

impl Default for ColorPickerConfig {
  fn default() -> Self {
    Self {
      mode: ColorPickerMode::Palette,
      format: ColorFormat::Hex,
      show_alpha: false,
      show_preview: true,
      show_recent: true,
      max_recent: 8,
      palettes: vec![ColorPalette::basic(), ColorPalette::material()],
      allow_custom_input: true,
    }
  }
}

/// ColorPicker styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPickerStyle {
  pub background: String,
  pub border_color: String,
  pub text_color: String,
  pub selected_border: String,
  pub hover_border: String,
  pub preview_border: String,
  pub slider_track: String,
  pub slider_thumb: String,
}

impl Default for ColorPickerStyle {
  fn default() -> Self {
    Self {
      background: "#ffffff".to_string(),
      border_color: "#cccccc".to_string(),
      text_color: "#333333".to_string(),
      selected_border: "#0078d4".to_string(),
      hover_border: "#666666".to_string(),
      preview_border: "#999999".to_string(),
      slider_track: "#e0e0e0".to_string(),
      slider_thumb: "#0078d4".to_string(),
    }
  }
}

/// ColorPicker widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPicker {
  pub selected_color: Color,
  pub previous_color: Option<Color>,
  pub recent_colors: Vec<Color>,
  pub config: ColorPickerConfig,
  pub style: ColorPickerStyle,
  pub current_palette: usize,
  pub selected_palette_index: Option<usize>,
  pub is_open: bool,
  pub slider_values: SliderValues,
}

/// Slider values for RGB/HSL mode
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderValues {
  pub r: u8,
  pub g: u8,
  pub b: u8,
  pub h: f32,
  pub s: f32,
  pub l: f32,
  pub a: f32,
}

impl SliderValues {
  fn from_color(color: Color) -> Self {
    let (h, s, l) = color.to_hsl();
    Self {
      r: color.r,
      g: color.g,
      b: color.b,
      h,
      s,
      l,
      a: color.a,
    }
  }

  #[allow(dead_code)]
  fn to_color(&self) -> Color {
    match self {
      _ => Color::with_alpha(self.r, self.g, self.b, self.a),
    }
  }
}

impl ColorPicker {
  /// Create a new ColorPicker
  pub fn new() -> Self {
    let default_color = Color::new(255, 0, 0);
    Self {
      selected_color: default_color,
      previous_color: None,
      recent_colors: Vec::new(),
      config: ColorPickerConfig::default(),
      style: ColorPickerStyle::default(),
      current_palette: 0,
      selected_palette_index: None,
      is_open: false,
      slider_values: SliderValues::from_color(default_color),
    }
  }

  /// Set selected color
  pub fn set_color(&mut self, color: Color) {
    self.previous_color = Some(self.selected_color);
    self.selected_color = color;
    self.slider_values = SliderValues::from_color(color);
    self.add_to_recent(color);
  }

  /// Add color to recent colors
  fn add_to_recent(&mut self, color: Color) {
    // Remove if already exists
    self.recent_colors.retain(|&c| c != color);

    // Add to front
    self.recent_colors.insert(0, color);

    // Limit size
    if self.recent_colors.len() > self.config.max_recent {
      self.recent_colors.truncate(self.config.max_recent);
    }
  }

  /// Open the color picker
  pub fn open(&mut self) {
    self.is_open = true;
  }

  /// Close the color picker
  pub fn close(&mut self) {
    self.is_open = false;
  }

  /// Toggle color picker open/closed
  pub fn toggle(&mut self) {
    self.is_open = !self.is_open;
  }

  /// Switch to next palette
  pub fn next_palette(&mut self) {
    if !self.config.palettes.is_empty() {
      self.current_palette = (self.current_palette + 1) % self.config.palettes.len();
      self.selected_palette_index = None;
    }
  }

  /// Switch to previous palette
  pub fn prev_palette(&mut self) {
    if !self.config.palettes.is_empty() {
      self.current_palette = if self.current_palette == 0 {
        self.config.palettes.len() - 1
      } else {
        self.current_palette - 1
      };
      self.selected_palette_index = None;
    }
  }

  /// Format color according to current format
  pub fn format_color(&self, color: Color) -> String {
    match self.config.format {
      ColorFormat::Hex => color.to_hex(),
      ColorFormat::RGB => {
        if self.config.show_alpha && color.a < 1.0 {
          format!("rgba({}, {}, {}, {:.2})", color.r, color.g, color.b, color.a)
        } else {
          format!("rgb({}, {}, {})", color.r, color.g, color.b)
        }
      }
      ColorFormat::HSL => {
        let (h, s, l) = color.to_hsl();
        if self.config.show_alpha && color.a < 1.0 {
          format!("hsla({:.0}, {:.0}%, {:.0}%, {:.2})", h, s * 100.0, l * 100.0, color.a)
        } else {
          format!("hsl({:.0}, {:.0}%, {:.0}%)", h, s * 100.0, l * 100.0)
        }
      }
    }
  }

  /// Render the ColorPicker
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    if !self.is_open {
      // Render closed state (color swatch)
      self.render_swatch(&mut output, rect, theme)?;
    } else {
      // Render color picker popup
      match self.config.mode {
        ColorPickerMode::Palette => self.render_palette_mode(&mut output, rect, theme)?,
        ColorPickerMode::Sliders => self.render_sliders_mode(&mut output, rect, theme)?,
        ColorPickerMode::Wheel => self.render_wheel_mode(&mut output, rect, theme)?,
        ColorPickerMode::Input => self.render_input_mode(&mut output, rect, theme)?,
      }
    }

    Ok(output)
  }

  /// Render color swatch (closed state)
  fn render_swatch(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);
    let color_bg = format!("\x1b[48;2;{};{};{}m", self.selected_color.r, self.selected_color.g, self.selected_color.b);

    // Draw swatch border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Color area
    for y in 1..rect.height - 1 {
      write!(output, "\x1b[{};{}H{}│{}{:width$}{}│",
             rect.y + y + 1, rect.x + 1, border_color, color_bg, "", border_color,
             width = rect.width as usize - 2)?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + rect.height, rect.x + 1, border_color)?;
    for _ in 0..rect.width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render palette mode
  fn render_palette_mode(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    let bg_color_def = get_palette_color(&theme.palette, &self.style.background)
      .map_err(|e| TuiError::render(e))?;
    let bg_color = color_to_ansi(bg_color_def, true);

    let border_color_def = get_palette_color(&theme.palette, &self.style.border_color)
      .map_err(|e| TuiError::render(e))?;
    let border_color = color_to_ansi(border_color_def, false);

    let text_color_def = get_palette_color(&theme.palette, &self.style.text_color)
      .map_err(|e| TuiError::render(e))?;
    let text_color = color_to_ansi(text_color_def, false);

    let selected_border_def = get_palette_color(&theme.palette, &self.style.selected_border)
      .map_err(|e| TuiError::render(e))?;
    let selected_border = color_to_ansi(selected_border_def, false);

    let picker_width = 30;
    let picker_height = 15;

    // Draw border
    write!(output, "\x1b[{};{}H{}┌", rect.y + 1, rect.x + 1, border_color)?;
    for _ in 0..picker_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┐")?;

    // Title
    let current_palette = &self.config.palettes[self.current_palette];
    write!(output, "\x1b[{};{}H{}│{}{}{:<width$}{}│",
           rect.y + 2, rect.x + 1, border_color, bg_color, text_color,
           format!("Palette: {}", current_palette.name), border_color,
           width = picker_width as usize - 2)?;

    // Palette colors (simplified grid)
    let colors_per_row = 6;
    let mut color_index = 0;

    for row in 0..3 {
      let y = rect.y + 4 + row * 2;
      write!(output, "\x1b[{};{}H{}│{}", y + 1, rect.x + 1, border_color, bg_color)?;

      for _col in 0..colors_per_row {
        if color_index < current_palette.colors.len() {
          let color = current_palette.colors[color_index];
          let is_selected = self.selected_palette_index == Some(color_index);

          let color_bg = format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b);
          let border = if is_selected { &selected_border } else { &border_color };

          write!(output, "{}[{}]", border, color_bg)?;
          write!(output, "  \x1b[0m{}", bg_color)?;

          color_index += 1;
        } else {
          write!(output, "    ")?;
        }
      }

      write!(output, "{}│", border_color)?;
    }

    // Preview area
    if self.config.show_preview {
      let y = rect.y + 11;
      write!(output, "\x1b[{};{}H{}│{}Preview: {}{:<width$}{}│",
             y + 1, rect.x + 1, border_color, bg_color, text_color,
             self.format_color(self.selected_color), border_color,
             width = picker_width as usize - 12)?;
    }

    // Recent colors
    if self.config.show_recent && !self.recent_colors.is_empty() {
      let y = rect.y + 13;
      write!(output, "\x1b[{};{}H{}│{}Recent: ",
             y + 1, rect.x + 1, border_color, bg_color)?;

      for (_i, &color) in self.recent_colors.iter().take(6).enumerate() {
        let color_bg = format!("\x1b[48;2;{};{};{}m", color.r, color.g, color.b);
        write!(output, "{}■\x1b[0m{}", color_bg, bg_color)?;
      }

      write!(output, "{:width$}{}│", "", border_color,
             width = picker_width as usize - 10 - self.recent_colors.len().min(6))?;
    }

    // Bottom border
    write!(output, "\x1b[{};{}H{}└", rect.y + picker_height, rect.x + 1, border_color)?;
    for _ in 0..picker_width - 2 {
      write!(output, "─")?;
    }
    write!(output, "┘")?;

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render sliders mode (simplified)
  fn render_sliders_mode(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    // For simplicity, render as palette mode
    self.render_palette_mode(output, rect, theme)
  }

  /// Render wheel mode (simplified)
  fn render_wheel_mode(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    // For simplicity, render as palette mode
    self.render_palette_mode(output, rect, theme)
  }

  /// Render input mode (simplified)
  fn render_input_mode(&self, output: &mut String, rect: LayoutRect, theme: &ColorTheme) -> Result<()> {
    // For simplicity, render as palette mode
    self.render_palette_mode(output, rect, theme)
  }

  /// Handle keyboard input
  pub fn handle_key(&mut self, key: &str) -> Result<Option<ColorPickerAction>> {
    if !self.is_open {
      match key {
        "Enter" | " " => {
          self.open();
          return Ok(Some(ColorPickerAction::Opened));
        }
        _ => return Ok(None),
      }
    }

    match key {
      "Escape" => {
        self.close();
        Ok(Some(ColorPickerAction::Closed))
      }
      "Tab" => {
        self.next_palette();
        Ok(Some(ColorPickerAction::PaletteChanged))
      }
      "ArrowLeft" => {
        if let Some(index) = self.selected_palette_index {
          let colors_per_row = 6;
          if index % colors_per_row > 0 {
            self.selected_palette_index = Some(index - 1);
          }
        } else if !self.config.palettes.is_empty() && !self.config.palettes[self.current_palette].colors.is_empty() {
          self.selected_palette_index = Some(0);
        }
        Ok(Some(ColorPickerAction::SelectionChanged))
      }
      "ArrowRight" => {
        if let Some(index) = self.selected_palette_index {
          let colors_per_row = 6;
          let current_palette = &self.config.palettes[self.current_palette];
          if index % colors_per_row < colors_per_row - 1 && index + 1 < current_palette.colors.len() {
            self.selected_palette_index = Some(index + 1);
          }
        } else if !self.config.palettes.is_empty() && !self.config.palettes[self.current_palette].colors.is_empty() {
          self.selected_palette_index = Some(0);
        }
        Ok(Some(ColorPickerAction::SelectionChanged))
      }
      "ArrowUp" => {
        if let Some(index) = self.selected_palette_index {
          let colors_per_row = 6;
          if index >= colors_per_row {
            self.selected_palette_index = Some(index - colors_per_row);
          }
        }
        Ok(Some(ColorPickerAction::SelectionChanged))
      }
      "ArrowDown" => {
        if let Some(index) = self.selected_palette_index {
          let colors_per_row = 6;
          let current_palette = &self.config.palettes[self.current_palette];
          if index + colors_per_row < current_palette.colors.len() {
            self.selected_palette_index = Some(index + colors_per_row);
          }
        }
        Ok(Some(ColorPickerAction::SelectionChanged))
      }
      "Enter" => {
        if let Some(index) = self.selected_palette_index {
          let current_palette = &self.config.palettes[self.current_palette];
          if index < current_palette.colors.len() {
            let color = current_palette.colors[index];
            self.set_color(color);
            self.close();
            return Ok(Some(ColorPickerAction::ColorSelected(color)));
          }
        }
        Ok(None)
      }
      _ => Ok(None),
    }
  }
}

impl Default for ColorPicker {
  fn default() -> Self {
    Self::new()
  }
}

/// Actions that can result from ColorPicker interactions
#[derive(Debug, Clone, PartialEq)]
pub enum ColorPickerAction {
  Opened,
  Closed,
  ColorSelected(Color),
  SelectionChanged,
  PaletteChanged,
}

/// Builder for ColorPicker
pub struct ColorPickerBuilder {
  colorpicker: ColorPicker,
}

impl ColorPickerBuilder {
  pub fn new() -> Self {
    Self {
      colorpicker: ColorPicker::new(),
    }
  }

  pub fn selected_color(mut self, color: Color) -> Self {
    self.colorpicker.selected_color = color;
    self.colorpicker.slider_values = SliderValues::from_color(color);
    self
  }

  pub fn mode(mut self, mode: ColorPickerMode) -> Self {
    self.colorpicker.config.mode = mode;
    self
  }

  pub fn format(mut self, format: ColorFormat) -> Self {
    self.colorpicker.config.format = format;
    self
  }

  pub fn show_alpha(mut self, show: bool) -> Self {
    self.colorpicker.config.show_alpha = show;
    self
  }

  pub fn palettes(mut self, palettes: Vec<ColorPalette>) -> Self {
    self.colorpicker.config.palettes = palettes;
    self
  }

  pub fn style(mut self, style: ColorPickerStyle) -> Self {
    self.colorpicker.style = style;
    self
  }

  pub fn build(self) -> ColorPicker {
    self.colorpicker
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_color_creation() {
    let color = Color::new(255, 128, 64);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
    assert_eq!(color.a, 1.0);
  }

  #[test]
  fn test_color_hex_conversion() {
    let color = Color::from_hex("#FF8040").unwrap();
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);

    assert_eq!(color.to_hex(), "#ff8040");
  }

  #[test]
  fn test_color_hsl_conversion() {
    let color = Color::new(255, 0, 0); // Pure red
    let (h, s, l) = color.to_hsl();
    assert!((h - 0.0).abs() < 0.01);
    assert!((s - 1.0).abs() < 0.01);
    assert!((l - 0.5).abs() < 0.01);

    let converted = Color::from_hsl(h, s, l);
    assert_eq!(converted.r, 255);
    assert!(converted.g <= 1); // Allow for rounding
    assert!(converted.b <= 1);
  }

  #[test]
  fn test_color_brightness() {
    let white = Color::new(255, 255, 255);
    let black = Color::new(0, 0, 0);

    assert!(white.brightness() > 0.9);
    assert!(black.brightness() < 0.1);
    assert!(!white.is_dark());
    assert!(black.is_dark());
  }

  #[test]
  fn test_colorpicker_creation() {
    let picker = ColorPicker::new();
    assert!(!picker.is_open);
    assert!(picker.recent_colors.is_empty());
    assert_eq!(picker.config.mode, ColorPickerMode::Palette);
  }

  #[test]
  fn test_colorpicker_recent_colors() {
    let mut picker = ColorPicker::new();
    let red = Color::new(255, 0, 0);
    let green = Color::new(0, 255, 0);

    picker.set_color(red);
    assert_eq!(picker.recent_colors.len(), 1);
    assert_eq!(picker.recent_colors[0], red);

    picker.set_color(green);
    assert_eq!(picker.recent_colors.len(), 2);
    assert_eq!(picker.recent_colors[0], green);
    assert_eq!(picker.recent_colors[1], red);
  }
}
