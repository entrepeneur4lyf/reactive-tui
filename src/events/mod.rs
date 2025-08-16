//! Event handling system

use crate::compat::{KeyEvent, MouseEvent};
use crate::error::Result;
use std::any::Any;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::mpsc;

pub mod actions;
pub mod focus;
pub mod keybinding;
pub mod messages;
pub mod routing;
pub mod targeting;

pub use actions::{
  Action, ActionBuilder, ActionCallback, ActionDispatcher, ActionHandler, ActionResult,
};
pub use focus::{FocusInfo, FocusManager, FocusableElement};
pub use keybinding::{
  ElementAction, KeyAction, KeyBindingManager, KeyBindingPreset, KeyBindingResult, KeyCombination,
  NavigationDirection,
};
pub use messages::{
  BlurMessage, ClickMessage, CustomMessage, FocusMessage, InputMessage, KeyPressMessage, Message,
  MessageEvent, MessageHandler, MessageManager, MountMessage, SubmitMessage, UnmountMessage,
};
pub use routing::{ComponentEventHandler, EventContext, EventPhase, EventRouter};
pub use targeting::{Bounds, ComponentTarget, MouseTargeting};

#[derive(Debug, Clone)]
pub enum Event {
  Key(KeyEvent),
  Mouse(MouseEvent),
  Resize(u16, u16),
  Touch(TouchEvent),
  Custom(String, serde_json::Value),
}

#[derive(Debug, Clone)]
pub struct TouchEvent {
  pub x: u16,
  pub y: u16,
  pub touch_type: TouchType,
  pub pressure: Option<f32>,
  pub id: u32,
}

#[derive(Debug, Clone, Copy)]
pub enum TouchType {
  Start,
  Move,
  End,
  Cancel,
}

#[derive(Debug, Clone)]
pub struct EventPropagation {
  pub prevent_default: bool,
  pub stop_propagation: bool,
  pub stop_immediate_propagation: bool,
}

impl EventPropagation {
  pub fn new() -> Self {
    Self {
      prevent_default: false,
      stop_propagation: false,
      stop_immediate_propagation: false,
    }
  }

  pub fn prevent_default(&mut self) {
    self.prevent_default = true;
  }

  pub fn stop_propagation(&mut self) {
    self.stop_propagation = true;
  }

  pub fn stop_immediate_propagation(&mut self) {
    self.stop_immediate_propagation = true;
    self.stop_propagation = true;
  }
}

pub type EventCallback = Box<dyn Fn(&Event, &mut EventPropagation) -> Result<()> + Send + Sync>;
pub type EventDelegateCallback = Box<dyn Fn(&Event, &str, &mut EventPropagation) -> Result<()> + Send + Sync>;

#[derive(Debug, Clone)]
pub struct EventListener {
  pub element_id: String,
  pub event_type: String,
  pub callback_id: String,
  pub use_capture: bool,
}

pub struct EventHandler {
  callbacks: HashMap<String, Vec<EventCallback>>,
  capture_callbacks: HashMap<String, Vec<EventCallback>>,
  delegate_callbacks: HashMap<String, Vec<EventDelegateCallback>>,
  event_listeners: Vec<EventListener>,
  event_sender: mpsc::UnboundedSender<Event>,
  event_receiver: Option<mpsc::UnboundedReceiver<Event>>,
  action_dispatcher: ActionDispatcher,
  message_manager: Arc<MessageManager>,
  event_router: Option<EventRouter>,
  custom_event_types: HashMap<String, Box<dyn Any + Send + Sync>>,
}

impl EventHandler {
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();

    Self {
      callbacks: HashMap::new(),
      capture_callbacks: HashMap::new(),
      delegate_callbacks: HashMap::new(),
      event_listeners: Vec::new(),
      event_sender: sender,
      event_receiver: Some(receiver),
      action_dispatcher: ActionDispatcher::new(),
      message_manager: Arc::new(MessageManager::new()),
      event_router: None,
      custom_event_types: HashMap::new(),
    }
  }

  /// Add event listener with bubbling phase (default)
  pub fn on<F>(&mut self, event_type: &str, callback: F)
  where
    F: Fn(&Event, &mut EventPropagation) -> Result<()> + Send + Sync + 'static,
  {
    self
      .callbacks
      .entry(event_type.to_string())
      .or_default()
      .push(Box::new(callback));
  }

  /// Add event listener with capture phase
  pub fn on_capture<F>(&mut self, event_type: &str, callback: F)
  where
    F: Fn(&Event, &mut EventPropagation) -> Result<()> + Send + Sync + 'static,
  {
    self
      .capture_callbacks
      .entry(event_type.to_string())
      .or_default()
      .push(Box::new(callback));
  }

  /// Add event delegation listener
  pub fn delegate<F>(&mut self, event_type: &str, callback: F)
  where
    F: Fn(&Event, &str, &mut EventPropagation) -> Result<()> + Send + Sync + 'static,
  {
    self
      .delegate_callbacks
      .entry(event_type.to_string())
      .or_default()
      .push(Box::new(callback));
  }

  /// Register a custom event type
  pub fn register_custom_event<T>(&mut self, event_name: &str, event_data: T)
  where
    T: Any + Send + Sync + 'static,
  {
    self.custom_event_types.insert(event_name.to_string(), Box::new(event_data));
  }

  /// Create and emit a custom event
  pub fn emit_custom_event(&self, event_name: &str, data: serde_json::Value) {
    let event = Event::Custom(event_name.to_string(), data);
    self.emit(event);
  }

  /// Add touch event support
  pub async fn handle_touch_event(&self, touch: TouchEvent) {
    let event = Event::Touch(touch);
    self.trigger_event_phases("touch", &event);
    self.emit(event);
  }

  pub fn emit(&self, event: Event) {
    let _ = self.event_sender.send(event);
  }

  pub async fn handle_key_event(&self, key: KeyEvent) {
    // Route through event router if available
    if let Some(router) = &self.event_router {
      if let Err(e) = router.route_key_event(key).await {
        eprintln!("Event routing error: {e}");
      }
    }

    let event = Event::Key(key);
    self.trigger_event_phases("key", &event);
    self.emit(event);
  }

  pub async fn handle_mouse_event(&self, mouse: MouseEvent) {
    // Route through event router if available
    if let Some(router) = &self.event_router {
      if let Err(e) = router.route_mouse_event(mouse).await {
        eprintln!("Event routing error: {e}");
      }
    }

    let event = Event::Mouse(mouse);
    self.trigger_event_phases("mouse", &event);
    self.emit(event);
  }

  pub async fn handle_resize_event(&self, width: u16, height: u16) {
    let event = Event::Resize(width, height);
    self.trigger_event_phases("resize", &event);
    self.emit(event);
  }

  /// Trigger event with proper capture and bubble phases
  fn trigger_event_phases(&self, event_type: &str, event: &Event) {
    let mut propagation = EventPropagation::new();

    // Capture phase
    if let Some(capture_callbacks) = self.capture_callbacks.get(event_type) {
      for callback in capture_callbacks {
        if propagation.stop_immediate_propagation {
          return;
        }
        if let Err(e) = callback(event, &mut propagation) {
          eprintln!("Event callback error (capture): {e}");
        }
        if propagation.stop_propagation {
          return;
        }
      }
    }

    // Target/Bubble phase
    if let Some(callbacks) = self.callbacks.get(event_type) {
      for callback in callbacks {
        if propagation.stop_immediate_propagation {
          return;
        }
        if let Err(e) = callback(event, &mut propagation) {
          eprintln!("Event callback error (bubble): {e}");
        }
        if propagation.stop_propagation {
          return;
        }
      }
    }

    // Event delegation
    if let Some(delegate_callbacks) = self.delegate_callbacks.get(event_type) {
      for callback in delegate_callbacks {
        if propagation.stop_immediate_propagation {
          return;
        }
        // For delegation, we'd need to determine the target element
        // This is a simplified version - in practice you'd get the target from the event
        let target_element = "unknown"; // Would be determined from event context
        if let Err(e) = callback(event, target_element, &mut propagation) {
          eprintln!("Event delegate callback error: {e}");
        }
        if propagation.stop_propagation {
          return;
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
    self
      .message_manager
      .build_hierarchy_from_element(element, parent_id)
  }

  /// Process queued messages
  pub fn process_message_queue(&self) -> crate::error::Result<()> {
    self.message_manager.process_queue()
  }

  /// Get the message manager for advanced usage
  pub fn message_manager(&self) -> &Arc<MessageManager> {
    &self.message_manager
  }

  /// Remove all handlers for a specific element (cleanup)
  pub fn remove_element_handlers(&self, element_id: &str) -> crate::error::Result<()> {
    self.message_manager.remove_element_handlers(element_id)
  }

  /// Initialize event router with focus manager - ACTUALLY SHARES MESSAGE MANAGER
  pub fn init_event_router(
    &mut self,
    focus_manager: std::sync::Arc<tokio::sync::RwLock<FocusManager>>,
  ) {
    // Share the EXACT SAME MessageManager instance - events route to working handlers
    self.event_router = Some(EventRouter::new(
      self.message_manager.clone(),
      focus_manager,
    ));
  }

  /// Update component bounds for mouse targeting
  pub async fn update_component_bounds(
    &self,
    element: &crate::components::Element,
    layout: &crate::layout::Layout,
  ) -> crate::error::Result<()> {
    if let Some(router) = &self.event_router {
      router.update_component_bounds(element, layout).await?;
    }
    Ok(())
  }

  /// Register a component event handler
  pub fn register_component_handler<F>(
    &mut self,
    element_id: String,
    event_type: String,
    handler: F,
  ) -> crate::error::Result<()>
  where
    F: Fn(&mut EventContext, &dyn Message) -> crate::error::Result<()> + Send + Sync + 'static,
  {
    if let Some(router) = &mut self.event_router {
      router.register_component_handler(element_id, event_type, handler)?;
    }
    Ok(())
  }

  /// Get access to the event router
  pub fn event_router(&self) -> Option<&EventRouter> {
    self.event_router.as_ref()
  }

  /// Get mutable access to the event router
  pub fn event_router_mut(&mut self) -> Option<&mut EventRouter> {
    self.event_router.as_mut()
  }

  /// Event performance monitoring
  pub fn profile_event_handling(&self, event_type: &str, event: &Event) -> EventProfile {
    let start_time = std::time::Instant::now();

    self.trigger_event_phases(event_type, event);

    let total_time = start_time.elapsed();

    EventProfile {
      event_type: event_type.to_string(),
      total_time,
      capture_listeners: self.capture_callbacks.get(event_type).map(|v| v.len()).unwrap_or(0),
      bubble_listeners: self.callbacks.get(event_type).map(|v| v.len()).unwrap_or(0),
      delegate_listeners: self.delegate_callbacks.get(event_type).map(|v| v.len()).unwrap_or(0),
    }
  }

  /// Debug event system state
  pub fn debug_event_system(&self) {
    println!("=== Event System Debug ===");
    println!("Bubble listeners:");
    for (event_type, callbacks) in &self.callbacks {
      println!("  {}: {} listeners", event_type, callbacks.len());
    }

    println!("Capture listeners:");
    for (event_type, callbacks) in &self.capture_callbacks {
      println!("  {}: {} listeners", event_type, callbacks.len());
    }

    println!("Delegate listeners:");
    for (event_type, callbacks) in &self.delegate_callbacks {
      println!("  {}: {} listeners", event_type, callbacks.len());
    }

    println!("Custom event types: {}", self.custom_event_types.len());
    println!("Event listeners: {}", self.event_listeners.len());
  }

  /// Remove event listeners for cleanup
  pub fn remove_listeners(&mut self, event_type: &str) {
    self.callbacks.remove(event_type);
    self.capture_callbacks.remove(event_type);
    self.delegate_callbacks.remove(event_type);
    self.event_listeners.retain(|listener| listener.event_type != event_type);
  }

  /// Remove all listeners for an element
  pub fn remove_element_listeners(&mut self, element_id: &str) {
    self.event_listeners.retain(|listener| listener.element_id != element_id);
  }

  /// Get event statistics
  pub fn get_event_stats(&self) -> EventStats {
    EventStats {
      total_bubble_listeners: self.callbacks.values().map(|v| v.len()).sum(),
      total_capture_listeners: self.capture_callbacks.values().map(|v| v.len()).sum(),
      total_delegate_listeners: self.delegate_callbacks.values().map(|v| v.len()).sum(),
      event_types: self.callbacks.keys().chain(self.capture_callbacks.keys())
        .chain(self.delegate_callbacks.keys()).cloned().collect(),
      custom_event_types: self.custom_event_types.keys().cloned().collect(),
    }
  }
}

#[derive(Debug, Clone)]
pub struct EventProfile {
  pub event_type: String,
  pub total_time: std::time::Duration,
  pub capture_listeners: usize,
  pub bubble_listeners: usize,
  pub delegate_listeners: usize,
}

#[derive(Debug, Clone)]
pub struct EventStats {
  pub total_bubble_listeners: usize,
  pub total_capture_listeners: usize,
  pub total_delegate_listeners: usize,
  pub event_types: Vec<String>,
  pub custom_event_types: Vec<String>,
}

impl Default for EventHandler {
  fn default() -> Self {
    Self::new()
  }
}
