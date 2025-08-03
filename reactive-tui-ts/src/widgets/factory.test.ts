/**
 * Widget Factory Pattern Tests
 * 
 * Comprehensive test suite for the widget factory system
 */

import { describe, test, expect, beforeEach, afterEach } from 'bun:test';
import { 
  WidgetFactory, 
  widgetRegistry, 
  ConfigValidator,
  WidgetFactoryError,
  createWidget,
  createWidgetCreator
} from '../widget-factory';
import { ButtonWidget, buttonSchema, primaryButton, secondaryButton, dangerButton } from './factory-button';
import type { ButtonConfig } from './factory-button';

describe('Widget Factory Pattern', () => {
  beforeEach(() => {
    WidgetFactory.clearCache();
  });

  afterEach(() => {
    WidgetFactory.clearCache();
  });

  describe('WidgetRegistry', () => {
    test('should register widget types', () => {
      const types = widgetRegistry.getTypes();
      expect(types).toContain('button');
    });

    test('should check if widget type exists', () => {
      expect(widgetRegistry.hasType('button')).toBe(true);
      expect(widgetRegistry.hasType('nonexistent')).toBe(false);
    });

    test('should get widget schema', () => {
      const schema = widgetRegistry.getSchema('button');
      expect(schema).toBeDefined();
      expect(schema?.type).toBe('button');
      expect(schema?.required).toContain('id');
      expect(schema?.required).toContain('text');
    });

    test('should provide cache statistics', () => {
      const stats = widgetRegistry.getCacheStats();
      expect(stats).toHaveProperty('totalInstances');
      expect(stats).toHaveProperty('typeDistribution');
      expect(stats).toHaveProperty('memoryUsage');
    });
  });

  describe('ConfigValidator', () => {
    test('should validate required fields', () => {
      const config = {
        id: 'test-button',
        type: 'button' as const,
        text: 'Click me'
      };

      const result = ConfigValidator.validate(config, buttonSchema);
      expect(result.valid).toBe(true);
      expect(result.errors).toHaveLength(0);
    });

    test('should detect missing required fields', () => {
      const config = {
        id: 'test-button',
        type: 'button' as const
        // missing 'text' field
      };

      const result = ConfigValidator.validate(config as any, buttonSchema);
      expect(result.valid).toBe(false);
      expect(result.errors).toContain("Required field 'text' is missing");
    });

    test('should validate field types', () => {
      const config = {
        id: 'test-button',
        type: 'button' as const,
        text: 123 // should be string
      };

      const result = ConfigValidator.validate(config as any, buttonSchema);
      expect(result.valid).toBe(false);
      expect(result.errors.some(e => e.includes('must be of type string'))).toBe(true);
    });

    test('should validate enum values', () => {
      const config = {
        id: 'test-button',
        type: 'button' as const,
        text: 'Click me',
        variant: 'invalid-variant' as any
      };

      const result = ConfigValidator.validate(config, buttonSchema);
      expect(result.valid).toBe(false);
      expect(result.errors.some(e => e.includes('must be one of'))).toBe(true);
    });

    test('should apply default values', () => {
      const config = {
        id: 'test-button',
        type: 'button' as const,
        text: 'Click me'
      };

      const withDefaults = ConfigValidator.applyDefaults(config, buttonSchema) as any;
      expect(withDefaults.variant).toBe('filled');
      expect(withDefaults.color).toBe('primary');
      expect(withDefaults.size).toBe('md');
    });

    test('should run custom validators', () => {
      const config = {
        id: '', // should fail custom validator
        type: 'button' as const,
        text: 'Click me'
      };

      const result = ConfigValidator.validate(config, buttonSchema);
      expect(result.valid).toBe(false);
      expect(result.errors.some(e => e.includes('failed custom validation'))).toBe(true);
    });
  });

  describe('WidgetFactory', () => {
    test('should create widget instances', () => {
      const config: ButtonConfig = {
        id: 'test-button',
        type: 'button',
        text: 'Click me',
        variant: 'filled',
        color: 'primary'
      };

      const instance = WidgetFactory.create('button', config);
      expect(instance).toBeInstanceOf(ButtonWidget);
      expect(instance.id).toBe('test-button');
      expect(instance.type).toBe('button');
    });

    test('should throw error for unknown widget type', () => {
      const config = {
        id: 'test-widget',
        type: 'unknown',
        text: 'Test'
      };

      expect(() => {
        WidgetFactory.create('unknown', config as any);
      }).toThrow(WidgetFactoryError);
    });

    test('should validate configuration by default', () => {
      const config = {
        id: 'test-button',
        type: 'button',
        // missing required 'text' field
      };

      expect(() => {
        WidgetFactory.create('button', config as any);
      }).toThrow(WidgetFactoryError);
    });

    test('should skip validation when disabled', () => {
      const config = {
        id: 'test-button',
        type: 'button',
        // missing required 'text' field
      };

      expect(() => {
        WidgetFactory.create('button', config as any, { validateConfig: false });
      }).not.toThrow();
    });

    test('should cache instances by default', () => {
      const config: ButtonConfig = {
        id: 'cached-button',
        type: 'button',
        text: 'Click me'
      };

      const instance1 = WidgetFactory.create('button', config);
      const instance2 = WidgetFactory.create('button', config);
      
      expect(instance1).toBe(instance2); // Same instance from cache
    });

    test('should create new instances when caching disabled', () => {
      const config: ButtonConfig = {
        id: 'no-cache-button',
        type: 'button',
        text: 'Click me'
      };

      const instance1 = WidgetFactory.create('button', config, { useCache: false });
      const instance2 = WidgetFactory.create('button', config, { useCache: false });
      
      expect(instance1).not.toBe(instance2); // Different instances
    });

    test('should create widgets in batch', () => {
      const widgets = [
        { type: 'button', config: { id: 'btn1', type: 'button' as const, text: 'Button 1' } },
        { type: 'button', config: { id: 'btn2', type: 'button' as const, text: 'Button 2' } },
        { type: 'button', config: { id: 'btn3', type: 'button' as const, text: 'Button 3' } }
      ];

      const instances = WidgetFactory.createBatch(widgets);
      expect(instances).toHaveLength(3);
      expect(instances.every(i => i.type === 'button')).toBe(true);
    });

    test('should handle batch errors gracefully', () => {
      const widgets = [
        { type: 'button', config: { id: 'btn1', type: 'button' as const, text: 'Button 1' } },
        { type: 'unknown', config: { id: 'btn2', type: 'unknown' as const, text: 'Button 2' } },
        { type: 'button', config: { id: 'btn3', type: 'button' as const, text: 'Button 3' } }
      ];

      expect(() => {
        WidgetFactory.createBatch(widgets as any);
      }).toThrow(WidgetFactoryError);

      // With continueOnError enabled
      const instances = WidgetFactory.createBatch(widgets as any, { continueOnError: true });
      expect(instances).toHaveLength(2); // Only successful ones
    });

    test('should update widget instances', () => {
      const config: ButtonConfig = {
        id: 'update-button',
        type: 'button',
        text: 'Original text'
      };

      const instance = WidgetFactory.create('button', config);
      expect((instance as ButtonWidget).getButtonConfig().text).toBe('Original text');

      WidgetFactory.update('update-button', { text: 'Updated text' } as any);
      expect((instance as ButtonWidget).getButtonConfig().text).toBe('Updated text');
    });

    test('should destroy widget instances', () => {
      const config: ButtonConfig = {
        id: 'destroy-button',
        type: 'button',
        text: 'Click me'
      };

      WidgetFactory.create('button', config);
      expect(WidgetFactory.getInstance('destroy-button')).toBeDefined();

      const destroyed = WidgetFactory.destroy('destroy-button');
      expect(destroyed).toBe(true);
      expect(WidgetFactory.getInstance('destroy-button')).toBeUndefined();
    });

    test('should list instances by type', () => {
      WidgetFactory.create('button', { id: 'btn1', type: 'button', text: 'Button 1' });
      WidgetFactory.create('button', { id: 'btn2', type: 'button', text: 'Button 2' });

      const buttonInstances = WidgetFactory.listInstances('button');
      expect(buttonInstances).toHaveLength(2);
      expect(buttonInstances.every(i => i.type === 'button')).toBe(true);

      const allInstances = WidgetFactory.listInstances();
      expect(allInstances.length).toBeGreaterThanOrEqual(2);
    });

    test('should provide factory statistics', () => {
      WidgetFactory.create('button', { id: 'stats-btn', type: 'button', text: 'Stats' });

      const stats = WidgetFactory.getStats();
      expect(stats.registeredTypes).toContain('button');
      expect(stats.cache.totalInstances).toBeGreaterThan(0);
    });
  });

  describe('ButtonWidget', () => {
    test('should render with correct classes', () => {
      const widget = new ButtonWidget({
        id: 'test-button',
        type: 'button',
        text: 'Click me',
        variant: 'outlined',
        color: 'secondary',
        size: 'lg'
      });

      const element = widget.render();
      expect(element.classes).toContain('btn');
      expect(element.classes).toContain('btn-outlined');
      expect(element.classes).toContain('btn-secondary');
      expect(element.classes).toContain('btn-lg');
    });

    test('should handle loading state', () => {
      const widget = new ButtonWidget({
        id: 'loading-button',
        type: 'button',
        text: 'Loading...',
        loading: true
      });

      const element = widget.render();
      expect(element.classes).toContain('btn-loading');
    });

    test('should handle icon positioning', () => {
      const widget = new ButtonWidget({
        id: 'icon-button',
        type: 'button',
        text: 'Save',
        icon: { symbol: '💾', position: 'left' }
      });

      const element = widget.render();
      // Icon should be rendered as part of content
      expect(element).toBeDefined();
    });

    test('should update configuration', () => {
      const widget = new ButtonWidget({
        id: 'update-test',
        type: 'button',
        text: 'Original'
      });

      widget.setText('Updated');
      expect(widget.getButtonConfig().text).toBe('Updated');

      widget.setLoading(true);
      expect(widget.getButtonConfig().loading).toBe(true);

      widget.setVariant('outlined');
      expect(widget.getButtonConfig().variant).toBe('outlined');

      widget.setColor('error');
      expect(widget.getButtonConfig().color).toBe('error');
    });

    test('should validate properly', () => {
      const validWidget = new ButtonWidget({
        id: 'valid-button',
        type: 'button',
        text: 'Valid'
      });
      expect(validWidget.validate()).toBe(true);

      const invalidWidget = new ButtonWidget({
        id: 'invalid-button',
        type: 'button',
        text: '' // Empty text should fail validation
      });
      expect(invalidWidget.validate()).toBe(false);
    });

    test('should handle destruction', () => {
      const widget = new ButtonWidget({
        id: 'destroy-test',
        type: 'button',
        text: 'Destroy me'
      });

      expect(widget.destroyed).toBe(false);
      widget.destroy();
      expect(widget.destroyed).toBe(true);

      expect(() => widget.render()).toThrow();
      expect(() => widget.update({})).toThrow();
    });
  });

  describe('Convenience Functions', () => {
    test('should create primary button', () => {
      const widget = primaryButton('primary-test', 'Primary Button');
      expect(widget.getButtonConfig().color).toBe('primary');
      expect(widget.getButtonConfig().variant).toBe('filled');
    });

    test('should create secondary button', () => {
      const widget = secondaryButton('secondary-test', 'Secondary Button');
      expect(widget.getButtonConfig().color).toBe('secondary');
      expect(widget.getButtonConfig().variant).toBe('outlined');
    });

    test('should create danger button', () => {
      const widget = dangerButton('danger-test', 'Danger Button');
      expect(widget.getButtonConfig().color).toBe('error');
      expect(widget.getButtonConfig().variant).toBe('filled');
    });

    test('should create widget creator function', () => {
      const createCustomButton = createWidgetCreator<ButtonConfig>('button', {
        variant: 'ghost',
        color: 'secondary'
      });

      const widget = createCustomButton({
        id: 'custom-test',
        type: 'button',
        text: 'Custom Button'
      });

      expect(widget.type).toBe('button');
    });

    test('should create generic widget', () => {
      const widget = createWidget('button', {
        id: 'generic-test',
        type: 'button',
        text: 'Generic Button'
      });

      expect(widget.type).toBe('button');
      expect(widget.id).toBe('generic-test');
    });
  });

  describe('Performance', () => {
    test('should create widgets quickly', () => {
      const start = performance.now();
      
      for (let i = 0; i < 100; i++) {
        WidgetFactory.create('button', {
          id: `perf-button-${i}`,
          type: 'button',
          text: `Button ${i}`
        });
      }
      
      const end = performance.now();
      const timePerWidget = (end - start) / 100;
      
      // Should create widgets in less than 1ms each
      expect(timePerWidget).toBeLessThan(1);
    });

    test('should handle large batch creation', () => {
      const widgets = Array.from({ length: 1000 }, (_, i) => ({
        type: 'button',
        config: {
          id: `batch-${i}`,
          type: 'button' as const,
          text: `Button ${i}`
        }
      }));

      const start = performance.now();
      const instances = WidgetFactory.createBatch(widgets);
      const end = performance.now();

      expect(instances).toHaveLength(1000);
      expect(end - start).toBeLessThan(100); // Should complete in under 100ms
    });

    test('should cache effectively', () => {
      const config: ButtonConfig = {
        id: 'cache-perf-test',
        type: 'button',
        text: 'Cached'
      };

      // First creation
      const start1 = performance.now();
      const instance1 = WidgetFactory.create('button', config);
      const end1 = performance.now();

      // Second creation (from cache)
      const start2 = performance.now();
      const instance2 = WidgetFactory.create('button', config);
      const end2 = performance.now();

      expect(instance1).toBe(instance2);
      expect(end2 - start2).toBeLessThan(end1 - start1); // Cache should be faster
    });
  });

  describe('Error Handling', () => {
    test('should provide meaningful error messages', () => {
      try {
        WidgetFactory.create('nonexistent', { id: 'test' } as any);
      } catch (error) {
        expect(error).toBeInstanceOf(WidgetFactoryError);
        expect((error as WidgetFactoryError).code).toBe('UNKNOWN_TYPE');
        expect((error as WidgetFactoryError).message).toContain('nonexistent');
      }
    });

    test('should handle validation errors gracefully', () => {
      try {
        WidgetFactory.create('button', { id: 'test' } as any); // Missing required fields
      } catch (error) {
        expect(error).toBeInstanceOf(WidgetFactoryError);
        expect((error as WidgetFactoryError).code).toBe('VALIDATION_ERROR');
      }
    });

    test('should handle update errors', () => {
      expect(() => {
        WidgetFactory.update('nonexistent-widget', { text: 'Updated' } as any);
      }).toThrow(WidgetFactoryError);
    });
  });
});