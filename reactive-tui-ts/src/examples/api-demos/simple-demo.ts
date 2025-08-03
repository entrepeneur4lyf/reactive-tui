#!/usr/bin/env bun
/**
 * Simple Demo - Basic TUI Components
 * 
 * Demonstrates the core TUI-Bun components with the correct API
 */

import { progress, ProgressStyle } from '../../packages/tui-bun/src/widgets/progress';
import { slider, SliderOrientation, SliderMode } from '../../packages/tui-bun/src/widgets/slider';
import { toast, ToastVariant } from '../../packages/tui-bun/src/widgets/toast';
import { grid, GridColumns } from '../../packages/tui-bun/src/widgets/grid';
import { div, button, text } from '../../packages/tui-bun/src/components';

// Simple progress bar
const progressBar = progress({
    id: 'main-progress',
    value: 75,
    max: 100,
    label: 'Loading...',
    style: ProgressStyle.Linear
});

// Horizontal slider
const volumeSlider = slider({
    id: 'volume',
    mode: SliderMode.Single,
    min: 0,
    max: 100,
    value: 50,
    orientation: SliderOrientation.Horizontal
});

// Toast notification
const successToast = toast({
    id: 'success',
    message: 'Operation completed successfully!',
    variant: ToastVariant.Success,
    duration: 3000
});

// Simple grid layout
const mainGrid = grid({
    id: 'main-grid',
    columns: GridColumns.Two,
    items: [
        {
            id: 'controls',
            content: 'Controls Panel',
            backgroundColor: '#1e293b'
        },
        {
            id: 'content',
            content: 'Main Content',
            backgroundColor: '#0f172a'
        }
    ]
});

// Main application component
const app = div({ class: 'app', id: 'main' })
    .child(text('TUI-Bun Simple Demo'))
    .child(progressBar)
    .child(volumeSlider)
    .child(successToast)
    .child(mainGrid)
    .child(
        button({ text: 'Click Me', id: 'demo-button' })
            .bind_enter()
    );

console.log('Simple demo created with generated types!');
console.log('Element structure:', JSON.stringify(app.build(), null, 2));