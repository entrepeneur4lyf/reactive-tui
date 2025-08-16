//! Progress indicators for showing task completion and loading states
//!
//! Provides linear and circular progress bars with customizable appearance,
//! animations, and real-time value updates.

use crate::{
  components::Element,
  error::{Result, TuiError},
  events::Message,
  layout::LayoutRect,
  reactive::ReactiveState,
  themes::ColorTheme,
};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

/// Progress bar variants and styles
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressStyle {
  /// Linear horizontal progress bar
  Linear {
    /// Fill character (e.g., '█', '=', '#')
    fill_char: char,
    /// Empty character (e.g., '░', '-', '.')
    empty_char: char,
    /// Show percentage text overlay
    show_percentage: bool,
    /// Show current/total values
    show_values: bool,
    /// Custom width (None = auto-fit container)
    width: Option<u16>,
  },
  /// Circular progress indicator
  Circular {
    /// Radius of the circle
    radius: u8,
    /// Use Unicode box drawing characters
    use_unicode: bool,
    /// Show percentage in center
    show_percentage: bool,
  },
  /// Arc/Gauge style progress
  Arc {
    /// Arc radius
    radius: u8,
    /// Start angle in degrees (0 = top)
    start_angle: f32,
    /// Sweep angle in degrees (360 = full circle)
    sweep_angle: f32,
    /// Show percentage
    show_percentage: bool,
  },
  /// Spinner for indeterminate progress
  Spinner {
    /// Spinner frames
    frames: Vec<String>,
    /// Animation speed (milliseconds per frame)
    speed: u64,
  },
}

impl Default for ProgressStyle {
  fn default() -> Self {
    Self::Linear {
      fill_char: '█',
      empty_char: '░',
      show_percentage: true,
      show_values: false,
      width: None,
    }
  }
}

/// Progress bar state and animation
#[derive(Debug, Clone, PartialEq)]
pub enum ProgressState {
  /// Determinate progress with known value
  Determinate {
    /// Current progress value
    value: f64,
    /// Maximum value (progress = value/max)
    max: f64,
    /// Minimum value
    min: f64,
  },
  /// Indeterminate progress (spinning/pulsing)
  Indeterminate {
    /// Current animation frame
    frame: usize,
    /// Last frame update time (using system time for serialization compatibility)
    last_update_millis: u64,
  },
}

impl Default for ProgressState {
  fn default() -> Self {
    Self::Determinate {
      value: 0.0,
      max: 100.0,
      min: 0.0,
    }
  }
}

/// Animation configuration for progress bars
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressAnimation {
  /// Enable smooth value transitions
  pub smooth_transitions: bool,
  /// Transition duration in milliseconds
  pub transition_duration: u64,
  /// Easing function for transitions
  pub easing: EasingFunction,
  /// Enable pulsing effect for completed progress
  pub pulse_on_complete: bool,
}

impl Default for ProgressAnimation {
  fn default() -> Self {
    Self {
      smooth_transitions: true,
      transition_duration: 300,
      easing: EasingFunction::EaseOut,
      pulse_on_complete: false,
    }
  }
}

/// Easing functions for smooth animations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum EasingFunction {
  Linear,
  EaseIn,
  EaseOut,
  EaseInOut,
  Bounce,
  Elastic,
}

impl EasingFunction {
  /// Apply easing function to progress value (0.0 to 1.0)
  pub fn apply(&self, t: f64) -> f64 {
    match self {
      Self::Linear => t,
      Self::EaseIn => t * t * t,
      Self::EaseOut => 1.0 - (1.0 - t).powi(3),
      Self::EaseInOut => {
        if t < 0.5 {
          4.0 * t * t * t
        } else {
          1.0 - (-2.0 * t + 2.0).powi(3) / 2.0
        }
      }
      Self::Bounce => {
        let n1 = 7.5625;
        let d1 = 2.75;

        if t < 1.0 / d1 {
          n1 * t * t
        } else if t < 2.0 / d1 {
          n1 * (t - 1.5 / d1) * (t - 1.5 / d1) + 0.75
        } else if t < 2.5 / d1 {
          n1 * (t - 2.25 / d1) * (t - 2.25 / d1) + 0.9375
        } else {
          n1 * (t - 2.625 / d1) * (t - 2.625 / d1) + 0.984375
        }
      }
      Self::Elastic => {
        if t == 0.0 {
          0.0
        } else if t == 1.0 {
          1.0
        } else {
          let c4 = (2.0 * std::f64::consts::PI) / 3.0;
          -(2.0_f64.powf(10.0 * t - 10.0)) * ((t * 10.0 - 10.75) * c4).sin()
        }
      }
    }
  }
}

/// Color scheme for progress bars
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProgressColors {
  /// Fill/progress color
  pub fill: String,
  /// Background/empty color
  pub background: String,
  /// Text color
  pub text: String,
  /// Border color (for circular/arc styles)
  pub border: Option<String>,
}

impl Default for ProgressColors {
  fn default() -> Self {
    Self {
      fill: "blue".to_string(),
      background: "gray".to_string(),
      text: "white".to_string(),
      border: None,
    }
  }
}

/// Complete progress bar configuration
#[derive(Clone)]
pub struct ProgressBar {
  /// Unique identifier for the progress bar
  pub id: String,
  /// Progress style and appearance
  pub style: ProgressStyle,
  /// Current state and value
  pub state: ProgressState,
  /// Animation configuration
  pub animation: ProgressAnimation,
  /// Color scheme
  pub colors: ProgressColors,
  /// Custom label text
  pub label: Option<String>,
  /// CSS classes for styling
  pub classes: Vec<String>,
  /// Custom attributes
  pub attributes: HashMap<String, String>,
  /// Reactive state for live updates
  pub reactive_state: Option<std::sync::Arc<ReactiveState>>,
}

impl ProgressBar {
  /// Create a new progress bar with default settings
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      id: id.into(),
      style: ProgressStyle::default(),
      state: ProgressState::default(),
      animation: ProgressAnimation::default(),
      colors: ProgressColors::default(),
      label: None,
      classes: Vec::new(),
      attributes: HashMap::new(),
      reactive_state: None,
    }
  }

  /// Get current progress percentage (0.0 to 1.0)
  pub fn progress(&self) -> f64 {
    match &self.state {
      ProgressState::Determinate { value, max, min } => {
        if max == min {
          0.0
        } else {
          ((value - min) / (max - min)).clamp(0.0, 1.0)
        }
      }
      ProgressState::Indeterminate { .. } => 0.0, // Indeterminate has no fixed progress
    }
  }

  /// Set progress value (will clamp to min/max range)
  pub fn set_value(&mut self, value: f64) -> Result<()> {
    match &mut self.state {
      ProgressState::Determinate {
        value: current,
        max,
        min,
      } => {
        *current = value.clamp(*min, *max);

        // Update reactive state if connected
        if let Some(reactive) = &self.reactive_state {
          reactive.set_field(&format!("{}.value", self.id), *current);
          reactive.set_field(&format!("{}.progress", self.id), self.progress());
        }

        Ok(())
      }
      ProgressState::Indeterminate { .. } => Err(TuiError::component(
        "Cannot set value on indeterminate progress bar".to_string(),
      )),
    }
  }

  /// Set progress percentage (0.0 to 1.0)
  pub fn set_progress(&mut self, progress: f64) -> Result<()> {
    match &self.state {
      ProgressState::Determinate { max, min, .. } => {
        let value = min + (progress.clamp(0.0, 1.0) * (max - min));
        self.set_value(value)
      }
      ProgressState::Indeterminate { .. } => Err(TuiError::component(
        "Cannot set progress on indeterminate progress bar".to_string(),
      )),
    }
  }

  /// Set min/max range for progress values
  pub fn set_range(&mut self, min: f64, max: f64) -> Result<()> {
    if min > max {
      return Err(TuiError::component(
        "Minimum value cannot be greater than maximum".to_string(),
      ));
    }

    match &mut self.state {
      ProgressState::Determinate {
        value,
        max: current_max,
        min: current_min,
      } => {
        *current_min = min;
        *current_max = max;
        *value = value.clamp(min, max);

        // Update reactive state
        if let Some(reactive) = &self.reactive_state {
          reactive.set_field(&format!("{}.min", self.id), min);
          reactive.set_field(&format!("{}.max", self.id), max);
          reactive.set_field(&format!("{}.value", self.id), *value);
          reactive.set_field(&format!("{}.progress", self.id), self.progress());
        }

        Ok(())
      }
      ProgressState::Indeterminate { .. } => Err(TuiError::component(
        "Cannot set range on indeterminate progress bar".to_string(),
      )),
    }
  }

  /// Switch to indeterminate mode (spinner/pulse)
  pub fn set_indeterminate(&mut self) {
    self.state = ProgressState::Indeterminate {
      frame: 0,
      last_update_millis: std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64,
    };
  }

  /// Switch to determinate mode with value
  pub fn set_determinate(&mut self, value: f64, min: f64, max: f64) -> Result<()> {
    if min > max {
      return Err(TuiError::component(
        "Minimum value cannot be greater than maximum".to_string(),
      ));
    }

    self.state = ProgressState::Determinate {
      value: value.clamp(min, max),
      min,
      max,
    };
    Ok(())
  }

  /// Update animation frame for indeterminate progress
  pub fn update_animation(&mut self) -> bool {
    match &mut self.state {
      ProgressState::Indeterminate {
        frame,
        last_update_millis,
      } => {
        let now_millis = std::time::SystemTime::now()
          .duration_since(std::time::UNIX_EPOCH)
          .unwrap_or_default()
          .as_millis() as u64;

        let frame_duration = match &self.style {
          ProgressStyle::Spinner { speed, .. } => *speed,
          _ => 100, // Default animation speed in ms
        };

        if now_millis.saturating_sub(*last_update_millis) >= frame_duration {
          let max_frames = match &self.style {
            ProgressStyle::Spinner { frames, .. } => frames.len(),
            _ => 8, // Default for other indeterminate styles
          };

          *frame = (*frame + 1) % max_frames;
          *last_update_millis = now_millis;
          true // Animation updated
        } else {
          false // No update needed
        }
      }
      ProgressState::Determinate { .. } => false, // No animation for determinate
    }
  }

  /// Render the progress bar as an Element
  pub fn render(&self) -> Element {
    let mut element = Element::with_tag("progress")
      .id(&self.id)
      .classes(self.classes.clone());

    // Add custom attributes
    for (key, value) in &self.attributes {
      element = element.attr(key, value);
    }

    // Add progress-specific attributes
    element = element
      .attr("progress", self.progress().to_string())
      .attr("style-type", self.style_type_name());

    // Set content based on style
    let content = match &self.style {
      ProgressStyle::Linear { .. } => self.render_linear(),
      ProgressStyle::Circular { .. } => self.render_circular(),
      ProgressStyle::Arc { .. } => self.render_arc(),
      ProgressStyle::Spinner { .. } => self.render_spinner(),
    };

    element.content(content).build()
  }

  /// Render the progress bar with a specific value (for demos and testing)
  pub fn render_with_value(
    &self,
    value: f64,
    _layout: &LayoutRect,
    _theme: Option<&ColorTheme>,
  ) -> String {
    // Create a temporary copy with the specified value
    let mut temp_bar = self.clone();
    let _ = temp_bar.set_value(value);

    // Render the content directly as a string
    match &temp_bar.style {
      ProgressStyle::Linear { .. } => temp_bar.render_linear(),
      ProgressStyle::Circular { .. } => temp_bar.render_circular(),
      ProgressStyle::Arc { .. } => temp_bar.render_arc(),
      ProgressStyle::Spinner { .. } => temp_bar.render_spinner(),
    }
  }

  /// Get style type name for CSS styling
  fn style_type_name(&self) -> String {
    match &self.style {
      ProgressStyle::Linear { .. } => "linear".to_string(),
      ProgressStyle::Circular { .. } => "circular".to_string(),
      ProgressStyle::Arc { .. } => "arc".to_string(),
      ProgressStyle::Spinner { .. } => "spinner".to_string(),
    }
  }

  /// Render linear progress bar
  fn render_linear(&self) -> String {
    let ProgressStyle::Linear {
      fill_char,
      empty_char,
      show_percentage,
      show_values,
      width,
    } = &self.style
    else {
      return String::new();
    };

    let progress = self.progress();
    let bar_width = width.unwrap_or(40) as usize;
    let filled_width = (progress * bar_width as f64) as usize;
    let empty_width = bar_width.saturating_sub(filled_width);

    let mut result = String::new();

    // Progress bar
    result.push_str(&fill_char.to_string().repeat(filled_width));
    result.push_str(&empty_char.to_string().repeat(empty_width));

    // Add percentage if requested
    if *show_percentage {
      result.push_str(&format!(" {:.1}%", progress * 100.0));
    }

    // Add values if requested
    if *show_values {
      if let ProgressState::Determinate { value, max, min: _ } = &self.state {
        result.push_str(&format!(" ({value:.1}/{max:.1})"));
      }
    }

    // Add label if present
    if let Some(label) = &self.label {
      result = format!("{label}: {result}");
    }

    result
  }

  /// Render circular progress indicator
  fn render_circular(&self) -> String {
    let ProgressStyle::Circular {
      radius,
      use_unicode,
      show_percentage,
    } = &self.style
    else {
      return String::new();
    };

    let progress = self.progress();
    let diameter = (*radius as usize) * 2 + 1;
    let center = *radius as f32;

    let mut lines = Vec::with_capacity(diameter);

    for y in 0..diameter {
      let mut line = String::new();
      for x in 0..diameter {
        let dx = x as f32 - center;
        let dy = y as f32 - center;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= center && distance >= center - 1.0 {
          // On the circle perimeter
          let angle = dy.atan2(dx) + std::f32::consts::PI;
          let normalized_angle = angle / (2.0 * std::f32::consts::PI);

          if normalized_angle <= progress as f32 {
            line.push(if *use_unicode { '●' } else { 'O' });
          } else {
            line.push(if *use_unicode { '○' } else { 'o' });
          }
        } else if distance < center - 1.0
          && *show_percentage
          && x == center as usize
          && y == center as usize
        {
          // Center - show percentage
          let pct_str = format!("{:.0}%", progress * 100.0);
          line.push_str(&pct_str);
          // Skip remaining characters for this line
          break;
        } else {
          line.push(' ');
        }
      }
      lines.push(line);
    }

    lines.join("\n")
  }

  /// Render arc/gauge progress indicator
  fn render_arc(&self) -> String {
    let ProgressStyle::Arc {
      radius,
      start_angle,
      sweep_angle,
      show_percentage,
    } = &self.style
    else {
      return String::new();
    };

    let progress = self.progress();
    let diameter = (*radius as usize) * 2 + 1;
    let center = *radius as f32;

    let start_rad = start_angle.to_radians();
    let sweep_rad = sweep_angle.to_radians();
    let progress_angle = start_rad + (progress as f32 * sweep_rad);

    let mut lines = Vec::with_capacity(diameter);

    for y in 0..diameter {
      let mut line = String::new();
      for x in 0..diameter {
        let dx = x as f32 - center;
        let dy = y as f32 - center;
        let distance = (dx * dx + dy * dy).sqrt();

        if distance <= center && distance >= center - 1.0 {
          let angle = dy.atan2(dx) + std::f32::consts::PI;
          let normalized_angle =
            (angle - start_rad + 2.0 * std::f32::consts::PI) % (2.0 * std::f32::consts::PI);

          if normalized_angle <= sweep_rad && angle <= progress_angle {
            line.push('█');
          } else if normalized_angle <= sweep_rad {
            line.push('░');
          } else {
            line.push(' ');
          }
        } else {
          line.push(' ');
        }
      }
      lines.push(line);
    }

    if *show_percentage {
      lines.push(format!("{:.1}%", progress * 100.0));
    }

    lines.join("\n")
  }

  /// Render spinner for indeterminate progress
  fn render_spinner(&self) -> String {
    let ProgressStyle::Spinner { frames, .. } = &self.style else {
      return String::new();
    };

    let frame_index = match &self.state {
      ProgressState::Indeterminate { frame, .. } => *frame % frames.len(),
      _ => 0,
    };

    let mut result = frames.get(frame_index).cloned().unwrap_or_default();

    if let Some(label) = &self.label {
      result = format!("{result} {label}");
    }

    result
  }

  /// Connect to reactive state for live updates
  pub fn connect_reactive(&mut self, state: std::sync::Arc<ReactiveState>) -> Result<()> {
    // Initialize reactive values
    state.set_field(&format!("{}.progress", self.id), self.progress());

    if let ProgressState::Determinate { value, min, max } = &self.state {
      state.set_field(&format!("{}.value", self.id), *value);
      state.set_field(&format!("{}.min", self.id), *min);
      state.set_field(&format!("{}.max", self.id), *max);
    }

    self.reactive_state = Some(state);
    Ok(())
  }
}

impl fmt::Debug for ProgressBar {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_struct("ProgressBar")
      .field("id", &self.id)
      .field("style", &self.style)
      .field("state", &self.state)
      .field("animation", &self.animation)
      .field("colors", &self.colors)
      .field("label", &self.label)
      .field("classes", &self.classes)
      .field("attributes", &self.attributes)
      .field("reactive_state", &self.reactive_state.is_some())
      .finish()
  }
}

impl fmt::Display for ProgressBar {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.render().content.unwrap_or_default())
  }
}

/// Builder for creating progress bars with fluent API
pub struct ProgressBarBuilder {
  progress_bar: ProgressBar,
}

impl ProgressBarBuilder {
  /// Create new progress bar builder
  pub fn new<S: Into<String>>(id: S) -> Self {
    Self {
      progress_bar: ProgressBar::new(id),
    }
  }

  /// Set progress style
  pub fn style(mut self, style: ProgressStyle) -> Self {
    self.progress_bar.style = style;
    self
  }

  /// Set linear style with custom characters
  pub fn linear(mut self, fill_char: char, empty_char: char, width: Option<u16>) -> Self {
    self.progress_bar.style = ProgressStyle::Linear {
      fill_char,
      empty_char,
      show_percentage: true,
      show_values: false,
      width,
    };
    self
  }

  /// Set circular style
  pub fn circular(mut self, radius: u8, use_unicode: bool) -> Self {
    self.progress_bar.style = ProgressStyle::Circular {
      radius,
      use_unicode,
      show_percentage: true,
    };
    self
  }

  /// Set spinner style with custom frames
  pub fn spinner(mut self, frames: Vec<String>, speed: u64) -> Self {
    self.progress_bar.style = ProgressStyle::Spinner { frames, speed };
    self
  }

  /// Set initial value and range
  pub fn value(mut self, value: f64, min: f64, max: f64) -> Self {
    let _ = self.progress_bar.set_determinate(value, min, max);
    self
  }

  /// Set to indeterminate mode
  pub fn indeterminate(mut self) -> Self {
    self.progress_bar.set_indeterminate();
    self
  }

  /// Set label text
  pub fn label<S: Into<String>>(mut self, label: S) -> Self {
    self.progress_bar.label = Some(label.into());
    self
  }

  /// Add CSS class
  pub fn class<S: Into<String>>(mut self, class: S) -> Self {
    self.progress_bar.classes.push(class.into());
    self
  }

  /// Set color scheme
  pub fn colors(mut self, colors: ProgressColors) -> Self {
    self.progress_bar.colors = colors;
    self
  }

  /// Enable animations
  pub fn animated(mut self, animation: ProgressAnimation) -> Self {
    self.progress_bar.animation = animation;
    self
  }

  /// Build the progress bar
  pub fn build(self) -> ProgressBar {
    self.progress_bar
  }
}

/// Messages for progress bar updates
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ProgressMessage {
  /// Update progress value
  SetValue { id: String, value: f64 },
  /// Update progress percentage
  SetProgress { id: String, progress: f64 },
  /// Update progress range
  SetRange { id: String, min: f64, max: f64 },
  /// Switch to determinate mode
  SetDeterminate {
    id: String,
    value: f64,
    min: f64,
    max: f64,
  },
  /// Switch to indeterminate mode
  SetIndeterminate { id: String },
  /// Update label
  SetLabel { id: String, label: Option<String> },
}

impl Message for ProgressMessage {
  fn clone_message(&self) -> Box<dyn Message> {
    Box::new(self.clone())
  }
}

/// Progress bar manager for handling multiple progress indicators
pub struct ProgressManager {
  progress_bars: HashMap<String, ProgressBar>,
  reactive_state: Option<std::sync::Arc<ReactiveState>>,
}

impl ProgressManager {
  /// Create new progress manager
  pub fn new() -> Self {
    Self {
      progress_bars: HashMap::new(),
      reactive_state: None,
    }
  }

  /// Add a progress bar
  pub fn add_progress(&mut self, progress_bar: ProgressBar) -> Result<()> {
    let id = progress_bar.id.clone();

    // Connect to reactive state if available
    if let Some(ref state) = self.reactive_state {
      let mut pb = progress_bar;
      pb.connect_reactive(state.clone())?;
      self.progress_bars.insert(id, pb);
    } else {
      self.progress_bars.insert(id, progress_bar);
    }

    Ok(())
  }

  /// Remove a progress bar
  pub fn remove_progress(&mut self, id: &str) -> Option<ProgressBar> {
    self.progress_bars.remove(id)
  }

  /// Get progress bar by ID
  pub fn get_progress(&self, id: &str) -> Option<&ProgressBar> {
    self.progress_bars.get(id)
  }

  /// Get mutable progress bar by ID
  pub fn get_progress_mut(&mut self, id: &str) -> Option<&mut ProgressBar> {
    self.progress_bars.get_mut(id)
  }

  /// Update all indeterminate progress animations
  pub fn update_animations(&mut self) -> Vec<String> {
    let mut updated = Vec::new();

    for (id, progress_bar) in &mut self.progress_bars {
      if progress_bar.update_animation() {
        updated.push(id.clone());
      }
    }

    updated
  }

  /// Handle progress message
  pub fn handle_message(&mut self, message: &ProgressMessage) -> Result<()> {
    match message {
      ProgressMessage::SetValue { id, value } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.set_value(*value)?;
        }
      }
      ProgressMessage::SetProgress { id, progress } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.set_progress(*progress)?;
        }
      }
      ProgressMessage::SetRange { id, min, max } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.set_range(*min, *max)?;
        }
      }
      ProgressMessage::SetDeterminate {
        id,
        value,
        min,
        max,
      } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.set_determinate(*value, *min, *max)?;
        }
      }
      ProgressMessage::SetIndeterminate { id } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.set_indeterminate();
        }
      }
      ProgressMessage::SetLabel { id, label } => {
        if let Some(progress_bar) = self.progress_bars.get_mut(id) {
          progress_bar.label = label.clone();
        }
      }
    }

    Ok(())
  }

  /// Connect to reactive state
  pub fn connect_reactive(&mut self, state: std::sync::Arc<ReactiveState>) -> Result<()> {
    // Connect existing progress bars
    for progress_bar in self.progress_bars.values_mut() {
      progress_bar.connect_reactive(state.clone())?;
    }

    self.reactive_state = Some(state);
    Ok(())
  }

  /// Render all progress bars as elements
  pub fn render_all(&self) -> Vec<Element> {
    self.progress_bars.values().map(|pb| pb.render()).collect()
  }
}

impl Default for ProgressManager {
  fn default() -> Self {
    Self::new()
  }
}

/// Predefined spinner styles based on the comprehensive spinners.json collection
pub mod spinners {
  use super::ProgressStyle;

  /// Dots animation - most popular and versatile
  pub fn dots() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "⠋".to_string(),
        "⠙".to_string(),
        "⠹".to_string(),
        "⠸".to_string(),
        "⠼".to_string(),
        "⠴".to_string(),
        "⠦".to_string(),
        "⠧".to_string(),
        "⠇".to_string(),
        "⠏".to_string(),
      ],
      speed: 80,
    }
  }

  /// Dots2 - more subtle braille pattern
  pub fn dots2() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "⣾".to_string(),
        "⣽".to_string(),
        "⣻".to_string(),
        "⢿".to_string(),
        "⡿".to_string(),
        "⣟".to_string(),
        "⣯".to_string(),
        "⣷".to_string(),
      ],
      speed: 80,
    }
  }

  /// Classic spinning bar - ASCII compatible
  pub fn line() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "-".to_string(),
        "\\".to_string(),
        "|".to_string(),
        "/".to_string(),
      ],
      speed: 130,
    }
  }

  /// Simple dots for low-resource environments
  pub fn simple_dots() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        ".  ".to_string(),
        ".. ".to_string(),
        "...".to_string(),
        "   ".to_string(),
      ],
      speed: 400,
    }
  }

  /// Growing vertical bar
  pub fn grow_vertical() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "▁".to_string(),
        "▃".to_string(),
        "▄".to_string(),
        "▅".to_string(),
        "▆".to_string(),
        "▇".to_string(),
        "▆".to_string(),
        "▅".to_string(),
        "▄".to_string(),
        "▃".to_string(),
      ],
      speed: 120,
    }
  }

  /// Growing horizontal bar
  pub fn grow_horizontal() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "▏".to_string(),
        "▎".to_string(),
        "▍".to_string(),
        "▌".to_string(),
        "▋".to_string(),
        "▊".to_string(),
        "▉".to_string(),
        "▊".to_string(),
        "▋".to_string(),
        "▌".to_string(),
        "▍".to_string(),
        "▎".to_string(),
      ],
      speed: 120,
    }
  }

  /// Clock spinner - 12 hour positions
  pub fn clock() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "🕛 ".to_string(),
        "🕐 ".to_string(),
        "🕑 ".to_string(),
        "🕒 ".to_string(),
        "🕓 ".to_string(),
        "🕔 ".to_string(),
        "🕕 ".to_string(),
        "🕖 ".to_string(),
        "🕗 ".to_string(),
        "🕘 ".to_string(),
        "🕙 ".to_string(),
        "🕚 ".to_string(),
      ],
      speed: 100,
    }
  }

  /// Moon phases
  pub fn moon() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "🌑 ".to_string(),
        "🌒 ".to_string(),
        "🌓 ".to_string(),
        "🌔 ".to_string(),
        "🌕 ".to_string(),
        "🌖 ".to_string(),
        "🌗 ".to_string(),
        "🌘 ".to_string(),
      ],
      speed: 80,
    }
  }

  /// Arrow rotation
  pub fn arrow() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "←".to_string(),
        "↖".to_string(),
        "↑".to_string(),
        "↗".to_string(),
        "→".to_string(),
        "↘".to_string(),
        "↓".to_string(),
        "↙".to_string(),
      ],
      speed: 100,
    }
  }

  /// Bouncing bar in brackets
  pub fn bouncing_bar() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "[    ]".to_string(),
        "[=   ]".to_string(),
        "[==  ]".to_string(),
        "[=== ]".to_string(),
        "[====]".to_string(),
        "[ ===]".to_string(),
        "[  ==]".to_string(),
        "[   =]".to_string(),
        "[    ]".to_string(),
        "[   =]".to_string(),
        "[  ==]".to_string(),
        "[ ===]".to_string(),
        "[====]".to_string(),
        "[=== ]".to_string(),
        "[==  ]".to_string(),
        "[=   ]".to_string(),
      ],
      speed: 80,
    }
  }

  /// Bouncing ball
  pub fn bouncing_ball() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "( ●    )".to_string(),
        "(  ●   )".to_string(),
        "(   ●  )".to_string(),
        "(    ● )".to_string(),
        "(     ●)".to_string(),
        "(    ● )".to_string(),
        "(   ●  )".to_string(),
        "(  ●   )".to_string(),
        "( ●    )".to_string(),
        "(●     )".to_string(),
      ],
      speed: 80,
    }
  }

  /// Circle quarters rotation
  pub fn circle_quarters() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "◴".to_string(),
        "◷".to_string(),
        "◶".to_string(),
        "◵".to_string(),
      ],
      speed: 120,
    }
  }

  /// Circle halves rotation
  pub fn circle_halves() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "◐".to_string(),
        "◓".to_string(),
        "◑".to_string(),
        "◒".to_string(),
      ],
      speed: 50,
    }
  }

  /// Triangle rotation
  pub fn triangle() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "◢".to_string(),
        "◣".to_string(),
        "◤".to_string(),
        "◥".to_string(),
      ],
      speed: 50,
    }
  }

  /// Square corners rotation
  pub fn square_corners() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "◰".to_string(),
        "◳".to_string(),
        "◲".to_string(),
        "◱".to_string(),
      ],
      speed: 180,
    }
  }

  /// Binary animation
  pub fn binary() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "010010".to_string(),
        "001100".to_string(),
        "100101".to_string(),
        "111010".to_string(),
        "111101".to_string(),
        "010111".to_string(),
        "101011".to_string(),
        "111000".to_string(),
        "110011".to_string(),
        "110101".to_string(),
      ],
      speed: 80,
    }
  }

  /// Hearts animation
  pub fn hearts() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "💛 ".to_string(),
        "💙 ".to_string(),
        "💜 ".to_string(),
        "💚 ".to_string(),
        "❤️ ".to_string(),
      ],
      speed: 100,
    }
  }

  /// Weather animation
  pub fn weather() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "☀️ ".to_string(),
        "☀️ ".to_string(),
        "☀️ ".to_string(),
        "🌤 ".to_string(),
        "⛅️ ".to_string(),
        "🌥 ".to_string(),
        "☁️ ".to_string(),
        "🌧 ".to_string(),
        "🌨 ".to_string(),
        "🌧 ".to_string(),
        "🌨 ".to_string(),
        "🌧 ".to_string(),
        "🌨 ".to_string(),
        "⛈ ".to_string(),
        "🌨 ".to_string(),
        "🌧 ".to_string(),
        "🌨 ".to_string(),
        "☁️ ".to_string(),
        "🌥 ".to_string(),
        "⛅️ ".to_string(),
        "🌤 ".to_string(),
        "☀️ ".to_string(),
        "☀️ ".to_string(),
      ],
      speed: 100,
    }
  }

  /// Toggle animation
  pub fn toggle() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec!["⊶".to_string(), "⊷".to_string()],
      speed: 250,
    }
  }

  /// Aesthetic progress bar
  pub fn aesthetic() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "▰▱▱▱▱▱▱".to_string(),
        "▰▰▱▱▱▱▱".to_string(),
        "▰▰▰▱▱▱▱".to_string(),
        "▰▰▰▰▱▱▱".to_string(),
        "▰▰▰▰▰▱▱".to_string(),
        "▰▰▰▰▰▰▱".to_string(),
        "▰▰▰▰▰▰▰".to_string(),
        "▰▱▱▱▱▱▱".to_string(),
      ],
      speed: 80,
    }
  }

  /// Point dots
  pub fn point() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec![
        "∙∙∙".to_string(),
        "●∙∙".to_string(),
        "∙●∙".to_string(),
        "∙∙●".to_string(),
        "∙∙∙".to_string(),
      ],
      speed: 125,
    }
  }

  /// Layer animation
  pub fn layer() -> ProgressStyle {
    ProgressStyle::Spinner {
      frames: vec!["-".to_string(), "=".to_string(), "≡".to_string()],
      speed: 150,
    }
  }

  /// Get spinner by name (useful for configuration)
  pub fn by_name(name: &str) -> Option<ProgressStyle> {
    match name {
      "dots" => Some(dots()),
      "dots2" => Some(dots2()),
      "line" => Some(line()),
      "simple_dots" => Some(simple_dots()),
      "grow_vertical" => Some(grow_vertical()),
      "grow_horizontal" => Some(grow_horizontal()),
      "clock" => Some(clock()),
      "moon" => Some(moon()),
      "arrow" => Some(arrow()),
      "bouncing_bar" => Some(bouncing_bar()),
      "bouncing_ball" => Some(bouncing_ball()),
      "circle_quarters" => Some(circle_quarters()),
      "circle_halves" => Some(circle_halves()),
      "triangle" => Some(triangle()),
      "square_corners" => Some(square_corners()),
      "binary" => Some(binary()),
      "hearts" => Some(hearts()),
      "weather" => Some(weather()),
      "toggle" => Some(toggle()),
      "aesthetic" => Some(aesthetic()),
      "point" => Some(point()),
      "layer" => Some(layer()),
      _ => None,
    }
  }

  /// Get all available spinner names
  pub fn available_spinners() -> Vec<&'static str> {
    vec![
      "dots",
      "dots2",
      "line",
      "simple_dots",
      "grow_vertical",
      "grow_horizontal",
      "clock",
      "moon",
      "arrow",
      "bouncing_bar",
      "bouncing_ball",
      "circle_quarters",
      "circle_halves",
      "triangle",
      "square_corners",
      "binary",
      "hearts",
      "weather",
      "toggle",
      "aesthetic",
      "point",
      "layer",
    ]
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_progress_bar_creation() {
    let pb = ProgressBar::new("test");
    assert_eq!(pb.id, "test");
    assert_eq!(pb.progress(), 0.0);
    assert!(matches!(pb.state, ProgressState::Determinate { .. }));
  }

  #[test]
  fn test_progress_value_setting() {
    let mut pb = ProgressBar::new("test");
    pb.set_value(50.0).unwrap();
    assert_eq!(pb.progress(), 0.5); // 50/100 = 0.5

    pb.set_value(150.0).unwrap(); // Should clamp to max
    assert_eq!(pb.progress(), 1.0);

    pb.set_value(-10.0).unwrap(); // Should clamp to min
    assert_eq!(pb.progress(), 0.0);
  }

  #[test]
  fn test_progress_percentage_setting() {
    let mut pb = ProgressBar::new("test");
    pb.set_progress(0.75).unwrap();
    assert_eq!(pb.progress(), 0.75);

    // Should clamp to 0.0-1.0 range
    pb.set_progress(1.5).unwrap();
    assert_eq!(pb.progress(), 1.0);

    pb.set_progress(-0.5).unwrap();
    assert_eq!(pb.progress(), 0.0);
  }

  #[test]
  fn test_progress_range() {
    let mut pb = ProgressBar::new("test");
    pb.set_range(10.0, 20.0).unwrap();
    pb.set_value(15.0).unwrap();
    assert_eq!(pb.progress(), 0.5); // (15-10)/(20-10) = 0.5

    // Invalid range should error
    assert!(pb.set_range(20.0, 10.0).is_err());
  }

  #[test]
  fn test_indeterminate_mode() {
    let mut pb = ProgressBar::new("test");
    pb.set_indeterminate();
    assert!(matches!(pb.state, ProgressState::Indeterminate { .. }));

    // Cannot set value in indeterminate mode
    assert!(pb.set_value(50.0).is_err());
  }

  #[test]
  fn test_builder_pattern() {
    let pb = ProgressBarBuilder::new("test")
      .linear('=', '-', Some(20))
      .value(30.0, 0.0, 100.0)
      .label("Loading")
      .class("my-progress")
      .build();

    assert_eq!(pb.id, "test");
    assert_eq!(pb.progress(), 0.3);
    assert_eq!(pb.label, Some("Loading".to_string()));
    assert!(pb.classes.contains(&"my-progress".to_string()));

    if let ProgressStyle::Linear {
      fill_char,
      empty_char,
      width,
      ..
    } = pb.style
    {
      assert_eq!(fill_char, '=');
      assert_eq!(empty_char, '-');
      assert_eq!(width, Some(20));
    } else {
      panic!("Expected Linear style");
    }
  }

  #[test]
  fn test_linear_rendering() {
    let pb = ProgressBarBuilder::new("test")
      .linear('█', '░', Some(10))
      .value(50.0, 0.0, 100.0)
      .build();

    let rendered = pb.render_linear();
    assert!(rendered.contains('█'));
    assert!(rendered.contains('░'));
    assert!(rendered.contains("50.0%"));
  }

  #[test]
  fn test_easing_functions() {
    let easing = EasingFunction::Linear;
    assert_eq!(easing.apply(0.0), 0.0);
    assert_eq!(easing.apply(0.5), 0.5);
    assert_eq!(easing.apply(1.0), 1.0);

    let ease_out = EasingFunction::EaseOut;
    assert!(ease_out.apply(0.5) > 0.5); // Should be further along
  }

  #[test]
  fn test_progress_manager() {
    let mut manager = ProgressManager::new();

    let pb1 = ProgressBar::new("pb1");
    let pb2 = ProgressBar::new("pb2");

    manager.add_progress(pb1).unwrap();
    manager.add_progress(pb2).unwrap();

    assert!(manager.get_progress("pb1").is_some());
    assert!(manager.get_progress("pb2").is_some());
    assert!(manager.get_progress("pb3").is_none());

    let removed = manager.remove_progress("pb1");
    assert!(removed.is_some());
    assert!(manager.get_progress("pb1").is_none());
  }

  #[test]
  fn test_progress_messages() {
    let mut manager = ProgressManager::new();
    let pb = ProgressBar::new("test");
    manager.add_progress(pb).unwrap();

    let msg = ProgressMessage::SetValue {
      id: "test".to_string(),
      value: 75.0,
    };

    manager.handle_message(&msg).unwrap();

    let progress_bar = manager.get_progress("test").unwrap();
    assert_eq!(progress_bar.progress(), 0.75);
  }

  #[test]
  fn test_predefined_spinners() {
    // Test dots spinner
    let dots_spinner = spinners::dots();
    if let ProgressStyle::Spinner { frames, speed } = dots_spinner {
      assert_eq!(frames.len(), 10);
      assert_eq!(speed, 80);
    } else {
      panic!("Expected Spinner style");
    }

    // Test line spinner
    let line_spinner = spinners::line();
    if let ProgressStyle::Spinner { frames, speed } = line_spinner {
      assert_eq!(frames.len(), 4);
      assert_eq!(speed, 130);
    } else {
      panic!("Expected Spinner style");
    }

    // Test clock spinner
    let clock_spinner = spinners::clock();
    if let ProgressStyle::Spinner { frames, speed } = clock_spinner {
      assert_eq!(frames.len(), 12);
      assert_eq!(speed, 100);
    } else {
      panic!("Expected Spinner style");
    }

    // Test spinner by name functionality
    assert!(spinners::by_name("dots").is_some());
    assert!(spinners::by_name("clock").is_some());
    assert!(spinners::by_name("nonexistent").is_none());

    // Test available spinners list
    let available = spinners::available_spinners();
    assert!(available.contains(&"dots"));
    assert!(available.contains(&"clock"));
    assert!(available.contains(&"hearts"));
    assert!(available.len() > 15); // Should have many spinners
  }
}

// ResponsiveWidget implementation for ProgressBar
impl crate::widgets::ResponsiveWidget for ProgressBar {
  fn to_element(&self) -> crate::components::Element {
    let mut builder = crate::components::Element::with_tag("progress")
      .id(&self.id);

    // Add progress attributes based on state
    match &self.state {
      ProgressState::Determinate { value, min, max } => {
        builder = builder
          .attr("value", &value.to_string())
          .attr("min", &min.to_string())
          .attr("max", &max.to_string())
          .attr("data-progress", &self.progress().to_string());
      }
      ProgressState::Indeterminate { .. } => {
        builder = builder.attr("indeterminate", "true");
      }
    }

    // Add label if present
    if let Some(label) = &self.label {
      builder = builder.attr("aria-label", label);
    }

    // Add CSS classes
    for class in &self.classes {
      builder = builder.class(class);
    }

    // Add style-specific classes and attributes
    match &self.style {
      ProgressStyle::Linear { fill_char, empty_char, width, show_percentage, .. } => {
        builder = builder
          .class("linear")
          .attr("data-fill-char", &fill_char.to_string())
          .attr("data-empty-char", &empty_char.to_string())
          .attr("data-show-percentage", &show_percentage.to_string());

        if let Some(w) = width {
          builder = builder.attr("data-width", &w.to_string());
        }
      }
      ProgressStyle::Circular { radius, use_unicode, show_percentage } => {
        builder = builder
          .class("circular")
          .attr("data-radius", &radius.to_string())
          .attr("data-use-unicode", &use_unicode.to_string())
          .attr("data-show-percentage", &show_percentage.to_string());
      }
      ProgressStyle::Arc { radius, start_angle, sweep_angle, show_percentage } => {
        builder = builder
          .class("arc")
          .attr("data-radius", &radius.to_string())
          .attr("data-start-angle", &start_angle.to_string())
          .attr("data-sweep-angle", &sweep_angle.to_string())
          .attr("data-show-percentage", &show_percentage.to_string());
      }
      ProgressStyle::Spinner { frames, speed } => {
        builder = builder
          .class("spinner")
          .attr("data-frame-count", &frames.len().to_string())
          .attr("data-speed", &speed.to_string());
      }
    }

    // Add animation state
    let animation = &self.animation;
    if animation.smooth_transitions {
      builder = builder
        .class("animated")
        .attr("data-animation-duration", &animation.transition_duration.to_string())
        .attr("data-animation-easing", match animation.easing {
          EasingFunction::Linear => "linear",
          EasingFunction::EaseIn => "ease-in",
          EasingFunction::EaseOut => "ease-out",
          EasingFunction::EaseInOut => "ease-in-out",
          EasingFunction::Bounce => "bounce",
          EasingFunction::Elastic => "elastic",
        });
    }

    if animation.pulse_on_complete && self.progress() >= 1.0 {
      builder = builder.class("pulse-complete");
    }

    builder.build()
  }

  fn render_with_layout(&self, _layout: &crate::layout::LayoutRect, _theme: Option<&crate::themes::ColorTheme>) -> String {
    // Use the existing render methods based on style
    match &self.style {
      ProgressStyle::Linear { .. } => self.render_linear(),
      ProgressStyle::Circular { .. } => self.render_circular(),
      ProgressStyle::Arc { .. } => self.render_circular(), // Arc uses similar rendering to circular
      ProgressStyle::Spinner { .. } => self.render_spinner(),
    }
  }

  fn min_size(&self) -> (u16, u16) {
    match &self.style {
      ProgressStyle::Linear { width, show_percentage, .. } => {
        let bar_width = width.unwrap_or(20);
        let percentage_width = if *show_percentage { 6 } else { 0 }; // " 100%"
        let label_width = self.label.as_ref().map(|l| l.chars().count() as u16 + 1).unwrap_or(0);

        let total_width = bar_width + percentage_width + label_width;
        (total_width.max(10), 1)
      }
      ProgressStyle::Circular { radius, .. } => {
        let diameter = (radius * 2) as u16;
        (diameter.max(5), diameter.max(3))
      }
      ProgressStyle::Arc { radius, .. } => {
        let diameter = (radius * 2) as u16;
        (diameter.max(5), diameter.max(3))
      }
      ProgressStyle::Spinner { frames, .. } => {
        let max_frame_width = frames.iter()
          .map(|f| f.chars().count() as u16)
          .max()
          .unwrap_or(1);
        let label_width = self.label.as_ref().map(|l| l.chars().count() as u16 + 1).unwrap_or(0);

        (max_frame_width + label_width, 1)
      }
    }
  }

  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    match &self.style {
      ProgressStyle::Linear { .. } => {
        // Linear progress bars can grow horizontally but have fixed height
        (None, Some(1))
      }
      ProgressStyle::Circular { radius, .. } => {
        // Circular progress bars have fixed size based on radius
        let diameter = (radius * 2) as u16;
        (Some(diameter), Some(diameter))
      }
      ProgressStyle::Arc { radius, .. } => {
        // Arc progress bars have fixed size based on radius
        let diameter = (radius * 2) as u16;
        (Some(diameter), Some(diameter))
      }
      ProgressStyle::Spinner { .. } => {
        // Spinners have fixed size based on their frames
        let (min_width, min_height) = self.min_size();
        (Some(min_width), Some(min_height))
      }
    }
  }

  fn can_grow_horizontal(&self) -> bool {
    matches!(self.style, ProgressStyle::Linear { .. })
  }

  fn can_grow_vertical(&self) -> bool {
    false // Progress bars generally have fixed height
  }
}
