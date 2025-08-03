/**
 * Modern Panel Demo - Clean, sophisticated layouts
 */

import { createApp, div, text, button, dashboardPanel, cardPanel, menuPanel } from '../../packages/tui-bun/src/index';

let currentExample = 0;
const examples = [
    'Dashboard Layout',
    'Card Layout', 
    'Menu Layout',
    'Mixed Layout'
];

function createPanelDemo() {
    const exampleTitle = examples[currentExample] || 'Panel Demo';
    
    return div({ class: 'panel-demo', id: 'main' }).child(
            // Clean minimal header
            div({ class: 'header' })
                .child(text(exampleTitle))
        )
        .child(text(''))
        .child(getCurrentExample())
        .child(text(''))
        .child(button({ text: 'Next Example →', id: 'next-example' }));
}

function getCurrentExample() {
    switch (currentExample) {
        case 0:
            // Dashboard layout with clean panels
            return div({ class: 'dashboard-layout' }).child(
                    dashboardPanel({
                        id: 'nav-panel',
                        content: '→ Dashboard\n→ Analytics\n→ Reports\n→ Settings'
                    }).build()
                )
                .child(
                    dashboardPanel({
                        id: 'main-panel', 
                        content: 'Welcome to the dashboard\n\nClean, modern interface\nwith responsive panels\n\nBuilt with CSS Grid System'
                    }).build()
                );
                
        case 1:
            // Card layout with shadows
            return div({ class: 'card-layout' }).child(
                    cardPanel({
                        id: 'feature-card',
                        content: 'Premium Feature\n\nAdvanced analytics\nReal-time updates\nCustom dashboards'
                    }).build()
                )
                .child(
                    cardPanel({
                        id: 'stats-card',
                        content: 'Performance Metrics\n\n↗ 25% growth\n👥 1.2K users\n📊 98% uptime'
                    }).build()
                );
                
        case 2:
            // Menu layout
            return div({ class: 'menu-layout' }).child(
                    menuPanel({
                        id: 'main-menu',
                        content: '1. File\n2. Edit\n3. View\n4. Tools\n5. Help'
                    }).build()
                )
                .child(
                    menuPanel({
                        id: 'context-menu',
                        content: '• Copy\n• Paste\n• Cut\n• Select All\n• Find'
                    }).build()
                );
                
        case 3:
        default:
            // Mixed layout
            return div({ class: 'mixed-layout' }).child(
                    dashboardPanel({
                        id: 'sidebar',
                        content: 'Quick Actions\n\n⚡ Deploy\n🔧 Configure\n📊 Monitor'
                    }).build()
                )
                .child(
                    cardPanel({
                        id: 'content',
                        content: 'Modern TUI Framework\n\nPowerful grid system\nResponsive design\nClean aesthetics'
                    }).build()
                )
                .child(
                    menuPanel({
                        id: 'tools',
                        content: '🔍 Search\n⚙️ Settings\n📝 Logs'
                    }).build()
                );
    }
}

// Create and run the interactive app
const app = createApp({
    component: createPanelDemo,
    width: process.stdout.columns || 80,
    height: process.stdout.rows || 24
});

// Panel demo ready to run

console.log('🎨 Starting Modern Panel Demo...');
console.log('Use Enter/Space to switch examples, arrow keys to navigate, q to quit');

app.run().catch(console.error);