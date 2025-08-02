/*!
 * Theme System Module
 *
 * Provides JSON theme loading, color management, and ANSI escape code generation
 * for the Rust TUI framework. Mirrors the TypeScript implementation.
 */

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
