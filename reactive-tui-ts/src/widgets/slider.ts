/**
 * Slider Component Builder - Integrated with TUI Framework
 * 
 * Creates slider elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system instead of independent rendering.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum SliderOrientation {
    Horizontal = 'horizontal',
    Vertical = 'vertical'
}

export enum SliderMode {
    Single = 'single',
    Range = 'range'
}

export interface SliderConfig {
    id?: string;
    mode: SliderMode;
    orientation: SliderOrientation;
    min: number;
    max: number;
    value: number;
    rangeEnd?: number;
    step?: number;
    label?: string;
    description?: string;
    showValues?: boolean;
    showPercentage?: boolean;
    classes?: string[];
}

/**
 * Create a slider element using the component builder pattern
 */
export function slider(config: SliderConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('slider');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration
    const classes = [
        'slider',
        `slider-${config.mode}`,
        `slider-${config.orientation}`
    ];
    if (config.classes) {
        classes.push(...config.classes);
    }
    builder.classes(classes);
    
    // Set attributes for Rust backend
    builder.attr('data-mode', config.mode);
    builder.attr('data-orientation', config.orientation);
    builder.attr('min', config.min.toString());
    builder.attr('max', config.max.toString());
    builder.attr('value', config.value.toString());
    if (config.rangeEnd !== undefined) builder.attr('range-end', config.rangeEnd.toString());
    if (config.step !== undefined) builder.attr('step', config.step.toString());
    if (config.label) builder.attr('label', config.label);
    if (config.description) builder.attr('aria-description', config.description);
    if (config.showValues) builder.attr('show-values', 'true');
    if (config.showPercentage) builder.attr('show-percentage', 'true');
    
    // Accessibility attributes
    builder.attr('role', 'slider');
    builder.attr('aria-valuemin', config.min.toString());
    builder.attr('aria-valuemax', config.max.toString());
    builder.attr('aria-valuenow', config.value.toString());
    builder.attr('aria-orientation', config.orientation);
    if (config.mode === SliderMode.Range) {
        builder.attr('aria-valuetext', `${config.value} to ${config.rangeEnd}`);
    }
    
    return builder;
}

/**
 * Single-value horizontal slider
 */
export function horizontalSlider(props: {
    id?: string;
    min: number;
    max: number;
    value: number;
    step?: number;
    label?: string;
    showValues?: boolean;
    classes?: string[];
}): ElementBuilder {
    return slider({
        ...props,
        mode: SliderMode.Single,
        orientation: SliderOrientation.Horizontal
    });
}

/**
 * Single-value vertical slider
 */
export function verticalSlider(props: {
    id?: string;
    min: number;
    max: number;
    value: number;
    step?: number;
    label?: string;
    showValues?: boolean;
    classes?: string[];
}): ElementBuilder {
    return slider({
        ...props,
        mode: SliderMode.Single,
        orientation: SliderOrientation.Vertical
    });
}

/**
 * Range slider (dual handle)
 */
export function rangeSlider(props: {
    id?: string;
    min: number;
    max: number;
    value: number;
    rangeEnd: number;
    step?: number;
    label?: string;
    showValues?: boolean;
    orientation?: SliderOrientation;
    classes?: string[];
}): ElementBuilder {
    return slider({
        ...props,
        mode: SliderMode.Range,
        orientation: props.orientation || SliderOrientation.Horizontal
    });
}

/**
 * Builder pattern for complex slider configurations
 */
export class SliderBuilder {
    private config: Partial<SliderConfig> = {
        mode: SliderMode.Single,
        orientation: SliderOrientation.Horizontal,
        min: 0,
        max: 100,
        value: 0
    };
    
    public static create(): SliderBuilder {
        return new SliderBuilder();
    }
    
    public id(id: string): this {
        this.config.id = id;
        return this;
    }
    
    public mode(mode: SliderMode): this {
        this.config.mode = mode;
        return this;
    }
    
    public orientation(orientation: SliderOrientation): this {
        this.config.orientation = orientation;
        return this;
    }
    
    public range(min: number, max: number): this {
        this.config.min = min;
        this.config.max = max;
        return this;
    }
    
    public value(value: number): this {
        this.config.value = value;
        return this;
    }
    
    public rangeEnd(end: number): this {
        this.config.rangeEnd = end;
        this.config.mode = SliderMode.Range;
        return this;
    }
    
    public step(step: number): this {
        this.config.step = step;
        return this;
    }
    
    public label(label: string): this {
        this.config.label = label;
        return this;
    }
    
    public description(description: string): this {
        this.config.description = description;
        return this;
    }
    
    public showValues(show: boolean = true): this {
        this.config.showValues = show;
        return this;
    }
    
    public showPercentage(show: boolean = true): this {
        this.config.showPercentage = show;
        return this;
    }
    
    public classes(classes: string[]): this {
        this.config.classes = classes;
        return this;
    }
    
    public horizontal(): this {
        this.config.orientation = SliderOrientation.Horizontal;
        return this;
    }
    
    public vertical(): this {
        this.config.orientation = SliderOrientation.Vertical;
        return this;
    }
    
    public single(): this {
        this.config.mode = SliderMode.Single;
        return this;
    }
    
    public dualRange(start: number, end: number): this {
        this.config.mode = SliderMode.Range;
        this.config.value = start;
        this.config.rangeEnd = end;
        return this;
    }
    
    public build(): ElementBuilder {
        // Validate required fields
        if (this.config.min === undefined || this.config.max === undefined || this.config.value === undefined) {
            throw new Error('Slider requires min, max, and value');
        }
        
        return slider(this.config as SliderConfig);
    }
}

// Convenience functions
export function createSlider(): SliderBuilder {
    return SliderBuilder.create();
}

export function createHorizontalSlider(min: number, max: number, value: number): SliderBuilder {
    return SliderBuilder.create()
        .horizontal()
        .range(min, max)
        .value(value);
}

export function createVerticalSlider(min: number, max: number, value: number): SliderBuilder {
    return SliderBuilder.create()
        .vertical()
        .range(min, max)
        .value(value);
}

export function createRangeSlider(min: number, max: number, start: number, end: number): SliderBuilder {
    return SliderBuilder.create()
        .range(min, max)
        .dualRange(start, end);
}