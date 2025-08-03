#!/usr/bin/env bun
/**
 * Progress Bar Examples - Bun/TypeScript Implementation
 *
 * Comprehensive examples showing all progress bar variants:
 * - Linear progress bars with different styles
 * - Circular progress indicators
 * - Arc-style progress displays
 * - Animated spinners with various designs
 */

import {
    progress,
    ProgressStyle,
    ProgressState,
    ProgressBuilder,
    SPINNER_TYPES
} from '../../packages/tui-bun/src/widgets/progress';

import { createApp } from '../../packages/tui-bun/src/app';
import { div, text } from '../../packages/tui-bun/src/components';

// Example 1: Linear Progress Bars
console.log('=== Linear Progress Bar Examples ===\n');

// Basic linear progress bar
const basicProgress = progress({
    id: 'basic-linear',
    style: ProgressStyle.Linear,
    label: 'Download',
    value: 65,
    min: 0,
    max: 100,
    showPercentage: true,
    showValue: true
});

console.log('Basic Linear Progress:');
console.log(JSON.stringify(basicProgress.build(), null, 2));

// Custom styled linear progress
const styledProgress = ProgressBuilder.create()
    .id('styled-linear')
    .style(ProgressStyle.Linear)
    .label('Processing')
    .value(42)
    .range(0, 100)
    .showPercentage(true)
    .build();

console.log('\nStyled Linear Progress:');
console.log(JSON.stringify(styledProgress.build(), null, 2));

// Example 2: Circular Progress Indicators
console.log('\n=== Circular Progress Examples ===\n');

const circularProgress = progress({
    id: 'circular',
    style: ProgressStyle.Circular,
    label: 'Upload Progress',
    value: 75,
    min: 0,
    max: 100
});

console.log('Circular Progress:');
console.log(JSON.stringify(circularProgress.build(), null, 2));

// Smaller circular indicator
const smallCircular = ProgressBuilder.create()
    .id('small-circular')
    .style(ProgressStyle.Circular)
    .label('Sync')
    .value(30)
    .range(0, 100)
    .build();

console.log('\nSmall Circular Progress:');
console.log(JSON.stringify(smallCircular.build(), null, 2));

// Example 3: Arc Progress Displays
console.log('\n=== Arc Progress Examples ===\n');

const arcProgress = progress({
    id: 'arc',
    style: ProgressStyle.Arc,
    label: 'Loading',
    value: 55,
    min: 0,
    max: 100
});

console.log('Arc Progress:');
console.log(JSON.stringify(arcProgress.build(), null, 2));

// Wide arc progress
const wideArc = ProgressBuilder.create()
    .id('wide-arc')
    .style(ProgressStyle.Arc)
    .label('Build Progress')
    .value(88)
    .range(0, 100)
    .build();

console.log('\nWide Arc Progress:');
console.log(JSON.stringify(wideArc.build(), null, 2));

// Example 4: Animated Spinners
console.log('\n=== Spinner Examples ===\n');

// Various spinner types
const spinnerTypes = [
    SPINNER_TYPES.dots,
    SPINNER_TYPES.dots2,
    SPINNER_TYPES.line,
    SPINNER_TYPES.growVertical,
    SPINNER_TYPES.clock,
    SPINNER_TYPES.moon,
    SPINNER_TYPES.bouncingBar
];

for (const spinnerType of spinnerTypes) {
    const spinnerProgress = progress({
        id: `${spinnerType}-spinner`,
        style: ProgressStyle.Spinner,
        label: `${spinnerType} loading`,
        state: ProgressState.Indeterminate,
        spinnerType: spinnerType
    });

    console.log(`${spinnerType}:`);
    console.log(JSON.stringify(spinnerProgress.build(), null, 2));
}

// Example 5: Component Integration Demo
console.log('\n=== Component Integration Demo ===\n');
// Create a comprehensive demo component
function createProgressDemo() {
    return div({ class: 'progress-demo', id: 'main' }).children([
            text('🚀 Progress Bar Examples Demo', { class: 'title' }),

            div({ class: 'section', id: 'linear-section' })
                .children([
                    text('Linear Progress Bars:', { class: 'section-title' }),
                    basicProgress,
                    styledProgress
                ]),

            div({ class: 'section', id: 'circular-section' })
                .children([
                    text('Circular Progress Indicators:', { class: 'section-title' }),
                    circularProgress,
                    smallCircular
                ]),

            div({ class: 'section', id: 'arc-section' })
                .children([
                    text('Arc Progress Displays:', { class: 'section-title' }),
                    arcProgress,
                    wideArc
                ]),

            div({ class: 'section', id: 'spinner-section' })
                .children([
                    text('Animated Spinners:', { class: 'section-title' }),
                    ...spinnerTypes.map(type =>
                        progress({
                            id: `demo-${type}`,
                            style: ProgressStyle.Spinner,
                            label: `${type} spinner`,
                            state: ProgressState.Indeterminate,
                            spinnerType: type
                        })
                    )
                ])
        ]);
}

// Create and run the demo app
async function main() {
    console.log('🎯 Creating interactive progress demo...\n');

    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createProgressDemo()
        });

        console.log('✨ Progress examples demo created successfully!');
        console.log('📊 Features demonstrated:');
        console.log('  • Linear progress bars with different configurations');
        console.log('  • Circular progress indicators');
        console.log('  • Arc-style progress displays');
        console.log('  • Various animated spinners');
        console.log('\n🎯 All progress widgets integrated into a demo application!');

        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createProgressDemo().build(), null, 2));

        console.log('\n✅ Progress Examples Demo Complete!');

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