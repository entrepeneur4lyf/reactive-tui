#!/usr/bin/env bun

/**
 * Basic Component Demo
 *
 * A clean, simple demonstration of the core TUI framework components
 * using the actual APIs that exist in our framework.
 */

import {
    div,
    text,
    createSimpleCheckbox,
    createLoadingSpinner,
    linearProgress,
    SpinnerType
} from '../packages/tui-bun/src';

function createComponentDemo() {
    console.log('🎨 Creating basic component demo...');

    return div({ classes: ['demo-container', 'p-4'] }).child(
            text({
                classes: ['text-2xl', 'font-bold', 'mb-6', 'text-center'],
                content: '🎨 TUI Framework Component Demo'
            })
        )
        .child(
            div({ classes: ['section', 'mb-6'] })
                .child(
                    text({
                        classes: ['text-xl', 'font-semibold', 'mb-3'],
                        content: '📝 Text Components'
                    })
                )
                .child(
                    text({
                        classes: ['text-base', 'mb-2'],
                        content: 'This is a basic text component with styling.'
                    })
                )
                .child(
                    text({
                        classes: ['text-sm', 'text-gray-600'],
                        content: 'Smaller text with different styling.'
                    })
                )
        )
        .child(
            div({ classes: ['section', 'mb-6'] })
                .child(
                    text({
                        classes: ['text-xl', 'font-semibold', 'mb-3'],
                        content: '☑ Checkbox Components'
                    })
                )
                .child(
                    createSimpleCheckbox({
                        id: 'demo-checkbox-1',
                        label: 'Enable notifications',
                        checked: true
                    })
                )
                .child(
                    createSimpleCheckbox({
                        id: 'demo-checkbox-2',
                        label: 'Dark mode',
                        checked: false
                    })
                )
                .child(
                    createSimpleCheckbox({
                        id: 'demo-checkbox-3',
                        label: 'Auto-save',
                        checked: true
                    })
                )
        )
        .child(
            div({ classes: ['section', 'mb-6'] })
                .child(
                    text({
                        classes: ['text-xl', 'font-semibold', 'mb-3'],
                        content: '📊 Progress Components'
                    })
                )
                .child(
                    text({
                        classes: ['text-sm', 'mb-2'],
                        content: 'Download Progress:'
                    })
                )
                .child(
                    linearProgress({
                        id: 'demo-progress-1',
                        value: 75,
                        max: 100,
                        label: 'Downloading files...'
                    })
                )
                .child(
                    text({
                        classes: ['text-sm', 'mb-2', 'mt-3'],
                        content: 'Installation Progress:'
                    })
                )
                .child(
                    linearProgress({
                        id: 'demo-progress-2',
                        value: 42,
                        max: 100,
                        label: 'Installing packages...'
                    })
                )
        )
        .child(
            div({ classes: ['section', 'mb-6'] })
                .child(
                    text({
                        classes: ['text-xl', 'font-semibold', 'mb-3'],
                        content: '🔄 Spinner Components'
                    })
                )
                .child(
                    div({ classes: ['grid', 'grid-cols-3', 'gap-4'] })
                        .child(
                            div({ classes: ['spinner-demo'] })
                                .child(
                                    text({
                                        classes: ['text-sm', 'mb-2'],
                                        content: 'Loading:'
                                    })
                                )
                                .child(
                                    createLoadingSpinner({
                                        id: 'demo-spinner-1',
                                        label: 'Loading...',
                                        type: SpinnerType.Dots
                                    })
                                )
                        )
                        .child(
                            div({ classes: ['spinner-demo'] })
                                .child(
                                    text({
                                        classes: ['text-sm', 'mb-2'],
                                        content: 'Processing:'
                                    })
                                )
                                .child(
                                    createLoadingSpinner({
                                        id: 'demo-spinner-2',
                                        label: 'Processing...',
                                        type: SpinnerType.Arc
                                    })
                                )
                        )
                        .child(
                            div({ classes: ['spinner-demo'] })
                                .child(
                                    text({
                                        classes: ['text-sm', 'mb-2'],
                                        content: 'Saving:'
                                    })
                                )
                                .child(
                                    createLoadingSpinner({
                                        id: 'demo-spinner-3',
                                        label: 'Saving...',
                                        type: SpinnerType.Line
                                    })
                                )
                        )
                )
        )
        .child(
            div({ classes: ['section', 'mb-6'] })
                .child(
                    text({
                        classes: ['text-xl', 'font-semibold', 'mb-3'],
                        content: '📦 Container Components'
                    })
                )
                .child(
                    div({ classes: ['border', 'p-4', 'rounded', 'bg-gray-50'] })
                        .child(
                            text({
                                classes: ['font-medium', 'mb-2'],
                                content: 'Card Container'
                            })
                        )
                        .child(
                            text({
                                classes: ['text-sm'],
                                content: 'This is content inside a styled container with borders and padding.'
                            })
                        )
                )
        )
        .child(
            div({ classes: ['summary', 'text-center', 'mt-6', 'p-4', 'bg-blue-50', 'rounded'] })
                .child(
                    text({
                        classes: ['font-bold', 'text-lg', 'mb-2'],
                        content: '✨ Component Demo Complete!'
                    })
                )
                .child(
                    text({
                        classes: ['text-sm'],
                        content: 'This demo showcases the core TUI framework components with clean, working APIs.'
                    })
                )
        );
}

// Create and display the demo
const demo = createComponentDemo();

console.log('✨ Component demo created successfully!');
console.log('📋 Components demonstrated:');
console.log('  • Text components with various styles');
console.log('  • Checkbox components with different states');
console.log('  • Progress bars with labels and percentages');
console.log('  • Spinner components with different types');
console.log('  • Container components with styling');
console.log('  • Clean, consistent API usage');
console.log('  • Full utility-first CSS support');
console.log('');

// Output the component structure
console.log('📋 Demo Component Structure:');
console.log(JSON.stringify(demo, null, 2));

console.log('\n✅ Basic Component Demo Complete!');