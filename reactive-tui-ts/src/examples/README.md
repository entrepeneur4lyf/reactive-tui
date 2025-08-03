# 🎯 TUI Framework Examples

This directory contains comprehensive examples demonstrating all features of the TUI framework, separated into **API demos** (component structure) and **TUI demos** (interactive terminal interfaces).

## 📁 Directory Structure

```
examples/
├── api-demos/         # Component API demonstrations (JSON output)
├── tui-demos/         # Interactive terminal user interfaces
├── README.md          # This file
├── package.json       # Node.js dependencies
├── styles.css         # Shared CSS styles
└── run-all-examples.ts # Master runner script
```

## 🎯 **Two Types of Examples**

### 📊 **API Demos** (`api-demos/`)
- **Purpose**: Demonstrate component APIs and structure
- **Output**: JSON component trees and configuration examples
- **Use Case**: Understanding widget APIs, testing component logic, documentation
- **Run Time**: Fast execution, no user interaction required

### 🖥️ **TUI Demos** (`tui-demos/`)
- **Purpose**: Full interactive terminal user interfaces
- **Output**: Live terminal applications with real-time interaction
- **Use Case**: Testing user experience, demonstrating complete applications
- **Run Time**: Interactive, requires user input, demonstrates real TUI behavior

## 🖥️ **Interactive TUI Demos**

### 🎯 **Interactive Checkbox Demo**
- **`tui-demos/interactive-checkbox-demo.ts`** - Fully interactive checkbox interface
  - Arrow key navigation between checkboxes
  - Spacebar/Enter to toggle states
  - Real-time smooth scaling animations
  - Focus indicators and visual feedback
  - Live state display and updates

### 🎛️ **Comprehensive TUI Demo**
- **`tui-demos/comprehensive-tui-demo.ts`** - Complete multi-widget application
  - Tab-based navigation between sections
  - Settings panel with checkboxes and sliders
  - Real-time progress indicators
  - Interactive controls and switches
  - Live status updates and state management

## 📊 **API Component Demos**

### ✅ **Checkbox API**
- **`api-demos/checkbox-demo.ts`** - Checkbox component API demonstration
  - Component structure and configuration options
  - Animation state examples
  - Style variations and label positioning
  - JSON output for documentation

### 🔄 **Spinner API**
- **`api-demos/spinner-demo.ts`** - Spinner component showcase
  - 30+ predefined spinner types
  - Custom spinner definitions
  - Animation configuration examples

### 📊 **Progress API**
- **`api-demos/progress-examples.ts`** - Progress component examples
  - Linear and circular progress bars
  - Configuration options and styling

### 🎚️ **Slider API**
- **`api-demos/slider-examples.ts`** - Slider component demonstrations
  - Range slider configurations
  - Tick marks and value formatting

### 🔘 **Form Controls API**
- **`api-demos/form-controls-demo.ts`** - Form widget API examples
  - Switch, radio, and checkbox components
  - Validation and accessibility features

### 🍞 **Toast API**
- **`api-demos/toast-examples.ts`** - Toast notification API
  - Toast types and positioning
  - Animation and timing configurations

### 🏗️ **Layout API**
- **`api-demos/grid-examples.ts`** - Grid system API demonstrations
- **`api-demos/layout-demo.ts`** - Layout pattern examples
- **`api-demos/comprehensive-grid-demo.ts`** - Advanced grid configurations

### 🎨 **Theme API**
- **`api-demos/theme-showcase-demo.ts`** - Theme system API
- **`api-demos/border-themes-demo.ts`** - Border styling API
- **`api-demos/json-theme-demo.ts`** - JSON-based theme loading

### 🔧 **Test Runners**
- **`api-demos/combined-runner.ts`** - Run multiple widget tests
- **`api-demos/progress-runner.ts`** - Progress widget test runner
- **`api-demos/slider-runner.ts`** - Slider widget test runner
- **`api-demos/toast-runner.ts`** - Toast notification test runner
- **`api-demos/grid-runner.ts`** - Grid layout test runner

## 🚀 **Running Examples**

### 🖥️ **Interactive TUI Demos** (Recommended)
```bash
# Interactive form with real keyboard controls
bun run examples/tui-demos/interactive-form-demo.ts

# Real-time system dashboard with live metrics
bun run examples/tui-demos/realtime-dashboard-demo.ts

# File manager - browse your actual file system
bun run examples/tui-demos/file-manager-demo.ts

# Snake game - fully playable in terminal
bun run examples/tui-demos/snake-game-demo.ts

# Chat interface with simulated users
bun run examples/tui-demos/chat-interface-demo.ts

# Original conceptual demos
bun run examples/tui-demos/interactive-checkbox-demo.ts
bun run examples/tui-demos/comprehensive-tui-demo.ts
```

### 📊 **API Component Demos**
```bash
# Run individual API demos (JSON output)
bun run examples/api-demos/checkbox-demo.ts
bun run examples/api-demos/spinner-demo.ts
bun run examples/api-demos/progress-examples.ts

# Run all API demos
bun run examples/run-all-examples.ts api-demos
```

### 🔧 **Master Runner**
```bash
# List all available examples
bun run examples/run-all-examples.ts list

# Run all TUI demos
bun run examples/run-all-examples.ts tui-demos

# Run all API demos
bun run examples/run-all-examples.ts api-demos

# Run everything
bun run examples/run-all-examples.ts
```

### 🦀 **Rust Examples**
```bash
# Navigate to tui-core
cd packages/tui-core

# Run individual examples
cargo run --example checkbox_showcase
cargo run --example spinner_showcase
cargo run --example progress_showcase
```

## 📋 **Additional Examples**

### 🎯 **Basic Examples** (Root Level)
- **`basic-demo.ts`** - Simple getting started example
- **`components-demo.ts`** - Basic component usage
- **`comprehensive-demo.ts`** - Complete feature overview

### 🔧 **Advanced API Examples**
- **`api-demos/panel-demo.ts`** - Panel component demonstrations
- **`api-demos/simple-demo.ts`** - Simple widget examples
- **`api-demos/theme-validation-demo.ts`** - Theme validation examples

## 🌟 **Featured Examples**

### 🏆 **Most Comprehensive**
1. **`tui-demos/interactive-checkbox-demo.ts`** - Interactive checkbox navigation
2. **`tui-demos/comprehensive-tui-demo.ts`** - Multi-widget TUI application
3. **`api-demos/checkbox-demo.ts`** - Complete checkbox API with animations
4. **`api-demos/form-controls-demo.ts`** - Full form widget library
5. **`api-demos/comprehensive-grid-demo.ts`** - Advanced grid layouts

### 🎬 **Animation Showcases**
1. **`api-demos/checkbox-demo.ts`** - Smooth scaling checkbox animations
2. **`api-demos/spinner-demo.ts`** - 30+ rotating and morphing spinners
3. **`api-demos/progress-examples.ts`** - Animated progress indicators
4. **`api-demos/toast-examples.ts`** - Sliding toast animations

### 🖥️ **Interactive TUI Experiences**
1. **`tui-demos/interactive-checkbox-demo.ts`** - Arrow key navigation and focus
2. **`tui-demos/comprehensive-tui-demo.ts`** - Tab-based multi-section interface

### ♿ **Accessibility Examples**
1. **`api-demos/form-controls-demo.ts`** - ARIA attributes and keyboard navigation
2. **`api-demos/checkbox-demo.ts`** - Screen reader compatibility
3. **`tui-demos/interactive-checkbox-demo.ts`** - Focus management and keyboard controls

## 🎨 **Styling Examples**

### 🌈 **Color Schemes**
- Light and dark themes
- Custom color palettes
- Accessibility-compliant colors
- Brand-specific styling

### 🎯 **Animation Styles**
- Smooth transitions
- Easing functions
- Micro-interactions
- Performance optimization

## 📚 **Learning Path**

### 1. **Start Here**
- `basic-demo.ts` - Understand the basics
- `components-demo.ts` - Learn component structure

### 2. **Explore Component APIs**
- `api-demos/checkbox-demo.ts` - Interactive form controls
- `api-demos/spinner-demo.ts` - Loading states and animations
- `api-demos/progress-examples.ts` - Progress indication

### 3. **Experience TUI Interactions**
- `tui-demos/interactive-checkbox-demo.ts` - Keyboard navigation
- `tui-demos/comprehensive-tui-demo.ts` - Multi-widget applications

### 4. **Master Layouts**
- `api-demos/grid-examples.ts` - Grid system
- `api-demos/layout-demo.ts` - Common patterns

### 5. **Customize Themes & Styling**
- `api-demos/theme-showcase-demo.ts` - Styling system
- `api-demos/border-themes-demo.ts` - Visual customization
- `api-demos/utility-styling-demo.ts` - Utility-first CSS support

### 6. **Advanced Features**
- `api-demos/combined-runner.ts` - Integration patterns
- Custom widget development
- Performance optimization

## 🔧 **Development Tools**

### 📊 **Debugging**
- Grid debugging utilities
- Component inspection tools
- Performance profiling
- Error handling examples

### 🧪 **Testing**
- Widget test runners
- Integration test examples
- Accessibility testing
- Cross-platform validation

---

## 🎯 **Quick Start**

```bash
# Install dependencies
npm install

# Experience interactive TUI demo
bun run examples/tui-demos/interactive-checkbox-demo.ts

# Explore component APIs
bun run examples/api-demos/checkbox-demo.ts

# See utility-first CSS support
bun run examples/api-demos/utility-styling-demo.ts

# Explore TypeScript ↔ Rust integration
bun run examples/api-demos/rust-integration-demo.ts

# Run comprehensive overview
bun run examples/comprehensive-demo.ts

# List all available examples
bun run examples/run-all-examples.ts list
```

## 📊 **Current Status**

### ✅ **Working Examples**
- **API Demos**: 28 component demonstrations with JSON output
- **TUI Demos**: 7 fully interactive terminal applications
- **Root Examples**: 4 basic getting-started examples
- **Master Runner**: Automated example execution and categorization
- **Styling Guide**: Comprehensive utility-first CSS support documentation
- **CSS Verification**: Automated testing of utility class support
- **Rust Integration**: TypeScript ↔ Rust utility CSS processing demo
- **Total**: 41 TypeScript examples + Rust examples

### 🚧 **In Development**
- Full TUI framework implementation for interactive demos
- Additional interactive TUI examples
- Real-time animation rendering in terminal

### 🎯 **Example Categories**
- **📊 API Demos**: Fast, JSON-based component structure examples
- **🖥️ TUI Demos**: Interactive terminal user interfaces
- **🔧 Test Runners**: Automated testing and validation tools

### 🎨 **Styling System**
- **🌊 Utility CSS**: Full utility-first CSS support on all widgets
- **📋 Utility Classes**: Complete control with `classes: ['p-4', 'text-blue-600', 'hover:shadow-lg']`
- **🎯 Semantic Classes**: Built-in widget classes (`checkbox`, `progress-linear`, `spinner-running`)
- **📱 Responsive Design**: Mobile-first responsive utilities (`md:grid-cols-2`, `lg:text-lg`)
- **🎬 Animations**: Smooth transitions and hover effects (`transition-all`, `hover:scale-105`)

**See**: `examples/UTILITY_STYLING.md` for complete documentation

### 🔗 **TypeScript ↔ Rust Integration**
- **🎨 Exact Color Matching**: Identical utility colors in TypeScript and Rust
- **⚡ ANSI Processing**: Rust `UtilityProcessor` converts classes to terminal codes
- **🔧 Seamless Integration**: TypeScript widgets → Rust backend → Terminal output
- **📊 Performance**: Fast HashMap lookups and efficient ANSI generation
- **🎯 Complete Pipeline**: `classes: ['text-blue-600']` → `\x1B[38;2;37;99;235m`

**See**: `examples/RUST_INTEGRATION.md` for complete integration guide

**Happy coding!** 🚀
