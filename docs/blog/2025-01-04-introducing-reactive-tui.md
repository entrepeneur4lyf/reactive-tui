---
slug: introducing-reactive-tui
title: Introducing Reactive TUI - Revolutionary CSS-styled Terminal UIs
authors: [shawn]
tags: [announcement, css, terminal, ui, rust, typescript]
date: 2025-01-04
---

# Introducing Reactive TUI

We're excited to announce **Reactive TUI**, a revolutionary CSS-styled terminal user interface framework that brings modern web-like styling to terminal applications.

<!-- truncate -->

## The Problem with Traditional TUIs

Traditional terminal user interfaces have been limited by basic ASCII art styling, making it challenging to create beautiful, maintainable, and responsive terminal applications. Developers had to choose between:

- **Basic TUI libraries** with limited styling capabilities
- **Complex custom rendering** that required significant time investment
- **Web-based solutions** that couldn't run in pure terminal environments

## The Reactive TUI Solution

Reactive TUI bridges this gap by providing:

### 🎨 **Full CSS Support**
- Complete CSS parsing with selectors (tag, class, ID)
- Flexbox and CSS Grid layout engines
- Responsive breakpoints and utility classes
- Theme system with JSON configuration

### ⚡ **High Performance**
- Rust core with NAPI bindings for Node.js/Bun
- Virtual rendering for large datasets
- Dirty region tracking to minimize redraws
- Arc/RwLock-based thread-safe state management

### 🧩 **Comprehensive Widget Library**
Over 25 built-in widgets including:
- **Layout**: Grid, Bar, Tabs, Modal, Accordion
- **Forms**: Input, Button, Checkbox, Radio, Select, Slider
- **Data**: DataTable, Tree, ScrollableList, Viewport
- **Feedback**: Toast, Progress, Spinner, Animation

### 🔄 **Reactive State Management**
- Broadcast channels for reactive updates
- Field-level change notifications
- Thread-safe state with Arc/RwLock
- Workspace isolation

## TypeScript + Rust Architecture

Reactive TUI combines the best of both worlds:

```typescript
// TypeScript Layer - Component system and developer API
import { createApp, button, input, grid } from 'reactive-tui';

const app = createApp({
  title: 'My App',
  stylesheet: './styles.css'
});

const loginForm = grid('login-form', config => config
  .template('1fr / 1fr 1fr')
  .gap('1rem')
  .child(input('username', c => c.placeholder('Username')))
  .child(button('submit', c => c.text('Login').variant('primary')))
);
```

```rust
// Rust Core - High-performance rendering engine
use reactive_tui::prelude::*;

fn main() -> Result<()> {
    let mut app = TuiApp::builder()
        .title("My App")
        .theme_from_file("./theme.json")?
        .build()?;
    
    app.run()
}
```

## Real-World Examples

We've built comprehensive examples showing Reactive TUI in action:

- **Dashboard Applications** with real-time data visualization
- **Code Editors** with syntax highlighting and file trees  
- **System Monitors** with live performance metrics
- **Development Tools** with hot-reload and debugging features

## Getting Started

Install Reactive TUI today:

```bash
# For Node.js/TypeScript projects
npm install reactive-tui

# For Rust projects  
cargo add reactive-tui
```

Check out our [Getting Started Guide](/docs/intro) and explore the [comprehensive examples](https://github.com/entrepeneur4lyf/reactive-tui/tree/main/examples).

## What's Next?

We're just getting started! Upcoming features include:

- **Plugin System** for extensible widgets
- **Advanced Animations** with timeline management  
- **Multi-screen Support** for complex applications
- **WebAssembly Support** for browser compatibility

## Join the Community

- 🌟 [Star us on GitHub](https://github.com/entrepeneur4lyf/reactive-tui)
- 💬 [Join Discussions](https://github.com/entrepeneur4lyf/reactive-tui/discussions)
- 🐛 [Report Issues](https://github.com/entrepeneur4lyf/reactive-tui/issues)
- 📦 [npm Package](https://www.npmjs.com/package/reactive-tui)
- 🦀 [Rust Crate](https://crates.io/crates/reactive-tui)

Welcome to the future of terminal user interfaces! 🚀