# CSS Module

The `css` module provides a complete CSS engine for styling terminal applications with familiar web CSS syntax.

## CssEngine

The main CSS processing engine that parses stylesheets and applies styles to widgets.

```rust
use reactive_tui::css::{CssEngine, Stylesheet};

let css = r#"
    .button {
        background-color: #007acc;
        color: white;
        border: 1px solid #005a9e;
        padding: 1px 2px;
        border-radius: 2px;
    }
    
    .button:hover {
        background-color: #005a9e;
    }
"#;

let stylesheet = Stylesheet::parse(css)?;
let mut engine = CssEngine::new();
engine.add_stylesheet(stylesheet)?;
```

### Methods

#### `new() -> CssEngine`
Creates a new CSS engine instance.

#### `add_stylesheet(&mut self, stylesheet: Stylesheet) -> Result<()>`
Adds a parsed stylesheet to the engine.

**Parameters:**
- `stylesheet` - A parsed CSS stylesheet

**Returns:** `Result<(), TuiError>`

#### `apply_styles(&self, element: &mut Element) -> Result<()>`
Applies matching CSS rules to an element.

**Parameters:**
- `element` - The element to apply styles to

**Returns:** `Result<(), TuiError>`

#### `get_computed_styles(&self, selector: &str) -> ComputedStyles`
Gets the computed styles for a selector.

**Parameters:**
- `selector` - CSS selector string

**Returns:** `ComputedStyles` object

#### `clear(&mut self)`
Clears all stylesheets from the engine.

## Stylesheet

Represents a parsed CSS stylesheet containing rules and selectors.

```rust
use reactive_tui::css::Stylesheet;

let css = r#"
    /* Layout styles */
    .container {
        display: flex;
        flex-direction: column;
        width: 100%;
        height: 100%;
        padding: 2px;
    }
    
    /* Button styles */
    button {
        background-color: #333;
        color: #fff;
        border: 1px solid #555;
        padding: 1px 3px;
        margin: 1px;
    }
    
    button:hover {
        background-color: #555;
    }
    
    button:active {
        background-color: #222;
    }
    
    /* Utility classes */
    .text-center { text-align: center; }
    .text-bold { font-weight: bold; }
    .bg-primary { background-color: #007acc; }
    .bg-success { background-color: #28a745; }
    .bg-danger { background-color: #dc3545; }
"#;

let stylesheet = Stylesheet::parse(css)?;
```

### Methods

#### `parse(css: &str) -> Result<Stylesheet>`
Parses CSS text into a stylesheet.

**Parameters:**
- `css` - CSS text to parse

**Returns:** `Result<Stylesheet, TuiError>`

#### `add_rule(&mut self, rule: CssRule)`
Adds a CSS rule to the stylesheet.

**Parameters:**
- `rule` - The CSS rule to add

#### `get_rules(&self) -> &[CssRule]`
Gets all CSS rules in the stylesheet.

**Returns:** Slice of CSS rules

#### `merge(&mut self, other: Stylesheet)`
Merges another stylesheet into this one.

**Parameters:**
- `other` - Stylesheet to merge

## CssRule

Represents a single CSS rule with selector and declarations.

```rust
use reactive_tui::css::{CssRule, Selector};

// Create a rule programmatically
let rule = CssRule::new(
    Selector::Class("button".to_string()),
    vec![
        ("background-color".to_string(), "#007acc".to_string()),
        ("color".to_string(), "white".to_string()),
        ("padding".to_string(), "1px 2px".to_string()),
    ]
);
```

### Methods

#### `new(selector: Selector, declarations: Vec<(String, String)>) -> CssRule`
Creates a new CSS rule.

**Parameters:**
- `selector` - The CSS selector
- `declarations` - Vector of property-value pairs

#### `matches(&self, element: &Element) -> bool`
Checks if this rule matches the given element.

**Parameters:**
- `element` - Element to test against

**Returns:** `true` if the rule matches

#### `get_selector(&self) -> &Selector`
Gets the rule's selector.

#### `get_declarations(&self) -> &[(String, String)]`
Gets the rule's CSS declarations.

## Selector

Enum representing different types of CSS selectors.

```rust
use reactive_tui::css::Selector;

// Different selector types
let tag_selector = Selector::Tag("button".to_string());
let class_selector = Selector::Class("primary".to_string());  
let id_selector = Selector::Id("submit-btn".to_string());
let universal_selector = Selector::Universal;
```

### Variants

#### `Tag(String)`
Selects elements by tag name (e.g., `button`, `input`).

#### `Class(String)`  
Selects elements by class name (e.g., `.primary`, `.large`).

#### `Id(String)`
Selects elements by ID (e.g., `#submit-btn`, `#main-form`).

#### `Universal`
Universal selector that matches all elements (`*`).

#### `Descendant(Box<Selector>, Box<Selector>)`
Descendant combinator (e.g., `.container button`).

#### `Child(Box<Selector>, Box<Selector>)`
Child combinator (e.g., `.form > input`).

#### `PseudoClass(Box<Selector>, String)`
Pseudo-class selector (e.g., `button:hover`, `input:focus`).

## Supported CSS Properties

### Layout Properties

```css
/* Display and positioning */
display: block | inline | flex | grid | none;
position: relative | absolute | fixed;
top: <length> | <percentage>;
right: <length> | <percentage>;
bottom: <length> | <percentage>;  
left: <length> | <percentage>;

/* Dimensions */
width: <length> | <percentage> | auto;
height: <length> | <percentage> | auto;
min-width: <length> | <percentage>;
max-width: <length> | <percentage>;
min-height: <length> | <percentage>;
max-height: <length> | <percentage>;

/* Spacing */
margin: <length>;
margin-top: <length>;
margin-right: <length>;
margin-bottom: <length>;
margin-left: <length>;
padding: <length>;
padding-top: <length>;
padding-right: <length>;
padding-bottom: <length>;
padding-left: <length>;
```

### Flexbox Properties

```css
/* Flex container */
flex-direction: row | column | row-reverse | column-reverse;
flex-wrap: nowrap | wrap | wrap-reverse;
justify-content: flex-start | flex-end | center | space-between | space-around | space-evenly;
align-items: flex-start | flex-end | center | baseline | stretch;
align-content: flex-start | flex-end | center | space-between | space-around | stretch;
gap: <length>;

/* Flex items */
flex: <number>;
flex-grow: <number>;
flex-shrink: <number>;
flex-basis: <length> | auto;
align-self: auto | flex-start | flex-end | center | baseline | stretch;
order: <integer>;
```

### Grid Properties

```css
/* Grid container */
display: grid;
grid-template-columns: <track-list>;
grid-template-rows: <track-list>;
grid-template-areas: <string>;
grid-gap: <length>;
grid-column-gap: <length>;
grid-row-gap: <length>;
justify-items: start | end | center | stretch;
align-items: start | end | center | stretch;

/* Grid items */
grid-column: <line>;
grid-row: <line>;
grid-column-start: <line>;
grid-column-end: <line>;
grid-row-start: <line>;
grid-row-end: <line>;
grid-area: <name>;
justify-self: start | end | center | stretch;
align-self: start | end | center | stretch;
```

### Visual Properties

```css
/* Colors */
color: <color>;
background-color: <color>;

/* Borders */
border: <width> <style> <color>;
border-width: <length>;
border-style: solid | dashed | dotted | double | none;
border-color: <color>;
border-radius: <length>;
border-top: <width> <style> <color>;
border-right: <width> <style> <color>;
border-bottom: <width> <style> <color>;
border-left: <width> <style> <color>;

/* Text */
text-align: left | center | right | justify;
font-weight: normal | bold | <number>;
font-style: normal | italic;
text-decoration: none | underline | line-through;
```

## Color Formats

The CSS engine supports various color formats:

```css
/* Named colors */
color: red;
color: blue;
color: green;
color: black;
color: white;

/* Hex colors */
color: #ff0000;    /* Red */
color: #00ff00;    /* Green */
color: #0000ff;    /* Blue */
color: #fff;       /* White (short form) */

/* RGB colors */
color: rgb(255, 0, 0);           /* Red */
color: rgb(0, 255, 0);           /* Green */
color: rgba(255, 0, 0, 0.5);     /* Semi-transparent red */

/* HSL colors */
color: hsl(0, 100%, 50%);        /* Red */
color: hsl(120, 100%, 50%);      /* Green */
color: hsla(240, 100%, 50%, 0.5); /* Semi-transparent blue */
```

## Advanced Features

### Responsive Design

```css 
/* Media queries for different terminal sizes */
@media (min-width: 80) {
    .container {
        max-width: 78;
        margin: 0 auto;
    }
}

@media (max-width: 40) {
    .sidebar {
        display: none;
    }
    
    .main-content {
        width: 100%;
    }
}
```

### CSS Custom Properties (Variables)

```css
:root {
    --primary-color: #007acc;
    --secondary-color: #6c757d;
    --border-radius: 2px;
    --spacing-unit: 1;
}

.button {
    background-color: var(--primary-color);
    border-radius: var(--border-radius);
    padding: var(--spacing-unit);
}

.button-secondary {
    background-color: var(--secondary-color);
}
```

### Pseudo-classes

```css
/* Interactive states */
button:hover {
    background-color: #005a9e;
}

button:active {
    background-color: #004080;
}

button:focus {
    outline: 1px solid #80bdff;
}

button:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* Structural pseudo-classes */
li:first-child {
    margin-top: 0;
}

li:last-child {
    margin-bottom: 0;
}

tr:nth-child(even) {
    background-color: #f8f9fa;
}
```

## Example: Complete Theme

```rust
use reactive_tui::css::Stylesheet;

let theme_css = r#"
/* Base theme variables */
:root {
    --bg-primary: #1e1e1e;
    --bg-secondary: #252525;
    --text-primary: #e0e0e0;
    --text-secondary: #a0a0a0;
    --accent-blue: #4d9de0;
    --accent-green: #6bcb77;
    --accent-red: #ff6b6b;
    --border-color: #333;
}

/* Layout */
.app-container {
    display: flex;
    flex-direction: column;
    width: 100%;
    height: 100%;
    background-color: var(--bg-primary);
    color: var(--text-primary);
}

.header {
    background-color: var(--bg-secondary);
    border-bottom: 1px solid var(--border-color);
    padding: 1px 2px;
}

.main-content {
    flex: 1;
    display: flex;
    flex-direction: row;
}

.sidebar {
    width: 20;
    background-color: var(--bg-secondary);
    border-right: 1px solid var(--border-color);
    padding: 1px;
}

.content {
    flex: 1;
    padding: 2px;
}

/* Components */
.button {
    background-color: var(--accent-blue);
    color: white;
    border: 1px solid var(--accent-blue);
    padding: 1px 2px;
    border-radius: 1px;
    cursor: pointer;
}

.button:hover {
    background-color: #2b8fd9;
    border-color: #2b8fd9;
}

.button-success {
    background-color: var(--accent-green);
    border-color: var(--accent-green);
}

.button-danger {
    background-color: var(--accent-red);
    border-color: var(--accent-red);
}

.input {
    background-color: var(--bg-primary);
    color: var(--text-primary);
    border: 1px solid var(--border-color);
    padding: 1px;
}

.input:focus {
    border-color: var(--accent-blue);
    outline: none;
}

/* Utility classes */
.text-center { text-align: center; }
.text-bold { font-weight: bold; }
.mt-1 { margin-top: 1; }
.mb-1 { margin-bottom: 1; }
.p-1 { padding: 1; }
.p-2 { padding: 2; }
"#;

let stylesheet = Stylesheet::parse(theme_css)?;
app.add_stylesheet(stylesheet)?;
```