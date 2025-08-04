# Widgets

Comprehensive collection of interactive UI components for terminal applications.

This module provides 25+ pre-built widgets that integrate seamlessly with the TUI framework's reactive state management, CSS styling, and event systems. All widgets implement responsive design principles and support theming, accessibility, and keyboard navigation.

## Widget Categories

### Layout Widgets

- **Grid**: Advanced grid layouts with column/row definitions (see [`layout`](./layout.md))
- **[Bar](./widgets/bar.md)**: Header/footer bars with flexible positioning
- **[Tabs](./widgets/tabs.md)**: Tab navigation with multiple orientations
- **[Modal](./widgets/modal.md)**: Overlay dialogs with backdrop
- **[Accordion](./widgets/accordion.md)**: Expandable/collapsible sections

### Form Controls

- **[Input](./widgets/input.md)**: Text input with validation
- **[Button](./widgets/button.md)**: Interactive buttons with states
- **[Checkbox](./widgets/checkbox.md)**: Single and grouped checkboxes
- **[Switch](./widgets/switch.md)**: Toggle switches with labels
- **[Radio](./widgets/radio.md)**: Radio button groups
- **[Select](./widgets/select.md)**: Dropdown selection with search
- **[Autocomplete](./widgets/autocomplete.md)**: Type-ahead search input
- **[Slider](./widgets/slider.md)**: Range sliders with ticks

### Data Display

- **[DataTable](./widgets/datatable.md)**: Sortable, filterable tables with pagination
- **[Tree](./widgets/tree.md)**: Hierarchical tree with lazy loading
- **[ScrollableList](./widgets/scrollable-list.md)**: Virtual scrolling lists
- **[Progress](./widgets/progress.md)**: Progress bars with animations
- **[Spinner](./widgets/spinner.md)**: Loading indicators (30+ types)

### Content Widgets

- **[RichText](./widgets/rich-text.md)**: Markdown rendering with syntax highlighting
- **[Textarea](./widgets/textarea.md)**: Multi-line text editing with vim-like features
- **[Viewport](./widgets/viewport.md)**: Scrollable areas with virtual rendering

### Feedback Widgets

- **[Toast](./widgets/toast.md)**: Notification toasts with positioning
- **[FormValidator](./widgets/form-validation.md)**: Real-time form validation

### Advanced Features

- **[Animation](./widgets/animation.md)**: Property animations with easing
- **[Menu](./widgets/menu.md)**: Navigation and action selection menus
- **[Overlay](./widgets/overlay.md)**: Floating UI element positioning system
- **[Theme](./themes.md)**: JSON-based theming system
- **[Plugin](./plugin.md)**: Extensible widget architecture

## Usage Patterns

### Basic Widget Creation

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

// Create a button with builder pattern
let button = Button::builder("my_button", "Click Me")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Medium)
    .build();

// Convert to element for layout
let element = button.to_element();
```

### Responsive Design

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

// Widget automatically adapts to container size
let user_data = vec![
    vec!["Alice".to_string(), "25".to_string(), "alice@example.com".to_string()],
    vec!["Bob".to_string(), "30".to_string(), "bob@example.com".to_string()],
];

let table = DataTableBuilder::new("user_table")
    .column(Column::new("name", "Name").width(100).sortable(true))
    .column(Column::new("age", "Age").width(60).sortable(true))
    .column(Column::new("email", "Email").width(200).sortable(true))
    .data(user_data)
    .sortable(true)
    .filterable(true)
    .build();

// Responsive behavior handled automatically via ResponsiveWidget trait
let element = table.to_element();
```

### Widget Composition

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

// Combine multiple widgets in layouts
let input = Input::builder("user_name")
    .placeholder("Enter name")
    .required(true)
    .build();

let button = Button::builder("submit_btn", "Submit")
    .button_type(ButtonType::Success)
    .build();

let form = Element::with_tag("form")
    .class("user-form")
    .child(Element::with_tag("input").id("user_name").build())
    .child(button.to_element())
    .build();
```

## Widget Trait: ResponsiveWidget

All widgets implement the `ResponsiveWidget` trait which provides:

### Methods

#### `to_element(&self) -> Element`

Converts the widget to a DOM-like element for CSS styling and layout computation.

#### `render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String`

Renders the widget with a computed layout from the layout engine.

#### `min_size(&self) -> (u16, u16)`

Returns the widget's preferred minimum size in terminal characters.

#### `max_size(&self) -> (Option<u16>, Option<u16>)`

Returns the widget's preferred maximum size in terminal characters.

#### `can_grow_horizontal(&self) -> bool`

Indicates whether the widget can grow horizontally beyond its minimum size.

#### `can_grow_vertical(&self) -> bool`

Indicates whether the widget can grow vertically beyond its minimum size.

## Common Widget Features

### CSS Integration

All widgets support:

- CSS class names
- Inline CSS styles
- Theme references
- Responsive breakpoints
- Utility classes

### Accessibility

Built-in support for:

- Keyboard navigation
- Tab indexing
- Focus management
- Screen reader compatibility
- ARIA attributes

### State Management

Widgets integrate with the reactive state system:

- State change notifications
- Automatic re-rendering
- Cross-component communication
- Event propagation control

### Event Handling

Comprehensive event system:

- Mouse events (click, hover, etc.)
- Keyboard events (keypress, shortcuts)
- Focus events (focus, blur)
- Custom events
- Event bubbling and capturing

## Performance Considerations

### Virtual Rendering

For large datasets, widgets implement virtual rendering:

- Only visible items are rendered
- Efficient memory usage
- Smooth scrolling
- Dynamic loading

### Dirty Region Tracking

Optimization features:

- Only changed regions are re-rendered
- Minimal DOM updates
- Frame rate optimization
- Battery life preservation

### Widget Caching

Performance optimizations:

- Layout computation caching
- Style application caching
- Event handler optimization
- Memory pooling

## Widget Factory

The widget factory provides a unified interface for creating and managing widgets:

```rust
use reactive_tui::widgets::factory::*;

// Register a custom widget
register_widget("my_widget", |config| {
    // Widget creation logic
});

// Create widget instances
let widget = create_widget("button", ButtonConfig {
    text: "Click me".to_string(),
    button_type: ButtonType::Primary,
});

// Batch operations
let widgets = create_batch(vec![
    ("button1", button_config1),
    ("input1", input_config1),
]);
```

## Widget Development

To create custom widgets:

1. Implement the `ResponsiveWidget` trait
2. Handle state management with reactive system
3. Implement CSS styling support
4. Add event handling
5. Register with the factory system

### Example Custom Widget

```rust
use reactive_tui::prelude::*;

pub struct MyWidget {
    id: String,
    state: Arc<ReactiveState>,
    // widget-specific fields
}

impl ResponsiveWidget for MyWidget {
    fn to_element(&self) -> Element {
        Element::with_tag("my-widget")
            .id(&self.id)
            .build()
    }

    fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
        // Custom rendering logic
        format!("My custom widget at {}x{}", layout.width, layout.height)
    }

    fn min_size(&self) -> (u16, u16) {
        (10, 3) // minimum 10x3 characters
    }
}
```

## Re-exported Items

The widgets module re-exports all widget types and their associated enums, structs, and functions. Key exports include:

### Accordion
- `Accordion`, `AccordionBuilder`, `AccordionConfig`
- `AccordionAnimation`, `AccordionState`
- Helper functions: `compact_accordion`, `faq_accordion`, `settings_accordion`

### Animation
- `Animation`, `AnimationBuilder`, `AnimationManager`
- `AnimatedProperty`, `AnimatedValue`, `EasingFunction`
- Helper functions: `fade_in`, `fade_out`, `slide_in_left`, `bounce`, `pulse`

### Button
- `Button`, `ButtonBuilder`, `ButtonConfig`
- `ButtonType`, `ButtonState`, `ButtonSize`
- Helper functions: `button`, `create_button`

### Input
- `Input`, `InputBuilder`, `InputConfig`
- `InputType`, `InputState`, `ValidationState`
- Helper functions: `input`, `create_input`

### Data Display
- `DataTable`, `DataTableBuilder`, `Column`
- `Tree`, `TreeBuilder`, `TreeNode`
- `ScrollableList`, `ListItem`
- `Progress`, `ProgressBar`, `Spinner`

### Form Controls
- `Checkbox`, `CheckboxGroup`, `RadioGroup`
- `Select`, `Autocomplete`, `Slider`
- `Switch`, `FormValidator`

### Layout
- `Modal`, `Tabs`, `Bar`, `Accordion`
- `Viewport`, `Toast`, `OverlayManager`

### Content
- `RichText`, `Textarea`
- `MarkdownElement`, `SyntaxLanguage`

All widgets support the complete feature set including theming, events, accessibility, and responsive design.