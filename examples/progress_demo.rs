use tui_core::prelude::*;

fn main() -> Result<()> {
    // Get terminal size dynamically
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    println!("Progress Bar Widget Demo\n");

    let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width.min(30), // Widget doesn't need full width
        height: 1,
    };
    let theme = tui_core::themes::colors::dark_theme();

    // Linear progress at 25%
    let progress_25 = ProgressBarBuilder::new("progress_25")
        .linear('█', '░', Some(20))
        .build();

    println!("25% Progress:");
    println!(
        "{}\n",
        progress_25.render_with_value(0.25, &layout, Some(&theme))
    );

    // Linear progress at 50%
    let progress_50 = ProgressBarBuilder::new("progress_50")
        .linear('█', '░', Some(20))
        .build();

    println!("50% Progress:");
    println!(
        "{}\n",
        progress_50.render_with_value(0.50, &layout, Some(&theme))
    );

    // Linear progress at 75%
    let progress_75 = ProgressBarBuilder::new("progress_75")
        .linear('█', '░', Some(20))
        .build();

    println!("75% Progress:");
    println!(
        "{}\n",
        progress_75.render_with_value(0.75, &layout, Some(&theme))
    );

    // Complete progress
    let progress_100 = ProgressBarBuilder::new("progress_100")
        .linear('█', '░', Some(20))
        .build();

    println!("100% Progress:");
    println!(
        "{}",
        progress_100.render_with_value(1.0, &layout, Some(&theme))
    );

    Ok(())
}
