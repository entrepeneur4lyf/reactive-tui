//! # Sixel Image Demo
//!
//! Demonstrates the Image widget with Sixel rendering capabilities.
//! This example shows how to display images in terminals that support Sixel graphics.

use reactive_tui::prelude::*;
use reactive_tui::widgets::image::*;
use reactive_tui::widgets::TerminalCapability;

/// Simple component that displays an image
struct ImageDemo;

impl Component for ImageDemo {
    fn render(&self) -> Element {
        // Create an image widget - this will automatically use Sixel rendering
        // in compatible terminals, with graceful fallbacks
        Element::with_tag("div")
            .class("image-demo")
            .child(
                Element::with_tag("h1")
                    .content("Sixel Image Demo")
                    .build()
            )
            .child(
                image("demo-image", |config| {
                    config
                        .source_file("assets/logo.png")
                        .width(80)
                        .height(40)
                        .scaling(ScalingMode::Fit)
                        .fallback(FallbackMode::AsciiArt)
                        .alt_text("Demo Image")
                })
            )
            .child(
                Element::with_tag("p")
                    .content("Press ESC to quit")
                    .build()
            )
            .build()
    }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
    // Check if we're running in a Sixel-capable terminal
    let terminal_capability = TerminalCapability::detect();
    println!("Detected terminal capability: {:?}", terminal_capability);
    println!("Starting image demo...");

    // Build the TUI app
    let mut app = TuiApp::builder()
        .component(ImageDemo)
        .with_title("Sixel Image Demo")
        .build()?;

    // Set up key binding to quit
    app.bind_key_to_action(
        reactive_tui::events::KeyCombination::escape(),
        "quit"
    ).await;

    app.register_action("quit", |_action| {
        println!("Quitting...");
        std::process::exit(0);
        #[allow(unreachable_code)]
        reactive_tui::events::ActionResult::Handled
    });

    // Run the app
    app.run().await?;

    Ok(())
}
