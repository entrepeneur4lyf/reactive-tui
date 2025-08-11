//! # Link Widget
//!
//! Interactive hyperlink component with URL handling, styling, and click events.
//!
//! The Link widget provides a flexible, CSS-styled interactive element for creating
//! clickable links that can open URLs, trigger actions, or navigate within the application.
//! Links integrate seamlessly with the theming system and support focus navigation,
//! keyboard activation, and various visual styles.
//!
//! ## Features
//!
//! - **URL Handling**: Local file paths, web URLs, email addresses, and custom protocols
//! - **Click Actions**: Open in browser, execute commands, trigger application events
//! - **State Management**: Normal, hover, active, focused, visited, disabled states
//! - **CSS Integration**: Full CSS styling support with utility classes
//! - **Accessibility**: Keyboard navigation and screen reader support
//! - **Customization**: Icons, underlines, colors, and custom styling
//! - **Security**: URL validation and safe opening mechanisms
//!
//! ## Examples
//!
//! ### Basic Web Link
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let web_link = link("docs-link", |builder| {
//!     builder
//!         .text("Documentation")
//!         .url("https://docs.rs/reactive-tui")
//!         .target(LinkTarget::Browser)
//! });
//! ```
//!
//! ### File Link with Icon
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let file_link = link("config-link", |builder| {
//!     builder
//!         .text("📄 Config File")
//!         .url("file://./config.toml")
//!         .target(LinkTarget::Application)
//!         .on_click("open_config")
//! });
//! ```
//!
//! ### Email Link
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let email_link = email_link("contact", "support@example.com", "Contact Support");
//! ```

use crate::components::Element;
use crate::error::{Result, TuiError};
use crate::layout::LayoutRect;
use crate::themes::ColorTheme;
use crate::widgets::ResponsiveWidget;

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Link target types for different opening behaviors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkTarget {
  /// Open in system default browser
  Browser,
  /// Handle within the application
  Application,
  /// Execute as system command
  Command,
  /// Open in default application for file type
  System,
  /// Custom handler (requires callback)
  Custom,
}

/// Link state for visual styling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkState {
  /// Normal unvisited link
  Normal,
  /// Mouse hovering over link
  Hover,
  /// Link currently being clicked
  Active,
  /// Link has keyboard focus
  Focused,
  /// Link has been visited before
  Visited,
  /// Link is disabled and non-interactive
  Disabled,
}

/// Link decoration styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LinkDecoration {
  /// No decoration
  None,
  /// Underline the link text
  Underline,
  /// Dotted underline
  Dotted,
  /// Dashed underline
  Dashed,
  /// Double underline
  Double,
  /// Overline decoration
  Overline,
  /// Strike through
  StrikeThrough,
}

/// Link widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LinkConfig {
  /// Link text to display
  pub text: String,
  /// Target URL or path
  pub url: String,
  /// How to handle link activation
  pub target: LinkTarget,
  /// Link decoration style
  pub decoration: LinkDecoration,
  /// Alternative text for accessibility
  pub alt_text: Option<String>,
  /// Tooltip text on hover
  pub tooltip: Option<String>,
  /// Whether link should be disabled
  pub disabled: bool,
  /// Custom CSS classes
  pub classes: Vec<String>,
  /// Custom attributes
  pub attributes: HashMap<String, String>,
  /// Callback event name for application handling
  pub on_click: Option<String>,
  /// Whether to show external link indicator
  pub show_external_indicator: bool,
  /// Whether link has been visited
  pub visited: bool,
}

impl Default for LinkConfig {
  fn default() -> Self {
    Self {
      text: String::new(),
      url: String::new(),
      target: LinkTarget::Browser,
      decoration: LinkDecoration::Underline,
      alt_text: None,
      tooltip: None,
      disabled: false,
      classes: Vec::new(),
      attributes: HashMap::new(),
      on_click: None,
      show_external_indicator: true,
      visited: false,
    }
  }
}

/// Core Link widget implementation
#[derive(Debug)]
pub struct LinkWidget {
  /// Widget identifier
  id: String,
  /// Link configuration
  config: LinkConfig,
  /// Current widget state
  state: LinkState,
  /// Widget bounds for coordinate mapping
  bounds: Option<LayoutRect>,
}

impl LinkWidget {
  /// Create a new Link widget with configuration
  pub fn new(id: impl Into<String>, config: LinkConfig) -> Self {
    let state = if config.disabled {
      LinkState::Disabled
    } else if config.visited {
      LinkState::Visited
    } else {
      LinkState::Normal
    };

    Self {
      id: id.into(),
      config,
      state,
      bounds: None,
    }
  }

  /// Get the widget's identifier
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Get current link state
  pub fn state(&self) -> LinkState {
    self.state
  }

  /// Set link state
  pub fn set_state(&mut self, state: LinkState) {
    if !self.config.disabled {
      self.state = state;
    }
  }

  /// Check if the URL is external (web URL)
  pub fn is_external_url(&self) -> bool {
    self.config.url.starts_with("http://")
      || self.config.url.starts_with("https://")
      || self.config.url.starts_with("ftp://")
  }

  /// Check if the URL is an email address
  pub fn is_email(&self) -> bool {
    self.config.url.starts_with("mailto:")
      || (self.config.url.contains('@') && !self.config.url.contains('/'))
  }

  /// Check if the URL is a file path
  pub fn is_file_path(&self) -> bool {
    self.config.url.starts_with("file://")
      || self.config.url.starts_with("./")
      || self.config.url.starts_with("../")
      || self.config.url.starts_with('/')
      || (cfg!(windows) && self.config.url.len() > 2 && self.config.url.chars().nth(1) == Some(':'))
  }

  /// Validate the URL format
  pub fn validate_url(&self) -> Result<()> {
    if self.config.url.is_empty() {
      return Err(TuiError::component("Link URL cannot be empty"));
    }

    // Basic URL validation
    if self.is_external_url() {
      // Web URL validation
      if !self.config.url.contains('.') {
        return Err(TuiError::component("Invalid web URL format"));
      }
    } else if self.is_email() {
      // Email validation
      let email = if self.config.url.starts_with("mailto:") {
        &self.config.url[7..]
      } else {
        &self.config.url
      };

      if !email.contains('@') || email.starts_with('@') || email.ends_with('@') {
        return Err(TuiError::component("Invalid email address format"));
      }
    }
    // File paths are generally more flexible, so less strict validation

    Ok(())
  }

  /// Handle link activation (click event)
  pub fn activate(&mut self) -> Result<()> {
    if self.config.disabled {
      return Ok(());
    }

    self.validate_url()?;

    // Set visited state
    if !self.config.visited {
      self.config.visited = true;
      if self.state == LinkState::Normal {
        self.state = LinkState::Visited;
      }
    }

    match self.config.target {
      LinkTarget::Browser => self.open_in_browser(),
      LinkTarget::System => self.open_with_system(),
      LinkTarget::Command => self.execute_command(),
      LinkTarget::Application | LinkTarget::Custom => {
        // These should be handled by the application
        Ok(())
      }
    }
  }

  /// Open URL in system default browser
  fn open_in_browser(&self) -> Result<()> {
    let url = if self.is_email() && !self.config.url.starts_with("mailto:") {
      format!("mailto:{}", self.config.url)
    } else {
      self.config.url.clone()
    };

    #[cfg(target_os = "windows")]
    {
      std::process::Command::new("cmd")
        .args(["/c", "start", "", &url])
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open URL: {e}")))?;
    }

    #[cfg(target_os = "macos")]
    {
      std::process::Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open URL: {e}")))?;
    }

    #[cfg(target_os = "linux")]
    {
      std::process::Command::new("xdg-open")
        .arg(&url)
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open URL: {e}")))?;
    }

    Ok(())
  }

  /// Open with system default application
  fn open_with_system(&self) -> Result<()> {
    let path = if self.config.url.starts_with("file://") {
      &self.config.url[7..]
    } else {
      &self.config.url
    };

    #[cfg(target_os = "windows")]
    {
      std::process::Command::new("cmd")
        .args(["/c", "start", "", path])
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open file: {e}")))?;
    }

    #[cfg(target_os = "macos")]
    {
      std::process::Command::new("open")
        .arg(path)
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open file: {e}")))?;
    }

    #[cfg(target_os = "linux")]
    {
      std::process::Command::new("xdg-open")
        .arg(path)
        .spawn()
        .map_err(|e| TuiError::component(format!("Failed to open file: {e}")))?;
    }

    Ok(())
  }

  /// Execute URL as system command
  fn execute_command(&self) -> Result<()> {
    // Security: Only allow if explicitly configured as command target
    if self.config.target != LinkTarget::Command {
      return Err(TuiError::component(
        "Command execution not allowed for this link",
      ));
    }

    let parts: Vec<&str> = self.config.url.split_whitespace().collect();
    if parts.is_empty() {
      return Err(TuiError::component("Empty command"));
    }

    let mut cmd = std::process::Command::new(parts[0]);
    if parts.len() > 1 {
      cmd.args(&parts[1..]);
    }

    cmd
      .spawn()
      .map_err(|e| TuiError::component(format!("Failed to execute command: {e}")))?;

    Ok(())
  }

  /// Update widget bounds for coordinate mapping
  pub fn set_bounds(&mut self, bounds: LayoutRect) {
    self.bounds = Some(bounds);
  }

  /// Get current widget bounds
  pub fn bounds(&self) -> Option<LayoutRect> {
    self.bounds
  }

  /// Get display text with external indicator if needed
  fn get_display_text(&self) -> String {
    let mut text = self.config.text.clone();

    if self.config.show_external_indicator && self.is_external_url() {
      text.push_str(" ↗");
    }

    text
  }

  /// Get CSS classes based on current state
  fn get_css_classes(&self) -> Vec<String> {
    let mut classes = vec![
      "link".to_string(),
      format!(
        "link-{}",
        match self.state {
          LinkState::Normal => "normal",
          LinkState::Hover => "hover",
          LinkState::Active => "active",
          LinkState::Focused => "focused",
          LinkState::Visited => "visited",
          LinkState::Disabled => "disabled",
        }
      ),
      format!(
        "link-decoration-{}",
        match self.config.decoration {
          LinkDecoration::None => "none",
          LinkDecoration::Underline => "underline",
          LinkDecoration::Dotted => "dotted",
          LinkDecoration::Dashed => "dashed",
          LinkDecoration::Double => "double",
          LinkDecoration::Overline => "overline",
          LinkDecoration::StrikeThrough => "strike",
        }
      ),
    ];

    if self.is_external_url() {
      classes.push("link-external".to_string());
    } else if self.is_email() {
      classes.push("link-email".to_string());
    } else if self.is_file_path() {
      classes.push("link-file".to_string());
    }

    classes.extend(self.config.classes.clone());
    classes
  }
}

impl ResponsiveWidget for LinkWidget {
  fn to_element(&self) -> Element {
    let mut builder = Element::with_tag("a")
      .id(&self.id)
      .classes(self.get_css_classes())
      .attr("href", &self.config.url)
      .attr("target", format!("{:?}", self.config.target).to_lowercase())
      .content(self.get_display_text())
      .focusable(!self.config.disabled);

    if let Some(alt_text) = &self.config.alt_text {
      builder = builder.attr("alt", alt_text).attr("aria-label", alt_text);
    }

    if let Some(tooltip) = &self.config.tooltip {
      builder = builder.attr("title", tooltip);
    }

    if let Some(on_click) = &self.config.on_click {
      builder = builder.attr("onclick", on_click);
    }

    // Add custom attributes
    for (key, value) in &self.config.attributes {
      builder = builder.attr(key, value);
    }

    builder.build()
  }

  fn render_with_layout(&self, layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    let text = self.get_display_text();
    let available_width = layout.width as usize;

    // Apply decoration based on link decoration setting
    let decorated_text = match self.config.decoration {
      LinkDecoration::None => text,
      LinkDecoration::Underline => format!("\x1b[4m{text}\x1b[24m"),
      LinkDecoration::Dotted => format!("\x1b[4:3m{text}\x1b[24m"),
      LinkDecoration::Dashed => format!("\x1b[4:2m{text}\x1b[24m"),
      LinkDecoration::Double => format!("\x1b[4:2m{text}\x1b[24m"),
      LinkDecoration::Overline => format!("\x1b[53m{text}\x1b[55m"),
      LinkDecoration::StrikeThrough => format!("\x1b[9m{text}\x1b[29m"),
    };

    // Apply state-based styling
    let styled_text = match self.state {
      LinkState::Normal => decorated_text,
      LinkState::Hover => format!("\x1b[1m{decorated_text}\x1b[22m"), // Bold
      LinkState::Active => format!("\x1b[2m{decorated_text}\x1b[22m"), // Dim
      LinkState::Focused => format!("\x1b[7m{decorated_text}\x1b[27m"), // Reverse
      LinkState::Visited => format!("\x1b[2m{decorated_text}\x1b[22m"), // Dim
      LinkState::Disabled => format!("\x1b[2;37m{decorated_text}\x1b[22;39m"), // Dim gray
    };

    // Truncate if needed
    if styled_text.len() > available_width {
      format!("{}…", &styled_text[..available_width.saturating_sub(1)])
    } else {
      styled_text
    }
  }

  fn min_size(&self) -> (u16, u16) {
    let text_len = self.get_display_text().len() as u16;
    (text_len.max(4), 1)
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    let text_len = self.get_display_text().len() as u16;
    (Some(text_len + 10), Some(1))
  }

  fn can_grow_horizontal(&self) -> bool {
    false
  }

  fn can_grow_vertical(&self) -> bool {
    false
  }
}

/// Link widget builder for fluent configuration
pub struct LinkBuilder {
  id: String,
  config: LinkConfig,
}

impl LinkBuilder {
  /// Create new Link widget builder
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      config: LinkConfig::default(),
    }
  }

  /// Set link text
  pub fn text(mut self, text: impl Into<String>) -> Self {
    self.config.text = text.into();
    self
  }

  /// Set target URL
  pub fn url(mut self, url: impl Into<String>) -> Self {
    self.config.url = url.into();
    self
  }

  /// Set link target behavior
  pub fn target(mut self, target: LinkTarget) -> Self {
    self.config.target = target;
    self
  }

  /// Set link decoration style
  pub fn decoration(mut self, decoration: LinkDecoration) -> Self {
    self.config.decoration = decoration;
    self
  }

  /// Set alternative text for accessibility
  pub fn alt_text(mut self, alt_text: impl Into<String>) -> Self {
    self.config.alt_text = Some(alt_text.into());
    self
  }

  /// Set tooltip text
  pub fn tooltip(mut self, tooltip: impl Into<String>) -> Self {
    self.config.tooltip = Some(tooltip.into());
    self
  }

  /// Set disabled state
  pub fn disabled(mut self, disabled: bool) -> Self {
    self.config.disabled = disabled;
    self
  }

  /// Add CSS class
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.config.classes.push(class.into());
    self
  }

  /// Add custom attribute
  pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
    self.config.attributes.insert(key.into(), value.into());
    self
  }

  /// Set click event callback
  pub fn on_click(mut self, callback: impl Into<String>) -> Self {
    self.config.on_click = Some(callback.into());
    self
  }

  /// Set whether to show external link indicator
  pub fn show_external_indicator(mut self, show: bool) -> Self {
    self.config.show_external_indicator = show;
    self
  }

  /// Set visited state
  pub fn visited(mut self, visited: bool) -> Self {
    self.config.visited = visited;
    self
  }

  /// Build the Link widget
  pub fn build(self) -> LinkWidget {
    LinkWidget::new(self.id, self.config)
  }
}

//
// Factory Functions - Following framework patterns
//

/// Primary link factory function with configuration callback
pub fn link<F>(id: &str, config: F) -> Element
where
  F: FnOnce(LinkBuilder) -> LinkBuilder,
{
  let builder = LinkBuilder::new(id);
  let configured_builder = config(builder);
  let widget = configured_builder.build();
  widget.to_element()
}

/// Convenience function for web links
pub fn web_link(id: &str, url: &str, text: &str) -> Element {
  link(id, |builder| {
    builder.text(text).url(url).target(LinkTarget::Browser)
  })
}

/// Convenience function for email links
pub fn email_link(id: &str, email: &str, text: &str) -> Element {
  let mailto_url = if email.starts_with("mailto:") {
    email.to_string()
  } else {
    format!("mailto:{email}")
  };

  link(id, |builder| {
    builder
      .text(text)
      .url(mailto_url)
      .target(LinkTarget::Browser)
      .class("link-email")
  })
}

/// Convenience function for file links
pub fn file_link(id: &str, path: &str, text: &str) -> Element {
  link(id, |builder| {
    builder
      .text(text)
      .url(path)
      .target(LinkTarget::System)
      .class("link-file")
  })
}

/// Convenience function for application internal links
pub fn app_link(id: &str, action: &str, text: &str) -> Element {
  link(id, |builder| {
    builder
      .text(text)
      .url(action)
      .target(LinkTarget::Application)
      .on_click(action)
      .show_external_indicator(false)
  })
}

/// Convenience function for command execution links
pub fn command_link(id: &str, command: &str, text: &str) -> Element {
  link(id, |builder| {
    builder
      .text(text)
      .url(command)
      .target(LinkTarget::Command)
      .class("link-command")
      .show_external_indicator(false)
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_link_config_default() {
    let config = LinkConfig::default();
    assert_eq!(config.text, "");
    assert_eq!(config.url, "");
    assert_eq!(config.target, LinkTarget::Browser);
    assert_eq!(config.decoration, LinkDecoration::Underline);
    assert!(!config.disabled);
    assert!(!config.visited);
  }

  #[test]
  fn test_link_builder() {
    let widget = LinkBuilder::new("test-link")
      .text("Test Link")
      .url("https://example.com")
      .target(LinkTarget::Browser)
      .decoration(LinkDecoration::None)
      .alt_text("Test link for example")
      .build();

    assert_eq!(widget.id(), "test-link");
    assert_eq!(widget.config.text, "Test Link");
    assert_eq!(widget.config.url, "https://example.com");
    assert_eq!(widget.config.target, LinkTarget::Browser);
    assert_eq!(widget.config.decoration, LinkDecoration::None);
  }

  #[test]
  fn test_url_type_detection() {
    let web_link = LinkWidget::new(
      "web",
      LinkConfig {
        url: "https://example.com".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(web_link.is_external_url());
    assert!(!web_link.is_email());
    assert!(!web_link.is_file_path());

    let email_link = LinkWidget::new(
      "email",
      LinkConfig {
        url: "user@example.com".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(!email_link.is_external_url());
    assert!(email_link.is_email());
    assert!(!email_link.is_file_path());

    let file_link = LinkWidget::new(
      "file",
      LinkConfig {
        url: "./config.toml".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(!file_link.is_external_url());
    assert!(!file_link.is_email());
    assert!(file_link.is_file_path());
  }

  #[test]
  fn test_url_validation() {
    let valid_web = LinkWidget::new(
      "web",
      LinkConfig {
        url: "https://example.com".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(valid_web.validate_url().is_ok());

    let valid_email = LinkWidget::new(
      "email",
      LinkConfig {
        url: "test@example.com".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(valid_email.validate_url().is_ok());

    let invalid_empty = LinkWidget::new(
      "empty",
      LinkConfig {
        url: "".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(invalid_empty.validate_url().is_err());

    let invalid_email = LinkWidget::new(
      "bad-email",
      LinkConfig {
        url: "@invalid".to_string(),
        ..LinkConfig::default()
      },
    );
    assert!(invalid_email.validate_url().is_err());
  }

  #[test]
  fn test_link_state_management() {
    let mut widget = LinkWidget::new("test", LinkConfig::default());
    assert_eq!(widget.state(), LinkState::Normal);

    widget.set_state(LinkState::Hover);
    assert_eq!(widget.state(), LinkState::Hover);

    // Disabled links should not change state
    let mut disabled_widget = LinkWidget::new(
      "disabled",
      LinkConfig {
        disabled: true,
        ..LinkConfig::default()
      },
    );
    assert_eq!(disabled_widget.state(), LinkState::Disabled);

    disabled_widget.set_state(LinkState::Hover);
    assert_eq!(disabled_widget.state(), LinkState::Disabled);
  }

  #[test]
  fn test_display_text_with_indicator() {
    let external_link = LinkWidget::new(
      "external",
      LinkConfig {
        text: "Example".to_string(),
        url: "https://example.com".to_string(),
        show_external_indicator: true,
        ..LinkConfig::default()
      },
    );
    assert_eq!(external_link.get_display_text(), "Example ↗");

    let internal_link = LinkWidget::new(
      "internal",
      LinkConfig {
        text: "Internal".to_string(),
        url: "./page.html".to_string(),
        show_external_indicator: true,
        ..LinkConfig::default()
      },
    );
    assert_eq!(internal_link.get_display_text(), "Internal");
  }

  #[test]
  fn test_factory_functions() {
    let web = web_link("web", "https://example.com", "Example");
    assert_eq!(web.id, Some("web".to_string()));
    assert_eq!(web.tag, "a");

    let email = email_link("email", "test@example.com", "Contact");
    assert_eq!(
      email.attributes.get("href"),
      Some(&"mailto:test@example.com".to_string())
    );

    let file = file_link("file", "./README.md", "Read Me");
    assert_eq!(
      file.attributes.get("href"),
      Some(&"./README.md".to_string())
    );
  }
}
