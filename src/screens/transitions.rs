/*!
 * Screen transition animations
 */

use super::*;
use crate::components::{div, text, Element};
use std::time::{Duration, Instant};

/// Transition types
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TransitionType {
  /// No transition
  None,
  /// Fade in/out
  Fade,
  /// Slide from right
  SlideRight,
  /// Slide from left
  SlideLeft,
  /// Slide from bottom
  SlideUp,
  /// Slide from top
  SlideDown,
  /// Scale up
  ScaleUp,
  /// Scale down
  ScaleDown,
  /// Flip horizontally
  FlipHorizontal,
  /// Flip vertically
  FlipVertical,
}

impl Default for TransitionType {
  fn default() -> Self {
    Self::None
  }
}

/// Transition state
#[derive(Debug)]
struct TransitionState {
  /// Transition type
  transition_type: TransitionType,
  /// Start time
  start_time: Instant,
  /// Duration
  duration: Duration,
  /// Progress (0.0 to 1.0)
  progress: f32,
}

/// Transition manager handles screen transitions
pub struct TransitionManager {
  /// Current transition state
  current_transition: Option<TransitionState>,
}

impl TransitionManager {
  /// Create new transition manager
  pub fn new() -> Self {
    Self {
      current_transition: None,
    }
  }

  /// Start a transition
  pub fn start_transition(&mut self, transition_type: TransitionType, duration_ms: u32) {
    if transition_type == TransitionType::None || duration_ms == 0 {
      self.current_transition = None;
      return;
    }

    self.current_transition = Some(TransitionState {
      transition_type,
      start_time: Instant::now(),
      duration: Duration::from_millis(duration_ms as u64),
      progress: 0.0,
    });
  }

  /// Update transition progress
  pub fn update(&mut self) {
    if let Some(ref mut transition) = self.current_transition {
      let elapsed = transition.start_time.elapsed();
      transition.progress = (elapsed.as_secs_f32() / transition.duration.as_secs_f32()).min(1.0);

      // Complete transition
      if transition.progress >= 1.0 {
        self.current_transition = None;
      }
    }
  }

  /// Check if transition is active
  pub fn is_transitioning(&self) -> bool {
    self.current_transition.is_some()
  }

  /// Get current progress
  pub fn progress(&self) -> f32 {
    self
      .current_transition
      .as_ref()
      .map(|t| t.progress)
      .unwrap_or(1.0)
  }

  /// Render transition effect placeholder
  pub fn render_placeholder(&self, screen_id: &str) -> Option<Element> {
    self.current_transition.as_ref().map(|transition| {
      // Update progress
      let progress = transition.progress;

      // Apply transition based on type
      match transition.transition_type {
        TransitionType::None => div()
          .child(text(format!("Screen: {screen_id}")).build())
          .build(),

        TransitionType::Fade => div()
          .class("transition-fade")
          .child(
            text(format!(
              "Screen: {} (fade {}%)",
              screen_id,
              (progress * 100.0) as i32
            ))
            .build(),
          )
          .build(),

        TransitionType::SlideRight => {
          let _offset = 100.0 * (1.0 - progress);
          div()
            .class("transition-slide-right")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::SlideLeft => {
          let _offset = -100.0 * (1.0 - progress);
          div()
            .class("transition-slide-left")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::SlideUp => {
          let _offset = 100.0 * (1.0 - progress);
          div()
            .class("transition-slide-up")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::SlideDown => {
          let _offset = -100.0 * (1.0 - progress);
          div()
            .class("transition-slide-down")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::ScaleUp => {
          let _scale = 0.8 + 0.2 * progress;
          div()
            .class("transition-scale-up")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::ScaleDown => {
          let _scale = 1.2 - 0.2 * progress;
          div()
            .class("transition-scale-down")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::FlipHorizontal => {
          let _rotation = 180.0 * (1.0 - progress);
          div()
            .class("transition-flip-horizontal")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }

        TransitionType::FlipVertical => {
          let _rotation = 180.0 * (1.0 - progress);
          div()
            .class("transition-flip-vertical")
            .child(text(format!("Screen: {screen_id}")).build())
            .build()
        }
      }
    })
  }
}

impl Default for TransitionManager {
  fn default() -> Self {
    Self::new()
  }
}

/// Easing functions for smooth transitions
#[allow(dead_code)]
pub mod easing {
  /// Linear easing (no easing)
  pub fn linear(t: f32) -> f32 {
    t
  }

  /// Ease in (quadratic)
  pub fn ease_in(t: f32) -> f32 {
    t * t
  }

  /// Ease out (quadratic)
  pub fn ease_out(t: f32) -> f32 {
    t * (2.0 - t)
  }

  /// Ease in-out (quadratic)
  pub fn ease_in_out(t: f32) -> f32 {
    if t < 0.5 {
      2.0 * t * t
    } else {
      -1.0 + (4.0 - 2.0 * t) * t
    }
  }

  /// Ease in (cubic)
  pub fn ease_in_cubic(t: f32) -> f32 {
    t * t * t
  }

  /// Ease out (cubic)
  pub fn ease_out_cubic(t: f32) -> f32 {
    let t = t - 1.0;
    t * t * t + 1.0
  }

  /// Ease in-out (cubic)
  pub fn ease_in_out_cubic(t: f32) -> f32 {
    if t < 0.5 {
      4.0 * t * t * t
    } else {
      let t = 2.0 * t - 2.0;
      1.0 + t * t * t / 2.0
    }
  }

  /// Elastic ease in
  pub fn ease_in_elastic(t: f32) -> f32 {
    if t == 0.0 || t == 1.0 {
      t
    } else {
      let p = 0.3;
      let a = 1.0;
      let s = p / 4.0;
      let t = t - 1.0;
      -(a * 2.0_f32.powf(10.0 * t) * ((t - s) * 2.0 * std::f32::consts::PI / p).sin())
    }
  }

  /// Bounce ease out
  pub fn ease_out_bounce(t: f32) -> f32 {
    if t < 1.0 / 2.75 {
      7.5625 * t * t
    } else if t < 2.0 / 2.75 {
      let t = t - 1.5 / 2.75;
      7.5625 * t * t + 0.75
    } else if t < 2.5 / 2.75 {
      let t = t - 2.25 / 2.75;
      7.5625 * t * t + 0.9375
    } else {
      let t = t - 2.625 / 2.75;
      7.5625 * t * t + 0.984375
    }
  }
}

/// Animated transition builder
pub struct TransitionBuilder {
  transition_type: TransitionType,
  duration: u32,
  easing: fn(f32) -> f32,
  delay: u32,
}

impl TransitionBuilder {
  /// Create new transition builder
  pub fn new(transition_type: TransitionType) -> Self {
    Self {
      transition_type,
      duration: 300,
      easing: easing::ease_in_out,
      delay: 0,
    }
  }

  /// Set duration in milliseconds
  pub fn duration(mut self, ms: u32) -> Self {
    self.duration = ms;
    self
  }

  /// Set easing function
  pub fn easing(mut self, easing_fn: fn(f32) -> f32) -> Self {
    self.easing = easing_fn;
    self
  }

  /// Set delay in milliseconds
  pub fn delay(mut self, ms: u32) -> Self {
    self.delay = ms;
    self
  }

  /// Build transition options
  pub fn build(self) -> NavigationOptions {
    NavigationOptions {
      transition: self.transition_type,
      duration: self.duration,
      ..Default::default()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_transition_manager() {
    let mut manager = TransitionManager::new();

    assert!(!manager.is_transitioning());

    // Start transition
    manager.start_transition(TransitionType::Fade, 1000);
    assert!(manager.is_transitioning());
    assert!(manager.progress() < 1.0);

    // Simulate time passing
    std::thread::sleep(Duration::from_millis(100));
    manager.update();

    let progress = manager.progress();
    assert!(progress > 0.0 && progress < 1.0);
  }

  #[test]
  fn test_easing_functions() {
    // Test boundary conditions
    assert_eq!(easing::linear(0.0), 0.0);
    assert_eq!(easing::linear(1.0), 1.0);

    assert_eq!(easing::ease_in(0.0), 0.0);
    assert_eq!(easing::ease_in(1.0), 1.0);

    assert_eq!(easing::ease_out(0.0), 0.0);
    assert_eq!(easing::ease_out(1.0), 1.0);

    // Test mid-point
    assert!((easing::ease_in_out(0.5) - 0.5).abs() < 0.01);
  }
}
