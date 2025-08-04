# Layout Module

Advanced CSS-based layout system with flexbox and grid support for precise positioning and responsive terminal interfaces.

## LayoutEngine

Main layout calculation engine that processes CSS properties and computes final widget positions.

```rust
use reactive_tui::layout::LayoutEngine;

let mut engine = LayoutEngine::new();
engine.set_viewport_size(80, 24);
engine.compute_layout(root_element)?;
```

## Layout Types

### DisplayType

```rust
pub enum DisplayType {
    Block,
    Inline,
    Flex,
    Grid,
    None,
}
```

### FlexDirection

```rust
pub enum FlexDirection {
    Row,
    Column,
    RowReverse,
    ColumnReverse,
}
```

### JustifyContent

```rust
pub enum JustifyContent {
    FlexStart,
    FlexEnd,
    Center,
    SpaceBetween,
    SpaceAround,
    SpaceEvenly,
}
```

### AlignItems

```rust
pub enum AlignItems {
    FlexStart,
    FlexEnd,
    Center,
    Baseline,
    Stretch,
}
```

## Grid Layout

### GridLayout

Advanced CSS Grid implementation for terminal interfaces.

```rust
use reactive_tui::layout::{GridLayout, GridTrack};

let grid = GridLayout::new()
    .columns(vec![
        GridTrack::fr(1.0),
        GridTrack::px(20),
        GridTrack::fr(2.0),
    ])
    .rows(vec![
        GridTrack::auto(),
        GridTrack::fr(1.0),
        GridTrack::px(3),
    ])
    .gap(1, 1);
```

### GridTrack

```rust
pub enum GridTrack {
    Auto,
    Px(u16),
    Fr(f32),
    MinContent,
    MaxContent,
    FitContent(u16),
}
```

## Layout Calculation

### LayoutRect

```rust
pub struct LayoutRect {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}
```

### ComputedStyles

```rust
pub struct ComputedStyles {
    pub display: DisplayType,
    pub position: PositionType,
    pub width: SizeValue,
    pub height: SizeValue,
    pub margin: Spacing,
    pub padding: Spacing,
    // ... more properties
}
```

## Example Usage

```rust
use reactive_tui::{
    layout::{LayoutEngine, DisplayType, FlexDirection},
    widgets::div,
};

// Create flex container
let container = div()
    .style("display", "flex")
    .style("flex-direction", "column")
    .style("justify-content", "space-between")
    .style("height", "100%");

// Add items
container.child(header)
    .child(main_content)
    .child(footer);

// Compute layout
let mut engine = LayoutEngine::new();
engine.compute_layout(&container)?;
```