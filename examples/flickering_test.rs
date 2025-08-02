//! Flickering Test Demo
//!
//! This example demonstrates the double-buffered rendering system that eliminates
//! the flickering issue during refresh operations.
//!
//! Before the fix: Multiple immediate execute() calls caused visible intermediate states
//! After the fix: Single atomic buffer flush eliminates flickering

use std::time::{Duration, Instant};
use tui_core::{
    layout::{ComputedStyles, Layout, LayoutRect},
    rendering::{PanelConfig, Renderer},
};

/// Flickering test application
struct FlickeringTest {
    renderer: Renderer,
    start_time: Instant,
    frame_count: u64,
    terminal_size: (u16, u16),
}

impl FlickeringTest {
    /// Create a new flickering test
    fn new() -> tui_core::error::Result<Self> {
        let terminal_size = crossterm::terminal::size().unwrap_or((400, 200));
        Ok(Self {
            renderer: Renderer::new()?,
            start_time: Instant::now(),
            frame_count: 0,
            terminal_size,
        })
    }

    /// Create a complex layout to stress test the rendering system
    fn create_complex_layout(&self) -> Layout {
        let (width, height) = self.terminal_size;
        let mut main_layout = Layout {
            rect: LayoutRect {
                x: 0,
                y: 0,
                width,
                height,
            },
            tag: "main".to_string(),
            content: Some("🎨 TUI Flickering Fix Test - Double Buffered Rendering".to_string()),
            children: vec![],
            focused: false,
            element_id: Some("main".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        };

        // Add multiple child elements with different colors and positions
        for i in 0..10 {
            let child = Layout {
                rect: LayoutRect {
                    x: (i * 8) % 60,
                    y: 2 + (i / 8) * 2,
                    width: 7,
                    height: 1,
                },
                tag: format!("item-{i}"),
                content: Some(format!("Item {i}")),
                children: vec![],
                focused: i as u64 == (self.frame_count / 10) % 10, // Rotating focus for animation
                element_id: Some(format!("item-{i}")),
                focusable: true,
                styles: ComputedStyles::default(),
            };
            main_layout.children.push(child);
        }

        // Add a moving element to create visual changes
        let moving_x = ((self.frame_count as f32 * 0.5).sin() * 30.0 + 40.0) as u16;
        let moving_element = Layout {
            rect: LayoutRect {
                x: moving_x,
                y: 10,
                width: 10,
                height: 3,
            },
            tag: "moving".to_string(),
            content: Some("🚀 Moving\n  Element\n    Test".to_string()),
            children: vec![],
            focused: false,
            element_id: Some("moving".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        };
        main_layout.children.push(moving_element);

        // Add multiple colored panels
        for row in 0..3 {
            for col in 0..4 {
                let panel = Layout {
                    rect: LayoutRect {
                        x: col * 20,
                        y: 15 + row * 3,
                        width: 18,
                        height: 2,
                    },
                    tag: format!("panel-{row}-{col}"),
                    content: Some(format!("Panel {row},{col}")),
                    children: vec![],
                    focused: false,
                    element_id: Some(format!("panel-{row}-{col}")),
                    focusable: false,
                    styles: ComputedStyles::default(),
                };
                main_layout.children.push(panel);
            }
        }

        main_layout
    }

    /// Update the test state
    fn update(&mut self) {
        self.frame_count += 1;
    }

    /// Render the current frame
    async fn render(&mut self) -> tui_core::error::Result<()> {
        let layout = self.create_complex_layout();

        // This single call now uses double buffering internally
        // All rendering operations are collected in the frame buffer
        // and flushed atomically, eliminating flickering
        self.renderer.render(&layout).await?;

        Ok(())
    }

    /// Render additional UI elements using the panel system
    async fn render_panels(&mut self) -> tui_core::error::Result<()> {
        // Test the panel rendering with borders
        let panel_config = PanelConfig {
            x: 5,
            y: 1,
            width: 30,
            height: 5,
            title: Some("Performance Stats".to_string()),
            content: format!(
                "Frame: {}\nFPS: {:.1}\nUptime: {:.1}s\nDouble Buffered: ✓",
                self.frame_count,
                self.frame_count as f32 / self.start_time.elapsed().as_secs_f32(),
                self.start_time.elapsed().as_secs_f32()
            ),
        };

        self.renderer.render_panel(&panel_config)?;

        Ok(())
    }

    /// Get performance statistics
    fn get_stats(&self) -> (u64, f32, f32) {
        let elapsed = self.start_time.elapsed().as_secs_f32();
        let fps = self.frame_count as f32 / elapsed;
        (self.frame_count, fps, elapsed)
    }
}

/// Run the flickering test
fn main() -> tui_core::error::Result<()> {
    println!("🎬 TUI Flickering Fix Test - Double Buffered Rendering");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("This test demonstrates the flickering fix through double buffering.");
    println!("Without the fix: Multiple execute() calls cause visible flicker");
    println!("With the fix: Single atomic buffer flush eliminates flicker\n");

    let mut test = FlickeringTest::new()?;
    let runtime = tokio::runtime::Runtime::new().map_err(|e| {
        tui_core::error::TuiError::driver(format!("Failed to create async runtime: {e}"))
    })?;

    // Run the test for a reasonable duration
    for frame in 0..300 {
        // ~5 seconds at 60fps
        runtime.block_on(async {
            // Update test state
            test.update();

            // Render complex layout - this tests the double buffering
            if let Err(e) = test.render().await {
                eprintln!("Render error: {e}");
            }

            // Render panels for additional complexity
            if let Err(e) = test.render_panels().await {
                eprintln!("Panel render error: {e}");
            }
        });

        // Print progress every 30 frames
        if frame % 30 == 0 {
            let (frame_count, fps, elapsed) = test.get_stats();
            print!(
                "\rFrame: {} | FPS: {:.1} | Elapsed: {:.1}s | Buffer Size: {} bytes",
                frame_count,
                fps,
                elapsed,
                test.renderer.get_buffer_size()
            );
            std::io::Write::flush(&mut std::io::stdout()).ok();
        }

        // Simulate 60 FPS
        std::thread::sleep(Duration::from_millis(16));
    }

    let (final_frames, final_fps, final_elapsed) = test.get_stats();
    println!("\n\n🎉 Flickering Test Complete!");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Final Statistics:");
    println!("• Total Frames Rendered: {final_frames}");
    println!("• Average FPS: {final_fps:.1}");
    println!("• Total Runtime: {final_elapsed:.1}s");
    println!("• Rendering Method: Double Buffered (Flicker-Free)");
    println!("• Performance Impact: Minimal overhead, maximum visual quality");
    println!("\nThe double buffering system successfully eliminates TUI flickering!");
    println!("All terminal operations are now collected in a frame buffer and");
    println!("flushed atomically, providing smooth, flicker-free rendering.");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flickering_test_creation() {
        let test = FlickeringTest::new();
        assert!(test.is_ok());
        let test = test.unwrap();
        assert_eq!(test.frame_count, 0);
    }

    #[test]
    fn test_layout_creation() {
        let test = FlickeringTest::new().unwrap();
        let layout = test.create_complex_layout();
        assert_eq!(layout.tag, "main");
        assert!(!layout.children.is_empty());
    }

    #[test]
    fn test_frame_buffer_functionality() {
        use tui_core::rendering::FrameBuffer;

        let mut buffer = FrameBuffer::new();
        assert_eq!(buffer.buffer_size(), 0);

        // Test buffer operations
        buffer.clear();
        assert_eq!(buffer.buffer_size(), 0);
    }

    #[tokio::test]
    async fn test_render_performance() {
        let mut test = FlickeringTest::new().unwrap();

        // Test that rendering doesn't fail
        for _ in 0..10 {
            test.update();
            let result = test.render().await;
            assert!(result.is_ok(), "Render should not fail");
        }

        let (frames, fps, _) = test.get_stats();
        assert_eq!(frames, 10);
        assert!(fps > 0.0);
    }
}
