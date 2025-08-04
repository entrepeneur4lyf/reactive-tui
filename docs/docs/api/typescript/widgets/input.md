# Input Widget

Comprehensive text input component with validation, formatting, accessibility, and advanced styling support.

## Overview

The Input widget provides a full-featured text input system with support for various input types, real-time validation, custom formatting, and extensive styling options.

```typescript
import { input, InputConfig, InputType } from 'reactive-tui-ts'

const emailInput = input({
  id: 'user-email',
  type: 'email',
  label: 'Email Address',
  placeholder: 'Enter your email',
  variant: 'outlined',
  validation: {
    required: true,
    pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  },
  onInput: (value) => {
    console.log('Email input:', value)
  }
})
```

## Types

### InputType

```typescript
export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' | 'search' | 'date' | 'time' | 'datetime-local'
```

### InputVariant

```typescript
export type InputVariant = 'filled' | 'outlined' | 'underlined' | 'ghost' | 'flushed'
```

### InputSize

```typescript
export type InputSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'
```

### InputStatus

```typescript
export type InputStatus = 'default' | 'success' | 'warning' | 'error' | 'loading'
```

## Configuration

### InputValidation

```typescript
interface InputValidation {
  required?: boolean
  minLength?: number
  maxLength?: number
  pattern?: RegExp
  custom?: (value: string) => string | null
}
```

### InputFormatting

```typescript
interface InputFormatting {
  mask?: string                    // Format mask like '(000) 000-0000'
  transform?: 'lowercase' | 'uppercase' | 'capitalize'
  filter?: RegExp                  // Allow only specific characters
}
```

### InputConfig

```typescript
interface InputConfig {
  id: string
  type?: InputType
  variant?: InputVariant
  size?: InputSize
  status?: InputStatus
  
  // Content
  value?: string
  placeholder?: string
  label?: string
  helpText?: string
  errorMessage?: string
  
  // Validation and Formatting
  validation?: InputValidation
  formatting?: InputFormatting
  
  // Behavior
  disabled?: boolean
  readonly?: boolean
  autoFocus?: boolean
  clearable?: boolean
  showCounter?: boolean
  
  // Styling
  width?: number | string
  cssClasses?: string[]
  style?: {
    backgroundColor?: string
    borderColor?: string
    textColor?: string
    placeholderColor?: string
    focusColor?: string
  }
  
  // Events
  onInput?: (value: string, event?: any) => void
  onChange?: (value: string, event?: any) => void
  onFocus?: (event?: any) => void
  onBlur?: (event?: any) => void
  onKeyPress?: (key: string, event?: any) => void
  onValidation?: (isValid: boolean, errors: string[]) => void
}
```

## Examples

### Basic Text Input

```typescript
import { input } from 'reactive-tui-ts'

const nameInput = input({
  id: 'full-name',
  type: 'text',
  label: 'Full Name',
  placeholder: 'Enter your full name',
  variant: 'outlined',
  size: 'md',
  validation: {
    required: true,
    minLength: 2
  },
  onInput: (value) => {
    console.log('Name changed:', value)
  }
})
```

### Email Input with Validation

```typescript
const emailField = input({
  id: 'email-address',
  type: 'email',
  label: 'Email Address',
  placeholder: 'user@example.com',
  variant: 'outlined',
  validation: {
    required: true,
    pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/
  },
  helpText: 'We will never share your email address',
  onValidation: (isValid, errors) => {
    if (!isValid) {
      console.log('Email validation errors:', errors)
    }
  }
})
```

### Password Input

```typescript
const passwordField = input({
  id: 'user-password',
  type: 'password',
  label: 'Password',
  placeholder: 'Enter your password',
  variant: 'outlined',
  clearable: true,
  showCounter: true,
  validation: {
    required: true,
    minLength: 8,
    maxLength: 128,
    custom: (value) => {
      if (!/(?=.*[a-z])(?=.*[A-Z])(?=.*\d)/.test(value)) {
        return 'Password must contain at least one uppercase letter, one lowercase letter, and one number'
      }
      return null
    }
  }
})
```

### Number Input with Range

```typescript
const ageInput = input({
  id: 'user-age',
  type: 'number',
  label: 'Age',
  placeholder: 'Enter your age',
  variant: 'filled',
  validation: {
    required: true,
    custom: (value) => {
      const num = parseInt(value)
      if (isNaN(num)) return 'Must be a valid number'
      if (num < 13) return 'Must be at least 13 years old'
      if (num > 120) return 'Must be less than 120 years old'
      return null
    }
  }
})
```

### Phone Number with Formatting

```typescript
const phoneInput = input({
  id: 'phone-number',
  type: 'tel',
  label: 'Phone Number',
  placeholder: '(555) 123-4567',
  variant: 'outlined',
  formatting: {
    mask: '(000) 000-0000',
    filter: /[0-9]/
  },
  validation: {
    required: true,
    pattern: /^\(\d{3}\) \d{3}-\d{4}$/
  }
})
```

### Search Input

```typescript
const searchField = input({
  id: 'search-query',
  type: 'search',
  placeholder: 'Search products...',
  variant: 'filled',
  clearable: true,
  size: 'lg',
  onInput: (value) => {
    // Debounced search
    if (value.length >= 3) {
      performSearch(value)
    }
  }
})
```

### Different Input Variants

```typescript
const variantExamples = [
  input({
    id: 'filled-input',
    label: 'Filled Input',
    variant: 'filled',
    placeholder: 'Filled variant'
  }),
  
  input({
    id: 'outlined-input',
    label: 'Outlined Input',
    variant: 'outlined',
    placeholder: 'Outlined variant'
  }),
  
  input({
    id: 'underlined-input',
    label: 'Underlined Input',
    variant: 'underlined',
    placeholder: 'Underlined variant'
  }),
  
  input({
    id: 'ghost-input',
    label: 'Ghost Input',
    variant: 'ghost',
    placeholder: 'Ghost variant'
  }),
  
  input({
    id: 'flushed-input',
    label: 'Flushed Input',
    variant: 'flushed',
    placeholder: 'Flushed variant'
  })
]
```

### Input Sizes

```typescript
const sizeExamples = [
  input({
    id: 'xs-input',
    label: 'Extra Small',
    size: 'xs',
    placeholder: 'XS input'
  }),
  
  input({
    id: 'sm-input',
    label: 'Small',
    size: 'sm',
    placeholder: 'Small input'
  }),
  
  input({
    id: 'md-input',
    label: 'Medium',
    size: 'md',
    placeholder: 'Medium input'
  }),
  
  input({
    id: 'lg-input',
    label: 'Large',
    size: 'lg',
    placeholder: 'Large input'
  }),
  
  input({
    id: 'xl-input',
    label: 'Extra Large',
    size: 'xl',
    placeholder: 'XL input'
  })
]
```

## Convenience Functions

### Text Input

```typescript
import { textInput } from 'reactive-tui-ts'

const basicText = textInput('user-name', {
  label: 'Name',
  placeholder: 'Enter your name',
  validation: { required: true }
})
```

### Email Input

```typescript
import { emailInput } from 'reactive-tui-ts'

const email = emailInput('user-email', {
  label: 'Email Address',
  placeholder: 'user@example.com'
})
```

### Password Input

```typescript
import { passwordInput } from 'reactive-tui-ts'

const password = passwordInput('user-password', {
  label: 'Password',
  validation: {
    required: true,
    minLength: 8
  }
})
```

### Number Input

```typescript
import { numberInput } from 'reactive-tui-ts'

const quantity = numberInput('item-quantity', 1, 100, {
  label: 'Quantity',
  placeholder: 'Enter quantity'
})
```

### Search Input

```typescript
import { searchInput } from 'reactive-tui-ts'

const search = searchInput('product-search', {
  placeholder: 'Search products...',
  size: 'lg'
})
```

### Phone Input

```typescript
import { phoneInput } from 'reactive-tui-ts'

const phone = phoneInput('contact-phone', {
  label: 'Phone Number',
  placeholder: '(555) 123-4567'
})
```

### URL Input

```typescript
import { urlInput } from 'reactive-tui-ts'

const website = urlInput('website-url', {
  label: 'Website',
  placeholder: 'https://example.com'
})
```

## Input Group

```typescript
import { inputGroup } from 'reactive-tui-ts'

const addressForm = inputGroup([
  input({
    id: 'street',
    label: 'Street Address',
    validation: { required: true }
  }),
  input({
    id: 'city',
    label: 'City',
    validation: { required: true }
  }),
  input({
    id: 'state',
    label: 'State',
    validation: { required: true }
  }),
  input({
    id: 'zip',
    label: 'ZIP Code',
    validation: {
      required: true,
      pattern: /^\d{5}(-\d{4})?$/
    }
  })
])

// Validate entire group
const groupValidation = addressForm.validate()
console.log('Form valid:', groupValidation.isValid)
console.log('Errors:', groupValidation.errors)
```

## CSS Styling

```css
/* Input container */
.input-wrapper {
  display: flex;
  flex-direction: column;
  gap: 4px;
  margin-bottom: 16px;
}

/* Input label */
.input-label {
  font-size: 14px;
  font-weight: 500;
  color: #374151;
  margin-bottom: 4px;
}

.input-label-xs { font-size: 12px; }
.input-label-sm { font-size: 13px; }
.input-label-md { font-size: 14px; }
.input-label-lg { font-size: 16px; }
.input-label-xl { font-size: 18px; }

/* Base input styles */
.input {
  width: 100%;
  font-family: inherit;
  transition: all 0.2s ease;
  outline: none;
}

/* Size variants */
.input-xs {
  padding: 6px 8px;
  font-size: 12px;
  height: 28px;
}

.input-sm {
  padding: 8px 12px;
  font-size: 13px;
  height: 32px;
}

.input-md {
  padding: 10px 14px;
  font-size: 14px;
  height: 36px;
}

.input-lg {
  padding: 12px 16px;
  font-size: 16px;
  height: 42px;
}

.input-xl {
  padding: 14px 18px;
  font-size: 18px;
  height: 48px;
}

/* Variant styles */
.input-outlined {
  border: 2px solid #d1d5db;
  border-radius: 6px;
  background-color: white;
}

.input-outlined:focus {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.input-filled {
  border: none;
  border-radius: 6px;
  background-color: #f3f4f6;
}

.input-filled:focus {
  background-color: #e5e7eb;
  box-shadow: 0 0 0 2px #3b82f6;
}

.input-underlined {
  border: none;
  border-bottom: 2px solid #d1d5db;
  border-radius: 0;
  background-color: transparent;
  padding-left: 0;
  padding-right: 0;
}

.input-underlined:focus {
  border-bottom-color: #3b82f6;
}

.input-ghost {
  border: 2px solid transparent;
  border-radius: 6px;
  background-color: transparent;
}

.input-ghost:hover {
  background-color: #f9fafb;
}

.input-ghost:focus {
  border-color: #3b82f6;
  background-color: white;
}

.input-flushed {
  border: none;
  border-bottom: 1px solid #d1d5db;
  border-radius: 0;
  background-color: transparent;
  padding-left: 0;
  padding-right: 0;
}

.input-flushed:focus {
  border-bottom-color: #3b82f6;
  border-bottom-width: 2px;
}

/* Status variants */
.input-success {
  border-color: #10b981;
}

.input-success:focus {
  border-color: #059669;
  box-shadow: 0 0 0 3px rgba(16, 185, 129, 0.1);
}

.input-warning {
  border-color: #f59e0b;
}

.input-warning:focus {
  border-color: #d97706;
  box-shadow: 0 0 0 3px rgba(245, 158, 11, 0.1);
}

.input-error {
  border-color: #ef4444;
}

.input-error:focus {
  border-color: #dc2626;
  box-shadow: 0 0 0 3px rgba(239, 68, 68, 0.1);
}

.input-loading {
  position: relative;
}

.input-loading::after {
  content: '';
  position: absolute;
  right: 10px;
  top: 50%;
  transform: translateY(-50%);
  width: 16px;
  height: 16px;
  border: 2px solid #e5e7eb;
  border-top: 2px solid #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: translateY(-50%) rotate(360deg); }
}

/* State modifiers */
.input-disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f9fafb;
}

.input-readonly {
  background-color: #f9fafb;
  cursor: default;
}

/* Help text and errors */
.input-help-text {
  font-size: 12px;
  color: #6b7280;
  margin-top: 4px;
}

.input-error-message {
  font-size: 12px;
  color: #ef4444;
  margin-top: 4px;
}

.input-counter {
  font-size: 12px;
  color: #9ca3af;
  text-align: right;
  margin-top: 4px;
}

/* Input group */
.input-group {
  display: flex;
  flex-direction: column;
  gap: 16px;
}

/* Clearable input */
.input-clearable {
  position: relative;
}

.input-clearable .clear-button {
  position: absolute;
  right: 8px;
  top: 50%;
  transform: translateY(-50%);
  background: none;
  border: none;
  cursor: pointer;
  padding: 4px;
  color: #9ca3af;
}

.input-clearable .clear-button:hover {
  color: #6b7280;
}
```

## Validation System

The Input widget includes a comprehensive validation system with built-in and custom validators:

```typescript
// Built-in validation rules
const validatedInput = input({
  id: 'validated-input',
  validation: {
    required: true,                    // Field is required
    minLength: 3,                     // Minimum 3 characters
    maxLength: 20,                    // Maximum 20 characters
    pattern: /^[a-zA-Z0-9_]+$/,      // Alphanumeric and underscore only
    custom: (value) => {              // Custom validation function
      if (value.includes('admin')) {
        return 'Username cannot contain "admin"';
      }
      return null; // null means valid
    }
  },
  onValidation: (isValid, errors) => {
    console.log('Validation result:', { isValid, errors });
    updateUIValidationState(isValid, errors);
  }
});

// Async validation example
const asyncValidatedInput = input({
  id: 'async-validated',
  validation: {
    required: true,
    custom: async (value) => {
      if (value.length < 3) return 'Too short';
      
      // Simulate API call
      const isAvailable = await checkAvailability(value);
      return isAvailable ? null : 'Already taken';
    }
  }
});
```

## Formatting System

The Input widget supports advanced formatting with masks and transformations:

```typescript
// Phone number formatting
const phoneInput = input({
  id: 'phone-formatted',
  formatting: {
    mask: '(000) 000-0000',    // Format template
    filter: /[0-9]/            // Allow only digits
  }
});

// Credit card formatting
const creditCardInput = input({
  id: 'credit-card',
  formatting: {
    mask: '0000 0000 0000 0000',
    filter: /[0-9]/,
    transform: 'uppercase'
  },
  validation: {
    pattern: /^\d{4} \d{4} \d{4} \d{4}$/,
    custom: (value) => {
      // Luhn algorithm validation
      const digits = value.replace(/\s/g, '');
      return validateCreditCard(digits) ? null : 'Invalid card number';
    }
  }
});

// Social Security Number formatting
const ssnInput = input({
  id: 'ssn',
  formatting: {
    mask: '000-00-0000',
    filter: /[0-9]/
  },
  validation: {
    pattern: /^\d{3}-\d{2}-\d{4}$/
  }
});

// Currency formatting
const currencyInput = input({
  id: 'price',
  formatting: {
    mask: '$0,000.00',
    filter: /[0-9.,]/
  },
  validation: {
    custom: (value) => {
      const amount = parseFloat(value.replace(/[$,]/g, ''));
      if (isNaN(amount) || amount < 0) {
        return 'Invalid amount';
      }
      if (amount > 999999) {
        return 'Amount too large';
      }
      return null;
    }
  }
});
```

## API Methods

The input function returns an object with these methods:

```typescript
const inputInstance = input(config);

// Set input value and trigger validation
inputInstance.setValue('new value');

// Get current value
const currentValue = inputInstance.getValue();

// Validate input manually
const validation = inputInstance.validate();
console.log('Is valid:', validation.isValid);
console.log('Errors:', validation.errors);

// Focus management
inputInstance.focus();
inputInstance.blur();

// Clear input (only works if clearable: true)
inputInstance.clear();

// Get DOM element
const element = inputInstance.build();
```

## Accessibility

The Input widget includes comprehensive accessibility features:

- ARIA attributes for screen readers
- Keyboard navigation support
- Focus management
- Label association
- Error announcement
- High contrast mode support

```typescript
const accessibleInput = input({
  id: 'accessible-input',
  label: 'Accessible input field',
  helpText: 'This input has full accessibility support',
  // Automatically includes:
  // - aria-labelledby for label association
  // - aria-describedby for help text
  // - aria-invalid for validation state
  // - proper focus management
  // - screen reader announcements
});
```

## Best Practices

1. **Accessibility**
   - Always provide meaningful labels for screen readers
   - Use appropriate input types (email, tel, url, etc.)
   - Implement proper error messaging
   - Ensure sufficient color contrast

2. **User Experience**
   - Use real-time validation for immediate feedback
   - Provide helpful placeholder text
   - Show character counters for length-limited fields
   - Use appropriate input variants for context

3. **Performance**
   - Debounce validation for expensive operations (API calls)
   - Use input formatting to guide user entry
   - Implement proper error recovery

4. **Form Design**
   - Group related inputs using inputGroup
   - Use consistent sizing and spacing
   - Provide clear help text for complex requirements

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const formContainer = new ElementBuilderImpl('div')
  .class('form-container')
  .child(
    textInput('username', {
      label: 'Username',
      validation: { required: true }
    })
  )
  .child(
    passwordInput('password', {
      label: 'Password'
    })
  )
  .build();
```

The Input widget provides comprehensive text input functionality with 8 convenience functions, real-time validation, custom formatting masks, dynamic methods, accessibility features, and extensive customization options for building robust forms and user interfaces.
