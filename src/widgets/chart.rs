/*!
 * Chart Components - Data visualization widgets
 *
 * A comprehensive chart library providing:
 * - Bar charts (horizontal and vertical)
 * - Line charts with multiple series
 * - Pie charts with labels and percentages
 * - Scatter plots with customizable markers
 * - Real-time data updates and animations
 * - Responsive design and theming
 */

use crate::{
  error::Result,
  layout::LayoutRect,
  themes::{color_to_ansi, get_palette_color, ColorTheme},
};
use serde::{Deserialize, Serialize};

use std::fmt::Write;

/// Chart data point
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataPoint {
  pub x: f64,
  pub y: f64,
  pub label: Option<String>,
}

/// Chart data series
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataSeries {
  pub name: String,
  pub data: Vec<DataPoint>,
  pub color: String,
  pub style: SeriesStyle,
}

/// Series styling options
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeriesStyle {
  pub line_style: LineStyle,
  pub marker_style: MarkerStyle,
  pub fill_opacity: f32,
}

/// Line style for line charts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LineStyle {
  Solid,
  Dashed,
  Dotted,
  None,
}

/// Marker style for data points
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MarkerStyle {
  Circle,
  Square,
  Triangle,
  Diamond,
  Cross,
  None,
}

/// Chart type enumeration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChartType {
  Bar { horizontal: bool },
  Line { smooth: bool },
  Pie { donut: bool },
  Scatter,
}

/// Chart configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartConfig {
  pub chart_type: ChartType,
  pub title: Option<String>,
  pub x_axis: AxisConfig,
  pub y_axis: AxisConfig,
  pub legend: LegendConfig,
  pub grid: GridConfig,
  pub style: ChartStyle,
}

/// Axis configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AxisConfig {
  pub title: Option<String>,
  pub min: Option<f64>,
  pub max: Option<f64>,
  pub show_labels: bool,
  pub show_ticks: bool,
  pub tick_count: Option<usize>,
}

/// Legend configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LegendConfig {
  pub show: bool,
  pub position: LegendPosition,
  pub alignment: LegendAlignment,
}

/// Legend position
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendPosition {
  Top,
  Bottom,
  Left,
  Right,
}

/// Legend alignment
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LegendAlignment {
  Start,
  Center,
  End,
}

/// Grid configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GridConfig {
  pub show_x: bool,
  pub show_y: bool,
  pub color: String,
  pub style: GridStyle,
}

/// Grid line style
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GridStyle {
  Solid,
  Dashed,
  Dotted,
}

/// Chart styling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChartStyle {
  pub background: String,
  pub border_color: String,
  pub text_color: String,
  pub title_color: String,
  pub axis_color: String,
  pub padding: u16,
}

impl Default for ChartStyle {
  fn default() -> Self {
    Self {
      background: "transparent".to_string(),
      border_color: "#cccccc".to_string(),
      text_color: "#333333".to_string(),
      title_color: "#000000".to_string(),
      axis_color: "#666666".to_string(),
      padding: 2,
    }
  }
}

/// Main Chart widget
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chart {
  pub config: ChartConfig,
  pub series: Vec<DataSeries>,
  pub width: u16,
  pub height: u16,
}

impl Chart {
  /// Create a new chart
  pub fn new(chart_type: ChartType) -> Self {
    Self {
      config: ChartConfig {
        chart_type,
        title: None,
        x_axis: AxisConfig::default(),
        y_axis: AxisConfig::default(),
        legend: LegendConfig::default(),
        grid: GridConfig::default(),
        style: ChartStyle::default(),
      },
      series: Vec::new(),
      width: 80,
      height: 24,
    }
  }

  /// Add a data series
  pub fn add_series(&mut self, series: DataSeries) -> &mut Self {
    self.series.push(series);
    self
  }

  /// Set chart title
  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.config.title = Some(title.into());
    self
  }

  /// Set chart dimensions
  pub fn dimensions(mut self, width: u16, height: u16) -> Self {
    self.width = width;
    self.height = height;
    self
  }

  /// Calculate data bounds
  fn calculate_bounds(&self) -> (f64, f64, f64, f64) {
    let mut min_x = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_y = f64::NEG_INFINITY;

    for series in &self.series {
      for point in &series.data {
        min_x = min_x.min(point.x);
        max_x = max_x.max(point.x);
        min_y = min_y.min(point.y);
        max_y = max_y.max(point.y);
      }
    }

    // Apply axis overrides
    if let Some(x_min) = self.config.x_axis.min {
      min_x = x_min;
    }
    if let Some(x_max) = self.config.x_axis.max {
      max_x = x_max;
    }
    if let Some(y_min) = self.config.y_axis.min {
      min_y = y_min;
    }
    if let Some(y_max) = self.config.y_axis.max {
      max_y = y_max;
    }

    (min_x, max_x, min_y, max_y)
  }

  /// Render the chart
  pub fn render(&self, rect: LayoutRect, theme: &ColorTheme) -> Result<String> {
    let mut output = String::new();

    match self.config.chart_type {
      ChartType::Bar { horizontal } => {
        self.render_bar_chart(&mut output, rect, theme, horizontal)?;
      }
      ChartType::Line { smooth } => {
        self.render_line_chart(&mut output, rect, theme, smooth)?;
      }
      ChartType::Pie { donut } => {
        self.render_pie_chart(&mut output, rect, theme, donut)?;
      }
      ChartType::Scatter => {
        self.render_scatter_chart(&mut output, rect, theme)?;
      }
    }

    Ok(output)
  }

  /// Render bar chart
  fn render_bar_chart(
    &self,
    output: &mut String,
    rect: LayoutRect,
    theme: &ColorTheme,
    horizontal: bool,
  ) -> Result<()> {
    if self.series.is_empty() {
      return Ok(());
    }

    let (min_x, max_x, min_y, max_y) = self.calculate_bounds();
    let chart_area = self.get_chart_area(rect);

    // Render title
    if let Some(ref title) = self.config.title {
      let color_def = get_palette_color(&theme.palette, &self.config.style.title_color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let title_color = color_to_ansi(color_def, false);
      let title_x = rect.x + (rect.width - title.len() as u16) / 2;
      write!(output, "\x1b[{};{}H{}{}", rect.y + 1, title_x + 1, title_color, title)?;
    }

    // Render axes
    self.render_axes(output, chart_area, theme, min_x, max_x, min_y, max_y)?;

    // Render bars
    let series = &self.series[0]; // For simplicity, use first series
    let bar_width = if horizontal {
      chart_area.height / series.data.len().max(1) as u16
    } else {
      chart_area.width / series.data.len().max(1) as u16
    };

    let color_def = get_palette_color(&theme.palette, &series.color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let series_color = color_to_ansi(color_def, false);

    for (i, point) in series.data.iter().enumerate() {
      if horizontal {
        // Horizontal bars
        let bar_length = ((point.y - min_y) / (max_y - min_y) * chart_area.width as f64) as u16;
        let y = chart_area.y + i as u16 * bar_width;

        for row in 0..bar_width.min(1) {
          write!(output, "\x1b[{};{}H{}", y + row + 1, chart_area.x + 1, series_color)?;
          for _ in 0..bar_length {
            write!(output, "█")?;
          }
        }
      } else {
        // Vertical bars
        let bar_height = ((point.y - min_y) / (max_y - min_y) * chart_area.height as f64) as u16;
        let x = chart_area.x + i as u16 * bar_width;

        for col in 0..bar_width.min(2) {
          for row in 0..bar_height {
            let y = chart_area.y + chart_area.height - row - 1;
            write!(output, "\x1b[{};{}H{}█", y + 1, x + col + 1, series_color)?;
          }
        }
      }
    }

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render line chart
  fn render_line_chart(
    &self,
    output: &mut String,
    rect: LayoutRect,
    theme: &ColorTheme,
    _smooth: bool,
  ) -> Result<()> {
    if self.series.is_empty() {
      return Ok(());
    }

    let (min_x, max_x, min_y, max_y) = self.calculate_bounds();
    let chart_area = self.get_chart_area(rect);

    // Render title
    if let Some(ref title) = self.config.title {
      let color_def = get_palette_color(&theme.palette, &self.config.style.title_color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let title_color = color_to_ansi(color_def, false);
      let title_x = rect.x + (rect.width - title.len() as u16) / 2;
      write!(output, "\x1b[{};{}H{}{}", rect.y + 1, title_x + 1, title_color, title)?;
    }

    // Render axes
    self.render_axes(output, chart_area, theme, min_x, max_x, min_y, max_y)?;

    // Render lines for each series
    for series in &self.series {
      let color_def = get_palette_color(&theme.palette, &series.color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let series_color = color_to_ansi(color_def, false);

      for i in 0..series.data.len().saturating_sub(1) {
        let p1 = &series.data[i];
        let p2 = &series.data[i + 1];

        // Convert to screen coordinates
        let x1 = chart_area.x + ((p1.x - min_x) / (max_x - min_x) * chart_area.width as f64) as u16;
        let y1 = chart_area.y + chart_area.height - ((p1.y - min_y) / (max_y - min_y) * chart_area.height as f64) as u16;
        let x2 = chart_area.x + ((p2.x - min_x) / (max_x - min_x) * chart_area.width as f64) as u16;
        let y2 = chart_area.y + chart_area.height - ((p2.y - min_y) / (max_y - min_y) * chart_area.height as f64) as u16;

        // Draw line (simplified - just draw points for now)
        write!(output, "\x1b[{};{}H{}●", y1 + 1, x1 + 1, series_color)?;
        write!(output, "\x1b[{};{}H{}●", y2 + 1, x2 + 1, series_color)?;
      }
    }

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render pie chart
  fn render_pie_chart(
    &self,
    output: &mut String,
    rect: LayoutRect,
    theme: &ColorTheme,
    _donut: bool,
  ) -> Result<()> {
    if self.series.is_empty() {
      return Ok(());
    }

    // Render title
    if let Some(ref title) = self.config.title {
      let color_def = get_palette_color(&theme.palette, &self.config.style.title_color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let title_color = color_to_ansi(color_def, false);
      let title_x = rect.x + (rect.width - title.len() as u16) / 2;
      write!(output, "\x1b[{};{}H{}{}", rect.y + 1, title_x + 1, title_color, title)?;
    }

    // Calculate total for percentages
    let series = &self.series[0];
    let total: f64 = series.data.iter().map(|p| p.y).sum();

    // Render pie slices as text representation
    let center_x = rect.x + rect.width / 2;
    let center_y = rect.y + rect.height / 2;

    let color_def = get_palette_color(&theme.palette, &series.color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let series_color = color_to_ansi(color_def, false);

    for (i, point) in series.data.iter().enumerate() {
      let percentage = (point.y / total * 100.0) as u16;
      let label = point.label.as_deref().unwrap_or("Unknown");

      let y = center_y + i as u16 - series.data.len() as u16 / 2;
      write!(output, "\x1b[{};{}H{}● {} ({}%)",
             y + 1, center_x - 10, series_color, label, percentage)?;
    }

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Render scatter chart
  fn render_scatter_chart(
    &self,
    output: &mut String,
    rect: LayoutRect,
    theme: &ColorTheme,
  ) -> Result<()> {
    if self.series.is_empty() {
      return Ok(());
    }

    let (min_x, max_x, min_y, max_y) = self.calculate_bounds();
    let chart_area = self.get_chart_area(rect);

    // Render title
    if let Some(ref title) = self.config.title {
      let color_def = get_palette_color(&theme.palette, &self.config.style.title_color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let title_color = color_to_ansi(color_def, false);
      let title_x = rect.x + (rect.width - title.len() as u16) / 2;
      write!(output, "\x1b[{};{}H{}{}", rect.y + 1, title_x + 1, title_color, title)?;
    }

    // Render axes
    self.render_axes(output, chart_area, theme, min_x, max_x, min_y, max_y)?;

    // Render scatter points
    for series in &self.series {
      let color_def = get_palette_color(&theme.palette, &series.color)
        .map_err(|e| crate::error::TuiError::render(e))?;
      let series_color = color_to_ansi(color_def, false);

      for point in &series.data {
        let x = chart_area.x + ((point.x - min_x) / (max_x - min_x) * chart_area.width as f64) as u16;
        let y = chart_area.y + chart_area.height - ((point.y - min_y) / (max_y - min_y) * chart_area.height as f64) as u16;

        write!(output, "\x1b[{};{}H{}●", y + 1, x + 1, series_color)?;
      }
    }

    write!(output, "\x1b[0m")?;
    Ok(())
  }

  /// Get chart area (excluding title, axes, legend)
  fn get_chart_area(&self, rect: LayoutRect) -> LayoutRect {
    let title_height = if self.config.title.is_some() { 2 } else { 0 };
    let axis_margin = 3;

    LayoutRect {
      x: rect.x + axis_margin,
      y: rect.y + title_height,
      width: rect.width.saturating_sub(axis_margin * 2),
      height: rect.height.saturating_sub(title_height + axis_margin),
    }
  }

  /// Render chart axes
  fn render_axes(
    &self,
    output: &mut String,
    chart_area: LayoutRect,
    theme: &ColorTheme,
    min_x: f64,
    max_x: f64,
    min_y: f64,
    max_y: f64,
  ) -> Result<()> {
    let color_def = get_palette_color(&theme.palette, &self.config.style.axis_color)
      .map_err(|e| crate::error::TuiError::render(e))?;
    let axis_color = color_to_ansi(color_def, false);

    // Y-axis
    for y in 0..chart_area.height {
      write!(output, "\x1b[{};{}H{}│", chart_area.y + y + 1, chart_area.x, axis_color)?;
    }

    // X-axis
    for x in 0..chart_area.width {
      write!(output, "\x1b[{};{}H{}─", chart_area.y + chart_area.height + 1, chart_area.x + x + 1, axis_color)?;
    }

    // Corner
    write!(output, "\x1b[{};{}H{}└", chart_area.y + chart_area.height + 1, chart_area.x, axis_color)?;

    // Y-axis labels
    if self.config.y_axis.show_labels {
      let label_count = 5;
      for i in 0..=label_count {
        let value = min_y + (max_y - min_y) * i as f64 / label_count as f64;
        let y = chart_area.y + chart_area.height - (i * chart_area.height as usize / label_count) as u16;
        write!(output, "\x1b[{};{}H{}{:.1}", y + 1, chart_area.x - 3, axis_color, value)?;
      }
    }

    // X-axis labels
    if self.config.x_axis.show_labels {
      let label_count = 5;
      for i in 0..=label_count {
        let value = min_x + (max_x - min_x) * i as f64 / label_count as f64;
        let x = chart_area.x + (i * chart_area.width as usize / label_count) as u16;
        write!(output, "\x1b[{};{}H{}{:.1}", chart_area.y + chart_area.height + 2, x + 1, axis_color, value)?;
      }
    }

    Ok(())
  }
}

impl Default for AxisConfig {
  fn default() -> Self {
    Self {
      title: None,
      min: None,
      max: None,
      show_labels: true,
      show_ticks: true,
      tick_count: None,
    }
  }
}

impl Default for LegendConfig {
  fn default() -> Self {
    Self {
      show: true,
      position: LegendPosition::Right,
      alignment: LegendAlignment::Center,
    }
  }
}

impl Default for GridConfig {
  fn default() -> Self {
    Self {
      show_x: true,
      show_y: true,
      color: "#f0f0f0".to_string(),
      style: GridStyle::Solid,
    }
  }
}

impl Default for SeriesStyle {
  fn default() -> Self {
    Self {
      line_style: LineStyle::Solid,
      marker_style: MarkerStyle::Circle,
      fill_opacity: 0.3,
    }
  }
}

/// Chart builder for easy construction
pub struct ChartBuilder {
  chart: Chart,
}

impl ChartBuilder {
  pub fn new(chart_type: ChartType) -> Self {
    Self {
      chart: Chart::new(chart_type),
    }
  }

  pub fn title(mut self, title: impl Into<String>) -> Self {
    self.chart.config.title = Some(title.into());
    self
  }

  pub fn dimensions(mut self, width: u16, height: u16) -> Self {
    self.chart.width = width;
    self.chart.height = height;
    self
  }

  pub fn series(mut self, series: DataSeries) -> Self {
    self.chart.add_series(series);
    self
  }

  pub fn x_axis(mut self, config: AxisConfig) -> Self {
    self.chart.config.x_axis = config;
    self
  }

  pub fn y_axis(mut self, config: AxisConfig) -> Self {
    self.chart.config.y_axis = config;
    self
  }

  pub fn build(self) -> Chart {
    self.chart
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_chart_creation() {
    let chart = Chart::new(ChartType::Bar { horizontal: false });
    assert!(matches!(chart.config.chart_type, ChartType::Bar { horizontal: false }));
    assert!(chart.series.is_empty());
  }

  #[test]
  fn test_data_series() {
    let series = DataSeries {
      name: "Test Series".to_string(),
      data: vec![
        DataPoint { x: 1.0, y: 10.0, label: Some("A".to_string()) },
        DataPoint { x: 2.0, y: 20.0, label: Some("B".to_string()) },
        DataPoint { x: 3.0, y: 15.0, label: Some("C".to_string()) },
      ],
      color: "#0078d4".to_string(),
      style: SeriesStyle::default(),
    };

    assert_eq!(series.data.len(), 3);
    assert_eq!(series.data[0].y, 10.0);
  }

  #[test]
  fn test_chart_builder() {
    let chart = ChartBuilder::new(ChartType::Line { smooth: true })
      .title("Test Chart")
      .dimensions(100, 30)
      .build();

    assert_eq!(chart.config.title, Some("Test Chart".to_string()));
    assert_eq!(chart.width, 100);
    assert_eq!(chart.height, 30);
  }
}
