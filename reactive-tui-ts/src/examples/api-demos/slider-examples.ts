#!/usr/bin/env bun
/**
 * Slider Examples - Bun/TypeScript Implementation
 *
 * Comprehensive examples demonstrating slider functionality:
 * - Single-value sliders with different orientations
 * - Range sliders with dual handles
 * - Interactive keyboard and mouse controls
 * - Custom styling and configurations
 */

import {
    slider,
    SliderMode,
    SliderOrientation
} from '../../packages/tui-bun/src/widgets/slider';

import { createApp } from '../../packages/tui-bun/src/app';
import { div, text } from '../../packages/tui-bun/src/components';

// Example 1: Basic Single-Value Sliders
console.log('=== Single-Value Slider Examples ===\n');

// Simple horizontal slider
const basicSlider = slider({
    id: 'basic-slider',
    mode: SliderMode.Single,
    orientation: SliderOrientation.Horizontal,
    min: 0,
    max: 100,
    value: 50,
    label: 'Volume',
    showValues: true
});

console.log('Basic Horizontal Slider:');
console.log(JSON.stringify(basicSlider.build(), null, 2));

// Slider with custom styling
const styledSlider = slider({
    id: 'styled-slider',
    mode: SliderMode.Single,
    orientation: SliderOrientation.Horizontal,
    min: 0,
    max: 10,
    value: 7.5,
    label: 'Brightness',
    showValues: true,
    showPercentage: true
});

console.log('\nStyled Slider:');
console.log(JSON.stringify(styledSlider.build(), null, 2));

// Vertical slider
const verticalSlider = slider({
    id: 'vertical-slider',
    mode: SliderMode.Single,
    orientation: SliderOrientation.Vertical,
    min: -10,
    max: 10,
    value: 3,
    label: 'Temperature',
    showValues: true
});

console.log('\nVertical Slider:');
console.log(JSON.stringify(verticalSlider.build(), null, 2));

// Example 2: Range Sliders (Dual Handle)
console.log('\n=== Range Slider Examples ===\n');

// Basic range slider
const rangeSlider = slider({
    id: 'basic-range',
    mode: SliderMode.Range,
    orientation: SliderOrientation.Horizontal,
    min: 0,
    max: 100,
    value: 25,
    rangeEnd: 75,
    label: 'Price Range',
    showValues: true
});

console.log('Basic Range Slider:');
console.log(JSON.stringify(rangeSlider.build(), null, 2));

// Time range selector
const timeRange = slider({
    id: 'time-range',
    mode: SliderMode.Range,
    orientation: SliderOrientation.Horizontal,
    min: 0,
    max: 24,
    value: 9,
    rangeEnd: 17,
    label: 'Working Hours',
    showValues: true
});

console.log('\nTime Range Slider:');
console.log(JSON.stringify(timeRange.build(), null, 2));

// Age range with custom formatting
const ageRange = slider({
    id: 'age-range',
    mode: SliderMode.Range,
    orientation: SliderOrientation.Horizontal,
    min: 18,
    max: 65,
    value: 25,
    rangeEnd: 45,
    label: 'Target Age Group',
    showValues: true,
    showPercentage: false
});

console.log('\nAge Range Slider:');
console.log(JSON.stringify(ageRange.build(), null, 2));

// Example 3: Different Step Values
console.log('\n=== Sliders with Step Values ===\n');

// Slider with step increments
const stepSlider = slider({
    id: 'step-slider',
    mode: SliderMode.Single,
    orientation: SliderOrientation.Horizontal,
    min: 0,
    max: 100,
    value: 60,
    step: 10,
    label: 'Progress',
    showValues: true
});

console.log('Step Slider:');
console.log(JSON.stringify(stepSlider.build(), null, 2));

// Musical scale slider with custom range
const scaleSlider = slider({
    id: 'scale-slider',
    mode: SliderMode.Single,
    orientation: SliderOrientation.Horizontal,
    min: 1,
    max: 8,
    value: 4,
    label: 'Musical Scale',
    showValues: true
});

console.log('\nMusical Scale Slider:');
console.log(JSON.stringify(scaleSlider.build(), null, 2));

// Example 4: Component Integration Demo
console.log('\n=== Component Integration Demo ===\n');

// Create a comprehensive demo component
function createSliderDemo() {
    return div({ class: 'slider-demo', id: 'main' }).children([
            text('🎚️ Slider Examples Demo', { class: 'title' }),

            div({ class: 'section', id: 'single-section' })
                .children([
                    text('Single-Value Sliders:', { class: 'section-title' }),
                    basicSlider,
                    styledSlider,
                    verticalSlider
                ]),

            div({ class: 'section', id: 'range-section' })
                .children([
                    text('Range Sliders:', { class: 'section-title' }),
                    rangeSlider,
                    timeRange,
                    ageRange
                ]),

            div({ class: 'section', id: 'step-section' })
                .children([
                    text('Step & Custom Sliders:', { class: 'section-title' }),
                    stepSlider,
                    scaleSlider
                ])
        ]);
}

// Create and run the demo app
async function main() {
    console.log('🎯 Creating interactive slider demo...\n');

    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createSliderDemo()
        });

        console.log('✨ Slider examples demo created successfully!');
        console.log('🎚️ Features demonstrated:');
        console.log('  • Single-value sliders with different orientations');
        console.log('  • Range sliders with dual handles');
        console.log('  • Step values and custom configurations');
        console.log('  • Horizontal and vertical orientations');
        console.log('\n🎯 All slider widgets integrated into a demo application!');

        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createSliderDemo().build(), null, 2));

        console.log('\n✅ Slider Examples Demo Complete!');

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