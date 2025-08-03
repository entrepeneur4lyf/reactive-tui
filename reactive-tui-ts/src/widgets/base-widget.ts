/**
 * Base Widget Implementation
 * 
 * Provides common functionality for all widgets created through the factory pattern
 */

import type { Element } from '../generated-types';
import type { ElementBuilder } from '../types';
import { div } from '../components';
import type { BaseWidgetConfig, WidgetInstance } from '../widget-factory';

/**
 * Abstract base class for all widgets
 */
export abstract class BaseWidget implements WidgetInstance {
  protected _config: BaseWidgetConfig;
  protected _element: Element | null = null;
  protected _destroyed = false;

  constructor(config: BaseWidgetConfig) {
    this._config = { ...config };
  }

  get id(): string {
    return this._config.id;
  }

  get type(): string {
    return this._config.type;
  }

  get config(): BaseWidgetConfig {
    return { ...this._config };
  }

  get destroyed(): boolean {
    return this._destroyed;
  }

  /**
   * Render the widget to an Element
   */
  render(): Element {
    if (this._destroyed) {
      throw new Error(`Cannot render destroyed widget: ${this.id}`);
    }

    if (!this._element || this.shouldRerender()) {
      this._element = this.createElement().build();
    }

    return this._element;
  }

  /**
   * Update widget configuration
   */
  update(updates: Partial<BaseWidgetConfig>): void {
    if (this._destroyed) {
      throw new Error(`Cannot update destroyed widget: ${this.id}`);
    }

    const oldConfig = { ...this._config };
    this._config = { ...this._config, ...updates };
    
    // Clear cached element to force re-render
    this._element = null;
    
    this.onConfigUpdate(oldConfig, this._config);
  }

  /**
   * Destroy the widget and clean up resources
   */
  destroy(): void {
    if (this._destroyed) return;

    this.onDestroy();
    this._destroyed = true;
    this._element = null;
  }

  /**
   * Validate widget state
   */
  validate(): boolean {
    if (this._destroyed) return false;
    if (!this._config.id || typeof this._config.id !== 'string') return false;
    if (!this._config.type || typeof this._config.type !== 'string') return false;
    
    return this.validateSpecific();
  }

  /**
   * Check if widget is currently visible
   */
  isVisible(): boolean {
    return this._config.visible !== false && !this._destroyed;
  }

  /**
   * Check if widget is currently enabled
   */
  isEnabled(): boolean {
    return !this._config.disabled && !this._destroyed;
  }

  /**
   * Check if widget is focusable
   */
  isFocusable(): boolean {
    return this._config.focusable === true && this.isEnabled() && this.isVisible();
  }

  /**
   * Get CSS classes for this widget
   */
  protected getClasses(): string[] {
    const classes = [
      `widget-${this.type}`,
      `widget-id-${this.id}`,
      ...(this._config.classes || [])
    ];

    if (this._config.disabled) classes.push('widget-disabled');
    if (!this.isVisible()) classes.push('widget-hidden');
    if (this.isFocusable()) classes.push('widget-focusable');

    return classes;
  }

  /**
   * Get attributes for this widget
   */
  protected getAttributes(): Record<string, string> {
    const attributes = { ...this._config.attributes };
    
    if (this._config.tabIndex !== undefined) {
      attributes.tabindex = this._config.tabIndex.toString();
    }

    return attributes;
  }

  /**
   * Create the element builder for this widget
   * Subclasses must implement this method
   */
  protected abstract createElement(): ElementBuilder;

  /**
   * Widget-specific validation logic
   * Subclasses can override this method
   */
  protected validateSpecific(): boolean {
    return true;
  }

  /**
   * Called when configuration is updated
   * Subclasses can override this method
   */
  protected onConfigUpdate(oldConfig: BaseWidgetConfig, newConfig: BaseWidgetConfig): void {
    // Default implementation does nothing
  }

  /**
   * Called when widget is being destroyed
   * Subclasses can override this method
   */
  protected onDestroy(): void {
    // Default implementation does nothing
  }

  /**
   * Determine if the widget should re-render
   * Subclasses can override this method
   */
  protected shouldRerender(): boolean {
    // Default: always re-render when render() is called without cached element
    return true;
  }

  /**
   * Create a base container div with common styling
   */
  protected createContainer(): ElementBuilder {
    const builder = div({
      id: this.id,
      classes: this.getClasses()
    });

    // Apply attributes
    const attributes = this.getAttributes();
    for (const [key, value] of Object.entries(attributes)) {
      builder.attr(key, value);
    }

    // Apply focusable settings
    if (this.isFocusable()) {
      builder.focusable(true);
      if (this._config.tabIndex !== undefined) {
        builder.tab_index(this._config.tabIndex);
      }
    }

    return builder;
  }
}

/**
 * Helper function to create widget builders that automatically register with factory
 */
export function createWidgetBuilder<T extends BaseWidgetConfig>(
  type: string,
  WidgetClass: new (config: T) => BaseWidget,
  schema: import('../widget-factory').WidgetSchema
) {
  // Register with factory
  import('../widget-factory').then(({ widgetRegistry }) => {
    widgetRegistry.register(
      type,
      (config: T) => new WidgetClass(config),
      schema
    );
  });

  // Return builder function
  return (config: T): BaseWidget => new WidgetClass(config);
}

/**
 * Performance monitoring mixin
 */
export interface PerformanceMonitoring {
  renderTime: number;
  updateCount: number;
  lastRenderAt: number;
}

// Note: withPerformanceMonitoring mixin removed due to TypeScript complexity
// Widgets can implement PerformanceMonitoring interface directly if needed

/**
 * State management mixin
 */
export interface StatefulWidget<S = any> {
  getState(): S;
  setState(state: Partial<S>): void;
  resetState(): void;
}

// Note: withState mixin removed due to TypeScript complexity
// Widgets can implement StatefulWidget interface directly if needed