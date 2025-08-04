# FormValidation Widget

The FormValidation widget provides a comprehensive form validation system with built-in validators (Required, Email, URL, Numeric, Pattern, etc.), custom validation support, real-time validation feedback, field state management, and complete form lifecycle handling for creating robust data input interfaces.

## Basic Usage

```typescript
import { FormValidator, FormField, ValidationRule, FieldType, FormValidatorBuilder } from 'reactive-tui';

// Basic form validation
const form = new FormValidator('user-form', [
  new FormField('email', 'Email Address')
    .fieldType(FieldType.Email)
    .required(true)
    .addRule(ValidationRule.Email)
    .placeholder('Enter your email'),
  
  new FormField('password', 'Password')
    .fieldType(FieldType.Password)
    .required(true)
    .addRule(ValidationRule.MinLength(8))
    .addRule(ValidationRule.Pattern(/(?=.*[A-Za-z])(?=.*\d)/))
    .helpText('Must contain letters and numbers')
]);

// Using the builder pattern
const builderForm = new FormValidatorBuilder('registration-form')
  .field(
    new FormField('username', 'Username')
      .fieldType(FieldType.Text)
      .required(true)
      .addRule(ValidationRule.MinLength(3))
      .addRule(ValidationRule.MaxLength(20))
  )
  .validateOnInput(true)
  .validateOnBlur(true)
  .debounceMs(300)
  .build();
```

## Configuration

### FormValidationConfig Interface

```typescript
interface FormValidationConfig {
  validateOnInput: boolean;         // Validate during typing (default: true)
  validateOnBlur: boolean;          // Validate when field loses focus (default: true)
  validateOnSubmit: boolean;        // Validate on form submission (default: true)
  showErrorsOnTouch: boolean;       // Show errors only after field interaction (default: true)
  debounceMs: number;              // Input validation delay (default: 300)
  maxErrorsPerField: number;       // Maximum errors to show per field (default: 3)
  stopOnFirstError: boolean;       // Stop validation at first error (default: false)
  caseSensitive: boolean;          // Case-sensitive validation (default: false)
  trimValues: boolean;             // Auto-trim field values (default: true)
  submitOnValid: boolean;          // Auto-submit when valid (default: false)
}
```

### FormValidationState Interface

```typescript
interface FormValidationState {
  fieldValues: Record<FieldId, string>;      // Current field values
  fieldErrors: Record<FieldId, ValidationMessage[]>; // Field error messages
  touchedFields: Set<FieldId>;               // Fields user has interacted with
  isValid: boolean;                          // Overall form validity
  isSubmitting: boolean;                     // Form submission state
  hasSubmitted: boolean;                     // Has attempted submission
  validationCount: number;                   // Number of validations performed
  lastValidationTime: Date | null;           // Last validation timestamp
}
```

### FieldType Enum

```typescript
enum FieldType {
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
```

## Core Features

### Built-in Validation Rules

```typescript
// Basic validators
const form = new FormValidatorBuilder('validation-demo')
  .field(
    new FormField('email', 'Email')
      .addRule(ValidationRule.Required)      // Required field
      .addRule(ValidationRule.Email)         // Valid email format
  )
  .field(
    new FormField('website', 'Website')
      .addRule(ValidationRule.Url)          // Valid URL format
  )
  .field(
    new FormField('age', 'Age')
      .fieldType(FieldType.Number)
      .addRule(ValidationRule.Numeric)       // Numeric value
      .addRule(ValidationRule.Positive)      // Positive numbers only
      .addRule(ValidationRule.MinValue(18))  // Minimum value
      .addRule(ValidationRule.MaxValue(120)) // Maximum value
  )
  .field(
    new FormField('username', 'Username')
      .addRule(ValidationRule.MinLength(3))  // Minimum length
      .addRule(ValidationRule.MaxLength(20)) // Maximum length
      .addRule(ValidationRule.Pattern(/^[a-zA-Z0-9_]+$/)) // Pattern matching
  )
  .build();

// Additional validators
form.fields.add(
  new FormField('confirmation', 'Confirm Password')
    .addRule(ValidationRule.Matches('password')) // Must match another field
);

form.fields.add(
  new FormField('role', 'User Role')
    .addRule(ValidationRule.OneOf(['admin', 'user', 'guest'])) // Must be one of
);

form.fields.add(
  new FormField('restricted', 'Restricted Field')
    .addRule(ValidationRule.NotOneOf(['admin', 'root'])) // Cannot be one of
);
```

### Custom Validation

```typescript
// Custom validator functions
const customForm = new FormValidator('custom-validation');

// Add custom validator
customForm.addCustomValidator('strongPassword', (value: string) => {
  const hasUpper = /[A-Z]/.test(value);
  const hasLower = /[a-z]/.test(value);
  const hasNumber = /\d/.test(value);
  const hasSpecial = /[!@#$%^&*]/.test(value);
  
  if (value.length < 12) {
    return { isValid: false, message: 'Password must be at least 12 characters long' };
  }
  
  if (!hasUpper || !hasLower || !hasNumber || !hasSpecial) {
    return { 
      isValid: false, 
      message: 'Password must contain uppercase, lowercase, number, and special character' 
    };
  }
  
  return { isValid: true };
});

// Use custom validator
const passwordField = new FormField('password', 'Password')
  .fieldType(FieldType.Password)
  .addRule(ValidationRule.Custom('strongPassword'));

// Custom validation callback
const callbackForm = new FormValidatorBuilder('callback-form')
  .field(passwordField)
  .onCustomValidate((ruleName, value) => {
    if (ruleName === 'uniqueEmail') {
      // Simulate async email uniqueness check
      const existingEmails = ['admin@example.com', 'user@example.com'];
      if (existingEmails.includes(value.toLowerCase())) {
        return { isValid: false, message: 'Email address is already taken' };
      }
    }
    return { isValid: true };
  })
  .build();
```

### Real-time Validation

```typescript
// Configure validation timing
const realtimeForm = new FormValidatorBuilder('realtime')
  .field(
    new FormField('email', 'Email Address')
      .fieldType(FieldType.Email)
      .required(true)
      .addRule(ValidationRule.Email)
  )
  .validateOnInput(true)      // Validate while typing
  .validateOnBlur(true)       // Validate when field loses focus
  .debounceMs(500)           // Wait 500ms after typing stops
  .onFieldValidate((fieldId, result) => {
    console.log(`Field ${fieldId} validation:`, result.isValid ? 'valid' : result.message);
  })
  .onFormValidate((isValid, messages) => {
    console.log(`Form valid: ${isValid}, errors:`, messages.length);
  })
  .build();

// Manual validation
realtimeForm.setFieldValue('email', 'user@example.com');
const isFieldValid = realtimeForm.validateField('email');
const isFormValid = realtimeForm.validateAll();
```

### Form Field Management

```typescript
class FormFieldManager {
  private form: FormValidator;
  
  constructor() {
    this.form = new FormValidator('managed-form');
  }
  
  // Field value operations
  setFieldValue(fieldId: string, value: string) {
    this.form.setFieldValue(fieldId, value, true); // Auto-validate
  }
  
  getFieldValue(fieldId: string): string {
    return this.form.getFieldValue(fieldId);
  }
  
  // Field interaction tracking
  markFieldAsTouched(fieldId: string) {
    this.form.touchField(fieldId);
  }
  
  isFieldTouched(fieldId: string): boolean {
    return this.form.isFieldTouched(fieldId);
  }
  
  // Error management
  getFieldErrors(fieldId: string) {
    return this.form.getFieldErrors(fieldId);
  }
  
  hasFieldErrors(fieldId: string): boolean {
    return this.form.hasFieldErrors(fieldId);
  }
  
  clearFieldErrors(fieldId?: string) {
    this.form.clearFieldErrors(fieldId);
  }
  
  // Form state
  getFormData() {
    return this.form.getFormData();
  }
  
  isFormValid(): boolean {
    return this.form.isValid();
  }
  
  resetForm() {
    this.form.reset();
  }
}
```

### Form Submission

```typescript
// Form submission handling
const submissionForm = new FormValidatorBuilder('submission')
  .field(
    new FormField('name', 'Full Name')
      .required(true)
      .addRule(ValidationRule.MinLength(2))
  )
  .field(
    new FormField('email', 'Email')
      .fieldType(FieldType.Email)
      .required(true)
      .addRule(ValidationRule.Email)
  )
  .onSubmit(async (formData) => {
    console.log('Form submitted with data:', formData);
    
    try {
      // Simulate API call
      const response = await fetch('/api/submit', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(formData)
      });
      
      if (response.ok) {
        console.log('Form submitted successfully');
      } else {
        console.error('Submission failed');
      }
    } catch (error) {
      console.error('Submission error:', error);
    }
  })
  .build();

// Submit form
const submitForm = async () => {
  const formData = await submissionForm.submit();
  if (formData) {
    console.log('Valid form data:', formData);
  } else {
    console.log('Form has validation errors');
  }
};
```

## Advanced Configuration

### Custom Styling

```typescript
interface FormValidationStyle {
  errorColor?: string;              // Error message color (default: '#ef4444')
  warningColor?: string;            // Warning message color (default: '#f59e0b')
  successColor?: string;            // Success indicator color (default: '#10b981')
  errorBackground?: string;         // Error background color
  fieldBorderError?: string;        // Error field border color (default: '#ef4444')
  fieldBorderValid?: string;        // Valid field border color (default: '#10b981')
  errorIcon?: string;              // Error icon (default: '✗')
  successIcon?: string;            // Success icon (default: '✓')
  warningIcon?: string;            // Warning icon (default: '⚠')
}

// Custom styled form
const styledForm = new FormValidator('styled', [], {
  validateOnInput: true,
  debounceMs: 200
}, {
  errorColor: '#dc2626',
  successColor: '#059669',
  fieldBorderError: '#fca5a5',
  fieldBorderValid: '#6ee7b7',
  errorIcon: '❌',
  successIcon: '✅'
});
```

### Event Callbacks

```typescript
const eventForm = new FormValidatorBuilder('events')
  .field(
    new FormField('email', 'Email')
      .fieldType(FieldType.Email)
      .required(true)
  )
  .onFieldChange((fieldId, value) => {
    console.log(`Field ${fieldId} changed to: ${value}`);
  })
  .onFieldValidate((fieldId, result) => {
    if (result.isValid) {
      console.log(`✓ Field ${fieldId} is valid`);
    } else {
      console.log(`✗ Field ${fieldId} error: ${result.message}`);
    }
  })
  .onFormValidate((isValid, messages) => {
    console.log(`Form validation: ${isValid ? 'PASSED' : 'FAILED'}`);
    if (!isValid) {
      console.log('Errors:', messages.map(m => `${m.fieldId}: ${m.message}`));
    }
  })
  .onSubmit((data) => {
    console.log('Form submitted:', data);
  })
  .build();
```

## Builder Pattern

```typescript
// Comprehensive form builder
const advancedForm = new FormValidatorBuilder('advanced-form')
  .field(
    new FormField('username', 'Username')
      .fieldType(FieldType.Text)
      .required(true)
      .addRule(ValidationRule.MinLength(3))
      .addRule(ValidationRule.MaxLength(20))
      .addRule(ValidationRule.Pattern(/^[a-zA-Z0-9_]+$/))
      .placeholder('Enter username')
      .helpText('3-20 characters, letters, numbers, and underscores only')
      .addClass('form-field-primary')
  )
  .field(
    new FormField('email', 'Email Address')
      .fieldType(FieldType.Email)
      .required(true)
      .addRule(ValidationRule.Email)
      .placeholder('user@example.com')
      .attribute('autocomplete', 'email')
  )
  .field(
    new FormField('password', 'Password')
      .fieldType(FieldType.Password)
      .required(true)
      .addRule(ValidationRule.MinLength(8))
      .addRule(ValidationRule.Pattern(/(?=.*[A-Za-z])(?=.*\d)(?=.*[!@#$%^&*])/))
      .placeholder('Enter password')
      .helpText('At least 8 characters with letters, numbers, and symbols')
  )
  .field(
    new FormField('confirmPassword', 'Confirm Password')
      .fieldType(FieldType.Password)
      .required(true)
      .addRule(ValidationRule.Matches('password'))
      .placeholder('Confirm password')
  )
  .validateOnInput(true)
  .validateOnBlur(true)
  .validateOnSubmit(true)
  .debounceMs(300)
  .maxErrorsPerField(2)
  .trimValues(true)
  .caseSensitive(false)
  .errorColor('#dc2626')
  .successColor('#059669')
  .addClass('registration-form')
  .onFieldValidate((fieldId, result) => {
    // Update UI with validation feedback
  })
  .onFormValidate((isValid, messages) => {
    // Update form submission state
  })
  .onSubmit((data) => {
    // Handle form submission
  })
  .build();
```

## Convenience Functions

```typescript
// Pre-configured forms for common use cases

// User registration form
const registrationForm = createRegistrationForm();
// Contains: username, email, password, confirm password fields
// With appropriate validation rules

// Contact form
const contactForm = createContactForm();
// Contains: name, email, subject, message fields
// With basic validation and length limits

// Login form
const loginForm = createLoginForm();
// Contains: email/username, password fields
// With basic required validation

// Example usage
registrationForm.setFieldValue('username', 'newuser');
registrationForm.setFieldValue('email', 'user@example.com');
registrationForm.setFieldValue('password', 'securepass123!');
registrationForm.setFieldValue('confirmPassword', 'securepass123!');

if (registrationForm.isValid()) {
  const userData = registrationForm.getFormData();
  console.log('Registration data:', userData);
}
```

## Real-World Examples

### User Registration System

```typescript
import { FormValidator, FormField, ValidationRule, FieldType, FormValidatorBuilder } from 'reactive-tui';

class UserRegistrationSystem {
  private form: FormValidator;
  private existingUsers: Set<string> = new Set(['admin@example.com', 'user@example.com']);
  
  constructor() {
    this.form = this.createRegistrationForm();
    this.setupValidation();
  }
  
  private createRegistrationForm(): FormValidator {
    return new FormValidatorBuilder('user-registration')
      .field(
        new FormField('firstName', 'First Name')
          .fieldType(FieldType.Text)
          .required(true)
          .addRule(ValidationRule.MinLength(2))
          .addRule(ValidationRule.MaxLength(50))
          .addRule(ValidationRule.Pattern(/^[a-zA-Z\s-']+$/))
          .placeholder('Enter your first name')
          .helpText('Letters, spaces, hyphens, and apostrophes only')
      )
      .field(
        new FormField('lastName', 'Last Name')
          .fieldType(FieldType.Text)
          .required(true)
          .addRule(ValidationRule.MinLength(2))
          .addRule(ValidationRule.MaxLength(50))
          .addRule(ValidationRule.Pattern(/^[a-zA-Z\s-']+$/))
          .placeholder('Enter your last name')
      )
      .field(
        new FormField('email', 'Email Address')
          .fieldType(FieldType.Email)
          .required(true)
          .addRule(ValidationRule.Email)
          .addRule(ValidationRule.Custom('uniqueEmail'))
          .placeholder('user@example.com')
          .helpText('Must be a valid, unique email address')
      )
      .field(
        new FormField('username', 'Username')
          .fieldType(FieldType.Text)
          .required(true)
          .addRule(ValidationRule.MinLength(3))
          .addRule(ValidationRule.MaxLength(20))
          .addRule(ValidationRule.Pattern(/^[a-zA-Z0-9_]+$/))
          .addRule(ValidationRule.Custom('uniqueUsername'))
          .placeholder('Choose a username')
          .helpText('3-20 characters: letters, numbers, underscores')
      )
      .field(
        new FormField('password', 'Password')
          .fieldType(FieldType.Password)
          .required(true)
          .addRule(ValidationRule.MinLength(12))
          .addRule(ValidationRule.Custom('strongPassword'))
          .placeholder('Create a strong password')
          .helpText('At least 12 characters with mixed case, numbers, and symbols')
      )
      .field(
        new FormField('confirmPassword', 'Confirm Password')
          .fieldType(FieldType.Password)
          .required(true)
          .addRule(ValidationRule.Matches('password'))
          .placeholder('Confirm your password')
      )
      .field(
        new FormField('dateOfBirth', 'Date of Birth')
          .fieldType(FieldType.Date)
          .required(true)
          .addRule(ValidationRule.Custom('minimumAge'))
          .helpText('You must be at least 13 years old')
      )
      .field(
        new FormField('agreeToTerms', 'Terms Agreement')
          .fieldType(FieldType.Checkbox)
          .required(true)
          .addRule(ValidationRule.Custom('mustAgreeToTerms'))
          .helpText('You must agree to the terms and conditions')
      )
      .validateOnInput(true)
      .validateOnBlur(true)
      .debounceMs(300)
      .trimValues(true)
      .maxErrorsPerField(2)
      .build();
  }
  
  private setupValidation() {
    // Custom validators
    this.form.addCustomValidator('uniqueEmail', (value: string) => {
      if (this.existingUsers.has(value.toLowerCase())) {
        return { isValid: false, message: 'This email address is already registered' };
      }
      return { isValid: true };
    });
    
    this.form.addCustomValidator('uniqueUsername', (value: string) => {
      // Simulate username check
      const existingUsernames = ['admin', 'user', 'test', 'demo'];
      if (existingUsernames.includes(value.toLowerCase())) {
        return { isValid: false, message: 'This username is already taken' };
      }
      return { isValid: true };
    });
    
    this.form.addCustomValidator('strongPassword', (value: string) => {
      const checks = {
        length: value.length >= 12,
        uppercase: /[A-Z]/.test(value),
        lowercase: /[a-z]/.test(value),
        number: /\d/.test(value),
        special: /[!@#$%^&*()_+\-=\[\]{};':"\\|,.<>\/?]/.test(value),
        noCommon: !['password', '12345678', 'qwerty'].some(common => 
          value.toLowerCase().includes(common)
        )
      };
      
      const failedChecks = Object.entries(checks)
        .filter(([_, passed]) => !passed)
        .map(([check]) => check);
      
      if (failedChecks.length > 0) {
        const messages = {
          length: 'at least 12 characters',
          uppercase: 'uppercase letters',
          lowercase: 'lowercase letters', 
          number: 'numbers',
          special: 'special characters',
          noCommon: 'avoid common passwords'
        };
        
        const requirements = failedChecks.map(check => messages[check]).join(', ');
        return { isValid: false, message: `Password must include: ${requirements}` };
      }
      
      return { isValid: true };
    });
    
    this.form.addCustomValidator('minimumAge', (value: string) => {
      const birthDate = new Date(value);
      const today = new Date();
      const age = today.getFullYear() - birthDate.getFullYear();
      const monthDiff = today.getMonth() - birthDate.getMonth();
      
      if (monthDiff < 0 || (monthDiff === 0 && today.getDate() < birthDate.getDate())) {
        age--;
      }
      
      if (age < 13) {
        return { isValid: false, message: 'You must be at least 13 years old to register' };
      }
      
      return { isValid: true };
    });
    
    this.form.addCustomValidator('mustAgreeToTerms', (value: string) => {
      if (value !== 'true' && value !== 'on' && value !== '1') {
        return { isValid: false, message: 'You must agree to the terms and conditions' };
      }
      return { isValid: true };
    });
  }
  
  // Public methods
  setField(fieldId: string, value: string) {
    this.form.setFieldValue(fieldId, value);
  }
  
  getField(fieldId: string): string {
    return this.form.getFieldValue(fieldId);
  }
  
  validateField(fieldId: string) {
    return this.form.validateField(fieldId);
  }
  
  validateAll(): boolean {
    return this.form.validateAll();
  }
  
  getErrors() {
    return this.form.getAllMessages();
  }
  
  async submitRegistration() {
    const formData = await this.form.submit();
    
    if (formData) {
      try {
        // Simulate registration API call
        console.log('Submitting registration:', formData);
        
        // Add to existing users to prevent duplicates
        this.existingUsers.add(formData.email.toLowerCase());
        
        return {
          success: true,
          message: 'Registration successful! Welcome to the platform.',
          userId: `user_${Date.now()}`
        };
      } catch (error) {
        return {
          success: false,
          message: 'Registration failed. Please try again.',
          error
        };
      }
    } else {
      return {
        success: false,
        message: 'Please fix the validation errors before submitting.',
        errors: this.form.getAllMessages()
      };
    }
  }
  
  reset() {
    this.form.reset();
  }
  
  getProgress(): { completed: number; total: number; percentage: number } {
    const fields = ['firstName', 'lastName', 'email', 'username', 'password', 'confirmPassword', 'dateOfBirth', 'agreeToTerms'];
    const completed = fields.filter(field => {
      const value = this.form.getFieldValue(field);
      return value && value.trim().length > 0 && !this.form.hasFieldErrors(field);
    }).length;
    
    return {
      completed,
      total: fields.length,
      percentage: Math.round((completed / fields.length) * 100)
    };
  }
  
  render(): string {
    return this.form.render();
  }
}

// Usage
const registrationSystem = new UserRegistrationSystem();

// Fill out form
registrationSystem.setField('firstName', 'John');
registrationSystem.setField('lastName', 'Doe');
registrationSystem.setField('email', 'john.doe@example.com');
registrationSystem.setField('username', 'johndoe123');
registrationSystem.setField('password', 'MySecurePassword123!');
registrationSystem.setField('confirmPassword', 'MySecurePassword123!');
registrationSystem.setField('dateOfBirth', '1990-05-15');
registrationSystem.setField('agreeToTerms', 'true');

// Check progress
const progress = registrationSystem.getProgress();
console.log(`Registration ${progress.percentage}% complete (${progress.completed}/${progress.total})`);

// Submit
const result = await registrationSystem.submitRegistration();
console.log('Registration result:', result);
```

### Multi-Step Form Wizard

```typescript
class MultiStepFormWizard {
  private forms: Map<string, FormValidator> = new Map();
  private currentStep: string = 'personal';
  private steps: string[] = ['personal', 'contact', 'preferences', 'review'];
  private completedSteps: Set<string> = new Set();
  
  constructor() {
    this.initializeForms();
  }
  
  private initializeForms() {
    // Step 1: Personal Information
    this.forms.set('personal', new FormValidatorBuilder('step-personal')
      .field(
        new FormField('title', 'Title')
          .fieldType(FieldType.Select)
          .addRule(ValidationRule.OneOf(['Mr', 'Mrs', 'Ms', 'Dr', 'Prof']))
          .defaultValue('Mr')
      )
      .field(
        new FormField('firstName', 'First Name')
          .required(true)
          .addRule(ValidationRule.MinLength(2))
          .addRule(ValidationRule.Pattern(/^[a-zA-Z\s-']+$/))
      )
      .field(
        new FormField('lastName', 'Last Name')
          .required(true)
          .addRule(ValidationRule.MinLength(2))
          .addRule(ValidationRule.Pattern(/^[a-zA-Z\s-']+$/))
      )
      .field(
        new FormField('dateOfBirth', 'Date of Birth')
          .fieldType(FieldType.Date)
          .required(true)
          .addRule(ValidationRule.Custom('validAge'))
      )
      .validateOnBlur(true)
      .build()
    );
    
    // Step 2: Contact Information  
    this.forms.set('contact', new FormValidatorBuilder('step-contact')
      .field(
        new FormField('email', 'Email Address')
          .fieldType(FieldType.Email)
          .required(true)
          .addRule(ValidationRule.Email)
      )
      .field(
        new FormField('phone', 'Phone Number')
          .fieldType(FieldType.Phone)
          .required(true)
          .addRule(ValidationRule.Pattern(/^\+?[\d\s\-\(\)]+$/))
          .addRule(ValidationRule.MinLength(10))
      )
      .field(
        new FormField('address', 'Street Address')
          .required(true)
          .addRule(ValidationRule.MinLength(5))
      )
      .field(
        new FormField('city', 'City')
          .required(true)
          .addRule(ValidationRule.MinLength(2))
      )
      .field(
        new FormField('postalCode', 'Postal Code')
          .required(true)
          .addRule(ValidationRule.Pattern(/^[A-Z0-9\s-]{3,10}$/))
      )
      .validateOnBlur(true)
      .build()
    );
    
    // Step 3: Preferences
    this.forms.set('preferences', new FormValidatorBuilder('step-preferences')
      .field(
        new FormField('newsletter', 'Newsletter Subscription')
          .fieldType(FieldType.Checkbox)
          .defaultValue('false')
      )
      .field(
        new FormField('notifications', 'Email Notifications')
          .fieldType(FieldType.Select)
          .addRule(ValidationRule.OneOf(['all', 'important', 'none']))
          .defaultValue('important')
      )
      .field(
        new FormField('language', 'Preferred Language')
          .fieldType(FieldType.Select)
          .required(true)
          .addRule(ValidationRule.OneOf(['en', 'es', 'fr', 'de']))
          .defaultValue('en')
      )
      .validateOnBlur(true)
      .build()
    );
    
    // Add custom validators
    this.forms.get('personal')!.addCustomValidator('validAge', (value: string) => {
      const birthDate = new Date(value);
      const today = new Date();
      const age = today.getFullYear() - birthDate.getFullYear();
      
      if (age < 18 || age > 120) {
        return { isValid: false, message: 'Age must be between 18 and 120 years' };
      }
      
      return { isValid: true };
    });
  }
  
  getCurrentStep(): string {
    return this.currentStep;
  }
  
  getSteps(): string[] {
    return [...this.steps];
  }
  
  getCompletedSteps(): string[] {
    return Array.from(this.completedSteps);
  }
  
  getCurrentForm(): FormValidator {
    return this.forms.get(this.currentStep)!;
  }
  
  validateCurrentStep(): boolean {
    const form = this.getCurrentForm();
    const isValid = form.validateAll();
    
    if (isValid) {
      this.completedSteps.add(this.currentStep);
    }
    
    return isValid;
  }
  
  canGoToNext(): boolean {
    return this.validateCurrentStep() && this.hasNextStep();
  }
  
  canGoToPrevious(): boolean {
    return this.hasPreviousStep();
  }
  
  hasNextStep(): boolean {
    const currentIndex = this.steps.indexOf(this.currentStep);
    return currentIndex < this.steps.length - 1;
  }
  
  hasPreviousStep(): boolean {
    const currentIndex = this.steps.indexOf(this.currentStep);
    return currentIndex > 0;
  }
  
  goToNext(): boolean {
    if (!this.canGoToNext()) return false;
    
    const currentIndex = this.steps.indexOf(this.currentStep);
    this.currentStep = this.steps[currentIndex + 1];
    return true;
  }
  
  goToPrevious(): boolean {
    if (!this.canGoToPrevious()) return false;
    
    const currentIndex = this.steps.indexOf(this.currentStep);
    this.currentStep = this.steps[currentIndex - 1];
    return true;
  }
  
  goToStep(stepName: string): boolean {
    if (!this.steps.includes(stepName)) return false;
    
    const targetIndex = this.steps.indexOf(stepName);
    const currentIndex = this.steps.indexOf(this.currentStep);
    
    // Can only go to previous steps or next step if current is valid
    if (targetIndex > currentIndex && !this.validateCurrentStep()) {
      return false;
    }
    
    this.currentStep = stepName;
    return true;
  }
  
  getProgress(): { current: number; total: number; percentage: number } {
    const currentIndex = this.steps.indexOf(this.currentStep);
    return {
      current: currentIndex + 1,
      total: this.steps.length,
      percentage: Math.round(((currentIndex + 1) / this.steps.length) * 100)
    };
  }
  
  getAllFormData(): Record<string, any> {
    const allData: Record<string, any> = {};
    
    for (const [stepName, form] of this.forms) {
      const stepData = form.getFormData();
      Object.assign(allData, stepData);
    }
    
    return allData;
  }
  
  getStepData(stepName: string): Record<string, string> | null {
    const form = this.forms.get(stepName);
    return form ? form.getFormData() : null;
  }
  
  isComplete(): boolean {
    return this.steps.every(step => this.completedSteps.has(step));
  }
  
  getAllErrors(): Record<string, any[]> {
    const errors: Record<string, any[]> = {};
    
    for (const [stepName, form] of this.forms) {
      const stepErrors = form.getAllMessages();
      if (stepErrors.length > 0) {
        errors[stepName] = stepErrors;
      }
    }
    
    return errors;
  }
  
  async submitWizard() {
    // Validate all steps
    const allValid = this.steps.every(stepName => {
      const form = this.forms.get(stepName)!;
      return form.validateAll();
    });
    
    if (!allValid) {
      return {
        success: false,
        message: 'Please complete all steps with valid information',
        errors: this.getAllErrors()
      };
    }
    
    try {
      const formData = this.getAllFormData();
      console.log('Submitting wizard data:', formData);
      
      // Simulate API submission
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      return {
        success: true,
        message: 'Form submitted successfully!',
        data: formData
      };
    } catch (error) {
      return {
        success: false,
        message: 'Submission failed. Please try again.',
        error
      };
    }
  }
  
  reset() {
    this.currentStep = 'personal';
    this.completedSteps.clear();
    
    for (const form of this.forms.values()) {
      form.reset();
    }
  }
  
  renderCurrentStep(): string {
    return this.getCurrentForm().render();
  }
  
  renderProgressIndicator(): string {
    const progress = this.getProgress();
    const indicators = this.steps.map((step, index) => {
      const isCompleted = this.completedSteps.has(step);
      const isCurrent = step === this.currentStep;
      
      let indicator = `${index + 1}`;
      if (isCompleted) indicator = '✓';
      if (isCurrent) indicator = `[${indicator}]`;
      
      return `${indicator} ${step}`;
    });
    
    return `Progress: ${progress.percentage}%\n${indicators.join(' → ')}`;
  }
}

// Usage
const wizard = new MultiStepFormWizard();

// Navigate through steps
console.log('Current step:', wizard.getCurrentStep());
console.log(wizard.renderProgressIndicator());

// Fill current step
const currentForm = wizard.getCurrentForm();
currentForm.setFieldValue('title', 'Mr');
currentForm.setFieldValue('firstName', 'John');
currentForm.setFieldValue('lastName', 'Doe');
currentForm.setFieldValue('dateOfBirth', '1990-01-01');

// Move to next step
if (wizard.goToNext()) {
  console.log('Moved to:', wizard.getCurrentStep());
  
  // Fill contact information
  const contactForm = wizard.getCurrentForm();
  contactForm.setFieldValue('email', 'john.doe@example.com');
  contactForm.setFieldValue('phone', '+1-555-123-4567');
  contactForm.setFieldValue('address', '123 Main St');
  contactForm.setFieldValue('city', 'Anytown');
  contactForm.setFieldValue('postalCode', '12345');
}

// Check if wizard is complete
console.log('Wizard complete:', wizard.isComplete());

// Submit when ready
const result = await wizard.submitWizard();
console.log('Wizard result:', result);
```

### Dynamic Survey Generator

```typescript
interface SurveyQuestion {
  id: string;
  type: 'text' | 'email' | 'number' | 'select' | 'radio' | 'checkbox' | 'textarea';
  label: string;
  required?: boolean;
  options?: string[];
  validation?: {
    minLength?: number;
    maxLength?: number;
    min?: number;
    max?: number;
    pattern?: string;
    custom?: string;
  };
  dependencies?: {
    field: string;
    value: string;
    condition: 'equals' | 'not_equals' | 'contains';
  }[];
}

interface SurveyDefinition {
  id: string;
  title: string;
  description: string;
  questions: SurveyQuestion[];
}

class DynamicSurveyGenerator {
  private form: FormValidator;
  private definition: SurveyDefinition;
  private responses: Map<string, string> = new Map();
  
  constructor(definition: SurveyDefinition) {
    this.definition = definition;
    this.form = this.generateForm();
  }
  
  private generateForm(): FormValidator {
    const builder = new FormValidatorBuilder(this.definition.id);
    
    for (const question of this.definition.questions) {
      const field = this.createFieldFromQuestion(question);
      builder.field(field);
    }
    
    const form = builder
      .validateOnBlur(true)
      .validateOnInput(false)
      .debounceMs(300)
      .onFieldChange((fieldId, value) => {
        this.responses.set(fieldId, value);
        this.updateConditionalFields();
      })
      .build();
    
    // Add custom validators
    this.addCustomValidators(form);
    
    return form;
  }
  
  private createFieldFromQuestion(question: SurveyQuestion): FormField {
    const fieldType = this.mapQuestionTypeToFieldType(question.type);
    
    const field = new FormField(question.id, question.label)
      .fieldType(fieldType);
    
    if (question.required) {
      field.required(true);
    }
    
    // Add validation rules
    if (question.validation) {
      const validation = question.validation;
      
      if (validation.minLength) {
        field.addRule(ValidationRule.MinLength(validation.minLength));
      }
      
      if (validation.maxLength) {
        field.addRule(ValidationRule.MaxLength(validation.maxLength));
      }
      
      if (validation.min !== undefined) {
        field.addRule(ValidationRule.MinValue(validation.min));
      }
      
      if (validation.max !== undefined) {
        field.addRule(ValidationRule.MaxValue(validation.max));
      }
      
      if (validation.pattern) {
        field.addRule(ValidationRule.Pattern(new RegExp(validation.pattern)));
      }
      
      if (validation.custom) {
        field.addRule(ValidationRule.Custom(validation.custom));
      }
    }
    
    // Add field type specific rules
    if (question.type === 'email') {
      field.addRule(ValidationRule.Email);
    }
    
    if (question.type === 'number') {
      field.addRule(ValidationRule.Numeric);
    }
    
    if (question.options && ['select', 'radio'].includes(question.type)) {
      field.addRule(ValidationRule.OneOf(question.options));
    }
    
    return field;
  }
  
  private mapQuestionTypeToFieldType(type: string): FieldType {
    const mapping: Record<string, FieldType> = {
      'text': FieldType.Text,
      'email': FieldType.Email,
      'number': FieldType.Number,
      'select': FieldType.Select,
      'radio': FieldType.Radio,
      'checkbox': FieldType.Checkbox,
      'textarea': FieldType.Textarea
    };
    
    return mapping[type] || FieldType.Text;
  }
  
  private addCustomValidators(form: FormValidator) {
    // Phone number validator
    form.addCustomValidator('phone', (value: string) => {
      const phoneRegex = /^\+?[\d\s\-\(\)]{10,}$/;
      if (!phoneRegex.test(value)) {
        return { isValid: false, message: 'Please enter a valid phone number' };
      }
      return { isValid: true };
    });
    
    // Age validator
    form.addCustomValidator('age', (value: string) => {
      const age = parseInt(value);
      if (age < 13 || age > 120) {
        return { isValid: false, message: 'Age must be between 13 and 120' };
      }
      return { isValid: true };
    });
    
    // URL validator
    form.addCustomValidator('website', (value: string) => {
      try {
        new URL(value);
        return { isValid: true };
      } catch {
        return { isValid: false, message: 'Please enter a valid website URL' };
      }
    });
  }
  
  private updateConditionalFields() {
    for (const question of this.definition.questions) {
      if (question.dependencies) {
        const shouldShow = this.evaluateConditions(question.dependencies);
        
        if (!shouldShow) {
          // Clear the field value if it should be hidden
          this.form.setFieldValue(question.id, '', false);
        }
      }
    }
  }
  
  private evaluateConditions(dependencies: SurveyQuestion['dependencies']): boolean {
    if (!dependencies) return true;
    
    return dependencies.every(dep => {
      const fieldValue = this.responses.get(dep.field) || '';
      
      switch (dep.condition) {
        case 'equals':
          return fieldValue === dep.value;
        case 'not_equals':
          return fieldValue !== dep.value;
        case 'contains':
          return fieldValue.includes(dep.value);
        default:
          return true;
      }
    });
  }
  
  // Public methods
  getVisibleQuestions(): SurveyQuestion[] {
    return this.definition.questions.filter(question => {
      if (!question.dependencies) return true;
      return this.evaluateConditions(question.dependencies);
    });
  }
  
  setResponse(questionId: string, value: string) {
    this.form.setFieldValue(questionId, value);
  }
  
  getResponse(questionId: string): string {
    return this.form.getFieldValue(questionId);
  }
  
  getAllResponses(): Record<string, string> {
    return this.form.getFormData();
  }
  
  validateSurvey(): boolean {
    return this.form.validateAll();
  }
  
  getValidationErrors() {
    return this.form.getAllMessages();
  }
  
  getProgress(): { answered: number; total: number; percentage: number } {
    const visibleQuestions = this.getVisibleQuestions();
    const answered = visibleQuestions.filter(q => {
      const value = this.getResponse(q.id);
      return value && value.trim().length > 0;
    }).length;
    
    return {
      answered,
      total: visibleQuestions.length,
      percentage: visibleQuestions.length > 0 ? Math.round((answered / visibleQuestions.length) * 100) : 0
    };
  }
  
  async submitSurvey() {
    if (!this.validateSurvey()) {
      return {
        success: false,
        message: 'Please complete all required questions',
        errors: this.getValidationErrors()
      };
    }
    
    try {
      const responses = this.getAllResponses();
      const submissionData = {
        surveyId: this.definition.id,
        responses,
        submittedAt: new Date().toISOString(),
        completionTime: this.calculateCompletionTime()
      };
      
      console.log('Submitting survey:', submissionData);
      
      // Simulate API submission
      await new Promise(resolve => setTimeout(resolve, 1000));
      
      return {
        success: true,
        message: 'Survey submitted successfully!',
        data: submissionData
      };
    } catch (error) {
      return {
        success: false,
        message: 'Survey submission failed. Please try again.',
        error
      };
    }
  }
  
  private calculateCompletionTime(): number {
    // This would track actual time in a real implementation
    return Math.floor(Math.random() * 300) + 60; // 1-5 minutes
  }
  
  reset() {
    this.form.reset();
    this.responses.clear();
  }
  
  exportResponses(): string {
    const responses = this.getAllResponses();
    const data = {
      survey: this.definition.title,
      responses,
      exportedAt: new Date().toISOString()
    };
    
    return JSON.stringify(data, null, 2);
  }
  
  render(): string {
    return this.form.render();
  }
}

// Example survey definition
const customerSatisfactionSurvey: SurveyDefinition = {
  id: 'customer-satisfaction-2024',
  title: 'Customer Satisfaction Survey',
  description: 'Help us improve our service by providing your feedback',
  questions: [
    {
      id: 'name',
      type: 'text',
      label: 'Full Name',
      required: true,
      validation: {
        minLength: 2,
        maxLength: 100
      }
    },
    {
      id: 'email',
      type: 'email',
      label: 'Email Address',
      required: true
    },
    {
      id: 'customerType',
      type: 'select',
      label: 'Customer Type',
      required: true,
      options: ['New Customer', 'Existing Customer', 'Returning Customer']
    },
    {
      id: 'yearsAsCustomer',
      type: 'number',
      label: 'How many years have you been our customer?',
      required: true,
      validation: {
        min: 0,
        max: 50
      },
      dependencies: [
        { field: 'customerType', value: 'New Customer', condition: 'not_equals' }
      ]
    },
    {
      id: 'overallSatisfaction',
      type: 'radio',
      label: 'Overall Satisfaction',
      required: true,
      options: ['Very Satisfied', 'Satisfied', 'Neutral', 'Dissatisfied', 'Very Dissatisfied']
    },
    {
      id: 'improvementSuggestions',
      type: 'textarea',
      label: 'What can we improve?',
      required: false,
      validation: {
        maxLength: 1000
      },
      dependencies: [
        { field: 'overallSatisfaction', value: 'Very Satisfied', condition: 'not_equals' }
      ]
    },
    {
      id: 'wouldRecommend',
      type: 'radio',
      label: 'Would you recommend us to others?',
      required: true,
      options: ['Definitely', 'Probably', 'Maybe', 'Probably Not', 'Definitely Not']
    },
    {
      id: 'contactForFollowup',
      type: 'checkbox',
      label: 'May we contact you for follow-up questions?',
      required: false
    },
    {
      id: 'preferredContactMethod',
      type: 'radio',
      label: 'Preferred Contact Method',
      required: false,
      options: ['Email', 'Phone', 'Text Message'],
      dependencies: [
        { field: 'contactForFollowup', value: 'true', condition: 'equals' }
      ]
    }
  ]
};

// Usage
const survey = new DynamicSurveyGenerator(customerSatisfactionSurvey);

// Fill out survey
survey.setResponse('name', 'John Doe');
survey.setResponse('email', 'john.doe@example.com');
survey.setResponse('customerType', 'Existing Customer');
survey.setResponse('yearsAsCustomer', '5');
survey.setResponse('overallSatisfaction', 'Satisfied');
survey.setResponse('wouldRecommend', 'Definitely');
survey.setResponse('contactForFollowup', 'true');
survey.setResponse('preferredContactMethod', 'Email');

// Check progress
const progress = survey.getProgress();
console.log(`Survey ${progress.percentage}% complete (${progress.answered}/${progress.total})`);

// Get only visible questions (conditional logic applied)
const visibleQuestions = survey.getVisibleQuestions();
console.log('Visible questions:', visibleQuestions.map(q => q.label));

// Submit survey
const result = await survey.submitSurvey();
console.log('Survey result:', result);

// Export responses
const exportData = survey.exportResponses();
console.log('Exported data:', exportData);
```

## Performance Considerations

```typescript
// Performance monitoring
const performanceForm = new FormValidatorBuilder('performance')
  .field(
    new FormField('email', 'Email')
      .fieldType(FieldType.Email)
      .required(true)
  )
  .debounceMs(300)        // Debounce validation calls
  .maxErrorsPerField(3)   // Limit error messages
  .trimValues(true)       // Auto-trim to reduce validation calls
  .stopOnFirstError(false) // Show all errors for better UX
  .build();

// Monitor validation performance
const validationTimes: number[] = [];
performanceForm.onFieldValidate((fieldId, result) => {
  const startTime = performance.now();
  // Validation logic here
  const endTime = performance.now();
  validationTimes.push(endTime - startTime);
  
  if (validationTimes.length > 100) {
    const avgTime = validationTimes.reduce((a, b) => a + b) / validationTimes.length;
    console.log(`Average validation time: ${avgTime.toFixed(2)}ms`);
    validationTimes.length = 0; // Reset
  }
});
```

## Best Practices

1. **Validation Strategy**
   - Use appropriate validation timing (onInput vs onBlur)
   - Debounce input validation to avoid excessive calls
   - Show errors only after user interaction

2. **User Experience**
   - Provide clear, helpful error messages
   - Use field help text for guidance
   - Show validation progress for complex forms

3. **Performance**
   - Limit maximum errors per field
   - Use stopOnFirstError for simple validations
   - Implement custom validators efficiently

4. **Accessibility**
   - Use semantic field types
   - Provide meaningful labels and help text
   - Ensure keyboard navigation works properly

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const container = new ElementBuilderImpl('div')
  .class('form-container')
  .child(
    new FormValidatorBuilder('integrated-form')
      .field(
        new FormField('name', 'Name')
          .required(true)
          .addClass('form-field-primary')
      )
      .build()
  )
  .build();
```

The FormValidation widget provides comprehensive form validation with built-in validators, custom validation support, real-time feedback, and complete form lifecycle management for creating robust data input interfaces in terminal applications.