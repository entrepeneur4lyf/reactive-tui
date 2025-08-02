/*!
 * Multi-screen Demo
 * 
 * Demonstrates the multi-screen and workspace features including:
 * - Multiple screens with navigation
 * - Screen transitions
 * - Workspaces and tabs
 * - Navigation history
 * - Keyboard shortcuts
 */

use tui_core::prelude::*;
use tui_core::screens::*;
use tui_core::components::{div, text, Element, button};
use tui_core::events::ActionResult;
use crossterm::event::KeyEvent;
use std::sync::{Arc, RwLock};
use std::collections::HashMap;
use async_trait::async_trait;

/// Home screen
struct HomeScreen;

#[async_trait]
impl Screen for HomeScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "home".to_string(),
            title: "Home".to_string(),
            ..Default::default()
        }
    }
}

impl Component for HomeScreen {
    fn render(&self) -> Element {
        div()
            .class("screen")
            .class("home-screen")
            .child(
                div()
                    .class("header")
                    .child(text("🏠 Home").class("title").build())
                    .build()
            )
            .child(
                div()
                    .class("content")
                    .class("p-4")
                    .child(text("Welcome to the Multi-Screen Demo!").build())
                    .child(
                        div()
                            .class("mt-4")
                            .child(text("Navigate with:").build())
                            .child(text("  [S] Settings").build())
                            .child(text("  [P] Profile").build())
                            .child(text("  [D] Dashboard").build())
                            .child(text("  [Tab] Next Screen").build())
                            .child(text("  [Esc] Previous Screen").build())
                            .child(text("  [Q] Quit").build())
                            .build()
                    )
                    .build()
            )
            .build()
    }
}

/// Settings screen with form
struct SettingsScreen {
    theme: Arc<RwLock<String>>,
    auto_save: Arc<RwLock<bool>>,
}

impl SettingsScreen {
    fn new() -> Self {
        Self {
            theme: Arc::new(RwLock::new("dark".to_string())),
            auto_save: Arc::new(RwLock::new(true)),
        }
    }
}

#[async_trait]
impl Screen for SettingsScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "settings".to_string(),
            title: "Settings".to_string(),
            preserve_state: true,
            ..Default::default()
        }
    }
    
    fn handle_key(&mut self, key: KeyEvent, _state: &mut ScreenState) -> ActionResult {
        match key.code {
            crossterm::event::KeyCode::Char('t') => {
                let mut theme = self.theme.write().unwrap();
                *theme = if *theme == "dark" { "light".to_string() } else { "dark".to_string() };
                ActionResult::Handled
            }
            crossterm::event::KeyCode::Char('a') => {
                let mut auto_save = self.auto_save.write().unwrap();
                *auto_save = !*auto_save;
                ActionResult::Handled
            }
            _ => ActionResult::NotHandled,
        }
    }
}

impl Component for SettingsScreen {
    fn render(&self) -> Element {
        let theme = self.theme.read().unwrap();
        let auto_save = self.auto_save.read().unwrap();
        
        div()
            .class("screen")
            .class("settings-screen")
            .child(
                div()
                    .class("header")
                    .child(text("⚙️ Settings").class("title").build())
                    .build()
            )
            .child(
                div()
                    .class("content")
                    .class("p-4")
                    .child(
                        div()
                            .class("setting-item")
                            .child(text("Theme:").class("label").build())
                            .child(text(&format!(" {} [T to toggle]", theme)).build())
                            .build()
                    )
                    .child(
                        div()
                            .class("setting-item")
                            .child(text("Auto-save:").class("label").build())
                            .child(text(&format!(" {} [A to toggle]", if *auto_save { "On" } else { "Off" })).build())
                            .build()
                    )
                    .build()
            )
            .build()
    }
}

/// Profile screen
struct ProfileScreen {
    user_name: String,
    email: String,
}

impl ProfileScreen {
    fn new() -> Self {
        Self {
            user_name: "Demo User".to_string(),
            email: "demo@example.com".to_string(),
        }
    }
}

#[async_trait]
impl Screen for ProfileScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "profile".to_string(),
            title: "Profile".to_string(),
            ..Default::default()
        }
    }
}

impl Component for ProfileScreen {
    fn render(&self) -> Element {
        div()
            .class("screen")
            .class("profile-screen")
            .child(
                div()
                    .class("header")
                    .child(text("👤 Profile").class("title").build())
                    .build()
            )
            .child(
                div()
                    .class("content")
                    .class("p-4")
                    .child(
                        div()
                            .class("profile-info")
                            .child(text(&format!("Name: {}", self.user_name)).build())
                            .child(text(&format!("Email: {}", self.email)).build())
                            .child(text("Role: Administrator").build())
                            .child(text("Last Login: Today").build())
                            .build()
                    )
                    .child(
                        div()
                            .class("actions")
                            .class("mt-4")
                            .child(
                                button()
                                    .child(text("Edit Profile").build())
                                    .build()
                            )
                            .child(
                                button()
                                    .child(text("Change Password").build())
                                    .class("secondary")
                                    .build()
                            )
                            .build()
                    )
                    .build()
            )
            .build()
    }
}

/// Dashboard screen with metrics
struct DashboardScreen {
    metrics: Arc<RwLock<DashboardMetrics>>,
}

#[derive(Clone)]
struct DashboardMetrics {
    cpu: f32,
    memory: f32,
    disk: f32,
    network: String,
}

impl DashboardScreen {
    fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(DashboardMetrics {
                cpu: 23.5,
                memory: 45.2,
                disk: 78.9,
                network: "Connected".to_string(),
            })),
        }
    }
}

#[async_trait]
impl Screen for DashboardScreen {
    fn config(&self) -> ScreenConfig {
        ScreenConfig {
            id: "dashboard".to_string(),
            title: "Dashboard".to_string(),
            ..Default::default()
        }
    }
}

impl Component for DashboardScreen {
    fn render(&self) -> Element {
        let metrics = self.metrics.read().unwrap();
        
        div()
            .class("screen")
            .class("dashboard-screen")
            .child(
                div()
                    .class("header")
                    .child(text("📊 Dashboard").class("title").build())
                    .build()
            )
            .child(
                div()
                    .class("content")
                    .class("p-4")
                    .child(
                        div()
                            .class("metrics-grid")
                            .child(
                                div()
                                    .class("metric-card")
                                    .child(text("CPU Usage").class("metric-label").build())
                                    .child(text(&format!("{}%", metrics.cpu)).class("metric-value").build())
                                    .build()
                            )
                            .child(
                                div()
                                    .class("metric-card")
                                    .child(text("Memory").class("metric-label").build())
                                    .child(text(&format!("{}%", metrics.memory)).class("metric-value").build())
                                    .build()
                            )
                            .child(
                                div()
                                    .class("metric-card")
                                    .child(text("Disk Usage").class("metric-label").build())
                                    .child(text(&format!("{}%", metrics.disk)).class("metric-value").build())
                                    .build()
                            )
                            .child(
                                div()
                                    .class("metric-card")
                                    .child(text("Network").class("metric-label").build())
                                    .child(text(&metrics.network).class("metric-value").build())
                                    .build()
                            )
                            .build()
                    )
                    .child(
                        div()
                            .class("actions")
                            .class("mt-4")
                            .child(button().child(text("Restart Service").build()).build())
                            .child(button().child(text("Clear Cache").build()).build())
                            .child(button().child(text("View Logs").build()).build())
                            .build()
                    )
                    .build()
            )
            .build()
    }
}

/// Multi-screen application manager
struct MultiScreenApp {
    screen_manager: Arc<ScreenManager>,
    #[allow(dead_code)]
    current_screen: Arc<RwLock<String>>,
}

impl MultiScreenApp {
    async fn new() -> Result<Self> {
        let config = ScreenManagerConfig {
            default_screen: "home".to_string(),
            enable_keyboard_nav: true,
            enable_history: true,
            max_history_size: 10,
            shortcuts: HashMap::new(),
        };
        
        let screen_manager = Arc::new(ScreenManager::new(config));
        
        // Register screens
        screen_manager.register(Box::new(HomeScreen)).await?;
        screen_manager.register(Box::new(SettingsScreen::new())).await?;
        screen_manager.register(Box::new(ProfileScreen::new())).await?;
        screen_manager.register(Box::new(DashboardScreen::new())).await?;
        
        // Create workspaces
        screen_manager.create_workspace("main", "Main Workspace")?;
        screen_manager.create_workspace("admin", "Admin Workspace")?;
        
        Ok(Self {
            screen_manager,
            current_screen: Arc::new(RwLock::new("home".to_string())),
        })
    }
    
    fn render(&self) -> Element {
        div()
            .class("multi-screen-app")
            .child(
                // Top bar
                div()
                    .class("bar")
                    .class("bar-top")
                    .children(vec![
                        div().child(text("File").build()).build(),
                        div().child(text("Edit").build()).build(),
                        div().child(text("View").build()).build(),
                        div().child(text("Tools").build()).build(),
                        div().child(text("Help").build()).build(),
                    ])
                    .build()
            )
            .child(
                // Main content (screen manager renders current screen)
                div()
                    .class("main-content")
                    .child(self.screen_manager.render())
                    .build()
            )
            .child(
                // Bottom bar
                div()
                    .class("bar")
                    .class("bar-bottom")
                    .children(vec![
                        div().child(text("[Tab] Next").build()).build(),
                        div().child(text("[Esc] Back").build()).build(),
                        div().child(text("[Q] Quit").build()).build(),
                    ])
                    .build()
            )
            .build()
    }
}

impl Component for MultiScreenApp {
    fn render(&self) -> Element {
        self.render()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("🚀 Multi-Screen Demo");
    println!("====================\n");
    
    let multi_app = MultiScreenApp::new().await?;
    
    let app = TuiAppBuilder::new()
        .component(multi_app)
        .with_title("Multi-Screen Demo")
        .build()?;
    
    app.run().await?;
    
    println!("\nThanks for trying the multi-screen demo!");
    Ok(())
}