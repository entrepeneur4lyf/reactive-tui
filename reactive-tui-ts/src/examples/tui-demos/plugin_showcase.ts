#!/usr/bin/env bun
/**
 * 🚀 Plugin System Showcase - TypeScript Version
 * 
 * Interactive demonstration of the TUI framework's plugin architecture
 * with live updates, theme switching, and dynamic widget creation.
 */

import { createApp, div, text, flexRow, flexColumn } from '../../packages/tui-bun/src';
import {
    Plugin,
    WidgetPlugin,
    PluginManager,
    PluginContext,
    PluginEvent,
    PluginResponse,
    PluginCapability,
    PluginMetadata,
    WidgetConfig
} from '../../packages/tui-bun/src/plugin';
// Theme system is available but current implementation uses simple theme provider
import { panel, dashboardPanel } from '../../packages/tui-bun/src/widgets/panel';
import { barItem, headerBar, statusBar } from '../../packages/tui-bun/src/widgets/bar';
import type { Element } from '../../packages/tui-bun/src/generated-types';

/**
 * 📊 Enhanced Gauge Widget Plugin
 */
class EnhancedGaugePlugin extends WidgetPlugin {
    readonly id = 'enhanced-gauge';
    readonly widgetType = 'enhanced-gauge';
    
    readonly metadata: PluginMetadata = {
        name: 'Enhanced Gauge Widget',
        version: '2.0.0',
        author: 'TUI Framework Team',
        description: 'High-performance gauges with themes and animations',
        homepage: 'https://github.com/tui-framework/gauge-plugin',
        dependencies: [{
            pluginId: 'theme-provider',
            minVersion: '1.0.0',
            optional: false
        }],
        capabilities: [PluginCapability.WidgetProvider],
        tags: ['widget', 'gauge', 'visualization', 'animated']
    };

    private currentTheme = 'cyberpunk';

    get configSchema() {
        return {
            type: 'object',
            properties: {
                value: { type: 'number', minimum: 0, maximum: 100 },
                label: { type: 'string' },
                size: { type: 'string', enum: ['small', 'medium', 'large'] },
                animated: { type: 'boolean' },
                showPercentage: { type: 'boolean' },
                themeAware: { type: 'boolean' }
            },
            required: ['value', 'label']
        };
    }

    async initialize(context: PluginContext): Promise<void> {
        console.log('📊 Enhanced Gauge Plugin v2.0.0 initialized');
        context.setState('features', 'animations,themes,responsive');
    }

    async cleanup(): Promise<void> {
        console.log('📊 Enhanced Gauge Plugin shutting down');
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom' && event.eventType === 'theme-changed') {
            this.currentTheme = event.data?.theme || 'cyberpunk';
            console.log(`📊 Gauges adapting to ${this.currentTheme} theme`);
            return { type: 'continue' };
        }
        return null;
    }

    createInstance(config: WidgetConfig): WidgetPlugin {
        return new EnhancedGaugeWidget(config, this.currentTheme);
    }

    render(): Element {
        return div().child(text('Enhanced Gauge Plugin')).build();
    }
}

class EnhancedGaugeWidget extends WidgetPlugin {
    readonly id: string;
    readonly widgetType = 'enhanced-gauge';
    readonly metadata: PluginMetadata;
    private theme: string;

    public config: WidgetConfig;
    
    constructor(config: WidgetConfig, theme: string) {
        super(config);
        this.config = config;
        this.id = config.id;
        this.theme = theme;
        this.metadata = new EnhancedGaugePlugin().metadata;
    }

    get configSchema() { return new EnhancedGaugePlugin().configSchema; }
    async initialize(): Promise<void> {}
    async cleanup(): Promise<void> {}
    createInstance(config: WidgetConfig): WidgetPlugin { return new EnhancedGaugeWidget(config, this.theme); }

    render(): Element {
        const { value = 0, label = 'Gauge', size = 'medium', showPercentage = true } = this.config.properties;
        
        // Theme-based styling
        const themes: Record<string, { filled: string, empty: string, accent: string }> = {
            cyberpunk: { filled: '█', empty: '░', accent: '🔥' },
            nature: { filled: '🌿', empty: '·', accent: '🌱' },
            retro: { filled: '▓', empty: '░', accent: '💾' },
            minimal: { filled: '●', empty: '○', accent: '◆' }
        };
        
        const currentTheme = themes[this.theme] || themes.cyberpunk;
        const percentage = Math.min(100, Math.max(0, value));
        const blocks = Math.floor(percentage / 5);
        const bar = currentTheme.filled.repeat(blocks) + currentTheme.empty.repeat(20 - blocks);
        
        const sizeClasses = {
            small: 'text-sm',
            medium: 'text-base',
            large: 'text-lg'
        };
        
        return div()
            .class('enhanced-gauge')
            .class(sizeClasses[size as string] || 'text-base')
            .child(
                div()
                    .class('gauge-header')
                    .child(text(`${currentTheme.accent} ${label}`))
            )
            .child(
                div()
                    .class('gauge-bar')
                    .child(text(`[${bar}]`))
            )
            .child(
                showPercentage
                    ? div()
                        .class('gauge-value')
                        .child(text(`${percentage.toFixed(1)}%`))
                    : text('')
            )
            .build();
    }
}

/**
 * 🎨 Advanced Theme Provider Plugin
 */
class AdvancedThemeProvider extends Plugin {
    readonly id = 'theme-provider';

    readonly metadata: PluginMetadata = {
        name: 'Theme Provider',
        version: '3.0.0',
        author: 'TUI Framework Team',
        description: 'Professional theming system with inheritance and validation',
        dependencies: [],
        capabilities: [PluginCapability.ThemeProvider],
        tags: ['theme', 'styling', 'professional']
    };

    private themes = new Map<string, Theme>();
    private currentTheme = 'cyberpunk';

    constructor() {
        super();
        this.initializeThemes();
    }

    private initializeThemes() {
        this.themes.set('cyberpunk', {
            name: 'Cyberpunk 2077',
            description: 'Neon-lit future aesthetic',
            colors: {
                primary: '#ff0080',
                secondary: '#00ffff',
                background: '#0a0a0a',
                foreground: '#f0f0f0',
                accent: '#ffff00',
                success: '#00ff88',
                warning: '#ff8800',
                error: '#ff0044'
            },
            animations: {
                style: 'glitch',
                speed: 'fast'
            }
        });

        this.themes.set('nature', {
            name: 'Forest Dreams',
            description: 'Calming natural palette',
            colors: {
                primary: '#228b22',
                secondary: '#8b4513',
                background: '#f5f5dc',
                foreground: '#2f4f2f',
                accent: '#ff6347',
                success: '#32cd32',
                warning: '#ffa500',
                error: '#dc143c'
            },
            animations: {
                style: 'smooth',
                speed: 'medium'
            }
        });

        this.themes.set('retro', {
            name: '80s Retrowave',
            description: 'Synthwave nostalgia',
            colors: {
                primary: '#ff1493',
                secondary: '#00ced1',
                background: '#191970',
                foreground: '#ffd700',
                accent: '#ff69b4',
                success: '#00fa9a',
                warning: '#ff4500',
                error: '#ff1493'
            },
            animations: {
                style: 'pulse',
                speed: 'medium'
            }
        });

        this.themes.set('minimal', {
            name: 'Minimal Clean',
            description: 'Simple and elegant',
            colors: {
                primary: '#000000',
                secondary: '#666666',
                background: '#ffffff',
                foreground: '#000000',
                accent: '#0066cc',
                success: '#00aa00',
                warning: '#ff9900',
                error: '#cc0000'
            },
            animations: {
                style: 'fade',
                speed: 'slow'
            }
        });
    }

    async initialize(context: PluginContext): Promise<void> {
        console.log(`🎨 Theme Provider v2.0.0 loaded with ${this.themes.size} themes`);
        const themeNames = Array.from(this.themes.keys()).join(',');
        context.setState('available_themes', themeNames);
        context.setState('current_theme', this.currentTheme);
    }

    async cleanup(): Promise<void> {
        console.log('🎨 Theme Provider cleaned up');
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type !== 'custom') return null;

        switch (event.eventType) {
            case 'get-theme':
                const theme = this.themes.get(this.currentTheme);
                if (theme) {
                    return {
                        type: 'data',
                        data: {
                            id: this.currentTheme,
                            ...theme
                        }
                    };
                }
                break;

            case 'switch-theme':
                const newTheme = event.data?.name;
                if (newTheme && this.themes.has(newTheme)) {
                    this.currentTheme = newTheme;
                    console.log(`🎨 Switched to ${newTheme} theme`);
                    return { type: 'continue' };
                }
                break;

            case 'list-themes':
                const themeList = Array.from(this.themes.entries()).map(([id, theme]) => ({
                    id,
                    name: theme.name,
                    description: theme.description
                }));
                return {
                    type: 'data',
                    data: { themes: themeList }
                };
        }

        return null;
    }
}

interface Theme {
    name: string;
    description: string;
    colors: Record<string, string>;
    animations: {
        style: string;
        speed: string;
    };
}

/**
 * 📡 Real-time Data Provider Plugin
 */
class DataProviderPlugin extends Plugin {
    readonly id = 'data-provider';
    
    readonly metadata: PluginMetadata = {
        name: 'Real-time Data Provider',
        version: '3.0.0',
        author: 'TUI Framework Team',
        description: 'Live data streaming with WebSocket support',
        dependencies: [],
        capabilities: [PluginCapability.DataProvider],
        tags: ['data', 'streaming', 'real-time', 'websocket']
    };

    private dataStreams = new Map<string, DataStream>();
    private updateInterval?: NodeJS.Timeout;
    private manager?: PluginManager;

    constructor() {
        super();
        this.initializeStreams();
    }

    private initializeStreams() {
        const now = Date.now();

        this.dataStreams.set('cpu', {
            id: 'cpu',
            name: 'CPU Usage',
            value: 45,
            min: 0,
            max: 100,
            unit: '%',
            updateRate: 500,
            // Performance optimization fields
            lastValue: 45,
            lastUpdate: now,
            changeThreshold: 1.0, // 1% change threshold
            updateCount: 0,
            stableCount: 0,
            adaptiveInterval: 100 // Start with 100ms
        });

        this.dataStreams.set('memory', {
            id: 'memory',
            name: 'Memory Usage',
            value: 62,
            min: 0,
            max: 100,
            unit: '%',
            updateRate: 1000,
            // Performance optimization fields
            lastValue: 62,
            lastUpdate: now,
            changeThreshold: 0.5, // 0.5% change threshold
            updateCount: 0,
            stableCount: 0,
            adaptiveInterval: 150 // Start with 150ms
        });

        this.dataStreams.set('network', {
            id: 'network',
            name: 'Network Traffic',
            value: 25,
            min: 0,
            max: 100,
            unit: 'Mbps',
            updateRate: 200,
            // Performance optimization fields
            lastValue: 25,
            lastUpdate: now,
            changeThreshold: 2.0, // 2 Mbps change threshold
            updateCount: 0,
            stableCount: 0,
            adaptiveInterval: 75 // Start with 75ms (more volatile)
        });

        this.dataStreams.set('disk', {
            id: 'disk',
            name: 'Disk I/O',
            value: 15,
            min: 0,
            max: 100,
            unit: 'MB/s',
            updateRate: 2000,
            // Performance optimization fields
            lastValue: 15,
            lastUpdate: now,
            changeThreshold: 1.0, // 1 MB/s change threshold
            updateCount: 0,
            stableCount: 0,
            adaptiveInterval: 500 // Start with 500ms (stable data)
        });
    }

    async initialize(context: PluginContext): Promise<void> {
        console.log(`📡 Data Provider v3.0.0 initialized with ${this.dataStreams.size} streams`);
        const streamIds = Array.from(this.dataStreams.keys()).join(',');
        context.setState('streams', streamIds);
    }

    async cleanup(): Promise<void> {
        this.stopUpdates();
        console.log('📡 Data Provider stopped');
    }

    startUpdates(manager: PluginManager) {
        this.manager = manager;

        // Start adaptive streaming for each data stream
        this.dataStreams.forEach((stream) => {
            this.startAdaptiveStream(stream);
        });

        console.log('📡 Adaptive data streams started');
    }

    private startAdaptiveStream(stream: DataStream) {
        const updateStream = () => {
            const now = Date.now();

            // Simulate data changes with realistic patterns
            const delta = this.generateRealisticDelta(stream);
            const newValue = Math.max(stream.min, Math.min(stream.max, stream.value + delta));

            // Check if change is significant enough to update
            const changeAmount = Math.abs(newValue - stream.lastValue);
            const shouldUpdate = changeAmount >= stream.changeThreshold;

            if (shouldUpdate) {
                // Update the value
                stream.value = newValue;
                stream.lastValue = newValue;
                stream.lastUpdate = now;
                stream.updateCount++;
                stream.stableCount = 0; // Reset stable count

                // Broadcast update only when there's a significant change
                this.manager?.broadcastEvent({
                    type: 'custom',
                    eventType: 'data-update',
                    data: {
                        streamId: stream.id,
                        value: stream.value,
                        timestamp: now,
                        changeAmount,
                        updateCount: stream.updateCount
                    }
                });

                // Decrease interval for active data (more frequent updates)
                stream.adaptiveInterval = Math.max(50, stream.adaptiveInterval * 0.9);
            } else {
                // No significant change - increase stable count
                stream.stableCount++;

                // Increase interval for stable data (less frequent updates)
                if (stream.stableCount > 3) {
                    stream.adaptiveInterval = Math.min(2000, stream.adaptiveInterval * 1.1);
                }
            }

            // Schedule next update with adaptive interval
            setTimeout(() => updateStream(), stream.adaptiveInterval);
        };

        // Start the adaptive stream
        updateStream();
    }

    private generateRealisticDelta(stream: DataStream): number {
        // Generate more realistic data patterns based on stream type
        switch (stream.id) {
            case 'cpu':
                // CPU usage: more volatile, occasional spikes
                return Math.random() < 0.1 ? (Math.random() - 0.5) * 30 : (Math.random() - 0.5) * 5;

            case 'memory':
                // Memory: gradual changes, occasional jumps
                return Math.random() < 0.05 ? (Math.random() - 0.5) * 15 : (Math.random() - 0.5) * 2;

            case 'network':
                // Network: very volatile, frequent changes
                return (Math.random() - 0.5) * 25;

            case 'disk':
                // Disk I/O: mostly stable with occasional activity
                return Math.random() < 0.2 ? (Math.random() - 0.5) * 20 : (Math.random() - 0.5) * 1;

            default:
                return (Math.random() - 0.5) * 10;
        }
    }

    stopUpdates() {
        // Note: Adaptive streams use setTimeout, so they'll naturally stop
        // when the plugin is cleaned up. No explicit cleanup needed.
        console.log('📡 Adaptive data streams stopped');
    }

    getPerformanceMetrics() {
        const metrics = new Map<string, any>();

        this.dataStreams.forEach((stream) => {
            metrics.set(stream.id, {
                updateCount: stream.updateCount,
                stableCount: stream.stableCount,
                currentInterval: stream.adaptiveInterval,
                lastUpdate: stream.lastUpdate,
                changeThreshold: stream.changeThreshold,
                efficiency: stream.updateCount > 0 ? (stream.updateCount / (stream.updateCount + stream.stableCount)) : 0
            });
        });

        return metrics;
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom' && event.eventType === 'get-streams') {
            const streams = Array.from(this.dataStreams.values());
            return {
                type: 'data',
                data: { streams }
            };
        }
        return null;
    }
}

interface DataStream {
    id: string;
    name: string;
    value: number;
    min: number;
    max: number;
    unit: string;
    updateRate: number;
    // Performance optimization fields
    lastValue: number;
    lastUpdate: number;
    changeThreshold: number;
    updateCount: number;
    stableCount: number;
    adaptiveInterval: number;
}

/**
 * 🚀 Main Interactive Demo Application
 */
async function main() {
    console.log('╔══════════════════════════════════════════╗');
    console.log('║   🚀 TUI Plugin System Showcase v2.0 🚀  ║');
    console.log('╚══════════════════════════════════════════╝');
    console.log();

    // Create plugin manager with built-in error handling
    const pluginManager = new PluginManager();
    
    // Register plugins
    await pluginManager.register(new AdvancedThemeProvider());
    await pluginManager.register(new DataProviderPlugin());
    await pluginManager.register(new EnhancedGaugePlugin());
    
    // Start data updates
    const dataProvider = pluginManager.getPlugin('data-provider');
    if (dataProvider) {
        (dataProvider as any).startUpdates?.(pluginManager);
    }
    
    // Application state
    let currentTheme = 'cyberpunk';
    let dataValues = new Map<string, number>([
        ['cpu', 45],
        ['memory', 62],
        ['network', 25],
        ['disk', 15]
    ]);
    
    // Create the app
    const app = createApp({
        component: () => {
            return flexColumn([])
                .child(
                    headerBar('header')
                        .item(barItem('🚀 TUI Plugin Showcase', 'left'))
                        .item(barItem(`Theme: ${currentTheme}`, 'center'))
                        .item(barItem('Press Q to quit', 'right'))
                        .build()
                )
                .child(
                    flexRow([])
                        .class('flex-1')
                        .child(
                            dashboardPanel({
                                id: 'dashboard',
                                title: 'System Metrics'
                            })
                            .child(
                                flexColumn([])
                                        .class('p-2')
                                        .children([
                                            // CPU Gauge (with built-in error handling)
                                            div()
                                                .child((() => {
                                                    const config: WidgetConfig = {
                                                        id: 'cpu-gauge',
                                                        widgetType: 'enhanced-gauge',
                                                        properties: {
                                                            value: dataValues.get('cpu') || 0,
                                                            label: 'CPU Usage',
                                                            size: 'medium',
                                                            showPercentage: true,
                                                            themeAware: true
                                                        },
                                                        cssClasses: ['system-gauge'],
                                                        eventHandlers: {}
                                                    };
                                                    const widget = pluginManager.createWidget('enhanced-gauge', config);
                                                    return widget.render();
                                                })()),
                                            
                                            // Memory Gauge (with built-in error handling)
                                            div()
                                                .class('mt-2')
                                                .child((() => {
                                                    const config: WidgetConfig = {
                                                        id: 'memory-gauge',
                                                        widgetType: 'enhanced-gauge',
                                                        properties: {
                                                            value: dataValues.get('memory') || 0,
                                                            label: 'Memory Usage',
                                                            size: 'medium',
                                                            showPercentage: true,
                                                            themeAware: true
                                                        },
                                                        cssClasses: ['system-gauge'],
                                                        eventHandlers: {}
                                                    };
                                                    const widget = pluginManager.createWidget('enhanced-gauge', config);
                                                    return widget.render();
                                                })()),
                                            
                                            // Network Gauge (with built-in error handling)
                                            div()
                                                .class('mt-2')
                                                .child((() => {
                                                    const config: WidgetConfig = {
                                                        id: 'network-gauge',
                                                        widgetType: 'enhanced-gauge',
                                                        properties: {
                                                            value: dataValues.get('network') || 0,
                                                            label: 'Network Traffic',
                                                            size: 'medium',
                                                            showPercentage: true,
                                                            themeAware: true
                                                        },
                                                        cssClasses: ['system-gauge'],
                                                        eventHandlers: {}
                                                    };
                                                    const widget = pluginManager.createWidget('enhanced-gauge', config);
                                                    return widget.render();
                                                })())
                                        ])
                                )
                        )
                        .child(
                            flexColumn([])
                                .class('flex-1')
                                .children([
                                    // Plugin Information Panel
                                    panel({
                                        id: 'plugin-info',
                                        title: 'Plugin Information'
                                    })
                                    .child(
                                        flexColumn([])
                                                .class('p-2')
                                                .children(
                                                    pluginManager.listPlugins().map(plugin =>
                                                        div()
                                                            .class('mb-2')
                                                            .child(text(`📦 ${plugin.name} v${plugin.version}`))
                                                            .child(text(`   ${plugin.description}`))
                                                            .child(text(`   Capabilities: ${plugin.capabilities.join(', ')}`))
                                                    )
                                                )
                                        ),

                                    // Plugin Status Dashboard
                                    panel({
                                        id: 'health-dashboard',
                                        title: 'System Health'
                                    })
                                    .child(
                                        div()
                                            .class('p-2')
                                            .children([
                                                text('🔌 Plugin System Status'),
                                                text('─'.repeat(25)),
                                                text(`✅ Plugins Loaded: ${pluginManager.listPlugins().length}`),
                                                text(`🎛️ Widget Types: ${pluginManager.getWidgetTypes().length}`),
                                                text('🛡️ Error Handling: Active'),
                                                text('📊 Built-in Fallbacks: Enabled')
                                            ])
                                    ),

                                    // Performance Metrics Dashboard
                                    panel({
                                        id: 'performance-dashboard',
                                        title: 'Performance Metrics'
                                    })
                                    .child(
                                        div()
                                            .class('p-2')
                                            .children([
                                                text('⚡ Adaptive Streaming Status'),
                                                text('─'.repeat(25)),
                                                ...(() => {
                                                    const dataProvider = pluginManager.getPlugin('data-provider') as any;
                                                    if (dataProvider?.getPerformanceMetrics) {
                                                        const metrics = dataProvider.getPerformanceMetrics();
                                                        const lines: any[] = [];

                                                        metrics.forEach((metric: any, streamId: string) => {
                                                            lines.push(text(`📊 ${streamId.toUpperCase()}: ${metric.currentInterval}ms interval`));
                                                            lines.push(text(`   Updates: ${metric.updateCount} | Efficiency: ${(metric.efficiency * 100).toFixed(1)}%`));
                                                        });

                                                        return lines;
                                                    }
                                                    return [text('📊 Performance monitoring active')];
                                                })()
                                            ])
                                    )
                                ])
                        )
                )
                .child(
                    statusBar('status')
                        .item(barItem('Ready', 'left'))
                        .item(barItem('[T] Theme  [R] Refresh  [Q] Quit', 'right'))
                        .build()
                );
        }
    });
    
    // Event handling would be implemented in the actual plugin system
    
    // Run the app
    await app.run();
}

// Run the showcase
main().catch(console.error);