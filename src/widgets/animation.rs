//! Animation System Widget
//!
//! A comprehensive animation system providing smooth transitions, easing functions,
//! and property animations for TUI widgets with frame-based timing and interpolation.
//!
//! # Features
//!
//! - **Property Animations**: Animate opacity, position, size, colors, and custom properties
//! - **Easing Functions**: Linear, ease-in/out, cubic bezier, bounce, elastic, and custom curves
//! - **Timeline Management**: Sequential, parallel, and staggered animation sequences
//! - **Frame-based Timing**: Smooth 60fps animations with delta time calculations
//! - **Animation States**: Play, pause, stop, reverse, and loop controls
//! - **Interpolation**: Smooth value transitions between keyframes
//! - **Performance Optimization**: Efficient dirty region tracking and batched updates
//! - **Event Callbacks**: Animation start, update, complete, and loop event handling
//!
//! # Basic Usage
//!
//! ```rust
//! use tui_core::widgets::{Animation, AnimationBuilder, EasingFunction, AnimatedProperty};
//!
//! let mut fade_in = AnimationBuilder::new("fade-in")
//!     .duration(Duration::from_millis(500))
//!     .easing(EasingFunction::EaseInOut)
//!     .animate_property(AnimatedProperty::Opacity(0.0, 1.0))
//!     .on_complete(|animation| println!("Fade in complete!"))
//!     .build();
//!
//! fade_in.play();
//! ```

use crate::{components::element::Element, reactive::Reactive, themes::ColorDefinition};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};

// Type aliases for complex function pointer types
type OnStartCallback = Arc<dyn Fn(&Animation) + Send + Sync>;
type OnUpdateCallback = Arc<dyn Fn(&Animation, &AnimatedValue) + Send + Sync>;
type OnCompleteCallback = Arc<dyn Fn(&Animation) + Send + Sync>;
type OnLoopCallback = Arc<dyn Fn(&Animation, u32) + Send + Sync>;
type OnPauseCallback = Arc<dyn Fn(&Animation) + Send + Sync>;
type OnStopCallback = Arc<dyn Fn(&Animation) + Send + Sync>;

/// Unique identifier for animations
pub type AnimationId = String;

/// Unique identifier for animation timelines
pub type TimelineId = String;

/// Animation easing functions for smooth transitions
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
    /// Linear interpolation (no easing)
    Linear,
    /// Ease in (slow start)
    EaseIn,
    /// Ease out (slow end)
    EaseOut,
    /// Ease in and out (slow start and end)
    EaseInOut,
    /// Cubic bezier curve with custom control points
    CubicBezier(f32, f32, f32, f32),
    /// Bounce effect at the end
    Bounce,
    /// Elastic effect with overshoot
    Elastic,
    /// Back effect with slight overshoot
    Back,
    /// Exponential easing
    Expo,
    /// Circular easing
    Circ,
    /// Sine wave easing
    Sine,
    /// Quadratic easing
    Quad,
    /// Cubic easing
    Cubic,
    /// Quartic easing
    Quart,
    /// Quintic easing
    Quint,
}

impl Default for EasingFunction {
    fn default() -> Self {
        Self::EaseInOut
    }
}

impl EasingFunction {
    /// Apply easing function to a normalized time value (0.0 to 1.0)
    pub fn apply(&self, t: f32) -> f32 {
        let t = t.clamp(0.0, 1.0);

        match self {
            Self::Linear => t,
            Self::EaseIn => t * t,
            Self::EaseOut => t * (2.0 - t),
            Self::EaseInOut => {
                if t < 0.5 {
                    2.0 * t * t
                } else {
                    -1.0 + (4.0 - 2.0 * t) * t
                }
            }
            Self::CubicBezier(_x1, y1, _x2, y2) => {
                // Simplified cubic bezier approximation
                let t2 = t * t;
                let t3 = t2 * t;
                3.0 * (1.0 - t) * (1.0 - t) * t * y1 + 3.0 * (1.0 - t) * t2 * y2 + t3
            }
            Self::Bounce => {
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
            Self::Elastic => {
                if t == 0.0 || t == 1.0 {
                    t
                } else {
                    let p = 0.3;
                    let s = p / 4.0;
                    -(2.0_f32.powf(10.0 * (t - 1.0)))
                        * ((t - 1.0 - s) * (2.0 * std::f32::consts::PI) / p).sin()
                }
            }
            Self::Back => {
                let c1 = 1.70158;
                let c3 = c1 + 1.0;
                c3 * t * t * t - c1 * t * t
            }
            Self::Expo => {
                if t == 0.0 {
                    0.0
                } else {
                    2.0_f32.powf(10.0 * (t - 1.0))
                }
            }
            Self::Circ => 1.0 - (1.0 - t * t).sqrt(),
            Self::Sine => 1.0 - (t * std::f32::consts::PI / 2.0).cos(),
            Self::Quad => t * t,
            Self::Cubic => t * t * t,
            Self::Quart => t * t * t * t,
            Self::Quint => t * t * t * t * t,
        }
    }
}

/// Properties that can be animated
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimatedProperty {
    /// Opacity animation (from, to)
    Opacity(f32, f32),
    /// Position animation (from_x, from_y, to_x, to_y)
    Position(i16, i16, i16, i16),
    /// Size animation (from_width, from_height, to_width, to_height)
    Size(u16, u16, u16, u16),
    /// Color animation (from, to)
    Color(ColorDefinition, ColorDefinition),
    /// Scale animation (from, to)
    Scale(f32, f32),
    /// Rotation animation in degrees (from, to)
    Rotation(f32, f32),
    /// Custom numeric property (name, from, to)
    Custom(String, f32, f32),
    /// Multiple properties animated together
    Multiple(Vec<AnimatedProperty>),
}

impl AnimatedProperty {
    /// Get the current interpolated value at time t (0.0 to 1.0)
    pub fn interpolate(&self, t: f32) -> AnimatedValue {
        match self {
            Self::Opacity(from, to) => AnimatedValue::Opacity(from + (to - from) * t),
            Self::Position(fx, fy, tx, ty) => {
                let x = *fx as f32 + (*tx as f32 - *fx as f32) * t;
                let y = *fy as f32 + (*ty as f32 - *fy as f32) * t;
                AnimatedValue::Position(x as i16, y as i16)
            }
            Self::Size(fw, fh, tw, th) => {
                let w = *fw as f32 + (*tw as f32 - *fw as f32) * t;
                let h = *fh as f32 + (*th as f32 - *fh as f32) * t;
                AnimatedValue::Size(w as u16, h as u16)
            }
            Self::Color(from, to) => {
                let r = from.r as f32 + (to.r as f32 - from.r as f32) * t;
                let g = from.g as f32 + (to.g as f32 - from.g as f32) * t;
                let b = from.b as f32 + (to.b as f32 - from.b as f32) * t;
                AnimatedValue::Color(ColorDefinition {
                    r: r as u8,
                    g: g as u8,
                    b: b as u8,
                })
            }
            Self::Scale(from, to) => AnimatedValue::Scale(from + (to - from) * t),
            Self::Rotation(from, to) => AnimatedValue::Rotation(from + (to - from) * t),
            Self::Custom(name, from, to) => {
                AnimatedValue::Custom(name.clone(), from + (to - from) * t)
            }
            Self::Multiple(properties) => {
                let values: Vec<AnimatedValue> =
                    properties.iter().map(|prop| prop.interpolate(t)).collect();
                AnimatedValue::Multiple(values)
            }
        }
    }
}

/// Current animated values
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AnimatedValue {
    /// Current opacity value
    Opacity(f32),
    /// Current position (x, y)
    Position(i16, i16),
    /// Current size (width, height)
    Size(u16, u16),
    /// Current color
    Color(ColorDefinition),
    /// Current scale factor
    Scale(f32),
    /// Current rotation in degrees
    Rotation(f32),
    /// Custom property value
    Custom(String, f32),
    /// Multiple values
    Multiple(Vec<AnimatedValue>),
}

/// Animation playback state
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum AnimationState {
    /// Animation is stopped/not started
    #[default]
    Stopped,
    /// Animation is playing
    Playing,
    /// Animation is paused
    Paused,
    /// Animation has completed
    Completed,
    /// Animation is playing in reverse
    Reversed,
}

/// Animation loop behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum LoopMode {
    /// Play once
    None,
    /// Loop indefinitely
    Infinite,
    /// Loop a specific number of times
    Count(u32),
    /// Ping-pong (forward then reverse)
    PingPong,
}

impl Default for LoopMode {
    fn default() -> Self {
        Self::None
    }
}

/// Animation configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AnimationConfig {
    /// Animation duration
    pub duration: Duration,
    /// Easing function
    pub easing: EasingFunction,
    /// Delay before starting
    pub delay: Duration,
    /// Loop behavior
    pub loop_mode: LoopMode,
    /// Animation direction
    pub reverse: bool,
    /// Speed multiplier (1.0 = normal speed)
    pub speed: f32,
    /// Whether to auto-play on creation
    pub auto_play: bool,
    /// Whether to auto-reverse on completion
    pub auto_reverse: bool,
}

impl Default for AnimationConfig {
    fn default() -> Self {
        Self {
            duration: Duration::from_millis(500),
            easing: EasingFunction::EaseInOut,
            delay: Duration::ZERO,
            loop_mode: LoopMode::None,
            reverse: false,
            speed: 1.0,
            auto_play: false,
            auto_reverse: false,
        }
    }
}

/// Animation runtime state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AnimationRuntimeState {
    /// Current playback state
    pub state: AnimationState,
    /// Current time position in animation
    pub current_time: Duration,
    /// Number of loops completed
    pub loops_completed: u32,
    /// Whether currently playing in reverse
    pub is_reversed: bool,
    /// Current animated values
    pub current_values: Option<AnimatedValue>,
    /// Progress (0.0 to 1.0)
    pub progress: f32,
}

/// Non-serializable runtime data
#[derive(Debug, Default)]
pub struct AnimationRuntime {
    /// Last frame timestamp
    pub last_frame_time: Option<Instant>,
}

/// Event callbacks for animation lifecycle
#[derive(Default)]
pub struct AnimationCallbacks {
    /// Called when animation starts
    pub on_start: Option<OnStartCallback>,
    /// Called on each frame update
    pub on_update: Option<OnUpdateCallback>,
    /// Called when animation completes
    pub on_complete: Option<OnCompleteCallback>,
    /// Called when animation loops
    pub on_loop: Option<OnLoopCallback>,
    /// Called when animation is paused
    pub on_pause: Option<OnPauseCallback>,
    /// Called when animation is stopped
    pub on_stop: Option<OnStopCallback>,
}

/// Main Animation widget
pub struct Animation {
    /// Unique animation identifier
    pub id: AnimationId,
    /// Property to animate
    pub property: AnimatedProperty,
    /// Animation configuration
    pub config: AnimationConfig,
    /// Runtime state
    pub state: Reactive<AnimationRuntimeState>,
    /// Non-serializable runtime data
    pub runtime: AnimationRuntime,
    /// Event callbacks
    pub callbacks: AnimationCallbacks,
    /// Start time for delay calculations
    start_time: Option<Instant>,
}

impl Animation {
    /// Create a new animation builder
    pub fn builder<S: Into<String>>(id: S) -> AnimationBuilder {
        AnimationBuilder::new(id)
    }

    /// Play the animation
    pub fn play(&mut self) {
        self.state.update(|state| {
            state.state = AnimationState::Playing;
        });

        self.runtime.last_frame_time = Some(Instant::now());
        self.start_time = Some(Instant::now());

        if let Some(callback) = &self.callbacks.on_start {
            callback(self);
        }
    }

    /// Pause the animation
    pub fn pause(&mut self) {
        self.state.update(|state| {
            state.state = AnimationState::Paused;
        });

        if let Some(callback) = &self.callbacks.on_pause {
            callback(self);
        }
    }

    /// Stop the animation and reset to beginning
    pub fn stop(&mut self) {
        self.state.update(|state| {
            state.state = AnimationState::Stopped;
            state.current_time = Duration::ZERO;
            state.progress = 0.0;
            state.loops_completed = 0;
            state.is_reversed = false;
            state.current_values = None;
        });

        self.runtime.last_frame_time = None;
        self.start_time = None;

        if let Some(callback) = &self.callbacks.on_stop {
            callback(self);
        }
    }

    /// Reverse the animation direction  
    pub fn reverse(&mut self) {
        self.state.update(|state| {
            state.is_reversed = !state.is_reversed;
            if state.state == AnimationState::Playing {
                state.state = AnimationState::Reversed;
            }
        });
    }

    /// Update animation frame (call this in main loop)
    pub fn update(&mut self, delta_time: Duration) -> bool {
        let mut state = self.state.get();

        if state.state != AnimationState::Playing {
            return false;
        }

        // Handle delay
        if let Some(start_time) = self.start_time {
            if start_time.elapsed() < self.config.delay {
                return false;
            }
        }

        // Update time
        let adjusted_delta = Duration::from_secs_f32(delta_time.as_secs_f32() * self.config.speed);
        state.current_time += adjusted_delta;

        // Calculate progress
        let total_duration = self.config.duration;
        let raw_progress = if total_duration > Duration::ZERO {
            (state.current_time.as_secs_f32() / total_duration.as_secs_f32()).min(1.0)
        } else {
            1.0
        };

        // Apply direction
        let progress = if state.is_reversed || self.config.reverse {
            1.0 - raw_progress
        } else {
            raw_progress
        };

        // Apply easing
        let eased_progress = self.config.easing.apply(progress);

        // Update state
        state.progress = eased_progress;
        state.current_values = Some(self.property.interpolate(eased_progress));

        // Trigger update callback
        if let Some(callback) = &self.callbacks.on_update {
            if let Some(ref values) = state.current_values {
                callback(self, values);
            }
        }

        // Check for completion
        if raw_progress >= 1.0 {
            self.handle_animation_complete(&mut state);
        }

        self.state.set(state);
        true
    }

    /// Handle animation completion and looping
    fn handle_animation_complete(&mut self, state: &mut AnimationRuntimeState) {
        match self.config.loop_mode {
            LoopMode::None => {
                state.state = AnimationState::Completed;
                if let Some(callback) = &self.callbacks.on_complete {
                    callback(self);
                }
            }
            LoopMode::Infinite => {
                self.restart_animation(state);
            }
            LoopMode::Count(count) => {
                state.loops_completed += 1;
                if state.loops_completed < count {
                    self.restart_animation(state);
                } else {
                    state.state = AnimationState::Completed;
                    if let Some(callback) = &self.callbacks.on_complete {
                        callback(self);
                    }
                }
            }
            LoopMode::PingPong => {
                state.is_reversed = !state.is_reversed;
                state.current_time = Duration::ZERO;
                state.loops_completed += 1;

                if let Some(callback) = &self.callbacks.on_loop {
                    callback(self, state.loops_completed);
                }
            }
        }
    }

    /// Restart animation for looping
    fn restart_animation(&mut self, state: &mut AnimationRuntimeState) {
        state.current_time = Duration::ZERO;
        state.progress = 0.0;
        state.loops_completed += 1;

        if self.config.auto_reverse {
            state.is_reversed = !state.is_reversed;
        }

        if let Some(callback) = &self.callbacks.on_loop {
            callback(self, state.loops_completed);
        }
    }

    /// Get current animation state
    pub fn get_state(&self) -> AnimationState {
        self.state.get().state
    }

    /// Get current progress (0.0 to 1.0)
    pub fn get_progress(&self) -> f32 {
        self.state.get().progress
    }

    /// Get current animated values
    pub fn get_current_values(&self) -> Option<AnimatedValue> {
        self.state.get().current_values
    }

    /// Check if animation is playing
    pub fn is_playing(&self) -> bool {
        matches!(
            self.get_state(),
            AnimationState::Playing | AnimationState::Reversed
        )
    }

    /// Check if animation is completed
    pub fn is_completed(&self) -> bool {
        self.get_state() == AnimationState::Completed
    }

    /// Set animation speed multiplier
    pub fn set_speed(&mut self, speed: f32) {
        self.config.speed = speed.max(0.0);
    }

    /// Seek to specific progress (0.0 to 1.0)
    pub fn seek(&mut self, progress: f32) {
        let progress = progress.clamp(0.0, 1.0);
        let target_time = Duration::from_secs_f32(self.config.duration.as_secs_f32() * progress);

        self.state.update(|state| {
            state.current_time = target_time;
            state.progress = progress;
            state.current_values = Some(self.property.interpolate(progress));
        });
    }

    /// Convert to Element for layout integration
    pub fn to_element(&self) -> Element {
        Element {
            tag: "div".to_string(),
            id: Some(self.id.clone()),
            classes: vec!["animation".to_string()],
            content: Some(format!("{} ({:.1}%)", self.id, self.get_progress() * 100.0)),
            children: Vec::new(),
            attributes: HashMap::new(),
            focusable: false,
            focused: false,
            disabled: false,
            tab_index: None,
            key_bindings: Vec::new(),
            modal: false,
        }
    }
}

/// Builder for creating animations
pub struct AnimationBuilder {
    id: AnimationId,
    property: Option<AnimatedProperty>,
    config: AnimationConfig,
    callbacks: AnimationCallbacks,
}

impl AnimationBuilder {
    /// Create a new animation builder
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            property: None,
            config: AnimationConfig::default(),
            callbacks: AnimationCallbacks::default(),
        }
    }

    /// Set the property to animate
    pub fn animate_property(mut self, property: AnimatedProperty) -> Self {
        self.property = Some(property);
        self
    }

    /// Set animation duration
    pub fn duration(mut self, duration: Duration) -> Self {
        self.config.duration = duration;
        self
    }

    /// Set easing function
    pub fn easing(mut self, easing: EasingFunction) -> Self {
        self.config.easing = easing;
        self
    }

    /// Set delay before starting
    pub fn delay(mut self, delay: Duration) -> Self {
        self.config.delay = delay;
        self
    }

    /// Set loop mode
    pub fn loop_mode(mut self, loop_mode: LoopMode) -> Self {
        self.config.loop_mode = loop_mode;
        self
    }

    /// Set animation speed
    pub fn speed(mut self, speed: f32) -> Self {
        self.config.speed = speed;
        self
    }

    /// Enable auto-play
    pub fn auto_play(mut self, auto_play: bool) -> Self {
        self.config.auto_play = auto_play;
        self
    }

    /// Enable auto-reverse
    pub fn auto_reverse(mut self, auto_reverse: bool) -> Self {
        self.config.auto_reverse = auto_reverse;
        self
    }

    /// Set start callback
    pub fn on_start<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Animation) + Send + Sync + 'static,
    {
        self.callbacks.on_start = Some(Arc::new(callback));
        self
    }

    /// Set update callback
    pub fn on_update<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Animation, &AnimatedValue) + Send + Sync + 'static,
    {
        self.callbacks.on_update = Some(Arc::new(callback));
        self
    }

    /// Set complete callback
    pub fn on_complete<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Animation) + Send + Sync + 'static,
    {
        self.callbacks.on_complete = Some(Arc::new(callback));
        self
    }

    /// Set loop callback
    pub fn on_loop<F>(mut self, callback: F) -> Self
    where
        F: Fn(&Animation, u32) + Send + Sync + 'static,
    {
        self.callbacks.on_loop = Some(Arc::new(callback));
        self
    }

    /// Build the animation
    pub fn build(self) -> Animation {
        let property = self.property.unwrap_or(AnimatedProperty::Opacity(0.0, 1.0));

        let mut animation = Animation {
            id: self.id,
            property,
            config: self.config,
            state: Reactive::new(AnimationRuntimeState::default()),
            runtime: AnimationRuntime::default(),
            callbacks: self.callbacks,
            start_time: None,
        };

        if animation.config.auto_play {
            animation.play();
        }

        animation
    }
}

/// Animation Timeline for managing multiple animations
pub struct AnimationTimeline {
    /// Unique timeline identifier
    pub id: TimelineId,
    /// Animations in this timeline
    pub animations: Vec<Animation>,
    /// Timeline state
    pub state: Reactive<AnimationState>,
    /// Whether animations play in sequence or parallel
    pub sequential: bool,
    /// Current animation index (for sequential)
    current_index: usize,
}

impl AnimationTimeline {
    /// Create a new timeline
    pub fn new<S: Into<String>>(id: S, sequential: bool) -> Self {
        Self {
            id: id.into(),
            animations: Vec::new(),
            state: Reactive::new(AnimationState::Stopped),
            sequential,
            current_index: 0,
        }
    }

    /// Add an animation to the timeline
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.push(animation);
    }

    /// Play the timeline
    pub fn play(&mut self) {
        self.state.set(AnimationState::Playing);

        if self.sequential {
            if !self.animations.is_empty() {
                self.current_index = 0;
                self.animations[0].play();
            }
        } else {
            for animation in &mut self.animations {
                animation.play();
            }
        }
    }

    /// Update timeline (call in main loop)
    pub fn update(&mut self, delta_time: Duration) -> bool {
        let state = self.state.get();
        if state != AnimationState::Playing {
            return false;
        }

        if self.sequential {
            if self.current_index < self.animations.len() {
                let current_animation = &mut self.animations[self.current_index];

                if !current_animation.update(delta_time) && current_animation.is_completed() {
                    self.current_index += 1;

                    if self.current_index < self.animations.len() {
                        self.animations[self.current_index].play();
                    } else {
                        self.state.set(AnimationState::Completed);
                        return false;
                    }
                }
            }
        } else {
            let mut any_playing = false;
            for animation in &mut self.animations {
                if animation.update(delta_time) {
                    any_playing = true;
                }
            }

            if !any_playing {
                self.state.set(AnimationState::Completed);
                return false;
            }
        }

        true
    }

    /// Stop the timeline
    pub fn stop(&mut self) {
        self.state.set(AnimationState::Stopped);
        self.current_index = 0;

        for animation in &mut self.animations {
            animation.stop();
        }
    }
}

/// Animation Manager for coordinating multiple animations
pub struct AnimationManager {
    /// All managed animations
    animations: HashMap<AnimationId, Animation>,
    /// All managed timelines
    timelines: HashMap<TimelineId, AnimationTimeline>,
    /// Last update time
    last_update: Instant,
}

impl AnimationManager {
    /// Create a new animation manager
    pub fn new() -> Self {
        Self {
            animations: HashMap::new(),
            timelines: HashMap::new(),
            last_update: Instant::now(),
        }
    }

    /// Add an animation
    pub fn add_animation(&mut self, animation: Animation) {
        self.animations.insert(animation.id.clone(), animation);
    }

    /// Add a timeline
    pub fn add_timeline(&mut self, timeline: AnimationTimeline) {
        self.timelines.insert(timeline.id.clone(), timeline);
    }

    /// Update all animations (call in main loop)
    pub fn update(&mut self) {
        let now = Instant::now();
        let delta_time = now.duration_since(self.last_update);
        self.last_update = now;

        // Update standalone animations
        self.animations.retain(|_, animation| {
            animation.update(delta_time);
            !animation.is_completed()
                || matches!(
                    animation.config.loop_mode,
                    LoopMode::Infinite | LoopMode::Count(_)
                )
        });

        // Update timelines
        for timeline in self.timelines.values_mut() {
            timeline.update(delta_time);
        }
    }

    /// Get animation by ID
    pub fn get_animation(&self, id: &str) -> Option<&Animation> {
        self.animations.get(id)
    }

    /// Get mutable animation by ID
    pub fn get_animation_mut(&mut self, id: &str) -> Option<&mut Animation> {
        self.animations.get_mut(id)
    }

    /// Remove completed animations
    pub fn cleanup_completed(&mut self) {
        self.animations
            .retain(|_, animation| !animation.is_completed());
        self.timelines
            .retain(|_, timeline| timeline.state.get() != AnimationState::Completed);
    }

    /// Get active animation count
    pub fn active_count(&self) -> usize {
        self.animations.len() + self.timelines.len()
    }
}

impl Default for AnimationManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Convenience functions for common animations
/// Create a fade in animation
pub fn fade_in(id: impl Into<String>, duration: Duration) -> Animation {
    AnimationBuilder::new(id)
        .animate_property(AnimatedProperty::Opacity(0.0, 1.0))
        .duration(duration)
        .easing(EasingFunction::EaseOut)
        .build()
}

/// Create a fade out animation
pub fn fade_out(id: impl Into<String>, duration: Duration) -> Animation {
    AnimationBuilder::new(id)
        .animate_property(AnimatedProperty::Opacity(1.0, 0.0))
        .duration(duration)
        .easing(EasingFunction::EaseIn)
        .build()
}

/// Create a slide in from left animation
pub fn slide_in_left(
    id: impl Into<String>,
    from_x: i16,
    to_x: i16,
    y: i16,
    duration: Duration,
) -> Animation {
    AnimationBuilder::new(id)
        .animate_property(AnimatedProperty::Position(from_x, y, to_x, y))
        .duration(duration)
        .easing(EasingFunction::EaseOut)
        .build()
}

/// Create a bounce animation
pub fn bounce(id: impl Into<String>, duration: Duration) -> Animation {
    AnimationBuilder::new(id)
        .animate_property(AnimatedProperty::Scale(1.0, 1.2))
        .duration(duration)
        .easing(EasingFunction::Bounce)
        .loop_mode(LoopMode::PingPong)
        .build()
}

/// Create a pulse animation
pub fn pulse(id: impl Into<String>, duration: Duration) -> Animation {
    AnimationBuilder::new(id)
        .animate_property(AnimatedProperty::Opacity(1.0, 0.5))
        .duration(duration)
        .easing(EasingFunction::EaseInOut)
        .loop_mode(LoopMode::PingPong)
        .build()
}
