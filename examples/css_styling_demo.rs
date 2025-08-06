//! CSS Styling Demo
//!
//! Demonstrates CSS styles being applied to terminal output with ANSI escapes

use reactive_tui::prelude::*;

/// A styled component that demonstrates CSS styling
struct StyledComponent;

impl Component for StyledComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("main-container")
            .child(
                Element::with_tag("h1")
                    .class("title")
                    .content("CSS Styling Demo")
                    .build()
            )
            .child(
                Element::with_tag("div")
                    .class("content-box")
                    .child(
                        Element::with_tag("p")
                            .class("text-primary")
                            .content("This text should be styled with CSS!")
                            .build()
                    )
                    .child(
                        Element::with_tag("p")
                            .class("text-secondary")
                            .content("This text has different styling.")
                            .build()
                    )
                    .child(
                        Element::with_tag("div")
                            .class("highlight-box")
                            .content("This box should have a background color and border.")
                            .build()
                    )
                    .build()
            )
            .child(
                Element::with_tag("div")
                    .class("footer")
                    .content("Press ESC to quit")
                    .build()
            )
            .build()
    }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
    println!("Starting CSS Styling Demo...");

    // Create CSS stylesheet
    let css = r#"
        .main-container {
            padding: 2;
            margin: 1;
        }

        .title {
            color: #00ff00;
            font-weight: bold;
            text-decoration: underline;
        }

        .content-box {
            padding: 1;
            border-width: 1;
            border-color: #ffffff;
        }

        .text-primary {
            color: #00ffff;
            font-weight: bold;
        }

        .text-secondary {
            color: #ffff00;
            font-style: italic;
        }

        .highlight-box {
            background-color: #ff0000;
            color: #ffffff;
            padding: 1;
            border-width: 1;
            border-color: #ffffff;
        }

        .footer {
            color: #888888;
            margin-top: 2;
        }
    "#;

    // Build the TUI app
    let mut app = TuiApp::builder()
        .component(StyledComponent)
        .with_title("CSS Styling Demo")
        .frame_rate(30)
        .build()?;

    // Load CSS from string
    app.load_css(css.to_string())?;

    // Set up key bindings
    app.bind_key_to_action(
        reactive_tui::events::KeyCombination::new(reactive_tui::compat::KeyCode::Esc),
        "quit"
    ).await;

    // Register action handlers
    app.register_action("quit", |_action| {
        println!("Quitting CSS demo...");
        reactive_tui::events::ActionResult::Handled
    });

    println!("Demo running! You should see styled text with colors, borders, and backgrounds.");
    println!("Press ESC to quit");

    // Run the app
    app.run().await?;

    println!("CSS Demo completed!");
    Ok(())
}
