//! # Error Handling
//!
//! Comprehensive error types and utilities for Reactive TUI applications.
//!
//! This module provides the [`TuiError`] enum and [`Result`] type alias used throughout
//! the framework for consistent error handling. All public APIs return `Result<T, TuiError>`
//! to ensure robust error management.
//!
//! ## Error Categories
//!
//! - **CSS Errors**: Stylesheet parsing and rule validation failures
//! - **Layout Errors**: Flexbox, grid, and positioning computation issues
//! - **Rendering Errors**: Terminal drawing and output problems
//! - **Component Errors**: Widget creation and lifecycle issues
//! - **Driver Errors**: Terminal backend and I/O failures
//! - **Event Errors**: Input processing and event handling problems
//! - **Animation Errors**: Animation timing and interpolation issues
//! - **Plugin Errors**: Plugin loading and execution failures
//!
//! ## Examples
//!
//! ### Error Handling in Components
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//!
//! fn create_button() -> Result<Element> {
//!     Ok(Element::with_tag("button")
//!         .class("primary")
//!         .content("Click me")
//!         .build())
//! }
//! ```
//!
//! ### Error Propagation
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//!
//! #[derive(Clone)]
//! struct MyComponent;
//!
//! impl Component for MyComponent {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div").content("Hello").build()
//!     }
//! }
//!
//! fn setup_app() -> Result<TuiApp> {
//!     let app = TuiApp::builder()
//!         .stylesheet("main.css")
//!         .component(MyComponent)
//!         .build()?;
//!
//!     Ok(app)
//! }
//! ```

use thiserror::Error;

/// Type alias for `Result<T, TuiError>` used throughout the framework.
///
/// This provides a convenient shorthand for functions that return TUI-related errors.
/// All public APIs use this type for consistent error handling.
///
/// # Examples
///
/// ```rust,no_run
/// use reactive_tui::prelude::*;
///
/// fn parse_css(input: &str) -> Result<()> {
///     // CSS parsing logic
///     Ok(())
/// }
/// ```
pub type Result<T> = std::result::Result<T, TuiError>;

/// # TUI Error Types
///
/// Comprehensive error enumeration covering all failure modes in Reactive TUI applications.
///
/// `TuiError` provides specific error variants for different subsystems, enabling precise
/// error handling and debugging. Each variant includes contextual information about the
/// failure to aid in troubleshooting.
///
/// ## Error Variants
///
/// - [`CssParseError`](TuiError::CssParseError): Malformed CSS syntax or invalid rules
/// - [`LayoutError`](TuiError::LayoutError): Layout computation failures (flexbox, grid)
/// - [`RenderError`](TuiError::RenderError): Terminal rendering and drawing issues
/// - [`ComponentError`](TuiError::ComponentError): Widget creation and lifecycle problems
/// - [`DriverError`](TuiError::DriverError): Terminal backend and I/O failures
/// - [`EventError`](TuiError::EventError): Input processing and event handling issues
/// - [`AnimationError`](TuiError::AnimationError): Animation timing and interpolation failures
/// - [`PluginError`](TuiError::PluginError): Plugin loading and execution problems
/// - [`IoError`](TuiError::IoError): File system and I/O operations
///
/// ## Usage Patterns
///
/// ### Custom Error Creation
///
/// ```rust,no_run
/// use reactive_tui::prelude::*;
///
/// fn validate_component() -> Result<()> {
///     let invalid_condition = false; // Example condition
///     if invalid_condition {
///         return Err(TuiError::component("Invalid component configuration"));
///     }
///     Ok(())
/// }
/// ```
///
/// ### Error Matching
///
/// ```rust,no_run
/// use reactive_tui::prelude::*;
///
/// #[tokio::main]
/// async fn main() {
///     let app = TuiApp::builder().build().unwrap();
///     match app.run().await {
///         Ok(_) => println!("App completed successfully"),
///         Err(TuiError::CssParseError(msg)) => eprintln!("CSS error: {}", msg),
///         Err(TuiError::DriverError(msg)) => eprintln!("Terminal error: {}", msg),
///         Err(e) => eprintln!("Other error: {}", e),
///     }
/// }
/// ```
#[derive(Error, Debug)]
pub enum TuiError {
  /// CSS parsing and validation failures.
  ///
  /// Occurs when stylesheet parsing fails due to invalid syntax, unsupported
  /// properties, or malformed selectors.
  #[error("CSS parsing error: {0}")]
  CssParseError(String),

  /// Layout computation and positioning failures.
  ///
  /// Triggered by issues in flexbox, grid, or absolute positioning calculations,
  /// often due to conflicting constraints or invalid dimensions.
  #[error("Layout error: {0}")]
  LayoutError(String),

  /// Terminal rendering and drawing failures.
  ///
  /// Occurs when terminal output operations fail, such as cursor positioning,
  /// color rendering, or screen buffer management.
  #[error("Rendering error: {0}")]
  RenderError(String),

  /// File system and I/O operation failures.
  ///
  /// Automatically converted from `std::io::Error` for file operations like
  /// reading stylesheets or writing output.
  #[error("IO error: {0}")]
  IoError(#[from] std::io::Error),

  /// Component creation and lifecycle failures.
  ///
  /// Covers widget instantiation errors, invalid properties, or component
  /// tree construction issues.
  #[error("Component error: {0}")]
  ComponentError(String),

  /// Animation timing and interpolation failures.
  ///
  /// Occurs during property animation setup, easing function application,
  /// or animation timeline management.
  #[error("Animation error: {0}")]
  AnimationError(String),

  /// Input processing and event handling failures.
  ///
  /// Triggered by keyboard/mouse input processing errors, event routing
  /// failures, or custom event handling issues.
  #[error("Event handling error: {0}")]
  EventError(String),

  /// Terminal driver and backend failures.
  ///
  /// Covers terminal initialization, capability detection, and low-level
  /// terminal operation failures.
  #[error("Driver error: {0}")]
  DriverError(String),

  /// Plugin loading and execution failures.
  ///
  /// Occurs during plugin system operations, including loading, initialization,
  /// or runtime execution errors.
  #[error("Plugin error: {0}")]
  PluginError(String),
}

impl TuiError {
  /// Creates a new CSS parsing error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the CSS parsing failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::css_parse("Invalid selector syntax");
  /// ```
  pub fn css_parse<S: Into<String>>(msg: S) -> Self {
    Self::CssParseError(msg.into())
  }

  /// Creates a new layout error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the layout computation failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::layout("Flexbox constraint conflict");
  /// ```
  pub fn layout<S: Into<String>>(msg: S) -> Self {
    Self::LayoutError(msg.into())
  }

  /// Creates a new rendering error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the rendering failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::render("Failed to draw to terminal buffer");
  /// ```
  pub fn render<S: Into<String>>(msg: S) -> Self {
    Self::RenderError(msg.into())
  }

  /// Creates a new component error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the component failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::component("Widget initialization failed");
  /// ```
  pub fn component<S: Into<String>>(msg: S) -> Self {
    Self::ComponentError(msg.into())
  }

  /// Creates a new driver error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the driver failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::driver("Terminal initialization failed");
  /// ```
  pub fn driver<S: Into<String>>(msg: S) -> Self {
    Self::DriverError(msg.into())
  }

  /// Creates a new plugin error with the provided message.
  ///
  /// # Arguments
  ///
  /// * `msg` - Error message describing the plugin failure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::error::TuiError;
  ///
  /// let error = TuiError::plugin("Plugin loading failed");
  /// ```
  pub fn plugin<S: Into<String>>(msg: S) -> Self {
    Self::PluginError(msg.into())
  }
}

// Allow conversion from widget factory errors
impl From<crate::widgets::factory::WidgetFactoryError> for TuiError {
  fn from(err: crate::widgets::factory::WidgetFactoryError) -> Self {
    TuiError::ComponentError(err.to_string())
  }
}

// Allow conversion from std::fmt::Error
impl From<std::fmt::Error> for TuiError {
  fn from(err: std::fmt::Error) -> Self {
    TuiError::RenderError(format!("Formatting error: {}", err))
  }
}
