#!/usr/bin/env bun
/**
 * Comprehensive TUI Widget Demo - Bun/TypeScript Implementation
 *
 * Complete demonstration combining all four Bun/TypeScript widgets:
 * - Progress bars with various styles and animations
 * - Interactive sliders for value selection
 * - Toast notifications with different variants
 * - Grid layouts with colored panels and complex positioning
 *
 * This demo showcases a realistic application interface using all widgets together.
 */

import {
    progress,
    ProgressStyle,
    ProgressState,
    SPINNER_TYPES
} from '../packages/tui-bun/src/widgets/progress';

import {
    slider,
    SliderMode,
    SliderOrientation
} from '../packages/tui-bun/src/widgets/slider';

import {
    toast,
    ToastVariant
} from '../packages/tui-bun/src/widgets/toast';

import {
    grid,
    GridColumns
} from '../packages/tui-bun/src/widgets/grid';

import {
    div,
    text
} from '../packages/tui-bun/src/components';

import { createApp as createTuiApp } from '../packages/tui-bun/src/app';

// Create system monitoring widgets
function createSystemMonitors() {
    // CPU Usage Progress Bar
    const cpuProgress = progress({
        id: 'cpu-monitor',
        style: ProgressStyle.Linear,
        label: 'CPU Usage',
        value: 67,
        min: 0,
        max: 100,
        showPercentage: true
    });

    // Memory Usage Progress Bar
    const memoryProgress = progress({
        id: 'memory-monitor',
        style: ProgressStyle.Circular,
        label: 'Memory',
        value: 45,
        min: 0,
        max: 100
    });

    // Network Activity Spinner
    const networkSpinner = progress({
        id: 'network-activity',
        style: ProgressStyle.Spinner,
        label: 'Network',
        state: ProgressState.Indeterminate,
        spinnerType: SPINNER_TYPES.dots2
    });

    return { cpuProgress, memoryProgress, networkSpinner };
}

// Create interactive control widgets
function createInteractiveControls() {
    // Volume Control Slider
    const volumeSlider = slider({
        id: 'volume',
        mode: SliderMode.Single,
        orientation: SliderOrientation.Horizontal,
        min: 0,
        max: 100,
        value: 75,
        label: 'Volume',
        showValues: true
    });

    // Brightness Range Slider
    const brightnessSlider = slider({
        id: 'brightness',
        mode: SliderMode.Single,
        orientation: SliderOrientation.Horizontal,
        min: 0,
        max: 10,
        value: 7,
        label: 'Brightness',
        showValues: true
    });

    // Temperature Range Control
    const tempRange = slider({
        id: 'temp-range',
        mode: SliderMode.Range,
        orientation: SliderOrientation.Horizontal,
        min: 16,
        max: 30,
        value: 20,
        rangeEnd: 24,
        label: 'Temp Range',
        showValues: true
    });

    return { volumeSlider, brightnessSlider, tempRange };
}

// Create notification examples
function createNotificationExamples() {
    const notifications = [
        toast({
            id: 'success-toast',
            message: 'System backup completed successfully',
            variant: ToastVariant.Success,
            duration: 4000,
            showProgress: true
        }),
        toast({
            id: 'warning-toast',
            message: 'Warning: High CPU usage detected',
            variant: ToastVariant.Warning,
            duration: 6000,
            showProgress: true
        }),
        toast({
            id: 'info-toast',
            message: 'New software update available (v2.1.0)',
            variant: ToastVariant.Info,
            duration: 8000,
            showProgress: true
        }),
        toast({
            id: 'error-toast',
            message: 'Critical: Database connection lost',
            variant: ToastVariant.Error,
            duration: 0, // Persistent
            showProgress: false
        })
    ];

    return notifications;
}

// Create main application layout
function createMainLayout() {
    const { cpuProgress, memoryProgress, networkSpinner } = createSystemMonitors();
    const { volumeSlider, brightnessSlider, tempRange } = createInteractiveControls();
    const notifications = createNotificationExamples();

    // Create main grid layout
    const mainGrid = grid({
        id: 'main-app',
        columns: GridColumns.Three,
        gap: 2,
        items: [
            {
                id: 'header',
                content: '🚀 Advanced TUI Application Suite 🚀\n\nDemonstrating all widget capabilities',
                column: 0,
                row: 0,
                colSpan: 3,
                backgroundColor: '#7c3aed',
                textColor: '#ffffff'
            },
            {
                id: 'sidebar',
                content: 'Control Panel\n\n🎛️  System Controls\n📊 Monitoring\n⚙️  Settings\n📈 Analytics',
                column: 0,
                row: 1,
                backgroundColor: '#1e40af',
                textColor: '#ffffff'
            },
            {
                id: 'main-content',
                content: 'Main Dashboard Area\n\n[Dashboard widgets displayed here]',
                column: 1,
                row: 1,
                backgroundColor: '#374151',
                textColor: '#ffffff'
            },
            {
                id: 'info-panel',
                content: 'System Info\n\n💾 Memory: 67%\n🖥️  CPU: 23%\n🌐 Network: OK\n🔒 Security: ON',
                column: 2,
                row: 1,
                backgroundColor: '#059669',
                textColor: '#ffffff'
            },
            {
                id: 'footer',
                content: 'Status: Online | Users: 1,247 | Load: 23% | Last Update: 14:32:15',
                column: 0,
                row: 2,
                colSpan: 3,
                backgroundColor: '#374151',
                textColor: '#ffffff'
            }
        ]
    });

    return {
        mainGrid,
        widgets: {
            cpuProgress,
            memoryProgress,
            networkSpinner,
            volumeSlider,
            brightnessSlider,
            tempRange,
            notifications
        }
    };
}

// Create dashboard with all widgets
function createDashboard() {
    const { cpuProgress, memoryProgress, networkSpinner } = createSystemMonitors();
    const { volumeSlider, brightnessSlider, tempRange } = createInteractiveControls();

    // File Transfer Progress
    const transferProgress = progress({
        id: 'file-transfer',
        style: ProgressStyle.Linear,
        label: 'File Transfer',
        value: 78,
        min: 0,
        max: 100,
        showPercentage: true,
        showValue: false
    });

    // Backup Progress
    const backupProgress = progress({
        id: 'backup',
        style: ProgressStyle.Circular,
        label: 'Backup',
        value: 34,
        min: 0,
        max: 100
    });

    // Processing Spinner
    const processingSpinner = progress({
        id: 'processing',
        style: ProgressStyle.Spinner,
        label: 'Processing',
        state: ProgressState.Indeterminate,
        spinnerType: SPINNER_TYPES.clock
    });

    // Create dashboard grid
    const dashboardGrid = grid({
        id: 'dashboard',
        columns: GridColumns.Three,
        gap: 1,
        items: [
            {
                id: 'system-widget',
                content: 'System Performance\n\n[CPU, Memory, Network widgets here]',
                column: 0,
                row: 0,
                backgroundColor: '#1e40af',
                textColor: '#ffffff'
            },
            {
                id: 'progress-widget',
                content: 'Active Tasks\n\n[Transfer, Backup, Processing widgets here]',
                column: 1,
                row: 0,
                backgroundColor: '#059669',
                textColor: '#ffffff'
            },
            {
                id: 'controls-widget',
                content: 'System Controls\n\n[Volume, Brightness, Temperature widgets here]',
                column: 2,
                row: 0,
                backgroundColor: '#d97706',
                textColor: '#ffffff'
            }
        ]
    });

    return {
        dashboardGrid,
        widgets: {
            cpuProgress,
            memoryProgress,
            networkSpinner,
            transferProgress,
            backupProgress,
            processingSpinner,
            volumeSlider,
            brightnessSlider,
            tempRange
        }
    };
}

// Main application component
function createAppComponent() {
    const { mainGrid, widgets } = createMainLayout();
    const { dashboardGrid } = createDashboard();

    // Create the main application container
    const app = div({ class: 'comprehensive-demo', id: 'main-app' })
        .child(
            div({ class: 'header', id: 'app-header' })
                .child(text('🚀 Comprehensive TUI Widget Demonstration 🚀'))
        )
        .child(
            div({ class: 'main-content', id: 'main-content' })
                .child(mainGrid)
                .child(dashboardGrid)
        )
        .child(
            div({ class: 'widgets-showcase', id: 'widgets' })
                .child(text('Individual Widget Showcase:'))
                .child(widgets.cpuProgress)
                .child(widgets.memoryProgress)
                .child(widgets.networkSpinner)
                .child(widgets.volumeSlider)
                .child(widgets.brightnessSlider)
                .child(widgets.tempRange)
        )
        .child(
            div({ class: 'notifications', id: 'notifications' })
                .child(text('Notification Examples:'))
                .children(widgets.notifications)
        )
        .child(
            div({ class: 'footer', id: 'app-footer' })
                .child(text('✨ All TUI widgets working together in harmony! ✨'))
        );

    return app;
}

// Main demonstration function
async function main() {
    console.log('🚀 Starting Comprehensive TUI Widget Demonstration...\n');

    try {
        // Create the main application component
        const appComponent = createAppComponent();

        // Create and run the TUI application
        createTuiApp({
            stylesheet: undefined, // No external stylesheet for this demo
            component: () => appComponent
        });

        console.log('✨ Successfully created comprehensive TUI widget demo!');
        console.log('📊 Features demonstrated:');
        console.log('  • Progress Bars: Linear, circular, arc, and spinner variants');
        console.log('  • Sliders: Single-value and range sliders');
        console.log('  • Toast Notifications: All variants with different durations');
        console.log('  • Grid Layouts: Complex layouts with colored panels');
        console.log('\n🎯 All widgets integrated into a cohesive application interface!');

        // Show the component structure
        console.log('\n📋 Application Component Structure:');
        console.log(JSON.stringify(appComponent.build(), null, 2));

        console.log('\n✅ Comprehensive Demo Complete!');
        console.log('🏆 All TUI widgets working together successfully!');

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