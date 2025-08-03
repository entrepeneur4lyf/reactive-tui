/**
 * Switch Toggle Component Builder - Integrated with TUI Framework
 * 
 * Creates switch toggle elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system for boolean state control.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum LabelPosition {
    Before = 'before',
    After = 'after',
    Both = 'both'
}

export interface SwitchConfig {
    id?: string;
    enabled?: boolean;
    interactive?: boolean;
    onLabel?: string;
    offLabel?: string;
    onHandle?: string;
    offHandle?: string;
    trackChar?: string;
    width?: number;
    showLabels?: boolean;
    labelPosition?: LabelPosition;
    description?: string;
    classes?: string[];
}

export interface SwitchState {
    enabled: boolean;
    interactive: boolean;
    focused: boolean;
}

/**
 * Create a switch toggle element using the component builder pattern
 */
export function switchToggle(config: SwitchConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('switch');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration
    const classes = ['switch'];
    
    if (config.enabled) {
        classes.push('switch-on');
    } else {
        classes.push('switch-off');
    }
    
    if (!config.interactive) {
        classes.push('switch-disabled');
    }
    
    // Add custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }
    
    builder.classes(classes);
    
    // Set ARIA attributes for accessibility
    builder.attr('role', 'switch');
    builder.attr('aria-checked', config.enabled ? 'true' : 'false');
    
    if (config.description) {
        builder.attr('aria-label', config.description);
    }
    
    // Set focusable state
    builder.focusable(config.interactive !== false);
    
    // Generate switch content
    const content = generateSwitchContent(config);
    builder.content(content);
    
    // Add data attributes for styling and behavior
    builder.attr('data-enabled', config.enabled ? 'true' : 'false');
    builder.attr('data-interactive', config.interactive !== false ? 'true' : 'false');
    
    if (config.onLabel) {
        builder.attr('data-on-label', config.onLabel);
    }
    
    if (config.offLabel) {
        builder.attr('data-off-label', config.offLabel);
    }
    
    return builder;
}

/**
 * Generate the visual content for the switch
 */
function generateSwitchContent(config: SwitchConfig): string {
    const enabled = config.enabled || false;
    const onHandle = config.onHandle || '●';
    const offHandle = config.offHandle || '○';
    const trackChar = config.trackChar || '─';
    const width = config.width || 8;
    const onLabel = config.onLabel || 'ON';
    const offLabel = config.offLabel || 'OFF';
    const showLabels = config.showLabels !== false;
    const labelPosition = config.labelPosition || LabelPosition.After;
    
    // Create the track
    const trackWidth = Math.max(1, width - 1);
    const handlePos = enabled ? Math.max(0, trackWidth - 1) : 0;
    
    let track = '';
    for (let i = 0; i < trackWidth; i++) {
        if (i === handlePos) {
            track += enabled ? onHandle : offHandle;
        } else {
            track += trackChar;
        }
    }
    
    // Add brackets
    const switchDisplay = `[${track}]`;
    
    // Add labels if enabled
    if (showLabels) {
        const currentLabel = enabled ? onLabel : offLabel;
        
        switch (labelPosition) {
            case LabelPosition.Before:
                return `${currentLabel} ${switchDisplay}`;
            case LabelPosition.After:
                return `${switchDisplay} ${currentLabel}`;
            case LabelPosition.Both:
                const otherLabel = enabled ? offLabel : onLabel;
                return `${otherLabel} ${switchDisplay} ${currentLabel}`;
            default:
                return `${switchDisplay} ${currentLabel}`;
        }
    } else {
        return switchDisplay;
    }
}

/**
 * Create a switch with default settings
 */
export function createSwitch(id: string, enabled: boolean = false): ElementBuilder {
    return switchToggle({
        id,
        enabled,
        interactive: true,
        onLabel: 'ON',
        offLabel: 'OFF',
        showLabels: true,
        labelPosition: LabelPosition.After
    });
}

/**
 * Create a custom styled switch
 */
export function createCustomSwitch(config: {
    id: string;
    enabled?: boolean;
    labels?: { on: string; off: string };
    handles?: { on: string; off: string };
    width?: number;
    position?: LabelPosition;
    description?: string;
}): ElementBuilder {
    return switchToggle({
        id: config.id,
        enabled: config.enabled || false,
        onLabel: config.labels?.on || 'ON',
        offLabel: config.labels?.off || 'OFF',
        onHandle: config.handles?.on || '●',
        offHandle: config.handles?.off || '○',
        width: config.width || 8,
        labelPosition: config.position || LabelPosition.After,
        description: config.description,
        interactive: true,
        showLabels: true
    });
}

/**
 * Create a disabled switch (non-interactive)
 */
export function createDisabledSwitch(id: string, enabled: boolean = false): ElementBuilder {
    return switchToggle({
        id,
        enabled,
        interactive: false,
        onLabel: 'Locked',
        offLabel: 'Unlocked',
        showLabels: true,
        labelPosition: LabelPosition.After,
        classes: ['switch-disabled']
    });
}

/**
 * Create a form-style switch with label positioning
 */
export function createFormSwitch(config: {
    id: string;
    label: string;
    enabled?: boolean;
    description?: string;
}): ElementBuilder {
    return switchToggle({
        id: config.id,
        enabled: config.enabled || false,
        onLabel: config.label,
        offLabel: config.label,
        labelPosition: LabelPosition.Before,
        description: config.description,
        interactive: true,
        showLabels: true,
        classes: ['switch-form']
    });
}

/**
 * Create a compact switch without labels
 */
export function createCompactSwitch(id: string, enabled: boolean = false): ElementBuilder {
    return switchToggle({
        id,
        enabled,
        interactive: true,
        showLabels: false,
        width: 6,
        classes: ['switch-compact']
    });
}

/**
 * Create a Unicode-styled switch
 */
export function createUnicodeSwitch(config: {
    id: string;
    enabled?: boolean;
    style?: 'emoji' | 'symbols' | 'geometric';
}): ElementBuilder {
    let handles: { on: string; off: string };
    
    switch (config.style) {
        case 'emoji':
            handles = { on: '🟢', off: '🔴' };
            break;
        case 'symbols':
            handles = { on: '✓', off: '✗' };
            break;
        case 'geometric':
            handles = { on: '◉', off: '◯' };
            break;
        default:
            handles = { on: '●', off: '○' };
    }
    
    return switchToggle({
        id: config.id,
        enabled: config.enabled || false,
        onHandle: handles.on,
        offHandle: handles.off,
        onLabel: 'Active',
        offLabel: 'Inactive',
        interactive: true,
        showLabels: true,
        labelPosition: LabelPosition.After,
        classes: ['switch-unicode', `switch-${config.style || 'default'}`]
    });
}

// Types already exported above via export interface declarations
