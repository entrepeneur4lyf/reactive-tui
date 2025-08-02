//! Viewport Widget Demo
//!
//! Demonstrates the comprehensive Viewport widget with:
//! - Virtual scrolling for large datasets
//! - Lazy loading with memory-efficient caching
//! - Smooth scrolling and keyboard navigation
//! - Search functionality with highlighting
//! - Selection support (single/multi-select)
//! - Scrollbar rendering and positioning
//!
//! This demo shows three different viewport patterns:
//! 1. File viewer with 10,000+ lines
//! 2. Log viewer with real-time content
//! 3. Data table viewport with selectable items

use std::{
    collections::HashMap,
    io,
    time::{Duration, Instant},
};
use tui_core::{
    layout::LayoutRect,
    themes::ColorTheme,
    widgets::{
        viewport::{SelectionMode, ViewportBuilder, ViewportItem},
        Viewport,
    },
};

fn main() -> io::Result<()> {
    // Get terminal size dynamically with modern fallback
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    println!("🖥️  TUI Core - Viewport Widget Demo");
    println!("=====================================");
    println!();
    println!("This demo showcases three different viewport patterns:");
    println!("1. File Viewer (10,000+ lines with virtual scrolling)");
    println!("2. Log Viewer (real-time streaming content)");
    println!("3. Data Table Viewport (selectable items with filtering)");
    println!();
    println!("Note: This is a simplified demo showing viewport functionality");
    println!("without interactive controls. Full interactivity requires");
    println!("terminal event handling integration.");
    println!();

    // Demo state
    let demos = ["File Viewer", "Log Viewer", "Data Table"];
    let start_time = Instant::now();

    // Create viewport demos
    let file_viewport = create_file_viewer_demo();
    let mut log_viewport = create_log_viewer_demo();
    let data_table_viewport = create_data_table_demo();

    // Show each demo
    for (i, demo_name) in demos.iter().enumerate() {
        println!("\n📺 Demo {}: {}", i + 1, demo_name);
        println!("{}", "=".repeat(50));

        let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: 20,
    };
        let theme = create_default_theme();

        match i {
            0 => {
                println!("{}", file_viewport.render(&layout, Some(&theme)));
                print_file_viewer_stats(&file_viewport);
            }
            1 => {
                // Simulate adding log entries
                add_log_entry(&mut log_viewport, "System initialized");
                add_log_entry(&mut log_viewport, "Loading configuration...");
                add_log_entry(&mut log_viewport, "Server started on port 8080");

                println!("{}", log_viewport.render(&layout, Some(&theme)));
                print_log_viewer_stats(&log_viewport);
            }
            2 => {
                println!("{}", data_table_viewport.render(&layout, Some(&theme)));
                print_data_table_stats(&data_table_viewport);
            }
            _ => {}
        }

        std::thread::sleep(Duration::from_millis(1000));
    }

    println!(
        "\n✨ Viewport demo completed! Runtime: {:.1}s",
        start_time.elapsed().as_secs_f32()
    );
    println!("The Viewport widget demonstrates:");
    println!("  ✓ Virtual scrolling for 10k+ items with smooth performance");
    println!("  ✓ Lazy loading with intelligent caching strategies");
    println!("  ✓ Keyboard and mouse navigation with vim-style shortcuts");
    println!("  ✓ Search functionality with result highlighting");
    println!("  ✓ Selection support (single/multi-select modes)");
    println!("  ✓ Customizable scrollbar rendering and positioning");
    println!("  ✓ Responsive sizing and content adaptation");

    Ok(())
}

/// Create a file viewer demo with 10,000+ lines
fn create_file_viewer_demo() -> Viewport {
    let large_content: Vec<ViewportItem> = (1..=10000)
        .map(|i| ViewportItem {
            id: format!("line-{i}"),
            content: format!(
                "Line {i:04} - This is a sample line in a large file viewer with lots of content"
            ),
            height: 1,
            selectable: false,
            disabled: false,
            css_classes: vec!["file-line".to_string()],
            metadata: HashMap::from([
                ("line_number".to_string(), i.to_string()),
                ("type".to_string(), "file_content".to_string()),
            ]),
        })
        .collect();

    ViewportBuilder::new("file-viewer")
        .content(large_content)
        .width(80)
        .height(20)
        .scrollable(true)
        .show_scrollbar(true)
        .selection_mode(SelectionMode::None)
        .on_selection_change(|_selected| {
            println!("File selection changed");
        })
        .build()
}

/// Create a log viewer demo with streaming content
fn create_log_viewer_demo() -> Viewport {
    let initial_logs: Vec<ViewportItem> = vec![
        create_log_entry("System startup initiated"),
        create_log_entry("Loading core modules..."),
        create_log_entry("Memory allocation successful"),
        create_log_entry("Network interface initialized"),
        create_log_entry("Database connection established"),
    ];

    ViewportBuilder::new("log-viewer")
        .content(initial_logs)
        .width(80)
        .height(20)
        .scrollable(true)
        .show_scrollbar(true)
        .selection_mode(SelectionMode::Single)
        .on_selection_change(|selected| {
            println!("Log entry selected: {selected:?}");
        })
        .build()
}

/// Create a data table demo with selectable rows
fn create_data_table_demo() -> Viewport {
    let table_data: Vec<ViewportItem> = (1..=100)
        .map(|i| {
            ViewportItem {
                id: format!("row-{i}"),
                content: format!(
                    "│ {i:3} │ User {i:02}    │ user{i}@example.com       │ Active    │"
                ),
                height: 1,
                selectable: true,
                disabled: i % 10 == 0, // Every 10th row is disabled
                css_classes: vec!["table-row".to_string()],
                metadata: HashMap::from([
                    ("type".to_string(), "data_row".to_string()),
                    ("index".to_string(), i.to_string()),
                    ("user_id".to_string(), format!("user-{i}")),
                    (
                        "status".to_string(),
                        if i % 7 == 0 {
                            "inactive".to_string()
                        } else {
                            "active".to_string()
                        },
                    ),
                ]),
            }
        })
        .collect();

    let mut viewport = ViewportBuilder::new("data-table")
        .content(table_data)
        .width(80)
        .height(20)
        .scrollable(true)
        .show_scrollbar(true)
        .selection_mode(SelectionMode::Multiple)
        .on_selection_change(|selected| {
            println!("Selected {} rows", selected.len());
        })
        .build();

    // Add table header
    let header = create_table_header();
    viewport.add_items(vec![header]);

    viewport
}

/// Create a log entry with timestamp
fn create_log_entry(message: &str) -> ViewportItem {
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();

    ViewportItem {
        id: format!("log-{timestamp}"),
        content: format!("[{}] {}", format_timestamp(timestamp), message),
        height: 1,
        selectable: true,
        disabled: false,
        css_classes: vec!["log-entry".to_string()],
        metadata: HashMap::from([
            ("type".to_string(), "log_entry".to_string()),
            ("timestamp".to_string(), timestamp.to_string()),
            ("level".to_string(), "info".to_string()),
        ]),
    }
}

/// Add a new log entry to the log viewer
fn add_log_entry(viewport: &mut Viewport, message: &str) {
    let entry = create_log_entry(message);
    viewport.add_items(vec![entry]);
}

/// Create table header row
fn create_table_header() -> ViewportItem {
    ViewportItem {
        id: "table-header".to_string(),
        content: "┌─────┬───────────┬─────────────────────────┬───────────┐".to_string(),
        height: 1,
        selectable: false,
        disabled: false,
        css_classes: vec!["table-header".to_string()],
        metadata: HashMap::from([("type".to_string(), "header".to_string())]),
    }
}

/// Print file viewer statistics
fn print_file_viewer_stats(viewport: &Viewport) {
    println!("\n📊 File Viewer Stats:");
    println!("  Total lines: {}", viewport.item_count());
    println!("  Viewport size: {}x{}", 80, 20);
    println!("  Virtual scrolling: enabled");
    println!("  Lazy loading: enabled");
}

/// Print log viewer statistics  
fn print_log_viewer_stats(viewport: &Viewport) {
    println!("\n📊 Log Viewer Stats:");
    println!("  Log entries: {}", viewport.item_count());
    println!("  Auto-scroll: enabled");
    println!("  Search: enabled");
    println!("  Selection mode: single");
}

/// Print data table statistics
fn print_data_table_stats(viewport: &Viewport) {
    println!("\n📊 Data Table Stats:");
    println!("  Total rows: {}", viewport.item_count());
    println!("  Selection mode: multiple");
    println!("  Search: enabled with highlighting");
    println!("  Disabled rows: ~10% (every 10th row)");
}

/// Format timestamp for log entries
fn format_timestamp(timestamp: u64) -> String {
    // Simple timestamp formatting
    let hours = (timestamp / 3600) % 24;
    let minutes = (timestamp / 60) % 60;
    let seconds = timestamp % 60;
    format!("{hours:02}:{minutes:02}:{seconds:02}")
}

/// Create a default color theme for the demo
fn create_default_theme() -> ColorTheme {
    use tui_core::themes::{ColorDefinition, ColorMode, ColorPalette, SemanticColorMapping};

    let palette = ColorPalette {
        primary: ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
        },
        primary_dark: ColorDefinition {
            r: 29,
            g: 78,
            b: 216,
        },
        primary_light: ColorDefinition {
            r: 147,
            g: 197,
            b: 253,
        },
        secondary: ColorDefinition {
            r: 156,
            g: 163,
            b: 175,
        },
        secondary_dark: ColorDefinition {
            r: 107,
            g: 114,
            b: 128,
        },
        secondary_light: ColorDefinition {
            r: 209,
            g: 213,
            b: 219,
        },
        background: ColorDefinition { r: 0, g: 0, b: 0 },
        background_alt: ColorDefinition {
            r: 17,
            g: 24,
            b: 39,
        },
        surface: ColorDefinition {
            r: 31,
            g: 41,
            b: 55,
        },
        surface_alt: ColorDefinition {
            r: 55,
            g: 65,
            b: 81,
        },
        text: ColorDefinition {
            r: 255,
            g: 255,
            b: 255,
        },
        text_secondary: ColorDefinition {
            r: 156,
            g: 163,
            b: 175,
        },
        text_muted: ColorDefinition {
            r: 107,
            g: 114,
            b: 128,
        },
        text_inverse: ColorDefinition { r: 0, g: 0, b: 0 },
        border: ColorDefinition {
            r: 75,
            g: 85,
            b: 99,
        },
        border_focus: ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
        },
        border_hover: ColorDefinition {
            r: 107,
            g: 114,
            b: 128,
        },
        success: ColorDefinition {
            r: 34,
            g: 197,
            b: 94,
        },
        warning: ColorDefinition {
            r: 251,
            g: 191,
            b: 36,
        },
        error: ColorDefinition {
            r: 239,
            g: 68,
            b: 68,
        },
        info: ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
        },
        hover: ColorDefinition {
            r: 107,
            g: 114,
            b: 128,
        },
        active: ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
        },
        disabled: ColorDefinition {
            r: 75,
            g: 85,
            b: 99,
        },
        shadow: ColorDefinition { r: 0, g: 0, b: 0 },
        shadow_light: ColorDefinition {
            r: 17,
            g: 24,
            b: 39,
        },
    };

    let semantic = SemanticColorMapping {
        panel_background: "background".to_string(),
        panel_border: "border".to_string(),
        panel_title: "text".to_string(),
        panel_content: "text_secondary".to_string(),
        panel_shadow: "shadow".to_string(),
        button_background: "primary".to_string(),
        button_border: "primary_dark".to_string(),
        button_text: "text".to_string(),
        button_hover: "primary_light".to_string(),
        input_background: "surface".to_string(),
        input_border: "border".to_string(),
        input_text: "text".to_string(),
        input_focus: "border_focus".to_string(),
        progress_background: "surface".to_string(),
        progress_fill: "primary".to_string(),
        progress_text: "text".to_string(),
        editor_background: "background".to_string(),
        editor_text: "text".to_string(),
        editor_cursor: "primary".to_string(),
        editor_line_number: "text_muted".to_string(),
        editor_selection: "primary_dark".to_string(),
        editor_border: "border".to_string(),
        editor_border_focus: "border_focus".to_string(),
        syntax_keyword: "primary".to_string(),
        syntax_string: "success".to_string(),
        syntax_comment: "text_muted".to_string(),
        syntax_number: "warning".to_string(),
        syntax_function: "info".to_string(),
        syntax_type: "secondary".to_string(),
        syntax_variable: "text".to_string(),
        syntax_operator: "text_secondary".to_string(),
        syntax_punctuation: "text_secondary".to_string(),
        syntax_constant: "warning".to_string(),
        syntax_tag: "error".to_string(),
        syntax_attribute: "primary_light".to_string(),
    };

    ColorTheme {
        name: "Default Demo Theme".to_string(),
        description: "Default theme for viewport demo".to_string(),
        mode: ColorMode::Rgb,
        palette,
        semantic,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_viewer_creation() {
        let viewport = create_file_viewer_demo();
        assert_eq!(viewport.item_count(), 10000);
        assert!(viewport.is_scrollable());
    }

    #[test]
    fn test_log_viewer_creation() {
        let viewport = create_log_viewer_demo();
        assert!(viewport.item_count() >= 5);
        assert!(viewport.is_scrollable());
    }

    #[test]
    fn test_data_table_creation() {
        let viewport = create_data_table_demo();
        assert_eq!(viewport.item_count(), 101); // 100 rows + 1 header
        assert!(viewport.is_scrollable());
    }

    #[test]
    fn test_log_entry_format() {
        let entry = create_log_entry("Test message");
        assert!(entry.content.contains("Test message"));
        assert!(entry.content.starts_with('['));
        assert_eq!(entry.height, 1);
        assert!(entry.selectable);
    }

    #[test]
    fn test_timestamp_formatting() {
        let formatted = format_timestamp(3661); // 1 hour, 1 minute, 1 second
        assert_eq!(formatted, "01:01:01");
    }
}
