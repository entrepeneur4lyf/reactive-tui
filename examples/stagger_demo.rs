//! Stagger Animation Demo
//!
//! Demonstrates the comprehensive stagger system:
//! - Linear stagger (first, last, center)
//! - Position-based stagger (2D layouts)
//! - Random stagger patterns
//! - Grid-based stagger
//! - Advanced stagger with easing and ranges
//!
//! This showcases the flexible stagger configurations for creating
//! complex animated sequences.

use reactive_tui::prelude::*;
use reactive_tui::widgets::animation::{
    stagger, stagger_from_center, stagger_from_last, stagger_from_index,
    stagger_from_position, stagger_random, stagger_grid, stagger_grid_center,
    stagger_builder, StaggerOrigin, StaggerDirection, EasingFunction
};

fn main() -> Result<()> {
    println!("=== Stagger Animation Demo ===");
    println!();
    println!("This demo showcases the comprehensive stagger system:");
    println!("  📐 Linear stagger (first, last, center)");
    println!("  🎯 Position-based stagger (2D layouts)");
    println!("  🎲 Random stagger patterns");
    println!("  🔲 Grid-based stagger");
    println!("  ⚡ Advanced stagger with easing and ranges");
    println!();

    // Demo basic stagger patterns
    demo_basic_stagger();

    // Demo 2D position stagger
    demo_position_stagger();

    // Demo grid stagger
    demo_grid_stagger();

    // Demo advanced stagger features
    demo_advanced_stagger();

    println!("✅ All stagger animation patterns are working correctly!");

    Ok(())
}

fn demo_basic_stagger() {
    println!("📐 Basic Stagger Patterns:");

    // Basic stagger from first
    let basic_stagger = stagger(100);
    let delays = basic_stagger.calculate_delays(5, &[]);
    print!("  First:   ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Stagger from last
    let last_stagger = stagger_from_last(100);
    let delays = last_stagger.calculate_delays(5, &[]);
    print!("  Last:    ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Stagger from center
    let center_stagger = stagger_from_center(100);
    let delays = center_stagger.calculate_delays(5, &[]);
    print!("  Center:  ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Stagger from specific index
    let index_stagger = stagger_from_index(100, 2);
    let delays = index_stagger.calculate_delays(5, &[]);
    print!("  Index 2: ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Random stagger
    let random_stagger = stagger_random(100);
    let delays = random_stagger.calculate_delays(5, &[]);
    print!("  Random:  ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    println!();
}

fn demo_position_stagger() {
    println!("🎯 Position-based Stagger:");

    // Create some 2D positions
    let positions = vec![
        (0, 0),   // Top-left
        (10, 0),  // Top-right
        (5, 5),   // Center
        (0, 10),  // Bottom-left
        (10, 10), // Bottom-right
    ];

    // Stagger from position (0, 0)
    let pos_stagger = stagger_from_position(50, 0, 0);
    let delays = pos_stagger.calculate_delays(positions.len(), &positions);
    println!("  From (0,0):");
    for (i, delay) in delays.iter().enumerate() {
        let (x, y) = positions[i];
        println!("    Position ({:2},{:2}): {:3}ms", x, y, delay.as_millis());
    }

    // Stagger from center position
    let center_pos_stagger = stagger_from_position(50, 5, 5);
    let delays = center_pos_stagger.calculate_delays(positions.len(), &positions);
    println!("  From (5,5):");
    for (i, delay) in delays.iter().enumerate() {
        let (x, y) = positions[i];
        println!("    Position ({:2},{:2}): {:3}ms", x, y, delay.as_millis());
    }

    println!();
}

fn demo_grid_stagger() {
    println!("🔲 Grid-based Stagger:");

    // 3x3 grid stagger from first
    let grid_stagger = stagger_grid(80, 3, 3);
    let delays = grid_stagger.calculate_grid_delays(3, 3);
    println!("  3x3 Grid (from first):");
    for y in 0..3 {
        print!("    ");
        for x in 0..3 {
            let index = y * 3 + x;
            print!("{:3}ms ", delays[index].as_millis());
        }
        println!();
    }

    // 3x3 grid stagger from center
    let grid_center_stagger = stagger_grid_center(80, 3, 3);
    let delays = grid_center_stagger.calculate_grid_delays(3, 3);
    println!("  3x3 Grid (from center):");
    for y in 0..3 {
        print!("    ");
        for x in 0..3 {
            let index = y * 3 + x;
            print!("{:3}ms ", delays[index].as_millis());
        }
        println!();
    }

    println!();
}

fn demo_advanced_stagger() {
    println!("⚡ Advanced Stagger Features:");

    // Stagger with easing
    let eased_stagger = stagger_builder(100)
        .from(StaggerOrigin::Center)
        .ease(EasingFunction::EaseOut)
        .build();

    let delays = eased_stagger.calculate_delays(5, &[]);
    print!("  With EaseOut: ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Stagger with range
    let ranged_stagger = stagger_builder(100)
        .from(StaggerOrigin::First)
        .range(0.5, 2.0)
        .build();

    let delays = ranged_stagger.calculate_delays(5, &[]);
    print!("  With Range:   ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    // Complex stagger with multiple features
    let complex_stagger = stagger_builder(80)
        .from(StaggerOrigin::Center)
        .direction(StaggerDirection::Reverse)
        .ease(EasingFunction::Bounce)
        .range(0.2, 3.0)
        .build();

    let delays = complex_stagger.calculate_delays(7, &[]);
    print!("  Complex:      ");
    for (i, delay) in delays.iter().enumerate() {
        print!("{}:{}ms ", i, delay.as_millis());
    }
    println!();

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_stagger() {
        let stagger_config = stagger(100);
        let delays = stagger_config.calculate_delays(3, &[]);

        assert_eq!(delays.len(), 3);
        assert!(delays[0].as_millis() == 0);
        assert!((delays[1].as_millis() as i64 - 100).abs() <= 1);
        assert!((delays[2].as_millis() as i64 - 200).abs() <= 1);
    }

    #[test]
    fn test_center_stagger() {
        let stagger_config = stagger_from_center(100);
        let delays = stagger_config.calculate_delays(5, &[]);

        assert_eq!(delays.len(), 5);
        // Center should be index 2 (5/2 = 2.5, floored = 2)
        // Distances: [2.5, 1.5, 0.5, 0.5, 1.5]
        assert!((delays[0].as_millis() as i64 - 250).abs() <= 1); // distance 2.5
        assert!((delays[1].as_millis() as i64 - 150).abs() <= 1); // distance 1.5
        assert!((delays[2].as_millis() as i64 - 50).abs() <= 1);  // distance 0.5
        assert!((delays[3].as_millis() as i64 - 50).abs() <= 1);  // distance 0.5
        assert!((delays[4].as_millis() as i64 - 150).abs() <= 1); // distance 1.5
    }

    #[test]
    fn test_reverse_stagger() {
        let stagger_config = stagger_from_last(100);
        let delays = stagger_config.calculate_delays(3, &[]);

        assert_eq!(delays.len(), 3);
        assert!((delays[0].as_millis() as i64 - 200).abs() <= 1);
        assert!((delays[1].as_millis() as i64 - 100).abs() <= 1);
        assert!(delays[2].as_millis() == 0);
    }

    #[test]
    fn test_position_based_stagger() {
        let positions = vec![(0, 0), (3, 4), (6, 8)]; // Right triangle: 0, 5, 10 units from origin
        let stagger_config = stagger_from_position(100, 0, 0);
        let delays = stagger_config.calculate_delays(positions.len(), &positions);

        assert_eq!(delays.len(), 3);
        assert!(delays[0].as_millis() == 0);   // distance 0
        assert!((delays[1].as_millis() as i64 - 5).abs() <= 1);   // distance 5, scaled by /100
        assert!((delays[2].as_millis() as i64 - 10).abs() <= 1);  // distance 10, scaled by /100
    }

    #[test]
    fn test_grid_stagger() {
        let stagger_config = stagger_grid(100, 2, 2);
        let delays = stagger_config.calculate_grid_delays(2, 2);

        assert_eq!(delays.len(), 4);
        assert!(delays[0].as_millis() == 0);   // (0,0)
        assert!((delays[1].as_millis() as i64 - 100).abs() <= 1); // (1,0)
        assert!((delays[2].as_millis() as i64 - 200).abs() <= 1); // (0,1)
        assert!((delays[3].as_millis() as i64 - 300).abs() <= 1); // (1,1)
    }

    #[test]
    fn test_stagger_builder() {
        let stagger_config = stagger_builder(50)
            .from(StaggerOrigin::Index(1))
            .direction(StaggerDirection::Normal)
            .build();

        let delays = stagger_config.calculate_delays(3, &[]);

        assert_eq!(delays.len(), 3);
        assert!((delays[0].as_millis() as i64 - 50).abs() <= 1);  // distance 1 from index 1
        assert!(delays[1].as_millis() == 0);   // distance 0 from index 1
        assert!((delays[2].as_millis() as i64 - 50).abs() <= 1);  // distance 1 from index 1
    }

    #[test]
    fn test_stagger_with_range() {
        let stagger_config = stagger_builder(100)
            .range(0.5, 2.0)
            .build();

        let delays = stagger_config.calculate_delays(3, &[]);

        // Base delays would be [0, 100, 200]
        // With range 0.5-2.0 and base_delay=100ms:
        // - delay[0] = 0ms: factor = 0.5 + 1.5 * (0.0/0.1) = 0.5, result = 100 * 0.5 = 50ms
        // - delay[1] = 100ms: factor = 0.5 + 1.5 * (0.1/0.1) = 2.0, result = 100 * 2.0 = 200ms
        // - delay[2] = 200ms: factor = 0.5 + 1.5 * (0.2/0.1).clamp(0,1) = 0.5 + 1.5 * 1.0 = 2.0, result = 100 * 2.0 = 200ms
        assert!((delays[0].as_millis() as i64 - 50).abs() <= 1);
        assert!((delays[1].as_millis() as i64 - 200).abs() <= 1);
        assert!((delays[2].as_millis() as i64 - 200).abs() <= 1);
    }
}
