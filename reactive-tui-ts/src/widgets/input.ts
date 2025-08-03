/**
 * Input Component - CSS-styled text input with comprehensive functionality
 * Supports validation, formatting, accessibility, and advanced styling
 */

export type InputType = 'text' | 'password' | 'email' | 'number' | 'tel' | 'url' | 'search' | 'date' | 'time' | 'datetime-local'
export type InputVariant = 'filled' | 'outlined' | 'underlined' | 'ghost' | 'flushed'
export type InputSize = 'xs' | 'sm' | 'md' | 'lg' | 'xl'
export type InputStatus = 'default' | 'success' | 'warning' | 'error' | 'loading'

export interface InputValidation {
    required?: boolean
    minLength?: number
    maxLength?: number
    pattern?: RegExp
    custom?: (value: string) => string | null
}

export interface InputFormatting {
    mask?: string
    transform?: 'lowercase' | 'uppercase' | 'capitalize'
    filter?: RegExp
}

export interface InputConfig {
    id: string
    type?: InputType
    variant?: InputVariant
    size?: InputSize
    status?: InputStatus
    
    // Content
    value?: string
    placeholder?: string
    label?: string
    helpText?: string
    errorMessage?: string
    
    // Validation
    validation?: InputValidation
    formatting?: InputFormatting
    
    // Behavior
    disabled?: boolean
    readonly?: boolean
    autoFocus?: boolean
    clearable?: boolean
    showCounter?: boolean
    
    // Styling
    width?: number | string
    cssClasses?: string[]
    style?: {
        backgroundColor?: string
        borderColor?: string
        textColor?: string
        placeholderColor?: string
        focusColor?: string
    }
    
    // Events
    onInput?: (value: string, event?: any) => void
    onChange?: (value: string, event?: any) => void
    onFocus?: (event?: any) => void
    onBlur?: (event?: any) => void
    onKeyPress?: (key: string, event?: any) => void
    onValidation?: (isValid: boolean, errors: string[]) => void
}

export function input(config: InputConfig): any {
    const {
        id,
        type = 'text',
        variant = 'outlined',
        size = 'md',
        status = 'default',
        value = '',
        placeholder = '',
        label,
        helpText,
        errorMessage,
        validation,
        formatting,
        disabled = false,
        readonly = false,
        autoFocus = false,
        clearable = false,
        showCounter = false,
        width,
        cssClasses = [],
        style = {},
        onInput,
        onChange,
        onFocus,
        onBlur,
        onValidation
    } = config

    // Build CSS classes
    const classes = [
        'input',
        `input-${variant}`,
        `input-${size}`,
        `input-${status}`,
        disabled ? 'input-disabled' : '',
        readonly ? 'input-readonly' : '',
        clearable ? 'input-clearable' : '',
        ...cssClasses
    ].filter(Boolean)

    // Build input element
    const element = {
        tag: 'input',
        id,
        classes,
        attributes: {
            type,
            value,
            placeholder,
            disabled: disabled.toString(),
            readonly: readonly.toString(),
            'data-variant': variant,
            'data-size': size,
            'data-status': status,
            'data-validation': validation ? JSON.stringify(validation) : '',
            'data-formatting': formatting ? JSON.stringify(formatting) : '',
            'data-width': width?.toString() || '',
            'data-auto-focus': autoFocus.toString(),
            'data-clearable': clearable.toString(),
            'data-show-counter': showCounter.toString(),
            'data-style': JSON.stringify(style)
        },
        content: value,
        children: []
    }

    // Add label and help text as siblings
    const wrapperChildren = []
    
    if (label) {
        wrapperChildren.push({
            tag: 'label',
            classes: ['input-label', `input-label-${size}`],
            attributes: { for: id },
            content: label,
            children: []
        })
    }

    wrapperChildren.push(element)

    if (helpText && !errorMessage) {
        wrapperChildren.push({
            tag: 'div',
            classes: ['input-help-text'],
            content: helpText,
            children: []
        })
    }

    if (errorMessage) {
        wrapperChildren.push({
            tag: 'div',
            classes: ['input-error-message'],
            content: errorMessage,
            children: []
        })
    }

    if (showCounter && validation?.maxLength) {
        wrapperChildren.push({
            tag: 'div',
            classes: ['input-counter'],
            content: `${value.length}/${validation.maxLength}`,
            children: []
        })
    }

    const wrapper = {
        tag: 'div',
        id: `${id}-wrapper`,
        classes: ['input-wrapper', `input-wrapper-${variant}`, `input-wrapper-${size}`],
        attributes: {},
        content: '',
        children: wrapperChildren
    }

    return {
        ...wrapper,
        setValue: (newValue: string) => {
            element.attributes.value = newValue
            element.content = newValue
            
            // Trigger validation
            if (validation && onValidation) {
                const errors = validateInput(newValue, validation)
                onValidation(errors.length === 0, errors)
            }
            
            // Trigger events
            onInput?.(newValue)
            onChange?.(newValue)
        },
        getValue: () => element.attributes.value,
        validate: () => {
            if (!validation) return { isValid: true, errors: [] }
            const errors = validateInput(element.attributes.value, validation)
            return { isValid: errors.length === 0, errors }
        },
        focus: () => {
            onFocus?.()
        },
        blur: () => {
            onBlur?.()
        },
        clear: () => {
            if (clearable) {
                element.attributes.value = ''
                element.content = ''
                onInput?.('')
                onChange?.('')
            }
        },
        build: () => wrapper
    }
}

// Validation helper
function validateInput(value: string, validation: InputValidation): string[] {
    const errors: string[] = []

    if (validation.required && !value.trim()) {
        errors.push('This field is required')
    }

    if (validation.minLength && value.length < validation.minLength) {
        errors.push(`Minimum length is ${validation.minLength} characters`)
    }

    if (validation.maxLength && value.length > validation.maxLength) {
        errors.push(`Maximum length is ${validation.maxLength} characters`)
    }

    if (validation.pattern && !validation.pattern.test(value)) {
        errors.push('Invalid format')
    }

    if (validation.custom) {
        const customError = validation.custom(value)
        if (customError) {
            errors.push(customError)
        }
    }

    return errors
}

// Convenience functions for common input types
export function textInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'text',
        variant: 'outlined',
        ...config
    })
}

export function passwordInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'password',
        variant: 'outlined',
        clearable: true,
        ...config
    })
}

export function emailInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'email',
        variant: 'outlined',
        validation: {
            pattern: /^[^\s@]+@[^\s@]+\.[^\s@]+$/,
            ...config?.validation
        },
        ...config
    })
}

export function numberInput(id: string, min?: number, max?: number, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'number',
        variant: 'outlined',
        validation: {
            custom: (value) => {
                const num = parseFloat(value)
                if (isNaN(num)) return 'Must be a valid number'
                if (min !== undefined && num < min) return `Must be at least ${min}`
                if (max !== undefined && num > max) return `Must be at most ${max}`
                return null
            },
            ...config?.validation
        },
        ...config
    })
}

export function searchInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'search',
        variant: 'filled',
        clearable: true,
        placeholder: 'Search...',
        ...config
    })
}

export function phoneInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'tel',
        variant: 'outlined',
        formatting: {
            mask: '(000) 000-0000',
            filter: /[0-9]/
        },
        validation: {
            pattern: /^\(\d{3}\) \d{3}-\d{4}$/,
            ...config?.validation
        },
        ...config
    })
}

export function urlInput(id: string, config?: Partial<InputConfig>) {
    return input({
        id,
        type: 'url',
        variant: 'outlined',
        validation: {
            pattern: /^https?:\/\/.+\..+/,
            ...config?.validation
        },
        ...config
    })
}

// Form integration helpers
export function formInput(id: string, label: string, required = false, config?: Partial<InputConfig>) {
    return input({
        id,
        label,
        validation: {
            required,
            ...config?.validation
        },
        ...config
    })
}

export function inputGroup(inputs: any[]) {
    return {
        tag: 'div',
        classes: ['input-group'],
        children: inputs.map(input => input.build()),
        validate: () => {
            const results = inputs.map(input => input.validate())
            const allValid = results.every(result => result.isValid)
            const allErrors = results.flatMap(result => result.errors)
            return { isValid: allValid, errors: allErrors }
        }
    }
}