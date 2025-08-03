# Reactive TUI TypeScript Framework (tui-bun)

| <img src="assets/logo.jpg" alt="Reactive TUI Logo" width="100" height="100"> | **Professional TypeScript TUI Framework with Rust-Powered Core**<br><br>[![NPM Version](https://img.shields.io/npm/v/reactive-tui.svg)](https://www.npmjs.com/package/reactive-tui) [![Build Status](https://img.shields.io/github/workflow/status/entrepeneur4lyf/reactive-tui/CI)](https://github.com/entrepeneur4lyf/reactive-tui/actions) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)<br>[![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/) [![Rust](https://img.shields.io/badge/Rust-Powered-orange.svg)](https://www.rust-lang.org/) [![Coverage](https://img.shields.io/badge/Coverage-72.31%25-brightgreen.svg)](#testing) |
|---|---|

*Advanced TypeScript framework for building sophisticated terminal applications with CSS styling, comprehensive widget library, and Rust-powered performance*

</div>

## 🚀 Overview

**tui-bun** is a comprehensive TypeScript framework built on top of the [Reactive TUI](https://www.npmjs.com/package/reactive-tui) Rust core. This is not just a collection of examples—it's a full-featured development framework that provides:

- **🏗️ Advanced Architecture**: 1,348-line app engine with sophisticated terminal rendering
- **🎨 Layout Engine**: 598-line CSS layout system with responsive design capabilities
- **🔌 Plugin System**: 692-line extensible architecture for custom widgets
- **🧭 Router System**: 432-line navigation with history and route guards
- **🎭 Theme Engine**: 690-line JSON-based theming with hot reload
- **🧩 Widget Library**: 25+ sophisticated widgets with TypeScript implementations
- **📱 41+ Examples**: Comprehensive demonstrations across API and TUI patterns

This framework bridges the gap between Rust performance and TypeScript developer experience, offering the best of both worlds for terminal application development.

## ✨ Framework Architecture

### 🏗️ **Core Framework Components**
- **App Engine** (1,348 lines): Advanced application management with terminal rendering, focus handling, and layered rendering system
- **Layout Engine** (598 lines): Sophisticated CSS layout computation with flexbox, grid, and responsive design
- **Plugin Manager** (692 lines): Extensible plugin architecture with dynamic loading, dependency resolution, and lifecycle management
- **Router System** (432 lines): Multi-screen navigation with history, route guards, and transition animations
- **Theme System** (690 lines): JSON-based theming with inheritance, hot reload, and semantic color mappings

### 🎯 **TypeScript Developer Experience**
- **Full Type Safety**: Complete TypeScript definitions with strict mode support
- **Modern ES Modules**: Clean import/export patterns with tree-shaking optimization
- **Bun Runtime**: Optimized for Bun's fast TypeScript execution and hot reload
- **Error Boundaries**: Comprehensive error handling and recovery systems
- **Development Tools**: Hot reload, CSS validation, performance monitoring

### 🎨 **Advanced Layout & Styling**
- **CSS Engine**: Full CSS parsing with flexbox, grid, responsive breakpoints, and animations
- **Responsive Design**: Terminal-aware layouts with `@media` queries for terminal width/height
- **Component Architecture**: React-like component system with lifecycle hooks and props
- **Theme Management**: JSON-based themes with semantic mappings and utility class generation
- **Animation System**: Property animations with easing functions and timeline control

### 🧩 **Comprehensive Widget Library (25+ Widgets)**

#### **Layout & Navigation**
- **Grid**: Advanced grid layouts with responsive behavior and column/row definitions
- **Bar**: Header/footer bars with flexible positioning (header, footer, navigation, status, toolbar)
- **Tabs**: Multi-orientation tab navigation (horizontal, vertical, bottom, card, minimal)
- **Modal**: Overlay dialogs with backdrop (alert, confirm, prompt, custom, fullscreen)
- **Accordion**: Expandable/collapsible sections with animations (compact, FAQ, settings)
- **Panel**: Container panels (dashboard, card, menu) with flexible layouts

#### **Form Controls & Input**
- **Input**: Text input with validation, placeholders, and state management (text, password, email, number, search, phone, URL)
- **Button**: Interactive buttons with 8 variants (primary, secondary, success, warning, danger, info, ghost, link)
- **Checkbox**: Single and grouped checkboxes with custom styling and animations
- **Switch**: Toggle switches with labels and state persistence
- **Radio**: Radio button groups with orientation control
- **Select**: Dropdown selection with search and filtering capabilities
- **Autocomplete**: Type-ahead search with suggestion filtering (command, country, language, user)
- **Slider**: Range sliders with ticks, orientation, and value formatting

#### **Data Display & Visualization**
- **DataTable**: Sortable, filterable tables with pagination, column management, and virtual scrolling (565-line implementation)
- **Tree**: Hierarchical tree with lazy loading, custom icons, and node management
- **ScrollableList**: Virtual scrolling lists with selection modes (file browser, menu, task lists)
- **Progress**: Progress bars with animations, colors, and custom styling (linear, circular, spinner)
- **Spinner**: Loading indicators with 30+ animation types (loading, processing, saving)
- **RichText**: Markdown rendering with syntax highlighting and custom elements
- **Viewport**: Scrollable areas with virtual rendering and lazy loading

#### **Feedback & Interaction**
- **Toast**: Notification toasts with positioning, duration, and variant types
- **Menu**: Context menus, menu bars, and dropdown navigation
- **FormValidator**: Real-time form validation with custom rules and error display
- **Animation**: Property animations with easing functions and timelines
- **HotReload**: Development hot reload for CSS and components

## 📦 Installation

```bash
# Clone this repository
git clone https://github.com/entrepeneur4lyf/reactive-tui.git
cd reactive-tui/reactive-tui-ts

# Install dependencies with Bun (recommended)
bun install

# Or with npm
npm install
```

## 🎮 Quick Start

> 📖 **For comprehensive examples and tutorials, see the [Examples Guide](./src/examples/README.md)**

### Basic Framework Usage

```typescript
import {
  createApp,
  createRouter,
  ThemeManager,
  div, text, button, input
} from 'tui-bun'

// Create advanced application with full framework features
async function createTuiApp() {
  // Initialize theme system
  const themeManager = new ThemeManager()
  await themeManager.loadThemeFile('./themes/professional-dark.json')

  // Create router for multi-screen navigation
  const router = createRouter({
    fullScreen: true,
    enableHistory: true,
    onNavigate: (event) => console.log(`Navigated to ${event.to}`)
  })

  // Register application routes
  router.registerRoutes({
    '/dashboard': {
      title: 'Dashboard',
      component: () => createDashboard()
    },
    '/settings': {
      title: 'Settings',
      component: () => createSettings()
    }
  })

  // Create main application
  const app = createApp({
    title: 'Advanced TUI Application',
    stylesheet: './styles/app.css',
    component: () => createMainLayout(router),
    theme: themeManager.getActiveTheme(),
    router: router,
    enableHotReload: true
  })

  await app.run()
}

// Create dashboard with advanced widgets
function createDashboard() {
  return div({ class: 'dashboard-container' })
    .child(
      div({ class: 'dashboard-header' })
        .child(text('System Dashboard').class('dashboard-title'))
    )
    .child(
      div({ class: 'dashboard-content' })
        .child(createMetricsGrid())
        .child(createDataTable())
    )
}

function createMetricsGrid() {
  return div({ class: 'metrics-grid' })
    .child(createMetricCard('CPU Usage', '67%', 'success'))
    .child(createMetricCard('Memory', '45%', 'warning'))
    .child(createMetricCard('Disk I/O', '23%', 'info'))
}

function createMetricCard(label: string, value: string, variant: string) {
  return div({ class: `metric-card metric-card--${variant}` })
    .child(text(label).class('metric-label'))
    .child(text(value).class('metric-value'))
}

// Start the application
createTuiApp().catch(console.error)
```

### Advanced CSS Styling

```css
/* Modern CSS with full framework support */
.dashboard-container {
  display: grid;
  grid-template-areas:
    "header header"
    "content sidebar";
  grid-template-rows: auto 1fr;
  grid-template-columns: 1fr 300px;
  height: 100vh;
  gap: 1rem;
  padding: 1rem;
  background: var(--color-background);
  color: var(--color-text);
}

.dashboard-header {
  grid-area: header;
  background: var(--color-surface);
  padding: 1rem;
  border-radius: var(--border-radius);
  border-bottom: 2px solid var(--color-primary);
}

.dashboard-title {
  font-size: 1.5rem;
  font-weight: bold;
  text-align: center;
  color: var(--color-primary);
}

.metrics-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin-bottom: 2rem;
}

.metric-card {
  background: var(--color-surface);
  padding: 1rem;
  border-radius: var(--border-radius);
  border-left: 4px solid var(--color-primary);
  transition: all 0.3s ease;
}

.metric-card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.2);
}

.metric-card--success {
  border-left-color: var(--color-success);
}

.metric-card--warning {
  border-left-color: var(--color-warning);
}

.metric-card--info {
  border-left-color: var(--color-info);
}

.metric-value {
  font-size: 1.5rem;
  font-weight: bold;
  color: var(--color-primary);
}

.metric-label {
  color: var(--color-text-secondary);
  font-size: 0.9rem;
}

/* Responsive design for different terminal sizes */
@media (max-width: 120) {
  .dashboard-container {
    grid-template-areas:
      "header"
      "content";
    grid-template-columns: 1fr;
  }

  .metrics-grid {
    grid-template-columns: 1fr;
  }
}
```

## 🎯 Comprehensive Examples (41+ Demos)

The framework includes extensive examples demonstrating all capabilities:

### **API Demonstrations (15+ Examples)**
```bash
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
bun run src/examples/api-demos/layout-demo.ts             # Advanced layouts
bun run src/examples/api-demos/responsive-demo.ts         # Responsive design
bun run src/examples/api-demos/utility-styling-demo.ts    # Utility CSS classes
bun run src/examples/api-demos/rust-integration-demo.ts   # Rust FFI integration
```

### **TUI Application Demos (26+ Examples)**
```bash
# Complete Applications
bun run src/examples/tui-demos/dashboard-demo.ts          # Full dashboard app
bun run src/examples/tui-demos/multiscreen-demo.ts        # Multi-screen navigation
bun run src/examples/tui-demos/workspace-tabs-demo.ts     # Workspace management
bun run src/examples/tui-demos/animated-transitions-demo.ts # Screen transitions

# Widget Demonstrations
bun run src/examples/tui-demos/datatable_demo.ts          # Data table with sorting
bun run src/examples/tui-demos/tree_demo.ts               # Tree widget with nodes
bun run src/examples/tui-demos/accordion_demo.ts          # Accordion sections
bun run src/examples/tui-demos/modal_demo.ts              # Modal dialogs
bun run src/examples/tui-demos/autocomplete_demo.ts       # Autocomplete search
bun run src/examples/tui-demos/progress_demo.ts           # Progress indicators
bun run src/examples/tui-demos/toast_demo.ts              # Toast notifications

# Advanced Features
bun run src/examples/tui-demos/plugin_demo.ts             # Plugin system
bun run src/examples/tui-demos/hot_reload_demo.ts         # Hot reload features
bun run src/examples/tui-demos/form_validation_demo.ts    # Form validation
bun run src/examples/tui-demos/animation_demo.ts          # Property animations

# Run all examples
bun run src/examples/run-all-examples.ts                  # Execute all demos
```

### **Quick Demo Commands**
```bash
# Main comprehensive demo
bun run demo:basic

# Individual examples
bun run demo:hello          # Hello World example
bun run demo:buttons        # Interactive buttons
bun run demo:cards          # Card layouts
bun run demo:dashboard      # Dashboard with metrics
bun run demo:toasts         # Toast notifications
bun run demo:themes         # Theme showcase

# List all available examples
bun run examples:list

# Get help with examples
bun run examples:help

# Show all available demos
bun run demo:all
```

> 📖 **For detailed information about each example, see the [Examples Guide](./src/examples/README.md)**

### 📊 Dashboard Demo
A comprehensive dashboard showcasing:
- Real-time data updates
- CSS Grid layouts
- Interactive charts and graphs
- Status indicators and metrics
- Responsive design patterns

### 🗂️ Workspace Demo
File manager-style interface featuring:
- Multiple tab management
- Tree view navigation
- Context menus and actions
- Keyboard shortcuts
- State persistence

### 🔄 Multiscreen Demo
Navigation between different views:
- Screen routing and transitions
- State management across screens
- Breadcrumb navigation
- Back/forward functionality

### ✨ Transitions Demo
Smooth animations and effects:
- Property animations
- State transition effects
- Loading indicators
- Progress animations

## 🛠️ Development

### Build System

```bash
# Development with hot reload
bun run dev

# Type checking
bun run typecheck

# Build for production
bun run build

# Clean build artifacts
bun run clean
```

### Code Quality

```bash
# Lint TypeScript code
bun run lint

# Format code
bun run format

# Run tests
bun run test
```

## 📚 Framework API Reference

### Core Framework Classes

#### `createApp()` - Advanced Application Builder
TypeScript-first application creation with comprehensive features:

```typescript
import { createApp, createRouter, ThemeManager } from 'tui-bun'

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
})

// Application methods
await app.run()                    // Start the application
await app.stop()                   // Stop the application
app.updateStylesheet(newCSS)       // Hot reload CSS
app.setComponent(newComponent)     // Update root component
app.navigate('/dashboard')         // Navigate to route
```

#### `LayoutEngine` - Advanced Layout System
Sophisticated layout computation with responsive design:

```typescript
import { LayoutEngine, ResponsiveWidget } from 'tui-bun'

const layoutEngine = new LayoutEngine({
  width: 400,
  height: 200,
  terminalSize: { width: 120, height: 30 }
})

// Responsive widget layout
const layout = layoutEngine.computeResponsiveLayout(widget, {
  x: 0, y: 0, width: 400, height: 200
})

// CSS-based layout computation
const cssLayout = layoutEngine.computeLayout(element, {
  display: 'flex',
  flexDirection: 'column',
  justifyContent: 'center',
  alignItems: 'stretch'
})

// Viewport management
layoutEngine.updateViewport({ width: 500, height: 300 })
const constraints = layoutEngine.getConstraints()
```

#### `ThemeManager` - Advanced Theme System
Comprehensive theme management with JSON configuration:

```typescript
import { ThemeManager, ThemeBuilder } from 'tui-bun'

const themeManager = new ThemeManager()

// Load themes from files
await themeManager.loadThemeFile('themes/dark.json')
await themeManager.loadThemeDirectory('./themes')

// Create custom themes
const customTheme = new ThemeBuilder()
  .name('Corporate Blue')
  .primary('#007acc')
  .secondary('#6c757d')
  .background('#1a1a1a')
  .surface('#2d2d2d')
  .build()

// Theme operations
themeManager.registerTheme(customTheme)
themeManager.setActiveTheme('corporate-blue')
const currentTheme = themeManager.getActiveTheme()

// Theme inheritance and composition
const extendedTheme = themeManager.extendTheme('dark', {
  colors: { primary: '#ff6b6b' }
})

// Hot reload themes
themeManager.enableHotReload('./themes')
themeManager.onThemeChange((theme) => app.applyTheme(theme))
```

#### `PluginManager` - Extensible Plugin System
Dynamic plugin loading and widget extension:

```typescript
import { PluginManager, WidgetPlugin, Plugin } from 'tui-bun'

const pluginManager = new PluginManager()

// Load plugins
await pluginManager.loadPlugin('./plugins/custom-widget.js')
await pluginManager.loadPluginsFromDirectory('./plugins')

// Create custom widget plugin
class CustomChartWidget extends WidgetPlugin {
  constructor() {
    super({
      name: 'custom-chart',
      version: '1.0.0',
      capabilities: ['widget-provider']
    })
  }

  render() {
    return div({ class: 'custom-chart' })
      .child(text('Custom Chart Widget'))
  }
}

// Register and use plugins
pluginManager.register(new CustomChartWidget())
const chartWidget = pluginManager.createWidget('custom-chart', {
  data: chartData,
  type: 'line'
})

// Plugin lifecycle
pluginManager.enablePlugin('custom-chart')
pluginManager.disablePlugin('custom-chart')
pluginManager.unloadPlugin('custom-chart')
```

#### `TUIRouter` - Navigation System
Advanced routing with history and guards:

```typescript
import { createRouter, fullScreenRouter } from 'tui-bun'

const router = createRouter({
  fullScreen: true,
  enableHistory: true,
  maxHistorySize: 50,
  onNavigate: (event) => console.log(`Navigated to ${event.to}`)
})

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
})

// Navigation
await router.navigate('/dashboard')
router.back()
router.forward()
router.replace('/login')

// Route guards and hooks
router.beforeEach(async (to, from) => {
  if (to.startsWith('/admin') && !isAdmin()) {
    return '/unauthorized'
  }
})

router.afterEach((to, from) => {
  analytics.track('page_view', { page: to })
})
```

### Advanced Widget System

#### `DataTable` - Sophisticated Data Management
565-line implementation with sorting, filtering, pagination, and virtual scrolling:

```typescript
import { dataTable, createColumn } from 'tui-bun'

interface UserData {
  id: number
  name: string
  email: string
  status: 'active' | 'inactive'
  lastLogin: Date
}

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
      console.log(`User ${row.name} ${selected ? 'selected' : 'deselected'}`)
    },
    onSort: (columnId, order) => {
      console.log(`Sorting by ${columnId} in ${order} order`)
    }
  }
})
```

#### `Tree` - Hierarchical Data Widget
Tree widget with lazy loading and custom icons:

```typescript
import { Tree, TreeBuilder, createTreeNode } from 'tui-bun'

const fileTree = new TreeBuilder()
  .root(createTreeNode('root', 'Project Files', 'folder'))
  .addNode('src', 'src/', 'folder', 'root')
  .addNode('components', 'components/', 'folder', 'src')
  .addNode('app.ts', 'app.ts', 'file', 'src')
  .addNode('index.ts', 'index.ts', 'file', 'src')
  .config({
    expandable: true,
    selectable: true,
    lazyLoad: true,
    showIcons: true
  })
  .callbacks({
    onNodeSelect: (nodeId, node) => console.log(`Selected: ${node.label}`),
    onNodeExpand: (nodeId, node) => loadChildNodes(nodeId),
    onNodeCollapse: (nodeId, node) => console.log(`Collapsed: ${node.label}`)
  })
  .build()
```

#### `Modal` - Advanced Dialog System
Modal dialogs with backdrop and positioning:

```typescript
import { modal, alertModal, confirmModal, promptModal } from 'tui-bun'

// Alert modal
const alert = alertModal({
  title: 'Success',
  message: 'Operation completed successfully!',
  type: 'success',
  buttons: ['OK']
})

// Confirmation modal
const confirm = confirmModal({
  title: 'Confirm Action',
  message: 'Are you sure you want to delete this item?',
  type: 'warning',
  buttons: ['Cancel', 'Delete'],
  onConfirm: () => deleteItem(),
  onCancel: () => console.log('Cancelled')
})

// Prompt modal
const prompt = promptModal({
  title: 'Enter Name',
  message: 'Please enter your name:',
  placeholder: 'Your name here...',
  validation: (value) => value.length >= 2,
  onSubmit: (value) => console.log(`Hello, ${value}!`)
})

// Custom modal
const customModal = modal({
  title: 'Custom Dialog',
  size: 'large',
  position: 'center',
  backdrop: true,
  closable: true,
  component: () => createCustomContent()
})
```

#### `Toast` - Notification System
Toast notifications with positioning and management:

```typescript
import { toast, ToastManager } from 'tui-bun'

// Create different toast types
const successToast = toast({
  message: 'Operation completed successfully!',
  variant: 'success',
  duration: 3000,
  position: 'top-right',
  closable: true
})

const errorToast = toast({
  message: 'An error occurred',
  variant: 'error',
  duration: 5000,
  position: 'top-center',
  actions: [
    { label: 'Retry', action: () => retryOperation() },
    { label: 'Dismiss', action: () => toast.dismiss() }
  ]
})

// Toast manager
const toastManager = new ToastManager({
  maxToasts: 5,
  defaultDuration: 3000,
  defaultPosition: 'top-right'
})

toastManager.show(successToast)
toastManager.show(errorToast)
toastManager.dismissAll()
```

## 🎨 CSS Styling

### Modern CSS Features

```css
/* Flexbox layouts */
.flex-container {
  display: flex;
  flex-direction: column;
  justify-content: space-between;
  align-items: stretch;
  gap: 1rem;
}

/* CSS Grid */
.grid-layout {
  display: grid;
  grid-template-columns: 200px 1fr 200px;
  grid-template-rows: auto 1fr auto;
  grid-gap: 1rem;
}

/* Responsive design */
@media (max-width: 80) {
  .responsive {
    flex-direction: column;
  }
}
```

### Color System

```css
/* Semantic colors */
.primary { background: var(--color-primary); }
.secondary { background: var(--color-secondary); }
.success { background: var(--color-success); }
.warning { background: var(--color-warning); }
.error { background: var(--color-error); }

/* Utility classes */
.bg-dark { background: #1e1e1e; }
.text-light { color: #ffffff; }
.border-subtle { border: 1px solid #333; }
```

## 🔧 Configuration

### TypeScript Configuration

The project uses optimized TypeScript settings:

```json
{
  "extends": "@tsconfig/bun/tsconfig.json",
  "compilerOptions": {
    "strict": true,
    "noUncheckedIndexedAccess": true,
    "exactOptionalPropertyTypes": true
  }
}
```

### Bun Configuration

Optimized for Bun runtime with hot reload support and fast builds.

## 📖 Learning Resources

### Example Patterns

1. **Component Composition**: Building complex UIs from simple components
2. **State Management**: Handling application state with TypeScript
3. **Event Handling**: Type-safe event listeners and callbacks
4. **CSS Architecture**: Scalable styling patterns for terminal UIs
5. **Performance**: Optimizing rendering and memory usage

### Best Practices

- Use TypeScript strict mode for maximum type safety
- Leverage CSS custom properties for theming
- Implement proper error boundaries
- Follow semantic HTML-like structure
- Use meaningful CSS class names

## 🤝 Contributing

We welcome contributions! Please:

1. Fork the repository
2. Create a feature branch
3. Add TypeScript examples with proper typing
4. Include documentation and comments
5. Submit a pull request

## 📄 License

MIT License - see [LICENSE](LICENSE) for details.

## 🙏 Acknowledgments

- Built on [Reactive TUI](https://www.npmjs.com/package/reactive-tui) framework
- Powered by [Bun](https://bun.sh/) runtime
- TypeScript integration by the community

## 🚀 Advanced Examples

### Real-World Applications

#### File Manager
```typescript
import { JsTuiApp, TuiUtils } from 'reactive-tui'

class FileManager {
  private app: JsTuiApp
  private currentPath: string = '/'

  constructor() {
    this.app = new JsTuiApp()
    this.setupUI()
  }

  private setupUI(): void {
    // Implementation with proper TypeScript typing
    const container = TuiUtils.div()
    container.addClass('file-manager')

    // Add breadcrumb navigation
    const breadcrumb = this.createBreadcrumb()
    container.addChild(breadcrumb)

    // Add file tree
    const fileTree = this.createFileTree()
    container.addChild(fileTree)

    this.app.setComponent(container)
  }

  private createBreadcrumb(): JsElement {
    const breadcrumb = TuiUtils.div()
    breadcrumb.addClass('breadcrumb')
    breadcrumb.setContent(`📁 ${this.currentPath}`)
    return breadcrumb
  }

  private createFileTree(): JsElement {
    // File tree implementation with type safety
    const tree = TuiUtils.div()
    tree.addClass('file-tree')
    return tree
  }
}
```

#### Data Dashboard
```typescript
interface MetricData {
  label: string
  value: number
  trend: 'up' | 'down' | 'stable'
  color: string
}

class Dashboard {
  private metrics: MetricData[] = []

  constructor(private app: JsTuiApp) {
    this.initializeMetrics()
    this.render()
  }

  private initializeMetrics(): void {
    this.metrics = [
      { label: 'CPU Usage', value: 45, trend: 'up', color: '#ff6b6b' },
      { label: 'Memory', value: 67, trend: 'stable', color: '#4ecdc4' },
      { label: 'Disk I/O', value: 23, trend: 'down', color: '#45b7d1' }
    ]
  }

  private render(): void {
    const dashboard = TuiUtils.div()
    dashboard.addClass('dashboard-grid')

    this.metrics.forEach(metric => {
      const card = this.createMetricCard(metric)
      dashboard.addChild(card)
    })

    this.app.setComponent(dashboard)
  }

  private createMetricCard(metric: MetricData): JsElement {
    const card = TuiUtils.div()
    card.addClass('metric-card')
    card.setAttribute('data-trend', metric.trend)

    const label = TuiUtils.div()
    label.addClass('metric-label')
    label.setContent(metric.label)

    const value = TuiUtils.div()
    value.addClass('metric-value')
    value.setContent(`${metric.value}%`)

    card.addChild(label)
    card.addChild(value)

    return card
  }
}
```

## 🎯 Performance Optimization

### Memory Management
```typescript
class OptimizedComponent {
  private elements: Map<string, JsElement> = new Map()

  createElement(id: string, tag: string): JsElement {
    if (this.elements.has(id)) {
      return this.elements.get(id)!
    }

    const element = new JsElement(tag)
    element.setId(id)
    this.elements.set(id, element)

    return element
  }

  cleanup(): void {
    this.elements.clear()
  }
}
```

### Efficient Rendering
```typescript
interface RenderOptions {
  batchSize: number
  debounceMs: number
}

class BatchRenderer {
  private pendingUpdates: JsElement[] = []
  private updateTimer: Timer | null = null

  constructor(private options: RenderOptions) {}

  scheduleUpdate(element: JsElement): void {
    this.pendingUpdates.push(element)

    if (this.updateTimer) {
      clearTimeout(this.updateTimer)
    }

    this.updateTimer = setTimeout(() => {
      this.flushUpdates()
    }, this.options.debounceMs)
  }

  private flushUpdates(): void {
    const batch = this.pendingUpdates.splice(0, this.options.batchSize)
    // Process batch updates
    this.updateTimer = null
  }
}
```

## 🧪 Testing Examples

### Unit Testing with Bun
```typescript
import { test, expect } from 'bun:test'
import { TuiUtils, JsElement } from 'reactive-tui'

test('TuiUtils creates elements with correct types', () => {
  const div = TuiUtils.div()
  expect(div).toBeInstanceOf(JsElement)

  const button = TuiUtils.button()
  expect(button).toBeInstanceOf(JsElement)
})

test('Element hierarchy works correctly', () => {
  const parent = TuiUtils.div()
  const child = TuiUtils.text('Hello')

  parent.addChild(child)
  // Test parent-child relationship
})
```

### Integration Testing
```typescript
import { JsTuiApp } from 'reactive-tui'

test('App initialization and component setting', () => {
  const app = new JsTuiApp()
  app.setTitle('Test App')

  const container = TuiUtils.div()
  app.setComponent(container)

  const status = app.start()
  expect(status).toContain('initialized')
})
```

## 🔍 Debugging and Development

### Debug Utilities
```typescript
class DebugUtils {
  static logElementTree(element: JsElement, depth: number = 0): void {
    const indent = '  '.repeat(depth)
    console.log(`${indent}Element: ${element.constructor.name}`)
    // Log element properties and children
  }

  static validateCSS(css: string): string[] {
    return TuiUtils.validateCss(css)
  }

  static getTerminalInfo(): { width: number; height: number } {
    const [width, height] = TuiUtils.getTerminalSize()
    return { width, height }
  }
}
```

### Development Workflow
```bash
# Start development server with hot reload
bun run dev

# Run type checking in watch mode
bun run typecheck --watch

# Format code on save
bun run format --watch

# Run tests continuously
bun run test --watch
```

## 📊 Framework Architecture

```
reactive-tui-ts/                    # TypeScript Framework Root
├── src/
│   ├── index.ts                    # Main framework exports (188 lines)
│   ├── app.ts                      # Application engine (1,348 lines)
│   ├── app-new.ts                  # Next-gen app architecture
│   ├── components.ts               # Component system (382 lines)
│   ├── layout.ts                   # Layout engine (598 lines)
│   ├── router.ts                   # Router system (432 lines)
│   ├── plugin.ts                   # Plugin architecture (692 lines)
│   ├── css.ts                      # CSS utilities and hot reload
│   ├── events.ts                   # Event system
│   ├── error-boundary.ts           # Error handling
│   ├── error-reporter.ts           # Error reporting
│   ├── types.ts                    # TypeScript definitions
│   ├── utils.ts                    # Utility functions
│   ├── widget-factory.ts           # Dynamic widget creation
│   ├── generated-types.ts          # Auto-generated types
│   │
│   ├── themes/                     # Advanced Theme System
│   │   ├── theme-system.ts         # Theme manager (690 lines)
│   │   ├── colors.ts               # Color system
│   │   ├── borders.ts              # Border themes
│   │   └── json-loader.ts          # JSON theme loading
│   │
│   ├── widgets/                    # Comprehensive Widget Library (25+ widgets)
│   │   ├── base-widget.ts          # Base widget class (254 lines)
│   │   ├── datatable.ts            # Data table (565 lines)
│   │   ├── tree.ts                 # Tree widget
│   │   ├── accordion.ts            # Accordion sections
│   │   ├── modal.ts                # Modal dialogs
│   │   ├── autocomplete.ts         # Autocomplete search
│   │   ├── progress.ts             # Progress indicators
│   │   ├── slider.ts               # Range sliders
│   │   ├── toast.ts                # Toast notifications
│   │   ├── button.ts               # Button widgets
│   │   ├── input.ts                # Input fields
│   │   ├── checkbox.ts             # Checkbox controls
│   │   ├── radio.ts                # Radio buttons
│   │   ├── select.ts               # Select dropdowns
│   │   ├── switch.ts               # Toggle switches
│   │   ├── tabs.ts                 # Tab navigation
│   │   ├── menu.ts                 # Menu systems
│   │   ├── panel.ts                # Panel containers
│   │   ├── bar.ts                  # Header/footer bars
│   │   ├── grid.ts                 # Grid layouts
│   │   ├── spinner.ts              # Loading spinners
│   │   ├── rich_text.ts            # Rich text rendering
│   │   ├── viewport.ts             # Scrollable viewports
│   │   ├── scrollable_list.ts      # Virtual scrolling lists
│   │   ├── form_validation.ts      # Form validation
│   │   ├── animation.ts            # Animation system
│   │   ├── hot_reload.ts           # Hot reload features
│   │   └── factory-button.ts       # Button factory
│   │
│   └── examples/                   # Comprehensive Examples (41+ demos)
│       ├── README.md               # Examples documentation
│       ├── RUST_INTEGRATION.md     # Rust integration guide
│       ├── UTILITY_STYLING.md      # CSS utility guide
│       ├── package.json            # Example dependencies
│       ├── styles.css              # Example styles
│       ├── run-all-examples.ts     # Example runner
│       │
│       ├── api-demos/              # API Demonstrations (15+ examples)
│       │   ├── border-themes-demo.ts
│       │   ├── json-theme-demo.ts
│       │   ├── theme-showcase-demo.ts
│       │   ├── checkbox-demo.ts
│       │   ├── form-controls-demo.ts
│       │   ├── progress-showcase.ts
│       │   ├── slider-showcase.ts
│       │   ├── toast-showcase.ts
│       │   ├── grid-showcase.ts
│       │   ├── layout-demo.ts
│       │   ├── responsive-demo.ts
│       │   ├── utility-styling-demo.ts
│       │   ├── rust-integration-demo.ts
│       │   └── combined-runner.ts
│       │
│       └── tui-demos/              # TUI Applications (26+ examples)
│           ├── dashboard-demo.ts
│           ├── multiscreen-demo.ts
│           ├── workspace-tabs-demo.ts
│           ├── animated-transitions-demo.ts
│           ├── datatable_demo.ts
│           ├── tree_demo.ts
│           ├── accordion_demo.ts
│           ├── modal_demo.ts
│           ├── plugin_demo.ts
│           ├── hot_reload_demo.ts
│           ├── form_validation_demo.ts
│           ├── animation_demo.ts
│           └── [21+ more demos...]
│
├── themes/                         # JSON Theme Definitions
├── dist/                           # Built framework output
├── package.json                    # Framework dependencies
├── tsconfig.json                   # TypeScript configuration
└── README.md                       # Framework documentation
```

### **Framework Statistics**
- **Total Lines**: 10,000+ lines of TypeScript code
- **Core Architecture**: 5 major systems (App, Layout, Router, Plugin, Theme)
- **Widget Library**: 25+ sophisticated widgets with full implementations
- **Examples**: 41+ comprehensive demonstrations
- **Type Safety**: 100% TypeScript with strict mode
- **Performance**: Optimized for Bun runtime with hot reload

## 🌟 Community Examples

### Contributed Examples
- **Terminal IDE**: Code editor with syntax highlighting
- **System Monitor**: Real-time system metrics
- **Chat Application**: Multi-user terminal chat
- **Game Framework**: Terminal-based games
- **API Client**: REST API testing tool

### Example Gallery
Visit our [Example Gallery](https://github.com/entrepeneur4lyf/reactive-tui/wiki/examples) to see community-contributed applications and get inspiration for your own projects.

---

<div align="center">

**Made with ❤️ and TypeScript**

[Documentation](https://github.com/entrepeneur4lyf/reactive-tui/wiki) • [Examples](./examples) • [API Reference](https://github.com/entrepeneur4lyf/reactive-tui#api-reference) • [Community](https://github.com/entrepeneur4lyf/reactive-tui/discussions)

</div>
