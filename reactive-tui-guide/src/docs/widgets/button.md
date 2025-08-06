# Button Widget

The Button widget provides interactive buttons with multiple variants, states, and styling options.

## Features

- **Multiple Variants**: Primary, Secondary, Success, Danger, Warning
- **States**: Normal, Hover, Active, Disabled, Loading
- **Icons**: Support for icons with flexible positioning
- **Keyboard Navigation**: Full accessibility support

## Basic Usage

```rust
use reactive_tui::widgets::*;

let button = Button::builder("save-btn", "Save File")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .icon("💾", IconPosition::Left)
    .disabled(false)
    .build();
```

## Variants

- **Primary**: Main action buttons
- **Secondary**: Secondary actions
- **Success**: Positive actions (save, confirm)
- **Danger**: Destructive actions (delete, remove)
- **Warning**: Caution actions

## Configuration Options

- **text**: Button label text
- **variant**: Button style variant
- **size**: Small, Medium, Large
- **disabled**: Enable/disable state
- **icon**: Icon with position (Left, Right, Only)

## Events

- **on_click**: Click event handler
- **on_hover**: Hover state changes
- **on_focus**: Focus state changes
