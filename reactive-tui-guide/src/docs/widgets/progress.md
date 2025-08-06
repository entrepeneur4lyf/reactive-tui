# Progress Widget

The Progress widget provides progress bars and indicators with animations and multiple display styles.

## Features

- **Multiple Styles**: Linear, Circular, Arc progress bars
- **Animations**: Smooth progress transitions
- **Labels**: Customizable progress labels
- **Percentage Display**: Optional percentage indicators

## Basic Usage

```rust
use reactive_tui::widgets::*;

let progress = Progress::builder("download")
    .value(65)
    .max(100)
    .label("Downloading files...")
    .show_percentage(true)
    .style(ProgressStyle::Linear)
    .build();
```

## Progress Styles

- **Linear**: Traditional horizontal progress bar
- **Circular**: Circular progress indicator
- **Arc**: Partial circle progress
- **Spinner**: Indeterminate progress spinner

## Configuration Options

- **value**: Current progress value
- **max**: Maximum progress value
- **min**: Minimum progress value (default: 0)
- **label**: Progress description text
- **show_percentage**: Display percentage
- **animated**: Enable animations

## Indeterminate Progress

```rust
let spinner = Progress::builder("loading")
    .indeterminate(true)
    .label("Loading...")
    .spinner_type(SpinnerType::Dots)
    .build();
```

## Events

- **on_complete**: Fired when progress reaches maximum
- **on_change**: Progress value changes
