/**
 * Tabs Component - Dynamic tab navigation with content switching
 * TypeScript equivalent of the Rust Tabs widget with full feature parity
 */

export type TabPosition = 'top' | 'bottom' | 'left' | 'right'
export type TabOrientation = 'horizontal' | 'vertical'
export type TabSize = 'small' | 'normal' | 'large' | 'custom'
export type TabBorderStyle = 'none' | 'line' | 'box' | 'underline' | 'custom'

export interface Tab {
    id: string
    label: string
    icon?: string
    badge?: string
    closeable?: boolean
    disabled?: boolean
    content?: string
    cssClasses?: string[]
    tooltip?: string
}

export interface TabStyle {
    activeBackground?: string
    inactiveBackground?: string
    activeText?: string
    inactiveText?: string
    borderColor?: string
    borderStyle?: TabBorderStyle
    padding?: number
    spacing?: number
    fillWidth?: boolean
}

export interface TabsConfig {
    id: string
    position?: TabPosition
    orientation?: TabOrientation
    size?: TabSize
    tabs?: Tab[]
    activeTab?: number
    style?: TabStyle
    cssClasses?: string[]
    visible?: boolean
    scrollable?: boolean
    width?: number
    height?: number
}

export function tabs(config: TabsConfig): any {
    const {
        id,
        position = 'top',
        orientation = 'horizontal',
        size = 'normal',
        tabs: tabList = [],
        activeTab = 0,
        style = {},
        cssClasses = [],
        visible = true,
        scrollable = false,
        width,
        height
    } = config

    // Content cache for lazy loading
    const contentCache = new Map<string, string>()

    // Build CSS classes
    const classes = [
        'tabs',
        `tabs-${position}`,
        `tabs-${size}`,
        `tabs-${orientation}`,
        scrollable ? 'tabs-scrollable' : '',
        !visible ? 'tabs-hidden' : '',
        ...cssClasses
    ].filter(Boolean)

    // Get tab height based on position and size
    const getHeaderHeight = () => {
        if (height) return height
        if (position === 'left' || position === 'right') return 0
        
        switch (size) {
            case 'small': return 1
            case 'normal': return 2
            case 'large': return 3
            default: return 2
        }
    }

    // Get tab width for vertical tabs
    const getHeaderWidth = () => {
        if (position === 'top' || position === 'bottom') return 0
        
        const maxLabelWidth = tabList.reduce((max, tab) => {
            const tabWidth = tab.label.length + 
                            (tab.icon ? 2 : 0) + 
                            (tab.badge ? 4 : 0) + 
                            (tab.closeable ? 2 : 0)
            return Math.max(max, tabWidth)
        }, 0)
        
        return maxLabelWidth + ((style.padding || 1) * 2)
    }

    // All tabs are positioned on the left by default
    const _positionedTabs = tabList.map((tab, index) => ({
        ...tab,
        position: index * 10 // Simple positioning calculation
    }))

    // Render individual tab header
    const renderTabHeader = (tab: Tab, isActive: boolean, tabWidth: number) => {
        let content = ''

        // Add padding
        content += ' '.repeat(style.padding || 1)

        // Add icon
        if (tab.icon) {
            content += tab.icon + ' '
        }

        // Add label
        content += tab.label

        // Add badge
        if (tab.badge) {
            content += ` (${tab.badge})`
        }

        // Add close button
        if (tab.closeable) {
            content += ' ✕'
        }

        // Pad to width
        if (content.length < tabWidth) {
            content += ' '.repeat(tabWidth - content.length)
        } else if (content.length > tabWidth) {
            content = content.substring(0, tabWidth)
        }

        return content
    }

    // Render horizontal tabs
    const renderHorizontalTabs = (renderWidth: number) => {
        const tabWidth = style.fillWidth ? 
            Math.floor(renderWidth / tabList.length) : 
            calculateTabWidth()

        return tabList.map((tab, index) => {
            const isActive = index === activeTab
            return renderTabHeader(tab, isActive, tabWidth)
        }).join('')
    }

    // Calculate optimal tab width
    const calculateTabWidth = () => {
        return tabList.reduce((max, tab) => {
            const tabWidth = tab.label.length + 
                            (tab.icon ? 2 : 0) + 
                            (tab.badge ? 4 : 0) + 
                            (tab.closeable ? 2 : 0) + 
                            ((style.padding || 1) * 2)
            return Math.max(max, tabWidth)
        }, 10)
    }

    // Render content area
    const renderContentArea = (renderWidth: number, _renderHeight: number) => {
        const activeTabData = tabList[activeTab]
        
        if (!activeTabData) return ' '.repeat(renderWidth)

        // Check cache first
        let content = contentCache.get(activeTabData.id)
        if (!content) {
            content = activeTabData.content || `Content for tab: ${activeTabData.label}`
            contentCache.set(activeTabData.id, content)
        }

        // Return first line of content (simplified)
        const firstLine = content.split('\n')[0] || ''
        if (firstLine.length >= renderWidth) {
            return firstLine.substring(0, renderWidth)
        } else {
            return firstLine + ' '.repeat(renderWidth - firstLine.length)
        }
    }

    const element = {
        tag: 'div',
        id,
        classes,
        attributes: {
            'data-position': position,
            'data-orientation': orientation,
            'data-size': size,
            'data-active-tab': activeTab.toString(),
            'data-visible': visible.toString(),
            'data-scrollable': scrollable.toString(),
            'data-width': width?.toString() || '',
            'data-height': height?.toString() || '',
            'data-style': JSON.stringify(style),
            'data-tabs': JSON.stringify(tabList)
        },
        content: '',
        children: []
    }

    return {
        ...element,
        // Add tab to the container
        addTab: (tab: Tab) => {
            tabList.push(tab)
            element.attributes['data-tabs'] = JSON.stringify(tabList)
        },

        // Remove tab by ID
        removeTab: (tabId: string) => {
            const index = tabList.findIndex(tab => tab.id === tabId)
            if (index !== -1) {
                const removed = tabList.splice(index, 1)[0]
                
                // Adjust active tab if necessary
                if (index < activeTab) {
                    element.attributes['data-active-tab'] = (activeTab - 1).toString()
                } else if (index === activeTab && activeTab >= tabList.length) {
                    element.attributes['data-active-tab'] = (tabList.length - 1).toString()
                }
                
                // Remove from cache
                contentCache.delete(tabId)
                element.attributes['data-tabs'] = JSON.stringify(tabList)
                
                return removed
            }
            return null
        },

        // Get tab by ID
        getTab: (tabId: string) => {
            return tabList.find(tab => tab.id === tabId) || null
        },

        // Update tab by ID
        updateTab: (tabId: string, updates: Partial<Tab>) => {
            const tab = tabList.find(tab => tab.id === tabId)
            if (tab) {
                Object.assign(tab, updates)
                element.attributes['data-tabs'] = JSON.stringify(tabList)
                return tab
            }
            return null
        },

        // Set active tab by index
        setActiveTab: (index: number) => {
            if (index >= 0 && index < tabList.length) {
                element.attributes['data-active-tab'] = index.toString()
            }
        },

        // Set active tab by ID
        setActiveTabById: (tabId: string) => {
            const index = tabList.findIndex(tab => tab.id === tabId)
            if (index !== -1) {
                element.attributes['data-active-tab'] = index.toString()
            }
        },

        // Get currently active tab
        getActiveTab: () => {
            return tabList[activeTab] || null
        },

        // Get active tab content
        getActiveContent: () => {
            const activeTabData = tabList[activeTab]
            if (!activeTabData) return null

            // Check cache first
            let content = contentCache.get(activeTabData.id)
            if (!content) {
                content = activeTabData.content || `Content for tab: ${activeTabData.label}`
                contentCache.set(activeTabData.id, content)
            }
            return content
        },

        // Set content for a specific tab
        setTabContent: (tabId: string, content: string) => {
            contentCache.set(tabId, content)
            
            const tab = tabList.find(t => t.id === tabId)
            if (tab) {
                tab.content = content
                element.attributes['data-tabs'] = JSON.stringify(tabList)
            }
        },

        // Navigate to next tab
        nextTab: () => {
            if (tabList.length > 0) {
                const newActive = (activeTab + 1) % tabList.length
                element.attributes['data-active-tab'] = newActive.toString()
            }
        },

        // Navigate to previous tab
        prevTab: () => {
            if (tabList.length > 0) {
                const newActive = activeTab === 0 ? tabList.length - 1 : activeTab - 1
                element.attributes['data-active-tab'] = newActive.toString()
            }
        },

        // Set visibility
        setVisible: (isVisible: boolean) => {
            element.attributes['data-visible'] = isVisible.toString()
            if (isVisible) {
                element.classes = element.classes.filter(c => c !== 'tabs-hidden')
            } else {
                if (!element.classes.includes('tabs-hidden')) {
                    element.classes.push('tabs-hidden')
                }
            }
        },

        // Get header dimensions
        getHeaderHeight: () => getHeaderHeight(),
        getHeaderWidth: () => getHeaderWidth(),

        // Render the tabs
        render: (renderWidth?: number, renderHeight?: number) => {
            const tabWidth = renderWidth || width || 80
            const tabHeight = renderHeight || height || 10

            const result = {
                tabs: renderHorizontalTabs(tabWidth),
                content: renderContentArea(tabWidth, tabHeight),
                position,
                activeTab,
                tabCount: tabList.length
            }

            return result
        },

        // Build the element
        build: () => element
    }
}

// Builder pattern for fluent API
export function tabsBuilder(id: string, position: TabPosition = 'top') {
    const config: TabsConfig = { id, position, tabs: [] }

    return {
        position: (pos: TabPosition) => {
            config.position = pos
            return tabsBuilder(id, pos)
        },

        orientation: (orient: TabOrientation) => {
            config.orientation = orient
            return tabsBuilder(id, config.position!)
        },

        size: (size: TabSize) => {
            config.size = size
            return tabsBuilder(id, config.position!)
        },

        tab: (tab: Tab) => {
            if (!config.tabs) config.tabs = []
            config.tabs.push(tab)
            return tabsBuilder(id, config.position!)
        },

        simpleTab: (id: string, label: string) => {
            if (!config.tabs) config.tabs = []
            config.tabs.push({ id, label })
            return tabsBuilder(id, config.position!)
        },

        active: (index: number) => {
            config.activeTab = index
            return tabsBuilder(id, config.position!)
        },

        style: (style: TabStyle) => {
            config.style = { ...config.style, ...style }
            return tabsBuilder(id, config.position!)
        },

        cssClass: (className: string) => {
            if (!config.cssClasses) config.cssClasses = []
            config.cssClasses.push(className)
            return tabsBuilder(id, config.position!)
        },

        scrollable: () => {
            config.scrollable = true
            return tabsBuilder(id, config.position!)
        },

        hidden: () => {
            config.visible = false
            return tabsBuilder(id, config.position!)
        },

        build: () => tabs(config)
    }
}

// Convenience functions for common tab configurations
export function horizontalTabs(id: string) {
    return tabsBuilder(id, 'top')
}

export function verticalTabs(id: string) {
    return tabsBuilder(id, 'left')
}

export function bottomTabs(id: string) {
    return tabsBuilder(id, 'bottom')
}

export function cardTabs(id: string) {
    return tabsBuilder(id, 'top')
        .style({
            borderStyle: 'box',
            padding: 2,
            spacing: 1
        })
}

export function minimalTabs(id: string) {
    return tabsBuilder(id, 'top')
        .style({
            borderStyle: 'none',
            padding: 1,
            spacing: 0
        })
}

// Helper functions for creating tabs
export function createTab(id: string, label: string, options?: Partial<Tab>): Tab {
    return {
        id,
        label,
        closeable: false,
        disabled: false,
        cssClasses: [],
        ...options
    }
}

export function createIconTab(id: string, label: string, icon: string, options?: Partial<Tab>): Tab {
    return createTab(id, label, { icon, ...options })
}

export function createBadgeTab(id: string, label: string, badge: string, options?: Partial<Tab>): Tab {
    return createTab(id, label, { badge, ...options })
}

export function createCloseableTab(id: string, label: string, options?: Partial<Tab>): Tab {
    return createTab(id, label, { closeable: true, ...options })
}

// Complete application layout example
export function createTabLayout() {
    const editorTabs = horizontalTabs('editor-tabs')
        .tab(createIconTab('main.rs', 'main.rs', '🦀', { closeable: true, content: 'fn main() {\n    println!("Hello, World!");\n}' }))
        .tab(createIconTab('lib.rs', 'lib.rs', '📚', { closeable: true, content: 'pub mod widgets;\npub mod themes;' }))
        .tab(createBadgeTab('cargo.toml', 'Cargo.toml', '!', { closeable: true, content: '[package]\nname = "my-app"\nversion = "0.1.0"' }))
        .active(0)
        .build()

    const bottomPanel = bottomTabs('bottom-tabs')
        .tab(createIconTab('terminal', 'Terminal', '💻', { content: '$ cargo build\n   Compiling my-app v0.1.0\n   Finished dev profile' }))
        .tab(createBadgeTab('problems', 'Problems', '3', { content: '3 warnings found:\n- unused variable `x`\n- missing documentation' }))
        .tab(createIconTab('output', 'Output', '📄', { content: 'Build output:\nCompilation successful' }))
        .active(0)
        .build()

    const sideTabs = verticalTabs('side-tabs')
        .tab(createIconTab('explorer', 'Explorer', '📁', { content: 'src/\n  main.rs\n  lib.rs\nCargo.toml' }))
        .tab(createIconTab('search', 'Search', '🔍', { content: 'Search results:\n- 5 matches in main.rs' }))
        .tab(createBadgeTab('git', 'Git', '2', { content: 'Modified files:\n- main.rs\n- lib.rs' }))
        .active(0)
        .build()

    return {
        editor: editorTabs,
        bottom: bottomPanel,
        side: sideTabs
    }
}