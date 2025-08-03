#!/usr/bin/env bun
/**
 * Interactive Slider Runner - Real TUI Application
 * 
 * Demonstrates interactive sliders with live updates and keyboard navigation.
 * Use arrow keys to adjust values, Tab to switch between sliders.
 */

import { createApp } from '../../packages/tui-bun/src/app';
import { 
    slider,
    SliderOrientation,
    SliderMode,
    horizontalSlider
} from '../../packages/tui-bun/src/widgets/slider';
import { div, text } from '../../packages/tui-bun/src/components';

let volumeValue = 50;
let brightnessValue = 128;
let temperatureValue = 22;
let priceRangeStart = 200;
let priceRangeEnd = 800;

function createSliderDemo() {
    return div({ class: 'slider-demo', id: 'main' }).child(text('🎛️  Interactive Slider Demo'))
        .child(text('Use ←→ arrows to adjust, Tab to switch, q to quit'))
        .child(text(''))
        
        // Volume slider
        .child(text('Volume Control:'))
        .child(slider({
            id: 'volume',
            mode: SliderMode.Single,
            min: 0,
            max: 100,
            value: volumeValue,
            orientation: SliderOrientation.Horizontal,
            label: `Volume: ${volumeValue}%`
        }))
        .child(text(''))
        
        // Brightness slider
        .child(text('Brightness Control:'))
        .child(slider({
            id: 'brightness',
            mode: SliderMode.Single,
            min: 0,
            max: 255,
            value: brightnessValue,
            orientation: SliderOrientation.Horizontal,
            label: `Brightness: ${brightnessValue}`
        }))
        .child(text(''))
        
        // Temperature slider with decimal steps
        .child(text('Temperature Control:'))
        .child(slider({
            id: 'temperature',
            mode: SliderMode.Single,
            min: -20,
            max: 50,
            value: temperatureValue,
            step: 0.5,
            orientation: SliderOrientation.Horizontal,
            label: `Temperature: ${temperatureValue}°C`
        }))
        .child(text(''))
        
        // Price range slider
        .child(text('Price Range:'))
        .child(slider({
            id: 'price-range',
            mode: SliderMode.Range,
            min: 0,
            max: 1000,
            value: priceRangeStart,
            rangeEnd: priceRangeEnd,
            step: 25,
            orientation: SliderOrientation.Horizontal,
            label: `Price: $${priceRangeStart} - $${priceRangeEnd}`
        }))
        .child(text(''))
        
        // Convenience function examples
        .child(text('Convenience Functions:'))
        .child(horizontalSlider({
            id: 'audio-volume',
            min: 0,
            max: 10,
            value: 7,
            step: 1,
            label: 'Audio Volume: 7/10'
        }))
        .child(text(''));
}

// Create and run the interactive app
const app = createApp({
    component: createSliderDemo,
    width: 80,
    height: 24
});

console.log('🎛️  Starting Interactive Slider Demo...');
console.log('Use arrow keys to navigate, Enter/Space to interact, q to quit');

app.run().catch(console.error);