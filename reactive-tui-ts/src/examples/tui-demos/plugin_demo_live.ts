#!/usr/bin/env bun
/**
 * 🚀 Live Plugin System Demo
 * 
 * A working demonstration of the TUI plugin architecture
 */

import { createApp, div, text, flexRow, flexColumn } from '../../packages/tui-bun/src';
import { panel } from '../../packages/tui-bun/src/widgets/panel';
import { bar, barItem } from '../../packages/tui-bun/src/widgets/bar';

// Simulated plugin system types
interface Plugin {
    id: string;
    name: string;
    version: string;
    render(): any;
}

// CPU Monitor Plugin
class CPUMonitorPlugin implements Plugin {
    id = 'cpu-monitor';
    name = 'CPU Monitor';
    version = '1.0.0';
    private value = 45;
    
    constructor() {
        // Simulate CPU updates
        setInterval(() => {
            this.value = Math.max(0, Math.min(100, this.value + (Math.random() - 0.5) * 20));
        }, 500);
    }
    
    render() {
        const bars = Math.floor(this.value / 10);
        const gauge = '█'.repeat(bars) + '░'.repeat(10 - bars);
        
        return div()
            .class('cpu-monitor')
            .child(text(`🖥️  CPU Usage`))
            .child(text(`[${gauge}] ${this.value.toFixed(1)}%`))
            .build();
    }
}

// Memory Monitor Plugin
class MemoryMonitorPlugin implements Plugin {
    id = 'memory-monitor';
    name = 'Memory Monitor';
    version = '1.0.0';
    private value = 62;
    
    constructor() {
        // Simulate memory updates
        setInterval(() => {
            this.value = Math.max(0, Math.min(100, this.value + (Math.random() - 0.5) * 15));
        }, 1000);
    }
    
    render() {
        const bars = Math.floor(this.value / 10);
        const gauge = '▓'.repeat(bars) + '░'.repeat(10 - bars);
        
        return div()
            .class('memory-monitor')
            .child(text(`💾 Memory Usage`))
            .child(text(`[${gauge}] ${this.value.toFixed(1)}%`))
            .build();
    }
}

// Network Monitor Plugin
class NetworkMonitorPlugin implements Plugin {
    id = 'network-monitor';
    name = 'Network Monitor';
    version = '2.0.0';
    private download = 25;
    private upload = 15;
    
    constructor() {
        // Simulate network updates
        setInterval(() => {
            this.download = Math.max(0, Math.min(100, this.download + (Math.random() - 0.5) * 30));
            this.upload = Math.max(0, Math.min(100, this.upload + (Math.random() - 0.5) * 20));
        }, 200);
    }
    
    render() {
        const downBars = Math.floor(this.download / 10);
        const upBars = Math.floor(this.upload / 10);
        
        return div()
            .class('network-monitor')
            .child(text(`🌐 Network Traffic`))
            .child(text(`↓ [${downBars > 0 ? '▼'.repeat(downBars) : ' '.repeat(10)}] ${this.download.toFixed(0)} Mbps`))
            .child(text(`↑ [${upBars > 0 ? '▲'.repeat(upBars) : ' '.repeat(10)}] ${this.upload.toFixed(0)} Mbps`))
            .build();
    }
}

// Theme Plugin
class ThemePlugin implements Plugin {
    id = 'theme-provider';
    name = 'Theme Provider';
    version = '1.0.0';
    private currentTheme = 'cyberpunk';
    private themes = ['cyberpunk', 'nature', 'retro', 'minimal'];
    
    switchTheme() {
        const idx = this.themes.indexOf(this.currentTheme);
        this.currentTheme = this.themes[(idx + 1) % this.themes.length];
        return this.currentTheme;
    }
    
    render() {
        const themeEmojis: Record<string, string> = {
            cyberpunk: '🔥',
            nature: '🌿',
            retro: '💾',
            minimal: '◆'
        };
        
        return div()
            .class('theme-info')
            .child(text(`${themeEmojis[this.currentTheme]} Theme: ${this.currentTheme}`))
            .build();
    }
}

// Simple Plugin Manager
class PluginManager {
    private plugins = new Map<string, Plugin>();
    
    register(plugin: Plugin) {
        this.plugins.set(plugin.id, plugin);
        console.log(`📦 Registered plugin: ${plugin.name} v${plugin.version}`);
    }
    
    getPlugin(id: string): Plugin | undefined {
        return this.plugins.get(id);
    }
    
    getAllPlugins(): Plugin[] {
        return Array.from(this.plugins.values());
    }
}

// Main application
async function main() {
    console.log('╔══════════════════════════════════════════╗');
    console.log('║   🚀 Live Plugin System Demo 🚀          ║');
    console.log('╚══════════════════════════════════════════╝');
    console.log();
    
    // Create plugin manager and register plugins
    const pluginManager = new PluginManager();
    const cpuPlugin = new CPUMonitorPlugin();
    const memoryPlugin = new MemoryMonitorPlugin();
    const networkPlugin = new NetworkMonitorPlugin();
    const themePlugin = new ThemePlugin();
    
    pluginManager.register(cpuPlugin);
    pluginManager.register(memoryPlugin);
    pluginManager.register(networkPlugin);
    pluginManager.register(themePlugin);
    
    // Create the app
    const app = createApp({
        title: 'Plugin System Demo',
        component: () => {
            const plugins = pluginManager.getAllPlugins();
            
            return flexColumn()
                .child(
                    bar()
                        .item(barItem('🚀 Plugin System Demo', 'left'))
                        .item(barItem(themePlugin.render(), 'center'))
                        .item(barItem('[T] Theme [Q] Quit', 'right'))
                        .build()
                )
                .child(
                    flexRow()
                        .class('flex-1')
                        .child(
                            panel()
                                .title('System Monitors')
                                .class('flex-1')
                                .child(
                                    flexColumn()
                                        .class('p-2')
                                        .child(cpuPlugin.render())
                                        .child(div().class('mt-2').child(memoryPlugin.render()))
                                        .child(div().class('mt-2').child(networkPlugin.render()))
                                )
                        )
                        .child(
                            panel()
                                .title('Loaded Plugins')
                                .class('flex-1')
                                .child(
                                    flexColumn()
                                        .class('p-2')
                                        .children(
                                            plugins.map(plugin => 
                                                div()
                                                    .class('mb-2')
                                                    .child(text(`📦 ${plugin.name}`))
                                                    .child(text(`   Version: ${plugin.version}`))
                                                    .child(text(`   ID: ${plugin.id}`))
                                            )
                                        )
                                )
                        )
                );
        },
        onKeyPress: async (key) => {
            switch (key) {
                case 'q':
                case 'Q':
                    process.exit(0);
                    break;
                case 't':
                case 'T':
                    const newTheme = themePlugin.switchTheme();
                    console.log(`🎨 Switched to ${newTheme} theme`);
                    break;
            }
        }
    });
    
    // Add update loop for live updates
    setInterval(() => {
        app.forceUpdate();
    }, 100);
    
    await app.run();
}

// Run the demo
main().catch(console.error);