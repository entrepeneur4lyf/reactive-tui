# Reactive Module

The `reactive` module provides reactive state management with automatic change detection and notifications.

## Reactive&lt;T&gt;

A reactive container that wraps a value and notifies watchers when the value changes.

```rust
use reactive_tui::reactive::Reactive;
use std::sync::Arc;

let counter = Arc::new(Reactive::new(0));
```

### Methods

#### `new(value: T) -> Reactive<T>`
Creates a new reactive value.

**Parameters:**
- `value` - The initial value

**Returns:** A new `Reactive<T>` instance

#### `get(&self) -> T`
Gets the current value (requires `T: Clone`).

**Returns:** A clone of the current value

#### `set(&self, value: T)`
Sets a new value and notifies all watchers.

**Parameters:**
- `value` - The new value to set

#### `update<F>(&self, f: F)` where `F: FnOnce(&mut T)`
Updates the value using a closure and notifies watchers.

**Parameters:**
- `f` - Closure that receives a mutable reference to the value

#### `watch<F>(&self, callback: F) -> WatcherId` where `F: Fn(&T) + Send + Sync + 'static`
Registers a callback to be called when the value changes.

**Parameters:**
- `callback` - Function to call when value changes

**Returns:** A `WatcherId` that can be used to unsubscribe

#### `unwatch(&self, id: WatcherId)`
Removes a watcher by ID.

**Parameters:**
- `id` - The watcher ID returned from `watch()`

## ReactiveState

A collection of reactive values that can be managed together.

```rust
use reactive_tui::reactive::{ReactiveState, Reactive};
use std::sync::Arc;

#[derive(Default)]
struct AppState {
    counter: Arc<Reactive<i32>>,
    text: Arc<Reactive<String>>,
}

let state = ReactiveState::new(AppState::default());
```

### Methods

#### `new(state: T) -> ReactiveState<T>`
Creates a new reactive state container.

**Parameters:**
- `state` - The initial state struct

**Returns:** A new `ReactiveState<T>` instance

#### `get(&self) -> &T`
Gets a reference to the state.

**Returns:** Reference to the wrapped state

#### `update<F, R>(&self, f: F) -> R` where `F: FnOnce(&mut T) -> R`
Updates the state using a closure.

**Parameters:**
- `f` - Closure that receives a mutable reference to the state

**Returns:** The return value of the closure

## Traits

### ReactiveStruct

Trait for structs that contain reactive fields.

```rust
use reactive_tui::reactive::{ReactiveStruct, Reactive};
use std::sync::Arc;

#[derive(ReactiveStruct)]
struct MyState {
    #[reactive]
    counter: Arc<Reactive<i32>>,
    
    #[reactive]
    enabled: Arc<Reactive<bool>>,
}
```

#### Required Methods

- `fn watch_all<F>(&self, callback: F)` where `F: Fn() + Send + Sync + 'static`

### Watchable

Trait for types that can be watched for changes.

#### Required Methods

- `fn watch<F>(&self, callback: F) -> WatcherId`
- `fn unwatch(&self, id: WatcherId)`

## Example Usage

```rust
use reactive_tui::reactive::Reactive;
use std::sync::Arc;

#[tokio::main]
async fn main() {
    // Create reactive state
    let counter = Arc::new(Reactive::new(0));
    let counter_clone = counter.clone();
    
    // Watch for changes
    let watcher_id = counter.watch(move |value| {
        println!("Counter changed to: {}", value);
    });
    
    // Update the value
    counter.set(1); // Prints: "Counter changed to: 1"
    counter.update(|val| *val += 5); // Prints: "Counter changed to: 6"
    
    // Stop watching
    counter.unwatch(watcher_id);
    
    // This won't trigger the callback
    counter.set(10);
}
```

## Macros

### `reactive!`

Macro for creating reactive values with less boilerplate.

```rust
use reactive_tui::reactive;

let counter = reactive!(0); // Creates Arc<Reactive<i32>>
let text = reactive!("Hello".to_string()); // Creates Arc<Reactive<String>>
```

### `impl_reactive_struct!`

Macro for implementing `ReactiveStruct` trait automatically.

```rust
use reactive_tui::impl_reactive_struct;

struct AppState {
    counter: Arc<Reactive<i32>>,
    enabled: Arc<Reactive<bool>>,
}

impl_reactive_struct!(AppState, counter, enabled);
```