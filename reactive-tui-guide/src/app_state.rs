//! Application state management for the Interactive Widget Guide

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Main application state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    pub current_screen: Screen,
    pub selected_widget: Option<WidgetType>,
    pub guide_mode: GuideMode,
    pub navigation_history: Vec<Screen>,
    pub framerate: f64,
    pub widget_parameters: HashMap<WidgetType, serde_json::Value>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            current_screen: Screen::Splash,
            selected_widget: None,
            guide_mode: GuideMode::Documentation,
            navigation_history: Vec::new(),
            framerate: 60.0,
            widget_parameters: HashMap::new(),
        }
    }
}

impl AppState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        self.navigation_history.push(self.current_screen.clone());
        self.current_screen = screen;
    }

    pub fn go_back(&mut self) -> bool {
        if let Some(previous_screen) = self.navigation_history.pop() {
            self.current_screen = previous_screen;
            true
        } else {
            false
        }
    }

    pub fn toggle_guide_mode(&mut self) {
        self.guide_mode = match self.guide_mode {
            GuideMode::Documentation => GuideMode::Interactive,
            GuideMode::Interactive => GuideMode::Documentation,
        };
    }

    pub fn set_widget_parameters(&mut self, widget_type: WidgetType, params: serde_json::Value) {
        self.widget_parameters.insert(widget_type, params);
    }

    pub fn get_widget_parameters(&self, widget_type: WidgetType) -> Option<&serde_json::Value> {
        self.widget_parameters.get(&widget_type)
    }

    pub fn update_framerate(&mut self, fps: f64) {
        self.framerate = fps;
    }
}

/// Application screens
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Screen {
    Splash,
    Home,
    WidgetDemo { widget_type: WidgetType, mode: GuideMode },
}

/// Guide modes for widget demonstration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GuideMode {
    Documentation,
    Interactive,
}

/// Available widget types in the framework
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum WidgetType {
    // Layout & Navigation
    Accordion,
    Bar,
    Grid,
    Modal,
    Panel,
    Tabs,
    
    // Form Controls
    Autocomplete,
    Button,
    Checkbox,
    Input,
    Radio,
    Select,
    Slider,
    Switch,
    
    // Data Display
    DataTable,
    Progress,
    ScrollableList,
    Spinner,
    Tree,
    Viewport,
    
    // Content & Feedback
    Animation,
    FormValidation,
    Image,
    Link,
    Menu,
    Mouse,
    Overlay,
    RichText,
    Textarea,
    Toast,
}

impl WidgetType {
    /// Get all available widget types
    pub fn all() -> Vec<WidgetType> {
        vec![
            // Layout & Navigation
            WidgetType::Accordion,
            WidgetType::Bar,
            WidgetType::Grid,
            WidgetType::Modal,
            WidgetType::Panel,
            WidgetType::Tabs,
            
            // Form Controls
            WidgetType::Autocomplete,
            WidgetType::Button,
            WidgetType::Checkbox,
            WidgetType::Input,
            WidgetType::Radio,
            WidgetType::Select,
            WidgetType::Slider,
            WidgetType::Switch,
            
            // Data Display
            WidgetType::DataTable,
            WidgetType::Progress,
            WidgetType::ScrollableList,
            WidgetType::Spinner,
            WidgetType::Tree,
            WidgetType::Viewport,
            
            // Content & Feedback
            WidgetType::Animation,
            WidgetType::FormValidation,
            WidgetType::Image,
            WidgetType::Link,
            WidgetType::Menu,
            WidgetType::Mouse,
            WidgetType::Overlay,
            WidgetType::RichText,
            WidgetType::Textarea,
            WidgetType::Toast,
        ]
    }

    /// Get the display name for the widget
    pub fn display_name(&self) -> &'static str {
        match self {
            WidgetType::Accordion => "Accordion",
            WidgetType::Bar => "Bar",
            WidgetType::Grid => "Grid",
            WidgetType::Modal => "Modal",
            WidgetType::Panel => "Panel",
            WidgetType::Tabs => "Tabs",
            WidgetType::Autocomplete => "Autocomplete",
            WidgetType::Button => "Button",
            WidgetType::Checkbox => "Checkbox",
            WidgetType::Input => "Input",
            WidgetType::Radio => "Radio",
            WidgetType::Select => "Select",
            WidgetType::Slider => "Slider",
            WidgetType::Switch => "Switch",
            WidgetType::DataTable => "DataTable",
            WidgetType::Progress => "Progress",
            WidgetType::ScrollableList => "ScrollableList",
            WidgetType::Spinner => "Spinner",
            WidgetType::Tree => "Tree",
            WidgetType::Viewport => "Viewport",
            WidgetType::Animation => "Animation",
            WidgetType::FormValidation => "FormValidation",
            WidgetType::Image => "Image",
            WidgetType::Link => "Link",
            WidgetType::Menu => "Menu",
            WidgetType::Mouse => "Mouse",
            WidgetType::Overlay => "Overlay",
            WidgetType::RichText => "RichText",
            WidgetType::Textarea => "Textarea",
            WidgetType::Toast => "Toast",
        }
    }

    /// Get the category for the widget
    pub fn category(&self) -> WidgetCategory {
        match self {
            WidgetType::Accordion | WidgetType::Bar | WidgetType::Grid | 
            WidgetType::Modal | WidgetType::Panel | WidgetType::Tabs => WidgetCategory::Layout,
            
            WidgetType::Autocomplete | WidgetType::Button | WidgetType::Checkbox | 
            WidgetType::Input | WidgetType::Radio | WidgetType::Select | 
            WidgetType::Slider | WidgetType::Switch => WidgetCategory::FormControls,
            
            WidgetType::DataTable | WidgetType::Progress | WidgetType::ScrollableList | 
            WidgetType::Spinner | WidgetType::Tree | WidgetType::Viewport => WidgetCategory::DataDisplay,
            
            WidgetType::Animation | WidgetType::FormValidation | WidgetType::Image | 
            WidgetType::Link | WidgetType::Menu | WidgetType::Mouse | WidgetType::Overlay | 
            WidgetType::RichText | WidgetType::Textarea | WidgetType::Toast => WidgetCategory::ContentFeedback,
        }
    }
}

/// Widget categories for organization
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum WidgetCategory {
    Layout,
    FormControls,
    DataDisplay,
    ContentFeedback,
}

impl WidgetCategory {
    pub fn display_name(&self) -> &'static str {
        match self {
            WidgetCategory::Layout => "Layout & Navigation",
            WidgetCategory::FormControls => "Form Controls",
            WidgetCategory::DataDisplay => "Data Display",
            WidgetCategory::ContentFeedback => "Content & Feedback",
        }
    }

    pub fn all() -> Vec<WidgetCategory> {
        vec![
            WidgetCategory::Layout,
            WidgetCategory::FormControls,
            WidgetCategory::DataDisplay,
            WidgetCategory::ContentFeedback,
        ]
    }
}
