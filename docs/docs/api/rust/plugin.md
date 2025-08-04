# Plugin Module

Extensible plugin architecture enabling custom widgets, themes, and functionality through dynamic loading and hot reloading support.

## Plugin System

Core plugin management system for loading, registering, and managing plugins.

```rust
use reactive_tui::plugin::{PluginManager, Plugin, PluginConfig};

let mut manager = PluginManager::new();

// Load plugins from directory
manager.load_plugins_from_directory("./plugins")?;

// Load individual plugin
let plugin_config = PluginConfig::from_file("custom_plugin.toml")?;
manager.load_plugin(plugin_config)?;

// Enable/disable plugins
manager.enable_plugin("custom_widgets")?;
manager.disable_plugin("experimental_features")?;
```

## Plugin Interface

### Plugin Trait

```rust
use reactive_tui::plugin::{Plugin, PluginContext, PluginResult};

pub trait Plugin: Send + Sync {
    /// Plugin metadata
    fn metadata(&self) -> &PluginMetadata;
    
    /// Initialize plugin
    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult<()>;
    
    /// Cleanup plugin resources
    fn cleanup(&mut self) -> PluginResult<()>;
    
    /// Handle plugin events
    fn handle_event(&mut self, event: &PluginEvent) -> PluginResult<()> {
        Ok(())
    }
    
    /// Plugin configuration changed
    fn on_config_change(&mut self, config: &PluginConfig) -> PluginResult<()> {
        Ok(())
    }
}
```

### Plugin Metadata

```rust
pub struct PluginMetadata {
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub license: String,
    pub homepage: Option<String>,
    pub repository: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub capabilities: Vec<PluginCapability>,
}

pub struct PluginDependency {
    pub name: String,
    pub version_requirement: String,
    pub optional: bool,
}

pub enum PluginCapability {
    Widget,
    Theme,
    Layout,
    Renderer,
    InputHandler,
    StateManager,
    Custom(String),
}
```

## Widget Plugins

### Custom Widget Development

```rust
use reactive_tui::plugin::{WidgetPlugin, WidgetFactory, WidgetConfig};
use reactive_tui::widgets::ResponsiveWidget;

pub struct CustomButtonPlugin;

impl Plugin for CustomButtonPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &PluginMetadata {
            name: "custom_button".to_string(),
            version: "1.0.0".to_string(),
            author: "Your Name".to_string(),
            description: "Custom button widget with enhanced features".to_string(),
            license: "MIT".to_string(),
            capabilities: vec![PluginCapability::Widget],
            ..Default::default()
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        // Register widget factory
        let factory = Box::new(CustomButtonFactory);
        context.register_widget_factory("custom_button", factory)?;
        Ok(())
    }
}

pub struct CustomButtonFactory;

impl WidgetFactory for CustomButtonFactory {
    fn create_widget(&self, config: &WidgetConfig) -> PluginResult<Box<dyn ResponsiveWidget>> {
        let button = CustomButton::new(config)?;
        Ok(Box::new(button))
    }
    
    fn supports_config(&self, config: &WidgetConfig) -> bool {
        config.widget_type == "custom_button"
    }
}

pub struct CustomButton {
    id: String,
    text: String,
    style: ButtonStyle,
    // custom fields
}

impl ResponsiveWidget for CustomButton {
    fn to_element(&self) -> Element {
        Element::with_tag("custom-button")
            .id(&self.id)
            .class("custom-btn")
            .text(&self.text)
            .build()
    }
    
    fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
        // Custom rendering logic
        format!("Custom Button: {}", self.text)
    }
    
    fn min_size(&self) -> (u16, u16) {
        (self.text.len() as u16 + 4, 3)
    }
}
```

### Widget Registration

```rust
use reactive_tui::plugin::{PluginRegistry, WidgetRegistry};

// Register widget plugin
let plugin = Box::new(CustomButtonPlugin);
manager.register_plugin(plugin)?;

// Use custom widget
let config = WidgetConfig::builder()
    .widget_type("custom_button")
    .property("text", "Click Me")
    .property("style", "primary")
    .build();

let widget = manager.create_widget(&config)?;
```

## Theme Plugins

### Custom Theme Provider

```rust
use reactive_tui::plugin::{ThemePlugin, ThemeProvider};
use reactive_tui::themes::ColorTheme;

pub struct MaterialThemePlugin;

impl Plugin for MaterialThemePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &PluginMetadata {
            name: "material_themes".to_string(),
            version: "1.0.0".to_string(),
            description: "Material Design themes".to_string(),
            capabilities: vec![PluginCapability::Theme],
            ..Default::default()
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        let provider = Box::new(MaterialThemeProvider);
        context.register_theme_provider("material", provider)?;
        Ok(())
    }
}

pub struct MaterialThemeProvider;

impl ThemeProvider for MaterialThemeProvider {
    fn get_theme(&self, name: &str) -> Option<ColorTheme> {
        match name {
            "material_dark" => Some(create_material_dark_theme()),
            "material_light" => Some(create_material_light_theme()),
            "material_blue" => Some(create_material_blue_theme()),
            _ => None,
        }
    }
    
    fn list_themes(&self) -> Vec<String> {
        vec![
            "material_dark".to_string(),
            "material_light".to_string(),
            "material_blue".to_string(),
        ]
    }
}

fn create_material_dark_theme() -> ColorTheme {
    ColorTheme::builder()
        .primary("#2196F3")
        .secondary("#03DAC6")
        .background("#121212")
        .surface("#1E1E1E")
        .error("#CF6679")
        .build()
}
```

## Layout Plugins

### Custom Layout Engine

```rust
use reactive_tui::plugin::{LayoutPlugin, LayoutEngine};
use reactive_tui::layout::{LayoutRect, ComputedStyles};

pub struct MasonryLayoutPlugin;

impl Plugin for MasonryLayoutPlugin {
    fn metadata(&self) -> &PluginMetadata {
        &PluginMetadata {
            name: "masonry_layout".to_string(),
            capabilities: vec![PluginCapability::Layout],
            ..Default::default()
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        let engine = Box::new(MasonryLayoutEngine);
        context.register_layout_engine("masonry", engine)?;
        Ok(())
    }
}

pub struct MasonryLayoutEngine;

impl LayoutEngine for MasonryLayoutEngine {
    fn compute_layout(&self, element: &Element, container: &LayoutRect) -> Vec<LayoutRect> {
        // Implement masonry layout algorithm
        let mut layouts = Vec::new();
        let mut columns = vec![0u16; 3]; // 3 columns
        
        for (i, child) in element.children().enumerate() {
            let min_column = columns.iter().enumerate()
                .min_by_key(|(_, &height)| height)
                .map(|(idx, _)| idx)
                .unwrap_or(0);
            
            let x = (container.width / 3) * min_column as u16;
            let y = columns[min_column];
            let width = container.width / 3;
            let height = calculate_child_height(child);
            
            layouts.push(LayoutRect::new(x, y, width, height));
            columns[min_column] += height;
        }
        
        layouts
    }
}
```

## Plugin Configuration

### Configuration Format

```toml
[plugin]
name = "custom_widgets"
version = "1.0.0"
author = "Your Name"
description = "Custom widget collection"
entry_point = "libcustom_widgets.so"

[dependencies.reactive_tui]
version = ">=0.1.0"

[capabilities]
widgets = ["custom_button", "custom_input", "custom_table"]
themes = []
layouts = []

[config]
theme_integration = true
hot_reload = true
debug_mode = false

[config.custom_button]
default_style = "primary"
animation_duration = 200
```

### Loading Configuration

```rust
use reactive_tui::plugin::{PluginConfig, ConfigLoader};

// Load from file
let config = PluginConfig::from_file("plugin.toml")?;

// Load from string
let toml_content = std::fs::read_to_string("plugin.toml")?;
let config = PluginConfig::from_str(&toml_content)?;

// Validate configuration
config.validate()?;

// Load plugin with config
manager.load_plugin_with_config(config)?;
```

## Hot Reloading

### Development Support

```rust
use reactive_tui::plugin::{HotReloadManager, FileWatcher};

let mut hot_reload = HotReloadManager::new();

// Watch plugin directory for changes
hot_reload.watch_directory("./plugins", |event| {
    match event {
        FileEvent::Modified(path) => {
            println!("Plugin modified: {:?}", path);
            manager.reload_plugin_from_path(path)?;
        },
        FileEvent::Created(path) => {
            println!("New plugin detected: {:?}", path);
            manager.load_plugin_from_path(path)?;
        },
        FileEvent::Deleted(path) => {
            println!("Plugin removed: {:?}", path);
            manager.unload_plugin_from_path(path)?;
        },
    }
})?;

// Enable hot reloading for specific plugin
manager.enable_hot_reload("custom_widgets")?;
```

### Reload Safety

```rust
use reactive_tui::plugin::ReloadSafety;

impl Plugin for MyPlugin {
    fn before_reload(&mut self) -> PluginResult<ReloadSafety> {
        // Save state before reload
        self.save_state()?;
        
        // Check if safe to reload
        if self.has_active_connections() {
            Ok(ReloadSafety::Unsafe("Active connections present".to_string()))
        } else {
            Ok(ReloadSafety::Safe)
        }
    }
    
    fn after_reload(&mut self) -> PluginResult<()> {
        // Restore state after reload
        self.restore_state()?;
        Ok(())
    }
}
```

## Plugin Communication

### Event System

```rust
use reactive_tui::plugin::{PluginEvent, EventBus};

// Plugin events
pub enum PluginEvent {
    WidgetCreated(String),
    ThemeChanged(String),
    ConfigUpdated(String),
    Custom(String, serde_json::Value),
}

// Subscribe to events
manager.subscribe_to_event(PluginEvent::ThemeChanged, |theme_name| {
    println!("Theme changed to: {}", theme_name);
    update_plugin_theme(theme_name);
});

// Emit events
manager.emit_event(PluginEvent::Custom(
    "user_action".to_string(),
    json!({"action": "button_clicked", "id": "submit_btn"})
));
```

### Inter-Plugin Communication

```rust
use reactive_tui::plugin::{PluginRegistry, PluginMessage};

// Send message to another plugin
let message = PluginMessage::new("data_provider", "fetch_user_data")
    .with_payload(json!({"user_id": 123}));

manager.send_message(message)?;

// Handle incoming messages
impl Plugin for MyPlugin {
    fn handle_message(&mut self, message: &PluginMessage) -> PluginResult<Option<serde_json::Value>> {
        match message.action.as_str() {
            "fetch_user_data" => {
                let user_id = message.payload["user_id"].as_u64().unwrap();
                let user_data = fetch_user(user_id)?;
                Ok(Some(json!(user_data)))
            },
            _ => Ok(None),
        }
    }
}
```

## Plugin Security

### Sandboxing

```rust
use reactive_tui::plugin::{PluginSandbox, SecurityPolicy};

let security_policy = SecurityPolicy::builder()
    .allow_file_system(false)
    .allow_network(false)
    .allow_process_spawn(false)
    .allowed_directories(vec!["/tmp/plugin_data".to_string()])
    .build();

let sandbox = PluginSandbox::new(security_policy);
manager.set_sandbox(sandbox);
```

### Permission System

```rust
use reactive_tui::plugin::{Permission, PermissionManager};

// Define permissions
let permissions = vec![
    Permission::ReadConfig,
    Permission::WriteConfig,
    Permission::AccessNetwork,
    Permission::CreateWidgets,
    Permission::ModifyThemes,
];

// Check permissions
if manager.has_permission("custom_widgets", Permission::CreateWidgets) {
    // Allow widget creation
    create_custom_widget()?;
} else {
    return Err(PluginError::PermissionDenied);
}
```

## Plugin Testing

### Test Framework

```rust
use reactive_tui::plugin::testing::{PluginTestRunner, MockPluginContext};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_plugin_initialization() {
        let mut plugin = CustomButtonPlugin;
        let mut context = MockPluginContext::new();
        
        // Test initialization
        assert!(plugin.initialize(&mut context).is_ok());
        
        // Verify widget was registered
        assert!(context.has_widget_factory("custom_button"));
    }
    
    #[test]
    fn test_widget_creation() {
        let factory = CustomButtonFactory;
        let config = WidgetConfig::builder()
            .widget_type("custom_button")
            .property("text", "Test")
            .build();
        
        let widget = factory.create_widget(&config).unwrap();
        assert_eq!(widget.min_size(), (8, 3)); // "Test" + padding
    }
}
```

## Example Integration

### Complete Plugin Example

```rust
use reactive_tui::plugin::*;
use reactive_tui::widgets::*;
use reactive_tui::themes::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize plugin manager
    let mut manager = PluginManager::new();
    
    // Load plugins
    manager.load_plugins_from_directory("./plugins")?;
    
    // Create application with plugin support
    let mut app = TuiApp::builder()
        .plugin_manager(manager)
        .build();
    
    // Use plugin-provided widgets
    let custom_widget = app.create_widget(&WidgetConfig::builder()
        .widget_type("custom_button")
        .property("text", "Plugin Button")
        .build())?;
    
    // Apply plugin-provided theme
    app.set_theme("material_dark")?;
    
    // Run application
    app.run()
}

// Plugin implementation
#[no_mangle]
pub extern "C" fn create_plugin() -> *mut dyn Plugin {
    Box::into_raw(Box::new(MyAwesomePlugin::new()))
}

pub struct MyAwesomePlugin {
    initialized: bool,
}

impl MyAwesomePlugin {
    pub fn new() -> Self {
        Self {
            initialized: false,
        }
    }
}

impl Plugin for MyAwesomePlugin {
    fn metadata(&self) -> &PluginMetadata {
        &PluginMetadata {
            name: "awesome_plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "Plugin Developer".to_string(),
            description: "An awesome plugin with custom widgets".to_string(),
            capabilities: vec![
                PluginCapability::Widget,
                PluginCapability::Theme,
            ],
            ..Default::default()
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> PluginResult<()> {
        // Register widgets
        context.register_widget_factory("awesome_button", Box::new(AwesomeButtonFactory))?;
        context.register_widget_factory("awesome_input", Box::new(AwesomeInputFactory))?;
        
        // Register themes
        context.register_theme_provider("awesome", Box::new(AwesomeThemeProvider))?;
        
        self.initialized = true;
        Ok(())
    }
    
    fn cleanup(&mut self) -> PluginResult<()> {
        self.initialized = false;
        Ok(())
    }
}
```