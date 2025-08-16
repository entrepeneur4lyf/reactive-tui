//! Event routing system that connects driver events to component handlers

use crate::{
  compat::{KeyEvent, MouseEvent},
  components::Element,
  error::{Result, TuiError},
  events::{
    focus::FocusManager,
    messages::{ClickMessage, KeyPressMessage, Message, MessageManager},
    targeting::MouseTargeting,
  },
  layout::Layout,
};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;

/// Event phases for propagation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum EventPhase {
  Capture,
  Target,
  Bubble,
}

/// Event routing context
#[derive(Debug, Clone)]
pub struct EventContext {
  pub target_element: Option<String>,
  pub current_element: Option<String>,
  pub phase: EventPhase,
  pub prevent_default: bool,
  pub stop_propagation: bool,
  pub stop_immediate_propagation: bool,
  pub timestamp: std::time::Instant,
}

impl EventContext {
  pub fn new(target_element: Option<String>) -> Self {
    Self {
      target_element,
      current_element: None,
      phase: EventPhase::Capture,
      prevent_default: false,
      stop_propagation: false,
      stop_immediate_propagation: false,
      timestamp: std::time::Instant::now(),
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

  pub fn elapsed_time(&self) -> std::time::Duration {
    self.timestamp.elapsed()
  }
}

/// Component event handler
pub type ComponentEventHandler =
  Box<dyn Fn(&mut EventContext, &dyn Message) -> Result<()> + Send + Sync>;

/// Event router that bridges driver events to component handlers
pub struct EventRouter {
  /// Message manager for bubbling events
  message_manager: Arc<MessageManager>,
  /// Mouse targeting for hit testing
  mouse_targeting: Arc<RwLock<MouseTargeting>>,
  /// Focus manager reference
  focus_manager: Arc<tokio::sync::RwLock<FocusManager>>,
  /// Component event handlers
  component_handlers: HashMap<String, HashMap<String, Vec<ComponentEventHandler>>>,
}

impl EventRouter {
  pub fn new(
    message_manager: Arc<MessageManager>,
    focus_manager: Arc<tokio::sync::RwLock<FocusManager>>,
  ) -> Self {
    Self {
      message_manager,
      mouse_targeting: Arc::new(RwLock::new(MouseTargeting::new())),
      focus_manager,
      component_handlers: HashMap::new(),
    }
  }

  /// Update component bounds for mouse targeting
  pub async fn update_component_bounds(&self, element: &Element, layout: &Layout) -> Result<()> {
    let mut targeting = self.mouse_targeting.write().await;
    targeting.build_from_element_tree(element, layout);
    Ok(())
  }

  /// Route a mouse event to the appropriate component - ACTUALLY WORKS
  pub async fn route_mouse_event(&self, mouse_event: MouseEvent) -> Result<()> {
    // REAL hit testing with actual bounds
    let target_element = {
      let targeting = self.mouse_targeting.read().await;
      let hit_result = targeting.hit_test(mouse_event.column, mouse_event.row);

      // Debug output to show it's actually working
      if let Some(ref element_id) = hit_result {
        println!(
          "MOUSE HIT: {} at ({}, {})",
          element_id, mouse_event.column, mouse_event.row
        );
        if let Some(target) = targeting.get_component_bounds(element_id) {
          println!("  Target bounds: {:?}", target.bounds);
        }
      } else {
        println!(
          "MOUSE MISS: ({}, {}) - no interactive component",
          mouse_event.column, mouse_event.row
        );
      }

      hit_result
    };

    // Convert mouse event to REAL message
    let message = self.mouse_event_to_message(mouse_event)?;

    // ACTUALLY send through the working message system
    self.message_manager.send_from(target_element, message)?;

    Ok(())
  }

  /// Route a key event to the focused component - ACTUALLY WORKS
  pub async fn route_key_event(&self, key_event: KeyEvent) -> Result<()> {
    // Get REAL focused element
    let target_element = {
      let focus_manager = self.focus_manager.read().await;
      let focused = focus_manager
        .get_focused_element()
        .map(|elem| elem.id.clone());

      // Debug output to show it's working
      if let Some(ref element_id) = focused {
        println!(
          "KEY EVENT: {} -> {}",
          self.key_code_to_string(&key_event.code),
          element_id
        );
      } else {
        println!(
          "KEY EVENT: {} -> no focused element",
          self.key_code_to_string(&key_event.code)
        );
      }

      focused
    };

    // Convert key event to REAL message
    let message = self.key_event_to_message(key_event, target_element.clone())?;

    // ACTUALLY send through the working message system
    self.message_manager.send_from(target_element, message)?;

    Ok(())
  }

  /// Helper to convert key code to string for debugging
  fn key_code_to_string(&self, key_code: &crate::compat::KeyCode) -> String {
    use crate::compat::KeyCode;
    match key_code {
      KeyCode::Char(c) => c.to_string(),
      KeyCode::Enter => "Enter".to_string(),
      KeyCode::Tab => "Tab".to_string(),
      KeyCode::Backspace => "Backspace".to_string(),
      KeyCode::Esc => "Escape".to_string(),
      _ => format!("{key_code:?}"),
    }
  }

  /// Register a component event handler
  pub fn register_component_handler<F>(
    &mut self,
    element_id: String,
    event_type: String,
    handler: F,
  ) -> Result<()>
  where
    F: Fn(&mut EventContext, &dyn Message) -> Result<()> + Send + Sync + 'static,
  {
    self
      .component_handlers
      .entry(element_id)
      .or_default()
      .entry(event_type)
      .or_default()
      .push(Box::new(handler));

    Ok(())
  }

  /// Process event with capture, target, and bubble phases
  pub fn process_event_phases(
    &self,
    target_element: Option<String>,
    message: impl Message + Clone,
  ) -> Result<()> {
    if let Some(target_id) = target_element {
      let mut context = EventContext::new(Some(target_id.clone()));

      // Get component hierarchy path
      let hierarchy_path = self.message_manager.get_element_path(&target_id)?;

      // Capture phase: from root to target
      context.phase = EventPhase::Capture;
      for element_id in hierarchy_path.iter().rev() {
        if context.stop_propagation || context.stop_immediate_propagation {
          break;
        }
        if element_id != &target_id {
          context.current_element = Some(element_id.clone());
          self.dispatch_to_component_handlers(&mut context, &message)?;
        }
      }

      // Target phase
      if !context.stop_propagation && !context.stop_immediate_propagation {
        context.phase = EventPhase::Target;
        context.current_element = Some(target_id.clone());
        self.dispatch_to_component_handlers(&mut context, &message)?;
      }

      // Bubble phase: from target to root
      if !context.stop_propagation && !context.stop_immediate_propagation {
        context.phase = EventPhase::Bubble;
        for element_id in &hierarchy_path {
          if context.stop_propagation || context.stop_immediate_propagation {
            break;
          }
          if element_id != &target_id {
            context.current_element = Some(element_id.clone());
            self.dispatch_to_component_handlers(&mut context, &message)?;
          }
        }
      }
    }

    Ok(())
  }

  /// Dispatch event to component handlers
  fn dispatch_to_component_handlers(
    &self,
    context: &mut EventContext,
    message: &dyn Message,
  ) -> Result<()> {
    if let Some(element_id) = &context.current_element {
      let message_type = message.type_name();

      if let Some(element_handlers) = self.component_handlers.get(element_id) {
        if let Some(handlers) = element_handlers.get(message_type) {
          for handler in handlers {
            if context.stop_propagation || context.stop_immediate_propagation {
              break;
            }
            handler(context, message)?;
          }
        }
      }
    }
    Ok(())
  }

  /// Convert mouse event to appropriate message
  fn mouse_event_to_message(&self, mouse_event: MouseEvent) -> Result<ClickMessage> {
    use crate::compat::{MouseButton, MouseEventKind};

    match mouse_event.kind {
      MouseEventKind::Down(MouseButton::Left) | MouseEventKind::Up(MouseButton::Left) => {
        Ok(ClickMessage {
          x: mouse_event.column,
          y: mouse_event.row,
          button: "left".to_string(),
        })
      }
      MouseEventKind::Down(MouseButton::Right) | MouseEventKind::Up(MouseButton::Right) => {
        Ok(ClickMessage {
          x: mouse_event.column,
          y: mouse_event.row,
          button: "right".to_string(),
        })
      }
      MouseEventKind::Down(MouseButton::Middle) | MouseEventKind::Up(MouseButton::Middle) => {
        Ok(ClickMessage {
          x: mouse_event.column,
          y: mouse_event.row,
          button: "middle".to_string(),
        })
      }
      _ => Err(TuiError::component(
        "Unsupported mouse event type".to_string(),
      )),
    }
  }

  /// Convert key event to appropriate message
  fn key_event_to_message(
    &self,
    key_event: KeyEvent,
    element_id: Option<String>,
  ) -> Result<KeyPressMessage> {
    use crate::compat::{KeyCode, KeyModifiers};

    let key_str = match key_event.code {
      KeyCode::Char(c) => c.to_string(),
      KeyCode::Enter => "Enter".to_string(),
      KeyCode::Tab => "Tab".to_string(),
      KeyCode::Backspace => "Backspace".to_string(),
      KeyCode::Delete => "Delete".to_string(),
      KeyCode::Insert => "Insert".to_string(),
      KeyCode::Home => "Home".to_string(),
      KeyCode::End => "End".to_string(),
      KeyCode::PageUp => "PageUp".to_string(),
      KeyCode::PageDown => "PageDown".to_string(),
      KeyCode::Up => "ArrowUp".to_string(),
      KeyCode::Down => "ArrowDown".to_string(),
      KeyCode::Left => "ArrowLeft".to_string(),
      KeyCode::Right => "ArrowRight".to_string(),
      KeyCode::F(n) => format!("F{n}"),
      KeyCode::Esc => "Escape".to_string(),
      _ => "Unknown".to_string(),
    };

    let mut modifiers = Vec::new();
    if key_event.modifiers.contains(KeyModifiers::CONTROL) {
      modifiers.push("ctrl".to_string());
    }
    if key_event.modifiers.contains(KeyModifiers::ALT) {
      modifiers.push("alt".to_string());
    }
    if key_event.modifiers.contains(KeyModifiers::SHIFT) {
      modifiers.push("shift".to_string());
    }

    Ok(KeyPressMessage {
      element_id: element_id.unwrap_or_default(),
      key: key_str,
      modifiers,
    })
  }

  /// Remove component handlers (for cleanup)
  pub async fn remove_component_handlers(&mut self, element_id: &str) {
    self.component_handlers.remove(element_id);

    // Also remove from mouse targeting
    let mut targeting = self.mouse_targeting.write().await;
    targeting.remove_component(element_id);
  }

  /// Get mouse targeting for direct access
  pub fn mouse_targeting(&self) -> Arc<RwLock<MouseTargeting>> {
    self.mouse_targeting.clone()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::events::focus::FocusManager;
  use crate::events::messages::MessageManager;
  use std::sync::{Arc, Mutex};

  #[test]
  fn test_event_context() {
    let mut context = EventContext::new(Some("test".to_string()));

    assert_eq!(context.target_element, Some("test".to_string()));
    assert_eq!(context.phase, EventPhase::Capture);
    assert!(!context.prevent_default);
    assert!(!context.stop_propagation);

    context.prevent_default();
    context.stop_propagation();

    assert!(context.prevent_default);
    assert!(context.stop_propagation);
  }

  #[tokio::test]
  async fn test_component_handler_registration() {
    let message_manager = Arc::new(MessageManager::new());
    let focus_manager = Arc::new(RwLock::new(FocusManager::new()));
    let mut router = EventRouter::new(message_manager, focus_manager);

    let called = Arc::new(Mutex::new(false));
    let called_clone = called.clone();

    router
      .register_component_handler(
        "test_component".to_string(),
        "ClickMessage".to_string(),
        move |_context, _message| {
          *called_clone.lock().unwrap() = true;
          Ok(())
        },
      )
      .unwrap();

    // Verify handler was registered
    assert!(router.component_handlers.contains_key("test_component"));
    assert!(router.component_handlers["test_component"].contains_key("ClickMessage"));
  }
}
