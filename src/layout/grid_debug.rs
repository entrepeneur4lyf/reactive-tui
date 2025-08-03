//! Grid debugging and inspection tools
//!
//! This module provides comprehensive debugging capabilities for grid layouts,
//! helping developers visualize, inspect, and troubleshoot grid positioning,
//! sizing, and content placement issues.

use crate::{
  components::Element,
  layout::{
    advanced_grid::{Grid, GridLayout},
    Layout, LayoutRect,
  },
  themes::{color_to_ansi, hex, ColorDefinition, UtilityProcessor},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Debug visualization modes for grid inspection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridDebugMode {
  /// Show grid lines and cell boundaries
  GridLines,
  /// Show item placement and spans
  ItemPlacement,
  /// Show gaps and spacing
  Spacing,
  /// Show occupied vs available cells
  Occupancy,
  /// Comprehensive debug view with all information
  Full,
  /// Minimal overlay for production debugging
  Minimal,
}

/// Grid debug configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridDebugConfig {
  /// Debug visualization mode
  pub mode: GridDebugMode,
  /// Show grid coordinates (0,0), (1,0), etc.
  pub show_coordinates: bool,
  /// Show item indices and IDs
  pub show_item_info: bool,
  /// Show cell dimensions
  pub show_dimensions: bool,
  /// Show CSS class information
  pub show_classes: bool,
  /// Color scheme for debug visualization
  pub color_scheme: GridDebugColors,
  /// Enable interactive debugging features
  pub interactive: bool,
}

/// Color scheme for grid debugging visualization
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridDebugColors {
  /// Grid line color
  pub grid_lines: ColorDefinition,
  /// Occupied cell background
  pub occupied_cell: ColorDefinition,
  /// Empty cell background
  pub empty_cell: ColorDefinition,
  /// Gap area color
  pub gap_color: ColorDefinition,
  /// Item border color
  pub item_border: ColorDefinition,
  /// Coordinate text color
  pub coordinate_text: ColorDefinition,
  /// Info text color
  pub info_text: ColorDefinition,
}

/// Detailed grid inspection data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridInspectionReport {
  /// Grid configuration summary
  pub config: GridConfigSummary,
  /// All grid items with detailed placement info
  pub items: Vec<GridItemReport>,
  /// Grid statistics
  pub stats: GridStatistics,
  /// Layout warnings and issues
  pub warnings: Vec<GridWarning>,
  /// Performance metrics
  pub performance: GridPerformanceMetrics,
}

/// Summary of grid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfigSummary {
  /// Number of columns
  pub columns: String,
  /// Number of rows
  pub rows: String,
  /// Gap configuration
  pub gap: String,
  /// Grid flow direction
  pub flow: String,
  /// Container dimensions
  pub container_size: (u16, u16),
  /// Cell dimensions
  pub cell_size: (u16, u16),
}

/// Individual grid item placement report
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridItemReport {
  /// Item element ID
  pub id: Option<String>,
  /// Item index in grid
  pub index: usize,
  /// Grid position (col, row)
  pub position: (usize, usize),
  /// Span (cols, rows)
  pub span: (usize, usize),
  /// Computed rectangle
  pub rect: LayoutRect,
  /// CSS classes applied
  pub classes: Vec<String>,
  /// Item content preview
  pub content: Option<String>,
  /// Placement type (auto or explicit)
  pub placement_type: PlacementType,
}

/// How an item was placed in the grid
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlacementType {
  /// Automatically placed by grid flow algorithm
  Auto,
  /// Explicitly positioned with start/end coordinates
  Explicit,
  /// Positioned with span classes only
  Span,
}

/// Grid layout statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridStatistics {
  /// Total number of cells
  pub total_cells: usize,
  /// Number of occupied cells
  pub occupied_cells: usize,
  /// Number of empty cells
  pub empty_cells: usize,
  /// Occupancy percentage
  pub occupancy_rate: f32,
  /// Average item span
  pub avg_item_span: f32,
  /// Grid efficiency score (0-100)
  pub efficiency_score: f32,
}

/// Grid layout warnings and issues
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridWarning {
  /// Warning type
  pub warning_type: GridWarningType,
  /// Human-readable warning message
  pub message: String,
  /// Item index causing the warning (if applicable)
  pub item_index: Option<usize>,
  /// Suggested fix
  pub suggestion: Option<String>,
}

/// Types of grid layout warnings
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum GridWarningType {
  /// Item extends beyond grid boundaries
  Overflow,
  /// Inefficient space utilization
  LowEfficiency,
  /// Items overlapping (shouldn't happen but good to check)
  Overlap,
  /// Very large gaps or wasted space
  ExcessiveGaps,
  /// Items too small for their content
  ContentTruncation,
  /// Performance issues (too many items, complex layout)
  Performance,
}

/// Performance metrics for grid layout computation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridPerformanceMetrics {
  /// Layout computation time in microseconds
  pub layout_time_us: u64,
  /// Number of placement iterations required
  pub placement_iterations: usize,
  /// Memory usage estimate in bytes
  pub memory_usage: usize,
  /// Complexity score (higher = more complex)
  pub complexity_score: f32,
}

/// Grid debugging and inspection tool
pub struct GridDebugger {
  config: GridDebugConfig,
  _utility_processor: UtilityProcessor,
}

impl Default for GridDebugConfig {
  fn default() -> Self {
    Self {
      mode: GridDebugMode::GridLines,
      show_coordinates: true,
      show_item_info: true,
      show_dimensions: false,
      show_classes: false,
      color_scheme: GridDebugColors::default(),
      interactive: false,
    }
  }
}

impl Default for GridDebugColors {
  fn default() -> Self {
    Self {
      grid_lines: hex("#4B5563").unwrap_or(ColorDefinition {
        r: 75,
        g: 85,
        b: 99,
      }), // Gray-600
      occupied_cell: hex("#DBEAFE").unwrap_or(ColorDefinition {
        r: 219,
        g: 234,
        b: 254,
      }), // Blue-100
      empty_cell: hex("#F9FAFB").unwrap_or(ColorDefinition {
        r: 249,
        g: 250,
        b: 251,
      }), // Gray-50
      gap_color: hex("#FEF3C7").unwrap_or(ColorDefinition {
        r: 254,
        g: 243,
        b: 199,
      }), // Yellow-100
      item_border: hex("#3B82F6").unwrap_or(ColorDefinition {
        r: 59,
        g: 130,
        b: 246,
      }), // Blue-500
      coordinate_text: hex("#6B7280").unwrap_or(ColorDefinition {
        r: 107,
        g: 114,
        b: 128,
      }), // Gray-500
      info_text: hex("#374151").unwrap_or(ColorDefinition {
        r: 55,
        g: 65,
        b: 81,
      }), // Gray-700
    }
  }
}

impl GridDebugger {
  /// Create a new grid debugger with default configuration
  pub fn new() -> Self {
    Self {
      config: GridDebugConfig::default(),
      _utility_processor: UtilityProcessor::new(),
    }
  }

  /// Create a grid debugger with custom configuration
  pub fn with_config(config: GridDebugConfig) -> Self {
    Self {
      config,
      _utility_processor: UtilityProcessor::new(),
    }
  }

  /// Set debug mode
  pub fn set_mode(&mut self, mode: GridDebugMode) {
    self.config.mode = mode;
  }

  /// Enable or disable coordinate display
  pub fn show_coordinates(&mut self, show: bool) {
    self.config.show_coordinates = show;
  }

  /// Enable or disable item info display
  pub fn show_item_info(&mut self, show: bool) {
    self.config.show_item_info = show;
  }

  /// Generate comprehensive inspection report for a grid layout
  pub fn inspect_grid(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<GridInspectionReport, crate::error::TuiError> {
    let start_time = std::time::Instant::now();

    // Parse grid configuration
    let grid_layout = GridLayout::new();
    let grid_config = grid_layout.parse_grid_config(element);

    // Compute layout
    let layout = grid_layout.compute_layout(element, container_rect)?;

    // Analyze placement
    let items = self.analyze_item_placements(element, &layout);
    let stats = self.calculate_statistics(&grid_config, &items, container_rect);
    let warnings = self.detect_warnings(&grid_config, &items, &stats);

    let elapsed = start_time.elapsed();
    let performance = GridPerformanceMetrics {
      layout_time_us: elapsed.as_micros() as u64,
      placement_iterations: items.len(), // Simplified metric
      memory_usage: self.estimate_memory_usage(&items),
      complexity_score: self.calculate_complexity_score(&grid_config, &items),
    };

    Ok(GridInspectionReport {
      config: self.create_config_summary(&grid_config, container_rect),
      items,
      stats,
      warnings,
      performance,
    })
  }

  /// Render grid debug visualization
  pub fn render_debug_overlay(&self, layout: &Layout, container_rect: LayoutRect) -> String {
    match self.config.mode {
      GridDebugMode::GridLines => self.render_grid_lines(layout, container_rect),
      GridDebugMode::ItemPlacement => self.render_item_placement(layout, container_rect),
      GridDebugMode::Spacing => self.render_spacing(layout, container_rect),
      GridDebugMode::Occupancy => self.render_occupancy(layout, container_rect),
      GridDebugMode::Full => self.render_full_debug(layout, container_rect),
      GridDebugMode::Minimal => self.render_minimal_debug(layout, container_rect),
    }
  }

  /// Create interactive grid inspector
  pub fn create_interactive_inspector(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<String, crate::error::TuiError> {
    let report = self.inspect_grid(element, container_rect)?;

    let mut output = String::new();

    // Header
    writeln!(
      output,
      "╭─────────────────────────────────────────────────────────────────╮"
    )
    .unwrap();
    writeln!(
      output,
      "│                    GRID LAYOUT INSPECTOR                        │"
    )
    .unwrap();
    writeln!(
      output,
      "╰─────────────────────────────────────────────────────────────────╯"
    )
    .unwrap();
    writeln!(output).unwrap();

    // Configuration summary
    writeln!(output, "📊 GRID CONFIGURATION").unwrap();
    writeln!(output, "├─ Columns: {}", report.config.columns).unwrap();
    writeln!(output, "├─ Rows: {}", report.config.rows).unwrap();
    writeln!(output, "├─ Gap: {}", report.config.gap).unwrap();
    writeln!(output, "├─ Flow: {}", report.config.flow).unwrap();
    writeln!(
      output,
      "├─ Container: {}×{}",
      report.config.container_size.0, report.config.container_size.1
    )
    .unwrap();
    writeln!(
      output,
      "└─ Cell Size: {}×{}",
      report.config.cell_size.0, report.config.cell_size.1
    )
    .unwrap();
    writeln!(output).unwrap();

    // Statistics
    writeln!(output, "📈 GRID STATISTICS").unwrap();
    writeln!(output, "├─ Total Cells: {}", report.stats.total_cells).unwrap();
    writeln!(
      output,
      "├─ Occupied: {} ({:.1}%)",
      report.stats.occupied_cells, report.stats.occupancy_rate
    )
    .unwrap();
    writeln!(output, "├─ Empty: {}", report.stats.empty_cells).unwrap();
    writeln!(
      output,
      "├─ Avg Item Span: {:.1}",
      report.stats.avg_item_span
    )
    .unwrap();
    writeln!(
      output,
      "└─ Efficiency: {:.1}/100",
      report.stats.efficiency_score
    )
    .unwrap();
    writeln!(output).unwrap();

    // Items
    if !report.items.is_empty() {
      writeln!(output, "🔍 GRID ITEMS ({} total)", report.items.len()).unwrap();
      for (i, item) in report.items.iter().enumerate() {
        let placement_icon = match item.placement_type {
          PlacementType::Auto => "🔄",
          PlacementType::Explicit => "📍",
          PlacementType::Span => "📏",
        };

        writeln!(
          output,
          "├─ {} Item {} {}",
          placement_icon,
          i + 1,
          item
            .id
            .as_ref()
            .map(|id| format!("({id})"))
            .unwrap_or_default()
        )
        .unwrap();
        writeln!(
          output,
          "│  ├─ Position: ({}, {})",
          item.position.0, item.position.1
        )
        .unwrap();
        writeln!(
          output,
          "│  ├─ Span: {} cols × {} rows",
          item.span.0, item.span.1
        )
        .unwrap();
        writeln!(
          output,
          "│  ├─ Size: {}×{}",
          item.rect.width, item.rect.height
        )
        .unwrap();

        if !item.classes.is_empty() {
          writeln!(output, "│  └─ Classes: {}", item.classes.join(", ")).unwrap();
        } else {
          writeln!(output, "│  └─ No grid classes").unwrap();
        }
      }
      writeln!(output).unwrap();
    }

    // Warnings
    if !report.warnings.is_empty() {
      writeln!(output, "⚠️  WARNINGS ({} issues)", report.warnings.len()).unwrap();
      for warning in &report.warnings {
        let icon = match warning.warning_type {
          GridWarningType::Overflow => "🔴",
          GridWarningType::LowEfficiency => "🟡",
          GridWarningType::Overlap => "🔺",
          GridWarningType::ExcessiveGaps => "📏",
          GridWarningType::ContentTruncation => "✂️",
          GridWarningType::Performance => "⏱️",
        };

        writeln!(output, "├─ {} {}", icon, warning.message).unwrap();
        if let Some(suggestion) = &warning.suggestion {
          writeln!(output, "│  └─ 💡 {suggestion}").unwrap();
        }
      }
      writeln!(output).unwrap();
    }

    // Performance
    writeln!(output, "⚡ PERFORMANCE").unwrap();
    writeln!(
      output,
      "├─ Layout Time: {}μs",
      report.performance.layout_time_us
    )
    .unwrap();
    writeln!(
      output,
      "├─ Iterations: {}",
      report.performance.placement_iterations
    )
    .unwrap();
    writeln!(
      output,
      "├─ Memory: ~{}KB",
      report.performance.memory_usage / 1024
    )
    .unwrap();
    writeln!(
      output,
      "└─ Complexity: {:.1}/10",
      report.performance.complexity_score
    )
    .unwrap();

    Ok(output)
  }

  /// Export grid debug information as JSON
  pub fn export_debug_json(
    &self,
    element: &Element,
    container_rect: LayoutRect,
  ) -> Result<String, crate::error::TuiError> {
    let report = self.inspect_grid(element, container_rect)?;
    serde_json::to_string_pretty(&report)
      .map_err(|e| crate::error::TuiError::component(format!("JSON export failed: {e}")))
  }

  /// Analyze item placements and generate detailed reports
  fn analyze_item_placements(&self, element: &Element, layout: &Layout) -> Vec<GridItemReport> {
    let grid_layout = GridLayout::new();
    let mut items = Vec::new();

    for (index, (child, child_layout)) in element
      .children
      .iter()
      .zip(layout.children.iter())
      .enumerate()
    {
      let item_config = grid_layout.parse_grid_item(child);

      // Determine placement type
      let placement_type = if item_config.column_start.is_some() || item_config.row_start.is_some()
      {
        PlacementType::Explicit
      } else if item_config.column_span > 1 || item_config.row_span > 1 {
        PlacementType::Span
      } else {
        PlacementType::Auto
      };

      // Calculate grid position from layout rect by reverse-engineering
      // from the computed layout positions and grid configuration
      let _grid_width = grid_layout.columns.len();
      let _grid_height = grid_layout.rows.len();

      // Calculate position based on the child's rect relative to grid origin
      let relative_x = child_layout.rect.x.saturating_sub(layout.rect.x);
      let relative_y = child_layout.rect.y.saturating_sub(layout.rect.y);

      // Find which column this x position corresponds to
      let mut col = 0;
      let mut accumulated_width = 0;
      for (i, &col_width) in grid_layout.columns.iter().enumerate() {
        if relative_x >= accumulated_width && relative_x < accumulated_width + col_width {
          col = i;
          break;
        }
        accumulated_width += col_width + grid_layout.gap;
      }

      // Find which row this y position corresponds to
      let mut row = 0;
      let mut accumulated_height = 0;
      for (i, &row_height) in grid_layout.rows.iter().enumerate() {
        if relative_y >= accumulated_height && relative_y < accumulated_height + row_height {
          row = i;
          break;
        }
        accumulated_height += row_height + grid_layout.gap;
      }

      let position = (col, row);

      items.push(GridItemReport {
        id: child.id.clone(),
        index,
        position,
        span: (
          item_config.column_span as usize,
          item_config.row_span as usize,
        ),
        rect: child_layout.rect,
        classes: child.classes.clone(),
        content: child.content.clone(),
        placement_type,
      });
    }

    items
  }

  /// Calculate grid statistics
  fn calculate_statistics(
    &self,
    grid_config: &Grid,
    items: &[GridItemReport],
    container_rect: LayoutRect,
  ) -> GridStatistics {
    // Determine grid dimensions
    let (columns, rows) = self.get_grid_dimensions(grid_config, items.len());
    let total_cells = columns * rows;

    // Calculate occupied cells
    let occupied_cells: usize = items.iter().map(|item| item.span.0 * item.span.1).sum();

    let empty_cells = total_cells.saturating_sub(occupied_cells);
    let occupancy_rate = if total_cells > 0 {
      (occupied_cells as f32 / total_cells as f32) * 100.0
    } else {
      0.0
    };

    // Calculate average item span
    let avg_item_span = if !items.is_empty() {
      items
        .iter()
        .map(|item| (item.span.0 * item.span.1) as f32)
        .sum::<f32>()
        / items.len() as f32
    } else {
      0.0
    };

    // Calculate efficiency score (considers occupancy and layout quality)
    let efficiency_score = self.calculate_efficiency_score(occupancy_rate, items, container_rect);

    GridStatistics {
      total_cells,
      occupied_cells,
      empty_cells,
      occupancy_rate,
      avg_item_span,
      efficiency_score,
    }
  }

  /// Detect potential issues and warnings
  fn detect_warnings(
    &self,
    grid_config: &Grid,
    items: &[GridItemReport],
    stats: &GridStatistics,
  ) -> Vec<GridWarning> {
    let mut warnings = Vec::new();

    // Check for low efficiency
    if stats.efficiency_score < 60.0 {
      warnings.push(GridWarning {
        warning_type: GridWarningType::LowEfficiency,
        message: format!(
          "Grid efficiency is low ({:.1}/100). Consider reducing gaps or adjusting item sizes.",
          stats.efficiency_score
        ),
        item_index: None,
        suggestion: Some(
          "Try using smaller gaps or better-sized items for the content.".to_string(),
        ),
      });
    }

    // Check for excessive gaps
    let _total_gap_area = match grid_config.gap.x as u16 + grid_config.gap.y as u16 {
      gap if gap > 6 => {
        warnings.push(GridWarning {
          warning_type: GridWarningType::ExcessiveGaps,
          message: format!(
            "Large gaps detected ({gap}). This may waste space on smaller terminals."
          ),
          item_index: None,
          suggestion: Some(
            "Consider using responsive gap classes or smaller gap values.".to_string(),
          ),
        });
        gap
      }
      gap => gap,
    };

    // Check individual items for issues
    for (index, item) in items.iter().enumerate() {
      // Check for very small items that might truncate content
      if item.rect.width < 10 || item.rect.height < 2 {
        warnings.push(GridWarning {
          warning_type: GridWarningType::ContentTruncation,
          message: format!(
            "Item {} is very small ({}×{}), content may be truncated.",
            index + 1,
            item.rect.width,
            item.rect.height
          ),
          item_index: Some(index),
          suggestion: Some(
            "Consider increasing item span or reducing number of columns.".to_string(),
          ),
        });
      }

      // Check for items with no content but taking up space
      if item.content.is_none() && item.span.0 * item.span.1 > 1 {
        warnings.push(GridWarning {
          warning_type: GridWarningType::LowEfficiency,
          message: format!(
            "Item {} spans {} cells but has no content.",
            index + 1,
            item.span.0 * item.span.1
          ),
          item_index: Some(index),
          suggestion: Some(
            "Consider reducing span or adding content to justify the space.".to_string(),
          ),
        });
      }
    }

    // Performance warnings
    if items.len() > 50 {
      warnings.push(GridWarning {
        warning_type: GridWarningType::Performance,
        message: format!(
          "Large number of grid items ({}). This may impact performance.",
          items.len()
        ),
        item_index: None,
        suggestion: Some("Consider pagination, virtualization, or grouping items.".to_string()),
      });
    }

    warnings
  }

  /// Render grid lines debug view
  fn render_grid_lines(&self, _layout: &Layout, container_rect: LayoutRect) -> String {
    let mut output = String::new();

    // Simple grid lines visualization
    let bg_color = color_to_ansi(self.config.color_scheme.occupied_cell, false);
    let line_color = color_to_ansi(self.config.color_scheme.grid_lines, true);

    // Top border
    output.push_str(&format!("{bg_color}{line_color}╭"));
    for _ in 0..container_rect.width.saturating_sub(2) {
      output.push('─');
    }
    output.push_str("╮\x1b[0m\n");

    // Grid content with coordinates if enabled
    for row in 0..container_rect.height.saturating_sub(2) {
      output.push_str(&format!("{bg_color}{line_color}│"));

      if self.config.show_coordinates && row == 0 {
        let coord_text = format!(" Grid {}×{} ", container_rect.width, container_rect.height);
        let padding = container_rect
          .width
          .saturating_sub(2)
          .saturating_sub(coord_text.len() as u16);
        output.push_str(&coord_text);
        for _ in 0..padding {
          output.push(' ');
        }
      } else {
        for _ in 0..container_rect.width.saturating_sub(2) {
          output.push(' ');
        }
      }

      output.push_str("│\x1b[0m\n");
    }

    // Bottom border
    output.push_str(&format!("{bg_color}{line_color}╰"));
    for _ in 0..container_rect.width.saturating_sub(2) {
      output.push('─');
    }
    output.push_str("╯\x1b[0m\n");

    output
  }

  /// Render item placement debug view
  fn render_item_placement(&self, layout: &Layout, _container_rect: LayoutRect) -> String {
    let mut output = String::new();

    for (i, child) in layout.children.iter().enumerate() {
      let item_color = color_to_ansi(self.config.color_scheme.item_border, false);
      let text_color = color_to_ansi(self.config.color_scheme.info_text, true);

      output.push_str(&format!(
        "{}{}[Item {}] {}×{} at ({},{})\x1b[0m\n",
        item_color,
        text_color,
        i + 1,
        child.rect.width,
        child.rect.height,
        child.rect.x,
        child.rect.y
      ));
    }

    output
  }

  /// Render spacing debug view
  fn render_spacing(&self, _layout: &Layout, container_rect: LayoutRect) -> String {
    let gap_color = color_to_ansi(self.config.color_scheme.gap_color, false);
    let text_color = color_to_ansi(self.config.color_scheme.info_text, true);

    format!(
      "{}{}[GAP VISUALIZATION] Container: {}×{}\x1b[0m\n",
      gap_color, text_color, container_rect.width, container_rect.height
    )
  }

  /// Render occupancy debug view
  fn render_occupancy(&self, layout: &Layout, _container_rect: LayoutRect) -> String {
    let occupied_color = color_to_ansi(self.config.color_scheme.occupied_cell, false);
    let empty_color = color_to_ansi(self.config.color_scheme.empty_cell, false);

    format!(
      "{}[OCCUPIED: {} items] {}[EMPTY SPACE]\x1b[0m\n",
      occupied_color,
      layout.children.len(),
      empty_color
    )
  }

  /// Render full debug view with all information
  fn render_full_debug(&self, layout: &Layout, container_rect: LayoutRect) -> String {
    let mut output = String::new();

    output.push_str(&self.render_grid_lines(layout, container_rect));
    output.push_str(&self.render_item_placement(layout, container_rect));
    output.push_str(&self.render_spacing(layout, container_rect));
    output.push_str(&self.render_occupancy(layout, container_rect));

    output
  }

  /// Render minimal debug overlay for production use
  fn render_minimal_debug(&self, layout: &Layout, container_rect: LayoutRect) -> String {
    let text_color = color_to_ansi(self.config.color_scheme.info_text, true);

    format!(
      "{}[Grid: {}×{}, {} items]\x1b[0m",
      text_color,
      container_rect.width,
      container_rect.height,
      layout.children.len()
    )
  }

  /// Helper methods for internal calculations
  fn get_grid_dimensions(&self, grid_config: &Grid, item_count: usize) -> (usize, usize) {
    use crate::layout::advanced_grid::{GridColumns, GridRows};

    let columns = match &grid_config.columns {
      GridColumns::Fixed(n) => *n as usize,
      GridColumns::Auto => ((item_count as f32).sqrt().ceil() as usize).max(1),
      _ => 3, // Default fallback
    };

    let rows = match &grid_config.rows {
      GridRows::Fixed(n) => *n as usize,
      GridRows::Auto => item_count.div_ceil(columns),
      _ => item_count.div_ceil(columns), // Default fallback
    };

    (columns, rows)
  }

  fn calculate_efficiency_score(
    &self,
    occupancy_rate: f32,
    items: &[GridItemReport],
    container_rect: LayoutRect,
  ) -> f32 {
    let mut score = occupancy_rate;

    // Bonus for well-sized items
    let avg_item_size = if !items.is_empty() {
      items
        .iter()
        .map(|item| item.rect.width as f32 * item.rect.height as f32)
        .sum::<f32>()
        / items.len() as f32
    } else {
      0.0
    };

    let container_size = container_rect.width as f32 * container_rect.height as f32;
    let ideal_item_size = container_size / items.len().max(1) as f32;

    // Efficiency bonus/penalty based on item sizing
    let size_ratio = if ideal_item_size > 0.0 {
      (avg_item_size / ideal_item_size).min(2.0)
    } else {
      1.0
    };

    if size_ratio > 0.8 && size_ratio < 1.2 {
      score += 10.0; // Bonus for well-sized items
    } else if !(0.5..=2.0).contains(&size_ratio) {
      score -= 15.0; // Penalty for poorly sized items
    }

    score.clamp(0.0, 100.0)
  }

  fn create_config_summary(
    &self,
    grid_config: &Grid,
    container_rect: LayoutRect,
  ) -> GridConfigSummary {
    use crate::layout::advanced_grid::{GridColumns, GridFlow, GridRows};

    let columns = match &grid_config.columns {
      GridColumns::Fixed(n) => format!("{n} fixed"),
      GridColumns::Auto => "auto".to_string(),
      GridColumns::Subgrid => "subgrid".to_string(),
      GridColumns::Custom(_) => "custom".to_string(),
    };

    let rows = match &grid_config.rows {
      GridRows::Fixed(n) => format!("{n} fixed"),
      GridRows::Auto => "auto".to_string(),
      GridRows::Subgrid => "subgrid".to_string(),
      GridRows::Custom(_) => "custom".to_string(),
    };

    let gap = format!("{}×{}", grid_config.gap.x, grid_config.gap.y);

    let flow = match grid_config.flow {
      GridFlow::Row => "row",
      GridFlow::Column => "column",
      GridFlow::RowDense => "row-dense",
      GridFlow::ColumnDense => "column-dense",
    }
    .to_string();

    // Calculate approximate cell size
    let (col_count, row_count) = self.get_grid_dimensions(grid_config, 1);
    let cell_width = container_rect.width / col_count.max(1) as u16;
    let cell_height = container_rect.height / row_count.max(1) as u16;

    GridConfigSummary {
      columns,
      rows,
      gap,
      flow,
      container_size: (container_rect.width, container_rect.height),
      cell_size: (cell_width, cell_height),
    }
  }

  fn estimate_memory_usage(&self, items: &[GridItemReport]) -> usize {
    // Rough estimate of memory usage
    std::mem::size_of_val(items)
      + items
        .iter()
        .map(|item| {
          item.classes.iter().map(|c| c.len()).sum::<usize>()
            + item.content.as_ref().map(|c| c.len()).unwrap_or(0)
            + item.id.as_ref().map(|id| id.len()).unwrap_or(0)
        })
        .sum::<usize>()
  }

  fn calculate_complexity_score(&self, grid_config: &Grid, items: &[GridItemReport]) -> f32 {
    let mut score = 1.0;

    // Base complexity from grid size
    let (cols, rows) = self.get_grid_dimensions(grid_config, items.len());
    score += (cols * rows) as f32 * 0.1;

    // Complexity from item spans
    let span_complexity: usize = items
      .iter()
      .map(|item| (item.span.0 - 1) + (item.span.1 - 1))
      .sum();
    score += span_complexity as f32 * 0.5;

    // Complexity from explicit positioning
    let explicit_items = items
      .iter()
      .filter(|item| item.placement_type == PlacementType::Explicit)
      .count();
    score += explicit_items as f32 * 0.3;

    score.min(10.0) // Cap at 10
  }
}

impl Default for GridDebugger {
  fn default() -> Self {
    Self::new()
  }
}

/// Convenience function to create a quick grid inspection
pub fn inspect_grid(
  element: &Element,
  container_rect: LayoutRect,
) -> Result<GridInspectionReport, crate::error::TuiError> {
  let debugger = GridDebugger::new();
  debugger.inspect_grid(element, container_rect)
}

/// Convenience function to create a grid debug overlay
pub fn debug_grid_overlay(
  layout: &Layout,
  container_rect: LayoutRect,
  mode: GridDebugMode,
) -> String {
  let config = GridDebugConfig {
    mode,
    ..Default::default()
  };
  let debugger = GridDebugger::with_config(config);
  debugger.render_debug_overlay(layout, container_rect)
}

/// Convenience function to create an interactive grid inspector
pub fn interactive_grid_inspector(
  element: &Element,
  container_rect: LayoutRect,
) -> Result<String, crate::error::TuiError> {
  let debugger = GridDebugger::new();
  debugger.create_interactive_inspector(element, container_rect)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::components::Element;

  #[test]
  fn test_grid_debugger_creation() {
    let debugger = GridDebugger::new();
    assert_eq!(debugger.config.mode, GridDebugMode::GridLines);
    assert!(debugger.config.show_coordinates);
  }

  #[test]
  fn test_debug_config_modes() {
    let mut debugger = GridDebugger::new();

    debugger.set_mode(GridDebugMode::ItemPlacement);
    assert_eq!(debugger.config.mode, GridDebugMode::ItemPlacement);

    debugger.show_coordinates(false);
    assert!(!debugger.config.show_coordinates);
  }

  #[test]
  fn test_grid_inspection() {
    let element = Element::with_tag("div")
      .class("grid")
      .class("grid-cols-2")
      .child(Element::with_tag("item").content("Test").build())
      .build();

    let container = LayoutRect {
      x: 0,
      y: 0,
      width: 100,
      height: 50,
    };
    let debugger = GridDebugger::new();

    let result = debugger.inspect_grid(&element, container);
    assert!(result.is_ok());

    let report = result.unwrap();
    assert!(!report.items.is_empty());
    assert!(report.stats.total_cells > 0);
  }

  #[test]
  fn test_debug_overlay_rendering() {
    let layout = Layout {
      rect: LayoutRect {
        x: 0,
        y: 0,
        width: 50,
        height: 20,
      },
      children: vec![Layout {
        rect: LayoutRect {
          x: 0,
          y: 0,
          width: 25,
          height: 20,
        },
        children: Vec::new(),
        element_id: Some("item1".to_string()),
        tag: "div".to_string(),
        content: Some("Item 1".to_string()),
        styles: crate::layout::ComputedStyles::default(),
        focused: false,
        focusable: false,
      }],
      element_id: None,
      tag: "grid".to_string(),
      content: None,
      styles: crate::layout::ComputedStyles::default(),
      focused: false,
      focusable: false,
    };

    let container = LayoutRect {
      x: 0,
      y: 0,
      width: 50,
      height: 20,
    };
    let debugger = GridDebugger::new();

    let overlay = debugger.render_debug_overlay(&layout, container);
    assert!(!overlay.is_empty());
    assert!(overlay.contains("Grid"));
  }

  #[test]
  fn test_convenience_functions() {
    let element = Element::with_tag("div")
      .class("grid")
      .class("grid-cols-1")
      .build();

    let container = LayoutRect {
      x: 0,
      y: 0,
      width: 50,
      height: 20,
    };

    // Test inspect_grid convenience function
    let report = inspect_grid(&element, container);
    assert!(report.is_ok());
  }
}
