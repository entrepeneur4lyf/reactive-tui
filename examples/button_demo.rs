use reactive_tui::prelude::*;

fn main() -> reactive_tui::error::Result<()> {
  println!("Button Widget Demo\n");

  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width.min(20), // Button doesn't need full width
    height: 3,
  };
  let theme = reactive_tui::themes::colors::dark_theme();

  // Primary button
  let primary = Button::builder("primary", "Primary")
    .button_type(ButtonType::Primary)
    .build();

  println!("Primary:");
  println!("{}\n", primary.render(&layout, Some(&theme)));

  // Secondary button
  let secondary = Button::builder("secondary", "Secondary")
    .button_type(ButtonType::Secondary)
    .build();

  println!("Secondary:");
  println!("{}\n", secondary.render(&layout, Some(&theme)));

  // Danger button
  let danger = Button::builder("danger", "Danger")
    .button_type(ButtonType::Danger)
    .build();

  println!("Danger:");
  println!("{}\n", danger.render(&layout, Some(&theme)));

  // Disabled button
  let disabled = Button::builder("disabled", "Disabled")
    .button_type(ButtonType::Primary)
    .disabled(true)
    .build();

  println!("Disabled:");
  println!("{}", disabled.render(&layout, Some(&theme)));

  Ok(())
}
