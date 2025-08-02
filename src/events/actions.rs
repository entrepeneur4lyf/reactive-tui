//! Action system with string-based routing for centralized command handling
//!
//! Provides a flexible action system that allows applications to define custom actions
//! and route them through a centralized dispatcher with hierarchical bubbling support.

use crate::error::{Result, TuiError};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use tokio::sync::mpsc;

/// Trait for objects that can handle actions
pub trait ActionHandler: Send + Sync {
    /// Handle an action with the given name and parameters
    /// Returns true if the action was handled and should not bubble up
    fn handle_action(&mut self, action: &str, params: Option<Value>) -> ActionResult;

    /// Get the ID of this handler (for debugging and routing)
    fn handler_id(&self) -> &str;
}

/// Result of handling an action
#[derive(Debug, Clone, PartialEq)]
pub enum ActionResult {
    /// Action was handled successfully, stop bubbling
    Handled,
    /// Action was handled but should continue bubbling
    HandledContinue,
    /// Action was not handled, continue bubbling
    NotHandled,
    /// Action failed with an error
    Error(String),
}

/// An action with its name, parameters, and metadata
#[derive(Debug, Clone)]
pub struct Action {
    /// The name of the action (e.g., "quit", "save_file", "toggle_sidebar")
    pub name: String,
    /// Optional parameters for the action
    pub params: Option<Value>,
    /// The source/origin of the action (for debugging)
    pub source: Option<String>,
    /// Whether this action should bubble up the hierarchy
    pub bubbles: bool,
    /// Whether this action can be cancelled
    pub cancelable: bool,
    /// Whether this action has been cancelled
    pub cancelled: bool,
}

impl Action {
    /// Create a new action with the given name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            params: None,
            source: None,
            bubbles: true,
            cancelable: true,
            cancelled: false,
        }
    }

    /// Create an action with parameters
    pub fn with_params<S: Into<String>>(name: S, params: Value) -> Self {
        Self {
            name: name.into(),
            params: Some(params),
            source: None,
            bubbles: true,
            cancelable: true,
            cancelled: false,
        }
    }

    /// Set the source of this action
    pub fn from_source<S: Into<String>>(mut self, source: S) -> Self {
        self.source = Some(source.into());
        self
    }

    /// Set whether this action bubbles up
    pub fn bubbles(mut self, bubbles: bool) -> Self {
        self.bubbles = bubbles;
        self
    }

    /// Set whether this action can be cancelled
    pub fn cancelable(mut self, cancelable: bool) -> Self {
        self.cancelable = cancelable;
        self
    }

    /// Cancel this action if it's cancelable
    pub fn cancel(&mut self) -> bool {
        if self.cancelable {
            self.cancelled = true;
            true
        } else {
            false
        }
    }

    /// Check if this action is cancelled
    pub fn is_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Get a parameter by key
    pub fn get_param(&self, key: &str) -> Option<&Value> {
        self.params.as_ref()?.get(key)
    }

    /// Get a string parameter
    pub fn get_string_param(&self, key: &str) -> Option<&str> {
        self.get_param(key)?.as_str()
    }

    /// Get a number parameter
    pub fn get_number_param(&self, key: &str) -> Option<f64> {
        self.get_param(key)?.as_f64()
    }

    /// Get a boolean parameter
    pub fn get_bool_param(&self, key: &str) -> Option<bool> {
        self.get_param(key)?.as_bool()
    }
}

/// Function-based action handler
pub type ActionCallback = Box<dyn Fn(&mut Action) -> ActionResult + Send + Sync>;

/// Action dispatcher that manages and routes actions
pub struct ActionDispatcher {
    /// Registered action handlers by name
    handlers: Arc<RwLock<HashMap<String, ActionCallback>>>,
    /// Action event channel
    action_sender: mpsc::UnboundedSender<Action>,
    action_receiver: Option<mpsc::UnboundedReceiver<Action>>,
    /// Hierarchical handlers (for bubbling)
    hierarchy: Arc<RwLock<Vec<String>>>,
    /// Default actions
    default_actions: Arc<RwLock<HashMap<String, ActionCallback>>>,
}

impl ActionDispatcher {
    /// Create a new action dispatcher
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        let mut dispatcher = Self {
            handlers: Arc::new(RwLock::new(HashMap::new())),
            action_sender: sender,
            action_receiver: Some(receiver),
            hierarchy: Arc::new(RwLock::new(Vec::new())),
            default_actions: Arc::new(RwLock::new(HashMap::new())),
        };

        dispatcher.setup_default_actions();
        dispatcher
    }

    /// Set up default application actions
    fn setup_default_actions(&mut self) {
        let mut defaults = self.default_actions.write().unwrap();

        // Default quit action
        defaults.insert(
            "quit".to_string(),
            Box::new(|action| {
                eprintln!("Application quit requested from: {:?}", action.source);
                std::process::exit(0);
            }),
        );

        // Default bell action
        defaults.insert(
            "bell".to_string(),
            Box::new(|_action| {
                print!("\x07"); // ASCII bell
                ActionResult::Handled
            }),
        );

        // Default no-op action
        defaults.insert(
            "no-op".to_string(),
            Box::new(|_action| ActionResult::Handled),
        );

        // Default screen refresh
        defaults.insert(
            "refresh".to_string(),
            Box::new(|action| {
                // Set a flag in action params to indicate refresh is needed
                // The main event loop will check for this flag
                if let Some(params) = &mut action.params {
                    if let Value::Object(map) = params {
                        map.insert("refresh_required".to_string(), Value::Bool(true));
                    }
                } else {
                    action.params = Some(Value::Object({
                        let mut map = serde_json::Map::new();
                        map.insert("refresh_required".to_string(), Value::Bool(true));
                        map
                    }));
                }
                ActionResult::HandledContinue // Continue so app can see the refresh flag
            }),
        );
    }

    /// Register an action handler
    pub fn register<F>(&mut self, action_name: &str, handler: F)
    where
        F: Fn(&mut Action) -> ActionResult + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.write().unwrap();
        handlers.insert(action_name.to_string(), Box::new(handler));
    }

    /// Register multiple actions with the same handler
    pub fn register_multiple<F>(&mut self, action_names: &[&str], handler: F)
    where
        F: Fn(&mut Action) -> ActionResult + Send + Sync + 'static + Clone,
    {
        let mut handlers = self.handlers.write().unwrap();
        for name in action_names {
            handlers.insert(name.to_string(), Box::new(handler.clone()));
        }
    }

    /// Unregister an action handler
    pub fn unregister(&mut self, action_name: &str) {
        let mut handlers = self.handlers.write().unwrap();
        handlers.remove(action_name);
    }

    /// Check if an action is registered
    pub fn is_registered(&self, action_name: &str) -> bool {
        let handlers = self.handlers.read().unwrap();
        handlers.contains_key(action_name)
            || self
                .default_actions
                .read()
                .unwrap()
                .contains_key(action_name)
    }

    /// Dispatch an action immediately
    pub fn dispatch(&self, mut action: Action) -> ActionResult {
        if action.is_cancelled() {
            return ActionResult::NotHandled;
        }

        // Try registered handlers first
        {
            let handlers = self.handlers.read().unwrap();
            if let Some(handler) = handlers.get(&action.name) {
                let result = handler(&mut action);
                if result != ActionResult::NotHandled {
                    return result;
                }
            }
        }

        // Try default handlers
        {
            let defaults = self.default_actions.read().unwrap();
            if let Some(handler) = defaults.get(&action.name) {
                return handler(&mut action);
            }
        }

        ActionResult::NotHandled
    }

    /// Send an action to be processed asynchronously
    pub fn send(&self, action: Action) -> Result<()> {
        self.action_sender
            .send(action)
            .map_err(|e| TuiError::component(format!("Failed to send action: {e}")))
    }

    /// Get the action receiver for processing actions in an event loop
    pub fn take_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<Action>> {
        self.action_receiver.take()
    }

    /// Set up action hierarchy for bubbling actions up the component tree
    pub fn set_hierarchy(&self, hierarchy: Vec<String>) -> Result<()> {
        let mut h = self
            .hierarchy
            .write()
            .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;
        *h = hierarchy;
        Ok(())
    }

    /// Add element to action hierarchy
    pub fn add_to_hierarchy(&self, element_id: String) -> Result<()> {
        let mut hierarchy = self
            .hierarchy
            .write()
            .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;
        hierarchy.push(element_id);
        Ok(())
    }

    /// Remove element from action hierarchy
    pub fn remove_from_hierarchy(&self, element_id: &str) -> Result<bool> {
        let mut hierarchy = self
            .hierarchy
            .write()
            .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;
        let original_len = hierarchy.len();
        hierarchy.retain(|id| id != element_id);
        Ok(hierarchy.len() != original_len)
    }

    /// Get current action hierarchy
    pub fn get_hierarchy(&self) -> Result<Vec<String>> {
        let hierarchy = self
            .hierarchy
            .read()
            .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;
        Ok(hierarchy.clone())
    }

    /// Get a list of all registered action names
    pub fn get_registered_actions(&self) -> Vec<String> {
        let handlers = self.handlers.read().unwrap();
        let defaults = self.default_actions.read().unwrap();

        let mut actions: Vec<String> = handlers.keys().cloned().collect();
        actions.extend(defaults.keys().cloned());
        actions.sort();
        actions
    }

    /// Create an action builder for fluent API
    pub fn action<S: Into<String>>(&self, name: S) -> ActionBuilder {
        ActionBuilder::new(name.into(), self.action_sender.clone())
    }
}

impl Default for ActionDispatcher {
    fn default() -> Self {
        Self::new()
    }
}

/// Builder for creating and dispatching actions with a fluent API
pub struct ActionBuilder {
    action: Action,
    sender: mpsc::UnboundedSender<Action>,
}

impl ActionBuilder {
    fn new(name: String, sender: mpsc::UnboundedSender<Action>) -> Self {
        Self {
            action: Action::new(name),
            sender,
        }
    }

    /// Add parameters to the action
    pub fn with_params(mut self, params: Value) -> Self {
        self.action.params = Some(params);
        self
    }

    /// Add a single parameter
    pub fn param<K: Into<String>, V: Into<Value>>(mut self, key: K, value: V) -> Self {
        let params = self
            .action
            .params
            .get_or_insert_with(|| Value::Object(serde_json::Map::new()));
        if let Value::Object(map) = params {
            map.insert(key.into(), value.into());
        }
        self
    }

    /// Set the source of the action
    pub fn from<S: Into<String>>(mut self, source: S) -> Self {
        self.action.source = Some(source.into());
        self
    }

    /// Set whether the action bubbles
    pub fn bubbles(mut self, bubbles: bool) -> Self {
        self.action.bubbles = bubbles;
        self
    }

    /// Set whether the action is cancelable
    pub fn cancelable(mut self, cancelable: bool) -> Self {
        self.action.cancelable = cancelable;
        self
    }

    /// Send the action for processing
    pub fn send(self) -> Result<()> {
        self.sender
            .send(self.action)
            .map_err(|e| TuiError::component(format!("Failed to send action via builder: {e}")))
    }

    /// Build the action without sending it
    pub fn build(self) -> Action {
        self.action
    }
}

/// Predefined action names for common operations
pub mod common {
    pub const QUIT: &str = "quit";
    pub const BELL: &str = "bell";
    pub const NO_OP: &str = "no-op";
    pub const REFRESH: &str = "refresh";
    pub const FOCUS_NEXT: &str = "focus_next";
    pub const FOCUS_PREVIOUS: &str = "focus_previous";
    pub const ACTIVATE: &str = "activate";
    pub const TOGGLE: &str = "toggle";
    pub const SCROLL_UP: &str = "scroll_up";
    pub const SCROLL_DOWN: &str = "scroll_down";
    pub const SCROLL_LEFT: &str = "scroll_left";
    pub const SCROLL_RIGHT: &str = "scroll_right";
    pub const SCROLL_HOME: &str = "scroll_home";
    pub const SCROLL_END: &str = "scroll_end";
    pub const COPY: &str = "copy";
    pub const PASTE: &str = "paste";
    pub const CUT: &str = "cut";
    pub const UNDO: &str = "undo";
    pub const REDO: &str = "redo";
    pub const SAVE: &str = "save";
    pub const LOAD: &str = "load";
    pub const NEW: &str = "new";
    pub const OPEN: &str = "open";
    pub const CLOSE: &str = "close";
    pub const MINIMIZE: &str = "minimize";
    pub const MAXIMIZE: &str = "maximize";
    pub const FULLSCREEN: &str = "fullscreen";
}

/// Macro for creating actions with parameters easily
#[macro_export]
macro_rules! action {
    ($name:expr) => {
        $crate::events::actions::Action::new($name)
    };

    ($name:expr, { $($key:expr => $value:expr),* $(,)? }) => {
        $crate::events::actions::Action::with_params(
            $name,
            serde_json::json!({ $($key: $value),* })
        )
    };

    ($name:expr, $params:expr) => {
        $crate::events::actions::Action::with_params($name, $params)
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_action_creation() {
        let action = Action::new("test");
        assert_eq!(action.name, "test");
        assert!(action.params.is_none());
        assert!(action.bubbles);
        assert!(action.cancelable);
        assert!(!action.cancelled);
    }

    #[test]
    fn test_action_with_params() {
        let params = json!({"key": "value", "number": 42});
        let action = Action::with_params("test", params.clone());
        assert_eq!(action.name, "test");
        assert_eq!(action.params, Some(params));
        assert_eq!(action.get_string_param("key"), Some("value"));
        assert_eq!(action.get_number_param("number"), Some(42.0));
    }

    #[test]
    fn test_action_cancellation() {
        let mut action = Action::new("test");
        assert!(!action.is_cancelled());

        assert!(action.cancel());
        assert!(action.is_cancelled());

        let mut non_cancelable = Action::new("test").cancelable(false);
        assert!(!non_cancelable.cancel());
        assert!(!non_cancelable.is_cancelled());
    }

    #[test]
    fn test_action_dispatcher() {
        let mut dispatcher = ActionDispatcher::new();

        dispatcher.register("test", |action| {
            assert_eq!(action.name, "test");
            ActionResult::Handled
        });

        assert!(dispatcher.is_registered("test"));
        assert!(dispatcher.is_registered("quit")); // Default action

        let action = Action::new("test");
        let result = dispatcher.dispatch(action);
        assert_eq!(result, ActionResult::Handled);
    }

    #[test]
    fn test_action_builder() {
        let dispatcher = ActionDispatcher::new();
        let action = dispatcher
            .action("test")
            .param("key", "value")
            .param("number", 42)
            .from("test_source")
            .bubbles(false)
            .build();

        assert_eq!(action.name, "test");
        assert_eq!(action.get_string_param("key"), Some("value"));
        assert_eq!(action.get_number_param("number"), Some(42.0));
        assert_eq!(action.source, Some("test_source".to_string()));
        assert!(!action.bubbles);
    }

    #[test]
    fn test_action_macro() {
        let action1 = action!("simple");
        assert_eq!(action1.name, "simple");

        let action2 = action!("with_params", {"key" => "value", "num" => 42});
        assert_eq!(action2.name, "with_params");
        assert_eq!(action2.get_string_param("key"), Some("value"));
        assert_eq!(action2.get_number_param("num"), Some(42.0));
    }

    #[test]
    fn test_default_actions() {
        let dispatcher = ActionDispatcher::new();
        let registered = dispatcher.get_registered_actions();

        assert!(registered.contains(&"quit".to_string()));
        assert!(registered.contains(&"bell".to_string()));
        assert!(registered.contains(&"no-op".to_string()));
        assert!(registered.contains(&"refresh".to_string()));
    }

    #[tokio::test]
    async fn test_async_action_processing() {
        let mut dispatcher = ActionDispatcher::new();
        let mut receiver = dispatcher.take_receiver().unwrap();

        // Send an action
        let action = Action::new("async_test");
        dispatcher.send(action).unwrap();

        // Receive the action
        let received = receiver.recv().await.unwrap();
        assert_eq!(received.name, "async_test");
    }
}
