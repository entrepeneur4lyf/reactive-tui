/*!
 * 🚀 Plugin System Showcase
 * 
 * A comprehensive demonstration of the TUI framework's plugin architecture,
 * featuring custom widgets, theme providers, data sources, and event handling.
 * 
 * This showcase demonstrates:
 * - Custom widget plugins (gauge, chart, status indicators)
 * - Theme provider plugins with hot-swapping
 * - Data source plugins with live updates
 * - Event interception and custom actions
 * - Plugin dependencies and lifecycle management
 */

use tui_core::prelude::*;
use tui_core::plugin::*;
use tui_core::components::{div, text, Element};
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// 📊 Gauge Widget Plugin - Visual progress indicators  
#[allow(dead_code)]
struct GaugeWidgetPlugin {
    id: String,
}

#[allow(dead_code)]
impl GaugeWidgetPlugin {
    fn new() -> Self {
        Self {
            id: "gauge-widget".to_string(),
        }
    }
}

impl Plugin for GaugeWidgetPlugin {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "gauge-widget".to_string(),
            name: "Gauge Widget".to_string(),
            version: "2.0.0".to_string(),
            author: "TUI Framework Team".to_string(),
            description: "High-performance gauge widgets with animations".to_string(),
            entry_point: "gauge_widget.wasm".to_string(),
            homepage: Some("https://github.com/tui-framework/gauge-plugin".to_string()),
            dependencies: vec![
                PluginDependency {
                    plugin_id: "theme-provider".to_string(),
                    min_version: Some("1.0.0".to_string()),
                    max_version: None,
                    optional: false,
                },
            ],
            capabilities: vec![PluginCapability::WidgetProvider],
            tags: vec!["widget".to_string(), "visualization".to_string(), "gauge".to_string()],
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()> {
        println!("📊 Gauge Widget Plugin v2.0.0 initializing...");
        context.set_state("animation_enabled", "true");
        context.set_state("default_style", "modern");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<()> {
        println!("📊 Gauge Widget Plugin shutting down gracefully");
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        match event {
            PluginEvent::Custom { event_type, data } if event_type == "theme-changed" => {
                if let Some(theme) = data.get("theme").and_then(|v| v.as_str()) {
                    println!("📊 Gauge adapting to {theme} theme");
                }
                Some(PluginResponse::Continue)
            }
            _ => None
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// Actual gauge widget implementation
struct GaugeWidget {
    config: WidgetConfig,
}

impl Plugin for GaugeWidget {
    fn id(&self) -> &str { &self.config.id }
    fn metadata(&self) -> PluginMetadata { 
        PluginMetadata {
            id: self.config.id.clone(),
            name: "Gauge Widget Instance".to_string(),
            version: "2.0.0".to_string(),
            author: "TUI Framework Team".to_string(),
            description: "Gauge widget instance".to_string(),
            entry_point: "gauge_widget_instance.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::WidgetProvider],
            tags: vec!["widget".to_string()],
        }
    }
    fn initialize(&mut self, _: &mut PluginContext) -> Result<()> { Ok(()) }
    fn cleanup(&mut self) -> Result<()> { Ok(()) }
    fn handle_event(&mut self, _: &PluginEvent) -> Option<PluginResponse> { None }
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

impl Component for GaugeWidget {
    fn render(&self) -> Element {
        let value = self.config.properties.get("value")
            .and_then(|v| v.as_f64())
            .unwrap_or(0.0);
        let label = self.config.properties.get("label")
            .and_then(|v| v.as_str())
            .unwrap_or("Progress");
        let style = self.config.properties.get("style")
            .and_then(|v| v.as_str())
            .unwrap_or("modern");
        
        let percentage = value.clamp(0.0, 100.0);
        let filled = (percentage / 10.0) as usize;
        
        let (filled_char, empty_char) = match style {
            "modern" => ('█', '░'),
            "classic" => ('=', '-'),
            "minimal" => ('●', '○'),
            _ => ('█', '░'),
        };
        
        let bar = format!(
            "{}{}",
            filled_char.to_string().repeat(filled),
            empty_char.to_string().repeat(10 - filled)
        );
        
        div()
            .class("gauge-widget")
            .classes(&self.config.css_classes)
            .child(text(format!("{label}: [{bar}] {percentage:.0}%")).build())
            .build()
    }
}

impl WidgetPlugin for GaugeWidget {
    fn widget_type(&self) -> &str { "gauge" }
    fn create_instance(&self, config: WidgetConfig) -> Box<dyn WidgetPlugin> {
        Box::new(GaugeWidget { config })
    }
    fn config_schema(&self) -> serde_json::Value { 
        serde_json::json!({
            "type": "object",
            "properties": {
                "value": { "type": "number", "minimum": 0, "maximum": 100 },
                "label": { "type": "string" },
                "style": { "type": "string", "enum": ["modern", "classic", "minimal"] },
                "animated": { "type": "boolean" },
                "color": { "type": "string" }
            },
            "required": ["value"]
        })
    }
    fn validate_config(&self, _: &WidgetConfig) -> Result<()> { Ok(()) }
}

/// 🎨 Theme Provider Plugin - Dynamic theming support
struct ThemeProviderPlugin {
    themes: Arc<RwLock<HashMap<String, ThemeDefinition>>>,
    current_theme: Arc<RwLock<String>>,
}

#[derive(Clone)]
struct ThemeDefinition {
    name: String,
    colors: HashMap<String, String>,
    styles: HashMap<String, String>,
}

impl ThemeProviderPlugin {
    fn new() -> Self {
        let mut themes = HashMap::new();
        
        // Cyberpunk theme
        themes.insert("cyberpunk".to_string(), ThemeDefinition {
            name: "Cyberpunk 2077".to_string(),
            colors: [
                ("primary", "#ff0080"),
                ("secondary", "#00ffff"),
                ("background", "#0a0a0a"),
                ("text", "#f0f0f0"),
                ("accent", "#ffff00"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            styles: [
                ("border", "neon"),
                ("animation", "glitch"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        });
        
        // Nature theme
        themes.insert("nature".to_string(), ThemeDefinition {
            name: "Forest Dreams".to_string(),
            colors: [
                ("primary", "#228b22"),
                ("secondary", "#8b4513"),
                ("background", "#f5f5dc"),
                ("text", "#2f4f2f"),
                ("accent", "#ff6347"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
            styles: [
                ("border", "organic"),
                ("animation", "smooth"),
            ].iter().map(|(k, v)| (k.to_string(), v.to_string())).collect(),
        });
        
        Self {
            themes: Arc::new(RwLock::new(themes)),
            current_theme: Arc::new(RwLock::new("cyberpunk".to_string())),
        }
    }
}

impl Plugin for ThemeProviderPlugin {
    fn id(&self) -> &str {
        "theme-provider"
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "theme-provider".to_string(),
            name: "Theme Provider".to_string(),
            version: "1.5.0".to_string(),
            author: "TUI Framework Team".to_string(),
            description: "Advanced theming with hot-reload support".to_string(),
            entry_point: "theme_provider.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::ThemeProvider],
            tags: vec!["theme".to_string(), "styling".to_string(), "customization".to_string()],
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()> {
        println!("🎨 Theme Provider v1.5.0 loaded with {} themes", self.themes.read().unwrap().len());
        context.set_state("available_themes", "cyberpunk,nature");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<()> {
        println!("🎨 Theme Provider cleaned up");
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        if let PluginEvent::Custom { event_type, data } = event {
            match event_type.as_str() {
                "get-theme" => {
                    let current = self.current_theme.read().unwrap().clone();
                    let theme = self.themes.read().unwrap().get(&current).cloned();
                    if let Some(theme) = theme {
                        return Some(PluginResponse::Data(serde_json::json!({
                            "name": theme.name,
                            "colors": theme.colors,
                            "styles": theme.styles
                        })));
                    }
                }
                "switch-theme" => {
                    if let Some(name) = data.get("name").and_then(|v| v.as_str()) {
                        if self.themes.read().unwrap().contains_key(name) {
                            *self.current_theme.write().unwrap() = name.to_string();
                            println!("🎨 Switched to {name} theme");
                            return Some(PluginResponse::Continue);
                        }
                    }
                }
                _ => {}
            }
        }
        None
    }
    
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// 📡 Data Source Plugin - Live data feeds
struct DataSourcePlugin {
    data_streams: Arc<RwLock<HashMap<String, DataStream>>>,
    running: Arc<RwLock<bool>>,
}

struct DataStream {
    name: String,
    value: f64,
    min: f64,
    max: f64,
    _update_rate: Duration,
    _last_update: Instant,
}

impl DataSourcePlugin {
    fn new() -> Self {
        let mut streams = HashMap::new();
        
        streams.insert("cpu".to_string(), DataStream {
            name: "CPU Usage".to_string(),
            value: 45.0,
            min: 0.0,
            max: 100.0,
            _update_rate: Duration::from_millis(500),
            _last_update: Instant::now(),
        });
        
        streams.insert("memory".to_string(), DataStream {
            name: "Memory Usage".to_string(),
            value: 62.0,
            min: 0.0,
            max: 100.0,
            _update_rate: Duration::from_secs(1),
            _last_update: Instant::now(),
        });
        
        Self {
            data_streams: Arc::new(RwLock::new(streams)),
            running: Arc::new(RwLock::new(false)),
        }
    }
}

impl Plugin for DataSourcePlugin {
    fn id(&self) -> &str {
        "data-source"
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "data-source".to_string(),
            name: "Data Source".to_string(),
            version: "3.0.0".to_string(),
            author: "TUI Framework Team".to_string(),
            description: "Real-time data streaming with multiple feeds".to_string(),
            entry_point: "data_source.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::DataProvider],
            tags: vec!["data".to_string(), "streaming".to_string(), "real-time".to_string()],
        }
    }
    
    fn initialize(&mut self, context: &mut PluginContext) -> Result<()> {
        println!("📡 Data Source v3.0.0 initializing {} data streams", 
                 self.data_streams.read().unwrap().len());
        context.set_state("streams", "cpu,memory");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<()> {
        *self.running.write().unwrap() = false;
        println!("📡 Data Source stopped");
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        match event {
            PluginEvent::Custom { event_type, data: _ } if event_type == "get-streams" => {
                let streams = self.data_streams.read().unwrap();
                let stream_data: Vec<_> = streams.iter().map(|(id, stream)| {
                    serde_json::json!({
                        "id": id,
                        "name": stream.name,
                        "value": stream.value,
                        "min": stream.min,
                        "max": stream.max
                    })
                }).collect();
                
                Some(PluginResponse::Data(serde_json::json!({
                    "streams": stream_data
                })))
            }
            _ => None
        }
    }
    
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// 🔍 Analytics Plugin - Track and analyze plugin usage
struct AnalyticsPlugin {
    event_count: Arc<RwLock<HashMap<String, usize>>>,
    plugin_metrics: Arc<RwLock<HashMap<String, PluginMetrics>>>,
}

struct PluginMetrics {
    load_time: Instant,
    event_count: usize,
    _last_active: Instant,
}

impl AnalyticsPlugin {
    fn new() -> Self {
        Self {
            event_count: Arc::new(RwLock::new(HashMap::new())),
            plugin_metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl Plugin for AnalyticsPlugin {
    fn id(&self) -> &str {
        "analytics"
    }
    
    fn metadata(&self) -> PluginMetadata {
        PluginMetadata {
            id: "analytics".to_string(),
            name: "Analytics Engine".to_string(),
            version: "1.0.0".to_string(),
            author: "TUI Framework Team".to_string(),
            description: "Comprehensive analytics and metrics tracking".to_string(),
            entry_point: "analytics.wasm".to_string(),
            homepage: None,
            dependencies: vec![],
            capabilities: vec![PluginCapability::EventInterceptor],
            tags: vec!["analytics".to_string(), "metrics".to_string(), "monitoring".to_string()],
        }
    }
    
    fn initialize(&mut self, _: &mut PluginContext) -> Result<()> {
        println!("🔍 Analytics Engine v1.0.0 started");
        Ok(())
    }
    
    fn cleanup(&mut self) -> Result<()> {
        let events = self.event_count.read().unwrap();
        let metrics = self.plugin_metrics.read().unwrap();
        
        println!("\n📊 Analytics Summary:");
        println!("═══════════════════");
        
        // Event statistics
        let total_events: usize = events.values().sum();
        println!("Total events tracked: {total_events}");
        
        // Top events
        let mut event_vec: Vec<_> = events.iter().collect();
        event_vec.sort_by(|a, b| b.1.cmp(a.1));
        
        println!("\nTop Events:");
        for (event_type, count) in event_vec.iter().take(5) {
            println!("  • {event_type}: {count} times");
        }
        
        // Plugin statistics
        println!("\nPlugin Metrics:");
        for (plugin_id, metrics) in metrics.iter() {
            let uptime = metrics.load_time.elapsed();
            println!("  • {plugin_id}: {} events, uptime: {:.1}s", 
                     metrics.event_count, uptime.as_secs_f64());
        }
        
        Ok(())
    }
    
    fn handle_event(&mut self, event: &PluginEvent) -> Option<PluginResponse> {
        // Track event counts
        let event_type = match event {
            PluginEvent::PluginLoaded { .. } => "plugin-loaded",
            PluginEvent::PluginUnloading { .. } => "plugin-unloading",
            PluginEvent::WidgetCreated { .. } => "widget-created",
            PluginEvent::WidgetDestroyed { .. } => "widget-destroyed",
            PluginEvent::Custom { event_type, .. } => event_type.as_str(),
        };
        
        *self.event_count.write().unwrap()
            .entry(event_type.to_string())
            .or_insert(0) += 1;
        
        // Track plugin-specific metrics
        if let PluginEvent::PluginLoaded { plugin_id } = event {
            self.plugin_metrics.write().unwrap().insert(plugin_id.clone(), PluginMetrics {
                load_time: Instant::now(),
                event_count: 0,
                _last_active: Instant::now(),
            });
        }
        
        Some(PluginResponse::Continue)
    }
    
    fn as_any(&self) -> &dyn std::any::Any { self }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any { self }
}

/// 🚀 Main showcase application
fn main() -> Result<()> {
    println!("╔══════════════════════════════════════════╗");
    println!("║     🚀 TUI Plugin System Showcase 🚀     ║");
    println!("╚══════════════════════════════════════════╝");
    println!();
    
    // Create plugin manager
    let plugin_manager = PluginManager::new();
    
    // Phase 1: Load core plugins
    println!("📦 Phase 1: Loading Core Plugins");
    println!("════════════════════════════════");
    
    // Load analytics first (event interceptor)
    plugin_manager.register(AnalyticsPlugin::new())?;
    
    // Load theme provider
    plugin_manager.register(ThemeProviderPlugin::new())?;
    
    // Load data source
    plugin_manager.register(DataSourcePlugin::new())?;
    
    // Load widget plugins
    let gauge_widget = GaugeWidget {
        config: WidgetConfig {
            id: "default-gauge".to_string(),
            widget_type: "gauge".to_string(),
            properties: serde_json::json!({}),
            css_classes: vec![],
            event_handlers: HashMap::new(),
        },
    };
    plugin_manager.register_widget(gauge_widget)?;
    
    println!();
    
    // Phase 2: Plugin Discovery
    println!("🔍 Phase 2: Plugin Discovery");
    println!("═══════════════════════════");
    
    let plugins = plugin_manager.list_plugins();
    for plugin in &plugins {
        println!("📌 {} v{}", plugin.name, plugin.version);
        println!("   Author: {}", plugin.author);
        println!("   Description: {}", plugin.description);
        println!("   Capabilities: {:?}", plugin.capabilities);
        if !plugin.dependencies.is_empty() {
            println!("   Dependencies: {:?}", plugin.dependencies);
        }
        println!();
    }
    
    // Phase 3: Theme Showcase
    println!("🎨 Phase 3: Theme Showcase");
    println!("═════════════════════════");
    
    let themes = ["cyberpunk", "nature"];
    for theme_name in &themes {
        plugin_manager.broadcast_event(PluginEvent::Custom {
            event_type: "switch-theme".to_string(),
            data: serde_json::json!({ "name": theme_name }),
        });
    }
    
    println!();
    
    // Phase 4: Widget Creation
    println!("🛠️  Phase 4: Widget Creation");
    println!("══════════════════════════");
    
    // Create gauge widgets
    let gauge_configs = [
        ("cpu-gauge", "CPU Usage", 75.0),
        ("memory-gauge", "Memory", 62.0),
        ("network-gauge", "Network", 25.0),
    ];
    
    for (id, label, value) in gauge_configs {
        let config = WidgetConfig {
            id: id.to_string(),
            widget_type: "gauge".to_string(),
            properties: serde_json::json!({
                "value": value,
                "label": label,
                "style": "modern",
                "animated": true
            }),
            css_classes: vec!["system-gauge".to_string()],
            event_handlers: HashMap::new(),
        };
        
        match plugin_manager.create_widget("gauge", config) {
            Ok(widget) => {
                let element = widget.render();
                println!("✅ Created {id}: {element:?}");
            }
            Err(e) => println!("❌ Failed to create {id}: {e}"),
        }
    }
    
    println!();
    
    // Phase 5: Plugin Communication
    println!("💬 Phase 5: Plugin Communication");
    println!("═══════════════════════════════");
    
    // Request current theme
    plugin_manager.broadcast_event(PluginEvent::Custom {
        event_type: "get-theme".to_string(),
        data: serde_json::json!({}),
    });
    
    // Request data streams
    plugin_manager.broadcast_event(PluginEvent::Custom {
        event_type: "get-streams".to_string(),
        data: serde_json::json!({}),
    });
    
    println!();
    
    // Phase 6: Cleanup
    println!("🧹 Phase 6: Cleanup");
    println!("═════════════════");
    
    // Unregister plugins in reverse order
    plugin_manager.unregister("data-source")?;
    plugin_manager.unregister("theme-provider")?;
    plugin_manager.unregister("analytics")?;
    
    println!("\n✨ Plugin showcase completed successfully!");
    
    Ok(())
}