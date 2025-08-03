/**
 * Button Widget Factory Implementation
 * 
 * Demonstrates how to integrate existing widgets with the factory pattern
 */

import type { Element } from '../generated-types';
import type { ElementBuilder } from '../types';
import { text } from '../components';
import { BaseWidget, createWidgetBuilder } from './base-widget';
import type { BaseWidgetConfig, WidgetSchema } from '../widget-factory';
import { widgetRegistry } from '../widget-factory';

// Button-specific configuration
export interface ButtonConfig extends BaseWidgetConfig {
  type: 'button';
  text: string;
  variant?: 'filled' | 'outlined' | 'ghost';
  color?: 'primary' | 'secondary' | 'success' | 'warning' | 'error';
  size?: 'xs' | 'sm' | 'md' | 'lg' | 'xl';
  icon?: {
    symbol: string;
    position: 'left' | 'right';
  };
  loading?: boolean;
  fullWidth?: boolean;
  onClick?: () => void;
}

// Button widget schema for validation
export const buttonSchema: WidgetSchema = {
  type: 'button',
  required: ['id', 'type', 'text'],
  properties: {
    id: {
      type: 'string',
      validator: (value: string) => value.length > 0
    },
    type: {
      type: 'string',
      enum: ['button'],
    },
    text: {
      type: 'string',
      validator: (value: string) => value.length > 0
    },
    variant: {
      type: 'string',
      enum: ['filled', 'outlined', 'ghost'],
      default: 'filled'
    },
    color: {
      type: 'string',
      enum: ['primary', 'secondary', 'success', 'warning', 'error'],
      default: 'primary'
    },
    size: {
      type: 'string',
      enum: ['xs', 'sm', 'md', 'lg', 'xl'],
      default: 'md'
    },
    loading: {
      type: 'boolean',
      default: false
    },
    fullWidth: {
      type: 'boolean',
      default: false
    },
    disabled: {
      type: 'boolean',
      default: false
    },
    visible: {
      type: 'boolean',
      default: true
    },
    focusable: {
      type: 'boolean',
      default: true
    }
  }
};

// Button widget implementation
export class ButtonWidget extends BaseWidget {
  private buttonConfig: ButtonConfig;

  constructor(config: ButtonConfig) {
    super(config);
    this.buttonConfig = config;
  }

  protected createElement(): ElementBuilder {
    const container = this.createContainer();
    
    // Add button-specific classes
    const classes = this.getButtonClasses();
    classes.forEach(cls => container.class(cls));

    // Handle click events
    if (this.buttonConfig.onClick && this.isEnabled()) {
      container.bind_enter();
      container.bind_space();
    }

    // Create button content
    const content = this.createButtonContent();
    container.child(content);

    return container;
  }

  protected override validateSpecific(): boolean {
    return super.validateSpecific() && 
           typeof this.buttonConfig.text === 'string' && 
           this.buttonConfig.text.length > 0;
  }

  protected override onConfigUpdate(oldConfig: BaseWidgetConfig, newConfig: BaseWidgetConfig): void {
    super.onConfigUpdate(oldConfig, newConfig);
    
    // Type-safe config update for button
    if ('text' in newConfig || 'variant' in newConfig || 'color' in newConfig) {
      this.buttonConfig = { ...this.buttonConfig, ...newConfig } as ButtonConfig;
    }
  }

  private getButtonClasses(): string[] {
    const classes = [
      'btn',
      `btn-${this.buttonConfig.variant || 'filled'}`,
      `btn-${this.buttonConfig.color || 'primary'}`,
      `btn-${this.buttonConfig.size || 'md'}`
    ];

    if (this.buttonConfig.loading) {
      classes.push('btn-loading');
    }

    if (this.buttonConfig.fullWidth) {
      classes.push('btn-full-width');
    }

    return classes;
  }

  private createButtonContent(): Element {
    const elements: Element[] = [];

    // Add loading spinner if loading
    if (this.buttonConfig.loading) {
      elements.push(
        text('⟳', { class: 'btn-spinner' }).build()
      );
    }

    // Add left icon
    if (this.buttonConfig.icon?.position === 'left') {
      elements.push(
        text(this.buttonConfig.icon.symbol, { class: 'btn-icon btn-icon-left' }).build()
      );
    }

    // Add button text
    elements.push(
      text(this.buttonConfig.text, { class: 'btn-text' }).build()
    );

    // Add right icon
    if (this.buttonConfig.icon?.position === 'right') {
      elements.push(
        text(this.buttonConfig.icon.symbol, { class: 'btn-icon btn-icon-right' }).build()
      );
    }

    // Return container with all elements
    const container = text('').children(elements);
    return container.build();
  }

  // Public API methods
  setText(text: string): void {
    this.buttonConfig.text = text;
    this.update({ text } as any);
  }

  setLoading(loading: boolean): void {
    (this.buttonConfig as any).loading = loading;
    this.update({ loading } as any);
  }

  setVariant(variant: ButtonConfig['variant']): void {
    this.update({ variant } as Partial<ButtonConfig>);
  }

  setColor(color: ButtonConfig['color']): void {
    this.update({ color } as Partial<ButtonConfig>);
  }

  getButtonConfig(): ButtonConfig {
    return { ...this.buttonConfig };
  }
}

// Create and register the button widget with factory
export const button = createWidgetBuilder<ButtonConfig>(
  'button',
  ButtonWidget,
  buttonSchema
);

// Convenience functions for common button types
export function primaryButton(id: string, text: string, onClick?: () => void): ButtonWidget {
  return new ButtonWidget({
    id,
    type: 'button',
    text,
    variant: 'filled',
    color: 'primary',
    onClick
  });
}

export function secondaryButton(id: string, text: string, onClick?: () => void): ButtonWidget {
  return new ButtonWidget({
    id,
    type: 'button',
    text,
    variant: 'outlined',
    color: 'secondary',
    onClick
  });
}

export function dangerButton(id: string, text: string, onClick?: () => void): ButtonWidget {
  return new ButtonWidget({
    id,
    type: 'button',
    text,
    variant: 'filled',
    color: 'error',
    onClick
  });
}

export function iconButton(
  id: string, 
  icon: string, 
  onClick?: () => void,
  options: Partial<ButtonConfig> = {}
): ButtonWidget {
  return new ButtonWidget({
    id,
    type: 'button',
    text: '',
    variant: 'ghost',
    icon: { symbol: icon, position: 'left' },
    onClick,
    ...options
  });
}

export function loadingButton(
  id: string, 
  text: string, 
  onClick?: () => void,
  loading: boolean = true
): ButtonWidget {
  return new ButtonWidget({
    id,
    type: 'button',
    text,
    loading,
    onClick
  });
}

// Button group utility
export function buttonGroup(buttons: ButtonWidget[], options: {
  id?: string;
  orientation?: 'horizontal' | 'vertical';
  spacing?: 'tight' | 'normal' | 'loose';
} = {}): Element {
  const {
    id = `button-group-${Date.now()}`,
    orientation = 'horizontal',
    spacing = 'normal'
  } = options;

  const container = text('').class('btn-group')
    .class(`btn-group-${orientation}`)
    .class(`btn-group-spacing-${spacing}`);

  if (id) {
    container.id(id);
  }

  // Render all buttons
  const buttonElements = buttons.map(btn => btn.render());
  container.children(buttonElements);

  return container.build();
}

// Register button widget with the factory
widgetRegistry.register('button', (config: ButtonConfig) => {
  return new ButtonWidget(config);
}, buttonSchema);

// ButtonConfig is already exported above as interface