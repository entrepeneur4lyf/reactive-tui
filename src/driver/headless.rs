//! Headless driver for testing and automation
//!
//! Provides a fully functional driver that doesn't require a terminal,
//! perfect for testing, CI/CD, and automated scenarios.

use super::{Driver, DriverCapabilities, DriverConfig, DriverEvent};
use crate::error::{Result, TuiError};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseButton, MouseEvent, MouseEventKind};
use std::collections::VecDeque;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use tokio::time::sleep;

/// Headless driver that simulates terminal behavior without actual output
pub struct HeadlessDriver {
  capabilities: DriverCapabilities,
  size: (u16, u16),
  title: String,
  cursor_position: (u16, u16),
  cursor_visible: bool,
  mouse_capture: bool,
  application_mode: bool,

  // Event simulation
  event_queue: Arc<Mutex<VecDeque<DriverEvent>>>,
  event_sender: Option<mpsc::UnboundedSender<DriverEvent>>,

  // Output capture for testing
  output_buffer: Arc<Mutex<String>>,

  // Timing for realistic simulation
  start_time: Instant,
}

impl HeadlessDriver {
  /// Create a new headless driver
  pub fn new(config: DriverConfig) -> Result<Self> {
    let size = config.size.unwrap_or((400, 200));

    let capabilities = DriverCapabilities {
      can_suspend: false, // Headless can't be suspended
      is_headless: true,
      is_inline: config.inline,
      is_web: false,
      supports_mouse: config.mouse,
      supports_colors: true,
      max_colors: 16_777_216, // Simulate full color support
    };

    Ok(Self {
      capabilities,
      size,
      title: config.title.unwrap_or_else(|| "Headless TUI".to_string()),
      cursor_position: (0, 0),
      cursor_visible: true,
      mouse_capture: false,
      application_mode: false,
      event_queue: Arc::new(Mutex::new(VecDeque::new())),
      event_sender: None,
      output_buffer: Arc::new(Mutex::new(String::new())),
      start_time: Instant::now(),
    })
  }

  /// Inject a key event for testing
  pub fn inject_key_event(&mut self, key: KeyEvent) {
    let mut queue = self.event_queue.lock().unwrap();
    queue.push_back(DriverEvent::Key(key));
  }

  /// Inject a mouse event for testing
  pub fn inject_mouse_event(&mut self, mouse: MouseEvent) {
    if self.capabilities.supports_mouse {
      let mut queue = self.event_queue.lock().unwrap();
      queue.push_back(DriverEvent::Mouse(mouse));
    }
  }

  /// Inject a resize event for testing
  pub fn inject_resize_event(&mut self, cols: u16, rows: u16) {
    self.size = (cols, rows);
    let mut queue = self.event_queue.lock().unwrap();
    queue.push_back(DriverEvent::Resize(cols, rows));
  }

  /// Inject a quit event for testing
  pub fn inject_quit_event(&mut self) {
    let mut queue = self.event_queue.lock().unwrap();
    queue.push_back(DriverEvent::Quit);
  }

  /// Inject a custom event for testing
  pub fn inject_custom_event(&mut self, name: String, data: serde_json::Value) {
    let mut queue = self.event_queue.lock().unwrap();
    queue.push_back(DriverEvent::Custom(name, data));
  }

  /// Get the captured output for testing
  pub fn get_output(&self) -> String {
    self.output_buffer.lock().unwrap().clone()
  }

  /// Clear the captured output
  pub fn clear_output(&mut self) {
    self.output_buffer.lock().unwrap().clear();
  }

  /// Get the current cursor position
  pub fn get_cursor_position(&self) -> (u16, u16) {
    self.cursor_position
  }

  /// Check if cursor is visible
  pub fn is_cursor_visible(&self) -> bool {
    self.cursor_visible
  }

  /// Check if mouse capture is enabled
  pub fn is_mouse_capture_enabled(&self) -> bool {
    self.mouse_capture
  }

  /// Get the current title
  pub fn get_title(&self) -> &str {
    &self.title
  }

  /// Get uptime since driver creation
  pub fn get_uptime(&self) -> Duration {
    self.start_time.elapsed()
  }

  /// Simulate typing a string
  pub fn type_string(&mut self, text: &str) {
    for ch in text.chars() {
      let key_event = KeyEvent::new(KeyCode::Char(ch), KeyModifiers::NONE);
      self.inject_key_event(key_event);
    }
  }

  /// Simulate pressing Enter
  pub fn press_enter(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE));
  }

  /// Simulate pressing Escape
  pub fn press_escape(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Esc, KeyModifiers::NONE));
  }

  /// Simulate pressing Tab
  pub fn press_tab(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Tab, KeyModifiers::NONE));
  }

  /// Simulate pressing a function key
  pub fn press_function_key(&mut self, f_key: u8) {
    self.inject_key_event(KeyEvent::new(KeyCode::F(f_key), KeyModifiers::NONE));
  }

  /// Simulate pressing Ctrl+C
  pub fn press_ctrl_c(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Char('c'), KeyModifiers::CONTROL));
  }

  /// Simulate pressing Ctrl+Z
  pub fn press_ctrl_z(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Char('z'), KeyModifiers::CONTROL));
  }

  /// Simulate arrow key presses
  pub fn press_arrow_up(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Up, KeyModifiers::NONE));
  }

  pub fn press_arrow_down(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Down, KeyModifiers::NONE));
  }

  pub fn press_arrow_left(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Left, KeyModifiers::NONE));
  }

  pub fn press_arrow_right(&mut self) {
    self.inject_key_event(KeyEvent::new(KeyCode::Right, KeyModifiers::NONE));
  }

  /// Simulate mouse click
  pub fn click_mouse(&mut self, x: u16, y: u16, button: MouseButton) {
    let mouse_event = MouseEvent {
      kind: MouseEventKind::Down(button),
      column: x,
      row: y,
      modifiers: KeyModifiers::NONE,
    };
    self.inject_mouse_event(mouse_event);

    // Also inject the mouse up event
    let mouse_up_event = MouseEvent {
      kind: MouseEventKind::Up(button),
      column: x,
      row: y,
      modifiers: KeyModifiers::NONE,
    };
    self.inject_mouse_event(mouse_up_event);
  }

  /// Simulate mouse drag
  pub fn drag_mouse(
    &mut self,
    start_x: u16,
    start_y: u16,
    end_x: u16,
    end_y: u16,
    button: MouseButton,
  ) {
    // Mouse down
    self.inject_mouse_event(MouseEvent {
      kind: MouseEventKind::Down(button),
      column: start_x,
      row: start_y,
      modifiers: KeyModifiers::NONE,
    });

    // Simulate drag movement (simplified - just a few intermediate points)
    let steps = 5;
    for i in 1..=steps {
      let x = start_x + ((end_x as i32 - start_x as i32) * i / steps) as u16;
      let y = start_y + ((end_y as i32 - start_y as i32) * i / steps) as u16;

      self.inject_mouse_event(MouseEvent {
        kind: MouseEventKind::Drag(button),
        column: x,
        row: y,
        modifiers: KeyModifiers::NONE,
      });
    }

    // Mouse up
    self.inject_mouse_event(MouseEvent {
      kind: MouseEventKind::Up(button),
      column: end_x,
      row: end_y,
      modifiers: KeyModifiers::NONE,
    });
  }

  /// Simulate mouse scroll
  pub fn scroll_mouse(&mut self, x: u16, y: u16, up: bool) {
    let kind = if up {
      MouseEventKind::ScrollUp
    } else {
      MouseEventKind::ScrollDown
    };

    self.inject_mouse_event(MouseEvent {
      kind,
      column: x,
      row: y,
      modifiers: KeyModifiers::NONE,
    });
  }

  /// Parse ANSI escape sequences from output (basic implementation)
  fn parse_ansi_sequences(&mut self, data: &str) {
    // Simple ANSI parser to track cursor position and screen state
    let mut chars = data.chars().peekable();

    while let Some(ch) = chars.next() {
      if ch == '\x1b' && chars.peek() == Some(&'[') {
        chars.next(); // consume '['
        let mut seq = String::new();

        // Collect the escape sequence
        for seq_char in chars.by_ref() {
          seq.push(seq_char);
          if seq_char.is_alphabetic() {
            break;
          }
        }

        // Parse common sequences
        match seq.chars().last() {
          Some('H') => {
            // Cursor position
            if seq.len() == 1 {
              self.cursor_position = (0, 0);
            } else {
              let coords: Vec<&str> = seq.trim_end_matches('H').split(';').collect();
              if coords.len() == 2 {
                if let (Ok(row), Ok(col)) = (coords[0].parse::<u16>(), coords[1].parse::<u16>()) {
                  self.cursor_position = (col.saturating_sub(1), row.saturating_sub(1));
                }
              }
            }
          }
          Some('J') => {
            // Clear screen
            if seq == "2J" {
              // Clear entire screen
            }
          }
          Some('h') | Some('l') => {
            // Set/reset mode
            if seq.contains("?25") {
              // Cursor visibility
              self.cursor_visible = seq.ends_with('h');
            }
            if seq.contains("?1000") || seq.contains("?1002") || seq.contains("?1003") {
              // Mouse tracking
              self.mouse_capture = seq.ends_with('h');
            }
          }
          _ => {}
        }
      } else {
        // Regular character - update cursor position
        match ch {
          '\n' => {
            self.cursor_position.1 = self.cursor_position.1.saturating_add(1);
            self.cursor_position.0 = 0;
          }
          '\r' => {
            self.cursor_position.0 = 0;
          }
          '\t' => {
            self.cursor_position.0 = ((self.cursor_position.0 / 8) + 1) * 8;
            if self.cursor_position.0 >= self.size.0 {
              self.cursor_position.0 = 0;
              self.cursor_position.1 = self.cursor_position.1.saturating_add(1);
            }
          }
          _ if ch.is_control() => {
            // Ignore other control characters
          }
          _ => {
            // Printable character
            self.cursor_position.0 = self.cursor_position.0.saturating_add(1);
            if self.cursor_position.0 >= self.size.0 {
              self.cursor_position.0 = 0;
              self.cursor_position.1 = self.cursor_position.1.saturating_add(1);
            }
          }
        }
      }
    }
  }
}

impl Driver for HeadlessDriver {
  fn start_application_mode(&mut self) -> Result<()> {
    if self.application_mode {
      return Err(TuiError::driver("Already in application mode"));
    }

    self.application_mode = true;

    // Simulate terminal setup
    self.cursor_position = (0, 0);
    self.cursor_visible = true;

    Ok(())
  }

  fn stop_application_mode(&mut self) -> Result<()> {
    if !self.application_mode {
      return Ok(()); // Already stopped
    }

    self.application_mode = false;

    // Simulate terminal cleanup
    self.mouse_capture = false;

    Ok(())
  }

  fn write(&mut self, data: &str) -> Result<()> {
    // Capture output for testing
    {
      let mut buffer = self.output_buffer.lock().unwrap();
      buffer.push_str(data);
    }

    // Parse ANSI sequences to maintain state
    self.parse_ansi_sequences(data);

    Ok(())
  }

  fn flush(&mut self) -> Result<()> {
    // Nothing to flush in headless mode
    Ok(())
  }

  fn get_terminal_size(&self) -> Result<(u16, u16)> {
    Ok(self.size)
  }

  fn capabilities(&self) -> &DriverCapabilities {
    &self.capabilities
  }

  fn start_event_loop(&mut self, event_sender: mpsc::UnboundedSender<DriverEvent>) -> Result<()> {
    self.event_sender = Some(event_sender.clone());

    // Spawn background task to process events
    let event_queue = self.event_queue.clone();
    let sender = event_sender.clone();

    tokio::spawn(async move {
      loop {
        // Check for events in the queue
        let event = {
          let mut queue = event_queue.lock().unwrap();
          queue.pop_front()
        };

        if let Some(event) = event {
          if sender.send(event).is_err() {
            // Receiver dropped, exit loop
            break;
          }
        } else {
          // No events, sleep briefly
          sleep(Duration::from_millis(10)).await;
        }
      }
    });

    // Send initial resize event
    let _ = event_sender.send(DriverEvent::Resize(self.size.0, self.size.1));

    Ok(())
  }

  fn stop_event_loop(&mut self) -> Result<()> {
    self.event_sender = None;
    Ok(())
  }

  fn set_cursor_position(&mut self, x: u16, y: u16) -> Result<()> {
    self.cursor_position = (
      x.min(self.size.0.saturating_sub(1)),
      y.min(self.size.1.saturating_sub(1)),
    );

    // Write ANSI sequence to output buffer
    let ansi_seq = format!("\x1b[{};{}H", y + 1, x + 1);
    self.write(&ansi_seq)?;

    Ok(())
  }

  fn set_cursor_visible(&mut self, visible: bool) -> Result<()> {
    self.cursor_visible = visible;

    // Write ANSI sequence to output buffer
    let ansi_seq = if visible { "\x1b[?25h" } else { "\x1b[?25l" };
    self.write(ansi_seq)?;

    Ok(())
  }

  fn set_title(&mut self, title: &str) -> Result<()> {
    self.title = title.to_string();

    // Write ANSI sequence to output buffer
    let ansi_seq = format!("\x1b]2;{title}\x1b\\");
    self.write(&ansi_seq)?;

    Ok(())
  }

  fn set_mouse_capture(&mut self, enabled: bool) -> Result<()> {
    if self.capabilities.supports_mouse {
      self.mouse_capture = enabled;

      // Write ANSI sequences to output buffer
      let ansi_seq = if enabled {
        "\x1b[?1000h\x1b[?1002h\x1b[?1015h\x1b[?1006h"
      } else {
        "\x1b[?1000l\x1b[?1002l\x1b[?1015l\x1b[?1006l"
      };
      self.write(ansi_seq)?;
    }

    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use tokio::time::{timeout, Duration};

  #[test]
  fn test_headless_driver_creation() {
    let config = DriverConfig {
      size: Some((100, 30)),
      mouse: true,
      ..Default::default()
    };

    let driver = HeadlessDriver::new(config).unwrap();
    assert_eq!(driver.get_terminal_size().unwrap(), (100, 30));
    assert!(driver.capabilities().is_headless);
    assert!(driver.capabilities().supports_mouse);
  }

  #[test]
  fn test_application_mode() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    assert!(!driver.application_mode);
    driver.start_application_mode().unwrap();
    assert!(driver.application_mode);
    driver.stop_application_mode().unwrap();
    assert!(!driver.application_mode);
  }

  #[test]
  fn test_output_capture() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    driver.write("Hello, World!").unwrap();
    assert_eq!(driver.get_output(), "Hello, World!");

    driver.clear_output();
    assert_eq!(driver.get_output(), "");
  }

  #[test]
  fn test_cursor_operations() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    driver.set_cursor_position(10, 5).unwrap();
    assert_eq!(driver.get_cursor_position(), (10, 5));

    driver.set_cursor_visible(false).unwrap();
    assert!(!driver.is_cursor_visible());

    driver.set_cursor_visible(true).unwrap();
    assert!(driver.is_cursor_visible());
  }

  #[test]
  fn test_event_injection() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    driver.type_string("hello");
    driver.press_enter();
    driver.press_escape();

    let queue = driver.event_queue.lock().unwrap();
    assert_eq!(queue.len(), 7); // 5 chars + Enter + Escape
  }

  #[test]
  fn test_mouse_simulation() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    driver.click_mouse(10, 5, MouseButton::Left);
    driver.scroll_mouse(20, 10, true);

    let queue = driver.event_queue.lock().unwrap();
    assert_eq!(queue.len(), 3); // Click (down+up) + Scroll
  }

  #[tokio::test]
  async fn test_event_loop() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();
    driver.start_application_mode().unwrap();

    let (tx, mut rx) = mpsc::unbounded_channel();
    driver.start_event_loop(tx).unwrap();

    // Inject a test event
    driver.press_enter();

    // Should receive the resize event first, then our key event
    let resize_event = timeout(Duration::from_millis(100), rx.recv())
      .await
      .unwrap()
      .unwrap();
    match resize_event {
      DriverEvent::Resize(80, 24) => {} // Default size
      _ => panic!("Expected resize event"),
    }

    let key_event = timeout(Duration::from_millis(100), rx.recv())
      .await
      .unwrap()
      .unwrap();
    match key_event {
      DriverEvent::Key(key) => assert_eq!(key.code, KeyCode::Enter),
      _ => panic!("Expected key event"),
    }

    driver.stop_event_loop().unwrap();
    driver.stop_application_mode().unwrap();
  }

  #[test]
  fn test_ansi_parsing() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    // Test cursor positioning
    driver.write("\x1b[10;20H").unwrap();
    assert_eq!(driver.get_cursor_position(), (19, 9)); // 0-based

    // Test cursor visibility
    driver.write("\x1b[?25l").unwrap();
    assert!(!driver.is_cursor_visible());

    driver.write("\x1b[?25h").unwrap();
    assert!(driver.is_cursor_visible());
  }

  #[test]
  fn test_convenience_methods() {
    let mut driver = HeadlessDriver::new(DriverConfig::default()).unwrap();

    driver.press_ctrl_c();
    driver.press_function_key(1);
    driver.press_arrow_up();
    driver.drag_mouse(0, 0, 10, 10, MouseButton::Left);

    let queue = driver.event_queue.lock().unwrap();
    assert!(!queue.is_empty());
  }
}
