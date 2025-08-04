# Slider Widget

Interactive range slider control for selecting numeric values within a specified range, supporting both single-value and dual-handle range selection with customizable styling and keyboard navigation.

## Overview

The Slider widget provides interactive controls for numeric value selection with support for single-value sliders and dual-handle range sliders. Features include keyboard navigation, mouse interaction, customizable appearance, tick marks, and full accessibility support.

```rust
use reactive_tui::widgets::*;

// Single value slider
let volume_slider = Slider::builder("volume")
    .range(0.0, 100.0)
    .value(75.0)
    .label("Volume")
    .width(30)
    .build()?;

// Range slider with dual handles
let price_range = Slider::builder("price-range")
    .range(0.0, 1000.0)
    .dual_range(100.0, 500.0)
    .label("Price Range")
    .width(40)
    .build()?;
```

## Features

- **Single & Range Modes**: Single-value selection or dual-handle range selection
- **Orientation Support**: Horizontal and vertical slider orientations
- **Keyboard Navigation**: Arrow keys, Home/End, Page Up/Down, Tab switching
- **Mouse Interaction**: Click-to-set value with handle detection
- **Visual Customization**: Customizable track, handles, and tick marks
- **Value Display**: Configurable value formatting and percentage display
- **Tick Marks**: Optional tick marks with labels and custom intervals
- **Step Support**: Configurable step increments for precise value control
- **Accessibility**: Full ARIA support with proper labeling and keyboard access
- **Responsive Design**: Adaptive sizing and visual feedback

## Core Components

### Slider

Main slider widget with interactive functionality.

```rust
pub struct Slider {
    pub id: String,
    pub mode: SliderMode,
    pub orientation: SliderOrientation,
    pub state: Reactive<SliderState>,
    pub style: SliderStyle,
    pub ticks: SliderTicks,
    pub classes: Vec<String>,
    pub attributes: std::collections::HashMap<String, String>,
    pub label: Option<String>,
    pub description: Option<String>,
}
```

### SliderState

State management for values, focus, and interaction.

```rust
pub struct SliderState {
    pub value: f64,
    pub range_end: f64,
    pub min: f64,
    pub max: f64,
    pub step: f64,
    pub active_handle: usize,
    pub focused: bool,
    pub disabled: bool,
}

impl SliderState {
    pub fn new(min: f64, max: f64, value: f64) -> Self
    pub fn value_percentage(&self) -> f64
    pub fn range_end_percentage(&self) -> f64
    pub fn set_value(&mut self, value: f64) -> Result<()>
    pub fn set_range_end(&mut self, value: f64) -> Result<()>
    pub fn increment(&mut self) -> Result<()>
    pub fn decrement(&mut self) -> Result<()>
    pub fn range_span(&self) -> f64
    pub fn is_at_min(&self) -> bool
    pub fn is_at_max(&self) -> bool
    pub fn validate(&self) -> Result<()>
}
```

### SliderMode

Slider interaction modes.

```rust
pub enum SliderMode {
    /// Single handle for selecting one value
    Single,
    /// Dual handles for selecting a range
    Range,
}
```

### SliderOrientation

Visual orientation options.

```rust
pub enum SliderOrientation {
    Horizontal,
    Vertical,
}
```

### SliderStyle

Visual styling configuration.

```rust
pub struct SliderStyle {
    pub track_char: char,
    pub active_track_char: char,
    pub handle_chars: [char; 2], // [primary, secondary for range mode]
    pub track_length: usize,
    pub show_values: bool,
    pub value_format: String,
    pub show_percentage: bool,
}

impl Default for SliderStyle {
    fn default() -> Self {
        Self {
            track_char: '─',
            active_track_char: '━',
            handle_chars: ['●', '○'],
            track_length: 20,
            show_values: true,
            value_format: "{:.1}".to_string(),
            show_percentage: false,
        }
    }
}
```

### SliderTicks

Tick mark configuration.

```rust
pub struct SliderTicks {
    pub enabled: bool,
    pub step: f64,
    pub show_labels: bool,
    pub custom_labels: Vec<String>,
    pub tick_char: char,
    pub major_tick_char: char,
    pub major_tick_interval: usize,
}

impl Default for SliderTicks {
    fn default() -> Self {
        Self {
            enabled: false,
            step: 0.0,
            show_labels: false,
            custom_labels: Vec::new(),
            tick_char: '|',
            major_tick_char: '┼',
            major_tick_interval: 5,
        }
    }
}
```

## Builder Pattern

### SliderBuilder

```rust
impl SliderBuilder {
    pub fn new(id: impl Into<String>) -> Self
    pub fn range(mut self, min: f64, max: f64) -> Self
    pub fn value(mut self, value: f64) -> Self
    pub fn width(mut self, width: usize) -> Self
    pub fn dual_range(mut self, start: f64, end: f64) -> Self
    pub fn orientation(mut self, orientation: SliderOrientation) -> Self
    pub fn style(mut self, style: SliderStyle) -> Self
    pub fn ticks(mut self, ticks: SliderTicks) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn attr(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
    pub fn label(mut self, label: impl Into<String>) -> Self
    pub fn description(mut self, description: impl Into<String>) -> Self
    pub fn build(self) -> Result<Slider>
}
```

## Methods

### Construction

```rust
impl Slider {
    // Create a new single-value slider
    pub fn new(id: impl Into<String>, min: f64, max: f64, value: f64) -> Result<Self>
    
    // Create a new slider builder
    pub fn builder(id: impl Into<String>) -> SliderBuilder
    
    // Create a range slider with dual handles
    pub fn range(id: impl Into<String>, min: f64, max: f64, start: f64, end: f64) -> Result<Self>
}
```

### Value Management

```rust
impl Slider {
    // Get current value
    pub fn value(&self) -> f64
    
    // Get range end value (for range sliders)
    pub fn range_end(&self) -> f64
    
    // Set value programmatically
    pub fn set_value(&self, value: f64) -> Result<()>
    
    // Set range values (for range sliders)
    pub fn set_range(&self, start: f64, end: f64) -> Result<()>
}
```

### State Management

```rust
impl Slider {
    // Enable/disable the slider
    pub fn set_disabled(&self, disabled: bool)
    
    // Set focus state
    pub fn set_focused(&self, focused: bool)
}
```

### Interaction Handling

```rust
impl Slider {
    // Handle keyboard input
    pub fn handle_key(&self, key: &str) -> Result<bool>
    
    // Handle mouse click at position (0.0 to 1.0 along track)
    pub fn handle_click(&self, position: f64) -> Result<()>
}
```

### Rendering

```rust
impl Slider {
    // Render the slider as text
    pub fn render_text(&self) -> String
    
    // Render with layout and theme support
    pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String
    
    // Convert to Element for UI framework integration
    pub fn to_element(&self) -> Result<Element>
}
```

## Examples

### Basic Volume Slider

```rust
use reactive_tui::widgets::*;

let volume_slider = Slider::builder("volume-control")
    .range(0.0, 100.0)
    .value(75.0)
    .label("Volume")
    .description("Adjust system volume")
    .width(25)
    .build()?;

println!("Current volume: {:.0}%", volume_slider.value());
```

### Temperature Range Selector

```rust
let temp_range = Slider::builder("temperature-range")
    .range(-20.0, 50.0)
    .dual_range(18.0, 25.0)
    .label("Comfortable Temperature Range")
    .width(40)
    .style(SliderStyle {
        track_char: '·',
        active_track_char: '━',
        handle_chars: ['❄', '🔥'],
        show_values: true,
        value_format: "{:.0}°C".to_string(),
        show_percentage: false,
        ..Default::default()
    })
    .build()?;

println!("Temperature range: {:.0}°C - {:.0}°C", 
    temp_range.value(), temp_range.range_end());
```

### Progress Slider with Ticks

```rust
let progress_slider = Slider::builder("progress")
    .range(0.0, 100.0)
    .value(67.5)
    .label("Progress")
    .width(50)
    .style(SliderStyle {
        track_char: '░',
        active_track_char: '▓',
        handle_chars: ['█', '█'],
        show_values: true,
        show_percentage: true,
        ..Default::default()
    })
    .ticks(SliderTicks {
        enabled: true,
        step: 10.0,
        show_labels: true,
        tick_char: '|',
        major_tick_char: '┃',
        major_tick_interval: 2,
        ..Default::default()
    })
    .build()?;
```

### Vertical Slider

```rust
let vertical_fader = Slider::builder("fader")
    .range(0.0, 127.0)
    .value(64.0)
    .orientation(SliderOrientation::Vertical)
    .label("Channel 1")
    .width(15) // Height for vertical
    .style(SliderStyle {
        track_char: '│',
        active_track_char: '┃',
        handle_chars: ['■', '□'],
        show_values: true,
        value_format: "{:.0}".to_string(),
        ..Default::default()
    })
    .build()?;
```

### Price Range Filter

```rust
let price_filter = Slider::builder("price-filter")
    .range(0.0, 2000.0)
    .dual_range(250.0, 750.0)
    .label("Price Range")
    .description("Filter products by price")
    .width(35)
    .style(SliderStyle {
        value_format: "${:.0}".to_string(),
        show_values: true,
        show_percentage: false,
        ..Default::default()
    })
    .class("price-slider")
    .attr("currency", "USD")
    .build()?;

// Handle user interaction
price_filter.handle_key("ArrowRight")?; // Increase value
price_filter.handle_key("Tab")?;        // Switch to end handle
price_filter.handle_key("ArrowLeft")?;  // Decrease end value
```

### Interactive Audio Equalizer

```rust
struct AudioEqualizer {
    bands: Vec<Slider>,
    band_names: Vec<String>,
}

impl AudioEqualizer {
    fn new() -> Result<Self> {
        let band_names = vec![
            "60Hz".to_string(),
            "170Hz".to_string(), 
            "310Hz".to_string(),
            "600Hz".to_string(),
            "1kHz".to_string(),
            "3kHz".to_string(),
            "6kHz".to_string(),
            "12kHz".to_string(),
            "14kHz".to_string(),
            "16kHz".to_string(),
        ];

        let mut bands = Vec::new();
        for (i, name) in band_names.iter().enumerate() {
            let band = Slider::builder(format!("eq-band-{}", i))
                .range(-12.0, 12.0)
                .value(0.0)
                .orientation(SliderOrientation::Vertical)
                .label(name)
                .width(12)
                .style(SliderStyle {
                    track_char: '│',
                    active_track_char: '┃',
                    handle_chars: ['●', '○'],
                    show_values: true,
                    value_format: "{:+.1}dB".to_string(),
                    ..Default::default()
                })
                .ticks(SliderTicks {
                    enabled: true,
                    step: 3.0,
                    show_labels: false,
                    tick_char: '─',
                    major_tick_char: '━',
                    major_tick_interval: 2,
                    ..Default::default()
                })
                .build()?;
            bands.push(band);
        }

        Ok(Self { bands, band_names })
    }

    fn render(&self) -> String {
        let mut output = String::new();
        output.push_str("Audio Equalizer\n");
        output.push_str("─".repeat(60).as_str());
        output.push('\n');

        // Render all bands side by side
        for (i, band) in self.bands.iter().enumerate() {
            if i > 0 {
                output.push_str("  ");
            }
            output.push_str(&band.render_text());
        }

        output
    }

    fn handle_key(&mut self, key: &str, active_band: usize) -> Result<bool> {
        if active_band < self.bands.len() {
            self.bands[active_band].handle_key(key)
        } else {
            Ok(false)
        }
    }
}
```

### Brightness Control with Animation

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

struct BrightnessControl {
    slider: Slider,
    current_brightness: Reactive<f64>,
}

impl BrightnessControl {
    fn new() -> Result<Self> {
        let slider = Slider::builder("brightness")
            .range(0.0, 100.0)
            .value(50.0)
            .label("Screen Brightness")
            .width(30)
            .style(SliderStyle {
                track_char: '░',
                active_track_char: '▓',
                handle_chars: ['☀', '☾'],
                show_values: true,
                show_percentage: true,
                ..Default::default()
            })
            .class("brightness-slider")
            .build()?;

        Ok(Self {
            slider,
            current_brightness: Reactive::new(50.0),
        })
    }

    fn set_brightness(&self, brightness: f64) -> Result<()> {
        self.slider.set_value(brightness)?;
        self.current_brightness.set(brightness);
        
        // Apply brightness to system (mock)
        println!("Setting system brightness to {:.0}%", brightness);
        Ok(())
    }

    fn increase_brightness(&self) -> Result<()> {
        let current = self.slider.value();
        let new_value = (current + 10.0).min(100.0);
        self.set_brightness(new_value)
    }

    fn decrease_brightness(&self) -> Result<()> {
        let current = self.slider.value();
        let new_value = (current - 10.0).max(0.0);
        self.set_brightness(new_value)
    }
}
```

### Gaming Settings Panel

```rust
struct GameSettings {
    volume: Slider,
    sensitivity: Slider,
    fov: Slider,
    brightness: Slider,
}

impl GameSettings {
    fn new() -> Result<Self> {
        let volume = Slider::builder("volume")
            .range(0.0, 100.0)
            .value(85.0)
            .label("Master Volume")
            .width(25)
            .style(SliderStyle {
                handle_chars: ['🔊', '🔇'],
                show_percentage: true,
                ..Default::default()
            })
            .build()?;

        let sensitivity = Slider::builder("mouse-sensitivity")
            .range(0.1, 10.0)
            .value(2.5)
            .label("Mouse Sensitivity")
            .width(25)
            .style(SliderStyle {
                value_format: "{:.2}x".to_string(),
                ..Default::default()
            })
            .build()?;

        let fov = Slider::builder("field-of-view")
            .range(60.0, 120.0)
            .value(90.0)
            .label("Field of View")
            .width(25)
            .style(SliderStyle {
                value_format: "{:.0}°".to_string(),
                ..Default::default()
            })
            .ticks(SliderTicks {
                enabled: true,
                step: 10.0,
                show_labels: true,
                ..Default::default()
            })
            .build()?;

        let brightness = Slider::builder("brightness")
            .range(0.0, 200.0)
            .value(100.0)
            .label("Brightness")
            .width(25)
            .style(SliderStyle {
                show_percentage: true,
                ..Default::default()
            })
            .build()?;

        Ok(Self {
            volume,
            sensitivity,
            fov,
            brightness,
        })
    }

    fn render(&self) -> String {
        format!(
            "Game Settings\n{}\n{}\n{}\n{}",
            self.volume.render_text(),
            self.sensitivity.render_text(),
            self.fov.render_text(),
            self.brightness.render_text()
        )
    }

    fn apply_settings(&self) -> Result<()> {
        println!("Applying game settings:");
        println!("  Volume: {:.0}%", self.volume.value());
        println!("  Sensitivity: {:.2}x", self.sensitivity.value());
        println!("  FOV: {:.0}°", self.fov.value());
        println!("  Brightness: {:.0}%", self.brightness.value());
        Ok(())
    }
}
```

### Color HSV Picker

```rust
struct ColorPicker {
    hue: Slider,
    saturation: Slider,
    value: Slider,
}

impl ColorPicker {
    fn new() -> Result<Self> {
        let hue = Slider::builder("hue")
            .range(0.0, 360.0)
            .value(180.0)
            .label("Hue")
            .width(36)
            .style(SliderStyle {
                track_char: '─',
                active_track_char: '█',
                handle_chars: ['●', '○'],
                value_format: "{:.0}°".to_string(),
                ..Default::default()
            })
            .build()?;

        let saturation = Slider::builder("saturation")
            .range(0.0, 100.0)
            .value(75.0)
            .label("Saturation")
            .width(36)
            .style(SliderStyle {
                track_char: '░',
                active_track_char: '▓',
                handle_chars: ['●', '○'],
                show_percentage: true,
                ..Default::default()
            })
            .build()?;

        let value = Slider::builder("value")
            .range(0.0, 100.0)
            .value(90.0)
            .label("Value")
            .width(36)
            .style(SliderStyle {
                track_char: '▁',
                active_track_char: '█',
                handle_chars: ['●', '○'],
                show_percentage: true,
                ..Default::default()
            })
            .build()?;

        Ok(Self {
            hue,
            saturation,
            value,
        })
    }

    fn get_hsv(&self) -> (f64, f64, f64) {
        (
            self.hue.value(),
            self.saturation.value() / 100.0,
            self.value.value() / 100.0,
        )
    }

    fn set_hsv(&self, h: f64, s: f64, v: f64) -> Result<()> {
        self.hue.set_value(h)?;
        self.saturation.set_value(s * 100.0)?;
        self.value.set_value(v * 100.0)?;
        Ok(())
    }

    fn render_with_preview(&self) -> String {
        let (h, s, v) = self.get_hsv();
        
        format!(
            "Color Picker\n{}\n{}\n{}\n\nHSV: ({:.0}°, {:.0}%, {:.0}%)\nPreview: {} [Color Block]",
            self.hue.render_text(),
            self.saturation.render_text(),
            self.value.render_text(),
            h, s * 100.0, v * 100.0,
            "█".repeat(8)
        )
    }
}
```

### Keyboard Input Handling

```rust
use reactive_tui::widgets::*;

fn handle_slider_input(slider: &Slider, key: &str) -> Result<bool> {
    match key {
        // Basic navigation
        "ArrowLeft" | "h" => slider.handle_key("ArrowLeft"),
        "ArrowRight" | "l" => slider.handle_key("ArrowRight"),
        "ArrowUp" | "k" => slider.handle_key("ArrowUp"),
        "ArrowDown" | "j" => slider.handle_key("ArrowDown"),
        
        // Jump to extremes
        "Home" | "0" => slider.handle_key("Home"),
        "End" | "$" => slider.handle_key("End"),
        
        // Large steps
        "PageUp" | "PageDown" => slider.handle_key(key),
        
        // Tab between handles (range mode)
        "Tab" => slider.handle_key("Tab"),
        
        // Custom shortcuts
        "=" | "+" => {
            // Increase by 5%
            let current = slider.value();
            let range = slider.state.get().max - slider.state.get().min;
            slider.set_value(current + range * 0.05)
        }
        "-" | "_" => {
            // Decrease by 5%
            let current = slider.value();
            let range = slider.state.get().max - slider.state.get().min;
            slider.set_value(current - range * 0.05)
        }
        
        _ => Ok(false),
    }
}
```

### Mouse Interaction

```rust
fn handle_slider_mouse(slider: &Slider, mouse_x: f64, slider_width: f64) -> Result<()> {
    // Convert mouse position to slider position (0.0 to 1.0)
    let position = (mouse_x / slider_width).clamp(0.0, 1.0);
    slider.handle_click(position)
}

// Example usage in event loop
fn process_mouse_event(slider: &Slider, mouse_event: MouseEvent) -> Result<()> {
    match mouse_event {
        MouseEvent::Click { x, y } => {
            // Assuming horizontal slider
            if y == slider_row {
                handle_slider_mouse(slider, x as f64, slider.style.track_length as f64)?;
            }
        }
        MouseEvent::Drag { x, y } => {
            // Handle dragging for smooth interaction
            if y == slider_row {
                handle_slider_mouse(slider, x as f64, slider.style.track_length as f64)?;
            }
        }
    }
    Ok(())
}
```

### Custom Styling Examples

```rust
// Gaming theme
let gaming_style = SliderStyle {
    track_char: '▬',
    active_track_char: '▰',
    handle_chars: ['◆', '◇'],
    track_length: 25,
    show_values: true,
    value_format: "{:.0}".to_string(),
    show_percentage: false,
};

// Minimal theme
let minimal_style = SliderStyle {
    track_char: '·',
    active_track_char: '━',
    handle_chars: ['●', '○'],
    track_length: 20,
    show_values: false,
    value_format: "{:.1}".to_string(),
    show_percentage: false,
};

// Retro theme
let retro_style = SliderStyle {
    track_char: '═',
    active_track_char: '█',
    handle_chars: ['▐', '▌'],
    track_length: 30,
    show_values: true,
    value_format: "[{:.0}]".to_string(),
    show_percentage: true,
};
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let settings_form = Element::with_tag("div")
    .class("settings-panel")
    .child(
        Element::with_tag("h2")
            .text("Audio Settings")
            .build()
    )
    .child(
        volume_slider.to_element()?
    )
    .child(
        bass_slider.to_element()?
    )
    .child(
        treble_slider.to_element()?
    )
    .build();
```

## CSS Styling

The slider generates semantic CSS classes:

```css
.slider {
    /* Base slider styles */
}

.slider-single {
    /* Single value slider */
}

.slider-range {
    /* Range slider with dual handles */
}

.slider-horizontal {
    /* Horizontal orientation */
}

.slider-vertical {
    /* Vertical orientation */
}

.slider-focused {
    /* Focused state */
}

.slider-disabled {
    /* Disabled state */
}
```

## Accessibility

- **ARIA Attributes**: Full ARIA support with `role="slider"`, `aria-valuemin`, `aria-valuemax`, `aria-valuenow`
- **Keyboard Navigation**: Complete keyboard accessibility with arrow keys, Home/End, Page Up/Down, Tab
- **Screen Reader**: Proper value announcements and state changes
- **Focus Management**: Clear focus indicators and tab navigation
- **Value Descriptions**: Accessible value formatting with units and context

## Performance Considerations

- **Efficient Rendering**: Only re-renders on value changes or focus state updates
- **Step Validation**: Automatic step snapping for consistent value increments
- **Range Validation**: Built-in validation to ensure values stay within bounds
- **Memory Efficient**: Lightweight state management with reactive updates

## Advanced Features

### Custom Step Increments

```rust
let precise_slider = Slider::builder("precise")
    .range(0.0, 1.0)
    .value(0.5)
    .build()?;

// Custom step handling
let mut state = precise_slider.state.get();
state.step = 0.01; // 1% increments
```

### Programmatic Animation

```rust
use tokio::time::{sleep, Duration};

async fn animate_slider_to_value(slider: &Slider, target: f64, duration_ms: u64) -> Result<()> {
    let start = slider.value();
    let steps = 30;
    let step_duration = Duration::from_millis(duration_ms / steps);
    
    for i in 0..=steps {
        let progress = i as f64 / steps as f64;
        let current = start + (target - start) * progress;
        slider.set_value(current)?;
        sleep(step_duration).await;
    }
    
    Ok(())
}
```

### Event-Driven Value Updates

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

let volume_level = Reactive::new(50.0);
let volume_clone = volume_level.clone();

let volume_slider = Slider::builder("volume")
    .range(0.0, 100.0)
    .value(volume_level.get())
    .build()?;

// Update slider when reactive value changes
volume_level.subscribe(move |new_value| {
    volume_slider.set_value(*new_value).unwrap();
});

// Update reactive value when slider changes (would need callback system)
// This demonstrates the pattern for bi-directional binding
```

The Slider widget provides comprehensive range selection functionality with extensive customization options, efficient interaction handling, and seamless integration with reactive state management and accessibility features.