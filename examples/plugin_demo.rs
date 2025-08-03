/*!
 * Plugin System Demo
 *
 * Demonstrates the extensible plugin architecture with custom widgets,
 * event interception, and dynamic plugin loading/unloading.
 */

use reactive_tui::components::{div, text, Element};
use reactive_tui::plugin::*;
use reactive_tui::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// Custom gauge widget plugin
struct GaugeWidgetPlugin {
  config: WidgetConfig,
}

impl Plugin for GaugeWidgetPlugin {
  fn id(&self) -> &str {
    "gauge-widget"
  }

  fn metadata(&self) -> PluginMetadata {
    PluginMetadata {
      id: "gauge-widget".to_string(),
      name: "Gauge Widget".to_string(),
      version: "1.0.0".to_string(),
      author: "Demo Author".to_string(),
      description: "A circular gauge widget for displaying values".to_string(),
      entry_point: "gauge_widget.wasm".to_string(),
      homepage: None,
      dependencies: vec![],
      capabilities: vec![PluginCapability::WidgetProvider],
      tags: vec![
        "widget".to_string(),
        "gauge".to_string(),
        "visualization".to_string(),
      ],
    }
  }

  fn initialize(&mut self, context: &mut PluginContext) -> Result<()> {
    println!("🔌 Gauge Widget Plugin initialized");
    context.set_state("theme", "default");
    Ok(())
  }

  fn cleanup(&mut self) -> Result<()> {
    println!("🔌 Gauge Widget Plugin cleaned up");
    Ok(())
  }

  fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
    match event {
      PluginEvent::Custom {
        event_type,
        data: _,
      } if event_type == "theme-change" => {
        println!("🎨 Gauge widget received theme change event");
        Some(PluginResponse::Continue)
      }
      _ => None,
    }
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

impl WidgetPlugin for GaugeWidgetPlugin {
  fn widget_type(&self) -> &str {
    "gauge"
  }

  fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> {
    Box::new(GaugeWidgetPlugin { config })
  }

  fn config_schema(&self) -> serde_json::Value {
    serde_json::json!({
        "type": "object",
        "properties": {
            "value": { "type": "number", "minimum": 0, "maximum": 100 },
            "label": { "type": "string" },
            "color": { "type": "string" },
            "size": { "type": "string", "enum": ["small", "medium", "large"] }
        },
        "required": ["value"]
    })
  }

  fn validate_config(&self, config: &WidgetConfig) -> Result<()> {
    if config.properties.get("value").is_none() {
      return Err(TuiError::plugin("Gauge widget requires 'value' property"));
    }
    Ok(())
  }
}

impl Component for GaugeWidgetPlugin {
  fn render(&self) -> Element {
    let value = self
      .config
      .properties
      .get("value")
      .and_then(|v| v.as_f64())
      .unwrap_or(0.0);
    let label = self
      .config
      .properties
      .get("label")
      .and_then(|v| v.as_str())
      .unwrap_or("Gauge");
    let _size = self
      .config
      .properties
      .get("size")
      .and_then(|v| v.as_str())
      .unwrap_or("medium");

    // Create a simple text-based gauge representation
    let percentage = (value.clamp(0.0, 100.0) / 100.0 * 10.0) as usize;
    let filled = "█".repeat(percentage);
    let empty = "░".repeat(10 - percentage);

    div()
      .class("gauge-widget")
      .classes(&self.config.css_classes)
      .child(text(format!("{label}: [{filled}{empty}] {value:.0}%")).build())
      .build()
  }
}

/// Event logger plugin that intercepts all events
struct EventLoggerPlugin {
  event_count: Arc<RwLock<usize>>,
}

impl Plugin for EventLoggerPlugin {
  fn id(&self) -> &str {
    "event-logger"
  }

  fn metadata(&self) -> PluginMetadata {
    PluginMetadata {
      id: "event-logger".to_string(),
      name: "Event Logger".to_string(),
      version: "1.0.0".to_string(),
      author: "Demo Author".to_string(),
      description: "Logs all plugin events for debugging".to_string(),
      entry_point: "event_logger.wasm".to_string(),
      homepage: None,
      dependencies: vec![],
      capabilities: vec![PluginCapability::EventInterceptor],
      tags: vec!["debug".to_string(), "logging".to_string()],
    }
  }

  fn initialize(&mut self, _context: &mut PluginContext) -> Result<()> {
    println!("📝 Event Logger Plugin initialized");
    Ok(())
  }

  fn cleanup(&mut self) -> Result<()> {
    let count = *self.event_count.read().unwrap();
    println!("📝 Event Logger Plugin cleaned up. Total events logged: {count}");
    Ok(())
  }

  fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
    *self.event_count.write().unwrap() += 1;

    match event {
      PluginEvent::PluginLoaded { plugin_id } => {
        println!("📝 [EVENT] Plugin loaded: {plugin_id}");
      }
      PluginEvent::PluginUnloading { plugin_id } => {
        println!("📝 [EVENT] Plugin unloading: {plugin_id}");
      }
      PluginEvent::WidgetCreated {
        widget_type,
        widget_id,
      } => {
        println!("📝 [EVENT] Widget created: {widget_type} ({widget_id})");
      }
      PluginEvent::Custom { event_type, .. } => {
        println!("📝 [EVENT] Custom event: {event_type}");
      }
      _ => {}
    }

    Some(PluginResponse::Continue)
  }

  fn as_any(&self) -> &dyn std::any::Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
    self
  }
}

fn main() -> Result<()> {
  println!("🔌 Plugin System Demo");
  println!("====================\n");

  // Create plugin manager
  let plugin_manager = Arc::new(RwLock::new(PluginManager::new()));

  // Register event logger plugin first (as it's an interceptor)
  {
    let manager = plugin_manager.write().unwrap();
    let logger_plugin = EventLoggerPlugin {
      event_count: Arc::new(RwLock::new(0)),
    };
    manager.register(logger_plugin)?;
  }

  // Register gauge widget plugin
  {
    let manager = plugin_manager.write().unwrap();
    let gauge_plugin = GaugeWidgetPlugin {
      config: WidgetConfig {
        id: "gauge-plugin".to_string(),
        widget_type: "gauge".to_string(),
        properties: serde_json::json!({}),
        css_classes: vec![],
        event_handlers: HashMap::new(),
      },
    };
    manager.register_widget(gauge_plugin)?;
  }

  // State for UI
  let plugin_list = Arc::new(RwLock::new(Vec::new()));
  let _gauge_values = Arc::new(RwLock::new([75.0, 42.0, 89.0]));

  // Update plugin list
  {
    let manager = plugin_manager.read().unwrap();
    *plugin_list.write().unwrap() = manager.list_plugins();
  }

  // Demo complete - print plugin information
  println!("🔌 Plugin Demo Complete!");
  println!("======================\n");

  let plugins = plugin_manager.read().unwrap().list_plugins();
  for plugin in &plugins {
    println!("📦 {} v{}", plugin.name, plugin.version);
    println!("   Author: {}", plugin.author);
    println!("   Description: {}", plugin.description);
    println!("   Capabilities: {:?}\n", plugin.capabilities);
  }

  // Create and render some gauge widgets
  println!("🔌 Creating sample widgets:");
  let gauge_values = [75.0, 42.0, 89.0];

  for (i, value) in gauge_values.iter().enumerate() {
    let config = WidgetConfig {
      id: format!("gauge-{i}"),
      widget_type: "gauge".to_string(),
      properties: serde_json::json!({
          "value": value,
          "label": format!("Metric {}", i + 1),
          "size": "medium"
      }),
      css_classes: vec!["gauge".to_string()],
      event_handlers: HashMap::new(),
    };

    match plugin_manager
      .read()
      .unwrap()
      .create_widget("gauge", config)
    {
      Ok(widget) => {
        let element = widget.render();
        println!("✅ Created gauge-{i}: {element:?}");
      }
      Err(e) => println!("❌ Failed to create gauge-{i}: {e}"),
    }
  }

  // Test event broadcasting
  println!("\n🎨 Testing event system:");
  plugin_manager
    .read()
    .unwrap()
    .broadcast_event(PluginEvent::Custom {
      event_type: "theme-change".to_string(),
      data: serde_json::json!({"theme": "dark"}),
    });

  // Cleanup plugins
  {
    let manager = plugin_manager.write().unwrap();
    manager.unregister("event-logger")?;
  }

  Ok(())
}
