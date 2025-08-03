use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

fn main() -> Result<()> {
  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  println!("Switch Widget Demo\n");

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width.min(20), // Widget doesn't need full width
    height: 1,
  };
  let theme = reactive_tui::themes::colors::dark_theme();

  // Switch ON
  let switch_on = switch("switch_on").enabled(true).build();

  println!("Switch ON:");
  println!("{}\n", switch_on.render(&layout, Some(&theme)));

  // Switch OFF
  let switch_off = switch("switch_off").enabled(false).build();

  println!("Switch OFF:");
  println!("{}\n", switch_off.render(&layout, Some(&theme)));

  // Disabled ON
  let disabled_on = switch("disabled_on")
    .enabled(true)
    .interactive(false)
    .build();

  println!("Disabled (ON):");
  println!("{}\n", disabled_on.render(&layout, Some(&theme)));

  // Disabled OFF
  let disabled_off = switch("disabled_off")
    .enabled(false)
    .interactive(false)
    .build();

  println!("Disabled (OFF):");
  println!("{}", disabled_off.render(&layout, Some(&theme)));

  Ok(())
}
