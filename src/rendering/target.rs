use crate::error::Result;
use crate::layout::LayoutRect;
use crate::rendering::{FrameBuffer, RenderStyle};

pub trait RenderTarget {
  fn apply_style(&mut self, style: &RenderStyle) -> Result<()>;
  fn move_to(&mut self, x: u16, y: u16) -> Result<()>;
  fn print(&mut self, text: &str) -> Result<()>;
  fn fill_background_rect(&mut self, rect: LayoutRect, ch: char) -> Result<()>;
}

pub struct AnsiTarget<'a> {
  fb: &'a mut FrameBuffer,
}

impl<'a> AnsiTarget<'a> {
  pub fn new(fb: &'a mut FrameBuffer) -> Self {
    Self { fb }
  }
}

impl<'a> RenderTarget for AnsiTarget<'a> {
  fn apply_style(&mut self, style: &RenderStyle) -> Result<()> {
    self.fb.apply_style(style)
  }
  fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
    self.fb.move_to(x, y)
  }
  fn print(&mut self, text: &str) -> Result<()> {
    self.fb.print(text)
  }
  fn fill_background_rect(&mut self, rect: LayoutRect, ch: char) -> Result<()> {
    let row = ch.to_string();
    for r in 0..rect.height {
      self.fb.move_to(rect.x, rect.y + r)?;
      for _ in 0..rect.width {
        self.fb.print(&row)?;
      }
    }
    Ok(())
  }
}

#[derive(Clone, Debug)]
pub struct Cell {
  pub ch: String, // preserve grapheme/ANSI sequences
  pub style: RenderStyle,
}

#[derive(Clone, Debug)]
pub struct CellGrid {
  pub width: u16,
  pub height: u16,
  pub rows: Vec<Vec<Cell>>, // rows of cells with style
}

impl CellGrid {
  pub fn new(width: u16, height: u16) -> Self {
    let mut rows = Vec::with_capacity(height as usize);
    for _ in 0..height {
      rows.push(Vec::new());
    }
    Self {
      width,
      height,
      rows,
    }
  }

  pub fn clear(&mut self) {
    for row in &mut self.rows {
      row.clear();
    }
  }
}

pub struct GridTarget<'a> {
  pub grid: &'a mut CellGrid,
  current_style: RenderStyle,
}

impl<'a> GridTarget<'a> {
  pub fn new(grid: &'a mut CellGrid) -> Self {
    Self {
      grid,
      current_style: RenderStyle::default(),
    }
  }
}

impl<'a> RenderTarget for GridTarget<'a> {
  fn apply_style(&mut self, style: &RenderStyle) -> Result<()> {
    self.current_style = style.clone();
    Ok(())
  }
  fn move_to(&mut self, _x: u16, _y: u16) -> Result<()> {
    Ok(())
  }
  fn print(&mut self, text: &str) -> Result<()> {
    // Append as a single segment to the last row; the renderer will place by move_to first
    if let Some(row) = self.grid.rows.last_mut() {
      row.push(Cell {
        ch: text.to_string(),
        style: self.current_style.clone(),
      });
    }
    Ok(())
  }
  fn fill_background_rect(&mut self, _rect: LayoutRect, _ch: char) -> Result<()> {
    Ok(())
  }
}
