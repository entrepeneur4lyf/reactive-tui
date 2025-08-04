---
sidebar_position: 3
---

# Widget API

Comprehensive widget library with 25+ pre-built components for terminal applications.

## Widget Categories

### Layout Widgets

#### Grid
Advanced grid layouts with CSS Grid support.

```rust
use reactive_tui::layout::advanced_grid::*;

let grid = Grid::new()
    .columns(GridColumns::Repeat(3, GridTrackSize::Fr(1.0)))
    .rows(GridRows::Auto)
    .gap(GridGap::uniform(8))
    .item(AdvancedGridItem::new("header")
        .column_span(3)
        .row(1))
    .item(AdvancedGridItem::new("sidebar")
        .column(1)
        .row(2))
    .item(AdvancedGridItem::new("content")
        .column_span(2)
        .row(2));
```

#### Bar
Header, footer, and navigation bars.

```rust
use reactive_tui::widgets::*;

// Header bar
let header = header_bar()
    .title("My Application")
    .items(vec![
        BarItem::text("File"),
        BarItem::text("Edit"),
        BarItem::text("View"),
    ])
    .build();

// Status bar
let status = status_bar()
    .position(BarPosition::Bottom)
    .items(vec![
        BarItem::text("Ready"),
        BarItem::separator(),
        BarItem::text("Line 1:1"),
    ])
    .build();
```

#### Tabs
Tab navigation with multiple orientations.

```rust
let tabs = Tabs::builder("main-tabs")
    .orientation(TabOrientation::Horizontal)
    .position(TabPosition::Top)
    .tab(Tab::new("home", "Home"))
    .tab(Tab::new("settings", "Settings"))
    .tab(Tab::new("about", "About"))
    .active_tab("home")
    .build();
```

### Form Controls

#### Button
Interactive buttons with multiple variants and states.

```rust
use reactive_tui::widgets::button::*;

// WidgetFactory pattern (recommended)
let button = button("save-btn", |config| {
    config.text("Save File")
          .variant(ButtonType::Primary)
          .size(ButtonSize::Large)
          .class("w-full")
          .on_click("save_action")
});

// Builder pattern
let button = Button::builder("save-btn")
    .text("Save File")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .icon("💾", IconPosition::Left)
    .disabled(false)
    .border_style(ButtonBorderStyle::Rounded)
    .build();

// Direct constructor
let simple_button = Button::new("my-btn", "Click Me");
```

**Button API:**

```rust
impl Button {
    pub fn new(id: &str, text: &str) -> Self
    pub fn builder(id: &str) -> ButtonBuilder
    
    // Configuration methods
    pub fn set_text(&mut self, text: &str)
    pub fn set_button_type(&mut self, button_type: ButtonType)
    pub fn set_size(&mut self, size: ButtonSize)
    pub fn set_icon(&mut self, icon: &str, position: IconPosition)
    pub fn set_disabled(&mut self, disabled: bool)
    pub fn set_border_style(&mut self, style: ButtonBorderStyle)
    
    // State queries
    pub fn is_disabled(&self) -> bool
    pub fn get_state(&self) -> ButtonState
    pub fn get_text(&self) -> &str
    pub fn get_button_type(&self) -> ButtonType
}

impl ButtonBuilder {
    pub fn text(self, text: &str) -> Self
    pub fn button_type(self, button_type: ButtonType) -> Self
    pub fn size(self, size: ButtonSize) -> Self
    pub fn icon(self, icon: &str, position: IconPosition) -> Self
    pub fn disabled(self, disabled: bool) -> Self
    pub fn border_style(self, style: ButtonBorderStyle) -> Self
    pub fn on_click(self, action: &str) -> Self
    pub fn on_focus(self, action: &str) -> Self
    pub fn on_blur(self, action: &str) -> Self
    pub fn build(self) -> Button
}
```

**Button Types:**
```rust
pub enum ButtonType {
    Primary,    // Main action button
    Secondary,  // Secondary actions  
    Success,    // Positive actions
    Warning,    // Caution actions
    Danger,     // Destructive actions
    Link,       // Text-like button
}

pub enum ButtonSize {
    Small,
    Medium,
    Large,
}

pub enum ButtonState {
    Normal,
    Hover,
    Pressed,
    Focused,
    Disabled,
}

pub enum IconPosition {
    Left,
    Right,
}

pub enum ButtonBorderStyle {
    None,
    Solid,
    Rounded,
    Double,
}
```

**Factory Function:**
```rust
pub fn button<F>(id: &str, config: F) -> Button
where
    F: FnOnce(ButtonConfig) -> ButtonConfig;

// Usage examples
let primary = button("primary", |c| c.text("Primary").variant("primary"));
let success = button("success", |c| c.text("Success").variant("success").size("large"));
let icon_btn = button("icon", |c| c.text("Save").icon("💾", "left").class("rounded"));
```

#### Input
Text input with validation and autocomplete.

```rust
use reactive_tui::widgets::input::*;

// WidgetFactory pattern (recommended)
let input = input("username", |config| {
    config.placeholder("Enter username")
          .required(true)
          .validation_pattern(r"^[a-zA-Z0-9_]+$")
          .class("form-control")
          .on_change("validate_username")
});

// Builder pattern
let email_input = Input::builder("email")
    .input_type(InputType::Email)
    .placeholder("user@example.com")
    .value("")
    .required(true)
    .max_length(100)
    .validation_state(ValidationState::None)
    .build();

// Direct constructor
let simple_input = Input::new("simple");
```

**Input API:**

```rust
impl Input {
    pub fn new(id: &str) -> Self
    pub fn builder(id: &str) -> InputBuilder
    
    // Configuration methods
    pub fn set_value(&mut self, value: &str)
    pub fn set_placeholder(&mut self, placeholder: &str)
    pub fn set_input_type(&mut self, input_type: InputType)
    pub fn set_required(&mut self, required: bool)
    pub fn set_max_length(&mut self, max_length: Option<usize>)
    pub fn set_validation_state(&mut self, state: ValidationState)
    pub fn set_disabled(&mut self, disabled: bool)
    
    // State queries
    pub fn get_value(&self) -> &str
    pub fn get_placeholder(&self) -> &str
    pub fn is_required(&self) -> bool
    pub fn is_disabled(&self) -> bool
    pub fn get_validation_state(&self) -> ValidationState
    pub fn is_valid(&self) -> bool
    pub fn get_cursor_position(&self) -> usize
    
    // Input operations
    pub fn insert_char(&mut self, ch: char) -> Result<()>
    pub fn delete_char(&mut self) -> Result<()>
    pub fn move_cursor(&mut self, position: usize) -> Result<()>
    pub fn select_all(&mut self)
    pub fn clear(&mut self)
    pub fn validate(&mut self) -> Result<()>
}

impl InputBuilder {
    pub fn value(self, value: &str) -> Self
    pub fn placeholder(self, placeholder: &str) -> Self
    pub fn input_type(self, input_type: InputType) -> Self
    pub fn required(self, required: bool) -> Self
    pub fn max_length(self, max_length: usize) -> Self
    pub fn validation_pattern(self, pattern: &str) -> Self
    pub fn disabled(self, disabled: bool) -> Self
    pub fn on_change(self, action: &str) -> Self
    pub fn on_focus(self, action: &str) -> Self
    pub fn on_blur(self, action: &str) -> Self
    pub fn on_submit(self, action: &str) -> Self
    pub fn build(self) -> Input
}
```

**Input Types:**
```rust
pub enum InputType {
    Text,           // Regular text input
    Password,       // Password input (masked)
    Email,          // Email validation
    Number,         // Numeric input only
    Search,         // Search input with clear button
    Url,            // URL validation
    Tel,            // Telephone number
    Date,           // Date picker
    Time,           // Time picker
    Color,          // Color picker
}

pub enum ValidationState {
    None,           // No validation performed
    Valid,          // Input is valid
    Invalid(String), // Input is invalid with message
    Pending,        // Validation in progress
}
```

**Factory Function:**
```rust
pub fn input<F>(id: &str, config: F) -> Input
where
    F: FnOnce(InputConfig) -> InputConfig;

// Usage examples
let username = input("username", |c| {
    c.placeholder("Username").required(true).max_length(20)
});

let password = input("password", |c| {
    c.input_type("password").placeholder("Password").required(true)
});

let email = input("email", |c| {
    c.input_type("email")
     .placeholder("user@example.com")
     .validation_pattern(r"^[^\s@]+@[^\s@]+\.[^\s@]+$")
});
```

#### Modal
Overlay dialogs with backdrop and positioning.

```rust
use reactive_tui::widgets::modal::*;

// WidgetFactory pattern (recommended)
let modal = modal("confirm-delete", |config| {
    config.title("Confirm Deletion")
          .confirm("Are you sure you want to delete this item?", "Delete", "Cancel")
          .position("center")
          .backdrop(true)
          .closable(true)
          .class("danger-modal")
});

// Builder pattern
let settings_modal = Modal::builder("settings")
    .title("Settings")
    .content(settings_element)
    .position(ModalPosition::Center)
    .size(ModalSize::Medium)
    .backdrop(true)
    .closable(true)
    .close_on_escape(true) 
    .close_on_backdrop_click(true)
    .build();

// Direct constructor
let simple_modal = Modal::new("simple", "Simple Modal");
```

**Modal API:**

```rust
impl Modal {
    pub fn new(id: &str, title: &str) -> Self
    pub fn builder(id: &str) -> ModalBuilder
    
    // Configuration methods
    pub fn set_title(&mut self, title: &str)
    pub fn set_content(&mut self, content: Element)
    pub fn set_position(&mut self, position: ModalPosition)
    pub fn set_size(&mut self, size: ModalSize)
    pub fn set_backdrop(&mut self, backdrop: bool)
    pub fn set_closable(&mut self, closable: bool)
    pub fn set_close_on_escape(&mut self, close_on_escape: bool)
    pub fn set_close_on_backdrop(&mut self, close_on_backdrop: bool)
    
    // State management
    pub fn show(&mut self) -> Result<()>
    pub fn hide(&mut self) -> Result<()>
    pub fn toggle(&mut self) -> Result<()>
    pub fn is_visible(&self) -> bool
    pub fn is_closable(&self) -> bool
    
    // Content management
    pub fn add_button(&mut self, button: ModalButton)
    pub fn remove_button(&mut self, button_id: &str)
    pub fn get_buttons(&self) -> &[ModalButton]
}

impl ModalBuilder {
    pub fn title(self, title: &str) -> Self
    pub fn content(self, content: Element) -> Self
    pub fn position(self, position: ModalPosition) -> Self
    pub fn size(self, size: ModalSize) -> Self
    pub fn backdrop(self, backdrop: bool) -> Self
    pub fn closable(self, closable: bool) -> Self
    pub fn close_on_escape(self, close_on_escape: bool) -> Self
    pub fn close_on_backdrop_click(self, close_on_backdrop: bool) -> Self
    pub fn button(self, button: ModalButton) -> Self
    pub fn on_show(self, action: &str) -> Self
    pub fn on_hide(self, action: &str) -> Self
    pub fn on_confirm(self, action: &str) -> Self
    pub fn on_cancel(self, action: &str) -> Self
    pub fn build(self) -> Modal
}
```

**Modal Types:**
```rust
pub enum ModalPosition {
    Center,         // Center of screen
    Top,           // Top of screen
    Bottom,        // Bottom of screen
    Left,          // Left side
    Right,         // Right side
    TopLeft,       // Top-left corner
    TopRight,      // Top-right corner
    BottomLeft,    // Bottom-left corner
    BottomRight,   // Bottom-right corner
    Custom(u16, u16), // Custom x, y position
}

pub enum ModalSize {
    Small,         // 40x20 characters
    Medium,        // 60x30 characters
    Large,         // 80x40 characters
    ExtraLarge,    // 100x50 characters
    FullScreen,    // Full terminal size
    Custom(u16, u16), // Custom width, height
}

pub struct ModalButton {
    pub id: String,
    pub text: String,
    pub button_type: ButtonType,
    pub action: String,
    pub default: bool,
    pub cancel: bool,
}
```

**Pre-built Modal Types:**
```rust
// Alert modal
pub fn alert_modal(title: &str, message: &str) -> Modal {
    modal("alert", |config| {
        config.title(title)
              .message(message)
              .button(ModalButton::ok("OK"))
              .position("center")
              .closable(true)
    })
}

// Confirm modal
pub fn confirm_modal(title: &str, message: &str, confirm_text: &str, cancel_text: &str) -> Modal {
    modal("confirm", |config| {
        config.title(title)
              .message(message)
              .button(ModalButton::confirm(confirm_text))
              .button(ModalButton::cancel(cancel_text))
              .position("center")
    })
}

// Input modal (prompt)
pub fn prompt_modal(title: &str, message: &str, default_value: &str) -> Modal {
    modal("prompt", |config| {
        config.title(title)
              .message(message)
              .input("prompt_input", default_value)
              .button(ModalButton::ok("OK"))
              .button(ModalButton::cancel("Cancel"))
              .position("center")
    })
}

// Custom content modal
pub fn custom_modal(id: &str, content: Element) -> Modal {
    modal(id, |config| {
        config.content(content)
              .position("center")
              .backdrop(true)
              .closable(true)
    })
}
```

**Factory Function:**
```rust
pub fn modal<F>(id: &str, config: F) -> Modal
where
    F: FnOnce(ModalConfig) -> ModalConfig;

// Usage examples
let info_modal = modal("info", |c| {
    c.title("Information")
     .message("Operation completed successfully!")
     .position("center")
     .closable(true)
});

let delete_confirm = modal("delete-confirm", |c| {
    c.title("Delete Item")
     .confirm("Are you sure you want to delete this item?", "Delete", "Cancel")
     .position("center")
     .backdrop(true)
});
```

### Data Display

#### DataTable
Sortable, filterable tables with pagination.

```rust
let users_data = vec![
    vec!["Alice".to_string(), "25".to_string(), "alice@example.com".to_string()],
    vec!["Bob".to_string(), "30".to_string(), "bob@example.com".to_string()],
    vec!["Charlie".to_string(), "35".to_string(), "charlie@example.com".to_string()],
];

let table = DataTable::builder("users-table")
    .column(Column::new("name", "Name")
        .width(100)
        .sortable(true)
        .alignment(ColumnAlignment::Left))
    .column(Column::new("age", "Age")
        .width(60)
        .sortable(true)
        .alignment(ColumnAlignment::Right))
    .column(Column::new("email", "Email")
        .width(200)
        .sortable(true))
    .data(users_data)
    .sortable(true)
    .filterable(true)
    .pagination(20) // 20 rows per page
    .selectable(true)
    .build();
```

#### Tree
Hierarchical data with lazy loading.

```rust
let file_tree = Tree::builder("file-browser")
    .root(TreeNode::new("root", "/")
        .child(TreeNode::new("home", "home")
            .child(TreeNode::new("user", "user")
                .child(TreeNode::leaf("document.txt", "document.txt"))
                .child(TreeNode::leaf("image.png", "image.png"))))
        .child(TreeNode::new("var", "var")
            .lazy_load(true))) // Load children on demand
    .lazy_loader(Box::new(FileSystemLoader))
    .expandable(true)
    .selectable(true)
    .build();
```

#### Progress
Progress bars with animations and customization.

```rust
let progress = ProgressBar::builder("download")
    .value(0.0)
    .max(100.0)
    .message("Downloading...")
    .show_percentage(true)
    .show_eta(true)
    .animation(ProgressAnimation::Pulse)
    .colors(ProgressColors {
        bar: rgb(0, 122, 255),
        background: rgb(50, 50, 50),
        text: rgb(255, 255, 255),
    })
    .build();
```

#### Spinner
Loading indicators with 30+ built-in types.

```rust
let spinner = spinner("loading", |config| {
    config.spinner_type(SpinnerType::Dots)
          .message("Loading data...")
          .position(SpinnerLabelPosition::Right)
          .class("center")
});

// Predefined spinners
let saving = saving_spinner("Saving...");
let processing = processing_spinner("Processing...");
let loading = loading_spinner();
```

### Content Widgets

#### RichText
Markdown rendering with syntax highlighting.

```rust
let markdown = RichText::builder("readme")
    .content(r#"
# My Project

This is a **bold** statement with some `code`.

```rust
fn main() {
    println!("Hello, world!");
}
```

- Item 1
- Item 2
- Item 3
"#)
    .syntax_language(SyntaxLanguage::Rust)
    .theme("dark")
    .scrollable(true)
    .build();
```

#### Textarea
Multi-line text editing with vim-like features.

```rust
let editor = Textarea::builder("code-editor")
    .content("fn main() {\n    println!(\"Hello, world!\");\n}")
    .language(SyntaxLanguage::Rust)
    .line_numbers(true)
    .vim_mode(true)
    .auto_indent(true)
    .tab_size(4)
    .build();
```

### Advanced Features

#### Animation
Property animations with easing functions.

```rust
let fade_in = Animation::builder()
    .property(AnimatedProperty::Opacity)
    .from(AnimatedValue::Float(0.0))
    .to(AnimatedValue::Float(1.0))
    .duration(500) // milliseconds
    .easing(EasingFunction::EaseInOut)
    .build();

// Pre-built animations
let bounce = bounce().duration(300);
let slide = slide_in_left().duration(400);
let pulse = pulse().loop_mode(LoopMode::Infinite);
```

## Widget Factory Pattern

The WidgetFactory pattern provides zero-boilerplate widget creation with type safety.

### Configuration Pattern

```rust
// Traditional builder pattern
let button = Button::builder("my-button", "Click Me")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .class("btn-large")
    .build();

// WidgetFactory pattern (recommended)
let button = button("my-button", |config| {
    config.text("Click Me")
          .variant("primary")
          .size("large")
          .class("btn-large")
});
```

### Available Factory Functions

```rust
// Form controls
let btn = button("id", |c| c.text("Click").variant("primary"));
let input = input("id", |c| c.placeholder("Enter text").required(true));
let modal = modal("id", |c| c.title("Dialog").confirm("Message", "OK", "Cancel"));

// More widgets available with factory pattern...
```

### Custom Widget Configuration

Implement `WidgetConfig` trait for custom widgets:

```rust
use reactive_tui::widgets::factory::*;

pub struct CustomConfig {
    id: String,
    title: String,
    color: String,
}

impl WidgetConfig for CustomConfig {
    fn id(&self) -> &str { &self.id }
    fn widget_type(&self) -> &str { "custom" }
    
    fn apply_common_config(&mut self, _config: &CommonConfig) -> Result<()> {
        // Apply common styling, classes, etc.
        Ok(())
    }
}

impl CustomConfig {
    pub fn title(mut self, title: &str) -> Self {
        self.title = title.to_string();
        self
    }
    
    pub fn color(mut self, color: &str) -> Self {
        self.color = color.to_string();
        self
    }
}

// Usage
let custom = custom_widget("my-widget", |config| {
    config.title("My Custom Widget")
          .color("blue")
});
```

## ResponsiveWidget Trait

All widgets implement the `ResponsiveWidget` trait for consistent behavior:

```rust
pub trait ResponsiveWidget {
    /// Convert to Element for CSS styling
    fn to_element(&self) -> Element;
    
    /// Render with computed layout
    fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String;
    
    /// Size constraints
    fn min_size(&self) -> (u16, u16) { (1, 1) }
    fn max_size(&self) -> (Option<u16>, Option<u16>) { (None, None) }
    
    /// Growth behavior
    fn can_grow_horizontal(&self) -> bool { true }
    fn can_grow_vertical(&self) -> bool { true }
}
```

## Widget State Management

Widgets integrate with reactive state management:

```rust
use reactive_tui::reactive::*;

#[derive(Debug, Clone)]
struct FormState {
    username: String,
    email: String,
    is_valid: bool,
}

let state = ReactiveState::new(FormState {
    username: String::new(),
    email: String::new(),
    is_valid: false,
});

// Bind widget to state
let username_input = input("username", |config| {
    config.value(&state.get().username)
          .on_change(|value| {
              state.update(|s| s.username = value);
          })
});
```

## Event Handling

Widgets support comprehensive event handling:

```rust
let interactive_button = button("action-btn", |config| {
    config.text("Interactive")
          .on_click("button_clicked")
          .on_focus("button_focused")
          .on_blur("button_blurred")
          .on_key(KeyCombination::new(KeyCode::Enter), "enter_pressed")
});

// Handle events in component
impl Component for MyComponent {
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        match action.name() {
            "button_clicked" => {
                println!("Button was clicked!");
                Ok(ActionResult::Handled)
            }
            "enter_pressed" => {
                println!("Enter key pressed on button!");
                Ok(ActionResult::Handled)
            }
            _ => Ok(ActionResult::Ignored)
        }
    }
}
```

## Widget Styling

All widgets support CSS styling and utility classes:

```rust
let styled_button = button("styled", |config| {
    config.text("Styled Button")
          .class("btn btn-large rounded shadow")
          .style(r#"
              background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
              border: none;
              color: white;
              padding: 1rem 2rem;
              margin: 0.5rem;
          "#)
});
```

## Performance Considerations

### Virtual Rendering

For large datasets, use virtual rendering:

```rust
let large_list = ScrollableList::builder("large-data")
    .items(large_dataset) // 10,000+ items
    .virtual_rendering(true) // Only render visible items
    .item_height(25)
    .viewport_height(400)
    .build();
```

### Widget Caching

Use the widget factory's caching system:

```rust
// Enable caching for expensive widgets
let cached_table = DataTable::builder("expensive-table")
    .data(complex_data)
    .cache_key("complex-data-v1") // Cache based on data version
    .build();
```

## Widget Extensions

Create custom widgets by extending existing ones:

```rust
use reactive_tui::widgets::*;

pub struct IconButton {
    inner: Button,
    icon: String,
}

impl IconButton {
    pub fn new(id: &str, text: &str, icon: &str) -> Self {
        Self {
            inner: Button::builder(id, text).build(),
            icon: icon.to_string(),
        }
    }
}

impl ResponsiveWidget for IconButton {
    fn to_element(&self) -> Element {
        let mut element = self.inner.to_element();
        element.set_content(&format!("{} {}", self.icon, element.content()));
        element
    }
    
    fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
        self.inner.render_with_layout(layout, theme)
    }
}
```