# Screens Module

Advanced screen management system with navigation, transitions, state preservation, and multi-screen applications for complex terminal user interfaces.

## Screen

Individual screen representation containing viewports, widgets, and application state.

```rust
use reactive_tui::screens::{Screen, ScreenBuilder, ScreenState};

let home_screen = ScreenBuilder::new("home")
    .title("Home Screen")
    .description("Main application screen")
    .add_viewport("main", main_viewport)
    .add_viewport("sidebar", sidebar_viewport)
    .add_widget("header", header_widget)
    .add_widget("footer", footer_widget)
    .state(ScreenState::Active)
    .build();
```

### Screen Configuration

```rust
pub struct ScreenConfig {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub layout: LayoutType,
    pub resizable: bool,
    pub modal: bool,
    pub persistent: bool,
    pub cache_strategy: CacheStrategy,
}

pub enum LayoutType {
    Fixed,
    Flexible,
    Grid(u16, u16),
    Tabs,
    Stack,
    Custom(Box<dyn Layout>),
}
```

## ScreenManager

Central screen orchestration managing navigation, transitions, and screen lifecycle.

```rust
use reactive_tui::screens::{ScreenManager, NavigationEvent};

let mut manager = ScreenManager::new();

// Register screens
manager.register(home_screen);
manager.register(settings_screen);
manager.register(about_screen);

// Set initial screen
manager.set_initial_screen("home")?;

// Navigation
manager.navigate_to("settings", None)?;
manager.navigate_back()?;
manager.navigate_forward()?;
```

### Navigation History

```rust
use reactive_tui::screens::{NavigationHistory, HistoryEntry};

let history = manager.get_navigation_history();

// History operations
println!("Current screen: {}", history.current().screen_id);
println!("Can go back: {}", history.can_go_back());
println!("Can go forward: {}", history.can_go_forward());

// Browse history
for entry in history.entries() {
    println!("Screen: {}, Timestamp: {:?}", entry.screen_id, entry.timestamp);
}
```

## Screen Transitions

### Transition Types

```rust
use reactive_tui::screens::{ScreenTransition, TransitionDirection, TransitionTiming};

// Slide transitions
let slide_left = ScreenTransition::slide(TransitionDirection::Left)
    .duration(300)
    .easing(EasingFunction::EaseInOut);

// Fade transitions
let fade_transition = ScreenTransition::fade()
    .duration(250)
    .timing(TransitionTiming::EaseIn);

// Custom transitions
let custom_transition = ScreenTransition::custom(Box::new(|from, to, progress| {
    // Custom transition implementation
    apply_custom_effect(from, to, progress);
}));
```

### Transition Events

```rust
use reactive_tui::screens::{TransitionEvent, TransitionListener};

struct MyTransitionListener;

impl TransitionListener for MyTransitionListener {
    fn on_transition_start(&mut self, event: &TransitionEvent) {
        println!("Transition started: {} -> {}", event.from, event.to);
    }
    
    fn on_transition_update(&mut self, event: &TransitionEvent, progress: f32) {
        update_progress_indicator(progress);
    }
    
    fn on_transition_complete(&mut self, event: &TransitionEvent) {
        println!("Transition completed: {} -> {}", event.from, event.to);
        cleanup_previous_screen(&event.from);
    }
}

manager.add_transition_listener(Box::new(MyTransitionListener));
```

## Screen State Management

### State Preservation

```rust
use reactive_tui::screens::{ScreenState, StateManager, SerializableState};

struct MyScreenState {
    user_input: String,
    selected_item: usize,
    scroll_position: Point,
}

impl SerializableState for MyScreenState {
    fn serialize(&self) -> Result<Vec<u8>, StateError> {
        // Serialize state to bytes
        Ok(bincode::serialize(self)?)
    }
    
    fn deserialize(data: &[u8]) -> Result<Self, StateError> {
        // Deserialize state from bytes
        Ok(bincode::deserialize(data)?)
    }
}

// Save screen state
let state = MyScreenState {
    user_input: "Hello".to_string(),
    selected_item: 2,
    scroll_position: Point::new(0, 10),
};

manager.save_screen_state("settings", Box::new(state))?;

// Restore screen state
if let Some(state) = manager.restore_screen_state::<MyScreenState>("settings")? {
    apply_restored_state(state);
}
```

### State Lifecycle

```rust
use reactive_tui::screens::{ScreenLifecycle, LifecycleEvent};

impl ScreenLifecycle for MyScreen {
    fn on_create(&mut self) -> Result<(), ScreenError> {
        // Initialize screen resources
        self.load_initial_data()?;
        Ok(())
    }
    
    fn on_show(&mut self) -> Result<(), ScreenError> {
        // Screen becomes visible
        self.start_background_tasks()?;
        Ok(())
    }
    
    fn on_hide(&mut self) -> Result<(), ScreenError> {
        // Screen becomes hidden
        self.pause_background_tasks()?;
        Ok(())
    }
    
    fn on_destroy(&mut self) -> Result<(), ScreenError> {
        // Clean up resources
        self.cleanup_resources()?;
        Ok(())
    }
    
    fn on_pause(&mut self) -> Result<(), ScreenError> {
        // Screen paused (background)
        self.save_state()?;
        Ok(())
    }
    
    fn on_resume(&mut self) -> Result<(), ScreenError> {
        // Screen resumed (foreground)
        self.restore_state()?;
        Ok(())
    }
}
```

## Modal Screens

### Modal Management

```rust
use reactive_tui::screens::{ModalScreen, ModalResult, Backdrop};

let confirmation_modal = ModalScreen::builder("confirm_delete")
    .title("Confirm Deletion")
    .content("Are you sure you want to delete this item?")
    .backdrop(Backdrop::Blur)
    .buttons(vec!["Cancel", "Delete"])
    .default_button(0)
    .build();

// Show modal and wait for result
let result = manager.show_modal(confirmation_modal).await?;

match result {
    ModalResult::Button(0) => {
        // Cancel pressed
        println!("Deletion cancelled");
    },
    ModalResult::Button(1) => {
        // Delete pressed
        perform_deletion();
    },
    ModalResult::Dismissed => {
        // Modal dismissed (ESC key)
        println!("Modal dismissed");
    },
}
```

### Modal Stack

```rust
use reactive_tui::screens::ModalStack;

let modal_stack = manager.get_modal_stack();

// Push multiple modals
modal_stack.push(info_modal)?;
modal_stack.push(confirmation_modal)?;

// Current modal
let current = modal_stack.current();
println!("Current modal: {}", current.id);

// Pop modals
let popped = modal_stack.pop()?;
```

## Screen Layout

### Layout Managers

```rust
use reactive_tui::screens::{LayoutManager, GridLayout, FlexLayout};

// Grid layout
let grid_layout = GridLayout::new(3, 2) // 3 columns, 2 rows
    .gap(1)
    .padding(2)
    .cell(0, 0, "header")
    .cell(0, 1, "sidebar")
    .cell(1, 0, "main")
    .cell(1, 1, "details")
    .cell(2, 0, "footer")
    .span(2, 1, 2); // Footer spans 2 columns

screen.set_layout(Box::new(grid_layout));

// Flex layout
let flex_layout = FlexLayout::new()
    .direction(FlexDirection::Column)
    .add_item("header", FlexItem::fixed(3))
    .add_item("main", FlexItem::flex(1))
    .add_item("footer", FlexItem::fixed(2));

screen.set_layout(Box::new(flex_layout));
```

### Responsive Layout

```rust
use reactive_tui::screens::{ResponsiveLayout, Breakpoint};

let responsive_layout = ResponsiveLayout::new()
    .breakpoint(Breakpoint::new(40, 20), mobile_layout)
    .breakpoint(Breakpoint::new(80, 24), tablet_layout)
    .breakpoint(Breakpoint::new(120, 40), desktop_layout)
    .default(desktop_layout);

screen.set_layout(Box::new(responsive_layout));
```

## Screen Events

### Event Handling

```rust
use reactive_tui::screens::{ScreenEvent, ScreenEventHandler};

struct MyScreenEventHandler;

impl ScreenEventHandler for MyScreenEventHandler {
    fn handle_screen_activated(&mut self, screen_id: &str) {
        println!("Screen activated: {}", screen_id);
        analytics::track_screen_view(screen_id);
    }
    
    fn handle_screen_deactivated(&mut self, screen_id: &str) {
        println!("Screen deactivated: {}", screen_id);
        save_screen_state(screen_id);
    }
    
    fn handle_navigation_request(&mut self, from: &str, to: &str) -> bool {
        // Return true to allow navigation, false to block
        validate_navigation(from, to)
    }
}

manager.set_event_handler(Box::new(MyScreenEventHandler));
```

### Custom Events

```rust
use reactive_tui::screens::{CustomScreenEvent, EventBus};

// Define custom events
#[derive(Debug, Clone)]
pub enum AppEvent {
    UserLoggedIn(String),
    DataRefreshRequired,
    ThemeChanged(String),
}

// Send custom events
let event_bus = manager.get_event_bus();
event_bus.emit(AppEvent::UserLoggedIn("john_doe".to_string()));

// Listen for custom events
event_bus.subscribe(|event: &AppEvent| {
    match event {
        AppEvent::UserLoggedIn(username) => {
            update_user_interface(username);
        },
        AppEvent::DataRefreshRequired => {
            refresh_all_screens();
        },
        AppEvent::ThemeChanged(theme) => {
            apply_theme_to_screens(theme);
        },
    }
});
```

## Screen Caching

### Cache Strategies

```rust
use reactive_tui::screens::{CacheStrategy, CacheManager};

pub enum CacheStrategy {
    None,           // No caching
    Memory,         // Keep in memory
    Disk,           // Serialize to disk
    Adaptive,       // Memory first, then disk
    LRU(usize),     // LRU with size limit
}

let cache_manager = CacheManager::new()
    .strategy(CacheStrategy::LRU(10))
    .memory_limit(64 * 1024 * 1024) // 64MB
    .disk_cache_dir("/tmp/screen_cache");

manager.set_cache_manager(cache_manager);
```

### Preloading

```rust
use reactive_tui::screens::ScreenPreloader;

let preloader = ScreenPreloader::new()
    .add_screen("settings", SettingsScreen::new())
    .add_screen("about", AboutScreen::new())
    .preload_strategy(PreloadStrategy::Eager);

// Preload screens in background
preloader.preload_all().await?;

// Screens are now instantly available
manager.navigate_to("settings", None)?; // Instant navigation
```

## Multi-Window Support

### Window Management

```rust
use reactive_tui::screens::{WindowManager, Window, WindowConfig};

let mut window_manager = WindowManager::new();

// Create windows
let main_window = Window::new(WindowConfig {
    id: "main".to_string(),
    title: "Main Window".to_string(),
    position: Point::new(0, 0),
    size: Size::new(80, 24),
    resizable: true,
    modal: false,
});

let popup_window = Window::new(WindowConfig {
    id: "popup".to_string(),
    title: "Popup".to_string(),
    position: Point::new(20, 8),
    size: Size::new(40, 8),
    resizable: false,
    modal: true,
});

window_manager.add_window(main_window);
window_manager.add_window(popup_window);

// Window focus management
window_manager.focus_window("popup")?;
let focused = window_manager.get_focused_window();
```

## Integration Examples

### Multi-Screen Application

```rust
use reactive_tui::screens::{ScreenManager, Screen, ScreenTransition};

fn create_application() -> Result<ScreenManager, ScreenError> {
    let mut manager = ScreenManager::new();
    
    // Home screen
    let home = Screen::builder("home")
        .title("Dashboard")
        .add_widget("welcome", create_welcome_widget())
        .add_widget("stats", create_stats_widget())
        .add_widget("recent", create_recent_items_widget())
        .build();
    
    // Settings screen
    let settings = Screen::builder("settings")
        .title("Settings")
        .add_widget("form", create_settings_form())
        .add_widget("preview", create_preview_widget())
        .build();
    
    // About screen
    let about = Screen::builder("about")
        .title("About")
        .add_widget("info", create_info_widget())
        .build();
    
    manager.register(home);
    manager.register(settings);
    manager.register(about);
    
    // Set up transitions
    let slide_transition = ScreenTransition::slide(TransitionDirection::Left)
        .duration(250);
    manager.set_default_transition(slide_transition);
    
    manager.set_initial_screen("home")?;
    
    Ok(manager)
}

// Main application loop
fn run_application() -> Result<(), ScreenError> {
    let mut manager = create_application()?;
    
    loop {
        // Handle events
        if let Ok(event) = poll_event() {
            match event {
                Event::Key(KeyEvent { code: KeyCode::F1, .. }) => {
                    manager.navigate_to("home", None)?;
                },
                Event::Key(KeyEvent { code: KeyCode::F2, .. }) => {
                    manager.navigate_to("settings", None)?;
                },
                Event::Key(KeyEvent { code: KeyCode::F3, .. }) => {
                    manager.navigate_to("about", None)?;
                },
                Event::Key(KeyEvent { code: KeyCode::Esc, .. }) => {
                    if manager.can_navigate_back() {
                        manager.navigate_back()?;
                    }
                },
                _ => {
                    manager.handle_event(event)?;
                }
            }
        }
        
        // Render current screen
        manager.render()?;
        
        // Check for exit condition
        if manager.should_exit() {
            break;
        }
    }
    
    Ok(())
}
```

### Wizard Interface

```rust
use reactive_tui::screens::{WizardManager, WizardStep, WizardNavigation};

fn create_setup_wizard() -> WizardManager {
    let mut wizard = WizardManager::new("setup_wizard");
    
    // Step 1: Welcome
    let welcome_step = WizardStep::new("welcome")
        .title("Welcome to Setup")
        .content("This wizard will guide you through setup")
        .navigation(WizardNavigation::NextOnly);
    
    // Step 2: Configuration
    let config_step = WizardStep::new("config")
        .title("Configuration")
        .content("Please configure your settings")
        .validation(Box::new(|data| validate_config(data)))
        .navigation(WizardNavigation::BackNext);
    
    // Step 3: Confirmation
    let confirm_step = WizardStep::new("confirm")
        .title("Confirmation")
        .content("Please review your settings")
        .navigation(WizardNavigation::BackFinish);
    
    wizard.add_step(welcome_step);
    wizard.add_step(config_step);
    wizard.add_step(confirm_step);
    
    wizard
}
```