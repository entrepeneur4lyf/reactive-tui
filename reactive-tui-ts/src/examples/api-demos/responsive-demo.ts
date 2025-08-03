#!/usr/bin/env bun

/**
 * Responsive Design Demo
 * 
 * Demonstrates how TUI components adapt to different terminal sizes:
 * - Narrow terminals (< 60 cols): Single column, compact spacing
 * - Medium terminals (60-99 cols): Two columns, balanced spacing  
 * - Wide terminals (≥ 100 cols): Three columns, generous spacing
 * 
 * Resize your terminal to see the responsive behavior in action!
 */

import { 
    div, 
    text, 
    checkboxWidget,
    linearProgress,
    slider
} from '../../packages/tui-bun/src';

// Terminal size detection
const terminalWidth = process.stdout.columns || 80;
const terminalHeight = process.stdout.rows || 24;

// Responsive breakpoints
const isNarrow = terminalWidth < 60;
const isMedium = terminalWidth >= 60 && terminalWidth < 100;
const isWide = terminalWidth >= 100;

// Responsive layout configuration
const getResponsiveLayout = () => {
    if (isNarrow) {
        return {
            name: 'Narrow',
            description: 'Single column, compact spacing',
            container: ['p-2', 'space-y-2'],
            title: ['text-lg', 'font-bold', 'mb-2'],
            grid: ['grid-cols-1', 'gap-2'],
            card: ['p-2', 'mb-2', 'bg-gray-800', 'rounded'],
            text: ['text-sm'],
            widget: ['mb-2']
        };
    } else if (isMedium) {
        return {
            name: 'Medium',
            description: 'Two columns, balanced spacing',
            container: ['p-4', 'space-y-4'],
            title: ['text-xl', 'font-bold', 'mb-3'],
            grid: ['grid-cols-2', 'gap-4'],
            card: ['p-3', 'mb-3', 'bg-gray-800', 'rounded-lg'],
            text: ['text-base'],
            widget: ['mb-3']
        };
    } else {
        return {
            name: 'Wide',
            description: 'Three columns, generous spacing',
            container: ['p-6', 'space-y-6'],
            title: ['text-2xl', 'font-bold', 'mb-4'],
            grid: ['grid-cols-3', 'gap-6'],
            card: ['p-4', 'mb-4', 'bg-gray-800', 'rounded-xl'],
            text: ['text-lg'],
            widget: ['mb-4']
        };
    }
};

const layout = getResponsiveLayout();

function createResponsiveDemo() {
    return div({ classes: ['min-h-screen', 'bg-gray-900', 'text-white', ...layout.container] }).child(
            // Header with terminal info
            div({ classes: ['text-center', 'mb-6'] })
                .child(
                    text({ 
                        classes: [...layout.title, 'text-cyan-400'],
                        content: '📱 Responsive TUI Demo'
                    })
                )
                .child(
                    text({ 
                        classes: ['text-gray-400', ...layout.text],
                        content: `Terminal: ${terminalWidth} × ${terminalHeight} columns`
                    })
                )
                .child(
                    text({ 
                        classes: ['text-green-400', 'font-semibold', ...layout.text],
                        content: `Layout: ${layout.name} (${layout.description})`
                    })
                )
        )
        .child(
            // Responsive grid of widgets
            div({ classes: ['grid', ...layout.grid] })
                .child(
                    div({ classes: [...layout.card] })
                        .child(
                            text({ 
                                classes: ['font-semibold', 'mb-2', 'text-blue-400', ...layout.text],
                                content: '⚙️ Settings'
                            })
                        )
                        .child(
                            checkboxWidget({
                                id: 'notifications',
                                label: 'Enable notifications',
                                checked: true,
                                classes: [...layout.widget]
                            })
                        )
                        .child(
                            checkboxWidget({
                                id: 'darkmode',
                                label: 'Dark mode',
                                checked: false,
                                classes: [...layout.widget]
                            })
                        )
                )
                .child(
                    div({ classes: [...layout.card] })
                        .child(
                            text({ 
                                classes: ['font-semibold', 'mb-2', 'text-green-400', ...layout.text],
                                content: '📊 Progress'
                            })
                        )
                        .child(
                            text({ 
                                classes: ['mb-1', ...layout.text],
                                content: 'Download: 75%'
                            })
                        )
                        .child(
                            linearProgress({
                                id: 'download',
                                value: 75,
                                max: 100,
                                classes: [...layout.widget]
                            })
                        )
                        .child(
                            text({ 
                                classes: ['mb-1', ...layout.text],
                                content: 'Upload: 45%'
                            })
                        )
                        .child(
                            linearProgress({
                                id: 'upload',
                                value: 45,
                                max: 100,
                                classes: [...layout.widget]
                            })
                        )
                )
                .child(
                    div({ classes: [...layout.card] })
                        .child(
                            text({ 
                                classes: ['font-semibold', 'mb-2', 'text-purple-400', ...layout.text],
                                content: '🎛️ Controls'
                            })
                        )
                        .child(
                            text({ 
                                classes: ['mb-1', ...layout.text],
                                content: 'Volume: 80%'
                            })
                        )
                        .child(
                            slider({
                                id: 'volume',
                                value: 80,
                                min: 0,
                                max: 100,
                                classes: [...layout.widget]
                            })
                        )
                )
        )
        .child(
            // Responsive breakpoint info
            div({ classes: ['mt-8', 'text-center', 'border-t', 'border-gray-700', 'pt-4'] })
                .child(
                    text({ 
                        classes: ['font-semibold', 'mb-2', 'text-yellow-400', ...layout.text],
                        content: '📏 Responsive Breakpoints'
                    })
                )
                .child(
                    text({ 
                        classes: ['text-gray-300', ...layout.text],
                        content: `Narrow: < 60 cols | Medium: 60-99 cols | Wide: ≥ 100 cols`
                    })
                )
                .child(
                    text({ 
                        classes: ['text-gray-400', 'mt-2', isNarrow ? 'text-xs' : 'text-sm'],
                        content: 'Resize your terminal to see the layout adapt!'
                    })
                )
        );
}

function main() {
    console.log('🚀 Starting Responsive TUI Demo...');
    console.log(`📐 Terminal size: ${terminalWidth} × ${terminalHeight}`);
    console.log(`📱 Layout mode: ${layout.name}`);
    console.log('');
    
    const demo = createResponsiveDemo();
    
    console.log('🎯 Responsive Demo created!');
    console.log('📋 Component structure:');
    console.log(JSON.stringify(demo, null, 2));
    
    console.log('\n✅ Demo completed!');
    console.log('💡 Try resizing your terminal and running again to see different layouts!');
}

if (import.meta.main) {
    main();
}
