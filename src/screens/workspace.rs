//! # Workspace Management
//!
//! Advanced workspace organization and multi-tabbed screen management.
//!
//! This module provides workspace functionality for organizing related screens into
//! logical groups, similar to browser tabs or IDE workspaces. Workspaces enable
//! users to switch between different contexts while preserving state and providing
//! organized navigation.
//!
//! ## Features
//!
//! - **Multi-Workspace Support**: Organize screens into logical workspaces
//! - **Tab Management**: Visual tabs for workspace navigation
//! - **State Preservation**: Maintain workspace state across switches
//! - **Context Isolation**: Separate navigation history per workspace
//! - **Workspace Persistence**: Save and restore workspace configurations
//! - **Custom Layouts**: Per-workspace layout configurations
//!
//! ## Examples
//!
//! ### Basic Workspace Setup
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{WorkspaceManager, Screen, ScreenConfig};
//! use async_trait::async_trait;
//!
//! // Simple screen implementations for demo
//! struct CodeEditorScreen;
//! struct TerminalScreen;
//!
//! #[async_trait]
//! impl Screen for CodeEditorScreen {
//!     fn config(&self) -> ScreenConfig {
//!         ScreenConfig { id: "code_editor".to_string(), title: "Code Editor".to_string(), ..Default::default() }
//!     }
//! }
//!
//! impl Component for CodeEditorScreen {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div").content("Code Editor").build()
//!     }
//! }
//!
//! #[async_trait]
//! impl Screen for TerminalScreen {
//!     fn config(&self) -> ScreenConfig {
//!         ScreenConfig { id: "terminal".to_string(), title: "Terminal".to_string(), ..Default::default() }
//!     }
//! }
//!
//! impl Component for TerminalScreen {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div").content("Terminal").build()
//!     }
//! }
//!
//! let mut workspace_manager = WorkspaceManager::new();
//!
//! // Create workspaces
//! workspace_manager.create_workspace("development", "Development")?;
//! workspace_manager.create_workspace("documentation", "Documentation")?;
//! workspace_manager.create_workspace("testing", "Testing")?;
//!
//! // Add screens to workspaces
//! workspace_manager.add_screen_to_workspace(
//!     "development",
//!     "code_editor",
//!     Box::new(CodeEditorScreen)
//! )?;
//! workspace_manager.add_screen_to_workspace(
//!     "development",
//!     "terminal",
//!     Box::new(TerminalScreen)
//! )?;
//!
//! // Switch to workspace
//! workspace_manager.switch_to_workspace("development")?;
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Workspace with Custom Layout
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::{WorkspaceManager, Screen, ScreenConfig, WorkspaceLayout, LayoutOrientation, SplitConfig, WorkspaceTabPosition};
//! use async_trait::async_trait;
//!
//! // Demo screen implementations
//! struct EditorScreen;
//! struct FileExplorerScreen;
//!
//! #[async_trait]
//! impl Screen for EditorScreen {
//!     fn config(&self) -> ScreenConfig {
//!         ScreenConfig { id: "editor".to_string(), title: "Editor".to_string(), ..Default::default() }
//!     }
//! }
//!
//! impl Component for EditorScreen {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div").class("editor").content("Editor Content").build()
//!     }
//! }
//!
//! #[async_trait]
//! impl Screen for FileExplorerScreen {
//!     fn config(&self) -> ScreenConfig {
//!         ScreenConfig { id: "file_explorer".to_string(), title: "Files".to_string(), ..Default::default() }
//!     }
//! }
//!
//! impl Component for FileExplorerScreen {
//!     fn render(&self) -> Element {
//!         Element::with_tag("div").class("file-explorer").content("File Explorer").build()
//!     }
//! }
//!
//! let mut workspace_manager = WorkspaceManager::new();
//!
//! // Create workspace with custom layout
//! let layout_config = WorkspaceLayout {
//!     orientation: LayoutOrientation::Horizontal,
//!     splits: vec![
//!         SplitConfig { size: 70, screen: "main_content".to_string() },
//!         SplitConfig { size: 30, screen: "sidebar".to_string() },
//!     ],
//!     tab_position: WorkspaceTabPosition::Top,
//! };
//!
//! workspace_manager.create_workspace_with_layout(
//!     "ide",
//!     "IDE Workspace",
//!     layout_config
//! )?;
//!
//! // Configure screens within the layout
//! workspace_manager.add_screen_to_split(
//!     "ide",
//!     "main_content",
//!     Box::new(EditorScreen)
//! )?;
//! workspace_manager.add_screen_to_split(
//!     "ide",
//!     "sidebar",
//!     Box::new(FileExplorerScreen)
//! )?;
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Workspace State and Persistence
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::screens::WorkspaceManager;
//!
//! let mut workspace_manager = WorkspaceManager::new();
//!
//! // Create a workspace first
//! workspace_manager.create_workspace("development", "Development")?;
//!
//! // Save workspace configuration
//! workspace_manager.save_workspace_config("development", "dev_config.json")?;
//!
//! // Load workspace configuration
//! workspace_manager.load_workspace_config("development", "dev_config.json")?;
//!
//! // Export all workspaces
//! let workspace_data = workspace_manager.export_workspaces()?;
//!
//! // Import workspace data
//! workspace_manager.import_workspaces(workspace_data)?;
//!
//! // Get workspace state
//! let current_state = workspace_manager.get_workspace_state("development")?;
//! println!("Workspace state retrieved successfully");
//!
//! // Get all workspace IDs
//! let workspace_ids = workspace_manager.workspace_ids();
//! println!("Available workspaces: {:?}", workspace_ids);
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```

use super::*;

/// Workspace represents a collection of related screens
#[derive(Debug, Clone)]
pub struct Workspace {
  /// Workspace ID
  id: String,
  /// Workspace name
  name: String,
  /// Screen IDs in this workspace
  screen_ids: Vec<String>,
  /// Currently active screen in this workspace
  active_screen: Option<String>,
  /// Workspace-specific shortcuts
  shortcuts: HashMap<char, String>,
  /// Workspace metadata
  #[allow(dead_code)]
  metadata: HashMap<String, String>,
}

impl Workspace {
  /// Create a new workspace
  pub fn new(id: &str, name: &str) -> Self {
    Self {
      id: id.to_string(),
      name: name.to_string(),
      screen_ids: Vec::new(),
      active_screen: None,
      shortcuts: HashMap::new(),
      metadata: HashMap::new(),
    }
  }

  /// Add a screen to the workspace
  pub fn add_screen(&mut self, screen_id: &str) {
    if !self.screen_ids.contains(&screen_id.to_string()) {
      self.screen_ids.push(screen_id.to_string());

      // Set as active if it's the first screen
      if self.active_screen.is_none() {
        self.active_screen = Some(screen_id.to_string());
      }
    }
  }

  /// Remove a screen from the workspace
  pub fn remove_screen(&mut self, screen_id: &str) {
    self.screen_ids.retain(|id| id != screen_id);

    // Update active screen if needed
    if self.active_screen.as_ref() == Some(&screen_id.to_string()) {
      self.active_screen = self.screen_ids.first().cloned();
    }
  }

  /// Set the active screen
  pub fn set_active_screen(&mut self, screen_id: &str) {
    if self.screen_ids.contains(&screen_id.to_string()) {
      self.active_screen = Some(screen_id.to_string());
    }
  }

  /// Get the active screen
  pub fn active_screen(&self) -> Option<String> {
    self.active_screen.clone()
  }

  /// Get all screen IDs
  pub fn screen_ids(&self) -> &[String] {
    &self.screen_ids
  }

  /// Add a keyboard shortcut
  pub fn add_shortcut(&mut self, key: char, screen_id: &str) {
    if self.screen_ids.contains(&screen_id.to_string()) {
      self.shortcuts.insert(key, screen_id.to_string());
    }
  }

  /// Get screen ID for a shortcut
  pub fn get_shortcut(&self, key: char) -> Option<&str> {
    self.shortcuts.get(&key).map(|s| s.as_str())
  }

  /// Get workspace ID
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Get workspace name
  pub fn name(&self) -> &str {
    &self.name
  }

  /// Set workspace name
  pub fn set_name(&mut self, name: &str) {
    self.name = name.to_string();
  }
}

/// Tabbed workspace component for visual representation
pub struct TabbedWorkspace {
  workspaces: Vec<Workspace>,
  active_workspace: usize,
}

impl TabbedWorkspace {
  /// Create a new tabbed workspace
  pub fn new() -> Self {
    Self {
      workspaces: vec![Workspace::new("default", "Default")],
      active_workspace: 0,
    }
  }

  /// Add a workspace
  pub fn add_workspace(&mut self, workspace: Workspace) {
    self.workspaces.push(workspace);
  }

  /// Remove a workspace
  pub fn remove_workspace(&mut self, index: usize) {
    if self.workspaces.len() > 1 && index < self.workspaces.len() {
      self.workspaces.remove(index);

      // Adjust active workspace if needed
      if self.active_workspace >= self.workspaces.len() {
        self.active_workspace = self.workspaces.len() - 1;
      }
    }
  }

  /// Set active workspace
  pub fn set_active(&mut self, index: usize) {
    if index < self.workspaces.len() {
      self.active_workspace = index;
    }
  }

  /// Get active workspace
  pub fn active(&self) -> Option<&Workspace> {
    self.workspaces.get(self.active_workspace)
  }

  /// Get active workspace (mutable)
  pub fn active_mut(&mut self) -> Option<&mut Workspace> {
    self.workspaces.get_mut(self.active_workspace)
  }

  /// Switch to next workspace
  pub fn next_workspace(&mut self) {
    if !self.workspaces.is_empty() {
      self.active_workspace = (self.active_workspace + 1) % self.workspaces.len();
    }
  }

  /// Switch to previous workspace
  pub fn prev_workspace(&mut self) {
    if !self.workspaces.is_empty() {
      self.active_workspace = if self.active_workspace == 0 {
        self.workspaces.len() - 1
      } else {
        self.active_workspace - 1
      };
    }
  }

  /// Get all workspaces
  pub fn workspaces(&self) -> &[Workspace] {
    &self.workspaces
  }
}

impl Default for TabbedWorkspace {
  fn default() -> Self {
    Self::new()
  }
}

impl Component for TabbedWorkspace {
  fn render(&self) -> Element {
    use crate::components::{div, text};

    div()
      .class("tabbed-workspace")
      .child(
        // Tab bar
        div()
          .class("tab-bar")
          .class("flex")
          .class("border-bottom")
          .children(
            self
              .workspaces
              .iter()
              .enumerate()
              .map(|(i, workspace)| {
                div()
                  .class("tab")
                  .class(if i == self.active_workspace {
                    "active"
                  } else {
                    ""
                  })
                  .class("px-2")
                  .class("py-1")
                  .class("cursor-pointer")
                  .child(text(&workspace.name).build())
                  .build()
              })
              .collect::<Vec<_>>(),
          )
          .build(),
      )
      .child(
        // Content area
        div()
          .class("workspace-content")
          .class("flex-1")
          .child(if let Some(workspace) = self.active() {
            div()
              .child(text(format!("Workspace: {}", workspace.name())).build())
              .build()
          } else {
            text("No active workspace").build()
          })
          .build(),
      )
      .build()
  }
}

/// Split screen layout for showing multiple screens simultaneously
pub struct SplitScreen {
  /// Screen IDs to display
  screen_ids: Vec<String>,
  /// Split orientation
  #[allow(dead_code)]
  orientation: SplitOrientation,
  /// Split ratios (should sum to 1.0)
  ratios: Vec<f32>,
}

#[derive(Debug, Clone, Copy)]
pub enum SplitOrientation {
  Horizontal,
  Vertical,
}

impl SplitScreen {
  /// Create a new split screen
  pub fn new(orientation: SplitOrientation) -> Self {
    Self {
      screen_ids: Vec::new(),
      orientation,
      ratios: Vec::new(),
    }
  }

  /// Add a screen to the split
  pub fn add_screen(&mut self, screen_id: &str, ratio: f32) {
    self.screen_ids.push(screen_id.to_string());
    self.ratios.push(ratio);

    // Normalize ratios
    let sum: f32 = self.ratios.iter().sum();
    if sum > 0.0 {
      for ratio in &mut self.ratios {
        *ratio /= sum;
      }
    }
  }

  /// Remove a screen from the split
  pub fn remove_screen(&mut self, index: usize) {
    if index < self.screen_ids.len() {
      self.screen_ids.remove(index);
      self.ratios.remove(index);

      // Normalize remaining ratios
      let sum: f32 = self.ratios.iter().sum();
      if sum > 0.0 {
        for ratio in &mut self.ratios {
          *ratio /= sum;
        }
      }
    }
  }

  /// Get screen IDs
  pub fn screen_ids(&self) -> &[String] {
    &self.screen_ids
  }
}

/// Workspace manager that coordinates multiple workspaces and screen management
pub struct WorkspaceManager {
  workspaces: HashMap<String, Workspace>,
  active_workspace: Option<String>,
  screens: HashMap<String, Box<dyn Screen>>,
  #[allow(dead_code)]
  config: ScreenManagerConfig,
  workspace_states: HashMap<String, ScreenState>,
}

impl WorkspaceManager {
  /// Create a new workspace manager
  pub fn new() -> Self {
    Self {
      workspaces: HashMap::new(),
      active_workspace: None,
      screens: HashMap::new(),
      config: ScreenManagerConfig::default(),
      workspace_states: HashMap::new(),
    }
  }

  /// Create a new workspace
  pub fn create_workspace(&mut self, id: &str, name: &str) -> Result<()> {
    let workspace = Workspace::new(id, name);
    self.workspaces.insert(id.to_string(), workspace);
    self
      .workspace_states
      .insert(id.to_string(), ScreenState::new());

    // Set as active if it's the first workspace
    if self.active_workspace.is_none() {
      self.active_workspace = Some(id.to_string());
    }

    Ok(())
  }

  /// Add a screen to a workspace
  pub fn add_screen_to_workspace(
    &mut self,
    workspace_id: &str,
    screen_id: &str,
    screen: Box<dyn Screen>,
  ) -> Result<()> {
    if let Some(workspace) = self.workspaces.get_mut(workspace_id) {
      workspace.add_screen(screen_id);
      self.screens.insert(screen_id.to_string(), screen);
      Ok(())
    } else {
      Err(crate::error::TuiError::component(format!(
        "Workspace '{workspace_id}' not found"
      )))
    }
  }

  /// Switch to a workspace
  pub fn switch_to_workspace(&mut self, workspace_id: &str) -> Result<()> {
    if self.workspaces.contains_key(workspace_id) {
      self.active_workspace = Some(workspace_id.to_string());
      Ok(())
    } else {
      Err(crate::error::TuiError::component(format!(
        "Workspace '{workspace_id}' not found"
      )))
    }
  }

  /// Get workspace state
  pub fn get_workspace_state(&self, workspace_id: &str) -> Result<&ScreenState> {
    self.workspace_states.get(workspace_id).ok_or_else(|| {
      crate::error::TuiError::component(format!("Workspace '{workspace_id}' not found"))
    })
  }

  /// Get mutable workspace state
  pub fn get_workspace_state_mut(&mut self, workspace_id: &str) -> Result<&mut ScreenState> {
    self.workspace_states.get_mut(workspace_id).ok_or_else(|| {
      crate::error::TuiError::component(format!("Workspace '{workspace_id}' not found"))
    })
  }

  /// Export workspace configurations to JSON
  pub fn export_workspaces(&self) -> Result<serde_json::Value> {
    let mut workspace_data = serde_json::Map::new();

    for (id, workspace) in &self.workspaces {
      let mut workspace_obj = serde_json::Map::new();
      workspace_obj.insert(
        "id".to_string(),
        serde_json::Value::String(workspace.id().to_string()),
      );
      workspace_obj.insert(
        "name".to_string(),
        serde_json::Value::String(workspace.name().to_string()),
      );
      workspace_obj.insert(
        "screen_ids".to_string(),
        serde_json::Value::Array(
          workspace
            .screen_ids()
            .iter()
            .map(|s| serde_json::Value::String(s.clone()))
            .collect(),
        ),
      );
      if let Some(active) = &workspace.active_screen() {
        workspace_obj.insert(
          "active_screen".to_string(),
          serde_json::Value::String(active.clone()),
        );
      }

      workspace_data.insert(id.clone(), serde_json::Value::Object(workspace_obj));
    }

    Ok(serde_json::Value::Object(workspace_data))
  }

  /// Import workspace configurations from JSON
  pub fn import_workspaces(&mut self, data: serde_json::Value) -> Result<()> {
    if let serde_json::Value::Object(workspaces) = data {
      for (id, workspace_data) in workspaces {
        if let serde_json::Value::Object(ws_obj) = workspace_data {
          if let (
            Some(serde_json::Value::String(name)),
            Some(serde_json::Value::Array(screen_ids)),
          ) = (ws_obj.get("name"), ws_obj.get("screen_ids"))
          {
            let mut workspace = Workspace::new(&id, name);

            // Add screens
            for screen_id in screen_ids {
              if let serde_json::Value::String(sid) = screen_id {
                workspace.add_screen(sid);
              }
            }

            // Set active screen
            if let Some(serde_json::Value::String(active)) = ws_obj.get("active_screen") {
              workspace.set_active_screen(active);
            }

            self.workspaces.insert(id.clone(), workspace);
            self.workspace_states.insert(id, ScreenState::new());
          }
        }
      }
    }
    Ok(())
  }

  /// Save workspace configuration to file
  pub fn save_workspace_config(&self, workspace_id: &str, file_path: &str) -> Result<()> {
    if let Some(_workspace) = self.workspaces.get(workspace_id) {
      let config_data = self.export_workspaces()?;
      let config_str = serde_json::to_string_pretty(&config_data).map_err(|e| {
        crate::error::TuiError::component(format!("Failed to serialize config: {e}"))
      })?;

      std::fs::write(file_path, config_str).map_err(|e| {
        crate::error::TuiError::component(format!("Failed to write config file: {e}"))
      })?;

      Ok(())
    } else {
      Err(crate::error::TuiError::component(format!(
        "Workspace '{workspace_id}' not found"
      )))
    }
  }

  /// Load workspace configuration from file
  pub fn load_workspace_config(&mut self, workspace_id: &str, file_path: &str) -> Result<()> {
    let config_str = std::fs::read_to_string(file_path)
      .map_err(|e| crate::error::TuiError::component(format!("Failed to read config file: {e}")))?;

    let config_data: serde_json::Value = serde_json::from_str(&config_str)
      .map_err(|e| crate::error::TuiError::component(format!("Failed to parse config: {e}")))?;

    self.import_workspaces(config_data)?;

    // Switch to the loaded workspace if it exists
    if self.workspaces.contains_key(workspace_id) {
      self.active_workspace = Some(workspace_id.to_string());
    }

    Ok(())
  }

  /// Get the active workspace
  pub fn active_workspace(&self) -> Option<&Workspace> {
    if let Some(active_id) = &self.active_workspace {
      self.workspaces.get(active_id)
    } else {
      None
    }
  }

  /// Get all workspace IDs
  pub fn workspace_ids(&self) -> Vec<String> {
    self.workspaces.keys().cloned().collect()
  }
}

impl Default for WorkspaceManager {
  fn default() -> Self {
    Self::new()
  }
}

/// Workspace layout configuration for advanced workspace layouts
#[derive(Debug, Clone)]
pub struct WorkspaceLayout {
  pub orientation: LayoutOrientation,
  pub splits: Vec<SplitConfig>,
  pub tab_position: TabPosition,
}

#[derive(Debug, Clone)]
pub enum LayoutOrientation {
  Horizontal,
  Vertical,
}

#[derive(Debug, Clone)]
pub struct SplitConfig {
  pub size: u32, // Percentage
  pub screen: String,
}

#[derive(Debug, Clone)]
pub enum TabPosition {
  Top,
  Bottom,
  Left,
  Right,
}

impl WorkspaceManager {
  /// Create workspace with custom layout
  pub fn create_workspace_with_layout(
    &mut self,
    id: &str,
    name: &str,
    layout: WorkspaceLayout,
  ) -> Result<()> {
    self.create_workspace(id, name)?;

    // Store layout configuration in workspace metadata
    if let Some(workspace) = self.workspaces.get_mut(id) {
      workspace.metadata.insert(
        "layout_orientation".to_string(),
        match layout.orientation {
          LayoutOrientation::Horizontal => "horizontal".to_string(),
          LayoutOrientation::Vertical => "vertical".to_string(),
        },
      );
      workspace.metadata.insert(
        "tab_position".to_string(),
        match layout.tab_position {
          TabPosition::Top => "top".to_string(),
          TabPosition::Bottom => "bottom".to_string(),
          TabPosition::Left => "left".to_string(),
          TabPosition::Right => "right".to_string(),
        },
      );

      // Add splits as screens
      for split in layout.splits {
        workspace.add_screen(&split.screen);
        workspace.metadata.insert(
          format!("split_{}_size", split.screen),
          split.size.to_string(),
        );
      }
    }

    Ok(())
  }

  /// Add screen to a specific split
  pub fn add_screen_to_split(
    &mut self,
    workspace_id: &str,
    split_name: &str,
    screen: Box<dyn Screen>,
  ) -> Result<()> {
    self.add_screen_to_workspace(workspace_id, split_name, screen)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_workspace() {
    let mut workspace = Workspace::new("main", "Main Workspace");

    // Add screens
    workspace.add_screen("home");
    workspace.add_screen("settings");
    workspace.add_screen("profile");

    assert_eq!(workspace.screen_ids().len(), 3);
    assert_eq!(workspace.active_screen(), Some("home".to_string()));

    // Set active screen
    workspace.set_active_screen("settings");
    assert_eq!(workspace.active_screen(), Some("settings".to_string()));

    // Add shortcuts
    workspace.add_shortcut('h', "home");
    workspace.add_shortcut('s', "settings");

    assert_eq!(workspace.get_shortcut('h'), Some("home"));
    assert_eq!(workspace.get_shortcut('s'), Some("settings"));

    // Remove screen
    workspace.remove_screen("home");
    assert_eq!(workspace.screen_ids().len(), 2);
    assert_eq!(workspace.active_screen(), Some("settings".to_string()));
  }

  #[test]
  fn test_tabbed_workspace() {
    let mut tabbed = TabbedWorkspace::new();

    // Add workspaces
    tabbed.add_workspace(Workspace::new("work", "Work"));
    tabbed.add_workspace(Workspace::new("personal", "Personal"));

    assert_eq!(tabbed.workspaces().len(), 3); // Including default

    // Navigate workspaces
    tabbed.next_workspace();
    assert_eq!(tabbed.active_workspace, 1);

    tabbed.prev_workspace();
    assert_eq!(tabbed.active_workspace, 0);

    // Set active
    tabbed.set_active(2);
    assert_eq!(tabbed.active().unwrap().id(), "personal");
  }
}
