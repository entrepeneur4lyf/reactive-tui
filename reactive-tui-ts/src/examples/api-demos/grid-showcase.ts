#!/usr/bin/env bun
/**
 * Grid Showcase - Functional API Examples
 * 
 * Demonstrates all grid layouts using the actual functional API.
 * Shows different column configurations and layouts.
 */

import { 
    grid,
    GridColumns,
    GridFlow,
    GridAlign,
    simpleGrid,
    autoGrid,
    createGrid,
    GRID_COLORS
} from '../../packages/tui-bun/src/widgets/grid';

async function main() {
    console.log('🏗️  Grid Showcase - TypeScript/Bun Implementation\n');
    
    // 1. Basic Grid Layouts
    console.log('📐 BASIC GRID LAYOUTS');
    console.log('=====================\n');
    
    // Simple two-column grid
    const twoColumnGrid = grid({
        id: 'two-column',
        columns: GridColumns.Two,
        items: [
            {
                id: 'left-panel',
                content: 'Left Panel',
                backgroundColor: GRID_COLORS.blue.backgroundColor
            },
            {
                id: 'right-panel',
                content: 'Right Panel',
                backgroundColor: GRID_COLORS.green.backgroundColor
            }
        ]
    });
    
    console.log('Two Column Grid:');
    console.log(JSON.stringify(twoColumnGrid.build(), null, 2));
    console.log();
    
    // Three-column grid
    const threeColumnGrid = grid({
        id: 'three-column',
        columns: GridColumns.Three,
        items: [
            {
                id: 'sidebar',
                content: 'Sidebar',
                backgroundColor: GRID_COLORS.purple.backgroundColor
            },
            {
                id: 'main-content',
                content: 'Main Content',
                backgroundColor: GRID_COLORS.gray.backgroundColor
            },
            {
                id: 'aside',
                content: 'Aside',
                backgroundColor: GRID_COLORS.yellow.backgroundColor
            }
        ]
    });
    
    console.log('Three Column Grid:');
    console.log(JSON.stringify(threeColumnGrid.build(), null, 2));
    console.log();
    
    // 2. Grid with Rows
    console.log('📏 GRID WITH ROWS');
    console.log('==================\n');
    
    const gridWithRows = grid({
        id: 'rows-grid',
        columns: GridColumns.Two,
        rows: GridColumns.Three,
        items: [
            {
                id: 'header',
                content: 'Header',
                column: 1,
                row: 1,
                colSpan: 2,
                backgroundColor: GRID_COLORS.red.backgroundColor
            },
            {
                id: 'sidebar',
                content: 'Sidebar',
                column: 1,
                row: 2,
                backgroundColor: GRID_COLORS.blue.backgroundColor
            },
            {
                id: 'content',
                content: 'Content',
                column: 2,
                row: 2,
                backgroundColor: GRID_COLORS.green.backgroundColor
            },
            {
                id: 'footer',
                content: 'Footer',
                column: 1,
                row: 3,
                colSpan: 2,
                backgroundColor: GRID_COLORS.gray.backgroundColor
            }
        ]
    });
    
    console.log('Grid with Rows:');
    console.log(JSON.stringify(gridWithRows.build(), null, 2));
    console.log();
    
    // 3. Convenience Functions
    console.log('🛠️  CONVENIENCE FUNCTIONS');
    console.log('==========================\n');
    
    const simpleLayout = simpleGrid(GridColumns.Four, GridColumns.Two, [
        {
            id: 'item1',
            content: 'Item 1',
            backgroundColor: GRID_COLORS.cyan.backgroundColor
        },
        {
            id: 'item2',
            content: 'Item 2',
            backgroundColor: GRID_COLORS.yellow.backgroundColor
        },
        {
            id: 'item3',
            content: 'Item 3',
            backgroundColor: GRID_COLORS.pink.backgroundColor
        },
        {
            id: 'item4',
            content: 'Item 4',
            backgroundColor: GRID_COLORS.purple.backgroundColor
        }
    ]);
    
    console.log('Simple Grid Layout:');
    console.log(JSON.stringify(simpleLayout.build(), null, 2));
    console.log();
    
    const autoLayout = autoGrid(GridColumns.Three, [
        {
            id: 'auto1',
            content: 'Auto Item 1',
            backgroundColor: GRID_COLORS.green.backgroundColor
        },
        {
            id: 'auto2',
            content: 'Auto Item 2',
            backgroundColor: GRID_COLORS.purple.backgroundColor
        },
        {
            id: 'auto3',
            content: 'Auto Item 3',
            backgroundColor: GRID_COLORS.yellow.backgroundColor
        },
        {
            id: 'auto4',
            content: 'Auto Item 4',
            backgroundColor: GRID_COLORS.cyan.backgroundColor
        },
        {
            id: 'auto5',
            content: 'Auto Item 5',
            backgroundColor: GRID_COLORS.pink.backgroundColor
        }
    ]);
    
    console.log('Auto Grid Layout:');
    console.log(JSON.stringify(autoLayout.build(), null, 2));
    console.log();
    
    // 4. Builder Pattern Examples
    console.log('🏗️  BUILDER PATTERN EXAMPLES');
    console.log('=============================\n');
    
    const builderExample = createGrid()
        .columns(GridColumns.Two)
        .gap(2)
        .alignItems(GridAlign.Center)
        .addItem({
            id: 'builder1',
            content: 'Builder Item 1',
            backgroundColor: GRID_COLORS.green.backgroundColor
        })
        .addItem({
            id: 'builder2',
            content: 'Builder Item 2',
            backgroundColor: GRID_COLORS.cyan.backgroundColor
        })
        .build();
    
    console.log('Builder Pattern Grid:');
    console.log(JSON.stringify(builderExample.build(), null, 2));
    console.log();
    
    // 5. Advanced Configurations
    console.log('⚙️  ADVANCED CONFIGURATIONS');
    console.log('============================\n');
    
    const configurations = [
        {
            name: 'Dashboard Layout',
            config: {
                id: 'dashboard',
                columns: GridColumns.Four,
                gap: 1,
                items: [
                    {
                        id: 'nav',
                        content: 'Navigation',
                        colSpan: 4,
                        backgroundColor: GRID_COLORS.gray.backgroundColor
                    },
                    {
                        id: 'sidebar',
                        content: 'Sidebar',
                        rowSpan: 2,
                        backgroundColor: GRID_COLORS.blue.backgroundColor
                    },
                    {
                        id: 'chart1',
                        content: 'Chart 1',
                        backgroundColor: GRID_COLORS.green.backgroundColor
                    },
                    {
                        id: 'chart2',
                        content: 'Chart 2',
                        backgroundColor: GRID_COLORS.yellow.backgroundColor
                    },
                    {
                        id: 'stats',
                        content: 'Statistics',
                        backgroundColor: GRID_COLORS.purple.backgroundColor
                    },
                    {
                        id: 'table',
                        content: 'Data Table',
                        colSpan: 2,
                        backgroundColor: GRID_COLORS.cyan.backgroundColor
                    },
                    {
                        id: 'activity',
                        content: 'Recent Activity',
                        backgroundColor: GRID_COLORS.yellow.backgroundColor
                    }
                ]
            }
        },
        {
            name: 'Card Layout',
            config: {
                id: 'cards',
                columns: GridColumns.Three,
                gap: 2,
                alignItems: GridAlign.Stretch,
                items: [
                    {
                        id: 'card1',
                        content: 'Card 1',
                        backgroundColor: GRID_COLORS.red.backgroundColor
                    },
                    {
                        id: 'card2',
                        content: 'Card 2',
                        backgroundColor: GRID_COLORS.blue.backgroundColor
                    },
                    {
                        id: 'card3',
                        content: 'Card 3',
                        backgroundColor: GRID_COLORS.green.backgroundColor
                    }
                ]
            }
        }
    ];
    
    configurations.forEach(({ name, config }) => {
        const gridWidget = grid(config);
        console.log(`${name}:`);
        console.log(JSON.stringify(gridWidget.build(), null, 2));
        console.log();
    });
    
    // 6. Complex Layout Examples
    console.log('🚀 COMPLEX LAYOUT EXAMPLES');
    console.log('===========================\n');
    
    const complexLayouts = [
        // Holy Grail Layout
        grid({
            id: 'holy-grail',
            columns: GridColumns.Three,
            rows: GridColumns.Three,
            items: [
                {
                    id: 'header',
                    content: 'Header',
                    column: 1,
                    row: 1,
                    colSpan: 3,
                    backgroundColor: GRID_COLORS.gray.backgroundColor
                },
                {
                    id: 'nav',
                    content: 'Navigation',
                    column: 1,
                    row: 2,
                    backgroundColor: GRID_COLORS.blue.backgroundColor
                },
                {
                    id: 'main',
                    content: 'Main Content',
                    column: 2,
                    row: 2,
                    backgroundColor: '#ffffff'
                },
                {
                    id: 'aside',
                    content: 'Sidebar',
                    column: 3,
                    row: 2,
                    backgroundColor: GRID_COLORS.green.backgroundColor
                },
                {
                    id: 'footer',
                    content: 'Footer',
                    column: 1,
                    row: 3,
                    colSpan: 3,
                    backgroundColor: GRID_COLORS.gray.backgroundColor
                }
            ]
        }),
        // Masonry-style Layout
        grid({
            id: 'masonry',
            columns: GridColumns.Four,
            flow: GridFlow.Dense,
            items: [
                {
                    id: 'tall1',
                    content: 'Tall Item',
                    rowSpan: 2,
                    backgroundColor: GRID_COLORS.purple.backgroundColor
                },
                {
                    id: 'wide1',
                    content: 'Wide Item',
                    colSpan: 2,
                    backgroundColor: GRID_COLORS.yellow.backgroundColor
                },
                {
                    id: 'normal1',
                    content: 'Normal',
                    backgroundColor: GRID_COLORS.cyan.backgroundColor
                },
                {
                    id: 'normal2',
                    content: 'Normal',
                    backgroundColor: GRID_COLORS.pink.backgroundColor
                },
                {
                    id: 'big1',
                    content: 'Big Item',
                    colSpan: 2,
                    rowSpan: 2,
                    backgroundColor: GRID_COLORS.red.backgroundColor
                },
                {
                    id: 'normal3',
                    content: 'Normal',
                    backgroundColor: GRID_COLORS.yellow.backgroundColor
                }
            ]
        })
    ];
    
    complexLayouts.forEach((layout, index) => {
        console.log(`Complex Layout ${index + 1}:`);
        console.log(JSON.stringify(layout.build(), null, 2));
        console.log();
    });
    
    console.log('🏗️  Grid Showcase Complete!');
    console.log('📝 All examples use the functional API with ElementBuilder.');
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Showcase interrupted by user');
    process.exit(0);
});

// Start the showcase
main().catch(console.error);