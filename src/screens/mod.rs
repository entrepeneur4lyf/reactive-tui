/*!
 * Multi-screen and Workspace Support
 *
 * A comprehensive screen management system that enables TUI applications to have
 * multiple screens, workspaces, and navigation between them.
 *
 * Features:
 * - Multiple screen support with navigation
 * - Workspace/tab management
 * - Screen lifecycle hooks
 * - Transition animations
 * - Navigation history and routing
 * - Screen state preservation
 * - Keyboard shortcuts for navigation
 *
 * Example:
 * ```rust,no_run
 * use reactive_tui::screens::*;
 * use reactive_tui::components::{div, text, Element, Component};
 * use reactive_tui::error::Result;
 * use async_trait::async_trait;
 *
 * // Define screens
 * struct HomeScreen;
 *
 * #[async_trait]
 * impl Screen for HomeScreen {
 *     fn config(&self) -> ScreenConfig {
 *         ScreenConfig {
 *             id: "home".to_string(),
 *             title: "Home".to_string(),
 *             ..Default::default()
 *         }
 *     }
 * }
 *
 * impl Component for HomeScreen {
 *     fn render(&self) -> Element {
 *         div().child(text("Home Screen").into()).build()
 *     }
 * }
 *
 * #[tokio::main]
 * async fn main() -> Result<()> {
 *     // Create screen manager with config
 *     let config = ScreenManagerConfig::default();
 *     let manager = ScreenManager::new(config);
 *     
 *     // Register screens
 *     manager.register(Box::new(HomeScreen)).await?;
 *     
 *     // Navigate between screens
 *     manager.navigate_to("home", NavigationOptions::default()).await?;
 *     
 *     Ok(())
 * }
 * ```
 */

mod manager;
mod navigation;
mod screen;
mod transitions;
mod workspace;

pub use manager::*;
pub use navigation::*;
pub use screen::*;
pub use transitions::*;
pub use workspace::*;

use crate::{
  components::{Component, Element},
  error::Result,
};
use std::collections::HashMap;
use std::sync::RwLock;

/// Screen lifecycle events
#[derive(Debug, Clone)]
pub enum ScreenEvent {
  /// Screen is being mounted
  Mount,
  /// Screen is being unmounted
  Unmount,
  /// Screen is being shown
  Show,
  /// Screen is being hidden
  Hide,
  /// Screen received focus
  Focus,
  /// Screen lost focus
  Blur,
  /// Custom screen event
  Custom(String, serde_json::Value),
}

/// Screen state that can be preserved across navigation
#[derive(Debug, Clone, Default)]
pub struct ScreenState {
  /// Saved state data
  data: HashMap<String, serde_json::Value>,
  /// Navigation parameters
  params: HashMap<String, String>,
  /// Screen metadata
  #[allow(dead_code)]
  metadata: HashMap<String, String>,
}

impl ScreenState {
  /// Create new screen state
  pub fn new() -> Self {
    Self::default()
  }

  /// Set state value
  pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<()> {
    self.data.insert(
      key.to_string(),
      serde_json::to_value(value).map_err(|e| crate::error::TuiError::component(e.to_string()))?,
    );
    Ok(())
  }

  /// Get state value
  pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Option<T> {
    self
      .data
      .get(key)
      .and_then(|v| serde_json::from_value(v.clone()).ok())
  }

  /// Set navigation parameter
  pub fn set_param(&mut self, key: &str, value: &str) {
    self.params.insert(key.to_string(), value.to_string());
  }

  /// Get navigation parameter
  pub fn get_param(&self, key: &str) -> Option<&str> {
    self.params.get(key).map(|s| s.as_str())
  }
}

/// Screen configuration
#[derive(Debug, Clone)]
pub struct ScreenConfig {
  /// Screen ID
  pub id: String,
  /// Screen title
  pub title: String,
  /// Screen icon (optional)
  pub icon: Option<String>,
  /// Whether to preserve state on unmount
  pub preserve_state: bool,
  /// Custom metadata
  pub metadata: HashMap<String, String>,
}

impl Default for ScreenConfig {
  fn default() -> Self {
    Self {
      id: String::new(),
      title: String::new(),
      icon: None,
      preserve_state: true,
      metadata: HashMap::new(),
    }
  }
}

/// Navigation options
#[derive(Debug, Clone, Default)]
pub struct NavigationOptions {
  /// Transition type
  pub transition: TransitionType,
  /// Transition duration in milliseconds
  pub duration: u32,
  /// Whether to add to history
  pub add_to_history: bool,
  /// Whether to replace current screen
  pub replace: bool,
  /// Navigation parameters
  pub params: HashMap<String, String>,
}

/// Workspace configuration
#[derive(Debug, Clone)]
pub struct WorkspaceConfig {
  /// Workspace ID
  pub id: String,
  /// Workspace name
  pub name: String,
  /// Default screen ID
  pub default_screen: String,
  /// Workspace shortcuts
  pub shortcuts: HashMap<char, String>,
}

/// Screen manager configuration
#[derive(Debug, Clone)]
pub struct ScreenManagerConfig {
  /// Default screen ID
  pub default_screen: String,
  /// Enable navigation history
  pub enable_history: bool,
  /// Maximum history size
  pub max_history_size: usize,
  /// Enable keyboard navigation
  pub enable_keyboard_nav: bool,
  /// Navigation shortcuts
  pub shortcuts: HashMap<String, String>,
}

impl Default for ScreenManagerConfig {
  fn default() -> Self {
    Self {
      default_screen: "home".to_string(),
      enable_history: true,
      max_history_size: 50,
      enable_keyboard_nav: true,
      shortcuts: HashMap::new(),
    }
  }
}
