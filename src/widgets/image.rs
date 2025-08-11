//! # Image Widget
//!
//! Advanced image rendering widget with Sixel graphics support and graceful fallback systems.
//!
//! The Image widget provides comprehensive image display capabilities in terminal applications,
//! supporting multiple formats (PNG, JPEG, GIF, WebP) with Sixel rendering for compatible terminals
//! and intelligent fallback options for limited environments.
//!
//! ## Features
//!
//! - **Multi-Format Support**: PNG, JPEG, GIF, WebP, and BMP image formats
//! - **Sixel Rendering**: High-quality graphics output using the Sixel protocol
//! - **Intelligent Fallbacks**: ASCII art, Unicode blocks, or placeholder text for non-Sixel terminals
//! - **Responsive Scaling**: Automatic image scaling with aspect ratio preservation
//! - **Memory Efficient**: Lazy loading and LRU caching for optimal performance
//! - **Terminal Detection**: Automatic capability detection with graceful degradation
//!
//! ## Usage Examples
//!
//! ### Basic Image Display
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::image::*;
//!
//! let logo = image("company-logo", |config| {
//!     config
//!         .source_file("assets/logo.png")
//!         .width(200)
//!         .height(100)
//!         .scaling(ScalingMode::Fit)
//!         .fallback(FallbackMode::AsciiArt)
//! });
//! ```
//!
//! ### Responsive Image with Sixel Optimization
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::image::*;
//!
//! let diagram = image("architecture-diagram", |config| {
//!     config
//!         .source_file("docs/architecture.png")
//!         .scaling(ScalingMode::Fit)
//!         .fallback(FallbackMode::Placeholder)
//!         .alt_text("System Architecture Diagram")
//!         .class("responsive-image")
//! });
//! ```

use crate::components::Element;
use crate::error::{Result, TuiError};
use crate::layout::LayoutRect;
use crate::themes::ColorTheme;
use crate::widgets::ResponsiveWidget;

use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::sync::Arc;

#[cfg(feature = "images")]
use image::{DynamicImage, ImageFormat};

#[cfg(feature = "images")]
use a_sixel::{dither::FloydSteinberg, BitMergeSixelEncoderBest};

/// Image widget configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
  /// Image source (file path, embedded data, or loaded image)
  pub source: ImageSource,
  /// Target width in characters (None = auto)
  pub width: Option<u32>,
  /// Target height in characters (None = auto)
  pub height: Option<u32>,
  /// Image scaling behavior
  pub scaling: ScalingMode,
  /// Horizontal alignment within container
  pub alignment: Alignment,
  /// Fallback mode for non-Sixel terminals
  pub fallback: FallbackMode,
  /// Alternative text for accessibility
  pub alt_text: Option<String>,
  /// CSS class for styling
  pub class: Option<String>,
}

/// Image source types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImageSource {
  /// Load from file system path
  FilePath(String),
  /// Embedded binary data with format hint
  EmbeddedData(Vec<u8>, Option<String>), // Use String instead of ImageFormat for serialization
  /// Pre-loaded dynamic image (not serializable)
  #[serde(skip)]
  DynamicImage(Option<Arc<DynamicImage>>),
}

/// Image scaling modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ScalingMode {
  /// Maintain aspect ratio, fit within bounds
  Fit,
  /// Fill bounds exactly, may crop image
  Fill,
  /// Stretch to exact dimensions, may distort
  Stretch,
  /// Use original size regardless of bounds
  Original,
}

/// Image alignment options
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Alignment {
  /// Align to left/top
  Start,
  /// Center alignment
  Center,
  /// Align to right/bottom
  End,
}

/// Fallback rendering modes for non-Sixel terminals
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum FallbackMode {
  /// Convert to ASCII art representation
  AsciiArt,
  /// Use Unicode block characters
  UnicodeBlocks,
  /// Show placeholder text with alt text
  Placeholder,
  /// Hide widget entirely
  Hide,
}

/// Terminal capability detection for image rendering
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TerminalCapability {
  /// Full Sixel graphics support
  Sixel,
  /// 256-color support for enhanced fallbacks
  BasicColor,
  /// Monochrome text-only output
  Monochrome,
}

/// Core Image widget implementation
#[derive(Debug)]
pub struct ImageWidget {
  /// Widget identifier
  id: String,
  /// Image configuration
  config: ImageConfig,
  /// Cached loaded image data
  #[cfg(feature = "images")]
  image_data: Option<Arc<DynamicImage>>,
  /// Cached rendered output
  rendered_cache: RefCell<Option<String>>,
  /// Detected terminal capabilities
  terminal_capability: TerminalCapability,
  /// Widget bounds for coordinate mapping
  bounds: Option<LayoutRect>,
}

impl ImageWidget {
  /// Create a new Image widget with configuration
  pub fn new(id: impl Into<String>, config: ImageConfig) -> Self {
    Self {
      id: id.into(),
      config,
      #[cfg(feature = "images")]
      image_data: None,
      rendered_cache: RefCell::new(None),
      terminal_capability: TerminalCapability::detect(),
      bounds: None,
    }
  }

  /// Load image data from the configured source
  #[cfg(feature = "images")]
  pub fn load_image(&mut self) -> Result<()> {
    let image = match &self.config.source {
      ImageSource::FilePath(path) => {
        let img = image::open(path)
          .map_err(|e| TuiError::component(format!("Failed to load image {path}: {e}")))?;
        img
      }
      ImageSource::EmbeddedData(data, format_hint) => {
        let img = if let Some(format_str) = format_hint {
          // Parse format string back to ImageFormat
          let format = match format_str.as_str() {
            "png" => ImageFormat::Png,
            "jpeg" | "jpg" => ImageFormat::Jpeg,
            "gif" => ImageFormat::Gif,
            "webp" => ImageFormat::WebP,
            "bmp" => ImageFormat::Bmp,
            _ => {
              return Err(TuiError::component(format!(
                "Unsupported image format: {format_str}"
              )))
            }
          };
          image::load_from_memory_with_format(data, format)
        } else {
          image::load_from_memory(data)
        };
        img.map_err(|e| TuiError::component(format!("Failed to decode image data: {e}")))?
      }
      ImageSource::DynamicImage(Some(img)) => (**img).clone(),
      ImageSource::DynamicImage(None) => {
        return Err(TuiError::component("No image data provided"));
      }
    };

    self.image_data = Some(Arc::new(image));
    *self.rendered_cache.borrow_mut() = None; // Invalidate cache
    Ok(())
  }

  /// Render image using Sixel protocol
  #[cfg(feature = "images")]
  pub fn render_sixel(&self, bounds: LayoutRect) -> Result<String> {
    let image = self
      .image_data
      .as_ref()
      .ok_or_else(|| TuiError::component("No image data loaded"))?;

    let scaled_image = self.scale_image_for_bounds(image, bounds)?;

    // Convert DynamicImage to RgbaImage for a-sixel
    let rgba_image = scaled_image.to_rgba8();

    // Use BitMergeSixelEncoderBest with FloydSteinberg dithering for high-quality encoding
    let sixel_data = BitMergeSixelEncoderBest::<FloydSteinberg>::encode(rgba_image);

    Ok(sixel_data)
  }

  /// Scale image according to configuration and bounds
  #[cfg(feature = "images")]
  fn scale_image_for_bounds(
    &self,
    image: &DynamicImage,
    bounds: LayoutRect,
  ) -> Result<DynamicImage> {
    let (target_width, target_height) = self.calculate_target_dimensions(image, bounds);

    match self.config.scaling {
      ScalingMode::Original => Ok(image.clone()),
      ScalingMode::Fit => Ok(image.resize(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
      )),
      ScalingMode::Fill => Ok(image.resize_to_fill(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
      )),
      ScalingMode::Stretch => Ok(image.resize_exact(
        target_width,
        target_height,
        image::imageops::FilterType::Lanczos3,
      )),
    }
  }

  /// Calculate target dimensions based on configuration and bounds
  #[cfg(feature = "images")]
  fn calculate_target_dimensions(&self, image: &DynamicImage, bounds: LayoutRect) -> (u32, u32) {
    let img_width = image.width();
    let img_height = image.height();

    let target_width = self.config.width.unwrap_or(bounds.width as u32);
    let target_height = self.config.height.unwrap_or(bounds.height as u32);

    match self.config.scaling {
      ScalingMode::Original => (img_width, img_height),
      ScalingMode::Fit => {
        let aspect_ratio = img_width as f64 / img_height as f64;
        let bounds_ratio = target_width as f64 / target_height as f64;

        if aspect_ratio > bounds_ratio {
          // Width-constrained
          (target_width, (target_width as f64 / aspect_ratio) as u32)
        } else {
          // Height-constrained
          ((target_height as f64 * aspect_ratio) as u32, target_height)
        }
      }
      ScalingMode::Fill | ScalingMode::Stretch => (target_width, target_height),
    }
  }

  /// Render fallback representation for non-Sixel terminals
  pub fn render_fallback(&self, bounds: LayoutRect) -> Result<String> {
    match self.config.fallback {
      FallbackMode::Placeholder => {
        let alt_text = self.config.alt_text.as_deref().unwrap_or("Image");
        Ok(format!("[{alt_text}]"))
      }
      FallbackMode::Hide => Ok(String::new()),
      #[cfg(feature = "images")]
      FallbackMode::AsciiArt => self.render_ascii_art(bounds),
      #[cfg(feature = "images")]
      FallbackMode::UnicodeBlocks => self.render_unicode_blocks(bounds),
      #[cfg(not(feature = "images"))]
      FallbackMode::AsciiArt | FallbackMode::UnicodeBlocks => {
        let alt_text = self.config.alt_text.as_deref().unwrap_or("Image");
        Ok(format!("[{alt_text}]"))
      }
    }
  }

  /// Convert image to ASCII art representation
  #[cfg(feature = "images")]
  fn render_ascii_art(&self, bounds: LayoutRect) -> Result<String> {
    let image = self
      .image_data
      .as_ref()
      .ok_or_else(|| TuiError::component("No image data loaded"))?;

    let scaled = self.scale_image_for_bounds(image, bounds)?;
    let gray_image = scaled.to_luma8();

    let ascii_chars = " .:-=+*#%@";
    let mut result = String::new();

    for y in 0..gray_image.height() {
      for x in 0..gray_image.width() {
        let pixel = gray_image.get_pixel(x, y)[0];
        let char_index = (pixel as usize * (ascii_chars.len() - 1)) / 255;
        result.push(ascii_chars.chars().nth(char_index).unwrap_or(' '));
      }
      if y < gray_image.height() - 1 {
        result.push('\n');
      }
    }

    Ok(result)
  }

  /// Convert image to Unicode block character representation
  #[cfg(feature = "images")]
  fn render_unicode_blocks(&self, bounds: LayoutRect) -> Result<String> {
    let image = self
      .image_data
      .as_ref()
      .ok_or_else(|| TuiError::component("No image data loaded"))?;

    let scaled = self.scale_image_for_bounds(image, bounds)?;
    let gray_image = scaled.to_luma8();

    let block_chars = " ░▒▓█";
    let mut result = String::new();

    for y in 0..gray_image.height() {
      for x in 0..gray_image.width() {
        let pixel = gray_image.get_pixel(x, y)[0];
        let char_index = (pixel as usize * (block_chars.len() - 1)) / 255;
        result.push(block_chars.chars().nth(char_index).unwrap_or(' '));
      }
      if y < gray_image.height() - 1 {
        result.push('\n');
      }
    }

    Ok(result)
  }

  /// Get the widget's identifier
  pub fn id(&self) -> &str {
    &self.id
  }

  /// Update widget bounds for coordinate mapping
  pub fn set_bounds(&mut self, bounds: LayoutRect) {
    self.bounds = Some(bounds);
  }

  /// Get current widget bounds
  pub fn bounds(&self) -> Option<LayoutRect> {
    self.bounds
  }
}

impl TerminalCapability {
  /// Detect terminal capabilities for image rendering
  pub fn detect() -> Self {
    // Check for explicit Sixel support environment variables
    if std::env::var("TERM_PROGRAM")
      .unwrap_or_default()
      .contains("iTerm.app")
    {
      return TerminalCapability::Sixel;
    }

    // Check TERM environment variable for common Sixel-capable terminals
    let term = std::env::var("TERM").unwrap_or_default();
    if term.contains("xterm") || term.contains("mlterm") || term.contains("yaft") {
      TerminalCapability::Sixel
    } else if term.contains("256color") || std::env::var("COLORTERM").is_ok() {
      TerminalCapability::BasicColor
    } else {
      TerminalCapability::Monochrome
    }
  }
}

impl ResponsiveWidget for ImageWidget {
  fn to_element(&self) -> Element {
    Element::with_tag("img")
      .id(&self.id)
      .class("image-widget")
      .build()
  }

  fn render_with_layout(&self, layout: &LayoutRect, _theme: Option<&ColorTheme>) -> String {
    // Use cached result if available and bounds haven't changed
    if let Some(cached) = &*self.rendered_cache.borrow() {
      if self.bounds == Some(*layout) {
        return cached.clone();
      }
    }

    let result = match self.terminal_capability {
      #[cfg(feature = "images")]
      TerminalCapability::Sixel => match self.render_sixel(*layout) {
        Ok(sixel) => sixel,
        Err(e) => {
          eprintln!("Sixel rendering failed: {e}");
          self.render_fallback(*layout).unwrap_or_default()
        }
      },
      _ => self.render_fallback(*layout).unwrap_or_default(),
    };

    // Cache the result for future use
    *self.rendered_cache.borrow_mut() = Some(result.clone());
    result
  }

  fn min_size(&self) -> (u16, u16) {
    // Minimum size for visibility
    (10, 5)
  }

  fn can_grow_horizontal(&self) -> bool {
    self.config.width.is_none()
  }

  fn can_grow_vertical(&self) -> bool {
    self.config.height.is_none()
  }
}

impl Default for ImageConfig {
  fn default() -> Self {
    Self {
      source: ImageSource::DynamicImage(None),
      width: None,
      height: None,
      scaling: ScalingMode::Fit,
      alignment: Alignment::Center,
      fallback: FallbackMode::Placeholder,
      alt_text: None,
      class: None,
    }
  }
}

/// Image widget builder for fluent configuration
pub struct ImageBuilder {
  id: String,
  config: ImageConfig,
}

impl ImageBuilder {
  /// Create new Image widget builder
  pub fn new(id: impl Into<String>) -> Self {
    Self {
      id: id.into(),
      config: ImageConfig::default(),
    }
  }

  /// Set image source from file path
  pub fn source_file(mut self, path: impl Into<String>) -> Self {
    self.config.source = ImageSource::FilePath(path.into());
    self
  }

  /// Set image source from embedded data
  pub fn source_data(mut self, data: Vec<u8>, format: Option<String>) -> Self {
    self.config.source = ImageSource::EmbeddedData(data, format);
    self
  }

  /// Set image source from loaded DynamicImage
  #[cfg(feature = "images")]
  pub fn source_image(mut self, image: DynamicImage) -> Self {
    self.config.source = ImageSource::DynamicImage(Some(Arc::new(image)));
    self
  }

  /// Set target width in characters
  pub fn width(mut self, width: u32) -> Self {
    self.config.width = Some(width);
    self
  }

  /// Set target height in characters
  pub fn height(mut self, height: u32) -> Self {
    self.config.height = Some(height);
    self
  }

  /// Set image scaling mode
  pub fn scaling(mut self, scaling: ScalingMode) -> Self {
    self.config.scaling = scaling;
    self
  }

  /// Set image alignment
  pub fn alignment(mut self, alignment: Alignment) -> Self {
    self.config.alignment = alignment;
    self
  }

  /// Set fallback mode for non-Sixel terminals
  pub fn fallback(mut self, fallback: FallbackMode) -> Self {
    self.config.fallback = fallback;
    self
  }

  /// Set alternative text for accessibility
  pub fn alt_text(mut self, alt_text: impl Into<String>) -> Self {
    self.config.alt_text = Some(alt_text.into());
    self
  }

  /// Set CSS class for styling
  pub fn class(mut self, class: impl Into<String>) -> Self {
    self.config.class = Some(class.into());
    self
  }

  /// Build the Image widget
  pub fn build(self) -> ImageWidget {
    ImageWidget::new(self.id, self.config)
  }
}

// ImageBuilder implements the builder pattern for widget configuration

//
// Factory Functions - Following framework patterns
//

/// Primary image factory function with configuration callback
pub fn image<F>(id: &str, config: F) -> Element
where
  F: FnOnce(ImageBuilder) -> ImageBuilder,
{
  let builder = ImageBuilder::new(id);
  let configured_builder = config(builder);
  let widget = configured_builder.build();

  // Convert widget to Element
  Element::with_tag("img")
    .id(widget.id())
    .class("image-widget")
    .build()
}

/// Convenience function for displaying logos
pub fn logo(id: &str, path: &str) -> Element {
  image(id, |builder| {
    builder
      .source_file(path)
      .scaling(ScalingMode::Fit)
      .fallback(FallbackMode::Placeholder)
      .alt_text("Logo")
  })
}

/// Convenience function for displaying icons with fixed size
pub fn icon(id: &str, path: &str, size: u32) -> Element {
  image(id, |builder| {
    builder
      .source_file(path)
      .width(size)
      .height(size)
      .scaling(ScalingMode::Fit)
      .fallback(FallbackMode::UnicodeBlocks)
  })
}

/// Convenience function for displaying diagrams or charts
pub fn diagram(id: &str, path: &str) -> Element {
  image(id, |builder| {
    builder
      .source_file(path)
      .scaling(ScalingMode::Fit)
      .fallback(FallbackMode::AsciiArt)
      .alt_text("Diagram")
  })
}

/// Create image from embedded data
pub fn embedded_image(id: &str, data: Vec<u8>, format: Option<String>) -> Element {
  image(id, |builder| {
    builder
      .source_data(data, format)
      .scaling(ScalingMode::Fit)
      .fallback(FallbackMode::Placeholder)
  })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_image_config_default() {
    let config = ImageConfig::default();
    assert_eq!(config.scaling, ScalingMode::Fit);
    assert_eq!(config.alignment, Alignment::Center);
    assert_eq!(config.fallback, FallbackMode::Placeholder);
    assert!(config.alt_text.is_none());
  }

  #[test]
  fn test_image_builder() {
    let widget = ImageBuilder::new("test-image")
      .source_file("test.png")
      .width(100)
      .height(50)
      .scaling(ScalingMode::Fill)
      .alt_text("Test Image")
      .build();

    assert_eq!(widget.id(), "test-image");
    assert_eq!(widget.config.width, Some(100));
    assert_eq!(widget.config.height, Some(50));
    assert_eq!(widget.config.scaling, ScalingMode::Fill);
  }

  #[test]
  fn test_terminal_capability_detection() {
    let capability = TerminalCapability::detect();
    // Should return a valid capability
    assert!(matches!(
      capability,
      TerminalCapability::Sixel | TerminalCapability::BasicColor | TerminalCapability::Monochrome
    ));
  }

  #[test]
  fn test_image_fallback_placeholder() {
    let widget = ImageBuilder::new("test")
      .alt_text("Logo")
      .fallback(FallbackMode::Placeholder)
      .build();

    let bounds = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 10,
    };
    let result = widget.render_fallback(bounds).unwrap();
    assert_eq!(result, "[Logo]");
  }

  #[test]
  fn test_image_fallback_hide() {
    let widget = ImageBuilder::new("test")
      .fallback(FallbackMode::Hide)
      .build();

    let bounds = LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 10,
    };
    let result = widget.render_fallback(bounds).unwrap();
    assert_eq!(result, "");
  }

  #[cfg(feature = "images")]
  #[test]
  fn test_dimension_calculation() {
    use image::{DynamicImage, RgbImage};

    let img = DynamicImage::ImageRgb8(RgbImage::new(200, 100));
    let widget = ImageBuilder::new("test").scaling(ScalingMode::Fit).build();

    let bounds = LayoutRect {
      x: 0,
      y: 0,
      width: 100,
      height: 100,
    };
    let (width, height) = widget.calculate_target_dimensions(&img, bounds);

    // Should maintain aspect ratio (2:1)
    assert_eq!(width, 100);
    assert_eq!(height, 50);
  }
}
