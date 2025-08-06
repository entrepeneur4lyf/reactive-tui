//! Component instance management for reactive integration

use crate::components::Component;
use crate::error::{Result, TuiError};
use std::collections::HashMap;
use tokio::sync::mpsc;
use uuid::Uuid;

/// Unique identifier for component instances
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ComponentId(String);

impl Default for ComponentId {
    fn default() -> Self {
        Self::new()
    }
}

impl ComponentId {
    /// Create a new unique component ID
    pub fn new() -> Self {
        Self(Uuid::new_v4().to_string())
    }

    /// Create a component ID from a string
    pub fn from_string(id: String) -> Self {
        Self(id)
    }

    /// Get the string representation
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for ComponentId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Component lifecycle events
#[derive(Debug, Clone)]
pub enum LifecycleEvent {
    Mounted(ComponentId),
    Unmounted(ComponentId),
    Updated(ComponentId),
    Error(ComponentId, String),
}

/// Component instance with lifecycle management
pub struct ComponentInstance {
    pub id: ComponentId,
    pub component: Box<dyn Component>,
    pub mounted: bool,
    pub needs_update: bool,
    pub last_render: Option<std::time::Instant>,
    pub render_count: u64,
    pub parent: Option<ComponentId>,
    pub children: Vec<ComponentId>,
}

impl ComponentInstance {
    /// Create a new component instance
    pub fn new(id: ComponentId, component: Box<dyn Component>) -> Self {
        Self {
            id,
            component,
            mounted: false,
            needs_update: true,
            last_render: None,
            render_count: 0,
            parent: None,
            children: Vec::new(),
        }
    }

    /// Mark component as needing update
    pub fn mark_for_update(&mut self) {
        self.needs_update = true;
    }

    /// Mark component as rendered
    pub fn mark_rendered(&mut self) {
        self.needs_update = false;
        self.last_render = Some(std::time::Instant::now());
        self.render_count += 1;
    }

    /// Check if component needs update
    pub fn needs_update(&self) -> bool {
        self.needs_update
    }

    /// Add a child component
    pub fn add_child(&mut self, child_id: ComponentId) {
        if !self.children.contains(&child_id) {
            self.children.push(child_id);
        }
    }

    /// Remove a child component
    pub fn remove_child(&mut self, child_id: &ComponentId) {
        self.children.retain(|id| id != child_id);
    }

    /// Set parent component
    pub fn set_parent(&mut self, parent_id: ComponentId) {
        self.parent = Some(parent_id);
    }

}

/// Manages component instances and their lifecycle
pub struct ComponentInstanceManager {
    instances: HashMap<ComponentId, ComponentInstance>,
    root_components: Vec<ComponentId>,
    component_hierarchy: HashMap<ComponentId, Vec<ComponentId>>,
    lifecycle_sender: Option<mpsc::UnboundedSender<LifecycleEvent>>,
}

impl ComponentInstanceManager {
    /// Create a new component instance manager
    pub fn new() -> Self {
        Self {
            instances: HashMap::new(),
            root_components: Vec::new(),
            component_hierarchy: HashMap::new(),
            lifecycle_sender: None,
        }
    }

    /// Create a new component instance manager with lifecycle events
    pub fn with_lifecycle_events() -> (Self, mpsc::UnboundedReceiver<LifecycleEvent>) {
        let (sender, receiver) = mpsc::unbounded_channel();
        let manager = Self {
            instances: HashMap::new(),
            root_components: Vec::new(),
            component_hierarchy: HashMap::new(),
            lifecycle_sender: Some(sender),
        };
        (manager, receiver)
    }

    /// Register a new component instance with full lifecycle
    pub async fn register_instance(&mut self, mut instance: ComponentInstance) -> Result<()> {
        let component_id = instance.id.clone();

        // Create component context
        let mut context = crate::components::ComponentContext::new(component_id.clone().to_string());
        context.state = crate::components::ComponentState::Created;

        // Call before mount lifecycle
        if let Err(e) = instance.component.on_before_mount(&mut context) {
            return Err(TuiError::component(format!(
                "Failed to call before_mount for component {component_id}: {e}"
            )));
        }

        // Call component mount lifecycle
        if let Err(e) = instance.component.on_mount(&mut context) {
            // Call error handler
            let _ = instance.component.on_error(&mut context, &e);
            return Err(TuiError::component(format!(
                "Failed to mount component {component_id}: {e}"
            )));
        }

        // Call after mount lifecycle
        if let Err(e) = instance.component.on_after_mount(&mut context) {
            eprintln!("Warning: after_mount failed for component {component_id}: {e}");
        }

        instance.mounted = true;

        // If no parent, it's a root component
        if instance.parent.is_none() {
            self.root_components.push(component_id.clone());
        }

        // Update hierarchy
        if let Some(parent_id) = &instance.parent {
            self.component_hierarchy
                .entry(parent_id.clone())
                .or_default()
                .push(component_id.clone());
        }

        // Store instance
        self.instances.insert(component_id.clone(), instance);

        // Send lifecycle event
        if let Some(sender) = &self.lifecycle_sender {
            let _ = sender.send(LifecycleEvent::Mounted(component_id));
        }

        Ok(())
    }

    /// Unregister a component instance
    pub async fn unregister_instance(&mut self, component_id: &ComponentId) -> Result<()> {
        if let Some(mut instance) = self.instances.remove(component_id) {
            // Unmount children first using Box::pin for recursion
            let children = instance.children.clone();
            for child_id in children {
                Box::pin(self.unregister_instance(&child_id)).await?;
            }

            // Call component unmount lifecycle
            let mut context = crate::components::ComponentContext::new(component_id.to_string());
            context.state = crate::components::ComponentState::Unmounting;

            // Call before unmount lifecycle
            if let Err(e) = instance.component.on_before_unmount(&mut context) {
                eprintln!("Warning: before_unmount failed for component {component_id}: {e}");
            }

            // Call unmount lifecycle
            if let Err(e) = instance.component.on_unmount(&mut context) {
                eprintln!("Warning: Failed to unmount component {component_id}: {e}");
            }

            // Remove from root components if present
            self.root_components.retain(|id| id != component_id);

            // Remove from hierarchy
            if let Some(parent_id) = &instance.parent {
                if let Some(siblings) = self.component_hierarchy.get_mut(parent_id) {
                    siblings.retain(|id| id != component_id);
                }
            }
            self.component_hierarchy.remove(component_id);

            // Update parent's children list
            if let Some(parent_id) = &instance.parent {
                if let Some(parent_instance) = self.instances.get_mut(parent_id) {
                    parent_instance.remove_child(component_id);
                }
            }

            // Send lifecycle event
            if let Some(sender) = &self.lifecycle_sender {
                let _ = sender.send(LifecycleEvent::Unmounted(component_id.clone()));
            }
        }

        Ok(())
    }

    /// Update a component and trigger its update lifecycle
    pub async fn update_component(&mut self, component_id: &ComponentId) -> Result<bool> {
        if let Some(instance) = self.instances.get_mut(component_id) {
            if !instance.mounted {
                return Ok(false); // Can't update unmounted component
            }

            let mut context = crate::components::ComponentContext::new(component_id.to_string());
            context.state = crate::components::ComponentState::Updating;

            // Check if component should update
            let old_context = crate::components::ComponentContext::new(component_id.to_string());
            if !instance.component.should_update(&old_context, &context) {
                return Ok(false);
            }

            // Call update lifecycle
            match instance.component.on_update(&mut context) {
                Ok(needs_render) => {
                    instance.needs_update = needs_render;
                    instance.render_count += 1;
                    instance.last_render = Some(std::time::Instant::now());

                    // Send lifecycle event
                    if let Some(sender) = &self.lifecycle_sender {
                        let _ = sender.send(LifecycleEvent::Updated(component_id.clone()));
                    }

                    Ok(needs_render)
                }
                Err(e) => {
                    // Call error handler
                    let _ = instance.component.on_error(&mut context, &e);

                    // Send error event
                    if let Some(sender) = &self.lifecycle_sender {
                        let _ = sender.send(LifecycleEvent::Error(component_id.clone(), e.to_string()));
                    }

                    Err(e)
                }
            }
        } else {
            Err(TuiError::component(format!("Component {component_id} not found")))
        }
    }

    /// Clear update flag for component
    pub fn clear_update_flag(&mut self, component_id: &ComponentId) {
        if let Some(instance) = self.instances.get_mut(component_id) {
            instance.needs_update = false;
        }
    }

    /// Get a component instance
    pub fn get_instance(&self, component_id: &ComponentId) -> Option<&ComponentInstance> {
        self.instances.get(component_id)
    }

    /// Get a mutable component instance
    pub fn get_instance_mut(&mut self, component_id: &ComponentId) -> Option<&mut ComponentInstance> {
        self.instances.get_mut(component_id)
    }

    /// Mark a component for update
    pub fn mark_component_for_update(&mut self, component_id: &ComponentId) {
        if let Some(instance) = self.instances.get_mut(component_id) {
            instance.mark_for_update();
        }
    }

    /// Get all components that need updates
    pub fn get_components_needing_update(&self) -> Vec<ComponentId> {
        self.instances
            .values()
            .filter(|instance| instance.needs_update())
            .map(|instance| instance.id.clone())
            .collect()
    }

    /// Get all root components
    pub fn get_root_components(&self) -> &[ComponentId] {
        &self.root_components
    }

    /// Get children of a component
    pub fn get_children(&self, component_id: &ComponentId) -> Vec<ComponentId> {
        self.component_hierarchy
            .get(component_id)
            .cloned()
            .unwrap_or_default()
    }

    /// Get parent of a component
    pub fn get_parent(&self, component_id: &ComponentId) -> Option<ComponentId> {
        self.instances
            .get(component_id)
            .and_then(|instance| instance.parent.clone())
    }

    /// Add parent-child relationship
    pub fn add_parent_child_relationship(
        &mut self,
        parent_id: &ComponentId,
        child_id: &ComponentId,
    ) -> Result<()> {
        // Update child's parent
        if let Some(child_instance) = self.instances.get_mut(child_id) {
            child_instance.set_parent(parent_id.clone());
        } else {
            return Err(TuiError::component(format!(
                "Child component {child_id} not found"
            )));
        }

        // Update parent's children
        if let Some(parent_instance) = self.instances.get_mut(parent_id) {
            parent_instance.add_child(child_id.clone());
        } else {
            return Err(TuiError::component(format!(
                "Parent component {parent_id} not found"
            )));
        }

        // Update hierarchy
        self.component_hierarchy
            .entry(parent_id.clone())
            .or_default()
            .push(child_id.clone());

        // Remove child from root components if it was there
        self.root_components.retain(|id| id != child_id);

        Ok(())
    }

    /// Get total number of components
    pub fn component_count(&self) -> usize {
        self.instances.len()
    }

    /// Check if a component exists
    pub fn has_component(&self, component_id: &ComponentId) -> bool {
        self.instances.contains_key(component_id)
    }

    /// Process all pending component updates
    pub async fn process_pending_updates(&mut self) -> Result<Vec<ComponentId>> {
        let components_to_update = self.get_components_needing_update();
        let mut updated_components = Vec::new();

        for component_id in components_to_update {
            match self.update_component(&component_id).await {
                Ok(true) => {
                    updated_components.push(component_id.clone());
                    self.clear_update_flag(&component_id);
                }
                Ok(false) => {
                    // Component didn't need re-render
                    self.clear_update_flag(&component_id);
                }
                Err(e) => {
                    eprintln!("Failed to update component {component_id}: {e}");
                    self.clear_update_flag(&component_id);
                }
            }
        }

        Ok(updated_components)
    }

    /// Get component lifecycle state
    pub fn get_component_state(&self, component_id: &ComponentId) -> Option<crate::components::ComponentState> {
        self.instances.get(component_id).map(|instance| {
            if instance.mounted {
                crate::components::ComponentState::Mounted
            } else {
                crate::components::ComponentState::Unmounted
            }
        })
    }

    /// Check if component is mounted
    pub fn is_component_mounted(&self, component_id: &ComponentId) -> bool {
        self.instances
            .get(component_id)
            .map(|instance| instance.mounted)
            .unwrap_or(false)
    }

    /// Get component render statistics
    pub fn get_component_stats(&self, component_id: &ComponentId) -> Option<ComponentStats> {
        self.instances.get(component_id).map(|instance| ComponentStats {
            render_count: instance.render_count,
            last_render: instance.last_render,
            mounted: instance.mounted,
            needs_update: instance.needs_update(),
        })
    }
}

/// Component statistics for monitoring
#[derive(Debug, Clone)]
pub struct ComponentStats {
    pub render_count: u64,
    pub last_render: Option<std::time::Instant>,
    pub mounted: bool,
    pub needs_update: bool,
}

impl Default for ComponentInstanceManager {
    fn default() -> Self {
        Self::new()
    }
}
