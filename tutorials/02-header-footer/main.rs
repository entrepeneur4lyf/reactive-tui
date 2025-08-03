use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("📱 Tutorial 2: Header/Footer Layout\n");

    // Get terminal dimensions
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((80, 24));
    
    // Create layout engine
    let mut engine = LayoutEngine::with_dimensions(term_width, term_height);
    
    // Load theme and utility processor
    let theme = dark_theme();
    let utility = UtilityProcessor::new();

    // Create header bar
    let header = header_bar("app-header")
        .left("📊 Dashboard App")
        .center("Main View")
        .right("User: Admin")
        .class("bg-blue-900")
        .class("text-blue-100")
        .class("font-bold")
        .class("border-b")
        .class("border-blue-700")
        .build();

    // Create main content area
    let content_area = Element::with_tag("div")
        .id("main-content")
        .class("bg-gray-800")
        .class("text-gray-100")
        .class("p-4")
        .class("flex-1")  // Take remaining space
        .child(
            // Content header
            Element::with_tag("div")
                .id("content-header")
                .content("📈 Application Dashboard")
                .class("text-white")
                .class("font-bold")
                .class("text-xl")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            // Stats section
            Element::with_tag("div")
                .id("stats-section")
                .class("grid")
                .class("grid-cols-3")
                .class("gap-4")
                .class("mb-4")
                .child(
                    // Stat card 1
                    Element::with_tag("div")
                        .class("bg-green-800")
                        .class("border")
                        .class("border-green-600")
                        .class("p-3")
                        .class("rounded")
                        .child(
                            Element::with_tag("div")
                                .content("✅ Active Users")
                                .class("text-green-200")
                                .class("font-semibold")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("1,247")
                                .class("text-green-100")
                                .class("text-2xl")
                                .class("font-bold")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    // Stat card 2
                    Element::with_tag("div")
                        .class("bg-blue-800")
                        .class("border")
                        .class("border-blue-600")
                        .class("p-3")
                        .class("rounded")
                        .child(
                            Element::with_tag("div")
                                .content("📊 Total Sales")
                                .class("text-blue-200")
                                .class("font-semibold")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("$45,230")
                                .class("text-blue-100")
                                .class("text-2xl")
                                .class("font-bold")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    // Stat card 3
                    Element::with_tag("div")
                        .class("bg-purple-800")
                        .class("border")
                        .class("border-purple-600")
                        .class("p-3")
                        .class("rounded")
                        .child(
                            Element::with_tag("div")
                                .content("⚡ Performance")
                                .class("text-purple-200")
                                .class("font-semibold")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("98.5%")
                                .class("text-purple-100")
                                .class("text-2xl")
                                .class("font-bold")
                                .into(),
                        )
                        .into(),
                )
                .into(),
        )
        .child(
            // Recent activity
            Element::with_tag("div")
                .id("recent-activity")
                .class("bg-gray-700")
                .class("border")
                .class("border-gray-600")
                .class("p-3")
                .class("rounded")
                .child(
                    Element::with_tag("div")
                        .content("📋 Recent Activity")
                        .class("text-gray-200")
                        .class("font-bold")
                        .class("mb-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• User john.doe logged in")
                        .class("text-gray-300")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• New order #1234 received")
                        .class("text-gray-300")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("• System backup completed")
                        .class("text-gray-300")
                        .into(),
                )
                .into(),
        )
        .build();

    // Create footer bar
    let footer = footer_bar("app-footer")
        .left("Ready")
        .center("Reactive TUI v0.1.0")
        .right("CPU: 12% | RAM: 45%")
        .class("bg-gray-900")
        .class("text-gray-300")
        .class("border-t")
        .class("border-gray-700")
        .build();

    // Create main application layout
    let app_layout = Element::with_tag("div")
        .id("app")
        .class("h-full")
        .class("w-full")
        .class("flex")
        .class("flex-col")
        .class("bg-gray-900")
        .child(header.into())
        .child(content_area.into())
        .child(footer.into())
        .build();

    // Compute layout
    let layout = engine.compute_layout(&app_layout)?;
    
    // Create layout rect for rendering
    let layout_rect = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: term_height,
    };

    // Render the application
    println!("🎨 Rendered Application:");
    println!("{}", app_layout.render(&layout_rect, Some(&theme)));
    
    // Show layout structure
    println!("\n📐 Layout Structure:");
    println!("┌─ Header Bar (Blue theme)");
    println!("├─ Content Area (Gray theme with stats grid)");
    println!("└─ Footer Bar (Dark theme with status)");
    
    // Show CSS classes being used
    println!("\n📋 CSS Classes Demonstration:");
    let header_classes = vec![
        "bg-blue-900".to_string(),
        "text-blue-100".to_string(),
        "font-bold".to_string(),
        "border-b".to_string(),
    ];
    
    let content_classes = vec![
        "bg-gray-800".to_string(),
        "grid".to_string(),
        "grid-cols-3".to_string(),
        "gap-4".to_string(),
    ];
    
    println!("Header classes: {}", utility.process_classes(&header_classes));
    println!("Content classes: {}", utility.process_classes(&content_classes));
    
    println!("\n✅ Tutorial 2 Complete!");
    println!("📚 Next: Tutorial 3 - Sidebar Layout for file managers");

    Ok(())
}
