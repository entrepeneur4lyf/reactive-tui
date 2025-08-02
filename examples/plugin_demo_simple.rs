/*!
 * Simple Plugin System Demo
 * 
 * Demonstrates the plugin architecture with custom widgets
 */

use tui_core::plugin::*;
use std::sync::{Arc, RwLock};

/// Custom widget plugin
struct SimpleWidgetPlugin {
    #[allow(dead_code)]
    widget_type: String,
}

impl Plugin for SimpleWidgetPlugin {
    fn id(&self) -> &str {
        "simple-widget"
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "simple-widget".to_string(),
            name: "Simple Widget".to_string(),
            version: "1.0.0".to_string(),
            author: "Demo Author".to_string(),
            description: "A simple demonstration widget".to_string(),
            entry_point: "simple_widget.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::WidgetProvider],
            tags: vec!["widget".to_string(), "demo".to_string()],
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> tui_core::error::Result<()> {
        println!("🔌 Simple Widget Plugin initialized");
        context.set_state("initialized", "true");
        Ok(())
    }
    
    fn cleanup(&mut self) -> tui_core::error::Result<()> {
        println!("🔌 Simple Widget Plugin cleaned up");
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        match event {
            PluginEvent::PluginLoaded { plugin_id } => {
                println!("📝 [Simple Widget] Plugin loaded: {plugin_id}");
                Some(PluginResponse::Continue)
            }
            _ => None
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}

/// Event logger plugin
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
            description: "Logs all plugin events".to_string(),
            entry_point: "event_logger.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::EventInterceptor],
            tags: vec!["logging".to_string()],
        }
    }
    
    fn initialize(&mut self, _context: &mut PluginContext) -> tui_core::error::Result<()> {
        println!("📝 Event Logger Plugin initialized");
        Ok(())
    }
    
    fn cleanup(&mut self) -> tui_core::error::Result<()> {
        let count = *self.event_count.read().unwrap();
        println!("📝 Event Logger Plugin cleaned up. Total events: {count}");
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
            PluginEvent::WidgetCreated { widget_type, widget_id } => {
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

fn main() -> tui_core::error::Result<()> {
    println!("🔌 Plugin System Demo");
    println!("====================\n");
    
    // Create plugin manager
    let plugin_manager = PluginManager::new();
    
    // Register event logger plugin
    {
        let logger_plugin = EventLoggerPlugin {
            event_count: Arc::new(RwLock::new(0)),
        };
        plugin_manager.register(logger_plugin)?;
    }
    
    // Register simple widget plugin
    {
        let widget_plugin = SimpleWidgetPlugin {
            widget_type: "simple".to_string(),
        };
        plugin_manager.register(widget_plugin)?;
    }
    
    // List registered plugins
    println!("\n📋 Registered Plugins:");
    for plugin in plugin_manager.list_plugins() {
        println!("  • {} v{} by {}", plugin.name, plugin.version, plugin.author);
        println!("    {}", plugin.description);
        println!("    Capabilities: {:?}", plugin.capabilities);
    }
    
    // Trigger a custom event
    println!("\n🎯 Triggering custom event...");
    plugin_manager.broadcast_event(PluginEvent::Custom {
        event_type: "demo-event".to_string(),
        data: serde_json::json!({"message": "Hello from demo!"}),
    });
    
    // Test plugin state
    println!("\n📊 Testing plugin state:");
    if let Some(_plugin_arc) = plugin_manager.get_plugin("simple-widget") {
        println!("  Found simple-widget plugin");
    }
    
    // Cleanup
    println!("\n🧹 Cleaning up...");
    plugin_manager.unregister("simple-widget")?;
    plugin_manager.unregister("event-logger")?;
    
    println!("\n✅ Demo completed successfully!");
    Ok(())
}