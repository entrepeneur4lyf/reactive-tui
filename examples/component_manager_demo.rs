//! Component Manager Demo
//!
//! Demonstrates the working ComponentInstanceManager with full lifecycle support.

use reactive_tui::prelude::*;
use reactive_tui::components::ComponentContext;
use reactive_tui::integration::component_manager::{
    ComponentInstanceManager, ComponentInstance, ComponentId, LifecycleEvent
};
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone)]
struct TestComponent {
    name: String,
    _mount_count: Arc<Mutex<u32>>,
}

impl TestComponent {
    fn new(name: String) -> Self {
        Self {
            name,
            _mount_count: Arc::new(Mutex::new(0)),
        }
    }
}

impl Component for TestComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("test-component")
            .content(format!("Test Component: {}", self.name))
            .build()
    }

    fn on_mount(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<()> {
        println!("MOUNTED: Component '{}' mounted successfully", self.name);
        let _ = context; // Use context to avoid warning
        Ok(())
    }

    fn on_before_mount(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<()> {
        println!("BEFORE_MOUNT: Component '{}' preparing to mount", self.name);
        let _ = context;
        Ok(())
    }

    fn on_after_mount(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<()> {
        println!("AFTER_MOUNT: Component '{}' fully mounted", self.name);
        let _ = context;
        Ok(())
    }

    fn on_update(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<bool> {
        println!("UPDATE: Component '{}' updating", self.name);
        let _ = context;
        Ok(true)
    }

    fn on_before_unmount(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<()> {
        println!("BEFORE_UNMOUNT: Component '{}' preparing to unmount", self.name);
        let _ = context;
        Ok(())
    }

    fn on_unmount(&mut self, context: &mut ComponentContext) -> reactive_tui::error::Result<()> {
        println!("UNMOUNTED: Component '{}' unmounted successfully", self.name);
        let _ = context;
        Ok(())
    }

    fn on_error(&mut self, context: &mut ComponentContext, error: &reactive_tui::error::TuiError) -> reactive_tui::error::Result<()> {
        println!("ERROR: Component '{}' encountered error: {}", self.name, error);
        let _ = context;
        Ok(())
    }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
    println!("Component Manager Lifecycle Demo");
    println!("================================");

    // Create component manager with lifecycle events
    let (mut manager, mut lifecycle_receiver) = ComponentInstanceManager::with_lifecycle_events();

    // Spawn task to handle lifecycle events
    let lifecycle_handler = tokio::spawn(async move {
        while let Some(event) = lifecycle_receiver.recv().await {
            match event {
                LifecycleEvent::Mounted(id) => println!("LIFECYCLE EVENT: Component {} mounted", id),
                LifecycleEvent::Unmounted(id) => println!("LIFECYCLE EVENT: Component {} unmounted", id),
                LifecycleEvent::Updated(id) => println!("LIFECYCLE EVENT: Component {} updated", id),
                LifecycleEvent::Error(id, error) => println!("LIFECYCLE EVENT: Component {} error: {}", id, error),
            }
        }
    });

    // Create test components
    let component1 = TestComponent::new("Button".to_string());
    let component2 = TestComponent::new("Input".to_string());
    let component3 = TestComponent::new("Modal".to_string());

    // Create component instances
    let instance1 = ComponentInstance::new(
        ComponentId::new(),
        Box::new(component1)
    );
    let instance2 = ComponentInstance::new(
        ComponentId::new(),
        Box::new(component2)
    );
    let instance3 = ComponentInstance::new(
        ComponentId::new(),
        Box::new(component3)
    );

    let id1 = instance1.id.clone();
    let id2 = instance2.id.clone();
    let id3 = instance3.id.clone();

    println!("\n1. Registering components...");

    // Register components (triggers full mount lifecycle)
    manager.register_instance(instance1).await?;
    manager.register_instance(instance2).await?;
    manager.register_instance(instance3).await?;

    // Wait a moment for lifecycle events to be processed
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n2. Component stats:");
    println!("Total components: {}", manager.component_count());
    println!("Root components: {}", manager.get_root_components().len());

    if let Some(stats) = manager.get_component_stats(&id1) {
        println!("Component {} stats: render_count={}, mounted={}",
            id1, stats.render_count, stats.mounted);
    }

    println!("\n3. Updating components...");

    // Update components (triggers update lifecycle)
    let updated = manager.update_component(&id1).await?;
    println!("Component {} update result: {}", id1, updated);

    // Process all pending updates
    let updated_components = manager.process_pending_updates().await?;
    println!("Updated components: {:?}", updated_components);

    // Wait for lifecycle events
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n4. Unregistering components...");

    // Unregister components (triggers full unmount lifecycle)
    manager.unregister_instance(&id1).await?;
    manager.unregister_instance(&id2).await?;
    manager.unregister_instance(&id3).await?;

    // Wait for final lifecycle events
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

    println!("\n5. Final stats:");
    println!("Total components: {}", manager.component_count());

    // Clean shutdown
    drop(manager);
    let _ = lifecycle_handler.await;

    println!("\nComponent Manager Demo completed successfully!");

    Ok(())
}
