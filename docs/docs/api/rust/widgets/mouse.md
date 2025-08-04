# Mouse Widget (Rust)

Advanced mouse interaction widget with gesture recognition, hover detection, and comprehensive event handling.

## Overview

The Mouse widget provides sophisticated mouse interaction capabilities for terminal applications, supporting various mouse events, gesture recognition, hover states, and multi-button interactions. It integrates seamlessly with the existing mouse infrastructure and event system to deliver responsive user interactions.

## Features

- **Click Detection**: Single, double, and triple-click detection with configurable timing
- **Hover States**: Enter, leave, and move tracking with hover callbacks  
- **Drag & Drop**: Full drag and drop support with drag start/end detection
- **Gesture Recognition**: Swipe detection and custom gesture patterns
- **Multi-Button Support**: Left, right, middle button support with separate handlers
- **Mouse Wheel**: Scroll up/down detection with momentum tracking
- **Coordinate Mapping**: Precise pixel coordinate mapping within widget bounds
- **State Management**: Comprehensive mouse state tracking and event propagation

## Basic Usage

### Simple Click Handler

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

let click_area = mouse("click-area", |config| {
    config
        .on_click("handle_click")
        .cursor_style(CursorStyle::Pointer)
});
```

### Advanced Gesture Recognition

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

let gesture_area = mouse("gesture-area", |config| {
    config
        .enable_drag_drop(true)
        .enable_gestures(true)
        .on_drag_start("drag_start")
        .on_drag_end("drag_end")
        .on_double_click("double_click_handler")
        .gesture_threshold(10)
        .drag_threshold(5)
});
```

## Configuration

### MouseConfig Structure

The mouse widget configuration supports comprehensive customization:

```rust
pub struct MouseConfig {
    pub id: String,
    pub classes: Vec<String>,
    pub cursor_style: CursorStyle,
    pub enable_hover: bool,
    pub enable_drag_drop: bool,
    pub enable_gestures: bool,
    pub double_click_threshold: Duration,
    pub triple_click_threshold: Duration,
    pub drag_threshold: u16,
    pub gesture_threshold: u16,
    // Event callbacks
    pub on_click: Option<String>,
    pub on_double_click: Option<String>,
    pub on_triple_click: Option<String>,
    pub on_right_click: Option<String>,
    pub on_middle_click: Option<String>,
    pub on_hover_enter: Option<String>,
    pub on_hover_leave: Option<String>,
    pub on_hover_move: Option<String>,
    pub on_drag_start: Option<String>,
    pub on_drag_move: Option<String>, 
    pub on_drag_end: Option<String>,
    pub on_scroll: Option<String>,
    pub on_gesture: Option<String>,
}
```

### Mouse States

```rust
pub enum MouseInteractionState {
    Normal,
    Hover,
    Pressed(MouseButtonType),
    Dragging(MouseButtonType),
    Released,
}
```

### Cursor Styles

```rust
pub enum CursorStyle {
    Default,
    Pointer,
    Text,
    Crosshair,
    Move,
    NotAllowed,
    Grab,
    Grabbing,
}
```

### Mouse Gestures

```rust
pub enum MouseGesture {
    Click { button: MouseButtonType, count: u8 },
    DoubleClick { button: MouseButtonType },
    TripleClick { button: MouseButtonType },
    Drag { button: MouseButtonType, start: (u16, u16), end: (u16, u16) },
    Swipe { direction: SwipeDirection, distance: f32 },
    Scroll { direction: ScrollDirection, amount: i8 },
}
```

## Advanced Usage

### Multi-Click Detection

```rust
let multi_click_area = mouse("multi-click", |config| {
    config
        .on_click("single_click")
        .on_double_click("double_click") 
        .on_triple_click("triple_click")
        .double_click_threshold(Duration::from_millis(300))
        .triple_click_threshold(Duration::from_millis(500))
});
```

### Drag and Drop with Thresholds

```rust
let drag_area = mouse("drag-zone", |config| {
    config
        .enable_drag_drop(true)
        .drag_threshold(5) // pixels before drag starts
        .on_drag_start("begin_drag")
        .on_drag_move("update_drag")
        .on_drag_end("complete_drag")
        .cursor_style(CursorStyle::Grab)
});
```

### Hover State Management

```rust
let hover_zone = mouse("hover-zone", |config| {
    config
        .enable_hover(true)
        .on_hover_enter("show_tooltip")
        .on_hover_leave("hide_tooltip")
        .on_hover_move("update_tooltip_position")
        .cursor_style(CursorStyle::Pointer)
});
```

### Right-Click Context Menu

```rust
let context_area = mouse("context-area", |config| {
    config
        .on_click("primary_action")
        .on_right_click("show_context_menu")
        .on_middle_click("alternative_action")
});
```

### Scroll Event Handling

```rust
let scrollable_area = mouse("scrollable", |config| {
    config
        .on_scroll("handle_scroll")
        .enable_hover(true)
        .on_hover_enter("show_scroll_indicator")
});
```

## Widget Class Usage

### Direct Widget Creation

```rust
use reactive_tui::widgets::{MouseWidget, MouseConfig, MouseBuilder};

let config = MouseConfig {
    id: "my-mouse".to_string(),
    cursor_style: CursorStyle::Pointer,
    enable_hover: true,
    enable_drag_drop: false,
    on_click: Some("handle_click".to_string()),
    ..Default::default()
};

let widget = MouseWidget::new(config);
```

### Builder Pattern

```rust
let widget = MouseBuilder::new("mouse-widget")
    .cursor_style(CursorStyle::Crosshair)
    .enable_gestures(true)
    .on_click("click_handler")
    .on_drag_start("drag_handler")
    .class("interactive-area")
    .build();
```

### Widget Methods

```rust
// Get current mouse state
let state = widget.get_state();

// Get last mouse position
if let Some(position) = widget.get_last_position() {
    println!("Mouse at: ({}, {})", position.x, position.y);
}

// Check hover status
if widget.is_hovering() {
    if let Some(duration) = widget.get_hover_duration() {
        println!("Hovering for: {:?}", duration);
    }
}

// Handle mouse event
let mouse_event = MouseEvent { /* ... */ };
widget.handle_mouse_event(mouse_event)?;
```

## Event System Integration

### Action Callbacks

Mouse events trigger action callbacks through the action system:

```rust
// In your action handler
fn handle_mouse_action(action: &str, params: Option<Value>) -> ActionResult {
    match action {
        "handle_click" => {
            if let Some(params) = params {
                let button = params["button"].as_str().unwrap();
                let position = params["position"].as_str().unwrap();
                println!("Clicked {} at {}", button, position);
            }
            ActionResult::Handled
        }
        "drag_start" => {
            println!("Drag started");
            ActionResult::Handled
        }
        _ => ActionResult::NotHandled
    }
}
```

### Event Parameters

Different events provide specific parameters:

- **Click events**: `button`, `position`, `count`
- **Hover events**: `position`, `relative_position`
- **Drag events**: `start_position`, `current_position`, `end_position`
- **Scroll events**: `direction`, `amount`

## Coordinate System

### Position Mapping

```rust
pub struct MousePosition {
    pub x: u16,           // Absolute screen coordinate
    pub y: u16,           // Absolute screen coordinate  
    pub relative_x: u16,  // Relative to widget bounds
    pub relative_y: u16,  // Relative to widget bounds
}
```

### Bounds Checking

The widget automatically handles coordinate mapping and bounds checking:

```rust
// Widget bounds are set automatically by the layout system
widget.set_bounds(LayoutRect {
    x: 10, y: 5, width: 100, height: 50
});

// Events outside bounds are ignored
// Relative coordinates are calculated automatically
```

## Factory Functions

### Convenience Functions

```rust
// Simple click area
let click_area = click_area("btn", "button_clicked");

// Drag and drop zone
let drop_zone = drag_drop_area("drop", "drag_start", "drag_end");

// Hover detection area
let hover_area = hover_area("hover", "enter", "leave");
```

### Custom Factory

```rust
fn interactive_panel(id: &str, primary_action: &str) -> Element {
    mouse(id, |config| {
        config
            .on_click(primary_action)
            .on_right_click("show_panel_menu")
            .enable_hover(true)
            .cursor_style(CursorStyle::Pointer)
            .class("interactive-panel")
    })
}
```

## Performance Considerations

### Event Optimization

- Click tracking uses efficient timestamp comparison
- Drag detection uses distance thresholds to avoid unnecessary events
- Hover states are managed with minimal overhead
- Coordinate calculations are optimized for performance

### Memory Management

```rust
// Mouse widget manages its own state efficiently
// Click tracking is reset automatically after timeouts
// Drag state is cleaned up after drag operations
// Event handlers are called with minimal allocations
```

## State Management

### State Transitions

```rust
// Normal → Hover (mouse enters)
// Hover → Pressed (button down)
// Pressed → Dragging (movement exceeds threshold)
// Dragging → Released (button up)
// Released → Normal/Hover (based on position)
```

### State Queries

```rust
match widget.get_state() {
    MouseInteractionState::Normal => println!("Idle"),
    MouseInteractionState::Hover => println!("Hovering"),
    MouseInteractionState::Pressed(button) => println!("Pressed {:?}", button),
    MouseInteractionState::Dragging(button) => println!("Dragging {:?}", button),
    MouseInteractionState::Released => println!("Just released"),
}
```

## Integration Examples

### With Layout System

```rust
use reactive_tui::layout::*;

let interactive_layout = Element::with_tag("div")
    .class("interactive-container")
    .child(
        mouse("main-area", |config| {
            config
                .on_click("area_clicked")
                .enable_hover(true)
                .class("fill-container")
        })
    )
    .build();
```

### With Theme System

```rust
// Mouse widget CSS classes integrate with theme system
.mouse-widget {
    cursor: pointer;
}

.mouse-hover {
    background-color: var(--hover-color);
}

.mouse-pressed {
    background-color: var(--active-color);
}

.mouse-dragging {
    cursor: grabbing;
    opacity: 0.8;
}
```

### With Animation System

```rust
// Combine with animation for visual feedback
let animated_button = mouse("anim-btn", |config| {
    config
        .on_hover_enter("start_hover_animation")
        .on_hover_leave("end_hover_animation") 
        .on_click("trigger_click_animation")
        .cursor_style(CursorStyle::Pointer)
});
```

## Error Handling

### Event Processing

```rust
// Mouse events return Results for error handling
match widget.handle_mouse_event(event) {
    Ok(handled) => {
        if handled {
            println!("Event processed successfully");
        }
    }
    Err(e) => {
        eprintln!("Mouse event error: {}", e);
    }
}
```

### Bounds Validation

```rust
// Widget validates coordinates automatically
// Out-of-bounds events are safely ignored
// Bounds must be set before event processing
```

## Testing

### Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_click_detection() {
        let config = MouseConfig::default();
        let mut widget = MouseWidget::new(config);
        
        // Test click tracking
        let position = (10, 10);
        widget.update_click_tracker(MouseButtonType::Left, position);
        
        assert_eq!(widget.click_tracker.unwrap().count, 1);
    }

    #[test]
    fn test_bounds_checking() {
        let widget = MouseWidget::new(MouseConfig::default());
        
        let bounds = LayoutRect { x: 0, y: 0, width: 100, height: 50 };
        assert!(widget.is_within_bounds(50, 25, &bounds));
        assert!(!widget.is_within_bounds(150, 25, &bounds));
    }

    #[test]
    fn test_distance_calculation() {
        let widget = MouseWidget::new(MouseConfig::default());
        
        let distance = widget.calculate_distance((0, 0), (3, 4));
        assert_eq!(distance, 5.0);
    }
}
```

### Integration Tests  

```rust
#[test]
fn test_mouse_with_layout() {
    let layout = LayoutRect { x: 10, y: 10, width: 100, height: 50 };
    let widget = mouse("test", |c| c.on_click("test_click"));
    
    // Test coordinate mapping
    let element = widget.to_element();
    assert_eq!(element.id, Some("test".to_string()));
}
```

## Related

- [Button Widget](button.md) - For simple click interactions
- [Viewport Widget](viewport.md) - For scrollable areas with mouse support
- [Layout System](../layout.md) - For positioning and bounds
- [Event System](../../events.md) - For action handling
- [Actions](../../events.md) - For callback processing