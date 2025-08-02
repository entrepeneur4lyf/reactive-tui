use tui_core::prelude::*;
use tui_core::widgets::*;

fn main() -> Result<()> {
    // Get terminal size dynamically
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    println!("Spinner Widget Demo\n");

    let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width.min(20), // Widget doesn't need full width
        height: 1,
    };
    let theme = tui_core::themes::colors::dark_theme();

    // Loading spinner
    let loading = loading_spinner("loading");

    println!("Loading spinner:");
    println!("{}\n", loading.render(&layout, Some(&theme)));

    // Processing spinner
    let processing = processing_spinner("processing");

    println!("Processing spinner:");
    println!("{}\n", processing.render(&layout, Some(&theme)));

    // Saving spinner
    let saving = saving_spinner("saving");

    println!("Saving spinner:");
    println!("{}\n", saving.render(&layout, Some(&theme)));

    // Custom spinner
    let custom_definition =
        SpinnerDefinition::from_static(&["|", "/", "-", "\\"], 130, Some("custom"));
    let custom = SpinnerBuilder::with_custom("custom", custom_definition)
        .label("Custom")
        .build();

    println!("Custom spinner:");
    println!("{}", custom.render(&layout, Some(&theme)));

    Ok(())
}
