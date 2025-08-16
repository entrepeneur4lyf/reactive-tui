//! Error Boundary Component
//!
//! Error boundaries are components that catch errors during rendering, in lifecycle methods,
//! and in constructors of the whole tree below them. They provide a way to gracefully handle
//! component failures and prevent them from crashing the entire application.
//!
//! ## Features
//!
//! - **Error Isolation**: Catches errors in child components without affecting the rest of the app
//! - **Fallback UI**: Displays a fallback interface when errors occur
//! - **Error Recovery**: Provides mechanisms to recover from errors and retry operations
//! - **Error Reporting**: Logs detailed error information for debugging
//! - **Graceful Degradation**: Maintains application stability even when components fail
//!
//! ## Usage
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//!
//! let error_boundary = ErrorBoundary::new()
//!     .fallback_content("Something went wrong. Please try again.")
//!     .on_error(|error, info| {
//!         eprintln!("Component error: {} at {}", error, info.component_stack);
//!     })
//!     .retry_button(true)
//!     .build();
//! ```

use crate::components::Element;
use crate::error::{Result, TuiError};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Error information captured when a component fails
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorInfo {
    /// The error message
    pub message: String,
    /// Stack trace of component hierarchy where error occurred
    pub component_stack: Vec<String>,
    /// Timestamp when the error occurred
    pub timestamp: u64,
    /// Additional context about the error
    pub context: HashMap<String, String>,
    /// Error severity level
    pub severity: ErrorSeverity,
}

/// Error severity levels for categorizing failures
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ErrorSeverity {
    /// Low severity - component can continue with degraded functionality
    Warning,
    /// Medium severity - component should show fallback UI
    Error,
    /// High severity - component must be isolated completely
    Critical,
}

/// Error boundary state tracking
#[derive(Debug, Clone)]
pub struct ErrorBoundaryState {
    /// Whether an error has occurred
    pub has_error: bool,
    /// The captured error information
    pub error_info: Option<ErrorInfo>,
    /// Number of retry attempts made
    pub retry_count: u32,
    /// Maximum number of retries allowed
    pub max_retries: u32,
    /// Time of last error occurrence
    pub last_error_time: Option<Instant>,
    /// Cooldown period before allowing retries
    pub retry_cooldown: Duration,
}

impl Default for ErrorBoundaryState {
    fn default() -> Self {
        Self {
            has_error: false,
            error_info: None,
            retry_count: 0,
            max_retries: 3,
            last_error_time: None,
            retry_cooldown: Duration::from_secs(5),
        }
    }
}

/// Error boundary configuration
#[derive(Clone)]
pub struct ErrorBoundaryConfig {
    /// Fallback content to display when error occurs
    pub fallback_content: String,
    /// Whether to show a retry button
    pub show_retry_button: bool,
    /// Custom error handler callback
    pub error_handler: Option<Arc<dyn Fn(&ErrorInfo) + Send + Sync>>,
    /// Maximum number of retries before giving up
    pub max_retries: u32,
    /// Cooldown period between retries
    pub retry_cooldown: Duration,
    /// Whether to log errors to console
    pub log_errors: bool,
    /// Custom CSS classes for error display
    pub error_classes: Vec<String>,
    /// Whether to show error details in UI
    pub show_error_details: bool,
}

impl std::fmt::Debug for ErrorBoundaryConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ErrorBoundaryConfig")
            .field("fallback_content", &self.fallback_content)
            .field("show_retry_button", &self.show_retry_button)
            .field("max_retries", &self.max_retries)
            .field("retry_cooldown", &self.retry_cooldown)
            .field("log_errors", &self.log_errors)
            .field("error_classes", &self.error_classes)
            .field("show_error_details", &self.show_error_details)
            .field("error_handler", &self.error_handler.as_ref().map(|_| "Some(Fn)"))
            .finish()
    }
}

impl Default for ErrorBoundaryConfig {
    fn default() -> Self {
        Self {
            fallback_content: "An error occurred. Please try again.".to_string(),
            show_retry_button: true,
            error_handler: None,
            max_retries: 3,
            retry_cooldown: Duration::from_secs(5),
            log_errors: true,
            error_classes: vec!["error-boundary".to_string()],
            show_error_details: false,
        }
    }
}

/// Error boundary component that catches and handles child component errors
pub struct ErrorBoundary {
    /// Unique identifier for this error boundary
    pub id: String,
    /// Configuration for error handling behavior
    pub config: ErrorBoundaryConfig,
    /// Current state of the error boundary
    pub state: Arc<Mutex<ErrorBoundaryState>>,
    /// Child elements wrapped by this boundary
    pub children: Vec<Element>,
}

impl ErrorBoundary {
    /// Create a new error boundary with default configuration
    pub fn new() -> ErrorBoundaryBuilder {
        ErrorBoundaryBuilder::new()
    }

    /// Create a new error boundary with the given ID
    pub fn with_id<S: Into<String>>(id: S) -> ErrorBoundaryBuilder {
        ErrorBoundaryBuilder::new().id(id)
    }

    /// Check if this boundary currently has an error
    pub fn has_error(&self) -> bool {
        self.state.lock().unwrap().has_error
    }

    /// Get the current error information if any
    pub fn error_info(&self) -> Option<ErrorInfo> {
        self.state.lock().unwrap().error_info.clone()
    }

    /// Catch an error and update the boundary state
    pub fn catch_error(&self, error: TuiError, component_stack: Vec<String>) -> Result<()> {
        let mut state = self.state.lock().unwrap();

        let error_info = ErrorInfo {
            message: error.to_string(),
            component_stack,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            context: HashMap::new(),
            severity: self.classify_error_severity(&error),
        };

        // Log error if enabled
        if self.config.log_errors {
            eprintln!(
                "[ErrorBoundary:{}] Caught error: {} in components: {:?}",
                self.id, error_info.message, error_info.component_stack
            );
        }

        // Call custom error handler if provided
        if let Some(handler) = &self.config.error_handler {
            handler(&error_info);
        }

        // Update state
        state.has_error = true;
        state.error_info = Some(error_info);
        state.last_error_time = Some(Instant::now());

        Ok(())
    }

    /// Attempt to recover from the error state
    pub fn retry(&self) -> Result<bool> {
        let mut state = self.state.lock().unwrap();

        // Check if we've exceeded max retries
        if state.retry_count >= self.config.max_retries {
            return Ok(false);
        }

        // Check cooldown period
        if let Some(last_error) = state.last_error_time {
            if last_error.elapsed() < self.config.retry_cooldown {
                return Ok(false);
            }
        }

        // Reset error state for retry
        state.has_error = false;
        state.error_info = None;
        state.retry_count += 1;
        state.last_error_time = None;

        if self.config.log_errors {
            eprintln!(
                "[ErrorBoundary:{}] Retrying (attempt {}/{})",
                self.id, state.retry_count, self.config.max_retries
            );
        }

        Ok(true)
    }

    /// Reset the error boundary to its initial state
    pub fn reset(&self) {
        let mut state = self.state.lock().unwrap();
        *state = ErrorBoundaryState::default();
        state.max_retries = self.config.max_retries;
        state.retry_cooldown = self.config.retry_cooldown;
    }

    /// Render the error boundary component
    pub fn render(&self) -> Element {
        let state = self.state.lock().unwrap();

        if state.has_error {
            self.render_error_fallback(&state)
        } else {
            self.render_children()
        }
    }

    /// Render the fallback UI when an error occurs
    fn render_error_fallback(&self, state: &ErrorBoundaryState) -> Element {
        let mut container = Element::with_tag("div")
            .id(format!("{}-error", self.id))
            .classes(self.config.error_classes.clone());

        // Main error message
        let error_message = Element::with_tag("div")
            .class("error-message")
            .content(&self.config.fallback_content)
            .build();

        container = container.child(error_message);

        // Show error details if enabled
        if self.config.show_error_details {
            if let Some(error_info) = &state.error_info {
                let details = Element::with_tag("div")
                    .class("error-details")
                    .content(&format!(
                        "Error: {}\nComponent Stack: {:?}\nTime: {}",
                        error_info.message,
                        error_info.component_stack,
                        error_info.timestamp
                    ))
                    .build();

                container = container.child(details);
            }
        }

        // Show retry button if enabled and retries are available
        if self.config.show_retry_button && state.retry_count < self.config.max_retries {
            let can_retry = state.last_error_time
                .map(|t| t.elapsed() >= self.config.retry_cooldown)
                .unwrap_or(true);

            if can_retry {
                let retry_button = Element::with_tag("button")
                    .class("retry-button")
                    .content("Retry")
                    .attr("onclick", &format!("errorBoundary.retry('{}')", self.id))
                    .build();

                container = container.child(retry_button);
            } else {
                let cooldown_message = Element::with_tag("div")
                    .class("cooldown-message")
                    .content(&format!(
                        "Please wait {} seconds before retrying...",
                        self.config.retry_cooldown.as_secs()
                    ))
                    .build();

                container = container.child(cooldown_message);
            }
        }

        container.build()
    }

    /// Render the child components when no error is present
    fn render_children(&self) -> Element {
        let mut container = Element::with_tag("div")
            .id(&self.id)
            .class("error-boundary-container");

        for child in &self.children {
            container = container.child(child.clone());
        }

        container.build()
    }

    /// Classify the severity of an error
    fn classify_error_severity(&self, error: &TuiError) -> ErrorSeverity {
        match error {
            TuiError::CssParseError(_) => ErrorSeverity::Warning,
            TuiError::LayoutError(_) => ErrorSeverity::Error,
            TuiError::RenderError(_) => ErrorSeverity::Error,
            TuiError::ComponentError(_) => ErrorSeverity::Error,
            TuiError::DriverError(_) => ErrorSeverity::Critical,
            TuiError::EventError(_) => ErrorSeverity::Warning,
            TuiError::AnimationError(_) => ErrorSeverity::Warning,
            TuiError::PluginError(_) => ErrorSeverity::Error,
            TuiError::IoError(_) => ErrorSeverity::Critical,
        }
    }
}

/// Builder for creating error boundary components
pub struct ErrorBoundaryBuilder {
    id: Option<String>,
    config: ErrorBoundaryConfig,
    children: Vec<Element>,
}

impl ErrorBoundaryBuilder {
    /// Create a new error boundary builder
    pub fn new() -> Self {
        Self {
            id: None,
            config: ErrorBoundaryConfig::default(),
            children: Vec::new(),
        }
    }

    /// Set the error boundary ID
    pub fn id<S: Into<String>>(mut self, id: S) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Set the fallback content to display when errors occur
    pub fn fallback_content<S: Into<String>>(mut self, content: S) -> Self {
        self.config.fallback_content = content.into();
        self
    }

    /// Set whether to show a retry button
    pub fn show_retry_button(mut self, show: bool) -> Self {
        self.config.show_retry_button = show;
        self
    }

    /// Set a custom error handler
    pub fn on_error<F>(mut self, handler: F) -> Self
    where
        F: Fn(&ErrorInfo) + Send + Sync + 'static,
    {
        self.config.error_handler = Some(Arc::new(handler));
        self
    }

    /// Set the maximum number of retries
    pub fn max_retries(mut self, max: u32) -> Self {
        self.config.max_retries = max;
        self
    }

    /// Set the retry cooldown period
    pub fn retry_cooldown(mut self, duration: Duration) -> Self {
        self.config.retry_cooldown = duration;
        self
    }

    /// Set whether to log errors to console
    pub fn log_errors(mut self, log: bool) -> Self {
        self.config.log_errors = log;
        self
    }

    /// Add CSS classes for error display styling
    pub fn error_classes<S: Into<String>>(mut self, classes: Vec<S>) -> Self {
        self.config.error_classes = classes.into_iter().map(|s| s.into()).collect();
        self
    }

    /// Set whether to show error details in the UI
    pub fn show_error_details(mut self, show: bool) -> Self {
        self.config.show_error_details = show;
        self
    }

    /// Add a child element to be wrapped by the error boundary
    pub fn child(mut self, child: Element) -> Self {
        self.children.push(child);
        self
    }

    /// Add multiple child elements
    pub fn children(mut self, children: Vec<Element>) -> Self {
        self.children.extend(children);
        self
    }

    /// Build the error boundary component
    pub fn build(self) -> ErrorBoundary {
        let id = self.id.unwrap_or_else(|| {
            format!("error-boundary-{}", uuid::Uuid::new_v4().to_string()[..8].to_string())
        });

        let mut state = ErrorBoundaryState::default();
        state.max_retries = self.config.max_retries;
        state.retry_cooldown = self.config.retry_cooldown;

        ErrorBoundary {
            id,
            config: self.config,
            state: Arc::new(Mutex::new(state)),
            children: self.children,
        }
    }
}

impl Default for ErrorBoundaryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Global error boundary registry for managing multiple boundaries
pub struct ErrorBoundaryRegistry {
    boundaries: Arc<Mutex<HashMap<String, Arc<ErrorBoundary>>>>,
}

impl ErrorBoundaryRegistry {
    /// Create a new error boundary registry
    pub fn new() -> Self {
        Self {
            boundaries: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Register an error boundary
    pub fn register(&self, boundary: ErrorBoundary) -> Arc<ErrorBoundary> {
        let boundary = Arc::new(boundary);
        let mut boundaries = self.boundaries.lock().unwrap();
        boundaries.insert(boundary.id.clone(), boundary.clone());
        boundary
    }

    /// Get an error boundary by ID
    pub fn get(&self, id: &str) -> Option<Arc<ErrorBoundary>> {
        let boundaries = self.boundaries.lock().unwrap();
        boundaries.get(id).cloned()
    }

    /// Remove an error boundary
    pub fn remove(&self, id: &str) -> Option<Arc<ErrorBoundary>> {
        let mut boundaries = self.boundaries.lock().unwrap();
        boundaries.remove(id)
    }

    /// Get all registered error boundaries
    pub fn list(&self) -> Vec<Arc<ErrorBoundary>> {
        let boundaries = self.boundaries.lock().unwrap();
        boundaries.values().cloned().collect()
    }

    /// Reset all error boundaries
    pub fn reset_all(&self) {
        let boundaries = self.boundaries.lock().unwrap();
        for boundary in boundaries.values() {
            boundary.reset();
        }
    }

    /// Get error statistics across all boundaries
    pub fn get_error_stats(&self) -> ErrorBoundaryStats {
        let boundaries = self.boundaries.lock().unwrap();
        let mut stats = ErrorBoundaryStats::default();

        for boundary in boundaries.values() {
            let state = boundary.state.lock().unwrap();
            stats.total_boundaries += 1;

            if state.has_error {
                stats.boundaries_with_errors += 1;
            }

            stats.total_retries += state.retry_count;

            if let Some(error_info) = &state.error_info {
                match error_info.severity {
                    ErrorSeverity::Warning => stats.warning_count += 1,
                    ErrorSeverity::Error => stats.error_count += 1,
                    ErrorSeverity::Critical => stats.critical_count += 1,
                }
            }
        }

        stats
    }
}

impl Default for ErrorBoundaryRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Statistics about error boundaries in the application
#[derive(Debug, Clone, Default)]
pub struct ErrorBoundaryStats {
    /// Total number of registered error boundaries
    pub total_boundaries: u32,
    /// Number of boundaries currently in error state
    pub boundaries_with_errors: u32,
    /// Total number of retry attempts across all boundaries
    pub total_retries: u32,
    /// Number of warning-level errors
    pub warning_count: u32,
    /// Number of error-level errors
    pub error_count: u32,
    /// Number of critical-level errors
    pub critical_count: u32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_error_boundary_creation() {
        let boundary = ErrorBoundary::new()
            .id("test-boundary")
            .fallback_content("Test error occurred")
            .max_retries(5)
            .build();

        assert_eq!(boundary.id, "test-boundary");
        assert_eq!(boundary.config.fallback_content, "Test error occurred");
        assert_eq!(boundary.config.max_retries, 5);
        assert!(!boundary.has_error());
    }

    #[test]
    fn test_error_catching() {
        let boundary = ErrorBoundary::new()
            .id("test-boundary")
            .build();

        let error = TuiError::component("Test component error");
        let stack = vec!["ComponentA".to_string(), "ComponentB".to_string()];

        boundary.catch_error(error, stack.clone()).unwrap();

        assert!(boundary.has_error());
        let error_info = boundary.error_info().unwrap();
        assert_eq!(error_info.component_stack, stack);
        assert!(error_info.message.contains("Test component error"));
    }

    #[test]
    fn test_retry_mechanism() {
        let boundary = ErrorBoundary::new()
            .id("test-boundary")
            .max_retries(2)
            .retry_cooldown(Duration::from_millis(10))
            .build();

        // Catch an error
        let error = TuiError::component("Test error");
        boundary.catch_error(error, vec!["TestComponent".to_string()]).unwrap();
        assert!(boundary.has_error());

        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(20));

        // First retry should succeed
        assert!(boundary.retry().unwrap());
        assert!(!boundary.has_error());

        // Catch another error
        let error = TuiError::component("Another test error");
        boundary.catch_error(error, vec!["TestComponent".to_string()]).unwrap();

        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(20));

        // Second retry should succeed
        assert!(boundary.retry().unwrap());

        // Catch another error
        let error = TuiError::component("Third test error");
        boundary.catch_error(error, vec!["TestComponent".to_string()]).unwrap();

        // Wait for cooldown
        std::thread::sleep(Duration::from_millis(20));

        // Third retry should fail (exceeded max retries)
        assert!(!boundary.retry().unwrap());
    }

    #[test]
    fn test_error_boundary_registry() {
        let registry = ErrorBoundaryRegistry::new();

        let boundary1 = ErrorBoundary::new().id("boundary1").build();
        let boundary2 = ErrorBoundary::new().id("boundary2").build();

        let registered1 = registry.register(boundary1);
        let _registered2 = registry.register(boundary2);

        assert_eq!(registry.list().len(), 2);
        assert!(registry.get("boundary1").is_some());
        assert!(registry.get("boundary2").is_some());

        // Test error stats
        let error = TuiError::component("Test error");
        registered1.catch_error(error, vec!["TestComponent".to_string()]).unwrap();

        let stats = registry.get_error_stats();
        assert_eq!(stats.total_boundaries, 2);
        assert_eq!(stats.boundaries_with_errors, 1);
        assert_eq!(stats.error_count, 1);
    }

    #[test]
    fn test_error_severity_classification() {
        let boundary = ErrorBoundary::new().build();

        assert_eq!(
            boundary.classify_error_severity(&TuiError::css_parse("test")),
            ErrorSeverity::Warning
        );
        assert_eq!(
            boundary.classify_error_severity(&TuiError::component("test")),
            ErrorSeverity::Error
        );
        assert_eq!(
            boundary.classify_error_severity(&TuiError::driver("test")),
            ErrorSeverity::Critical
        );
    }
}
