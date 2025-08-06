//! # Sixel Image Demo
//!
//! Demonstrates the Image widget with Sixel rendering capabilities.
//! This example shows how to display images in terminals that support Sixel graphics.

#[cfg(feature = "images")]
use reactive_tui::widgets::image::*;
#[cfg(feature = "images")]
use reactive_tui::widgets::TerminalCapability;

fn main() {
    #[cfg(feature = "images")]
    {
        // Check if we're running in a Sixel-capable terminal
        let terminal_capability = TerminalCapability::detect();
        println!("Detected terminal capability: {:?}", terminal_capability);

        // Create an image widget - this will automatically use Sixel rendering
        // in compatible terminals, with graceful fallbacks
        let image_widget = image("demo-image", |config| {
            config
                .source_file("assets/logo.png") // You can replace this with any image file
                .width(80)
                .height(40)
                .scaling(ScalingMode::Fit)
                .fallback(FallbackMode::AsciiArt)
                .alt_text("Demo Image")
        });

        println!("Image widget created successfully!");
        println!("In a Sixel-capable terminal, this would render the image directly.");
        println!("In other terminals, it would fall back to ASCII art representation.");
    }

    #[cfg(not(feature = "images"))]
    {
        println!("Image feature not enabled. Compile with --features images to enable Sixel support.");
    }
}
