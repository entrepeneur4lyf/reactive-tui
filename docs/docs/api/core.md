---
sidebar_position: 2
---

# Core API

Core application and component APIs for building reactive TUI applications.

## TuiApp

Main application class for creating and managing TUI applications.

### Constructor

```rust
impl TuiApp {
    pub fn new(title: &str) -> Result<Self>
    pub fn builder() -> TuiAppBuilder
}
```

### Methods

```rust
impl TuiApp {
    // Application lifecycle
    pub async fn run(&self) -> Result<()>
    pub fn stop(&self) -> Result<()>
    pub fn is_running(&self) -> bool
    
    // Configuration
    pub fn set_title(&mut self, title: &str) -> Result<()>
    pub fn load_css(&mut self, css: &str) -> Result<()>
    pub fn load_stylesheet(&mut self, path: &str) -> Result<()>
    pub fn set_theme(&mut self, theme: ColorTheme) -> Result<()>
    
    // Component management
    pub fn set_component<C: Component>(&mut self, component: C) -> Result<()>
    pub fn add_screen(&mut self, screen: Box<dyn Screen>) -> Result<()>
    
    // Event handling
    pub fn send_message(&self, message: impl Message) -> Result<()>
    pub fn dispatch_action(&self, action: Action) -> Result<ActionResult>
}
```

**Example:**
```rust
use reactive_tui::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = TuiApp::new("My Application")?;
    
    app.load_css(r#"
        .container {
            display: flex;
            flex-direction: column;
            gap: 1rem;
            padding: 2rem;
        }
    "#)?;
    
    app.set_component(MyComponent::new())?;
    app.run().await
}
```

## TuiAppBuilder

Builder pattern for TuiApp configuration.

```rust
impl TuiAppBuilder {
    pub fn new() -> Self
    pub fn title(self, title: &str) -> Self
    pub fn stylesheet(self, path: &str) -> Self
    pub fn theme(self, theme: ColorTheme) -> Self
    pub fn component<C: Component>(self, component: C) -> Self
    pub fn with_driver(self, driver: Box<dyn Driver>) -> Self
    pub fn build(self) -> Result<TuiApp>
}
```

**Example:**
```rust
let app = TuiApp::builder()
    .title("Dashboard")
    .stylesheet("assets/styles.css")
    .theme(ColorTheme::dark())
    .component(DashboardComponent::new())
    .build()?;
```

## Component Trait

Core trait for all UI components.

```rust
pub trait Component: Send + Sync {
    /// Render the component to an Element
    fn render(&self) -> Element;
    
    /// Optional lifecycle methods
    fn mount(&mut self) -> Result<()> { Ok(()) }
    fn unmount(&mut self) -> Result<()> { Ok(()) }
    fn update(&mut self) -> Result<()> { Ok(()) }
    
    /// Event handling
    fn handle_message(&mut self, message: &dyn Message) -> Result<()> { Ok(()) }
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> { 
        Ok(ActionResult::Ignored) 
    }
}
```

**Example:**
```rust
#[derive(Debug)]
struct CounterComponent {
    count: i32,
}

impl Component for CounterComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("counter")
            .child(
                Element::with_tag("h1")
                    .content(&format!("Count: {}", self.count))
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .content("Increment")
                    .attr("onclick", "increment")
                    .build()
            )
            .build()
    }
    
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        match action.name() {
            "increment" => {
                self.count += 1;
                Ok(ActionResult::Handled)
            }
            _ => Ok(ActionResult::Ignored)
        }
    }
}
```

## Element

DOM-like element for component structure and styling.

### Constructor

```rust
impl Element {
    pub fn new() -> ElementBuilder
    pub fn with_tag(tag: &str) -> ElementBuilder
}
```

### ElementBuilder

```rust
impl ElementBuilder {
    // Identity
    pub fn id(self, id: &str) -> Self
    pub fn class(self, class: &str) -> Self
    pub fn tag(self, tag: &str) -> Self
    
    // Content
    pub fn content(self, content: &str) -> Self
    pub fn attr(self, name: &str, value: &str) -> Self
    
    // Hierarchy
    pub fn child(self, child: Element) -> Self
    pub fn children(self, children: Vec<Element>) -> Self
    
    // Interactivity
    pub fn focusable(self, tab_index: Option<i32>) -> Self
    pub fn on_click(self, action: &str) -> Self
    pub fn on_key(self, key: KeyCombination, action: &str) -> Self
    
    // Build
    pub fn build(self) -> Element
}
```

**Example:**
```rust
let form = Element::with_tag("form")
    .class("login-form")
    .child(
        Element::with_tag("input")
            .id("username")
            .attr("type", "text")
            .attr("placeholder", "Username")
            .focusable(Some(1))
            .build()
    )
    .child(
        Element::with_tag("button")
            .content("Login")
            .class("btn primary")
            .on_click("submit_login")
            .focusable(Some(2))
            .build()
    )
    .build();
```

## Error Types

Comprehensive error handling with TuiError.

```rust
#[derive(Debug, thiserror::Error)]
pub enum TuiError {
    #[error("Render error: {0}")]
    Render(String),
    
    #[error("CSS error: {0}")]
    Css(String),
    
    #[error("Layout error: {0}")]
    Layout(String),
    
    #[error("Component error: {0}")]
    Component(String),
    
    #[error("Driver error: {0}")]
    Driver(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("Configuration error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, TuiError>;
```

**Error Handling Example:**
```rust
use reactive_tui::prelude::*;

fn create_component() -> Result<Element> {
    Element::with_tag("div")
        .class("component")
        .content("Hello World")
        .build()
        .map_err(|e| TuiError::Component(format!("Failed to build element: {}", e)))
}

fn main() {
    match create_component() {
        Ok(element) => println!("Component created successfully"),
        Err(TuiError::Component(msg)) => eprintln!("Component error: {}", msg),
        Err(e) => eprintln!("Unexpected error: {}", e),
    }
}
```

## Result Type

Standard Result type used throughout the API.

```rust
pub type Result<T> = std::result::Result<T, TuiError>;
```

All fallible operations return `Result<T>` for consistent error handling patterns across the entire API.