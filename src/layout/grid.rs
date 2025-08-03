//! Advanced grid layout system for structured component arrangement
//!
//! This module provides a flexible, CSS-like grid system that supports:
//! - Dynamic column/row sizing with fr units, percentages, and fixed sizes
//! - Widget spanning across multiple cells
//! - Auto-sizing based on content
//! - Grid gutters and spacing
//! - Responsive behavior with expand/shrink options

use crate::{components::Element, error::Result, layout::LayoutRect};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A scalar value for grid sizing, similar to CSS units
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum GridScalar {
  /// Fixed size in terminal cells
  Cells(u16),
  /// Fraction units (like CSS fr) - proportional sizing
  Fr(f32),
  /// Percentage of available space
  Percent(f32),
  /// Auto-size based on content
  Auto,
  /// Minimum content size
  MinContent,
  /// Maximum content size
  MaxContent,
}

impl GridScalar {
  /// Parse a scalar from string (e.g., "1fr", "50%", "10", "auto")
  pub fn parse(input: &str) -> Option<Self> {
    let input = input.trim();

    if input == "auto" {
      return Some(GridScalar::Auto);
    }

    if input == "min-content" {
      return Some(GridScalar::MinContent);
    }

    if input == "max-content" {
      return Some(GridScalar::MaxContent);
    }

    if input.ends_with("fr") {
      if let Ok(value) = input.trim_end_matches("fr").parse::<f32>() {
        return Some(GridScalar::Fr(value));
      }
    }

    if input.ends_with('%') {
      if let Ok(value) = input.trim_end_matches('%').parse::<f32>() {
        return Some(GridScalar::Percent(value));
      }
    }

    if input.ends_with("px") {
      if let Ok(value) = input.trim_end_matches("px").parse::<u16>() {
        return Some(GridScalar::Cells(value));
      }
    }

    if let Ok(value) = input.parse::<u16>() {
      return Some(GridScalar::Cells(value));
    }

    None
  }

  /// Check if this scalar represents a fraction unit
  pub fn is_fraction(&self) -> bool {
    matches!(self, GridScalar::Fr(_))
  }

  /// Check if this scalar requires content measurement
  pub fn is_auto(&self) -> bool {
    matches!(
      self,
      GridScalar::Auto | GridScalar::MinContent | GridScalar::MaxContent
    )
  }

  /// Get the fraction value if this is a fraction unit
  pub fn fraction_value(&self) -> f32 {
    match self {
      GridScalar::Fr(value) => *value,
      _ => 0.0,
    }
  }
}

/// Configuration for grid layout
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
  /// Column definitions
  pub columns: Vec<GridScalar>,
  /// Row definitions
  pub rows: Vec<GridScalar>,
  /// Horizontal gutter between columns
  pub column_gap: u16,
  /// Vertical gutter between rows
  pub row_gap: u16,
  /// Number of columns (auto-calculated if None)
  pub column_count: Option<usize>,
  /// Number of rows (auto-calculated if None)
  pub row_count: Option<usize>,
  /// Minimum column width for auto-sizing
  pub min_column_width: Option<u16>,
  /// Whether to stretch row heights to be equal
  pub stretch_height: bool,
  /// Expand grid to fill container if smaller
  pub expand: bool,
  /// Shrink grid to fit container if larger
  pub shrink: bool,
  /// Auto-detect minimum widths when shrinking
  pub auto_minimum: bool,
}

impl Default for GridConfig {
  fn default() -> Self {
    Self {
      columns: vec![GridScalar::Fr(1.0)],
      rows: vec![GridScalar::Auto],
      column_gap: 0,
      row_gap: 0,
      column_count: None,
      row_count: None,
      min_column_width: None,
      stretch_height: false,
      expand: false,
      shrink: false,
      auto_minimum: false,
    }
  }
}

/// Represents a cell position and span in the grid
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct GridCell {
  pub column: usize,
  pub row: usize,
  pub column_span: usize,
  pub row_span: usize,
}

impl GridCell {
  pub fn new(column: usize, row: usize) -> Self {
    Self {
      column,
      row,
      column_span: 1,
      row_span: 1,
    }
  }

  pub fn with_span(column: usize, row: usize, column_span: usize, row_span: usize) -> Self {
    Self {
      column,
      row,
      column_span,
      row_span,
    }
  }

  /// Get all coordinates occupied by this cell
  pub fn occupied_coords(&self) -> Vec<(usize, usize)> {
    let mut coords = Vec::new();
    for row in self.row..(self.row + self.row_span) {
      for col in self.column..(self.column + self.column_span) {
        coords.push((col, row));
      }
    }
    coords
  }
}

/// Grid placement information for a widget
#[derive(Debug, Clone)]
pub struct GridPlacement {
  pub element_id: String,
  pub cell: GridCell,
  pub rect: LayoutRect,
  pub is_primary: bool, // True for the top-left cell of a spanned widget
}

/// Resolved grid track (column or row) with offset and size
#[derive(Debug, Clone, Copy)]
pub struct GridTrack {
  pub offset: u16,
  pub size: u16,
}

/// Advanced grid layout engine
#[derive(Debug)]
pub struct GridLayout {
  config: GridConfig,
  terminal_width: u16,
  terminal_height: u16,
}

impl GridLayout {
  pub fn new(config: GridConfig) -> Self {
    // Detect actual terminal size, fallback to modern high-resolution defaults
    let (width, height) = crossterm::terminal::size().unwrap_or((400, 200));
    Self {
      config,
      terminal_width: width,
      terminal_height: height,
    }
  }

  pub fn with_dimensions(mut self, width: u16, height: u16) -> Self {
    self.terminal_width = width;
    self.terminal_height = height;
    self
  }

  /// Update terminal dimensions for responsive layouts
  pub fn update_dimensions(&mut self, width: u16, height: u16) {
    self.terminal_width = width;
    self.terminal_height = height;
  }

  /// Get current terminal dimensions
  pub fn dimensions(&self) -> (u16, u16) {
    (self.terminal_width, self.terminal_height)
  }

  /// Compute the grid layout for the given elements
  pub fn compute_layout(
    &self,
    parent: &Element,
    container_rect: LayoutRect,
  ) -> Result<Vec<GridPlacement>> {
    let children = &parent.children;
    if children.is_empty() {
      return Ok(Vec::new());
    }

    // Build cell map - assign each widget to grid cells
    let (cell_map, column_count, row_count) = self.build_cell_map(children)?;

    // Resolve column and row tracks
    let columns = self.resolve_tracks(
      &self.config.columns,
      column_count,
      container_rect.width,
      true,
    )?;
    let rows = self.resolve_tracks(&self.config.rows, row_count, container_rect.height, false)?;

    // Create placements for each widget
    let mut placements = Vec::new();

    for (element_id, cell) in cell_map.iter() {
      // Find the corresponding element
      let element = children
        .iter()
        .find(|e| {
          let child_id = e.id.clone().unwrap_or_else(|| format!("element_{}", e.tag));
          &child_id == element_id
        })
        .ok_or_else(|| {
          crate::error::TuiError::LayoutError(format!(
            "Could not find element with id: {element_id}"
          ))
        })?;

      let placement = self.create_placement(element, *cell, &columns, &rows, container_rect)?;
      placements.push(placement);
    }

    Ok(placements)
  }

  /// Build the cell map by assigning widgets to grid positions
  fn build_cell_map(
    &self,
    children: &[Element],
  ) -> Result<(HashMap<String, GridCell>, usize, usize)> {
    let mut cell_map = HashMap::new();
    let mut occupied_coords: HashMap<(usize, usize), String> = HashMap::new();

    let mut current_column = 0;
    let mut current_row = 0;
    let mut max_column = 0;
    let mut max_row = 0;

    // Determine grid size
    let column_count = self.config.column_count.unwrap_or_else(|| {
      if let Some(min_width) = self.config.min_column_width {
        let available_width = self.terminal_width;
        let cols =
          (available_width + self.config.column_gap) / (min_width + self.config.column_gap);
        cols.min(children.len() as u16).max(1) as usize
      } else {
        ((children.len() as f32).sqrt().ceil() as usize).max(1)
      }
    });

    for child in children {
      let element_id = child
        .id
        .clone()
        .unwrap_or_else(|| format!("element_{}", child.tag));

      // Get span information from attributes
      let column_span = child
        .get_attribute("column-span")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);
      let row_span = child
        .get_attribute("row-span")
        .and_then(|s| s.parse().ok())
        .unwrap_or(1);

      // Get explicit position if specified
      let explicit_column = child
        .get_attribute("grid-column")
        .and_then(|s| s.parse().ok());
      let explicit_row = child.get_attribute("grid-row").and_then(|s| s.parse().ok());

      // Find next available position
      let (column, row) = if let (Some(col), Some(row)) = (explicit_column, explicit_row) {
        (col, row)
      } else {
        self.find_next_available_cell(
          &occupied_coords,
          current_column,
          current_row,
          column_span,
          row_span,
          column_count,
        )
      };

      let cell = GridCell::with_span(column, row, column_span, row_span);

      // Mark all occupied coordinates
      for coord in cell.occupied_coords() {
        occupied_coords.insert(coord, element_id.clone());
      }

      cell_map.insert(element_id, cell);

      max_column = max_column.max(column + column_span - 1);
      max_row = max_row.max(row + row_span - 1);

      // Update current position for next widget
      current_column = column + column_span;
      if current_column >= column_count {
        current_column = 0;
        current_row = row + 1;
      }
    }

    let final_column_count = (max_column + 1).max(column_count);
    let final_row_count = max_row + 1;

    Ok((cell_map, final_column_count, final_row_count))
  }

  /// Find the next available cell that can accommodate the given span
  fn find_next_available_cell(
    &self,
    occupied: &HashMap<(usize, usize), String>,
    start_column: usize,
    start_row: usize,
    column_span: usize,
    row_span: usize,
    column_count: usize,
  ) -> (usize, usize) {
    let mut row = start_row;
    let mut column = start_column;

    loop {
      // Check if current position can accommodate the span
      let mut can_fit = true;

      for r in row..(row + row_span) {
        for c in column..(column + column_span) {
          if occupied.contains_key(&(c, r)) {
            can_fit = false;
            break;
          }
        }
        if !can_fit {
          break;
        }
      }

      if can_fit {
        return (column, row);
      }

      // Move to next position
      column += 1;
      if column + column_span > column_count {
        column = 0;
        row += 1;
      }
    }
  }

  /// Resolve grid tracks (columns or rows) into concrete positions and sizes
  fn resolve_tracks(
    &self,
    scalars: &[GridScalar],
    count: usize,
    available_space: u16,
    is_columns: bool,
  ) -> Result<Vec<GridTrack>> {
    let gap = if is_columns {
      self.config.column_gap
    } else {
      self.config.row_gap
    };
    let total_gap = gap * (count.saturating_sub(1)) as u16;
    let content_space = available_space.saturating_sub(total_gap) as f32;

    // Extend scalars to match count by repeating
    let mut extended_scalars = Vec::new();
    for i in 0..count {
      extended_scalars.push(
        scalars
          .get(i % scalars.len())
          .copied()
          .unwrap_or(GridScalar::Fr(1.0)),
      );
    }

    // First pass: resolve fixed sizes and calculate remaining space
    let mut resolved_sizes = vec![0.0; count];
    let mut total_fractions = 0.0;
    let mut used_space = 0.0;

    for (i, scalar) in extended_scalars.iter().enumerate() {
      match scalar {
        GridScalar::Cells(cells) => {
          resolved_sizes[i] = *cells as f32;
          used_space += *cells as f32;
        }
        GridScalar::Percent(pct) => {
          resolved_sizes[i] = content_space * (pct / 100.0);
          used_space += resolved_sizes[i];
        }
        GridScalar::Fr(fr) => {
          total_fractions += fr;
        }
        GridScalar::Auto | GridScalar::MinContent | GridScalar::MaxContent => {
          // TODO: Implement content-based sizing
          // For now, treat as 1fr
          total_fractions += 1.0;
        }
      }
    }

    // Second pass: resolve fraction units
    let remaining_space = (content_space - used_space).max(0.0);
    let fraction_unit = if total_fractions > 0.0 {
      remaining_space / total_fractions
    } else {
      0.0
    };

    for (i, scalar) in extended_scalars.iter().enumerate() {
      match scalar {
        GridScalar::Fr(fr) => {
          resolved_sizes[i] = fraction_unit * fr;
        }
        GridScalar::Auto | GridScalar::MinContent | GridScalar::MaxContent => {
          resolved_sizes[i] = fraction_unit;
        }
        _ => {} // Already resolved
      }
    }

    // Handle expand/shrink if configured
    if self.config.expand || self.config.shrink {
      let total_used = resolved_sizes.iter().sum::<f32>();
      let space_diff = content_space - total_used;

      if self.config.expand && space_diff > 0.0 {
        // Distribute extra space proportionally
        let scale_factor = content_space / total_used;
        for size in &mut resolved_sizes {
          *size *= scale_factor;
        }
      } else if self.config.shrink && space_diff < 0.0 {
        // Scale down proportionally
        let scale_factor = content_space / total_used;
        for size in &mut resolved_sizes {
          *size *= scale_factor;
        }
      }
    }

    // Build tracks with offsets
    let mut tracks = Vec::new();
    let mut current_offset = 0;

    for (i, size) in resolved_sizes.iter().enumerate() {
      tracks.push(GridTrack {
        offset: current_offset,
        size: size.round() as u16,
      });

      current_offset += size.round() as u16;
      if i < count - 1 {
        current_offset += gap;
      }
    }

    Ok(tracks)
  }

  /// Create a grid placement for a widget
  fn create_placement(
    &self,
    element: &Element,
    cell: GridCell,
    columns: &[GridTrack],
    rows: &[GridTrack],
    container_rect: LayoutRect,
  ) -> Result<GridPlacement> {
    let start_column = columns.get(cell.column).ok_or_else(|| {
      crate::error::TuiError::LayoutError("Column index out of bounds".to_string())
    })?;

    let start_row = rows
      .get(cell.row)
      .ok_or_else(|| crate::error::TuiError::LayoutError("Row index out of bounds".to_string()))?;

    // Calculate spanned area
    let end_column_idx = (cell.column + cell.column_span - 1).min(columns.len() - 1);
    let end_row_idx = (cell.row + cell.row_span - 1).min(rows.len() - 1);

    let end_column = &columns[end_column_idx];
    let end_row = &rows[end_row_idx];

    let width = (end_column.offset + end_column.size) - start_column.offset;
    let height = (end_row.offset + end_row.size) - start_row.offset;

    let rect = LayoutRect {
      x: container_rect.x + start_column.offset,
      y: container_rect.y + start_row.offset,
      width,
      height,
    };

    let element_id = element
      .id
      .clone()
      .unwrap_or_else(|| format!("element_{}", element.tag));

    Ok(GridPlacement {
      element_id,
      cell,
      rect,
      is_primary: true,
    })
  }

  /// Update grid configuration
  pub fn set_config(&mut self, config: GridConfig) {
    self.config = config;
  }

  /// Get current grid configuration
  pub fn config(&self) -> &GridConfig {
    &self.config
  }
}

/// CSS-style grid template parsing utilities
pub mod template {
  use super::GridScalar;

  /// Parse a grid template string like "1fr 100px 2fr" or "repeat(3, 1fr)"
  pub fn parse_template(template: &str) -> Vec<GridScalar> {
    let template = template.trim();

    // Handle repeat() function
    if template.starts_with("repeat(") && template.ends_with(')') {
      return parse_repeat_template(template);
    }

    // Split by whitespace and parse each value
    template
      .split_whitespace()
      .filter_map(GridScalar::parse)
      .collect()
  }

  fn parse_repeat_template(template: &str) -> Vec<GridScalar> {
    // Extract content between repeat( and )
    let content = &template[7..template.len() - 1]; // Remove "repeat(" and ")"

    let parts: Vec<&str> = content.splitn(2, ',').collect();
    if parts.len() != 2 {
      return Vec::new();
    }

    let count = parts[0].trim().parse::<usize>().unwrap_or(1);
    let pattern = parse_template(parts[1].trim());

    pattern.into_iter().cycle().take(count).collect()
  }

  #[cfg(test)]
  mod tests {
    use super::*;

    #[test]
    fn test_parse_template() {
      let template = parse_template("1fr 100px 2fr");
      assert_eq!(template.len(), 3);
      assert!(matches!(template[0], GridScalar::Fr(1.0)));
      assert!(matches!(template[1], GridScalar::Cells(100)));
      assert!(matches!(template[2], GridScalar::Fr(2.0)));
    }

    #[test]
    fn test_parse_repeat() {
      let template = parse_repeat_template("repeat(3, 1fr)");
      assert_eq!(template.len(), 3);
      assert!(template.iter().all(|s| matches!(s, GridScalar::Fr(1.0))));
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::components::Element;

  #[test]
  fn test_grid_scalar_parsing() {
    assert_eq!(GridScalar::parse("auto"), Some(GridScalar::Auto));
    assert_eq!(GridScalar::parse("1fr"), Some(GridScalar::Fr(1.0)));
    assert_eq!(GridScalar::parse("50%"), Some(GridScalar::Percent(50.0)));
    assert_eq!(GridScalar::parse("100"), Some(GridScalar::Cells(100)));
    assert_eq!(
      GridScalar::parse("min-content"),
      Some(GridScalar::MinContent)
    );
  }

  #[test]
  fn test_grid_cell_coords() {
    let cell = GridCell::with_span(1, 2, 2, 3);
    let coords = cell.occupied_coords();
    assert_eq!(coords.len(), 6); // 2 columns × 3 rows
    assert!(coords.contains(&(1, 2)));
    assert!(coords.contains(&(2, 4)));
  }

  #[test]
  fn test_grid_layout_basic() {
    let config = GridConfig {
      columns: vec![GridScalar::Fr(1.0), GridScalar::Fr(1.0)],
      rows: vec![GridScalar::Auto],
      column_count: Some(2),
      ..Default::default()
    };

    let grid = GridLayout::new(config);

    let parent = Element::with_tag("div")
      .child(Element::with_tag("item1").build())
      .child(Element::with_tag("item2").build())
      .build();

    let container = LayoutRect {
      x: 0,
      y: 0,
      width: 100,
      height: 50,
    };
    let placements = grid.compute_layout(&parent, container).unwrap();

    assert_eq!(placements.len(), 2);
    assert_eq!(placements[0].cell.column, 0);
    assert_eq!(placements[1].cell.column, 1);
  }
}
