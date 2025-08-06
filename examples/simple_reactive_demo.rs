//! Simple demonstration of reactive integration
//!
//! Shows how reactive state changes trigger component re-rendering

use reactive_tui::prelude::*;
use reactive_tui::reactive::Reactive;
use std::sync::Arc;

/// Simple counter component with reactive state
struct SimpleCounter {
    count: Arc<Reactive<i32>>,
}

impl SimpleCounter {
    fn new() -> Self {
        let count = Arc::new(Reactive::new(0));

        // Add a watcher to see when the value changes
        count.watch(|old, new| {
            println!("Counter changed from {} to {}", old, new);
        });

        Self { count }
    }

    /// Get the reactive counter for external access
    pub fn counter(&self) -> Arc<Reactive<i32>> {
        self.count.clone()
    }

    /// Increment the counter
    #[allow(dead_code)]
    pub fn increment(&self) {
        let current = self.count.get();
        self.count.set(current + 1);
    }
}

impl Component for SimpleCounter {
    fn render(&self) -> Element {
        let count = self.count.get();

        Element::with_tag("div")
            .class("counter")
            .child(
                Element::with_tag("h1")
                    .content("Simple Reactive Counter")
                    .build()
            )
            .child(
                Element::with_tag("div")
                    .class("count-display")
                    .content(&format!("Count: {}", count))
                    .build()
            )
            .child(
                Element::with_tag("div")
                    .class("instructions")
                    .content("Press SPACE to increment, ESC to quit")
                    .build()
            )
            .build()
    }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
    println!("Starting Simple Reactive Demo...");

    // Create the counter component
    let counter_component = SimpleCounter::new();
    let counter = counter_component.counter();

    // Build the TUI app
    let mut app = TuiApp::builder()
        .component(counter_component)
        .with_title("Simple Reactive Demo")
        .frame_rate(30)
        .build()?;

    // Set up key bindings
    app.bind_key_to_action(
        reactive_tui::events::KeyCombination::char(' '),
        "increment"
    ).await;

    app.bind_key_to_action(
        reactive_tui::events::KeyCombination::escape(),
        "quit"
    ).await;

    // Register action handlers
    app.register_action("increment", move |_action| {
        let current = counter.get();
        counter.set(current + 1);
        println!("Incremented counter to: {}", current + 1);
        reactive_tui::events::ActionResult::Handled
    });

    app.register_action("quit", |_action| {
        println!("Quitting...");
        std::process::exit(0);
        #[allow(unreachable_code)]
        reactive_tui::events::ActionResult::Handled
    });

    println!("Demo running! Press SPACE to increment, ESC to quit");

    // Run the app
    app.run().await?;

    println!("Demo completed!");
    Ok(())
}
