# ScrollableList Widget

High-performance vertical scrolling list widget with item selection, keyboard navigation, search functionality, and efficient rendering for large datasets.

## Overview

The ScrollableList widget provides efficient scrolling through lists of items with full keyboard navigation, multi-selection modes, search/filtering, and smooth scrolling animations.

```rust
use reactive_tui::widgets::{ScrollableList, ListItem, SelectionMode};

let file_list = ScrollableList::builder("file-browser")
    .height(15)
    .selection_mode(SelectionMode::Multiple)
    .search_enabled(true)
    .show_icons(true)
    .show_subtitles(true)
    .items(vec![
        ListItem::new("file1", "README.md")
            .subtitle("Documentation file")
            .icon("📄"),
        ListItem::new("file2", "src/")
            .subtitle("Source directory")
            .icon("📁"),
    ])
    .build();
```

## Core Types

### ListItem

```rust
pub struct ListItem {
    pub id: String,
    pub text: String,
    pub subtitle: Option<String>,
    pub icon: Option<String>,
    pub metadata: HashMap<String, String>,
    pub disabled: bool,
    pub css_classes: Vec<String>,
}

impl ListItem {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self
    pub fn subtitle(mut self, subtitle: impl Into<String>) -> Self
    pub fn icon(mut self, icon: impl Into<String>) -> Self
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
}
```

### SelectionMode

```rust
pub enum SelectionMode {
    Single,    // Single item selection
    Multiple,  // Multiple item selection with checkboxes
    None,      // No selection allowed (read-only)
}
```

### ScrollableListState

```rust
pub struct ScrollableListState {
    pub scroll_position: usize,
    pub highlighted_index: Option<usize>,
    pub selected_items: Vec<String>,
    pub is_focused: bool,
    pub search_query: String,
    pub filtered_indices: Vec<usize>,
    pub total_items: usize,
    pub visible_items: usize,
    pub search_active: bool,
}
```

## ScrollableListBuilder

```rust
impl ScrollableListBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn items(mut self, items: Vec<ListItem>) -> Self
    pub fn item(mut self, item: ListItem) -> Self
    pub fn height(mut self, height: usize) -> Self
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self
    pub fn show_scrollbar(mut self, show: bool) -> Self
    pub fn show_icons(mut self, show: bool) -> Self
    pub fn show_subtitles(mut self, show: bool) -> Self
    pub fn search_enabled(mut self, enabled: bool) -> Self
    pub fn scroll_step(mut self, step: usize) -> Self
    pub fn on_selection_change<F>(mut self, callback: F) -> Self
    pub fn on_item_activate<F>(mut self, callback: F) -> Self
    pub fn build(self) -> ScrollableList
}
```

## Configuration

```rust
pub struct ScrollableListConfig {
    pub height: usize,                // List height in lines
    pub selection_mode: SelectionMode, // Selection behavior
    pub show_scrollbar: bool,         // Show scrollbar indicator
    pub show_icons: bool,             // Display item icons
    pub show_subtitles: bool,         // Display item subtitles
    pub search_enabled: bool,         // Enable search functionality
    pub vim_navigation: bool,         // Vim-style navigation keys
    pub auto_scroll: bool,            // Auto-scroll to highlighted item
    pub smooth_scrolling: bool,       // Smooth scrolling animation
    pub scroll_step: usize,           // Lines to scroll at once
    pub item_height: usize,           // Height of each item
    pub padding: usize,               // Internal padding
    pub border_width: usize,          // Border width
}
```

## Examples

### Basic File Browser List

```rust
use reactive_tui::widgets::{ScrollableList, ListItem, SelectionMode};

let file_items = vec![
    ListItem::new("readme", "README.md")
        .subtitle("2.1 KB")
        .icon("📄")
        .metadata("size", "2048")
        .metadata("type", "file"),
    
    ListItem::new("src", "src/")
        .subtitle("Directory")
        .icon("📁")
        .metadata("type", "directory"),
    
    ListItem::new("cargo", "Cargo.toml")
        .subtitle("1.5 KB")
        .icon("📦")
        .metadata("size", "1536")
        .metadata("type", "file"),
];

let file_browser = ScrollableList::builder("file-browser")
    .items(file_items)
    .height(10)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(true)
    .on_selection_change(|selected_ids| {
        println!("Selected files: {:?}", selected_ids);
    })
    .on_item_activate(|item_id, item| {
        println!("Opened: {} ({})", item.text, item_id);
    })
    .build();
```

### Task Management List

```rust
use reactive_tui::widgets::{ScrollableList, ListItem, SelectionMode};

let task_items = vec![
    ListItem::new("task1", "Implement authentication system")
        .subtitle("High priority • Due tomorrow")
        .icon("🔴")
        .metadata("priority", "high")
        .metadata("status", "in_progress"),
    
    ListItem::new("task2", "Write unit tests")
        .subtitle("Medium priority • Due next week")
        .icon("🟡")
        .metadata("priority", "medium")
        .metadata("status", "todo"),
    
    ListItem::new("task3", "Deploy to production")
        .subtitle("Low priority • Completed")
        .icon("✅")
        .metadata("priority", "low")
        .metadata("status", "completed"),
];

let task_list = ScrollableList::builder("task-manager")
    .items(task_items)
    .height(12)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(true)
    .on_selection_change(|selected_task_ids| {
        println!("Selected {} tasks", selected_task_ids.len());
    })
    .on_item_activate(|task_id, task| {
        let status = task.metadata.get("status").unwrap_or(&"unknown".to_string());
        println!("Task {} status: {}", task.text, status);
    })
    .build();
```

### Menu Navigation List

```rust
let menu_items = vec![
    ListItem::new("new", "New File")
        .subtitle("Ctrl+N")
        .icon("📄")
        .metadata("action", "file.new"),
    
    ListItem::new("open", "Open File")
        .subtitle("Ctrl+O")
        .icon("📂")
        .metadata("action", "file.open"),
    
    ListItem::new("save", "Save File")
        .subtitle("Ctrl+S")
        .icon("💾")
        .metadata("action", "file.save"),
    
    ListItem::new("separator", "─────────────")
        .disabled(true),
    
    ListItem::new("exit", "Exit")
        .subtitle("Ctrl+Q")
        .icon("🚪")
        .metadata("action", "app.exit"),
];

let context_menu = ScrollableList::builder("context-menu")
    .items(menu_items)
    .height(menu_items.len().min(8))
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(false)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(false)
    .on_item_activate(|item_id, item| {
        if let Some(action) = item.metadata.get("action") {
            execute_menu_action(action);
        }
    })
    .build();
```

### Large Dataset with Search

```rust
use reactive_tui::{widgets::ScrollableList, reactive::Reactive};

// Generate large dataset
let large_dataset: Vec<ListItem> = (0..10000)
    .map(|i| {
        ListItem::new(format!("item_{}", i), format!("Item #{}", i + 1))
            .subtitle(format!("Description for item {}", i + 1))
            .icon(if i % 3 == 0 { "📄" } else if i % 3 == 1 { "📁" } else { "🔧" })
            .metadata("index", i.to_string())
            .metadata("category", match i % 4 {
                0 => "documents",
                1 => "folders", 
                2 => "tools",
                _ => "misc",
            })
    })
    .collect();

let large_list = ScrollableList::builder("large-dataset")
    .items(large_dataset)
    .height(15)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(true)
    .scroll_step(5)
    .on_search_change(|query, result_count| {
        if !query.is_empty() {
            println!("Search '{}' found {} results", query, result_count);
        }
    })
    .build();
```

### Dynamic List Updates

```rust
use reactive_tui::{widgets::ScrollableList, reactive::Reactive};

let mut notification_list = ScrollableList::builder("notifications")
    .height(8)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .show_icons(true)
    .show_subtitles(true)
    .search_enabled(false)
    .build();

// Add notifications dynamically
fn add_notification(list: &mut ScrollableList, message: &str, level: &str) {
    let icon = match level {
        "error" => "❌",
        "warning" => "⚠️",
        "info" => "ℹ️",
        "success" => "✅",
        _ => "📝",
    };
    
    let timestamp = chrono::Utc::now().format("%H:%M:%S");
    let notification = ListItem::new(
        format!("notif_{}", timestamp),
        message.to_string()
    )
    .subtitle(format!("{} • {}", level.to_uppercase(), timestamp))
    .icon(icon)
    .metadata("level", level)
    .metadata("timestamp", timestamp.to_string());
    
    list.add_item(notification);
    
    // Auto-scroll to latest notification
    if !list.items.is_empty() {
        list.scroll_to_item(list.items.len() - 1);
    }
}

// Usage
add_notification(&mut notification_list, "Application started", "info");
add_notification(&mut notification_list, "Configuration loaded", "success");
add_notification(&mut notification_list, "Database connection failed", "error");
```

## Keyboard Navigation

The ScrollableList supports comprehensive keyboard navigation:

```rust
// Default key bindings (can be customized)
let key_bindings = vec![
    ("ArrowUp", "Move up one item"),
    ("ArrowDown", "Move down one item"), 
    ("k", "Move up (vim-style)"),
    ("j", "Move down (vim-style)"),
    ("Home", "Go to first item"),
    ("End", "Go to last item"),
    ("PageUp", "Scroll up one page"),
    ("PageDown", "Scroll down one page"),
    ("Enter", "Activate highlighted item"),
    ("Space", "Toggle selection (multiple mode)"),
    ("Ctrl+A", "Select all items"),
    ("Escape", "Clear search/selection"),
    ("Delete", "Remove selected items"),
    ("/", "Start search mode"),
];

// Handle key events
impl ScrollableList {
    pub fn handle_key_press(&mut self, key: &str) -> bool {
        match key {
            "ArrowUp" | "k" => self.select_previous(),
            "ArrowDown" | "j" => self.select_next(),
            "Home" => self.select_first(),
            "End" => self.select_last(),
            "PageUp" => self.scroll_page_up(),
            "PageDown" => self.scroll_page_down(),
            "Enter" | " " => self.activate_highlighted(),
            "Escape" => self.clear_search(),
            _ => false,
        }
    }
}
```

## State Management

```rust
use reactive_tui::{widgets::ScrollableList, reactive::Reactive};

// Create list with reactive state
let list_state = Reactive::new(ScrollableListState::default());
let state_clone = list_state.clone();

let mut list = ScrollableList::builder("stateful-list")
    .items(initial_items)
    .height(10)
    .selection_mode(SelectionMode::Multiple)
    .on_selection_change(move |selected| {
        // Update reactive state when selection changes
        state_clone.update(|state| {
            state.selected_items = selected.to_vec();
        });
    })
    .build();

// Watch for state changes
list_state.watch(|state| {
    println!("Selection changed: {} items selected", state.selected_items.len());
    println!("Current search: '{}'", state.search_query);
    println!("Scroll position: {}", state.scroll_position);
});
```

## Convenience Functions

```rust
// Pre-configured list types
use reactive_tui::widgets::{file_browser_list, menu_list, task_list};

// File browser
let file_list = file_browser_list(vec![
    ("README.md", "file", Some("2.1 KB")),
    ("src/", "directory", None),
    ("Cargo.toml", "file", Some("1.5 KB")),
]);

// Context menu
let context_menu = menu_list(vec![
    ("new", "New File", Some("Ctrl+N")),
    ("open", "Open File", Some("Ctrl+O")),
    ("save", "Save File", Some("Ctrl+S")),
]);

// Task list
let tasks = task_list(vec![
    ("task1", "Complete project", "in_progress"),
    ("task2", "Review code", "completed"),
    ("task3", "Deploy to prod", "pending"),
]);
```

## CSS Styling

```css
.scrollable-list {
    border: 1px solid #ccc;
    border-radius: 4px;
    background-color: #ffffff;
}

.scrollable-list-item {
    padding: 4px 8px;
    display: flex;
    align-items: center;
    gap: 8px;
    border-bottom: 1px solid #f0f0f0;
}

.scrollable-list-item:hover {
    background-color: #f5f5f5;
}

.scrollable-list-item.selected {
    background-color: #007acc;
    color: white;
}

.scrollable-list-item.highlighted {
    background-color: #e1ecf4;
    border-left: 3px solid #007acc;
}

.scrollable-list-item.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.scrollable-list-item-icon {
    font-size: 16px;
    width: 20px;
    text-align: center;
}

.scrollable-list-item-content {
    flex: 1;
    min-width: 0;
}

.scrollable-list-item-title {
    font-weight: 500;
    font-size: 14px;
    color: #333;
    overflow: hidden;
    text-overflow: ellipsis;
}

.scrollable-list-item-subtitle {
    font-size: 12px;
    color: #666;
    margin-top: 2px;
    overflow: hidden;
    text-overflow: ellipsis;
}

.scrollable-list-scrollbar {
    position: absolute;
    right: 0;
    top: 0;
    bottom: 0;
    width: 1px;
    background-color: #ddd;
}

.scrollable-list-scrollbar-thumb {
    background-color: #999;
    width: 100%;
    border-radius: 2px;
}

.scrollable-list-search-highlight {
    background-color: #ffff00;
    font-weight: bold;
    color: #000;
}

.scrollable-list-empty {
    padding: 20px;
    text-align: center;
    color: #999;
    font-style: italic;
}
```

## Performance Considerations

The ScrollableList widget is optimized for large datasets:

- **Virtual Rendering**: Only visible items are rendered
- **Efficient Filtering**: Search operations use optimized algorithms
- **Lazy Loading**: Items can be loaded on-demand
- **Memory Management**: Minimal memory footprint for large lists
- **Smooth Scrolling**: Hardware-accelerated scrolling when available

```rust
// For very large datasets, use pagination or virtual scrolling
let optimized_list = ScrollableList::builder("large-list")
    .items(items)
    .height(20)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .search_enabled(true)
    .scroll_step(10) // Larger scroll steps for better performance
    .build();
```

## Integration Examples

### With Form Controls

```rust
use reactive_tui::widgets::{ScrollableList, Button, Input, Element};

let file_manager = Element::with_tag("div")
    .class("file-manager")
    .child(
        Input::builder("search")
            .placeholder("Search files...")
            .on_change(|query| {
                // Update list search in real-time
                file_list.set_search_query(&query);
                Ok(())
            })
            .build()
            .to_element()
    )
    .child(
        ScrollableList::builder("files")
            .items(file_items)
            .height(15)
            .selection_mode(SelectionMode::Multiple)
            .show_scrollbar(true)
            .show_icons(true)
            .show_subtitles(true)
            .search_enabled(true)
            .build()
            .to_element()
    )
    .child(
        Element::with_tag("div")
            .class("file-actions")
            .child(
                Button::builder("delete", "Delete Selected")
                    .button_type(ButtonType::Danger)
                    .on_click(|| delete_selected_files())
                    .build()
                    .to_element()
            )
            .build()
    )
    .build();
```

The ScrollableList widget provides comprehensive list functionality with efficient rendering, flexible selection modes, and extensive customization options for terminal applications.