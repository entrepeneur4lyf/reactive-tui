#!/usr/bin/env bun
/**
 * Toast Showcase - Functional API Examples
 * 
 * Demonstrates all toast variants using the actual functional API.
 * Shows different types, positions, and configurations.
 */

import { 
    toast,
    ToastVariant,
    ToastPosition,
    createToast,
    infoToast,
    successToast,
    warningToast,
    errorToast
} from '../../packages/tui-bun/src/widgets/toast';

async function main() {
    console.log('🍞 Toast Showcase - TypeScript/Bun Implementation\n');
    
    // 1. Basic Toast Types
    console.log('📢 BASIC TOAST TYPES');
    console.log('====================\n');
    
    // Info toast
    const infoExample = toast({
        id: 'info-example',
        message: 'This is an informational message',
        variant: ToastVariant.Info,
        duration: 3000
    });
    
    console.log('Info Toast:');
    console.log(JSON.stringify(infoExample.build(), null, 2));
    console.log();
    
    // Success toast
    const successExample = toast({
        id: 'success-example',
        message: 'Operation completed successfully!',
        variant: ToastVariant.Success,
        duration: 2000
    });
    
    console.log('Success Toast:');
    console.log(JSON.stringify(successExample.build(), null, 2));
    console.log();
    
    // Warning toast
    const warningExample = toast({
        id: 'warning-example',
        message: 'Please check your input',
        variant: ToastVariant.Warning,
        duration: 4000
    });
    
    console.log('Warning Toast:');
    console.log(JSON.stringify(warningExample.build(), null, 2));
    console.log();
    
    // Error toast
    const errorExample = toast({
        id: 'error-example',
        message: 'An error occurred while processing',
        variant: ToastVariant.Error,
        duration: 5000
    });
    
    console.log('Error Toast:');
    console.log(JSON.stringify(errorExample.build(), null, 2));
    console.log();
    
    // 2. Convenience Functions
    console.log('🛠️  CONVENIENCE FUNCTIONS');
    console.log('==========================\n');
    
    const quickInfo = infoToast({
        id: 'quick-info',
        message: 'Quick info message',
        duration: 1500
    });
    
    console.log('Quick Info Toast:');
    console.log(JSON.stringify(quickInfo.build(), null, 2));
    console.log();
    
    const quickSuccess = successToast({
        id: 'quick-success',
        message: 'File saved successfully',
        duration: 2000
    });
    
    console.log('Quick Success Toast:');
    console.log(JSON.stringify(quickSuccess.build(), null, 2));
    console.log();
    
    const quickWarning = warningToast({
        id: 'quick-warning',
        message: 'Connection unstable',
        duration: 3000
    });
    
    console.log('Quick Warning Toast:');
    console.log(JSON.stringify(quickWarning.build(), null, 2));
    console.log();
    
    const quickError = errorToast({
        id: 'quick-error',
        message: 'Failed to connect to server',
        duration: 4000
    });
    
    console.log('Quick Error Toast:');
    console.log(JSON.stringify(quickError.build(), null, 2));
    console.log();
    
    // 3. Custom Toasts
    console.log('🎨 CUSTOM TOASTS');
    console.log('=================\n');
    
    const customToast = toast({
        id: 'custom-toast',
        message: 'Custom styled toast notification',
        variant: ToastVariant.Info,
        classes: ['custom-bg', 'custom-border', 'custom-text'],
        duration: 3000
    });
    
    console.log('Custom Toast:');
    console.log(JSON.stringify(customToast.build(), null, 2));
    console.log();
    
    // 4. Builder Pattern Examples
    console.log('🏗️  BUILDER PATTERN EXAMPLES');
    console.log('=============================\n');
    
    const builderExample = createToast('Builder pattern toast', ToastVariant.Info)
        .duration(2500)
        .position(ToastPosition.TopRight)
        .dismissible(true)
        .build();
    
    console.log('Builder Pattern Toast:');
    console.log(JSON.stringify(builderExample.build(), null, 2));
    console.log();
    
    // 5. Different Configurations
    console.log('⚙️  CONFIGURATION EXAMPLES');
    console.log('===========================\n');
    
    const configurations = [
        {
            name: 'Long Duration Toast',
            config: {
                id: 'long-duration',
                message: 'This toast will stay visible longer',
                variant: ToastVariant.Info,
                duration: 10000
            }
        },
        {
            name: 'Auto-dismiss Toast',
            config: {
                id: 'auto-dismiss',
                message: 'This toast will auto-dismiss',
                variant: ToastVariant.Success,
                duration: 1000,
                autoDismiss: true
            }
        },
        {
            name: 'Persistent Toast',
            config: {
                id: 'persistent',
                message: 'This toast requires manual dismissal',
                variant: ToastVariant.Warning,
                duration: 0, // 0 means persistent
                dismissible: true
            }
        }
    ];
    
    configurations.forEach(({ name, config }) => {
        const toastWidget = toast(config);
        console.log(`${name}:`);
        console.log(JSON.stringify(toastWidget.build(), null, 2));
        console.log();
    });
    
    // 6. Advanced Builder Examples
    console.log('🚀 ADVANCED BUILDER EXAMPLES');
    console.log('=============================\n');
    
    const advancedToasts = [
        createToast('Multi-line toast message\\nwith line breaks', ToastVariant.Info)
            .duration(5000)
            .dismissible(true)
            .build(),
        createToast('Persistent toast notification', ToastVariant.Success)
            .duration(0)
            .dismissible(true)
            .showProgress(false)
            .build()
    ];
    
    advancedToasts.forEach((toastBuilder, index) => {
        console.log(`Advanced Toast ${index + 1}:`);
        console.log(JSON.stringify(toastBuilder.build(), null, 2));
        console.log();
    });
    
    console.log('🍞 Toast Showcase Complete!');
    console.log('📝 All examples use the functional API with ElementBuilder.');
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Showcase interrupted by user');
    process.exit(0);
});

// Start the showcase
main().catch(console.error);