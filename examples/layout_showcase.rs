use crossterm::{
  cursor::{Hide, MoveTo, Show},
  event::{read, Event, KeyCode},
  execute,
  terminal::{disable_raw_mode, enable_raw_mode, size, Clear, ClearType},
};
use reactive_tui::components::main as main_element;
use reactive_tui::components::{div, footer, header, section, text};
use reactive_tui::prelude::*;
use std::io::{stdout, Write};

struct LayoutShowcase {
  current_page: usize,
  terminal_size: (u16, u16),
  layout_engine: LayoutEngine,
}

impl LayoutShowcase {
  fn new() -> Self {
    Self {
      current_page: 0,
      terminal_size: (400, 200),
      layout_engine: LayoutEngine::new(),
    }
  }

  fn total_pages(&self) -> usize {
    4
  }

  fn render(&mut self) -> Result<()> {
    let (width, height) = self.terminal_size;
    self.layout_engine.update_dimensions(width, height);

    execute!(stdout(), Clear(ClearType::All), MoveTo(0, 0))?;

    let root_element = match self.current_page {
      0 => self.create_basic_layouts_page(),
      1 => self.create_flexbox_layouts_page(),
      2 => self.create_grid_layouts_page(),
      3 => self.create_complex_app_page(),
      _ => self.create_basic_layouts_page(),
    };

    let layout = self.layout_engine.compute_layout(&root_element)?;
    self.render_layout(&layout)?;

    // Footer
    execute!(stdout(), MoveTo(0, height - 1))?;
    print!(
      "Page {}/{} | Space: next | Q: quit",
      self.current_page + 1,
      self.total_pages()
    );

    stdout().flush()?;
    Ok(())
  }

  fn create_basic_layouts_page(&self) -> Element {
    div()
      .class("flex")
      .class("flex-col")
      .class("h-full")
      .child_builder(
        header()
          .class("h-1/4")
          .class("flex")
          .class("items-center")
          .class("justify-center")
          .child_builder(text("BASIC LAYOUTS")),
      )
      .child_builder(
        main_element()
          .class("flex-1")
          .class("flex")
          .class("flex-row")
          .child_builder(
            section()
              .class("w-1/2")
              .class("flex")
              .class("flex-col")
              .class("p-2")
              .child_builder(div().class("h-1/4").child_builder(text("Vertical Layout")))
              .child_builder(
                div()
                  .class("flex-1")
                  .class("flex")
                  .class("flex-col")
                  .child_builder(div().class("h-1/3").child_builder(text("Header")))
                  .child_builder(div().class("flex-1").child_builder(text("Content")))
                  .child_builder(div().class("h-1/3").child_builder(text("Footer"))),
              ),
          )
          .child_builder(
            section()
              .class("w-1/2")
              .class("flex")
              .class("flex-col")
              .class("p-2")
              .child_builder(
                div()
                  .class("h-1/4")
                  .child_builder(text("Horizontal Layout")),
              )
              .child_builder(
                div()
                  .class("flex-1")
                  .class("flex")
                  .class("flex-row")
                  .child_builder(div().class("w-1/3").child_builder(text("Left")))
                  .child_builder(div().class("flex-1").child_builder(text("Center")))
                  .child_builder(div().class("w-1/3").child_builder(text("Right"))),
              ),
          ),
      )
      .build()
  }

  fn create_flexbox_layouts_page(&self) -> Element {
    div()
      .class("flex")
      .class("flex-col")
      .class("h-full")
      .child_builder(
        header()
          .class("h-1/6")
          .class("flex")
          .class("items-center")
          .class("justify-center")
          .child_builder(text("FLEXBOX LAYOUTS")),
      )
      .child_builder(
        main_element()
          .class("flex-1")
          .class("flex")
          .class("flex-col")
          .child_builder(
            div()
              .class("h-1/3")
              .class("flex")
              .class("justify-center")
              .class("items-center")
              .child_builder(text("justify-center + items-center")),
          )
          .child_builder(
            div()
              .class("h-1/3")
              .class("flex")
              .class("justify-between")
              .class("items-center")
              .child_builder(text("Left"))
              .child_builder(text("Center"))
              .child_builder(text("Right")),
          )
          .child_builder(
            div()
              .class("h-1/3")
              .class("flex")
              .class("flex-col")
              .class("justify-between")
              .child_builder(text("justify-between"))
              .child_builder(text("in column"))
              .child_builder(text("layout")),
          ),
      )
      .build()
  }

  fn create_grid_layouts_page(&self) -> Element {
    div()
      .class("flex")
      .class("flex-col")
      .class("h-full")
      .child_builder(
        header()
          .class("h-1/6")
          .class("flex")
          .class("items-center")
          .class("justify-center")
          .child_builder(text("GRID LAYOUTS")),
      )
      .child_builder(
        main_element()
          .class("flex-1")
          .class("flex")
          .class("flex-row")
          .child_builder(
            section()
              .class("w-1/2")
              .class("grid")
              .class("grid-cols-3")
              .class("grid-rows-3")
              .class("gap-1")
              .class("p-2")
              .child_builder(text("1"))
              .child_builder(text("2"))
              .child_builder(text("3"))
              .child_builder(text("4"))
              .child_builder(text("5"))
              .child_builder(text("6"))
              .child_builder(text("7"))
              .child_builder(text("8"))
              .child_builder(text("9")),
          )
          .child_builder(
            section()
              .class("w-1/2")
              .class("grid")
              .class("grid-cols-2")
              .class("grid-rows-4")
              .class("gap-2")
              .class("p-2")
              .child_builder(
                div()
                  .class("col-span-2")
                  .child_builder(text("Spanning 2 columns")),
              )
              .child_builder(text("A"))
              .child_builder(text("B"))
              .child_builder(text("C"))
              .child_builder(text("D"))
              .child_builder(
                div()
                  .class("col-span-2")
                  .child_builder(text("Another span")),
              ),
          ),
      )
      .build()
  }

  fn create_complex_app_page(&self) -> Element {
    div()
      .class("flex")
      .class("flex-col")
      .class("h-full")
      .child_builder(
        header()
          .class("h-1/12")
          .class("flex")
          .class("items-center")
          .class("justify-between")
          .class("p-1")
          .child_builder(text("App Title"))
          .child_builder(text("Menu"))
          .child_builder(text("User")),
      )
      .child_builder(
        main_element()
          .class("flex-1")
          .class("flex")
          .class("flex-row")
          .child_builder(
            section()
              .class("w-1/4")
              .class("flex")
              .class("flex-col")
              .child_builder(
                div()
                  .class("h-1/2")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Sidebar")),
              )
              .child_builder(
                div()
                  .class("h-1/2")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Tools")),
              ),
          )
          .child_builder(
            section()
              .class("flex-1")
              .class("flex")
              .class("flex-col")
              .child_builder(
                div()
                  .class("h-1/12")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Toolbar")),
              )
              .child_builder(
                div()
                  .class("flex-1")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Main Content Area")),
              )
              .child_builder(
                div()
                  .class("h-1/4")
                  .class("flex")
                  .class("flex-row")
                  .child_builder(
                    div()
                      .class("w-1/2")
                      .class("flex")
                      .class("items-center")
                      .class("justify-center")
                      .child_builder(text("Terminal")),
                  )
                  .child_builder(
                    div()
                      .class("w-1/2")
                      .class("flex")
                      .class("items-center")
                      .class("justify-center")
                      .child_builder(text("Output")),
                  ),
              ),
          )
          .child_builder(
            section()
              .class("w-1/6")
              .class("flex")
              .class("flex-col")
              .child_builder(
                div()
                  .class("h-1/3")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Minimap")),
              )
              .child_builder(
                div()
                  .class("flex-1")
                  .class("flex")
                  .class("items-center")
                  .class("justify-center")
                  .child_builder(text("Properties")),
              ),
          ),
      )
      .child_builder(
        footer()
          .class("h-1/12")
          .class("flex")
          .class("items-center")
          .class("justify-between")
          .class("p-1")
          .child_builder(text("Status"))
          .child_builder(text("Ready"))
          .child_builder(text("100%")),
      )
      .build()
  }

  fn render_layout(&self, layout: &Layout) -> Result<()> {
    self.render_layout_recursive(layout, 0)?;
    Ok(())
  }

  #[allow(clippy::only_used_in_recursion)]
  fn render_layout_recursive(&self, layout: &Layout, depth: usize) -> Result<()> {
    let rect = &layout.rect;

    // Draw border and content for this layout with depth-based styling
    if rect.width > 2 && rect.height > 2 {
      // Use different border styles based on depth
      let border_char = match depth % 3 {
        0 => "─",
        1 => "═",
        _ => "━",
      };

      // Draw border
      execute!(stdout(), MoveTo(rect.x, rect.y))?;
      print!("┌{}┐", border_char.repeat((rect.width - 2) as usize));

      for y in 1..(rect.height - 1) {
        execute!(stdout(), MoveTo(rect.x, rect.y + y))?;
        print!("│");
        execute!(stdout(), MoveTo(rect.x + rect.width - 1, rect.y + y))?;
        print!("│");
      }

      execute!(stdout(), MoveTo(rect.x, rect.y + rect.height - 1))?;
      print!("└{}┘", "─".repeat((rect.width - 2) as usize));

      // Render content
      if let Some(content) = &layout.content {
        if !content.is_empty() && rect.width > content.len() as u16 + 2 {
          execute!(
            stdout(),
            MoveTo(
              rect.x + (rect.width - content.len() as u16) / 2,
              rect.y + rect.height / 2
            )
          )?;
          print!("{content}");
        }
      }
    }

    // Render children
    for child in &layout.children {
      self.render_layout_recursive(child, depth + 1)?;
    }

    Ok(())
  }

  fn next_page(&mut self) {
    self.current_page = (self.current_page + 1) % self.total_pages();
  }
}

fn main() -> Result<()> {
  enable_raw_mode()?;
  execute!(stdout(), Hide, Clear(ClearType::All))?;

  let mut showcase = LayoutShowcase::new();
  showcase.terminal_size = size()?;

  loop {
    showcase.render()?;

    if let Event::Key(key_event) = read()? {
      match key_event.code {
        KeyCode::Char('q') | KeyCode::Char('Q') => break,
        KeyCode::Char(' ') | KeyCode::Right | KeyCode::Enter => showcase.next_page(),
        _ => {}
      }
    }
  }

  disable_raw_mode()?;
  execute!(stdout(), Show, Clear(ClearType::All), MoveTo(0, 0))?;
  println!("Layout showcase completed!");

  Ok(())
}
