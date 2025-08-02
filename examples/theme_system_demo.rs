/*!
 * Theme System Demo
 * 
 * Demonstrates the JSON-based theme system with runtime switching,
 * inheritance, and component styling.
 */

use tui_core::prelude::*;
use tui_core::components::{div, text, Element};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use serde::{Serialize, Deserialize};

/// Simple theme definition
#[derive(Clone, Debug, Serialize, Deserialize)]
struct ThemeDefinition {
    name: String,
    version: String,
    author: String,
    description: String,
    colors: HashMap<String, String>,
    styles: HashMap<String, String>,
}

/// Theme manager for our demo
struct ThemeManager {
    themes: HashMap<String, ThemeDefinition>,
    current_theme: String,
}

impl ThemeManager {
    fn new() -> Self {
        let mut themes = HashMap::new();
        
        // Dark theme
        let dark_theme = ThemeDefinition {
            name: "Dark Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "TUI Framework".to_string(),
            description: "A modern dark theme with high contrast".to_string(),
            colors: [
                ("primary", "#3b82f6"),
                ("secondary", "#64748b"),
                ("accent", "#f59e0b"),
                ("background", "#0f172a"),
                ("surface", "#1e293b"),
                ("text", "#f8fafc"),
                ("text_secondary", "#94a3b8"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            styles: [
                ("border", "rounded"),
                ("shadow", "md"),
                ("opacity", "90"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        };
        
        // Light theme
        let light_theme = ThemeDefinition {
            name: "Light Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "TUI Framework".to_string(),
            description: "A clean light theme with subtle shadows".to_string(),
            colors: [
                ("primary", "#2563eb"),
                ("secondary", "#475569"),
                ("accent", "#d97706"),
                ("background", "#ffffff"),
                ("surface", "#f8fafc"),
                ("text", "#0f172a"),
                ("text_secondary", "#475569"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            styles: [
                ("border", "solid"),
                ("shadow", "lg"),
                ("opacity", "95"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        };
        
        // Custom theme
        let custom_theme = ThemeDefinition {
            name: "Custom Theme".to_string(),
            version: "1.0.0".to_string(),
            author: "Demo User".to_string(),
            description: "A custom theme created in the demo".to_string(),
            colors: [
                ("primary", "#8b5cf6"),
                ("secondary", "#6b7280"),
                ("accent", "#10b981"),
                ("background", "#1f2937"),
                ("surface", "#374151"),
                ("text", "#f9fafb"),
                ("text_secondary", "#9ca3af"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            styles: [
                ("border", "double"),
                ("shadow", "xl"),
                ("opacity", "85"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        };
        
        themes.insert("dark".to_string(), dark_theme);
        themes.insert("light".to_string(), light_theme);
        themes.insert("custom".to_string(), custom_theme);
        
        Self {
            themes,
            current_theme: "dark".to_string(),
        }
    }
    
    fn get_theme(&self, name: &str) -> Option<&ThemeDefinition> {
        self.themes.get(name)
    }
    
    fn list_themes(&self) -> Vec<String> {
        self.themes.keys().cloned().collect()
    }
    
    fn set_active_theme(&mut self, name: &str) -> Result<()> {
        if self.themes.contains_key(name) {
            self.current_theme = name.to_string();
            println!("🎨 Switched to theme: {name}");
            Ok(())
        } else {
            Err(TuiError::component(format!("Theme '{name}' not found")))
        }
    }
    
    fn get_current_theme(&self) -> &ThemeDefinition {
        self.themes.get(&self.current_theme).unwrap()
    }
    
    fn export_theme(&self, theme_name: &str, file_path: &str) -> Result<()> {
        if let Some(theme) = self.themes.get(theme_name) {
            let json = serde_json::to_string_pretty(theme)
                .map_err(|e| TuiError::component(format!("Failed to serialize theme: {e}")))?;
            
            std::fs::write(file_path, json)
                .map_err(|e| TuiError::component(format!("Failed to write file: {e}")))?;
            
            println!("✅ Exported theme '{theme_name}' to {file_path}");
            Ok(())
        } else {
            Err(TuiError::component(format!("Theme '{theme_name}' not found")))
        }
    }
    
    fn reload_themes(&mut self) -> Result<()> {
        println!("🔄 Reloading themes...");
        
        // Clear existing themes (except built-in ones)
        let builtin_keys = ["dark", "light", "custom"];
        self.themes.retain(|k, _| builtin_keys.contains(&k.as_str()));
        
        // Scan theme directory for JSON files
        let theme_dir = std::path::Path::new("themes");
        if theme_dir.exists() && theme_dir.is_dir() {
            match std::fs::read_dir(theme_dir) {
                Ok(entries) => {
                    for entry in entries.flatten() {
                        let path = entry.path();
                        if path.extension().and_then(|s| s.to_str()) == Some("json") {
                            // Try to load the theme file
                            match std::fs::read_to_string(&path) {
                                Ok(content) => {
                                    match serde_json::from_str::<ThemeDefinition>(&content) {
                                        Ok(theme) => {
                                            let theme_id = path.file_stem()
                                                .and_then(|s| s.to_str())
                                                .unwrap_or("unknown")
                                                .to_string();
                                            println!("  📄 Loaded theme: {} from {}", theme.name, path.display());
                                            self.themes.insert(theme_id, theme);
                                        }
                                        Err(e) => {
                                            eprintln!("  ⚠️  Failed to parse {}: {}", path.display(), e);
                                        }
                                    }
                                }
                                Err(e) => {
                                    eprintln!("  ⚠️  Failed to read {}: {}", path.display(), e);
                                }
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("  ⚠️  Failed to read theme directory: {}", e);
                }
            }
        } else {
            println!("  📁 Theme directory 'themes/' not found, using built-in themes only");
        }
        
        println!("✅ {} themes loaded", self.themes.len());
        Ok(())
    }
}

/// Create a color preview component
fn create_color_preview(name: &str, color: &str) -> Element {
    div()
        .class("color-preview")
        .child(text(format!("{name}: {color}")).build())
        .build()
}

/// Create a theme preview panel
fn create_theme_preview(theme: &ThemeDefinition) -> Element {
    div()
        .class("theme-preview")
        .child(text(format!("Theme: {}", theme.name)).build())
        .child(text("").build())
        .child(text(format!("Version: {}", theme.version)).build())
        .child(text(format!("Author: {}", theme.author)).build())
        .child(text(format!("Description: {}", theme.description)).build())
        .child(text("").build())
        .child(text("Colors:").build())
        .children(
            theme.colors.iter().map(|(name, color)| {
                create_color_preview(name, color)
            }).collect::<Vec<_>>()
        )
        .child(text("").build())
        .child(text("Styles:").build())
        .children(
            theme.styles.iter().map(|(name, style)| {
                div()
                    .child(text(format!("{name}: {style}")).build())
                    .build()
            }).collect::<Vec<_>>()
        )
        .build()
}

/// Create theme selector
fn create_theme_selector(themes: &[String], current: &str) -> Element {
    div()
        .class("theme-selector")
        .child(text("📦 Available Themes").build())
        .child(text("").build())
        .children(
            themes.iter().map(|theme_name| {
                let marker = if theme_name == current { "► " } else { "  " };
                div()
                    .class("theme-item")
                    .child(text(format!("{marker}{theme_name}")).build())
                    .build()
            }).collect::<Vec<_>>()
        )
        .build()
}

/// Main demo function
fn main() -> Result<()> {
    println!("🎨 Theme System Demo");
    println!("===================\n");
    
    // Create theme manager
    let theme_manager = Arc::new(RwLock::new(ThemeManager::new()));
    
    // Theme list for UI state
    let theme_list = {
        let manager = theme_manager.read().unwrap();
        manager.list_themes()
    };
    
    println!("📂 Loaded themes:");
    for theme_name in &theme_list {
        let manager = theme_manager.read().unwrap();
        if let Some(theme) = manager.get_theme(theme_name) {
            println!("   - {} v{} by {}", theme.name, theme.version, theme.author);
        }
    }
    println!();
    
    // Demo theme switching
    println!("🔄 Theme Switching Demo:");
    println!("========================");
    
    for theme_name in &theme_list {
        println!("\n🎨 Switching to theme: {theme_name}");
        
        {
            let mut manager = theme_manager.write().unwrap();
            manager.set_active_theme(theme_name)?;
        }
        
        // Create preview for current theme
        let manager = theme_manager.read().unwrap();
        let current_theme = manager.get_current_theme();
        
        println!("📋 Theme Details:");
        println!("   Name: {}", current_theme.name);
        println!("   Description: {}", current_theme.description);
        println!("   Colors: {} defined", current_theme.colors.len());
        println!("   Styles: {} defined", current_theme.styles.len());
        
        // Create UI components
        let theme_selector = create_theme_selector(&theme_list, theme_name);
        let theme_preview = create_theme_preview(current_theme);
        
        println!("   Theme Selector: {theme_selector:?}");
        println!("   Theme Preview: {theme_preview:?}");
    }
    
    println!("\n🛠️  Theme Management Demo:");
    println!("===========================");
    
    // Export current theme
    {
        let manager = theme_manager.read().unwrap();
        let current = &manager.current_theme;
        let export_path = format!("{current}-export.json");
        manager.export_theme(current, &export_path)?;
    }
    
    // Reload themes
    {
        let mut manager = theme_manager.write().unwrap();
        manager.reload_themes()?;
    }
    
    println!("\n📊 Theme Statistics:");
    println!("====================");
    
    {
        let manager = theme_manager.read().unwrap();
        println!("Total themes: {}", manager.themes.len());
        println!("Current theme: {}", manager.current_theme);
        
        // Color analysis
        let mut total_colors = 0;
        let mut total_styles = 0;
        
        for theme in manager.themes.values() {
            total_colors += theme.colors.len();
            total_styles += theme.styles.len();
        }
        
        println!("Total colors defined: {total_colors}");
        println!("Total styles defined: {total_styles}");
        println!("Average colors per theme: {:.1}", total_colors as f64 / manager.themes.len() as f64);
        println!("Average styles per theme: {:.1}", total_styles as f64 / manager.themes.len() as f64);
    }
    
    println!("\n🎮 Interactive Features:");
    println!("========================");
    println!("In a full implementation, you could:");
    println!("  [Tab] - Switch between themes");
    println!("  [E] - Export current theme");
    println!("  [R] - Reload themes from directory");
    println!("  [C] - Create new custom theme");
    println!("  [Q] - Quit application");
    
    println!("\n✨ Theme system demo completed successfully!");
    
    Ok(())
}