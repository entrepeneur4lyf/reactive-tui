# Progress Widget

Progress indicator component with various styles, animations, and support for determinate and indeterminate progress states.

## Overview

The Progress widget provides visual feedback for ongoing operations with support for linear and circular progress bars, custom styling, and smooth animations.

```rust
use reactive_tui::widgets::{Progress, ProgressBuilder, ProgressStyle, ProgressAnimation};

let download_progress = Progress::builder("download")
    .style(ProgressStyle::Linear)
    .value(45.0)
    .max_value(100.0)
    .label("Downloading...")
    .show_percentage(true)
    .animated(true)
    .build();
```

## ProgressBuilder

```rust
impl ProgressBuilder {
    pub fn new(id: &str) -> Self
    pub fn style(mut self, style: ProgressStyle) -> Self
    pub fn value(mut self, value: f64) -> Self
    pub fn max_value(mut self, max_value: f64) -> Self
    pub fn min_value(mut self, min_value: f64) -> Self
    pub fn label(mut self, label: &str) -> Self
    pub fn show_percentage(mut self, show: bool) -> Self
    pub fn show_value(mut self, show: bool) -> Self
    pub fn indeterminate(mut self, indeterminate: bool) -> Self
    pub fn animated(mut self, animated: bool) -> Self
    pub fn color(mut self, color: ProgressColor) -> Self
    pub fn size(mut self, size: ProgressSize) -> Self
    pub fn striped(mut self, striped: bool) -> Self
    pub fn build(self) -> Progress
}
```

## Progress Styles

```rust
pub enum ProgressStyle {
    Linear,     // Horizontal bar
    Circular,   // Circular progress ring
    Radial,     // Radial progress indicator
    Steps,      // Step-based progress
}
```

## Progress Colors

```rust
pub enum ProgressColor {
    Primary,    // Theme primary color
    Success,    // Green
    Warning,    // Orange/Yellow
    Danger,     // Red
    Info,       // Blue
    Custom(Color), // Custom color
}
```

## Progress Sizes

```rust
pub enum ProgressSize {
    Small,      // Thin progress bar
    Medium,     // Default size
    Large,      // Thick progress bar
    ExtraLarge, // Very thick bar
}
```

## Examples

### Basic Linear Progress

```rust
use reactive_tui::widgets::Progress;

let basic_progress = Progress::builder("basic")
    .style(ProgressStyle::Linear)
    .value(75.0)
    .max_value(100.0)
    .label("Progress")
    .show_percentage(true)
    .color(ProgressColor::Primary)
    .build();
```

### Animated Loading Progress

```rust
let loading_progress = Progress::builder("loading")
    .style(ProgressStyle::Linear)
    .indeterminate(true)
    .animated(true)
    .label("Loading...")
    .color(ProgressColor::Info)
    .striped(true)
    .build();
```

### Circular Progress

```rust
let circular_progress = Progress::builder("circular")
    .style(ProgressStyle::Circular)
    .value(60.0)
    .max_value(100.0)
    .show_percentage(true)
    .size(ProgressSize::Large)
    .color(ProgressColor::Success)
    .animated(true)
    .build();
```

### Step Progress Indicator

```rust
let step_progress = Progress::builder("steps")
    .style(ProgressStyle::Steps)
    .value(2.0)
    .max_value(4.0)
    .steps(vec![
        ProgressStep::new("Setup", "Initial configuration", true),
        ProgressStep::new("Install", "Installing components", true),
        ProgressStep::new("Configure", "Configuring settings", false),
        ProgressStep::new("Complete", "Finishing up", false),
    ])
    .build();
```

### File Upload Progress

```rust
use reactive_tui::{widgets::Progress, reactive::Reactive};

let upload_progress = Reactive::new(0.0);
let upload_status = Reactive::new("Preparing upload...".to_string());

let upload_clone = upload_progress.clone();
let status_clone = upload_status.clone();

let file_upload_progress = Progress::builder("upload")
    .style(ProgressStyle::Linear)
    .value(upload_progress.get())
    .max_value(100.0)
    .label(&upload_status.get())
    .show_percentage(true)
    .show_value(true)
    .color(ProgressColor::Info)
    .size(ProgressSize::Medium)
    .animated(true)
    .on_complete(|| {
        show_success_message("Upload completed!");
        Ok(())
    })
    .build();

// Update progress
tokio::spawn(async move {
    for i in 0..=100 {
        upload_clone.set(i as f64);
        
        let status = match i {
            0..=10 => "Preparing upload...",
            11..=50 => "Uploading file...",
            51..=90 => "Processing...",
            91..=99 => "Finalizing...",
            100 => "Upload complete!",
        };
        
        status_clone.set(status.to_string());
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    }
});
```

### Multiple Progress Bars

```rust
let multi_progress = vec![
    Progress::builder("cpu")
        .label("CPU Usage")
        .value(45.0)
        .max_value(100.0)
        .color(ProgressColor::Info)
        .show_percentage(true)
        .build(),
        
    Progress::builder("memory")
        .label("Memory Usage")
        .value(78.0)
        .max_value(100.0)
        .color(ProgressColor::Warning)
        .show_percentage(true)
        .build(),
        
    Progress::builder("disk")
        .label("Disk Usage")
        .value(92.0)
        .max_value(100.0)
        .color(ProgressColor::Danger)
        .show_percentage(true)
        .build(),
        
    Progress::builder("network")
        .label("Network Activity")
        .indeterminate(true)
        .animated(true)
        .color(ProgressColor::Success)
        .build(),
];
```

### Task Progress with ETA

```rust
use reactive_tui::widgets::{Progress, Element};
use std::time::{Duration, Instant};

let task_start_time = Instant::now();
let task_progress = Reactive::new(0.0);

let progress_with_eta = Element::with_tag("div")
    .class("task-progress")
    .child(
        Progress::builder("task")
            .style(ProgressStyle::Linear)
            .value(task_progress.get())
            .max_value(100.0)
            .label("Processing items...")
            .show_percentage(true)
            .color(ProgressColor::Primary)
            .size(ProgressSize::Medium)
            .build()
            .to_element()
    )
    .child(
        Element::with_tag("div")
            .class("progress-details")
            .text(&format!(
                "ETA: {} | Elapsed: {}",
                calculate_eta(task_progress.get(), task_start_time),
                format_duration(task_start_time.elapsed())
            ))
            .build()
    )
    .build();

fn calculate_eta(progress: f64, start_time: Instant) -> String {
    if progress <= 0.0 {
        return "Unknown".to_string();
    }
    
    let elapsed = start_time.elapsed();
    let rate = progress / elapsed.as_secs_f64();
    let remaining = (100.0 - progress) / rate;
    
    format_duration(Duration::from_secs_f64(remaining))
}
```

### Build Progress Dashboard

```rust
let build_dashboard = Element::with_tag("div")
    .class("build-dashboard")
    .child(
        Element::with_tag("h2")
            .text("Build Progress")
            .build()
    )
    .child(
        Progress::builder("overall")
            .label("Overall Progress")
            .value(35.0)
            .max_value(100.0)
            .size(ProgressSize::Large)
            .color(ProgressColor::Primary)
            .show_percentage(true)
            .build()
            .to_element()
    )
    .child(
        Element::with_tag("div")
            .class("build-steps")
            .child(
                Progress::builder("compile")
                    .label("Compiling (3/8)")
                    .value(37.5)
                    .max_value(100.0)
                    .color(ProgressColor::Info)
                    .animated(true)
                    .build()
                    .to_element()
            )
            .child(
                Progress::builder("test")
                    .label("Running Tests")
                    .value(0.0)
                    .max_value(100.0)
                    .color(ProgressColor::Warning)
                    .indeterminate(false)
                    .build()
                    .to_element()
            )
            .child(
                Progress::builder("deploy")
                    .label("Deployment")
                    .value(0.0)
                    .max_value(100.0)
                    .color(ProgressColor::Success)
                    .indeterminate(false)
                    .build()
                    .to_element()
            )
            .build()
    )
    .build();
```

### Progress with Custom Segments

```rust
let segmented_progress = Progress::builder("segments")
    .style(ProgressStyle::Linear)
    .value(65.0)
    .max_value(100.0)
    .segments(vec![
        ProgressSegment::new(0.0, 25.0, ProgressColor::Success),   // Completed
        ProgressSegment::new(25.0, 50.0, ProgressColor::Success), // Completed
        ProgressSegment::new(50.0, 75.0, ProgressColor::Info),    // In progress
        ProgressSegment::new(75.0, 100.0, ProgressColor::Custom(Color::Gray)), // Pending
    ])
    .show_segments(true)
    .build();
```

### Interactive Progress Control

```rust
use reactive_tui::widgets::{Progress, Button, Element};

let controllable_progress = Reactive::new(50.0);
let progress_clone = controllable_progress.clone();

let progress_control = Element::with_tag("div")
    .class("progress-control")
    .child(
        Progress::builder("controllable")
            .value(controllable_progress.get())
            .max_value(100.0)
            .label("Adjustable Progress")
            .show_percentage(true)
            .color(ProgressColor::Primary)
            .build()
            .to_element()
    )
    .child(
        Element::with_tag("div")
            .class("progress-buttons")
            .child(
                Button::builder("decrease", "-10")
                    .on_click({
                        let progress = progress_clone.clone();
                        move || {
                            let current = progress.get();
                            progress.set((current - 10.0).max(0.0));
                            Ok(())
                        }
                    })
                    .build()
                    .to_element()
            )
            .child(
                Button::builder("increase", "+10")
                    .on_click({
                        let progress = controllable_progress.clone();
                        move || {
                            let current = progress.get();
                            progress.set((current + 10.0).min(100.0));
                            Ok(())
                        }
                    })
                    .build()
                    .to_element()
            )
            .build()
    )
    .build();
```

## State Management

```rust
use reactive_tui::{widgets::Progress, reactive::Reactive};

struct ProgressState {
    current_value: f64,
    max_value: f64,
    label: String,
    color: ProgressColor,
    is_complete: bool,
}

let progress_state = Reactive::new(ProgressState {
    current_value: 0.0,
    max_value: 100.0,
    label: "Starting...".to_string(),
    color: ProgressColor::Info,
    is_complete: false,
});

let stateful_progress = Progress::builder("stateful")
    .value(progress_state.get().current_value)
    .max_value(progress_state.get().max_value)
    .label(&progress_state.get().label)
    .color(progress_state.get().color)
    .show_percentage(true)
    .on_change({
        let state = progress_state.clone();
        move |new_value| {
            let mut current_state = state.get();
            current_state.current_value = new_value;
            
            // Update label based on progress
            current_state.label = match new_value {
                0.0..=25.0 => "Getting started...".to_string(),
                25.1..=50.0 => "Making progress...".to_string(),
                50.1..=75.0 => "Almost there...".to_string(),
                75.1..=99.9 => "Finishing up...".to_string(),
                100.0 => "Complete!".to_string(),
                _ => "Processing...".to_string(),
            };
            
            // Update color based on progress
            current_state.color = match new_value {
                0.0..=33.0 => ProgressColor::Info,
                33.1..=66.0 => ProgressColor::Warning,
                66.1..=100.0 => ProgressColor::Success,
                _ => ProgressColor::Primary,
            };
            
            current_state.is_complete = new_value >= current_state.max_value;
            state.set(current_state);
            
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.progress-container {
    width: 100%;
    margin: 8px 0;
}

.progress-label {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 4px;
    font-size: 14px;
    font-weight: 500;
}

.progress-percentage {
    font-size: 12px;
    color: #6b7280;
}

.progress-bar-container {
    position: relative;
    width: 100%;
    background-color: #e5e7eb;
    border-radius: 6px;
    overflow: hidden;
}

.progress-bar {
    height: 8px;
    background-color: #3b82f6;
    border-radius: 6px;
    transition: width 0.3s ease;
    position: relative;
}

.progress-bar.animated {
    background-image: linear-gradient(
        45deg,
        rgba(255, 255, 255, 0.15) 25%,
        transparent 25%,
        transparent 50%,
        rgba(255, 255, 255, 0.15) 50%,
        rgba(255, 255, 255, 0.15) 75%,
        transparent 75%,
        transparent
    );
    background-size: 20px 20px;
    animation: progress-stripes 1s linear infinite;
}

@keyframes progress-stripes {
    0% {
        background-position: 0 0;
    }
    100% {
        background-position: 20px 0;
    }
}

.progress-bar.indeterminate {
    width: 30% !important;
    animation: progress-indeterminate 2s ease-in-out infinite;
}

@keyframes progress-indeterminate {
    0% {
        transform: translateX(-100%);
    }
    50% {
        transform: translateX(250%);
    }
    100% {
        transform: translateX(-100%);
    }
}

/* Size variants */
.progress-small .progress-bar {
    height: 4px;
}

.progress-medium .progress-bar {
    height: 8px;
}

.progress-large .progress-bar {
    height: 12px;
}

.progress-extra-large .progress-bar {
    height: 16px;
}

/* Color variants */
.progress-success .progress-bar {
    background-color: #10b981;
}

.progress-warning .progress-bar {
    background-color: #f59e0b;
}

.progress-danger .progress-bar {
    background-color: #ef4444;
}

.progress-info .progress-bar {
    background-color: #06b6d4;
}

/* Circular progress */
.progress-circular {
    width: 64px;
    height: 64px;
    position: relative;
}

.progress-circular-svg {
    width: 100%;
    height: 100%;
    transform: rotate(-90deg);
}

.progress-circular-track {
    fill: none;
    stroke: #e5e7eb;
    stroke-width: 4;
}

.progress-circular-fill {
    fill: none;
    stroke: #3b82f6;
    stroke-width: 4;
    stroke-linecap: round;
    transition: stroke-dasharray 0.3s ease;
}

.progress-circular-text {
    position: absolute;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    font-size: 12px;
    font-weight: 600;
    text-align: center;
}

/* Step progress */
.progress-steps {
    display: flex;
    align-items: center;
    gap: 16px;
}

.progress-step {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    position: relative;
}

.progress-step-circle {
    width: 32px;
    height: 32px;
    border-radius: 50%;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: 600;
    font-size: 14px;
    margin-bottom: 8px;
    transition: all 0.2s ease;
}

.progress-step.completed .progress-step-circle {
    background-color: #10b981;
    color: white;
}

.progress-step.active .progress-step-circle {
    background-color: #3b82f6;
    color: white;
}

.progress-step.pending .progress-step-circle {
    background-color: #e5e7eb;
    color: #6b7280;
}

.progress-step-label {
    font-size: 12px;
    font-weight: 500;
    color: #374151;
}

.progress-step-description {
    font-size: 10px;
    color: #6b7280;
    margin-top: 2px;
}

.progress-step-connector {
    position: absolute;
    top: 16px;
    left: 50%;
    width: 100%;
    height: 2px;
    background-color: #e5e7eb;
    z-index: -1;
}

.progress-step.completed .progress-step-connector {
    background-color: #10b981;
}
```

## Integration Examples

### System Monitor Progress

```rust
use reactive_tui::widgets::{Progress, Grid, Panel};

let system_monitor = Panel::builder("system_monitor")
    .title("System Monitor")
    .content(
        Grid::builder("metrics_grid")
            .columns(2)
            .gap(16)
            .add_item(
                Progress::builder("cpu_usage")
                    .label("CPU Usage")
                    .value(get_cpu_usage())
                    .color(ProgressColor::Info)
                    .show_percentage(true)
                    .build()
            )
            .add_item(
                Progress::builder("memory_usage")
                    .label("Memory Usage")
                    .value(get_memory_usage())
                    .color(ProgressColor::Warning)
                    .show_percentage(true)
                    .build()
            )
            .add_item(
                Progress::builder("disk_usage")
                    .label("Disk Usage")
                    .value(get_disk_usage())
                    .color(ProgressColor::Danger)
                    .show_percentage(true)
                    .build()
            )
            .add_item(
                Progress::builder("network_activity")
                    .label("Network Activity")
                    .indeterminate(true)
                    .animated(true)
                    .color(ProgressColor::Success)
                    .build()
            )
            .build()
    )
    .build();
```

The Progress widget provides comprehensive progress indication with multiple styles, animations, and state management capabilities for terminal applications.