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
//! let red = hex("#FF0000").unwrap();
//!
//! // Convert to ANSI
//! let ansi_code = color_to_ansi(blue, false);
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
//!     primary_dark: rgb(0, 100, 220),
//!     primary_light: rgb(100, 180, 255),
//!     secondary: rgb(108, 117, 125),
//!     secondary_dark: rgb(80, 90, 100),
//!     secondary_light: rgb(150, 160, 170),
//!     background: rgb(255, 255, 255),
//!     background_alt: rgb(248, 249, 250),
//!     surface: rgb(255, 255, 255),
//!     surface_alt: rgb(245, 245, 245),
//!     text: rgb(0, 0, 0),
//!     text_secondary: rgb(108, 117, 125),
//!     text_muted: rgb(168, 175, 180),
//!     text_inverse: rgb(255, 255, 255),
//!     border: rgb(200, 200, 200),
//!     border_focus: rgb(0, 122, 255),
//!     border_hover: rgb(150, 150, 150),
//!     success: rgb(40, 167, 69),
//!     warning: rgb(255, 193, 7),
//!     error: rgb(220, 53, 69),
//!     info: rgb(23, 162, 184),
//!     hover: rgb(0, 100, 220),
//!     active: rgb(0, 80, 180),
//!     disabled: rgb(200, 200, 200),
//!     shadow: rgb(0, 0, 0),
//!     shadow_light: rgb(100, 100, 100),
//! };
//!
//! let semantic = SemanticColorMapping {
//!     panel_background: "background".to_string(),
//!     panel_border: "border".to_string(),
//!     panel_title: "text".to_string(),
//!     panel_content: "text".to_string(),
//!     panel_shadow: "shadow".to_string(),
//!     button_background: "primary".to_string(),
//!     button_border: "primary".to_string(),
//!     button_text: "text_inverse".to_string(),
//!     button_hover: "hover".to_string(),
//!     input_background: "surface".to_string(),
//!     input_border: "border".to_string(),
//!     input_text: "text".to_string(),
//!     input_focus: "border_focus".to_string(),
//!     progress_background: "surface_alt".to_string(),
//!     progress_fill: "primary".to_string(),
//!     progress_text: "text".to_string(),
//!     editor_background: "background".to_string(),
//!     editor_text: "text".to_string(),
//!     editor_cursor: "primary".to_string(),
//!     editor_line_number: "text_muted".to_string(),
//!     editor_selection: "hover".to_string(),
//!     editor_border: "border".to_string(),
//!     editor_border_focus: "border_focus".to_string(),
//!     syntax_keyword: "primary".to_string(),
//!     syntax_string: "success".to_string(),
//!     syntax_comment: "text_muted".to_string(),
//!     syntax_number: "info".to_string(),
//!     syntax_function: "primary".to_string(),
//!     syntax_type: "warning".to_string(),
//!     syntax_variable: "text".to_string(),
//!     syntax_operator: "text".to_string(),
//!     syntax_punctuation: "text".to_string(),
//!     syntax_constant: "error".to_string(),
//!     syntax_tag: "primary".to_string(),
//!     syntax_attribute: "warning".to_string(),
//! };
//!
//! let theme = ColorTheme {
//!     name: "Custom Theme".to_string(),
//!     description: "My custom color theme".to_string(),
//!     mode: ColorMode::Rgb,
//!     palette,
//!     semantic,
//! };
//! ```

pub mod borders;
pub mod color_support;
pub mod colors;
pub mod json_loader;
pub mod utility_css;

pub use colors::{
  color_to_ansi, create_variant, get_palette_color, get_semantic_background, get_semantic_color, hex, rgb,
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
