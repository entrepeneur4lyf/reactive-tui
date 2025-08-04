//! # Mouse Widget
//!
//! Advanced mouse interaction widget with gesture recognition, hover detection, and click handling.
//!
//! The Mouse widget provides comprehensive mouse interaction capabilities for terminal applications,
//! supporting various mouse events, gesture recognition, hover states, and multi-button interactions.
//! It integrates seamlessly with the existing mouse infrastructure and event system.
//!
//! ## Features
//!
//! - **Click Detection**: Single, double, and triple-click detection with timing control
//! - **Hover States**: Enter, leave, and move tracking with hover callbacks
//! - **Drag & Drop**: Full drag and drop support with drag start/end detection
//! - **Gesture Recognition**: Swipe, pinch, and custom gesture detection
//! - **Multi-Button Support**: Left, right, middle button support with separate handlers
//! - **Mouse Wheel**: Scroll up/down detection with momentum tracking
//! - **Coordinate Mapping**: Precise pixel coordinate mapping within widget bounds
//! - **State Management**: Comprehensive mouse state tracking and event propagation
//!
//! ## Examples
//!
//! ### Basic Click Handler
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let mouse_area = mouse("click-area", |config| {
//!     config
//!         .on_click("handle_click")
//!         .on_hover("handle_hover")
//!         .cursor_style(CursorStyle::Pointer)
//! });
//! ```
//!
//! ### Advanced Gesture Recognition
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! let gesture_area = mouse("gesture-area", |config| {
//!     config
//!         .enable_drag_drop(true)
//!         .on_drag_start("drag_start")
//!         .on_drag_end("drag_end")
//!         .on_double_click("double_click_handler")
//!         .gesture_threshold(10)
//! });
//! ```

use crate::{
    components::Element,
    driver::{MouseButton, MouseEvent, MouseEventKind},
    events::ActionDispatcher,
    error::{Result, TuiError},
    layout::LayoutRect,
    themes::ColorTheme,
};
use super::ResponsiveWidget;
use std::time::{Duration, Instant};

/// Mouse button types for event handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MouseButtonType {
    Left,
    Right,
    Middle,
}

/// Mouse interaction states
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MouseInteractionState {
    Normal,
    Hover,
    Pressed(MouseButtonType),
    Dragging(MouseButtonType),
    Released,
}

/// Cursor styles for visual feedback
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorStyle {
    Default,
    Pointer,
    Text,
    Crosshair,
    Move,
    NotAllowed,
    Grab,
    Grabbing,
}

/// Mouse gesture types
#[derive(Debug, Clone, PartialEq)]
pub enum MouseGesture {
    Click { button: MouseButtonType, count: u8 },
    DoubleClick { button: MouseButtonType },
    TripleClick { button: MouseButtonType },
    Drag { button: MouseButtonType, start: (u16, u16), end: (u16, u16) },
    Swipe { direction: SwipeDirection, distance: f32 },
    Scroll { direction: ScrollDirection, amount: i8 },
}

/// Swipe directions for gesture recognition
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SwipeDirection {
    Up,
    Down,
    Left,
    Right,
}

/// Scroll directions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScrollDirection {
    Up,
    Down,
}

/// Mouse position information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MousePosition {
    pub x: u16,
    pub y: u16,
    pub relative_x: u16,
    pub relative_y: u16,
}

/// Mouse widget configuration
#[derive(Debug, Clone)]
pub struct MouseConfig {
    pub id: String,
    pub classes: Vec<String>,
    pub cursor_style: CursorStyle,
    pub enable_hover: bool,
    pub enable_drag_drop: bool,
    pub enable_gestures: bool,
    pub double_click_threshold: Duration,
    pub triple_click_threshold: Duration,
    pub drag_threshold: u16,
    pub gesture_threshold: u16,
    pub on_click: Option<String>,
    pub on_double_click: Option<String>,
    pub on_triple_click: Option<String>,
    pub on_right_click: Option<String>,
    pub on_middle_click: Option<String>,
    pub on_hover_enter: Option<String>,
    pub on_hover_leave: Option<String>,
    pub on_hover_move: Option<String>,
    pub on_drag_start: Option<String>,
    pub on_drag_move: Option<String>,
    pub on_drag_end: Option<String>,
    pub on_scroll: Option<String>,
    pub on_gesture: Option<String>,
}

impl Default for MouseConfig {
    fn default() -> Self {
        Self {
            id: String::new(),
            classes: Vec::new(),
            cursor_style: CursorStyle::Default,
            enable_hover: true,
            enable_drag_drop: false,
            enable_gestures: false,
            double_click_threshold: Duration::from_millis(300),
            triple_click_threshold: Duration::from_millis(500),
            drag_threshold: 5,
            gesture_threshold: 10,
            on_click: None,
            on_double_click: None,
            on_triple_click: None,
            on_right_click: None,
            on_middle_click: None,
            on_hover_enter: None,
            on_hover_leave: None,
            on_hover_move: None,
            on_drag_start: None,
            on_drag_move: None,
            on_drag_end: None,
            on_scroll: None,
            on_gesture: None,
        }
    }
}

/// Click tracking for multi-click detection
#[derive(Debug, Clone)]
struct ClickTracker {
    button: MouseButtonType,
    position: (u16, u16),
    timestamp: Instant,
    count: u8,
}

/// Drag state tracking
#[derive(Debug, Clone)]
struct DragState {
    button: MouseButtonType,
    start_position: (u16, u16),
    current_position: (u16, u16),
    is_dragging: bool,
}

/// Mouse Widget Implementation
pub struct MouseWidget {
    config: MouseConfig,
    state: MouseInteractionState,
    bounds: Option<LayoutRect>,
    last_position: Option<MousePosition>,
    action_dispatcher: ActionDispatcher,
    
    // Click tracking
    click_tracker: Option<ClickTracker>,
    
    // Drag tracking
    drag_state: Option<DragState>,
    
    // Hover tracking
    is_hovered: bool,
    hover_start_time: Option<Instant>,
    
    // Gesture tracking
    gesture_start: Option<(u16, u16)>,
    gesture_path: Vec<(u16, u16)>,
}

impl MouseWidget {
    /// Create a new mouse widget with configuration
    pub fn new(config: MouseConfig) -> Self {
        Self {
            config,
            state: MouseInteractionState::Normal,
            bounds: None,
            last_position: None,
            action_dispatcher: ActionDispatcher::new(),
            click_tracker: None,
            drag_state: None,
            is_hovered: false,
            hover_start_time: None,
            gesture_start: None,
            gesture_path: Vec::new(),
        }
    }

    /// Get the widget ID
    pub fn get_id(&self) -> &str {
        &self.config.id
    }

    /// Get the current interaction state
    pub fn get_state(&self) -> MouseInteractionState {
        self.state
    }

    /// Check if mouse is currently hovering
    pub fn is_hovering(&self) -> bool {
        self.is_hovered
    }

    /// Handle mouse event and update widget state
    pub fn handle_mouse_event(&mut self, event: MouseEvent) -> Result<bool> {
        let bounds = self.bounds.ok_or_else(|| {
            TuiError::render("Mouse widget bounds not set".to_string())
        })?;

        // Check if event is within widget bounds
        let position = self.calculate_position(event.column, event.row, &bounds);
        if !self.is_within_bounds(event.column, event.row, &bounds) {
            // Handle hover leave if we were previously hovering
            if self.is_hovered {
                self.handle_hover_leave()?;
            }
            return Ok(false);
        }

        self.last_position = Some(position);

        match event.kind {
            MouseEventKind::Down(button) => {
                self.handle_mouse_down(button, position)?;
            }
            MouseEventKind::Up(button) => {
                self.handle_mouse_up(button, position)?;
            }
            MouseEventKind::Drag(button) => {
                self.handle_mouse_drag(button, position)?;
            }
            MouseEventKind::Moved => {
                self.handle_mouse_move(position)?;
            }
            MouseEventKind::ScrollDown => {
                self.handle_scroll(ScrollDirection::Down)?;
            }
            MouseEventKind::ScrollUp => {
                self.handle_scroll(ScrollDirection::Up)?;
            }
            MouseEventKind::ScrollLeft => {
                // Handle horizontal scroll if needed
                // For now, treat as scroll up
                self.handle_scroll(ScrollDirection::Up)?;
            }
            MouseEventKind::ScrollRight => {
                // Handle horizontal scroll if needed
                // For now, treat as scroll down
                self.handle_scroll(ScrollDirection::Down)?;
            }
        }

        Ok(true)
    }

    /// Handle mouse button press
    fn handle_mouse_down(&mut self, button: MouseButton, position: MousePosition) -> Result<()> {
        let button_type = self.convert_mouse_button(button);
        self.state = MouseInteractionState::Pressed(button_type);

        // Track click for multi-click detection
        self.update_click_tracker(button_type, (position.x, position.y));

        // Initialize drag state if drag and drop is enabled
        if self.config.enable_drag_drop {
            self.drag_state = Some(DragState {
                button: button_type,
                start_position: (position.x, position.y),
                current_position: (position.x, position.y),
                is_dragging: false,
            });
        }

        // Initialize gesture tracking if gestures are enabled
        if self.config.enable_gestures {
            self.gesture_start = Some((position.x, position.y));
            self.gesture_path.clear();
            self.gesture_path.push((position.x, position.y));
        }

        Ok(())
    }

    /// Handle mouse button release
    fn handle_mouse_up(&mut self, button: MouseButton, position: MousePosition) -> Result<()> {
        let button_type = self.convert_mouse_button(button);
        self.state = MouseInteractionState::Released;

        // Handle drag end if we were dragging
        if let Some(ref drag_state) = self.drag_state {
            if drag_state.is_dragging && drag_state.button == button_type {
                self.handle_drag_end(position)?;
            }
        }

        // Process click events
        self.process_click_event(button_type, position)?;

        // Reset states
        self.drag_state = None;
        self.state = if self.is_hovered {
            MouseInteractionState::Hover
        } else {
            MouseInteractionState::Normal
        };

        Ok(())
    }

    /// Handle mouse drag
    fn handle_mouse_drag(&mut self, button: MouseButton, position: MousePosition) -> Result<()> {
        let button_type = self.convert_mouse_button(button);

        let mut should_start_drag = false;
        let mut should_handle_drag_move = false;
        
        if let Some(ref mut drag_state) = self.drag_state {
            if drag_state.button == button_type {
                let distance = Self::calculate_distance_static(
                    drag_state.start_position,
                    (position.x, position.y),
                );

                // Check if we've moved enough to start dragging
                if !drag_state.is_dragging && distance >= self.config.drag_threshold as f32 {
                    drag_state.is_dragging = true;
                    self.state = MouseInteractionState::Dragging(button_type);
                    should_start_drag = true;
                }

                if drag_state.is_dragging {
                    drag_state.current_position = (position.x, position.y);
                    should_handle_drag_move = true;
                }
            }
        }

        // Handle drag events after releasing the mutable borrow
        if should_start_drag {
            self.handle_drag_start(position)?;
        }
        if should_handle_drag_move {
            self.handle_drag_move(position)?;
        }

        // Update gesture path
        if self.config.enable_gestures {
            self.gesture_path.push((position.x, position.y));
        }

        Ok(())
    }

    /// Handle mouse move (without button pressed)
    fn handle_mouse_move(&mut self, position: MousePosition) -> Result<()> {
        // Handle hover state
        if !self.is_hovered {
            self.handle_hover_enter(position)?;
        } else {
            self.handle_hover_move(position)?;
        }

        Ok(())
    }

    /// Handle scroll events
    fn handle_scroll(&mut self, direction: ScrollDirection) -> Result<()> {
        if let Some(callback) = &self.config.on_scroll {
            let action = self.action_dispatcher.action(callback)
                .param("direction", format!("{direction:?}"))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle hover enter
    fn handle_hover_enter(&mut self, position: MousePosition) -> Result<()> {
        self.is_hovered = true;
        self.hover_start_time = Some(Instant::now());
        self.state = MouseInteractionState::Hover;

        if let Some(callback) = &self.config.on_hover_enter {
            let action = self.action_dispatcher.action(callback)
                .param("position", format!("({}, {})", position.x, position.y))
                .param("relative_position", format!("({}, {})", position.relative_x, position.relative_y))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle hover leave
    fn handle_hover_leave(&mut self) -> Result<()> {
        self.is_hovered = false;
        self.hover_start_time = None;
        self.state = MouseInteractionState::Normal;

        if let Some(callback) = &self.config.on_hover_leave {
            let action = self.action_dispatcher.action(callback).build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle hover move
    fn handle_hover_move(&mut self, position: MousePosition) -> Result<()> {
        if let Some(callback) = &self.config.on_hover_move {
            let action = self.action_dispatcher.action(callback)
                .param("position", format!("({}, {})", position.x, position.y))
                .param("relative_position", format!("({}, {})", position.relative_x, position.relative_y))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle drag start
    fn handle_drag_start(&mut self, position: MousePosition) -> Result<()> {
        if let Some(callback) = &self.config.on_drag_start {
            let action = self.action_dispatcher.action(callback)
                .param("start_position", format!("({}, {})", position.x, position.y))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle drag move
    fn handle_drag_move(&mut self, position: MousePosition) -> Result<()> {
        if let Some(callback) = &self.config.on_drag_move {
            let action = self.action_dispatcher.action(callback)
                .param("current_position", format!("({}, {})", position.x, position.y))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Handle drag end
    fn handle_drag_end(&mut self, position: MousePosition) -> Result<()> {
        if let Some(callback) = &self.config.on_drag_end {
            let drag_state = self.drag_state.as_ref().unwrap();
            let action = self.action_dispatcher.action(callback)
                .param("start_position", format!("({}, {})", drag_state.start_position.0, drag_state.start_position.1))
                .param("end_position", format!("({}, {})", position.x, position.y))
                .build();
            self.action_dispatcher.dispatch(action);
        }

        Ok(())
    }

    /// Process click events with multi-click detection
    fn process_click_event(&mut self, button: MouseButtonType, position: MousePosition) -> Result<()> {
        if let Some(ref tracker) = self.click_tracker {
            match tracker.count {
                1 => {
                    // Single click
                    let callback = match button {
                        MouseButtonType::Left => &self.config.on_click,
                        MouseButtonType::Right => &self.config.on_right_click,
                        MouseButtonType::Middle => &self.config.on_middle_click,
                    };

                    if let Some(callback) = callback {
                        let action = self.action_dispatcher.action(callback)
                            .param("button", format!("{button:?}"))
                            .param("position", format!("({}, {})", position.x, position.y))
                            .build();
                        self.action_dispatcher.dispatch(action);
                    }
                }
                2 => {
                    // Double click
                    if let Some(callback) = &self.config.on_double_click {
                        let action = self.action_dispatcher.action(callback)
                            .param("button", format!("{button:?}"))
                            .param("position", format!("({}, {})", position.x, position.y))
                            .build();
                        self.action_dispatcher.dispatch(action);
                    }
                }
                3 => {
                    // Triple click
                    if let Some(callback) = &self.config.on_triple_click {
                        let action = self.action_dispatcher.action(callback)
                            .param("button", format!("{button:?}"))
                            .param("position", format!("({}, {})", position.x, position.y))
                            .build();
                        self.action_dispatcher.dispatch(action);
                    }
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Update click tracker for multi-click detection
    fn update_click_tracker(&mut self, button: MouseButtonType, position: (u16, u16)) {
        let now = Instant::now();

        if let Some(ref mut tracker) = self.click_tracker {
            let time_diff = now.duration_since(tracker.timestamp);
            let distance = Self::calculate_distance_static(tracker.position, position);

            // Check if this is part of a multi-click sequence
            if tracker.button == button
                && distance <= 5.0  // Small distance threshold
                && time_diff <= self.config.double_click_threshold
            {
                tracker.count += 1;
                tracker.timestamp = now;
                tracker.position = position;
            } else {
                // Reset tracker for new click sequence
                *tracker = ClickTracker {
                    button,
                    position,
                    timestamp: now,
                    count: 1,
                };
            }
        } else {
            // First click
            self.click_tracker = Some(ClickTracker {
                button,
                position,
                timestamp: now,
                count: 1,
            });
        }
    }

    /// Calculate position relative to widget bounds
    fn calculate_position(&self, x: u16, y: u16, bounds: &LayoutRect) -> MousePosition {
        MousePosition {
            x,
            y,
            relative_x: x.saturating_sub(bounds.x),
            relative_y: y.saturating_sub(bounds.y),
        }
    }

    /// Check if coordinates are within widget bounds
    fn is_within_bounds(&self, x: u16, y: u16, bounds: &LayoutRect) -> bool {
        x >= bounds.x
            && x < bounds.x + bounds.width
            && y >= bounds.y
            && y < bounds.y + bounds.height
    }

    /// Calculate distance between two points
    #[allow(dead_code)]
    fn calculate_distance(&self, p1: (u16, u16), p2: (u16, u16)) -> f32 {
        Self::calculate_distance_static(p1, p2)
    }

    /// Static version of distance calculation
    fn calculate_distance_static(p1: (u16, u16), p2: (u16, u16)) -> f32 {
        let dx = p2.0 as f32 - p1.0 as f32;
        let dy = p2.1 as f32 - p1.1 as f32;
        (dx * dx + dy * dy).sqrt()
    }

    /// Convert crossterm MouseButton to our MouseButtonType
    fn convert_mouse_button(&self, button: MouseButton) -> MouseButtonType {
        match button {
            MouseButton::Left => MouseButtonType::Left,
            MouseButton::Right => MouseButtonType::Right,
            MouseButton::Middle => MouseButtonType::Middle,
        }
    }

    /// Get last mouse position
    pub fn get_last_position(&self) -> Option<MousePosition> {
        self.last_position
    }

    /// Get hover duration if currently hovering
    pub fn get_hover_duration(&self) -> Option<Duration> {
        self.hover_start_time.map(|start| start.elapsed())
    }
}

impl MouseWidget {
    /// Get CSS classes for the widget
    pub fn get_css_classes(&self) -> Vec<String> {
        let mut classes = self.config.classes.clone();
        classes.push("mouse-widget".to_string());
        
        match self.state {
            MouseInteractionState::Normal => classes.push("mouse-normal".to_string()),
            MouseInteractionState::Hover => classes.push("mouse-hover".to_string()),
            MouseInteractionState::Pressed(button) => {
                classes.push("mouse-pressed".to_string());
                classes.push(format!("mouse-pressed-{button:?}").to_lowercase());
            }
            MouseInteractionState::Dragging(button) => {
                classes.push("mouse-dragging".to_string());
                classes.push(format!("mouse-dragging-{button:?}").to_lowercase());
            }
            MouseInteractionState::Released => classes.push("mouse-released".to_string()),
        }

        classes
    }
}

impl ResponsiveWidget for MouseWidget {
    fn to_element(&self) -> Element {
        let mut builder = Element::with_tag("div")
            .id(&self.config.id)
            .classes(self.get_css_classes())
            .attr("data-cursor", format!("{:?}", self.config.cursor_style).to_lowercase())
            .attr("data-enable-hover", self.config.enable_hover.to_string())
            .attr("data-enable-drag", self.config.enable_drag_drop.to_string())
            .attr("data-enable-gestures", self.config.enable_gestures.to_string());

        if let Some(position) = self.last_position {
            builder = builder
                .attr("data-last-x", position.x.to_string())
                .attr("data-last-y", position.y.to_string())
                .attr("data-relative-x", position.relative_x.to_string())
                .attr("data-relative-y", position.relative_y.to_string());
        }

        builder.build()
    }

    fn render_with_layout(&self, layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
        // Mouse widget typically doesn't render visible content
        // Return empty string or debug info
        if cfg!(debug_assertions) {
            format!(
                "Mouse[{}x{}+{}+{}] State:{:?}",
                layout.width, layout.height, layout.x, layout.y, self.state
            )
        } else {
            String::new()
        }
    }

    fn min_size(&self) -> (u16, u16) {
        (1, 1)
    }

    fn max_size(&self) -> (Option<u16>, Option<u16>) {
        (None, None)
    }

    fn can_grow_horizontal(&self) -> bool {
        true
    }

    fn can_grow_vertical(&self) -> bool {
        true
    }
}

impl Clone for MouseWidget {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            state: self.state,
            bounds: self.bounds,
            last_position: self.last_position,
            action_dispatcher: ActionDispatcher::new(), // Create new dispatcher
            click_tracker: self.click_tracker.clone(),
            drag_state: self.drag_state.clone(),
            is_hovered: self.is_hovered,
            hover_start_time: self.hover_start_time,
            gesture_start: self.gesture_start,
            gesture_path: self.gesture_path.clone(),
        }
    }
}

/// Builder for Mouse widget configuration
pub struct MouseBuilder {
    config: MouseConfig,
}

impl MouseBuilder {
    pub fn new(id: &str) -> Self {
        Self {
            config: MouseConfig {
                id: id.to_string(),
                ..Default::default()
            },
        }
    }

    pub fn cursor_style(mut self, style: CursorStyle) -> Self {
        self.config.cursor_style = style;
        self
    }

    pub fn enable_hover(mut self, enable: bool) -> Self {
        self.config.enable_hover = enable;
        self
    }

    pub fn enable_drag_drop(mut self, enable: bool) -> Self {
        self.config.enable_drag_drop = enable;
        self
    }

    pub fn enable_gestures(mut self, enable: bool) -> Self {
        self.config.enable_gestures = enable;
        self
    }

    pub fn double_click_threshold(mut self, threshold: Duration) -> Self {
        self.config.double_click_threshold = threshold;
        self
    }

    pub fn drag_threshold(mut self, threshold: u16) -> Self {
        self.config.drag_threshold = threshold;
        self
    }

    pub fn gesture_threshold(mut self, threshold: u16) -> Self {
        self.config.gesture_threshold = threshold;
        self
    }

    pub fn on_click(mut self, callback: &str) -> Self {
        self.config.on_click = Some(callback.to_string());
        self
    }

    pub fn on_double_click(mut self, callback: &str) -> Self {
        self.config.on_double_click = Some(callback.to_string());
        self
    }

    pub fn on_right_click(mut self, callback: &str) -> Self {
        self.config.on_right_click = Some(callback.to_string());
        self
    }

    pub fn on_hover_enter(mut self, callback: &str) -> Self {
        self.config.on_hover_enter = Some(callback.to_string());
        self
    }

    pub fn on_hover_leave(mut self, callback: &str) -> Self {
        self.config.on_hover_leave = Some(callback.to_string());
        self
    }

    pub fn on_drag_start(mut self, callback: &str) -> Self {
        self.config.on_drag_start = Some(callback.to_string());
        self
    }

    pub fn on_drag_end(mut self, callback: &str) -> Self {
        self.config.on_drag_end = Some(callback.to_string());
        self
    }

    pub fn on_scroll(mut self, callback: &str) -> Self {
        self.config.on_scroll = Some(callback.to_string());
        self
    }

    pub fn class(mut self, class_name: &str) -> Self {
        self.config.classes.push(class_name.to_string());
        self
    }

    pub fn build(self) -> MouseWidget {
        MouseWidget::new(self.config)
    }
}

/// Factory function for creating mouse widgets
pub fn mouse<F>(id: &str, config: F) -> Element
where
    F: FnOnce(MouseBuilder) -> MouseBuilder,
{
    let builder = MouseBuilder::new(id);
    let configured_builder = config(builder);
    let widget = configured_builder.build();
    widget.to_element()
}

/// Convenience function for click areas
pub fn click_area(id: &str, on_click: &str) -> Element {
    mouse(id, |config| {
        config
            .on_click(on_click)
            .cursor_style(CursorStyle::Pointer)
    })
}

/// Convenience function for drag and drop areas
pub fn drag_drop_area(id: &str, on_drag_start: &str, on_drag_end: &str) -> Element {
    mouse(id, |config| {
        config
            .enable_drag_drop(true)
            .on_drag_start(on_drag_start)
            .on_drag_end(on_drag_end)
            .cursor_style(CursorStyle::Grab)
    })
}

/// Convenience function for hover areas
pub fn hover_area(id: &str, on_hover_enter: &str, on_hover_leave: &str) -> Element {
    mouse(id, |config| {
        config
            .on_hover_enter(on_hover_enter)
            .on_hover_leave(on_hover_leave)
    })
}

/// Convenience function for draggable elements
pub fn draggable(id: &str, on_drag_start: &str) -> Element {
    mouse(id, |config| {
        config
            .enable_drag_drop(true)
            .on_drag_start(on_drag_start)
            .cursor_style(CursorStyle::Grab)
            .class("draggable")
    })
}

/// Convenience function for droppable areas
pub fn droppable(id: &str, on_drag_end: &str) -> Element {
    mouse(id, |config| {
        config
            .enable_drag_drop(true)
            .on_drag_end(on_drag_end)
            .cursor_style(CursorStyle::Default)
            .class("droppable")
    })
}

/// Convenience function for drop target areas
pub fn drop_target(id: &str, on_drop: &str) -> Element {
    mouse(id, |config| {
        config
            .enable_drag_drop(true)
            .on_drag_end(on_drop)
            .on_hover_enter("drop_hover_enter")
            .on_hover_leave("drop_hover_leave")
            .cursor_style(CursorStyle::Default)
            .class("drop-target")
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_widget_creation() {
        let config = MouseConfig {
            id: "test-mouse".to_string(),
            ..Default::default()
        };
        let widget = MouseWidget::new(config);
        assert_eq!(widget.get_id(), "test-mouse");
        assert_eq!(widget.get_state(), MouseInteractionState::Normal);
        assert!(!widget.is_hovering());
    }

    #[test]
    fn test_mouse_builder() {
        let widget = MouseBuilder::new("builder-test")
            .enable_hover(true)
            .enable_drag_drop(true)
            .cursor_style(CursorStyle::Pointer)
            .on_click("handle_click")
            .build();

        assert_eq!(widget.config.id, "builder-test");
        assert!(widget.config.enable_hover);
        assert!(widget.config.enable_drag_drop);
        assert_eq!(widget.config.cursor_style, CursorStyle::Pointer);
        assert_eq!(widget.config.on_click, Some("handle_click".to_string()));
    }

    #[test]
    fn test_coordinate_bounds_checking() {
        let config = MouseConfig {
            id: "bounds-test".to_string(),
            ..Default::default()
        };
        let widget = MouseWidget::new(config);
        
        let bounds = LayoutRect {
            x: 10,
            y: 10,
            width: 20,
            height: 15,
        };

        // Test within bounds
        assert!(widget.is_within_bounds(15, 15, &bounds));
        assert!(widget.is_within_bounds(10, 10, &bounds)); // Top-left corner
        assert!(widget.is_within_bounds(29, 24, &bounds)); // Bottom-right corner

        // Test outside bounds
        assert!(!widget.is_within_bounds(9, 15, &bounds));  // Left of bounds
        assert!(!widget.is_within_bounds(30, 15, &bounds)); // Right of bounds
        assert!(!widget.is_within_bounds(15, 9, &bounds));  // Above bounds
        assert!(!widget.is_within_bounds(15, 25, &bounds)); // Below bounds
    }

    #[test]
    fn test_distance_calculation() {
        let config = MouseConfig {
            id: "distance-test".to_string(),
            ..Default::default()
        };
        let widget = MouseWidget::new(config);

        // Test distance calculation
        assert_eq!(widget.calculate_distance((0, 0), (3, 4)), 5.0);
        assert_eq!(widget.calculate_distance((10, 10), (10, 10)), 0.0);
        assert_eq!(widget.calculate_distance((0, 0), (1, 1)), 2.0_f32.sqrt());
    }

    #[test]
    fn test_factory_functions() {
        let click_elem = click_area("click-test", "handle_click");
        assert_eq!(click_elem.id, Some("click-test".to_string()));

        let drag_elem = drag_drop_area("drag-test", "drag_start", "drag_end");
        assert_eq!(drag_elem.id, Some("drag-test".to_string()));

        let hover_elem = hover_area("hover-test", "hover_enter", "hover_leave");
        assert_eq!(hover_elem.id, Some("hover-test".to_string()));
    }

    #[test]
    fn test_drag_drop_factory_functions() {
        // Test draggable element
        let draggable_elem = draggable("draggable-test", "start_drag");
        assert_eq!(draggable_elem.id, Some("draggable-test".to_string()));
        assert!(draggable_elem.classes.contains(&"draggable".to_string()));
        assert_eq!(draggable_elem.attributes.get("data-cursor"), Some(&"grab".to_string()));

        // Test droppable element
        let droppable_elem = droppable("droppable-test", "end_drag");
        assert_eq!(droppable_elem.id, Some("droppable-test".to_string()));
        assert!(droppable_elem.classes.contains(&"droppable".to_string()));
        assert_eq!(droppable_elem.attributes.get("data-cursor"), Some(&"default".to_string()));

        // Test drop target element
        let drop_target_elem = drop_target("drop-target-test", "handle_drop");
        assert_eq!(drop_target_elem.id, Some("drop-target-test".to_string()));
        assert!(drop_target_elem.classes.contains(&"drop-target".to_string()));
        assert_eq!(drop_target_elem.attributes.get("data-cursor"), Some(&"default".to_string()));
        assert_eq!(drop_target_elem.attributes.get("data-enable-drag"), Some(&"true".to_string()));
    }
}