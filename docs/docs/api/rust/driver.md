# Driver Module

Cross-platform terminal driver system providing hardware abstraction, input handling, and display management for consistent behavior across different terminal environments.

## TerminalDriver

Core driver interface that abstracts terminal-specific operations and provides a unified API.

```rust
use reactive_tui::driver::{TerminalDriver, DriverConfig};

let config = DriverConfig::default()
    .with_color_support(true)
    .with_mouse_support(true)
    .with_alternate_screen(true);

let mut driver = TerminalDriver::new(config)?;
driver.initialize()?;
driver.enable_raw_mode()?;
```

### Driver Configuration

```rust
pub struct DriverConfig {
    pub color_support: ColorSupport,
    pub mouse_support: bool,
    pub keyboard_support: bool,
    pub alternate_screen: bool,
    pub cursor_shape: CursorShape,
    pub scroll_region: bool,
    pub title_support: bool,
    pub resize_handling: bool,
}

pub enum ColorSupport {
    None,
    Basic,      // 16 colors
    Extended,   // 256 colors
    TrueColor,  // 24-bit RGB
    Auto,       // Detect automatically
}
```

## Platform Drivers

### Unix/Linux Driver

```rust
use reactive_tui::driver::unix::UnixDriver;

let driver = UnixDriver::new()?;
driver.setup_signal_handlers()?;
driver.configure_termios()?;
```

### Windows Driver

```rust
use reactive_tui::driver::windows::WindowsDriver;

let driver = WindowsDriver::new()?;
driver.setup_console_mode()?;
driver.enable_virtual_terminal_sequences()?;
```

### WASM Driver

```rust
use reactive_tui::driver::wasm::WasmDriver;

let driver = WasmDriver::new()?;
driver.setup_canvas_context()?;
driver.bind_dom_events()?;
```

## Input Handling

### Key Events

```rust
use reactive_tui::driver::{KeyEvent, KeyCode, KeyModifiers};

fn handle_key_event(event: KeyEvent) {
    match event {
        KeyEvent { 
            code: KeyCode::Char('q'), 
            modifiers: KeyModifiers::CONTROL,
            .. 
        } => {
            // Ctrl+Q pressed
            quit_application();
        },
        KeyEvent {
            code: KeyCode::Enter,
            ..
        } => {
            // Enter pressed
            submit_form();
        },
        _ => {
            // Other keys
        }
    }
}
```

### Mouse Events

```rust
use reactive_tui::driver::{MouseEvent, MouseButton, MouseEventKind};

fn handle_mouse_event(event: MouseEvent) {
    match event.kind {
        MouseEventKind::Down(MouseButton::Left) => {
            println!("Left click at ({}, {})", event.column, event.row);
        },
        MouseEventKind::Drag(MouseButton::Left) => {
            println!("Dragging at ({}, {})", event.column, event.row);
        },
        MouseEventKind::ScrollUp => {
            println!("Scroll up at ({}, {})", event.column, event.row);
        },
        _ => {}
    }
}
```

### Event Polling

```rust
use reactive_tui::driver::{TerminalDriver, Event};
use std::time::Duration;

let mut driver = TerminalDriver::new(DriverConfig::default())?;

loop {
    if let Ok(event) = driver.poll_event(Duration::from_millis(100)) {
        match event {
            Event::Key(key_event) => handle_key_event(key_event),
            Event::Mouse(mouse_event) => handle_mouse_event(mouse_event),
            Event::Resize(width, height) => handle_resize(width, height),
            Event::FocusGained => handle_focus_gained(),
            Event::FocusLost => handle_focus_lost(),
        }
    }
}
```

## Display Management

### Screen Operations

```rust
use reactive_tui::driver::{TerminalDriver, Rect, Color};

let mut driver = TerminalDriver::new(DriverConfig::default())?;

// Screen management
driver.clear_screen()?;
driver.enter_alternate_screen()?;
driver.leave_alternate_screen()?;

// Cursor operations
driver.hide_cursor()?;
driver.show_cursor()?;
driver.move_cursor(10, 5)?;
driver.set_cursor_shape(CursorShape::Block)?;

// Drawing operations
driver.draw_text(0, 0, "Hello World", Color::White, Color::Black)?;
driver.draw_rectangle(Rect::new(5, 5, 20, 10), '#', Color::Blue, Color::Black)?;
driver.flush()?;
```

### Buffer Management

```rust
use reactive_tui::driver::{DisplayBuffer, Cell};

let mut buffer = DisplayBuffer::new(80, 24);

// Direct buffer manipulation
buffer.set_cell(10, 5, Cell::new('A', Color::Red, Color::Black));
buffer.fill_region(Rect::new(0, 0, 80, 1), Cell::new(' ', Color::White, Color::Blue));

// Render buffer to terminal
driver.render_buffer(&buffer)?;
```

## Terminal Capabilities

### Capability Detection

```rust
use reactive_tui::driver::{TerminalCapabilities, ColorSupport};

let caps = TerminalCapabilities::detect()?;

if caps.supports_color() {
    println!("Color support: {:?}", caps.color_support);
}

if caps.supports_mouse() {
    println!("Mouse support available");
}

if caps.supports_unicode() {
    println!("Unicode support available");
}

println!("Terminal size: {}x{}", caps.columns, caps.rows);
```

### Feature Testing

```rust
use reactive_tui::driver::TerminalFeatures;

let features = TerminalFeatures::new(&driver);

// Test specific features
if features.test_true_color()? {
    println!("24-bit color supported");
}

if features.test_mouse_tracking()? {
    println!("Mouse tracking supported");
}

if features.test_bracketed_paste()? {
    println!("Bracketed paste supported");
}
```

## Signal Handling

### Unix Signal Management

```rust
use reactive_tui::driver::signals::{SignalHandler, Signal};

let mut handler = SignalHandler::new();

handler.register(Signal::SIGWINCH, Box::new(|_| {
    // Handle terminal resize
    update_terminal_size();
}));

handler.register(Signal::SIGTERM, Box::new(|_| {
    // Handle graceful shutdown
    cleanup_and_exit();
}));

handler.install()?;
```

### Resize Handling

```rust
use reactive_tui::driver::{ResizeHandler, TerminalSize};

struct AppResizeHandler;

impl ResizeHandler for AppResizeHandler {
    fn handle_resize(&mut self, new_size: TerminalSize) {
        println!("Terminal resized to {}x{}", new_size.columns, new_size.rows);
        
        // Update application layout
        update_layout(new_size);
        redraw_screen();
    }
}

let handler = Box::new(AppResizeHandler);
driver.set_resize_handler(handler);
```

## Performance Optimization

### Batch Operations

```rust
use reactive_tui::driver::{BatchRenderer, DrawCommand};

let mut renderer = BatchRenderer::new(&mut driver);

// Queue multiple operations
renderer.queue_text(0, 0, "Line 1", Color::White, Color::Black);
renderer.queue_text(0, 1, "Line 2", Color::White, Color::Black);
renderer.queue_rectangle(5, 5, 10, 3, '#', Color::Blue, Color::Black);

// Execute all operations at once
renderer.flush()?;
```

### Dirty Region Tracking

```rust
use reactive_tui::driver::{DirtyRegionTracker, Rect};

let mut tracker = DirtyRegionTracker::new(80, 24);

// Mark regions as dirty
tracker.mark_dirty(Rect::new(10, 5, 20, 10));
tracker.mark_dirty(Rect::new(30, 15, 15, 5));

// Get optimized regions for rendering
let regions = tracker.get_dirty_regions();
for region in regions {
    driver.render_region(&buffer, region)?;
}

tracker.clear_dirty_regions();
```

## Error Handling

### Driver Errors

```rust
use reactive_tui::driver::{DriverError, DriverResult};

pub enum DriverError {
    InitializationFailed,
    UnsupportedTerminal,
    IoError(std::io::Error),
    CapabilityMissing(String),
    InvalidInput,
    DisplayError,
}

fn safe_driver_operation() -> DriverResult<()> {
    let driver = TerminalDriver::new(DriverConfig::default())
        .map_err(|_| DriverError::InitializationFailed)?;
    
    if !driver.capabilities().supports_color() {
        return Err(DriverError::CapabilityMissing("color support".to_string()));
    }
    
    driver.clear_screen()
        .map_err(DriverError::IoError)?;
    
    Ok(())
}
```

## Testing Support

### Mock Driver

```rust
use reactive_tui::driver::mock::MockDriver;

let mut mock = MockDriver::new(80, 24);

// Set up expected behavior
mock.expect_clear_screen().times(1);
mock.expect_move_cursor(10, 5).times(1);
mock.expect_draw_text("Hello", Color::White, Color::Black).times(1);

// Use mock in tests
test_function(&mut mock)?;

// Verify expectations
mock.verify();
```

### Event Simulation

```rust
use reactive_tui::driver::testing::{EventSimulator, SimulatedEvent};

let mut simulator = EventSimulator::new();

// Simulate key sequence
simulator.send_key(KeyCode::Char('h'));
simulator.send_key(KeyCode::Char('i'));
simulator.send_key(KeyCode::Enter);

// Simulate mouse events
simulator.send_mouse_click(10, 5, MouseButton::Left);
simulator.send_mouse_drag(10, 5, 15, 8, MouseButton::Left);

// Process simulated events
while let Some(event) = simulator.next_event() {
    handle_event(event);
}
```

## Integration Examples

### Basic Terminal Application

```rust
use reactive_tui::driver::{TerminalDriver, DriverConfig, Event, KeyCode};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = DriverConfig::default()
        .with_mouse_support(true)
        .with_alternate_screen(true);
    
    let mut driver = TerminalDriver::new(config)?;
    driver.initialize()?;
    driver.enable_raw_mode()?;
    driver.enter_alternate_screen()?;
    driver.hide_cursor()?;
    
    // Main event loop
    loop {
        // Draw application
        driver.clear_screen()?;
        driver.draw_text(0, 0, "Press 'q' to quit", Color::White, Color::Black)?;
        driver.flush()?;
        
        // Handle events
        if let Ok(event) = driver.poll_event(Duration::from_millis(100)) {
            match event {
                Event::Key(key) if key.code == KeyCode::Char('q') => break,
                Event::Resize(width, height) => {
                    println!("Resized to {}x{}", width, height);
                },
                _ => {}
            }
        }
    }
    
    // Cleanup
    driver.show_cursor()?;
    driver.leave_alternate_screen()?;
    driver.disable_raw_mode()?;
    
    Ok(())
}
```

### Custom Driver Implementation

```rust
use reactive_tui::driver::{Driver, DriverResult, Event, DisplayBuffer};

struct CustomDriver {
    // driver-specific fields
}

impl Driver for CustomDriver {
    fn initialize(&mut self) -> DriverResult<()> {
        // Initialize terminal
        Ok(())
    }
    
    fn poll_event(&mut self, timeout: Duration) -> DriverResult<Event> {
        // Poll for events
        todo!()
    }
    
    fn render_buffer(&mut self, buffer: &DisplayBuffer) -> DriverResult<()> {
        // Render buffer to terminal
        Ok(())
    }
    
    fn cleanup(&mut self) -> DriverResult<()> {
        // Cleanup resources
        Ok(())
    }
}
```