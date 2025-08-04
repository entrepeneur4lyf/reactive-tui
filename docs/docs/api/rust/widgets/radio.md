# Radio Widget

Radio button group widget for single selection from a group with customizable appearance, labels, and reactive state integration.

## Overview

The RadioGroup widget provides grouped radio buttons with support for horizontal and vertical layouts, custom styling, and keyboard navigation. Only one option can be selected at a time within a group.

```rust
use reactive_tui::widgets::*;

let radio_group = RadioGroup::new("theme_selection")
    .option("light", "Light Theme")
    .option("dark", "Dark Theme")
    .option("auto", "Auto")
    .selected("dark")
    .orientation(RadioOrientation::Vertical)
    .chars('●', '○')
    .spacing(1);
```

## Core Components

### RadioGroup

Main radio button group widget.

```rust
pub struct RadioGroup {
    pub id: String,
    pub options: Vec<RadioOption>,
    pub state: RadioState,
    pub style: RadioStyle,
    pub reactive_state: Option<Arc<ReactiveState>>,
}
```

### RadioOption

Individual radio button option.

```rust
pub struct RadioOption {
    pub value: String,
    pub label: String,
    pub enabled: bool,
    pub description: Option<String>,
}

impl RadioOption {
    pub fn new<V: Into<String>, L: Into<String>>(value: V, label: L) -> Self
    pub fn enabled(mut self, enabled: bool) -> Self
    pub fn description<S: Into<String>>(mut self, description: S) -> Self
}
```

### RadioState

Radio button group state management.

```rust
pub struct RadioState {
    pub selected: Option<String>,
    pub interactive: bool,
    pub focused_index: usize,
}
```

### RadioStyle

Visual styling configuration.

```rust
pub struct RadioStyle {
    pub selected_char: char,      // Default: '●'
    pub unselected_char: char,    // Default: '○'
    pub spacing: u16,             // Default: 1
    pub show_labels: bool,        // Default: true
    pub orientation: RadioOrientation,
}
```

### RadioOrientation

Layout orientation options.

```rust
pub enum RadioOrientation {
    Vertical,
    Horizontal,
}
```

## Builder Pattern

### RadioGroupBuilder

```rust
impl RadioGroupBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn option<V: Into<String>, L: Into<String>>(mut self, value: V, label: L) -> Self
    pub fn options<I, V, L>(mut self, options: I) -> Self
    pub fn selected<S: Into<String>>(mut self, value: S) -> Self
    pub fn interactive(mut self, interactive: bool) -> Self
    pub fn orientation(mut self, orientation: RadioOrientation) -> Self
    pub fn chars(mut self, selected: char, unselected: char) -> Self
    pub fn spacing(mut self, spacing: u16) -> Self
    pub fn build(self) -> RadioGroup
}
```

## Methods

### Selection Management

```rust
impl RadioGroup {
    // Select an option by value
    pub fn select<S: Into<String>>(&mut self, value: S) -> Result<()>
    
    // Get currently selected value
    pub fn get_selected(&self) -> Option<&String>
    
    // Get currently selected option
    pub fn selected_option(&self) -> Option<&RadioOption>
}
```

### Navigation

```rust
impl RadioGroup {
    // Move focus to next option
    pub fn focus_next(&mut self) -> Result<()>
    
    // Move focus to previous option  
    pub fn focus_previous(&mut self) -> Result<()>
    
    // Select currently focused option
    pub fn select_focused(&mut self) -> Result<()>
}
```

### Reactive State

```rust
impl RadioGroup {
    // Connect to reactive state for live updates
    pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()>
}
```

## Examples

### Basic Radio Group

```rust
use reactive_tui::widgets::*;

let color_selection = RadioGroup::new("color")
    .option("red", "Red")
    .option("green", "Green")  
    .option("blue", "Blue")
    .selected("blue")
    .orientation(RadioOrientation::Vertical);
```

### Horizontal Layout

```rust
let size_selection = RadioGroup::new("size")
    .option("small", "S")
    .option("medium", "M")
    .option("large", "L")
    .option("xlarge", "XL")
    .selected("medium")
    .orientation(RadioOrientation::Horizontal)
    .spacing(2);
```

### Custom Characters

```rust
let priority_selection = RadioGroup::new("priority")
    .option("low", "Low Priority")
    .option("medium", "Medium Priority")
    .option("high", "High Priority")
    .chars('◉', '◯')
    .selected("medium");
```

### With Descriptions

```rust
let theme_options = vec![
    RadioOption::new("light", "Light Theme")
        .description("Clean white background with dark text"),
    RadioOption::new("dark", "Dark Theme")  
        .description("Dark background with light text"),
    RadioOption::new("auto", "Auto Theme")
        .description("Follows system preference"),
];

let theme_radio = RadioGroup::new("theme")
    .options(theme_options.into_iter().map(|opt| (opt.value, opt.label)))
    .selected("auto");
```

### Disabled Options

```rust
let subscription_tiers = RadioGroup::new("subscription")
    .option("free", "Free Tier")
    .option("pro", "Pro Tier")
    .option("enterprise", "Enterprise Tier");

// Disable enterprise option for this user
if let Some(enterprise_option) = subscription_tiers.options.iter_mut()
    .find(|opt| opt.value == "enterprise") {
    enterprise_option.enabled = false;
}
```

### With Reactive State

```rust
use reactive_tui::{widgets::*, reactive::ReactiveState};

let state = Arc::new(ReactiveState::new());
let mut language_selection = RadioGroup::new("language")
    .option("rust", "Rust")
    .option("javascript", "JavaScript")
    .option("python", "Python")
    .selected("rust");

language_selection.connect_reactive(state.clone())?;

// React to selection changes
let language_clone = state.clone();
state.watch_field("language_selection.selected", move |value| {
    println!("Language changed to: {}", value);
});
```

### Form Integration

```rust
use reactive_tui::{widgets::*, components::*};

let preferences_form = Element::with_tag("form")
    .class("user-preferences")
    .child(
        Element::with_tag("fieldset")
            .child(
                Element::with_tag("legend")
                    .text("Theme Preference")
                    .build()
            )
            .child(
                RadioGroup::new("theme")
                    .option("light", "Light")
                    .option("dark", "Dark")
                    .option("auto", "Auto")
                    .selected("auto")
                    .to_element()
            )
            .build()
    )
    .child(
        Element::with_tag("fieldset")
            .child(
                Element::with_tag("legend")
                    .text("Notification Settings")
                    .build()
            )
            .child(
                RadioGroup::new("notifications")
                    .option("all", "All Notifications")
                    .option("important", "Important Only")
                    .option("none", "No Notifications")
                    .selected("important")
                    .to_element()
            )
            .build()
    )
    .build();
```

### Dynamic Options

```rust
let mut platform_selection = RadioGroup::new("platform");

// Add options dynamically based on available platforms
let available_platforms = get_available_platforms();
for (id, name) in available_platforms {
    platform_selection = platform_selection.option(id, name);
}

if let Some(default_platform) = get_default_platform() {
    platform_selection = platform_selection.selected(default_platform);
}
```

## Accessibility

The RadioGroup widget provides comprehensive accessibility support:

- **ARIA Attributes**: Uses `radiogroup` role with proper `aria-activedescendant`
- **Keyboard Navigation**: Arrow keys for navigation, Space/Enter for selection
- **Focus Management**: Visual focus indicators and tab navigation
- **Screen Reader Support**: Proper labels and state announcements

## CSS Classes

The widget generates the following CSS classes for styling:

```css
.radiogroup {
    /* Base radio group styles */
}

.radiogroup.radio-disabled {
    /* Disabled state */
}

.radiogroup.radio-vertical {
    /* Vertical orientation */
}

.radiogroup.radio-horizontal {
    /* Horizontal orientation */
}
```

## Key Bindings

Default keyboard shortcuts:

- **Arrow Keys**: Navigate between options
- **Space/Enter**: Select current option
- **Tab**: Move focus to/from group

## Convenience Function

```rust
// Create a radio group with builder pattern
pub fn radio_group<S: Into<String>>(id: S) -> RadioGroupBuilder {
    RadioGroupBuilder::new(id)
}
```

## Integration

The RadioGroup widget integrates seamlessly with:

- **Reactive State System**: For real-time updates and state management
- **CSS Styling**: Full CSS class and styling support
- **Form Validation**: Integration with FormValidator widget
- **Layout System**: ResponsiveWidget trait implementation
- **Event System**: Keyboard and mouse event handling