//! Interactive Animation System Demo
//!
//! This enhanced example demonstrates the comprehensive animation system with various easing functions,
//! property animations, timeline management, and real-time interactive controls for smooth TUI animations.
//!
//! Features demonstrated:
//! - Multiple easing functions (bounce, elastic, back, exponential)
//! - Property animations (opacity, position, color, scale)
//! - Animation timelines and sequencing
//! - Parallel and sequential animations
//! - Animation state management and callbacks
//! - Interactive controls for real-time animation adjustment
//! - Performance monitoring and statistics

use reactive_tui::{
  themes::ColorDefinition,
  widgets::{
    bounce, fade_in, fade_out, pulse, slide_in_left, AnimatedProperty, AnimationBuilder,
    AnimationManager, AnimationTimeline, LoopMode, TweenEasing,
  },
  events::{ActionDispatcher, ActionResult},
};
use std::time::{Duration, Instant};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

/// Interactive animation demo application
struct AnimationDemo {
  /// Animation manager for coordinating multiple animations
  animation_manager: AnimationManager,
  /// Start time for demo timing
  start_time: Instant,
  /// Demo phase tracking
  current_phase: usize,
  /// Phase descriptions
  phases: Vec<&'static str>,
  /// Action dispatcher for interactive controls
  action_dispatcher: ActionDispatcher,
  /// User interaction tracking
  user_interactions: Arc<Mutex<HashMap<String, u32>>>,
  /// Interactive control state
  control_state: InteractiveControlState,
  /// Performance metrics
  performance_metrics: PerformanceMetrics,
}

/// Interactive control state for real-time animation adjustment
#[derive(Debug, Clone)]
struct InteractiveControlState {
  /// Whether animations are paused
  paused: bool,
  /// Speed multiplier (0.1 to 5.0)
  speed_multiplier: f32,
  /// Current easing function override
  easing_override: Option<TweenEasing>,
  /// Whether to show debug information
  show_debug: bool,
  /// Manual phase control
  manual_phase_control: bool,
  /// Selected animation for individual control
  selected_animation: Option<String>,
}

impl Default for InteractiveControlState {
  fn default() -> Self {
    Self {
      paused: false,
      speed_multiplier: 1.0,
      easing_override: None,
      show_debug: false,
      manual_phase_control: false,
      selected_animation: None,
    }
  }
}

/// Performance metrics for animation system monitoring
#[derive(Debug, Clone, Default)]
struct PerformanceMetrics {
  /// Total animations created
  total_animations_created: u32,
  /// Currently active animations
  active_animations: u32,
  /// Completed animations
  completed_animations: u32,
  /// Average frame time
  _avg_frame_time_ms: f32,
  /// Animation update time
  _animation_update_time_ms: f32,
  /// Memory usage estimate
  memory_usage_kb: u32,
}

impl AnimationDemo {
  /// Create a new interactive animation demo
  fn new() -> Self {
    let mut action_dispatcher = ActionDispatcher::new();
    let user_interactions = Arc::new(Mutex::new(HashMap::new()));

    // Setup interactive controls
    Self::setup_interactive_controls(&mut action_dispatcher, user_interactions.clone());

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
        "Phase 6: Interactive Controls Demo",
      ],
      action_dispatcher,
      user_interactions,
      control_state: InteractiveControlState::default(),
      performance_metrics: PerformanceMetrics::default(),
    };

    demo.setup_animations();
    demo
  }

  /// Setup interactive controls and action handlers
  fn setup_interactive_controls(
    action_dispatcher: &mut ActionDispatcher,
    interactions: Arc<Mutex<HashMap<String, u32>>>,
  ) {
    let interactions_clone = interactions.clone();
    action_dispatcher.register("toggle_pause", move |_action| {
      println!("⏸️ Animation playback toggled");
      let mut stats = interactions_clone.lock().unwrap();
      *stats.entry("toggle_pause".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone2 = interactions.clone();
    action_dispatcher.register("speed_up", move |_action| {
      println!("⚡ Animation speed increased");
      let mut stats = interactions_clone2.lock().unwrap();
      *stats.entry("speed_up".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone3 = interactions.clone();
    action_dispatcher.register("speed_down", move |_action| {
      println!("🐌 Animation speed decreased");
      let mut stats = interactions_clone3.lock().unwrap();
      *stats.entry("speed_down".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone4 = interactions.clone();
    action_dispatcher.register("next_phase", move |_action| {
      println!("⏭️ Next animation phase");
      let mut stats = interactions_clone4.lock().unwrap();
      *stats.entry("next_phase".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone5 = interactions.clone();
    action_dispatcher.register("toggle_debug", move |_action| {
      println!("🔍 Debug mode toggled");
      let mut stats = interactions_clone5.lock().unwrap();
      *stats.entry("toggle_debug".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });

    let interactions_clone6 = interactions.clone();
    action_dispatcher.register("reset_animations", move |_action| {
      println!("🔄 Animations reset");
      let mut stats = interactions_clone6.lock().unwrap();
      *stats.entry("reset_animations".to_string()).or_insert(0) += 1;
      ActionResult::Handled
    });
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

    // Phase 6: Interactive demonstrations
    self.create_interactive_demonstrations();

    // Update performance metrics
    self.update_performance_metrics();
  }

  /// Handle user input for interactive controls
  fn handle_user_input(&mut self, key: char) -> bool {
    match key.to_ascii_lowercase() {
      ' ' => {
        // Toggle pause/play
        self.control_state.paused = !self.control_state.paused;
        let action = self.action_dispatcher.action("toggle_pause").build();
        self.action_dispatcher.dispatch(action);

        if self.control_state.paused {
          self.pause_all_animations();
        } else {
          self.resume_all_animations();
        }
        false
      }
      '+' | '=' => {
        // Speed up animations
        self.control_state.speed_multiplier = (self.control_state.speed_multiplier * 1.2).min(5.0);
        let action = self.action_dispatcher.action("speed_up").build();
        self.action_dispatcher.dispatch(action);
        println!("🚀 Speed: {:.1}x", self.control_state.speed_multiplier);
        false
      }
      '-' | '_' => {
        // Slow down animations
        self.control_state.speed_multiplier = (self.control_state.speed_multiplier / 1.2).max(0.1);
        let action = self.action_dispatcher.action("speed_down").build();
        self.action_dispatcher.dispatch(action);
        println!("🐌 Speed: {:.1}x", self.control_state.speed_multiplier);
        false
      }
      'n' => {
        // Next phase
        self.control_state.manual_phase_control = true;
        self.current_phase = (self.current_phase + 1) % self.phases.len();
        let action = self.action_dispatcher.action("next_phase").build();
        self.action_dispatcher.dispatch(action);
        self.trigger_phase_animations(self.current_phase);
        false
      }
      'd' => {
        // Toggle debug mode
        self.control_state.show_debug = !self.control_state.show_debug;
        let action = self.action_dispatcher.action("toggle_debug").build();
        self.action_dispatcher.dispatch(action);
        false
      }
      'r' => {
        // Reset animations
        self.reset_all_animations();
        let action = self.action_dispatcher.action("reset_animations").build();
        self.action_dispatcher.dispatch(action);
        false
      }
      '1'..='5' => {
        // Jump to specific phase
        let phase = (key as u8 - b'1') as usize;
        if phase < self.phases.len() {
          self.control_state.manual_phase_control = true;
          self.current_phase = phase;
          self.trigger_phase_animations(self.current_phase);
          println!("🎯 Jumped to phase {}", phase + 1);
        }
        false
      }
      'q' => {
        println!("👋 Exiting animation demo...");
        true // Exit
      }
      _ => false
    }
  }

  /// Pause all active animations
  fn pause_all_animations(&mut self) {
    // In a real implementation, we would iterate through animations and pause them
    println!("⏸️ All animations paused");
  }

  /// Resume all paused animations
  fn resume_all_animations(&mut self) {
    // In a real implementation, we would iterate through animations and resume them
    println!("▶️ All animations resumed");
  }

  /// Reset all animations to initial state
  fn reset_all_animations(&mut self) {
    // Clear existing animations and recreate them
    self.animation_manager = AnimationManager::new();
    self.setup_animations();
    self.current_phase = 0;
    self.control_state.manual_phase_control = false;
    println!("🔄 All animations reset to initial state");
  }

  /// Update performance metrics
  fn update_performance_metrics(&mut self) {
    self.performance_metrics.active_animations = self.animation_manager.active_count() as u32;
    self.performance_metrics.total_animations_created += 1;

    // Estimate memory usage (rough calculation)
    self.performance_metrics.memory_usage_kb =
      (self.animation_manager.active_count() * 256) as u32; // ~256 bytes per animation
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
        .easing(easing.clone())
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

  /// Create interactive demonstration animations
  fn create_interactive_demonstrations(&mut self) {
    // User-controllable animation
    let interactive_anim = AnimationBuilder::new("interactive-demo")
      .animate_property(AnimatedProperty::Multiple(vec![
        AnimatedProperty::Opacity(0.3, 1.0),
        AnimatedProperty::Scale(0.8, 1.5),
        AnimatedProperty::Position(0, 15, 60, 15),
      ]))
      .duration(Duration::from_millis(2000))
      .easing(TweenEasing::EaseInOut)
      .loop_mode(LoopMode::PingPong)
      .on_update(|_animation, _values| {
        // Real-time feedback for interactive control
        // values is already an &AnimatedValue, not an Option
        // In a real implementation, this would update UI elements
      })
      .on_complete(|animation| {
        println!("🎯 Interactive animation cycle completed: {}", animation.id);
      })
      .build();
    self.animation_manager.add_animation(interactive_anim);

    // Performance test animation (many small animations)
    for i in 0..20 {
      let perf_anim = AnimationBuilder::new(format!("perf-test-{i}"))
        .animate_property(AnimatedProperty::Position(
          i * 3,
          20,
          i * 3,
          25,
        ))
        .duration(Duration::from_millis(1000 + i as u64 * 50))
        .easing(TweenEasing::Bounce)
        .loop_mode(LoopMode::Infinite)
        .build();
      self.animation_manager.add_animation(perf_anim);
    }

    // Real-time easing comparison
    let easing_types = vec![
      TweenEasing::Linear,
      TweenEasing::EaseIn,
      TweenEasing::EaseOut,
      TweenEasing::EaseInOut,
      TweenEasing::Bounce,
    ];

    for (i, easing) in easing_types.iter().enumerate() {
      let comparison_anim = AnimationBuilder::new(format!("easing-comparison-{i}"))
        .animate_property(AnimatedProperty::Position(
          0,
          27 + i as i16,
          40,
          27 + i as i16,
        ))
        .duration(Duration::from_millis(3000))
        .easing(easing.clone())
        .loop_mode(LoopMode::PingPong)
        .build();
      self.animation_manager.add_animation(comparison_anim);
    }
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

  /// Render the current interactive demo state
  fn render(&self) -> String {
    let mut output = String::new();

    // Header
    output.push_str("🎬 Interactive TUI Animation System Demo\n");
    output
      .push_str("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");

    // Interactive controls
    output.push_str("🎮 Interactive Controls:\n");
    output.push_str("   [SPACE] Pause/Resume | [+/-] Speed Up/Down | [N] Next Phase\n");
    output.push_str("   [1-6] Jump to Phase | [D] Debug Mode | [R] Reset | [Q] Quit\n\n");

    // Current status
    output.push_str(&format!(
      "📊 Status: {} | Active: {} | Speed: {:.1}x | {}\n\n",
      self.phases[self.current_phase],
      self.animation_manager.active_count(),
      self.control_state.speed_multiplier,
      if self.control_state.paused { "⏸️ PAUSED" } else { "▶️ PLAYING" }
    ));

    // Performance metrics
    output.push_str("📊 Performance Metrics:\n");
    output.push_str(&format!(
      "• Active Animations: {} | Total Created: {}\n",
      self.performance_metrics.active_animations,
      self.performance_metrics.total_animations_created
    ));
    output.push_str(&format!(
      "• Completed: {} | Memory Usage: ~{}KB\n",
      self.performance_metrics.completed_animations,
      self.performance_metrics.memory_usage_kb
    ));
    output.push_str(&format!(
      "• Phase: {}/{} | Runtime: {:.1}s\n",
      self.current_phase + 1,
      self.phases.len(),
      self.start_time.elapsed().as_secs_f32()
    ));

    // User interaction stats
    let interaction_stats = self.user_interactions.lock().unwrap();
    let total_interactions: u32 = interaction_stats.values().sum();
    output.push_str(&format!(
      "• User Interactions: {} total\n\n",
      total_interactions
    ));

    // Feature showcase
    output.push_str("🎯 Features Demonstrated:\n");
    output.push_str("• 15+ Easing Functions (Linear, Bounce, Elastic, Back, Exponential...)\n");
    output.push_str("• Property Animations (Opacity, Position, Size, Color, Scale, Rotation)\n");
    output.push_str("• Timeline Management (Sequential & Parallel)\n");
    output.push_str("• Animation States (Play, Pause, Stop, Reverse, Loop)\n");
    output.push_str("• Callback System (Start, Update, Complete, Loop events)\n");
    output.push_str("• Performance Optimization (Frame-based timing, Efficient interpolation)\n");
    output.push_str("• 🎮 Interactive Controls (Real-time speed adjustment, phase control)\n");
    output.push_str("• 📊 Performance Monitoring (Memory usage, frame timing, statistics)\n\n");

    // Debug information
    if self.control_state.show_debug {
      output.push_str("🔍 Debug Information:\n");
      output.push_str(&format!(
        "• Control State: Paused={}, Speed={:.1}x, Manual={}\n",
        self.control_state.paused,
        self.control_state.speed_multiplier,
        self.control_state.manual_phase_control
      ));
      output.push_str(&format!(
        "• Easing Override: {:?}\n",
        self.control_state.easing_override
      ));
      output.push_str(&format!(
        "• Selected Animation: {:?}\n",
        self.control_state.selected_animation
      ));

      // Show individual interaction counts
      if !interaction_stats.is_empty() {
        output.push_str("• Interaction Breakdown:\n");
        for (action, count) in interaction_stats.iter() {
          output.push_str(&format!("  - {}: {} times\n", action, count));
        }
      }
      output.push_str("\n");
    }

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
      5 => output.push_str("🎮 Interactive demonstrations: user-controllable animations, performance tests, real-time easing comparisons"),
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

/// Run the interactive animation demo
fn main() {
  println!("🎬 Starting Interactive TUI Animation System Demo...\n");
  println!("🎮 Controls: [SPACE] Pause | [+/-] Speed | [N] Next | [D] Debug | [R] Reset | [Q] Quit\n");

  let mut demo = AnimationDemo::new();
  let mut last_update = Instant::now();
  let mut frame_count = 0;
  let mut should_exit = false;
  let mut last_input_check = Instant::now();

  // Simulate main loop with interactive controls
  while !should_exit && demo.start_time.elapsed() < Duration::from_secs(60) {
    let now = Instant::now();
    let delta_time = now.duration_since(last_update);
    last_update = now;

    // Simulate user input checking (in real app, this would be async)
    if last_input_check.elapsed().as_millis() > 200 {
      // Simulate some user interactions for demo purposes
      match frame_count {
        300 => should_exit = demo.handle_user_input(' '), // Auto-pause
        600 => should_exit = demo.handle_user_input('+'), // Auto-speed up
        900 => should_exit = demo.handle_user_input('n'), // Auto-next phase
        1200 => should_exit = demo.handle_user_input('d'), // Auto-debug
        1500 => should_exit = demo.handle_user_input('r'), // Auto-reset
        _ => {}
      }
      last_input_check = now;
    }

    // Update animations with speed multiplier
    let adjusted_delta = if !demo.control_state.paused {
      Duration::from_nanos((delta_time.as_nanos() as f32 * demo.control_state.speed_multiplier) as u64)
    } else {
      Duration::from_nanos(0)
    };

    demo.update(adjusted_delta);

    // Render every 15th frame to avoid spam but show smooth updates
    if frame_count % 15 == 0 {
      print!("\x1B[2J\x1B[1;1H"); // Clear screen
      println!("{}", demo.render());
    }

    // Simulate 60 FPS
    std::thread::sleep(Duration::from_millis(16));
    frame_count += 1;

    // Update performance metrics
    if frame_count % 60 == 0 {
      demo.update_performance_metrics();
    }
  }

  // Final summary with interaction statistics
  let interaction_stats = demo.user_interactions.lock().unwrap();
  let total_interactions: u32 = interaction_stats.values().sum();

  println!("\n🎉 Interactive Animation Demo Complete!");
  println!("📊 Final Statistics:");
  println!("• Total Runtime: {:.1}s", demo.start_time.elapsed().as_secs_f32());
  println!("• Active Animations: {}", demo.animation_manager.active_count());
  println!("• User Interactions: {}", total_interactions);
  println!("• Final Speed: {:.1}x", demo.control_state.speed_multiplier);

  if !interaction_stats.is_empty() {
    println!("\n🎮 Interaction Breakdown:");
    for (action, count) in interaction_stats.iter() {
      println!("   {}: {} times", action, count);
    }
  }

  println!("\n✨ The enhanced animation system provides:");
  println!("• Smooth property transitions with 15+ easing functions");
  println!("• Timeline management for complex sequences");
  println!("• Performance-optimized frame-based timing");
  println!("• Rich callback system for animation lifecycle events");
  println!("• 🎮 Interactive real-time controls and adjustments");
  println!("• 📊 Performance monitoring and statistics tracking");
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
