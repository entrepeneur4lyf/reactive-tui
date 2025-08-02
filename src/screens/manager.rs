/*!
 * Screen Manager - Core screen management and navigation
 */

use super::*;
use crate::{
    components::{Component, Element, div, text},
    events::{Event, ActionResult},
    error::{Result, TuiError},
};
use std::sync::Arc;
use tokio::sync::RwLock as TokioRwLock;

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
                return Err(TuiError::component(format!("Screen '{}' not found", screen_id)));
            }
        }
        
        // Handle current screen deactivation
        if let Some(current_id) = self.current_screen.read().unwrap().clone() {
            let mut screens = self.screens.write().await;
            if let Some(current) = screens.get_mut(&current_id) {
                // Check if current screen can be deactivated
                if !current.screen.can_deactivate(&current.state) {
                    return Err(TuiError::component("Current screen cannot be deactivated".to_string()));
                }
                
                // Hide current screen
                current.screen.handle_event(ScreenEvent::Hide, &mut current.state).await?;
                current.screen.handle_event(ScreenEvent::Blur, &mut current.state).await?;
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
                    new_screen.screen.handle_event(ScreenEvent::Mount, &mut new_screen.state).await?;
                    new_screen.mounted = true;
                }
                
                // Apply navigation params
                for (key, value) in options.params {
                    new_screen.state.set_param(&key, &value);
                }
                
                // Show screen
                new_screen.screen.handle_event(ScreenEvent::Show, &mut new_screen.state).await?;
                new_screen.screen.handle_event(ScreenEvent::Focus, &mut new_screen.state).await?;
                new_screen.visible = true;
            }
        }
        
        // Update current screen
        let previous = self.current_screen.write().unwrap().replace(screen_id.to_string());
        
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
            self.navigate_to(&screen_id, NavigationOptions {
                add_to_history: false,
                ..Default::default()
            }).await
        } else {
            Err(TuiError::component("No history to navigate back".to_string()))
        }
    }
    
    /// Navigate forward in history
    pub async fn navigate_forward(&self) -> Result<()> {
        let next = self.history.write().unwrap().forward();
        if let Some(screen_id) = next {
            self.navigate_to(&screen_id, NavigationOptions {
                add_to_history: false,
                ..Default::default()
            }).await
        } else {
            Err(TuiError::component("No history to navigate forward".to_string()))
        }
    }
    
    /// Get current screen ID
    pub fn current_screen(&self) -> Option<String> {
        self.current_screen.read().unwrap().clone()
    }
    
    /// Create a new workspace
    pub fn create_workspace(&self, id: &str, name: &str) -> Result<()> {
        let workspace = Workspace::new(id, name);
        self.workspaces.write().unwrap().insert(id.to_string(), workspace);
        Ok(())
    }
    
    /// Switch to a workspace
    pub async fn switch_workspace(&self, workspace_id: &str) -> Result<()> {
        let workspaces = self.workspaces.read().unwrap();
        if !workspaces.contains_key(workspace_id) {
            return Err(TuiError::component(format!("Workspace '{}' not found", workspace_id)));
        }
        
        // Get the active screen in the new workspace
        let screen_id = workspaces.get(workspace_id)
            .and_then(|w| w.active_screen())
            .or_else(|| Some(self.config.default_screen.clone()))
            .unwrap();
        
        // Update current workspace
        *self.current_workspace.write().unwrap() = workspace_id.to_string();
        
        // Navigate to the workspace's active screen
        self.navigate_to(&screen_id, NavigationOptions::default()).await
    }
    
    /// Handle keyboard input
    pub async fn handle_input(&self, event: Event) -> ActionResult {
        // Handle navigation shortcuts
        if let Event::Key(key) = &event {
            if self.config.enable_keyboard_nav {
                // Check global navigation shortcuts
                match key.code {
                    crossterm::event::KeyCode::Esc => {
                        // Navigate back
                        if let Ok(()) = self.navigate_back().await {
                            return ActionResult::Handled;
                        }
                    }
                    crossterm::event::KeyCode::Tab if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
                        // Cycle through workspaces
                        let workspaces = self.workspaces.read().unwrap();
                        let current = self.current_workspace.read().unwrap().clone();
                        let workspace_ids: Vec<String> = workspaces.keys().cloned().collect();
                        
                        if let Some(current_index) = workspace_ids.iter().position(|id| id == &current) {
                            let next_index = (current_index + 1) % workspace_ids.len();
                            let next_id = &workspace_ids[next_index];
                            drop(workspaces);
                            
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
        if let Some(screen_id) = self.current_screen.read().unwrap().clone() {
            let mut screens = self.screens.write().await;
            if let Some(screen_instance) = screens.get_mut(&screen_id) {
                return screen_instance.screen.handle_input(event, &mut screen_instance.state);
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
}

/// Component implementation for ScreenManager
impl Component for ScreenManager {
    fn render(&self) -> Element {
        // Get current screen
        let current_id = self.current_screen.read().unwrap().clone();
        
        if let Some(screen_id) = current_id {
            // Use tokio::task::block_in_place to safely access async lock in sync context
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
                    .child(text(&format!("Screen: {}", screen_id)).build())
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