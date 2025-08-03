/*!
 * Terminal Color Support Detection
 *
 * Optimized color capability detection with caching to prevent performance bottlenecks.
 * Inspired by r3bl-open-core's excellent implementation patterns.
 */

use std::{
  env,
  sync::atomic::{AtomicI8, Ordering},
};

/// Terminal color support levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorSupport {
  /// 24-bit truecolor support (16.7M colors)
  Truecolor,
  /// 8-bit ANSI 256 colors
  Ansi256,
  /// 4-bit ANSI 16 colors
  Ansi16,
  /// Grayscale only
  Grayscale,
  /// No color support
  NoColor,
}

/// Output stream type for color detection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
  Stdout,
  Stderr,
}

/// Global color support detection with performance optimization
pub mod global_color_support {
  use super::{
    examine_env_vars_to_determine_color_support, AtomicI8, ColorSupport, Ordering, Stream,
  };

  /// Global override for color support (highest priority)
  static COLOR_SUPPORT_GLOBAL: AtomicI8 = AtomicI8::new(-1);

  /// Cached color support detection (performance optimization)
  static COLOR_SUPPORT_CACHED: AtomicI8 = AtomicI8::new(-1);

  /// Detect color support with caching for performance
  pub fn detect() -> ColorSupport {
    // Check global override first (highest priority)
    let override_value = COLOR_SUPPORT_GLOBAL.load(Ordering::Relaxed);
    if override_value != -1 {
      return ColorSupport::try_from(override_value).unwrap_or(ColorSupport::NoColor);
    }

    // Check cached value
    let cached_value = COLOR_SUPPORT_CACHED.load(Ordering::Relaxed);
    if cached_value != -1 {
      return ColorSupport::try_from(cached_value).unwrap_or(ColorSupport::NoColor);
    }

    // Perform detection and cache result
    let detected = examine_env_vars_to_determine_color_support(Stream::Stdout);
    COLOR_SUPPORT_CACHED.store(detected as i8, Ordering::Relaxed);
    detected
  }

  /// Set global color support override
  pub fn set_override(color_support: ColorSupport) {
    COLOR_SUPPORT_GLOBAL.store(color_support as i8, Ordering::Relaxed);
  }

  /// Clear global color support override
  pub fn clear_override() {
    COLOR_SUPPORT_GLOBAL.store(-1, Ordering::Relaxed);
  }

  /// Clear cached detection result (force re-detection)
  pub fn clear_cache() {
    COLOR_SUPPORT_CACHED.store(-1, Ordering::Relaxed);
  }
}

/// Examine environment variables to determine color support
pub fn examine_env_vars_to_determine_color_support(stream: Stream) -> ColorSupport {
  // Check NO_COLOR environment variable (https://no-color.org/)
  if env::var("NO_COLOR").is_ok() {
    return ColorSupport::NoColor;
  }

  // Check FORCE_COLOR environment variable
  if let Ok(force_color) = env::var("FORCE_COLOR") {
    match force_color.as_str() {
      "0" => return ColorSupport::NoColor,
      "1" => return ColorSupport::Ansi16,
      "2" => return ColorSupport::Ansi256,
      "3" => return ColorSupport::Truecolor,
      _ => {} // Continue with detection
    }
  }

  // Check if we're in a TTY
  if !is_tty(stream) {
    return ColorSupport::NoColor;
  }

  // Check TERM environment variable
  let term = env::var("TERM").unwrap_or_default();
  if term.is_empty() || term == "dumb" {
    return ColorSupport::NoColor;
  }

  // Check COLORTERM for truecolor support
  if let Ok(colorterm) = env::var("COLORTERM") {
    if colorterm == "truecolor" || colorterm == "24bit" {
      return ColorSupport::Truecolor;
    }
  }

  // Check TERM for truecolor indicators
  if term.contains("truecolor") || term.contains("24bit") {
    return ColorSupport::Truecolor;
  }

  // Check for iTerm2 (supports truecolor)
  if env::var("TERM_PROGRAM").unwrap_or_default() == "iTerm.app" {
    return ColorSupport::Truecolor;
  }

  // Check for VS Code terminal (supports truecolor)
  if env::var("TERM_PROGRAM").unwrap_or_default() == "vscode" {
    return ColorSupport::Truecolor;
  }

  // Check for Windows Terminal (supports truecolor)
  if env::var("WT_SESSION").is_ok() {
    return ColorSupport::Truecolor;
  }

  // Check for known 256-color terminals
  if term.contains("256") || term.contains("256color") {
    return ColorSupport::Ansi256;
  }

  // macOS Terminal.app detection (only supports 256 colors, not truecolor)
  if cfg!(target_os = "macos") {
    if let Ok(term_program) = env::var("TERM_PROGRAM") {
      if term_program == "Apple_Terminal" {
        return ColorSupport::Ansi256;
      }
    }
  }

  // Check TERM for color support indicators
  if term.contains("color") {
    return ColorSupport::Ansi256;
  }

  // Known color-supporting terminals
  let color_terms = [
    "xterm",
    "screen",
    "tmux",
    "rxvt",
    "konsole",
    "gnome",
    "mate",
    "terminator",
    "alacritty",
    "kitty",
    "hyper",
    "wezterm",
  ];

  for color_term in &color_terms {
    if term.contains(color_term) {
      return ColorSupport::Ansi256;
    }
  }

  // Default to no color if we can't determine support
  ColorSupport::NoColor
}

/// Check if the given stream is a TTY
fn is_tty(stream: Stream) -> bool {
  use is_terminal::IsTerminal;

  match stream {
    Stream::Stdout => std::io::stdout().is_terminal(),
    Stream::Stderr => std::io::stderr().is_terminal(),
  }
}

/// Convert ColorSupport to i8 for atomic storage
impl From<ColorSupport> for i8 {
  fn from(color_support: ColorSupport) -> Self {
    match color_support {
      ColorSupport::Truecolor => 3,
      ColorSupport::Ansi256 => 2,
      ColorSupport::Ansi16 => 1,
      ColorSupport::Grayscale => 0,
      ColorSupport::NoColor => -1,
    }
  }
}

/// Convert i8 to ColorSupport for atomic retrieval
impl TryFrom<i8> for ColorSupport {
  type Error = ();

  fn try_from(value: i8) -> Result<Self, Self::Error> {
    match value {
      3 => Ok(ColorSupport::Truecolor),
      2 => Ok(ColorSupport::Ansi256),
      1 => Ok(ColorSupport::Ansi16),
      0 => Ok(ColorSupport::Grayscale),
      -1 => Ok(ColorSupport::NoColor),
      _ => Err(()),
    }
  }
}

/// Get appropriate ANSI color code based on terminal capability
pub fn get_compatible_ansi_color(
  r: u8,
  g: u8,
  b: u8,
  background: bool,
  color_support: ColorSupport,
) -> String {
  match color_support {
    ColorSupport::Truecolor => {
      let base = if background { 48 } else { 38 };
      format!("\x1B[{base};2;{r};{g};{b}m")
    }
    ColorSupport::Ansi256 => {
      let ansi_code = rgb_to_ansi256(r, g, b);
      let base = if background { 48 } else { 38 };
      format!("\x1B[{base};5;{ansi_code}m")
    }
    ColorSupport::Ansi16 => {
      let ansi_code = rgb_to_ansi16(r, g, b);
      let base = if background { 40 } else { 30 };
      format!("\x1B[{}m", base + ansi_code)
    }
    ColorSupport::Grayscale => {
      let gray = rgb_to_grayscale(r, g, b);
      let ansi_code = if gray > 128 { 37 } else { 30 }; // White or black
      let base = if background {
        ansi_code + 10
      } else {
        ansi_code
      };
      format!("\x1B[{base}m")
    }
    ColorSupport::NoColor => String::new(),
  }
}

/// Convert RGB to ANSI 256 color code
fn rgb_to_ansi256(r: u8, g: u8, b: u8) -> u8 {
  // Convert RGB to 6x6x6 color cube
  let r6 = (r as f32 / 255.0 * 5.0).round() as u8;
  let g6 = (g as f32 / 255.0 * 5.0).round() as u8;
  let b6 = (b as f32 / 255.0 * 5.0).round() as u8;

  // Color cube starts at 16
  16 + (36 * r6) + (6 * g6) + b6
}

/// Convert RGB to ANSI 16 color code (0-7)
fn rgb_to_ansi16(r: u8, g: u8, b: u8) -> u8 {
  let r_bit = if r > 128 { 1 } else { 0 };
  let g_bit = if g > 128 { 2 } else { 0 };
  let b_bit = if b > 128 { 4 } else { 0 };

  r_bit + g_bit + b_bit
}

/// Convert RGB to grayscale value
fn rgb_to_grayscale(r: u8, g: u8, b: u8) -> u8 {
  // Use standard luminance weights
  (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_color_support_conversion() {
    let truecolor = ColorSupport::Truecolor;
    let as_i8: i8 = truecolor.into();
    let back_to_enum = ColorSupport::try_from(as_i8).unwrap();
    assert_eq!(truecolor, back_to_enum);
  }

  #[test]
  fn test_rgb_to_ansi256() {
    // Test red
    let red_ansi = rgb_to_ansi256(255, 0, 0);
    assert_eq!(red_ansi, 196); // Standard red in 256-color palette

    // Test white
    let white_ansi = rgb_to_ansi256(255, 255, 255);
    assert_eq!(white_ansi, 231); // Standard white in 256-color palette
  }

  #[test]
  fn test_rgb_to_ansi16() {
    // Test red
    let red_ansi = rgb_to_ansi16(255, 0, 0);
    assert_eq!(red_ansi, 1); // Red bit only

    // Test white
    let white_ansi = rgb_to_ansi16(255, 255, 255);
    assert_eq!(white_ansi, 7); // All bits set
  }

  #[test]
  fn test_global_color_support() {
    // Save original state (for potential future restoration)
    let _original_override = global_color_support::detect();

    // Test override
    global_color_support::set_override(ColorSupport::NoColor);
    assert_eq!(global_color_support::detect(), ColorSupport::NoColor);

    // Clear override and cache to ensure fresh detection
    global_color_support::clear_override();
    global_color_support::clear_cache();

    // Should now use detection (may vary by environment)
    let detected = global_color_support::detect();
    assert!(matches!(
      detected,
      ColorSupport::NoColor
        | ColorSupport::Ansi16
        | ColorSupport::Ansi256
        | ColorSupport::Truecolor
    ));

    // Clean up: restore original state if it was an override
    // Note: We can't perfectly restore the original state since we don't know
    // if it was an override or detection, but clearing everything is safest
    global_color_support::clear_override();
    global_color_support::clear_cache();
  }

  #[test]
  fn test_compatible_ansi_color() {
    let red_true = get_compatible_ansi_color(255, 0, 0, false, ColorSupport::Truecolor);
    assert_eq!(red_true, "\x1B[38;2;255;0;0m");

    let red_256 = get_compatible_ansi_color(255, 0, 0, false, ColorSupport::Ansi256);
    assert_eq!(red_256, "\x1B[38;5;196m");

    let no_color = get_compatible_ansi_color(255, 0, 0, false, ColorSupport::NoColor);
    assert_eq!(no_color, "");
  }
}
