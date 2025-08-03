//! Adaptive FPS Demo - Monitor Capability Detection
//!
//! This demo shows how the TUI framework automatically detects terminal capabilities
//! and adapts FPS to provide optimal performance without direct monitor access.

use reactive_tui::{
  display::AdaptiveFpsManager,
  layout::{ComputedStyles, Layout, LayoutRect},
  rendering::Renderer,
};
use std::time::Instant;

/// Demo showcasing adaptive FPS based on terminal capabilities
struct AdaptiveFpsDemo {
  fps_manager: AdaptiveFpsManager,
  renderer: Renderer,
  frame_count: u64,
  start_time: Instant,
  terminal_size: (u16, u16),
}

impl AdaptiveFpsDemo {
  /// Create new adaptive FPS demo
  async fn new() -> reactive_tui::error::Result<Self> {
    let fps_manager = AdaptiveFpsManager::new().await?;
    let renderer = Renderer::new()?;
    let terminal_size = crossterm::terminal::size().unwrap_or((400, 200));

    Ok(Self {
      fps_manager,
      renderer,
      frame_count: 0,
      start_time: Instant::now(),
      terminal_size,
    })
  }

  /// Create demo layout showing FPS adaptation info
  fn create_demo_layout(&self) -> Layout {
    let capabilities = self.fps_manager.get_capabilities();
    let metrics = self.fps_manager.get_performance_metrics();
    let (width, height) = self.terminal_size;

    Layout {
      rect: LayoutRect {
        x: 0,
        y: 0,
        width,
        height,
      },
      tag: "adaptive_fps_demo".to_string(),
      content: Some(format!(
        "🎯 Adaptive FPS Demo - Monitor Capability Detection\n\
                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                \n\
                Terminal Detection:\n\
                • Program: {}\n\
                • Connection: {:?}\n\
                • Color Depth: {:?}\n\
                • GPU Acceleration: {}\n\
                • High Refresh Support: {}\n\
                \n\
                Performance Capabilities:\n\
                • Max FPS: {} (detected)\n\
                • Recommended FPS: {} (optimized)\n\
                • Current Target: {} (adaptive)\n\
                • Avg Render Time: {:.2}ms\n\
                • Current FPS: {:.1}\n\
                • Frame Stability: {}\n\
                \n\
                Live Metrics:\n\
                • Frame: {} | Uptime: {:.1}s\n\
                • Drop Rate: {:.1}%\n\
                • Auto-Adaptation: Enabled\n\
                \n\
                How It Works Without Monitor Access:\n\
                • Environment Variables: TERM_PROGRAM, SSH_CLIENT, COLORTERM\n\
                • Performance Benchmarking: Real-time render time measurement\n\
                • Terminal Heuristics: Known terminal capabilities database\n\
                • Connection Analysis: Local vs SSH vs Web detection\n\
                • Adaptive Adjustment: Real-time FPS optimization",
        capabilities
          .terminal_info
          .program
          .as_deref()
          .unwrap_or("Unknown"),
        capabilities.terminal_info.connection_type,
        capabilities.terminal_info.color_depth,
        if capabilities.terminal_info.has_gpu_acceleration {
          "Yes"
        } else {
          "No"
        },
        if capabilities.terminal_info.supports_high_refresh {
          "Yes"
        } else {
          "No"
        },
        capabilities.max_fps,
        capabilities.recommended_fps,
        self.fps_manager.get_target_fps(),
        metrics.avg_render_time_ms,
        metrics.current_fps,
        if metrics.is_stable {
          "Stable"
        } else {
          "Adjusting"
        },
        self.frame_count,
        self.start_time.elapsed().as_secs_f32(),
        metrics.drop_rate_percent
      )),
      children: self.create_animated_elements(),
      focused: false,
      element_id: Some("demo".to_string()),
      focusable: false,
      styles: ComputedStyles::default(),
    }
  }

  /// Create animated elements to demonstrate performance
  fn create_animated_elements(&self) -> Vec<Layout> {
    let mut elements = Vec::new();

    // Add animated bars showing different terminal scenarios
    let scenarios = [
      ("Local Terminal", 144, "🟢"),
      ("SSH Connection", 60, "🟡"),
      ("Web Terminal", 30, "🔴"),
      ("GPU Accelerated", 240, "🚀"),
    ];

    for (i, (name, max_fps, icon)) in scenarios.iter().enumerate() {
      let bar_width = ((*max_fps as f32 / 240.0) * 40.0) as u16;
      let animated_width = ((self.frame_count as f32 * 0.1).sin().abs() * bar_width as f32) as u16;

      elements.push(Layout {
        rect: LayoutRect {
          x: 20,
          y: 20 + i as u16 * 2,
          width: 60,
          height: 1,
        },
        tag: format!("scenario_{i}"),
        content: Some(format!(
          "{} {} [{:width$}] {}fps",
          icon,
          name,
          "█".repeat(animated_width as usize),
          max_fps,
          width = 20
        )),
        children: vec![],
        focused: false,
        element_id: Some(format!("scenario_{i}")),
        focusable: false,
        styles: ComputedStyles::default(),
      });
    }

    elements
  }

  /// Run the adaptive FPS demo
  async fn run_demo(&mut self) -> reactive_tui::error::Result<()> {
    println!("🎯 Starting Adaptive FPS Demo...");
    println!("{}", self.fps_manager.get_recommendation_summary());
    println!("\nPress Ctrl+C to exit\n");

    for _ in 0..1000 {
      // Run for extended period to show adaptation
      let frame_start = Instant::now();

      // Render frame
      let render_start = Instant::now();
      let layout = self.create_demo_layout();
      self.renderer.render(&layout).await?;
      let render_time = render_start.elapsed();

      // Calculate frame metrics
      let target_duration = self.fps_manager.get_frame_duration();
      let frame_time = frame_start.elapsed();
      let dropped = frame_time > target_duration;

      // Record performance for adaptive adjustment
      self
        .fps_manager
        .record_frame_performance(frame_time, render_time, dropped);

      self.frame_count += 1;

      // Show live adaptation
      if self.frame_count % 60 == 0 {
        let metrics = self.fps_manager.get_performance_metrics();
        println!(
          "Frame {}: Target {}fps | Actual {:.1}fps | Render {:.2}ms | Drops {:.1}%",
          self.frame_count,
          self.fps_manager.get_target_fps(),
          metrics.current_fps,
          metrics.avg_render_time_ms,
          metrics.drop_rate_percent
        );
      }

      // Frame timing
      let elapsed = frame_start.elapsed();
      if elapsed < target_duration {
        tokio::time::sleep(target_duration - elapsed).await;
      }
    }

    // Final summary
    let final_metrics = self.fps_manager.get_performance_metrics();
    println!("\n🏁 Demo Complete!");
    println!(
      "Final FPS: {:.1} | Stability: {}",
      final_metrics.current_fps,
      if final_metrics.is_stable {
        "Excellent"
      } else {
        "Good"
      }
    );

    Ok(())
  }
}

/// Show how different terminals are detected and handled
fn demonstrate_terminal_detection() {
  println!("🔍 Terminal Detection Methods (Without Monitor Access):");
  println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

  // Show actual environment detection
  println!("\n📊 Current Environment:");
  if let Ok(term_program) = std::env::var("TERM_PROGRAM") {
    println!("• TERM_PROGRAM: {term_program}");
  }
  if let Ok(term) = std::env::var("TERM") {
    println!("• TERM: {term}");
  }
  if let Ok(colorterm) = std::env::var("COLORTERM") {
    println!("• COLORTERM: {colorterm}");
  }

  // Show connection detection
  println!("\n🌐 Connection Detection:");
  if std::env::var("SSH_CLIENT").is_ok() {
    println!("• SSH connection detected");
  }
  if std::env::var("TMUX").is_ok() {
    println!("• TMUX session detected");
  }

  println!("\n🎯 FPS Recommendations by Terminal Type:");
  let terminal_types = [
    ("iTerm2", "144fps", "GPU accelerated, excellent performance"),
    ("WezTerm", "144fps", "Rust-based, GPU accelerated"),
    ("Alacritty", "120fps", "High performance renderer"),
    ("Windows Terminal", "90fps", "Modern Windows terminal"),
    ("SSH Terminal", "60fps", "Network latency limited"),
    ("Web Terminal", "30fps", "Browser performance constraints"),
    ("Standard Terminal", "60fps", "Conservative default"),
  ];

  for (terminal, fps, note) in &terminal_types {
    println!("• {terminal:<16} → {fps:<6} ({note})");
  }

  println!("\n💡 Key Insight: We use intelligent heuristics instead of direct monitor queries!");
  println!("   This provides better practical performance than raw monitor specs.");
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
  demonstrate_terminal_detection();

  println!("\n{}", "=".repeat(80));

  let mut demo = AdaptiveFpsDemo::new().await?;
  demo.run_demo().await
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_adaptive_fps_demo_creation() {
    let demo = AdaptiveFpsDemo::new().await;
    assert!(demo.is_ok());
  }

  #[test]
  fn test_terminal_detection() {
    // Just ensure it doesn't panic
    demonstrate_terminal_detection();
  }
}
