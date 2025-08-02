use tui_core::prelude::*;
use tui_core::themes::utility_css::UtilityProcessor;
use tui_core::widgets::*;

fn main() -> Result<()> {
    println!("☑️ Checkbox Widget Demo\n");

    // Get terminal size dynamically
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    let _layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width.min(40), // Checkbox doesn't need full width
        height: 1,
    };
    let _theme = tui_core::themes::colors::dark_theme();
    let utility = UtilityProcessor::new();

    // Define checkbox styling
    let checked_classes = vec!["text-green-400".to_string(), "font-bold".to_string()];
    let _unchecked_classes = ["text-gray-400".to_string()];
    let _disabled_classes = ["text-gray-600".to_string()];

    println!(
        "Checked Style: {}",
        utility.process_classes(&checked_classes)
    );

    // Checked checkbox
    let checked = checkbox("checked")
        .label("Checked option")
        .checked(true)
        .build();

    println!("Checked Checkbox:");
    println!("{}\n", checked.render_string());

    // Unchecked checkbox
    let unchecked = checkbox("unchecked")
        .label("Unchecked option")
        .checked(false)
        .build();

    println!("Unchecked Checkbox:");
    println!("{}\n", unchecked.render_string());

    // Different styles
    let square_checked = checkbox("square")
        .label("Square style checked")
        .style(CheckboxStyle::Square)
        .checked(true)
        .build();

    println!("Square Style:");
    println!("{}\n", square_checked.render_string());

    let round_unchecked = checkbox("round")
        .label("Round style unchecked")
        .style(CheckboxStyle::Round)
        .checked(false)
        .build();

    println!("Round Style:");
    println!("{}\n", round_unchecked.render_string());

    // Custom style
    let custom_checkbox = checkbox("custom")
        .label("Custom style")
        .style(CheckboxStyle::Custom {
            unchecked: "○".to_string(),
            checked: "●".to_string(),
        })
        .checked(true)
        .build();

    println!("Custom Style:");
    println!("{}\n", custom_checkbox.render_string());

    // Disabled checkbox
    let disabled_checked = checkbox("disabled_checked")
        .label("Disabled checked")
        .checked(true)
        .enabled(false)
        .build();

    println!("Disabled (checked):");
    println!("{}\n", disabled_checked.render_string());

    // Label positioning
    let label_before = checkbox("label_before")
        .label("Label before checkbox")
        .label_position(CheckboxLabelPosition::Before)
        .checked(true)
        .build();

    println!("Label Before:");
    println!("{}\n", label_before.render_string());

    // Convenience functions
    let simple = simple_checkbox("simple", "Simple checkbox");
    println!("Simple Checkbox:");
    println!("{}\n", simple.render_string());

    // Checkbox group demo
    let options = vec![
        ("Option 1", "opt1"),
        ("Option 2", "opt2"),
        ("Option 3", "opt3"),
    ];
    let mut group = simple_checkbox_group("demo_group", "Select multiple options:", options);
    group.check_option("opt1").unwrap();
    group.check_option("opt3").unwrap();

    println!("Checkbox Group:");
    println!("{}\n", group.render_string());

    // Horizontal checkbox group
    let horizontal_options = vec![("Yes", "yes"), ("No", "no"), ("Maybe", "maybe")];
    let horizontal_group =
        horizontal_checkbox_group("horizontal_group", "Horizontal layout:", horizontal_options);

    println!("Horizontal Checkbox Group:");
    println!("{}", horizontal_group.render_string());

    println!("\n🎨 Theme Integration Demo Complete - All checkbox variants with proper API usage");

    Ok(())
}
