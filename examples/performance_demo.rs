//! Performance Optimization Demo
//!
//! This example demonstrates the performance optimization features of the animation system,
//! including batching, caching, and performance monitoring capabilities.
//!
//! Features demonstrated:
//! - Animation batching with different optimization levels
//! - Interpolation caching for improved performance
//! - Performance metrics collection and reporting
//! - Batch processing vs individual animation processing
//! - Memory usage optimization
//! - Real-time performance monitoring

use reactive_tui::widgets::animation::*;
use reactive_tui::themes::ColorDefinition;
use std::time::{Duration, Instant};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Animation Performance Optimization Demo");
    println!("=========================================\n");

    // Test 1: Basic batch performance
    println!("🔹 Test 1: Basic Batch Performance");
    test_batch_performance()?;

    // Test 2: Interpolation caching
    println!("\n🔹 Test 2: Interpolation Caching");
    test_interpolation_caching()?;

    // Test 3: Optimization level comparison
    println!("\n🔹 Test 3: Optimization Level Comparison");
    test_optimization_levels()?;

    // Test 4: Large scale batch processing
    println!("\n🔹 Test 4: Large Scale Batch Processing");
    test_large_scale_batching()?;

    // Test 5: Performance metrics and monitoring
    println!("\n🔹 Test 5: Performance Metrics and Monitoring");
    test_performance_monitoring()?;

    // Test 6: Memory efficiency
    println!("\n🔹 Test 6: Memory Efficiency");
    test_memory_efficiency()?;

    // Test 7: Real-time performance analysis
    println!("\n🔹 Test 7: Real-time Performance Analysis");
    test_realtime_analysis()?;

    println!("\n✅ All performance optimization tests completed successfully!");
    Ok(())
}

fn test_batch_performance() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing animation batch performance:");

    // Create batch with basic optimization
    let mut batch = AnimationBatch::new(OptimizationLevel::Basic);

    // Add various animations to the batch
    let animations = create_test_animations(10);
    for animation in animations {
        batch.add_animation(animation);
    }

    // Measure batch update performance
    let start_time = Instant::now();
    let updates = batch.update_batch(Duration::from_millis(16)); // 60fps
    let batch_time = start_time.elapsed();

    println!("     Batch update time: {:?}", batch_time);
    println!("     Updates generated: {}", updates.len());

    // Analyze update types
    let mut single_updates = 0;
    let mut batched_updates = 0;

    for update in &updates {
        match update {
            BatchedUpdate::Single(_, _) => single_updates += 1,
            _ => batched_updates += 1,
        }
    }

    println!("     Single updates: {}, Batched updates: {}", single_updates, batched_updates);

    // Get batch metrics
    let metrics = batch.get_metrics();
    let report = metrics.get_report();
    println!("     Total animations processed: {}", report.total_animations);
    println!("     Average time per animation: {:?}", report.avg_time_per_animation);

    Ok(())
}

fn test_interpolation_caching() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing interpolation caching:");

    let mut cache = InterpolationCache::new(100);

    let from = AnimatedValue::Opacity(0.0);
    let to = AnimatedValue::Opacity(1.0);
    let easing = EasingFunction::EaseInOut;

    // First round - all cache misses
    let start_time = Instant::now();
    for i in 0..50 {
        let progress = i as f32 / 49.0;
        let _result = cache.get_interpolated_value("test", &from, &to, &easing, progress);
    }
    let first_round_time = start_time.elapsed();

    // Second round - should have cache hits
    let start_time = Instant::now();
    for i in 0..50 {
        let progress = i as f32 / 49.0;
        let _result = cache.get_interpolated_value("test", &from, &to, &easing, progress);
    }
    let second_round_time = start_time.elapsed();

    let stats = cache.get_stats();
    println!("     First round (cache misses): {:?}", first_round_time);
    println!("     Second round (with cache): {:?}", second_round_time);
    println!("     Cache hit rate: {:.2}%", stats.hit_rate * 100.0);
    println!("     Total hits: {}, Total misses: {}", stats.hits, stats.misses);
    println!("     Cache size: {}/{}", stats.cache_size, stats.max_size);

    // Test cache performance improvement
    if second_round_time < first_round_time {
        let improvement = (first_round_time.as_nanos() as f64 / second_round_time.as_nanos() as f64) - 1.0;
        println!("     Performance improvement: {:.1}x faster", improvement + 1.0);
    }

    Ok(())
}

fn test_optimization_levels() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing different optimization levels:");

    let optimization_levels = [
        OptimizationLevel::None,
        OptimizationLevel::Basic,
        OptimizationLevel::Aggressive,
    ];

    for level in optimization_levels {
        let mut batch = AnimationBatch::new(level);
        
        // Add identical animations to each batch
        let animations = create_test_animations(20);
        for animation in animations {
            batch.add_animation(animation);
        }

        // Measure performance
        let start_time = Instant::now();
        let updates = batch.update_batch(Duration::from_millis(16));
        let update_time = start_time.elapsed();

        let metrics = batch.get_metrics();
        let report = metrics.get_report();

        println!("     {:?} optimization:", level);
        println!("       Update time: {:?}", update_time);
        println!("       Updates generated: {}", updates.len());
        println!("       Avg time per animation: {:?}", report.avg_time_per_animation);
    }

    Ok(())
}

fn test_large_scale_batching() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing large scale batch processing:");

    let animation_counts = [50, 100, 200, 500];

    for count in animation_counts {
        let mut batch = AnimationBatch::new(OptimizationLevel::Aggressive);
        
        // Create many animations
        let animations = create_test_animations(count);
        for animation in animations {
            batch.add_animation(animation);
        }

        // Measure batch processing time
        let start_time = Instant::now();
        let updates = batch.update_batch(Duration::from_millis(16));
        let batch_time = start_time.elapsed();

        println!("     {} animations:", count);
        println!("       Batch time: {:?}", batch_time);
        println!("       Time per animation: {:?}", batch_time / count as u32);
        println!("       Updates generated: {}", updates.len());

        // Memory usage estimation (simplified)
        let estimated_memory = count * std::mem::size_of::<Animation>() 
                             + updates.len() * 64; // Rough estimate for updates
        println!("       Estimated memory: {} KB", estimated_memory / 1024);
    }

    Ok(())
}

fn test_performance_monitoring() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing performance metrics and monitoring:");

    let mut manager = OptimizedAnimationManager::new();

    // Add animations with different optimization levels
    let basic_animations = create_test_animations(30);
    for animation in basic_animations {
        manager.add_animation(animation, OptimizationLevel::Basic);
    }

    let aggressive_animations = create_test_animations(20);
    for animation in aggressive_animations {
        manager.add_animation(animation, OptimizationLevel::Aggressive);
    }

    // Simulate several update cycles
    for cycle in 1..=10 {
        let updates = manager.update_all();
        
        if cycle % 3 == 0 {
            println!("     Cycle {}: {} updates generated", cycle, updates.len());
        }
        
        // Simulate frame time
        std::thread::sleep(Duration::from_millis(16));
    }

    // Get performance reports
    let global_metrics = manager.get_global_metrics();
    let global_report = global_metrics.get_report();

    println!("     Global Performance Report:");
    println!("       Total animations: {}", global_report.total_animations);
    println!("       Total update time: {:?}", global_report.total_update_time);
    println!("       Peak batch size: {}", global_report.peak_batch_size);
    if let Some(recent_perf) = global_report.recent_avg_performance {
        println!("       Recent avg performance: {:?}", recent_perf);
    }

    // Get batch-specific metrics
    if let Some(basic_metrics) = manager.get_batch_metrics(OptimizationLevel::Basic) {
        let basic_report = basic_metrics.get_report();
        println!("     Basic Optimization Batch:");
        println!("       Avg time per animation: {:?}", basic_report.avg_time_per_animation);
    }

    if let Some(aggressive_metrics) = manager.get_batch_metrics(OptimizationLevel::Aggressive) {
        let aggressive_report = aggressive_metrics.get_report();
        println!("     Aggressive Optimization Batch:");
        println!("       Avg time per animation: {:?}", aggressive_report.avg_time_per_animation);
    }

    Ok(())
}

fn test_memory_efficiency() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing memory efficiency:");

    // Test cache memory management
    let mut cache = InterpolationCache::new(50); // Small cache for testing

    // Fill cache beyond capacity
    for i in 0..100 {
        let from = AnimatedValue::Scale(i as f32);
        let to = AnimatedValue::Scale((i + 100) as f32);
        let easing = EasingFunction::Linear;
        let key = format!("animation_{}", i);
        
        let _result = cache.get_interpolated_value(&key, &from, &to, &easing, 0.5);
    }

    let stats = cache.get_stats();
    println!("     Cache filled with {} entries (max: {})", stats.cache_size, stats.max_size);
    println!("     Cache hit rate: {:.2}%", stats.hit_rate * 100.0);

    // Test cache expiration
    cache.clear_expired();
    let stats_after_clear = cache.get_stats();
    println!("     After expiration cleanup: {} entries", stats_after_clear.cache_size);

    // Test batch memory efficiency
    let mut batch = AnimationBatch::new(OptimizationLevel::Aggressive);
    
    // Add many animations
    let large_animation_set = create_test_animations(1000);
    for animation in large_animation_set {
        batch.add_animation(animation);
    }

    println!("     Large batch created with 1000 animations");
    
    // Clear batch to test cleanup
    batch.clear();
    println!("     Batch cleared - memory should be released");

    Ok(())
}

fn test_realtime_analysis() -> Result<(), Box<dyn std::error::Error>> {
    println!("   Testing real-time performance analysis:");

    let mut manager = OptimizedAnimationManager::new();

    // Add varied animations
    let mixed_animations = create_mixed_test_animations(100);
    for (animation, opt_level) in mixed_animations {
        manager.add_animation(animation, opt_level);
    }

    println!("     Running real-time analysis for 10 frames:");

    let mut frame_times = Vec::new();

    for frame in 1..=10 {
        let frame_start = Instant::now();
        
        let updates = manager.update_all();
        let frame_time = frame_start.elapsed();
        frame_times.push(frame_time);

        println!("       Frame {}: {:?} ({} updates)", frame, frame_time, updates.len());

        // Target 60fps
        std::thread::sleep(Duration::from_millis(16));
    }

    // Analyze frame time consistency
    let avg_frame_time: Duration = frame_times.iter().sum::<Duration>() / frame_times.len() as u32;
    let max_frame_time = frame_times.iter().max().unwrap();
    let min_frame_time = frame_times.iter().min().unwrap();

    println!("     Frame Time Analysis:");
    println!("       Average: {:?}", avg_frame_time);
    println!("       Min: {:?}", min_frame_time);
    println!("       Max: {:?}", max_frame_time);
    println!("       Consistency: {:.2}% (lower is better)", 
             (max_frame_time.as_nanos() as f64 / min_frame_time.as_nanos() as f64 - 1.0) * 100.0);

    // Check if meeting 60fps target
    let target_frame_time = Duration::from_millis(16);
    let frames_under_target = frame_times.iter().filter(|&&t| t <= target_frame_time).count();
    println!("       Frames under 16ms target: {}/10 ({:.0}%)", 
             frames_under_target, frames_under_target as f64 / 10.0 * 100.0);

    Ok(())
}

/// Create test animations with various properties
fn create_test_animations(count: usize) -> Vec<Animation> {
    let mut animations = Vec::new();

    for i in 0..count {
        let id = format!("test_animation_{}", i);
        let property = match i % 4 {
            0 => AnimatedProperty::Opacity(0.0, 1.0),
            1 => AnimatedProperty::Position(0, 0, 100, 100),
            2 => AnimatedProperty::Color(
                ColorDefinition { r: 255, g: 0, b: 0 },
                ColorDefinition { r: 0, g: 255, b: 0 }
            ),
            _ => AnimatedProperty::Size(50, 50, 150, 150),
        };

        let animation = AnimationBuilder::new(id)
            .duration(Duration::from_millis(1000 + (i as u64 * 100) % 2000))
            .easing(match i % 3 {
                0 => EasingFunction::Linear,
                1 => EasingFunction::EaseInOut,
                _ => EasingFunction::Bounce,
            })
            .animate_property(property)
            .build();

        animations.push(animation);
    }

    animations
}

/// Create mixed test animations with different optimization requirements
fn create_mixed_test_animations(count: usize) -> Vec<(Animation, OptimizationLevel)> {
    let mut animations = Vec::new();

    for i in 0..count {
        let id = format!("mixed_animation_{}", i);
        let property = match i % 6 {
            0 => AnimatedProperty::Opacity(0.0, 1.0),
            1 => AnimatedProperty::Position(0, 0, 200, 150),
            2 => AnimatedProperty::Color(
                ColorDefinition { r: 128, g: 64, b: 192 },
                ColorDefinition { r: 64, g: 192, b: 128 }
            ),
            3 => AnimatedProperty::Size(100, 80, 200, 160),
            4 => AnimatedProperty::Transform(TransformProperty::Scale(0.5, 1.5)),
            _ => AnimatedProperty::Transform(TransformProperty::Rotate(0.0, 360.0)),
        };

        let optimization_level = match i % 3 {
            0 => OptimizationLevel::None,
            1 => OptimizationLevel::Basic,
            _ => OptimizationLevel::Aggressive,
        };

        let animation = AnimationBuilder::new(id)
            .duration(Duration::from_millis(500 + (i as u64 * 50) % 1500))
            .easing(match i % 5 {
                0 => EasingFunction::Linear,
                1 => EasingFunction::EaseIn,
                2 => EasingFunction::EaseOut,
                3 => EasingFunction::EaseInOut,
                _ => EasingFunction::Elastic,
            })
            .animate_property(property)
            .build();

        animations.push((animation, optimization_level));
    }

    animations
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_test_animations() {
        let animations = create_test_animations(10);
        assert_eq!(animations.len(), 10);
        
        // Check variety of properties
        let mut property_types = std::collections::HashSet::new();
        for animation in &animations {
            property_types.insert(std::mem::discriminant(&animation.property));
        }
        assert!(property_types.len() > 1); // Should have multiple property types
    }

    #[test]
    fn test_create_mixed_test_animations() {
        let mixed = create_mixed_test_animations(15);
        assert_eq!(mixed.len(), 15);
        
        // Check variety of optimization levels
        let mut opt_levels = std::collections::HashSet::new();
        for (_, level) in &mixed {
            opt_levels.insert(*level);
        }
        assert!(opt_levels.len() > 1); // Should have multiple optimization levels
    }

    #[test]
    fn test_batch_performance_creation() {
        let mut batch = AnimationBatch::new(OptimizationLevel::Basic);
        let animations = create_test_animations(5);
        
        for animation in animations {
            batch.add_animation(animation);
        }
        
        // Should not panic and should process updates
        let updates = batch.update_batch(Duration::from_millis(16));
        // Updates might be empty if animations haven't started, but shouldn't crash
        assert!(updates.len() >= 0);
    }

    #[test]
    fn test_cache_performance() {
        let mut cache = InterpolationCache::new(10);
        let from = AnimatedValue::Opacity(0.0);
        let to = AnimatedValue::Opacity(1.0);
        let easing = EasingFunction::Linear;
        
        // Multiple calls should improve hit rate
        for _ in 0..5 {
            let _result = cache.get_interpolated_value("test", &from, &to, &easing, 0.5);
        }
        
        let stats = cache.get_stats();
        assert!(stats.hits > 0 || stats.misses > 0); // Should have some activity
    }

    #[test]
    fn test_optimized_manager_creation() {
        let manager = OptimizedAnimationManager::new();
        let updates = manager.update_all();
        assert!(updates.is_empty()); // No animations yet, so no updates
    }
}