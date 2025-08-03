/*!
 * Theme System with JSON Configuration
 * 
 * A comprehensive theming system that allows complete customization of the TUI
 * appearance through JSON configuration files. Supports runtime theme switching,
 * inheritance, and dynamic color schemes.
 * 
 * Features:
 * - JSON-based theme definitions
 * - Theme inheritance and composition
 * - Runtime theme switching
 * - Color scheme variants (light/dark/high-contrast)
 * - Component-specific styling
 * - Theme validation and hot reload
 * - Export/import theme packages
 * 
 * Example:
 * ```rust
 * use reactive_tui::themes::theme_system::*;
 * 
 * let theme_manager = ThemeManager::new();
 * theme_manager.load_theme_file("themes/dark.json")?;
 * theme_manager.set_active_theme("dark")?;
 * ```
 */

use crate::{
    error::{Result, TuiError},
    themes::{ColorDefinition, ColorPalette, ColorTheme, SemanticColorMapping},
};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    sync::{Arc, RwLock},
};

/// Theme definition structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeDefinition {
    /// Theme metadata
    pub meta: ThemeMetadata,
    /// Color palette
    pub colors: ThemeColors,
    /// Typography settings
    pub typography: Typography,
    /// Component styles
    pub components: HashMap<String, ComponentStyle>,
    /// Layout settings
    pub layout: LayoutTheme,
    /// Animation settings
    pub animations: AnimationTheme,
}

/// Theme metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeMetadata {
    /// Theme name
    pub name: String,
    /// Theme version
    pub version: String,
    /// Theme author
    pub author: String,
    /// Theme description
    pub description: String,
    /// Parent theme to inherit from
    pub extends: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
}

/// Theme color definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColors {
    /// Primary color palette
    pub primary: ColorScale,
    /// Secondary color palette
    pub secondary: ColorScale,
    /// Accent color palette
    pub accent: ColorScale,
    /// Neutral color palette
    pub neutral: ColorScale,
    /// Semantic colors
    pub semantic: SemanticColors,
    /// Surface colors
    pub surfaces: SurfaceColors,
}

/// Color scale with shades
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorScale {
    pub lightest: String,
    pub lighter: String,
    pub light: String,
    pub base: String,
    pub dark: String,
    pub darker: String,
    pub darkest: String,
}

/// Semantic color definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticColors {
    pub success: String,
    pub warning: String,
    pub error: String,
    pub info: String,
}

/// Surface color definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SurfaceColors {
    pub background: String,
    pub foreground: String,
    pub border: String,
    pub shadow: String,
    pub overlay: String,
}

/// Typography settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Typography {
    /// Font families
    pub fonts: FontFamilies,
    /// Font sizes
    pub sizes: FontSizes,
    /// Font weights
    pub weights: FontWeights,
    /// Line heights
    pub line_heights: LineHeights,
}

/// Font family definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontFamilies {
    pub mono: String,
    pub sans: String,
    pub serif: String,
}

/// Font size scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontSizes {
    pub xs: f32,
    pub sm: f32,
    pub base: f32,
    pub lg: f32,
    pub xl: f32,
    pub xxl: f32,
}

/// Font weight scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FontWeights {
    pub light: u16,
    pub regular: u16,
    pub medium: u16,
    pub bold: u16,
    pub black: u16,
}

/// Line height scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineHeights {
    pub tight: f32,
    pub normal: f32,
    pub relaxed: f32,
    pub loose: f32,
}

/// Component-specific styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentStyle {
    /// Base styles
    pub base: HashMap<String, serde_json::Value>,
    /// State variants
    pub states: HashMap<String, HashMap<String, serde_json::Value>>,
    /// Size variants
    pub sizes: HashMap<String, HashMap<String, serde_json::Value>>,
    /// Color variants
    pub variants: HashMap<String, HashMap<String, serde_json::Value>>,
}

/// Layout theme settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LayoutTheme {
    /// Spacing scale
    pub spacing: SpacingScale,
    /// Border radius scale
    pub radius: RadiusScale,
    /// Z-index layers
    pub z_index: ZIndexLayers,
}

/// Spacing scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingScale {
    pub none: u16,
    pub xs: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
    pub xxl: u16,
}

/// Border radius scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadiusScale {
    pub none: u16,
    pub sm: u16,
    pub md: u16,
    pub lg: u16,
    pub xl: u16,
    pub full: u16,
}

/// Z-index layer definitions
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZIndexLayers {
    pub base: i32,
    pub dropdown: i32,
    pub sticky: i32,
    pub fixed: i32,
    pub modal_backdrop: i32,
    pub modal: i32,
    pub popover: i32,
    pub tooltip: i32,
}

/// Animation theme settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnimationTheme {
    /// Duration scale
    pub durations: DurationScale,
    /// Easing functions
    pub easings: HashMap<String, String>,
}

/// Animation duration scale
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DurationScale {
    pub instant: u16,
    pub fast: u16,
    pub normal: u16,
    pub slow: u16,
    pub slower: u16,
}

/// Theme manager for handling multiple themes
pub struct ThemeManager {
    /// Loaded themes
    themes: Arc<RwLock<HashMap<String, ThemeDefinition>>>,
    /// Active theme name
    active_theme: Arc<RwLock<Option<String>>>,
    /// Theme directory
    theme_dir: PathBuf,
    /// Theme cache
    cache: Arc<RwLock<ThemeCache>>,
}

/// Theme cache for performance
struct ThemeCache {
    /// Compiled color themes
    color_themes: HashMap<String, ColorTheme>,
    /// Resolved component styles
    component_styles: HashMap<String, HashMap<String, String>>,
}

impl ThemeManager {
    /// Create a new theme manager
    pub fn new() -> Self {
        Self::with_directory("themes")
    }
    
    /// Create theme manager with custom directory
    pub fn with_directory<P: AsRef<Path>>(dir: P) -> Self {
        Self {
            themes: Arc::new(RwLock::new(HashMap::new())),
            active_theme: Arc::new(RwLock::new(None)),
            theme_dir: dir.as_ref().to_path_buf(),
            cache: Arc::new(RwLock::new(ThemeCache {
                color_themes: HashMap::new(),
                component_styles: HashMap::new(),
            })),
        }
    }
    
    /// Load a theme from file
    pub fn load_theme_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let content = fs::read_to_string(path.as_ref())
            .map_err(|e| TuiError::io(format!("Failed to read theme file: {}", e)))?;
        
        let theme: ThemeDefinition = serde_json::from_str(&content)
            .map_err(|e| TuiError::theme(format!("Failed to parse theme JSON: {}", e)))?;
        
        self.validate_theme(&theme)?;
        
        let theme_name = theme.meta.name.clone();
        self.themes.write().unwrap().insert(theme_name.clone(), theme);
        
        // Clear cache for this theme
        self.invalidate_cache(&theme_name);
        
        Ok(())
    }
    
    /// Load all themes from directory
    pub fn load_theme_directory(&self) -> Result<()> {
        if !self.theme_dir.exists() {
            return Ok(());
        }
        
        for entry in fs::read_dir(&self.theme_dir)? {
            let entry = entry?;
            let path = entry.path();
            
            if path.extension().and_then(|s| s.to_str()) == Some("json") {
                if let Err(e) = self.load_theme_file(&path) {
                    eprintln!("Failed to load theme {:?}: {}", path, e);
                }
            }
        }
        
        Ok(())
    }
    
    /// Set active theme
    pub fn set_active_theme(&self, name: &str) -> Result<()> {
        let themes = self.themes.read().unwrap();
        if !themes.contains_key(name) {
            return Err(TuiError::theme(format!("Theme '{}' not found", name)));
        }
        
        *self.active_theme.write().unwrap() = Some(name.to_string());
        Ok(())
    }
    
    /// Get active theme
    pub fn get_active_theme(&self) -> Option<ThemeDefinition> {
        let active = self.active_theme.read().unwrap();
        if let Some(name) = active.as_ref() {
            self.themes.read().unwrap().get(name).cloned()
        } else {
            None
        }
    }
    
    /// Get theme by name
    pub fn get_theme(&self, name: &str) -> Option<ThemeDefinition> {
        self.themes.read().unwrap().get(name).cloned()
    }
    
    /// List available themes
    pub fn list_themes(&self) -> Vec<String> {
        self.themes.read().unwrap().keys().cloned().collect()
    }
    
    /// Export theme to file
    pub fn export_theme<P: AsRef<Path>>(&self, name: &str, path: P) -> Result<()> {
        let theme = self.get_theme(name)
            .ok_or_else(|| TuiError::theme(format!("Theme '{}' not found", name)))?;
        
        let json = serde_json::to_string_pretty(&theme)
            .map_err(|e| TuiError::theme(format!("Failed to serialize theme: {}", e)))?;
        
        fs::write(path, json)
            .map_err(|e| TuiError::io(format!("Failed to write theme file: {}", e)))?;
        
        Ok(())
    }
    
    /// Create a new theme from scratch
    pub fn create_theme(&self, name: &str) -> ThemeBuilder {
        ThemeBuilder::new(name)
    }
    
    /// Validate theme definition
    fn validate_theme(&self, theme: &ThemeDefinition) -> Result<()> {
        // Validate color formats
        self.validate_color(&theme.colors.primary.base)?;
        self.validate_color(&theme.colors.secondary.base)?;
        self.validate_color(&theme.colors.accent.base)?;
        self.validate_color(&theme.colors.neutral.base)?;
        
        // Validate inheritance
        if let Some(parent) = &theme.meta.extends {
            if !self.themes.read().unwrap().contains_key(parent) {
                return Err(TuiError::theme(format!(
                    "Parent theme '{}' not found",
                    parent
                )));
            }
        }
        
        Ok(())
    }
    
    /// Validate color format
    fn validate_color(&self, color: &str) -> Result<()> {
        if !color.starts_with('#') || (color.len() != 7 && color.len() != 9) {
            return Err(TuiError::theme(format!("Invalid color format: {}", color)));
        }
        Ok(())
    }
    
    /// Invalidate cache for theme
    fn invalidate_cache(&self, theme_name: &str) {
        let mut cache = self.cache.write().unwrap();
        cache.color_themes.remove(theme_name);
        cache.component_styles.remove(theme_name);
    }
    
    /// Resolve theme with inheritance
    pub fn resolve_theme(&self, name: &str) -> Result<ThemeDefinition> {
        let themes = self.themes.read().unwrap();
        let base_theme = themes.get(name)
            .ok_or_else(|| TuiError::theme(format!("Theme '{}' not found", name)))?;
        
        if let Some(parent_name) = &base_theme.meta.extends {
            let parent_theme = self.resolve_theme(parent_name)?;
            Ok(self.merge_themes(parent_theme, base_theme.clone()))
        } else {
            Ok(base_theme.clone())
        }
    }
    
    /// Merge two themes (child overrides parent)
    fn merge_themes(&self, parent: ThemeDefinition, mut child: ThemeDefinition) -> ThemeDefinition {
        // Deep merge - child values override parent values
        
        // Merge colors - child colors override parent colors
        for (key, value) in parent.colors.primary {
            child.colors.primary.entry(key).or_insert(value);
        }
        for (key, value) in parent.colors.semantic {
            child.colors.semantic.entry(key).or_insert(value);
        }
        for (key, value) in parent.colors.syntax {
            child.colors.syntax.entry(key).or_insert(value);
        }
        
        // Merge typography - use parent values if child doesn't specify
        if child.typography.font_family.is_empty() && !parent.typography.font_family.is_empty() {
            child.typography.font_family = parent.typography.font_family;
        }
        if child.typography.base_size == 0 && parent.typography.base_size > 0 {
            child.typography.base_size = parent.typography.base_size;
        }
        
        // Merge component styles - child components override parent
        for (component, style) in parent.components {
            child.components.entry(component).or_insert(style);
        }
        
        // Merge layout settings - use parent values if child doesn't specify
        if child.layout.spacing.base_unit == 0 && parent.layout.spacing.base_unit > 0 {
            child.layout.spacing.base_unit = parent.layout.spacing.base_unit;
        }
        
        // Merge animation settings - use parent values if child doesn't specify
        if child.animations.duration.fast == 0 && parent.animations.duration.fast > 0 {
            child.animations.duration = parent.animations.duration;
        }
        if child.animations.easing.standard.is_empty() && !parent.animations.easing.standard.is_empty() {
            child.animations.easing = parent.animations.easing;
        }
        
        child
    }
}

/// Theme builder for creating themes programmatically
pub struct ThemeBuilder {
    theme: ThemeDefinition,
}

impl ThemeBuilder {
    /// Create a new theme builder
    pub fn new(name: &str) -> Self {
        Self {
            theme: ThemeDefinition {
                meta: ThemeMetadata {
                    name: name.to_string(),
                    version: "1.0.0".to_string(),
                    author: String::new(),
                    description: String::new(),
                    extends: None,
                    tags: Vec::new(),
                },
                colors: default_colors(),
                typography: default_typography(),
                components: HashMap::new(),
                layout: default_layout(),
                animations: default_animations(),
            },
        }
    }
    
    /// Set theme metadata
    pub fn metadata(mut self, meta: ThemeMetadata) -> Self {
        self.theme.meta = meta;
        self
    }
    
    /// Set color scheme
    pub fn colors(mut self, colors: ThemeColors) -> Self {
        self.theme.colors = colors;
        self
    }
    
    /// Set typography
    pub fn typography(mut self, typography: Typography) -> Self {
        self.theme.typography = typography;
        self
    }
    
    /// Add component style
    pub fn component(mut self, name: &str, style: ComponentStyle) -> Self {
        self.theme.components.insert(name.to_string(), style);
        self
    }
    
    /// Set layout theme
    pub fn layout(mut self, layout: LayoutTheme) -> Self {
        self.theme.layout = layout;
        self
    }
    
    /// Set animation theme
    pub fn animations(mut self, animations: AnimationTheme) -> Self {
        self.theme.animations = animations;
        self
    }
    
    /// Build the theme
    pub fn build(self) -> ThemeDefinition {
        self.theme
    }
}

/// Default color scheme
fn default_colors() -> ThemeColors {
    ThemeColors {
        primary: ColorScale {
            lightest: "#e3f2fd".to_string(),
            lighter: "#bbdefb".to_string(),
            light: "#90caf9".to_string(),
            base: "#2196f3".to_string(),
            dark: "#1976d2".to_string(),
            darker: "#1565c0".to_string(),
            darkest: "#0d47a1".to_string(),
        },
        secondary: ColorScale {
            lightest: "#f3e5f5".to_string(),
            lighter: "#e1bee7".to_string(),
            light: "#ce93d8".to_string(),
            base: "#9c27b0".to_string(),
            dark: "#7b1fa2".to_string(),
            darker: "#6a1b9a".to_string(),
            darkest: "#4a148c".to_string(),
        },
        accent: ColorScale {
            lightest: "#fff3e0".to_string(),
            lighter: "#ffe0b2".to_string(),
            light: "#ffcc80".to_string(),
            base: "#ff9800".to_string(),
            dark: "#f57c00".to_string(),
            darker: "#ef6c00".to_string(),
            darkest: "#e65100".to_string(),
        },
        neutral: ColorScale {
            lightest: "#fafafa".to_string(),
            lighter: "#f5f5f5".to_string(),
            light: "#e0e0e0".to_string(),
            base: "#9e9e9e".to_string(),
            dark: "#616161".to_string(),
            darker: "#424242".to_string(),
            darkest: "#212121".to_string(),
        },
        semantic: SemanticColors {
            success: "#4caf50".to_string(),
            warning: "#ff9800".to_string(),
            error: "#f44336".to_string(),
            info: "#2196f3".to_string(),
        },
        surfaces: SurfaceColors {
            background: "#ffffff".to_string(),
            foreground: "#000000".to_string(),
            border: "#e0e0e0".to_string(),
            shadow: "rgba(0,0,0,0.1)".to_string(),
            overlay: "rgba(0,0,0,0.5)".to_string(),
        },
    }
}

/// Default typography
fn default_typography() -> Typography {
    Typography {
        fonts: FontFamilies {
            mono: "monospace".to_string(),
            sans: "sans-serif".to_string(),
            serif: "serif".to_string(),
        },
        sizes: FontSizes {
            xs: 0.75,
            sm: 0.875,
            base: 1.0,
            lg: 1.125,
            xl: 1.25,
            xxl: 1.5,
        },
        weights: FontWeights {
            light: 300,
            regular: 400,
            medium: 500,
            bold: 700,
            black: 900,
        },
        line_heights: LineHeights {
            tight: 1.25,
            normal: 1.5,
            relaxed: 1.75,
            loose: 2.0,
        },
    }
}

/// Default layout theme
fn default_layout() -> LayoutTheme {
    LayoutTheme {
        spacing: SpacingScale {
            none: 0,
            xs: 2,
            sm: 4,
            md: 8,
            lg: 16,
            xl: 24,
            xxl: 32,
        },
        radius: RadiusScale {
            none: 0,
            sm: 2,
            md: 4,
            lg: 8,
            xl: 16,
            full: 9999,
        },
        z_index: ZIndexLayers {
            base: 0,
            dropdown: 1000,
            sticky: 1020,
            fixed: 1030,
            modal_backdrop: 1040,
            modal: 1050,
            popover: 1060,
            tooltip: 1070,
        },
    }
}

/// Default animations
fn default_animations() -> AnimationTheme {
    AnimationTheme {
        durations: DurationScale {
            instant: 0,
            fast: 150,
            normal: 300,
            slow: 500,
            slower: 1000,
        },
        easings: HashMap::from([
            ("linear".to_string(), "linear".to_string()),
            ("ease".to_string(), "ease".to_string()),
            ("ease-in".to_string(), "ease-in".to_string()),
            ("ease-out".to_string(), "ease-out".to_string()),
            ("ease-in-out".to_string(), "ease-in-out".to_string()),
        ]),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_theme_builder() {
        let theme = ThemeBuilder::new("test-theme")
            .metadata(ThemeMetadata {
                name: "test-theme".to_string(),
                version: "1.0.0".to_string(),
                author: "Test Author".to_string(),
                description: "Test theme".to_string(),
                extends: None,
                tags: vec!["test".to_string()],
            })
            .build();
        
        assert_eq!(theme.meta.name, "test-theme");
        assert_eq!(theme.meta.author, "Test Author");
    }
    
    #[test]
    fn test_theme_manager() {
        let manager = ThemeManager::new();
        let theme = ThemeBuilder::new("dark").build();
        
        // Add theme to manager
        manager.themes.write().unwrap().insert("dark".to_string(), theme);
        
        // Set active theme
        assert!(manager.set_active_theme("dark").is_ok());
        
        // Get active theme
        assert!(manager.get_active_theme().is_some());
        
        // List themes
        let themes = manager.list_themes();
        assert!(themes.contains(&"dark".to_string()));
    }
}