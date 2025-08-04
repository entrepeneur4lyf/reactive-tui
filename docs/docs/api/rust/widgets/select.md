# Select Widget

Comprehensive dropdown selection widget supporting single/multi-select modes, search filtering, keyboard navigation, custom options with icons and descriptions, and smart positioning.

## Overview

The Select widget provides a complete dropdown selection experience with support for single and multiple selection modes, search-as-you-type filtering, keyboard navigation, custom option rendering with icons and descriptions, smart positioning, and extensive customization options.

```rust
use reactive_tui::widgets::{Select, SelectBuilder, SelectOption, SelectMode};

// Simple single-select dropdown
let language_select = SelectBuilder::new("language-select")
    .options(vec!["Rust", "TypeScript", "Python", "Go"])
    .selected(Some(0))
    .placeholder("Choose a language...")
    .build();

// Multi-select with search
let tags_select = SelectBuilder::new("tags-select")
    .custom_options(vec![
        SelectOption::new("frontend", "Frontend").icon("🎨"),
        SelectOption::new("backend", "Backend").icon("⚙️"),
        SelectOption::new("database", "Database").icon("🗄️"),
        SelectOption::new("mobile", "Mobile").icon("📱"),
    ])
    .multi_select(true)
    .searchable(true)
    .selected_indices(vec![0, 2])
    .build();
```

## Features

- **Selection Modes**: Single-select and multi-select with different behaviors
- **Search Filtering**: Real-time option filtering as user types with custom filter functions
- **Keyboard Navigation**: Arrow keys, Enter/Space selection, Escape to close, Tab switching
- **Custom Options**: Flexible option display with icons, descriptions, grouping, and custom data
- **Smart Positioning**: Intelligent dropdown positioning with viewport awareness (Below, Above, Auto)
- **State Management**: Complete state tracking with reactive updates
- **Accessibility**: Full ARIA support and screen reader compatibility
- **Customizable Styling**: Extensive CSS class configuration for all components
- **Event Callbacks**: Change and toggle callbacks for interaction handling

## Core Components

### Select

Main select dropdown widget with comprehensive functionality.

```rust
pub struct Select {
    pub id: String,
    pub options: Vec<SelectOption>,
    pub state: Reactive<SelectState>,
    pub mode: SelectMode,
    pub searchable: bool,
    pub placeholder: String,
    pub position: DropdownPosition,
    pub style: SelectStyle,
    pub on_change: Option<OnChangeCallback>,
    pub on_toggle: Option<Arc<dyn Fn(bool) + Send + Sync>>,
    pub disabled: bool,
    pub required: bool,
    pub filter_fn: Option<FilterCallback>,
}
```

### SelectOption

Individual option configuration with extensive customization.

```rust
pub struct SelectOption {
    pub id: String,
    pub label: String,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub group: Option<String>,
    pub disabled: bool,
    pub data: HashMap<String, String>,
}

impl SelectOption {
    pub fn new<S: Into<String>>(id: S, label: S) -> Self
    pub fn icon<S: Into<String>>(mut self, icon: S) -> Self
    pub fn description<S: Into<String>>(mut self, description: S) -> Self
    pub fn group<S: Into<String>>(mut self, group: S) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn data<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self
}
```

### SelectState

Comprehensive state management for dropdown behavior.

```rust
pub struct SelectState {
    pub open: bool,
    pub highlighted_index: Option<usize>,
    pub selected_indices: Vec<usize>,
    pub search_query: String,
    pub filtered_indices: Vec<usize>,
    pub focused: bool,
    pub scroll_offset: usize,
}
```

### SelectMode

Selection behavior modes.

```rust
pub enum SelectMode {
    /// Single selection - only one option can be selected
    Single,
    /// Multiple selection - multiple options can be selected
    Multiple,
}
```

### DropdownPosition

Smart positioning options for dropdown overlay.

```rust
pub enum DropdownPosition {
    /// Prefer to show below the select input
    Below,
    /// Prefer to show above the select input
    Above,
    /// Automatically choose based on available space
    Auto,
}
```

### SelectStyle

Comprehensive styling configuration.

```rust
pub struct SelectStyle {
    pub container_classes: Vec<String>,
    pub trigger_classes: Vec<String>,
    pub dropdown_classes: Vec<String>,
    pub option_classes: Vec<String>,
    pub selected_option_classes: Vec<String>,
    pub highlighted_option_classes: Vec<String>,
    pub disabled_option_classes: Vec<String>,
    pub search_input_classes: Vec<String>,
    pub group_classes: Vec<String>,
    pub dropdown_arrow: String,
    pub selected_marker: String,
    pub max_height: usize,
    pub show_icons: bool,
    pub show_descriptions: bool,
}
```

## Builder Pattern

### SelectBuilder

```rust
impl SelectBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn options<S: Into<String>>(mut self, labels: Vec<S>) -> Self
    pub fn custom_options(mut self, options: Vec<SelectOption>) -> Self
    pub fn multi_select(mut self, multi: bool) -> Self
    pub fn searchable(mut self, searchable: bool) -> Self
    pub fn placeholder<S: Into<String>>(mut self, placeholder: S) -> Self
    pub fn position(mut self, position: DropdownPosition) -> Self
    pub fn style(mut self, style: SelectStyle) -> Self
    pub fn max_height(mut self, height: usize) -> Self
    pub fn selected(mut self, index: Option<usize>) -> Self
    pub fn selected_indices(mut self, indices: Vec<usize>) -> Self
    pub fn on_change<F>(mut self, callback: F) -> Self
    pub fn on_toggle<F>(mut self, callback: F) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn required(mut self, required: bool) -> Self
    pub fn filter<F>(mut self, filter_fn: F) -> Self
    pub fn build(self) -> Select
}
```

## Methods

### Construction

```rust
impl Select {
    // Create a new select widget builder
    pub fn builder<S: Into<String>>(id: S) -> SelectBuilder
    
    // Get the currently selected options
    pub fn selected_options(&self) -> Vec<&SelectOption>
    
    // Get the currently selected option IDs
    pub fn selected_ids(&self) -> Vec<String>
}
```

### Selection Management

```rust
impl Select {
    // Check if an option is selected
    pub fn is_selected(&self, index: usize) -> bool
    
    // Select an option by index
    pub fn select(&mut self, index: usize) -> Result<()>
    
    // Deselect an option by index
    pub fn deselect(&mut self, index: usize) -> Result<()>
    
    // Toggle selection of an option
    pub fn toggle_selection(&mut self, index: usize) -> Result<()>
    
    // Clear all selections
    pub fn clear_selection(&mut self)
}
```

### Dropdown Control

```rust
impl Select {
    // Open the dropdown
    pub fn open(&mut self)
    
    // Close the dropdown
    pub fn close(&mut self)
    
    // Toggle the dropdown open/closed state
    pub fn toggle(&mut self)
}
```

### Search and Filtering

```rust
impl Select {
    // Set the search query and update filtered options
    pub fn set_search_query<S: Into<String>>(&mut self, query: S)
    
    // Update the filtered options based on current search query
    fn update_filtered_options(&mut self)
}
```

### Keyboard Navigation

```rust
impl Select {
    // Navigate to the next option
    pub fn navigate_next(&mut self)
    
    // Navigate to the previous option
    pub fn navigate_previous(&mut self)
    
    // Select the currently highlighted option
    pub fn select_highlighted(&mut self) -> Result<()>
    
    // Handle key events for the select widget
    pub fn handle_key(&mut self, key: &str) -> Result<bool>
}
```

### Rendering

```rust
impl Select {
    // Convert to Element for rendering
    pub fn to_element(&self) -> Element
    
    // Create the dropdown overlay element
    fn create_dropdown_element(&self, state: &SelectState) -> Element
    
    // Get display text for the selected value(s)
    pub fn display_text(&self) -> String
}
```

## Examples

### Basic Language Selector

```rust
use reactive_tui::widgets::{SelectBuilder, SelectOption};

let language_select = SelectBuilder::new("language")
    .placeholder("Choose a programming language...")
    .custom_options(vec![
        SelectOption::new("rust", "Rust").icon("🦀"),
        SelectOption::new("typescript", "TypeScript").icon("📘"),
        SelectOption::new("python", "Python").icon("🐍"),
        SelectOption::new("go", "Go").icon("🐹"),
        SelectOption::new("java", "Java").icon("☕"),
    ])
    .searchable(true)
    .on_change(|selected| {
        println!("Selected language: {:?}", selected);
    })
    .build();
```

### Multi-Select Skills Picker

```rust
let skills_select = SelectBuilder::new("skills")
    .placeholder("Select your skills...")
    .custom_options(vec![
        SelectOption::new("frontend", "Frontend Development")
            .icon("🎨")
            .description("HTML, CSS, JavaScript, React"),
        SelectOption::new("backend", "Backend Development")
            .icon("⚙️")
            .description("APIs, databases, server architecture"),
        SelectOption::new("mobile", "Mobile Development")
            .icon("📱")
            .description("iOS, Android, React Native"),
        SelectOption::new("devops", "DevOps")
            .icon("🚀")
            .description("CI/CD, containerization, cloud"),
        SelectOption::new("design", "UI/UX Design")
            .icon("🎯")
            .description("User interface and experience design"),
    ])
    .multi_select(true)
    .searchable(true)
    .max_height(8)
    .selected_indices(vec![0, 1])
    .on_change(|selected| {
        println!("Selected skills: {:?}", selected);
    })
    .build();
```

### Grouped Country Selector

```rust
let country_select = SelectBuilder::new("country")
    .placeholder("Select your country...")
    .custom_options(vec![
        // North America
        SelectOption::new("us", "United States")
            .group("North America")
            .icon("🇺🇸"),
        SelectOption::new("ca", "Canada")
            .group("North America")
            .icon("🇨🇦"),
        SelectOption::new("mx", "Mexico")
            .group("North America")
            .icon("🇲🇽"),
        
        // Europe
        SelectOption::new("uk", "United Kingdom")
            .group("Europe")
            .icon("🇬🇧"),
        SelectOption::new("de", "Germany")
            .group("Europe")
            .icon("🇩🇪"),
        SelectOption::new("fr", "France")
            .group("Europe")
            .icon("🇫🇷"),
        
        // Asia
        SelectOption::new("jp", "Japan")
            .group("Asia")
            .icon("🇯🇵"),
        SelectOption::new("cn", "China")
            .group("Asia")
            .icon("🇨🇳"),
        SelectOption::new("kr", "South Korea")
            .group("Asia")
            .icon("🇰🇷"),
    ])
    .searchable(true)
    .position(DropdownPosition::Auto)
    .build();
```

### Priority Selector with Custom Styling

```rust
use reactive_tui::widgets::{SelectBuilder, SelectOption, SelectStyle};

let priority_select = SelectBuilder::new("priority")
    .placeholder("Select priority level...")
    .custom_options(vec![
        SelectOption::new("critical", "Critical")
            .icon("🔴")
            .description("Needs immediate attention"),
        SelectOption::new("high", "High")
            .icon("🟠")
            .description("Important but not urgent"),
        SelectOption::new("medium", "Medium")
            .icon("🟡")
            .description("Standard priority"),
        SelectOption::new("low", "Low")
            .icon("🟢")
            .description("Can be done later"),
        SelectOption::new("none", "No Priority")
            .icon("⚪")
            .description("Backlog item")
            .disabled(true),
    ])
    .style(SelectStyle {
        container_classes: vec!["priority-select".to_string()],
        trigger_classes: vec![
            "priority-trigger".to_string(),
            "border-2".to_string(),
            "rounded-lg".to_string(),
        ],
        dropdown_classes: vec![
            "priority-dropdown".to_string(),
            "shadow-xl".to_string(),
        ],
        selected_marker: "✅".to_string(),
        show_icons: true,
        show_descriptions: true,
        max_height: 6,
        ..Default::default()
    })
    .selected(Some(2)) // Default to medium priority
    .build();
```

### Searchable User Selector with Custom Filter

```rust
let user_select = SelectBuilder::new("assignee")
    .placeholder("Search for a user...")
    .custom_options(vec![
        SelectOption::new("alice", "Alice Johnson")
            .description("alice@company.com")
            .data("role", "Developer")
            .data("team", "Frontend"),
        SelectOption::new("bob", "Bob Smith")
            .description("bob@company.com")
            .data("role", "Designer")
            .data("team", "UX"),
        SelectOption::new("charlie", "Charlie Brown")
            .description("charlie@company.com")
            .data("role", "Manager")
            .data("team", "Product"),
        SelectOption::new("diana", "Diana Prince")
            .description("diana@company.com")
            .data("role", "Developer")
            .data("team", "Backend"),
    ])
    .searchable(true)
    .filter(|option, query| {
        let query_lower = query.to_lowercase();
        // Search in name, email, role, and team
        option.label.to_lowercase().contains(&query_lower)
            || option.description.as_ref()
                .map(|desc| desc.to_lowercase().contains(&query_lower))
                .unwrap_or(false)
            || option.data.values()
                .any(|value| value.to_lowercase().contains(&query_lower))
    })
    .on_change(|selected| {
        if let Some(&index) = selected.first() {
            println!("Assigned to user at index: {}", index);
        }
    })
    .build();
```

### Form Integration Example

```rust
use reactive_tui::widgets::{SelectBuilder, SelectOption};

struct UserProfileForm {
    country_select: Select,
    timezone_select: Select,
    language_select: Select,
}

impl UserProfileForm {
    fn new() -> Self {
        let country_select = SelectBuilder::new("country")
            .placeholder("Select your country...")
            .custom_options(get_countries())
            .searchable(true)
            .required(true)
            .on_change(|selected| {
                // Update timezone options based on country
                update_timezone_options(selected);
            })
            .build();

        let timezone_select = SelectBuilder::new("timezone")
            .placeholder("Select your timezone...")
            .custom_options(vec![]) // Populated based on country
            .searchable(true)
            .disabled(true) // Enabled after country selection
            .build();

        let language_select = SelectBuilder::new("languages")
            .placeholder("Select languages you speak...")
            .custom_options(get_languages())
            .multi_select(true)
            .searchable(true)
            .max_height(8)
            .build();

        Self {
            country_select,
            timezone_select,
            language_select,
        }
    }

    fn validate(&self) -> Result<(), String> {
        if self.country_select.selected_options().is_empty() {
            return Err("Please select a country".to_string());
        }
        
        if self.timezone_select.selected_options().is_empty() {
            return Err("Please select a timezone".to_string());
        }
        
        Ok(())
    }

    fn get_form_data(&self) -> ProfileData {
        ProfileData {
            country: self.country_select.selected_ids().first().cloned(),
            timezone: self.timezone_select.selected_ids().first().cloned(),
            languages: self.language_select.selected_ids(),
        }
    }
}

fn get_countries() -> Vec<SelectOption> {
    vec![
        SelectOption::new("us", "United States").icon("🇺🇸"),
        SelectOption::new("ca", "Canada").icon("🇨🇦"),
        SelectOption::new("uk", "United Kingdom").icon("🇬🇧"),
        SelectOption::new("de", "Germany").icon("🇩🇪"),
        // ... more countries
    ]
}

fn get_languages() -> Vec<SelectOption> {
    vec![
        SelectOption::new("en", "English"),
        SelectOption::new("es", "Spanish"),
        SelectOption::new("fr", "French"),
        SelectOption::new("de", "German"),
        SelectOption::new("zh", "Chinese"),
        SelectOption::new("ja", "Japanese"),
        // ... more languages
    ]
}
```

### Advanced Settings Selector

```rust
let settings_select = SelectBuilder::new("theme-settings")
    .placeholder("Choose theme settings...")
    .custom_options(vec![
        SelectOption::new("light", "Light Theme")
            .icon("☀️")
            .description("Clean and bright interface")
            .data("primary", "#3b82f6")
            .data("background", "#ffffff"),
        
        SelectOption::new("dark", "Dark Theme")
            .icon("🌙")
            .description("Easy on the eyes")
            .data("primary", "#60a5fa")
            .data("background", "#111827"),
        
        SelectOption::new("auto", "Auto Theme")
            .icon("🔄")
            .description("Follows system preference")
            .data("primary", "auto")
            .data("background", "auto"),
        
        SelectOption::new("high-contrast", "High Contrast")
            .icon("🔆")
            .description("Maximum readability")
            .data("primary", "#000000")
            .data("background", "#ffffff"),
    ])
    .searchable(false) // Disable search for simple theme selection
    .position(DropdownPosition::Above) // Show above if near bottom
    .on_change(|selected| {
        if let Some(&index) = selected.first() {
            apply_theme_settings(index);
        }
    })
    .on_toggle(|is_open| {
        if is_open {
            println!("Theme selector opened");
        } else {
            println!("Theme selector closed");
        }
    })
    .build();

fn apply_theme_settings(theme_index: usize) {
    println!("Applying theme at index: {}", theme_index);
    // Apply theme configuration
}
```

### Keyboard Event Handling

```rust
use reactive_tui::widgets::Select;

fn handle_select_input(select: &mut Select, key: &str) -> Result<bool, String> {
    match key {
        // Open/close dropdown
        "Enter" | " " => {
            let handled = select.handle_key(key).map_err(|e| e.to_string())?;
            Ok(handled)
        }
        
        // Navigation
        "ArrowDown" | "ArrowUp" => {
            let handled = select.handle_key(key).map_err(|e| e.to_string())?;
            Ok(handled)
        }
        
        // Close dropdown
        "Escape" => {
            select.close();
            Ok(true)
        }
        
        // Clear selection (if enabled)
        "Delete" | "Backspace" => {
            if key == "Delete" {
                select.clear_selection();
                Ok(true)
            } else {
                // Handle backspace in search
                let handled = select.handle_key(key).map_err(|e| e.to_string())?;
                Ok(handled)
            }
        }
        
        // Character input for search
        _ if key.len() == 1 && select.searchable => {
            let handled = select.handle_key(key).map_err(|e| e.to_string())?;
            Ok(handled)
        }
        
        _ => Ok(false),
    }
}

// Example usage in event loop
fn process_input_event(select: &mut Select, key_event: KeyEvent) -> Result<(), String> {
    let key = format_key_event(key_event);
    let handled = handle_select_input(select, &key)?;
    
    if handled {
        // Trigger re-render
        render_ui();
    }
    
    Ok(())
}
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let form_panel = Element::with_tag("div")
    .class("form-panel")
    .child(
        Element::with_tag("h2")
            .text("User Information")
            .class("form-title")
            .build()
    )
    .child(
        Element::with_tag("div")
            .class("form-field")
            .child(
                Element::with_tag("label")
                    .text("Country")
                    .class("form-label")
                    .build()
            )
            .child(country_select.to_element())
            .build()
    )
    .child(
        Element::with_tag("div")
            .class("form-field")
            .child(
                Element::with_tag("label")
                    .text("Skills")
                    .class("form-label")
                    .build()
            )
            .child(skills_select.to_element())
            .build()
    )
    .build();
```

## CSS Styling

The select widget generates semantic CSS classes for styling:

```css
.select {
    /* Base select container */
}

.select-disabled {
    /* Disabled state */
    opacity: 0.5;
    cursor: not-allowed;
}

.select-focused {
    /* Focused state */
}

.select-open {
    /* Open dropdown state */
}

.select-trigger {
    /* Select button/trigger */
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 8px 12px;
    border: 1px solid var(--border-color);
    border-radius: 4px;
    cursor: pointer;
}

.select-dropdown {
    /* Dropdown overlay */
    position: absolute;
    background: var(--background-color);
    border: 1px solid var(--border-color);
    border-radius: 4px;
    box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.select-option {
    /* Individual option */
    padding: 8px 12px;
    cursor: pointer;
}

.select-option-selected {
    /* Selected option */
    background-color: var(--primary-color);
    color: var(--primary-text-color);
}

.select-option-highlighted {
    /* Highlighted option (keyboard navigation) */
    background-color: var(--hover-color);
}

.select-option-disabled {
    /* Disabled option */
    opacity: 0.5;
    cursor: not-allowed;
}

.select-search {
    /* Search input */
    padding: 8px 12px;
    border: none;
    border-bottom: 1px solid var(--border-color);
    outline: none;
}

.select-group {
    /* Option group header */
    padding: 4px 12px;
    font-weight: bold;
    background-color: var(--group-background);
    color: var(--group-text-color);
}
```

## Accessibility

- **ARIA Attributes**: Full ARIA support with `role="combobox"`, `aria-expanded`, `aria-haspopup`
- **Keyboard Navigation**: Complete keyboard accessibility with arrow keys, Enter/Space, Escape
- **Screen Reader**: Proper option announcements and state changes
- **Focus Management**: Clear focus indicators and proper tab navigation
- **Selection Announcements**: Screen reader announcements for selection changes

## Performance Considerations

- **Efficient Filtering**: Fast search filtering with customizable filter functions
- **Virtual Scrolling**: Handles large option lists efficiently (when implemented)
- **State Optimization**: Reactive state management with minimal re-renders
- **Memory Efficient**: Lightweight option storage with on-demand rendering

## Advanced Features

### Custom Filter Functions

```rust
let custom_filtered_select = SelectBuilder::new("custom-filter")
    .custom_options(get_complex_options())
    .searchable(true)
    .filter(|option, query| {
        // Custom fuzzy search implementation
        fuzzy_match(&option.label, query) > 0.5
            || option.data.values().any(|v| fuzzy_match(v, query) > 0.3)
    })
    .build();

fn fuzzy_match(text: &str, query: &str) -> f64 {
    // Implement fuzzy matching algorithm
    if text.to_lowercase().contains(&query.to_lowercase()) {
        1.0
    } else {
        0.0
    }
}
```

### Dynamic Option Loading

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

let dynamic_options = Reactive::new(Vec::<SelectOption>::new());
let loading_state = Reactive::new(false);

let dynamic_select = SelectBuilder::new("dynamic")
    .placeholder("Start typing to search...")
    .custom_options(dynamic_options.get())
    .searchable(true)
    .on_change(move |selected| {
        println!("Selected: {:?}", selected);
    })
    .build();

// Update options reactively
dynamic_options.subscribe(move |new_options| {
    // Update select options when reactive value changes
    update_select_options(&dynamic_select, new_options.clone());
});

async fn search_and_update_options(query: String) {
    loading_state.set(true);
    
    let results = perform_search(&query).await;
    let options: Vec<SelectOption> = results
        .into_iter()
        .map(|item| {
            SelectOption::new(&item.id, &item.name)
                .description(&item.description)
                .data("category", &item.category)
        })
        .collect();
    
    dynamic_options.set(options);
    loading_state.set(false);
}
```

The Select widget provides comprehensive dropdown selection functionality with extensive customization options, efficient search and filtering, smart positioning, and seamless integration with reactive state management and accessibility features.