//! Screen management system for the Interactive Widget Guide

pub mod splash;
pub mod home;
pub mod widget_demo;

pub use splash::SplashScreen;
pub use home::HomeScreen;
pub use widget_demo::WidgetDemoScreen;

use crate::app_state::{AppState, Screen};
use reactive_tui::prelude::*;

/// Screen manager for handling navigation between different screens
pub struct ScreenManager {
    current_component: Box<dyn Component>,
}

impl ScreenManager {
    pub fn new() -> Self {
        Self {
            current_component: Box::new(SplashScreen::new()),
        }
    }

    /// Update the current screen based on app state
    pub fn update_screen(&mut self, state: &AppState) {
        match &state.current_screen {
            Screen::Splash => {
                self.current_component = Box::new(SplashScreen::new());
            }
            Screen::Home => {
                self.current_component = Box::new(HomeScreen::new());
            }
            Screen::WidgetDemo { widget_type, mode } => {
                self.current_component = Box::new(WidgetDemoScreen::new(*widget_type, mode.clone()));
            }
        }
    }

    /// Get the current component for rendering
    pub fn current_component(&self) -> &dyn Component {
        self.current_component.as_ref()
    }
}
