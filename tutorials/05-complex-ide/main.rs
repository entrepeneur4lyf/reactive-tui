use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("💻 Tutorial 5: Complex IDE Layout\n");

    // Get terminal dimensions (IDE needs more space)
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((160, 40));

    // Create layout engine
    let mut engine = LayoutEngine::with_dimensions(term_width, term_height);

    // Load theme and utility processor
    let theme = dark_theme();
    let utility = UtilityProcessor::new();

    // Create menu bar
    let menu_bar = header_bar("ide-menu")
        .left("📁 File | ✏️ Edit | 🔍 View | 🔧 Tools")
        .center("🚀 Reactive IDE Pro")
        .right("🔌 Extensions | ⚙️ Settings")
        .class("bg-gray-900")
        .class("text-gray-200")
        .class("font-semibold")
        .class("border-b")
        .class("border-gray-700")
        .build();

    // Create toolbar
    let toolbar = Element::with_tag("div")
        .id("toolbar")
        .class("bg-gray-800")
        .class("border-b")
        .class("border-gray-700")
        .class("p-2")
        .class("flex")
        .class("items-center")
        .class("gap-2")
        .child(
            Element::with_tag("div")
                .content("▶️")
                .class("text-green-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("rounded")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("⏸️")
                .class("text-yellow-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("rounded")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("⏹️")
                .class("text-red-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("rounded")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("|")
                .class("text-gray-600")
                .class("mx-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("💾")
                .class("text-blue-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("rounded")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🔄")
                .class("text-purple-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("rounded")
                .into(),
        )
        .build();

    // Create main IDE workspace
    let workspace = Element::with_tag("div")
        .id("workspace")
        .class("flex")
        .class("flex-1")
        .class("bg-gray-900")
        .child(
            // Left sidebar (file explorer)
            create_file_explorer().into(),
        )
        .child(
            // Main editor area
            Element::with_tag("div")
                .id("editor-area")
                .class("flex-1")
                .class("flex")
                .class("flex-col")
                .child(create_editor_tabs().into())
                .child(create_editor_content().into())
                .into(),
        )
        .child(
            // Right sidebar (outline/properties)
            create_right_sidebar().into(),
        )
        .build();

    // Create bottom panel (terminal/output)
    let bottom_panel = create_bottom_panel();

    // Create status bar
    let status_bar = footer_bar("ide-status")
        .left("🟢 Ready | 📄 main.rs | 🔤 UTF-8")
        .center("Ln 42, Col 15 | 🎯 Rust")
        .right("🔧 LSP Connected | 🚀 Git: main")
        .class("bg-blue-900")
        .class("text-blue-100")
        .class("border-t")
        .class("border-blue-700")
        .build();

    // Create main IDE layout
    let ide_layout = Element::with_tag("div")
        .id("ide")
        .class("h-full")
        .class("w-full")
        .class("flex")
        .class("flex-col")
        .class("bg-gray-900")
        .child(menu_bar.into())
        .child(toolbar.into())
        .child(
            // Main content area (workspace + bottom panel)
            Element::with_tag("div")
                .class("flex")
                .class("flex-col")
                .class("flex-1")
                .child(workspace.into())
                .child(bottom_panel.into())
                .into(),
        )
        .child(status_bar.into())
        .build();

    // Compute layout
    let layout = engine.compute_layout(&ide_layout)?;

    // Create layout rect for rendering
    let layout_rect = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: term_height,
    };

    // Render the application
    println!("🎨 Rendered Application:");
    println!("{}", ide_layout.render(&layout_rect, Some(&theme)));

    // Show layout structure
    println!("\n📐 Complex IDE Layout Structure:");
    println!("┌─ Menu Bar (File, Edit, View, Tools)");
    println!("├─ Toolbar (Run, Debug, Save controls)");
    println!("├─ Main Workspace (Horizontal flex)");
    println!("│  ├─ File Explorer Sidebar (20% width)");
    println!("│  ├─ Editor Area (60% width)");
    println!("│  │  ├─ Editor Tabs");
    println!("│  │  └─ Code Editor Content");
    println!("│  └─ Right Sidebar (20% width, Outline)");
    println!("├─ Bottom Panel (Terminal/Output)");
    println!("└─ Status Bar (File info, cursor position)");

    // Show CSS classes being used
    println!("\n📋 Advanced CSS Classes:");
    let complex_classes = vec![
        "flex".to_string(),
        "flex-col".to_string(),
        "flex-1".to_string(),
        "grid".to_string(),
        "grid-cols-5".to_string(),
        "gap-2".to_string(),
        "hover:bg-gray-700".to_string(),
    ];

    println!("Complex layout classes: {}", utility.process_classes(&complex_classes));

    println!("\n🎉 Congratulations! You've completed all 5 tutorials!");
    println!("🏆 You now know how to create professional TUI layouts with Reactive TUI!");

    Ok(())
}

// Helper function to create file explorer
fn create_file_explorer() -> Element {
    Element::with_tag("div")
        .id("file-explorer")
        .class("bg-gray-800")
        .class("border-r")
        .class("border-gray-700")
        .class("w-1/5")  // 20% width
        .class("p-3")
        .child(
            Element::with_tag("div")
                .content("📁 Explorer")
                .class("text-gray-200")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📂 src/")
                .class("text-blue-300")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("  📄 main.rs")
                .class("text-green-300")
                .class("cursor-pointer")
                .class("bg-gray-700")  // Selected
                .class("border-l-2")
                .class("border-green-500")
                .class("p-1")
                .class("ml-2")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("  📄 lib.rs")
                .class("text-gray-300")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("ml-2")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📂 tests/")
                .class("text-blue-300")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📄 Cargo.toml")
                .class("text-yellow-300")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .into(),
        )
        .build()
}

// Helper function to create editor tabs
fn create_editor_tabs() -> Element {
    Element::with_tag("div")
        .id("editor-tabs")
        .class("bg-gray-800")
        .class("border-b")
        .class("border-gray-700")
        .class("flex")
        .class("items-center")
        .child(
            Element::with_tag("div")
                .class("bg-gray-700")
                .class("text-white")
                .class("px-4")
                .class("py-2")
                .class("border-r")
                .class("border-gray-600")
                .class("flex")
                .class("items-center")
                .child(
                    Element::with_tag("div")
                        .content("📄 main.rs")
                        .class("mr-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("✕")
                        .class("text-gray-400")
                        .class("hover:text-white")
                        .class("cursor-pointer")
                        .into(),
                )
                .into(),
        )
        .child(
            Element::with_tag("div")
                .class("bg-gray-800")
                .class("text-gray-300")
                .class("px-4")
                .class("py-2")
                .class("border-r")
                .class("border-gray-600")
                .class("hover:bg-gray-700")
                .class("cursor-pointer")
                .class("flex")
                .class("items-center")
                .child(
                    Element::with_tag("div")
                        .content("📄 lib.rs")
                        .class("mr-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("✕")
                        .class("text-gray-500")
                        .class("hover:text-gray-300")
                        .class("cursor-pointer")
                        .into(),
                )
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("➕")
                .class("text-gray-500")
                .class("hover:text-gray-300")
                .class("cursor-pointer")
                .class("px-3")
                .class("py-2")
                .into(),
        )
        .build()
}

// Helper function to create editor content
fn create_editor_content() -> Element {
    Element::with_tag("div")
        .id("editor-content")
        .class("bg-gray-900")
        .class("flex-1")
        .class("p-4")
        .class("font-mono")
        .child(
            Element::with_tag("div")
                .content("1  use reactive_tui::prelude::*;")
                .class("text-gray-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("2  ")
                .class("text-gray-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("3  fn main() -> Result<()> {")
                .class("text-blue-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("4      println!(\"Hello, Reactive TUI!\");")
                .class("text-green-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("5      Ok(())")
                .class("text-purple-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("6  }")
                .class("text-blue-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("7  █")  // Cursor
                .class("text-white")
                .class("bg-white")
                .class("mb-1")
                .into(),
        )
        .build()
}

// Helper function to create right sidebar
fn create_right_sidebar() -> Element {
    Element::with_tag("div")
        .id("right-sidebar")
        .class("bg-gray-800")
        .class("border-l")
        .class("border-gray-700")
        .class("w-1/5")  // 20% width
        .class("p-3")
        .child(
            Element::with_tag("div")
                .content("🗂️ Outline")
                .class("text-gray-200")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🔧 main()")
                .class("text-blue-300")
                .class("cursor-pointer")
                .class("hover:bg-gray-700")
                .class("p-1")
                .class("mb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📋 Properties")
                .class("text-gray-200")
                .class("font-bold")
                .class("mb-2")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("Type: Function")
                .class("text-gray-400")
                .class("text-sm")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("Returns: Result<()>")
                .class("text-gray-400")
                .class("text-sm")
                .into(),
        )
        .build()
}

// Helper function to create bottom panel
fn create_bottom_panel() -> Element {
    Element::with_tag("div")
        .id("bottom-panel")
        .class("bg-gray-800")
        .class("border-t")
        .class("border-gray-700")
        .class("h-1/4")  // 25% height
        .child(
            // Panel tabs
            Element::with_tag("div")
                .class("bg-gray-800")
                .class("border-b")
                .class("border-gray-700")
                .class("flex")
                .class("items-center")
                .child(
                    Element::with_tag("div")
                        .content("💻 Terminal")
                        .class("bg-gray-700")
                        .class("text-white")
                        .class("px-3")
                        .class("py-1")
                        .class("border-r")
                        .class("border-gray-600")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("📤 Output")
                        .class("text-gray-400")
                        .class("px-3")
                        .class("py-1")
                        .class("hover:bg-gray-700")
                        .class("cursor-pointer")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("🐛 Debug")
                        .class("text-gray-400")
                        .class("px-3")
                        .class("py-1")
                        .class("hover:bg-gray-700")
                        .class("cursor-pointer")
                        .into(),
                )
                .into(),
        )
        .child(
            // Terminal content
            Element::with_tag("div")
                .class("bg-black")
                .class("text-green-400")
                .class("p-3")
                .class("font-mono")
                .class("flex-1")
                .child(
                    Element::with_tag("div")
                        .content("$ cargo run")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("   Compiling reactive-tui v0.1.0")
                        .class("text-yellow-400")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("    Finished dev [unoptimized + debuginfo] target(s)")
                        .class("text-green-300")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("     Running `target/debug/main`")
                        .class("text-blue-400")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("Hello, Reactive TUI!")
                        .class("text-white")
                        .class("mb-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content("$ █")  // Cursor
                        .class("text-green-400")
                        .into(),
                )
                .into(),
        )
        .build()
}
