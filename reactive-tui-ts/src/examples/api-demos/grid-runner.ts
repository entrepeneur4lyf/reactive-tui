#!/usr/bin/env bun
/**
 * Interactive Grid Runner - Real TUI Application
 * 
 * Demonstrates responsive grid layouts with different configurations.
 * Navigate through different grid examples with Tab/arrow keys.
 */

import { createApp } from '../../packages/tui-bun/src/app';
import { 
    grid,
    GridColumns,
    GridFlow,
    GridAlign,
    GRID_COLORS
} from '../../packages/tui-bun/src/widgets/grid';
import { div, text } from '../../packages/tui-bun/src/components';

let currentExample = 0;
const examples = [
    'Two Column Layout',
    'Three Column Layout', 
    'Grid with Spans',
    'Complex Layout',
    'Auto Grid'
];

function createGridDemo() {
    const exampleTitle = examples[currentExample] || 'Grid Demo';
    
    return div({ class: 'grid-demo', id: 'main' }).child(
            // Minimal header
            div({ class: 'header-bar' })
                .child(text(`${exampleTitle}`))
        )
        .child(text(''))
        .child(getCurrentExample());
}

function getCurrentExample() {
    switch (currentExample) {
        case 0:
            // Clean dashboard layout
            return grid({
                id: 'dashboard',
                columns: GridColumns.Three,
                gap: 1,
                items: [
                    {
                        id: 'nav',
                        content: 'Navigation\n\n→ Dashboard\n→ Analytics\n→ Reports\n→ Settings',
                        backgroundColor: GRID_COLORS.blue.backgroundColor
                    },
                    {
                        id: 'content',
                        content: 'Main Content\n\nWelcome to the dashboard\nClean, modern interface\nwith responsive grids',
                        colSpan: 2,
                        backgroundColor: GRID_COLORS.green.backgroundColor
                    }
                ]
            });
            
        case 1:
            // Three column layout
            return grid({
                id: 'three-column',
                columns: GridColumns.Three,
                gap: 1,
                items: [
                    {
                        id: 'sidebar',
                        content: 'Sidebar\n\nQuick Links:\n• Dashboard\n• Settings',
                        backgroundColor: GRID_COLORS.purple.backgroundColor
                    },
                    {
                        id: 'main-content',
                        content: 'Main Content\n\nThis is the primary\ncontent area where\nmost information\nis displayed.',
                        backgroundColor: GRID_COLORS.gray.backgroundColor
                    },
                    {
                        id: 'aside',
                        content: 'Aside Panel\n\nAdditional info:\n• News\n• Updates\n• Tips',
                        backgroundColor: GRID_COLORS.yellow.backgroundColor
                    }
                ]
            });
            
        case 2:
            // Grid with spans
            return grid({
                id: 'spans-grid',
                columns: GridColumns.Four,
                gap: 1,
                items: [
                    {
                        id: 'header',
                        content: 'Header (Spans 4 columns)',
                        colSpan: 4,
                        backgroundColor: GRID_COLORS.red.backgroundColor
                    },
                    {
                        id: 'nav',
                        content: 'Navigation',
                        backgroundColor: GRID_COLORS.blue.backgroundColor
                    },
                    {
                        id: 'main',
                        content: 'Main Content (Spans 2)',
                        colSpan: 2,
                        backgroundColor: GRID_COLORS.green.backgroundColor
                    },
                    {
                        id: 'sidebar',
                        content: 'Sidebar',
                        backgroundColor: GRID_COLORS.cyan.backgroundColor
                    },
                    {
                        id: 'footer',
                        content: 'Footer (Spans 4 columns)',
                        colSpan: 4,
                        backgroundColor: GRID_COLORS.pink.backgroundColor
                    }
                ]
            });
            
        case 3:
            // Complex layout with rows and spans
            return grid({
                id: 'complex-grid',
                columns: GridColumns.Three,
                rows: GridColumns.Four,
                gap: 1,
                items: [
                    {
                        id: 'header',
                        content: 'App Header',
                        colSpan: 3,
                        backgroundColor: GRID_COLORS.purple.backgroundColor
                    },
                    {
                        id: 'nav',
                        content: 'Navigation\n\n• Home\n• Products\n• Services',
                        rowSpan: 2,
                        backgroundColor: GRID_COLORS.blue.backgroundColor
                    },
                    {
                        id: 'content',
                        content: 'Main Content Area\n\nWelcome to our app!\nThis showcases complex\ngrid layouts.',
                        colSpan: 2,
                        backgroundColor: GRID_COLORS.green.backgroundColor
                    },
                    {
                        id: 'ads',
                        content: 'Advertisement\n\nSpecial Offer!\n50% off today',
                        backgroundColor: GRID_COLORS.yellow.backgroundColor
                    },
                    {
                        id: 'related',
                        content: 'Related Items\n\n• Item 1\n• Item 2\n• Item 3',
                        backgroundColor: GRID_COLORS.cyan.backgroundColor
                    },
                    {
                        id: 'footer',
                        content: 'Footer - Copyright 2024',
                        colSpan: 3,
                        backgroundColor: GRID_COLORS.gray.backgroundColor
                    }
                ]
            });
            
        case 4:
        default:
            // Auto grid with many items
            return grid({
                id: 'auto-grid',
                columns: GridColumns.Four,
                gap: 1,
                flow: GridFlow.Row,
                alignItems: GridAlign.Center,
                items: [
                    { id: 'item1', content: 'Card 1\n\nFeature A', backgroundColor: GRID_COLORS.red.backgroundColor },
                    { id: 'item2', content: 'Card 2\n\nFeature B', backgroundColor: GRID_COLORS.green.backgroundColor },
                    { id: 'item3', content: 'Card 3\n\nFeature C', backgroundColor: GRID_COLORS.blue.backgroundColor },
                    { id: 'item4', content: 'Card 4\n\nFeature D', backgroundColor: GRID_COLORS.yellow.backgroundColor },
                    { id: 'item5', content: 'Card 5\n\nFeature E', backgroundColor: GRID_COLORS.purple.backgroundColor },
                    { id: 'item6', content: 'Card 6\n\nFeature F', backgroundColor: GRID_COLORS.cyan.backgroundColor },
                    { id: 'item7', content: 'Card 7\n\nFeature G', backgroundColor: GRID_COLORS.pink.backgroundColor },
                    { id: 'item8', content: 'Card 8\n\nFeature H', backgroundColor: GRID_COLORS.gray.backgroundColor }
                ]
            });
    }
}

// Create and run the interactive app
const app = createApp({
    component: createGridDemo,
    width: 80,
    height: 24
});

// Override the button activation to cycle through examples
const originalRun = app.run.bind(app);
app.run = async function() {
    // Custom input handling for example switching
    process.stdin.on('data', (key) => {
        const keyStr = key.toString();
        if (keyStr === '\r' || keyStr === ' ') { // Enter or Space
            currentExample = (currentExample + 1) % examples.length;
            // Trigger re-render (in real implementation)
        }
    });
    
    return originalRun();
};

console.log('🏗️  Starting Interactive Grid Demo...');
console.log('Use Enter/Space to switch examples, arrow keys to navigate, q to quit');

app.run().catch(console.error);