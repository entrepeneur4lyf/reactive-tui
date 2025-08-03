#!/usr/bin/env bun
/**
 * Hot Reload CSS Demo - TypeScript Version
 * 
 * Demonstrates live CSS updates during development without restarting the application.
 * 
 * Instructions:
 * 1. Run this demo with: bun run examples/tui-demos/hot_reload_demo.ts
 * 2. Edit the CSS files in ./examples/styles/ while the demo is running
 * 3. Watch the UI update instantly when you save changes
 * 
 * This demo showcases:
 * - Live CSS updates without restart
 * - Error recovery with fallback CSS
 * - Debounced reloading for rapid changes
 * - Visual feedback during reload
 * - Multiple CSS file watching
 */

import { createApp, div, text, button } from '../../packages/tui-bun/src';
import { HotReloadBuilder, type HotReloadEventType } from '../../packages/tui-bun/src/widgets/hot_reload';
import { CssEngine } from '../../packages/tui-bun/src/css';
import { ReactiveState } from '../../packages/tui-bun/src/reactive';
import { writeFileSync, mkdirSync, existsSync } from 'fs';
import { join } from 'path';

// Sample CSS content that will be watched
const INITIAL_CSS = `
/* Main container styles */
.container {
    background-color: #1a1a2e;
    border: 2px solid #16213e;
    padding: 2rem;
    margin: 1rem;
}

/* Header styles */
.header {
    color: #eee;
    font-size: 1.5rem;
    font-weight: bold;
    text-align: center;
    margin-bottom: 1rem;
}

/* Status box styles */
.status-box {
    background-color: #0f3460;
    border: 1px solid #533483;
    padding: 1rem;
    margin: 0.5rem;
    border-radius: 4px;
}

/* Button styles */
.button {
    background-color: #533483;
    color: white;
    padding: 0.5rem 1rem;
    border: none;
    cursor: pointer;
    transition: background-color 0.3s;
}

.button:hover {
    background-color: #e94560;
}

/* Info text */
.info {
    color: #999;
    font-size: 0.9rem;
    font-style: italic;
}

/* Success message */
.success {
    color: #4caf50;
    font-weight: bold;
}

/* Error message */
.error {
    color: #f44336;
    font-weight: bold;
}
`;

// Alternative theme CSS files
const DARK_THEME_CSS = `
/* Dark theme */
.container {
    background-color: #000;
    border: 1px solid #333;
    color: #fff;
}

.header {
    color: #fff;
    text-shadow: 0 0 10px rgba(255, 255, 255, 0.5);
}

.status-box {
    background-color: #111;
    border: 1px solid #444;
}

.button {
    background-color: #333;
    border: 1px solid #555;
}

.button:hover {
    background-color: #555;
    box-shadow: 0 0 10px rgba(255, 255, 255, 0.3);
}
`;

const LIGHT_THEME_CSS = `
/* Light theme */
.container {
    background-color: #f5f5f5;
    border: 2px solid #ddd;
    color: #333;
}

.header {
    color: #2c3e50;
    border-bottom: 2px solid #3498db;
    padding-bottom: 0.5rem;
}

.status-box {
    background-color: #fff;
    border: 1px solid #e0e0e0;
    box-shadow: 0 2px 4px rgba(0, 0, 0, 0.1);
}

.button {
    background-color: #3498db;
    color: white;
    border-radius: 4px;
}

.button:hover {
    background-color: #2980b9;
    transform: translateY(-1px);
}

.info {
    color: #666;
}

.success {
    color: #27ae60;
}

.error {
    color: #e74c3c;
}
`;

const NEON_THEME_CSS = `
/* Neon cyberpunk theme */
.container {
    background-color: #0a0a0a;
    border: 2px solid #ff006e;
    box-shadow: 0 0 20px #ff006e;
    color: #fff;
}

.header {
    color: #00f5ff;
    text-shadow: 0 0 10px #00f5ff, 0 0 20px #00f5ff;
    animation: pulse 2s infinite;
}

@keyframes pulse {
    0% { opacity: 1; }
    50% { opacity: 0.8; }
    100% { opacity: 1; }
}

.status-box {
    background-color: rgba(255, 0, 110, 0.1);
    border: 1px solid #8b00ff;
    box-shadow: inset 0 0 10px rgba(139, 0, 255, 0.3);
}

.button {
    background: linear-gradient(45deg, #ff006e, #8b00ff);
    color: white;
    border: none;
    text-transform: uppercase;
    letter-spacing: 2px;
}

.button:hover {
    background: linear-gradient(45deg, #8b00ff, #ff006e);
    box-shadow: 0 0 20px #8b00ff;
    transform: scale(1.05);
}

.info {
    color: #fb5607;
    text-shadow: 0 0 5px #fb5607;
}

.success {
    color: #00f5ff;
    text-shadow: 0 0 10px #00f5ff;
}

.error {
    color: #ff006e;
    text-shadow: 0 0 10px #ff006e;
}
`;

async function main() {
    console.log('🔥 Hot Reload CSS Demo');
    console.log('====================\n');
    
    // Create styles directory if it doesn't exist
    const stylesDir = './examples/styles';
    if (!existsSync(stylesDir)) {
        mkdirSync(stylesDir, { recursive: true });
    }
    
    // Write initial CSS file
    const cssPath = join(stylesDir, 'demo.css');
    writeFileSync(cssPath, INITIAL_CSS);
    
    // Write theme files
    writeFileSync(join(stylesDir, 'dark-theme.css'), DARK_THEME_CSS);
    writeFileSync(join(stylesDir, 'light-theme.css'), LIGHT_THEME_CSS);
    writeFileSync(join(stylesDir, 'neon-theme.css'), NEON_THEME_CSS);
    
    console.log('📝 CSS files created in:', stylesDir);
    console.log('✏️  Edit any CSS file to see live updates!');
    console.log();
    
    // Create CSS engine
    const cssEngine = new CssEngine();
    
    // Parse and apply initial CSS
    const stylesheet = cssEngine.parse(INITIAL_CSS);
    cssEngine.applyStylesheet(stylesheet);
    
    // Create hot reload manager
    const hotReload = new HotReloadBuilder()
        .watchPaths([stylesDir])
        .extensions(['.css'])
        .debounce(100)
        .validate(true)
        .maxRetries(3)
        .verbose(true)
        .cssEngine(cssEngine)
        .build();
    
    // Start hot reload
    await hotReload.start();
    
    // Create reactive state for UI
    const reloadStatus = new ReactiveState('Watching for changes...');
    const currentTheme = new ReactiveState('demo.css');
    const eventLog = new ReactiveState<string[]>([]);
    
    // Subscribe to hot reload events
    hotReload.on('css-changed', (event: any) => {
        reloadStatus.value = `Reloading CSS from: ${event.path}`;
        addToEventLog(`🔄 CSS changed: ${event.path}`);
    });
    
    hotReload.on('reload-success', (event: any) => {
        reloadStatus.value = `✅ Successfully reloaded: ${event.path}`;
        currentTheme.value = event.path.split('/').pop() || 'unknown';
        addToEventLog(`✅ Reload success: ${event.path}`);
    });
    
    hotReload.on('reload-error', (event: any) => {
        reloadStatus.value = `❌ Failed to reload: ${event.error}`;
        addToEventLog(`❌ Reload error: ${event.error}`);
    });
    
    hotReload.on('watcher-error', (event: any) => {
        reloadStatus.value = `⚠️ Watcher error: ${event.error}`;
        addToEventLog(`⚠️ Watcher error: ${event.error}`);
    });
    
    function addToEventLog(message: string) {
        const log = eventLog.value;
        log.push(`[${new Date().toLocaleTimeString()}] ${message}`);
        // Keep only last 10 events
        if (log.length > 10) {
            log.shift();
        }
        eventLog.value = [...log];
    }
    
    // Theme switcher function
    async function switchTheme(themeName: string) {
        const themePath = join(stylesDir, themeName);
        await hotReload.reloadCss(themePath);
    }
    
    // Create demo UI
    const app = createApp({
        title: 'Hot Reload CSS Demo',
        component: () => {
            const stats = hotReload.getStats();
            
            return div()
                .class('container')
                .child(
                    div()
                        .class('header')
                        .child(text('🔥 Hot Reload CSS Demo'))
                )
                .child(
                    div()
                        .class('status-box')
                        .child(
                            div()
                                .class(stats.isReloading ? 'info' : 'success')
                                .child(text(`Status: ${reloadStatus.value}`))
                        )
                        .child(
                            div()
                                .class('info')
                                .child(text(`Reload Count: ${stats.reloadCount}`))
                        )
                        .child(
                            div()
                                .class('info')
                                .child(text(`Current Theme: ${currentTheme.value}`))
                        )
                        .child(
                            div()
                                .class('info')
                                .child(text(`Watched Paths: ${stats.watchedPaths.join(', ')}`))
                        )
                )
                .child(
                    div()
                        .class('status-box')
                        .child(
                            div()
                                .class('info')
                                .child(text('Quick Theme Switcher:'))
                        )
                        .child(
                            div()
                                .style({ display: 'flex', gap: '0.5rem', marginTop: '0.5rem' })
                                .child(
                                    button()
                                        .class('button')
                                        .text('Default')
                                        .onClick(() => switchTheme('demo.css'))
                                )
                                .child(
                                    button()
                                        .class('button')
                                        .text('Dark')
                                        .onClick(() => switchTheme('dark-theme.css'))
                                )
                                .child(
                                    button()
                                        .class('button')
                                        .text('Light')
                                        .onClick(() => switchTheme('light-theme.css'))
                                )
                                .child(
                                    button()
                                        .class('button')
                                        .text('Neon')
                                        .onClick(() => switchTheme('neon-theme.css'))
                                )
                        )
                )
                .child(
                    div()
                        .class('status-box')
                        .child(
                            div()
                                .class('info')
                                .child(text('Instructions:'))
                        )
                        .child(
                            div()
                                .child(text(`1. Open ${stylesDir}/demo.css in your editor`))
                        )
                        .child(
                            div()
                                .child(text('2. Modify any CSS property (colors, padding, etc.)'))
                        )
                        .child(
                            div()
                                .child(text('3. Save the file and watch the UI update instantly!'))
                        )
                )
                .child(
                    div()
                        .class('status-box')
                        .child(
                            div()
                                .class('info')
                                .child(text('Try these changes:'))
                        )
                        .child(
                            div()
                                .child(text('• Change .container background-color to #2d3436'))
                        )
                        .child(
                            div()
                                .child(text('• Modify .header color to #74b9ff'))
                        )
                        .child(
                            div()
                                .child(text('• Update .status-box border to 2px dashed #a29bfe'))
                        )
                        .child(
                            div()
                                .child(text('• Add text-shadow: 2px 2px 4px rgba(0,0,0,0.5) to .header'))
                        )
                )
                .child(
                    div()
                        .class('status-box')
                        .child(
                            div()
                                .class('info')
                                .child(text('Event Log:'))
                        )
                        .child(
                            div()
                                .style({ fontSize: '0.8rem', fontFamily: 'monospace' })
                                .children(
                                    eventLog.value.map(log => 
                                        div()
                                            .style({ marginTop: '0.25rem' })
                                            .child(text(log))
                                    )
                                )
                        )
                );
        },
        onExit: () => {
            // Stop hot reload on exit
            hotReload.stop();
            console.log('\n👋 Hot reload stopped');
        }
    });
    
    // Run the app
    await app.run();
}

// Run the demo
main().catch(console.error);