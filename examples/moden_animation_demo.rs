//! Modern Animation Demo
//!
//! Demonstrates the modern animation features:
//! - Transform animations (translateX, translateY, scale, rotate)
//! - CSS property animations with units
//! - Numeric property animations
//! - Transform matrix animations
//!
//! This showcases the enhanced property system that enables modern animations.

use reactive_tui::prelude::*;
use reactive_tui::widgets::animation::{
    translate_x, translate_y, scale_animation, rotate_animation,
    css_property, numeric_property, matrix_animation,
    CssValue, TransformMatrix
};
use std::time::Duration;

fn main() -> Result<()> {
    println!("=== Modern Animation Demo ===");
    println!();
    println!("This demo showcases the new modern animation features:");
    println!("  🔄 Transform animations (translateX, translateY, scale, rotate)");
    println!("  📐 CSS property animations with units");
    println!("  🔢 Numeric property animations");
    println!("  ⚡ Transform matrix animations");
    println!();

    // Demo transform animations
    demo_transform_animations();

    // Demo CSS property animations
    demo_css_property_animations();

    // Demo numeric property animations
    demo_numeric_property_animations();

    // Demo transform matrix animations
    demo_matrix_animations();

    println!("✅ All modern animation features are working correctly!");

    Ok(())
}

fn demo_transform_animations() {
    println!("🔄 Transform Animations:");

    // translateX animation
    let translate_x_anim = translate_x("slide-right", 0.0, 100.0, Duration::from_millis(500));
    println!("  translateX: {} -> slide from 0 to 100px", translate_x_anim.id);

    // translateY animation
    let translate_y_anim = translate_y("slide-down", 0.0, 50.0, Duration::from_millis(300));
    println!("  translateY: {} -> slide from 0 to 50px", translate_y_anim.id);

    // Scale animation
    let scale_anim = scale_animation("grow", 1.0, 1.5, Duration::from_millis(400));
    println!("  scale: {} -> grow from 1.0 to 1.5x", scale_anim.id);

    // Rotate animation
    let rotate_anim = rotate_animation("spin", 0.0, 360.0, Duration::from_millis(1000));
    println!("  rotate: {} -> spin from 0 to 360 degrees", rotate_anim.id);

    println!();
}

fn demo_css_property_animations() {
    println!("📐 CSS Property Animations:");

    // Width animation with pixels
    let width_anim = css_property(
        "expand-width",
        "width",
        CssValue::pixels(100.0),
        CssValue::pixels(200.0),
        Duration::from_millis(500)
    );
    println!("  width: {} -> expand from 100px to 200px", width_anim.id);

    // Height animation with percentages
    let height_anim = css_property(
        "expand-height",
        "height",
        CssValue::percentage(50.0),
        CssValue::percentage(100.0),
        Duration::from_millis(600)
    );
    println!("  height: {} -> expand from 50% to 100%", height_anim.id);

    // Color animation
    let color_anim = css_property(
        "color-fade",
        "color",
        CssValue::color(255, 0, 0),  // Red
        CssValue::color(0, 0, 255),  // Blue
        Duration::from_millis(800)
    );
    println!("  color: {} -> fade from red to blue", color_anim.id);

    println!();
}

fn demo_numeric_property_animations() {
    println!("🔢 Numeric Property Animations:");

    // Opacity animation
    let opacity_anim = numeric_property("fade-in", "opacity", 0.0, 1.0, Duration::from_millis(400));
    println!("  opacity: {} -> fade from 0.0 to 1.0", opacity_anim.id);

    // Custom property animation
    let custom_anim = numeric_property("custom-prop", "blur", 0.0, 10.0, Duration::from_millis(500));
    println!("  blur: {} -> increase from 0.0 to 10.0", custom_anim.id);

    println!();
}

fn demo_matrix_animations() {
    println!("⚡ Transform Matrix Animations:");

    // Identity to scale matrix
    let from_matrix = TransformMatrix::default(); // Identity matrix
    let to_matrix = TransformMatrix {
        a: 2.0, b: 0.0, c: 0.0,  // Scale X by 2
        d: 2.0, e: 50.0, f: 30.0, // Scale Y by 2, translate (50, 30)
    };

    let matrix_anim = matrix_animation("complex-transform", from_matrix, to_matrix, Duration::from_millis(750));
    println!("  matrix: {} -> complex transform (scale + translate)", matrix_anim.id);

    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transform_animations() {
        let anim = translate_x("test", 0.0, 100.0, Duration::from_millis(100));

        // Test that it creates the correct property type
        match &anim.property {
            AnimatedProperty::Transform(TransformProperty::TranslateX(from, to)) => {
                assert_eq!(*from, 0.0);
                assert_eq!(*to, 100.0);
            }
            _ => panic!("Expected TranslateX property"),
        }
    }

    #[test]
    fn test_css_property_animations() {
        let anim = css_property(
            "test",
            "width",
            CssValue::pixels(100.0),
            CssValue::pixels(200.0),
            Duration::from_millis(100)
        );

        match &anim.property {
            AnimatedProperty::CssProperty(name, from, to) => {
                assert_eq!(name, "width");
                assert_eq!(*from, CssValue::pixels(100.0));
                assert_eq!(*to, CssValue::pixels(200.0));
            }
            _ => panic!("Expected CssProperty"),
        }
    }

    #[test]
    fn test_numeric_property_animations() {
        let anim = numeric_property("test", "opacity", 0.0, 1.0, Duration::from_millis(100));

        match &anim.property {
            AnimatedProperty::Property(name, from, to) => {
                assert_eq!(name, "opacity");
                assert_eq!(*from, 0.0);
                assert_eq!(*to, 1.0);
            }
            _ => panic!("Expected Property"),
        }
    }

    #[test]
    fn test_css_value_helpers() {
        assert_eq!(CssValue::pixels(100.0), CssValue::Pixels(100.0));
        assert_eq!(CssValue::percentage(50.0), CssValue::Percentage(50.0));
        assert_eq!(CssValue::em(2.0), CssValue::Em(2.0));
        assert_eq!(CssValue::color(255, 0, 0), CssValue::Color(reactive_tui::themes::ColorDefinition { r: 255, g: 0, b: 0 }));
    }

    #[test]
    fn test_animation_value_helpers() {
        assert_eq!(AnimationValue::pixels(100.0), AnimationValue::Unit(100.0, "px".to_string()));
        assert_eq!(AnimationValue::percentage(50.0), AnimationValue::Unit(50.0, "%".to_string()));
        assert_eq!(AnimationValue::number(42.0), AnimationValue::Number(42.0));
        assert_eq!(AnimationValue::array(vec![1.0, 2.0, 3.0]), AnimationValue::Array(vec![1.0, 2.0, 3.0]));
    }
}
