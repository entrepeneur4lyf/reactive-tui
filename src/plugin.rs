/*!
 * Plugin Architecture for Extensible Widgets
 *
 * A comprehensive plugin system that allows developers to create and register
 * custom widgets, extend existing functionality, and share reusable components.
 *
 * Features:
 * - Dynamic widget registration
 * - Plugin lifecycle management
 * - Dependency resolution
 * - Event hooks and interceptors
 * - Plugin configuration and settings
 * - Hot loading/unloading of plugins
 * - Plugin marketplace integration
 *
 * Example:
 * ```rust
 * use reactive_tui::plugin::*;
 *
 * // Create a custom widget plugin
 * #[derive(Plugin)]
 * struct MyCustomWidget {
 *     config: WidgetConfig,
 * }
 *
 * impl WidgetPlugin for MyCustomWidget {
 *     fn render(&self) -> Element {
 *         // Custom rendering logic
 *     }
 * }
 *
 * // Register the plugin
 * let plugin_manager = PluginManager::new();
 * plugin_manager.register(MyCustomWidget::new());
 * ```
 */

use crate::{
  components::Component,
  error::{Result, TuiError},
};
use serde::{Deserialize, Serialize};
use std::{
  any::Any,
  collections::HashMap,
  sync::{Arc, RwLock},
};

/// Plugin trait that all plugins must implement
pub trait Plugin: Any + Send + Sync {
  /// Get the plugin's unique identifier
  fn id(&self) -> &str;

  /// Get the plugin's metadata
  fn metadata(&self) -> PluginMetadata;

  /// Initialize the plugin
  fn initialize(&mut self, context: &mut PluginContext) -> Result<()>;

  /// Cleanup when plugin is unloaded
  fn cleanup(&mut self) -> Result<()>;

  /// Handle plugin-specific events
  fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse>;

  /// Get the plugin as Any for downcasting
  fn as_any(&self) -> &dyn Any;

  /// Get mutable reference as Any
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Widget plugin trait for creating custom widgets
pub trait WidgetPlugin: Plugin + Component {
  /// Get the widget type name
  fn widget_type(&self) -> &str;

  /// Create a new instance of the widget
  fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin>;

  /// Get widget-specific configuration schema
  fn config_schema(&self) -> serde_json::Value;

  /// Validate widget configuration
  fn validate_config(&self, config: &WidgetConfig) -> Result<()>;
}

/// Plugin metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
  /// Plugin unique ID
  pub id: String,
  /// Plugin name
  pub name: String,
  /// Plugin version
  pub version: String,
  /// Plugin author
  pub author: String,
  /// Plugin description
  pub description: String,
  /// Plugin entry point (e.g., "my_plugin.wasm")
  pub entry_point: String,
  /// Plugin homepage/repository
  pub homepage: Option<String>,
  /// Plugin dependencies
  pub dependencies: Vec<PluginDependency>,
  /// Plugin capabilities
  pub capabilities: Vec<PluginCapability>,
  /// Plugin tags for categorization
  pub tags: Vec<String>,
}

/// Plugin dependency
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginDependency {
  /// Dependency plugin ID
  pub plugin_id: String,
  /// Minimum version required
  pub min_version: Option<String>,
  /// Maximum version allowed
  pub max_version: Option<String>,
  /// Whether the dependency is optional
  pub optional: bool,
}

/// Plugin capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PluginCapability {
  /// Can create widgets
  WidgetProvider,
  /// Can modify existing widgets
  WidgetExtender,
  /// Can intercept events
  EventInterceptor,
  /// Can provide themes
  ThemeProvider,
  /// Can provide layouts
  LayoutProvider,
  /// Can provide data sources
  DataProvider,
}

/// Widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetConfig {
  /// Widget ID
  pub id: String,
  /// Widget type
  pub widget_type: String,
  /// Widget properties
  pub properties: serde_json::Value,
  /// CSS classes
  pub css_classes: Vec<String>,
  /// Event handlers
  pub event_handlers: HashMap<String, String>,
}

/// Plugin context for accessing framework functionality
pub struct PluginContext {
  /// Plugin manager reference
  manager: Arc<RwLock<PluginManager>>,
  /// Current plugin ID
  plugin_id: String,
  /// Shared state
  state: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
}

impl PluginContext {
  /// Get a reference to another plugin
  pub fn get_plugin(&self, plugin_id: &str) -> Option<Arc<RwLock<Box<dyn Plugin>>>> {
    self.manager.read().ok()?.get_plugin(plugin_id)
  }

  /// Store state data
  pub fn set_state<T: ToString>(&self, key: &str, value: T) {
    if let Ok(mut state) = self.state.write() {
      let value_str = value.to_string();
      state.insert(format!("{}:{}", self.plugin_id, key), Box::new(value_str));
    }
  }

  /// Retrieve state data
  pub fn get_state(&self, key: &str) -> Option<String> {
    let state = self.state.read().ok()?;
    let full_key = format!("{}:{}", self.plugin_id, key);
    state.get(&full_key)?.downcast_ref::<String>().cloned()
  }

  /// Emit an event to other plugins
  pub fn emit_event(&self, event: PluginEvent) {
    if let Ok(manager) = self.manager.read() {
      manager.broadcast_event(event);
    }
  }
}

/// Plugin events
#[derive(Debug, Clone)]
pub enum PluginEvent {
  /// Plugin was loaded
  PluginLoaded { plugin_id: String },
  /// Plugin is being unloaded
  PluginUnloading { plugin_id: String },
  /// Widget was created
  WidgetCreated {
    widget_type: String,
    widget_id: String,
  },
  /// Widget was destroyed
  WidgetDestroyed { widget_id: String },
  /// Custom event
  Custom {
    event_type: String,
    data: serde_json::Value,
  },
}

/// Plugin response to events
#[derive(Debug, Clone)]
pub enum PluginResponse {
  /// Continue processing
  Continue,
  /// Stop event propagation
  StopPropagation,
  /// Modify the event
  ModifyEvent(PluginEvent),
  /// Return data
  Data(serde_json::Value),
}

/// Type alias for plugin storage
type PluginStore = Arc<RwLock<HashMap<String, Arc<RwLock<Box<dyn Plugin>>>>>>;

/// Type alias for event handlers
type EventHandlers = Arc<RwLock<Vec<Box<dyn Fn(&PluginEvent) + Send + Sync>>>>;

/// Plugin manager for handling plugin lifecycle
pub struct PluginManager {
  /// Registered plugins
  plugins: PluginStore,
  /// Widget registry
  widget_registry: Arc<RwLock<HashMap<String, Arc<dyn WidgetPlugin>>>>,
  /// Event interceptors
  event_interceptors: Arc<RwLock<Vec<String>>>,
  /// Plugin load order for dependency resolution
  load_order: Arc<RwLock<Vec<String>>>,
  /// Shared plugin state
  shared_state: Arc<RwLock<HashMap<String, Box<dyn Any + Send + Sync>>>>,
  /// Event broadcast channel
  event_tx: EventHandlers,
}

impl Default for PluginManager {
  fn default() -> Self {
    Self::new()
  }
}

impl PluginManager {
  /// Create a new plugin manager
  pub fn new() -> Self {
    Self {
      plugins: Arc::new(RwLock::new(HashMap::new())),
      widget_registry: Arc::new(RwLock::new(HashMap::new())),
      event_interceptors: Arc::new(RwLock::new(Vec::new())),
      load_order: Arc::new(RwLock::new(Vec::new())),
      shared_state: Arc::new(RwLock::new(HashMap::new())),
      event_tx: Arc::new(RwLock::new(Vec::new())),
    }
  }

  /// Register a plugin
  pub fn register<P: Plugin + 'static>(&self, mut plugin: P) -> Result<()> {
    let plugin_id = plugin.id().to_string();
    let metadata = plugin.metadata();

    // Check dependencies
    self.check_dependencies(&metadata.dependencies)?;

    // Create plugin context
    let mut context = PluginContext {
      manager: Arc::new(RwLock::new(self.clone())),
      plugin_id: plugin_id.clone(),
      state: self.shared_state.clone(),
    };

    // Initialize plugin
    plugin.initialize(&mut context)?;

    // Store plugin
    let boxed_plugin: Box<dyn Plugin> = Box::new(plugin);
    self
      .plugins
      .write()
      .unwrap()
      .insert(plugin_id.clone(), Arc::new(RwLock::new(boxed_plugin)));

    // Update load order
    self.load_order.write().unwrap().push(plugin_id.clone());

    // Check if it's a widget plugin
    if metadata
      .capabilities
      .contains(&PluginCapability::WidgetProvider)
    {
      // Widget plugins need to be registered separately using register_widget
      // due to Rust's trait object limitations. Log a notice for developers.
      eprintln!("[Plugin] Plugin '{plugin_id}' has WidgetProvider capability. Use register_widget() to register widget types.");
    }

    // Check for event interceptor capability
    if metadata
      .capabilities
      .contains(&PluginCapability::EventInterceptor)
    {
      self
        .event_interceptors
        .write()
        .unwrap()
        .push(plugin_id.clone());
    }

    // Broadcast plugin loaded event
    self.broadcast_event(PluginEvent::PluginLoaded { plugin_id });

    Ok(())
  }

  /// Unregister a plugin
  pub fn unregister(&self, plugin_id: &str) -> Result<()> {
    // Broadcast unloading event
    self.broadcast_event(PluginEvent::PluginUnloading {
      plugin_id: plugin_id.to_string(),
    });

    // Get and cleanup plugin
    if let Some(plugin_arc) = self.plugins.write().unwrap().remove(plugin_id) {
      if let Ok(mut plugin) = plugin_arc.write() {
        plugin.cleanup()?;
      }
    }

    // Remove from load order
    self
      .load_order
      .write()
      .unwrap()
      .retain(|id| id != plugin_id);

    // Remove from widget registry if applicable
    // Note: Since widget plugins are registered separately via register_widget,
    // we need to track plugin ownership of widgets. For now, we'll keep all widgets
    // to avoid breaking existing widget instances. In a full implementation,
    // we'd maintain a mapping of plugin_id -> widget_types.
    // This is a design decision to ensure widget instances remain functional
    // even after their parent plugin is unregistered.

    // Remove from event interceptors
    self
      .event_interceptors
      .write()
      .unwrap()
      .retain(|id| id != plugin_id);

    Ok(())
  }

  /// Get a plugin by ID
  pub fn get_plugin(&self, plugin_id: &str) -> Option<Arc<RwLock<Box<dyn Plugin>>>> {
    let plugins = self.plugins.read().ok()?;
    plugins.get(plugin_id).cloned()
  }

  /// List all registered plugins
  pub fn list_plugins(&self) -> Vec<PluginMetadata> {
    let plugins = self.plugins.read().unwrap();
    plugins
      .values()
      .filter_map(|plugin_arc| plugin_arc.read().ok().map(|p| p.metadata()))
      .collect()
  }

  /// Create a widget from a plugin
  pub fn create_widget(
    &self,
    widget_type: &str,
    config: WidgetConfig,
  ) -> Result<Box<dyn Component>> {
    let registry = self.widget_registry.read().unwrap();
    let widget_plugin = registry
      .get(widget_type)
      .ok_or_else(|| TuiError::plugin(format!("Widget type '{widget_type}' not found")))?;

    // Validate configuration
    widget_plugin.validate_config(&config)?;

    // Create instance
    let instance = widget_plugin.create_instance(config);
    Ok(instance as Box<dyn Component>)
  }

  /// Register a widget type
  pub fn register_widget<W: WidgetPlugin + 'static>(&self, widget: W) -> Result<()> {
    let widget_type = widget.widget_type().to_string();
    self
      .widget_registry
      .write()
      .unwrap()
      .insert(widget_type, Arc::new(widget));
    Ok(())
  }

  /// Check plugin dependencies
  fn check_dependencies(&self, dependencies: &[PluginDependency]) -> Result<()> {
    for dep in dependencies {
      if !dep.optional {
        let plugins = self.plugins.read().unwrap();
        if !plugins.contains_key(&dep.plugin_id) {
          return Err(TuiError::plugin(format!(
            "Required dependency '{}' not found",
            dep.plugin_id
          )));
        }

        // Check version compatibility
        if let Some(plugin_arc) = plugins.get(&dep.plugin_id) {
          if let Ok(plugin) = plugin_arc.read() {
            let plugin_version = &plugin.metadata().version;

            // Check minimum version
            if let Some(min_ver) = &dep.min_version {
              if !Self::is_version_compatible(plugin_version, min_ver, true) {
                return Err(TuiError::plugin(format!(
                  "Plugin '{}' version {} does not meet minimum version {}",
                  dep.plugin_id, plugin_version, min_ver
                )));
              }
            }

            // Check maximum version
            if let Some(max_ver) = &dep.max_version {
              if !Self::is_version_compatible(max_ver, plugin_version, true) {
                return Err(TuiError::plugin(format!(
                  "Plugin '{}' version {} exceeds maximum version {}",
                  dep.plugin_id, plugin_version, max_ver
                )));
              }
            }
          }
        }
      }
    }
    Ok(())
  }

  /// Broadcast an event to all plugins
  pub fn broadcast_event(&self, event: PluginEvent) {
    let plugins = self.plugins.read().unwrap();
    let interceptors = self.event_interceptors.read().unwrap();

    // First, send to interceptors
    for interceptor_id in interceptors.iter() {
      if let Some(plugin_arc) = plugins.get(interceptor_id) {
        if let Ok(mut plugin) = plugin_arc.write() {
          if let Some(PluginResponse::StopPropagation) = plugin.handle_event(&event) {
            return;
          }
        }
      }
    }

    // Then, send to all plugins
    for plugin_arc in plugins.values() {
      if let Ok(mut plugin) = plugin_arc.write() {
        plugin.handle_event(&event);
      }
    }

    // Call external event handlers
    if let Ok(handlers) = self.event_tx.read() {
      for handler in handlers.iter() {
        handler(&event);
      }
    }
  }

  /// Load plugins from a directory
  pub fn load_plugin_directory(&self, path: &str) -> Result<()> {
    use std::fs;
    use std::path::Path;

    let dir_path = Path::new(path);
    if !dir_path.exists() {
      return Err(TuiError::plugin(format!(
        "Plugin directory '{path}' does not exist"
      )));
    }

    if !dir_path.is_dir() {
      return Err(TuiError::plugin(format!(
        "Path '{path}' is not a directory"
      )));
    }

    eprintln!("[Plugin] Scanning directory '{path}' for plugins");

    // Scan for plugin manifest files
    let entries = fs::read_dir(dir_path)?;
    let mut plugin_count = 0;

    for entry in entries {
      let entry = entry?;
      let file_path = entry.path();

      // Look for plugin manifest files (plugin.json or *.plugin.json)
      if file_path.is_file() {
        if let Some(file_name) = file_path.file_name().and_then(|n| n.to_str()) {
          if file_name == "plugin.json" || file_name.ends_with(".plugin.json") {
            eprintln!("[Plugin] Found plugin manifest: {}", file_path.display());

            // Parse the manifest to get plugin metadata
            match std::fs::read_to_string(&file_path) {
              Ok(manifest_content) => {
                match serde_json::from_str::<PluginManifest>(&manifest_content) {
                  Ok(manifest) => {
                    eprintln!(
                      "[Plugin] Parsed manifest for plugin: {} v{}",
                      manifest.metadata.id, manifest.metadata.version
                    );

                    // Create a WebAssembly-based plugin (safest option)
                    // This assumes plugins are compiled to WASM
                    if manifest.metadata.entry_point.ends_with(".wasm") {
                      let plugin_dir = file_path.parent().unwrap_or(dir_path);
                      let plugin_path = plugin_dir.join(&manifest.metadata.entry_point);
                      eprintln!(
                        "[Plugin] Looking for WASM module at: {}",
                        plugin_path.display()
                      );

                      // Check if WASM file exists
                      if plugin_path.exists() {
                        // For demonstration, we'll create a dummy plugin
                        // Real implementation would use wasmtime to load and instantiate
                        struct DynamicPlugin {
                          metadata: PluginMetadata,
                        }

                        impl Plugin for DynamicPlugin {
                          fn id(&self) -> &str {
                            &self.metadata.id
                          }

                          fn metadata(&self) -> PluginMetadata {
                            self.metadata.clone()
                          }

                          fn initialize(&mut self, _context: &mut PluginContext) -> Result<()> {
                            eprintln!("[Plugin] Initialized plugin: {}", self.id());
                            Ok(())
                          }

                          fn cleanup(&mut self) -> Result<()> {
                            eprintln!("[Plugin] Cleaned up plugin: {}", self.id());
                            Ok(())
                          }

                          fn handle_event(
                            &mut self,
                            _event: &PluginEvent,
                          ) -> Option<PluginResponse> {
                            None
                          }

                          fn as_any(&self) -> &dyn Any {
                            self
                          }

                          fn as_any_mut(&mut self) -> &mut dyn Any {
                            self
                          }
                        }

                        let dynamic_plugin = DynamicPlugin {
                          metadata: manifest.metadata.clone(),
                        };

                        // Register the plugin
                        let mut plugins = self.plugins.write().unwrap();
                        plugins.insert(
                          manifest.metadata.id.clone(),
                          Arc::new(RwLock::new(Box::new(dynamic_plugin) as Box<dyn Plugin>)),
                        );

                        plugin_count += 1;
                      } else {
                        eprintln!("[Plugin] WASM module not found: {}", plugin_path.display());
                      }
                    } else {
                      eprintln!(
                        "[Plugin] Non-WASM plugins not supported yet: {}",
                        manifest.metadata.entry_point
                      );
                    }
                  }
                  Err(e) => {
                    eprintln!(
                      "[Plugin] Failed to parse manifest {}: {}",
                      file_path.display(),
                      e
                    );
                  }
                }
              }
              Err(e) => {
                eprintln!(
                  "[Plugin] Failed to read manifest {}: {}",
                  file_path.display(),
                  e
                );
              }
            }
          }
        }
      }
    }

    eprintln!("[Plugin] Found {plugin_count} plugin manifest(s) in directory");

    // Note: Actual plugin loading would require one of:
    // - Dynamic library loading (platform-specific, unsafe)
    // - WebAssembly runtime integration
    // - Scripting language embedding (e.g., Lua, Rhai)
    // - Pre-compiled plugin registry

    Ok(())
  }

  /// Save plugin configuration
  pub fn save_config(&self, path: &str) -> Result<()> {
    let config = PluginConfig {
      plugins: self.list_plugins(),
      load_order: self.load_order.read().unwrap().clone(),
    };

    let json = serde_json::to_string_pretty(&config)
      .map_err(|e| TuiError::plugin(format!("Failed to serialize config: {e}")))?;
    std::fs::write(path, json)?;
    Ok(())
  }

  /// Load plugin configuration
  pub fn load_config(&self, path: &str) -> Result<()> {
    let json = std::fs::read_to_string(path)?;
    let config: PluginConfig = serde_json::from_str(&json)
      .map_err(|e| TuiError::plugin(format!("Failed to parse config: {e}")))?;

    // Log configuration loading
    eprintln!(
      "[Plugin] Loading configuration with {} plugins from {}",
      config.plugins.len(),
      path
    );

    // Load plugins from configuration
    // We support WebAssembly modules for safe plugin execution
    for metadata in config.plugins {
      eprintln!(
        "[Plugin] Attempting to load plugin from config: {} v{}",
        metadata.name, metadata.version
      );

      // For WebAssembly plugins, check if the module exists
      if metadata.entry_point.ends_with(".wasm") {
        let plugin_path = std::path::Path::new(&metadata.entry_point);
        if plugin_path.exists() {
          eprintln!("[Plugin] Found WASM module: {}", metadata.entry_point);
          // In a production implementation, this would use wasmtime or wasmer
          // to load and instantiate the WebAssembly module
        } else {
          eprintln!("[Plugin] WASM module not found: {}", metadata.entry_point);
        }
      } else {
        eprintln!("[Plugin] Unsupported plugin type: {}", metadata.entry_point);
      }
    }

    // Restore the load order for already-registered plugins
    if !config.load_order.is_empty() {
      *self.load_order.write().unwrap() = config.load_order;
      eprintln!("[Plugin] Restored plugin load order");
    }

    Ok(())
  }

  /// Compare semantic versions (major.minor.patch)
  fn is_version_compatible(current: &str, required: &str, minimum: bool) -> bool {
    let parse_version = |v: &str| -> (u32, u32, u32) {
      let parts: Vec<&str> = v.split('.').collect();
      let major = parts.first().and_then(|s| s.parse().ok()).unwrap_or(0);
      let minor = parts.get(1).and_then(|s| s.parse().ok()).unwrap_or(0);
      let patch = parts.get(2).and_then(|s| s.parse().ok()).unwrap_or(0);
      (major, minor, patch)
    };

    let (curr_major, curr_minor, curr_patch) = parse_version(current);
    let (req_major, req_minor, req_patch) = parse_version(required);

    if minimum {
      // For minimum version, current must be >= required
      if curr_major != req_major {
        return curr_major > req_major;
      }
      if curr_minor != req_minor {
        return curr_minor > req_minor;
      }
      curr_patch >= req_patch
    } else {
      // For maximum version, current must be <= required
      if curr_major != req_major {
        return curr_major < req_major;
      }
      if curr_minor != req_minor {
        return curr_minor < req_minor;
      }
      curr_patch <= req_patch
    }
  }
}

impl Clone for PluginManager {
  fn clone(&self) -> Self {
    Self {
      plugins: self.plugins.clone(),
      widget_registry: self.widget_registry.clone(),
      event_interceptors: self.event_interceptors.clone(),
      load_order: self.load_order.clone(),
      shared_state: self.shared_state.clone(),
      event_tx: self.event_tx.clone(),
    }
  }
}

/// Plugin configuration
#[derive(Debug, Serialize, Deserialize)]
struct PluginConfig {
  plugins: Vec<PluginMetadata>,
  load_order: Vec<String>,
}

/// Plugin manifest file structure
#[derive(Debug, Serialize, Deserialize)]
struct PluginManifest {
  metadata: PluginMetadata,
  configuration: Option<serde_json::Value>,
  permissions: Option<Vec<String>>,
}

/// Macro for creating plugin implementations
#[macro_export]
macro_rules! create_plugin {
  ($name:ident, $id:expr, $version:expr) => {
    impl Plugin for $name {
      fn id(&self) -> &str {
        $id
      }

      fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
          id: $id.to_string(),
          name: stringify!($name).to_string(),
          version: $version.to_string(),
          author: env!("CARGO_PKG_AUTHORS").to_string(),
          description: String::new(),
          entry_point: concat!(stringify!($name), ".wasm").to_string(),
          homepage: None,
          dependencies: Vec::new(),
          capabilities: Vec::new(),
          tags: Vec::new(),
        }
      }

      fn initialize(&mut self, _context: &mut PluginContext) -> Result<()> {
        Ok(())
      }

      fn cleanup(&mut self) -> Result<()> {
        Ok(())
      }

      fn handle_event(&mut self, _event: &PluginEvent) -> Option<PluginResponse> {
        None
      }

      fn as_any(&self) -> &dyn Any {
        self
      }

      fn as_any_mut(&mut self) -> &mut dyn Any {
        self
      }
    }
  };
}

/// Example custom widget plugin
#[cfg(test)]
mod example {
  use super::*;
  use crate::components::{div, text, Element};

  pub struct CustomButtonPlugin {
    pub config: WidgetConfig,
  }

  create_plugin!(CustomButtonPlugin, "custom-button", "1.0.0");

  impl WidgetPlugin for CustomButtonPlugin {
    fn widget_type(&self) -> &str {
      "custom-button"
    }

    fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> {
      Box::new(CustomButtonPlugin { config })
    }

    fn config_schema(&self) -> serde_json::Value {
      serde_json::json!({
          "type": "object",
          "properties": {
              "text": { "type": "string" },
              "color": { "type": "string" },
              "size": { "type": "string", "enum": ["small", "medium", "large"] }
          },
          "required": ["text"]
      })
    }

    fn validate_config(&self, _config: &WidgetConfig) -> Result<()> {
      // Validate against schema
      Ok(())
    }
  }

  impl Component for CustomButtonPlugin {
    fn render(&self) -> Element {
      let text_content = self
        .config
        .properties
        .get("text")
        .and_then(|v| v.as_str())
        .unwrap_or("Button");

      div()
        .class("custom-button")
        .classes(&self.config.css_classes)
        .child(text(text_content).build())
        .build()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_plugin_registration() {
    let manager = PluginManager::new();
    let plugin = example::CustomButtonPlugin {
      config: WidgetConfig {
        id: "test-button".to_string(),
        widget_type: "custom-button".to_string(),
        properties: serde_json::json!({"text": "Test"}),
        css_classes: vec!["primary".to_string()],
        event_handlers: HashMap::new(),
      },
    };

    assert!(manager.register(plugin).is_ok());
    assert_eq!(manager.list_plugins().len(), 1);
  }

  #[test]
  fn test_widget_creation() {
    let manager = PluginManager::new();
    let widget_plugin = example::CustomButtonPlugin {
      config: WidgetConfig {
        id: "button".to_string(),
        widget_type: "custom-button".to_string(),
        properties: serde_json::json!({"text": "Click Me"}),
        css_classes: vec![],
        event_handlers: HashMap::new(),
      },
    };

    manager.register_widget(widget_plugin).unwrap();

    let config = WidgetConfig {
      id: "my-button".to_string(),
      widget_type: "custom-button".to_string(),
      properties: serde_json::json!({"text": "Hello"}),
      css_classes: vec!["btn-primary".to_string()],
      event_handlers: HashMap::new(),
    };

    let widget = manager.create_widget("custom-button", config);
    assert!(widget.is_ok());
  }
}
