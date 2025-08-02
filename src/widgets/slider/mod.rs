//! Slider Control Widget - Range slider with customizable appearance
//!
//! Provides interactive slider controls for selecting numeric values within a range.
//! Supports both single-value sliders and range sliders with dual handles.

mod builder;
mod state;
mod style;
mod tests;
mod widget;

pub use builder::SliderBuilder;
pub use state::{SliderMode, SliderOrientation, SliderState};
pub use style::{SliderStyle, SliderTicks};
pub use widget::Slider;
