//! Comprehensive tests for the slider widget

#[cfg(test)]
mod slider_tests {
  use super::super::*;

  #[test]
  fn test_slider_creation() {
    let slider = Slider::new("test", 0.0, 100.0, 50.0).unwrap();
    assert_eq!(slider.value(), 50.0);
    assert_eq!(slider.mode, SliderMode::Single);
  }

  #[test]
  fn test_range_slider() {
    let slider = Slider::range("test", 0.0, 100.0, 25.0, 75.0).unwrap();
    assert_eq!(slider.value(), 25.0);
    assert_eq!(slider.range_end(), 75.0);
    assert_eq!(slider.mode, SliderMode::Range);
  }

  #[test]
  fn test_value_clamping() {
    let slider = Slider::new("test", 0.0, 100.0, 150.0).unwrap(); // Over max
    assert_eq!(slider.value(), 100.0);

    let slider = Slider::new("test", 10.0, 100.0, 5.0).unwrap(); // Under min
    assert_eq!(slider.value(), 10.0);
  }

  #[test]
  fn test_percentage_conversion() {
    let slider = Slider::new("test", 0.0, 100.0, 25.0).unwrap();
    let state = slider.state.get();
    assert_eq!(state.value_percentage(), 0.25);

    let slider = Slider::new("test", 50.0, 150.0, 100.0).unwrap();
    let state = slider.state.get();
    assert_eq!(state.value_percentage(), 0.5);
  }

  #[test]
  fn test_step_snapping() {
    let slider = Slider::new("test", 0.0, 100.0, 0.0).unwrap();
    slider.state.update(|state| {
      state.step = 10.0;
      state.set_value(23.7).unwrap(); // Should snap to 20.0
    });
    assert_eq!(slider.value(), 20.0);
  }

  #[test]
  fn test_keyboard_handling() {
    let slider = Slider::new("test", 0.0, 100.0, 50.0).unwrap();
    slider.state.update(|state| state.step = 5.0);

    // Test increment
    slider.handle_key("ArrowRight").unwrap();
    assert_eq!(slider.value(), 55.0);

    // Test decrement
    slider.handle_key("ArrowLeft").unwrap();
    assert_eq!(slider.value(), 50.0);

    // Test home/end
    slider.handle_key("Home").unwrap();
    assert_eq!(slider.value(), 0.0);

    slider.handle_key("End").unwrap();
    assert_eq!(slider.value(), 100.0);
  }

  #[test]
  fn test_mouse_click_handling() {
    let slider = Slider::new("test", 0.0, 100.0, 0.0).unwrap();

    // Click at 50% position
    slider.handle_click(0.5).unwrap();
    assert_eq!(slider.value(), 50.0);

    // Click at 75% position
    slider.handle_click(0.75).unwrap();
    assert_eq!(slider.value(), 75.0);
  }

  #[test]
  fn test_range_slider_consistency() {
    let slider = Slider::range("test", 0.0, 100.0, 30.0, 70.0).unwrap();

    // Move start value beyond end - should adjust end
    slider.set_value(80.0).unwrap();
    assert_eq!(slider.value(), 80.0);
    assert!(slider.range_end() >= 80.0);
  }

  #[test]
  fn test_disabled_state() {
    let slider = Slider::new("test", 0.0, 100.0, 50.0).unwrap();
    slider.set_disabled(true);

    let initial_value = slider.value();
    slider.handle_key("ArrowRight").unwrap();
    assert_eq!(slider.value(), initial_value); // Should not change
  }

  #[test]
  fn test_validation() {
    // Create an invalid state manually and test validation
    let mut state = SliderState::new(0.0, 100.0, 50.0);
    state.min = 100.0; // Manually create invalid state
    state.max = 50.0;
    assert!(state.validate().is_err());

    // Valid slider should validate
    let slider = Slider::new("test", 0.0, 100.0, 50.0).unwrap();
    assert!(slider.state.get().validate().is_ok());

    // Test that SliderState::new handles invalid ranges correctly
    let fixed_state = SliderState::new(100.0, 50.0, 75.0);
    assert_eq!(fixed_state.min, 50.0); // Should be swapped
    assert_eq!(fixed_state.max, 100.0);
    assert!(fixed_state.validate().is_ok());
  }

  #[test]
  fn test_builder_pattern() {
    let slider = SliderBuilder::new("test")
      .range(0.0, 200.0)
      .value(100.0)
      .dual_range(50.0, 150.0)
      .orientation(SliderOrientation::Vertical)
      .class("custom-slider")
      .label("Volume")
      .build()
      .unwrap();

    assert_eq!(slider.mode, SliderMode::Range);
    assert_eq!(slider.orientation, SliderOrientation::Vertical);
    assert_eq!(slider.value(), 50.0);
    assert_eq!(slider.range_end(), 150.0);
    assert!(slider.classes.contains(&"custom-slider".to_string()));
    assert_eq!(slider.label, Some("Volume".to_string()));
  }

  #[test]
  fn test_render_output() {
    let slider = SliderBuilder::new("test")
      .range(0.0, 100.0)
      .value(50.0)
      .label("Volume")
      .build()
      .unwrap();

    let output = slider.render_text();
    assert!(output.contains("Volume"));
    assert!(output.contains("50.0"));
    assert!(output.contains("●")); // Handle character
  }

  #[test]
  fn test_state_methods() {
    let mut state = SliderState::new(0.0, 100.0, 50.0);

    // Test range span
    state.range_end = 75.0;
    assert_eq!(state.range_span(), 25.0);

    // Test min/max checks
    state.value = 0.0;
    assert!(state.is_at_min());
    assert!(!state.is_at_max());

    state.value = 100.0;
    assert!(!state.is_at_min());
    assert!(state.is_at_max());

    // Test percentage from value
    state.set_value_from_percentage(0.75).unwrap();
    assert_eq!(state.value, 75.0);
  }

  #[test]
  fn test_tick_configuration() {
    let ticks = SliderTicks {
      enabled: true,
      step: 20.0,
      show_labels: true,
      major_tick_interval: 2,
      ..Default::default()
    };

    let slider = SliderBuilder::new("test")
      .range(0.0, 100.0)
      .value(60.0)
      .ticks(ticks)
      .build()
      .unwrap();

    assert!(slider.ticks.enabled);
    assert_eq!(slider.ticks.step, 20.0);
    assert!(slider.ticks.show_labels);
  }

  #[test]
  fn test_style_configuration() {
    let style = SliderStyle {
      track_char: '═',
      active_track_char: '█',
      handle_chars: ['▲', '▼'],
      track_length: 30,
      show_values: true,
      value_format: "{:.2}".to_string(),
      show_percentage: true,
    };

    let slider = SliderBuilder::new("test")
      .range(0.0, 100.0)
      .value(33.33)
      .style(style)
      .build()
      .unwrap();

    assert_eq!(slider.style.track_char, '═');
    assert_eq!(slider.style.handle_chars[0], '▲');
    assert_eq!(slider.style.track_length, 30);
    assert!(slider.style.show_percentage);

    let output = slider.render_text();
    assert!(output.contains("33.33")); // Formatted value
    assert!(output.contains("33%")); // Percentage
  }

  #[test]
  fn test_element_conversion() {
    let slider = SliderBuilder::new("test-slider")
      .range(0.0, 100.0)
      .value(75.0)
      .class("custom-class")
      .attr("data-test", "value")
      .build()
      .unwrap();

    let element = slider.to_element().unwrap();

    assert_eq!(element.tag, "slider");
    assert_eq!(element.id, Some("test-slider".to_string()));
    assert_eq!(
      element.get_attribute("role").map(|s| s.as_str()),
      Some("slider")
    );
    assert_eq!(
      element.get_attribute("aria-valuemin").map(|s| s.as_str()),
      Some("0")
    );
    assert_eq!(
      element.get_attribute("aria-valuemax").map(|s| s.as_str()),
      Some("100")
    );
    assert_eq!(
      element.get_attribute("aria-valuenow").map(|s| s.as_str()),
      Some("75")
    );
    assert_eq!(
      element.get_attribute("data-test").map(|s| s.as_str()),
      Some("value")
    );
    assert!(element.classes.contains(&"custom-class".to_string()));
    assert!(element.classes.contains(&"slider".to_string()));
    assert!(element.classes.contains(&"slider-single".to_string()));
  }
}
