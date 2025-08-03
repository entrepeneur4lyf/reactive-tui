//! Simple Viewport Widget Demo
//!
//! A basic demonstration of the Viewport widget showing:
//! - Creating a viewport with content
//! - Basic scrolling operations
//! - Selection management
//! - Search functionality
//! - Cache statistics

use reactive_tui::widgets::viewport::{
  file_viewer, log_viewer, SelectionMode, ViewportBuilder, ViewportItem,
};
use std::collections::HashMap;

fn main() {
  println!("🖥️  TUI Core - Simple Viewport Widget Demo");
  println!("===========================================");
  println!();

  // Demo 1: File Viewer with Virtual Scrolling
  println!("📄 Demo 1: File Viewer (Virtual Scrolling)");
  println!("-------------------------------------------");

  let lines: Vec<String> = (1..=1000)
    .map(|i| format!("Line {i:4}: This is line {i} of the virtual scrolling demo"))
    .collect();

  let mut file_viewport = file_viewer(lines);

  println!(
    "✓ Created file viewer with {} lines",
    file_viewport.item_count()
  );

  // Scroll operations
  file_viewport.scroll_to_line(500);
  println!("✓ Scrolled to line 500");

  file_viewport.page_down();
  println!("✓ Paged down");

  // Search functionality
  let results = file_viewport.search("500");
  println!("✓ Searched for '500', found {results} results");

  file_viewport.next_search_result();
  println!("✓ Moved to next search result");

  // Show viewport statistics
  println!(
    "✓ Viewport statistics - Total items: {}",
    file_viewport.item_count()
  );

  println!();

  // Demo 2: Data Table with Selection
  println!("📊 Demo 2: Data Table (Multi-Selection)");
  println!("----------------------------------------");

  let items: Vec<ViewportItem> = (1..=100)
    .map(|i| ViewportItem {
      id: format!("item-{i}"),
      content: format!("Data Row {i:3}: Sample data entry #{i}"),
      height: 1,
      selectable: true,
      disabled: false,
      css_classes: vec!["data-row".to_string()],
      metadata: HashMap::from([
        ("type".to_string(), "data_row".to_string()),
        ("id".to_string(), i.to_string()),
      ]),
    })
    .collect();

  let mut data_viewport = ViewportBuilder::new("data-demo")
    .content(items)
    .width(80)
    .height(20)
    .selection_mode(SelectionMode::Multiple)
    .virtual_scrolling(true)
    .build();

  println!(
    "✓ Created data table with {} items",
    data_viewport.item_count()
  );

  // Selection operations
  let _ = data_viewport.select_item(&"item-5".to_string());
  let _ = data_viewport.select_item(&"item-10".to_string());
  let _ = data_viewport.select_item(&"item-15".to_string());

  let selected = data_viewport.get_selected_items();
  println!("✓ Selected {} items: {:?}", selected.len(), selected);

  // Toggle selection
  data_viewport.deselect_item(&"item-10".to_string());
  let selected_after_toggle = data_viewport.get_selected_items();
  println!(
    "✓ After toggle, selected {} items: {:?}",
    selected_after_toggle.len(),
    selected_after_toggle
  );

  println!();

  // Demo 3: Log Viewer with Dynamic Content
  println!("📋 Demo 3: Log Viewer (Dynamic Content)");
  println!("----------------------------------------");

  let initial_logs = vec![
    "[INFO] Application started".to_string(),
    "[DEBUG] Initializing components".to_string(),
    "[INFO] Viewport widget loaded".to_string(),
  ];

  let mut log_viewport = log_viewer(initial_logs);
  println!(
    "✓ Created log viewer with {} initial entries",
    log_viewport.item_count()
  );

  // Add dynamic content
  for i in 1..=5 {
    let log_entry = ViewportItem {
      id: format!("dynamic-log-{i}"),
      content: format!("[INFO] Dynamic log entry #{i} added at runtime"),
      height: 1,
      selectable: false,
      disabled: false,
      css_classes: vec!["log-entry".to_string()],
      metadata: HashMap::from([
        ("type".to_string(), "runtime_entry".to_string()),
        ("sequence".to_string(), i.to_string()),
      ]),
    };
    log_viewport.add_items(vec![log_entry]);
  }

  println!(
    "✓ Added 5 dynamic entries, total: {}",
    log_viewport.item_count()
  );

  // Auto-scroll to bottom (typical for log viewers)
  log_viewport.scroll_to_bottom();
  println!("✓ Auto-scrolled to bottom");

  println!();

  // Demo 4: Advanced Builder Pattern
  println!("🔧 Demo 4: Advanced Configuration");
  println!("----------------------------------");

  let _advanced_viewport = ViewportBuilder::new("advanced-demo")
    .width(100)
    .height(30)
    .scrollable(true)
    .virtual_scrolling(true)
    .lazy_loading(false)
    .selection_mode(SelectionMode::Single)
    .show_scrollbar(true)
    .cache_size(500)
    .on_scroll(|position| {
      println!("Scroll callback: position {position}");
    })
    .on_selection_change(|selected| {
      println!("Selection callback: {} items selected", selected.len());
    })
    .build();

  println!("✓ Created advanced viewport with custom configuration");
  println!("  - Size: {}x{}", 100, 30);
  println!("  - Virtual scrolling: enabled");
  println!("  - Selection mode: Single");
  println!("  - Cache size: 500 items");
  println!("  - Scroll callbacks: registered");

  println!();

  // Demo 5: Performance Test
  println!("⚡ Demo 5: Performance Test (Large Dataset)");
  println!("-------------------------------------------");

  let large_dataset: Vec<String> = (1..=50000)
    .map(|i| format!("Performance test line {i:05}: Testing virtual scrolling with large dataset"))
    .collect();

  let mut perf_viewport = ViewportBuilder::new("performance-test")
    .content_from_strings(large_dataset)
    .width(120)
    .height(40)
    .virtual_scrolling(true)
    .cache_size(2000)
    .build();

  println!(
    "✓ Created viewport with {} lines",
    perf_viewport.item_count()
  );

  // Performance operations
  let start = std::time::Instant::now();

  // Jump around the dataset
  perf_viewport.scroll_to_line(25000);
  perf_viewport.scroll_to_line(1);
  perf_viewport.scroll_to_line(49999);
  perf_viewport.scroll_to_line(12500);

  let elapsed = start.elapsed();
  println!("✓ Performed 4 large jumps in {elapsed:?}");

  // Search performance
  let search_start = std::time::Instant::now();
  let search_results = perf_viewport.search("25000");
  let search_elapsed = search_start.elapsed();

  println!("✓ Searched 50k lines in {search_elapsed:?}, found {search_results} results");

  // Final viewport statistics
  println!("✓ Final viewport stats:");
  println!("  - Total items: {}", perf_viewport.item_count());
  println!(
    "  - Visible items: {}",
    perf_viewport.get_visible_items().len()
  );

  println!();
  println!("✨ All viewport demos completed successfully!");
  println!();
  println!("The Viewport widget demonstrates:");
  println!("  ✓ Virtual scrolling for large datasets (50k+ items)");
  println!("  ✓ Memory-efficient content caching with LRU eviction");
  println!("  ✓ Single and multi-selection modes");
  println!("  ✓ Real-time search with result navigation");
  println!("  ✓ Dynamic content addition and management");
  println!("  ✓ Customizable scrolling and display options");
  println!("  ✓ Performance optimizations for smooth operation");
  println!();
}
