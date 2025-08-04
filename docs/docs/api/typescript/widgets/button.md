# Button Widget

The Button widget provides comprehensive button functionality with multiple variants, sizes, colors, shapes, states, icons, and advanced interactions. It supports loading states, accessibility features, custom styling, responsive layout integration, and includes convenience functions for common button patterns.

## Basic Usage

```typescript
import { button, primaryButton, iconButton, ButtonConfig } from 'reactive-tui';

// Basic button
const basicButton = button({
  id: 'my-button',
  text: 'Click Me',
  onClick: () => console.log('Button clicked!')
});

// Primary button with convenience function
const primary = primaryButton('submit-btn', 'Submit');

// Icon button
const iconBtn = iconButton('menu-btn', '☰');
```

## Types

### ButtonVariant

```typescript
export type ButtonVariant = 'filled' | 'outlined' | 'ghost' | 'link' | 'gradient'
```

### ButtonSize

```typescript
export type ButtonSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'
```

### ButtonColor

```typescript
export type ButtonColor = 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info' | 'neutral'
```

### ButtonShape

```typescript
export type ButtonShape = 'rectangle' | 'rounded' | 'pill' | 'circle' | 'square'
```

## Configuration

### ButtonIcon

```typescript
interface ButtonIcon {
  symbol: string
  position?: 'left' | 'right' | 'top' | 'bottom'
  size?: number
}
```

### ButtonConfig

```typescript
interface ButtonConfig {
  id: string
  variant?: ButtonVariant
  size?: ButtonSize
  color?: ButtonColor
  shape?: ButtonShape
  
  // Content
  text?: string
  icon?: ButtonIcon
  iconOnly?: boolean
  
  // State
  disabled?: boolean
  loading?: boolean
  active?: boolean
  
  // Behavior
  type?: 'button' | 'submit' | 'reset'
  fullWidth?: boolean
  autoFocus?: boolean
  
  // Styling
  width?: number | string
  height?: number | string
  cssClasses?: string[]
  style?: {
    backgroundColor?: string
    borderColor?: string
    textColor?: string
    hoverColor?: string
    activeColor?: string
    disabledColor?: string
  }
  
  // Events
  onClick?: (event?: any) => void
  onMouseEnter?: (event?: any) => void
  onMouseLeave?: (event?: any) => void
  onFocus?: (event?: any) => void
  onBlur?: (event?: any) => void
  onKeyPress?: (key: string, event?: any) => void
}
```

## Dynamic Button Methods

The button function returns an object with methods for runtime manipulation:

```typescript
const dynamicButton = button({
  id: 'dynamic-demo',
  text: 'Initial Text',
  variant: 'filled',
  color: 'primary'
});

// Change button text
dynamicButton.setText('Updated Text');

// Update icon
dynamicButton.setIcon({
  symbol: '✓',
  position: 'right'
});

// Toggle loading state
dynamicButton.setLoading(true);
setTimeout(() => dynamicButton.setLoading(false), 2000);

// Toggle disabled state
dynamicButton.setDisabled(true);

// Toggle active state
dynamicButton.setActive(true);

// Programmatically trigger click
dynamicButton.click();

// Focus management
dynamicButton.focus();
dynamicButton.blur();

// Add child elements
dynamicButton.child('Additional content');

// Get DOM element
const element = dynamicButton.build();
```

## Examples

### Basic Button

```typescript
import { button } from 'reactive-tui-ts'

const basicButton = button({
  id: 'basic-button',
  text: 'Click Me',
  onClick: () => {
    console.log('Button clicked!')
  }
})
```

### Button Variants

```typescript
const variantExamples = [
  button({
    id: 'filled-btn',
    text: 'Filled Button',
    variant: 'filled',
    color: 'primary'
  }),
  
  button({
    id: 'outlined-btn',
    text: 'Outlined Button',
    variant: 'outlined',
    color: 'primary'
  }),
  
  button({
    id: 'ghost-btn',
    text: 'Ghost Button',
    variant: 'ghost',
    color: 'primary'
  }),
  
  button({
    id: 'link-btn',
    text: 'Link Button',
    variant: 'link',
    color: 'primary'
  }),
  
  button({
    id: 'gradient-btn',
    text: 'Gradient Button',
    variant: 'gradient',
    color: 'primary'
  })
]
```

### Button Colors

```typescript
const colorExamples = [
  button({
    id: 'primary-btn',
    text: 'Primary',
    color: 'primary'
  }),
  
  button({
    id: 'secondary-btn',
    text: 'Secondary',
    color: 'secondary'
  }),
  
  button({
    id: 'success-btn',
    text: 'Success',
    color: 'success'
  }),
  
  button({
    id: 'warning-btn',
    text: 'Warning',
    color: 'warning'
  }),
  
  button({
    id: 'error-btn',
    text: 'Error',
    color: 'error'
  }),
  
  button({
    id: 'info-btn',
    text: 'Info',
    color: 'info'
  }),
  
  button({
    id: 'neutral-btn',
    text: 'Neutral',
    color: 'neutral'
  })
]
```

### Button Sizes

```typescript
const sizeExamples = [
  button({
    id: 'xs-btn',
    text: 'Extra Small',
    size: 'xs'
  }),
  
  button({
    id: 'sm-btn',
    text: 'Small',
    size: 'sm'
  }),
  
  button({
    id: 'md-btn',
    text: 'Medium',
    size: 'md'
  }),
  
  button({
    id: 'lg-btn',
    text: 'Large',
    size: 'lg'
  }),
  
  button({
    id: 'xl-btn',
    text: 'Extra Large',
    size: 'xl'
  })
]
```

### Button Shapes

```typescript
const shapeExamples = [
  button({
    id: 'rectangle-btn',
    text: 'Rectangle',
    shape: 'rectangle'
  }),
  
  button({
    id: 'rounded-btn',
    text: 'Rounded',
    shape: 'rounded'
  }),
  
  button({
    id: 'pill-btn',
    text: 'Pill',
    shape: 'pill'
  }),
  
  button({
    id: 'circle-btn',
    text: '●',
    shape: 'circle',
    iconOnly: true
  }),
  
  button({
    id: 'square-btn',
    text: '■',
    shape: 'square',
    iconOnly: true
  })
]
```

### Buttons with Icons

```typescript
const iconExamples = [
  button({
    id: 'icon-left',
    text: 'Save',
    icon: {
      symbol: '💾',
      position: 'left'
    }
  }),
  
  button({
    id: 'icon-right',
    text: 'Next',
    icon: {
      symbol: '→',
      position: 'right'
    }
  }),
  
  button({
    id: 'icon-top',
    text: 'Upload',
    icon: {
      symbol: '↑',
      position: 'top'
    }
  }),
  
  button({
    id: 'icon-bottom',
    text: 'Download',
    icon: {
      symbol: '↓',
      position: 'bottom'
    }
  }),
  
  button({
    id: 'icon-only',
    icon: {
      symbol: '✕'
    },
    iconOnly: true,
    shape: 'circle'
  })
]
```

### Loading Button

```typescript
const loadingButton = button({
  id: 'loading-btn',
  text: 'Processing...',
  loading: true,
  disabled: true,
  onClick: () => {
    // This won't be called while loading
  }
})

// Toggle loading state
const toggleLoadingButton = button({
  id: 'toggle-loading',
  text: 'Submit',
  onClick: async () => {
    toggleLoadingButton.setLoading(true)
    toggleLoadingButton.setText('Submitting...')
    
    try {
      await submitData()
      console.log('Data submitted successfully!')
    } catch (error) {
      console.error('Submission failed:', error)
    } finally {
      toggleLoadingButton.setLoading(false)
      toggleLoadingButton.setText('Submit')
    }
  }
})
```

### Active/Toggle Button

```typescript
const toggleButton = button({
  id: 'toggle-btn',
  text: 'Toggle Me',
  active: false,
  onClick: () => {
    const isActive = toggleButton.attributes['data-active'] === 'true'
    toggleButton.setActive(!isActive)
    toggleButton.setText(isActive ? 'Activate' : 'Deactivate')
  }
})
```

### Full Width Button

```typescript
const fullWidthButton = button({
  id: 'full-width',
  text: 'Full Width Button',
  fullWidth: true,
  size: 'lg',
  variant: 'filled',
  color: 'primary'
})
```

### Custom Styled Button

```typescript
const customButton = button({
  id: 'custom-btn',
  text: 'Custom Button',
  cssClasses: ['custom-button', 'special-effect'],
  style: {
    backgroundColor: '#6366f1',
    borderColor: '#4f46e5',
    textColor: '#ffffff',
    hoverColor: '#4338ca',
    activeColor: '#3730a3'
  },
  width: '200px',
  height: '50px'
})
```

### Form Buttons

```typescript
import { button, input } from 'reactive-tui-ts'

const formExample = {
  fields: [
    input({
      id: 'username',
      label: 'Username',
      type: 'text',
      required: true
    }),
    
    input({
      id: 'password',
      label: 'Password',
      type: 'password',
      required: true
    })
  ],
  
  buttons: [
    button({
      id: 'submit',
      text: 'Sign In',
      type: 'submit',
      variant: 'filled',
      color: 'primary',
      onClick: () => {
        console.log('Form submitted')
      }
    }),
    
    button({
      id: 'cancel',
      text: 'Cancel',
      type: 'button',
      variant: 'outlined',
      color: 'neutral',
      onClick: () => {
        console.log('Form cancelled')
      }
    }),
    
    button({
      id: 'reset',
      text: 'Reset',
      type: 'reset',
      variant: 'ghost',
      color: 'warning',
      onClick: () => {
        console.log('Form reset')
      }
    })
  ]
}
```

## Convenience Functions

The Button widget provides several convenience functions for common button types:

### Primary Button

```typescript
import { primaryButton } from 'reactive-tui-ts'

const primary = primaryButton('save-btn', 'Save Document', {
  size: 'lg',
  onClick: () => saveDocument()
})
```

### Secondary Button

```typescript
import { secondaryButton } from 'reactive-tui-ts'

const secondary = secondaryButton('cancel-btn', 'Cancel', {
  onClick: () => cancelAction()
})
```

### Danger Button

```typescript
import { dangerButton } from 'reactive-tui-ts'

const danger = dangerButton('delete-btn', 'Delete Item', {
  onClick: () => deleteItem()
})
```

### Success Button

```typescript
import { successButton } from 'reactive-tui-ts'

const success = successButton('complete-btn', 'Complete Task', {
  onClick: () => completeTask()
})
```

### Ghost Button

```typescript
import { ghostButton } from 'reactive-tui-ts'

const ghost = ghostButton('skip-btn', 'Skip Step', {
  onClick: () => skipStep()
})
```

### Link Button

```typescript
import { linkButton } from 'reactive-tui-ts'

const link = linkButton('learn-more', 'Learn More', {
  onClick: () => openHelp()
})
```

### Icon Button

```typescript
import { iconButton } from 'reactive-tui-ts'

const icon = iconButton('close-btn', '✕', {
  color: 'error',
  onClick: () => closeDialog()
})
```

### Loading Button

```typescript
import { loadingButton } from 'reactive-tui-ts'

const loading = loadingButton('process-btn', 'Processing', {
  color: 'primary'
})
```

## Button Group

```typescript
import { buttonGroup } from 'reactive-tui-ts'

const actionGroup = buttonGroup([
  primaryButton('save', 'Save'),
  secondaryButton('cancel', 'Cancel'),
  dangerButton('delete', 'Delete')
], {
  orientation: 'horizontal',
  spacing: 'md',
  justify: 'center'
})

const verticalGroup = buttonGroup([
  button({ id: 'option1', text: 'Option 1' }),
  button({ id: 'option2', text: 'Option 2' }),
  button({ id: 'option3', text: 'Option 3' })
], {
  orientation: 'vertical',
  spacing: 'sm'
})
```

## Action Buttons

### Submit Button

```typescript
import { submitButton } from 'reactive-tui-ts'

const submit = submitButton('form-submit', 'Submit Form', {
  size: 'lg',
  fullWidth: true
})
```

### Cancel Button

```typescript
import { cancelButton } from 'reactive-tui-ts'

const cancel = cancelButton('form-cancel', 'Cancel', {
  variant: 'ghost'
})
```

### Reset Button

```typescript
import { resetButton } from 'reactive-tui-ts'

const reset = resetButton('form-reset', 'Reset Form', {
  color: 'warning'
})
```

## Toggle Button

```typescript
import { toggleButton } from 'reactive-tui-ts'

const toggle = toggleButton('theme-toggle', 'Dark Mode', false, {
  color: 'info',
  icon: {
    symbol: '🌙',
    position: 'left'
  }
})

// Toggle state
const currentState = toggle.toggle()
console.log('Dark mode:', currentState)

// Get current state
const isActive = toggle.getState()

// Set state directly
toggle.setState(true)
```

## ResponsiveWidget Integration

The Button widget implements the ResponsiveWidget interface for layout system integration:

```typescript
// Button widget class for layout system integration
const buttonWidget = new ButtonWidget({
  id: 'responsive-btn',
  text: 'Responsive Button',
  variant: 'filled',
  color: 'primary',
  fullWidth: true
});

// Get element representation
const element = buttonWidget.toElement();

// Render with specific layout
const layoutRect = { x: 0, y: 0, width: 20, height: 3 };
const rendered = buttonWidget.renderWithLayout(layoutRect);
console.log(rendered); // "[  Click Me     ]"

// Size constraints
const [minWidth, minHeight] = buttonWidget.minSize(); // [10, 1]
const [maxWidth, maxHeight] = buttonWidget.maxSize(); // [30, 3]

// Growth behavior
const canGrowH = buttonWidget.canGrowHorizontal(); // true (if fullWidth)
const canGrowV = buttonWidget.canGrowVertical(); // false
```

## Real-World Examples

### Confirmation Dialog Buttons

```typescript
// Confirmation dialog buttons
class ConfirmationDialog {
  private confirmButton: any;
  private cancelButton: any;

  constructor(
    message: string,
    onConfirm: () => void,
    onCancel: () => void
  ) {
    this.confirmButton = dangerButton('confirm-action', 'Confirm', {
      onClick: () => {
        this.confirmButton.setLoading(true);
        onConfirm();
      }
    });

    this.cancelButton = secondaryButton('cancel-action', 'Cancel', {
      onClick: onCancel
    });
  }

  getButtons() {
    return buttonGroup([this.cancelButton, this.confirmButton], {
      orientation: 'horizontal',
      spacing: 'md',
      justify: 'end'
    });
  }

  setConfirmLoading(loading: boolean) {
    this.confirmButton.setLoading(loading);
    this.confirmButton.setDisabled(loading);
  }
}

// Usage
const dialog = new ConfirmationDialog(
  'Are you sure you want to delete this item?',
  async () => {
    try {
      await deleteItem();
      dialog.setConfirmLoading(false);
      closeDialog();
    } catch (error) {
      dialog.setConfirmLoading(false);
      showError('Failed to delete item');
    }
  },
  () => closeDialog()
);
```

### Document Toolbar

```typescript
class DocumentToolbar {
  private buttons: Map<string, any> = new Map();

  constructor() {
    this.setupButtons();
  }

  private setupButtons() {
    // File operations
    this.buttons.set('save', iconButton('save-doc', '💾', {
      onClick: () => this.saveDocument(),
      disabled: true // Initially disabled
    }));

    // Text formatting
    this.buttons.set('bold', toggleButton('bold', 'B', false, {
      onClick: () => this.toggleBold()
    }));
  }

  private async saveDocument() {
    this.setButtonLoading('save', true);
    try {
      await saveCurrentDocument();
      this.buttons.get('save')?.setIcon({ symbol: '✓', position: 'left' });
      setTimeout(() => {
        this.buttons.get('save')?.setIcon({ symbol: '💾', position: 'left' });
      }, 1000);
    } catch (error) {
      this.buttons.get('save')?.setIcon({ symbol: '⚠', position: 'left' });
    } finally {
      this.setButtonLoading('save', false);
    }
  }

  private setButtonLoading(id: string, loading: boolean) {
    this.buttons.get(id)?.setLoading(loading);
  }

  getToolbar() {
    return buttonGroup([
      this.buttons.get('save'),
      { tag: 'div', classes: ['toolbar-separator'], content: '|', children: [] },
      this.buttons.get('bold')
    ], {
      orientation: 'horizontal',
      spacing: 'sm',
      justify: 'start'
    });
  }
}
```

### Multi-Step Form Navigation

```typescript
class MultiStepForm {
  private currentStep = 1;
  private totalSteps = 4;
  private buttons: { prev: any; next: any; submit: any; };

  constructor() {
    this.setupNavigation();
  }

  private setupNavigation() {
    this.buttons = {
      prev: secondaryButton('prev-step', 'Previous', {
        icon: { symbol: '←', position: 'left' },
        onClick: () => this.previousStep(),
        disabled: true // First step
      }),

      next: primaryButton('next-step', 'Next', {
        icon: { symbol: '→', position: 'right' },
        onClick: () => this.nextStep()
      }),

      submit: successButton('submit-form', 'Submit Form', {
        icon: { symbol: '✓', position: 'left' },
        onClick: () => this.submitForm()
      })
    };
  }

  private async nextStep() {
    this.buttons.next.setLoading(true);
    
    try {
      const isValid = await validateStep(this.currentStep);
      if (!isValid) {
        throw new Error('Please complete all required fields');
      }

      this.currentStep++;
      this.updateNavigation();
    } catch (error) {
      showValidationError(error.message);
    } finally {
      this.buttons.next.setLoading(false);
    }
  }

  private async submitForm() {
    this.buttons.submit.setLoading(true);
    this.buttons.submit.setText('Submitting...');

    try {
      await submitFormData();
      this.buttons.submit.setText('Success!');
      this.buttons.submit.setIcon({ symbol: '✅', position: 'left' });
    } catch (error) {
      this.buttons.submit.setText('Retry Submit');
      this.buttons.submit.setIcon({ symbol: '⚠', position: 'left' });
      this.buttons.submit.setLoading(false);
    }
  }

  private updateNavigation() {
    // Update Previous button
    this.buttons.prev.setDisabled(this.currentStep === 1);

    // Update step indicator in Next button
    this.buttons.next.setText(`Next (${this.currentStep + 1}/${this.totalSteps})`);
  }

  getNavigation() {
    return buttonGroup([
      this.buttons.prev,
      this.buttons.next,
      this.buttons.submit
    ], {
      orientation: 'horizontal',
      spacing: 'md',
      justify: 'space-between'
    });
  }
}
```

## CSS Styling

```css
/* Base button styles */
.button {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  gap: 8px;
  border-radius: 6px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.2s ease;
  border: 1px solid transparent;
  text-decoration: none;
  user-select: none;
  font-family: inherit;
}

/* Size variants */
.button-xs {
  padding: 4px 8px;
  font-size: 12px;
  min-height: 24px;
}

.button-sm {
  padding: 6px 12px;
  font-size: 14px;
  min-height: 32px;
}

.button-md {
  padding: 8px 16px;
  font-size: 16px;
  min-height: 40px;
}

.button-lg {
  padding: 12px 20px;
  font-size: 18px;
  min-height: 48px;
}

.button-xl {
  padding: 16px 24px;
  font-size: 20px;
  min-height: 56px;
}

/* Variant styles */
.button-filled {
  background-color: var(--button-bg, #3b82f6);
  color: var(--button-text, white);
  border-color: var(--button-bg, #3b82f6);
}

.button-filled:hover {
  background-color: var(--button-hover, #2563eb);
  border-color: var(--button-hover, #2563eb);
}

.button-outlined {
  background-color: transparent;
  color: var(--button-border, #3b82f6);
  border-color: var(--button-border, #3b82f6);
}

.button-outlined:hover {
  background-color: var(--button-border, #3b82f6);
  color: white;
}

.button-ghost {
  background-color: transparent;
  color: var(--button-text, #3b82f6);
  border-color: transparent;
}

.button-ghost:hover {
  background-color: var(--button-hover-bg, rgba(59, 130, 246, 0.1));
}

.button-link {
  background-color: transparent;
  color: var(--button-text, #3b82f6);
  border-color: transparent;
  text-decoration: underline;
  padding-left: 0;
  padding-right: 0;
}

.button-link:hover {
  color: var(--button-hover, #2563eb);
}

.button-gradient {
  background: linear-gradient(135deg, var(--gradient-from, #3b82f6), var(--gradient-to, #1d4ed8));
  color: white;
  border: none;
}

.button-gradient:hover {
  transform: translateY(-1px);
  box-shadow: 0 4px 12px rgba(59, 130, 246, 0.4);
}

/* Shape variants */
.button-rectangle {
  border-radius: 0;
}

.button-rounded {
  border-radius: 6px;
}

.button-pill {
  border-radius: 50px;
}

.button-circle {
  border-radius: 50%;
  width: var(--button-size, 40px);
  height: var(--button-size, 40px);
  padding: 0;
}

.button-square {
  border-radius: 6px;
  width: var(--button-size, 40px);
  height: var(--button-size, 40px);
  padding: 0;
}

/* Color variants */
.button-primary {
  --button-bg: #3b82f6;
  --button-hover: #2563eb;
  --button-border: #3b82f6;
  --button-text: #3b82f6;
}

.button-secondary {
  --button-bg: #6b7280;
  --button-hover: #4b5563;
  --button-border: #6b7280;
  --button-text: #6b7280;
}

.button-success {
  --button-bg: #10b981;
  --button-hover: #059669;
  --button-border: #10b981;
  --button-text: #10b981;
}

.button-warning {
  --button-bg: #f59e0b;
  --button-hover: #d97706;
  --button-border: #f59e0b;
  --button-text: #f59e0b;
}

.button-error {
  --button-bg: #ef4444;
  --button-hover: #dc2626;
  --button-border: #ef4444;
  --button-text: #ef4444;
}

.button-info {
  --button-bg: #06b6d4;
  --button-hover: #0891b2;
  --button-border: #06b6d4;
  --button-text: #06b6d4;
}

.button-neutral {
  --button-bg: #64748b;
  --button-hover: #475569;
  --button-border: #64748b;
  --button-text: #64748b;
}

/* State modifiers */
.button-disabled {
  opacity: 0.5;
  cursor: not-allowed;
  pointer-events: none;
}

.button-loading {
  position: relative;
  pointer-events: none;
}

.button-loading .button-spinner {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.button-active {
  transform: scale(0.95);
  box-shadow: inset 0 2px 4px rgba(0, 0, 0, 0.1);
}

.button-full-width {
  width: 100%;
}

/* Icon positioning */
.button-icon-left {
  order: -1;
}

.button-icon-right {
  order: 1;
}

.button-icon-top {
  order: -1;
}

.button-icon-bottom {
  order: 1;
}

.button-icon-only {
  min-width: var(--button-size, 40px);
}

/* Focus styles */
.button:focus-visible {
  outline: 2px solid var(--focus-color, #3b82f6);
  outline-offset: 2px;
}

/* Button group */
.button-group {
  display: flex;
  gap: 8px;
}

.button-group-horizontal {
  flex-direction: row;
}

.button-group-vertical {
  flex-direction: column;
}

.button-group-spacing-none {
  gap: 0;
}

.button-group-spacing-sm {
  gap: 4px;
}

.button-group-spacing-md {
  gap: 8px;
}

.button-group-spacing-lg {
  gap: 16px;
}

.button-group-wrap {
  flex-wrap: wrap;
}

.button-group-justify-start {
  justify-content: flex-start;
}

.button-group-justify-center {
  justify-content: center;
}

.button-group-justify-end {
  justify-content: flex-end;
}

.button-group-justify-space-between {
  justify-content: space-between;
}

.button-group-justify-space-around {
  justify-content: space-around;
}
```

## API Methods

The button function returns an object with these methods:

```typescript
const buttonInstance = button(config)

// Update button text
buttonInstance.setText('New Text')

// Set icon
buttonInstance.setIcon({
  symbol: '✓',
  position: 'left'
})

// Toggle loading state
buttonInstance.setLoading(true)

// Toggle disabled state
buttonInstance.setDisabled(true)

// Toggle active state
buttonInstance.setActive(true)

// Programmatically click
buttonInstance.click()

// Focus button
buttonInstance.focus()

// Blur button
buttonInstance.blur()

// Add child element
buttonInstance.child('Additional content')

// Get DOM element
const element = buttonInstance.build()
```

## Best Practices

1. **Accessibility**
   - Always provide meaningful text for screen readers
   - Use proper button types (`submit`, `reset`)
   - Implement keyboard navigation
   - Use appropriate color contrast

2. **User Experience**
   - Use loading states for async operations
   - Provide visual feedback for user actions
   - Disable buttons when actions are unavailable
   - Use consistent sizing and spacing

3. **Performance**
   - Minimize re-renders by batching state changes
   - Use icon-only buttons for space-constrained interfaces
   - Implement proper event cleanup

4. **Styling**
   - Follow consistent design system colors and sizes
   - Use semantic button types (primary, secondary, danger)
   - Maintain visual hierarchy with button variants

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const buttonContainer = new ElementBuilderImpl('div')
  .class('button-container')
  .child(
    primaryButton('action-btn', 'Primary Action')
  )
  .child(
    secondaryButton('cancel-btn', 'Cancel')
  )
  .build();
```

## Accessibility

The Button widget includes comprehensive accessibility features:

- ARIA attributes for screen readers
- Keyboard navigation support
- Focus management
- High contrast mode support
- Proper semantic roles

```typescript
const accessibleButton = button({
  id: 'accessible-button',
  text: 'Accessible button',
  // Automatically includes:
  // - role="button"
  // - tabindex="0"
  // - aria-label or aria-labelledby
  // - keyboard event handling
})
```

The Button widget provides comprehensive button functionality with extensive customization options, state management, event handling, and responsive layout integration for building rich terminal user interfaces.
