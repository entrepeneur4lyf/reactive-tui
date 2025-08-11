//! Demonstration of reactive integration system
//!
//! This example shows how reactive state changes automatically trigger component updates

use reactive_tui::integration::{ComponentId, ReactiveBinding, ReactiveChangeEvent};
use reactive_tui::prelude::*;
use reactive_tui::reactive::Reactive;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::sleep;

/// A counter component that uses reactive state
#[derive(Debug)]
struct CounterComponent {
  counter: Arc<Reactive<i32>>,
  message: Arc<Reactive<String>>,
}

impl CounterComponent {
  fn new() -> Self {
    Self {
      counter: Arc::new(Reactive::new(0)),
      message: Arc::new(Reactive::new("Hello, Reactive TUI!".to_string())),
    }
  }

  /// Get the reactive counter for external access
  pub fn counter(&self) -> Arc<Reactive<i32>> {
    self.counter.clone()
  }

  /// Get the reactive message for external access
  pub fn message(&self) -> Arc<Reactive<String>> {
    self.message.clone()
  }

  /// Increment the counter
  #[allow(dead_code)]
  pub fn increment(&self) {
    let current = self.counter.get();
    self.counter.set(current + 1);
    self.message.set(format!("Count is now: {}", current + 1));
  }
}

impl Component for CounterComponent {
  fn render(&self) -> Element {
    let count = self.counter.get();
    let msg = self.message.get();

    Element::with_tag("div")
      .class("counter-container")
      .child(
        Element::with_tag("h1")
          .content("Reactive Counter Demo")
          .build(),
      )
      .child(
        Element::with_tag("div")
          .class("counter-display")
          .content(&format!("Count: {}", count))
          .build(),
      )
      .child(
        Element::with_tag("div")
          .class("message-display")
          .content(&msg)
          .build(),
      )
      .child(
        Element::with_tag("div")
          .class("instructions")
          .content("Press 'q' to quit, 'i' to increment")
          .build(),
      )
      .build()
  }

  fn on_mount(
    &mut self,
    _context: &mut reactive_tui::components::ComponentContext,
  ) -> reactive_tui::error::Result<()> {
    println!("CounterComponent mounted!");
    Ok(())
  }

  fn on_unmount(
    &mut self,
    _context: &mut reactive_tui::components::ComponentContext,
  ) -> reactive_tui::error::Result<()> {
    println!("CounterComponent unmounted!");
    Ok(())
  }
}

/// Demo application that shows reactive integration
struct ReactiveDemo {
  counter_component: CounterComponent,
}

impl ReactiveDemo {
  fn new() -> Self {
    Self {
      counter_component: CounterComponent::new(),
    }
  }

  /// Get the counter component for external access
  pub fn counter_component(&self) -> &CounterComponent {
    &self.counter_component
  }
}

impl Component for ReactiveDemo {
  fn render(&self) -> Element {
    Element::with_tag("div")
      .class("app-container")
      .child(self.counter_component.render())
      .build()
  }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
  println!("Starting Reactive Integration Demo...");

  // Create the demo app
  let demo = ReactiveDemo::new();

  // Get references to reactive values for external manipulation
  let counter = demo.counter_component().counter();
  let message = demo.counter_component().message();

  // Build the TUI app
  let mut app = TuiApp::builder()
    .component(demo)
    .with_title("Reactive Integration Demo")
    .frame_rate(60) // Higher frame rate for responsive updates
    .build()?;

  // Create reactive bindings for the counter component
  let counter_bindings = vec![
    ReactiveBinding::new("counter".to_string(), ComponentId::new()),
    ReactiveBinding::new("message".to_string(), ComponentId::new()),
  ];

  // Mount the counter component with reactive bindings
  let counter_component = CounterComponent::new();
  let component_id = app
    .mount_reactive_component(Box::new(counter_component), counter_bindings)
    .await?;

  println!("Component mounted with ID: {}", component_id);

  // Get the reactive change sender
  let reactive_sender = app.get_reactive_change_sender().clone();

  // Spawn a task to simulate reactive changes
  let counter_clone = counter.clone();
  let message_clone = message.clone();
  let sender_clone = reactive_sender.clone();

  tokio::spawn(async move {
    let mut count = 0;
    loop {
      sleep(Duration::from_secs(2)).await;

      count += 1;
      counter_clone.set(count);
      message_clone.set(format!("Auto-updated: {}", count));

      // Send reactive change events
      let _ = sender_clone.send(ReactiveChangeEvent::ValueChange {
        reactive_id: "counter".to_string(),
        field_name: "value".to_string(),
        timestamp: std::time::Instant::now(),
      });

      let _ = sender_clone.send(ReactiveChangeEvent::ValueChange {
        reactive_id: "message".to_string(),
        field_name: "value".to_string(),
        timestamp: std::time::Instant::now(),
      });

      println!(
        "Reactive values updated: count={}, message={}",
        count,
        message_clone.get()
      );

      if count >= 10 {
        break;
      }
    }
  });

  // Set up key bindings for manual interaction
  app
    .bind_key_to_action(
      reactive_tui::events::KeyCombination::char('i'),
      "increment_counter",
    )
    .await;

  app
    .bind_key_to_action(reactive_tui::events::KeyCombination::char('q'), "quit")
    .await;

  // Register action handlers
  app.register_action("increment_counter", move |_action| {
    let current = counter.get();
    counter.set(current + 1);
    message.set(format!("Manually incremented: {}", current + 1));

    // Send reactive change event
    let _ = reactive_sender.send(ReactiveChangeEvent::ValueChange {
      reactive_id: "counter".to_string(),
      field_name: "value".to_string(),
      timestamp: std::time::Instant::now(),
    });

    reactive_tui::events::ActionResult::Handled
  });

  app.register_action("quit", |_action| {
    println!("Quitting demo...");
    std::process::exit(0);
    #[allow(unreachable_code)]
    reactive_tui::events::ActionResult::Handled
  });

  println!("Demo running! Press 'i' to increment manually, 'q' to quit");
  println!("The counter will also auto-increment every 2 seconds");

  // Run the app
  app.run().await?;

  println!("Demo completed!");
  Ok(())
}
