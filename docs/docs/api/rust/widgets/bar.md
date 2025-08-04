# Bar Widget

Header and footer bar component with flexible positioning, content alignment, and responsive design for application navigation and status display.

## Overview

The Bar widget provides horizontal navigation bars, headers, footers, and status bars with support for multiple content sections, icons, and responsive layouts.

```rust
use reactive_tui::widgets::{Bar, BarBuilder, BarPosition, BarSection};

let header_bar = Bar::builder("header")
    .position(BarPosition::Top)
    .height(3)
    .add_section(BarSection::left().content("My App"))
    .add_section(BarSection::center().content("Dashboard"))
    .add_section(BarSection::right().content("User: John"))
    .build();
```

## BarBuilder

```rust
impl BarBuilder {
    pub fn new(id: &str) -> Self
    pub fn position(mut self, position: BarPosition) -> Self
    pub fn height(mut self, height: u16) -> Self
    pub fn width(mut self, width: u16) -> Self
    pub fn add_section(mut self, section: BarSection) -> Self
    pub fn background_color(mut self, color: Color) -> Self
    pub fn border(mut self, border: BorderStyle) -> Self
    pub fn padding(mut self, padding: u16) -> Self
    pub fn sticky(mut self, sticky: bool) -> Self
    pub fn build(self) -> Bar
}
```

## Bar Positions

```rust
pub enum BarPosition {
    Top,        // Header bar at top
    Bottom,     // Footer bar at bottom
    Fixed,      // Fixed position overlay
    Sticky,     // Sticks to viewport edge when scrolling
}
```

## Bar Sections

```rust
pub struct BarSection {
    pub alignment: SectionAlignment,
    pub content: String,
    pub width: SectionWidth,
    pub clickable: bool,
    pub on_click: Option<Box<dyn Fn() -> Result<()>>>,
}

pub enum SectionAlignment {
    Left,
    Center,
    Right,
}

pub enum SectionWidth {
    Auto,           // Content-based width
    Fixed(u16),     // Fixed width
    Flex(f32),      // Proportional width
    Fill,           // Fill remaining space
}

impl BarSection {
    pub fn left() -> Self
    pub fn center() -> Self
    pub fn right() -> Self
    pub fn content(mut self, content: &str) -> Self
    pub fn width(mut self, width: SectionWidth) -> Self
    pub fn clickable(mut self, clickable: bool) -> Self
    pub fn on_click<F>(mut self, callback: F) -> Self
    where F: Fn() -> Result<()> + 'static
}
```

## Examples

### Application Header

```rust
use reactive_tui::widgets::{Bar, BarSection, BarPosition};

let app_header = Bar::builder("app-header")
    .position(BarPosition::Top)
    .height(3)
    .add_section(
        BarSection::left()
            .content("🚀 Reactive TUI")
            .width(SectionWidth::Fixed(20))
    )
    .add_section(
        BarSection::center()
            .content("File Manager")
            .width(SectionWidth::Flex(1.0))
    )
    .add_section(
        BarSection::right()
            .content("Settings | Help | Exit")
            .width(SectionWidth::Auto)
            .clickable(true)
            .on_click(|| {
                show_menu();
                Ok(())
            })
    )
    .border(BorderStyle::Single)
    .build();
```

### Status Footer

```rust
let status_footer = Bar::builder("status-footer")
    .position(BarPosition::Bottom)
    .height(1)
    .add_section(
        BarSection::left()
            .content("Ready")
            .width(SectionWidth::Fixed(10))
    )
    .add_section(
        BarSection::center()
            .content("Line 42, Col 15")
            .width(SectionWidth::Auto)
    )
    .add_section(
        BarSection::right()
            .content("UTF-8 | LF | Rust")
            .width(SectionWidth::Auto)
    )
    .background_color(Color::DarkGray)
    .build();
```

### Navigation Bar

```rust
let nav_bar = Bar::builder("navigation")
    .position(BarPosition::Top)
    .height(2)
    .add_section(
        BarSection::left()
            .content("Home")
            .clickable(true)
            .on_click(|| {
                navigate_to("home");
                Ok(())
            })
    )
    .add_section(
        BarSection::left()
            .content("Projects")
            .clickable(true)
            .on_click(|| {
                navigate_to("projects");
                Ok(())
            })
    )
    .add_section(
        BarSection::left()
            .content("Settings")
            .clickable(true)
            .on_click(|| {
                navigate_to("settings");
                Ok(())
            })
    )
    .add_section(
        BarSection::right()
            .content("Profile")
            .clickable(true)
            .on_click(|| {
                show_profile();
                Ok(())
            })
    )
    .build();
```

### Progress Bar

```rust
use reactive_tui::{widgets::Bar, reactive::Reactive};

let progress = Reactive::new(45f32); // 45% progress
let progress_clone = progress.clone();

let progress_bar = Bar::builder("progress")
    .position(BarPosition::Fixed)
    .height(1)
    .add_section(
        BarSection::left()
            .content(&format!("Progress: {:.0}%", progress.get()))
            .width(SectionWidth::Fill)
    )
    .background_color(Color::Blue)
    .build();

// Update progress reactively
progress.set(75.0);
```

### Breadcrumb Bar

```rust
let breadcrumb_bar = Bar::builder("breadcrumb")
    .position(BarPosition::Top)
    .height(1)
    .add_section(
        BarSection::left()
            .content("Home > Projects > My App > src")
            .width(SectionWidth::Fill)
    )
    .padding(1)
    .background_color(Color::LightGray)
    .build();
```

## CSS Styling

```css
.bar {
    display: flex;
    align-items: center;
    background-color: #f8f9fa;
    border-bottom: 1px solid #dee2e6;
    padding: 0 16px;
}

.bar-top {
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    z-index: 100;
}

.bar-bottom {
    position: fixed;
    bottom: 0;
    left: 0;
    right: 0;
    z-index: 100;
}

.bar-section {
    display: flex;
    align-items: center;
    height: 100%;
}

.bar-section-left {
    justify-content: flex-start;
}

.bar-section-center {
    justify-content: center;
}

.bar-section-right {
    justify-content: flex-end;
    margin-left: auto;
}

.bar-section-clickable {
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 4px;
    transition: background-color 0.2s ease;
}

.bar-section-clickable:hover {
    background-color: rgba(0, 0, 0, 0.1);
}

.bar-header {
    background-color: #343a40;
    color: white;
    font-weight: 600;
}

.bar-footer {
    background-color: #6c757d;
    color: white;
    font-size: 12px;
}

.bar-navigation {
    background-color: #007bff;
    color: white;
}

.bar-status {
    background-color: #28a745;
    color: white;
    font-family: monospace;
}
```

## Responsive Behavior

```rust
use reactive_tui::widgets::{Bar, ResponsiveBreakpoint};

let responsive_bar = Bar::builder("responsive-header")
    .position(BarPosition::Top)
    .height(3)
    .add_section(
        BarSection::left()
            .content("App Name")
            .width(SectionWidth::Fixed(15))
            .responsive(ResponsiveBreakpoint::new(60, SectionWidth::Auto))
    )
    .add_section(
        BarSection::center()
            .content("Navigation Menu")
            .width(SectionWidth::Flex(1.0))
            .responsive(ResponsiveBreakpoint::new(60, SectionWidth::Fixed(0))) // Hide on small screens
    )
    .add_section(
        BarSection::right()
            .content("☰") // Hamburger menu on small screens
            .width(SectionWidth::Auto)
            .responsive(ResponsiveBreakpoint::new(60, SectionWidth::Fixed(3)))
    )
    .build();
```

## Integration Examples

### With State Management

```rust
use reactive_tui::{widgets::Bar, reactive::Reactive};

let current_user = Reactive::new("Guest".to_string());
let notification_count = Reactive::new(0u32);

let user_clone = current_user.clone();
let notif_clone = notification_count.clone();

let dynamic_bar = Bar::builder("dynamic-header")
    .position(BarPosition::Top)
    .height(2)
    .add_section(
        BarSection::left()
            .content("My Application")
            .width(SectionWidth::Fixed(20))
    )
    .add_section(
        BarSection::right()
            .content(&format!("Welcome, {} ({})", user_clone.get(), notif_clone.get()))
            .width(SectionWidth::Auto)
    )
    .build();

// Update user and notifications
current_user.set("John Doe".to_string());
notification_count.set(3);
```

### Multi-level Navigation

```rust
let main_nav = Bar::builder("main-nav")
    .position(BarPosition::Top)
    .height(2)
    .add_section(BarSection::left().content("File"))
    .add_section(BarSection::left().content("Edit"))
    .add_section(BarSection::left().content("View"))
    .add_section(BarSection::left().content("Help"))
    .build();

let sub_nav = Bar::builder("sub-nav")
    .position(BarPosition::Top)
    .height(1)
    .add_section(BarSection::left().content("New | Open | Save | Save As"))
    .background_color(Color::LightGray)
    .build();
```

The Bar widget provides flexible header, footer, and navigation solutions with comprehensive styling and responsive design capabilities for terminal applications.