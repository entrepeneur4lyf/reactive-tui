/**
 * Spinner Widget Component Builder - Integrated with TUI Framework
 * 
 * Creates animated spinner elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system for loading states and progress indication.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum SpinnerType {
    // Classic braille dot spinners
    Dots = 'dots',
    Dots2 = 'dots2',
    
    // Simple character spinners
    Line = 'line',
    Pipe = 'pipe',
    SimpleDots = 'simpleDots',
    SimpleDotsScrolling = 'simpleDotsScrolling',
    
    // Star animations
    Star = 'star',
    Star2 = 'star2',
    
    // Geometric shapes
    Arc = 'arc',
    Circle = 'circle',
    CircleQuarters = 'circleQuarters',
    CircleHalves = 'circleHalves',
    SquareCorners = 'squareCorners',
    Triangle = 'triangle',
    
    // Toggle animations
    Toggle = 'toggle',
    Toggle2 = 'toggle2',
    Toggle3 = 'toggle3',
    
    // Movement animations
    Bounce = 'bounce',
    BoxBounce = 'boxBounce',
    GrowVertical = 'growVertical',
    GrowHorizontal = 'growHorizontal',
    Balloon = 'balloon',
    Noise = 'noise',
    Arrow = 'arrow',
    BouncingBar = 'bouncingBar',
    BouncingBall = 'bouncingBall',
    
    // Emoji spinners
    Hearts = 'hearts',
    Clock = 'clock',
    Earth = 'earth',
    Moon = 'moon',
    Weather = 'weather',
    Smiley = 'smiley',
    Monkey = 'monkey',
    Runner = 'runner',
    Christmas = 'christmas'
}

export enum SpinnerLabelPosition {
    Before = 'before',
    After = 'after',
    Above = 'above',
    Below = 'below',
    None = 'none'
}

export enum SpinnerAnimationState {
    Running = 'running',
    Paused = 'paused',
    Stopped = 'stopped'
}

export interface SpinnerDefinition {
    frames: string[];
    interval: number;
    name?: string;
}

export interface SpinnerConfig {
    id?: string;
    type?: SpinnerType;
    customDefinition?: SpinnerDefinition;
    label?: string;
    labelPosition?: SpinnerLabelPosition;
    spacing?: number;
    prefix?: string;
    suffix?: string;
    showSpinner?: boolean;
    visible?: boolean;
    animationState?: SpinnerAnimationState;
    currentFrame?: number;
    classes?: string[];
}

export interface SpinnerState {
    animationState: SpinnerAnimationState;
    currentFrame: number;
    visible: boolean;
    lastUpdate?: number;
}

/**
 * Get predefined spinner definition
 */
function getSpinnerDefinition(type: SpinnerType): SpinnerDefinition {
    const definitions: Record<SpinnerType, SpinnerDefinition> = {
        [SpinnerType.Dots]: {
            frames: ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏'],
            interval: 80,
            name: 'dots'
        },
        [SpinnerType.Dots2]: {
            frames: ['⣾', '⣽', '⣻', '⢿', '⡿', '⣟', '⣯', '⣷'],
            interval: 80,
            name: 'dots2'
        },
        [SpinnerType.Line]: {
            frames: ['-', '\\', '|', '/'],
            interval: 130,
            name: 'line'
        },
        [SpinnerType.Pipe]: {
            frames: ['┤', '┘', '┴', '└', '├', '┌', '┬', '┐'],
            interval: 100,
            name: 'pipe'
        },
        [SpinnerType.SimpleDots]: {
            frames: ['.  ', '.. ', '...', '   '],
            interval: 400,
            name: 'simpleDots'
        },
        [SpinnerType.SimpleDotsScrolling]: {
            frames: ['.  ', '.. ', '...', ' ..', '  .', '   '],
            interval: 200,
            name: 'simpleDotsScrolling'
        },
        [SpinnerType.Star]: {
            frames: ['✶', '✸', '✹', '✺', '✹', '✷'],
            interval: 70,
            name: 'star'
        },
        [SpinnerType.Star2]: {
            frames: ['+', 'x', '*'],
            interval: 80,
            name: 'star2'
        },
        [SpinnerType.Arc]: {
            frames: ['◜', '◠', '◝', '◞', '◡', '◟'],
            interval: 100,
            name: 'arc'
        },
        [SpinnerType.Circle]: {
            frames: ['◡', '⊙', '◠'],
            interval: 120,
            name: 'circle'
        },
        [SpinnerType.CircleQuarters]: {
            frames: ['◴', '◷', '◶', '◵'],
            interval: 120,
            name: 'circleQuarters'
        },
        [SpinnerType.CircleHalves]: {
            frames: ['◐', '◓', '◑', '◒'],
            interval: 50,
            name: 'circleHalves'
        },
        [SpinnerType.SquareCorners]: {
            frames: ['◰', '◳', '◲', '◱'],
            interval: 180,
            name: 'squareCorners'
        },
        [SpinnerType.Triangle]: {
            frames: ['◢', '◣', '◤', '◥'],
            interval: 50,
            name: 'triangle'
        },
        [SpinnerType.Toggle]: {
            frames: ['⊶', '⊷'],
            interval: 250,
            name: 'toggle'
        },
        [SpinnerType.Toggle2]: {
            frames: ['▫', '▪'],
            interval: 80,
            name: 'toggle2'
        },
        [SpinnerType.Toggle3]: {
            frames: ['□', '■'],
            interval: 120,
            name: 'toggle3'
        },
        [SpinnerType.Bounce]: {
            frames: ['⠁', '⠂', '⠄', '⠂'],
            interval: 120,
            name: 'bounce'
        },
        [SpinnerType.BoxBounce]: {
            frames: ['▖', '▘', '▝', '▗'],
            interval: 120,
            name: 'boxBounce'
        },
        [SpinnerType.GrowVertical]: {
            frames: ['▁', '▃', '▄', '▅', '▆', '▇', '▆', '▅', '▄', '▃'],
            interval: 120,
            name: 'growVertical'
        },
        [SpinnerType.GrowHorizontal]: {
            frames: ['▏', '▎', '▍', '▌', '▋', '▊', '▉', '▊', '▋', '▌', '▍', '▎'],
            interval: 120,
            name: 'growHorizontal'
        },
        [SpinnerType.Balloon]: {
            frames: [' ', '.', 'o', 'O', '@', '*', ' '],
            interval: 140,
            name: 'balloon'
        },
        [SpinnerType.Noise]: {
            frames: ['▓', '▒', '░'],
            interval: 100,
            name: 'noise'
        },
        [SpinnerType.Arrow]: {
            frames: ['←', '↖', '↑', '↗', '→', '↘', '↓', '↙'],
            interval: 100,
            name: 'arrow'
        },
        [SpinnerType.BouncingBar]: {
            frames: [
                '[    ]', '[=   ]', '[==  ]', '[=== ]', '[====]',
                '[ ===]', '[  ==]', '[   =]', '[    ]', '[   =]',
                '[  ==]', '[ ===]', '[====]', '[=== ]', '[==  ]', '[=   ]'
            ],
            interval: 80,
            name: 'bouncingBar'
        },
        [SpinnerType.BouncingBall]: {
            frames: [
                '( ●    )', '(  ●   )', '(   ●  )', '(    ● )', '(     ●)',
                '(    ● )', '(   ●  )', '(  ●   )', '( ●    )', '(●     )'
            ],
            interval: 80,
            name: 'bouncingBall'
        },
        [SpinnerType.Hearts]: {
            frames: ['💛 ', '💙 ', '💜 ', '💚 ', '❤️ '],
            interval: 100,
            name: 'hearts'
        },
        [SpinnerType.Clock]: {
            frames: ['🕛 ', '🕐 ', '🕑 ', '🕒 ', '🕓 ', '🕔 ', '🕕 ', '🕖 ', '🕗 ', '🕘 ', '🕙 ', '🕚 '],
            interval: 100,
            name: 'clock'
        },
        [SpinnerType.Earth]: {
            frames: ['🌍 ', '🌎 ', '🌏 '],
            interval: 180,
            name: 'earth'
        },
        [SpinnerType.Moon]: {
            frames: ['🌑 ', '🌒 ', '🌓 ', '🌔 ', '🌕 ', '🌖 ', '🌗 ', '🌘 '],
            interval: 80,
            name: 'moon'
        },
        [SpinnerType.Weather]: {
            frames: [
                '☀️ ', '☀️ ', '☀️ ', '🌤 ', '⛅️ ', '🌥 ', '☁️ ', '🌧 ', '🌨 ', '🌧 ', '🌨 ', '🌧 ', '🌨 ',
                '⛈ ', '🌨 ', '🌧 ', '🌨 ', '☁️ ', '🌥 ', '⛅️ ', '🌤 ', '☀️ ', '☀️ '
            ],
            interval: 100,
            name: 'weather'
        },
        [SpinnerType.Smiley]: {
            frames: ['😄 ', '😝 '],
            interval: 200,
            name: 'smiley'
        },
        [SpinnerType.Monkey]: {
            frames: ['🙈 ', '🙈 ', '🙉 ', '🙊 '],
            interval: 300,
            name: 'monkey'
        },
        [SpinnerType.Runner]: {
            frames: ['🚶 ', '🏃 '],
            interval: 140,
            name: 'runner'
        },
        [SpinnerType.Christmas]: {
            frames: ['🌲', '🎄'],
            interval: 400,
            name: 'christmas'
        }
    };

    return definitions[type];
}

/**
 * Generate spinner content with label positioning
 */
function generateSpinnerContent(config: SpinnerConfig, currentFrame: string): string {
    if (!config.visible) {
        return '';
    }

    let result = '';

    // Add prefix
    if (config.prefix) {
        result += config.prefix;
    }

    // Handle label positioning
    const spacing = ' '.repeat(config.spacing || 1);
    
    switch (config.labelPosition) {
        case SpinnerLabelPosition.Before:
            if (config.label) {
                result += config.label + spacing;
            }
            break;
        case SpinnerLabelPosition.Above:
            if (config.label) {
                result += config.label + '\n';
            }
            break;
    }

    // Add spinner if visible
    if (config.showSpinner !== false) {
        result += currentFrame;
    }

    // Handle label positioning (after/below)
    switch (config.labelPosition) {
        case SpinnerLabelPosition.After:
            if (config.label) {
                result += spacing + config.label;
            }
            break;
        case SpinnerLabelPosition.Below:
            if (config.label) {
                result += '\n' + config.label;
            }
            break;
    }

    // Add suffix
    if (config.suffix) {
        result += config.suffix;
    }

    return result;
}

/**
 * Create a spinner element using the component builder pattern
 */
export function spinnerWidget(config: SpinnerConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('spinner');

    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }

    // Get spinner definition
    const definition = config.customDefinition ||
        (config.type ? getSpinnerDefinition(config.type) : getSpinnerDefinition(SpinnerType.Dots));

    // Get current frame
    const currentFrame = definition.frames[config.currentFrame || 0] || definition.frames[0] || '';

    // Generate content
    const content = generateSpinnerContent(config, currentFrame);
    builder.content(content);

    // Apply CSS classes
    const classes = ['spinner'];

    // Add animation state classes
    switch (config.animationState) {
        case SpinnerAnimationState.Running:
            classes.push('spinner-running');
            break;
        case SpinnerAnimationState.Paused:
            classes.push('spinner-paused');
            break;
        case SpinnerAnimationState.Stopped:
            classes.push('spinner-stopped');
            break;
    }

    if (!config.visible) {
        classes.push('spinner-hidden');
    }

    // Add custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }

    builder.classes(classes);

    // Set ARIA attributes for accessibility
    builder.attr('role', 'status');
    builder.attr('aria-live', 'polite');

    // Add accessibility label
    if (config.label) {
        builder.attr('aria-label', `Loading: ${config.label}`);
    } else {
        builder.attr('aria-label', 'Loading');
    }

    // Add data attributes for state and behavior
    builder.attr('data-animation-state', config.animationState || SpinnerAnimationState.Stopped);
    builder.attr('data-current-frame', (config.currentFrame || 0).toString());
    builder.attr('data-visible', (config.visible !== false).toString());

    if (definition.name) {
        builder.attr('data-spinner-type', definition.name);
    }

    return builder;
}

/**
 * Create a loading spinner with default settings
 */
export function createLoadingSpinner(config: {
    id: string;
    label?: string;
    type?: SpinnerType;
    classes?: string[];
}): ElementBuilder {
    return spinnerWidget({
        id: config.id,
        type: config.type || SpinnerType.Dots,
        label: config.label || 'Loading...',
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Running,
        visible: true,
        classes: config.classes
    });
}

/**
 * Create a processing spinner
 */
export function createProcessingSpinner(id: string, label?: string): ElementBuilder {
    return spinnerWidget({
        id,
        type: SpinnerType.Arc,
        label: label || 'Processing...',
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Running,
        visible: true
    });
}

/**
 * Create a saving spinner
 */
export function createSavingSpinner(id: string, label?: string): ElementBuilder {
    return spinnerWidget({
        id,
        type: SpinnerType.CircleHalves,
        label: label || 'Saving...',
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Running,
        visible: true
    });
}

/**
 * Create a custom spinner with specific configuration
 */
export function createCustomSpinner(config: {
    id: string;
    type?: SpinnerType;
    customDefinition?: SpinnerDefinition;
    label?: string;
    labelPosition?: SpinnerLabelPosition;
    prefix?: string;
    suffix?: string;
}): ElementBuilder {
    return spinnerWidget({
        id: config.id,
        type: config.type || SpinnerType.Dots,
        customDefinition: config.customDefinition,
        label: config.label,
        labelPosition: config.labelPosition || SpinnerLabelPosition.After,
        prefix: config.prefix,
        suffix: config.suffix,
        animationState: SpinnerAnimationState.Running,
        visible: true
    });
}

/**
 * Create an emoji spinner
 */
export function createEmojiSpinner(config: {
    id: string;
    type: SpinnerType.Hearts | SpinnerType.Clock | SpinnerType.Earth | SpinnerType.Moon | SpinnerType.Weather | SpinnerType.Smiley | SpinnerType.Monkey | SpinnerType.Runner | SpinnerType.Christmas;
    label?: string;
}): ElementBuilder {
    return spinnerWidget({
        id: config.id,
        type: config.type,
        label: config.label,
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Running,
        visible: true,
        classes: ['spinner-emoji']
    });
}

/**
 * Create a minimal spinner without labels
 */
export function createMinimalSpinner(id: string, type: SpinnerType = SpinnerType.Dots): ElementBuilder {
    return spinnerWidget({
        id,
        type,
        labelPosition: SpinnerLabelPosition.None,
        animationState: SpinnerAnimationState.Running,
        visible: true,
        classes: ['spinner-minimal']
    });
}

/**
 * Create a progress-style spinner
 */
export function createProgressSpinner(id: string, label?: string): ElementBuilder {
    const customDefinition: SpinnerDefinition = {
        frames: ['▰▱▱▱▱', '▰▰▱▱▱', '▰▰▰▱▱', '▰▰▰▰▱', '▰▰▰▰▰', '▱▰▰▰▰', '▱▱▰▰▰', '▱▱▱▰▰', '▱▱▱▱▰', '▱▱▱▱▱'],
        interval: 120,
        name: 'progress'
    };

    return spinnerWidget({
        id,
        customDefinition,
        label: label || 'Progress...',
        labelPosition: SpinnerLabelPosition.Before,
        animationState: SpinnerAnimationState.Running,
        visible: true,
        classes: ['spinner-progress']
    });
}

/**
 * Create a binary-style spinner
 */
export function createBinarySpinner(id: string, label?: string): ElementBuilder {
    const customDefinition: SpinnerDefinition = {
        frames: ['010010', '001100', '100101', '111010', '111101', '010111', '101011', '111000'],
        interval: 80,
        name: 'binary'
    };

    return spinnerWidget({
        id,
        customDefinition,
        label: label || 'Computing...',
        labelPosition: SpinnerLabelPosition.After,
        animationState: SpinnerAnimationState.Running,
        visible: true,
        classes: ['spinner-binary']
    });
}

// Types already exported above via export interface declarations
