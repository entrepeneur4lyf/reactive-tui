/**
 * Bar Component - Comprehensive header/footer bars with flexible content positioning
 * TypeScript equivalent of the Rust Bar widget with full feature parity
 */

export type BarType = 'header' | 'footer' | 'navigation' | 'status' | 'toolbar' | 'tabbar' | 'custom'
export type BarPosition = 'left' | 'center' | 'right' | 'justify'
export type BarBorderStyle = 'none' | 'single' | 'double' | 'thick' | 'custom'
export type BarSize = 'compact' | 'normal' | 'large' | 'custom'

export interface BarItem {
    content: string
    position: BarPosition
    cssClasses?: string[]
    icon?: string
    id?: string
    clickable?: boolean
    weight?: number
    action?: string
    hotkey?: string
}

export interface BarStyle {
    background?: string
    foreground?: string
    borderColor?: string
    borderStyle?: BarBorderStyle
    padding?: number
    separator?: string
    fillWidth?: boolean
}

export interface BarConfig {
    id: string
    barType?: BarType
    size?: BarSize
    items?: BarItem[]
    style?: BarStyle
    cssClasses?: string[]
    visible?: boolean
    sticky?: boolean
    width?: number
    height?: number
}

export function bar(config: BarConfig): any {
    const {
        id,
        barType = 'header',
        size = 'normal',
        items = [],
        style = {},
        cssClasses = [],
        visible = true,
        sticky = false,
        width,
        height
    } = config

    // Build CSS classes
    const classes = [
        'bar',
        `bar-${barType}`,
        `bar-${size}`,
        sticky ? 'bar-sticky' : '',
        !visible ? 'bar-hidden' : '',
        ...cssClasses
    ].filter(Boolean)

    // Get bar height based on size
    const getBarHeight = () => {
        if (height) return height
        switch (size) {
            case 'compact': return 3  // 1 content + 2 borders
            case 'normal': return 4   // 2 content + 2 borders
            case 'large': return 5    // 3 content + 2 borders
            default: return 4
        }
    }

    // Position content within the bar
    const positionContent = () => {
        if (items.length === 0) return { left: [], center: [], right: [], justify: [] }

        // Group items by position
        const leftItems = items.filter(item => item.position === 'left')
        const centerItems = items.filter(item => item.position === 'center')
        const rightItems = items.filter(item => item.position === 'right')
        const justifyItems = items.filter(item => item.position === 'justify')

        return {
            left: leftItems,
            center: centerItems,
            right: rightItems,
            justify: justifyItems
        }
    }

    // Render item content with icon
    const renderItemContent = (item: BarItem) => {
        let content = ''
        if (item.icon) {
            content += item.icon + ' '
        }
        content += item.content
        return content
    }

    // Render positioned items
    const renderPositionedItems = (itemGroup: BarItem[], separator = ' │ ') => {
        return itemGroup.map(item => renderItemContent(item)).join(separator)
    }

    // Render justified items with weighted distribution
    const renderJustifiedItems = (itemGroup: BarItem[], barWidth: number) => {
        if (itemGroup.length === 0) return ''

        const totalWeight = itemGroup.reduce((sum, item) => sum + (item.weight || 1), 0)
        const totalContentLength = itemGroup.reduce((sum, item) => sum + renderItemContent(item).length, 0)
        
        const availableSpace = barWidth - totalContentLength
        if (availableSpace <= 0) {
            return itemGroup.map(item => renderItemContent(item)).join('')
        }

        let result = ''
        itemGroup.forEach((item, index) => {
            result += renderItemContent(item)
            
            // Add proportional spacing (except after last item)
            if (index < itemGroup.length - 1) {
                const itemSpace = Math.floor(((item.weight || 1) / totalWeight) * availableSpace)
                result += ' '.repeat(itemSpace)
            }
        })

        return result
    }

    const element = {
        tag: 'div',
        id,
        classes,
        attributes: {
            'data-bar-type': barType,
            'data-size': size,
            'data-height': getBarHeight().toString(),
            'data-visible': visible.toString(),
            'data-sticky': sticky.toString(),
            'data-width': width?.toString() || '',
            'data-style': JSON.stringify(style),
            'data-items': JSON.stringify(items)
        },
        content: '',
        children: []
    }

    return {
        ...element,
        // Add item to the bar
        addItem: (item: BarItem) => {
            items.push(item)
            element.attributes['data-items'] = JSON.stringify(items)
        },
        
        // Remove item by ID
        removeItem: (itemId: string) => {
            const index = items.findIndex(item => item.id === itemId)
            if (index !== -1) {
                const removed = items.splice(index, 1)[0]
                element.attributes['data-items'] = JSON.stringify(items)
                return removed
            }
            return null
        },
        
        // Get item by ID
        getItem: (itemId: string) => {
            return items.find(item => item.id === itemId) || null
        },
        
        // Update item by ID
        updateItem: (itemId: string, updates: Partial<BarItem>) => {
            const item = items.find(item => item.id === itemId)
            if (item) {
                Object.assign(item, updates)
                element.attributes['data-items'] = JSON.stringify(items)
                return item
            }
            return null
        },
        
        // Set visibility
        setVisible: (isVisible: boolean) => {
            element.attributes['data-visible'] = isVisible.toString()
            if (isVisible) {
                element.classes = element.classes.filter(c => c !== 'bar-hidden')
            } else {
                if (!element.classes.includes('bar-hidden')) {
                    element.classes.push('bar-hidden')
                }
            }
        },
        
        // Set sticky behavior
        setSticky: (isSticky: boolean) => {
            element.attributes['data-sticky'] = isSticky.toString()
            if (isSticky) {
                if (!element.classes.includes('bar-sticky')) {
                    element.classes.push('bar-sticky')
                }
            } else {
                element.classes = element.classes.filter(c => c !== 'bar-sticky')
            }
        },
        
        // Get bar height
        getHeight: () => getBarHeight(),
        
        // Render the bar with positioning
        render: (renderWidth?: number) => {
            const barWidth = renderWidth || width || 80
            const positioned = positionContent()
            
            let renderResult = {
                left: renderPositionedItems(positioned.left),
                center: renderPositionedItems(positioned.center),
                right: renderPositionedItems(positioned.right),
                justify: renderJustifiedItems(positioned.justify, barWidth)
            }
            
            return renderResult
        },
        
        // Build the element
        build: () => element
    }
}

// Builder pattern for fluent API
export function barBuilder(id: string, barType: BarType = 'header') {
    const config: BarConfig = { id, barType, items: [] }
    
    return {
        size: (size: BarSize) => {
            config.size = size
            return barBuilder(id, barType)
        },
        
        item: (item: BarItem) => {
            if (!config.items) config.items = []
            config.items.push(item)
            return barBuilder(id, barType)
        },
        
        left: (content: string, options?: Partial<BarItem>) => {
            if (!config.items) config.items = []
            config.items.push({
                content,
                position: 'left',
                ...options
            })
            return barBuilder(id, barType)
        },
        
        center: (content: string, options?: Partial<BarItem>) => {
            if (!config.items) config.items = []
            config.items.push({
                content,
                position: 'center',
                ...options
            })
            return barBuilder(id, barType)
        },
        
        right: (content: string, options?: Partial<BarItem>) => {
            if (!config.items) config.items = []
            config.items.push({
                content,
                position: 'right',
                ...options
            })
            return barBuilder(id, barType)
        },
        
        style: (style: BarStyle) => {
            config.style = { ...config.style, ...style }
            return barBuilder(id, barType)
        },
        
        cssClass: (className: string) => {
            if (!config.cssClasses) config.cssClasses = []
            config.cssClasses.push(className)
            return barBuilder(id, barType)
        },
        
        background: (color: string) => {
            if (!config.style) config.style = {}
            config.style.background = color
            return barBuilder(id, barType)
        },
        
        foreground: (color: string) => {
            if (!config.style) config.style = {}
            config.style.foreground = color
            return barBuilder(id, barType)
        },
        
        border: (borderStyle: BarBorderStyle) => {
            if (!config.style) config.style = {}
            config.style.borderStyle = borderStyle
            return barBuilder(id, barType)
        },
        
        padding: (padding: number) => {
            if (!config.style) config.style = {}
            config.style.padding = padding
            return barBuilder(id, barType)
        },
        
        sticky: () => {
            config.sticky = true
            return barBuilder(id, barType)
        },
        
        hidden: () => {
            config.visible = false
            return barBuilder(id, barType)
        },
        
        build: () => bar(config)
    }
}

// Convenience functions for common bar types
export function headerBar(id: string) {
    return barBuilder(id, 'header')
        .background('#3B82F6')
        .foreground('#FFFFFF')
}

export function footerBar(id: string) {
    return barBuilder(id, 'footer')
        .background('#6B7280')
        .foreground('#FFFFFF')
}

export function statusBar(id: string) {
    return barBuilder(id, 'status')
        .size('compact')
        .background('#22C55E')
        .foreground('#FFFFFF')
}

export function navigationBar(id: string) {
    return barBuilder(id, 'navigation')
        .background('#1F2937')
        .foreground('#F9FAFB')
}

export function toolbar(id: string) {
    return barBuilder(id, 'toolbar')
        .background('#F3F4F6')
        .foreground('#374151')
        .border('single')
}

// Helper functions for creating bar items
export function barItem(content: string, position: BarPosition, options?: Partial<BarItem>): BarItem {
    return {
        content,
        position,
        weight: 1.0,
        clickable: false,
        ...options
    }
}

export function clickableBarItem(content: string, position: BarPosition, action?: string, options?: Partial<BarItem>): BarItem {
    return {
        content,
        position,
        clickable: true,
        action,
        weight: 1.0,
        ...options
    }
}

export function iconBarItem(content: string, icon: string, position: BarPosition, options?: Partial<BarItem>): BarItem {
    return {
        content,
        icon,
        position,
        weight: 1.0,
        clickable: false,
        ...options
    }
}

export function weightedBarItem(content: string, position: BarPosition, weight: number, options?: Partial<BarItem>): BarItem {
    return {
        content,
        position,
        weight,
        clickable: false,
        ...options
    }
}

// Menu bar helpers
export function menuBarItem(text: string, action: string, hotkey?: string): BarItem {
    return {
        content: text,
        position: 'left',
        clickable: true,
        action,
        hotkey,
        weight: 1.0
    }
}

export function statusIndicator(text: string, icon: string, color?: string): BarItem {
    return {
        content: text,
        icon,
        position: 'left',
        cssClasses: color ? [`status-${color}`] : [],
        weight: 1.0,
        clickable: false
    }
}

// Complete application layout example
export function createApplicationLayout() {
    const header = headerBar('app-header')
        .left('📝 TextEditor Pro')
        .center('document.txt *')
        .right('👤 User')
        .sticky()
        .build()
    
    const menu = navigationBar('app-menu')
        .item(menuBarItem('File', 'file-menu'))
        .item(menuBarItem('Edit', 'edit-menu'))
        .item(menuBarItem('View', 'view-menu'))
        .item(menuBarItem('Search', 'search-menu'))
        .item(menuBarItem('Tools', 'tools-menu'))
        .item(menuBarItem('Help', 'help-menu'))
        .right('🔍 Search')
        .build()
    
    const toolbarItems = toolbar('app-toolbar')
        .item(iconBarItem('New', '📄', 'left', { clickable: true, action: 'new-file' }))
        .item(iconBarItem('Open', '📂', 'left', { clickable: true, action: 'open-file' }))
        .item(iconBarItem('Save', '💾', 'left', { clickable: true, action: 'save-file' }))
        .item(iconBarItem('Undo', '↶', 'left', { clickable: true, action: 'undo' }))
        .item(iconBarItem('Redo', '↷', 'left', { clickable: true, action: 'redo' }))
        .right('Ready')
        .build()
    
    const status = statusBar('app-status')
        .item(statusIndicator('Ready', '🟢'))
        .item(statusIndicator('TypeScript', '🔷'))
        .center('No issues found')
        .item(iconBarItem('Ln 42, Col 15', '📍', 'right'))
        .right('UTF-8')
        .item(iconBarItem('100%', '🔍', 'right'))
        .build()
    
    const footer = footerBar('app-footer')
        .left('© 2024 TextEditor Pro')
        .center('F1=Help F2=Save F3=Open F4=Exit')
        .right('v2.1.0')
        .build()
    
    return {
        header,
        menu,
        toolbar: toolbarItems,
        status,
        footer
    }
}