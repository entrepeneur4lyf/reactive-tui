---
sidebar_position: 1
---

# createApp()

The `createApp()` function is the entry point for all Reactive TUI applications. It creates and configures a new TUI application instance.

## Signature

```typescript
function createApp(config: AppConfig): TuiApp
```

## Parameters

### `config: AppConfig`

The application configuration object with the following properties:

#### Required Properties

##### `component: () => Element | ElementBuilder`
A function that returns the root UI element of your application.

```typescript
const app = createApp({
  component: () =>
    div({ class: 'app' })
      .child(text('Hello World!'))
      .build()
})
```

#### Optional Properties

##### `stylesheet?: string`
Path to a CSS file for styling your application.

```typescript
const app = createApp({
  stylesheet: './styles/app.css',
  component: () => /* ... */
})
```

##### `width?: number`
Application width in characters.

```typescript
const app = createApp({
  width: 120,
  component: () => /* ... */
})
```

##### `height?: number`
Application height in characters.

```typescript
const app = createApp({
  height: 40,
  component: () => /* ... */
})
```

##### `targetFps?: number`
Target frames per second for rendering.

```typescript
const app = createApp({
  targetFps: 60,
  component: () => /* ... */
})
```

**Supported paths:**
- Relative paths: `'./styles/app.css'`
- Absolute paths: `'/path/to/styles.css'`
- URLs: `'https://example.com/styles.css'` (if supported)

## Return Value

### `TuiApp`

Returns a TUI application instance with the following methods:

#### `run(): Promise<void>`
Starts the application and displays the UI.

```typescript
const app = createApp({
  title: 'My App',
  component: () => createMainComponent()
})

await app.run()
```

#### `stop(): Promise<void>`
Gracefully stops the application.

```typescript
// Graceful shutdown
process.on('SIGINT', async () => {
  await app.stop()
  process.exit(0)
})
```

#### `setComponent(component: () => Element): void`
Updates the root component of the application.

```typescript
let currentView = 'home'

function updateView(newView: string) {
  currentView = newView
  app.setComponent(() => createView(currentView))
}
```

#### `updateStylesheet(css: string): Promise<void>`
Updates the application's CSS styles at runtime.

```typescript
const newCSS = `
  .container { background: #1e1e1e; }
  .text { color: #ffffff; }
`
await app.updateStylesheet(newCSS)
```

## Complete Example

```typescript
import { createApp, div, text, button, header, main, footer } from 'reactive-tui-ts'

// Application state
let count = 0

// Component functions
function createHeader() {
  return header({ class: 'app-header' })
    .child(text('Counter Application', { class: 'title' }))
    .build()
}

function createCounter() {
  return div({ class: 'counter' })
    .child(text(`Count: ${count}`, { class: 'count-display' }))
    .child(
      div({ class: 'button-group' })
        .child(
          button({
            id: 'decrement',
            text: '-',
            variant: 'outlined',
            onClick: () => {
              count--
              updateApp()
            }
          })
        )
        .child(
          button({
            id: 'increment',
            text: '+',
            variant: 'filled',
            color: 'primary',
            onClick: () => {
              count++
              updateApp()
            }
          })
        )
        .build()
    )
    .build()
}

function createFooter() {
  return footer({ class: 'app-footer' })
    .child(text('Press Ctrl+C to exit'))
    .build()
}

function createMainComponent() {
  return div({ class: 'app' })
    .child(createHeader())
    .child(
      main({ class: 'app-main' })
        .child(createCounter())
        .build()
    )
    .child(createFooter())
    .build()
}

// Create the application
const app = createApp({
  title: 'Counter App',
  stylesheet: './counter.css',
  component: createMainComponent
})

// Update function for reactive updates
function updateApp() {
  app.setComponent(createMainComponent)
}

// Error handling
async function startApp() {
  try {
    await app.run()
  } catch (error) {
    console.error('Application error:', error)
    process.exit(1)
  }
}

// Graceful shutdown
process.on('SIGINT', async () => {
  console.log('\nShutting down...')
  await app.stop()
  process.exit(0)
})

// Start the application
startApp()
```

## Advanced Usage

### Dynamic Configuration

```typescript
interface AppOptions {
  theme: 'light' | 'dark'
  debug: boolean
  title: string
}

function createTuiApp(options: AppOptions) {
  const stylesheet = options.theme === 'dark' 
    ? './styles/dark.css' 
    : './styles/light.css'
    
  return createApp({
    title: options.title,
    stylesheet,
    component: () => createMainComponent(options)
  })
}

// Usage
const app = createTuiApp({
  theme: 'dark',
  debug: true,
  title: 'My Advanced App'
})
```

### Conditional Stylesheets

```typescript
const isDevelopment = process.env.NODE_ENV === 'development'

const app = createApp({
  title: 'My App',
  stylesheet: isDevelopment ? './dev.css' : './prod.css',
  component: () => createMainComponent()
})
```

### Hot Reloading (Development)

```typescript
import { watch } from 'fs'

const app = createApp({
  title: 'Dev App',
  stylesheet: './app.css',
  component: () => createMainComponent()
})

// Watch for CSS changes in development
if (process.env.NODE_ENV === 'development') {
  watch('./app.css', async () => {
    const css = await fs.readFile('./app.css', 'utf-8')
    await app.updateStylesheet(css)
    console.log('🎨 Styles reloaded!')
  })
}
```

## Error Handling

### Application-Level Errors

```typescript
const app = createApp({
  title: 'My App',
  component: () => createMainComponent()
})

try {
  await app.run()
} catch (error) {
  if (error instanceof TuiError) {
    console.error('TUI Error:', error.message)
  } else {
    console.error('Unexpected error:', error)
  }
  
  // Log to file, send to monitoring service, etc.
  await logError(error)
  
  process.exit(1)
}
```

### Component-Level Errors

```typescript
function createSafeComponent() {
  try {
    return createComplexComponent()
  } catch (error) {
    console.error('Component error:', error)
    
    return div({ class: 'error-fallback' })
      .child(text('⚠️ Something went wrong'))
      .child(text('Please try again later'))
      .build()
  }
}

const app = createApp({
  title: 'Safe App',
  component: createSafeComponent
})
```

## Performance Considerations

### Efficient Updates

```typescript
// ✅ Good - update only when needed
let lastState = null

function updateIfChanged(newState: AppState) {
  if (JSON.stringify(newState) !== JSON.stringify(lastState)) {
    lastState = newState
    app.setComponent(() => createComponent(newState))
  }
}

// ❌ Avoid - unnecessary updates
function updateAlways(newState: AppState) {
  app.setComponent(() => createComponent(newState))
}
```

### Lazy Component Creation

```typescript
function createLazyComponent() {
  return div({ class: 'container' })
    .child(createHeader())
    .child(createMainContent()) // Only create when needed
    .build()
}

const app = createApp({
  title: 'Lazy App',
  component: createLazyComponent // Function reference, not call
})
```

## TypeScript Types

### AppConfig Interface

```typescript
interface AppConfig {
  title: string
  stylesheet?: string
  component: () => Element
}
```

### TuiApp Interface

```typescript
interface TuiApp {
  run(): Promise<void>
  stop(): Promise<void>
  setComponent(component: () => Element): void
  updateStylesheet(css: string): Promise<void>
}
```

## Related APIs

- Elements - Element creation and manipulation (coming soon)
- Components - Reusable component patterns (coming soon)
- Types - Complete TypeScript type definitions (coming soon)
- Themes - Styling and theming system (coming soon)

## Examples

- Hello World - Simple app creation (coming soon)
- Counter App - Interactive application (coming soon)
- File Manager - Complex application (coming soon)
- Dashboard - Real-time updates (coming soon)
