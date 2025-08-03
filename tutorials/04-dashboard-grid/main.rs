use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    println!("📊 Tutorial 4: Dashboard Grid Layout\n");

    // Get terminal dimensions
    let (term_width, term_height) = crossterm::terminal::size().unwrap_or((140, 35));

    // Create layout engine
    let mut engine = LayoutEngine::with_dimensions(term_width, term_height);

    // Load theme and utility processor
    let theme = dark_theme();
    let utility = UtilityProcessor::new();

    // Create header bar with navigation
    let header = header_bar("dashboard-header")
        .left("📊 Analytics Dashboard")
        .center("🏠 Home | 📈 Analytics | ⚙️ Settings")
        .right("🔔 3 | 👤 Admin")
        .class("bg-purple-900")
        .class("text-purple-100")
        .class("font-bold")
        .class("border-b")
        .class("border-purple-700")
        .build();

    // Create main dashboard content
    let dashboard_content = Element::with_tag("div")
        .id("dashboard")
        .class("bg-gray-800")
        .class("p-4")
        .class("flex-1")
        .child(
            // Dashboard title
            Element::with_tag("div")
                .content("📈 System Overview Dashboard")
                .class("text-white")
                .class("font-bold")
                .class("text-xl")
                .class("mb-4")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            // Top metrics row (4 columns)
            Element::with_tag("div")
                .id("metrics-row")
                .class("grid")
                .class("grid-cols-4")
                .class("gap-4")
                .class("mb-6")
                .child(create_metric_card("👥", "Active Users", "2,847", "+12%", "text-green-400", "bg-green-900", "border-green-700"))
                .child(create_metric_card("💰", "Revenue", "$45,230", "+8%", "text-blue-400", "bg-blue-900", "border-blue-700"))
                .child(create_metric_card("📦", "Orders", "1,234", "+15%", "text-yellow-400", "bg-yellow-900", "border-yellow-700"))
                .child(create_metric_card("⚡", "Performance", "98.5%", "+2%", "text-purple-400", "bg-purple-900", "border-purple-700"))
                .into(),
        )
        .child(
            // Middle section (2 columns)
            Element::with_tag("div")
                .id("middle-section")
                .class("grid")
                .class("grid-cols-2")
                .class("gap-4")
                .class("mb-6")
                .child(create_chart_widget())
                .child(create_activity_widget())
                .into(),
        )
        .child(
            // Bottom section (3 columns)
            Element::with_tag("div")
                .id("bottom-section")
                .class("grid")
                .class("grid-cols-3")
                .class("gap-4")
                .child(create_status_widget())
                .child(create_alerts_widget())
                .child(create_quick_actions_widget())
                .into(),
        )
        .build();

    // Create footer status bar
    let footer = footer_bar("dashboard-footer")
        .left("🟢 All Systems Operational")
        .center("Last Updated: 2 minutes ago")
        .right("🖥️ CPU: 23% | 💾 RAM: 67% | 💿 Disk: 45%")
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
        .child(dashboard_content.into())
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
    println!("┌─ Header Bar (Purple theme with navigation)");
    println!("├─ Dashboard Content");
    println!("│  ├─ Top Metrics (4-column grid)");
    println!("│  ├─ Middle Section (2-column grid)");
    println!("│  └─ Bottom Section (3-column grid)");
    println!("└─ Footer Status Bar");

    // Show CSS classes being used
    println!("\n📋 CSS Classes Demonstration:");
    let grid_classes = vec![
        "grid".to_string(),
        "grid-cols-4".to_string(),
        "grid-cols-2".to_string(),
        "grid-cols-3".to_string(),
        "gap-4".to_string(),
    ];

    println!("Grid classes: {}", utility.process_classes(&grid_classes));

    println!("\n✅ Tutorial 4 Complete!");
    println!("📚 Next: Tutorial 5 - Complex IDE Layout");

    Ok(())
}

// Helper function to create metric cards
fn create_metric_card(
    icon: &str,
    title: &str,
    value: &str,
    change: &str,
    text_color: &str,
    bg_color: &str,
    border_color: &str,
) -> Element {
    Element::with_tag("div")
        .class(bg_color)
        .class("border")
        .class(border_color)
        .class("p-3")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .class("flex")
                .class("items-center")
                .class("mb-2")
                .child(
                    Element::with_tag("div")
                        .content(icon)
                        .class("text-2xl")
                        .class("mr-2")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .content(title)
                        .class(text_color)
                        .class("font-semibold")
                        .into(),
                )
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content(value)
                .class("text-white")
                .class("text-2xl")
                .class("font-bold")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content(&format!("{} from last month", change))
                .class("text-green-400")
                .class("text-sm")
                .into(),
        )
        .build()
}

// Helper function to create chart widget
fn create_chart_widget() -> Element {
    Element::with_tag("div")
        .class("bg-gray-700")
        .class("border")
        .class("border-gray-600")
        .class("p-4")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .content("📊 Sales Chart")
                .class("text-white")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("▁▂▃▅▆▇█▇▆▅▃▂▁ Sales Trend")
                .class("text-blue-400")
                .class("font-mono")
                .class("mb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📈 +15% increase this month")
                .class("text-green-400")
                .class("text-sm")
                .into(),
        )
        .build()
}

// Helper function to create activity widget
fn create_activity_widget() -> Element {
    Element::with_tag("div")
        .class("bg-gray-700")
        .class("border")
        .class("border-gray-600")
        .class("p-4")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .content("🔄 Recent Activity")
                .class("text-white")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
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
                .content("• Order #1234 completed")
                .class("text-gray-300")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("• System backup finished")
                .class("text-gray-300")
                .into(),
        )
        .build()
}

// Helper function to create status widget
fn create_status_widget() -> Element {
    Element::with_tag("div")
        .class("bg-gray-700")
        .class("border")
        .class("border-gray-600")
        .class("p-4")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .content("🔧 System Status")
                .class("text-white")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🟢 Database: Online")
                .class("text-green-400")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🟢 API: Healthy")
                .class("text-green-400")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🟡 Cache: Warning")
                .class("text-yellow-400")
                .into(),
        )
        .build()
}

// Helper function to create alerts widget
fn create_alerts_widget() -> Element {
    Element::with_tag("div")
        .class("bg-gray-700")
        .class("border")
        .class("border-gray-600")
        .class("p-4")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .content("🚨 Alerts")
                .class("text-white")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("⚠️ High CPU usage")
                .class("text-yellow-400")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("ℹ️ Update available")
                .class("text-blue-400")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("✅ Backup completed")
                .class("text-green-400")
                .into(),
        )
        .build()
}

// Helper function to create quick actions widget
fn create_quick_actions_widget() -> Element {
    Element::with_tag("div")
        .class("bg-gray-700")
        .class("border")
        .class("border-gray-600")
        .class("p-4")
        .class("rounded")
        .child(
            Element::with_tag("div")
                .content("⚡ Quick Actions")
                .class("text-white")
                .class("font-bold")
                .class("mb-3")
                .class("border-b")
                .class("border-gray-600")
                .class("pb-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("🔄 Restart Services")
                .class("text-blue-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-600")
                .class("p-1")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("📊 Generate Report")
                .class("text-green-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-600")
                .class("p-1")
                .class("mb-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .content("⚙️ System Settings")
                .class("text-purple-400")
                .class("cursor-pointer")
                .class("hover:bg-gray-600")
                .class("p-1")
                .into(),
        )
        .build()
}
