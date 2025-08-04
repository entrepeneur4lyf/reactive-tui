# Animation System

A comprehensive, modern animation framework with web-like API, spring physics, advanced easing, performance optimizations, and professional animation capabilities for terminal applications.

## Overview

The Animation System provides a complete solution for creating smooth, performant animations in terminal applications. It includes universal property animation, transform system, keyframe sequences, stagger animations, spring physics, and performance optimizations with batching and caching.

```rust
use reactive_tui::widgets::animation::*;
use std::time::Duration;

// Modern API - Simple fade in
let fade = animate("element", AnimateParams {
    opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
    duration: Some(500.0),
    easing: Some(EasingFunction::EaseOut),
    ..Default::default()
});

// Builder API - Complex animation
let complex = AnimationBuilder::new("complex-anim")
    .duration(Duration::from_millis(1000))
    .easing(EasingFunction::Spring(SpringConfig::bouncy()))
    .animate_property(AnimatedProperty::Transform(
        TransformProperty::Scale(0.8, 1.2)
    ))
    .build();
```

## Features

- **Universal Property Animation**: Animate any property with flexible value types
- **Transform System**: Complete transform support (translate, scale, rotate, skew, matrix)
- **Keyframe Sequences**: Multi-property keyframe animations with custom timing
- **Stagger Animations**: Sophisticated stagger system with multiple origin points
- **Spring Physics**: Realistic spring animations with presets and custom configurations
- **Advanced Easing**: 20+ easing functions including parametric variations
- **Modern API**: Web-like `animate()`, `stagger_delay()`, and `create_timeline()` functions
- **Performance Optimizations**: Animation batching, interpolation caching, and memory management
- **Timeline Management**: Sequential, parallel, and precisely-timed animation sequences
- **Reactive State Integration**: Seamless integration with reactive state system

## Architecture

### Core Components

The animation system consists of several interconnected modules:

```rust
// Core animation types
pub struct Animation {
    pub id: AnimationId,
    pub property: AnimatedProperty,
    pub config: AnimationConfig,
    pub state: Reactive<AnimationRuntimeState>,
    pub runtime: AnimationRuntime,
    pub callbacks: AnimationCallbacks,
    start_time: Option<Instant>,
}

// Universal property system
pub enum AnimatedProperty {
    // Basic properties
    Opacity(f32, f32),
    Position(i16, i16, i16, i16),
    Size(u16, u16, u16, u16),
    Color(ColorDefinition, ColorDefinition),
    Scale(f32, f32),
    Rotation(f32, f32),
    
    // Advanced properties
    Property(String, f32, f32),
    Transform(TransformProperty),
    CssProperty(String, CssValue, CssValue),
    PropertySet(Vec<PropertyAnimation>),
    Multiple(Vec<AnimatedProperty>),
    Keyframes(KeyframeSequence),
}

// Transform system
pub enum TransformProperty {
    TranslateX(f32, f32),
    TranslateY(f32, f32),
    Translate(f32, f32, f32, f32),
    ScaleX(f32, f32),
    ScaleY(f32, f32),
    Scale(f32, f32),
    Rotate(f32, f32),
    SkewX(f32, f32),
    SkewY(f32, f32),
    Matrix(TransformMatrix, TransformMatrix),
}
```

### Advanced Easing Functions

The system includes comprehensive easing support:

```rust
pub enum EasingFunction {
    // Basic easing
    Linear,
    EaseIn,
    EaseOut,
    EaseInOut,
    
    // Advanced easing
    CubicBezier(f32, f32, f32, f32),
    Bounce,
    Elastic,
    Back,
    Expo,
    Circ,
    Sine,
    Quad,
    Cubic,
    Quart,
    Quint,
    
    // Spring physics
    Spring(SpringConfig),
    
    // Advanced variations
    Steps(u32, bool),
    LinearPoints(Vec<f32>),
    Irregular(u32, f32),
    
    // Parametric variations
    InPower(f32),
    OutPower(f32),
    InOutPower(f32),
    InBack(f32),
    OutBack(f32),
    InOutBack(f32),
    InElastic(f32, f32),
    OutElastic(f32, f32),
    InOutElastic(f32, f32),
}

impl EasingFunction {
    pub fn apply(&self, t: f32) -> f32
    pub fn apply_with_values(&self, t: f32, from: f32, to: f32) -> f32
    
    // Spring presets
    pub fn spring_gentle() -> Self
    pub fn spring_wobbly() -> Self  
    pub fn spring_stiff() -> Self
    pub fn spring_slow() -> Self
    pub fn spring_bouncy() -> Self
    
    // Parametric constructors
    pub fn power_in(power: f32) -> Self
    pub fn power_out(power: f32) -> Self
    pub fn back_in(overshoot: f32) -> Self
    pub fn elastic_out(amplitude: f32, period: f32) -> Self
    pub fn steps(count: u32, jump_at_start: bool) -> Self
    pub fn linear_points(points: Vec<f32>) -> Self
}
```

### Spring Physics System

Realistic spring animations with physics-based calculations:

```rust
pub struct SpringConfig {
    pub mass: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub velocity: f32,
    pub precision: f32,
}

impl SpringConfig {
    pub fn new(mass: f32, stiffness: f32, damping: f32) -> Self
    pub fn with_velocity(mut self, velocity: f32) -> Self
    pub fn with_precision(mut self, precision: f32) -> Self
    
    // Physics calculations
    pub fn calculate_position(&self, time: f32, from: f32, to: f32) -> f32
    pub fn calculate_velocity(&self, time: f32, from: f32, to: f32) -> f32
    pub fn estimate_duration(&self, from: f32, to: f32) -> f32
    pub fn is_settled(&self, time: f32, from: f32, to: f32) -> bool
    
    // Presets
    pub fn gentle() -> Self      // Smooth, natural motion
    pub fn wobbly() -> Self      // Bouncy with oscillation
    pub fn stiff() -> Self       // Quick, snappy motion
    pub fn slow() -> Self        // Gradual, peaceful motion
    pub fn bouncy() -> Self      // High bounce effect
    pub fn no_overshoot() -> Self // Critically damped
}
```

### Keyframe System

Multi-property keyframe animations with precise timing:

```rust
pub struct Keyframe {
    pub time: f32,  // 0.0 to 1.0
    pub properties: HashMap<String, KeyframeValue>,
    pub ease: Option<EasingFunction>,
}

pub enum KeyframeValue {
    Number(f32),
    Position(i16, i16),
    Size(u16, u16),
    Color(ColorDefinition),
    Transform(TransformProperty),
    Custom(String),
}

pub struct KeyframeSequence {
    pub keyframes: Vec<Keyframe>,
    pub duration: Duration,
    pub loop_mode: LoopMode,
}

impl KeyframeSequence {
    pub fn new(duration: Duration) -> Self
    pub fn add_keyframe(&mut self, keyframe: Keyframe)
    pub fn interpolate_at(&self, progress: f32) -> HashMap<String, KeyframeValue>
    
    // Convenience constructors
    pub fn fade_in(duration_ms: u64) -> Self
    pub fn fade_out(duration_ms: u64) -> Self
    pub fn slide_in_from_left(duration_ms: u64, distance: f32) -> Self
    pub fn bounce_in(duration_ms: u64) -> Self
    pub fn pulse(duration_ms: u64) -> Self
    pub fn shake(duration_ms: u64) -> Self
}
```

### Stagger System

Advanced stagger animations with multiple calculation methods:

```rust
pub struct StaggerConfig {
    pub delay: Duration,
    pub from: StaggerOrigin,
    pub direction: StaggerDirection,
    pub ease: Option<EasingFunction>,
    pub grid: Option<(usize, usize)>,
    pub range: Option<(f32, f32)>,
}

pub enum StaggerOrigin {
    First,              // Start from first element
    Last,               // Start from last element
    Center,             // Start from center outward
    Random,             // Random starting point
    Index(usize),       // Start from specific index
    Position(i16, i16), // Start from spatial position
}

pub enum StaggerDirection {
    Normal,   // Forward direction
    Reverse,  // Reverse direction
    Random,   // Random direction per element
}

impl StaggerConfig {
    pub fn calculate_delays(&self, target_count: usize, positions: &[(i16, i16)]) -> Vec<Duration>
    
    // Builder methods
    pub fn from_center() -> StaggerBuilder
    pub fn from_last() -> StaggerBuilder
    pub fn from_index(index: usize) -> StaggerBuilder
    pub fn from_position(x: i16, y: i16) -> StaggerBuilder
    pub fn random() -> StaggerBuilder
    pub fn grid(cols: usize, rows: usize) -> StaggerBuilder
}
```

## Modern Animation API

### High-Level animate() Function

Web-like animation API for easy use:

```rust
pub fn animate<T>(targets: T, params: AnimateParams) -> Animation
where T: Into<AnimationTargets>

pub struct AnimateParams {
    // Timing and control
    pub id: Option<String>,
    pub duration: Option<f32>,
    pub delay: Option<DelayValue>,
    pub easing: Option<EasingFunction>,
    pub loop_mode: Option<LoopMode>,
    pub direction: Option<AnimationDirection>,
    pub autoplay: Option<bool>,
    
    // Keyframes
    pub keyframes: Option<KeyframeSequence>,
    
    // Common properties
    pub opacity: Option<PropertyValue>,
    pub translate_x: Option<PropertyValue>,
    pub translate_y: Option<PropertyValue>,
    pub scale: Option<PropertyValue>,
    pub rotate: Option<PropertyValue>,
    pub color: Option<ColorValue>,
    pub size: Option<SizeValue>,
    pub position: Option<PositionValue>,
    
    // Custom properties
    pub custom: Option<HashMap<String, PropertyValue>>,
    pub css: Option<HashMap<String, CssValue>>,
    pub transform: Option<HashMap<String, f32>>,
}

pub enum PropertyValue {
    Single(f32),                    // Animate to this value
    FromTo { from: f32, to: f32 },  // Explicit from-to range
    Array(Vec<f32>),                // Keyframe-like progression
    Relative(String),               // Relative change ("+50", "-25")
}
```

### Timeline Creation

Precise timeline control with positioning:

```rust
pub fn create_timeline(params: Option<TimelineParams>) -> TimelineBuilder

pub struct TimelineBuilder {
    timeline: AnimationTimeline,
    current_time: Duration,
}

impl TimelineBuilder {
    pub fn add<T>(self, targets: T, params: AnimateParams, position: Option<&str>) -> Self
    pub fn add_label(self, name: &str) -> Self
    pub fn loop_mode(self, loop_mode: LoopMode) -> Self
    pub fn build(self) -> AnimationTimeline
}

// Timeline positioning syntax
// None or "0"     - Start immediately
// "+=500"         - Start 500ms after previous ends  
// "-=200"         - Start 200ms before previous ends
// "50%"           - Start at 50% of timeline
// "1.5s"          - Start at 1.5 seconds
// "label"         - Start at named label
```

### Stagger Functions

Easy stagger creation:

```rust
pub fn stagger_delay(delay_ms: f32, options: Option<StaggerOptions>) -> StaggerConfig

pub struct StaggerOptions {
    pub from: StaggerOrigin,
    pub direction: StaggerDirection,
    pub easing: Option<EasingFunction>,
    pub grid: Option<(usize, usize)>,
    pub range: Option<(f32, f32)>,
}
```

### Convenience Functions

Pre-configured animations:

```rust
// Fade animations
pub fn fade_in<T>(targets: T, duration_ms: f32) -> Animation
pub fn fade_out<T>(targets: T, duration_ms: f32) -> Animation

// Movement animations  
pub fn slide<T>(targets: T, x: f32, y: f32, duration_ms: f32) -> Animation
pub fn scale<T>(targets: T, scale_factor: f32, duration_ms: f32) -> Animation

// Spring animations
pub fn spring_animate<T>(targets: T, property: &str, to_value: f32, spring_config: SpringConfig) -> Animation
```

## Performance Optimization System

### Animation Batching

Efficient batch processing for multiple animations:

```rust
pub struct AnimationBatch {
    animations: Vec<Animation>,
    optimization_level: OptimizationLevel,
    metrics: PerformanceMetrics,
    cache: InterpolationCache,
}

pub enum OptimizationLevel {
    None,        // No optimizations
    Basic,       // Basic batching
    Aggressive,  // Advanced optimizations
    GPU,         // GPU acceleration (future)
}

impl AnimationBatch {
    pub fn new(optimization_level: OptimizationLevel) -> Self
    pub fn add_animation(&mut self, animation: Animation)
    pub fn update_batch(&mut self, delta_time: Duration) -> Vec<BatchedUpdate>
    pub fn get_metrics(&self) -> &PerformanceMetrics
}

pub enum BatchedUpdate {
    Single(AnimationId, AnimatedValue),
    OpacityBatch(Vec<(AnimationId, f32)>),
    PositionBatch(Vec<(AnimationId, i16, i16)>),
    ColorBatch(Vec<(AnimationId, ColorDefinition)>),
    TransformBatch(Vec<(AnimationId, String, f32)>),
}
```

### Interpolation Caching

LRU cache for expensive interpolation calculations:

```rust
pub struct InterpolationCache {
    cache: HashMap<String, CachedInterpolation>,
    max_size: usize,
    hits: u64,
    misses: u64,
}

impl InterpolationCache {
    pub fn new(max_size: usize) -> Self
    pub fn get_interpolated_value(
        &mut self,
        key: &str,
        from: &AnimatedValue,
        to: &AnimatedValue,
        easing: &EasingFunction,
        progress: f32,
    ) -> AnimatedValue
    pub fn get_stats(&self) -> CacheStats
    pub fn clear_expired(&mut self)
}

pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub hit_rate: f64,
    pub cache_size: usize,
    pub max_size: usize,
}
```

### Performance Monitoring

Real-time performance analysis:

```rust
pub struct PerformanceMetrics {
    total_animations: u64,
    total_update_time: Duration,
    peak_batch_size: usize,
    update_history: Vec<(usize, Duration)>,
}

impl PerformanceMetrics {
    pub fn record_batch_update(&mut self, animation_count: usize, update_time: Duration)
    pub fn avg_time_per_animation(&self) -> Duration
    pub fn recent_avg_performance(&self) -> Option<Duration>
    pub fn get_report(&self) -> PerformanceReport
}

pub struct PerformanceReport {
    pub total_animations: u64,
    pub total_update_time: Duration,
    pub avg_time_per_animation: Duration,
    pub peak_batch_size: usize,
    pub recent_avg_performance: Option<Duration>,
}
```

### Optimized Animation Manager

Production-ready animation management:

```rust
pub struct OptimizedAnimationManager {
    batches: HashMap<OptimizationLevel, AnimationBatch>,
    individual_animations: HashMap<AnimationId, Animation>,
    global_metrics: PerformanceMetrics,
}

impl OptimizedAnimationManager {
    pub fn new() -> Self
    pub fn add_animation(&mut self, animation: Animation, optimization_level: OptimizationLevel)
    pub fn update_all(&mut self) -> Vec<BatchedUpdate>
    pub fn get_global_metrics(&self) -> &PerformanceMetrics
    pub fn get_batch_metrics(&self, level: OptimizationLevel) -> Option<&PerformanceMetrics>
}
```

## Usage Examples

### Modern API Examples

```rust
use reactive_tui::widgets::animation::*;

// Simple fade in
let fade = animate("my-element", AnimateParams {
    opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
    duration: Some(500.0),
    easing: Some(EasingFunction::EaseOut),
    ..Default::default()
});

// Complex multi-property animation
let entrance = animate("hero-element", AnimateParams {
    opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
    translate_y: Some(PropertyValue::FromTo { from: 50.0, to: 0.0 }),
    scale: Some(PropertyValue::FromTo { from: 0.95, to: 1.0 }),
    duration: Some(800.0),
    easing: Some(EasingFunction::Spring(SpringConfig::gentle())),
    ..Default::default()
});

// Keyframe animation
let keyframe_anim = animate("bouncing-ball", AnimateParams {
    translate_y: Some(PropertyValue::Array(vec![0.0, -100.0, -80.0, -60.0, 0.0])),
    scale: Some(PropertyValue::Array(vec![1.0, 0.9, 1.1, 0.95, 1.0])),
    duration: Some(1200.0),
    easing: Some(EasingFunction::Bounce),
    ..Default::default()
});
```

### Timeline Examples

```rust
// Sequential timeline
let timeline = create_timeline(Some(TimelineParams {
    id: Some("intro-sequence".to_string()),
    autoplay: Some(true),
    ..Default::default()
}))
.add("title", AnimateParams {
    opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
    duration: Some(500.0),
    ..Default::default()
}, None)
.add("subtitle", AnimateParams {
    translate_x: Some(PropertyValue::Single(100.0)),
    duration: Some(400.0),
    ..Default::default()
}, Some("-=200")) // Start 200ms before previous ends
.add("button", AnimateParams {
    scale: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
    duration: Some(300.0),
    easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
    ..Default::default()
}, None)
.build();
```

### Stagger Examples

```rust
// Basic stagger
let basic_stagger = stagger_delay(100.0, None);

// Advanced stagger from center
let center_stagger = stagger_delay(150.0, Some(StaggerOptions {
    from: StaggerOrigin::Center,
    easing: Some(EasingFunction::EaseOut),
    ..Default::default()
}));

// Grid-based stagger
let grid_stagger = stagger_delay(200.0, Some(StaggerOptions {
    from: StaggerOrigin::First,
    grid: Some((3, 3)),
    easing: Some(EasingFunction::elastic_out(1.0, 0.3)),
    ..Default::default()
}));
```

### Spring Physics Examples

```rust
use reactive_tui::widgets::animation::*;

// Spring presets
let gentle_spring = EasingFunction::spring_gentle();
let bouncy_spring = EasingFunction::spring_bouncy();
let stiff_spring = EasingFunction::spring_stiff();

// Custom spring configuration
let custom_spring = SpringConfig::new(0.8, 150.0, 12.0)
    .with_velocity(50.0)
    .with_precision(0.5);

let spring_anim = animate("spring-element", AnimateParams {
    translate_y: Some(PropertyValue::Single(-150.0)),
    easing: Some(EasingFunction::Spring(custom_spring)),
    ..Default::default()
});

// Spring with convenience function
let spring_translate = spring_animate("element", "translateX", 200.0, SpringConfig::wobbly());
```

### Performance Optimization Examples

```rust
use reactive_tui::widgets::animation::*;

// Create optimized manager
let mut manager = OptimizedAnimationManager::new();

// Add animations with different optimization levels
manager.add_animation(basic_animation, OptimizationLevel::Basic);
manager.add_animation(complex_animation, OptimizationLevel::Aggressive);

// Update all animations efficiently
let updates = manager.update_all();

// Process batched updates
for update in updates {
    match update {
        BatchedUpdate::OpacityBatch(batch) => {
            for (id, opacity) in batch {
                apply_opacity(&id, opacity);
            }
        }
        BatchedUpdate::PositionBatch(batch) => {
            for (id, x, y) in batch {
                apply_position(&id, x, y);
            }
        }
        _ => {} // Handle other batch types
    }
}

// Get performance metrics
let global_metrics = manager.get_global_metrics();
let report = global_metrics.get_report();
println!("Avg time per animation: {:?}", report.avg_time_per_animation);
```

### Transform System Examples

```rust
use reactive_tui::widgets::animation::*;

// Individual transforms
let translate = AnimationBuilder::new("translate")
    .animate_property(AnimatedProperty::Transform(
        TransformProperty::TranslateX(0.0, 100.0)
    ))
    .duration(Duration::from_millis(500))
    .build();

let scale = AnimationBuilder::new("scale") 
    .animate_property(AnimatedProperty::Transform(
        TransformProperty::Scale(1.0, 1.5)
    ))
    .duration(Duration::from_millis(400))
    .build();

// Complex transform with modern API
let complex_transform = animate("transformer", AnimateParams {
    transform: Some({
        let mut transforms = HashMap::new();
        transforms.insert("translateX".to_string(), 200.0);
        transforms.insert("translateY".to_string(), 100.0);
        transforms.insert("rotate".to_string(), 360.0);
        transforms.insert("scaleX".to_string(), 1.5);
        transforms.insert("scaleY".to_string(), 0.8);
        transforms
    }),
    duration: Some(1500.0),
    easing: Some(EasingFunction::elastic_out(1.2, 0.4)),
    ..Default::default()
});
```

### Keyframe System Examples

```rust
use reactive_tui::widgets::animation::*;

// Using keyframe builder
let keyframe_seq = KeyframeSequence::new(Duration::from_millis(2000))
    .add_keyframe(Keyframe {
        time: 0.0,
        properties: {
            let mut props = HashMap::new();
            props.insert("opacity".to_string(), KeyframeValue::Number(0.0));
            props.insert("translateX".to_string(), KeyframeValue::Number(0.0));
            props
        },
        ease: Some(EasingFunction::EaseOut),
    })
    .add_keyframe(Keyframe {
        time: 0.5,
        properties: {
            let mut props = HashMap::new();
            props.insert("opacity".to_string(), KeyframeValue::Number(1.0));
            props.insert("translateX".to_string(), KeyframeValue::Number(50.0));
            props
        },
        ease: Some(EasingFunction::EaseInOut),
    })
    .add_keyframe(Keyframe {
        time: 1.0,
        properties: {
            let mut props = HashMap::new();
            props.insert("opacity".to_string(), KeyframeValue::Number(0.8));
            props.insert("translateX".to_string(), KeyframeValue::Number(100.0));
            props
        },
        ease: None,
    });

let keyframe_animation = AnimationBuilder::new("keyframe-demo")
    .animate_property(AnimatedProperty::Keyframes(keyframe_seq))
    .build();

// Pre-built keyframe animations
let fade_in_keyframes = keyframes::fade_in(500);
let bounce_in_keyframes = keyframes::bounce_in(800);
let pulse_keyframes = keyframes::pulse(1000);
```

## Best Practices

### Performance Optimization

1. **Use Appropriate Optimization Levels**:
   - `None`: Simple animations, debugging
   - `Basic`: Most production use cases
   - `Aggressive`: High-performance requirements

2. **Batch Similar Animations**:
   ```rust
   // Good: Batch similar property animations
   manager.add_animation(opacity_anim1, OptimizationLevel::Basic);
   manager.add_animation(opacity_anim2, OptimizationLevel::Basic);
   
   // Avoid: Mixing different optimization needs
   ```

3. **Cache Expensive Calculations**:
   ```rust
   let mut cache = InterpolationCache::new(1000);
   // Cache will automatically optimize repeated interpolations
   ```

4. **Monitor Performance**:
   ```rust
   let metrics = manager.get_global_metrics();
   let report = metrics.get_report();
   if report.avg_time_per_animation > Duration::from_micros(100) {
       // Consider optimization
   }
   ```

### Animation Design

1. **Choose Appropriate Durations**:
   - UI feedback: 100-200ms
   - Transitions: 200-500ms
   - Attention: 500-1000ms
   - Ambient: 1000ms+

2. **Select Proper Easing**:
   - Entrances: `EaseOut`, `Spring`
   - Exits: `EaseIn`
   - Attention: `Bounce`, `Elastic`
   - Natural motion: `Spring` variants

3. **Use Stagger Thoughtfully**:
   ```rust
   // Good: Moderate stagger delays
   let stagger = stagger_delay(100.0, Some(StaggerOptions {
       from: StaggerOrigin::Center,
       ..Default::default()
   }));
   
   // Avoid: Excessive delays that feel slow
   ```

### Code Organization

1. **Organize by Context**:
   ```rust
   struct UIAnimations {
       entrance: AnimationTimeline,
       interactions: HashMap<String, Animation>,
       background: Vec<Animation>,
   }
   ```

2. **Use Factory Functions**:
   ```rust
   fn create_button_hover_animation(element_id: &str) -> Animation {
       animate(element_id, AnimateParams {
           scale: Some(PropertyValue::FromTo { from: 1.0, to: 1.05 }),
           duration: Some(150.0),
           easing: Some(EasingFunction::EaseOut),
           ..Default::default()
       })
   }
   ```

3. **Handle Lifecycle Properly**:
   ```rust
   impl Widget {
       fn cleanup_animations(&mut self) {
           self.manager.cleanup_completed();
           self.cache.clear_expired();
       }
   }
   ```

## Integration Examples

### With Reactive State

```rust
use reactive_tui::{widgets::animation::*, reactive::Reactive};

struct AnimatedComponent {
    opacity: Reactive<f32>,
    position: Reactive<(i16, i16)>,
    animation_manager: OptimizedAnimationManager,
}

impl AnimatedComponent {
    fn animate_to_visible(&mut self) {
        let opacity_clone = self.opacity.clone();
        let position_clone = self.position.clone();
        
        let visibility_anim = animate("visibility", AnimateParams {
            opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
            translate_y: Some(PropertyValue::FromTo { from: 20.0, to: 0.0 }),
            duration: Some(400.0),
            easing: Some(EasingFunction::Spring(SpringConfig::gentle())),
            ..Default::default()
        });
        
        // Connect animation to reactive state
        visibility_anim.on_update(move |_, value| {
            match value {
                AnimatedValue::Opacity(opacity) => opacity_clone.set(*opacity),
                AnimatedValue::Position(x, y) => position_clone.set((*x, *y)),
                _ => {}
            }
        });
        
        self.animation_manager.add_animation(visibility_anim, OptimizationLevel::Basic);
    }
}
```

### With UI Events

```rust
struct InteractiveButton {
    element_id: String,
    animations: HashMap<String, Animation>,
    manager: OptimizedAnimationManager,
}

impl InteractiveButton {
    fn on_hover_start(&mut self) {
        let hover_anim = animate(&self.element_id, AnimateParams {
            scale: Some(PropertyValue::FromTo { from: 1.0, to: 1.1 }),
            duration: Some(200.0),
            easing: Some(EasingFunction::EaseOut),
            ..Default::default()
        });
        
        self.manager.add_animation(hover_anim, OptimizationLevel::Basic);
    }
    
    fn on_click(&mut self) {
        // Create click feedback animation
        let click_timeline = create_timeline(None)
            .add(&self.element_id, AnimateParams {
                scale: Some(PropertyValue::FromTo { from: 1.1, to: 0.95 }),
                duration: Some(100.0),
                easing: Some(EasingFunction::EaseInOut),
                ..Default::default()
            }, None)
            .add(&self.element_id, AnimateParams {
                scale: Some(PropertyValue::FromTo { from: 0.95, to: 1.0 }),
                duration: Some(150.0),
                easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
                ..Default::default()
            }, None)
            .build();
            
        // Execute timeline...
    }
}
```

The Animation System provides comprehensive, production-ready animation capabilities with modern API design, advanced physics, and performance optimizations suitable for professional terminal applications.