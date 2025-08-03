use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("📁 Tutorial 3: Sidebar Layout\n");

    // Get terminal dimensions
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((120, 30));
    
    // Create layout engine
    let mut engine = LayoutEngine::with_dimensions(term_width, term_height);
    
    // Load theme and utility processor
    let theme = dark_theme();
    let utility = UtilityProcessor::new();

    // Create header bar
    let header = header_bar("file-manager-header")
        .left("📁 File Manager Pro")
        .center("/home/user/documents")
        .right("🔍 Search")
        .class("bg-indigo-900")
        .class("text-indigo-100")
        .class("font-bold")
        .class("border-b")
        .class("border-indigo-700")
        .build();

    // Create sidebar navigation
    let sidebar = Element::with_tag("div")
        .id("sidebar")
        .class("bg-gray-900")
        .class("border-r")
        .class("border-gray-700")
        .class("w-1/4")  // 25% width
        .class("h-full")
        .class("p-2")
        .child(
            // Sidebar title
            Element::with_tag("div")
                .content("📂 Navigation")
                .class("text-gray-200")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-700")
                .class("pb-1")
                .into(),
        )
        .child(
            // Quick access section
            Element::with_tag("div")
                .id("quick-access")
                .class("mb-4")
                .child(
                    Element::with_tag("div")
                        .content("⚡ Quick Access")
                        .class("text-gray-400")
                        .class("text-sm")
                        .class("mb-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("🏠 Home")
                        .class("text-blue-300")
                        .class("p-1")
                        .class("hover:bg-gray-800")
                        .class("cursor-pointer")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("📄 Documents")
                        .class("text-green-300")
                        .class("p-1")
                        .class("bg-gray-800")  // Selected
                        .class("border-l-2")
                        .class("border-green-500")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("🖼️  Pictures")
                        .class("text-purple-300")
                        .class("p-1")
                        .class("hover:bg-gray-800")
                        .class("cursor-pointer")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("💾 Downloads")
                        .class("text-yellow-300")
                        .class("p-1")
                        .class("hover:bg-gray-800")
                        .class("cursor-pointer")
                        .into(),
                )
                .into(),
        )
        .child(
            // Folder tree section
            Element::with_tag("div")
                .id("folder-tree")
                .child(
                    Element::with_tag("div")
                        .content("📁 Folder Tree")
                        .class("text-gray-400")
                        .class("text-sm")
                        .class("mb-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("📂 projects/")
                        .class("text-gray-300")
                        .class("p-1")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("  📂 reactive-tui/")
                        .class("text-gray-300")
                        .class("p-1")
                        .class("ml-2")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("  📂 web-app/")
                        .class("text-gray-300")
                        .class("p-1")
                        .class("ml-2")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("📂 documents/")
                        .class("text-gray-300")
                        .class("p-1")
                        .into(),
                )
                .into(),
        )
        .build();

    // Create main content area
    let main_content = Element::with_tag("div")
        .id("main-content")
        .class("bg-gray-800")
        .class("flex-1")  // Take remaining space
        .class("p-4")
        .child(
            // Content header
            Element::with_tag("div")
                .id("content-header")
                .class("flex")
                .class("justify-between")
                .class("items-center")
                .class("mb-4")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .child(
                    Element::with_tag("div")
                        .content("📄 Documents (24 items)")
                        .class("text-white")
                        .class("font-bold")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("🔄 Last modified: 2 hours ago")
                        .class("text-gray-400")
                        .class("text-sm")
                        .into(),
                )
                .into(),
        )
        .child(
            // File grid
            Element::with_tag("div")
                .id("file-grid")
                .class("grid")
                .class("grid-cols-4")
                .class("gap-3")
                .child(
                    // File item 1
                    Element::with_tag("div")
                        .class("bg-gray-700")
                        .class("border")
                        .class("border-gray-600")
                        .class("p-3")
                        .class("rounded")
                        .class("hover:bg-gray-600")
                        .class("cursor-pointer")
                        .child(
                            Element::with_tag("div")
                                .content("📄")
                                .class("text-2xl")
                                .class("text-center")
                                .class("mb-2")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("report.pdf")
                                .class("text-gray-200")
                                .class("text-sm")
                                .class("text-center")
                                .class("truncate")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    // File item 2
                    Element::with_tag("div")
                        .class("bg-gray-700")
                        .class("border")
                        .class("border-gray-600")
                        .class("p-3")
                        .class("rounded")
                        .class("hover:bg-gray-600")
                        .class("cursor-pointer")
                        .child(
                            Element::with_tag("div")
                                .content("📊")
                                .class("text-2xl")
                                .class("text-center")
                                .class("mb-2")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("data.xlsx")
                                .class("text-gray-200")
                                .class("text-sm")
                                .class("text-center")
                                .class("truncate")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    // File item 3
                    Element::with_tag("div")
                        .class("bg-gray-700")
                        .class("border")
                        .class("border-gray-600")
                        .class("p-3")
                        .class("rounded")
                        .class("hover:bg-gray-600")
                        .class("cursor-pointer")
                        .child(
                            Element::with_tag("div")
                                .content("🖼️")
                                .class("text-2xl")
                                .class("text-center")
                                .class("mb-2")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("photo.jpg")
                                .class("text-gray-200")
                                .class("text-sm")
                                .class("text-center")
                                .class("truncate")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    // File item 4
                    Element::with_tag("div")
                        .class("bg-gray-700")
                        .class("border")
                        .class("border-gray-600")
                        .class("p-3")
                        .class("rounded")
                        .class("hover:bg-gray-600")
                        .class("cursor-pointer")
                        .child(
                            Element::with_tag("div")
                                .content("📝")
                                .class("text-2xl")
                                .class("text-center")
                                .class("mb-2")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .content("notes.txt")
                                .class("text-gray-200")
                                .class("text-sm")
                                .class("text-center")
                                .class("truncate")
                                .into(),
                        )
                        .into(),
                )
                .into(),
        )
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
        .child(
            // Content area with sidebar and main content
            Element::with_tag("div")
                .class("flex")
                .class("flex-1")
                .child(sidebar.into())
                .child(main_content.into())
                .into(),
        )
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
    println!("┌─ Header Bar (Indigo theme)");
    println!("├─ Content Area (Horizontal flex)");
    println!("│  ├─ Sidebar (25% width, navigation)");
    println!("│  └─ Main Content (75% width, file grid)");
    
    // Show CSS classes being used
    println!("\n📋 CSS Classes Demonstration:");
    let sidebar_classes = vec![
        "w-1/4".to_string(),
        "bg-gray-900".to_string(),
        "border-r".to_string(),
        "border-gray-700".to_string(),
    ];
    
    let grid_classes = vec![
        "grid".to_string(),
        "grid-cols-4".to_string(),
        "gap-3".to_string(),
        "hover:bg-gray-600".to_string(),
    ];
    
    println!("Sidebar classes: {}", utility.process_classes(&sidebar_classes));
    println!("Grid classes: {}", utility.process_classes(&grid_classes));
    
    println!("\n✅ Tutorial 3 Complete!");
    println!("📚 Next: Tutorial 4 - Dashboard Grid Layout");

    Ok(())
}
