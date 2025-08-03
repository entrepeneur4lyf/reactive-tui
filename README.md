| <img src="assets/logo.jpg" alt="Reactive TUI Logo" width="100" height="100"> | **Revolutionary CSS-Styled Terminal User Interface Framework**<br><br>[![Crates.io](https://img.shields.io/crates/v/reactive-tui.svg)](https://crates.io/crates/reactive-tui) [![NPM Version](https://img.shields.io/npm/v/reactive-tui.svg)](https://www.npmjs.com/package/reactive-tui) [![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)<br>[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/) [![TypeScript](https://img.shields.io/badge/TypeScript-Ready-blue.svg)](https://www.typescriptlang.org/) [![Build Status](https://img.shields.io/github/actions/workflow/status/entrepeneur4lyf/reactive-tui/ci.yml?branch=main)](https://github.com/entrepeneur4lyf/reactive-tui/actions)<br><br>*This is not just another ratatui. This is the future of terminal UI development.* |
|---|---|

## 🚀 Why Reactive TUI?

Reactive TUI revolutionizes terminal application development by bringing **modern web paradigms** to the terminal. Unlike traditional TUI frameworks that require complex manual positioning and styling, Reactive TUI provides:

- **🎨 Full CSS Support** - Flexbox, Grid, animations, responsive design
- **🏭 WidgetFactory Pattern** - Zero-boilerplate widget creation with type safety
- **⚡ Dual Architecture** - High-performance Rust core + JavaScript/TypeScript bindings
- **📱 Responsive Design** - Terminal-aware layouts that adapt to window size
- **🎯 25+ Pre-built Widgets** - Professional UI components out of the box

**Traditional TUI** ❌:
```rust
// Complex manual positioning and styling
let mut rect = layout[0];
rect.x += 2;
rect.y += 1;
rect.width -= 4;
rect.height -= 2;
// ... pages of manual layout code
```

**Reactive TUI** ✅:
```rust
// Modern, declarative approach
let button = button("save-btn", |config| {
    config.text("Save File")
          .variant("primary")
          .class("w-full")
          .on_click("save_action")
});
```

## 📦 Installation

### Rust Crate
```toml
[dependencies]
reactive-tui = "0.0.2"
```

### TypeScript SDK (Recommended)
```bash
npm install reactive-tui-ts
```

### NPM Package (Core Bindings)
```bash
npm install reactive-tui
```

## ⚡ Quick Start

### Rust
```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

fn main() -> Result<()> {
    let app = TuiApp::new("My App");
    
    // Create widgets with the WidgetFactory pattern
    let header = button("header-btn", |c| {
        c.text("🚀 Reactive TUI Demo")
         .variant("primary")
         .class("w-full p-2")
    });
    
    let form = input("username", |c| {
        c.placeholder("Enter username")
         .required(true)
         .class("border rounded")
    });
    
    let modal = modal("confirm", |c| {
        c.title("Confirm Action")
         .confirm("Save changes?", "Yes", "No")
         .class("modal-center")
    });
    
    // CSS-styled layout
    app.load_css(r#"
        .container {
            display: flex;
            flex-direction: column;
            gap: 1rem;
            padding: 2rem;
            height: 100vh;
        }
        .w-full { width: 100%; }
        .border { border: 1px solid #ccc; }
        .rounded { border-radius: 4px; }
    "#);
    
    app.run()
}
```

### TypeScript SDK
```typescript
import { createApp, button, input, modal } from 'reactive-tui-ts';

const app = createApp({
  title: 'My TUI App',
  css: `
    .container { 
      display: flex; 
      gap: 1rem; 
      padding: 2rem; 
    }
  `,
  component: () =>
    button('main-btn', config => 
      config.text('Click Me!')
            .variant('success')
            .class('btn-large')
    )
});

await app.run();
```

### JavaScript (Core Bindings)
```javascript
const { JsTuiApp, TuiUtils } = require('reactive-tui');

const app = new JsTuiApp();
app.setTitle('My TUI App');

const button = TuiUtils.button('main-btn');
button.setText('Click Me!');
button.setVariant('success');

app.setComponent(button);
app.start();
```

## 🎯 Key Features

### 🏭 WidgetFactory Pattern
Eliminate boilerplate with our revolutionary factory pattern:

```rust
// Old way - verbose and error-prone
let mut button = Button::new("my-button");
button.set_text("Click Me");
button.set_variant(ButtonVariant::Primary);
button.add_css_class("btn-large");
button.set_click_handler(|_| { /* handler */ });

// New way - concise and type-safe
let button = button("my-button", |c| {
    c.text("Click Me")
     .variant("primary") 
     .class("btn-large")
     .on_click("handle_click")
});
```

### 🎨 Full CSS Engine
Real CSS that works in the terminal:

```css
.dashboard {
  display: grid;
  grid-template-areas: 
    "header header"
    "sidebar main";
  grid-template-columns: 200px 1fr;
  gap: 1rem;
  height: 100vh;
}

@media (max-width: 80ch) {
  .dashboard {
    grid-template-areas: 
      "header"
      "main";
    grid-template-columns: 1fr;
  }
}
```

### ⚡ Performance
- **186 Unit Tests** + **71 Doc Tests** - 100% passing
- **Virtual Rendering** - Only updates changed regions
- **Memory Efficient** - Rust-powered with minimal overhead
- **Cross-Platform** - 38+ target platforms supported

## 🧩 Widget Library

**Form Controls**: Button, Input, Checkbox, Radio, Select, Slider, Switch  
**Layout**: Grid, Flexbox, Tabs, Modal, Accordion, Bar  
**Data**: DataTable, Tree, List, Progress, Spinner  
**Content**: RichText, Textarea, Viewport  
**Advanced**: Animation, Toast, FormValidator, Plugin System  

All widgets support:
- ✅ WidgetFactory pattern for zero boilerplate
- ✅ CSS styling with utility classes
- ✅ Reactive state management
- ✅ Event handling and validation
- ✅ Theme system integration

## 📖 Documentation

- **[Rust API Docs](https://docs.rs/reactive-tui)** - Complete Rust crate reference
- **[TypeScript SDK](https://github.com/entrepeneur4lyf/reactive-tui-ts)** - Enhanced TypeScript package
- **[Widget Gallery](https://reactive-tui.com/widgets)** - Interactive widget showcase  
- **[CSS Guide](https://reactive-tui.com/css)** - Complete CSS implementation
- **[Examples](examples/)** - 37+ comprehensive examples
- **[Migration Guide](docs/MIGRATION.md)** - From other TUI frameworks

## 🎮 Examples

```bash
# Widget showcase
cargo run --example widget_showcase

# Complex dashboard
cargo run --example dashboard_demo

# Form validation
cargo run --example form_demo

# Real-time data
cargo run --example data_streaming
```

## 🏗️ Architecture

```
┌──────────────────────┐    ┌─────────────────────┐    ┌─────────────────────┐
│   TypeScript SDK     │    │    NPM Package      │    │     Rust Crate     │
│   reactive-tui-ts    │    │   reactive-tui      │    │   reactive-tui      │
│   ────────────────   │    │   ──────────────    │    │   ──────────────    │
│   • Enhanced APIs    │    │   • NAPI Bindings   │    │   • Core Engine     │
│   • Widget Factory   │◄──►│   • FFI Layer       │◄──►│   • Layout System   │
│   • Type Definitions │    │   • Memory Bridge   │    │   • Widget Library  │
│   • Developer Tools  │    │   • Event Bridge    │    │   • CSS Engine      │
└──────────────────────┘    └─────────────────────┘    └─────────────────────┘
```

**Three-Package Architecture:**
- **`reactive-tui`** (Rust) - High-performance core engine and widget system
- **`reactive-tui`** (NPM) - NAPI-rs bindings for JavaScript integration  
- **`reactive-tui-ts`** (NPM) - Enhanced TypeScript SDK with developer experience features  

## 🚀 Why Not Just Use Ratatui?

| Feature | Reactive TUI | Ratatui | Others |
|---------|--------------|---------|---------|
| **CSS Styling** | ✅ Full CSS with Flexbox/Grid | ❌ Manual layout only | ❌ Limited styling |
| **WidgetFactory** | ✅ Zero-boilerplate creation | ❌ Verbose widget setup | ❌ Complex APIs |
| **TypeScript Support** | ✅ Dedicated TS SDK + bindings | ❌ Rust only | ❌ Limited languages |
| **Responsive Design** | ✅ CSS media queries | ❌ Manual breakpoints | ❌ Fixed layouts |
| **State Management** | ✅ Reactive with JSON | ❌ Manual state handling | ❌ Basic state |
| **Developer Experience** | ✅ 3-tier architecture | ❌ Single-language | ❌ Limited tooling |

## 🎯 Roadmap

- 🔄 **Hot Reload** - Live CSS and component updates
- 🌐 **Web Export** - Compile TUI apps to WebAssembly
- 📱 **Mobile Support** - React Native-style mobile TUI
- 🎨 **Visual Designer** - Drag-and-drop TUI builder
- 🔌 **Plugin Ecosystem** - Community widget marketplace

## 🤝 Contributing

We welcome contributions! Check out our [Contributing Guide](CONTRIBUTING.md).

- 🐛 **Bug Reports**: [GitHub Issues](https://github.com/entrepeneur4lyf/reactive-tui/issues)
- 💡 **Feature Requests**: [GitHub Discussions](https://github.com/entrepeneur4lyf/reactive-tui/discussions)  
- 📖 **Documentation**: Help improve our docs
- 🧩 **Widgets**: Create new widgets for the community

## 📄 License

Apache-2.0 License - see [LICENSE](LICENSE) for details.

---

<div align="center">

**Made with ❤️ by the Reactive TUI team - Shawn McAllister [@entrepreneur4lyf](https://x.com/entrepeneur4lyf) and [Claude Code](https://www.anthropic.com/claude-code) w/ [@claudeai](https://x.com/claudeai)**

[Website](https://reactive-tui.com) • [Documentation](https://docs.rs/reactive-tui) • [Examples](examples/) • [Discord](https://discord.gg/reactive-tui)

*Star us if you find Reactive TUI useful! ⭐*

</div>
