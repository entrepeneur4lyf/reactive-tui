# Switch Widget

Interactive toggle switch widget for boolean state control with customizable appearance, labels, and reactive state integration.

## Overview

The Switch widget provides an interactive on/off toggle control with extensive customization options including custom labels, handle characters, track styling, label positioning, and full reactive state integration. Perfect for settings panels, preferences, and boolean state controls.

```rust
use reactive_tui::widgets::{switch, Switch, SwitchBuilder, LabelPosition};

// Basic on/off switch
let power_switch = switch("power")
    .enabled(true)
    .labels("ON", "OFF")
    .build();

// Custom styled switch
let notification_switch = switch("notifications")
    .enabled(false)
    .labels("🔔", "🔕")
    .handles('●', '○')
    .width(10)
    .description("Enable notifications")
    .build();
```

## Features

- **Interactive Toggle**: Click or keyboard (Space/Enter) to toggle state
- **Custom Labels**: Configurable on/off text labels with positioning options
- **Custom Handles**: Customizable handle characters for on/off states
- **Flexible Sizing**: Adjustable track width to fit interface needs
- **Label Positioning**: Before, after, or both sides of the switch
- **Reactive State**: Full integration with reactive state management
- **Accessibility**: ARIA attributes and keyboard navigation support
- **Disabled State**: Non-interactive mode for read-only displays
- **Focus Management**: Visual focus indicators and proper tab navigation

## Core Components

### Switch

Main switch widget with interactive functionality.

```rust
pub struct Switch {
    pub id: String,
    pub state: SwitchState,
    pub style: SwitchStyle,
    pub description: Option<String>,
    pub reactive_state: Option<Arc<ReactiveState>>,
}
```

### SwitchState

State management for toggle state and interaction.

```rust
pub struct SwitchState {
    /// Current toggle state (on/off)
    pub enabled: bool,
    /// Whether the switch is interactive
    pub interactive: bool,
    /// Whether the switch is focused
    pub focused: bool,
}

impl Default for SwitchState {
    fn default() -> Self {
        Self {
            enabled: false,
            interactive: true,
            focused: false,
        }
    }
}
```

### SwitchStyle

Visual styling and appearance configuration.

```rust
pub struct SwitchStyle {
    /// Text to show when switch is ON
    pub on_label: String,
    /// Text to show when switch is OFF  
    pub off_label: String,
    /// Character for the switch handle when ON
    pub on_handle: char,
    /// Character for the switch handle when OFF
    pub off_handle: char,
    /// Character for the track background
    pub track_char: char,
    /// Width of the switch track
    pub width: u16,
    /// Show labels inline with switch
    pub show_labels: bool,
    /// Position labels (before/after switch)
    pub label_position: LabelPosition,
}

impl Default for SwitchStyle {
    fn default() -> Self {
        Self {
            on_label: "ON".to_string(),
            off_label: "OFF".to_string(),
            on_handle: '●',
            off_handle: '○',
            track_char: '─',
            width: 8,
            show_labels: true,
            label_position: LabelPosition::After,
        }
    }
}
```

### LabelPosition

Label positioning options around the switch.

```rust
pub enum LabelPosition {
    Before,  // Label before switch: "ON [●─────]" 
    After,   // Label after switch: "[●─────] ON"
    Both,    // Labels on both sides: "OFF [●─────] ON"
}
```

## Builder Pattern

### SwitchBuilder

```rust
impl SwitchBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn enabled(mut self, enabled: bool) -> Self
    pub fn interactive(mut self, interactive: bool) -> Self
    pub fn labels<S1: Into<String>, S2: Into<String>>(mut self, on_label: S1, off_label: S2) -> Self
    pub fn handles(mut self, on_handle: char, off_handle: char) -> Self
    pub fn width(mut self, width: u16) -> Self
    pub fn label_position(mut self, position: LabelPosition) -> Self
    pub fn description<S: Into<String>>(mut self, description: S) -> Self
    pub fn build(self) -> Switch
}
```

### Convenience Function

```rust
pub fn switch<S: Into<String>>(id: S) -> SwitchBuilder
```

## Methods

### Construction

```rust
impl Switch {
    // Create a new switch with default settings
    pub fn new<S: Into<String>>(id: S) -> Self
    
    // Create a builder for fluent configuration
    pub fn switch<S: Into<String>>(id: S) -> SwitchBuilder
}
```

### Configuration

```rust
impl Switch {
    // Set the initial enabled state
    pub fn enabled(mut self, enabled: bool) -> Self
    
    // Set whether the switch is interactive
    pub fn interactive(mut self, interactive: bool) -> Self
    
    // Set custom labels
    pub fn labels<S1: Into<String>, S2: Into<String>>(mut self, on_label: S1, off_label: S2) -> Self
    
    // Set custom handle characters
    pub fn handles(mut self, on_handle: char, off_handle: char) -> Self
    
    // Set switch width
    pub fn width(mut self, width: u16) -> Self
    
    // Set label position
    pub fn label_position(mut self, position: LabelPosition) -> Self
    
    // Set description/tooltip
    pub fn description<S: Into<String>>(mut self, description: S) -> Self
}
```

### State Management

```rust
impl Switch {
    // Toggle the switch state
    pub fn toggle(&mut self) -> Result<()>
    
    // Set the switch state explicitly
    pub fn set_enabled(&mut self, enabled: bool) -> Result<()>
    
    // Get current enabled state
    pub fn is_enabled(&self) -> bool
    
    // Get current interactive state
    pub fn is_interactive(&self) -> bool
}
```

### Reactive State Integration

```rust
impl Switch {
    // Connect to reactive state for live updates
    pub fn connect_reactive(&mut self, state: Arc<ReactiveState>) -> Result<()>
    
    // Sync state to reactive state if connected
    fn sync_reactive_state(&self)
}
```

### Rendering

```rust
impl Switch {
    // Render the switch as a string for display
    pub fn render_string(&self) -> String
    
    // Render the switch with layout and theme support
    pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String
    
    // Convert to Element for integration with the component system
    pub fn to_element(&self) -> Element
}
```

## Examples

### Basic Toggle Switch

```rust
use reactive_tui::widgets::{switch, Switch};

// Simple on/off switch
let power_switch = switch("power")
    .enabled(false)
    .labels("ON", "OFF")
    .description("Power toggle")
    .build();

println!("Power: {}", power_switch.render_string());
// Output: "[○─────] OFF"
```

### Settings Panel Switches

```rust
use reactive_tui::widgets::{switch, LabelPosition};

// Notification settings
let notifications = switch("notifications")
    .enabled(true)
    .labels("Notifications", "Silent")
    .label_position(LabelPosition::Before)
    .width(6)
    .build();

let dark_mode = switch("dark_mode")
    .enabled(false)
    .labels("Dark", "Light")
    .handles('🌙', '☀')
    .label_position(LabelPosition::After)
    .build();

let auto_save = switch("auto_save")
    .enabled(true)
    .labels("Auto", "Manual")
    .width(10)
    .description("Automatic saving")
    .build();

println!("Settings Panel:");
println!("Notifications [●───] Notifications");
println!("[○──────] Light");
println!("[●────────] Auto");
```

### Custom Styled Switches

```rust
// Gaming theme switches
let vsync = switch("vsync")
    .enabled(true)
    .labels("V-Sync", "OFF")
    .handles('▰', '▱')
    .width(12)
    .build();

let fps_counter = switch("fps")
    .enabled(false)
    .labels("📊", "❌")
    .handles('●', '○')
    .width(6)
    .label_position(LabelPosition::Both)
    .build();

println!("Gaming Settings:");
println!("{}", vsync.render_string());
println!("{}", fps_counter.render_string());
```

### Interactive Switch Panel

```rust
use reactive_tui::widgets::{switch, Switch};
use std::collections::HashMap;

struct SettingsPanel {
    switches: HashMap<String, Switch>,
}

impl SettingsPanel {
    fn new() -> Self {
        let mut switches = HashMap::new();
        
        switches.insert("wifi".to_string(), 
            switch("wifi")
                .enabled(true)
                .labels("WiFi", "OFF")
                .handles('📶', '📵')
                .description("Wireless network connection")
                .build()
        );
        
        switches.insert("bluetooth".to_string(),
            switch("bluetooth")
                .enabled(false)
                .labels("Bluetooth", "OFF")
                .handles('🔵', '⚪')
                .description("Bluetooth connectivity")
                .build()
        );
        
        switches.insert("location".to_string(),
            switch("location")
                .enabled(true)
                .labels("GPS", "OFF")
                .handles('📍', '❌')
                .description("Location services")
                .build()
        );
        
        switches.insert("airplane".to_string(),
            switch("airplane")
                .enabled(false)
                .labels("Airplane", "OFF")
                .handles('✈️', '🛬')
                .description("Airplane mode")
                .build()
        );
        
        Self { switches }
    }
    
    fn toggle_switch(&mut self, id: &str) -> Result<(), String> {
        if let Some(switch) = self.switches.get_mut(id) {
            switch.toggle().map_err(|e| e.to_string())?;
            println!("Toggled {}: {}", id, if switch.is_enabled() { "ON" } else { "OFF" });
        }
        Ok(())
    }
    
    fn render(&self) -> String {
        let mut output = String::new();
        output.push_str("Settings Panel\n");
        output.push_str("─".repeat(30).as_str());
        output.push('\n');
        
        for (name, switch) in &self.switches {
            output.push_str(&format!("{:12} {}\n", name, switch.render_string()));
        }
        
        output
    }
    
    fn is_airplane_mode(&self) -> bool {
        self.switches.get("airplane")
            .map(|s| s.is_enabled())
            .unwrap_or(false)
    }
    
    fn apply_airplane_mode(&mut self) {
        let airplane_on = self.is_airplane_mode();
        
        if airplane_on {
            // Turn off other radios when airplane mode is on
            for (id, switch) in self.switches.iter_mut() {
                if id != "airplane" && (id == "wifi" || id == "bluetooth") {
                    let _ = switch.set_enabled(false);
                }
            }
            println!("Airplane mode enabled - wireless radios disabled");
        } else {
            println!("Airplane mode disabled");
        }
    }
}

// Usage example
let mut panel = SettingsPanel::new();
println!("{}", panel.render());

// Toggle airplane mode and apply restrictions
panel.toggle_switch("airplane")?;
panel.apply_airplane_mode();
println!("{}", panel.render());
```

### Reactive State Integration

```rust
use reactive_tui::{widgets::switch, reactive::ReactiveState};
use std::sync::Arc;

// Create reactive state
let app_state = Arc::new(ReactiveState::new());

// Create switches with reactive integration
let mut notifications = switch("notifications")
    .enabled(true)
    .labels("🔔", "🔕")
    .description("Push notifications")
    .build();

let mut location = switch("location")
    .enabled(false)
    .labels("📍", "❌")
    .description("Location services")
    .build();

// Connect to reactive state
notifications.connect_reactive(app_state.clone())?;
location.connect_reactive(app_state.clone())?;

// Watch for state changes
app_state.watch_field("notifications.enabled", |enabled: bool| {
    println!("Notifications: {}", if enabled { "Enabled" } else { "Disabled" });
});

app_state.watch_field("location.enabled", |enabled: bool| {
    println!("Location: {}", if enabled { "Enabled" } else { "Disabled" });
});

// Toggle switches - state changes will be broadcast
notifications.toggle()?;
location.toggle()?;
```

### Form Integration Example

```rust
use reactive_tui::widgets::{switch, Switch, LabelPosition};

struct UserPreferences {
    email_notifications: Switch,
    push_notifications: Switch,
    dark_mode: Switch,
    auto_save: Switch,
    analytics: Switch,
}

impl UserPreferences {
    fn new() -> Self {
        Self {
            email_notifications: switch("email_notifications")
                .enabled(true)
                .labels("Email Notifications", "Disabled")
                .label_position(LabelPosition::After)
                .description("Receive email notifications")
                .build(),
                
            push_notifications: switch("push_notifications")
                .enabled(false)
                .labels("Push Notifications", "Disabled")
                .label_position(LabelPosition::After)
                .build(),
                
            dark_mode: switch("dark_mode")
                .enabled(true)
                .labels("🌙", "☀️")
                .handles('●', '○')
                .width(6)
                .label_position(LabelPosition::Both)
                .description("Dark mode theme")
                .build(),
                
            auto_save: switch("auto_save")
                .enabled(true)
                .labels("Auto Save", "Manual")
                .width(8)
                .build(),
                
            analytics: switch("analytics")
                .enabled(false)
                .labels("Analytics", "Private")
                .label_position(LabelPosition::After)
                .description("Share usage analytics")
                .build(),
        }
    }
    
    fn render_form(&self) -> String {
        format!(
            "User Preferences\n\
             ═══════════════\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n",
            self.email_notifications.render_string(),
            self.push_notifications.render_string(),
            self.dark_mode.render_string(),
            self.auto_save.render_string(),
            self.analytics.render_string()
        )
    }
    
    fn validate(&self) -> Result<(), String> {
        // At least one notification method must be enabled
        if !self.email_notifications.is_enabled() && !self.push_notifications.is_enabled() {
            return Err("At least one notification method must be enabled".to_string());
        }
        Ok(())
    }
    
    fn to_json(&self) -> serde_json::Value {
        serde_json::json!({
            "email_notifications": self.email_notifications.is_enabled(),
            "push_notifications": self.push_notifications.is_enabled(),
            "dark_mode": self.dark_mode.is_enabled(),
            "auto_save": self.auto_save.is_enabled(),
            "analytics": self.analytics.is_enabled()
        })
    }
}

// Usage
let preferences = UserPreferences::new();
println!("{}", preferences.render_form());
println!("Config: {}", preferences.to_json());
```

### Advanced Switch Styling

```rust
use reactive_tui::widgets::{switch, LabelPosition};

// Different visual styles
let styles = vec![
    // Classic toggle
    switch("classic")
        .enabled(true)
        .labels("ON", "OFF")
        .handles('●', '○')
        .width(8)
        .build(),
    
    // Emoji style
    switch("emoji")
        .enabled(false)
        .labels("✅", "❌")
        .handles('🟢', '🔴')
        .width(6)
        .build(),
    
    // Minimalist
    switch("minimal")
        .enabled(true)
        .labels("", "")
        .handles('█', '░')
        .width(4)
        .build(),
    
    // Progress style
    switch("progress")
        .enabled(true)
        .labels("Active", "Inactive")
        .handles('▰', '▱')
        .width(10)
        .label_position(LabelPosition::Before)
        .build(),
    
    // Binary style
    switch("binary")
        .enabled(false)
        .labels("1", "0")
        .handles('█', '▫')
        .width(3)
        .label_position(LabelPosition::Both)
        .build(),
];

println!("Switch Styles:");
for (i, switch) in styles.iter().enumerate() {
    println!("Style {}: {}", i + 1, switch.render_string());
}
```

### Gaming Settings Panel

```rust
struct GameSettings {
    graphics: GameGraphicsSettings,
    audio: GameAudioSettings,
    controls: GameControlSettings,
}

impl GameSettings {
    fn new() -> Self {
        Self {
            graphics: GameGraphicsSettings::new(),
            audio: GameAudioSettings::new(),
            controls: GameControlSettings::new(),
        }
    }
}

struct GameGraphicsSettings {
    vsync: Switch,
    fullscreen: Switch,
    anti_aliasing: Switch,
    shadows: Switch,
    particles: Switch,
}

impl GameGraphicsSettings {
    fn new() -> Self {
        Self {
            vsync: switch("vsync")
                .enabled(true)
                .labels("V-Sync", "OFF")
                .handles('🔒', '🔓')
                .width(8)
                .description("Vertical synchronization")
                .build(),
                
            fullscreen: switch("fullscreen")
                .enabled(false)
                .labels("Fullscreen", "Windowed")
                .handles('⛶', '🗔')
                .label_position(LabelPosition::After)
                .build(),
                
            anti_aliasing: switch("aa")
                .enabled(true)
                .labels("AA", "OFF")
                .handles('●', '○')
                .width(6)
                .description("Anti-aliasing")
                .build(),
                
            shadows: switch("shadows")
                .enabled(true)
                .labels("Shadows", "OFF")
                .handles('🌑', '⚪')
                .build(),
                
            particles: switch("particles")
                .enabled(false)
                .labels("Particles", "OFF")
                .handles('✨', '❌')
                .build(),
        }
    }
    
    fn render(&self) -> String {
        format!(
            "Graphics Settings\n\
             ─────────────────\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n\
             {}\n",
            self.vsync.render_string(),
            self.fullscreen.render_string(),
            self.anti_aliasing.render_string(),
            self.shadows.render_string(),
            self.particles.render_string()
        )
    }
    
    fn performance_mode(&mut self) {
        // Optimize for performance
        let _ = self.vsync.set_enabled(false);
        let _ = self.anti_aliasing.set_enabled(false);
        let _ = self.shadows.set_enabled(false);
        let _ = self.particles.set_enabled(false);
        println!("Performance mode enabled");
    }
    
    fn quality_mode(&mut self) {
        // Optimize for quality
        let _ = self.vsync.set_enabled(true);
        let _ = self.anti_aliasing.set_enabled(true);
        let _ = self.shadows.set_enabled(true);
        let _ = self.particles.set_enabled(true);
        println!("Quality mode enabled");
    }
}
```

### Keyboard Input Handling

```rust
use reactive_tui::widgets::Switch;

fn handle_switch_input(switch: &mut Switch, key: &str) -> Result<bool, String> {
    match key {
        // Toggle switch
        " " | "Enter" => {
            if switch.is_interactive() {
                switch.toggle().map_err(|e| e.to_string())?;
                println!("Switch {}: {}", 
                    switch.id, 
                    if switch.is_enabled() { "ON" } else { "OFF" }
                );
                Ok(true)
            } else {
                Ok(false)
            }
        }
        
        // Explicit on/off
        "1" | "y" | "Y" => {
            if switch.is_interactive() {
                switch.set_enabled(true).map_err(|e| e.to_string())?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        
        "0" | "n" | "N" => {
            if switch.is_interactive() {
                switch.set_enabled(false).map_err(|e| e.to_string())?;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        
        _ => Ok(false),
    }
}

// Example usage in event loop
fn process_key_event(switches: &mut Vec<Switch>, key: &str, active_index: usize) -> Result<(), String> {
    if active_index < switches.len() {
        let handled = handle_switch_input(&mut switches[active_index], key)?;
        
        if handled {
            // Trigger re-render or state update
            render_switches(switches);
        }
    }
    
    Ok(())
}

fn render_switches(switches: &[Switch]) {
    println!("Switch Panel:");
    for (i, switch) in switches.iter().enumerate() {
        let indicator = if switch.state.focused { "► " } else { "  " };
        println!("{}{}", indicator, switch.render_string());
    }
}
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let settings_form = Element::with_tag("div")
    .class("settings-form")
    .child(
        Element::with_tag("h2")
            .text("Application Settings")
            .class("form-title")
            .build()
    )
    .child(
        Element::with_tag("div")
            .class("form-field")
            .child(
                Element::with_tag("label")
                    .text("Notifications")
                    .class("form-label")
                    .build()
            )
            .child(notifications_switch.to_element())
            .build()
    )
    .child(
        Element::with_tag("div")
            .class("form-field")
            .child(
                Element::with_tag("label")
                    .text("Dark Mode")
                    .class("form-label")
                    .build()
            )
            .child(dark_mode_switch.to_element())
            .build()
    )
    .build();
```

## CSS Styling

The switch widget generates semantic CSS classes for styling:

```css
.switch {
    /* Base switch styles */
    display: inline-block;
    cursor: pointer;
}

.switch-on {
    /* Switch in ON state */
    color: var(--switch-on-color, #22c55e);
}

.switch-off {
    /* Switch in OFF state */
    color: var(--switch-off-color, #6b7280);
}

.switch-focused {
    /* Focused switch */
    outline: 2px solid var(--focus-color, #3b82f6);
    outline-offset: 2px;
}

.switch-disabled {
    /* Disabled switch */
    opacity: 0.5;
    cursor: not-allowed;
}

/* Custom styling for different switch states */
.switch[aria-checked="true"] {
    /* ON state styling */
}

.switch[aria-checked="false"] {
    /* OFF state styling */
}
```

## Accessibility

- **ARIA Attributes**: Full ARIA support with `role="switch"`, `aria-checked`, `aria-label`
- **Keyboard Navigation**: Space and Enter keys to toggle state
- **Focus Management**: Clear focus indicators and proper tab navigation
- **Screen Reader**: Proper state announcements and descriptions
- **Semantic HTML**: Correct semantic markup for assistive technologies

## Performance Considerations

- **Lightweight Rendering**: Efficient string-based rendering with minimal memory usage
- **State Synchronization**: Reactive state updates only when values change
- **Event Handling**: Optimized keyboard and interaction event processing
- **Memory Efficient**: Minimal resource usage for simple toggle functionality

## Advanced Features

### Custom Track Characters

```rust
let custom_switch = switch("custom")
    .enabled(true)
    .handles('▰', '▱')
    .width(12)
    .build();

// Output: [▰──────────]
```

### Dynamic Label Updates

```rust
let mut status_switch = switch("status")
    .enabled(false)
    .labels("Online", "Offline")
    .build();

// Update labels based on connection state
if connection_established {
    status_switch = status_switch.labels("Connected", "Connecting...");
} else {
    status_switch = status_switch.labels("Online", "Offline");
}
```

### Conditional Interactivity

```rust
let mut protected_switch = switch("protected")
    .enabled(true)
    .labels("Enabled", "Disabled")
    .interactive(user_has_permission)
    .build();

// Disable interaction for unauthorized users
if !user_has_admin_rights {
    protected_switch = protected_switch.interactive(false);
}
```

The Switch widget provides comprehensive toggle functionality with extensive customization options, reactive state integration, accessibility support, and seamless integration with the reactive-tui component system.