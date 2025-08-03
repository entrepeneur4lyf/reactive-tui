//! Message-based event system with bubbling
//!
//! This module provides a sophisticated message passing system inspired by DOM events,
//! allowing components to communicate through typed messages that can bubble up
//! the component hierarchy.

use crate::{
  components::Element,
  error::{Result, TuiError},
};
use serde::{Deserialize, Serialize};
use std::{
  any::{Any, TypeId},
  collections::HashMap,
  fmt,
  sync::{Arc, RwLock},
};
use tokio::sync::mpsc;

/// Trait for messages that can be sent between components
pub trait Message: Any + Send + Sync + fmt::Debug {
  /// Get the message type name for debugging
  fn type_name(&self) -> &'static str {
    std::any::type_name::<Self>()
  }

  /// Clone the message as a boxed trait object
  fn clone_message(&self) -> Box<dyn Message>;

  /// Check if this message should bubble up the component tree
  fn should_bubble(&self) -> bool {
    true
  }

  /// Check if this message can be prevented from propagating
  fn can_prevent_default(&self) -> bool {
    true
  }
}

/// Message event that carries context about where and how it was sent
#[derive(Debug)]
pub struct MessageEvent {
  /// The actual message data
  pub message: Arc<dyn Message>,
  /// ID of the element that originally sent the message
  pub sender_id: Option<String>,
  /// Path of element IDs from sender to current handler
  pub path: Vec<String>,
  /// Whether default behavior should be prevented
  pub prevent_default: bool,
  /// Whether the message should stop bubbling
  pub stop_propagation: bool,
  /// Whether the message should stop immediate propagation (within same element)
  pub stop_immediate_propagation: bool,
  /// Timestamp when the message was created
  pub timestamp: std::time::Instant,
}

impl MessageEvent {
  /// Create a new message event
  pub fn new(message: impl Message, sender_id: Option<String>) -> Self {
    Self {
      message: Arc::new(message),
      sender_id,
      path: Vec::new(),
      prevent_default: false,
      stop_propagation: false,
      stop_immediate_propagation: false,
      timestamp: std::time::Instant::now(),
    }
  }

  /// Prevent the default behavior for this message
  pub fn prevent_default(&mut self) {
    if self.message.can_prevent_default() {
      self.prevent_default = true;
    }
  }

  /// Stop the message from bubbling further up the tree
  pub fn stop_propagation(&mut self) {
    self.stop_propagation = true;
  }

  /// Stop immediate propagation (within the same element)
  pub fn stop_immediate_propagation(&mut self) {
    self.stop_immediate_propagation = true;
  }

  /// Check if the message is of a specific type
  pub fn is<T: Message + 'static>(&self) -> bool {
    self.message.as_ref().type_id() == TypeId::of::<T>()
  }

  /// Try to downcast the message to a specific type
  pub fn downcast<T: Message + 'static>(&self) -> Option<&T> {
    // We need to cast to Any first, then downcast
    let any_ref: &dyn Any = self.message.as_ref();
    any_ref.downcast_ref::<T>()
  }

  /// Get the age of this message
  pub fn age(&self) -> std::time::Duration {
    self.timestamp.elapsed()
  }
}

/// Handler function for messages
pub type MessageHandler = Box<dyn Fn(&mut MessageEvent) -> Result<()> + Send + Sync>;

/// Type alias for element handlers map to reduce complexity
pub type ElementHandlersMap = HashMap<String, HashMap<TypeId, Vec<MessageHandler>>>;

/// Message listener that can be registered for specific message types
#[derive(Debug)]
pub struct MessageListener {
  /// Unique ID for this listener
  pub id: String,
  /// Element ID this listener is attached to (None for global listeners)
  pub element_id: Option<String>,
  /// Message type this listener responds to
  pub message_type: TypeId,
  /// Whether this listener should only fire once
  pub once: bool,
  /// Priority of this listener (higher values execute first)
  pub priority: i32,
  /// Whether this listener has been triggered and should be removed
  pub triggered: bool,
}

/// Manager for the message-based event system
pub struct MessageManager {
  /// Message handlers organized by type
  handlers: Arc<RwLock<HashMap<TypeId, Vec<MessageHandler>>>>,
  /// Element-specific handlers
  element_handlers: Arc<RwLock<ElementHandlersMap>>,
  /// Message listeners
  listeners: Arc<RwLock<Vec<MessageListener>>>,
  /// Channel for async message processing
  message_sender: mpsc::UnboundedSender<MessageEvent>,
  message_receiver: Option<mpsc::UnboundedReceiver<MessageEvent>>,
  /// Component hierarchy for bubbling
  hierarchy: Arc<RwLock<HashMap<String, Option<String>>>>, // child_id -> parent_id
  /// Message queue for batch processing
  message_queue: Arc<RwLock<Vec<MessageEvent>>>,
}

impl MessageManager {
  /// Create a new message manager
  pub fn new() -> Self {
    let (sender, receiver) = mpsc::unbounded_channel();

    Self {
      handlers: Arc::new(RwLock::new(HashMap::new())),
      element_handlers: Arc::new(RwLock::new(HashMap::new())),
      listeners: Arc::new(RwLock::new(Vec::new())),
      message_sender: sender,
      message_receiver: Some(receiver),
      hierarchy: Arc::new(RwLock::new(HashMap::new())),
      message_queue: Arc::new(RwLock::new(Vec::new())),
    }
  }

  /// Register a global message handler for a specific message type
  pub fn on<T, F>(&self, handler: F) -> Result<()>
  where
    T: Message + 'static,
    F: Fn(&mut MessageEvent) -> Result<()> + Send + Sync + 'static,
  {
    let type_id = TypeId::of::<T>();
    let mut handlers = self
      .handlers
      .write()
      .map_err(|_| TuiError::component("Failed to acquire handlers lock".to_string()))?;

    handlers.entry(type_id).or_default().push(Box::new(handler));

    Ok(())
  }

  /// Register a message handler for a specific element
  pub fn on_element<T, F>(&self, element_id: &str, handler: F) -> Result<()>
  where
    T: Message + 'static,
    F: Fn(&mut MessageEvent) -> Result<()> + Send + Sync + 'static,
  {
    let type_id = TypeId::of::<T>();
    let mut element_handlers = self
      .element_handlers
      .write()
      .map_err(|_| TuiError::component("Failed to acquire element handlers lock".to_string()))?;

    element_handlers
      .entry(element_id.to_string())
      .or_default()
      .entry(type_id)
      .or_default()
      .push(Box::new(handler));

    Ok(())
  }

  /// Send a message from a specific element
  pub fn send_from(&self, element_id: Option<String>, message: impl Message) -> Result<()> {
    let event = MessageEvent::new(message, element_id);
    // Process immediately for synchronous operation
    self.process_message(event)?;
    Ok(())
  }

  /// Send a message (convenience method)
  pub fn send(&self, message: impl Message) -> Result<()> {
    self.send_from(None, message)
  }

  /// Process a message event, including bubbling
  pub fn process_message(&self, mut event: MessageEvent) -> Result<()> {
    let message_type = self.get_message_type_id(&event);

    // Start with the sender element if specified
    let mut current_element = event.sender_id.clone();

    while let Some(element_id) = current_element {
      // Add to path for debugging
      event.path.push(element_id.clone());

      // Handle element-specific handlers first
      self.handle_element_message(&element_id, &mut event, message_type)?;

      // Check if propagation should stop
      if event.stop_propagation || event.stop_immediate_propagation {
        break;
      }

      // Check if the message should bubble
      if !event.message.should_bubble() {
        break;
      }

      // Move to parent element
      current_element = {
        let hierarchy = self
          .hierarchy
          .read()
          .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;
        hierarchy.get(&element_id).cloned().flatten()
      };
    }

    // Handle global handlers if not stopped
    if !event.stop_propagation {
      self.handle_global_message(&mut event, message_type)?;
    }

    Ok(())
  }

  /// Handle message for a specific element
  fn handle_element_message(
    &self,
    element_id: &str,
    event: &mut MessageEvent,
    message_type: TypeId,
  ) -> Result<()> {
    let element_handlers = self
      .element_handlers
      .read()
      .map_err(|_| TuiError::component("Failed to acquire element handlers lock".to_string()))?;

    if let Some(handlers_map) = element_handlers.get(element_id) {
      if let Some(handlers) = handlers_map.get(&message_type) {
        for handler in handlers {
          if event.stop_immediate_propagation {
            break;
          }
          handler(event)?;
        }
      }
    }

    Ok(())
  }

  /// Handle global message handlers
  fn handle_global_message(&self, event: &mut MessageEvent, message_type: TypeId) -> Result<()> {
    let handlers = self
      .handlers
      .read()
      .map_err(|_| TuiError::component("Failed to acquire handlers lock".to_string()))?;

    if let Some(handlers) = handlers.get(&message_type) {
      for handler in handlers {
        if event.stop_immediate_propagation {
          break;
        }
        handler(event)?;
      }
    }

    Ok(())
  }

  /// Update the component hierarchy for bubbling
  pub fn update_hierarchy(&self, child_id: &str, parent_id: Option<String>) -> Result<()> {
    let mut hierarchy = self
      .hierarchy
      .write()
      .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;

    hierarchy.insert(child_id.to_string(), parent_id);
    Ok(())
  }

  /// Build hierarchy from an element tree
  pub fn build_hierarchy_from_element(
    &self,
    element: &Element,
    parent_id: Option<String>,
  ) -> Result<()> {
    if let Some(id) = &element.id {
      self.update_hierarchy(id, parent_id.clone())?;

      // Recursively build hierarchy for children
      for child in &element.children {
        self.build_hierarchy_from_element(child, Some(id.clone()))?;
      }
    } else {
      // If element doesn't have an ID, process children with the same parent
      for child in &element.children {
        self.build_hierarchy_from_element(child, parent_id.clone())?;
      }
    }

    Ok(())
  }

  /// Get the message receiver for async processing
  pub fn take_receiver(&mut self) -> Option<mpsc::UnboundedReceiver<MessageEvent>> {
    self.message_receiver.take()
  }

  /// Add a message to the queue for batch processing
  pub fn queue_message(&self, message: impl Message, sender_id: Option<String>) -> Result<()> {
    let event = MessageEvent::new(message, sender_id);
    let mut queue = self
      .message_queue
      .write()
      .map_err(|_| TuiError::component("Failed to acquire message queue lock".to_string()))?;

    queue.push(event);
    Ok(())
  }

  /// Send message to async channel for processing
  pub fn send_async(&self, message: impl Message, sender_id: Option<String>) -> Result<()> {
    let event = MessageEvent::new(message, sender_id);
    self
      .message_sender
      .send(event)
      .map_err(|_| TuiError::component("Failed to send message to async channel".to_string()))?;
    Ok(())
  }

  /// Add a message listener with priority and configuration
  pub fn add_listener(&self, listener: MessageListener) -> Result<()> {
    let mut listeners = self
      .listeners
      .write()
      .map_err(|_| TuiError::component("Failed to acquire listeners lock".to_string()))?;

    listeners.push(listener);
    // Sort by priority (higher values first)
    listeners.sort_by(|a, b| b.priority.cmp(&a.priority));
    Ok(())
  }

  /// Remove listeners by ID
  pub fn remove_listener(&self, listener_id: &str) -> Result<bool> {
    let mut listeners = self
      .listeners
      .write()
      .map_err(|_| TuiError::component("Failed to acquire listeners lock".to_string()))?;

    let original_len = listeners.len();
    listeners.retain(|l| l.id != listener_id);
    Ok(listeners.len() != original_len)
  }

  /// Process async messages from the channel
  pub async fn process_async_messages(&mut self) -> Result<()> {
    if let Some(mut receiver) = self.message_receiver.take() {
      while let Some(event) = receiver.recv().await {
        self.process_message(event)?;
      }
    }
    Ok(())
  }

  /// Process all queued messages
  pub fn process_queue(&self) -> Result<()> {
    let messages = {
      let mut queue = self
        .message_queue
        .write()
        .map_err(|_| TuiError::component("Failed to acquire message queue lock".to_string()))?;
      // Move out the messages
      std::mem::take(&mut *queue)
    };

    for event in messages {
      self.process_message(event)?;
    }

    Ok(())
  }

  /// Get the TypeId of a message
  fn get_message_type_id(&self, event: &MessageEvent) -> TypeId {
    event.message.as_ref().type_id()
  }

  /// Remove all handlers for a specific element (cleanup)
  pub fn remove_element_handlers(&self, element_id: &str) -> Result<()> {
    let mut element_handlers = self
      .element_handlers
      .write()
      .map_err(|_| TuiError::component("Failed to acquire element handlers lock".to_string()))?;

    element_handlers.remove(element_id);

    let mut hierarchy = self
      .hierarchy
      .write()
      .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;

    hierarchy.remove(element_id);

    Ok(())
  }

  /// Get hierarchy information for debugging
  pub fn get_hierarchy(&self) -> Result<HashMap<String, Option<String>>> {
    let hierarchy = self
      .hierarchy
      .read()
      .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;

    Ok(hierarchy.clone())
  }

  /// Get the path from an element to the root
  pub fn get_element_path(&self, element_id: &str) -> Result<Vec<String>> {
    let hierarchy = self
      .hierarchy
      .read()
      .map_err(|_| TuiError::component("Failed to acquire hierarchy lock".to_string()))?;

    let mut path = Vec::new();
    let mut current = Some(element_id.to_string());

    while let Some(id) = current {
      path.push(id.clone());
      current = hierarchy.get(&id).cloned().flatten();
    }

    path.reverse(); // Start from root
    Ok(path)
  }
}

impl Default for MessageManager {
  fn default() -> Self {
    Self::new()
  }
}

/// Common message types for the TUI system
///
/// Message sent when an element is clicked
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClickMessage {
  pub x: u16,
  pub y: u16,
  pub button: String,
}

impl Message for ClickMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Message sent when an element gains focus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FocusMessage {
  pub element_id: String,
  pub previous_focus: Option<String>,
}

impl Message for FocusMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Message sent when an element loses focus
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlurMessage {
  pub element_id: String,
  pub next_focus: Option<String>,
}

impl Message for BlurMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Message sent when text input changes
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputMessage {
  pub element_id: String,
  pub value: String,
  pub previous_value: String,
}

impl Message for InputMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Message sent when form is submitted
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubmitMessage {
  pub form_id: String,
  pub data: HashMap<String, String>,
}

impl Message for SubmitMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }

  fn should_bubble(&self) -> bool {
    true // Forms typically bubble to parent containers
  }
}

/// Message sent when a key is pressed within an element
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyPressMessage {
  pub element_id: String,
  pub key: String,
  pub modifiers: Vec<String>,
}

impl Message for KeyPressMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Message sent when an element is mounted to the DOM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MountMessage {
  pub element_id: String,
}

impl Message for MountMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }

  fn should_bubble(&self) -> bool {
    false // Mount messages don't typically bubble
  }
}

/// Message sent when an element is unmounted from the DOM
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnmountMessage {
  pub element_id: String,
}

impl Message for UnmountMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }

  fn should_bubble(&self) -> bool {
    false // Unmount messages don't typically bubble
  }
}

/// Custom message for application-specific events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CustomMessage {
  pub name: String,
  pub data: serde_json::Value,
  pub bubble: bool,
}

impl Message for CustomMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }

  fn should_bubble(&self) -> bool {
    self.bubble
  }
}

impl CustomMessage {
  pub fn new(name: impl Into<String>, data: impl Serialize) -> Result<Self> {
    Ok(Self {
      name: name.into(),
      data: serde_json::to_value(data).map_err(|e| {
        TuiError::component(format!("Failed to serialize custom message data: {e}"))
      })?,
      bubble: true,
    })
  }

  pub fn no_bubble(mut self) -> Self {
    self.bubble = false;
    self
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::sync::{Arc, Mutex};

  #[derive(Debug, Clone)]
  struct TestMessage {
    content: String,
  }

  impl Message for TestMessage {
    fn clone_message(&self) -> Box<dyn Message> {
      Box::new(self.clone())
    }
  }

  #[tokio::test]
  async fn test_message_handler_registration() {
    let manager = MessageManager::new();
    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();

    manager
      .on::<TestMessage, _>(move |event| {
        if let Some(msg) = event.downcast::<TestMessage>() {
          received_clone.lock().unwrap().push(msg.content.clone());
        }
        Ok(())
      })
      .unwrap();

    let event = MessageEvent::new(
      TestMessage {
        content: "test".to_string(),
      },
      None,
    );
    manager.process_message(event).unwrap();

    assert_eq!(received.lock().unwrap()[0], "test");
  }

  #[tokio::test]
  async fn test_message_bubbling() {
    let manager = MessageManager::new();

    // Set up hierarchy: child -> parent -> root
    manager
      .update_hierarchy("child", Some("parent".to_string()))
      .unwrap();
    manager
      .update_hierarchy("parent", Some("root".to_string()))
      .unwrap();
    manager.update_hierarchy("root", None).unwrap();

    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();

    // Register handlers for each element
    manager
      .on_element::<TestMessage, _>("child", {
        let received = received_clone.clone();
        move |_| {
          received.lock().unwrap().push("child".to_string());
          Ok(())
        }
      })
      .unwrap();

    manager
      .on_element::<TestMessage, _>("parent", {
        let received = received_clone.clone();
        move |_| {
          received.lock().unwrap().push("parent".to_string());
          Ok(())
        }
      })
      .unwrap();

    manager
      .on_element::<TestMessage, _>("root", {
        let received = received_clone.clone();
        move |_| {
          received.lock().unwrap().push("root".to_string());
          Ok(())
        }
      })
      .unwrap();

    // Send message from child
    let event = MessageEvent::new(
      TestMessage {
        content: "bubble test".to_string(),
      },
      Some("child".to_string()),
    );
    manager.process_message(event).unwrap();

    let received_messages = received.lock().unwrap();
    assert_eq!(
      *received_messages,
      vec![
        "child".to_string(),
        "parent".to_string(),
        "root".to_string()
      ]
    );
  }

  #[tokio::test]
  async fn test_stop_propagation() {
    let manager = MessageManager::new();

    // Set up hierarchy
    manager
      .update_hierarchy("child", Some("parent".to_string()))
      .unwrap();
    manager.update_hierarchy("parent", None).unwrap();

    let received = Arc::new(Mutex::new(Vec::new()));
    let received_clone = received.clone();

    // Child handler stops propagation
    manager
      .on_element::<TestMessage, _>("child", {
        let received = received_clone.clone();
        move |event| {
          received.lock().unwrap().push("child".to_string());
          event.stop_propagation();
          Ok(())
        }
      })
      .unwrap();

    manager
      .on_element::<TestMessage, _>("parent", {
        let received = received_clone.clone();
        move |_| {
          received.lock().unwrap().push("parent".to_string());
          Ok(())
        }
      })
      .unwrap();

    // Send message from child
    let event = MessageEvent::new(
      TestMessage {
        content: "stop test".to_string(),
      },
      Some("child".to_string()),
    );
    manager.process_message(event).unwrap();

    let received_messages = received.lock().unwrap();
    assert_eq!(*received_messages, vec!["child".to_string()]); // Parent should not receive
  }
}
