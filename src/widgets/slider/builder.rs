//! Builder pattern for creating sliders with fluent API

use super::{Slider, SliderMode, SliderOrientation, SliderStyle, SliderTicks};
use crate::error::Result;

/// Builder for creating sliders with fluent API
#[derive(Debug)]
pub struct SliderBuilder {
    id: String,
    mode: SliderMode,
    orientation: SliderOrientation,
    min: f64,
    max: f64,
    value: f64,
    range_end: Option<f64>,
    style: SliderStyle,
    ticks: SliderTicks,
    classes: Vec<String>,
    attributes: std::collections::HashMap<String, String>,
    label: Option<String>,
    description: Option<String>,
}

impl SliderBuilder {
    /// Create a new slider builder
    pub fn new(id: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            mode: SliderMode::Single,
            orientation: SliderOrientation::Horizontal,
            min: 0.0,
            max: 100.0,
            value: 0.0,
            range_end: None,
            style: SliderStyle::default(),
            ticks: SliderTicks::default(),
            classes: Vec::new(),
            attributes: std::collections::HashMap::new(),
            label: None,
            description: None,
        }
    }

    /// Set the value range
    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    /// Set initial value
    pub fn value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// Set slider width (track length)
    pub fn width(mut self, width: usize) -> Self {
        self.style.track_length = width;
        self
    }

    /// Enable range mode with dual handles
    pub fn dual_range(mut self, start: f64, end: f64) -> Self {
        self.mode = SliderMode::Range;
        self.value = start;
        self.range_end = Some(end);
        self
    }

    /// Set orientation
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set visual style
    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable tick marks
    pub fn ticks(mut self, ticks: SliderTicks) -> Self {
        self.ticks = ticks;
        self
    }

    /// Add CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add attribute
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Set label
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Build the slider
    pub fn build(self) -> Result<Slider> {
        let mut slider = match self.mode {
            SliderMode::Single => Slider::new(self.id, self.min, self.max, self.value)?,
            SliderMode::Range => {
                let end = self.range_end.unwrap_or(self.max);
                Slider::range(self.id, self.min, self.max, self.value, end)?
            }
        };

        slider.orientation = self.orientation;
        slider.style = self.style;
        slider.ticks = self.ticks;
        slider.classes = self.classes;
        slider.attributes = self.attributes;
        slider.label = self.label;
        slider.description = self.description;

        Ok(slider)
    }
}
