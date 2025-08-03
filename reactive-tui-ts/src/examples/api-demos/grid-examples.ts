#!/usr/bin/env bun
/**
 * Grid Layout Examples - Bun/TypeScript Implementation
 *
 * Comprehensive examples demonstrating grid layout system:
 * - Colored panels with different layouts
 * - Different column configurations
 * - Grid items with custom styling and content
 * - Complex grid positioning and spanning
 */

import {
    grid,
    GridColumns
} from '../../packages/tui-bun/src/widgets/grid';

import { createApp } from '../../packages/tui-bun/src/app';
import { div, text } from '../../packages/tui-bun/src/components';

// Example 1: Basic Colored Grid (Reference Image Style)
console.log('=== Basic Colored Grid (Reference Image Style) ===\n');

// Create grid matching the reference image layout
const coloredGrid = grid({
    id: 'reference-grid',
    columns: GridColumns.Three,
    gap: 1,
    items: [
        { id: 'first', content: 'First column', column: 0, row: 0, backgroundColor: '#1e40af', textColor: '#ffffff' },
        { id: 'two', content: 'Two', column: 1, row: 0, backgroundColor: '#059669', textColor: '#ffffff' },
        { id: 'three', content: 'Three', column: 2, row: 0, backgroundColor: '#dc2626', textColor: '#ffffff' },
        { id: 'four', content: 'Four', column: 0, row: 1, backgroundColor: '#d97706', textColor: '#ffffff' },
        { id: 'five', content: 'Five', column: 1, row: 1, backgroundColor: '#7c3aed', textColor: '#ffffff' },
        { id: 'six', content: 'Six', column: 2, row: 1, backgroundColor: '#0891b2', textColor: '#ffffff' }
    ]
});

console.log('Basic Colored Grid:');
console.log(JSON.stringify(coloredGrid.build(), null, 2));
// Example 2: Advanced Grid Templates
console.log('\n=== Advanced Grid Templates ===\n');

// Application layout grid
const templateGrid = grid({
    id: 'template-grid',
    columns: GridColumns.Three,
    gap: 2,
    items: [
        {
            id: 'header',
            content: 'Application Header',
            column: 0,
            row: 0,
            colSpan: 3,
            backgroundColor: '#374151',
            textColor: '#ffffff'
        },
        {
            id: 'sidebar',
            content: 'Navigation\nMenu\nItems',
            column: 0,
            row: 1,
            backgroundColor: '#1e40af',
            textColor: '#ffffff'
        },
        {
            id: 'main',
            content: 'Main Content Area\n\nThis is where the primary content would be displayed.',
            column: 1,
            row: 1,
            backgroundColor: '#059669',
            textColor: '#ffffff'
        },
        {
            id: 'sidebar-right',
            content: 'Right Panel\n\nAdditional info',
            column: 2,
            row: 1,
            backgroundColor: '#d97706',
            textColor: '#ffffff'
        },
        {
            id: 'footer',
            content: 'Footer Content',
            column: 0,
            row: 2,
            colSpan: 3,
            backgroundColor: '#374151',
            textColor: '#ffffff'
        }
    ]
});

console.log('Advanced Template Grid:');
console.log(JSON.stringify(templateGrid.build(), null, 2));

// Example 3: Different Column Configurations
console.log('\n=== Different Column Configurations ===\n');

// Two column grid
const twoColumnGrid = grid({
    id: 'two-column-grid',
    columns: GridColumns.Two,
    gap: 1,
    items: [
        {
            id: 'left',
            content: 'Left Column\n\nContent here',
            column: 0,
            row: 0,
            backgroundColor: '#7c3aed',
            textColor: '#ffffff'
        },
        {
            id: 'right',
            content: 'Right Column\n\nMore content',
            column: 1,
            row: 0,
            backgroundColor: '#dc2626',
            textColor: '#ffffff'
        }
    ]
});

console.log('Two Column Grid:');
console.log(JSON.stringify(twoColumnGrid.build(), null, 2));

// Example 4: Four Column Grid
console.log('\n=== Four Column Grid ===\n');

const fourColumnGrid = grid({
    id: 'four-column-grid',
    columns: GridColumns.Four,
    gap: 1,
    items: [
        {
            id: 'col1',
            content: 'Column 1\n\nFirst quarter',
            column: 0,
            row: 0,
            backgroundColor: '#dc2626',
            textColor: '#ffffff'
        },
        {
            id: 'col2',
            content: 'Column 2\n\nSecond quarter',
            column: 1,
            row: 0,
            backgroundColor: '#059669',
            textColor: '#ffffff'
        },
        {
            id: 'col3',
            content: 'Column 3\n\nThird quarter',
            column: 2,
            row: 0,
            backgroundColor: '#1e40af',
            textColor: '#ffffff'
        },
        {
            id: 'col4',
            content: 'Column 4\n\nFourth quarter',
            column: 3,
            row: 0,
            backgroundColor: '#7c3aed',
            textColor: '#ffffff'
        }
    ]
});

console.log('Four Column Grid:');
console.log(JSON.stringify(fourColumnGrid.build(), null, 2));

// Example 5: Component Integration Demo
console.log('\n=== Component Integration Demo ===\n');

// Create a comprehensive demo component
function createGridDemo() {
    return div({ class: 'grid-demo', id: 'main' }).children([
            text('📐 Grid Layout Examples Demo', { class: 'title' }),

            div({ class: 'section', id: 'basic-section' })
                .children([
                    text('Basic Colored Grid:', { class: 'section-title' }),
                    coloredGrid
                ]),

            div({ class: 'section', id: 'template-section' })
                .children([
                    text('Advanced Template Grid:', { class: 'section-title' }),
                    templateGrid
                ]),

            div({ class: 'section', id: 'columns-section' })
                .children([
                    text('Different Column Configurations:', { class: 'section-title' }),
                    twoColumnGrid,
                    fourColumnGrid
                ])
        ]);
}

// Create and run the demo app
async function main() {
    console.log('🎯 Creating interactive grid demo...\n');

    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createGridDemo()
        });

        console.log('✨ Grid examples demo created successfully!');
        console.log('📐 Features demonstrated:');
        console.log('  • Basic colored grid layouts');
        console.log('  • Advanced template grids with spanning');
        console.log('  • Different column configurations (2, 3, 4 columns)');
        console.log('  • Complex grid positioning and styling');
        console.log('\n🎯 All grid widgets integrated into a demo application!');

        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createGridDemo().build(), null, 2));

        console.log('\n✅ Grid Examples Demo Complete!');

    } catch (error) {
        console.error('❌ Demo failed:', error);
        process.exit(1);
    }
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Demo interrupted by user');
    process.exit(0);
});

// Start the demonstration
main().catch(console.error);