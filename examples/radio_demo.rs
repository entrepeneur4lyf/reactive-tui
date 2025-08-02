use tui_core::prelude::*;
use tui_core::widgets::*;

fn main() -> Result<()> {
    // Get terminal size dynamically
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    println!("Radio Group Widget Demo\n");

    let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width.min(20), // Widget doesn't need full width
        height: 4,
    };
    let theme = tui_core::themes::colors::dark_theme();

    // Radio group with first option selected
    let radio_0 = radio_group("size_group")
        .option("small", "Small")
        .option("medium", "Medium")
        .option("large", "Large")
        .selected("small")
        .build();

    println!("First option selected:");
    println!("{}\n", radio_0.render(&layout, Some(&theme)));

    // Radio group with second option selected
    let radio_1 = radio_group("size_group_2")
        .option("small", "Small")
        .option("medium", "Medium")
        .option("large", "Large")
        .selected("medium")
        .build();

    println!("Second option selected:");
    println!("{}\n", radio_1.render(&layout, Some(&theme)));

    // Radio group with third option selected
    let radio_2 = radio_group("size_group_3")
        .option("small", "Small")
        .option("medium", "Medium")
        .option("large", "Large")
        .selected("large")
        .build();

    println!("Third option selected:");
    println!("{}", radio_2.render(&layout, Some(&theme)));

    Ok(())
}
