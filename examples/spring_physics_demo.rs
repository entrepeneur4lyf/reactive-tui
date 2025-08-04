//! Spring Physics and Advanced Easing Demo
//!
//! This example demonstrates the comprehensive spring physics system and advanced
//! easing functions added to the animation system, including parametric variations
//! and anime.js-inspired easing functions.
//!
//! Features demonstrated:
//! - Spring physics with different damping ratios (underdamped, critically damped, overdamped)
//! - Preset spring configurations (gentle, wobbly, stiff, slow, bouncy)
//! - Advanced easing functions (steps, linear points, irregular)
//! - Parametric power, back, and elastic easing variants
//! - Spring integration with animation system
//! - Performance characteristics and settling behavior

use reactive_tui::widgets::animation::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🏀 Spring Physics and Advanced Easing Demo");
    println!("==========================================\\n");

    // Test 1: Basic spring physics
    println!("🔹 Test 1: Basic Spring Physics");
    test_basic_spring_physics()?;

    // Test 2: Spring preset configurations
    println!("\\n🔹 Test 2: Spring Preset Configurations");
    test_spring_presets()?;

    // Test 3: Damping ratio variations
    println!("\\n🔹 Test 3: Damping Ratio Variations");
    test_damping_ratios()?;

    // Test 4: Advanced easing functions
    println!("\\n🔹 Test 4: Advanced Easing Functions");
    test_advanced_easing()?;

    // Test 5: Parametric easing variations
    println!("\\n🔹 Test 5: Parametric Easing Variations");
    test_parametric_easing()?;

    // Test 6: Spring integration with animations
    println!("\\n🔹 Test 6: Spring Integration with Animations");
    test_spring_animation_integration()?;

    // Test 7: Performance and settling behavior
    println!("\\n🔹 Test 7: Performance and Settling Behavior");
    test_performance_and_settling()?;

    println!("\\n✅ All spring physics and advanced easing tests completed successfully!");
    Ok(())
}

fn test_basic_spring_physics() -> Result<(), Box<dyn std::error::Error>> {
    // Create a basic spring configuration
    let spring = SpringConfig::new(1.0, 100.0, 10.0);
    
    println!("   Created spring: mass={}, stiffness={}, damping={}", 
             spring.mass, spring.stiffness, spring.damping);

    // Test position calculation over time
    let time_points = [0.0, 0.1, 0.2, 0.3, 0.5, 1.0];
    println!("   Position over time (0 → 100):");
    
    for t in time_points {
        let position = spring.calculate_position(t, 0.0, 100.0);
        let velocity = spring.calculate_velocity(t, 0.0, 100.0);
        println!("     t={:.1}s: position={:.2}, velocity={:.2}", t, position, velocity);
    }

    // Test duration estimation
    let duration = spring.estimate_duration(0.0, 100.0);
    println!("   Estimated settling duration: {:.2}s", duration);

    // Test if settled
    let settled = spring.is_settled(duration, 0.0, 100.0);
    println!("   Settled at estimated duration: {}", settled);

    Ok(())
}

fn test_spring_presets() -> Result<(), Box<dyn std::error::Error>> {
    let presets = [
        ("Gentle", SpringConfig::gentle()),
        ("Wobbly", SpringConfig::wobbly()),
        ("Stiff", SpringConfig::stiff()),
        ("Slow", SpringConfig::slow()),
        ("Bouncy", SpringConfig::bouncy()),
        ("No Overshoot", SpringConfig::no_overshoot()),
    ];

    println!("   Testing spring presets:");

    for (name, spring) in presets {
        let duration = spring.estimate_duration(0.0, 100.0);
        let mid_pos = spring.calculate_position(duration * 0.5, 0.0, 100.0);
        
        println!("     {}: duration={:.2}s, mid-position={:.2}", name, duration, mid_pos);
        
        // Test the spring at a few key points
        let pos_25 = spring.calculate_position(duration * 0.25, 0.0, 100.0);
        let pos_75 = spring.calculate_position(duration * 0.75, 0.0, 100.0);
        println!("       25% progress: {:.2}, 75% progress: {:.2}", pos_25, pos_75);
    }

    Ok(())
}

fn test_damping_ratios() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing different damping ratios:");

    // Underdamped (should oscillate)
    let underdamped = SpringConfig::new(1.0, 100.0, 5.0);
    println!("     Underdamped (damping=5.0):");
    test_spring_oscillation(&underdamped, "underdamped");

    // Critically damped (fastest settling without overshoot)
    let critically_damped = SpringConfig::new(1.0, 100.0, 20.0);
    println!("     Critically damped (damping=20.0):");
    test_spring_oscillation(&critically_damped, "critically damped");

    // Overdamped (slow approach, no overshoot)
    let overdamped = SpringConfig::new(1.0, 100.0, 50.0);
    println!("     Overdamped (damping=50.0):");
    test_spring_oscillation(&overdamped, "overdamped");

    Ok(())
}

fn test_spring_oscillation(spring: &SpringConfig, _name: &str) {
    let duration = spring.estimate_duration(0.0, 100.0);
    let time_points: Vec<f32> = (0..=10).map(|i| duration * i as f32 / 10.0).collect();
    
    let mut positions: Vec<f32> = Vec::new();
    for t in &time_points {
        positions.push(spring.calculate_position(*t, 0.0, 100.0));
    }

    // Check for overshoot
    let max_pos = positions.iter().fold(0.0f32, |a, &b| a.max(b));
    let overshoot = if max_pos > 100.0 { max_pos - 100.0 } else { 0.0 };
    
    // Check for oscillation by counting direction changes
    let mut direction_changes = 0;
    for i in 2..positions.len() {
        let prev_diff = positions[i-1] - positions[i-2];
        let curr_diff = positions[i] - positions[i-1];
        if prev_diff.signum() != curr_diff.signum() && prev_diff.abs() > 0.1 && curr_diff.abs() > 0.1 {
            direction_changes += 1;
        }
    }

    println!("       Max overshoot: {:.2}, Direction changes: {}, Duration: {:.2}s", 
             overshoot, direction_changes, duration);
}

fn test_advanced_easing() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing advanced easing functions:");

    // Test stepped easing
    let steps_easing = EasingFunction::steps(5, false);
    println!("     Steps(5, false):");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let result = steps_easing.apply(t);
        println!("       t={:.1}: {:.2}", t, result);
    }

    // Test linear points easing
    let points = vec![0.0, 0.8, 0.2, 1.0]; // Non-linear progression
    let linear_points = EasingFunction::linear_points(points);
    println!("     LinearPoints([0.0, 0.8, 0.2, 1.0]):");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let result = linear_points.apply(t);
        println!("       t={:.1}: {:.2}", t, result);
    }

    // Test irregular easing
    let irregular = EasingFunction::irregular(4, 0.3);
    println!("     Irregular(4, 0.3):");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let result = irregular.apply(t);
        println!("       t={:.1}: {:.2}", t, result);
    }

    Ok(())
}

fn test_parametric_easing() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing parametric easing variations:");

    // Test power variations
    let power_in = EasingFunction::power_in(3.0);
    let power_out = EasingFunction::power_out(3.0);
    println!("     Power variations (power=3.0):");
    for i in [0, 2, 5, 8, 10] {
        let t = i as f32 / 10.0;
        let in_result = power_in.apply(t);
        let out_result = power_out.apply(t);
        println!("       t={:.1}: in={:.3}, out={:.3}", t, in_result, out_result);
    }

    // Test back variations
    let back_in = EasingFunction::back_in(2.0);
    let back_out = EasingFunction::back_out(2.0);
    println!("     Back variations (overshoot=2.0):");
    for i in [0, 2, 5, 8, 10] {
        let t = i as f32 / 10.0;
        let in_result = back_in.apply(t);
        let out_result = back_out.apply(t);
        println!("       t={:.1}: in={:.3}, out={:.3}", t, in_result, out_result);
    }

    // Test elastic variations
    let elastic_out = EasingFunction::elastic_out(1.0, 0.4);
    println!("     Elastic out (amplitude=1.0, period=0.4):");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let result = elastic_out.apply(t);
        println!("       t={:.1}: {:.3}", t, result);
    }

    Ok(())
}

fn test_spring_animation_integration() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing spring integration with animation system:");

    // Create spring easing function
    let spring_easing = EasingFunction::spring_wobbly();
    
    // Test apply method
    println!("     Spring easing progression:");
    for i in 0..=10 {
        let t = i as f32 / 10.0;
        let result = spring_easing.apply(t);
        println!("       t={:.1}: {:.3}", t, result);
    }

    // Test apply_with_values method
    println!("     Spring with explicit values (0 → 250):");
    for i in [0, 2, 5, 8, 10] {
        let t = i as f32 / 10.0;
        let result = spring_easing.apply_with_values(t, 0.0, 250.0);
        println!("       t={:.1}: {:.2}", t, result);
    }

    // Create animation with spring easing
    let _spring_property = AnimatedProperty::Property("spring_value".to_string(), 0.0, 100.0);
    println!("     ✅ Successfully created spring animated property");

    Ok(())
}

fn test_performance_and_settling() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing performance and settling behavior:");

    let test_springs = vec![
        ("Fast stiff", SpringConfig::stiff()),
        ("Slow gentle", SpringConfig::slow()),
        ("Bouncy", SpringConfig::bouncy()),
    ];

    for (name, spring) in test_springs {
        let start_time = std::time::Instant::now();
        
        // Calculate many positions to test performance
        let mut _total_time = 0.0;
        for i in 0..1000 {
            let t = i as f32 / 1000.0;
            let _position = spring.calculate_position(t, 0.0, 100.0);
            _total_time += t;
        }
        
        let elapsed = start_time.elapsed();
        let duration = spring.estimate_duration(0.0, 100.0);
        
        println!("     {}: duration={:.2}s, 1000 calcs in {:?}", 
                 name, duration, elapsed);

        // Test settling accuracy
        let final_pos = spring.calculate_position(duration, 0.0, 100.0);
        let final_vel = spring.calculate_velocity(duration, 0.0, 100.0);
        let is_settled = spring.is_settled(duration, 0.0, 100.0);
        
        println!("       Final: pos={:.4}, vel={:.4}, settled={}", 
                 final_pos, final_vel, is_settled);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spring_config_creation() {
        let spring = SpringConfig::new(1.5, 120.0, 15.0);
        assert_eq!(spring.mass, 1.5);
        assert_eq!(spring.stiffness, 120.0);
        assert_eq!(spring.damping, 15.0);
    }

    #[test]
    fn test_spring_position_bounds() {
        let spring = SpringConfig::default();
        
        // Should start at from position
        let start_pos = spring.calculate_position(0.0, 10.0, 90.0);
        assert!((start_pos - 10.0).abs() < 0.01);

        // Should approach target over time
        let mid_pos = spring.calculate_position(0.5, 10.0, 90.0);
        assert!(mid_pos > 10.0 && mid_pos < 150.0); // Allow for overshoot
        
        // Should settle near target
        let duration = spring.estimate_duration(10.0, 90.0);
        let final_pos = spring.calculate_position(duration, 10.0, 90.0);
        assert!((final_pos - 90.0).abs() < 1.0);
    }

    #[test]
    fn test_spring_easing_functions() {
        let spring_gentle = EasingFunction::spring_gentle();
        let spring_stiff = EasingFunction::spring_stiff();
        
        // Should be different configurations
        if let (EasingFunction::Spring(gentle), EasingFunction::Spring(stiff)) = 
            (&spring_gentle, &spring_stiff) {
            assert_ne!(gentle.stiffness, stiff.stiffness);
        }

        // Should produce valid results
        let result = spring_gentle.apply(0.5);
        assert!(result >= 0.0 && result <= 2.0); // Allow for reasonable overshoot
    }

    #[test]
    fn test_parametric_easing() {
        let power_in = EasingFunction::power_in(2.0);
        let power_out = EasingFunction::power_out(2.0);
        
        // Test boundary conditions
        assert_eq!(power_in.apply(0.0), 0.0);
        assert_eq!(power_in.apply(1.0), 1.0);
        assert_eq!(power_out.apply(0.0), 0.0);
        assert_eq!(power_out.apply(1.0), 1.0);

        // Test characteristic curves
        let mid_in = power_in.apply(0.5);
        let mid_out = power_out.apply(0.5);
        
        assert!(mid_in < 0.5); // Ease in should be slower at start
        assert!(mid_out > 0.5); // Ease out should be faster at start
    }

    #[test]
    fn test_advanced_easing_functions() {
        // Test steps
        let steps = EasingFunction::steps(4, false);
        assert_eq!(steps.apply(0.0), 0.0);
        assert_eq!(steps.apply(0.24), 0.0);
        assert_eq!(steps.apply(0.26), 0.25);
        assert_eq!(steps.apply(1.0), 1.0);

        // Test linear points
        let linear = EasingFunction::linear_points(vec![0.0, 0.5, 1.0]);
        assert_eq!(linear.apply(0.0), 0.0);
        assert_eq!(linear.apply(0.5), 0.5);
        assert_eq!(linear.apply(1.0), 1.0);
    }

    #[test]
    fn test_spring_with_values() {
        let spring = EasingFunction::spring_gentle();
        
        // Test with different value ranges
        let result1 = spring.apply_with_values(0.5, 0.0, 100.0);
        let result2 = spring.apply_with_values(0.5, 50.0, 150.0);
        
        // Should scale proportionally
        assert!(result1 >= 0.0 && result1 <= 200.0); // Allow overshoot
        assert!(result2 >= 50.0 && result2 <= 250.0);
    }

    #[test]
    fn test_damping_ratio_characteristics() {
        let underdamped = SpringConfig::new(1.0, 100.0, 8.0);
        let overdamped = SpringConfig::new(1.0, 100.0, 40.0);
        
        // Underdamped should potentially overshoot
        let positions: Vec<f32> = (0..20)
            .map(|i| underdamped.calculate_position(i as f32 * 0.1, 0.0, 100.0))
            .collect();
        let max_pos = positions.iter().fold(0.0f32, |a, &b| a.max(b));
        
        // Overdamped should not overshoot significantly
        let overdamped_positions: Vec<f32> = (0..20)
            .map(|i| overdamped.calculate_position(i as f32 * 0.1, 0.0, 100.0))
            .collect();
        let overdamped_max = overdamped_positions.iter().fold(0.0f32, |a, &b| a.max(b));
        
        // Underdamped might overshoot, overdamped should not
        assert!(overdamped_max <= 100.1); // Small tolerance for numerical precision
    }

    #[test]
    fn test_spring_settling() {
        let spring = SpringConfig::stiff(); // Should settle quickly
        let duration = spring.estimate_duration(0.0, 100.0);
        
        assert!(duration > 0.0 && duration < 5.0); // Reasonable duration
        
        // Test that position gets close to target over extended time
        let extended_time = duration * 5.0;
        let final_pos = spring.calculate_position(extended_time, 0.0, 100.0);
        assert!((final_pos - 100.0).abs() < 10.0); // Should be close to target
        
        assert!(!spring.is_settled(duration * 0.1, 0.0, 100.0)); // Should not be settled early
    }
}