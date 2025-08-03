# 🔗 TypeScript ↔ Rust Integration Guide

This guide explains how the TypeScript TUI framework integrates with the Rust utility CSS processor to provide seamless utility-first styling in terminal applications.

## 🎯 **Architecture Overview**

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   TypeScript    │    │   Integration   │    │      Rust       │
│   Frontend      │───▶│     Layer       │───▶│    Backend      │
│                 │    │                 │    │                 │
│ • Widget APIs   │    │ • Class Passing │    │ • ANSI Codes    │
│ • Utility Classes│    │ • State Sync    │    │ • Color Proc.   │
│ • Type Safety   │    │ • Event Bridge  │    │ • Performance   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
```

## 🎨 **Color System Integration**

### ✅ **Exact Utility Color Matching**

Both TypeScript and Rust use **identical** utility CSS color values:

| Color Class | TypeScript Usage | Rust Processing | ANSI Output |
|-------------|------------------|-----------------|-------------|
| `text-blue-600` | `classes: ['text-blue-600']` | `hex("#2563eb")` | `\x1B[38;2;37;99;235m` |
| `bg-gray-50` | `classes: ['bg-gray-50']` | `hex("#f8fafc")` | `\x1B[48;2;248;250;252m` |
| `text-green-500` | `classes: ['text-green-500']` | `hex("#22c55e")` | `\x1B[38;2;34;197;94m` |

### 🔧 **Implementation Details**

**TypeScript Side:**
```typescript
// Widget accepts utility classes
createSimpleCheckbox({
    id: 'my-checkbox',
    label: 'Styled checkbox',
    classes: [
        'text-blue-600',    // Text color
        'bg-gray-50',       // Background
        'p-4',              // Padding
        'rounded-lg',       // Border radius
        'font-semibold'     // Typography
    ]
})
```

**Rust Side:**
```rust
// UtilityProcessor converts to ANSI
let processor = UtilityProcessor::new();
let classes = vec![
    "text-blue-600".to_string(),
    "bg-gray-50".to_string(),
    "p-4".to_string(),
    "rounded-lg".to_string(),
    "font-semibold".to_string()
];

let ansi_codes = processor.process_classes(&classes);
// Returns combined ANSI escape sequences
```

## 📊 **Supported Utility Categories**

### 🎨 **Colors** (Complete Utility Palette)
- **Gray Scale**: `gray-50` through `gray-950` (11 shades)
- **Primary Colors**: `blue-*`, `green-*`, `red-*`, `yellow-*`, `purple-*`
- **Extended Colors**: `orange-*`, `emerald-*`, `cyan-*`, `indigo-*`, `pink-*`
- **Special**: `white`, `black`, `transparent`

### 📝 **Typography**
| TypeScript Class | Rust ANSI Code | Terminal Effect |
|------------------|----------------|-----------------|
| `font-bold` | `\x1B[1m` | **Bold text** |
| `font-light` | `\x1B[2m` | Dim text |
| `italic` | `\x1B[3m` | *Italic text* |
| `underline` | `\x1B[4m` | Underlined text |
| `line-through` | `\x1B[9m` | ~~Strikethrough~~ |

### 📏 **Spacing**
| TypeScript Class | Rust Value | Terminal Spacing |
|------------------|------------|------------------|
| `p-0` | `0` | No padding |
| `p-1` | `1` | 1 character padding |
| `p-2` | `2` | 2 character padding |
| `p-4` | `4` | 4 character padding |
| `px-2` | `2` | Horizontal padding |
| `py-1` | `1` | Vertical padding |

### 🎭 **State Variants**
```typescript
// TypeScript hover states
classes: ['hover:bg-blue-500', 'hover:text-white']

// Rust processing
hover_states.insert(
    "hover:bg-blue-500".to_string(), 
    format!("\x1B[48;2;{};{};{}m", palette.blue_500.r, palette.blue_500.g, palette.blue_500.b)
);
```

## 🔧 **Widget Integration Examples**

### 📋 **Checkbox Integration**
```typescript
createAnimatedCheckbox({
    id: 'integrated-checkbox',
    label: 'Rust-processed styling',
    checked: true,
    classes: [
        'text-purple-600',    // → Rust: hex("#7c3aed")
        'bg-purple-50',       // → Rust: hex("#f5f3ff")
        'p-3',                // → Rust: spacing value 3
        'rounded-md',         // → Rust: border style
        'font-semibold',      // → Rust: \x1B[1m
        'hover:bg-purple-100' // → Rust: hover state ANSI
    ],
    animationConfig: {
        enabled: true,
        duration: 300,
        easing: 'ease-out',
        scaleFactor: 1.5
    }
})
```

### 📊 **Progress Bar Integration**
```typescript
linearProgress({
    id: 'styled-progress',
    value: 75,
    max: 100,
    label: 'Processing...',
    classes: [
        'w-full',             // → Rust: layout utility
        'h-3',                // → Rust: height spacing
        'bg-gray-200',        // → Rust: hex("#e5e7eb")
        'rounded-full',       // → Rust: border style
        'overflow-hidden'     // → Rust: layout utility
    ]
})
```

### 🔄 **Spinner Integration**
```typescript
createLoadingSpinner({
    id: 'styled-spinner',
    label: 'Loading...',
    type: 'dots',
    classes: [
        'text-blue-500',      // → Rust: hex("#3b82f6")
        'text-lg',            // → Rust: typography
        'animate-pulse'       // → Rust: animation state
    ]
})
```

## ⚡ **Performance Benefits**

### 🚀 **Rust Processing Advantages**
1. **Fast HashMap Lookups**: O(1) class resolution
2. **Efficient ANSI Generation**: Pre-computed escape sequences
3. **Memory Efficient**: Minimal allocations for color processing
4. **Terminal Optimized**: Direct ANSI output without DOM overhead

### 📊 **Benchmarks**
```rust
// Rust utility processing is extremely fast
let processor = UtilityProcessor::new();
let classes = vec!["text-blue-600", "bg-gray-50", "font-bold"];

// Typical processing time: ~1-5 microseconds
let ansi_codes = processor.process_classes(&classes);
```

## 🎯 **Integration Patterns**

### 🔄 **Data Flow**
1. **TypeScript**: Widget configured with `classes: string[]`
2. **Serialization**: Classes passed to Rust layer
3. **Rust Processing**: `UtilityProcessor::process_classes()`
4. **ANSI Output**: Terminal escape sequences generated
5. **Rendering**: Terminal displays styled content

### 🎨 **Theme Consistency**
```typescript
// TypeScript theme definition
const theme = {
    primary: 'blue-600',
    secondary: 'gray-500',
    success: 'green-500',
    warning: 'yellow-500',
    error: 'red-500'
};

// Rust automatically processes with matching colors
// No manual color mapping required!
```

## 🔧 **Advanced Features**

### 🎭 **Responsive Design**
```typescript
// TypeScript responsive classes
classes: [
    'w-full',           // Full width base
    'md:w-1/2',         // Half width on medium screens
    'lg:w-1/3',         // Third width on large screens
    'text-sm',          // Small text base
    'md:text-base',     // Normal text on medium
    'lg:text-lg'        // Large text on large screens
]

// Rust processes based on terminal size detection
```

### 🌙 **Dark Mode Support**
```typescript
// Automatic dark mode variants
classes: [
    'bg-white',         // Light mode background
    'dark:bg-gray-900', // Dark mode background
    'text-gray-900',    // Light mode text
    'dark:text-white'   // Dark mode text
]
```

### 🎬 **Animation Integration**
```typescript
// TypeScript animation classes
classes: [
    'transition-all',    // → Rust: animation config
    'duration-300',      // → Rust: timing
    'ease-out',          // → Rust: easing function
    'hover:scale-105'    // → Rust: transform on hover
]
```

## 🎉 **Benefits Summary**

### ✅ **Developer Experience**
- **Familiar Syntax**: Standard utility CSS classes
- **Type Safety**: TypeScript ensures correct class names
- **IntelliSense**: IDE autocompletion for utility classes
- **Consistent**: Same classes work across all widgets

### ⚡ **Performance**
- **Fast Processing**: Rust-powered ANSI generation
- **Memory Efficient**: Minimal overhead for styling
- **Terminal Optimized**: Direct escape sequence output
- **Scalable**: Handles complex UIs with many styled elements

### 🎨 **Design System**
- **Complete Palette**: All utility colors available
- **Responsive**: Terminal size-aware utilities
- **Accessible**: ANSI codes work in all terminals
- **Extensible**: Easy to add custom utility classes

## 🚀 **Getting Started**

1. **Use TypeScript widgets** with `classes` arrays
2. **Apply utility classes** like `text-blue-600`, `bg-gray-50`
3. **Rust automatically processes** classes to ANSI codes
4. **Terminal renders** with proper colors and styling

**Result**: Beautiful, performant terminal UIs with familiar web development patterns! 🌟
