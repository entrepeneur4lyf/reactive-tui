---
sidebar_position: 5
---

# State Management API

Reactive state management system with JSON support and observer patterns.

## ReactiveState

Core state management with thread-safe access and change notifications.

### Basic Usage

```rust
use reactive_tui::reactive::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct AppState {
    counter: i32,
    user: Option<String>,
    settings: Settings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Settings {
    theme: String,
    language: String,
    notifications: bool,
}

// Create reactive state
let state = ReactiveState::new(AppState {
    counter: 0,
    user: None,
    settings: Settings {
        theme: "dark".to_string(),
        language: "en".to_string(),
        notifications: true,
    },
});
```

### State Operations

```rust
// Read state
let current_state = state.get();
println!("Counter: {}", current_state.counter);

// Update state
state.update(|s| {
    s.counter += 1;
    s.user = Some("Alice".to_string());
});

// Replace entire state
let new_state = AppState {
    counter: 42,
    user: Some("Bob".to_string()),
    settings: Settings {
        theme: "light".to_string(),
        language: "fr".to_string(),
        notifications: false,
    },
};
state.set(new_state);

// Conditional updates
state.update_if(|s| s.counter < 100, |s| {
    s.counter += 10;
});
```

### JSON State Management

```rust
use serde_json::json;

// Set state from JSON
let json_state = json!({
    "counter": 25,
    "user": "Charlie",
    "settings": {
        "theme": "dark",
        "language": "es",
        "notifications": true
    }
});

state.set_state_json(&json_state)?;

// Get state as JSON
let current_json = state.get_state_json()?;
println!("Current state: {}", current_json);

// Nested updates with dot notation
state.set_nested_value("settings.theme", &json!("light"))?;
state.set_nested_value("counter", &json!(30))?;

// Get nested values
let theme = state.get_nested_value("settings.theme")?;
let counter = state.get_nested_value("counter")?;
```

### State Watching

```rust
// Watch for changes
let watcher_id = state.watch(|old_state, new_state| {
    if old_state.counter != new_state.counter {
        println!("Counter changed: {} -> {}", old_state.counter, new_state.counter);
    }
    
    if old_state.user != new_state.user {
        println!("User changed: {:?} -> {:?}", old_state.user, new_state.user);
    }
});

// Unwatch when no longer needed
state.unwatch(watcher_id);

// Watch specific fields
let counter_watcher = state.watch_field("counter", |old_value, new_value| {
    println!("Counter: {} -> {}", old_value, new_value);
});

// Async watchers
let async_watcher = state.watch_async(|old_state, new_state| async move {
    if new_state.counter > 50 {
        // Perform async operation
        save_state_to_db(&new_state).await?;
    }
    Ok(())
});
```

## Reactive Components

Integrate state management with components.

### ReactiveComponent

```rust
use reactive_tui::reactive::*;

#[derive(Debug)]
struct CounterComponent {
    state: ReactiveState<AppState>,
    watcher_id: Option<u64>,
}

impl ReactiveComponent<AppState> for CounterComponent {
    fn new(state: ReactiveState<AppState>) -> Self {
        Self {
            state,
            watcher_id: None,
        }
    }
    
    fn state(&self) -> &ReactiveState<AppState> {
        &self.state
    }
    
    fn on_state_change(&mut self, old_state: &AppState, new_state: &AppState) {
        if old_state.counter != new_state.counter {
            // Trigger re-render or other side effects
            self.request_render();
        }
    }
}

impl Component for CounterComponent {
    fn render(&self) -> Element {
        let state = self.state.get();
        
        Element::with_tag("div")
            .class("counter-component")
            .child(
                Element::with_tag("h1")
                    .content(&format!("Count: {}", state.counter))
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .content("Increment")
                    .on_click("increment")
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .content("Decrement")
                    .on_click("decrement")
                    .build()
            )
            .build()
    }
    
    fn handle_action(&mut self, action: &Action) -> Result<ActionResult> {
        match action.name() {
            "increment" => {
                self.state.update(|s| s.counter += 1);
                Ok(ActionResult::Handled)
            }
            "decrement" => {
                self.state.update(|s| s.counter -= 1);
                Ok(ActionResult::Handled)
            }
            _ => Ok(ActionResult::Ignored)
        }
    }
    
    fn mount(&mut self) -> Result<()> {
        // Start watching state changes
        let watcher_id = self.state.watch({
            let weak_self = Arc::downgrade(&Arc::new(Mutex::new(self)));
            move |old_state, new_state| {
                if let Some(component) = weak_self.upgrade() {
                    if let Ok(mut comp) = component.lock() {
                        comp.on_state_change(old_state, new_state);
                    }
                }
            }
        });
        self.watcher_id = Some(watcher_id);
        Ok(())
    }
    
    fn unmount(&mut self) -> Result<()> {
        // Stop watching state changes
        if let Some(watcher_id) = self.watcher_id.take() {
            self.state.unwatch(watcher_id);
        }
        Ok(())
    }
}
```

### State Binding

```rust
// Bind form inputs to state
struct FormComponent {
    state: ReactiveState<FormData>,
}

impl FormComponent {
    fn create_username_input(&self) -> Element {
        let current_value = self.state.get().username.clone();
        
        Element::with_tag("input")
            .id("username")
            .attr("type", "text")
            .attr("value", &current_value)
            .on_input("update_username")
            .build()
    }
    
    fn handle_username_change(&mut self, new_value: String) {
        self.state.update(|s| {
            s.username = new_value;
            s.is_valid = self.validate_form(&s);
        });
    }
}

// Two-way data binding helper
struct DataBinding<T> {
    state: ReactiveState<T>,
    field_path: String,
}

impl<T> DataBinding<T> 
where 
    T: Clone + serde::Serialize + serde::de::DeserializeOwned,
{
    pub fn new(state: ReactiveState<T>, field_path: &str) -> Self {
        Self {
            state,
            field_path: field_path.to_string(),
        }
    }
    
    pub fn get_value(&self) -> Result<serde_json::Value> {
        self.state.get_nested_value(&self.field_path)
    }
    
    pub fn set_value(&self, value: serde_json::Value) -> Result<()> {
        self.state.set_nested_value(&self.field_path, &value)
    }
    
    pub fn bind_to_input(&self, input_id: &str) -> Element {
        let current_value = self.get_value()
            .unwrap_or(serde_json::Value::Null);
        
        Element::with_tag("input")
            .id(input_id)
            .attr("value", &current_value.to_string())
            .on_input(&format!("update_{}", self.field_path.replace(".", "_")))
            .build()
    }
}
```

## Advanced State Patterns

### State Composition

```rust
// Combine multiple state objects
struct AppStateManager {
    user_state: ReactiveState<UserState>,
    ui_state: ReactiveState<UIState>,
    data_state: ReactiveState<DataState>,
}

impl AppStateManager {
    pub fn new() -> Self {
        Self {
            user_state: ReactiveState::new(UserState::default()),
            ui_state: ReactiveState::new(UIState::default()),
            data_state: ReactiveState::new(DataState::default()),
        }
    }
    
    // Computed properties across states
    pub fn is_ready(&self) -> bool {
        let user = self.user_state.get();
        let ui = self.ui_state.get();
        let data = self.data_state.get();
        
        user.is_authenticated && !ui.is_loading && data.is_loaded
    }
    
    // Cross-state reactions
    pub fn setup_state_reactions(&self) {
        // When user logs out, clear data
        self.user_state.watch({
            let data_state = self.data_state.clone();
            move |old_user, new_user| {
                if old_user.is_authenticated && !new_user.is_authenticated {
                    data_state.update(|s| s.clear());
                }
            }
        });
        
        // When data loads, update UI
        self.data_state.watch({
            let ui_state = self.ui_state.clone();
            move |_old_data, new_data| {
                ui_state.update(|s| {
                    s.is_loading = false;
                    s.has_data = !new_data.items.is_empty();
                });
            }
        });
    }
}
```

### State Persistence

```rust
use std::fs;
use std::path::Path;

impl ReactiveState<AppState> {
    // Save state to file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let state = self.get();
        let json = serde_json::to_string_pretty(&*state)?;
        fs::write(path, json)?;
        Ok(())
    }
    
    // Load state from file
    pub fn load_from_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let json = fs::read_to_string(path)?;
        let state: AppState = serde_json::from_str(&json)?;
        self.set(state);
        Ok(())
    }
    
    // Auto-save on changes
    pub fn enable_auto_save<P: AsRef<Path> + Clone + Send + 'static>(
        &self, 
        path: P
    ) -> u64 {
        self.watch(move |_old_state, new_state| {
            // Debounced save (implement debouncing logic)
            if let Err(e) = save_state_debounced(&path, new_state) {
                eprintln!("Failed to auto-save state: {}", e);
            }
        })
    }
}

// State versioning and migration
#[derive(Debug, Clone, Serialize, Deserialize)]
struct VersionedState {
    version: u32,
    data: serde_json::Value,
}

impl VersionedState {
    pub fn migrate_to_current(mut self) -> Result<AppState> {
        while self.version < CURRENT_STATE_VERSION {
            self = self.migrate_one_version()?;
        }
        
        Ok(serde_json::from_value(self.data)?)
    }
    
    fn migrate_one_version(mut self) -> Result<Self> {
        match self.version {
            1 => {
                // Migration from v1 to v2
                if let Some(obj) = self.data.as_object_mut() {
                    obj.insert("new_field".to_string(), json!("default_value"));
                }
                self.version = 2;
            }
            2 => {
                // Migration from v2 to v3
                // ... migration logic
                self.version = 3;
            }
            _ => return Err(TuiError::Config("Unknown state version".into())),
        }
        Ok(self)
    }
}
```

### State Validation

```rust
use schemars::{JsonSchema, schema::RootSchema};

#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
struct ValidatedState {
    #[schemars(range(min = 0, max = 100))]
    counter: i32,
    
    #[schemars(regex = "^[a-zA-Z0-9_]+$")]
    username: String,
    
    #[schemars(email)]
    email: Option<String>,
}

impl ReactiveState<ValidatedState> {
    pub fn update_with_validation<F>(&self, updater: F) -> Result<()>
    where
        F: FnOnce(&mut ValidatedState),
    {
        let mut new_state = (*self.get()).clone();
        updater(&mut new_state);
        
        // Validate against schema
        self.validate_state(&new_state)?;
        
        self.set(new_state);
        Ok(())
    }
    
    fn validate_state(&self, state: &ValidatedState) -> Result<()> {
        // Counter validation
        if state.counter < 0 || state.counter > 100 {
            return Err(TuiError::Config("Counter must be between 0 and 100".into()));
        }
        
        // Username validation
        if !state.username.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(TuiError::Config("Username contains invalid characters".into()));
        }
        
        // Email validation
        if let Some(email) = &state.email {
            if !email.contains('@') || !email.contains('.') {
                return Err(TuiError::Config("Invalid email format".into()));
            }
        }
        
        Ok(())
    }
}
```

### State Debugging

```rust
impl ReactiveState<AppState> {
    // Debug state changes
    pub fn enable_debug_logging(&self) -> u64 {
        self.watch(|old_state, new_state| {
            println!("State change detected:");
            println!("  Old: {:?}", old_state);
            println!("  New: {:?}", new_state);
            
            // Log specific field changes
            if old_state.counter != new_state.counter {
                println!("  Counter: {} -> {}", old_state.counter, new_state.counter);
            }
            
            if old_state.user != new_state.user {
                println!("  User: {:?} -> {:?}", old_state.user, new_state.user);
            }
        })
    }
    
    // State history tracking
    pub fn enable_history_tracking(&self, max_history: usize) -> u64 {
        let history = Arc::new(Mutex::new(VecDeque::with_capacity(max_history)));
        
        self.watch({
            let history = Arc::clone(&history);
            move |_old_state, new_state| {
                let mut hist = history.lock().unwrap();
                
                if hist.len() >= max_history {
                    hist.pop_front();
                }
                
                hist.push_back(StateSnapshot {
                    timestamp: SystemTime::now(),
                    state: new_state.clone(),
                });
            }
        })
    }
    
    // Performance monitoring
    pub fn monitor_performance(&self) -> u64 {
        self.watch(|_old_state, _new_state| {
            let start = Instant::now();
            // Monitor how long state changes take
            let duration = start.elapsed();
            
            if duration > Duration::from_millis(10) {
                println!("Warning: State change took {:?}", duration);
            }
        })
    }
}

#[derive(Debug, Clone)]
struct StateSnapshot<T> {
    timestamp: SystemTime,
    state: T,
}
```

## Integration with Widgets

### State-Driven Widgets

```rust
// Create widgets that automatically update with state
fn create_state_driven_ui(state: ReactiveState<AppState>) -> Element {
    let current_state = state.get();
    
    Element::with_tag("div")
        .class("app")
        .child(create_header(&current_state))
        .child(create_counter(&state))
        .child(create_user_info(&current_state))
        .child(create_settings(&state))
        .build()
}

fn create_counter(state: &ReactiveState<AppState>) -> Element {
    let current_count = state.get().counter;
    
    Element::with_tag("div")
        .class("counter")
        .child(
            Element::with_tag("span")
                .content(&format!("Count: {}", current_count))
                .build()
        )
        .child(
            button("increment", |config| {
                config.text("+")
                      .on_click("increment_counter")
            }).to_element()
        )
        .child(
            button("decrement", |config| {
                config.text("-")
                      .on_click("decrement_counter")
            }).to_element()
        )
        .build()
}

// State-aware form
fn create_form_with_state(state: ReactiveState<FormState>) -> Element {
    let form_state = state.get();
    
    Element::with_tag("form")
        .class("form")
        .child(
            input("username", |config| {
                config.value(&form_state.username)
                      .placeholder("Enter username")
                      .required(true)
                      .on_change("update_username")
            }).to_element()
        )
        .child(
            input("email", |config| {
                config.value(&form_state.email)
                      .placeholder("Enter email")
                      .input_type("email")
                      .on_change("update_email")
            }).to_element()
        )
        .child(
            button("submit", |config| {
                config.text("Submit")
                      .variant("primary")
                      .disabled(!form_state.is_valid)
                      .on_click("submit_form")
            }).to_element()
        )
        .build()
}
```