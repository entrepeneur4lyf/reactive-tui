//! Windows driver with full Windows Console API support
//!
//! Provides complete Windows terminal control including:
//! - Windows Console API integration
//! - Windows-specific key handling
//! - Mouse support via Windows Console
//! - Virtual terminal sequences when available
//! - Legacy console fallback support

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

/// Windows driver using Windows Console API and crossterm
pub struct WindowsDriver {
  capabilities: DriverCapabilities,
  config: DriverConfig,
  application_mode: Arc<AtomicBool>,
  event_sender: Option<mpsc::UnboundedSender<DriverEvent>>,
  event_thread_handle: Option<thread::JoinHandle<()>>,
  stop_flag: Arc<AtomicBool>,

  // Terminal state tracking
  mouse_capture_enabled: bool,
  cursor_visible: bool,
  current_title: String,
  virtual_terminal_supported: bool,

  // Original state tracking for restoration
  original_raw_mode: Option<bool>,
  original_cursor_visible: Option<bool>,
  original_in_alternate_screen: bool,

  // Windows-specific
  #[allow(dead_code)]
  original_console_mode: Option<u32>,
  #[allow(dead_code)]
  console_handle: Option<isize>,

  // Output handling
  stdout: Arc<Mutex<io::Stdout>>,
}

impl WindowsDriver {
  /// Create a new Windows driver
  pub fn new(config: DriverConfig) -> Result<Self> {
    let virtual_terminal_supported = Self::check_virtual_terminal_support();

    let capabilities = DriverCapabilities {
      can_suspend: false, // Windows doesn't have Ctrl+Z suspend like Unix
      is_headless: false,
      is_inline: config.inline,
      is_web: false,
      supports_mouse: config.mouse,
      supports_colors: Self::detect_color_support(),
      max_colors: Self::detect_max_colors(),
    };

    Ok(Self {
      capabilities,
      config: config.clone(),
      application_mode: Arc::new(AtomicBool::new(false)),
      event_sender: None,
      event_thread_handle: None,
      stop_flag: Arc::new(AtomicBool::new(false)),
      mouse_capture_enabled: false,
      cursor_visible: true,
      current_title: config
        .title
        .unwrap_or_else(|| "TUI Application".to_string()),
      virtual_terminal_supported,
      original_raw_mode: None,
      original_cursor_visible: None,
      original_in_alternate_screen: false,
      original_console_mode: None,
      console_handle: None,
      stdout: Arc::new(Mutex::new(io::stdout())),
    })
  }

  /// Check if Windows Terminal or virtual terminal sequences are supported
  fn check_virtual_terminal_support() -> bool {
    // Check if we're running in Windows Terminal
    if std::env::var("WT_SESSION").is_ok() {
      return true;
    }

    // Check if we're running in a terminal that supports VT sequences
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
      match term_program.as_str() {
        "WezTerm" | "Alacritty" | "mintty" => return true,
        _ => {}
      }
    }

    // Try to enable virtual terminal processing
    #[cfg(windows)]
    {
      use windows_sys::Win32::Foundation::*;
      use windows_sys::Win32::System::Console::*;

      unsafe {
        let console_handle = GetStdHandle(STD_OUTPUT_HANDLE);
        if console_handle != INVALID_HANDLE_VALUE {
          let mut console_mode = 0;
          if GetConsoleMode(console_handle, &mut console_mode) != 0 {
            // Try to enable virtual terminal processing
            let new_mode = console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING;
            if SetConsoleMode(console_handle, new_mode) != 0 {
              // Reset to original mode for now
              SetConsoleMode(console_handle, console_mode);
              return true;
            }
          }
        }
      }
    }

    false
  }

  /// Detect color support on Windows
  fn detect_color_support() -> bool {
    // Windows Terminal and modern terminals support colors
    if Self::check_virtual_terminal_support() {
      return true;
    }

    // Check if we're in a TTY
    crossterm::tty::IsTty::is_tty(&io::stdout())
  }

  /// Detect maximum color support on Windows
  fn detect_max_colors() -> u32 {
    if !Self::detect_color_support() {
      return 1;
    }

    // Windows Terminal supports truecolor
    if std::env::var("WT_SESSION").is_ok() {
      return 16_777_216; // 24-bit color
    }

    // Check for other terminals with truecolor support
    if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
      match term_program.as_str() {
        "WezTerm" | "Alacritty" => return 16_777_216,
        _ => {}
      }
    }

    // Modern Windows consoles support 256 colors
    if Self::check_virtual_terminal_support() {
      return 256;
    }

    // Legacy console supports 16 colors
    16
  }

  /// Set up Windows console for application mode
  #[cfg(windows)]
  fn setup_windows_console(&mut self) -> Result<()> {
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::System::Console::*;

    unsafe {
      // Get console handles
      let input_handle = GetStdHandle(STD_INPUT_HANDLE);
      let output_handle = GetStdHandle(STD_OUTPUT_HANDLE);

      if input_handle == INVALID_HANDLE_VALUE || output_handle == INVALID_HANDLE_VALUE {
        return Err(TuiError::driver("Failed to get console handles"));
      }

      self.console_handle = Some(output_handle);

      // Save original console mode
      let mut original_mode = 0;
      if GetConsoleMode(input_handle, &mut original_mode) != 0 {
        self.original_console_mode = Some(original_mode);
      }

      // Set input mode for raw input and mouse
      let mut input_mode = ENABLE_WINDOW_INPUT | ENABLE_EXTENDED_FLAGS;
      if self.capabilities.supports_mouse {
        input_mode |= ENABLE_MOUSE_INPUT;
      }

      // Disable line input, echo input, and processed input for raw mode
      input_mode &= !(ENABLE_LINE_INPUT | ENABLE_ECHO_INPUT | ENABLE_PROCESSED_INPUT);

      if SetConsoleMode(input_handle, input_mode) == 0 {
        return Err(TuiError::driver("Failed to set console input mode"));
      }

      // Set output mode for virtual terminal sequences if supported
      let mut output_mode = 0;
      if GetConsoleMode(output_handle, &mut output_mode) != 0 {
        if self.virtual_terminal_supported {
          output_mode |= ENABLE_VIRTUAL_TERMINAL_PROCESSING | ENABLE_PROCESSED_OUTPUT;
        }
        SetConsoleMode(output_handle, output_mode);
      }
    }

    Ok(())
  }

  /// Restore Windows console to original state
  #[cfg(windows)]
  fn cleanup_windows_console(&mut self) -> Result<()> {
    use windows_sys::Win32::Foundation::*;
    use windows_sys::Win32::System::Console::*;

    unsafe {
      let input_handle = GetStdHandle(STD_INPUT_HANDLE);

      if let Some(original_mode) = self.original_console_mode {
        SetConsoleMode(input_handle, original_mode);
      }
    }

    Ok(())
  }

  /// Non-Windows placeholder implementations
  #[cfg(not(windows))]
  fn setup_windows_console(&mut self) -> Result<()> {
    Ok(())
  }

  #[cfg(not(windows))]
  fn cleanup_windows_console(&mut self) -> Result<()> {
    Ok(())
  }

  /// Check if the terminal is currently in raw mode
  fn is_raw_mode_enabled(&self) -> bool {
    // Windows doesn't have a direct equivalent to Unix raw mode
    // crossterm manages this internally, so we'll use a heuristic
    // This is a limitation - we can't reliably detect raw mode on Windows
    false
  }

  /// Set up the terminal for application mode
  fn setup_terminal(&mut self) -> Result<()> {
    // Capture original state before making changes
    self.original_raw_mode = Some(self.is_raw_mode_enabled());
    self.original_cursor_visible = Some(self.cursor_visible);
    // We assume we start NOT in alternate screen (typical case)
    self.original_in_alternate_screen = false;

    // Windows-specific console setup first
    self.setup_windows_console()?;

    {
      let mut stdout = self.stdout.lock().unwrap();

      if !self.config.inline {
        execute!(stdout, EnterAlternateScreen)?;
      }

      // Hide cursor initially
      execute!(stdout, cursor::Hide)?;

      // Clear screen
      execute!(stdout, Clear(ClearType::All))?;

      // Set title if provided
      if !self.current_title.is_empty() && self.virtual_terminal_supported {
        execute!(
          stdout,
          Print(format!("\\x1b]2;{}\\x1b\\\\", self.current_title))
        )?;
      }

      stdout.flush()?;
    }

    // Enable raw mode through crossterm
    terminal::enable_raw_mode()?;
    self.cursor_visible = false;

    // Set up mouse capture if enabled
    if self.capabilities.supports_mouse {
      let mut stdout = self.stdout.lock().unwrap();
      if self.virtual_terminal_supported {
        execute!(
          stdout,
          Print("\\x1b[?1000h"), // Basic mouse reporting
          Print("\\x1b[?1002h"), // Button event tracking
          Print("\\x1b[?1006h"), // SGR extended reporting
        )?;
      }
      stdout.flush()?;
      self.mouse_capture_enabled = true;
    }

    Ok(())
  }

  /// Restore terminal to original state
  fn cleanup_terminal(&mut self) -> Result<()> {
    // Disable mouse capture
    if self.mouse_capture_enabled {
      let mut stdout = self.stdout.lock().unwrap();
      if self.virtual_terminal_supported {
        execute!(
          stdout,
          Print("\\x1b[?1006l"), // Disable SGR extended reporting
          Print("\\x1b[?1002l"), // Disable button event tracking
          Print("\\x1b[?1000l"), // Disable basic mouse reporting
        )?;
      }
      stdout.flush()?;
      self.mouse_capture_enabled = false;
    }

    {
      let mut stdout = self.stdout.lock().unwrap();

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
      terminal::disable_raw_mode()?;
    }

    // Windows-specific cleanup
    self.cleanup_windows_console()?;

    Ok(())
  }

  /// Enable mouse capture (Windows-specific handling)
  #[allow(dead_code)]
  fn enable_mouse_capture(&mut self, stdout: &mut impl Write) -> Result<()> {
    if !self.capabilities.supports_mouse {
      return Ok(());
    }

    if self.virtual_terminal_supported {
      // Use ANSI escape sequences for modern terminals
      execute!(
        stdout,
        Print("\x1b[?1000h"), // Basic mouse reporting
        Print("\x1b[?1002h"), // Button event tracking
        Print("\x1b[?1006h"), // SGR extended reporting
      )?;
    }
    // For legacy console, mouse input is handled by console mode

    self.mouse_capture_enabled = true;
    Ok(())
  }

  /// Disable mouse capture
  #[allow(dead_code)]
  fn disable_mouse_capture(&mut self, stdout: &mut impl Write) -> Result<()> {
    if !self.mouse_capture_enabled {
      return Ok(());
    }

    if self.virtual_terminal_supported {
      execute!(
        stdout,
        Print("\x1b[?1006l"), // Disable SGR extended reporting
        Print("\x1b[?1002l"), // Disable button event tracking
        Print("\x1b[?1000l"), // Disable basic mouse reporting
      )?;
    }

    self.mouse_capture_enabled = false;
    Ok(())
  }

  /// Set terminal title (Windows-specific)
  #[allow(dead_code)]
  fn set_terminal_title(&mut self, stdout: &mut impl Write, title: &str) -> Result<()> {
    if self.virtual_terminal_supported {
      // Use ANSI escape sequence
      execute!(stdout, Print(format!("\x1b]2;{title}\x1b\\")))?;
    } else {
      // Use Windows Console API
      #[cfg(windows)]
      {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::System::Console::SetConsoleTitleW;

        let wide_title: Vec<u16> = OsStr::new(title)
          .encode_wide()
          .chain(std::iter::once(0))
          .collect();

        unsafe {
          SetConsoleTitleW(wide_title.as_ptr());
        }
      }
    }

    Ok(())
  }

  /// Process events in a background thread (Windows-specific)
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
              // Handle Windows-specific key combinations
              if key_event.modifiers.contains(KeyModifiers::CONTROL) {
                match key_event.code {
                  KeyCode::Char('c') => {
                    // Ctrl+C - send quit event
                    if event_sender.send(DriverEvent::Quit).is_err() {
                      break;
                    }
                    continue;
                  }
                  KeyCode::Char('d') => {
                    // Ctrl+D can also be used for quit on Windows
                    if event_sender.send(DriverEvent::Quit).is_err() {
                      break;
                    }
                    continue;
                  }
                  _ => {}
                }
              }

              // Handle Alt+F4 for Windows
              if key_event.modifiers.contains(KeyModifiers::ALT) && key_event.code == KeyCode::F(4)
              {
                if event_sender.send(DriverEvent::Quit).is_err() {
                  break;
                }
                continue;
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
              let custom_event = serde_json::json!({"type": "focus_gained", "platform": "windows"});
              if event_sender
                .send(DriverEvent::Custom("focus".to_string(), custom_event))
                .is_err()
              {
                break;
              }
            }
            Ok(Event::FocusLost) => {
              let custom_event = serde_json::json!({"type": "focus_lost", "platform": "windows"});
              if event_sender
                .send(DriverEvent::Custom("focus".to_string(), custom_event))
                .is_err()
              {
                break;
              }
            }
            Ok(Event::Paste(text)) => {
              let custom_event =
                serde_json::json!({"type": "paste", "text": text, "platform": "windows"});
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

  /// Check if the terminal supports the given capability
  pub fn supports_capability(&self, capability: &str) -> bool {
    match capability {
      "colors" => self.capabilities.supports_colors,
      "mouse" => self.capabilities.supports_mouse,
      "suspend" => false, // Windows doesn't support suspend like Unix
      "alternate_screen" => !self.capabilities.is_inline,
      "title" => true, // Windows always supports title setting
      "virtual_terminal" => self.virtual_terminal_supported,
      "windows_console" => true,
      _ => false,
    }
  }

  /// Get detailed terminal information
  pub fn get_terminal_info(&self) -> serde_json::Value {
    serde_json::json!({
        "type": "windows",
        "colors": self.capabilities.max_colors,
        "supports_mouse": self.capabilities.supports_mouse,
        "can_suspend": false,
        "is_inline": self.capabilities.is_inline,
        "virtual_terminal_supported": self.virtual_terminal_supported,
        "environment": {
            "WT_SESSION": std::env::var("WT_SESSION").ok(),
            "TERM_PROGRAM": std::env::var("TERM_PROGRAM").ok(),
            "PROCESSOR_ARCHITECTURE": std::env::var("PROCESSOR_ARCHITECTURE").unwrap_or_default(),
        }
    })
  }

  /// Get Windows version information
  #[cfg(windows)]
  pub fn get_windows_version(&self) -> String {
    use windows_sys::Win32::System::SystemInformation::*;

    unsafe {
      let mut version_info: OSVERSIONINFOW = std::mem::zeroed();
      version_info.dwOSVersionInfoSize = std::mem::size_of::<OSVERSIONINFOW>() as u32;

      // Note: GetVersionExW is deprecated, but still works for basic version info
      // In production, you might want to use a different method
      format!("Windows (version detection not implemented)")
    }
  }

  #[cfg(not(windows))]
  pub fn get_windows_version(&self) -> String {
    "Not Windows".to_string()
  }
}

impl Driver for WindowsDriver {
  fn start_application_mode(&mut self) -> Result<()> {
    if self.application_mode.load(Ordering::Relaxed) {
      return Err(TuiError::driver("Already in application mode"));
    }

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
    let mut stdout = self.stdout.lock().unwrap();
    stdout.write_all(data.as_bytes())?;
    Ok(())
  }

  fn flush(&mut self) -> Result<()> {
    let mut stdout = self.stdout.lock().unwrap();
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

    let handle = thread::spawn(move || {
      Self::event_loop(event_sender, stop_flag, supports_mouse);
    });

    self.event_thread_handle = Some(handle);
    Ok(())
  }

  fn stop_event_loop(&mut self) -> Result<()> {
    self.stop_flag.store(true, Ordering::Relaxed);

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
    Err(TuiError::driver("Suspend not supported on Windows"))
  }

  fn resume(&mut self) -> Result<()> {
    Err(TuiError::driver("Resume not supported on Windows"))
  }

  fn set_cursor_position(&mut self, x: u16, y: u16) -> Result<()> {
    let mut stdout = self.stdout.lock().unwrap();
    execute!(stdout, cursor::MoveTo(x, y))?;
    Ok(())
  }

  fn set_cursor_visible(&mut self, visible: bool) -> Result<()> {
    let mut stdout = self.stdout.lock().unwrap();

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
    let mut stdout = self.stdout.lock().unwrap();

    if self.virtual_terminal_supported {
      execute!(stdout, Print(format!("\\x1b]2;{title}\\x1b\\\\")))?;
    } else {
      #[cfg(windows)]
      {
        use std::ffi::OsStr;
        use std::os::windows::ffi::OsStrExt;
        use windows_sys::Win32::System::Console::SetConsoleTitleW;

        let wide_title: Vec<u16> = OsStr::new(title)
          .encode_wide()
          .chain(std::iter::once(0))
          .collect();

        unsafe {
          SetConsoleTitleW(wide_title.as_ptr());
        }
      }
    }

    stdout.flush()?;
    Ok(())
  }

  fn set_mouse_capture(&mut self, enabled: bool) -> Result<()> {
    if !self.capabilities.supports_mouse {
      return Ok(());
    }

    let mut stdout = self.stdout.lock().unwrap();

    if enabled && !self.mouse_capture_enabled {
      if self.virtual_terminal_supported {
        execute!(
          stdout,
          Print("\\x1b[?1000h"), // Basic mouse reporting
          Print("\\x1b[?1002h"), // Button event tracking
          Print("\\x1b[?1006h"), // SGR extended reporting
        )?;
      }
      self.mouse_capture_enabled = true;
    } else if !enabled && self.mouse_capture_enabled {
      if self.virtual_terminal_supported {
        execute!(
          stdout,
          Print("\\x1b[?1006l"), // Disable SGR extended reporting
          Print("\\x1b[?1002l"), // Disable button event tracking
          Print("\\x1b[?1000l"), // Disable basic mouse reporting
        )?;
      }
      self.mouse_capture_enabled = false;
    }

    stdout.flush()?;
    Ok(())
  }

  fn clear_screen(&mut self) -> Result<()> {
    let mut stdout = self.stdout.lock().unwrap();
    execute!(stdout, Clear(ClearType::All), cursor::MoveTo(0, 0))?;
    Ok(())
  }

  fn cursor_home(&mut self) -> Result<()> {
    let mut stdout = self.stdout.lock().unwrap();
    execute!(stdout, cursor::MoveTo(0, 0))?;
    Ok(())
  }
}

impl Drop for WindowsDriver {
  fn drop(&mut self) {
    // Ensure terminal is cleaned up
    let _ = self.stop_application_mode();
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_windows_driver_creation() {
    let config = DriverConfig {
      mouse: true,
      title: Some("Test Windows App".to_string()),
      ..Default::default()
    };

    let driver = WindowsDriver::new(config).unwrap();
    assert!(!driver.capabilities.is_headless);
    assert!(!driver.capabilities.can_suspend); // Windows doesn't support suspend
    assert_eq!(driver.current_title, "Test Windows App");
  }

  #[test]
  fn test_virtual_terminal_detection() {
    // Test environment-based VT detection
    std::env::set_var("WT_SESSION", "test-session");
    assert!(WindowsDriver::check_virtual_terminal_support());
    std::env::remove_var("WT_SESSION");

    std::env::set_var("TERM_PROGRAM", "WezTerm");
    assert!(WindowsDriver::check_virtual_terminal_support());
    std::env::remove_var("TERM_PROGRAM");
  }

  #[test]
  fn test_color_detection() {
    // Test Windows Terminal
    std::env::set_var("WT_SESSION", "test");
    assert_eq!(WindowsDriver::detect_max_colors(), 16_777_216);
    std::env::remove_var("WT_SESSION");

    // Test other terminals
    std::env::set_var("TERM_PROGRAM", "Alacritty");
    assert_eq!(WindowsDriver::detect_max_colors(), 16_777_216);
    std::env::remove_var("TERM_PROGRAM");
  }

  #[test]
  fn test_capabilities() {
    let driver = WindowsDriver::new(DriverConfig::default()).unwrap();

    assert!(driver.supports_capability("colors"));
    assert!(driver.supports_capability("mouse"));
    assert!(!driver.supports_capability("suspend")); // Windows doesn't support suspend
    assert!(driver.supports_capability("title"));
    assert!(driver.supports_capability("windows_console"));
    assert!(!driver.supports_capability("unknown"));
  }

  #[test]
  fn test_terminal_info() {
    let driver = WindowsDriver::new(DriverConfig::default()).unwrap();
    let info = driver.get_terminal_info();

    assert_eq!(info["type"], "windows");
    assert!(info["colors"].is_number());
    assert!(info["supports_mouse"].is_boolean());
    assert_eq!(info["can_suspend"], false);
    assert!(info["environment"].is_object());
  }

  #[test]
  fn test_windows_version() {
    let driver = WindowsDriver::new(DriverConfig::default()).unwrap();
    let version = driver.get_windows_version();
    assert!(!version.is_empty());
  }

  // Note: Actual terminal tests would require a Windows console
  #[test]
  #[ignore = "requires Windows console"]
  fn test_application_mode() {
    let _driver = WindowsDriver::new(DriverConfig::default()).unwrap();

    // These will only work on Windows with a console
    #[cfg(windows)]
    {
      driver.start_application_mode().unwrap();
      assert!(driver.application_mode.load(Ordering::Relaxed));

      driver.stop_application_mode().unwrap();
      assert!(!driver.application_mode.load(Ordering::Relaxed));
    }
  }
}
