---
sidebar_position: 1
---

# API Reference Overview

Complete reference for all Reactive TUI classes and functions.

## Core Classes

### JsTuiApp

Main application class for creating TUI applications.

```typescript
class JsTuiApp {
  constructor()
  setTitle(title: string): void
  loadCss(css: string): void
  loadStylesheet(path: string): void
  setComponent(element: JsElement): void
  start(): string
  sendMessage(message: string): void
}
```

**Example:**
```javascript
const app = new JsTuiApp();
app.setTitle('My TUI App');
app.loadCss('.container { background: #1e1e1e; }');
app.setComponent(container);
const status = app.start();
```

### JsElement

Represents a UI element with CSS styling and hierarchy support.

```typescript
class JsElement {
  constructor(tag: string)
  setId(id: string): void
  addClass(className: string): void
  setContent(content: string): void
  addChild(child: JsElement): void
  setAttribute(name: string, value: string): void
  makeFocusable(tabIndex?: number): void
}
```

**Example:**
```javascript
const element = new JsElement('div');
element.setId('my-element');
element.addClass('container');
element.setContent('Hello World');
element.makeFocusable(0);
```

### TuiUtils

Utility functions for creating common UI elements.

```typescript
class TuiUtils {
  static div(): JsElement
  static button(): JsElement
  static input(): JsElement
  static text(content: string): JsElement
  static getTerminalSize(): [number, number]
  static validateCss(css: string): string[]
}
```

**Example:**
```javascript
const container = TuiUtils.div();
const button = TuiUtils.button();
const [width, height] = TuiUtils.getTerminalSize();
const errors = TuiUtils.validateCss('.test { color: red; }');
```

## Toast System

### JsToast

Toast notification system for user feedback.

```typescript
class JsToast {
  static info(message: string): JsToast
  static success(message: string): JsToast
  static warning(message: string): JsToast
  static error(message: string): JsToast
  setTitle(title: string): void
  setDuration(durationMs: number): void
}
```

**Example:**
```javascript
const toast = JsToast.success('Operation completed!');
toast.setTitle('Success');
toast.setDuration(3000);
```

### JsToastManager

Manages multiple toast notifications.

```typescript
class JsToastManager {
  constructor(viewportWidth: number, viewportHeight: number)
  showToast(toast: JsToast): void
  dismissToast(toastId: string): boolean
  cleanupExpired(): string[]
}
```

**Example:**
```javascript
const [width, height] = TuiUtils.getTerminalSize();
const manager = new JsToastManager(width, height);
manager.showToast(toast);
const expiredIds = manager.cleanupExpired();
```

## Color System

### JsColorDefinition

Color creation and manipulation.

```typescript
class JsColorDefinition {
  static rgb(r: number, g: number, b: number): JsColorDefinition
  static hex(hexColor: string): JsColorDefinition
  getRgb(): [number, number, number]
  toAnsi(background: boolean): string
}
```

**Example:**
```javascript
const rgbColor = JsColorDefinition.rgb(255, 0, 128);
const hexColor = JsColorDefinition.hex('#ff0080');
const [r, g, b] = rgbColor.getRgb();
const ansiCode = rgbColor.toAnsi(false);
```

### JsColorTheme

Theme management with predefined and custom themes.

```typescript
class JsColorTheme {
  static dark(): JsColorTheme
  static light(): JsColorTheme
  static terminal(): JsColorTheme
  static fromJson(json: string): JsColorTheme
  toJson(): string
  getName(): string
  getDescription(): string
  getSemanticColor(semanticKey: string): string
  getSemanticBackground(semanticKey: string): string
}
```

**Example:**
```javascript
const darkTheme = JsColorTheme.dark();
const lightTheme = JsColorTheme.light();
const themeName = darkTheme.getName();
const themeJson = darkTheme.toJson();
const customTheme = JsColorTheme.fromJson(themeJson);
```

## State Management

### JsReactiveState

Reactive state management for dynamic UIs.

```typescript
class JsReactiveState {
  constructor()
  getStateJson(): string
  setStateJson(json: string): void
}
```

**Example:**
```javascript
const state = new JsReactiveState();
const appState = { count: 0, user: 'John' };
state.setStateJson(JSON.stringify(appState));

const currentState = JSON.parse(state.getStateJson());
currentState.count++;
state.setStateJson(JSON.stringify(currentState));
```

## Actions and Events

### Actions

Predefined action constants for common operations.

```typescript
class Actions {
  get quit(): string
  get refresh(): string
  get focusNext(): string
  get focusPrevious(): string
  get activate(): string
  get scrollUp(): string
  get scrollDown(): string
  get copy(): string
  get paste(): string
  get save(): string
}
```

**Example:**
```javascript
console.log(Actions.quit);        // "quit"
console.log(Actions.refresh);     // "refresh"
console.log(Actions.focusNext);   // "focus_next"
```

## Metadata and Types

### EnhancedFfiTypes

Metadata access for TypeScript integration and tooling.

```typescript
class EnhancedFfiTypes {
  static semanticColorKeys(): string[]
  static colorPaletteKeys(): string[]
  static widgetTypes(): string[]
  static elementAttributes(): string[]
  static cssUtilityPrefixes(): string[]
}
```

**Example:**
```javascript
const colorKeys = EnhancedFfiTypes.semanticColorKeys();
const widgetTypes = EnhancedFfiTypes.widgetTypes();
const attributes = EnhancedFfiTypes.elementAttributes();
```

## Module Functions

### Package Information

```typescript
function getVersion(): string
function init(): void
function initTui(): void
```

**Example:**
```javascript
console.log(getVersion());  // "0.1.0"
initTui();  // Initialize the TUI library
```

## Error Handling

All methods that can fail will throw JavaScript errors with descriptive messages:

```javascript
try {
  const color = JsColorDefinition.hex('invalid-color');
} catch (error) {
  console.error('Invalid color format:', error.message);
}

try {
  const theme = JsColorTheme.fromJson('invalid json');
} catch (error) {
  console.error('Invalid theme JSON:', error.message);
}
```

## Performance Considerations

### Batch Operations

When creating many elements, batch operations for better performance:

```javascript
// Good: Create elements in batch
const elements = [];
for (let i = 0; i < 1000; i++) {
  const item = TuiUtils.div();
  item.setContent(`Item ${i}`);
  elements.push(item);
}

// Add all at once
elements.forEach(item => container.addChild(item));
```

### CSS Validation

Validate CSS before applying to catch errors early:

```javascript
const css = '.container { color: red; background: blue; }';
const errors = TuiUtils.validateCss(css);
if (errors.length === 0) {
  app.loadCss(css);
} else {
  console.error('CSS validation errors:', errors);
}
```

### Memory Management

Objects are automatically garbage collected, but you can help by:

- Removing event listeners when components unmount
- Not keeping unnecessary references to large data structures
- Using the cleanup methods provided by managers

## Next Steps

- 📚 [Back to Getting Started](../intro)
- 🏗️ [Follow the Tutorial](../tutorial-basics/create-a-document)
- 💡 [Learn Core Concepts](../core-concepts)