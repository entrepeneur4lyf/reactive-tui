# Overlay Widget

Overlay positioning system for floating UI elements such as toasts, modals, dropdowns, and tooltips that appear above the main content.

## Overview

The Overlay system provides utilities for positioning and rendering UI elements that float above the main content. It handles positioning calculations, collision detection, and stacking management for multiple overlays.

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, OverlayStyle};

let overlay_manager = OverlayManager::new(terminal_width, terminal_height);

let toast_position = overlay_manager.calculate_position(
    30, // width
    5,  // height
    OverlayPosition::TopRight { padding: 2 },
    &OverlayStyle::default()
)?;
```

## Core Types

### OverlayPosition

```rust
pub enum OverlayPosition {
    /// Top left corner with padding
    TopLeft { padding: u16 },
    /// Top right corner with padding  
    TopRight { padding: u16 },
    /// Bottom left corner with padding
    BottomLeft { padding: u16 },
    /// Bottom right corner with padding
    BottomRight { padding: u16 },
    /// Center of the screen
    Center,
    /// Custom absolute position
    Absolute { x: u16, y: u16 },
}
```

### OverlayStyle

```rust
pub struct OverlayStyle {
    /// Whether to show a border around the overlay
    pub border: bool,
    /// Border color (CSS color string)
    pub border_color: Option<String>,
    /// Background color (CSS color string) 
    pub background_color: Option<String>,
    /// Shadow effect
    pub shadow: bool,
    /// Maximum width of the overlay
    pub max_width: Option<u16>,
    /// Maximum height of the overlay
    pub max_height: Option<u16>,
}
```

### OverlayManager

```rust
pub struct OverlayManager {
    viewport_width: u16,
    viewport_height: u16,
}

impl OverlayManager {
    pub fn new(viewport_width: u16, viewport_height: u16) -> Self
    pub fn update_viewport(&mut self, width: u16, height: u16)
    pub fn calculate_position(
        &self,
        content_width: u16,
        content_height: u16, 
        position: OverlayPosition,
        style: &OverlayStyle,
    ) -> Result<LayoutRect>
    pub fn stack_overlays(
        &self,
        overlays: &[(u16, u16, OverlayPosition, &OverlayStyle)],
    ) -> Result<Vec<LayoutRect>>
    pub fn viewport_size(&self) -> (u16, u16)
}
```

## Examples

### Toast Notifications

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, OverlayStyle, Toast};

struct ToastSystem {
    overlay_manager: OverlayManager,
    active_toasts: Vec<Toast>,
}

impl ToastSystem {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            overlay_manager: OverlayManager::new(terminal_width, terminal_height),
            active_toasts: Vec::new(),
        }
    }

    pub fn show_toast(&mut self, message: &str, variant: ToastVariant) -> Result<()> {
        let toast = Toast::builder("toast")
            .message(message)
            .variant(variant)
            .duration(5000) // 5 seconds
            .build();

        // Calculate position for the new toast
        let toast_style = OverlayStyle {
            border: true,
            border_color: Some("#ccc".to_string()),
            background_color: Some("#fff".to_string()),
            shadow: true,
            max_width: Some(60),
            max_height: Some(10),
        };

        let position = self.overlay_manager.calculate_position(
            50, // width
            3,  // height
            OverlayPosition::TopRight { padding: 2 },
            &toast_style,
        )?;

        toast.set_position(position);
        self.active_toasts.push(toast);
        
        Ok(())
    }

    pub fn show_multiple_toasts(&mut self) -> Result<()> {
        let toasts = vec![
            ("File saved successfully", ToastVariant::Success),
            ("Warning: Unsaved changes", ToastVariant::Warning), 
            ("Error: Connection failed", ToastVariant::Error),
        ];

        // Calculate positions for multiple toasts with stacking
        let overlay_specs: Vec<_> = toasts.iter()
            .map(|(_, _)| (50u16, 3u16, OverlayPosition::TopRight { padding: 2 }, &OverlayStyle::default()))
            .collect();

        let positions = self.overlay_manager.stack_overlays(&overlay_specs)?;

        for ((message, variant), position) in toasts.iter().zip(positions.iter()) {
            let toast = Toast::builder(&format!("toast_{}", self.active_toasts.len()))
                .message(message)
                .variant(*variant)
                .duration(5000)
                .build();

            toast.set_position(*position);
            self.active_toasts.push(toast);
        }

        Ok(())
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        // Update viewport size on terminal resize
        self.overlay_manager.update_viewport(terminal_width, terminal_height);
        
        // Remove expired toasts
        self.active_toasts.retain(|toast| !toast.is_expired());
        
        // Recalculate positions for remaining toasts
        self.recalculate_toast_positions();
    }

    fn recalculate_toast_positions(&mut self) {
        if self.active_toasts.is_empty() {
            return;
        }

        let overlay_specs: Vec<_> = self.active_toasts.iter()
            .map(|_| (50u16, 3u16, OverlayPosition::TopRight { padding: 2 }, &OverlayStyle::default()))
            .collect();

        if let Ok(positions) = self.overlay_manager.stack_overlays(&overlay_specs) {
            for (toast, position) in self.active_toasts.iter_mut().zip(positions.iter()) {
                toast.set_position(*position);
            }
        }
    }
}
```

### Modal Dialog Positioning

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, Modal};

struct ModalSystem {
    overlay_manager: OverlayManager,
    modal_stack: Vec<Modal>,
}

impl ModalSystem {
    pub fn show_centered_modal(&mut self, title: &str, content: &str) -> Result<()> {
        let modal = Modal::builder("centered-modal")
            .title(title)
            .content(content)
            .size(ModalSize::Medium)
            .build();

        let modal_style = OverlayStyle {
            border: true,
            border_color: Some("#333".to_string()),
            background_color: Some("#fff".to_string()),
            shadow: true,
            max_width: Some(80),
            max_height: Some(25),
        };

        let position = self.overlay_manager.calculate_position(
            60, // width
            20, // height
            OverlayPosition::Center,
            &modal_style,
        )?;

        modal.set_position(position);
        self.modal_stack.push(modal);
        
        Ok(())
    }

    pub fn show_confirmation_dialog(&mut self, message: &str) -> Result<()> {
        let modal = Modal::builder("confirm-dialog")
            .title("Confirmation")
            .content(message)
            .size(ModalSize::Small)
            .modal_type(ModalType::Confirmation)
            .add_button(ModalButton::new("yes", "Yes"))
            .add_button(ModalButton::new("no", "No"))
            .build();

        let position = self.overlay_manager.calculate_position(
            40, // width
            10, // height
            OverlayPosition::Center,
            &OverlayStyle::default(),
        )?;

        modal.set_position(position);
        self.modal_stack.push(modal);
        
        Ok(())
    }

    pub fn show_stacked_modals(&mut self) -> Result<()> {
        // Show multiple modals with automatic positioning
        let modals_data = vec![
            ("Settings", "Configure application settings", 50, 15),
            ("Preferences", "User preferences", 45, 12),
            ("About", "Application information", 40, 8),
        ];

        for (i, (title, content, width, height)) in modals_data.iter().enumerate() {
            let modal = Modal::builder(&format!("modal_{}", i))
                .title(title)
                .content(content)
                .build();

            // Offset each modal slightly for stacked effect
            let offset = i as u16 * 3;
            let position = self.overlay_manager.calculate_position(
                *width,
                *height,
                OverlayPosition::Absolute { x: 20 + offset, y: 10 + offset },
                &OverlayStyle::default(),
            )?;

            modal.set_position(position);
            self.modal_stack.push(modal);
        }

        Ok(())
    }
}
```

### Dropdown Menu Positioning

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, Menu, Button};

struct DropdownSystem {
    overlay_manager: OverlayManager,
}

impl DropdownSystem {
    pub fn show_dropdown(&mut self, button_rect: LayoutRect, menu_items: Vec<MenuItem>) -> Result<LayoutRect> {
        let menu_height = menu_items.len() as u16 * 2; // Approximate height
        let menu_width = 30;

        // Try to position below the button first
        let mut position = OverlayPosition::Absolute {
            x: button_rect.x,
            y: button_rect.y + button_rect.height,
        };

        let mut menu_rect = self.overlay_manager.calculate_position(
            menu_width,
            menu_height,
            position,
            &OverlayStyle::default(),
        )?;

        // If dropdown goes off screen, position above button instead
        let (viewport_width, viewport_height) = self.overlay_manager.viewport_size();
        if menu_rect.y + menu_rect.height > viewport_height {
            position = OverlayPosition::Absolute {
                x: button_rect.x,
                y: button_rect.y.saturating_sub(menu_height),
            };
            
            menu_rect = self.overlay_manager.calculate_position(
                menu_width,
                menu_height,
                position,
                &OverlayStyle::default(),
            )?;
        }

        // If dropdown goes off screen horizontally, adjust
        if menu_rect.x + menu_rect.width > viewport_width {
            let new_x = viewport_width.saturating_sub(menu_rect.width);
            menu_rect.x = new_x;
        }

        Ok(menu_rect)
    }

    pub fn show_context_menu(&mut self, click_x: u16, click_y: u16, menu_items: Vec<MenuItem>) -> Result<LayoutRect> {
        let menu_height = menu_items.len() as u16 * 2;
        let menu_width = 25;

        // Position at click location
        let position = OverlayPosition::Absolute { x: click_x, y: click_y };
        
        let mut menu_rect = self.overlay_manager.calculate_position(
            menu_width,
            menu_height,
            position,
            &OverlayStyle {
                border: true,
                border_color: Some("#666".to_string()),
                background_color: Some("#f0f0f0".to_string()),
                shadow: true,
                ..Default::default()
            },
        )?;

        // Ensure menu stays within viewport bounds
        let (viewport_width, viewport_height) = self.overlay_manager.viewport_size();
        
        if menu_rect.x + menu_rect.width > viewport_width {
            menu_rect.x = viewport_width.saturating_sub(menu_rect.width);
        }
        
        if menu_rect.y + menu_rect.height > viewport_height {
            menu_rect.y = viewport_height.saturating_sub(menu_rect.height);
        }

        Ok(menu_rect)
    }
}
```

### Tooltip System

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, OverlayStyle};

struct TooltipSystem {
    overlay_manager: OverlayManager,
    active_tooltip: Option<Tooltip>,
}

struct Tooltip {
    id: String,
    text: String,
    position: LayoutRect,
    target_rect: LayoutRect,
}

impl TooltipSystem {
    pub fn show_tooltip(&mut self, target_rect: LayoutRect, text: &str) -> Result<()> {
        let tooltip_width = (text.len() as u16).min(50);
        let tooltip_height = 3;

        // Try different positions around the target element
        let positions_to_try = vec![
            // Above target
            OverlayPosition::Absolute {
                x: target_rect.x + target_rect.width / 2 - tooltip_width / 2,
                y: target_rect.y.saturating_sub(tooltip_height + 1),
            },
            // Below target
            OverlayPosition::Absolute {
                x: target_rect.x + target_rect.width / 2 - tooltip_width / 2,
                y: target_rect.y + target_rect.height + 1,
            },
            // Right of target
            OverlayPosition::Absolute {
                x: target_rect.x + target_rect.width + 1,
                y: target_rect.y + target_rect.height / 2 - tooltip_height / 2,
            },
            // Left of target
            OverlayPosition::Absolute {
                x: target_rect.x.saturating_sub(tooltip_width + 1),
                y: target_rect.y + target_rect.height / 2 - tooltip_height / 2,
            },
        ];

        let tooltip_style = OverlayStyle {
            border: true,
            border_color: Some("#333".to_string()),
            background_color: Some("#fffbf0".to_string()),
            shadow: false,
            max_width: Some(50),
            max_height: Some(5),
        };

        // Find the first position that fits within viewport
        for position in positions_to_try {
            if let Ok(tooltip_rect) = self.overlay_manager.calculate_position(
                tooltip_width,
                tooltip_height,
                position,
                &tooltip_style,
            ) {
                let (viewport_width, viewport_height) = self.overlay_manager.viewport_size();
                
                // Check if tooltip fits within viewport
                if tooltip_rect.x + tooltip_rect.width <= viewport_width &&
                   tooltip_rect.y + tooltip_rect.height <= viewport_height {
                    
                    let tooltip = Tooltip {
                        id: "tooltip".to_string(),
                        text: text.to_string(),
                        position: tooltip_rect,
                        target_rect,
                    };
                    
                    self.active_tooltip = Some(tooltip);
                    return Ok(());
                }
            }
        }

        // If no position works, show at center as fallback
        let center_position = self.overlay_manager.calculate_position(
            tooltip_width,
            tooltip_height,
            OverlayPosition::Center,
            &tooltip_style,
        )?;

        let tooltip = Tooltip {
            id: "tooltip".to_string(),
            text: text.to_string(),
            position: center_position,
            target_rect,
        };

        self.active_tooltip = Some(tooltip);
        Ok(())
    }

    pub fn hide_tooltip(&mut self) {
        self.active_tooltip = None;
    }

    pub fn update_tooltip_position(&mut self, new_target_rect: LayoutRect) -> Result<()> {
        if let Some(tooltip) = &self.active_tooltip {
            let text = tooltip.text.clone();
            self.show_tooltip(new_target_rect, &text)?;
        }
        Ok(())
    }
}
```

### Loading Overlay

```rust
use reactive_tui::widgets::{OverlayManager, OverlayPosition, Spinner};

struct LoadingOverlay {
    overlay_manager: OverlayManager,
    spinner: Spinner,
    message: String,
    is_visible: bool,
}

impl LoadingOverlay {
    pub fn new(terminal_width: u16, terminal_height: u16) -> Self {
        Self {
            overlay_manager: OverlayManager::new(terminal_width, terminal_height),
            spinner: Spinner::builder("loading")
                .spinner_type(SpinnerType::Dots)
                .label("Loading...")
                .build(),
            message: "Loading...".to_string(),
            is_visible: false,
        }
    }

    pub fn show(&mut self, message: &str) -> Result<()> {
        self.message = message.to_string();
        self.is_visible = true;

        let overlay_width = (message.len() as u16 + 10).min(60);
        let overlay_height = 5;

        let position = self.overlay_manager.calculate_position(
            overlay_width,
            overlay_height,
            OverlayPosition::Center,
            &OverlayStyle {
                border: true,
                border_color: Some("#333".to_string()),
                background_color: Some("rgba(0, 0, 0, 0.8)".to_string()),
                shadow: true,
                max_width: Some(60),
                max_height: Some(10),
            },
        )?;

        self.spinner.set_position(position);
        self.spinner.set_label(&self.message);
        
        Ok(())
    }

    pub fn hide(&mut self) {
        self.is_visible = false;
    }

    pub fn update(&mut self) {
        if self.is_visible {
            self.spinner.update();
        }
    }

    pub fn render(&self) -> String {
        if self.is_visible {
            self.spinner.render()
        } else {
            String::new()
        }
    }
}
```

### Multi-Screen Overlay Support

```rust
struct MultiScreenOverlayManager {
    screens: Vec<OverlayManager>,
    current_screen: usize,
}

impl MultiScreenOverlayManager {
    pub fn new(screen_configs: Vec<(u16, u16)>) -> Self {
        let screens = screen_configs
            .into_iter()
            .map(|(width, height)| OverlayManager::new(width, height))
            .collect();

        Self {
            screens,
            current_screen: 0,
        }
    }

    pub fn switch_screen(&mut self, screen_index: usize) -> Result<()> {
        if screen_index >= self.screens.len() {
            return Err(TuiError::InvalidScreenIndex);
        }
        self.current_screen = screen_index;
        Ok(())
    }

    pub fn show_overlay_on_screen(
        &mut self,
        screen_index: usize,
        width: u16,
        height: u16,
        position: OverlayPosition,
        style: &OverlayStyle,
    ) -> Result<LayoutRect> {
        if screen_index >= self.screens.len() {
            return Err(TuiError::InvalidScreenIndex);
        }

        self.screens[screen_index].calculate_position(width, height, position, style)
    }

    pub fn current_screen(&mut self) -> &mut OverlayManager {
        &mut self.screens[self.current_screen]
    }
}
```

## CSS Styling

```css
.overlay-container {
    position: absolute;
    z-index: 1000;
    pointer-events: auto;
}

.overlay-backdrop {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
    z-index: 999;
}

.overlay-content {
    background-color: white;
    border-radius: 4px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.15);
    max-width: 90vw;
    max-height: 90vh;
    overflow: auto;
}

.overlay-bordered {
    border: 1px solid #ccc;
}

.overlay-shadow {
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.15);
}

/* Position-specific styles */
.overlay-top-left {
    top: 16px;
    left: 16px;
}

.overlay-top-right {
    top: 16px;
    right: 16px;
}

.overlay-bottom-left {
    bottom: 16px;
    left: 16px;
}

.overlay-bottom-right {
    bottom: 16px;
    right: 16px;
}

.overlay-center {
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
}

/* Animation styles */
.overlay-fade-in {
    animation: fadeIn 0.2s ease-out;
}

.overlay-slide-down {
    animation: slideDown 0.3s ease-out;
}

@keyframes fadeIn {
    from {
        opacity: 0;
    }
    to {
        opacity: 1;
    }
}

@keyframes slideDown {
    from {
        opacity: 0;
        transform: translateY(-10px);
    }
    to {
        opacity: 1;
        transform: translateY(0);
    }
}
```

## Integration Examples

### Complete Toast System

```rust
use reactive_tui::widgets::{OverlayManager, Toast, ToastVariant, Button, Element};

struct Application {
    overlay_manager: OverlayManager,
    toast_system: ToastSystem,
}

impl Application {
    pub fn show_success_notification(&mut self, message: &str) {
        let _ = self.toast_system.show_toast(message, ToastVariant::Success);
    }

    pub fn show_error_notification(&mut self, message: &str) {
        let _ = self.toast_system.show_toast(message, ToastVariant::Error);
    }

    pub fn create_ui(&self) -> Element {
        Element::with_tag("div")
            .class("app-container")
            .child(
                Button::builder("success-btn", "Show Success")
                    .on_click(|| {
                        self.show_success_notification("Operation completed successfully!");
                        Ok(())
                    })
                    .build()
                    .to_element()
            )
            .child(
                Button::builder("error-btn", "Show Error")
                    .on_click(|| {
                        self.show_error_notification("An error occurred!");
                        Ok(())
                    })
                    .build()
                    .to_element()
            )
            .build()
    }

    pub fn update(&mut self, terminal_width: u16, terminal_height: u16) {
        self.overlay_manager.update_viewport(terminal_width, terminal_height);
        self.toast_system.update(terminal_width, terminal_height);
    }
}
```

The Overlay system provides comprehensive positioning and management functionality for floating UI elements in terminal applications, with automatic collision handling, multi-screen support, and flexible positioning options.