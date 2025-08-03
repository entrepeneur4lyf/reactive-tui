/**
 * Plugin Architecture for Extensible Widgets - TypeScript Implementation
 * 
 * A comprehensive plugin system that allows developers to create and register
 * custom widgets, extend existing functionality, and share reusable components.
 * 
 * Features:
 * - Dynamic widget registration
 * - Plugin lifecycle management
 * - Dependency resolution
 * - Event hooks and interceptors
 * - Plugin configuration and settings
 * - Hot loading/unloading of plugins
 * - Plugin marketplace integration
 * 
 * Example:
 * ```typescript
 * import { PluginManager, WidgetPlugin } from 'tui-bun/plugin';
 * 
 * // Create a custom widget plugin
 * class MyCustomWidget extends WidgetPlugin {
 *   render() {
 *     return div()
 *       .class('custom-widget')
 *       .child(text(this.config.properties.text));
 *   }
 * }
 * 
 * // Register the plugin
 * const pluginManager = new PluginManager();
 * pluginManager.register(new MyCustomWidget({
 *   id: 'my-custom-widget',
 *   version: '1.0.0'
 * }));
 * ```
 */

import type { Component, Element } from './components';
import { EventEmitter } from 'events';
import { writeFile, readFile, readdir } from 'fs/promises';
import { join } from 'path';

/// Plugin metadata
export interface PluginMetadata {
    /// Plugin name
    name: string;
    /// Plugin version
    version: string;
    /// Plugin author
    author: string;
    /// Plugin description
    description: string;
    /// Plugin homepage/repository
    homepage?: string;
    /// Plugin dependencies
    dependencies: PluginDependency[];
    /// Plugin capabilities
    capabilities: PluginCapability[];
    /// Plugin tags for categorization
    tags: string[];
}

/// Plugin dependency
export interface PluginDependency {
    /// Dependency plugin ID
    pluginId: string;
    /// Minimum version required
    minVersion?: string;
    /// Maximum version allowed
    maxVersion?: string;
    /// Whether the dependency is optional
    optional: boolean;
}

/// Plugin capabilities
export enum PluginCapability {
    /// Can create widgets
    WidgetProvider = 'widget-provider',
    /// Can modify existing widgets
    WidgetExtender = 'widget-extender',
    /// Can intercept events
    EventInterceptor = 'event-interceptor',
    /// Can provide themes
    ThemeProvider = 'theme-provider',
    /// Can provide layouts
    LayoutProvider = 'layout-provider',
    /// Can provide data sources
    DataProvider = 'data-provider',
}

/// Widget configuration
export interface WidgetConfig {
    /// Widget ID
    id: string;
    /// Widget type
    widgetType: string;
    /// Widget properties
    properties: Record<string, any>;
    /// CSS classes
    cssClasses: string[];
    /// Event handlers
    eventHandlers: Record<string, string>;
}

/// Plugin events
export type PluginEvent = 
    | { type: 'plugin-loaded'; pluginId: string }
    | { type: 'plugin-unloading'; pluginId: string }
    | { type: 'widget-created'; widgetType: string; widgetId: string }
    | { type: 'widget-destroyed'; widgetId: string }
    | { type: 'custom'; eventType: string; data: any };

/// Plugin response to events
export type PluginResponse = 
    | { type: 'continue' }
    | { type: 'stop-propagation' }
    | { type: 'modify-event'; event: PluginEvent }
    | { type: 'data'; data: any };

/// Plugin context for accessing framework functionality
export class PluginContext {
    constructor(
        private manager: PluginManager,
        private pluginId: string,
        private sharedState: Map<string, any>
    ) {}

    /// Get a reference to another plugin
    getPlugin(pluginId: string): Plugin | undefined {
        return this.manager.getPlugin(pluginId);
    }

    /// Store state data
    setState<T>(key: string, value: T): void {
        const fullKey = `${this.pluginId}:${key}`;
        this.sharedState.set(fullKey, value);
    }

    /// Retrieve state data
    getState<T>(key: string): T | undefined {
        const fullKey = `${this.pluginId}:${key}`;
        return this.sharedState.get(fullKey);
    }

    /// Emit an event to other plugins
    emitEvent(event: PluginEvent): void {
        this.manager.broadcastEvent(event);
    }
}

/// Base plugin class
export abstract class Plugin extends EventEmitter {
    /// Plugin ID
    abstract readonly id: string;
    
    /// Plugin metadata
    abstract readonly metadata: PluginMetadata;

    /// Initialize the plugin
    abstract initialize(context: PluginContext): Promise<void>;

    /// Cleanup when plugin is unloaded
    abstract cleanup(): Promise<void>;

    /// Handle plugin-specific events
    handleEvent(_event: PluginEvent): PluginResponse | null {
        return null;
    }
}

/// Widget plugin base class
export abstract class WidgetPlugin extends Plugin implements Component {
    /// Widget configuration
    protected config: WidgetConfig;

    constructor(config: Partial<WidgetConfig> = {}) {
        super();
        this.config = {
            id: config.id || this.generateId(),
            widgetType: config.widgetType || 'unknown',
            properties: config.properties || {},
            cssClasses: config.cssClasses || [],
            eventHandlers: config.eventHandlers || {},
        };
    }

    /// Get the widget type name
    abstract get widgetType(): string;

    /// Create a new instance of the widget
    abstract createInstance(config: WidgetConfig): WidgetPlugin;

    /// Get widget-specific configuration schema
    abstract get configSchema(): any;

    /// Validate widget configuration
    validateConfig(config: WidgetConfig): boolean {
        // Basic validation - can be overridden
        return !!config.id && config.widgetType === this.widgetType;
    }

    /// Render the widget
    abstract render(): Element;

    /// Generate unique ID
    private generateId(): string {
        return `${this.widgetType}-${Date.now()}-${Math.random().toString(36).substr(2, 9)}`;
    }
}

/// Plugin manager for handling plugin lifecycle with error boundaries
export class PluginManager extends EventEmitter {
    /// Registered plugins
    private plugins = new Map<string, Plugin>();
    /// Widget registry
    private widgetRegistry = new Map<string, WidgetPlugin>();
    /// Event interceptors
    private eventInterceptors: string[] = [];
    /// Plugin load order for dependency resolution
    private loadOrder: string[] = [];
    /// Shared plugin state
    private sharedState = new Map<string, any>();

    /// Register a plugin with error handling
    async register(plugin: Plugin): Promise<void> {
        const pluginId = plugin.id;
        const metadata = plugin.metadata;

        try {
            // Check dependencies
            this.checkDependencies(metadata.dependencies);

            // Create plugin context
            const context = new PluginContext(this, pluginId, this.sharedState);

            // Initialize plugin
            await plugin.initialize(context);

            // Store plugin
            this.plugins.set(pluginId, plugin);
            this.loadOrder.push(pluginId);

            // Check if it's a widget plugin
            if (plugin instanceof WidgetPlugin) {
                this.widgetRegistry.set(plugin.widgetType, plugin);
            }

            // Check for event interceptor capability
            if (metadata.capabilities.includes(PluginCapability.EventInterceptor)) {
                this.eventInterceptors.push(pluginId);
            }

            console.log(`✅ Plugin '${pluginId}' registered successfully`);

        } catch (error) {
            console.warn(`⚠️ Plugin '${pluginId}' failed to register:`, error);
            // Don't throw - allow app to continue without this plugin
        }

        // Broadcast plugin loaded event
        this.broadcastEvent({ type: 'plugin-loaded', pluginId });
        this.emit('plugin-loaded', pluginId);
    }

    /// Unregister a plugin
    async unregister(pluginId: string): Promise<void> {
        const plugin = this.plugins.get(pluginId);
        if (!plugin) {
            throw new Error(`Plugin '${pluginId}' not found`);
        }

        // Broadcast unloading event
        this.broadcastEvent({ type: 'plugin-unloading', pluginId });

        // Cleanup plugin
        await plugin.cleanup();

        // Remove from registries
        this.plugins.delete(pluginId);
        this.loadOrder = this.loadOrder.filter(id => id !== pluginId);
        this.eventInterceptors = this.eventInterceptors.filter(id => id !== pluginId);

        // Remove widget registrations
        if (plugin instanceof WidgetPlugin) {
            this.widgetRegistry.delete(plugin.widgetType);
        }

        this.emit('plugin-unloaded', pluginId);
    }

    /// Get a plugin by ID
    getPlugin(pluginId: string): Plugin | undefined {
        return this.plugins.get(pluginId);
    }

    /// List all registered plugins
    listPlugins(): PluginMetadata[] {
        return Array.from(this.plugins.values()).map(p => p.metadata);
    }

    /// Create a widget from a plugin with error handling
    createWidget(widgetType: string, config: WidgetConfig): Component {
        try {
            const widgetPlugin = this.widgetRegistry.get(widgetType);
            if (!widgetPlugin) {
                console.warn(`Widget type '${widgetType}' not found, creating fallback`);
                return this.createFallbackWidget(widgetType, config);
            }

            // Validate configuration
            if (!widgetPlugin.validateConfig(config)) {
                console.warn(`Invalid configuration for widget type '${widgetType}', using defaults`);
                // Try with minimal config
                const minimalConfig = { ...config, properties: {} };
                if (widgetPlugin.validateConfig(minimalConfig)) {
                    return widgetPlugin.createInstance(minimalConfig);
                }
                return this.createFallbackWidget(widgetType, config);
            }

            // Create instance
            const widget = widgetPlugin.createInstance(config);

            // Wrap render method with error boundary
            if (widget.render) {
                const originalRender = widget.render.bind(widget);
                widget.render = () => {
                    try {
                        return originalRender();
                    } catch (error) {
                        console.warn(`Widget rendering failed for '${widgetType}':`, error);
                        return this.createErrorElement(widgetType, error as Error);
                    }
                };
            }

            return widget;

        } catch (error) {
            console.warn(`Widget creation failed for type '${widgetType}':`, error);
            return this.createFallbackWidget(widgetType, config);
        }
    }

    /// Create fallback widget when plugin fails
    private createFallbackWidget(widgetType: string, config: WidgetConfig): Component {
        return {
            render: () => this.createErrorElement(widgetType, new Error('Widget unavailable')),
            id: config.id,
            type: 'fallback-widget'
        } as Component;
    }

    /// Create error element for failed widgets
    private createErrorElement(widgetType: string, error: Error): Element {
        return {
            tag: 'div',
            classes: ['error-fallback'],
            attributes: {},
            content: null,
            children: [
                {
                    tag: 'text',
                    classes: [],
                    attributes: {},
                    content: `🚨 ${widgetType} Error: ${error.message}`,
                    children: [],
                    id: null,
                    focusable: false,
                    focused: false,
                    tab_index: null,
                    key_bindings: [],
                    modal: false
                }
            ],
            id: `error-${widgetType}-${Date.now()}`,
            focusable: false,
            focused: false,
            tab_index: null,
            key_bindings: [],
            modal: false
        };
    }

    /// Register a widget type
    registerWidget(widget: WidgetPlugin): void {
        this.widgetRegistry.set(widget.widgetType, widget);
    }

    /// Check plugin dependencies
    private checkDependencies(dependencies: PluginDependency[]): void {
        for (const dep of dependencies) {
            if (!dep.optional && !this.plugins.has(dep.pluginId)) {
                throw new Error(`Required dependency '${dep.pluginId}' not found`);
            }
            // Check version compatibility
            if (dep.minVersion && this.plugins.has(dep.pluginId)) {
                const plugin = this.plugins.get(dep.pluginId)!;
                const pluginVersion = plugin.metadata.version;
                if (!this.isVersionCompatible(pluginVersion, dep.minVersion)) {
                    throw new Error(
                        `Plugin '${dep.pluginId}' version ${pluginVersion} does not meet minimum version ${dep.minVersion}`
                    );
                }
            }
        }
    }

    /// Broadcast an event to all plugins
    broadcastEvent(event: PluginEvent): void {
        // First, send to interceptors
        for (const interceptorId of this.eventInterceptors) {
            const plugin = this.plugins.get(interceptorId);
            if (plugin) {
                const response = plugin.handleEvent(event);
                if (response?.type === 'stop-propagation') {
                    return;
                }
                if (response?.type === 'modify-event') {
                    event = response.event;
                }
            }
        }

        // Then, send to all plugins
        for (const plugin of this.plugins.values()) {
            plugin.handleEvent(event);
        }
    }

    /// Load plugins from a directory
    async loadPluginDirectory(path: string): Promise<void> {
        try {
            const files = await readdir(path);
            const pluginFiles = files.filter(f => 
                f.endsWith('.plugin.js') || 
                f.endsWith('.plugin.ts') || 
                f.endsWith('.plugin.mjs')
            );
            
            for (const file of pluginFiles) {
                const fullPath = join(path, file);
                try {
                    const pluginModule = await import(fullPath);
                    if (pluginModule.default && typeof pluginModule.default === 'function') {
                        const plugin = new pluginModule.default();
                        if (this.isValidPlugin(plugin)) {
                            await this.register(plugin);
                        }
                    }
                } catch (error) {
                    console.error(`Failed to load plugin from ${file}:`, error);
                }
            }
        } catch (error) {
            console.error(`Failed to read plugin directory ${path}:`, error);
        }
    }

    /// Save plugin configuration
    async saveConfig(path: string): Promise<void> {
        const config = {
            plugins: this.listPlugins(),
            loadOrder: this.loadOrder,
            version: '1.0.0',
            savedAt: new Date().toISOString()
        };
        
        const json = JSON.stringify(config, null, 2);
        await writeFile(path, json, 'utf-8');
    }

    /// Load plugin configuration
    async loadConfig(path: string): Promise<void> {
        try {
            const json = await readFile(path, 'utf-8');
            const config = JSON.parse(json);
            
            // Validate config structure
            if (!config.plugins || !Array.isArray(config.plugins)) {
                throw new Error('Invalid plugin configuration format');
            }
            
            // Load plugins in the saved order
            for (const pluginInfo of config.plugins) {
                // Plugin loading would need to be implemented based on
                // how plugins are stored/referenced (by path, npm package, etc)
                console.log(`Would load plugin: ${pluginInfo.name} v${pluginInfo.version}`);
            }
            
            // Restore load order if present
            if (config.loadOrder && Array.isArray(config.loadOrder)) {
                this.loadOrder = config.loadOrder;
            }
        } catch (error) {
            throw new Error(`Failed to load plugin configuration: ${error}`);
        }
    }

    /// Get widget types
    getWidgetTypes(): string[] {
        return Array.from(this.widgetRegistry.keys());
    }

    /// Get widget schema
    getWidgetSchema(widgetType: string): any {
        const widget = this.widgetRegistry.get(widgetType);
        return widget?.configSchema;
    }
    
    /// Check if version meets minimum requirement
    private isVersionCompatible(current: string, minimum: string): boolean {
        const parseVersion = (v: string) => {
            const parts = v.split('.').map(p => parseInt(p, 10));
            return {
                major: parts[0] || 0,
                minor: parts[1] || 0,
                patch: parts[2] || 0
            };
        };
        
        const curr = parseVersion(current);
        const min = parseVersion(minimum);
        
        if (curr.major < min.major) return false;
        if (curr.major > min.major) return true;
        if (curr.minor < min.minor) return false;
        if (curr.minor > min.minor) return true;
        return curr.patch >= min.patch;
    }
    
    /// Validate if an object is a valid plugin
    private isValidPlugin(obj: any): obj is Plugin {
        return obj && 
               typeof obj.id === 'string' && 
               typeof obj.metadata === 'object' &&
               typeof obj.initialize === 'function' &&
               typeof obj.cleanup === 'function';
    }
}

/**
 * Example custom widget plugin
 */
export class ExampleCustomButton extends WidgetPlugin {
    readonly id = 'example-custom-button-plugin';
    readonly widgetType = 'custom-button';
    
    readonly metadata: PluginMetadata = {
        name: 'Custom Button Plugin',
        version: '1.0.0',
        author: 'TUI Framework',
        description: 'Example custom button widget plugin',
        dependencies: [],
        capabilities: [PluginCapability.WidgetProvider],
        tags: ['widget', 'button', 'example'],
    };

    get configSchema() {
        return {
            type: 'object',
            properties: {
                text: { type: 'string' },
                color: { type: 'string' },
                size: { type: 'string', enum: ['small', 'medium', 'large'] },
            },
            required: ['text'],
        };
    }

    async initialize(context: PluginContext): Promise<void> {
        // Plugin initialization
        context.setState('initialized', true);
    }

    async cleanup(): Promise<void> {
        // Plugin cleanup
    }

    createInstance(config: WidgetConfig): WidgetPlugin {
        const instance = new ExampleCustomButton(config);
        return instance;
    }

    render(): Element {
        const { text = 'Button', color = 'primary', size = 'medium' } = this.config.properties;
        
        return {
            tag: 'div',
            classes: ['custom-button', `btn-${color}`, `btn-${size}`, ...this.config.cssClasses],
            attributes: {},
            content: text,
            children: [],
            id: this.config.id,
            focusable: true,
            focused: false,
            tab_index: null,
            key_bindings: [],
            modal: false
        };
    }
}

/**
 * Plugin builder for easier plugin creation
 */
export class PluginBuilder {
    private pluginConfig: Partial<PluginMetadata> = {
        dependencies: [],
        capabilities: [],
        tags: [],
    };

    id(id: string): this {
        this.pluginConfig.name = id;
        return this;
    }

    version(version: string): this {
        this.pluginConfig.version = version;
        return this;
    }

    author(author: string): this {
        this.pluginConfig.author = author;
        return this;
    }

    description(description: string): this {
        this.pluginConfig.description = description;
        return this;
    }

    dependency(dep: PluginDependency): this {
        this.pluginConfig.dependencies!.push(dep);
        return this;
    }

    capability(cap: PluginCapability): this {
        this.pluginConfig.capabilities!.push(cap);
        return this;
    }

    tag(tag: string): this {
        this.pluginConfig.tags!.push(tag);
        return this;
    }

    build(): PluginMetadata {
        if (!this.pluginConfig.name || !this.pluginConfig.version || !this.pluginConfig.author) {
            throw new Error('Plugin must have name, version, and author');
        }
        return this.pluginConfig as PluginMetadata;
    }
}

/**
 * Create a simple plugin
 */
export function createPlugin(
    id: string,
    metadata: PluginMetadata,
    handlers: {
        initialize?: (context: PluginContext) => Promise<void>;
        cleanup?: () => Promise<void>;
        handleEvent?: (event: PluginEvent) => PluginResponse | null;
    }
): Plugin {
    return new (class extends Plugin {
        readonly id = id;
        readonly metadata = metadata;

        async initialize(context: PluginContext): Promise<void> {
            if (handlers.initialize) {
                await handlers.initialize(context);
            }
        }

        async cleanup(): Promise<void> {
            if (handlers.cleanup) {
                await handlers.cleanup();
            }
        }

        override handleEvent(event: PluginEvent): PluginResponse | null {
            if (handlers.handleEvent) {
                return handlers.handleEvent(event);
            }
            return null;
        }
    })();
}