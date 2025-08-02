//! Slider styling and visual configuration

use serde::{Deserialize, Serialize};

/// Configuration for slider tick marks
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderTicks {
    /// Show tick marks
    pub enabled: bool,
    /// Step between tick marks (0 = auto-calculate)
    pub step: f64,
    /// Show labels on ticks
    pub show_labels: bool,
    /// Custom tick labels (overrides auto-generated labels)
    pub custom_labels: Vec<String>,
    /// Tick mark character
    pub tick_char: char,
    /// Major tick character (every nth tick)
    pub major_tick_char: char,
    /// Major tick interval (every nth tick is major)
    pub major_tick_interval: usize,
}

impl Default for SliderTicks {
    fn default() -> Self {
        Self {
            enabled: false,
            step: 0.0,
            show_labels: false,
            custom_labels: Vec::new(),
            tick_char: '|',
            major_tick_char: '┼',
            major_tick_interval: 5,
        }
    }
}

/// Visual styling configuration for the slider
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SliderStyle {
    /// Track character (inactive portion)
    pub track_char: char,
    /// Active track character (between handles in range mode)
    pub active_track_char: char,
    /// Handle character(s)
    pub handle_chars: [char; 2], // [primary, secondary for range mode]
    /// Track length in characters
    pub track_length: usize,
    /// Show current value(s) as labels
    pub show_values: bool,
    /// Value label format string
    pub value_format: String,
    /// Show percentage alongside values
    pub show_percentage: bool,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            track_char: '─',
            active_track_char: '━',
            handle_chars: ['●', '○'],
            track_length: 20,
            show_values: true,
            value_format: "{:.1}".to_string(),
            show_percentage: false,
        }
    }
}
