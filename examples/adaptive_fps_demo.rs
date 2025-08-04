//! Interactive Adaptive FPS Demo - Real-time Performance Monitoring
//!
//! This enhanced demo shows how the TUI framework automatically detects terminal capabilities
//! and adapts FPS to provide optimal performance, with interactive controls for real-time testing.

use reactive_tui::{
  display::AdaptiveFpsManager,
  layout::{ComputedStyles, Layout, LayoutRect},
  rendering::Renderer,
  events::{ActionDispatcher, ActionResult, EventHandler},
};
use std::time::Instant;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Interactive demo showcasing adaptive FPS with real-time controls
struct AdaptiveFpsDemo {
  fps_manager: AdaptiveFpsManager,
  renderer: Renderer,
  frame_count: u64,
  start_time: Instant,
  terminal_size: (u16, u16),
  _event_handler: EventHandler,
  action_dispatcher: ActionDispatcher,
  user_interactions: Arc<Mutex<HashMap<String, u32>>>,
  demo_mode: DemoMode,
  stress_test_active: bool,
  manual_fps_override: Option<u32>,
}

#[derive(Debug, Clone, PartialEq)]
enum DemoMode {
  Automatic,      // Let the system adapt automatically
  Manual,         // User controls FPS manually
  StressTest,     // High load to test adaptation
  PowerSave,      // Low power mode testing
  #[allow(dead_code)]
  Benchmark,      // Performance benchmarking
}

impl AdaptiveFpsDemo {
  /// Create new interactive adaptive FPS demo
  async fn new() -> reactive_tui::error::Result<Self> {
    let fps_manager = AdaptiveFpsManager::new().await?;
    let renderer = Renderer::new()?;
    let terminal_size = crossterm::terminal::size().unwrap_or((400, 200));

    // Create event handling system
    let event_handler = EventHandler::new();
    let mut action_dispatcher = ActionDispatcher::new();
    let user_interactions = Arc::new(Mutex::new(HashMap::new()));

    // Setup interactive controls
    Self::setup_interactive_controls(&mut action_dispatcher, user_interactions.clone());

    Ok(Self {
      fps_manager,
      renderer,
      frame_count: 0,
      start_time: Instant::now(),
      terminal_size,
      _event_handler: event_handler,
      action_dispatcher,
      user_interactions,
      demo_mode: DemoMode::Automatic,
      stress_test_active: false,
      manual_fps_override: None,
    })
  }

  /// Setup interactive controls and action handlers
  fn setup_interactive_controls(
    action_dispatcher: &mut ActionDispatcher,
    interactions: Arc<Mutex<HashMap<String, u32>>>,
  ) {
    let interactions_clone = interactions.clone();
    action_dispatcher.register("fps_increase", move |_action| {
      println!("🔼 User requested FPS increase");
      let mut stats = interactions_clone.lock().unwrap();
      *stats.entry("fps_increase".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone2 = interactions.clone();
    action_dispatcher.register("fps_decrease", move |_action| {
      println!("🔽 User requested FPS decrease");
      let mut stats = interactions_clone2.lock().unwrap();
      *stats.entry("fps_decrease".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone3 = interactions.clone();
    action_dispatcher.register("toggle_stress_test", move |_action| {
      println!("🔥 Stress test toggled");
      let mut stats = interactions_clone3.lock().unwrap();
      *stats.entry("stress_test_toggle".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone4 = interactions.clone();
    action_dispatcher.register("toggle_power_save", move |_action| {
      println!("🔋 Power save mode toggled");
      let mut stats = interactions_clone4.lock().unwrap();
      *stats.entry("power_save_toggle".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone5 = interactions.clone();
    action_dispatcher.register("reset_to_auto", move |_action| {
      println!("🔄 Reset to automatic mode");
      let mut stats = interactions_clone5.lock().unwrap();
      *stats.entry("reset_auto".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });
  }

  /// Create interactive demo layout showing FPS adaptation info and controls
  fn create_demo_layout(&self) -> Layout {
    let capabilities = self.fps_manager.get_capabilities();
    let metrics = self.fps_manager.get_performance_metrics();
    let (width, height) = self.terminal_size;

    // Get user interaction stats
    let interaction_stats = self.user_interactions.lock().unwrap();
    let total_interactions: u32 = interaction_stats.values().sum();

    Layout {
      rect: LayoutRect {
        x: 0,
        y: 0,
        width,
        height,
      },
      tag: "interactive_adaptive_fps_demo".to_string(),
      content: Some(format!(
        "🎯 Interactive Adaptive FPS Demo - Real-time Performance Control\n\
                ━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n\
                \n\
                🎮 Interactive Controls:\n\
                • [+/-] Increase/Decrease FPS manually\n\
                • [S] Toggle stress test mode\n\
                • [P] Toggle power save mode\n\
                • [R] Reset to automatic mode\n\
                • [Q] Quit demo\n\
                \n\
                📊 Current Mode: {:?} {}\n\
                🎯 Manual Override: {}\n\
                🔥 Stress Test: {}\n\
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
                • Auto-Adaptation: {}\n\
                • User Interactions: {} total\n\
                \n\
                🔬 Advanced Features:\n\
                • Real-time FPS adjustment based on performance\n\
                • Interactive stress testing\n\
                • Power-saving mode simulation\n\
                • Manual override capabilities\n\
                • Performance metrics tracking",
        self.demo_mode,
        match self.demo_mode {
          DemoMode::Automatic => "🤖",
          DemoMode::Manual => "👤",
          DemoMode::StressTest => "🔥",
          DemoMode::PowerSave => "🔋",
          DemoMode::Benchmark => "📊",
        },
        self.manual_fps_override.map_or("None".to_string(), |fps| format!("{fps}fps")),
        if self.stress_test_active { "Active 🔥" } else { "Inactive" },
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
        self.manual_fps_override.unwrap_or(self.fps_manager.get_target_fps()),
        metrics.avg_render_time_ms,
        metrics.current_fps,
        if metrics.is_stable {
          "Stable"
        } else {
          "Adjusting"
        },
        self.frame_count,
        self.start_time.elapsed().as_secs_f32(),
        metrics.drop_rate_percent,
        if self.demo_mode == DemoMode::Automatic { "Enabled" } else { "Manual Override" },
        total_interactions
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

  /// Handle user input for interactive controls
  fn handle_user_input(&mut self, key: char) -> bool {
    match key.to_ascii_lowercase() {
      '+' | '=' => {
        self.demo_mode = DemoMode::Manual;
        let current_fps = self.manual_fps_override.unwrap_or(self.fps_manager.get_target_fps());
        self.manual_fps_override = Some((current_fps + 10).min(240));

        let action = self.action_dispatcher.action("fps_increase").build();
        self.action_dispatcher.dispatch(action);
        println!("🔼 FPS increased to: {}fps", self.manual_fps_override.unwrap());
        false
      }
      '-' | '_' => {
        self.demo_mode = DemoMode::Manual;
        let current_fps = self.manual_fps_override.unwrap_or(self.fps_manager.get_target_fps());
        self.manual_fps_override = Some((current_fps.saturating_sub(10)).max(15));

        let action = self.action_dispatcher.action("fps_decrease").build();
        self.action_dispatcher.dispatch(action);
        println!("🔽 FPS decreased to: {}fps", self.manual_fps_override.unwrap());
        false
      }
      's' => {
        self.stress_test_active = !self.stress_test_active;
        self.demo_mode = if self.stress_test_active { DemoMode::StressTest } else { DemoMode::Automatic };

        let action = self.action_dispatcher.action("toggle_stress_test").build();
        self.action_dispatcher.dispatch(action);
        println!("🔥 Stress test: {}", if self.stress_test_active { "ACTIVE" } else { "INACTIVE" });
        false
      }
      'p' => {
        self.demo_mode = if self.demo_mode == DemoMode::PowerSave {
          DemoMode::Automatic
        } else {
          DemoMode::PowerSave
        };

        let action = self.action_dispatcher.action("toggle_power_save").build();
        self.action_dispatcher.dispatch(action);
        println!("🔋 Power save mode: {}", if self.demo_mode == DemoMode::PowerSave { "ON" } else { "OFF" });
        false
      }
      'r' => {
        self.demo_mode = DemoMode::Automatic;
        self.manual_fps_override = None;
        self.stress_test_active = false;

        let action = self.action_dispatcher.action("reset_to_auto").build();
        self.action_dispatcher.dispatch(action);
        println!("🔄 Reset to automatic mode");
        false
      }
      'q' => {
        println!("👋 Exiting demo...");
        true // Exit
      }
      _ => false
    }
  }

  /// Apply current demo mode settings to FPS manager
  fn apply_demo_mode_settings(&mut self) {
    match self.demo_mode {
      DemoMode::PowerSave => {
        // Simulate power save mode with lower FPS
        self.manual_fps_override = Some(30);
      }
      DemoMode::StressTest => {
        // Stress test mode - let adaptive system handle high load
        self.manual_fps_override = None;
      }
      DemoMode::Manual => {
        // Manual mode - use user-specified FPS
        // manual_fps_override is already set by user input
      }
      DemoMode::Automatic => {
        // Automatic mode - let system decide
        self.manual_fps_override = None;
      }
      DemoMode::Benchmark => {
        // Benchmark mode - try maximum performance
        self.manual_fps_override = Some(144);
      }
    }
  }

  /// Create stress test workload to test adaptation
  fn create_stress_test_workload(&self) -> Vec<Layout> {
    if !self.stress_test_active {
      return vec![];
    }

    let mut stress_elements = Vec::new();

    // Create many animated elements to stress the renderer
    for i in 0..50 {
      let x = (i % 10) * 8;
      let y = 25 + (i / 10) * 2;
      let animation_phase = (self.frame_count as f32 * 0.2 + i as f32).sin();

      stress_elements.push(Layout {
        rect: LayoutRect {
          x: x as u16,
          y: y as u16,
          width: 6,
          height: 1,
        },
        tag: format!("stress_{i}"),
        content: Some(format!(
          "🔥{:>3.0}",
          (animation_phase * 100.0).abs()
        )),
        children: vec![],
        focused: false,
        element_id: Some(format!("stress_{i}")),
        focusable: false,
        styles: ComputedStyles::default(),
      });
    }

    stress_elements
  }

  /// Run the interactive adaptive FPS demo
  async fn run_demo(&mut self) -> reactive_tui::error::Result<()> {
    println!("🎯 Starting Interactive Adaptive FPS Demo...");
    println!("{}", self.fps_manager.get_recommendation_summary());
    println!("\n🎮 Interactive Controls:");
    println!("   [+/-] Increase/Decrease FPS manually");
    println!("   [S] Toggle stress test mode");
    println!("   [P] Toggle power save mode");
    println!("   [R] Reset to automatic mode");
    println!("   [Q] Quit demo");
    println!("\nDemo running... (press keys for interactive control)\n");

    // Enable raw mode for key input (simplified simulation)
    let mut should_exit = false;
    let mut last_input_check = Instant::now();

    for frame_num in 0..5000 {  // Extended demo for more interaction time
      let frame_start = Instant::now();

      // Apply current demo mode settings
      self.apply_demo_mode_settings();

      // Simulate user input checking (in real app, this would be async)
      if last_input_check.elapsed().as_millis() > 100 {
        // Simulate some user interactions for demo purposes
        if frame_num == 120 {
          should_exit = self.handle_user_input('s'); // Auto-trigger stress test
        } else if frame_num == 300 {
          should_exit = self.handle_user_input('p'); // Auto-trigger power save
        } else if frame_num == 500 {
          should_exit = self.handle_user_input('r'); // Auto-reset
        }
        last_input_check = Instant::now();
      }

      if should_exit {
        break;
      }

      // Create layout with stress test elements if active
      let render_start = Instant::now();
      let mut layout = self.create_demo_layout();

      // Add stress test workload
      if self.stress_test_active {
        layout.children.extend(self.create_stress_test_workload());
      }

      self.renderer.render(&layout).await?;
      let render_time = render_start.elapsed();

      // Calculate frame metrics with manual override consideration
      let target_fps = self.manual_fps_override.unwrap_or(self.fps_manager.get_target_fps());
      let target_duration = std::time::Duration::from_nanos(1_000_000_000 / target_fps as u64);
      let frame_time = frame_start.elapsed();
      let dropped = frame_time > target_duration;

      // Record performance for adaptive adjustment (only if not in manual mode)
      if self.demo_mode == DemoMode::Automatic || self.demo_mode == DemoMode::StressTest {
        self
          .fps_manager
          .record_frame_performance(frame_time, render_time, dropped);
      }

      self.frame_count += 1;

      // Show live adaptation with mode information
      if self.frame_count % 60 == 0 {
        let metrics = self.fps_manager.get_performance_metrics();
        let current_target = self.manual_fps_override.unwrap_or(self.fps_manager.get_target_fps());
        println!(
          "Frame {}: Mode {:?} | Target {}fps | Actual {:.1}fps | Render {:.2}ms | Drops {:.1}%",
          self.frame_count,
          self.demo_mode,
          current_target,
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

    // Final summary with interaction statistics
    let final_metrics = self.fps_manager.get_performance_metrics();
    let interaction_stats = self.user_interactions.lock().unwrap();

    println!("\n🏁 Interactive Demo Complete!");
    println!(
      "Final FPS: {:.1} | Stability: {} | Mode: {:?}",
      final_metrics.current_fps,
      if final_metrics.is_stable {
        "Excellent"
      } else {
        "Good"
      },
      self.demo_mode
    );

    println!("\n📊 User Interaction Summary:");
    if interaction_stats.is_empty() {
      println!("   No manual interactions (demo ran automatically)");
    } else {
      for (action, count) in interaction_stats.iter() {
        println!("   {action}: {count} times");
      }
    }

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
