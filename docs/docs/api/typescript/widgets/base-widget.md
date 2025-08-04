# BaseWidget

The BaseWidget provides the foundational implementation for all widgets in the reactive-tui framework. It offers common functionality including lifecycle management, configuration updates, validation, state management, and element rendering with consistent behavior patterns across all widget types.

## Basic Usage

```typescript
import { BaseWidget, createWidgetBuilder } from 'reactive-tui';

// BaseWidget is typically extended by other widgets
class CustomWidget extends BaseWidget {
  protected createElement(): ElementBuilder {
    return this.createContainer()
      .child(div().text(this._config.content || 'Custom Widget'));
  }
}

// Using widget builder helper
const customWidgetBuilder = createWidgetBuilder(
  'custom',
  CustomWidget,
  {
    id: { type: 'string', required: true },
    type: { type: 'string', required: true },
    content: { type: 'string', required: false }
  }
);
```

## Configuration

### BaseWidgetConfig Interface

```typescript
interface BaseWidgetConfig {
  id: string;                       // Unique widget identifier
  type: string;                     // Widget type name
  visible?: boolean;                // Widget visibility (default: true)
  disabled?: boolean;               // Widget disabled state (default: false)
  focusable?: boolean;              // Can receive focus (default: false)
  tabIndex?: number;                // Tab order index
  classes?: string[];               // Additional CSS classes
  attributes?: Record<string, string>; // HTML attributes
}
```

### WidgetInstance Interface

```typescript
interface WidgetInstance {
  id: string;                       // Widget identifier
  type: string;                     // Widget type
  config: BaseWidgetConfig;         // Current configuration
  destroyed: boolean;               // Destruction state
  
  render(): Element;                // Render widget to Element
  update(updates: Partial<BaseWidgetConfig>): void; // Update configuration
  destroy(): void;                  // Clean up resources
  validate(): boolean;              // Validate widget state
  isVisible(): boolean;             // Check visibility
  isEnabled(): boolean;             // Check enabled state
  isFocusable(): boolean;           // Check focusability
}
```

## Core Features

### Widget Lifecycle Management

```typescript
class ExampleWidget extends BaseWidget {
  constructor(config: BaseWidgetConfig) {
    super(config);
    console.log(`Widget ${this.id} created`);
  }
  
  protected createElement(): ElementBuilder {
    return this.createContainer()
      .child(div().text(`Widget: ${this.id}`));
  }
  
  protected onConfigUpdate(oldConfig: BaseWidgetConfig, newConfig: BaseWidgetConfig): void {
    console.log(`Widget ${this.id} configuration updated`);
    
    // Handle specific configuration changes
    if (oldConfig.disabled !== newConfig.disabled) {
      console.log(`Disabled state changed: ${newConfig.disabled}`);
    }
  }
  
  protected onDestroy(): void {
    console.log(`Widget ${this.id} being destroyed`);
    // Clean up resources, event listeners, etc.
  }
  
  protected validateSpecific(): boolean {
    // Widget-specific validation logic
    return this._config.id.length > 0;
  }
}

// Widget lifecycle
const widget = new ExampleWidget({ id: 'example', type: 'example' });

// Render widget
const element = widget.render();

// Update configuration
widget.update({ disabled: true, classes: ['updated'] });

// Validate state
const isValid = widget.validate();

// Destroy widget
widget.destroy();
```

### State Management Integration

```typescript
// Implement StatefulWidget interface for state management
interface CustomWidgetState {
  count: number;
  message: string;
  items: string[];
}

class StatefulCustomWidget extends BaseWidget implements StatefulWidget<CustomWidgetState> {
  private state: CustomWidgetState = {
    count: 0,
    message: '',
    items: []
  };
  
  protected createElement(): ElementBuilder {
    return this.createContainer()
      .child(div().text(`Count: ${this.state.count}`))
      .child(div().text(`Message: ${this.state.message}`))
      .child(div().text(`Items: ${this.state.items.length}`));
  }
  
  getState(): CustomWidgetState {
    return { ...this.state };
  }
  
  setState(state: Partial<CustomWidgetState>): void {
    const oldState = { ...this.state };
    this.state = { ...this.state, ...state };
    
    // Force re-render when state changes
    this._element = null;
    
    console.log('State updated:', { oldState, newState: this.state });
  }
  
  resetState(): void {
    this.setState({
      count: 0,
      message: '',
      items: []
    });
  }
  
  // Custom methods using state
  increment(): void {
    this.setState({ count: this.state.count + 1 });
  }
  
  setMessage(message: string): void {
    this.setState({ message });
  }
  
  addItem(item: string): void {
    this.setState({ items: [...this.state.items, item] });
  }
}

// Usage
const statefulWidget = new StatefulCustomWidget({ id: 'stateful', type: 'stateful' });

statefulWidget.increment();
statefulWidget.setMessage('Hello World');
statefulWidget.addItem('First Item');

const currentState = statefulWidget.getState();
console.log('Current state:', currentState);
```

### Performance Monitoring

```typescript
// Implement PerformanceMonitoring interface
class PerformanceWidget extends BaseWidget implements PerformanceMonitoring {
  renderTime: number = 0;
  updateCount: number = 0;
  lastRenderAt: number = 0;
  
  render(): Element {
    const startTime = performance.now();
    
    const element = super.render();
    
    this.renderTime = performance.now() - startTime;
    this.lastRenderAt = Date.now();
    
    // Log slow renders
    if (this.renderTime > 10) {
      console.warn(`Slow render detected for ${this.id}: ${this.renderTime.toFixed(2)}ms`);
    }
    
    return element;
  }
  
  update(updates: Partial<BaseWidgetConfig>): void {
    this.updateCount++;
    super.update(updates);
    
    console.log(`Widget ${this.id} updated ${this.updateCount} times`);
  }
  
  protected createElement(): ElementBuilder {
    return this.createContainer()
      .child(div().text(`Render time: ${this.renderTime.toFixed(2)}ms`))
      .child(div().text(`Update count: ${this.updateCount}`))
      .child(div().text(`Last render: ${new Date(this.lastRenderAt).toLocaleTimeString()}`));
  }
  
  getPerformanceStats() {
    return {
      renderTime: this.renderTime,
      updateCount: this.updateCount,
      lastRenderAt: this.lastRenderAt,
      averageRenderTime: this.renderTime // Could track history for true average
    };
  }
}
```

### Widget Validation

```typescript
class ValidatedWidget extends BaseWidget {
  protected validateSpecific(): boolean {
    // Validate required configuration
    if (!this._config.id || this._config.id.length < 3) {
      console.error(`Widget ID must be at least 3 characters: ${this._config.id}`);
      return false;
    }
    
    // Validate classes
    if (this._config.classes) {
      const invalidClasses = this._config.classes.filter(cls => 
        typeof cls !== 'string' || cls.length === 0
      );
      if (invalidClasses.length > 0) {
        console.error(`Invalid CSS classes:`, invalidClasses);
        return false;
      }
    }
    
    // Validate tab index
    if (this._config.tabIndex !== undefined) {
      if (!Number.isInteger(this._config.tabIndex) || this._config.tabIndex < -1) {
        console.error(`Invalid tabIndex: ${this._config.tabIndex}`);
        return false;
      }
    }
    
    return true;
  }
  
  protected createElement(): ElementBuilder {
    // Only render if valid
    if (!this.validate()) {
      return div().text('Invalid widget configuration');
    }
    
    return this.createContainer()
      .child(div().text('Valid widget'));
  }
}
```

## Advanced Usage

### Custom Widget Development

```typescript
// Define custom widget configuration
interface ButtonWidgetConfig extends BaseWidgetConfig {
  text: string;
  variant: 'primary' | 'secondary' | 'danger';
  size: 'small' | 'medium' | 'large';
  onClick?: () => void;
}

// Create custom widget extending BaseWidget
class ButtonWidget extends BaseWidget {
  private _config: ButtonWidgetConfig;
  
  constructor(config: ButtonWidgetConfig) {
    super(config);
    this._config = config;
  }
  
  protected createElement(): ElementBuilder {
    const button = this.createContainer()
      .tag('button')
      .text(this._config.text)
      .addClass(`btn-${this._config.variant}`)
      .addClass(`btn-${this._config.size}`);
    
    // Add click handler
    if (this._config.onClick && this.isEnabled()) {
      button.on('click', this._config.onClick);
    }
    
    return button;
  }
  
  protected validateSpecific(): boolean {
    if (!this._config.text || this._config.text.trim().length === 0) {
      console.error('Button text is required');
      return false;
    }
    
    const validVariants = ['primary', 'secondary', 'danger'];
    if (!validVariants.includes(this._config.variant)) {
      console.error(`Invalid button variant: ${this._config.variant}`);
      return false;
    }
    
    const validSizes = ['small', 'medium', 'large'];
    if (!validSizes.includes(this._config.size)) {
      console.error(`Invalid button size: ${this._config.size}`);
      return false;
    }
    
    return true;
  }
  
  protected onConfigUpdate(oldConfig: BaseWidgetConfig, newConfig: BaseWidgetConfig): void {
    const oldButton = oldConfig as ButtonWidgetConfig;
    const newButton = newConfig as ButtonWidgetConfig;
    
    // Force re-render if text, variant, or size changed
    if (oldButton.text !== newButton.text ||
        oldButton.variant !== newButton.variant ||
        oldButton.size !== newButton.size) {
      this._element = null;
    }
  }
  
  // Custom button-specific methods
  setText(text: string): void {
    this.update({ text } as Partial<ButtonWidgetConfig>);
  }
  
  setVariant(variant: ButtonWidgetConfig['variant']): void {
    this.update({ variant } as Partial<ButtonWidgetConfig>);
  }
  
  click(): void {
    if (this.isEnabled() && this._config.onClick) {
      this._config.onClick();
    }
  }
}

// Create widget builder for registration
const buttonWidget = createWidgetBuilder(
  'button',
  ButtonWidget,
  {
    id: { type: 'string', required: true },
    type: { type: 'string', required: true },
    text: { type: 'string', required: true },
    variant: { type: 'string', required: true, enum: ['primary', 'secondary', 'danger'] },
    size: { type: 'string', required: true, enum: ['small', 'medium', 'large'] },
    onClick: { type: 'function', required: false }
  }
);

// Usage
const button = new ButtonWidget({
  id: 'my-button',
  type: 'button',
  text: 'Click Me',
  variant: 'primary',
  size: 'medium',
  onClick: () => console.log('Button clicked!')
});

button.render(); // Returns DOM element
button.setText('Updated Text');
button.setVariant('danger');
button.click(); // Programmatically trigger click
```

### Widget Composition

```typescript
// Create composite widget using BaseWidget
class FormWidget extends BaseWidget {
  private fields: BaseWidget[] = [];
  
  protected createElement(): ElementBuilder {
    const form = this.createContainer()
      .tag('form')
      .addClass('form-widget');
    
    // Add all field widgets
    this.fields.forEach(field => {
      form.child(field.render());
    });
    
    return form;
  }
  
  addField(field: BaseWidget): void {
    this.fields.push(field);
    this._element = null; // Force re-render
  }
  
  removeField(fieldId: string): void {
    const index = this.fields.findIndex(field => field.id === fieldId);
    if (index >= 0) {
      const field = this.fields[index];
      field.destroy();
      this.fields.splice(index, 1);
      this._element = null; // Force re-render
    }
  }
  
  getField(fieldId: string): BaseWidget | null {
    return this.fields.find(field => field.id === fieldId) || null;
  }
  
  validateForm(): boolean {
    // Validate all fields
    return this.fields.every(field => field.validate());
  }
  
  protected onDestroy(): void {
    // Clean up all field widgets
    this.fields.forEach(field => field.destroy());
    this.fields = [];
  }
}

// Usage
const form = new FormWidget({ id: 'user-form', type: 'form' });

const nameField = new ButtonWidget({
  id: 'name-field',
  type: 'input',
  text: 'Name',
  variant: 'primary',
  size: 'medium'
});

const emailField = new ButtonWidget({
  id: 'email-field',
  type: 'input',
  text: 'Email',
  variant: 'secondary',
  size: 'medium'
});

form.addField(nameField);
form.addField(emailField);

const isValid = form.validateForm();
console.log('Form is valid:', isValid);
```

### Widget Factory Integration

```typescript
// Register widget with factory system
import { widgetRegistry } from 'reactive-tui';

// Define widget schema
const customWidgetSchema = {
  id: { type: 'string', required: true },
  type: { type: 'string', required: true },
  title: { type: 'string', required: true },
  description: { type: 'string', required: false },
  level: { type: 'number', required: false, default: 1 }
};

// Create widget class
class CustomWidget extends BaseWidget {
  protected createElement(): ElementBuilder {
    const config = this._config as any;
    
    return this.createContainer()
      .child(
        div()
          .addClass(`heading-level-${config.level || 1}`)
          .text(config.title)
      )
      .child(
        config.description 
          ? div().addClass('description').text(config.description)
          : null
      );
  }
}

// Register with factory
widgetRegistry.register(
  'custom',
  (config) => new CustomWidget(config),
  customWidgetSchema
);

// Create widget through factory
const widget = widgetRegistry.create('custom', {
  id: 'my-custom',
  type: 'custom',
  title: 'Custom Widget',
  description: 'This is a custom widget',
  level: 2
});
```

## Utility Functions

### Widget Builder Helper

```typescript
// Helper function for creating widget builders
function createWidgetBuilder<TConfig extends BaseWidgetConfig>(
  type: string,
  WidgetClass: new (config: TConfig) => BaseWidget,
  schema: WidgetSchema
) {
  // Register with factory
  widgetRegistry.register(
    type,
    (config: TConfig) => new WidgetClass(config),
    schema
  );

  // Return builder function
  return (config: TConfig): BaseWidget => new WidgetClass(config);
}

// Usage
const myWidget = createWidgetBuilder(
  'my-widget',
  MyWidget,
  {
    id: { type: 'string', required: true },
    type: { type: 'string', required: true },
    customProp: { type: 'string', required: false }
  }
);
```

### Performance Monitoring Utilities

```typescript
// Utility class for widget performance monitoring
class WidgetPerformanceMonitor {
  private widgets: Map<string, PerformanceMonitoring> = new Map();
  
  register(widget: BaseWidget & PerformanceMonitoring): void {
    this.widgets.set(widget.id, widget);
  }
  
  unregister(widgetId: string): void {
    this.widgets.delete(widgetId);
  }
  
  getStats(widgetId?: string) {
    if (widgetId) {
      const widget = this.widgets.get(widgetId);
      return widget ? {
        id: widgetId,
        renderTime: widget.renderTime,
        updateCount: widget.updateCount,
        lastRenderAt: widget.lastRenderAt
      } : null;
    }
    
    // Return stats for all widgets
    const stats: any[] = [];
    this.widgets.forEach((widget, id) => {
      stats.push({
        id,
        renderTime: widget.renderTime,
        updateCount: widget.updateCount,
        lastRenderAt: widget.lastRenderAt
      });
    });
    
    return stats;
  }
  
  getSlowWidgets(threshold: number = 10): any[] {
    const stats = this.getStats() as any[];
    return stats.filter(stat => stat.renderTime > threshold);
  }
  
  getMostUpdatedWidgets(limit: number = 5): any[] {
    const stats = this.getStats() as any[];
    return stats
      .sort((a, b) => b.updateCount - a.updateCount)
      .slice(0, limit);
  }
}

// Usage
const performanceMonitor = new WidgetPerformanceMonitor();

// Register widgets for monitoring
const widget1 = new PerformanceWidget({ id: 'widget1', type: 'performance' });
const widget2 = new PerformanceWidget({ id: 'widget2', type: 'performance' });

performanceMonitor.register(widget1);
performanceMonitor.register(widget2);

// Monitor performance
setInterval(() => {
  const slowWidgets = performanceMonitor.getSlowWidgets(5);
  if (slowWidgets.length > 0) {
    console.warn('Slow widgets detected:', slowWidgets);
  }
  
  const mostUpdated = performanceMonitor.getMostUpdatedWidgets(3);
  console.log('Most frequently updated widgets:', mostUpdated);
}, 10000);
```

## Best Practices

1. **Widget Development**
   - Always extend BaseWidget for new widgets
   - Implement proper validation in `validateSpecific()`
   - Handle configuration updates in `onConfigUpdate()`
   - Clean up resources in `onDestroy()`

2. **State Management**
   - Implement StatefulWidget interface for complex state
   - Force re-render by setting `_element = null` when state changes
   - Use immutable state updates

3. **Performance**
   - Implement PerformanceMonitoring for complex widgets
   - Cache rendered elements when possible
   - Override `shouldRerender()` for optimization

4. **Configuration**
   - Validate configuration thoroughly
   - Provide sensible defaults
   - Use TypeScript interfaces for type safety

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const widgetContainer = new ElementBuilderImpl('div')
  .class('widget-container')
  .child(
    new CustomWidget({
      id: 'integrated-widget',
      type: 'custom',
      classes: ['integrated']
    }).render()
  )
  .build();
```

The BaseWidget provides the essential foundation for all widgets in reactive-tui, offering consistent lifecycle management, configuration handling, validation, and rendering patterns that ensure reliable and maintainable widget development.