//! Core Element type representing the virtual DOM

use crate::events::{ElementAction, KeyCombination};
use std::collections::HashMap;

#[cfg(feature = "typescript")]
use ts_rs::TS;

/// Component trait for building reusable UI components
pub trait Component: Send + Sync {
  fn render(&self) -> Element;
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct Element {
  pub tag: String,
  pub classes: Vec<String>,
  pub attributes: HashMap<String, String>,
  pub content: Option<String>,
  pub children: Vec<Element>,
  pub id: Option<String>,
  pub focusable: bool,
  pub focused: bool,
  pub disabled: bool,
  pub tab_index: Option<i32>,
  pub key_bindings: Vec<ElementKeyBinding>,
  pub modal: bool, // True if this is a modal (gets ESC dismiss by default)
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct ElementKeyBinding {
  pub key: KeyCombination,
  pub action: ElementAction,
}

impl Element {
  pub fn with_tag<S: Into<String>>(tag: S) -> ElementBuilder {
    ElementBuilder {
      element: Element {
        tag: tag.into(),
        classes: Vec::new(),
        attributes: HashMap::new(),
        content: None,
        children: Vec::new(),
        id: None,
        focusable: false,
        focused: false,
        disabled: false,
        tab_index: None,
        key_bindings: Vec::new(),
        modal: false,
      },
    }
  }

  pub fn has_class(&self, class: &str) -> bool {
    self.classes.contains(&class.to_string())
  }

  pub fn get_attribute(&self, key: &str) -> Option<&String> {
    self.attributes.get(key)
  }

  pub fn is_focusable(&self) -> bool {
    self.focusable && !self.disabled
  }

  pub fn is_focused(&self) -> bool {
    self.focused
  }

  pub fn set_focused(&mut self, focused: bool) {
    self.focused = focused;
  }

  pub fn is_disabled(&self) -> bool {
    self.disabled
  }

  pub fn set_disabled(&mut self, disabled: bool) {
    self.disabled = disabled;
  }

  pub fn get_tab_index(&self) -> Option<i32> {
    self.tab_index
  }

  pub fn get_key_bindings(&self) -> &[ElementKeyBinding] {
    &self.key_bindings
  }

  pub fn is_modal(&self) -> bool {
    self.modal
  }

  pub fn add_key_binding(&mut self, key: KeyCombination, action: ElementAction) {
    self.key_bindings.push(ElementKeyBinding { key, action });
  }
}

#[derive(Debug)]
pub struct ElementBuilder {
  element: Element,
}

impl ElementBuilder {
  pub fn class<S: Into<String>>(mut self, class: S) -> Self {
    self.element.classes.push(class.into());
    self
  }

  pub fn classes<I, S>(mut self, classes: I) -> Self
  where
    I: IntoIterator<Item = S>,
    S: Into<String>,
  {
    for class in classes {
      self.element.classes.push(class.into());
    }
    self
  }

  pub fn id<S: Into<String>>(mut self, id: S) -> Self {
    self.element.id = Some(id.into());
    self
  }

  pub fn attr<K, V>(mut self, key: K, value: V) -> Self
  where
    K: Into<String>,
    V: Into<String>,
  {
    self.element.attributes.insert(key.into(), value.into());
    self
  }

  pub fn content<S: Into<String>>(mut self, content: S) -> Self {
    self.element.content = Some(content.into());
    self
  }

  pub fn child(mut self, child: Element) -> Self {
    self.element.children.push(child);
    self
  }

  pub fn child_builder(mut self, child: ElementBuilder) -> Self {
    self.element.children.push(child.build());
    self
  }

  pub fn children<I>(mut self, children: I) -> Self
  where
    I: IntoIterator<Item = Element>,
  {
    self.element.children.extend(children);
    self
  }

  pub fn children_builders<I>(mut self, children: I) -> Self
  where
    I: IntoIterator<Item = ElementBuilder>,
  {
    for child in children {
      self.element.children.push(child.build());
    }
    self
  }

  pub fn focusable(mut self, focusable: bool) -> Self {
    self.element.focusable = focusable;
    self
  }

  pub fn disabled(mut self, disabled: bool) -> Self {
    self.element.disabled = disabled;
    if disabled {
      // When disabled, element should not be focusable
      self.element.focusable = false;
    }
    self
  }

  pub fn tab_index(mut self, index: i32) -> Self {
    self.element.tab_index = Some(index);
    // Only enable focusable if not disabled
    if !self.element.disabled {
      self.element.focusable = true;
    }
    self
  }

  /// Bind a key to this element
  pub fn bind_key(mut self, key: KeyCombination, action: ElementAction) -> Self {
    self
      .element
      .key_bindings
      .push(ElementKeyBinding { key, action });
    self
  }

  /// Convenient method to bind a character key
  pub fn bind_char(self, c: char, action: ElementAction) -> Self {
    self.bind_key(KeyCombination::char(c), action)
  }

  /// Convenient method to bind Enter key to activate
  pub fn bind_enter(self) -> Self {
    self.bind_key(KeyCombination::enter(), ElementAction::Activate)
  }

  /// Convenient method to bind Space key to activate  
  pub fn bind_space(self) -> Self {
    self.bind_key(KeyCombination::space(), ElementAction::Activate)
  }

  /// Mark this element as a modal (gets ESC dismiss by default)
  pub fn modal(mut self, is_modal: bool) -> Self {
    self.element.modal = is_modal;
    if is_modal {
      // Add default ESC binding to dismiss modal
      self.element.key_bindings.push(ElementKeyBinding {
        key: KeyCombination::escape(),
        action: ElementAction::Custom("dismiss".to_string()),
      });
    }
    self
  }

  /// Create a button element with default bindings
  pub fn button<S: Into<String>>(text: S) -> ElementBuilder {
    Element::with_tag("button")
      .content(text.into())
      .focusable(true)
      .bind_enter()
      .bind_space()
  }

  /// Create an input element with default bindings
  pub fn input<S: Into<String>>(placeholder: S) -> ElementBuilder {
    Element::with_tag("input")
      .attr("placeholder", placeholder.into())
      .focusable(true)
  }

  /// Create a modal element with ESC dismiss
  pub fn modal_dialog<S: Into<String>>(title: S) -> ElementBuilder {
    Element::with_tag("modal")
      .class("modal")
      .attr("title", title.into())
      .modal(true)
      .focusable(true)
  }

  pub fn build(self) -> Element {
    self.element
  }
}

// Implement From trait for seamless conversion
impl From<ElementBuilder> for Element {
  fn from(builder: ElementBuilder) -> Self {
    builder.build()
  }
}
