use reactive_tui::prelude::*;
use reactive_tui::themes::colors::dark_theme;
use reactive_tui::widgets::*;

fn main() -> Result<()> {
  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  println!("🍽️ Menu Widget Demo\n");

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width,
    height: 15,
  };
  let theme = dark_theme();

  // 1. Simple Menu with Builder API
  println!("📋 Simple Menu (Builder API):");
  let simple_menu = Menu::builder("simple-menu")
    .action("new", "New File", "file:new")
    .action("open", "Open File", "file:open")
    .action("save", "Save File", "file:save")
    .separator("sep1")
    .action("exit", "Exit", "app:exit")
    .build();

  println!("{}\n", simple_menu.render(&layout, Some(&theme)));

  // 2. Menu with Icons and Shortcuts
  println!("⚡ Menu with Icons and Shortcuts:");
  let icon_menu = Menu::builder("icon-menu")
    .item(MenuItem {
      id: "new".to_string(),
      label: "New Document".to_string(),
      item_type: MenuItemType::Action {
        action: "doc:new".to_string(),
      },
      icon: Some('📄'),
      shortcut: Some("Ctrl+N".to_string()),
      enabled: true,
      visible: true,
      css_classes: vec!["menu-item".to_string()],
      tooltip: Some("Create a new document".to_string()),
      data: std::collections::HashMap::new(),
    })
    .item(MenuItem {
      id: "open".to_string(),
      label: "Open Document".to_string(),
      item_type: MenuItemType::Action {
        action: "doc:open".to_string(),
      },
      icon: Some('📂'),
      shortcut: Some("Ctrl+O".to_string()),
      enabled: true,
      visible: true,
      css_classes: vec!["menu-item".to_string()],
      tooltip: Some("Open an existing document".to_string()),
      data: std::collections::HashMap::new(),
    })
    .item(MenuItem {
      id: "save".to_string(),
      label: "Save Document".to_string(),
      item_type: MenuItemType::Action {
        action: "doc:save".to_string(),
      },
      icon: Some('💾'),
      shortcut: Some("Ctrl+S".to_string()),
      enabled: true,
      visible: true,
      css_classes: vec!["menu-item".to_string()],
      tooltip: Some("Save the current document".to_string()),
      data: std::collections::HashMap::new(),
    })
    .build();

  println!("{}\n", icon_menu.render(&layout, Some(&theme)));

  // 3. Menu with Toggle and Radio Items
  println!("🔘 Menu with Toggle and Radio Items:");
  let interactive_menu = Menu::builder("interactive-menu")
    .header("view-header", "View Options")
    .toggle("word-wrap", "Word Wrap", false)
    .toggle("line-numbers", "Line Numbers", true)
    .separator("sep1")
    .header("theme-header", "Theme")
    .item(MenuItem {
      id: "theme-light".to_string(),
      label: "Light Theme".to_string(),
      item_type: MenuItemType::Radio {
        group: "theme".to_string(),
        selected: false,
      },
      icon: Some('☀'),
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: std::collections::HashMap::new(),
    })
    .item(MenuItem {
      id: "theme-dark".to_string(),
      label: "Dark Theme".to_string(),
      item_type: MenuItemType::Radio {
        group: "theme".to_string(),
        selected: true,
      },
      icon: Some('🌙'),
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: std::collections::HashMap::new(),
    })
    .build();

  println!("{}\n", interactive_menu.render(&layout, Some(&theme)));

  // 4. Hierarchical Menu with Submenus
  println!("🗂️ Hierarchical Menu with Submenus:");
  let file_submenu = vec![
    MenuItem {
      id: "new-text".to_string(),
      label: "Text File".to_string(),
      item_type: MenuItemType::Action {
        action: "file:new:text".to_string(),
      },
      icon: Some('📝'),
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: std::collections::HashMap::new(),
    },
    MenuItem {
      id: "new-markdown".to_string(),
      label: "Markdown File".to_string(),
      item_type: MenuItemType::Action {
        action: "file:new:markdown".to_string(),
      },
      icon: Some('📋'),
      shortcut: None,
      enabled: true,
      visible: true,
      css_classes: Vec::new(),
      tooltip: None,
      data: std::collections::HashMap::new(),
    },
  ];

  let hierarchical_menu = Menu::builder("hierarchical-menu")
    .submenu("file", "File", file_submenu)
    .action("edit", "Edit", "edit:menu")
    .action("view", "View", "view:menu")
    .action("help", "Help", "help:menu")
    .build();

  println!("{}\n", hierarchical_menu.render(&layout, Some(&theme)));

  // 5. Menu from JSON Configuration
  println!("📄 Menu from JSON Configuration:");
  let json_config = r#"[
        {
            "id": "git-status",
            "label": "Git Status",
            "type": "Action",
            "action": "git:status",
            "shortcut": "Ctrl+G S"
        },
        {
            "id": "git-commit",
            "label": "Git Commit",
            "type": "Action", 
            "action": "git:commit",
            "shortcut": "Ctrl+G C"
        },
        {
            "id": "git-push",
            "label": "Git Push",
            "type": "Action",
            "action": "git:push", 
            "shortcut": "Ctrl+G P"
        },
        {
            "id": "sep",
            "label": "",
            "type": "Separator"
        },
        {
            "id": "git-branch",
            "label": "Branch Management",
            "type": "Submenu",
            "items": [
                {
                    "id": "new-branch",
                    "label": "New Branch",
                    "type": "Action",
                    "action": "git:branch:new"
                },
                {
                    "id": "switch-branch", 
                    "label": "Switch Branch",
                    "type": "Action",
                    "action": "git:branch:switch"
                }
            ]
        }
    ]"#;

  let json_menu = Menu::from_json("json-menu", json_config)?;
  println!("{}\n", json_menu.render(&layout, Some(&theme)));

  // 6. Menu from YAML Configuration
  println!("📝 Menu from YAML Configuration:");
  let yaml_config = r#"
- id: debug-start
  label: Start Debugging
  type: Action
  action: debug:start
  shortcut: F5

- id: debug-step
  label: Step Over
  type: Action
  action: debug:step
  shortcut: F10

- id: debug-into
  label: Step Into
  type: Action
  action: debug:into
  shortcut: F11

- id: debug-out
  label: Step Out
  type: Action
  action: debug:out
  shortcut: Shift+F11

- id: sep
  label: ""
  type: Separator

- id: debug-breakpoints
  label: Breakpoints
  type: Submenu
  items:
    - id: toggle-breakpoint
      label: Toggle Breakpoint
      type: Action
      action: debug:breakpoint:toggle
      shortcut: F9
    
    - id: clear-breakpoints
      label: Clear All Breakpoints
      type: Action
      action: debug:breakpoint:clear
"#;

  let yaml_menu = Menu::from_yaml("yaml-menu", yaml_config)?;
  println!("{}\n", yaml_menu.render(&layout, Some(&theme)));

  // 7. Navigation Demo
  println!("🧭 Menu Navigation Demo:");
  let mut nav_menu = Menu::builder("nav-menu")
    .action("item1", "First Item", "action1")
    .action("item2", "Second Item", "action2")
    .action("item3", "Third Item", "action3")
    .build();

  println!("Initial selection (item 1):");
  println!("{}", nav_menu.render(&layout, Some(&theme)));

  // Navigate and show selection changes
  nav_menu.next_item();
  println!("After next_item() (item 2):");
  println!("{}", nav_menu.render(&layout, Some(&theme)));

  nav_menu.next_item();
  println!("After next_item() (item 3):");
  println!("{}", nav_menu.render(&layout, Some(&theme)));

  // Test activation
  if let Some(action) = nav_menu.activate_selected() {
    println!("Activated action: {action}\n");
  }

  // 8. Different Menu Styles
  println!("🎨 Menu Style Variations:");

  // Context menu style
  let context_menu = context_menu("context")
    .action("cut", "Cut", "edit:cut")
    .action("copy", "Copy", "edit:copy")
    .action("paste", "Paste", "edit:paste")
    .build();

  println!("Context Menu:");
  println!("{}", context_menu.render(&layout, Some(&theme)));

  // Menu bar style
  let menu_bar = menu_bar("menubar")
    .action("file", "File", "file:menu")
    .action("edit", "Edit", "edit:menu")
    .action("view", "View", "view:menu")
    .action("help", "Help", "help:menu")
    .build();

  println!("Menu Bar:");
  println!("{}", menu_bar.render(&layout, Some(&theme)));

  // Dropdown menu style
  let dropdown = dropdown_menu("dropdown")
    .action("option1", "Option 1", "option1")
    .action("option2", "Option 2", "option2")
    .action("option3", "Option 3", "option3")
    .build();

  println!("Dropdown Menu:");
  println!("{}", dropdown.render(&layout, Some(&theme)));

  println!("\n🎨 Menu Widget Demo Complete - All menu types and configurations demonstrated");

  Ok(())
}
