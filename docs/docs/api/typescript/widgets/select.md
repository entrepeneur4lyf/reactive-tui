# Select Widget

A comprehensive dropdown selection widget supporting single/multi-select modes, search filtering, keyboard navigation, and customizable rendering.

## Overview

The Select widget provides advanced dropdown functionality with real-time search filtering, keyboard navigation, and support for both simple string options and complex option objects with icons and descriptions.

```typescript
import { select, SelectOption } from 'reactive-tui-ts'

const languageSelect = select({
  id: 'language-select',
  options: ['Rust', 'TypeScript', 'Python', 'Go'],
  selected: 0,
  placeholder: 'Choose a language...',
  onChange: (selectedIndices) => {
    console.log('Selected:', selectedIndices)
  }
})
```

## Types

### SelectOption

```typescript
interface SelectOption {
  id: string
  label: string
  icon?: string
  description?: string
  group?: string
  disabled?: boolean
  data?: Record<string, string>
}
```

### SelectMode

```typescript
type SelectMode = 'single' | 'multiple'
```

### DropdownPosition

```typescript
type DropdownPosition = 'below' | 'above' | 'auto'
```

## Configuration

### SelectConfig

```typescript
interface SelectConfig {
  id: string
  options?: string[]
  customOptions?: SelectOption[]
  mode?: SelectMode
  multiSelect?: boolean
  searchable?: boolean
  placeholder?: string
  position?: DropdownPosition
  style?: Partial<SelectStyle>
  maxHeight?: number
  selected?: number
  selectedIndices?: number[]
  onChange?: (selectedIndices: number[]) => void
  onToggle?: (open: boolean) => void
  disabled?: boolean
  required?: boolean
  filter?: (option: SelectOption, query: string) => boolean
}
```

### SelectStyle

```typescript
interface SelectStyle {
  containerClasses: string[]
  triggerClasses: string[]
  dropdownClasses: string[]
  optionClasses: string[]
  selectedOptionClasses: string[]
  highlightedOptionClasses: string[]
  disabledOptionClasses: string[]
  searchInputClasses: string[]
  groupClasses: string[]
  dropdownArrow: string
  selectedMarker: string
  maxHeight: number
  showIcons: boolean
  showDescriptions: boolean
}
```

### SelectState

```typescript
interface SelectState {
  open: boolean
  highlightedIndex?: number
  selectedIndices: number[]
  searchQuery: string
  filteredIndices: number[]
  focused: boolean
  scrollOffset: number
}
```

## Basic Usage

### Simple Select

```typescript
import { select } from 'reactive-tui-ts'

const simpleSelect = select({
  id: 'simple-select',
  options: ['Option 1', 'Option 2', 'Option 3', 'Option 4'],
  placeholder: 'Choose an option...',
  onChange: (selectedIndices) => {
    console.log('Selected index:', selectedIndices[0])
    const selectedOption = simpleSelect.getSelectedOptions()[0]
    console.log('Selected option:', selectedOption?.label)
  }
})

// Open the dropdown
simpleSelect.open()

// Select an option programmatically
simpleSelect.select(1)

// Get current selection
const selectedOptions = simpleSelect.getSelectedOptions()
console.log('Currently selected:', selectedOptions)
```

### Multi-Select

```typescript
const multiSelect = select({
  id: 'multi-select',
  options: ['Frontend', 'Backend', 'Database', 'Mobile', 'DevOps'],
  multiSelect: true,
  placeholder: 'Select technologies...',
  selectedIndices: [0, 2], // Pre-select Frontend and Database
  onChange: (selectedIndices) => {
    console.log(`Selected ${selectedIndices.length} items:`, selectedIndices)
    const selectedOptions = multiSelect.getSelectedOptions()
    selectedOptions.forEach(option => {
      console.log(`- ${option.label}`)
    })
  }
})

// Toggle selection of an option
multiSelect.toggleSelection(3) // Toggle Mobile

// Clear all selections
multiSelect.clearSelection()

// Check if an option is selected
if (multiSelect.isSelected(1)) {
  console.log('Backend is selected')
}
```

### Searchable Select

```typescript
const searchableSelect = select({
  id: 'searchable-select',
  options: [
    'Apple', 'Banana', 'Cherry', 'Date', 'Elderberry',
    'Fig', 'Grape', 'Honeydew', 'Kiwi', 'Lemon'
  ],
  searchable: true,
  placeholder: 'Search fruits...',
  maxHeight: 5, // Show max 5 options at once
  onChange: (selectedIndices) => {
    const fruit = searchableSelect.getSelectedOptions()[0]
    console.log('Selected fruit:', fruit?.label)
  }
})

// Set search query programmatically
searchableSelect.setSearchQuery('ap') // Will show Apple

// The search automatically filters options as user types
```

## Advanced Usage

### Custom Options with Icons and Descriptions

```typescript
import { select, SelectOption } from 'reactive-tui-ts'

const customOptions: SelectOption[] = [
  {
    id: 'rust',
    label: 'Rust',
    icon: '🦀',
    description: 'Systems programming language'
  },
  {
    id: 'typescript',
    label: 'TypeScript',
    icon: '📘',
    description: 'Typed JavaScript'
  },
  {
    id: 'python',
    label: 'Python',
    icon: '🐍',
    description: 'General-purpose programming'
  },
  {
    id: 'go',
    label: 'Go',
    icon: '🐹',
    description: 'Concurrent programming'
  },
  {
    id: 'java',
    label: 'Java',
    icon: '☕',
    description: 'Object-oriented programming',
    disabled: true // This option cannot be selected
  }
]

const advancedSelect = select({
  id: 'advanced-select',
  customOptions,
  searchable: true,
  placeholder: 'Choose a programming language...',
  style: {
    showIcons: true,
    showDescriptions: true,
    maxHeight: 6,
    dropdownArrow: '▼',
    selectedMarker: '✓'
  },
  onChange: (selectedIndices) => {
    const language = advancedSelect.getSelectedOptions()[0]
    if (language) {
      console.log(`Selected: ${language.label} ${language.icon}`)
      console.log(`Description: ${language.description}`)
    }
  }
})
```

### Custom Filtering

```typescript
const customFilterSelect = select({
  id: 'custom-filter',
  customOptions: [
    { id: '1', label: 'John Doe', description: 'Software Engineer' },
    { id: '2', label: 'Jane Smith', description: 'Product Manager' },
    { id: '3', label: 'Bob Johnson', description: 'Designer' },
    { id: '4', label: 'Alice Brown', description: 'Data Scientist' }
  ],
  searchable: true,
  placeholder: 'Search employees...',
  filter: (option, query) => {
    // Custom filter that searches both name and job title
    const queryLower = query.toLowerCase()
    return option.label.toLowerCase().includes(queryLower) ||
           (option.description?.toLowerCase().includes(queryLower) ?? false)
  },
  onChange: (selectedIndices) => {
    const employee = customFilterSelect.getSelectedOptions()[0]
    console.log('Selected employee:', employee)
  }
})

// Searching "engineer" will find John Doe
// Searching "john" will find John Doe
// Searching "manager" will find Jane Smith
```

## Select API

### Selection Methods

```typescript
const apiSelect = select({
  id: 'api-demo',
  options: ['Red', 'Green', 'Blue', 'Yellow', 'Purple'],
  multiSelect: true
})

// Selection operations
apiSelect.select(0)                    // Select Red
apiSelect.deselect(0)                  // Deselect Red
apiSelect.toggleSelection(1)           // Toggle Green
apiSelect.clearSelection()             // Clear all selections

// Get selection information
const selectedOptions = apiSelect.getSelectedOptions()
const selectedIds = apiSelect.getSelectedIds()
const isRedSelected = apiSelect.isSelected(0)

console.log('Selected options:', selectedOptions)
console.log('Selected IDs:', selectedIds)
console.log('Is Red selected?', isRedSelected)
```

### Dropdown Control

```typescript
// Dropdown operations
apiSelect.open()                       // Open dropdown
apiSelect.close()                      // Close dropdown
apiSelect.toggle()                     // Toggle open/closed state

// Navigation
apiSelect.navigateNext()               // Highlight next option
apiSelect.navigatePrevious()           // Highlight previous option
apiSelect.selectHighlighted()          // Select currently highlighted option

// Display
const displayText = apiSelect.getDisplayText()
console.log('Display text:', displayText)
// Single-select: shows selected option or placeholder
// Multi-select: shows "X items selected" or single item name
```

### State Management

```typescript
// Get current state
const state = apiSelect.getState()
console.log('Current state:', state)
// {
//   open: false,
//   highlightedIndex: undefined,
//   selectedIndices: [1, 3],
//   searchQuery: '',
//   filteredIndices: [0, 1, 2, 3, 4],
//   focused: false,
//   scrollOffset: 0
// }

// Update configuration
apiSelect.updateConfig({
  placeholder: 'New placeholder text',
  maxHeight: 8,
  searchable: true
})
```

## Keyboard Navigation

The Select widget provides comprehensive keyboard support:

```typescript
const keyboardSelect = select({
  id: 'keyboard-demo',
  options: ['Option A', 'Option B', 'Option C', 'Option D'],
  searchable: true
})

// Keyboard events are handled automatically:
// - Enter/Space: Open dropdown or select highlighted option
// - Escape: Close dropdown
// - ArrowDown: Open dropdown or navigate to next option
// - ArrowUp: Navigate to previous option
// - Tab: Close dropdown and continue tab navigation
// - Backspace: Remove last character from search (when searchable)
// - Any character: Add to search query (when searchable)

// Handle keyboard events manually if needed
const handleKeyboard = (event: KeyboardEvent) => {
  const handled = keyboardSelect.handleKeyEvent(event)
  if (handled) {
    event.preventDefault()
  }
}

// Attach to your event system
document.addEventListener('keydown', handleKeyboard)
```

## Event Handling

```typescript
const eventSelect = select({
  id: 'event-demo',
  options: ['First', 'Second', 'Third'],
  onChange: (selectedIndices) => {
    console.log('Selection changed:', selectedIndices)
    
    // Get detailed information about the change
    const selectedOptions = eventSelect.getSelectedOptions()
    selectedOptions.forEach((option, index) => {
      console.log(`Selected ${index + 1}: ${option.label}`)
    })
  },
  onToggle: (open) => {
    console.log('Dropdown is now:', open ? 'open' : 'closed')
    
    if (open) {
      console.log('User opened the dropdown')
      // Could load dynamic data here
    } else {
      console.log('User closed the dropdown')
      // Could save state or clean up
    }
  }
})

// Focus/blur handling
eventSelect.handleFocus()              // Called when select gains focus
eventSelect.handleBlur()               // Called when select loses focus
```

## Select Patterns

### Pre-built Pattern Functions

```typescript
import { selectPatterns } from 'reactive-tui-ts'

// Yes/No select
const confirmSelect = selectPatterns.yesNo('confirm-action')
confirmSelect.onChange = (selectedIndices) => {
  const answer = selectedIndices[0] === 0 ? 'Yes' : 'No'
  console.log('User answered:', answer)
}

// Programming language select with icons
const languageSelect = selectPatterns.languages('programming-language')

// Priority select with color-coded icons
const prioritySelect = selectPatterns.priority('task-priority')
prioritySelect.onChange = (selectedIndices) => {
  const priorities = ['high', 'medium', 'low']
  const selectedPriority = priorities[selectedIndices[0]]
  console.log('Selected priority:', selectedPriority)
}
```

### Custom Pattern Implementation

```typescript
// Create a custom pattern for status selection
const createStatusSelect = (id: string) => {
  const statusOptions: SelectOption[] = [
    { id: 'draft', label: 'Draft', icon: '📝', description: 'Work in progress' },
    { id: 'review', label: 'In Review', icon: '👀', description: 'Awaiting review' },
    { id: 'approved', label: 'Approved', icon: '✅', description: 'Ready to publish' },
    { id: 'published', label: 'Published', icon: '🚀', description: 'Live and active' },
    { id: 'archived', label: 'Archived', icon: '📦', description: 'No longer active' }
  ]

  return select({
    id,
    customOptions: statusOptions,
    placeholder: 'Select status...',
    style: {
      showIcons: true,
      showDescriptions: true
    }
  })
}

const statusSelect = createStatusSelect('document-status')
```

## Styling and Customization

### Custom Styling

```typescript
const styledSelect = select({
  id: 'styled-select',
  options: ['Option 1', 'Option 2', 'Option 3'],
  style: {
    containerClasses: ['custom-select', 'primary-select'],
    triggerClasses: ['select-trigger', 'styled-trigger'],
    dropdownClasses: ['select-dropdown', 'fancy-dropdown'],
    optionClasses: ['select-option', 'custom-option'],
    selectedOptionClasses: ['select-option-selected', 'highlighted'],
    highlightedOptionClasses: ['select-option-highlighted', 'keyboard-focus'],
    disabledOptionClasses: ['select-option-disabled', 'muted'],
    searchInputClasses: ['select-search', 'search-input'],
    dropdownArrow: '⬇️',
    selectedMarker: '✓',
    maxHeight: 6,
    showIcons: true,
    showDescriptions: true
  }
})
```

### Responsive and Adaptive Behavior

```typescript
const responsiveSelect = select({
  id: 'responsive-select',
  options: ['Small Screen', 'Medium Screen', 'Large Screen'],
  position: 'auto', // Automatically positions dropdown above/below
  maxHeight: 5,
  style: {
    maxHeight: 5 // Limits visible options for small screens
  }
})

// The dropdown automatically:
// - Positions above trigger if not enough space below
// - Scrolls options if more than maxHeight
// - Adjusts search behavior based on available space
```

## Complete Application Example

```typescript
import { select, SelectOption } from 'reactive-tui-ts'

class UserPreferencesForm {
  private themeSelect: any
  private languageSelect: any
  private notificationSelect: any
  private categorySelect: any

  constructor() {
    this.setupSelects()
  }

  private setupSelects() {
    // Theme selection
    this.themeSelect = select({
      id: 'theme-select',
      customOptions: [
        { id: 'light', label: 'Light Theme', icon: '☀️', description: 'Bright and clean' },
        { id: 'dark', label: 'Dark Theme', icon: '🌙', description: 'Easy on the eyes' },
        { id: 'auto', label: 'Auto Theme', icon: '🌓', description: 'Follows system' }
      ],
      selected: 0,
      onChange: (selectedIndices) => {
        const theme = this.themeSelect.getSelectedOptions()[0]
        this.applyTheme(theme.id)
      }
    })

    // Language selection with search
    this.languageSelect = select({
      id: 'language-select',
      customOptions: [
        { id: 'en', label: 'English', icon: '🇺🇸' },
        { id: 'es', label: 'Español', icon: '🇪🇸' },
        { id: 'fr', label: 'Français', icon: '🇫🇷' },
        { id: 'de', label: 'Deutsch', icon: '🇩🇪' },
        { id: 'ja', label: '日本語', icon: '🇯🇵' },
        { id: 'zh', label: '中文', icon: '🇨🇳' }
      ],
      searchable: true,
      placeholder: 'Select language...',
      onChange: (selectedIndices) => {
        const language = this.languageSelect.getSelectedOptions()[0]
        this.changeLanguage(language.id)
      }
    })

    // Notification preferences (multi-select)
    this.notificationSelect = select({
      id: 'notifications-select',
      customOptions: [
        { id: 'email', label: 'Email Notifications', icon: '📧' },
        { id: 'push', label: 'Push Notifications', icon: '🔔' },
        { id: 'sms', label: 'SMS Notifications', icon: '📱' },
        { id: 'desktop', label: 'Desktop Notifications', icon: '💻' }
      ],
      multiSelect: true,
      selectedIndices: [0, 1], // Pre-select email and push
      placeholder: 'Choose notification types...',
      onChange: (selectedIndices) => {
        const notifications = this.notificationSelect.getSelectedOptions()
        this.updateNotificationSettings(notifications)
      }
    })

    // Interest categories with custom filter
    this.categorySelect = select({
      id: 'categories-select',
      customOptions: [
        { id: 'tech', label: 'Technology', description: 'Programming, AI, Gadgets' },
        { id: 'science', label: 'Science', description: 'Research, Discoveries' },
        { id: 'business', label: 'Business', description: 'Startups, Finance, Markets' },
        { id: 'design', label: 'Design', description: 'UI/UX, Graphics, Art' },
        { id: 'health', label: 'Health', description: 'Fitness, Nutrition, Wellness' },
        { id: 'travel', label: 'Travel', description: 'Destinations, Culture' }
      ],
      multiSelect: true,
      searchable: true,
      placeholder: 'Select interests...',
      filter: (option, query) => {
        const queryLower = query.toLowerCase()
        return option.label.toLowerCase().includes(queryLower) ||
               (option.description?.toLowerCase().includes(queryLower) ?? false)
      },
      onChange: (selectedIndices) => {
        const categories = this.categorySelect.getSelectedOptions()
        this.updateInterests(categories)
      }
    })
  }

  private applyTheme(themeId: string) {
    console.log(`Applying theme: ${themeId}`)
    // Theme application logic
    document.body.className = `theme-${themeId}`
  }

  private changeLanguage(languageId: string) {
    console.log(`Changing language to: ${languageId}`)
    // Language change logic
    localStorage.setItem('preferred-language', languageId)
  }

  private updateNotificationSettings(notifications: SelectOption[]) {
    console.log('Updated notification settings:')
    notifications.forEach(notification => {
      console.log(`- ${notification.label} enabled`)
    })
    
    // Save to preferences
    const notificationIds = notifications.map(n => n.id)
    localStorage.setItem('notifications', JSON.stringify(notificationIds))
  }

  private updateInterests(categories: SelectOption[]) {
    console.log('Updated interests:')
    categories.forEach(category => {
      console.log(`- ${category.label}: ${category.description}`)
    })
    
    // Save to user profile
    const categoryIds = categories.map(c => c.id)
    this.saveUserInterests(categoryIds)
  }

  private saveUserInterests(categoryIds: string[]) {
    // API call to save user interests
    console.log('Saving interests to server:', categoryIds)
  }

  public render() {
    return `
      <div class="preferences-form">
        <h2>User Preferences</h2>
        
        <div class="form-group">
          <label>Theme</label>
          ${this.themeSelect.render().outerHTML}
        </div>
        
        <div class="form-group">
          <label>Language</label>
          ${this.languageSelect.render().outerHTML}
        </div>
        
        <div class="form-group">
          <label>Notifications</label>
          ${this.notificationSelect.render().outerHTML}
        </div>
        
        <div class="form-group">
          <label>Interests</label>
          ${this.categorySelect.render().outerHTML}
        </div>
      </div>
    `
  }

  public async loadUserPreferences(userId: string) {
    // Load user preferences from API
    const preferences = await this.fetchUserPreferences(userId)
    
    // Apply loaded preferences
    if (preferences.theme) {
      const themeIndex = this.themeSelect.customOptions.findIndex((opt: SelectOption) => opt.id === preferences.theme)
      if (themeIndex !== -1) {
        this.themeSelect.select(themeIndex)
      }
    }
    
    if (preferences.language) {
      const langIndex = this.languageSelect.customOptions.findIndex((opt: SelectOption) => opt.id === preferences.language)
      if (langIndex !== -1) {
        this.languageSelect.select(langIndex)
      }
    }
    
    if (preferences.notifications) {
      this.notificationSelect.clearSelection()
      preferences.notifications.forEach((notifId: string) => {
        const index = this.notificationSelect.customOptions.findIndex((opt: SelectOption) => opt.id === notifId)
        if (index !== -1) {
          this.notificationSelect.select(index)
        }
      })
    }
    
    if (preferences.interests) {
      this.categorySelect.clearSelection()
      preferences.interests.forEach((categoryId: string) => {
        const index = this.categorySelect.customOptions.findIndex((opt: SelectOption) => opt.id === categoryId)
        if (index !== -1) {
          this.categorySelect.select(index)
        }
      })
    }
  }

  private async fetchUserPreferences(userId: string) {
    // Mock API call
    return {
      theme: 'dark',
      language: 'en',
      notifications: ['email', 'push'],
      interests: ['tech', 'design']
    }
  }
}

// Usage
const preferencesForm = new UserPreferencesForm()
await preferencesForm.loadUserPreferences('user123')
console.log(preferencesForm.render())
```

## CSS Styling

```css
/* Select container */
.select {
  position: relative;
  display: inline-block;
  min-width: 200px;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

.select-disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.select-focused {
  outline: 2px solid #007bff;
  outline-offset: 2px;
}

/* Select trigger button */
.select-trigger {
  width: 100%;
  padding: 0.5rem 1rem;
  border: 1px solid #ced4da;
  background: #ffffff;
  color: #212529;
  text-align: left;
  cursor: pointer;
  font-family: inherit;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.select-trigger:hover {
  border-color: #007bff;
  background: #f8f9fa;
}

.select-trigger:disabled {
  cursor: not-allowed;
  background: #e9ecef;
}

.select-open .select-trigger {
  border-color: #007bff;
  box-shadow: 0 0 0 0.2rem rgba(0, 123, 255, 0.25);
}

/* Dropdown */
.select-dropdown {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  background: #ffffff;
  border: 1px solid #ced4da;
  border-top: none;
  max-height: 200px;
  overflow-y: auto;
  z-index: 1000;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

/* Search input */
.select-search {
  width: 100%;
  padding: 0.5rem;
  border: none;
  border-bottom: 1px solid #e9ecef;
  outline: none;
  font-family: inherit;
}

.select-search:focus {
  border-bottom-color: #007bff;
}

/* Options */
.select-option {
  padding: 0.5rem 1rem;
  cursor: pointer;
  border-bottom: 1px solid #f8f9fa;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.select-option:last-child {
  border-bottom: none;
}

.select-option:hover {
  background: #f8f9fa;
}

.select-option-highlighted {
  background: #e3f2fd;
}

.select-option-selected {
  background: #007bff;
  color: white;
}

.select-option-disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background: #f8f9fa;
}

.select-option-disabled:hover {
  background: #f8f9fa;
}

/* Multi-select styles */
.select[data-mode="multiple"] .select-option-selected {
  background: #e8f4f8;
  color: #212529;
  border-left: 3px solid #007bff;
}

.select[data-mode="multiple"] .select-option-selected:hover {
  background: #d1ecf1;
}

/* Option content */
.select-option-icon {
  font-size: 1rem;
  width: 1.2rem;
  text-align: center;
}

.select-option-label {
  flex: 1;
  font-weight: 500;
}

.select-option-description {
  font-size: 0.875rem;
  color: #6c757d;
  font-style: italic;
}

.select-option-marker {
  font-weight: bold;
  color: #28a745;
}

/* Groups */
.select-group {
  padding: 0.25rem 1rem;
  background: #e9ecef;
  font-weight: bold;
  font-size: 0.875rem;
  color: #495057;
  border-bottom: 1px solid #dee2e6;
}

/* Dropdown positioning */
.select-dropdown.dropdown-above {
  top: auto;
  bottom: 100%;
  border-top: 1px solid #ced4da;
  border-bottom: none;
}

/* Loading state */
.select-loading .select-trigger::after {
  content: '⏳';
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Accessibility */
.select:focus-within {
  outline: 2px solid #007bff;
  outline-offset: 2px;
}

@media (prefers-reduced-motion: reduce) {
  .select-loading .select-trigger::after {
    animation: none;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .select-trigger,
  .select-dropdown {
    background: #2d3748;
    color: #e2e8f0;
    border-color: #4a5568;
  }
  
  .select-trigger:hover {
    background: #4a5568;
    border-color: #63b3ed;
  }
  
  .select-option:hover {
    background: #4a5568;
  }
  
  .select-option-highlighted {
    background: #2b6cb0;
  }
  
  .select-option-selected {
    background: #3182ce;
  }
  
  .select-group {
    background: #1a202c;
    color: #a0aec0;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .select-trigger {
    border-width: 2px;
  }
  
  .select-option-selected {
    border: 2px solid #ffffff;
  }
}
```

## Best Practices

### 1. Use Appropriate Selection Modes

```typescript
// ✅ Good - single-select for exclusive choices
const themeSelect = select({
  id: 'theme',
  options: ['Light', 'Dark', 'Auto'],
  mode: 'single',
  placeholder: 'Choose theme...'
})

// ✅ Good - multi-select for multiple valid choices
const skillsSelect = select({
  id: 'skills',
  options: ['JavaScript', 'TypeScript', 'React', 'Node.js', 'Python'],
  multiSelect: true,
  placeholder: 'Select your skills...'
})
```

### 2. Provide Search for Large Option Lists

```typescript
// ✅ Good - searchable for many options
const countrySelect = select({
  id: 'country',
  options: getAllCountries(), // 195+ countries
  searchable: true,
  maxHeight: 8,
  placeholder: 'Search countries...'
})
```

### 3. Use Meaningful Icons and Descriptions

```typescript
// ✅ Good - clear visual hierarchy
const prioritySelect = select({
  id: 'priority',
  customOptions: [
    { id: 'urgent', label: 'Urgent', icon: '🔴', description: 'Needs immediate attention' },
    { id: 'high', label: 'High', icon: '🟠', description: 'Important but not urgent' },
    { id: 'normal', label: 'Normal', icon: '🟡', description: 'Standard priority' },
    { id: 'low', label: 'Low', icon: '🟢', description: 'Can be done later' }
  ],
  style: {
    showIcons: true,
    showDescriptions: true
  }
})
```

### 4. Handle Loading States Properly

```typescript
// ✅ Good - proper async handling
const dynamicSelect = select({
  id: 'dynamic-data',
  options: [],
  placeholder: 'Loading options...',
  disabled: true
})

// Load data asynchronously
const loadSelectData = async () => {
  try {
    const options = await fetchOptionsFromAPI()
    dynamicSelect.updateConfig({
      options: options.map(opt => opt.name),
      placeholder: 'Select an option...',
      disabled: false
    })
  } catch (error) {
    dynamicSelect.updateConfig({
      placeholder: 'Failed to load options',
      disabled: true
    })
  }
}

loadSelectData()
```

### 5. Provide Clear Feedback

```typescript
// ✅ Good - comprehensive feedback
const feedbackSelect = select({
  id: 'feedback-demo',
  options: ['Option 1', 'Option 2', 'Option 3'],
  onChange: (selectedIndices) => {
    // Show immediate feedback
    const count = selectedIndices.length
    if (count === 0) {
      showMessage('No options selected')
    } else if (count === 1) {
      const option = feedbackSelect.getSelectedOptions()[0]
      showMessage(`Selected: ${option.label}`)
    } else {
      showMessage(`Selected ${count} options`)
    }
  },
  onToggle: (open) => {
    if (open) {
      console.log('Dropdown opened - user browsing options')
    }
  }
})
```

## Related Widgets

- **[Input](./input)** - Text input fields and validation
- **[Radio](./radio)** - Single-choice selection with visible options
- **[Checkbox](./checkbox)** - Multiple selections with visible options
- **[Autocomplete](./autocomplete)** - Type-ahead input with suggestions
- **[Menu](./menu)** - Navigation and action menus

## Examples

- **[Basic Select](../../examples/basic/select-basic)** - Simple dropdown examples
- **[Multi-Select](../../examples/basic/select-multi)** - Multiple selection patterns
- **[Search Select](../../examples/advanced/select-search)** - Searchable dropdowns
- **[Form Integration](../../examples/apps/form-builder)** - Select widgets in forms
- **[Dynamic Data](../../examples/advanced/dynamic-select)** - Async data loading

The Select widget provides comprehensive dropdown functionality with search, multi-select, keyboard navigation, and extensive customization options for building sophisticated user interfaces.