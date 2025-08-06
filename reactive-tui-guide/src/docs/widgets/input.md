# Input Widget

The Input widget provides text input fields with validation, formatting, and accessibility features.

## Features

- **Input Types**: Text, Password, Email, Number, URL
- **Validation**: Built-in and custom validators
- **Formatting**: Auto-formatting for specific input types
- **Accessibility**: Screen reader support and keyboard navigation

## Basic Usage

```rust
use reactive_tui::widgets::*;

let input = Input::builder("username")
    .placeholder("Enter username")
    .required(true)
    .max_length(20)
    .validation_pattern(r"^[a-zA-Z0-9_]+$")
    .build();
```

## Input Types

- **Text**: General text input
- **Password**: Masked password input
- **Email**: Email validation
- **Number**: Numeric input with constraints
- **URL**: URL validation

## Validation

- **required**: Field is required
- **min_length/max_length**: Length constraints
- **pattern**: Regex validation
- **custom_validator**: Custom validation function

## Configuration Options

- **placeholder**: Placeholder text
- **value**: Initial/current value
- **disabled**: Enable/disable state
- **readonly**: Read-only mode

## Events

- **on_change**: Value change events
- **on_focus**: Focus state changes
- **on_blur**: Blur events
- **on_validation**: Validation state changes
