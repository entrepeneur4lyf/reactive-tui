use reactive_tui::prelude::*;

fn main() -> Result<()> {
  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  println!("Slider Widget Demo\n");

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width.min(25), // Widget doesn't need full width
    height: 1,
  };
  let theme = reactive_tui::themes::colors::dark_theme();

  // Slider at 0%
  let slider_0 = Slider::builder("slider_0")
    .range(0.0, 100.0)
    .value(0.0)
    .width(20)
    .build()?;

  println!("0% (Min):");
  println!("{}\n", slider_0.render(&layout, Some(&theme)));

  // Slider at 25%
  let slider_25 = Slider::builder("slider_25")
    .range(0.0, 100.0)
    .value(25.0)
    .width(20)
    .build()?;

  println!("25%:");
  println!("{}\n", slider_25.render(&layout, Some(&theme)));

  // Slider at 50%
  let slider_50 = Slider::builder("slider_50")
    .range(0.0, 100.0)
    .value(50.0)
    .width(20)
    .build()?;

  println!("50%:");
  println!("{}\n", slider_50.render(&layout, Some(&theme)));

  // Slider at 100%
  let slider_100 = Slider::builder("slider_100")
    .range(0.0, 100.0)
    .value(100.0)
    .width(20)
    .build()?;

  println!("100% (Max):");
  println!("{}", slider_100.render(&layout, Some(&theme)));

  Ok(())
}
