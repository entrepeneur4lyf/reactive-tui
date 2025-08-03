/**
 * Toast Component Builder - Integrated with TUI Framework
 * 
 * Creates toast notification elements that work with the Rust backend via FFI.
 * Uses the existing Element/layout system instead of independent rendering.
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

export enum ToastVariant {
    Info = 'info',
    Success = 'success',
    Warning = 'warning',
    Error = 'error'
}

export enum ToastPosition {
    TopLeft = 'top-left',
    TopCenter = 'top-center',
    TopRight = 'top-right',
    BottomLeft = 'bottom-left',
    BottomCenter = 'bottom-center',
    BottomRight = 'bottom-right'
}

export interface ToastConfig {
    id?: string;
    message: string;
    variant: ToastVariant;
    position?: ToastPosition;
    duration?: number; // milliseconds, 0 = persistent
    dismissible?: boolean;
    showProgress?: boolean;
    showIcon?: boolean;
    showCloseButton?: boolean;
    classes?: string[];
}

/**
 * Create a toast notification element using the component builder pattern
 */
export function toast(config: ToastConfig): ElementBuilder {
    const builder = new ElementBuilderImpl('toast');
    
    // Set element ID
    if (config.id) {
        builder.id(config.id);
    }
    
    // Apply CSS classes based on configuration
    const classes = [
        'toast',
        `toast-${config.variant}`,
        `toast-${config.position || ToastPosition.TopRight}`
    ];
    if (config.dismissible !== false) {
        classes.push('toast-dismissible');
    }
    if (config.showProgress) {
        classes.push('toast-progress');
    }
    if (config.classes) {
        classes.push(...config.classes);
    }
    builder.classes(classes);
    
    // Set attributes for Rust backend
    builder.attr('data-variant', config.variant);
    builder.attr('data-position', config.position || ToastPosition.TopRight);
    builder.attr('message', config.message);
    if (config.duration !== undefined) builder.attr('duration', config.duration.toString());
    if (config.dismissible !== undefined) builder.attr('dismissible', config.dismissible.toString());
    if (config.showProgress) builder.attr('show-progress', 'true');
    if (config.showIcon !== false) builder.attr('show-icon', 'true');
    if (config.showCloseButton !== false) builder.attr('show-close-button', 'true');
    
    // Accessibility attributes
    builder.attr('role', 'alert');
    builder.attr('aria-live', config.variant === ToastVariant.Error ? 'assertive' : 'polite');
    if (config.dismissible !== false) {
        builder.attr('aria-label', `${config.variant} notification: ${config.message}. Dismissible.`);
    }
    
    return builder;
}

/**
 * Info toast notification
 */
export function infoToast(props: {
    id?: string;
    message: string;
    position?: ToastPosition;
    duration?: number;
    dismissible?: boolean;
    classes?: string[];
}): ElementBuilder {
    return toast({
        ...props,
        variant: ToastVariant.Info
    });
}

/**
 * Success toast notification
 */
export function successToast(props: {
    id?: string;
    message: string;
    position?: ToastPosition;
    duration?: number;
    dismissible?: boolean;
    classes?: string[];
}): ElementBuilder {
    return toast({
        ...props,
        variant: ToastVariant.Success
    });
}

/**
 * Warning toast notification
 */
export function warningToast(props: {
    id?: string;
    message: string;
    position?: ToastPosition;
    duration?: number;
    dismissible?: boolean;
    classes?: string[];
}): ElementBuilder {
    return toast({
        ...props,
        variant: ToastVariant.Warning
    });
}

/**
 * Error toast notification
 */
export function errorToast(props: {
    id?: string;
    message: string;
    position?: ToastPosition;
    duration?: number;
    dismissible?: boolean;
    classes?: string[];
}): ElementBuilder {
    return toast({
        ...props,
        variant: ToastVariant.Error
    });
}

/**
 * Builder pattern for complex toast configurations
 */
export class ToastBuilder {
    private config: Partial<ToastConfig> = {
        variant: ToastVariant.Info,
        position: ToastPosition.TopRight,
        duration: 5000,
        dismissible: true,
        showProgress: false,
        showIcon: true,
        showCloseButton: true
    };
    
    public static create(): ToastBuilder {
        return new ToastBuilder();
    }
    
    public id(id: string): this {
        this.config.id = id;
        return this;
    }
    
    public message(message: string): this {
        this.config.message = message;
        return this;
    }
    
    public variant(variant: ToastVariant): this {
        this.config.variant = variant;
        return this;
    }
    
    public position(position: ToastPosition): this {
        this.config.position = position;
        return this;
    }
    
    public duration(duration: number): this {
        this.config.duration = duration;
        return this;
    }
    
    public persistent(): this {
        this.config.duration = 0;
        return this;
    }
    
    public dismissible(dismissible: boolean = true): this {
        this.config.dismissible = dismissible;
        return this;
    }
    
    public showProgress(show: boolean = true): this {
        this.config.showProgress = show;
        return this;
    }
    
    public showIcon(show: boolean = true): this {
        this.config.showIcon = show;
        return this;
    }
    
    public showCloseButton(show: boolean = true): this {
        this.config.showCloseButton = show;
        return this;
    }
    
    public classes(classes: string[]): this {
        this.config.classes = classes;
        return this;
    }
    
    public info(): this {
        this.config.variant = ToastVariant.Info;
        return this;
    }
    
    public success(): this {
        this.config.variant = ToastVariant.Success;
        return this;
    }
    
    public warning(): this {
        this.config.variant = ToastVariant.Warning;
        return this;
    }
    
    public error(): this {
        this.config.variant = ToastVariant.Error;
        return this;
    }
    
    public topLeft(): this {
        this.config.position = ToastPosition.TopLeft;
        return this;
    }
    
    public topCenter(): this {
        this.config.position = ToastPosition.TopCenter;
        return this;
    }
    
    public topRight(): this {
        this.config.position = ToastPosition.TopRight;
        return this;
    }
    
    public bottomLeft(): this {
        this.config.position = ToastPosition.BottomLeft;
        return this;
    }
    
    public bottomCenter(): this {
        this.config.position = ToastPosition.BottomCenter;
        return this;
    }
    
    public bottomRight(): this {
        this.config.position = ToastPosition.BottomRight;
        return this;
    }
    
    public build(): ElementBuilder {
        // Validate required fields
        if (!this.config.message) {
            throw new Error('Toast requires a message');
        }
        
        return toast(this.config as ToastConfig);
    }
}

// Convenience functions
export function createToast(message: string, variant: ToastVariant = ToastVariant.Info): ToastBuilder {
    return ToastBuilder.create().message(message).variant(variant);
}

export function createInfoToast(message: string): ToastBuilder {
    return ToastBuilder.create().message(message).info();
}

export function createSuccessToast(message: string): ToastBuilder {
    return ToastBuilder.create().message(message).success();
}

export function createWarningToast(message: string): ToastBuilder {
    return ToastBuilder.create().message(message).warning();
}

export function createErrorToast(message: string): ToastBuilder {
    return ToastBuilder.create().message(message).error();
}