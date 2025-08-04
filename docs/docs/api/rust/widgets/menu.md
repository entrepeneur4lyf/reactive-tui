# Menu Widget

Comprehensive menu widget for navigation and action selection with hierarchical structure, keyboard shortcuts, and flexible styling for terminal user interfaces.

## Overview

The Menu widget provides hierarchical navigation menus with support for submenus, keyboard shortcuts, different item types (actions, toggles, radio buttons), and data-driven configuration from JSON/YAML.

```rust
use reactive_tui::widgets::{Menu, MenuItem, MenuItemType, MenuBuilder};

let main_menu = Menu::builder("main-menu")
    .orientation(MenuOrientation::Vertical)
    .add_item(MenuItem {
        id: "file".to_string(),
        label: "File".to_string(),
        item_type: MenuItemType::Submenu {
            items: vec![
                MenuItem::action("new", "New File", Some('n'), Some("Ctrl+N")),
                MenuItem::action("open", "Open File", Some('o'), Some("Ctrl+O")),
                MenuItem::separator(),
                MenuItem::action("exit", "Exit", Some('x'), Some("Ctrl+Q")),
            ]
        },
        icon: Some('📁'),
        shortcut: None,
        enabled: true,
        visible: true,
        css_classes: vec![],
        tooltip: Some("File operations".to_string()),
        data: HashMap::new(),
    })
    .build();
```

## Core Types

### MenuItemType

```rust
pub enum MenuItemType {
    /// Regular action item
    Action { action: String },
    /// Submenu with nested items  
    Submenu { items: Vec<MenuItem> },
    /// Separator line
    Separator,
    /// Header/label (non-interactive)
    Header,
    /// Toggle item with on/off state
    Toggle { state: bool },
    /// Radio group item
    Radio { group: String, selected: bool },
}
```

### MenuItem

```rust
pub struct MenuItem {
    pub id: String,
    pub label: String,
    pub item_type: MenuItemType,
    pub icon: Option<char>,
    pub shortcut: Option<String>,
    pub enabled: bool,
    pub visible: bool,
    pub css_classes: Vec<String>,
    pub tooltip: Option<String>,
    pub data: HashMap<String, String>,
}

impl MenuItem {
    pub fn action(id: &str, label: &str, icon: Option<char>, shortcut: Option<&str>) -> Self
    pub fn submenu(id: &str, label: &str, items: Vec<MenuItem>) -> Self
    pub fn separator() -> Self
    pub fn header(label: &str) -> Self
    pub fn toggle(id: &str, label: &str, state: bool) -> Self
    pub fn radio(id: &str, label: &str, group: &str, selected: bool) -> Self
}
```

### MenuOrientation

```rust
pub enum MenuOrientation {
    Vertical,    // Standard vertical menu
    Horizontal,  // Horizontal menu bar
}
```

### MenuItemState

```rust
pub enum MenuItemState {
    Normal,     // Default state
    Hovered,    // Mouse hover or keyboard highlight
    Selected,   // Currently selected
    Disabled,   // Disabled/inactive
}
```

## MenuBuilder

```rust
impl MenuBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn orientation(mut self, orientation: MenuOrientation) -> Self
    pub fn add_item(mut self, item: MenuItem) -> Self
    pub fn add_items(mut self, items: Vec<MenuItem>) -> Self
    pub fn style(mut self, style: MenuStyle) -> Self
    pub fn on_item_select<F>(mut self, callback: F) -> Self
    pub fn on_submenu_open<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Menu
}
```

## Styling Configuration

```rust
pub struct MenuStyle {
    pub background: Option<ColorDefinition>,
    pub foreground: Option<ColorDefinition>,
    pub hover_background: Option<ColorDefinition>,
    pub hover_foreground: Option<ColorDefinition>,
    pub selected_background: Option<ColorDefinition>,
    pub selected_foreground: Option<ColorDefinition>,
    pub disabled_foreground: Option<ColorDefinition>,
    pub border_color: Option<ColorDefinition>,
    pub separator_color: Option<ColorDefinition>,
    pub padding: u16,
    pub item_height: u16,
    pub show_borders: bool,
    pub show_icons: bool,
    pub show_shortcuts: bool,
    pub submenu_indent: u16,
}
```

## Examples

### File Menu

```rust
use reactive_tui::widgets::{Menu, MenuItem, MenuItemType};

let file_menu = Menu::builder("file-menu")
    .orientation(MenuOrientation::Vertical)
    .add_item(MenuItem {
        id: "file".to_string(),
        label: "File".to_string(),
        item_type: MenuItemType::Submenu {
            items: vec![
                MenuItem::action("new", "New File", Some('📄'), Some("Ctrl+N")),
                MenuItem::action("open", "Open File", Some('📂'), Some("Ctrl+O")),
                MenuItem::action("save", "Save", Some('💾'), Some("Ctrl+S")),
                MenuItem::action("save_as", "Save As...", Some('💾'), Some("Ctrl+Shift+S")),
                MenuItem::separator(),
                MenuItem::action("recent", "Recent Files", Some('📋'), None),
                MenuItem::separator(),
                MenuItem::action("exit", "Exit", Some('🚪'), Some("Ctrl+Q")),
            ]
        },
        icon: Some('📁'),
        shortcut: None,
        enabled: true,
        visible: true,
        css_classes: vec!["file-menu".to_string()],
        tooltip: Some("File operations".to_string()),
        data: HashMap::new(),
    })
    .on_item_select(|menu_item| {
        match menu_item.id.as_str() {
            "new" => create_new_file(),
            "open" => open_file_dialog(),
            "save" => save_current_file(),
            "exit" => quit_application(),
            _ => {}
        }
    })
    .build();
```

### Context Menu

```rust
let context_menu = Menu::builder("context-menu")
    .orientation(MenuOrientation::Vertical)
    .add_items(vec![
        MenuItem::action("cut", "Cut", Some('✂'), Some("Ctrl+X")),
        MenuItem::action("copy", "Copy", Some('📋'), Some("Ctrl+C")),
        MenuItem::action("paste", "Paste", Some('📌'), Some("Ctrl+V")),
        MenuItem::separator(),
        MenuItem::action("delete", "Delete", Some('🗑'), Some("Delete")),
        MenuItem::action("rename", "Rename", Some('✏'), Some("F2")),
        MenuItem::separator(),
        MenuItem::action("properties", "Properties", Some('⚙'), Some("Alt+Enter")),
    ])
    .style(MenuStyle {
        background: Some(ColorDefinition { r: 40, g: 40, b: 40 }),
        foreground: Some(ColorDefinition { r: 255, g: 255, b: 255 }),
        hover_background: Some(ColorDefinition { r: 70, g: 130, b: 180 }),
        border_color: Some(ColorDefinition { r: 128, g: 128, b: 128 }),
        show_borders: true,
        show_icons: true,
        show_shortcuts: true,
        padding: 1,
        item_height: 1,
        ..Default::default()
    })
    .build();
```

### Menu Bar with Multiple Menus

```rust
let menu_bar = Menu::builder("menu-bar")
    .orientation(MenuOrientation::Horizontal)
    .add_items(vec![
        MenuItem {
            id: "file".to_string(),
            label: "File".to_string(),
            item_type: MenuItemType::Submenu {
                items: vec![
                    MenuItem::action("new", "New", Some('📄'), Some("Ctrl+N")),
                    MenuItem::action("open", "Open", Some('📂'), Some("Ctrl+O")),
                    MenuItem::separator(),
                    MenuItem::action("exit", "Exit", Some('🚪'), Some("Ctrl+Q")),
                ]
            },
            icon: None,
            shortcut: None,
            enabled: true,
            visible: true,
            css_classes: vec![],
            tooltip: None,
            data: HashMap::new(),
        },
        MenuItem {
            id: "edit".to_string(),
            label: "Edit".to_string(),
            item_type: MenuItemType::Submenu {
                items: vec![
                    MenuItem::action("undo", "Undo", Some('↶'), Some("Ctrl+Z")),
                    MenuItem::action("redo", "Redo", Some('↷'), Some("Ctrl+Y")),
                    MenuItem::separator(),
                    MenuItem::action("cut", "Cut", Some('✂'), Some("Ctrl+X")),
                    MenuItem::action("copy", "Copy", Some('📋'), Some("Ctrl+C")),
                    MenuItem::action("paste", "Paste", Some('📌'), Some("Ctrl+V")),
                ]
            },
            icon: None,
            shortcut: None,
            enabled: true,
            visible: true,
            css_classes: vec![],
            tooltip: None,
            data: HashMap::new(),
        },
        MenuItem {
            id: "view".to_string(),
            label: "View".to_string(),
            item_type: MenuItemType::Submenu {
                items: vec![
                    MenuItem::toggle("word_wrap", "Word Wrap", false),
                    MenuItem::toggle("line_numbers", "Line Numbers", true),
                    MenuItem::separator(),
                    MenuItem::radio("theme", "Light Theme", "theme", false),
                    MenuItem::radio("theme", "Dark Theme", "theme", true),
                    MenuItem::radio("theme", "High Contrast", "theme", false),
                ]
            },
            icon: None,
            shortcut: None,
            enabled: true,
            visible: true,
            css_classes: vec![],
            tooltip: None,
            data: HashMap::new(),
        },
    ])
    .build();
```

### Settings Menu with Toggles and Radio Buttons

```rust
let settings_menu = Menu::builder("settings-menu")
    .orientation(MenuOrientation::Vertical)
    .add_items(vec![
        MenuItem::header("Display Settings"),
        MenuItem::toggle("dark_mode", "Dark Mode", true),
        MenuItem::toggle("show_line_numbers", "Show Line Numbers", true),
        MenuItem::toggle("word_wrap", "Word Wrap", false),
        MenuItem::separator(),
        MenuItem::header("Theme"),
        MenuItem::radio("theme", "Default", "theme", true),
        MenuItem::radio("theme", "Monokai", "theme", false),
        MenuItem::radio("theme", "Solarized", "theme", false),
        MenuItem::separator(),
        MenuItem::header("Font Size"),
        MenuItem::radio("font_size", "Small", "font_size", false),
        MenuItem::radio("font_size", "Medium", "font_size", true),
        MenuItem::radio("font_size", "Large", "font_size", false),
        MenuItem::separator(),
        MenuItem::action("restore_defaults", "Restore Defaults", Some('🔄'), None),
    ])
    .on_item_select(|item| {
        match &item.item_type {
            MenuItemType::Toggle { state } => {
                println!("Toggled {}: {}", item.label, state);
                update_setting(&item.id, *state);
            },
            MenuItemType::Radio { group, selected } => {
                if *selected {
                    println!("Selected {} in group {}", item.label, group);
                    update_radio_setting(group, &item.id);
                }
            },
            MenuItemType::Action { action } => {
                println!("Executed action: {}", action);
                execute_action(action);
            },
            _ => {}
        }
    })
    .build();
```

### Dropdown Menu

```rust
use reactive_tui::widgets::{Menu, MenuItem, Button, Element};

let dropdown_button = Button::builder("dropdown-btn", "Options ▼")
    .on_click(|| {
        show_dropdown_menu();
        Ok(())
    })
    .build();

let dropdown_menu = Menu::builder("dropdown-menu")
    .orientation(MenuOrientation::Vertical)
    .add_items(vec![
        MenuItem::action("edit", "Edit Item", Some('✏'), None),
        MenuItem::action("duplicate", "Duplicate", Some('📋'), None),
        MenuItem::separator(),
        MenuItem::action("move_up", "Move Up", Some('⬆'), None),
        MenuItem::action("move_down", "Move Down", Some('⬇'), None),
        MenuItem::separator(),
        MenuItem::action("delete", "Delete", Some('🗑'), Some("Delete")),
    ])
    .style(MenuStyle {
        show_borders: true,
        padding: 1,
        ..Default::default()
    })
    .build();

let dropdown_container = Element::with_tag("div")
    .class("dropdown-container")
    .child(dropdown_button.to_element())
    .child(dropdown_menu.to_element())
    .build();
```

### Nested Submenu

```rust
let nested_menu = Menu::builder("nested-menu")
    .add_item(MenuItem {
        id: "tools".to_string(),
        label: "Tools".to_string(),
        item_type: MenuItemType::Submenu {
            items: vec![
                MenuItem {
                    id: "development".to_string(),
                    label: "Development".to_string(),
                    item_type: MenuItemType::Submenu {
                        items: vec![
                            MenuItem::action("build", "Build Project", Some('🔨'), Some("Ctrl+B")),
                            MenuItem::action("test", "Run Tests", Some('🧪'), Some("Ctrl+T")),
                            MenuItem::action("debug", "Start Debugging", Some('🐛'), Some("F5")),
                            MenuItem::separator(),
                            MenuItem::action("clean", "Clean Build", Some('🧹'), None),
                        ]
                    },
                    icon: Some('⚙'),
                    shortcut: None,
                    enabled: true,
                    visible: true,
                    css_classes: vec![],
                    tooltip: Some("Development tools".to_string()),
                    data: HashMap::new(),
                },
                MenuItem {
                    id: "git".to_string(),
                    label: "Git".to_string(),
                    item_type: MenuItemType::Submenu {
                        items: vec![
                            MenuItem::action("commit", "Commit", Some('📝'), Some("Ctrl+Enter")),
                            MenuItem::action("push", "Push", Some('⤴'), None),
                            MenuItem::action("pull", "Pull", Some('⤵'), None),
                            MenuItem::separator(),
                            MenuItem::action("status", "Status", Some('📊'), None),
                            MenuItem::action("log", "Show Log", Some('📜'), None),
                        ]
                    },
                    icon: Some('🌿'),
                    shortcut: None,
                    enabled: true,
                    visible: true,
                    css_classes: vec![],
                    tooltip: Some("Git version control".to_string()),
                    data: HashMap::new(),
                },
                MenuItem::separator(),
                MenuItem::action("terminal", "Open Terminal", Some('💻'), Some("Ctrl+`")),
            ]
        },
        icon: Some('🔧'),
        shortcut: None,
        enabled: true,
        visible: true,
        css_classes: vec![],
        tooltip: None,
        data: HashMap::new(),
    })
    .build();
```

## Menu State Management

```rust
use reactive_tui::{widgets::Menu, reactive::Reactive};

#[derive(Debug, Clone, Default)]
pub struct MenuState {
    pub selected_index: usize,
    pub navigation_stack: Vec<Vec<MenuItem>>,
    pub current_items: Vec<MenuItem>,
    pub expanded_paths: Vec<String>,
    pub radio_selections: HashMap<String, String>,
    pub toggle_states: HashMap<String, bool>,
}

let menu_state = Reactive::new(MenuState::default());
let state_clone = menu_state.clone();

let stateful_menu = Menu::builder("stateful-menu")
    .add_items(menu_items)
    .on_item_select(move |item| {
        state_clone.update(|state| {
            match &item.item_type {
                MenuItemType::Toggle { state: toggle_state } => {
                    state.toggle_states.insert(item.id.clone(), *toggle_state);
                },
                MenuItemType::Radio { group, selected } => {
                    if *selected {
                        state.radio_selections.insert(group.clone(), item.id.clone());
                    }
                },
                _ => {}
            }
        });
    })
    .build();

// Watch for state changes
menu_state.watch(|state| {
    println!("Menu state updated:");
    println!("  Toggle states: {:?}", state.toggle_states);
    println!("  Radio selections: {:?}", state.radio_selections);
});
```

## Keyboard Navigation

```rust
impl Menu {
    pub fn handle_key_press(&mut self, key: &str) -> bool {
        match key {
            "ArrowUp" | "k" => self.navigate_up(),
            "ArrowDown" | "j" => self.navigate_down(),
            "ArrowRight" | "l" | "Enter" => self.enter_submenu_or_activate(),
            "ArrowLeft" | "h" | "Escape" => self.exit_submenu(),
            "Home" => self.navigate_to_first(),
            "End" => self.navigate_to_last(),
            "Space" => self.toggle_current_item(),
            _ => {
                // Check for shortcut keys
                self.activate_shortcut(key)
            }
        }
    }
}
```

## Convenience Functions

```rust
use reactive_tui::widgets::{context_menu, dropdown_menu, menu_bar};

// Create a context menu
let context = context_menu(vec![
    ("cut", "Cut", Some("Ctrl+X")),
    ("copy", "Copy", Some("Ctrl+C")),
    ("paste", "Paste", Some("Ctrl+V")),
]);

// Create a dropdown menu
let dropdown = dropdown_menu("Options", vec![
    ("edit", "Edit", None),
    ("delete", "Delete", Some("Del")),
]);

// Create a menu bar
let menu_bar = menu_bar(vec![
    ("file", "File", vec![
        ("new", "New", Some("Ctrl+N")),
        ("open", "Open", Some("Ctrl+O")),
    ]),
    ("edit", "Edit", vec![
        ("undo", "Undo", Some("Ctrl+Z")),
        ("redo", "Redo", Some("Ctrl+Y")),
    ]),
]);
```

## JSON/YAML Configuration

```rust
// Load menu from JSON configuration
use serde_json;

let menu_config = r#"
{
  "id": "main-menu",
  "orientation": "vertical",
  "items": [
    {
      "id": "file",
      "label": "File",
      "type": {
        "Submenu": {
          "items": [
            {
              "id": "new",
              "label": "New File",
              "type": { "Action": { "action": "file.new" } },
              "icon": "📄",
              "shortcut": "Ctrl+N"
            },
            {
              "id": "open",
              "label": "Open File", 
              "type": { "Action": { "action": "file.open" } },
              "icon": "📂",
              "shortcut": "Ctrl+O"
            }
          ]
        }
      }
    }
  ]
}
"#;

let menu_data: MenuData = serde_json::from_str(menu_config)?;
let menu = Menu::from_config(menu_data);
```

## CSS Styling

```css
.menu-container {
    background-color: #ffffff;
    border: 1px solid #ccc;
    border-radius: 4px;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.1);
    padding: 4px 0;
}

.menu-item {
    padding: 8px 16px;
    display: flex;
    align-items: center;
    gap: 8px;
    cursor: pointer;
    font-size: 14px;
    color: #333;
}

.menu-item:hover {
    background-color: #f0f0f0;
}

.menu-item.selected {
    background-color: #007acc;
    color: white;
}

.menu-item.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.menu-item-icon {
    width: 16px;
    text-align: center;
    font-size: 14px;
}

.menu-item-label {
    flex: 1;
    font-weight: 500;
}

.menu-item-shortcut {
    font-size: 12px;
    color: #666;
    font-family: monospace;
}

.menu-separator {
    height: 1px;
    background-color: #e0e0e0;
    margin: 4px 0;
}

.menu-header {
    padding: 4px 16px;
    font-size: 12px;
    font-weight: 600;
    color: #666;
    text-transform: uppercase;
    letter-spacing: 0.5px;
}

.menu-submenu-indicator {
    font-size: 12px;
    color: #999;
}

/* Horizontal menu bar */
.menu-bar {
    display: flex;
    background-color: #f8f8f8;
    border-bottom: 1px solid #ddd;
    padding: 0;
}

.menu-bar .menu-item {
    padding: 8px 16px;
    border-radius: 0;
}

/* Toggle and radio items */
.menu-item-toggle {
    position: relative;
}

.menu-item-toggle::before {
    content: "☐";
    margin-right: 8px;
}

.menu-item-toggle.checked::before {
    content: "☑";
}

.menu-item-radio {
    position: relative;
}

.menu-item-radio::before {
    content: "○";
    margin-right: 8px;
}

.menu-item-radio.selected::before {
    content: "●";
}
```

## Integration Examples

### With Modal Dialog

```rust
use reactive_tui::widgets::{Menu, Modal, Button, Element};

let menu_modal = Modal::builder("menu-modal")
    .title("Options")
    .content(
        Menu::builder("modal-menu")
            .add_items(vec![
                MenuItem::action("option1", "Option 1", Some('1'), None),
                MenuItem::action("option2", "Option 2", Some('2'), None),
                MenuItem::action("option3", "Option 3", Some('3'), None),
            ])
            .on_item_select(|item| {
                execute_option(&item.id);
                close_modal();
            })
            .build()
            .to_element()
    )
    .build();
```

The Menu widget provides comprehensive navigation and action selection functionality with hierarchical structure, keyboard shortcuts, and extensive customization options for terminal applications.