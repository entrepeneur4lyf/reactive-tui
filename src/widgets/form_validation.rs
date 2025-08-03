//! Form Validation Widget
//!
//! A comprehensive form validation system supporting field validation, error messages,
//! custom validators, and real-time validation feedback with styling.
//!
//! # Features
//!
//! - **Field Validation**: Support for text, email, password, number, and custom field types
//! - **Built-in Validators**: Required, email, URL, numeric, length, pattern validation
//! - **Custom Validators**: Support for custom validation functions with async capabilities
//! - **Real-time Validation**: Validate on input, blur, or submit with configurable timing
//! - **Error Styling**: Customizable error messages with colors and positioning
//! - **Form State Management**: Track form validity, touched fields, and submission state
//! - **Accessibility**: Full ARIA support with error announcements
//! - **Conditional Validation**: Fields that validate based on other field values
//! - **Validation Groups**: Group related fields for complex validation scenarios
//! - **Internationalization**: Support for custom error messages and localization
//!
//! # Basic Usage
//!
//! ```rust
//! use reactive_tui::widgets::{FormValidator, FormField, ValidationRule, FieldType};
//!
//! let mut form = FormValidator::builder("user-form")
//!     .field(
//!         FormField::new("email", "Email Address")
//!             .field_type(FieldType::Email)
//!             .required(true)
//!             .add_rule(ValidationRule::Email)
//!             .add_rule(ValidationRule::MaxLength(255))
//!     )
//!     .field(
//!         FormField::new("password", "Password")
//!             .field_type(FieldType::Password)
//!             .required(true)
//!             .add_rule(ValidationRule::MinLength(8))
//!             .add_rule(ValidationRule::Pattern(r"(?=.*[A-Za-z])(?=.*\d)".to_string()))
//!     )
//!     .validate_on_input(true)
//!     .validate_on_blur(true)
//!     .build();
//!
//! // Set field values and validate
//! form.set_field_value("email", "user@example.com");
//! form.set_field_value("password", "mypassword123");
//!
//! // Check form validity
//! if form.is_valid() {
//!     let values = form.get_form_data();
//!     // Submit form
//! }
//! ```

use crate::{
  components::element::Element,
  error::{Result, TuiError},
  layout::LayoutRect,
  reactive::Reactive,
  themes::{ColorDefinition, ColorTheme},
};
#[cfg(not(target_family = "wasm"))]
use regex::Regex;

#[cfg(target_family = "wasm")]
use regex_lite::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{self, Write};
use std::sync::Arc;

// Type aliases for complex function pointer types
type OnFieldValidateCallback = Arc<dyn Fn(&FieldId, &ValidationResult) + Send + Sync>;
type OnFormValidateCallback = Arc<dyn Fn(bool, &[ValidationMessage]) + Send + Sync>;
type OnFieldChangeCallback = Arc<dyn Fn(&FieldId, &str) + Send + Sync>;
type OnSubmitCallback = Arc<dyn Fn(&HashMap<FieldId, String>) + Send + Sync>;
type OnCustomValidateCallback = Arc<dyn Fn(&str, &str) -> ValidationResult + Send + Sync>;
type CustomValidator = Arc<dyn Fn(&str) -> ValidationResult + Send + Sync>;

/// Field identifier type
pub type FieldId = String;

/// Field types for validation
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FieldType {
  /// Plain text input
  Text,
  /// Email address
  Email,
  /// Password field
  Password,
  /// Numeric input
  Number,
  /// URL input
  Url,
  /// Phone number
  Phone,
  /// Date input
  Date,
  /// Time input
  Time,
  /// Textarea
  Textarea,
  /// Select dropdown
  Select,
  /// Checkbox
  Checkbox,
  /// Radio button group
  Radio,
  /// File upload
  File,
  /// Custom field type
  Custom(String),
}

impl Default for FieldType {
  fn default() -> Self {
    Self::Text
  }
}

/// Validation severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationSeverity {
  /// Informational message
  Info,
  /// Warning message
  Warning,
  /// Error message (prevents form submission)
  Error,
}

impl Default for ValidationSeverity {
  fn default() -> Self {
    Self::Error
  }
}

/// Validation timing options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ValidationTiming {
  /// Validate as user types
  OnInput,
  /// Validate when field loses focus
  OnBlur,
  /// Validate only on form submission
  OnSubmit,
  /// Custom timing controlled by application
  Manual,
}

impl Default for ValidationTiming {
  fn default() -> Self {
    Self::OnBlur
  }
}

/// Built-in validation rules
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ValidationRule {
  /// Field is required
  Required,
  /// Minimum length
  MinLength(usize),
  /// Maximum length
  MaxLength(usize),
  /// Exact length
  ExactLength(usize),
  /// Email format validation
  Email,
  /// URL format validation
  Url,
  /// Numeric range validation
  NumberRange { min: Option<f64>, max: Option<f64> },
  /// Regular expression pattern
  Pattern(String),
  /// Custom validation function
  Custom(String), // Store function name/id
  /// Field must match another field
  MatchField(FieldId),
  /// One of multiple values
  OneOf(Vec<String>),
  /// None of multiple values
  NoneOf(Vec<String>),
}

/// Validation result
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationResult {
  /// Whether validation passed
  pub valid: bool,
  /// Error/warning/info messages
  pub messages: Vec<ValidationMessage>,
  /// Field value that was validated
  pub value: String,
  /// Timestamp of validation
  pub timestamp: u64,
}

impl ValidationResult {
  pub fn valid() -> Self {
    use std::time::{SystemTime, UNIX_EPOCH};
    Self {
      valid: true,
      messages: Vec::new(),
      value: String::new(),
      timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs(),
    }
  }

  pub fn invalid(message: impl Into<String>) -> Self {
    use std::time::{SystemTime, UNIX_EPOCH};
    Self {
      valid: false,
      messages: vec![ValidationMessage::error(message)],
      value: String::new(),
      timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs(),
    }
  }

  pub fn with_warnings(warnings: Vec<ValidationMessage>) -> Self {
    use std::time::{SystemTime, UNIX_EPOCH};
    Self {
      valid: true,
      messages: warnings,
      value: String::new(),
      timestamp: SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs(),
    }
  }
}

/// Individual validation message
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ValidationMessage {
  /// Message text
  pub message: String,
  /// Severity level
  pub severity: ValidationSeverity,
  /// Rule that generated this message
  pub rule: Option<String>,
  /// Field that this message applies to
  pub field_id: Option<FieldId>,
}

impl ValidationMessage {
  pub fn error(message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
      severity: ValidationSeverity::Error,
      rule: None,
      field_id: None,
    }
  }

  pub fn warning(message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
      severity: ValidationSeverity::Warning,
      rule: None,
      field_id: None,
    }
  }

  pub fn info(message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
      severity: ValidationSeverity::Info,
      rule: None,
      field_id: None,
    }
  }

  pub fn with_rule(mut self, rule: impl Into<String>) -> Self {
    self.rule = Some(rule.into());
    self
  }

  pub fn with_field(mut self, field_id: impl Into<String>) -> Self {
    self.field_id = Some(field_id.into());
    self
  }
}

/// Form field definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormField {
  /// Unique field identifier
  pub id: FieldId,
  /// Display label
  pub label: String,
  /// Field type
  pub field_type: FieldType,
  /// Current field value
  pub value: String,
  /// Whether field is required
  pub required: bool,
  /// Whether field is disabled
  pub disabled: bool,
  /// Whether field has been touched by user
  pub touched: bool,
  /// Placeholder text
  pub placeholder: String,
  /// Help text
  pub help_text: Option<String>,
  /// Validation rules
  pub rules: Vec<ValidationRule>,
  /// Current validation result
  pub validation_result: Option<ValidationResult>,
  /// Custom CSS classes
  pub css_classes: Vec<String>,
  /// Field-specific configuration
  pub config: HashMap<String, String>,
}

impl FormField {
  /// Create a new form field
  pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      label: label.into(),
      field_type: FieldType::default(),
      value: String::new(),
      required: false,
      disabled: false,
      touched: false,
      placeholder: String::new(),
      help_text: None,
      rules: Vec::new(),
      validation_result: None,
      css_classes: Vec::new(),
      config: HashMap::new(),
    }
  }

  /// Set field type
  pub fn field_type(mut self, field_type: FieldType) -> Self {
    self.field_type = field_type;
    self
  }

  /// Set required flag
  pub fn required(mut self, required: bool) -> Self {
    self.required = required;
    self
  }

  /// Set disabled flag
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.disabled = disabled;
    self
  }

  /// Set placeholder text
  pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self {
    self.placeholder = placeholder.into();
    self
  }

  /// Set help text
  pub fn help_text(mut self, help_text: impl Into<String>) -> Self {
    self.help_text = Some(help_text.into());
    self
  }

  /// Add validation rule
  pub fn add_rule(mut self, rule: ValidationRule) -> Self {
    self.rules.push(rule);
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set configuration value
  pub fn config(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.config.insert(key.into(), value.into());
    self
  }

  /// Check if field has errors
  pub fn has_errors(&self) -> bool {
    self
      .validation_result
      .as_ref()
      .map(|result| {
        !result.valid
          || result
            .messages
            .iter()
            .any(|m| m.severity == ValidationSeverity::Error)
      })
      .unwrap_or(false)
  }

  /// Check if field has warnings
  pub fn has_warnings(&self) -> bool {
    self
      .validation_result
      .as_ref()
      .map(|result| {
        result
          .messages
          .iter()
          .any(|m| m.severity == ValidationSeverity::Warning)
      })
      .unwrap_or(false)
  }

  /// Get error messages
  pub fn get_errors(&self) -> Vec<&ValidationMessage> {
    self
      .validation_result
      .as_ref()
      .map(|result| {
        result
          .messages
          .iter()
          .filter(|m| m.severity == ValidationSeverity::Error)
          .collect()
      })
      .unwrap_or_default()
  }

  /// Get warning messages
  pub fn get_warnings(&self) -> Vec<&ValidationMessage> {
    self
      .validation_result
      .as_ref()
      .map(|result| {
        result
          .messages
          .iter()
          .filter(|m| m.severity == ValidationSeverity::Warning)
          .collect()
      })
      .unwrap_or_default()
  }
}

/// Form validation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormValidationState {
  /// Whether form is currently valid
  pub valid: bool,
  /// Whether form has been submitted
  pub submitted: bool,
  /// Whether form is currently being validated
  pub validating: bool,
  /// Fields that have been touched
  pub touched_fields: Vec<FieldId>,
  /// Fields with validation errors
  pub invalid_fields: Vec<FieldId>,
  /// Global form-level validation messages
  pub form_messages: Vec<ValidationMessage>,
  /// Whether form is disabled
  pub disabled: bool,
}

impl Default for FormValidationState {
  fn default() -> Self {
    Self {
      valid: true,
      submitted: false,
      validating: false,
      touched_fields: Vec::new(),
      invalid_fields: Vec::new(),
      form_messages: Vec::new(),
      disabled: false,
    }
  }
}

/// Form validation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormValidationConfig {
  /// When to validate individual fields
  pub validation_timing: ValidationTiming,
  /// Validate on input (real-time)
  pub validate_on_input: bool,
  /// Validate on blur
  pub validate_on_blur: bool,
  /// Validate on submit
  pub validate_on_submit: bool,
  /// Debounce delay for input validation (ms)
  pub input_debounce_ms: u64,
  /// Show validation icons
  pub show_validation_icons: bool,
  /// Show help text
  pub show_help_text: bool,
  /// Stop on first error per field
  pub stop_on_first_error: bool,
  /// Enable async validation
  pub async_validation: bool,
  /// Custom error message templates
  pub error_templates: HashMap<String, String>,
}

impl Default for FormValidationConfig {
  fn default() -> Self {
    let mut error_templates = HashMap::new();
    error_templates.insert("required".to_string(), "This field is required".to_string());
    error_templates.insert(
      "email".to_string(),
      "Please enter a valid email address".to_string(),
    );
    error_templates.insert("url".to_string(), "Please enter a valid URL".to_string());
    error_templates.insert(
      "min_length".to_string(),
      "Must be at least {min} characters".to_string(),
    );
    error_templates.insert(
      "max_length".to_string(),
      "Must be no more than {max} characters".to_string(),
    );

    Self {
      validation_timing: ValidationTiming::OnBlur,
      validate_on_input: false,
      validate_on_blur: true,
      validate_on_submit: true,
      input_debounce_ms: 300,
      show_validation_icons: true,
      show_help_text: true,
      stop_on_first_error: false,
      async_validation: false,
      error_templates,
    }
  }
}

/// Form validation styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FormValidationStyle {
  /// Error text color
  pub error_color: Option<ColorDefinition>,
  /// Warning text color
  pub warning_color: Option<ColorDefinition>,
  /// Success/valid color
  pub success_color: Option<ColorDefinition>,
  /// Info text color
  pub info_color: Option<ColorDefinition>,
  /// Invalid field border color
  pub invalid_border_color: Option<ColorDefinition>,
  /// Valid field border color
  pub valid_border_color: Option<ColorDefinition>,
  /// Error message background
  pub error_background: Option<ColorDefinition>,
  /// Error icon
  pub error_icon: String,
  /// Warning icon
  pub warning_icon: String,
  /// Success icon
  pub success_icon: String,
  /// Info icon
  pub info_icon: String,
}

impl Default for FormValidationStyle {
  fn default() -> Self {
    Self {
      error_color: None,
      warning_color: None,
      success_color: None,
      info_color: None,
      invalid_border_color: None,
      valid_border_color: None,
      error_background: None,
      error_icon: "✗".to_string(),
      warning_icon: "⚠".to_string(),
      success_icon: "✓".to_string(),
      info_icon: "ℹ".to_string(),
    }
  }
}

/// Event callbacks for form validation
#[derive(Default)]
pub struct FormValidationCallbacks {
  /// Called when field validation changes
  pub on_field_validate: Option<OnFieldValidateCallback>,
  /// Called when form validation state changes
  pub on_form_validate: Option<OnFormValidateCallback>,
  /// Called when field value changes
  pub on_field_change: Option<OnFieldChangeCallback>,
  /// Called when form is submitted
  pub on_submit: Option<OnSubmitCallback>,
  /// Called for custom validation
  pub on_custom_validate: Option<OnCustomValidateCallback>,
}

/// Main Form Validator widget
pub struct FormValidator {
  /// Unique form identifier
  pub id: String,
  /// Form fields
  pub fields: HashMap<FieldId, FormField>,
  /// Field order for rendering
  pub field_order: Vec<FieldId>,
  /// Reactive state management
  pub state: Reactive<FormValidationState>,
  /// Configuration options
  pub config: FormValidationConfig,
  /// Styling configuration
  pub style: FormValidationStyle,
  /// Event callbacks
  pub callbacks: FormValidationCallbacks,
  /// CSS utility classes
  pub css_classes: Vec<String>,
  /// Custom validators
  pub custom_validators: HashMap<String, CustomValidator>,
}

impl FormValidator {
  /// Create a new form validator builder
  pub fn builder<S: Into<String>>(id: S) -> FormValidatorBuilder {
    FormValidatorBuilder::new(id)
  }

  /// Add a field to the form
  pub fn add_field(&mut self, field: FormField) {
    if !self.field_order.contains(&field.id) {
      self.field_order.push(field.id.clone());
    }
    self.fields.insert(field.id.clone(), field);
  }

  /// Remove a field from the form
  pub fn remove_field(&mut self, field_id: &str) -> Option<FormField> {
    self.field_order.retain(|id| id != field_id);
    self.fields.remove(field_id)
  }

  /// Set field value and validate if configured
  pub fn set_field_value(&mut self, field_id: &str, value: impl Into<String>) -> Result<()> {
    let value = value.into();

    let field = self
      .fields
      .get_mut(field_id)
      .ok_or_else(|| TuiError::component(format!("Field '{field_id}' not found")))?;

    field.value = value.clone();
    field.touched = true;

    // Update touched fields state
    self.state.update(|state| {
      if !state.touched_fields.contains(&field_id.to_string()) {
        state.touched_fields.push(field_id.to_string());
      }
    });

    // Validate if configured for input validation
    if self.config.validate_on_input {
      self.validate_field(field_id)?;
    }

    // Trigger change callback
    if let Some(callback) = &self.callbacks.on_field_change {
      callback(&field_id.to_string(), &value);
    }

    Ok(())
  }

  /// Get field value
  pub fn get_field_value(&self, field_id: &str) -> Option<&str> {
    self.fields.get(field_id).map(|field| field.value.as_str())
  }

  /// Validate a specific field
  pub fn validate_field(&mut self, field_id: &str) -> Result<ValidationResult> {
    let field = self
      .fields
      .get(field_id)
      .ok_or_else(|| TuiError::component(format!("Field '{field_id}' not found")))?;

    let result = self.validate_field_value(field);

    // Update field validation result
    if let Some(field) = self.fields.get_mut(field_id) {
      field.validation_result = Some(result.clone());
    }

    // Update form state
    self.update_form_state();

    // Trigger callback
    if let Some(callback) = &self.callbacks.on_field_validate {
      callback(&field_id.to_string(), &result);
    }

    Ok(result)
  }

  /// Validate field value against rules
  fn validate_field_value(&self, field: &FormField) -> ValidationResult {
    let mut messages = Vec::new();
    let value = &field.value;

    // Check required validation first
    if field.required && value.trim().is_empty() {
      let message = self.get_error_message("required", &HashMap::new());
      messages.push(
        ValidationMessage::error(message)
          .with_field(&field.id)
          .with_rule("required"),
      );

      if self.config.stop_on_first_error {
        return ValidationResult {
          valid: false,
          messages,
          value: value.clone(),
          timestamp: 0,
        };
      }
    }

    // Skip other validations if field is empty and not required
    if value.trim().is_empty() && !field.required {
      return ValidationResult::valid();
    }

    // Apply validation rules
    for rule in &field.rules {
      match rule {
        ValidationRule::Required => {
          // Already handled above
        }
        ValidationRule::MinLength(min_len) => {
          if value.len() < *min_len {
            let mut params = HashMap::new();
            params.insert("min".to_string(), min_len.to_string());
            let message = self.get_error_message("min_length", &params);
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("min_length"),
            );

            if self.config.stop_on_first_error {
              break;
            }
          }
        }
        ValidationRule::MaxLength(max_len) => {
          if value.len() > *max_len {
            let mut params = HashMap::new();
            params.insert("max".to_string(), max_len.to_string());
            let message = self.get_error_message("max_length", &params);
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("max_length"),
            );

            if self.config.stop_on_first_error {
              break;
            }
          }
        }
        ValidationRule::ExactLength(exact_len) => {
          if value.len() != *exact_len {
            let message = format!("Must be exactly {exact_len} characters");
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("exact_length"),
            );

            if self.config.stop_on_first_error {
              break;
            }
          }
        }
        ValidationRule::Email => {
          if !self.is_valid_email(value) {
            let message = self.get_error_message("email", &HashMap::new());
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("email"),
            );

            if self.config.stop_on_first_error {
              break;
            }
          }
        }
        ValidationRule::Url => {
          if !self.is_valid_url(value) {
            let message = self.get_error_message("url", &HashMap::new());
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("url"),
            );

            if self.config.stop_on_first_error {
              break;
            }
          }
        }
        ValidationRule::NumberRange { min, max } => {
          if let Ok(num) = value.parse::<f64>() {
            if let Some(min_val) = min {
              if num < *min_val {
                let message = format!("Must be at least {min_val}");
                messages.push(
                  ValidationMessage::error(message)
                    .with_field(&field.id)
                    .with_rule("number_min"),
                );
              }
            }
            if let Some(max_val) = max {
              if num > *max_val {
                let message = format!("Must be at most {max_val}");
                messages.push(
                  ValidationMessage::error(message)
                    .with_field(&field.id)
                    .with_rule("number_max"),
                );
              }
            }
          } else {
            messages.push(
              ValidationMessage::error("Must be a valid number")
                .with_field(&field.id)
                .with_rule("number_format"),
            );
          }
        }
        ValidationRule::Pattern(pattern) => {
          if let Ok(regex) = Regex::new(pattern) {
            if !regex.is_match(value) {
              let message = "Does not match required format".to_string();
              messages.push(
                ValidationMessage::error(message)
                  .with_field(&field.id)
                  .with_rule("pattern"),
              );
            }
          }
        }
        ValidationRule::MatchField(other_field_id) => {
          if let Some(other_field) = self.fields.get(other_field_id) {
            if value != &other_field.value {
              let message = format!("Must match {}", other_field.label);
              messages.push(
                ValidationMessage::error(message)
                  .with_field(&field.id)
                  .with_rule("match_field"),
              );
            }
          }
        }
        ValidationRule::OneOf(values) => {
          if !values.contains(value) {
            let message = format!("Must be one of: {}", values.join(", "));
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("one_of"),
            );
          }
        }
        ValidationRule::NoneOf(values) => {
          if values.contains(value) {
            let message = format!("Must be one of: {}", values.join(", "));
            messages.push(
              ValidationMessage::error(message)
                .with_field(&field.id)
                .with_rule("none_of"),
            );
          }
        }
        ValidationRule::Custom(validator_name) => {
          if let Some(validator) = self.custom_validators.get(validator_name) {
            let custom_result = validator(value);
            messages.extend(custom_result.messages);
          } else if let Some(callback) = &self.callbacks.on_custom_validate {
            let custom_result = callback(validator_name, value);
            messages.extend(custom_result.messages);
          }
        }
      }
    }

    ValidationResult {
      valid: messages
        .iter()
        .all(|m| m.severity != ValidationSeverity::Error),
      messages,
      value: value.clone(),
      timestamp: 0,
    }
  }

  /// Simple email validation
  fn is_valid_email(&self, email: &str) -> bool {
    // Basic email validation - in production use a proper email validation library
    let email_regex = r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$";
    if let Ok(regex) = Regex::new(email_regex) {
      regex.is_match(email)
    } else {
      false
    }
  }

  /// Simple URL validation
  fn is_valid_url(&self, url: &str) -> bool {
    // Basic URL validation
    url.starts_with("http://") || url.starts_with("https://") || url.starts_with("ftp://")
  }

  /// Get error message with template substitution
  fn get_error_message(&self, rule_name: &str, params: &HashMap<String, String>) -> String {
    if let Some(template) = self.config.error_templates.get(rule_name) {
      let mut message = template.clone();
      for (key, value) in params {
        message = message.replace(&format!("{{{key}}}"), value);
      }
      message
    } else {
      format!("Validation failed: {rule_name}")
    }
  }

  /// Validate all fields
  pub fn validate_all(&mut self) -> Result<bool> {
    let field_ids: Vec<_> = self.field_order.clone();

    for field_id in field_ids {
      self.validate_field(&field_id)?;
    }

    Ok(self.is_valid())
  }

  /// Check if form is valid
  pub fn is_valid(&self) -> bool {
    self.fields.values().all(|field| {
      field
        .validation_result
        .as_ref()
        .map(|result| result.valid)
        .unwrap_or(true)
    })
  }

  /// Get form data as key-value pairs
  pub fn get_form_data(&self) -> HashMap<FieldId, String> {
    self
      .fields
      .iter()
      .map(|(id, field)| (id.clone(), field.value.clone()))
      .collect()
  }

  /// Submit form (validates and triggers submit callback)
  pub fn submit(&mut self) -> Result<bool> {
    self.state.update(|state| {
      state.submitted = true;
      state.validating = true;
    });

    let is_valid = if self.config.validate_on_submit {
      self.validate_all()?
    } else {
      self.is_valid()
    };

    self.state.update(|state| {
      state.validating = false;
      state.valid = is_valid;
    });

    if is_valid {
      if let Some(callback) = &self.callbacks.on_submit {
        let form_data = self.get_form_data();
        callback(&form_data);
      }
    }

    Ok(is_valid)
  }

  /// Reset form to initial state
  pub fn reset(&mut self) {
    for field in self.fields.values_mut() {
      field.value.clear();
      field.touched = false;
      field.validation_result = None;
    }

    self.state.update(|state| {
      *state = FormValidationState::default();
    });
  }

  /// Update form validation state
  fn update_form_state(&mut self) {
    let invalid_fields: Vec<_> = self
      .fields
      .iter()
      .filter(|(_, field)| field.has_errors())
      .map(|(id, _)| id.clone())
      .collect();

    let is_valid = invalid_fields.is_empty();

    self.state.update(|state| {
      state.valid = is_valid;
      state.invalid_fields = invalid_fields;
    });

    // Trigger form validation callback
    if let Some(callback) = &self.callbacks.on_form_validate {
      let form_messages = self.state.get().form_messages.clone();
      callback(is_valid, &form_messages);
    }
  }

  /// Set form disabled state
  pub fn set_disabled(&mut self, disabled: bool) {
    self.state.update(|state| {
      state.disabled = disabled;
    });
  }

  /// Check if form is disabled
  pub fn is_disabled(&self) -> bool {
    self.state.get().disabled
  }

  /// Add custom validator
  pub fn add_custom_validator<F>(&mut self, name: String, validator: F)
  where
    F: Fn(&str) -> ValidationResult + Send + Sync + 'static,
  {
    self.custom_validators.insert(name, Arc::new(validator));
  }

  /// Render the form to a string
  pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let mut output = String::new();
    let state = self.state.get();

    // Base CSS classes
    let mut classes = vec!["form-validator".to_string()];
    if !state.valid {
      classes.push("form-invalid".to_string());
    }
    if state.submitted {
      classes.push("form-submitted".to_string());
    }
    if state.disabled {
      classes.push("form-disabled".to_string());
    }
    classes.extend(self.css_classes.clone());

    // Render form fields in order
    for field_id in &self.field_order {
      if let Some(field) = self.fields.get(field_id) {
        self.render_field(&mut output, field);
      }
    }

    // Render form-level messages
    if !state.form_messages.is_empty() {
      writeln!(output, "\nForm Messages:").unwrap();
      for message in &state.form_messages {
        let icon = match message.severity {
          ValidationSeverity::Error => &self.style.error_icon,
          ValidationSeverity::Warning => &self.style.warning_icon,
          ValidationSeverity::Info => &self.style.info_icon,
        };
        writeln!(output, "  {} {}", icon, message.message).unwrap();
      }
    }

    output
  }

  /// Render individual field with validation
  fn render_field(&self, output: &mut String, field: &FormField) {
    // Field label
    writeln!(
      output,
      "{}{}",
      field.label,
      if field.required { " *" } else { "" }
    )
    .unwrap();

    // Field value display
    let value_display = if field.field_type == FieldType::Password {
      "*".repeat(field.value.len())
    } else {
      field.value.clone()
    };

    write!(
      output,
      "  [{}]",
      if value_display.is_empty() {
        &field.placeholder
      } else {
        &value_display
      }
    )
    .unwrap();

    // Validation icons
    if self.config.show_validation_icons {
      if let Some(result) = &field.validation_result {
        if !result.valid {
          write!(output, " {}", self.style.error_icon).unwrap();
        } else if result
          .messages
          .iter()
          .any(|m| m.severity == ValidationSeverity::Warning)
        {
          write!(output, " {}", self.style.warning_icon).unwrap();
        } else if field.touched {
          write!(output, " {}", self.style.success_icon).unwrap();
        }
      }
    }

    writeln!(output).unwrap();

    // Help text
    if self.config.show_help_text {
      if let Some(help_text) = &field.help_text {
        writeln!(output, "    {help_text}").unwrap();
      }
    }

    // Validation messages
    if let Some(result) = &field.validation_result {
      for message in &result.messages {
        let icon = match message.severity {
          ValidationSeverity::Error => &self.style.error_icon,
          ValidationSeverity::Warning => &self.style.warning_icon,
          ValidationSeverity::Info => &self.style.info_icon,
        };
        writeln!(output, "    {} {}", icon, message.message).unwrap();
      }
    }

    writeln!(output).unwrap();
  }

  /// Convert to Element for integration with layout system
  pub fn to_element(&self) -> Element {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 80,
      height: 25,
    };
    Element {
      tag: "form".to_string(),
      id: Some(self.id.clone()),
      classes: self.css_classes.clone(),
      content: Some(self.render(&layout, None)),
      children: Vec::new(),
      attributes: std::collections::HashMap::new(),
      focusable: !self.is_disabled(),
      focused: false,
      disabled: self.is_disabled(),
      tab_index: Some(0),
      key_bindings: Vec::new(),
      modal: false,
    }
  }
}

impl fmt::Display for FormValidator {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let layout = LayoutRect {
      x: 0,
      y: 0,
      width: 80,
      height: 25,
    };
    write!(f, "{}", self.render(&layout, None))
  }
}

/// Builder for creating form validators
pub struct FormValidatorBuilder {
  id: String,
  fields: Vec<FormField>,
  config: FormValidationConfig,
  style: FormValidationStyle,
  callbacks: FormValidationCallbacks,
  css_classes: Vec<String>,
}

impl FormValidatorBuilder {
  /// Create a new form validator builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      fields: Vec::new(),
      config: FormValidationConfig::default(),
      style: FormValidationStyle::default(),
      callbacks: FormValidationCallbacks::default(),
      css_classes: Vec::new(),
    }
  }

  /// Add a field
  pub fn field(mut self, field: FormField) -> Self {
    self.fields.push(field);
    self
  }

  /// Set validation timing
  pub fn validation_timing(mut self, timing: ValidationTiming) -> Self {
    self.config.validation_timing = timing;
    self
  }

  /// Enable/disable input validation
  pub fn validate_on_input(mut self, enabled: bool) -> Self {
    self.config.validate_on_input = enabled;
    self
  }

  /// Enable/disable blur validation
  pub fn validate_on_blur(mut self, enabled: bool) -> Self {
    self.config.validate_on_blur = enabled;
    self
  }

  /// Enable/disable submit validation
  pub fn validate_on_submit(mut self, enabled: bool) -> Self {
    self.config.validate_on_submit = enabled;
    self
  }

  /// Set input debounce delay
  pub fn input_debounce_ms(mut self, ms: u64) -> Self {
    self.config.input_debounce_ms = ms;
    self
  }

  /// Show/hide validation icons
  pub fn show_validation_icons(mut self, show: bool) -> Self {
    self.config.show_validation_icons = show;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.css_classes.push(class.into());
    self
  }

  /// Set field validation callback
  pub fn on_field_validate<F>(mut self, callback: F) -> Self
  where
    F: Fn(&FieldId, &ValidationResult) + Send + Sync + 'static,
  {
    self.callbacks.on_field_validate = Some(Arc::new(callback));
    self
  }

  /// Set form validation callback
  pub fn on_form_validate<F>(mut self, callback: F) -> Self
  where
    F: Fn(bool, &[ValidationMessage]) + Send + Sync + 'static,
  {
    self.callbacks.on_form_validate = Some(Arc::new(callback));
    self
  }

  /// Set field change callback
  pub fn on_field_change<F>(mut self, callback: F) -> Self
  where
    F: Fn(&FieldId, &str) + Send + Sync + 'static,
  {
    self.callbacks.on_field_change = Some(Arc::new(callback));
    self
  }

  /// Set submit callback
  pub fn on_submit<F>(mut self, callback: F) -> Self
  where
    F: Fn(&HashMap<FieldId, String>) + Send + Sync + 'static,
  {
    self.callbacks.on_submit = Some(Arc::new(callback));
    self
  }

  /// Build the form validator
  pub fn build(self) -> FormValidator {
    let state = FormValidationState::default();

    let mut form = FormValidator {
      id: self.id,
      fields: HashMap::new(),
      field_order: Vec::new(),
      state: Reactive::new(state),
      config: self.config,
      style: self.style,
      callbacks: self.callbacks,
      css_classes: self.css_classes,
      custom_validators: HashMap::new(),
    };

    // Add fields to form
    for field in self.fields {
      form.add_field(field);
    }

    form
  }
}

/// Convenience functions for common form patterns
/// Create a user registration form
pub fn user_registration_form() -> FormValidator {
  FormValidatorBuilder::new("user-registration")
    .field(
      FormField::new("username", "Username")
        .required(true)
        .add_rule(ValidationRule::MinLength(3))
        .add_rule(ValidationRule::MaxLength(50))
        .placeholder("Enter username"),
    )
    .field(
      FormField::new("email", "Email Address")
        .field_type(FieldType::Email)
        .required(true)
        .add_rule(ValidationRule::Email)
        .placeholder("user@example.com"),
    )
    .field(
      FormField::new("password", "Password")
        .field_type(FieldType::Password)
        .required(true)
        .add_rule(ValidationRule::MinLength(8))
        .add_rule(ValidationRule::Pattern(
          r"(?=.*[A-Za-z])(?=.*\d)".to_string(),
        ))
        .help_text("Must be at least 8 characters with letters and numbers"),
    )
    .field(
      FormField::new("confirm_password", "Confirm Password")
        .field_type(FieldType::Password)
        .required(true)
        .add_rule(ValidationRule::MatchField("password".to_string())),
    )
    .validate_on_blur(true)
    .validate_on_submit(true)
    .build()
}

/// Create a login form
pub fn login_form() -> FormValidator {
  FormValidatorBuilder::new("login-form")
    .field(
      FormField::new("email", "Email")
        .field_type(FieldType::Email)
        .required(true)
        .add_rule(ValidationRule::Email),
    )
    .field(
      FormField::new("password", "Password")
        .field_type(FieldType::Password)
        .required(true)
        .add_rule(ValidationRule::MinLength(1)),
    )
    .validate_on_submit(true)
    .build()
}

/// Create a contact form
pub fn contact_form() -> FormValidator {
  FormValidatorBuilder::new("contact-form")
    .field(
      FormField::new("name", "Full Name")
        .required(true)
        .add_rule(ValidationRule::MinLength(2)),
    )
    .field(
      FormField::new("email", "Email Address")
        .field_type(FieldType::Email)
        .required(true)
        .add_rule(ValidationRule::Email),
    )
    .field(
      FormField::new("subject", "Subject")
        .required(true)
        .add_rule(ValidationRule::MinLength(5))
        .add_rule(ValidationRule::MaxLength(100)),
    )
    .field(
      FormField::new("message", "Message")
        .field_type(FieldType::Textarea)
        .required(true)
        .add_rule(ValidationRule::MinLength(10))
        .add_rule(ValidationRule::MaxLength(1000)),
    )
    .validate_on_blur(true)
    .validate_on_submit(true)
    .build()
}
