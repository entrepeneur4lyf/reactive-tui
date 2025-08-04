//! Modern Animation API Layer
//!
//! Provides high-level convenience functions for creating animations with a modern,
//! developer-friendly interface inspired by modern web animation libraries.
//!
//! Features:
//! - `animate()` function for simple property animations
//! - `stagger()` function for creating staggered animation delays
//! - `create_timeline()` function for complex animation sequences
//! - Flexible parameter handling and intuitive API design

use super::*;
use std::collections::HashMap;
use std::time::Duration;
use serde::{Deserialize, Serialize};

/// Animation targets - can be a single ID or multiple IDs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationTargets {
    Single(String),
    Multiple(Vec<String>),
}

impl From<String> for AnimationTargets {
    fn from(id: String) -> Self {
        Self::Single(id)
    }
}

impl From<&str> for AnimationTargets {
    fn from(id: &str) -> Self {
        Self::Single(id.to_string())
    }
}

impl From<Vec<String>> for AnimationTargets {
    fn from(ids: Vec<String>) -> Self {
        Self::Multiple(ids)
    }
}

impl From<Vec<&str>> for AnimationTargets {
    fn from(ids: Vec<&str>) -> Self {
        Self::Multiple(ids.iter().map(|&s| s.to_string()).collect())
    }
}

/// Parameters for the animate function
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AnimateParams {
    /// Animation ID (auto-generated if not provided)
    pub id: Option<String>,
    /// Duration in milliseconds
    pub duration: Option<f32>,
    /// Delay before animation starts
    pub delay: Option<DelayValue>,
    /// Easing function
    pub easing: Option<EasingFunction>,
    /// Loop mode
    pub loop_mode: Option<LoopMode>,
    /// Animation direction
    pub direction: Option<AnimationDirection>,
    /// Auto-play the animation
    pub autoplay: Option<bool>,
    /// Keyframe sequence for complex animations
    pub keyframes: Option<keyframes::KeyframeSequence>,
    
    // Common property animations
    /// Opacity animation
    pub opacity: Option<PropertyValue>,
    /// X translation
    pub translate_x: Option<PropertyValue>,
    /// Y translation
    pub translate_y: Option<PropertyValue>,
    /// Scale factor
    pub scale: Option<PropertyValue>,
    /// Rotation in degrees
    pub rotate: Option<PropertyValue>,
    /// Color animation
    pub color: Option<ColorValue>,
    /// Size animation (width, height)
    pub size: Option<SizeValue>,
    /// Position animation (x, y)
    pub position: Option<PositionValue>,
    
    // Custom properties
    /// Custom numeric properties
    pub custom: Option<HashMap<String, PropertyValue>>,
    /// CSS-style properties
    pub css: Option<HashMap<String, CssValue>>,
    /// Transform properties
    pub transform: Option<HashMap<String, f32>>,
}

/// Value for animating properties
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    /// Single target value (animates from current to this value)
    Single(f32),
    /// Explicit from-to range
    FromTo { from: f32, to: f32 },
    /// Array of values for keyframe-like progression
    Array(Vec<f32>),
    /// Relative change (e.g., "+50" means add 50 to current value)
    Relative(String),
}

/// Value for color animations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorValue {
    /// RGB color
    Rgb(u8, u8, u8),
    /// RGBA color
    Rgba(u8, u8, u8, u8),
    /// From-to color animation
    FromTo { from: (u8, u8, u8), to: (u8, u8, u8) },
}

/// Value for size animations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SizeValue {
    /// Both width and height
    Both(u16, u16),
    /// From-to size animation
    FromTo { from: (u16, u16), to: (u16, u16) },
}

/// Value for position animations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PositionValue {
    /// Both x and y
    Both(i16, i16),
    /// From-to position animation
    FromTo { from: (i16, i16), to: (i16, i16) },
}

/// Delay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DelayValue {
    /// Fixed delay in milliseconds
    Fixed(f32),
    /// Staggered delay configuration
    Stagger(stagger::StaggerConfig),
}

/// Animation direction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

/// Generate a unique animation ID
fn generate_id() -> String {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(0);
    format!("anim_{}", COUNTER.fetch_add(1, Ordering::Relaxed))
}

/// Main animation creation function
///
/// Creates animations with a modern, flexible API that supports property animations,
/// keyframes, staggering, and advanced easing functions.
///
/// # Examples
///
/// ```rust
/// use reactive_tui::widgets::animation::*;
///
/// // Simple fade in
/// let fade = animate("my-element", AnimateParams {
///     opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
///     duration: Some(500.0),
///     easing: Some(EasingFunction::EaseOut),
///     ..Default::default()
/// });
///
/// // Complex transform animation
/// let transform = animate("element", AnimateParams {
///     translate_x: Some(PropertyValue::Single(100.0)),
///     scale: Some(PropertyValue::FromTo { from: 0.8, to: 1.2 }),
///     rotate: Some(PropertyValue::Single(360.0)),
///     duration: Some(1000.0),
///     easing: Some(EasingFunction::spring_wobbly()),
///     ..Default::default()
/// });
/// ```
pub fn animate<T>(targets: T, params: AnimateParams) -> Animation 
where 
    T: Into<AnimationTargets>
{
    let _targets = targets.into();
    let animation_id = params.id.clone().unwrap_or_else(generate_id);
    
    let mut builder = AnimationBuilder::new(animation_id);
    
    // Set duration
    if let Some(duration) = params.duration {
        builder = builder.duration(Duration::from_millis(duration as u64));
    }
    
    // Set delay
    if let Some(ref delay) = params.delay {
        match delay {
            DelayValue::Fixed(ms) => {
                builder = builder.delay(Duration::from_millis(*ms as u64));
            }
            DelayValue::Stagger(config) => {
                // For stagger, we'll need to handle multiple targets
                // This is a simplified version - full stagger support would need timeline
                builder = builder.delay(config.delay);
            }
        }
    }
    
    // Set easing
    if let Some(ref easing) = params.easing {
        builder = builder.easing(easing.clone());
    }
    
    // Set loop mode
    if let Some(loop_mode) = params.loop_mode {
        builder = builder.loop_mode(loop_mode);
    }
    
    // Handle keyframes if provided
    if let Some(keyframes) = params.keyframes {
        builder = builder.animate_property(AnimatedProperty::Keyframes(keyframes));
    } else {
        // Extract animated properties from parameters
        let properties = extract_animated_properties(&params);
        
        if properties.len() == 1 {
            builder = builder.animate_property(properties.into_iter().next().unwrap());
        } else if !properties.is_empty() {
            builder = builder.animate_property(AnimatedProperty::Multiple(properties));
        }
    }
    
    // Set auto-play
    let mut animation = builder.build();
    if params.autoplay.unwrap_or(true) {
        animation.play();
    }
    
    animation
}

/// Extract animated properties from parameters
fn extract_animated_properties(params: &AnimateParams) -> Vec<AnimatedProperty> {
    let mut properties = Vec::new();
    
    // Handle opacity
    if let Some(opacity) = &params.opacity {
        properties.push(convert_property_value_to_animated("opacity", opacity));
    }
    
    // Handle translation
    if let Some(translate_x) = &params.translate_x {
        properties.push(AnimatedProperty::Transform(
            convert_property_to_transform("translateX", translate_x)
        ));
    }
    
    if let Some(translate_y) = &params.translate_y {
        properties.push(AnimatedProperty::Transform(
            convert_property_to_transform("translateY", translate_y)
        ));
    }
    
    // Handle scale
    if let Some(scale) = &params.scale {
        properties.push(AnimatedProperty::Transform(
            convert_property_to_transform("scale", scale)
        ));
    }
    
    // Handle rotation
    if let Some(rotate) = &params.rotate {
        properties.push(AnimatedProperty::Transform(
            convert_property_to_transform("rotate", rotate)
        ));
    }
    
    // Handle color
    if let Some(color) = &params.color {
        properties.push(convert_color_value_to_animated(color));
    }
    
    // Handle size
    if let Some(size) = &params.size {
        properties.push(convert_size_value_to_animated(size));
    }
    
    // Handle position
    if let Some(position) = &params.position {
        properties.push(convert_position_value_to_animated(position));
    }
    
    // Handle custom properties
    if let Some(custom) = &params.custom {
        for (name, value) in custom {
            properties.push(convert_property_value_to_animated(name, value));
        }
    }
    
    // Handle CSS properties
    if let Some(css) = &params.css {
        for (name, value) in css {
            properties.push(AnimatedProperty::CssProperty(
                name.clone(),
                value.clone(),
                value.clone(), // TODO: This needs proper from/to handling
            ));
        }
    }
    
    // Handle transform properties
    if let Some(transform) = &params.transform {
        for (name, value) in transform {
            let transform_prop = match name.as_str() {
                "translateX" => TransformProperty::TranslateX(0.0, *value),
                "translateY" => TransformProperty::TranslateY(0.0, *value),
                "scale" | "scaleX" => TransformProperty::ScaleX(1.0, *value),
                "scaleY" => TransformProperty::ScaleY(1.0, *value),
                "rotate" => TransformProperty::Rotate(0.0, *value),
                "skewX" => TransformProperty::SkewX(0.0, *value),
                "skewY" => TransformProperty::SkewY(0.0, *value),
                _ => continue,
            };
            properties.push(AnimatedProperty::Transform(transform_prop));
        }
    }
    
    properties
}

/// Convert PropertyValue to AnimatedProperty
fn convert_property_value_to_animated(name: &str, value: &PropertyValue) -> AnimatedProperty {
    match value {
        PropertyValue::Single(to) => {
            // Determine the appropriate property type based on name
            match name {
                "opacity" => AnimatedProperty::Opacity(0.0, *to), // Will be overridden by current value
                _ => AnimatedProperty::Property(name.to_string(), 0.0, *to),
            }
        }
        PropertyValue::FromTo { from, to } => {
            match name {
                "opacity" => AnimatedProperty::Opacity(*from, *to),
                _ => AnimatedProperty::Property(name.to_string(), *from, *to),
            }
        }
        PropertyValue::Array(values) => {
            // Convert array to keyframe sequence
            if values.len() >= 2 {
                match name {
                    "opacity" => AnimatedProperty::Opacity(values[0], values[values.len() - 1]),
                    _ => AnimatedProperty::Property(name.to_string(), values[0], values[values.len() - 1]),
                }
            } else {
                AnimatedProperty::Property(name.to_string(), 0.0, values.first().copied().unwrap_or(0.0))
            }
        }
        PropertyValue::Relative(_rel) => {
            // TODO: Implement relative value parsing
            AnimatedProperty::Property(name.to_string(), 0.0, 0.0)
        }
    }
}

/// Convert PropertyValue to TransformProperty
fn convert_property_to_transform(transform_type: &str, value: &PropertyValue) -> TransformProperty {
    match value {
        PropertyValue::Single(to) => {
            match transform_type {
                "translateX" => TransformProperty::TranslateX(0.0, *to),
                "translateY" => TransformProperty::TranslateY(0.0, *to),
                "scale" => TransformProperty::Scale(1.0, *to),
                "rotate" => TransformProperty::Rotate(0.0, *to),
                _ => TransformProperty::TranslateX(0.0, *to),
            }
        }
        PropertyValue::FromTo { from, to } => {
            match transform_type {
                "translateX" => TransformProperty::TranslateX(*from, *to),
                "translateY" => TransformProperty::TranslateY(*from, *to),
                "scale" => TransformProperty::Scale(*from, *to),
                "rotate" => TransformProperty::Rotate(*from, *to),
                _ => TransformProperty::TranslateX(*from, *to),
            }
        }
        PropertyValue::Array(values) => {
            if values.len() >= 2 {
                let from = values[0];
                let to = values[values.len() - 1];
                match transform_type {
                    "translateX" => TransformProperty::TranslateX(from, to),
                    "translateY" => TransformProperty::TranslateY(from, to),
                    "scale" => TransformProperty::Scale(from, to),
                    "rotate" => TransformProperty::Rotate(from, to),
                    _ => TransformProperty::TranslateX(from, to),
                }
            } else {
                TransformProperty::TranslateX(0.0, values.first().copied().unwrap_or(0.0))
            }
        }
        PropertyValue::Relative(_) => {
            // TODO: Implement relative transforms
            TransformProperty::TranslateX(0.0, 0.0)
        }
    }
}

/// Convert ColorValue to AnimatedProperty
fn convert_color_value_to_animated(color: &ColorValue) -> AnimatedProperty {
    match color {
        ColorValue::Rgb(r, g, b) => {
            let to_color = ColorDefinition { r: *r, g: *g, b: *b };
            let from_color = ColorDefinition { r: 0, g: 0, b: 0 }; // Will be overridden
            AnimatedProperty::Color(from_color, to_color)
        }
        ColorValue::Rgba(r, g, b, _a) => {
            // TODO: Handle alpha channel
            let to_color = ColorDefinition { r: *r, g: *g, b: *b };
            let from_color = ColorDefinition { r: 0, g: 0, b: 0 };
            AnimatedProperty::Color(from_color, to_color)
        }
        ColorValue::FromTo { from, to } => {
            let from_color = ColorDefinition { r: from.0, g: from.1, b: from.2 };
            let to_color = ColorDefinition { r: to.0, g: to.1, b: to.2 };
            AnimatedProperty::Color(from_color, to_color)
        }
    }
}

/// Convert SizeValue to AnimatedProperty
fn convert_size_value_to_animated(size: &SizeValue) -> AnimatedProperty {
    match size {
        SizeValue::Both(w, h) => {
            AnimatedProperty::Size(0, 0, *w, *h) // Will be overridden by current size
        }
        SizeValue::FromTo { from, to } => {
            AnimatedProperty::Size(from.0, from.1, to.0, to.1)
        }
    }
}

/// Convert PositionValue to AnimatedProperty
fn convert_position_value_to_animated(position: &PositionValue) -> AnimatedProperty {
    match position {
        PositionValue::Both(x, y) => {
            AnimatedProperty::Position(0, 0, *x, *y) // Will be overridden by current position
        }
        PositionValue::FromTo { from, to } => {
            AnimatedProperty::Position(from.0, from.1, to.0, to.1)
        }
    }
}

/// Modern stagger function for creating staggered delays
///
/// # Examples
///
/// ```rust
/// use reactive_tui::widgets::animation::*;
///
/// // Basic stagger with 100ms delay
/// let stagger_config = stagger_delay(100.0, None);
///
/// // Stagger from center with easing
/// let center_stagger = stagger_delay(150.0, Some(StaggerOptions {
///     from: StaggerOrigin::Center,
///     easing: Some(EasingFunction::EaseOut),
///     ..Default::default()
/// }));
/// ```
pub fn stagger_delay(delay_ms: f32, options: Option<StaggerOptions>) -> stagger::StaggerConfig {
    let opts = options.unwrap_or_default();
    
    stagger::StaggerConfig {
        delay: Duration::from_millis(delay_ms as u64),
        from: opts.from,
        direction: opts.direction,
        ease: opts.easing,
        grid: opts.grid,
        range: opts.range,
    }
}

/// Options for stagger configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StaggerOptions {
    /// Origin point for stagger calculation
    pub from: stagger::StaggerOrigin,
    /// Direction of stagger
    pub direction: stagger::StaggerDirection,
    /// Easing function for stagger delay calculation
    pub easing: Option<EasingFunction>,
    /// Grid dimensions for 2D stagger
    pub grid: Option<(usize, usize)>,
    /// Range modifier for stagger delays
    pub range: Option<(f32, f32)>,
}

impl Default for StaggerOptions {
    fn default() -> Self {
        Self {
            from: stagger::StaggerOrigin::First,
            direction: stagger::StaggerDirection::Normal,
            easing: None,
            grid: None,
            range: None,
        }
    }
}

/// Parameters for timeline creation
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TimelineParams {
    /// Timeline ID
    pub id: Option<String>,
    /// Total timeline duration
    pub duration: Option<Duration>,
    /// Loop mode for the entire timeline
    pub loop_mode: Option<LoopMode>,
    /// Auto-play the timeline
    pub autoplay: Option<bool>,
}

/// Timeline builder for creating complex animation sequences
pub struct TimelineBuilder {
    timeline: AnimationTimeline,
    current_time: Duration,
}

impl TimelineBuilder {
    /// Create a new timeline builder
    pub fn new(id: String) -> Self {
        Self {
            timeline: AnimationTimeline::new(id, false), // non-sequential by default
            current_time: Duration::ZERO,
        }
    }
    
    /// Add an animation to the timeline
    pub fn add<T>(mut self, targets: T, params: AnimateParams, position: Option<&str>) -> Self 
    where
        T: Into<AnimationTargets>
    {
        let animation = animate(targets, params);
        
        let _timeline_position = position
            .map(parse_timeline_position)
            .unwrap_or_else(|| {
                // Default: start after previous animation
                let pos = self.current_time;
                self.current_time += animation.config.duration;
                pos
            });
        
        // For now, just add the animation without precise positioning
        // TODO: Implement precise timeline positioning
        self.timeline.add_animation(animation);
        self
    }
    
    /// Add a label at the current timeline position
    pub fn add_label(self, _name: &str) -> Self {
        // TODO: Implement timeline labels
        self
    }
    
    /// Set timeline loop mode
    pub fn loop_mode(self, _loop_mode: LoopMode) -> Self {
        // TODO: Implement timeline loop mode
        self
    }
    
    /// Build the final timeline
    pub fn build(self) -> AnimationTimeline {
        self.timeline
    }
}

/// Parse timeline position strings (e.g., "-=500", "+=200", "50%", "1.5s")
fn parse_timeline_position(position: &str) -> Duration {
    if let Some(stripped) = position.strip_prefix("-=") {
        // Relative to previous animation end, subtract time
        let ms: f32 = stripped.parse().unwrap_or(0.0);
        Duration::from_millis(ms as u64) // This would need proper context
    } else if let Some(stripped) = position.strip_prefix("+=") {
        // Relative to previous animation end, add time
        let ms: f32 = stripped.parse().unwrap_or(0.0);
        Duration::from_millis(ms as u64)
    } else if let Some(stripped) = position.strip_suffix('%') {
        // Percentage of timeline
        let percent: f32 = stripped.parse().unwrap_or(0.0);
        Duration::from_millis((percent * 10.0) as u64) // Simplified
    } else if let Some(stripped) = position.strip_suffix('s') {
        // Seconds
        let secs: f32 = stripped.parse().unwrap_or(0.0);
        Duration::from_secs_f32(secs)
    } else {
        // Assume milliseconds
        let ms: f32 = position.parse().unwrap_or(0.0);
        Duration::from_millis(ms as u64)
    }
}

/// Create a timeline with optional parameters
///
/// # Examples
///
/// ```rust
/// use reactive_tui::widgets::animation::*;
///
/// let timeline = create_timeline(Some(TimelineParams {
///     id: Some("main-timeline".to_string()),
///     autoplay: Some(true),
///     ..Default::default()
/// }))
/// .add("element1", AnimateParams {
///     opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
///     duration: Some(500.0),
///     ..Default::default()
/// }, None)
/// .add("element2", AnimateParams {
///     translate_x: Some(PropertyValue::Single(100.0)),
///     duration: Some(300.0),
///     ..Default::default()
/// }, Some("-=200")) // Start 200ms before previous ends
/// .build();
/// ```
pub fn create_timeline(params: Option<TimelineParams>) -> TimelineBuilder {
    let params = params.unwrap_or_default();
    let id = params.id.unwrap_or_else(generate_id);
    
    let mut builder = TimelineBuilder::new(id);
    
    if let Some(loop_mode) = params.loop_mode {
        builder = builder.loop_mode(loop_mode);
    }
    
    builder
}

// Convenience functions for common animations

/// Create a fade in animation
pub fn fade_in<T>(targets: T, duration_ms: f32) -> Animation 
where 
    T: Into<AnimationTargets>
{
    animate(targets, AnimateParams {
        opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
        duration: Some(duration_ms),
        easing: Some(EasingFunction::EaseOut),
        ..Default::default()
    })
}

/// Create a fade out animation
pub fn fade_out<T>(targets: T, duration_ms: f32) -> Animation 
where 
    T: Into<AnimationTargets>
{
    animate(targets, AnimateParams {
        opacity: Some(PropertyValue::FromTo { from: 1.0, to: 0.0 }),
        duration: Some(duration_ms),
        easing: Some(EasingFunction::EaseIn),
        ..Default::default()
    })
}

/// Create a slide animation
pub fn slide<T>(targets: T, x: f32, y: f32, duration_ms: f32) -> Animation 
where 
    T: Into<AnimationTargets>
{
    animate(targets, AnimateParams {
        translate_x: Some(PropertyValue::Single(x)),
        translate_y: Some(PropertyValue::Single(y)),
        duration: Some(duration_ms),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    })
}

/// Create a scale animation
pub fn scale<T>(targets: T, scale_factor: f32, duration_ms: f32) -> Animation 
where 
    T: Into<AnimationTargets>
{
    animate(targets, AnimateParams {
        scale: Some(PropertyValue::FromTo { from: 1.0, to: scale_factor }),
        duration: Some(duration_ms),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    })
}

/// Create a spring animation
pub fn spring_animate<T>(targets: T, property: &str, to_value: f32, spring_config: spring::SpringConfig) -> Animation 
where 
    T: Into<AnimationTargets>
{
    let mut custom = HashMap::new();
    custom.insert(property.to_string(), PropertyValue::Single(to_value));
    
    animate(targets, AnimateParams {
        custom: Some(custom),
        easing: Some(EasingFunction::Spring(spring_config)),
        ..Default::default()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_animate_function() {
        let animation = animate("test-element", AnimateParams {
            opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
            duration: Some(500.0),
            ..Default::default()
        });
        
        assert_eq!(animation.config.duration, Duration::from_millis(500));
        assert!(animation.is_playing());
    }

    #[test]
    fn test_animation_targets() {
        let single: AnimationTargets = "element".into();
        let multiple: AnimationTargets = vec!["el1", "el2", "el3"].into();
        
        match single {
            AnimationTargets::Single(id) => assert_eq!(id, "element"),
            _ => panic!("Expected single target"),
        }
        
        match multiple {
            AnimationTargets::Multiple(ids) => assert_eq!(ids.len(), 3),
            _ => panic!("Expected multiple targets"),
        }
    }

    #[test]
    fn test_property_value_conversion() {
        let opacity_prop = convert_property_value_to_animated("opacity", &PropertyValue::FromTo { from: 0.0, to: 1.0 });
        
        match opacity_prop {
            AnimatedProperty::Opacity(from, to) => {
                assert_eq!(from, 0.0);
                assert_eq!(to, 1.0);
            }
            _ => panic!("Expected opacity property"),
        }
    }

    #[test]
    fn test_stagger_delay() {
        let stagger_config = stagger_delay(100.0, Some(StaggerOptions {
            from: stagger::StaggerOrigin::Center,
            ..Default::default()
        }));
        
        assert_eq!(stagger_config.delay, Duration::from_millis(100));
        assert_eq!(stagger_config.from, stagger::StaggerOrigin::Center);
    }

    #[test]
    fn test_timeline_creation() {
        let timeline = create_timeline(Some(TimelineParams {
            id: Some("test-timeline".to_string()),
            ..Default::default()
        }))
        .add("element1", AnimateParams {
            opacity: Some(PropertyValue::Single(1.0)),
            duration: Some(500.0),
            ..Default::default()
        }, None)
        .add_label("middle")
        .add("element2", AnimateParams {
            translate_x: Some(PropertyValue::Single(100.0)),
            duration: Some(300.0),
            ..Default::default()
        }, None)
        .build();
        
        assert_eq!(timeline.id, "test-timeline");
        assert!(!timeline.animations.is_empty());
    }

    #[test]
    fn test_convenience_functions() {
        let fade = fade_in("element", 500.0);
        assert_eq!(fade.config.duration, Duration::from_millis(500));
        
        let slide_anim = slide("element", 100.0, 50.0, 750.0);
        assert_eq!(slide_anim.config.duration, Duration::from_millis(750));
        
        let scale_anim = scale("element", 1.5, 400.0);
        assert_eq!(scale_anim.config.duration, Duration::from_millis(400));
    }

    #[test]
    fn test_timeline_position_parsing() {
        assert_eq!(parse_timeline_position("500"), Duration::from_millis(500));
        assert_eq!(parse_timeline_position("1.5s"), Duration::from_millis(1500));
        assert_eq!(parse_timeline_position("+=200"), Duration::from_millis(200));
        assert_eq!(parse_timeline_position("-=100"), Duration::from_millis(100));
    }

    #[test]
    fn test_complex_animation() {
        let complex = animate("element", AnimateParams {
            translate_x: Some(PropertyValue::Array(vec![0.0, 50.0, 100.0])),
            scale: Some(PropertyValue::FromTo { from: 0.8, to: 1.2 }),
            color: Some(ColorValue::FromTo { 
                from: (255, 0, 0), 
                to: (0, 255, 0) 
            }),
            duration: Some(1000.0),
            easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
            ..Default::default()
        });
        
        assert_eq!(complex.config.duration, Duration::from_millis(1000));
        assert!(complex.is_playing());
    }

    #[test]
    fn test_spring_animation() {
        let spring_anim = spring_animate("element", "translateX", 100.0, spring::SpringConfig::bouncy());
        assert!(spring_anim.is_playing());
    }
}