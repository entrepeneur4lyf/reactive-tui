#!/usr/bin/env bun
/**
 * Progress Bar Showcase - Functional API Examples
 * 
 * Demonstrates all progress bar variants using the actual functional API.
 * Shows different styles and configurations.
 */

import { 
    progress,
    ProgressStyle, 
    createProgress
} from '../../packages/tui-bun/src/widgets/progress';

async function main() {
    console.log('🎯 Progress Bar Showcase - TypeScript/Bun Implementation\n');
    
    // 1. Linear Progress Bars
    console.log('📊 LINEAR PROGRESS BARS');
    console.log('========================\n');
    
    // Basic linear progress
    const basicLinear = progress({
        id: 'basic-linear',
        style: ProgressStyle.Linear,
        value: 45,
        max: 100,
        label: 'Download Progress',
        showPercentage: true
    });
    
    console.log('Basic Linear Progress:');
    console.log(JSON.stringify(basicLinear.build(), null, 2));
    console.log();
    
    // Linear progress with custom properties
    const customLinear = progress({
        id: 'upload-progress',
        style: ProgressStyle.Linear,
        value: 75,
        max: 100,
        label: 'File Upload',
        showValue: true
    });
    
    console.log('Custom Linear Progress:');
    console.log(JSON.stringify(customLinear.build(), null, 2));
    console.log();
    
    // 2. Circular Progress
    console.log('🔄 CIRCULAR PROGRESS');
    console.log('=====================\n');
    
    const circularDemo = progress({
        id: 'processing',
        style: ProgressStyle.Circular,
        value: 85,
        max: 100,
        label: 'Processing Data'
    });
    
    console.log('Circular Progress:');
    console.log(JSON.stringify(circularDemo.build(), null, 2));
    console.log();
    
    // 3. Builder Pattern Examples
    console.log('🏗️  BUILDER PATTERN EXAMPLES');
    console.log('=============================\n');
    
    const builderExample = createProgress(ProgressStyle.Linear)
        .value(60)
        .range(0, 100)
        .label('Builder Example')
        .showPercentage(true)
        .build();
    
    console.log('Builder Pattern Progress:');
    console.log(JSON.stringify(builderExample.build(), null, 2));
    console.log();
    
    // 4. Different Configurations
    console.log('⚙️  CONFIGURATION EXAMPLES');
    console.log('===========================\n');
    
    const configurations = [
        {
            name: 'Simple Progress',
            config: {
                id: 'simple',
                style: ProgressStyle.Linear,
                value: 30,
                max: 100,
                label: 'Simple'
            }
        },
        {
            name: 'With Percentage',
            config: {
                id: 'with-percent',
                style: ProgressStyle.Linear,
                value: 65,
                max: 100,
                label: 'With Percentage',
                showPercentage: true
            }
        },
        {
            name: 'Custom Max Value',
            config: {
                id: 'custom-max',
                style: ProgressStyle.Linear,
                value: 150,
                max: 200,
                label: 'Custom Max (150/200)',
                showValue: true
            }
        }
    ];
    
    configurations.forEach(({ name, config }) => {
        const progressBar = progress(config);
        console.log(`${name}:`);
        console.log(JSON.stringify(progressBar.build(), null, 2));
        console.log();
    });
    
    console.log('🎯 Progress Bar Showcase Complete!');
    console.log('📝 All examples use the functional API with ElementBuilder.');
}

// Handle process cleanup
process.on('SIGINT', () => {
    console.log('\n\n👋 Showcase interrupted by user');
    process.exit(0);
});

// Start the showcase
main().catch(console.error);