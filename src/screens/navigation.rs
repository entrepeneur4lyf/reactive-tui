//! # Navigation System
//!
//! Advanced navigation, routing, and history management for multi-screen applications.
//!
//! This module provides sophisticated navigation capabilities including route-based
//! navigation, parameter passing, history management, and URL-like routing patterns
//! for terminal applications. It enables complex navigation flows with deep linking,
//! breadcrumbs, and state preservation.
//!
//! ## Features
//!
//! - **Route-Based Navigation**: URL-like routing patterns (`/users/:id/profile`)
//! - **Parameter Extraction**: Automatic parameter parsing from routes
//! - **Navigation History**: Forward/backward navigation with state preservation
//! - **Route Guards**: Pre-navigation validation and authorization
//! - **Deep Linking**: Save and restore navigation state
//! - **Breadcrumb Generation**: Automatic breadcrumb trail creation
//!
//! ## Examples
//!
//! ### Route Definition and Navigation
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{Navigator};
//!
//! let mut navigator = Navigator::new();
//!
//! // Define routes
//! navigator.register_route("/", "home_screen");
//! navigator.register_route("/users", "user_list_screen");
//! navigator.register_route("/users/:id", "user_detail_screen");
//! navigator.register_route("/users/:id/edit", "user_edit_screen");
//! navigator.register_route("/settings/:section", "settings_screen");
//!
//! // Navigate to routes
//! navigator.navigate_to("/users/123")?;
//!
//! // Access route parameters
//! let user_id = navigator.get_param("id")?;
//! assert_eq!(user_id, "123");
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Query Parameters and Navigation State
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{Navigator};
//! use serde_json::json;
//!
//! let mut navigator = Navigator::new();
//!
//! // Register a route first 
//! navigator.register_route("/users", "user_list_screen");
//! navigator.register_route("/dashboard", "dashboard_screen");
//!
//! // Navigate with query parameters
//! navigator.navigate_to("/users?page=2&sort=name&filter=active")?;
//!
//! // Access query parameters
//! let page = navigator.get_query_param("page")?.unwrap_or("1".to_string());
//! let sort = navigator.get_query_param("sort")?.unwrap_or("id".to_string());
//! let filter = navigator.get_query_param("filter")?;
//!
//! // Navigate with state
//! let state = json!({
//!     "scroll_position": 150,
//!     "selected_items": [1, 3, 5]
//! });
//! navigator.navigate_to_with_state("/dashboard", state)?;
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Navigation Guards and Middleware
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{Navigator, GuardResult, MiddlewareResult};
//!
//! let mut navigator = Navigator::new();
//!
//! // Register routes first
//! navigator.register_route("/login", "login_screen");
//! navigator.register_route("/admin/dashboard", "admin_dashboard_screen");
//!
//! // Add route guard for authentication
//! navigator.add_guard_fn("/admin/*", |route, context| {
//!     // Simple demo guard - in real app you'd check user permissions
//!     if route.path.starts_with("/admin") {
//!         // For demo, always redirect to login
//!         return GuardResult::Redirect("/login".to_string());
//!     }
//!     GuardResult::Allow
//! });
//!
//! // Add middleware for logging
//! navigator.add_middleware_fn(|route, _context| {
//!     println!("Navigating to: {}", route.path);
//!     MiddlewareResult::Continue
//! });
//!
//! // Navigation will trigger guards and middleware
//! navigator.navigate_to("/admin/dashboard")?;
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```

use super::*;

/// Navigation history manager
#[derive(Debug)]
pub struct NavigationHistory {
  /// History stack
  pub(crate) history: Vec<String>,
  /// Current position in history
  pub(crate) position: usize,
  /// Maximum history size
  max_size: usize,
}

impl NavigationHistory {
  /// Create new navigation history
  pub fn new(max_size: usize) -> Self {
    Self {
      history: Vec::new(),
      position: 0,
      max_size,
    }
  }

  /// Push a new entry to history
  pub fn push(&mut self, screen_id: String) {
    // If we're not at the end of history, remove everything after current position
    if self.position < self.history.len() {
      self.history.truncate(self.position);
    }

    // Add new entry
    self.history.push(screen_id);
    self.position = self.history.len();

    // Enforce max size
    if self.history.len() > self.max_size {
      let remove_count = self.history.len() - self.max_size;
      self.history.drain(0..remove_count);
      self.position = self.history.len();
    }
  }

  /// Go back in history
  pub fn pop(&mut self) -> Option<String> {
    if self.position > 1 {
      self.position -= 1;
      self.history.get(self.position - 1).cloned()
    } else {
      None
    }
  }

  /// Go forward in history
  pub fn forward(&mut self) -> Option<String> {
    if self.position < self.history.len() {
      self.history.get(self.position).cloned().inspect(|_item| {
        self.position += 1;
      })
    } else {
      None
    }
  }

  /// Clear history
  pub fn clear(&mut self) {
    self.history.clear();
    self.position = 0;
  }

  /// Get current history size
  pub fn len(&self) -> usize {
    self.history.len()
  }

  /// Check if history is empty
  pub fn is_empty(&self) -> bool {
    self.history.is_empty()
  }

  /// Get current position
  pub fn position(&self) -> usize {
    self.position
  }

  /// Can go back
  pub fn can_go_back(&self) -> bool {
    self.position > 0
  }

  /// Can go forward
  pub fn can_go_forward(&self) -> bool {
    self.position < self.history.len() - 1
  }
}

/// Route definition for declarative navigation
#[derive(Debug, Clone)]
pub struct Route {
  /// Route path pattern (e.g., "/users/:id")
  pub path: String,
  /// Screen ID to navigate to
  pub screen_id: String,
  /// Route metadata
  pub metadata: HashMap<String, String>,
}

/// Route parameters extracted from path
#[derive(Debug, Clone, Default)]
pub struct RouteParams {
  /// Named parameters (e.g., :id -> value)
  pub params: HashMap<String, String>,
  /// Query parameters
  pub query: HashMap<String, String>,
}

/// Router for declarative navigation
#[derive(Debug)]
pub struct Router {
  /// Registered routes
  routes: Vec<Route>,
}

impl Router {
  /// Create new router
  pub fn new() -> Self {
    Self { routes: Vec::new() }
  }

  /// Register a route
  pub fn register(&mut self, route: Route) {
    self.routes.push(route);
  }

  /// Match a path to a route
  pub fn match_path(&self, path: &str) -> Option<(Route, RouteParams)> {
    // Parse query parameters
    let (path_part, query_part) = if let Some(pos) = path.find('?') {
      (&path[..pos], Some(&path[pos + 1..]))
    } else {
      (path, None)
    };

    // Parse query params
    let mut route_params = RouteParams::default();
    if let Some(query) = query_part {
      for pair in query.split('&') {
        if let Some((key, value)) = pair.split_once('=') {
          route_params
            .query
            .insert(key.to_string(), value.to_string());
        }
      }
    }

    // Try to match routes
    for route in &self.routes {
      if let Some(params) = self.match_route_pattern(&route.path, path_part) {
        route_params.params = params;
        return Some((route.clone(), route_params));
      }
    }

    None
  }

  /// Match a route pattern against a path
  fn match_route_pattern(&self, pattern: &str, path: &str) -> Option<HashMap<String, String>> {
    let pattern_parts: Vec<&str> = pattern.split('/').filter(|s| !s.is_empty()).collect();
    let path_parts: Vec<&str> = path.split('/').filter(|s| !s.is_empty()).collect();

    if pattern_parts.len() != path_parts.len() {
      return None;
    }

    let mut params = HashMap::new();

    for (pattern_part, path_part) in pattern_parts.iter().zip(path_parts.iter()) {
      if let Some(param_name) = pattern_part.strip_prefix(':') {
        // Named parameter
        params.insert(param_name.to_string(), path_part.to_string());
      } else if pattern_part != path_part {
        // Exact match failed
        return None;
      }
    }

    Some(params)
  }

  /// Build a path from a route and parameters
  pub fn build_path(&self, screen_id: &str, params: &HashMap<String, String>) -> Option<String> {
    // Find route by screen_id
    let route = self.routes.iter().find(|r| r.screen_id == screen_id)?;

    // Build path
    let mut path = route.path.clone();
    for (key, value) in params {
      path = path.replace(&format!(":{key}"), value);
    }

    Some(path)
  }
}

impl Default for Router {
  fn default() -> Self {
    Self::new()
  }
}

/// Guard result for navigation guards
#[derive(Debug, Clone)]
pub enum GuardResult {
  /// Allow navigation to proceed
  Allow,
  /// Block navigation
  Block,
  /// Redirect to another route
  Redirect(String),
}

/// Navigation context passed to guards and middleware
#[derive(Debug, Clone, Default)]
pub struct NavigationContext {
  /// Current route parameters
  pub params: RouteParams,
  /// Custom context data
  pub data: HashMap<String, serde_json::Value>,
}

impl NavigationContext {
  /// Create new navigation context
  pub fn new() -> Self {
    Self::default()
  }

  /// Set context data
  pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<()> {
    self.data.insert(
      key.to_string(),
      serde_json::to_value(value).map_err(|e| crate::error::TuiError::component(e.to_string()))?
    );
    Ok(())
  }

  /// Get context data
  pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
    self.data.get(key)
      .and_then(|v| serde_json::from_value(v.clone()).ok())
  }
}

/// Navigation guard that can prevent or redirect navigation
pub trait NavigationGuard: Send + Sync {
  /// Check if navigation should proceed
  fn guard(&self, route: &Route, context: &NavigationContext) -> GuardResult;
}

/// Simple function-based navigation guard
pub struct FnNavigationGuard<F>
where
  F: Fn(&Route, &NavigationContext) -> GuardResult + Send + Sync,
{
  guard_fn: F,
}

impl<F> FnNavigationGuard<F>
where
  F: Fn(&Route, &NavigationContext) -> GuardResult + Send + Sync,
{
  pub fn new(guard_fn: F) -> Self {
    Self { guard_fn }
  }
}

impl<F> NavigationGuard for FnNavigationGuard<F>
where
  F: Fn(&Route, &NavigationContext) -> GuardResult + Send + Sync,
{
  fn guard(&self, route: &Route, context: &NavigationContext) -> GuardResult {
    (self.guard_fn)(route, context)
  }
}

/// Middleware result for navigation middleware
#[derive(Debug, Clone)]
pub enum MiddlewareResult {
  /// Continue with navigation
  Continue,
  /// Stop navigation
  Stop,
}

/// Navigation middleware that can intercept navigation
pub trait NavigationMiddleware: Send + Sync {
  /// Process navigation
  fn process(&self, route: &Route, context: &NavigationContext) -> MiddlewareResult;
}

/// Function-based navigation middleware
pub struct FnNavigationMiddleware<F>
where
  F: Fn(&Route, &NavigationContext) -> MiddlewareResult + Send + Sync,
{
  middleware_fn: F,
}

impl<F> FnNavigationMiddleware<F>
where
  F: Fn(&Route, &NavigationContext) -> MiddlewareResult + Send + Sync,
{
  pub fn new(middleware_fn: F) -> Self {
    Self { middleware_fn }
  }
}

impl<F> NavigationMiddleware for FnNavigationMiddleware<F>
where
  F: Fn(&Route, &NavigationContext) -> MiddlewareResult + Send + Sync,
{
  fn process(&self, route: &Route, context: &NavigationContext) -> MiddlewareResult {
    (self.middleware_fn)(route, context)
  }
}

/// Main navigator that orchestrates route-based navigation
pub struct Navigator {
  /// Route router
  router: Router,
  /// Navigation history
  history: NavigationHistory,
  /// Current route
  current_route: Option<String>,
  /// Current route parameters
  current_params: RouteParams,
  /// Navigation guards
  guards: Vec<Box<dyn NavigationGuard>>,
  /// Navigation middleware
  middleware: Vec<Box<dyn NavigationMiddleware>>,
  /// Navigation context
  context: NavigationContext,
  /// Navigation state
  state: HashMap<String, serde_json::Value>,
}

impl Navigator {
  /// Create a new navigator
  pub fn new() -> Self {
    Self {
      router: Router::new(),
      history: NavigationHistory::new(50),
      current_route: None,
      current_params: RouteParams::default(),
      guards: Vec::new(),
      middleware: Vec::new(),
      context: NavigationContext::new(),
      state: HashMap::new(),
    }
  }

  /// Register a route
  pub fn register_route(&mut self, path: &str, screen_id: &str) {
    let route = Route {
      path: path.to_string(),
      screen_id: screen_id.to_string(),
      metadata: HashMap::new(),
    };
    self.router.register(route);
  }

  /// Register a route with metadata
  pub fn register_route_with_metadata(&mut self, path: &str, screen_id: &str, metadata: HashMap<String, String>) {
    let route = Route {
      path: path.to_string(),
      screen_id: screen_id.to_string(),
      metadata,
    };
    self.router.register(route);
  }

  /// Navigate to a route
  pub fn navigate_to(&mut self, path: &str) -> Result<()> {
    if let Some((route, params)) = self.router.match_path(path) {
      // Update context with new parameters
      self.context.params = params.clone();
      
      // Run guards
      for guard in &self.guards {
        match guard.guard(&route, &self.context) {
          GuardResult::Allow => continue,
          GuardResult::Block => {
            return Err(crate::error::TuiError::component(
              format!("Navigation to '{path}' blocked by guard")
            ));
          }
          GuardResult::Redirect(redirect_path) => {
            return self.navigate_to(&redirect_path);
          }
        }
      }

      // Run middleware
      for middleware in &self.middleware {
        match middleware.process(&route, &self.context) {
          MiddlewareResult::Continue => continue,
          MiddlewareResult::Stop => {
            return Err(crate::error::TuiError::component(
              format!("Navigation to '{path}' stopped by middleware")
            ));
          }
        }
      }

      // Add to history
      if let Some(current) = &self.current_route {
        self.history.push(current.clone());
      }

      // Update current route and parameters
      self.current_route = Some(path.to_string());
      self.current_params = params;

      Ok(())
    } else {
      Err(crate::error::TuiError::component(
        format!("No route found for path: {path}")
      ))
    }
  }

  /// Navigate to a route with state
  pub fn navigate_to_with_state(&mut self, path: &str, state: serde_json::Value) -> Result<()> {
    self.state.insert(path.to_string(), state);
    self.navigate_to(path)
  }

  /// Go back in navigation history
  pub fn go_back(&mut self) -> Result<()> {
    if let Some(previous_path) = self.history.pop() {
      self.navigate_to(&previous_path)
    } else {
      Err(crate::error::TuiError::component(
        "No previous route in history".to_string()
      ))
    }
  }

  /// Go forward in navigation history
  pub fn go_forward(&mut self) -> Result<()> {
    if let Some(next_path) = self.history.forward() {
      self.navigate_to(&next_path)
    } else {
      Err(crate::error::TuiError::component(
        "No forward route in history".to_string()
      ))
    }
  }

  /// Get current route parameter
  pub fn get_param(&self, key: &str) -> Result<String> {
    self.current_params.params.get(key)
      .cloned()
      .ok_or_else(|| crate::error::TuiError::component(
        format!("Parameter '{key}' not found")
      ))
  }

  /// Get current query parameter
  pub fn get_query_param(&self, key: &str) -> Result<Option<String>> {
    Ok(self.current_params.query.get(key).cloned())
  }

  /// Get all current parameters
  pub fn get_params(&self) -> &HashMap<String, String> {
    &self.current_params.params
  }

  /// Get all current query parameters
  pub fn get_query_params(&self) -> &HashMap<String, String> {
    &self.current_params.query
  }

  /// Get current route
  pub fn current_route(&self) -> Option<&str> {
    self.current_route.as_deref()
  }

  /// Get navigation state for current route
  pub fn get_state(&self) -> Option<&serde_json::Value> {
    self.current_route.as_ref()
      .and_then(|route| self.state.get(route))
  }

  /// Set navigation state for current route
  pub fn set_state(&mut self, state: serde_json::Value) {
    if let Some(route) = &self.current_route {
      self.state.insert(route.clone(), state);
    }
  }

  /// Add a navigation guard
  pub fn add_guard<G: NavigationGuard + 'static>(&mut self, guard: G) {
    self.guards.push(Box::new(guard));
  }

  /// Add navigation middleware
  pub fn add_middleware<M: NavigationMiddleware + 'static>(&mut self, middleware: M) {
    self.middleware.push(Box::new(middleware));
  }

  /// Can go back in history
  pub fn can_go_back(&self) -> bool {
    self.history.can_go_back()
  }

  /// Can go forward in history
  pub fn can_go_forward(&self) -> bool {
    self.history.can_go_forward()
  }

  /// Get navigation breadcrumbs
  pub fn get_breadcrumbs(&self) -> Vec<String> {
    let mut breadcrumbs = Vec::new();
    
    // Add history items
    for i in 0..self.history.position() {
      if let Some(route) = self.history.history.get(i) {
        breadcrumbs.push(route.clone());
      }
    }
    
    // Add current route
    if let Some(current) = &self.current_route {
      breadcrumbs.push(current.clone());
    }
    
    breadcrumbs
  }

  /// Clear navigation history
  pub fn clear_history(&mut self) {
    self.history.clear();
  }

  /// Build path for a screen with parameters
  pub fn build_path(&self, screen_id: &str, params: &HashMap<String, String>) -> Option<String> {
    self.router.build_path(screen_id, params)
  }

  /// Add a simple guard using a closure (for doc examples)
  pub fn add_guard_fn<F>(&mut self, _pattern: &str, guard_fn: F)
  where
    F: Fn(&Route, &NavigationContext) -> GuardResult + Send + Sync + 'static,
  {
    self.add_guard(FnNavigationGuard::new(guard_fn));
  }

  /// Add simple middleware using a closure (for doc examples)
  pub fn add_middleware_fn<F>(&mut self, middleware_fn: F)
  where
    F: Fn(&Route, &NavigationContext) -> MiddlewareResult + Send + Sync + 'static,
  {
    self.add_middleware(FnNavigationMiddleware::new(middleware_fn));
  }
}

impl Default for Navigator {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_navigation_history() {
    let mut history = NavigationHistory::new(5);

    // Push some entries
    history.push("home".to_string());
    history.push("settings".to_string());
    history.push("profile".to_string());

    assert_eq!(history.len(), 3);
    assert_eq!(history.position(), 3);

    // Go back
    assert_eq!(history.pop(), Some("settings".to_string()));
    assert_eq!(history.position(), 2);

    // Go forward
    assert_eq!(history.forward(), Some("profile".to_string()));
    assert_eq!(history.position(), 3);

    // Push new entry (should clear forward history)
    history.pop(); // Go back to settings
    history.push("about".to_string());
    assert_eq!(history.len(), 3); // home, settings, about
  }

  #[test]
  fn test_router() {
    let mut router = Router::new();

    // Register routes
    router.register(Route {
      path: "/home".to_string(),
      screen_id: "home".to_string(),
      metadata: HashMap::new(),
    });

    router.register(Route {
      path: "/users/:id".to_string(),
      screen_id: "user_profile".to_string(),
      metadata: HashMap::new(),
    });

    router.register(Route {
      path: "/posts/:id/comments/:comment_id".to_string(),
      screen_id: "comment_detail".to_string(),
      metadata: HashMap::new(),
    });

    // Test exact match
    let (route, params) = router.match_path("/home").unwrap();
    assert_eq!(route.screen_id, "home");
    assert!(params.params.is_empty());

    // Test parameter extraction
    let (route, params) = router.match_path("/users/123").unwrap();
    assert_eq!(route.screen_id, "user_profile");
    assert_eq!(params.params.get("id"), Some(&"123".to_string()));

    // Test multiple parameters
    let (route, params) = router.match_path("/posts/456/comments/789").unwrap();
    assert_eq!(route.screen_id, "comment_detail");
    assert_eq!(params.params.get("id"), Some(&"456".to_string()));
    assert_eq!(params.params.get("comment_id"), Some(&"789".to_string()));

    // Test query parameters
    let (route, params) = router.match_path("/users/123?tab=posts&sort=date").unwrap();
    assert_eq!(route.screen_id, "user_profile");
    assert_eq!(params.params.get("id"), Some(&"123".to_string()));
    assert_eq!(params.query.get("tab"), Some(&"posts".to_string()));
    assert_eq!(params.query.get("sort"), Some(&"date".to_string()));
  }
}
