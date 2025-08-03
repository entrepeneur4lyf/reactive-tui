#!/usr/bin/env bun
/**
 * Combined Interactive Runner - Full TUI Application
 * 
 * Demonstrates all widgets together in a comprehensive dashboard.
 * Navigate between different sections and interact with various widgets.
 */

import { createApp } from '../../packages/tui-bun/src/app';
import { 
    slider, SliderOrientation, SliderMode
} from '../../packages/tui-bun/src/widgets/slider';
import { 
    progress, ProgressStyle
} from '../../packages/tui-bun/src/widgets/progress';
import { 
    grid, GridColumns, GRID_COLORS
} from '../../packages/tui-bun/src/widgets/grid';
import { div, text, button } from '../../packages/tui-bun/src/components';

let currentTab = 0;
const tabs = ['Dashboard', 'Controls', 'Settings', 'About'];

// State management
let systemHealth = 85;
let cpuUsage = 45;
let memoryUsage = 68;
let networkSpeed = 1200;
let volumeLevel = 75;
let brightnessLevel = 180;

function createMainApp() {
    const activeTab = tabs[currentTab];
    
    return div({ class: 'main-app', id: 'app' }).child(createHeader())
        .child(createTabBar())
        .child(createTabContent(activeTab))
        .child(createFooter());
}

function createHeader() {
    return div({ class: 'header' }).child(text('🚀 TUI Dashboard - Full Widget Demonstration'))
        .child(text(''));
}

function createTabBar() {
    const tabBar = div({ class: 'tab-bar' });
    
    tabs.forEach((tab, index) => {
        const isActive = index === currentTab;
        tabBar.child(button({ 
            text: isActive ? `[${tab}]` : ` ${tab} `,
            id: `tab-${index}`,
            class: isActive ? 'active-tab' : 'tab'
        }));
    });
    
    return tabBar.child(text(''));
}

function createTabContent(activeTab: string) {
    switch (activeTab) {
        case 'Dashboard':
            return createDashboardContent();
        case 'Controls':
            return createControlsContent();
        case 'Settings':
            return createSettingsContent();
        case 'About':
            return createAboutContent();
        default:
            return div().child(text('Unknown tab'));
    }
}

function createDashboardContent() {
    return div({ class: 'dashboard-content' }).child(text('📊 System Dashboard'))
        .child(text(''))
        
        // System metrics grid
        .child(grid({
            id: 'metrics-grid',
            columns: GridColumns.Two,
            gap: 2,
            items: [
                {
                    id: 'cpu-panel',
                    content: `CPU Usage\n${cpuUsage}%\n\nStatus: Normal\nCores: 8\nFreq: 3.2GHz`,
                    backgroundColor: cpuUsage > 80 ? GRID_COLORS.red.backgroundColor : GRID_COLORS.green.backgroundColor
                },
                {
                    id: 'memory-panel',
                    content: `Memory Usage\n${memoryUsage}%\n\nTotal: 16GB\nUsed: ${Math.round(16 * memoryUsage / 100)}GB\nFree: ${16 - Math.round(16 * memoryUsage / 100)}GB`,
                    backgroundColor: memoryUsage > 85 ? GRID_COLORS.yellow.backgroundColor : GRID_COLORS.blue.backgroundColor
                }
            ]
        }))
        .child(text(''))
        
        // Progress indicators
        .child(text('System Health:'))
        .child(progress({
            id: 'system-health',
            style: ProgressStyle.Linear,
            value: systemHealth,
            max: 100,
            label: 'Overall System Health',
            showPercentage: true
        }))
        .child(text(''))
        
        .child(text('Network Activity:'))
        .child(progress({
            id: 'network',
            style: ProgressStyle.Linear,
            value: networkSpeed,
            max: 2000,
            label: `Network Speed: ${networkSpeed} Mbps`,
            showValue: true
        }));
}

function createControlsContent() {
    return div({ class: 'controls-content' }).child(text('🎛️  System Controls'))
        .child(text('Use ←→ arrows to adjust values'))
        .child(text(''))
        
        .child(text('Audio Controls:'))
        .child(slider({
            id: 'volume-control',
            mode: SliderMode.Single,
            min: 0,
            max: 100,
            value: volumeLevel,
            orientation: SliderOrientation.Horizontal,
            label: `Volume: ${volumeLevel}%`
        }))
        .child(text(''))
        
        .child(text('Display Controls:'))
        .child(slider({
            id: 'brightness-control',
            mode: SliderMode.Single,
            min: 0,
            max: 255,
            value: brightnessLevel,
            orientation: SliderOrientation.Horizontal,
            label: `Brightness: ${brightnessLevel}`
        }))
        .child(text(''))
        
        .child(text('Temperature Control:'))
        .child(slider({
            id: 'temp-control',
            mode: SliderMode.Single,
            min: 15,
            max: 30,
            value: 22,
            step: 0.5,
            orientation: SliderOrientation.Horizontal,
            label: 'Room Temperature: 22°C'
        }))
        .child(text(''))
        
        .child(text('Quick Actions:'))
        .child(button({ text: 'Show Info Toast', id: 'info-toast-btn' }))
        .child(button({ text: 'Show Success Toast', id: 'success-toast-btn' }))
        .child(button({ text: 'Show Warning Toast', id: 'warning-toast-btn' }));
}

function createSettingsContent() {
    return div({ class: 'settings-content' }).child(text('⚙️  Application Settings'))
        .child(text(''))
        
        .child(grid({
            id: 'settings-grid',
            columns: GridColumns.One,
            gap: 1,
            items: [
                {
                    id: 'theme-setting',
                    content: 'Theme: Dark Mode ✓\nChange theme settings and appearance preferences',
                    backgroundColor: GRID_COLORS.purple.backgroundColor
                },
                {
                    id: 'notification-setting',
                    content: 'Notifications: Enabled ✓\nManage toast notifications and system alerts',
                    backgroundColor: GRID_COLORS.blue.backgroundColor
                },
                {
                    id: 'performance-setting',
                    content: 'Performance: High Performance Mode\nOptimize system resources and responsiveness',
                    backgroundColor: GRID_COLORS.green.backgroundColor
                }
            ]
        }))
        .child(text(''))
        
        .child(text('Configuration Progress:'))
        .child(progress({
            id: 'config-progress',
            style: ProgressStyle.Circular,
            value: 75,
            max: 100,
            label: 'Configuration Complete'
        }));
}

function createAboutContent() {
    return div({ class: 'about-content' }).child(text('ℹ️  About TUI Dashboard'))
        .child(text(''))
        .child(text('This demo showcases all TUI widgets:'))
        .child(text(''))
        .child(text('🎛️  Sliders - Interactive value controls'))
        .child(text('📊 Progress - Linear, circular, and arc progress'))
        .child(text('🏗️  Grids - Flexible layout system'))
        .child(text('🍞 Toasts - Notification system'))
        .child(text(''))
        .child(text('Built with:'))
        .child(text('• TypeScript + Bun runtime'))
        .child(text('• Rust TUI backend (simulated)'))
        .child(text('• CSS-inspired styling system'))
        .child(text('• Component-based architecture'))
        .child(text(''))
        .child(text('Navigation:'))
        .child(text('• Tab/Shift+Tab: Switch between widgets'))
        .child(text('• Arrow keys: Navigate and adjust'))
        .child(text('• Enter/Space: Activate buttons'))
        .child(text('• q: Quit application'));
}

function createFooter() {
    return div({ class: 'footer' }).child(text(''))
        .child(text('Tab: Switch widgets | ←→: Adjust | Enter: Activate | q: Quit'));
}

// Create and run the interactive app
const app = createApp({
    component: createMainApp,
    width: 80,  
    height: 24
});

// Override to add custom tab switching
const originalRun = app.run.bind(app);
app.run = async function() {
    // Custom input handling for tab switching and controls
    process.stdin.on('data', (key) => {
        const keyStr = key.toString();
        
        // Tab switching with number keys
        if (keyStr >= '1' && keyStr <= '4') {
            currentTab = parseInt(keyStr) - 1;
        }
        
        // Quick toast triggers
        switch (keyStr.toLowerCase()) {
            case 'i':
                // Trigger info toast (would be handled by toast system)
                break;
            case 's':
                // Trigger success toast
                break;
            case 'w':
                // Trigger warning toast
                break;
        }
    });
    
    return originalRun();
};

console.log('🚀 Starting Combined TUI Dashboard...');
console.log('Press 1-4 to switch tabs, arrow keys to navigate, q to quit');
console.log('This demo shows all widgets working together!');

app.run().catch(console.error);