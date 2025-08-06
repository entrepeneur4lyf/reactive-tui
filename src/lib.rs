//! # Reactive TUI
//!
//! Revolutionary CSS-styled Terminal User Interface framework with Rust/TypeScript hybrid architecture.
//!
//! This crate provides a high-performance terminal user interface framework that brings modern web
//! development paradigms to terminal applications. Build beautiful, responsive terminal interfaces
//! using familiar CSS styling and React-like components, all powered by Rust with seamless
//! JavaScript integration.
//!
//! ## Architecture
//!
//! Reactive TUI consists of two main components:
//!
//! 1. **Rust Core (`reactive-tui`)**: High-performance rendering engine with NAPI bindings
//! 2. **TypeScript Layer (`tui-bun`)**: Component system, CSS engine, and developer API
//!
//! ## Key Features
//!
//! ### 🎨 **CSS-First Design**
//! - **Full CSS Support**: Use familiar CSS syntax including flexbox, grid, animations
//! - **Utility Classes**: Tailwind-inspired CSS classes (`flex`, `grid-cols-3`, `p-4`)
//! - **Responsive Design**: Terminal-aware layouts with media queries
//! - **Theme System**: Built-in dark/light themes with custom theme support
//! - **Hot Reload**: Live CSS updates during development
//!
//! ### 🧩 **Rich Widget Library**
//! - **25+ Widgets**: Input, Button, DataTable, Modal, Toast, Progress, Spinner
//! - **Advanced Layouts**: CSS Grid, Flexbox, responsive containers
//! - **Interactive Components**: Forms with validation, autocomplete, menus
//! - **Data Visualization**: Charts, tables with sorting/filtering
//!
//! ### ⚡ **Performance & Integration**
//! - **Native Speed**: Rust-powered rendering with zero-cost abstractions
//! - **NAPI Bindings**: Seamless Node.js/Bun integration
//! - **TypeScript Support**: Full type definitions and IntelliSense
//! - **Cross-Platform**: Windows, macOS, Linux (38 target platforms)
//!
//! ### 🔄 **Reactive State Management**
//! - **Component State**: React-like state management with hooks
//! - **Event System**: Comprehensive keyboard, mouse, and custom events
//! - **Async Support**: Promise-based APIs with async/await patterns
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//!
//! // Create a simple button component
//! #[derive(Debug, Clone)]
//! struct MyApp {
//!     counter: i32,
//! }
//!
//! impl Component for MyApp {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div")
//!             .class("container")
//!             .child(
//!                 Element::with_tag("h1")
//!                     .class("title")
//!                     .content("🚀 Reactive TUI Demo")
//!                     .build()
//!             )
//!             .child(
//!                 Element::with_tag("button")
//!                     .class("btn primary")
//!                     .content(&format!("Clicked {} times", self.counter))
//!                     .build()
//!             )
//!             .build()
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let app = TuiApp::builder()
//!         .component(MyApp { counter: 0 })
//!         .stylesheet("styles.css")
//!         .with_title("My TUI App")
//!         .build()?;
//!
//!     app.run().await
//! }
//! ```
//!
//! ## CSS Styling Example
//!
//! ```css
//! .container {
//!     display: flex;
//!     flex-direction: column;
//!     align-items: center;
//!     padding: 2rem;
//!     background: #1e1e1e;
//!     color: #ffffff;
//! }
//!
//! .title {
//!     font-size: 1.5rem;
//!     font-weight: bold;
//!     margin-bottom: 1rem;
//! }
//!
//! .btn {
//!     padding: 0.5rem 1rem;
//!     border: 1px solid;
//!     margin: 0.25rem;
//! }
//!
//! .btn.primary {
//!     background: #007acc;
//!     color: white;
//!     border-color: #005a9e;
//! }
//!
//! .btn:focus {
//!     outline: 2px solid #ffaa00;
//! }
//! ```
//!
//! ## Advanced Features
//!
//! ### Widget Library
//!
//! Reactive TUI includes a comprehensive widget library:
//!
//! - **Layout Widgets**: Grid, Bar, Tabs, Modal, Accordion
//! - **Form Controls**: Input, Button, Checkbox, Switch, Radio, Select, Slider
//! - **Data Display**: DataTable, Tree, ScrollableList, Progress, Spinner
//! - **Content**: RichText, Textarea, Viewport
//! - **Feedback**: Toast, FormValidator
//! - **Advanced**: Animation, Theme, Plugin system
//!
//! ### Performance Optimization
//!
//! - **Virtual Rendering**: Efficient handling of large datasets (>1000 items)
//! - **Dirty Region Tracking**: Only re-render changed areas
//! - **Frame Buffer**: Double buffering prevents flickering
//! - **Arc/RwLock Patterns**: Minimize lock contention with read-heavy patterns
//!
//! ### NAPI Integration
//!
//! When built with the `ffi` feature, this crate exposes JavaScript bindings:
//!
//! ```javascript
//! const { JsTuiApp, TuiUtils } = require('reactive-tui');
//!
//! const app = new JsTuiApp();
//! app.setTitle('My App');
//! app.loadCss('.btn { background: blue; }');
//!
//! const button = TuiUtils.button();
//! button.setContent('Click me!');
//! app.setComponent(button);
//! ```
//!
//! ## Examples
//!
//! The repository includes 37+ comprehensive examples:
//!
//! ```bash
//! cargo run --example button_demo      # Interactive buttons
//! cargo run --example datatable_demo   # Sortable data tables
//! cargo run --example layout_showcase  # Advanced layouts
//! cargo run --example theme_system_demo # Theming system
//! cargo run --example animation_demo   # Property animations
//! ```
//!
//! ## Error Handling
//!
//! All public APIs use `Result<T, TuiError>` for consistent error handling:
//!
//! ```rust
//! use reactive_tui::prelude::*;
//!
//! fn create_widget() -> Result<Element> {
//!     Ok(Element::with_tag("div")
//!         .class("widget")
//!         .content("Hello World")
//!         .build())
//! }
//! ```
//!
//! ## Development
//!
//! ### Building from Source
//!
//! ```bash
//! # Clone and build
//! git clone https://github.com/entrepeneur4lyf/reactive-tui.git
//! cd reactive-tui
//! cargo build --release
//!
//! # Run examples
//! cargo run --example button_demo
//!
//! # Run tests
//! cargo test
//! cargo test --doc
//! ```
//!
//! ### Feature Flags
//!
//! - `default = ["css"]` - Basic CSS support (always enabled)
//! - `hot-reload` - CSS hot reload during development
//! - `ffi` - NAPI bindings for JavaScript integration
//! - `typescript` - TypeScript type generation
//!
//! ## License
//!
//! This project is dual-licensed:
//! - **Apache License 2.0** for open source use
//! - **Commercial License** for enterprise use
//!
//! See [LICENSE](../LICENSE) and [LICENSE-COMMERCIAL](../LICENSE-COMMERCIAL) for details.

pub mod app;
pub mod compat;
pub mod components;
pub mod css;
pub mod display;
pub mod driver;
pub mod error;
pub mod events;
pub mod integration;
pub mod layout;
pub mod performance;
pub mod plugin;
pub mod reactive;
pub mod rendering;
pub mod screens;
pub mod themes;
pub mod widgets;

// Removed hot_reload and testing modules - they had API compatibility issues

pub mod prelude {
  //! Common imports for TUI applications

  pub use crate::app::{TuiApp, TuiAppBuilder};
  pub use crate::components::{Component, Element, ElementBuilder};
  pub use crate::css::{ComponentNode, ComponentTree, CssEngine, Stylesheet};
  pub use crate::driver::{
    Driver, DriverCapabilities, DriverConfig, DriverEvent, DriverManager, DriverType,
  };
  pub use crate::error::{Result, TuiError};
  pub use crate::events::actions::common as actions;
  pub use crate::events::{
    Action, ActionBuilder, ActionDispatcher, ActionResult, BlurMessage, ClickMessage,
    CustomMessage, Event, EventHandler, FocusManager, FocusMessage, InputMessage, KeyAction,
    KeyBindingManager, KeyBindingResult, KeyCombination, KeyPressMessage, Message, MessageEvent,
    MessageHandler, MessageManager, MountMessage, NavigationDirection, SubmitMessage,
    UnmountMessage,
  };
  pub use crate::layout::advanced_grid::{
    AdvancedGridItem, Grid, GridColumns, GridFlow, GridGap, GridLayout as AdvancedGridLayout,
    GridRows,
  };
  pub use crate::layout::grid::{GridCell, GridConfig, GridLayout as BasicGridLayout, GridScalar};
  pub use crate::layout::grid_debug::{
    debug_grid_overlay, inspect_grid, interactive_grid_inspector, GridDebugColors, GridDebugConfig,
    GridDebugMode, GridDebugger, GridInspectionReport, GridItemReport, GridStatistics, GridWarning,
    GridWarningType,
  };
  pub use crate::layout::{
    AlignItems, ComputedStyles, DisplayType, FlexDirection, JustifyContent, Layout, LayoutEngine,
    LayoutRect, SizeValue, Spacing,
  };
  pub use crate::performance::{PerformanceMonitor, PerformanceMetrics, PerformanceReport};
  pub use crate::reactive::{Reactive, ReactiveComponent, ReactiveState, ReactiveStruct};
  pub use crate::rendering::{RenderStyle, Renderer};
  pub use crate::themes::{
    color_to_ansi, create_variant, default_utility_palette, generate_utility_classes,
    get_border_set, get_semantic_background, get_semantic_color, hex,
    load_theme_collection_from_file, load_theme_from_file, rgb, BorderSet, BorderStyle,
    ColorDefinition, ColorMode, ColorPalette, ColorSupport, ColorTheme, SemanticColorMapping,
    UtilityClasses, UtilityPalette, UtilityProcessor, RESET_COLOR,
  };
  pub use crate::widgets::{
    bottom_tabs, card_tabs, footer_bar, header_bar, horizontal_tabs, minimal_tabs, navigation_bar,
    spinners, status_bar, toolbar, vertical_tabs, Bar, BarBorderStyle, BarBuilder, BarItem,
    BarPosition, BarSize, BarStyle, BarType, Button, ButtonBorderStyle, ButtonBuilder, ButtonSize,
    ButtonState, ButtonStyle, ButtonType, CursorPosition, EasingFunction, EditOperation, History,
    IconPosition, Input, InputBuilder, InputState, InputStyle, InputType, OverlayManager,
    OverlayPosition, OverlayStyle, ProgressAnimation, ProgressBar, ProgressBarBuilder,
    ProgressColors, ProgressManager, ProgressMessage, ProgressState, ProgressStyle, Search,
    Selection, Slider, SliderBuilder, SliderMode, SliderOrientation, SliderState, SliderStyle,
    SliderTicks, Tab, TabBorderStyle, TabOrientation, TabPosition, TabSize, TabStyle, Tabs,
    TabsBuilder, Textarea, TextareaBuilder, TextareaState, TextareaStyle, Toast, ToastBuilder,
    ToastManager, ToastVariant, ValidationState, Viewport, YankBuffer,
  };
}

// FFI module for NAPI bindings - only when ffi feature is enabled
#[cfg(feature = "ffi")]
pub mod ffi;

// Re-export all NAPI bindings when ffi feature is enabled
#[cfg(feature = "ffi")]
pub use ffi::*;

// NAPI module initialization
#[cfg(feature = "ffi")]
#[napi_derive::module_init]
fn init() {
  // Module initialization if needed
}
