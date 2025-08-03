/**
 * Grid Component Builder - Integrated with TUI Framework
 * 
 * Creates grid layout elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system instead of independent rendering.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum GridColumns {
    One = '1',
    Two = '2', 
    Three = '3',
    Four = '4',
    Five = '5',
    Six = '6',
    Seven = '7',
    Eight = '8',
    Nine = '9',
    Ten = '10',
    Eleven = '11',
    Twelve = '12'
}

export enum GridFlow {
    Row = 'row',
    Column = 'column',
    Dense = 'dense'
}

export enum GridAlign {
    Start = 'start',
    End = 'end',
    Center = 'center',
    Stretch = 'stretch'
}

export interface GridItemConfig {
    id?: string;
    content: string;
    column?: number;
    row?: number;
    colSpan?: number;
    rowSpan?: number;
    backgroundColor?: string;
    borderColor?: string;
    textColor?: string;
    classes?: string[];
}

export interface GridConfig {
    id?: string;
    columns: GridColumns | string;
    rows?: GridColumns | string;
    gap?: number;
    columnGap?: number;
    rowGap?: number;
    items: GridItemConfig[];
    flow?: GridFlow;
    alignItems?: GridAlign;
    justifyItems?: GridAlign;
    classes?: string[];
}

/**
 * Create a grid layout element using the component builder pattern
 */
export function grid(config: GridConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('div');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration - matching Rust backend expectations
    const classes = ['grid'];
    
    // Grid columns class
    if (typeof config.columns === 'string') {
        classes.push(`grid-cols-${config.columns}`);
    } else {
        classes.push(`grid-cols-${config.columns}`);
    }
    
    // Grid rows class
    if (config.rows) {
        if (typeof config.rows === 'string') {
            classes.push(`grid-rows-${config.rows}`);
        } else {
            classes.push(`grid-rows-${config.rows}`);
        }
    }
    
    // Gap classes
    if (config.gap !== undefined) {
        classes.push(`gap-${config.gap}`);
    }
    if (config.columnGap !== undefined) {
        classes.push(`gap-x-${config.columnGap}`);
    }
    if (config.rowGap !== undefined) {
        classes.push(`gap-y-${config.rowGap}`);
    }
    
    // Flow classes
    if (config.flow) {
        switch (config.flow) {
            case GridFlow.Row:
                classes.push('grid-flow-row');
                break;
            case GridFlow.Column:
                classes.push('grid-flow-col');
                break;
            case GridFlow.Dense:
                classes.push('grid-flow-row-dense');
                break;
        }
    }
    
    // Custom classes
    if (config.classes) {
        classes.push(...config.classes);
    }
    
    builder.classes(classes);
    
    // Create child elements for grid items
    config.items.forEach((item, _index) => {
        const itemBuilder = new ElementBuilderImpl('div');
        
        if (item.id) {
            itemBuilder.id(item.id);
        }
        
        const itemClasses = [];
        
        // Column span classes
        if (item.colSpan !== undefined) {
            itemClasses.push(`col-span-${item.colSpan}`);
        }
        
        // Row span classes
        if (item.rowSpan !== undefined) {
            itemClasses.push(`row-span-${item.rowSpan}`);
        }
        
        // Column start/end classes
        if (item.column !== undefined) {
            itemClasses.push(`col-start-${item.column}`);
        }
        
        // Row start/end classes
        if (item.row !== undefined) {
            itemClasses.push(`row-start-${item.row}`);
        }
        
        // Custom item classes
        if (item.classes) {
            itemClasses.push(...item.classes);
        }
        
        itemBuilder.classes(itemClasses);
        
        // Set styling attributes for Rust backend processing
        if (item.backgroundColor) itemBuilder.attr('data-bg-color', item.backgroundColor);
        if (item.borderColor) itemBuilder.attr('data-border-color', item.borderColor);
        if (item.textColor) itemBuilder.attr('data-text-color', item.textColor);
        
        // Set content
        itemBuilder.content(item.content);
        
        builder.child(itemBuilder);
    });
    
    return builder;
}

/**
 * Simple grid with specified columns and rows
 */
export function simpleGrid(columns: GridColumns, rows: GridColumns, items: GridItemConfig[]): ElementBuilder {
    return grid({
        columns,
        rows,
        items
    });
}

/**
 * Auto-sizing grid that arranges items in columns
 */
export function autoGrid(columns: GridColumns, items: GridItemConfig[]): ElementBuilder {
    return grid({
        columns,
        items
    });
}

/**
 * Builder pattern for complex grid configurations
 */
export class GridBuilder {
    private config: Partial<GridConfig> = {
        items: []
    };
    
    public static create(): GridBuilder {
        return new GridBuilder();
    }
    
    public id(id: string): this {
        this.config.id = id;
        return this;
    }
    
    public columns(columns: GridColumns | string): this {
        this.config.columns = columns;
        return this;
    }
    
    public rows(rows: GridColumns | string): this {
        this.config.rows = rows;
        return this;
    }
    
    public gap(gap: number): this {
        this.config.gap = gap;
        return this;
    }
    
    public columnGap(gap: number): this {
        this.config.columnGap = gap;
        return this;
    }
    
    public rowGap(gap: number): this {
        this.config.rowGap = gap;
        return this;
    }
    
    public flow(flow: GridFlow): this {
        this.config.flow = flow;
        return this;
    }
    
    public alignItems(align: GridAlign): this {
        this.config.alignItems = align;
        return this;
    }
    
    public justifyItems(justify: GridAlign): this {
        this.config.justifyItems = justify;
        return this;
    }
    
    public classes(classes: string[]): this {
        this.config.classes = classes;
        return this;
    }
    
    public addItem(item: GridItemConfig): this {
        if (!this.config.items) {
            this.config.items = [];
        }
        this.config.items.push(item);
        return this;
    }
    
    public items(items: GridItemConfig[]): this {
        this.config.items = items;
        return this;
    }
    
    public build(): ElementBuilder {
        // Validate required fields
        if (!this.config.columns) {
            throw new Error('Grid requires columns');
        }
        if (!this.config.items || this.config.items.length === 0) {
            throw new Error('Grid requires at least one item');
        }
        
        return grid(this.config as GridConfig);
    }
}

// Color palette for grid items
export const GRID_COLORS = {
    blue: { backgroundColor: '#1e3a8a', borderColor: '#3b82f6', textColor: '#dbeafe' },
    green: { backgroundColor: '#166534', borderColor: '#10b981', textColor: '#d1fae5' },
    red: { backgroundColor: '#991b1b', borderColor: '#ef4444', textColor: '#fecaca' },
    yellow: { backgroundColor: '#a16207', borderColor: '#f59e0b', textColor: '#fef3c7' },
    purple: { backgroundColor: '#7c2d92', borderColor: '#a855f7', textColor: '#e9d5ff' },
    gray: { backgroundColor: '#374151', borderColor: '#6b7280', textColor: '#f3f4f6' },
    cyan: { backgroundColor: '#155e75', borderColor: '#06b6d4', textColor: '#cffafe' },
    pink: { backgroundColor: '#be185d', borderColor: '#ec4899', textColor: '#fce7f3' }
};

// Convenience functions
export function createGrid(): GridBuilder {
    return GridBuilder.create();
}

export function createColoredGrid(columns: GridColumns = GridColumns.Three): ElementBuilder {
    return GridBuilder.create()
        .columns(columns)
        .gap(1)
        .items([
            { content: 'Panel 1', ...GRID_COLORS.blue },
            { content: 'Panel 2', ...GRID_COLORS.green },
            { content: 'Panel 3', ...GRID_COLORS.red },
            { content: 'Panel 4', ...GRID_COLORS.yellow },
            { content: 'Panel 5', ...GRID_COLORS.purple },
            { content: 'Panel 6', ...GRID_COLORS.cyan }
        ])
        .build();
}