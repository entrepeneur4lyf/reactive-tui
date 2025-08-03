/**
 * Checkbox Widget Component Builder - Single and Group Checkboxes with Animation
 * 
 * Creates checkbox elements with smooth scaling animations and multi-selection groups.
 * Uses CSS animations for smooth visual feedback when checking/unchecking.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum CheckboxStyle {
    Ballot = 'ballot',      // ☐ ☑
    Square = 'square',      // [ ] [x]
    Round = 'round',        // ( ) (x)
    Custom = 'custom'       // User-defined characters
}

export enum CheckboxLabelPosition {
    Before = 'before',
    After = 'after',
    Above = 'above',
    Below = 'below',
    None = 'none'
}

export enum CheckboxGroupOrientation {
    Vertical = 'vertical',
    Horizontal = 'horizontal'
}

export enum CheckboxAnimationState {
    Idle = 'idle',
    CheckingIn = 'checking-in',
    CheckingOut = 'checking-out'
}

export interface CheckboxAnimationConfig {
    enabled: boolean;
    duration: number;        // milliseconds
    easing: string;         // CSS easing function
    scaleFactor: number;    // How much larger the initial checkmark is
}

export interface CheckboxConfig {
    id?: string;
    label?: string;
    checked?: boolean;
    enabled?: boolean;
    visible?: boolean;
    style?: CheckboxStyle;
    labelPosition?: CheckboxLabelPosition;
    spacing?: number;
    customChars?: { unchecked: string; checked: string };
    animationConfig?: CheckboxAnimationConfig;
    animationState?: CheckboxAnimationState;
    classes?: string[];
}

export interface CheckboxOption {
    id: string;
    label: string;
    value: string;
    checked?: boolean;
    enabled?: boolean;
}

export interface CheckboxGroupConfig {
    id?: string;
    label?: string;
    options: CheckboxOption[];
    style?: CheckboxStyle;
    orientation?: CheckboxGroupOrientation;
    spacing?: number;
    enabled?: boolean;
    visible?: boolean;
    selectedValues?: string[];
    customChars?: { unchecked: string; checked: string };
    classes?: string[];
}

/**
 * Get checkbox character based on style and state
 */
function getCheckboxChar(config: CheckboxConfig): string {
    const { style = CheckboxStyle.Ballot, checked = false, customChars } = config;
    
    switch (style) {
        case CheckboxStyle.Ballot:
            return checked ? '☑' : '☐';
        case CheckboxStyle.Square:
            return checked ? '[x]' : '[ ]';
        case CheckboxStyle.Round:
            return checked ? '(x)' : '( )';
        case CheckboxStyle.Custom:
            if (customChars) {
                return checked ? customChars.checked : customChars.unchecked;
            }
            return checked ? '●' : '○';
        default:
            return checked ? '☑' : '☐';
    }
}

/**
 * Get animated checkbox character for scaling effect
 */
function getAnimatedCheckboxChar(config: CheckboxConfig): string {
    const { style = CheckboxStyle.Ballot, checked = false, animationState } = config;
    
    if (!checked || animationState !== CheckboxAnimationState.CheckingIn) {
        return getCheckboxChar(config);
    }

    // Return larger character during animation
    switch (style) {
        case CheckboxStyle.Ballot:
            return '✅'; // Large heavy check mark
        case CheckboxStyle.Square:
            return '[✓]'; // Large check in square
        case CheckboxStyle.Round:
            return '(✓)'; // Large check in parentheses
        case CheckboxStyle.Custom:
            if (config.customChars) {
                return config.customChars.checked;
            }
            return '●';
        default:
            return '✅';
    }
}

/**
 * Generate checkbox content with label positioning
 */
function generateCheckboxContent(config: CheckboxConfig): string {
    if (config.visible === false) {
        return '';
    }

    const isAnimating = config.animationConfig?.enabled && 
                       config.animationState === CheckboxAnimationState.CheckingIn;
    
    const checkbox = isAnimating ? 
        getAnimatedCheckboxChar(config) : 
        getCheckboxChar(config);
    
    const spacing = ' '.repeat(config.spacing || 1);
    
    if (!config.label) {
        return checkbox;
    }

    switch (config.labelPosition) {
        case CheckboxLabelPosition.Before:
            return `${config.label}${spacing}${checkbox}`;
        case CheckboxLabelPosition.After:
            return `${checkbox}${spacing}${config.label}`;
        case CheckboxLabelPosition.Above:
            return `${config.label}\n${checkbox}`;
        case CheckboxLabelPosition.Below:
            return `${checkbox}\n${config.label}`;
        case CheckboxLabelPosition.None:
            return checkbox;
        default:
            return `${checkbox}${spacing}${config.label}`;
    }
}

/**
 * Create a single checkbox element
 */
export function checkboxWidget(config: CheckboxConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('checkbox');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Generate content
    const content = generateCheckboxContent(config);
    builder.content(content);
    
    // Apply CSS classes
    const classes = ['checkbox'];
    
    // Add state classes
    if (config.checked) {
        classes.push('checkbox-checked');
    } else {
        classes.push('checkbox-unchecked');
    }
    
    if (config.enabled === false) {
        classes.push('checkbox-disabled');
    }
    
    if (config.visible === false) {
        classes.push('checkbox-hidden');
    }
    
    // Add style classes
    if (config.style) {
        classes.push(`checkbox-${config.style}`);
    }
    
    // Add animation classes and data attributes
    if (config.animationConfig?.enabled && config.animationState === CheckboxAnimationState.CheckingIn) {
        classes.push('checkbox-animating');

        // Add animation data attributes for CSS
        const duration = config.animationConfig.duration || 250;
        const easing = config.animationConfig.easing || 'ease-out';
        const scale = config.animationConfig.scaleFactor || 1.5;

        builder.attr('data-animation-duration', duration.toString());
        builder.attr('data-animation-easing', easing);
        builder.attr('data-animation-scale', scale.toString());
    }
    
    // Add custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }
    
    builder.classes(classes);
    
    // Set ARIA attributes for accessibility
    builder.attr('role', 'checkbox');
    builder.attr('aria-checked', (config.checked || false).toString());
    
    if (config.enabled === false) {
        builder.attr('aria-disabled', 'true');
    }
    
    // Add accessibility label
    if (config.label) {
        builder.attr('aria-label', config.label);
    }
    
    // Add data attributes for state
    builder.attr('data-checked', (config.checked || false).toString());
    builder.attr('data-enabled', (config.enabled !== false).toString());
    builder.attr('data-visible', (config.visible !== false).toString());
    
    if (config.animationState) {
        builder.attr('data-animation-state', config.animationState);
    }
    
    return builder;
}

/**
 * Create a checkbox group element
 */
export function checkboxGroupWidget(config: CheckboxGroupConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('checkbox-group');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Generate content
    let content = '';
    
    // Add group label if present
    if (config.label) {
        content += config.label + '\n';
    }
    
    const spacing = ' '.repeat(config.spacing || 1);
    const separator = config.orientation === CheckboxGroupOrientation.Horizontal ? spacing : '\n';
    
    const optionStrings = config.options.map(option => {
        const checkboxConfig: CheckboxConfig = {
            style: config.style,
            checked: option.checked || config.selectedValues?.includes(option.value),
            enabled: option.enabled && config.enabled !== false,
            customChars: config.customChars,
            spacing: 1
        };
        
        const checkbox = getCheckboxChar(checkboxConfig);
        return `${checkbox} ${option.label}`;
    });
    
    content += optionStrings.join(separator);
    builder.content(content);
    
    // Apply CSS classes
    const classes = ['checkbox-group'];
    
    if (config.orientation) {
        classes.push(`checkbox-group-${config.orientation}`);
    }
    
    if (config.enabled === false) {
        classes.push('checkbox-group-disabled');
    }
    
    if (config.visible === false) {
        classes.push('checkbox-group-hidden');
    }
    
    // Add custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }
    
    builder.classes(classes);
    
    // Set ARIA attributes for accessibility
    builder.attr('role', 'group');
    
    if (config.label) {
        builder.attr('aria-label', config.label);
    }
    
    // Add data attributes
    builder.attr('data-total-options', config.options.length.toString());
    builder.attr('data-selected-count', (config.selectedValues?.length || 0).toString());
    builder.attr('data-enabled', (config.enabled !== false).toString());
    builder.attr('data-visible', (config.visible !== false).toString());
    builder.attr('data-orientation', config.orientation || CheckboxGroupOrientation.Vertical);
    
    if (config.selectedValues) {
        builder.attr('data-selected-values', config.selectedValues.join(','));
    }
    
    return builder;
}

/**
 * Create a simple checkbox with default settings
 */
export function createSimpleCheckbox(config: {
    id: string;
    label: string;
    checked?: boolean;
    classes?: string[];
}): ElementBuilder {
    return checkboxWidget({
        id: config.id,
        label: config.label,
        checked: config.checked || false,
        style: CheckboxStyle.Ballot,
        labelPosition: CheckboxLabelPosition.After,
        classes: config.classes,
        animationConfig: {
            enabled: true,
            duration: 250,
            easing: 'ease-out',
            scaleFactor: 1.5
        }
    });
}

/**
 * Create an animated checkbox with custom animation settings
 */
export function createAnimatedCheckbox(config: {
    id: string;
    label: string;
    checked?: boolean;
    duration?: number;
    scaleFactor?: number;
    classes?: string[];
    animationConfig?: CheckboxAnimationConfig;
}): ElementBuilder {
    return checkboxWidget({
        id: config.id,
        label: config.label,
        checked: config.checked || false,
        style: CheckboxStyle.Ballot,
        labelPosition: CheckboxLabelPosition.After,
        classes: config.classes,
        animationConfig: config.animationConfig || {
            enabled: true,
            duration: config.duration || 300,
            easing: 'cubic-bezier(0.68, -0.55, 0.265, 1.55)', // Bouncy easing
            scaleFactor: config.scaleFactor || 1.8
        },
        animationState: config.checked ? CheckboxAnimationState.CheckingIn : CheckboxAnimationState.Idle
    });
}

/**
 * Create a custom styled checkbox
 */
export function createCustomCheckbox(config: {
    id: string;
    label: string;
    unchecked: string;
    checkedChar: string;
    checked?: boolean;
}): ElementBuilder {
    return checkboxWidget({
        id: config.id,
        label: config.label,
        checked: config.checked || false,
        style: CheckboxStyle.Custom,
        customChars: {
            unchecked: config.unchecked,
            checked: config.checkedChar
        },
        labelPosition: CheckboxLabelPosition.After
    });
}

/**
 * Create a checkbox group with default settings
 */
export function createCheckboxGroup(config: {
    id: string;
    label: string;
    options: Array<{ label: string; value: string; checked?: boolean }>;
    orientation?: CheckboxGroupOrientation;
    classes?: string[];
}): ElementBuilder {
    const options: CheckboxOption[] = config.options.map((opt, index) => ({
        id: `${config.id}-option-${index}`,
        label: opt.label,
        value: opt.value,
        checked: opt.checked || false,
        enabled: true
    }));

    return checkboxGroupWidget({
        id: config.id,
        label: config.label,
        options,
        style: CheckboxStyle.Ballot,
        orientation: config.orientation || CheckboxGroupOrientation.Vertical,
        spacing: 1,
        classes: config.classes
    });
}

/**
 * Create a horizontal checkbox group
 */
export function createHorizontalCheckboxGroup(config: {
    id: string;
    label: string;
    options: Array<{ label: string; value: string; checked?: boolean }>;
}): ElementBuilder {
    return createCheckboxGroup({
        ...config,
        orientation: CheckboxGroupOrientation.Horizontal
    });
}

/**
 * Create a feature selection checkbox group
 */
export function createFeatureCheckboxGroup(id: string, features: string[]): ElementBuilder {
    const options = features.map(feature => ({
        label: feature,
        value: feature.toLowerCase().replace(/\s+/g, '_')
    }));

    return createCheckboxGroup({
        id,
        label: 'Select features:',
        options
    });
}

/**
 * Create a multi-select checkbox group with pre-selected values
 */
export function createMultiSelectCheckboxGroup(config: {
    id: string;
    label: string;
    options: Array<{ label: string; value: string }>;
    selectedValues: string[];
}): ElementBuilder {
    const options: CheckboxOption[] = config.options.map((opt, index) => ({
        id: `${config.id}-option-${index}`,
        label: opt.label,
        value: opt.value,
        checked: config.selectedValues.includes(opt.value),
        enabled: true
    }));

    return checkboxGroupWidget({
        id: config.id,
        label: config.label,
        options,
        selectedValues: config.selectedValues,
        style: CheckboxStyle.Ballot,
        orientation: CheckboxGroupOrientation.Vertical
    });
}

/**
 * CSS styles for checkbox animations
 */
export const checkboxAnimationCSS = `
/* Checkbox animation keyframes */
@keyframes checkboxCheckIn {
    0% {
        transform: scale(1.8);
        opacity: 0.8;
    }
    50% {
        transform: scale(1.1);
        opacity: 0.9;
    }
    100% {
        transform: scale(1.0);
        opacity: 1.0;
    }
}

/* Animation variants for different scale factors */
@keyframes checkboxCheckInSubtle {
    0% {
        transform: scale(1.3);
        opacity: 0.9;
    }
    100% {
        transform: scale(1.0);
        opacity: 1.0;
    }
}

@keyframes checkboxCheckInBouncy {
    0% {
        transform: scale(2.2);
        opacity: 0.7;
    }
    50% {
        transform: scale(1.2);
        opacity: 0.9;
    }
    100% {
        transform: scale(1.0);
        opacity: 1.0;
    }
}

/* Checkbox base styles */
.checkbox {
    display: inline-block;
    cursor: pointer;
    user-select: none;
    transition: all 0.2s ease;
}

.checkbox:hover {
    opacity: 0.8;
}

.checkbox-disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.checkbox-hidden {
    display: none;
}

.checkbox-animating {
    animation-fill-mode: both;
}

/* Dynamic animations based on data attributes */
.checkbox-animating[data-animation-scale="1.2"] {
    animation: checkboxCheckInSubtle 200ms ease-out;
}

.checkbox-animating[data-animation-scale="1.5"] {
    animation: checkboxCheckIn 250ms ease-out;
}

.checkbox-animating[data-animation-scale="1.8"] {
    animation: checkboxCheckIn 300ms ease-out;
}

.checkbox-animating[data-animation-scale="2.2"] {
    animation: checkboxCheckInBouncy 500ms cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

/* Checkbox group styles */
.checkbox-group {
    display: block;
}

.checkbox-group-horizontal {
    display: flex;
    flex-wrap: wrap;
    gap: 0.5rem;
}

.checkbox-group-vertical .checkbox {
    display: block;
    margin-bottom: 0.25rem;
}

.checkbox-group-disabled {
    opacity: 0.5;
    pointer-events: none;
}

/* Style-specific classes */
.checkbox-ballot {
    font-family: 'Segoe UI Symbol', 'Apple Color Emoji', sans-serif;
}

.checkbox-square {
    font-family: monospace;
}

.checkbox-round {
    font-family: monospace;
}

.checkbox-custom {
    /* Custom styling can be applied here */
}
`;

// Types already exported above via export interface declarations
