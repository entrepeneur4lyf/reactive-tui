//! # Reactive State Management
//!
//! Thread-safe reactive state system with automatic change detection and UI updates.
//!
//! This module provides a React-like state management system for terminal applications,
//! enabling automatic UI updates when state changes. The reactive system uses Arc/RwLock
//! for thread safety and broadcast channels for efficient change notifications.
//!
//! ## Features
//!
//! - **Automatic Updates**: UI components re-render when reactive state changes
//! - **Thread Safety**: Arc/RwLock-based shared state across threads
//! - **Change Watchers**: Register callbacks for specific state changes
//! - **Broadcast Events**: Efficient notification system for multiple subscribers
//! - **Field-Level Granularity**: Track changes to specific fields within structs
//! - **Type Safety**: Generic system with compile-time type checking
//!
//! ## Core Components
//!
//! - [`Reactive<T>`](Reactive): A reactive value container with change notifications
//! - [`ReactiveState`]: JSON-based state management for complex data
//! - [`ReactiveComponent`]: Component trait for reactive UI elements
//! - [`ReactiveStruct`]: Derive macro for automatic reactivity
//!
//! ## Examples
//!
//! ### Basic Reactive Value
//!
//! ```rust,no_run
//! use reactive_tui::reactive::Reactive;
//!
//! // Create reactive counter
//! let counter = Reactive::new(0);
//!
//! // Add watcher for changes
//! counter.watch(|old_val, new_val| {
//!     println!("Counter changed from {} to {}", old_val, new_val);
//! });
//!
//! // Update value (triggers watcher)
//! counter.set(1);
//! ```
//!
//! ### Component with Reactive State
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::reactive::*;
//!
//! struct CounterComponent {
//!     count: Reactive<i32>,
//! }
//!
//! impl CounterComponent {
//!     fn new() -> Self {
//!         Self {
//!             count: Reactive::new(0),
//!         }
//!     }
//!
//!     fn increment(&self) {
//!         let current = self.count.get();
//!         self.count.set(current + 1);
//!     }
//! }
//!
//! impl Component for CounterComponent {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div")
//!             .class("counter")
//!             .content(&format!("Count: {}", self.count.get()))
//!             .build()
//!     }
//! }
//! ```
//!
//! ### Complex State with JSON
//!
//! ```rust,no_run
//! use reactive_tui::reactive::ReactiveState;
//! use serde_json::json;
//!
//! // Create complex state
//! let state = ReactiveState::new();
//! state.set_state_json(&json!({
//!     "user": {
//!         "name": "John",
//!         "email": "john@example.com",
//!         "preferences": {
//!             "theme": "dark",
//!             "notifications": true
//!         }
//!     },
//!     "app": {
//!         "version": "1.0.0",
//!         "debug": false
//!     }
//! })).unwrap();
//!
//! // Watch for specific field changes
//! state.watch_field("user.name", |old, new| {
//!     println!("User name changed from {:?} to {:?}", old, new);
//! });
//!
//! // Update nested values
//! state.set_field("user.preferences.theme", "light".to_string());
//! ```
//!
//! ### Reactive Struct Implementation
//!
//! ```rust,no_run
//! use reactive_tui::reactive::{Reactive, ReactiveState, ReactiveStruct};
//! use reactive_tui::error::Result;
//!
//! struct AppSettings {
//!     theme: Reactive<String>,
//!     font_size: Reactive<u16>,
//!     auto_save: Reactive<bool>,
//!     state: ReactiveState,
//! }
//!
//! impl AppSettings {
//!     fn new() -> Self {
//!         Self {
//!             theme: Reactive::new("dark".to_string()),
//!             font_size: Reactive::new(14),
//!             auto_save: Reactive::new(true),
//!             state: ReactiveState::new(),
//!         }
//!     }
//!
//!     fn watch_theme<F>(&self, watcher: F)
//!     where
//!         F: Fn(&String, &String) + Send + Sync + 'static,
//!     {
//!         self.theme.watch(watcher);
//!     }
//! }
//!
//! impl ReactiveStruct for AppSettings {
//!     fn init_reactive(&mut self) {
//!         // Initialize watchers for reactive fields
//!     }
//!     fn reactive_state(&self) -> &ReactiveState { &self.state }
//!     fn reactive_state_mut(&mut self) -> &mut ReactiveState { &mut self.state }
//!     fn sync_to_state(&mut self) -> Result<()> { Ok(()) }
//!     fn load_from_state(&mut self) -> Result<()> { Ok(()) }
//! }
//!
//! let settings = AppSettings::new();
//! settings.watch_theme(|old, new| {
//!     println!("Theme changed: {} -> {}", old, new);
//! });
//! ```

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::broadcast;

/// Trait for objects that can be watched for changes
pub trait Watchable<T> {
  /// Called when the value changes
  fn on_change(&mut self, old_value: T, new_value: T);
}

/// A reactive value that notifies watchers when it changes
#[derive(Clone)]
pub struct Reactive<T: Clone + PartialEq + Send + Sync + 'static> {
  value: Arc<RwLock<T>>,
  watchers: Arc<RwLock<Vec<ReactiveWatcher<T>>>>,
  sender: broadcast::Sender<ReactiveChange<T>>,
}

/// Change notification for reactive values
#[derive(Debug, Clone)]
pub struct ReactiveChange<T> {
  pub old_value: T,
  pub new_value: T,
  pub field_name: String,
}

/// Watcher callback for reactive value changes
pub type ReactiveWatcher<T> = Box<dyn Fn(&T, &T) + Send + Sync>;

impl<T: Clone + PartialEq + Send + Sync + 'static> Reactive<T> {
  /// Create a new reactive value
  pub fn new(initial_value: T) -> Self {
    let (sender, _) = broadcast::channel(100);

    Self {
      value: Arc::new(RwLock::new(initial_value)),
      watchers: Arc::new(RwLock::new(Vec::new())),
      sender,
    }
  }

  /// Get the current value
  pub fn get(&self) -> T {
    self.value.read().unwrap().clone()
  }

  /// Set a new value, triggering watchers if changed
  pub fn set(&self, new_value: T) {
    let old_value = {
      let mut current = self.value.write().unwrap();
      let old = current.clone();

      if *current != new_value {
        *current = new_value.clone();
        old
      } else {
        return; // No change, don't trigger watchers
      }
    };

    // Notify watchers
    let watchers = self.watchers.read().unwrap();
    for watcher in watchers.iter() {
      watcher(&old_value, &new_value);
    }

    // Send broadcast notification
    let change = ReactiveChange {
      old_value,
      new_value,
      field_name: "value".to_string(),
    };
    let _ = self.sender.send(change);
  }

  /// Add a watcher function that's called when the value changes
  pub fn watch<F>(&self, watcher: F)
  where
    F: Fn(&T, &T) + Send + Sync + 'static,
  {
    let mut watchers = self.watchers.write().unwrap();
    watchers.push(Box::new(watcher));
  }

  /// Subscribe to changes via broadcast channel
  pub fn subscribe(&self) -> broadcast::Receiver<ReactiveChange<T>> {
    self.sender.subscribe()
  }

  /// Update the value using a closure
  pub fn update<F>(&self, updater: F)
  where
    F: FnOnce(&mut T),
  {
    let old_value = {
      let mut current = self.value.write().unwrap();
      let old = current.clone();
      updater(&mut *current);
      old
    };

    let new_value = self.get();

    if old_value != new_value {
      // Notify watchers
      let watchers = self.watchers.read().unwrap();
      for watcher in watchers.iter() {
        watcher(&old_value, &new_value);
      }

      // Send broadcast notification
      let change = ReactiveChange {
        old_value,
        new_value,
        field_name: "value".to_string(),
      };
      let _ = self.sender.send(change);
    }
  }
}

/// Type alias for complex watcher function type
pub type WatcherFn = Box<dyn Fn(&dyn std::any::Any, &dyn std::any::Any) + Send + Sync>;
/// Type alias for watchers map to reduce complexity
pub type WatchersMap = HashMap<String, Vec<WatcherFn>>;

/// Reactive state container for managing application state
pub struct ReactiveState {
  fields: Arc<RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>>,
  watchers: Arc<RwLock<WatchersMap>>,
  change_sender: broadcast::Sender<StateChange>,
}

/// Change notification for reactive state
#[derive(Debug, Clone)]
pub struct StateChange {
  pub field_name: String,
  pub timestamp: std::time::Instant,
}

impl ReactiveState {
  /// Create a new reactive state container
  pub fn new() -> Self {
    let (sender, _) = broadcast::channel(100);

    Self {
      fields: Arc::new(RwLock::new(HashMap::new())),
      watchers: Arc::new(RwLock::new(HashMap::new())),
      change_sender: sender,
    }
  }

  /// Set state from JSON value, supporting nested object structures
  pub fn set_state_json(&self, json_value: &serde_json::Value) -> crate::error::Result<()> {
    self.set_state_json_recursive("", json_value)
  }

  /// Recursively set nested JSON values with dot notation keys
  fn set_state_json_recursive(
    &self,
    prefix: &str,
    value: &serde_json::Value,
  ) -> crate::error::Result<()> {
    match value {
      serde_json::Value::Object(map) => {
        for (key, val) in map {
          let field_name = if prefix.is_empty() {
            key.clone()
          } else {
            format!("{prefix}.{key}")
          };
          self.set_state_json_recursive(&field_name, val)?;
        }
      }
      serde_json::Value::String(s) => {
        self.set_field(prefix, s.clone());
      }
      serde_json::Value::Number(n) => {
        if let Some(i) = n.as_i64() {
          self.set_field(prefix, i);
        } else if let Some(f) = n.as_f64() {
          self.set_field(prefix, f);
        }
      }
      serde_json::Value::Bool(b) => {
        self.set_field(prefix, *b);
      }
      serde_json::Value::Array(arr) => {
        // Store arrays as JSON strings for now
        self.set_field(
          prefix,
          serde_json::to_string(arr).map_err(|e| {
            crate::error::TuiError::component(format!("Failed to serialize array: {e}"))
          })?,
        );
      }
      serde_json::Value::Null => {
        // Skip null values
      }
    }
    Ok(())
  }

  /// Get state as JSON value, reconstructing nested objects from dot notation
  pub fn get_state_json(&self) -> crate::error::Result<serde_json::Value> {
    let fields = self.fields.read().map_err(|_| {
      crate::error::TuiError::component("Failed to acquire fields lock".to_string())
    })?;

    let mut result = serde_json::Map::new();

    for (key, value) in fields.iter() {
      self.insert_nested_value(&mut result, key, value)?;
    }

    Ok(serde_json::Value::Object(result))
  }

  /// Insert a value into nested JSON structure using dot notation
  fn insert_nested_value(
    &self,
    target: &mut serde_json::Map<String, serde_json::Value>,
    key: &str,
    value: &Box<dyn std::any::Any + Send + Sync>,
  ) -> crate::error::Result<()> {
    let parts: Vec<&str> = key.split('.').collect();
    self.insert_nested_value_recursive(target, &parts, value)
  }

  /// Recursive helper for inserting nested values
  fn insert_nested_value_recursive(
    &self,
    target: &mut serde_json::Map<String, serde_json::Value>,
    parts: &[&str],
    value: &Box<dyn std::any::Any + Send + Sync>,
  ) -> crate::error::Result<()> {
    if parts.is_empty() {
      return Ok(());
    }

    if parts.len() == 1 {
      // Base case: insert the value
      let json_value = self.any_to_json_value(value)?;
      target.insert(parts[0].to_string(), json_value);
      return Ok(());
    }

    // Recursive case: navigate to next level
    let current_key = parts[0].to_string();
    let remaining_parts = &parts[1..];

    // Ensure the key exists and is an object
    let entry = target
      .entry(current_key)
      .or_insert_with(|| serde_json::Value::Object(serde_json::Map::new()));

    // Ensure it's an object, replace if not
    if !matches!(entry, serde_json::Value::Object(_)) {
      *entry = serde_json::Value::Object(serde_json::Map::new());
    }

    // Recurse into the nested object
    if let serde_json::Value::Object(nested_map) = entry {
      self.insert_nested_value_recursive(nested_map, remaining_parts, value)
    } else {
      Ok(())
    }
  }

  /// Convert Any value to JSON Value
  fn any_to_json_value(
    &self,
    value: &Box<dyn std::any::Any + Send + Sync>,
  ) -> crate::error::Result<serde_json::Value> {
    // Try common types
    if let Some(s) = value.downcast_ref::<String>() {
      Ok(serde_json::Value::String(s.clone()))
    } else if let Some(i) = value.downcast_ref::<i32>() {
      Ok(serde_json::Value::Number(serde_json::Number::from(*i)))
    } else if let Some(i) = value.downcast_ref::<i64>() {
      Ok(serde_json::Value::Number(serde_json::Number::from(*i)))
    } else if let Some(f) = value.downcast_ref::<f64>() {
      Ok(serde_json::Value::Number(
        serde_json::Number::from_f64(*f).unwrap_or(serde_json::Number::from(0)),
      ))
    } else if let Some(b) = value.downcast_ref::<bool>() {
      Ok(serde_json::Value::Bool(*b))
    } else {
      // Fallback to string representation
      Ok(serde_json::Value::String(format!("{value:?}")))
    }
  }

  /// Set a reactive field value
  pub fn set_field<T>(&self, name: &str, value: T)
  where
    T: Clone + PartialEq + Send + Sync + 'static,
  {
    let old_value = {
      let mut fields = self.fields.write().unwrap();
      fields.insert(name.to_string(), Box::new(value.clone()))
    };

    // Trigger watchers for this field
    let watchers = self.watchers.read().unwrap();
    if let Some(field_watchers) = watchers.get(name) {
      for watcher in field_watchers {
        if let Some(old_any) = old_value.as_ref() {
          watcher(old_any.as_ref(), &value);
        }
      }
    }

    // Send change notification
    let change = StateChange {
      field_name: name.to_string(),
      timestamp: std::time::Instant::now(),
    };
    let _ = self.change_sender.send(change);
  }

  /// Get a reactive field value
  pub fn get_field<T>(&self, name: &str) -> Option<T>
  where
    T: Clone + 'static,
  {
    let fields = self.fields.read().unwrap();
    fields
      .get(name)
      .and_then(|any| any.downcast_ref::<T>())
      .cloned()
  }

  /// Watch for changes to a specific field
  pub fn watch_field<F>(&self, field_name: &str, watcher: F)
  where
    F: Fn(&dyn std::any::Any, &dyn std::any::Any) + Send + Sync + 'static,
  {
    let mut watchers = self.watchers.write().unwrap();
    watchers
      .entry(field_name.to_string())
      .or_default()
      .push(Box::new(watcher));
  }

  /// Subscribe to all state changes
  pub fn subscribe(&self) -> broadcast::Receiver<StateChange> {
    self.change_sender.subscribe()
  }

  /// Get read access to the fields map
  pub fn fields(&self) -> &Arc<RwLock<HashMap<String, Box<dyn std::any::Any + Send + Sync>>>> {
    &self.fields
  }

  /// Get the change sender for sending state change notifications
  pub fn change_sender(&self) -> &broadcast::Sender<StateChange> {
    &self.change_sender
  }
}

impl Default for ReactiveState {
  fn default() -> Self {
    Self::new()
  }
}

/// Trait for structs that contain reactive state and can initialize reactive watchers
/// This provides a standard interface for reactive components
pub trait ReactiveStruct {
  /// Initialize reactive watchers and setup automatic state synchronization
  fn init_reactive(&mut self);

  /// Get the reactive state container for advanced state management
  fn reactive_state(&self) -> &ReactiveState;

  /// Get mutable access to the reactive state container
  fn reactive_state_mut(&mut self) -> &mut ReactiveState;

  /// Sync all reactive values to the state container
  fn sync_to_state(&mut self) -> crate::error::Result<()>;

  /// Load all reactive values from the state container
  fn load_from_state(&mut self) -> crate::error::Result<()>;
}

/// Convenience macro for creating reactive values with watchers
#[macro_export]
macro_rules! reactive {
  ($initial:expr) => {
    $crate::reactive::Reactive::new($initial)
  };

  ($initial:expr, $watcher:expr) => {{
    let reactive = $crate::reactive::Reactive::new($initial);
    reactive.watch($watcher);
    reactive
  }};
}

/// Macro to implement ReactiveStruct for structs with a ReactiveState field
/// This simplifies the boilerplate for reactive components
///
/// # Example
/// ```rust,no_run
/// use reactive_tui::reactive::{Reactive, ReactiveState, ReactiveStruct};
/// use reactive_tui::error::Result;
///
/// struct MyComponent {
///     name: Reactive<String>,
///     count: Reactive<i32>,
///     enabled: Reactive<bool>,
///     state: ReactiveState,
/// }
///
/// impl MyComponent {
///     fn new() -> Self {
///         Self {
///             name: Reactive::new("default".to_string()),
///             count: Reactive::new(0),
///             enabled: Reactive::new(true),
///             state: ReactiveState::new(),
///         }
///     }
/// }
///
/// // Implement ReactiveStruct manually
/// impl ReactiveStruct for MyComponent {
///     fn init_reactive(&mut self) {
///         // Initialize reactive field watchers
///     }
///     fn reactive_state(&self) -> &ReactiveState { &self.state }
///     fn reactive_state_mut(&mut self) -> &mut ReactiveState { &mut self.state }
///     fn sync_to_state(&mut self) -> Result<()> { Ok(()) }
///     fn load_from_state(&mut self) -> Result<()> { Ok(()) }
/// }
/// ```
#[macro_export]
macro_rules! impl_reactive_struct {
    ($struct_name:ident, $state_field:ident, $($field:ident : $field_type:ty),*) => {
        impl $crate::reactive::ReactiveStruct for $struct_name {
            fn init_reactive(&mut self) {
                // Initialize watchers for reactive fields
                $(
                    let field_name = stringify!($field);
                    self.$field.watch({
                        let state = self.$state_field.clone();
                        move |_old, new| {
                            let _ = state.set_field(field_name, new.clone());
                        }
                    });
                )*
            }

            fn reactive_state(&self) -> &$crate::reactive::ReactiveState {
                &self.$state_field
            }

            fn reactive_state_mut(&mut self) -> &mut $crate::reactive::ReactiveState {
                &mut self.$state_field
            }

            fn sync_to_state(&mut self) -> $crate::error::Result<()> {
                $(
                    let field_name = stringify!($field);
                    self.$state_field.set_field(field_name, self.$field.get());
                )*
                Ok(())
            }

            fn load_from_state(&mut self) -> $crate::error::Result<()> {
                $(
                    let field_name = stringify!($field);
                    if let Some(value) = self.$state_field.get_field::<$field_type>(field_name) {
                        self.$field.set(value);
                    }
                )*
                Ok(())
            }
        }
    };
}

/// Example reactive component implementation
pub struct ReactiveComponent {
  pub numbers: Reactive<String>,
  pub count: Reactive<i32>,
  pub show_button: Reactive<bool>,
  state: ReactiveState,
}

impl ReactiveComponent {
  pub fn new() -> Self {
    let numbers = Reactive::new("0".to_string());
    let count = Reactive::new(0);
    let show_button = Reactive::new(true);

    // Example watchers
    numbers.watch(|old, new| {
      println!("Numbers changed from '{old}' to '{new}'");
    });

    count.watch(|old, new| {
      println!("Count changed from {old} to {new}");
    });

    show_button.watch(|old, new| {
      println!("Show button changed from {old} to {new}");
    });

    Self {
      numbers,
      count,
      show_button,
      state: ReactiveState::new(),
    }
  }

  /// Get access to the reactive state for advanced state management
  pub fn state(&self) -> &ReactiveState {
    &self.state
  }

  /// Get mutable access to the reactive state
  pub fn state_mut(&mut self) -> &mut ReactiveState {
    &mut self.state
  }

  /// Update multiple reactive values atomically using state management
  pub fn batch_update<F>(&mut self, updater: F) -> crate::error::Result<()>
  where
    F: FnOnce(&mut Self) -> crate::error::Result<()>,
  {
    // Store original values for rollback if needed
    let original_numbers = self.numbers.get();
    let original_count = self.count.get();
    let original_show_button = self.show_button.get();

    match updater(self) {
      Ok(()) => Ok(()),
      Err(e) => {
        // Rollback on error
        self.numbers.set(original_numbers);
        self.count.set(original_count);
        self.show_button.set(original_show_button);
        Err(e)
      }
    }
  }

  /// Sync reactive values with the state manager
  pub fn sync_with_state(&mut self) -> crate::error::Result<()> {
    // For now, just store values directly in fields until ReactiveState has set/get methods
    let mut fields = self.state.fields.write().map_err(|_| {
      crate::error::TuiError::component("Failed to acquire fields lock".to_string())
    })?;

    // Store values directly as boxed Any instead of using JSON serialization
    fields.insert("numbers".to_string(), Box::new(self.numbers.get().clone()));
    fields.insert("count".to_string(), Box::new(self.count.get()));
    fields.insert("show_button".to_string(), Box::new(self.show_button.get()));

    Ok(())
  }

  /// Load values from state manager into reactive values
  pub fn load_from_state(&mut self) -> crate::error::Result<()> {
    let fields = self.state.fields.read().map_err(|_| {
      crate::error::TuiError::component("Failed to acquire fields lock".to_string())
    })?;

    // Use Any downcast instead of JSON deserialization
    if let Some(numbers_any) = fields.get("numbers") {
      if let Some(numbers) = numbers_any.downcast_ref::<String>() {
        self.numbers.set(numbers.clone());
      }
    }
    if let Some(count_any) = fields.get("count") {
      if let Some(count) = count_any.downcast_ref::<i32>() {
        self.count.set(*count);
      }
    }
    if let Some(show_button_any) = fields.get("show_button") {
      if let Some(show_button) = show_button_any.downcast_ref::<bool>() {
        self.show_button.set(*show_button);
      }
    }

    Ok(())
  }

  /// Computed property example with automatic dependency tracking
  pub fn computed_display(&self) -> String {
    let count = self.count.get();
    let numbers = self.numbers.get();

    if count > 0 {
      format!("{numbers} ({count})")
    } else {
      numbers
    }
  }

  /// Example method that updates multiple reactive values
  pub fn increment(&self) {
    let current = self.count.get();
    self.count.set(current + 1);
    self.numbers.set(format!("Count: {}", current + 1));

    // Show button only when count < 10
    self.show_button.set(current < 9);
  }
}

impl Default for ReactiveComponent {
  fn default() -> Self {
    Self::new()
  }
}

impl ReactiveStruct for ReactiveComponent {
  /// Initialize reactive watchers and setup automatic state synchronization
  fn init_reactive(&mut self) {
    // The watchers are already set up in the constructor,
    // but this method can be used to reinitialize or add additional watchers

    // Watch state changes and sync back to reactive values
    // For now, we just log state changes since StateChange doesn't carry the new value
    let mut state_changes = self.state.subscribe();

    tokio::spawn(async move {
      while let Ok(change) = state_changes.recv().await {
        eprintln!(
          "[Reactive] Field '{}' changed at {:?}",
          change.field_name, change.timestamp
        );
      }
    });
  }

  /// Get the reactive state container for advanced state management
  fn reactive_state(&self) -> &ReactiveState {
    &self.state
  }

  /// Get mutable access to the reactive state container
  fn reactive_state_mut(&mut self) -> &mut ReactiveState {
    &mut self.state
  }

  /// Sync all reactive values to the state container
  fn sync_to_state(&mut self) -> crate::error::Result<()> {
    self.sync_with_state()
  }

  /// Load all reactive values from the state container
  fn load_from_state(&mut self) -> crate::error::Result<()> {
    self.load_from_state()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::{Arc, Mutex};
  use std::time::Duration;
  use tokio::time::timeout;

  #[test]
  fn test_reactive_value_basic() {
    let reactive_str = Reactive::new("hello".to_string());
    assert_eq!(reactive_str.get(), "hello");

    reactive_str.set("world".to_string());
    assert_eq!(reactive_str.get(), "world");
  }

  #[test]
  fn test_reactive_watcher() {
    let reactive_int = Reactive::new(42);
    let changes = Arc::new(Mutex::new(Vec::new()));
    let changes_clone = changes.clone();

    reactive_int.watch(move |old, new| {
      changes_clone.lock().unwrap().push((*old, *new));
    });

    reactive_int.set(100);
    reactive_int.set(200);

    let recorded_changes = changes.lock().unwrap();
    assert_eq!(recorded_changes.len(), 2);
    assert_eq!(recorded_changes[0], (42, 100));
    assert_eq!(recorded_changes[1], (100, 200));
  }

  #[tokio::test]
  async fn test_reactive_subscription() {
    let reactive_str = Reactive::new("initial".to_string());
    let mut receiver = reactive_str.subscribe();

    // Set value in another task
    let reactive_clone = reactive_str.clone();
    tokio::spawn(async move {
      reactive_clone.set("changed".to_string());
    });

    // Wait for change notification
    let change = timeout(Duration::from_millis(100), receiver.recv())
      .await
      .expect("Timeout")
      .expect("Channel error");

    assert_eq!(change.old_value, "initial");
    assert_eq!(change.new_value, "changed");
  }

  #[test]
  fn test_reactive_component() {
    let component = ReactiveComponent::new();

    assert_eq!(component.count.get(), 0);
    assert_eq!(component.numbers.get(), "0");
    assert_eq!(component.computed_display(), "0");

    component.increment();

    assert_eq!(component.count.get(), 1);
    assert_eq!(component.numbers.get(), "Count: 1");
    assert_eq!(component.computed_display(), "Count: 1 (1)");
  }

  #[test]
  fn test_reactive_state() {
    let state = ReactiveState::new();

    state.set_field("name", "Alice".to_string());
    state.set_field("age", 30);

    assert_eq!(state.get_field::<String>("name"), Some("Alice".to_string()));
    assert_eq!(state.get_field::<i32>("age"), Some(30));
    assert_eq!(state.get_field::<String>("nonexistent"), None);
  }

  #[test]
  fn test_reactive_macro() {
    let reactive_val = reactive!(100);
    assert_eq!(reactive_val.get(), 100);

    let changes = Arc::new(Mutex::new(Vec::new()));
    let changes_clone = changes.clone();

    let reactive_with_watcher = reactive!(0, move |old, new| {
      changes_clone.lock().unwrap().push((*old, *new));
    });

    reactive_with_watcher.set(5);

    let recorded_changes = changes.lock().unwrap();
    assert_eq!(recorded_changes.len(), 1);
    assert_eq!(recorded_changes[0], (0, 5));
  }

  #[test]
  fn test_reactive_struct_trait() {
    let mut component = ReactiveComponent::new();

    // Test that ReactiveStruct trait methods work
    assert!(component.sync_to_state().is_ok());
    assert!(component.load_from_state().is_ok());

    // Test state access
    let state = component.reactive_state();
    assert!(state.get_field::<String>("nonexistent").is_none());

    // Test state synchronization
    component.increment();
    assert!(component.sync_to_state().is_ok());

    // Verify values were synced to state
    let state = component.reactive_state();
    assert_eq!(state.get_field::<i32>("count"), Some(1));
    assert_eq!(
      state.get_field::<String>("numbers"),
      Some("Count: 1".to_string())
    );
    assert_eq!(state.get_field::<bool>("show_button"), Some(true));
  }
}
