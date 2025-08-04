# Modal Widget

Modal dialog component with backdrop, focus management, and various modal types for user interactions and content overlays.

## Overview

The Modal widget provides overlay dialogs with backdrop support, focus trapping, keyboard navigation, and multiple modal types including alerts, confirmations, and custom content.

```rust
use reactive_tui::widgets::{Modal, ModalBuilder, ModalType, Backdrop};

let confirmation_modal = Modal::builder("confirm-delete")
    .title("Confirm Deletion")
    .content("Are you sure you want to delete this item?")
    .modal_type(ModalType::Confirmation)
    .backdrop(Backdrop::Blur)
    .closable(true)
    .add_button("Cancel", ButtonType::Secondary, || {
        close_modal();
        Ok(())
    })
    .add_button("Delete", ButtonType::Danger, || {
        perform_deletion();
        close_modal();
        Ok(())
    })
    .build();
```

## ModalBuilder

```rust
impl ModalBuilder {
    pub fn new(id: &str) -> Self
    pub fn title(mut self, title: &str) -> Self
    pub fn content(mut self, content: &str) -> Self
    pub fn modal_type(mut self, modal_type: ModalType) -> Self
    pub fn size(mut self, size: ModalSize) -> Self
    pub fn position(mut self, position: ModalPosition) -> Self
    pub fn backdrop(mut self, backdrop: Backdrop) -> Self
    pub fn closable(mut self, closable: bool) -> Self
    pub fn draggable(mut self, draggable: bool) -> Self
    pub fn resizable(mut self, resizable: bool) -> Self
    pub fn add_button<F>(mut self, text: &str, button_type: ButtonType, callback: F) -> Self
    pub fn custom_content(mut self, content: Element) -> Self
    pub fn on_open<F>(mut self, callback: F) -> Self
    pub fn on_close<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Modal
}
```

## Modal Types

```rust
pub enum ModalType {
    Alert,          // Simple alert dialog
    Confirmation,   // Yes/No confirmation
    Prompt,         // Input prompt dialog
    Custom,         // Custom content modal
    Form,           // Form dialog
    Loading,        // Loading/progress modal
    FullScreen,     // Full screen overlay
}
```

## Modal Sizes

```rust
pub enum ModalSize {
    Small,          // 300x200
    Medium,         // 500x300
    Large,          // 700x400
    ExtraLarge,     // 900x600
    FullScreen,     // Full screen
    Custom(u16, u16), // Custom width/height
    Auto,           // Size to content
}
```

## Backdrop Options

```rust
pub enum Backdrop {
    None,           // No backdrop
    Dim,            // Dim background
    Blur,           // Blur background
    Color(Color),   // Solid color backdrop
    Custom(String), // Custom backdrop style
}
```

## Modal Positioning

```rust
pub enum ModalPosition {
    Center,         // Center of screen
    Top,            // Top of screen
    Bottom,         // Bottom of screen
    Custom(u16, u16), // Custom x, y position
}
```

## Examples

### Alert Modal

```rust
use reactive_tui::widgets::{Modal, ModalType, Backdrop};

let alert_modal = Modal::builder("alert")
    .title("Error")
    .content("Failed to save file. Please check permissions and try again.")
    .modal_type(ModalType::Alert)
    .size(ModalSize::Medium)
    .backdrop(Backdrop::Dim)
    .add_button("OK", ButtonType::Primary, || {
        close_modal();
        Ok(())
    })
    .build();

// Show modal
alert_modal.show()?;
```

### Confirmation Modal

```rust
let confirm_modal = Modal::builder("confirm-action")
    .title("Confirm Action")
    .content("This action cannot be undone. Do you want to continue?")
    .modal_type(ModalType::Confirmation)
    .size(ModalSize::Small)
    .backdrop(Backdrop::Blur)
    .add_button("Cancel", ButtonType::Secondary, || {
        close_modal();
        Ok(())
    })
    .add_button("Continue", ButtonType::Primary, || {
        perform_action();
        close_modal();
        Ok(())
    })
    .build();
```

### Input Prompt Modal

```rust
use reactive_tui::{widgets::{Modal, Input}, reactive::Reactive};

let input_value = Reactive::new(String::new());
let input_clone = input_value.clone();

let prompt_modal = Modal::builder("prompt")
    .title("Enter Name")
    .modal_type(ModalType::Prompt)
    .size(ModalSize::Medium)
    .custom_content(
        Element::with_tag("div")
            .child(
                Element::with_tag("p")
                    .text("Please enter your name:")
                    .build()
            )
            .child(
                Input::builder("name-input")
                    .placeholder("Your name")
                    .value(&input_value.get())
                    .on_change(move |value| {
                        input_clone.set(value.to_string());
                        Ok(())
                    })
                    .build()
                    .to_element()
            )
            .build()
    )
    .add_button("Cancel", ButtonType::Secondary, || {
        close_modal();
        Ok(())
    })
    .add_button("OK", ButtonType::Primary, {
        let input = input_value.clone();
        move || {
            let name = input.get();
            if !name.is_empty() {
                save_name(&name);
                close_modal();
            }
            Ok(())
        }
    })
    .build();
```

### Form Modal

```rust
use reactive_tui::widgets::{Modal, Form, Input, Button};

let form_modal = Modal::builder("user-form")
    .title("Edit User")
    .modal_type(ModalType::Form)
    .size(ModalSize::Large)
    .draggable(true)
    .resizable(true)
    .custom_content(
        Form::builder("user-form")
            .add_field(
                Input::builder("username")
                    .label("Username")
                    .required(true)
                    .build()
            )
            .add_field(
                Input::builder("email")
                    .label("Email")
                    .input_type(InputType::Email)
                    .required(true)
                    .build()
            )
            .add_field(
                Input::builder("phone")
                    .label("Phone")
                    .input_type(InputType::Tel)
                    .build()
            )
            .build()
            .to_element()
    )
    .add_button("Cancel", ButtonType::Secondary, || {
        close_modal();
        Ok(())
    })
    .add_button("Save", ButtonType::Primary, || {
        if validate_form() {
            save_user_data();
            close_modal();
        }
        Ok(())
    })
    .on_close(|| {
        cleanup_form_data();
        Ok(())
    })
    .build();
```

### Loading Modal

```rust
use reactive_tui::{widgets::{Modal, Spinner}, reactive::Reactive};

let loading_progress = Reactive::new(0f32);
let progress_clone = loading_progress.clone();

let loading_modal = Modal::builder("loading")
    .title("Processing...")
    .modal_type(ModalType::Loading)
    .size(ModalSize::Medium)
    .backdrop(Backdrop::Dim)
    .closable(false) // Prevent manual closing
    .custom_content(
        Element::with_tag("div")
            .class("loading-content")
            .child(
                Spinner::new("loading-spinner")
                    .spinner_type(SpinnerType::Dots)
                    .build()
                    .to_element()
            )
            .child(
                Element::with_tag("p")
                    .text(&format!("Progress: {:.0}%", loading_progress.get()))
                    .build()
            )
            .build()
    )
    .build();

// Update progress
tokio::spawn(async move {
    for i in 0..=100 {
        progress_clone.set(i as f32);
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
    }
    close_modal();
});
```

### Full Screen Modal

```rust
let fullscreen_modal = Modal::builder("fullscreen")
    .title("Full Screen View")
    .modal_type(ModalType::FullScreen)
    .size(ModalSize::FullScreen)
    .backdrop(Backdrop::None)
    .closable(true)
    .custom_content(
        Element::with_tag("div")
            .class("fullscreen-content")
            .text("Full screen modal content")
            .build()
    )
    .add_button("Close", ButtonType::Secondary, || {
        close_modal();
        Ok(())
    })
    .build();
```

## Modal Management

```rust
use reactive_tui::widgets::{ModalManager, ModalStack};

let mut modal_manager = ModalManager::new();

// Show modal
modal_manager.show_modal(confirmation_modal).await?;

// Modal stack for multiple modals
let modal_stack = ModalStack::new();
modal_stack.push(first_modal);
modal_stack.push(second_modal); // Shows on top of first

// Close current modal
modal_stack.pop();

// Close all modals
modal_stack.clear();
```

## State Management

```rust
use reactive_tui::{widgets::Modal, reactive::Reactive};

let modal_state = Reactive::new(ModalState {
    is_open: false,
    current_modal: None,
    data: None,
});

let state_clone = modal_state.clone();

let stateful_modal = Modal::builder("stateful")
    .title("Stateful Modal")
    .on_open(move || {
        let mut state = state_clone.get();
        state.is_open = true;
        state_clone.set(state);
        Ok(())
    })
    .on_close({
        let state = modal_state.clone();
        move || {
            let mut current_state = state.get();
            current_state.is_open = false;
            current_state.current_modal = None;
            state.set(current_state);
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.modal-overlay {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    z-index: 1000;
}

.modal-backdrop {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background-color: rgba(0, 0, 0, 0.5);
}

.modal-backdrop-blur {
    backdrop-filter: blur(4px);
}

.modal-container {
    position: relative;
    background-color: white;
    border-radius: 8px;
    box-shadow: 0 10px 25px rgba(0, 0, 0, 0.2);
    max-width: 90vw;
    max-height: 90vh;
    overflow: hidden;
    display: flex;
    flex-direction: column;
    animation: modal-appear 0.2s ease-out;
}

@keyframes modal-appear {
    from {
        opacity: 0;
        transform: scale(0.9) translateY(-20px);
    }
    to {
        opacity: 1;
        transform: scale(1) translateY(0);
    }
}

.modal-header {
    padding: 16px 20px;
    border-bottom: 1px solid #e5e7eb;
    display: flex;
    align-items: center;
    justify-content: space-between;
}

.modal-title {
    font-size: 18px;
    font-weight: 600;
    margin: 0;
}

.modal-close {
    background: none;
    border: none;
    font-size: 24px;
    cursor: pointer;
    padding: 4px;
    border-radius: 4px;
    transition: background-color 0.2s ease;
}

.modal-close:hover {
    background-color: #f3f4f6;
}

.modal-body {
    padding: 20px;
    flex: 1;
    overflow-y: auto;
}

.modal-footer {
    padding: 16px 20px;
    border-top: 1px solid #e5e7eb;
    display: flex;
    justify-content: flex-end;
    gap: 12px;
}

.modal-small {
    width: 300px;
    min-height: 200px;
}

.modal-medium {
    width: 500px;
    min-height: 300px;
}

.modal-large {
    width: 700px;
    min-height: 400px;
}

.modal-fullscreen {
    width: 100vw;
    height: 100vh;
    border-radius: 0;
}

.modal-draggable .modal-header {
    cursor: move;
}

.modal-resizable {
    resize: both;
    overflow: hidden;
}
```

## Keyboard Navigation

```rust
// Built-in keyboard shortcuts
// Escape: Close modal (if closable)
// Tab: Navigate between buttons
// Enter: Activate focused button
// Space: Activate focused button

let keyboard_modal = Modal::builder("keyboard-modal")
    .keyboard_navigation(true)
    .escape_to_close(true)
    .focus_trap(true) // Keep focus within modal
    .initial_focus("ok-button")
    .build();
```

## Integration Examples

### Async Modal Operations

```rust
use reactive_tui::widgets::Modal;

async fn show_async_modal() -> Result<bool> {
    let modal = Modal::builder("async-confirm")
        .title("Async Confirmation")
        .content("Do you want to proceed with the async operation?")
        .modal_type(ModalType::Confirmation)
        .build();
    
    // Show modal and wait for result
    let result = modal.show_and_wait().await?;
    
    match result {
        ModalResult::Button(0) => Ok(false), // Cancel
        ModalResult::Button(1) => Ok(true),  // Confirm
        _ => Ok(false),
    }
}

// Usage
if show_async_modal().await? {
    perform_async_operation().await?;
}
```

The Modal widget provides comprehensive dialog functionality with backdrop management, focus trapping, and extensive customization options for terminal applications.