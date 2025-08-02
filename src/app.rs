//! Main application runner and builder

use crate::events::actions::common;
use crate::{
    components::Component,
    css::CssEngine,
    driver::{DriverConfig, DriverEvent, DriverManager},
    error::{Result, TuiError},
    events::{
        Action, ActionResult, Event, EventHandler, FocusManager, KeyAction, KeyBindingManager,
        KeyBindingResult, KeyCombination, NavigationDirection,
    },
    layout::LayoutEngine,
    rendering::Renderer,
};
use serde_json::Value;
use std::{path::PathBuf, sync::Arc, time::Duration};
use tokio::sync::RwLock;

pub struct TuiApp {
    css_engine: Arc<RwLock<CssEngine>>,
    layout_engine: Arc<RwLock<LayoutEngine>>,
    renderer: Arc<RwLock<Renderer>>,
    event_handler: EventHandler,
    focus_manager: Arc<RwLock<FocusManager>>,
    key_binding_manager: Arc<RwLock<KeyBindingManager>>,
    root_component: Option<Box<dyn Component>>,
    stylesheets: Vec<PathBuf>,
    is_running: Arc<RwLock<bool>>,
    driver_manager: DriverManager,
    frame_rate: Duration,
}

impl TuiApp {
    pub fn builder() -> TuiAppBuilder {
        TuiAppBuilder::new()
    }

    /// Load all configured stylesheets into the CSS engine
    pub fn load_stylesheets(&mut self) -> Result<()> {
        let mut css_engine = self
            .css_engine
            .try_write()
            .map_err(|_| TuiError::component("Failed to acquire CSS engine lock".to_string()))?;

        for stylesheet_path in &self.stylesheets {
            let css_content = std::fs::read_to_string(stylesheet_path).map_err(|e| {
                TuiError::component(format!(
                    "Failed to read stylesheet {}: {}",
                    stylesheet_path.display(),
                    e
                ))
            })?;

            css_engine.load_stylesheet(&css_content).map_err(|e| {
                TuiError::component(format!(
                    "Failed to parse stylesheet {}: {}",
                    stylesheet_path.display(),
                    e
                ))
            })?;
        }

        Ok(())
    }

    /// Reload all stylesheets (useful for hot reload)
    pub fn reload_stylesheets(&mut self) -> Result<()> {
        let mut css_engine = self
            .css_engine
            .try_write()
            .map_err(|_| TuiError::component("Failed to acquire CSS engine lock".to_string()))?;

        css_engine.clear_stylesheets();
        drop(css_engine);

        self.load_stylesheets()
    }

    /// Add a stylesheet at runtime
    pub fn add_stylesheet<P: Into<PathBuf>>(&mut self, path: P) -> Result<()> {
        let path = path.into();
        self.stylesheets.push(path.clone());

        let css_content = std::fs::read_to_string(&path).map_err(|e| {
            TuiError::component(format!(
                "Failed to read stylesheet {}: {}",
                path.display(),
                e
            ))
        })?;

        let mut css_engine = self
            .css_engine
            .try_write()
            .map_err(|_| TuiError::component("Failed to acquire CSS engine lock".to_string()))?;

        css_engine.load_stylesheet(&css_content).map_err(|e| {
            TuiError::component(format!(
                "Failed to parse stylesheet {}: {}",
                path.display(),
                e
            ))
        })?;

        Ok(())
    }

    /// Set the window title (delegates to driver)
    pub fn set_title(&mut self, title: &str) {
        self.driver_manager.set_title(title);
    }

    /// Load CSS from a string
    pub fn load_css(&mut self, css: String) -> Result<()> {
        let mut css_engine = self
            .css_engine
            .try_write()
            .map_err(|_| TuiError::component("Failed to acquire CSS engine lock".to_string()))?;

        css_engine.load_stylesheet(&css)
    }

    /// Set the root component
    pub fn set_component(&mut self, component: Box<dyn Component>) -> Result<()> {
        self.root_component = Some(component);
        Ok(())
    }

    /// Check if a root component has been set
    pub fn has_component(&self) -> bool {
        self.root_component.is_some()
    }

    pub async fn run(mut self) -> Result<()> {
        // Start the driver and get event receiver
        let mut event_receiver = self.driver_manager.start()?;

        // Main event loop
        while *self.is_running.read().await {
            // Handle driver events
            tokio::select! {
                // Handle driver events
                event = event_receiver.recv() => {
                    match event {
                        Some(DriverEvent::Key(key)) => {
                            let should_continue = self.handle_key_binding(&key).await;
                            if !should_continue {
                                self.stop().await;
                                break;
                            }
                            self.event_handler.handle_key_event(key).await;
                        }
                        Some(DriverEvent::Mouse(mouse)) => {
                            self.event_handler.handle_mouse_event(mouse).await;
                        }
                        Some(DriverEvent::Resize(width, height)) => {
                            self.handle_resize(width, height).await?;
                        }
                        Some(DriverEvent::Quit) => {
                            self.stop().await;
                            break;
                        }
                        Some(DriverEvent::Custom(name, data)) => {
                            let event = Event::Custom(name, data);
                            self.event_handler.emit(event);
                        }
                        None => {
                            // Channel closed, exit
                            break;
                        }
                    }
                }

                // Render frame periodically
                _ = tokio::time::sleep(self.frame_rate) => {
                    self.render_frame().await?;
                }
            }
        }

        // Clean up
        self.driver_manager.stop()?;
        Ok(())
    }

    pub async fn stop(&self) {
        *self.is_running.write().await = false;
    }

    /// Bind a key to an app-level action
    pub async fn bind_key(&self, key: KeyCombination, action: KeyAction) {
        let mut key_binding_manager = self.key_binding_manager.write().await;
        key_binding_manager.bind_app_key(key, action);
    }

    /// Unbind a key
    pub async fn unbind_key(&self, key: &KeyCombination) {
        let mut key_binding_manager = self.key_binding_manager.write().await;
        key_binding_manager.unbind_key(key);
    }

    /// Get help text for all key bindings
    pub async fn get_key_bindings_help(&self) -> String {
        let key_binding_manager = self.key_binding_manager.read().await;
        key_binding_manager.get_help_text()
    }

    /// Register an action handler
    pub fn register_action<F>(&mut self, action_name: &str, handler: F)
    where
        F: Fn(&mut Action) -> ActionResult + Send + Sync + 'static,
    {
        self.event_handler.register_action(action_name, handler);
    }

    /// Dispatch an action immediately
    pub fn dispatch_action(&self, action: Action) -> ActionResult {
        self.event_handler.dispatch_action(action)
    }

    /// Send an action for async processing
    pub fn send_action(&self, action: Action) -> Result<()> {
        self.event_handler.send_action(action)
    }

    /// Create an action builder
    pub fn action<S: Into<String>>(&self, name: S) -> crate::events::ActionBuilder {
        self.event_handler.action(name)
    }

    /// Bind a key to an action
    pub async fn bind_key_to_action(&self, key: KeyCombination, action_name: &str) {
        let mut key_binding_manager = self.key_binding_manager.write().await;
        key_binding_manager.bind_app_key(key, KeyAction::Action(action_name.to_string()));
    }

    /// Bind a key to an action with parameters
    pub async fn bind_key_to_action_with_params(
        &self,
        key: KeyCombination,
        action_name: &str,
        params: serde_json::Value,
    ) {
        let mut key_binding_manager = self.key_binding_manager.write().await;
        key_binding_manager.bind_app_key(
            key,
            KeyAction::ActionWithParams(action_name.to_string(), params),
        );
    }

    async fn render_frame(&self) -> Result<()> {
        if let Some(component) = &self.root_component {
            // Build virtual DOM
            let mut element = component.render();

            // Update focus manager and apply focus to element tree
            {
                let mut focus_manager = self.focus_manager.write().await;
                focus_manager.build_focus_list(&element);
                focus_manager.apply_focus_to_tree(&mut element);
            }

            // Apply CSS styles
            let computed_styles = {
                let css_engine = self.css_engine.read().await;
                css_engine.apply_styles(&element)
            };

            // Compute layout
            let layout = {
                let mut layout_engine = self.layout_engine.write().await;
                layout_engine.compute_layout_with_styles(&element, &computed_styles)?
            };

            // Render to terminal
            {
                let mut renderer = self.renderer.write().await;
                renderer.render(&layout).await?;
            }
        }

        Ok(())
    }

    /// Handle key binding and return whether app should continue running
    async fn handle_key_binding(&self, key: &crossterm::event::KeyEvent) -> bool {
        let mut key_binding_manager = self.key_binding_manager.write().await;
        let binding_result = key_binding_manager.handle_key(key);

        match binding_result {
            KeyBindingResult::AppAction(action) => {
                match action {
                    KeyAction::Quit => return false,
                    KeyAction::Custom(name) => {
                        if name == "activate_focused" {
                            self.activate_focused_element().await;
                        }
                        // Could emit custom event here for other custom actions
                    }
                    KeyAction::ActivateElement(element_id) => {
                        self.activate_element_by_id(&element_id).await;
                    }
                    KeyAction::FocusElement(element_id) => {
                        self.focus_element_by_id(&element_id).await;
                    }
                    KeyAction::Action(action_name) => {
                        let action = Action::new(action_name).from_source("key_binding");
                        let result = self.event_handler.dispatch_action(action);
                        if let ActionResult::Error(msg) = result {
                            eprintln!("Action error: {msg}");
                        }
                    }
                    KeyAction::ActionWithParams(action_name, params) => {
                        let action =
                            Action::with_params(action_name, params).from_source("key_binding");
                        let result = self.event_handler.dispatch_action(action);
                        if let ActionResult::Error(msg) = result {
                            eprintln!("Action error: {msg}");
                        }
                    }
                    _ => {}
                }
            }
            KeyBindingResult::ElementAction(binding) => {
                self.handle_element_action(&binding.element_id, &binding.action)
                    .await;
            }
            KeyBindingResult::Navigation(direction) => {
                self.handle_navigation(direction).await;
            }
            KeyBindingResult::Unhandled => {
                // Key not bound to anything - could emit unhandled key event
            }
        }

        true // Continue running
    }

    async fn handle_navigation(&self, direction: NavigationDirection) {
        let mut focus_manager = self.focus_manager.write().await;

        match direction {
            NavigationDirection::Next | NavigationDirection::Down | NavigationDirection::Right => {
                focus_manager.focus_next();
            }
            NavigationDirection::Previous | NavigationDirection::Up | NavigationDirection::Left => {
                focus_manager.focus_previous();
            }
        }
    }

    async fn activate_focused_element(&self) {
        let focus_manager = self.focus_manager.read().await;
        if let Some(focused_element) = focus_manager.get_focused_element() {
            // Emit custom event for focused element activation
            let event = Event::Custom(
                "element_activated".to_string(),
                serde_json::json!({
                    "element_id": focused_element.id,
                    "tab_index": focused_element.tab_index
                }),
            );
            self.event_handler.emit(event);
        }
    }

    async fn activate_element_by_id(&self, element_id: &str) {
        let event = Event::Custom(
            "element_activated".to_string(),
            serde_json::json!({
                "element_id": element_id,
                "activation_method": "key_binding"
            }),
        );
        self.event_handler.emit(event);
    }

    async fn focus_element_by_id(&self, element_id: &str) {
        let mut focus_manager = self.focus_manager.write().await;
        focus_manager.focus_by_id(element_id);
    }

    async fn handle_element_action(&self, element_id: &str, action: &crate::events::ElementAction) {
        match action {
            crate::events::ElementAction::Activate => {
                self.activate_element_by_id(element_id).await;
            }
            crate::events::ElementAction::Focus => {
                self.focus_element_by_id(element_id).await;
            }
            crate::events::ElementAction::Toggle => {
                let event = Event::Custom(
                    "element_toggled".to_string(),
                    serde_json::json!({
                        "element_id": element_id
                    }),
                );
                self.event_handler.emit(event);
            }
            crate::events::ElementAction::Custom(action_name) => {
                if action_name == "dismiss" {
                    // Handle modal dismissal
                    let event = Event::Custom(
                        "modal_dismissed".to_string(),
                        serde_json::json!({
                            "element_id": element_id
                        }),
                    );
                    self.event_handler.emit(event);
                } else {
                    let event = Event::Custom(
                        action_name.clone(),
                        serde_json::json!({
                            "element_id": element_id
                        }),
                    );
                    self.event_handler.emit(event);
                }
            }
        }
    }

    async fn handle_resize(&self, width: u16, height: u16) -> Result<()> {
        let mut renderer = self.renderer.write().await;
        renderer.resize(width, height).await?;
        Ok(())
    }

    /// Get access to the underlying driver for advanced operations
    pub fn driver(&self) -> &dyn crate::driver::Driver {
        self.driver_manager.driver()
    }

    /// Get mutable access to the underlying driver for advanced operations
    pub fn driver_mut(&mut self) -> &mut dyn crate::driver::Driver {
        self.driver_manager.driver_mut()
    }
}

pub struct TuiAppBuilder {
    stylesheets: Vec<PathBuf>,
    component: Option<Box<dyn Component>>,
    driver_config: DriverConfig,
    frame_rate: Duration,
}

impl TuiAppBuilder {
    pub fn new() -> Self {
        Self {
            stylesheets: Vec::new(),
            component: None,
            driver_config: DriverConfig::default(),
            frame_rate: Duration::from_millis(33), // ~30 FPS, more reasonable for TUI
        }
    }

    pub fn stylesheet<P: Into<PathBuf>>(mut self, path: P) -> Self {
        self.stylesheets.push(path.into());
        self
    }

    pub fn component<C: Component + 'static>(mut self, component: C) -> Self {
        self.component = Some(Box::new(component));
        self
    }

    /// Configure the driver (terminal backend)
    pub fn driver_config(mut self, config: DriverConfig) -> Self {
        self.driver_config = config;
        self
    }

    /// Enable mouse support
    pub fn with_mouse(mut self, enabled: bool) -> Self {
        self.driver_config.mouse = enabled;
        self
    }

    /// Set terminal title
    pub fn with_title<S: Into<String>>(mut self, title: S) -> Self {
        self.driver_config.title = Some(title.into());
        self
    }

    /// Enable inline mode (non-fullscreen)
    pub fn inline_mode(mut self, enabled: bool) -> Self {
        self.driver_config.inline = enabled;
        self
    }

    /// Enable debug mode
    pub fn debug_mode(mut self, enabled: bool) -> Self {
        self.driver_config.debug = enabled;
        self
    }

    /// Set custom terminal size (for testing)
    pub fn with_size(mut self, cols: u16, rows: u16) -> Self {
        self.driver_config.size = Some((cols, rows));
        self
    }

    /// Use headless driver (for testing/automation)
    pub fn headless(mut self) -> Self {
        self.driver_config.driver_type = Some(crate::driver::DriverType::Headless);
        self
    }

    /// Set frame rate (default: 30 FPS)
    pub fn frame_rate(mut self, fps: u32) -> Self {
        self.frame_rate = Duration::from_millis(1000 / fps as u64);
        self
    }

    /// Set frame rate from duration
    pub fn frame_duration(mut self, duration: Duration) -> Self {
        self.frame_rate = duration;
        self
    }

    pub fn build(self) -> Result<TuiApp> {
        let css_engine = Arc::new(RwLock::new(CssEngine::new()));
        let layout_engine = Arc::new(RwLock::new(LayoutEngine::new()));
        let renderer = Arc::new(RwLock::new(Renderer::new()?));
        let mut event_handler = EventHandler::new();
        let driver_manager = DriverManager::with_config(self.driver_config)?;

        // Set up default actions
        Self::setup_default_actions(&mut event_handler);

        // Set up automatic layout responsive behavior
        let layout_engine_for_resize = layout_engine.clone();
        event_handler.on("resize", move |event| {
            if let Event::Resize(width, height) = event {
                if let Ok(mut engine) = layout_engine_for_resize.try_write() {
                    engine.update_dimensions(*width, *height);
                    // Could trigger a re-layout here if needed
                }
            }
            false // Don't stop propagation
        });

        let mut app = TuiApp {
            css_engine,
            layout_engine,
            renderer,
            event_handler,
            focus_manager: Arc::new(RwLock::new(FocusManager::new())),
            key_binding_manager: Arc::new(RwLock::new(KeyBindingManager::new())),
            root_component: self.component,
            stylesheets: self.stylesheets.clone(),
            is_running: Arc::new(RwLock::new(true)),
            driver_manager,
            frame_rate: self.frame_rate,
        };

        // Load all stylesheets
        app.load_stylesheets()?;

        Ok(app)
    }

    /// Set up default actions for the application
    fn setup_default_actions(event_handler: &mut EventHandler) {
        // Focus navigation actions
        event_handler.register_action(common::FOCUS_NEXT, |action| {
            // Set a flag for the focus manager to process
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    map.insert("focus_direction".to_string(), Value::String("next".to_string()));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("focus_direction".to_string(), Value::String("next".to_string()));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        event_handler.register_action(common::FOCUS_PREVIOUS, |action| {
            // Set a flag for the focus manager to process
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    map.insert("focus_direction".to_string(), Value::String("previous".to_string()));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("focus_direction".to_string(), Value::String("previous".to_string()));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        // Activation action
        event_handler.register_action(common::ACTIVATE, |action| {
            // Set activation flag for the focused component
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    map.insert("activated".to_string(), Value::Bool(true));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("activated".to_string(), Value::Bool(true));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        // Scroll actions
        event_handler.register_action(common::SCROLL_UP, |action| {
            let lines = action.get_number_param("lines").unwrap_or(1.0) as i32;
            // Set scroll parameters for the focused component
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    map.insert("scroll_direction".to_string(), Value::String("up".to_string()));
                    map.insert("scroll_lines".to_string(), Value::Number(serde_json::Number::from(lines)));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("scroll_direction".to_string(), Value::String("up".to_string()));
                    map.insert("scroll_lines".to_string(), Value::Number(serde_json::Number::from(lines)));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        event_handler.register_action(common::SCROLL_DOWN, |action| {
            let lines = action.get_number_param("lines").unwrap_or(1.0) as i32;
            // Set scroll parameters for the focused component
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    map.insert("scroll_direction".to_string(), Value::String("down".to_string()));
                    map.insert("scroll_lines".to_string(), Value::Number(serde_json::Number::from(lines)));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("scroll_direction".to_string(), Value::String("down".to_string()));
                    map.insert("scroll_lines".to_string(), Value::Number(serde_json::Number::from(lines)));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        // Example custom action
        event_handler.register_action("toggle_help", |action| {
            // Set help toggle flag
            if let Some(params) = &mut action.params {
                if let Value::Object(map) = params {
                    let current = map.get("help_visible")
                        .and_then(|v| v.as_bool())
                        .unwrap_or(false);
                    map.insert("help_visible".to_string(), Value::Bool(!current));
                    map.insert("help_toggled".to_string(), Value::Bool(true));
                }
            } else {
                action.params = Some(Value::Object({
                    let mut map = serde_json::Map::new();
                    map.insert("help_visible".to_string(), Value::Bool(true));
                    map.insert("help_toggled".to_string(), Value::Bool(true));
                    map
                }));
            }
            ActionResult::HandledContinue
        });

        // Example parameterized action
        event_handler.register_action("show_message", |action| {
            if let Some(message) = action.get_string_param("message") {
                eprintln!("Message: {message}");
                ActionResult::Handled
            } else {
                ActionResult::Error("Missing 'message' parameter".to_string())
            }
        });
    }
}

impl Default for TuiAppBuilder {
    fn default() -> Self {
        Self::new()
    }
}
