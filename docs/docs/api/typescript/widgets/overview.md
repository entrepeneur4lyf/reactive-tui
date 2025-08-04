---
sidebar_position: 1
---

# Widget System Overview

Comprehensive guide to the **29 widgets** available in Reactive TUI TypeScript SDK, organized by category and use case. Each widget includes multiple convenience functions, real-world examples, and production-ready code patterns.

## Widget Categories

### 🏗️ Layout & Navigation Widgets

Widgets for organizing and structuring your application layout.

#### **Grid**
Flexible grid system for complex layouts.
```typescript
import { grid } from 'reactive-tui-ts'

const layout = grid({
  id: 'main-grid',
  columns: 3,
  rows: 2,
  gap: '1rem'
})
```

#### **Bar**
Header bars, toolbars, status bars, and navigation bars.
```typescript
import { bar, headerBar, statusBar, toolbar } from 'reactive-tui-ts'

const header = headerBar({
  id: 'app-header',
  title: 'My Application',
  actions: ['menu', 'search', 'profile']
})
```

#### **Tabs**
Tabbed interface for organizing content.
```typescript
import { tabs } from 'reactive-tui-ts'

const tabContainer = tabs({
  id: 'main-tabs',
  tabs: [
    { id: 'home', label: 'Home', content: homeContent },
    { id: 'settings', label: 'Settings', content: settingsContent }
  ]
})
```

#### **Modal**
Dialog boxes and overlay windows.
```typescript
import { modal } from 'reactive-tui-ts'

const confirmDialog = modal({
  id: 'confirm-modal',
  title: 'Confirm Action',
  content: 'Are you sure you want to proceed?',
  buttons: ['Cancel', 'Confirm']
})
```

#### **Accordion**
Collapsible content sections.
```typescript
import { createAccordion } from 'reactive-tui-ts'

const faq = createAccordion({
  id: 'faq-accordion',
  sections: [
    { id: 'what-is', title: 'What is Reactive TUI?', content: 'A terminal UI framework...' },
    { id: 'getting-started', title: 'How do I get started?', content: 'Install the package...' }
  ]
})
```

#### **Panel**
Flexible panels for content organization.
```typescript
import { panel, dashboardPanel, cardPanel } from 'reactive-tui-ts'

const infoPanel = cardPanel({
  id: 'info-panel',
  title: 'System Information',
  content: systemInfo
})
```

### 📝 Form Controls & Input Widgets

Widgets for user input and form handling.

#### **Input**
Text input fields with validation and formatting.
```typescript
import { input, textInput, passwordInput, emailInput } from 'reactive-tui-ts'

const nameField = textInput({
  id: 'name',
  placeholder: 'Enter your name',
  validation: { required: true, minLength: 2 }
})
```

#### **Button**
Interactive buttons with multiple variants and states.
```typescript
import { button, successButton, ghostButton } from 'reactive-tui-ts'

const submitBtn = successButton({
  id: 'submit',
  text: 'Submit Form',
  onClick: () => handleSubmit()
})
```

#### **Checkbox**
Single checkboxes and checkbox groups.
```typescript
import { checkboxWidget, checkboxGroupWidget } from 'reactive-tui-ts'

const features = checkboxGroupWidget({
  id: 'features',
  label: 'Features',
  options: [
    { label: 'Dark Mode', value: 'dark-mode', checked: true },
    { label: 'Notifications', value: 'notifications', checked: false }
  ]
})
```

#### **Switch**
Toggle switches for boolean values.
```typescript
import { switchToggle, createSwitch } from 'reactive-tui-ts'

const darkModeSwitch = createSwitch('dark-mode', true)
```

#### **Radio**
Radio button groups for single selection.
```typescript
import { radioGroup, createRadioGroup } from 'reactive-tui-ts'

const themeSelector = radioGroup({
  id: 'theme',
  options: [
    { value: 'light', label: 'Light Theme' },
    { value: 'dark', label: 'Dark Theme' },
    { value: 'auto', label: 'Auto' }
  ],
  selected: 'dark'
})
```

#### **Select**
Dropdown selection widgets.
```typescript
import { select } from 'reactive-tui-ts'

const countrySelect = select({
  id: 'country',
  options: [
    { value: 'us', label: 'United States' },
    { value: 'ca', label: 'Canada' },
    { value: 'uk', label: 'United Kingdom' }
  ],
  placeholder: 'Select a country'
})
```

#### **Autocomplete**
Auto-completing input fields.
```typescript
import { Autocomplete, countryAutocomplete } from 'reactive-tui-ts'

const cityAutocomplete = new Autocomplete({
  id: 'city',
  placeholder: 'Enter city name',
  suggestions: cityDatabase,
  filterMode: 'fuzzy'
})
```

#### **Slider**
Range sliders for numeric input.
```typescript
import { slider, SliderMode, SliderOrientation } from 'reactive-tui-ts'

const volumeSlider = slider({
  id: 'volume',
  mode: SliderMode.Single,
  orientation: SliderOrientation.Horizontal,
  min: 0,
  max: 100,
  value: 50,
  step: 5,
  label: 'Volume'
})
```

### 📊 Data Display & Visualization Widgets

Widgets for displaying and visualizing data.

#### **DataTable**
Comprehensive data tables with sorting, filtering, and pagination.
```typescript
import { dataTable, createColumn } from 'reactive-tui-ts'

const userTable = dataTable({
  id: 'users',
  columns: [
    createColumn('name', 'Name', { sortable: true }),
    createColumn('email', 'Email', { width: 200 }),
    createColumn('role', 'Role', { filterable: true })
  ],
  data: userData,
  pagination: { pageSize: 10 }
})
```

#### **Tree**
Hierarchical tree views for nested data.
```typescript
import { tree, createTreeNode } from 'reactive-tui-ts'

const fileTree = tree({
  id: 'file-explorer',
  data: fileSystemData,
  expandable: true,
  selectable: true,
  onSelect: (node) => openFile(node)
})
```

#### **ScrollableList**
Scrollable lists for large datasets.
```typescript
import { ScrollableList } from 'reactive-tui-ts'

const logList = new ScrollableList({
  id: 'logs',
  items: logEntries,
  itemHeight: 20,
  virtualScrolling: true
})
```

#### **Progress**
Progress bars and indicators.
```typescript
import { linearProgress, circularProgress } from 'reactive-tui-ts'

const uploadProgress = linearProgress({
  id: 'upload',
  value: 65,
  max: 100,
  label: 'Uploading...',
  showPercentage: true
})
```

#### **Spinner**
Loading spinners and animations.
```typescript
import { spinnerWidget, createLoadingSpinner } from 'reactive-tui-ts'

const loadingSpinner = createLoadingSpinner({
  id: 'loading',
  type: 'dots',
  message: 'Loading data...'
})
```

#### **RichText**
Rich text display with markdown support.
```typescript
import { RichText, documentationViewer } from 'reactive-tui-ts'

const readme = documentationViewer({
  id: 'readme',
  content: markdownContent,
  syntaxHighlighting: true
})
```

#### **Viewport**
Scrollable viewport for large content.
```typescript
import { Viewport } from 'reactive-tui-ts'

const contentViewport = new Viewport({
  id: 'content',
  width: 800,
  height: 600,
  scrollable: true
})
```

### 🔔 Feedback & Interaction Widgets

Widgets for user feedback and interaction.

#### **Toast**
Temporary notification messages.
```typescript
import { toast } from 'reactive-tui-ts'

const successToast = toast({
  id: 'success',
  message: 'Operation completed successfully!',
  variant: ToastVariant.Success,
  duration: 3000,
  position: ToastPosition.TopRight
})
```

#### **Menu**
Context menus and navigation menus.
```typescript
import { menuBar, menuItem, submenuItem } from 'reactive-tui-ts'

const appMenu = menuBar({
  id: 'app-menu',
  items: [
    submenuItem({ id: 'file', label: 'File', items: fileMenuItems }),
    submenuItem({ id: 'edit', label: 'Edit', items: editMenuItems }),
    submenuItem({ id: 'view', label: 'View', items: viewMenuItems })
  ]
})
```

#### **FormValidation**
Form validation and error display.
```typescript
import { formValidation } from 'reactive-tui-ts'

const validator = formValidation({
  id: 'form-validator',
  rules: {
    email: { required: true, email: true },
    password: { required: true, minLength: 8 }
  },
  showErrors: true
})
```

#### **Animation**
Animation states and transitions.
```typescript
import { WidgetAnimationState } from 'reactive-tui-ts'

const fadeIn = new WidgetAnimationState({
  type: 'fade',
  duration: 300,
  easing: 'ease-in-out'
})
```

#### **HotReload**
Development hot reload functionality.
```typescript
import { HotReloadManager, createHotReload } from 'reactive-tui-ts'

const hotReload = createHotReload({
  id: 'dev-reload',
  watchPaths: ['./src/**/*.ts'],
  enabled: process.env.NODE_ENV === 'development'
})
```

## Widget Factory System

### Creating Widgets

All widgets can be created using the factory pattern:

```typescript
import { WidgetFactory, createWidget } from 'reactive-tui-ts'

// Using the factory
const button = WidgetFactory.create('button', {
  id: 'my-button',
  text: 'Click me',
  variant: 'filled'
})

// Using the helper function
const input = createWidget('input', {
  id: 'my-input',
  type: 'text',
  placeholder: 'Enter text'
})
```

### Widget Configuration

All widgets share a common base configuration:

```typescript
interface BaseWidgetConfig {
  id: string                    // Required unique identifier
  type: string                  // Widget type
  classes?: string[]            // CSS classes
  attributes?: Record<string, string>  // Custom attributes
  disabled?: boolean            // Disabled state
  visible?: boolean             // Visibility
  focusable?: boolean           // Can receive focus
  tabIndex?: number             // Tab order
}
```

### Widget Instance

All widgets implement the `WidgetInstance` interface:

```typescript
interface WidgetInstance {
  readonly id: string
  readonly type: string
  readonly config: BaseWidgetConfig
  render(): Element
  update(config: Partial<BaseWidgetConfig>): void
  destroy(): void
  validate(): boolean
}
```

## Responsive Widgets

Widgets that adapt to terminal size implement `ResponsiveWidget`:

```typescript
interface ResponsiveWidget {
  toElement(): Element
  renderWithLayout(layout: LayoutRect, theme?: any): string
  minSize(): [number, number]
  maxSize(): [number | null, number | null]
  canGrowHorizontal(): boolean
  canGrowVertical(): boolean
}
```

## Widget Builders

Many widgets provide builder patterns for complex configuration:

```typescript
import { TreeBuilder, ProgressBuilder } from 'reactive-tui-ts'

// Tree builder
const tree = new TreeBuilder()
  .id('file-tree')
  .expandable(true)
  .selectable(true)
  .addNode('root', 'Root Folder')
  .addChild('root', 'file1', 'File 1.txt')
  .addChild('root', 'file2', 'File 2.txt')
  .build()

// Progress builder
const progress = new ProgressBuilder()
  .id('download')
  .type('linear')
  .value(50)
  .max(100)
  .animated(true)
  .showLabel(true)
  .build()
```

## Widget Styling

### CSS Classes

Widgets automatically receive CSS classes based on their type and state:

```css
/* Base widget classes */
.widget { /* Base widget styles */ }
.widget-button { /* Button-specific styles */ }
.widget-input { /* Input-specific styles */ }

/* State classes */
.widget-disabled { opacity: 0.5; }
.widget-focused { outline: 2px solid blue; }
.widget-hover { /* Hover effects */ }

/* Variant classes */
.button-filled { /* Filled button styles */ }
.button-outlined { /* Outlined button styles */ }
.input-error { border-color: red; }
```

### Custom Styling

```typescript
const styledButton = button({
  id: 'styled-btn',
  text: 'Custom Button',
  classes: ['custom-button', 'large'],
  attributes: {
    'data-theme': 'dark',
    'aria-label': 'Custom styled button'
  }
})
```

## Best Practices

### 1. Use Semantic Widget Types

```typescript
// ✅ Good - semantic widget choice
const submitButton = button({
  id: 'submit',
  text: 'Submit Form',
  variant: 'filled',
  color: 'primary'
})

const cancelButton = button({
  id: 'cancel',
  text: 'Cancel',
  variant: 'outlined',
  color: 'secondary'
})
```

### 2. Provide Unique IDs

```typescript
// ✅ Good - unique, descriptive IDs
const userNameInput = input({
  id: 'user-name-input',
  type: 'text'
})

// ❌ Avoid - generic or duplicate IDs
const input1 = input({
  id: 'input1',
  type: 'text'
})
```

### 3. Handle Events Properly

```typescript
// ✅ Good - proper event handling
const searchInput = input({
  id: 'search',
  type: 'text',
  onInput: (value) => {
    debounce(() => performSearch(value), 300)
  },
  onFocus: () => showSearchSuggestions(),
  onBlur: () => hideSearchSuggestions()
})
```

### 4. Use Appropriate Widget Sizes

```typescript
// ✅ Good - appropriate sizing
const compactButton = button({
  id: 'compact',
  text: 'OK',
  size: 'sm'
})

const prominentButton = button({
  id: 'prominent',
  text: 'Get Started',
  size: 'lg',
  variant: 'filled',
  color: 'primary'
})
```

## Complete Widget Documentation

### 🏗️ Layout & Navigation
- **[Accordion Widget](./accordion)** - Collapsible content sections with animation
- **[Bar Widget](./bar)** - Headers, toolbars, status bars with flexible positioning
- **[Grid Widget](./grid)** - CSS Grid system with responsive layouts
- **[Modal Widget](./modal)** - Dialog boxes and overlay windows
- **[Panel Widget](./panel)** - Flexible panels with Unicode border themes
- **[Tabs Widget](./tabs)** - Tab navigation with dynamic management

### 📝 Form Controls & Input
- **[Autocomplete Widget](./autocomplete)** - Auto-completing input with fuzzy search
- **[Button Widget](./button)** - Interactive buttons with variants and states
- **[Checkbox Widget](./checkbox)** - Single checkboxes and groups with animations
- **[Input Widget](./input)** - Text inputs with validation and 8 convenience functions
- **[Radio Widget](./radio)** - Radio button groups with multiple orientations
- **[Select Widget](./select)** - Dropdown selections with search and multi-select
- **[Slider Widget](./slider)** - Range sliders with single/range modes
- **[Switch Widget](./switch)** - Toggle switches with 6 convenience functions

### 📊 Data Display & Visualization
- **[DataTable Widget](./datatable)** - Advanced data tables with 4 convenience functions
- **[Progress Widget](./progress)** - Progress bars with 4 convenience functions and 15+ spinner types
- **[RichText Widget](./rich_text)** - Markdown rendering with syntax highlighting
- **[ScrollableList Widget](./scrollable_list)** - Virtual scrolling for large datasets
- **[Spinner Widget](./spinner)** - Loading animations with 30+ types
- **[Tree Widget](./tree)** - Hierarchical data with lazy loading
- **[Viewport Widget](./viewport)** - Scrollable viewport with virtual rendering

### 🔔 Feedback & Interaction
- **[Animation Widget](./animation)** - Property transitions with advanced easing
- **[FormValidation Widget](./form_validation)** - Form validation with built-in validators
- **[HotReload Widget](./hot_reload)** - Development hot reload functionality
- **[Menu Widget](./menu)** - Context and navigation menus
- **[Toast Widget](./toast)** - Notification toasts with positioning

### 🔧 Development & Advanced
- **[BaseWidget](./base-widget)** - Base widget class for custom development

### 📚 Getting Started
- **[Widget Factory](../core/widget-factory)** - Creating widgets with factory patterns
- **[Element Builder](../core/element-builder)** - Building complex UI structures
