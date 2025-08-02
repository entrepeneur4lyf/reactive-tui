//! Advanced terminal rendering system with CSS support and double buffering

use crate::display::AdaptiveFpsManager;
use crate::error::{Result, TuiError};
use crate::layout::Layout;
pub mod borders;
pub use borders::{BorderPosition, BorderSet, BorderStyle};
use crossterm::{
    cursor::{Hide, MoveTo, Show},
    style::{
        Attribute, Color as CrosstermColor, Print, ResetColor, SetAttribute, SetBackgroundColor,
        SetForegroundColor,
    },
    terminal::{Clear, ClearType},
    Command,
};
use std::collections::HashMap;
use std::io::{self, Write};
use std::time::Instant;

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

#[derive(Debug, Clone, Default)]
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
        self.queue(Print(text))?;
        self.cursor_x += text.len() as u16; // Approximate - doesn't handle Unicode properly
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
}

impl Renderer {
    pub fn new() -> Result<Self> {
        let (width, height) = crossterm::terminal::size()
            .map_err(|e| TuiError::render(format!("Failed to get terminal size: {e}")))?;

        Ok(Self {
            width,
            height,
            style_cache: HashMap::new(),
            border_set: BorderSet::new(),
            frame_buffer: FrameBuffer::new(),
            fps_manager: None,
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
        self.fps_manager
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

    /// Get current target FPS
    pub fn get_target_fps(&self) -> Option<u32> {
        self.fps_manager.as_ref().map(|fps| fps.get_target_fps())
    }

    /// Get FPS manager capabilities summary
    pub fn get_fps_summary(&self) -> Option<String> {
        self.fps_manager
            .as_ref()
            .map(|fps| fps.get_recommendation_summary())
    }

    pub async fn render(&mut self, layout: &Layout) -> Result<()> {
        let frame_start = Instant::now();
        let mut stdout = io::stdout();

        // Clear frame buffer and prepare for new frame
        self.frame_buffer.clear();

        // Queue all rendering operations to buffer (no immediate output)
        self.frame_buffer.queue(Clear(ClearType::All))?;
        self.frame_buffer.queue(Hide)?;

        // Render the layout tree recursively into buffer
        let render_start = Instant::now();
        self.render_layout_to_buffer(layout)?;
        let render_time = render_start.elapsed();

        // Queue cursor show
        self.frame_buffer.queue(Show)?;

        // Atomic flush - single write operation to terminal
        self.frame_buffer.flush_to_stdout(&mut stdout)?;

        // Record performance metrics for adaptive FPS if enabled
        let total_frame_time = frame_start.elapsed();
        let target_duration = self
            .get_target_frame_duration()
            .unwrap_or(std::time::Duration::from_millis(16)); // Default 60fps
        let frame_dropped = total_frame_time > target_duration;
        self.record_frame_performance(total_frame_time, render_time, frame_dropped);

        Ok(())
    }

    fn render_layout_to_buffer(&mut self, layout: &Layout) -> Result<()> {
        // Apply styles from element if available
        if let Some(style) = self.get_element_style(layout) {
            self.frame_buffer.apply_style(&style)?;
        }

        // Render element content
        if let Some(content) = &layout.content {
            // Handle multi-line content
            let lines: Vec<&str> = content.lines().collect();
            for (i, line) in lines.iter().enumerate() {
                let y_pos = layout.rect.y + (i as u16);
                if y_pos < self.height {
                    // Optimized cursor movement
                    self.frame_buffer.move_to(layout.rect.x, y_pos)?;

                    // Truncate line if it exceeds width
                    let display_line = if line.len() > layout.rect.width as usize {
                        &line[..layout.rect.width as usize]
                    } else {
                        line
                    };

                    self.frame_buffer.print(display_line)?;
                }
            }
        }

        // Render children recursively
        for child in &layout.children {
            self.render_layout_to_buffer(child)?;
        }

        // Reset styles after rendering this element
        self.frame_buffer.queue(ResetColor)?;

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

            // Truncate line if it exceeds content width
            let display_line = if line.len() > content_width as usize {
                &line[..content_width as usize]
            } else {
                line
            };

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
        })
    }
}
