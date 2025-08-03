//! Animation System Demo
//!
//! This example demonstrates the comprehensive animation system with various easing functions,
//! property animations, and timeline management for smooth TUI animations.
//!
//! Features demonstrated:
//! - Multiple easing functions (bounce, elastic, back, exponential)
//! - Property animations (opacity, position, color, scale)
//! - Animation timelines and sequencing
//! - Parallel and sequential animations
//! - Animation state management and callbacks

use reactive_tui::{
  themes::ColorDefinition,
  widgets::{
    bounce, fade_in, fade_out, pulse, slide_in_left, AnimatedProperty, AnimationBuilder,
    AnimationManager, AnimationTimeline, LoopMode, TweenEasing,
  },
};
use std::time::{Duration, Instant};

/// Main demo application
struct AnimationDemo {
  /// Animation manager for coordinating multiple animations
  animation_manager: AnimationManager,
  /// Start time for demo timing
  start_time: Instant,
  /// Demo phase tracking
  current_phase: usize,
  /// Phase descriptions
  phases: Vec<&'static str>,
}

impl AnimationDemo {
  /// Create a new animation demo
  fn new() -> Self {
    let mut demo = Self {
      animation_manager: AnimationManager::new(),
      start_time: Instant::now(),
      current_phase: 0,
      phases: vec![
        "Phase 1: Basic Fade Animations",
        "Phase 2: Easing Function Showcase",
        "Phase 3: Property Animations",
        "Phase 4: Timeline Sequences",
        "Phase 5: Complex Compositions",
      ],
    };

    demo.setup_animations();
    demo
  }

  /// Setup all demonstration animations
  fn setup_animations(&mut self) {
    // Phase 1: Basic fade animations
    self.create_basic_fade_animations();

    // Phase 2: Easing function showcase
    self.create_easing_showcase();

    // Phase 3: Property animations
    self.create_property_animations();

    // Phase 4: Timeline sequences
    self.create_timeline_sequences();

    // Phase 5: Complex compositions
    self.create_complex_compositions();
  }

  /// Create basic fade in/out animations
  fn create_basic_fade_animations(&mut self) {
    // Simple fade in
    let fade_in_anim = fade_in("fade-in-demo", Duration::from_millis(1000));
    self.animation_manager.add_animation(fade_in_anim);

    // Fade out with callback
    let fade_out_anim = AnimationBuilder::new("fade-out-demo")
      .animate_property(AnimatedProperty::Opacity(1.0, 0.0))
      .duration(Duration::from_millis(1500))
      .easing(TweenEasing::EaseOut)
      .delay(Duration::from_millis(2000))
      .on_complete(|animation| {
        println!("Fade out completed: {}", animation.id);
      })
      .build();
    self.animation_manager.add_animation(fade_out_anim);
  }

  /// Create showcase of different easing functions
  fn create_easing_showcase(&mut self) {
    let easing_functions = vec![
      ("linear", TweenEasing::Linear),
      ("ease-in", TweenEasing::EaseIn),
      ("ease-out", TweenEasing::EaseOut),
      ("ease-in-out", TweenEasing::EaseInOut),
      ("bounce", TweenEasing::Bounce),
      ("elastic", TweenEasing::Elastic),
      ("back", TweenEasing::Back),
      ("exponential", TweenEasing::Expo),
      ("circular", TweenEasing::Circ),
      ("sine", TweenEasing::Sine),
    ];

    for (i, (name, easing)) in easing_functions.iter().enumerate() {
      let animation = AnimationBuilder::new(format!("easing-{name}"))
        .animate_property(AnimatedProperty::Position(
          0,
          (i * 3) as i16,
          50,
          (i * 3) as i16,
        ))
        .duration(Duration::from_millis(2000))
        .easing(*easing)
        .delay(Duration::from_millis(500 + i as u64 * 200))
        .loop_mode(LoopMode::PingPong)
        .on_update(|_animation, _values| {
          // Animation update callback for real-time visualization
        })
        .build();

      self.animation_manager.add_animation(animation);
    }
  }

  /// Create various property animations
  fn create_property_animations(&mut self) {
    // Color transition animation
    let color_anim = AnimationBuilder::new("color-transition")
      .animate_property(AnimatedProperty::Color(
        ColorDefinition { r: 255, g: 0, b: 0 }, // Red
        ColorDefinition {
          r: 0,
          g: 255,
          b: 255,
        }, // Cyan
      ))
      .duration(Duration::from_millis(3000))
      .easing(TweenEasing::EaseInOut)
      .loop_mode(LoopMode::PingPong)
      .build();
    self.animation_manager.add_animation(color_anim);

    // Scale animation
    let scale_anim = AnimationBuilder::new("scale-animation")
      .animate_property(AnimatedProperty::Scale(1.0, 1.5))
      .duration(Duration::from_millis(1000))
      .easing(TweenEasing::Back)
      .loop_mode(LoopMode::PingPong)
      .build();
    self.animation_manager.add_animation(scale_anim);

    // Rotation animation
    let rotation_anim = AnimationBuilder::new("rotation-animation")
      .animate_property(AnimatedProperty::Rotation(0.0, 360.0))
      .duration(Duration::from_millis(4000))
      .easing(TweenEasing::Linear)
      .loop_mode(LoopMode::Infinite)
      .build();
    self.animation_manager.add_animation(rotation_anim);

    // Multiple properties animation
    let multi_anim = AnimationBuilder::new("multi-property")
      .animate_property(AnimatedProperty::Multiple(vec![
        AnimatedProperty::Opacity(0.5, 1.0),
        AnimatedProperty::Scale(0.8, 1.2),
        AnimatedProperty::Position(10, 10, 30, 30),
      ]))
      .duration(Duration::from_millis(2500))
      .easing(TweenEasing::Bounce)
      .build();
    self.animation_manager.add_animation(multi_anim);
  }

  /// Create timeline-based sequential animations
  fn create_timeline_sequences(&mut self) {
    // Sequential timeline
    let mut sequential_timeline = AnimationTimeline::new("sequential-demo", true);

    sequential_timeline.add_animation(fade_in("seq-1", Duration::from_millis(500)));
    sequential_timeline.add_animation(slide_in_left(
      "seq-2",
      -20,
      0,
      10,
      Duration::from_millis(750),
    ));
    sequential_timeline.add_animation(bounce("seq-3", Duration::from_millis(1000)));
    sequential_timeline.add_animation(fade_out("seq-4", Duration::from_millis(500)));

    self.animation_manager.add_timeline(sequential_timeline);

    // Parallel timeline
    let mut parallel_timeline = AnimationTimeline::new("parallel-demo", false);

    parallel_timeline.add_animation(pulse("par-1", Duration::from_millis(2000)));
    parallel_timeline.add_animation(
      AnimationBuilder::new("par-2")
        .animate_property(AnimatedProperty::Color(
          ColorDefinition { r: 0, g: 255, b: 0 },
          ColorDefinition {
            r: 255,
            g: 0,
            b: 255,
          },
        ))
        .duration(Duration::from_millis(2000))
        .easing(TweenEasing::Sine)
        .build(),
    );
    parallel_timeline.add_animation(
      AnimationBuilder::new("par-3")
        .animate_property(AnimatedProperty::Rotation(0.0, 180.0))
        .duration(Duration::from_millis(2000))
        .easing(TweenEasing::EaseInOut)
        .build(),
    );

    self.animation_manager.add_timeline(parallel_timeline);
  }

  /// Create complex animation compositions with advanced effects
  fn create_complex_compositions(&mut self) {
    // Staggered entrance animation
    for i in 0..8 {
      let stagger_anim = AnimationBuilder::new(format!("stagger-{i}"))
        .animate_property(AnimatedProperty::Multiple(vec![
          AnimatedProperty::Opacity(0.0, 1.0),
          AnimatedProperty::Position(-10, i * 4, 0, i * 4),
          AnimatedProperty::Scale(0.5, 1.0),
        ]))
        .duration(Duration::from_millis(800))
        .easing(TweenEasing::Back)
        .delay(Duration::from_millis(i as u64 * 100))
        .build();

      self.animation_manager.add_animation(stagger_anim);
    }

    // Wave effect animation
    let wave_anim = AnimationBuilder::new("wave-effect")
      .animate_property(AnimatedProperty::Custom(
        "wave_phase".to_string(),
        0.0,
        std::f32::consts::TAU,
      ))
      .duration(Duration::from_millis(3000))
      .easing(TweenEasing::Sine)
      .loop_mode(LoopMode::Infinite)
      .on_update(|_animation, _values| {
        // Custom wave calculation would be applied here
      })
      .build();
    self.animation_manager.add_animation(wave_anim);

    // Elastic bounce sequence
    let elastic_sequence = AnimationBuilder::new("elastic-sequence")
      .animate_property(AnimatedProperty::Multiple(vec![
        AnimatedProperty::Position(40, 20, 40, 5),
        AnimatedProperty::Scale(1.0, 0.8),
      ]))
      .duration(Duration::from_millis(1200))
      .easing(TweenEasing::Elastic)
      .loop_mode(LoopMode::Count(3))
      .auto_reverse(true)
      .on_loop(|_animation, loop_count| {
        println!("Elastic sequence loop {loop_count} completed");
      })
      .build();
    self.animation_manager.add_animation(elastic_sequence);
  }

  /// Update the demo - call this in your main loop
  fn update(&mut self, _delta_time: Duration) {
    // Update all animations
    self.animation_manager.update();

    // Phase management
    let elapsed = self.start_time.elapsed();
    let phase_duration = Duration::from_secs(6);
    let new_phase = (elapsed.as_secs() / phase_duration.as_secs()) as usize % self.phases.len();

    if new_phase != self.current_phase {
      self.current_phase = new_phase;
      println!("Entering: {}", self.phases[self.current_phase]);

      // Trigger phase-specific animations
      self.trigger_phase_animations(self.current_phase);
    }
  }

  /// Trigger animations for specific demo phase
  fn trigger_phase_animations(&mut self, phase: usize) {
    match phase {
      0 => {
        // Start basic fade animations
        if let Some(animation) = self.animation_manager.get_animation_mut("fade-in-demo") {
          animation.play();
        }
      }
      1 => {
        // Start easing showcase
        for i in 0..10 {
          let easing_names = [
            "linear",
            "ease-in",
            "ease-out",
            "ease-in-out",
            "bounce",
            "elastic",
            "back",
            "exponential",
            "circular",
            "sine",
          ];
          if i < easing_names.len() {
            if let Some(animation) = self
              .animation_manager
              .get_animation_mut(&format!("easing-{}", easing_names[i]))
            {
              animation.play();
            }
          }
        }
      }
      2 => {
        // Start property animations
        for anim_id in [
          "color-transition",
          "scale-animation",
          "rotation-animation",
          "multi-property",
        ] {
          if let Some(animation) = self.animation_manager.get_animation_mut(anim_id) {
            animation.play();
          }
        }
      }
      3 => {
        // Start timeline sequences
        // Timelines would be triggered through the timeline system
      }
      4 => {
        // Start complex compositions
        for i in 0..8 {
          if let Some(animation) = self
            .animation_manager
            .get_animation_mut(&format!("stagger-{i}"))
          {
            animation.play();
          }
        }
        if let Some(animation) = self.animation_manager.get_animation_mut("wave-effect") {
          animation.play();
        }
        if let Some(animation) = self.animation_manager.get_animation_mut("elastic-sequence") {
          animation.play();
        }
      }
      _ => {}
    }
  }

  /// Render the current demo state
  fn render(&self) -> String {
    let mut output = String::new();

    // Header
    output.push_str("🎬 TUI Animation System Demo\n");
    output
      .push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
    output.push_str(&format!(
      "Current: {} | Active Animations: {}\n\n",
      self.phases[self.current_phase],
      self.animation_manager.active_count()
    ));

    // Demo status
    output.push_str("📊 Animation Statistics:\n");
    output.push_str(&format!(
      "• Total Active: {}\n",
      self.animation_manager.active_count()
    ));
    output.push_str(&format!(
      "• Current Phase: {}/{}\n",
      self.current_phase + 1,
      self.phases.len()
    ));
    output.push_str(&format!(
      "• Runtime: {:.1}s\n\n",
      self.start_time.elapsed().as_secs_f32()
    ));

    // Feature showcase
    output.push_str("🎯 Features Demonstrated:\n");
    output.push_str("• 15+ Easing Functions (Linear, Bounce, Elastic, Back, Exponential...)\n");
    output.push_str("• Property Animations (Opacity, Position, Size, Color, Scale, Rotation)\n");
    output.push_str("• Timeline Management (Sequential & Parallel)\n");
    output.push_str("• Animation States (Play, Pause, Stop, Reverse, Loop)\n");
    output.push_str("• Callback System (Start, Update, Complete, Loop events)\n");
    output.push_str("• Performance Optimization (Frame-based timing, Efficient interpolation)\n\n");

    // Current phase details
    output.push_str("📈 Current Phase Details:\n");
    match self.current_phase {
      0 => output.push_str("Basic fade in/out animations with different durations and delays"),
      1 => output.push_str("Showcase of 10 different easing functions with position animations"),
      2 => {
        output.push_str("Property animations: color transitions, scaling, rotation, multi-property")
      }
      3 => output.push_str("Timeline sequences: sequential and parallel animation coordination"),
      4 => output
        .push_str("Complex compositions: staggered entrances, wave effects, elastic sequences"),
      _ => output.push_str("Animation cycle complete - restarting demonstration"),
    }
    output.push_str("\n\n");

    // Usage example
    output.push_str("💡 Usage Example:\n");
    output.push_str("```rust\n");
    output.push_str("let animation = AnimationBuilder::new(\"my-animation\")\n");
    output.push_str("    .animate_property(AnimatedProperty::Opacity(0.0, 1.0))\n");
    output.push_str("    .duration(Duration::from_millis(1000))\n");
    output.push_str("    .easing(TweenEasing::Bounce)\n");
    output.push_str("    .on_complete(|anim| println!(\"Done!\"))\n");
    output.push_str("    .build();\n");
    output.push_str("animation.play();\n");
    output.push_str("```\n");

    output
  }
}

/// Run the animation demo
fn main() {
  println!("Starting TUI Animation System Demo...\n");

  let mut demo = AnimationDemo::new();
  let mut last_update = Instant::now();

  // Simulate main loop for demonstration
  for frame in 0..1000 {
    let now = Instant::now();
    let delta_time = now.duration_since(last_update);
    last_update = now;

    // Update animations
    demo.update(delta_time);

    // Render every 10th frame to avoid spam
    if frame % 10 == 0 {
      print!("\x1B[2J\x1B[1;1H"); // Clear screen
      println!("{}", demo.render());
    }

    // Simulate 60 FPS
    std::thread::sleep(Duration::from_millis(16));

    // Exit after reasonable demo time
    if demo.start_time.elapsed() > Duration::from_secs(30) {
      break;
    }
  }

  println!("\n🎉 Animation Demo Complete!");
  println!("The animation system provides comprehensive support for:");
  println!("• Smooth property transitions with 15+ easing functions");
  println!("• Timeline management for complex sequences");
  println!("• Performance-optimized frame-based timing");
  println!("• Rich callback system for animation lifecycle events");
  println!("• Full dual-language implementation (Rust + TypeScript)");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_animation_demo_creation() {
    let demo = AnimationDemo::new();
    assert_eq!(demo.current_phase, 0);
    assert_eq!(demo.phases.len(), 5);
    assert!(demo.animation_manager.active_count() > 0);
  }

  #[test]
  fn test_phase_management() {
    let mut demo = AnimationDemo::new();
    let initial_phase = demo.current_phase;

    // Simulate time passage
    demo.start_time = Instant::now() - Duration::from_secs(7);
    demo.update(Duration::from_millis(16));

    // Phase should have changed
    assert_ne!(demo.current_phase, initial_phase);
  }

  #[test]
  fn test_animation_system_integration() {
    let animation = AnimationBuilder::new("test-animation")
      .animate_property(AnimatedProperty::Opacity(0.0, 1.0))
      .duration(Duration::from_millis(100))
      .easing(TweenEasing::Linear)
      .build();

    assert_eq!(animation.get_progress(), 0.0);
    assert!(!animation.is_playing());
    assert!(!animation.is_completed());
  }
}
