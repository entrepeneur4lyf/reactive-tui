# Spinner Widget

Loading indicator component with 30+ predefined spinner types, animations, and comprehensive styling options for providing visual feedback during async operations.

## Overview

The Spinner widget provides animated loading indicators with support for different spinner types, custom definitions, labels, and positioning. It includes 30+ predefined spinner animations from simple dots to complex emoji sequences.

```rust
use reactive_tui::widgets::*;

let loading_spinner = Spinner::new("loading", SpinnerType::Dots)
    .label("Loading...")
    .label_position(SpinnerLabelPosition::After)
    .spacing(1);

loading_spinner.start()?;
```

## Core Components

### Spinner

Main spinner widget with animation control.

```rust
pub struct Spinner {
    pub id: String,
    pub definition: SpinnerDefinition,
    pub state: SpinnerState,
    pub style: SpinnerStyle,
    pub reactive_state: Option<Arc<ReactiveState>>,
}
```

### SpinnerType

Predefined spinner types with 30+ variants.

```rust
pub enum SpinnerType {
    // Braille patterns
    Dots,              // ⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏
    Dots2,             // ⣾⣽⣻⢿⡿⣟⣯⣷
    
    // Simple animations
    Line,              // -\|/
    Pipe,              // ┤┘┴└├┌┬┐
    SimpleDots,        // ...   
    SimpleDotsScrolling, // .  .. ... ..
    
    // Geometric shapes
    Arc,               // ◜◠◝◞◡◟
    Circle,            // ◡⊙◠
    CircleQuarters,    // ◴◷◶◵
    CircleHalves,      // ◐◓◑◒
    SquareCorners,     // ◰◳◲◱
    Triangle,          // ◢◣◤◥
    
    // Advanced animations
    Star,              // ✶✸✹✺✹✷
    Star2,             // +x*
    Toggle,            // ⊶⊷
    Toggle2,           // ▫▪
    Toggle3,           // □■
    Bounce,            // ⠁⠂⠄⠂
    BoxBounce,         // ▖▘▝▗
    GrowVertical,      // ▁▃▄▅▆▇▆▅▄▃
    GrowHorizontal,    // ▏▎▍▌▋▊▉▊▋▌▍▎
    Balloon,           //  .oO@* 
    Noise,             // ▓▒░
    Arrow,             // ←↖↑↗→↘↓↙
    BouncingBar,       // [====]
    BouncingBall,      // ( ●    )
    
    // Emoji spinners
    Hearts,            // 💛💙💜💚❤️
    Clock,             // 🕛🕐🕑🕒...
    Earth,             // 🌍🌎🌏
    Moon,              // 🌑🌒🌓🌔🌕🌖🌗🌘
    Weather,           // ☀️🌤⛅️🌥☁️🌧🌨⛈
    Smiley,            // 😄😝
    Monkey,            // 🙈🙉🙊
    Runner,            // 🚶🏃
    Christmas,         // 🌲🎄
}
```

### SpinnerDefinition

Custom spinner definition with frames and timing.

```rust
pub struct SpinnerDefinition {
    pub frames: Vec<String>,
    pub interval: u64,           // Milliseconds between frames
    pub name: Option<String>,
}

impl SpinnerDefinition {
    pub fn from_static(
        frames: &'static [&'static str],
        interval: u64,
        name: Option<&'static str>,
    ) -> Self
}
```

### SpinnerState

Animation and visibility state.

```rust
pub struct SpinnerState {
    pub animation_state: SpinnerAnimationState,
    pub current_frame: usize,
    pub visible: bool,
    pub last_update: Option<Instant>,
}

pub enum SpinnerAnimationState {
    Running,
    Paused,
    Stopped,
}
```

### SpinnerStyle

Visual styling and positioning.

```rust
pub struct SpinnerStyle {
    pub label: Option<String>,
    pub label_position: SpinnerLabelPosition,
    pub spacing: u16,
    pub show_spinner: bool,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
}

pub enum SpinnerLabelPosition {
    Before,    // Label before spinner (left/above)
    After,     // Label after spinner (right/below)
    Above,     // Label above spinner
    Below,     // Label below spinner
    None,      // No label
}
```

## Builder Pattern

### SpinnerBuilder

```rust
impl SpinnerBuilder {
    pub fn new<S: Into<String>>(id: S, spinner_type: SpinnerType) -> Self
    pub fn with_custom<S: Into<String>>(id: S, definition: SpinnerDefinition) -> Self
    pub fn label<S: Into<String>>(mut self, label: S) -> Self
    pub fn label_position(mut self, position: SpinnerLabelPosition) -> Self
    pub fn spacing(mut self, spacing: u16) -> Self
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self
    pub fn suffix<S: Into<String>>(mut self, suffix: S) -> Self
    pub fn hide_spinner(mut self) -> Self
    pub fn build(self) -> Spinner
}
```

## Methods

### Animation Control

```rust
impl Spinner {
    // Start animation
    pub fn start(&mut self) -> Result<()>
    
    // Stop animation and reset
    pub fn stop(&mut self) -> Result<()>
    
    // Pause animation
    pub fn pause(&mut self) -> Result<()>
    
    // Resume paused animation
    pub fn resume(&mut self) -> Result<()>
    
    // Update animation frame (call in main loop)
    pub fn update(&mut self) -> Result<bool>
}
```

### State Management

```rust
impl Spinner {
    // Get current frame text
    pub fn current_frame(&self) -> &str
    
    // Check if running
    pub fn is_running(&self) -> bool
    
    // Check if visible
    pub fn is_visible(&self) -> bool
    
    // Set visibility
    pub fn set_visible(&mut self, visible: bool) -> Result<()>
}
```

### Reactive State Integration

```rust
impl Spinner {
    // Connect to reactive state
    pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()>
}
```

## Examples

### Basic Loading Spinner

```rust
use reactive_tui::widgets::*;

let mut loader = Spinner::new("loader", SpinnerType::Dots)
    .label("Loading...")
    .label_position(SpinnerLabelPosition::After)
    .spacing(1);

loader.start()?;

// In main loop
loop {
    loader.update()?;
    // Render and handle other logic
}
```

### Custom Spinner Definition

```rust
let custom_frames = vec!["▰", "▰▰", "▰▰▰", "▰▰▰▰", "▰▰▰▰▰"];
let custom_definition = SpinnerDefinition {
    frames: custom_frames,
    interval: 100,
    name: Some("progress-bar".to_string()),
};

let progress_spinner = Spinner::with_custom("progress", custom_definition)
    .label("Processing...")
    .label_position(SpinnerLabelPosition::Above);
```

### Multiple Spinner Types

```rust
let spinners = vec![
    Spinner::new("dot-spinner", SpinnerType::Dots)
        .label("Dots"),
    Spinner::new("arc-spinner", SpinnerType::Arc)
        .label("Arc"),
    Spinner::new("bounce-spinner", SpinnerType::Bounce)
        .label("Bounce"),
    Spinner::new("heart-spinner", SpinnerType::Hearts)
        .label("Hearts"),
    Spinner::new("clock-spinner", SpinnerType::Clock)
        .label("Clock"),
];

for mut spinner in spinners {
    spinner.start()?;
}
```

### With Prefix and Suffix

```rust
let download_spinner = Spinner::new("download", SpinnerType::BouncingBar)
    .prefix("[")
    .suffix("]")
    .label("Downloading file.zip")
    .label_position(SpinnerLabelPosition::After)
    .spacing(2);
```

### File Operation Spinners

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

let operation_status = Reactive::new("Initializing...".to_string());
let status_clone = operation_status.clone();

let file_spinner = Spinner::new("file-op", SpinnerType::GrowHorizontal)
    .label(&operation_status.get())
    .label_position(SpinnerLabelPosition::After);

// Update status during operation
tokio::spawn(async move {
    let operations = [
        "Reading file...",
        "Processing data...",
        "Writing output...",
        "Finalizing..."
    ];
    
    for op in operations {
        status_clone.set(op.to_string());
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }
});
```

### Conditional Spinners

```rust
let mut api_spinner = Spinner::new("api", SpinnerType::Earth)
    .label("Connecting to API...")
    .label_position(SpinnerLabelPosition::After);

if network_available() {
    api_spinner.start()?;
} else {
    api_spinner.set_visible(false)?;
    // Show error message instead
}
```

### Spinner Showcase

```rust
let spinner_showcase = vec![
    ("Braille Dots", SpinnerType::Dots),
    ("Enhanced Dots", SpinnerType::Dots2),
    ("Rotating Line", SpinnerType::Line),
    ("Box Pipe", SpinnerType::Pipe),
    ("Arc Animation", SpinnerType::Arc),
    ("Circle", SpinnerType::Circle),
    ("Star", SpinnerType::Star),
    ("Bounce", SpinnerType::Bounce),
    ("Growing Bar", SpinnerType::GrowVertical),
    ("Balloon", SpinnerType::Balloon),
    ("Arrow Rotation", SpinnerType::Arrow),
    ("Bouncing Ball", SpinnerType::BouncingBall),
    ("Hearts", SpinnerType::Hearts),
    ("Clock", SpinnerType::Clock),
    ("Earth", SpinnerType::Earth),
    ("Moon Phases", SpinnerType::Moon),
    ("Weather", SpinnerType::Weather),
];

for (name, spinner_type) in spinner_showcase {
    let spinner = Spinner::new(format!("demo-{}", name), spinner_type)
        .label(name)
        .label_position(SpinnerLabelPosition::After)
        .spacing(2);
    
    println!("{}: {}", name, spinner.render_string());
}
```

### Integration with Progress Tracking

```rust
use reactive_tui::{widgets::*, components::*};

struct TaskProgress {
    current: usize,
    total: usize,
    current_task: String,
}

let progress = Reactive::new(TaskProgress {
    current: 0,
    total: 5,
    current_task: "Starting...".to_string(),
});

let task_spinner = Spinner::new("task", SpinnerType::CircleQuarters)
    .label(&format!("{} ({}/{})", 
        progress.get().current_task,
        progress.get().current, 
        progress.get().total))
    .label_position(SpinnerLabelPosition::After);

// Update progress
let progress_clone = progress.clone();
tokio::spawn(async move {
    let tasks = [
        "Initializing...",
        "Loading configuration...",
        "Connecting to services...",
        "Processing data...",
        "Finalizing...",
    ];
    
    for (i, task) in tasks.iter().enumerate() {
        progress_clone.update(|p| {
            p.current = i + 1;
            p.current_task = task.to_string();
        });
        tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
    }
});
```

## Convenience Functions

Pre-configured spinner functions for common use cases:

```rust
// Default loading spinner (Dots with "Loading...")
pub fn loading_spinner<S: Into<String>>(id: S) -> Spinner

// Processing spinner (Arc with "Processing...")
pub fn processing_spinner<S: Into<String>>(id: S) -> Spinner

// Saving spinner (CircleHalves with "Saving...")
pub fn saving_spinner<S: Into<String>>(id: S) -> Spinner

// Generic spinner builder
pub fn spinner<S: Into<String>>(id: S, spinner_type: SpinnerType) -> SpinnerBuilder
```

## Performance Considerations

- **Frame Rate**: Spinners automatically manage frame timing based on their interval settings
- **Resource Usage**: Only active (running) spinners consume CPU cycles during updates
- **Memory Efficiency**: Predefined spinners use static string references for optimal memory usage
- **Batch Updates**: Multiple spinners can be updated in a single loop iteration

## CSS Integration

The spinner generates semantic CSS classes for styling:

```css
.spinner {
    /* Base spinner styles */
}

.spinner-running {
    /* Active animation state */
}

.spinner-paused {
    /* Paused state */
}

.spinner-stopped {
    /* Stopped state */
}

.spinner-hidden {
    /* Hidden state */
}
```

## Accessibility

- **ARIA Support**: Uses `role="status"` and `aria-live="polite"` attributes
- **Screen Reader**: Announces loading state changes
- **Semantic Labels**: Descriptive labels for different loading contexts

## Animation Loop Integration

```rust
use reactive_tui::widgets::*;
use std::time::{Duration, Instant};

let mut spinner = loading_spinner("main");
spinner.start()?;

let mut last_frame = Instant::now();
let target_fps = Duration::from_millis(16); // ~60 FPS

loop {
    let now = Instant::now();
    if now.duration_since(last_frame) >= target_fps {
        // Update spinner animation
        spinner.update()?;
        
        // Render UI
        render_ui(&spinner);
        
        last_frame = now;
    }
    
    // Handle other events
    if should_stop_loading() {
        spinner.stop()?;
        break;
    }
}
```

The Spinner widget provides comprehensive loading indication functionality with extensive customization options and seamless integration with the reactive state system and CSS styling.