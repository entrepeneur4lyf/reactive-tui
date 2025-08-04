# Link Widget

Interactive hyperlink component with URL handling, styling, and click events.

## Overview

The Link widget provides a flexible, CSS-styled interactive element for creating clickable links that can open URLs, trigger actions, or navigate within the application. Links integrate seamlessly with the theming system and support focus navigation, keyboard activation, and various visual styles.

## Features

- **URL Handling**: Local file paths, web URLs, email addresses, and custom protocols
- **Click Actions**: Open in browser, execute commands, trigger application events
- **State Management**: Normal, hover, active, focused, visited, disabled states
- **CSS Integration**: Full CSS styling support with utility classes
- **Accessibility**: Keyboard navigation and screen reader support
- **Customization**: Icons, underlines, colors, and custom styling
- **Security**: URL validation and safe opening mechanisms

## Basic Usage

### Simple Web Link

```rust
use reactive_tui::prelude::*;
use reactive_tui::widgets::*;

let docs_link = link("docs-link", |builder| {
    builder
        .text("Documentation")
        .url("https://docs.rs/reactive-tui")
        .target(LinkTarget::Browser)
});
```

### Factory Functions

The Link widget provides several convenience factory functions:

```rust
// Web link
let web = web_link("homepage", "https://example.com", "Visit Homepage");

// Email link
let email = email_link("contact", "support@example.com", "Contact Support");

// File link
let file = file_link("config", "./config.toml", "📄 Configuration");

// Application link
let app = app_link("settings", "open_settings", "Settings");

// Command link (use with caution)
let cmd = command_link("build", "cargo build", "Build Project");
```

## Configuration

### LinkConfig

```rust
pub struct LinkConfig {
    pub text: String,                      // Link text to display
    pub url: String,                       // Target URL or path
    pub target: LinkTarget,                // How to handle link activation
    pub decoration: LinkDecoration,        // Link decoration style
    pub alt_text: Option<String>,          // Alternative text for accessibility
    pub tooltip: Option<String>,           // Tooltip text on hover
    pub disabled: bool,                    // Whether link is disabled
    pub classes: Vec<String>,              // Custom CSS classes
    pub attributes: HashMap<String, String>, // Custom attributes
    pub on_click: Option<String>,          // Callback event name
    pub show_external_indicator: bool,     // Show external link indicator
    pub visited: bool,                     // Whether link has been visited
}
```

### Link Targets

```rust
pub enum LinkTarget {
    Browser,      // Open in system default browser
    Application,  // Handle within the application
    Command,      // Execute as system command
    System,       // Open in default application for file type
    Custom,       // Custom handler (requires callback)
}
```

### Link States

```rust
pub enum LinkState {
    Normal,    // Normal unvisited link
    Hover,     // Mouse hovering over link
    Active,    // Link currently being clicked
    Focused,   // Link has keyboard focus
    Visited,   // Link has been visited before
    Disabled,  // Link is disabled and non-interactive
}
```

### Link Decorations

```rust
pub enum LinkDecoration {
    None,           // No decoration
    Underline,      // Underline the link text
    Dotted,         // Dotted underline
    Dashed,         // Dashed underline
    Double,         // Double underline
    Overline,       // Overline decoration
    StrikeThrough,  // Strike through
}
```

## Advanced Usage

### Custom Link Configuration

```rust
let custom_link = link("custom", |builder| {
    builder
        .text("Advanced Link")
        .url("https://example.com/advanced")
        .target(LinkTarget::Browser)
        .decoration(LinkDecoration::Dotted)
        .alt_text("Link to advanced documentation")
        .tooltip("Click to open advanced guide")
        .show_external_indicator(true)
        .class("custom-link")
        .class("highlight")
});
```

### Email Link with Subject

```rust
let email_with_subject = link("support", |builder| {
    builder
        .text("Get Support")
        .url("mailto:support@example.com?subject=Help%20Request")
        .target(LinkTarget::Browser)
        .class("support-link")
});
```

### Application Navigation

```rust
let nav_link = link("profile", |builder| {
    builder
        .text("My Profile")
        .url("profile")
        .target(LinkTarget::Application)
        .on_click("navigate_to_profile")
        .show_external_indicator(false)
        .class("nav-link")
});
```

### Disabled Link

```rust
let disabled_link = link("disabled", |builder| {
    builder
        .text("Coming Soon")
        .url("https://example.com/soon")
        .disabled(true)
        .decoration(LinkDecoration::None)
        .class("disabled-link")
});
```

## URL Types and Handling

### Automatic URL Type Detection

The widget automatically detects URL types:

```rust
// Web URLs
"https://example.com"     // Opens in browser
"http://example.com"      // Opens in browser
"ftp://files.example.com" // Opens in browser

// Email addresses
"user@example.com"        // Opens in email client
"mailto:user@example.com" // Opens in email client

// File paths
"./config.toml"           // Opens with system default
"../README.md"            // Opens with system default
"/etc/hosts"              // Opens with system default
"file:///path/to/file"    // Opens with system default

// Commands (use with LinkTarget::Command)
"ls -la"                  // Executes command
"code ."                  // Opens VS Code
```

### Cross-Platform Support

Link opening works across platforms:

- **Windows**: Uses `cmd /c start`
- **macOS**: Uses `open`
- **Linux**: Uses `xdg-open`

## Security Considerations

### URL Validation

All URLs are validated before opening:

```rust
let widget = LinkWidget::new("test", config);
match widget.validate_url() {
    Ok(()) => println!("URL is valid"),
    Err(e) => eprintln!("Invalid URL: {}", e),
}
```

### Command Execution Safety

Command execution requires explicit target:

```rust
// Safe - won't execute commands
let safe_link = link("safe", |builder| {
    builder
        .text("Safe Link")
        .url("rm -rf /")  // This won't execute
        .target(LinkTarget::Browser)  // Will try to open as URL
});

// Dangerous - will execute commands
let dangerous_link = link("dangerous", |builder| {
    builder
        .text("Dangerous Link")
        .url("rm -rf /")
        .target(LinkTarget::Command)  // This WILL execute!
});
```

## State Management

### Programmatic State Control

```rust
let mut widget = LinkWidget::new("interactive", config);

// Set state
widget.set_state(LinkState::Hover);

// Get current state
match widget.state() {
    LinkState::Normal => println!("Link is normal"),
    LinkState::Visited => println!("Link has been visited"),
    LinkState::Disabled => println!("Link is disabled"),
    _ => {}
}

// Activate link
match widget.activate() {
    Ok(()) => println!("Link activated successfully"),
    Err(e) => eprintln!("Failed to activate link: {}", e),
}
```

### Automatic State Transitions

- Links automatically become `Visited` after activation
- Disabled links cannot change state
- Focus and hover states are managed by the UI system

## CSS Integration

The Link widget generates semantic CSS classes:

```rust
let styled_link = link("styled", |builder| {
    builder
        .text("Styled Link")
        .url("https://example.com")
        .class("primary")
        .class("bold")
});
```

### Generated CSS Classes

- `link` - Base widget class
- `link-{state}` - Current state (normal, hover, active, etc.)
- `link-decoration-{decoration}` - Decoration style
- `link-external` - External URL indicator
- `link-email` - Email link indicator
- `link-file` - File link indicator

## Accessibility

### Alternative Text and Labels

```rust
let accessible_link = link("accessible", |builder| {
    builder
        .text("Download")
        .url("https://example.com/download")
        .alt_text("Download the latest version of our software")
        .tooltip("Click to start download")
});
```

### Keyboard Navigation

Links are automatically focusable and support:
- **Tab**: Navigate to link
- **Enter**: Activate link
- **Space**: Activate link

## Visual Styling

### ANSI Escape Sequences

The widget uses ANSI escape sequences for terminal styling:

```rust
// Underline styles
LinkDecoration::Underline      // \x1b[4m text \x1b[24m
LinkDecoration::Dotted         // \x1b[4:3m text \x1b[24m
LinkDecoration::Dashed         // \x1b[4:2m text \x1b[24m

// State styles
LinkState::Hover               // \x1b[1m text \x1b[22m (bold)
LinkState::Active              // \x1b[2m text \x1b[22m (dim)
LinkState::Focused             // \x1b[7m text \x1b[27m (reverse)
LinkState::Disabled            // \x1b[2;37m text \x1b[22;39m (dim gray)
```

## Examples

### Navigation Menu

```rust
let nav_menu = flex_row()
    .child(app_link("home", "home", "Home"))
    .child(app_link("about", "about", "About"))
    .child(app_link("contact", "contact", "Contact"))
    .child(web_link("blog", "https://blog.example.com", "Blog ↗"))
    .class("nav-menu");
```

### Footer Links

```rust
let footer = flex_column()
    .child(
        flex_row()
            .child(web_link("privacy", "https://example.com/privacy", "Privacy Policy"))
            .child(web_link("terms", "https://example.com/terms", "Terms of Service"))
            .child(email_link("support", "support@example.com", "Support"))
    )
    .child(text("© 2025 Example Corp"))
    .class("footer");
```

### Link List

```rust
let links = flex_column()
    .child(web_link("docs", "https://docs.example.com", "📚 Documentation"))
    .child(web_link("api", "https://api.example.com", "🔧 API Reference"))  
    .child(file_link("config", "./config.toml", "⚙️ Configuration"))
    .child(app_link("settings", "settings", "🎛️ Settings"))
    .class("link-list");
```

### Conditional Links

```rust
let conditional_link = if user.is_admin() {
    app_link("admin", "admin_panel", "Admin Panel")
} else {
    link("disabled-admin", |builder| {
        builder
            .text("Admin Panel")
            .url("#")
            .disabled(true)
            .tooltip("Admin access required")
    })
};
```

## Error Handling

All link operations return results:

```rust
// URL validation
match widget.validate_url() {
    Ok(()) => {},
    Err(TuiError::Component(msg)) => eprintln!("Invalid URL: {}", msg),
    Err(e) => eprintln!("Error: {}", e),
}

// Link activation
match widget.activate() {
    Ok(()) => println!("Link opened successfully"),
    Err(e) => eprintln!("Failed to open link: {}", e),
}
```

## Related

- [Button Widget](button.md) - For clickable actions without URLs
- [Layout System](../layout.md) - For link positioning
- [CSS Engine](../css.md) - For styling links
- [Events](../events.md) - For handling link interactions
- [ResponsiveWidget](../widgets.md#responsivewidget) - Base widget interface