use anyhow::Result;
use reactive_tui::prelude::*;
use reactive_tui::events::KeyCombination;
use reactive_tui::compat::KeyCode;
use reactive_tui::components::{div, text, section};
use reactive_tui::widgets::image::logo;
use crossterm::execute;

#[tokio::main]
async fn main() -> Result<()> {
    let splash_screen = SplashScreen::new();

    let mut app = TuiApp::builder()
        .component(splash_screen)
        .with_title("Reactive TUI - Interactive Widget Guide")
        .build()?;

    // Register the quit action with proper terminal cleanup
    app.register_action("quit", |_action| {
        // Restore terminal state before exit
        let _ = crossterm::terminal::disable_raw_mode();
        let _ = execute!(
            std::io::stdout(),
            crossterm::terminal::LeaveAlternateScreen,
            crossterm::cursor::Show
        );
        std::process::exit(0);
    });

    // Register key bindings
    app.bind_key_to_action(KeyCombination::char('q'), "quit").await;
    app.bind_key_to_action(KeyCombination::new(KeyCode::Enter), "quit").await;

    // Run app with cleanup handling
    let result = app.run().await;
    
    // Always restore terminal state on exit
    let _ = crossterm::terminal::disable_raw_mode();
    let _ = execute!(
        std::io::stdout(),
        crossterm::terminal::LeaveAlternateScreen,
        crossterm::cursor::Show
    );
    
    result.map_err(|e| anyhow::anyhow!("TUI error: {}", e))
}

#[derive(Debug, Clone)]
pub struct SplashScreen;

impl SplashScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Component for SplashScreen {
    fn render(&self) -> Element {
        div()
            .class("flex")
            .class("flex-col")
            .class("h-full")
            .class("items-center")
            .class("justify-center")
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/6")
                    .child(logo("reactive-tui-logo", "assets/reactive-tui-logo.png"))
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("flex-col")
                    .class("items-center")
                    .class("h-1/6")
                    .child_builder(
                        text("REACTIVE TUI")
                            .class("text-4xl")
                            .class("font-bold")
                    )
                    .child_builder(
                        text("Interactive Widget Guide")
                            .class("text-xl")
                    )
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/12")
                    .child_builder(
                        text("A revolutionary CSS-styled terminal UI framework")
                    )
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("flex-col")
                    .class("items-center")
                    .class("h-1/4")
                    .class("justify-center")
                    .child_builder(text("• CSS-Styled Terminal Interfaces"))
                    .child_builder(text("• 25+ Production-Ready Widgets"))
                    .child_builder(text("• High Performance Rust + TypeScript"))
                    .child_builder(text("• Full TypeScript SDK Integration"))
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/12")
                    .child_builder(
                        text("Press ENTER to continue or Q to quit")
                            .class("text-lg")
                            .class("font-bold")
                    )
            )
            .build()
    }
}
