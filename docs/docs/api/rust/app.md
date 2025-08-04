# App Module

The `app` module provides the main application struct and builder for creating TUI applications.

## TuiApp

The main application struct that manages the entire TUI application lifecycle.

```rust
use reactive_tui::app::TuiApp;

let app = TuiApp::new("My App")?;
app.run().await?;
```

### Methods

#### `new(title: &str) -> Result<TuiApp>`
Creates a new TUI application with the specified title.

**Parameters:**
- `title` - The window/application title

**Returns:** `Result<TuiApp, TuiError>`

#### `run(self) -> Result<()>`
Starts the main event loop and runs the application.

**Returns:** `Result<(), TuiError>`

#### `add_widget<W: Widget>(&mut self, widget: W) -> Result<()>`
Adds a widget to the application.

**Parameters:**
- `widget` - Any type implementing the `Widget` trait

**Returns:** `Result<(), TuiError>`

#### `set_theme(&mut self, theme: Theme)`
Sets the application theme.

**Parameters:**
- `theme` - A `Theme` instance defining colors and styling

#### `add_stylesheet(&mut self, stylesheet: Stylesheet) -> Result<()>`
Adds a CSS stylesheet to the application.

**Parameters:**
- `stylesheet` - A parsed CSS stylesheet

**Returns:** `Result<(), TuiError>`

## TuiAppBuilder

Builder pattern implementation for creating TUI applications with configuration.

```rust
use reactive_tui::app::TuiAppBuilder;

let app = TuiAppBuilder::new()
    .title("My App")
    .fps(60)
    .theme(dark_theme())
    .build()?;
```

### Methods

#### `new() -> TuiAppBuilder`
Creates a new application builder with default settings.

#### `title(mut self, title: &str) -> Self`
Sets the application title.

**Parameters:**
- `title` - The application title

**Returns:** Self for method chaining

#### `fps(mut self, fps: u32) -> Self`
Sets the target frames per second.

**Parameters:**
- `fps` - Target FPS (default: 60)

**Returns:** Self for method chaining

#### `theme(mut self, theme: Theme) -> Self`
Sets the application theme.

**Parameters:**
- `theme` - Theme configuration

**Returns:** Self for method chaining

#### `size(mut self, width: u16, height: u16) -> Self`
Sets the initial application size.

**Parameters:**
- `width` - Initial width in terminal columns
- `height` - Initial height in terminal rows

**Returns:** Self for method chaining

#### `build(self) -> Result<TuiApp>`
Builds and returns the configured TUI application.

**Returns:** `Result<TuiApp, TuiError>`

## Example

```rust
use reactive_tui::{TuiAppBuilder, widgets::Button, themes::dark_theme};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut app = TuiAppBuilder::new()
        .title("My Reactive TUI App")
        .fps(60)
        .theme(dark_theme())
        .size(80, 24)
        .build()?;

    let button = Button::new("main-btn", "Click me!");
    app.add_widget(button)?;

    app.run().await?;
    Ok(())
}
```