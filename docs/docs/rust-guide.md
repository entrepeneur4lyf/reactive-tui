# Rust-Only TUI Development Guide

This guide covers building terminal user interfaces using only the Rust crate, without the TypeScript/Node.js layer.

## Quick Start

Add `reactive-tui` to your `Cargo.toml`:

```toml
[dependencies]
reactive-tui = "0.0.4"
tokio = { version = "1.0", features = ["full"] }
```

## Basic Application Structure

```rust
use reactive_tui::{TuiApp, TuiAppBuilder, widgets::*, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = TuiAppBuilder::new()
        .title("My Rust TUI")
        .build()?;
    
    // Create your UI
    let button = Button::new("main-btn", "Click me!");
    app.add_widget(button)?;
    
    // Start the event loop
    app.run().await
}
```

## Core Concepts

### Reactive State Management

```rust
use reactive_tui::reactive::Reactive;
use std::sync::Arc;

// Create reactive state
let counter = Arc::new(Reactive::new(0));
let counter_clone = counter.clone();

// Watch for changes
counter.watch(move |value| {
    println!("Counter changed to: {}", value);
});

// Update state
counter.update(|n| *n += 1);
```

### Widget Creation

```rust
use reactive_tui::widgets::{Button, Input, Modal, DataTable};

// Button with click handler
let mut button = Button::new("btn1", "Save");
button.on_click(|_| {
    println!("Button clicked!");
});

// Input field with validation
let mut input = Input::new("name-input");
input.set_placeholder("Enter your name");
input.on_change(|value| {
    println!("Input changed: {}", value);
});

// Modal dialog
let modal = Modal::new("confirm-modal")
    .title("Confirm Action")
    .content("Are you sure?")
    .width(40)
    .height(15);
```

### CSS Styling

```rust
use reactive_tui::css::{CssEngine, Stylesheet};

let css = r#"
    .button {
        background-color: #007acc;
        color: white;
        border: 1px solid #005a9e;
        padding: 1px 2px;
    }
    
    .button:hover {
        background-color: #005a9e;
    }
    
    .input {
        border: 1px solid #ccc;
        padding: 0px 1px;
    }
    
    .modal {
        background-color: #f0f0f0;
        border: 2px solid #333;
        box-shadow: 0 2px 4px rgba(0,0,0,0.3);
    }
"#;

let stylesheet = Stylesheet::parse(css)?;
app.add_stylesheet(stylesheet)?;
```

### Layout Systems

#### Flexbox Layout

```rust
use reactive_tui::layout::{FlexContainer, FlexDirection, JustifyContent};

let container = FlexContainer::new()
    .direction(FlexDirection::Row)
    .justify_content(JustifyContent::SpaceBetween)
    .add_child(button1)
    .add_child(button2)
    .add_child(button3);
```

#### Grid Layout

```rust
use reactive_tui::widgets::Grid;

let grid = Grid::new("main-grid")
    .columns("1fr 2fr 1fr")
    .rows("auto 1fr auto")
    .gap(1);

grid.place_widget(header, 0, 0, 3, 1)?; // span 3 columns
grid.place_widget(sidebar, 0, 1, 1, 1)?;
grid.place_widget(content, 1, 1, 1, 1)?;
grid.place_widget(footer, 0, 2, 3, 1)?;
```

### Event Handling

```rust
use reactive_tui::events::{Event, EventHandler, KeyCode, KeyModifiers};

let mut event_handler = EventHandler::new();

// Handle keyboard events
event_handler.on_key(KeyCode::Char('q'), KeyModifiers::CONTROL, |_| {
    std::process::exit(0);
});

event_handler.on_key(KeyCode::Enter, KeyModifiers::empty(), |_| {
    println!("Enter pressed!");
});

// Handle mouse events
event_handler.on_mouse_click(|x, y, button| {
    println!("Mouse clicked at ({}, {}) with {:?}", x, y, button);
});

app.set_event_handler(event_handler);
```

## Complete Example

Here's a complete example showing a data entry form:

```rust
use reactive_tui::{
    TuiAppBuilder, 
    widgets::*,
    reactive::Reactive,
    css::Stylesheet,
    events::{Event, KeyCode, KeyModifiers},
    Result
};
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    // Create app
    let mut app = TuiAppBuilder::new()
        .title("Data Entry Form")
        .build()?;

    // Reactive state
    let form_data = Arc::new(Reactive::new(FormData::default()));
    
    // CSS styling
    let css = r#"
        .form-container {
            display: flex;
            flex-direction: column;
            padding: 2px;
            gap: 1px;
        }
        
        .input-group {
            display: flex;
            flex-direction: row;
            align-items: center;
            gap: 2px;
        }
        
        .label {
            min-width: 12px;
            text-align: right;
        }
        
        .input {
            flex: 1;
            border: 1px solid #ccc;
            padding: 0px 1px;
        }
        
        .input:focus {
            border-color: #007acc;
        }
        
        .button-group {
            display: flex;
            justify-content: flex-end;
            gap: 2px;
            margin-top: 2px;
        }
        
        .button {
            padding: 1px 3px;
            border: 1px solid #007acc;
            background-color: #007acc;
            color: white;
        }
        
        .button:hover {
            background-color: #005a9e;
        }
    "#;
    
    app.add_stylesheet(Stylesheet::parse(css)?)?;

    // Create form widgets
    let name_input = Input::new("name")
        .placeholder("Enter full name")
        .class("input");
    
    let email_input = Input::new("email")
        .placeholder("Enter email address")
        .class("input");
    
    let age_input = Input::new("age")
        .placeholder("Enter age")
        .input_type(InputType::Number)
        .class("input");

    // Form container
    let form = div()
        .class("form-container")
        .child(
            div().class("input-group")
                .child(span().text("Name:").class("label"))
                .child(name_input)
        )
        .child(
            div().class("input-group")
                .child(span().text("Email:").class("label"))
                .child(email_input)
        )
        .child(
            div().class("input-group")
                .child(span().text("Age:").class("label"))
                .child(age_input)
        )
        .child(
            div().class("button-group")
                .child(Button::new("save", "Save").class("button"))
                .child(Button::new("cancel", "Cancel").class("button"))
        );

    app.add_widget(form)?;

    // Event handlers
    app.on_key(KeyCode::Char('q'), KeyModifiers::CONTROL, |_| {
        std::process::exit(0);
    });

    app.on_widget_event("save", "click", |_| {
        println!("Form saved!");
    });

    app.on_widget_event("cancel", "click", |_| {
        println!("Form cancelled!");
    });

    // Start the application
    app.run().await
}

#[derive(Default)]
struct FormData {
    name: String,
    email: String, 
    age: u32,
}
```

## Advanced Features

### Custom Widgets

```rust
use reactive_tui::{Widget, RenderContext, Result};

pub struct CustomCounter {
    id: String,
    value: Arc&lt;Reactive&lt;i32&gt;&gt;,
}

impl CustomCounter {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            value: Arc::new(Reactive::new(0)),
        }
    }
    
    pub fn increment(&self) {
        self.value.update(|v| *v += 1);
    }
}

impl Widget for CustomCounter {
    fn id(&self) -> &str {
        &self.id
    }
    
    fn render(&self, ctx: &mut RenderContext) -> Result<()> {
        let value = self.value.get();
        ctx.write_text(&format!("Count: {}", value))?;
        Ok(())
    }
}
```

### Themes

```rust
use reactive_tui::themes::{Theme, ThemeManager};

let dark_theme = Theme::builder()
    .name("dark")
    .background_color("#1e1e1e")
    .text_color("#ffffff")
    .accent_color("#007acc")
    .build();

let theme_manager = ThemeManager::new();
theme_manager.add_theme(dark_theme);
theme_manager.set_active_theme("dark");

app.set_theme_manager(theme_manager);
```

### Animations

```rust
use reactive_tui::animation::{Animation, Easing, Duration};

let fade_in = Animation::new()
    .property("opacity")
    .from(0.0)
    .to(1.0)
    .duration(Duration::from_millis(500))
    .easing(Easing::EaseInOut);

widget.animate(fade_in);
```

## Performance Tips

1. **Use Arc&lt;Reactive&lt;T&gt;&gt;** for shared state to minimize clones
2. **Batch updates** using `update_batch()` for multiple state changes
3. **Use virtual rendering** for large lists with `ScrollableList::virtual(true)`
4. **Minimize re-renders** by using specific state watchers
5. **Use release builds** for production applications

## Best Practices

1. **Structure your app** with clear separation between UI and business logic
2. **Use CSS classes** instead of inline styles for maintainability  
3. **Handle errors gracefully** with proper error propagation
4. **Implement proper cleanup** for resources and event handlers
5. **Test your TUI** with automated integration tests

## Examples to Try

Check out the examples directory for more complete applications:

- `cargo run --example button_demo` - Interactive buttons
- `cargo run --example datatable_demo` - Sortable data tables
- `cargo run --example modal_demo` - Modal dialogs
- `cargo run --example layout_demo` - Advanced layouts
- `cargo run --example theme_system_demo` - Theme switching

For the complete Rust API reference, visit: [/rust-api/reactive_tui/](/rust-api/reactive_tui/)