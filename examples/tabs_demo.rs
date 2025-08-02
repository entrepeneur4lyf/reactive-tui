use tui_core::prelude::*;
use tui_core::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("🗂️ Tabs Component Demo\n");

    // Get terminal size dynamically
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: 10,
    };
    let theme = tui_core::themes::colors::dark_theme();
    let utility = UtilityProcessor::new();

    // Define tab styling classes
    let active_classes = vec![
        "bg-blue-600".to_string(),
        "text-white".to_string(),
        "font-bold".to_string(),
    ];

    println!(
        "Active Tab Style: {}",
        utility.process_classes(&active_classes)
    );

    // Top tabs with theme integration
    let top_tabs = horizontal_tabs("top-tabs")
        .simple_tab("home", "Home")
        .simple_tab("about", "About")
        .simple_tab("contact", "Contact")
        .active(0)
        .build();

    println!("Top Tabs (Blue Active Theme):");
    println!("{}\n", top_tabs.render(&layout, Some(&theme)));

    // Bottom tabs with different theme
    let bottom_tabs = bottom_tabs("bottom-tabs")
        .simple_tab("dashboard", "Dashboard")
        .simple_tab("analytics", "Analytics")
        .simple_tab("settings", "Settings")
        .active(1)
        .build();

    println!("Bottom Tabs (Green Active Theme):");
    println!("{}\n", bottom_tabs.render(&layout, Some(&theme)));

    // Vertical tabs with custom styling
    let vertical_tabs = vertical_tabs("vertical-tabs")
        .simple_tab("files", "Files")
        .simple_tab("search", "Search")
        .simple_tab("git", "Git")
        .active(0)
        .build();

    println!("Vertical Tabs (Purple Active Theme):");
    println!("{}\n", vertical_tabs.render(&layout, Some(&theme)));

    // Feature-rich tabs with badge styling
    let feature_tabs = horizontal_tabs("feature-tabs")
        .tab(
            Tab::new("inbox", "Inbox")
                .icon('📥')
                .badge("12")
                .content("You have 12 new messages"),
        )
        .tab(
            Tab::new("drafts", "Drafts")
                .icon('📝')
                .badge("3")
                .closeable()
                .content("3 draft messages"),
        )
        .tab(Tab::new("sent", "Sent").icon('📤').content("Sent messages"))
        .active(0)
        .build();

    println!("Feature-Rich Tabs (Indigo Theme with Colored Badges):");
    println!("{}\n", feature_tabs.render(&layout, Some(&theme)));

    // Minimal tabs with subtle styling
    let minimal_tabs = horizontal_tabs("minimal-tabs")
        .simple_tab("overview", "Overview")
        .simple_tab("details", "Details")
        .simple_tab("history", "History")
        .active(2)
        .build();

    println!("Minimal Tabs (Cyan Underline Theme):");
    println!("{}\n", minimal_tabs.render(&layout, Some(&theme)));

    // Status tabs with semantic colors
    let status_tabs = horizontal_tabs("status-tabs")
        .tab(
            Tab::new("success", "Success")
                .icon('✓')
                .class("text-green-400")
                .class("bg-green-900")
                .content("All systems operational"),
        )
        .tab(
            Tab::new("warning", "Warning")
                .icon('⚠')
                .class("text-yellow-400")
                .class("bg-yellow-900")
                .content("Minor issues detected"),
        )
        .tab(
            Tab::new("error", "Error")
                .icon('✗')
                .class("text-red-400")
                .class("bg-red-900")
                .content("Critical errors found"),
        )
        .active(1)
        .build();

    println!("Status Tabs (Semantic Color Theme):");
    println!("{}", status_tabs.render(&layout, Some(&theme)));

    println!("\n🎨 Theme Integration Demo Complete - All tab variants with utility CSS styling");

    Ok(())
}
