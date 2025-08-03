# 🎨 Utility CSS Support

All TUI framework widgets are designed to be **fully compatible** with utility-first CSS frameworks like Tailwind CSS. Every widget accepts a `classes` array that allows you to apply any utility classes for complete styling control.

## ✅ **Widget CSS Class Support**

### 📋 **Universal Support**
All widgets support the `classes?: string[]` property in their configuration:

```typescript
// Every widget accepts utility classes
createSimpleCheckbox({
    id: 'my-checkbox',
    label: 'Enable notifications',
    classes: ['text-blue-600', 'font-medium', 'hover:text-blue-800']
})
```

### 🎯 **Supported Widgets**

| Widget | Config Interface | Classes Support | Built-in Classes |
|--------|------------------|-----------------|------------------|
| **Checkbox** | `CheckboxConfig` | ✅ `classes?: string[]` | `checkbox`, `checkbox-checked`, `checkbox-ballot` |
| **Checkbox Group** | `CheckboxGroupConfig` | ✅ `classes?: string[]` | `checkbox-group`, `checkbox-group-vertical` |
| **Progress** | `ProgressConfig` | ✅ `classes?: string[]` | `progress`, `progress-linear`, `progress-indeterminate` |
| **Spinner** | `SpinnerConfig` | ✅ `classes?: string[]` | `spinner`, `spinner-running`, `spinner-paused` |
| **Radio Group** | `RadioGroupConfig` | ✅ `classes?: string[]` | `radio-group`, `radio-horizontal`, `radio-vertical` |
| **Slider** | `SliderConfig` | ✅ `classes?: string[]` | `slider`, `slider-single`, `slider-horizontal` |
| **Switch** | `SwitchConfig` | ✅ `classes?: string[]` | `switch`, `switch-on`, `switch-off` |
| **Text** | `TextConfig` | ✅ `classes?: string[]` | None (pure utility) |
| **Div** | `DivConfig` | ✅ `classes?: string[]` | None (pure utility) |

## 🎨 **Tailwind Utility Categories**

### 🎯 **Layout Classes**
```typescript
// Flexbox
classes: ['flex', 'items-center', 'justify-between', 'flex-wrap']

// Grid
classes: ['grid', 'grid-cols-2', 'md:grid-cols-3', 'gap-4']

// Container
classes: ['container', 'mx-auto', 'max-w-4xl']
```

### 📏 **Spacing Classes**
```typescript
// Padding
classes: ['p-4', 'px-6', 'py-2', 'pt-8']

// Margin
classes: ['m-4', 'mx-auto', 'mb-6', 'mt-2']

// Space Between
classes: ['space-y-4', 'space-x-2']
```

### 🎨 **Color Classes**
```typescript
// Text Colors
classes: ['text-blue-600', 'text-gray-800', 'text-green-500']

// Background Colors
classes: ['bg-white', 'bg-gray-50', 'bg-blue-100']

// Border Colors
classes: ['border-gray-300', 'border-blue-500']
```

### 📝 **Typography Classes**
```typescript
// Font Size
classes: ['text-sm', 'text-lg', 'text-2xl', 'text-4xl']

// Font Weight
classes: ['font-light', 'font-medium', 'font-bold']

// Text Alignment
classes: ['text-left', 'text-center', 'text-right']
```

### 🎭 **Visual Effects**
```typescript
// Shadows
classes: ['shadow-sm', 'shadow-md', 'shadow-lg', 'drop-shadow-lg']

// Borders
classes: ['border', 'border-2', 'rounded-lg', 'rounded-full']

// Opacity
classes: ['opacity-50', 'opacity-75', 'opacity-100']
```

### 🎬 **Animation Classes**
```typescript
// Transitions
classes: ['transition-all', 'duration-300', 'ease-in-out']

// Transforms
classes: ['transform', 'scale-105', 'rotate-45', 'translate-x-2']

// Hover Effects
classes: ['hover:scale-105', 'hover:text-blue-800', 'hover:shadow-lg']
```

### 📱 **Responsive Classes**
```typescript
// Responsive Design
classes: ['w-full', 'md:w-1/2', 'lg:w-1/3', 'xl:w-1/4']

// Responsive Grid
classes: ['grid-cols-1', 'md:grid-cols-2', 'lg:grid-cols-3']

// Responsive Text
classes: ['text-sm', 'md:text-base', 'lg:text-lg']
```

## 🔧 **Implementation Details**

### 📦 **How Classes Are Applied**

1. **Built-in Classes**: Each widget has semantic classes automatically applied
2. **Custom Classes**: Your utility classes are merged with built-in classes
3. **Class Order**: Custom classes are appended, allowing overrides

```typescript
// Internal implementation example
const classes = ['checkbox']; // Built-in semantic class

if (config.checked) {
    classes.push('checkbox-checked'); // State-based class
}

if (config.classes) {
    classes.push(...config.classes); // Your utility classes
}

builder.classes(classes); // Applied to element
```

### 🎯 **Class Merging Strategy**

```typescript
// Example: Checkbox with utility classes
createSimpleCheckbox({
    id: 'styled-checkbox',
    label: 'My checkbox',
    checked: true,
    classes: ['text-blue-600', 'font-medium', 'mb-4']
})

// Resulting classes:
// ['checkbox', 'checkbox-checked', 'checkbox-ballot', 'text-blue-600', 'font-medium', 'mb-4']
```

## 🎨 **Real-World Examples**

### 🏗️ **Layout Example**
```typescript
div({ 
    classes: ['min-h-screen', 'bg-gray-50', 'py-8', 'px-4'] 
})
    .child(
        div({ classes: ['max-w-4xl', 'mx-auto', 'space-y-8'] })
            .child(
                createSimpleCheckbox({
                    id: 'option-1',
                    label: 'Enable feature',
                    classes: [
                        'p-4', 'bg-white', 'rounded-lg', 'shadow-md',
                        'border', 'border-gray-200', 'hover:shadow-lg',
                        'transition-shadow', 'duration-200'
                    ]
                })
            )
    )
```

### 🎨 **Styled Components**
```typescript
// Card-style checkbox
createAnimatedCheckbox({
    id: 'card-checkbox',
    label: 'Premium Feature',
    classes: [
        'bg-gradient-to-r', 'from-blue-500', 'to-purple-600',
        'text-white', 'p-6', 'rounded-xl', 'shadow-xl',
        'transform', 'transition-all', 'duration-300',
        'hover:scale-105', 'hover:shadow-2xl'
    ]
})

// Progress bar with gradient
linearProgress({
    id: 'styled-progress',
    value: 75,
    max: 100,
    classes: [
        'w-full', 'h-4', 'bg-gray-200', 'rounded-full',
        'overflow-hidden', 'shadow-inner',
        'bg-gradient-to-r', 'from-green-400', 'to-blue-500'
    ]
})
```

### 📱 **Responsive Design**
```typescript
createCheckboxGroup({
    id: 'responsive-group',
    label: 'Select options',
    options: [...],
    classes: [
        'grid', 'grid-cols-1', 'md:grid-cols-2', 'lg:grid-cols-3',
        'gap-4', 'p-6', 'bg-white', 'rounded-lg', 'shadow-md'
    ]
})
```

## ✨ **Best Practices**

### 🎯 **Recommended Patterns**

1. **Consistent Spacing**: Use Tailwind's spacing scale (`p-4`, `mb-6`, `space-y-4`)
2. **Color Harmony**: Stick to a consistent color palette (`blue-600`, `gray-800`)
3. **Responsive Design**: Always consider mobile-first responsive classes
4. **Hover States**: Add interactive feedback with hover utilities
5. **Transitions**: Use smooth transitions for better UX

### 🚫 **Avoid These Patterns**

```typescript
// ❌ Don't override semantic classes
classes: ['checkbox', 'my-custom-checkbox'] // Conflicts with built-in

// ✅ Do add complementary classes
classes: ['text-blue-600', 'font-medium', 'hover:text-blue-800']
```

## 🔗 **Integration with CSS Frameworks**

### 🌊 **Utility CSS**
Perfect compatibility - all utility classes work as expected.

### 🎨 **Custom Utility Systems**
Any utility-first CSS framework will work:
- **Tachyons**: `classes: ['f4', 'fw6', 'blue']`
- **Basscss**: `classes: ['h3', 'bold', 'blue']`
- **Custom**: `classes: ['u-text-large', 'u-color-primary']`

## 🎉 **Summary**

✅ **Complete Utility Support**: All widgets accept `classes?: string[]`  
✅ **Semantic Classes**: Built-in classes for widget states and types  
✅ **Class Merging**: Custom classes merge with built-in classes  
✅ **No Conflicts**: Utility classes complement semantic classes  
✅ **Full Control**: Style every aspect with utility classes  

**Result**: You have complete styling control over every TUI framework widget using familiar utility-first CSS patterns! 🚀
