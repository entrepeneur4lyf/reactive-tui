//! Select Widget Demo
//!
//! Demonstrates the comprehensive select dropdown widget with different configurations.

use tui_core::widgets::{SelectBuilder, SelectOption};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Select Widget Demo ===\n");

    // Basic single-select dropdown
    println!("1. Creating basic single-select dropdown...");
    let basic_select = SelectBuilder::new("language-select")
        .options(vec!["Rust", "TypeScript", "Python", "Go"])
        .selected(Some(0))
        .placeholder("Choose a language...")
        .build();

    println!("   Selected: {:?}", basic_select.selected_ids());
    println!("   Display text: {}", basic_select.display_text());
    println!();

    // Multi-select with custom options
    println!("2. Creating multi-select with custom options...");
    let custom_options = vec![
        SelectOption::new("rust", "Rust")
            .icon("🦀")
            .description("Systems programming language"),
        SelectOption::new("typescript", "TypeScript")
            .icon("📘")
            .description("Typed JavaScript"),
        SelectOption::new("python", "Python")
            .icon("🐍")
            .description("General-purpose programming"),
        SelectOption::new("go", "Go")
            .icon("🐹")
            .description("Simple, fast, reliable"),
    ];

    let mut multi_select = SelectBuilder::new("tags-select")
        .custom_options(custom_options)
        .multi_select(true)
        .searchable(true)
        .selected_indices(vec![0, 2])
        .placeholder("Select languages...")
        .build();

    println!("   Initial selected: {:?}", multi_select.selected_ids());
    println!("   Display text: {}", multi_select.display_text());

    // Test some operations
    println!("\n3. Testing operations...");
    multi_select.select(1)?; // Select TypeScript
    println!(
        "   After selecting TypeScript: {:?}",
        multi_select.selected_ids()
    );

    multi_select.deselect(0)?; // Deselect Rust
    println!(
        "   After deselecting Rust: {:?}",
        multi_select.selected_ids()
    );

    // Test key handling
    println!("\n4. Testing key handling...");
    multi_select.open();
    println!("   Dropdown opened");

    let handled = multi_select.handle_key("ArrowDown")?;
    println!("   ArrowDown handled: {handled}");

    let handled = multi_select.handle_key("Enter")?;
    println!("   Enter handled: {handled}");

    // Test searchable functionality
    println!("\n5. Testing search functionality...");
    multi_select.open();
    multi_select.set_search_query("rust");
    println!("   Search query set to 'rust'");

    // Test element conversion
    println!("\n6. Testing element conversion...");
    let element = basic_select.to_element();
    println!("   Element tag: {}", element.tag);
    println!("   Element classes: {:?}", element.classes);
    println!("   Element focusable: {}", element.focusable);

    // Test convenience builders
    println!("\n7. Testing convenience builders...");
    let yes_no = SelectBuilder::yes_no("confirm".to_string());
    println!("   Yes/No select created: {}", yes_no.display_text());

    let languages = SelectBuilder::languages("lang".to_string());
    println!("   Languages select created: {}", languages.display_text());

    let priority = SelectBuilder::priority("prio".to_string());
    println!("   Priority select created: {}", priority.display_text());

    println!("\n=== Demo completed successfully! ===");
    Ok(())
}
