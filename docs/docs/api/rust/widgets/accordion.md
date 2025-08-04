# Accordion Widget

Expandable and collapsible sections component with animation support, multiple expansion modes, and flexible content organization.

## Overview

The Accordion widget provides collapsible content sections with smooth animations, keyboard navigation, and support for single or multiple panel expansion.

```rust
use reactive_tui::widgets::{Accordion, AccordionBuilder, AccordionPanel, ExpansionMode};

let settings_accordion = Accordion::builder("settings")
    .expansion_mode(ExpansionMode::Single)
    .add_panel(AccordionPanel::new("general", "General Settings", general_content))
    .add_panel(AccordionPanel::new("appearance", "Appearance", appearance_content))
    .add_panel(AccordionPanel::new("advanced", "Advanced", advanced_content))
    .animation_duration(300)
    .build();
```

## AccordionBuilder

```rust
impl AccordionBuilder {
    pub fn new(id: &str) -> Self
    pub fn expansion_mode(mut self, mode: ExpansionMode) -> Self
    pub fn add_panel(mut self, panel: AccordionPanel) -> Self
    pub fn default_expanded(mut self, panel_ids: Vec<&str>) -> Self
    pub fn animation_duration(mut self, duration_ms: u32) -> Self
    pub fn collapsible(mut self, collapsible: bool) -> Self
    pub fn bordered(mut self, bordered: bool) -> Self
    pub fn on_expand<F>(mut self, callback: F) -> Self
    pub fn on_collapse<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Accordion
}
```

## Accordion Panel

```rust
pub struct AccordionPanel {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub expanded: bool,
    pub disabled: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tooltip: Option<String>,
}

impl AccordionPanel {
    pub fn new(id: &str, title: &str, content: Element) -> Self
    pub fn expanded(mut self, expanded: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn icon(mut self, icon: &str) -> Self
    pub fn badge(mut self, badge: &str) -> Self
    pub fn tooltip(mut self, tooltip: &str) -> Self
    pub fn collapsible(mut self, collapsible: bool) -> Self
}
```

## Expansion Modes

```rust
pub enum ExpansionMode {
    Single,         // Only one panel can be expanded
    Multiple,       // Multiple panels can be expanded
    AtLeastOne,     // At least one panel must remain expanded
}
```

## Animation Options

```rust
pub enum AccordionAnimation {
    None,           // No animation
    Slide,          // Slide up/down animation
    Fade,           // Fade in/out animation
    Scale,          // Scale animation
    Custom(Box<dyn AnimationFunction>),
}

pub struct AnimationConfig {
    pub duration: u32,      // Duration in milliseconds
    pub easing: EasingFunction,
    pub animation_type: AccordionAnimation,
}
```

## Examples

### Basic Accordion

```rust
use reactive_tui::widgets::{Accordion, AccordionPanel, Element};

let basic_accordion = Accordion::builder("basic")
    .expansion_mode(ExpansionMode::Single)
    .add_panel(
        AccordionPanel::new("panel1", "First Panel", 
            Element::with_tag("div").text("Content of first panel").build()
        )
    )
    .add_panel(
        AccordionPanel::new("panel2", "Second Panel",
            Element::with_tag("div").text("Content of second panel").build()
        )
    )
    .add_panel(
        AccordionPanel::new("panel3", "Third Panel",
            Element::with_tag("div").text("Content of third panel").build()
        )
    )
    .build();
```

### Settings Accordion

```rust
let settings_accordion = Accordion::builder("settings")
    .expansion_mode(ExpansionMode::Multiple)
    .default_expanded(vec!["general"])
    .add_panel(
        AccordionPanel::new("general", "General", create_general_settings())
            .icon("⚙️")
            .expanded(true)
    )
    .add_panel(
        AccordionPanel::new("appearance", "Appearance", create_appearance_settings())
            .icon("🎨")
    )
    .add_panel(
        AccordionPanel::new("privacy", "Privacy & Security", create_privacy_settings())
            .icon("🔒")
            .badge("New")
    )
    .add_panel(
        AccordionPanel::new("advanced", "Advanced", create_advanced_settings())
            .icon("🔧")
            .tooltip("Advanced configuration options")
    )
    .bordered(true)
    .animation_duration(250)
    .build();
```

### FAQ Accordion

```rust
let faq_accordion = Accordion::builder("faq")
    .expansion_mode(ExpansionMode::Multiple)
    .add_panel(
        AccordionPanel::new("q1", "How do I install the application?",
            Element::with_tag("div")
                .child(Element::with_tag("p").text("You can install the application using:").build())
                .child(Element::with_tag("code").text("cargo install reactive-tui").build())
                .build()
        )
    )
    .add_panel(
        AccordionPanel::new("q2", "How do I create custom widgets?",
            Element::with_tag("div")
                .text("To create custom widgets, implement the ResponsiveWidget trait...")
                .build()
        )
    )
    .add_panel(
        AccordionPanel::new("q3", "Can I use custom themes?",
            Element::with_tag("div")
                .text("Yes, you can create custom themes using the ColorTheme builder...")
                .build()
        )
    )
    .collapsible(true)
    .build();
```

### Nested Accordions

```rust
let nested_accordion = Accordion::builder("main")
    .expansion_mode(ExpansionMode::Multiple)
    .add_panel(
        AccordionPanel::new("section1", "Main Section 1",
            Element::with_tag("div")
                .child(
                    Element::with_tag("p")
                        .text("This section contains a nested accordion:")
                        .build()
                )
                .child(
                    Accordion::builder("nested1")
                        .expansion_mode(ExpansionMode::Single)
                        .add_panel(AccordionPanel::new("sub1", "Subsection 1", subsection_content1))
                        .add_panel(AccordionPanel::new("sub2", "Subsection 2", subsection_content2))
                        .build()
                        .to_element()
                )
                .build()
        )
    )
    .add_panel(
        AccordionPanel::new("section2", "Main Section 2", main_section_content2)
    )
    .build();
```

### Form Accordion

```rust
use reactive_tui::widgets::{Accordion, Form, Input, Checkbox};

let form_accordion = Accordion::builder("user-form")
    .expansion_mode(ExpansionMode::AtLeastOne)
    .default_expanded(vec!["personal"])
    .add_panel(
        AccordionPanel::new("personal", "Personal Information",
            Form::builder("personal-form")
                .add_field(Input::builder("first_name").label("First Name").required(true).build())
                .add_field(Input::builder("last_name").label("Last Name").required(true).build())
                .add_field(Input::builder("email").label("Email").input_type(InputType::Email).build())
                .build()
                .to_element()
        )
        .icon("👤")
    )
    .add_panel(
        AccordionPanel::new("address", "Address",
            Form::builder("address-form")
                .add_field(Input::builder("street").label("Street Address").build())
                .add_field(Input::builder("city").label("City").build())
                .add_field(Input::builder("postal").label("Postal Code").build())
                .build()
                .to_element()
        )
        .icon("🏠")
    )
    .add_panel(
        AccordionPanel::new("preferences", "Preferences",
            Form::builder("preferences-form")
                .add_field(Checkbox::builder("newsletter").label("Subscribe to newsletter").build())
                .add_field(Checkbox::builder("notifications").label("Enable notifications").build())
                .build()
                .to_element()
        )
        .icon("⚙️")
    )
    .on_expand(|panel_id| {
        println!("Panel expanded: {}", panel_id);
        Ok(())
    })
    .build();
```

### Dynamic Accordion

```rust
use reactive_tui::{widgets::Accordion, reactive::Reactive};

let accordion_data = Reactive::new(vec![
    ("item1", "Dynamic Item 1", "Content 1"),
    ("item2", "Dynamic Item 2", "Content 2"),
]);

let dynamic_accordion = Accordion::builder("dynamic")
    .expansion_mode(ExpansionMode::Multiple)
    .build();

// Add panels dynamically
fn update_accordion_content() {
    dynamic_accordion.clear_panels();
    
    for (id, title, content) in accordion_data.get() {
        let panel = AccordionPanel::new(id, title,
            Element::with_tag("div").text(content).build()
        );
        dynamic_accordion.add_panel(panel);
    }
}

// Add new item
fn add_accordion_item(id: &str, title: &str, content: &str) {
    let mut data = accordion_data.get();
    data.push((id, title, content));
    accordion_data.set(data);
    update_accordion_content();
}
```

## State Management

```rust
use reactive_tui::{widgets::Accordion, reactive::Reactive};

struct AccordionState {
    expanded_panels: Vec<String>,
    disabled_panels: Vec<String>,
}

let accordion_state = Reactive::new(AccordionState {
    expanded_panels: vec!["panel1".to_string()],
    disabled_panels: vec![],
});

let stateful_accordion = Accordion::builder("stateful")
    .expansion_mode(ExpansionMode::Multiple)
    .on_expand({
        let state = accordion_state.clone();
        move |panel_id| {
            let mut current_state = state.get();
            if !current_state.expanded_panels.contains(&panel_id.to_string()) {
                current_state.expanded_panels.push(panel_id.to_string());
            }
            state.set(current_state);
            Ok(())
        }
    })
    .on_collapse({
        let state = accordion_state.clone();
        move |panel_id| {
            let mut current_state = state.get();
            current_state.expanded_panels.retain(|p| p != panel_id);
            state.set(current_state);
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.accordion {
    border: 1px solid #e5e7eb;
    border-radius: 6px;
    overflow: hidden;
}

.accordion-panel {
    border-bottom: 1px solid #e5e7eb;
}

.accordion-panel:last-child {
    border-bottom: none;
}

.accordion-header {
    display: flex;
    align-items: center;
    padding: 12px 16px;
    background-color: #f9fafb;
    cursor: pointer;
    transition: background-color 0.2s ease;
    border: none;
    width: 100%;
    text-align: left;
}

.accordion-header:hover {
    background-color: #f3f4f6;
}

.accordion-header.expanded {
    background-color: #e5e7eb;
}

.accordion-header.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.accordion-icon {
    margin-right: 8px;
    font-size: 16px;
}

.accordion-title {
    flex: 1;
    font-weight: 500;
    font-size: 14px;
}

.accordion-badge {
    background-color: #3b82f6;
    color: white;
    padding: 2px 8px;
    border-radius: 12px;
    font-size: 11px;
    margin-left: 8px;
}

.accordion-chevron {
    margin-left: 8px;
    transition: transform 0.2s ease;
}

.accordion-chevron.expanded {
    transform: rotate(180deg);
}

.accordion-content {
    padding: 16px;
    background-color: white;
    border-top: 1px solid #e5e7eb;
    animation: accordion-expand 0.2s ease-out;
}

.accordion-content.collapsed {
    display: none;
}

@keyframes accordion-expand {
    from {
        opacity: 0;
        max-height: 0;
        padding-top: 0;
        padding-bottom: 0;
    }
    to {
        opacity: 1;
        max-height: 1000px;
        padding-top: 16px;
        padding-bottom: 16px;
    }
}

.accordion-slide .accordion-content {
    overflow: hidden;
    transition: max-height 0.3s ease, padding 0.3s ease;
}

.accordion-fade .accordion-content {
    transition: opacity 0.3s ease;
}
```

## Keyboard Navigation

```rust
// Built-in keyboard shortcuts
// Space/Enter: Toggle panel expansion
// Arrow Up/Down: Navigate between panels
// Home: Focus first panel
// End: Focus last panel

let keyboard_accordion = Accordion::builder("keyboard")
    .keyboard_navigation(true)
    .focus_visible(true)
    .shortcuts(vec![
        AccordionShortcut::new("Ctrl+A", "expand_all"),
        AccordionShortcut::new("Ctrl+Shift+A", "collapse_all"),
    ])
    .build();
```

## Integration Examples

### File Explorer Accordion

```rust
use reactive_tui::widgets::{Accordion, Tree};

let file_explorer = Accordion::builder("file-explorer")
    .expansion_mode(ExpansionMode::Multiple)
    .add_panel(
        AccordionPanel::new("recent", "Recent Files",
            create_recent_files_list()
        )
        .icon("🕒")
        .expanded(true)
    )
    .add_panel(
        AccordionPanel::new("projects", "Project Folders",
            Tree::builder("project-tree")
                .root_nodes(get_project_folders())
                .build()
                .to_element()
        )
        .icon("📁")
    )
    .add_panel(
        AccordionPanel::new("bookmarks", "Bookmarks",
            create_bookmarks_list()
        )
        .icon("⭐")
    )
    .build();
```

### Compact Settings Accordion

```rust
let compact_accordion = Accordion::builder("compact-settings")
    .expansion_mode(ExpansionMode::Single)
    .animation_duration(200)
    .add_panel(
        AccordionPanel::new("display", "Display", display_settings)
            .icon("🖥️")
    )
    .add_panel(
        AccordionPanel::new("audio", "Audio", audio_settings)
            .icon("🔊")
    )
    .add_panel(
        AccordionPanel::new("network", "Network", network_settings)
            .icon("🌐")
    )
    .bordered(false)
    .build();
```

The Accordion widget provides flexible content organization with smooth animations, multiple expansion modes, and comprehensive customization options for terminal applications.