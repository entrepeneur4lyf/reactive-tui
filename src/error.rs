//! Error types for the TUI framework

use thiserror::Error;

pub type Result<T> = std::result::Result<T, TuiError>;

#[derive(Error, Debug)]
pub enum TuiError {
    #[error("CSS parsing error: {0}")]
    CssParseError(String),

    #[error("Layout error: {0}")]
    LayoutError(String),

    #[error("Rendering error: {0}")]
    RenderError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Component error: {0}")]
    ComponentError(String),

    #[error("Animation error: {0}")]
    AnimationError(String),

    #[error("Event handling error: {0}")]
    EventError(String),

    #[error("Driver error: {0}")]
    DriverError(String),
    
    #[error("Plugin error: {0}")]
    PluginError(String),
}

impl TuiError {
    pub fn css_parse<S: Into<String>>(msg: S) -> Self {
        Self::CssParseError(msg.into())
    }

    pub fn layout<S: Into<String>>(msg: S) -> Self {
        Self::LayoutError(msg.into())
    }

    pub fn render<S: Into<String>>(msg: S) -> Self {
        Self::RenderError(msg.into())
    }

    pub fn component<S: Into<String>>(msg: S) -> Self {
        Self::ComponentError(msg.into())
    }

    pub fn driver<S: Into<String>>(msg: S) -> Self {
        Self::DriverError(msg.into())
    }
    
    pub fn plugin<S: Into<String>>(msg: S) -> Self {
        Self::PluginError(msg.into())
    }
}

// Allow conversion from widget factory errors
impl From<crate::widgets::factory::WidgetFactoryError> for TuiError {
    fn from(err: crate::widgets::factory::WidgetFactoryError) -> Self {
        TuiError::ComponentError(err.to_string())
    }
}
