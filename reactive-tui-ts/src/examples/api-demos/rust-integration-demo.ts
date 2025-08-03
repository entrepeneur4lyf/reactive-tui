#!/usr/bin/env bun

/**
 * Rust Integration Demo
 * 
 * Demonstrates how TypeScript utility classes integrate with the Rust
 * utility CSS processor for terminal rendering.
 */

import {
    div,
    text,
    createSimpleCheckbox,
    createAnimatedCheckbox,
    linearProgress
} from '../../packages/tui-bun/src';

function createRustIntegrationDemo() {
    console.log('🔗 Creating Rust integration demo...');
    console.log('📋 This demo shows how TypeScript utility classes map to Rust ANSI processing\n');
    
    return div({ 
        classes: [
            // Layout utilities - processed by Rust UtilityProcessor
            'min-h-screen', 'bg-gray-50', 'p-8'
        ] 
    }).child(
            text({ 
                classes: [
                    // Typography utilities - converted to ANSI codes
                    'text-4xl', 'font-bold', 'text-center', 'mb-8',
                    // Color utilities - exact Tailwind colors in Rust
                    'text-blue-600'
                ],
                content: '🔗 TypeScript ↔ Rust Integration'
            })
        )
        .child(
            text({ 
                classes: ['text-lg', 'text-gray-600', 'text-center', 'mb-12'],
                content: 'Utility classes flow from TypeScript widgets to Rust ANSI processing'
            })
        )
        
        // Color Mapping Section
        .child(
            div({ classes: ['mb-12'] })
                .child(
                    text({ 
                        classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                        content: '🎨 Color Mapping Examples'
                    })
                )
                .child(
                    div({ classes: ['grid', 'grid-cols-1', 'md:grid-cols-2', 'gap-6'] })
                        .child(
                            div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'border'] })
                                .child(
                                    text({ 
                                        classes: ['font-medium', 'mb-4'],
                                        content: 'TypeScript → Rust Color Processing'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm', 'mb-2', 'font-mono'],
                                        content: 'TypeScript: classes: ["text-blue-600"]'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm', 'mb-2', 'font-mono'],
                                        content: 'Rust: hex("#2563eb") → \\x1B[38;2;37;99;235m'
                                    })
                                )
                                .child(
                                    createSimpleCheckbox({
                                        id: 'color-demo-1',
                                        label: 'Blue text example',
                                        checked: true,
                                        classes: ['text-blue-600', 'font-medium']
                                    })
                                )
                        )
                        .child(
                            div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'border'] })
                                .child(
                                    text({ 
                                        classes: ['font-medium', 'mb-4'],
                                        content: 'Background Color Processing'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm', 'mb-2', 'font-mono'],
                                        content: 'TypeScript: classes: ["bg-green-100"]'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm', 'mb-2', 'font-mono'],
                                        content: 'Rust: hex("#dcfce7") → \\x1B[48;2;220;252;231m'
                                    })
                                )
                                .child(
                                    createSimpleCheckbox({
                                        id: 'color-demo-2',
                                        label: 'Green background example',
                                        checked: false,
                                        classes: ['bg-green-100', 'p-2', 'rounded']
                                    })
                                )
                        )
                )
        )
        
        // Typography Integration
        .child(
            div({ classes: ['mb-12'] })
                .child(
                    text({ 
                        classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                        content: '📝 Typography Integration'
                    })
                )
                .child(
                    div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'border'] })
                        .child(
                            text({ 
                                classes: ['font-medium', 'mb-4'],
                                content: 'ANSI Typography Codes'
                            })
                        )
                        .child(
                            div({ classes: ['space-y-3'] })
                                .child(
                                    div({ classes: ['flex', 'justify-between', 'items-center'] })
                                        .child(
                                            text({ 
                                                classes: ['font-mono', 'text-sm'],
                                                content: 'font-bold → \\x1B[1m'
                                            })
                                        )
                                        .child(
                                            text({ 
                                                classes: ['font-bold'],
                                                content: 'Bold Text'
                                            })
                                        )
                                )
                                .child(
                                    div({ classes: ['flex', 'justify-between', 'items-center'] })
                                        .child(
                                            text({ 
                                                classes: ['font-mono', 'text-sm'],
                                                content: 'italic → \\x1B[3m'
                                            })
                                        )
                                        .child(
                                            text({ 
                                                classes: ['italic'],
                                                content: 'Italic Text'
                                            })
                                        )
                                )
                                .child(
                                    div({ classes: ['flex', 'justify-between', 'items-center'] })
                                        .child(
                                            text({ 
                                                classes: ['font-mono', 'text-sm'],
                                                content: 'underline → \\x1B[4m'
                                            })
                                        )
                                        .child(
                                            text({ 
                                                classes: ['underline'],
                                                content: 'Underlined Text'
                                            })
                                        )
                                )
                        )
                )
        )
        
        // Widget Integration Examples
        .child(
            div({ classes: ['mb-12'] })
                .child(
                    text({ 
                        classes: ['text-2xl', 'font-semibold', 'mb-6', 'text-gray-800'],
                        content: '🔧 Widget Integration Examples'
                    })
                )
                .child(
                    div({ classes: ['grid', 'grid-cols-1', 'md:grid-cols-2', 'gap-6'] })
                        .child(
                            div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'border'] })
                                .child(
                                    text({ 
                                        classes: ['font-medium', 'mb-4'],
                                        content: 'Animated Checkbox with Rust Processing'
                                    })
                                )
                                .child(
                                    createAnimatedCheckbox({
                                        id: 'rust-animated-checkbox',
                                        label: 'Rust-processed animations',
                                        checked: true,
                                        classes: [
                                            // These classes are processed by Rust UtilityProcessor
                                            'text-purple-600',    // → \x1B[38;2;124;58;237m
                                            'bg-purple-50',       // → \x1B[48;2;245;243;255m
                                            'p-3',                // → spacing: 3
                                            'rounded-md',         // → border style
                                            'font-semibold'       // → \x1B[1m
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
                        .child(
                            div({ classes: ['bg-white', 'p-6', 'rounded-lg', 'border'] })
                                .child(
                                    text({ 
                                        classes: ['font-medium', 'mb-4'],
                                        content: 'Progress Bar with Rust Colors'
                                    })
                                )
                                .child(
                                    linearProgress({
                                        id: 'rust-progress',
                                        value: 75,
                                        max: 100,
                                        label: 'Rust color processing...',
                                        classes: [
                                            // Exact Tailwind colors processed by Rust
                                            'bg-gradient-to-r',   // Layout utility
                                            'from-cyan-400',      // → hex("#22d3ee")
                                            'to-blue-500',        // → hex("#3b82f6")
                                            'h-3',                // → spacing: 3
                                            'rounded-full'        // → border style
                                        ]
                                    })
                                )
                        )
                )
        )
        
        // Integration Summary
        .child(
            div({ 
                classes: [
                    'bg-gradient-to-r', 'from-blue-50', 'to-purple-50',
                    'p-8', 'rounded-xl', 'border', 'text-center'
                ] 
            })
                .child(
                    text({ 
                        classes: ['text-2xl', 'font-bold', 'mb-4', 'text-gray-800'],
                        content: '🎯 Integration Summary'
                    })
                )
                .child(
                    div({ classes: ['grid', 'grid-cols-1', 'md:grid-cols-3', 'gap-6', 'text-left'] })
                        .child(
                            div({ classes: ['space-y-2'] })
                                .child(
                                    text({ 
                                        classes: ['font-semibold', 'text-blue-600'],
                                        content: '📝 TypeScript Layer'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Widget APIs accept classes[]'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Tailwind-style utility classes'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Type-safe configuration'
                                    })
                                )
                        )
                        .child(
                            div({ classes: ['space-y-2'] })
                                .child(
                                    text({ 
                                        classes: ['font-semibold', 'text-purple-600'],
                                        content: '🔗 Integration Layer'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Classes passed to Rust'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Exact color matching'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• State management'
                                    })
                                )
                        )
                        .child(
                            div({ classes: ['space-y-2'] })
                                .child(
                                    text({ 
                                        classes: ['font-semibold', 'text-green-600'],
                                        content: '⚡ Rust Layer'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• ANSI code generation'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Terminal rendering'
                                    })
                                )
                                .child(
                                    text({ 
                                        classes: ['text-sm'],
                                        content: '• Performance optimization'
                                    })
                                )
                        )
                )
        );
}

// Create and display the demo
const demo = createRustIntegrationDemo();

console.log('✨ Rust integration demo created!');
console.log('🔗 Integration features demonstrated:');
console.log('  • TypeScript utility classes → Rust ANSI processing');
console.log('  • Exact utility color matching between layers');
console.log('  • Typography and spacing integration');
console.log('  • Widget styling with Rust backend');
console.log('  • State management across TypeScript/Rust boundary');
console.log('');

// Output the component structure
console.log('📋 Rust Integration Demo Structure:');
console.log(JSON.stringify(demo, null, 2));

console.log('\n🎉 TypeScript ↔ Rust Integration Demo Complete!');
