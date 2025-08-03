//! Cross-platform driver architecture for terminal handling
//!
//! Provides an abstraction layer over different terminal platforms,
//! to support modern TUI application driver system.

use crate::error::Result;
use tokio::sync::mpsc;

pub use crossterm::event::{
  KeyCode, KeyEvent, KeyEventKind, KeyEventState, KeyModifiers, MediaKeyCode, ModifierKeyCode,
  MouseButton, MouseEvent, MouseEventKind,
};

pub mod headless;

#[cfg(unix)]
pub mod unix;

#[cfg(windows)]
pub mod windows;

/// Events that can be sent from drivers to the application
#[derive(Debug, Clone)]
pub enum DriverEvent {
  /// Key press event
  Key(KeyEvent),
  /// Mouse event
  Mouse(MouseEvent),
  /// Terminal resize event
  Resize(u16, u16),
  /// Application should quit
  Quit,
  /// Custom driver-specific event
  Custom(String, serde_json::Value),
}

/// Driver capabilities and properties
#[derive(Debug, Clone)]
pub struct DriverCapabilities {
  /// Can the driver be suspended (Ctrl+Z support)
  pub can_suspend: bool,
  /// Is this a headless driver (no output)
  pub is_headless: bool,
  /// Is this an inline driver (not fullscreen)
  pub is_inline: bool,
  /// Is this a web-based driver
  pub is_web: bool,
  /// Does the driver support mouse input
  pub supports_mouse: bool,
  /// Does the driver support colors
  pub supports_colors: bool,
  /// Maximum number of colors supported
  pub max_colors: u32,
}

impl Default for DriverCapabilities {
  fn default() -> Self {
    Self {
      can_suspend: false,
      is_headless: false,
      is_inline: false,
      is_web: false,
      supports_mouse: true,
      supports_colors: true,
      max_colors: 16_777_216, // 24-bit color
    }
  }
}

/// Abstract driver trait for different terminal platforms
pub trait Driver: Send + Sync {
  /// Start application mode (fullscreen, raw input, etc.)
  fn start_application_mode(&mut self) -> Result<()>;

  /// Stop application mode and restore terminal
  fn stop_application_mode(&mut self) -> Result<()>;

  /// Write data to the terminal output
  fn write(&mut self, data: &str) -> Result<()>;

  /// Flush any buffered output
  fn flush(&mut self) -> Result<()>;

  /// Get current terminal size in columns and rows
  fn get_terminal_size(&self) -> Result<(u16, u16)>;

  /// Get driver capabilities
  fn capabilities(&self) -> &DriverCapabilities;

  /// Start the event loop, sending events through the channel
  fn start_event_loop(&mut self, event_sender: mpsc::UnboundedSender<DriverEvent>) -> Result<()>;

  /// Stop the event loop
  fn stop_event_loop(&mut self) -> Result<()>;

  /// Suspend the application (Ctrl+Z handling)
  fn suspend(&mut self) -> Result<()> {
    if self.capabilities().can_suspend {
      self.stop_application_mode()?;
    }
    Ok(())
  }

  /// Resume the application after suspension
  fn resume(&mut self) -> Result<()> {
    if self.capabilities().can_suspend {
      self.start_application_mode()?;
    }
    Ok(())
  }

  /// Set cursor position (if supported)
  fn set_cursor_position(&mut self, x: u16, y: u16) -> Result<()> {
    let _ = (x, y);
    Ok(()) // Default implementation does nothing
  }

  /// Show or hide cursor
  fn set_cursor_visible(&mut self, visible: bool) -> Result<()> {
    let _ = visible;
    Ok(()) // Default implementation does nothing
  }

  /// Set the terminal title (if supported)
  fn set_title(&mut self, title: &str) -> Result<()> {
    let _ = title;
    Ok(()) // Default implementation does nothing
  }

  /// Enable or disable mouse capture
  fn set_mouse_capture(&mut self, enabled: bool) -> Result<()> {
    let _ = enabled;
    Ok(()) // Default implementation does nothing
  }

  /// Clear the terminal screen
  fn clear_screen(&mut self) -> Result<()> {
    self.write("\x1b[2J\x1b[H")?;
    self.flush()
  }

  /// Move cursor to home position (0,0)
  fn cursor_home(&mut self) -> Result<()> {
    self.write("\x1b[H")?;
    self.flush()
  }

  /// Write raw bytes directly to output (for frame buffer support)
  fn write_bytes(&mut self, data: &[u8]) -> Result<()> {
    // Default implementation converts bytes to string
    let data_str = String::from_utf8_lossy(data);
    self.write(&data_str)
  }
}

/// Driver manager that selects and manages the appropriate driver
pub struct DriverManager {
  driver: Box<dyn Driver>,
  event_receiver: Option<mpsc::UnboundedReceiver<DriverEvent>>,
  _capabilities: DriverCapabilities,
}

impl DriverManager {
  /// Create a new driver manager with automatic platform detection
  pub fn new() -> Result<Self> {
    Self::with_config(DriverConfig::default())
  }

  /// Create a driver manager with specific configuration
  pub fn with_config(config: DriverConfig) -> Result<Self> {
    let driver = Self::create_driver(config)?;
    let capabilities = driver.capabilities().clone();

    Ok(Self {
      driver,
      event_receiver: None,
      _capabilities: capabilities,
    })
  }

  /// Create a driver for testing purposes
  pub fn headless() -> Result<Self> {
    let config = DriverConfig {
      driver_type: Some(DriverType::Headless),
      ..Default::default()
    };
    Self::with_config(config)
  }

  /// Start the driver and begin event processing
  pub fn start(&mut self) -> Result<mpsc::UnboundedReceiver<DriverEvent>> {
    self.driver.start_application_mode()?;

    let (sender, receiver) = mpsc::unbounded_channel();
    self.driver.start_event_loop(sender)?;
    self.event_receiver = Some(receiver);

    Ok(self.event_receiver.take().unwrap())
  }

  /// Stop the driver and clean up
  pub fn stop(&mut self) -> Result<()> {
    self.driver.stop_event_loop()?;
    self.driver.stop_application_mode()?;
    Ok(())
  }

  /// Get mutable reference to the underlying driver
  pub fn driver_mut(&mut self) -> &mut dyn Driver {
    &mut *self.driver
  }

  /// Get reference to the underlying driver
  pub fn driver(&self) -> &dyn Driver {
    &*self.driver
  }

  /// Create the appropriate driver based on platform and config
  fn create_driver(config: DriverConfig) -> Result<Box<dyn Driver>> {
    match config.driver_type.unwrap_or_else(Self::detect_platform) {
      DriverType::Headless => Ok(Box::new(headless::HeadlessDriver::new(config)?)),
      #[cfg(unix)]
      DriverType::Unix => Ok(Box::new(unix::UnixDriver::new(config)?)),
      #[cfg(not(unix))]
      DriverType::Unix => Err(crate::error::TuiError::driver(
        "Unix driver not available on this platform",
      )),
      #[cfg(windows)]
      DriverType::Windows => Ok(Box::new(windows::WindowsDriver::new(config)?)),
      #[cfg(not(windows))]
      DriverType::Windows => Err(crate::error::TuiError::driver(
        "Windows driver not available on this platform",
      )),
    }
  }

  /// Automatically detect the best driver for the current platform
  fn detect_platform() -> DriverType {
    #[cfg(target_os = "windows")]
    return DriverType::Windows;

    #[cfg(any(
      target_os = "linux",
      target_os = "macos",
      target_os = "freebsd",
      target_os = "openbsd",
      target_os = "netbsd"
    ))]
    return DriverType::Unix;

    #[cfg(not(any(
      target_os = "windows",
      target_os = "linux",
      target_os = "macos",
      target_os = "freebsd",
      target_os = "openbsd",
      target_os = "netbsd"
    )))]
    return DriverType::Headless;
  }

  /// Set the terminal window title
  pub fn set_title(&mut self, title: &str) {
    let _ = self.driver.set_title(title);
  }
}

impl Default for DriverManager {
  fn default() -> Self {
    Self::new().expect("Failed to create default driver manager")
  }
}

impl Drop for DriverManager {
  fn drop(&mut self) {
    let _ = self.stop();
  }
}

/// Driver type selection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverType {
  /// Cross-platform headless driver for testing
  Headless,
  /// Unix-like systems (Linux, macOS, BSD)
  Unix,
  /// Windows systems
  Windows,
}

/// Configuration for driver creation
#[derive(Debug, Clone)]
pub struct DriverConfig {
  /// Specific driver type to use (None for auto-detect)
  pub driver_type: Option<DriverType>,
  /// Enable debug mode
  pub debug: bool,
  /// Enable mouse support
  pub mouse: bool,
  /// Initial terminal size (None for auto-detect)
  pub size: Option<(u16, u16)>,
  /// Enable inline mode (non-fullscreen)
  pub inline: bool,
  /// Custom title for the terminal
  pub title: Option<String>,
}

impl Default for DriverConfig {
  fn default() -> Self {
    Self {
      driver_type: None,
      debug: false,
      mouse: true,
      size: None,
      inline: false,
      title: None,
    }
  }
}

/// Utilities for driver implementations
pub mod utils {
  use super::*;
  use crate::compat::terminal;

  /// Get terminal size using crossterm
  pub fn get_terminal_size() -> Result<(u16, u16)> {
    let (cols, rows) = terminal::size()?;
    Ok((cols, rows))
  }

  /// Check if stdout is a TTY
  pub fn is_tty() -> bool {
    crate::compat::IsTty::is_tty(&std::io::stdout())
  }

  /// Check if the terminal supports colors
  pub fn supports_colors() -> bool {
    // Check environment variables for color support
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

    // Default to supporting colors if we can't determine
    is_tty()
  }

  /// Get the number of colors supported by the terminal
  pub fn get_color_count() -> u32 {
    if let Ok(colorterm) = std::env::var("COLORTERM") {
      if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return 16_777_216; // 24-bit color
      }
    }

    if let Ok(term) = std::env::var("TERM") {
      if term.contains("256") {
        return 256;
      } else if term.contains("color") {
        return 16;
      }
    }

    // Conservative default
    if supports_colors() {
      16
    } else {
      1
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  // use tokio::time::{timeout, Duration};

  #[test]
  fn test_driver_config_default() {
    let config = DriverConfig::default();
    assert!(config.driver_type.is_none());
    assert!(!config.debug);
    assert!(config.mouse);
    assert!(config.size.is_none());
    assert!(!config.inline);
    assert!(config.title.is_none());
  }

  #[test]
  fn test_driver_capabilities_default() {
    let caps = DriverCapabilities::default();
    assert!(!caps.can_suspend);
    assert!(!caps.is_headless);
    assert!(!caps.is_inline);
    assert!(!caps.is_web);
    assert!(caps.supports_mouse);
    assert!(caps.supports_colors);
    assert_eq!(caps.max_colors, 16_777_216);
  }

  #[test]
  fn test_platform_detection() {
    let driver_type = DriverManager::detect_platform();

    #[cfg(target_os = "windows")]
    assert_eq!(driver_type, DriverType::Windows);

    #[cfg(any(target_os = "linux", target_os = "macos"))]
    assert_eq!(driver_type, DriverType::Unix);
  }

  #[tokio::test]
  async fn test_headless_driver_creation() {
    let manager = DriverManager::headless();
    assert!(manager.is_ok());

    let manager = manager.unwrap();
    assert!(manager.driver().capabilities().is_headless);
  }

  #[test]
  fn test_utils_terminal_size() {
    // This test might fail in some CI environments
    match utils::get_terminal_size() {
      Ok((cols, rows)) => {
        assert!(cols > 0);
        assert!(rows > 0);
      }
      Err(_) => {
        // Terminal size detection failed, which is acceptable in some environments
      }
    }
  }

  #[test]
  fn test_utils_color_support() {
    // These should not panic
    let _supports_colors = utils::supports_colors();
    let _color_count = utils::get_color_count();
    let _is_tty = utils::is_tty();
  }
}
