use reactive_tui::prelude::*;
use crossterm::{
  event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseButton, MouseEventKind},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io;

struct InputDemoState {
  text_input: Input,
  email_input: Input,
  password_input: Input,
  number_input: Input,
  search_input: Input,
  focused_input: usize, // 0=text, 1=email, 2=password, 3=number, 4=search
  exit: bool,
}

impl InputDemoState {
  fn new() -> Self {
    Self {
      text_input: Input::builder("text-demo")
        .input_type(InputType::Text)
        .placeholder("Enter your name")
        .build(),
      email_input: Input::builder("email-demo")
        .input_type(InputType::Email)
        .placeholder("user@example.com")
        .required(true)
        .build(),
      password_input: Input::builder("password-demo")
        .input_type(InputType::Password)
        .placeholder("Enter password")
        .min_length(8)
        .build(),
      number_input: Input::builder("number-demo")
        .input_type(InputType::Number)
        .placeholder("Enter age")
        .build(),
      search_input: Input::builder("search-demo")
        .input_type(InputType::Search)
        .placeholder("Search...")
        .build(),
      focused_input: 0,
      exit: false,
    }
  }

  fn get_current_input_mut(&mut self) -> &mut Input {
    match self.focused_input {
      0 => &mut self.text_input,
      1 => &mut self.email_input,
      2 => &mut self.password_input,
      3 => &mut self.number_input,
      4 => &mut self.search_input,
      _ => &mut self.text_input,
    }
  }

  fn update_focus(&mut self) {
    // Clear all focus states
    self.text_input.set_focused(false);
    self.email_input.set_focused(false);
    self.password_input.set_focused(false);
    self.number_input.set_focused(false);
    self.search_input.set_focused(false);

    // Set focus on current input
    match self.focused_input {
      0 => self.text_input.set_focused(true),
      1 => self.email_input.set_focused(true),
      2 => self.password_input.set_focused(true),
      3 => self.number_input.set_focused(true),
      4 => self.search_input.set_focused(true),
      _ => {}
    }
  }
}

fn main() -> Result<()> {
  // Setup terminal for interactive mode
  enable_raw_mode()?;
  let mut stdout = io::stdout();
  execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

  let mut state = InputDemoState::new();
  state.update_focus();

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
            KeyCode::Tab => {
              state.focused_input = (state.focused_input + 1) % 5;
              state.update_focus();
            }
            KeyCode::BackTab => {
              state.focused_input = if state.focused_input == 0 { 4 } else { state.focused_input - 1 };
              state.update_focus();
            }
            KeyCode::Up => {
              state.focused_input = if state.focused_input == 0 { 4 } else { state.focused_input - 1 };
              state.update_focus();
            }
            KeyCode::Down => {
              state.focused_input = (state.focused_input + 1) % 5;
              state.update_focus();
            }
            _ => {
              // Forward key events to the focused input
              let _key_str = format!("{:?}", key.code);
              let key_str = match key.code {
                KeyCode::Char(c) => c.to_string(),
                KeyCode::Backspace => "Backspace".to_string(),
                KeyCode::Delete => "Delete".to_string(),
                KeyCode::Left => "ArrowLeft".to_string(),
                KeyCode::Right => "ArrowRight".to_string(),
                KeyCode::Home => "Home".to_string(),
                KeyCode::End => "End".to_string(),
                KeyCode::Enter => "Enter".to_string(),
                _ => continue,
              };

              state.get_current_input_mut().handle_key_event(&key_str);
            }
          }
        }
        Event::Mouse(mouse) => {
          if mouse.kind == MouseEventKind::Down(MouseButton::Left) {
            // Handle mouse clicks to focus inputs
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

  println!("Interactive Input Demo Complete!");
  Ok(())
}

fn render_demo(state: &InputDemoState) -> Result<()> {
  println!("┌─────────────────────────────────────────────────────────────┐");
  println!("│                Interactive Input Components                 │");
  println!("│                    Like DaisyUI for TUI                    │");
  println!("└─────────────────────────────────────────────────────────────┘");
  println!();

  println!("🎮 Interactive Controls:");
  println!("   • Tab/Shift+Tab or ↑/↓ to navigate inputs");
  println!("   • Type to enter text in focused input");
  println!("   • Click on inputs with mouse");
  println!("   • Press 'q' or ESC to exit");
  println!();

  let layout = LayoutRect { x: 0, y: 0, width: 60, height: 3 };
  let theme = reactive_tui::themes::colors::dark_theme();

  // Demo 1: Input Types
  println!("1. Input Types & Validation");
  println!("   ─────────────────────────");

  let focus_indicator = |focused: bool| if focused { "→ " } else { "  " };

  println!("   {}Text Input:", focus_indicator(state.focused_input == 0));
  println!("   {}", state.text_input.render(&layout, Some(&theme)));
  println!("   Value: \"{}\"", state.text_input.value());
  println!();

  println!("   {}Email Input (Required):", focus_indicator(state.focused_input == 1));
  println!("   {}", state.email_input.render(&layout, Some(&theme)));
  println!("   Value: \"{}\" | Valid: {}",
    state.email_input.value(),
    matches!(state.email_input.state.validation_state, ValidationState::Valid)
  );
  if let Some(msg) = &state.email_input.state.validation_message {
    println!("   Error: {}", msg);
  }
  println!();

  println!("   {}Password Input (Min 8 chars):", focus_indicator(state.focused_input == 2));
  println!("   {}", state.password_input.render(&layout, Some(&theme)));
  println!("   Length: {} | Valid: {}",
    state.password_input.value().len(),
    matches!(state.password_input.state.validation_state, ValidationState::Valid)
  );
  if let Some(msg) = &state.password_input.state.validation_message {
    println!("   Error: {}", msg);
  }
  println!();

  println!("   {}Number Input:", focus_indicator(state.focused_input == 3));
  println!("   {}", state.number_input.render(&layout, Some(&theme)));
  println!("   Value: \"{}\" | Valid: {}",
    state.number_input.value(),
    matches!(state.number_input.state.validation_state, ValidationState::Valid)
  );
  if let Some(msg) = &state.number_input.state.validation_message {
    println!("   Error: {}", msg);
  }
  println!();

  println!("   {}Search Input:", focus_indicator(state.focused_input == 4));
  println!("   {}", state.search_input.render(&layout, Some(&theme)));
  println!("   Value: \"{}\"", state.search_input.value());
  println!();

  // Demo 2: Live State Display
  println!("2. Live State Display");
  println!("   ──────────────────");
  println!("   Current Focus: Input {} ({})",
    state.focused_input + 1,
    match state.focused_input {
      0 => "Text",
      1 => "Email",
      2 => "Password",
      3 => "Number",
      4 => "Search",
      _ => "Unknown"
    }
  );

  let total_chars: usize = [
    state.text_input.value().len(),
    state.email_input.value().len(),
    state.password_input.value().len(),
    state.number_input.value().len(),
    state.search_input.value().len(),
  ].iter().sum();

  println!("   Total Characters: {}", total_chars);
  let cursor_pos = match state.focused_input {
    0 => state.text_input.state.cursor_position,
    1 => state.email_input.state.cursor_position,
    2 => state.password_input.state.cursor_position,
    3 => state.number_input.state.cursor_position,
    4 => state.search_input.state.cursor_position,
    _ => 0,
  };
  println!("   Cursor Position: {}", cursor_pos);
  println!();

  println!("┌─────────────────────────────────────────────────────────────┐");
  println!("│                    Interactive Features                     │");
  println!("├─────────────────────────────────────────────────────────────┤");
  println!("│ ✓ Real-time validation                                     │");
  println!("│ ✓ Keyboard navigation (Tab/Arrow keys)                     │");
  println!("│ ✓ Mouse click focus                                        │");
  println!("│ ✓ Multiple input types                                     │");
  println!("│ ✓ Live state display                                       │");
  println!("│ ✓ Modern terminal UI                                       │");
  println!("└─────────────────────────────────────────────────────────────┘");

  Ok(())
}

fn handle_mouse_click(state: &mut InputDemoState, _x: u16, y: u16) {
  // Simple row-based click handling for input focus
  match y {
    12..=14 => { state.focused_input = 0; state.update_focus(); },
    16..=18 => { state.focused_input = 1; state.update_focus(); },
    22..=24 => { state.focused_input = 2; state.update_focus(); },
    28..=30 => { state.focused_input = 3; state.update_focus(); },
    34..=36 => { state.focused_input = 4; state.update_focus(); },
    _ => {}
  }
}
