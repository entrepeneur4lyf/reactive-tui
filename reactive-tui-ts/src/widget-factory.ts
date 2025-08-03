/**
 * Widget Factory Pattern Implementation
 * 
 * Provides type-safe widget creation with configuration validation,
 * instance caching, and consistent API across all widget types.
 */

import type { Element } from './generated-types';

// Core widget configuration interface
export interface BaseWidgetConfig {
  id: string;
  type: string;
  classes?: string[];
  attributes?: Record<string, string>;
  disabled?: boolean;
  visible?: boolean;
  focusable?: boolean;
  tabIndex?: number;
}

// Widget instance with lifecycle methods
export interface WidgetInstance {
  readonly id: string;
  readonly type: string;
  readonly config: BaseWidgetConfig;
  render(): Element;
  update(config: Partial<BaseWidgetConfig>): void;
  destroy(): void;
  validate(): boolean;
}

// Widget configuration schema for validation
export interface WidgetSchema {
  type: string;
  required: string[];
  properties: Record<string, {
    type: 'string' | 'number' | 'boolean' | 'array' | 'object';
    enum?: any[];
    default?: any;
    validator?: (value: any) => boolean;
  }>;
}

// Widget builder function type
export type WidgetBuilder<T extends BaseWidgetConfig> = (config: T) => WidgetInstance;

// Widget factory registry
class WidgetRegistry {
  private builders = new Map<string, WidgetBuilder<any>>();
  private schemas = new Map<string, WidgetSchema>();
  private instances = new Map<string, WidgetInstance>();

  /**
   * Register a widget type with its builder and schema
   */
  register<T extends BaseWidgetConfig>(
    type: string,
    builder: WidgetBuilder<T>,
    schema: WidgetSchema
  ): void {
    this.builders.set(type, builder);
    this.schemas.set(type, schema);
  }

  /**
   * Unregister a widget type
   */
  unregister(type: string): void {
    this.builders.delete(type);
    this.schemas.delete(type);
  }

  /**
   * Get registered widget types
   */
  getTypes(): string[] {
    return Array.from(this.builders.keys());
  }

  /**
   * Check if widget type is registered
   */
  hasType(type: string): boolean {
    return this.builders.has(type);
  }

  /**
   * Get widget schema
   */
  getSchema(type: string): WidgetSchema | undefined {
    return this.schemas.get(type);
  }

  /**
   * Get widget builder
   */
  getBuilder<T extends BaseWidgetConfig>(type: string): WidgetBuilder<T> | undefined {
    return this.builders.get(type);
  }

  /**
   * Store widget instance for caching
   */
  cacheInstance(instance: WidgetInstance): void {
    this.instances.set(instance.id, instance);
  }

  /**
   * Get cached widget instance
   */
  getInstance(id: string): WidgetInstance | undefined {
    return this.instances.get(id);
  }

  /**
   * Remove cached instance
   */
  removeInstance(id: string): void {
    const instance = this.instances.get(id);
    if (instance) {
      instance.destroy();
      this.instances.delete(id);
    }
  }

  /**
   * Clear all cached instances
   */
  clearCache(): void {
    for (const instance of this.instances.values()) {
      instance.destroy();
    }
    this.instances.clear();
  }

  /**
   * Get cache statistics
   */
  getCacheStats() {
    const typeCount = new Map<string, number>();
    for (const instance of this.instances.values()) {
      typeCount.set(instance.type, (typeCount.get(instance.type) || 0) + 1);
    }
    return {
      totalInstances: this.instances.size,
      typeDistribution: Object.fromEntries(typeCount),
      memoryUsage: this.estimateMemoryUsage()
    };
  }

  private estimateMemoryUsage(): string {
    // Rough estimation - each instance ~1KB average
    const estimatedBytes = this.instances.size * 1024;
    if (estimatedBytes > 1024 * 1024) {
      return `${(estimatedBytes / (1024 * 1024)).toFixed(1)}MB`;
    } else if (estimatedBytes > 1024) {
      return `${(estimatedBytes / 1024).toFixed(1)}KB`;
    }
    return `${estimatedBytes}B`;
  }
}

// Global widget registry instance
export const widgetRegistry = new WidgetRegistry();

/**
 * Configuration validation utilities
 */
export class ConfigValidator {
  /**
   * Validate widget configuration against schema
   */
  static validate<T extends BaseWidgetConfig>(config: T, schema: WidgetSchema): WidgetValidationResult {
    const errors: string[] = [];
    const warnings: string[] = [];

    // Guard against undefined schema
    if (!schema || !schema.required) {
      return { valid: true, errors: [], warnings: [] };
    }

    // Check required fields
    for (const field of schema.required) {
      if (!(field in config) || config[field as keyof T] === undefined) {
        errors.push(`Required field '${field}' is missing`);
      }
    }

    // Validate property types and constraints
    for (const [key, value] of Object.entries(config)) {
      const propSchema = schema.properties[key];
      if (!propSchema) {
        warnings.push(`Unknown property '${key}' not defined in schema`);
        continue;
      }

      // Type validation
      if (!this.validateType(value, propSchema.type)) {
        errors.push(`Property '${key}' must be of type ${propSchema.type}, got ${typeof value}`);
        continue;
      }

      // Enum validation
      if (propSchema.enum && !propSchema.enum.includes(value)) {
        errors.push(`Property '${key}' must be one of: ${propSchema.enum.join(', ')}`);
      }

      // Custom validator
      if (propSchema.validator && !propSchema.validator(value)) {
        errors.push(`Property '${key}' failed custom validation`);
      }
    }

    return {
      valid: errors.length === 0,
      errors,
      warnings
    };
  }

  /**
   * Apply default values from schema
   */
  static applyDefaults<T extends BaseWidgetConfig>(config: T, schema: WidgetSchema): T {
    const result = { ...config };

    // Guard against undefined properties
    if (!schema.properties) {
      return result;
    }

    for (const [key, propSchema] of Object.entries(schema.properties)) {
      if (!(key in result) && propSchema.default !== undefined) {
        (result as any)[key] = propSchema.default;
      }
    }

    return result;
  }

  private static validateType(value: any, expectedType: string): boolean {
    switch (expectedType) {
      case 'string':
        return typeof value === 'string';
      case 'number':
        return typeof value === 'number' && !isNaN(value);
      case 'boolean':
        return typeof value === 'boolean';
      case 'array':
        return Array.isArray(value);
      case 'object':
        return typeof value === 'object' && value !== null && !Array.isArray(value);
      default:
        return true;
    }
  }
}

export interface WidgetValidationResult {
  valid: boolean;
  errors: string[];
  warnings: string[];
}

/**
 * Main widget factory class
 */
export class WidgetFactory {
  /**
   * Create a widget instance with type safety and validation
   */
  static create<T extends BaseWidgetConfig>(
    type: string,
    config: T,
    options: CreateWidgetOptions = {}
  ): WidgetInstance {
    // Check if widget type is registered
    if (!widgetRegistry.hasType(type)) {
      throw new WidgetFactoryError(`Unknown widget type: ${type}`, 'UNKNOWN_TYPE');
    }

    // Get schema and builder
    const schema = widgetRegistry.getSchema(type)!;
    const builder = widgetRegistry.getBuilder<T>(type)!;

    // Apply defaults
    const configWithDefaults = ConfigValidator.applyDefaults(config, schema);

    // Validate configuration
    if (options.validateConfig !== false) {
      const validation = ConfigValidator.validate(configWithDefaults, schema);
      if (!validation.valid) {
        throw new WidgetFactoryError(
          `Widget configuration validation failed: ${validation.errors.join(', ')}`,
          'VALIDATION_ERROR',
          { validation }
        );
      }

      // Log warnings if any
      if (validation.warnings.length > 0 && options.logWarnings !== false) {
        console.warn(`Widget ${type}(${config.id}) warnings:`, validation.warnings);
      }
    }

    // Check for cached instance if caching enabled
    if (options.useCache !== false) {
      const cached = widgetRegistry.getInstance(config.id);
      if (cached && cached.type === type) {
        // Update cached instance with new config
        cached.update(configWithDefaults);
        return cached;
      }
    }

    // Create new instance
    try {
      const instance = builder(configWithDefaults);
      
      // Cache instance if enabled
      if (options.useCache !== false) {
        widgetRegistry.cacheInstance(instance);
      }

      return instance;
    } catch (error) {
      throw new WidgetFactoryError(
        `Failed to create widget ${type}(${config.id}): ${error}`,
        'CREATION_ERROR',
        { originalError: error }
      );
    }
  }

  /**
   * Create multiple widgets in batch
   */
  static createBatch<T extends BaseWidgetConfig>(
    widgets: Array<{ type: string; config: T }>,
    options: CreateWidgetOptions = {}
  ): WidgetInstance[] {
    const results: WidgetInstance[] = [];
    const errors: Array<{ index: number; error: Error }> = [];

    for (let i = 0; i < widgets.length; i++) {
      try {
        const { type, config } = widgets[i];
        results.push(this.create(type, config, options));
      } catch (error) {
        errors.push({ index: i, error: error as Error });
        if (!options.continueOnError) {
          throw new WidgetFactoryError(
            `Batch creation failed at index ${i}: ${error}`,
            'BATCH_ERROR',
            { errors, partialResults: results }
          );
        }
      }
    }

    if (errors.length > 0 && options.continueOnError) {
      console.warn(`Batch creation completed with ${errors.length} errors:`, errors);
    }

    return results;
  }

  /**
   * Update widget configuration
   */
  static update<T extends BaseWidgetConfig>(
    id: string,
    updates: Partial<T>,
    options: UpdateWidgetOptions = {}
  ): WidgetInstance {
    const instance = widgetRegistry.getInstance(id);
    if (!instance) {
      throw new WidgetFactoryError(`Widget not found: ${id}`, 'NOT_FOUND');
    }

    // Validate updates if schema available
    if (options.validateUpdates !== false) {
      const schema = widgetRegistry.getSchema(instance.type);
      if (schema) {
        const mergedConfig = { ...instance.config, ...updates };
        const validation = ConfigValidator.validate(mergedConfig, schema);
        if (!validation.valid) {
          throw new WidgetFactoryError(
            `Widget update validation failed: ${validation.errors.join(', ')}`,
            'VALIDATION_ERROR',
            { validation }
          );
        }
      }
    }

    instance.update(updates);
    return instance;
  }

  /**
   * Destroy widget instance
   */
  static destroy(id: string): boolean {
    const instance = widgetRegistry.getInstance(id);
    if (instance) {
      widgetRegistry.removeInstance(id);
      return true;
    }
    return false;
  }

  /**
   * Get widget instance
   */
  static getInstance(id: string): WidgetInstance | undefined {
    return widgetRegistry.getInstance(id);
  }

  /**
   * List all widget instances
   */
  static listInstances(type?: string): WidgetInstance[] {
    const all = Array.from(widgetRegistry['instances'].values());
    return type ? all.filter(instance => instance.type === type) : all;
  }

  /**
   * Get factory statistics
   */
  static getStats() {
    return {
      registeredTypes: widgetRegistry.getTypes(),
      cache: widgetRegistry.getCacheStats(),
      performance: {
        // Add performance metrics here
      }
    };
  }

  /**
   * Clear all cached instances
   */
  static clearCache(): void {
    widgetRegistry.clearCache();
  }
}

// Configuration options for widget creation
export interface CreateWidgetOptions {
  validateConfig?: boolean;
  useCache?: boolean;
  logWarnings?: boolean;
  continueOnError?: boolean;
}

export interface UpdateWidgetOptions {
  validateUpdates?: boolean;
}

// Custom error class for widget factory
export class WidgetFactoryError extends Error {
  constructor(
    message: string,
    public code: string,
    public details?: any
  ) {
    super(message);
    this.name = 'WidgetFactoryError';
  }
}

/**
 * Utility function for creating type-safe widget creation functions
 */
export function createWidgetCreator<T extends BaseWidgetConfig>(
  type: string,
  defaultConfig: Partial<T> = {}
) {
  return (config: T, options?: CreateWidgetOptions): WidgetInstance => {
    const mergedConfig = { ...defaultConfig, ...config } as T;
    return WidgetFactory.create(type, mergedConfig, options);
  };
}

/**
 * Generic widget creation function with type inference
 */
export function createWidget<T extends BaseWidgetConfig>(
  type: string,
  config: T,
  options?: CreateWidgetOptions
): WidgetInstance {
  return WidgetFactory.create(type, config, options);
}