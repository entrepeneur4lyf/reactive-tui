---
sidebar_position: 4
---

# CSS API

Full CSS support with Flexbox, Grid, animations, and responsive design in the terminal.

## CSS Engine

The CSS engine provides comprehensive styling capabilities with familiar web CSS syntax.

### CssEngine

Core CSS parsing and application engine.

```rust
use reactive_tui::css::*;

let mut css_engine = CssEngine::new();

// Load CSS from string
css_engine.load_css(r#"
    .container {
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding: 2rem;
        background: #1e1e1e;
        color: #ffffff;
    }
    
    .btn {
        padding: 0.5rem 1rem;
        border: 1px solid #333;
        border-radius: 4px;
        background: #007acc;
        color: white;
    }
    
    .btn:hover {
        background: #005a9e;
    }
    
    .btn:focus {
        outline: 2px solid #ffaa00;
    }
"#)?;

// Load from file
css_engine.load_stylesheet("assets/styles.css")?;

// Apply styles to element
let element = Element::with_tag("button")
    .class("btn")
    .content("Click Me")
    .build();

let computed_styles = css_engine.compute_styles(&element)?;
```

### Stylesheet

CSS stylesheet management and parsing.

```rust
let stylesheet = Stylesheet::from_css(r#"
    /* Layout */
    .grid {
        display: grid;
        grid-template-columns: repeat(3, 1fr);
        gap: 1rem;
    }
    
    /* Typography */
    .title {
        font-size: 1.5rem;
        font-weight: bold;
        margin-bottom: 1rem;
    }
    
    /* Colors */
    .primary { color: #007acc; }
    .secondary { color: #6c757d; }
    .success { color: #28a745; }
    .warning { color: #ffc107; }
    .danger { color: #dc3545; }
"#)?;

// Merge multiple stylesheets
let combined = stylesheet.merge(&other_stylesheet)?;

// Export to CSS
let css_output = stylesheet.to_css();
```

## CSS Properties

### Layout Properties

#### Display
```css
.container {
    display: flex;        /* Flexbox layout */
    display: grid;        /* CSS Grid layout */
    display: block;       /* Block-level element */
    display: inline;      /* Inline element */
    display: none;        /* Hidden element */
}
```

#### Flexbox
```css
.flex-container {
    display: flex;
    flex-direction: row | column | row-reverse | column-reverse;
    justify-content: flex-start | flex-end | center | space-between | space-around | space-evenly;
    align-items: flex-start | flex-end | center | stretch | baseline;
    flex-wrap: nowrap | wrap | wrap-reverse;
    gap: 1rem;
}

.flex-item {
    flex: 1;              /* Grow and shrink */
    flex-grow: 1;         /* Grow factor */
    flex-shrink: 0;       /* Shrink factor */
    flex-basis: auto;     /* Initial size */
    align-self: auto | flex-start | flex-end | center | stretch;
}
```

#### CSS Grid
```css
.grid-container {
    display: grid;
    grid-template-columns: 200px 1fr 100px;
    grid-template-rows: auto 1fr auto;
    grid-template-areas: 
        "header header header"
        "sidebar main aside"
        "footer footer footer";
    gap: 1rem;
    grid-gap: 1rem;       /* Legacy syntax */
}

.grid-item {
    grid-area: header;
    grid-column: 1 / 3;
    grid-row: 1 / 2;
    grid-column-start: 1;
    grid-column-end: 3;
    grid-row-start: 1;
    grid-row-end: 2;
}
```

### Sizing Properties

```css
.sized-element {
    width: 100px;         /* Fixed width */
    height: 50px;         /* Fixed height */
    min-width: 10px;      /* Minimum width */
    max-width: 500px;     /* Maximum width */
    min-height: 5px;      /* Minimum height */
    max-height: 300px;    /* Maximum height */
}

/* Responsive units */
.responsive {
    width: 100%;          /* Percentage of parent */
    width: 50ch;          /* Character units */
    width: 20rem;         /* Relative to root font-size */
    width: 1.5em;         /* Relative to element font-size */
    width: 100vw;         /* Viewport width */
    width: 100vh;         /* Viewport height */
}
```

### Spacing Properties

```css
.spaced-element {
    /* Margin */
    margin: 1rem;                    /* All sides */
    margin: 1rem 2rem;              /* Vertical | Horizontal */
    margin: 1rem 2rem 3rem 4rem;    /* Top | Right | Bottom | Left */
    margin-top: 1rem;
    margin-right: 2rem;
    margin-bottom: 3rem;
    margin-left: 4rem;
    
    /* Padding */
    padding: 0.5rem;                 /* All sides */
    padding: 0.5rem 1rem;           /* Vertical | Horizontal */
    padding: 0.5rem 1rem 1.5rem 2rem; /* Top | Right | Bottom | Left */
    padding-top: 0.5rem;
    padding-right: 1rem;
    padding-bottom: 1.5rem;
    padding-left: 2rem;
}
```

### Color Properties

```css
.colored-element {
    /* Text color */
    color: #ffffff;
    color: rgb(255, 255, 255);
    color: hsl(0, 0%, 100%);
    
    /* Background color */
    background: #1e1e1e;
    background-color: rgb(30, 30, 30);
    
    /* Gradients */
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    background: radial-gradient(circle, #ff6b6b, #4ecdc4);
}

/* Semantic colors */
.semantic {
    color: var(--primary);
    color: var(--secondary);
    color: var(--success);
    color: var(--warning);
    color: var(--danger);
    background: var(--bg-primary);
    background: var(--bg-secondary);
}
```

### Border Properties

```css
.bordered-element {
    /* Border shorthand */
    border: 1px solid #333;
    
    /* Individual borders */
    border-top: 2px solid #007acc;
    border-right: 1px dashed #666;
    border-bottom: 3px double #999;
    border-left: 1px dotted #ccc;
    
    /* Border properties */
    border-width: 1px;
    border-style: solid | dashed | dotted | double | none;
    border-color: #333;
    
    /* Border radius */
    border-radius: 4px;
    border-radius: 4px 8px;          /* Top-left/bottom-right | Top-right/bottom-left */
    border-radius: 4px 8px 12px 16px; /* Top-left | Top-right | Bottom-right | Bottom-left */
}
```

### Typography Properties

```css
.text-element {
    font-size: 1rem;         /* Font size */
    font-weight: normal;     /* normal | bold | 100-900 */
    font-style: normal;      /* normal | italic */
    text-align: left;        /* left | center | right | justify */
    text-decoration: none;   /* none | underline | line-through */
    line-height: 1.4;       /* Line height multiplier */
    letter-spacing: 0.1em;  /* Letter spacing */
    word-spacing: 0.2em;    /* Word spacing */
}
```

## CSS Selectors

### Basic Selectors

```css
/* Element selector */
button {
    background: #007acc;
}

/* Class selector */
.btn {
    padding: 0.5rem 1rem;
}

/* ID selector */
#main-button {
    font-weight: bold;
}

/* Universal selector */
* {
    box-sizing: border-box;
}
```

### Combinators

```css
/* Descendant combinator */
.container button {
    margin: 0.25rem;
}

/* Child combinator */
.navbar > .nav-item {
    display: inline-block;
}

/* Adjacent sibling combinator */
.label + .input {
    margin-left: 0.5rem;
}

/* General sibling combinator */
.heading ~ .paragraph {
    margin-top: 1rem;
}
```

### Pseudo-classes

```css
/* Interactive states */
.btn:hover {
    background: #005a9e;
}

.btn:focus {
    outline: 2px solid #ffaa00;
}

.btn:active {
    transform: translateY(1px);
}

.btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

/* Structural pseudo-classes */
.item:first-child {
    border-top: none;
}

.item:last-child {
    border-bottom: none;
}

.item:nth-child(odd) {
    background: #f5f5f5;
}

.item:nth-child(2n) {
    background: #ffffff;
}
```

## Responsive Design

### Media Queries

```css
/* Terminal width-based queries */
@media (max-width: 80ch) {
    .desktop-only {
        display: none;
    }
    
    .grid {
        grid-template-columns: 1fr;
    }
}

@media (min-width: 120ch) {
    .container {
        max-width: 100ch;
        margin: 0 auto;
    }
}

/* Terminal height-based queries */
@media (max-height: 24) {
    .sidebar {
        display: none;
    }
}

/* Feature-based queries */
@media (color) {
    .color-text {
        color: #007acc;
    }
}

@media (monochrome) {
    .emphasis {
        font-weight: bold;
    }
}
```

### Responsive Units

```css
.responsive-element {
    /* Character-based units */
    width: 40ch;          /* 40 characters wide */
    max-width: 80ch;      /* Maximum 80 characters */
    
    /* Viewport units */
    width: 50vw;          /* 50% of terminal width */
    height: 30vh;         /* 30% of terminal height */
    
    /* Relative units */
    font-size: 1.2rem;    /* Relative to root font size */
    padding: 0.5em;       /* Relative to element font size */
    
    /* Percentage units */
    width: 100%;          /* Full parent width */
    margin-left: 25%;     /* 25% of parent width */
}
```

## Utility Classes

### Layout Utilities

```css
/* Display */
.flex { display: flex; }
.grid { display: grid; }
.block { display: block; }
.inline { display: inline; }
.hidden { display: none; }

/* Flexbox utilities */
.flex-row { flex-direction: row; }
.flex-col { flex-direction: column; }
.justify-start { justify-content: flex-start; }
.justify-center { justify-content: center; }
.justify-end { justify-content: flex-end; }
.justify-between { justify-content: space-between; }
.items-start { align-items: flex-start; }
.items-center { align-items: center; }
.items-end { align-items: flex-end; }
.items-stretch { align-items: stretch; }

/* Grid utilities */
.grid-cols-1 { grid-template-columns: repeat(1, 1fr); }
.grid-cols-2 { grid-template-columns: repeat(2, 1fr); }
.grid-cols-3 { grid-template-columns: repeat(3, 1fr); }
.grid-cols-4 { grid-template-columns: repeat(4, 1fr); }
```

### Spacing Utilities

```css
/* Margin utilities */
.m-0 { margin: 0; }
.m-1 { margin: 0.25rem; }
.m-2 { margin: 0.5rem; }
.m-3 { margin: 0.75rem; }
.m-4 { margin: 1rem; }

.mx-auto { margin-left: auto; margin-right: auto; }
.my-2 { margin-top: 0.5rem; margin-bottom: 0.5rem; }

/* Padding utilities */
.p-0 { padding: 0; }
.p-1 { padding: 0.25rem; }
.p-2 { padding: 0.5rem; }
.p-3 { padding: 0.75rem; }
.p-4 { padding: 1rem; }

.px-2 { padding-left: 0.5rem; padding-right: 0.5rem; }
.py-1 { padding-top: 0.25rem; padding-bottom: 0.25rem; }
```

### Color Utilities

```css
/* Text colors */
.text-primary { color: var(--primary); }
.text-secondary { color: var(--secondary); }
.text-success { color: var(--success); }
.text-warning { color: var(--warning); }
.text-danger { color: var(--danger); }

/* Background colors */
.bg-primary { background: var(--bg-primary); }
.bg-secondary { background: var(--bg-secondary); }
.bg-dark { background: var(--bg-dark); }
.bg-light { background: var(--bg-light); }
```

## Animations

### CSS Animations

```css
/* Keyframe animations */
@keyframes fade-in {
    from { opacity: 0; }
    to { opacity: 1; }
}

@keyframes slide-in {
    from { transform: translateX(-100%); }
    to { transform: translateX(0); }
}

@keyframes bounce {
    0%, 20%, 53%, 80%, 100% {
        transform: translateY(0);
    }
    40%, 43% {
        transform: translateY(-8px);
    }
    70% {
        transform: translateY(-4px);
    }
    90% {
        transform: translateY(-2px);
    }
}

/* Animation properties */
.animated {
    animation-name: fade-in;
    animation-duration: 0.5s;
    animation-timing-function: ease-in-out;
    animation-delay: 0.1s;
    animation-iteration-count: 1;
    animation-direction: normal;
    animation-fill-mode: forwards;
}

/* Animation shorthand */
.fade-in {
    animation: fade-in 0.5s ease-in-out 0.1s forwards;
}

.bounce {
    animation: bounce 1s infinite;
}
```

### Transitions

```css
.transition-element {
    transition: all 0.3s ease;
    transition-property: background, color, transform;
    transition-duration: 0.3s;
    transition-timing-function: ease-in-out;
    transition-delay: 0.1s;
}

.button {
    background: #007acc;
    transition: background 0.2s ease;
}

.button:hover {
    background: #005a9e;
}
```

## CSS Variables

### Custom Properties

```css
:root {
    /* Color scheme */
    --primary: #007acc;
    --secondary: #6c757d;
    --success: #28a745;
    --warning: #ffc107;
    --danger: #dc3545;
    
    /* Background colors */
    --bg-primary: #ffffff;
    --bg-secondary: #f8f9fa;
    --bg-dark: #1e1e1e;
    
    /* Spacing */
    --spacing-xs: 0.25rem;
    --spacing-sm: 0.5rem;
    --spacing-md: 1rem;
    --spacing-lg: 1.5rem;
    --spacing-xl: 2rem;
    
    /* Typography */
    --font-size-sm: 0.875rem;
    --font-size-base: 1rem;
    --font-size-lg: 1.125rem;
    --font-size-xl: 1.25rem;
    
    /* Borders */
    --border-width: 1px;
    --border-radius: 4px;
    --border-color: #dee2e6;
}

/* Using variables */
.component {
    color: var(--primary);
    background: var(--bg-primary);
    padding: var(--spacing-md);
    border: var(--border-width) solid var(--border-color);
    border-radius: var(--border-radius);
}
```

## CSS Integration

### Programmatic CSS

```rust
use reactive_tui::css::*;

// Dynamic CSS generation
let css_engine = CssEngine::new();

// Add CSS rules programmatically
css_engine.add_rule(CssRule {
    selector: Selector::Class("dynamic-button".to_string()),
    properties: vec![
        ("background".to_string(), format!("#{:06x}", color_value)),
        ("color".to_string(), "white".to_string()),
        ("padding".to_string(), "0.5rem 1rem".to_string()),
    ],
})?;

// Generate utility classes
let utility_processor = UtilityProcessor::new();
let utilities = utility_processor.generate_utilities(&UtilityPalette::default())?;
css_engine.add_utilities(utilities)?;
```

### CSS-in-Rust

```rust
use reactive_tui::prelude::*;

// Inline styles
let styled_element = Element::with_tag("div")
    .class("container")
    .style(r#"
        display: flex;
        flex-direction: column;
        gap: 1rem;
        padding: 2rem;
        background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
        border-radius: 8px;
        box-shadow: 0 4px 6px rgba(0, 0, 0, 0.1);
    "#)
    .build();

// Dynamic styles based on state
let dynamic_styles = format!(r#"
    background: {};
    color: {};
    border: 2px solid {};
"#, 
    if is_active { "#007acc" } else { "#6c757d" },
    if is_active { "white" } else { "black" },
    if has_error { "#dc3545" } else { "transparent" }
);

let conditional_element = Element::with_tag("button")
    .style(&dynamic_styles)
    .build();
```

### CSS Validation

```rust
use reactive_tui::css::*;

// Validate CSS before applying
let css_content = r#"
    .invalid-selector {
        invalid-property: invalid-value;
        background: invalid-color;
    }
"#;

match CssEngine::validate_css(css_content) {
    Ok(_) => println!("CSS is valid"),
    Err(errors) => {
        for error in errors {
            eprintln!("CSS Error: {}", error);
        }
    }
}
```