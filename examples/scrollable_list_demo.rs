//! ScrollableList Demo - High-Performance Scrolling Widget
//!
//! This demo showcases the ScrollableList widget with:
//! - Efficient rendering for datasets
//! - Keyboard navigation and selection
//! - Search and filtering capabilities  
//! - Multiple selection modes
//! - Custom styling and themes

use std::time::{Duration, Instant};
use tui_core::{
    error::Result,
    layout::{ComputedStyles, Layout, LayoutRect},
    rendering::Renderer,
    widgets::scrollable_list::{ListItem, ScrollableList, SelectionMode},
};

#[derive(Debug, Clone, PartialEq)]
enum Priority {
    Low,
    Medium,
    High,
    Critical,
}

impl Priority {
    fn icon(&self) -> &'static str {
        match self {
            Priority::Low => "🟢",
            Priority::Medium => "🟡",
            Priority::High => "🟠",
            Priority::Critical => "🔴",
        }
    }
}

/// ScrollableList demo application
struct ScrollableListDemo {
    renderer: Renderer,
    list: ScrollableList,
    frame_count: u64,
    start_time: Instant,
    terminal_size: (u16, u16),
}

impl ScrollableListDemo {
    /// Create new demo with sample data
    async fn new() -> Result<Self> {
        let renderer = Renderer::with_adaptive_fps().await?;
        let items = Self::generate_sample_data(50);
        let terminal_size = crossterm::terminal::size().unwrap_or((400, 200));

        let mut list = ScrollableList::builder("demo_list")
            .selection_mode(SelectionMode::Multiple)
            .height(15)
            .search_enabled(true)
            .build();

        list.set_items(items);

        Ok(Self {
            renderer,
            list,
            frame_count: 0,
            start_time: Instant::now(),
            terminal_size,
        })
    }

    /// Generate sample data for demonstration
    fn generate_sample_data(count: usize) -> Vec<ListItem> {
        let categories = [
            "Development",
            "Design",
            "Testing",
            "Documentation",
            "Deployment",
            "Maintenance",
        ];
        let priorities = [
            Priority::Low,
            Priority::Medium,
            Priority::High,
            Priority::Critical,
        ];
        let titles = [
            "Implement authentication system",
            "Design user interface",
            "Write unit tests",
            "Update API documentation",
            "Deploy to production",
            "Fix memory leak",
            "Optimize database queries",
            "Create wireframes",
            "Setup CI/CD pipeline",
            "Refactor legacy code",
            "Add error handling",
            "Improve performance",
            "Update dependencies",
            "Review security",
            "Add monitoring",
        ];
        let descriptions = [
            "Comprehensive implementation with OAuth 2.0 support",
            "Modern, responsive design following UI/UX best practices",
            "Full test coverage including edge cases and error scenarios",
            "Complete API documentation with examples and tutorials",
            "Zero-downtime deployment with rollback capabilities",
            "Memory profiling and leak detection in production environment",
        ];

        (0..count)
            .map(|i| {
                let priority = &priorities[i % priorities.len()];
                let title = format!("{} #{}", titles[i % titles.len()], i + 1);
                let description = descriptions[i % descriptions.len()];
                let category = categories[i % categories.len()];

                ListItem::new(i.to_string(), title)
                    .subtitle(format!("{category} | {description}"))
                    .icon(priority.icon().to_string())
                    .metadata("category", category)
                    .metadata("priority", format!("{priority:?}"))
                    .class("demo-item")
            })
            .collect()
    }

    /// Create demo layout
    fn create_demo_layout(&self) -> Layout {
        let state = self.list.state.get();
        let (width, height) = self.terminal_size;

        Layout {
            rect: LayoutRect { x: 0, y: 0, width, height },
            tag: "scrollable_list_demo".to_string(),
            content: Some(format!(
                "📋 ScrollableList Demo - High-Performance Scrolling Widget\n\
                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                \n\
                Dataset: {} items | Selected: {} items | Position: {}\n\
                Search: '{}' | Visible items: {}\n\
                Runtime: {:.1}s | Frame: {}\n\
                \n\
                🎮 Controls:\n\
                • ↑/↓: Navigate  • Space: Toggle selection  • Ctrl+A: Select all\n\
                • /: Search mode  • Esc: Clear selection  • Enter: Activate item\n\
                \n\
                📊 Widget Features:\n\
                • Efficient scrolling with keyboard navigation\n\
                • Multiple selection mode with visual feedback\n\
                • Search and filtering capabilities\n\
                • Custom icons and styling support\n\
                • Reactive state management\n\
                \n\
                📋 List Contents:",
                state.total_items,
                state.selected_items.len(),
                state.scroll_position + 1,
                state.search_query,
                state.visible_items,
                self.start_time.elapsed().as_secs_f32(),
                self.frame_count
            )),
            children: vec![
                // Main scrollable list area (placeholder - would render actual list)
                Layout {
                    rect: LayoutRect { 
                        x: 2, 
                        y: 16, 
                        width: width.saturating_sub(40).max(40), // Leave space for info panel
                        height: height.saturating_sub(18).max(10) 
                    },
                    tag: "list_area".to_string(),
                    content: Some({
                        // Render the actual scrollable list widget
                        let list_rect = LayoutRect {
                            x: 0,
                            y: 0,
                            width: width.saturating_sub(40).max(40),
                            height: height.saturating_sub(18).max(10)
                        };
                        
                        // Use the actual ScrollableList render method
                        self.list.render(&list_rect, None)
                    }),
                    children: vec![],
                    focused: true,
                    element_id: Some("list_widget".to_string()),
                    focusable: true,
                    styles: ComputedStyles::default(),
                },
                // Information panel
                Layout {
                    rect: LayoutRect { 
                        x: width.saturating_sub(35).max(45), 
                        y: 16, 
                        width: 33.min(width.saturating_sub(2)), 
                        height: height.saturating_sub(18).max(10) 
                    },
                    tag: "info_panel".to_string(),
                    content: Some(format!(
                        "🎯 List Statistics:\n\
                        ┌─────────────────────────────┐\n\
                        │ Total Items: {:>14} │\n\
                        │ Selected: {:>17} │\n\
                        │ Visible: {:>18} │\n\
                        │ Position: {:>17} │\n\
                        │                             │\n\
                        │ 🔧 Configuration:           │\n\
                        │ • Height: {} lines        │\n\
                        │ • Selection: Multiple       │\n\
                        │ • Search: Enabled           │\n\
                        │ • Icons: Enabled            │\n\
                        └─────────────────────────────┘",
                        state.total_items,
                        state.selected_items.len(),
                        state.visible_items,
                        state.scroll_position + 1,
                        self.list.config.height
                    )),
                    children: vec![],
                    focused: false,
                    element_id: Some("info_panel".to_string()),
                    focusable: false,
                    styles: ComputedStyles::default(),
                },
            ],
            focused: false,
            element_id: Some("demo".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        }
    }

    /// Simulate user interactions for demo
    fn simulate_interaction(&mut self) {
        // Navigate through items
        if self.frame_count % 45 == 0 {
            // Move down every 0.75 seconds
            if let Some(current) = self.list.state.get().highlighted_index {
                if current < self.list.items.len() - 1 {
                    let next_id = self.list.items[current + 1].id.clone();
                    self.list.select_item(&next_id);
                }
            } else if !self.list.items.is_empty() {
                let first_id = self.list.items[0].id.clone();
                self.list.select_item(&first_id);
            }
        }

        // Toggle selection occasionally
        if self.frame_count % 120 == 0 {
            // Toggle selection every 2 seconds
            if let Some(current) = self.list.state.get().highlighted_index {
                let item_id = &self.list.items[current].id;
                let mut state = self.list.state.get();
                if state.selected_items.contains(item_id) {
                    state.selected_items.retain(|id| id != item_id);
                } else {
                    state.selected_items.push(item_id.clone());
                }
                // Note: In real usage, this would be handled by the widget's methods
            }
        }

        // Add new items dynamically
        if self.frame_count % 240 == 0 && self.list.items.len() < 100 {
            // Add item every 4 seconds
            let new_item = ListItem::new(
                format!("dynamic_{}", self.list.items.len()),
                format!("Dynamic Item #{}", self.list.items.len() + 1),
            )
            .subtitle("Dynamically added during runtime")
            .icon("⚡")
            .metadata("type", "dynamic")
            .class("dynamic-item");

            self.list.add_item(new_item);
        }
    }

    /// Update demo state
    fn update(&mut self) {
        self.frame_count += 1;
        self.simulate_interaction();
    }

    /// Run the demo
    async fn run_demo(&mut self) -> Result<()> {
        println!("🚀 Starting ScrollableList Demo...");
        println!(
            "📋 {} items loaded with smooth scrolling",
            self.list.items.len()
        );
        println!("🎮 Simulating user interactions...\n");

        for _ in 0..600 {
            // Run for 10 seconds at 60fps
            let frame_start = Instant::now();

            // Update state
            self.update();

            // Render frame
            let layout = self.create_demo_layout();
            self.renderer.render(&layout).await?;

            // Progress reporting
            if self.frame_count % 60 == 0 {
                let state = self.list.state.get();
                println!(
                    "Frame {}: Items: {} | Selected: {} | Position: {}",
                    self.frame_count,
                    state.total_items,
                    state.selected_items.len(),
                    state.scroll_position + 1
                );
            }

            // Frame timing (60fps)
            let elapsed = frame_start.elapsed();
            let target_duration = Duration::from_millis(16);
            if elapsed < target_duration {
                tokio::time::sleep(target_duration - elapsed).await;
            }
        }

        let final_state = self.list.state.get();
        println!("\n🏁 ScrollableList Demo Complete!");
        println!("📊 Final Statistics:");
        println!("• Total Items: {}", final_state.total_items);
        println!("• Selected Items: {}", final_state.selected_items.len());
        println!("• Final Position: {}", final_state.scroll_position + 1);
        println!("• Runtime: {:.1}s", self.start_time.elapsed().as_secs_f32());

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut demo = ScrollableListDemo::new().await?;
    demo.run_demo().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_scrollable_list_demo_creation() {
        let demo = ScrollableListDemo::new().await;
        assert!(demo.is_ok());

        let demo = demo.unwrap();
        assert_eq!(demo.list.items.len(), 50);
    }

    #[test]
    fn test_sample_data_generation() {
        let items = ScrollableListDemo::generate_sample_data(10);
        assert_eq!(items.len(), 10);
        assert!(items.iter().all(|item| !item.text.is_empty()));
        assert!(items.iter().all(|item| item.subtitle.is_some()));
        assert!(items.iter().all(|item| item.icon.is_some()));
    }

    #[test]
    fn test_priority_icons() {
        assert_eq!(Priority::Low.icon(), "🟢");
        assert_eq!(Priority::Medium.icon(), "🟡");
        assert_eq!(Priority::High.icon(), "🟠");
        assert_eq!(Priority::Critical.icon(), "🔴");
    }
}
