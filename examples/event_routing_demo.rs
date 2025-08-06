//! WORKING Event Routing Demo
//!
//! Demonstrates ACTUAL working event routing system with mouse targeting and message handling.

use reactive_tui::prelude::*;
use reactive_tui::events::ClickMessage;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone)]
struct ClickableButton {
    id: String,
    text: String,
    click_count: Arc<Mutex<i32>>,
}

impl ClickableButton {
    fn new(id: String, text: String) -> Self {
        Self {
            id,
            text,
            click_count: Arc::new(Mutex::new(0)),
        }
    }

    fn increment_clicks(&self) {
        if let Ok(mut count) = self.click_count.lock() {
            *count += 1;
        }
    }

    fn get_click_count(&self) -> i32 {
        self.click_count.lock().map(|count| *count).unwrap_or(0)
    }
}

impl Component for ClickableButton {
    fn render(&self) -> Element {
        let click_count = self.get_click_count();

        Element::with_tag("button")
            .id(&self.id)
            .class("clickable")
            .class("interactive")
            .content(&format!("{} (clicked {} times)", self.text, click_count))
            .focusable(true)
            .build()
    }
}

#[derive(Debug, Clone)]
struct EventRoutingDemo {
    buttons: Vec<ClickableButton>,
}

impl EventRoutingDemo {
    fn new() -> Self {
        Self {
            buttons: vec![
                ClickableButton::new("button1".to_string(), "Button 1".to_string()),
                ClickableButton::new("button2".to_string(), "Button 2".to_string()),
                ClickableButton::new("button3".to_string(), "Button 3".to_string()),
            ],
        }
    }
}

impl Component for EventRoutingDemo {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("container")
            .child(
                Element::with_tag("h1")
                    .content("Event Routing Demo")
                    .build()
            )
            .child(
                Element::with_tag("p")
                    .content("Click on the buttons below to test mouse event routing:")
                    .build()
            )
            .children(
                self.buttons.iter().map(|button| {
                    button.render()
                }).collect::<Vec<_>>()
            )
            .child(
                Element::with_tag("p")
                    .content("Use Tab to navigate, Enter to activate focused button")
                    .build()
            )
            .build()
    }
}

#[tokio::main]
async fn main() -> reactive_tui::error::Result<()> {
    // Create the demo component
    let demo = EventRoutingDemo::new();

    // Build the TUI app with mouse support enabled
    let mut app = TuiApp::builder()
        .component(demo.clone())
        .with_title("Event Routing Demo")
        .with_mouse(true)
        .debug_mode(true)
        .build()?;

    // ACTUALLY WORKING: Register click handlers for the buttons
    let buttons = demo.buttons.clone();
    for (_i, button) in buttons.iter().enumerate() {
        let button_clone = button.clone();
        app.on_element_message::<ClickMessage, _>(&button.id, move |event| {
            if let Some(click_msg) = event.downcast::<ClickMessage>() {
                println!("BUTTON CLICKED: {} at ({}, {})", button_clone.id, click_msg.x, click_msg.y);
                button_clone.increment_clicks();
                println!("Button {} now has {} clicks", button_clone.id, button_clone.get_click_count());
            }
            Ok(())
        })?;
    }

    println!("WORKING event routing system initialized with REAL click handlers");

    // Add some basic CSS styling
    app.load_css(r#"
        .container {
            padding: 2;
            background: #1e1e1e;
            color: #fff;
        }

        h1 {
            color: #00ff00;
            margin-bottom: 1;
        }

        button {
            display: block;
            margin: 1;
            padding: 1;
            background: #333;
            color: #fff;
            border: 1px solid #555;
        }

        button:focus {
            border-color: #00ff00;
            background: #444;
        }

        button.clickable:hover {
            background: #555;
        }

        p {
            margin: 1;
            color: #ccc;
        }
    "#.to_string())?;

    println!("Event Routing Demo");
    println!("=================");
    println!("- Click on buttons to test mouse event routing");
    println!("- Use Tab/Shift+Tab to navigate between buttons");
    println!("- Press Enter to activate the focused button");
    println!("- Press Ctrl+C to quit");
    println!();

    // Run the application
    app.run().await?;

    Ok(())
}
