# Toast Widget

Non-intrusive notification messages that appear temporarily and automatically dismiss, with support for different variants (info, success, warning, error) and overlay positioning.

## Overview

The Toast widget provides temporary notification messages with semantic styling and automatic dismissal. It includes a ToastManager for handling multiple toasts with proper positioning and lifecycle management.

```rust
use reactive_tui::widgets::*;

let success_toast = ToastBuilder::success("Operation completed successfully!")
    .title("Success")
    .duration(Duration::from_secs(3))
    .build();

let mut toast_manager = ToastManager::new(100, 50);
toast_manager.show_toast(success_toast)?;
```

## Core Components

### Toast

Individual toast notification with content and styling.

```rust
pub struct Toast {
    pub id: String,
    pub message: String,
    pub title: Option<String>,
    pub variant: ToastVariant,
    pub created_at: Instant,
    pub duration: Duration,
    pub position: OverlayPosition,
    pub style: OverlayStyle,
}

impl Toast {
    // Check if toast should be dismissed based on duration
    pub fn should_dismiss(&self) -> bool
    
    // Render toast as an Element
    pub fn render(&self) -> Element
    
    // Get estimated dimensions for positioning
    pub fn estimated_dimensions(&self) -> (u16, u16)
}
```

### ToastVariant

Semantic styling variants with predefined CSS classes.

```rust
pub enum ToastVariant {
    /// Informational message (blue theme)
    Info,
    /// Success message (green theme)
    Success,
    /// Warning message (yellow theme)
    Warning,
    /// Error message (red theme)
    Error,
    /// Custom variant with specified CSS classes
    Custom { classes: Vec<String> },
}

impl ToastVariant {
    // Get CSS classes for styling
    pub fn css_classes(&self) -> Vec<String>
}
```

### ToastManager

Manages multiple toast notifications with positioning and lifecycle.

```rust
pub struct ToastManager {
    toasts: HashMap<String, Toast>,
    overlay_manager: OverlayManager,
    next_id: u64,
}

impl ToastManager {
    pub fn new(viewport_width: u16, viewport_height: u16) -> Self
    pub fn update_viewport(&mut self, width: u16, height: u16)
    pub fn show_toast(&mut self, toast: Toast) -> Result<()>
    pub fn dismiss_toast(&mut self, id: &str) -> Result<bool>
    pub fn cleanup_expired(&mut self) -> Vec<String>
    pub fn active_toasts(&self) -> Vec<&Toast>
    pub fn calculate_positions(&self) -> Result<HashMap<String, LayoutRect>>
}
```

## Builder Pattern

### ToastBuilder

```rust
impl ToastBuilder {
    pub fn new(message: impl Into<String>) -> Self
    pub fn title(mut self, title: impl Into<String>) -> Self
    pub fn variant(mut self, variant: ToastVariant) -> Self
    pub fn duration(mut self, duration: Duration) -> Self
    pub fn position(mut self, position: OverlayPosition) -> Self
    pub fn style(mut self, style: OverlayStyle) -> Self
    
    // Convenience constructors
    pub fn info(message: impl Into<String>) -> Self
    pub fn success(message: impl Into<String>) -> Self
    pub fn warning(message: impl Into<String>) -> Self
    pub fn error(message: impl Into<String>) -> Self
    
    pub fn build(self) -> Toast
    pub fn show(self) -> ShowToastMessage  // For message system integration
}
```

## Message System Integration

### Message Types

```rust
pub struct ShowToastMessage {
    pub toast: Toast,
}

pub struct DismissToastMessage {
    pub toast_id: String,
}

impl ToastManager {
    // Handle message events
    pub fn handle_message(&mut self, event: &mut MessageEvent) -> Result<()>
}
```

## Examples

### Basic Toast Notifications

```rust
use reactive_tui::widgets::*;
use std::time::Duration;

// Info toast
let info_toast = ToastBuilder::info("File saved successfully")
    .duration(Duration::from_secs(3))
    .build();

// Success toast with title
let success_toast = ToastBuilder::success("Operation completed!")
    .title("Success")
    .duration(Duration::from_secs(4))
    .build();

// Warning toast
let warning_toast = ToastBuilder::warning("Low disk space")
    .title("Warning")
    .duration(Duration::from_secs(5))
    .build();

// Error toast
let error_toast = ToastBuilder::error("Failed to connect to server")
    .title("Connection Error")
    .duration(Duration::from_secs(10))
    .build();
```

### Toast Manager Usage

```rust
use reactive_tui::widgets::*;

// Create toast manager for 100x50 viewport
let mut toast_manager = ToastManager::new(100, 50);

// Show multiple toasts
toast_manager.show_toast(
    ToastBuilder::info("Starting backup process...")
        .duration(Duration::from_secs(2))
        .build()
)?;

toast_manager.show_toast(
    ToastBuilder::success("Backup completed successfully")
        .title("Backup Complete")
        .duration(Duration::from_secs(5))
        .build()
)?;

toast_manager.show_toast(
    ToastBuilder::warning("Backup location is almost full")
        .title("Storage Warning")
        .duration(Duration::from_secs(8))
        .build()
)?;

// Get active toasts
let active = toast_manager.active_toasts();
println!("Currently showing {} toasts", active.len());

// Clean up expired toasts
let expired = toast_manager.cleanup_expired();
println!("Removed {} expired toasts", expired.len());
```

### Custom Toast Variant

```rust
let custom_toast = ToastBuilder::new("Custom notification message")
    .variant(ToastVariant::Custom { 
        classes: vec![
            "toast-custom".to_string(),
            "bg-purple-500".to_string(),
            "text-white".to_string(),
            "border-purple-700".to_string(),
        ]
    })
    .title("Custom Toast")
    .duration(Duration::from_secs(6))
    .build();
```

### Application Status Notifications

```rust
use reactive_tui::widgets::*;

struct AppNotifications {
    toast_manager: ToastManager,
}

impl AppNotifications {
    fn new(width: u16, height: u16) -> Self {
        Self {
            toast_manager: ToastManager::new(width, height),
        }
    }
    
    fn notify_save_success(&mut self, filename: &str) -> Result<()> {
        let toast = ToastBuilder::success(format!("Saved '{}'", filename))
            .title("File Saved")
            .duration(Duration::from_secs(3))
            .build();
        
        self.toast_manager.show_toast(toast)
    }
    
    fn notify_error(&mut self, error: &str) -> Result<()> {
        let toast = ToastBuilder::error(error)
            .title("Error")
            .duration(Duration::from_secs(8))
            .build();
        
        self.toast_manager.show_toast(toast)
    }
    
    fn notify_progress(&mut self, message: &str) -> Result<()> {
        let toast = ToastBuilder::info(message)
            .duration(Duration::from_secs(2))
            .build();
        
        self.toast_manager.show_toast(toast)
    }
    
    fn notify_warning(&mut self, message: &str) -> Result<()> {
        let toast = ToastBuilder::warning(message)
            .title("Warning")
            .duration(Duration::from_secs(6))
            .build();
        
        self.toast_manager.show_toast(toast)
    }
}

// Usage
let mut notifications = AppNotifications::new(120, 40);

notifications.notify_save_success("document.txt")?;
notifications.notify_warning("Auto-save is disabled")?;
notifications.notify_error("Network connection lost")?;
```

### Real-time Event Notifications

```rust
use reactive_tui::{widgets::*, reactive::Reactive};
use tokio::sync::mpsc;

enum NotificationEvent {
    Info(String),
    Success(String),
    Warning(String),
    Error(String),
}

struct NotificationSystem {
    toast_manager: ToastManager,
    receiver: mpsc::UnboundedReceiver<NotificationEvent>,
}

impl NotificationSystem {
    fn new(width: u16, height: u16) -> (Self, mpsc::UnboundedSender<NotificationEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        
        let system = Self {
            toast_manager: ToastManager::new(width, height),
            receiver,
        };
        
        (system, sender)
    }
    
    async fn run(&mut self) -> Result<()> {
        while let Some(event) = self.receiver.recv().await {
            let toast = match event {
                NotificationEvent::Info(message) => {
                    ToastBuilder::info(message)
                        .duration(Duration::from_secs(3))
                        .build()
                }
                NotificationEvent::Success(message) => {
                    ToastBuilder::success(message)
                        .title("Success")
                        .duration(Duration::from_secs(4))
                        .build()
                }
                NotificationEvent::Warning(message) => {
                    ToastBuilder::warning(message)
                        .title("Warning")
                        .duration(Duration::from_secs(6))
                        .build()
                }
                NotificationEvent::Error(message) => {
                    ToastBuilder::error(message)
                        .title("Error")
                        .duration(Duration::from_secs(10))
                        .build()
                }
            };
            
            self.toast_manager.show_toast(toast)?;
            
            // Clean up expired toasts periodically
            self.toast_manager.cleanup_expired();
        }
        
        Ok(())
    }
}

// Usage
let (mut notification_system, sender) = NotificationSystem::new(100, 50);

// Send notifications from different parts of application
let sender_clone = sender.clone();
tokio::spawn(async move {
    sender_clone.send(NotificationEvent::Info("Application started".to_string())).unwrap();
    tokio::time::sleep(Duration::from_secs(2)).await;
    sender_clone.send(NotificationEvent::Success("Connected to database".to_string())).unwrap();
    tokio::time::sleep(Duration::from_secs(3)).await;
    sender_clone.send(NotificationEvent::Warning("Memory usage high".to_string())).unwrap();
});

// Run notification system
tokio::spawn(async move {
    notification_system.run().await.unwrap();
});
```

### Form Validation Notifications

```rust
use reactive_tui::{widgets::*, components::*};

struct FormWithNotifications {
    form_element: Element,
    notifications: ToastManager,
}

impl FormWithNotifications {
    fn new() -> Self {
        let form = Element::with_tag("form")
            .class("user-form")
            .child(
                Element::with_tag("input")
                    .attr("type", "email")
                    .attr("placeholder", "Enter email")
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .attr("type", "submit")
                    .text("Submit")
                    .build()
            )
            .build();
        
        Self {
            form_element: form,
            notifications: ToastManager::new(80, 30),
        }
    }
    
    fn validate_and_submit(&mut self, email: &str) -> Result<()> {
        if email.is_empty() {
            self.notifications.show_toast(
                ToastBuilder::error("Email is required")
                    .title("Validation Error")
                    .duration(Duration::from_secs(5))
                    .build()
            )?;
            return Ok(());
        }
        
        if !email.contains('@') {
            self.notifications.show_toast(
                ToastBuilder::error("Please enter a valid email address")
                    .title("Invalid Email")
                    .duration(Duration::from_secs(5))
                    .build()
            )?;
            return Ok(());
        }
        
        // Simulate form submission
        self.notifications.show_toast(
            ToastBuilder::info("Submitting form...")
                .duration(Duration::from_secs(2))
                .build()
        )?;
        
        // Simulate success response
        tokio::time::sleep(Duration::from_secs(1)).await;
        
        self.notifications.show_toast(
            ToastBuilder::success("Form submitted successfully!")
                .title("Success")
                .duration(Duration::from_secs(4))
                .build()
        )?;
        
        Ok(())
    }
}
```

### Toast Positioning and Stacking

```rust
use reactive_tui::widgets::{*, overlay::*};

let mut positioned_manager = ToastManager::new(120, 40);

// Top-right notifications
let top_right_toast = ToastBuilder::info("Auto-save enabled")
    .position(OverlayPosition::TopRight { margin_x: 2, margin_y: 1 })
    .duration(Duration::from_secs(3))
    .build();

// Bottom-left notifications
let bottom_left_toast = ToastBuilder::warning("Low battery")
    .position(OverlayPosition::BottomLeft { margin_x: 2, margin_y: 1 })
    .duration(Duration::from_secs(5))
    .build();

// Center notifications for important messages
let center_toast = ToastBuilder::error("Critical system error")
    .title("Critical Error")
    .position(OverlayPosition::Center)
    .duration(Duration::from_secs(10))
    .build();

positioned_manager.show_toast(top_right_toast)?;
positioned_manager.show_toast(bottom_left_toast)?;
positioned_manager.show_toast(center_toast)?;

// Calculate positions for rendering
let positions = positioned_manager.calculate_positions()?;
for (toast_id, rect) in positions {
    println!("Toast {} positioned at {:?}", toast_id, rect);
}
```

### Integration with Task Progress

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

struct TaskWithNotifications {
    task_name: String,
    progress: Reactive<f64>,
    notifications: ToastManager,
}

impl TaskWithNotifications {
    fn new(task_name: String) -> Self {
        Self {
            task_name,
            progress: Reactive::new(0.0),
            notifications: ToastManager::new(100, 40),
        }
    }
    
    async fn run_task(&mut self) -> Result<()> {
        // Start notification
        self.notifications.show_toast(
            ToastBuilder::info(format!("Starting {}", self.task_name))
                .duration(Duration::from_secs(2))
                .build()
        )?;
        
        let steps = 10;
        for i in 1..=steps {
            // Simulate work
            tokio::time::sleep(Duration::from_millis(500)).await;
            
            let progress = (i as f64 / steps as f64) * 100.0;
            self.progress.set(progress);
            
            // Progress notifications every 25%
            if i % (steps / 4) == 0 {
                self.notifications.show_toast(
                    ToastBuilder::info(format!("{} - {:.0}% complete", self.task_name, progress))
                        .duration(Duration::from_secs(2))
                        .build()
                )?;
            }
        }
        
        // Completion notification
        self.notifications.show_toast(
            ToastBuilder::success(format!("{} completed successfully!", self.task_name))
                .title("Task Complete")
                .duration(Duration::from_secs(5))
                .build()
        )?;
        
        Ok(())
    }
    
    fn handle_error(&mut self, error: &str) -> Result<()> {
        self.notifications.show_toast(
            ToastBuilder::error(format!("{} failed: {}", self.task_name, error))
                .title("Task Failed")
                .duration(Duration::from_secs(8))
                .build()
        )
    }
}
```

### Batch Toast Operations

```rust
struct BatchNotificationManager {
    toast_manager: ToastManager,
}

impl BatchNotificationManager {
    fn new(width: u16, height: u16) -> Self {
        Self {
            toast_manager: ToastManager::new(width, height),
        }
    }
    
    fn show_batch_success(&mut self, operations: &[String]) -> Result<()> {
        if operations.len() == 1 {
            self.toast_manager.show_toast(
                ToastBuilder::success(format!("'{}' completed", operations[0]))
                    .duration(Duration::from_secs(3))
                    .build()
            )
        } else {
            self.toast_manager.show_toast(
                ToastBuilder::success(format!("{} operations completed", operations.len()))
                    .title("Batch Complete")
                    .duration(Duration::from_secs(4))
                    .build()
            )
        }
    }
    
    fn show_batch_errors(&mut self, errors: &[(String, String)]) -> Result<()> {
        if errors.len() == 1 {
            self.toast_manager.show_toast(
                ToastBuilder::error(format!("'{}' failed: {}", errors[0].0, errors[0].1))
                    .title("Operation Failed")
                    .duration(Duration::from_secs(8))
                    .build()
            )
        } else {
            self.toast_manager.show_toast(
                ToastBuilder::error(format!("{} operations failed", errors.len()))
                    .title("Batch Errors")
                    .duration(Duration::from_secs(10))
                    .build()
            )
        }
    }
}
```

## CSS Styling

The toast system generates semantic CSS classes:

```css
.toast {
    padding: 8px 12px;
    border-radius: 4px;
    margin: 4px 0;
    position: relative;
}

/* Variant styling */
.toast-info {
    background-color: #3b82f6;
    color: white;
}

.toast-success {
    background-color: #10b981;
    color: white;
}

.toast-warning {
    background-color: #f59e0b;
    color: black;
}

.toast-error {
    background-color: #ef4444;
    color: white;
}

/* Toast components */
.toast-title {
    font-weight: bold;
    margin-bottom: 4px;
}

.toast-message {
    font-size: 14px;
    line-height: 1.4;
}
```

## Performance Considerations

- **Auto-cleanup**: Expired toasts are automatically removed to prevent memory leaks
- **Position Calculation**: Efficient overlay positioning algorithm for multiple toasts
- **Event-driven**: Message system integration for decoupled notification handling
- **Batch Operations**: Efficient handling of multiple toast operations

## Accessibility

- **Semantic HTML**: Uses proper div structure with semantic classes
- **Screen Reader**: Compatible with assistive technologies
- **Color Contrast**: High contrast color schemes for visibility
- **Duration Control**: Configurable display duration for different reading speeds

## Integration Examples

### With Command System

```rust
use reactive_tui::{widgets::*, commands::*};

struct CommandNotifications {
    toast_manager: ToastManager,
}

impl CommandNotifications {
    fn handle_command_result(&mut self, result: CommandResult) -> Result<()> {
        match result {
            CommandResult::Success(message) => {
                self.toast_manager.show_toast(
                    ToastBuilder::success(message)
                        .duration(Duration::from_secs(3))
                        .build()
                )
            }
            CommandResult::Error(error) => {
                self.toast_manager.show_toast(
                    ToastBuilder::error(error)
                        .title("Command Failed")
                        .duration(Duration::from_secs(6))
                        .build()
                )
            }
            CommandResult::Warning(warning) => {
                self.toast_manager.show_toast(
                    ToastBuilder::warning(warning)
                        .duration(Duration::from_secs(4))
                        .build()
                )
            }
        }
    }
}
```

The Toast widget provides comprehensive notification functionality with semantic styling, automatic lifecycle management, and flexible positioning for creating user-friendly notification systems.