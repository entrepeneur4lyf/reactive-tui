# Getting Started

Learn how to create your first Reactive TUI application using the actual tui-bun API.

## Installation

```bash
npm install reactive-tui
```

## Your First App

```typescript
import { createApp, div, text } from 'reactive-tui';

const app = createApp({
  stylesheet: './styles.css', // Optional CSS file
  component: () => 
    div({ class: 'container' })
      .child(text('🚀 Hello, CSS-styled TUI!'))
});

await app.run();
```

## Adding Interactivity

```typescript
import { createApp, div, button, text } from 'reactive-tui';

const app = createApp({
  component: () => 
    div({ class: 'app' })
      .child(
        text('Welcome to Reactive TUI!')
      )
      .child(
        button({ class: 'primary-button' })
          .child(text('Click me!'))
      )
});

await app.run();
```

## Adding CSS Styles

Create a `styles.css` file:

```css
.app {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 1rem;
  padding: 2rem;
  background: #1e1e1e;
  color: #ffffff;
}

.primary-button {
  background: #007acc;
  color: white;
  padding: 1rem 2rem;
  border-radius: 4px;
  border: 1px solid #005a9f;
  font-weight: bold;
}

.primary-button:hover {
  background: #005a9f;
}
```

## Advanced Example with Multiple Components

```typescript
import { 
  createApp, div, text, button, input, 
  checkboxWidget, progressBar, grid 
} from 'reactive-tui';

const app = createApp({
  stylesheet: './styles.css',
  component: () =>
    div({ class: 'app-container' })
      .child(
        // Header
        div({ class: 'header' })
          .child(text('My Terminal Application'))
      )
      .child(
        // Main content with grid layout
        grid({ class: 'grid-cols-2 gap-4' })
          .child(
            div({ class: 'panel' })
              .child(text('Form Controls'))
              .child(input({ placeholder: 'Enter text...' }))
              .child(checkboxWidget({ label: 'Enable feature' }))
              .child(button({ class: 'primary-button' })
                .child(text('Submit'))
              )
          )
          .child(
            div({ class: 'panel' })
              .child(text('Progress'))
              .child(progressBar({ value: 75, max: 100 }))
          )
      )
      .child(
        // Footer
        div({ class: 'footer' })
          .child(text('Press q to quit'))
      )
});

await app.run();
```

## Component Builder Pattern

Reactive TUI uses a fluent builder pattern for creating components:

```typescript
// Basic structure
const element = div({ class: 'container', id: 'main' })
  .child(text('Child 1'))
  .child(text('Child 2'))
  .child(
    div({ class: 'nested' })
      .child(text('Nested content'))
  );

// Attributes and properties
const input = input({
  type: 'password',
  placeholder: 'Enter password',
  value: ''
});

// CSS classes
const styledDiv = div()
  .addClass('primary')
  .addClass('large')
  .removeClass('disabled');
```

## Quick Start Function

For simple applications, use the `runApp` helper:

```typescript
import { runApp, div, text } from 'reactive-tui';

await runApp('./styles.css', () => 
  div({ class: 'app' })
    .child(text('Quick start application!'))
);
```

## What's Next?

- **[Core Concepts](../core-concepts)**: Understand the architecture and component system
- **[CSS Styling](./styling)**: Learn about the CSS engine and responsive design
- **[Widget Library](./widgets)**: Explore all available components
- **[Layout System](./layout)**: Master flexbox and grid layouts