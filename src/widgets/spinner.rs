//! Spinner widget for loading states and progress indication
//!
//! Provides animated spinners with predefined types, custom definitions,
//! and comprehensive styling options.

use crate::{
    components::Element, error::Result, layout::LayoutRect, reactive::ReactiveState,
    themes::ColorTheme,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};

/// Predefined spinner types based on popular spinner collections
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpinnerType {
    /// Classic braille dot spinner (⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏)
    Dots,
    /// Enhanced braille dots (⣾⣽⣻⢿⡿⣟⣯⣷)
    Dots2,
    /// Simple rotating line (-\|/)
    Line,
    /// Box drawing pipe characters (┤┘┴└├┌┬┐)
    Pipe,
    /// Simple dot progression (...   )
    SimpleDots,
    /// Scrolling dots (.  .. ... ..)
    SimpleDotsScrolling,
    /// Star animation (✶✸✹✺✹✷)
    Star,
    /// Simple star (+x*)
    Star2,
    /// Arc animation (◜◠◝◞◡◟)
    Arc,
    /// Circle animation (◡⊙◠)
    Circle,
    /// Circle quarters (◴◷◶◵)
    CircleQuarters,
    /// Circle halves (◐◓◑◒)
    CircleHalves,
    /// Square corners (◰◳◲◱)
    SquareCorners,
    /// Toggle animation (⊶⊷)
    Toggle,
    /// Box toggle (▫▪)
    Toggle2,
    /// Square toggle (□■)
    Toggle3,
    /// Triangle animation (◢◣◤◥)
    Triangle,
    /// Bouncing animation (⠁⠂⠄⠂)
    Bounce,
    /// Box bounce (▖▘▝▗)
    BoxBounce,
    /// Growing vertical (▁▃▄▅▆▇▆▅▄▃)
    GrowVertical,
    /// Growing horizontal (▏▎▍▌▋▊▉▊▋▌▍▎)
    GrowHorizontal,
    /// Balloon animation ( .oO@* )
    Balloon,
    /// Noise animation (▓▒░)
    Noise,
    /// Arrow rotation (←↖↑↗→↘↓↙)
    Arrow,
    /// Bouncing bar ([====])
    BouncingBar,
    /// Bouncing ball (( ●    ))
    BouncingBall,
    /// Hearts emoji (💛💙💜💚❤️)
    Hearts,
    /// Clock emoji (🕛🕐🕑🕒...)
    Clock,
    /// Earth emoji (🌍🌎🌏)
    Earth,
    /// Moon phases (🌑🌒🌓🌔🌕🌖🌗🌘)
    Moon,
    /// Weather animation (☀️🌤⛅️🌥☁️🌧🌨⛈)
    Weather,
    /// Smiley faces (😄😝)
    Smiley,
    /// Monkey emoji (🙈🙉🙊)
    Monkey,
    /// Runner animation (🚶🏃)
    Runner,
    /// Christmas tree (🌲🎄)
    Christmas,
}

/// Custom spinner definition
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinnerDefinition {
    /// Animation frames
    pub frames: Vec<String>,
    /// Milliseconds between frames
    pub interval: u64,
    /// Optional name for the spinner
    pub name: Option<String>,
}

impl SpinnerDefinition {
    /// Create a spinner definition from static frames (efficient for predefined spinners)
    pub fn from_static(
        frames: &'static [&'static str],
        interval: u64,
        name: Option<&'static str>,
    ) -> Self {
        Self {
            frames: frames.iter().map(|s| (*s).to_string()).collect(),
            interval,
            name: name.map(|s| s.to_string()),
        }
    }
}

/// Label positioning relative to spinner
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpinnerLabelPosition {
    /// Label before spinner (left/above)
    Before,
    /// Label after spinner (right/below)
    After,
    /// Label above spinner
    Above,
    /// Label below spinner
    Below,
    /// No label
    None,
}

impl Default for SpinnerLabelPosition {
    fn default() -> Self {
        Self::After
    }
}

/// Spinner animation state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SpinnerAnimationState {
    /// Animation is running
    Running,
    /// Animation is paused
    Paused,
    /// Animation is stopped
    Stopped,
}

impl Default for SpinnerAnimationState {
    fn default() -> Self {
        Self::Stopped
    }
}

/// Spinner widget state
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinnerState {
    /// Current animation state
    pub animation_state: SpinnerAnimationState,
    /// Current frame index
    pub current_frame: usize,
    /// Whether the spinner is visible
    pub visible: bool,
    /// Last update time for animation timing
    #[serde(skip)]
    pub last_update: Option<Instant>,
}

impl Default for SpinnerState {
    fn default() -> Self {
        Self {
            animation_state: SpinnerAnimationState::default(),
            current_frame: 0,
            visible: true,
            last_update: None,
        }
    }
}

/// Spinner widget styling
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SpinnerStyle {
    /// Label text
    pub label: Option<String>,
    /// Label position relative to spinner
    pub label_position: SpinnerLabelPosition,
    /// Spacing between spinner and label
    pub spacing: u16,
    /// Whether to show the spinner
    pub show_spinner: bool,
    /// Custom prefix text
    pub prefix: Option<String>,
    /// Custom suffix text
    pub suffix: Option<String>,
}

impl Default for SpinnerStyle {
    fn default() -> Self {
        Self {
            label: None,
            label_position: SpinnerLabelPosition::default(),
            spacing: 1,
            show_spinner: true,
            prefix: None,
            suffix: None,
        }
    }
}

/// Main spinner widget
#[derive(Clone)]
pub struct Spinner {
    /// Widget identifier
    pub id: String,
    /// Spinner definition (frames and timing)
    pub definition: SpinnerDefinition,
    /// Current state
    pub state: SpinnerState,
    /// Visual styling
    pub style: SpinnerStyle,
    /// Reactive state for live updates
    pub reactive_state: Option<Arc<ReactiveState>>,
}

impl Spinner {
    /// Create a new spinner with a predefined type
    pub fn new<S: Into<String>>(id: S, spinner_type: SpinnerType) -> Self {
        Self {
            id: id.into(),
            definition: Self::get_predefined_definition(spinner_type),
            state: SpinnerState::default(),
            style: SpinnerStyle::default(),
            reactive_state: None,
        }
    }

    /// Create a spinner with a custom definition
    pub fn with_custom<S: Into<String>>(id: S, definition: SpinnerDefinition) -> Self {
        Self {
            id: id.into(),
            definition,
            state: SpinnerState::default(),
            style: SpinnerStyle::default(),
            reactive_state: None,
        }
    }

    /// Set the label text
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.style.label = Some(label.into());
        self.sync_reactive_state();
        self
    }

    /// Set the label position
    pub fn label_position(mut self, position: SpinnerLabelPosition) -> Self {
        self.style.label_position = position;
        self.sync_reactive_state();
        self
    }

    /// Set spacing between spinner and label
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.style.spacing = spacing;
        self
    }

    /// Set prefix text
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.style.prefix = Some(prefix.into());
        self
    }

    /// Set suffix text
    pub fn suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.style.suffix = Some(suffix.into());
        self
    }

    /// Hide the spinner (show only label)
    pub fn hide_spinner(mut self) -> Self {
        self.style.show_spinner = false;
        self.sync_reactive_state();
        self
    }

    /// Connect to reactive state for live updates
    pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()> {
        // Initialize reactive fields
        state.set_field(
            &format!("{}.animation_state", self.id),
            format!("{:?}", self.state.animation_state),
        );
        state.set_field(
            &format!("{}.current_frame", self.id),
            self.state.current_frame,
        );
        state.set_field(&format!("{}.visible", self.id), self.state.visible);

        self.reactive_state = Some(state);
        Ok(())
    }

    /// Sync state to reactive state if connected
    fn sync_reactive_state(&self) {
        if let Some(reactive) = &self.reactive_state {
            reactive.set_field(
                &format!("{}.animation_state", self.id),
                format!("{:?}", self.state.animation_state),
            );
            reactive.set_field(
                &format!("{}.current_frame", self.id),
                self.state.current_frame,
            );
            reactive.set_field(&format!("{}.visible", self.id), self.state.visible);
        }
    }

    /// Start the spinner animation
    pub fn start(&mut self) -> Result<()> {
        self.state.animation_state = SpinnerAnimationState::Running;
        self.state.last_update = Some(Instant::now());
        self.sync_reactive_state();
        Ok(())
    }

    /// Stop the spinner animation
    pub fn stop(&mut self) -> Result<()> {
        self.state.animation_state = SpinnerAnimationState::Stopped;
        self.state.current_frame = 0;
        self.state.last_update = None;
        self.sync_reactive_state();
        Ok(())
    }

    /// Pause the spinner animation
    pub fn pause(&mut self) -> Result<()> {
        self.state.animation_state = SpinnerAnimationState::Paused;
        self.sync_reactive_state();
        Ok(())
    }

    /// Resume the spinner animation
    pub fn resume(&mut self) -> Result<()> {
        if self.state.animation_state == SpinnerAnimationState::Paused {
            self.state.animation_state = SpinnerAnimationState::Running;
            self.state.last_update = Some(Instant::now());
            self.sync_reactive_state();
        }
        Ok(())
    }

    /// Update animation frame if enough time has passed
    pub fn update(&mut self) -> Result<bool> {
        if self.state.animation_state != SpinnerAnimationState::Running {
            return Ok(false);
        }

        let now = Instant::now();
        if let Some(last_update) = self.state.last_update {
            let elapsed = now.duration_since(last_update);
            let interval = Duration::from_millis(self.definition.interval);

            if elapsed >= interval {
                self.state.current_frame =
                    (self.state.current_frame + 1) % self.definition.frames.len();
                self.state.last_update = Some(now);
                self.sync_reactive_state();
                return Ok(true);
            }
        }

        Ok(false)
    }

    /// Get the current frame text
    pub fn current_frame(&self) -> &str {
        if self.definition.frames.is_empty() {
            ""
        } else {
            &self.definition.frames[self.state.current_frame]
        }
    }

    /// Check if the spinner is running
    pub fn is_running(&self) -> bool {
        self.state.animation_state == SpinnerAnimationState::Running
    }

    /// Check if the spinner is visible
    pub fn is_visible(&self) -> bool {
        self.state.visible
    }

    /// Set visibility
    pub fn set_visible(&mut self, visible: bool) -> Result<()> {
        self.state.visible = visible;
        self.sync_reactive_state();
        Ok(())
    }

    /// Get predefined spinner definition
    fn get_predefined_definition(spinner_type: SpinnerType) -> SpinnerDefinition {
        match spinner_type {
            SpinnerType::Dots => SpinnerDefinition::from_static(
                &["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"],
                80,
                Some("dots"),
            ),
            SpinnerType::Dots2 => SpinnerDefinition::from_static(
                &["⣾", "⣽", "⣻", "⢿", "⡿", "⣟", "⣯", "⣷"],
                80,
                Some("dots2"),
            ),
            SpinnerType::Line => {
                SpinnerDefinition::from_static(&["-", "\\", "|", "/"], 130, Some("line"))
            }
            SpinnerType::Pipe => SpinnerDefinition::from_static(
                &["┤", "┘", "┴", "└", "├", "┌", "┬", "┐"],
                100,
                Some("pipe"),
            ),
            SpinnerType::SimpleDots => SpinnerDefinition::from_static(
                &[".  ", ".. ", "...", "   "],
                400,
                Some("simpleDots"),
            ),
            SpinnerType::SimpleDotsScrolling => SpinnerDefinition::from_static(
                &[".  ", ".. ", "...", " ..", "  .", "   "],
                200,
                Some("simpleDotsScrolling"),
            ),
            SpinnerType::Star => {
                SpinnerDefinition::from_static(&["✶", "✸", "✹", "✺", "✹", "✷"], 70, Some("star"))
            }
            SpinnerType::Star2 => {
                SpinnerDefinition::from_static(&["+", "x", "*"], 80, Some("star2"))
            }
            SpinnerType::Arc => {
                SpinnerDefinition::from_static(&["◜", "◠", "◝", "◞", "◡", "◟"], 100, Some("arc"))
            }
            SpinnerType::Circle => {
                SpinnerDefinition::from_static(&["◡", "⊙", "◠"], 120, Some("circle"))
            }
            SpinnerType::CircleQuarters => {
                SpinnerDefinition::from_static(&["◴", "◷", "◶", "◵"], 120, Some("circleQuarters"))
            }
            SpinnerType::CircleHalves => {
                SpinnerDefinition::from_static(&["◐", "◓", "◑", "◒"], 50, Some("circleHalves"))
            }
            SpinnerType::SquareCorners => {
                SpinnerDefinition::from_static(&["◰", "◳", "◲", "◱"], 180, Some("squareCorners"))
            }
            SpinnerType::Toggle => SpinnerDefinition::from_static(&["⊶", "⊷"], 250, Some("toggle")),
            SpinnerType::Toggle2 => {
                SpinnerDefinition::from_static(&["▫", "▪"], 80, Some("toggle2"))
            }
            SpinnerType::Toggle3 => {
                SpinnerDefinition::from_static(&["□", "■"], 120, Some("toggle3"))
            }
            SpinnerType::Triangle => {
                SpinnerDefinition::from_static(&["◢", "◣", "◤", "◥"], 50, Some("triangle"))
            }
            SpinnerType::Bounce => {
                SpinnerDefinition::from_static(&["⠁", "⠂", "⠄", "⠂"], 120, Some("bounce"))
            }
            SpinnerType::BoxBounce => {
                SpinnerDefinition::from_static(&["▖", "▘", "▝", "▗"], 120, Some("boxBounce"))
            }
            SpinnerType::GrowVertical => SpinnerDefinition::from_static(
                &["▁", "▃", "▄", "▅", "▆", "▇", "▆", "▅", "▄", "▃"],
                120,
                Some("growVertical"),
            ),
            SpinnerType::GrowHorizontal => SpinnerDefinition::from_static(
                &["▏", "▎", "▍", "▌", "▋", "▊", "▉", "▊", "▋", "▌", "▍", "▎"],
                120,
                Some("growHorizontal"),
            ),
            SpinnerType::Balloon => SpinnerDefinition::from_static(
                &[" ", ".", "o", "O", "@", "*", " "],
                140,
                Some("balloon"),
            ),
            SpinnerType::Noise => {
                SpinnerDefinition::from_static(&["▓", "▒", "░"], 100, Some("noise"))
            }
            SpinnerType::Arrow => SpinnerDefinition::from_static(
                &["←", "↖", "↑", "↗", "→", "↘", "↓", "↙"],
                100,
                Some("arrow"),
            ),
            SpinnerType::BouncingBar => SpinnerDefinition::from_static(
                &[
                    "[    ]", "[=   ]", "[==  ]", "[=== ]", "[====]", "[ ===]", "[  ==]", "[   =]",
                    "[    ]", "[   =]", "[  ==]", "[ ===]", "[====]", "[=== ]", "[==  ]", "[=   ]",
                ],
                80,
                Some("bouncingBar"),
            ),
            SpinnerType::BouncingBall => SpinnerDefinition::from_static(
                &[
                    "( ●    )",
                    "(  ●   )",
                    "(   ●  )",
                    "(    ● )",
                    "(     ●)",
                    "(    ● )",
                    "(   ●  )",
                    "(  ●   )",
                    "( ●    )",
                    "(●     )",
                ],
                80,
                Some("bouncingBall"),
            ),
            SpinnerType::Hearts => SpinnerDefinition::from_static(
                &["💛 ", "💙 ", "💜 ", "💚 ", "❤️ "],
                100,
                Some("hearts"),
            ),
            SpinnerType::Clock => SpinnerDefinition::from_static(
                &[
                    "🕛 ", "🕐 ", "🕑 ", "🕒 ", "🕓 ", "🕔 ", "🕕 ", "🕖 ", "🕗 ", "🕘 ", "🕙 ",
                    "🕚 ",
                ],
                100,
                Some("clock"),
            ),
            SpinnerType::Earth => {
                SpinnerDefinition::from_static(&["🌍 ", "🌎 ", "🌏 "], 180, Some("earth"))
            }
            SpinnerType::Moon => SpinnerDefinition::from_static(
                &["🌑 ", "🌒 ", "🌓 ", "🌔 ", "🌕 ", "🌖 ", "🌗 ", "🌘 "],
                80,
                Some("moon"),
            ),
            SpinnerType::Weather => SpinnerDefinition::from_static(
                &[
                    "☀️ ", "☀️ ", "☀️ ", "🌤 ", "⛅️ ", "🌥 ", "☁️ ", "🌧 ", "🌨 ", "🌧 ", "🌨 ", "🌧 ",
                    "🌨 ", "⛈ ", "🌨 ", "🌧 ", "🌨 ", "☁️ ", "🌥 ", "⛅️ ", "🌤 ", "☀️ ", "☀️ ",
                ],
                100,
                Some("weather"),
            ),
            SpinnerType::Smiley => {
                SpinnerDefinition::from_static(&["😄 ", "😝 "], 200, Some("smiley"))
            }
            SpinnerType::Monkey => {
                SpinnerDefinition::from_static(&["🙈 ", "🙈 ", "🙉 ", "🙊 "], 300, Some("monkey"))
            }
            SpinnerType::Runner => {
                SpinnerDefinition::from_static(&["🚶 ", "🏃 "], 140, Some("runner"))
            }
            SpinnerType::Christmas => {
                SpinnerDefinition::from_static(&["🌲", "🎄"], 400, Some("christmas"))
            }
        }
    }

    /// Render the spinner as a string
    pub fn render_string(&self) -> String {
        if !self.state.visible {
            return String::new();
        }

        let mut result = String::new();

        // Add prefix if present
        if let Some(prefix) = &self.style.prefix {
            result.push_str(prefix);
        }

        // Handle label positioning
        match self.style.label_position {
            SpinnerLabelPosition::Before => {
                if let Some(label) = &self.style.label {
                    result.push_str(label);
                    result.push_str(&" ".repeat(self.style.spacing as usize));
                }
            }
            SpinnerLabelPosition::Above => {
                if let Some(label) = &self.style.label {
                    result.push_str(label);
                    result.push('\n');
                }
            }
            _ => {}
        }

        // Add spinner if visible
        if self.style.show_spinner {
            result.push_str(self.current_frame());
        }

        // Handle label positioning (after/below)
        match self.style.label_position {
            SpinnerLabelPosition::After => {
                if let Some(label) = &self.style.label {
                    result.push_str(&" ".repeat(self.style.spacing as usize));
                    result.push_str(label);
                }
            }
            SpinnerLabelPosition::Below => {
                if let Some(label) = &self.style.label {
                    result.push('\n');
                    result.push_str(label);
                }
            }
            _ => {}
        }

        // Add suffix if present
        if let Some(suffix) = &self.style.suffix {
            result.push_str(suffix);
        }

        result
    }

    /// Render the spinner with layout and theme support
    pub fn render(&self, _layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
        self.render_string()
    }

    /// Convert to Element for integration with the component system
    pub fn to_element(&self) -> Element {
        let content = self.render_string();

        let mut element = Element::with_tag("spinner")
            .id(&self.id)
            .content(content)
            .class("spinner")
            .attr("role", "status")
            .attr("aria-live", "polite");

        // Add state attributes
        element = element
            .attr(
                "data-animation-state",
                format!("{:?}", self.state.animation_state),
            )
            .attr("data-current-frame", self.state.current_frame.to_string())
            .attr("data-visible", self.state.visible.to_string());

        // Add spinner type if available
        if let Some(name) = &self.definition.name {
            element = element.attr("data-spinner-type", name);
        }

        // Add accessibility label
        if let Some(label) = &self.style.label {
            element = element.attr("aria-label", format!("Loading: {label}"));
        } else {
            element = element.attr("aria-label", "Loading");
        }

        // Add CSS classes based on state
        match self.state.animation_state {
            SpinnerAnimationState::Running => element = element.class("spinner-running"),
            SpinnerAnimationState::Paused => element = element.class("spinner-paused"),
            SpinnerAnimationState::Stopped => element = element.class("spinner-stopped"),
        }

        if !self.state.visible {
            element = element.class("spinner-hidden");
        }

        element.build()
    }
}

/// Builder for creating spinners
pub struct SpinnerBuilder {
    spinner: Spinner,
}

impl SpinnerBuilder {
    /// Create a new spinner builder with predefined type
    pub fn new<S: Into<String>>(id: S, spinner_type: SpinnerType) -> Self {
        Self {
            spinner: Spinner::new(id, spinner_type),
        }
    }

    /// Create a spinner builder with custom definition
    pub fn with_custom<S: Into<String>>(id: S, definition: SpinnerDefinition) -> Self {
        Self {
            spinner: Spinner::with_custom(id, definition),
        }
    }

    /// Set label
    pub fn label<S: Into<String>>(mut self, label: S) -> Self {
        self.spinner = self.spinner.label(label);
        self
    }

    /// Set label position
    pub fn label_position(mut self, position: SpinnerLabelPosition) -> Self {
        self.spinner = self.spinner.label_position(position);
        self
    }

    /// Set spacing
    pub fn spacing(mut self, spacing: u16) -> Self {
        self.spinner = self.spinner.spacing(spacing);
        self
    }

    /// Set prefix
    pub fn prefix<S: Into<String>>(mut self, prefix: S) -> Self {
        self.spinner = self.spinner.prefix(prefix);
        self
    }

    /// Set suffix
    pub fn suffix<S: Into<String>>(mut self, suffix: S) -> Self {
        self.spinner = self.spinner.suffix(suffix);
        self
    }

    /// Hide spinner (show only label)
    pub fn hide_spinner(mut self) -> Self {
        self.spinner = self.spinner.hide_spinner();
        self
    }

    /// Build the spinner
    pub fn build(self) -> Spinner {
        self.spinner
    }
}

/// Convenience function for creating a spinner
pub fn spinner<S: Into<String>>(id: S, spinner_type: SpinnerType) -> SpinnerBuilder {
    SpinnerBuilder::new(id, spinner_type)
}

/// Create a loading spinner with default settings
pub fn loading_spinner<S: Into<String>>(id: S) -> Spinner {
    Spinner::new(id, SpinnerType::Dots)
        .label("Loading...")
        .label_position(SpinnerLabelPosition::After)
}

/// Create a processing spinner
pub fn processing_spinner<S: Into<String>>(id: S) -> Spinner {
    Spinner::new(id, SpinnerType::Arc)
        .label("Processing...")
        .label_position(SpinnerLabelPosition::After)
}

/// Create a saving spinner
pub fn saving_spinner<S: Into<String>>(id: S) -> Spinner {
    Spinner::new(id, SpinnerType::CircleHalves)
        .label("Saving...")
        .label_position(SpinnerLabelPosition::After)
}
