---
sidebar_position: 1
---

# API Overview

Complete reference for the Reactive TUI TypeScript SDK API.

## Core API

### Application Creation

#### `createApp(config: AppConfig): TuiApp`

Creates a new TUI application instance.

```typescript
import { createApp } from 'reactive-tui-ts'

const app = createApp({
  title: 'My App',
  stylesheet: './styles.css',
  component: () => createMainComponent()
})
```

**Parameters:**
- `config: AppConfig` - Application configuration object

**Returns:** `TuiApp` - Application instance

### Element Builders

#### `div(props?: ElementProps): ElementBuilder`

Creates a div element builder.

```typescript
import { div } from 'reactive-tui-ts'

const container = div({ class: 'container', id: 'main' })
  .child(/* child elements */)
  .build()
```

#### `text(content: string, props?: ElementProps): ElementBuilder`

Creates a text element builder.

```typescript
import { text } from 'reactive-tui-ts'

const title = text('Hello World!', { class: 'title' })
const subtitle = text('Welcome to TUI', { class: 'subtitle' })
```

#### `span(content?: string, props?: ElementProps): ElementBuilder`

Creates a span element builder.

```typescript
import { span } from 'reactive-tui-ts'

const highlight = span('Important!', { class: 'highlight' })
```

## Widget API

### Button Widget

#### `button(config: ButtonConfig): ButtonWidget`

Creates a button widget with comprehensive styling and interaction options.

```typescript
import { button } from 'reactive-tui-ts'

const btn = button({
  id: 'submit-btn',
  text: 'Submit',
  variant: 'filled',
  color: 'primary',
  size: 'md',
  onClick: () => console.log('Clicked!')
})
```

**Configuration:**
- `id: string` - Unique identifier (required)
- `text?: string` - Button text
- `variant?: 'filled' | 'outlined' | 'ghost' | 'link'` - Visual style
- `color?: 'primary' | 'secondary' | 'success' | 'warning' | 'error'` - Color theme
- `size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'` - Button size
- `disabled?: boolean` - Disabled state
- `onClick?: () => void` - Click event handler

### Input Widget

#### `input(config: InputConfig): InputWidget`

Creates an input field for user text entry.

```typescript
import { input } from 'reactive-tui-ts'

const nameInput = input({
  id: 'name',
  type: 'text',
  placeholder: 'Enter your name...',
  variant: 'outlined',
  onInput: (value) => console.log('Input:', value)
})
```

### Tree Widget

#### `tree(id: string, config?: Partial<TreeConfig>): Tree`

Creates a hierarchical tree view for displaying nested data.

```typescript
import { tree } from 'reactive-tui-ts'

const fileTree = tree('file-tree', {
  expandable: true,
  multiSelect: false,
  lazyLoading: true
})

// Add nodes programmatically
const rootId = fileTree.addNode('root', 'Project', 'folder')
fileTree.addNode('src', 'src/', 'folder', rootId)
```

### Data Table Widget

#### `dataTable<T = any>(config: DataTableProps): any`

Creates a data table for displaying tabular data.

```typescript
import { dataTable, createColumn } from 'reactive-tui-ts'

const table = dataTable({
  id: 'users-table',
  columns: [
    createColumn({ id: 'name', title: 'Name', width: 200 }),
    createColumn({ id: 'email', title: 'Email', width: 300 })
  ],
  data: userData,
  config: {
    sortable: true,
    selectable: true,
    bordered: true
  }
})
```

### Modal Widget

#### `modal(config: ModalConfig): ModalWidget`

Creates a modal dialog for overlaying content.

```typescript
import { modal } from 'reactive-tui-ts'

const confirmModal = modal({
  id: 'confirm-modal',
  title: 'Confirm Action',
  content: 'Are you sure you want to proceed?',
  buttons: [
    { text: 'Cancel', variant: 'outlined' },
    { text: 'Confirm', variant: 'filled', color: 'primary' }
  ]
})
```

### Toast Widget

#### `toast(config: ToastConfig): ToastWidget`

Creates a toast notification for temporary messages.

```typescript
import { toast } from 'reactive-tui-ts'

const successToast = toast({
  id: 'success-toast',
  message: 'Operation completed successfully!',
  variant: ToastVariant.Success,
  duration: 3000,
  position: ToastPosition.TopRight
})
```

## Theme API

### Color Themes

#### `colorThemes: Record<string, ColorTheme>`

Predefined color themes for consistent styling.

```typescript
import { colorThemes } from 'reactive-tui-ts'

const darkTheme = colorThemes.dark
const lightTheme = colorThemes.light
const blueTheme = colorThemes.blue
```

#### `getColorTheme(name: string): ColorTheme`

Retrieves a color theme by name.

```typescript
import { getColorTheme } from 'reactive-tui-ts'

const theme = getColorTheme('dark-professional')
```

### Border Themes

#### `borderThemes: Record<string, BorderTheme>`

Predefined border styles for UI elements.

```typescript
import { borderThemes } from 'reactive-tui-ts'

const roundedBorders = borderThemes.rounded
const sharpBorders = borderThemes.sharp
```

### Theme Manager

#### `ThemeManager`

Advanced theme management and customization.

```typescript
import { ThemeManager } from 'reactive-tui-ts'

const themeManager = new ThemeManager()
await themeManager.loadTheme('dark-professional')
```

## Type Definitions

### Core Types

#### `AppConfig`

Configuration object for creating applications.

```typescript
interface AppConfig {
  stylesheet?: string
  component: () => Element | ElementBuilder
  width?: number
  height?: number
  targetFps?: number
}
```

#### `Element`

Represents a rendered UI element.

```typescript
interface Element {
  tag: string
  id: string | null
  classes: string[]
  attributes: Record<string, string>
  content: string | null
  children: Element[]
  focusable: boolean
  focused: boolean
}
```

#### `ElementBuilder`

Builder interface for creating elements.

```typescript
interface ElementBuilder {
  id(id: string): ElementBuilder
  class(className: string): ElementBuilder
  classes(classNames: string[]): ElementBuilder
  attr(name: string, value: string): ElementBuilder
  content(content: string): ElementBuilder
  child(child: Element | ElementBuilder): ElementBuilder
  build(): Element
}
```

### Widget Types

#### `ButtonConfig`

Configuration for button widgets.

```typescript
interface ButtonConfig {
  id: string
  text?: string
  variant?: 'filled' | 'outlined' | 'ghost' | 'link'
  color?: 'primary' | 'secondary' | 'success' | 'warning' | 'error'
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl'
  disabled?: boolean
  onClick?: () => void
}
```

#### `InputConfig`

Configuration for input widgets.

```typescript
interface InputConfig {
  id: string
  type?: 'text' | 'password' | 'email' | 'number'
  placeholder?: string
  variant?: 'outlined' | 'filled' | 'underlined'
  disabled?: boolean
  onInput?: (value: string) => void
  onFocus?: () => void
  onBlur?: () => void
}
```

## Utility Functions

### Layout Helpers

#### `flexRow(props?: ElementProps): ElementBuilder`

Creates a horizontal flex container.

```typescript
import { flexRow } from 'reactive-tui-ts'

const row = flexRow({ class: 'button-row' })
  .child(button1)
  .child(button2)
  .build()
```

#### `flexColumn(props?: ElementProps): ElementBuilder`

Creates a vertical flex container.

```typescript
import { flexColumn } from 'reactive-tui-ts'

const column = flexColumn({ class: 'form-column' })
  .child(input1)
  .child(input2)
  .build()
```

### Content Helpers

#### `line(content?: string): ElementBuilder`

Creates a single line of text.

```typescript
import { line } from 'reactive-tui-ts'

const separator = line('─'.repeat(40))
```

#### `hr(): ElementBuilder`

Creates a horizontal rule separator.

```typescript
import { hr } from 'reactive-tui-ts'

const divider = hr()
```

## Error Handling

### TuiError

Base error class for TUI-specific errors.

```typescript
try {
  await app.run()
} catch (error) {
  if (error instanceof TuiError) {
    console.error('TUI Error:', error.message)
  }
}
```

## Constants

### Key Codes

```typescript
export const KEY_CODES = {
  ENTER: 'Enter',
  ESCAPE: 'Escape',
  TAB: 'Tab',
  SPACE: ' ',
  ARROW_UP: 'ArrowUp',
  ARROW_DOWN: 'ArrowDown',
  ARROW_LEFT: 'ArrowLeft',
  ARROW_RIGHT: 'ArrowRight'
}
```

### Colors

```typescript
export const COLORS = {
  PRIMARY: '#0969da',
  SECONDARY: '#656d76',
  SUCCESS: '#1a7f37',
  WARNING: '#9a6700',
  ERROR: '#cf222e',
  INFO: '#0969da'
}
```

## Next Steps

- **[Core API](./core/create-app)** - Detailed core API documentation
- **[Widget API](./widgets/overview)** - Complete widget reference
- Theme API - Theming and styling reference (coming soon)
- Examples - See the API in action (coming soon)
