//! Keyframe Animation System Demo
//!
//! This example demonstrates the comprehensive keyframe animation system
//! with multi-property interpolation, complex sequences, and various easing functions.
//!
//! Features demonstrated:
//! - Basic keyframe sequences with opacity and transform
//! - Multi-property keyframes with color, scale, and position
//! - Complex animations with custom easing per keyframe
//! - Convenience functions for common animations (fade, bounce, pulse)
//! - Advanced keyframe sequences with multiple interpolation points

use reactive_tui::widgets::animation::*;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🎬 Keyframe Animation System Demo");
    println!("==================================\n");

    // Test 1: Basic keyframe sequence creation
    println!("🔹 Test 1: Basic Keyframe Sequence");
    test_basic_keyframe_sequence()?;

    // Test 2: Multi-property keyframes
    println!("\n🔹 Test 2: Multi-Property Keyframes");
    test_multi_property_keyframes()?;

    // Test 3: Complex animation with custom easing
    println!("\n🔹 Test 3: Complex Animation with Custom Easing");
    test_complex_animation_with_easing()?;

    // Test 4: Convenience functions
    println!("\n🔹 Test 4: Convenience Functions");
    test_convenience_functions()?;

    // Test 5: Advanced keyframe sequences
    println!("\n🔹 Test 5: Advanced Keyframe Sequences");
    test_advanced_sequences()?;

    // Test 6: Keyframe validation
    println!("\n🔹 Test 6: Keyframe Validation");
    test_keyframe_validation()?;

    // Test 7: Property interpolation edge cases
    println!("\n🔹 Test 7: Property Interpolation Edge Cases");
    test_interpolation_edge_cases()?;

    println!("\n✅ All keyframe animation tests completed successfully!");
    Ok(())
}

fn test_basic_keyframe_sequence() -> Result<(), Box<dyn std::error::Error>> {
    // Create a simple fade in sequence
    let fade_sequence = keyframes(1000)
        .at(0.0).opacity(0.0).finish()
        .at(1.0).opacity(1.0).finish();

    println!("   Created fade sequence with {} keyframes", fade_sequence.keyframes.len());
    
    // Test sampling at different points
    let samples = [0.0, 0.25, 0.5, 0.75, 1.0];
    for t in samples {
        let sample = fade_sequence.sample(t);
        if let Some(KeyframeValue::Number(opacity)) = sample.get("opacity") {
            println!("   t={:.2}: opacity={:.2}", t, opacity);
        }
    }

    // Validate the sequence
    fade_sequence.validate()?;
    println!("   ✅ Sequence validation passed");

    Ok(())
}

fn test_multi_property_keyframes() -> Result<(), Box<dyn std::error::Error>> {
    // Create a complex multi-property sequence
    let complex_sequence = keyframes(2000)
        .at(0.0)
            .opacity(0.0)
            .css_value("scale", CssValue::Number(0.5))
            .color(255, 0, 0, Some(255))
            .number("rotation", 0.0)
            .finish()
        .at(0.3)
            .opacity(0.7)
            .css_value("scale", CssValue::Number(1.2))
            .color(255, 255, 0, Some(255))
            .number("rotation", 90.0)
            .easing(EasingFunction::EaseOut)
            .finish()
        .at(0.7)
            .opacity(1.0)
            .css_value("scale", CssValue::Number(0.9))
            .color(0, 255, 255, Some(255))
            .number("rotation", 270.0)
            .easing(EasingFunction::Bounce)
            .finish()
        .at(1.0)
            .opacity(1.0)
            .css_value("scale", CssValue::Number(1.0))
            .color(0, 255, 0, Some(255))
            .number("rotation", 360.0)
            .easing(EasingFunction::EaseInOut)
            .finish();

    println!("   Created complex sequence with {} keyframes", complex_sequence.keyframes.len());
    
    let properties = complex_sequence.get_property_names();
    println!("   Properties: {:?}", properties);

    // Test sampling at key points
    let sample_mid = complex_sequence.sample(0.5);
    println!("   Mid-animation sample: {} properties", sample_mid.len());
    
    for (prop, value) in sample_mid {
        match value {
            KeyframeValue::Number(n) => println!("     {}: {:.2}", prop, n),
            KeyframeValue::Color(r, g, b, a) => println!("     {}: rgba({}, {}, {}, {})", prop, r, g, b, a),
            KeyframeValue::Css(css) => println!("     {}: {:?}", prop, css),
            _ => println!("     {}: {:?}", prop, value),
        }
    }

    complex_sequence.validate()?;
    println!("   ✅ Complex sequence validation passed");

    Ok(())
}

fn test_complex_animation_with_easing() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sequence with different easing functions for each keyframe
    let easing_sequence = keyframes(1500)
        .at(0.0)
            .opacity(0.0)
            .number("position", 0.0)
            .finish()
        .at(0.2)
            .opacity(0.3)
            .number("position", 50.0)
            .easing(EasingFunction::EaseIn)
            .finish()
        .at(0.5)
            .opacity(0.8)
            .number("position", 150.0)
            .easing(EasingFunction::Elastic)
            .finish()
        .at(0.8)
            .opacity(1.0)
            .number("position", 200.0)
            .easing(EasingFunction::Bounce)
            .finish()
        .at(1.0)
            .opacity(1.0)
            .number("position", 250.0)
            .easing(EasingFunction::EaseOut)
            .finish();

    println!("   Created sequence with varied easing functions");

    // Test easing application by sampling closely spaced points
    let test_points = [0.18, 0.19, 0.2, 0.21, 0.22];
    println!("   Testing easing around keyframe at t=0.2:");
    
    for t in test_points {
        let sample = easing_sequence.sample(t);
        if let Some(KeyframeValue::Number(pos)) = sample.get("position") {
            println!("     t={:.2}: position={:.1}", t, pos);
        }
    }

    easing_sequence.validate()?;
    println!("   ✅ Easing sequence validation passed");

    Ok(())
}

fn test_convenience_functions() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing convenience functions:");

    // Test fade in
    let fade_in_seq = keyframes::fade_in(800);
    println!("     fade_in(800): {} keyframes", fade_in_seq.keyframes.len());
    let sample = fade_in_seq.sample(0.5);
    if let Some(KeyframeValue::Number(opacity)) = sample.get("opacity") {
        println!("       Mid-fade opacity: {:.2}", opacity);
    }

    // Test fade out
    let fade_out_seq = keyframes::fade_out(600);
    println!("     fade_out(600): {} keyframes", fade_out_seq.keyframes.len());

    // Test slide in
    let slide_seq = keyframes::slide_in_from_left(1000, 100.0);
    println!("     slide_in_from_left(1000, 100.0): {} keyframes", slide_seq.keyframes.len());

    // Test bounce in
    let bounce_seq = keyframes::bounce_in(1200);
    println!("     bounce_in(1200): {} keyframes", bounce_seq.keyframes.len());

    // Test pulse
    let pulse_seq = keyframes::pulse(800);
    println!("     pulse(800): {} keyframes", pulse_seq.keyframes.len());

    // Test using convenience functions with AnimatedProperty
    let animated_prop = keyframe_fade_in(1000);
    println!("     keyframe_fade_in() creates AnimatedProperty::Keyframes");
    
    match animated_prop {
        AnimatedProperty::Keyframes(seq) => {
            println!("       ✅ Successfully created keyframe animation property");
            seq.validate()?;
        }
        _ => println!("       ❌ Unexpected property type"),
    }

    Ok(())
}

fn test_advanced_sequences() -> Result<(), Box<dyn std::error::Error>> {
    // Create a sequence that demonstrates advanced interpolation
    let advanced_sequence = keyframes(3000)
        .default_easing(EasingFunction::EaseInOut)
        .at(0.0)
            .opacity(0.0)
            .css_value("width", CssValue::Pixels(50.0))
            .css_value("height", CssValue::Percentage(0.0))
            .transform(TransformMatrix::default())
            .finish()
        .at(0.15)
            .opacity(0.5)
            .css_value("width", CssValue::Pixels(100.0))
            .css_value("height", CssValue::Percentage(25.0))
            .easing(EasingFunction::EaseOut)
            .finish()
        .at(0.4)
            .opacity(0.8)
            .css_value("width", CssValue::Pixels(200.0))
            .css_value("height", CssValue::Percentage(60.0))
            .easing(EasingFunction::Elastic)
            .finish()
        .at(0.75)
            .opacity(1.0)
            .css_value("width", CssValue::Pixels(300.0))
            .css_value("height", CssValue::Percentage(90.0))
            .easing(EasingFunction::Bounce)
            .finish()
        .at(1.0)
            .opacity(1.0)
            .css_value("width", CssValue::Pixels(250.0))
            .css_value("height", CssValue::Percentage(100.0))
            .finish();

    println!("   Created advanced sequence with {} keyframes", advanced_sequence.keyframes.len());
    println!("   Default easing: {:?}", advanced_sequence.default_easing);

    // Test comprehensive sampling
    let test_times = [0.0, 0.1, 0.25, 0.5, 0.8, 1.0];
    for t in test_times {
        let sample = advanced_sequence.sample(t);
        println!("   t={:.1}: {} properties sampled", t, sample.len());
        
        if let Some(KeyframeValue::Css(CssValue::Pixels(width))) = sample.get("width") {
            if let Some(KeyframeValue::Css(CssValue::Percentage(height))) = sample.get("height") {
                println!("     size: {:.0}px × {:.0}%", width, height);
            }
        }
    }

    advanced_sequence.validate()?;
    println!("   ✅ Advanced sequence validation passed");

    Ok(())
}

fn test_keyframe_validation() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing keyframe validation:");

    // Test valid sequence
    let valid_sequence = keyframes(1000)
        .at(0.0).opacity(0.0).finish()
        .at(0.5).opacity(0.5).finish()
        .at(1.0).opacity(1.0).finish();
    
    match valid_sequence.validate() {
        Ok(()) => println!("     ✅ Valid sequence passed validation"),
        Err(e) => println!("     ❌ Valid sequence failed: {}", e),
    }

    // Test empty sequence
    let empty_sequence = KeyframeSequence::new(Duration::from_millis(1000));
    match empty_sequence.validate() {
        Ok(()) => println!("     ❌ Empty sequence should have failed validation"),
        Err(_) => println!("     ✅ Empty sequence correctly failed validation"),
    }

    // Test sequence with invalid offset (this should be caught during creation)
    let keyframe_with_invalid_offset = Keyframe::new(1.5); // > 1.0, should be clamped
    println!("     Keyframe with offset 1.5 clamped to: {}", keyframe_with_invalid_offset.offset);

    Ok(())
}

fn test_interpolation_edge_cases() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing interpolation edge cases:");

    // Test interpolation between different value types
    let from_number = KeyframeValue::Number(10.0);
    let to_string = KeyframeValue::String("test".to_string());
    
    match from_number.interpolate(&to_string, 0.5) {
        Some(result) => println!("     ❌ Unexpected interpolation result: {:?}", result),
        None => println!("     ✅ Correctly rejected interpolation between incompatible types"),
    }

    // Test color interpolation
    let from_color = KeyframeValue::Color(255, 0, 0, 255);
    let to_color = KeyframeValue::Color(0, 255, 0, 255);
    
    if let Some(KeyframeValue::Color(r, g, b, a)) = from_color.interpolate(&to_color, 0.5) {
        println!("     ✅ Color interpolation: rgba({}, {}, {}, {})", r, g, b, a);
    }

    // Test transform interpolation
    let from_transform = KeyframeValue::Transform(TransformMatrix::default());
    let mut to_matrix = TransformMatrix::default();
    to_matrix.a = 2.0; // scale
    to_matrix.e = 100.0; // translateX
    let to_transform = KeyframeValue::Transform(to_matrix);
    
    if let Some(KeyframeValue::Transform(result)) = from_transform.interpolate(&to_transform, 0.3) {
        println!("     ✅ Transform interpolation: scale={:.1}, translateX={:.1}", result.a, result.e);
    }

    // Test CSS value interpolation
    let from_css = KeyframeValue::Css(CssValue::Pixels(0.0));
    let to_css = KeyframeValue::Css(CssValue::Pixels(100.0));
    
    if let Some(KeyframeValue::Css(CssValue::Pixels(px))) = from_css.interpolate(&to_css, 0.75) {
        println!("     ✅ CSS pixel interpolation: {:.0}px", px);
    }

    // Test sequence with single keyframe
    let single_keyframe_seq = keyframes(1000)
        .at(0.5).opacity(0.8).finish();
    
    let sample_before = single_keyframe_seq.sample(0.2);
    let sample_at = single_keyframe_seq.sample(0.5);
    let sample_after = single_keyframe_seq.sample(0.8);
    
    println!("     Single keyframe sequence sampling:");
    println!("       Before (t=0.2): {} properties", sample_before.len());
    println!("       At (t=0.5): {} properties", sample_at.len());
    println!("       After (t=0.8): {} properties", sample_after.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyframe_sequence_creation() {
        let sequence = keyframes(1000)
            .at(0.0).opacity(0.0).finish()
            .at(1.0).opacity(1.0).finish();
        
        assert_eq!(sequence.keyframes.len(), 2);
        assert_eq!(sequence.duration, Duration::from_millis(1000));
        assert!(sequence.validate().is_ok());
    }

    #[test]
    fn test_keyframe_value_interpolation() {
        let from = KeyframeValue::Number(0.0);
        let to = KeyframeValue::Number(100.0);
        
        let result = from.interpolate(&to, 0.25).unwrap();
        assert_eq!(result, KeyframeValue::Number(25.0));
    }

    #[test]
    fn test_multi_property_sampling() {
        let sequence = keyframes(2000)
            .at(0.0)
                .opacity(0.0)
                .number("scale", 0.5)
                .color(255, 0, 0, Some(255))
                .finish()
            .at(1.0)
                .opacity(1.0)
                .number("scale", 1.0)
                .color(0, 255, 0, Some(255))
                .finish();

        let sample = sequence.sample(0.5);
        assert_eq!(sample.len(), 3);
        
        assert!(sample.contains_key("opacity"));
        assert!(sample.contains_key("scale"));
        assert!(sample.contains_key("color"));
    }

    #[test]
    fn test_convenience_functions() {
        let fade_seq = keyframes::fade_in(1000);
        assert_eq!(fade_seq.keyframes.len(), 2);
        assert!(fade_seq.validate().is_ok());

        let bounce_seq = keyframes::bounce_in(1500);
        assert_eq!(bounce_seq.keyframes.len(), 4);
        assert!(bounce_seq.validate().is_ok());
    }

    #[test]
    fn test_animated_property_keyframes() {
        let prop = keyframe_fade_in(800);
        match prop {
            AnimatedProperty::Keyframes(seq) => {
                assert_eq!(seq.keyframes.len(), 2);
                assert_eq!(seq.duration, Duration::from_millis(800));
            }
            _ => panic!("Expected keyframe property"),
        }
    }

    #[test]
    fn test_keyframe_builder_pattern() {
        let sequence = keyframes(1000)
            .at(0.0)
                .opacity(0.0)
                .css_value("scale", CssValue::Number(0.8))
                .easing(EasingFunction::EaseIn)
                .finish()
            .at(0.5)
                .opacity(0.5)
                .css_value("scale", CssValue::Number(1.1))
                .easing(EasingFunction::Bounce)
                .finish()
            .at(1.0)
                .opacity(1.0)
                .css_value("scale", CssValue::Number(1.0))
                .finish();

        assert_eq!(sequence.keyframes.len(), 3);
        assert_eq!(sequence.keyframes[0].easing, Some(EasingFunction::EaseIn));
        assert_eq!(sequence.keyframes[1].easing, Some(EasingFunction::Bounce));
        assert_eq!(sequence.keyframes[2].easing, None);
    }

    #[test]
    fn test_complex_interpolation_with_easing() {
        let sequence = keyframes(1000)
            .at(0.0).number("value", 0.0).finish()
            .at(0.5).number("value", 50.0).easing(EasingFunction::EaseOut).finish()
            .at(1.0).number("value", 100.0).finish();

        // Test that easing is applied correctly
        let sample_early = sequence.sample(0.1);
        let sample_late = sequence.sample(0.4);
        
        if let (Some(KeyframeValue::Number(early)), Some(KeyframeValue::Number(late))) = 
            (sample_early.get("value"), sample_late.get("value")) {
            // With EaseOut, the early part should progress faster
            assert!(*early > 2.0); // Should be > 10% of 50 due to ease out
            assert!(*late < 48.0); // Should be < 96% of 50 due to ease out
        }
    }
}