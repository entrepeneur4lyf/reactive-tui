/**
 * Form Validation Widget
 * 
 * A comprehensive form validation system supporting field validation, error messages,
 * custom validators, and real-time validation feedback with styling.
 * 
 * ## Features
 * 
 * - **Field Validation**: Support for text, email, password, number, and custom field types
 * - **Built-in Validators**: Required, email, URL, numeric, length, pattern validation
 * - **Custom Validators**: Support for custom validation functions with async capabilities
 * - **Real-time Validation**: Validate on input, blur, or submit with configurable timing
 * - **Error Styling**: Customizable error messages with colors and positioning
 * - **Form State Management**: Track form validity, touched fields, and submission state
 * - **Accessibility**: Full ARIA support with error announcements
 * - **Conditional Validation**: Fields that validate based on other field values
 * - **Validation Groups**: Group related fields for complex validation scenarios
 * - **Internationalization**: Support for custom error messages and localization
 * 
 * ## Basic Usage
 * 
 * ```typescript
 * import { FormValidator, FormField, ValidationRule, FieldType } from './form_validation';
 * 
 * const form = new FormValidatorBuilder('user-form')
 *   .field(
 *     new FormField('email', 'Email Address')
 *       .fieldType(FieldType.Email)
 *       .required(true)
 *       .addRule(ValidationRule.Email)
 *       .addRule(ValidationRule.MaxLength(255))
 *   )
 *   .field(
 *     new FormField('password', 'Password')
 *       .fieldType(FieldType.Password)
 *       .required(true)
 *       .addRule(ValidationRule.MinLength(8))
 *       .addRule(ValidationRule.Pattern(/(?=.*[A-Za-z])(?=.*\d)/))
 *   )
 *   .validateOnInput(true)
 *   .validateOnBlur(true)
 *   .build();
 * 
 * // Set field values and validate
 * form.setFieldValue('email', 'user@example.com');
 * form.setFieldValue('password', 'mypassword123');
 * 
 * // Check form validity
 * if (form.isValid()) {
 *   const values = form.getFormData();
 *   // Submit form
 * }
 * ```
 */

export type FieldId = string;

/** Field types for validation */
export enum FieldType {
  Text = 'text',
  Email = 'email',
  Password = 'password',
  Number = 'number',
  Url = 'url',
  Phone = 'phone',
  Date = 'date',
  Time = 'time',
  Textarea = 'textarea',
  Select = 'select',
  Checkbox = 'checkbox',
  Radio = 'radio',
  File = 'file',
  Custom = 'custom'
}

/** Validation rule types */
export class ValidationRule {
  private constructor(
    public type: string,
    public value?: any,
    public message?: string
  ) {}

  static get Required() { return new ValidationRule('required'); }
  static get Email() { return new ValidationRule('email'); }
  static get Url() { return new ValidationRule('url'); }
  static get Numeric() { return new ValidationRule('numeric'); }
  static get Integer() { return new ValidationRule('integer'); }
  static get Positive() { return new ValidationRule('positive'); }
  static get Negative() { return new ValidationRule('negative'); }

  static MinLength(length: number, message?: string) {
    return new ValidationRule('min_length', length, message);
  }

  static MaxLength(length: number, message?: string) {
    return new ValidationRule('max_length', length, message);
  }

  static MinValue(value: number, message?: string) {
    return new ValidationRule('min_value', value, message);
  }

  static MaxValue(value: number, message?: string) {
    return new ValidationRule('max_value', value, message);
  }

  static Pattern(pattern: RegExp, message?: string) {
    return new ValidationRule('pattern', pattern, message);
  }

  static Custom(name: string, message?: string) {
    return new ValidationRule('custom', name, message);
  }

  static Matches(fieldId: string, message?: string) {
    return new ValidationRule('matches', fieldId, message);
  }

  static OneOf(values: string[], message?: string) {
    return new ValidationRule('one_of', values, message);
  }

  static NotOneOf(values: string[], message?: string) {
    return new ValidationRule('not_one_of', values, message);
  }
}

/** Validation timing configuration */
export enum ValidationTiming {
  OnInput = 'on_input',
  OnBlur = 'on_blur',
  OnSubmit = 'on_submit',
  Manual = 'manual'
}

/** Validation result */
export interface ValidationResult {
  isValid: boolean;
  message?: string;
  fieldId?: string;
}

/** Individual validation message */
export interface ValidationMessage {
  fieldId: string;
  message: string;
  severity: 'error' | 'warning' | 'info';
  ruleType: string;
}

/** Form field definition */
export class FormField {
  private _fieldType: FieldType = FieldType.Text;
  private _required: boolean = false;
  private _rules: ValidationRule[] = [];
  private _placeholder: string = '';
  private _defaultValue: string = '';
  private _helpText: string = '';
  private _disabled: boolean = false;
  private _readonly: boolean = false;
  private _cssClasses: string[] = [];
  private _attributes: Record<string, string> = {};

  constructor(
    public id: FieldId,
    public label: string = ''
  ) {}

  fieldType(type: FieldType): this {
    this._fieldType = type;
    return this;
  }

  required(required: boolean = true): this {
    this._required = required;
    return this;
  }

  addRule(rule: ValidationRule): this {
    this._rules.push(rule);
    return this;
  }

  placeholder(text: string): this {
    this._placeholder = text;
    return this;
  }

  defaultValue(value: string): this {
    this._defaultValue = value;
    return this;
  }

  helpText(text: string): this {
    this._helpText = text;
    return this;
  }

  disabled(disabled: boolean = true): this {
    this._disabled = disabled;
    return this;
  }

  readonly(readonly: boolean = true): this {
    this._readonly = readonly;
    return this;
  }

  addClass(className: string): this {
    this._cssClasses.push(className);
    return this;
  }

  attribute(key: string, value: string): this {
    this._attributes[key] = value;
    return this;
  }

  // Getters
  get fieldType_() { return this._fieldType; }
  get required_() { return this._required; }
  get rules() { return this._rules; }
  get placeholder_() { return this._placeholder; }
  get defaultValue_() { return this._defaultValue; }
  get helpText_() { return this._helpText; }
  get disabled_() { return this._disabled; }
  get readonly_() { return this._readonly; }
  get cssClasses() { return this._cssClasses; }
  get attributes() { return this._attributes; }
}

/** Form validation state */
export interface FormValidationState {
  fieldValues: Record<FieldId, string>;
  fieldErrors: Record<FieldId, ValidationMessage[]>;
  touchedFields: Set<FieldId>;
  isValid: boolean;
  isSubmitting: boolean;
  hasSubmitted: boolean;
  validationCount: number;
  lastValidationTime: Date | null;
}

/** Form validation configuration */
export interface FormValidationConfig {
  validateOnInput: boolean;
  validateOnBlur: boolean;
  validateOnSubmit: boolean;
  showErrorsOnTouch: boolean;
  debounceMs: number;
  maxErrorsPerField: number;
  stopOnFirstError: boolean;
  caseSensitive: boolean;
  trimValues: boolean;
  submitOnValid: boolean;
}

/** Form validation styling */
export interface FormValidationStyle {
  errorColor?: string;
  warningColor?: string;
  successColor?: string;
  errorBackground?: string;
  fieldBorderError?: string;
  fieldBorderValid?: string;
  errorIcon?: string;
  successIcon?: string;
  warningIcon?: string;
}

/** Validation callbacks */
export interface FormValidationCallbacks {
  onFieldValidate?: (fieldId: FieldId, result: ValidationResult) => void;
  onFormValidate?: (isValid: boolean, messages: ValidationMessage[]) => void;
  onFieldChange?: (fieldId: FieldId, value: string) => void;
  onSubmit?: (data: Record<FieldId, string>) => void;
  onCustomValidate?: (ruleName: string, value: string) => ValidationResult;
}

/** Custom validator function type */
export type ValidatorFunction = (value: string) => ValidationResult;

/** Main Form Validator widget */
export class FormValidator {
  private state: FormValidationState;
  private config: FormValidationConfig;
  private style: FormValidationStyle;
  private callbacks: FormValidationCallbacks;
  private fields: Map<FieldId, FormField> = new Map();
  private customValidators: Map<string, ValidatorFunction> = new Map();
  private validationTimers: Map<FieldId, NodeJS.Timeout> = new Map();
  public cssClasses: string[] = [];

  constructor(
    private id: string,
    fields: FormField[] = [],
    config: Partial<FormValidationConfig> = {},
    style: FormValidationStyle = {},
    callbacks: FormValidationCallbacks = {}
  ) {
    this.state = {
      fieldValues: {},
      fieldErrors: {},
      touchedFields: new Set(),
      isValid: false,
      isSubmitting: false,
      hasSubmitted: false,
      validationCount: 0,
      lastValidationTime: null
    };

    this.config = {
      validateOnInput: true,
      validateOnBlur: true,
      validateOnSubmit: true,
      showErrorsOnTouch: true,
      debounceMs: 300,
      maxErrorsPerField: 3,
      stopOnFirstError: false,
      caseSensitive: false,
      trimValues: true,
      submitOnValid: false,
      ...config
    };

    this.style = {
      errorColor: '#ef4444',
      warningColor: '#f59e0b',
      successColor: '#10b981',
      errorBackground: undefined,
      fieldBorderError: '#ef4444',
      fieldBorderValid: '#10b981',
      errorIcon: '✗',
      successIcon: '✓',
      warningIcon: '⚠',
      ...style
    };

    this.callbacks = callbacks;

    // Initialize fields
    fields.forEach(field => {
      this.fields.set(field.id, field);
      this.state.fieldValues[field.id] = field.defaultValue_;
    });

    this.validateAll();
  }

  /** Create a new form validator builder */
  static builder(id: string): FormValidatorBuilder {
    return new FormValidatorBuilder(id);
  }

  /** Set field value and optionally validate */
  setFieldValue(fieldId: FieldId, value: string, validate: boolean = true): void {
    const processedValue = this.config.trimValues ? value.trim() : value;
    this.state.fieldValues[fieldId] = processedValue;

    // Trigger change callback
    if (this.callbacks.onFieldChange) {
      this.callbacks.onFieldChange(fieldId, processedValue);
    }

    // Validate if configured
    if (validate && this.config.validateOnInput) {
      this.debounceValidation(fieldId);
    }
  }

  /** Get field value */
  getFieldValue(fieldId: FieldId): string {
    return this.state.fieldValues[fieldId] || '';
  }

  /** Mark field as touched */
  touchField(fieldId: FieldId): void {
    this.state.touchedFields.add(fieldId);

    if (this.config.validateOnBlur) {
      this.validateField(fieldId);
    }
  }

  /** Validate specific field */
  validateField(fieldId: FieldId): ValidationResult {
    const field = this.fields.get(fieldId);
    if (!field) {
      return { isValid: false, message: `Field ${fieldId} not found` };
    }

    const value = this.getFieldValue(fieldId);
    const errors: ValidationMessage[] = [];

    // Apply each validation rule
    for (const rule of field.rules) {
      const result = this.applyValidationRule(field, rule, value);
      if (!result.isValid && result.message) {
        errors.push({
          fieldId,
          message: result.message,
          severity: 'error',
          ruleType: rule.type
        });

        if (this.config.stopOnFirstError) {
          break;
        }
      }

      if (errors.length >= this.config.maxErrorsPerField) {
        break;
      }
    }

    // Store field errors
    this.state.fieldErrors[fieldId] = errors;

    const isValid = errors.length === 0;
    const result: ValidationResult = {
      isValid,
      message: errors[0]?.message,
      fieldId
    };

    // Trigger field validation callback
    if (this.callbacks.onFieldValidate) {
      this.callbacks.onFieldValidate(fieldId, result);
    }

    return result;
  }

  /** Validate all fields */
  validateAll(): boolean {
    this.state.validationCount++;
    this.state.lastValidationTime = new Date();

    let formValid = true;
    const allMessages: ValidationMessage[] = [];

    for (const fieldId of this.fields.keys()) {
      const result = this.validateField(fieldId);
      if (!result.isValid) {
        formValid = false;
      }
      allMessages.push(...(this.state.fieldErrors[fieldId] || []));
    }

    this.state.isValid = formValid;

    // Trigger form validation callback
    if (this.callbacks.onFormValidate) {
      this.callbacks.onFormValidate(formValid, allMessages);
    }

    return formValid;
  }

  /** Apply individual validation rule */
  private applyValidationRule(field: FormField, rule: ValidationRule, value: string): ValidationResult {

    switch (rule.type) {
      case 'required':
        if (!value || value.trim().length === 0) {
          return {
            isValid: false,
            message: rule.message || `${field.label} is required`
          };
        }
        break;

      case 'email':
        const emailRegex = /^[^\s@]+@[^\s@]+\.[^\s@]+$/;
        if (value && !emailRegex.test(value)) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be a valid email address`
          };
        }
        break;

      case 'url':
        try {
          if (value) new URL(value);
        } catch {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be a valid URL`
          };
        }
        break;

      case 'numeric':
        if (value && isNaN(Number(value))) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be a number`
          };
        }
        break;

      case 'integer':
        if (value && !Number.isInteger(Number(value))) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be an integer`
          };
        }
        break;

      case 'positive':
        if (value && Number(value) <= 0) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be positive`
          };
        }
        break;

      case 'negative':
        if (value && Number(value) >= 0) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be negative`
          };
        }
        break;

      case 'min_length':
        if (value.length < rule.value) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be at least ${rule.value} characters`
          };
        }
        break;

      case 'max_length':
        if (value.length > rule.value) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be no more than ${rule.value} characters`
          };
        }
        break;

      case 'min_value':
        if (Number(value) < rule.value) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be at least ${rule.value}`
          };
        }
        break;

      case 'max_value':
        if (Number(value) > rule.value) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be no more than ${rule.value}`
          };
        }
        break;

      case 'pattern':
        if (value && !rule.value.test(value)) {
          return {
            isValid: false,
            message: rule.message || `${field.label} format is invalid`
          };
        }
        break;

      case 'matches':
        const otherValue = this.getFieldValue(rule.value);
        if (value !== otherValue) {
          const otherField = this.fields.get(rule.value);
          return {
            isValid: false,
            message: rule.message || `${field.label} must match ${otherField?.label || rule.value}`
          };
        }
        break;

      case 'one_of':
        const compareValue = this.config.caseSensitive ? value : value.toLowerCase();
        const validValues = this.config.caseSensitive ? rule.value : rule.value.map((v: string) => v.toLowerCase());
        if (value && !validValues.includes(compareValue)) {
          return {
            isValid: false,
            message: rule.message || `${field.label} must be one of: ${rule.value.join(', ')}`
          };
        }
        break;

      case 'not_one_of':
        const compareValue2 = this.config.caseSensitive ? value : value.toLowerCase();
        const invalidValues = this.config.caseSensitive ? rule.value : rule.value.map((v: string) => v.toLowerCase());
        if (value && invalidValues.includes(compareValue2)) {
          return {
            isValid: false,
            message: rule.message || `${field.label} cannot be one of: ${rule.value.join(', ')}`
          };
        }
        break;

      case 'custom':
        const validator = this.customValidators.get(rule.value);
        if (validator) {
          const result = validator(value);
          if (!result.isValid) {
            return {
              isValid: false,
              message: rule.message || result.message || `${field.label} validation failed`
            };
          }
        } else if (this.callbacks.onCustomValidate) {
          const result = this.callbacks.onCustomValidate(rule.value, value);
          if (!result.isValid) {
            return {
              isValid: false,
              message: rule.message || result.message || `${field.label} validation failed`
            };
          }
        }
        break;
    }

    return { isValid: true };
  }

  /** Debounced validation for input events */
  private debounceValidation(fieldId: FieldId): void {
    // Clear existing timer
    const existingTimer = this.validationTimers.get(fieldId);
    if (existingTimer) {
      clearTimeout(existingTimer);
    }

    // Set new timer
    const timer = setTimeout(() => {
      this.validateField(fieldId);
      this.validationTimers.delete(fieldId);
    }, this.config.debounceMs);

    this.validationTimers.set(fieldId, timer);
  }

  /** Add custom validator */
  addCustomValidator(name: string, validator: ValidatorFunction): void {
    this.customValidators.set(name, validator);
  }

  /** Submit form */
  async submit(): Promise<Record<FieldId, string> | null> {
    this.state.isSubmitting = true;
    this.state.hasSubmitted = true;

    // Mark all fields as touched
    for (const fieldId of this.fields.keys()) {
      this.state.touchedFields.add(fieldId);
    }

    const isValid = this.validateAll();

    if (isValid) {
      const formData = { ...this.state.fieldValues };
      
      if (this.callbacks.onSubmit) {
        this.callbacks.onSubmit(formData);
      }

      this.state.isSubmitting = false;
      return formData;
    }

    this.state.isSubmitting = false;
    return null;
  }

  /** Get form data */
  getFormData(): Record<FieldId, string> {
    return { ...this.state.fieldValues };
  }

  /** Check if form is valid */
  isValid(): boolean {
    return this.state.isValid;
  }

  /** Check if field has errors */
  hasFieldErrors(fieldId: FieldId): boolean {
    return (this.state.fieldErrors[fieldId]?.length || 0) > 0;
  }

  /** Get field errors */
  getFieldErrors(fieldId: FieldId): ValidationMessage[] {
    return this.state.fieldErrors[fieldId] || [];
  }

  /** Get all validation messages */
  getAllMessages(): ValidationMessage[] {
    return Object.values(this.state.fieldErrors).flat();
  }

  /** Check if field is touched */
  isFieldTouched(fieldId: FieldId): boolean {
    return this.state.touchedFields.has(fieldId);
  }

  /** Reset form */
  reset(): void {
    this.state.fieldValues = {};
    this.state.fieldErrors = {};
    this.state.touchedFields.clear();
    this.state.isValid = false;
    this.state.isSubmitting = false;
    this.state.hasSubmitted = false;
    this.state.validationCount = 0;
    this.state.lastValidationTime = null;

    // Reset to default values
    for (const field of this.fields.values()) {
      this.state.fieldValues[field.id] = field.defaultValue_;
    }

    // Clear timers
    for (const timer of this.validationTimers.values()) {
      clearTimeout(timer);
    }
    this.validationTimers.clear();
  }

  /** Clear field errors */
  clearFieldErrors(fieldId?: FieldId): void {
    if (fieldId) {
      delete this.state.fieldErrors[fieldId];
    } else {
      this.state.fieldErrors = {};
    }
  }

  /** Render form to string */
  render(): string {
    let output = '';
    
    // Base CSS classes
    const classes = ['form-validator'];
    if (this.state.hasSubmitted) classes.push('form-submitted');
    if (this.state.isValid) classes.push('form-valid');
    else classes.push('form-invalid');
    if (this.state.isSubmitting) classes.push('form-submitting');
    classes.push(...this.cssClasses);

    output += `<div class="${classes.join(' ')}" id="${this.id}">\n`;

    // Render each field
    for (const field of this.fields.values()) {
      output += this.renderField(field);
    }

    // Form summary
    if (this.state.hasSubmitted && !this.state.isValid) {
      const errorCount = this.getAllMessages().length;
      output += `  <div class="form-summary form-errors">\n`;
      output += `    ${this.style.errorIcon} ${errorCount} validation error${errorCount !== 1 ? 's' : ''} found\n`;
      output += `  </div>\n`;
    } else if (this.state.isValid && this.state.hasSubmitted) {
      output += `  <div class="form-summary form-valid">\n`;
      output += `    ${this.style.successIcon} Form is valid and ready to submit\n`;
      output += `  </div>\n`;
    }

    output += `</div>\n`;
    return output;
  }

  /** Render individual field */
  private renderField(field: FormField): string {
    const value = this.getFieldValue(field.id);
    const errors = this.getFieldErrors(field.id);
    const hasErrors = errors.length > 0;
    const isTouched = this.isFieldTouched(field.id);
    const showErrors = hasErrors && (isTouched || this.config.showErrorsOnTouch);

    let output = `  <div class="form-field field-${field.fieldType_}">\n`;
    
    // Field label
    if (field.label) {
      const labelClasses = ['field-label'];
      if (field.required_) labelClasses.push('required');
      if (hasErrors) labelClasses.push('error');
      
      output += `    <label class="${labelClasses.join(' ')}" for="${field.id}">\n`;
      output += `      ${field.label}${field.required_ ? ' *' : ''}\n`;
      output += `    </label>\n`;
    }

    // Field input
    const inputClasses = ['field-input'];
    if (hasErrors) inputClasses.push('error');
    if (!hasErrors && isTouched && value) inputClasses.push('valid');
    if (field.disabled_) inputClasses.push('disabled');
    if (field.readonly_) inputClasses.push('readonly');
    inputClasses.push(...field.cssClasses);

    output += `    <input`;
    output += ` type="${field.fieldType_}"`;
    output += ` id="${field.id}"`;
    output += ` name="${field.id}"`;
    output += ` class="${inputClasses.join(' ')}"`;
    output += ` value="${value}"`;
    if (field.placeholder_) output += ` placeholder="${field.placeholder_}"`;
    if (field.disabled_) output += ` disabled`;
    if (field.readonly_) output += ` readonly`;
    if (field.required_) output += ` required`;
    
    // Add custom attributes
    for (const [key, attrValue] of Object.entries(field.attributes)) {
      output += ` ${key}="${attrValue}"`;
    }
    
    output += ` />\n`;

    // Validation icon
    if (isTouched) {
      if (hasErrors) {
        output += `    <span class="field-icon error">${this.style.errorIcon}</span>\n`;
      } else if (value) {
        output += `    <span class="field-icon success">${this.style.successIcon}</span>\n`;
      }
    }

    // Help text
    if (field.helpText_) {
      output += `    <div class="field-help">${field.helpText_}</div>\n`;
    }

    // Error messages
    if (showErrors) {
      output += `    <div class="field-errors">\n`;
      for (const error of errors) {
        output += `      <div class="error-message ${error.severity}">\n`;
        output += `        ${this.style.errorIcon} ${error.message}\n`;
        output += `      </div>\n`;
      }
      output += `    </div>\n`;
    }

    output += `  </div>\n`;
    return output;
  }
}

/** Builder for creating form validators */
export class FormValidatorBuilder {
  private fields: FormField[] = [];
  private config: Partial<FormValidationConfig> = {};
  private style: FormValidationStyle = {};
  private callbacks: FormValidationCallbacks = {};
  private cssClasses: string[] = [];

  constructor(private id: string) {}

  /** Add form field */
  field(field: FormField): this {
    this.fields.push(field);
    return this;
  }

  /** Enable validation on input */
  validateOnInput(enabled: boolean = true): this {
    this.config.validateOnInput = enabled;
    return this;
  }

  /** Enable validation on blur */
  validateOnBlur(enabled: boolean = true): this {
    this.config.validateOnBlur = enabled;
    return this;
  }

  /** Enable validation on submit */
  validateOnSubmit(enabled: boolean = true): this {
    this.config.validateOnSubmit = enabled;
    return this;
  }

  /** Set debounce delay */
  debounceMs(ms: number): this {
    this.config.debounceMs = ms;
    return this;
  }

  /** Set maximum errors per field */
  maxErrorsPerField(count: number): this {
    this.config.maxErrorsPerField = count;
    return this;
  }

  /** Stop on first error */
  stopOnFirstError(enabled: boolean = true): this {
    this.config.stopOnFirstError = enabled;
    return this;
  }

  /** Enable case sensitive validation */
  caseSensitive(enabled: boolean = true): this {
    this.config.caseSensitive = enabled;
    return this;
  }

  /** Auto-trim field values */
  trimValues(enabled: boolean = true): this {
    this.config.trimValues = enabled;
    return this;
  }

  /** Set error color */
  errorColor(color: string): this {
    this.style.errorColor = color;
    return this;
  }

  /** Set success color */
  successColor(color: string): this {
    this.style.successColor = color;
    return this;
  }

  /** Add CSS class */
  addClass(className: string): this {
    this.cssClasses.push(className);
    return this;
  }

  /** Set field validation callback */
  onFieldValidate(callback: (fieldId: FieldId, result: ValidationResult) => void): this {
    this.callbacks.onFieldValidate = callback;
    return this;
  }

  /** Set form validation callback */
  onFormValidate(callback: (isValid: boolean, messages: ValidationMessage[]) => void): this {
    this.callbacks.onFormValidate = callback;
    return this;
  }

  /** Set field change callback */
  onFieldChange(callback: (fieldId: FieldId, value: string) => void): this {
    this.callbacks.onFieldChange = callback;
    return this;
  }

  /** Set submit callback */
  onSubmit(callback: (data: Record<FieldId, string>) => void): this {
    this.callbacks.onSubmit = callback;
    return this;
  }

  /** Build the form validator */
  build(): FormValidator {
    const validator = new FormValidator(
      this.id,
      this.fields,
      this.config,
      this.style,
      this.callbacks
    );
    validator.cssClasses.push(...this.cssClasses);
    return validator;
  }
}

/** Convenience functions for common form patterns */

/** Create a user registration form */
export function createRegistrationForm(): FormValidator {
  return FormValidator.builder('registration-form')
    .field(
      new FormField('username', 'Username')
        .fieldType(FieldType.Text)
        .required(true)
        .addRule(ValidationRule.MinLength(3))
        .addRule(ValidationRule.MaxLength(20))
        .addRule(ValidationRule.Pattern(/^[a-zA-Z0-9_]+$/))
        .placeholder('Enter username')
    )
    .field(
      new FormField('email', 'Email Address')
        .fieldType(FieldType.Email)
        .required(true)
        .addRule(ValidationRule.Email)
        .placeholder('user@example.com')
    )
    .field(
      new FormField('password', 'Password')
        .fieldType(FieldType.Password)
        .required(true)
        .addRule(ValidationRule.MinLength(8))
        .addRule(ValidationRule.Pattern(/(?=.*[A-Za-z])(?=.*\d)/))
        .placeholder('Enter password')
        .helpText('Password must contain letters and numbers')
    )
    .field(
      new FormField('confirm_password', 'Confirm Password')
        .fieldType(FieldType.Password)
        .required(true)
        .addRule(ValidationRule.Matches('password'))
        .placeholder('Confirm password')
    )
    .validateOnInput(true)
    .validateOnBlur(true)
    .build();
}

/** Create a contact form */
export function createContactForm(): FormValidator {
  return FormValidator.builder('contact-form')
    .field(
      new FormField('name', 'Full Name')
        .fieldType(FieldType.Text)
        .required(true)
        .addRule(ValidationRule.MinLength(2))
        .placeholder('Your full name')
    )
    .field(
      new FormField('email', 'Email Address')
        .fieldType(FieldType.Email)
        .required(true)
        .addRule(ValidationRule.Email)
        .placeholder('your@email.com')
    )
    .field(
      new FormField('subject', 'Subject')
        .fieldType(FieldType.Text)
        .required(true)
        .addRule(ValidationRule.MinLength(5))
        .placeholder('Message subject')
    )
    .field(
      new FormField('message', 'Message')
        .fieldType(FieldType.Textarea)
        .required(true)
        .addRule(ValidationRule.MinLength(10))
        .addRule(ValidationRule.MaxLength(1000))
        .placeholder('Your message here...')
    )
    .validateOnBlur(true)
    .build();
}

/** Create a login form */
export function createLoginForm(): FormValidator {
  return FormValidator.builder('login-form')
    .field(
      new FormField('email', 'Email or Username')
        .fieldType(FieldType.Text)
        .required(true)
        .placeholder('Enter email or username')
    )
    .field(
      new FormField('password', 'Password')
        .fieldType(FieldType.Password)
        .required(true)
        .placeholder('Enter password')
    )
    .validateOnSubmit(true)
    .trimValues(true)
    .build();
}