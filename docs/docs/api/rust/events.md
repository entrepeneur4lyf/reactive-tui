# Events Module

The `events` module provides a comprehensive event handling system for user interactions, keyboard input, and application messages.

## Event

Core event enum that represents all possible events in the application.

```rust
use reactive_tui::events::Event;

match event {
    Event::Key(key_event) => {
        // Handle keyboard input
    },
    Event::Mouse(mouse_event) => {
        // Handle mouse input
    },
    Event::Resize(width, height) => {
        // Handle terminal resize
    },
    Event::Focus(element_id) => {
        // Handle focus change
    },
    Event::Custom(data) => {
        // Handle custom events
    },
}
```

### Variants

#### `Key(KeyEvent)`
Keyboard input events.

#### `Mouse(MouseEvent)`  
Mouse input events.

#### `Resize(u16, u16)`
Terminal resize events with new width and height.

#### `Focus(String)`
Focus change events with element ID.

#### `Blur(String)`
Focus lost events with element ID.

#### `Custom(Box<dyn Any + Send + Sync>)`
Custom application events.

## EventHandler

Central event handling system that manages event registration and dispatch.

```rust
use reactive_tui::events::{EventHandler, KeyCode, KeyModifiers};

let mut handler = EventHandler::new();

// Register keyboard shortcut
handler.on_key(KeyCode::Char('s'), KeyModifiers::CONTROL, |_| {
    println!("Save shortcut pressed!");
});

// Register mouse click handler
handler.on_mouse_click(|x, y, button| {
    println!("Mouse clicked at ({}, {}) with {:?}", x, y, button);
});
```

### Methods

#### `new() -> EventHandler`
Creates a new event handler.

#### `on_key<F>(&mut self, key: KeyCode, modifiers: KeyModifiers, callback: F)` where `F: Fn(&KeyEvent) + Send + Sync + 'static`
Registers a keyboard event handler.

**Parameters:**
- `key` - The key code to listen for
- `modifiers` - Required modifier keys (Ctrl, Alt, Shift)
- `callback` - Function to call when key combination is pressed

#### `on_mouse_click<F>(&mut self, callback: F)` where `F: Fn(u16, u16, MouseButton) + Send + Sync + 'static`
Registers a mouse click handler.

**Parameters:**
- `callback` - Function called with (x, y, button) on click

#### `on_resize<F>(&mut self, callback: F)` where `F: Fn(u16, u16) + Send + Sync + 'static`
Registers a terminal resize handler.

**Parameters:**
- `callback` - Function called with (width, height) on resize

#### `handle_event(&self, event: &Event) -> Result<bool>`
Processes an event and calls appropriate handlers.

**Parameters:**
- `event` - The event to handle

**Returns:** `Ok(true)` if event was handled, `Ok(false)` if not, or `Err` on error

#### `clear(&mut self)`
Removes all registered event handlers.

## Key Bindings

### KeyBindingManager

Manages keyboard shortcuts and key combinations.

```rust
use reactive_tui::events::keybinding::{KeyBindingManager, KeyCombination, KeyAction};

let mut manager = KeyBindingManager::new();

// Single key binding
manager.bind(
    KeyCombination::new(KeyCode::Char('q')),
    KeyAction::Quit
);

// Key combination with modifiers
manager.bind(
    KeyCombination::new(KeyCode::Char('s')).with_ctrl(),
    KeyAction::Save
);

// Sequence of keys (like Vim)
manager.bind_sequence(
    vec![KeyCode::Char('g'), KeyCode::Char('g')],
    KeyAction::GoToTop
);
```

### KeyCombination

Represents a combination of keys and modifiers.

```rust
use reactive_tui::events::keybinding::KeyCombination;

// Different ways to create key combinations
let ctrl_s = KeyCombination::new(KeyCode::Char('s')).with_ctrl();
let alt_enter = KeyCombination::new(KeyCode::Enter).with_alt();
let shift_tab = KeyCombination::new(KeyCode::Tab).with_shift();
let ctrl_alt_d = KeyCombination::new(KeyCode::Char('d')).with_ctrl().with_alt();
```

### KeyAction

Enum representing different actions that can be triggered by key combinations.

```rust
use reactive_tui::events::keybinding::KeyAction;

pub enum KeyAction {
    // Navigation
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    PageUp,
    PageDown,
    Home,
    End,
    
    // Editing
    Insert,
    Delete,
    Backspace,
    Cut,
    Copy,
    Paste,
    Undo,
    Redo,
    
    // Application
    Save,
    Open,
    Quit,
    Help,
    Search,
    
    // Focus management
    NextTab,
    PreviousTab,
    FocusNext,
    FocusPrevious,
    
    // Custom actions
    Custom(String),
}
```

## Focus Management

### FocusManager

Manages focus state and navigation between focusable elements.

```rust
use reactive_tui::events::focus::FocusManager;

let mut focus_manager = FocusManager::new();

// Register focusable elements
focus_manager.register("input1", true);  // focusable
focus_manager.register("button1", true);
focus_manager.register("label1", false); // not focusable

// Navigate focus
focus_manager.focus_next();
focus_manager.focus_previous();
focus_manager.focus_element("button1");

// Get current focus
if let Some(focused_id) = focus_manager.get_focused() {
    println!("Currently focused: {}", focused_id);
}
```

### Methods

#### `new() -> FocusManager`
Creates a new focus manager.

#### `register(&mut self, element_id: &str, focusable: bool)`
Registers an element as focusable or not.

#### `focus_next(&mut self) -> Option<String>`
Moves focus to the next focusable element.

#### `focus_previous(&mut self) -> Option<String>`
Moves focus to the previous focusable element.

#### `focus_element(&mut self, element_id: &str) -> bool`
Sets focus to a specific element.

#### `get_focused(&self) -> Option<&str>`
Gets the currently focused element ID.

#### `clear_focus(&mut self)`
Removes focus from all elements.

## Messages

### MessageManager

Handles inter-component communication through messages.

```rust
use reactive_tui::events::messages::{MessageManager, MessageEvent};

let mut manager = MessageManager::new();

// Subscribe to messages
manager.subscribe("user_login", |message| {
    if let Some(username) = message.get_data::<String>() {
        println!("User {} logged in", username);
    }
});

// Send messages
manager.send(MessageEvent::new(
    "user_login",
    "john_doe".to_string()
));
```

### MessageEvent

Represents a message with type and optional data payload.

```rust
use reactive_tui::events::messages::MessageEvent;

// Create different types of messages
let login_msg = MessageEvent::new("user_login", "username".to_string());
let click_msg = MessageEvent::new("button_click", 42u32);
let custom_msg = MessageEvent::custom("app_event", CustomData { id: 1, name: "test" });
```

### Built-in Message Types

```rust
use reactive_tui::events::messages::*;

// Input events
let input_msg = InputMessage::new("username_field", "john");
let submit_msg = SubmitMessage::new("login_form");

// Focus events  
let focus_msg = FocusMessage::new("submit_button");
let blur_msg = BlurMessage::new("username_field");

// Mouse events
let click_msg = ClickMessage::new("save_button", 10, 5);

// Lifecycle events
let mount_msg = MountMessage::new("modal_dialog");
let unmount_msg = UnmountMessage::new("modal_dialog");
```

## Action System

### ActionDispatcher

Handles application-wide actions and commands.

```rust
use reactive_tui::events::actions::{ActionDispatcher, Action};

let mut dispatcher = ActionDispatcher::new();

// Register action handlers
dispatcher.register("save_file", |context| {
    // Save the current file
    context.file_manager.save_current()?;
    Ok(ActionResult::Success)
});

dispatcher.register("open_modal", |context| {
    // Show a modal dialog
    context.ui.show_modal("settings")?;
    Ok(ActionResult::Handled)
});

// Dispatch actions
dispatcher.dispatch(Action::new("save_file"))?;
dispatcher.dispatch(Action::with_data("open_modal", "confirmation"))?;
```

### Action

Represents an action that can be dispatched.

```rust
use reactive_tui::events::actions::Action;

// Create actions
let save_action = Action::new("save_file");
let open_action = Action::with_data("open_file", "/path/to/file.txt");
let quit_action = Action::new("quit_app").with_priority(10);
```

## Example: Complete Event Handling

```rust
use reactive_tui::{
    TuiAppBuilder,
    events::{EventHandler, KeyCode, KeyModifiers, Event},
    widgets::Input,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TuiAppBuilder::new()
        .title("Event Handling Demo")
        .build()?;

    // Create event handler
    let mut event_handler = EventHandler::new();
    
    // Global keyboard shortcuts
    event_handler.on_key(KeyCode::Char('q'), KeyModifiers::CONTROL, |_| {
        std::process::exit(0);
    });
    
    event_handler.on_key(KeyCode::F1, KeyModifiers::empty(), |_| {
        println!("Help: Ctrl+Q to quit, Ctrl+S to save");
    });
    
    event_handler.on_key(KeyCode::Char('s'), KeyModifiers::CONTROL, |_| {
        println!("Save command triggered!");
    });
    
    // Handle window resize
    event_handler.on_resize(|width, height| {
        println!("Terminal resized to {}x{}", width, height);
    });
    
    // Create input with event handling
    let mut input = Input::new("main_input");
    input.on_change(|value| {
        println!("Input changed: {}", value);
    });
    
    input.on_submit(|value| {
        println!("Input submitted: {}", value);
    });
    
    app.set_event_handler(event_handler);
    app.add_widget(input)?;
    
    app.run().await?;
    Ok(())
}
```