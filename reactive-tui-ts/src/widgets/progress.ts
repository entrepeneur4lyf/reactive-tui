/**
 * Progress Component Builder - Integrated with TUI Framework
 * 
 * Creates progress bar elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system instead of independent rendering.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum ProgressStyle {
    Linear = 'linear',
    Circular = 'circular', 
    Arc = 'arc',
    Spinner = 'spinner'
}

export enum ProgressState {
    Determinate = 'determinate',
    Indeterminate = 'indeterminate'
}

export interface ProgressConfig {
    id?: string;
    style: ProgressStyle;
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    showPercentage?: boolean;
    showValue?: boolean;
    state?: ProgressState;
    spinnerType?: string;
    classes?: string[];
}

/**
 * Create a progress bar element using the component builder pattern
 */
export function progress(config: ProgressConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('progress');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration
    const classes = ['progress', `progress-${config.style}`];
    if (config.state === ProgressState.Indeterminate) {
        classes.push('progress-indeterminate');
    }
    if (config.classes) {
        classes.push(...config.classes);
    }
    builder.classes(classes);
    
    // Set attributes for Rust backend
    builder.attr('data-style', config.style);
    if (config.value !== undefined) builder.attr('value', config.value.toString());
    if (config.min !== undefined) builder.attr('min', config.min.toString());
    if (config.max !== undefined) builder.attr('max', config.max.toString());
    if (config.label) builder.attr('label', config.label);
    if (config.showPercentage) builder.attr('show-percentage', 'true');
    if (config.showValue) builder.attr('show-value', 'true');
    if (config.state) builder.attr('state', config.state);
    if (config.spinnerType) builder.attr('spinner-type', config.spinnerType);
    
    return builder;
}

/**
 * Linear progress bar
 */
export function linearProgress(props: {
    id?: string;
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    showPercentage?: boolean;
    classes?: string[];
}): ElementBuilder {
    return progress({
        ...props,
        style: ProgressStyle.Linear
    });
}

/**
 * Circular progress indicator
 */
export function circularProgress(props: {
    id?: string;
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    classes?: string[];
}): ElementBuilder {
    return progress({
        ...props,
        style: ProgressStyle.Circular
    });
}

/**
 * Arc-style progress indicator
 */
export function arcProgress(props: {
    id?: string;
    value?: number;
    min?: number;
    max?: number;
    label?: string;
    classes?: string[];
}): ElementBuilder {
    return progress({
        ...props,
        style: ProgressStyle.Arc
    });
}

/**
 * Spinner (indeterminate progress)
 */
export function spinner(props: {
    id?: string;
    label?: string;
    spinnerType?: string;
    classes?: string[];
}): ElementBuilder {
    return progress({
        ...props,
        style: ProgressStyle.Spinner,
        state: ProgressState.Indeterminate
    });
}

// Built-in spinner types that the Rust backend will recognize
export const SPINNER_TYPES = {
    dots: 'dots',
    dots2: 'dots2',
    line: 'line',
    simpleDots: 'simple-dots',
    growVertical: 'grow-vertical',
    growHorizontal: 'grow-horizontal',
    clock: 'clock',
    moon: 'moon',
    arrow: 'arrow',
    bouncingBar: 'bouncing-bar',
    circleQuarters: 'circle-quarters',
    triangle: 'triangle',
    hearts: 'hearts',
    weather: 'weather',
    aesthetic: 'aesthetic'
};

/**
 * Builder pattern for complex progress configurations
 */
export class ProgressBuilder {
    private config: ProgressConfig = { style: ProgressStyle.Linear };
    
    public static create(): ProgressBuilder {
        return new ProgressBuilder();
    }
    
    public id(id: string): this {
        this.config.id = id;
        return this;
    }
    
    public style(style: ProgressStyle): this {
        this.config.style = style;
        return this;
    }
    
    public value(value: number): this {
        this.config.value = value;
        return this;
    }
    
    public range(min: number, max: number): this {
        this.config.min = min;
        this.config.max = max;
        return this;
    }
    
    public label(label: string): this {
        this.config.label = label;
        return this;
    }
    
    public showPercentage(show: boolean = true): this {
        this.config.showPercentage = show;
        return this;
    }
    
    public showValue(show: boolean = true): this {
        this.config.showValue = show;
        return this;
    }
    
    public indeterminate(): this {
        this.config.state = ProgressState.Indeterminate;
        return this;
    }
    
    public spinnerType(type: string): this {
        this.config.spinnerType = type;
        return this;
    }
    
    public classes(classes: string[]): this {
        this.config.classes = classes;
        return this;
    }
    
    public build(): ElementBuilder {
        return progress(this.config);
    }
}

// Convenience functions
export function createProgress(style: ProgressStyle = ProgressStyle.Linear): ProgressBuilder {
    return ProgressBuilder.create().style(style);
}

export function createLinearProgress(): ProgressBuilder {
    return ProgressBuilder.create().style(ProgressStyle.Linear);
}

export function createCircularProgress(): ProgressBuilder {
    return ProgressBuilder.create().style(ProgressStyle.Circular);
}

export function createSpinner(type: string = SPINNER_TYPES.dots): ProgressBuilder {
    return ProgressBuilder.create()
        .style(ProgressStyle.Spinner)
        .indeterminate()
        .spinnerType(type);
}