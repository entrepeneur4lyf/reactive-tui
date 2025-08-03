#!/usr/bin/env bun
/**
 * Plugin System Demo - TypeScript Version
 * 
 * Demonstrates the extensible plugin architecture with custom widgets,
 * event interception, and dynamic plugin loading/unloading.
 */

import { createApp, div, text } from '../../packages/tui-bun/src';
import { 
    PluginManager, 
    Plugin, 
    WidgetPlugin, 
    PluginContext,
    PluginEvent,
    PluginResponse,
    PluginCapability,
    type PluginMetadata,
    type WidgetConfig 
} from '../../packages/tui-bun/src/plugin';
import { panel } from '../../packages/tui-bun/src/widgets/panel';
import { bar, barItem } from '../../packages/tui-bun/src/widgets/bar';
import { grid } from '../../packages/tui-bun/src/widgets/grid';

/**
 * Custom gauge widget plugin
 */
class GaugeWidgetPlugin extends WidgetPlugin {
    readonly id = 'gauge-widget';
    readonly widgetType = 'gauge';
    
    readonly metadata: PluginMetadata = {
        name: 'Gauge Widget',
        version: '1.0.0',
        author: 'Demo Author',
        description: 'A circular gauge widget for displaying values',
        dependencies: [],
        capabilities: [PluginCapability.WidgetProvider],
        tags: ['widget', 'gauge', 'visualization'],
    };

    get configSchema() {
        return {
            type: 'object',
            properties: {
                value: { type: 'number', minimum: 0, maximum: 100 },
                label: { type: 'string' },
                color: { type: 'string' },
                size: { type: 'string', enum: ['small', 'medium', 'large'] },
            },
            required: ['value'],
        };
    }

    async initialize(context: PluginContext): Promise<void> {
        console.log('🔌 Gauge Widget Plugin initialized');
        context.setState('theme', 'default');
    }

    async cleanup(): Promise<void> {
        console.log('🔌 Gauge Widget Plugin cleaned up');
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom' && event.eventType === 'theme-change') {
            console.log('🎨 Gauge widget received theme change event');
            return { type: 'continue' };
        }
        return null;
    }

    createInstance(config: WidgetConfig): WidgetPlugin {
        return new GaugeWidgetPlugin(config);
    }

    render() {
        const { value = 0, label = 'Gauge', size = 'medium' } = this.config.properties;
        
        // Create a simple text-based gauge representation
        const percentage = Math.min(100, Math.max(0, value)) / 100 * 10;
        const filled = '█'.repeat(Math.floor(percentage));
        const empty = '░'.repeat(10 - Math.floor(percentage));
        
        return div()
            .class('gauge-widget')
            .classes(this.config.cssClasses)
            .child(text(`${label}: [${filled}${empty}] ${value.toFixed(0)}%`));
    }
}

/**
 * Event logger plugin that intercepts all events
 */
class EventLoggerPlugin extends Plugin {
    readonly id = 'event-logger';
    private eventCount = 0;
    
    readonly metadata: PluginMetadata = {
        name: 'Event Logger',
        version: '1.0.0',
        author: 'Demo Author',
        description: 'Logs all plugin events for debugging',
        dependencies: [],
        capabilities: [PluginCapability.EventInterceptor],
        tags: ['debug', 'logging'],
    };

    async initialize(context: PluginContext): Promise<void> {
        console.log('📝 Event Logger Plugin initialized');
    }

    async cleanup(): Promise<void> {
        console.log(`📝 Event Logger Plugin cleaned up. Total events logged: ${this.eventCount}`);
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        this.eventCount++;
        
        switch (event.type) {
            case 'plugin-loaded':
                console.log(`📝 [EVENT] Plugin loaded: ${event.pluginId}`);
                break;
            case 'plugin-unloading':
                console.log(`📝 [EVENT] Plugin unloading: ${event.pluginId}`);
                break;
            case 'widget-created':
                console.log(`📝 [EVENT] Widget created: ${event.widgetType} (${event.widgetId})`);
                break;
            case 'custom':
                console.log(`📝 [EVENT] Custom event: ${event.eventType}`);
                break;
        }
        
        return { type: 'continue' };
    }
}

/**
 * Theme provider plugin
 */
class ThemeProviderPlugin extends Plugin {
    readonly id = 'theme-provider';
    
    readonly metadata: PluginMetadata = {
        name: 'Theme Provider',
        version: '1.0.0',
        author: 'Demo Author',
        description: 'Provides themes for the application',
        dependencies: [],
        capabilities: [PluginCapability.ThemeProvider],
        tags: ['theme', 'styling'],
    };

    private themes = {
        light: {
            background: '#ffffff',
            foreground: '#000000',
            primary: '#0066cc',
        },
        dark: {
            background: '#1a1a1a',
            foreground: '#ffffff',
            primary: '#66b3ff',
        },
    };

    async initialize(context: PluginContext): Promise<void> {
        console.log('🎨 Theme Provider Plugin initialized');
        context.setState('available-themes', Object.keys(this.themes));
        context.setState('current-theme', 'light');
    }

    async cleanup(): Promise<void> {
        console.log('🎨 Theme Provider Plugin cleaned up');
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom' && event.eventType === 'get-theme') {
            const themeName = event.data?.theme || 'light';
            return {
                type: 'data',
                data: this.themes[themeName] || this.themes.light,
            };
        }
        return null;
    }
}

async function main() {
    console.log('🔌 Plugin System Demo');
    console.log('====================\n');
    
    // Create plugin manager
    const pluginManager = new PluginManager();
    
    // Register plugins
    await pluginManager.register(new EventLoggerPlugin());
    await pluginManager.register(new ThemeProviderPlugin());
    await pluginManager.register(new GaugeWidgetPlugin());
    
    // State
    let gaugeValues = [75, 42, 89];
    const plugins = pluginManager.listPlugins();
    
    // Create demo app
    const app = createApp({
        title: 'Plugin System Demo',
        component: () => {
            return div()
                .child(
                    grid()
                        .columns(['1fr', '2fr'])
                        .gap(2)
                        .child(
                            panel()
                                .title('Loaded Plugins')
                                .borderStyle('rounded')
                                .child(
                                    div()
                                        .style({ padding: '1rem' })
                                        .children(
                                            plugins.map(plugin => 
                                                div()
                                                    .style({ marginBottom: '1rem' })
                                                    .child(text(`📦 ${plugin.name}`))
                                                    .child(text(`   Version: ${plugin.version}`))
                                                    .child(text(`   Author: ${plugin.author}`))
                                                    .child(text(`   Capabilities: ${plugin.capabilities.join(', ')}`))
                                            )
                                        )
                                )
                        )
                        .child(
                            panel()
                                .title('Custom Widgets Demo')
                                .borderStyle('double')
                                .child(
                                    div()
                                        .style({ padding: '1rem' })
                                        .child(text('Gauge Widgets:'))
                                        .child(text(''))
                                        .children(
                                            gaugeValues.map((value, i) => {
                                                try {
                                                    const widget = pluginManager.createWidget('gauge', {
                                                        id: `gauge-${i}`,
                                                        widgetType: 'gauge',
                                                        properties: {
                                                            value,
                                                            label: `Metric ${i + 1}`,
                                                            size: 'medium',
                                                        },
                                                        cssClasses: ['gauge'],
                                                        eventHandlers: {},
                                                    });
                                                    return widget.render();
                                                } catch (error) {
                                                    return text('Failed to create gauge');
                                                }
                                            })
                                        )
                                        .child(text(''))
                                        .child(text('Plugin Features:'))
                                        .child(text('• Dynamic widget registration'))
                                        .child(text('• Event interception and logging'))
                                        .child(text('• Configuration validation'))
                                        .child(text('• Dependency resolution'))
                                        .child(text(''))
                                        .child(text('Available Widget Types:'))
                                        .children(
                                            pluginManager.getWidgetTypes().map(type =>
                                                text(`• ${type}`)
                                            )
                                        )
                                )
                        )
                )
                .child(
                    bar()
                        .position('bottom')
                        .items([
                            barItem('[R] Reload Plugins'),
                            barItem('[T] Trigger Theme Event'),
                            barItem('[U] Update Gauge Values'),
                            barItem('[L] List Widget Schemas'),
                            barItem('[Q] Quit'),
                        ])
                );
        },
        onKeyPress: async (key) => {
            switch (key) {
                case 'q':
                case 'Q':
                    // Cleanup plugins before exit
                    await pluginManager.unregister('event-logger');
                    await pluginManager.unregister('theme-provider');
                    await pluginManager.unregister('gauge-widget');
                    process.exit(0);
                    break;
                    
                case 'r':
                case 'R':
                    console.log('🔄 Reloading plugins...');
                    // In a real implementation, this would reload plugins
                    break;
                    
                case 't':
                case 'T':
                    // Trigger a custom theme change event
                    pluginManager.broadcastEvent({
                        type: 'custom',
                        eventType: 'theme-change',
                        data: { theme: 'dark' },
                    });
                    break;
                    
                case 'u':
                case 'U':
                    // Update gauge values randomly
                    gaugeValues = gaugeValues.map(() => Math.round(Math.random() * 100));
                    break;
                    
                case 'l':
                case 'L':
                    // List widget schemas
                    console.log('\n📋 Widget Schemas:');
                    for (const widgetType of pluginManager.getWidgetTypes()) {
                        const schema = pluginManager.getWidgetSchema(widgetType);
                        console.log(`\n${widgetType}:`);
                        console.log(JSON.stringify(schema, null, 2));
                    }
                    break;
            }
        },
    });
    
    // Run the app
    await app.run();
}

// Run the demo
main().catch(console.error);