use reactive_tui::prelude::*;
use reactive_tui::widgets::*;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

struct CheckboxDemoState {
  basic_checked: bool,
  basic_unchecked: bool,
  ballot_checked: bool,
  square_checked: bool,
  round_checked: bool,
  custom_checked: bool,
  label_after_checked: bool,
  label_before_checked: bool,
  exit: bool,
}

impl CheckboxDemoState {
  fn new() -> Self {
    Self {
      basic_checked: true,
      basic_unchecked: false,
      ballot_checked: true,
      square_checked: true,
      round_checked: false,
      custom_checked: true,
      label_after_checked: true,
      label_before_checked: true,

      exit: false,
    }
  }
}

fn main() -> Result<()> {
  // Setup terminal for interactive mode
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

  let mut state = CheckboxDemoState::new();

  // Main interactive loop
  loop {
    // Clear screen and render current state
    print!("\x1B[2J\x1B[H"); // Clear screen and move cursor to top

    render_demo(&state)?;

    if state.exit {
      break;
    }

    // Handle events
    if event::poll(std::time::Duration::from_millis(100))? {
      match event::read()? {
        Event::Key(key) => {
          match key.code {
            KeyCode::Char('q') | KeyCode::Esc => state.exit = true,
            KeyCode::Char('1') => state.basic_checked = !state.basic_checked,
            KeyCode::Char('2') => state.basic_unchecked = !state.basic_unchecked,
            KeyCode::Char('3') => state.ballot_checked = !state.ballot_checked,
            KeyCode::Char('4') => state.square_checked = !state.square_checked,
            KeyCode::Char('5') => state.round_checked = !state.round_checked,
            KeyCode::Char('6') => state.custom_checked = !state.custom_checked,
            _ => {}
          }
        }
        Event::Mouse(mouse) => {
          if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            // Handle mouse clicks on checkboxes
            handle_mouse_click(&mut state, mouse.column, mouse.row);
          }
        }
        _ => {}
      }
    }
  }

  // Cleanup terminal
  disable_raw_mode()?;
  execute!(stdout, LeaveAlternateScreen, DisableMouseCapture)?;

  println!("Interactive Checkbox Demo Complete!");
  Ok(())
}

fn render_demo(state: &CheckboxDemoState) -> Result<()> {
  println!("┌─────────────────────────────────────────────────────────────┐");
  println!("│              Interactive Checkbox Components               │");
  println!("│                    Like DaisyUI for TUI                    │");
  println!("└─────────────────────────────────────────────────────────────┘");
  println!();

  println!("🎮 Interactive Controls:");
  println!("   • Press 1-6 to toggle checkboxes");
  println!("   • Click on checkboxes with mouse");
  println!("   • Press 'q' or ESC to exit");
  println!();

  // Demo 1: Basic States (Interactive)
  println!("1. Basic States (Press 1-3 to toggle)");
  println!("   ────────────────────────────────────");

  let basic_checked = checkbox("basic-checked")
    .label("Basic checked (Press 1)")
    .checked(state.basic_checked)
    .build();

  let basic_unchecked = checkbox("basic-unchecked")
    .label("Basic unchecked (Press 2)")
    .checked(state.basic_unchecked)
    .build();

  let disabled = checkbox("basic-disabled")
    .label("Disabled checkbox")
    .checked(true)
    .enabled(false)
    .build();

  println!("   {}", basic_checked.render_string());
  println!("   {}", basic_unchecked.render_string());
  println!("   {}", disabled.render_string());
  println!();

  // Demo 2: Visual Styles (Interactive)
  println!("2. Visual Styles (Press 3-6 to toggle)");
  println!("   ─────────────────────────────────────");

  let ballot_style = checkbox("ballot")
    .label("Ballot style (Press 3)")
    .style(CheckboxStyle::Ballot)
    .checked(state.ballot_checked)
    .build();

  let square_style = checkbox("square")
    .label("Square bracket style (Press 4)")
    .style(CheckboxStyle::Square)
    .checked(state.square_checked)
    .build();

  let round_style = checkbox("round")
    .label("Round parentheses style (Press 5)")
    .style(CheckboxStyle::Round)
    .checked(state.round_checked)
    .build();

  let custom_style = checkbox("custom")
    .label("Custom symbols (Press 6)")
    .style(CheckboxStyle::Custom {
      unchecked: "○".to_string(),
      checked: "●".to_string(),
    })
    .checked(state.custom_checked)
    .build();

  println!("   {}", ballot_style.render_string());
  println!("   {}", square_style.render_string());
  println!("   {}", round_style.render_string());
  println!("   {}", custom_style.render_string());
  println!();

  // Demo 3: Label Positioning
  println!("3. Label Positioning");
  println!("   ─────────────────");

  let label_after = checkbox("label-after")
    .label("Label after checkbox")
    .label_position(CheckboxLabelPosition::After)
    .checked(state.label_after_checked)
    .build();

  let label_before = checkbox("label-before")
    .label("Label before checkbox")
    .label_position(CheckboxLabelPosition::Before)
    .checked(state.label_before_checked)
    .build();

  println!("   {}", label_after.render_string());
  println!("   {}", label_before.render_string());
  println!();

  // Demo 4: Code Examples
  println!("4. Code Examples");
  println!("   ─────────────");
  println!("   // Basic checkbox");
  println!("   checkbox(\"my-checkbox\")");
  println!("     .label(\"Check me\")");
  println!("     .checked(true)");
  println!("     .build()");
  println!();
  println!("   // Custom style");
  println!("   checkbox(\"custom\")");
  println!("     .style(CheckboxStyle::Custom {{");
  println!("       unchecked: \"○\".to_string(),");
  println!("       checked: \"●\".to_string(),");
  println!("     }})");
  println!("     .build()");
  println!();

  // Demo 5: Live State Display
  println!("5. Live State Display");
  println!("   ──────────────────");
  println!("   Current States:");
  println!("   • Basic checked: {}", state.basic_checked);
  println!("   • Basic unchecked: {}", state.basic_unchecked);
  println!("   • Ballot style: {}", state.ballot_checked);
  println!("   • Square style: {}", state.square_checked);
  println!("   • Round style: {}", state.round_checked);
  println!("   • Custom style: {}", state.custom_checked);
  println!();

  println!("┌─────────────────────────────────────────────────────────────┐");
  println!("│                    Interactive Features                     │");
  println!("├─────────────────────────────────────────────────────────────┤");
  println!("│ ✓ Real-time state updates                                  │");
  println!("│ ✓ Keyboard controls (1-6)                                  │");
  println!("│ ✓ Mouse click support                                      │");
  println!("│ ✓ Multiple visual styles                                   │");
  println!("│ ✓ Live code examples                                       │");
  println!("│ ✓ Modern terminal UI                                       │");
  println!("└─────────────────────────────────────────────────────────────┘");

  Ok(())
}

fn handle_mouse_click(state: &mut CheckboxDemoState, _x: u16, y: u16) {
  // Simple row-based click handling
  match y {
    12 => state.basic_checked = !state.basic_checked,
    13 => state.basic_unchecked = !state.basic_unchecked,
    19 => state.ballot_checked = !state.ballot_checked,
    20 => state.square_checked = !state.square_checked,
    21 => state.round_checked = !state.round_checked,
    22 => state.custom_checked = !state.custom_checked,
    _ => {}
  }
}
