/*!
 * Workspace management for organizing screens
 */

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
