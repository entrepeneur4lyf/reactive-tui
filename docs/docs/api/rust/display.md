# Display Module

Advanced display management system with viewport handling, scrolling, coordinate transformations, and optimized screen updates for efficient terminal rendering.

## DisplayManager

Central display coordination system managing multiple viewports, screens, and rendering contexts.

```rust
use reactive_tui::display::{DisplayManager, Viewport, Screen};

let mut manager = DisplayManager::new(80, 24)?;

// Create and manage viewports
let main_viewport = Viewport::new(0, 0, 60, 20);
let sidebar_viewport = Viewport::new(60, 0, 20, 20);
let status_viewport = Viewport::new(0, 20, 80, 4);

manager.add_viewport("main", main_viewport);
manager.add_viewport("sidebar", sidebar_viewport);
manager.add_viewport("status", status_viewport);
```

## Viewport System

### Viewport Creation and Management

```rust
use reactive_tui::display::{Viewport, ViewportConfig, ScrollBehavior};

let config = ViewportConfig::builder()
    .position(10, 5)
    .size(50, 20)
    .scrollable(true)
    .scroll_behavior(ScrollBehavior::Auto)
    .borders(true)
    .padding(1)
    .build();

let viewport = Viewport::with_config(config);

// Content management
viewport.set_content_size(100, 50); // Larger than viewport
viewport.scroll_to(0, 10); // Scroll to specific position
viewport.scroll_by(0, 5); // Relative scrolling
```

### Viewport Types

```rust
pub enum ViewportType {
    Fixed,      // Fixed size and position
    Flexible,   // Can resize with parent
    Scrollable, // Can scroll content
    Virtual,    // Virtual rendering for large content
}

pub struct ViewportConfig {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
    pub viewport_type: ViewportType,
    pub scroll_behavior: ScrollBehavior,
    pub borders: bool,
    pub padding: u16,
}
```

## Screen Management

### Multi-Screen Support

```rust
use reactive_tui::display::{Screen, ScreenManager, ScreenTransition};

let mut screen_manager = ScreenManager::new();

// Define screens
let home_screen = Screen::new("home")
    .with_viewport("main", main_viewport)
    .with_viewport("sidebar", sidebar_viewport);

let settings_screen = Screen::new("settings")
    .with_viewport("form", form_viewport)
    .with_viewport("preview", preview_viewport);

screen_manager.register_screen(home_screen);
screen_manager.register_screen(settings_screen);

// Screen transitions
let transition = ScreenTransition::slide_left().duration(300);
screen_manager.switch_to("settings", Some(transition))?;
```

### Screen Stack

```rust
use reactive_tui::display::ScreenStack;

let mut stack = ScreenStack::new();

// Push screens onto stack
stack.push("home")?;
stack.push("dialog")?; // Modal dialog over home
stack.push("confirmation")?; // Confirmation over dialog

// Pop screens
let current = stack.pop()?; // Returns "confirmation"
let previous = stack.current(); // Returns "dialog"
```

## Coordinate Systems

### Coordinate Transformations

```rust
use reactive_tui::display::{CoordinateSystem, Point, Transform};

let viewport_coords = CoordinateSystem::viewport();
let screen_coords = CoordinateSystem::screen();
let world_coords = CoordinateSystem::world();

// Transform between coordinate systems
let screen_point = Point::new(10, 5);
let viewport_point = viewport_coords.from_screen(screen_point, &viewport);
let world_point = world_coords.from_viewport(viewport_point, &viewport);
```

### Clipping and Bounds

```rust
use reactive_tui::display::{ClipRect, BoundsChecker};

let clip_rect = ClipRect::new(5, 5, 30, 15);
let bounds_checker = BoundsChecker::new(clip_rect);

// Check if points/rects are within bounds
if bounds_checker.contains_point(Point::new(10, 8)) {
    // Point is within clipping bounds
    draw_pixel(10, 8);
}

let rect = Rect::new(8, 7, 10, 5);
let clipped_rect = bounds_checker.clip_rect(rect);
```

## Scrolling System

### Scroll Controllers

```rust
use reactive_tui::display::{ScrollController, ScrollDirection, ScrollAmount};

let mut scroll_controller = ScrollController::new(viewport);

// Configure scrolling
scroll_controller.set_smooth_scrolling(true);
scroll_controller.set_scroll_speed(3);
scroll_controller.set_elastic_bounds(true);

// Scroll operations
scroll_controller.scroll(ScrollDirection::Down, ScrollAmount::Line(1));
scroll_controller.scroll(ScrollDirection::Right, ScrollAmount::Page(1));
scroll_controller.scroll_to_position(Point::new(50, 100));

// Smooth scrolling animation
scroll_controller.animate_to(Point::new(0, 200), Duration::from_millis(500));
```

### Scroll Events

```rust
use reactive_tui::display::{ScrollEvent, ScrollListener};

struct MyScrollListener;

impl ScrollListener for MyScrollListener {
    fn on_scroll_start(&mut self, event: &ScrollEvent) {
        println!("Scroll started at ({}, {})", event.x, event.y);
    }
    
    fn on_scroll_update(&mut self, event: &ScrollEvent) {
        println!("Scrolling to ({}, {})", event.x, event.y);
        update_scroll_indicators(event.x, event.y);
    }
    
    fn on_scroll_end(&mut self, event: &ScrollEvent) {
        println!("Scroll ended at ({}, {})", event.x, event.y);
    }
}

scroll_controller.add_listener(Box::new(MyScrollListener));
```

## Virtual Rendering

### Large Content Handling

```rust
use reactive_tui::display::{VirtualViewport, VirtualContent};

// Handle content larger than memory
let virtual_content = VirtualContent::new()
    .total_size(10000, 5000) // Very large content
    .item_renderer(|row, col| {
        // Generate content on-demand
        format!("Item at ({}, {})", col, row)
    });

let mut virtual_viewport = VirtualViewport::new(viewport, virtual_content);

// Only renders visible content
virtual_viewport.render()?;
```

### Lazy Loading

```rust
use reactive_tui::display::{LazyRenderer, ContentLoader};

struct DatabaseContentLoader;

impl ContentLoader for DatabaseContentLoader {
    fn load_range(&self, start_row: usize, end_row: usize) -> Vec<String> {
        // Load data from database for visible range
        database.fetch_rows(start_row, end_row)
    }
}

let loader = Box::new(DatabaseContentLoader);
let lazy_renderer = LazyRenderer::new(viewport, loader);
lazy_renderer.set_buffer_size(100); // Cache 100 items
```

## Display Effects

### Transitions and Animations

```rust
use reactive_tui::display::{DisplayEffect, Transition, Animation};

// Fade transition
let fade_in = Transition::fade_in()
    .duration(500)
    .easing(EasingFunction::EaseInOut);

viewport.apply_transition(fade_in)?;

// Slide animation
let slide_animation = Animation::slide()
    .from(Point::new(-50, 0))
    .to(Point::new(0, 0))
    .duration(300)
    .loop_count(1);

viewport.add_animation(slide_animation);
```

### Visual Effects

```rust
use reactive_tui::display::{VisualEffect, BlurEffect, ShadowEffect};

// Apply blur effect
let blur = BlurEffect::new().radius(2);
viewport.add_effect(Box::new(blur));

// Apply shadow effect
let shadow = ShadowEffect::new()
    .offset(2, 1)
    .color(Color::Black)
    .blur(1);

viewport.add_effect(Box::new(shadow));
```

## Buffer Management

### Display Buffers

```rust
use reactive_tui::display::{DisplayBuffer, BufferPool, BufferConfig};

let config = BufferConfig::builder()
    .double_buffered(true)
    .compression(true)
    .memory_limit(1024 * 1024) // 1MB limit
    .build();

let mut buffer_pool = BufferPool::new(config);

// Get buffer from pool
let mut buffer = buffer_pool.acquire_buffer(80, 24)?;

// Use buffer for rendering
buffer.clear();
buffer.draw_text(0, 0, "Hello World");

// Return buffer to pool
buffer_pool.release_buffer(buffer);
```

### Damage Tracking

```rust
use reactive_tui::display::{DamageTracker, DamageRegion};

let mut damage_tracker = DamageTracker::new(80, 24);

// Mark regions as damaged
damage_tracker.mark_damaged(Rect::new(10, 5, 20, 8));
damage_tracker.mark_damaged(Rect::new(30, 15, 15, 5));

// Get optimized damage regions
let damage_regions = damage_tracker.get_damage_regions();
for region in damage_regions {
    render_region(&buffer, region);
}

damage_tracker.clear_damage();
```

## Performance Optimization

### Render Optimization

```rust
use reactive_tui::display::{RenderOptimizer, OptimizationLevel};

let optimizer = RenderOptimizer::new()
    .level(OptimizationLevel::Aggressive)
    .enable_culling(true)
    .enable_batching(true)
    .enable_compression(true);

// Optimize rendering operations
let optimized_commands = optimizer.optimize(render_commands);
execute_commands(optimized_commands);
```

### Memory Management

```rust
use reactive_tui::display::{MemoryManager, MemoryPolicy};

let memory_manager = MemoryManager::new()
    .policy(MemoryPolicy::Conservative)
    .max_buffers(10)
    .gc_threshold(0.8);

// Automatic memory management
memory_manager.start_gc_thread();
```

## Event Integration

### Display Events

```rust
use reactive_tui::display::{DisplayEvent, DisplayEventHandler};

struct MyDisplayEventHandler;

impl DisplayEventHandler for MyDisplayEventHandler {
    fn handle_viewport_resize(&mut self, viewport_id: &str, new_size: Size) {
        println!("Viewport {} resized to {:?}", viewport_id, new_size);
    }
    
    fn handle_scroll_bounds_reached(&mut self, direction: ScrollDirection) {
        println!("Scroll bounds reached in direction: {:?}", direction);
    }
    
    fn handle_screen_transition_complete(&mut self, from: &str, to: &str) {
        println!("Transitioned from {} to {}", from, to);
    }
}

display_manager.set_event_handler(Box::new(MyDisplayEventHandler));
```

## Integration Examples

### Multi-Pane Application

```rust
use reactive_tui::display::{DisplayManager, Viewport, SplitLayout};

let mut display = DisplayManager::new(120, 40)?;

// Create split layout
let main_split = SplitLayout::horizontal()
    .left_pane(0.7)  // 70% width
    .right_pane(0.3) // 30% width
    .split_at(84);   // Split at column 84

let left_viewport = main_split.left_viewport();
let right_viewport = main_split.right_viewport();

// Further split left pane vertically
let left_split = SplitLayout::vertical()
    .top_pane(0.8)    // 80% height
    .bottom_pane(0.2) // 20% height
    .apply_to(left_viewport);

display.add_viewport("editor", left_split.top);
display.add_viewport("console", left_split.bottom);
display.add_viewport("sidebar", right_viewport);
```

### Scrollable Content Viewer

```rust
use reactive_tui::display::{ScrollableViewport, ContentProvider};

struct FileContentProvider {
    lines: Vec<String>,
}

impl ContentProvider for FileContentProvider {
    fn get_line(&self, line_no: usize) -> Option<&str> {
        self.lines.get(line_no).map(|s| s.as_str())
    }
    
    fn line_count(&self) -> usize {
        self.lines.len()
    }
}

let content = FileContentProvider {
    lines: std::fs::read_to_string("large_file.txt")?
        .lines()
        .map(|s| s.to_string())
        .collect(),
};

let scrollable = ScrollableViewport::new(viewport, Box::new(content));
scrollable.enable_line_numbers(true);
scrollable.enable_search(true);
```