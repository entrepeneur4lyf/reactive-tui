# Checkbox Widget

The Checkbox widget provides comprehensive checkbox functionality with single checkboxes, checkbox groups, smooth scaling animations, custom character styles, and extensive configuration options. It includes convenience functions for common patterns and full accessibility support.

## Basic Usage

```typescript
import { 
  checkboxWidget, 
  checkboxGroupWidget, 
  createSimpleCheckbox,
  createAnimatedCheckbox,
  CheckboxStyle 
} from 'reactive-tui';

// Basic checkbox
const basicCheckbox = checkboxWidget({
  id: 'terms-agreement',
  label: 'I agree to the terms and conditions',
  checked: false,
  style: CheckboxStyle.Ballot
});

// Simple checkbox with convenience function
const simple = createSimpleCheckbox({
  id: 'newsletter',
  label: 'Subscribe to newsletter',
  checked: false
});

// Animated checkbox
const animated = createAnimatedCheckbox({
  id: 'animated',
  label: 'Animated checkbox',
  checked: false,
  duration: 300,
  scaleFactor: 1.8
});
```

## Types

### CheckboxStyle

```typescript
export enum CheckboxStyle {
  Ballot = 'ballot',    // ☐ ☑
  Square = 'square',    // [ ] [x]
  Round = 'round',      // ( ) (x)
  Custom = 'custom'     // User-defined characters
}
```

### CheckboxLabelPosition

```typescript
export enum CheckboxLabelPosition {
  Before = 'before',
  After = 'after',
  Above = 'above',
  Below = 'below',
  None = 'none'
}
```

### CheckboxGroupOrientation

```typescript
export enum CheckboxGroupOrientation {
  Vertical = 'vertical',
  Horizontal = 'horizontal'
}
```

### CheckboxAnimationState

```typescript
export enum CheckboxAnimationState {
  Idle = 'idle',
  CheckingIn = 'checking-in',
  CheckingOut = 'checking-out'
}
```

## Configuration

### CheckboxConfig

```typescript
interface CheckboxConfig {
  id?: string;                          // Unique checkbox identifier
  label?: string;                       // Text label for the checkbox
  checked?: boolean;                    // Checked state (default: false)
  enabled?: boolean;                    // Enabled state (default: true)
  visible?: boolean;                    // Visibility state (default: true)
  style?: CheckboxStyle;                // Visual style (default: Ballot)
  labelPosition?: CheckboxLabelPosition; // Label positioning (default: After)
  spacing?: number;                     // Space between checkbox and label (default: 1)
  customChars?: { unchecked: string; checked: string }; // Custom characters for Custom style
  animationConfig?: CheckboxAnimationConfig; // Animation settings
  animationState?: CheckboxAnimationState;   // Current animation state
  classes?: string[];                   // Additional CSS classes
}
```

### CheckboxAnimationConfig

```typescript
interface CheckboxAnimationConfig {
  enabled: boolean
  duration: number      // milliseconds
  easing: string       // CSS easing function
  scaleFactor: number  // How much larger the initial checkmark is
}
```

### CheckboxGroupConfig

```typescript
interface CheckboxGroupConfig {
  id?: string
  label?: string
  options: CheckboxOption[]
  style?: CheckboxStyle
  orientation?: CheckboxGroupOrientation
  spacing?: number
  enabled?: boolean
  visible?: boolean
  selectedValues?: string[]
  customChars?: { unchecked: string; checked: string }
  classes?: string[]
  
  // Events
  onChange?: (selectedValues: string[], event?: any) => void
}
```

### CheckboxOption

```typescript
interface CheckboxOption {
  id: string
  label: string
  value: string
  checked?: boolean
  enabled?: boolean
}
```

## Convenience Functions

The Checkbox widget provides several convenience functions for common patterns:

### createSimpleCheckbox

Creates a checkbox with default animation settings:

```typescript
function createSimpleCheckbox(config: {
  id: string;
  label: string;
  checked?: boolean;
  classes?: string[];
}): ElementBuilder
```

### createAnimatedCheckbox

Creates a checkbox with custom animation configuration:

```typescript
function createAnimatedCheckbox(config: {
  id: string;
  label: string;
  checked?: boolean;
  duration?: number;         // Animation duration in milliseconds
  scaleFactor?: number;      // Scale factor for animation
  classes?: string[];
  animationConfig?: CheckboxAnimationConfig;
}): ElementBuilder
```

### createCustomCheckbox

Creates a checkbox with custom characters:

```typescript
function createCustomCheckbox(config: {
  id: string;
  label: string;
  unchecked: string;        // Character for unchecked state
  checkedChar: string;      // Character for checked state
  checked?: boolean;
}): ElementBuilder
```

### createCheckboxGroup

Creates a checkbox group with default settings:

```typescript
function createCheckboxGroup(config: {
  id: string;
  label: string;
  options: Array<{ label: string; value: string; checked?: boolean }>;
  orientation?: CheckboxGroupOrientation;
  classes?: string[];
}): ElementBuilder
```

### createHorizontalCheckboxGroup

Creates a horizontal checkbox group:

```typescript
function createHorizontalCheckboxGroup(config: {
  id: string;
  label: string;
  options: Array<{ label: string; value: string; checked?: boolean }>;
}): ElementBuilder
```

### createFeatureCheckboxGroup

Creates a feature selection checkbox group:

```typescript
function createFeatureCheckboxGroup(
  id: string, 
  features: string[]
): ElementBuilder
```

### createMultiSelectCheckboxGroup

Creates a checkbox group with pre-selected values:

```typescript
function createMultiSelectCheckboxGroup(config: {
  id: string;
  label: string;
  options: Array<{ label: string; value: string }>;
  selectedValues: string[];
}): ElementBuilder
```

## Examples

### Basic Checkbox

```typescript
import { checkboxWidget, createSimpleCheckbox, CheckboxStyle } from 'reactive-tui'

// Using checkboxWidget function
const subscribeCheckbox = checkboxWidget({
  id: 'newsletter-subscribe',
  label: 'Subscribe to newsletter',
  checked: false,
  style: CheckboxStyle.Ballot,
  labelPosition: CheckboxLabelPosition.After
});

// Using convenience function
const simpleCheckbox = createSimpleCheckbox({
  id: 'terms-agree',
  label: 'I agree to the terms and conditions',
  checked: false
});
```

### Animated Checkbox

```typescript
import { createAnimatedCheckbox, checkboxWidget, CheckboxAnimationState } from 'reactive-tui'

// Using convenience function with custom animation
const animatedCheckbox = createAnimatedCheckbox({
  id: 'animated-checkbox',
  label: 'Animated checkbox',
  checked: false,
  duration: 300,
  scaleFactor: 1.8
});

// Manual animation configuration
const customAnimatedCheckbox = checkboxWidget({
  id: 'custom-animated',
  label: 'Custom animated checkbox',
  checked: false,
  animationConfig: {
    enabled: true,
    duration: 500,
    easing: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)', // Bouncy easing
    scaleFactor: 2.2
  },
  animationState: CheckboxAnimationState.Idle
});

// Different animation presets
const subtleAnimation = createAnimatedCheckbox({
  id: 'subtle',
  label: 'Subtle animation',
  duration: 200,
  scaleFactor: 1.2
});

const bouncyAnimation = createAnimatedCheckbox({
  id: 'bouncy',
  label: 'Bouncy animation',
  duration: 500,
  scaleFactor: 2.2,
  animationConfig: {
    enabled: true,
    duration: 500,
    easing: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)',
    scaleFactor: 2.2
  }
});
```

### Different Styles

```typescript
import { checkboxWidget, createCustomCheckbox, CheckboxStyle } from 'reactive-tui'

// Ballot style (default) - Unicode ballot symbols
const ballotStyle = checkboxWidget({
  id: 'ballot-style',
  label: 'Ballot style (☐ ☑)',
  style: CheckboxStyle.Ballot,
  checked: true
});

// Square style - ASCII square brackets
const squareStyle = checkboxWidget({
  id: 'square-style',
  label: 'Square style ([ ] [x])',
  style: CheckboxStyle.Square,
  checked: false
});

// Round style - ASCII parentheses
const roundStyle = checkboxWidget({
  id: 'round-style',
  label: 'Round style (( ) (x))',
  style: CheckboxStyle.Round,
  checked: true
});

// Custom style using convenience function
const customStyle1 = createCustomCheckbox({
  id: 'custom-circles',
  label: 'Custom circles',
  unchecked: '○',
  checkedChar: '●',
  checked: false
});

// More custom styles
const customStyle2 = createCustomCheckbox({
  id: 'custom-stars',
  label: 'Custom stars',
  unchecked: '☆',
  checkedChar: '★',
  checked: true
});

const customStyle3 = createCustomCheckbox({
  id: 'custom-arrows',
  label: 'Custom arrows',
  unchecked: '▷',
  checkedChar: '▶',
  checked: false
});

// Custom with emojis
const emojiStyle = createCustomCheckbox({
  id: 'emoji-style',
  label: 'Emoji style',
  unchecked: '⚪',
  checkedChar: '🟢',
  checked: true
});
```

### Checkbox Group

```typescript
import { 
  checkboxGroupWidget, 
  createCheckboxGroup,
  createFeatureCheckboxGroup,
  CheckboxGroupOrientation 
} from 'reactive-tui'

// Using convenience function
const featuresGroup = createCheckboxGroup({
  id: 'app-features',
  label: 'Select Features',
  options: [
    { label: 'Dark Mode', value: 'dark-mode', checked: true },
    { label: 'Push Notifications', value: 'notifications', checked: false },
    { label: 'Auto Save', value: 'auto-save', checked: true },
    { label: 'Cloud Sync', value: 'sync', checked: false }
  ],
  orientation: CheckboxGroupOrientation.Vertical
});

// Using feature convenience function
const features = createFeatureCheckboxGroup('features', [
  'Real-time Collaboration',
  'Advanced Analytics',
  'API Access',
  'Priority Support',
  'Custom Integrations'
]);

// Manual checkbox group with detailed configuration
const detailedGroup = checkboxGroupWidget({
  id: 'detailed-features',
  label: 'Application Features',
  orientation: CheckboxGroupOrientation.Vertical,
  spacing: 1,
  options: [
    {
      id: 'dark-mode',
      label: 'Dark Mode',
      value: 'dark-mode',
      checked: true,
      enabled: true
    },
    {
      id: 'notifications',
      label: 'Push Notifications',
      value: 'notifications',
      checked: false,
      enabled: true
    },
    {
      id: 'sync',
      label: 'Cloud Sync (Coming Soon)',
      value: 'sync',
      checked: false,
      enabled: false
    }
  ],
  selectedValues: ['dark-mode']
});
```

### Horizontal Checkbox Group

```typescript
import { createHorizontalCheckboxGroup, createMultiSelectCheckboxGroup } from 'reactive-tui'

// Using convenience function
const priorityGroup = createHorizontalCheckboxGroup({
  id: 'task-priority',
  label: 'Task Priorities',
  options: [
    { label: 'Low', value: 'low', checked: false },
    { label: 'Medium', value: 'medium', checked: true },
    { label: 'High', value: 'high', checked: false },
    { label: 'Urgent', value: 'urgent', checked: false }
  ]
});

// Multi-select with pre-selected values
const selectedCategories = createMultiSelectCheckboxGroup({
  id: 'categories',
  label: 'Content Categories',
  options: [
    { label: 'Technology', value: 'tech' },
    { label: 'Design', value: 'design' },
    { label: 'Business', value: 'business' },
    { label: 'Science', value: 'science' },
    { label: 'Art', value: 'art' }
  ],
  selectedValues: ['tech', 'design'] // Pre-selected
});

// Manual horizontal configuration
const manualHorizontal = checkboxGroupWidget({
  id: 'manual-horizontal',
  label: 'Filter Options',
  orientation: CheckboxGroupOrientation.Horizontal,
  spacing: 2,
  options: [
    { id: 'active', label: 'Active', value: 'active', checked: true },
    { id: 'completed', label: 'Completed', value: 'completed', checked: false },
    { id: 'archived', label: 'Archived', value: 'archived', checked: false }
  ]
});
```

## Real-World Examples

### Registration Form with Validation

```typescript
import { 
  checkboxWidget, 
  createCheckboxGroup, 
  createSimpleCheckbox,
  CheckboxLabelPosition 
} from 'reactive-tui'

class RegistrationForm {
  private termsCheckbox: any;
  private privacyCheckbox: any;
  private marketingCheckbox: any;
  private interestsGroup: any;
  private isValid = false;

  constructor() {
    this.setupForm();
  }

  private setupForm() {
    // Required agreement checkboxes
    this.termsCheckbox = createSimpleCheckbox({
      id: 'terms',
      label: 'I agree to the Terms of Service',
      checked: false,
      classes: ['required-checkbox']
    });

    this.privacyCheckbox = createSimpleCheckbox({
      id: 'privacy',
      label: 'I agree to the Privacy Policy',
      checked: false,
      classes: ['required-checkbox']
    });

    // Optional marketing checkbox
    this.marketingCheckbox = checkboxWidget({
      id: 'marketing',
      label: 'I want to receive marketing emails and updates',
      checked: false,
      labelPosition: CheckboxLabelPosition.After,
      classes: ['optional-checkbox']
    });

    // Interests selection
    this.interestsGroup = createCheckboxGroup({
      id: 'interests',
      label: 'Select your interests (optional)',
      options: [
        { label: 'Technology & Programming', value: 'tech' },
        { label: 'Design & Creativity', value: 'design' },
        { label: 'Business & Entrepreneurship', value: 'business' },
        { label: 'Science & Research', value: 'science' },
        { label: 'Arts & Entertainment', value: 'arts' }
      ]
    });
  }

  validateForm(): boolean {
    const termsAccepted = this.getCheckboxState(this.termsCheckbox);
    const privacyAccepted = this.getCheckboxState(this.privacyCheckbox);
    
    this.isValid = termsAccepted && privacyAccepted;
    
    // Update UI based on validation
    this.updateValidationUI();
    
    return this.isValid;
  }

  private getCheckboxState(checkbox: any): boolean {
    return checkbox.getAttribute('data-checked') === 'true';
  }

  private updateValidationUI() {
    const submitButton = document.getElementById('submit-button');
    if (submitButton) {
      submitButton.disabled = !this.isValid;
    }
  }

  getFormData() {
    return {
      termsAccepted: this.getCheckboxState(this.termsCheckbox),
      privacyAccepted: this.getCheckboxState(this.privacyCheckbox),
      marketingOptIn: this.getCheckboxState(this.marketingCheckbox),
      interests: this.getSelectedInterests()
    };
  }

  private getSelectedInterests(): string[] {
    const selectedValues = this.interestsGroup.getAttribute('data-selected-values');
    return selectedValues ? selectedValues.split(',') : [];
  }
}

// Usage
const registrationForm = new RegistrationForm();

// Validate on any checkbox change
document.addEventListener('change', (event) => {
  if (event.target?.classList.contains('checkbox')) {
    registrationForm.validateForm();
  }
});
```

### Settings Panel with Categories

```typescript
class ApplicationSettings {
  private generalSettings: any;
  private notificationSettings: any;
  private privacySettings: any;
  private advancedSettings: any;

  constructor() {
    this.setupSettingsCategories();
  }

  private setupSettingsCategories() {
    // General settings
    this.generalSettings = createCheckboxGroup({
      id: 'general-settings',
      label: 'General Settings',
      options: [
        { label: 'Enable dark mode', value: 'dark_mode', checked: true },
        { label: 'Auto-save documents', value: 'auto_save', checked: true },
        { label: 'Show welcome screen on startup', value: 'welcome_screen', checked: false },
        { label: 'Check for updates automatically', value: 'auto_update', checked: true }
      ]
    });

    // Notification settings
    this.notificationSettings = createCheckboxGroup({
      id: 'notification-settings',
      label: 'Notification Preferences',
      options: [
        { label: 'Desktop notifications', value: 'desktop_notifications', checked: true },
        { label: 'Email notifications', value: 'email_notifications', checked: false },
        { label: 'Sound alerts', value: 'sound_alerts', checked: true },
        { label: 'Push notifications', value: 'push_notifications', checked: false }
      ]
    });

    // Privacy settings
    this.privacySettings = createCheckboxGroup({
      id: 'privacy-settings',
      label: 'Privacy & Data',
      options: [
        { label: 'Share usage analytics', value: 'analytics', checked: false },
        { label: 'Allow crash reporting', value: 'crash_reporting', checked: true },
        { label: 'Enable location services', value: 'location', checked: false },
        { label: 'Share data for improvements', value: 'data_sharing', checked: false }
      ]
    });

    // Advanced settings (initially disabled)
    this.advancedSettings = checkboxGroupWidget({
      id: 'advanced-settings',
      label: 'Advanced Features (Beta)',
      enabled: false,
      options: [
        { id: 'ai-assist', label: 'AI Assistant', value: 'ai_assistant', enabled: false },
        { id: 'experimental', label: 'Experimental features', value: 'experimental', enabled: false },
        { id: 'developer-mode', label: 'Developer mode', value: 'developer_mode', enabled: false }
      ]
    });
  }

  enableAdvancedSettings(enable: boolean) {
    this.advancedSettings.enabled = enable;
    
    // Update visual state
    const advancedElement = document.getElementById('advanced-settings');
    if (advancedElement) {
      if (enable) {
        advancedElement.classList.remove('checkbox-group-disabled');
      } else {
        advancedElement.classList.add('checkbox-group-disabled');
      }
    }
  }

  getAllSettings() {
    return {
      general: this.getSelectedValues(this.generalSettings),
      notifications: this.getSelectedValues(this.notificationSettings),
      privacy: this.getSelectedValues(this.privacySettings),
      advanced: this.getSelectedValues(this.advancedSettings)
    };
  }

  private getSelectedValues(group: any): string[] {
    const selectedValues = group.getAttribute('data-selected-values');
    return selectedValues ? selectedValues.split(',') : [];
  }

  saveSettings() {
    const settings = this.getAllSettings();
    console.log('Saving settings:', settings);
    
    // Simulate API call
    return fetch('/api/settings', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(settings)
    });
  }
}
```

### Feature Toggle Dashboard

```typescript
class FeatureToggleDashboard {
  private featureGroups: Map<string, any> = new Map();
  private dependencies: Map<string, string[]> = new Map();

  constructor() {
    this.setupFeatureGroups();
    this.setupDependencies();
  }

  private setupFeatureGroups() {
    // Core features (always enabled)
    const coreFeatures = createCheckboxGroup({
      id: 'core-features',
      label: 'Core Features',
      options: [
        { label: 'User Authentication', value: 'auth', checked: true },
        { label: 'Data Storage', value: 'storage', checked: true },
        { label: 'Basic UI', value: 'basic_ui', checked: true }
      ]
    });
    this.featureGroups.set('core', coreFeatures);

    // Premium features
    const premiumFeatures = createCheckboxGroup({
      id: 'premium-features',
      label: 'Premium Features',
      options: [
        { label: 'Advanced Analytics', value: 'analytics', checked: false },
        { label: 'Custom Reports', value: 'reports', checked: false },
        { label: 'API Access', value: 'api_access', checked: false },
        { label: 'Priority Support', value: 'priority_support', checked: false }
      ]
    });
    this.featureGroups.set('premium', premiumFeatures);

    // Experimental features
    const experimentalFeatures = createCheckboxGroup({
      id: 'experimental-features',
      label: 'Experimental Features (Use with caution)',
      options: [
        { label: 'AI-Powered Insights', value: 'ai_insights', checked: false },
        { label: 'Real-time Collaboration', value: 'realtime_collab', checked: false },
        { label: 'Advanced Integrations', value: 'integrations', checked: false }
      ]
    });
    this.featureGroups.set('experimental', experimentalFeatures);
  }

  private setupDependencies() {
    // Define feature dependencies
    this.dependencies.set('reports', ['analytics']);
    this.dependencies.set('ai_insights', ['analytics', 'api_access']);
    this.dependencies.set('realtime_collab', ['api_access']);
  }

  toggleFeature(featureValue: string, enabled: boolean) {
    if (enabled) {
      // Check dependencies
      const deps = this.dependencies.get(featureValue) || [];
      const missingDeps = deps.filter(dep => !this.isFeatureEnabled(dep));
      
      if (missingDeps.length > 0) {
        console.warn(`Cannot enable ${featureValue}. Missing dependencies: ${missingDeps.join(', ')}`);
        return false;
      }
    } else {
      // Check what depends on this feature
      const dependents = this.getDependentFeatures(featureValue);
      if (dependents.length > 0) {
        console.warn(`Disabling ${featureValue} will also disable: ${dependents.join(', ')}`);
        
        // Disable dependent features
        dependents.forEach(dependent => {
          this.updateFeatureState(dependent, false);
        });
      }
    }

    this.updateFeatureState(featureValue, enabled);
    return true;
  }

  private isFeatureEnabled(featureValue: string): boolean {
    for (const [, group] of this.featureGroups) {
      const selectedValues = this.getSelectedValues(group);
      if (selectedValues.includes(featureValue)) {
        return true;
      }
    }
    return false;
  }

  private getDependentFeatures(featureValue: string): string[] {
    const dependents: string[] = [];
    for (const [feature, deps] of this.dependencies) {
      if (deps.includes(featureValue) && this.isFeatureEnabled(feature)) {
        dependents.push(feature);
      }
    }
    return dependents;
  }

  private updateFeatureState(featureValue: string, enabled: boolean) {
    // Find and update the specific feature checkbox
    for (const [groupName, group] of this.featureGroups) {
      const options = group.options || [];
      const option = options.find((opt: any) => opt.value === featureValue);
      if (option) {
        option.checked = enabled;
        // Update the DOM element
        this.updateGroupState(groupName, group);
        break;
      }
    }
  }

  private updateGroupState(groupName: string, group: any) {
    const selectedValues = group.options
      .filter((opt: any) => opt.checked)
      .map((opt: any) => opt.value);
    
    group.selectedValues = selectedValues;
    
    // Update DOM
    const element = document.getElementById(group.id);
    if (element) {
      element.setAttribute('data-selected-values', selectedValues.join(','));
    }
  }

  private getSelectedValues(group: any): string[] {
    return group.selectedValues || [];
  }

  getFeatureConfiguration() {
    const config: Record<string, string[]> = {};
    for (const [groupName, group] of this.featureGroups) {
      config[groupName] = this.getSelectedValues(group);
    }
    return config;
  }

  exportConfiguration(): string {
    return JSON.stringify(this.getFeatureConfiguration(), null, 2);
  }

  importConfiguration(configJson: string): boolean {
    try {
      const config = JSON.parse(configJson);
      
      for (const [groupName, features] of Object.entries(config)) {
        const group = this.featureGroups.get(groupName);
        if (group && Array.isArray(features)) {
          (features as string[]).forEach(feature => {
            this.toggleFeature(feature, true);
          });
        }
      }
      
      return true;
    } catch (error) {
      console.error('Failed to import configuration:', error);
      return false;
    }
  }
}

// Usage
const featureDashboard = new FeatureToggleDashboard();

// Handle feature toggle events
document.addEventListener('change', (event) => {
  if (event.target?.classList.contains('checkbox')) {
    const checkbox = event.target as HTMLElement;
    const featureValue = checkbox.getAttribute('data-value');
    const isChecked = checkbox.getAttribute('data-checked') === 'true';
    
    if (featureValue) {
      featureDashboard.toggleFeature(featureValue, isChecked);
    }
  }
});
```

### Settings Panel with Checkboxes

```typescript
import { panel, checkboxWidget, checkboxGroupWidget } from 'reactive-tui-ts'

const settingsPanel = panel({
  id: 'app-settings',
  title: 'Application Settings',
  content: [
    checkboxWidget({
      id: 'auto-update',
      label: 'Automatic Updates',
      checked: true,
      animationConfig: {
        enabled: true,
        duration: 250,
        easing: 'ease-out',
        scaleFactor: 1.3
      }
    }),
    
    checkboxWidget({
      id: 'error-reporting',
      label: 'Send Error Reports',
      checked: false,
      labelPosition: CheckboxLabelPosition.After
    }),
    
    checkboxGroupWidget({
      id: 'notification-types',
      label: 'Notification Types',
      options: [
        { id: 'system', label: 'System Notifications', value: 'system', checked: true },
        { id: 'updates', label: 'Update Notifications', value: 'updates', checked: true },
        { id: 'security', label: 'Security Alerts', value: 'security', checked: true },
        { id: 'marketing', label: 'Marketing Messages', value: 'marketing', checked: false }
      ],
      onChange: (selectedTypes) => {
        updateNotificationSettings(selectedTypes)
      }
    })
  ]
})
```

### Conditional Checkbox Logic

```typescript
const masterCheckbox = checkboxWidget({
  id: 'enable-features',
  label: 'Enable Advanced Features',
  checked: false,
  onChange: (checked) => {
    // Enable/disable dependent checkboxes
    toggleDependentFeatures(checked)
  }
})

const dependentFeatures = checkboxGroupWidget({
  id: 'advanced-features',
  label: 'Advanced Features',
  enabled: false, // Initially disabled
  options: [
    { id: 'ai-assist', label: 'AI Assistant', value: 'ai-assist' },
    { id: 'analytics', label: 'Advanced Analytics', value: 'analytics' },
    { id: 'integrations', label: 'Third-party Integrations', value: 'integrations' }
  ],
  onChange: (selectedFeatures) => {
    activateAdvancedFeatures(selectedFeatures)
  }
})

function toggleDependentFeatures(enabled: boolean) {
  dependentFeatures.enabled = enabled
  if (!enabled) {
    // Clear selections when disabled
    dependentFeatures.selectedValues = []
  }
}
```

## CSS Styling

The Checkbox widget includes comprehensive CSS styles with animations:

```css
/* Checkbox animation keyframes */
@keyframes checkboxCheckIn {
  0% {
    transform: scale(1.8);
    opacity: 0.8;
  }
  50% {
    transform: scale(1.1);
    opacity: 0.9;
  }
  100% {
    transform: scale(1.0);
    opacity: 1.0;
  }
}

/* Animation variants for different scale factors */
@keyframes checkboxCheckInSubtle {
  0% {
    transform: scale(1.3);
    opacity: 0.9;
  }
  100% {
    transform: scale(1.0);
    opacity: 1.0;
  }
}

@keyframes checkboxCheckInBouncy {
  0% {
    transform: scale(2.2);
    opacity: 0.7;
  }
  50% {
    transform: scale(1.2);
    opacity: 0.9;
  }
  100% {
    transform: scale(1.0);
    opacity: 1.0;
  }
}

/* Checkbox base styles */
.checkbox {
  display: inline-block;
  cursor: pointer;
  user-select: none;
  transition: all 0.2s ease;
}

.checkbox:hover {
  opacity: 0.8;
}

.checkbox-disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.checkbox-hidden {
  display: none;
}

.checkbox-animating {
  animation-fill-mode: both;
}

/* Dynamic animations based on data attributes */
.checkbox-animating[data-animation-scale="1.2"] {
  animation: checkboxCheckInSubtle 200ms ease-out;
}

.checkbox-animating[data-animation-scale="1.5"] {
  animation: checkboxCheckIn 250ms ease-out;
}

.checkbox-animating[data-animation-scale="1.8"] {
  animation: checkboxCheckIn 300ms ease-out;
}

.checkbox-animating[data-animation-scale="2.2"] {
  animation: checkboxCheckInBouncy 500ms cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

/* Checkbox group styles */
.checkbox-group {
  display: block;
}

.checkbox-group-horizontal {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
}

.checkbox-group-vertical .checkbox {
  display: block;
  margin-bottom: 0.25rem;
}

.checkbox-group-disabled {
  opacity: 0.5;
  pointer-events: none;
}

/* Style-specific classes */
.checkbox-ballot {
  font-family: 'Segoe UI Symbol', 'Apple Color Emoji', sans-serif;
}

.checkbox-square {
  font-family: monospace;
}

.checkbox-round {
  font-family: monospace;
}

.checkbox-custom {
  /* Custom styling can be applied here */
}

/* State classes */
.checkbox-checked {
  /* Styles for checked state */
}

.checkbox-unchecked {
  /* Styles for unchecked state */
}

/* Focus styles for accessibility */
.checkbox:focus-visible {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

/* Label positioning styles */
.checkbox-label-before {
  flex-direction: row-reverse;
}

.checkbox-label-after {
  flex-direction: row;
}

.checkbox-label-above {
  flex-direction: column-reverse;
}

.checkbox-label-below {
  flex-direction: column;
}

/* Responsive adjustments */
@media (max-width: 768px) {
  .checkbox-group-horizontal {
    flex-direction: column;
    gap: 0.25rem;
  }
}
```

## Animation System

The checkbox widget includes a comprehensive animation system with CSS keyframes:

```typescript
// Animation CSS can be imported
import { checkboxAnimationCSS } from 'reactive-tui';

// Add to your stylesheet
const styleElement = document.createElement('style');
styleElement.textContent = checkboxAnimationCSS;
document.head.appendChild(styleElement);
```

## Best Practices

1. **Accessibility**
   - Always provide meaningful labels for checkboxes
   - Use proper ARIA attributes for screen readers
   - Implement keyboard navigation (Space to toggle)
   - Ensure sufficient color contrast

2. **User Experience**
   - Use consistent animation settings across your application
   - Group related checkboxes using checkbox groups
   - Provide clear visual feedback for states
   - Use appropriate label positioning

3. **Performance**
   - Limit animation scale factors for smooth performance
   - Use CSS animations rather than JavaScript animations
   - Batch checkbox state updates when possible

4. **Form Design**
   - Use checkbox groups for multi-selection options
   - Implement proper validation for required checkboxes
   - Consider dependency relationships between options

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const checkboxContainer = new ElementBuilderImpl('div')
  .class('checkbox-container')
  .child(
    createSimpleCheckbox({
      id: 'feature-checkbox',
      label: 'Enable feature'
    })
  )
  .child(
    createCheckboxGroup({
      id: 'options-group',
      label: 'Select options',
      options: [
        { label: 'Option 1', value: 'opt1' },
        { label: 'Option 2', value: 'opt2' }
      ]
    })
  )
  .build();
```

## Accessibility

The Checkbox widget includes comprehensive accessibility features:

- ARIA attributes for screen readers (`role="checkbox"`, `aria-checked`)
- Keyboard navigation support (Space to toggle)
- Focus management with visual indicators
- High contrast mode support
- Proper label association

```typescript
const accessibleCheckbox = checkboxWidget({
  id: 'accessible-checkbox',
  label: 'Accessible checkbox',
  // Automatically includes:
  // - role="checkbox"
  // - aria-checked attribute
  // - aria-label for screen readers
  // - data attributes for state tracking
  // - keyboard event handling
});
```

The Checkbox widget provides comprehensive checkbox functionality with smooth scaling animations, flexible styling options, extensive convenience functions, and full accessibility support for building rich form interfaces.