//! # Theme System
//!
//! Comprehensive theming and color management for terminal user interfaces.
//!
//! This module provides a complete theming system supporting JSON-based theme definitions,
//! color palette management, ANSI color code generation, and automatic terminal capability
//! detection. The theme system integrates seamlessly with CSS styling and provides both
//! semantic color mappings and utility class generation.
//!
//! ## Features
//!
//! - **JSON Themes**: Load and save themes from JSON files
//! - **Color Management**: RGB, hex, and ANSI color support
//! - **Terminal Detection**: Automatic color capability detection
//! - **Semantic Colors**: Predefined color roles (primary, success, danger, etc.)
//! - **Utility Classes**: Generate CSS utility classes from color palettes
//! - **Border Styles**: Configurable border sets and styles
//! - **Color Variants**: Automatic generation of lighter/darker variants
//!
//! ## Examples
//!
//! ### Basic Color Usage
//!
//! ```rust,no_run
//! use reactive_tui::themes::*;
//!
//! // Create colors
//! let blue = rgb(0, 122, 255);
//! let red = hex("#FF0000");
//!
//! // Convert to ANSI
//! let ansi_code = color_to_ansi(&blue, false);
//! println!("{}Blue text{}", ansi_code, RESET_COLOR);
//! ```
//!
//! ### Theme Loading
//!
//! ```rust,no_run
//! use reactive_tui::themes::*;
//!
//! // Load theme from JSON file
//! let theme = load_theme_from_file("dark_theme.json")?;
//!
//! // Get semantic colors
//! let primary = get_semantic_color(&theme, "primary");
//! let background = get_semantic_background(&theme, "surface");
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ### Color Palette Creation
//!
//! ```rust,no_run
//! use reactive_tui::themes::*;
//!
//! let palette = ColorPalette {
//!     primary: rgb(0, 122, 255),
//!     secondary: rgb(108, 117, 125),
//!     success: rgb(40, 167, 69),
//!     danger: rgb(220, 53, 69),
//!     warning: rgb(255, 193, 7),
//!     info: rgb(23, 162, 184),
//!     light: rgb(248, 249, 250),
//!     dark: rgb(52, 58, 64),
//! };
//!
//! let theme = ColorTheme {
//!     name: "Custom Theme".to_string(),
//!     description: "My custom color theme".to_string(),
//!     palette,
//! };
//! ```

pub mod borders;
pub mod color_support;
pub mod colors;
pub mod json_loader;
pub mod utility_css;

pub use colors::{
  color_to_ansi, create_variant, get_semantic_background, get_semantic_color, hex, rgb,
  ColorDefinition, ColorMode, ColorPalette, ColorTheme, SemanticColorMapping, RESET_COLOR,
};

pub use json_loader::{
  get_theme_preview, load_theme_collection_from_file, load_theme_from_file, parse_json_color,
  save_theme_to_file, theme_to_ansi_codes, validate_json_color, validate_json_theme,
  JSONColorDefinition, JSONColorPalette, JSONColorTheme, JSONThemeCollection,
};

pub use borders::{get_border_set, BorderSet, BorderStyle};

pub use color_support::{
  examine_env_vars_to_determine_color_support, get_compatible_ansi_color, global_color_support,
  ColorSupport, Stream,
};

pub use utility_css::{
  default_utility_palette, generate_utility_classes, UtilityClasses, UtilityPalette,
  UtilityProcessor,
};
