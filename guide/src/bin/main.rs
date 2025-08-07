use anyhow::Result;
use reactive_tui::prelude::*;

#[derive(Debug, Clone)]
struct SimpleGuide;

impl Component for SimpleGuide {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("guide-app")
            .content("🚀 Reactive TUI Interactive Widget Guide\n\nWelcome to the comprehensive widget demonstration!\n\nThis guide showcases all 29+ widgets available in the Reactive TUI framework.\n\n[Coming Soon: Full interactive experience]")
            .build()
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging for development
    #[cfg(debug_assertions)]
    env_logger::init();

    // Create and run a simple TUI app
    let app = TuiApp::builder()
        .component(SimpleGuide)
        .with_title("Reactive TUI Interactive Widget Guide")
        .frame_rate(60)
        .build()?;
    
    app.run().await?;

    Ok(())
}
