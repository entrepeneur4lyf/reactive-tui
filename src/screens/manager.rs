//! # Screen Manager
//!
//! Central orchestration for multi-screen navigation and lifecycle management.
//!
//! The [`ScreenManager`] provides the core functionality for managing multiple screens
//! in a terminal application, handling navigation, state transitions, and screen
//! lifecycle events. It maintains the screen stack, manages transitions, and provides
//! navigation history and breadcrumb support.
//!
//! ## Features
//!
//! - **Screen Stack**: Push/pop navigation with history
//! - **Lifecycle Management**: Automatic mount/unmount/activate/deactivate calls
//! - **Navigation History**: Back/forward navigation with breadcrumbs
//! - **State Persistence**: Optional screen state preservation
//! - **Transition Animations**: Smooth screen transitions
//! - **Event Routing**: Screen-specific event handling
//!
//! ## Examples
//!
//! ### Basic Screen Management
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{ScreenManager, ScreenManagerConfig, SimpleScreen, NavigationOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = ScreenManagerConfig::default();
//!     let manager = ScreenManager::new(config);
//!
//!     // Register screens with the manager
//!     manager.register(Box::new(SimpleScreen::new("main_menu", "Main Menu", |_| {
//!         Element::with_tag("div").content("Main Menu Screen").build()
//!     }))).await?;
//!
//!     // Navigate to the main menu
//!     manager.navigate_to("main_menu", NavigationOptions::default()).await?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ### Screen Navigation with Parameters
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{ScreenManager, ScreenManagerConfig};
//! use serde_json::json;
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = ScreenManagerConfig::default();
//!     let manager = ScreenManager::new(config);
//!
//!     // Navigate with parameters
//!     let params = json!({
//!         "user_id": 123,
//!         "tab": "profile"
//!     });
//!
//!     manager.navigate_to_with_params("user_profile", params).await?;
//!
//!     // Access current screen
//!     if let Some(current_screen_id) = manager.current_screen() {
//!         println!("Current screen: {}", current_screen_id);
//!     }
//!     
//!     Ok(())
//! }
//! ```
//!
//! ### Navigation History and Breadcrumbs
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{ScreenManager, ScreenManagerConfig, SimpleScreen, NavigationOptions};
//!
//! #[tokio::main]
//! async fn main() -> Result<()> {
//!     let config = ScreenManagerConfig::default();
//!     let manager = ScreenManager::new(config);
//!
//!     // Register some demo screens first
//!     manager.register(Box::new(SimpleScreen::new("home", "Home", |_| {
//!         Element::with_tag("div").content("Home Screen").build()
//!     }))).await?;
//!     manager.register(Box::new(SimpleScreen::new("catalog", "Catalog", |_| {
//!         Element::with_tag("div").content("Catalog Screen").build()
//!     }))).await?;
//!     manager.register(Box::new(SimpleScreen::new("product_detail", "Product Detail", |_| {
//!         Element::with_tag("div").content("Product Detail Screen").build()
//!     }))).await?;
//!
//!     // Build navigation history
//!     manager.navigate_to("home", NavigationOptions::default()).await?;
//!     manager.push_screen("catalog").await?;
//!     manager.push_screen("product_detail").await?;
//!
//!     // Get breadcrumb trail
//!     let breadcrumbs = manager.get_breadcrumbs();
//!     println!("Breadcrumbs: {:?}", breadcrumbs);
//!
//!     // Navigate back in history
//!     manager.go_back().await?; // Now at "catalog"
//!     manager.go_forward().await?; // Back to "product_detail"
//!
//!     // Check navigation state
//!     assert!(manager.can_go_back());
//!     assert!(!manager.can_go_forward());
//!     
//!     Ok(())
//! }
//! ```

use super::*;
use crate::{
  components::{div, text, Component, Element},
  error::{Result, TuiError},
  events::{ActionResult, Event},
};
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

/// Navigation history manager
#[derive(Debug)]
pub struct NavigationHistory {
  history: Vec<String>,
  current_index: isize,
  max_size: usize,
}

impl NavigationHistory {
  /// Create a new navigation history
  pub fn new(max_size: usize) -> Self {
    Self {
      history: Vec::new(),
      current_index: -1,
      max_size,
    }
  }

  /// Push a new screen to history
  pub fn push(&mut self, screen_id: String) {
    // Remove any history after current index if we're in the middle
    if self.current_index >= 0 {
      let truncate_at = (self.current_index + 1) as usize;
      if truncate_at < self.history.len() {
        self.history.truncate(truncate_at);
      }
    }

    // Add new screen
    self.history.push(screen_id);
    self.current_index = self.history.len() as isize - 1;

    // Trim history if too large
    if self.history.len() > self.max_size {
      self.history.remove(0);
      self.current_index -= 1;
    }
  }

  /// Pop the last screen from history
  pub fn pop(&mut self) -> Option<String> {
    if self.current_index > 0 {
      self.current_index -= 1;
      self.history.get(self.current_index as usize).cloned()
    } else {
      None
    }
  }

  /// Move forward in history
  pub fn forward(&mut self) -> Option<String> {
    if self.current_index + 1 < self.history.len() as isize {
      self.current_index += 1;
      self.history.get(self.current_index as usize).cloned()
    } else {
      None
    }
  }

  /// Get current history as breadcrumbs
  pub fn breadcrumbs(&self) -> Vec<String> {
    if self.current_index >= 0 {
      let end_index = (self.current_index + 1) as usize;
      self.history[..end_index].to_vec()
    } else {
      Vec::new()
    }
  }

  /// Check if we can go back
  pub fn can_go_back(&self) -> bool {
    self.current_index > 0
  }

  /// Check if we can go forward
  pub fn can_go_forward(&self) -> bool {
    self.current_index + 1 < self.history.len() as isize
  }
}

/// Transition manager for screen transitions
#[derive(Debug)]
pub struct TransitionManager {
  current_transition: Option<(TransitionType, u32)>,
  start_time: Option<std::time::Instant>,
}

impl TransitionManager {
  /// Create a new transition manager
  pub fn new() -> Self {
    Self {
      current_transition: None,
      start_time: None,
    }
  }

  /// Start a transition
  pub fn start_transition(&mut self, transition_type: TransitionType, duration: u32) {
    self.current_transition = Some((transition_type, duration));
    self.start_time = Some(std::time::Instant::now());
  }

  /// Check if a transition is in progress
  pub fn is_transitioning(&self) -> bool {
    if let (Some((_, duration)), Some(start_time)) = (&self.current_transition, &self.start_time) {
      start_time.elapsed().as_millis() < *duration as u128
    } else {
      false
    }
  }

  /// Render transition placeholder if transitioning
  pub fn render_placeholder(&self, screen_id: &str) -> Option<Element> {
    if self.is_transitioning() {
      Some(
        div()
          .class("screen-transition")
          .child(text(format!("Transitioning to {screen_id}...")).build())
          .build(),
      )
    } else {
      None
    }
  }

  /// Complete the current transition
  #[allow(dead_code)]
  pub fn complete_transition(&mut self) {
    self.current_transition = None;
    self.start_time = None;
  }
}

/// Screen instance with state
struct ScreenInstance {
  screen: Box<dyn Screen>,
  state: ScreenState,
  mounted: bool,
  visible: bool,
}

/// Screen Manager handles multiple screens and navigation
pub struct ScreenManager {
  /// Registered screens
  screens: Arc<TokioRwLock<HashMap<String, ScreenInstance>>>,
  /// Current active screen ID
  current_screen: Arc<RwLock<Option<String>>>,
  /// Navigation history
  history: Arc<RwLock<NavigationHistory>>,
  /// Active workspaces
  workspaces: Arc<RwLock<HashMap<String, Workspace>>>,
  /// Current workspace ID
  current_workspace: Arc<RwLock<String>>,
  /// Configuration
  config: ScreenManagerConfig,
  /// Transition manager
  transition_manager: Arc<RwLock<TransitionManager>>,
}

impl ScreenManager {
  /// Create a new screen manager
  pub fn new(config: ScreenManagerConfig) -> Self {
    let default_workspace = Workspace::new("main", "Main");
    let mut workspaces = HashMap::new();
    workspaces.insert("main".to_string(), default_workspace);

    Self {
      screens: Arc::new(TokioRwLock::new(HashMap::new())),
      current_screen: Arc::new(RwLock::new(None)),
      history: Arc::new(RwLock::new(NavigationHistory::new(config.max_history_size))),
      workspaces: Arc::new(RwLock::new(workspaces)),
      current_workspace: Arc::new(RwLock::new("main".to_string())),
      config,
      transition_manager: Arc::new(RwLock::new(TransitionManager::new())),
    }
  }

  /// Register a screen
  pub async fn register(&self, screen: Box<dyn Screen>) -> Result<()> {
    let config = screen.config();
    let id = config.id.clone();

    let instance = ScreenInstance {
      screen,
      state: ScreenState::new(),
      mounted: false,
      visible: false,
    };

    self.screens.write().await.insert(id.clone(), instance);

    // If this is the first screen or the default screen, set it as current
    if self.current_screen.read().unwrap().is_none() || id == self.config.default_screen {
      self.navigate_to(&id, NavigationOptions::default()).await?;
    }

    Ok(())
  }

  /// Navigate to a screen
  pub async fn navigate_to(&self, screen_id: &str, options: NavigationOptions) -> Result<()> {
    // Check if screen exists
    {
      let screens = self.screens.read().await;
      if !screens.contains_key(screen_id) {
        return Err(TuiError::component(format!(
          "Screen '{screen_id}' not found"
        )));
      }
    }

    // Handle current screen deactivation
    let current_id = self.current_screen.read().unwrap().clone();
    if let Some(current_id) = current_id {
      let mut screens = self.screens.write().await;
      if let Some(current) = screens.get_mut(&current_id) {
        // Check if current screen can be deactivated
        if !current.screen.can_deactivate(&current.state) {
          return Err(TuiError::component(
            "Current screen cannot be deactivated".to_string(),
          ));
        }

        // Hide current screen
        current
          .screen
          .handle_event(ScreenEvent::Hide, &mut current.state)
          .await?;
        current
          .screen
          .handle_event(ScreenEvent::Blur, &mut current.state)
          .await?;
        current.visible = false;
      }
    }

    // Start transition
    {
      let mut transition_manager = self.transition_manager.write().unwrap();
      transition_manager.start_transition(options.transition, options.duration);
    }

    // Activate new screen
    {
      let mut screens = self.screens.write().await;
      if let Some(new_screen) = screens.get_mut(screen_id) {
        // Mount if needed
        if !new_screen.mounted {
          new_screen
            .screen
            .handle_event(ScreenEvent::Mount, &mut new_screen.state)
            .await?;
          new_screen.mounted = true;
        }

        // Apply navigation params
        for (key, value) in options.params {
          new_screen.state.set_param(&key, &value);
        }

        // Show screen
        new_screen
          .screen
          .handle_event(ScreenEvent::Show, &mut new_screen.state)
          .await?;
        new_screen
          .screen
          .handle_event(ScreenEvent::Focus, &mut new_screen.state)
          .await?;
        new_screen.visible = true;
      }
    }

    // Update current screen
    let previous = self
      .current_screen
      .write()
      .unwrap()
      .replace(screen_id.to_string());

    // Update history
    if options.add_to_history && !options.replace {
      if let Some(prev_id) = previous {
        self.history.write().unwrap().push(prev_id);
      }
    }

    // Update workspace
    {
      let workspace_id = self.current_workspace.read().unwrap().clone();
      let mut workspaces = self.workspaces.write().unwrap();
      if let Some(workspace) = workspaces.get_mut(&workspace_id) {
        workspace.set_active_screen(screen_id);
      }
    }

    Ok(())
  }

  /// Navigate back in history
  pub async fn navigate_back(&self) -> Result<()> {
    let previous = self.history.write().unwrap().pop();
    if let Some(screen_id) = previous {
      self
        .navigate_to(
          &screen_id,
          NavigationOptions {
            add_to_history: false,
            ..Default::default()
          },
        )
        .await
    } else {
      Err(TuiError::component(
        "No history to navigate back".to_string(),
      ))
    }
  }

  /// Navigate forward in history
  pub async fn navigate_forward(&self) -> Result<()> {
    let next = self.history.write().unwrap().forward();
    if let Some(screen_id) = next {
      self
        .navigate_to(
          &screen_id,
          NavigationOptions {
            add_to_history: false,
            ..Default::default()
          },
        )
        .await
    } else {
      Err(TuiError::component(
        "No history to navigate forward".to_string(),
      ))
    }
  }

  /// Get current screen ID
  pub fn current_screen(&self) -> Option<String> {
    self.current_screen.read().unwrap().clone()
  }

  /// Create a new workspace
  pub fn create_workspace(&self, id: &str, name: &str) -> Result<()> {
    let workspace = Workspace::new(id, name);
    self
      .workspaces
      .write()
      .unwrap()
      .insert(id.to_string(), workspace);
    Ok(())
  }

  /// Switch to a workspace
  pub async fn switch_workspace(&self, workspace_id: &str) -> Result<()> {
    let screen_id = {
      let workspaces = self.workspaces.read().unwrap();
      if !workspaces.contains_key(workspace_id) {
        return Err(TuiError::component(format!(
          "Workspace '{workspace_id}' not found"
        )));
      }

      // Get the active screen in the new workspace
      workspaces
        .get(workspace_id)
        .and_then(|w| w.active_screen())
        .or_else(|| Some(self.config.default_screen.clone()))
        .unwrap()
    };

    // Update current workspace
    *self.current_workspace.write().unwrap() = workspace_id.to_string();

    // Navigate to the workspace's active screen
    self
      .navigate_to(&screen_id, NavigationOptions::default())
      .await
  }

  /// Handle keyboard input
  pub async fn handle_input(&self, event: Event) -> ActionResult {
    // Handle navigation shortcuts
    if let Event::Key(key) = &event {
      if self.config.enable_keyboard_nav {
        // Check global navigation shortcuts
        match key.code {
          crate::compat::KeyCode::Esc => {
            // Navigate back
            if let Ok(()) = self.navigate_back().await {
              return ActionResult::Handled;
            }
          }
          crate::compat::KeyCode::Tab
            if key.modifiers.contains(crate::compat::KeyModifiers::CONTROL) =>
          {
            // Cycle through workspaces
            let (current, workspace_ids) = {
              let workspaces = self.workspaces.read().unwrap();
              let current = self.current_workspace.read().unwrap().clone();
              let workspace_ids: Vec<String> = workspaces.keys().cloned().collect();
              (current, workspace_ids)
            };

            if let Some(current_index) = workspace_ids.iter().position(|id| id == &current) {
              let next_index = (current_index + 1) % workspace_ids.len();
              let next_id = &workspace_ids[next_index];

              if let Ok(()) = self.switch_workspace(next_id).await {
                return ActionResult::Handled;
              }
            }
          }
          _ => {}
        }
      }
    }

    // Pass to current screen
    let screen_id = self.current_screen.read().unwrap().clone();
    if let Some(screen_id) = screen_id {
      let mut screens = self.screens.write().await;
      if let Some(screen_instance) = screens.get_mut(&screen_id) {
        return screen_instance
          .screen
          .handle_input(event, &mut screen_instance.state);
      }
    }

    ActionResult::NotHandled
  }

  /// Get current workspace ID
  pub fn current_workspace(&self) -> String {
    self.current_workspace.read().unwrap().clone()
  }

  /// Get all workspace IDs
  pub fn workspace_ids(&self) -> Vec<String> {
    self.workspaces.read().unwrap().keys().cloned().collect()
  }

  /// Navigate to screen with parameters
  pub async fn navigate_to_with_params(
    &self,
    screen_id: &str,
    params: serde_json::Value,
  ) -> Result<()> {
    let mut nav_params = HashMap::new();
    if let serde_json::Value::Object(map) = params {
      for (key, value) in map {
        if let serde_json::Value::String(s) = value {
          nav_params.insert(key, s);
        } else {
          nav_params.insert(key, value.to_string());
        }
      }
    }

    let options = NavigationOptions {
      params: nav_params,
      add_to_history: true,
      ..Default::default()
    };

    self.navigate_to(screen_id, options).await
  }

  /// Push a screen onto the navigation stack
  pub async fn push_screen(&self, screen_id: &str) -> Result<()> {
    self
      .navigate_to(
        screen_id,
        NavigationOptions {
          add_to_history: true,
          ..Default::default()
        },
      )
      .await
  }

  /// Go back in navigation history
  pub async fn go_back(&self) -> Result<()> {
    self.navigate_back().await
  }

  /// Go forward in navigation history
  pub async fn go_forward(&self) -> Result<()> {
    self.navigate_forward().await
  }

  /// Check if we can go back
  pub fn can_go_back(&self) -> bool {
    self.history.read().unwrap().can_go_back()
  }

  /// Check if we can go forward
  pub fn can_go_forward(&self) -> bool {
    self.history.read().unwrap().can_go_forward()
  }

  /// Get breadcrumb trail
  pub fn get_breadcrumbs(&self) -> Vec<String> {
    self.history.read().unwrap().breadcrumbs()
  }

  /// Get current screen state
  pub async fn get_current_screen_state(&self) -> Option<ScreenState> {
    let screen_id = self.current_screen.read().unwrap().clone()?;
    let screens = self.screens.read().await;
    screens
      .get(&screen_id)
      .map(|instance| instance.state.clone())
  }
}

/// Component implementation for ScreenManager
impl Component for ScreenManager {
  fn render(&self) -> Element {
    // Get current screen
    let current_id = self.current_screen.read().unwrap().clone();

    if let Some(screen_id) = current_id {
      // Use crate::compat::tokio_compat::block_in_place to safely access async lock in sync context
      #[cfg(target_family = "wasm")]
      let screens = crate::compat::tokio_compat::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(self.screens.read())
      });
      #[cfg(not(target_family = "wasm"))]
      let screens = tokio::task::block_in_place(|| {
        tokio::runtime::Handle::current().block_on(self.screens.read())
      });

      if let Some(_screen_instance) = screens.get(&screen_id) {
        // Check if we're in a transition
        let transition_manager = self.transition_manager.read().unwrap();
        if let Some(transition_element) = transition_manager.render_placeholder(&screen_id) {
          return transition_element;
        }

        // Screens must implement Component directly
        // For now, return a placeholder
        return div()
          .class("screen-placeholder")
          .child(text(format!("Screen: {screen_id}")).build())
          .build();
      }
    }

    // No screen to render
    div()
      .class("screen-manager-empty")
      .child(text("No active screen").build())
      .build()
  }
}
