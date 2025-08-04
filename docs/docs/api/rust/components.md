# Components Module

Core component system with DOM-like element structure and event handling for building reactive terminal interfaces.

## Element

Central component representing DOM-like elements with styling, event handling, and hierarchical structure.

```rust
use reactive_tui::components::Element;

let element = Element::with_tag("div")
    .id("main-container")
    .class("container")
    .style("display", "flex")
    .style("flex-direction", "column")
    .child(Element::with_tag("p").text("Hello World").build())
    .build();
```

### ElementBuilder

Builder pattern for creating elements with fluent API:

```rust
pub struct ElementBuilder {
    // builder fields
}

impl ElementBuilder {
    pub fn new(tag: &str) -> Self
    pub fn id(mut self, id: &str) -> Self
    pub fn class(mut self, class: &str) -> Self
    pub fn style(mut self, property: &str, value: &str) -> Self
    pub fn text(mut self, text: &str) -> Self
    pub fn child(mut self, child: Element) -> Self
    pub fn build(self) -> Element
}
```

## EventHandler

Event handling system for interactive components with support for bubbling and capturing phases.

```rust
use reactive_tui::components::{EventHandler, EventType};

let handler = EventHandler::new()
    .on(EventType::Click, |event| {
        println!("Element clicked: {:?}", event.target);
    })
    .on(EventType::KeyPress, |event| {
        if let Some(key) = event.key {
            println!("Key pressed: {:?}", key);
        }
    });
```

### EventType

```rust
pub enum EventType {
    Click,
    DoubleClick,
    MouseMove,
    MouseEnter,
    MouseLeave,
    KeyPress,
    KeyDown,
    KeyUp,
    Focus,
    Blur,
    Change,
    Custom(String),
}
```

## Component Hierarchy

### Parent-Child Relationships

Elements maintain hierarchical relationships similar to HTML DOM:

```rust
let parent = Element::with_tag("div")
    .child(
        Element::with_tag("header")
            .child(Element::with_tag("h1").text("Title").build())
            .build()
    )
    .child(
        Element::with_tag("main")
            .child(Element::with_tag("p").text("Content").build())
            .build()
    )
    .build();
```

### Tree Traversal

```rust
// Find elements by selector
let elements = parent.query_selector(".my-class");
let element = parent.query_selector_one("#my-id");

// Traverse hierarchy
for child in parent.children() {
    println!("Child tag: {}", child.tag());
}
```

## Style Management

### Inline Styles

```rust
let styled_element = Element::with_tag("div")
    .style("width", "100%")
    .style("height", "50px")
    .style("background-color", "blue")
    .style("color", "white")
    .build();
```

### CSS Classes

```rust
let element = Element::with_tag("button")
    .class("btn")
    .class("btn-primary")
    .class("btn-large")
    .build();
```

## State Integration

Components integrate with the reactive state system:

```rust
use reactive_tui::{components::Element, reactive::Reactive};

let state = Reactive::new("initial value");
let element = Element::with_tag("input")
    .bind_value(state.clone())
    .on_change(move |new_value| {
        state.set(new_value);
    })
    .build();
```

## Event Propagation

### Event Bubbling

Events bubble up from child to parent elements:

```rust
let parent = Element::with_tag("div")
    .on_click(|event| {
        println!("Parent clicked");
    })
    .child(
        Element::with_tag("button")
            .text("Click me")
            .on_click(|event| {
                println!("Button clicked");
                // event.stop_propagation(); // Prevents bubbling
            })
            .build()
    )
    .build();
```

### Event Capturing

Events can be captured during the capturing phase:

```rust
let element = Element::with_tag("div")
    .on_click_capture(|event| {
        println!("Captured during capturing phase");
    })
    .build();
```

## Component Lifecycle

### Mounting and Unmounting

```rust
impl Element {
    pub fn mount(&mut self) -> Result<()> {
        // Component mounting logic
    }
    
    pub fn unmount(&mut self) -> Result<()> {
        // Cleanup logic
    }
    
    pub fn is_mounted(&self) -> bool {
        // Check if component is mounted
    }
}
```

### Update Cycle

```rust
impl Element {
    pub fn update(&mut self) -> Result<()> {
        // Update component state and trigger re-render
    }
    
    pub fn force_update(&mut self) -> Result<()> {
        // Force immediate re-render
    }
}
```

## Performance Optimizations

### Virtual DOM

The component system implements virtual DOM for efficient updates:

- Minimal DOM operations
- Diff-based updates
- Batch rendering
- Memory-efficient tree structure

### Component Caching

```rust
// Components can be cached for performance
let cached_element = Element::with_tag("expensive-component")
    .cache_key("user-123")
    .build();
```

## Integration with Widgets

Components serve as the foundation for all widgets:

```rust
use reactive_tui::{components::Element, widgets::Button};

// Widget creates element internally
let button = Button::new("my-button", "Click Me");
let element = button.to_element();

// Manual element creation
let manual_button = Element::with_tag("button")
    .class("btn")
    .text("Click Me")
    .build();
```

## Example Usage

```rust
use reactive_tui::{
    components::{Element, EventType},
    reactive::Reactive,
};

// Create a reactive counter component
let counter = Reactive::new(0i32);
let counter_clone = counter.clone();

let app = Element::with_tag("div")
    .class("counter-app")
    .child(
        Element::with_tag("h1")
            .text("Counter Example")
            .build()
    )
    .child(
        Element::with_tag("div")
            .class("counter-display")
            .text(&format!("Count: {}", counter.get()))
            .build()
    )
    .child(
        Element::with_tag("button")
            .class("increment-btn")
            .text("Increment")
            .on_click(move |_| {
                let new_value = counter_clone.get() + 1;
                counter_clone.set(new_value);
            })
            .build()
    )
    .build();
```