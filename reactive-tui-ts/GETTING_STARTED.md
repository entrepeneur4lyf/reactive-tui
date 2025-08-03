# Getting Started with Reactive TUI TypeScript

This guide will help you get up and running with Reactive TUI using TypeScript in just a few minutes.

## 🚀 Quick Start

### Prerequisites

- **Bun** >= 1.0.0 (recommended) or **Node.js** >= 16.0.0
- **TypeScript** >= 5.0.0
- A modern terminal with ANSI color support

### Installation

```bash
# Clone this repository
git clone https://github.com/entrepeneur4lyf/reactive-tui.git
cd reactive-tui/reactive-tui-ts

# Install dependencies with Bun (recommended)
bun install

# Or with npm
npm install
```

### Run Your First Example

```bash
# Run the basic TypeScript demo
bun run example

# Or use the start script
bun start
```

You should see a beautiful terminal interface with:
- Modern CSS styling
- Interactive buttons
- Toast notifications
- Responsive layout
- Full TypeScript type safety

## 📝 Your First Application

Create a new file `my-app.ts`:

```typescript
import { JsTuiApp, TuiUtils, JsToast } from 'reactive-tui'

// Create the application
const app = new JsTuiApp()
app.setTitle('My First TUI App')

// Add some CSS styling
app.loadCss(`
  .container {
    background: #1e1e1e;
    color: #ffffff;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }
  
  .title {
    font-size: 1.5rem;
    font-weight: bold;
    color: #58a6ff;
  }
  
  .button {
    background: #238636;
    color: white;
    padding: 0.75rem 1.5rem;
    border: 1px solid #2ea043;
    border-radius: 6px;
    cursor: pointer;
  }
  
  .button:hover {
    background: #2ea043;
  }
`)

// Create UI elements
const container = TuiUtils.div()
container.addClass('container')

const title = TuiUtils.div()
title.addClass('title')
title.setContent('🎉 Hello, Reactive TUI!')

const button = TuiUtils.button()
button.addClass('button')
button.setContent('Click Me!')
button.makeFocusable(0)

// Build the UI
container.addChild(title)
container.addChild(button)

// Start the app
app.setComponent(container)
const status = app.start()
console.log(`App started: ${status}`)
```

Run it with:
```bash
bun run my-app.ts
```

## 🎯 Key Concepts

### 1. Type Safety

Reactive TUI provides full TypeScript support:

```typescript
import { JsElement, TuiUtils } from 'reactive-tui'

// All functions are fully typed
const element: JsElement = TuiUtils.div()
element.setId('my-element')        // ✅ Type-safe
element.addClass('my-class')       // ✅ Type-safe
element.setContent('Hello')        // ✅ Type-safe
// element.invalidMethod()         // ❌ TypeScript error
```

### 2. CSS-First Design

Style your terminal interfaces with familiar CSS:

```css
/* Modern layouts */
.grid-container {
  display: grid;
  grid-template-columns: 1fr 2fr 1fr;
  gap: 1rem;
}

.flex-container {
  display: flex;
  justify-content: space-between;
  align-items: center;
}

/* Responsive design */
@media (max-width: 80) {
  .responsive {
    flex-direction: column;
  }
}

/* Beautiful styling */
.card {
  background: #161b22;
  border: 1px solid #30363d;
  border-radius: 8px;
  padding: 1.5rem;
}
```

### 3. Component Architecture

Build reusable components:

```typescript
class Button {
  private element: JsElement

  constructor(text: string, variant: 'primary' | 'secondary' = 'primary') {
    this.element = TuiUtils.button()
    this.element.addClass(`btn btn-${variant}`)
    this.element.setContent(text)
    this.element.makeFocusable()
  }

  onClick(handler: () => void): void {
    this.element.setAttribute('data-click', 'true')
    // Add click handling logic
  }

  getElement(): JsElement {
    return this.element
  }
}

// Usage
const saveButton = new Button('Save', 'primary')
const cancelButton = new Button('Cancel', 'secondary')
```

### 4. State Management

Handle application state with TypeScript:

```typescript
interface AppState {
  currentView: 'home' | 'settings' | 'about'
  user: { name: string; email: string } | null
  notifications: Array<{ id: string; message: string; type: string }>
}

class StateManager {
  private state: AppState = {
    currentView: 'home',
    user: null,
    notifications: []
  }

  setState(updates: Partial<AppState>): void {
    this.state = { ...this.state, ...updates }
    this.notifyListeners()
  }

  getState(): AppState {
    return { ...this.state }
  }

  private notifyListeners(): void {
    // Trigger UI updates
  }
}
```

## 🎨 Styling Guide

### Color System

Use semantic colors for consistency:

```css
:root {
  --color-primary: #58a6ff;
  --color-secondary: #8b949e;
  --color-success: #238636;
  --color-warning: #d29922;
  --color-error: #f85149;
  --color-background: #0d1117;
  --color-surface: #161b22;
  --color-border: #30363d;
}

.btn-primary {
  background: var(--color-primary);
  color: white;
}

.btn-success {
  background: var(--color-success);
  color: white;
}
```

### Layout Patterns

Common layout patterns for terminal interfaces:

```css
/* Dashboard layout */
.dashboard {
  display: grid;
  grid-template-areas: 
    "header header header"
    "sidebar main aside"
    "footer footer footer";
  grid-template-rows: auto 1fr auto;
  grid-template-columns: 200px 1fr 200px;
  height: 100vh;
}

/* Card grid */
.card-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
}

/* Centered content */
.centered {
  display: flex;
  justify-content: center;
  align-items: center;
  min-height: 100vh;
}
```

## 🧪 Testing Your App

### Type Checking

```bash
# Check types without building
bun run typecheck

# Watch mode for development
bun run typecheck --watch
```

### Code Quality

```bash
# Lint your code
bun run lint

# Format your code
bun run format
```

### Testing

```bash
# Run tests
bun test

# Watch mode
bun test --watch
```

## 🔧 Development Workflow

### Hot Reload Development

```bash
# Start development server with hot reload
bun run dev
```

### Building for Production

```bash
# Build TypeScript declarations and JavaScript
bun run build

# Clean build artifacts
bun run clean
```

### Project Structure

Organize your project like this:

```
my-tui-app/
├── src/
│   ├── components/          # Reusable UI components
│   ├── views/              # Application screens/views
│   ├── utils/              # Utility functions
│   ├── types/              # TypeScript type definitions
│   └── index.ts            # Main entry point
├── styles/
│   ├── base.css            # Base styles
│   ├── components.css      # Component styles
│   └── themes.css          # Theme definitions
├── package.json
├── tsconfig.json
└── README.md
```

## 📚 Next Steps

1. **Explore Examples**: Check out the `examples/` directory for more complex applications
2. **Read the API Reference**: See the full API documentation in the main README
3. **Join the Community**: Participate in discussions and share your creations
4. **Build Something Amazing**: Start building your own terminal application!

## 🆘 Getting Help

- **Documentation**: [Main README](./README.md)
- **API Reference**: [Reactive TUI Docs](https://github.com/entrepeneur4lyf/reactive-tui#api-reference)
- **Issues**: [GitHub Issues](https://github.com/entrepeneur4lyf/reactive-tui/issues)
- **Discussions**: [GitHub Discussions](https://github.com/entrepeneur4lyf/reactive-tui/discussions)

## 🎉 You're Ready!

You now have everything you need to start building beautiful, type-safe terminal applications with Reactive TUI and TypeScript. Happy coding! 🚀
