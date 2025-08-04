# Radio Widget

Radio button groups for single selection from multiple options with customizable styling and layout orientations.

## Overview

The Radio widget provides radio button group functionality with support for vertical and horizontal layouts, custom styling, and specialized patterns for common use cases like yes/no questions and ratings.

```typescript
import { radioGroup, RadioOption, RadioOrientation } from 'reactive-tui-ts'

const languageOptions: RadioOption[] = [
  { value: 'rust', label: 'Rust', enabled: true },
  { value: 'typescript', label: 'TypeScript', enabled: true },
  { value: 'python', label: 'Python', enabled: true }
]

const languageRadio = radioGroup({
  id: 'language-selection',
  options: languageOptions,
  selected: 'rust',
  interactive: true,
  orientation: RadioOrientation.Vertical
})
```

## Types

### RadioOrientation

```typescript
export enum RadioOrientation {
  Vertical = 'vertical',
  Horizontal = 'horizontal'
}
```

### RadioOption

```typescript
interface RadioOption {
  value: string
  label: string
  enabled?: boolean
  description?: string
}
```

### RadioConfig

```typescript
interface RadioConfig {
  id?: string
  options: RadioOption[]
  selected?: string
  interactive?: boolean
  orientation?: RadioOrientation
  selectedChar?: string
  unselectedChar?: string
  spacing?: number
  showLabels?: boolean
  classes?: string[]
}
```

### RadioState

```typescript
interface RadioState {
  selected: string | null
  interactive: boolean
  focusedIndex: number
}
```

## Basic Usage

### Simple Radio Group

```typescript
import { createRadioGroup } from 'reactive-tui-ts'

// Create from string array
const simpleRadio = createRadioGroup(
  'simple-radio',
  ['Option A', 'Option B', 'Option C'],
  'Option A' // Pre-selected value
)

// The radio group automatically creates RadioOption objects
// from the string array with matching value and label
```

### Custom Radio Group

```typescript
import { radioGroup, RadioOrientation } from 'reactive-tui-ts'

const customOptions: RadioOption[] = [
  { value: 'beginner', label: 'Beginner', enabled: true },
  { value: 'intermediate', label: 'Intermediate', enabled: true },
  { value: 'advanced', label: 'Advanced', enabled: true },
  { value: 'expert', label: 'Expert', enabled: false } // Disabled option
]

const skillLevelRadio = radioGroup({
  id: 'skill-level',
  options: customOptions,
  selected: 'intermediate',
  interactive: true,
  orientation: RadioOrientation.Vertical,
  selectedChar: '●',
  unselectedChar: '○',
  spacing: 1,
  showLabels: true,
  classes: ['skill-selector']
})
```

### Horizontal Radio Group

```typescript
import { createHorizontalRadioGroup } from 'reactive-tui-ts'

const horizontalOptions: RadioOption[] = [
  { value: 'small', label: 'Small', enabled: true },
  { value: 'medium', label: 'Medium', enabled: true },
  { value: 'large', label: 'Large', enabled: true }
]

const sizeRadio = createHorizontalRadioGroup(
  'size-selection',
  horizontalOptions,
  'medium'
)

// Results in: ○ Small  ● Medium  ○ Large
```

## Styling and Customization

### Custom Styled Radio Groups

```typescript
import { createCustomRadioGroup, RadioOrientation } from 'reactive-tui-ts'

// Unicode style
const unicodeRadio = createCustomRadioGroup({
  id: 'unicode-radio',
  options: [
    { value: 'option1', label: 'First Option', enabled: true },
    { value: 'option2', label: 'Second Option', enabled: true }
  ],
  selected: 'option1',
  style: 'unicode',
  orientation: RadioOrientation.Vertical
})
// Uses ◉ for selected, ◯ for unselected

// Symbols style
const symbolsRadio = createCustomRadioGroup({
  id: 'symbols-radio',
  options: [
    { value: 'yes', label: 'Accept', enabled: true },
    { value: 'no', label: 'Decline', enabled: true }
  ],
  style: 'symbols',
  orientation: RadioOrientation.Horizontal
})
// Uses ✓ for selected, ○ for unselected

// Default style
const defaultRadio = createCustomRadioGroup({
  id: 'default-radio',
  options: [
    { value: 'a', label: 'Choice A', enabled: true },
    { value: 'b', label: 'Choice B', enabled: true }
  ],
  style: 'default'
})
// Uses ● for selected, ○ for unselected
```

### Form-Style Radio Groups with Descriptions

```typescript
import { createFormRadioGroup } from 'reactive-tui-ts'

const formRadio = createFormRadioGroup({
  id: 'subscription-plan',
  title: 'Choose your subscription plan',
  options: [
    {
      value: 'basic',
      label: 'Basic Plan',
      description: '$9.99/month - Essential features'
    },
    {
      value: 'pro',
      label: 'Pro Plan',
      description: '$19.99/month - Advanced features'
    },
    {
      value: 'enterprise',
      label: 'Enterprise Plan',
      description: '$49.99/month - Full feature set'
    }
  ],
  selected: 'basic'
})

// Includes ARIA label and form-specific styling
```

### Compact Radio Groups

```typescript
import { createCompactRadioGroup } from 'reactive-tui-ts'

// Create compact radio without labels (just circles)
const compactRadio = createCompactRadioGroup(
  'compact-selector',
  5, // Number of options
  2  // Selected index (0-based)
)

// Results in: ○○●○○ (no labels, tight spacing)
```

## Specialized Radio Patterns

### Yes/No Radio

```typescript
import { createYesNoRadio } from 'reactive-tui-ts'

const confirmRadio = createYesNoRadio('confirm-action', true)
// Pre-selected to 'yes'

const neutralRadio = createYesNoRadio('neutral-choice')
// No pre-selection
```

### Rating Radio

```typescript
import { createRatingRadio } from 'reactive-tui-ts'

const ratingRadio = createRatingRadio('service-rating', 4)
// Creates 5-star rating, pre-selected to 4 stars
// Results in: ★★★★☆
```

### Disabled Radio Groups

```typescript
import { createDisabledRadioGroup } from 'reactive-tui-ts'

const readOnlyOptions: RadioOption[] = [
  { value: 'current', label: 'Current Selection', enabled: true },
  { value: 'other', label: 'Other Option', enabled: true }
]

const disabledRadio = createDisabledRadioGroup(
  'readonly-radio',
  readOnlyOptions,
  'current'
)

// Shows current selection but prevents user interaction
```

## Advanced Configuration

### Custom Characters and Spacing

```typescript
const customRadio = radioGroup({
  id: 'custom-chars',
  options: [
    { value: 'option1', label: 'First', enabled: true },
    { value: 'option2', label: 'Second', enabled: true },
    { value: 'option3', label: 'Third', enabled: true }
  ],
  selected: 'option2',
  selectedChar: '◉',      // Custom selected character
  unselectedChar: '◯',    // Custom unselected character
  spacing: 2,             // Space between character and label
  showLabels: true,       // Show option labels
  orientation: RadioOrientation.Vertical,
  interactive: true,
  classes: ['custom-radio', 'themed']
})
```

### Layout Customization

```typescript
// Vertical layout (default)
const verticalRadio = radioGroup({
  id: 'vertical-layout',
  options: [
    { value: 'a', label: 'Option A', enabled: true },
    { value: 'b', label: 'Option B', enabled: true },
    { value: 'c', label: 'Option C', enabled: true }
  ],
  orientation: RadioOrientation.Vertical
})
// Results in:
// ○ Option A
// ○ Option B  
// ○ Option C

// Horizontal layout
const horizontalRadio = radioGroup({
  id: 'horizontal-layout',
  options: [
    { value: 'x', label: 'X', enabled: true },
    { value: 'y', label: 'Y', enabled: true },
    { value: 'z', label: 'Z', enabled: true }
  ],
  orientation: RadioOrientation.Horizontal
})
// Results in: ○ X  ○ Y  ○ Z
```

### Hide Labels for Icon-Only Display

```typescript
const iconOnlyRadio = radioGroup({
  id: 'icon-only',
  options: [
    { value: 'small', label: 'Small Size', enabled: true },
    { value: 'medium', label: 'Medium Size', enabled: true },
    { value: 'large', label: 'Large Size', enabled: true }
  ],
  selected: 'medium',
  showLabels: false,      // Hide labels, show only radio buttons
  orientation: RadioOrientation.Horizontal,
  spacing: 0
})
// Results in: ○●○ (no text labels)
```

## Accessibility Features

### ARIA Support

```typescript
const accessibleRadio = radioGroup({
  id: 'accessible-radio',
  options: [
    { value: 'morning', label: 'Morning Shift', enabled: true },
    { value: 'afternoon', label: 'Afternoon Shift', enabled: true },
    { value: 'evening', label: 'Evening Shift', enabled: true }
  ],
  selected: 'morning',
  interactive: true
})

// Automatically includes:
// - role="radiogroup"
// - aria-activedescendant for selected option
// - Proper focus management
// - Keyboard navigation support
```

### Form Integration

```typescript
const formIntegratedRadio = createFormRadioGroup({
  id: 'form-radio',
  title: 'Preferred Contact Method',
  options: [
    {
      value: 'email',
      label: 'Email',
      description: 'Receive updates via email'
    },
    {
      value: 'phone',
      label: 'Phone',
      description: 'Receive calls during business hours'
    },
    {
      value: 'mail',
      label: 'Mail',
      description: 'Receive physical mail'
    }
  ],
  selected: 'email'
})

// Includes aria-label for screen readers and form semantics
```

## Complete Application Examples

### Settings Panel with Multiple Radio Groups

```typescript
import { 
  radioGroup, 
  createYesNoRadio, 
  createCustomRadioGroup, 
  createRatingRadio,
  RadioOrientation 
} from 'reactive-tui-ts'

class SettingsPanel {
  private themeRadio: any
  private notificationRadio: any
  private difficultyRadio: any
  private satisfactionRadio: any

  constructor() {
    this.setupRadioGroups()
  }

  private setupRadioGroups() {
    // Theme selection
    this.themeRadio = createCustomRadioGroup({
      id: 'theme-selector',
      options: [
        { value: 'light', label: 'Light Theme', enabled: true },
        { value: 'dark', label: 'Dark Theme', enabled: true },
        { value: 'auto', label: 'Auto (System)', enabled: true }
      ],
      selected: 'auto',
      style: 'unicode',
      orientation: RadioOrientation.Vertical
    })

    // Notification preferences
    this.notificationRadio = createYesNoRadio('notifications-enabled', true)

    // Difficulty level
    this.difficultyRadio = radioGroup({
      id: 'difficulty-level',
      options: [
        { value: 'easy', label: 'Easy', enabled: true },
        { value: 'normal', label: 'Normal', enabled: true },
        { value: 'hard', label: 'Hard', enabled: true },
        { value: 'expert', label: 'Expert', enabled: true }
      ],
      selected: 'normal',
      orientation: RadioOrientation.Horizontal,
      selectedChar: '●',
      unselectedChar: '○',
      classes: ['difficulty-selector']
    })

    // User satisfaction rating
    this.satisfactionRadio = createRatingRadio('user-satisfaction', 4)
  }

  getSelectedValues() {
    return {
      theme: this.getRadioValue(this.themeRadio),
      notifications: this.getRadioValue(this.notificationRadio) === 'yes',
      difficulty: this.getRadioValue(this.difficultyRadio),
      satisfaction: parseInt(this.getRadioValue(this.satisfactionRadio) || '0')
    }
  }

  private getRadioValue(radio: any): string | null {
    // In a real implementation, this would extract the selected value
    // from the radio group element or state management system
    return radio.getAttribute('data-selected') || null
  }

  applySettings() {
    const settings = this.getSelectedValues()
    
    console.log('Applied settings:', settings)
    
    // Apply theme
    document.body.className = `theme-${settings.theme}`
    
    // Configure notifications
    if (settings.notifications) {
      this.enableNotifications()
    } else {
      this.disableNotifications()
    }
    
    // Set difficulty
    this.setGameDifficulty(settings.difficulty)
    
    // Record satisfaction
    this.recordSatisfactionRating(settings.satisfaction)
  }

  private enableNotifications() {
    console.log('Notifications enabled')
  }

  private disableNotifications() {
    console.log('Notifications disabled')
  }

  private setGameDifficulty(difficulty: string) {
    console.log(`Game difficulty set to: ${difficulty}`)
  }

  private recordSatisfactionRating(rating: number) {
    console.log(`User satisfaction rating: ${rating}/5 stars`)
  }

  render() {
    return `
      <div class="settings-panel">
        <h2>Application Settings</h2>
        
        <div class="setting-group">
          <h3>Theme Preference</h3>
          ${this.themeRadio.build().toString()}
        </div>
        
        <div class="setting-group">
          <h3>Enable Notifications</h3>
          ${this.notificationRadio.build().toString()}
        </div>
        
        <div class="setting-group">
          <h3>Difficulty Level</h3>
          ${this.difficultyRadio.build().toString()}
        </div>
        
        <div class="setting-group">
          <h3>How satisfied are you with this app?</h3>
          ${this.satisfactionRadio.build().toString()}
        </div>
        
        <button onclick="this.applySettings()">Apply Settings</button>
      </div>
    `
  }
}

const settingsPanel = new SettingsPanel()
console.log(settingsPanel.render())
```

### Survey Form with Multiple Question Types

```typescript
import { 
  createFormRadioGroup, 
  createYesNoRadio, 
  createRatingRadio, 
  radioGroup,
  RadioOrientation 
} from 'reactive-tui-ts'

class SurveyForm {
  private questions: Map<string, any> = new Map()

  constructor() {
    this.setupSurveyQuestions()
  }

  private setupSurveyQuestions() {
    // Age group selection
    this.questions.set('age-group', createFormRadioGroup({
      id: 'age-group',
      title: 'What is your age group?',
      options: [
        { value: '18-24', label: '18-24 years old' },
        { value: '25-34', label: '25-34 years old' },
        { value: '35-44', label: '35-44 years old' },
        { value: '45-54', label: '45-54 years old' },
        { value: '55+', label: '55+ years old' }
      ]
    }))

    // Experience level
    this.questions.set('experience', radioGroup({
      id: 'experience-level',
      options: [
        { value: 'beginner', label: 'Beginner (0-1 years)', enabled: true },
        { value: 'intermediate', label: 'Intermediate (2-5 years)', enabled: true },
        { value: 'advanced', label: 'Advanced (6-10 years)', enabled: true },
        { value: 'expert', label: 'Expert (10+ years)', enabled: true }
      ],
      orientation: RadioOrientation.Vertical,
      interactive: true,
      classes: ['experience-question']
    }))

    // Yes/No questions
    this.questions.set('recommend', createYesNoRadio('would-recommend'))
    this.questions.set('return-customer', createYesNoRadio('return-customer'))

    // Satisfaction ratings
    this.questions.set('overall-satisfaction', createRatingRadio('overall-satisfaction'))
    this.questions.set('ease-of-use', createRatingRadio('ease-of-use'))
    this.questions.set('value-for-money', createRatingRadio('value-for-money'))

    // Frequency of use
    this.questions.set('usage-frequency', radioGroup({
      id: 'usage-frequency',
      options: [
        { value: 'daily', label: 'Daily', enabled: true },
        { value: 'weekly', label: 'Weekly', enabled: true },
        { value: 'monthly', label: 'Monthly', enabled: true },
        { value: 'rarely', label: 'Rarely', enabled: true },
        { value: 'first-time', label: 'First time user', enabled: true }
      ],
      orientation: RadioOrientation.Vertical,
      interactive: true
    }))
  }

  collectResponses(): Record<string, any> {
    const responses: Record<string, any> = {}
    
    for (const [key, question] of this.questions) {
      const value = this.getQuestionValue(question)
      if (value) {
        responses[key] = value
      }
    }
    
    return responses
  }

  private getQuestionValue(question: any): string | null {
    // Extract selected value from question element
    return question.getAttribute('data-selected') || null
  }

  validateSurvey(): { valid: boolean; missing: string[] } {
    const responses = this.collectResponses()
    const required = ['age-group', 'experience', 'recommend', 'overall-satisfaction']
    const missing = required.filter(key => !responses[key])
    
    return {
      valid: missing.length === 0,
      missing
    }
  }

  submitSurvey() {
    const validation = this.validateSurvey()
    
    if (!validation.valid) {
      console.log('Please complete all required questions:', validation.missing)
      return false
    }
    
    const responses = this.collectResponses()
    console.log('Survey responses:', responses)
    
    // Process responses
    this.analyzeSurveyData(responses)
    return true
  }

  private analyzeSurveyData(responses: Record<string, any>) {
    console.log('Survey Analysis:')
    
    // Age demographics
    console.log(`Age group: ${responses['age-group']}`)
    
    // Experience level
    console.log(`Experience level: ${responses['experience']}`)
    
    // Satisfaction metrics
    const satisfaction = parseInt(responses['overall-satisfaction'] || '0')
    const easeOfUse = parseInt(responses['ease-of-use'] || '0')
    const valueForMoney = parseInt(responses['value-for-money'] || '0')
    
    const averageRating = (satisfaction + easeOfUse + valueForMoney) / 3
    console.log(`Average satisfaction rating: ${averageRating.toFixed(1)}/5`)
    
    // Recommendations
    const wouldRecommend = responses['recommend'] === 'yes'
    const returnCustomer = responses['return-customer'] === 'yes'
    
    console.log(`Would recommend: ${wouldRecommend}`)
    console.log(`Would return: ${returnCustomer}`)
    
    // Usage patterns
    console.log(`Usage frequency: ${responses['usage-frequency']}`)
  }

  render() {
    return `
      <form class="survey-form">
        <h1>Customer Satisfaction Survey</h1>
        
        <div class="question-section">
          <h3>Demographics</h3>
          ${this.questions.get('age-group').build().toString()}
        </div>
        
        <div class="question-section">
          <h3>Experience Level</h3>
          <p>How would you describe your experience with our product?</p>
          ${this.questions.get('experience').build().toString()}
        </div>
        
        <div class="question-section">
          <h3>Satisfaction Ratings</h3>
          
          <div class="rating-question">
            <p>Overall satisfaction:</p>
            ${this.questions.get('overall-satisfaction').build().toString()}
          </div>
          
          <div class="rating-question">
            <p>Ease of use:</p>
            ${this.questions.get('ease-of-use').build().toString()}
          </div>
          
          <div class="rating-question">
            <p>Value for money:</p>
            ${this.questions.get('value-for-money').build().toString()}
          </div>
        </div>
        
        <div class="question-section">
          <h3>Recommendations</h3>
          
          <div class="yesno-question">
            <p>Would you recommend our product to others?</p>
            ${this.questions.get('recommend').build().toString()}
          </div>
          
          <div class="yesno-question">
            <p>Are you likely to use our product again?</p>
            ${this.questions.get('return-customer').build().toString()}
          </div>
        </div>
        
        <div class="question-section">
          <h3>Usage Patterns</h3>
          <p>How often do you use our product?</p>
          ${this.questions.get('usage-frequency').build().toString()}
        </div>
        
        <button type="button" onclick="this.submitSurvey()">Submit Survey</button>
      </form>
    `
  }
}

// Usage
const survey = new SurveyForm()
console.log(survey.render())

// Simulate form completion and submission
setTimeout(() => {
  const submitted = survey.submitSurvey()
  if (submitted) {
    console.log('Survey submitted successfully!')
  }
}, 5000)
```

## CSS Styling

```css
/* Radio group container */
.radio-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

.radio-horizontal {
  flex-direction: row;
  gap: 1rem;
}

.radio-vertical {
  flex-direction: column;
}

/* Radio options */
.radio-option {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  padding: 0.25rem;
  border-radius: 4px;
  transition: background-color 0.15s ease;
}

.radio-option:hover {
  background-color: #f8f9fa;
}

.radio-option:focus {
  outline: 2px solid #007bff;
  outline-offset: 2px;
}

/* Radio button characters */
.radio-selected {
  color: #007bff;
  font-weight: bold;
}

.radio-unselected {
  color: #6c757d;
}

/* Disabled state */
.radio-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.radio-disabled .radio-option {
  cursor: not-allowed;
}

.radio-disabled .radio-option:hover {
  background-color: transparent;
}

/* Custom styles */
.radio-custom {
  border: 1px solid #dee2e6;
  border-radius: 6px;
  padding: 1rem;
}

.radio-unicode .radio-selected {
  color: #28a745;
}

.radio-symbols .radio-selected {
  color: #ffc107;
}

/* Form-style radio groups */
.radio-form {
  background: #f8f9fa;
  border: 1px solid #e9ecef;
  border-radius: 6px;
  padding: 1rem;
}

.radio-form .radio-option {
  padding: 0.5rem;
  margin-bottom: 0.25rem;
}

.radio-form .radio-option:last-child {
  margin-bottom: 0;
}

/* Compact radio groups */
.radio-compact {
  gap: 0.25rem;
}

.radio-compact .radio-option {
  padding: 0.125rem;
  min-width: 1.5rem;
  justify-content: center;
}

/* Yes/No radio styling */
.radio-yesno {
  display: inline-flex;
  gap: 2rem;
  padding: 0.5rem;
  background: #e3f2fd;
  border-radius: 8px;
}

.radio-yesno .radio-option {
  font-weight: 500;
}

/* Rating radio styling */
.radio-rating {
  gap: 0.125rem;
}

.radio-rating .radio-option {
  font-size: 1.5rem;
  color: #ffc107;
  padding: 0.25rem;
}

.radio-rating .radio-unselected {
  color: #e9ecef;
}

/* Accessibility enhancements */
.radio-group[role="radiogroup"] {
  outline: none;
}

.radio-group:focus-within {
  outline: 2px solid #007bff;
  outline-offset: 2px;
  border-radius: 4px;
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .radio-option {
    border: 1px solid transparent;
  }
  
  .radio-option:focus {
    border-color: #000000;
    outline: none;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .radio-group {
    color: #e2e8f0;
  }
  
  .radio-option:hover {
    background-color: #374151;
  }
  
  .radio-unselected {
    color: #9ca3af;
  }
  
  .radio-form {
    background: #374151;
    border-color: #4b5563;
  }
  
  .radio-yesno {
    background: #1e3a8a;
  }
}

/* Animation for selection changes */
.radio-option {
  transition: all 0.2s ease;
}

.radio-selected {
  transform: scale(1.05);
}

/* Responsive design */
@media (max-width: 768px) {
  .radio-horizontal {
    flex-direction: column;
    gap: 0.5rem;
  }
  
  .radio-rating {
    gap: 0.5rem;
  }
  
  .radio-rating .radio-option {
    font-size: 1.25rem;
  }
}
```

## Best Practices

### 1. Use Appropriate Orientations

```typescript
// ✅ Good - vertical for many options or long labels
const detailedRadio = radioGroup({
  id: 'detailed-options',
  options: [
    { value: 'option1', label: 'Comprehensive Option A with detailed description', enabled: true },
    { value: 'option2', label: 'Comprehensive Option B with detailed description', enabled: true }
  ],
  orientation: RadioOrientation.Vertical
})

// ✅ Good - horizontal for few options or short labels
const compactRadio = radioGroup({
  id: 'compact-options',
  options: [
    { value: 'yes', label: 'Yes', enabled: true },
    { value: 'no', label: 'No', enabled: true }
  ],
  orientation: RadioOrientation.Horizontal
})
```

### 2. Provide Clear Visual Feedback

```typescript
// ✅ Good - distinctive selected/unselected characters
const clearRadio = createCustomRadioGroup({
  id: 'clear-feedback',
  options: [
    { value: 'option1', label: 'First Choice', enabled: true },
    { value: 'option2', label: 'Second Choice', enabled: true }
  ],
  style: 'unicode', // Uses ◉ and ◯ for clear distinction
  selected: 'option1'
})
```

### 3. Use Specialized Patterns for Common Cases

```typescript
// ✅ Good - use specific patterns for common scenarios
const confirmation = createYesNoRadio('user-confirmation', false)

const userRating = createRatingRadio('service-rating', 3)

const surveyQuestion = createFormRadioGroup({
  id: 'survey-q1',
  title: 'How did you hear about us?',
  options: [
    { value: 'social', label: 'Social Media' },
    { value: 'search', label: 'Search Engine' },
    { value: 'friend', label: 'Friend/Family' },
    { value: 'ad', label: 'Advertisement' }
  ]
})
```

### 4. Handle Disabled States Appropriately

```typescript
// ✅ Good - clear indication of disabled options
const conditionalRadio = radioGroup({
  id: 'conditional-options',
  options: [
    { value: 'available', label: 'Available Option', enabled: true },
    { value: 'premium', label: 'Premium Option (Upgrade Required)', enabled: false },
    { value: 'enterprise', label: 'Enterprise Option (Contact Sales)', enabled: false }
  ],
  selected: 'available',
  classes: ['conditional-selection']
})
```

### 5. Implement Proper Accessibility

```typescript
// ✅ Good - accessible radio groups
const accessibleRadio = createFormRadioGroup({
  id: 'accessible-survey',
  title: 'Required: Please select your preferred contact method',
  options: [
    { value: 'email', label: 'Email', description: 'We will send updates to your email address' },
    { value: 'phone', label: 'Phone', description: 'We will call during business hours' },
    { value: 'mail', label: 'Physical Mail', description: 'We will send letters to your address' }
  ]
})

// Includes proper ARIA attributes and semantic structure
```

## Related Widgets

- **[Checkbox](./checkbox)** - Multiple selection options
- **[Select](./select)** - Dropdown selection for many options
- **[Switch](./switch)** - Binary on/off toggle
- **[Button](./button)** - Action triggers and selection buttons

## Examples

- **[Radio Basics](../../examples/basic/radio-basic)** - Simple radio group examples
- **[Radio Styles](../../examples/basic/radio-styles)** - Different visual styles
- **[Survey Form](../../examples/advanced/survey-form)** - Complete survey implementation
- **[Settings Panel](../../examples/apps/settings-radio)** - Configuration interface
- **[Rating System](../../examples/patterns/rating-radio)** - Star rating implementation

The Radio widget provides comprehensive single-selection functionality with multiple layout options, styling patterns, and specialized components for common use cases like ratings and yes/no questions.