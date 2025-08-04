# Themes Module

Comprehensive theming system with JSON-based theme definitions, color schemes, and runtime theme switching for consistent visual design.

## ColorTheme

Core theme structure defining colors, typography, and visual elements.

```rust
use reactive_tui::themes::{ColorTheme, ThemeManager};

let theme = ColorTheme::builder()
    .primary("#3b82f6")
    .secondary("#10b981") 
    .background("#1f2937")
    .foreground("#f9fafb")
    .accent("#f59e0b")
    .build();

// Apply theme globally
ThemeManager::set_global_theme(theme);
```

### Color Definitions

```rust
pub struct ColorTheme {
    pub primary: Color,
    pub secondary: Color,
    pub accent: Color,
    pub background: Color,
    pub foreground: Color,
    pub surface: Color,
    pub border: Color,
    pub success: Color,
    pub warning: Color,
    pub error: Color,
    pub info: Color,
    pub muted: Color,
}
```

## Built-in Themes

### Default Theme

```rust
use reactive_tui::themes::defaults::*;

// Load built-in themes
let dark_theme = default_dark_theme();
let light_theme = default_light_theme();
let high_contrast = high_contrast_theme();
let terminal_theme = terminal_theme();
```

### Theme Variants

```rust
// Dark themes
let dark_blue = dark_blue_theme();
let dark_green = dark_green_theme();
let midnight = midnight_theme();

// Light themes  
let light_blue = light_blue_theme();
let light_green = light_green_theme();
let minimal = minimal_theme();

// Specialty themes
let matrix = matrix_theme();
let cyberpunk = cyberpunk_theme();
let retro = retro_theme();
```

## ThemeManager

Central theme management with runtime switching and persistence.

```rust
use reactive_tui::themes::ThemeManager;

let mut manager = ThemeManager::new();

// Load themes
manager.register_theme("dark", dark_theme());
manager.register_theme("light", light_theme());
manager.register_theme("custom", custom_theme());

// Switch themes
manager.set_active_theme("dark")?;
let current = manager.get_active_theme();

// Save theme preferences
manager.save_preferences("~/.config/myapp/theme.json")?;
manager.load_preferences("~/.config/myapp/theme.json")?;
```

## JSON Theme Format

Themes can be defined in JSON format for easy customization:

```json
{
  "name": "Custom Dark Theme",
  "version": "1.0.0",
  "colors": {
    "primary": "#3b82f6",
    "secondary": "#10b981",
    "accent": "#f59e0b",
    "background": "#1f2937",
    "foreground": "#f9fafb",
    "surface": "#374151",
    "border": "#6b7280",
    "success": "#22c55e",
    "warning": "#eab308",
    "error": "#ef4444",
    "info": "#06b6d4",
    "muted": "#9ca3af"
  },
  "typography": {
    "font_family": "monospace",
    "font_size": 14,
    "line_height": 1.4
  },
  "spacing": {
    "xs": 2,
    "sm": 4,
    "md": 8,
    "lg": 16,
    "xl": 24
  },
  "borders": {
    "width": 1,
    "radius": 4,
    "style": "solid"
  }
}
```

### Loading JSON Themes

```rust
use reactive_tui::themes::JsonThemeLoader;

let theme = JsonThemeLoader::from_file("themes/dark.json")?;
let theme = JsonThemeLoader::from_string(json_content)?;

// Validate theme
if let Err(errors) = theme.validate() {
    for error in errors {
        eprintln!("Theme validation error: {}", error);
    }
}
```

## Theme Components

### Typography

```rust
pub struct Typography {
    pub font_family: String,
    pub font_size: u16,
    pub line_height: f32,
    pub letter_spacing: f32,
    pub font_weight: FontWeight,
}

pub enum FontWeight {
    Normal,
    Bold,
    Light,
    Thin,
}
```

### Spacing System

```rust
pub struct Spacing {
    pub xs: u16,      // 2px
    pub sm: u16,      // 4px  
    pub md: u16,      // 8px
    pub lg: u16,      // 16px
    pub xl: u16,      // 24px
    pub xxl: u16,     // 32px
}
```

### Border Styles

```rust
pub struct BorderTheme {
    pub width: u16,
    pub radius: u16,
    pub style: BorderStyle,
    pub color: Color,
}
```

## CSS Variable Integration

Themes generate CSS variables for consistent styling:

```rust
use reactive_tui::themes::css_integration::ThemeCssGenerator;

let css_vars = ThemeCssGenerator::generate_variables(&theme);
// Outputs:
// --color-primary: #3b82f6;
// --color-secondary: #10b981;
// --spacing-md: 8px;
// etc.
```

### Using Theme Variables

```rust
let element = Element::with_tag("div")
    .style("background-color", "var(--color-primary)")
    .style("color", "var(--color-foreground)")
    .style("padding", "var(--spacing-md)")
    .build();
```

## Dynamic Theme Switching

### Runtime Theme Changes

```rust
use reactive_tui::themes::{ThemeManager, ThemeChangeEvent};

let mut manager = ThemeManager::new();

// Listen for theme changes
manager.on_theme_change(|event: ThemeChangeEvent| {
    println!("Theme changed from {} to {}", event.old_theme, event.new_theme);
    
    // Update all components
    app.update_theme(&event.new_theme);
});

// Switch themes
manager.set_active_theme("dark")?;
```

### Animated Theme Transitions

```rust
use reactive_tui::themes::ThemeTransition;

let transition = ThemeTransition::new()
    .from_theme(&current_theme)
    .to_theme(&target_theme)
    .duration(500) // 500ms
    .easing(EasingFunction::EaseInOut);

manager.apply_transition(transition)?;
```

## Theme Inheritance

### Base Theme Extension

```rust
let base_theme = default_dark_theme();
let custom_theme = ColorTheme::extend(&base_theme)
    .primary("#ff6b6b")  // Override primary color
    .accent("#4ecdc4")   // Override accent color
    .build();
```

### Theme Composition

```rust
let theme = ColorTheme::compose()
    .base(default_dark_theme())
    .overlay(high_contrast_adjustments())
    .overlay(user_preferences())
    .build();
```

## Accessibility Support

### High Contrast Themes

```rust
let high_contrast = ColorTheme::builder()
    .primary("#ffffff")
    .background("#000000")
    .contrast_ratio(7.0) // WCAG AAA compliance
    .build();
```

### Color Blind Support

```rust
use reactive_tui::themes::accessibility::*;

let protanopia_theme = adapt_for_protanopia(&base_theme);
let deuteranopia_theme = adapt_for_deuteranopia(&base_theme);
let tritanopia_theme = adapt_for_tritanopia(&base_theme);
```

## Theme Validation

### Color Contrast Checking

```rust
use reactive_tui::themes::validation::*;

let validator = ThemeValidator::new();
let results = validator.validate(&theme);

for warning in results.warnings {
    println!("Warning: {}", warning);
}

for error in results.errors {
    println!("Error: {}", error);
}
```

### Accessibility Compliance

```rust
let compliance = validator.check_wcag_compliance(&theme);
println!("WCAG AA: {}", compliance.aa_compliant);
println!("WCAG AAA: {}", compliance.aaa_compliant);
```

## Custom Theme Development

### Theme Builder Pattern

```rust
use reactive_tui::themes::{ColorTheme, ThemeBuilder};

let theme = ThemeBuilder::new("My Custom Theme")
    .version("1.0.0")
    .author("Your Name")
    .description("A beautiful custom theme")
    .primary("#your-color")
    .secondary("#your-color")
    .background("#your-color")
    .foreground("#your-color")
    .typography(Typography {
        font_family: "JetBrains Mono".to_string(),
        font_size: 14,
        line_height: 1.4,
        ..Default::default()
    })
    .spacing(Spacing {
        xs: 2,
        sm: 4,
        md: 8,
        lg: 16,
        xl: 24,
        xxl: 32,
    })
    .build();
```

### Theme Testing

```rust
use reactive_tui::themes::testing::*;

let test_suite = ThemeTestSuite::new();
test_suite.test_contrast_ratios(&theme);
test_suite.test_color_accessibility(&theme);
test_suite.test_component_rendering(&theme);

let report = test_suite.generate_report();
println!("{}", report);
```

## Integration with Components

### Theme-Aware Components

```rust
use reactive_tui::{themes::ThemedComponent, components::Element};

impl ThemedComponent for MyWidget {
    fn apply_theme(&mut self, theme: &ColorTheme) {
        self.element.style("background-color", &theme.primary.to_string());
        self.element.style("color", &theme.foreground.to_string());
        self.element.style("border-color", &theme.border.to_string());
    }
}
```

### Automatic Theme Application

```rust
let widget = Button::new("my-button", "Click Me")
    .theme_aware(true) // Automatically updates with theme changes
    .build();

// Theme changes automatically applied
ThemeManager::set_active_theme("light")?;
```

## Example Usage

```rust
use reactive_tui::{
    themes::{ColorTheme, ThemeManager, JsonThemeLoader},
    components::Element,
};

// Load custom theme
let theme = JsonThemeLoader::from_file("themes/my-theme.json")?;

// Register and activate
let mut manager = ThemeManager::new();
manager.register_theme("custom", theme);
manager.set_active_theme("custom")?;

// Create themed components
let app = Element::with_tag("div")
    .class("app")
    .style("background-color", "var(--color-background)")
    .style("color", "var(--color-foreground)")
    .child(
        Element::with_tag("header")
            .class("header")
            .style("background-color", "var(--color-primary)")
            .child(
                Element::with_tag("h1")
                    .text("My Themed App")
                    .build()
            )
            .build()
    )
    .build();

// Runtime theme switching
manager.set_active_theme("dark")?; // All components update automatically
```