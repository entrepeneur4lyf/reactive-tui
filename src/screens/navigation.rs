/*!
 * Navigation history and routing
 */

use super::*;

/// Navigation history manager
#[derive(Debug)]
pub struct NavigationHistory {
    /// History stack
    history: Vec<String>,
    /// Current position in history
    position: usize,
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
        if self.position > 0 {
            self.position -= 1;
            self.history.get(self.position).cloned()
        } else {
            None
        }
    }
    
    /// Go forward in history
    pub fn forward(&mut self) -> Option<String> {
        if self.position < self.history.len() - 1 {
            self.position += 1;
            self.history.get(self.position).cloned()
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
        Self {
            routes: Vec::new(),
        }
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
                    route_params.query.insert(key.to_string(), value.to_string());
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
            if pattern_part.starts_with(':') {
                // Named parameter
                let param_name = &pattern_part[1..];
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
            path = path.replace(&format!(":{}", key), value);
        }
        
        Some(path)
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

/// Navigation guard that can prevent navigation
pub trait NavigationGuard: Send + Sync {
    /// Check if navigation should proceed
    fn can_navigate(&self, from: Option<&str>, to: &str, params: &RouteParams) -> bool;
}

/// Simple function-based navigation guard
pub struct FnNavigationGuard<F>
where
    F: Fn(Option<&str>, &str, &RouteParams) -> bool + Send + Sync,
{
    guard_fn: F,
}

impl<F> FnNavigationGuard<F>
where
    F: Fn(Option<&str>, &str, &RouteParams) -> bool + Send + Sync,
{
    pub fn new(guard_fn: F) -> Self {
        Self { guard_fn }
    }
}

impl<F> NavigationGuard for FnNavigationGuard<F>
where
    F: Fn(Option<&str>, &str, &RouteParams) -> bool + Send + Sync,
{
    fn can_navigate(&self, from: Option<&str>, to: &str, params: &RouteParams) -> bool {
        (self.guard_fn)(from, to, params)
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