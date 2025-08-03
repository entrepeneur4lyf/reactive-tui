/*!
 * Color Theme System for Rust TUI
 *
 * Provides color definitions, palettes, themes, and ANSI escape code generation.
 * Mirrors the TypeScript color system implementation.
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// RGB color definition with validation
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct ColorDefinition {
  pub r: u8,
  pub g: u8,
  pub b: u8,
}

/// Color mode for theme rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum ColorMode {
  #[serde(rename = "rgb")]
  #[default]
  Rgb,
  #[serde(rename = "ansi")]
  Ansi,
  #[serde(rename = "auto")]
  Auto,
}

/// Complete color palette with semantic color definitions
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorPalette {
  // Primary colors
  pub primary: ColorDefinition,
  pub primary_dark: ColorDefinition,
  pub primary_light: ColorDefinition,

  // Secondary colors
  pub secondary: ColorDefinition,
  pub secondary_dark: ColorDefinition,
  pub secondary_light: ColorDefinition,

  // Neutral colors
  pub background: ColorDefinition,
  pub background_alt: ColorDefinition,
  pub surface: ColorDefinition,
  pub surface_alt: ColorDefinition,

  // Text colors
  pub text: ColorDefinition,
  pub text_secondary: ColorDefinition,
  pub text_muted: ColorDefinition,
  pub text_inverse: ColorDefinition,

  // Border colors
  pub border: ColorDefinition,
  pub border_focus: ColorDefinition,
  pub border_hover: ColorDefinition,

  // Status colors
  pub success: ColorDefinition,
  pub warning: ColorDefinition,
  pub error: ColorDefinition,
  pub info: ColorDefinition,

  // Interactive colors
  pub hover: ColorDefinition,
  pub active: ColorDefinition,
  pub disabled: ColorDefinition,

  // Shadow colors
  pub shadow: ColorDefinition,
  pub shadow_light: ColorDefinition,
}

/// Semantic color mappings for UI components
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SemanticColorMapping {
  pub panel_background: String,
  pub panel_border: String,
  pub panel_title: String,
  pub panel_content: String,
  pub panel_shadow: String,

  pub button_background: String,
  pub button_border: String,
  pub button_text: String,
  pub button_hover: String,

  pub input_background: String,
  pub input_border: String,
  pub input_text: String,
  pub input_focus: String,

  pub progress_background: String,
  pub progress_fill: String,
  pub progress_text: String,

  // Rich text editor colors
  pub editor_background: String,
  pub editor_text: String,
  pub editor_cursor: String,
  pub editor_line_number: String,
  pub editor_selection: String,
  pub editor_border: String,
  pub editor_border_focus: String,

  // Syntax highlighting colors
  pub syntax_keyword: String,
  pub syntax_string: String,
  pub syntax_comment: String,
  pub syntax_number: String,
  pub syntax_function: String,
  pub syntax_type: String,
  pub syntax_variable: String,
  pub syntax_operator: String,
  pub syntax_punctuation: String,
  pub syntax_constant: String,
  pub syntax_tag: String,
  pub syntax_attribute: String,
}

/// Complete color theme with palette and semantic mappings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ColorTheme {
  pub name: String,
  pub description: String,
  pub mode: ColorMode,
  pub palette: ColorPalette,
  pub semantic: SemanticColorMapping,
}

/// Create RGB color with validation
pub fn rgb(r: u8, g: u8, b: u8) -> ColorDefinition {
  ColorDefinition { r, g, b }
}

/// Create color from hex string
pub fn hex(hex_color: &str) -> Result<ColorDefinition, String> {
  let hex = hex_color.trim_start_matches('#');

  if hex.len() != 6 {
    return Err(format!("Invalid hex color length: {hex_color}"));
  }

  let r = u8::from_str_radix(&hex[0..2], 16)
    .map_err(|_| format!("Invalid red component in hex color: {hex_color}"))?;
  let g = u8::from_str_radix(&hex[2..4], 16)
    .map_err(|_| format!("Invalid green component in hex color: {hex_color}"))?;
  let b = u8::from_str_radix(&hex[4..6], 16)
    .map_err(|_| format!("Invalid blue component in hex color: {hex_color}"))?;

  Ok(rgb(r, g, b))
}

/// Create color variant (lighter/darker)
pub fn create_variant(color: ColorDefinition, factor: f32) -> ColorDefinition {
  if factor > 0.0 {
    // Lighten
    let r = (color.r as f32 + (255.0 - color.r as f32) * factor).min(255.0) as u8;
    let g = (color.g as f32 + (255.0 - color.g as f32) * factor).min(255.0) as u8;
    let b = (color.b as f32 + (255.0 - color.b as f32) * factor).min(255.0) as u8;
    rgb(r, g, b)
  } else {
    // Darken
    let dark_factor = factor.abs();
    let r = (color.r as f32 * (1.0 - dark_factor)).max(0.0) as u8;
    let g = (color.g as f32 * (1.0 - dark_factor)).max(0.0) as u8;
    let b = (color.b as f32 * (1.0 - dark_factor)).max(0.0) as u8;
    rgb(r, g, b)
  }
}

/// Convert color to ANSI escape sequence with terminal capability detection
pub fn color_to_ansi(color: ColorDefinition, background: bool) -> String {
  use super::color_support::{get_compatible_ansi_color, global_color_support};

  let color_support = global_color_support::detect();
  get_compatible_ansi_color(color.r, color.g, color.b, background, color_support)
}

/// Get semantic color ANSI code from theme
pub fn get_semantic_color(theme: &ColorTheme, semantic_key: &str) -> Result<String, String> {
  let palette_key = match semantic_key {
    "panel_background" => &theme.semantic.panel_background,
    "panel_border" => &theme.semantic.panel_border,
    "panel_title" => &theme.semantic.panel_title,
    "panel_content" => &theme.semantic.panel_content,
    "panel_shadow" => &theme.semantic.panel_shadow,
    "button_background" => &theme.semantic.button_background,
    "button_border" => &theme.semantic.button_border,
    "button_text" => &theme.semantic.button_text,
    "button_hover" => &theme.semantic.button_hover,
    "input_background" => &theme.semantic.input_background,
    "input_border" => &theme.semantic.input_border,
    "input_text" => &theme.semantic.input_text,
    "input_focus" => &theme.semantic.input_focus,
    "progress_background" => &theme.semantic.progress_background,
    "progress_fill" => &theme.semantic.progress_fill,
    "progress_text" => &theme.semantic.progress_text,
    "editor_background" => &theme.semantic.editor_background,
    "editor_text" => &theme.semantic.editor_text,
    "editor_cursor" => &theme.semantic.editor_cursor,
    "editor_line_number" => &theme.semantic.editor_line_number,
    "editor_selection" => &theme.semantic.editor_selection,
    "editor_border" => &theme.semantic.editor_border,
    "editor_border_focus" => &theme.semantic.editor_border_focus,
    "syntax_keyword" => &theme.semantic.syntax_keyword,
    "syntax_string" => &theme.semantic.syntax_string,
    "syntax_comment" => &theme.semantic.syntax_comment,
    "syntax_number" => &theme.semantic.syntax_number,
    "syntax_function" => &theme.semantic.syntax_function,
    "syntax_type" => &theme.semantic.syntax_type,
    "syntax_variable" => &theme.semantic.syntax_variable,
    "syntax_operator" => &theme.semantic.syntax_operator,
    "syntax_punctuation" => &theme.semantic.syntax_punctuation,
    "syntax_constant" => &theme.semantic.syntax_constant,
    "syntax_tag" => &theme.semantic.syntax_tag,
    "syntax_attribute" => &theme.semantic.syntax_attribute,
    _ => return Err(format!("Unknown semantic key: {semantic_key}")),
  };

  let color = get_palette_color(&theme.palette, palette_key)?;
  Ok(color_to_ansi(color, false))
}

/// Get semantic background color ANSI code from theme
pub fn get_semantic_background(theme: &ColorTheme, semantic_key: &str) -> Result<String, String> {
  let palette_key = match semantic_key {
    "panel_background" => &theme.semantic.panel_background,
    "panel_border" => &theme.semantic.panel_border,
    "panel_title" => &theme.semantic.panel_title,
    "panel_content" => &theme.semantic.panel_content,
    "panel_shadow" => &theme.semantic.panel_shadow,
    "button_background" => &theme.semantic.button_background,
    "button_border" => &theme.semantic.button_border,
    "button_text" => &theme.semantic.button_text,
    "button_hover" => &theme.semantic.button_hover,
    "input_background" => &theme.semantic.input_background,
    "input_border" => &theme.semantic.input_border,
    "input_text" => &theme.semantic.input_text,
    "input_focus" => &theme.semantic.input_focus,
    "progress_background" => &theme.semantic.progress_background,
    "progress_fill" => &theme.semantic.progress_fill,
    "progress_text" => &theme.semantic.progress_text,
    _ => return Err(format!("Unknown semantic key: {semantic_key}")),
  };

  let color = get_palette_color(&theme.palette, palette_key)?;
  Ok(color_to_ansi(color, true))
}

/// Get color from palette by key
fn get_palette_color(palette: &ColorPalette, key: &str) -> Result<ColorDefinition, String> {
  match key {
    "primary" => Ok(palette.primary),
    "primary_dark" => Ok(palette.primary_dark),
    "primary_light" => Ok(palette.primary_light),
    "secondary" => Ok(palette.secondary),
    "secondary_dark" => Ok(palette.secondary_dark),
    "secondary_light" => Ok(palette.secondary_light),
    "background" => Ok(palette.background),
    "background_alt" => Ok(palette.background_alt),
    "surface" => Ok(palette.surface),
    "surface_alt" => Ok(palette.surface_alt),
    "text" => Ok(palette.text),
    "text_secondary" => Ok(palette.text_secondary),
    "text_muted" => Ok(palette.text_muted),
    "text_inverse" => Ok(palette.text_inverse),
    "border" => Ok(palette.border),
    "border_focus" => Ok(palette.border_focus),
    "border_hover" => Ok(palette.border_hover),
    "success" => Ok(palette.success),
    "warning" => Ok(palette.warning),
    "error" => Ok(palette.error),
    "info" => Ok(palette.info),
    "hover" => Ok(palette.hover),
    "active" => Ok(palette.active),
    "disabled" => Ok(palette.disabled),
    "shadow" => Ok(palette.shadow),
    "shadow_light" => Ok(palette.shadow_light),
    _ => Err(format!("Unknown palette key: {key}")),
  }
}

/// ANSI reset code
pub const RESET_COLOR: &str = "\x1B[0m";

/// Built-in dark theme
pub fn dark_theme() -> ColorTheme {
  ColorTheme {
    name: "dark".to_string(),
    description: "Modern dark theme with professional colors".to_string(),
    mode: ColorMode::Rgb,
    palette: ColorPalette {
      primary: rgb(99, 102, 241),
      primary_dark: rgb(79, 70, 229),
      primary_light: rgb(129, 140, 248),
      secondary: rgb(16, 185, 129),
      secondary_dark: rgb(5, 150, 105),
      secondary_light: rgb(52, 211, 153),
      background: rgb(17, 24, 39),
      background_alt: rgb(31, 41, 55),
      surface: rgb(55, 65, 81),
      surface_alt: rgb(75, 85, 99),
      text: rgb(249, 250, 251),
      text_secondary: rgb(209, 213, 219),
      text_muted: rgb(156, 163, 175),
      text_inverse: rgb(17, 24, 39),
      border: rgb(75, 85, 99),
      border_focus: rgb(99, 102, 241),
      border_hover: rgb(107, 114, 128),
      success: rgb(34, 197, 94),
      warning: rgb(251, 191, 36),
      error: rgb(239, 68, 68),
      info: rgb(59, 130, 246),
      hover: rgb(67, 56, 202),
      active: rgb(55, 48, 163),
      disabled: rgb(107, 114, 128),
      shadow: rgb(0, 0, 0),
      shadow_light: rgb(31, 41, 55),
    },
    semantic: default_semantic_mapping(),
  }
}

/// Built-in light theme
pub fn light_theme() -> ColorTheme {
  ColorTheme {
    name: "light".to_string(),
    description: "Clean light theme for bright environments".to_string(),
    mode: ColorMode::Rgb,
    palette: ColorPalette {
      primary: rgb(99, 102, 241),
      primary_dark: rgb(79, 70, 229),
      primary_light: rgb(165, 180, 252),
      secondary: rgb(16, 185, 129),
      secondary_dark: rgb(5, 150, 105),
      secondary_light: rgb(110, 231, 183),
      background: rgb(255, 255, 255),
      background_alt: rgb(249, 250, 251),
      surface: rgb(243, 244, 246),
      surface_alt: rgb(229, 231, 235),
      text: rgb(17, 24, 39),
      text_secondary: rgb(55, 65, 81),
      text_muted: rgb(107, 114, 128),
      text_inverse: rgb(249, 250, 251),
      border: rgb(209, 213, 219),
      border_focus: rgb(99, 102, 241),
      border_hover: rgb(156, 163, 175),
      success: rgb(34, 197, 94),
      warning: rgb(245, 158, 11),
      error: rgb(239, 68, 68),
      info: rgb(59, 130, 246),
      hover: rgb(129, 140, 248),
      active: rgb(109, 40, 217),
      disabled: rgb(156, 163, 175),
      shadow: rgb(0, 0, 0),
      shadow_light: rgb(107, 114, 128),
    },
    semantic: default_semantic_mapping(),
  }
}

/// Built-in terminal theme
pub fn terminal_theme() -> ColorTheme {
  ColorTheme {
    name: "terminal".to_string(),
    description: "Classic terminal colors for retro feel".to_string(),
    mode: ColorMode::Ansi,
    palette: ColorPalette {
      primary: rgb(0, 255, 0),
      primary_dark: rgb(0, 128, 0),
      primary_light: rgb(144, 238, 144),
      secondary: rgb(255, 255, 0),
      secondary_dark: rgb(255, 165, 0),
      secondary_light: rgb(255, 255, 224),
      background: rgb(0, 0, 0),
      background_alt: rgb(32, 32, 32),
      surface: rgb(64, 64, 64),
      surface_alt: rgb(96, 96, 96),
      text: rgb(255, 255, 255),
      text_secondary: rgb(192, 192, 192),
      text_muted: rgb(128, 128, 128),
      text_inverse: rgb(0, 0, 0),
      border: rgb(128, 128, 128),
      border_focus: rgb(0, 255, 0),
      border_hover: rgb(192, 192, 192),
      success: rgb(0, 255, 0),
      warning: rgb(255, 255, 0),
      error: rgb(255, 0, 0),
      info: rgb(0, 255, 255),
      hover: rgb(0, 128, 0),
      active: rgb(0, 64, 0),
      disabled: rgb(64, 64, 64),
      shadow: rgb(0, 0, 0),
      shadow_light: rgb(32, 32, 32),
    },
    semantic: default_semantic_mapping(),
  }
}

/// Default semantic color mappings
pub fn default_semantic_mapping() -> SemanticColorMapping {
  SemanticColorMapping {
    panel_background: "surface".to_string(),
    panel_border: "border".to_string(),
    panel_title: "text".to_string(),
    panel_content: "text_secondary".to_string(),
    panel_shadow: "shadow".to_string(),
    button_background: "primary".to_string(),
    button_border: "primary_dark".to_string(),
    button_text: "text_inverse".to_string(),
    button_hover: "hover".to_string(),
    input_background: "background_alt".to_string(),
    input_border: "border".to_string(),
    input_text: "text".to_string(),
    input_focus: "border_focus".to_string(),
    progress_background: "surface".to_string(),
    progress_fill: "primary".to_string(),
    progress_text: "text".to_string(),

    // Rich text editor colors
    editor_background: "background".to_string(),
    editor_text: "text".to_string(),
    editor_cursor: "primary".to_string(),
    editor_line_number: "text_muted".to_string(),
    editor_selection: "primary".to_string(),
    editor_border: "border".to_string(),
    editor_border_focus: "border_focus".to_string(),

    // Syntax highlighting colors
    syntax_keyword: "primary".to_string(),
    syntax_string: "success".to_string(),
    syntax_comment: "text_muted".to_string(),
    syntax_number: "warning".to_string(),
    syntax_function: "info".to_string(),
    syntax_type: "secondary".to_string(),
    syntax_variable: "text".to_string(),
    syntax_operator: "text_secondary".to_string(),
    syntax_punctuation: "text_secondary".to_string(),
    syntax_constant: "warning".to_string(),
    syntax_tag: "primary".to_string(),
    syntax_attribute: "secondary".to_string(),
  }
}

/// Theme registry for dynamic theme management
pub struct ThemeRegistry {
  themes: HashMap<String, ColorTheme>,
}

impl ThemeRegistry {
  pub fn new() -> Self {
    let mut registry = Self {
      themes: HashMap::new(),
    };

    // Register built-in themes
    registry.register_theme(dark_theme());
    registry.register_theme(light_theme());
    registry.register_theme(terminal_theme());

    registry
  }

  pub fn register_theme(&mut self, theme: ColorTheme) {
    self.themes.insert(theme.name.clone(), theme);
  }

  pub fn get_theme(&self, name: &str) -> Option<&ColorTheme> {
    self.themes.get(name)
  }

  pub fn get_theme_names(&self) -> Vec<String> {
    self.themes.keys().cloned().collect()
  }

  pub fn get_default_theme(&self) -> &ColorTheme {
    self.themes.get("dark").unwrap()
  }
}

impl Default for ThemeRegistry {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rgb_color_creation() {
    let color = rgb(255, 128, 64);
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);
  }

  #[test]
  fn test_hex_color_parsing() {
    let color = hex("#FF8040").unwrap();
    assert_eq!(color.r, 255);
    assert_eq!(color.g, 128);
    assert_eq!(color.b, 64);

    let color2 = hex("FF8040").unwrap();
    assert_eq!(color2, color);
  }

  #[test]
  fn test_invalid_hex_color() {
    assert!(hex("#ZZZ").is_err());
    assert!(hex("#12345").is_err());
    assert!(hex("#1234567").is_err());
  }

  #[test]
  fn test_color_variant() {
    let base = rgb(100, 100, 100);
    let lighter = create_variant(base, 0.5);
    let darker = create_variant(base, -0.5);

    assert!(lighter.r > base.r);
    assert!(darker.r < base.r);
  }

  #[test]
  fn test_ansi_conversion() {
    use crate::themes::color_support::{get_compatible_ansi_color, ColorSupport};

    let color = rgb(255, 0, 0);

    // Test truecolor support directly
    let ansi = get_compatible_ansi_color(color.r, color.g, color.b, false, ColorSupport::Truecolor);
    assert_eq!(ansi, "\x1B[38;2;255;0;0m");

    let bg_ansi =
      get_compatible_ansi_color(color.r, color.g, color.b, true, ColorSupport::Truecolor);
    assert_eq!(bg_ansi, "\x1B[48;2;255;0;0m");
  }

  #[test]
  fn test_theme_registry() {
    let registry = ThemeRegistry::new();
    assert!(registry.get_theme("dark").is_some());
    assert!(registry.get_theme("light").is_some());
    assert!(registry.get_theme("terminal").is_some());
    assert!(registry.get_theme("nonexistent").is_none());
  }
}
