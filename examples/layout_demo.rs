use tui_core::components::Element;
use tui_core::layout::{DisplayType, Layout, LayoutEngine};
use tui_core::prelude::*;

fn main() -> Result<()> {
    println!("🎯 Layout System Demo\n");

    let mut engine = LayoutEngine::with_dimensions(400, 200);

    // Vertical Layout Demo
    println!("📐 Vertical Layout (Column):");
    let vertical_element = Element::with_tag("vbox")
        .id("app")
        .class("flex-col")
        .class("h-full")
        .child(
            Element::with_tag("div")
                .id("header")
                .content("Header Section")
                .attr("height", "3")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("content")
                .content("Main Content Area")
                .class("h-2/3")
                .class("p-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("footer")
                .content("Footer Section")
                .attr("height", "2")
                .class("p-1")
                .into(),
        )
        .build();

    let vertical_layout = engine.compute_layout(&vertical_element)?;
    println!("{}\n", render_layout_outline(&vertical_layout, 0));

    // Horizontal Layout Demo
    println!("↔️ Horizontal Layout (Row):");
    let horizontal_element = Element::with_tag("hbox")
        .id("container")
        .class("flex-row")
        .class("w-full")
        .child(
            Element::with_tag("div")
                .id("sidebar")
                .content("Sidebar")
                .attr("width", "20")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("main")
                .content("Main Content")
                .class("w-2/3")
                .class("p-2")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("aside")
                .content("Aside")
                .attr("width", "15")
                .class("p-1")
                .into(),
        )
        .build();

    let horizontal_layout = engine.compute_layout(&horizontal_element)?;
    println!("{}\n", render_layout_outline(&horizontal_layout, 0));

    // Complex App Layout Demo
    println!("🏗️ Complex App Layout:");
    let app_element = Element::with_tag("div")
        .id("app")
        .class("flex-col")
        .class("h-full")
        .child(
            Element::with_tag("div")
                .id("header")
                .content("📋 Application Header")
                .attr("height", "3")
                .class("flex")
                .class("justify-center")
                .class("items-center")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("body")
                .class("flex-row")
                .class("h-3/4")
                .child(
                    Element::with_tag("div")
                        .id("sidebar")
                        .content("📁 Navigation")
                        .attr("width", "18")
                        .class("p-1")
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .id("content")
                        .class("flex-col")
                        .class("w-full")
                        .class("p-2")
                        .child(
                            Element::with_tag("div")
                                .id("toolbar")
                                .content("🛠️ Toolbar")
                                .attr("height", "2")
                                .class("p-1")
                                .into(),
                        )
                        .child(
                            Element::with_tag("div")
                                .id("workspace")
                                .content("📄 Workspace Area")
                                .class("h-full")
                                .class("p-1")
                                .into(),
                        )
                        .into(),
                )
                .child(
                    Element::with_tag("div")
                        .id("properties")
                        .content("⚙️ Properties")
                        .attr("width", "22")
                        .class("p-1")
                        .into(),
                )
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("statusbar")
                .content("✅ Ready | 12:34 PM")
                .attr("height", "1")
                .class("flex")
                .class("justify-between")
                .class("items-center")
                .into(),
        )
        .build();

    let app_layout = engine.compute_layout(&app_element)?;
    println!("{}\n", render_layout_outline(&app_layout, 0));

    // CSS Grid Layout Demo
    println!("📊 CSS Grid Layout:");
    let grid_element = Element::with_tag("div")
        .id("grid-container")
        .class("grid")
        .class("w-full")
        .class("h-full")
        .child(
            Element::with_tag("div")
                .id("grid-header")
                .content("Grid Header (1,1)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("grid-nav")
                .content("Grid Nav (2,1)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("grid-main")
                .content("Grid Main (2,2)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("grid-aside")
                .content("Grid Aside (2,3)")
                .class("p-1")
                .into(),
        )
        .build();

    let grid_layout = engine.compute_layout(&grid_element)?;
    println!("{}\n", render_layout_outline(&grid_layout, 0));

    // Centered Layout Demo
    println!("🎯 Centered Layout:");
    let centered_element = Element::with_tag("center")
        .id("centered-container")
        .class("w-full")
        .class("h-full")
        .child(
            Element::with_tag("div")
                .id("modal")
                .content("🪟 Centered Modal Dialog")
                .attr("width", "40")
                .attr("height", "10")
                .class("p-2")
                .into(),
        )
        .build();

    let centered_layout = engine.compute_layout(&centered_element)?;
    println!("{}", render_layout_outline(&centered_layout, 0));

    println!("\n🎨 Layout System Demo Complete - All layout types demonstrated");

    Ok(())
}

fn render_layout_outline(layout: &Layout, indent: usize) -> String {
    let mut output = String::new();
    let prefix = "  ".repeat(indent);

    // Format the layout info with emojis and colors
    let icon = match layout.tag.as_str() {
        "vbox" | "column" => "📐",
        "hbox" | "row" => "↔️",
        "center" => "🎯",
        "grid" => "📊",
        "div" => "📦",
        _ => "🔸",
    };

    let display_type = match layout.styles.display {
        DisplayType::Flex => "flex",
        DisplayType::Grid => "grid",
        DisplayType::Block => "block",
        DisplayType::Inline => "inline",
        DisplayType::None => "none",
    };

    output.push_str(&format!(
        "{}{} {} '{}' [{}]: x={}, y={}, w={}, h={}\n",
        prefix,
        icon,
        display_type,
        layout.element_id.as_deref().unwrap_or("unnamed"),
        layout.tag,
        layout.rect.x,
        layout.rect.y,
        layout.rect.width,
        layout.rect.height
    ));

    // Add content preview if available
    if let Some(content) = &layout.content {
        if !content.is_empty() {
            output.push_str(&format!("{prefix}    📝 \"{content}\"\n"));
        }
    }

    // Render children recursively
    for child in &layout.children {
        output.push_str(&render_layout_outline(child, indent + 1));
    }

    output
}
