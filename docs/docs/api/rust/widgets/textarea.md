# Textarea Widget

Advanced multi-line text editor widget with comprehensive editing features including undo/redo, syntax highlighting, search functionality, line numbers, and full cursor management capabilities.

## Overview

The Textarea widget provides a full-featured multi-line text editor with advanced capabilities including syntax highlighting, undo/redo operations, search with regular expressions, text selection, line numbers, viewport scrolling, and yank/paste support. It integrates seamlessly with the CSS theming system and component architecture.

```rust
use reactive_tui::widgets::*;

let code_editor = Textarea::builder("code-editor")
    .text("fn main() {\n    println!(\"Hello, world!\");\n}")
    .language("rust")
    .syntax_highlighting(true)
    .show_line_numbers(true)
    .highlight_current_line(true)
    .viewport_size(20, 80)
    .build();
```

## Features

- **Multi-line Text Editing**: Full text insertion, deletion, and navigation
- **Undo/Redo System**: Complete operation history with configurable stack size
- **Syntax Highlighting**: Built-in highlighting for Rust, JavaScript, Python, Markdown, and Syntect integration
- **Search & Replace**: Case-sensitive and whole-word search with match highlighting
- **Text Selection**: Range selection with copy/paste operations
- **Line Numbers**: Optional line number display with custom formatting
- **Cursor Management**: Full cursor positioning with line/column tracking
- **Viewport Scrolling**: Efficient scrolling for large documents
- **Yank Buffer**: Copy/paste functionality with line-mode support
- **Theme Integration**: CSS utility classes and color theme support
- **Accessibility**: Full keyboard navigation and screen reader support
- **Performance Optimized**: Efficient rendering with viewport-based display

## Core Components

### Textarea

Main textarea widget with comprehensive editing functionality.

```rust
pub struct Textarea {
    pub id: String,
    pub state: TextareaState,
    pub style: TextareaStyle,
    pub css_classes: Vec<String>,
    pub theme: Option<String>,
    pub current_theme: Option<ColorTheme>,
    pub history: History,
    pub search: Search,
    pub yank_buffer: YankBuffer,
    pub syntect_highlighter: TextareaSyntectHighlighter,
}
```

### TextareaState

State management for text content, cursor, and editor settings.

```rust
pub struct TextareaState {
    pub lines: Vec<String>,
    pub cursor: CursorPosition,
    pub selection: Option<Selection>,
    pub viewport: Viewport,
    pub focused: bool,
    pub readonly: bool,
    pub show_line_numbers: bool,
    pub highlight_current_line: bool,
    pub syntax_highlighting: bool,
    pub language: Option<String>,
    pub search_active: bool,
    pub modified: bool,
}
```

### CursorPosition

Cursor position tracking in the text editor.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct CursorPosition {
    pub row: usize,
    pub col: usize,
}
```

### Selection

Text selection range management.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Selection {
    pub start: CursorPosition,
    pub end: CursorPosition,
}

impl Selection {
    pub fn new(start: CursorPosition, end: CursorPosition) -> Self
    pub fn is_empty(&self) -> bool
    pub fn normalize(&self) -> Self
}
```

### Viewport

Scrollable viewport for large documents.

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Viewport {
    pub scroll_row: usize,
    pub scroll_col: usize,
    pub visible_rows: usize,
    pub visible_cols: usize,
}
```

### EditOperation

Represents editing operations for undo/redo functionality.

```rust
pub enum EditOperation {
    InsertChar { pos: CursorPosition, ch: char },
    DeleteChar { pos: CursorPosition, ch: char },
    InsertText { pos: CursorPosition, text: String },
    DeleteText { pos: CursorPosition, text: String },
    InsertLine { row: usize, text: String },
    DeleteLine { row: usize, text: String },
    SplitLine { pos: CursorPosition },
    JoinLines { row: usize, text: String },
}

impl EditOperation {
    pub fn apply(&self, lines: &mut Vec<String>, cursor: &mut CursorPosition)
    pub fn inverse(&self) -> EditOperation
}
```

### History

Undo/redo history management.

```rust
pub struct History {
    operations: VecDeque<EditOperation>,
    current: usize,
    max_size: usize,
}

impl History {
    pub fn push(&mut self, op: EditOperation)
    pub fn undo(&mut self) -> Option<&EditOperation>
    pub fn redo(&mut self) -> Option<&EditOperation>
    pub fn can_undo(&self) -> bool
    pub fn can_redo(&self) -> bool
}
```

### Search

Search functionality with advanced options.

```rust
pub struct Search {
    pub query: String,
    pub case_sensitive: bool,
    pub whole_word: bool,
}

impl Search {
    pub fn set_query(&mut self, query: String)
    pub fn find_matches(&self, text: &str) -> Vec<(usize, usize)>
}
```

### YankBuffer

Copy/paste buffer with line-mode support.

```rust
pub struct YankBuffer {
    pub text: String,
    pub is_line_mode: bool,
}

impl YankBuffer {
    pub fn set_text(&mut self, text: String, is_line_mode: bool)
    pub fn get_text(&self) -> &str
    pub fn is_empty(&self) -> bool
}
```

## Builder Pattern

### TextareaBuilder

```rust
impl TextareaBuilder {
    pub fn new(id: impl Into<String>) -> Self
    pub fn text(mut self, text: &str) -> Self
    pub fn language(mut self, language: &str) -> Self
    pub fn syntax_highlighting(mut self, enabled: bool) -> Self
    pub fn show_line_numbers(mut self, show: bool) -> Self
    pub fn highlight_current_line(mut self, highlight: bool) -> Self
    pub fn readonly(mut self, readonly: bool) -> Self
    pub fn viewport_size(mut self, rows: usize, cols: usize) -> Self
    pub fn class(mut self, class: impl Into<String>) -> Self
    pub fn classes(mut self, classes: &[&str]) -> Self
    pub fn bg_color(mut self, color: &str) -> Self
    pub fn text_color(mut self, color: &str) -> Self
    pub fn border_color(mut self, color: &str) -> Self
    pub fn build(self) -> Textarea
}
```

## Methods

### Text Management

```rust
impl Textarea {
    // Create a new textarea
    pub fn new(id: impl Into<String>) -> Self
    
    // Create a builder
    pub fn builder(id: impl Into<String>) -> TextareaBuilder
    
    // Set text content
    pub fn set_text(&mut self, text: &str)
    
    // Get text content
    pub fn text(&self) -> String
    
    // Set language for syntax highlighting
    pub fn set_language(&mut self, language: Option<String>)
    
    // Enable/disable syntax highlighting
    pub fn set_syntax_highlighting(&mut self, enabled: bool)
}
```

### Editing Operations

```rust
impl Textarea {
    // Insert character at cursor
    pub fn insert_char(&mut self, ch: char)
    
    // Delete character at cursor
    pub fn delete_char(&mut self)
    
    // Delete character before cursor (backspace)
    pub fn delete_char_before(&mut self)
    
    // Insert new line at cursor
    pub fn insert_newline(&mut self)
}
```

### Cursor Movement

```rust
impl Textarea {
    // Move cursor left
    pub fn move_cursor_left(&mut self)
    
    // Move cursor right  
    pub fn move_cursor_right(&mut self)
    
    // Move cursor up
    pub fn move_cursor_up(&mut self)
    
    // Move cursor down
    pub fn move_cursor_down(&mut self)
    
    // Move cursor to line start
    pub fn move_cursor_to_line_start(&mut self)
    
    // Move cursor to line end
    pub fn move_cursor_to_line_end(&mut self)
}
```

### State Management

```rust
impl Textarea {
    // Set focus state
    pub fn set_focused(&mut self, focused: bool)
    
    // Set readonly state
    pub fn set_readonly(&mut self, readonly: bool)
    
    // Set viewport size
    pub fn set_viewport_size(&mut self, rows: usize, cols: usize)
    
    // Set theme
    pub fn set_theme(&mut self, theme: ColorTheme)
}
```

### History Operations

```rust
impl Textarea {
    // Undo last operation
    pub fn undo(&mut self) -> bool
    
    // Redo last undone operation
    pub fn redo(&mut self) -> bool
}
```

### Rendering

```rust
impl Textarea {
    // Render to Element
    pub fn render(&self) -> Element
}
```

## Examples

### Basic Code Editor

```rust
use reactive_tui::widgets::*;

let code_editor = Textarea::builder("code-editor")
    .text("fn main() {\n    println!(\"Hello, world!\");\n}")
    .language("rust")
    .syntax_highlighting(true)
    .show_line_numbers(true)
    .highlight_current_line(true)
    .viewport_size(25, 80)
    .class("code-editor")
    .build();

// Edit the text
code_editor.insert_char('!');
code_editor.insert_newline();
code_editor.set_text("let x = 42;");
```

### Multi-language Text Editor

```rust
struct CodeEditor {
    textarea: Textarea,
    current_language: String,
}

impl CodeEditor {
    fn new() -> Self {
        let textarea = Textarea::builder("multi-lang-editor")
            .syntax_highlighting(true)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .viewport_size(30, 100)
            .classes(&["editor", "code-editor", "dark-theme"])
            .build();

        Self {
            textarea,
            current_language: "rust".to_string(),
        }
    }

    fn set_language(&mut self, language: &str) {
        self.current_language = language.to_string();
        self.textarea.set_language(Some(language.to_string()));
    }

    fn load_file(&mut self, content: &str, extension: &str) {
        self.textarea.set_text(content);
        
        let language = match extension {
            "rs" => "rust",
            "js" | "ts" => "javascript", 
            "py" => "python",
            "md" => "markdown",
            _ => "text",
        };
        
        self.set_language(language);
    }

    fn save_file(&self) -> String {
        self.textarea.text()
    }
}

// Usage
let mut editor = CodeEditor::new();
editor.load_file("fn hello() {\n    println!(\"Hello!\");\n}", "rs");
```

### Configuration File Editor

```rust
let config_editor = Textarea::builder("config-editor")
    .text("# Configuration file\nport = 8080\nhost = \"localhost\"\ndebug = true")
    .language("toml")
    .syntax_highlighting(true)
    .show_line_numbers(true)
    .highlight_current_line(false)
    .viewport_size(20, 60)
    .classes(&["config-editor", "monospace"])
    .bg_color("gray-900")
    .text_color("gray-100")
    .border_color("gray-600")
    .build();
```

### Documentation Writer

```rust
let doc_editor = Textarea::builder("documentation")
    .text("# Project Documentation\n\n## Getting Started\n\nThis project provides...")
    .language("markdown")
    .syntax_highlighting(true)
    .show_line_numbers(false)
    .highlight_current_line(true)
    .viewport_size(25, 90)
    .classes(&["doc-editor", "prose"])
    .build();
```

### Interactive Terminal Editor

```rust
use reactive_tui::widgets::*;

struct TerminalEditor {
    textarea: Textarea,
    filename: Option<String>,
    saved: bool,
}

impl TerminalEditor {
    fn new() -> Self {
        let textarea = Textarea::builder("terminal-editor")
            .syntax_highlighting(true)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .viewport_size(24, 80)
            .build();

        Self {
            textarea,
            filename: None,
            saved: true,
        }
    }

    fn open_file(&mut self, filename: &str, content: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.textarea.set_text(content);
        self.filename = Some(filename.to_string());
        self.saved = true;

        // Detect language from file extension
        if let Some(ext) = std::path::Path::new(filename).extension() {
            let language = match ext.to_str().unwrap_or("") {
                "rs" => Some("rust".to_string()),
                "js" | "ts" => Some("javascript".to_string()),
                "py" => Some("python".to_string()),
                "md" => Some("markdown".to_string()),
                _ => None,
            };
            self.textarea.set_language(language);
        }

        Ok(())
    }

    fn save_file(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(filename) = &self.filename {
            std::fs::write(filename, self.textarea.text())?;
            self.saved = true;
            println!("Saved: {}", filename);
        }
        Ok(())
    }

    fn handle_key(&mut self, key: &str) -> bool {
        match key {
            // File operations
            "Ctrl+s" => {
                if let Err(e) = self.save_file() {
                    eprintln!("Save failed: {}", e);
                }
                true
            }
            "Ctrl+z" => {
                self.textarea.undo();
                true
            }
            "Ctrl+y" | "Ctrl+Shift+z" => {
                self.textarea.redo();
                true
            }
            
            // Navigation
            "ArrowLeft" => {
                self.textarea.move_cursor_left();
                true
            }
            "ArrowRight" => {
                self.textarea.move_cursor_right();
                true
            }
            "ArrowUp" => {
                self.textarea.move_cursor_up();
                true
            }
            "ArrowDown" => {
                self.textarea.move_cursor_down();
                true
            }
            "Home" => {
                self.textarea.move_cursor_to_line_start();
                true
            }
            "End" => {
                self.textarea.move_cursor_to_line_end();
                true
            }
            
            // Editing
            "Enter" => {
                self.textarea.insert_newline();
                self.saved = false;
                true
            }
            "Backspace" => {
                self.textarea.delete_char_before();
                self.saved = false;
                true
            }
            "Delete" => {
                self.textarea.delete_char();
                self.saved = false;
                true
            }
            
            // Regular character input
            _ if key.len() == 1 => {
                if let Some(ch) = key.chars().next() {
                    self.textarea.insert_char(ch);
                    self.saved = false;
                }
                true
            }
            
            _ => false,
        }
    }

    fn get_status_line(&self) -> String {
        let cursor = self.textarea.state.cursor;
        let modified = if self.saved { "" } else { "*" };
        let filename = self.filename.as_deref().unwrap_or("[untitled]");
        
        format!(
            "{}{} - Line {}, Column {} - {} lines",
            filename,
            modified,
            cursor.row + 1,
            cursor.col + 1,
            self.textarea.state.lines.len()
        )
    }
}
```

### Search and Replace Editor

```rust
struct SearchableEditor {
    textarea: Textarea,
    search_query: String,
    matches: Vec<(usize, usize)>,
    current_match: usize,
}

impl SearchableEditor {
    fn new() -> Self {
        let textarea = Textarea::builder("searchable-editor")
            .syntax_highlighting(true)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .viewport_size(20, 80)
            .build();

        Self {
            textarea,
            search_query: String::new(),
            matches: Vec::new(),
            current_match: 0,
        }
    }

    fn start_search(&mut self, query: String) {
        self.search_query = query.clone();
        self.textarea.search.set_query(query);
        self.find_matches();
    }

    fn find_matches(&mut self) {
        let text = self.textarea.text();
        self.matches = self.textarea.search.find_matches(&text);
        self.current_match = 0;
        
        if !self.matches.is_empty() {
            self.goto_match(0);
        }
    }

    fn next_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = (self.current_match + 1) % self.matches.len();
            self.goto_match(self.current_match);
        }
    }

    fn previous_match(&mut self) {
        if !self.matches.is_empty() {
            self.current_match = if self.current_match == 0 {
                self.matches.len() - 1
            } else {
                self.current_match - 1
            };
            self.goto_match(self.current_match);
        }
    }

    fn goto_match(&mut self, match_index: usize) {
        if let Some((start, _end)) = self.matches.get(match_index) {
            // Convert byte position to line/column
            let text = self.textarea.text();
            let mut line = 0;
            let mut col = 0;
            
            for (i, ch) in text.char_indices() {
                if i >= *start {
                    break;
                }
                if ch == '\n' {
                    line += 1;
                    col = 0;
                } else {
                    col += 1;
                }
            }
            
            self.textarea.state.cursor = CursorPosition { row: line, col };
        }
    }

    fn get_search_status(&self) -> String {
        if self.matches.is_empty() {
            format!("No matches for '{}'", self.search_query)
        } else {
            format!(
                "Match {} of {} for '{}'",
                self.current_match + 1,
                self.matches.len(),
                self.search_query
            )
        }
    }
}
```

### Collaborative Editor Base

```rust
use std::sync::{Arc, Mutex};

struct CollaborativeEditor {
    textarea: Arc<Mutex<Textarea>>,
    user_id: String,
    cursors: std::collections::HashMap<String, CursorPosition>,
}

impl CollaborativeEditor {
    fn new(user_id: String) -> Self {
        let textarea = Arc::new(Mutex::new(
            Textarea::builder("collaborative-editor")
                .syntax_highlighting(true)
                .show_line_numbers(true)
                .highlight_current_line(true)
                .viewport_size(25, 100)
                .build()
        ));

        Self {
            textarea,
            user_id,
            cursors: std::collections::HashMap::new(),
        }
    }

    fn apply_remote_operation(&mut self, operation: EditOperation, user_id: &str) {
        if let Ok(mut textarea) = self.textarea.lock() {
            // Apply operation without adding to local history
            operation.apply(&mut textarea.state.lines, &mut textarea.state.cursor);
            
            // Update remote user cursor position
            self.cursors.insert(user_id.to_string(), textarea.state.cursor);
        }
    }

    fn get_local_operation(&mut self, op: EditOperation) -> (EditOperation, CursorPosition) {
        let cursor = if let Ok(textarea) = self.textarea.lock() {
            textarea.state.cursor
        } else {
            CursorPosition::default()
        };
        
        (op, cursor)
    }

    fn render_with_cursors(&self) -> String {
        if let Ok(textarea) = self.textarea.lock() {
            let mut content = textarea.render().content.unwrap_or_default();
            
            // Add visual indicators for other users' cursors
            for (user, cursor) in &self.cursors {
                if user != &self.user_id {
                    content.push_str(&format!(" [{}@{}:{}]", user, cursor.row, cursor.col));
                }
            }
            
            content
        } else {
            String::new()
        }
    }
}
```

### Syntax-Aware Code Formatter

```rust
struct CodeFormatter {
    textarea: Textarea,
}

impl CodeFormatter {
    fn new() -> Self {
        let textarea = Textarea::builder("formatter")
            .syntax_highlighting(true)
            .show_line_numbers(true)
            .viewport_size(30, 120)
            .build();

        Self { textarea }
    }

    fn format_code(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let language = self.textarea.state.language.as_deref().unwrap_or("text");
        let content = self.textarea.text();
        
        let formatted = match language {
            "rust" => self.format_rust(&content)?,
            "javascript" => self.format_javascript(&content)?,
            "python" => self.format_python(&content)?,
            _ => content, // No formatting for unsupported languages
        };

        self.textarea.set_text(&formatted);
        Ok(())
    }

    fn format_rust(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Mock Rust formatting - in real implementation, use rustfmt
        let mut formatted = String::new();
        let mut indent_level = 0;
        
        for line in code.lines() {
            let trimmed = line.trim();
            
            if trimmed.ends_with('{') {
                formatted.push_str(&format!("{}{}\n", "    ".repeat(indent_level), trimmed));
                indent_level += 1;
            } else if trimmed.starts_with('}') {
                indent_level = indent_level.saturating_sub(1);
                formatted.push_str(&format!("{}{}\n", "    ".repeat(indent_level), trimmed));
            } else if !trimmed.is_empty() {
                formatted.push_str(&format!("{}{}\n", "    ".repeat(indent_level), trimmed));
            } else {
                formatted.push('\n');
            }
        }
        
        Ok(formatted)
    }

    fn format_javascript(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Mock JavaScript formatting - in real implementation, use prettier
        Ok(code.to_string()) // Placeholder
    }

    fn format_python(&self, code: &str) -> Result<String, Box<dyn std::error::Error>> {
        // Mock Python formatting - in real implementation, use black
        Ok(code.to_string()) // Placeholder
    }
}
```

### Advanced Editor with Custom Keybindings

```rust
use std::collections::HashMap;

#[derive(Debug, Clone)]
enum EditorCommand {
    MoveCursor(Direction),
    Insert(char),
    Delete(DeleteMode),
    Save,
    Undo,
    Redo,
    Search(String),
    Replace(String, String),
    ToggleLineNumbers,
    ToggleSyntaxHighlighting,
}

#[derive(Debug, Clone)]
enum Direction {
    Left, Right, Up, Down, LineStart, LineEnd,
}

#[derive(Debug, Clone)]
enum DeleteMode {
    Char, CharBefore, Line, Word,
}

struct AdvancedEditor {
    textarea: Textarea,
    keybindings: HashMap<String, EditorCommand>,
    mode: EditMode,
}

#[derive(Debug, Clone, PartialEq)]
enum EditMode {
    Normal, Insert, Visual,
}

impl AdvancedEditor {
    fn new() -> Self {
        let textarea = Textarea::builder("advanced-editor")
            .syntax_highlighting(true)
            .show_line_numbers(true)
            .highlight_current_line(true)
            .viewport_size(30, 100)
            .build();

        let mut keybindings = HashMap::new();
        
        // Vim-like keybindings for normal mode
        keybindings.insert("h".to_string(), EditorCommand::MoveCursor(Direction::Left));
        keybindings.insert("j".to_string(), EditorCommand::MoveCursor(Direction::Down));
        keybindings.insert("k".to_string(), EditorCommand::MoveCursor(Direction::Up));
        keybindings.insert("l".to_string(), EditorCommand::MoveCursor(Direction::Right));
        keybindings.insert("0".to_string(), EditorCommand::MoveCursor(Direction::LineStart));
        keybindings.insert("$".to_string(), EditorCommand::MoveCursor(Direction::LineEnd));
        keybindings.insert("x".to_string(), EditorCommand::Delete(DeleteMode::Char));
        keybindings.insert("dd".to_string(), EditorCommand::Delete(DeleteMode::Line));
        keybindings.insert("u".to_string(), EditorCommand::Undo);
        keybindings.insert("Ctrl+r".to_string(), EditorCommand::Redo);
        keybindings.insert(":w".to_string(), EditorCommand::Save);

        Self {
            textarea,
            keybindings,
            mode: EditMode::Normal,
        }
    }

    fn handle_key(&mut self, key: &str) -> bool {
        match self.mode {
            EditMode::Normal => self.handle_normal_mode(key),
            EditMode::Insert => self.handle_insert_mode(key),
            EditMode::Visual => self.handle_visual_mode(key),
        }
    }

    fn handle_normal_mode(&mut self, key: &str) -> bool {
        if let Some(command) = self.keybindings.get(key).cloned() {
            self.execute_command(command);
            true
        } else {
            match key {
                "i" => {
                    self.mode = EditMode::Insert;
                    true
                }
                "v" => {
                    self.mode = EditMode::Visual;
                    true
                }
                "Escape" => true, // Stay in normal mode
                _ => false,
            }
        }
    }

    fn handle_insert_mode(&mut self, key: &str) -> bool {
        match key {
            "Escape" => {
                self.mode = EditMode::Normal;
                true
            }
            key if key.len() == 1 => {
                if let Some(ch) = key.chars().next() {
                    self.textarea.insert_char(ch);
                }
                true
            }
            "Enter" => {
                self.textarea.insert_newline();
                true
            }
            "Backspace" => {
                self.textarea.delete_char_before();
                true
            }
            _ => false,
        }
    }

    fn handle_visual_mode(&mut self, key: &str) -> bool {
        match key {
            "Escape" => {
                self.mode = EditMode::Normal;
                true
            }
            // Add visual mode commands here
            _ => false,
        }
    }

    fn execute_command(&mut self, command: EditorCommand) {
        match command {
            EditorCommand::MoveCursor(direction) => {
                match direction {
                    Direction::Left => self.textarea.move_cursor_left(),
                    Direction::Right => self.textarea.move_cursor_right(),
                    Direction::Up => self.textarea.move_cursor_up(),
                    Direction::Down => self.textarea.move_cursor_down(),
                    Direction::LineStart => self.textarea.move_cursor_to_line_start(),
                    Direction::LineEnd => self.textarea.move_cursor_to_line_end(),
                }
            }
            EditorCommand::Insert(ch) => self.textarea.insert_char(ch),
            EditorCommand::Delete(mode) => {
                match mode {
                    DeleteMode::Char => self.textarea.delete_char(),
                    DeleteMode::CharBefore => self.textarea.delete_char_before(),
                    DeleteMode::Line => {/* implement line deletion */}
                    DeleteMode::Word => {/* implement word deletion */}
                }
            }
            EditorCommand::Undo => { self.textarea.undo(); }
            EditorCommand::Redo => { self.textarea.redo(); }
            EditorCommand::Save => {/* implement save */}
            EditorCommand::Search(_query) => {/* implement search */}
            EditorCommand::Replace(_find, _replace) => {/* implement replace */}
            EditorCommand::ToggleLineNumbers => {
                self.textarea.state.show_line_numbers = !self.textarea.state.show_line_numbers;
            }
            EditorCommand::ToggleSyntaxHighlighting => {
                self.textarea.state.syntax_highlighting = !self.textarea.state.syntax_highlighting;
            }
        }
    }

    fn get_mode_indicator(&self) -> &str {
        match self.mode {
            EditMode::Normal => "NORMAL",
            EditMode::Insert => "INSERT",
            EditMode::Visual => "VISUAL",
        }
    }
}
```

## Integration with UI Components

```rust
use reactive_tui::{widgets::*, components::*};

let editor_panel = Element::with_tag("div")
    .class("editor-panel")
    .child(
        Element::with_tag("div")
            .class("editor-toolbar")
            .child(
                Element::with_tag("button")
                    .text("Save")
                    .class("toolbar-button")
                    .build()
            )
            .child(
                Element::with_tag("button")
                    .text("Undo")
                    .class("toolbar-button")
                    .build()
            )
            .build()
    )
    .child(
        textarea.render()
    )
    .child(
        Element::with_tag("div")
            .class("status-bar")
            .text(&format!("Line {}, Column {}", cursor.row + 1, cursor.col + 1))
            .build()
    )
    .build();
```

## CSS Styling

The textarea generates semantic CSS classes:

```css
.textarea {
    /* Base textarea styles */
}

.textarea.focused {
    /* Focused state with border highlighting */
}

.textarea.readonly {
    /* Read-only state with disabled appearance */
}

.textarea.modified {
    /* Modified/unsaved state indicator */
}

.textarea.highlight-current-line {
    /* Current line highlighting enabled */
}

.textarea.show-line-numbers {
    /* Line numbers displayed */
}

.textarea.syntax-highlighted {
    /* Syntax highlighting enabled */
}

/* Syntax highlighting classes */
.keyword { color: #c678dd; }
.string { color: #98c379; }
.comment { color: #5c6370; }
.function { color: #61afef; }
.number { color: #d19a66; }
```

## Performance Considerations

- **Viewport Rendering**: Only renders visible text lines for efficient handling of large documents
- **Incremental Operations**: Edit operations are applied incrementally rather than reprocessing entire document
- **History Management**: Configurable history size prevents memory bloat in long editing sessions
- **Syntax Highlighting**: Cached highlighting results and line-by-line processing for optimal performance
- **Search Optimization**: Efficient search algorithms with result caching

## Accessibility

- **Keyboard Navigation**: Full keyboard accessibility with standard editor keybindings
- **Screen Reader**: Proper text content exposure and cursor position announcements
- **Focus Management**: Clear focus indicators and proper tab navigation
- **ARIA Attributes**: Complete ARIA labeling for editor state and content

## Advanced Features

### Custom Syntax Highlighting

```rust
// Implement custom syntax highlighter
impl Textarea {
    pub fn set_custom_highlighter<F>(&mut self, highlighter: F) 
    where 
        F: Fn(&str, &str) -> String + Send + Sync + 'static 
    {
        // Custom highlighting implementation
    }
}
```

### Language Server Integration

```rust
// Mock language server integration
struct LanguageServerEditor {
    textarea: Textarea,
    diagnostics: Vec<Diagnostic>,
    completions: Vec<Completion>,
}

impl LanguageServerEditor {
    fn get_hover_info(&self, position: CursorPosition) -> Option<String> {
        // Return hover information for position
        None
    }

    fn trigger_completion(&mut self) -> Vec<Completion> {
        // Trigger completion at cursor position
        Vec::new()
    }

    fn get_diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }
}
```

The Textarea widget provides comprehensive multi-line text editing functionality with advanced features including syntax highlighting, undo/redo operations, search capabilities, and extensive customization options for building sophisticated text editors and code editing interfaces.