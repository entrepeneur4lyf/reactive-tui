/*!
 * Screen trait and implementations
 */

use super::*;
use crate::{
    components::{Component, Element, div, text},
    events::{Event, ActionResult},
};
use crossterm::event::KeyEvent;
use async_trait::async_trait;

/// Screen trait that all screens must implement
#[async_trait]
pub trait Screen: Send + Sync {
    /// Get screen configuration
    fn config(&self) -> ScreenConfig;
    
    /// Handle screen lifecycle events
    async fn handle_event(&mut self, event: ScreenEvent, state: &mut ScreenState) -> Result<()> {
        match event {
            ScreenEvent::Mount => self.on_mount(state).await,
            ScreenEvent::Unmount => self.on_unmount(state).await,
            ScreenEvent::Show => self.on_show(state).await,
            ScreenEvent::Hide => self.on_hide(state).await,
            ScreenEvent::Focus => self.on_focus(state).await,
            ScreenEvent::Blur => self.on_blur(state).await,
            ScreenEvent::Custom(event_type, data) => self.on_custom_event(&event_type, data, state).await,
        }
    }
    
    /// Called when screen is mounted
    async fn on_mount(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Called when screen is unmounted
    async fn on_unmount(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Called when screen becomes visible
    async fn on_show(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Called when screen becomes hidden
    async fn on_hide(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Called when screen receives focus
    async fn on_focus(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Called when screen loses focus
    async fn on_blur(&mut self, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Handle custom events
    async fn on_custom_event(&mut self, _event_type: &str, _data: serde_json::Value, _state: &mut ScreenState) -> Result<()> {
        Ok(())
    }
    
    /// Handle keyboard input
    fn handle_key(&mut self, _key: KeyEvent, _state: &mut ScreenState) -> ActionResult {
        ActionResult::NotHandled
    }
    
    /// Handle generic events
    fn handle_input(&mut self, event: Event, state: &mut ScreenState) -> ActionResult {
        match event {
            Event::Key(key) => self.handle_key(key, state),
            _ => ActionResult::NotHandled,
        }
    }
    
    /// Check if screen can be deactivated (useful for forms with unsaved changes)
    fn can_deactivate(&self, _state: &ScreenState) -> bool {
        true
    }
    
    /// Get screen title (can be dynamic based on state)
    fn get_title(&self, _state: &ScreenState) -> String {
        self.config().title
    }
}

/// Screens must implement Component trait directly to avoid conflicts
/// 
/// A simple screen implementation for quick prototyping
pub struct SimpleScreen {
    config: ScreenConfig,
    render_fn: Box<dyn Fn(&ScreenState) -> Element + Send + Sync>,
}

impl SimpleScreen {
    /// Create a new simple screen
    pub fn new(id: &str, title: &str, render_fn: impl Fn(&ScreenState) -> Element + Send + Sync + 'static) -> Self {
        Self {
            config: ScreenConfig {
                id: id.to_string(),
                title: title.to_string(),
                ..Default::default()
            },
            render_fn: Box::new(render_fn),
        }
    }
}

#[async_trait]
impl Screen for SimpleScreen {
    fn config(&self) -> ScreenConfig {
        self.config.clone()
    }
}

impl Component for SimpleScreen {
    fn render(&self) -> Element {
        let state = ScreenState::new();
        (self.render_fn)(&state)
    }
}

/// Loading screen shown during transitions
pub struct LoadingScreen {
    message: String,
}

impl LoadingScreen {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.to_string(),
        }
    }
}

#[async_trait]
impl Screen for LoadingScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "loading".to_string(),
            title: "Loading".to_string(),
            preserve_state: false,
            ..Default::default()
        }
    }
}

impl Component for LoadingScreen {
    fn render(&self) -> Element {
        div()
            .class("loading-screen")
            .class("flex")
            .class("items-center")
            .class("justify-center")
            .class("h-full")
            .child(
                div()
                    .class("text-center")
                    .child(text(&self.message).build())
                    .build()
            )
            .build()
    }
}

/// Error screen shown when navigation fails
pub struct ErrorScreen {
    error: String,
}

impl ErrorScreen {
    pub fn new(error: &str) -> Self {
        Self {
            error: error.to_string(),
        }
    }
}

#[async_trait]
impl Screen for ErrorScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "error".to_string(),
            title: "Error".to_string(),
            preserve_state: false,
            ..Default::default()
        }
    }
}

impl Component for ErrorScreen {
    fn render(&self) -> Element {
        div()
            .class("error-screen")
            .class("flex")
            .class("items-center")
            .class("justify-center")
            .class("h-full")
            .child(
                div()
                    .class("text-center")
                    .class("text-red")
                    .child(text("⚠️ Error").build())
                    .child(text(&self.error).build())
                    .build()
            )
            .build()
    }
}