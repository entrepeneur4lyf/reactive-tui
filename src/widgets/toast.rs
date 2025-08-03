//! Toast notification component with overlay positioning
//!
//! Provides non-intrusive notification messages that appear temporarily
//! and automatically dismiss. Supports different variants (info, success,
//! warning, error) with semantic styling.

use crate::components::Element;
use crate::error::Result;
use crate::events::{Message, MessageEvent};
use crate::layout::LayoutRect;
use crate::widgets::overlay::{OverlayManager, OverlayPosition, OverlayStyle};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Toast notification variants with semantic styling
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ToastVariant {
  /// Informational message (blue theme)
  Info,
  /// Success message (green theme)
  Success,
  /// Warning message (yellow theme)
  Warning,
  /// Error message (red theme)
  Error,
  /// Custom variant with specified CSS classes
  Custom { classes: Vec<String> },
}

impl ToastVariant {
  /// Get CSS classes for this variant
  pub fn css_classes(&self) -> Vec<String> {
    match self {
      ToastVariant::Info => vec![
        "toast-info".to_string(),
        "bg-blue-500".to_string(),
        "text-white".to_string(),
      ],
      ToastVariant::Success => vec![
        "toast-success".to_string(),
        "bg-green-500".to_string(),
        "text-white".to_string(),
      ],
      ToastVariant::Warning => vec![
        "toast-warning".to_string(),
        "bg-yellow-500".to_string(),
        "text-black".to_string(),
      ],
      ToastVariant::Error => vec![
        "toast-error".to_string(),
        "bg-red-500".to_string(),
        "text-white".to_string(),
      ],
      ToastVariant::Custom { classes } => classes.clone(),
    }
  }
}

/// Individual toast notification
#[derive(Debug, Clone)]
pub struct Toast {
  /// Unique identifier for this toast
  pub id: String,
  /// Main message content
  pub message: String,
  /// Optional title
  pub title: Option<String>,
  /// Variant determining styling
  pub variant: ToastVariant,
  /// When this toast was created
  pub created_at: Instant,
  /// How long to show this toast
  pub duration: Duration,
  /// Position on screen
  pub position: OverlayPosition,
  /// Styling configuration
  pub style: OverlayStyle,
}

impl Toast {
  /// Check if this toast should be dismissed based on its duration
  pub fn should_dismiss(&self) -> bool {
    self.created_at.elapsed() >= self.duration
  }

  /// Render this toast as an Element
  pub fn render(&self) -> Element {
    use crate::components::{div, text};

    let mut classes = vec![
      "toast".to_string(),
      "p-2".to_string(),
      "rounded".to_string(),
    ];
    classes.extend(self.variant.css_classes());

    let mut content_div = div().classes(classes).id(&self.id);

    // Add title if present
    if let Some(title) = &self.title {
      content_div = content_div.child(
        div()
          .class("toast-title font-bold mb-1")
          .child(text(title).build())
          .build(),
      );
    }

    // Add message
    content_div = content_div.child(
      div()
        .class("toast-message")
        .child(text(&self.message).build())
        .build(),
    );

    content_div.build()
  }

  /// Get estimated dimensions for this toast
  pub fn estimated_dimensions(&self) -> (u16, u16) {
    // Rough estimation based on content
    let title_lines = self.title.as_ref().map(|_| 1).unwrap_or(0);
    let message_lines = (self.message.len() / 40).max(1) as u16; // Assume ~40 chars per line
    let height = title_lines + message_lines + 2; // +2 for padding
    let width = self.message.len().min(50) as u16 + 4; // +4 for padding
    (width, height)
  }
}

/// Message types for toast system integration
#[derive(Debug, Clone)]
pub struct ShowToastMessage {
  pub toast: Toast,
}

impl Message for ShowToastMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

#[derive(Debug, Clone)]
pub struct DismissToastMessage {
  pub toast_id: String,
}

impl Message for DismissToastMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Manages multiple toast notifications
pub struct ToastManager {
  /// Active toasts by ID
  toasts: HashMap<String, Toast>,
  /// Overlay manager for positioning
  overlay_manager: OverlayManager,
  /// Counter for generating unique IDs
  next_id: u64,
}

impl ToastManager {
  /// Create a new toast manager
  pub fn new(viewport_width: u16, viewport_height: u16) -> Self {
    Self {
      toasts: HashMap::new(),
      overlay_manager: OverlayManager::new(viewport_width, viewport_height),
      next_id: 1,
    }
  }

  /// Update viewport dimensions
  pub fn update_viewport(&mut self, width: u16, height: u16) {
    self.overlay_manager.update_viewport(width, height);
  }

  /// Add a new toast
  pub fn show_toast(&mut self, mut toast: Toast) -> Result<()> {
    // Generate unique ID if not provided
    if toast.id.is_empty() {
      toast.id = format!("{}", self.next_id);
      self.next_id += 1;
    }

    self.toasts.insert(toast.id.clone(), toast);
    Ok(())
  }

  /// Remove a toast by ID
  pub fn dismiss_toast(&mut self, id: &str) -> Result<bool> {
    Ok(self.toasts.remove(id).is_some())
  }

  /// Remove expired toasts
  pub fn cleanup_expired(&mut self) -> Vec<String> {
    let expired_ids: Vec<String> = self
      .toasts
      .iter()
      .filter(|(_, toast)| toast.should_dismiss())
      .map(|(id, _)| id.clone())
      .collect();

    for id in &expired_ids {
      self.toasts.remove(id);
    }

    expired_ids
  }

  /// Get all active toasts
  pub fn active_toasts(&self) -> Vec<&Toast> {
    self.toasts.values().collect()
  }

  /// Calculate positions for all active toasts
  pub fn calculate_positions(&self) -> Result<HashMap<String, LayoutRect>> {
    let overlay_specs: Vec<_> = self
      .toasts
      .values()
      .map(|toast| {
        let (width, height) = toast.estimated_dimensions();
        (width, height, toast.position, &toast.style)
      })
      .collect();

    let positions = self.overlay_manager.stack_overlays(&overlay_specs)?;

    let mut result = HashMap::new();
    for (toast, position) in self.toasts.values().zip(positions.iter()) {
      result.insert(toast.id.clone(), *position);
    }

    Ok(result)
  }

  /// Handle message events
  pub fn handle_message(&mut self, event: &mut MessageEvent) -> Result<()> {
    if let Some(msg) = event.downcast::<ShowToastMessage>() {
      self.show_toast(msg.toast.clone())?;
    } else if let Some(msg) = event.downcast::<DismissToastMessage>() {
      self.dismiss_toast(&msg.toast_id)?;
    }
    Ok(())
  }
}

/// Builder for creating toast notifications
pub struct ToastBuilder {
  message: String,
  title: Option<String>,
  variant: ToastVariant,
  duration: Duration,
  position: OverlayPosition,
  style: OverlayStyle,
}

impl ToastBuilder {
  /// Create a new toast builder with a message
  pub fn new(message: impl Into<String>) -> Self {
    Self {
      message: message.into(),
      title: None,
      variant: ToastVariant::Info,
      duration: Duration::from_secs(5),
      position: OverlayPosition::default(),
      style: OverlayStyle::default(),
    }
  }

  /// Set the title
  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.title = Some(title.into());
    self
  }

  /// Set the variant
  pub fn variant(mut self, variant: ToastVariant) -> Self {
    self.variant = variant;
    self
  }

  /// Set the duration
  pub fn duration(mut self, duration: Duration) -> Self {
    self.duration = duration;
    self
  }

  /// Set the position
  pub fn position(mut self, position: OverlayPosition) -> Self {
    self.position = position;
    self
  }

  /// Set the style
  pub fn style(mut self, style: OverlayStyle) -> Self {
    self.style = style;
    self
  }

  /// Create an info toast
  pub fn info(message: impl Into<String>) -> Self {
    Self::new(message).variant(ToastVariant::Info)
  }

  /// Create a success toast
  pub fn success(message: impl Into<String>) -> Self {
    Self::new(message).variant(ToastVariant::Success)
  }

  /// Create a warning toast
  pub fn warning(message: impl Into<String>) -> Self {
    Self::new(message).variant(ToastVariant::Warning)
  }

  /// Create an error toast
  pub fn error(message: impl Into<String>) -> Self {
    Self::new(message).variant(ToastVariant::Error)
  }

  /// Build the toast
  pub fn build(self) -> Toast {
    Toast {
      id: String::new(), // Will be generated by ToastManager
      message: self.message,
      title: self.title,
      variant: self.variant,
      created_at: Instant::now(),
      duration: self.duration,
      position: self.position,
      style: self.style,
    }
  }

  /// Build and create a show message
  pub fn show(self) -> ShowToastMessage {
    ShowToastMessage {
      toast: self.build(),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_toast_creation() {
    let toast = ToastBuilder::success("Operation completed")
      .title("Success")
      .duration(Duration::from_secs(3))
      .build();

    assert_eq!(toast.message, "Operation completed");
    assert_eq!(toast.title, Some("Success".to_string()));
    assert_eq!(toast.variant, ToastVariant::Success);
    assert_eq!(toast.duration, Duration::from_secs(3));
  }

  #[test]
  fn test_toast_manager() {
    let mut manager = ToastManager::new(100, 50);

    let toast = ToastBuilder::info("Test message").build();
    let toast_id = toast.id.clone();

    // Show toast
    manager.show_toast(toast).unwrap();
    assert_eq!(manager.active_toasts().len(), 1);

    // Dismiss toast
    assert!(manager.dismiss_toast(&toast_id).unwrap());
    assert_eq!(manager.active_toasts().len(), 0);
  }

  #[test]
  fn test_toast_variants() {
    let info_classes = ToastVariant::Info.css_classes();
    assert!(info_classes.contains(&"toast-info".to_string()));
    assert!(info_classes.contains(&"bg-blue-500".to_string()));

    let error_classes = ToastVariant::Error.css_classes();
    assert!(error_classes.contains(&"toast-error".to_string()));
    assert!(error_classes.contains(&"bg-red-500".to_string()));
  }

  #[test]
  fn test_toast_expiration() {
    let toast = Toast {
      id: "test".to_string(),
      message: "Test".to_string(),
      title: None,
      variant: ToastVariant::Info,
      created_at: Instant::now() - Duration::from_secs(10),
      duration: Duration::from_secs(5),
      position: OverlayPosition::default(),
      style: OverlayStyle::default(),
    };

    assert!(toast.should_dismiss());
  }
}
