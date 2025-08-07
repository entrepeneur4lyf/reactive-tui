# Reactive TUI Interactive Widget Guide

A comprehensive demonstration application showcasing all 29+ widgets available in the Reactive TUI framework. This guide provides both documentation and interactive examples for each widget, serving as both a learning tool and a showcase of the framework's capabilities.

## Features

- **🎯 Interactive Demonstrations**: Live widget instances with real-time parameter control
- **📚 Comprehensive Documentation**: Formatted Markdown with syntax highlighting for code examples
- **⚡ Performance Monitoring**: 60 FPS target with real-time framerate display and memory usage tracking
- **🔧 Plugin Architecture**: Modular design with lazy loading of widget demonstrations
- **🎨 Professional UI**: Splash screen with Sixel logo, responsive navigation, and polished layout
- **🏗️ Widget Categories**: Organized by Layout & Navigation, Form Controls, Data Display, Content & Feedback, and Development

## Quick Start

### Prerequisites

- Rust 1.70+ with Cargo
- Terminal with true color support
- Optional: Sixel-capable terminal for enhanced image display

### Running the Guide

```bash
# From the reactive-tui project root
cd guide
cargo run --bin reactive-tui-guide
```

### Development Mode

```bash
# Run with debug features enabled
cargo run --bin reactive-tui-guide --features debug-mode
```

## Architecture

The Interactive Widget Guide follows a plugin-based architecture as specified in the technical requirements:

### Core Components

- **Application State**: Reactive state management using the framework's built-in reactive system
- **Screen Manager**: Handles navigation between splash, home, widget demo, and category screens
- **Widget Registry**: Dynamic loading and metadata management for all available widgets
- **Performance Monitor**: Real-time tracking of framerate, memory usage, and response times
- **Demo Factory**: Creates interactive demonstrations and documentation views for each widget

### Screen Flow

1. **Splash Screen**: Branded interface with framework logo and version information
2. **Home Screen**: Framework overview, feature highlights, and getting started information
3. **Category Overview**: Browse widgets organized by functional categories
4. **Widget Demonstration**: Toggle between documentation and interactive modes for each widget

## Widget Categories

### 🏗️ Layout & Navigation
- Accordion, Bar, Grid, Modal, Panel, Tabs

### 📝 Form Controls  
- Autocomplete, Button, Checkbox, Input, Radio, Select, Slider, Switch

### 📊 Data Display
- DataTable, Progress, ScrollableList, Spinner, Tree, Viewport

### 🎨 Content & Feedback
- Animation, FormValidation, HotReload, Image, Menu, RichText, Toast

### 🔧 Development
- BaseWidget

## Navigation

### Keyboard Shortcuts

- **Enter**: Navigate from splash screen to home
- **Tab**: Toggle between Documentation and Interactive modes (in widget demos)
- **ESC**: Go back to previous screen
- **↑/↓**: Navigate categories and widget lists
- **←/→**: Navigate between widgets in the same category
- **Ctrl+C**: Quit application

### Interactive Mode

When viewing a widget in Interactive mode:

- **Real-time Parameter Control**: Modify widget properties using form controls
- **Live Preview**: See changes immediately reflected in the widget
- **Code Export**: Copy current widget configuration as code snippet
- **Performance Metrics**: Monitor rendering performance and memory usage

## Performance Targets

As specified in the technical requirements:

- **60 FPS Target**: Adaptive framerate with real-time monitoring
- **Sub-100ms Response Time**: For navigation and widget interactions  
- **Memory Efficiency**: <50MB RAM usage for typical usage patterns
- **Flicker-Free Rendering**: Demonstrating framework's advanced rendering capabilities

## Development

### Project Structure

```
guide/
├── src/
│   ├── app.rs              # Main application structure
│   ├── config.rs           # Configuration management
│   ├── state.rs            # Application state management
│   ├── performance/        # Performance monitoring
│   ├── registry/           # Widget registry and metadata
│   ├── screens/            # Screen management and components
│   └── widgets/            # Widget demonstrations and controls
├── styles/                 # CSS stylesheets
├── assets/                 # Images and resources
└── docs/                   # Additional documentation
```

### Adding New Widget Demonstrations

1. Implement the `WidgetDemo` trait for your widget
2. Register the demo in `WidgetDemoFactory`
3. Add metadata to the widget registry
4. Update category configuration if needed

### Customization

The guide can be customized through:

- **Configuration**: Modify `GuideConfig` for application settings
- **Styling**: Update CSS stylesheets for visual customization
- **Content**: Add or modify widget documentation and examples
- **Performance**: Adjust monitoring thresholds and targets

## License

This project is licensed under the Business Source License 1.1 (BUSL-1.1) - see the LICENSE file for details.
