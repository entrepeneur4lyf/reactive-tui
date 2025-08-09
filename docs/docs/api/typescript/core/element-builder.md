---
sidebar_position: 2
---

# ElementBuilder

The `ElementBuilder` interface provides a fluent, chainable API for constructing UI elements with type safety and comprehensive configuration options.

## Interface Definition

```typescript
interface ElementBuilder {
  id(id: string): ElementBuilder
  class(className: string): ElementBuilder
  classes(classNames: string[]): ElementBuilder
  attr(name: string, value: string): ElementBuilder
  content(content: string): ElementBuilder
  child(child: Element | ElementBuilder): ElementBuilder
  children(children: (Element | ElementBuilder)[]): ElementBuilder
  focusable(focusable: boolean): ElementBuilder
  tab_index(index: number): ElementBuilder
  bind_key(key: KeyCombination, action: ElementAction): ElementBuilder
  bind_char(c: string, action: ElementAction): ElementBuilder
  bind_enter(): ElementBuilder
  bind_space(): ElementBuilder
  modal(is_modal: boolean): ElementBuilder
  build(): Element
}
```

## Basic Usage

### Creating Elements

```typescript
import { div, text, button } from 'reactive-tui-ts'

// Simple element
const container = div()
  .id('main-container')
  .class('container')
  .build()

// Element with content
const title = text('Welcome to My App')
  .class('title')
  .build()

// Complex element
const card = div()
  .id('user-card')
  .classes(['card', 'user-card', 'elevated'])
  .attr('data-user-id', '123')
  .child(text('John Doe', { class: 'name' }))
  .child(text('john@example.com', { class: 'email' }))
  .build()
```

### Method Chaining

All builder methods return the builder instance, enabling fluent chaining:

```typescript
const complexElement = div()
  .id('complex-element')                    // Set unique ID
  .class('primary-container')               // Add single class
  .classes(['responsive', 'themed'])        // Add multiple classes
  .attr('role', 'main')                     // Set ARIA role
  .attr('data-testid', 'main-container')    // Add test identifier
  .content('Container content')             // Set text content
  .child(createHeaderElement())             // Add child element
  .child(createContentElement())            // Add another child
  .build()                                  // Convert to Element
```

## Builder Methods

### `id(id: string): ElementBuilder`

Sets the unique identifier for the element.

```typescript
const element = div()
  .id('unique-element-id')
  .build()

// Result: <div id="unique-element-id"></div>
```

**Best Practices:**
- Use descriptive, kebab-case IDs
- Ensure IDs are unique within the application
- Include context in ID names

```typescript
// ✅ Good - descriptive IDs
const userProfileCard = div().id('user-profile-card')
const submitButton = button().id('form-submit-button')

// ❌ Avoid - generic IDs
const element1 = div().id('div1')
const btn = button().id('btn')
```

### `class(className: string): ElementBuilder`

Adds a single CSS class to the element.

```typescript
const element = div()
  .class('container')
  .class('responsive')  // Can be chained
  .build()

// Result: <div class="container responsive"></div>
```

### `classes(classNames: string[]): ElementBuilder`

Adds multiple CSS classes at once.

```typescript
const element = div()
  .classes(['container', 'responsive', 'themed'])
  .build()

// Result: <div class="container responsive themed"></div>
```

**Class Naming Conventions:**
```typescript
// ✅ Good - semantic class names
const card = div()
  .classes(['card', 'user-card', 'elevated', 'interactive'])

// ✅ Good - utility classes
const button = div()
  .classes(['btn', 'btn-primary', 'btn-lg', 'rounded'])

// ❌ Avoid - presentation-only names
const element = div()
  .classes(['red-background', 'big-text', 'left-aligned'])
```

### `attr(name: string, value: string): ElementBuilder`

Sets custom attributes on the element.

```typescript
const element = div()
  .attr('role', 'button')
  .attr('aria-label', 'Close dialog')
  .attr('data-testid', 'close-button')
  .attr('tabindex', '0')
  .build()

// Result: <div role="button" aria-label="Close dialog" data-testid="close-button" tabindex="0"></div>
```

**Common Attribute Patterns:**
```typescript
// Accessibility attributes
const accessibleElement = div()
  .attr('role', 'button')
  .attr('aria-label', 'Submit form')
  .attr('aria-describedby', 'submit-help')
  .attr('tabindex', '0')

// Data attributes for testing
const testableElement = div()
  .attr('data-testid', 'user-profile')
  .attr('data-user-id', '123')
  .attr('data-component', 'UserCard')

// Custom attributes
const customElement = div()
  .attr('draggable', 'true')
  .attr('contenteditable', 'false')
  .attr('spellcheck', 'false')
```

### `content(content: string): ElementBuilder`

Sets the text content of the element.

```typescript
const element = div()
  .content('This is the element content')
  .build()

// Result: <div>This is the element content</div>
```

**Note:** Setting content will replace any existing child elements.

```typescript
// ⚠️ Warning - content replaces children
const element = div()
  .child(text('Child element'))
  .content('New content')  // This replaces the child
  .build()

// Result: <div>New content</div> (child is gone)
```

### `child(child: Element | ElementBuilder): ElementBuilder`

Adds a child element to the current element.

```typescript
const parent = div()
  .child(text('First child'))
  .child(text('Second child'))
  .child(
    div()
      .class('nested')
      .child(text('Nested content'))
      .build()
  )
  .build()
```

**Child Types:**
```typescript
// Element (already built)
const builtElement = text('Built element').build()
const parent1 = div().child(builtElement).build()

// ElementBuilder (will be built automatically)
const parent2 = div()
  .child(text('Builder element'))  // Auto-built
  .build()

// Mixed children
const parent3 = div()
  .child(builtElement)                    // Pre-built Element
  .child(text('Auto-built'))              // ElementBuilder
  .child(button({ id: 'btn', text: 'Click' }))  // Widget
  .build()
```

### `build(): Element`

Converts the builder to an immutable `Element` instance.

```typescript
const element: Element = div()
  .id('my-element')
  .class('container')
  .child(text('Content'))
  .build()  // Returns Element
```

**Important:** Always call `build()` to finalize the element.

```typescript
// ✅ Good - properly built
const element = div().class('container').build()

// ❌ Error - missing build()
const element = div().class('container')  // Still a builder!
```

## Advanced Patterns

### Conditional Building

Use JavaScript conditionals to build dynamic elements:

```typescript
function createUserCard(user: User, isAdmin: boolean) {
  const card = div()
    .id(`user-${user.id}`)
    .classes(['card', 'user-card'])
    .child(text(user.name, { class: 'name' }))
    .child(text(user.email, { class: 'email' }))

  // Conditionally add admin badge
  if (isAdmin) {
    card.child(
      div()
        .class('admin-badge')
        .content('Admin')
        .build()
    )
  }

  // Conditionally add actions
  if (user.canEdit) {
    card.child(createEditButton(user.id))
  }

  return card.build()
}
```

### Loop Building

Build repeated elements with loops:

```typescript
function createNavigationMenu(items: MenuItem[]) {
  const menu = div()
    .id('navigation-menu')
    .class('nav-menu')

  items.forEach((item, index) => {
    menu.child(
      div()
        .id(`nav-item-${index}`)
        .classes(['nav-item', item.active ? 'active' : ''])
        .child(text(item.label))
        .attr('data-route', item.route)
        .build()
    )
  })

  return menu.build()
}
```

### Builder Composition

Compose complex elements from smaller builders:

```typescript
function createArticleCard(article: Article) {
  return div()
    .id(`article-${article.id}`)
    .class('article-card')
    .child(createArticleHeader(article))
    .child(createArticleBody(article))
    .child(createArticleFooter(article))
    .build()
}

function createArticleHeader(article: Article) {
  return div()
    .class('article-header')
    .child(text(article.title, { class: 'article-title' }))
    .child(text(article.author, { class: 'article-author' }))
    .child(text(formatDate(article.date), { class: 'article-date' }))
    .build()
}

function createArticleBody(article: Article) {
  return div()
    .class('article-body')
    .content(article.excerpt)
    .build()
}

function createArticleFooter(article: Article) {
  return div()
    .class('article-footer')
    .child(createReadMoreButton(article.id))
    .child(createShareButton(article.id))
    .build()
}
```

### Builder Factory Pattern

Create reusable builder factories:

```typescript
class ElementFactory {
  static card(id: string) {
    return div()
      .id(id)
      .classes(['card', 'elevated'])
      .attr('role', 'article')
  }

  static button(id: string, text: string, variant: 'primary' | 'secondary' = 'primary') {
    return button()
      .id(id)
      .classes(['btn', `btn-${variant}`])
      .content(text)
      .attr('type', 'button')
  }

  static input(id: string, type: string = 'text') {
    return input()
      .id(id)
      .classes(['form-input'])
      .attr('type', type)
      .attr('autocomplete', 'off')
  }
}

// Usage
const userCard = ElementFactory.card('user-123')
  .child(text('User Name'))
  .build()

const submitBtn = ElementFactory.button('submit', 'Submit Form', 'primary')
  .attr('form', 'user-form')
  .build()
```

### Builder Validation

Add validation to builders:

```typescript
class ValidatedBuilder {
  private builder: ElementBuilder
  private errors: string[] = []

  constructor(builder: ElementBuilder) {
    this.builder = builder
  }

  validateId(id: string): this {
    if (!id || id.length === 0) {
      this.errors.push('ID is required')
    } else if (!/^[a-zA-Z][a-zA-Z0-9-_]*$/.test(id)) {
      this.errors.push('ID must start with a letter and contain only letters, numbers, hyphens, and underscores')
    }
    return this
  }

  validateClasses(classes: string[]): this {
    classes.forEach(className => {
      if (!/^[a-zA-Z][a-zA-Z0-9-_]*$/.test(className)) {
        this.errors.push(`Invalid class name: ${className}`)
      }
    })
    return this
  }

  build(): Element {
    if (this.errors.length > 0) {
      throw new Error(`Builder validation failed: ${this.errors.join(', ')}`)
    }
    return this.builder.build()
  }
}

// Usage
const validatedElement = new ValidatedBuilder(div())
  .validateId('valid-id')
  .validateClasses(['valid-class', 'another-class'])
  .build()
```

## Performance Considerations

### Efficient Building

```typescript
// ✅ Good - build once at the end
const element = div()
  .id('container')
  .class('main')
  .child(text('Content 1'))
  .child(text('Content 2'))
  .child(text('Content 3'))
  .build()  // Single build call

// ❌ Avoid - multiple builds
const container = div().id('container').build()
const withClass = div().id('container').class('main').build()
const final = div().id('container').class('main').child(text('Content')).build()
```

### Memory Management

```typescript
// ✅ Good - reuse builders when possible
function createCards(items: Item[]) {
  const container = div().class('card-container')
  
  items.forEach(item => {
    container.child(createCard(item))
  })
  
  return container.build()
}

function createCard(item: Item) {
  // Create fresh builder for each card
  return div()
    .id(`card-${item.id}`)
    .class('card')
    .child(text(item.title))
    .build()
}
```

## TypeScript Integration

### Type-Safe Builders

```typescript
interface TypedElementBuilder<T extends Element> extends ElementBuilder {
  build(): T
}

interface ButtonElement extends Element {
  type: 'button'
  variant: 'filled' | 'outlined' | 'ghost'
  disabled: boolean
}

class TypedButtonBuilder implements TypedElementBuilder<ButtonElement> {
  private config: Partial<ButtonElement> = { type: 'button' }

  variant(variant: 'filled' | 'outlined' | 'ghost'): this {
    this.config.variant = variant
    return this
  }

  disabled(disabled: boolean = true): this {
    this.config.disabled = disabled
    return this
  }

  build(): ButtonElement {
    return {
      ...this.config,
      tag: 'button',
      id: this.config.id || '',
      classes: this.config.classes || [],
      attributes: this.config.attributes || {},
      content: this.config.content || null,
      children: this.config.children || [],
      focusable: true,
      focused: false
    } as ButtonElement
  }

  // Implement other ElementBuilder methods...
  id(id: string): this { this.config.id = id; return this }
  class(className: string): this { /* implementation */ return this }
  // ... etc
}
```

## Best Practices

### 1. Use Descriptive IDs and Classes

```typescript
// ✅ Good - semantic naming
const userProfileCard = div()
  .id('user-profile-card')
  .classes(['card', 'user-profile', 'elevated'])

// ❌ Avoid - generic naming
const element = div()
  .id('div1')
  .classes(['box', 'thing'])
```

### 2. Build Elements Once

```typescript
// ✅ Good - single build call
const element = div()
  .id('container')
  .class('main')
  .child(text('Content'))
  .build()

// ❌ Avoid - multiple builds
const step1 = div().id('container').build()
const step2 = div().id('container').class('main').build()
```

### 3. Use Composition for Complex Elements

```typescript
// ✅ Good - composed from smaller functions
function createUserCard(user: User) {
  return div()
    .id(`user-${user.id}`)
    .class('user-card')
    .child(createUserHeader(user))
    .child(createUserBody(user))
    .child(createUserActions(user))
    .build()
}
```

### 4. Validate Critical Properties

```typescript
// ✅ Good - validate required properties
function createElement(id: string, className: string) {
  if (!id) throw new Error('ID is required')
  if (!className) throw new Error('Class name is required')
  
  return div()
    .id(id)
    .class(className)
    .build()
}
```

## Related APIs

- Elements - Element interface and properties (coming soon)
- Components - Component-based architecture (coming soon)
- **[Widget Factory](./widget-factory)** - Widget creation system
- Styling - CSS styling for elements (coming soon)

## Examples

- Element Building - Basic element creation (coming soon)
- Complex Layouts - Advanced element composition (coming soon)
- Dynamic Content - Conditional and loop building (coming soon)
