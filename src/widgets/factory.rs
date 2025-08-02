/*!
 * Widget Factory Pattern - Rust Implementation
 * 
 * Provides type-safe widget creation with configuration validation,
 * instance caching, and consistent API across all widget types.
 * 
 * This implementation mirrors the TypeScript factory pattern while
 * leveraging Rust's type system for compile-time safety.
 */

use std::collections::HashMap;
use std::sync::{Mutex, RwLock};
use std::sync::Arc;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use thiserror::Error;

use crate::components::Element;
use crate::error::Result;

/// Core widget configuration trait
pub trait WidgetConfig: Clone + Send + Sync + 'static {
    /// Unique identifier for the widget
    fn id(&self) -> &str;
    
    /// Widget type identifier
    fn widget_type(&self) -> &str;
    
    /// CSS classes for styling
    fn classes(&self) -> &[String] { &[] }
    
    /// HTML-like attributes
    fn attributes(&self) -> &HashMap<String, String> { 
        static EMPTY: Lazy<HashMap<String, String>> = Lazy::new(|| HashMap::new());
        &EMPTY
    }
    
    /// Whether the widget is disabled
    fn disabled(&self) -> bool { false }
    
    /// Whether the widget is visible
    fn visible(&self) -> bool { true }
    
    /// Whether the widget can receive focus
    fn focusable(&self) -> bool { false }
    
    /// Tab index for focus order
    fn tab_index(&self) -> Option<i32> { None }
    
    /// Validate the configuration
    fn validate(&self) -> std::result::Result<(), ValidationError> { Ok(()) }
}

/// Widget instance trait
pub trait WidgetInstance: Send + Sync {
    /// Get the widget's unique identifier
    fn id(&self) -> &str;
    
    /// Get the widget type
    fn widget_type(&self) -> &str;
    
    /// Render the widget to an Element
    fn render(&mut self) -> Result<Element>;
    
    /// Update widget configuration
    fn update(&mut self, updates: Box<dyn std::any::Any + Send + Sync>) -> Result<()>;
    
    /// Destroy the widget and clean up resources
    fn destroy(&mut self);
    
    /// Validate widget state
    fn validate(&self) -> bool;
    
    /// Check if widget needs re-rendering
    fn needs_rerender(&self) -> bool;
    
    /// Get performance metrics
    fn get_metrics(&self) -> WidgetMetrics;
}

/// Widget builder function type
pub type WidgetBuilder<C> = Box<dyn Fn(C) -> Box<dyn WidgetInstance> + Send + Sync>;

/// Widget schema for validation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WidgetSchema {
    pub widget_type: String,
    pub required_fields: Vec<String>,
    pub properties: HashMap<String, PropertySchema>,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertySchema {
    pub property_type: PropertyType,
    pub required: bool,
    pub default_value: Option<serde_json::Value>,
    pub enum_values: Option<Vec<serde_json::Value>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PropertyType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}

/// Widget performance metrics
#[derive(Debug, Clone, Default)]
pub struct WidgetMetrics {
    pub render_time_ms: f64,
    pub update_count: u64,
    pub last_render_at: u64,
    pub memory_usage_bytes: usize,
}

/// Widget factory errors
#[derive(Error, Debug)]
pub enum WidgetFactoryError {
    #[error("Unknown widget type: {0}")]
    UnknownType(String),
    
    #[error("Widget validation failed: {0}")]
    ValidationFailed(String),
    
    #[error("Widget creation failed: {0}")]
    CreationFailed(String),
    
    #[error("Widget not found: {0}")]
    NotFound(String),
    
    #[error("Widget already exists: {0}")]
    AlreadyExists(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
}

#[derive(Error, Debug)]
pub enum ValidationError {
    #[error("Missing required field: {0}")]
    MissingField(String),
    
    #[error("Invalid field type for {field}: expected {expected}, got {actual}")]
    InvalidType { field: String, expected: String, actual: String },
    
    #[error("Invalid enum value for {field}: {value}")]
    InvalidEnum { field: String, value: String },
    
    #[error("Custom validation failed for {field}: {message}")]
    CustomValidation { field: String, message: String },
}

/// Widget registry for managing widget types and instances
pub struct WidgetRegistry {
    builders: RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>,
    schemas: RwLock<HashMap<String, WidgetSchema>>,
    instances: Arc<Mutex<HashMap<String, Arc<Mutex<Box<dyn WidgetInstance>>>>>>,
    #[allow(dead_code)]
    performance_stats: Mutex<HashMap<String, WidgetMetrics>>,
}

impl WidgetRegistry {
    /// Create a new widget registry
    pub fn new() -> Self {
        Self {
            builders: RwLock::new(HashMap::new()),
            schemas: RwLock::new(HashMap::new()),
            instances: Arc::new(Mutex::new(HashMap::new())),
            performance_stats: Mutex::new(HashMap::new()),
        }
    }
    
    /// Register a widget type with its builder and schema
    pub fn register<C: WidgetConfig + 'static>(
        &self,
        widget_type: String,
        builder: WidgetBuilder<C>,
        schema: WidgetSchema,
    ) -> Result<()> {
        let mut builders = self.builders.write().unwrap();
        let mut schemas = self.schemas.write().unwrap();
        
        if builders.contains_key(&widget_type) {
            return Err(crate::error::TuiError::component(format!("Widget type already exists: {}", widget_type)));
        }
        
        builders.insert(widget_type.clone(), Box::new(builder));
        schemas.insert(widget_type, schema);
        
        Ok(())
    }
    
    /// Unregister a widget type
    pub fn unregister(&self, widget_type: &str) {
        let mut builders = self.builders.write().unwrap();
        let mut schemas = self.schemas.write().unwrap();
        
        builders.remove(widget_type);
        schemas.remove(widget_type);
    }
    
    /// Get all registered widget types
    pub fn get_types(&self) -> Vec<String> {
        let builders = self.builders.read().unwrap();
        builders.keys().cloned().collect()
    }
    
    /// Check if widget type is registered
    pub fn has_type(&self, widget_type: &str) -> bool {
        let builders = self.builders.read().unwrap();
        builders.contains_key(widget_type)
    }
    
    /// Get widget schema
    pub fn get_schema(&self, widget_type: &str) -> Option<WidgetSchema> {
        let schemas = self.schemas.read().unwrap();
        schemas.get(widget_type).cloned()
    }
    
    /// Create a widget instance
    pub fn create_widget<C: WidgetConfig + 'static>(
        &self,
        config: C,
        options: CreateWidgetOptions,
    ) -> std::result::Result<(), WidgetFactoryError> {
        let widget_type = config.widget_type();
        
        // Validate configuration if requested
        if options.validate_config {
            if let Err(e) = config.validate() {
                return Err(WidgetFactoryError::ValidationFailed(e.to_string()));
            }
            
            if let Some(schema) = self.get_schema(widget_type) {
                if let Err(e) = self.validate_against_schema(&config, &schema) {
                    return Err(WidgetFactoryError::ValidationFailed(e.to_string()));
                }
            }
        }
        
        // Check for cached instance
        if options.use_cache {
            let instances = self.instances.lock().unwrap();
            if let Some(instance_arc) = instances.get(config.id()) {
                // Update existing instance with new configuration
                let mut instance = instance_arc.lock().unwrap();
                if let Err(e) = instance.update(Box::new(config.clone())) {
                    return Err(WidgetFactoryError::ConfigError(
                        format!("Failed to update cached widget: {}", e)
                    ));
                }
                
                // Update performance metrics
                drop(instance);
                drop(instances);
                let mut perf_stats = self.performance_stats.lock().unwrap();
                if let Some(metrics) = perf_stats.get_mut(config.id()) {
                    metrics.update_count += 1;
                    metrics.last_render_at = std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_millis() as u64;
                }
                return Ok(());
            }
        }
        
        // Get builder and create instance
        let widget_id = config.id().to_string();
        let _widget_type_str = config.widget_type().to_string();
        let start_time = std::time::Instant::now();
        
        // Create widget using registered builder
        let builders = self.builders.read().unwrap();
        let builder_any = builders.get(widget_type)
            .ok_or_else(|| WidgetFactoryError::UnknownType(
                widget_type.to_string()
            ))?;
        
        // Safe downcast - we control both registration and usage
        let builder = builder_any.downcast_ref::<WidgetBuilder<C>>()
            .ok_or_else(|| WidgetFactoryError::CreationFailed(
                format!("Widget builder type mismatch for: {}", widget_type)
            ))?;
        
        let instance = builder(config.clone());
        let creation_time = start_time.elapsed();
        
        // Record performance metrics
        {
            let mut perf_stats = self.performance_stats.lock().unwrap();
            perf_stats.insert(widget_id.clone(), WidgetMetrics {
                render_time_ms: creation_time.as_secs_f64() * 1000.0,
                update_count: 0,
                last_render_at: std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
                memory_usage_bytes: std::mem::size_of_val(&*instance),
            });
        }
        
        // Cache instance if requested
        if options.use_cache {
            let mut instances = self.instances.lock().unwrap();
            instances.insert(widget_id, Arc::new(Mutex::new(instance)));
        }
        
        Ok(())
    }
    
    /// Check if widget instance exists
    pub fn has_instance(&self, id: &str) -> bool {
        let instances = self.instances.lock().unwrap();
        instances.contains_key(id)
    }
    
    /// Get widget instance by ID
    pub fn get_instance(&self, id: &str) -> Option<Arc<Mutex<Box<dyn WidgetInstance>>>> {
        let instances = self.instances.lock().unwrap();
        instances.get(id).cloned()
    }
    
    /// List all widget instances
    pub fn list_instances(&self, widget_type: Option<&str>) -> Vec<String> {
        let instances = self.instances.lock().unwrap();
        instances
            .iter()
            .filter(|(_, instance_arc)| {
                widget_type.map_or(true, |t| {
                    let instance = instance_arc.lock().unwrap();
                    instance.widget_type() == t
                })
            })
            .map(|(id, _)| id.clone())
            .collect()
    }
    
    /// Update widget instance
    pub fn update_widget<T: 'static + Send + Sync>(
        &self,
        id: &str,
        updates: T,
    ) -> std::result::Result<(), WidgetFactoryError> {
        let instances = self.instances.lock().unwrap();
        if let Some(instance_arc) = instances.get(id) {
            let mut instance = instance_arc.lock().unwrap();
            instance.update(Box::new(updates))
                .map_err(|e| WidgetFactoryError::ConfigError(e.to_string()))?;
            
            // Update metrics
            drop(instance);
            drop(instances);
            let mut perf_stats = self.performance_stats.lock().unwrap();
            if let Some(metrics) = perf_stats.get_mut(id) {
                metrics.update_count += 1;
                metrics.last_render_at = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
            }
            Ok(())
        } else {
            Err(WidgetFactoryError::NotFound(id.to_string()))
        }
    }
    
    /// Destroy widget instance
    pub fn destroy_widget(&self, id: &str) -> bool {
        let mut instances = self.instances.lock().unwrap();
        if let Some(instance_arc) = instances.remove(id) {
            let mut instance = instance_arc.lock().unwrap();
            instance.destroy();
            drop(instance);
            
            // Clean up performance stats
            let mut perf_stats = self.performance_stats.lock().unwrap();
            perf_stats.remove(id);
            true
        } else {
            false
        }
    }
    
    /// Clear all cached instances
    pub fn clear_cache(&self) {
        let mut instances = self.instances.lock().unwrap();
        for (_id, instance_arc) in instances.drain() {
            let mut instance = instance_arc.lock().unwrap();
            instance.destroy();
            drop(instance);
        }
        
        // Clear performance stats too
        let mut perf_stats = self.performance_stats.lock().unwrap();
        perf_stats.clear();
    }
    
    /// Create multiple widgets in batch
    pub fn create_batch<C: WidgetConfig + 'static>(
        &self,
        widgets: Vec<(String, C)>,
        options: Option<BatchCreateOptions>,
    ) -> std::result::Result<Vec<String>, Vec<BatchError>> {
        let opts = options.unwrap_or_default();
        let mut results = Vec::new();
        let mut errors = Vec::new();
        
        for (index, (widget_type, config)) in widgets.into_iter().enumerate() {
            let widget_id = config.id().to_string();
            match self.create_widget(config, opts.create_options.clone()) {
                Ok(()) => results.push(widget_id),
                Err(e) => {
                    errors.push(BatchError {
                        index,
                        widget_type,
                        error: e.to_string(),
                    });
                    if !opts.continue_on_error {
                        return if results.is_empty() {
                            Err(errors)
                        } else {
                            // Return partial results with errors
                            Err(errors)
                        };
                    }
                }
            }
        }
        
        if errors.is_empty() {
            Ok(results)
        } else if opts.continue_on_error && !results.is_empty() {
            // Log errors but return successes
            eprintln!("Batch creation completed with {} errors", errors.len());
            Ok(results)
        } else {
            Err(errors)
        }
    }
    
    /// Get factory statistics
    pub fn get_stats(&self) -> FactoryStats {
        let instances = self.instances.lock().unwrap();
        let schemas = self.schemas.read().unwrap();
        
        FactoryStats {
            registered_types: self.get_types(),
            total_instances: instances.len(),
            total_schemas: schemas.len(),
            cache_stats: self.get_cache_stats(),
        }
    }
    
    /// Get cache statistics
    pub fn get_cache_stats(&self) -> CacheStats {
        let instances = self.instances.lock().unwrap();
        let mut type_counts = HashMap::new();
        let total_memory = 0;
        
        for instance_arc in instances.values() {
            let instance = instance_arc.lock().unwrap();
            let widget_type = instance.widget_type();
            *type_counts.entry(widget_type.to_string()).or_insert(0) += 1;
            // get_metrics doesn't exist on WidgetInstance trait, so we'll skip memory tracking
            // total_memory += instance.get_metrics().memory_usage_bytes;
        }
        
        CacheStats {
            total_instances: instances.len(),
            type_distribution: type_counts,
            memory_usage_bytes: total_memory,
        }
    }
    
    /// Validate configuration against schema
    fn validate_against_schema<C: WidgetConfig>(
        &self,
        config: &C,
        schema: &WidgetSchema,
    ) -> std::result::Result<(), ValidationError> {
        // First run config's own validation
        config.validate()?;
        
        // Check required fields presence
        for field in &schema.required_fields {
            match field.as_str() {
                "id" if config.id().is_empty() => {
                    return Err(ValidationError::MissingField("id".to_string()));
                }
                "type" if config.widget_type().is_empty() => {
                    return Err(ValidationError::MissingField("type".to_string()));
                }
                _ => {} // Other fields handled by widget-specific validation
            }
        }
        
        Ok(())
    }
}

/// Widget creation options
#[derive(Debug, Clone)]
pub struct CreateWidgetOptions {
    pub validate_config: bool,
    pub use_cache: bool,
    pub log_warnings: bool,
}

impl Default for CreateWidgetOptions {
    fn default() -> Self {
        Self {
            validate_config: true,
            use_cache: true,
            log_warnings: true,
        }
    }
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub total_instances: usize,
    pub type_distribution: HashMap<String, usize>,
    pub memory_usage_bytes: usize,
}

/// Batch creation options
#[derive(Debug, Clone)]
pub struct BatchCreateOptions {
    pub continue_on_error: bool,
    pub create_options: CreateWidgetOptions,
}

impl Default for BatchCreateOptions {
    fn default() -> Self {
        Self {
            continue_on_error: false,
            create_options: CreateWidgetOptions::default(),
        }
    }
}

/// Batch error information
#[derive(Debug, Clone)]
pub struct BatchError {
    pub index: usize,
    pub widget_type: String,
    pub error: String,
}

/// Factory statistics
#[derive(Debug, Clone)]
pub struct FactoryStats {
    pub registered_types: Vec<String>,
    pub total_instances: usize,
    pub total_schemas: usize,
    pub cache_stats: CacheStats,
}

/// Global widget factory instance
pub static WIDGET_FACTORY: Lazy<Arc<WidgetRegistry>> = Lazy::new(|| Arc::new(WidgetRegistry::new()));

/// Convenience function for creating widgets
pub fn create_widget<C: WidgetConfig + 'static>(
    config: C,
    options: Option<CreateWidgetOptions>,
) -> std::result::Result<(), WidgetFactoryError> {
    WIDGET_FACTORY.create_widget(config, options.unwrap_or_default())
}

/// Update a widget by ID
pub fn update_widget<T: 'static + Send + Sync>(
    id: &str,
    updates: T,
) -> std::result::Result<(), WidgetFactoryError> {
    WIDGET_FACTORY.update_widget(id, updates)
}

/// Destroy a widget by ID
pub fn destroy_widget(id: &str) -> bool {
    WIDGET_FACTORY.destroy_widget(id)
}

/// Get widget instance by ID
pub fn get_instance(id: &str) -> Option<Arc<Mutex<Box<dyn WidgetInstance>>>> {
    WIDGET_FACTORY.get_instance(id)
}

/// List all widget instances
pub fn list_instances(widget_type: Option<&str>) -> Vec<String> {
    WIDGET_FACTORY.list_instances(widget_type)
}

/// Create multiple widgets in batch
pub fn create_batch<C: WidgetConfig + 'static>(
    widgets: Vec<(String, C)>,
    options: Option<BatchCreateOptions>,
) -> std::result::Result<Vec<String>, Vec<BatchError>> {
    WIDGET_FACTORY.create_batch(widgets, options)
}

/// Get factory statistics
pub fn get_stats() -> FactoryStats {
    WIDGET_FACTORY.get_stats()
}

/// Clear widget cache
pub fn clear_cache() {
    WIDGET_FACTORY.clear_cache()
}

/// Convenience function for registering widget types
pub fn register_widget<C: WidgetConfig + 'static>(
    widget_type: String,
    builder: WidgetBuilder<C>,
    schema: WidgetSchema,
) -> Result<()> {
    WIDGET_FACTORY.register(widget_type, builder, schema)
}

/// Macro for easy widget registration
#[macro_export]
macro_rules! register_widget_type {
    ($widget_type:literal, $config_type:ty, $widget_class:ty) => {
        {
            use $crate::widgets::factory::{register_widget, WidgetBuilder, WidgetSchema};
            use std::collections::HashMap;
            
            let builder: WidgetBuilder<$config_type> = Box::new(|config| {
                Box::new(<$widget_class>::new(config))
            });
            
            let schema = WidgetSchema {
                widget_type: $widget_type.to_string(),
                required_fields: vec!["id".to_string(), "type".to_string()],
                properties: HashMap::new(), // Properties will be populated by widget implementations
                version: "1.0.0".to_string(),
            };
            
            register_widget($widget_type.to_string(), builder, schema)
        }
    };
}

/// Base widget implementation helpers
pub mod base {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    /// Base widget configuration
    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct BaseConfig {
        pub id: String,
        pub widget_type: String,
        pub classes: Vec<String>,
        pub attributes: HashMap<String, String>,
        pub disabled: bool,
        pub visible: bool,
        pub focusable: bool,
        pub tab_index: Option<i32>,
    }
    
    impl WidgetConfig for BaseConfig {
        fn id(&self) -> &str { &self.id }
        fn widget_type(&self) -> &str { &self.widget_type }
        fn classes(&self) -> &[String] { &self.classes }
        fn attributes(&self) -> &HashMap<String, String> { &self.attributes }
        fn disabled(&self) -> bool { self.disabled }
        fn visible(&self) -> bool { self.visible }
        fn focusable(&self) -> bool { self.focusable }
        fn tab_index(&self) -> Option<i32> { self.tab_index }
    }
    
    /// Base widget implementation
    pub struct BaseWidget {
        config: BaseConfig,
        element: Option<Element>,
        destroyed: bool,
        metrics: WidgetMetrics,
        #[allow(dead_code)]
        last_update: u64,
    }
    
    impl BaseWidget {
        pub fn new(config: BaseConfig) -> Self {
            Self {
                config,
                element: None,
                destroyed: false,
                metrics: WidgetMetrics::default(),
                last_update: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64,
            }
        }
        
        pub fn config(&self) -> &BaseConfig {
            &self.config
        }
        
        pub fn is_destroyed(&self) -> bool {
            self.destroyed
        }
        
        pub fn needs_rerender(&self) -> bool {
            self.element.is_none() || self.config_changed()
        }
        
        fn config_changed(&self) -> bool {
            // Check if element needs re-rendering based on last update time
            let current_time = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            // Re-render if more than 16ms have passed (60 FPS)
            current_time - self.last_update > 16
        }
        
        /// Create base element with common styling
        pub fn create_base_element(&self) -> crate::components::Element {
            use crate::widgets::div;
            
            let mut classes = vec![
                format!("widget-{}", self.config.widget_type),
                format!("widget-id-{}", self.config.id),
            ];
            classes.extend(self.config.classes.clone());
            
            if self.config.disabled {
                classes.push("widget-disabled".to_string());
            }
            if !self.config.visible {
                classes.push("widget-hidden".to_string());
            }
            if self.config.focusable {
                classes.push("widget-focusable".to_string());
            }
            
            let mut element = div()
                .id(&self.config.id);
                
            for class in &classes {
                element = element.class(class);
            }
            
            for (key, value) in &self.config.attributes {
                element = element.attr(key, value);
            }
            
            if self.config.focusable {
                element = element.focusable(true);
                if let Some(tab_index) = self.config.tab_index {
                    element = element.tab_index(tab_index);
                }
            }
            
            element.build()
        }
    }
    
    impl WidgetInstance for BaseWidget {
        fn id(&self) -> &str {
            &self.config.id
        }
        
        fn widget_type(&self) -> &str {
            &self.config.widget_type
        }
        
        fn render(&mut self) -> Result<Element> {
            if self.destroyed {
                return Err(crate::error::TuiError::component(
                    format!("Cannot render destroyed widget: {}", self.id())
                ));
            }
            
            let start_time = std::time::Instant::now();
            
            if self.needs_rerender() {
                self.element = Some(self.create_base_element());
            }
            
            self.metrics.render_time_ms = start_time.elapsed().as_secs_f64() * 1000.0;
            self.metrics.last_render_at = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            
            Ok(self.element.as_ref().unwrap().clone())
        }
        
        fn update(&mut self, _updates: Box<dyn std::any::Any + Send + Sync>) -> Result<()> {
            if self.destroyed {
                return Err(crate::error::TuiError::component(
                    format!("Cannot update destroyed widget: {}", self.id())
                ));
            }
            
            // Update metrics and tracking
            self.metrics.update_count += 1;
            self.last_update = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;
            self.element = None; // Force re-render
            
            Ok(())
        }
        
        fn destroy(&mut self) {
            if self.destroyed {
                return;
            }
            
            self.destroyed = true;
            self.element = None;
        }
        
        fn validate(&self) -> bool {
            !self.destroyed && 
            !self.config.id.is_empty() && 
            !self.config.widget_type.is_empty()
        }
        
        fn needs_rerender(&self) -> bool {
            BaseWidget::needs_rerender(self)
        }
        
        fn get_metrics(&self) -> WidgetMetrics {
            self.metrics.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::base::*;
    
    #[test]
    fn test_widget_registry_creation() {
        let registry = WidgetRegistry::new();
        assert_eq!(registry.get_types().len(), 0);
    }
    
    #[test]
    fn test_base_widget() {
        let config = BaseConfig {
            id: "test-widget".to_string(),
            widget_type: "test".to_string(),
            classes: vec!["test-class".to_string()],
            attributes: HashMap::new(),
            disabled: false,
            visible: true,
            focusable: true,
            tab_index: Some(1),
        };
        
        let mut widget = BaseWidget::new(config);
        assert_eq!(widget.id(), "test-widget");
        assert_eq!(widget.widget_type(), "test");
        assert!(!widget.is_destroyed());
        assert!(widget.validate());
        
        // Test render
        let result = widget.render();
        assert!(result.is_ok());
        
        // Test destroy
        widget.destroy();
        assert!(widget.is_destroyed());
        assert!(!widget.validate());
    }
    
    #[test]
    fn test_factory_stats() {
        // Create a local registry instead of using the global one
        let registry = WidgetRegistry::new();
        let stats = registry.get_stats();
        assert_eq!(stats.registered_types.len(), 0);
        assert_eq!(stats.total_instances, 0);
        assert_eq!(stats.total_schemas, 0);
    }
    
    #[test]
    fn test_cache_operations() {
        let registry = WidgetRegistry::new();
        
        // Initial state
        let stats = registry.get_cache_stats();
        assert_eq!(stats.total_instances, 0);
        
        // Clear cache should not panic on empty registry
        registry.clear_cache();
        let stats_after = registry.get_cache_stats();
        assert_eq!(stats_after.total_instances, 0);
    }
    
    #[test]
    fn test_list_instances() {
        let registry = WidgetRegistry::new();
        
        // Should return empty lists
        let all_instances = registry.list_instances(None);
        let button_instances = registry.list_instances(Some("button"));
        
        assert_eq!(all_instances.len(), 0);
        assert_eq!(button_instances.len(), 0);
    }
}