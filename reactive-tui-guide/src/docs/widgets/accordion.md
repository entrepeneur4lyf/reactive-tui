# Accordion Widget

The Accordion widget provides expandable/collapsible sections for organizing content in a space-efficient manner.

## Features

- **Multiple Sections**: Support for multiple collapsible sections
- **Animations**: Smooth expand/collapse animations
- **Keyboard Navigation**: Full keyboard accessibility
- **Customizable Styling**: CSS-based styling support

## Basic Usage

```rust
use reactive_tui::widgets::*;

let accordion = Accordion::builder("settings-accordion")
    .section("General Settings", general_content)
    .section("Advanced Options", advanced_content)
    .section("Security", security_content)
    .animation(AccordionAnimation::Slide)
    .build();
```

## Configuration Options

- **animation**: Animation type (Slide, Fade, None)
- **allow_multiple**: Allow multiple sections to be open
- **default_open**: Sections that are open by default

## Keyboard Controls

- **Enter/Space**: Toggle section
- **↑/↓**: Navigate between sections
- **Home/End**: Jump to first/last section
