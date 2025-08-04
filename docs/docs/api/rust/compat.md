# Compat Module

Compatibility layer providing interoperability with popular TUI libraries, migration utilities, and adapter patterns for seamless integration.

## Overview

The compat module enables integration with existing terminal UI libraries and provides migration paths from other frameworks to reactive-tui.

```rust
use reactive_tui::compat::{CrosstermAdapter, TuiAdapter, CursiveAdapter};

// Create adapters for different backends
let crossterm_adapter = CrosstermAdapter::new()?;
let tui_adapter = TuiAdapter::new()?;
let cursive_adapter = CursiveAdapter::new()?;
```

## Crossterm Integration

### CrosstermAdapter

Provides compatibility with the crossterm library for terminal manipulation.

```rust
use reactive_tui::compat::crossterm::{CrosstermAdapter, CrosstermConfig};
use crossterm::{
    terminal::{self, ClearType},
    cursor, queue, style,
    event::{self, Event, KeyCode},
};

let config = CrosstermConfig::builder()
    .enable_mouse(true)
    .enable_paste(true)
    .raw_mode(true)
    .alternate_screen(true)
    .build();

let mut adapter = CrosstermAdapter::with_config(config)?;

// Use crossterm APIs through adapter
adapter.execute(terminal::Clear(ClearType::All))?;
adapter.execute(cursor::MoveTo(10, 5))?;
adapter.queue(style::Print("Hello World"))?;
adapter.flush()?;
```

### Event Translation

```rust
use reactive_tui::compat::crossterm::CrosstermEventTranslator;

let translator = CrosstermEventTranslator::new();

// Convert crossterm events to reactive-tui events
match event::read()? {
    crossterm::event::Event::Key(key_event) => {
        let tui_event = translator.translate_key_event(key_event)?;
        handle_key_event(tui_event);
    },
    crossterm::event::Event::Mouse(mouse_event) => {
        let tui_event = translator.translate_mouse_event(mouse_event)?;
        handle_mouse_event(tui_event);
    },
    _ => {}
}
```

## Tui-rs Integration

### TuiAdapter

Compatibility layer for tui-rs (ratatui) applications.

```rust
use reactive_tui::compat::tui::{TuiAdapter, TuiBackend};
use tui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Constraint, Direction},
    Terminal,
};

// Create adapter with tui-rs backend
let backend = CrosstermBackend::new(std::io::stdout());
let mut terminal = Terminal::new(backend)?;
let adapter = TuiAdapter::new(terminal)?;

// Use tui-rs widgets within reactive-tui
let tui_widget = Paragraph::new("Hello from tui-rs")
    .block(Block::default().title("TUI Widget").borders(Borders::ALL));

let reactive_element = adapter.wrap_tui_widget(tui_widget)?;
```

### Layout Translation

```rust
use reactive_tui::compat::tui::LayoutTranslator;

let translator = LayoutTranslator::new();

// Convert tui-rs layout to reactive-tui layout
let tui_layout = Layout::default()
    .direction(Direction::Vertical)
    .constraints([
        Constraint::Length(3),
        Constraint::Min(0),
        Constraint::Length(3),
    ]);

let reactive_layout = translator.translate_layout(tui_layout)?;
```

## Cursive Integration

### CursiveAdapter

Adapter for Cursive library applications.

```rust
use reactive_tui::compat::cursive::{CursiveAdapter, CursiveViewAdapter};
use cursive::{
    views::{TextView, EditView, Button, LinearLayout},
    Cursive,
};

let mut cursive = Cursive::default();
let adapter = CursiveAdapter::new(cursive)?;

// Wrap Cursive views
let text_view = TextView::new("Hello from Cursive");
let reactive_text = adapter.wrap_view(text_view)?;

let edit_view = EditView::new().content("Initial text");
let reactive_input = adapter.wrap_view(edit_view)?;
```

### Event Bridge

```rust
use reactive_tui::compat::cursive::CursiveEventBridge;

let bridge = CursiveEventBridge::new();

// Forward reactive-tui events to Cursive
bridge.forward_event_to_cursive(&reactive_event, &mut cursive_app)?;

// Forward Cursive callbacks to reactive-tui
bridge.forward_cursive_callback(cursive_callback, &reactive_app)?;
```

## Migration Utilities

### Migration Assistant

```rust
use reactive_tui::compat::migration::{MigrationAssistant, MigrationReport};

let assistant = MigrationAssistant::new();

// Analyze existing codebase
let report = assistant.analyze_codebase("./src")?;

println!("Migration Report:");
println!("- Files to migrate: {}", report.files_count);
println!("- Widgets found: {}", report.widgets_found.len());
println!("- Estimated effort: {:?}", report.effort_estimate);

// Generate migration suggestions
for suggestion in report.suggestions {
    println!("Suggestion: {}", suggestion.description);
    println!("File: {}", suggestion.file_path);
    println!("Line: {}", suggestion.line_number);
    println!("Replacement: {}", suggestion.replacement);
}
```

### Code Transformation

```rust
use reactive_tui::compat::migration::CodeTransformer;

let transformer = CodeTransformer::new();

// Transform tui-rs code to reactive-tui
let original_code = r#"
use tui::widgets::{Block, Borders, Paragraph};

let widget = Paragraph::new("Hello")
    .block(Block::default().borders(Borders::ALL));
"#;

let transformed_code = transformer.transform_tui_rs_code(original_code)?;
println!("Transformed code:\n{}", transformed_code);
```

## Backend Abstraction

### UniversalBackend

Unified backend that can work with multiple terminal libraries.

```rust
use reactive_tui::compat::backend::{UniversalBackend, BackendType};

let backend = UniversalBackend::auto_detect()?;

match backend.backend_type() {
    BackendType::Crossterm => {
        println!("Using Crossterm backend");
    },
    BackendType::Termion => {
        println!("Using Termion backend");
    },
    BackendType::Pancurses => {
        println!("Using Pancurses backend");
    },
    BackendType::Custom(name) => {
        println!("Using custom backend: {}", name);
    },
}

// Use unified API regardless of backend
backend.clear_screen()?;
backend.move_cursor(10, 5)?;
backend.print("Hello World")?;
backend.flush()?;
```

### Backend Selection

```rust
use reactive_tui::compat::backend::BackendSelector;

let selector = BackendSelector::new()
    .prefer(BackendType::Crossterm)
    .fallback(BackendType::Termion)
    .fallback(BackendType::Pancurses);

let backend = selector.select_best_available()?;
```

## Widget Adapters

### Generic Widget Wrapper

```rust
use reactive_tui::compat::widgets::{WidgetWrapper, ExternalWidget};

// Wrap any external widget
pub struct ExternalButtonWrapper {
    inner: external_lib::Button,
}

impl WidgetWrapper for ExternalButtonWrapper {
    type Inner = external_lib::Button;
    
    fn wrap(inner: Self::Inner) -> Self {
        Self { inner }
    }
    
    fn inner(&self) -> &Self::Inner {
        &self.inner
    }
    
    fn inner_mut(&mut self) -> &mut Self::Inner {
        &mut self.inner
    }
}

impl ResponsiveWidget for ExternalButtonWrapper {
    fn to_element(&self) -> Element {
        // Convert external widget to reactive-tui element
        Element::with_tag("button")
            .text(&self.inner.text())
            .build()
    }
    
    fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
        // Render using external widget's render method
        self.inner.render(layout.width, layout.height)
    }
    
    fn min_size(&self) -> (u16, u16) {
        let size = self.inner.preferred_size();
        (size.width, size.height)
    }
}
```

### Widget Translation

```rust
use reactive_tui::compat::widgets::WidgetTranslator;

let translator = WidgetTranslator::new();

// Register translation rules
translator.register_translation("tui::Paragraph", |tui_widget| {
    Element::with_tag("p")
        .text(&tui_widget.text())
        .class("paragraph")
        .build()
});

translator.register_translation("cursive::TextView", |cursive_view| {
    Element::with_tag("div")
        .text(&cursive_view.get_content())
        .class("text-view")
        .build()
});

// Translate widgets
let reactive_widget = translator.translate(external_widget)?;
```

## Style Translation

### CSS Translation

```rust
use reactive_tui::compat::style::{StyleTranslator, TuiStyleConverter};

let converter = TuiStyleConverter::new();

// Convert tui-rs styles to reactive-tui CSS
let tui_style = tui::style::Style::default()
    .fg(tui::style::Color::Blue)
    .bg(tui::style::Color::White)
    .add_modifier(tui::style::Modifier::BOLD);

let css_styles = converter.convert_style(tui_style)?;
// Returns: "color: blue; background-color: white; font-weight: bold;"
```

### Theme Conversion

```rust
use reactive_tui::compat::theme::{ThemeConverter, ExternalTheme};

let converter = ThemeConverter::new();

// Convert external theme to reactive-tui theme
let external_theme = load_external_theme("theme.toml")?;
let reactive_theme = converter.convert_theme(external_theme)?;

// Apply converted theme
theme_manager.register_theme("converted", reactive_theme);
theme_manager.set_active_theme("converted")?;
```

## Async Integration

### Async Runtime Compatibility

```rust
use reactive_tui::compat::async_runtime::{AsyncRuntimeAdapter, RuntimeType};

// Support different async runtimes
let adapter = AsyncRuntimeAdapter::detect_runtime()?;

match adapter.runtime_type() {
    RuntimeType::Tokio => {
        println!("Using Tokio runtime");
    },
    RuntimeType::AsyncStd => {
        println!("Using async-std runtime");
    },
    RuntimeType::Smol => {
        println!("Using smol runtime");
    },
}

// Spawn tasks using the detected runtime
adapter.spawn(async {
    // Async task
    let data = fetch_data().await?;
    update_ui(data);
});
```

### Future Compatibility

```rust
use reactive_tui::compat::futures::{FutureAdapter, StreamAdapter};

// Integrate with external futures
let external_future = external_lib::fetch_data();
let adapted_future = FutureAdapter::new(external_future);

let result = adapted_future.await?;
```

## Testing Compatibility

### Test Adapters

```rust
use reactive_tui::compat::testing::{TestAdapter, MockTerminal};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_with_mock_terminal() {
        let mock_terminal = MockTerminal::new(80, 24);
        let adapter = TestAdapter::new(mock_terminal);
        
        // Test application with mocked terminal
        let app = create_test_app(adapter);
        app.render().unwrap();
        
        // Verify output
        let output = adapter.get_output();
        assert!(output.contains("Expected text"));
    }
}
```

## Integration Examples

### Gradual Migration

```rust
use reactive_tui::{
    prelude::*,
    compat::{TuiAdapter, MigrationAssistant},
};

// Gradual migration from tui-rs
fn migrate_application() -> Result<()> {
    let assistant = MigrationAssistant::new();
    
    // Start with hybrid approach
    let tui_backend = CrosstermBackend::new(std::io::stdout());
    let mut terminal = Terminal::new(tui_backend)?;
    let adapter = TuiAdapter::new(terminal)?;
    
    // Use existing tui-rs widgets alongside reactive-tui
    let legacy_widget = tui::widgets::Paragraph::new("Legacy widget");
    let new_widget = Button::new("new-btn", "New Widget");
    
    let app = TuiApp::builder()
        .add_legacy_widget(adapter.wrap_tui_widget(legacy_widget)?)
        .add_widget(new_widget)
        .build()?;
    
    app.run()
}
```

### Multi-Library Integration

```rust
use reactive_tui::compat::{CrosstermAdapter, CursiveAdapter, TuiAdapter};

fn create_multi_library_app() -> Result<TuiApp> {
    let mut app = TuiApp::builder();
    
    // Integrate widgets from different libraries
    if cfg!(feature = "crossterm") {
        let crossterm_widget = create_crossterm_widget()?;
        app = app.add_external_widget("crossterm", crossterm_widget);
    }
    
    if cfg!(feature = "cursive") {
        let cursive_widget = create_cursive_widget()?;
        app = app.add_external_widget("cursive", cursive_widget);
    }
    
    if cfg!(feature = "tui-rs") {
        let tui_widget = create_tui_widget()?;
        app = app.add_external_widget("tui-rs", tui_widget);
    }
    
    app.build()
}
```

The compat module makes it easy to integrate reactive-tui into existing projects or migrate from other terminal UI libraries while maintaining compatibility and reducing migration effort.