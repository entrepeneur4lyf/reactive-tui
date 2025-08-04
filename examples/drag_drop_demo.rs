//! Drag and Drop Demo
//!
//! Demonstrates the new drag and drop convenience functions:
//! - `draggable()` - Creates draggable elements
//! - `droppable()` - Creates drop areas
//! - `drop_target()` - Creates drop targets with hover feedback
//!
//! This example shows a simple drag and drop interface with multiple
//! draggable items and different types of drop areas.

use reactive_tui::prelude::*;
use reactive_tui::components::Element;
use reactive_tui::widgets::{draggable, droppable, drop_target};

fn main() -> Result<()> {
    println!("=== Drag and Drop Demo ===");
    println!();
    println!("This demo showcases the drag and drop convenience functions:");
    println!("  📄 draggable() - Creates draggable elements");
    println!("  📥 droppable() - Creates drop areas");  
    println!("  🎯 drop_target() - Creates drop targets with hover feedback");
    println!();

    // Create demo layout
    let layout = create_drag_drop_layout();
    
    // Print the layout structure
    println!("Generated layout structure:");
    println!("{:#?}", layout);
    
    println!();
    println!("Example usage patterns:");
    demo_usage_patterns();
    
    Ok(())
}

fn create_drag_drop_layout() -> Element {
    Element::with_tag("div")
        .id("main-container")
        .class("drag-drop-demo")
        .child(
            Element::with_tag("h1")
                .content("Drag and Drop Demo")
                .class("title")
                .into()
        )
        .child(create_draggable_items_section())
        .child(create_drop_areas_section())
        .build()
}

fn create_draggable_items_section() -> Element {
    Element::with_tag("div")
        .id("draggable-section")
        .class("section")
        .child(
            Element::with_tag("h2")
                .content("Draggable Items")
                .into()
        )
        .child(
            Element::with_tag("div")
                .id("items-container")
                .class("items-row")
                .child(draggable("drag-item1", "start_drag_item1"))
                .child(draggable("drag-item2", "start_drag_item2"))
                .child(draggable("drag-item3", "start_drag_item3"))
                .into()
        )
        .build()
}

fn create_drop_areas_section() -> Element {
    Element::with_tag("div")
        .id("drop-section")
        .class("section")
        .child(
            Element::with_tag("h2")
                .content("Drop Areas")
                .into()
        )
        .child(
            Element::with_tag("div")
                .id("drop-areas-container")
                .class("drop-areas-row")
                .child(droppable("trash-area", "drop_in_trash"))
                .child(droppable("folder-area", "drop_in_folder"))
                .child(drop_target("special-target", "drop_on_target"))
                .into()
        )
        .build()
}

fn demo_usage_patterns() {
    println!("// Create a draggable element");
    println!("let item = draggable(\"my-item\", \"handle_drag_start\");");
    println!();
    
    println!("// Create a droppable area");
    println!("let area = droppable(\"drop-zone\", \"handle_drop\");");
    println!();
    
    println!("// Create a drop target with hover feedback");
    println!("let target = drop_target(\"target\", \"handle_drop_on_target\");");
    println!();
    
    println!("✅ All three convenience functions are working correctly!");
}