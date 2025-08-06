# Bar Widget

The Bar widget provides header/footer bars with flexible positioning and content areas.

## Features

- **Flexible Layout**: Left, center, and right content areas
- **Positioning**: Top or bottom positioning
- **Responsive**: Adapts to terminal width
- **Styling**: Full CSS styling support

## Basic Usage

```rust
use reactive_tui::widgets::*;

let header = Bar::builder("header")
    .position(BarPosition::Top)
    .left_content("🚀 Reactive TUI")
    .center_content("Interactive Widget Guide")
    .right_content("60 FPS")
    .height(3)
    .build();
```

## Configuration Options

- **position**: Top or Bottom
- **height**: Bar height in lines
- **border**: Border styling options
- **background**: Background color

## Content Areas

- **Left**: Logo, branding, navigation
- **Center**: Title, status, primary content
- **Right**: Metrics, actions, secondary info
