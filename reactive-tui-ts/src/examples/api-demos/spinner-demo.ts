#!/usr/bin/env bun

/**
 * Spinner Widget Demo - TypeScript/Bun Implementation
 * 
 * Demonstrates the comprehensive spinner widget with various types,
 * animations, and styling options in TypeScript.
 */

import {
    createApp,
    div,
    text,
    spinnerWidget,
    createLoadingSpinner,
    createProcessingSpinner,
    createSavingSpinner,
    createCustomSpinner,
    createEmojiSpinner,
    createMinimalSpinner,
    createProgressSpinner,
    createBinarySpinner,
    SpinnerType,
    SpinnerLabelPosition,
    SpinnerAnimationState
} from '../../packages/tui-bun/src';

function createSpinnerDemo() {
    return div({ class: 'demo-container p-4' }).child(
            text('🔄 Spinner Widget Demo - TypeScript/Bun')
                .class('text-2xl font-bold mb-4 text-center')
        )
        
        // Basic Spinner Types Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Basic Spinner Types').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-3 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Dots Spinner:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'dots-spinner',
                                    type: SpinnerType.Dots,
                                    label: 'Loading...',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Arc Spinner:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'arc-spinner',
                                    type: SpinnerType.Arc,
                                    label: 'Processing...',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Line Spinner:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'line-spinner',
                                    type: SpinnerType.Line,
                                    label: 'Working...',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                )
                
                .child(
                    div({ class: 'grid grid-cols-3 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Circle Quarters:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'circle-quarters-spinner',
                                    type: SpinnerType.CircleQuarters,
                                    label: 'Analyzing...',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Triangle:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'triangle-spinner',
                                    type: SpinnerType.Triangle,
                                    label: 'Computing...',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Bouncing Bar:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'bouncing-bar-spinner',
                                    type: SpinnerType.BouncingBar,
                                    label: 'Progress',
                                    labelPosition: SpinnerLabelPosition.Before,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                )
        )
        
        // Emoji Spinners Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Emoji Spinners').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-4 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Hearts:').class('font-medium mb-2'))
                                .child(createEmojiSpinner({
                                    id: 'hearts-spinner',
                                    type: SpinnerType.Hearts,
                                    label: 'Love'
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Clock:').class('font-medium mb-2'))
                                .child(createEmojiSpinner({
                                    id: 'clock-spinner',
                                    type: SpinnerType.Clock,
                                    label: 'Time'
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Earth:').class('font-medium mb-2'))
                                .child(createEmojiSpinner({
                                    id: 'earth-spinner',
                                    type: SpinnerType.Earth,
                                    label: 'Global'
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Moon:').class('font-medium mb-2'))
                                .child(createEmojiSpinner({
                                    id: 'moon-spinner',
                                    type: SpinnerType.Moon,
                                    label: 'Phases'
                                }))
                        )
                )
        )
        
        // Convenience Functions Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Convenience Functions').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Loading Spinner:').class('font-medium mb-2'))
                                .child(createLoadingSpinner('loading-spinner', 'Fetching data...'))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Processing Spinner:').class('font-medium mb-2'))
                                .child(createProcessingSpinner('processing-spinner', 'Analyzing results...'))
                        )
                )
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Saving Spinner:').class('font-medium mb-2'))
                                .child(createSavingSpinner('saving-spinner', 'Saving changes...'))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Minimal Spinner:').class('font-medium mb-2'))
                                .child(createMinimalSpinner('minimal-spinner', SpinnerType.Dots2))
                        )
                )
        )
        
        // Custom Spinners Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Custom Spinners').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Progress Spinner:').class('font-medium mb-2'))
                                .child(createProgressSpinner('progress-spinner', 'Building project...'))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Binary Spinner:').class('font-medium mb-2'))
                                .child(createBinarySpinner('binary-spinner', 'Computing hash...'))
                        )
                )
                
                .child(
                    div({ class: 'custom-spinner-demo mb-4' })
                        .child(text('Custom Definition:').class('font-medium mb-2'))
                        .child(createCustomSpinner({
                            id: 'custom-spinner',
                            customDefinition: {
                                frames: ['🔴', '🟠', '🟡', '🟢', '🔵', '🟣'],
                                interval: 200,
                                name: 'rainbow'
                            },
                            label: 'Rainbow loading...',
                            labelPosition: SpinnerLabelPosition.After
                        }))
                )
        )
        
        // Label Positioning Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Label Positioning').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Label Before:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'before-spinner',
                                    type: SpinnerType.Star,
                                    label: 'Loading data',
                                    labelPosition: SpinnerLabelPosition.Before,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Label After:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'after-spinner',
                                    type: SpinnerType.Star,
                                    label: 'Loading data',
                                    labelPosition: SpinnerLabelPosition.After,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                )
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Label Above:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'above-spinner',
                                    type: SpinnerType.Circle,
                                    label: 'Loading data',
                                    labelPosition: SpinnerLabelPosition.Above,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                        .child(
                            div({ class: 'spinner-demo' })
                                .child(text('Label Below:').class('font-medium mb-2'))
                                .child(spinnerWidget({
                                    id: 'below-spinner',
                                    type: SpinnerType.Circle,
                                    label: 'Loading data',
                                    labelPosition: SpinnerLabelPosition.Below,
                                    animationState: SpinnerAnimationState.Running
                                }))
                        )
                )
        )
        
        // Summary
        .child(
            div({ class: 'summary text-center mt-6 p-4 bg-gray-100 rounded' })
                .child(text('✨ Spinner Demo Complete!').class('font-bold text-lg mb-2'))
                .child(
                    div({ class: 'features-list text-sm' })
                        .child(text('• 30+ predefined spinner types'))
                        .child(text('• Custom spinner definitions'))
                        .child(text('• Emoji and Unicode support'))
                        .child(text('• Flexible label positioning'))
                        .child(text('• Animation state control'))
                        .child(text('• Accessibility features (ARIA)'))
                        .child(text('• TypeScript type safety'))
                        .child(text('• Convenience functions'))
                )
        );
}

// Create and run the demo app
async function main() {
    console.log('🔄 Creating spinner demo...\n');
    
    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createSpinnerDemo()
        });

        console.log('✨ Spinner demo created successfully!');
        console.log('📋 Features demonstrated:');
        console.log('  • Basic spinner types (dots, arc, line, circle, triangle, etc.)');
        console.log('  • Emoji spinners (hearts, clock, earth, moon, weather)');
        console.log('  • Convenience functions (loading, processing, saving)');
        console.log('  • Custom spinner definitions');
        console.log('  • Label positioning (before, after, above, below)');
        console.log('  • Animation state control');
        console.log('  • Accessibility features');
        
        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createSpinnerDemo().build(), null, 2));

        console.log('\n✅ Spinner Demo Complete!');

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
