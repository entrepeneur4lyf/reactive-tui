# Input Widget

CSS-styled comprehensive input field widget with full CSS integration, theme support, validation, various input types, cursor management, and advanced rendering capabilities.

## Overview

The Input widget provides a complete text input experience with CSS styling system integration, theme framework support, comprehensive validation, cursor management, selection handling, scrolling for long text, and extensive customization options. Features both modern utility CSS and legacy theme rendering.

```rust
use reactive_tui::widgets::{input, Input, InputType, InputConfig, BorderStyle};

// Fluent factory function
let username_input = input("username", |c| {
    c.placeholder("Enter your username")
     .input_type(InputType::Text)
     .required(true)
     .max_length(50)
     .class("form-input")
});

// Builder pattern
let styled_input = Input::builder("styled")
    .input_type(InputType::Email)
    .placeholder("user@example.com")
    .css_class("border-rounded")
    .css_class("px-4")
    .css_class("py-2")
    .border_style(BorderStyle::Rounded)
    .build();
```

## Methods

### Construction

```rust
impl Input {
    // Create a new input with default settings
    pub fn new(id: impl Into<String>) -> Self
    
    // Create a builder for fluent API
    pub fn builder(id: impl Into<String>) -> InputBuilder
}
```

### Value Management

```rust
impl Input {
    // Set the input value
    pub fn set_value(&mut self, value: impl Into<String>)
    
    // Get the current input value
    pub fn value(&self) -> &str
    
    // Clear the input
    pub fn clear(&mut self)
    
    // Insert character at cursor position
    pub fn insert_char(&mut self, ch: char)
    
    // Delete character at cursor position
    pub fn delete_char(&mut self)
}
```

### Focus Management

```rust
impl Input {
    // Set focus state
    pub fn set_focused(&mut self, focused: bool)
    
    // Check if input has focus
    pub fn is_focused(&self) -> bool
}
```

### Cursor Control

```rust
impl Input {
    // Move cursor left
    pub fn move_cursor_left(&mut self)
    
    // Move cursor right
    pub fn move_cursor_right(&mut self)
    
    // Move cursor to start
    pub fn move_cursor_home(&mut self)
    
    // Move cursor to end
    pub fn move_cursor_end(&mut self)
}
```

### Validation

```rust
impl Input {
    // Validate the current input value
    pub fn validate(&mut self)
    
    // Simple email validation (basic check)
    fn is_valid_email(&self, email: &str) -> bool
}
```

### Rendering

```rust
impl Input {
    // Render with utility CSS classes and theme support
    pub fn render_with_utilities(
        &self,
        layout: &LayoutRect,
        utility_processor: &UtilityProcessor,
    ) -> String
    
    // Render with CSS styling and theme support (legacy method)
    pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String
}
```

### Event Handling

```rust
impl Input {
    // Handle keyboard input events
    pub fn handle_key_event(&mut self, key: &str) -> bool
}
```

## Core Components

### Input

Main CSS-styled input field widget with comprehensive functionality.

```rust
pub struct Input {
    pub id: String,
    pub input_type: InputType,
    pub state: InputState,
    pub style: InputStyle,
    pub placeholder: String,
    pub css_classes: Vec<String>,
    pub inline_styles: HashMap<String, String>,
    pub theme: Option<String>,
    pub validator: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub required: bool,
    pub autocomplete: Vec<String>,
}
```

### InputType

Six input types with specific behaviors and validation.

```rust
pub enum InputType {
    /// Single-line text input
    Text,
    /// Password input (masked with asterisks)
    Password,
    /// Numeric input with number validation
    Number,
    /// Email input with email format validation
    Email,
    /// Search input for search functionality
    Search,
    /// Multi-line text area input
    TextArea,
}
```

### InputState

Comprehensive state management for input behavior.

```rust
pub struct InputState {
    /// Current input value
    pub value: String,
    /// Cursor position in the value
    pub cursor_position: usize,
    /// Whether the input has focus
    pub focused: bool,
    /// Current validation state
    pub validation_state: ValidationState,
    /// Validation error message
    pub validation_message: Option<String>,
    /// Whether the input is disabled
    pub disabled: bool,
    /// Whether the input is read-only
    pub readonly: bool,
    /// Selection start (for text selection)
    pub selection_start: Option<usize>,
    /// Selection end (for text selection)
    pub selection_end: Option<usize>,
    /// Scroll offset for long text
    pub scroll_offset: usize,
}
```

### ValidationState

Validation states with async support.

```rust
pub enum ValidationState {
    /// No validation performed yet
    None,
    /// Input is valid
    Valid,
    /// Input has validation errors
    Invalid,
    /// Currently validating (async)
    Validating,
}
```

### InputStyle

Advanced styling configuration for visual appearance.

```rust
pub struct InputStyle {
    /// Border style for the input
    pub border_style: BorderStyle,
    /// Whether to show a border
    pub show_border: bool,
    /// Padding inside the input (horizontal, vertical)
    pub padding: (u16, u16),
    /// Minimum width of the input
    pub min_width: u16,
    /// Maximum width of the input (0 = unlimited)
    pub max_width: u16,
    /// Height for multi-line inputs
    pub height: u16,
    /// Whether to show placeholder text
    pub show_placeholder: bool,
    /// Cursor character (default: '│')
    pub cursor_char: char,
    /// Whether cursor blinks
    pub cursor_blink: bool,
}
```

### InputConfig

Factory configuration for widget creation.

```rust
pub struct InputConfig {
    pub id: String,
    pub input_type: InputType,
    pub placeholder: String,
    pub classes: Vec<String>,
    pub attributes: HashMap<String, String>,
    pub disabled: bool,
    pub visible: bool,
    pub focusable: bool,
    pub tab_index: Option<i32>,
    pub theme: Option<String>,
    pub validator: Option<String>,
    pub max_length: Option<usize>,
    pub min_length: Option<usize>,
    pub pattern: Option<String>,
    pub required: bool,
    pub autocomplete: Vec<String>,
    pub value: String,
}
```

## Builder Patterns

### InputBuilder

Fluent builder for Input widget creation.

```rust
impl InputBuilder {
    pub fn input_type(mut self, input_type: InputType) -> Self
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
    pub fn value(mut self, value: impl Into<String>) -> Self
    pub fn css_class(mut self, class: impl Into<String>) -> Self
    pub fn css_classes(mut self, classes: Vec<String>) -> Self
    pub fn inline_style(mut self, property: impl Into<String>, value: impl Into<String>) -> Self
    pub fn theme(mut self, theme: impl Into<String>) -> Self
    pub fn max_length(mut self, max_length: usize) -> Self
    pub fn min_length(mut self, min_length: usize) -> Self
    pub fn required(mut self, required: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn readonly(mut self, readonly: bool) -> Self
    pub fn border_style(mut self, style: BorderStyle) -> Self
    pub fn show_border(mut self, show: bool) -> Self
    pub fn padding(mut self, horizontal: u16, vertical: u16) -> Self
    pub fn width(mut self, min: u16, max: u16) -> Self
    pub fn height(mut self, height: u16) -> Self
    pub fn autocomplete(mut self, suggestions: Vec<String>) -> Self
    pub fn build(self) -> Input
}
```

### InputConfig Builder

Factory pattern configuration builder.

```rust
impl InputConfig {
    pub fn new(id: &str) -> Self
    pub fn input_type(mut self, input_type: InputType) -> Self
    pub fn placeholder(mut self, placeholder: &str) -> Self
    pub fn value(mut self, value: &str) -> Self
    pub fn class(mut self, class: &str) -> Self
    pub fn classes(mut self, classes: &[&str]) -> Self
    pub fn attribute(mut self, key: &str, value: &str) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn required(mut self, required: bool) -> Self
    pub fn max_length(mut self, max_length: usize) -> Self
    pub fn min_length(mut self, min_length: usize) -> Self
    pub fn pattern(mut self, pattern: &str) -> Self
    pub fn validator(mut self, validator: &str) -> Self
    pub fn autocomplete(mut self, suggestion: &str) -> Self
    pub fn theme(mut self, theme: &str) -> Self
    pub fn build(self) -> Input
}
```

### Convenience Functions

```rust
// Factory function with closure configuration
pub fn input<F>(id: &str, f: F) -> Input
where F: FnOnce(InputConfig) -> InputConfig

// Direct config-based creation
pub fn create_input(config: InputConfig) -> Input
```

## Examples

### Basic Input Examples

```rust
use reactive_tui::widgets::{input, Input, InputType, BorderStyle};

// Factory function approach
let username_input = input("username", |c| {
    c.placeholder("Enter your username")
     .required(true)
     .max_length(50)
     .class("form-input")
     .class("px-3")
     .class("py-2")
});

// Builder pattern approach
let email_input = Input::builder("email")
    .input_type(InputType::Email)
    .placeholder("user@example.com")
    .css_class("border-rounded")
    .css_class("focus:border-blue")
    .required(true)
    .max_length(100)
    .build();

// Direct creation
let simple_input = Input::new("simple");
```

### Input Type Examples

```rust
// Password input with masking
let password_input = input("password", |c| {
    c.input_type(InputType::Password)
     .placeholder("Enter password")
     .min_length(8)
     .required(true)
     .class("form-control")
});

// Number input with validation
let age_input = input("age", |c| {
    c.input_type(InputType::Number)
     .placeholder("Enter age")
     .pattern(r"^\d+$")
     .class("number-input")
});

// Search input
let search_input = input("search", |c| {
    c.input_type(InputType::Search)
     .placeholder("Search products...")
     .class("search-field")
     .class("w-full")
});

// Multi-line text area
let comment_input = Input::builder("comment")
    .input_type(InputType::TextArea)
    .placeholder("Enter your comment...")
    .height(4)
    .css_class("textarea")
    .css_class("resize-none")
    .build();
```

### CSS Styling Examples

```rust
// Utility CSS classes
let styled_input = input("styled", |c| {
    c.placeholder("Styled input")
     .class("bg-gray-100")
     .class("border-2")
     .class("border-blue-300")
     .class("rounded-lg")
     .class("px-4")
     .class("py-2")
     .class("focus:border-blue-500")
     .class("focus:bg-white")
});

// Custom border styling
let border_input = Input::builder("border")
    .placeholder("Custom border")
    .border_style(BorderStyle::Double)
    .show_border(true)
    .padding(2, 1)
    .build();

// Inline styles
let inline_styled = Input::builder("inline")
    .placeholder("Inline styled")
    .inline_style("background-color", "#f0f0f0")
    .inline_style("color", "#333")
    .inline_style("border-radius", "8px")
    .build();
```

### Validation Examples

```rust
// Required field validation
let required_input = input("required", |c| {
    c.placeholder("This field is required")
     .required(true)
     .class("required-field")
});

// Length validation
let length_input = input("length", |c| {
    c.placeholder("5-20 characters")
     .min_length(5)
     .max_length(20)
     .class("length-validated")
});

// Pattern validation (regex)
let pattern_input = input("pattern", |c| {
    c.placeholder("Only letters and numbers")
     .pattern(r"^[a-zA-Z0-9]+$")
     .class("pattern-input")
});

// Email format validation
let email_validated = input("email-val", |c| {
    c.input_type(InputType::Email)
     .placeholder("Valid email required")
     .required(true)
});
```

### Interactive Input Management

```rust
use reactive_tui::widgets::{Input, InputType, ValidationState};

struct InputManager {
    inputs: Vec<Input>,
    active_index: usize,
}

impl InputManager {
    fn new() -> Self {
        let inputs = vec![
            input("name", |c| {
                c.placeholder("Full Name")
                 .required(true)
                 .max_length(100)
                 .class("form-input")
            }),
            
            input("email", |c| {
                c.input_type(InputType::Email)
                 .placeholder("Email Address")
                 .required(true)
                 .class("form-input")
            }),
            
            input("phone", |c| {
                c.input_type(InputType::Number)
                 .placeholder("Phone Number")
                 .pattern(r"^\d{10}$")
                 .class("form-input")
            }),
        ];
        
        Self {
            inputs,
            active_index: 0,
        }
    }
    
    fn handle_input(&mut self, key: &str) {
        if let Some(input) = self.inputs.get_mut(self.active_index) {
            match key {
                "Tab" => self.next_input(),
                "Shift+Tab" => self.previous_input(),
                "Enter" => self.submit_form(),
                _ => {
                    input.handle_key_event(key);
                }
            }
        }
    }
    
    fn next_input(&mut self) {
        if let Some(current) = self.inputs.get_mut(self.active_index) {
            current.set_focused(false);
        }
        
        self.active_index = (self.active_index + 1) % self.inputs.len();
        
        if let Some(next) = self.inputs.get_mut(self.active_index) {
            next.set_focused(true);
        }
    }
    
    fn previous_input(&mut self) {
        if let Some(current) = self.inputs.get_mut(self.active_index) {
            current.set_focused(false);
        }
        
        self.active_index = if self.active_index == 0 {
            self.inputs.len() - 1
        } else {
            self.active_index - 1
        };
        
        if let Some(prev) = self.inputs.get_mut(self.active_index) {
            prev.set_focused(true);
        }
    }
    
    fn validate_all(&mut self) -> bool {
        let mut all_valid = true;
        
        for input in &mut self.inputs {
            input.validate();
            if input.state.validation_state == ValidationState::Invalid {
                all_valid = false;
            }
        }
        
        all_valid
    }
    
    fn submit_form(&mut self) {
        if self.validate_all() {
            println!("Form submitted successfully!");
            // Process form data
            for input in &self.inputs {
                println!("{}: {}", input.id, input.value());
            }
        } else {
            println!("Please fix validation errors");
        }
    }
}
```

### Advanced Styling with Themes

```rust
use reactive_tui::widgets::{Input, InputType};
use reactive_tui::themes::{BorderStyle, ColorTheme};

// Theme-aware inputs
let light_input = Input::builder("light")
    .placeholder("Light theme input")
    .theme("light")
    .css_class("light-input")
    .border_style(BorderStyle::Rounded)
    .build();

let dark_input = Input::builder("dark")
    .placeholder("Dark theme input")
    .theme("dark")
    .css_class("dark-input")
    .border_style(BorderStyle::Thick)
    .build();

// Custom themed form
struct ThemedForm {
    theme: String,
    inputs: Vec<Input>,
}

impl ThemedForm {
    fn new(theme: &str) -> Self {
        let theme_classes = match theme {
            "dark" => vec!["dark-bg", "dark-text", "dark-border"],
            "light" => vec!["light-bg", "light-text", "light-border"],
            "high-contrast" => vec!["hc-bg", "hc-text", "hc-border"],
            _ => vec!["default-bg", "default-text", "default-border"],
        };
        
        let inputs = vec![
            Input::builder("themed-name")
                .placeholder("Name")
                .theme(theme)
                .css_classes(theme_classes.iter().map(|s| s.to_string()).collect())
                .build(),
            
            Input::builder("themed-email")
                .input_type(InputType::Email)
                .placeholder("Email")
                .theme(theme)
                .css_classes(theme_classes.iter().map(|s| s.to_string()).collect())
                .build(),
        ];
        
        Self {
            theme: theme.to_string(),
            inputs,
        }
    }
    
    fn switch_theme(&mut self, new_theme: &str) {
        self.theme = new_theme.to_string();
        
        let theme_classes = match new_theme {
            "dark" => vec!["dark-bg", "dark-text", "dark-border"],
            "light" => vec!["light-bg", "light-text", "light-border"],
            "high-contrast" => vec!["hc-bg", "hc-text", "hc-border"],
            _ => vec!["default-bg", "default-text", "default-border"],
        };
        
        for input in &mut self.inputs {
            input.theme = Some(new_theme.to_string());
            input.css_classes = theme_classes.iter().map(|s| s.to_string()).collect();
        }
    }
}
```

### Autocomplete Integration

```rust
// Input with autocomplete suggestions
let autocomplete_input = Input::builder("autocomplete")
    .placeholder("Start typing...")
    .autocomplete(vec![
        "Apple".to_string(),
        "Banana".to_string(),
        "Cherry".to_string(),
        "Date".to_string(),
        "Elderberry".to_string(),
    ])
    .css_class("autocomplete-input")
    .build();

// Dynamic autocomplete with filtering
struct AutocompleteInput {
    input: Input,
    suggestions: Vec<String>,
    filtered_suggestions: Vec<String>,
}

impl AutocompleteInput {
    fn new(id: &str, suggestions: Vec<String>) -> Self {
        let input = Input::builder(id)
            .placeholder("Type to search...")
            .css_class("autocomplete")
            .build();
        
        Self {
            input,
            suggestions: suggestions.clone(),
            filtered_suggestions: suggestions,
        }
    }
    
    fn update_suggestions(&mut self) {
        let query = self.input.value().to_lowercase();
        
        if query.is_empty() {
            self.filtered_suggestions = self.suggestions.clone();
        } else {
            self.filtered_suggestions = self.suggestions
                .iter()
                .filter(|suggestion| {
                    suggestion.to_lowercase().contains(&query)
                })
                .cloned()
                .collect();
        }
    }
    
    fn handle_key(&mut self, key: &str) -> bool {
        let handled = self.input.handle_key_event(key);
        
        if handled {
            self.update_suggestions();
        }
        
        handled
    }
}
```

### Keyboard Event Handling

```rust
use reactive_tui::widgets::Input;

fn handle_input_events(input: &mut Input, key: &str) -> bool {
    match key {
        // Navigation keys
        "ArrowLeft" => {
            input.move_cursor_left();
            true
        }
        "ArrowRight" => {
            input.move_cursor_right();
            true
        }
        "Home" => {
            input.move_cursor_home();
            true
        }
        "End" => {
            input.move_cursor_end();
            true
        }
        
        // Editing keys
        "Backspace" => {
            input.delete_char();
            true
        }
        "Delete" => {
            if input.state.cursor_position < input.value().len() {
                input.state.value.remove(input.state.cursor_position);
                input.validate();
            }
            true
        }
        
        // Enter key behavior
        "Enter" => {
            if input.input_type == InputType::TextArea {
                input.insert_char('\n');
            }
            // For single-line inputs, Enter might submit form
            true
        }
        
        // Regular character input
        _ if key.len() == 1 => {
            if let Some(ch) = key.chars().next() {
                if ch.is_ascii_graphic() || ch == ' ' {
                    input.insert_char(ch);
                    return true;
                }
            }
            false
        }
        
        _ => false,
    }
}

// Example usage in event loop
fn process_input_event(input: &mut Input, key_event: KeyEvent) -> bool {
    let key = format_key_event(key_event);
    handle_input_events(input, &key)
}
```

## CSS Styling

The input widget supports extensive CSS customization with semantic classes:

```css
.input {
    /* Base input styles */
    display: block;
    width: 100%;
    padding: 8px 12px;
    border: 1px solid var(--input-border-color, #d1d5db);
    border-radius: 4px;
    font-size: 14px;
    line-height: 1.4;
    background-color: var(--input-bg-color, #ffffff);
    color: var(--input-text-color, #374151);
    transition: all 0.2s ease;
}

/* Input States */
.input:focus {
    outline: none;
    border-color: var(--input-focus-border, #3b82f6);
    box-shadow: 0 0 0 3px var(--input-focus-shadow, rgba(59, 130, 246, 0.1));
    background-color: var(--input-focus-bg, #ffffff);
}

.input:disabled {
    background-color: var(--input-disabled-bg, #f9fafb);
    color: var(--input-disabled-text, #9ca3af);
    cursor: not-allowed;
    opacity: 0.6;
}

.input:readonly {
    background-color: var(--input-readonly-bg, #f3f4f6);
    cursor: default;
}

.input.invalid {
    border-color: var(--input-error-border, #ef4444);
    background-color: var(--input-error-bg, #fef2f2);
}

.input.invalid:focus {
    border-color: var(--input-error-border, #ef4444);
    box-shadow: 0 0 0 3px var(--input-error-shadow, rgba(239, 68, 68, 0.1));
}

.input.valid {
    border-color: var(--input-success-border, #10b981);
    background-color: var(--input-success-bg, #f0fdf4);
}

/* Input Types */
.input[type="password"] {
    font-family: 'Courier New', monospace;
    letter-spacing: 2px;
}

.input[type="number"] {
    text-align: right;
}

.input[type="search"] {
    padding-right: 32px; /* Space for search icon */
}

.input.textarea {
    resize: vertical;
    min-height: 80px;
    line-height: 1.6;
}

/* Placeholder Styling */
.input-placeholder {
    color: var(--input-placeholder-color, #9ca3af);
    font-style: italic;
    opacity: 0.7;
}

/* Size Variants */
.input-sm {
    padding: 4px 8px;
    font-size: 12px;
}

.input-lg {
    padding: 12px 16px;
    font-size: 16px;
}

.input-xl {
    padding: 16px 20px;
    font-size: 18px;
}

/* Border Styles */
.input-border-none {
    border: none;
    border-bottom: 2px solid var(--input-border-color, #d1d5db);
    border-radius: 0;
}

.input-border-thick {
    border-width: 2px;
}

.input-border-rounded {
    border-radius: 8px;
}

.input-border-pill {
    border-radius: 50px;
}

/* Validation Message */
.input-error-message {
    color: var(--error-text-color, #ef4444);
    font-size: 12px;
    margin-top: 4px;
    display: flex;
    align-items: center;
    gap: 4px;
}

.input-error-message::before {
    content: "⚠";
    font-size: 14px;
}

/* Theme Variants */
.input-dark {
    background-color: var(--dark-input-bg, #374151);
    color: var(--dark-input-text, #f3f4f6);
    border-color: var(--dark-input-border, #6b7280);
}

.input-dark:focus {
    border-color: var(--dark-input-focus, #60a5fa);
    background-color: var(--dark-input-focus-bg, #4b5563);
}

.input-light {
    background-color: var(--light-input-bg, #ffffff);
    color: var(--light-input-text, #1f2937);
    border-color: var(--light-input-border, #d1d5db);
}

/* Utility Classes */
.w-full {
    width: 100%;
}

.px-3 {
    padding-left: 12px;
    padding-right: 12px;
}

.py-2 {
    padding-top: 8px;
    padding-bottom: 8px;
}

.rounded-lg {
    border-radius: 8px;
}

.border-2 {
    border-width: 2px;
}

.focus\:border-blue:focus {
    border-color: #3b82f6;
}

.bg-gray-100 {
    background-color: #f3f4f6;
}
```

## Accessibility

- **ARIA Attributes**: Full ARIA support with `role="textbox"`, `aria-label`, `aria-invalid`
- **Keyboard Navigation**: Complete keyboard accessibility with arrow keys, Home/End, text selection
- **Screen Reader Support**: Proper field descriptions and validation announcements
- **Focus Management**: Clear visual focus indicators and logical tab navigation
- **Validation Announcements**: Screen reader announcements for validation state changes
- **Placeholder Accessibility**: Proper placeholder handling that doesn't interfere with screen readers

## Performance Considerations

- **Efficient Rendering**: Optimized text rendering with cursor positioning and scrolling
- **CSS Processing**: Fast utility class processing with caching
- **Validation Optimization**: Incremental validation only when value changes
- **Memory Efficient**: Minimal memory usage for text storage and state management
- **Event Handling**: Optimized keyboard event processing for responsive typing

## Advanced Features

### Custom Validation with Regex

```rust
let pattern_input = input("custom-pattern", |c| {
    c.placeholder("Custom format (ABC-123)")
     .pattern(r"^[A-Z]{3}-\d{3}$")
     .class("pattern-input")
});

// Validation will automatically check against the regex pattern
```

### Multi-line Text Areas

```rust
let textarea = Input::builder("description")
    .input_type(InputType::TextArea)
    .placeholder("Enter description...")
    .height(6)
    .css_class("textarea")
    .css_class("resize-vertical")
    .build();

// Handles Enter key for newlines, scroll management for long text
```

### Scroll Management for Long Text

```rust
// The input automatically handles horizontal scrolling for long text
let long_input = input("long-text", |c| {
    c.placeholder("This input handles long text with scrolling")
     .max_length(500)
     .width(20, 0) // min_width: 20, max_width: unlimited
});

// Cursor position automatically adjusts scroll offset
```

The Input widget provides comprehensive text input functionality with CSS styling integration, advanced validation, theme support, and accessibility features for professional terminal applications.

The Input widget provides a comprehensive CSS-styled text input system with advanced features including utility CSS processing, theme integration, validation with regex support, cursor management, text selection, scrolling, accessibility compliance, and seamless integration with the reactive-tui component system.