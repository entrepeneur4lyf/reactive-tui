//! Integration layer connecting reactive state to component updates
//!
//! This module provides the missing integration between the reactive system and component system,
//! enabling automatic UI updates when reactive state changes.

pub mod component_manager;
pub mod reactive_scheduler;
pub mod update_coordinator;

#[cfg(test)]
mod tests;

pub use component_manager::{ComponentId, ComponentInstance, ComponentInstanceManager};
pub use reactive_scheduler::{ReactiveUpdateScheduler, UpdateRequest};
pub use update_coordinator::{ReactiveBinding, UpdateCoordinator};

use crate::components::{Component, Element};
use crate::error::Result;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

/// Unique identifier for reactive values
pub type ReactiveId = String;

/// Integration system that connects reactive state changes to component updates
pub struct ReactiveIntegration {
  component_manager: Arc<RwLock<ComponentInstanceManager>>,
  update_scheduler: Arc<RwLock<ReactiveUpdateScheduler>>,
  update_coordinator: Arc<RwLock<UpdateCoordinator>>,
  reactive_receiver: broadcast::Receiver<ReactiveChangeEvent>,
  update_sender: broadcast::Sender<UpdateRequest>,
}

/// Unified reactive change event
#[derive(Debug, Clone)]
pub enum ReactiveChangeEvent {
  ValueChange {
    reactive_id: ReactiveId,
    field_name: String,
    timestamp: std::time::Instant,
  },
  StateChange {
    field_name: String,
    timestamp: std::time::Instant,
  },
}

impl ReactiveIntegration {
  /// Create a new reactive integration system
  pub fn new() -> (Self, broadcast::Sender<ReactiveChangeEvent>) {
    let (reactive_sender, reactive_receiver) = broadcast::channel(1000);
    let (update_sender, _) = broadcast::channel(1000);

    let component_manager = Arc::new(RwLock::new(ComponentInstanceManager::new()));
    let update_scheduler = Arc::new(RwLock::new(ReactiveUpdateScheduler::new()));
    let update_coordinator = Arc::new(RwLock::new(UpdateCoordinator::new()));

    let integration = Self {
      component_manager,
      update_scheduler,
      update_coordinator,
      reactive_receiver,
      update_sender,
    };

    (integration, reactive_sender)
  }

  /// Mount a component and track its reactive dependencies
  pub async fn mount_component(
    &self,
    component: Box<dyn Component>,
    reactive_bindings: Vec<ReactiveBinding>,
  ) -> Result<ComponentId> {
    let component_id = ComponentId::new();

    // Create component instance
    let instance = ComponentInstance::new(component_id.clone(), component);

    // Register with component manager
    {
      let mut manager = self.component_manager.write().await;
      manager.register_instance(instance).await?;
    }

    // Register reactive bindings
    {
      let mut coordinator = self.update_coordinator.write().await;
      for binding in reactive_bindings {
        coordinator.bind_reactive_to_component(binding.reactive_id, component_id.clone());
      }
    }

    Ok(component_id)
  }

  /// Unmount a component and clean up its reactive bindings
  pub async fn unmount_component(&self, component_id: &ComponentId) -> Result<()> {
    // Remove from component manager
    {
      let mut manager = self.component_manager.write().await;
      manager.unregister_instance(component_id).await?;
    }

    // Clean up reactive bindings
    {
      let mut coordinator = self.update_coordinator.write().await;
      coordinator.cleanup_component_bindings(component_id);
    }

    Ok(())
  }

  /// Start the reactive integration system
  pub async fn start(&mut self) -> Result<()> {
    let component_manager = self.component_manager.clone();
    let update_scheduler = self.update_scheduler.clone();
    let update_coordinator = self.update_coordinator.clone();
    let update_sender = self.update_sender.clone();

    // Spawn task to handle reactive changes
    let mut reactive_receiver = self.reactive_receiver.resubscribe();
    tokio::spawn(async move {
      while let Ok(change_event) = reactive_receiver.recv().await {
        if let Err(e) = Self::handle_reactive_change(
          &change_event,
          &component_manager,
          &update_scheduler,
          &update_coordinator,
          &update_sender,
        )
        .await
        {
          eprintln!("Error handling reactive change: {e}");
        }
      }
    });

    Ok(())
  }

  /// Handle a reactive change event
  async fn handle_reactive_change(
    change_event: &ReactiveChangeEvent,
    _component_manager: &Arc<RwLock<ComponentInstanceManager>>,
    update_scheduler: &Arc<RwLock<ReactiveUpdateScheduler>>,
    update_coordinator: &Arc<RwLock<UpdateCoordinator>>,
    update_sender: &broadcast::Sender<UpdateRequest>,
  ) -> Result<()> {
    let reactive_id = match change_event {
      ReactiveChangeEvent::ValueChange { reactive_id, .. } => reactive_id.clone(),
      ReactiveChangeEvent::StateChange { field_name, .. } => field_name.clone(),
    };

    // Find components affected by this reactive change
    let affected_components = {
      let coordinator = update_coordinator.read().await;
      coordinator.get_components_for_reactive(&reactive_id)
    };

    // Schedule updates for affected components
    for component_id in affected_components {
      let update_request = UpdateRequest {
        component_id: component_id.clone(),
        reactive_id: reactive_id.clone(),
        timestamp: std::time::Instant::now(),
        priority: crate::integration::reactive_scheduler::UpdatePriority::Normal,
      };

      // Add to scheduler
      {
        let mut scheduler = update_scheduler.write().await;
        scheduler.schedule_update(update_request.clone()).await?;
      }

      // Send update notification
      let _ = update_sender.send(update_request);
    }

    Ok(())
  }

  /// Get update receiver for the app to listen to
  pub fn subscribe_to_updates(&self) -> broadcast::Receiver<UpdateRequest> {
    self.update_sender.subscribe()
  }

  /// Process pending updates and return components that need re-rendering
  pub async fn process_updates(&self) -> Result<Vec<(ComponentId, Element)>> {
    let mut results = Vec::new();

    // Get pending updates from scheduler
    let pending_updates = {
      let mut scheduler = self.update_scheduler.write().await;
      scheduler.get_pending_updates().await?
    };

    // Process each update
    for update_request in pending_updates {
      let element = {
        let manager = self.component_manager.read().await;
        manager
          .get_instance(&update_request.component_id)
          .map(|instance| instance.component.render())
      };

      if let Some(element) = element {
        results.push((update_request.component_id, element));
      }
    }

    Ok(results)
  }
}

impl Default for ReactiveIntegration {
  fn default() -> Self {
    Self::new().0
  }
}
