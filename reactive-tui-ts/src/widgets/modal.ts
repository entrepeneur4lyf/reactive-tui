/**
 * Modal/Dialog Component - Overlay dialogs with backdrop and focus trapping
 * TypeScript equivalent of the Rust Modal widget with full feature parity
 */

export type ModalPosition = 'center' | 'top-center' | 'bottom-center' | 'left-center' | 'right-center' | 
                           'top-left' | 'top-right' | 'bottom-left' | 'bottom-right' | 'custom'

export type ModalSize = 'small' | 'medium' | 'large' | 'extra-large' | 'full-screen' | 'custom'

export type ModalType = 'basic' | 'alert' | 'confirm' | 'prompt' | 'custom'

export interface ModalButton {
    id: string
    label: string
    variant: string
    closesModal: boolean
    isDefault: boolean
    action?: () => void
}

export interface ModalBackdrop {
    visible: boolean
    clickToClose: boolean
    color?: string
    opacity: number
    character: string
}

export interface ModalStyle {
    background?: string
    textColor?: string
    borderColor?: string
    borderChar: string
    titleColor?: string
    buttonColors: Record<string, string>
    padding: number
    shadow: boolean
    rounded: boolean
}

export interface ModalConfig {
    id: string
    title?: string
    content?: string
    modalType?: ModalType
    position?: ModalPosition
    size?: ModalSize
    backdrop?: ModalBackdrop
    style?: ModalStyle
    cssClasses?: string[]
    closeable?: boolean
    animate?: boolean
    customPosition?: { x: number, y: number }
    customSize?: { width: number, height: number }
    // Type-specific configurations
    alertMessage?: string
    confirmMessage?: string
    confirmYesLabel?: string
    confirmNoLabel?: string
    promptMessage?: string
    promptPlaceholder?: string
    promptDefaultValue?: string
    customButtons?: ModalButton[]
}

export function modal(config: ModalConfig): any {
    const {
        id,
        title,
        content = '',
        modalType = 'basic',
        position = 'center',
        size = 'medium',
        backdrop = {
            visible: true,
            clickToClose: true,
            color: '#000000',
            opacity: 0.5,
            character: '█'
        },
        style = {
            background: '#ffffff',
            textColor: '#000000',
            borderColor: '#808080',
            borderChar: '┌',
            titleColor: '#3b82f6',
            buttonColors: {},
            padding: 2,
            shadow: true,
            rounded: true
        },
        cssClasses = [],
        closeable = true,
        animate = true,
        customPosition,
        customSize,
        alertMessage,
        confirmMessage,
        confirmYesLabel = 'Yes',
        confirmNoLabel = 'No',
        promptMessage,
        promptPlaceholder: _promptPlaceholder = '',
        promptDefaultValue: _promptDefaultValue = '',
        customButtons = []
    } = config

    // Modal state
    let isOpen = false
    let result: string | null = null
    let focusableElements: string[] = []
    let focusedElement = 0
    let promptInputValue = promptDefaultValue || ''

    // Update focusable elements based on modal type
    const updateFocusableElements = () => {
        focusableElements = []
        
        switch (modalType) {
            case 'alert':
                focusableElements.push('ok')
                break
            case 'confirm':
                focusableElements.push('yes', 'no')
                break
            case 'prompt':
                focusableElements.push('input', 'ok', 'cancel')
                break
            case 'custom':
                customButtons.forEach(button => {
                    focusableElements.push(button.id)
                })
                break
            case 'basic':
                // Custom focusable elements would be set externally
                break
        }

        // Set default focus
        if (modalType === 'custom') {
            const defaultButton = customButtons.findIndex(b => b.isDefault)
            if (defaultButton !== -1) {
                focusedElement = defaultButton
            }
        } else {
            focusedElement = 0
        }
    }

    // Handle keyboard input
    const handleKey = (key: string): boolean => {
        if (!isOpen) return false

        switch (key) {
            case 'Escape':
                if (closeable) {
                    close()
                    return true
                }
                break
            case 'Tab':
                focusNext()
                return true
            case 'Shift+Tab':
                focusPrevious()
                return true
            case 'Enter':
                activateFocusedElement()
                return true
            case 'Backspace':
                if (modalType === 'prompt' && focusableElements[focusedElement] === 'input') {
                    promptInputValue = promptInputValue.slice(0, -1)
                    return true
                }
                break
            default:
                // Handle character input for prompt
                if (modalType === 'prompt' && focusableElements[focusedElement] === 'input' && key.length === 1) {
                    promptInputValue += key
                    return true
                }
                break
        }
        return false
    }

    // Move focus to next element
    const focusNext = () => {
        if (focusableElements.length > 0) {
            focusedElement = (focusedElement + 1) % focusableElements.length
        }
    }

    // Move focus to previous element
    const focusPrevious = () => {
        if (focusableElements.length > 0) {
            focusedElement = focusedElement === 0 
                ? focusableElements.length - 1 
                : focusedElement - 1
        }
    }

    // Activate currently focused element
    const activateFocusedElement = () => {
        const elementId = focusableElements[focusedElement]
        if (!elementId) return

        switch (modalType) {
            case 'alert':
                if (elementId === 'ok') {
                    closeWithResult('ok')
                }
                break
            case 'confirm':
                if (elementId === 'yes') {
                    closeWithResult('yes')
                } else if (elementId === 'no') {
                    closeWithResult('no')
                }
                break
            case 'prompt':
                if (elementId === 'ok') {
                    // Get input value from the prompt input field
                    const inputValue = promptInputValue || promptDefaultValue || '';
                    closeWithResult('ok', inputValue)
                } else if (elementId === 'cancel') {
                    closeWithResult('cancel')
                }
                break
            case 'custom':
                const button = customButtons.find(b => b.id === elementId)
                if (button) {
                    // Execute button action if provided
                    if (button.action) {
                        button.action()
                    }
                    // Close modal if configured to do so
                    if (button.closesModal) {
                        closeWithResult(button.id)
                    }
                }
                break
        }
    }

    // Calculate modal dimensions and position
    const calculateLayout = (screenWidth: number, screenHeight: number) => {
        let width: number, height: number

        switch (size) {
            case 'small':
                width = 40; height = 15
                break
            case 'medium':
                width = 60; height = 20
                break
            case 'large':
                width = 80; height = 30
                break
            case 'extra-large':
                width = 100; height = 40
                break
            case 'full-screen':
                width = screenWidth; height = screenHeight
                break
            case 'custom':
                width = customSize?.width || 60
                height = customSize?.height || 20
                break
            default:
                width = 60; height = 20
        }

        let x: number, y: number

        switch (position) {
            case 'center':
                x = Math.floor((screenWidth - width) / 2)
                y = Math.floor((screenHeight - height) / 2)
                break
            case 'top-center':
                x = Math.floor((screenWidth - width) / 2)
                y = Math.floor(screenHeight / 4)
                break
            case 'bottom-center':
                x = Math.floor((screenWidth - width) / 2)
                y = Math.floor(screenHeight * 3 / 4 - height)
                break
            case 'left-center':
                x = Math.floor(screenWidth / 4)
                y = Math.floor((screenHeight - height) / 2)
                break
            case 'right-center':
                x = Math.floor(screenWidth * 3 / 4 - width)
                y = Math.floor((screenHeight - height) / 2)
                break
            case 'top-left':
                x = Math.floor(screenWidth / 10)
                y = Math.floor(screenHeight / 10)
                break
            case 'top-right':
                x = Math.floor(screenWidth * 9 / 10 - width)
                y = Math.floor(screenHeight / 10)
                break
            case 'bottom-left':
                x = Math.floor(screenWidth / 10)
                y = Math.floor(screenHeight * 9 / 10 - height)
                break
            case 'bottom-right':
                x = Math.floor(screenWidth * 9 / 10 - width)
                y = Math.floor(screenHeight * 9 / 10 - height)
                break
            case 'custom':
                x = customPosition?.x || Math.floor((screenWidth - width) / 2)
                y = customPosition?.y || Math.floor((screenHeight - height) / 2)
                break
            default:
                x = Math.floor((screenWidth - width) / 2)
                y = Math.floor((screenHeight - height) / 2)
        }

        return {
            x: Math.max(0, Math.min(x, screenWidth - width)),
            y: Math.max(0, Math.min(y, screenHeight - height)),
            width: Math.min(width, screenWidth),
            height: Math.min(height, screenHeight)
        }
    }

    // Render backdrop
    const renderBackdrop = (screenWidth: number, screenHeight: number) => {
        if (!backdrop.visible) return ''
        
        const lines = []
        for (let i = 0; i < screenHeight; i++) {
            lines.push(backdrop.character.repeat(screenWidth))
        }
        return lines.join('\n')
    }

    // Render modal content
    const renderModalContent = (layout: { x: number, y: number, width: number, height: number }) => {
        const lines = []
        
        for (let row = 0; row < layout.height; row++) {
            let line = ''
            
            if (row === 0) {
                // Top border
                line = '┌' + '─'.repeat(layout.width - 2) + '┐'
            } else if (row === layout.height - 1) {
                // Bottom border
                line = '└' + '─'.repeat(layout.width - 2) + '┘'
            } else {
                // Content area
                line = '│' + renderContentLine(row - 1, layout.width - 2) + '│'
            }
            
            lines.push(line)
        }
        
        return lines.join('\n')
    }

    // Render a single content line
    const renderContentLine = (lineIndex: number, width: number) => {
        const padding = style.padding
        const contentWidth = width - (padding * 2)

        if (lineIndex < padding) {
            // Top padding
            return ' '.repeat(width)
        }

        const contentLine = lineIndex - padding

        if (contentLine === 0 && title) {
            // Title line
            const titleText = title.substring(0, contentWidth)
            const paddingRight = contentWidth - titleText.length
            return ' '.repeat(padding) + titleText + ' '.repeat(paddingRight + padding)
        } else if (contentLine === 1 || (contentLine === 0 && !title)) {
            // Content line
            const message = getContentMessage()
            const contentText = message.substring(0, contentWidth)
            const paddingRight = contentWidth - contentText.length
            return ' '.repeat(padding) + contentText + ' '.repeat(paddingRight + padding)
        } else if (modalType === 'prompt' && (contentLine === 2 || (contentLine === 1 && !title))) {
            // Prompt input line
            return renderPromptInputLine(width)
        } else if (contentLine === 4 || (contentLine === 3 && !title) || (contentLine === 3 && modalType === 'prompt')) {
            // Button line
            return renderButtonsLine(width)
        } else {
            // Empty line or additional content
            return ' '.repeat(width)
        }
    }

    // Get content message based on modal type
    const getContentMessage = () => {
        switch (modalType) {
            case 'alert':
                return alertMessage || content
            case 'confirm':
                return confirmMessage || content
            case 'prompt':
                return promptMessage || content
            default:
                return content
        }
    }

    // Render prompt input line
    const renderPromptInputLine = (width: number) => {
        const padding = style.padding
        const inputWidth = width - (padding * 2) - 2 // Account for brackets
        const displayValue = promptInputValue || promptPlaceholder || ''
        const isFocused = focusableElements[focusedElement] === 'input'
        
        // Create input display with cursor if focused
        let inputDisplay = displayValue.substring(0, inputWidth - 1)
        if (isFocused) {
            inputDisplay += '█' // Cursor
        }
        
        // Pad to full width
        inputDisplay = inputDisplay.padEnd(inputWidth, ' ')
        
        // Build the full line with brackets
        const inputLine = isFocused ? `[${inputDisplay}]` : ` ${inputDisplay} `
        const centerPos = Math.floor((width - inputLine.length) / 2)
        
        return ' '.repeat(centerPos) + inputLine + ' '.repeat(width - centerPos - inputLine.length)
    }

    // Render buttons line
    const renderButtonsLine = (width: number) => {
        let buttonsText = ''

        switch (modalType) {
            case 'alert':
                buttonsText = focusedElement === 0 ? '[OK]' : ' OK '
                break
            case 'confirm':
                const yesText = focusedElement === 0 ? `[${confirmYesLabel}]` : ` ${confirmYesLabel} `
                const noText = focusedElement === 1 ? `[${confirmNoLabel}]` : ` ${confirmNoLabel} `
                buttonsText = `${yesText}  ${noText}`
                break
            case 'prompt':
                const okText = focusedElement === 1 ? '[OK]' : ' OK '
                const cancelText = focusedElement === 2 ? '[Cancel]' : ' Cancel '
                buttonsText = `${okText}  ${cancelText}`
                break
            case 'custom':
                const buttonTexts = customButtons.map((button, i) => {
                    return i === focusedElement ? `[${button.label}]` : ` ${button.label} `
                })
                buttonsText = buttonTexts.join('  ')
                break
            case 'basic':
                return ' '.repeat(width)
        }

        const buttonLen = buttonsText.length
        const centerPos = Math.floor((width - buttonLen) / 2)
        return ' '.repeat(centerPos) + buttonsText + ' '.repeat(width - centerPos - buttonLen)
    }

    // Open modal
    const open = () => {
        isOpen = true
        result = null
        updateFocusableElements()
    }

    // Close modal
    const close = () => {
        isOpen = false
    }

    // Close with result
    const closeWithResult = (modalResult: string, additionalData?: string) => {
        result = modalResult
        if (additionalData !== undefined && modalType === 'prompt') {
            result = `${modalResult}:${additionalData}`
        }
        close()
    }
    
    // Update prompt input value
    const updatePromptValue = (value: string) => {
        if (modalType === 'prompt') {
            promptInputValue = value
        }
    }

    // Build CSS classes
    const classes = [
        'modal',
        `modal-${position}`,
        `modal-${size}`,
        `modal-${modalType}`,
        animate ? 'modal-animated' : '',
        !isOpen ? 'modal-hidden' : '',
        ...cssClasses
    ].filter(Boolean)

    const element = {
        tag: 'div',
        id,
        classes,
        attributes: {
            'data-position': position,
            'data-size': size,
            'data-type': modalType,
            'data-open': isOpen.toString(),
            'data-closeable': closeable.toString(),
            'data-animate': animate.toString(),
            'data-style': JSON.stringify(style),
            'data-backdrop': JSON.stringify(backdrop),
            'data-focused-element': focusedElement.toString(),
            'data-focusable-elements': JSON.stringify(focusableElements)
        },
        content: '',
        children: []
    }

    return {
        ...element,
        // Open the modal
        open,
        
        // Close the modal
        close,
        
        // Close with result
        closeWithResult,
        
        // Check if modal is open
        isOpen: () => isOpen,
        
        // Get modal result
        getResult: () => {
            if (modalType === 'prompt' && result && result.includes(':')) {
                // Return structured result for prompt modals
                const [action, value] = result.split(':', 2)
                return { action, value }
            }
            return result
        },
        
        // Handle keyboard input
        handleKey,
        
        // Focus navigation
        focusNext,
        focusPrevious,
        activateFocusedElement,
        
        // Get currently focused element
        getFocusedElement: () => focusableElements[focusedElement],
        
        // Set focused element by ID
        setFocusedElement: (elementId: string) => {
            const index = focusableElements.indexOf(elementId)
            if (index !== -1) {
                focusedElement = index
                element.attributes['data-focused-element'] = focusedElement.toString()
            }
        },
        
        // Calculate layout
        calculateLayout,
        
        // Render the modal
        render: (screenWidth?: number, screenHeight?: number) => {
            if (!isOpen) return { backdrop: '', modal: '', position: null }
            
            const width = screenWidth || 80
            const height = screenHeight || 24
            const layout = calculateLayout(width, height)
            
            return {
                backdrop: renderBackdrop(width, height),
                modal: renderModalContent(layout),
                position: layout,
                isOpen,
                focusedElement: focusableElements[focusedElement],
                focusableElements: [...focusableElements]
            }
        },
        
        // Update modal configuration
        updateConfig: (newConfig: Partial<ModalConfig>) => {
            Object.assign(element.attributes, {
                'data-position': newConfig.position || position,
                'data-size': newConfig.size || size,
                'data-type': newConfig.modalType || modalType,
                'data-closeable': (newConfig.closeable !== undefined ? newConfig.closeable : closeable).toString(),
                'data-animate': (newConfig.animate !== undefined ? newConfig.animate : animate).toString()
            })
        },
        
        // Build the element
        build: () => element,
        
        // Update prompt input value (for prompt modals)
        updatePromptValue
    }
}

// Builder pattern for fluent API
export function modalBuilder(id: string, modalType: ModalType = 'basic') {
    const config: ModalConfig = { id, modalType }

    return {
        title: (title: string) => {
            config.title = title
            return modalBuilder(id, modalType)
        },

        content: (content: string) => {
            config.content = content
            return modalBuilder(id, modalType)
        },

        position: (position: ModalPosition) => {
            config.position = position
            return modalBuilder(id, modalType)
        },

        size: (size: ModalSize) => {
            config.size = size
            return modalBuilder(id, modalType)
        },

        backdrop: (backdrop: Partial<ModalBackdrop>) => {
            config.backdrop = { ...config.backdrop, ...backdrop } as ModalBackdrop
            return modalBuilder(id, modalType)
        },

        style: (style: Partial<ModalStyle>) => {
            config.style = { ...config.style, ...style } as ModalStyle
            return modalBuilder(id, modalType)
        },

        cssClass: (className: string) => {
            if (!config.cssClasses) config.cssClasses = []
            config.cssClasses.push(className)
            return modalBuilder(id, modalType)
        },

        notCloseable: () => {
            config.closeable = false
            return modalBuilder(id, modalType)
        },

        noAnimation: () => {
            config.animate = false
            return modalBuilder(id, modalType)
        },

        customPosition: (x: number, y: number) => {
            config.position = 'custom'
            config.customPosition = { x, y }
            return modalBuilder(id, modalType)
        },

        customSize: (width: number, height: number) => {
            config.size = 'custom'
            config.customSize = { width, height }
            return modalBuilder(id, modalType)
        },

        build: () => modal(config)
    }
}

// Convenience functions for common modal types

export function alertModal(id: string, title: string, message: string) {
    return modalBuilder(id, 'alert')
        .title(title)
        .content(message)
        .size('small')
        .build()
}

export function confirmModal(id: string, title: string, message: string, yesLabel = 'Yes', noLabel = 'No') {
    const config: ModalConfig = {
        id,
        title,
        modalType: 'confirm',
        size: 'medium',
        confirmMessage: message,
        confirmYesLabel: yesLabel,
        confirmNoLabel: noLabel
    }
    return modal(config)
}

export function promptModal(id: string, title: string, message: string, placeholder = '', defaultValue = '') {
    const config: ModalConfig = {
        id,
        title,
        modalType: 'prompt',
        size: 'medium',
        promptMessage: message,
        promptPlaceholder: placeholder,
        promptDefaultValue: defaultValue
    }
    return modal(config)
}

export function customModal(id: string, title: string, content: string, buttons: ModalButton[]) {
    const config: ModalConfig = {
        id,
        title,
        content,
        modalType: 'custom',
        size: 'medium',
        customButtons: buttons
    }
    return modal(config)
}

export function fullscreenModal(id: string, content: string) {
    return modalBuilder(id, 'basic')
        .content(content)
        .size('full-screen')
        .backdrop({
            visible: false,
            clickToClose: false,
            color: '#000000',
            opacity: 0,
            character: ' '
        })
        .build()
}

// Helper functions for creating modal buttons
export function createModalButton(
    id: string, 
    label: string, 
    options?: Partial<ModalButton>
): ModalButton {
    return {
        id,
        label,
        variant: 'secondary',
        closesModal: false,
        isDefault: false,
        ...options
    }
}

export function createPrimaryButton(id: string, label: string, closesModal = true): ModalButton {
    return createModalButton(id, label, { 
        variant: 'primary', 
        closesModal, 
        isDefault: true 
    })
}

export function createSecondaryButton(id: string, label: string, closesModal = true): ModalButton {
    return createModalButton(id, label, { 
        variant: 'secondary', 
        closesModal 
    })
}

export function createDangerButton(id: string, label: string, closesModal = true): ModalButton {
    return createModalButton(id, label, { 
        variant: 'danger', 
        closesModal 
    })
}

// Complete application modal examples
export function createModalExamples() {
    const simpleAlert = alertModal('simple-alert', 'Success', 'Operation completed successfully!')
    
    const deleteConfirm = confirmModal(
        'delete-confirm', 
        'Delete File', 
        'Are you sure you want to delete this file? This action cannot be undone.',
        'Delete',
        'Cancel'
    )
    
    const namePrompt = promptModal(
        'name-prompt',
        'Enter Name',
        'Please enter your full name:',
        'John Doe',
        ''
    )
    
    const customSave = customModal(
        'save-dialog',
        'Save Changes',
        'You have unsaved changes. What would you like to do?',
        [
            createPrimaryButton('save', 'Save'),
            createSecondaryButton('discard', 'Discard Changes'),
            createSecondaryButton('cancel', 'Cancel', false)
        ]
    )
    
    const settingsModal = modalBuilder('settings', 'basic')
        .title('Application Settings')
        .content('Configure your application preferences here.')
        .size('large')
        .position('center')
        .cssClass('settings-modal')
        .build()

    return {
        alert: simpleAlert,
        confirm: deleteConfirm,
        prompt: namePrompt,
        custom: customSave,
        settings: settingsModal
    }
}