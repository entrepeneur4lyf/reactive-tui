//! Component system with React-like API

pub mod element;

pub use element::{Element, ElementBuilder};

use crate::error::Result;
use std::collections::HashMap;

/// Component lifecycle state
#[derive(Debug, Clone, PartialEq)]
#[derive(Default)]
pub enum ComponentState {
  #[default]
  Created,
  Mounting,
  Mounted,
  Updating,
  Unmounting,
  Unmounted,
  Error(String),
}


/// Component context provided during lifecycle events
pub struct ComponentContext {
  pub component_id: String,
  pub state: ComponentState,
  pub props: HashMap<String, serde_json::Value>,
  pub reactive_bindings: Vec<String>,
}

impl ComponentContext {
  pub fn new(component_id: String) -> Self {
    Self {
      component_id,
      state: ComponentState::Created,
      props: HashMap::new(),
      reactive_bindings: Vec::new(),
    }
  }

  /// Set a prop value
  pub fn set_prop(&mut self, key: &str, value: serde_json::Value) {
    self.props.insert(key.to_string(), value);
  }

  /// Get a prop value
  pub fn get_prop<T>(&self, key: &str) -> Option<T>
  where
    T: serde::de::DeserializeOwned,
  {
    self.props.get(key)
      .and_then(|v| serde_json::from_value(v.clone()).ok())
  }

  /// Add a reactive binding
  pub fn add_reactive_binding(&mut self, reactive_id: String) {
    if !self.reactive_bindings.contains(&reactive_id) {
      self.reactive_bindings.push(reactive_id);
    }
  }
}

/// Core trait for all UI components
pub trait Component: Send + Sync {
  /// Render the component to an Element tree
  fn render(&self) -> Element;

  /// Component lifecycle: called when component is mounted
  fn on_mount(&mut self, context: &mut ComponentContext) -> Result<()> {
    context.state = ComponentState::Mounted;
    Ok(())
  }

  /// Component lifecycle: called when component is unmounted
  fn on_unmount(&mut self, context: &mut ComponentContext) -> Result<()> {
    context.state = ComponentState::Unmounted;
    Ok(())
  }

  /// Component lifecycle: called when props or state change
  fn on_update(&mut self, context: &mut ComponentContext) -> Result<bool> {
    context.state = ComponentState::Mounted;
    Ok(true) // Return true if re-render is needed
  }

  /// Component lifecycle: called before mounting
  fn on_before_mount(&mut self, context: &mut ComponentContext) -> Result<()> {
    context.state = ComponentState::Mounting;
    Ok(())
  }

  /// Component lifecycle: called after mounting
  fn on_after_mount(&mut self, _context: &mut ComponentContext) -> Result<()> {
    Ok(())
  }

  /// Component lifecycle: called before unmounting
  fn on_before_unmount(&mut self, context: &mut ComponentContext) -> Result<()> {
    context.state = ComponentState::Unmounting;
    Ok(())
  }

  /// Component lifecycle: called when an error occurs
  fn on_error(&mut self, context: &mut ComponentContext, error: &crate::error::TuiError) -> Result<()> {
    context.state = ComponentState::Error(error.to_string());
    Ok(())
  }

  /// Get component's reactive dependencies
  fn get_reactive_dependencies(&self) -> Vec<String> {
    Vec::new()
  }

  /// Check if component should update based on props/state changes
  fn should_update(&self, _old_context: &ComponentContext, _new_context: &ComponentContext) -> bool {
    true
  }
}

/// Props that can be passed to components
#[derive(Debug, Clone, Default)]
pub struct Props {
  pub data: HashMap<String, serde_json::Value>,
}

impl Props {
  pub fn new() -> Self {
    Self::default()
  }

  pub fn set<T: serde::Serialize>(&mut self, key: &str, value: T) -> Result<()> {
    let json_value = serde_json::to_value(value)
      .map_err(|e| crate::error::TuiError::component(format!("Failed to serialize prop: {e}")))?;
    self.data.insert(key.to_string(), json_value);
    Ok(())
  }

  pub fn get<T: serde::de::DeserializeOwned>(&self, key: &str) -> Result<Option<T>> {
    if let Some(value) = self.data.get(key) {
      let deserialized = serde_json::from_value(value.clone()).map_err(|e| {
        crate::error::TuiError::component(format!("Failed to deserialize prop: {e}"))
      })?;
      Ok(Some(deserialized))
    } else {
      Ok(None)
    }
  }
}

/// Convenience functions for creating elements
pub fn div() -> ElementBuilder {
  Element::with_tag("div")
}

pub fn text<S: Into<String>>(content: S) -> ElementBuilder {
  Element::with_tag("text").content(content)
}

pub fn button() -> ElementBuilder {
  Element::with_tag("button")
}

pub fn input() -> ElementBuilder {
  Element::with_tag("input")
}

pub fn span() -> ElementBuilder {
  Element::with_tag("span")
}

pub fn section() -> ElementBuilder {
  Element::with_tag("section")
}

pub fn header() -> ElementBuilder {
  Element::with_tag("header")
}

pub fn footer() -> ElementBuilder {
  Element::with_tag("footer")
}

pub fn main() -> ElementBuilder {
  Element::with_tag("main")
}

// CLI/TUI-specific components

/// Line break/spacing element
pub fn line() -> ElementBuilder {
  Element::with_tag("br")
}

/// Horizontal rule with customizable character and width
pub fn hr() -> ElementBuilder {
  Element::with_tag("hr")
}

/// Alias for hr()
pub fn separator() -> ElementBuilder {
  hr()
}

/// Empty vertical space
pub fn spacer(height: u32) -> ElementBuilder {
  Element::with_tag("spacer").attr("height", height.to_string())
}

/// Code block with optional language syntax highlighting
pub fn code<S: Into<String>>(content: S) -> ElementBuilder {
  Element::with_tag("code").content(content)
}

/// Preformatted text
pub fn pre<S: Into<String>>(content: S) -> ElementBuilder {
  Element::with_tag("pre").content(content)
}

/// Simple bullet/numbered list
pub fn list(items: Vec<String>) -> ElementBuilder {
  let items_json = serde_json::to_string(&items).unwrap_or_default();
  Element::with_tag("list").attr("items", items_json)
}

// Layout and styling helpers

/// Wrap content in a border
pub fn border(child: Element) -> ElementBuilder {
  Element::with_tag("border").child(child)
}

/// Add padding around content
pub fn padding(child: Element, amount: u32) -> ElementBuilder {
  Element::with_tag("padding")
    .attr("amount", amount.to_string())
    .child(child)
}

/// Center-align content
pub fn center(child: Element) -> ElementBuilder {
  Element::with_tag("center").child(child)
}

/// Left-align content
pub fn left(child: Element) -> ElementBuilder {
  Element::with_tag("left").child(child)
}

/// Right-align content
pub fn right(child: Element) -> ElementBuilder {
  Element::with_tag("right").child(child)
}

// Higher-order layout components

/// Container for grouping elements
pub fn container() -> ElementBuilder {
  div().class("container")
}

/// Horizontal flex layout
pub fn flex_row() -> ElementBuilder {
  div().class("flex-row")
}

/// Vertical flex layout
pub fn flex_column() -> ElementBuilder {
  div().class("flex-column")
}

// Widget bridge functions - convert widgets to Elements for responsive layout

/// Create a button widget as an Element
pub fn button_widget(id: &str, text: &str) -> Element {
  use crate::widgets::Button;
  let button = Button::new(id, text);
  button.to_element()
}

/// Create a primary button widget as an Element
pub fn primary_button(id: &str, text: &str) -> Element {
  use crate::widgets::{Button, ButtonType};
  let button = Button::builder(id, text)
    .button_type(ButtonType::Primary)
    .build();
  button.to_element()
}

/// Create a secondary button widget as an Element
pub fn secondary_button(id: &str, text: &str) -> Element {
  use crate::widgets::{Button, ButtonType};
  let button = Button::builder(id, text)
    .button_type(ButtonType::Secondary)
    .build();
  button.to_element()
}

/// Create a danger button widget as an Element
pub fn danger_button(id: &str, text: &str) -> Element {
  use crate::widgets::{Button, ButtonType};
  let button = Button::builder(id, text)
    .button_type(ButtonType::Danger)
    .build();
  button.to_element()
}
