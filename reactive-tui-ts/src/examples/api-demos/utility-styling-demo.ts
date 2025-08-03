#!/usr/bin/env bun

/**
 * Utility-First CSS Demo
 *
 * Demonstrates how all TUI framework widgets support utility-first CSS classes
 * for comprehensive styling and layout control.
 */

import {
    div,
    text,
    createSimpleCheckbox,
    createAnimatedCheckbox,
    createLoadingSpinner,
    linearProgress,
    SpinnerType
} from '../../packages/tui-bun/src';

function createUtilityDemo() {
    console.log('🎨 Creating utility-first CSS demo...');
    
    return div({ 
        classes: [
            'min-h-screen', 'bg-gray-50', 'py-8', 'px-4',
            'font-sans', 'antialiased'
        ] 
    }).child(
            div({ classes: ['max-w-4xl', 'mx-auto'] })
                .child(
                    text({ 
                        classes: [
                            'text-4xl', 'font-bold', 'text-center', 'mb-8',
                            'bg-gradient-to-r', 'from-blue-600', 'to-purple-600',
                            'bg-clip-text', 'text-transparent'
                        ],
                        content: '🎨 Utility-First Widget Styling'
                    })
                )
                .child(
                    text({ 
                        classes: [
                            'text-lg', 'text-gray-600', 'text-center', 'mb-12',
                            'max-w-2xl', 'mx-auto'
                        ],
                        content: 'All TUI framework widgets support utility-first CSS classes for complete styling control'
                    })
                )
                
                // Checkbox Section
                .child(
                    div({ classes: ['mb-12'] })
                        .child(
                            text({ 
                                classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                                content: '☑ Checkbox Components'
                            })
                        )
                        .child(
                            div({ classes: ['grid', 'grid-cols-1', 'md:grid-cols-2', 'gap-6'] })
                                .child(
                                    div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'shadow-md'] })
                                        .child(
                                            text({ 
                                                classes: ['font-medium', 'mb-4', 'text-gray-700'],
                                                content: 'Basic Styling'
                                            })
                                        )
                                        .child(
                                            createSimpleCheckbox({
                                                id: 'basic-1',
                                                label: 'Default checkbox',
                                                checked: true,
                                                classes: ['mb-3']
                                            })
                                        )
                                        .child(
                                            createSimpleCheckbox({
                                                id: 'basic-2',
                                                label: 'Styled checkbox',
                                                checked: false,
                                                classes: [
                                                    'text-blue-600', 'font-medium',
                                                    'hover:text-blue-800', 'transition-colors'
                                                ]
                                            })
                                        )
                                )
                                .child(
                                    div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'shadow-md'] })
                                        .child(
                                            text({ 
                                                classes: ['font-medium', 'mb-4', 'text-gray-700'],
                                                content: 'Animated Styling'
                                            })
                                        )
                                        .child(
                                            createAnimatedCheckbox({
                                                id: 'animated-1',
                                                label: 'Smooth animation',
                                                checked: true,
                                                classes: [
                                                    'text-green-600', 'font-semibold',
                                                    'transform', 'transition-all', 'duration-300',
                                                    'hover:scale-105'
                                                ],
                                                animationConfig: {
                                                    enabled: true,
                                                    duration: 300,
                                                    easing: 'ease-out',
                                                    scaleFactor: 1.5
                                                }
                                            })
                                        )
                                )
                        )
                )
                
                // Progress Section
                .child(
                    div({ classes: ['mb-12'] })
                        .child(
                            text({ 
                                classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                                content: '📊 Progress Components'
                            })
                        )
                        .child(
                            div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'shadow-md'] })
                                .child(
                                    text({ 
                                        classes: ['font-medium', 'mb-4', 'text-gray-700'],
                                        content: 'Styled Progress Bars'
                                    })
                                )
                                .child(
                                    div({ classes: ['space-y-4'] })
                                        .child(
                                            div({ classes: ['space-y-2'] })
                                                .child(
                                                    text({ 
                                                        classes: ['text-sm', 'font-medium', 'text-gray-600'],
                                                        content: 'Download Progress'
                                                    })
                                                )
                                                .child(
                                                    linearProgress({
                                                        id: 'progress-1',
                                                        value: 75,
                                                        max: 100,
                                                        label: 'Downloading...',
                                                        classes: [
                                                            'w-full', 'h-2', 'bg-gray-200', 'rounded-full',
                                                            'overflow-hidden', 'shadow-inner'
                                                        ]
                                                    })
                                                )
                                        )
                                        .child(
                                            div({ classes: ['space-y-2'] })
                                                .child(
                                                    text({ 
                                                        classes: ['text-sm', 'font-medium', 'text-gray-600'],
                                                        content: 'Upload Progress'
                                                    })
                                                )
                                                .child(
                                                    linearProgress({
                                                        id: 'progress-2',
                                                        value: 42,
                                                        max: 100,
                                                        label: 'Uploading...',
                                                        classes: [
                                                            'w-full', 'h-3', 'bg-gradient-to-r',
                                                            'from-purple-400', 'to-pink-400',
                                                            'rounded-lg', 'shadow-lg'
                                                        ]
                                                    })
                                                )
                                        )
                                )
                        )
                )
                
                // Spinner Section
                .child(
                    div({ classes: ['mb-12'] })
                        .child(
                            text({ 
                                classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                                content: '🔄 Spinner Components'
                            })
                        )
                        .child(
                            div({ classes: ['grid', 'grid-cols-1', 'md:grid-cols-3', 'gap-6'] })
                                .child(
                                    div({ 
                                        classes: [
                                            'bg-white', 'p-6', 'rounded-lg', 'shadow-md',
                                            'text-center', 'border-l-4', 'border-blue-500'
                                        ] 
                                    })
                                        .child(
                                            createLoadingSpinner({
                                                id: 'spinner-1',
                                                label: 'Loading...',
                                                type: SpinnerType.Dots,
                                                classes: [
                                                    'text-blue-500', 'text-lg', 'font-medium',
                                                    'animate-pulse'
                                                ]
                                            })
                                        )
                                )
                                .child(
                                    div({ 
                                        classes: [
                                            'bg-white', 'p-6', 'rounded-lg', 'shadow-md',
                                            'text-center', 'border-l-4', 'border-green-500'
                                        ] 
                                    })
                                        .child(
                                            createLoadingSpinner({
                                                id: 'spinner-2',
                                                label: 'Processing...',
                                                type: SpinnerType.Arc,
                                                classes: [
                                                    'text-green-500', 'text-lg', 'font-medium',
                                                    'transform', 'rotate-45'
                                                ]
                                            })
                                        )
                                )
                                .child(
                                    div({ 
                                        classes: [
                                            'bg-white', 'p-6', 'rounded-lg', 'shadow-md',
                                            'text-center', 'border-l-4', 'border-purple-500'
                                        ] 
                                    })
                                        .child(
                                            createLoadingSpinner({
                                                id: 'spinner-3',
                                                label: 'Saving...',
                                                type: SpinnerType.Line,
                                                classes: [
                                                    'text-purple-500', 'text-lg', 'font-medium',
                                                    'filter', 'drop-shadow-lg'
                                                ]
                                            })
                                        )
                                )
                        )
                )
                
                // Layout Section
                .child(
                    div({ classes: ['mb-12'] })
                        .child(
                            text({ 
                                classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                                content: '📐 Layout & Spacing'
                            })
                        )
                        .child(
                            div({ 
                                classes: [
                                    'bg-white', 'p-6', 'rounded-lg', 'shadow-md',
                                    'grid', 'grid-cols-1', 'md:grid-cols-2', 'gap-8'
                                ] 
                            })
                                .child(
                                    div({ classes: ['space-y-4'] })
                                        .child(
                                            text({ 
                                                classes: ['font-medium', 'text-gray-700', 'mb-3'],
                                                content: 'Flexbox Layout'
                                            })
                                        )
                                        .child(
                                            div({ 
                                                classes: [
                                                    'flex', 'items-center', 'justify-between',
                                                    'p-4', 'bg-gray-50', 'rounded-md'
                                                ] 
                                            })
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'flex-1',
                                                        label: 'Option 1',
                                                        checked: true,
                                                        classes: ['flex-shrink-0']
                                                    })
                                                )
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'flex-2',
                                                        label: 'Option 2',
                                                        checked: false,
                                                        classes: ['flex-shrink-0']
                                                    })
                                                )
                                        )
                                )
                                .child(
                                    div({ classes: ['space-y-4'] })
                                        .child(
                                            text({ 
                                                classes: ['font-medium', 'text-gray-700', 'mb-3'],
                                                content: 'Grid Layout'
                                            })
                                        )
                                        .child(
                                            div({ 
                                                classes: [
                                                    'grid', 'grid-cols-2', 'gap-3',
                                                    'p-4', 'bg-gray-50', 'rounded-md'
                                                ] 
                                            })
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'grid-1',
                                                        label: 'A',
                                                        checked: true,
                                                        classes: ['text-center']
                                                    })
                                                )
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'grid-2',
                                                        label: 'B',
                                                        checked: false,
                                                        classes: ['text-center']
                                                    })
                                                )
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'grid-3',
                                                        label: 'C',
                                                        checked: true,
                                                        classes: ['text-center']
                                                    })
                                                )
                                                .child(
                                                    createSimpleCheckbox({
                                                        id: 'grid-4',
                                                        label: 'D',
                                                        checked: false,
                                                        classes: ['text-center']
                                                    })
                                                )
                                        )
                                )
                        )
                )
                
                // Summary
                .child(
                    div({ 
                        classes: [
                            'bg-gradient-to-r', 'from-blue-50', 'to-purple-50',
                            'p-8', 'rounded-xl', 'shadow-lg', 'text-center'
                        ] 
                    })
                        .child(
                            text({ 
                                classes: [
                                    'text-2xl', 'font-bold', 'mb-4',
                                    'bg-gradient-to-r', 'from-blue-600', 'to-purple-600',
                                    'bg-clip-text', 'text-transparent'
                                ],
                                content: '✨ Utility-First CSS Complete!'
                            })
                        )
                        .child(
                            text({ 
                                classes: ['text-gray-600', 'text-lg'],
                                content: 'All widgets support comprehensive Tailwind-style utility classes for styling, layout, spacing, colors, animations, and more!'
                            })
                        )
                )
        );
}

// Create and display the demo
const demo = createUtilityDemo();

console.log('✨ Utility-first CSS demo created!');
console.log('🎨 Utility classes demonstrated:');
console.log('  • Layout: grid, flex, max-w, mx-auto, space-y, gap');
console.log('  • Spacing: p-4, p-6, p-8, mb-3, mb-4, mb-6, mb-12');
console.log('  • Colors: text-blue-600, bg-gray-50, border-purple-500');
console.log('  • Typography: text-2xl, font-bold, font-semibold');
console.log('  • Effects: shadow-md, shadow-lg, rounded-lg, gradient');
console.log('  • Animations: transition-all, hover:scale-105, animate-pulse');
console.log('  • Responsive: md:grid-cols-2, md:grid-cols-3');
console.log('');

// Output the component structure
console.log('📋 Tailwind Demo Component Structure:');
console.log(JSON.stringify(demo, null, 2));

console.log('\n🎨 Utility-First CSS Demo Complete!');
