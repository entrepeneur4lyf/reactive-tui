/**
 * Button Component - CSS-styled button with comprehensive functionality
 * Supports variants, sizes, states, icons, and advanced interactions
 */

import type { ResponsiveWidget } from '../types';

export type ButtonVariant = 'filled' | 'outlined' | 'ghost' | 'link' | 'gradient'
export type ButtonSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'
export type ButtonColor = 'primary' | 'secondary' | 'success' | 'warning' | 'error' | 'info' | 'neutral'
export type ButtonShape = 'rectangle' | 'rounded' | 'pill' | 'circle' | 'square'

export interface ButtonIcon {
    symbol: string
    position?: 'left' | 'right' | 'top' | 'bottom'
    size?: number
}

export interface ButtonConfig {
    id: string
    variant?: ButtonVariant
    size?: ButtonSize
    color?: ButtonColor
    shape?: ButtonShape
    
    // Content
    text?: string
    icon?: ButtonIcon
    iconOnly?: boolean
    
    // State
    disabled?: boolean
    loading?: boolean
    active?: boolean
    
    // Behavior
    type?: 'button' | 'submit' | 'reset'
    fullWidth?: boolean
    autoFocus?: boolean
    
    // Styling
    width?: number | string
    height?: number | string
    cssClasses?: string[]
    style?: {
        backgroundColor?: string
        borderColor?: string
        textColor?: string
        hoverColor?: string
        activeColor?: string
        disabledColor?: string
    }
    
    // Events
    onClick?: (event?: any) => void
    onMouseEnter?: (event?: any) => void
    onMouseLeave?: (event?: any) => void
    onFocus?: (event?: any) => void
    onBlur?: (event?: any) => void
    onKeyPress?: (key: string, event?: any) => void
}

export function button(config: ButtonConfig): any {
    const {
        id,
        variant = 'filled',
        size = 'md',
        color = 'primary',
        shape = 'rounded',
        text = '',
        icon,
        iconOnly = false,
        disabled = false,
        loading = false,
        active = false,
        type = 'button',
        fullWidth = false,
        autoFocus = false,
        width,
        height,
        cssClasses = [],
        style = {},
        onClick,
        onFocus,
        onBlur
    } = config

    // Build CSS classes
    const classes = [
        'button',
        `button-${variant}`,
        `button-${size}`,
        `button-${color}`,
        `button-${shape}`,
        disabled ? 'button-disabled' : '',
        loading ? 'button-loading' : '',
        active ? 'button-active' : '',
        fullWidth ? 'button-full-width' : '',
        iconOnly ? 'button-icon-only' : '',
        ...cssClasses
    ].filter(Boolean)

    // Build button content
    let content = ''
    const children: any[] = []

    if (loading) {
        children.push({
            tag: 'span',
            classes: ['button-spinner'],
            content: '⟳',
            children: []
        })
    }

    if (icon && !loading) {
        const iconElement = {
            tag: 'span',
            classes: ['button-icon', `button-icon-${icon.position || 'left'}`],
            attributes: {
                'data-size': icon.size?.toString() || ''
            },
            content: icon.symbol,
            children: []
        }

        if (icon.position === 'right' || icon.position === 'bottom') {
            if (!iconOnly && text) {
                children.push({
                    tag: 'span',
                    classes: ['button-text'],
                    content: text,
                    children: []
                })
            }
            children.push(iconElement)
        } else {
            children.push(iconElement)
            if (!iconOnly && text) {
                children.push({
                    tag: 'span',
                    classes: ['button-text'],
                    content: text,
                    children: []
                })
            }
        }
    } else if (!loading && text) {
        children.push({
            tag: 'span',
            classes: ['button-text'],
            content: text,
            children: []
        })
    }

    // Build button element
    const element = {
        tag: 'button',
        id,
        classes,
        attributes: {
            type,
            disabled: disabled.toString(),
            'data-variant': variant,
            'data-size': size,
            'data-color': color,
            'data-shape': shape,
            'data-loading': loading.toString(),
            'data-active': active.toString(),
            'data-full-width': fullWidth.toString(),
            'data-icon-only': iconOnly.toString(),
            'data-auto-focus': autoFocus.toString(),
            'data-width': width?.toString() || '',
            'data-height': height?.toString() || '',
            'data-style': JSON.stringify(style)
        },
        content,
        children
    }

    const api = {
        ...element,
        setText: (newText: string) => {
            const textChild = element.children.find(child => child.classes.includes('button-text'))
            if (textChild) {
                textChild.content = newText
            } else if (!iconOnly) {
                element.children.push({
                    tag: 'span',
                    classes: ['button-text'],
                    content: newText,
                    children: []
                })
            }
        },
        setIcon: (newIcon: ButtonIcon) => {
            // Remove existing icon
            element.children = element.children.filter(child => !child.classes.includes('button-icon'))
            
            // Add new icon
            const iconElement = {
                tag: 'span',
                classes: ['button-icon', `button-icon-${newIcon.position || 'left'}`],
                attributes: {
                    'data-size': newIcon.size?.toString() || ''
                },
                content: newIcon.symbol,
                children: []
            }

            if (newIcon.position === 'right' || newIcon.position === 'bottom') {
                element.children.push(iconElement)
            } else {
                element.children.unshift(iconElement)
            }
        },
        setLoading: (isLoading: boolean) => {
            element.attributes['data-loading'] = isLoading.toString()
            
            if (isLoading) {
                element.classes = element.classes.filter(c => c !== 'button-loading')
                element.classes.push('button-loading')
                
                // Add spinner
                element.children.unshift({
                    tag: 'span',
                    classes: ['button-spinner'],
                    content: '⟳',
                    children: []
                })
            } else {
                element.classes = element.classes.filter(c => c !== 'button-loading')
                element.children = element.children.filter(child => !child.classes.includes('button-spinner'))
            }
        },
        setDisabled: (isDisabled: boolean) => {
            element.attributes.disabled = isDisabled.toString()
            
            if (isDisabled) {
                element.classes = element.classes.filter(c => c !== 'button-disabled')
                element.classes.push('button-disabled')
            } else {
                element.classes = element.classes.filter(c => c !== 'button-disabled')
            }
        },
        setActive: (isActive: boolean) => {
            element.attributes['data-active'] = isActive.toString()
            
            if (isActive) {
                element.classes = element.classes.filter(c => c !== 'button-active')
                element.classes.push('button-active')
            } else {
                element.classes = element.classes.filter(c => c !== 'button-active')
            }
        },
        click: () => {
            if (!disabled && !loading) {
                onClick?.()
            }
        },
        focus: () => {
            onFocus?.()
        },
        blur: () => {
            onBlur?.()
        },
        child: (child: any) => {
            // Add child element
            if (typeof child === 'string') {
                element.children.push({
                    tag: 'text',
                    content: child,
                    classes: [],
                    children: []
                })
            } else if (child && typeof child === 'object') {
                element.children.push(child.build ? child.build() : child)
            }
            return api
        },
        build: () => element
    }
    
    return api
}

// Convenience functions for common button types
export function primaryButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'filled',
        color: 'primary',
        ...config
    })
}

export function secondaryButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'outlined',
        color: 'secondary',
        ...config
    })
}

export function dangerButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'filled',
        color: 'error',
        ...config
    })
}

export function successButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'filled',
        color: 'success',
        ...config
    })
}

export function ghostButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'ghost',
        ...config
    })
}

export function linkButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        variant: 'link',
        ...config
    })
}

export function iconButton(id: string, icon: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        icon: { symbol: icon },
        iconOnly: true,
        shape: 'circle',
        ...config
    })
}

export function loadingButton(id: string, text: string, config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        loading: true,
        disabled: true,
        ...config
    })
}

// Button group for organizing multiple buttons
export function buttonGroup(buttons: any[], config?: {
    orientation?: 'horizontal' | 'vertical'
    spacing?: 'none' | 'sm' | 'md' | 'lg'
    wrap?: boolean
    justify?: 'start' | 'center' | 'end' | 'space-between' | 'space-around'
}) {
    const {
        orientation = 'horizontal',
        spacing = 'sm',
        wrap = false,
        justify = 'start'
    } = config || {}

    return {
        tag: 'div',
        classes: [
            'button-group',
            `button-group-${orientation}`,
            `button-group-spacing-${spacing}`,
            wrap ? 'button-group-wrap' : '',
            `button-group-justify-${justify}`
        ].filter(Boolean),
        attributes: {
            'data-orientation': orientation,
            'data-spacing': spacing,
            'data-wrap': wrap.toString(),
            'data-justify': justify
        },
        children: buttons.map(btn => btn.build()),
        content: ''
    }
}

// Action button variants
export function submitButton(id: string, text = 'Submit', config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        type: 'submit',
        variant: 'filled',
        color: 'primary',
        ...config
    })
}

export function cancelButton(id: string, text = 'Cancel', config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        type: 'button',
        variant: 'outlined',
        color: 'neutral',
        ...config
    })
}

export function resetButton(id: string, text = 'Reset', config?: Partial<ButtonConfig>) {
    return button({
        id,
        text,
        type: 'reset',
        variant: 'ghost',
        color: 'neutral',
        ...config
    })
}

// Toggle button for on/off states
export function toggleButton(id: string, text: string, initialState = false, config?: Partial<ButtonConfig>) {
    const btn = button({
        id,
        text,
        active: initialState,
        variant: 'outlined',
        ...config
    })

    return {
        ...btn,
        toggle: () => {
            const currentState = btn.attributes['data-active'] === 'true'
            btn.setActive(!currentState)
            return !currentState
        },
        getState: () => btn.attributes['data-active'] === 'true',
        setState: (state: boolean) => btn.setActive(state)
    }
}

// ResponsiveWidget implementation for Button
export class ButtonWidget implements ResponsiveWidget {
    private config: ButtonConfig
    private buttonInstance: any

    constructor(config: ButtonConfig) {
        this.config = config
        this.buttonInstance = button(config)
    }

    toElement(): import('../generated-types').Element {
        return {
            tag: this.buttonInstance.tag,
            id: this.buttonInstance.id,
            classes: this.buttonInstance.classes,
            attributes: this.buttonInstance.attributes,
            content: this.buttonInstance.content,
            children: this.buttonInstance.children,
            key_bindings: [],
            focusable: !this.config.disabled,
            focused: false,
            modal: false,
            tab_index: this.config.autoFocus ? 0 : undefined
        }
    }

    renderWithLayout(layout: import('../layout').LayoutRect, _theme?: any): string {
        // Use the layout to render the button with computed size
        const { width, height: _height } = layout
        // Note: _height is available for future enhancements like multi-line buttons or vertical centering

        // Create a text representation of the button
        const text = this.config.text || ''
        const disabled = this.config.disabled ? ' (disabled)' : ''
        const loading = this.config.loading ? ' ⟳' : ''
        const icon = this.config.icon ? `${this.config.icon.symbol} ` : ''

        const content = `${icon}${text}${loading}${disabled}`
        const padding = 2 // Button padding
        const availableWidth = Math.max(1, width - padding)

        // Truncate or pad content to fit width
        const displayContent = content.length > availableWidth
            ? content.substring(0, availableWidth - 1) + '…'
            : content.padEnd(availableWidth)

        return `[${displayContent}]`
    }

    minSize(): [number, number] {
        const text = this.config.text || ''
        const icon = this.config.icon ? this.config.icon.symbol.length + 1 : 0
        const minWidth = Math.max(4, text.length + icon + 4) // text + icon + padding + borders
        const minHeight = 1
        return [minWidth, minHeight]
    }

    maxSize(): [number | null, number | null] {
        const text = this.config.text || ''
        const icon = this.config.icon ? this.config.icon.symbol.length + 1 : 0
        const maxWidth = this.config.fullWidth ? null : text.length + icon + 10 // Allow some extra space
        const maxHeight = 3 // Buttons don't need to be very tall
        return [maxWidth, maxHeight]
    }

    canGrowHorizontal(): boolean {
        return this.config.fullWidth || false
    }

    canGrowVertical(): boolean {
        return false // Buttons typically don't grow vertically
    }
}