# Image Widget

Advanced image rendering widget with Sixel graphics support and graceful fallback systems.

## Overview

The Image widget provides comprehensive image display capabilities in terminal applications, supporting multiple formats with Sixel rendering for compatible terminals and intelligent fallback options for limited environments.

## Features

- **Multi-Format Support**: PNG, JPEG, GIF, WebP, and BMP image formats
- **Sixel Rendering**: High-quality graphics output using the Sixel protocol
- **Intelligent Fallbacks**: ASCII art, Unicode blocks, or placeholder text for non-Sixel terminals
- **Responsive Scaling**: Automatic image scaling with aspect ratio preservation
- **Memory Efficient**: Lazy loading and LRU caching for optimal performance
- **Terminal Detection**: Automatic capability detection with graceful degradation

## Basic Usage

### Simple Image Display

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

let logo = image("company-logo", |builder| {
    builder
        .source_file("assets/logo.png")
        .width(200)
        .height(100)
        .scaling(ScalingMode::Fit)
        .fallback(FallbackMode::AsciiArt)
});
```

### Factory Functions

The Image widget provides several convenience factory functions:

```rust
// Logo with automatic sizing
let logo = logo("header-logo", "assets/logo.png");

// Fixed-size icon
let icon = icon("settings-icon", "icons/settings.png", 32);

// Diagram with ASCII fallback
let diagram = diagram("architecture", "docs/architecture.png");

// Embedded image data
let embedded = embedded_image("avatar", image_bytes, Some("png".to_string()));
```

## Configuration

### ImageConfig

```rust
pub struct ImageConfig {
    pub source: ImageSource,           // Image source (file, data, or loaded)
    pub width: Option<u32>,            // Target width in characters
    pub height: Option<u32>,           // Target height in characters
    pub scaling: ScalingMode,          // Image scaling behavior
    pub alignment: Alignment,          // Horizontal alignment
    pub fallback: FallbackMode,        // Fallback for non-Sixel terminals
    pub alt_text: Option<String>,      // Alternative text for accessibility
}
```

### Image Sources

```rust
pub enum ImageSource {
    FilePath(String),                           // Load from file system
    EmbeddedData(Vec<u8>, Option<String>),     // Binary data with format hint
    DynamicImage(Option<Arc<DynamicImage>>),   // Pre-loaded image
}
```

### Scaling Modes

```rust
pub enum ScalingMode {
    Fit,        // Maintain aspect ratio, fit within bounds
    Fill,       // Fill bounds exactly, may crop image
    Stretch,    // Stretch to exact dimensions, may distort
    Original,   // Use original size regardless of bounds
}
```

### Fallback Modes

```rust
pub enum FallbackMode {
    AsciiArt,       // Convert to ASCII art representation
    UnicodeBlocks,  // Use Unicode block characters
    Placeholder,    // Show placeholder text with alt text
    Hide,           // Hide widget entirely
}
```

## Advanced Usage

### Custom Configuration

```rust
let custom_image = image("custom", |builder| {
    builder
        .source_file("photo.jpg")
        .width(150)
        .height(100)
        .scaling(ScalingMode::Fill)
        .alignment(Alignment::Center)
        .fallback(FallbackMode::UnicodeBlocks)
        .alt_text("Profile photo")
        .class("profile-image")
});
```

### Loading from Binary Data

```rust
let image_data: Vec<u8> = load_image_bytes();
let data_image = image("data-image", |builder| {
    builder
        .source_data(image_data, Some("jpeg".to_string()))
        .scaling(ScalingMode::Fit)
        .fallback(FallbackMode::Placeholder)
});
```

### Responsive Image

```rust
let responsive = image("hero-image", |builder| {
    builder
        .source_file("hero.png")
        .scaling(ScalingMode::Fit)
        .fallback(FallbackMode::AsciiArt)
        .class("hero-image")
        .class("w-full")  // CSS utility class for full width
});
```

## Terminal Compatibility

### Sixel Support Detection

The widget automatically detects terminal capabilities:

```rust
pub enum TerminalCapability {
    Sixel,      // Full Sixel graphics support
    BasicColor, // 256-color support for enhanced fallbacks
    Monochrome, // Monochrome text-only output
}
```

### Supported Terminals

**Sixel Compatible:**
- xterm (with Sixel support compiled)
- iTerm2 (macOS)
- mintty (Windows)
- mlterm
- RLogin

**Fallback Support:**
- All major terminal emulators
- Graceful degradation to text representations

## Feature Flag

The Image widget requires the `images` feature flag:

```toml
[dependencies]
reactive-tui = { version = "0.0.5", features = ["images"] }
```

## Implementation Details

### Memory Management

- **Lazy Loading**: Images loaded only when needed
- **Arc Caching**: Shared references to loaded image data
- **Render Caching**: Cached Sixel output for repeated renders

### Performance

- **Efficient Scaling**: High-quality Lanczos3 filtering
- **Optimized Encoding**: BitMergeSixelEncoder with FloydSteinberg dithering
- **Bounds Checking**: Only re-render when layout changes

### Error Handling

All image operations return `Result<T, TuiError>`:

```rust
let mut widget = ImageWidget::new("test", config);
match widget.load_image() {
    Ok(()) => println!("Image loaded successfully"),
    Err(e) => eprintln!("Failed to load image: {}", e),
}
```

## CSS Integration

The Image widget supports standard CSS classes:

```rust
let styled_image = image("styled", |builder| {
    builder
        .source_file("image.png")
        .class("border")
        .class("rounded")
        .class("shadow")
});
```

### Generated CSS Classes

- `image-widget` - Base widget class
- `image-scaling-{mode}` - Scaling mode class
- `image-align-{alignment}` - Alignment class
- `image-fallback-{mode}` - Fallback mode class

## Accessibility

### Alternative Text

Always provide alt text for screen readers:

```rust
let accessible_image = image("chart", |builder| {
    builder
        .source_file("sales-chart.png")
        .alt_text("Q4 sales performance showing 15% growth")
});
```

### Keyboard Navigation

Images are focusable when part of interactive elements:

```rust
let clickable_image = image("clickable", |builder| {
    builder
        .source_file("button.png")
        .alt_text("Submit button")
        .class("focusable")
});
```

## Examples

### Logo in Header

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

let header = header()
    .child(logo("app-logo", "assets/logo.png"))
    .child(text("My Application"))
    .class("header");
```

### Image Gallery

```rust
let gallery = flex_row()
    .child(image("img1", |b| b.source_file("1.jpg").width(100)))
    .child(image("img2", |b| b.source_file("2.jpg").width(100)))
    .child(image("img3", |b| b.source_file("3.jpg").width(100)))
    .class("gallery");
```

### Adaptive Image

```rust
let adaptive = image("adaptive", |builder| {
    builder
        .source_file("chart.png")
        .scaling(ScalingMode::Fit)
        .fallback(FallbackMode::AsciiArt)
        .alt_text("Performance metrics chart")
        .class("responsive-image")
});
```

## Related

- [Layout System](../layout.md) - For responsive image placement
- [CSS Engine](../css.md) - For styling images
- [Themes](../themes.md) - For consistent visual appearance
- [ResponsiveWidget](../widgets.md#responsivewidget) - Base widget interface