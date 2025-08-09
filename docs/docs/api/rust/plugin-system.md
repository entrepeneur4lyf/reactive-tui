# Plugin System (Current Implementation)

This page documents the real plugin system as implemented in src/plugin.rs, rather than an aspirational API.

At a glance:
- Safe architecture: no dynamic library loading; optional WASM manifest discovery is stubbed
- Trait-based plugins executed in-process (Rust), registered programmatically via PluginManager
- Widget plugins require explicit registration (separate from generic plugin registration)
- Config save/load persists metadata and load order only (no dynamic code loading)

## Key types

```rust
use reactive_tui::plugin::{
  Plugin, WidgetPlugin, PluginManager, PluginMetadata, PluginCapability, PluginDependency,
  PluginEvent, PluginResponse, WidgetConfig
};
```

### Plugin trait
```rust
pub trait Plugin: Any + Send + Sync {
  fn id(&self) -> &str;
  fn metadata(&self) -> PluginMetadata;
  fn initialize(&mut self, context: &mut PluginContext) -> Result<>;
  fn cleanup(&mut self) -> Result<>;
  fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse>;
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
```

### WidgetPlugin trait
Widget plugins are also Components and must be registered separately via register_widget.

```rust
pub trait WidgetPlugin: Plugin + Component {
  fn widget_type(&self) -> &str;
  fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin>;
  fn config_schema(&self) -> serde_json::Value;
  fn validate_config(&self, config: &WidgetConfig) -> Result<()>;
}
```

### PluginManager (selected methods)
```rust
impl PluginManager {
  pub fn new() -> Self;
  pub fn register<P: Plugin + 'static>(&self, plugin: P) -> Result<()>;
  pub fn unregister(&self, plugin_id: &str) -> Result<()>;
  pub fn get_plugin(&self, plugin_id: &str) -> Option<Arc<RwLock<Box<dyn Plugin>>>>;
  pub fn list_plugins(&self) -> Vec<PluginMetadata>;
  pub fn register_widget<W: WidgetPlugin + 'static>(&self, widget: W) -> Result<()>;
  pub fn create_widget(&self, widget_type: &str, config: WidgetConfig) -> Result<Box<dyn Component>>;
  pub fn broadcast_event(&self, event: PluginEvent);
  pub fn save_config(&self, path: &str) -> Result<()>;
  pub fn load_config(&self, path: &str) -> Result<()>;
}
```

Notes:
- register() stores the plugin and updates load_order; if the plugin claims WidgetProvider, a notice is logged advising to use register_widget() as well
- create_widget() validates config and returns a Component instance via the registered WidgetPlugin
- load_plugin_directory() scans for plugin manifests (plugin.json or *.plugin.json) and logs prospective WASM modules, but does not execute them (safety-first)
- save_config/load_config serialize/restore metadata and load order; when loading, existing registered plugins are reconciled and load order is restored

## Minimal example

```rust
use reactive_tui::plugin::*;
use reactive_tui::components::{div, text, Component};

struct MyButtonPlugin { config: WidgetConfig }

create_plugin!(MyButtonPlugin, "my-button", "1.0.0");

impl WidgetPlugin for MyButtonPlugin {
  fn widget_type(&self) -> &str { "my-button" }
  fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> { Box::new(MyButtonPlugin { config }) }
  fn config_schema(&self) -> serde_json::Value { serde_json::json!({}) }
  fn validate_config(&self, _config: &WidgetConfig) -> Result<()> { Ok(()) }
}

impl Component for MyButtonPlugin {
  fn render(&self) -> reactive_tui::components::Element {
    div().class("my-btn").child(text("Click").build()).build()
  }
}

fn main() -> reactive_tui::error::Result<()> {
  let manager = PluginManager::new();
  manager.register_widget(MyButtonPlugin { config: WidgetConfig {
    id: "btn1".into(), widget_type: "my-button".into(), properties: serde_json::json!({"text":"Click"}), css_classes: vec![], event_handlers: Default::default()
  }})?;

  // generic registration is optional unless you need events/lifecycle
  // manager.register(MyButtonPlugin { config: ... })?;

  let _comp = manager.create_widget("my-button", WidgetConfig {
    id: "btn2".into(), widget_type: "my-button".into(), properties: serde_json::json!({"text":"Hi"}), css_classes: vec![], event_handlers: Default::default()
  })?;
  Ok(())
}
```

## Persistence

```text
save_config(path): writes PluginConfig { plugins: Vec<PluginMetadata>, load_order: Vec<String> }
load_config(path): restores load order; logs manifests; does not execute WASM by default
```

## Roadmap
- Optional WASM runtime for sandboxed plugin execution
- Registry of widget types by plugin for safer unregister
- Richer event interceptor pipeline
- Tooling to validate manifest schemas

