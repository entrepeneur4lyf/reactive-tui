---
sidebar_position: 3
---

# Widget Factory

The Widget Factory system provides a unified interface for creating and managing widgets with consistent configuration patterns and type safety.

## Overview

The Widget Factory system consists of:

- **`WidgetFactory`** - Central factory for creating widgets
- **`BaseWidget`** - Base interface for all widgets
- **`WidgetInstance`** - Runtime widget instance interface
- **`ResponsiveWidget`** - Interface for responsive widgets

## WidgetFactory

### Creating Widgets

```typescript
import { WidgetFactory, createWidget } from 'reactive-tui-ts'

// Using the factory directly
const button = WidgetFactory.create('button', {
  id: 'my-button',
  text: 'Click me',
  variant: 'filled',
  color: 'primary'
})

// Using the helper function
const input = createWidget('input', {
  id: 'my-input',
  type: 'text',
  placeholder: 'Enter text'
})

// Using specialized factory methods
const table = WidgetFactory.dataTable({
  id: 'users-table',
  columns: tableColumns,
  data: userData
})
```

### Factory Methods

#### `create<T>(type: string, config: T): WidgetInstance`

Generic widget creation method:

```typescript
// Button widget
const button = WidgetFactory.create('button', {
  id: 'submit-btn',
  text: 'Submit',
  variant: 'filled',
  onClick: () => handleSubmit()
})

// Input widget
const input = WidgetFactory.create('input', {
  id: 'email-input',
  type: 'email',
  placeholder: 'Enter email',
  validation: { required: true, email: true }
})

// Tree widget
const tree = WidgetFactory.create('tree', {
  id: 'file-tree',
  data: fileSystemData,
  expandable: true,
  selectable: true
})
```

#### Specialized Factory Methods

```typescript
// Layout widgets
const grid = WidgetFactory.grid({
  id: 'main-grid',
  columns: 3,
  rows: 2,
  gap: '1rem'
})

const tabs = WidgetFactory.tabs({
  id: 'content-tabs',
  tabs: [
    { id: 'home', label: 'Home', content: homeContent },
    { id: 'settings', label: 'Settings', content: settingsContent }
  ]
})

// Form widgets
const checkbox = WidgetFactory.checkbox({
  id: 'agree-terms',
  label: 'I agree to the terms',
  checked: false
})

const select = WidgetFactory.select({
  id: 'country-select',
  options: countryOptions,
  placeholder: 'Select country'
})

// Data widgets
const dataTable = WidgetFactory.dataTable({
  id: 'users-table',
  columns: userColumns,
  data: users,
  sortable: true,
  filterable: true
})

const progressBar = WidgetFactory.progress({
  id: 'upload-progress',
  value: 65,
  max: 100,
  type: 'linear'
})
```

## BaseWidget Interface

All widgets implement the `BaseWidget` interface:

```typescript
interface BaseWidget {
  readonly id: string
  readonly type: string
  readonly config: BaseWidgetConfig
  render(): Element
  update(config: Partial<BaseWidgetConfig>): void
  destroy(): void
  validate(): boolean
}
```

### Base Widget Configuration

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
  style?: Record<string, string>  // Inline styles
}
```

### Widget Implementation Example

```typescript
class ButtonWidget implements BaseWidget {
  readonly id: string
  readonly type = 'button'
  private _config: ButtonConfig

  constructor(config: ButtonConfig) {
    this.id = config.id
    this._config = { ...config }
  }

  get config(): ButtonConfig {
    return { ...this._config }
  }

  render(): Element {
    return button()
      .id(this.id)
      .classes(this.getClasses())
      .attr('type', 'button')
      .attr('disabled', this._config.disabled ? 'true' : 'false')
      .content(this._config.text || '')
      .build()
  }

  update(config: Partial<ButtonConfig>): void {
    this._config = { ...this._config, ...config }
  }

  destroy(): void {
    // Cleanup logic
    this.removeEventListeners()
    this.clearReferences()
  }

  validate(): boolean {
    return !!(this.id && this._config.text)
  }

  private getClasses(): string[] {
    const classes = ['widget', 'widget-button']
    
    if (this._config.variant) {
      classes.push(`button-${this._config.variant}`)
    }
    
    if (this._config.color) {
      classes.push(`button-${this._config.color}`)
    }
    
    if (this._config.size) {
      classes.push(`button-${this._config.size}`)
    }
    
    if (this._config.classes) {
      classes.push(...this._config.classes)
    }
    
    return classes
  }

  private removeEventListeners(): void {
    // Remove event listeners
  }

  private clearReferences(): void {
    // Clear object references
  }
}
```

## ResponsiveWidget Interface

Widgets that adapt to terminal size implement `ResponsiveWidget`:

```typescript
interface ResponsiveWidget extends BaseWidget {
  toElement(): Element
  renderWithLayout(layout: LayoutRect, theme?: any): string
  minSize(): [number, number]
  maxSize(): [number | null, number | null]
  canGrowHorizontal(): boolean
  canGrowVertical(): boolean
}
```

### Responsive Widget Example

```typescript
class ResponsiveDataTable implements ResponsiveWidget {
  // ... BaseWidget implementation

  toElement(): Element {
    return this.render()
  }

  renderWithLayout(layout: LayoutRect, theme?: any): string {
    // Adapt rendering based on available space
    const { width, height } = layout
    
    if (width < 600) {
      return this.renderCompactTable(layout, theme)
    } else {
      return this.renderFullTable(layout, theme)
    }
  }

  minSize(): [number, number] {
    // Minimum width and height
    return [300, 200]
  }

  maxSize(): [number | null, number | null] {
    // Maximum width and height (null = unlimited)
    return [null, 800]
  }

  canGrowHorizontal(): boolean {
    return true
  }

  canGrowVertical(): boolean {
    return this._config.virtualScrolling || false
  }

  private renderCompactTable(layout: LayoutRect, theme?: any): string {
    // Render mobile-friendly table
    return this.createCompactTableHTML(layout, theme)
  }

  private renderFullTable(layout: LayoutRect, theme?: any): string {
    // Render full desktop table
    return this.createFullTableHTML(layout, theme)
  }
}
```

## Widget Registration

### Registering Custom Widgets

```typescript
import { WidgetFactory } from 'reactive-tui-ts'

// Register a custom widget
WidgetFactory.register('custom-widget', CustomWidget)

// Register with factory method
WidgetFactory.registerFactory('custom-widget', (config) => {
  return new CustomWidget(config)
})

// Use the registered widget
const customWidget = WidgetFactory.create('custom-widget', {
  id: 'my-custom',
  customProperty: 'value'
})
```

### Custom Widget Example

```typescript
interface CustomWidgetConfig extends BaseWidgetConfig {
  title: string
  content: string
  collapsible?: boolean
  collapsed?: boolean
}

class CustomWidget implements BaseWidget {
  readonly id: string
  readonly type = 'custom-widget'
  private _config: CustomWidgetConfig

  constructor(config: CustomWidgetConfig) {
    this.id = config.id
    this._config = { ...config }
  }

  get config(): CustomWidgetConfig {
    return { ...this._config }
  }

  render(): Element {
    return div()
      .id(this.id)
      .classes(['widget', 'custom-widget'])
      .child(this.renderHeader())
      .child(this.renderContent())
      .build()
  }

  private renderHeader(): Element {
    const header = div()
      .class('custom-widget-header')
      .child(text(this._config.title, { class: 'title' }))

    if (this._config.collapsible) {
      header.child(
        button({
          id: `${this.id}-toggle`,
          text: this._config.collapsed ? '▶' : '▼',
          variant: 'ghost',
          onClick: () => this.toggle()
        })
      )
    }

    return header.build()
  }

  private renderContent(): Element {
    if (this._config.collapsed) {
      return div().build() // Empty when collapsed
    }

    return div()
      .class('custom-widget-content')
      .content(this._config.content)
      .build()
  }

  private toggle(): void {
    this._config.collapsed = !this._config.collapsed
    // Trigger re-render
  }

  update(config: Partial<CustomWidgetConfig>): void {
    this._config = { ...this._config, ...config }
  }

  destroy(): void {
    // Cleanup
  }

  validate(): boolean {
    return !!(this.id && this._config.title)
  }
}

// Register the custom widget
WidgetFactory.register('custom-widget', CustomWidget)
```

## Widget Builders

Many widgets provide builder patterns for complex configuration:

```typescript
import { TreeBuilder, ProgressBuilder, FormBuilder } from 'reactive-tui-ts'

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

// Form builder
const form = new FormBuilder()
  .id('user-form')
  .addField('name', 'text', { label: 'Name', required: true })
  .addField('email', 'email', { label: 'Email', required: true })
  .addField('role', 'select', { 
    label: 'Role', 
    options: ['User', 'Admin'] 
  })
  .addSubmitButton('Create User')
  .build()
```

## Widget Lifecycle

### Lifecycle Methods

```typescript
interface WidgetLifecycle {
  onCreate?(): void
  onMount?(): void
  onUpdate?(changes: any): void
  onUnmount?(): void
  onDestroy?(): void
}

class LifecycleWidget implements BaseWidget, WidgetLifecycle {
  // ... BaseWidget implementation

  onCreate(): void {
    console.log('Widget created')
    this.initializeState()
  }

  onMount(): void {
    console.log('Widget mounted')
    this.setupEventListeners()
    this.loadInitialData()
  }

  onUpdate(changes: any): void {
    console.log('Widget updated', changes)
    this.handleStateChanges(changes)
  }

  onUnmount(): void {
    console.log('Widget unmounted')
    this.cleanupEventListeners()
    this.saveState()
  }

  onDestroy(): void {
    console.log('Widget destroyed')
    this.cleanup()
  }

  private initializeState(): void {
    // Initialize widget state
  }

  private setupEventListeners(): void {
    // Set up event listeners
  }

  private loadInitialData(): void {
    // Load any required data
  }

  private handleStateChanges(changes: any): void {
    // Handle state changes
  }

  private cleanupEventListeners(): void {
    // Remove event listeners
  }

  private saveState(): void {
    // Save widget state
  }

  private cleanup(): void {
    // Final cleanup
  }
}
```

## Widget Manager

### Managing Widget Instances

```typescript
class WidgetManager {
  private widgets = new Map<string, BaseWidget>()

  create<T extends BaseWidget>(type: string, config: any): T {
    const widget = WidgetFactory.create(type, config) as T
    this.widgets.set(widget.id, widget)
    
    if ('onCreate' in widget) {
      (widget as any).onCreate()
    }
    
    return widget
  }

  get(id: string): BaseWidget | undefined {
    return this.widgets.get(id)
  }

  update(id: string, config: any): void {
    const widget = this.widgets.get(id)
    if (widget) {
      widget.update(config)
      
      if ('onUpdate' in widget) {
        (widget as any).onUpdate(config)
      }
    }
  }

  destroy(id: string): void {
    const widget = this.widgets.get(id)
    if (widget) {
      if ('onDestroy' in widget) {
        (widget as any).onDestroy()
      }
      
      widget.destroy()
      this.widgets.delete(id)
    }
  }

  destroyAll(): void {
    for (const [id, widget] of this.widgets) {
      this.destroy(id)
    }
  }

  validate(id: string): boolean {
    const widget = this.widgets.get(id)
    return widget ? widget.validate() : false
  }

  validateAll(): boolean {
    return Array.from(this.widgets.values()).every(widget => widget.validate())
  }
}
```

## Best Practices

### 1. Use Factory Methods for Consistency

```typescript
// ✅ Good - use factory methods
const button = WidgetFactory.create('button', config)
const input = createWidget('input', config)

// ❌ Avoid - direct instantiation
const button = new ButtonWidget(config)
```

### 2. Implement Proper Cleanup

```typescript
class ProperWidget implements BaseWidget {
  private eventListeners: (() => void)[] = []
  private timers: NodeJS.Timeout[] = []

  destroy(): void {
    // Clean up event listeners
    this.eventListeners.forEach(cleanup => cleanup())
    this.eventListeners = []

    // Clear timers
    this.timers.forEach(timer => clearTimeout(timer))
    this.timers = []

    // Clear references
    this.clearReferences()
  }

  private addEventListener(cleanup: () => void): void {
    this.eventListeners.push(cleanup)
  }

  private addTimer(timer: NodeJS.Timeout): void {
    this.timers.push(timer)
  }
}
```

### 3. Validate Widget Configuration

```typescript
class ValidatedWidget implements BaseWidget {
  constructor(config: WidgetConfig) {
    this.validateConfig(config)
    this._config = config
  }

  private validateConfig(config: WidgetConfig): void {
    if (!config.id) {
      throw new Error('Widget ID is required')
    }

    if (config.id.length === 0) {
      throw new Error('Widget ID cannot be empty')
    }

    if (!/^[a-zA-Z][a-zA-Z0-9-_]*$/.test(config.id)) {
      throw new Error('Widget ID must start with a letter and contain only letters, numbers, hyphens, and underscores')
    }
  }

  validate(): boolean {
    return this.validateConfig(this._config)
  }
}
```

## Related APIs

- **[BaseWidget](./base-widget)** - Base widget interface
- **[ResponsiveWidget](./responsive-widget)** - Responsive widget interface
- **[Widget Builders](./widget-builders)** - Builder pattern implementations
- **[Component System](../concepts/components)** - Component-based architecture

## Examples

- **[Custom Widgets](../../examples/advanced/custom-widgets)** - Creating custom widgets
- **[Widget Composition](../../examples/advanced/widget-composition)** - Combining widgets
- **[Responsive Widgets](../../examples/advanced/responsive-widgets)** - Adaptive widgets
