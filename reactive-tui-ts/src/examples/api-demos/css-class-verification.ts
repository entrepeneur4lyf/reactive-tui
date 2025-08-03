#!/usr/bin/env bun

/**
 * CSS Class Support Verification
 * 
 * Verifies that all TUI framework widgets properly support CSS classes
 * and that the classes are correctly applied to the generated elements.
 */

import {
    div,
    text,
    createSimpleCheckbox,
    createAnimatedCheckbox,
    createCheckboxGroup,
    createLoadingSpinner,
    linearProgress,
    circularProgress,
    SpinnerType
} from '../../packages/tui-bun/src';

function verifyCSSClassSupport() {
    console.log('🔍 Verifying CSS class support across all widgets...\n');
    
    const testClasses = ['test-class-1', 'test-class-2', 'utility-class'];
    const results: { widget: string; supported: boolean; classes: string[]; error?: string }[] = [];
    
    // Test each widget type
    const widgets = [
        {
            name: 'Text Component',
            create: () => text({ 
                classes: testClasses,
                content: 'Test text'
            })
        },
        {
            name: 'Div Container',
            create: () => div({ 
                classes: testClasses
            })
        },
        {
            name: 'Simple Checkbox',
            create: () => createSimpleCheckbox({
                id: 'test-checkbox',
                label: 'Test checkbox',
                classes: testClasses
            })
        },
        {
            name: 'Animated Checkbox',
            create: () => createAnimatedCheckbox({
                id: 'test-animated-checkbox',
                label: 'Test animated checkbox',
                classes: testClasses,
                animationConfig: { enabled: true, duration: 200, easing: 'ease-out', scaleFactor: 1.2 }
            })
        },
        {
            name: 'Checkbox Group',
            create: () => createCheckboxGroup({
                id: 'test-checkbox-group',
                label: 'Test group',
                options: [
                    { label: 'Option 1', value: 'opt1' },
                    { label: 'Option 2', value: 'opt2' }
                ],
                classes: testClasses
            })
        },
        {
            name: 'Loading Spinner',
            create: () => createLoadingSpinner({
                id: 'test-spinner',
                label: 'Test spinner',
                type: SpinnerType.Dots,
                classes: testClasses
            })
        },
        {
            name: 'Linear Progress',
            create: () => linearProgress({
                id: 'test-progress',
                value: 50,
                max: 100,
                label: 'Test progress',
                classes: testClasses
            })
        },
        {
            name: 'Circular Progress',
            create: () => circularProgress({
                id: 'test-circular-progress',
                value: 75,
                max: 100,
                label: 'Test circular progress',
                classes: testClasses
            })
        }
    ];
    
    // Test each widget
    widgets.forEach(widget => {
        try {
            const element = widget.create();
            const elementData = element.build ? element.build() : element;

            // Check if classes are present in the element structure
            let classesFound: string[] = [];

            // The element structure has classes directly on the element
            if (elementData.classes && Array.isArray(elementData.classes)) {
                classesFound = elementData.classes;
            }
            
            // Verify test classes are included
            const hasTestClasses = testClasses.every(testClass => 
                classesFound.includes(testClass)
            );
            
            results.push({
                widget: widget.name,
                supported: hasTestClasses,
                classes: classesFound,
                error: hasTestClasses ? undefined : 'Test classes not found in element'
            });
            
        } catch (error) {
            results.push({
                widget: widget.name,
                supported: false,
                classes: [],
                error: `Error creating widget: ${error}`
            });
        }
    });
    
    // Display results
    console.log('📊 CSS Class Support Results:\n');
    
    let allSupported = true;
    
    results.forEach(result => {
        const status = result.supported ? '✅' : '❌';
        console.log(`${status} ${result.widget}`);
        
        if (result.supported) {
            console.log(`   Classes: [${result.classes.join(', ')}]`);
        } else {
            console.log(`   Error: ${result.error}`);
            allSupported = false;
        }
        console.log('');
    });
    
    // Summary
    console.log('📋 Summary:');
    console.log(`   Total widgets tested: ${results.length}`);
    console.log(`   Widgets with CSS support: ${results.filter(r => r.supported).length}`);
    console.log(`   Widgets without CSS support: ${results.filter(r => !r.supported).length}`);
    console.log('');
    
    if (allSupported) {
        console.log('🎉 All widgets support CSS classes correctly!');
        console.log('✨ Your TUI framework is fully compatible with utility-first CSS!');
    } else {
        console.log('⚠️  Some widgets need CSS class support improvements.');
    }
    
    return allSupported;
}

// Test utility-first CSS classes
function testUtilityClasses() {
    console.log('\n🌊 Testing utility-first CSS classes...\n');
    
    const utilityClasses = [
        // Layout
        'flex', 'grid', 'block', 'inline-block',
        // Spacing
        'p-4', 'mx-auto', 'mb-6', 'space-y-4',
        // Colors
        'text-blue-600', 'bg-gray-50', 'border-red-500',
        // Typography
        'text-lg', 'font-bold', 'text-center',
        // Effects
        'shadow-lg', 'rounded-md', 'opacity-75',
        // Responsive
        'md:grid-cols-2', 'lg:text-xl'
    ];
    
    const testElement = div({
        classes: utilityClasses
    });

    const elementData = testElement.build ? testElement.build() : testElement;
    const appliedClasses = elementData.classes || [];

    console.log('🎨 Applied utility classes:');
    utilityClasses.forEach(className => {
        const applied = Array.isArray(appliedClasses) && appliedClasses.includes(className);
        const status = applied ? '✅' : '❌';
        console.log(`   ${status} ${className}`);
    });

    const allApplied = Array.isArray(appliedClasses) && utilityClasses.every(className =>
        appliedClasses.includes(className)
    );

    console.log('\n📊 Utility CSS Test Result:');
    if (allApplied) {
        console.log('✅ All utility classes applied successfully!');
    } else {
        console.log('❌ Some utility classes were not applied.');
    }
    
    return allApplied;
}

// Run verification
console.log('🎨 TUI Framework CSS Class Support Verification\n');
console.log('=' .repeat(60));

const widgetSupport = verifyCSSClassSupport();
const utilitySupport = testUtilityClasses();

console.log('\n' + '=' .repeat(60));
console.log('🏆 Final Results:');
console.log(`   Widget CSS Support: ${widgetSupport ? '✅ PASS' : '❌ FAIL'}`);
console.log(`   Utility Classes: ${utilitySupport ? '✅ PASS' : '❌ FAIL'}`);

if (widgetSupport && utilitySupport) {
    console.log('\n🎉 VERIFICATION COMPLETE: Full CSS utility support confirmed!');
    console.log('🌊 Your TUI framework is ready for utility-first CSS frameworks!');
} else {
    console.log('\n⚠️  VERIFICATION FAILED: CSS support needs improvement.');
}

console.log('\n✨ CSS Class Verification Complete!');
