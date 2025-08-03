# Examples Guide - Reactive TUI TypeScript

This guide will help you run and understand the TypeScript examples for Reactive TUI.

## 🚀 Quick Start

### Prerequisites

Before running the examples, make sure you have:

- **Bun** >= 1.0.0 (recommended) or **Node.js** >= 16.0.0
- **Git** for cloning the repository
- A modern terminal with ANSI color support
- Terminal size of at least 80x24 characters (recommended: 120x30)

### Installation

```bash
# Clone the repository
git clone https://github.com/entrepeneur4lyf/reactive-tui.git
cd reactive-tui/reactive-tui-ts

# Install dependencies
bun install

# Verify installation
bun run typecheck
```

## 📋 Available Examples

### 1. **Basic Demo** - `example.ts`
The main demonstration showcasing core features.

```bash
# Run the basic demo
bun run example

# Alternative commands
bun start
bun run demo:basic
```

**What it demonstrates:**
- Modern CSS styling with GitHub-inspired dark theme
- Interactive components (buttons, cards, status indicators)
- Responsive layout with CSS Grid and Flexbox
- Toast notification system
- TypeScript integration with full type safety

### 2. **Hello World Example**
Simple introduction to Reactive TUI.

```bash
# Run from the examples module
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.helloWorld();
app.start();
"
```

**What it demonstrates:**
- Basic application setup
- Simple text rendering
- CSS styling fundamentals

### 3. **Button Examples**
Interactive button demonstrations.

```bash
# Run button examples
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.buttons();
app.start();
"
```

**What it demonstrates:**
- Different button variants (primary, secondary, danger)
- Button styling and interactions
- Focus management

### 4. **Card Layout Example**
Grid-based card layout demonstration.

```bash
# Run card layout example
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.cards();
app.start();
"
```

**What it demonstrates:**
- CSS Grid layouts
- Card component patterns
- Responsive design
- Content organization

### 5. **Dashboard Example**
Comprehensive dashboard with metrics and data.

```bash
# Run dashboard example
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.dashboard();
app.start();
"
```

**What it demonstrates:**
- Complex layouts with header and main content
- Status indicators and metrics
- Real-time data visualization patterns
- Professional dashboard design

### 6. **Toast Notifications**
Notification system demonstration.

```bash
# Run toast example
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.toasts();
app.start();
"
```

**What it demonstrates:**
- Toast notification system
- Different notification types (info, success, warning, error)
- User feedback patterns

### 7. **Theme Showcase**
Color themes and styling demonstration.

```bash
# Run theme example
bun -e "
import { EXAMPLES } from './src/examples.ts';
const app = EXAMPLES.themes();
app.start();
"
```

**What it demonstrates:**
- Theme system capabilities
- Color palette showcase
- Styling variations

## 🛠️ Development Commands

### Code Quality

```bash
# Type checking
bun run typecheck

# Linting
bun run lint

# Build the project
bun run build

# Clean build artifacts
bun run clean
```

### Development Workflow

```bash
# Development with hot reload
bun run dev

# Watch mode for type checking
bun run typecheck --watch
```

## 🎯 Creating Your Own Examples

### 1. Simple Example

Create a new file `my-example.ts`:

```typescript
import { JsTuiApp, TuiUtils } from 'reactive-tui'

// Create the application
const app = new JsTuiApp()
app.setTitle('My Custom Example')

// Add CSS styling
app.loadCss(`
  .container {
    background: #1e1e1e;
    color: #ffffff;
    padding: 2rem;
    display: flex;
    justify-content: center;
    align-items: center;
    min-height: 100vh;
  }
  
  .message {
    font-size: 1.5rem;
    text-align: center;
  }
`)

// Create UI
const container = TuiUtils.div()
container.addClass('container')

const message = TuiUtils.div()
message.addClass('message')
message.setContent('🎉 Hello from my custom example!')

container.addChild(message)

// Start the app
app.setComponent(container)
const status = app.start()
console.log(`App started: ${status}`)
```

Run it:
```bash
bun run my-example.ts
```

### 2. Using TypeScript Utilities

```typescript
import { TuiAppBuilder, TypeScriptTuiUtils, generateCompleteTheme } from './src/index.ts'

const app = new TuiAppBuilder('Advanced Example')
  .loadCSS(generateCompleteTheme())

const container = TuiUtils.div()
container.addClass('app-container flex flex-col items-center justify-center')

// Use TypeScript utilities
const button = TypeScriptTuiUtils.createButton('Click Me!', 'primary')
const card = TypeScriptTuiUtils.createCard('My Card', 'This is a custom card example')

container.addChild(card.getElement())
container.addChild(button)

app.setRoot(container)
app.start()
```

## 🎨 Styling Guide

### CSS Classes Available

The examples use a comprehensive CSS framework with these utility classes:

#### Layout
- `.flex`, `.flex-col`, `.flex-row`
- `.grid`, `.grid-cols-1`, `.grid-cols-2`, `.grid-cols-3`
- `.justify-center`, `.items-center`, `.items-stretch`

#### Spacing
- `.p-1`, `.p-2`, `.p-4`, `.p-6`, `.p-8` (padding)
- `.m-1`, `.m-2`, `.m-4`, `.m-6`, `.m-8` (margin)
- `.gap-1`, `.gap-2`, `.gap-4`, `.gap-6` (gap)

#### Components
- `.btn`, `.btn-primary`, `.btn-secondary`, `.btn-danger`
- `.card`, `.card-header`, `.card-body`, `.card-footer`
- `.text-center`, `.text-lg`, `.text-2xl`

#### Colors
- `.text-primary`, `.text-secondary`, `.text-muted`
- `.bg-primary`, `.bg-secondary`, `.bg-success`

### Custom CSS

You can add custom CSS to any example:

```typescript
app.loadCss(`
  .my-custom-style {
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
    border-radius: 8px;
    padding: 1rem;
    animation: pulse 2s infinite;
  }
  
  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.7; }
  }
`)
```

## 🐛 Troubleshooting

### Common Issues

#### 1. **Terminal Size Too Small**
```
Error: Terminal size insufficient
```
**Solution:** Resize your terminal to at least 80x24 characters.

#### 2. **TypeScript Errors**
```
Type errors in compilation
```
**Solution:** Run `bun run typecheck` to see detailed errors.

#### 3. **Import Errors**
```
Cannot find module 'reactive-tui'
```
**Solution:** Make sure dependencies are installed: `bun install`

#### 4. **Application Not Starting**
```
App status: failed
```
**Solution:** Check that you're in a proper terminal environment, not a web-based terminal.

### Debug Mode

Enable debug output:

```typescript
import { ValidationUtils } from './src/index.ts'

// Check terminal requirements
if (!ValidationUtils.validateTerminalSize(80, 24)) {
  console.warn('Terminal size may be too small for optimal display')
}

// Validate CSS
const { isValid, errors } = ValidationUtils.validateCSS(yourCSS)
if (!isValid) {
  console.warn('CSS validation issues:', errors)
}
```

## 📚 Learning Path

### Beginner
1. Start with `bun run example` to see the main demo
2. Try the Hello World example
3. Experiment with button examples
4. Read the [Getting Started Guide](./GETTING_STARTED.md)

### Intermediate
1. Explore card layout and dashboard examples
2. Study the CSS patterns in `src/utils.ts`
3. Create your own simple examples
4. Learn about TypeScript utilities

### Advanced
1. Build complex applications using the component system
2. Create custom themes and styling
3. Implement responsive layouts
4. Contribute new examples to the project

## 🤝 Contributing Examples

Want to add your own example? Follow these steps:

1. Create your example in a new file
2. Add it to the `EXAMPLES` object in `src/examples.ts`
3. Update this guide with documentation
4. Submit a pull request

### Example Template

```typescript
export function createMyExample(): JsTuiApp {
  const app = new TuiAppBuilder('My Example')
    .loadCSS(generateCompleteTheme())

  // Your implementation here

  return app.getApp()
}

// Add to EXAMPLES object
export const EXAMPLES = {
  // ... existing examples
  myExample: createMyExample
}
```

## 📖 Additional Resources

- [Main README](./README.md) - Project overview and API reference
- [Getting Started Guide](./GETTING_STARTED.md) - Step-by-step tutorial
- [Reactive TUI Documentation](https://github.com/entrepeneur4lyf/reactive-tui) - Core framework docs
- [TypeScript Handbook](https://www.typescriptlang.org/docs/) - TypeScript reference

---

Happy coding with Reactive TUI! 🚀
