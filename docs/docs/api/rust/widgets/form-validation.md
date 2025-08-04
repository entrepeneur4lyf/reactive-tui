# FormValidator Widget

A comprehensive form validation system supporting field validation, error messages, custom validators, and real-time validation feedback with styling and accessibility support.

## Overview

The FormValidator widget provides complete form validation functionality with built-in validators, custom validation rules, real-time feedback, and integration with reactive state management. It supports multiple field types and validation timing options.

```rust
use reactive_tui::widgets::*;

let mut form = FormValidator::builder("user-form")
    .field(
        FormField::new("email", "Email Address")
            .field_type(FieldType::Email)
            .required(true)
            .add_rule(ValidationRule::Email)
            .add_rule(ValidationRule::MaxLength(255))
    )
    .field(
        FormField::new("password", "Password")
            .field_type(FieldType::Password)
            .required(true)
            .add_rule(ValidationRule::MinLength(8))
            .add_rule(ValidationRule::Pattern(r"(?=.*[A-Za-z])(?=.*\d)".to_string()))
    )
    .validate_on_blur(true)
    .build();

// Set values and validate
form.set_field_value("email", "user@example.com")?;
form.set_field_value("password", "mypassword123")?;

if form.is_valid() {
    let form_data = form.get_form_data();
    // Submit form
}
```

## Features

- **Field Validation**: Support for text, email, password, number, and custom field types
- **Built-in Validators**: Required, email, URL, numeric, length, pattern validation
- **Custom Validators**: Support for custom validation functions with async capabilities
- **Real-time Validation**: Validate on input, blur, or submit with configurable timing
- **Error Styling**: Customizable error messages with colors and positioning
- **Form State Management**: Track form validity, touched fields, and submission state
- **Accessibility**: Full ARIA support with error announcements
- **Conditional Validation**: Fields that validate based on other field values
- **Validation Groups**: Group related fields for complex validation scenarios
- **Internationalization**: Support for custom error messages and localization

## Core Components

### FormValidator

Main form validation widget managing multiple fields.

```rust
pub struct FormValidator {
    pub id: String,
    pub fields: HashMap<FieldId, FormField>,
    pub field_order: Vec<FieldId>,
    pub state: Reactive<FormValidationState>,
    pub config: FormValidationConfig,
    pub style: FormValidationStyle,
    pub callbacks: FormValidationCallbacks,
    pub css_classes: Vec<String>,
    pub custom_validators: HashMap<String, CustomValidator>,
}
```

### FormField

Individual form field with validation rules and state.

```rust
pub struct FormField {
    pub id: FieldId,
    pub label: String,
    pub field_type: FieldType,
    pub value: String,
    pub required: bool,
    pub disabled: bool,
    pub touched: bool,
    pub placeholder: String,
    pub help_text: Option<String>,
    pub rules: Vec<ValidationRule>,
    pub validation_result: Option<ValidationResult>,
    pub css_classes: Vec<String>,
    pub config: HashMap<String, String>,
}

impl FormField {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self
    pub fn field_type(mut self, field_type: FieldType) -> Self
    pub fn required(mut self, required: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
    pub fn help_text(mut self, help_text: impl Into<String>) -> Self
    pub fn add_rule(mut self, rule: ValidationRule) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
    
    // State checking methods
    pub fn has_errors(&self) -> bool
    pub fn has_warnings(&self) -> bool
    pub fn get_errors(&self) -> Vec<&ValidationMessage>
    pub fn get_warnings(&self) -> Vec<&ValidationMessage>
}
```

### Field Types

```rust
pub enum FieldType {
    Text,
    Email,
    Password,
    Number,
    Url,
    Phone,
    Date,
    Time,
    Textarea,
    Select,
    Checkbox,
    Radio,
    File,
    Custom(String),
}
```

### Validation Rules

```rust
pub enum ValidationRule {
    Required,
    MinLength(usize),
    MaxLength(usize),
    ExactLength(usize),
    Email,
    Url,
    NumberRange { min: Option<f64>, max: Option<f64> },
    Pattern(String),
    Custom(String),           // Reference to custom validator
    MatchField(FieldId),      // Must match another field
    OneOf(Vec<String>),       // Must be one of specified values
    NoneOf(Vec<String>),      // Must not be any of specified values
}
```

### Validation Results

```rust
pub struct ValidationResult {
    pub valid: bool,
    pub messages: Vec<ValidationMessage>,
    pub value: String,
    pub timestamp: u64,
}

impl ValidationResult {
    pub fn valid() -> Self
    pub fn invalid(message: impl Into<String>) -> Self
    pub fn with_warnings(warnings: Vec<ValidationMessage>) -> Self
}

pub struct ValidationMessage {
    pub message: String,
    pub severity: ValidationSeverity,
    pub rule: Option<String>,
    pub field_id: Option<FieldId>,
}

pub enum ValidationSeverity {
    Info,
    Warning,
    Error,
}
```

### Configuration

```rust
pub struct FormValidationConfig {
    pub validation_timing: ValidationTiming,
    pub validate_on_input: bool,
    pub validate_on_blur: bool,
    pub validate_on_submit: bool,
    pub input_debounce_ms: u64,
    pub show_validation_icons: bool,
    pub show_help_text: bool,
    pub stop_on_first_error: bool,
    pub async_validation: bool,
    pub error_templates: HashMap<String, String>,
}

pub enum ValidationTiming {
    OnInput,    // Validate as user types
    OnBlur,     // Validate when field loses focus
    OnSubmit,   // Validate only on form submission
    Manual,     // Custom timing controlled by application
}
```

## Builder Pattern

### FormValidatorBuilder

```rust
impl FormValidatorBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn field(mut self, field: FormField) -> Self
    pub fn validation_timing(mut self, timing: ValidationTiming) -> Self
    pub fn validate_on_input(mut self, enabled: bool) -> Self
    pub fn validate_on_blur(mut self, enabled: bool) -> Self
    pub fn validate_on_submit(mut self, enabled: bool) -> Self
    pub fn input_debounce_ms(mut self, ms: u64) -> Self
    pub fn show_validation_icons(mut self, show: bool) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn on_field_validate<F>(mut self, callback: F) -> Self
    pub fn on_form_validate<F>(mut self, callback: F) -> Self
    pub fn on_field_change<F>(mut self, callback: F) -> Self
    pub fn on_submit<F>(mut self, callback: F) -> Self
    pub fn build(self) -> FormValidator
}
```

## Methods

### Field Management

```rust
impl FormValidator {
    // Add field to form
    pub fn add_field(&mut self, field: FormField)
    
    // Remove field from form
    pub fn remove_field(&mut self, field_id: &str) -> Option<FormField>
    
    // Set field value and validate if configured
    pub fn set_field_value(&mut self, field_id: &str, value: impl Into<String>) -> Result<()>
    
    // Get field value
    pub fn get_field_value(&self, field_id: &str) -> Option<&str>
}
```

### Validation Control

```rust
impl FormValidator {
    // Validate specific field
    pub fn validate_field(&mut self, field_id: &str) -> Result<ValidationResult>
    
    // Validate all fields
    pub fn validate_all(&mut self) -> Result<bool>
    
    // Check if form is valid
    pub fn is_valid(&self) -> bool
    
    // Submit form (validates and triggers callback)
    pub fn submit(&mut self) -> Result<bool>
    
    // Reset form to initial state
    pub fn reset(&mut self)
}
```

### State Management

```rust
impl FormValidator {
    // Get form data as key-value pairs
    pub fn get_form_data(&self) -> HashMap<FieldId, String>
    
    // Set form disabled state
    pub fn set_disabled(&mut self, disabled: bool)
    
    // Check if form is disabled
    pub fn is_disabled(&self) -> bool
    
    // Add custom validator
    pub fn add_custom_validator<F>(&mut self, name: String, validator: F)
    where F: Fn(&str) -> ValidationResult + Send + Sync + 'static
}
```

## Examples

### User Registration Form

```rust
use reactive_tui::widgets::*;

let registration_form = FormValidator::builder("registration")
    .field(
        FormField::new("username", "Username")
            .required(true)
            .add_rule(ValidationRule::MinLength(3))
            .add_rule(ValidationRule::MaxLength(50))
            .add_rule(ValidationRule::Pattern(r"^[a-zA-Z0-9_]+$".to_string()))
            .placeholder("Enter username")
            .help_text("3-50 characters, letters, numbers, and underscores only")
    )
    .field(
        FormField::new("email", "Email Address")
            .field_type(FieldType::Email)
            .required(true)
            .add_rule(ValidationRule::Email)
            .add_rule(ValidationRule::MaxLength(255))
            .placeholder("user@example.com")
    )
    .field(
        FormField::new("password", "Password")
            .field_type(FieldType::Password)
            .required(true)
            .add_rule(ValidationRule::MinLength(8))
            .add_rule(ValidationRule::Pattern(r"(?=.*[A-Za-z])(?=.*\d)(?=.*[@$!%*?&])".to_string()))
            .help_text("At least 8 characters with letters, numbers, and special characters")
    )
    .field(
        FormField::new("confirm_password", "Confirm Password")
            .field_type(FieldType::Password)
            .required(true)
            .add_rule(ValidationRule::MatchField("password".to_string()))
            .help_text("Must match password")
    )
    .field(
        FormField::new("age", "Age")
            .field_type(FieldType::Number)
            .required(true)
            .add_rule(ValidationRule::NumberRange { min: Some(13.0), max: Some(120.0) })
    )
    .validate_on_blur(true)
    .validate_on_submit(true)
    .on_submit(|form_data| {
        println!("Registration data: {:?}", form_data);
        create_user_account(form_data);
    })
    .build();
```

### Login Form with Real-time Validation

```rust
let mut login_form = FormValidator::builder("login")
    .field(
        FormField::new("email", "Email")
            .field_type(FieldType::Email)
            .required(true)
            .add_rule(ValidationRule::Email)
            .placeholder("Enter your email")
    )
    .field(
        FormField::new("password", "Password")
            .field_type(FieldType::Password)
            .required(true)
            .add_rule(ValidationRule::MinLength(1))
            .placeholder("Enter your password")
    )
    .validate_on_input(true)
    .input_debounce_ms(300)
    .on_field_validate(|field_id, result| {
        if !result.valid {
            println!("Field {} has errors: {:?}", field_id, result.messages);
        }
    })
    .on_submit(|form_data| {
        authenticate_user(&form_data["email"], &form_data["password"]);
    })
    .build();

// Simulate user input
login_form.set_field_value("email", "user@example.com")?;
login_form.set_field_value("password", "mypassword")?;

if login_form.submit()? {
    println!("Login successful!");
}
```

### Contact Form with Custom Validation

```rust
let mut contact_form = FormValidator::builder("contact")
    .field(
        FormField::new("name", "Full Name")
            .required(true)
            .add_rule(ValidationRule::MinLength(2))
            .add_rule(ValidationRule::MaxLength(100))
    )
    .field(
        FormField::new("email", "Email Address")
            .field_type(FieldType::Email)
            .required(true)
            .add_rule(ValidationRule::Email)
    )
    .field(
        FormField::new("phone", "Phone Number")
            .field_type(FieldType::Phone)
            .add_rule(ValidationRule::Custom("phone_validation".to_string()))
    )
    .field(
        FormField::new("subject", "Subject")
            .required(true)
            .add_rule(ValidationRule::MinLength(5))
            .add_rule(ValidationRule::MaxLength(200))
    )
    .field(
        FormField::new("message", "Message")
            .field_type(FieldType::Textarea)
            .required(true)
            .add_rule(ValidationRule::MinLength(20))
            .add_rule(ValidationRule::MaxLength(2000))
    )
    .validate_on_blur(true)
    .build();

// Add custom phone validator
contact_form.add_custom_validator(
    "phone_validation".to_string(),
    |value| {
        if value.is_empty() {
            return ValidationResult::valid();
        }
        
        let phone_regex = r"^\+?[\d\s\-\(\)]{10,}$";
        if regex::Regex::new(phone_regex).unwrap().is_match(value) {
            ValidationResult::valid()
        } else {
            ValidationResult::invalid("Please enter a valid phone number")
        }
    }
);
```

### Multi-step Form Validation

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

struct MultiStepForm {
    current_step: Reactive<usize>,
    steps: Vec<FormValidator>,
}

impl MultiStepForm {
    fn new() -> Self {
        let step1 = FormValidator::builder("step1")
            .field(
                FormField::new("first_name", "First Name")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(2))
            )
            .field(
                FormField::new("last_name", "Last Name")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(2))
            )
            .field(
                FormField::new("email", "Email")
                    .field_type(FieldType::Email)
                    .required(true)
                    .add_rule(ValidationRule::Email)
            )
            .validate_on_blur(true)
            .build();
        
        let step2 = FormValidator::builder("step2")
            .field(
                FormField::new("address", "Address")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(10))
            )
            .field(
                FormField::new("city", "City")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(2))
            )
            .field(
                FormField::new("postal_code", "Postal Code")
                    .required(true)
                    .add_rule(ValidationRule::Pattern(r"^\d{5}(-\d{4})?$".to_string()))
            )
            .validate_on_blur(true)
            .build();
        
        let step3 = FormValidator::builder("step3")
            .field(
                FormField::new("card_number", "Card Number")
                    .field_type(FieldType::Custom("credit_card".to_string()))
                    .required(true)
                    .add_rule(ValidationRule::Custom("credit_card_validation".to_string()))
            )
            .field(
                FormField::new("expiry", "Expiry Date")
                    .required(true)
                    .add_rule(ValidationRule::Pattern(r"^(0[1-9]|1[0-2])\/\d{2}$".to_string()))
                    .placeholder("MM/YY")
            )
            .field(
                FormField::new("cvv", "CVV")
                    .required(true)
                    .add_rule(ValidationRule::Pattern(r"^\d{3,4}$".to_string()))
            )
            .validate_on_blur(true)
            .build();
        
        Self {
            current_step: Reactive::new(0),
            steps: vec![step1, step2, step3],
        }
    }
    
    fn next_step(&mut self) -> Result<bool> {
        let current = self.current_step.get();
        if current < self.steps.len() {
            if self.steps[current].validate_all()? {
                self.current_step.set(current + 1);
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }
    
    fn previous_step(&mut self) {
        let current = self.current_step.get();
        if current > 0 {
            self.current_step.set(current - 1);
        }
    }
    
    fn submit_form(&mut self) -> Result<bool> {
        // Validate all steps
        for step in &mut self.steps {
            if !step.validate_all()? {
                return Ok(false);
            }
        }
        
        // Collect all form data
        let mut all_data = HashMap::new();
        for step in &self.steps {
            all_data.extend(step.get_form_data());
        }
        
        println!("Submitting complete form: {:?}", all_data);
        Ok(true)
    }
}
```

### Dynamic Form with Conditional Fields

```rust
struct DynamicForm {
    form: FormValidator,
    user_type: Reactive<String>,
}

impl DynamicForm {
    fn new() -> Self {
        let mut form = FormValidator::builder("dynamic")
            .field(
                FormField::new("user_type", "User Type")
                    .field_type(FieldType::Select)
                    .required(true)
                    .add_rule(ValidationRule::OneOf(vec![
                        "individual".to_string(),
                        "business".to_string(),
                        "organization".to_string(),
                    ]))
            )
            .field(
                FormField::new("name", "Name")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(2))
            )
            .field(
                FormField::new("email", "Email")
                    .field_type(FieldType::Email)
                    .required(true)
                    .add_rule(ValidationRule::Email)
            )
            .validate_on_blur(true)
            .build();
        
        let user_type = Reactive::new("individual".to_string());
        
        Self { form, user_type }
    }
    
    fn update_conditional_fields(&mut self) -> Result<()> {
        let user_type = self.user_type.get();
        
        // Remove conditional fields
        self.form.remove_field("company_name");
        self.form.remove_field("tax_id");
        self.form.remove_field("organization_type");
        
        match user_type.as_str() {
            "business" => {
                self.form.add_field(
                    FormField::new("company_name", "Company Name")
                        .required(true)
                        .add_rule(ValidationRule::MinLength(2))
                );
                self.form.add_field(
                    FormField::new("tax_id", "Tax ID")
                        .required(true)
                        .add_rule(ValidationRule::Pattern(r"^\d{2}-\d{7}$".to_string()))
                );
            }
            "organization" => {
                self.form.add_field(
                    FormField::new("organization_type", "Organization Type")
                        .field_type(FieldType::Select)
                        .required(true)
                        .add_rule(ValidationRule::OneOf(vec![
                            "nonprofit".to_string(),
                            "government".to_string(),
                            "educational".to_string(),
                        ]))
                );
            }
            _ => {} // Individual - no additional fields
        }
        
        Ok(())
    }
}
```

### Form with Async Validation

```rust
use reactive_tui::{widgets::*, reactive::Reactive};
use tokio::time::{timeout, Duration};

struct AsyncValidationForm {
    form: FormValidator,
}

impl AsyncValidationForm {
    fn new() -> Self {
        let mut form = FormValidator::builder("async-form")
            .field(
                FormField::new("username", "Username")
                    .required(true)
                    .add_rule(ValidationRule::MinLength(3))
                    .add_rule(ValidationRule::Custom("username_availability".to_string()))
            )
            .field(
                FormField::new("email", "Email")
                    .field_type(FieldType::Email)
                    .required(true)
                    .add_rule(ValidationRule::Email)
                    .add_rule(ValidationRule::Custom("email_availability".to_string()))
            )
            .validate_on_blur(true)
            .input_debounce_ms(500)
            .build();
        
        // Add async validators
        form.add_custom_validator(
            "username_availability".to_string(),
            |username| {
                // Simulate async API call
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(async {
                    timeout(Duration::from_secs(2), check_username_availability(username)).await
                });
                
                match result {
                    Ok(Ok(available)) => {
                        if available {
                            ValidationResult::valid()
                        } else {
                            ValidationResult::invalid("Username is already taken")
                        }
                    }
                    Ok(Err(_)) => {
                        ValidationResult::invalid("Error checking username availability")
                    }
                    Err(_) => {
                        ValidationResult::with_warnings(vec![
                            ValidationMessage::warning("Timeout checking username availability")
                        ])
                    }
                }
            }
        );
        
        form.add_custom_validator(
            "email_availability".to_string(),
            |email| {
                let rt = tokio::runtime::Runtime::new().unwrap();
                let result = rt.block_on(async {
                    timeout(Duration::from_secs(2), check_email_availability(email)).await
                });
                
                match result {
                    Ok(Ok(available)) => {
                        if available {
                            ValidationResult::valid()
                        } else {
                            ValidationResult::invalid("Email is already registered")
                        }
                    }
                    Ok(Err(_)) => {
                        ValidationResult::invalid("Error checking email availability")
                    }
                    Err(_) => {
                        ValidationResult::with_warnings(vec![
                            ValidationMessage::warning("Timeout checking email availability")
                        ])
                    }
                }
            }
        );
        
        Self { form }
    }
}

async fn check_username_availability(username: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Simulate API call
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(!["admin", "root", "user", "test"].contains(&username))
}

async fn check_email_availability(email: &str) -> Result<bool, Box<dyn std::error::Error>> {
    // Simulate API call
    tokio::time::sleep(Duration::from_millis(700)).await;
    Ok(!email.ends_with("@blocked.com"))
}
```

## Convenience Functions

Pre-configured form validators for common use cases:

```rust
// User registration form with standard fields
pub fn user_registration_form() -> FormValidator

// Simple login form (email/password)
pub fn login_form() -> FormValidator

// Contact form with name, email, subject, message
pub fn contact_form() -> FormValidator
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let form_ui = Element::with_tag("div")
    .class("form-container")
    .child(
        Element::with_tag("h2")
            .text("User Registration")
            .build()
    )
    .child(
        registration_form.to_element()
    )
    .child(
        Element::with_tag("div")
            .class("form-actions")
            .child(
                Element::with_tag("button")
                    .attr("type", "submit")
                    .text("Create Account")
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .attr("type", "reset")
                    .text("Clear Form")
                    .build()
            )
            .build()
    )
    .build();
```

## CSS Styling

The form validator generates semantic CSS classes:

```css
.form-validator {
    /* Base form styles */
}

.form-validator.form-invalid {
    /* Form with validation errors */
}

.form-validator.form-submitted {
    /* Form that has been submitted */
}

.form-validator.form-disabled {
    /* Disabled form */
}

.form-field {
    margin-bottom: 16px;
}

.form-field.field-error {
    /* Field with validation errors */
}

.form-field.field-warning {
    /* Field with validation warnings */
}

.form-field.field-valid {
    /* Valid field */
}

.validation-message {
    font-size: 12px;
    margin-top: 4px;
}

.validation-message.error {
    color: #ef4444;
}

.validation-message.warning {
    color: #f59e0b;
}

.validation-message.info {
    color: #3b82f6;
}
```

## Performance Considerations

- **Debounced Input**: Configurable debounce delay prevents excessive validation during typing
- **Selective Validation**: Only validates changed fields for optimal performance
- **Custom Validators**: Efficient custom validation with caching support
- **Memory Management**: Automatic cleanup of validation results and state

## Accessibility

- **ARIA Attributes**: Full ARIA support with proper field labeling and error associations
- **Screen Reader**: Error announcements and validation state changes
- **Keyboard Navigation**: Complete keyboard accessibility for all form interactions
- **Focus Management**: Proper focus handling during validation and error states

The FormValidator widget provides comprehensive form validation functionality with extensive customization options, real-time feedback, and seamless integration with reactive state management and accessibility features.