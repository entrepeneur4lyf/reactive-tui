# Viewport Widget

A comprehensive scrollable viewport widget supporting virtual scrolling, lazy loading, and efficient rendering of large datasets with smooth scrolling and keyboard navigation.

## Overview

The Viewport widget provides a scrollable container that can efficiently handle large datasets through virtual scrolling and lazy loading. It supports selection, search, and smooth scrolling with customizable styling and event handling.

```rust
use reactive_tui::widgets::*;

let file_viewer = Viewport::builder("file-viewer")
    .content_from_strings(file_lines)
    .width(100)
    .height(30)
    .scrollable(true)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .build();
```

## Features

- **Virtual Scrolling**: Efficiently render only visible items for large datasets (10k+ items)
- **Lazy Loading**: Load content on-demand with async callbacks and loading indicators
- **Smooth Scrolling**: Pixel-perfect scrolling with momentum and easing
- **Keyboard Navigation**: Arrow keys, Page Up/Down, Home/End, vim-style navigation
- **Mouse Support**: Mouse wheel scrolling and drag scrolling
- **Scrollbar Rendering**: Customizable scrollbar with position indicators
- **Content Caching**: Intelligent content caching with LRU eviction
- **Search Integration**: Find and scroll to content with highlighting
- **Selection Support**: Single/multi-selection with keyboard and mouse
- **Responsive Sizing**: Automatic sizing based on container and content

## Core Components

### Viewport

Main viewport widget with scrolling and selection capabilities.

```rust
pub struct Viewport {
    pub id: String,
    pub items: Vec<ViewportItem>,
    pub state: Reactive<ViewportState>,
    pub config: ViewportConfig,
    pub style: ViewportStyle,
    pub callbacks: ViewportCallbacks,
    pub css_classes: Vec<String>,
}
```

### ViewportItem

Individual content item in the viewport.

```rust
pub struct ViewportItem {
    pub id: ContentId,
    pub content: String,
    pub height: u16,
    pub selectable: bool,
    pub disabled: bool,
    pub css_classes: Vec<String>,
    pub metadata: HashMap<String, String>,
}

impl ViewportItem {
    pub fn new(id: impl Into<String>, content: impl Into<String>) -> Self
    pub fn height(mut self, height: u16) -> Self
    pub fn selectable(mut self, selectable: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
}
```

### ViewportState

State management for scrolling, selection, and search.

```rust
pub struct ViewportState {
    pub scroll_position: f64,
    pub visible_range: (LineNumber, LineNumber),
    pub selected_items: Vec<ContentId>,
    pub highlighted_item: Option<ContentId>,
    pub focused: bool,
    pub disabled: bool,
    pub search_query: String,
    pub search_results: Vec<LineNumber>,
    pub current_search_result: Option<usize>,
    pub lazy_states: HashMap<ContentId, LazyLoadState>,
    pub cache_stats: CacheStats,
}
```

### ViewportConfig

Configuration options for viewport behavior.

```rust
pub struct ViewportConfig {
    pub width: u16,
    pub height: u16,
    pub scrollable: bool,
    pub scroll_mode: ScrollMode,
    pub selection_mode: SelectionMode,
    pub scrollbar_position: ScrollbarPosition,
    pub show_scrollbar: bool,
    pub virtual_scrolling: bool,
    pub virtual_buffer: usize,
    pub lazy_loading: bool,
    pub lazy_threshold: usize,
    pub cache_size: usize,
    pub mouse_support: bool,
    pub keyboard_navigation: bool,
    pub scroll_sensitivity: f64,
    pub momentum_scrolling: bool,
}
```

### Enums

```rust
pub enum ScrollMode {
    Line,    // Line-by-line scrolling
    Pixel,   // Pixel-perfect scrolling
    Page,    // Page-based scrolling
}

pub enum SelectionMode {
    None,      // No selection
    Single,    // Single item selection
    Multiple,  // Multiple item selection
    Range,     // Range selection
}

pub enum ScrollbarPosition {
    Right,   // Right side of viewport
    Left,    // Left side of viewport
    Hidden,  // Hidden scrollbar
}

pub enum LazyLoadState {
    NotLoaded,
    Loading,
    Loaded,
    Error(String),
}
```

## Builder Pattern

### ViewportBuilder

```rust
impl ViewportBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn content(mut self, items: Vec<ViewportItem>) -> Self
    pub fn content_from_strings(mut self, strings: Vec<String>) -> Self
    pub fn item(mut self, item: ViewportItem) -> Self
    pub fn width(mut self, width: u16) -> Self
    pub fn height(mut self, height: u16) -> Self
    pub fn scrollable(mut self, scrollable: bool) -> Self
    pub fn scroll_mode(mut self, mode: ScrollMode) -> Self
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self
    pub fn show_scrollbar(mut self, show: bool) -> Self
    pub fn virtual_scrolling(mut self, enabled: bool) -> Self
    pub fn lazy_loading(mut self, enabled: bool) -> Self
    pub fn cache_size(mut self, size: usize) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn on_scroll<F>(mut self, callback: F) -> Self
    pub fn on_selection_change<F>(mut self, callback: F) -> Self
    pub fn on_item_activate<F>(mut self, callback: F) -> Self
    pub fn on_lazy_load<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Viewport
}
```

## Methods

### Content Management

```rust
impl Viewport {
    // Set viewport content
    pub fn set_content(&mut self, items: Vec<ViewportItem>)
    
    // Add items to viewport
    pub fn add_items(&mut self, items: Vec<ViewportItem>)
    
    // Insert item at specific position
    pub fn insert_item(&mut self, index: usize, item: ViewportItem)
    
    // Remove item by ID
    pub fn remove_item(&mut self, id: &ContentId) -> Option<ViewportItem>
    
    // Get item by ID
    pub fn get_item(&self, id: &ContentId) -> Option<&ViewportItem>
    
    // Get item by line number
    pub fn get_item_at_line(&self, line: LineNumber) -> Option<&ViewportItem>
}
```

### Scrolling Control

```rust
impl Viewport {
    // Scroll to specific position
    pub fn scroll_to(&mut self, position: f64) -> bool
    
    // Scroll to specific line
    pub fn scroll_to_line(&mut self, line: LineNumber) -> bool
    
    // Scroll down by amount
    pub fn scroll_down(&mut self, amount: f64) -> bool
    
    // Scroll up by amount
    pub fn scroll_up(&mut self, amount: f64) -> bool
    
    // Page down
    pub fn page_down(&mut self) -> bool
    
    // Page up
    pub fn page_up(&mut self) -> bool
    
    // Scroll to top
    pub fn scroll_to_top(&mut self) -> bool
    
    // Scroll to bottom
    pub fn scroll_to_bottom(&mut self) -> bool
}
```

### Selection Management

```rust
impl Viewport {
    // Select item by ID
    pub fn select_item(&mut self, id: &ContentId) -> Result<()>
    
    // Deselect item by ID
    pub fn deselect_item(&mut self, id: &ContentId)
    
    // Clear all selections
    pub fn clear_selection(&mut self)
    
    // Get selected item IDs
    pub fn get_selected_items(&self) -> Vec<ContentId>
    
    // Highlight item by ID
    pub fn highlight_item(&mut self, id: &ContentId) -> Result<()>
    
    // Clear highlight
    pub fn clear_highlight(&mut self)
}
```

### Search Functionality

```rust
impl Viewport {
    // Search for content
    pub fn search(&mut self, query: impl Into<String>) -> usize
    
    // Navigate to next search result
    pub fn next_search_result(&mut self) -> bool
    
    // Navigate to previous search result
    pub fn previous_search_result(&mut self) -> bool
    
    // Clear search
    pub fn clear_search(&mut self)
}
```

### State Management

```rust
impl Viewport {
    // Set focus state
    pub fn set_focused(&mut self, focused: bool)
    
    // Check if viewport is focused
    pub fn is_focused(&self) -> bool
    
    // Enable/disable viewport
    pub fn set_disabled(&mut self, disabled: bool)
    
    // Check if viewport is disabled
    pub fn is_disabled(&self) -> bool
    
    // Get total item count
    pub fn item_count(&self) -> usize
    
    // Get visible items
    pub fn get_visible_items(&self) -> Vec<&ViewportItem>
}
```

## Examples

### Basic File Viewer

```rust
use reactive_tui::widgets::*;
use std::fs;

let file_content = fs::read_to_string("large_file.txt")?;
let lines: Vec<String> = file_content.lines().map(String::from).collect();

let file_viewer = Viewport::builder("file-viewer")
    .content_from_strings(lines)
    .width(100)
    .height(30)
    .scrollable(true)
    .scroll_mode(ScrollMode::Line)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .build();
```

### Log Viewer with Virtual Scrolling

```rust
let log_entries: Vec<String> = load_log_entries(); // 10,000+ entries

let log_viewer = Viewport::builder("log-viewer")
    .content_from_strings(log_entries)
    .width(120)
    .height(25)
    .virtual_scrolling(true)
    .scroll_mode(ScrollMode::Line)
    .selection_mode(SelectionMode::None)
    .show_scrollbar(true)
    .cache_size(1000)
    .build();
```

### Data Table with Selection

```rust
let table_data = vec![
    ViewportItem::new("row1", "Alice    | 25  | alice@example.com")
        .selectable(true)
        .metadata("email", "alice@example.com"),
    ViewportItem::new("row2", "Bob      | 30  | bob@example.com")
        .selectable(true)
        .metadata("email", "bob@example.com"),
    ViewportItem::new("row3", "Charlie  | 28  | charlie@example.com")
        .selectable(true)
        .metadata("email", "charlie@example.com"),
];

let data_table = Viewport::builder("data-table")
    .content(table_data)
    .width(80)
    .height(20)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .on_selection_change(|selected_ids| {
        println!("Selected items: {:?}", selected_ids);
    })
    .build();
```

### Lazy Loading Viewport

```rust
use reactive_tui::widgets::*;

let mut lazy_viewport = Viewport::builder("lazy-data")
    .width(100)
    .height(20)
    .lazy_loading(true)
    .virtual_scrolling(true)
    .on_lazy_load(|start_line, count| {
        // Load data on demand
        load_data_range(start_line, count)
    })
    .build();

// Set initial placeholder items
let placeholder_items: Vec<ViewportItem> = (0..10000)
    .map(|i| ViewportItem::new(format!("item-{}", i), format!("Loading item {}...", i)))
    .collect();

lazy_viewport.set_content(placeholder_items);
```

### Search Integration

```rust
let mut searchable_viewport = Viewport::builder("searchable")
    .content_from_strings(document_lines)
    .width(80)
    .height(30)
    .selection_mode(SelectionMode::Single)
    .on_search(|query, results| {
        println!("Found {} matches for '{}'", results.len(), query);
    })
    .build();

// Search functionality
searchable_viewport.search("TODO");
searchable_viewport.next_search_result();
searchable_viewport.previous_search_result();
```

### Interactive File Browser

```rust
use reactive_tui::{widgets::*, components::*};

struct FileBrowserItem {
    path: String,
    is_directory: bool,
    size: u64,
}

let file_items: Vec<ViewportItem> = scan_directory("./")
    .into_iter()
    .map(|file| {
        let icon = if file.is_directory { "📁" } else { "📄" };
        let content = format!("{} {} ({})", icon, file.path, format_size(file.size));
        
        ViewportItem::new(file.path.clone(), content)
            .selectable(true)
            .metadata("is_directory", file.is_directory.to_string())
            .metadata("size", file.size.to_string())
    })
    .collect();

let file_browser = Viewport::builder("file-browser")
    .content(file_items)
    .width(60)
    .height(25)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .on_item_activate(|id, item| {
        if item.metadata.get("is_directory") == Some(&"true".to_string()) {
            // Navigate into directory
            change_directory(&item.content);
        } else {
            // Open file
            open_file(&item.content);
        }
    })
    .build();
```

### Real-time Log Monitor

```rust
use reactive_tui::{widgets::*, reactive::Reactive};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader};

let log_lines = Reactive::new(Vec::<String>::new());
let lines_clone = log_lines.clone();

let log_monitor = Viewport::builder("log-monitor")
    .content_from_strings(log_lines.get())
    .width(120)
    .height(30)
    .scroll_mode(ScrollMode::Line)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .virtual_scrolling(true)
    .build();

// Tail log file
tokio::spawn(async move {
    let file = File::open("/var/log/app.log").await.unwrap();
    let reader = BufReader::new(file);
    let mut lines = reader.lines();
    
    while let Some(line) = lines.next_line().await.unwrap() {
        lines_clone.update(|logs| {
            logs.push(line);
            // Keep only last 1000 lines
            if logs.len() > 1000 {
                logs.remove(0);
            }
        });
    }
});
```

### Multi-Column Data Viewer

```rust
struct DataRecord {
    id: u32,
    name: String,
    email: String,
    status: String,
}

let records = load_data_records();
let formatted_items: Vec<ViewportItem> = records
    .into_iter()
    .map(|record| {
        let content = format!(
            "{:<8} {:<20} {:<30} {}",
            record.id, record.name, record.email, record.status
        );
        
        ViewportItem::new(format!("record-{}", record.id), content)
            .selectable(true)
            .class(format!("status-{}", record.status.to_lowercase()))
            .metadata("id", record.id.to_string())
            .metadata("status", record.status)
    })
    .collect();

let data_viewer = Viewport::builder("data-viewer")
    .content(formatted_items)
    .width(80)
    .height(25)
    .selection_mode(SelectionMode::Multiple)
    .show_scrollbar(true)
    .virtual_scrolling(true)
    .on_selection_change(|selected| {
        println!("Selected {} records", selected.len());
    })
    .build();
```

### Paginated Content Loader

```rust
struct PaginatedViewport {
    viewport: Viewport,
    current_page: usize,
    page_size: usize,
    total_items: usize,
}

impl PaginatedViewport {
    fn new(id: String, page_size: usize) -> Self {
        let viewport = Viewport::builder(&id)
            .width(100)
            .height(20)
            .selection_mode(SelectionMode::Single)
            .show_scrollbar(true)
            .build();
            
        Self {
            viewport,
            current_page: 0,
            page_size,
            total_items: 0,
        }
    }
    
    fn load_page(&mut self, page: usize) -> Result<()> {
        let start = page * self.page_size;
        let end = (start + self.page_size).min(self.total_items);
        
        let items = load_items_range(start, end)?;
        let viewport_items = items.into_iter()
            .enumerate()
            .map(|(i, item)| {
                ViewportItem::new(
                    format!("item-{}", start + i),
                    format!("Page {} Item {}: {}", page + 1, i + 1, item)
                )
            })
            .collect();
            
        self.viewport.set_content(viewport_items);
        self.current_page = page;
        
        Ok(())
    }
    
    fn next_page(&mut self) -> Result<()> {
        let max_page = (self.total_items + self.page_size - 1) / self.page_size;
        if self.current_page + 1 < max_page {
            self.load_page(self.current_page + 1)
        } else {
            Ok(())
        }
    }
    
    fn previous_page(&mut self) -> Result<()> {
        if self.current_page > 0 {
            self.load_page(self.current_page - 1)
        } else {
            Ok(())
        }
    }
}
```

## Convenience Functions

Pre-configured viewport functions for common use cases:

```rust
// File viewer viewport
pub fn file_viewer(lines: Vec<String>) -> Viewport

// Log viewer viewport with virtual scrolling
pub fn log_viewer(logs: Vec<String>) -> Viewport

// Data table viewport with selection
pub fn data_table_viewport(items: Vec<ViewportItem>) -> Viewport
```

## Performance Optimization

### Virtual Scrolling

For large datasets (1000+ items):

```rust
let large_dataset = generate_large_dataset(50000); // 50k items

let efficient_viewport = Viewport::builder("large-data")
    .content(large_dataset)
    .virtual_scrolling(true)
    .virtual_buffer(50)    // Keep 50 items above/below visible area
    .cache_size(1000)      // Cache up to 1000 items
    .lazy_loading(true)    // Load items on demand
    .build();
```

### Memory Management

```rust
let memory_efficient = Viewport::builder("efficient")
    .content_from_strings(data)
    .virtual_scrolling(true)
    .cache_size(500)       // Limit memory usage
    .lazy_threshold(10)    // Load items 10 lines before needed
    .build();

// Monitor cache performance
let cache_stats = memory_efficient.state.get().cache_stats;
println!("Cache hit rate: {:.1}%", cache_stats.hit_rate * 100.0);
println!("Memory usage: {} bytes", cache_stats.memory_usage);
```

## Event Handling

```rust
let interactive_viewport = Viewport::builder("interactive")
    .content(items)
    .selection_mode(SelectionMode::Multiple)
    .on_scroll(|position| {
        println!("Scrolled to position: {}", position);
    })
    .on_selection_change(|selected_ids| {
        println!("Selection changed: {} items", selected_ids.len());
    })
    .on_item_activate(|id, item| {
        println!("Activated item: {} - {}", id, item.content);
        open_item_details(id, item);
    })
    .on_lazy_load(|start_line, count| {
        println!("Loading {} items starting at line {}", count, start_line);
        async_load_data(start_line, count)
    })
    .build();
```

## CSS Integration

The viewport generates semantic CSS classes:

```css
.viewport {
    /* Base viewport styles */
}

.viewport-focused {
    /* Focused state */
}

.viewport-disabled {
    /* Disabled state */
}

.viewport-item {
    /* Individual item styling */
}

.viewport-item-selected {
    /* Selected item */
}

.viewport-item-highlighted {
    /* Highlighted item */
}

.viewport-scrollbar {
    /* Scrollbar styling */
}
```

## Accessibility

- **Keyboard Navigation**: Full arrow key and page navigation support
- **Screen Reader**: Proper ARIA labels and live regions
- **Focus Management**: Clear focus indicators and tab navigation
- **Selection Announcements**: Screen reader feedback for selection changes

The Viewport widget provides comprehensive scrollable content functionality with excellent performance characteristics and extensive customization options for handling large datasets efficiently.