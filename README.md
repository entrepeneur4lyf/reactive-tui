| <img src="assets/logo.png" alt="Reactive TUI Logo" width="100" height="100"> | <h1>Reactive TUI</h1><br>**CSS-styled Terminal User Interfaces for Node.js and Bun**<br><br>[![NPM Version](https://img.shields.io/npm/v/reactive-tui.svg)](https://www.npmjs.com/package/reactive-tui) [![Build Status](https://img.shields.io/github/workflow/status/entrepeneur4lyf/reactive-tui/CI)](https://github.com/entrepeneur4lyf/reactive-tui/actions) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)<br>[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/) [![Rust](https://img.shields.io/badge/Rust-Powered-orange.svg)](https://www.rust-lang.org/) [![Coverage](https://img.shields.io/badge/Coverage-72.31%25-brightgreen.svg)](#testing) |
|---|---|

A revolutionary TUI framework that brings modern web development paradigms to terminal applications. Build beautiful, responsive terminal interfaces using familiar CSS styling and React-like components, all powered by high-performance Rust with seamless JavaScript integration.

## ✨ Features

### 🎨 **CSS-First Design**
- **Utility Classes**: Tailwind-inspired CSS classes (`flex`, `grid-cols-3`, `p-4`, `text-center`)
- **Responsive Design**: Terminal-aware layouts that adapt to window size
- **Theme System**: Built-in dark/light themes with custom theme support
- **Hot Reload**: Live CSS updates during development

### 🧩 **Rich Widget Library**
- **25+ Widgets**: Input, Button, DataTable, Modal, Toast, Progress, Spinner
- **Advanced Layouts**: CSS Grid, Flexbox, responsive containers
- **Interactive Components**: Forms with validation, autocomplete, menus
- **Data Visualization**: Charts, tables with sorting/filtering

### ⚡ **Performance & Integration**
- **Native Speed**: Rust-powered rendering with NAPI bindings
- **Zero Dependencies**: Self-contained with minimal JavaScript footprint
- **TypeScript Support**: Full type definitions and IntelliSense
- **Cross-Platform**: Windows, macOS, Linux support (38 target platforms)

### 🔄 **Reactive State Management**
- **Component State**: React-like state management
- **Event System**: Keyboard, mouse, and custom events
- **Async Support**: Promise-based APIs with async/await

## 🚀 Quick Start

### Installation

```bash
npm install reactive-tui
# or
yarn add reactive-tui
# or
bun add reactive-tui
```

### Basic Usage

```javascript
const { JsTuiApp, TuiUtils } = require('reactive-tui');

// Create a new TUI application
const app = new JsTuiApp();
app.setTitle('My TUI App');

// Load CSS styling
app.loadCss(`
  .container {
    background: #1e1e1e;
    color: #ffffff;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
  }
  
  .header {
    font-size: 1.5rem;
    font-weight: bold;
    margin-bottom: 1rem;
  }
  
  .button {
    background: #007acc;
    color: white;
    padding: 0.5rem 1rem;
    border: 1px solid #005a9e;
    margin: 0.25rem;
  }
`);

// Create UI components
const container = TuiUtils.div();
container.addClass('container');

const header = TuiUtils.div();
header.addClass('header');
header.setContent('🚀 Welcome to Reactive TUI');

const button = TuiUtils.button();
button.addClass('button');
button.setContent('Click Me!');
button.makeFocusable(0);

// Build component hierarchy
container.addChild(header);
container.addChild(button);

// Set root component and start
app.setComponent(container);
const status = app.start();
console.log(status); // "TUI Application initialized in headless mode"
```

### TypeScript Usage

```typescript
import { 
  JsTuiApp, 
  JsElement, 
  TuiUtils, 
  JsToast,
  JsColorTheme 
} from 'reactive-tui';

async function createApp(): Promise<void> {
  const app = new JsTuiApp();
  app.setTitle('TypeScript TUI Demo');
  
  // Type-safe element creation
  const container: JsElement = TuiUtils.div();
  container.setId('app-root');
  container.addClass('main-container');
  
  // Toast notifications with full typing
  const toast = JsToast.success('TypeScript integration working!');
  toast.setTitle('Success');
  toast.setDuration(3000);
  
  // Theme system with type safety
  const theme = JsColorTheme.dark();
  console.log(`Using theme: ${theme.getName()}`);
  
  app.setComponent(container);
  const status: string = app.start();
  console.log(`App status: ${status}`);
}
```

## 📚 API Reference

### Core Classes

#### `JsTuiApp`
Main application class for creating TUI applications.

```javascript
const app = new JsTuiApp();
app.setTitle(title: string): void
app.loadCss(css: string): void
app.setComponent(element: JsElement): void
app.start(): string
```

#### `JsElement`
Represents a UI element with CSS styling and hierarchy support.

```javascript
const element = new JsElement(tag: string);
element.setId(id: string): void
element.addClass(className: string): void
element.setContent(content: string): void
element.addChild(child: JsElement): void
element.setAttribute(name: string, value: string): void
element.makeFocusable(tabIndex?: number): void
```

#### `TuiUtils`
Utility functions for creating common UI elements.

```javascript
TuiUtils.div(): JsElement
TuiUtils.button(): JsElement
TuiUtils.input(): JsElement
TuiUtils.text(content: string): JsElement
TuiUtils.getTerminalSize(): [number, number]
TuiUtils.validateCss(css: string): string[]
```

### Toast Notifications

```javascript
const { JsToast, JsToastManager } = require('reactive-tui');

// Create different toast types
const infoToast = JsToast.info('Information message');
const successToast = JsToast.success('Success message');
const warningToast = JsToast.warning('Warning message');
const errorToast = JsToast.error('Error message');

// Configure toasts
toast.setTitle('Custom Title');
toast.setDuration(5000); // 5 seconds

// Manage toasts
const [width, height] = TuiUtils.getTerminalSize();
const manager = new JsToastManager(width, height);
manager.showToast(toast);
```

### Color System

```javascript
const { JsColorDefinition, JsColorTheme } = require('reactive-tui');

// Create colors
const rgbColor = JsColorDefinition.rgb(255, 0, 128);
const hexColor = JsColorDefinition.hex('#ff0080');

// Get color values
const [r, g, b] = rgbColor.getRgb(); // [255, 0, 128]
const ansiCode = rgbColor.toAnsi(false); // ANSI escape sequence

// Use predefined themes
const darkTheme = JsColorTheme.dark();
const lightTheme = JsColorTheme.light();
const terminalTheme = JsColorTheme.terminal();

// Theme information
console.log(darkTheme.getName()); // "dark"
console.log(darkTheme.getDescription()); // Theme description
const themeJson = darkTheme.toJson(); // Serialize to JSON
```

### Reactive State Management

```javascript
const { JsReactiveState } = require('reactive-tui');

const state = new JsReactiveState();

// Manage state as JSON
const appState = { count: 0, name: 'MyApp', active: true };
state.setStateJson(JSON.stringify(appState));

// Retrieve state
const currentState = JSON.parse(state.getStateJson());
console.log(currentState); // { count: 0, name: 'MyApp', active: true }
```

## 🎨 CSS Styling

Reactive TUI supports a comprehensive CSS subset optimized for terminal interfaces:

### Layout Properties
```css
.container {
  display: flex;
  flex-direction: column;
  justify-content: center;
  align-items: stretch;
  gap: 1rem;
}

.grid {
  display: grid;
  grid-template-columns: 1fr 2fr 1fr;
  grid-gap: 0.5rem;
}
```

### Visual Properties
```css
.styled-element {
  background: #1e1e1e;
  color: #ffffff;
  border: 1px solid #333;
  padding: 1rem;
  margin: 0.5rem;
  border-radius: 4px;
}
```

### Responsive Design
```css
@media (max-width: 80) {
  .responsive {
    flex-direction: column;
  }
}

@media (min-width: 120) {
  .wide-layout {
    grid-template-columns: repeat(4, 1fr);
  }
}
```

### Utility Classes
```css
/* Spacing */
.p-4 { padding: 1rem; }
.m-2 { margin: 0.5rem; }

/* Colors */
.bg-primary { background: #007acc; }
.text-white { color: #ffffff; }

/* Layout */
.flex { display: flex; }
.grid { display: grid; }
.justify-center { justify-content: center; }
```

## 🔧 Advanced Usage

### Custom Themes

```javascript
const customTheme = {
  name: "custom",
  description: "My Custom Theme",
  palette: {
    primary: { r: 0, g: 122, b: 204 },
    secondary: { r: 108, g: 117, b: 125 },
    background: { r: 30, g: 30, b: 30 },
    text: { r: 255, g: 255, b: 255 }
  }
};

const theme = JsColorTheme.fromJson(JSON.stringify(customTheme));
```

### Complex Layouts

```javascript
// Create a dashboard layout
const dashboard = TuiUtils.div();
dashboard.addClass('dashboard');

const sidebar = TuiUtils.div();
sidebar.addClass('sidebar');

const mainContent = TuiUtils.div();
mainContent.addClass('main-content');

const header = TuiUtils.div();
header.addClass('header');

dashboard.addChild(header);
dashboard.addChild(sidebar);
dashboard.addChild(mainContent);

// Apply CSS
app.loadCss(`
  .dashboard {
    display: grid;
    grid-template-areas: 
      "header header"
      "sidebar main";
    grid-template-rows: auto 1fr;
    grid-template-columns: 200px 1fr;
    height: 100vh;
  }
  
  .header { grid-area: header; }
  .sidebar { grid-area: sidebar; }
  .main-content { grid-area: main; }
`);
```

### Performance Optimization

```javascript
// Efficient batch operations
const elements = [];
for (let i = 0; i < 1000; i++) {
  const item = TuiUtils.div();
  item.setId(`item-${i}`);
  item.setContent(`Item ${i}`);
  elements.push(item);
}

// Add all at once
elements.forEach(item => container.addChild(item));
```

## 🧪 Testing

Reactive TUI includes comprehensive test coverage:

```bash
# Run all tests
npm test

# Run specific test suites
npx ava __test__/comprehensive.spec.ts
npx ava __test__/performance.spec.ts

# Run with verbose output
npm test -- --verbose
```

### Test Results
- ✅ **24 comprehensive functionality tests**
- ✅ **8 performance benchmarks**
- ✅ **Performance**: 311ms for 1000 operations (0.3ms per operation)
- ✅ **Memory management validation**
- ✅ **Error handling coverage**

## 🚀 Performance

Reactive TUI is built for performance:

- **App Creation**: ~6ms for 100 apps
- **Element Operations**: ~5ms for 1000 elements with full styling
- **CSS Processing**: ~5ms for large stylesheets (1000 rules)
- **Color Operations**: ~15ms for 10,000 color conversions
- **Memory Efficient**: Proper garbage collection, minimal memory leaks

## 🛠️ Development

### Building from Source

```bash
# Clone the repository
git clone https://github.com/entrepeneur4lyf/reactive-tui.git
cd reactive-tui

# Install dependencies
npm install

# Build the native module
npm run build:debug  # Development build
npm run build        # Production build

# Run examples
cargo run --example button_demo
cargo run --example layout_showcase

# Run tests
npm test
cargo test
```

### Rust Examples

The repository includes 37+ Rust examples demonstrating various features:

```bash
cargo run --example datatable_demo    # Data table with sorting
cargo run --example animation_demo    # Property animations
cargo run --example theme_system_demo # Theming system
cargo run --example plugin_demo       # Plugin architecture
cargo run --example modal_demo        # Modal dialogs
```

## 📋 Requirements

### Runtime Requirements
- **Node.js**: >= 12.22.0 (LTS recommended)
- **Platforms**: Windows, macOS, Linux (38 target platforms supported)
- **Terminal**: Any modern terminal with ANSI color support

### Development Requirements
- **Rust**: >= 1.70.0
- **Node.js**: >= 16.0.0
- **Cargo**: Latest stable

## 🤝 Contributing

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

### Development Setup

1. Fork the repository
2. Create a feature branch: `git checkout -b feature/amazing-feature`
3. Make your changes and add tests
4. Ensure all tests pass: `npm test && cargo test`
5. Commit with conventional commits: `git commit -m "feat: add amazing feature"`
6. Push to your fork and submit a pull request

## 📜 License

This project is dual-licensed under your choice of:

- **Apache License 2.0** - see the [LICENSE](LICENSE) file for open source use
- **Commercial License** - see the [LICENSE-COMMERCIAL](LICENSE-COMMERCIAL) file for commercial use with additional rights and restrictions

### Open Source License (Apache 2.0)
For open source projects, small businesses (under $1M revenue), and non-commercial use, this project is available under the Apache 2.0 license. This allows you to freely use, modify, and distribute the software with attribution.

### Commercial License
For large enterprises ($1M+ revenue) and organizations requiring additional rights or support, a commercial license is available. The commercial license includes:

- **Framework-only restrictions**: You cannot create competing TUI frameworks (building apps is encouraged!)
- **Enterprise support**: Priority technical support and consulting  
- **Commercial rights**: Use in proprietary applications without attribution requirements
- **Indemnification**: Legal protection for enterprise deployments

### Licensing Summary
- 🆓 **Small companies** (< $1M revenue): Free under Apache 2.0
- 💼 **Large enterprises** ($1M+ revenue): Commercial license required
- 🚫 **Framework competitors**: Commercial license required regardless of size

Contact [sales@reactive-tui.dev](mailto:sales@reactive-tui.dev) for commercial licensing inquiries.

## 🙏 Acknowledgments

- Built with [napi-rs](https://napi.rs/) for seamless Rust-Node.js integration
- Inspired by modern web frameworks and terminal UI libraries
- CSS layout powered by [Taffy](https://github.com/DioxusLabs/taffy)
- Terminal handling via [Crossterm](https://github.com/crossterm-rs/crossterm)

## 📞 Support

- 📖 [Documentation](https://github.com/entrepeneur4lyf/reactive-tui/wiki)
- 🐛 [Issue Tracker](https://github.com/entrepeneur4lyf/reactive-tui/issues)
- 💬 [Discussions](https://github.com/entrepeneur4lyf/reactive-tui/discussions)
- 📧 [Email Support](mailto:support@reactive-tui.dev)

---

**Made with ❤️ by the Reactive TUI team**
