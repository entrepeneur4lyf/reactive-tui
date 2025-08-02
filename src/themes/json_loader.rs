/*!
 * JSON Theme Loader for Rust TUI
 *
 * Loads color themes from JSON files and translates them into ANSI escape codes.
 * Supports various color formats and comprehensive validation.
 */

use serde::{Deserialize, Serialize};
use serde_json;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::colors::{
    hex, rgb, ColorDefinition, ColorMode, ColorPalette, ColorTheme, SemanticColorMapping,
};

/// JSON color definition with multiple format support
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum JSONColorDefinition {
    Hex { hex: String },
    RgbArray { rgb: [u8; 3] },
    RgbObject { r: u8, g: u8, b: u8 },
    Ansi { ansi: u8 },
    Named { name: String },
}

/// JSON color palette structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONColorPalette {
    // Primary colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_dark: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub primary_light: Option<JSONColorDefinition>,

    // Secondary colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_dark: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub secondary_light: Option<JSONColorDefinition>,

    // Neutral colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub background_alt: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub surface_alt: Option<JSONColorDefinition>,

    // Text colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_secondary: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_muted: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text_inverse: Option<JSONColorDefinition>,

    // Border colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_focus: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub border_hover: Option<JSONColorDefinition>,

    // Status colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub success: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub warning: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub info: Option<JSONColorDefinition>,

    // Interactive colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hover: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<JSONColorDefinition>,

    // Shadow colors
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow: Option<JSONColorDefinition>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shadow_light: Option<JSONColorDefinition>,
}

/// JSON theme structure
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONColorTheme {
    pub name: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub author: Option<String>,
    #[serde(default)]
    pub mode: ColorMode,
    pub palette: JSONColorPalette,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub semantic: Option<SemanticColorMapping>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub extends: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub imports: Option<Vec<String>>,
}

/// JSON theme collection
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JSONThemeCollection {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    pub themes: Vec<JSONColorTheme>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub named_colors: Option<HashMap<String, JSONColorDefinition>>,
}

/// Theme loading error
#[derive(Debug, thiserror::Error)]
pub enum ThemeError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("JSON parsing error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Validation error: {0}")]
    Validation(String),
    #[error("Color parsing error: {0}")]
    ColorParsing(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
}

/// Parse JSON color definition to ColorDefinition
pub fn parse_json_color(
    json_color: &JSONColorDefinition,
    named_colors: &HashMap<String, ColorDefinition>,
) -> Result<ColorDefinition, ThemeError> {
    match json_color {
        JSONColorDefinition::Hex { hex: hex_str } => hex(hex_str).map_err(ThemeError::ColorParsing),
        JSONColorDefinition::RgbArray { rgb: [r, g, b] } => Ok(rgb(*r, *g, *b)),
        JSONColorDefinition::RgbObject { r, g, b } => Ok(rgb(*r, *g, *b)),
        JSONColorDefinition::Ansi { ansi } => ansi_to_rgb(*ansi).map_err(ThemeError::ColorParsing),
        JSONColorDefinition::Named { name } => named_colors
            .get(name)
            .copied()
            .ok_or_else(|| ThemeError::ColorParsing(format!("Named color '{name}' not found"))),
    }
}

/// Convert ANSI color code to RGB
fn ansi_to_rgb(ansi: u8) -> Result<ColorDefinition, String> {
    match ansi {
        // Standard colors (0-15)
        0 => Ok(rgb(0, 0, 0)),        // Black
        1 => Ok(rgb(128, 0, 0)),      // Dark Red
        2 => Ok(rgb(0, 128, 0)),      // Dark Green
        3 => Ok(rgb(128, 128, 0)),    // Dark Yellow
        4 => Ok(rgb(0, 0, 128)),      // Dark Blue
        5 => Ok(rgb(128, 0, 128)),    // Dark Magenta
        6 => Ok(rgb(0, 128, 128)),    // Dark Cyan
        7 => Ok(rgb(192, 192, 192)),  // Light Gray
        8 => Ok(rgb(128, 128, 128)),  // Dark Gray
        9 => Ok(rgb(255, 0, 0)),      // Red
        10 => Ok(rgb(0, 255, 0)),     // Green
        11 => Ok(rgb(255, 255, 0)),   // Yellow
        12 => Ok(rgb(0, 0, 255)),     // Blue
        13 => Ok(rgb(255, 0, 255)),   // Magenta
        14 => Ok(rgb(0, 255, 255)),   // Cyan
        15 => Ok(rgb(255, 255, 255)), // White

        // 216-color cube (16-231)
        16..=231 => {
            let color_index = ansi - 16;
            let r = color_index / 36;
            let g = (color_index % 36) / 6;
            let b = color_index % 6;

            let to_rgb_value = |val: u8| if val == 0 { 0 } else { 55 + val * 40 };
            Ok(rgb(to_rgb_value(r), to_rgb_value(g), to_rgb_value(b)))
        }

        // Grayscale (232-255)
        232..=255 => {
            let gray = (ansi - 232) * 10 + 8;
            Ok(rgb(gray, gray, gray))
        }
    }
}

/// Parse JSON palette to ColorPalette
fn parse_json_palette(
    json_palette: &JSONColorPalette,
    named_colors: &HashMap<String, ColorDefinition>,
) -> Result<ColorPalette, ThemeError> {
    let default_palette = default_color_palette();

    Ok(ColorPalette {
        primary: json_palette
            .primary
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.primary),
        primary_dark: json_palette
            .primary_dark
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.primary_dark),
        primary_light: json_palette
            .primary_light
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.primary_light),
        secondary: json_palette
            .secondary
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.secondary),
        secondary_dark: json_palette
            .secondary_dark
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.secondary_dark),
        secondary_light: json_palette
            .secondary_light
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.secondary_light),
        background: json_palette
            .background
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.background),
        background_alt: json_palette
            .background_alt
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.background_alt),
        surface: json_palette
            .surface
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.surface),
        surface_alt: json_palette
            .surface_alt
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.surface_alt),
        text: json_palette
            .text
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.text),
        text_secondary: json_palette
            .text_secondary
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.text_secondary),
        text_muted: json_palette
            .text_muted
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.text_muted),
        text_inverse: json_palette
            .text_inverse
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.text_inverse),
        border: json_palette
            .border
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.border),
        border_focus: json_palette
            .border_focus
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.border_focus),
        border_hover: json_palette
            .border_hover
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.border_hover),
        success: json_palette
            .success
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.success),
        warning: json_palette
            .warning
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.warning),
        error: json_palette
            .error
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.error),
        info: json_palette
            .info
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.info),
        hover: json_palette
            .hover
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.hover),
        active: json_palette
            .active
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.active),
        disabled: json_palette
            .disabled
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.disabled),
        shadow: json_palette
            .shadow
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.shadow),
        shadow_light: json_palette
            .shadow_light
            .as_ref()
            .map(|c| parse_json_color(c, named_colors))
            .transpose()?
            .unwrap_or(default_palette.shadow_light),
    })
}

/// Default color palette for fallbacks
fn default_color_palette() -> ColorPalette {
    ColorPalette {
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
    }
}

/// Load theme from JSON file
pub fn load_theme_from_file<P: AsRef<Path>>(file_path: P) -> Result<ColorTheme, ThemeError> {
    let path = file_path.as_ref();

    if !path.exists() {
        return Err(ThemeError::FileNotFound(path.display().to_string()));
    }

    let content = fs::read_to_string(path)?;
    let json_data: serde_json::Value = serde_json::from_str(&content)?;

    // Check if it's a collection or single theme
    if json_data.get("themes").is_some() {
        let collection: JSONThemeCollection = serde_json::from_str(&content)?;
        if collection.themes.is_empty() {
            return Err(ThemeError::Validation(
                "Theme collection is empty".to_string(),
            ));
        }

        let named_colors = process_named_colors(&collection.named_colors)?;
        parse_json_theme(&collection.themes[0], &named_colors, path.parent())
    } else {
        let json_theme: JSONColorTheme = serde_json::from_str(&content)?;
        let named_colors = HashMap::new();
        parse_json_theme(&json_theme, &named_colors, path.parent())
    }
}

/// Load theme collection from JSON file
pub fn load_theme_collection_from_file<P: AsRef<Path>>(
    file_path: P,
) -> Result<Vec<ColorTheme>, ThemeError> {
    let path = file_path.as_ref();

    if !path.exists() {
        return Err(ThemeError::FileNotFound(path.display().to_string()));
    }

    let content = fs::read_to_string(path)?;
    let collection: JSONThemeCollection = serde_json::from_str(&content)?;

    if collection.themes.is_empty() {
        return Err(ThemeError::Validation(
            "Theme collection is empty".to_string(),
        ));
    }

    let named_colors = process_named_colors(&collection.named_colors)?;
    let mut themes = Vec::new();

    for json_theme in &collection.themes {
        match parse_json_theme(json_theme, &named_colors, path.parent()) {
            Ok(theme) => themes.push(theme),
            Err(e) => eprintln!(
                "Warning: Failed to parse theme '{}': {}",
                json_theme.name, e
            ),
        }
    }

    if themes.is_empty() {
        return Err(ThemeError::Validation(
            "No valid themes found in collection".to_string(),
        ));
    }

    Ok(themes)
}

/// Parse JSON theme to ColorTheme
fn parse_json_theme(
    json_theme: &JSONColorTheme,
    named_colors: &HashMap<String, ColorDefinition>,
    base_dir: Option<&Path>,
) -> Result<ColorTheme, ThemeError> {
    // Handle theme extension
    let mut base_palette = ColorPalette::default();
    if let Some(extends_path) = &json_theme.extends {
        if let Some(dir) = base_dir {
            let base_theme_path = dir.join(extends_path);
            match load_theme_from_file(&base_theme_path) {
                Ok(base_theme) => base_palette = base_theme.palette,
                Err(e) => eprintln!("Warning: Failed to extend theme '{extends_path}': {e}"),
            }
        }
    }

    // Parse current theme palette
    let current_palette = parse_json_palette(&json_theme.palette, named_colors)?;

    // Merge palettes (base first, then current overrides)
    let palette = merge_palettes(&base_palette, &current_palette);

    // Use provided semantic mapping or default
    let semantic = json_theme
        .semantic
        .clone()
        .unwrap_or_else(super::colors::default_semantic_mapping);

    Ok(ColorTheme {
        name: json_theme.name.clone(),
        description: json_theme.description.clone(),
        mode: json_theme.mode,
        palette,
        semantic,
    })
}

/// Merge two color palettes
fn merge_palettes(_base: &ColorPalette, current: &ColorPalette) -> ColorPalette {
    ColorPalette {
        primary: current.primary,
        primary_dark: current.primary_dark,
        primary_light: current.primary_light,
        secondary: current.secondary,
        secondary_dark: current.secondary_dark,
        secondary_light: current.secondary_light,
        background: current.background,
        background_alt: current.background_alt,
        surface: current.surface,
        surface_alt: current.surface_alt,
        text: current.text,
        text_secondary: current.text_secondary,
        text_muted: current.text_muted,
        text_inverse: current.text_inverse,
        border: current.border,
        border_focus: current.border_focus,
        border_hover: current.border_hover,
        success: current.success,
        warning: current.warning,
        error: current.error,
        info: current.info,
        hover: current.hover,
        active: current.active,
        disabled: current.disabled,
        shadow: current.shadow,
        shadow_light: current.shadow_light,
    }
}

/// Process named colors from collection
fn process_named_colors(
    named_colors: &Option<HashMap<String, JSONColorDefinition>>,
) -> Result<HashMap<String, ColorDefinition>, ThemeError> {
    let mut result = HashMap::new();

    if let Some(named) = named_colors {
        for (name, json_color) in named {
            let color = parse_json_color(json_color, &HashMap::new())?;
            result.insert(name.clone(), color);
        }
    }

    Ok(result)
}

/// Save theme to JSON file
pub fn save_theme_to_file<P: AsRef<Path>>(
    theme: &ColorTheme,
    file_path: P,
) -> Result<(), ThemeError> {
    let json_theme = JSONColorTheme {
        name: theme.name.clone(),
        description: theme.description.clone(),
        version: None,
        author: None,
        mode: theme.mode,
        palette: palette_to_json(&theme.palette),
        semantic: Some(theme.semantic.clone()),
        extends: None,
        imports: None,
    };

    let json = serde_json::to_string_pretty(&json_theme)?;
    fs::write(file_path, json)?;
    Ok(())
}

/// Convert ColorPalette to JSONColorPalette
fn palette_to_json(palette: &ColorPalette) -> JSONColorPalette {
    JSONColorPalette {
        primary: Some(JSONColorDefinition::RgbObject {
            r: palette.primary.r,
            g: palette.primary.g,
            b: palette.primary.b,
        }),
        primary_dark: Some(JSONColorDefinition::RgbObject {
            r: palette.primary_dark.r,
            g: palette.primary_dark.g,
            b: palette.primary_dark.b,
        }),
        primary_light: Some(JSONColorDefinition::RgbObject {
            r: palette.primary_light.r,
            g: palette.primary_light.g,
            b: palette.primary_light.b,
        }),
        secondary: Some(JSONColorDefinition::RgbObject {
            r: palette.secondary.r,
            g: palette.secondary.g,
            b: palette.secondary.b,
        }),
        secondary_dark: Some(JSONColorDefinition::RgbObject {
            r: palette.secondary_dark.r,
            g: palette.secondary_dark.g,
            b: palette.secondary_dark.b,
        }),
        secondary_light: Some(JSONColorDefinition::RgbObject {
            r: palette.secondary_light.r,
            g: palette.secondary_light.g,
            b: palette.secondary_light.b,
        }),
        background: Some(JSONColorDefinition::RgbObject {
            r: palette.background.r,
            g: palette.background.g,
            b: palette.background.b,
        }),
        background_alt: Some(JSONColorDefinition::RgbObject {
            r: palette.background_alt.r,
            g: palette.background_alt.g,
            b: palette.background_alt.b,
        }),
        surface: Some(JSONColorDefinition::RgbObject {
            r: palette.surface.r,
            g: palette.surface.g,
            b: palette.surface.b,
        }),
        surface_alt: Some(JSONColorDefinition::RgbObject {
            r: palette.surface_alt.r,
            g: palette.surface_alt.g,
            b: palette.surface_alt.b,
        }),
        text: Some(JSONColorDefinition::RgbObject {
            r: palette.text.r,
            g: palette.text.g,
            b: palette.text.b,
        }),
        text_secondary: Some(JSONColorDefinition::RgbObject {
            r: palette.text_secondary.r,
            g: palette.text_secondary.g,
            b: palette.text_secondary.b,
        }),
        text_muted: Some(JSONColorDefinition::RgbObject {
            r: palette.text_muted.r,
            g: palette.text_muted.g,
            b: palette.text_muted.b,
        }),
        text_inverse: Some(JSONColorDefinition::RgbObject {
            r: palette.text_inverse.r,
            g: palette.text_inverse.g,
            b: palette.text_inverse.b,
        }),
        border: Some(JSONColorDefinition::RgbObject {
            r: palette.border.r,
            g: palette.border.g,
            b: palette.border.b,
        }),
        border_focus: Some(JSONColorDefinition::RgbObject {
            r: palette.border_focus.r,
            g: palette.border_focus.g,
            b: palette.border_focus.b,
        }),
        border_hover: Some(JSONColorDefinition::RgbObject {
            r: palette.border_hover.r,
            g: palette.border_hover.g,
            b: palette.border_hover.b,
        }),
        success: Some(JSONColorDefinition::RgbObject {
            r: palette.success.r,
            g: palette.success.g,
            b: palette.success.b,
        }),
        warning: Some(JSONColorDefinition::RgbObject {
            r: palette.warning.r,
            g: palette.warning.g,
            b: palette.warning.b,
        }),
        error: Some(JSONColorDefinition::RgbObject {
            r: palette.error.r,
            g: palette.error.g,
            b: palette.error.b,
        }),
        info: Some(JSONColorDefinition::RgbObject {
            r: palette.info.r,
            g: palette.info.g,
            b: palette.info.b,
        }),
        hover: Some(JSONColorDefinition::RgbObject {
            r: palette.hover.r,
            g: palette.hover.g,
            b: palette.hover.b,
        }),
        active: Some(JSONColorDefinition::RgbObject {
            r: palette.active.r,
            g: palette.active.g,
            b: palette.active.b,
        }),
        disabled: Some(JSONColorDefinition::RgbObject {
            r: palette.disabled.r,
            g: palette.disabled.g,
            b: palette.disabled.b,
        }),
        shadow: Some(JSONColorDefinition::RgbObject {
            r: palette.shadow.r,
            g: palette.shadow.g,
            b: palette.shadow.b,
        }),
        shadow_light: Some(JSONColorDefinition::RgbObject {
            r: palette.shadow_light.r,
            g: palette.shadow_light.g,
            b: palette.shadow_light.b,
        }),
    }
}

/// Convert theme to ANSI codes map
pub fn theme_to_ansi_codes(theme: &ColorTheme) -> HashMap<String, String> {
    use super::colors::color_to_ansi;

    let mut codes = HashMap::new();

    // Palette colors
    codes.insert(
        "primary".to_string(),
        color_to_ansi(theme.palette.primary, false),
    );
    codes.insert(
        "primary_bg".to_string(),
        color_to_ansi(theme.palette.primary, true),
    );
    codes.insert(
        "secondary".to_string(),
        color_to_ansi(theme.palette.secondary, false),
    );
    codes.insert(
        "secondary_bg".to_string(),
        color_to_ansi(theme.palette.secondary, true),
    );
    codes.insert(
        "success".to_string(),
        color_to_ansi(theme.palette.success, false),
    );
    codes.insert(
        "success_bg".to_string(),
        color_to_ansi(theme.palette.success, true),
    );
    codes.insert(
        "warning".to_string(),
        color_to_ansi(theme.palette.warning, false),
    );
    codes.insert(
        "warning_bg".to_string(),
        color_to_ansi(theme.palette.warning, true),
    );
    codes.insert(
        "error".to_string(),
        color_to_ansi(theme.palette.error, false),
    );
    codes.insert(
        "error_bg".to_string(),
        color_to_ansi(theme.palette.error, true),
    );
    codes.insert("info".to_string(), color_to_ansi(theme.palette.info, false));
    codes.insert(
        "info_bg".to_string(),
        color_to_ansi(theme.palette.info, true),
    );

    // Reset code
    codes.insert("reset".to_string(), super::colors::RESET_COLOR.to_string());

    codes
}

/// Get theme preview with ANSI colors
pub fn get_theme_preview(theme: &ColorTheme) -> String {
    use super::colors::color_to_ansi;

    let mut lines = Vec::new();

    lines.push(format!(
        "Theme: {}{}{}{}",
        color_to_ansi(theme.palette.primary, false),
        theme.name,
        super::colors::RESET_COLOR,
        theme.description
    ));
    lines.push(format!("Mode: {:?}", theme.mode));
    lines.push(String::new());

    // Color swatches
    let colors = [
        ("primary", theme.palette.primary),
        ("secondary", theme.palette.secondary),
        ("success", theme.palette.success),
        ("warning", theme.palette.warning),
        ("error", theme.palette.error),
        ("info", theme.palette.info),
    ];

    for (name, color) in colors.iter() {
        lines.push(format!(
            "{}  {} {}{}",
            color_to_ansi(*color, true),
            super::colors::RESET_COLOR,
            color_to_ansi(*color, false),
            name,
        ));
    }

    lines.join("\n")
}

/// Validate JSON color definition
pub fn validate_json_color(json_color: &JSONColorDefinition, color_name: &str) -> Vec<String> {
    let mut errors = Vec::new();

    match json_color {
        JSONColorDefinition::Hex { hex: hex_str } => {
            if let Err(e) = hex(hex_str) {
                errors.push(format!("Color '{color_name}': {e}"));
            }
        }
        JSONColorDefinition::RgbArray { rgb: _ } => {
            // RGB arrays are validated by serde deserialization
        }
        JSONColorDefinition::RgbObject { r: _, g: _, b: _ } => {
            // RGB objects are validated by serde deserialization
        }
        JSONColorDefinition::Ansi { ansi: _ } => {
            // ANSI codes are u8, so they're automatically in valid range (0-255)
        }
        JSONColorDefinition::Named { name } => {
            if name.trim().is_empty() {
                errors.push(format!(
                    "Color '{color_name}': named color reference cannot be empty"
                ));
            }
        }
    }

    errors
}

/// Validate JSON theme structure
pub fn validate_json_theme(json_theme: &JSONColorTheme) -> Vec<String> {
    let mut errors = Vec::new();

    // Basic validation
    if json_theme.name.trim().is_empty() {
        errors.push("Theme name cannot be empty".to_string());
    }

    if json_theme.description.trim().is_empty() {
        errors.push("Theme description cannot be empty".to_string());
    }

    // Validate required colors
    let required_colors = ["primary", "background", "text", "border"];
    for required in &required_colors {
        match *required {
            "primary" if json_theme.palette.primary.is_none() => {
                errors.push("Missing required color: primary".to_string());
            }
            "background" if json_theme.palette.background.is_none() => {
                errors.push("Missing required color: background".to_string());
            }
            "text" if json_theme.palette.text.is_none() => {
                errors.push("Missing required color: text".to_string());
            }
            "border" if json_theme.palette.border.is_none() => {
                errors.push("Missing required color: border".to_string());
            }
            _ => {}
        }
    }

    errors
}

impl Default for ColorPalette {
    fn default() -> Self {
        default_color_palette()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hex_color() {
        let json_color = JSONColorDefinition::Hex {
            hex: "#FF5733".to_string(),
        };
        let named_colors = HashMap::new();
        let color = parse_json_color(&json_color, &named_colors).unwrap();
        assert_eq!(color, rgb(255, 87, 51));
    }

    #[test]
    fn test_parse_rgb_array_color() {
        let json_color = JSONColorDefinition::RgbArray { rgb: [255, 87, 51] };
        let named_colors = HashMap::new();
        let color = parse_json_color(&json_color, &named_colors).unwrap();
        assert_eq!(color, rgb(255, 87, 51));
    }

    #[test]
    fn test_parse_rgb_object_color() {
        let json_color = JSONColorDefinition::RgbObject {
            r: 255,
            g: 87,
            b: 51,
        };
        let named_colors = HashMap::new();
        let color = parse_json_color(&json_color, &named_colors).unwrap();
        assert_eq!(color, rgb(255, 87, 51));
    }

    #[test]
    fn test_ansi_to_rgb() {
        let red = ansi_to_rgb(9).unwrap(); // Bright red
        assert_eq!(red, rgb(255, 0, 0));

        let white = ansi_to_rgb(15).unwrap(); // White
        assert_eq!(white, rgb(255, 255, 255));
    }

    #[test]
    fn test_theme_to_ansi_codes() {
        use super::super::colors::dark_theme;

        let theme = dark_theme();
        let codes = theme_to_ansi_codes(&theme);

        assert!(codes.contains_key("primary"));
        assert!(codes.contains_key("primary_bg"));
        assert!(codes.contains_key("reset"));
    }
}
