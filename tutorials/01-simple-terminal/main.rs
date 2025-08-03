use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("🖥️  Tutorial 1: Simple Terminal Layout\n");

    // Get terminal dimensions
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 24));
    
    // Create layout engine
    let mut engine = LayoutEngine::with_dimensions(term_width, term_height);
    
    // Load dark theme for colors
    let theme = dark_theme();
    let utility = UtilityProcessor::new();

    // Create a simple terminal application container
    let app_container = Element::with_tag("div")
        .id("terminal-app")
        .class("bg-gray-900")      // Dark background
        .class("text-green-400")   // Green terminal text
        .class("p-4")              // Padding
        .class("border")           // Border
        .class("border-green-500") // Green border
        .class("h-full")           // Full height
        .class("w-full")           // Full width
        .child(
            // Title section
            Element::with_tag("div")
                .id("title")
                .content("🖥️  Simple Terminal Application")
                .class("text-green-300")
                .class("font-bold")
                .class("text-center")
                .class("mb-2")
                .class("border-b")
                .class("border-green-600")
                .class("pb-1")
                .into(),
        )
        .child(
            // Welcome message
            Element::with_tag("div")
                .id("welcome")
                .content("Welcome to your first Reactive TUI application!")
                .class("text-green-400")
                .class("mb-2")
                .into(),
        )
        .child(
            // Status section
            Element::with_tag("div")
                .id("status")
                .content("Status: ✅ Running")
                .class("text-green-300")
                .class("mb-2")
                .into(),
        )
        .child(
            // Info box
            Element::with_tag("div")
                .id("info-box")
                .class("bg-gray-800")
                .class("border")
                .class("border-blue-500")
                .class("p-2")
                .class("mb-2")
                .child(
                    Element::with_tag("div")
                        .content("💡 This is a styled info box with:")
                        .class("text-blue-300")
                        .class("font-bold")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• Custom background color")
                        .class("text-blue-200")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• Blue border styling")
                        .class("text-blue-200")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• Proper padding and margins")
                        .class("text-blue-200")
                        .into(),
                )
                .into(),
        )
        .child(
            // Command prompt simulation
            Element::with_tag("div")
                .id("prompt")
                .content("$ reactive-tui --version")
                .class("text-yellow-400")
                .class("font-mono")
                .class("bg-gray-800")
                .class("p-1")
                .class("border-l-4")
                .class("border-yellow-500")
                .into(),
        )
        .child(
            // Output
            Element::with_tag("div")
                .id("output")
                .content("reactive-tui 0.1.0")
                .class("text-gray-300")
                .class("font-mono")
                .class("ml-2")
                .class("mt-1")
                .into(),
        )
        .build();

    // Compute layout
    let layout = engine.compute_layout(&app_container)?;
    
    // Create layout rect for rendering
    let layout_rect = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: term_height,
    };

    // Render the application
    println!("🎨 Rendered Application:");
    println!("{}", app_container.render(&layout_rect, Some(&theme)));
    
    // Show CSS classes being used
    println!("\n📋 CSS Classes Demonstration:");
    let demo_classes = vec![
        "bg-gray-900".to_string(),
        "text-green-400".to_string(),
        "border".to_string(),
        "border-green-500".to_string(),
        "p-4".to_string(),
    ];
    
    println!("Applied classes: {}", utility.process_classes(&demo_classes));
    
    println!("\n✅ Tutorial 1 Complete!");
    println!("📚 Next: Tutorial 2 - Header/Footer Layout with Bar widgets");

    Ok(())
}
