//! Integration tests for reactive component system

#[cfg(test)]
mod tests {

    use crate::{
        components::{Component, Element},
        reactive::Reactive,
        error::Result,
        integration::{
            ReactiveIntegration, ReactiveChangeEvent, ComponentId, ReactiveBinding,
            ComponentInstance, ComponentInstanceManager, ReactiveUpdateScheduler,
            UpdateCoordinator, UpdateRequest,
        },
    };
    use std::sync::Arc;
    use tokio::time::{sleep, Duration};

    /// Test component that uses reactive state
    #[derive(Debug)]
    struct TestReactiveComponent {
        counter: Arc<Reactive<i32>>,
        message: Arc<Reactive<String>>,
    }

    impl TestReactiveComponent {
        fn new(counter: Arc<Reactive<i32>>, message: Arc<Reactive<String>>) -> Self {
            Self { counter, message }
        }
    }

    impl Component for TestReactiveComponent {
        fn render(&self) -> Element {
            let count = self.counter.get();
            let msg = self.message.get();

            Element::with_tag("div")
                .class("counter-component")
                .content(&format!("Count: {} - {}", count, msg))
                .build()
        }

        fn on_mount(&mut self, context: &mut crate::components::ComponentContext) -> Result<()> {
            println!("TestReactiveComponent mounted with ID: {}", context.component_id);
            Ok(())
        }

        fn on_unmount(&mut self, context: &mut crate::components::ComponentContext) -> Result<()> {
            println!("TestReactiveComponent unmounted with ID: {}", context.component_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_reactive_integration_basic() -> Result<()> {
        // Create reactive integration system
        let (mut integration, reactive_sender) = ReactiveIntegration::new();

        // Start the integration system
        integration.start().await?;

        // Create reactive values
        let counter = Arc::new(Reactive::new(0));
        let message = Arc::new(Reactive::new("Hello".to_string()));

        // Create component
        let component = TestReactiveComponent::new(counter.clone(), message.clone());

        // Create reactive bindings
        let bindings = vec![
            ReactiveBinding::new("counter".to_string(), ComponentId::new()),
            ReactiveBinding::new("message".to_string(), ComponentId::new()),
        ];

        // Mount component
        let component_id = integration
            .mount_component(Box::new(component), bindings)
            .await?;

        // Verify component was mounted
        assert!(integration.component_manager.read().await.has_component(&component_id));

        // Change reactive values
        counter.set(5);
        message.set("World".to_string());

        // Send reactive change events
        let _ = reactive_sender.send(ReactiveChangeEvent::ValueChange {
            reactive_id: "counter".to_string(),
            field_name: "value".to_string(),
            timestamp: std::time::Instant::now(),
        });

        let _ = reactive_sender.send(ReactiveChangeEvent::ValueChange {
            reactive_id: "message".to_string(),
            field_name: "value".to_string(),
            timestamp: std::time::Instant::now(),
        });

        // Wait for processing
        sleep(Duration::from_millis(10)).await;

        // Process updates
        let updated_components = integration.process_updates().await?;

        // Should have updates for our component
        assert!(!updated_components.is_empty());

        // Verify the rendered content includes updated values
        let (_, element) = &updated_components[0];
        let content = element.content.as_ref().unwrap();
        assert!(content.contains("Count: 5"));
        assert!(content.contains("World"));

        // Unmount component
        integration.unmount_component(&component_id).await?;

        // Verify component was unmounted
        assert!(!integration.component_manager.read().await.has_component(&component_id));

        Ok(())
    }

    #[tokio::test]
    async fn test_reactive_update_scheduler() -> Result<()> {
        let mut scheduler = ReactiveUpdateScheduler::new();

        // Create update requests
        let component_id = ComponentId::new();
        let request1 = UpdateRequest::new(
            component_id.clone(),
            "test_reactive".to_string(),
            crate::integration::reactive_scheduler::UpdatePriority::Normal,
        );

        let request2 = UpdateRequest::new(
            component_id.clone(),
            "test_reactive2".to_string(),
            crate::integration::reactive_scheduler::UpdatePriority::High,
        );

        // Schedule updates
        scheduler.schedule_update(request1).await?;
        scheduler.schedule_update(request2).await?;

        // Get pending updates
        let updates = scheduler.get_pending_updates().await?;

        // Should have updates (high priority first)
        assert!(!updates.is_empty());

        // First update should be high priority
        assert_eq!(updates[0].priority, crate::integration::reactive_scheduler::UpdatePriority::High);

        Ok(())
    }

    #[tokio::test]
    async fn test_update_coordinator() -> Result<()> {
        let mut coordinator = UpdateCoordinator::new();

        let component_id = ComponentId::new();
        let reactive_id = "test_reactive".to_string();

        // Bind reactive to component
        coordinator.bind_reactive_to_component(reactive_id.clone(), component_id.clone());

        // Verify binding exists
        let components = coordinator.get_components_for_reactive(&reactive_id);
        assert_eq!(components.len(), 1);
        assert_eq!(components[0], component_id);

        let reactives = coordinator.get_reactives_for_component(&component_id);
        assert_eq!(reactives.len(), 1);
        assert_eq!(reactives[0], reactive_id);

        // Clean up component bindings
        coordinator.cleanup_component_bindings(&component_id);

        // Verify bindings were cleaned up
        let components_after = coordinator.get_components_for_reactive(&reactive_id);
        assert!(components_after.is_empty());

        Ok(())
    }

    #[tokio::test]
    async fn test_component_instance_manager() -> Result<()> {
        let mut manager = ComponentInstanceManager::new();

        // Create test component
        let counter = Arc::new(Reactive::new(0));
        let message = Arc::new(Reactive::new("Test".to_string()));
        let component = TestReactiveComponent::new(counter, message);
        let component_id = ComponentId::new();

        // Create instance
        let instance = ComponentInstance::new(component_id.clone(), Box::new(component));

        // Register instance
        manager.register_instance(instance).await?;

        // Verify instance exists
        assert!(manager.has_component(&component_id));
        assert_eq!(manager.component_count(), 1);

        // Mark for update
        manager.mark_component_for_update(&component_id);

        // Get components needing update
        let needs_update = manager.get_components_needing_update();
        assert_eq!(needs_update.len(), 1);
        assert_eq!(needs_update[0], component_id);

        // Unregister instance
        manager.unregister_instance(&component_id).await?;

        // Verify instance was removed
        assert!(!manager.has_component(&component_id));
        assert_eq!(manager.component_count(), 0);

        Ok(())
    }

    #[tokio::test]
    async fn test_reactive_integration_multiple_components() -> Result<()> {
        let (mut integration, reactive_sender) = ReactiveIntegration::new();
        integration.start().await?;

        // Create shared reactive value
        let shared_counter = Arc::new(Reactive::new(0));

        // Create multiple components using the same reactive value
        let component1 = TestReactiveComponent::new(
            shared_counter.clone(),
            Arc::new(Reactive::new("Component 1".to_string())),
        );
        let component2 = TestReactiveComponent::new(
            shared_counter.clone(),
            Arc::new(Reactive::new("Component 2".to_string())),
        );

        // Mount both components
        let bindings = vec![ReactiveBinding::new("shared_counter".to_string(), ComponentId::new())];

        let component_id1 = integration
            .mount_component(Box::new(component1), bindings.clone())
            .await?;
        let component_id2 = integration
            .mount_component(Box::new(component2), bindings)
            .await?;

        // Change shared reactive value
        shared_counter.set(42);

        // Send reactive change event
        let _ = reactive_sender.send(ReactiveChangeEvent::ValueChange {
            reactive_id: "shared_counter".to_string(),
            field_name: "value".to_string(),
            timestamp: std::time::Instant::now(),
        });

        // Wait for processing
        sleep(Duration::from_millis(10)).await;

        // Process updates
        let updated_components = integration.process_updates().await?;

        // Both components should be updated
        assert_eq!(updated_components.len(), 2);

        // Verify both components have updated content
        for (_, element) in &updated_components {
            let content = element.content.as_ref().unwrap();
            assert!(content.contains("Count: 42"));
        }

        // Clean up
        integration.unmount_component(&component_id1).await?;
        integration.unmount_component(&component_id2).await?;

        Ok(())
    }
}
