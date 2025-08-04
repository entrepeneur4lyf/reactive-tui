# Rendering Module

High-performance terminal rendering engine with virtual DOM, dirty region tracking, and optimized frame buffer management.

## RenderEngine

Core rendering engine that manages the terminal display and optimizes updates.

```rust
use reactive_tui::rendering::RenderEngine;

let mut engine = RenderEngine::new()?;
engine.set_fps_limit(60);
engine.enable_vsync(true);
engine.render_frame(&root_element)?;
```

### Performance Features

- **Double Buffering**: Prevents screen flickering
- **Dirty Region Tracking**: Only updates changed areas
- **Virtual Rendering**: Efficient memory usage for large content
- **Frame Rate Control**: Adaptive FPS based on content changes

## RenderContext

Rendering context providing drawing operations and state management.

```rust
use reactive_tui::rendering::{RenderContext, Color, Style};

fn custom_render(ctx: &mut RenderContext) -> Result<()> {
    ctx.set_foreground(Color::Blue);
    ctx.set_background(Color::White);
    ctx.draw_text(10, 5, "Hello World");
    ctx.draw_rectangle(0, 0, 20, 10, Style::Bold);
    Ok(())
}
```

### Drawing Operations

```rust
impl RenderContext {
    // Text operations
    pub fn draw_text(&mut self, x: u16, y: u16, text: &str)
    pub fn draw_styled_text(&mut self, x: u16, y: u16, text: &str, style: Style)
    
    // Shape operations  
    pub fn draw_rectangle(&mut self, x: u16, y: u16, width: u16, height: u16, style: Style)
    pub fn draw_border(&mut self, rect: Rect, border_style: BorderStyle)
    pub fn draw_line(&mut self, x1: u16, y1: u16, x2: u16, y2: u16, char: char)
    
    // Color management
    pub fn set_foreground(&mut self, color: Color)
    pub fn set_background(&mut self, color: Color)
    pub fn reset_colors(&mut self)
}
```

## FrameBuffer

Double-buffered frame management for flicker-free rendering.

```rust
use reactive_tui::rendering::{FrameBuffer, Cell};

let mut buffer = FrameBuffer::new(80, 24);
buffer.set_cell(10, 5, Cell::new('A', Color::Red, Color::Black));
buffer.clear_region(0, 0, 80, 10);
buffer.swap_buffers();
```

### Cell Structure

```rust
pub struct Cell {
    pub character: char,
    pub foreground: Color,
    pub background: Color,
    pub style: Style,
}

impl Cell {
    pub fn new(ch: char, fg: Color, bg: Color) -> Self
    pub fn with_style(ch: char, fg: Color, bg: Color, style: Style) -> Self
    pub fn clear() -> Self
    pub fn is_empty(&self) -> bool
}
```

## Virtual Rendering

Efficient rendering for large content areas using viewport-based rendering.

```rust
use reactive_tui::rendering::{VirtualRenderer, Viewport};

let viewport = Viewport::new(0, 0, 80, 24);
let mut renderer = VirtualRenderer::new(viewport);

// Only render visible items
renderer.set_total_items(10000);
renderer.set_item_height(1);
renderer.render_visible_items(|start, end| {
    // Render only items from start to end
    for i in start..end {
        // Render item i
    }
});
```

## Style System

### Color Support

```rust
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Gray,
    DarkGray,
    LightRed,
    LightGreen,
    LightYellow,
    LightBlue,
    LightMagenta,
    LightCyan,
    RGB(u8, u8, u8),
    Hex(String),
    Reset,
}
```

### Text Styles

```rust
pub enum Style {
    Normal,
    Bold,
    Dim,
    Italic,
    Underline,
    SlowBlink,
    RapidBlink,
    Reverse,
    Strikethrough,
}
```

### Border Styles

```rust
pub enum BorderStyle {
    None,
    Single,
    Double,
    Thick,
    Rounded,
    Custom {
        top: char,
        bottom: char,
        left: char,
        right: char,
        top_left: char,
        top_right: char,
        bottom_left: char,
        bottom_right: char,
    },
}
```

## Rendering Pipeline

### Frame Rendering Process

1. **Layout Calculation**: Compute element positions and sizes
2. **Style Application**: Apply CSS styles and themes
3. **Dirty Region Detection**: Identify changed areas
4. **Virtual Rendering**: Render only visible content
5. **Buffer Operations**: Update frame buffer
6. **Terminal Output**: Flush changes to terminal

```rust
use reactive_tui::rendering::RenderPipeline;

let pipeline = RenderPipeline::new()
    .add_stage(LayoutStage::new())
    .add_stage(StyleStage::new())
    .add_stage(RenderStage::new())
    .add_stage(OutputStage::new());

pipeline.render(&root_element)?;
```

## Performance Optimizations

### Dirty Region Tracking

```rust
use reactive_tui::rendering::DirtyRegion;

let mut dirty_regions = Vec::new();
dirty_regions.push(DirtyRegion::new(10, 5, 20, 10));

// Only render dirty regions
engine.render_regions(&dirty_regions)?;
```

### Adaptive FPS

```rust
let mut engine = RenderEngine::new()?;
engine.set_adaptive_fps(true);
engine.set_fps_range(10, 60); // Min 10 FPS, Max 60 FPS

// FPS automatically adjusts based on content changes
```

### Memory Management

```rust
// Configure memory usage for large content
engine.set_buffer_size(1024 * 1024); // 1MB buffer
engine.enable_compression(true);
engine.set_gc_threshold(0.8); // Trigger GC at 80% memory usage
```

## Animation Support

### Frame-based Animation

```rust
use reactive_tui::rendering::{AnimationFrame, Interpolator};

let animation = AnimationFrame::new()
    .duration(1000) // 1 second
    .interpolator(Interpolator::EaseInOut)
    .on_frame(|progress| {
        // Update element based on progress (0.0 to 1.0)
        let x = (progress * 100.0) as u16;
        element.set_position(x, 10);
    });

engine.add_animation(animation);
```

### Transition Effects

```rust
use reactive_tui::rendering::Transition;

let fade_in = Transition::fade_in()
    .duration(500)
    .target(&element);

let slide_left = Transition::slide()
    .from_right()
    .duration(300)
    .target(&element);

engine.apply_transition(fade_in)?;
```

## Integration with Layout Engine

The rendering system works closely with the layout engine:

```rust
use reactive_tui::{rendering::RenderEngine, layout::LayoutEngine};

let mut layout_engine = LayoutEngine::new();
let mut render_engine = RenderEngine::new()?;

// Layout calculation
let layout_tree = layout_engine.compute_layout(&root_element)?;

// Rendering with computed layout
render_engine.render_with_layout(&root_element, &layout_tree)?;
```

## Debug and Profiling

### Debug Rendering

```rust
let mut engine = RenderEngine::new()?;
engine.enable_debug_mode(true);
engine.show_fps_counter(true);
engine.show_dirty_regions(true);
engine.show_render_stats(true);
```

### Performance Metrics

```rust
let stats = engine.get_render_stats();
println!("FPS: {}", stats.fps);
println!("Frame time: {}ms", stats.frame_time);
println!("Dirty regions: {}", stats.dirty_regions);
println!("Memory usage: {}MB", stats.memory_usage_mb);
```

## Example Usage

```rust
use reactive_tui::{
    rendering::{RenderEngine, RenderContext, Color, Style},
    layout::LayoutEngine,
    components::Element,
};

// Create render engine
let mut engine = RenderEngine::new()?;
engine.set_fps_limit(60);

// Create content
let content = Element::with_tag("div")
    .style("width", "100%")
    .style("height", "100%")
    .child(
        Element::with_tag("h1")
            .text("Welcome to Reactive TUI")
            .style("color", "blue")
            .build()
    )
    .build();

// Render loop
loop {
    engine.render_frame(&content)?;
    
    if engine.should_exit() {
        break;
    }
    
    std::thread::sleep(std::time::Duration::from_millis(16)); // ~60 FPS
}
```