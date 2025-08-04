//! Modern Animation API Demo
//!
//! This example demonstrates the modern, high-level animation API that provides
//! intuitive functions for creating animations with minimal boilerplate.
//!
//! Features demonstrated:
//! - animate() function with flexible parameter handling
//! - Convenience functions (fade_in, fade_out, slide, scale)
//! - Timeline creation with precise positioning
//! - Stagger configuration for sequence animations
//! - Spring-based animations with physics
//! - Complex multi-property animations
//! - Modern web-like animation patterns

use reactive_tui::widgets::animation::*;
use std::collections::HashMap;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎨 Modern Animation API Demo");
    println!("============================\\n");

    // Test 1: Basic animate() function
    println!("🔹 Test 1: Basic Animate Function");
    test_basic_animate()?;

    // Test 2: Convenience functions
    println!("\\n🔹 Test 2: Convenience Functions");
    test_convenience_functions()?;

    // Test 3: Complex multi-property animations
    println!("\\n🔹 Test 3: Complex Multi-Property Animations");
    test_complex_animations()?;

    // Test 4: Timeline creation and sequencing
    println!("\\n🔹 Test 4: Timeline Creation and Sequencing");
    test_timeline_creation()?;

    // Test 5: Stagger animations
    println!("\\n🔹 Test 5: Stagger Animations");
    test_stagger_animations()?;

    // Test 6: Spring animations
    println!("\\n🔹 Test 6: Spring Animations");
    test_spring_animations()?;

    // Test 7: Advanced timeline positioning
    println!("\\n🔹 Test 7: Advanced Timeline Positioning");
    test_advanced_timeline()?;

    println!("\\n✅ All modern animation API tests completed successfully!");
    Ok(())
}

fn test_basic_animate() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing basic animate() function:");

    // Simple fade in animation
    let fade_in = animate("fade-element", AnimateParams {
        opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
        duration: Some(500.0),
        easing: Some(EasingFunction::EaseOut),
        ..Default::default()
    });
    
    println!("     fade_in: duration={}ms, playing={}", 
             fade_in.config.duration.as_millis(), fade_in.is_playing());

    // Transform animation
    let transform = animate("transform-element", AnimateParams {
        translate_x: Some(PropertyValue::Single(100.0)),
        scale: Some(PropertyValue::FromTo { from: 0.8, to: 1.2 }),
        rotate: Some(PropertyValue::Single(45.0)),
        duration: Some(750.0),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    });
    
    println!("     transform: duration={}ms, properties=3", 
             transform.config.duration.as_millis());

    // Color animation
    let color_anim = animate("color-element", AnimateParams {
        color: Some(ColorValue::FromTo { 
            from: (255, 0, 0), 
            to: (0, 255, 0) 
        }),
        duration: Some(1000.0),
        ..Default::default()
    });
    
    println!("     color: duration={}ms, red→green", 
             color_anim.config.duration.as_millis());

    // Multiple targets
    let multi_target = animate(vec!["el1", "el2", "el3"], AnimateParams {
        opacity: Some(PropertyValue::Single(0.8)),
        duration: Some(600.0),
        ..Default::default()
    });
    
    println!("     multi_target: duration={}ms, targets=3", 
             multi_target.config.duration.as_millis());

    Ok(())
}

fn test_convenience_functions() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing convenience functions:");

    // Fade animations
    let fade_in = api_fade_in("element1", 400.0);
    let fade_out = api_fade_out("element2", 300.0);
    
    println!("     fade_in: {}ms", fade_in.config.duration.as_millis());
    println!("     fade_out: {}ms", fade_out.config.duration.as_millis());

    // Slide animation
    let slide_anim = slide("slider", 150.0, -50.0, 800.0);
    println!("     slide: {}ms, translate=(150, -50)", slide_anim.config.duration.as_millis());

    // Scale animation
    let scale_anim = scale("scaler", 1.5, 500.0);
    println!("     scale: {}ms, factor=1.5", scale_anim.config.duration.as_millis());

    // Spring animation
    let spring_anim = spring_animate("bouncer", "translateY", -100.0, SpringConfig::bouncy());
    println!("     spring: bouncy physics, translateY=-100");

    // Custom animation with multiple properties
    let mut custom_props = HashMap::new();
    custom_props.insert("blur".to_string(), PropertyValue::FromTo { from: 0.0, to: 5.0 });
    custom_props.insert("brightness".to_string(), PropertyValue::Single(1.2));
    
    let custom = animate("custom-element", AnimateParams {
        custom: Some(custom_props),
        duration: Some(900.0),
        easing: Some(EasingFunction::Elastic),
        ..Default::default()
    });
    
    println!("     custom: {}ms, blur+brightness", custom.config.duration.as_millis());

    Ok(())
}

fn test_complex_animations() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing complex multi-property animations:");

    // Complex entrance animation
    let entrance = animate("hero-element", AnimateParams {
        opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
        translate_y: Some(PropertyValue::FromTo { from: 50.0, to: 0.0 }),
        scale: Some(PropertyValue::FromTo { from: 0.95, to: 1.0 }),
        duration: Some(800.0),
        easing: Some(EasingFunction::spring_gentle()),
        ..Default::default()
    });
    
    println!("     entrance: opacity+translateY+scale, spring_gentle");

    // Array-based keyframe animation
    let keyframe_like = animate("bouncing-ball", AnimateParams {
        translate_y: Some(PropertyValue::Array(vec![0.0, -100.0, -80.0, -60.0, 0.0])),
        scale: Some(PropertyValue::Array(vec![1.0, 0.9, 1.1, 0.95, 1.0])),
        duration: Some(1200.0),
        easing: Some(EasingFunction::Bounce),
        ..Default::default()
    });
    
    println!("     keyframe_like: bounce trajectory with scaling");

    // Size and position animation
    let resize_move = animate("window", AnimateParams {
        size: Some(SizeValue::FromTo { 
            from: (200, 150), 
            to: (400, 300) 
        }),
        position: Some(PositionValue::FromTo { 
            from: (10, 10), 
            to: (50, 25) 
        }),
        duration: Some(1000.0),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    });
    
    println!("     resize_move: size+position, smooth transition");

    // Transform-heavy animation
    let mut transforms = HashMap::new();
    transforms.insert("translateX".to_string(), 200.0);
    transforms.insert("translateY".to_string(), 100.0);
    transforms.insert("rotate".to_string(), 360.0);
    transforms.insert("scaleX".to_string(), 1.5);
    transforms.insert("scaleY".to_string(), 0.8);
    
    let complex_transform = animate("transformer", AnimateParams {
        transform: Some(transforms),
        duration: Some(1500.0),
        easing: Some(EasingFunction::elastic_out(1.2, 0.4)),
        ..Default::default()
    });
    
    println!("     complex_transform: 5 transforms, elastic_out");

    println!("     ✅ Complex animations created successfully");

    Ok(())
}

fn test_timeline_creation() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing timeline creation:");

    // Basic sequential timeline
    let basic_timeline = create_timeline(Some(TimelineParams {
        id: Some("basic-sequence".to_string()),
        autoplay: Some(true),
        ..Default::default()
    }))
    .add("step1", AnimateParams {
        opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
        duration: Some(300.0),
        ..Default::default()
    }, None)
    .add("step2", AnimateParams {
        translate_x: Some(PropertyValue::Single(100.0)),
        duration: Some(400.0),
        ..Default::default()
    }, None)
    .add("step3", AnimateParams {
        scale: Some(PropertyValue::Single(1.2)),
        duration: Some(200.0),
        ..Default::default()
    }, None)
    .build();
    
    println!("     basic_timeline: 3 sequential animations, id={}", basic_timeline.id);

    // Timeline with labels and precise positioning
    let complex_timeline = create_timeline(Some(TimelineParams {
        id: Some("complex-sequence".to_string()),
        loop_mode: Some(LoopMode::PingPong),
        ..Default::default()
    }))
    .add("intro", AnimateParams {
        opacity: Some(PropertyValue::Single(1.0)),
        duration: Some(500.0),
        ..Default::default()
    }, None)
    .add_label("main-start")
    .add("main1", AnimateParams {
        translate_x: Some(PropertyValue::Single(150.0)),
        duration: Some(600.0),
        easing: Some(EasingFunction::EaseOut),
        ..Default::default()
    }, None)
    .add("main2", AnimateParams {
        translate_y: Some(PropertyValue::Single(-75.0)),
        duration: Some(400.0),
        ..Default::default()
    }, Some("-=200")) // Start 200ms before previous ends
    .add_label("finale")
    .add("outro", AnimateParams {
        opacity: Some(PropertyValue::Single(0.0)),
        scale: Some(PropertyValue::Single(0.8)),
        duration: Some(350.0),
        easing: Some(EasingFunction::EaseIn),
        ..Default::default()
    }, None)
    .build();
    
    println!("     complex_timeline: labels, overlapping, ping-pong loop");

    // Staggered timeline entries
    let stagger_timeline = create_timeline(None)
        .add("item1", AnimateParams {
            translate_y: Some(PropertyValue::FromTo { from: 20.0, to: 0.0 }),
            opacity: Some(PropertyValue::Single(1.0)),
            duration: Some(400.0),
            delay: Some(DelayValue::Fixed(0.0)),
            ..Default::default()
        }, None)
        .add("item2", AnimateParams {
            translate_y: Some(PropertyValue::FromTo { from: 20.0, to: 0.0 }),
            opacity: Some(PropertyValue::Single(1.0)),
            duration: Some(400.0),
            delay: Some(DelayValue::Fixed(100.0)),
            ..Default::default()
        }, Some("-=300"))
        .add("item3", AnimateParams {
            translate_y: Some(PropertyValue::FromTo { from: 20.0, to: 0.0 }),
            opacity: Some(PropertyValue::Single(1.0)),
            duration: Some(400.0),
            delay: Some(DelayValue::Fixed(200.0)),
            ..Default::default()
        }, Some("-=300"))
        .build();
    
    println!("     stagger_timeline: overlapping entrance sequence");

    println!("     ✅ {} timelines created successfully", 3);

    Ok(())
}

fn test_stagger_animations() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing stagger animations:");

    // Basic stagger from first
    let basic_stagger = stagger_delay(120.0, None);
    println!("     basic_stagger: 120ms delay, from first");

    // Center origin stagger
    let center_stagger = stagger_delay(80.0, Some(StaggerOptions {
        from: stagger::StaggerOrigin::Center,
        direction: stagger::StaggerDirection::Normal,
        ..Default::default()
    }));
    println!("     center_stagger: 80ms delay, from center");

    // Grid-based stagger
    let grid_stagger = stagger_delay(150.0, Some(StaggerOptions {
        from: stagger::StaggerOrigin::First,
        grid: Some((3, 3)),
        easing: Some(EasingFunction::EaseOut),
        ..Default::default()
    }));
    println!("     grid_stagger: 150ms, 3x3 grid, ease_out");

    // Random stagger
    let random_stagger = stagger_delay(100.0, Some(StaggerOptions {
        from: stagger::StaggerOrigin::Random,
        direction: stagger::StaggerDirection::Random,
        range: Some((0.5, 1.5)),
        ..Default::default()
    }));
    println!("     random_stagger: 100ms base, random origin/direction");

    // Position-based stagger
    let position_stagger = stagger_delay(200.0, Some(StaggerOptions {
        from: stagger::StaggerOrigin::Position(50, 50),
        easing: Some(EasingFunction::elastic_out(1.0, 0.3)),
        ..Default::default()
    }));
    println!("     position_stagger: from (50,50), elastic_out");

    // Test stagger delay calculation
    let test_positions = vec![(0, 0), (10, 0), (20, 0), (0, 10), (10, 10)];
    let delays = center_stagger.calculate_delays(5, &test_positions);
    
    println!("     calculated delays for 5 elements:");
    for (i, delay) in delays.iter().enumerate() {
        println!("       element {}: {}ms", i, delay.as_millis());
    }

    println!("     ✅ Stagger configurations created successfully");

    Ok(())
}

fn test_spring_animations() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing spring animations:");

    // Different spring presets
    let spring_presets = [
        ("gentle", SpringConfig::gentle()),
        ("wobbly", SpringConfig::wobbly()),
        ("stiff", SpringConfig::stiff()),
        ("slow", SpringConfig::slow()),
        ("bouncy", SpringConfig::bouncy()),
        ("no_overshoot", SpringConfig::no_overshoot()),
    ];

    for (name, config) in spring_presets {
        let duration_estimate = config.estimate_duration(0.0, 200.0);
        let _spring_anim = spring_animate("spring-element", "translateX", 200.0, config);
        
        println!("     {}: estimated_duration={:.2}s", name, duration_estimate);
    }

    // Custom spring configuration
    let custom_spring = SpringConfig::new(0.8, 150.0, 12.0)
        .with_velocity(50.0)
        .with_precision(0.5);
    
    let custom_spring_anim = animate("custom-spring", AnimateParams {
        translate_y: Some(PropertyValue::Single(-150.0)),
        easing: Some(EasingFunction::Spring(custom_spring)),
        ..Default::default()
    });
    
    println!("     custom_spring: mass=0.8, stiffness=150, damping=12, velocity=50");

    // Spring with multiple properties
    let multi_spring = animate("multi-spring", AnimateParams {
        translate_x: Some(PropertyValue::Single(100.0)),
        translate_y: Some(PropertyValue::Single(-50.0)),
        scale: Some(PropertyValue::FromTo { from: 0.9, to: 1.1 }),
        easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
        ..Default::default()
    });
    
    println!("     multi_spring: translate+scale with bouncy physics");

    // Spring timeline
    let spring_timeline = create_timeline(None)
        .add("spring1", AnimateParams {
            translate_x: Some(PropertyValue::Single(80.0)),
            easing: Some(EasingFunction::spring_gentle()),
            ..Default::default()
        }, None)
        .add("spring2", AnimateParams {
            translate_y: Some(PropertyValue::Single(-60.0)),
            easing: Some(EasingFunction::spring_wobbly()),
            ..Default::default()
        }, Some("-=100"))
        .add("spring3", AnimateParams { 
            scale: Some(PropertyValue::Single(1.3)),
            easing: Some(EasingFunction::spring_stiff()),
            ..Default::default()
        }, Some("-=50"))
        .build();
    
    println!("     spring_timeline: cascading spring animations");

    println!("     ✅ Spring animations created successfully");

    Ok(())
}

fn test_advanced_timeline() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing advanced timeline positioning:");

    // Timeline with complex positioning
    let advanced = create_timeline(Some(TimelineParams {
        id: Some("advanced-timeline".to_string()),
        ..Default::default()
    }))
    // Opening sequence
    .add("title", AnimateParams {
        opacity: Some(PropertyValue::Single(1.0)),
        translate_y: Some(PropertyValue::FromTo { from: -30.0, to: 0.0 }),
        duration: Some(600.0),
        easing: Some(EasingFunction::EaseOut),
        ..Default::default()
    }, None)
    
    // Subtitle appears 200ms before title finishes
    .add("subtitle", AnimateParams {
        opacity: Some(PropertyValue::Single(1.0)),
        translate_y: Some(PropertyValue::FromTo { from: 20.0, to: 0.0 }),
        duration: Some(400.0),
        ..Default::default()
    }, Some("-=200"))
    
    .add_label("content-start")
    
    // Content slides in from right
    .add("content", AnimateParams {
        translate_x: Some(PropertyValue::FromTo { from: 100.0, to: 0.0 }),
        opacity: Some(PropertyValue::Single(1.0)),
        duration: Some(500.0),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    }, Some("+=100")) // 100ms after previous starts
    
    // Button bounces in with spring physics
    .add("button", AnimateParams {
        scale: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
        duration: Some(800.0),
        easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
        ..Default::default()
    }, Some("-=300"))
    
    .add_label("finale")
    
    // Final flourish - everything scales slightly
    .add("all-elements", AnimateParams {
        scale: Some(PropertyValue::FromTo { from: 1.0, to: 1.02 }),
        duration: Some(200.0),
        easing: Some(EasingFunction::EaseInOut),
        ..Default::default()
    }, Some("+=200"))
    
    .loop_mode(LoopMode::None)
    .build();
    
    println!("     advanced: complex sequencing with labels and precise timing");

    // Demonstration timeline - showcasing different easing types
    let easing_showcase = create_timeline(Some(TimelineParams {
        id: Some("easing-showcase".to_string()),
        ..Default::default()
    }))
    .add("linear", AnimateParams {
        translate_x: Some(PropertyValue::Single(100.0)),
        duration: Some(500.0),
        easing: Some(EasingFunction::Linear),
        ..Default::default()
    }, None)
    .add("ease-in", AnimateParams {
        translate_x: Some(PropertyValue::Single(200.0)),
        duration: Some(500.0),
        easing: Some(EasingFunction::EaseIn),
        ..Default::default()
    }, Some("-=400"))
    .add("bounce", AnimateParams {
        translate_x: Some(PropertyValue::Single(300.0)),
        duration: Some(800.0),
        easing: Some(EasingFunction::Bounce),
        ..Default::default()
    }, Some("-=300"))
    .add("elastic", AnimateParams {
        translate_x: Some(PropertyValue::Single(400.0)),
        duration: Some(1000.0),
        easing: Some(EasingFunction::elastic_out(1.5, 0.4)),
        ..Default::default()
    }, Some("-=600"))
    .add("spring", AnimateParams {
        translate_x: Some(PropertyValue::Single(500.0)),
        easing: Some(EasingFunction::spring_wobbly()),
        ..Default::default()
    }, Some("-=700"))
    .build();
    
    println!("     easing_showcase: parallel animations with different easing");

    println!("     ✅ Advanced timelines created successfully");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_modern_api_animate() {
        let animation = animate("test", AnimateParams {
            opacity: Some(PropertyValue::Single(0.8)),
            duration: Some(500.0),
            ..Default::default()
        });
        
        assert_eq!(animation.config.duration.as_millis(), 500);
        assert!(animation.is_playing());
    }

    #[test]
    fn test_convenience_functions() {
        let fade = api_fade_in("element", 400.0);
        assert_eq!(fade.config.duration.as_millis(), 400);
        
        let slide_anim = slide("element", 50.0, 25.0, 600.0);
        assert_eq!(slide_anim.config.duration.as_millis(), 600);
    }

    #[test]
    fn test_timeline_builder() {
        let timeline = create_timeline(None)
            .add("el1", AnimateParams {
                opacity: Some(PropertyValue::Single(1.0)),
                duration: Some(300.0),
                ..Default::default()
            }, None)
            .add("el2", AnimateParams {
                translate_x: Some(PropertyValue::Single(100.0)),
                duration: Some(400.0),
                ..Default::default()
            }, None)
            .build();
        
        assert!(!timeline.animations.is_empty());
    }

    #[test]
    fn test_stagger_configuration() {
        let stagger_config = stagger_delay(100.0, Some(StaggerOptions {
            from: stagger::StaggerOrigin::Center,
            easing: Some(EasingFunction::EaseOut),
            ..Default::default()
        }));
        
        assert_eq!(stagger_config.delay.as_millis(), 100);
        assert_eq!(stagger_config.from, stagger::StaggerOrigin::Center);
    }

    #[test]
    fn test_complex_multi_property() {
        let complex = animate("complex", AnimateParams {
            opacity: Some(PropertyValue::FromTo { from: 0.0, to: 1.0 }),
            translate_x: Some(PropertyValue::Single(100.0)),
            scale: Some(PropertyValue::Array(vec![0.8, 1.2, 1.0])),
            color: Some(ColorValue::Rgb(255, 128, 0)),
            duration: Some(1000.0),
            easing: Some(EasingFunction::Spring(SpringConfig::bouncy())),
            ..Default::default()
        });
        
        assert_eq!(complex.config.duration.as_millis(), 1000);
        assert!(complex.is_playing());
    }

    #[test]
    fn test_spring_integration() {
        let spring_anim = spring_animate("spring-test", "translateY", 150.0, SpringConfig::gentle());
        assert!(spring_anim.is_playing());
    }
}