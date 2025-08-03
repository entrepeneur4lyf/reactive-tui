#!/usr/bin/env bun

/**
 * Checkbox Widget Demo - TypeScript/Bun Implementation with Animations
 * 
 * Demonstrates both single checkboxes and checkbox groups with smooth
 * scaling animations and various styling options.
 */

import { 
    createApp, 
    div, 
    text, 
    checkboxWidget,
    createSimpleCheckbox,
    createAnimatedCheckbox,
    createCustomCheckbox,
    createCheckboxGroup,
    createHorizontalCheckboxGroup,
    createFeatureCheckboxGroup,
    createMultiSelectCheckboxGroup,
    checkboxAnimationCSS,
    CheckboxStyle,
    CheckboxLabelPosition,
    CheckboxGroupOrientation
} from '../../packages/tui-bun/src';

function createCheckboxDemo() {
    return div({ class: 'demo-container p-4' }).child(
            text('☑ Checkbox Widget Demo - TypeScript/Bun with Animations')
                .class('text-2xl font-bold mb-4 text-center')
        )
        
        // Basic Checkbox Types Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Basic Checkbox Types').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Simple Checkbox:').class('font-medium mb-2'))
                                .child(createSimpleCheckbox({
                                    id: 'simple-checkbox',
                                    label: 'Enable notifications'
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Checked Checkbox:').class('font-medium mb-2'))
                                .child(createSimpleCheckbox({
                                    id: 'checked-checkbox',
                                    label: 'Auto-save enabled',
                                    checked: true
                                }))
                        )
                )
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Animated Checkbox:').class('font-medium mb-2'))
                                .child(createAnimatedCheckbox({
                                    id: 'animated-checkbox',
                                    label: 'Smooth animation',
                                    checked: true,
                                    duration: 400,
                                    scaleFactor: 2.0
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Custom Checkbox:').class('font-medium mb-2'))
                                .child(createCustomCheckbox({
                                    id: 'custom-checkbox',
                                    label: 'Custom style',
                                    unchecked: '◯',
                                    checkedChar: '◉'
                                }))
                        )
                )
        )
        
        // Checkbox Styles Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Checkbox Styles').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-4 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Ballot Style:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'ballot-style',
                                    label: 'Ballot',
                                    checked: true,
                                    style: CheckboxStyle.Ballot,
                                    labelPosition: CheckboxLabelPosition.After
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Square Style:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'square-style',
                                    label: 'Square',
                                    checked: true,
                                    style: CheckboxStyle.Square,
                                    labelPosition: CheckboxLabelPosition.After
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Round Style:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'round-style',
                                    label: 'Round',
                                    checked: true,
                                    style: CheckboxStyle.Round,
                                    labelPosition: CheckboxLabelPosition.After
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Custom Style:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'custom-style',
                                    label: 'Custom',
                                    checked: true,
                                    style: CheckboxStyle.Custom,
                                    customChars: { unchecked: '⭕', checked: '✅' },
                                    labelPosition: CheckboxLabelPosition.After
                                }))
                        )
                )
        )
        
        // Label Positioning Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Label Positioning').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Label Before:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'before-label',
                                    label: 'Before checkbox',
                                    checked: true,
                                    labelPosition: CheckboxLabelPosition.Before
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Label After:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'after-label',
                                    label: 'After checkbox',
                                    checked: true,
                                    labelPosition: CheckboxLabelPosition.After
                                }))
                        )
                )
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Label Above:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'above-label',
                                    label: 'Above checkbox',
                                    checked: true,
                                    labelPosition: CheckboxLabelPosition.Above
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Label Below:').class('font-medium mb-2'))
                                .child(checkboxWidget({
                                    id: 'below-label',
                                    label: 'Below checkbox',
                                    checked: true,
                                    labelPosition: CheckboxLabelPosition.Below
                                }))
                        )
                )
        )
        
        // Checkbox Groups Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Checkbox Groups').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Vertical Group:').class('font-medium mb-2'))
                                .child(createCheckboxGroup({
                                    id: 'vertical-group',
                                    label: 'Select features:',
                                    options: [
                                        { label: 'Dark mode', value: 'dark_mode', checked: true },
                                        { label: 'Notifications', value: 'notifications' },
                                        { label: 'Auto-save', value: 'auto_save', checked: true },
                                        { label: 'Sync', value: 'sync' }
                                    ],
                                    orientation: CheckboxGroupOrientation.Vertical
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Horizontal Group:').class('font-medium mb-2'))
                                .child(createHorizontalCheckboxGroup({
                                    id: 'horizontal-group',
                                    label: 'Choose platforms:',
                                    options: [
                                        { label: 'Web', value: 'web', checked: true },
                                        { label: 'Mobile', value: 'mobile' },
                                        { label: 'Desktop', value: 'desktop', checked: true }
                                    ]
                                }))
                        )
                )
        )
        
        // Convenience Functions Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Convenience Functions').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Feature Selection:').class('font-medium mb-2'))
                                .child(createFeatureCheckboxGroup('features', [
                                    'Real-time collaboration',
                                    'Version control',
                                    'Cloud backup',
                                    'Advanced analytics'
                                ]))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Multi-Select:').class('font-medium mb-2'))
                                .child(createMultiSelectCheckboxGroup({
                                    id: 'multi-select',
                                    label: 'Programming languages:',
                                    options: [
                                        { label: 'Rust', value: 'rust' },
                                        { label: 'TypeScript', value: 'typescript' },
                                        { label: 'Python', value: 'python' },
                                        { label: 'Go', value: 'go' }
                                    ],
                                    selectedValues: ['rust', 'typescript']
                                }))
                        )
                )
        )
        
        // Animation Demo Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Animation Showcase').class('text-xl font-semibold mb-3'))
                
                .child(
                    div({ class: 'grid grid-cols-3 gap-4 mb-4' })
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Fast Animation:').class('font-medium mb-2'))
                                .child(createAnimatedCheckbox({
                                    id: 'fast-animation',
                                    label: 'Quick (150ms)',
                                    checked: true,
                                    duration: 150,
                                    scaleFactor: 1.3
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Bouncy Animation:').class('font-medium mb-2'))
                                .child(createAnimatedCheckbox({
                                    id: 'bouncy-animation',
                                    label: 'Bouncy (500ms)',
                                    checked: true,
                                    duration: 500,
                                    scaleFactor: 2.2
                                }))
                        )
                        .child(
                            div({ class: 'checkbox-demo' })
                                .child(text('Subtle Animation:').class('font-medium mb-2'))
                                .child(createAnimatedCheckbox({
                                    id: 'subtle-animation',
                                    label: 'Subtle (200ms)',
                                    checked: true,
                                    duration: 200,
                                    scaleFactor: 1.2
                                }))
                        )
                )
        )
        
        // Summary
        .child(
            div({ class: 'summary text-center mt-6 p-4 bg-gray-100 rounded' })
                .child(text('✨ Checkbox Demo Complete!').class('font-bold text-lg mb-2'))
                .child(
                    div({ class: 'features-list text-sm' })
                        .child(text('• Single checkboxes with animations'))
                        .child(text('• Multiple checkbox styles (ballot, square, round, custom)'))
                        .child(text('• Flexible label positioning'))
                        .child(text('• Checkbox groups (vertical & horizontal)'))
                        .child(text('• Smooth scaling animations'))
                        .child(text('• Accessibility features (ARIA)'))
                        .child(text('• TypeScript type safety'))
                        .child(text('• CSS-based animations'))
                        .child(text('• Convenience functions'))
                )
        );
}

// Create and run the demo app
async function main() {
    console.log('☑ Creating animated checkbox demo...\n');
    
    try {
        const _app = createApp({
            stylesheet: checkboxAnimationCSS, // Include the animation CSS
            component: () => createCheckboxDemo()
        });

        console.log('✨ Checkbox demo created successfully!');
        console.log('📋 Features demonstrated:');
        console.log('  • Single checkboxes with smooth scaling animations');
        console.log('  • Multiple styles: ballot (☐☑), square ([ ][x]), round (( )(x)), custom');
        console.log('  • Label positioning: before, after, above, below');
        console.log('  • Checkbox groups: vertical and horizontal layouts');
        console.log('  • Animation configurations: duration, easing, scale factor');
        console.log('  • Convenience functions for common use cases');
        console.log('  • Accessibility features with ARIA attributes');
        console.log('  • CSS-based animations with hardware acceleration');
        
        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createCheckboxDemo().build(), null, 2));

        console.log('\n✅ Animated Checkbox Demo Complete!');

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
