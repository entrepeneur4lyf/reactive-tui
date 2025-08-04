# Tabs Widget

Tab navigation component with multiple orientations, closable tabs, and dynamic tab management for organized content display.

## Overview

The Tabs widget provides tabbed interfaces with support for horizontal/vertical orientations, closable tabs, drag-and-drop reordering, and lazy loading of tab content.

```rust
use reactive_tui::widgets::{Tabs, TabsBuilder, Tab, TabOrientation};

let tabs = Tabs::builder("main-tabs")
    .orientation(TabOrientation::Horizontal)
    .add_tab(Tab::new("home", "Home", home_content))
    .add_tab(Tab::new("settings", "Settings", settings_content))
    .add_tab(Tab::new("about", "About", about_content))
    .closable(true)
    .scrollable(true)
    .build();
```

## TabsBuilder

```rust
impl TabsBuilder {
    pub fn new(id: &str) -> Self
    pub fn orientation(mut self, orientation: TabOrientation) -> Self
    pub fn add_tab(mut self, tab: Tab) -> Self
    pub fn active_tab(mut self, tab_id: &str) -> Self
    pub fn closable(mut self, closable: bool) -> Self
    pub fn scrollable(mut self, scrollable: bool) -> Self
    pub fn reorderable(mut self, reorderable: bool) -> Self
    pub fn tab_width(mut self, width: TabWidth) -> Self
    pub fn max_tabs(mut self, max_tabs: usize) -> Self
    pub fn on_tab_change<F>(mut self, callback: F) -> Self
    pub fn on_tab_close<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Tabs
}
```

## Tab Structure

```rust
pub struct Tab {
    pub id: String,
    pub title: String,
    pub content: Element,
    pub closable: bool,
    pub icon: Option<String>,
    pub badge: Option<String>,
    pub tooltip: Option<String>,
    pub lazy_load: bool,
}

impl Tab {
    pub fn new(id: &str, title: &str, content: Element) -> Self
    pub fn closable(mut self, closable: bool) -> Self
    pub fn icon(mut self, icon: &str) -> Self
    pub fn badge(mut self, badge: &str) -> Self
    pub fn tooltip(mut self, tooltip: &str) -> Self
    pub fn lazy_load(mut self, lazy: bool) -> Self
}
```

## Tab Orientations

```rust
pub enum TabOrientation {
    Horizontal,     // Tabs at top/bottom
    Vertical,       // Tabs at left/right
    Auto,           // Adaptive based on container size
}

pub enum TabPosition {
    Top,
    Bottom,
    Left,
    Right,
}
```

## Tab Width Options

```rust
pub enum TabWidth {
    Auto,           // Content-based width
    Fixed(u16),     // Fixed width for all tabs
    Uniform,        // Equal width distribution
    MinMax(u16, u16), // Min/max width constraints
}
```

## Examples

### Basic Horizontal Tabs

```rust
use reactive_tui::widgets::{Tabs, Tab, Element};

let basic_tabs = Tabs::builder("basic-tabs")
    .orientation(TabOrientation::Horizontal)
    .add_tab(Tab::new("tab1", "Dashboard", 
        Element::with_tag("div").text("Dashboard content").build()
    ))
    .add_tab(Tab::new("tab2", "Analytics", 
        Element::with_tag("div").text("Analytics content").build()
    ))
    .add_tab(Tab::new("tab3", "Settings", 
        Element::with_tag("div").text("Settings content").build()
    ))
    .build();
```

### Closable Tabs with Icons

```rust
let file_tabs = Tabs::builder("file-tabs")
    .orientation(TabOrientation::Horizontal)
    .closable(true)
    .add_tab(
        Tab::new("main.rs", "main.rs", file_content_1)
            .icon("📄")
            .closable(true)
            .tooltip("Main application file")
    )
    .add_tab(
        Tab::new("config.toml", "config.toml", file_content_2)
            .icon("⚙️")
            .closable(true)
            .tooltip("Configuration file")
    )
    .add_tab(
        Tab::new("readme.md", "README.md", file_content_3)
            .icon("📖")
            .closable(false) // Prevent closing this tab
            .tooltip("Project documentation")
    )
    .on_tab_close(|tab_id| {
        println!("Closing tab: {}", tab_id);
        save_file_if_dirty(tab_id)?;
        Ok(())
    })
    .build();
```

### Vertical Tabs

```rust
let vertical_tabs = Tabs::builder("sidebar-tabs")
    .orientation(TabOrientation::Vertical)
    .tab_width(TabWidth::Fixed(120))
    .add_tab(Tab::new("explorer", "Explorer", explorer_content).icon("📁"))
    .add_tab(Tab::new("search", "Search", search_content).icon("🔍"))
    .add_tab(Tab::new("git", "Git", git_content).icon("🌿"))
    .add_tab(Tab::new("debug", "Debug", debug_content).icon("🐛"))
    .build();
```

### Dynamic Tab Management

```rust
use reactive_tui::{widgets::Tabs, reactive::Reactive};

let tab_data = Reactive::new(Vec::<TabInfo>::new());
let active_tab = Reactive::new("home".to_string());

let dynamic_tabs = Tabs::builder("dynamic-tabs")
    .orientation(TabOrientation::Horizontal)
    .scrollable(true)
    .reorderable(true)
    .max_tabs(10)
    .on_tab_change({
        let active = active_tab.clone();
        move |tab_id| {
            active.set(tab_id.to_string());
            Ok(())
        }
    })
    .on_tab_close(|tab_id| {
        remove_tab(tab_id);
        Ok(())
    })
    .build();

// Add tabs dynamically
fn add_new_tab(title: &str, content: Element) {
    let tab = Tab::new(&format!("tab_{}", uuid::Uuid::new_v4()), title, content);
    dynamic_tabs.add_tab(tab);
}

// Remove tabs
fn close_tab(tab_id: &str) {
    dynamic_tabs.remove_tab(tab_id);
}
```

### Tabs with Badges

```rust
let notification_tabs = Tabs::builder("notification-tabs")
    .add_tab(
        Tab::new("inbox", "Inbox", inbox_content)
            .icon("📧")
            .badge("12") // 12 unread messages
    )
    .add_tab(
        Tab::new("sent", "Sent", sent_content)
            .icon("📤")
    )
    .add_tab(
        Tab::new("drafts", "Drafts", drafts_content)
            .icon("📝")
            .badge("3") // 3 draft messages
    )
    .build();
```

### Lazy Loading Tabs

```rust
let lazy_tabs = Tabs::builder("lazy-tabs")
    .add_tab(
        Tab::new("reports", "Reports", Element::with_tag("div").build())
            .lazy_load(true)
            .on_activate(|| {
                // Load content when tab becomes active
                let content = load_reports_content()?;
                set_tab_content("reports", content);
                Ok(())
            })
    )
    .add_tab(
        Tab::new("analytics", "Analytics", Element::with_tag("div").build())
            .lazy_load(true)
            .on_activate(|| {
                let content = load_analytics_content()?;
                set_tab_content("analytics", content);
                Ok(())
            })
    )
    .build();
```

## State Management

```rust
use reactive_tui::{widgets::Tabs, reactive::Reactive};

struct TabState {
    active_tab: String,
    tab_count: usize,
    has_unsaved_changes: bool,
}

let tab_state = Reactive::new(TabState {
    active_tab: "home".to_string(),
    tab_count: 3,
    has_unsaved_changes: false,
});

let stateful_tabs = Tabs::builder("stateful-tabs")
    .active_tab(&tab_state.get().active_tab)
    .on_tab_change({
        let state = tab_state.clone();
        move |tab_id| {
            let mut current_state = state.get();
            current_state.active_tab = tab_id.to_string();
            state.set(current_state);
            Ok(())
        }
    })
    .on_content_change({
        let state = tab_state.clone();
        move |_tab_id| {
            let mut current_state = state.get();
            current_state.has_unsaved_changes = true;
            state.set(current_state);
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.tabs {
    display: flex;
    flex-direction: column;
}

.tabs-horizontal {
    flex-direction: column;
}

.tabs-vertical {
    flex-direction: row;
}

.tab-list {
    display: flex;
    background-color: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
}

.tab-list-vertical {
    flex-direction: column;
    border-right: 1px solid #dee2e6;
    border-bottom: none;
    width: 200px;
}

.tab-button {
    padding: 8px 16px;
    border: none;
    background: transparent;
    cursor: pointer;
    border-bottom: 3px solid transparent;
    transition: all 0.2s ease;
    display: flex;
    align-items: center;
    gap: 8px;
}

.tab-button:hover {
    background-color: #e9ecef;
}

.tab-button.active {
    background-color: white;
    border-bottom-color: #007bff;
    font-weight: 600;
}

.tab-button-vertical {
    border-bottom: none;
    border-right: 3px solid transparent;
    justify-content: flex-start;
}

.tab-button-vertical.active {
    border-right-color: #007bff;
}

.tab-icon {
    font-size: 16px;
}

.tab-badge {
    background-color: #dc3545;
    color: white;
    font-size: 11px;
    padding: 2px 6px;
    border-radius: 10px;
    min-width: 18px;
    text-align: center;
}

.tab-close {
    margin-left: 8px;
    padding: 2px 4px;
    border-radius: 3px;
    opacity: 0.6;
    transition: opacity 0.2s ease;
}

.tab-close:hover {
    opacity: 1;
    background-color: rgba(0, 0, 0, 0.1);
}

.tab-content {
    flex: 1;
    padding: 16px;
    background-color: white;
}

.tab-scroll-buttons {
    display: flex;
    align-items: center;
}

.tab-scroll-button {
    padding: 8px;
    background: transparent;
    border: none;
    cursor: pointer;
}

.tab-scroll-button:hover {
    background-color: #e9ecef;
}
```

## Keyboard Navigation

```rust
// Built-in keyboard shortcuts
// Tab: Next tab
// Shift+Tab: Previous tab
// Ctrl+W: Close current tab (if closable)
// Ctrl+T: New tab (if supported)
// Ctrl+1-9: Switch to tab by number

let keyboard_tabs = Tabs::builder("keyboard-tabs")
    .keyboard_navigation(true)
    .shortcuts(vec![
        KeyboardShortcut::new("Ctrl+T", "new_tab"),
        KeyboardShortcut::new("Ctrl+W", "close_tab"),
        KeyboardShortcut::new("Ctrl+Tab", "next_tab"),
        KeyboardShortcut::new("Ctrl+Shift+Tab", "prev_tab"),
    ])
    .build();
```

## Integration Examples

### File Editor Tabs

```rust
use reactive_tui::widgets::{Tabs, Tab, TextEditor};

let editor_tabs = Tabs::builder("editor-tabs")
    .orientation(TabOrientation::Horizontal)
    .closable(true)
    .reorderable(true)
    .on_tab_close(|tab_id| {
        if has_unsaved_changes(tab_id) {
            show_save_dialog(tab_id)?;
        }
        close_file(tab_id);
        Ok(())
    })
    .on_tab_change(|tab_id| {
        update_status_bar(tab_id);
        Ok(())
    })
    .build();

// Add file tabs
fn open_file(file_path: &str) -> Result<()> {
    let content = std::fs::read_to_string(file_path)?;
    let editor = TextEditor::new(&content);
    
    let tab = Tab::new(file_path, file_path, editor.to_element())
        .icon(get_file_icon(file_path))
        .closable(true)
        .tooltip(file_path);
    
    editor_tabs.add_tab(tab);
    Ok(())
}
```

The Tabs widget provides comprehensive tabbed interface functionality with support for various orientations, dynamic tab management, and extensive customization options for terminal applications.