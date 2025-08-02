//! Event handling system

use crate::error::Result;
use crossterm::event::{KeyEvent, MouseEvent};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub mod actions;
pub mod focus;
pub mod keybinding;
pub mod messages;

pub use actions::{
    Action, ActionBuilder, ActionCallback, ActionDispatcher, ActionHandler, ActionResult,
};
pub use focus::{FocusInfo, FocusManager, FocusableElement};
pub use keybinding::{
    ElementAction, KeyAction, KeyBindingManager, KeyBindingPreset, KeyBindingResult,
    KeyCombination, NavigationDirection,
};
pub use messages::{
    BlurMessage, ClickMessage, CustomMessage, FocusMessage, InputMessage, KeyPressMessage, Message,
    MessageEvent, MessageHandler, MessageManager, MountMessage, SubmitMessage, UnmountMessage,
};

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    Resize(u16, u16),
    Custom(String, serde_json::Value),
}

pub type EventCallback = Box<dyn Fn(&Event) -> bool + Send + Sync>;

pub struct EventHandler {
    callbacks: HashMap<String, Vec<EventCallback>>,
    event_sender: mpsc::UnboundedSender<Event>,
    event_receiver: Option<mpsc::UnboundedReceiver<Event>>,
    action_dispatcher: ActionDispatcher,
    message_manager: MessageManager,
}

impl EventHandler {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::unbounded_channel();

        Self {
            callbacks: HashMap::new(),
            event_sender: sender,
            event_receiver: Some(receiver),
            action_dispatcher: ActionDispatcher::new(),
            message_manager: MessageManager::new(),
        }
    }

    pub fn on<F>(&mut self, event_type: &str, callback: F)
    where
        F: Fn(&Event) -> bool + Send + Sync + 'static,
    {
        self.callbacks
            .entry(event_type.to_string())
            .or_default()
            .push(Box::new(callback));
    }

    pub fn emit(&self, event: Event) {
        let _ = self.event_sender.send(event);
    }

    pub async fn handle_key_event(&self, key: KeyEvent) {
        let event = Event::Key(key);
        self.trigger_callbacks("key", &event);
        self.emit(event);
    }

    pub async fn handle_mouse_event(&self, mouse: MouseEvent) {
        let event = Event::Mouse(mouse);
        self.trigger_callbacks("mouse", &event);
        self.emit(event);
    }

    pub async fn handle_resize_event(&self, width: u16, height: u16) {
        let event = Event::Resize(width, height);
        self.trigger_callbacks("resize", &event);
        self.emit(event);
    }

    fn trigger_callbacks(&self, event_type: &str, event: &Event) {
        if let Some(callbacks) = self.callbacks.get(event_type) {
            for callback in callbacks {
                if callback(event) {
                    // If callback returns true, stop propagation
                    break;
                }
            }
        }
    }

    pub fn take_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<Event>> {
        self.event_receiver.take()
    }

    /// Register an action handler
    pub fn register_action<F>(&mut self, action_name: &str, handler: F)
    where
        F: Fn(&mut Action) -> ActionResult + Send + Sync + 'static,
    {
        self.action_dispatcher.register(action_name, handler);
    }

    /// Dispatch an action
    pub fn dispatch_action(&self, action: Action) -> ActionResult {
        self.action_dispatcher.dispatch(action)
    }

    /// Send an action for async processing
    pub fn send_action(&self, action: Action) -> Result<()> {
        self.action_dispatcher.send(action)
    }

    /// Create an action builder
    pub fn action<S: Into<String>>(&self, name: S) -> ActionBuilder {
        self.action_dispatcher.action(name)
    }

    /// Get the action dispatcher for advanced usage
    pub fn action_dispatcher(&self) -> &ActionDispatcher {
        &self.action_dispatcher
    }

    /// Get mutable access to the action dispatcher
    pub fn action_dispatcher_mut(&mut self) -> &mut ActionDispatcher {
        &mut self.action_dispatcher
    }

    /// Register a message handler for a specific message type
    pub fn on_message<T, F>(&self, handler: F) -> crate::error::Result<()>
    where
        T: Message + 'static,
        F: Fn(&mut MessageEvent) -> crate::error::Result<()> + Send + Sync + 'static,
    {
        self.message_manager.on::<T, _>(handler)
    }

    /// Register a message handler for a specific element
    pub fn on_element_message<T, F>(&self, element_id: &str, handler: F) -> crate::error::Result<()>
    where
        T: Message + 'static,
        F: Fn(&mut MessageEvent) -> crate::error::Result<()> + Send + Sync + 'static,
    {
        self.message_manager.on_element::<T, _>(element_id, handler)
    }

    /// Send a message from a specific element
    pub fn send_message_from(
        &self,
        element_id: Option<String>,
        message: impl Message,
    ) -> crate::error::Result<()> {
        self.message_manager.send_from(element_id, message)
    }

    /// Send a message (convenience method)
    pub fn send_message(&self, message: impl Message) -> crate::error::Result<()> {
        self.message_manager.send(message)
    }

    /// Update the component hierarchy for message bubbling
    pub fn update_hierarchy(
        &self,
        child_id: &str,
        parent_id: Option<String>,
    ) -> crate::error::Result<()> {
        self.message_manager.update_hierarchy(child_id, parent_id)
    }

    /// Build hierarchy from an element tree
    pub fn build_hierarchy_from_element(
        &self,
        element: &crate::components::Element,
        parent_id: Option<String>,
    ) -> crate::error::Result<()> {
        self.message_manager
            .build_hierarchy_from_element(element, parent_id)
    }

    /// Process queued messages
    pub fn process_message_queue(&self) -> crate::error::Result<()> {
        self.message_manager.process_queue()
    }

    /// Get the message manager for advanced usage
    pub fn message_manager(&self) -> &MessageManager {
        &self.message_manager
    }

    /// Get mutable access to the message manager
    pub fn message_manager_mut(&mut self) -> &mut MessageManager {
        &mut self.message_manager
    }

    /// Remove all handlers for a specific element (cleanup)
    pub fn remove_element_handlers(&self, element_id: &str) -> crate::error::Result<()> {
        self.message_manager.remove_element_handlers(element_id)
    }
}

impl Default for EventHandler {
    fn default() -> Self {
        Self::new()
    }
}
