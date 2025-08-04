# Error Module

Comprehensive error handling system with structured error types, error propagation, and debugging support for robust terminal applications.

## TuiError

Main error type providing detailed error information with context and recovery suggestions.

```rust
use reactive_tui::error::{TuiError, Result};

fn example_function() -> Result<()> {
    match risky_operation() {
        Ok(value) => Ok(()),
        Err(e) => Err(TuiError::render(format!("Failed to render: {}", e))),
    }
}
```

### Error Categories

```rust
pub enum TuiError {
    // Rendering errors
    Render(String),
    Layout(String),
    Style(String),
    
    // Input/Output errors
    Io(std::io::Error),
    Terminal(String),
    
    // Configuration errors
    Config(String),
    Theme(String),
    
    // State management errors
    State(String),
    Reactive(String),
    
    // Widget errors
    Widget(String),
    Component(String),
    
    // System errors
    System(String),
    Memory(String),
    
    // Custom application errors
    Custom(String),
}
```

## Error Construction

### Static Error Creation

```rust
use reactive_tui::error::TuiError;

// Specific error types
let render_error = TuiError::render("Buffer overflow during rendering");
let layout_error = TuiError::layout("Invalid grid dimensions");
let widget_error = TuiError::widget("Button widget initialization failed");

// Generic error with context
let custom_error = TuiError::custom("User authentication failed")
    .with_context("login_module")
    .with_suggestion("Check username and password");
```

### Error Chain Building

```rust
use reactive_tui::error::{TuiError, ErrorChain};

let error_chain = ErrorChain::new()
    .root_cause("Network connection failed")
    .add_context("data_fetcher", "Failed to fetch user data")
    .add_context("user_widget", "Cannot render user profile")
    .build();

let error = TuiError::from_chain(error_chain);
```

## Result Type

Convenient `Result` type alias for TUI operations:

```rust
use reactive_tui::error::Result;

pub type Result<T> = std::result::Result<T, TuiError>;

// Usage in functions
fn create_widget(config: WidgetConfig) -> Result<Widget> {
    // Function implementation
}

fn render_frame() -> Result<()> {
    // Function implementation
}
```

## Error Context

### Adding Context Information

```rust
use reactive_tui::error::{TuiError, ErrorContext};

let error = TuiError::io(io_error)
    .with_context("file_operations")
    .with_file("config.json")
    .with_line(42)
    .with_suggestion("Check file permissions and path");
```

### Context Types

```rust
pub struct ErrorContext {
    pub module: Option<String>,
    pub function: Option<String>,
    pub file: Option<String>,
    pub line: Option<u32>,
    pub suggestion: Option<String>,
    pub user_data: HashMap<String, String>,
}
```

## Error Propagation

### Using the `?` Operator

```rust
use reactive_tui::error::Result;

fn complex_operation() -> Result<String> {
    let config = load_config()?;
    let theme = load_theme(&config.theme_path)?;
    let widget = create_widget(&config, &theme)?;
    Ok(widget.render()?)
}
```

### Error Mapping

```rust
use reactive_tui::error::TuiError;

fn operation_with_mapping() -> Result<()> {
    std::fs::read_to_string("config.json")
        .map_err(|e| TuiError::config(format!("Failed to read config: {}", e)))?;
    Ok(())
}
```

## Error Recovery

### Recoverable Errors

```rust
use reactive_tui::error::{TuiError, RecoveryAction};

fn resilient_operation() -> Result<Widget> {
    match create_primary_widget() {
        Ok(widget) => Ok(widget),
        Err(error) => {
            log::warn!("Primary widget creation failed: {}", error);
            
            // Attempt recovery
            match error.recovery_action() {
                Some(RecoveryAction::UseDefault) => Ok(create_default_widget()),
                Some(RecoveryAction::Retry) => create_primary_widget(),
                None => Err(error),
            }
        }
    }
}
```

### Error Handlers

```rust
use reactive_tui::error::{ErrorHandler, TuiError};

struct AppErrorHandler;

impl ErrorHandler for AppErrorHandler {
    fn handle_error(&self, error: &TuiError) -> bool {
        match error {
            TuiError::Render(_) => {
                // Log error and continue
                log::error!("Render error: {}", error);
                true // Error handled
            },
            TuiError::System(_) => {
                // Critical error, cannot recover
                log::fatal!("System error: {}", error);
                false // Error not handled
            },
            _ => {
                // Default handling
                log::error!("Error: {}", error);
                true
            }
        }
    }
}

// Register error handler
let handler = AppErrorHandler;
TuiError::set_global_handler(Box::new(handler));
```

## Debugging Support

### Error Formatting

```rust
use reactive_tui::error::TuiError;

let error = TuiError::widget("Button initialization failed")
    .with_context("ui_builder")
    .with_suggestion("Check widget configuration");

// Different formatting options
println!("Display: {}", error);
println!("Debug: {:?}", error);
println!("Verbose: {:#?}", error);

// Custom formatting
println!("Custom: {}", error.format_with_context());
```

### Stack Traces

```rust
use reactive_tui::error::ErrorTrace;

fn traced_function() -> Result<()> {
    let trace = ErrorTrace::capture();
    
    risky_operation()
        .map_err(|e| TuiError::custom("Operation failed")
            .with_trace(trace)
            .with_context("traced_function"))
}
```

## Error Logging Integration

### Log Level Mapping

```rust
use reactive_tui::error::{TuiError, LogLevel};

impl TuiError {
    pub fn log_level(&self) -> LogLevel {
        match self {
            TuiError::System(_) => LogLevel::Fatal,
            TuiError::Render(_) => LogLevel::Error,
            TuiError::Widget(_) => LogLevel::Warn,
            TuiError::Config(_) => LogLevel::Info,
            _ => LogLevel::Debug,
        }
    }
}

// Automatic logging
error.log_with_level();
```

### Structured Logging

```rust
use reactive_tui::error::StructuredError;

let structured = StructuredError::from_error(&error)
    .add_field("user_id", "12345")
    .add_field("session_id", "abcdef")
    .add_field("timestamp", chrono::Utc::now().to_rfc3339());

log::error!("{}", structured.to_json());
```

## Validation Errors

### Field Validation

```rust
use reactive_tui::error::{ValidationError, ValidationResult};

fn validate_user_input(input: &UserInput) -> ValidationResult<()> {
    let mut errors = Vec::new();
    
    if input.username.is_empty() {
        errors.push(ValidationError::required_field("username"));
    }
    
    if input.password.len() < 8 {
        errors.push(ValidationError::min_length("password", 8));
    }
    
    if !input.email.contains('@') {
        errors.push(ValidationError::invalid_format("email", "Must contain @"));
    }
    
    if errors.is_empty() {
        Ok(())
    } else {
        Err(ValidationError::multiple(errors))
    }
}
```

### Form Validation Integration

```rust
use reactive_tui::{error::ValidationError, widgets::FormValidator};

let validator = FormValidator::new()
    .field("username", |value| {
        if value.len() < 3 {
            Err(ValidationError::min_length("username", 3))
        } else {
            Ok(())
        }
    })
    .field("email", |value| {
        if !value.contains('@') {
            Err(ValidationError::invalid_format("email", "Invalid email format"))
        } else {
            Ok(())
        }
    });
```

## Async Error Handling

### Future Error Handling

```rust
use reactive_tui::error::{TuiError, Result};
use tokio::time::timeout;

async fn async_operation() -> Result<String> {
    let result = timeout(Duration::from_secs(5), fetch_data())
        .await
        .map_err(|_| TuiError::custom("Operation timed out"))?
        .map_err(|e| TuiError::io(e))?;
    
    Ok(result)
}
```

### Stream Error Handling

```rust
use reactive_tui::error::TuiError;
use futures::StreamExt;

async fn process_stream() -> Result<()> {
    let mut stream = data_stream();
    
    while let Some(result) = stream.next().await {
        match result {
            Ok(data) => process_data(data)?,
            Err(e) => {
                let error = TuiError::custom(format!("Stream error: {}", e));
                if error.is_recoverable() {
                    log::warn!("Recoverable stream error: {}", error);
                    continue;
                } else {
                    return Err(error);
                }
            }
        }
    }
    
    Ok(())
}
```

## Testing Support

### Error Testing Utilities

```rust
use reactive_tui::error::{TuiError, ErrorTestHelpers};

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_creation() {
        let error = TuiError::widget("Test error");
        assert!(error.is_widget_error());
        assert_eq!(error.message(), "Test error");
    }
    
    #[test]
    fn test_error_chain() {
        let result = function_that_fails();
        assert!(result.is_err());
        
        let error = result.unwrap_err();
        assert!(error.has_context("expected_context"));
    }
}
```

## Example Usage

```rust
use reactive_tui::{
    error::{TuiError, Result, ErrorHandler},
    widgets::Button,
    components::Element,
};

// Error-aware widget creation
fn create_dashboard() -> Result<Element> {
    let header = create_header()
        .map_err(|e| TuiError::widget("Failed to create header").with_source(e))?;
    
    let sidebar = create_sidebar()
        .map_err(|e| TuiError::widget("Failed to create sidebar").with_source(e))?;
    
    let main_content = create_main_content()
        .map_err(|e| TuiError::widget("Failed to create main content").with_source(e))?;
    
    let dashboard = Element::with_tag("div")
        .class("dashboard")
        .child(header)
        .child(sidebar)
        .child(main_content)
        .build();
    
    Ok(dashboard)
}

// Application-level error handling
fn main() -> Result<()> {
    // Set up error handler
    TuiError::set_global_handler(Box::new(AppErrorHandler));
    
    // Run application with error handling
    match run_app() {
        Ok(()) => {
            log::info!("Application completed successfully");
            Ok(())
        },
        Err(error) => {
            log::error!("Application error: {}", error.format_with_context());
            
            // Attempt graceful shutdown
            if let Err(shutdown_error) = graceful_shutdown() {
                log::error!("Shutdown error: {}", shutdown_error);
            }
            
            Err(error)
        }
    }
}
```