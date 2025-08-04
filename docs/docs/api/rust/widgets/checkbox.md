# Checkbox Widget

Checkbox input component with support for individual checkboxes, checkbox groups, indeterminate states, and custom styling.

## Overview

The Checkbox widget provides boolean input controls with support for groups, tri-state checkboxes, and extensive customization options.

```rust
use reactive_tui::widgets::{Checkbox, CheckboxBuilder, CheckboxGroup, CheckboxState};

let subscribe_checkbox = Checkbox::builder("subscribe")
    .label("Subscribe to newsletter")
    .checked(false)
    .required(false)
    .build();
```

## CheckboxBuilder

```rust
impl CheckboxBuilder {
    pub fn new(id: &str) -> Self
    pub fn label(mut self, label: &str) -> Self
    pub fn checked(mut self, checked: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn required(mut self, required: bool) -> Self
    pub fn indeterminate(mut self, indeterminate: bool) -> Self
    pub fn value(mut self, value: &str) -> Self
    pub fn description(mut self, description: &str) -> Self
    pub fn on_change<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Checkbox
}
```

## Checkbox States

```rust
pub enum CheckboxState {
    Unchecked,      // □ Not checked
    Checked,        // ✓ Checked
    Indeterminate,  // ◐ Partially checked (for groups)
}

impl Checkbox {
    pub fn is_checked(&self) -> bool
    pub fn is_indeterminate(&self) -> bool
    pub fn set_state(&mut self, state: CheckboxState)
    pub fn toggle(&mut self)
}
```

## Checkbox Groups

```rust
pub struct CheckboxGroup {
    pub id: String,
    pub name: String,
    pub checkboxes: Vec<Checkbox>,
    pub required_count: Option<usize>,
    pub max_selections: Option<usize>,
}

impl CheckboxGroup {
    pub fn builder(id: &str) -> CheckboxGroupBuilder
    pub fn add_checkbox(&mut self, checkbox: Checkbox)
    pub fn get_selected_values(&self) -> Vec<String>
    pub fn select_all(&mut self)
    pub fn deselect_all(&mut self)
    pub fn is_valid(&self) -> bool
}
```

## Examples

### Basic Checkbox

```rust
use reactive_tui::widgets::Checkbox;

let terms_checkbox = Checkbox::builder("terms")
    .label("I agree to the terms and conditions")
    .required(true)
    .description("You must agree to continue")
    .on_change(|checked| {
        update_form_validity();
        Ok(())
    })
    .build();
```

### Checkbox with State Management

```rust
use reactive_tui::{widgets::Checkbox, reactive::Reactive};

let newsletter_subscribed = Reactive::new(false);
let newsletter_clone = newsletter_subscribed.clone();

let newsletter_checkbox = Checkbox::builder("newsletter")
    .label("Subscribe to newsletter")
    .checked(newsletter_subscribed.get())
    .description("Receive weekly updates about new features")
    .on_change(move |checked| {
        newsletter_clone.set(checked);
        if checked {
            println!("Subscribed to newsletter");
        } else {
            println!("Unsubscribed from newsletter");
        }
        Ok(())
    })
    .build();
```

### Checkbox Group

```rust
use reactive_tui::widgets::{CheckboxGroup, Checkbox};

let preferences_group = CheckboxGroup::builder("preferences")
    .name("Notification Preferences")
    .add_checkbox(
        Checkbox::builder("email_notifications")
            .label("Email notifications")
            .value("email")
            .checked(true)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("push_notifications")
            .label("Push notifications")
            .value("push")
            .checked(false)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("sms_notifications")
            .label("SMS notifications")
            .value("sms")
            .checked(false)
            .build()
    )
    .required_count(1) // At least one must be selected
    .on_change(|selected_values| {
        println!("Selected notifications: {:?}", selected_values);
        Ok(())
    })
    .build();
```

### Indeterminate Checkbox (Parent/Child)

```rust
use reactive_tui::widgets::{Checkbox, CheckboxState};

let select_all_checkbox = Checkbox::builder("select_all")
    .label("Select All")
    .indeterminate(true)
    .on_change(|checked| {
        if checked {
            select_all_children();
        } else {
            deselect_all_children();
        }
        Ok(())
    })
    .build();

let child_checkboxes = vec![
    Checkbox::builder("item1").label("Item 1").build(),
    Checkbox::builder("item2").label("Item 2").build(),
    Checkbox::builder("item3").label("Item 3").build(),
];

// Update parent state based on children
fn update_parent_state() {
    let checked_count = child_checkboxes.iter()
        .filter(|cb| cb.is_checked())
        .count();
    
    let parent_state = if checked_count == 0 {
        CheckboxState::Unchecked
    } else if checked_count == child_checkboxes.len() {
        CheckboxState::Checked
    } else {
        CheckboxState::Indeterminate
    };
    
    select_all_checkbox.set_state(parent_state);
}
```

### Feature Selection Checkboxes

```rust
let feature_checkboxes = CheckboxGroup::builder("features")
    .name("Select Features")
    .add_checkbox(
        Checkbox::builder("dark_mode")
            .label("Dark Mode")
            .description("Enable dark theme")
            .checked(true)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("auto_save")
            .label("Auto Save")
            .description("Automatically save changes")
            .checked(false)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("spell_check")
            .label("Spell Check")
            .description("Check spelling as you type")
            .checked(true)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("advanced_features")
            .label("Advanced Features")
            .description("Enable experimental features")
            .checked(false)
            .disabled(true) // Coming soon
            .build()
    )
    .build();
```

### Permissions Checkboxes

```rust
let permissions_group = CheckboxGroup::builder("permissions")
    .name("User Permissions")
    .add_checkbox(
        Checkbox::builder("read")
            .label("Read")
            .value("read")
            .checked(true)
            .required(true) // Always required
            .build()
    )
    .add_checkbox(
        Checkbox::builder("write")
            .label("Write")
            .value("write")
            .checked(false)
            .build()
    )
    .add_checkbox(
        Checkbox::builder("delete")
            .label("Delete")
            .value("delete")
            .checked(false)
            .description("Allow user to delete items")
            .build()
    )
    .add_checkbox(
        Checkbox::builder("admin")
            .label("Administrator")
            .value("admin")
            .checked(false)
            .description("Full administrative access")
            .on_change(|checked| {
                if checked {
                    // Auto-enable all other permissions
                    enable_all_permissions();
                }
                Ok(())
            })
            .build()
    )
    .validation(|selected| {
        if selected.contains("admin") && selected.len() == 1 {
            Err("Admin users must have additional permissions".to_string())
        } else {
            Ok(())
        }
    })
    .build();
```

### Form Integration

```rust
use reactive_tui::widgets::{Form, Checkbox, Input, Button};

let registration_form = Form::builder("registration")
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
        Checkbox::builder("terms")
            .label("I agree to the Terms of Service")
            .required(true)
            .build()
    )
    .add_field(
        Checkbox::builder("privacy")
            .label("I agree to the Privacy Policy")
            .required(true)
            .build()
    )
    .add_field(
        Checkbox::builder("marketing")
            .label("I want to receive marketing emails")
            .checked(false)
            .description("Optional - you can change this later")
            .build()
    )
    .add_button(
        Button::builder("submit", "Register")
            .button_type(ButtonType::Primary)
            .disabled_unless_valid(true)
            .build()
    )
    .build();
```

## State Management

```rust
use reactive_tui::{widgets::Checkbox, reactive::Reactive};

struct CheckboxFormState {
    notifications_enabled: bool,
    email_notifications: bool,
    push_notifications: bool,
    sms_notifications: bool,
}

let form_state = Reactive::new(CheckboxFormState {
    notifications_enabled: false,
    email_notifications: false,
    push_notifications: false,
    sms_notifications: false,
});

let master_checkbox = Checkbox::builder("notifications_master")
    .label("Enable Notifications")
    .checked(form_state.get().notifications_enabled)
    .on_change({
        let state = form_state.clone();
        move |checked| {
            let mut current_state = state.get();
            current_state.notifications_enabled = checked;
            
            // Disable all sub-options if master is disabled
            if !checked {
                current_state.email_notifications = false;
                current_state.push_notifications = false;
                current_state.sms_notifications = false;
            }
            
            state.set(current_state);
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.checkbox-container {
    display: flex;
    align-items: flex-start;
    gap: 8px;
    padding: 8px 0;
    cursor: pointer;
}

.checkbox-input {
    width: 16px;
    height: 16px;
    border: 2px solid #d1d5db;
    border-radius: 3px;
    background-color: white;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s ease;
    flex-shrink: 0;
    margin-top: 2px;
}

.checkbox-input:hover {
    border-color: #3b82f6;
}

.checkbox-input.checked {
    background-color: #3b82f6;
    border-color: #3b82f6;
    color: white;
}

.checkbox-input.indeterminate {
    background-color: #3b82f6;
    border-color: #3b82f6;
    color: white;
}

.checkbox-input.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.checkbox-checkmark {
    font-size: 12px;
    font-weight: bold;
}

.checkbox-indeterminate-mark {
    width: 8px;
    height: 2px;
    background-color: white;
}

.checkbox-label-container {
    flex: 1;
}

.checkbox-label {
    font-size: 14px;
    font-weight: 500;
    color: #374151;
    line-height: 1.4;
}

.checkbox-label.required::after {
    content: " *";
    color: #ef4444;
}

.checkbox-label.disabled {
    color: #9ca3af;
}

.checkbox-description {
    font-size: 12px;
    color: #6b7280;
    margin-top: 2px;
    line-height: 1.3;
}

.checkbox-group {
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    padding: 16px;
}

.checkbox-group-title {
    font-size: 16px;
    font-weight: 600;
    margin-bottom: 12px;
    color: #374151;
}

.checkbox-group-item {
    margin-bottom: 8px;
}

.checkbox-group-item:last-child {
    margin-bottom: 0;
}

.checkbox-validation-error {
    color: #ef4444;
    font-size: 12px;
    margin-top: 4px;
}
```

## Validation

```rust
use reactive_tui::widgets::{Checkbox, ValidationRule};

let validated_checkbox = Checkbox::builder("terms")
    .label("I agree to the terms")
    .required(true)
    .validation(ValidationRule::Required)
    .error_message("You must agree to the terms to continue")
    .build();

// Group validation
let group_with_validation = CheckboxGroup::builder("skills")
    .name("Select Your Skills")
    .add_checkbox(Checkbox::builder("rust").label("Rust").value("rust").build())
    .add_checkbox(Checkbox::builder("javascript").label("JavaScript").value("js").build())
    .add_checkbox(Checkbox::builder("python").label("Python").value("python").build())
    .validation(|selected| {
        if selected.is_empty() {
            Err("Please select at least one skill".to_string())
        } else if selected.len() > 3 {
            Err("Please select no more than 3 skills".to_string())
        } else {
            Ok(())
        }
    })
    .build();
```

## Accessibility

```rust
let accessible_checkbox = Checkbox::builder("accessible")
    .label("Accessible checkbox")
    .aria_label("Subscribe to newsletter updates")
    .aria_describedby("newsletter-description")
    .keyboard_navigation(true)
    .focus_visible(true)
    .build();
```

## Integration Examples

### Settings Panel

```rust
use reactive_tui::widgets::{Checkbox, CheckboxGroup, Panel};

let settings_panel = Panel::builder("app-settings")
    .title("Application Settings")
    .content(
        Element::with_tag("div")
            .child(
                CheckboxGroup::builder("ui_settings")
                    .name("User Interface")
                    .add_checkbox(Checkbox::builder("dark_mode").label("Dark Mode").build())
                    .add_checkbox(Checkbox::builder("compact_view").label("Compact View").build())
                    .add_checkbox(Checkbox::builder("animations").label("Enable Animations").build())
                    .build()
                    .to_element()
            )
            .child(
                CheckboxGroup::builder("privacy_settings")
                    .name("Privacy")
                    .add_checkbox(Checkbox::builder("analytics").label("Send Usage Analytics").build())
                    .add_checkbox(Checkbox::builder("crash_reports").label("Send Crash Reports").build())
                    .build()
                    .to_element()
            )
            .build()
    )
    .build();
```

The Checkbox widget provides comprehensive boolean input functionality with support for groups, validation, and extensive customization options for terminal applications.