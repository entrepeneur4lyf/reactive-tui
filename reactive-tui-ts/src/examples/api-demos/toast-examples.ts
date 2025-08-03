#!/usr/bin/env bun
/**
 * Toast Examples - Bun/TypeScript Implementation
 *
 * Comprehensive examples demonstrating toast notifications:
 * - Different toast variants (info, success, warning, error)
 * - Various positioning options
 * - Auto-dismiss and persistent toasts
 * - Progress indicators and animations
 */

import {
    toast,
    ToastVariant,
    ToastPosition
} from '../../packages/tui-bun/src/widgets/toast';

import { createApp } from '../../packages/tui-bun/src/app';
import { div, text } from '../../packages/tui-bun/src/components';

// Example 1: Basic Toast Variants
console.log('=== Basic Toast Variants ===\n');

// Info toast
const infoToast = toast({
    id: 'info-toast',
    message: 'System information updated successfully',
    variant: ToastVariant.Info,
    duration: 4000,
    showProgress: true
});

console.log('Info Toast:');
console.log(JSON.stringify(infoToast.build(), null, 2));

// Success toast
const successToast = toast({
    id: 'success-toast',
    message: 'File saved successfully!',
    variant: ToastVariant.Success,
    duration: 3000,
    showProgress: true
});

console.log('\nSuccess Toast:');
console.log(JSON.stringify(successToast.build(), null, 2));

// Warning toast
const warningToast = toast({
    id: 'warning-toast',
    message: 'Low disk space detected',
    variant: ToastVariant.Warning,
    duration: 6000,
    showProgress: true
});

console.log('\nWarning Toast:');
console.log(JSON.stringify(warningToast.build(), null, 2));

// Error toast
const errorToast = toast({
    id: 'error-toast',
    message: 'Failed to connect to server',
    variant: ToastVariant.Error,
    duration: 8000,
    showProgress: true
});

console.log('\nError Toast:');
console.log(JSON.stringify(errorToast.build(), null, 2));

// Example 2: Custom Styled Toasts
console.log('\n=== Custom Styled Toasts ===\n');

// Minimal toast without icons
const minimalToast = toast({
    id: 'minimal-toast',
    message: 'Clean, minimal notification',
    variant: ToastVariant.Info,
    showIcon: false,
    showCloseButton: false,
    duration: 3000
});

console.log('Minimal Toast:');
console.log(JSON.stringify(minimalToast.build(), null, 2));

// Persistent toast (no auto-dismiss)
const persistentToast = toast({
    id: 'persistent-toast',
    message: 'This notification will stay until manually dismissed',
    variant: ToastVariant.Error,
    duration: 0, // Persistent
    showProgress: false,
    dismissible: true
});

console.log('\nPersistent Toast:');
console.log(JSON.stringify(persistentToast.build(), null, 2));

// Example 3: Toast Positioning Examples
console.log('\n=== Toast Positioning Examples ===\n');

const positions = [
    ToastPosition.TopLeft,
    ToastPosition.TopCenter,
    ToastPosition.TopRight,
    ToastPosition.BottomLeft,
    ToastPosition.BottomCenter,
    ToastPosition.BottomRight
];

for (const position of positions) {
    const positionToast = toast({
        id: `position-${position}`,
        message: `Positioned at ${position}`,
        variant: ToastVariant.Info,
        position: position,
        duration: 0 // Persistent
    });

    console.log(`${position} Toast:`);
    console.log(JSON.stringify(positionToast.build(), null, 2));
}

// Example 4: Different Duration Examples
console.log('\n=== Different Duration Examples ===\n');

// Quick toast (short duration)
const quickToast = toast({
    id: 'quick-toast',
    message: 'Quick notification',
    variant: ToastVariant.Success,
    duration: 2000,
    showProgress: true
});

console.log('Quick Toast (2s):');
console.log(JSON.stringify(quickToast.build(), null, 2));

// Long toast (extended duration)
const longToast = toast({
    id: 'long-toast',
    message: 'This notification will stay visible for a longer time',
    variant: ToastVariant.Warning,
    duration: 10000,
    showProgress: true
});

console.log('\nLong Toast (10s):');
console.log(JSON.stringify(longToast.build(), null, 2));

// Example 5: Component Integration Demo
console.log('\n=== Component Integration Demo ===\n');

// Create a comprehensive demo component
function createToastDemo() {
    return div({ class: 'toast-demo', id: 'main' }).children([
            text('🔔 Toast Examples Demo', { class: 'title' }),

            div({ class: 'section', id: 'variants-section' })
                .children([
                    text('Toast Variants:', { class: 'section-title' }),
                    infoToast,
                    successToast,
                    warningToast,
                    errorToast
                ]),

            div({ class: 'section', id: 'styled-section' })
                .children([
                    text('Custom Styled Toasts:', { class: 'section-title' }),
                    minimalToast,
                    persistentToast
                ]),

            div({ class: 'section', id: 'duration-section' })
                .children([
                    text('Duration Examples:', { class: 'section-title' }),
                    quickToast,
                    longToast
                ])
        ]);
}

// Create and run the demo app
async function main() {
    console.log('🎯 Creating interactive toast demo...\n');

    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createToastDemo()
        });

        console.log('✨ Toast examples demo created successfully!');
        console.log('🔔 Features demonstrated:');
        console.log('  • Different toast variants (info, success, warning, error)');
        console.log('  • Various positioning options');
        console.log('  • Auto-dismiss and persistent toasts');
        console.log('  • Progress indicators and custom styling');
        console.log('\n🎯 All toast widgets integrated into a demo application!');

        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createToastDemo().build(), null, 2));

        console.log('\n✅ Toast Examples Demo Complete!');

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