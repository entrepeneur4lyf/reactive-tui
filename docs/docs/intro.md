---
sidebar_position: 1
---

# Introduction to Reactive TUI

**Reactive TUI** is a revolutionary CSS-styled terminal user interface framework that brings modern web development paradigms to terminal applications. Built with a Rust core and TypeScript layer, it delivers unparalleled performance while maintaining familiar web-like development patterns.

## Why Reactive TUI?

### 🚀 **Web-like Development Experience**
- **CSS Styling**: Use familiar CSS properties including flexbox, grid, animations, and responsive design
- **Component-Based**: Build reusable UI components with props and state management
- **TypeScript First**: Full type safety with comprehensive IntelliSense support

### ⚡ **High Performance**
- **Rust Core**: Native performance with zero-cost abstractions
- **Virtual Rendering**: Efficient updates with dirty region tracking
- **NAPI Bindings**: Seamless Node.js integration without performance penalties

### 🎨 **Rich UI Capabilities**
- **30+ Built-in Widgets**: From simple buttons to complex data tables
- **Animation System**: Smooth property animations with configurable easing
- **Theme Support**: JSON-based themes with dark/light mode switching
- **Responsive Design**: Breakpoint-based layouts that adapt to terminal size

### 🔧 **Developer Friendly**
- **Hot Reload**: Instant feedback during development
- **Comprehensive Docs**: Extensive examples and API documentation
- **Plugin Architecture**: Extensible widget system for custom components

## Architecture Overview

The framework consists of three main layers:

1. **Rust Core (`reactive-tui`)**
   - High-performance rendering engine
   - Layout calculations and virtual DOM
   - Terminal abstraction layer
   - Event handling system

2. **TypeScript Layer (`tui-bun`)**
   - Developer-facing API
   - CSS parsing and styling
   - Component lifecycle management
   - State management utilities

3. **NAPI Bridge**
   - Zero-copy data transfer
   - Async operation support
   - Memory-safe FFI bindings

## Quick Start

### Installation

```bash
npm install reactive-tui
```

### Hello World

```typescript
import { JsTuiApp, TuiUtils } from 'reactive-tui';

const app = new JsTuiApp();
app.setTitle('Hello Reactive TUI');

// Create a styled button
const button = TuiUtils.button();
button.setText('Hello, World!');
button.addClass('primary-button');

// Apply CSS styling
app.loadCss(`
  .primary-button {
    background: #007acc;
    color: white;
    padding: 1rem 2rem;
    border-radius: 4px;
    border: 1px solid #005a9f;
    font-weight: bold;
  }
  .primary-button:hover {
    background: #005a9f;
  }
`);

// Handle events
button.onClick(() => {
  console.log('Hello from terminal UI!');
});

app.setComponent(button);
app.start();
```

### Advanced Example with State

```typescript
import { JsTuiApp, TuiUtils, JsReactiveState } from 'reactive-tui';

const app = new JsTuiApp();

// Create reactive state
const state = new JsReactiveState({ count: 0 });

// Create container
const container = TuiUtils.div();
container.addClass('counter-app');

// Create counter display
const display = TuiUtils.text();
display.addClass('counter-display');

// Create buttons
const decrementBtn = TuiUtils.button();
decrementBtn.setText('−');
decrementBtn.addClass('counter-btn decrement');

const incrementBtn = TuiUtils.button();
incrementBtn.setText('+');
incrementBtn.addClass('counter-btn increment');

// Update display when state changes
state.watch('count', (newValue) => {
  display.setText(`Count: ${newValue}`);
});

// Handle button clicks
decrementBtn.onClick(() => {
  state.setState({ count: state.getState().count - 1 });
});

incrementBtn.onClick(() => {
  state.setState({ count: state.getState().count + 1 });
});

// Build UI structure
const buttonContainer = TuiUtils.div();
buttonContainer.addClass('button-row');
buttonContainer.appendChild(decrementBtn);
buttonContainer.appendChild(incrementBtn);

container.appendChild(display);
container.appendChild(buttonContainer);

// Apply styles
app.loadCss(`
  .counter-app {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    padding: 2rem;
    background: #1e1e1e;
    color: #ffffff;
    border: 1px solid #333;
  }
  
  .counter-display {
    font-size: 1.5rem;
    font-weight: bold;
    color: #61dafb;
  }
  
  .button-row {
    display: flex;
    gap: 1rem;
  }
  
  .counter-btn {
    padding: 0.5rem 1rem;
    font-size: 1.2rem;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
  }
  
  .decrement {
    background: #ff6b6b;
    border: 1px solid #ff5252;
  }
  
  .increment {
    background: #4ecdc4;
    border: 1px solid #26a69a;
  }
  
  .counter-btn:hover {
    opacity: 0.8;
  }
`);

// Initialize display
display.setText(`Count: ${state.getState().count}`);

app.setComponent(container);
app.start();
```

## What's Next?

- **[Core Concepts](./core-concepts)**: Understand the fundamental building blocks
- **[Tutorial](./tutorial/getting-started)**: Build your first application step-by-step
- **[API Reference](./api/overview)**: Complete API documentation

## Platform Support

Reactive TUI supports all major platforms:

- **Node.js**: 16.x, 18.x, 20.x, 21.x
- **Bun**: Latest stable
- **Operating Systems**: Windows, macOS, Linux
- **Architectures**: x64, ARM64

## Commercial Support

For enterprise users requiring commercial licensing, priority support, or custom development:

- **Enterprise License**: Contact [sales@reactive-tui.dev](mailto:sales@reactive-tui.dev)
- **Support**: [support@reactive-tui.dev](mailto:support@reactive-tui.dev)
- **Revenue Threshold**: Free for companies under $1M annual revenue

---

Ready to revolutionize your terminal applications? Let's [get started](./tutorial/getting-started)! 🚀
