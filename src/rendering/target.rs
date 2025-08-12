#![allow(dead_code)]

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

#[derive(Clone, Debug, PartialEq)]
pub struct Cell {
  pub x: u16,
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
  cur_x: u16,
  cur_y: u16,
}

impl<'a> GridTarget<'a> {
  pub fn new(grid: &'a mut CellGrid) -> Self {
    Self {
      grid,
      current_style: RenderStyle::default(),
      cur_x: 0,
      cur_y: 0,
    }
  }
}

impl<'a> RenderTarget for GridTarget<'a> {
  fn apply_style(&mut self, style: &RenderStyle) -> Result<()> {
    self.current_style = style.clone();
    Ok(())
  }
  fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
    self.cur_x = x;
    self.cur_y = y;
    Ok(())
  }
  fn print(&mut self, text: &str) -> Result<()> {
    // Append as a single segment to the addressed row
    let y = self.cur_y as usize;
    if self.grid.rows.len() <= y {
      self.grid.rows.resize_with(y + 1, Vec::new);
    }
    self.grid.rows[y].push(Cell {
      x: self.cur_x,
      ch: text.to_string(),
      style: self.current_style.clone(),
    });
    Ok(())
  }
  fn fill_background_rect(&mut self, _rect: LayoutRect, _ch: char) -> Result<()> {
    Ok(())
  }
}

impl<'a> GridTarget<'a> {
  pub fn fill_background_rect_rows(&mut self, rect: LayoutRect, ch: char) {
    let y_end = rect.y.saturating_add(rect.height);
    let row_str = ch.to_string().repeat(rect.width as usize);
    for y in rect.y..y_end {
      let row_idx = y as usize;
      if self.grid.rows.len() <= row_idx {
        self.grid.rows.resize_with(row_idx + 1, Vec::new);
      }
      self.grid.rows[row_idx].push(Cell {
        x: rect.x,
        ch: row_str.clone(),
        style: self.current_style.clone(),
      });
    }
  }
}

pub struct MultiTarget<'a> {
  pub ansi: AnsiTarget<'a>,
  pub grid: GridTarget<'a>,
}

impl<'a> MultiTarget<'a> {
  pub fn new(fb: &'a mut FrameBuffer, grid: &'a mut CellGrid) -> Self {
    Self {
      ansi: AnsiTarget::new(fb),
      grid: GridTarget::new(grid),
    }
  }
}

impl<'a> RenderTarget for MultiTarget<'a> {
  fn apply_style(&mut self, style: &RenderStyle) -> Result<()> {
    self.ansi.apply_style(style)?;
    self.grid.apply_style(style)
  }
  fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
    self.ansi.move_to(x, y)?;
    self.grid.move_to(x, y)
  }
  fn print(&mut self, text: &str) -> Result<()> {
    self.ansi.print(text)?;
    self.grid.print(text)
  }
  fn fill_background_rect(&mut self, rect: LayoutRect, ch: char) -> Result<()> {
    self.ansi.fill_background_rect(rect, ch)?;
    self.grid.fill_background_rect_rows(rect, ch);
    Ok(())
  }
}
