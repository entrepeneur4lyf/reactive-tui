#!/usr/bin/env bun
/**
 * Slider Showcase - Functional API Examples
 * 
 * Demonstrates all slider variants using the actual functional API.
 * Shows different orientations, modes, and configurations.
 */

import { 
    slider,
    SliderOrientation,
    SliderMode,
    createSlider,
    horizontalSlider,
    verticalSlider,
    rangeSlider
} from '../../packages/tui-bun/src/widgets/slider';

async function main() {
    console.log('🎛️  Slider Showcase - TypeScript/Bun Implementation\n');
    
    // 1. Basic Sliders
    console.log('📊 BASIC SLIDERS');
    console.log('=================\n');
    
    // Horizontal slider
    const basicHorizontal = slider({
        id: 'volume',
        mode: SliderMode.Single,
        min: 0,
        max: 100,
        value: 50,
        orientation: SliderOrientation.Horizontal,
        label: 'Volume Control'
    });
    
    console.log('Horizontal Slider:');
    console.log(JSON.stringify(basicHorizontal.build(), null, 2));
    console.log();
    
    // Vertical slider
    const basicVertical = slider({
        id: 'brightness',
        mode: SliderMode.Single,
        min: 0,
        max: 255,
        value: 128,
        orientation: SliderOrientation.Vertical,
        label: 'Brightness'
    });
    
    console.log('Vertical Slider:');
    console.log(JSON.stringify(basicVertical.build(), null, 2));
    console.log();
    
    // 2. Convenience Functions
    console.log('🛠️  CONVENIENCE FUNCTIONS');
    console.log('==========================\n');
    
    const volumeSlider = horizontalSlider({
        id: 'audio-volume',
        min: 0,
        max: 10,
        value: 7,
        step: 1,
        label: 'Audio Volume'
    });
    
    console.log('Horizontal Convenience Function:');
    console.log(JSON.stringify(volumeSlider.build(), null, 2));
    console.log();
    
    const levelSlider = verticalSlider({
        id: 'water-level',
        min: 0,
        max: 1000,
        value: 650,
        step: 10,
        label: 'Water Level (ml)'
    });
    
    console.log('Vertical Convenience Function:');
    console.log(JSON.stringify(levelSlider.build(), null, 2));
    console.log();
    
    // 3. Range Slider
    console.log('📏 RANGE SLIDER');
    console.log('================\n');
    
    const priceRange = rangeSlider({
        id: 'price-range',
        min: 0,
        max: 1000,
        value: 200,
        rangeEnd: 800,
        step: 25,
        label: 'Price Range ($)'
    });
    
    console.log('Range Slider:');
    console.log(JSON.stringify(priceRange.build(), null, 2));
    console.log();
    
    // 4. Builder Pattern Examples
    console.log('🏗️  BUILDER PATTERN EXAMPLES');
    console.log('=============================\n');
    
    const builderExample = createSlider()
        .range(0, 360)
        .value(180)
        .step(15)
        .horizontal()
        .label('Rotation (degrees)')
        .build();
    
    console.log('Builder Pattern Slider:');
    console.log(JSON.stringify(builderExample.build(), null, 2));
    console.log();
    
    // 5. Different Configurations
    console.log('⚙️  CONFIGURATION EXAMPLES');
    console.log('===========================\n');
    
    const configurations = [
        {
            name: 'Temperature Control',
            config: {
                id: 'temperature',
                mode: SliderMode.Single,
                min: -20,
                max: 50,
                value: 22,
                step: 0.5,
                orientation: SliderOrientation.Horizontal,
                label: 'Temperature (°C)'
            }
        },
        {
            name: 'Progress Selector',
            config: {
                id: 'progress',
                mode: SliderMode.Single,
                min: 0,
                max: 100,
                value: 75,
                step: 5,
                orientation: SliderOrientation.Horizontal,
                label: 'Progress (%)'
            }
        },
        {
            name: 'Frequency Tuner',
            config: {
                id: 'frequency',
                mode: SliderMode.Single,
                min: 20,
                max: 20000,
                value: 1000,
                step: 10,
                orientation: SliderOrientation.Vertical,
                label: 'Frequency (Hz)'
            }
        }
    ];
    
    configurations.forEach(({ name, config }) => {
        const sliderWidget = slider(config);
        console.log(`${name}:`);
        console.log(JSON.stringify(sliderWidget.build(), null, 2));
        console.log();
    });
    
    // 6. Advanced Builder Examples
    console.log('🚀 ADVANCED BUILDER EXAMPLES');
    console.log('=============================\n');
    
    const advancedSliders = [
        createSlider()
            .range(0, 1)
            .value(0.5)
            .step(0.01)
            .label('Opacity')
            .build(),
        createSlider()
            .range(-100, 100)
            .value(0)
            .step(1)
            .horizontal()
            .label('Balance')
            .build()
    ];
    
    advancedSliders.forEach((sliderBuilder, index) => {
        console.log(`Advanced Slider ${index + 1}:`);
        console.log(JSON.stringify(sliderBuilder.build(), null, 2));
        console.log();
    });
    
    console.log('🎛️  Slider Showcase Complete!');
    console.log('📝 All examples use the functional API with ElementBuilder.');
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Showcase interrupted by user');
    process.exit(0);
});

// Start the showcase
main().catch(console.error);