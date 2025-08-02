//! Flexible key binding system for elements and application-level actions

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};

#[cfg(feature = "typescript")]
use ts_rs::TS;

/// Represents a key combination (key + modifiers)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub struct KeyCombination {
    #[cfg_attr(feature = "typescript", ts(type = "string"))]
    pub code: KeyCode,
    #[cfg_attr(feature = "typescript", ts(type = "number"))]
    pub modifiers: KeyModifiers,
}

impl KeyCombination {
    pub fn new(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::NONE,
        }
    }

    pub fn with_ctrl(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::CONTROL,
        }
    }

    pub fn with_alt(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::ALT,
        }
    }

    pub fn with_shift(code: KeyCode) -> Self {
        Self {
            code,
            modifiers: KeyModifiers::SHIFT,
        }
    }

    pub fn with_modifiers(code: KeyCode, modifiers: KeyModifiers) -> Self {
        Self { code, modifiers }
    }

    /// Create from a simple character
    pub fn char(c: char) -> Self {
        Self::new(KeyCode::Char(c))
    }

    /// Create from KeyEvent
    pub fn from_event(event: &KeyEvent) -> Self {
        Self {
            code: event.code,
            modifiers: event.modifiers,
        }
    }

    /// Format as a human-readable string  
    pub fn format_string(&self) -> String {
        let mut parts = Vec::new();

        if self.modifiers.contains(KeyModifiers::CONTROL) {
            parts.push("Ctrl".to_string());
        }
        if self.modifiers.contains(KeyModifiers::ALT) {
            parts.push("Alt".to_string());
        }
        if self.modifiers.contains(KeyModifiers::SHIFT) {
            parts.push("Shift".to_string());
        }

        let key_str = match self.code {
            KeyCode::Char(c) => c.to_string(),
            KeyCode::F(n) => format!("F{n}"),
            KeyCode::Enter => "Enter".to_string(),
            KeyCode::Tab => "Tab".to_string(),
            KeyCode::BackTab => "Shift+Tab".to_string(),
            KeyCode::Backspace => "Backspace".to_string(),
            KeyCode::Delete => "Delete".to_string(),
            KeyCode::Insert => "Insert".to_string(),
            KeyCode::Home => "Home".to_string(),
            KeyCode::End => "End".to_string(),
            KeyCode::PageUp => "PageUp".to_string(),
            KeyCode::PageDown => "PageDown".to_string(),
            KeyCode::Up => "Up".to_string(),
            KeyCode::Down => "Down".to_string(),
            KeyCode::Left => "Left".to_string(),
            KeyCode::Right => "Right".to_string(),
            KeyCode::Esc => "Escape".to_string(),
            _ => "Unknown".to_string(),
        };

        parts.push(key_str);
        parts.join("+")
    }
}

impl std::fmt::Display for KeyCombination {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_string())
    }
}

/// Represents a sequence of key combinations (chord)
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct KeySequence {
    pub keys: Vec<KeyCombination>,
    pub timeout: Duration,
}

impl KeySequence {
    /// Create a new key sequence
    pub fn new(keys: Vec<KeyCombination>) -> Self {
        Self {
            keys,
            timeout: Duration::from_millis(1000), // Default 1 second timeout
        }
    }

    /// Create a key sequence with custom timeout
    pub fn with_timeout(keys: Vec<KeyCombination>, timeout: Duration) -> Self {
        Self { keys, timeout }
    }

    /// Create a simple single-key sequence
    pub fn single(key: KeyCombination) -> Self {
        Self::new(vec![key])
    }

    /// Create a two-key chord
    pub fn chord(first: KeyCombination, second: KeyCombination) -> Self {
        Self::new(vec![first, second])
    }

    /// Create a three-key chord
    pub fn triple(first: KeyCombination, second: KeyCombination, third: KeyCombination) -> Self {
        Self::new(vec![first, second, third])
    }

    /// Parse a key sequence from a string like "ctrl+k,s" or "ctrl+x,ctrl+c"
    pub fn parse(input: &str) -> Option<Self> {
        let parts: Vec<&str> = input.split(',').collect();
        let mut keys = Vec::new();

        for part in parts {
            if let Some(key) = Self::parse_key_combination(part.trim()) {
                keys.push(key);
            } else {
                return None;
            }
        }

        if keys.is_empty() {
            None
        } else {
            Some(Self::new(keys))
        }
    }

    /// Parse a single key combination from string like "ctrl+k" or "alt+shift+f1"
    fn parse_key_combination(input: &str) -> Option<KeyCombination> {
        let parts: Vec<String> = input.split('+').map(|s| s.trim().to_lowercase()).collect();

        if parts.is_empty() {
            return None;
        }

        let mut modifiers = KeyModifiers::NONE;
        let mut key_part = None;

        for part in &parts {
            match part.as_str() {
                "ctrl" | "control" => modifiers |= KeyModifiers::CONTROL,
                "alt" => modifiers |= KeyModifiers::ALT,
                "shift" => modifiers |= KeyModifiers::SHIFT,
                _ => key_part = Some(part.as_str()),
            }
        }

        let key_part = key_part?;
        let code = match key_part {
            "enter" => KeyCode::Enter,
            "tab" => KeyCode::Tab,
            "space" => KeyCode::Char(' '),
            "escape" | "esc" => KeyCode::Esc,
            "backspace" => KeyCode::Backspace,
            "delete" | "del" => KeyCode::Delete,
            "insert" | "ins" => KeyCode::Insert,
            "home" => KeyCode::Home,
            "end" => KeyCode::End,
            "pageup" => KeyCode::PageUp,
            "pagedown" => KeyCode::PageDown,
            "up" => KeyCode::Up,
            "down" => KeyCode::Down,
            "left" => KeyCode::Left,
            "right" => KeyCode::Right,
            key if key.starts_with('f') && key.len() > 1 => {
                if let Ok(n) = key[1..].parse::<u8>() {
                    KeyCode::F(n)
                } else {
                    return None;
                }
            }
            key if key.len() == 1 => KeyCode::Char(key.chars().next()?),
            _ => return None,
        };

        Some(KeyCombination::with_modifiers(code, modifiers))
    }

    /// Get the length of the sequence
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// Check if the sequence is empty
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// Check if this sequence starts with the given partial sequence
    pub fn starts_with(&self, partial: &[KeyCombination]) -> bool {
        if partial.len() > self.keys.len() {
            return false;
        }

        self.keys[..partial.len()] == *partial
    }

    /// Format as a human-readable string  
    pub fn format_string(&self) -> String {
        self.keys
            .iter()
            .map(|k| k.format_string())
            .collect::<Vec<String>>()
            .join(", ")
    }
}

impl std::fmt::Display for KeySequence {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.format_string())
    }
}

/// Action to take when a key is pressed
#[derive(Debug, Clone)]
pub enum KeyAction {
    /// Quit the application
    Quit,
    /// Activate element with given ID
    ActivateElement(String),
    /// Focus element with given ID
    FocusElement(String),
    /// Navigate focus (next/previous)
    Navigate(NavigationDirection),
    /// Custom action with callback
    Custom(String),
    /// Emit custom event
    EmitEvent(String, serde_json::Value),
    /// Dispatch an action through the action system
    Action(String),
    /// Dispatch an action with parameters
    ActionWithParams(String, serde_json::Value),
}

#[derive(Debug, Clone)]
pub enum NavigationDirection {
    Next,
    Previous,
    Up,
    Down,
    Left,
    Right,
}

/// Element-specific key binding
#[derive(Debug, Clone)]
pub struct ElementKeyBinding {
    pub element_id: String,
    pub key: KeyCombination,
    pub action: ElementAction,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "typescript", derive(TS))]
#[cfg_attr(feature = "typescript", ts(export))]
pub enum ElementAction {
    /// Activate this element (click/press)
    Activate,
    /// Focus this element
    Focus,
    /// Toggle element state
    Toggle,
    /// Custom element action
    Custom(String),
}

/// State for tracking compound key sequences
#[derive(Debug, Clone)]
pub struct KeySequenceState {
    /// Current partial sequence being built
    partial_sequence: Vec<KeyCombination>,
    /// When the current sequence was started
    sequence_start: Option<Instant>,
    /// Maximum time to wait for next key in sequence
    default_timeout: Duration,
}

impl KeySequenceState {
    pub fn new() -> Self {
        Self {
            partial_sequence: Vec::new(),
            sequence_start: None,
            default_timeout: Duration::from_millis(1000),
        }
    }

    /// Check if we're currently in a key sequence
    pub fn is_in_sequence(&self) -> bool {
        !self.partial_sequence.is_empty()
    }

    /// Add a key to the current sequence
    pub fn add_key(&mut self, key: KeyCombination) {
        if self.partial_sequence.is_empty() {
            self.sequence_start = Some(Instant::now());
        }
        self.partial_sequence.push(key);
    }

    /// Check if the current sequence has timed out
    pub fn is_timed_out(&self) -> bool {
        if let Some(start) = self.sequence_start {
            start.elapsed() > self.default_timeout
        } else {
            false
        }
    }

    /// Clear the current sequence
    pub fn clear(&mut self) {
        self.partial_sequence.clear();
        self.sequence_start = None;
    }

    /// Get the current partial sequence
    pub fn partial_sequence(&self) -> &[KeyCombination] {
        &self.partial_sequence
    }

    /// Set the default timeout for sequences
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.default_timeout = timeout;
    }
}

impl Default for KeySequenceState {
    fn default() -> Self {
        Self::new()
    }
}

/// Manages all key bindings for the application
#[derive(Debug)]
pub struct KeyBindingManager {
    /// App-level key bindings (single keys)
    app_bindings: HashMap<KeyCombination, KeyAction>,
    /// App-level key sequence bindings (chords)
    app_sequence_bindings: HashMap<KeySequence, KeyAction>,
    /// Element-specific key bindings
    element_bindings: HashMap<KeyCombination, ElementKeyBinding>,
    /// Element-specific key sequence bindings
    element_sequence_bindings: HashMap<KeySequence, ElementKeyBinding>,
    /// Navigation key bindings
    navigation_bindings: HashMap<KeyCombination, NavigationDirection>,
    /// Navigation key sequence bindings
    navigation_sequence_bindings: HashMap<KeySequence, NavigationDirection>,
    /// Default bindings enabled
    use_defaults: bool,
    /// Key sequence state for compound keys
    sequence_state: KeySequenceState,
}

impl KeyBindingManager {
    pub fn new() -> Self {
        let mut manager = Self {
            app_bindings: HashMap::new(),
            app_sequence_bindings: HashMap::new(),
            element_bindings: HashMap::new(),
            element_sequence_bindings: HashMap::new(),
            navigation_bindings: HashMap::new(),
            navigation_sequence_bindings: HashMap::new(),
            use_defaults: true,
            sequence_state: KeySequenceState::new(),
        };

        manager.setup_default_bindings();
        manager
    }

    pub fn new_without_defaults() -> Self {
        Self {
            app_bindings: HashMap::new(),
            app_sequence_bindings: HashMap::new(),
            element_bindings: HashMap::new(),
            element_sequence_bindings: HashMap::new(),
            navigation_bindings: HashMap::new(),
            navigation_sequence_bindings: HashMap::new(),
            use_defaults: false,
            sequence_state: KeySequenceState::new(),
        }
    }

    fn setup_default_bindings(&mut self) {
        // Default app bindings
        self.app_bindings
            .insert(KeyCombination::char('q'), KeyAction::Quit);
        self.app_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('c')),
            KeyAction::Quit,
        );

        // Default navigation bindings
        self.navigation_bindings
            .insert(KeyCombination::new(KeyCode::Tab), NavigationDirection::Next);
        self.navigation_bindings.insert(
            KeyCombination::new(KeyCode::BackTab),
            NavigationDirection::Previous,
        );
        self.navigation_bindings.insert(
            KeyCombination::new(KeyCode::Down),
            NavigationDirection::Down,
        );
        self.navigation_bindings
            .insert(KeyCombination::new(KeyCode::Up), NavigationDirection::Up);
        self.navigation_bindings.insert(
            KeyCombination::new(KeyCode::Right),
            NavigationDirection::Right,
        );
        self.navigation_bindings.insert(
            KeyCombination::new(KeyCode::Left),
            NavigationDirection::Left,
        );

        // Enter for activation
        self.app_bindings.insert(
            KeyCombination::new(KeyCode::Enter),
            KeyAction::Action("activate".to_string()),
        );

        // F1 for help
        self.app_bindings.insert(
            KeyCombination::f(1),
            KeyAction::Action("toggle_help".to_string()),
        );

        // Default key sequence bindings (VS Code/Emacs inspired)

        // Ctrl+K,S - Save file
        if let Some(save_sequence) = KeySequence::parse("ctrl+k,s") {
            self.app_sequence_bindings
                .insert(save_sequence, KeyAction::Action("save".to_string()));
        }

        // Ctrl+K,O - Open file
        if let Some(open_sequence) = KeySequence::parse("ctrl+k,o") {
            self.app_sequence_bindings
                .insert(open_sequence, KeyAction::Action("open".to_string()));
        }

        // Ctrl+K,Ctrl+C - Toggle comments
        if let Some(comment_sequence) = KeySequence::parse("ctrl+k,ctrl+c") {
            self.app_sequence_bindings.insert(
                comment_sequence,
                KeyAction::Action("toggle_comment".to_string()),
            );
        }

        // Ctrl+X,Ctrl+C - Exit (Emacs style)
        if let Some(exit_sequence) = KeySequence::parse("ctrl+x,ctrl+c") {
            self.app_sequence_bindings
                .insert(exit_sequence, KeyAction::Quit);
        }

        // Ctrl+W,H - Navigate left window (Vim style)
        if let Some(nav_left) = KeySequence::parse("ctrl+w,h") {
            self.navigation_sequence_bindings
                .insert(nav_left, NavigationDirection::Left);
        }

        // Ctrl+W,L - Navigate right window (Vim style)
        if let Some(nav_right) = KeySequence::parse("ctrl+w,l") {
            self.navigation_sequence_bindings
                .insert(nav_right, NavigationDirection::Right);
        }
    }

    /// Bind a key to an app-level action
    pub fn bind_app_key(&mut self, key: KeyCombination, action: KeyAction) {
        self.app_bindings.insert(key, action);
    }

    /// Bind a key sequence to an app-level action
    pub fn bind_app_sequence(&mut self, sequence: KeySequence, action: KeyAction) {
        self.app_sequence_bindings.insert(sequence, action);
    }

    /// Bind a key sequence from string (e.g., "ctrl+k,s") to an app-level action
    pub fn bind_app_sequence_str(
        &mut self,
        sequence_str: &str,
        action: KeyAction,
    ) -> Result<(), String> {
        if let Some(sequence) = KeySequence::parse(sequence_str) {
            self.bind_app_sequence(sequence, action);
            Ok(())
        } else {
            Err(format!("Invalid key sequence: {sequence_str}"))
        }
    }

    /// Bind a key to activate a specific element
    pub fn bind_element_key(
        &mut self,
        key: KeyCombination,
        element_id: String,
        action: ElementAction,
    ) {
        let binding = ElementKeyBinding {
            element_id: element_id.clone(),
            key: key.clone(),
            action,
        };
        self.element_bindings.insert(key, binding);
    }

    /// Bind a key sequence to activate a specific element
    pub fn bind_element_sequence(
        &mut self,
        sequence: KeySequence,
        element_id: String,
        action: ElementAction,
    ) {
        let binding = ElementKeyBinding {
            element_id: element_id.clone(),
            key: sequence.keys[0].clone(), // Store first key for backward compatibility
            action,
        };
        self.element_sequence_bindings.insert(sequence, binding);
    }

    /// Bind a key to navigation action
    pub fn bind_navigation_key(&mut self, key: KeyCombination, direction: NavigationDirection) {
        self.navigation_bindings.insert(key, direction);
    }

    /// Bind a key sequence to navigation action
    pub fn bind_navigation_sequence(
        &mut self,
        sequence: KeySequence,
        direction: NavigationDirection,
    ) {
        self.navigation_sequence_bindings
            .insert(sequence, direction);
    }

    /// Remove a key binding
    pub fn unbind_key(&mut self, key: &KeyCombination) {
        self.app_bindings.remove(key);
        self.element_bindings.remove(key);
        self.navigation_bindings.remove(key);

        // Remove any sequences that start with this key
        self.app_sequence_bindings
            .retain(|seq, _| !(seq.keys.first() == Some(key)));
        self.element_sequence_bindings
            .retain(|seq, _| !(seq.keys.first() == Some(key)));
        self.navigation_sequence_bindings
            .retain(|seq, _| !(seq.keys.first() == Some(key)));
    }

    /// Remove a key sequence binding
    pub fn unbind_sequence(&mut self, sequence: &KeySequence) {
        self.app_sequence_bindings.remove(sequence);
        self.element_sequence_bindings.remove(sequence);
        self.navigation_sequence_bindings.remove(sequence);
    }

    /// Handle a key event and return the appropriate action
    /// This method needs to be called with mutable access to manage sequence state
    pub fn handle_key(&mut self, event: &KeyEvent) -> KeyBindingResult {
        let key = KeyCombination::from_event(event);

        // Check if current sequence has timed out
        if self.sequence_state.is_timed_out() {
            self.sequence_state.clear();
        }

        // Add key to current sequence
        self.sequence_state.add_key(key.clone());

        // Get current sequence as owned Vec to avoid borrowing issues
        let current_sequence: Vec<KeyCombination> = self.sequence_state.partial_sequence().to_vec();

        // First check if we have a complete sequence match
        let complete_result = self.check_complete_sequences(&current_sequence);
        if let Some(result) = complete_result {
            self.sequence_state.clear();
            return result;
        }

        // Check if current sequence is a prefix of any registered sequence
        if self.has_sequence_prefix(&current_sequence) {
            // Continue building sequence
            return KeyBindingResult::Unhandled;
        }

        // No matching sequence prefix, clear state and try single key bindings
        self.sequence_state.clear();

        // If the sequence had only one key, check single key bindings
        if current_sequence.len() == 1 {
            return self.check_single_key_bindings(&key);
        }

        KeyBindingResult::Unhandled
    }

    /// Check if current sequence matches any complete sequence bindings
    fn check_complete_sequences(&self, sequence: &[KeyCombination]) -> Option<KeyBindingResult> {
        // Check element sequence bindings first (highest priority)
        for (seq, binding) in &self.element_sequence_bindings {
            if seq.keys == sequence {
                return Some(KeyBindingResult::ElementAction(binding.clone()));
            }
        }

        // Check navigation sequence bindings
        for (seq, direction) in &self.navigation_sequence_bindings {
            if seq.keys == sequence {
                return Some(KeyBindingResult::Navigation(direction.clone()));
            }
        }

        // Check app sequence bindings
        for (seq, action) in &self.app_sequence_bindings {
            if seq.keys == sequence {
                return Some(KeyBindingResult::AppAction(action.clone()));
            }
        }

        None
    }

    /// Check if current sequence is a prefix of any registered sequence
    fn has_sequence_prefix(&self, partial: &[KeyCombination]) -> bool {
        // Check all sequence bindings to see if any start with the current partial sequence
        for seq in self.app_sequence_bindings.keys() {
            if seq.starts_with(partial) && seq.keys.len() > partial.len() {
                return true;
            }
        }

        for seq in self.element_sequence_bindings.keys() {
            if seq.starts_with(partial) && seq.keys.len() > partial.len() {
                return true;
            }
        }

        for seq in self.navigation_sequence_bindings.keys() {
            if seq.starts_with(partial) && seq.keys.len() > partial.len() {
                return true;
            }
        }

        false
    }

    /// Check single key bindings (original behavior)
    fn check_single_key_bindings(&self, key: &KeyCombination) -> KeyBindingResult {
        // Check element bindings first (highest priority)
        if let Some(binding) = self.element_bindings.get(key) {
            return KeyBindingResult::ElementAction(binding.clone());
        }

        // Check navigation bindings
        if let Some(direction) = self.navigation_bindings.get(key) {
            return KeyBindingResult::Navigation(direction.clone());
        }

        // Check app bindings
        if let Some(action) = self.app_bindings.get(key) {
            return KeyBindingResult::AppAction(action.clone());
        }

        KeyBindingResult::Unhandled
    }

    /// Get all bindings for a specific element
    pub fn get_element_bindings(&self, element_id: &str) -> Vec<&ElementKeyBinding> {
        self.element_bindings
            .values()
            .filter(|binding| binding.element_id == element_id)
            .collect()
    }

    /// Check if a key is bound to anything (including as part of sequences)
    pub fn is_key_bound(&self, key: &KeyCombination) -> bool {
        // Check direct key bindings
        if self.app_bindings.contains_key(key)
            || self.element_bindings.contains_key(key)
            || self.navigation_bindings.contains_key(key)
        {
            return true;
        }

        // Check if key is part of any sequence
        for seq in self.app_sequence_bindings.keys() {
            if seq.keys.contains(key) {
                return true;
            }
        }

        for seq in self.element_sequence_bindings.keys() {
            if seq.keys.contains(key) {
                return true;
            }
        }

        for seq in self.navigation_sequence_bindings.keys() {
            if seq.keys.contains(key) {
                return true;
            }
        }

        false
    }

    /// Check if a key sequence is bound
    pub fn is_sequence_bound(&self, sequence: &KeySequence) -> bool {
        self.app_sequence_bindings.contains_key(sequence)
            || self.element_sequence_bindings.contains_key(sequence)
            || self.navigation_sequence_bindings.contains_key(sequence)
    }

    /// Get help text for all key bindings
    pub fn get_help_text(&self) -> String {
        let mut help = String::new();

        help.push_str("Key Bindings:\n");
        help.push_str("=============\n\n");

        help.push_str("App Controls:\n");
        for (key, action) in &self.app_bindings {
            help.push_str(&format!("  {} - {:?}\n", key.format_string(), action));
        }

        help.push_str("\nApp Key Sequences:\n");
        for (sequence, action) in &self.app_sequence_bindings {
            help.push_str(&format!("  {} - {:?}\n", sequence.format_string(), action));
        }

        help.push_str("\nNavigation:\n");
        for (key, direction) in &self.navigation_bindings {
            help.push_str(&format!("  {} - {:?}\n", key.format_string(), direction));
        }

        help.push_str("\nNavigation Key Sequences:\n");
        for (sequence, direction) in &self.navigation_sequence_bindings {
            help.push_str(&format!(
                "  {} - {:?}\n",
                sequence.format_string(),
                direction
            ));
        }

        help.push_str("\nElement Bindings:\n");
        for (key, binding) in &self.element_bindings {
            help.push_str(&format!(
                "  {} - {} {:?}\n",
                key.format_string(),
                binding.element_id,
                binding.action
            ));
        }

        help.push_str("\nElement Key Sequences:\n");
        for (sequence, binding) in &self.element_sequence_bindings {
            help.push_str(&format!(
                "  {} - {} {:?}\n",
                sequence.format_string(),
                binding.element_id,
                binding.action
            ));
        }

        help
    }

    /// Enable or disable default bindings
    pub fn set_use_defaults(&mut self, use_defaults: bool) {
        self.use_defaults = use_defaults;
        if use_defaults {
            self.setup_default_bindings();
        } else {
            self.app_bindings.clear();
            self.app_sequence_bindings.clear();
            self.navigation_bindings.clear();
            self.navigation_sequence_bindings.clear();
            self.element_bindings.clear();
            self.element_sequence_bindings.clear();
        }
    }

    /// Create a preset binding configuration
    pub fn with_preset(preset: KeyBindingPreset) -> Self {
        let mut manager = Self::new();

        match preset {
            KeyBindingPreset::Default => {
                // Already set up in new()
            }
            KeyBindingPreset::Vim => {
                manager.setup_vim_bindings();
            }
            KeyBindingPreset::Emacs => {
                manager.setup_emacs_bindings();
            }
            KeyBindingPreset::Gaming => {
                manager.setup_gaming_bindings();
            }
        }

        manager
    }

    fn setup_vim_bindings(&mut self) {
        // Vim-style navigation
        self.navigation_bindings
            .insert(KeyCombination::char('j'), NavigationDirection::Down);
        self.navigation_bindings
            .insert(KeyCombination::char('k'), NavigationDirection::Up);
        self.navigation_bindings
            .insert(KeyCombination::char('h'), NavigationDirection::Left);
        self.navigation_bindings
            .insert(KeyCombination::char('l'), NavigationDirection::Right);

        // Vim-style quit
        self.app_bindings.remove(&KeyCombination::char('q'));
        self.app_bindings
            .insert(KeyCombination::new(KeyCode::Esc), KeyAction::Quit);
    }

    fn setup_emacs_bindings(&mut self) {
        // Emacs-style navigation
        self.navigation_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('n')),
            NavigationDirection::Down,
        );
        self.navigation_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('p')),
            NavigationDirection::Up,
        );
        self.navigation_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('f')),
            NavigationDirection::Right,
        );
        self.navigation_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('b')),
            NavigationDirection::Left,
        );

        // Emacs-style quit
        self.app_bindings.insert(
            KeyCombination::with_ctrl(KeyCode::Char('x')),
            KeyAction::Quit,
        );
    }

    fn setup_gaming_bindings(&mut self) {
        // WASD navigation
        self.navigation_bindings
            .insert(KeyCombination::char('w'), NavigationDirection::Up);
        self.navigation_bindings
            .insert(KeyCombination::char('a'), NavigationDirection::Left);
        self.navigation_bindings
            .insert(KeyCombination::char('s'), NavigationDirection::Down);
        self.navigation_bindings
            .insert(KeyCombination::char('d'), NavigationDirection::Right);

        // Space for activation
        self.app_bindings.insert(
            KeyCombination::new(KeyCode::Char(' ')),
            KeyAction::Custom("activate_focused".to_string()),
        );
    }
}

#[derive(Debug, Clone)]
pub enum KeyBindingResult {
    AppAction(KeyAction),
    ElementAction(ElementKeyBinding),
    Navigation(NavigationDirection),
    Unhandled,
}

#[derive(Debug, Clone)]
pub enum KeyBindingPreset {
    Default,
    Vim,
    Emacs,
    Gaming,
}

impl Default for KeyBindingManager {
    fn default() -> Self {
        Self::new()
    }
}

// Convenience functions for common key combinations
impl KeyCombination {
    pub fn ctrl_c() -> Self {
        Self::with_ctrl(KeyCode::Char('c'))
    }
    pub fn ctrl_x() -> Self {
        Self::with_ctrl(KeyCode::Char('x'))
    }
    pub fn ctrl_z() -> Self {
        Self::with_ctrl(KeyCode::Char('z'))
    }
    pub fn escape() -> Self {
        Self::new(KeyCode::Esc)
    }
    pub fn enter() -> Self {
        Self::new(KeyCode::Enter)
    }
    pub fn space() -> Self {
        Self::new(KeyCode::Char(' '))
    }
    pub fn tab() -> Self {
        Self::new(KeyCode::Tab)
    }
    pub fn shift_tab() -> Self {
        Self::new(KeyCode::BackTab)
    }
    pub fn f(n: u8) -> Self {
        Self::new(KeyCode::F(n))
    }
}

impl KeyAction {
    /// Create an action that dispatches to the action system
    pub fn action<S: Into<String>>(name: S) -> Self {
        Self::Action(name.into())
    }

    /// Create an action with parameters
    pub fn action_with_params<S: Into<String>>(name: S, params: serde_json::Value) -> Self {
        Self::ActionWithParams(name.into(), params)
    }
}

impl ElementAction {
    /// Create an action that dispatches to the action system
    pub fn action<S: Into<String>>(name: S) -> Self {
        Self::Custom(name.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};

    #[test]
    fn test_key_combination_creation() {
        let key = KeyCombination::new(KeyCode::Char('a'));
        assert_eq!(key.code, KeyCode::Char('a'));
        assert_eq!(key.modifiers, KeyModifiers::NONE);

        let ctrl_key = KeyCombination::with_ctrl(KeyCode::Char('c'));
        assert_eq!(ctrl_key.code, KeyCode::Char('c'));
        assert_eq!(ctrl_key.modifiers, KeyModifiers::CONTROL);
    }

    #[test]
    fn test_key_sequence_parsing() {
        let sequence = KeySequence::parse("ctrl+k,s").unwrap();
        assert_eq!(sequence.keys.len(), 2);
        assert_eq!(
            sequence.keys[0],
            KeyCombination::with_ctrl(KeyCode::Char('k'))
        );
        assert_eq!(sequence.keys[1], KeyCombination::new(KeyCode::Char('s')));

        let complex_sequence = KeySequence::parse("ctrl+x,ctrl+c").unwrap();
        assert_eq!(complex_sequence.keys.len(), 2);
        assert_eq!(
            complex_sequence.keys[0],
            KeyCombination::with_ctrl(KeyCode::Char('x'))
        );
        assert_eq!(
            complex_sequence.keys[1],
            KeyCombination::with_ctrl(KeyCode::Char('c'))
        );

        // Test invalid sequence
        assert!(KeySequence::parse("invalid+sequence").is_none());
    }

    #[test]
    fn test_key_sequence_starts_with() {
        let sequence = KeySequence::parse("ctrl+k,s,f").unwrap();
        let partial = vec![
            KeyCombination::with_ctrl(KeyCode::Char('k')),
            KeyCombination::new(KeyCode::Char('s')),
        ];

        assert!(sequence.starts_with(&partial));
        assert!(!sequence.starts_with(&[KeyCombination::new(KeyCode::Char('x'))]));
    }

    #[test]
    fn test_key_binding_manager() {
        let mut manager = KeyBindingManager::new();

        // Test binding and checking
        let key = KeyCombination::char('x');
        manager.bind_app_key(key.clone(), KeyAction::Quit);
        assert!(manager.is_key_bound(&key));

        // Test unbinding
        manager.unbind_key(&key);
        assert!(!manager.is_key_bound(&key));
    }

    #[test]
    fn test_key_sequence_binding() {
        let mut manager = KeyBindingManager::new_without_defaults();
        let sequence = KeySequence::parse("ctrl+k,s").unwrap();

        manager.bind_app_sequence(sequence.clone(), KeyAction::Action("save".to_string()));
        assert!(manager.is_sequence_bound(&sequence));

        // Test string-based binding
        manager
            .bind_app_sequence_str("ctrl+x,c", KeyAction::Quit)
            .unwrap();
        let quit_sequence = KeySequence::parse("ctrl+x,c").unwrap();
        assert!(manager.is_sequence_bound(&quit_sequence));
    }

    #[test]
    fn test_compound_key_handling() {
        let mut manager = KeyBindingManager::new_without_defaults();
        let sequence = KeySequence::parse("ctrl+k,s").unwrap();
        manager.bind_app_sequence(sequence, KeyAction::Action("save".to_string()));

        // First key in sequence
        let first_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL);
        let result1 = manager.handle_key(&first_event);
        assert!(matches!(result1, KeyBindingResult::Unhandled)); // Should be waiting for next key

        // Second key in sequence
        let second_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        let result2 = manager.handle_key(&second_event);
        match result2 {
            KeyBindingResult::AppAction(KeyAction::Action(name)) => {
                assert_eq!(name, "save");
            }
            _ => panic!("Expected save action, got {result2:?}"),
        }
    }

    #[test]
    fn test_sequence_timeout() {
        let mut manager = KeyBindingManager::new_without_defaults();
        let sequence = KeySequence::parse("ctrl+k,s").unwrap();
        manager.bind_app_sequence(sequence, KeyAction::Action("save".to_string()));

        // Start sequence
        let first_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::CONTROL);
        let _result1 = manager.handle_key(&first_event);

        // Manually trigger timeout
        manager.sequence_state.sequence_start =
            Some(std::time::Instant::now() - std::time::Duration::from_secs(2));

        // Next key should reset and try single key binding
        let second_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        let result2 = manager.handle_key(&second_event);
        assert!(matches!(result2, KeyBindingResult::Unhandled)); // No single 's' key binding
    }

    #[test]
    fn test_navigation_bindings() {
        let mut manager = KeyBindingManager::new();
        let up_key = KeyCombination::new(KeyCode::Up);

        manager.bind_navigation_key(up_key.clone(), NavigationDirection::Up);

        let event = KeyEvent::new(KeyCode::Up, KeyModifiers::NONE);
        let result = manager.handle_key(&event);

        match result {
            KeyBindingResult::Navigation(NavigationDirection::Up) => {}
            _ => panic!("Expected navigation result"),
        }
    }

    #[test]
    fn test_element_bindings() {
        let mut manager = KeyBindingManager::new_without_defaults();
        let enter_key = KeyCombination::new(KeyCode::Enter);

        manager.bind_element_key(
            enter_key.clone(),
            "button1".to_string(),
            ElementAction::Activate,
        );

        let event = KeyEvent::new(KeyCode::Enter, KeyModifiers::NONE);
        let result = manager.handle_key(&event);

        match result {
            KeyBindingResult::ElementAction(binding) => {
                assert_eq!(binding.element_id, "button1");
                assert!(matches!(binding.action, ElementAction::Activate));
            }
            _ => panic!("Expected element action result"),
        }
    }

    #[test]
    fn test_sequence_priority() {
        let mut manager = KeyBindingManager::new_without_defaults();

        // Bind single key
        manager.bind_app_key(
            KeyCombination::char('k'),
            KeyAction::Action("single_k".to_string()),
        );

        // Bind sequence starting with same key
        let sequence = KeySequence::parse("k,s").unwrap();
        manager.bind_app_sequence(sequence, KeyAction::Action("k_s_sequence".to_string()));

        // Test single key press followed by timeout/different key
        let k_event = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        let result1 = manager.handle_key(&k_event);
        assert!(matches!(result1, KeyBindingResult::Unhandled)); // Waiting for sequence

        // Press unrelated key - should clear sequence and handle single key
        let x_event = KeyEvent::new(KeyCode::Char('x'), KeyModifiers::NONE);
        let result2 = manager.handle_key(&x_event);
        assert!(matches!(result2, KeyBindingResult::Unhandled)); // No binding for 'x'

        // Now test the complete sequence
        let k_event2 = KeyEvent::new(KeyCode::Char('k'), KeyModifiers::NONE);
        let _result3 = manager.handle_key(&k_event2);

        let s_event = KeyEvent::new(KeyCode::Char('s'), KeyModifiers::NONE);
        let result4 = manager.handle_key(&s_event);
        match result4 {
            KeyBindingResult::AppAction(KeyAction::Action(name)) => {
                assert_eq!(name, "k_s_sequence");
            }
            _ => panic!("Expected k_s_sequence action, got {result4:?}"),
        }
    }

    #[test]
    fn test_default_sequences() {
        let manager = KeyBindingManager::new();

        // Test that default sequences are set up
        let save_sequence = KeySequence::parse("ctrl+k,s").unwrap();
        assert!(manager.is_sequence_bound(&save_sequence));

        let exit_sequence = KeySequence::parse("ctrl+x,ctrl+c").unwrap();
        assert!(manager.is_sequence_bound(&exit_sequence));
    }
}
