//! Unix/Linux/macOS driver with full POSIX terminal support
//!
//! Provides complete terminal control including:
//! - Raw mode terminal input/output
//! - Signal handling (SIGWINCH, SIGTSTP, SIGCONT)
//! - Mouse capture and tracking
//! - Async event processing
//! - Suspend/resume support

use super::{Driver, DriverCapabilities, DriverConfig, DriverEvent};
use crate::error::{Result, TuiError};
use crossterm::{
  cursor,
  event::{self, Event, KeyCode, KeyModifiers},
  execute,
  style::Print,
  terminal::{self, Clear, ClearType, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self, Write};
use std::sync::{
  atomic::{AtomicBool, Ordering},
  Arc, Mutex,
};
use std::thread;
use std::time::Duration;
use tokio::sync::mpsc;

#[cfg(unix)]
extern crate libc;

/// Unix driver for Linux, macOS, and other POSIX systems
pub struct UnixDriver {
  capabilities: DriverCapabilities,
  config: DriverConfig,
  application_mode: Arc<AtomicBool>,
  event_sender: Option<mpsc::UnboundedSender<DriverEvent>>,
  event_thread_handle: Option<thread::JoinHandle<()>>,
  stop_flag: Arc<AtomicBool>,
  // Signal handling thread
  signal_thread_handle: Option<thread::JoinHandle<()>>,
  signal_stop: Arc<AtomicBool>,

  // Terminal state tracking
  original_raw_mode: Option<bool>,
  original_cursor_visible: Option<bool>,
  original_in_alternate_screen: bool,
  mouse_capture_enabled: bool,
  cursor_visible: bool,
  current_title: String,

  // Signal handling
  signal_handlers_installed: bool,
  suspend_callback: Option<Box<dyn Fn() + Send + Sync>>,
  resume_callback: Option<Box<dyn Fn() + Send + Sync>>,

  // Output handling with buffering support
  stdout: Arc<Mutex<io::Stdout>>,
  /// Internal write buffer for batching operations
  write_buffer: Vec<u8>,
}

impl UnixDriver {
  /// Create a new Unix driver
  pub fn new(config: DriverConfig) -> Result<Self> {
    #[cfg(feature = "tracing")]
    tracing::trace!("UnixDriver::new - begin");
    let capabilities = DriverCapabilities {
      can_suspend: true, // Unix systems support Ctrl+Z
      is_headless: false,
      is_inline: config.inline,
      is_web: false,
      supports_mouse: config.mouse,
      supports_colors: Self::detect_color_support(),
      max_colors: Self::detect_max_colors(),
    };

    let driver = Self {
      capabilities,
      config: config.clone(),
      application_mode: Arc::new(AtomicBool::new(false)),
      event_sender: None,
      event_thread_handle: None,
      stop_flag: Arc::new(AtomicBool::new(false)),
      signal_thread_handle: None,
      signal_stop: Arc::new(AtomicBool::new(false)),
      original_raw_mode: None,
      original_cursor_visible: None,
      original_in_alternate_screen: false,
      mouse_capture_enabled: false,
      cursor_visible: true,
      current_title: config
        .title
        .unwrap_or_else(|| "TUI Application".to_string()),
      signal_handlers_installed: false,
      suspend_callback: None,
      resume_callback: None,
      stdout: Arc::new(Mutex::new(io::stdout())),
      write_buffer: Vec::with_capacity(4096),
    };
    #[cfg(feature = "tracing")]
    tracing::trace!("UnixDriver::new - end");
    Ok(driver)
  }

  /// Detect color support based on environment
  fn detect_color_support() -> bool {
    // Check if we're in a TTY
    if !crossterm::tty::IsTty::is_tty(&io::stdout()) {
      return false;
    }

    // Check environment variables
    if let Ok(term) = std::env::var("TERM") {
      if term.contains("color") || term.contains("256") || term.contains("truecolor") {
        return true;
      }
    }

    if let Ok(colorterm) = std::env::var("COLORTERM") {
      if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return true;
      }
    }

    // Check for specific terminal emulators known to support colors
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
      match term_program.as_str() {
        "iTerm.app" | "Apple_Terminal" | "WezTerm" | "Alacritty" => return true,
        _ => {}
      }
    }

    // Default to true for TTY
    true
  }

  /// Detect maximum color support
  fn detect_max_colors() -> u32 {
    if !Self::detect_color_support() {
      return 1;
    }

    // Check for truecolor support
    if let Ok(colorterm) = std::env::var("COLORTERM") {
      if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return 16_777_216; // 24-bit color
      }
    }

    // Check TERM variable
    if let Ok(term) = std::env::var("TERM") {
      if term.contains("256") {
        return 256;
      } else if term.contains("color") {
        return 16;
      }
    }

    // Check terminal capabilities using terminfo (simplified)
    if let Ok(colors) = std::env::var("COLORS") {
      if let Ok(color_count) = colors.parse::<u32>() {
        return color_count;
      }
    }

    // Default to 256 colors for modern terminals
    256
  }

  /// Install signal handlers marker (actual thread starts with event loop)
  fn install_signal_handlers(&mut self) -> Result<()> {
    if self.signal_handlers_installed {
      return Ok(());
    }
    // Defer starting the signal thread until we have an event_sender in start_event_loop
    self.signal_handlers_installed = true;
    Ok(())
  }

  /// Spawn a dedicated thread to handle SIGWINCH/SIGTSTP/SIGCONT safely
  fn spawn_signal_thread(&mut self, tx: mpsc::UnboundedSender<DriverEvent>) -> Result<()> {
    use signal_hook::consts::{SIGCONT, SIGTSTP, SIGWINCH};
    use signal_hook::iterator::Signals;

    // Reset stop flag
    self.signal_stop.store(false, Ordering::Relaxed);

    let stop = self.signal_stop.clone();
    let _stop_dbg = &self.signal_stop; // keep for possible future diagnostics

    #[cfg(feature = "tracing")]
    tracing::debug!("UnixDriver signal thread starting");
    let handle = thread::spawn(move || {
      let mut signals = match Signals::new([SIGWINCH, SIGTSTP, SIGCONT]) {
        Ok(s) => s,
        Err(e) => {
          eprintln!("[signals] failed to create iterator: {e}");
          return;
        }
      };

      // Helper: emit resize using crossterm size (avoid borrowing driver)
      let emit_resize = |tx: &mpsc::UnboundedSender<DriverEvent>| {
        if let Ok((c, r)) = terminal::size() {
          let _ = tx.send(DriverEvent::Resize(c, r));
        }
      };

      while !stop.load(Ordering::Relaxed) {
        for sig in signals.pending() {
          match sig {
            SIGWINCH => {
              emit_resize(&tx);
            }
            SIGTSTP => {
              // Best-effort: disable raw mode before default suspend
              let _ = terminal::disable_raw_mode();
              emit_resize(&tx);
              unsafe { libc::raise(libc::SIGTSTP) };
            }
            SIGCONT => {
              // Best-effort: re-enable raw mode and emit resize
              let _ = terminal::enable_raw_mode();
              emit_resize(&tx);
            }
            _ => {}
          }
        }
        thread::sleep(Duration::from_millis(50));
      }
    });

    self.signal_thread_handle = Some(handle);
    Ok(())
  }

  /// Check if the terminal is currently in raw mode
  fn is_raw_mode_enabled(&self) -> bool {
    // crossterm doesn't provide a direct way to check raw mode state,
    // so we use a platform-specific approach
    #[cfg(unix)]
    {
      use std::mem;
      use std::os::unix::io::AsRawFd;

      let fd = io::stdout().as_raw_fd();

      // Get current terminal attributes
      let mut termios: libc::termios = unsafe { mem::zeroed() };
      if unsafe { libc::tcgetattr(fd, &mut termios) } == 0 {
        // Check if ICANON (canonical mode) is disabled - indicates raw mode
        (termios.c_lflag & libc::ICANON) == 0
      } else {
        // If we can't get terminal attributes, assume not in raw mode
        false
      }
    }

    #[cfg(not(unix))]
    {
      // For non-Unix systems, we can't easily check raw mode
      // This should not be reached in UnixDriver, but provide fallback
      false
    }
  }

  /// Set up the terminal for application mode
  fn setup_terminal(&mut self) -> Result<()> {
    // Capture original state before making changes
    self.original_raw_mode = Some(self.is_raw_mode_enabled());
    self.original_cursor_visible = Some(self.cursor_visible);
    // We assume we start NOT in alternate screen (typical case)
    self.original_in_alternate_screen = false;

    {
      let mut stdout = self
        .stdout
        .lock()
        .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;

      if !self.config.inline {
        execute!(stdout, EnterAlternateScreen)?;
      }

      // Hide cursor initially
      execute!(stdout, cursor::Hide)?;

      // Clear screen
      execute!(stdout, Clear(ClearType::All))?;

      // Set title if provided
      if !self.current_title.is_empty() {
        execute!(
          stdout,
          Print(format!("\x1b]2;{}\x1b\\", self.current_title))
        )?;
      }

      stdout.flush()?;
    }

    terminal::enable_raw_mode()?;
    self.cursor_visible = false;

    // Set up mouse capture if enabled
    if self.capabilities.supports_mouse {
      let mut stdout = self
        .stdout
        .lock()
        .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
      execute!(
        stdout,
        Print("\\x1b[?1000h"), // Basic mouse reporting
        Print("\\x1b[?1002h"), // Button event tracking
        Print("\\x1b[?1006h"), // SGR extended reporting
        Print("\\x1b[?1015h"), // urxvt extended reporting
      )?;
      stdout.flush()?;
      self.mouse_capture_enabled = true;
    }

    Ok(())
  }

  /// Restore terminal to original state
  fn cleanup_terminal(&mut self) -> Result<()> {
    // Disable mouse capture
    if self.mouse_capture_enabled {
      let mut stdout = self
        .stdout
        .lock()
        .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
      execute!(
        stdout,
        Print("\\x1b[?1015l"), // Disable urxvt extended reporting
        Print("\\x1b[?1006l"), // Disable SGR extended reporting
        Print("\\x1b[?1002l"), // Disable button event tracking
        Print("\\x1b[?1000l"), // Disable basic mouse reporting
      )?;
      stdout.flush()?;
      self.mouse_capture_enabled = false;
    }

    {
      let mut stdout = self
        .stdout
        .lock()
        .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;

      // Restore cursor visibility to original state
      if let Some(original_visible) = self.original_cursor_visible {
        if original_visible && !self.cursor_visible {
          execute!(stdout, cursor::Show)?;
        } else if !original_visible && self.cursor_visible {
          execute!(stdout, cursor::Hide)?;
        }
      } else {
        // If we don't know original state, show cursor (safer default)
        execute!(stdout, cursor::Show)?;
      }

      // Leave alternate screen if we entered it and weren't originally in it
      if !self.original_in_alternate_screen && !self.config.inline {
        execute!(stdout, LeaveAlternateScreen)?;
      }

      stdout.flush()?;
    }

    // Restore raw mode to original state
    if let Some(original_raw) = self.original_raw_mode {
      if original_raw {
        // Terminal was originally in raw mode, ensure it stays enabled
        if !self.is_raw_mode_enabled() {
          terminal::enable_raw_mode()?;
        }
      } else {
        // Terminal was not in raw mode, disable it
        if self.is_raw_mode_enabled() {
          terminal::disable_raw_mode()?;
        }
      }
    } else {
      // If we don't know original state, disable raw mode (safer default)
      if self.is_raw_mode_enabled() {
        terminal::disable_raw_mode()?;
      }
    }

    Ok(())
  }

  /// Enable mouse capture and tracking
  #[allow(dead_code)]
  fn enable_mouse_capture(&mut self, stdout: &mut impl Write) -> Result<()> {
    if !self.capabilities.supports_mouse {
      return Ok(());
    }

    // Enable mouse tracking modes
    // 1000: Basic mouse reporting
    // 1002: Button event tracking
    // 1003: All event tracking
    // 1006: SGR extended reporting
    // 1015: urxvt extended reporting
    execute!(
      stdout,
      Print("\x1b[?1000h"), // Basic mouse reporting
      Print("\x1b[?1002h"), // Button event tracking
      Print("\x1b[?1006h"), // SGR extended reporting
      Print("\x1b[?1015h"), // urxvt extended reporting
    )?;

    self.mouse_capture_enabled = true;
    Ok(())
  }

  /// Disable mouse capture
  #[allow(dead_code)]
  fn disable_mouse_capture(&mut self, stdout: &mut impl Write) -> Result<()> {
    if !self.mouse_capture_enabled {
      return Ok(());
    }

    execute!(
      stdout,
      Print("\x1b[?1015l"), // Disable urxvt extended reporting
      Print("\x1b[?1006l"), // Disable SGR extended reporting
      Print("\x1b[?1002l"), // Disable button event tracking
      Print("\x1b[?1000l"), // Disable basic mouse reporting
    )?;

    self.mouse_capture_enabled = false;
    Ok(())
  }

  /// Set terminal title
  #[allow(dead_code)]
  fn set_terminal_title(&mut self, stdout: &mut impl Write, title: &str) -> Result<()> {
    // Use OSC (Operating System Command) sequence to set title
    execute!(stdout, Print(format!("\x1b]2;{title}\x1b\\")))?;
    Ok(())
  }

  /// Process events in a background thread
  fn event_loop(
    event_sender: mpsc::UnboundedSender<DriverEvent>,
    stop_flag: Arc<AtomicBool>,
    supports_mouse: bool,
  ) {
    while !stop_flag.load(Ordering::Relaxed) {
      // Check for events with a timeout
      match event::poll(Duration::from_millis(100)) {
        Ok(true) => {
          match event::read() {
            Ok(Event::Key(key_event)) => {
              // Handle special key combinations
              if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                match key_event.code {
                  KeyCode::Char('c') => {
                    // Ctrl+C - send quit event
                    if event_sender.send(DriverEvent::Quit).is_err() {
                      break;
                    }
                    continue;
                  }
                  KeyCode::Char('z') => {
                    // Ctrl+Z - suspend (handled by system)
                    // We could send a custom event here if needed
                  }
                  _ => {}
                }
              }

              if event_sender.send(DriverEvent::Key(key_event)).is_err() {
                break;
              }
            }
            Ok(Event::Mouse(mouse_event)) => {
              if supports_mouse && event_sender.send(DriverEvent::Mouse(mouse_event)).is_err() {
                break;
              }
            }
            Ok(Event::Resize(cols, rows)) => {
              if event_sender.send(DriverEvent::Resize(cols, rows)).is_err() {
                break;
              }
            }
            Ok(Event::FocusGained) => {
              let custom_event = serde_json::json!({"type": "focus_gained"});
              if event_sender
                .send(DriverEvent::Custom("focus".to_string(), custom_event))
                .is_err()
              {
                break;
              }
            }
            Ok(Event::FocusLost) => {
              let custom_event = serde_json::json!({"type": "focus_lost"});
              if event_sender
                .send(DriverEvent::Custom("focus".to_string(), custom_event))
                .is_err()
              {
                break;
              }
            }
            Ok(Event::Paste(text)) => {
              let custom_event = serde_json::json!({"type": "paste", "text": text});
              if event_sender
                .send(DriverEvent::Custom("paste".to_string(), custom_event))
                .is_err()
              {
                break;
              }
            }
            Err(e) => {
              eprintln!("Error reading event: {e}");
              // Continue processing other events
            }
          }
        }
        Ok(false) => {
          // No events available, continue polling
        }
        Err(e) => {
          eprintln!("Error polling events: {e}");
          // Small delay to prevent busy loop on persistent errors
          thread::sleep(Duration::from_millis(10));
        }
      }
    }
  }

  /// Set callback for suspend events
  pub fn set_suspend_callback<F>(&mut self, callback: F)
  where
    F: Fn() + Send + Sync + 'static,
  {
    self.suspend_callback = Some(Box::new(callback));
  }

  /// Set callback for resume events
  pub fn set_resume_callback<F>(&mut self, callback: F)
  where
    F: Fn() + Send + Sync + 'static,
  {
    self.resume_callback = Some(Box::new(callback));
  }

  /// Check if the terminal supports the given capability
  pub fn supports_capability(&self, capability: &str) -> bool {
    match capability {
      "colors" => self.capabilities.supports_colors,
      "mouse" => self.capabilities.supports_mouse,
      "suspend" => self.capabilities.can_suspend,
      "alternate_screen" => !self.capabilities.is_inline,
      "title" => true,        // Most terminals support title setting
      "cursor_shape" => true, // Most terminals support cursor shape changes
      _ => false,
    }
  }

  /// Get detailed terminal information
  pub fn get_terminal_info(&self) -> serde_json::Value {
    serde_json::json!({
        "type": "unix",
        "colors": self.capabilities.max_colors,
        "supports_mouse": self.capabilities.supports_mouse,
        "can_suspend": self.capabilities.can_suspend,
        "is_inline": self.capabilities.is_inline,
        "environment": {
            "TERM": std::env::var("TERM").unwrap_or_default(),
            "COLORTERM": std::env::var("COLORTERM").unwrap_or_default(),
            "TERM_PROGRAM": std::env::var("TERM_PROGRAM").unwrap_or_default(),
        }
    })
  }
}

impl Driver for UnixDriver {
  fn start_application_mode(&mut self) -> Result<()> {
    if self.application_mode.load(Ordering::Relaxed) {
      return Err(TuiError::driver("Already in application mode"));
    }

    self.install_signal_handlers()?;
    self.setup_terminal()?;

    self.application_mode.store(true, Ordering::Relaxed);
    Ok(())
  }

  fn stop_application_mode(&mut self) -> Result<()> {
    if !self.application_mode.load(Ordering::Relaxed) {
      return Ok(()); // Already stopped
    }

    self.application_mode.store(false, Ordering::Relaxed);

    // Stop event loop first
    self.stop_event_loop()?;

    // Clean up terminal
    self.cleanup_terminal()?;

    Ok(())
  }

  fn write(&mut self, data: &str) -> Result<()> {
    // Buffer writes for better performance and reduced flickering
    self.write_buffer.extend_from_slice(data.as_bytes());
    Ok(())
  }

  fn flush(&mut self) -> Result<()> {
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;

    // Write all buffered data in one atomic operation
    if !self.write_buffer.is_empty() {
      stdout.write_all(&self.write_buffer)?;
      self.write_buffer.clear();
    }

    stdout.flush()?;
    Ok(())
  }

  fn get_terminal_size(&self) -> Result<(u16, u16)> {
    if let Some(size) = self.config.size {
      Ok(size)
    } else {
      let (cols, rows) = terminal::size()?;
      Ok((cols, rows))
    }
  }

  fn capabilities(&self) -> &DriverCapabilities {
    &self.capabilities
  }

  fn start_event_loop(&mut self, event_sender: mpsc::UnboundedSender<DriverEvent>) -> Result<()> {
    if self.event_thread_handle.is_some() {
      return Err(TuiError::driver("Event loop already running"));
    }

    self.event_sender = Some(event_sender.clone());
    self.stop_flag.store(false, Ordering::Relaxed);

    // Send initial resize event
    let (cols, rows) = self.get_terminal_size()?;
    let _ = event_sender.send(DriverEvent::Resize(cols, rows));

    // Start event processing thread
    let stop_flag = self.stop_flag.clone();
    let supports_mouse = self.capabilities.supports_mouse;

    #[cfg(feature = "tracing")]
    tracing::debug!("UnixDriver event loop thread starting");
    let handle = thread::spawn(move || {
      Self::event_loop(event_sender.clone(), stop_flag, supports_mouse);
    });

    // Start signal handling thread now that we have a sender
    let _ = self.spawn_signal_thread(self.event_sender.as_ref().unwrap().clone());

    self.event_thread_handle = Some(handle);
    Ok(())
  }

  fn stop_event_loop(&mut self) -> Result<()> {
    self.stop_flag.store(true, Ordering::Relaxed);

    // Stop signal thread
    self.signal_stop.store(true, Ordering::Relaxed);
    if let Some(h) = self.signal_thread_handle.take() {
      let _ = h.join();
    }

    if let Some(handle) = self.event_thread_handle.take() {
      // Wait for thread to finish
      if let Err(e) = handle.join() {
        eprintln!("Error joining event thread: {e:?}");
      }
    }

    self.event_sender = None;
    Ok(())
  }

  fn suspend(&mut self) -> Result<()> {
    if !self.capabilities.can_suspend {
      return Err(TuiError::driver("Suspend not supported"));
    }

    // Call suspend callback if set
    if let Some(callback) = &self.suspend_callback {
      callback();
    }

    self.stop_application_mode()?;

    // Send SIGTSTP to ourselves (suspend)
    unsafe {
      libc::raise(libc::SIGTSTP);
    }

    Ok(())
  }

  fn resume(&mut self) -> Result<()> {
    if !self.capabilities.can_suspend {
      return Err(TuiError::driver("Resume not supported"));
    }

    self.start_application_mode()?;

    // Call resume callback if set
    if let Some(callback) = &self.resume_callback {
      callback();
    }

    Ok(())
  }

  fn set_cursor_position(&mut self, x: u16, y: u16) -> Result<()> {
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
    execute!(stdout, cursor::MoveTo(x, y))?;
    Ok(())
  }

  fn set_cursor_visible(&mut self, visible: bool) -> Result<()> {
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;

    if visible && !self.cursor_visible {
      execute!(stdout, cursor::Show)?;
    } else if !visible && self.cursor_visible {
      execute!(stdout, cursor::Hide)?;
    }

    self.cursor_visible = visible;
    Ok(())
  }

  fn set_title(&mut self, title: &str) -> Result<()> {
    self.current_title = title.to_string();
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
    execute!(stdout, Print(format!("\x1b]2;{title}\x1b\\")))?;
    stdout.flush()?;
    Ok(())
  }

  fn set_mouse_capture(&mut self, enabled: bool) -> Result<()> {
    if !self.capabilities.supports_mouse {
      return Ok(());
    }

    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;

    if enabled && !self.mouse_capture_enabled {
      execute!(
        stdout,
        Print("\\x1b[?1000h"), // Basic mouse reporting
        Print("\\x1b[?1002h"), // Button event tracking
        Print("\\x1b[?1006h"), // SGR extended reporting
        Print("\\x1b[?1015h"), // urxvt extended reporting
      )?;
      self.mouse_capture_enabled = true;
    } else if !enabled && self.mouse_capture_enabled {
      execute!(
        stdout,
        Print("\\x1b[?1015l"), // Disable urxvt extended reporting
        Print("\\x1b[?1006l"), // Disable SGR extended reporting
        Print("\\x1b[?1002l"), // Disable button event tracking
        Print("\\x1b[?1000l"), // Disable basic mouse reporting
      )?;
      self.mouse_capture_enabled = false;
    }

    stdout.flush()?;
    Ok(())
  }

  fn clear_screen(&mut self) -> Result<()> {
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    Ok(())
  }

  fn cursor_home(&mut self) -> Result<()> {
    let mut stdout = self
      .stdout
      .lock()
      .map_err(|_| TuiError::driver("Internal error: stdout unavailable (poisoned lock)"))?;
    execute!(stdout, cursor::MoveTo(0, 0))?;
    Ok(())
  }

  fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
    // Buffer raw bytes directly for maximum performance
    self.write_buffer.extend_from_slice(data);
    Ok(())
  }
}

impl Drop for UnixDriver {
  fn drop(&mut self) {
    // Ensure terminal is cleaned up
    let _ = self.stop_application_mode();
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use std::time::Duration;
  use tokio::time::timeout;

  #[test]
  fn test_unix_driver_creation() {
    let config = DriverConfig {
      mouse: true,
      title: Some("Test App".to_string()),
      ..Default::default()
    };

    let driver = UnixDriver::new(config).unwrap();
    assert!(!driver.capabilities.is_headless);
    assert!(driver.capabilities.can_suspend);
    assert_eq!(driver.current_title, "Test App");
  }

  #[test]
  fn test_color_detection() {
    // Test environment-based color detection
    // Note: In test environments without TTY, color detection will return 1
    // This is expected behavior for non-interactive environments

    // Use a guard to ensure environment is restored even on panic
    struct EnvGuard {
      term: Option<String>,
      colorterm: Option<String>,
    }

    impl Drop for EnvGuard {
      fn drop(&mut self) {
        // Restore original environment
        match &self.term {
          Some(val) => std::env::set_var("TERM", val),
          None => std::env::remove_var("TERM"),
        }
        match &self.colorterm {
          Some(val) => std::env::set_var("COLORTERM", val),
          None => std::env::remove_var("COLORTERM"),
        }
      }
    }

    // Save original environment
    let _guard = EnvGuard {
      term: std::env::var("TERM").ok(),
      colorterm: std::env::var("COLORTERM").ok(),
    };

    // Clear COLORTERM to test TERM-based detection
    std::env::remove_var("COLORTERM");
    std::env::set_var("TERM", "xterm-256color");
    let color_support = UnixDriver::detect_color_support();
    let max_colors = UnixDriver::detect_max_colors();

    // In TTY environments, we expect 256 colors; in non-TTY, we get 1
    if color_support {
      assert_eq!(max_colors, 256);
    } else {
      assert_eq!(max_colors, 1); // Expected in test environments
    }

    std::env::set_var("COLORTERM", "truecolor");
    let max_colors_truecolor = UnixDriver::detect_max_colors();
    if color_support {
      assert_eq!(max_colors_truecolor, 16_777_216);
    } else {
      assert_eq!(max_colors_truecolor, 1); // Expected in test environments
    }

    // Guard will automatically restore environment when it goes out of scope
  }

  #[test]
  fn test_capabilities() {
    let driver = UnixDriver::new(DriverConfig::default()).unwrap();

    // Color support depends on TTY detection, which may fail in test environments
    // Test other capabilities that should be consistent
    assert!(driver.supports_capability("mouse"));
    assert!(driver.supports_capability("suspend"));
    assert!(driver.supports_capability("title"));
    assert!(!driver.supports_capability("unknown"));

    // Color capability test - environment dependent
    let _color_support = driver.supports_capability("colors"); // Don't assert
  }

  #[test]
  fn test_terminal_info() {
    let driver = UnixDriver::new(DriverConfig::default()).unwrap();
    let info = driver.get_terminal_info();

    assert_eq!(info["type"], "unix");
    assert!(info["colors"].is_number());
    assert!(info["supports_mouse"].is_boolean());
    assert!(info["environment"].is_object());
  }

  #[test]
  fn test_title_sequence_bytes() {
    let mut driver = UnixDriver::new(DriverConfig::default()).unwrap();
    let mut buf: Vec<u8> = Vec::new();
    driver.set_terminal_title(&mut buf, "Hello").unwrap();
    assert_eq!(buf, b"\x1b]2;Hello\x1b\\");
  }

  // Note: These tests require a TTY and may not work in all CI environments
  #[test]
  #[ignore = "requires TTY"]
  fn test_application_mode() {
    let mut driver = UnixDriver::new(DriverConfig::default()).unwrap();

    // These will only work if we have a TTY
    if crossterm::tty::IsTty::is_tty(&io::stdout()) {
      // Use a guard to ensure cleanup even on panic
      struct AppModeGuard<'a> {
        driver: &'a mut UnixDriver,
        started: bool,
      }

      impl<'a> Drop for AppModeGuard<'a> {
        fn drop(&mut self) {
          if self.started {
            let _ = self.driver.stop_application_mode();
          }
        }
      }

      let mut guard = AppModeGuard {
        driver: &mut driver,
        started: false,
      };

      guard.driver.start_application_mode().unwrap();
      guard.started = true;
      assert!(guard.driver.application_mode.load(Ordering::Relaxed));

      guard.driver.stop_application_mode().unwrap();
      guard.started = false;
      assert!(!guard.driver.application_mode.load(Ordering::Relaxed));
    }
  }

  #[tokio::test]
  #[ignore = "requires TTY"]
  async fn test_event_loop() {
    let mut driver = UnixDriver::new(DriverConfig::default()).unwrap();

    if crossterm::tty::IsTty::is_tty(&io::stdout()) {
      // Use a guard to ensure cleanup even on panic
      struct AppModeGuard<'a> {
        driver: &'a mut UnixDriver,
        started: bool,
      }

      impl<'a> Drop for AppModeGuard<'a> {
        fn drop(&mut self) {
          if self.started {
            let _ = self.driver.stop_event_loop();
            let _ = self.driver.stop_application_mode();
          }
        }
      }

      let mut guard = AppModeGuard {
        driver: &mut driver,
        started: false,
      };

      guard.driver.start_application_mode().unwrap();
      guard.started = true;

      let (tx, mut rx) = mpsc::unbounded_channel();
      guard.driver.start_event_loop(tx).unwrap();

      // Should receive initial resize event
      let event = timeout(Duration::from_millis(100), rx.recv()).await;
      assert!(event.is_ok());

      guard.driver.stop_event_loop().unwrap();
      guard.driver.stop_application_mode().unwrap();
      guard.started = false;
    }
  }
}
