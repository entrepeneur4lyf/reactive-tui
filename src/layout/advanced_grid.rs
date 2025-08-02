//! Advanced grid system for TUI applications
//!
//! This module provides an intuitive, utility-class-based grid system with CSS-like semantics.
//! It combines powerful grid layout capabilities with a user-friendly class-based approach.
//!
//! # Grid Classes
//!
//! **Container Classes:**
//! - `grid` - Creates a grid container
//! - `grid-cols-{n}` - Sets number of columns (1-12)
//! - `grid-rows-{n}` - Sets number of rows (1-12)
//! - `gap-{n}` - Sets gap between items (0-8)
//! - `gap-x-{n}` - Sets horizontal gap
//! - `gap-y-{n}` - Sets vertical gap
//!
//! **Item Classes:**
//! - `col-span-{n}` - Spans across n columns
//! - `row-span-{n}` - Spans across n rows
//! - `col-start-{n}` - Starts at column n
//! - `col-end-{n}` - Ends at column n
//! - `row-start-{n}` - Starts at row n
//! - `row-end-{n}` - Ends at row n
//!
//! **Responsive & Auto Classes:**
//! - `grid-cols-auto` - Auto-sized columns
//! - `grid-flow-row` - Items flow by rows
//! - `grid-flow-col` - Items flow by columns

use crate::{
    components::Element,
    error::Result,
    layout::{ComputedStyles, DisplayType, Layout, LayoutRect},
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Parameters for finding next position in grid
#[derive(Debug, Clone)]
struct PositionParams {
    start_col: usize,
    start_row: usize,
    span_cols: usize,
    span_rows: usize,
    column_count: usize,
}

/// Parameters for calculating cell rectangle
#[derive(Debug, Clone)]
struct CellRectParams {
    col: usize,
    row: usize,
    span_cols: usize,
    span_rows: usize,
    cell_width: u16,
    cell_height: u16,
    gap_x: u16,
    gap_y: u16,
}

/// CSS-style grid configuration parsed from CSS classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Grid {
    /// Number of columns (grid-cols-{n})
    pub columns: GridColumns,
    /// Number of rows (grid-rows-{n})
    pub rows: GridRows,
    /// Gap between grid items
    pub gap: GridGap,
    /// Grid flow direction
    pub flow: GridFlow,
    /// Auto-placement algorithm
    pub auto_placement: AutoPlacement,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridColumns {
    /// Fixed number of columns
    Fixed(u8),
    /// Auto-sized columns
    Auto,
    /// Subgrid (inherits from parent)
    Subgrid,
    /// Custom column definitions
    Custom(Vec<GridTrackSize>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridRows {
    /// Fixed number of rows
    Fixed(u8),
    /// Auto-sized rows
    Auto,
    /// Subgrid (inherits from parent)
    Subgrid,
    /// Custom row definitions
    Custom(Vec<GridTrackSize>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GridGap {
    pub x: u8,
    pub y: u8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridFlow {
    Row,
    Column,
    RowDense,
    ColumnDense,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum AutoPlacement {
    Auto,
    Dense,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum GridTrackSize {
    /// Fixed size in cells
    Fixed(u16),
    /// Fraction unit (fr)
    Fr(f32),
    /// Percentage
    Percent(f32),
    /// Auto-size to content
    Auto,
    /// Minimum content size
    MinContent,
    /// Maximum content size
    MaxContent,
    /// Min-max range
    MinMax(Box<GridTrackSize>, Box<GridTrackSize>),
}

/// Grid item placement information from CSS classes
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AdvancedGridItem {
    /// Column span (col-span-{n})
    pub column_span: u8,
    /// Row span (row-span-{n})
    pub row_span: u8,
    /// Column start position (col-start-{n})
    pub column_start: Option<u8>,
    /// Column end position (col-end-{n})
    pub column_end: Option<u8>,
    /// Row start position (row-start-{n})
    pub row_start: Option<u8>,
    /// Row end position (row-end-{n})
    pub row_end: Option<u8>,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            columns: GridColumns::Fixed(1),
            rows: GridRows::Auto,
            gap: GridGap { x: 0, y: 0 },
            flow: GridFlow::Row,
            auto_placement: AutoPlacement::Auto,
        }
    }
}

impl Default for AdvancedGridItem {
    fn default() -> Self {
        Self {
            column_span: 1,
            row_span: 1,
            column_start: None,
            column_end: None,
            row_start: None,
            row_end: None,
        }
    }
}

/// Advanced Grid Layout Engine
#[derive(Debug, Clone)]
pub struct GridLayout {
    /// Column widths in pixels
    pub columns: Vec<u16>,
    /// Row heights in pixels
    pub rows: Vec<u16>,
    /// Gap between grid items in pixels
    pub gap: u16,
}

impl GridLayout {
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            gap: 0,
        }
    }

    pub fn with_dimensions(_width: u16, _height: u16) -> Self {
        // Advanced grid is container-relative, so dimensions are handled by parent layout
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
            gap: 0,
        }
    }

    /// Create responsive grid configuration based on terminal size
    pub fn create_responsive_config(terminal_width: u16, terminal_height: u16) -> Grid {
        Grid {
            columns: match terminal_width {
                0..=40 => GridColumns::Fixed(1),    // Mobile-like: single column
                41..=80 => GridColumns::Fixed(2),   // Small: two columns
                81..=120 => GridColumns::Fixed(3),  // Medium: three columns
                121..=160 => GridColumns::Fixed(4), // Large: four columns
                _ => GridColumns::Fixed(6),         // Extra large: six columns
            },
            gap: match terminal_width {
                0..=60 => GridGap { x: 0, y: 0 }, // Tight spacing for narrow terminals
                61..=100 => GridGap { x: 1, y: 1 }, // Balanced spacing
                _ => GridGap { x: 2, y: 1 },      // Generous spacing for wide terminals
            },
            rows: if terminal_height < 20 {
                GridRows::Fixed(2) // Limit rows for short terminals
            } else {
                GridRows::Auto // Auto-size for normal terminals
            },
            ..Default::default()
        }
    }

    /// Optimize grid configuration for content
    pub fn optimize_for_content(config: &mut Grid, content_count: usize, terminal_width: u16) {
        if let GridColumns::Auto = config.columns {
            // Calculate optimal column count based on content and terminal width
            let optimal_cols = match content_count {
                0..=2 => 1,
                3..=4 => 2,
                5..=9 => 3,
                10..=16 => 4,
                _ => 6,
            };

            // Constrain by terminal width
            let max_cols = (terminal_width / 15).max(1) as u8; // Minimum 15 chars per column
            let final_cols = optimal_cols.min(max_cols as usize).max(1);

            config.columns = GridColumns::Fixed(final_cols as u8);
        }
    }

    /// Get terminal dimensions for responsive calculations
    pub fn get_terminal_size() -> (u16, u16) {
        // Fallback to modern 1440p defaults (180x90)
        crossterm::terminal::size().unwrap_or((180, 90))
    }

    /// Parse CSS grid classes from an element
    pub fn parse_grid_config(&self, element: &Element) -> Grid {
        let mut config = Grid::default();

        for class in &element.classes {
            match class.as_str() {
                // Column definitions
                "grid-cols-1" => config.columns = GridColumns::Fixed(1),
                "grid-cols-2" => config.columns = GridColumns::Fixed(2),
                "grid-cols-3" => config.columns = GridColumns::Fixed(3),
                "grid-cols-4" => config.columns = GridColumns::Fixed(4),
                "grid-cols-5" => config.columns = GridColumns::Fixed(5),
                "grid-cols-6" => config.columns = GridColumns::Fixed(6),
                "grid-cols-7" => config.columns = GridColumns::Fixed(7),
                "grid-cols-8" => config.columns = GridColumns::Fixed(8),
                "grid-cols-9" => config.columns = GridColumns::Fixed(9),
                "grid-cols-10" => config.columns = GridColumns::Fixed(10),
                "grid-cols-11" => config.columns = GridColumns::Fixed(11),
                "grid-cols-12" => config.columns = GridColumns::Fixed(12),
                "grid-cols-auto" => config.columns = GridColumns::Auto,
                "grid-cols-subgrid" => config.columns = GridColumns::Subgrid,

                // Row definitions
                "grid-rows-1" => config.rows = GridRows::Fixed(1),
                "grid-rows-2" => config.rows = GridRows::Fixed(2),
                "grid-rows-3" => config.rows = GridRows::Fixed(3),
                "grid-rows-4" => config.rows = GridRows::Fixed(4),
                "grid-rows-5" => config.rows = GridRows::Fixed(5),
                "grid-rows-6" => config.rows = GridRows::Fixed(6),
                "grid-rows-auto" => config.rows = GridRows::Auto,
                "grid-rows-subgrid" => config.rows = GridRows::Subgrid,

                // Gap classes
                "gap-0" => config.gap = GridGap { x: 0, y: 0 },
                "gap-1" => config.gap = GridGap { x: 1, y: 1 },
                "gap-2" => config.gap = GridGap { x: 2, y: 2 },
                "gap-3" => config.gap = GridGap { x: 3, y: 3 },
                "gap-4" => config.gap = GridGap { x: 4, y: 4 },
                "gap-5" => config.gap = GridGap { x: 5, y: 5 },
                "gap-6" => config.gap = GridGap { x: 6, y: 6 },
                "gap-8" => config.gap = GridGap { x: 8, y: 8 },

                // X gap classes
                "gap-x-0" => config.gap.x = 0,
                "gap-x-1" => config.gap.x = 1,
                "gap-x-2" => config.gap.x = 2,
                "gap-x-3" => config.gap.x = 3,
                "gap-x-4" => config.gap.x = 4,
                "gap-x-5" => config.gap.x = 5,
                "gap-x-6" => config.gap.x = 6,
                "gap-x-8" => config.gap.x = 8,

                // Y gap classes
                "gap-y-0" => config.gap.y = 0,
                "gap-y-1" => config.gap.y = 1,
                "gap-y-2" => config.gap.y = 2,
                "gap-y-3" => config.gap.y = 3,
                "gap-y-4" => config.gap.y = 4,
                "gap-y-5" => config.gap.y = 5,
                "gap-y-6" => config.gap.y = 6,
                "gap-y-8" => config.gap.y = 8,

                // Flow classes
                "grid-flow-row" => config.flow = GridFlow::Row,
                "grid-flow-col" => config.flow = GridFlow::Column,
                "grid-flow-row-dense" => config.flow = GridFlow::RowDense,
                "grid-flow-col-dense" => config.flow = GridFlow::ColumnDense,

                _ => {}
            }
        }

        config
    }

    /// Parse CSS grid item classes from an element
    pub fn parse_grid_item(&self, element: &Element) -> AdvancedGridItem {
        let mut item = AdvancedGridItem::default();

        for class in &element.classes {
            match class.as_str() {
                // Column span
                "col-span-1" => item.column_span = 1,
                "col-span-2" => item.column_span = 2,
                "col-span-3" => item.column_span = 3,
                "col-span-4" => item.column_span = 4,
                "col-span-5" => item.column_span = 5,
                "col-span-6" => item.column_span = 6,
                "col-span-7" => item.column_span = 7,
                "col-span-8" => item.column_span = 8,
                "col-span-9" => item.column_span = 9,
                "col-span-10" => item.column_span = 10,
                "col-span-11" => item.column_span = 11,
                "col-span-12" => item.column_span = 12,
                "col-span-full" => item.column_span = 255, // Will be clamped to grid size

                // Row span
                "row-span-1" => item.row_span = 1,
                "row-span-2" => item.row_span = 2,
                "row-span-3" => item.row_span = 3,
                "row-span-4" => item.row_span = 4,
                "row-span-5" => item.row_span = 5,
                "row-span-6" => item.row_span = 6,
                "row-span-full" => item.row_span = 255, // Will be clamped to grid size

                // Column start
                "col-start-1" => item.column_start = Some(1),
                "col-start-2" => item.column_start = Some(2),
                "col-start-3" => item.column_start = Some(3),
                "col-start-4" => item.column_start = Some(4),
                "col-start-5" => item.column_start = Some(5),
                "col-start-6" => item.column_start = Some(6),
                "col-start-7" => item.column_start = Some(7),
                "col-start-8" => item.column_start = Some(8),
                "col-start-9" => item.column_start = Some(9),
                "col-start-10" => item.column_start = Some(10),
                "col-start-11" => item.column_start = Some(11),
                "col-start-12" => item.column_start = Some(12),
                "col-start-auto" => item.column_start = None,

                // Column end
                "col-end-1" => item.column_end = Some(1),
                "col-end-2" => item.column_end = Some(2),
                "col-end-3" => item.column_end = Some(3),
                "col-end-4" => item.column_end = Some(4),
                "col-end-5" => item.column_end = Some(5),
                "col-end-6" => item.column_end = Some(6),
                "col-end-7" => item.column_end = Some(7),
                "col-end-8" => item.column_end = Some(8),
                "col-end-9" => item.column_end = Some(9),
                "col-end-10" => item.column_end = Some(10),
                "col-end-11" => item.column_end = Some(11),
                "col-end-12" => item.column_end = Some(12),
                "col-end-13" => item.column_end = Some(13), // End after last column
                "col-end-auto" => item.column_end = None,

                // Row start
                "row-start-1" => item.row_start = Some(1),
                "row-start-2" => item.row_start = Some(2),
                "row-start-3" => item.row_start = Some(3),
                "row-start-4" => item.row_start = Some(4),
                "row-start-5" => item.row_start = Some(5),
                "row-start-6" => item.row_start = Some(6),
                "row-start-7" => item.row_start = Some(7),
                "row-start-auto" => item.row_start = None,

                // Row end
                "row-end-1" => item.row_end = Some(1),
                "row-end-2" => item.row_end = Some(2),
                "row-end-3" => item.row_end = Some(3),
                "row-end-4" => item.row_end = Some(4),
                "row-end-5" => item.row_end = Some(5),
                "row-end-6" => item.row_end = Some(6),
                "row-end-7" => item.row_end = Some(7),
                "row-end-auto" => item.row_end = None,

                _ => {}
            }
        }

        // Handle explicit positioning from start/end
        if let (Some(start), Some(end)) = (item.column_start, item.column_end) {
            item.column_span = (end - start).max(1);
        }

        if let (Some(start), Some(end)) = (item.row_start, item.row_end) {
            item.row_span = (end - start).max(1);
        }

        item
    }

    /// Compute the grid layout using CSS-style classes
    pub fn compute_layout(&self, parent: &Element, container_rect: LayoutRect) -> Result<Layout> {
        // Check if this is a grid container
        if !parent.classes.contains(&"grid".to_string()) {
            return Err(crate::error::TuiError::LayoutError(
                "Element must have 'grid' class to use grid layout".to_string(),
            ));
        }

        let grid_config = self.parse_grid_config(parent);
        let children = &parent.children;

        if children.is_empty() {
            return Ok(Layout {
                rect: container_rect,
                children: Vec::new(),
                element_id: parent.id.clone(),
                tag: parent.tag.clone(),
                content: parent.content.clone(),
                styles: ComputedStyles::default(),
                focused: parent.focused,
                focusable: parent.focusable,
            });
        }

        // Build grid placement map
        let placements = self.build_grid_placements(&grid_config, children, container_rect)?;

        // Convert placements to child layouts
        let mut child_layouts = Vec::new();

        for (element, rect) in placements {
            let child_layout = Layout {
                rect,
                children: Vec::new(), // Simplified for now
                element_id: element.id.clone(),
                tag: element.tag.clone(),
                content: element.content.clone(),
                styles: ComputedStyles::default(),
                focused: element.focused,
                focusable: element.focusable,
            };
            child_layouts.push(child_layout);
        }

        Ok(Layout {
            rect: container_rect,
            children: child_layouts,
            element_id: parent.id.clone(),
            tag: parent.tag.clone(),
            content: parent.content.clone(),
            styles: ComputedStyles {
                display: DisplayType::Block, // Grid is block-level
                ..ComputedStyles::default()
            },
            focused: parent.focused,
            focusable: parent.focusable,
        })
    }

    /// Build grid placements for all children
    fn build_grid_placements(
        &self,
        config: &Grid,
        children: &[Element],
        container_rect: LayoutRect,
    ) -> Result<Vec<(Element, LayoutRect)>> {
        // Determine grid dimensions
        let column_count = match &config.columns {
            GridColumns::Fixed(n) => *n as usize,
            GridColumns::Auto => {
                // Auto-calculate based on children count and aspect ratio
                let count = children.len();
                if count <= 4 {
                    count
                } else {
                    ((count as f32).sqrt().ceil() as usize).max(1)
                }
            }
            GridColumns::Subgrid => 1,   // Simplified for now
            GridColumns::Custom(_) => 1, // Simplified for now
        };

        let row_count = match &config.rows {
            GridRows::Fixed(n) => *n as usize,
            GridRows::Auto => children.len().div_ceil(column_count),
            GridRows::Subgrid => 1,   // Simplified for now
            GridRows::Custom(_) => 1, // Simplified for now
        };

        // Calculate cell dimensions with improved precision
        let total_gap_x = config.gap.x as u16 * (column_count.saturating_sub(1)) as u16;
        let total_gap_y = config.gap.y as u16 * (row_count.saturating_sub(1)) as u16;

        let available_width = container_rect.width.saturating_sub(total_gap_x);
        let available_height = container_rect.height.saturating_sub(total_gap_y);

        // Use integer division with remainder distribution for better space utilization
        let cell_width = available_width / column_count as u16;
        let width_remainder = available_width % column_count as u16;

        let cell_height = available_height / row_count as u16;
        let height_remainder = available_height % row_count as u16;

        // Track occupied cells
        let mut occupied_cells: HashMap<(usize, usize), usize> = HashMap::new();
        let mut placements = Vec::new();

        // Place items with explicit positions first
        let mut auto_items = Vec::new();

        for (index, child) in children.iter().enumerate() {
            let item_config = self.parse_grid_item(child);

            if let (Some(col_start), Some(row_start)) =
                (item_config.column_start, item_config.row_start)
            {
                // Explicit positioning (1-based to 0-based)
                let col = (col_start - 1) as usize;
                let row = (row_start - 1) as usize;

                let span_cols = (item_config.column_span as usize).min(column_count - col);
                let span_rows = (item_config.row_span as usize).min(row_count - row);

                // Mark cells as occupied
                for r in row..(row + span_rows) {
                    for c in col..(col + span_cols) {
                        occupied_cells.insert((c, r), index);
                    }
                }

                let rect = self.calculate_cell_rect_with_remainder(
                    CellRectParams {
                        col,
                        row,
                        span_cols,
                        span_rows,
                        cell_width,
                        cell_height,
                        gap_x: config.gap.x as u16,
                        gap_y: config.gap.y as u16,
                    },
                    container_rect,
                    width_remainder,
                    height_remainder,
                    column_count,
                    row_count,
                );

                placements.push((child.clone(), rect));
            } else {
                auto_items.push((index, child, item_config));
            }
        }

        // Place auto items using grid flow
        let mut current_col = 0;
        let mut current_row = 0;

        for (index, child, item_config) in auto_items {
            // Find next available position
            let (col, row) = self.find_next_position(
                &occupied_cells,
                PositionParams {
                    start_col: current_col,
                    start_row: current_row,
                    span_cols: item_config.column_span as usize,
                    span_rows: item_config.row_span as usize,
                    column_count,
                },
                &config.flow,
            );

            let span_cols =
                (item_config.column_span as usize).min(column_count.saturating_sub(col));
            let span_rows = (item_config.row_span as usize).min(row_count.saturating_sub(row));

            // Mark cells as occupied
            for r in row..(row + span_rows) {
                for c in col..(col + span_cols) {
                    occupied_cells.insert((c, r), index);
                }
            }

            let rect = self.calculate_cell_rect_with_remainder(
                CellRectParams {
                    col,
                    row,
                    span_cols,
                    span_rows,
                    cell_width,
                    cell_height,
                    gap_x: config.gap.x as u16,
                    gap_y: config.gap.y as u16,
                },
                container_rect,
                width_remainder,
                height_remainder,
                column_count,
                row_count,
            );

            placements.push((child.clone(), rect));

            // Update current position based on flow
            match config.flow {
                GridFlow::Row | GridFlow::RowDense => {
                    current_col = col + span_cols;
                    if current_col >= column_count {
                        current_col = 0;
                        current_row = row + 1;
                    }
                }
                GridFlow::Column | GridFlow::ColumnDense => {
                    current_row = row + span_rows;
                    if current_row >= row_count {
                        current_row = 0;
                        current_col = col + 1;
                    }
                }
            }
        }

        Ok(placements)
    }

    /// Find the next available position in the grid with optimized search
    fn find_next_position(
        &self,
        occupied: &HashMap<(usize, usize), usize>,
        params: PositionParams,
        flow: &GridFlow,
    ) -> (usize, usize) {
        let mut row = params.start_row;
        let mut col = params.start_col;

        // Dense packing tries to fill gaps
        let dense = matches!(flow, GridFlow::RowDense | GridFlow::ColumnDense);

        if dense {
            // Start from beginning for dense packing
            row = 0;
            col = 0;
        }

        // Optimization: pre-calculate maximum search bounds to prevent infinite loops
        let max_row = occupied.keys().map(|(_, r)| *r).max().unwrap_or(0) + params.span_rows + 10;
        let max_col = params.column_count;

        loop {
            // Check if we've exceeded reasonable bounds
            if row > max_row || col >= max_col {
                // Fallback: extend grid vertically
                return (0, max_row);
            }

            // Optimized fit check: early exit on first collision
            if self.can_fit_at_position(occupied, col, row, params.span_cols, params.span_rows) {
                return (col, row);
            }

            // Move to next position based on flow
            match flow {
                GridFlow::Row | GridFlow::RowDense => {
                    col += 1;
                    if col + params.span_cols > params.column_count {
                        col = 0;
                        row += 1;
                    }
                }
                GridFlow::Column | GridFlow::ColumnDense => {
                    row += 1;
                    if row > max_row {
                        row = 0;
                        col += 1;
                    }
                }
            }
        }
    }

    /// Optimized helper to check if an item can fit at a specific position
    #[inline]
    fn can_fit_at_position(
        &self,
        occupied: &HashMap<(usize, usize), usize>,
        col: usize,
        row: usize,
        span_cols: usize,
        span_rows: usize,
    ) -> bool {
        for r in row..(row + span_rows) {
            for c in col..(col + span_cols) {
                if occupied.contains_key(&(c, r)) {
                    return false;
                }
            }
        }
        true
    }

    /// Calculate the rectangle for a grid cell with remainder pixel distribution
    fn calculate_cell_rect_with_remainder(
        &self,
        params: CellRectParams,
        container_rect: LayoutRect,
        width_remainder: u16,
        height_remainder: u16,
        _column_count: usize,
        _row_count: usize,
    ) -> LayoutRect {
        // Calculate cumulative offset considering remainder distribution
        let mut x_offset = 0u16;
        let mut y_offset = 0u16;

        // Distribute width remainder across first columns
        for col in 0..params.col {
            let extra_width = if col < width_remainder as usize { 1 } else { 0 };
            x_offset += params.cell_width + extra_width + params.gap_x;
        }

        // Distribute height remainder across first rows
        for row in 0..params.row {
            let extra_height = if row < height_remainder as usize {
                1
            } else {
                0
            };
            y_offset += params.cell_height + extra_height + params.gap_y;
        }

        let x = container_rect.x + x_offset;
        let y = container_rect.y + y_offset;

        // Calculate spanned width and height with remainder distribution
        let mut width = 0u16;
        let mut height = 0u16;

        for col in params.col..(params.col + params.span_cols) {
            let extra_width = if col < width_remainder as usize { 1 } else { 0 };
            width += params.cell_width + extra_width;
            if col < params.col + params.span_cols - 1 {
                width += params.gap_x;
            }
        }

        for row in params.row..(params.row + params.span_rows) {
            let extra_height = if row < height_remainder as usize {
                1
            } else {
                0
            };
            height += params.cell_height + extra_height;
            if row < params.row + params.span_rows - 1 {
                height += params.gap_y;
            }
        }

        LayoutRect {
            x,
            y,
            width,
            height,
        }
    }

    /// Calculate the rectangle for a grid cell (legacy method for compatibility)
    #[allow(dead_code)]
    fn calculate_cell_rect(
        &self,
        params: CellRectParams,
        container_rect: LayoutRect,
    ) -> LayoutRect {
        self.calculate_cell_rect_with_remainder(params, container_rect, 0, 0, 0, 0)
    }
}

impl Default for GridLayout {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::components::Element;

    #[test]
    fn test_parse_grid_config() {
        let layout = GridLayout::new();
        let element = Element::with_tag("div")
            .class("grid")
            .class("grid-cols-3")
            .class("gap-2")
            .build();

        let config = layout.parse_grid_config(&element);
        assert_eq!(config.columns, GridColumns::Fixed(3));
        assert_eq!(config.gap.x, 2);
        assert_eq!(config.gap.y, 2);
    }

    #[test]
    fn test_parse_grid_item() {
        let layout = GridLayout::new();
        let element = Element::with_tag("div")
            .class("col-span-2")
            .class("row-start-3")
            .build();

        let item = layout.parse_grid_item(&element);
        assert_eq!(item.column_span, 2);
        assert_eq!(item.row_start, Some(3));
    }

    #[test]
    fn test_grid_layout_basic() {
        let layout = GridLayout::new();
        let parent = Element::with_tag("div")
            .class("grid")
            .class("grid-cols-2")
            .child(Element::with_tag("item").content("Item 1").build())
            .child(Element::with_tag("item").content("Item 2").build())
            .build();

        let container = LayoutRect {
            x: 0,
            y: 0,
            width: 100,
            height: 50,
        };
        let result = layout.compute_layout(&parent, container).unwrap();

        assert_eq!(result.children.len(), 2);
        // First child should be in left column
        assert_eq!(result.children[0].rect.x, 0);
        // Second child should be in right column
        assert_eq!(result.children[1].rect.x, 50);
    }

    #[test]
    fn test_grid_with_spans() {
        let layout = GridLayout::new();
        let parent = Element::with_tag("div")
            .class("grid")
            .class("grid-cols-3")
            .child(
                Element::with_tag("item")
                    .class("col-span-2")
                    .content("Wide Item")
                    .build(),
            )
            .child(Element::with_tag("item").content("Normal Item").build())
            .build();

        let container = LayoutRect {
            x: 0,
            y: 0,
            width: 90,
            height: 30,
        };
        let result = layout.compute_layout(&parent, container).unwrap();

        assert_eq!(result.children.len(), 2);
        // First child spans 2 columns, should be 60 units wide
        assert_eq!(result.children[0].rect.width, 60);
        // Second child should be in the remaining column
        assert_eq!(result.children[1].rect.width, 30);
    }
}
