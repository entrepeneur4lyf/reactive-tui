//! Main slider widget implementation

use crate::{
    components::Element, error::Result, layout::LayoutRect, reactive::Reactive, themes::ColorTheme,
};
use std::fmt;

use super::{SliderBuilder, SliderMode, SliderOrientation, SliderState, SliderStyle, SliderTicks};

/// Format a value using a format string
fn format_value(format_str: &str, value: f64) -> String {
    // Simple format string handling - supports basic decimal formatting
    if format_str.contains("{:.1}") {
        format!("{value:.1}")
    } else if format_str.contains("{:.2}") {
        format!("{value:.2}")
    } else if format_str.contains("{:.0}") {
        format!("{value:.0}")
    } else {
        format!("{value:.1}") // Default fallback
    }
}

/// Main slider widget implementation
#[derive(Clone)]
pub struct Slider {
    /// Unique identifier
    pub id: String,
    /// Slider operation mode
    pub mode: SliderMode,
    /// Visual orientation
    pub orientation: SliderOrientation,
    /// Current state
    pub state: Reactive<SliderState>,
    /// Visual styling
    pub style: SliderStyle,
    /// Tick mark configuration
    pub ticks: SliderTicks,
    /// CSS classes
    pub classes: Vec<String>,
    /// Custom attributes
    pub attributes: std::collections::HashMap<String, String>,
    /// Optional label
    pub label: Option<String>,
    /// Optional description/help text
    pub description: Option<String>,
}

impl Slider {
    /// Create a new slider with the given range and initial value
    pub fn new(id: impl Into<String>, min: f64, max: f64, value: f64) -> Result<Self> {
        let state = SliderState::new(min, max, value);
        state.validate()?;

        Ok(Self {
            id: id.into(),
            mode: SliderMode::Single,
            orientation: SliderOrientation::Horizontal,
            state: Reactive::new(state),
            style: SliderStyle::default(),
            ticks: SliderTicks::default(),
            classes: Vec::new(),
            attributes: std::collections::HashMap::new(),
            label: None,
            description: None,
        })
    }

    /// Create a new slider builder for fluent API
    pub fn builder(id: impl Into<String>) -> SliderBuilder {
        SliderBuilder::new(id)
    }

    /// Create a range slider with dual handles
    pub fn range(id: impl Into<String>, min: f64, max: f64, start: f64, end: f64) -> Result<Self> {
        let mut state = SliderState::new(min, max, start);
        state.range_end = end.clamp(min, max);
        state.validate()?;

        Ok(Self {
            id: id.into(),
            mode: SliderMode::Range,
            orientation: SliderOrientation::Horizontal,
            state: Reactive::new(state),
            style: SliderStyle::default(),
            ticks: SliderTicks::default(),
            classes: Vec::new(),
            attributes: std::collections::HashMap::new(),
            label: None,
            description: None,
        })
    }

    /// Set slider orientation
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self {
        self.orientation = orientation;
        self
    }

    /// Set visual styling
    pub fn style(mut self, style: SliderStyle) -> Self {
        self.style = style;
        self
    }

    /// Enable tick marks with configuration
    pub fn ticks(mut self, ticks: SliderTicks) -> Self {
        self.ticks = ticks;
        self
    }

    /// Add CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.classes.push(class.into());
        self
    }

    /// Add custom attribute
    pub fn attribute(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.attributes.insert(key.into(), value.into());
        self
    }

    /// Set label text
    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Set description/help text
    pub fn description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Get current value
    pub fn value(&self) -> f64 {
        self.state.get().value
    }

    /// Get range end value (for range sliders)
    pub fn range_end(&self) -> f64 {
        self.state.get().range_end
    }

    /// Set value programmatically
    pub fn set_value(&self, value: f64) -> Result<()> {
        let mut result = Ok(());
        self.state.update(|state| {
            if let Err(e) = state.set_value(value) {
                result = Err(e);
            }
        });
        result
    }

    /// Set range values (for range sliders)
    pub fn set_range(&self, start: f64, end: f64) -> Result<()> {
        let mut result = Ok(());
        self.state.update(|state| {
            if let Err(e) = state.set_value(start) {
                result = Err(e);
                return;
            }
            if let Err(e) = state.set_range_end(end) {
                result = Err(e);
            }
        });
        result
    }

    /// Enable/disable the slider
    pub fn set_disabled(&self, disabled: bool) {
        self.state.update(|state| {
            state.disabled = disabled;
        });
    }

    /// Set focus state
    pub fn set_focused(&self, focused: bool) {
        self.state.update(|state| {
            state.focused = focused;
        });
    }

    /// Handle keyboard input
    pub fn handle_key(&self, key: &str) -> Result<bool> {
        let mut handled = true;

        let mut result = Ok(());
        self.state.update(|state| {
            if state.disabled {
                return;
            }

            match key {
                "ArrowLeft" | "h" => {
                    if self.orientation == SliderOrientation::Horizontal {
                        if let Err(e) = state.decrement() {
                            result = Err(e);
                        }
                    }
                }
                "ArrowRight" | "l" => {
                    if self.orientation == SliderOrientation::Horizontal {
                        if let Err(e) = state.increment() {
                            result = Err(e);
                        }
                    }
                }
                "ArrowUp" | "k" => {
                    if self.orientation == SliderOrientation::Vertical {
                        if let Err(e) = state.increment() {
                            result = Err(e);
                        }
                    }
                }
                "ArrowDown" | "j" => {
                    if self.orientation == SliderOrientation::Vertical {
                        if let Err(e) = state.decrement() {
                            result = Err(e);
                        }
                    }
                }
                "Home" => {
                    if state.active_handle == 0 {
                        if let Err(e) = state.set_value(state.min) {
                            result = Err(e);
                        }
                    } else if let Err(e) = state.set_range_end(state.min) {
                        result = Err(e);
                    }
                }
                "End" => {
                    if state.active_handle == 0 {
                        if let Err(e) = state.set_value(state.max) {
                            result = Err(e);
                        }
                    } else if let Err(e) = state.set_range_end(state.max) {
                        result = Err(e);
                    }
                }
                "Tab" => {
                    if self.mode == SliderMode::Range {
                        state.active_handle = (state.active_handle + 1) % 2;
                    }
                }
                "PageUp" => {
                    let large_step = (state.max - state.min) / 10.0;
                    let new_value = if state.active_handle == 0 {
                        state.value + large_step
                    } else {
                        state.range_end + large_step
                    };

                    if state.active_handle == 0 {
                        if let Err(e) = state.set_value(new_value) {
                            result = Err(e);
                        }
                    } else if let Err(e) = state.set_range_end(new_value) {
                        result = Err(e);
                    }
                }
                "PageDown" => {
                    let large_step = (state.max - state.min) / 10.0;
                    let new_value = if state.active_handle == 0 {
                        state.value - large_step
                    } else {
                        state.range_end - large_step
                    };

                    if state.active_handle == 0 {
                        if let Err(e) = state.set_value(new_value) {
                            result = Err(e);
                        }
                    } else if let Err(e) = state.set_range_end(new_value) {
                        result = Err(e);
                    }
                }
                _ => {
                    handled = false;
                }
            }
        });
        result?;

        Ok(handled)
    }

    /// Handle mouse click at position (0.0 to 1.0 along track)
    pub fn handle_click(&self, position: f64) -> Result<()> {
        let mut result = Ok(());
        self.state.update(|state| {
            if state.disabled {
                return;
            }

            let target_value = state.min + (state.max - state.min) * position.clamp(0.0, 1.0);

            match self.mode {
                SliderMode::Single => {
                    if let Err(e) = state.set_value(target_value) {
                        result = Err(e);
                    }
                }
                SliderMode::Range => {
                    // Click on the handle closest to the click position
                    let dist_to_start = (position - state.value_percentage()).abs();
                    let dist_to_end = (position - state.range_end_percentage()).abs();

                    if dist_to_start < dist_to_end {
                        state.active_handle = 0;
                        if let Err(e) = state.set_value(target_value) {
                            result = Err(e);
                        }
                    } else {
                        state.active_handle = 1;
                        if let Err(e) = state.set_range_end(target_value) {
                            result = Err(e);
                        }
                    }
                }
            }
        });
        result
    }

    /// Render the slider as a string
    pub fn render_text(&self) -> String {
        let state = self.state.get();
        let mut output = String::new();

        // Add label if present
        if let Some(label) = &self.label {
            output.push_str(label);
            output.push(' ');
        }

        match self.orientation {
            SliderOrientation::Horizontal => {
                output.push_str(&self.render_horizontal(&state));
            }
            SliderOrientation::Vertical => {
                output.push_str(&self.render_vertical(&state));
            }
        }

        // Add value display
        if self.style.show_values {
            output.push(' ');
            match self.mode {
                SliderMode::Single => {
                    let formatted = format_value(&self.style.value_format, state.value);
                    output.push_str(&formatted);

                    if self.style.show_percentage {
                        let pct = state.value_percentage() * 100.0;
                        output.push_str(&format!(" ({pct:.0}%)"));
                    }
                }
                SliderMode::Range => {
                    let start_formatted = format_value(&self.style.value_format, state.value);
                    let end_formatted = format_value(&self.style.value_format, state.range_end);
                    output.push_str(&format!("[{start_formatted} - {end_formatted}]"));

                    if self.style.show_percentage {
                        let start_pct = state.value_percentage() * 100.0;
                        let end_pct = state.range_end_percentage() * 100.0;
                        output.push_str(&format!(" ({start_pct:.0}% - {end_pct:.0}%)"));
                    }
                }
            }
        }

        // Add description if present
        if let Some(description) = &self.description {
            output.push('\n');
            output.push_str(description);
        }

        output
    }

    /// Render the slider with layout and theme support
    pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
        self.render_text()
    }

    /// Render horizontal slider
    fn render_horizontal(&self, state: &SliderState) -> String {
        let mut track = vec![self.style.track_char; self.style.track_length];

        let start_pos =
            (state.value_percentage() * (self.style.track_length - 1) as f64).round() as usize;
        let end_pos = if self.mode == SliderMode::Range {
            (state.range_end_percentage() * (self.style.track_length - 1) as f64).round() as usize
        } else {
            start_pos
        };

        // Fill active track in range mode
        if self.mode == SliderMode::Range {
            let (left, right) = if start_pos <= end_pos {
                (start_pos, end_pos)
            } else {
                (end_pos, start_pos)
            };

            for i in left..=right {
                if i < track.len() {
                    track[i] = self.style.active_track_char;
                }
            }
        }

        // Place handles
        if start_pos < track.len() {
            track[start_pos] = self.style.handle_chars[0];
        }

        if self.mode == SliderMode::Range && end_pos < track.len() && end_pos != start_pos {
            track[end_pos] = self.style.handle_chars[1];
        }

        let mut result = track.into_iter().collect::<String>();

        // Add tick marks if enabled
        if self.ticks.enabled {
            result.push('\n');
            result.push_str(&self.render_horizontal_ticks(state));
        }

        result
    }

    /// Render vertical slider
    fn render_vertical(&self, state: &SliderState) -> String {
        let mut lines = Vec::new();
        let height = self.style.track_length;

        let start_pos = ((1.0 - state.value_percentage()) * (height - 1) as f64).round() as usize;
        let end_pos = if self.mode == SliderMode::Range {
            ((1.0 - state.range_end_percentage()) * (height - 1) as f64).round() as usize
        } else {
            start_pos
        };

        for i in 0..height {
            let mut line = String::new();

            // Add tick if enabled
            if self.ticks.enabled {
                if i % self.ticks.major_tick_interval == 0 {
                    line.push(self.ticks.major_tick_char);
                } else {
                    line.push(self.ticks.tick_char);
                }
                line.push(' ');
            }

            // Add track character
            if self.mode == SliderMode::Range {
                let (top, bottom) = if start_pos <= end_pos {
                    (start_pos, end_pos)
                } else {
                    (end_pos, start_pos)
                };

                if i >= top && i <= bottom {
                    line.push(self.style.active_track_char);
                } else {
                    line.push(self.style.track_char);
                }
            } else {
                line.push(self.style.track_char);
            }

            // Place handles
            if i == start_pos {
                line.pop(); // Remove track char
                line.push(self.style.handle_chars[0]);
            }

            if self.mode == SliderMode::Range && i == end_pos && end_pos != start_pos {
                line.pop(); // Remove track char
                line.push(self.style.handle_chars[1]);
            }

            lines.push(line);
        }

        lines.join("\n")
    }

    /// Render horizontal tick marks
    fn render_horizontal_ticks(&self, state: &SliderState) -> String {
        let mut ticks = vec![' '; self.style.track_length];

        let step = if self.ticks.step > 0.0 {
            self.ticks.step
        } else {
            (state.max - state.min) / (self.style.track_length - 1) as f64
        };

        let mut value = state.min;
        let mut tick_index = 0;

        while value <= state.max && tick_index < self.style.track_length {
            let pos = ((value - state.min) / (state.max - state.min)
                * (self.style.track_length - 1) as f64)
                .round() as usize;

            if pos < ticks.len() {
                if tick_index % self.ticks.major_tick_interval == 0 {
                    ticks[pos] = self.ticks.major_tick_char;
                } else {
                    ticks[pos] = self.ticks.tick_char;
                }
            }

            value += step;
            tick_index += 1;
        }

        let tick_line = ticks.into_iter().collect::<String>();

        if self.ticks.show_labels {
            let mut label_line = format!("{tick_line}\n");
            label_line.push_str(&format!("{:.1}", state.min));
            let padding = self.style.track_length.saturating_sub(10); // Rough estimate
            label_line.push_str(&" ".repeat(padding));
            label_line.push_str(&format!("{:.1}", state.max));
            label_line
        } else {
            tick_line
        }
    }

    /// Convert to Element for rendering in the UI framework
    pub fn to_element(&self) -> Result<Element> {
        let state = self.state.get();

        let mut element = Element::with_tag("slider")
            .id(&self.id)
            .attr("role", "slider")
            .attr("aria-valuemin", state.min.to_string())
            .attr("aria-valuemax", state.max.to_string())
            .attr("aria-valuenow", state.value.to_string())
            .attr("aria-disabled", state.disabled.to_string())
            .attr("tabindex", if state.disabled { "-1" } else { "0" });

        // Add mode-specific attributes
        match self.mode {
            SliderMode::Single => {
                element = element.attr("aria-valuenow", state.value.to_string());
            }
            SliderMode::Range => {
                element = element.attr("aria-valuenow", state.value.to_string()).attr(
                    "aria-valuetext",
                    format!("{}/{}", state.value, state.range_end),
                );
            }
        }

        // Add orientation
        element = element.attr(
            "aria-orientation",
            match self.orientation {
                SliderOrientation::Horizontal => "horizontal",
                SliderOrientation::Vertical => "vertical",
            },
        );

        // Add CSS classes
        for class in &self.classes {
            element = element.class(class);
        }

        // Add default classes
        element = element
            .class("slider")
            .class(match self.mode {
                SliderMode::Single => "slider-single",
                SliderMode::Range => "slider-range",
            })
            .class(match self.orientation {
                SliderOrientation::Horizontal => "slider-horizontal",
                SliderOrientation::Vertical => "slider-vertical",
            });

        if state.focused {
            element = element.class("slider-focused");
        }

        if state.disabled {
            element = element.class("slider-disabled");
        }

        // Add custom attributes
        for (key, value) in &self.attributes {
            element = element.attr(key, value);
        }

        // Set content to rendered text
        element = element.content(self.render_text());

        Ok(element.build())
    }
}

impl fmt::Display for Slider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.render_text())
    }
}
