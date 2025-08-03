use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{read, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use reactive_tui::components::Element;
use reactive_tui::layout::{Layout, LayoutEngine};
use reactive_tui::prelude::*;
use std::io::{stdout, Write};

struct LayoutVisualDemo {
  current_demo: usize,
  terminal_size: (u16, u16),
  layout_engine: LayoutEngine,
}

impl LayoutVisualDemo {
  fn new() -> Self {
    Self {
      current_demo: 0,
      terminal_size: (80, 24),
      layout_engine: LayoutEngine::new(),
    }
  }

  fn total_demos(&self) -> usize {
    5
  }

  fn render(&mut self) -> Result<()> {
    let (width, height) = self.terminal_size;
    self.layout_engine.update_dimensions(width, height - 3); // Leave space for title and footer

    // Clear the entire screen first
    execute!(stdout(), Clear(ClearType::All))?;

    let (title, element) = match self.current_demo {
      0 => ("📐 Vertical Layout (Column)", self.create_vertical_layout()),
      1 => (
        "↔️ Horizontal Layout (Row)",
        self.create_horizontal_layout(),
      ),
      2 => ("🏗️ Complex App Layout", self.create_complex_app_layout()),
      3 => ("📊 CSS Grid Layout", self.create_grid_layout()),
      4 => ("🎯 Centered Layout", self.create_centered_layout()),
      _ => ("📐 Vertical Layout (Column)", self.create_vertical_layout()),
    };

    // Print title at the top
    execute!(stdout(), MoveTo(0, 0))?;
    print!("{}", title);

    let layout = self.layout_engine.compute_layout(&element)?;
    self.render_layout_visual(&layout, 0)?;

    // Footer at the bottom
    execute!(stdout(), MoveTo(0, height - 1))?;
    print!(
      "Demo {}/{} | Space/Enter: next | Left: prev | Q: quit",
      self.current_demo + 1,
      self.total_demos()
    );

    stdout().flush()?;
    Ok(())
  }

  fn create_vertical_layout(&self) -> Element {
    Element::with_tag("vbox")
      .id("app")
      .class("flex-col")
      .class("h-full")
      .child(
        Element::with_tag("div")
          .id("header")
          .content("Header Section")
          .attr("height", "3")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("content")
          .content("Main Content Area")
          .class("h-2/3")
          .class("p-2")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("footer")
          .content("Footer Section")
          .attr("height", "2")
          .class("p-1")
          .into(),
      )
      .build()
  }

  fn create_horizontal_layout(&self) -> Element {
    Element::with_tag("hbox")
      .id("container")
      .class("flex-row")
      .class("w-full")
      .child(
        Element::with_tag("div")
          .id("sidebar")
          .content("Sidebar")
          .attr("width", "15")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("main")
          .content("Main Content")
          .class("flex-1")
          .class("p-2")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("aside")
          .content("Aside")
          .attr("width", "12")
          .class("p-1")
          .into(),
      )
      .build()
  }

  fn create_complex_app_layout(&self) -> Element {
    Element::with_tag("div")
      .id("app")
      .class("flex-col")
      .class("h-full")
      .child(
        Element::with_tag("div")
          .id("header")
          .content("📋 Application Header")
          .attr("height", "3")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("body")
          .class("flex-row")
          .class("flex-1")
          .child(
            Element::with_tag("div")
              .id("sidebar")
              .content("📁 Navigation")
              .attr("width", "18")
              .class("p-1")
              .into(),
          )
          .child(
            Element::with_tag("div")
              .id("main-area")
              .class("flex-col")
              .class("flex-1")
              .child(
                Element::with_tag("div")
                  .id("toolbar")
                  .content("🛠️ Toolbar")
                  .attr("height", "3")
                  .class("p-1")
                  .into(),
              )
              .child(
                Element::with_tag("div")
                  .id("workspace")
                  .content("📄 Main Workspace")
                  .class("flex-1")
                  .class("p-1")
                  .into(),
              )
              .into(),
          )
          .child(
            Element::with_tag("div")
              .id("properties")
              .content("⚙️ Properties")
              .attr("width", "20")
              .class("p-1")
              .into(),
          )
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("statusbar")
          .content("✅ Ready | 12:34 PM")
          .attr("height", "3")
          .class("p-1")
          .into(),
      )
      .build()
  }

  fn create_grid_layout(&self) -> Element {
    Element::with_tag("div")
      .id("grid-container")
      .class("grid")
      .class("w-full")
      .class("h-full")
      .child(
        Element::with_tag("div")
          .id("grid-header")
          .content("Grid Header (1,1)")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("grid-nav")
          .content("Grid Nav (2,1)")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("grid-main")
          .content("Grid Main (2,2)")
          .class("p-1")
          .into(),
      )
      .child(
        Element::with_tag("div")
          .id("grid-aside")
          .content("Grid Aside (2,3)")
          .class("p-1")
          .into(),
      )
      .build()
  }

  fn create_centered_layout(&self) -> Element {
    Element::with_tag("center")
      .id("centered-container")
      .class("w-full")
      .class("h-full")
      .child(
        Element::with_tag("div")
          .id("modal")
          .content("🪟 Centered Modal Dialog")
          .attr("width", "30")
          .attr("height", "8")
          .class("p-2")
          .into(),
      )
      .build()
  }

  fn render_layout_visual(&self, layout: &Layout, depth: usize) -> Result<()> {
    let rect = &layout.rect;

    // Skip rendering if too small
    if rect.width < 4 || rect.height < 3 {
      return Ok(());
    }

    // Choose border style based on depth and element type
    let (top_left, top_right, bottom_left, bottom_right, horizontal, vertical) = match depth % 4 {
      0 => ("┌", "┐", "└", "┘", "─", "│"), // Standard
      1 => ("╔", "╗", "╚", "╝", "═", "║"), // Double
      2 => ("┏", "┓", "┗", "┛", "━", "┃"), // Heavy
      _ => ("╭", "╮", "╰", "╯", "─", "│"), // Rounded
    };

    // Offset all rendering by 2 lines (1 for title, 1 for spacing)
    let y_offset = 2;

    // Only draw borders for elements that have content or are leaf nodes
    let should_draw_border = layout.content.is_some() || layout.children.is_empty();

    if should_draw_border {
      // Draw top border
      execute!(stdout(), MoveTo(rect.x, rect.y + y_offset))?;
      print!(
        "{}{}{}",
        top_left,
        horizontal.repeat((rect.width - 2) as usize),
        top_right
      );

      // Draw side borders
      for y in 1..(rect.height - 1) {
        execute!(stdout(), MoveTo(rect.x, rect.y + y + y_offset))?;
        print!("{}", vertical);
        execute!(
          stdout(),
          MoveTo(rect.x + rect.width - 1, rect.y + y + y_offset)
        )?;
        print!("{}", vertical);
      }

      // Draw bottom border
      execute!(
        stdout(),
        MoveTo(rect.x, rect.y + rect.height - 1 + y_offset)
      )?;
      print!(
        "{}{}{}",
        bottom_left,
        horizontal.repeat((rect.width - 2) as usize),
        bottom_right
      );

      // Render content in the center
      if let Some(content) = &layout.content {
        if !content.is_empty() && rect.width > content.len() as u16 + 2 && rect.height > 2 {
          let content_x = rect.x + (rect.width - content.len() as u16) / 2;
          let content_y = rect.y + rect.height / 2 + y_offset;
          execute!(stdout(), MoveTo(content_x, content_y))?;
          print!("{}", content);
        }
      }

      // Add element ID in top-left corner if there's space and no content
      if layout.content.is_none() {
        if let Some(id) = &layout.element_id {
          if rect.width > id.len() as u16 + 4 && rect.height > 2 {
            execute!(stdout(), MoveTo(rect.x + 2, rect.y + y_offset))?;
            print!("{}", id);
          }
        }
      }
    }

    // Render children
    for child in &layout.children {
      self.render_layout_visual(child, depth + 1)?;
    }

    Ok(())
  }

  fn next_demo(&mut self) {
    self.current_demo = (self.current_demo + 1) % self.total_demos();
  }
}

fn main() -> Result<()> {
  enable_raw_mode()?;
  execute!(stdout(), Hide, Clear(ClearType::All))?;

  let mut demo = LayoutVisualDemo::new();
  demo.terminal_size = size()?;

  loop {
    demo.render()?;

    if let Event::Key(key_event) = read()? {
      match key_event.code {
        KeyCode::Char('q') | KeyCode::Char('Q') | KeyCode::Esc => break,
        KeyCode::Char(' ') | KeyCode::Right | KeyCode::Enter => demo.next_demo(),
        KeyCode::Left => {
          // Go to previous demo
          demo.current_demo = if demo.current_demo == 0 {
            demo.total_demos() - 1
          } else {
            demo.current_demo - 1
          };
        }
        _ => {}
      }
    }
  }

  disable_raw_mode()?;
  execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0))?;
  println!("Layout visual demo completed!");

  Ok(())
}
