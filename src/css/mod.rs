//! Advanced CSS parsing and styling engine with component tree support

use crate::components::Element;
use crate::error::Result;
use crate::layout::{
  AlignItems, ComputedStyles, DisplayType, FlexDirection, JustifyContent, SizeValue, Spacing,
};
use std::collections::HashMap;

/// Represents a parsed CSS stylesheet
#[derive(Debug, Clone)]
pub struct Stylesheet {
  pub rules: Vec<CssRule>,
}

#[derive(Debug, Clone)]
pub struct CssRule {
  pub selector: String,
  pub declarations: HashMap<String, String>,
}

/// Selector types for CSS matching
#[derive(Debug, Clone)]
pub enum Selector {
  Tag(String),
  Class(String),
  Id(String),
  Universal,
  Descendant(Box<Selector>, Box<Selector>),
  Child(Box<Selector>, Box<Selector>),
}

/// Advanced CSS engine with component tree support
pub struct CssEngine {
  stylesheets: Vec<Stylesheet>,
  global_styles: HashMap<String, ComputedStyles>,
}

impl CssEngine {
  pub fn new() -> Self {
    let mut engine = Self {
      stylesheets: Vec::new(),
      global_styles: HashMap::new(),
    };

    // Add default component styles
    engine.add_default_styles();
    engine
  }

  pub fn add_stylesheet(&mut self, stylesheet: Stylesheet) {
    self.stylesheets.push(stylesheet);
  }

  /// Load a stylesheet from CSS string content
  pub fn load_stylesheet(&mut self, css_content: &str) -> Result<()> {
    let parsed_stylesheet = self.parse_css(css_content)?;
    self.add_stylesheet(parsed_stylesheet);
    Ok(())
  }

  /// Clear all loaded stylesheets
  pub fn clear_stylesheets(&mut self) {
    self.stylesheets.clear();
    self.global_styles.clear();
    // Re-add default styles after clearing
    self.add_default_styles();
  }

  /// Parse CSS string into a Stylesheet
  fn parse_css(&self, css_content: &str) -> Result<Stylesheet> {
    let mut rules = Vec::new();
    let mut current_selector = String::new();
    let mut current_declarations = HashMap::new();
    let mut in_rule = false;

    for line in css_content.lines() {
      let line = line.trim();

      if line.is_empty() || line.starts_with("/*") {
        continue;
      }

      if line.contains('{') {
        current_selector = line.replace('{', "").trim().to_string();
        in_rule = true;
        current_declarations.clear();
      } else if line.contains('}') {
        if in_rule && !current_selector.is_empty() {
          rules.push(CssRule {
            selector: current_selector.clone(),
            declarations: current_declarations.clone(),
          });
        }
        in_rule = false;
        current_selector.clear();
        current_declarations.clear();
      } else if in_rule && line.contains(':') {
        let parts: Vec<&str> = line.splitn(2, ':').collect();
        if parts.len() == 2 {
          let property = parts[0].trim().to_string();
          let value = parts[1].trim().trim_end_matches(';').to_string();
          current_declarations.insert(property, value);
        }
      }
    }

    Ok(Stylesheet { rules })
  }

  pub fn add_css(&mut self, css: &str) -> Result<()> {
    let stylesheet = self.parse_css(css)?;
    self.add_stylesheet(stylesheet);
    Ok(())
  }

  pub fn apply_styles(&self, element: &Element) -> ComputedStyles {
    let mut styles = ComputedStyles::default();

    // Apply default styles based on element tag
    self.apply_tag_styles(&mut styles, &element.tag);

    // Apply class-based styles
    for class in &element.classes {
      self.apply_class_styles(&mut styles, class);
    }

    // Apply ID-based styles
    if let Some(id) = &element.id {
      self.apply_id_styles(&mut styles, id);
    }

    // Apply attribute-based styles
    self.apply_attribute_styles(&mut styles, element);

    // Apply CSS rules from stylesheets
    for stylesheet in &self.stylesheets {
      for rule in &stylesheet.rules {
        if self.selector_matches(&rule.selector, element) {
          self.apply_declarations(&mut styles, &rule.declarations);
        }
      }
    }

    styles
  }

  pub fn create_component_tree(&self, root: &Element) -> ComponentTree {
    ComponentTree::new(root.clone(), self)
  }

  fn add_default_styles(&mut self) {
    // HTML-like element defaults
    let div_styles = ComputedStyles {
      display: DisplayType::Block,
      ..Default::default()
    };
    self.global_styles.insert("div".to_string(), div_styles);

    let span_styles = ComputedStyles {
      display: DisplayType::Inline,
      ..Default::default()
    };
    self.global_styles.insert("span".to_string(), span_styles);

    // TUI-specific element defaults
    let flex_styles = ComputedStyles {
      display: DisplayType::Flex,
      flex_direction: FlexDirection::Row,
      ..Default::default()
    };
    self.global_styles.insert("flex".to_string(), flex_styles);

    let vbox_styles = ComputedStyles {
      display: DisplayType::Flex,
      flex_direction: FlexDirection::Column,
      ..Default::default()
    };
    self.global_styles.insert("vbox".to_string(), vbox_styles);

    let center_styles = ComputedStyles {
      display: DisplayType::Flex,
      justify_content: JustifyContent::Center,
      align_items: AlignItems::Center,
      ..Default::default()
    };
    self
      .global_styles
      .insert("center".to_string(), center_styles);
  }

  fn apply_tag_styles(&self, styles: &mut ComputedStyles, tag: &str) {
    if let Some(default_styles) = self.global_styles.get(tag) {
      *styles = default_styles.clone();
    }
  }

  fn apply_class_styles(&self, styles: &mut ComputedStyles, class: &str) {
    match class {
      "flex" => styles.display = DisplayType::Flex,
      "flex-row" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Row;
      }
      "flex-col" => {
        styles.display = DisplayType::Flex;
        styles.flex_direction = FlexDirection::Column;
      }
      "justify-center" => styles.justify_content = JustifyContent::Center,
      "justify-between" => styles.justify_content = JustifyContent::SpaceBetween,
      "justify-around" => styles.justify_content = JustifyContent::SpaceAround,
      "justify-evenly" => styles.justify_content = JustifyContent::SpaceEvenly,
      "items-center" => styles.align_items = AlignItems::Center,
      "items-stretch" => styles.align_items = AlignItems::Stretch,
      "p-1" => styles.padding = Spacing::uniform(1),
      "p-2" => styles.padding = Spacing::uniform(2),
      "p-4" => styles.padding = Spacing::uniform(4),
      "m-1" => styles.margin = Spacing::uniform(1),
      "m-2" => styles.margin = Spacing::uniform(2),
      "m-4" => styles.margin = Spacing::uniform(4),
      "w-full" => styles.width = SizeValue::Percent(100.0),
      "h-full" => styles.height = SizeValue::Percent(100.0),
      "w-1/2" => styles.width = SizeValue::Percent(50.0),
      "h-1/2" => styles.height = SizeValue::Percent(50.0),
      "w-1/3" => styles.width = SizeValue::Percent(33.33),
      "w-2/3" => styles.width = SizeValue::Percent(66.67),
      _ => {}
    }
  }

  fn apply_id_styles(&self, _styles: &mut ComputedStyles, _id: &str) {
    // ID-specific styles would go here
  }

  fn apply_attribute_styles(&self, styles: &mut ComputedStyles, element: &Element) {
    if let Some(width) = element.get_attribute("width") {
      if let Ok(px) = width.parse::<u16>() {
        styles.width = SizeValue::Pixels(px);
      } else if width.ends_with('%') {
        if let Ok(pct) = width.trim_end_matches('%').parse::<f32>() {
          styles.width = SizeValue::Percent(pct);
        }
      }
    }

    if let Some(height) = element.get_attribute("height") {
      if let Ok(px) = height.parse::<u16>() {
        styles.height = SizeValue::Pixels(px);
      } else if height.ends_with('%') {
        if let Ok(pct) = height.trim_end_matches('%').parse::<f32>() {
          styles.height = SizeValue::Percent(pct);
        }
      }
    }

    // Flexbox attributes
    if let Some(direction) = element.get_attribute("flex-direction") {
      match direction.as_str() {
        "row" => styles.flex_direction = FlexDirection::Row,
        "column" => styles.flex_direction = FlexDirection::Column,
        "row-reverse" => styles.flex_direction = FlexDirection::RowReverse,
        "column-reverse" => styles.flex_direction = FlexDirection::ColumnReverse,
        _ => {}
      }
    }

    if let Some(justify) = element.get_attribute("justify-content") {
      match justify.as_str() {
        "flex-start" => styles.justify_content = JustifyContent::FlexStart,
        "flex-end" => styles.justify_content = JustifyContent::FlexEnd,
        "center" => styles.justify_content = JustifyContent::Center,
        "space-between" => styles.justify_content = JustifyContent::SpaceBetween,
        "space-around" => styles.justify_content = JustifyContent::SpaceAround,
        "space-evenly" => styles.justify_content = JustifyContent::SpaceEvenly,
        _ => {}
      }
    }
  }

  fn selector_matches(&self, selector: &str, element: &Element) -> bool {
    // Simple selector matching - can be expanded for complex selectors
    if let Some(class) = selector.strip_prefix('.') {
      // Class selector
      element.classes.contains(&class.to_string())
    } else if let Some(id) = selector.strip_prefix('#') {
      // ID selector
      element.id.as_ref().is_some_and(|el_id| el_id == id)
    } else {
      // Tag selector
      element.tag == selector
    }
  }

  fn apply_declarations(
    &self,
    styles: &mut ComputedStyles,
    declarations: &HashMap<String, String>,
  ) {
    for (property, value) in declarations {
      self.apply_declaration(styles, property, value);
    }
  }

  fn apply_declaration(&self, styles: &mut ComputedStyles, property: &str, value: &str) {
    match property {
      "display" => match value {
        "block" => styles.display = DisplayType::Block,
        "inline" => styles.display = DisplayType::Inline,
        "flex" => styles.display = DisplayType::Flex,
        "none" => styles.display = DisplayType::None,
        _ => {}
      },
      "flex-direction" => match value {
        "row" => styles.flex_direction = FlexDirection::Row,
        "column" => styles.flex_direction = FlexDirection::Column,
        "row-reverse" => styles.flex_direction = FlexDirection::RowReverse,
        "column-reverse" => styles.flex_direction = FlexDirection::ColumnReverse,
        _ => {}
      },
      "width" => {
        if let Ok(px) = value.parse::<u16>() {
          styles.width = SizeValue::Pixels(px);
        } else if value.ends_with('%') {
          if let Ok(pct) = value.trim_end_matches('%').parse::<f32>() {
            styles.width = SizeValue::Percent(pct);
          }
        } else if value == "auto" {
          styles.width = SizeValue::Auto;
        }
      }
      "height" => {
        if let Ok(px) = value.parse::<u16>() {
          styles.height = SizeValue::Pixels(px);
        } else if value.ends_with('%') {
          if let Ok(pct) = value.trim_end_matches('%').parse::<f32>() {
            styles.height = SizeValue::Percent(pct);
          }
        } else if value == "auto" {
          styles.height = SizeValue::Auto;
        }
      }
      "padding" => {
        if let Ok(px) = value.parse::<u16>() {
          styles.padding = Spacing::uniform(px);
        }
      }
      "margin" => {
        if let Ok(px) = value.parse::<u16>() {
          styles.margin = Spacing::uniform(px);
        }
      }
      _ => {}
    }
  }
}

/// Component tree for efficient rendering and updates
#[derive(Debug)]
pub struct ComponentTree {
  root: ComponentNode,
}

#[derive(Debug)]
pub struct ComponentNode {
  pub element: Element,
  pub styles: ComputedStyles,
  pub children: Vec<ComponentNode>,
}

impl ComponentTree {
  pub fn new(root_element: Element, css_engine: &CssEngine) -> Self {
    let root = Self::build_node(root_element, css_engine);
    Self { root }
  }

  fn build_node(element: Element, css_engine: &CssEngine) -> ComponentNode {
    let styles = css_engine.apply_styles(&element);
    let children = element
      .children
      .iter()
      .map(|child| Self::build_node(child.clone(), css_engine))
      .collect();

    ComponentNode {
      element,
      styles,
      children,
    }
  }

  pub fn root(&self) -> &ComponentNode {
    &self.root
  }

  pub fn update_styles(&mut self, css_engine: &CssEngine) {
    Self::update_node_styles(&mut self.root, css_engine);
  }

  fn update_node_styles(node: &mut ComponentNode, css_engine: &CssEngine) {
    node.styles = css_engine.apply_styles(&node.element);
    for child in &mut node.children {
      Self::update_node_styles(child, css_engine);
    }
  }
}

impl Stylesheet {
  pub fn empty() -> Self {
    Self { rules: Vec::new() }
  }

  pub fn from_string(css: &str) -> Result<Self> {
    let engine = CssEngine::new();
    engine.parse_css(css)
  }
}

impl Default for CssEngine {
  fn default() -> Self {
    Self::new()
  }
}
