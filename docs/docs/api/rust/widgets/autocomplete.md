# Autocomplete Widget

A comprehensive autocomplete widget supporting search-as-you-type functionality with filtered suggestions, keyboard navigation, custom filtering, and multi-selection capabilities.

## Overview

The Autocomplete widget provides real-time suggestion filtering as users type, with support for multiple filter modes, keyboard navigation, custom rendering, and both single and multi-selection modes. It includes async support, debouncing, and extensive customization options.

```rust
use reactive_tui::widgets::*;

let mut autocomplete = Autocomplete::builder("country-search")
    .placeholder("Search countries...")
    .suggestions(vec![
        AutocompleteSuggestion::new("us", "United States").description("North America"),
        AutocompleteSuggestion::new("uk", "United Kingdom").description("Europe"),
        AutocompleteSuggestion::new("ca", "Canada").description("North America"),
    ])
    .filter_mode(FilterMode::Contains)
    .max_suggestions(10)
    .debounce_ms(300)
    .build();

autocomplete.set_query("uni");
let suggestions = autocomplete.get_visible_suggestions();
```

## Features

- **Search-as-you-type**: Real-time filtering of suggestions as user types
- **Multiple Filter Modes**: Contains, starts with, fuzzy matching, exact, and custom filtering
- **Keyboard Navigation**: Arrow keys, Enter/Escape, Tab completion support
- **Selection Modes**: Single selection or multi-selection with visual indicators
- **Rich Suggestions**: Support for descriptions, metadata, scores, and custom styling
- **Async Support**: Debounced input and async suggestion loading capabilities
- **Accessibility**: Full ARIA support and screen reader compatibility
- **Event System**: Comprehensive callbacks for selection, filtering, focus, and dropdown events
- **Customizable Styling**: CSS classes, themes, and custom rendering options
- **Performance Optimized**: Efficient filtering algorithms and suggestion scoring

## Core Components

### Autocomplete

Main autocomplete widget with suggestion management and interaction handling.

```rust
pub struct Autocomplete {
    pub id: String,
    pub suggestions: Vec<AutocompleteSuggestion>,
    pub state: Reactive<AutocompleteState>,
    pub config: AutocompleteConfig,
    pub style: AutocompleteStyle,
    pub callbacks: AutocompleteCallbacks,
    pub css_classes: Vec<String>,
    pub custom_filter: Option<CustomFilterCallback>,
}
```

### AutocompleteSuggestion

Individual suggestion with metadata and styling options.

```rust
pub struct AutocompleteSuggestion {
    pub id: SuggestionId,
    pub text: String,
    pub description: Option<String>,
    pub metadata: HashMap<String, String>,
    pub score: f32,
    pub disabled: bool,
    pub css_classes: Vec<String>,
}

impl AutocompleteSuggestion {
    pub fn new(id: impl Into<String>, text: impl Into<String>) -> Self
    pub fn description(mut self, description: impl Into<String>) -> Self
    pub fn metadata(mut self, key: impl Into<String>, value: impl Into<String>) -> Self
    pub fn score(mut self, score: f32) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
}
```

### AutocompleteState

State management for query, selections, and UI state.

```rust
pub struct AutocompleteState {
    pub query: String,
    pub selected_suggestions: Vec<SuggestionId>,
    pub highlighted_suggestion: Option<SuggestionId>,
    pub dropdown_open: bool,
    pub focused: bool,
    pub disabled: bool,
    pub filtered_suggestions: Vec<SuggestionId>,
    pub loading: bool,
}
```

### FilterMode

Different filtering algorithms for suggestion matching.

```rust
pub enum FilterMode {
    /// Match suggestions that contain the query (case-insensitive)
    Contains,
    /// Match suggestions that start with the query (case-insensitive)
    StartsWith,
    /// Fuzzy matching algorithm
    Fuzzy,
    /// Exact matching (case-sensitive)
    Exact,
    /// Custom filter function
    Custom,
}
```

### SelectionMode

Single or multiple selection behavior.

```rust
pub enum SelectionMode {
    /// Single selection (replaces current selection)
    Single,
    /// Multiple selection (adds to selection list)
    Multiple,
}
```

### Configuration

```rust
pub struct AutocompleteConfig {
    pub filter_mode: FilterMode,
    pub selection_mode: SelectionMode,
    pub max_suggestions: usize,
    pub min_query_length: usize,
    pub debounce_ms: u64,
    pub case_sensitive: bool,
    pub show_descriptions: bool,
    pub keyboard_navigation: bool,
    pub auto_select_first: bool,
    pub close_on_select: bool,
    pub clear_on_select: bool,
    pub placeholder: String,
}
```

### Styling

```rust
pub struct AutocompleteStyle {
    pub input_background: Option<ColorDefinition>,
    pub input_text_color: Option<ColorDefinition>,
    pub input_border_color: Option<ColorDefinition>,
    pub dropdown_background: Option<ColorDefinition>,
    pub suggestion_text_color: Option<ColorDefinition>,
    pub highlighted_background: Option<ColorDefinition>,
    pub selected_background: Option<ColorDefinition>,
    pub disabled_opacity: f32,
    pub max_dropdown_height: u16,
    pub suggestion_padding: u16,
}
```

## Builder Pattern

### AutocompleteBuilder

```rust
impl AutocompleteBuilder {
    pub fn new<S: Into<String>>(id: S) -> Self
    pub fn suggestions(mut self, suggestions: Vec<AutocompleteSuggestion>) -> Self
    pub fn suggestion(mut self, suggestion: AutocompleteSuggestion) -> Self
    pub fn placeholder(mut self, placeholder: impl Into<String>) -> Self
    pub fn filter_mode(mut self, mode: FilterMode) -> Self
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self
    pub fn max_suggestions(mut self, max: usize) -> Self
    pub fn min_query_length(mut self, min: usize) -> Self
    pub fn debounce_ms(mut self, ms: u64) -> Self
    pub fn case_sensitive(mut self, sensitive: bool) -> Self
    pub fn show_descriptions(mut self, show: bool) -> Self
    pub fn keyboard_navigation(mut self, enabled: bool) -> Self
    pub fn auto_select_first(mut self, enabled: bool) -> Self
    pub fn close_on_select(mut self, close: bool) -> Self
    pub fn clear_on_select(mut self, clear: bool) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn custom_filter<F>(mut self, filter: F) -> Self
    pub fn on_select<F>(mut self, callback: F) -> Self
    pub fn on_filter<F>(mut self, callback: F) -> Self
    pub fn on_change<F>(mut self, callback: F) -> Self
    pub fn on_focus<F>(mut self, callback: F) -> Self
    pub fn on_dropdown_toggle<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Autocomplete
}
```

## Methods

### Query Management

```rust
impl Autocomplete {
    // Set the current search query
    pub fn set_query(&mut self, query: impl Into<String>)
    
    // Get the current query
    pub fn get_query(&self) -> String
    
    // Filter suggestions based on current query
    pub fn filter_suggestions(&mut self)
}
```

### Selection Management

```rust
impl Autocomplete {
    // Select a suggestion by ID
    pub fn select_suggestion(&mut self, suggestion_id: impl AsRef<str>) -> Result<()>
    
    // Deselect a suggestion (for multi-select mode)
    pub fn deselect_suggestion(&mut self, suggestion_id: impl AsRef<str>)
    
    // Get currently selected suggestions
    pub fn get_selected_suggestions(&self) -> Vec<String>
    
    // Clear all selections
    pub fn clear_selections(&mut self)
    
    // Get visible suggestions (filtered)
    pub fn get_visible_suggestions(&self) -> Vec<&AutocompleteSuggestion>
}
```

### Navigation Control

```rust
impl Autocomplete {
    // Highlight next suggestion
    pub fn highlight_next(&mut self) -> Result<()>
    
    // Highlight previous suggestion
    pub fn highlight_previous(&mut self) -> Result<()>
    
    // Select the currently highlighted suggestion
    pub fn select_highlighted(&mut self) -> Result<()>
}
```

### Dropdown Management

```rust
impl Autocomplete {
    // Open the dropdown
    pub fn open_dropdown(&mut self)
    
    // Close the dropdown
    pub fn close_dropdown(&mut self)
    
    // Toggle dropdown open/closed
    pub fn toggle_dropdown(&mut self)
}
```

### State Management

```rust
impl Autocomplete {
    // Set focus state
    pub fn set_focused(&mut self, focused: bool)
    
    // Check if autocomplete is focused
    pub fn is_focused(&self) -> bool
    
    // Enable/disable the autocomplete
    pub fn set_disabled(&mut self, disabled: bool)
    
    // Check if autocomplete is disabled
    pub fn is_disabled(&self) -> bool
    
    // Set loading state
    pub fn set_loading(&mut self, loading: bool)
    
    // Check if autocomplete is loading
    pub fn is_loading(&self) -> bool
}
```

### Suggestion Management

```rust
impl Autocomplete {
    // Add suggestions to the list
    pub fn add_suggestions(&mut self, suggestions: Vec<AutocompleteSuggestion>)
    
    // Clear all suggestions
    pub fn clear_suggestions(&mut self)
    
    // Get suggestion count
    pub fn suggestion_count(&self) -> usize
    
    // Get filtered suggestion count
    pub fn filtered_count(&self) -> usize
}
```

## Examples

### Basic Country Search

```rust
use reactive_tui::widgets::*;

let mut country_search = Autocomplete::builder("countries")
    .placeholder("Search countries...")
    .suggestions(vec![
        AutocompleteSuggestion::new("us", "United States")
            .description("North America")
            .metadata("code", "US")
            .metadata("population", "331000000"),
        AutocompleteSuggestion::new("uk", "United Kingdom")
            .description("Europe")
            .metadata("code", "GB")
            .metadata("population", "67000000"),
        AutocompleteSuggestion::new("ca", "Canada")
            .description("North America") 
            .metadata("code", "CA")
            .metadata("population", "38000000"),
        AutocompleteSuggestion::new("fr", "France")
            .description("Europe")
            .metadata("code", "FR")
            .metadata("population", "68000000"),
    ])
    .filter_mode(FilterMode::Contains)
    .max_suggestions(5)
    .case_sensitive(false)
    .build();

// User interaction
country_search.set_query("uni");
let matches = country_search.get_visible_suggestions();
println!("Found {} matches", matches.len());
```

### Programming Language Autocomplete

```rust
let language_search = Autocomplete::builder("language-search")
    .placeholder("Search programming languages...")
    .suggestions(vec![
        AutocompleteSuggestion::new("rust", "Rust")
            .description("Systems programming language")
            .score(0.9)
            .metadata("year", "2010")
            .metadata("paradigm", "Systems"),
        AutocompleteSuggestion::new("typescript", "TypeScript")
            .description("JavaScript with static types")
            .score(0.8)
            .metadata("year", "2012")
            .metadata("paradigm", "Web"),
        AutocompleteSuggestion::new("python", "Python")
            .description("General-purpose programming")
            .score(0.85)
            .metadata("year", "1991")
            .metadata("paradigm", "General"),
        AutocompleteSuggestion::new("javascript", "JavaScript")
            .description("Web development language")
            .score(0.75)
            .metadata("year", "1995")
            .metadata("paradigm", "Web"),
    ])
    .filter_mode(FilterMode::StartsWith)
    .show_descriptions(true)
    .auto_select_first(true)
    .build();
```

### Multi-Select User Search

```rust
let user_search = Autocomplete::builder("user-search")
    .placeholder("Search users...")
    .suggestions(vec![
        AutocompleteSuggestion::new("user1", "Alice Johnson")
            .description("alice@company.com")
            .metadata("email", "alice@company.com")
            .metadata("department", "Engineering"),
        AutocompleteSuggestion::new("user2", "Bob Smith")
            .description("bob@company.com")
            .metadata("email", "bob@company.com")
            .metadata("department", "Design"),
        AutocompleteSuggestion::new("user3", "Carol Davis")
            .description("carol@company.com")
            .metadata("email", "carol@company.com")
            .metadata("department", "Marketing"),
    ])
    .selection_mode(SelectionMode::Multiple)
    .max_suggestions(8)
    .close_on_select(false)
    .on_select(|id, suggestion| {
        println!("Selected user: {} ({})", suggestion.text, id);
    })
    .build();
```

### Custom Filter Example

```rust
let custom_autocomplete = Autocomplete::builder("custom-filter")
    .placeholder("Search with custom logic...")
    .suggestions(vec![
        AutocompleteSuggestion::new("item1", "Important Task")
            .metadata("priority", "high")
            .score(0.9),
        AutocompleteSuggestion::new("item2", "Regular Task")
            .metadata("priority", "medium")
            .score(0.5),
        AutocompleteSuggestion::new("item3", "Low Priority Task")
            .metadata("priority", "low")
            .score(0.2),
    ])
    .filter_mode(FilterMode::Custom)
    .custom_filter(|query, suggestion| {
        // Custom logic: prioritize high-priority items
        let priority_match = suggestion.metadata.get("priority")
            .map(|p| p.contains(&query.to_lowercase()))
            .unwrap_or(false);
        
        let text_match = suggestion.text.to_lowercase().contains(&query.to_lowercase());
        
        priority_match || text_match
    })
    .build();
```

### Async Search with Debouncing

```rust
use reactive_tui::{widgets::*, reactive::Reactive};
use tokio::time::{sleep, Duration};

let search_results = Reactive::new(Vec::<AutocompleteSuggestion>::new());
let results_clone = search_results.clone();

let async_search = Autocomplete::builder("async-search")
    .placeholder("Search API...")
    .debounce_ms(500)
    .min_query_length(2)
    .on_filter(move |query| {
        // Simulate async API call
        let query = query.to_string();
        let results = results_clone.clone();
        
        tokio::spawn(async move {
            sleep(Duration::from_millis(200)).await;
            
            // Simulate API response
            let api_results = vec![
                AutocompleteSuggestion::new("api1", format!("API result for '{}'", query))
                    .description("From API endpoint")
                    .score(0.8),
                AutocompleteSuggestion::new("api2", format!("Another result for '{}'", query)) 
                    .description("From different endpoint")
                    .score(0.6),
            ];
            
            results.set(api_results);
        });
        
        // Return current results immediately
        results.get()
    })
    .on_change(|query| {
        println!("Search query changed: {}", query);
    })
    .build();
```

### Command Line Interface

```rust
let cli_autocomplete = Autocomplete::builder("cli-commands")
    .placeholder("Type command...")
    .suggestions(vec![
        AutocompleteSuggestion::new("help", "help")
            .description("Show help information"),
        AutocompleteSuggestion::new("exit", "exit")
            .description("Exit the application"),
        AutocompleteSuggestion::new("save", "save <filename>")
            .description("Save current work"),
        AutocompleteSuggestion::new("load", "load <filename>")
            .description("Load saved work"),
        AutocompleteSuggestion::new("settings", "settings")
            .description("Open configuration"),
        AutocompleteSuggestion::new("clear", "clear")
            .description("Clear the screen"),
    ])
    .filter_mode(FilterMode::StartsWith)
    .auto_select_first(true)
    .close_on_select(true)
    .clear_on_select(true)
    .keyboard_navigation(true)
    .on_select(|id, suggestion| {
        println!("Executing command: {}", suggestion.text);
        execute_command(id);
    })
    .build();
```

### File Path Autocomplete

```rust
use std::fs;

let file_autocomplete = Autocomplete::builder("file-search")
    .placeholder("Enter file path...")
    .min_query_length(1)
    .debounce_ms(200)
    .on_filter(|query| {
        // Dynamic file system search
        let mut suggestions = Vec::new();
        
        if let Ok(entries) = fs::read_dir(".") {
            for entry in entries.flatten() {
                let path = entry.path();
                let path_str = path.to_string_lossy();
                
                if path_str.contains(query) {
                    let suggestion = AutocompleteSuggestion::new(
                        path_str.to_string(),
                        path.file_name().unwrap_or_default().to_string_lossy().to_string()
                    )
                    .description(if path.is_dir() { "Directory" } else { "File" })
                    .metadata("type", if path.is_dir() { "dir" } else { "file" })
                    .class(if path.is_dir() { "file-dir" } else { "file-item" });
                    
                    suggestions.push(suggestion);
                }
            }
        }
        
        suggestions
    })
    .on_select(|_id, suggestion| {
        if suggestion.metadata.get("type") == Some(&"dir".to_string()) {
            println!("Navigating to directory: {}", suggestion.text);
        } else {
            println!("Opening file: {}", suggestion.text);
        }
    })
    .build();
```

### Fuzzy Search Example

```rust
let fuzzy_search = Autocomplete::builder("fuzzy-search")
    .placeholder("Fuzzy search...")
    .suggestions(vec![
        AutocompleteSuggestion::new("javascript", "JavaScript"),
        AutocompleteSuggestion::new("typescript", "TypeScript"),
        AutocompleteSuggestion::new("coffeescript", "CoffeeScript"),
        AutocompleteSuggestion::new("livescript", "LiveScript"),
        AutocompleteSuggestion::new("actionscript", "ActionScript"),
    ])
    .filter_mode(FilterMode::Fuzzy)
    .max_suggestions(3)
    .build();

// User types "jvspt" - will match "JavaScript"
fuzzy_search.set_query("jvspt");
```

### Event-Driven Autocomplete

```rust
use reactive_tui::{widgets::*, reactive::Reactive};

let current_selection = Reactive::new(String::new());
let dropdown_state = Reactive::new(false);
let selection_clone = current_selection.clone();
let dropdown_clone = dropdown_state.clone();

let event_autocomplete = Autocomplete::builder("event-driven")
    .placeholder("Search with events...")
    .suggestions(vec![
        AutocompleteSuggestion::new("item1", "First Item"),
        AutocompleteSuggestion::new("item2", "Second Item"),
        AutocompleteSuggestion::new("item3", "Third Item"),
    ])
    .on_select(move |id, suggestion| {
        selection_clone.set(suggestion.text.clone());
        println!("Selected: {} (ID: {})", suggestion.text, id);
    })
    .on_change(|query| {
        println!("Query changed: '{}'", query);
    })
    .on_focus(|focused| {
        println!("Focus changed: {}", if focused { "gained" } else { "lost" });
    })
    .on_dropdown_toggle(move |open| {
        dropdown_clone.set(open);
        println!("Dropdown {}", if open { "opened" } else { "closed" });
    })
    .build();
```

## Convenience Functions

Pre-configured autocomplete functions for common use cases:

```rust
// Country search autocomplete
pub fn country_autocomplete() -> Autocomplete

// Programming language autocomplete
pub fn language_autocomplete() -> Autocomplete

// User search autocomplete
pub fn user_autocomplete(users: Vec<(&str, &str, &str)>) -> Autocomplete

// Command line autocomplete
pub fn command_autocomplete() -> Autocomplete
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let search_form = Element::with_tag("div")
    .class("search-container")
    .child(
        Element::with_tag("label")
            .text("Search:")
            .build()
    )
    .child(
        autocomplete.to_element()
    )
    .child(
        Element::with_tag("div")
            .class("search-results")
            .text(&format!("Found {} results", autocomplete.filtered_count()))
            .build()
    )
    .build();
```

## CSS Styling

The autocomplete generates semantic CSS classes:

```css
.autocomplete {
    /* Base autocomplete styles */
}

.autocomplete-focused {
    /* Focused state */
}

.autocomplete-disabled {
    /* Disabled state */
}

.autocomplete-loading {
    /* Loading state */
}

.autocomplete-input {
    /* Input field styles */
}

.autocomplete-dropdown {
    /* Dropdown container */
}

.autocomplete-suggestion {
    /* Individual suggestion */
}

.autocomplete-suggestion-highlighted {
    /* Highlighted suggestion */
}

.autocomplete-suggestion-selected {
    /* Selected suggestion */
}

.autocomplete-suggestion-disabled {
    /* Disabled suggestion */
}
```

## Performance Considerations

- **Debounced Input**: Configurable debounce delay prevents excessive filtering during rapid typing
- **Efficient Filtering**: Optimized filtering algorithms with early termination
- **Score-based Ranking**: Suggestions sorted by relevance score for better user experience
- **Virtual Scrolling**: For large suggestion lists (1000+ items)
- **Memory Management**: Automatic cleanup of filtered results and state

## Accessibility

- **ARIA Attributes**: Full ARIA support with proper labeling and live regions
- **Screen Reader**: Announces suggestion changes and selection events
- **Keyboard Navigation**: Complete keyboard accessibility with arrow keys, Enter, Escape
- **Focus Management**: Proper focus handling and visual indicators

## Advanced Features

### Custom Filtering

```rust
let advanced_filter = Autocomplete::builder("advanced")
    .custom_filter(|query, suggestion| {
        // Implement sophisticated matching logic
        let query_lower = query.to_lowercase();
        let text_lower = suggestion.text.to_lowercase();
        
        // Exact match gets highest priority
        if text_lower == query_lower {
            return true;
        }
        
        // Starts with match
        if text_lower.starts_with(&query_lower) {
            return true;
        }
        
        // Word boundary match
        for word in text_lower.split_whitespace() {
            if word.starts_with(&query_lower) {
                return true;
            }
        }
        
        // Fuzzy match as fallback
        fuzzy_match(&query_lower, &text_lower)
    })
    .build();
```

### Dynamic Suggestion Loading

```rust
let dynamic_search = Autocomplete::builder("dynamic")
    .on_filter(|query| {
        // Load suggestions based on query context
        match query.len() {
            1..=2 => load_basic_suggestions(query),
            3..=5 => load_detailed_suggestions(query),
            _ => load_comprehensive_suggestions(query),
        }
    })
    .build();
```

The Autocomplete widget provides comprehensive search-as-you-type functionality with extensive customization options, efficient filtering algorithms, and seamless integration with reactive state management and accessibility features.