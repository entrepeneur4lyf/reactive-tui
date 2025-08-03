//! FFI bindings for TypeScript/Bun integration
//!
//! This module provides N-API bindings to expose the TUI framework to TypeScript/JavaScript
//! environments through Bun's FFI capabilities. Enables building TUI applications with
//! TypeScript while leveraging Rust's performance for the core framework.

#[cfg(feature = "ffi")]
use crate::{
  app::TuiApp,
  components::Element,
  driver::{DriverConfig, DriverType},
  events::actions::common as actions,
  reactive::ReactiveState,
  themes::colors::{ColorDefinition, ColorTheme},
  widgets::{Toast, ToastBuilder, ToastManager},
};
#[cfg(feature = "ffi")]
use napi_derive::napi;
#[cfg(feature = "ffi")]
use std::{
  sync::{Arc, Mutex},
  time::Duration,
};
#[cfg(feature = "ffi")]
use tokio::sync::broadcast;

#[cfg(feature = "ffi")]
pub mod exports {
  use super::*;

  /// JavaScript-compatible TUI Application wrapper
  #[napi]
  pub struct JsTuiApp {
    #[allow(dead_code)]
    app: Arc<Mutex<TuiApp>>,
    message_tx: Arc<Mutex<Option<broadcast::Sender<String>>>>,
  }

  #[napi]
  impl JsTuiApp {
    /// Create a new TUI application builder
    #[napi(constructor)]
    pub fn new() -> napi::Result<Self> {
      let config = DriverConfig {
        driver_type: Some(DriverType::Headless), // Use headless for FFI compatibility
        ..Default::default()
      };

      let app = TuiApp::builder()
        .driver_config(config)
        .build()
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;

      let (tx, _) = broadcast::channel(100);

      Ok(Self {
        app: Arc::new(Mutex::new(app)),
        message_tx: Arc::new(Mutex::new(Some(tx))),
      })
    }

    /// Set the application title
    #[napi]
    pub fn set_title(&self, title: String) -> napi::Result<()> {
      let mut app = self.app.lock().unwrap();
      app.set_title(&title);
      Ok(())
    }

    /// Load CSS stylesheet from file
    #[napi]
    pub fn load_stylesheet(&self, path: String) -> napi::Result<()> {
      use std::fs;
      let css_content = fs::read_to_string(&path)
        .map_err(|e| napi::Error::from_reason(format!("Failed to read stylesheet {path}: {e}")))?;

      let mut app = self.app.lock().unwrap();
      app
        .load_css(css_content)
        .map_err(|e| napi::Error::from_reason(format!("Failed to load CSS: {e}")))?;
      Ok(())
    }

    /// Load CSS from string
    #[napi]
    pub fn load_css(&self, css: String) -> napi::Result<()> {
      let mut app = self.app.lock().unwrap();
      app
        .load_css(css)
        .map_err(|e| napi::Error::from_reason(format!("Failed to load CSS: {e}")))?;
      Ok(())
    }

    /// Set the root component
    #[napi]
    pub fn set_component(&self, element: &JsElement) -> napi::Result<()> {
      use crate::components::Component;

      // Create a wrapper component that renders the element
      struct FfiComponent {
        element: crate::components::Element,
      }

      impl Component for FfiComponent {
        fn render(&self) -> crate::components::Element {
          self.element.clone()
        }
      }

      let mut app = self.app.lock().unwrap();
      let component = FfiComponent {
        element: element.element.clone(),
      };
      app
        .set_component(Box::new(component))
        .map_err(|e| napi::Error::from_reason(format!("Failed to set component: {e}")))?;
      Ok(())
    }

    /// Run the application
    #[napi]
    pub fn start(&self) -> napi::Result<String> {
      // Note: Running the TUI app in FFI context requires special handling
      // because it needs to manage the terminal state and event loop.
      // For headless mode, we can simulate running without blocking.

      let app = self.app.lock().unwrap();

      // Validate app is properly configured
      if !app.has_component() {
        return Err(napi::Error::from_reason(
          "No component set. Call set_component() first.",
        ));
      }

      // In headless mode, we don't actually run the event loop
      // Instead, we return a status indicating the app is ready
      Ok("TUI Application initialized in headless mode".to_string())
    }

    /// Send a message to the application
    #[napi]
    pub fn send_message(&self, message: String) -> napi::Result<()> {
      let tx_guard = self.message_tx.lock().unwrap();
      if let Some(tx) = tx_guard.as_ref() {
        tx.send(message)
          .map_err(|e| napi::Error::from_reason(format!("Failed to send message: {e}")))?;
      }
      Ok(())
    }
  }

  /// JavaScript-compatible Element wrapper
  #[napi]
  pub struct JsElement {
    element: Element,
  }

  #[napi]
  impl JsElement {
    /// Create a new element with tag
    #[napi(constructor)]
    pub fn new(tag: String) -> Self {
      Self {
        element: Element::with_tag(tag).build(),
      }
    }

    /// Add a CSS class
    #[napi]
    pub fn add_class(&mut self, class_name: String) -> napi::Result<()> {
      self.element.classes.push(class_name);
      Ok(())
    }

    /// Set element ID
    #[napi]
    pub fn set_id(&mut self, id: String) -> napi::Result<()> {
      self.element.id = Some(id);
      Ok(())
    }

    /// Set text content
    #[napi]
    pub fn set_content(&mut self, content: String) -> napi::Result<()> {
      self.element.content = Some(content);
      Ok(())
    }

    /// Add a child element
    #[napi]
    pub fn add_child(&mut self, child: &JsElement) -> napi::Result<()> {
      self.element.children.push(child.element.clone());
      Ok(())
    }

    /// Set an attribute
    #[napi]
    pub fn set_attribute(&mut self, name: String, value: String) -> napi::Result<()> {
      self.element.attributes.insert(name, value);
      Ok(())
    }

    /// Make element focusable
    #[napi]
    pub fn make_focusable(&mut self, tab_index: Option<i32>) -> napi::Result<()> {
      self.element.focusable = true;
      self.element.tab_index = tab_index;
      Ok(())
    }
  }

  /// JavaScript-compatible Toast wrapper
  #[napi]
  pub struct JsToast {
    toast: Toast,
  }

  #[napi]
  impl JsToast {
    /// Create an info toast
    #[napi(factory)]
    pub fn info(message: String) -> Self {
      Self {
        toast: ToastBuilder::info(message).build(),
      }
    }

    /// Create a success toast
    #[napi(factory)]
    pub fn success(message: String) -> Self {
      Self {
        toast: ToastBuilder::success(message).build(),
      }
    }

    /// Create a warning toast
    #[napi(factory)]
    pub fn warning(message: String) -> Self {
      Self {
        toast: ToastBuilder::warning(message).build(),
      }
    }

    /// Create an error toast
    #[napi(factory)]
    pub fn error(message: String) -> Self {
      Self {
        toast: ToastBuilder::error(message).build(),
      }
    }

    /// Set toast title
    #[napi]
    pub fn set_title(&mut self, title: String) -> napi::Result<()> {
      self.toast.title = Some(title);
      Ok(())
    }

    /// Set toast duration in milliseconds
    #[napi]
    pub fn set_duration(&mut self, duration_ms: u32) -> napi::Result<()> {
      self.toast.duration = Duration::from_millis(duration_ms as u64);
      Ok(())
    }
  }

  /// JavaScript-compatible Toast Manager
  #[napi]
  pub struct JsToastManager {
    manager: Arc<Mutex<ToastManager>>,
  }

  #[napi]
  impl JsToastManager {
    /// Create a new toast manager
    #[napi(constructor)]
    pub fn new(viewport_width: u32, viewport_height: u32) -> Self {
      Self {
        manager: Arc::new(Mutex::new(ToastManager::new(
          viewport_width as u16,
          viewport_height as u16,
        ))),
      }
    }

    /// Show a toast
    #[napi]
    pub fn show_toast(&self, toast: &JsToast) -> napi::Result<()> {
      let mut manager = self.manager.lock().unwrap();
      manager
        .show_toast(toast.toast.clone())
        .map_err(|e| napi::Error::from_reason(e.to_string()))?;
      Ok(())
    }

    /// Dismiss a toast by ID
    #[napi]
    pub fn dismiss_toast(&self, toast_id: String) -> napi::Result<bool> {
      let mut manager = self.manager.lock().unwrap();
      manager
        .dismiss_toast(&toast_id)
        .map_err(|e| napi::Error::from_reason(e.to_string()))
    }

    /// Clean up expired toasts
    #[napi]
    pub fn cleanup_expired(&self) -> Vec<String> {
      let mut manager = self.manager.lock().unwrap();
      manager.cleanup_expired()
    }
  }

  /// JavaScript-compatible Reactive State wrapper
  #[napi]
  pub struct JsReactiveState {
    state: Arc<ReactiveState>,
  }

  #[napi]
  impl JsReactiveState {
    /// Create a new reactive state
    #[napi(constructor)]
    pub fn new() -> Self {
      Self {
        state: Arc::new(ReactiveState::new()),
      }
    }
  }

  impl Default for JsReactiveState {
    fn default() -> Self {
      Self::new()
    }
  }

  #[napi]
  impl JsReactiveState {
    /// Get state as JSON string
    #[napi]
    pub fn get_state_json(&self) -> napi::Result<String> {
      // ReactiveState stores values as Any types in a HashMap
      // For FFI, we'll create a simplified JSON representation
      let fields = self.state.fields().read().unwrap();

      // Create a JSON object with string representations of the fields
      let mut json_map = serde_json::Map::new();

      for (key, value) in fields.iter() {
        // Use dynamic type detection to serialize values properly
        if let Some(string_val) = value.downcast_ref::<String>() {
          json_map.insert(key.clone(), serde_json::Value::String(string_val.clone()));
        } else if let Some(bool_val) = value.downcast_ref::<bool>() {
          json_map.insert(key.clone(), serde_json::Value::Bool(*bool_val));
        } else if let Some(i32_val) = value.downcast_ref::<i32>() {
          json_map.insert(
            key.clone(),
            serde_json::Value::Number(serde_json::Number::from(*i32_val)),
          );
        } else if let Some(i64_val) = value.downcast_ref::<i64>() {
          json_map.insert(
            key.clone(),
            serde_json::Value::Number(serde_json::Number::from(*i64_val)),
          );
        } else if let Some(u32_val) = value.downcast_ref::<u32>() {
          json_map.insert(
            key.clone(),
            serde_json::Value::Number(serde_json::Number::from(*u32_val)),
          );
        } else if let Some(u64_val) = value.downcast_ref::<u64>() {
          json_map.insert(
            key.clone(),
            serde_json::Value::Number(serde_json::Number::from(*u64_val)),
          );
        } else if let Some(f32_val) = value.downcast_ref::<f32>() {
          if let Some(num) = serde_json::Number::from_f64(*f32_val as f64) {
            json_map.insert(key.clone(), serde_json::Value::Number(num));
          } else {
            json_map.insert(key.clone(), serde_json::Value::Null);
          }
        } else if let Some(f64_val) = value.downcast_ref::<f64>() {
          if let Some(num) = serde_json::Number::from_f64(*f64_val) {
            json_map.insert(key.clone(), serde_json::Value::Number(num));
          } else {
            json_map.insert(key.clone(), serde_json::Value::Null);
          }
        } else if let Some(vec_str) = value.downcast_ref::<Vec<String>>() {
          let arr = vec_str
            .iter()
            .map(|s| serde_json::Value::String(s.clone()))
            .collect();
          json_map.insert(key.clone(), serde_json::Value::Array(arr));
        } else {
          // For unknown types, store as a type indicator string
          json_map.insert(
            key.clone(),
            serde_json::Value::String(format!("<{key}: unsupported type>")),
          );
        }
      }

      let json_value = serde_json::Value::Object(json_map);
      serde_json::to_string(&json_value)
        .map_err(|e| napi::Error::from_reason(format!("Failed to serialize state: {e}")))
    }

    /// Set state from JSON string
    #[napi]
    pub fn set_state_json(&self, json: String) -> napi::Result<()> {
      // Parse JSON into a value
      let value: serde_json::Value = serde_json::from_str(&json)
        .map_err(|e| napi::Error::from_reason(format!("Invalid JSON: {e}")))?;

      // Update the reactive state fields
      if let serde_json::Value::Object(map) = value {
        let mut fields = self.state.fields().write().unwrap();

        // Clear existing fields and add new ones from JSON
        fields.clear();

        for (key, val) in map {
          // Store JSON values as boxed Any types
          // In a full implementation, we'd convert based on value type
          let boxed_value: Box<dyn std::any::Any + Send + Sync> = Box::new(val.to_string());
          fields.insert(key.clone(), boxed_value);

          // Notify watchers about the change
          let _ = self
            .state
            .change_sender()
            .send(crate::reactive::StateChange {
              field_name: key,
              timestamp: std::time::Instant::now(),
            });
        }
      } else {
        return Err(napi::Error::from_reason("JSON must be an object"));
      }

      Ok(())
    }
  }

  /// Utility functions for common TUI operations
  #[napi]
  pub struct TuiUtils;

  #[napi]
  impl TuiUtils {
    /// Create a div element
    #[napi]
    pub fn div() -> JsElement {
      JsElement::new("div".to_string())
    }

    /// Create a text element
    #[napi]
    pub fn text(content: String) -> JsElement {
      let mut element = JsElement::new("text".to_string());
      let _ = element.set_content(content);
      element
    }

    /// Create a button element
    #[napi]
    pub fn button() -> JsElement {
      JsElement::new("button".to_string())
    }

    /// Create an input element
    #[napi]
    pub fn input() -> JsElement {
      JsElement::new("input".to_string())
    }

    /// Parse CSS and return validation errors
    #[napi]
    pub fn validate_css(css: String) -> napi::Result<Vec<String>> {
      use crate::css::CssEngine;

      let mut errors = Vec::new();

      // Check for empty CSS
      if css.trim().is_empty() {
        errors.push("Empty CSS input".to_string());
        return Ok(errors);
      }

      // Use the CSS engine to validate CSS by attempting to parse it
      let mut engine = CssEngine::new();
      match engine.load_stylesheet(&css) {
        Ok(_) => {
          // CSS parsed successfully, no errors
          // Could potentially return warnings about unsupported properties
        }
        Err(e) => {
          // Return parsing errors
          errors.push(format!("CSS parsing error: {e}"));
        }
      }

      Ok(errors)
    }

    /// Get terminal size
    #[napi]
    pub fn get_terminal_size() -> napi::Result<(u32, u32)> {
      let (cols, rows) = crate::compat::terminal::size()
        .map_err(|e| napi::Error::from_reason(format!("Failed to get terminal size: {e}")))?;
      Ok((cols as u32, rows as u32))
    }
  }

  /// Action constants for JavaScript
  #[napi]
  pub struct Actions;

  #[napi]
  impl Actions {
    #[napi(getter)]
    pub fn quit() -> String {
      actions::QUIT.to_string()
    }

    #[napi(getter)]
    pub fn refresh() -> String {
      actions::REFRESH.to_string()
    }

    #[napi(getter)]
    pub fn focus_next() -> String {
      actions::FOCUS_NEXT.to_string()
    }

    #[napi(getter)]
    pub fn focus_previous() -> String {
      actions::FOCUS_PREVIOUS.to_string()
    }

    #[napi(getter)]
    pub fn activate() -> String {
      actions::ACTIVATE.to_string()
    }

    #[napi(getter)]
    pub fn scroll_up() -> String {
      actions::SCROLL_UP.to_string()
    }

    #[napi(getter)]
    pub fn scroll_down() -> String {
      actions::SCROLL_DOWN.to_string()
    }

    #[napi(getter)]
    pub fn copy() -> String {
      actions::COPY.to_string()
    }

    #[napi(getter)]
    pub fn paste() -> String {
      actions::PASTE.to_string()
    }

    #[napi(getter)]
    pub fn save() -> String {
      actions::SAVE.to_string()
    }
  }

  /// Initialize the TUI library (call this first from JavaScript)
  #[napi]
  pub fn init_tui() -> napi::Result<()> {
    // Initialize any global state if needed
    Ok(())
  }

  /// Module-level export for package.json "napi" field
  #[napi]
  pub fn get_version() -> String {
    env!("CARGO_PKG_VERSION").to_string()
  }

  // ============================================================================
  // ENHANCED FFI INTEGRATION WITH COMPREHENSIVE TYPES
  // ============================================================================

  /// JavaScript-compatible ColorDefinition wrapper
  #[napi]
  pub struct JsColorDefinition {
    color: ColorDefinition,
  }

  #[napi]
  impl JsColorDefinition {
    /// Create color from RGB values
    #[napi(factory)]
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
      Self {
        color: ColorDefinition { r, g, b },
      }
    }

    /// Create color from hex string
    #[napi(factory)]
    pub fn hex(hex_color: String) -> napi::Result<Self> {
      match crate::themes::colors::hex(&hex_color) {
        Ok(color) => Ok(Self { color }),
        Err(e) => Err(napi::Error::from_reason(e)),
      }
    }

    /// Get RGB values
    #[napi]
    pub fn get_rgb(&self) -> (u8, u8, u8) {
      (self.color.r, self.color.g, self.color.b)
    }

    /// Convert to ANSI escape code
    #[napi]
    pub fn to_ansi(&self, background: bool) -> String {
      crate::themes::colors::color_to_ansi(self.color, background)
    }
  }

  /// JavaScript-compatible ColorTheme wrapper with comprehensive theme support
  #[napi]
  pub struct JsColorTheme {
    theme: ColorTheme,
  }

  #[napi]
  impl JsColorTheme {
    /// Get dark theme
    #[napi(factory)]
    pub fn dark() -> Self {
      Self {
        theme: crate::themes::colors::dark_theme(),
      }
    }

    /// Get light theme
    #[napi(factory)]
    pub fn light() -> Self {
      Self {
        theme: crate::themes::colors::light_theme(),
      }
    }

    /// Get terminal theme
    #[napi(factory)]
    pub fn terminal() -> Self {
      Self {
        theme: crate::themes::colors::terminal_theme(),
      }
    }

    /// Get theme as JSON string (with camelCase conversion)
    #[napi]
    pub fn to_json(&self) -> napi::Result<String> {
      serde_json::to_string_pretty(&self.theme)
        .map_err(|e| napi::Error::from_reason(format!("Failed to serialize theme: {e}")))
    }

    /// Load theme from JSON string (with camelCase support)
    #[napi(factory)]
    pub fn from_json(json: String) -> napi::Result<Self> {
      let theme: ColorTheme = serde_json::from_str(&json)
        .map_err(|e| napi::Error::from_reason(format!("Failed to parse theme JSON: {e}")))?;
      Ok(Self { theme })
    }

    /// Get semantic color as ANSI code
    #[napi]
    pub fn get_semantic_color(&self, semantic_key: String) -> napi::Result<String> {
      crate::themes::colors::get_semantic_color(&self.theme, &semantic_key)
        .map_err(napi::Error::from_reason)
    }

    /// Get semantic background color as ANSI code
    #[napi]
    pub fn get_semantic_background(&self, semantic_key: String) -> napi::Result<String> {
      crate::themes::colors::get_semantic_background(&self.theme, &semantic_key)
        .map_err(napi::Error::from_reason)
    }

    /// Get theme name
    #[napi]
    pub fn get_name(&self) -> String {
      self.theme.name.clone()
    }

    /// Get theme description
    #[napi]
    pub fn get_description(&self) -> String {
      self.theme.description.clone()
    }
  }

  /// Export comprehensive type definitions for TypeScript generation
  #[napi]
  pub struct EnhancedFFITypes;

  #[napi]
  impl EnhancedFFITypes {
    /// Get all available semantic color keys
    #[napi]
    pub fn semantic_color_keys() -> Vec<String> {
      vec![
        "panel_background".to_string(),
        "panel_border".to_string(),
        "panel_title".to_string(),
        "panel_content".to_string(),
        "panel_shadow".to_string(),
        "button_background".to_string(),
        "button_border".to_string(),
        "button_text".to_string(),
        "button_hover".to_string(),
        "input_background".to_string(),
        "input_border".to_string(),
        "input_text".to_string(),
        "input_focus".to_string(),
        "progress_background".to_string(),
        "progress_fill".to_string(),
        "progress_text".to_string(),
        "editor_background".to_string(),
        "editor_text".to_string(),
        "editor_cursor".to_string(),
        "editor_line_number".to_string(),
        "editor_selection".to_string(),
        "editor_border".to_string(),
        "editor_border_focus".to_string(),
        "syntax_keyword".to_string(),
        "syntax_string".to_string(),
        "syntax_comment".to_string(),
        "syntax_number".to_string(),
        "syntax_function".to_string(),
        "syntax_type".to_string(),
        "syntax_variable".to_string(),
        "syntax_operator".to_string(),
        "syntax_punctuation".to_string(),
        "syntax_constant".to_string(),
        "syntax_tag".to_string(),
        "syntax_attribute".to_string(),
      ]
    }

    /// Get all available color palette keys
    #[napi]
    pub fn color_palette_keys() -> Vec<String> {
      vec![
        "primary".to_string(),
        "primary_dark".to_string(),
        "primary_light".to_string(),
        "secondary".to_string(),
        "secondary_dark".to_string(),
        "secondary_light".to_string(),
        "background".to_string(),
        "background_alt".to_string(),
        "surface".to_string(),
        "surface_alt".to_string(),
        "text".to_string(),
        "text_secondary".to_string(),
        "text_muted".to_string(),
        "text_inverse".to_string(),
        "border".to_string(),
        "border_focus".to_string(),
        "border_hover".to_string(),
        "success".to_string(),
        "warning".to_string(),
        "error".to_string(),
        "info".to_string(),
        "hover".to_string(),
        "active".to_string(),
        "disabled".to_string(),
        "shadow".to_string(),
        "shadow_light".to_string(),
      ]
    }

    /// Get widget type constants for TypeScript
    #[napi]
    pub fn widget_types() -> Vec<String> {
      vec![
        "Button".to_string(),
        "Input".to_string(),
        "Menu".to_string(),
        "Modal".to_string(),
        "Progress".to_string(),
        "Slider".to_string(),
        "Switch".to_string(),
        "Tabs".to_string(),
        "Toast".to_string(),
        "Textarea".to_string(),
      ]
    }

    /// Get element attribute constants
    #[napi]
    pub fn element_attributes() -> Vec<String> {
      vec![
        "id".to_string(),
        "class".to_string(),
        "style".to_string(),
        "focusable".to_string(),
        "disabled".to_string(),
        "visible".to_string(),
        "role".to_string(),
        "aria-label".to_string(),
        "aria-describedby".to_string(),
        "data-*".to_string(),
      ]
    }

    /// Get CSS utility class prefixes
    #[napi]
    pub fn css_utility_prefixes() -> Vec<String> {
      vec![
        "bg-".to_string(),
        "text-".to_string(),
        "border-".to_string(),
        "p-".to_string(),
        "m-".to_string(),
        "w-".to_string(),
        "h-".to_string(),
        "flex-".to_string(),
        "grid-".to_string(),
        "rounded-".to_string(),
        "shadow-".to_string(),
      ]
    }
  }

  /// NAPI module registration
  #[napi]
  fn init() -> napi::Result<()> {
    Ok(())
  }
} // end of exports module

#[cfg(feature = "ffi")]
pub use exports::*;
