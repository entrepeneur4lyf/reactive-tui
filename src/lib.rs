//! # TUI Core
//!
//! Revolutionary CSS-styled Terminal User Interface framework.
//!
//! ## Features
//!
//! - **CSS Styling**: Use familiar CSS syntax to style terminal UIs
//! - **High Performance**: Built in Rust with zero-cost abstractions
//! - **Hot Reload**: CSS changes update instantly during development
//! - **Component System**: React-like components with hooks
//! - **Responsive**: Media queries and flexible layouts
//! - **Type Safe**: Full TypeScript support via FFI bindings
//!
//! ## Quick Start
//!
//! ```rust
//! use reactive_tui::prelude::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let app = TuiApp::new()
//!         .stylesheet("styles.css")
//!         .component(|| {
//!             div()
//!                 .class("container")
//!                 .child(text("Hello, CSS-styled TUI!"))
//!         })
//!         .run()
//!         .await?;
//!         
//!     Ok(())
//! }
//! ```

pub mod app;
pub mod components;
pub mod css;
pub mod display;
pub mod driver;
pub mod error;
pub mod events;
pub mod layout;
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
