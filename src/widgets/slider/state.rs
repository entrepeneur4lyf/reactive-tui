//! Slider state management and data structures

use crate::error::{Result, TuiError};
use serde::{Deserialize, Serialize};

/// Orientation of the slider
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SliderOrientation {
  Horizontal,
  Vertical,
}

impl Default for SliderOrientation {
  fn default() -> Self {
    Self::Horizontal
  }
}

/// Slider interaction mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum SliderMode {
  /// Single handle for selecting one value
  Single,
  /// Dual handles for selecting a range
  Range,
}

impl Default for SliderMode {
  fn default() -> Self {
    Self::Single
  }
}

/// Current state of the slider
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SliderState {
  /// Primary value (or single value in single mode)
  pub value: f64,
  /// Secondary value (only used in range mode)
  pub range_end: f64,
  /// Minimum value
  pub min: f64,
  /// Maximum value
  pub max: f64,
  /// Step increment for value changes
  pub step: f64,
  /// Currently active handle (0 = primary, 1 = secondary)
  pub active_handle: usize,
  /// Whether the slider is focused
  pub focused: bool,
  /// Whether the slider is disabled
  pub disabled: bool,
}

impl SliderState {
  /// Create new slider state
  pub fn new(min: f64, max: f64, value: f64) -> Self {
    let (actual_min, actual_max) = if min <= max { (min, max) } else { (max, min) };
    Self {
      value: value.clamp(actual_min, actual_max),
      range_end: actual_max,
      min: actual_min,
      max: actual_max,
      step: (actual_max - actual_min) / 100.0, // Default to 1% steps
      active_handle: 0,
      focused: false,
      disabled: false,
    }
  }

  /// Get the current value as a percentage (0.0 to 1.0)
  pub fn value_percentage(&self) -> f64 {
    if self.max == self.min {
      0.0
    } else {
      (self.value - self.min) / (self.max - self.min)
    }
  }

  /// Get the range end value as a percentage (0.0 to 1.0)
  pub fn range_end_percentage(&self) -> f64 {
    if self.max == self.min {
      1.0
    } else {
      (self.range_end - self.min) / (self.max - self.min)
    }
  }

  /// Set value from percentage (0.0 to 1.0)
  pub fn set_value_from_percentage(&mut self, percentage: f64) -> Result<()> {
    let percentage = percentage.clamp(0.0, 1.0);
    let new_value = self.min + (self.max - self.min) * percentage;
    self.set_value(new_value)
  }

  /// Set value with step and range validation
  pub fn set_value(&mut self, value: f64) -> Result<()> {
    if self.disabled {
      return Ok(());
    }

    let clamped = value.clamp(self.min, self.max);

    // Snap to step if configured
    let snapped = if self.step > 0.0 {
      let steps = ((clamped - self.min) / self.step).round();
      self.min + steps * self.step
    } else {
      clamped
    };

    self.value = snapped.clamp(self.min, self.max);

    // Ensure range consistency in range mode
    if self.range_end < self.value {
      self.range_end = self.value;
    }

    Ok(())
  }

  /// Set range end value
  pub fn set_range_end(&mut self, value: f64) -> Result<()> {
    if self.disabled {
      return Ok(());
    }

    let clamped = value.clamp(self.min, self.max);

    // Snap to step if configured
    let snapped = if self.step > 0.0 {
      let steps = ((clamped - self.min) / self.step).round();
      self.min + steps * self.step
    } else {
      clamped
    };

    self.range_end = snapped.clamp(self.min, self.max);

    // Ensure range consistency
    if self.range_end < self.value {
      self.value = self.range_end;
    }

    Ok(())
  }

  /// Increment value by step
  pub fn increment(&mut self) -> Result<()> {
    let new_value = if self.active_handle == 0 {
      self.value + self.step
    } else {
      self.range_end + self.step
    };

    if self.active_handle == 0 {
      self.set_value(new_value)
    } else {
      self.set_range_end(new_value)
    }
  }

  /// Decrement value by step
  pub fn decrement(&mut self) -> Result<()> {
    let new_value = if self.active_handle == 0 {
      self.value - self.step
    } else {
      self.range_end - self.step
    };

    if self.active_handle == 0 {
      self.set_value(new_value)
    } else {
      self.set_range_end(new_value)
    }
  }

  /// Get the range span (difference between range_end and value)
  pub fn range_span(&self) -> f64 {
    (self.range_end - self.value).abs()
  }

  /// Check if value is at minimum
  pub fn is_at_min(&self) -> bool {
    (self.value - self.min).abs() < f64::EPSILON
  }

  /// Check if value is at maximum
  pub fn is_at_max(&self) -> bool {
    (self.value - self.max).abs() < f64::EPSILON
  }

  /// Validate slider state consistency
  pub fn validate(&self) -> Result<()> {
    if self.min >= self.max {
      return Err(TuiError::component(
        "Slider minimum must be less than maximum",
      ));
    }

    if self.value < self.min || self.value > self.max {
      return Err(TuiError::component("Slider value is outside valid range"));
    }

    if self.range_end < self.min || self.range_end > self.max {
      return Err(TuiError::component(
        "Slider range end is outside valid range",
      ));
    }

    if self.step < 0.0 {
      return Err(TuiError::component("Slider step must be non-negative"));
    }

    Ok(())
  }
}
