---
sidebar_position: 6
---

# Plugin System API

Extensible plugin architecture for creating custom widgets and extending framework functionality.

## Overview

The Reactive TUI plugin system provides a comprehensive framework for creating reusable, distributable components and extending the core functionality. The system supports:

- **Dynamic widget registration** - Register custom widgets at runtime
- **Plugin lifecycle management** - Initialize, configure, and cleanup plugins
- **Dependency resolution** - Automatic handling of plugin dependencies
- **Event hooks and interceptors** - React to framework events
- **Hot loading/unloading** - Load and unload plugins without restart
- **Plugin marketplace integration** - Share and discover plugins

## Core Traits

### Plugin Trait

All plugins must implement the `Plugin` trait:

```rust
use reactive_tui::plugin::*;
use std::any::Any;

pub trait Plugin: Send + Sync {
    /// Unique plugin identifier
    fn id(&self) -> &str;
    
    /// Plugin metadata and information
    fn metadata(&self) -> PluginMetadata;
    
    /// Initialize the plugin with context
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()>;
    
    /// Cleanup plugin resources
    fn cleanup(&mut self) -> Result<()>;
    
    /// Handle plugin events
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse>;
    
    /// Type erasure support
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

### WidgetPlugin Trait

For plugins that provide custom widgets:

```rust
pub trait WidgetPlugin: Plugin + Component {
    /// Widget type identifier
    fn widget_type(&self) -> &str;
    
    /// Create new widget instance
    fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin>;
    
    /// JSON schema for widget configuration
    fn config_schema(&self) -> serde_json::Value;
    
    /// Validate widget configuration
    fn validate_config(&self, config: &WidgetConfig) -> Result<()>;
}
```

## Plugin Metadata

Plugin information and capabilities:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub entry_point: String,
    pub homepage: Option<String>,
    pub dependencies: Vec<PluginDependency>,
    pub capabilities: Vec<PluginCapability>,
    pub tags: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
    pub id: String,
    pub version: String,
    pub optional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginCapability {
    WidgetProvider,     // Provides custom widgets
    EventHandler,       // Handles framework events
    ThemeProvider,      // Provides themes
    StyleProvider,      // Provides CSS utilities
    DataProvider,       // Provides data sources
    ServiceProvider,    // Provides services
}
```

## Plugin Manager

Central plugin management system:

```rust
impl PluginManager {
    pub fn new() -> Self
    
    // Plugin lifecycle
    pub fn register_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<()>
    pub fn unregister_plugin(&mut self, plugin_id: &str) -> Result<()>
    pub fn initialize_plugin(&mut self, plugin_id: &str) -> Result<()>
    pub fn shutdown_plugin(&mut self, plugin_id: &str) -> Result<()>
    
    // Plugin queries
    pub fn get_plugin(&self, plugin_id: &str) -> Option<&dyn Plugin>
    pub fn get_plugin_mut(&mut self, plugin_id: &str) -> Option<&mut dyn Plugin>
    pub fn list_plugins(&self) -> Vec<&PluginMetadata>
    pub fn find_plugins_by_capability(&self, capability: PluginCapability) -> Vec<&dyn Plugin>
    
    // Widget plugins
    pub fn register_widget_plugin(&mut self, plugin: Box<dyn WidgetPlugin>) -> Result<()>
    pub fn create_widget(&self, widget_type: &str, config: WidgetConfig) -> Result<Box<dyn WidgetPlugin>>
    pub fn list_widget_types(&self) -> Vec<String>
    
    // Dependency management
    pub fn resolve_dependencies(&self, plugin_id: &str) -> Result<Vec<String>>
    pub fn check_dependencies(&self, metadata: &PluginMetadata) -> Result<()>
    
    // Event handling
    pub fn broadcast_event(&mut self, event: &PluginEvent) -> Vec<PluginResponse>
    pub fn send_event_to_plugin(&mut self, plugin_id: &str, event: &PluginEvent) -> Option<PluginResponse>
}
```

## Plugin Context

Runtime context provided to plugins:

```rust
pub struct PluginContext {
    // Framework access
    app: &mut TuiApp,
    
    // Configuration
    config: HashMap<String, serde_json::Value>,
    
    // Inter-plugin communication
    message_bus: &mut MessageBus,
    
    // Resource management
    resources: HashMap<String, Box<dyn Any>>,
}

impl PluginContext {
    // Configuration access
    pub fn get_config<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>>
    pub fn set_config<T: Serialize>(&mut self, key: &str, value: T) -> Result<()>
    
    // Framework integration
    pub fn get_app(&self) -> &TuiApp
    pub fn get_app_mut(&mut self) -> &mut TuiApp
    
    // Resource management
    pub fn set_resource<T: 'static>(&mut self, key: &str, resource: T)
    pub fn get_resource<T: 'static>(&self, key: &str) -> Option<&T>
    pub fn remove_resource(&mut self, key: &str) -> Option<Box<dyn Any>>
    
    // Inter-plugin communication
    pub fn send_message(&mut self, target: &str, message: serde_json::Value) -> Result<()>
    pub fn subscribe_to_events(&mut self, event_types: Vec<&str>) -> Result<()>
    pub fn publish_event(&mut self, event: PluginEvent) -> Result<()>
}
```

## Plugin Events

Event system for plugin communication:

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginEvent {
    // Lifecycle events
    PluginLoaded { plugin_id: String },
    PluginUnloaded { plugin_id: String },
    
    // Framework events
    AppStarted,
    AppStopping,
    ThemeChanged { theme_name: String },
    WindowResized { width: u16, height: u16 },
    
    // Widget events
    WidgetCreated { widget_type: String, widget_id: String },
    WidgetDestroyed { widget_id: String },
    WidgetClicked { widget_id: String },
    WidgetFocused { widget_id: String },
    
    // Custom events
    Custom { event_type: String, data: serde_json::Value },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PluginResponse {
    // Event handling
    Handled,
    Ignored,
    StopPropagation,
    
    // Data responses
    Data { data: serde_json::Value },
    Error { message: String },
    
    // Custom responses
    Custom { response_type: String, data: serde_json::Value },
}
```

## Creating Custom Widgets

### Basic Widget Plugin

```rust
use reactive_tui::plugin::*;
use reactive_tui::components::*;

#[derive(Debug)]
pub struct CounterWidget {
    id: String,
    config: WidgetConfig,
    count: i32,
}

impl Plugin for CounterWidget {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: self.id.clone(),
            name: "Counter Widget".to_string(),
            version: "1.0.0".to_string(),
            author: "Plugin Developer".to_string(),
            description: "A simple counter widget".to_string(),
            entry_point: "counter_widget".to_string(),
            homepage: Some("https://github.com/example/counter-widget".to_string()),
            dependencies: vec![],
            capabilities: vec![PluginCapability::WidgetProvider],
            tags: vec!["widget".to_string(), "counter".to_string()],
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()> {
        // Initialize widget state from config
        if let Some(initial_count) = context.get_config::<i32>("initial_count")? {
            self.count = initial_count;
        }
        
        // Subscribe to relevant events
        context.subscribe_to_events(vec!["widget_clicked"])?;
        
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<()> {
        // Cleanup resources
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        match event {
            PluginEvent::WidgetClicked { widget_id } if widget_id == &self.id => {
                self.count += 1;
                Some(PluginResponse::Handled)
            }
            _ => None
        }
    }
    
    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}

impl Component for CounterWidget {
    fn render(&self) -> Element {
        div()
            .class("counter-widget")
            .child(
                text(&format!("Count: {}", self.count))
                    .class("counter-display")
                    .build()
            )
            .child(
                button("increment", |config| {
                    config.text("Increment")
                          .on_click("increment_counter")
                }).to_element()
            )
            .build()
    }
    
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        match action.name() {
            "increment_counter" => {
                self.count += 1;
                Ok(ActionResult::Handled)
            }
            _ => Ok(ActionResult::Ignored)
        }
    }
}

impl WidgetPlugin for CounterWidget {
    fn widget_type(&self) -> &str {
        "counter"
    }
    
    fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> {
        Box::new(CounterWidget {
            id: config.id.clone(),
            config,
            count: 0,
        })
    }
    
    fn config_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "initial_count": {
                    "type": "integer",
                    "default": 0,
                    "description": "Initial counter value"
                },
                "max_count": {
                    "type": "integer",
                    "description": "Maximum counter value"
                }
            }
        })
    }
    
    fn validate_config(&self, config: &WidgetConfig) -> Result<()> {
        // Validate configuration parameters
        if let Some(initial_count) = config.get_property::<i32>("initial_count")? {
            if initial_count < 0 {
                return Err(TuiError::Config("initial_count must be non-negative".into()));
            }
        }
        
        if let Some(max_count) = config.get_property::<i32>("max_count")? {
            if max_count <= 0 {
                return Err(TuiError::Config("max_count must be positive".into()));
            }
        }
        
        Ok(())
    }
}
```

### Advanced Widget with State Management

```rust
use reactive_tui::reactive::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub id: String,
    pub text: String,
    pub completed: bool,
}

#[derive(Debug)]
pub struct TodoListWidget {
    id: String,
    config: WidgetConfig,
    state: ReactiveState<Vec<TodoItem>>,
}

impl WidgetPlugin for TodoListWidget {
    fn widget_type(&self) -> &str {
        "todo_list"
    }
    
    fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> {
        let initial_todos = config.get_property::<Vec<TodoItem>>("todos")
            .unwrap_or_default()
            .unwrap_or_default();
            
        Box::new(TodoListWidget {
            id: config.id.clone(),
            config,
            state: ReactiveState::new(initial_todos),
        })
    }
    
    fn config_schema(&self) -> serde_json::Value {
        serde_json::json!({
            "type": "object",
            "properties": {
                "todos": {
                    "type": "array",
                    "items": {
                        "type": "object",
                        "properties": {
                            "id": { "type": "string" },
                            "text": { "type": "string" },
                            "completed": { "type": "boolean" }
                        },
                        "required": ["id", "text", "completed"]
                    },
                    "description": "Initial todo items"
                },
                "allow_add": {
                    "type": "boolean",
                    "default": true,
                    "description": "Allow adding new todos"
                },
                "allow_delete": {
                    "type": "boolean", 
                    "default": true,
                    "description": "Allow deleting todos"
                }
            }
        })
    }
}

impl Component for TodoListWidget {
    fn render(&self) -> Element {
        let todos = self.state.get();
        let allow_add = self.config.get_property::<bool>("allow_add").unwrap_or(true);
        
        let mut todo_list = div().class("todo-list");
        
        // Render todo items
        for todo in todos.iter() {
            let todo_item = div()
                .class("todo-item")
                .child(
                    checkbox(&format!("todo_{}", todo.id), |config| {
                        config.checked(todo.completed)
                              .label(&todo.text)
                              .on_change(&format!("toggle_{}", todo.id))
                    }).to_element()
                )
                .child(
                    button(&format!("delete_{}", todo.id), |config| {
                        config.text("Delete")
                              .variant("danger")
                              .size("small")
                              .on_click(&format!("delete_{}", todo.id))
                    }).to_element()
                )
                .build();
                
            todo_list = todo_list.child(todo_item);
        }
        
        // Add new todo input if allowed
        if allow_add {
            let add_section = div()
                .class("add-todo")
                .child(
                    input("new_todo", |config| {
                        config.placeholder("Enter new todo...")
                              .on_submit("add_todo")
                    }).to_element()
                )
                .child(
                    button("add_btn", |config| {
                        config.text("Add")
                              .variant("primary")
                              .on_click("add_todo")
                    }).to_element()
                )
                .build();
                
            todo_list = todo_list.child(add_section);
        }
        
        todo_list.build()
    }
    
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        let action_name = action.name();
        
        if action_name == "add_todo" {
            if let Some(text) = action.get_data::<String>("text") {
                let new_todo = TodoItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    text: text.clone(),
                    completed: false,
                };
                
                self.state.update(|todos| {
                    todos.push(new_todo);
                });
                
                return Ok(ActionResult::Handled);
            }
        }
        
        if action_name.starts_with("toggle_") {
            let todo_id = &action_name[7..]; // Remove "toggle_" prefix
            
            self.state.update(|todos| {
                if let Some(todo) = todos.iter_mut().find(|t| t.id == todo_id) {
                    todo.completed = !todo.completed;
                }
            });
            
            return Ok(ActionResult::Handled);
        }
        
        if action_name.starts_with("delete_") {
            let todo_id = &action_name[7..]; // Remove "delete_" prefix
            
            self.state.update(|todos| {
                todos.retain(|t| t.id != todo_id);
            });
            
            return Ok(ActionResult::Handled);
        }
        
        Ok(ActionResult::Ignored)
    }
}
```

## Plugin Registration and Usage

### Registering Plugins

```rust
use reactive_tui::plugin::*;

fn main() -> Result<()> {
    let mut app = TuiApp::new("Plugin Demo")?;
    let mut plugin_manager = PluginManager::new();
    
    // Register widget plugins
    plugin_manager.register_widget_plugin(Box::new(CounterWidget {
        id: "counter".to_string(),
        config: WidgetConfig::default(),
        count: 0,
    }))?;
    
    plugin_manager.register_widget_plugin(Box::new(TodoListWidget {
        id: "todos".to_string(),
        config: WidgetConfig::default(),
        state: ReactiveState::new(vec![]),
    }))?;
    
    // Initialize all plugins
    plugin_manager.initialize_all_plugins()?;
    
    // Use plugins to create widgets
    let counter = plugin_manager.create_widget("counter", WidgetConfig {
        id: "my_counter".to_string(),
        properties: [("initial_count".to_string(), json!(5))].into(),
    })?;
    
    let todos = plugin_manager.create_widget("todo_list", WidgetConfig {
        id: "my_todos".to_string(),
        properties: [
            ("allow_add".to_string(), json!(true)),
            ("allow_delete".to_string(), json!(true)),
        ].into(),
    })?;
    
    // Create layout with plugin widgets
    let layout = div()
        .class("app-layout")
        .child(counter.render())
        .child(todos.render())
        .build();
    
    app.set_component(layout);
    app.run().await
}
```

### Plugin Hot Loading

```rust
impl PluginManager {
    // Hot loading support
    pub fn load_plugin_from_file(&mut self, path: &str) -> Result<String> {
        let plugin_lib = unsafe { libloading::Library::new(path)? };
        
        // Get plugin factory function
        let create_plugin: libloading::Symbol<fn() -> Box<dyn Plugin>> = 
            unsafe { plugin_lib.get(b"create_plugin")? };
        
        let plugin = create_plugin();
        let plugin_id = plugin.id().to_string();
        
        self.register_plugin(plugin)?;
        self.initialize_plugin(&plugin_id)?;
        
        Ok(plugin_id)
    }
    
    pub fn unload_plugin(&mut self, plugin_id: &str) -> Result<()> {
        self.shutdown_plugin(plugin_id)?;
        self.unregister_plugin(plugin_id)?;
        Ok(())
    }
    
    pub fn reload_plugin(&mut self, plugin_id: &str, path: &str) -> Result<()> {
        self.unload_plugin(plugin_id)?;
        self.load_plugin_from_file(path)?;
        Ok(())
    }
}
```

## Plugin Macros

Convenience macros for plugin development:

```rust
use reactive_tui::create_plugin;

// Simplified plugin creation
create_plugin! {
    id: "my_plugin",
    name: "My Plugin",
    version: "1.0.0",
    author: "Developer",
    description: "A sample plugin",
    
    widget CounterWidget {
        type: "counter",
        config: {
            initial_count: i32 = 0,
            max_count: Option<i32> = None,
        },
        
        state: {
            count: i32 = config.initial_count,
        },
        
        render: |self| {
            div()
                .child(text(&format!("Count: {}", self.count)))
                .child(button("inc", |c| c.text("+")))
                .build()
        },
        
        actions: {
            "inc" => |self| {
                if let Some(max) = self.config.max_count {
                    if self.count < max {
                        self.count += 1;
                    }
                } else {
                    self.count += 1;
                }
            },
        },
    }
}
```

## Plugin Distribution

### Plugin Manifest

Create a `plugin.toml` file for your plugin:

```toml
[plugin]
id = "my_counter_plugin"
name = "Counter Widget Plugin"
version = "1.0.0"
author = "Your Name <your.email@example.com>"
description = "A simple counter widget for Reactive TUI"
homepage = "https://github.com/yourname/counter-plugin"
repository = "https://github.com/yourname/counter-plugin"
license = "MIT"
keywords = ["widget", "counter", "tui"]

[plugin.dependencies]
reactive-tui = "0.0.3"

[plugin.capabilities]
widget_provider = true
event_handler = false
theme_provider = false

[plugin.config]
initial_count = { type = "integer", default = 0, description = "Initial counter value" }
max_count = { type = "integer", optional = true, description = "Maximum counter value" }

[build]
entry_point = "create_plugin"
```

### Building Plugin Libraries

```bash
# Build as dynamic library
cargo build --lib --crate-type cdylib

# Package plugin
reactive-tui package-plugin --manifest plugin.toml --output counter-plugin.rtp

# Install plugin
reactive-tui install-plugin counter-plugin.rtp

# List installed plugins
reactive-tui list-plugins

# Enable/disable plugins
reactive-tui enable-plugin my_counter_plugin
reactive-tui disable-plugin my_counter_plugin
```

## Best Practices

### Plugin Development

1. **Follow semantic versioning** for plugin versions
2. **Implement proper error handling** in all plugin methods
3. **Use configuration schemas** for type-safe configuration
4. **Subscribe only to relevant events** to avoid performance issues
5. **Clean up resources** in the cleanup method
6. **Test plugin compatibility** with different framework versions

### Performance Considerations

1. **Minimize event subscriptions** - only subscribe to necessary events
2. **Use efficient rendering** - avoid expensive operations in render()
3. **Implement caching** for expensive computations
4. **Profile plugin performance** using built-in profiling tools
5. **Batch operations** when possible to reduce overhead

### Security Guidelines

1. **Validate all inputs** from configuration and events
2. **Sanitize user-provided data** before rendering
3. **Use safe plugin loading** mechanisms
4. **Implement capability-based permissions** for sensitive operations
5. **Audit plugin dependencies** for security vulnerabilities

The plugin system provides a powerful foundation for extending Reactive TUI with custom functionality while maintaining framework stability and performance.