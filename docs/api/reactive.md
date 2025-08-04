---
title: Reactive Module
description: Thread-safe reactive state system with automatic change detection and UI updates
sidebar_position: 2
---

# Reactive Module

Thread-safe reactive state system with automatic change detection and UI updates.

This module provides a React-like state management system for terminal applications, enabling automatic UI updates when state changes. The reactive system uses Arc/RwLock for thread safety and broadcast channels for efficient change notifications.

## Features

- **Automatic Updates**: UI components re-render when reactive state changes
- **Thread Safety**: Arc/RwLock-based shared state across threads
- **Change Watchers**: Register callbacks for specific state changes
- **Broadcast Events**: Efficient notification system for multiple subscribers
- **Field-Level Granularity**: Track changes to specific fields within structs
- **Type Safety**: Generic system with compile-time type checking

## Core Components

- [`Reactive<T>`](#reactive): A reactive value container with change notifications
- [`ReactiveState`](#reactivestate): JSON-based state management for complex data
- [`ReactiveComponent`](#reactivecomponent): Component trait for reactive UI elements
- [`ReactiveStruct`](#reactivestruct): Derive macro for automatic reactivity

## Examples

### Basic Reactive Value

```rust
use reactive_tui::reactive::Reactive;

// Create reactive counter
let counter = Reactive::new(0);

// Add watcher for changes
counter.watch(|old_val, new_val| {
    println!("Counter changed from {} to {}", old_val, new_val);
});

// Update value (triggers watcher)
counter.set(1);
```

### Component with Reactive State

```rust
use reactive_tui::prelude::*;
use reactive_tui::reactive::*;

struct CounterComponent {
    count: Reactive<i32>,
}

impl CounterComponent {
    fn new() -> Self {
        Self {
            count: Reactive::new(0),
        }
    }

    fn increment(&self) {
        let current = self.count.get();
        self.count.set(current + 1);
    }
}

impl Component for CounterComponent {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("counter")
            .content(&format!("Count: {}", self.count.get()))
            .build()
    }
}
```

### Complex State with JSON

```rust
use reactive_tui::reactive::ReactiveState;
use serde_json::json;

// Create complex state
let state = ReactiveState::new();
state.set_state_json(&json!({
    "user": {
        "name": "John",
        "email": "john@example.com",
        "preferences": {
            "theme": "dark",
            "notifications": true
        }
    },
    "app": {
        "version": "1.0.0",
        "debug": false
    }
})).unwrap();

// Watch for specific field changes
state.watch_field("user.name", |old, new| {
    println!("User name changed from {:?} to {:?}", old, new);
});

// Update nested values
state.set_field("user.preferences.theme", "light".to_string());
```

### Reactive Struct Implementation

```rust
use reactive_tui::reactive::{Reactive, ReactiveState, ReactiveStruct};
use reactive_tui::error::Result;

struct AppSettings {
    theme: Reactive<String>,
    font_size: Reactive<u16>,
    auto_save: Reactive<bool>,
    state: ReactiveState,
}

impl AppSettings {
    fn new() -> Self {
        Self {
            theme: Reactive::new("dark".to_string()),
            font_size: Reactive::new(14),
            auto_save: Reactive::new(true),
            state: ReactiveState::new(),
        }
    }

    fn watch_theme<F>(&self, watcher: F)
    where
        F: Fn(&String, &String) + Send + Sync + 'static,
    {
        self.theme.watch(watcher);
    }
}

impl ReactiveStruct for AppSettings {
    fn init_reactive(&mut self) {
        // Initialize watchers for reactive fields
    }
    fn reactive_state(&self) -> &ReactiveState { &self.state }
    fn reactive_state_mut(&mut self) -> &mut ReactiveState { &mut self.state }
    fn sync_to_state(&mut self) -> Result<()> { Ok(()) }
    fn load_from_state(&mut self) -> Result<()> { Ok(()) }
}

let settings = AppSettings::new();
settings.watch_theme(|old, new| {
    println!("Theme changed: {} -> {}", old, new);
});
```

## Structs

### Reactive\<T\>

A reactive value that notifies watchers when it changes.

**Type Parameters:**
- `T`: The value type, must implement `Clone + PartialEq + Send + Sync + 'static`

#### Methods

##### `new(initial_value: T) -> Self`

Create a new reactive value

##### `get(&self) -> T`

Get the current value

##### `set(&self, new_value: T)`

Set a new value, triggering watchers if changed

##### `watch<F>(&self, watcher: F)`

Add a watcher function that's called when the value changes

Where `F: Fn(&T, &T) + Send + Sync + 'static`

##### `subscribe(&self) -> Receiver<ReactiveChange<T>>`

Subscribe to changes via broadcast channel

##### `update<F>(&self, updater: F)`

Update the value using a closure

Where `F: FnOnce(&mut T)`

### ReactiveChange

Change notification for reactive values

### ReactiveComponent

Example reactive component implementation

### ReactiveState

Reactive state container for managing application state

### StateChange

Change notification for reactive state

## Traits

### ReactiveStruct

Trait for structs that contain reactive state and can initialize reactive watchers. This provides a standard interface for reactive components.

#### Required Methods

##### `init_reactive(&mut self)`

Initialize reactive watchers and setup

##### `reactive_state(&self) -> &ReactiveState`

Get reference to the reactive state

##### `reactive_state_mut(&mut self) -> &mut ReactiveState`

Get mutable reference to the reactive state

##### `sync_to_state(&mut self) -> Result<()>`

Synchronize current values to reactive state

##### `load_from_state(&mut self) -> Result<()>`

Load values from reactive state

### Watchable

Trait for objects that can be watched for changes

## Type Aliases

### ReactiveWatcher

```rust
type ReactiveWatcher<T> = Box<dyn Fn(&T, &T) + Send + Sync>;
```

Watcher callback for reactive value changes

### WatcherFn

Type alias for complex watcher function type

### WatchersMap

Type alias for watchers map to reduce complexity