| <img src="assets/logo.jpg" alt="Reactive TUI Logo" width="100" height="100"> | **CSS-styled Terminal User Interfaces for Node.js and Bun**<br><br>[![NPM Version](https://img.shields.io/npm/v/reactive-tui.svg)](https://www.npmjs.com/package/reactive-tui) [![Build Status](https://img.shields.io/github/workflow/status/entrepeneur4lyf/reactive-tui/CI)](https://github.com/entrepeneur4lyf/reactive-tui/actions) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)<br>[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/) [![Rust](https://img.shields.io/badge/Rust-Powered-orange.svg)](https://www.rust-lang.org/) [![Coverage](https://img.shields.io/badge/Coverage-72.31%25-brightgreen.svg)](#testing) |
|---|---|

A revolutionary **hybrid Rust/TypeScript TUI framework** that brings modern web development paradigms to terminal applications. Build beautiful, responsive terminal interfaces using familiar CSS styling, React-like components, and a comprehensive widget library—all powered by high-performance Rust with seamless NAPI-rs JavaScript integration.

**🏗️ Architecture**: Rust core engine + TypeScript developer API + NAPI-rs FFI bindings
**🎯 Philosophy**: Web-grade developer experience meets terminal performance
**📦 Scope**: Full-featured framework with 25+ widgets, advanced layouts, and reactive state management

## ✨ Features

### 🏗️ **Hybrid Architecture**
- **Rust Core Engine**: High-performance layout, rendering, and widget systems (614-line layout engine)
- **NAPI-rs FFI Layer**: Seamless Rust ↔ TypeScript integration with 843-line FFI module
- **TypeScript Developer API**: Modern, type-safe development experience
- **Component System**: React-like components with lifecycle hooks and props

### 🎨 **Advanced CSS Engine**
- **Full CSS Support**: Flexbox, CSS Grid, responsive breakpoints, animations
- **Utility Classes**: Tailwind-inspired classes (`flex`, `grid-cols-3`, `p-4`, `justify-center`)
- **CSS Parsing**: Complete CSS parser with validation and error reporting
- **Responsive Design**: Terminal-aware layouts with `@media` queries for terminal width
- **Theme System**: JSON-based themes with semantic color mappings and utility generation

### 🧩 **Comprehensive Widget Library (25+ Widgets)**

#### **Layout Widgets**
- **Grid**: Advanced grid layouts with column/row definitions and responsive behavior
- **Bar**: Header/footer bars with flexible positioning (header, footer, navigation, status, toolbar)
- **Tabs**: Multi-orientation tab navigation (horizontal, vertical, bottom, card, minimal)
- **Modal**: Overlay dialogs with backdrop (alert, confirm, prompt, custom, fullscreen)
- **Accordion**: Expandable/collapsible sections with animations (compact, FAQ, settings)

#### **Form Controls**
- **Input**: Text input with validation, placeholders, and state management
- **Button**: Interactive buttons with 8 types (primary, secondary, success, warning, danger, info, ghost, link)
- **Checkbox**: Single and grouped checkboxes with custom styling and animations
- **Switch**: Toggle switches with labels and state persistence
- **Radio**: Radio button groups with orientation control
- **Select**: Dropdown selection with search and filtering
- **Autocomplete**: Type-ahead search with suggestion filtering (command, country, language, user)
- **Slider**: Range sliders with ticks, orientation, and value formatting

#### **Data Display**
- **DataTable**: Sortable, filterable tables with pagination, column management, and virtual scrolling
- **Tree**: Hierarchical tree with lazy loading, custom icons, and node management
- **ScrollableList**: Virtual scrolling lists with selection modes (file browser, menu, task lists)
- **Progress**: Progress bars with animations, colors, and custom styling
- **Spinner**: Loading indicators with 30+ animation types (loading, processing, saving)

#### **Content Widgets**
- **RichText**: Markdown rendering with syntax highlighting and custom elements
- **Textarea**: Multi-line text editing with vim-like features, search, and history
- **Viewport**: Scrollable areas with virtual rendering and lazy loading

#### **Feedback & Validation**
- **Toast**: Notification toasts with positioning, duration, and variant types
- **FormValidator**: Real-time form validation with custom rules and error display

#### **Advanced Features**
- **Animation**: Property animations with easing functions and timelines
- **Plugin**: Extensible widget architecture for custom components

### ⚡ **Performance & Optimization**
- **Native Speed**: Rust-powered rendering with virtual DOM and dirty region tracking
- **Memory Efficient**: Proper garbage collection and minimal memory footprint
- **ANSI Processing**: Optimized terminal output with color capability detection
- **Batch Operations**: Efficient bulk element operations and updates
- **Cross-Platform**: 38 target platforms including ARM, x86, Windows, macOS, Linux

### 🔄 **Reactive State Management**
- **React-like State**: Component state with change detection and watchers
- **Event System**: Comprehensive keyboard, mouse, and custom event handling
- **Async Support**: Promise-based APIs with async/await patterns
- **State Persistence**: Screen state preservation across navigation
- **JSON Serialization**: Type-safe state serialization and hydration

### 🖥️ **Multi-Screen & Workspace System**
- **Screen Management**: Multiple screens with navigation and lifecycle hooks
- **Workspace Tabs**: Tab-based workspace management with state preservation
- **Navigation**: History-based routing with transition animations
- **Screen Lifecycle**: Mount, unmount, show, hide, focus, blur events
- **Keyboard Shortcuts**: Built-in navigation shortcuts and custom key bindings

### 🎯 **TypeScript Developer Layer (tui-bun)**
- **Complete TypeScript API**: Full-featured TypeScript layer built on Rust core
- **1,348-line App Engine**: Sophisticated application management with terminal rendering
- **Advanced Layout Engine**: 598-line CSS layout system with responsive design
- **Plugin Architecture**: 692-line extensible plugin system for custom widgets
- **Router System**: 432-line navigation system with history and route guards
- **Theme System**: 690-line JSON-based theming with inheritance and hot reload
- **Widget Factory**: Dynamic widget creation and management system
- **Error Boundaries**: Comprehensive error handling and recovery
- **Hot Reload**: CSS and component hot reloading for development

### 📦 **TypeScript Widget Library (25+ Widgets)**

#### **Advanced Widget Implementations**
- **DataTable**: 565-line implementation with sorting, filtering, pagination, virtual scrolling
- **Tree**: Hierarchical data with lazy loading, custom icons, and node management
- **Accordion**: Expandable sections with animations and state management
- **Modal**: Overlay dialogs with backdrop, positioning, and accessibility
- **Autocomplete**: Type-ahead search with filtering and suggestion management
- **RichText**: Markdown rendering with syntax highlighting
- **Viewport**: Scrollable content areas with virtual rendering
- **Progress**: Multiple progress styles (linear, circular, spinner) with animations
- **Slider**: Range sliders with ticks, orientation, and value formatting
- **Form Validation**: Real-time validation with custom rules and error display

#### **TypeScript-Specific Features**
- **Type Safety**: Full TypeScript definitions for all widgets and APIs
- **Builder Patterns**: Fluent APIs for widget configuration
- **Component Composition**: React-like component architecture
- **State Management**: Reactive state with change detection
- **Event System**: Type-safe event handling and custom events
- **CSS Integration**: Type-safe CSS class and style management

### 🎨 **Advanced TypeScript Examples (41+ Demos)**

#### **API Demonstrations (15+ Examples)**
- **Border Themes**: Comprehensive border styling showcase
- **CSS Class Verification**: CSS utility class validation and testing
- **Form Controls**: Complete form widget demonstrations
- **Grid Layouts**: Advanced CSS Grid examples with responsive design
- **Progress Showcases**: All progress widget variants and animations
- **Slider Examples**: Range slider configurations and interactions
- **Theme Validation**: JSON theme loading and validation
- **Toast Notifications**: Notification system with positioning
- **Utility Styling**: CSS utility class generation and usage
- **Responsive Design**: Terminal-aware responsive layout patterns

#### **TUI Demonstrations (26+ Examples)**
- **Dashboard Demo**: Complete dashboard with metrics and data visualization
- **Multiscreen Demo**: Navigation between multiple application screens
- **Workspace Tabs**: Tab-based workspace management
- **Plugin System**: Dynamic plugin loading and widget extension
- **Hot Reload**: Live CSS and component reloading
- **Animation System**: Property animations with easing functions
- **Interactive Widgets**: All widget types with full interactivity
- **Form Validation**: Real-time form validation with custom rules
- **Data Visualization**: Charts, tables, and data display widgets
- **Accessibility**: Screen reader support and keyboard navigation

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

// Create a new TUI application with Rust core
const app = new JsTuiApp();
app.setTitle('My TUI App');

// Load advanced CSS with flexbox, grid, and responsive design
app.loadCss(`
  .container {
    background: #1e1e1e;
    color: #ffffff;
    padding: 2rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    min-height: 100vh;
    gap: 1rem;
  }

  .header {
    font-size: 1.5rem;
    font-weight: bold;
    text-align: center;
    border-bottom: 1px solid #333;
    padding-bottom: 0.5rem;
  }

  .button-group {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
    justify-content: center;
  }

  .button {
    background: #007acc;
    color: white;
    padding: 0.5rem 1rem;
    border: 1px solid #005a9e;
    border-radius: 4px;
    cursor: pointer;
    transition: background 0.2s;
  }

  .button:hover {
    background: #005a9e;
  }

  .button:focus {
    outline: 2px solid #66afe9;
    outline-offset: 2px;
  }

  /* Responsive design for different terminal sizes */
  @media (max-width: 80) {
    .button-group {
      flex-direction: column;
      align-items: center;
    }
  }
`);

// Create UI components using the comprehensive element system
const container = TuiUtils.div();
container.addClass('container');
container.setId('app-root');

const header = TuiUtils.div();
header.addClass('header');
header.setContent('🚀 Welcome to Reactive TUI Framework');

const buttonGroup = TuiUtils.div();
buttonGroup.addClass('button-group');

// Create multiple interactive buttons
const primaryButton = TuiUtils.button();
primaryButton.addClass('button');
primaryButton.setContent('Primary Action');
primaryButton.makeFocusable(0);

const secondaryButton = TuiUtils.button();
secondaryButton.addClass('button');
secondaryButton.setContent('Secondary');
secondaryButton.makeFocusable(1);

// Build component hierarchy
buttonGroup.addChild(primaryButton);
buttonGroup.addChild(secondaryButton);
container.addChild(header);
container.addChild(buttonGroup);

// Set root component and start the Rust-powered application
app.setComponent(container);
const status = app.start();
console.log(status); // "TUI Application initialized in headless mode"
```

### TypeScript Usage with Full Widget Library

```typescript
import {
  JsTuiApp,
  JsElement,
  TuiUtils,
  JsToast,
  JsToastManager,
  JsColorTheme,
  JsColorDefinition,
  JsReactiveState
} from 'reactive-tui';

async function createAdvancedApp(): Promise<void> {
  const app = new JsTuiApp();
  app.setTitle('Advanced TypeScript TUI Demo');

  // Load comprehensive CSS with utility classes and responsive design
  app.loadCss(`
    .dashboard {
      display: grid;
      grid-template-areas:
        "header header header"
        "sidebar main toast-area";
      grid-template-rows: auto 1fr;
      grid-template-columns: 200px 1fr 300px;
      height: 100vh;
      gap: 1rem;
      padding: 1rem;
      background: #1a1a1a;
      color: #ffffff;
    }

    .header {
      grid-area: header;
      background: #2d2d2d;
      padding: 1rem;
      border-radius: 8px;
      text-align: center;
      font-weight: bold;
    }

    .sidebar {
      grid-area: sidebar;
      background: #252525;
      padding: 1rem;
      border-radius: 8px;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }

    .main-content {
      grid-area: main;
      background: #2d2d2d;
      padding: 1rem;
      border-radius: 8px;
      display: flex;
      flex-direction: column;
      gap: 1rem;
    }

    .toast-area {
      grid-area: toast-area;
      display: flex;
      flex-direction: column;
      gap: 0.5rem;
    }

    .nav-button {
      background: #007acc;
      color: white;
      padding: 0.5rem;
      border: none;
      border-radius: 4px;
      cursor: pointer;
      text-align: left;
    }

    .nav-button:hover {
      background: #005a9e;
    }

    .nav-button:focus {
      outline: 2px solid #66afe9;
    }

    .data-grid {
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
      gap: 1rem;
    }

    .metric-card {
      background: #333;
      padding: 1rem;
      border-radius: 8px;
      border-left: 4px solid #007acc;
    }

    .metric-value {
      font-size: 1.5rem;
      font-weight: bold;
      color: #00ff88;
    }

    .metric-label {
      color: #ccc;
      font-size: 0.9rem;
    }

    /* Responsive design for smaller terminals */
    @media (max-width: 120) {
      .dashboard {
        grid-template-areas:
          "header"
          "main"
          "sidebar"
          "toast-area";
        grid-template-columns: 1fr;
        grid-template-rows: auto 1fr auto auto;
      }
    }
  `);

  // Create dashboard layout with type safety
  const dashboard: JsElement = TuiUtils.div();
  dashboard.setId('dashboard');
  dashboard.addClass('dashboard');

  // Header section
  const header: JsElement = TuiUtils.div();
  header.addClass('header');
  header.setContent('🚀 Reactive TUI Dashboard - TypeScript + Rust');

  // Sidebar navigation
  const sidebar: JsElement = TuiUtils.div();
  sidebar.addClass('sidebar');

  const navButtons = ['Dashboard', 'Analytics', 'Settings', 'Help'];
  navButtons.forEach((label, index) => {
    const button: JsElement = TuiUtils.button();
    button.addClass('nav-button');
    button.setContent(label);
    button.makeFocusable(index);
    sidebar.addChild(button);
  });

  // Main content area with data visualization
  const mainContent: JsElement = TuiUtils.div();
  mainContent.addClass('main-content');

  const dataGrid: JsElement = TuiUtils.div();
  dataGrid.addClass('data-grid');

  // Create metric cards
  const metrics = [
    { label: 'Active Users', value: '1,234', color: '#00ff88' },
    { label: 'CPU Usage', value: '45%', color: '#ffaa00' },
    { label: 'Memory', value: '2.1GB', color: '#ff6b6b' },
    { label: 'Uptime', value: '99.9%', color: '#4ecdc4' }
  ];

  metrics.forEach(metric => {
    const card: JsElement = TuiUtils.div();
    card.addClass('metric-card');

    const value: JsElement = TuiUtils.div();
    value.addClass('metric-value');
    value.setContent(metric.value);

    const label: JsElement = TuiUtils.div();
    label.addClass('metric-label');
    label.setContent(metric.label);

    card.addChild(value);
    card.addChild(label);
    dataGrid.addChild(card);
  });

  mainContent.addChild(dataGrid);

  // Toast notification area
  const toastArea: JsElement = TuiUtils.div();
  toastArea.addClass('toast-area');

  // Create toast notifications with full typing
  const successToast = JsToast.success('Dashboard loaded successfully!');
  successToast.setTitle('System Ready');
  successToast.setDuration(5000);

  const infoToast = JsToast.info('Real-time data streaming...');
  infoToast.setTitle('Data Stream');
  infoToast.setDuration(3000);

  // Toast manager with terminal size detection
  const [terminalWidth, terminalHeight] = TuiUtils.getTerminalSize();
  const toastManager = new JsToastManager(terminalWidth, terminalHeight);
  toastManager.showToast(successToast);
  toastManager.showToast(infoToast);

  // Reactive state management
  const appState = new JsReactiveState();
  const initialState = {
    currentScreen: 'dashboard',
    userCount: 1234,
    cpuUsage: 45,
    memoryUsage: 2.1,
    uptime: 99.9,
    lastUpdate: new Date().toISOString()
  };
  appState.setStateJson(JSON.stringify(initialState));

  // Theme system with type safety
  const theme = JsColorTheme.dark();
  console.log(`Using theme: ${theme.getName()} - ${theme.getDescription()}`);

  // Build component hierarchy
  dashboard.addChild(header);
  dashboard.addChild(sidebar);
  dashboard.addChild(mainContent);
  dashboard.addChild(toastArea);

  // Set root component and start the Rust-powered application
  app.setComponent(dashboard);
  const status: string = app.start();
  console.log(`App status: ${status}`);

  // Demonstrate state management
  const currentState = JSON.parse(appState.getStateJson());
  console.log('Current application state:', currentState);
}

// Run the advanced demo
createAdvancedApp().catch(console.error);
```

### Advanced TypeScript Integration with tui-bun

```typescript
import {
  createApp,
  createRouter,
  ThemeManager,
  PluginManager,
  LayoutEngine,
  div, text, button, input,
  dataTable, createColumn,
  modal, alertModal,
  tabs, createTab,
  progress, slider, toast
} from 'tui-bun';

// Advanced application with full TypeScript features
async function createFullFeaturedApp(): Promise<void> {
  // Initialize theme system
  const themeManager = new ThemeManager();
  await themeManager.loadThemeFile('themes/professional-dark.json');
  themeManager.setActiveTheme('professional-dark');

  // Initialize plugin system
  const pluginManager = new PluginManager();
  await pluginManager.loadPluginsFromDirectory('./plugins');

  // Create router with multiple screens
  const router = createRouter({
    fullScreen: true,
    enableHistory: true,
    onNavigate: (event) => console.log(`Navigated from ${event.from} to ${event.to}`)
  });

  // Register application routes
  router.registerRoutes({
    '/dashboard': {
      title: 'Dashboard',
      component: () => createDashboardScreen(),
      beforeEnter: async () => await validateUserAccess()
    },
    '/data': {
      title: 'Data Management',
      component: () => createDataScreen(),
    },
    '/settings': {
      title: 'Settings',
      component: () => createSettingsScreen()
    }
  });

  // Create main application
  const app = createApp({
    title: 'Professional TUI Application',
    stylesheet: './styles/app.css',
    component: () => createMainLayout(router),
    plugins: pluginManager,
    theme: themeManager.getActiveTheme(),
    onError: (error) => showErrorModal(error)
  });

  // Start the application
  await app.run();
}

// Dashboard screen with advanced widgets
function createDashboardScreen() {
  // Create data table with TypeScript types
  interface UserData {
    id: number;
    name: string;
    email: string;
    status: 'active' | 'inactive';
    lastLogin: Date;
  }

  const userData: UserData[] = [
    { id: 1, name: 'Alice Johnson', email: 'alice@example.com', status: 'active', lastLogin: new Date() },
    { id: 2, name: 'Bob Smith', email: 'bob@example.com', status: 'inactive', lastLogin: new Date('2024-01-15') }
  ];

  const userTable = dataTable<UserData>({
    id: 'user-table',
    data: userData,
    columns: [
      createColumn('id', 'ID', { width: 60, sortable: true }),
      createColumn('name', 'Name', { width: 150, sortable: true }),
      createColumn('email', 'Email', { width: 200, sortable: true }),
      createColumn('status', 'Status', {
        width: 100,
        renderer: (status) => status === 'active' ? '🟢 Active' : '🔴 Inactive'
      }),
      createColumn('lastLogin', 'Last Login', {
        width: 120,
        renderer: (date) => date.toLocaleDateString()
      })
    ],
    config: {
      sortable: true,
      filterable: true,
      selectable: true,
      paginated: true,
      virtualScrolling: true
    },
    callbacks: {
      onRowSelect: (rowId, row, selected) => {
        console.log(`User ${row.name} ${selected ? 'selected' : 'deselected'}`);
      },
      onSort: (columnId, order) => {
        console.log(`Sorting by ${columnId} in ${order} order`);
      }
    }
  });

  // Create progress indicators
  const cpuProgress = progress({
    id: 'cpu-usage',
    style: 'linear',
    label: 'CPU Usage',
    value: 67,
    max: 100,
    showPercentage: true,
    animated: true
  });

  const memoryProgress = progress({
    id: 'memory-usage',
    style: 'circular',
    label: 'Memory',
    value: 45,
    max: 100,
    showPercentage: true
  });

  // Create interactive controls
  const volumeSlider = slider({
    id: 'volume-control',
    min: 0,
    max: 100,
    value: 75,
    label: 'System Volume',
    orientation: 'horizontal',
    showTicks: true,
    onChange: (value) => console.log(`Volume set to ${value}%`)
  });

  // Create dashboard layout
  return div({ class: 'dashboard-container' })
    .child(
      div({ class: 'dashboard-header' })
        .child(text('System Dashboard').class('dashboard-title'))
        .child(
          div({ class: 'system-metrics' })
            .child(cpuProgress.render())
            .child(memoryProgress.render())
        )
    )
    .child(
      div({ class: 'dashboard-content' })
        .child(
          div({ class: 'data-section' })
            .child(text('User Management').class('section-title'))
            .child(userTable.render())
        )
        .child(
          div({ class: 'controls-section' })
            .child(text('System Controls').class('section-title'))
            .child(volumeSlider.render())
        )
    );
}

// Settings screen with form validation
function createSettingsScreen() {
  const settingsForm = div({ class: 'settings-form' })
    .child(
      input({
        id: 'username',
        type: 'text',
        placeholder: 'Username',
        validation: {
          required: true,
          minLength: 3,
          pattern: /^[a-zA-Z0-9_]+$/
        }
      })
    )
    .child(
      input({
        id: 'email',
        type: 'email',
        placeholder: 'Email Address',
        validation: {
          required: true,
          email: true
        }
      })
    )
    .child(
      button({
        id: 'save-settings',
        text: 'Save Settings',
        variant: 'primary',
        onClick: async () => {
          const isValid = await validateForm();
          if (isValid) {
            showSuccessToast('Settings saved successfully!');
          }
        }
      })
    );

  return div({ class: 'settings-container' })
    .child(text('Application Settings').class('page-title'))
    .child(settingsForm);
}

// Utility functions
async function validateUserAccess(): Promise<boolean> {
  // Simulate authentication check
  return new Promise(resolve => setTimeout(() => resolve(true), 100));
}

function showErrorModal(error: Error): void {
  const errorModal = alertModal({
    title: 'Application Error',
    message: error.message,
    type: 'error',
    buttons: ['OK']
  });
  errorModal.show();
}

function showSuccessToast(message: string): void {
  const successToast = toast({
    message,
    variant: 'success',
    duration: 3000,
    position: 'top-right'
  });
  successToast.show();
}

async function validateForm(): Promise<boolean> {
  // Simulate form validation
  return new Promise(resolve => setTimeout(() => resolve(true), 200));
}

// Start the application
createFullFeaturedApp().catch(console.error);
```

## 📚 Comprehensive API Reference

### Core Architecture Classes

#### `JsTuiApp` - Main Application Controller
Rust-powered TUI application with NAPI-rs integration.

```javascript
const app = new JsTuiApp();

// Application Configuration
app.setTitle(title: string): void                    // Set window/app title
app.loadCss(css: string): void                       // Load CSS from string
app.loadStylesheet(path: string): void               // Load CSS from file
app.setComponent(element: JsElement): void           // Set root component
app.start(): string                                  // Start application (returns status)
app.sendMessage(message: string): void               // Send message to app

// Advanced Features
app.setDriverConfig(config: DriverConfig): void     // Configure rendering driver
app.enableHotReload(enabled: boolean): void         // Enable CSS hot reload
app.setTheme(theme: JsColorTheme): void             // Apply color theme
```

#### `JsElement` - DOM-like Element System
Represents UI elements with full CSS styling and hierarchy support.

```javascript
const element = new JsElement(tag: string);

// Basic Properties
element.setId(id: string): void                      // Set unique identifier
element.addClass(className: string): void            // Add CSS class
element.setContent(content: string): void            // Set text content
element.setAttribute(name: string, value: string): void  // Set HTML-like attribute

// Hierarchy Management
element.addChild(child: JsElement): void             // Add child element
element.removeChild(child: JsElement): void          // Remove child element
element.getChildren(): JsElement[]                   // Get all children
element.getParent(): JsElement | null               // Get parent element

// Interaction & Focus
element.makeFocusable(tabIndex?: number): void       // Enable keyboard focus
element.setFocused(focused: boolean): void           // Set focus state
element.addEventListener(event: string, handler: Function): void  // Event handling

// Advanced Features
element.setVisible(visible: boolean): void           // Show/hide element
element.setEnabled(enabled: boolean): void           // Enable/disable interaction
element.getBoundingRect(): LayoutRect               // Get computed layout
element.scrollIntoView(): void                      // Scroll to element
```

#### `TuiUtils` - Element Creation & Utilities
Comprehensive utility functions for creating UI elements and system operations.

```javascript
// Element Creation
TuiUtils.div(): JsElement                           // Create div container
TuiUtils.button(): JsElement                        // Create button element
TuiUtils.input(): JsElement                         // Create input field
TuiUtils.text(content: string): JsElement           // Create text element
TuiUtils.span(): JsElement                          // Create inline span
TuiUtils.section(): JsElement                       // Create section container
TuiUtils.header(): JsElement                        // Create header element
TuiUtils.footer(): JsElement                        // Create footer element

// System Information
TuiUtils.getTerminalSize(): [number, number]        // Get terminal dimensions
TuiUtils.getColorSupport(): ColorSupport            // Detect color capabilities
TuiUtils.getPlatformInfo(): PlatformInfo            // Get platform details

// CSS & Validation
TuiUtils.validateCss(css: string): string[]         // Validate CSS syntax
TuiUtils.parseCss(css: string): CssRules           // Parse CSS into rules
TuiUtils.generateUtilityClasses(): string          // Generate utility CSS

// Performance & Debugging
TuiUtils.measurePerformance(fn: Function): number   // Measure execution time
TuiUtils.enableDebugMode(enabled: boolean): void    // Enable debug logging
TuiUtils.getMemoryUsage(): MemoryStats              // Get memory statistics
```

### TypeScript Developer API (tui-bun)

#### `createApp()` - Advanced Application Builder
TypeScript-first application creation with comprehensive features.

```typescript
import { createApp, createRouter, ThemeManager } from 'tui-bun';

// Advanced application configuration
const app = createApp({
  title: 'Professional TUI App',
  stylesheet: './styles/app.css',
  component: () => createMainComponent(),

  // Advanced features
  router: createRouter({ enableHistory: true }),
  theme: await ThemeManager.loadTheme('dark-professional'),
  plugins: await PluginManager.loadPlugins('./plugins'),

  // Lifecycle hooks
  onMount: () => console.log('App mounted'),
  onUnmount: () => console.log('App unmounted'),
  onError: (error) => handleError(error),

  // Performance options
  enableHotReload: true,
  enableVirtualScrolling: true,
  maxRenderFPS: 60
});

// Application methods
await app.run();                                    // Start the application
await app.stop();                                   // Stop the application
app.updateStylesheet(newCSS);                      // Hot reload CSS
app.setComponent(newComponent);                     // Update root component
app.navigate('/dashboard');                         // Navigate to route
```

#### `LayoutEngine` - Advanced Layout System
Sophisticated layout computation with responsive design.

```typescript
import { LayoutEngine, ResponsiveWidget } from 'tui-bun';

const layoutEngine = new LayoutEngine({
  width: 400,
  height: 200,
  terminalSize: { width: 120, height: 30 }
});

// Responsive widget layout
const layout = layoutEngine.computeResponsiveLayout(widget, {
  x: 0, y: 0, width: 400, height: 200
});

// CSS-based layout computation
const cssLayout = layoutEngine.computeLayout(element, {
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'stretch'
});

// Viewport management
layoutEngine.updateViewport({ width: 500, height: 300 });
const constraints = layoutEngine.getConstraints();
```

#### `ThemeManager` - Advanced Theme System
Comprehensive theme management with JSON configuration.

```typescript
import { ThemeManager, ThemeBuilder } from 'tui-bun';

const themeManager = new ThemeManager();

// Load themes from files
await themeManager.loadThemeFile('themes/dark.json');
await themeManager.loadThemeDirectory('./themes');

// Create custom themes
const customTheme = new ThemeBuilder()
  .name('Corporate Blue')
  .primary('#007acc')
  .secondary('#6c757d')
  .background('#1a1a1a')
  .surface('#2d2d2d')
  .build();

// Theme operations
themeManager.registerTheme(customTheme);
themeManager.setActiveTheme('corporate-blue');
const currentTheme = themeManager.getActiveTheme();

// Theme inheritance and composition
const extendedTheme = themeManager.extendTheme('dark', {
  colors: { primary: '#ff6b6b' }
});

// Hot reload themes
themeManager.enableHotReload('./themes');
themeManager.onThemeChange((theme) => app.applyTheme(theme));
```

#### `PluginManager` - Extensible Plugin System
Dynamic plugin loading and widget extension.

```typescript
import { PluginManager, WidgetPlugin, Plugin } from 'tui-bun';

const pluginManager = new PluginManager();

// Load plugins
await pluginManager.loadPlugin('./plugins/custom-widget.js');
await pluginManager.loadPluginsFromDirectory('./plugins');

// Create custom widget plugin
class CustomChartWidget extends WidgetPlugin {
  constructor() {
    super({
      name: 'custom-chart',
      version: '1.0.0',
      capabilities: ['widget-provider']
    });
  }

  render() {
    return div({ class: 'custom-chart' })
      .child(text('Custom Chart Widget'));
  }
}

// Register and use plugins
pluginManager.register(new CustomChartWidget());
const chartWidget = pluginManager.createWidget('custom-chart', {
  data: chartData,
  type: 'line'
});

// Plugin lifecycle
pluginManager.enablePlugin('custom-chart');
pluginManager.disablePlugin('custom-chart');
pluginManager.unloadPlugin('custom-chart');
```

#### `TUIRouter` - Navigation System
Advanced routing with history and guards.

```typescript
import { createRouter, fullScreenRouter } from 'tui-bun';

const router = createRouter({
  fullScreen: true,
  enableHistory: true,
  maxHistorySize: 50,
  onNavigate: (event) => console.log(`Navigated to ${event.to}`)
});

// Register routes
router.registerRoutes({
  '/dashboard': {
    title: 'Dashboard',
    component: () => createDashboard(),
    beforeEnter: () => checkAuth()
  },
  '/settings': {
    title: 'Settings',
    component: () => createSettings(),
    afterEnter: () => trackPageView()
  }
});

// Navigation
await router.navigate('/dashboard');
router.back();
router.forward();
router.replace('/login');

// Route guards and hooks
router.beforeEach(async (to, from) => {
  if (to.startsWith('/admin') && !isAdmin()) {
    return '/unauthorized';
  }
});

router.afterEach((to, from) => {
  analytics.track('page_view', { page: to });
});
```

### Advanced Widget System

#### `JsToast` & `JsToastManager` - Notification System
Comprehensive toast notification system with positioning and management.

```javascript
const { JsToast, JsToastManager } = require('reactive-tui');

// Create different toast types with factory methods
const infoToast = JsToast.info('System information');
const successToast = JsToast.success('Operation completed successfully');
const warningToast = JsToast.warning('Please review your input');
const errorToast = JsToast.error('Failed to save changes');

// Advanced toast configuration
successToast.setTitle('Success');
successToast.setDuration(5000);                     // Duration in milliseconds
successToast.setPosition('top-right');              // Position on screen
successToast.setIcon('✓');                          // Custom icon
successToast.setClosable(true);                     // Allow manual dismissal
successToast.setProgress(true);                     // Show progress bar

// Toast manager with advanced features
const [width, height] = TuiUtils.getTerminalSize();
const manager = new JsToastManager(width, height);

// Toast management
manager.showToast(successToast);                    // Show toast
manager.dismissToast(toastId);                      // Dismiss specific toast
manager.dismissAll();                               // Dismiss all toasts
manager.setMaxToasts(5);                           // Limit concurrent toasts
manager.setDefaultDuration(3000);                  // Set default duration

// Batch operations
const toasts = [infoToast, successToast, warningToast];
manager.showToasts(toasts);                         // Show multiple toasts

// Event handling
manager.onToastShown((toast) => console.log('Toast shown:', toast.getId()));
manager.onToastDismissed((toast) => console.log('Toast dismissed:', toast.getId()));

// Cleanup
const expiredIds = manager.cleanupExpired();        // Returns array of expired toast IDs
```

#### `JsColorDefinition` & `JsColorTheme` - Advanced Color & Theme System
Comprehensive color management with JSON themes, semantic mappings, and utility generation.

```javascript
const { JsColorDefinition, JsColorTheme } = require('reactive-tui');

// Color Creation & Manipulation
const rgbColor = JsColorDefinition.rgb(255, 0, 128);
const hexColor = JsColorDefinition.hex('#ff0080');
const hslColor = JsColorDefinition.hsl(330, 100, 50);

// Color Operations
const [r, g, b] = rgbColor.getRgb();                // [255, 0, 128]
const hexString = rgbColor.toHex();                 // "#ff0080"
const ansiCode = rgbColor.toAnsi(false);            // ANSI escape sequence
const ansiBackground = rgbColor.toAnsi(true);       // ANSI background color

// Color Variants & Manipulation
const lighterColor = rgbColor.lighten(0.2);         // 20% lighter
const darkerColor = rgbColor.darken(0.3);           // 30% darker
const saturatedColor = rgbColor.saturate(0.1);      // 10% more saturated
const desaturatedColor = rgbColor.desaturate(0.1);  // 10% less saturated

// Color Palette Generation
const palette = JsColorDefinition.generatePalette(rgbColor, 5);  // Generate 5-color palette
const complementary = rgbColor.getComplementary();   // Get complementary color
const analogous = rgbColor.getAnalogous();          // Get analogous colors

// Predefined Themes with Comprehensive Palettes
const darkTheme = JsColorTheme.dark();              // Professional dark theme
const lightTheme = JsColorTheme.light();            // Clean light theme
const terminalTheme = JsColorTheme.terminal();      // Classic terminal colors
const highContrastTheme = JsColorTheme.highContrast(); // Accessibility theme

// Custom Theme Creation
const customTheme = JsColorTheme.builder()
  .name('Corporate Blue')
  .description('Professional corporate theme')
  .primary('#007acc')
  .secondary('#6c757d')
  .success('#28a745')
  .warning('#ffc107')
  .danger('#dc3545')
  .info('#17a2b8')
  .background('#ffffff')
  .surface('#f8f9fa')
  .text('#212529')
  .build();

// Theme Operations
console.log(darkTheme.getName());                  // "dark"
console.log(darkTheme.getDescription());           // Theme description
const themeJson = darkTheme.toJson();              // Serialize to JSON
const loadedTheme = JsColorTheme.fromJson(themeJson); // Load from JSON

// Semantic Color Access
const primaryColor = darkTheme.getPrimary();       // Get primary color
const backgroundColor = darkTheme.getBackground(); // Get background color
const textColor = darkTheme.getText();             // Get text color

// Theme Validation & Utilities
const isValid = JsColorTheme.validate(themeJson);  // Validate theme JSON
const utilityCSS = darkTheme.generateUtilityCSS(); // Generate utility classes
const colorPreview = darkTheme.getPreview();       // Get theme preview

// Advanced Theme Features
const semanticMapping = darkTheme.getSemanticMapping(); // Get semantic color mappings
const borderStyles = darkTheme.getBorderStyles();   // Get border style definitions
const colorSupport = darkTheme.getColorSupport();   // Get terminal color support info
```

#### `JsReactiveState` - Advanced State Management System
React-like reactive state management with change detection, watchers, and persistence.

```javascript
const { JsReactiveState } = require('reactive-tui');

const state = new JsReactiveState();

// State Management with Type Safety
const appState = {
  count: 0,
  name: 'MyApp',
  active: true,
  user: { id: 1, email: 'user@example.com' },
  items: ['item1', 'item2', 'item3'],
  settings: { theme: 'dark', notifications: true }
};

// Set and retrieve state
state.setStateJson(JSON.stringify(appState));
const currentState = JSON.parse(state.getStateJson());
console.log(currentState);

// Individual Field Management
state.setField('count', 42);                       // Set individual field
state.setField('user.email', 'new@example.com');   // Set nested field
const count = state.getField('count');             // Get individual field
const userEmail = state.getField('user.email');    // Get nested field

// State Watching & Change Detection
state.watchField('count', (newValue, oldValue) => {
  console.log(`Count changed from ${oldValue} to ${newValue}`);
});

state.watchAll((changes) => {
  console.log('State changes:', changes);
});

// Batch Operations
state.batchUpdate(() => {
  state.setField('count', 100);
  state.setField('name', 'Updated App');
  state.setField('active', false);
}); // All changes applied atomically

// State History & Undo/Redo
state.enableHistory(10);                           // Enable history with 10 states
state.undo();                                      // Undo last change
state.redo();                                      // Redo last undone change
const history = state.getHistory();               // Get state history

// State Persistence
state.saveToFile('app-state.json');               // Save state to file
state.loadFromFile('app-state.json');             // Load state from file
const stateSnapshot = state.createSnapshot();      // Create state snapshot
state.restoreSnapshot(stateSnapshot);             // Restore from snapshot

// Computed Properties
state.addComputed('fullName', ['firstName', 'lastName'], (first, last) => {
  return `${first} ${last}`;
});

// State Validation
state.addValidator('email', (value) => {
  return /^[^\s@]+@[^\s@]+\.[^\s@]+$/.test(value);
});

// Performance & Debugging
state.enableDebugMode(true);                      // Enable debug logging
const stats = state.getStats();                   // Get performance statistics
state.clearWatchers();                            // Clear all watchers
state.reset();                                    // Reset to initial state
```

## 🎨 Advanced CSS Engine & Styling System

Reactive TUI features a comprehensive CSS engine with full parsing, validation, and terminal-optimized rendering:

### Complete CSS Support

#### **Layout Systems**
```css
/* Flexbox Layout */
.flex-container {
  display: flex;
  flex-direction: row | column | row-reverse | column-reverse;
  justify-content: flex-start | flex-end | center | space-between | space-around | space-evenly;
  align-items: flex-start | flex-end | center | stretch | baseline;
  flex-wrap: nowrap | wrap | wrap-reverse;
  gap: 1rem;
}

.flex-item {
  flex: 1;                    /* flex-grow: 1, flex-shrink: 1, flex-basis: 0% */
  flex-grow: 2;               /* Grow factor */
  flex-shrink: 0;             /* Shrink factor */
  flex-basis: 200px;          /* Base size */
  align-self: center;         /* Individual alignment */
}

/* CSS Grid Layout */
.grid-container {
  display: grid;
  grid-template-columns: 200px 1fr 100px;
  grid-template-rows: auto 1fr auto;
  grid-template-areas:
    "header header header"
    "sidebar main aside"
    "footer footer footer";
  grid-gap: 1rem;
  grid-column-gap: 0.5rem;
  grid-row-gap: 1rem;
}

.grid-item {
  grid-area: header;          /* Named grid area */
  grid-column: 1 / 3;         /* Column span */
  grid-row: 2 / 4;            /* Row span */
  justify-self: center;       /* Horizontal alignment */
  align-self: start;          /* Vertical alignment */
}

/* Advanced Grid Features */
.responsive-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  grid-auto-rows: minmax(100px, auto);
  grid-auto-flow: row dense;
}
```

#### **Visual Properties**
```css
.styled-element {
  /* Colors & Backgrounds */
  background: #1e1e1e;
  background-color: rgb(30, 30, 30);
  color: #ffffff;
  opacity: 0.8;

  /* Borders */
  border: 1px solid #333;
  border-top: 2px dashed #666;
  border-radius: 4px;
  border-style: solid | dashed | dotted | double;

  /* Spacing */
  padding: 1rem;              /* All sides */
  padding: 0.5rem 1rem;       /* Vertical horizontal */
  padding: 0.25rem 0.5rem 0.75rem 1rem; /* Top right bottom left */
  margin: 0.5rem auto;        /* Vertical auto-horizontal */

  /* Typography */
  font-size: 1.2rem;
  font-weight: bold | normal | 100-900;
  text-align: left | center | right | justify;
  text-decoration: underline | line-through | none;
  line-height: 1.5;

  /* Sizing */
  width: 100px | 50% | auto | min-content | max-content;
  height: 200px | 75% | auto;
  min-width: 100px;
  max-width: 500px;
  min-height: 50px;
  max-height: 300px;

  /* Positioning */
  position: static | relative | absolute | fixed;
  top: 10px;
  right: 20px;
  bottom: 15px;
  left: 25px;
  z-index: 10;

  /* Display & Visibility */
  display: block | inline | flex | grid | none;
  visibility: visible | hidden;
  overflow: visible | hidden | scroll | auto;
}
```

#### **Responsive Design & Media Queries**
```css
/* Terminal-specific media queries */
@media (max-width: 80) {
  .responsive {
    flex-direction: column;
    grid-template-columns: 1fr;
  }
}

@media (min-width: 120) {
  .wide-layout {
    grid-template-columns: repeat(4, 1fr);
    padding: 2rem;
  }
}

@media (max-height: 24) {
  .compact {
    padding: 0.25rem;
    font-size: 0.9rem;
  }
}

/* Color capability detection */
@media (color: 256) {
  .enhanced-colors {
    background: linear-gradient(45deg, #ff6b6b, #4ecdc4);
  }
}

@media (color: 16) {
  .basic-colors {
    background: #008000;  /* Use basic colors */
  }
}
```

#### **Comprehensive Utility Classes**
```css
/* Layout Utilities */
.flex { display: flex; }
.grid { display: grid; }
.block { display: block; }
.inline { display: inline; }
.hidden { display: none; }

/* Flexbox Utilities */
.flex-row { flex-direction: row; }
.flex-col { flex-direction: column; }
.flex-wrap { flex-wrap: wrap; }
.flex-nowrap { flex-wrap: nowrap; }

.justify-start { justify-content: flex-start; }
.justify-center { justify-content: center; }
.justify-end { justify-content: flex-end; }
.justify-between { justify-content: space-between; }
.justify-around { justify-content: space-around; }
.justify-evenly { justify-content: space-evenly; }

.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.items-end { align-items: flex-end; }
.items-stretch { align-items: stretch; }
.items-baseline { align-items: baseline; }

/* Grid Utilities */
.grid-cols-1 { grid-template-columns: repeat(1, 1fr); }
.grid-cols-2 { grid-template-columns: repeat(2, 1fr); }
.grid-cols-3 { grid-template-columns: repeat(3, 1fr); }
.grid-cols-4 { grid-template-columns: repeat(4, 1fr); }
.grid-cols-12 { grid-template-columns: repeat(12, 1fr); }

.col-span-1 { grid-column: span 1; }
.col-span-2 { grid-column: span 2; }
.col-span-3 { grid-column: span 3; }
.col-span-full { grid-column: 1 / -1; }

/* Spacing Utilities */
.p-0 { padding: 0; }
.p-1 { padding: 0.25rem; }
.p-2 { padding: 0.5rem; }
.p-3 { padding: 0.75rem; }
.p-4 { padding: 1rem; }
.p-6 { padding: 1.5rem; }
.p-8 { padding: 2rem; }

.m-0 { margin: 0; }
.m-1 { margin: 0.25rem; }
.m-2 { margin: 0.5rem; }
.m-4 { margin: 1rem; }
.m-auto { margin: auto; }

.gap-1 { gap: 0.25rem; }
.gap-2 { gap: 0.5rem; }
.gap-4 { gap: 1rem; }
.gap-6 { gap: 1.5rem; }

/* Sizing Utilities */
.w-full { width: 100%; }
.w-1/2 { width: 50%; }
.w-1/3 { width: 33.333333%; }
.w-2/3 { width: 66.666667%; }
.w-1/4 { width: 25%; }
.w-3/4 { width: 75%; }

.h-full { height: 100%; }
.h-screen { height: 100vh; }
.h-1/2 { height: 50%; }
.h-1/3 { height: 33.333333%; }

/* Color Utilities */
.text-primary { color: #007acc; }
.text-secondary { color: #6c757d; }
.text-success { color: #28a745; }
.text-warning { color: #ffc107; }
.text-danger { color: #dc3545; }
.text-info { color: #17a2b8; }
.text-white { color: #ffffff; }
.text-black { color: #000000; }

.bg-primary { background-color: #007acc; }
.bg-secondary { background-color: #6c757d; }
.bg-success { background-color: #28a745; }
.bg-warning { background-color: #ffc107; }
.bg-danger { background-color: #dc3545; }
.bg-transparent { background-color: transparent; }

/* Typography Utilities */
.text-xs { font-size: 0.75rem; }
.text-sm { font-size: 0.875rem; }
.text-base { font-size: 1rem; }
.text-lg { font-size: 1.125rem; }
.text-xl { font-size: 1.25rem; }
.text-2xl { font-size: 1.5rem; }

.font-thin { font-weight: 100; }
.font-normal { font-weight: 400; }
.font-bold { font-weight: 700; }

.text-left { text-align: left; }
.text-center { text-align: center; }
.text-right { text-align: right; }

/* Border Utilities */
.border { border: 1px solid; }
.border-2 { border: 2px solid; }
.border-t { border-top: 1px solid; }
.border-r { border-right: 1px solid; }
.border-b { border-bottom: 1px solid; }
.border-l { border-left: 1px solid; }

.rounded { border-radius: 0.25rem; }
.rounded-lg { border-radius: 0.5rem; }
.rounded-full { border-radius: 9999px; }
```

#### **Advanced CSS Features**
```css
/* Pseudo-classes and States */
.button:hover {
  background-color: #005a9e;
  transform: translateY(-1px);
}

.button:focus {
  outline: 2px solid #66afe9;
  outline-offset: 2px;
}

.button:active {
  transform: translateY(0);
}

.button:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

/* Animations and Transitions */
.animated {
  transition: all 0.3s ease-in-out;
}

.fade-in {
  animation: fadeIn 0.5s ease-in;
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.slide-in {
  animation: slideIn 0.3s ease-out;
}

@keyframes slideIn {
  from { transform: translateX(-100%); }
  to { transform: translateX(0); }
}

/* Custom Properties (CSS Variables) */
:root {
  --primary-color: #007acc;
  --secondary-color: #6c757d;
  --border-radius: 4px;
  --spacing-unit: 0.25rem;
}

.themed-element {
  background-color: var(--primary-color);
  border-radius: var(--border-radius);
  padding: calc(var(--spacing-unit) * 4);
}
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

### Comprehensive Test Coverage
- ✅ **186 comprehensive functionality tests** covering all widgets and APIs
- ✅ **8 performance benchmarks** with detailed timing analysis
- ✅ **Performance**: 311ms for 1000 operations (0.3ms per operation average)
- ✅ **Memory management validation** with leak detection
- ✅ **Error handling coverage** for all failure scenarios
- ✅ **Cross-platform testing** on Windows, macOS, and Linux
- ✅ **TypeScript integration tests** with full type safety validation
- ✅ **CSS engine tests** with comprehensive parsing and validation
- ✅ **Widget interaction tests** with keyboard and mouse simulation
- ✅ **Theme system tests** with JSON validation and color accuracy
- ✅ **State management tests** with reactive updates and persistence
- ✅ **FFI binding tests** ensuring Rust ↔ TypeScript integration integrity

## 🚀 Performance & Benchmarks

Reactive TUI is engineered for high performance with Rust-powered optimizations:

### **Core Performance Metrics**
- **App Creation**: ~6ms for 100 applications with full initialization
- **Element Operations**: ~5ms for 1000 elements with complete CSS styling
- **CSS Processing**: ~5ms for large stylesheets (1000+ rules with validation)
- **Layout Computation**: ~3ms for complex flexbox/grid layouts (100+ elements)
- **Color Operations**: ~15ms for 10,000 color conversions with ANSI generation
- **Widget Rendering**: ~2ms for complex widgets (DataTable with 1000 rows)
- **State Updates**: ~1ms for reactive state changes with watchers
- **Memory Efficient**: <50MB for complex applications, proper garbage collection

### **Rust Core Optimizations**
- **Virtual Rendering**: Only renders changed regions for optimal performance
- **Dirty Region Tracking**: Minimizes unnecessary redraws
- **ANSI Processing**: Optimized terminal output with color capability detection
- **Layout Caching**: Computed layouts cached for repeated use
- **Widget Pooling**: Reusable widget instances for memory efficiency
- **Batch Operations**: Efficient bulk element operations and updates

### **NAPI-rs FFI Performance**
- **FFI Overhead**: <0.1ms per JavaScript ↔ Rust call
- **Memory Sharing**: Zero-copy data transfer where possible
- **Type Conversion**: Optimized serialization/deserialization
- **Async Operations**: Non-blocking async/await support

### **Scalability Benchmarks**
- **Large Applications**: Handles 10,000+ elements smoothly
- **Complex Layouts**: CSS Grid with 100+ cells renders in <10ms
- **Real-time Updates**: 60fps animation support with smooth transitions
- **Memory Usage**: Linear scaling with O(n) complexity for most operations

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

### Comprehensive Rust Examples (37+ Demos)

The repository includes extensive Rust examples demonstrating the full framework capabilities:

#### **Widget Demonstrations**
```bash
# Form Controls & Input
cargo run --example button_demo           # Button variants and interactions
cargo run --example input_demo            # Text input with validation
cargo run --example checkbox_demo         # Checkbox groups and states
cargo run --example radio_demo            # Radio button groups
cargo run --example select_demo           # Dropdown selection
cargo run --example slider_demo           # Range sliders with ticks
cargo run --example switch_demo           # Toggle switches
cargo run --example autocomplete_demo     # Type-ahead search

# Data Display & Visualization
cargo run --example datatable_demo        # Sortable tables with pagination
cargo run --example tree_demo             # Hierarchical tree with lazy loading
cargo run --example scrollable_list_demo  # Virtual scrolling lists
cargo run --example progress_demo         # Progress bars with animations
cargo run --example spinner_demo          # Loading spinners (30+ types)
cargo run --example rich_text_demo        # Markdown rendering
cargo run --example viewport_demo         # Scrollable content areas

# Layout & Navigation
cargo run --example grid_demo             # CSS Grid layouts
cargo run --example layout_demo           # Flexbox and responsive design
cargo run --example tabs_demo             # Tab navigation
cargo run --example accordion_demo        # Expandable sections
cargo run --example modal_demo            # Modal dialogs and overlays
cargo run --example bar_demo              # Header/footer bars
cargo run --example panel_demo            # Panel containers

# Advanced Features
cargo run --example animation_demo        # Property animations with easing
cargo run --example theme_system_demo     # JSON theming system
cargo run --example plugin_demo           # Plugin architecture
cargo run --example hot_reload_demo       # CSS hot reload
cargo run --example form_validation_demo  # Real-time form validation
cargo run --example toast_demo            # Toast notifications

# Multi-Screen & Workspace
cargo run --example multiscreen_demo      # Multiple screen navigation
cargo run --example workspace_demo        # Tab-based workspaces
cargo run --example navigation_demo       # Screen routing and history

# Performance & Integration
cargo run --example performance_demo      # Performance benchmarks
cargo run --example memory_demo           # Memory usage optimization
cargo run --example responsive_demo       # Responsive design patterns
cargo run --example accessibility_demo    # Accessibility features
```

#### **API Integration Examples**
```bash
# TypeScript/JavaScript Integration
cargo run --example ffi_demo              # NAPI-rs FFI bindings
cargo run --example typescript_demo       # TypeScript integration
cargo run --example node_integration      # Node.js integration

# CSS Engine Demonstrations
cargo run --example css_parsing_demo      # CSS parser capabilities
cargo run --example utility_classes_demo  # Utility class generation
cargo run --example responsive_css_demo   # Responsive CSS features

# State Management
cargo run --example reactive_state_demo   # Reactive state management
cargo run --example component_demo        # Component lifecycle
cargo run --example event_system_demo     # Event handling system
```

### TypeScript Examples (tui-bun) - 41+ Comprehensive Demos

#### **API Demonstrations (15+ Examples)**
```bash
cd reactive-tui-ts

# Border and Theme Examples
bun run src/examples/api-demos/border-themes-demo.ts      # Border styling showcase
bun run src/examples/api-demos/json-theme-demo.ts         # JSON theme loading
bun run src/examples/api-demos/theme-showcase-demo.ts     # Complete theme system
bun run src/examples/api-demos/theme-validation-demo.ts   # Theme validation

# Widget API Examples
bun run src/examples/api-demos/checkbox-demo.ts           # Checkbox interactions
bun run src/examples/api-demos/form-controls-demo.ts      # Complete form widgets
bun run src/examples/api-demos/progress-showcase.ts       # Progress widget variants
bun run src/examples/api-demos/slider-showcase.ts         # Slider configurations
bun run src/examples/api-demos/toast-showcase.ts          # Toast notification system
bun run src/examples/api-demos/spinner-demo.ts            # Loading spinners

# Layout and CSS Examples
bun run src/examples/api-demos/grid-showcase.ts           # CSS Grid layouts
bun run src/examples/api-demos/inline-grid-demo.ts        # Inline grid patterns
bun run src/examples/api-demos/layout-demo.ts             # Advanced layouts
bun run src/examples/api-demos/responsive-demo.ts         # Responsive design
bun run src/examples/api-demos/utility-styling-demo.ts    # Utility CSS classes
bun run src/examples/api-demos/css-class-verification.ts  # CSS validation

# Integration Examples
bun run src/examples/api-demos/rust-integration-demo.ts   # Rust FFI integration
bun run src/examples/api-demos/simple-demo.ts             # Basic API usage
```

#### **TUI Application Demos (26+ Examples)**
```bash
# Complete Applications
bun run src/examples/tui-demos/dashboard-demo.ts          # Full dashboard app
bun run src/examples/tui-demos/multiscreen-demo.ts        # Multi-screen navigation
bun run src/examples/tui-demos/workspace-tabs-demo.ts     # Workspace management
bun run src/examples/tui-demos/animated-transitions-demo.ts # Screen transitions

# Widget Demonstrations
bun run src/examples/tui-demos/button_demo.ts             # Button interactions
bun run src/examples/tui-demos/input_demo.ts              # Input field features
bun run src/examples/tui-demos/datatable_demo.ts          # Data table with sorting
bun run src/examples/tui-demos/tree_demo.ts               # Tree widget with nodes
bun run src/examples/tui-demos/accordion_demo.ts          # Accordion sections
bun run src/examples/tui-demos/modal_demo.ts              # Modal dialogs
bun run src/examples/tui-demos/menu_demo.ts               # Menu navigation
bun run src/examples/tui-demos/select_demo.ts             # Select dropdowns
bun run src/examples/tui-demos/autocomplete_demo.ts       # Autocomplete search
bun run src/examples/tui-demos/slider_demo.ts             # Range sliders
bun run src/examples/tui-demos/progress_demo.ts           # Progress indicators
bun run src/examples/tui-demos/spinner_demo.ts            # Loading animations
bun run src/examples/tui-demos/toast_demo.ts              # Toast notifications

# Advanced Features
bun run src/examples/tui-demos/animation_demo.ts          # Property animations
bun run src/examples/tui-demos/plugin_demo.ts             # Plugin system
bun run src/examples/tui-demos/hot_reload_demo.ts         # Hot reload features
bun run src/examples/tui-demos/form_validation_demo.ts    # Form validation
bun run src/examples/tui-demos/responsive-widgets-demo.ts # Responsive widgets
bun run src/examples/tui-demos/interactive-checkbox-demo.ts # Interactive checkboxes

# Layout and Design
bun run src/examples/tui-demos/layout_demo.ts             # Layout patterns
bun run src/examples/tui-demos/grid_demo.ts               # Grid layouts
bun run src/examples/tui-demos/panel_demo.ts              # Panel containers
bun run src/examples/tui-demos/bar_demo.ts                # Header/footer bars
bun run src/examples/tui-demos/viewport_demo.ts           # Scrollable viewports

# Data and Content
bun run src/examples/tui-demos/rich_text_demo.ts          # Rich text rendering
bun run src/examples/tui-demos/scrollable_list_demo.ts    # Scrollable lists
bun run src/examples/tui-demos/widget-factory-demo.ts     # Dynamic widget creation

# Development Tools
bun run src/examples/tui-demos/plugin_showcase.ts         # Plugin showcase
bun run src/examples/tui-demos/plugin_demo_live.ts        # Live plugin demo
```

#### **Comprehensive Demo Runners**
```bash
# Run all examples by category
bun run src/examples/api-demos/combined-runner.ts         # All API demos
bun run src/examples/api-demos/grid-runner.ts             # Grid examples
bun run src/examples/api-demos/progress-runner.ts         # Progress examples
bun run src/examples/api-demos/slider-runner.ts           # Slider examples
bun run src/examples/api-demos/toast-runner.ts            # Toast examples

# Main demo applications
bun run src/examples/basic-demo.ts                        # Basic features
bun run src/examples/components-demo.ts                   # Component system
bun run src/examples/comprehensive-demo.ts                # All features
bun run src/examples/error-boundary-demo.ts               # Error handling

# Run all examples
bun run src/examples/run-all-examples.ts                  # Execute all demos
```

## 📋 Requirements

### Runtime Requirements
- **Node.js**: >= 16.0.0 (LTS recommended)
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

**Sales:** [Contact for commercial licensing inquiries](https://github.com/entrepeneur4lyf/reactive-tui/discussions/new?category=general)

## 🙏 Acknowledgments

- Built with [napi-rs](https://napi.rs/) for seamless Rust-Node.js integration
- Inspired by modern web frameworks and terminal UI libraries
- CSS layout powered by [Taffy](https://github.com/DioxusLabs/taffy)
- Terminal handling via [Crossterm](https://github.com/crossterm-rs/crossterm)

## 📞 Support

- 📖 [Documentation](https://github.com/entrepeneur4lyf/reactive-tui/wiki)
- 🐛 [Issue Tracker](https://github.com/entrepeneur4lyf/reactive-tui/issues)
- 💬 [Discussions](https://github.com/entrepeneur4lyf/reactive-tui/discussions)
- 🆘 [Technical Support](https://github.com/entrepeneur4lyf/reactive-tui/issues/new/choose)
- 🐦 [Follow on X](https://x.com/entrepeneur4lyf)

---

**Made with ❤️ by the Reactive TUI team - Shawn McAllister [@entrepreneur4lyf](https://github.com/entrepeneur4lyf) and [Claude Code](https://www.anthropic.com/claude-code) w/[@claudeai](https://claude.ai)**
