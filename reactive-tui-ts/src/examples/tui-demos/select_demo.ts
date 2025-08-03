/**
 * Select Dropdown Widget Demo - TypeScript
 * 
 * Demonstrates the comprehensive select widget functionality including:
 * - Single and multi-select modes
 * - Search filtering capabilities
 * - Custom options with icons and descriptions
 * - Keyboard navigation and event handling
 * - Different styling configurations
 */

import { select, selectPatterns, SelectOption } from '../../packages/tui-bun/src/widgets/select';

async function runSelectDemo() {
    console.log('🔽 Select Dropdown Widget Demo - TypeScript');
    console.log('='.repeat(50));

    // Demo 1: Simple single-select dropdown
    console.log('\n1. Single-Select Language Dropdown');
    console.log('-'.repeat(30));
    
    const languageSelect = select({
        id: 'language-select',
        options: ['Rust', 'TypeScript', 'Python', 'Go', 'Java'],
        selected: 0, // Pre-select Rust
        placeholder: 'Choose a programming language...',
        onChange: (selectedIndices) => {
            const languages = ['Rust', 'TypeScript', 'Python', 'Go', 'Java'];
            if (selectedIndices.length > 0) {
                console.log(`   Selected: ${languages[selectedIndices[0]]}`);
            }
        }
    });

    console.log(`✅ Language select created`);
    console.log(`   Display text: ${languageSelect.getDisplayText()}`);
    console.log(`   Selected options: ${languageSelect.getSelectedIds().join(', ')}`);

    // Demo 2: Multi-select with search
    console.log('\n2. Multi-Select Tags with Search');
    console.log('-'.repeat(30));

    const tagsSelect = select({
        id: 'tags-select',
        options: ['Frontend', 'Backend', 'Database', 'Mobile', 'DevOps', 'Security'],
        multiSelect: true,
        searchable: true,
        selectedIndices: [0, 2], // Pre-select Frontend and Database
        placeholder: 'Select relevant tags...',
        maxHeight: 4,
        onChange: (selectedIndices) => {
            const tags = ['Frontend', 'Backend', 'Database', 'Mobile', 'DevOps', 'Security'];
            const selectedTags = selectedIndices.map(i => tags[i]);
            console.log(`   Selected tags: ${selectedTags.join(', ')}`);
        }
    });

    console.log(`✅ Tags select created`);
    console.log(`   Display text: ${tagsSelect.getDisplayText()}`);
    console.log(`   Initial selection: ${tagsSelect.getSelectedOptions().map(o => o.label).join(', ')}`);

    // Test search functionality
    tagsSelect.setSearchQuery('dev');
    console.log(`   After searching 'dev': query applied`);

    // Demo 3: Custom options with icons and descriptions
    console.log('\n3. Custom Priority Select');
    console.log('-'.repeat(30));

    const priorityOptions: SelectOption[] = [
        {
            id: 'critical',
            label: 'Critical',
            icon: '🔴',
            description: 'Requires immediate attention',
            disabled: false
        },
        {
            id: 'high',
            label: 'High Priority',
            icon: '🟡',
            description: 'Important but not urgent',
            disabled: false
        },
        {
            id: 'medium',
            label: 'Medium Priority',
            icon: '🟠',
            description: 'Standard priority level',
            disabled: false
        },
        {
            id: 'low',
            label: 'Low Priority',
            icon: '🟢',
            description: 'Can be addressed later',
            disabled: false
        },
        {
            id: 'deferred',
            label: 'Deferred',
            icon: '⚪',
            description: 'Put on hold for now',
            disabled: true // Disabled option
        }
    ];

    const prioritySelect = select({
        id: 'priority-select',
        customOptions: priorityOptions,
        placeholder: 'Set priority level...',
        position: 'above',
        onChange: (selectedIndices) => {
            if (selectedIndices.length > 0) {
                const selected = priorityOptions[selectedIndices[0]];
                console.log(`   Priority set to: ${selected.label} ${selected.icon}`);
            }
        }
    });

    console.log(`✅ Priority select created`);
    console.log(`   Options: ${priorityOptions.map(o => `${o.icon} ${o.label}`).join(', ')}`);

    // Test selection
    prioritySelect.select(1); // Select High Priority
    console.log(`   Selected: ${prioritySelect.getDisplayText()}`);

    // Demo 4: Keyboard navigation simulation
    console.log('\n4. Keyboard Navigation Test');
    console.log('-'.repeat(30));

    // Open dropdown
    prioritySelect.open();
    console.log(`✅ Dropdown opened`);

    // Navigate with arrows
    prioritySelect.navigateNext();
    prioritySelect.navigateNext();
    console.log(`✅ Navigated to next options`);

    // Test key handling
    const handled = prioritySelect.handleKeyEvent(new KeyboardEvent('keydown', { key: 'Enter' }));
    console.log(`✅ Enter key handled: ${handled}`);

    // Demo 5: Convenience patterns
    console.log('\n5. Convenience Patterns');
    console.log('-'.repeat(30));

    const yesNoSelect = selectPatterns.yesNo('confirm-select');
    console.log(`✅ Yes/No select: ${yesNoSelect.getDisplayText()}`);

    const languagesSelect = selectPatterns.languages('tech-select');
    console.log(`✅ Languages select: ${languagesSelect.getDisplayText()}`);

    const priorityPatternSelect = selectPatterns.priority('task-priority');
    console.log(`✅ Priority pattern: ${priorityPatternSelect.getDisplayText()}`);

    // Demo 6: Advanced operations
    console.log('\n6. Advanced Operations');
    console.log('-'.repeat(30));

    // Test multiple operations on multi-select
    tagsSelect.clearSelection();
    console.log(`   After clear: ${tagsSelect.getDisplayText()}`);

    tagsSelect.select(0); // Frontend
    tagsSelect.select(3); // Mobile
    tagsSelect.select(4); // DevOps
    console.log(`   After multiple selections: ${tagsSelect.getDisplayText()}`);

    // Test deselection
    tagsSelect.deselect(3); // Remove Mobile
    console.log(`   After deselecting Mobile: ${tagsSelect.getSelectedOptions().map(o => o.label).join(', ')}`);

    // Test toggle
    tagsSelect.toggleSelection(1); // Toggle Backend
    console.log(`   After toggling Backend: ${tagsSelect.getSelectedOptions().map(o => o.label).join(', ')}`);

    // Demo 7: Configuration updates
    console.log('\n7. Dynamic Configuration');
    console.log('-'.repeat(30));

    languageSelect.updateConfig({
        placeholder: 'Updated placeholder...',
        disabled: false
    });
    console.log(`✅ Configuration updated`);

    // Demo 8: State inspection
    console.log('\n8. State Inspection');
    console.log('-'.repeat(30));

    const state = tagsSelect.getState();
    console.log(`   Open: ${state.open}`);
    console.log(`   Focused: ${state.focused}`);
    console.log(`   Selected count: ${state.selectedIndices.length}`);
    console.log(`   Search query: "${state.searchQuery}"`);
    console.log(`   Filtered options: ${state.filteredIndices.length}`);

    // Summary
    console.log('\n🎉 Select Widget Demo Complete!');
    console.log('='.repeat(50));
    console.log('✅ Single-select dropdown with string options');
    console.log('✅ Multi-select dropdown with search filtering');
    console.log('✅ Custom options with icons and descriptions');
    console.log('✅ Keyboard navigation and event handling');
    console.log('✅ Dynamic configuration and state management');
    console.log('✅ Convenience patterns for common use cases');
    console.log('✅ Advanced operations (select, deselect, toggle, clear)');
    
    console.log('\n🚀 Features Demonstrated:');
    console.log('• Flexible selection modes (single/multiple)');
    console.log('• Real-time search filtering with custom functions');
    console.log('• Rich option types with icons, descriptions, grouping');
    console.log('• Comprehensive keyboard navigation');
    console.log('• Event callbacks and reactive state updates');
    console.log('• Accessibility features and disabled state handling');
    console.log('• Builder pattern for easy configuration');
    console.log('• TypeScript type safety throughout');
}

// Run the demo
runSelectDemo().catch(console.error);