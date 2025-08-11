//! Advanced terminal rendering system with CSS support and double buffering
mod diff;

// Helper for rect intersection used in clipping
fn intersect_rect(a: &LayoutRect, b: &LayoutRect) -> Option<LayoutRect> {
  let x1 = a.x.max(b.x);
  let y1 = a.y.max(b.y);
  let x2 = (a.x + a.width).min(b.x + b.width);
  let y2 = (a.y + a.height).min(b.y + b.height);
  if x2 > x1 && y2 > y1 {
    Some(LayoutRect {
      x: x1,
      y: y1,
      width: x2 - x1,
      height: y2 - y1,
    })
  } else {
    None
  }
}

use crate::display::AdaptiveFpsManager;
use crate::error::{Result, TuiError};
use crate::layout::Layout;
use crate::layout::LayoutRect;

pub mod borders;
pub use borders::{BorderPosition, BorderSet, BorderStyle};
#[cfg(not(target_family = "wasm"))]
use crossterm::{
  cursor::{Hide, MoveTo, Show},
  style::{
    Attribute, Color as CrosstermColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
    SetForegroundColor,
  },
  terminal::{Clear, ClearType},
  Command,
};

#[cfg(target_family = "wasm")]
use crate::compat::{
  Attribute, Clear, ClearType, Command, Hide, MoveTo, Print, ResetColor, SetAttribute,
  SetBackgroundColor, SetForegroundColor, Show,
};

#[cfg(target_family = "wasm")]
pub type CrosstermColor = crate::compat::Color;
use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;

use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

fn ansi_token_end(s: &str, start: usize) -> Option<usize> {
  let bytes = s.as_bytes();
  if start >= bytes.len() || bytes[start] != 0x1b {
    // ESC
    return None;
  }
  let len = bytes.len();
  if start + 1 >= len {
    return Some(len);
  }
  match bytes[start + 1] {
    b'[' => {
      // CSI: ESC [ ... final byte 0x40..=0x7E
      let mut j = start + 2;
      while j < len {
        let b = bytes[j];
        if (0x40..=0x7e).contains(&b) {
          return Some(j + 1);
        }
        j += 1;
      }
      Some(len)
    }
    b']' => {
      // OSC: ESC ] ... BEL (0x07) or ST (ESC \)
      let mut j = start + 2;
      while j < len {
        if bytes[j] == 0x07 {
          return Some(j + 1);
        }
        if bytes[j] == 0x1b && j + 1 < len && bytes[j + 1] == b'\\' {
          return Some(j + 2);
        }
        j += 1;
      }
      Some(len)
    }
    _ => {
      // Fallback: skip ESC and next byte
      Some((start + 2).min(len))
    }
  }
}

fn display_width(s: &str) -> usize {
  let mut i = 0usize;
  let mut w = 0usize;
  while i < s.len() {
    if let Some(end) = ansi_token_end(s, i) {
      i = end;
      continue;
    }
    if let Some((_, g)) = s[i..].grapheme_indices(true).next() {
      w += UnicodeWidthStr::width(g);
      i += g.len();
    } else {
      break;
    }
  }
  w
}

fn truncate_to_display_width(s: &str, max_cols: usize) -> &str {
  if max_cols == 0 {
    return "";
  }
  let mut i = 0usize;
  let mut cols = 0usize;
  let mut last_end = 0usize;
  while i < s.len() {
    if let Some(end) = ansi_token_end(s, i) {
      // Include ANSI sequences, zero-width
      i = end;
      last_end = i;
      continue;
    }
    if let Some((rel_idx, g)) = s[i..].grapheme_indices(true).next() {
      let gi = i + rel_idx; // should be i
      let gw = UnicodeWidthStr::width(g);
      if cols + gw > max_cols {
        break;
      }
      cols += gw;
      i = gi + g.len();
      last_end = i;
    } else {
      break;
    }
  }
  &s[..last_end]
}

/// Panel rendering configuration
#[derive(Debug, Clone)]
pub struct PanelConfig {
  pub x: u16,
  pub y: u16,
  pub width: u16,
  pub height: u16,
  pub title: Option<String>,
  pub content: String,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct RenderStyle {
  pub color: Option<CrosstermColor>,
  pub background: Option<CrosstermColor>,
  pub bold: bool,
  pub italic: bool,
  pub underline: bool,
}

/// Frame buffer for double-buffered rendering to eliminate flickering
#[derive(Debug, Clone)]
pub struct FrameBuffer {
  /// Buffer to collect all terminal operations before atomic flush
  buffer: Vec<u8>,
  /// Current cursor position for optimization
  cursor_x: u16,
  cursor_y: u16,
  /// Current style state for optimization
  current_style: RenderStyle,
}

impl Default for FrameBuffer {
  fn default() -> Self {
    Self::new()
  }
}

impl FrameBuffer {
  pub fn new() -> Self {
    Self {
      buffer: Vec::with_capacity(8192), // Pre-allocate reasonable size
      cursor_x: 0,
      cursor_y: 0,
      current_style: RenderStyle::default(),
    }
  }

  /// Clear the frame buffer
  pub fn clear(&mut self) {
    self.buffer.clear();
    self.cursor_x = 0;
    self.cursor_y = 0;
    self.current_style = RenderStyle::default();
  }

  /// Queue a command without immediate execution
  pub fn queue<T: Command>(&mut self, command: T) -> Result<()> {
    // Use a string buffer to collect ANSI sequences, then convert to bytes
    let mut ansi_string = String::new();
    command
      .write_ansi(&mut ansi_string)
      .map_err(|e| TuiError::render(format!("Failed to write ANSI command: {e}")))?;
    self.buffer.extend_from_slice(ansi_string.as_bytes());
    Ok(())
  }

  /// Optimized cursor movement that skips redundant moves
  pub fn move_to(&mut self, x: u16, y: u16) -> Result<()> {
    if self.cursor_x != x || self.cursor_y != y {
      self.queue(MoveTo(x, y))?;
      self.cursor_x = x;
      self.cursor_y = y;
    }
    Ok(())
  }

  /// Apply style changes only if they differ from current state
  pub fn apply_style(&mut self, style: &RenderStyle) -> Result<()> {
    // Only apply color if different
    if style.color != self.current_style.color {
      if let Some(color) = style.color {
        self.queue(SetForegroundColor(color))?;
      }
    }

    if style.background != self.current_style.background {
      if let Some(bg) = style.background {
        self.queue(SetBackgroundColor(bg))?;
      }
    }

    // Only apply attributes if they changed
    if style.bold != self.current_style.bold && style.bold {
      self.queue(SetAttribute(Attribute::Bold))?;
    }

    if style.italic != self.current_style.italic && style.italic {
      self.queue(SetAttribute(Attribute::Italic))?;
    }

    if style.underline != self.current_style.underline && style.underline {
      self.queue(SetAttribute(Attribute::Underlined))?;
    }

    self.current_style = style.clone();
    Ok(())
  }

  /// Print text at current cursor position
  pub fn print(&mut self, text: &str) -> Result<()> {
    self.queue(Print(text.to_string()))?;
    self.cursor_x = self.cursor_x.saturating_add(display_width(text) as u16);
    Ok(())
  }

  /// Flush the entire buffer to stdout in one atomic operation
  pub fn flush_to_stdout(&mut self, stdout: &mut impl Write) -> Result<()> {
    if !self.buffer.is_empty() {
      stdout.write_all(&self.buffer)?;
      stdout.flush()?;
      self.clear();
    }
    Ok(())
  }

  /// Flush buffer to a TUI driver for optimal performance
  pub fn flush_to_driver(&mut self, driver: &mut dyn crate::driver::Driver) -> Result<()> {
    if !self.buffer.is_empty() {
      driver.write_bytes(&self.buffer)?;
      driver.flush()?;
      self.clear();
    }
    Ok(())
  }

  /// Take the current buffered frame bytes and reset internal state
  pub fn take_bytes(&mut self) -> Vec<u8> {
    if self.buffer.is_empty() {
      return Vec::new();
    }
    let bytes = std::mem::take(&mut self.buffer);
    // Reset cursor/style state for the next frame
    self.cursor_x = 0;
    self.cursor_y = 0;
    self.current_style = RenderStyle::default();
    bytes
  }

  /// Get buffer size for debugging
  pub fn buffer_size(&self) -> usize {
    self.buffer.len()
  }
}

pub struct Renderer {
  width: u16,
  height: u16,
  style_cache: HashMap<String, RenderStyle>,
  border_set: BorderSet,
  /// Double buffer for flicker-free rendering
  frame_buffer: FrameBuffer,
  /// Optional adaptive FPS manager for intelligent refresh rate management
  fps_manager: Option<AdaptiveFpsManager>,
  /// Previous frame rows for line-diff rendering (bytes per row)
  last_diff_rows: Option<Vec<Vec<u8>>>,
  /// Optional interval to force a full repaint during diff mode (defensive reset)
  diff_full_repaint_interval: Option<usize>,
  /// Counter since last full repaint when diff mode is enabled
  diff_frames_since_full: usize,
}

impl Renderer {
  pub fn new() -> Result<Self> {
    let (width, height) = crate::compat::terminal::size()
      .map_err(|e| TuiError::render(format!("Failed to get terminal size: {e}")))?;

    Ok(Self {
      width,
      height,
      style_cache: HashMap::new(),
      border_set: BorderSet::new(),
      frame_buffer: FrameBuffer::new(),
      fps_manager: None,
      last_diff_rows: None,
      diff_full_repaint_interval: Some(300), // default: repaint every 300 diff frames (~5s @60fps)
      diff_frames_since_full: 0,
    })
  }

  /// Create renderer with adaptive FPS management
  pub async fn with_adaptive_fps() -> Result<Self> {
    let mut renderer = Self::new()?;
    renderer.fps_manager = Some(AdaptiveFpsManager::new().await?);
    Ok(renderer)
  }

  /// Enable adaptive FPS management
  pub async fn enable_adaptive_fps(&mut self) -> Result<()> {
    self.fps_manager = Some(AdaptiveFpsManager::new().await?);
    Ok(())
  }

  /// Get target frame duration from FPS manager
  pub fn get_target_frame_duration(&self) -> Option<std::time::Duration> {
    self
      .fps_manager
      .as_ref()
      .map(|fps| fps.get_frame_duration())
  }

  /// Record frame performance for adaptive adjustment
  pub fn record_frame_performance(
    &mut self,
    frame_time: std::time::Duration,
    render_time: std::time::Duration,
    dropped: bool,
  ) {
    if let Some(fps_manager) = &mut self.fps_manager {
      fps_manager.record_frame_performance(frame_time, render_time, dropped);
    }
  }
  /// Enable line-diff rendering. Subsequent frames will use minimal updates without full Clear.
  pub fn enable_diff_mode(&mut self) {
    if self.last_diff_rows.is_none() {
      self.last_diff_rows = Some(Vec::new());
    }
  }

  /// Disable line-diff rendering and clear diff state.
  pub fn disable_diff_mode(&mut self) {
    self.last_diff_rows = None;
  }

  /// Get current target FPS
  pub fn get_target_fps(&self) -> Option<u32> {
    self.fps_manager.as_ref().map(|fps| fps.get_target_fps())
  }

  /// Get FPS manager capabilities summary
  pub fn get_fps_summary(&self) -> Option<String> {
    self
      .fps_manager
      .as_ref()
      .map(|fps| fps.get_recommendation_summary())
  }

  pub async fn render(&mut self, layout: &Layout) -> Result<Vec<u8>> {
    let frame_start = Instant::now();

    // Clear frame buffer and prepare for new frame
    self.frame_buffer.clear();

    // Queue all rendering operations to buffer (no immediate output)
    self.frame_buffer.queue(Clear(ClearType::All))?;
    self.frame_buffer.queue(Hide)?;

    // Render the layout tree recursively into buffer
    let render_start = Instant::now();
    self.render_layout_to_buffer(layout, None)?;
    let render_time = render_start.elapsed();

    // Queue cursor show
    self.frame_buffer.queue(Show)?;

    // Return frame bytes to caller to route through driver
    let bytes = self.frame_buffer.take_bytes();

    // Record performance metrics for adaptive FPS if enabled
    let total_frame_time = frame_start.elapsed();
    let target_duration = self
      .get_target_frame_duration()
      .unwrap_or(std::time::Duration::from_millis(16)); // Default 60fps
    let frame_dropped = total_frame_time > target_duration;
    self.record_frame_performance(total_frame_time, render_time, frame_dropped);

    Ok(bytes)
  }

  /// Render with CSS component tree (proper per-element styling)
  pub async fn render_with_component_tree(
    &mut self,
    layout: &Layout,
    component_tree: &crate::css::ComponentTree,
  ) -> Result<Vec<u8>> {
    let frame_start = Instant::now();

    // Clear frame buffer and prepare for new frame
    self.frame_buffer.clear();

    // Queue all rendering operations to buffer (no immediate output)
    self.frame_buffer.queue(Clear(ClearType::All))?;
    self.frame_buffer.queue(Hide)?;

    // Render the layout tree recursively into buffer with component tree styles
    let render_start = Instant::now();
    self.render_layout_with_component_tree(layout, component_tree.root())?;
    let render_time = render_start.elapsed();

    // Queue cursor show
    self.frame_buffer.queue(Show)?;

    // Return frame bytes to caller to route through driver
    let bytes = self.frame_buffer.take_bytes();

    // Record performance metrics for adaptive FPS if enabled
    let total_frame_time = frame_start.elapsed();
    let target_duration = self
      .get_target_frame_duration()
      .unwrap_or(std::time::Duration::from_millis(16)); // Default 60fps
    let frame_dropped = total_frame_time > target_duration;
    self.record_frame_performance(total_frame_time, render_time, frame_dropped);

    Ok(bytes)
  }

  /// Render into an offscreen buffer without emitting Clear/Hide/Show control codes.
  /// Useful for benchmarks or preparations where terminal should not visibly flicker.
  pub async fn render_offscreen(&mut self, layout: &Layout) -> Result<Vec<u8>> {
    let frame_start = Instant::now();

    // Reset frame buffer but do not emit Clear/Hide/Show
    self.frame_buffer.clear();

    // Render layout into buffer
    let render_start = Instant::now();
    self.render_layout_to_buffer(layout, None)?;
    let render_time = render_start.elapsed();

    // Return bytes without terminal control codes
    let bytes = self.frame_buffer.take_bytes();

    // Record performance for adaptive FPS if enabled
    let total_frame_time = frame_start.elapsed();
    let target_duration = self
      .get_target_frame_duration()
      .unwrap_or(std::time::Duration::from_millis(16));
    let frame_dropped = total_frame_time > target_duration;
    self.record_frame_performance(total_frame_time, render_time, frame_dropped);

    Ok(bytes)
  }

  /// Render with CSS computed styles
  pub async fn render_with_styles(
    &mut self,
    layout: &Layout,
    css_styles: &crate::css::ComputedStyles,
  ) -> Result<Vec<u8>> {
    let frame_start = Instant::now();

    // Clear frame buffer and prepare for new frame
    self.frame_buffer.clear();

    // Queue all rendering operations to buffer (no immediate output)
    self.frame_buffer.queue(Clear(ClearType::All))?;
    self.frame_buffer.queue(Hide)?;

    // Render the layout tree recursively into buffer with CSS styles
    let render_start = Instant::now();
    self.render_layout_with_css_styles(layout, css_styles)?;
    let render_time = render_start.elapsed();

    // Queue cursor show
    self.frame_buffer.queue(Show)?;

    // Return frame bytes to caller to route through driver
    let bytes = self.frame_buffer.take_bytes();

    // Record performance metrics for adaptive FPS if enabled
    let total_frame_time = frame_start.elapsed();
    let target_duration = self
      .get_target_frame_duration()
      .unwrap_or(std::time::Duration::from_millis(16)); // Default 60fps
    let frame_dropped = total_frame_time > target_duration;
    self.record_frame_performance(total_frame_time, render_time, frame_dropped);

    Ok(bytes)
  }

  fn render_layout_to_buffer(
    &mut self,
    layout: &Layout,
    parent_clip: Option<LayoutRect>,
  ) -> Result<()> {
    // Compute clip rect from this element's overflow and the parent clip
    let element_clip_opt = match layout.styles.overflow {
      crate::layout::Overflow::Visible => None,
      crate::layout::Overflow::Hidden | crate::layout::Overflow::Clip => Some(layout.rect),
    };
    let content_clip: Option<LayoutRect> = match (parent_clip, element_clip_opt) {
      (Some(a), Some(b)) => intersect_rect(&a, &b),
      (Some(a), None) => Some(a),
      (None, Some(b)) => Some(b),
      (None, None) => None,
    };

    // Apply styles from element if available
    if let Some(style) = self.get_element_style(layout) {
      // Fill background if requested (respect clip)
      if let Some(bg) = style.background {
        self.render_background_at_clipped(
          layout.rect.x,
          layout.rect.y,
          layout.rect.width,
          layout.rect.height,
          bg,
          content_clip,
        )?;
      }
      self.frame_buffer.apply_style(&style)?;
    }

    // Render element content (respect clip)
    if let Some(content) = &layout.content {
      let lines: Vec<&str> = content.lines().collect();
      for (i, line) in lines.iter().enumerate() {
        let y_pos = layout.rect.y + (i as u16);
        if y_pos < self.height {
          self.print_clipped_line(&layout.rect, i as u16, line, content_clip)?;
        }
      }
    }

    // Render children recursively, propagating clip
    for child in &layout.children {
      self.render_layout_to_buffer(child, content_clip)?;
    }

    // Reset styles only if we actually changed something from default
    if self.frame_buffer.current_style != RenderStyle::default() {
      self.frame_buffer.queue(ResetColor)?;
      self.frame_buffer.current_style = RenderStyle::default();
    }

    Ok(())
  }

  fn render_layout_with_css_styles(
    &mut self,
    layout: &Layout,
    css_styles: &crate::css::ComputedStyles,
  ) -> Result<()> {
    // Convert CSS styles to render style and apply
    let render_style = css_styles.to_render_style();
    self.frame_buffer.apply_style(&render_style)?;

    // Render background if specified
    if let Some(bg_color) = css_styles.background_color {
      self.render_background_at(
        layout.rect.x,
        layout.rect.y,
        layout.rect.width,
        layout.rect.height,
        bg_color,
      )?;
    }

    // Render border if specified
    if css_styles.border_width > 0 {
      if let Some(border_color) = css_styles.border_color {
        self.render_border_with_color(
          layout.rect.x,
          layout.rect.y,
          layout.rect.width,
          layout.rect.height,
          border_color,
        )?;
      }
    }

    // Render element content with styles (no overflow in css::ComputedStyles currently)
    if let Some(content) = &layout.content {
      let lines: Vec<&str> = content.lines().collect();
      for (i, line) in lines.iter().enumerate() {
        let y_pos = layout.rect.y + (i as u16);
        if y_pos < self.height {
          self.frame_buffer.apply_style(&render_style)?;
          self.print_clipped_line(&layout.rect, i as u16, line, None)?;
        }
      }
    }

    // Render children recursively with inherited styles
    for child in &layout.children {
      self.render_layout_with_css_styles(child, css_styles)?;
    }

    // Reset styles only if we actually changed something from default
    if self.frame_buffer.current_style != RenderStyle::default() {
      self.frame_buffer.queue(ResetColor)?;
      self.frame_buffer.current_style = RenderStyle::default();
    }

    Ok(())
  }

  fn render_layout_with_component_tree(
    &mut self,
    layout: &Layout,
    component_node: &crate::css::ComponentNode,
  ) -> Result<()> {
    // Convert CSS styles to render style and apply
    let render_style = component_node.styles.to_render_style();
    self.frame_buffer.apply_style(&render_style)?;

    // Render background if specified (before content)
    if let Some(bg_color) = component_node.styles.background_color {
      self.render_background_at(
        layout.rect.x,
        layout.rect.y,
        layout.rect.width,
        layout.rect.height,
        bg_color,
      )?;
    }

    // Render element content
    if let Some(content) = &layout.content {
      let lines: Vec<&str> = content.lines().collect();
      for (line_idx, line) in lines.iter().enumerate() {
        let y_pos = layout.rect.y + line_idx as u16;
        if y_pos < layout.rect.y + layout.rect.height {
          self.print_clipped_line(&layout.rect, line_idx as u16, line, None)?;
        }
      }
    }

    // Render children recursively with their own styles
    for (child_layout, child_node) in layout.children.iter().zip(component_node.children.iter()) {
      self.render_layout_with_component_tree(child_layout, child_node)?;
    }

    // Reset styles only if we actually changed something from default
    if self.frame_buffer.current_style != RenderStyle::default() {
      self.frame_buffer.queue(ResetColor)?;
      self.frame_buffer.current_style = RenderStyle::default();
    }

    Ok(())
  }

  fn get_element_style(&self, layout: &Layout) -> Option<RenderStyle> {
    // Check cache first
    if let Some(cached) = self.style_cache.get(&layout.tag) {
      let mut style = cached.clone();

      // Apply focus styling if element is focused
      if layout.focused {
        style.background = Some(CrosstermColor::Blue);
        style.color = Some(CrosstermColor::White);
        style.bold = true;
      }

      return Some(style);
    }

    // Generate default styles based on element tag
    let mut style = match layout.tag.as_str() {
      "h1" | "h2" | "h3" => RenderStyle {
        color: Some(CrosstermColor::Cyan),
        bold: true,
        ..Default::default()
      },
      "error" => RenderStyle {
        color: Some(CrosstermColor::Red),
        bold: true,
        ..Default::default()
      },
      "success" => RenderStyle {
        color: Some(CrosstermColor::Green),
        ..Default::default()
      },
      "warning" => RenderStyle {
        color: Some(CrosstermColor::Yellow),
        ..Default::default()
      },
      "code" => RenderStyle {
        color: Some(CrosstermColor::Magenta),
        background: Some(CrosstermColor::DarkGrey),
        ..Default::default()
      },
      "button" | "input" => RenderStyle {
        color: Some(CrosstermColor::White),
        background: Some(CrosstermColor::DarkGrey),
        ..Default::default()
      },
      _ => RenderStyle::default(),
    };

    // Apply focus styling for focused elements
    if layout.focused {
      style.background = Some(CrosstermColor::Blue);
      style.color = Some(CrosstermColor::White);
      style.bold = true;
    }

    Some(style)
  }

  // Note: apply_style is now handled by FrameBuffer::apply_style for optimization

  pub async fn resize(&mut self, width: u16, height: u16) -> Result<()> {
    self.width = width;
    self.height = height;
    // Clear frame buffer on resize to ensure clean state
    self.frame_buffer.clear();
    Ok(())
  }

  pub fn set_style_for_tag(&mut self, tag: String, style: RenderStyle) {
    self.style_cache.insert(tag, style);
  }

  pub fn get_dimensions(&self) -> (u16, u16) {
    (self.width, self.height)
  }

  /// Get current frame buffer size for debugging
  pub fn get_buffer_size(&self) -> usize {
    self.frame_buffer.buffer_size()
  }

  /// Render background at specific position with color (row-buffered)
  fn render_background_at(
    &mut self,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: CrosstermColor,
  ) -> Result<()> {
    if width == 0 || height == 0 {
      return Ok(());
    }

    let max_rows = height.min(self.height.saturating_sub(y));
    if max_rows == 0 {
      return Ok(());
    }
    // Prepare one row worth of spaces
    let row_str = " ".repeat(width as usize);
    // Set BG color once before filling
    self.frame_buffer.queue(SetBackgroundColor(color))?;
    for row in 0..max_rows {
      self.frame_buffer.move_to(x, y + row)?;
      // Single print per row to minimize commands
      self.frame_buffer.queue(Print(row_str.as_str()))?;
    }
    Ok(())
  }

  /// Render border with specific color
  fn render_border_with_color(
    &mut self,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: CrosstermColor,
  ) -> Result<()> {
    if width < 2 || height < 2 {
      return Ok(()); // Too small for border
    }

    self.frame_buffer.queue(SetForegroundColor(color))?;

    // Top border
    self.frame_buffer.move_to(x, y)?;
    self.frame_buffer.print("┌")?;
    for _ in 1..width - 1 {
      self.frame_buffer.print("─")?;
    }
    self.frame_buffer.print("┐")?;

    // Side borders
    for row in 1..height - 1 {
      self.frame_buffer.move_to(x, y + row)?;
      self.frame_buffer.print("│")?;
      self.frame_buffer.move_to(x + width - 1, y + row)?;
      self.frame_buffer.print("│")?;
    }

    // Bottom border
    self.frame_buffer.move_to(x, y + height - 1)?;
    self.frame_buffer.print("└")?;
    for _ in 1..width - 1 {
      self.frame_buffer.print("─")?;
    }
    self.frame_buffer.print("┘")?;

    Ok(())
  }

  /// Render a border to frame buffer using Unicode box-drawing characters
  pub fn render_border(&mut self, x: u16, y: u16, width: u16, height: u16) -> Result<()> {
    self.render_border_with_style(x, y, width, height, BorderStyle::Light)
  }

  /// Render a border with specific style to frame buffer using Unicode box-drawing characters
  pub fn render_border_with_style(
    &mut self,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    style: BorderStyle,
  ) -> Result<()> {
    if width < 2 || height < 2 {
      return Ok(()); // Too small for border
    }

    // Get Unicode box-drawing characters from BorderSet
    let chars = self.border_set.get_chars(style);
    let top_left = chars.top_left;
    let top_right = chars.top_right;
    let bottom_left = chars.bottom_left;
    let bottom_right = chars.bottom_right;
    let horizontal = chars.horizontal;
    let vertical = chars.vertical;

    // Top border - single buffered operation
    self.frame_buffer.move_to(x, y)?;
    let mut top_border = String::with_capacity(width as usize);
    top_border.push(top_left);
    for _ in 1..width - 1 {
      top_border.push(horizontal);
    }
    top_border.push(top_right);
    self.frame_buffer.print(&top_border)?;

    // Side borders - batch operations

    for row in 1..height - 1 {
      // Left border
      self.frame_buffer.move_to(x, y + row)?;
      self.frame_buffer.print(&vertical.to_string())?;

      // Right border
      self.frame_buffer.move_to(x + width - 1, y + row)?;
      self.frame_buffer.print(&vertical.to_string())?;
    }

    // Bottom border - single buffered operation
    self.frame_buffer.move_to(x, y + height - 1)?;
    let mut bottom_border = String::with_capacity(width as usize);
    bottom_border.push(bottom_left);
    for _ in 1..width - 1 {
      bottom_border.push(horizontal);
    }
    bottom_border.push(bottom_right);
    self.frame_buffer.print(&bottom_border)?;

    Ok(())
  }

  /// Render a panel with border to frame buffer
  pub fn render_panel(&mut self, config: &PanelConfig) -> Result<()> {
    // Render border to buffer
    self.render_border(config.x, config.y, config.width, config.height)?;

    // Render title if provided
    if let Some(ref title) = config.title {
      let title_x = config.x + 2;
      let title_y = config.y;
      self.frame_buffer.move_to(title_x, title_y)?;
      self.frame_buffer.print(&format!(" {title} "))?;
    }

    // Render content inside border
    let content_x = config.x + 1;
    let content_y = config.y + 1;
    let content_width = config.width.saturating_sub(2);

    let content_height = config.height.saturating_sub(2);

    let lines: Vec<&str> = config.content.lines().collect();
    for (i, line) in lines.iter().enumerate() {
      if i >= content_height as usize {
        break;
      }

      self.frame_buffer.move_to(content_x, content_y + i as u16)?;

      // Truncate line by display columns (Unicode-aware)
      let display_line = truncate_to_display_width(line, content_width as usize);

      self.frame_buffer.print(display_line)?;
    }

    Ok(())
  }
}

impl Default for Renderer {
  fn default() -> Self {
    Self::new().unwrap_or(Self {
      width: 132,
      height: 43,
      style_cache: HashMap::new(),
      border_set: BorderSet::new(),
      frame_buffer: FrameBuffer::new(),
      fps_manager: None,
      last_diff_rows: None,
      diff_full_repaint_interval: Some(300),
      diff_frames_since_full: 0,
    })
  }
}

impl Renderer {
  fn render_background_at_clipped(
    &mut self,
    x: u16,
    y: u16,
    width: u16,
    height: u16,
    color: CrosstermColor,
    clip: Option<LayoutRect>,
  ) -> Result<()> {
    if width == 0 || height == 0 {
      return Ok(());
    }
    let fill_rect = LayoutRect {
      x,
      y,
      width,
      height,
    };
    let actual = if let Some(c) = clip {
      if let Some(r) = intersect_rect(&fill_rect, &c) {
        r
      } else {
        return Ok(());
      }
    } else {
      fill_rect
    };
    let max_rows = actual.height.min(self.height.saturating_sub(actual.y));
    if max_rows == 0 {
      return Ok(());
    }
    let row_str = " ".repeat(actual.width as usize);
    self.frame_buffer.queue(SetBackgroundColor(color))?;
    for row in 0..max_rows {
      self.frame_buffer.move_to(actual.x, actual.y + row)?;
      self.frame_buffer.queue(Print(row_str.as_str()))?;
    }
    Ok(())
  }

  /// Print a line of text clipped to a given rect (x,y,width,height).
  /// If clip is Some(rect), we intersect with it; otherwise, use rect as clipping bounds.
  fn print_clipped_line(
    &mut self,
    rect: &LayoutRect,
    line_index: u16,
    text: &str,
    clip: Option<LayoutRect>,
  ) -> Result<()> {
    let clip_rect = clip.unwrap_or(*rect);
    // Compute y; if out of clip, skip
    let y = rect.y + line_index;
    if y < clip_rect.y || y >= clip_rect.y + clip_rect.height {
      return Ok(());
    }

    // Horizontal clipping: start/end columns relative to rect
    let start_x = clip_rect.x.max(rect.x);
    let end_x = (clip_rect.x + clip_rect.width).min(rect.x + rect.width);
    if end_x <= start_x {
      return Ok(());
    }

    // Compute visible slice by display columns (ANSI/grapheme-aware)
    let left_cols = start_x.saturating_sub(rect.x) as usize;
    let visible_cols = end_x.saturating_sub(start_x) as usize;

    let (visible, _s, _e) =
      crate::widgets::input_unicode::visible_slice_by_width(text, left_cols, visible_cols);

    // Move and print
    self.frame_buffer.move_to(start_x, y)?;
    self.frame_buffer.print(visible)?;
    Ok(())
  }
}
