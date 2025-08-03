#!/usr/bin/env bun

/**
 * Form Controls Demo - Switch and Radio Button Widgets
 * 
 * Demonstrates the new Switch and Radio Button widgets in TypeScript/Bun
 * Shows various configurations and styling options
 */

import {
    createApp,
    div,
    text,
    createSwitch,
    createCustomSwitch,
    createFormSwitch,
    createUnicodeSwitch,
    createRadioGroup,
    createHorizontalRadioGroup,
    createCustomRadioGroup,
    createYesNoRadio,
    createRatingRadio,
    LabelPosition,
    RadioOrientation
} from '../../packages/tui-bun/src';

function createFormControlsDemo() {
    return div({ class: 'demo-container p-4' }).child(
            text('🔘 Form Controls Demo - Switch & Radio Widgets')
                .class('text-2xl font-bold mb-4 text-center')
        )
        
        // Switch Examples Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Switch Widgets').class('text-xl font-semibold mb-3'))
                
                // Basic switches
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'switch-demo' })
                                .child(text('Basic Switch:').class('font-medium mb-2'))
                                .child(createSwitch('basic-switch', false))
                        )
                        .child(
                            div({ class: 'switch-demo' })
                                .child(text('Enabled Switch:').class('font-medium mb-2'))
                                .child(createSwitch('enabled-switch', true))
                        )
                )
                
                // Custom styled switches
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'switch-demo' })
                                .child(text('Custom Switch:').class('font-medium mb-2'))
                                .child(createCustomSwitch({
                                    id: 'custom-switch',
                                    enabled: true,
                                    labels: { on: 'Active', off: 'Inactive' },
                                    handles: { on: '●', off: '○' },
                                    width: 12,
                                    position: LabelPosition.After
                                }))
                        )
                        .child(
                            div({ class: 'switch-demo' })
                                .child(text('Unicode Switch:').class('font-medium mb-2'))
                                .child(createUnicodeSwitch({
                                    id: 'unicode-switch',
                                    enabled: false,
                                    }))
                        )
                )
                
                // Form switches
                .child(
                    div({ class: 'form-switches mb-4' })
                        .child(text('Form Settings:').class('font-medium mb-2'))
                        .child(
                            div({ class: 'space-y-2' })
                                .child(createFormSwitch({
                                    id: 'notifications',
                                    label: 'Enable Notifications',
                                    enabled: true,
                                    description: 'Receive push notifications'
                                }))
                                .child(createFormSwitch({
                                    id: 'dark-mode',
                                    label: 'Dark Mode',
                                    enabled: false,
                                    description: 'Use dark theme'
                                }))
                                .child(createFormSwitch({
                                    id: 'auto-save',
                                    label: 'Auto Save',
                                    enabled: true,
                                    description: 'Automatically save changes'
                                }))
                        )
                )
        )
        
        // Radio Button Examples Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Radio Button Groups').class('text-xl font-semibold mb-3'))
                
                // Basic radio groups
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'radio-demo' })
                                .child(text('Theme Selection:').class('font-medium mb-2'))
                                .child(createRadioGroup(
                                    'theme-selection',
                                    ['Light', 'Dark', 'Auto'],
                                    'Auto'
                                ))
                        )
                        .child(
                            div({ class: 'radio-demo' })
                                .child(text('Size Options:').class('font-medium mb-2'))
                                .child(createHorizontalRadioGroup(
                                    'size-options',
                                    [
                                        { value: 'small', label: 'S', enabled: true },
                                        { value: 'medium', label: 'M', enabled: true },
                                        { value: 'large', label: 'L', enabled: true },
                                        { value: 'xlarge', label: 'XL', enabled: true }
                                    ],
                                    'medium'
                                ))
                        )
                )
                
                // Custom styled radio groups
                .child(
                    div({ class: 'grid grid-cols-2 gap-4 mb-4' })
                        .child(
                            div({ class: 'radio-demo' })
                                .child(text('Priority Level:').class('font-medium mb-2'))
                                .child(createCustomRadioGroup({
                                    id: 'priority-level',
                                    options: [
                                        { value: 'low', label: 'Low Priority', enabled: true },
                                        { value: 'medium', label: 'Medium Priority', enabled: true },
                                        { value: 'high', label: 'High Priority', enabled: true },
                                        { value: 'urgent', label: 'Urgent', enabled: true }
                                    ],
                                    selected: 'medium',
                                    orientation: RadioOrientation.Vertical
                                }))
                        )
                        .child(
                            div({ class: 'radio-demo' })
                                .child(text('Quick Decision:').class('font-medium mb-2'))
                                .child(createYesNoRadio('quick-decision', true))
                        )
                )
                
                // Rating radio
                .child(
                    div({ class: 'rating-demo mb-4' })
                        .child(text('Rate this demo:').class('font-medium mb-2'))
                        .child(createRatingRadio('demo-rating', 4))
                )
        )
        
        // Interactive Demo Section
        .child(
            div({ class: 'section mb-6' })
                .child(text('Interactive Demo').class('text-xl font-semibold mb-3'))
                .child(
                    div({ class: 'interactive-form p-4 border border-gray-300 rounded' })
                        .child(text('User Preferences Form').class('font-bold mb-4'))
                        
                        // Profile settings
                        .child(
                            div({ class: 'form-group mb-4' })
                                .child(text('Profile Settings:').class('font-medium mb-2'))
                                .child(
                                    div({ class: 'space-y-2' })
                                        .child(createFormSwitch({
                                            id: 'profile-public',
                                            label: 'Public Profile',
                                            enabled: false,
                                            description: 'Make your profile visible to others'
                                        }))
                                        .child(createFormSwitch({
                                            id: 'show-email',
                                            label: 'Show Email',
                                            enabled: false,
                                            description: 'Display email on profile'
                                        }))
                                )
                        )
                        
                        // Communication preferences
                        .child(
                            div({ class: 'form-group mb-4' })
                                .child(text('Communication:').class('font-medium mb-2'))
                                .child(createRadioGroup(
                                    'communication-pref',
                                    ['Email Only', 'SMS Only', 'Both', 'None'],
                                    'Email Only'
                                ))
                        )
                        
                        // Language preference
                        .child(
                            div({ class: 'form-group mb-4' })
                                .child(text('Language:').class('font-medium mb-2'))
                                .child(createHorizontalRadioGroup(
                                    'language-pref',
                                    [
                                        { value: 'en', label: 'EN', enabled: true },
                                        { value: 'es', label: 'ES', enabled: true },
                                        { value: 'fr', label: 'FR', enabled: true },
                                        { value: 'de', label: 'DE', enabled: true }
                                    ],
                                    'en'
                                ))
                        )
                )
        )
        
        // Summary
        .child(
            div({ class: 'summary text-center mt-6 p-4 bg-gray-100 rounded' })
                .child(text('✨ Form Controls Demo Complete!').class('font-bold text-lg mb-2'))
                .child(
                    div({ class: 'features-list text-sm' })
                        .child(text('• Switch widgets with custom styling and labels'))
                        .child(text('• Radio button groups with multiple orientations'))
                        .child(text('• Unicode and emoji support'))
                        .child(text('• Form integration patterns'))
                        .child(text('• Accessibility features (ARIA attributes)'))
                        .child(text('• Responsive layouts with CSS Grid'))
                )
        );
}

// Create and run the demo app
async function main() {
    console.log('🎯 Creating form controls demo...\n');
    
    try {
        const _app = createApp({
            stylesheet: undefined,
            component: () => createFormControlsDemo()
        });

        console.log('✨ Form controls demo created successfully!');
        console.log('📋 Features demonstrated:');
        console.log('  • Switch widgets (basic, custom, form, Unicode)');
        console.log('  • Radio button groups (vertical, horizontal, styled)');
        console.log('  • Interactive form layouts');
        console.log('  • Accessibility features');
        console.log('  • CSS Grid responsive layouts');
        
        // Show the component structure
        console.log('\n📋 Demo Component Structure:');
        console.log(JSON.stringify(createFormControlsDemo().build(), null, 2));

        console.log('\n✅ Form Controls Demo Complete!');

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
