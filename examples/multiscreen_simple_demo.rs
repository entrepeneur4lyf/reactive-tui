/*!
 * Simple Multi-screen Demo
 *
 * A simplified demonstration of multi-screen functionality
 */

use reactive_tui::components::{div, text, Element};
use reactive_tui::prelude::*;
use std::sync::{Arc, RwLock};

/// Simple screen state manager
struct ScreenManager {
  screens: Vec<Screen>,
  current: Arc<RwLock<usize>>,
  #[allow(dead_code)]
  history: Arc<RwLock<Vec<usize>>>,
}

struct Screen {
  #[allow(dead_code)]
  id: String,
  title: String,
  content: String,
}

#[allow(dead_code)]
impl ScreenManager {
  fn new() -> Self {
    let screens = vec![
            Screen {
                id: "home".to_string(),
                title: "🏠 Home".to_string(),
                content: "Welcome to the multi-screen demo!\n\nPress:\n[1] Home\n[2] Settings\n[3] Profile\n[4] Dashboard\n[Tab] Next screen\n[Esc] Previous screen\n[Q] Quit".to_string(),
            },
            Screen {
                id: "settings".to_string(),
                title: "⚙️ Settings".to_string(),
                content: "Application Settings\n\nTheme: Dark\nLanguage: English\nAnimations: Enabled\n\nPress [Esc] to go back".to_string(),
            },
            Screen {
                id: "profile".to_string(),
                title: "👤 Profile".to_string(),
                content: "User Profile\n\nUsername: demo_user\nEmail: demo@example.com\nRole: Administrator\n\nPress [Esc] to go back".to_string(),
            },
            Screen {
                id: "dashboard".to_string(),
                title: "📊 Dashboard".to_string(),
                content: "System Dashboard\n\nCPU Usage: 23%\nMemory: 4.2 GB\nDisk: 120 GB free\nNetwork: Connected\n\nPress [Esc] to go back".to_string(),
            },
        ];

    Self {
      screens,
      current: Arc::new(RwLock::new(0)),
      history: Arc::new(RwLock::new(vec![])),
    }
  }

  fn navigate_to(&self, index: usize) {
    if index < self.screens.len() {
      let current = *self.current.read().unwrap();
      self.history.write().unwrap().push(current);
      *self.current.write().unwrap() = index;
    }
  }

  fn navigate_back(&self) {
    if let Some(prev) = self.history.write().unwrap().pop() {
      *self.current.write().unwrap() = prev;
    }
  }

  fn next_screen(&self) {
    let current = *self.current.read().unwrap();
    let next = (current + 1) % self.screens.len();
    self.navigate_to(next);
  }

  fn render(&self) -> Element {
    let current = *self.current.read().unwrap();
    let screen = &self.screens[current];

    div()
      .class("screen-manager")
      .class("h-full")
      .child(
        // Header
        div()
          .class("header")
          .class("border-bottom")
          .class("p-2")
          .child(text(&format!("{} | Multi-Screen Demo", screen.title)).build())
          .build(),
      )
      .child(
        // Content
        div()
          .class("content")
          .class("flex-1")
          .class("p-4")
          .child(text(&screen.content).build())
          .build(),
      )
      .child(
        // Footer
        div()
          .class("footer")
          .class("border-top")
          .class("p-2")
          .child(
            text(&format!(
              "Screen {} of {} | [Tab] Next | [Esc] Back | [Q] Quit",
              current + 1,
              self.screens.len()
            ))
            .build(),
          )
          .build(),
      )
      .build()
  }
}

impl Component for ScreenManager {
  fn render(&self) -> Element {
    self.render()
  }
}

#[tokio::main]
async fn main() -> Result<()> {
  println!("🚀 Simple Multi-Screen Demo");
  println!("==========================\n");

  let screen_manager = ScreenManager::new();

  let app = TuiAppBuilder::new()
    .with_title("Multi-Screen Demo")
    .component(screen_manager)
    .build()?;

  app.run().await?;

  println!("\nThanks for trying the multi-screen demo!");
  Ok(())
}
