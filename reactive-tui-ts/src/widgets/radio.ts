/**
 * Radio Button Component Builder - Integrated with TUI Framework
 * 
 * Creates radio button group elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system for single selection from groups.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum RadioOrientation {
    Vertical = 'vertical',
    Horizontal = 'horizontal'
}

export interface RadioOption {
    value: string;
    label: string;
    enabled?: boolean;
    description?: string;
}

export interface RadioConfig {
    id?: string;
    options: RadioOption[];
    selected?: string;
    interactive?: boolean;
    orientation?: RadioOrientation;
    selectedChar?: string;
    unselectedChar?: string;
    spacing?: number;
    showLabels?: boolean;
    classes?: string[];
}

export interface RadioState {
    selected: string | null;
    interactive: boolean;
    focusedIndex: number;
}

/**
 * Create a radio button group element using the component builder pattern
 */
export function radioGroup(config: RadioConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('radiogroup');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration
    const classes = ['radio-group'];
    
    if (config.orientation === RadioOrientation.Horizontal) {
        classes.push('radio-horizontal');
    } else {
        classes.push('radio-vertical');
    }
    
    if (!config.interactive) {
        classes.push('radio-disabled');
    }
    
    // Add custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }
    
    builder.classes(classes);
    
    // Set ARIA attributes for accessibility
    builder.attr('role', 'radiogroup');
    
    if (config.selected) {
        builder.attr('aria-activedescendant', config.selected);
    }
    
    // Set focusable state
    builder.focusable(config.interactive !== false);
    
    // Generate radio group content
    const content = generateRadioContent(config);
    builder.content(content);
    
    // Add data attributes for styling and behavior
    builder.attr('data-selected', config.selected || '');
    builder.attr('data-interactive', config.interactive !== false ? 'true' : 'false');
    builder.attr('data-orientation', config.orientation || RadioOrientation.Vertical);
    
    return builder;
}

/**
 * Generate the visual content for the radio group
 */
function generateRadioContent(config: RadioConfig): string {
    const selectedChar = config.selectedChar || '●';
    const unselectedChar = config.unselectedChar || '○';
    const spacing = ' '.repeat(config.spacing || 1);
    const showLabels = config.showLabels !== false;
    const orientation = config.orientation || RadioOrientation.Vertical;
    
    const parts: string[] = [];
    
    for (const option of config.options) {
        const isSelected = config.selected === option.value;
        const radioChar = isSelected ? selectedChar : unselectedChar;
        
        let optionText = radioChar;
        if (showLabels) {
            optionText += spacing + option.label;
        }
        
        parts.push(optionText);
    }
    
    return orientation === RadioOrientation.Vertical 
        ? parts.join('\n')
        : parts.join('  ');
}

/**
 * Create a simple radio group with string options
 */
export function createRadioGroup(
    id: string, 
    options: string[], 
    selected?: string
): ElementBuilder {
    const radioOptions: RadioOption[] = options.map(option => ({
        value: option,
        label: option,
        enabled: true
    }));
    
    return radioGroup({
        id,
        options: radioOptions,
        selected,
        interactive: true,
        orientation: RadioOrientation.Vertical
    });
}

/**
 * Create a horizontal radio group
 */
export function createHorizontalRadioGroup(
    id: string,
    options: RadioOption[],
    selected?: string
): ElementBuilder {
    return radioGroup({
        id,
        options,
        selected,
        interactive: true,
        orientation: RadioOrientation.Horizontal
    });
}

/**
 * Create a custom styled radio group
 */
export function createCustomRadioGroup(config: {
    id: string;
    options: RadioOption[];
    selected?: string;
    style?: 'default' | 'unicode' | 'symbols';
    orientation?: RadioOrientation;
}): ElementBuilder {
    let chars: { selected: string; unselected: string };
    
    switch (config.style) {
        case 'unicode':
            chars = { selected: '◉', unselected: '◯' };
            break;
        case 'symbols':
            chars = { selected: '✓', unselected: '○' };
            break;
        default:
            chars = { selected: '●', unselected: '○' };
    }
    
    return radioGroup({
        id: config.id,
        options: config.options,
        selected: config.selected,
        selectedChar: chars.selected,
        unselectedChar: chars.unselected,
        orientation: config.orientation || RadioOrientation.Vertical,
        interactive: true,
        showLabels: true,
        classes: ['radio-custom', `radio-${config.style || 'default'}`]
    });
}

/**
 * Create a form-style radio group with descriptions
 */
export function createFormRadioGroup(config: {
    id: string;
    options: Array<{ value: string; label: string; description?: string }>;
    selected?: string;
    title?: string;
}): ElementBuilder {
    const radioOptions: RadioOption[] = config.options.map(opt => ({
        value: opt.value,
        label: opt.label,
        description: opt.description,
        enabled: true
    }));
    
    const builder = radioGroup({
        id: config.id,
        options: radioOptions,
        selected: config.selected,
        interactive: true,
        orientation: RadioOrientation.Vertical,
        classes: ['radio-form']
    });
    
    if (config.title) {
        builder.attr('aria-label', config.title);
    }
    
    return builder;
}

/**
 * Create a compact radio group without labels
 */
export function createCompactRadioGroup(
    id: string,
    optionCount: number,
    selected?: number
): ElementBuilder {
    const options: RadioOption[] = Array.from({ length: optionCount }, (_, i) => ({
        value: i.toString(),
        label: `Option ${i + 1}`,
        enabled: true
    }));
    
    return radioGroup({
        id,
        options,
        selected: selected?.toString(),
        interactive: true,
        showLabels: false,
        orientation: RadioOrientation.Horizontal,
        spacing: 0,
        classes: ['radio-compact']
    });
}

/**
 * Create a disabled radio group
 */
export function createDisabledRadioGroup(
    id: string,
    options: RadioOption[],
    selected?: string
): ElementBuilder {
    return radioGroup({
        id,
        options,
        selected,
        interactive: false,
        orientation: RadioOrientation.Vertical,
        classes: ['radio-disabled']
    });
}

/**
 * Create a yes/no radio group
 */
export function createYesNoRadio(id: string, selected?: boolean): ElementBuilder {
    return radioGroup({
        id,
        options: [
            { value: 'yes', label: 'Yes', enabled: true },
            { value: 'no', label: 'No', enabled: true }
        ],
        selected: selected === true ? 'yes' : selected === false ? 'no' : undefined,
        interactive: true,
        orientation: RadioOrientation.Horizontal,
        classes: ['radio-yesno']
    });
}

/**
 * Create a rating radio group (1-5 stars)
 */
export function createRatingRadio(id: string, rating?: number): ElementBuilder {
    const options: RadioOption[] = Array.from({ length: 5 }, (_, i) => ({
        value: (i + 1).toString(),
        label: '★'.repeat(i + 1),
        enabled: true
    }));
    
    return radioGroup({
        id,
        options,
        selected: rating?.toString(),
        selectedChar: '★',
        unselectedChar: '☆',
        interactive: true,
        orientation: RadioOrientation.Horizontal,
        spacing: 0,
        classes: ['radio-rating']
    });
}

// Types already exported above via export interface declarations
