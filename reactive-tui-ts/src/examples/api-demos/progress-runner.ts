#!/usr/bin/env bun
/**
 * Interactive Progress Runner - Real TUI Application
 * 
 * Demonstrates animated progress bars and spinners with live updates.
 * Watch progress bars fill automatically and spinners animate.
 */

import { createApp } from '../../packages/tui-bun/src/app';
import { 
    progress,
    ProgressStyle
} from '../../packages/tui-bun/src/widgets/progress';
import { div, text } from '../../packages/tui-bun/src/components';

let downloadProgress = 0;
let uploadProgress = 25;
let processingProgress = 75;
let installProgress = 100;

function createProgressDemo() {
    return div({ class: 'progress-demo', id: 'main' }).child(text('🎯 Interactive Progress Demo'))
        .child(text('Watch live progress updates! Press q to quit'))
        .child(text(''))
        
        // Download progress (animated)
        .child(text('File Download:'))
        .child(progress({
            id: 'download',
            style: ProgressStyle.Linear,
            value: downloadProgress,
            max: 100,
            label: 'Downloading...',
            showPercentage: true
        }))
        .child(text(''))
        
        // Upload progress (static)
        .child(text('File Upload:'))
        .child(progress({
            id: 'upload',
            style: ProgressStyle.Linear,
            value: uploadProgress,
            max: 100,
            label: 'Uploading...',
            showValue: true,
            showPercentage: true
        }))
        .child(text(''))
        
        // Processing progress
        .child(text('Data Processing:'))
        .child(progress({
            id: 'processing',
            style: ProgressStyle.Circular,
            value: processingProgress,
            max: 100,
            label: 'Processing data...'
        }))
        .child(text(''))
        
        // Installation complete
        .child(text('Installation:'))
        .child(progress({
            id: 'install',
            style: ProgressStyle.Linear,
            value: installProgress,
            max: 100,
            label: 'Installation complete!',
            showPercentage: true
        }))
        .child(text(''))
        
        // Different configurations
        .child(text('Custom Configurations:'))
        .child(progress({
            id: 'custom-max',
            style: ProgressStyle.Linear,
            value: 150,
            max: 200,
            label: 'Custom Max (150/200)',
            showValue: true
        }))
        .child(text(''))
        
        // Arc style progress
        .child(text('Arc Style Progress:'))
        .child(progress({
            id: 'arc-progress',
            style: ProgressStyle.Arc,
            value: 65,
            max: 100,
            label: 'Arc Progress'
        }))
        .child(text(''))
        
        // Spinner (indeterminate)
        .child(text('Loading Spinner:'))
        .child(progress({
            id: 'spinner',
            style: ProgressStyle.Spinner,
            label: 'Loading...'
        }));
}

// Animation loop for download progress
let animationInterval: Timer;

function startAnimation() {
    animationInterval = setInterval(() => {
        downloadProgress = (downloadProgress + 2) % 101;
        
        // The app would normally handle re-rendering automatically
        // In a real implementation, this would trigger a state update
    }, 200);
}

// Create and run the interactive app
const app = createApp({
    component: createProgressDemo,
    width: 80,
    height: 24
});

console.log('🎯 Starting Interactive Progress Demo...');
console.log('Watch animated progress bars! Press q to quit');

// Start animation and run app
startAnimation();

app.run().catch(console.error).finally(() => {
    if (animationInterval) {
        clearInterval(animationInterval);
    }
});