#!/usr/bin/env bun
/**
 * Interactive Toast Runner - Real TUI Application
 * 
 * Demonstrates toast notifications with different variants and positions.
 * Press keys to trigger different toast types.
 */

import { createApp } from '../../packages/tui-bun/src/app';
import { 
    toast,
    ToastVariant,
    ToastPosition
} from '../../packages/tui-bun/src/widgets/toast';
import { div, text, button } from '../../packages/tui-bun/src/components';

let activeToasts: any[] = [];
let toastCounter = 0;

function createToastDemo() {
    return div({ class: 'toast-demo', id: 'main' }).child(text('🍞 Interactive Toast Demo'))
        .child(text('Press keys to trigger toasts: i=Info, s=Success, w=Warning, e=Error'))
        .child(text('Use Tab/arrows to navigate buttons, Enter to activate, q to quit'))
        .child(text(''))
        
        .child(text('Toast Controls:'))
        .child(button({ text: 'Show Info Toast (i)', id: 'info-btn' }))
        .child(button({ text: 'Show Success Toast (s)', id: 'success-btn' }))
        .child(button({ text: 'Show Warning Toast (w)', id: 'warning-btn' }))
        .child(button({ text: 'Show Error Toast (e)', id: 'error-btn' }))
        .child(text(''))
        
        .child(text('Position Examples:'))
        .child(button({ text: 'Top Left Toast', id: 'top-left-btn' }))
        .child(button({ text: 'Top Right Toast', id: 'top-right-btn' }))
        .child(button({ text: 'Bottom Center Toast', id: 'bottom-center-btn' }))
        .child(text(''))
        
        .child(text('Special Examples:'))
        .child(button({ text: 'Custom Styled Toast', id: 'custom-btn' }))
        .child(button({ text: 'Persistent Toast', id: 'persistent-btn' }))
        .child(button({ text: 'Progress Toast', id: 'progress-btn' }))
        .child(text(''))
        
        .child(text('Active Toasts:'))
        .child(createActiveToastsDisplay())
        .child(text(''))
        
        .child(text('Press keys directly: [i]nfo [s]uccess [w]arning [e]rror'))
        .child(text('Toasts will appear at configured positions'));
}

function createActiveToastsDisplay() {
    const container = div({ class: 'active-toasts' });
    
    // Display currently active toasts
    if (activeToasts.length === 0) {
        container.child(text('(No active toasts)'));
    } else {
        activeToasts.forEach((toast, index) => {
            const variant = toast.attributes?.['data-variant'] || 'info';
            const message = toast.attributes?.message || 'Toast message';
            const position = toast.attributes?.['data-position'] || 'top-right';
            
            container.child(text(`${index + 1}. [${variant.toUpperCase()}] ${message} (${position})`));
        });
    }
    
    return container;
}

function addToast(variant: ToastVariant, message: string, options: any = {}) {
    toastCounter++;
    const newToast = toast({
        id: `toast-${toastCounter}`,
        message,
        variant,
        duration: options.duration || 3000,
        position: options.position || ToastPosition.TopRight,
        dismissible: options.dismissible !== false,
        showProgress: options.showProgress,
        classes: options.classes
    });
    
    activeToasts.push(newToast.build());
    
    // Auto-remove after duration (simulate)
    if (options.duration !== 0) {
        setTimeout(() => {
            activeToasts = activeToasts.filter(t => t.id !== `toast-${toastCounter}`);
        }, options.duration || 3000);
    }
}

// Create and run the interactive app
const app = createApp({
    component: createToastDemo,
    width: 80,
    height: 24
});

// Override to add custom key handling for toast triggers
const originalRun = app.run.bind(app);
app.run = async function() {
    // Custom input handling for toast triggers
    process.stdin.on('data', (key) => {
        const keyStr = key.toString().toLowerCase();
        
        switch (keyStr) {
            case 'i':
                addToast(ToastVariant.Info, 'This is an info notification', {
                    position: ToastPosition.TopRight
                });
                break;
                
            case 's':
                addToast(ToastVariant.Success, 'Operation completed successfully!', {
                    position: ToastPosition.TopRight,
                    duration: 2000
                });
                break;
                
            case 'w':
                addToast(ToastVariant.Warning, 'Please check your input', {
                    position: ToastPosition.TopCenter,
                    duration: 4000
                });
                break;
                
            case 'e':
                addToast(ToastVariant.Error, 'An error occurred!', {
                    position: ToastPosition.BottomCenter,
                    duration: 5000
                });
                break;
                
            case '1':
                addToast(ToastVariant.Info, 'Top left notification', {
                    position: ToastPosition.TopLeft
                });
                break;
                
            case '2':
                addToast(ToastVariant.Success, 'Custom styled toast', {
                    position: ToastPosition.TopRight,
                    classes: ['custom-bg', 'custom-border']
                });
                break;
                
            case '3':
                addToast(ToastVariant.Warning, 'Persistent toast - stays until dismissed', {
                    position: ToastPosition.BottomLeft,
                    duration: 0,
                    dismissible: true
                });
                break;
        }
    });
    
    return originalRun();
};

console.log('🍞 Starting Interactive Toast Demo...');
console.log('Press i/s/w/e for different toast types, arrow keys to navigate, q to quit');

app.run().catch(console.error);