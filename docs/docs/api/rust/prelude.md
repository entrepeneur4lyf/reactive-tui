# Prelude Module

Convenience module that re-exports commonly used types, traits, and functions for easy importing and quick development setup.

## What's Included

The prelude provides a curated selection of the most frequently used items from across the reactive-tui crate, allowing you to get started quickly with a single import.

```rust
use reactive_tui::prelude::*;

// Now you have access to all commonly used types and traits
let app = TuiApp::builder()
    .title("My App")
    .size(80, 24)
    .build();
```

## Core Types

### Application and State

```rust
// Re-exported from reactive_tui::app
pub use crate::app::{TuiApp, TuiAppBuilder, TuiAppConfig};

// Re-exported from reactive_tui::reactive
pub use crate::reactive::{Reactive, ReactiveState, Watcher, StateChange};
```

### Components and Elements

```rust
// Re-exported from reactive_tui::components
pub use crate::components::{Element, ElementBuilder, EventHandler, EventType};

// Re-exported from reactive_tui::widgets
pub use crate::widgets::{ResponsiveWidget, Widget, WidgetConfig};
```

### Layout and Styling

```rust
// Re-exported from reactive_tui::layout
pub use crate::layout::{
    LayoutEngine, LayoutRect, ComputedStyles, DisplayType,
    FlexDirection, JustifyContent, AlignItems, GridLayout, GridTrack
};

// Re-exported from reactive_tui::css
pub use crate::css::{CssEngine, Stylesheet, Selector, ComponentTree};
```

### Theming

```rust
// Re-exported from reactive_tui::themes
pub use crate::themes::{ColorTheme, ThemeManager, Color, Style};
```

### Events

```rust
// Re-exported from reactive_tui::events
pub use crate::events::{
    Event, KeyEvent, MouseEvent, KeyCode, KeyModifiers,
    MouseButton, MouseEventKind, EventManager
};
```

### Error Handling

```rust
// Re-exported from reactive_tui::error
pub use crate::error::{TuiError, Result};
```

## Common Traits

### Widget Development

```rust
// Essential traits for widget development
pub use crate::widgets::ResponsiveWidget;
pub use crate::components::ElementBuilder;
pub use crate::reactive::ReactiveValue;
```

### Event Handling

```rust
// Event handling traits
pub use crate::events::{EventHandler, EventListener};
```

### Rendering

```rust
// Rendering traits
pub use crate::rendering::{Renderable, RenderContext};
```

## Utility Functions

### Element Creation Helpers

```rust
// Convenience functions for creating elements
pub fn div() -> ElementBuilder {
    Element::with_tag("div")
}

pub fn span() -> ElementBuilder {
    Element::with_tag("span")
}

pub fn button(text: &str) -> ElementBuilder {
    Element::with_tag("button").text(text)
}
```

### Common Widget Builders

```rust
// Re-exported widget builders for convenience
pub use crate::widgets::{
    Button, ButtonBuilder,
    Input, InputBuilder,
    DataTable, DataTableBuilder,
};
```

## Type Aliases

### Common Result Types

```rust
// Convenient type aliases
pub type TuiResult<T> = Result<T, TuiError>;
pub type ElementResult = Result<Element, TuiError>;
pub type WidgetResult<T> = Result<T, TuiError>;
```

### Callback Types

```rust
// Common callback type aliases
pub type EventCallback = Box<dyn Fn(&Event) -> Result<()> + Send + Sync>;
pub type StateCallback<T> = Box<dyn Fn(&T) -> Result<()> + Send + Sync>;
pub type RenderCallback = Box<dyn Fn(&RenderContext) -> Result<()> + Send + Sync>;
```

## Constants

### Default Values

```rust
// Common default values
pub const DEFAULT_WINDOW_WIDTH: u16 = 80;
pub const DEFAULT_WINDOW_HEIGHT: u16 = 24;
pub const DEFAULT_FPS: u16 = 60;
pub const DEFAULT_BUFFER_SIZE: usize = 1024;
```

### Key Codes

```rust
// Re-exported common key codes for convenience
pub use crate::events::{
    KEY_ENTER, KEY_ESCAPE, KEY_TAB, KEY_BACKSPACE,
    KEY_ARROW_UP, KEY_ARROW_DOWN, KEY_ARROW_LEFT, KEY_ARROW_RIGHT,
    KEY_F1, KEY_F2, KEY_F3, KEY_F4, KEY_F5, KEY_F6,
    KEY_F7, KEY_F8, KEY_F9, KEY_F10, KEY_F11, KEY_F12,
};
```

## Quick Start Template

Using the prelude, you can quickly set up a basic TUI application:

```rust
use reactive_tui::prelude::*;

fn main() -> TuiResult<()> {
    // Create application
    let mut app = TuiApp::builder()
        .title("My TUI App")
        .size(DEFAULT_WINDOW_WIDTH, DEFAULT_WINDOW_HEIGHT)
        .fps_limit(DEFAULT_FPS)
        .build()?;
    
    // Create reactive state
    let counter = Reactive::new(0i32);
    let counter_clone = counter.clone();
    
    // Create UI elements
    let root = div()
        .class("app")
        .child(
            div()
                .class("header")
                .child(
                    Element::with_tag("h1")
                        .text("Counter App")
                        .build()
                )
                .build()
        )
        .child(
            div()
                .class("counter")
                .child(
                    span()
                        .text(&format!("Count: {}", counter.get()))
                        .build()
                )
                .build()
        )
        .child(
            button("Increment")
                .class("increment-btn")
                .on_click(move |_| {
                    let new_value = counter_clone.get() + 1;
                    counter_clone.set(new_value);
                    Ok(())
                })
                .build()
        )
        .build();
    
    // Set root element
    app.set_root(root);
    
    // Run application
    app.run()
}
```

## Feature-Specific Imports

If you need more specific functionality, you can import individual modules:

```rust
// For advanced layout features
use reactive_tui::layout::{GridLayout, FlexLayout, ConstraintLayout};

// For custom rendering
use reactive_tui::rendering::{RenderEngine, FrameBuffer, VirtualRenderer};

// For plugin development
use reactive_tui::plugin::{Plugin, PluginManager, WidgetPlugin};

// For advanced state management
use reactive_tui::reactive::{StateManager, ReactiveCollection, ComputedValue};
```

## Best Practices

### Import Strategy

```rust
// Recommended: Use prelude for common development
use reactive_tui::prelude::*;

// For specific advanced features, import explicitly
use reactive_tui::rendering::VirtualRenderer;
use reactive_tui::plugin::PluginManager;
```

### Avoiding Conflicts

```rust
// If you have naming conflicts, use selective imports
use reactive_tui::prelude::{TuiApp, TuiResult};
use reactive_tui::widgets::Button as TuiButton;

// Or use module prefixes
use reactive_tui::{prelude::*, widgets};
let button = widgets::Button::new("my-button", "Click me");
```

## What's Not Included

The prelude intentionally excludes:

- **Advanced/Specialized APIs**: Complex rendering internals, plugin development APIs
- **Platform-Specific Code**: Driver implementations, platform-specific utilities  
- **Experimental Features**: Unstable or experimental functionality
- **Internal Types**: Implementation details not needed for typical usage
- **Large Enums**: Comprehensive enumerations that would clutter the namespace

For these features, import them explicitly from their respective modules.

## Version Compatibility

The prelude maintains stability across minor versions:

- **Additions**: New commonly-used items may be added in minor versions
- **Deprecations**: Items are deprecated before removal with clear migration paths
- **Breaking Changes**: Only occur in major version updates

## Usage Examples

### Basic Widget Creation

```rust
use reactive_tui::prelude::*;

// Create a simple form
let form = div()
    .class("form")
    .child(
        Input::builder("username")
            .placeholder("Enter username")
            .required(true)
            .build()
            .to_element()
    )
    .child(
        Input::builder("password")
            .input_type(InputType::Password)
            .placeholder("Enter password")
            .required(true)
            .build()
            .to_element()
    )
    .child(
        Button::builder("submit", "Login")
            .button_type(ButtonType::Primary)
            .build()
            .to_element()
    )
    .build();
```

### Event Handling

```rust
use reactive_tui::prelude::*;

let button = button("Click me")
    .on_click(|event| {
        println!("Button clicked!");
        Ok(())
    })
    .on_keydown(|event| {
        if event.key_code == KeyCode::Enter {
            println!("Enter pressed on button");
        }
        Ok(())
    })
    .build();
```

### State Management

```rust
use reactive_tui::prelude::*;

let name = Reactive::new(String::new());
let greeting = Reactive::computed({
    let name = name.clone();
    move || {
        let name_value = name.get();
        if name_value.is_empty() {
            "Hello, stranger!".to_string()
        } else {
            format!("Hello, {}!", name_value)
        }
    }
});

// Use in UI
let display = span()
    .text(&greeting.get())
    .build();
```

The prelude makes reactive-tui development more ergonomic by providing easy access to the most commonly used functionality while keeping the import surface clean and manageable.