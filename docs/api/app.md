---
title: App Module
description: TUI Application Core - Main application orchestration and lifecycle management
sidebar_position: 1
---

# App Module

Main application orchestration and lifecycle management for Reactive TUI applications.

This module provides the [`TuiApp`](#tuiapp) and [`TuiAppBuilder`](#tuiappbuilder) types for creating and running terminal user interface applications with CSS styling, event handling, and reactive state management.

## Quick Start

```rust
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct HelloApp;

impl Component for HelloApp {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("container")
            .content("Hello, Reactive TUI!")
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = TuiApp::builder()
        .component(HelloApp)
        .with_title("My TUI App")
        .build()?;
         
    app.run().await
}
```

## Features

- **CSS Styling**: Load stylesheets and apply CSS rules to components
- **Event Handling**: Keyboard, mouse, and custom event processing  
- **Focus Management**: Tab navigation and element focusing
- **Key Bindings**: Configurable keyboard shortcuts and actions
- **Hot Reload**: Live stylesheet reloading during development
- **Driver Abstraction**: Support for different terminal backends

## Structs

### TuiApp

TUI Application

The main application struct that orchestrates terminal UI components, styling, and event handling.

`TuiApp` manages the complete lifecycle of a terminal user interface application, including:

- CSS styling and layout computation
- Component rendering and updates
- Event processing (keyboard, mouse, custom events)
- Focus management and navigation
- Terminal driver abstraction

#### Examples

##### Basic Application

```rust
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct MyApp;

impl Component for MyApp {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("main")
            .content("Hello World!")
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = TuiApp::builder()
        .component(MyApp)
        .build()?;
         
    app.run().await
}
```

##### Application with Styling

```rust
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct MyApp;

impl Component for MyApp {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("main")
            .content("Styled Content")
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = TuiApp::builder()
        .component(MyApp)
        .stylesheet("styles.css")
        .with_title("Styled App")
        .build()?;

    // Load additional CSS at runtime
    app.load_css(".main { background: blue; color: white; }".to_string())?;
         
    app.run().await
}
```

#### Methods

##### `builder() -> TuiAppBuilder`

Creates a new [`TuiAppBuilder`](#tuiappbuilder) for configuring and building a TUI application.

This is the recommended way to create a new `TuiApp` instance with custom configuration.

**Examples:**

```rust
use reactive_tui::prelude::*;

let app = TuiApp::builder()
    .with_title("My App")
    .headless()
    .frame_rate(60)
    .build()?;
```

##### `load_stylesheets(&mut self) -> Result<()>`

Loads all configured stylesheets into the CSS engine.

This method reads CSS files from the filesystem and parses them into the internal CSS engine. It's called automatically during app initialization, but can be used to reload stylesheets manually.

**Errors:**

Returns a [`TuiError`](../error#tuierror) if:
- Any stylesheet file cannot be read
- CSS parsing fails for any stylesheet  
- The CSS engine lock cannot be acquired

**Examples:**

```rust
use reactive_tui::prelude::*;

let mut app = TuiApp::builder()
    .stylesheet("main.css")
    .stylesheet("theme.css")
    .build()?;
     
// Manually reload all stylesheets
app.load_stylesheets()?;
```

##### `reload_stylesheets(&mut self) -> Result<()>`

Reload all stylesheets (useful for hot reload)

##### `add_stylesheet<P: Into<PathBuf>>(&mut self, path: P) -> Result<()>`

Add a stylesheet at runtime

##### `set_title(&mut self, title: &str)`

Set the window title (delegates to driver)

##### `load_css(&mut self, css: String) -> Result<()>`

Load CSS from a string

##### `set_component(&mut self, component: Box<dyn Component>) -> Result<()>`

Set the root component

##### `has_component(&self) -> bool`

Check if a root component has been set

##### `run(self) -> Result<()>` (async)

Run the application

##### `stop(&self)` (async)

Stop the application

##### `bind_key(&self, key: KeyCombination, action: KeyAction)` (async)

Bind a key to an app-level action

##### `unbind_key(&self, key: &KeyCombination)` (async)

Unbind a key

##### `get_key_bindings_help(&self) -> String` (async)

Get help text for all key bindings

##### `register_action<F>(&mut self, action_name: &str, handler: F)`

Register an action handler

Where `F: Fn(&mut Action) -> ActionResult + Send + Sync + 'static`

##### `dispatch_action(&self, action: Action) -> ActionResult`

Dispatch an action immediately

##### `send_action(&self, action: Action) -> Result<()>`

Send an action for async processing

##### `action<S: Into<String>>(&self, name: S) -> ActionBuilder`

Create an action builder

##### `bind_key_to_action(&self, key: KeyCombination, action_name: &str)` (async)

Bind a key to an action

##### `bind_key_to_action_with_params(&self, key: KeyCombination, action_name: &str, params: Value)` (async)

Bind a key to an action with parameters

##### `driver(&self) -> &dyn Driver`

Get access to the underlying driver for advanced operations

##### `driver_mut(&mut self) -> &mut dyn Driver`

Get mutable access to the underlying driver for advanced operations

---

### TuiAppBuilder

TUI Application Builder

A builder pattern for configuring and creating [`TuiApp`](#tuiapp) instances.

`TuiAppBuilder` provides a fluent interface for setting up applications with custom configurations including stylesheets, components, driver settings, and performance options.

#### Configuration Options

- **Components**: Set the root component that defines the UI structure
- **Stylesheets**: Add CSS files for styling components
- **Driver Settings**: Configure terminal backend (crossterm, headless, etc.)
- **Performance**: Set frame rate and rendering options
- **Input Handling**: Enable/disable mouse support
- **Display Options**: Set title, inline mode, debug mode

#### Examples

##### Basic Configuration

```rust
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct MyComponent;

impl Component for MyComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .content("Hello World")
            .build()
    }
}

let app = TuiApp::builder()
    .component(MyComponent)
    .with_title("My App")
    .build()?;
```

##### Advanced Configuration

```rust
use reactive_tui::prelude::*;
use std::time::Duration;

#[derive(Debug, Clone)]
struct MyDashboard {
    title: String,
}

impl Component for MyDashboard {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("dashboard")
            .child(
                Element::with_tag("h1")
                    .content(&self.title)
                    .build()
            )
            .build()
    }
}

let dashboard = MyDashboard {
    title: "System Dashboard".to_string(),
};

let app = TuiApp::builder()
    .component(dashboard)
    .stylesheet("styles/main.css")
    .stylesheet("styles/theme.css")
    .with_title("Dashboard")
    .with_mouse(true)
    .frame_rate(60)
    .debug_mode(true)
    .build()?;
```

##### Testing Configuration

```rust
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct TestComponent {
    test_data: Vec<String>,
}

impl Component for TestComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("test-container")
            .child(
                Element::with_tag("ul")
                    .children(
                        self.test_data.iter().map(|item| {
                            Element::with_tag("li")
                                .content(item)
                                .build()
                        }).collect::<Vec<_>>()
                    )
                    .build()
            )
            .build()
    }
}

let test_component = TestComponent {
    test_data: vec!["Test 1".to_string(), "Test 2".to_string()],
};

let app = TuiApp::builder()
    .component(test_component)
    .headless()
    .with_size(80, 24)
    .build()?;
```

#### Methods

##### `new() -> Self`

Create a new builder instance

##### `stylesheet<P: Into<PathBuf>>(self, path: P) -> Self`

Add a stylesheet to load

##### `component<C: Component + 'static>(self, component: C) -> Self`

Set the root component

##### `driver_config(self, config: DriverConfig) -> Self`

Configure the driver (terminal backend)

##### `with_mouse(self, enabled: bool) -> Self`

Enable mouse support

##### `with_title<S: Into<String>>(self, title: S) -> Self`

Set terminal title

##### `inline_mode(self, enabled: bool) -> Self`

Enable inline mode (non-fullscreen)

##### `debug_mode(self, enabled: bool) -> Self`

Enable debug mode

##### `with_size(self, cols: u16, rows: u16) -> Self`

Set custom terminal size (for testing)

##### `headless(self) -> Self`

Use headless driver (for testing/automation)

##### `frame_rate(self, fps: u32) -> Self`

Set frame rate (default: 30 FPS)

##### `frame_duration(self, duration: Duration) -> Self`

Set frame rate from duration

##### `build(self) -> Result<TuiApp>`

Build the configured TuiApp instance

#### Trait Implementations

##### `Default`

```rust
fn default() -> Self
```

Returns the "default value" for a type.