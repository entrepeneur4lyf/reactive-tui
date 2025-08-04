---
sidebar_position: 6
---

# Events API

Comprehensive event handling system with actions, messages, and keyboard navigation.

## Event System Overview

The event system provides multiple layers of interaction:
- **Actions**: High-level semantic actions (save, quit, navigate)
- **Messages**: Component-specific messages (focus, blur, input change)
- **Key Bindings**: Keyboard shortcuts and navigation
- **Event Handlers**: Low-level event processing

## Actions

Actions represent semantic operations that can be triggered by various inputs.

### Action Structure

```rust
use reactive_tui::events::*;

// Create actions
let save_action = Action::new("save")
    .with_data("filename", "document.txt")
    .with_context("editor");

let navigate_action = Action::new("navigate")
    .with_data("direction", "next")
    .with_data("target", "tab");

// Action builder pattern
let complex_action = ActionBuilder::new("complex_operation")
    .data("param1", "value1")
    .data("param2", 42)
    .context("component_id")
    .priority(ActionPriority::High)
    .build();
```

### Action Dispatcher

```rust
use reactive_tui::events::*;

struct MyComponent {
    dispatcher: ActionDispatcher,
}

impl MyComponent {
    fn new() -> Self {
        let mut dispatcher = ActionDispatcher::new();
        
        // Register action handlers
        dispatcher.register("save", Box::new(|action| {
            println!("Saving file: {:?}", action.get_data("filename"));
            ActionResult::Handled
        }));
        
        dispatcher.register("quit", Box::new(|action| {
            std::process::exit(0);
        }));
        
        Self { dispatcher }
    }
    
    fn handle_user_input(&mut self, input: &str) {
        match input {
            "s" => {
                let action = Action::new("save")
                    .with_data("filename", "current.txt");
                self.dispatcher.dispatch(action);
            }
            "q" => {
                let action = Action::new("quit");
                self.dispatcher.dispatch(action);
            }
            _ => {}
        }
    }
}
```

### Built-in Actions

```rust
use reactive_tui::events::actions::common::*;

// Navigation actions
let focus_next = actions::focus_next();
let focus_previous = actions::focus_previous();
let scroll_up = actions::scroll_up();
let scroll_down = actions::scroll_down();

// Application actions
let quit = actions::quit();
let refresh = actions::refresh();
let save = actions::save();

// Editing actions
let copy = actions::copy();
let paste = actions::paste();
let undo = actions::undo();
let redo = actions::redo();

// Custom actions with common patterns
let navigate_tab = actions::navigate("tab", "next");
let toggle_sidebar = actions::toggle("sidebar");
let select_item = actions::select("item", 42);
```

### Action Results

```rust
#[derive(Debug, Clone)]
pub enum ActionResult {
    Handled,                    // Action was processed
    Ignored,                    // Action was not applicable
    Propagate,                  // Continue to parent handlers
    Error(String),              // Error occurred
    Async(AsyncActionHandle),   // Async operation started
}

// Handling action results
impl Component for MyComponent {
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        match action.name() {
            "save" => {
                if let Some(filename) = action.get_data("filename") {
                    self.save_file(filename)?;
                    Ok(ActionResult::Handled)
                } else {
                    Ok(ActionResult::Error("Missing filename".to_string()))
                }
            }
            "async_operation" => {
                let handle = self.start_async_operation();
                Ok(ActionResult::Async(handle))
            }
            _ => Ok(ActionResult::Ignored)
        }
    }
}
```

## Messages

Messages provide component-specific communication and lifecycle events.

### Message Types

```rust
use reactive_tui::events::messages::*;

// Lifecycle messages
let mount_msg = MountMessage::new("component_id");
let unmount_msg = UnmountMessage::new("component_id");

// Focus messages
let focus_msg = FocusMessage::new("button_id", FocusInfo {
    previous_focus: Some("input_id".to_string()),
    focus_reason: FocusReason::UserNavigation,
});

let blur_msg = BlurMessage::new("button_id", BlurInfo {
    next_focus: Some("next_input".to_string()),
    blur_reason: BlurReason::UserNavigation,
});

// Input messages
let input_msg = InputMessage::new("text_input", "Hello, World!");
let key_press_msg = KeyPressMessage::new("component", KeyEvent {
    code: KeyCode::Enter,
    modifiers: KeyModifiers::CONTROL,
});

// Custom messages
let custom_msg = CustomMessage::new("my_message")
    .with_data("key", "value")
    .with_data("number", 42);
```

### Message Handling

```rust
use reactive_tui::events::*;

impl Component for InteractiveComponent {
    fn handle_message(&mut self, message: &dyn Message) -> Result<()> {
        // Handle different message types
        if let Some(focus_msg) = message.downcast_ref::<FocusMessage>() {
            self.on_focus(focus_msg)?;
        } else if let Some(blur_msg) = message.downcast_ref::<BlurMessage>() {
            self.on_blur(blur_msg)?;
        } else if let Some(input_msg) = message.downcast_ref::<InputMessage>() {
            self.on_input_change(input_msg)?;
        } else if let Some(key_msg) = message.downcast_ref::<KeyPressMessage>() {
            self.on_key_press(key_msg)?;
        }
        
        Ok(())
    }
    
    fn on_focus(&mut self, message: &FocusMessage) -> Result<()> {
        println!("Component {} received focus", message.target_id());
        self.is_focused = true;
        self.request_render();
        Ok(())
    }
    
    fn on_input_change(&mut self, message: &InputMessage) -> Result<()> {
        self.value = message.value().to_string();
        self.validate_input();
        self.request_render();
        Ok(())
    }
}
```

### Message Manager

```rust
use reactive_tui::events::messages::*;

struct AppMessageManager {
    manager: MessageManager,
    handlers: HashMap<String, Box<dyn MessageHandler>>,
}

impl AppMessageManager {
    fn new() -> Self {
        let mut manager = MessageManager::new();
        
        // Register global message handlers
        manager.register_global_handler(Box::new(GlobalFocusHandler));
        manager.register_global_handler(Box::new(KeyboardNavigationHandler));
        
        Self {
            manager,
            handlers: HashMap::new(),
        }
    }
    
    fn register_component_handler(&mut self, component_id: String, handler: Box<dyn MessageHandler>) {
        self.handlers.insert(component_id, handler);
        self.manager.register_handler(&component_id, handler);
    }
    
    fn send_message(&mut self, message: Box<dyn Message>) -> Result<()> {
        self.manager.send_message(message)
    }
    
    fn broadcast_message(&mut self, message: Box<dyn Message>) -> Result<()> {
        self.manager.broadcast_message(message)
    }
}
```

## Keyboard Navigation

Comprehensive keyboard handling with customizable bindings.

### Key Bindings

```rust
use reactive_tui::events::keybinding::*;

let mut key_manager = KeyBindingManager::new();

// Basic key bindings
key_manager.bind(
    KeyCombination::new(KeyCode::Char('s')).with_ctrl(),
    ElementAction::Action("save".to_string())
)?;

key_manager.bind(
    KeyCombination::new(KeyCode::Char('q')).with_ctrl(),
    ElementAction::Action("quit".to_string())
)?;

// Navigation bindings
key_manager.bind(
    KeyCombination::new(KeyCode::Tab),
    ElementAction::Navigation(NavigationDirection::Next)
)?;

key_manager.bind(
    KeyCombination::new(KeyCode::Tab).with_shift(),
    ElementAction::Navigation(NavigationDirection::Previous)
)?;

// Arrow key navigation
key_manager.bind(
    KeyCombination::new(KeyCode::Up),
    ElementAction::Navigation(NavigationDirection::Up)
)?;

key_manager.bind(
    KeyCombination::new(KeyCode::Down),
    ElementAction::Navigation(NavigationDirection::Down)
)?;
```

### Key Combination Patterns

```rust
// Simple key combinations
let ctrl_s = KeyCombination::new(KeyCode::Char('s')).with_ctrl();
let alt_enter = KeyCombination::new(KeyCode::Enter).with_alt();
let shift_tab = KeyCombination::new(KeyCode::Tab).with_shift();

// Complex combinations
let ctrl_shift_o = KeyCombination::new(KeyCode::Char('o'))
    .with_ctrl()
    .with_shift();

// Function keys
let f1 = KeyCombination::new(KeyCode::F(1));
let ctrl_f12 = KeyCombination::new(KeyCode::F(12)).with_ctrl();

// Special keys
let escape = KeyCombination::new(KeyCode::Esc);
let page_up = KeyCombination::new(KeyCode::PageUp);
let home = KeyCombination::new(KeyCode::Home);

// Key sequences
let vim_save = KeySequence::new()
    .key(KeyCode::Esc)
    .key(KeyCode::Char(':'))
    .key(KeyCode::Char('w'))
    .key(KeyCode::Enter);

key_manager.bind_sequence(vim_save, ElementAction::Action("save".to_string()))?;
```

### Key Binding Presets

```rust
use reactive_tui::events::keybinding::*;

// Load predefined key binding sets
let mut key_manager = KeyBindingManager::new();

// Standard application bindings
key_manager.load_preset(KeyBindingPreset::StandardApp)?;

// Vim-style bindings
key_manager.load_preset(KeyBindingPreset::Vim)?;

// Emacs-style bindings
key_manager.load_preset(KeyBindingPreset::Emacs)?;

// Custom preset
let custom_preset = KeyBindingPreset::Custom(vec![
    (KeyCombination::new(KeyCode::Char('h')).with_ctrl(), ElementAction::Action("help".to_string())),
    (KeyCombination::new(KeyCode::Char('n')).with_ctrl(), ElementAction::Action("new".to_string())),
    (KeyCombination::new(KeyCode::Char('o')).with_ctrl(), ElementAction::Action("open".to_string())),
]);

key_manager.load_preset(custom_preset)?;
```

### Focus Management

```rust
use reactive_tui::events::focus::*;

struct AppFocusManager {
    focus_manager: FocusManager,
    focusable_elements: Vec<FocusableElement>,
}

impl AppFocusManager {
    fn new() -> Self {
        Self {
            focus_manager: FocusManager::new(),
            focusable_elements: Vec::new(),
        }
    }
    
    fn register_focusable(&mut self, element: FocusableElement) {
        self.focusable_elements.push(element.clone());
        self.focus_manager.register_element(element);
    }
    
    fn focus_next(&mut self) -> Result<Option<String>> {
        self.focus_manager.focus_next()
    }
    
    fn focus_previous(&mut self) -> Result<Option<String>> {
        self.focus_manager.focus_previous()
    }
    
    fn focus_element(&mut self, element_id: &str) -> Result<bool> {
        self.focus_manager.focus_element(element_id)
    }
    
    fn get_focused_element(&self) -> Option<&FocusableElement> {
        self.focus_manager.get_focused_element()
    }
}

// Create focusable elements
let button = FocusableElement::new("save_button")
    .with_tab_index(1)
    .with_focus_on_click(true)
    .with_focus_hint("Save the current document");

let input = FocusableElement::new("username_input")
    .with_tab_index(2)
    .with_auto_focus(true)
    .with_focus_hint("Enter your username");

focus_manager.register_focusable(button);
focus_manager.register_focusable(input);
```

## Event Handlers

Low-level event processing and delegation.

### Event Handler Implementation

```rust
use reactive_tui::events::*;

struct CustomEventHandler {
    component_id: String,
    action_dispatcher: ActionDispatcher,
}

impl EventHandler for CustomEventHandler {
    fn handle_event(&mut self, event: &Event) -> Result<bool> {
        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            Event::Resize(width, height) => self.handle_resize(*width, *height),
            Event::Custom(custom_event) => self.handle_custom_event(custom_event),
        }
    }
    
    fn can_handle(&self, event: &Event) -> bool {
        // Determine if this handler should process the event
        match event {
            Event::Key(_) => true,
            Event::Mouse(mouse_event) => {
                // Only handle mouse events within our component bounds
                self.is_within_bounds(mouse_event.position)
            }
            _ => false,
        }
    }
}

impl CustomEventHandler {
    fn handle_key_event(&mut self, key_event: &KeyEvent) -> Result<bool> {
        match (key_event.code, key_event.modifiers) {
            (KeyCode::Enter, KeyModifiers::NONE) => {
                let action = Action::new("activate")
                    .with_context(&self.component_id);
                self.action_dispatcher.dispatch(action);
                Ok(true) // Event handled
            }
            (KeyCode::Esc, KeyModifiers::NONE) => {
                let action = Action::new("cancel")
                    .with_context(&self.component_id);
                self.action_dispatcher.dispatch(action);
                Ok(true)
            }
            _ => Ok(false) // Event not handled
        }
    }
    
    fn handle_mouse_event(&mut self, mouse_event: &MouseEvent) -> Result<bool> {
        match mouse_event.kind {
            MouseEventKind::Down(MouseButton::Left) => {
                let action = Action::new("click")
                    .with_data("x", mouse_event.column)
                    .with_data("y", mouse_event.row)
                    .with_context(&self.component_id);
                self.action_dispatcher.dispatch(action);
                Ok(true)
            }
            MouseEventKind::Scroll(direction) => {
                let action_name = match direction {
                    ScrollDirection::Up => "scroll_up",
                    ScrollDirection::Down => "scroll_down",
                };
                let action = Action::new(action_name)
                    .with_context(&self.component_id);
                self.action_dispatcher.dispatch(action);
                Ok(true)
            }
            _ => Ok(false)
        }
    }
}
```

### Event Flow

```rust
use reactive_tui::events::*;

struct EventFlow {
    handlers: Vec<Box<dyn EventHandler>>,
    global_handlers: Vec<Box<dyn EventHandler>>,
    key_bindings: KeyBindingManager,
    focus_manager: FocusManager,
}

impl EventFlow {
    fn process_event(&mut self, event: Event) -> Result<()> {
        // 1. Check global handlers first
        for handler in &mut self.global_handlers {
            if handler.can_handle(&event) && handler.handle_event(&event)? {
                return Ok(()); // Event consumed by global handler
            }
        }
        
        // 2. Check key bindings
        if let Event::Key(key_event) = &event {
            if let Some(binding_result) = self.key_bindings.process_key(key_event)? {
                match binding_result {
                    KeyBindingResult::Action(action) => {
                        self.dispatch_action(action)?;
                        return Ok(());
                    }
                    KeyBindingResult::Navigation(direction) => {
                        self.handle_navigation(direction)?;
                        return Ok(());
                    }
                    KeyBindingResult::Custom(custom) => {
                        self.handle_custom_binding(custom)?;
                        return Ok(());
                    }
                }
            }
        }
        
        // 3. Send to focused element first
        if let Some(focused_element) = self.focus_manager.get_focused_element() {
            if let Some(handler) = self.get_handler_for_element(&focused_element.id) {
                if handler.can_handle(&event) && handler.handle_event(&event)? {
                    return Ok(());
                }
            }
        }
        
        // 4. Bubble through other handlers
        for handler in &mut self.handlers {
            if handler.can_handle(&event) && handler.handle_event(&event)? {
                return Ok(()); // Event consumed
            }
        }
        
        // Event not handled
        Ok(())
    }
    
    fn handle_navigation(&mut self, direction: NavigationDirection) -> Result<()> {
        match direction {
            NavigationDirection::Next => {
                self.focus_manager.focus_next()?;
            }
            NavigationDirection::Previous => {
                self.focus_manager.focus_previous()?;
            }
            NavigationDirection::Up | NavigationDirection::Down |
            NavigationDirection::Left | NavigationDirection::Right => {
                self.focus_manager.focus_directional(direction)?;
            }
        }
        Ok(())
    }
}
```

## Custom Events

Create application-specific events and handlers.

### Custom Event Types

```rust
use reactive_tui::events::*;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DataLoadedEvent {
    source: String,
    record_count: usize,
    load_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct UserActionEvent {
    user_id: String,
    action_type: String,
    timestamp: u64,
    metadata: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ValidationErrorEvent {
    field_id: String,
    error_message: String,
    error_code: String,
}

// Custom event wrapper
#[derive(Debug, Clone)]
enum CustomEvent {
    DataLoaded(DataLoadedEvent),
    UserAction(UserActionEvent),
    ValidationError(ValidationErrorEvent),
}

impl Event for CustomEvent {
    fn event_type(&self) -> &'static str {
        match self {
            CustomEvent::DataLoaded(_) => "data_loaded",
            CustomEvent::UserAction(_) => "user_action",
            CustomEvent::ValidationError(_) => "validation_error",
        }
    }
    
    fn timestamp(&self) -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64
    }
}
```

### Event Broadcasting

```rust
use reactive_tui::events::*;

struct EventBroadcaster {
    listeners: HashMap<String, Vec<Box<dyn EventListener>>>,
}

impl EventBroadcaster {
    fn new() -> Self {
        Self {
            listeners: HashMap::new(),
        }
    }
    
    fn subscribe<E: Event + 'static>(&mut self, listener: Box<dyn EventListener<E>>) {
        let event_type = std::any::type_name::<E>();
        self.listeners
            .entry(event_type.to_string())
            .or_insert_with(Vec::new)
            .push(Box::new(listener));
    }
    
    fn emit<E: Event>(&mut self, event: E) -> Result<()> {
        let event_type = event.event_type();
        
        if let Some(listeners) = self.listeners.get_mut(event_type) {
            for listener in listeners {
                listener.handle_event(&event)?;
            }
        }
        
        Ok(())
    }
}

// Event listener trait
trait EventListener<E: Event> {
    fn handle_event(&mut self, event: &E) -> Result<()>;
}

// Example listener implementation
struct DataLoadedListener {
    component_id: String,
}

impl EventListener<DataLoadedEvent> for DataLoadedListener {
    fn handle_event(&mut self, event: &DataLoadedEvent) -> Result<()> {
        println!("Component {} received data loaded event: {} records from {}",
                 self.component_id, event.record_count, event.source);
        
        // Update component state, trigger re-render, etc.
        Ok(())
    }
}
```

### Async Event Handling

```rust
use reactive_tui::events::*;
use tokio::sync::mpsc;

struct AsyncEventHandler {
    event_sender: mpsc::UnboundedSender<Event>,
}

impl AsyncEventHandler {
    fn new() -> (Self, mpsc::UnboundedReceiver<Event>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        (Self {
            event_sender: sender,
        }, receiver)
    }
    
    fn send_event(&self, event: Event) -> Result<()> {
        self.event_sender.send(event)
            .map_err(|e| TuiError::Component(format!("Failed to send event: {}", e)))?;
        Ok(())
    }
    
    async fn process_events(mut receiver: mpsc::UnboundedReceiver<Event>) -> Result<()> {
        while let Some(event) = receiver.recv().await {
            // Process events asynchronously
            match event {
                Event::Custom(custom_event) => {
                    // Handle custom events that might require async operations
                    Self::handle_async_event(custom_event).await?;
                }
                _ => {
                    // Handle synchronous events
                    Self::handle_sync_event(event)?;
                }
            }
        }
        Ok(())
    }
    
    async fn handle_async_event(event: CustomEvent) -> Result<()> {
        match event {
            CustomEvent::DataLoaded(data_event) => {
                // Async database update
                update_database(&data_event).await?;
            }
            CustomEvent::UserAction(user_event) => {
                // Async logging
                log_user_action(&user_event).await?;
            }
            _ => {}
        }
        Ok(())
    }
}
```