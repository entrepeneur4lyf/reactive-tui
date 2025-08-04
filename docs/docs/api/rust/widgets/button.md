# Button Widget

Comprehensive interactive button component with 8 button types, 5 sizes, 6 states, 9 border styles, and advanced Unicode rendering for rich terminal user interfaces.

## Overview

The Button widget provides a complete clickable interface element with extensive customization options including visual styles, sizes, states, border rendering, accessibility features, and ResponsiveWidget trait integration. Features advanced Unicode border rendering with 9 different border styles and sophisticated visual feedback.

```rust
use reactive_tui::widgets::{button, Button, ButtonType, ButtonSize, BorderStyle};

// Basic button with fluent API
let button = button("my-button")
    .text("Click Me")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Medium)
    .border_style(BorderStyle::Rounded)
    .build();

// Advanced styled button
let styled_button = button("advanced-btn")
    .text("🚀 Launch")
    .button_type(ButtonType::Success)
    .size(ButtonSize::Large)
    .border_style(BorderStyle::BracketCorners)
    .state(ButtonState::Focused)
    .build();
```

## Builder Pattern

### ButtonBuilder

Comprehensive builder for creating fully customized buttons.

```rust
impl ButtonBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn text<S: Into<String>>(mut self, text: S) -> Self
    pub fn button_type(mut self, button_type: ButtonType) -> Self
    pub fn size(mut self, size: ButtonSize) -> Self
    pub fn state(mut self, state: ButtonState) -> Self
    pub fn border_style(mut self, border_style: BorderStyle) -> Self
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self
    pub fn tooltip<S: Into<String>>(mut self, tooltip: S) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn loading(mut self, loading: bool) -> Self
    pub fn on_click<F>(mut self, callback: F) -> Self
    where F: Fn() -> Result<()> + Send + Sync + 'static
    pub fn build(self) -> Button
}
```

### Convenience Function

```rust
pub fn button<S: Into<String>>(id: S) -> ButtonBuilder
```

## Core Components

### Button

Main button widget with comprehensive functionality.

```rust
pub struct Button {
    pub id: String,
    pub text: String,
    pub button_type: ButtonType,
    pub size: ButtonSize,
    pub state: ButtonState,
    pub border_style: BorderStyle,
    pub icon: Option<String>,
    pub tooltip: Option<String>,
    pub disabled: bool,
    pub loading: bool,
    pub on_click: Option<Arc<dyn Fn() -> Result<()> + Send + Sync>>,
    pub reactive_state: Option<Arc<ReactiveState>>,
}
```

### ButtonType

Eight distinct button types for different UI contexts.

```rust
pub enum ButtonType {
    Primary,    // Main call-to-action button with prominent styling
    Secondary,  // Secondary action with subdued styling
    Success,    // Success/confirmation action with green theming
    Warning,    // Warning action with amber/yellow theming
    Danger,     // Destructive action with red theming
    Info,       // Informational action with blue theming
    Ghost,      // Transparent background with border styling
    Link,       // Text-only link style without borders
}
```

### ButtonSize

Five size variants for different interface contexts.

```rust
pub enum ButtonSize {
    ExtraSmall, // Minimal padding for compact interfaces (1 char padding)
    Small,      // Compact size for secondary actions (2 char padding)
    Medium,     // Default size for standard buttons (3 char padding)
    Large,      // Prominent size for primary actions (4 char padding)
    Block,      // Full width button spanning available space
}
```

### ButtonState

Six interactive states with visual feedback.

```rust
pub enum ButtonState {
    Normal,   // Default resting state
    Hover,    // Mouse hover with highlight effect
    Active,   // Being pressed/clicked with pressed effect
    Focused,  // Keyboard focus with focus indicators
    Disabled, // Cannot be interacted with, grayed out
    Loading,  // Async operation in progress with spinner
}
```

### BorderStyle

Nine unique border rendering styles with custom Unicode characters.

```rust
pub enum BorderStyle {
    None,           // No border, text only
    Simple,         // Basic ASCII border: [Button]
    Rounded,        // Rounded corners: ╭─Button─╮
    Double,         // Double line border: ╔═Button═╗
    Thick,          // Thick border: ┏━Button━┓
    PseudoRounded,  // Pseudo rounded: /‾Button‾\
    BracketCorners, // Bracket corners: ┌[Button]┐
    CurlyHooks,     // Curly hooks: ╭{Button}╮
    ArrowEnds,      // Arrow endings: ◄─Button─►
}
```

## Methods

### Construction

```rust
impl Button {
    // Create a new button with default settings
    pub fn new<S: Into<String>>(id: S) -> Self
    
    // Create a builder for fluent configuration
    pub fn builder<S: Into<String>>(id: S) -> ButtonBuilder
}
```

### Configuration

```rust
impl Button {
    // Set button text
    pub fn text<S: Into<String>>(mut self, text: S) -> Self
    
    // Set button type
    pub fn button_type(mut self, button_type: ButtonType) -> Self
    
    // Set button size
    pub fn size(mut self, size: ButtonSize) -> Self
    
    // Set button state
    pub fn state(mut self, state: ButtonState) -> Self
    
    // Set border style
    pub fn border_style(mut self, border_style: BorderStyle) -> Self
    
    // Set icon
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self
    
    // Set tooltip
    pub fn tooltip<S: Into<String>>(mut self, tooltip: S) -> Self
    
    // Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self
    
    // Set loading state
    pub fn loading(mut self, loading: bool) -> Self
}
```

### State Management

```rust
impl Button {
    // Check if button is currently disabled
    pub fn is_disabled(&self) -> bool
    
    // Check if button is in loading state
    pub fn is_loading(&self) -> bool
    
    // Get current button state
    pub fn get_state(&self) -> &ButtonState
    
    // Update button state
    pub fn set_state(&mut self, state: ButtonState)
}
```

### Rendering

```rust
impl Button {
    // Render button with specified border style
    pub fn render_with_border(&self, border_style: &BorderStyle) -> String
    
    // Render simple bordered button
    pub fn render_simple(&self) -> String
    
    // Render rounded corner button
    pub fn render_rounded(&self) -> String
    
    // Render double-line border button
    pub fn render_double(&self) -> String
    
    // Render thick border button
    pub fn render_thick(&self) -> String
    
    // Render pseudo-rounded button
    pub fn render_pseudo_rounded(&self) -> String
    
    // Render bracket corners button
    pub fn render_bracket_corners(&self) -> String
    
    // Render curly hooks button
    pub fn render_curly_hooks(&self) -> String
    
    // Render arrow ends button
    pub fn render_arrow_ends(&self) -> String
    
    // Convert to Element for component system integration
    pub fn to_element(&self) -> Element
}
```

### Event Handling

```rust
impl Button {
    // Set click callback
    pub fn on_click<F>(mut self, callback: F) -> Self
    where F: Fn() -> Result<()> + Send + Sync + 'static
    
    // Trigger click event
    pub fn click(&self) -> Result<()>
}
```

### ResponsiveWidget Implementation

```rust
impl ResponsiveWidget for Button {
    fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String
    fn handle_event(&mut self, event: &Event) -> Result<bool>
    fn is_focusable(&self) -> bool
    fn set_focused(&mut self, focused: bool)
    fn min_size(&self) -> (u16, u16)
    fn preferred_size(&self) -> (u16, u16)
}
```

## Examples

### Basic Button Examples

```rust
use reactive_tui::widgets::{button, Button, ButtonType, ButtonSize, BorderStyle};

// Simple button with default styling
let simple_button = button("submit")
    .text("Submit")
    .build();

// Primary action button
let primary_button = button("save")
    .text("Save Changes")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .border_style(BorderStyle::Rounded)
    .build();

// Danger button with thick border
let delete_button = button("delete")
    .text("Delete Item")
    .button_type(ButtonType::Danger)
    .border_style(BorderStyle::Thick)
    .tooltip("Permanently delete this item")
    .build();
```

### Border Style Examples

```rust
// Showcase different border styles
let border_examples = vec![
    // Simple ASCII border
    button("simple")
        .text("Simple")
        .border_style(BorderStyle::Simple)
        .build(),
    
    // Rounded corners with Unicode
    button("rounded")
        .text("Rounded")
        .border_style(BorderStyle::Rounded)
        .build(),
    
    // Double-line border
    button("double")
        .text("Double")
        .border_style(BorderStyle::Double)
        .build(),
    
    // Thick Unicode border
    button("thick")
        .text("Thick")
        .border_style(BorderStyle::Thick)
        .build(),
    
    // Bracket corners style
    button("bracket")
        .text("Bracket")
        .border_style(BorderStyle::BracketCorners)
        .build(),
    
    // Curly hooks design
    button("curly")
        .text("Curly")
        .border_style(BorderStyle::CurlyHooks)
        .build(),
    
    // Arrow endings
    button("arrow")
        .text("Arrow")
        .border_style(BorderStyle::ArrowEnds)
        .build(),
];

println!("Border Style Examples:");
for (i, btn) in border_examples.iter().enumerate() {
    println!("Style {}: {}", i + 1, btn.render_with_border(&btn.border_style));
}
```

### Size Variations

```rust
// Different button sizes
let size_examples = vec![
    button("xs").text("XS").size(ButtonSize::ExtraSmall).build(),
    button("sm").text("Small").size(ButtonSize::Small).build(),
    button("md").text("Medium").size(ButtonSize::Medium).build(),
    button("lg").text("Large Button").size(ButtonSize::Large).build(),
    button("block").text("Block Button").size(ButtonSize::Block).build(),
];

println!("Size Examples:");
for btn in size_examples {
    println!("{}", btn.render_with_border(&BorderStyle::Rounded));
}
```

### Interactive Button with State Management

```rust
use reactive_tui::widgets::{button, ButtonState};
use std::sync::{Arc, Mutex};

let counter = Arc::new(Mutex::new(0));
let counter_clone = counter.clone();

let mut counter_button = button("counter")
    .text("Count: 0")
    .button_type(ButtonType::Info)
    .border_style(BorderStyle::Double)
    .on_click(move || {
        let mut count = counter_clone.lock().unwrap();
        *count += 1;
        println!("Button clicked {} times!", *count);
        Ok(())
    })
    .build();

// Update button text and state
counter_button.text(format!("Count: {}", counter.lock().unwrap()));
counter_button.set_state(ButtonState::Focused);
```

### Loading and Disabled States

```rust
// Loading button for async operations
let loading_button = button("loading")
    .text("Processing...")
    .button_type(ButtonType::Primary)
    .state(ButtonState::Loading)
    .disabled(true)
    .build();

// Disabled button
let disabled_button = button("disabled")
    .text("Unavailable")
    .button_type(ButtonType::Secondary)
    .state(ButtonState::Disabled)
    .disabled(true)
    .build();

// Button with hover state
let hover_button = button("hover")
    .text("Hover Me")
    .button_type(ButtonType::Success)
    .state(ButtonState::Hover)
    .border_style(BorderStyle::PseudoRounded)
    .build();
```

### Advanced Button Panel

```rust
struct ButtonPanel {
    buttons: Vec<Button>,
    active_index: usize,
}

impl ButtonPanel {
    fn new() -> Self {
        let buttons = vec![
            button("primary")
                .text("🚀 Launch")
                .button_type(ButtonType::Primary)
                .size(ButtonSize::Large)
                .border_style(BorderStyle::ArrowEnds)
                .tooltip("Launch the application")
                .build(),
            
            button("settings")
                .text("⚙️ Settings")
                .button_type(ButtonType::Secondary)
                .border_style(BorderStyle::CurlyHooks)
                .tooltip("Open settings panel")
                .build(),
            
            button("help")
                .text("❓ Help")
                .button_type(ButtonType::Info)
                .border_style(BorderStyle::BracketCorners)
                .tooltip("Show help documentation")
                .build(),
            
            button("exit")
                .text("❌ Exit")
                .button_type(ButtonType::Danger)
                .border_style(BorderStyle::Thick)
                .tooltip("Exit the application")
                .build(),
        ];
        
        Self {
            buttons,
            active_index: 0,
        }
    }
    
    fn render(&self) -> String {
        let mut output = String::new();
        output.push_str("Button Panel\n");
        output.push_str("═".repeat(40).as_str());
        output.push('\n');
        
        for (i, button) in self.buttons.iter().enumerate() {
            let indicator = if i == self.active_index { "► " } else { "  " };
            let rendered = button.render_with_border(&button.border_style);
            output.push_str(&format!("{}{} {}\n", indicator, i + 1, rendered));
        }
        
        output
    }
    
    fn navigate_next(&mut self) {
        self.active_index = (self.active_index + 1) % self.buttons.len();
    }
    
    fn navigate_previous(&mut self) {
        self.active_index = if self.active_index == 0 { 
            self.buttons.len() - 1 
        } else { 
            self.active_index - 1 
        };
    }
    
    fn activate_current(&mut self) -> Result<()> {
        if let Some(button) = self.buttons.get(self.active_index) {
            button.click()?;
        }
        Ok(())
    }
}

// Usage example
let mut panel = ButtonPanel::new();
println!("{}", panel.render());

// Navigate and activate
panel.navigate_next();
panel.activate_current()?;
```

## CSS Styling

Buttons support extensive CSS customization with semantic classes:

```css
.button {
    /* Base button styles */
    display: inline-block;
    cursor: pointer;
    user-select: none;
    transition: all 0.2s ease;
}

/* Button Types */
.button-primary {
    color: var(--primary-button-text, #ffffff);
    background-color: var(--primary-button-bg, #3b82f6);
    border-color: var(--primary-button-border, #3b82f6);
}

.button-secondary {
    color: var(--secondary-button-text, #374151);
    background-color: var(--secondary-button-bg, #f3f4f6);
    border-color: var(--secondary-button-border, #d1d5db);
}

.button-success {
    color: var(--success-button-text, #ffffff);
    background-color: var(--success-button-bg, #10b981);
    border-color: var(--success-button-border, #10b981);
}

.button-warning {
    color: var(--warning-button-text, #92400e);
    background-color: var(--warning-button-bg, #fbbf24);
    border-color: var(--warning-button-border, #f59e0b);
}

.button-danger {
    color: var(--danger-button-text, #ffffff);
    background-color: var(--danger-button-bg, #ef4444);
    border-color: var(--danger-button-border, #ef4444);
}

.button-info {
    color: var(--info-button-text, #ffffff);
    background-color: var(--info-button-bg, #3b82f6);
    border-color: var(--info-button-border, #3b82f6);
}

.button-ghost {
    color: var(--ghost-button-text, #374151);
    background-color: transparent;
    border-color: var(--ghost-button-border, #d1d5db);
}

.button-link {
    color: var(--link-button-text, #3b82f6);
    background-color: transparent;
    border: none;
    text-decoration: underline;
}

/* Button Sizes */
.button-extra-small {
    padding: 2px 6px;
    font-size: 0.75rem;
}

.button-small {
    padding: 4px 8px;
    font-size: 0.875rem;
}

.button-medium {
    padding: 6px 12px;
    font-size: 1rem;
}

.button-large {
    padding: 8px 16px;
    font-size: 1.125rem;
}

.button-block {
    width: 100%;
    padding: 8px 16px;
    text-align: center;
}

/* Button States */
.button-hover {
    filter: brightness(1.1);
    transform: translateY(-1px);
}

.button-active {
    filter: brightness(0.9);
    transform: translateY(1px);
}

.button-focused {
    outline: 2px solid var(--focus-color, #3b82f6);
    outline-offset: 2px;
}

.button-disabled {
    opacity: 0.5;
    cursor: not-allowed;
    pointer-events: none;
}

.button-loading {
    position: relative;
    color: transparent;
}

.button-loading::before {
    content: "⏳";
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    color: currentColor;
    animation: spin 1s linear infinite;
}

/* Border Styles */
.button-border-simple {
    border: 1px solid currentColor;
}

.button-border-rounded {
    border-radius: 8px;
    border: 1px solid currentColor;
}

.button-border-double {
    border: 2px double currentColor;
}

.button-border-thick {
    border: 3px solid currentColor;
}

@keyframes spin {
    from { transform: translate(-50%, -50%) rotate(0deg); }
    to { transform: translate(-50%, -50%) rotate(360deg); }
}
```

## Accessibility

Buttons include comprehensive accessibility features:

- **ARIA Attributes**: Full ARIA support with `role="button"`, `aria-label`, `aria-pressed`
- **Keyboard Navigation**: Enter and Space key activation with proper focus management
- **Screen Reader Support**: Descriptive labels and state announcements
- **Focus Management**: Clear visual focus indicators and tab navigation
- **Disabled State**: Proper disabled semantics with `aria-disabled`
- **Loading State**: Screen reader announcements for async operations
- **Tooltip Integration**: Accessible tooltip descriptions

```rust
let accessible_button = button("accessible-save")
    .text("Save Document")
    .button_type(ButtonType::Primary)
    .tooltip("Save the current document to disk")
    .border_style(BorderStyle::Rounded)
    .build();

// The button automatically generates proper ARIA attributes:
// aria-label="Save Document"
// role="button"
// aria-pressed="false"
// tabindex="0"
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let button_toolbar = Element::with_tag("div")
    .class("button-toolbar")
    .child(
        Element::with_tag("div")
            .class("button-group")
            .child(primary_button.to_element())
            .child(secondary_button.to_element())
            .child(danger_button.to_element())
            .build()
    )
    .build();
```

## Performance Considerations

- **Lightweight Rendering**: Efficient Unicode character rendering with minimal memory usage
- **State Caching**: Button states are cached to avoid unnecessary re-renders
- **Event Optimization**: Click handlers are optimized for responsive interactions
- **Border Rendering**: Each border style has its own optimized rendering method
- **Memory Efficient**: Minimal allocation for text and border character composition

## Advanced Features

### Custom Border Rendering

```rust
// Each border style uses specific Unicode characters
let rounded_button = button("rounded")
    .text("Rounded")
    .border_style(BorderStyle::Rounded)
    .build();

// Renders as: ╭─ Rounded ─╮
//            ╰─────────╯

let double_button = button("double")
    .text("Double")
    .border_style(BorderStyle::Double)
    .build();

// Renders as: ╔═ Double ═╗
//            ╚════════╝
```

### Dynamic State Updates

```rust
let mut dynamic_button = button("dynamic")
    .text("Click Me")
    .button_type(ButtonType::Primary)
    .build();

// Update state based on conditions
if user_action_required {
    dynamic_button.set_state(ButtonState::Focused);
} else if operation_in_progress {
    dynamic_button.set_state(ButtonState::Loading);
    dynamic_button = dynamic_button.disabled(true);
}
```

### Reactive State Integration

```rust
use reactive_tui::{widgets::button, reactive::ReactiveState};
use std::sync::Arc;

// Create reactive state
let app_state = Arc::new(ReactiveState::new());

// Create button with reactive integration
let mut reactive_button = button("reactive")
    .text("Reactive Button")
    .button_type(ButtonType::Info)
    .build();

// Connect to reactive state
reactive_button.connect_reactive(app_state.clone())?;

// Watch for state changes
app_state.watch_field("button.enabled", |enabled: bool| {
    println!("Button enabled state: {}", enabled);
});

// Button state changes will be broadcast
reactive_button.set_state(ButtonState::Active);
```

The Button widget provides a comprehensive foundation for user interactions in terminal applications with 8 button types, 5 sizes, 6 states, 9 unique border styles with Unicode rendering, extensive customization options, reactive state management, accessibility support, and seamless integration with the reactive-tui component system.