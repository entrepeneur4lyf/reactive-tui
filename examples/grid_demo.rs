use tui_core::components::Element;
use tui_core::layout::grid::{GridConfig, GridLayout, GridScalar};
use tui_core::prelude::*;

fn main() -> Result<()> {
    println!("📊 Grid Layout Demo\n");

    // Get terminal size dynamically
    let (term_width, term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    let container = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: term_height,
    };

    // 2x2 Grid Demo
    println!("2x2 Grid Layout:");
    let grid_config_2x2 = GridConfig {
        columns: vec![GridScalar::Fr(1.0), GridScalar::Fr(1.0)],
        rows: vec![GridScalar::Fr(1.0), GridScalar::Fr(1.0)],
        column_count: Some(2),
        row_count: Some(2),
        column_gap: 1,
        row_gap: 1,
        ..Default::default()
    };

    let grid_layout_2x2 = GridLayout::new(grid_config_2x2).with_dimensions(term_width, term_height);
    let grid_element_2x2 = create_grid_element("2x2-grid", 4, "Item");
    let placements_2x2 = grid_layout_2x2.compute_layout(&grid_element_2x2, container)?;

    render_grid_placements(&placements_2x2);
    println!();

    // 3x3 Grid Demo
    println!("3x3 Grid Layout:");
    let grid_config_3x3 = GridConfig {
        columns: vec![
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
        ],
        rows: vec![
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
        ],
        column_count: Some(3),
        row_count: Some(3),
        column_gap: 1,
        row_gap: 1,
        ..Default::default()
    };

    let grid_layout_3x3 = GridLayout::new(grid_config_3x3).with_dimensions(term_width, term_height);
    let grid_element_3x3 = create_grid_element("3x3-grid", 9, "Cell");
    let placements_3x3 = grid_layout_3x3.compute_layout(&grid_element_3x3, container)?;

    render_grid_placements(&placements_3x3);
    println!();

    // 1x4 Vertical Layout
    println!("1x4 Vertical Layout:");
    let grid_config_vertical = GridConfig {
        columns: vec![GridScalar::Fr(1.0)],
        rows: vec![
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
        ],
        column_count: Some(1),
        row_count: Some(4),
        row_gap: 1,
        ..Default::default()
    };

    let grid_layout_vertical = GridLayout::new(grid_config_vertical).with_dimensions(term_width, term_height);
    let grid_element_vertical = create_grid_element("vertical-grid", 4, "Row");
    let placements_vertical =
        grid_layout_vertical.compute_layout(&grid_element_vertical, container)?;

    render_grid_placements(&placements_vertical);
    println!();

    // 4x1 Horizontal Layout
    println!("4x1 Horizontal Layout:");
    let grid_config_horizontal = GridConfig {
        columns: vec![
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
            GridScalar::Fr(1.0),
        ],
        rows: vec![GridScalar::Fr(1.0)],
        column_count: Some(4),
        row_count: Some(1),
        column_gap: 1,
        ..Default::default()
    };

    let grid_layout_horizontal = GridLayout::new(grid_config_horizontal).with_dimensions(term_width, term_height);
    let grid_element_horizontal = create_grid_element("horizontal-grid", 4, "Col");
    let placements_horizontal =
        grid_layout_horizontal.compute_layout(&grid_element_horizontal, container)?;

    render_grid_placements(&placements_horizontal);
    println!();

    // Mixed Size Grid Demo
    println!("Mixed Size Grid (Fixed + Fraction):");
    let grid_config_mixed = GridConfig {
        columns: vec![
            GridScalar::Cells(20),
            GridScalar::Fr(2.0),
            GridScalar::Cells(15),
        ],
        rows: vec![
            GridScalar::Cells(5),
            GridScalar::Fr(1.0),
            GridScalar::Cells(3),
        ],
        column_count: Some(3),
        row_count: Some(3),
        column_gap: 2,
        row_gap: 1,
        ..Default::default()
    };

    let grid_layout_mixed = GridLayout::new(grid_config_mixed).with_dimensions(term_width, term_height);
    let grid_element_mixed = create_mixed_grid_element();
    let placements_mixed = grid_layout_mixed.compute_layout(&grid_element_mixed, container)?;

    render_grid_placements(&placements_mixed);

    println!("\n🎨 Grid Layout Demo Complete - All grid types demonstrated");

    Ok(())
}

/// Create a grid element with the specified number of children
fn create_grid_element(id: &str, count: usize, prefix: &str) -> Element {
    let mut element = Element::with_tag("div").id(id);

    for i in 1..=count {
        let child = Element::with_tag("div")
            .id(format!("{id}-{i}"))
            .content(format!("{prefix} {i}"))
            .class("grid-item")
            .class("p-1")
            .into();
        element = element.child(child);
    }

    element.build()
}

/// Create a mixed grid element with descriptive content
fn create_mixed_grid_element() -> Element {
    Element::with_tag("div")
        .id("mixed-grid")
        .child(
            Element::with_tag("div")
                .id("header-left")
                .content("Header Left (20px)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("header-center")
                .content("Header Center (2fr)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("header-right")
                .content("Header Right (15px)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("sidebar")
                .content("Sidebar (5px height)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("main-content")
                .content("Main Content (flexible)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("panel")
                .content("Side Panel")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("footer-left")
                .content("Footer (3px height)")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("footer-center")
                .content("Footer Center")
                .class("p-1")
                .into(),
        )
        .child(
            Element::with_tag("div")
                .id("footer-right")
                .content("Footer Right")
                .class("p-1")
                .into(),
        )
        .build()
}

/// Render grid placements in a readable format
fn render_grid_placements(placements: &[tui_core::layout::grid::GridPlacement]) {
    for placement in placements {
        println!(
            "  📦 '{}' at ({},{}) -> {}x{} [col:{}, row:{}, span:{}x{}]",
            placement.element_id,
            placement.rect.x,
            placement.rect.y,
            placement.rect.width,
            placement.rect.height,
            placement.cell.column,
            placement.cell.row,
            placement.cell.column_span,
            placement.cell.row_span
        );
    }
}
