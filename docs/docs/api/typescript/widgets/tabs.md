# Tabs Widget

Dynamic tab navigation with content switching, supporting multiple orientations, positions, and customizable styling with lazy content loading.

## Overview

The Tabs widget provides a complete tabbed interface with support for horizontal/vertical orientations, multiple positioning options, icons, badges, closeable tabs, and efficient content management with caching.

```typescript
import { tabs, createTab } from 'reactive-tui-ts'

const editorTabs = tabs({
  id: 'editor-tabs',
  position: 'top',
  tabs: [
    createTab('main', 'main.rs', { icon: '🦀', closeable: true }),
    createTab('lib', 'lib.rs', { icon: '📚', closeable: true }),
    createTab('config', 'Cargo.toml', { badge: '!', closeable: true })
  ],
  activeTab: 0,
  scrollable: true
})
```

## Types

### TabPosition

```typescript
type TabPosition = 'top' | 'bottom' | 'left' | 'right'
```

### TabOrientation

```typescript
type TabOrientation = 'horizontal' | 'vertical'
```

### TabSize

```typescript
type TabSize = 'small' | 'normal' | 'large' | 'custom'
```

### Tab

```typescript
interface Tab {
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
```

### TabStyle

```typescript
interface TabStyle {
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
```

### TabsConfig

```typescript
interface TabsConfig {
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
```

## Basic Usage

### Simple Horizontal Tabs

```typescript
import { tabs, createTab } from 'reactive-tui-ts'

const simpleTabs = tabs({
  id: 'simple-tabs',
  position: 'top',
  tabs: [
    createTab('tab1', 'Home', { content: 'Welcome to the home page!' }),
    createTab('tab2', 'About', { content: 'Learn more about our company.' }),
    createTab('tab3', 'Contact', { content: 'Get in touch with us.' })
  ],
  activeTab: 0
})

// Navigate through tabs
simpleTabs.nextTab()          // Switch to About
simpleTabs.prevTab()          // Back to Home
simpleTabs.setActiveTab(2)    // Jump to Contact

// Get current content
const currentContent = simpleTabs.getActiveContent()
console.log('Current content:', currentContent)
```

### Tabs with Icons and Badges

```typescript
import { createIconTab, createBadgeTab } from 'reactive-tui-ts'

const iconTabs = tabs({
  id: 'icon-tabs',
  position: 'top',
  tabs: [
    createIconTab('dashboard', 'Dashboard', '📊', {
      content: 'Analytics and metrics overview'
    }),
    createBadgeTab('messages', 'Messages', '3', {
      content: 'You have 3 unread messages'
    }),
    createIconTab('settings', 'Settings', '⚙️', {
      content: 'Configure your preferences'
    })
  ],
  activeTab: 0,
  style: {
    padding: 2,
    spacing: 1,
    borderStyle: 'underline'
  }
})
```

### Vertical Tabs

```typescript
import { verticalTabs, createTab } from 'reactive-tui-ts'

const sidebarTabs = verticalTabs('sidebar-tabs')
  .tab(createTab('files', 'Files', { 
    icon: '📁',
    content: 'File explorer content'
  }))
  .tab(createTab('search', 'Search', { 
    icon: '🔍',
    content: 'Search results'
  }))
  .tab(createTab('extensions', 'Extensions', { 
    icon: '🧩',
    content: 'Installed extensions'
  }))
  .active(0)
  .build()

// Vertical tabs automatically adjust their layout
const headerWidth = sidebarTabs.getHeaderWidth() // Width needed for tab headers
console.log('Sidebar width needed:', headerWidth)
```

## Advanced Configuration

### Closeable Tabs with Dynamic Management

```typescript
import { createCloseableTab } from 'reactive-tui-ts'

const documentTabs = tabs({
  id: 'document-tabs',
  position: 'top',
  scrollable: true,
  tabs: [
    createCloseableTab('doc1', 'Document 1.txt', {
      content: 'Content of document 1'
    }),
    createCloseableTab('doc2', 'Untitled', {
      badge: '*',
      content: 'Unsaved document content'
    })
  ]
})

// Add new tab dynamically
documentTabs.addTab(createCloseableTab('doc3', 'New Document', {
  content: 'Fresh document content'
}))

// Close tab by ID
const closedTab = documentTabs.removeTab('doc1')
if (closedTab) {
  console.log(`Closed tab: ${closedTab.label}`)
}

// Update tab properties
documentTabs.updateTab('doc2', {
  label: 'Saved Document.txt',
  badge: undefined // Remove the unsaved indicator
})
```

### Custom Styling and Positioning

```typescript
const styledTabs = tabs({
  id: 'styled-tabs',
  position: 'bottom',
  size: 'large',
  tabs: [
    createTab('tab1', 'Primary', { content: 'Primary content' }),
    createTab('tab2', 'Secondary', { content: 'Secondary content' }),
    createTab('tab3', 'Tertiary', { content: 'Tertiary content' })
  ],
  style: {
    activeBackground: '#007bff',
    inactiveBackground: '#f8f9fa',
    activeText: '#ffffff',
    inactiveText: '#6c757d',
    borderStyle: 'box',
    padding: 3,
    spacing: 2,
    fillWidth: true
  },
  cssClasses: ['custom-tabs', 'bottom-positioned']
})
```

## Builder Pattern

### Fluent Tab Construction

```typescript
import { tabsBuilder, createIconTab } from 'reactive-tui-ts'

const builderTabs = tabsBuilder('builder-example', 'top')
  .size('normal')
  .orientation('horizontal')
  .tab(createIconTab('home', 'Home', '🏠'))
  .tab(createIconTab('profile', 'Profile', '👤'))
  .tab(createIconTab('settings', 'Settings', '⚙️'))
  .active(0)
  .style({
    borderStyle: 'underline',
    padding: 1,
    spacing: 1
  })
  .cssClass('primary-tabs')
  .scrollable()
  .build()

// The builder pattern allows for clean, readable configuration
```

### Convenience Builder Functions

```typescript
import { horizontalTabs, cardTabs, minimalTabs } from 'reactive-tui-ts'

// Pre-configured horizontal tabs
const headerTabs = horizontalTabs('header-navigation')
  .simpleTab('home', 'Home')
  .simpleTab('products', 'Products')
  .simpleTab('about', 'About')
  .build()

// Card-style tabs with box borders
const cardStyleTabs = cardTabs('card-tabs')
  .tab(createTab('overview', 'Overview'))
  .tab(createTab('details', 'Details'))
  .tab(createTab('history', 'History'))
  .build()

// Minimal tabs without borders
const minimalStyleTabs = minimalTabs('minimal-tabs')
  .tab(createTab('view1', 'View 1'))
  .tab(createTab('view2', 'View 2'))
  .build()
```

## Content Management

### Lazy Loading and Caching

```typescript
const contentTabs = tabs({
  id: 'content-tabs',
  tabs: [
    createTab('tab1', 'Tab 1'), // No initial content
    createTab('tab2', 'Tab 2'),
    createTab('tab3', 'Tab 3')
  ]
})

// Set content dynamically (automatically cached)
contentTabs.setTabContent('tab1', 'This content is loaded on demand')
contentTabs.setTabContent('tab2', 'Another piece of content')

// Switch to tab and get its content
contentTabs.setActiveTabById('tab1')
const content = contentTabs.getActiveContent()
console.log('Active content:', content)

// Content is cached for performance
const cachedContent = contentTabs.getActiveContent() // Retrieved from cache
```

### Content Switching

```typescript
const switchingTabs = tabs({
  id: 'switching-demo',
  tabs: [
    createTab('editor', 'Editor', {
      content: 'function hello() {\n  console.log("Hello, World!");\n}'
    }),
    createTab('preview', 'Preview', {
      content: '<h1>Hello, World!</h1>\n<p>This is a preview.</p>'
    }),
    createTab('settings', 'Settings', {
      content: 'Theme: Dark\nFont Size: 14px\nLine Numbers: On'
    })
  ]
})

// Navigate and observe content changes
switchingTabs.setActiveTab(0)
console.log('Editor content:', switchingTabs.getActiveContent())

switchingTabs.setActiveTab(1)
console.log('Preview content:', switchingTabs.getActiveContent())
```

## Tab API Methods

### Tab Management

```typescript
const managedTabs = tabs({
  id: 'managed-tabs',
  tabs: [
    createTab('initial', 'Initial Tab', { content: 'Initial content' })
  ]
})

// Add tabs
managedTabs.addTab(createIconTab('new', 'New Tab', '➕', {
  content: 'Dynamically added tab'
}))

// Get tab information
const tab = managedTabs.getTab('new')
console.log('Tab details:', tab)

// Update tab
managedTabs.updateTab('new', {
  label: 'Updated Tab',
  badge: '1',
  icon: '✨'
})

// Remove tab
const removedTab = managedTabs.removeTab('initial')
console.log('Removed tab:', removedTab?.label)

// Get active tab
const activeTab = managedTabs.getActiveTab()
console.log('Currently active:', activeTab?.label)
```

### Navigation Methods

```typescript
const navTabs = tabs({
  id: 'navigation-tabs',
  tabs: [
    createTab('tab1', 'First'),
    createTab('tab2', 'Second'),
    createTab('tab3', 'Third'),
    createTab('tab4', 'Fourth')
  ]
})

// Navigate through tabs
navTabs.nextTab()              // Move to next tab
navTabs.prevTab()              // Move to previous tab
navTabs.setActiveTab(2)        // Jump to specific index
navTabs.setActiveTabById('tab4') // Jump to specific ID

// Get current position
const activeTab = navTabs.getActiveTab()
console.log('Current tab:', activeTab?.label)
```

### Visibility Control

```typescript
const visibilityTabs = tabs({
  id: 'visibility-demo',
  tabs: [
    createTab('visible1', 'Always Visible'),
    createTab('visible2', 'Sometimes Hidden')
  ],
  visible: true
})

// Toggle visibility
visibilityTabs.setVisible(false)  // Hide entire tab container
visibilityTabs.setVisible(true)   // Show tab container

// Check current visibility state
const tabElement = visibilityTabs.build()
const isVisible = tabElement.attributes['data-visible'] === 'true'
console.log('Tabs visible:', isVisible)
```

## Real-World Examples

### Code Editor with Multiple Panels

```typescript
import { 
  tabs, 
  horizontalTabs, 
  verticalTabs, 
  bottomTabs,
  createIconTab,
  createBadgeTab,
  createCloseableTab
} from 'reactive-tui-ts'

class CodeEditor {
  private editorTabs: any
  private bottomPanel: any
  private sidePanel: any

  constructor() {
    this.setupEditorTabs()
    this.setupBottomPanel()
    this.setupSidePanel()
  }

  private setupEditorTabs() {
    this.editorTabs = horizontalTabs('editor-tabs')
      .tab(createCloseableTab('main', 'main.rs', {
        icon: '🦀',
        content: 'fn main() {\n    println!("Hello, World!");\n}'
      }))
      .tab(createCloseableTab('lib', 'lib.rs', {
        icon: '📚',
        content: 'pub mod widgets;\npub mod themes;'
      }))
      .tab(createBadgeTab('config', 'Cargo.toml', '!', {
        closeable: true,
        content: '[package]\nname = "my-app"\nversion = "0.1.0"'
      }))
      .active(0)
      .scrollable()
      .build()
  }

  private setupBottomPanel() {
    this.bottomPanel = bottomTabs('bottom-panel')
      .tab(createIconTab('terminal', 'Terminal', '💻', {
        content: '$ cargo build\n   Compiling my-app v0.1.0\n   Finished dev profile'
      }))
      .tab(createBadgeTab('problems', 'Problems', '3', {
        content: '3 warnings found:\n- unused variable `x`\n- missing documentation'
      }))
      .tab(createIconTab('output', 'Output', '📄', {
        content: 'Build output:\nCompilation successful'
      }))
      .active(0)
      .build()
  }

  private setupSidePanel() {
    this.sidePanel = verticalTabs('side-panel')
      .tab(createIconTab('explorer', 'Explorer', '📁', {
        content: 'src/\n  main.rs\n  lib.rs\nCargo.toml\nREADME.md'
      }))
      .tab(createIconTab('search', 'Search', '🔍', {
        content: 'Search results:\n- 5 matches in main.rs\n- 2 matches in lib.rs'
      }))
      .tab(createBadgeTab('git', 'Git', '2', {
        content: 'Modified files:\n- main.rs (M)\n- lib.rs (M)\n\nUntracked files:\n- temp.txt'
      }))
      .tab(createIconTab('debug', 'Debug', '🐛', {
        content: 'Breakpoints:\n- main.rs:5\n- lib.rs:12\n\nVariables:\n- x: 42\n- y: "hello"'
      }))
      .active(0)
      .build()
  }

  newFile(filename: string, content: string = '') {
    const fileExtension = filename.split('.').pop()
    let icon = '📄'
    
    // Set icon based on file type
    switch (fileExtension) {
      case 'rs': icon = '🦀'; break
      case 'toml': icon = '⚙️'; break
      case 'md': icon = '📝'; break
      case 'json': icon = '📋'; break
      case 'ts': icon = '📘'; break
      case 'js': icon = '📜'; break
    }

    const tabId = filename.replace(/[^a-zA-Z0-9]/g, '_')
    this.editorTabs.addTab(createCloseableTab(tabId, filename, {
      icon,
      content: content || `// New file: ${filename}\n`
    }))

    // Switch to the new tab
    this.editorTabs.setActiveTabById(tabId)
  }

  closeCurrentFile() {
    const activeTab = this.editorTabs.getActiveTab()
    if (activeTab && activeTab.closeable) {
      this.editorTabs.removeTab(activeTab.id)
    }
  }

  saveCurrentFile() {
    const activeTab = this.editorTabs.getActiveTab()
    if (activeTab) {
      // Remove the unsaved indicator (badge)
      this.editorTabs.updateTab(activeTab.id, {
        badge: undefined
      })
      console.log(`Saved file: ${activeTab.label}`)
    }
  }

  updateProblemCount(count: number) {
    this.bottomPanel.updateTab('problems', {
      badge: count > 0 ? count.toString() : undefined,
      label: count === 1 ? 'Problem' : 'Problems'
    })
  }

  updateGitStatus(modifiedFiles: string[]) {
    const count = modifiedFiles.length
    this.sidePanel.updateTab('git', {
      badge: count > 0 ? count.toString() : undefined
    })

    const gitContent = count > 0 
      ? `Modified files:\n${modifiedFiles.map(f => `- ${f} (M)`).join('\n')}`
      : 'Working directory clean'
    
    this.sidePanel.setTabContent('git', gitContent)
  }

  render() {
    const editorRender = this.editorTabs.render(80, 20)
    const bottomRender = this.bottomPanel.render(80, 8)
    const sideRender = this.sidePanel.render(20, 30)

    return {
      editor: {
        tabs: editorRender.tabs,
        content: editorRender.content,
        activeFile: this.editorTabs.getActiveTab()?.label
      },
      bottom: {
        tabs: bottomRender.tabs,
        content: bottomRender.content,
        activePanel: this.bottomPanel.getActiveTab()?.label
      },
      side: {
        tabs: sideRender.tabs,
        content: sideRender.content,
        activePanel: this.sidePanel.getActiveTab()?.label
      }
    }
  }

  // Keyboard shortcuts
  handleKeyPress(key: string, ctrlKey: boolean = false) {
    if (ctrlKey) {
      switch (key) {
        case 'n':
          // Ctrl+N: New file
          const filename = `untitled${Date.now()}.rs`
          this.newFile(filename)
          break
        
        case 'w':
          // Ctrl+W: Close current file
          this.closeCurrentFile()
          break
        
        case 's':
          // Ctrl+S: Save current file
          this.saveCurrentFile()
          break
        
        case 'Tab':
          // Ctrl+Tab: Next tab
          this.editorTabs.nextTab()
          break
      }
    }
  }
}

// Usage
const editor = new CodeEditor()

// Create some files
editor.newFile('utils.rs', 'pub fn helper() -> String {\n    "Helper function".to_string()\n}')
editor.newFile('tests.rs', '#[cfg(test)]\nmod tests {\n    #[test]\n    fn test_example() {\n        assert_eq!(2 + 2, 4);\n    }\n}')

// Update status
editor.updateProblemCount(1)
editor.updateGitStatus(['main.rs', 'utils.rs', 'tests.rs'])

// Render the complete interface
const rendered = editor.render()
console.log('Editor Interface:', rendered)
```

### Dashboard with Multiple Views

```typescript
import { tabs, createIconTab, createBadgeTab } from 'reactive-tui-ts'

class DashboardManager {
  private mainTabs: any
  private metricsTabs: any

  constructor() {
    this.setupMainTabs()
    this.setupMetricsTabs()
  }

  private setupMainTabs() {
    this.mainTabs = tabs({
      id: 'dashboard-main',
      position: 'top',
      size: 'large',
      tabs: [
        createIconTab('overview', 'Overview', '📊', {
          content: 'System overview and key metrics'
        }),
        createBadgeTab('alerts', 'Alerts', '5', {
          content: '5 active alerts requiring attention'
        }),
        createIconTab('reports', 'Reports', '📈', {
          content: 'Generated reports and analytics'
        }),
        createIconTab('settings', 'Settings', '⚙️', {
          content: 'System configuration and preferences'
        })
      ],
      style: {
        borderStyle: 'underline',
        fillWidth: true,
        padding: 2
      },
      scrollable: true
    })
  }

  private setupMetricsTabs() {
    this.metricsTabs = tabs({
      id: 'metrics-detail',
      position: 'top',
      size: 'normal',
      tabs: [
        createIconTab('cpu', 'CPU', '⚡', {
          content: 'CPU Usage: 45%\nLoad Average: 1.2, 1.5, 1.3'
        }),
        createIconTab('memory', 'Memory', '🧠', {
          content: 'Memory Usage: 8.2GB / 16GB (51%)\nSwap Usage: 256MB / 2GB'
        }),
        createIconTab('disk', 'Disk', '💾', {
          content: 'Disk Usage: 425GB / 1TB (42%)\nI/O: 12MB/s read, 8MB/s write'
        }),
        createIconTab('network', 'Network', '🌐', {
          content: 'Network Traffic: 1.2Mbps up, 5.8Mbps down\nConnections: 45 active'
        })
      ],
      activeTab: 0
    })
  }

  updateAlertCount(count: number) {
    this.mainTabs.updateTab('alerts', {
      badge: count > 0 ? count.toString() : undefined,
      label: count === 1 ? 'Alert' : 'Alerts'
    })
  }

  updateMetrics(metrics: Record<string, string>) {
    // Update CPU metrics
    if (metrics.cpu) {
      this.metricsTabs.setTabContent('cpu', metrics.cpu)
    }
    
    // Update memory metrics
    if (metrics.memory) {
      this.metricsTabs.setTabContent('memory', metrics.memory)
    }
    
    // Update disk metrics
    if (metrics.disk) {
      this.metricsTabs.setTabContent('disk', metrics.disk)
    }
    
    // Update network metrics
    if (metrics.network) {
      this.metricsTabs.setTabContent('network', metrics.network)
    }
  }

  switchToAlertsView() {
    this.mainTabs.setActiveTabById('alerts')
  }

  switchToMetricDetail(metric: string) {
    this.mainTabs.setActiveTabById('overview')
    this.metricsTabs.setActiveTabById(metric)
  }

  render() {
    return {
      main: this.mainTabs.render(100, 15),
      metrics: this.metricsTabs.render(100, 10)
    }
  }
}

// Usage
const dashboard = new DashboardManager()

// Update with real-time data
dashboard.updateAlertCount(3)
dashboard.updateMetrics({
  cpu: 'CPU Usage: 67%\nLoad Average: 2.1, 1.8, 1.6\nTemperature: 72°C',
  memory: 'Memory Usage: 12.8GB / 16GB (80%)\nSwap Usage: 1.2GB / 2GB\nCache: 2.1GB',
  disk: 'Disk Usage: 456GB / 1TB (45%)\nI/O: 25MB/s read, 15MB/s write\nQueue Depth: 3',
  network: 'Traffic: 2.8Mbps up, 12.5Mbps down\nConnections: 128 active\nPacket Loss: 0.02%'
})

const rendered = dashboard.render()
console.log('Dashboard rendered:', rendered)
```

## CSS Styling

```css
/* Tab container */
.tabs {
  display: flex;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

.tabs-top {
  flex-direction: column;
}

.tabs-bottom {
  flex-direction: column-reverse;
}

.tabs-left {
  flex-direction: row;
}

.tabs-right {
  flex-direction: row-reverse;
}

.tabs-hidden {
  display: none;
}

/* Tab headers container */
.tabs-headers {
  display: flex;
  border-bottom: 2px solid #e2e8f0;
}

.tabs-left .tabs-headers,
.tabs-right .tabs-headers {
  flex-direction: column;
  border-bottom: none;
  border-right: 2px solid #e2e8f0;
}

.tabs-bottom .tabs-headers {
  border-bottom: none;
  border-top: 2px solid #e2e8f0;
}

/* Individual tabs */
.tab-header {
  padding: 0.5rem 1rem;
  background: #f8f9fa;
  border: 1px solid #dee2e6;
  border-bottom: none;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: all 0.2s ease;
}

.tab-header:hover {
  background: #e9ecef;
}

.tab-header.active {
  background: #ffffff;
  border-bottom: 2px solid #ffffff;
  position: relative;
  z-index: 1;
}

.tab-header.disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: #f8f9fa;
}

.tab-header.disabled:hover {
  background: #f8f9fa;
}

/* Tab sizes */
.tabs-small .tab-header {
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
}

.tabs-large .tab-header {
  padding: 0.75rem 1.5rem;
  font-size: 1.125rem;
}

/* Tab content */
.tab-content {
  flex: 1;
  padding: 1rem;
  background: #ffffff;
  border: 1px solid #dee2e6;
  border-top: none;
}

.tabs-bottom .tab-content {
  border-top: 1px solid #dee2e6;
  border-bottom: none;
}

.tabs-left .tab-content,
.tabs-right .tab-content {
  border-top: 1px solid #dee2e6;
  border-left: none;
}

/* Tab elements */
.tab-icon {
  font-size: 1rem;
}

.tab-label {
  font-weight: 500;
}

.tab-badge {
  background: #dc3545;
  color: white;
  border-radius: 10px;
  padding: 0.125rem 0.375rem;
  font-size: 0.75rem;
  font-weight: bold;
  min-width: 1rem;
  text-align: center;
}

.tab-close {
  opacity: 0.6;
  margin-left: 0.5rem;
  padding: 0.125rem;
  border-radius: 2px;
  transition: all 0.15s ease;
}

.tab-close:hover {
  opacity: 1;
  background: rgba(220, 53, 69, 0.1);
  color: #dc3545;
}

/* Scrollable tabs */
.tabs-scrollable .tabs-headers {
  overflow-x: auto;
  overflow-y: hidden;
  scrollbar-width: thin;
}

.tabs-scrollable .tabs-headers::-webkit-scrollbar {
  height: 4px;
}

.tabs-scrollable .tabs-headers::-webkit-scrollbar-track {
  background: #f1f1f1;
}

.tabs-scrollable .tabs-headers::-webkit-scrollbar-thumb {
  background: #c1c1c1;
  border-radius: 2px;
}

/* Vertical scrollable tabs */
.tabs-scrollable.tabs-left .tabs-headers,
.tabs-scrollable.tabs-right .tabs-headers {
  overflow-x: hidden;
  overflow-y: auto;
}

/* Border styles */
.tabs[data-border-style="none"] .tab-header {
  border: none;
  background: transparent;
}

.tabs[data-border-style="line"] .tab-header {
  border: none;
  border-bottom: 2px solid transparent;
}

.tabs[data-border-style="line"] .tab-header.active {
  border-bottom: 2px solid #007bff;
}

.tabs[data-border-style="underline"] .tabs-headers {
  border-bottom: 1px solid #dee2e6;
}

.tabs[data-border-style="underline"] .tab-header {
  border: none;
  border-bottom: 2px solid transparent;
  background: transparent;
}

.tabs[data-border-style="underline"] .tab-header.active {
  border-bottom: 2px solid #007bff;
}

/* Fill width */
.tabs[data-fill-width="true"] .tab-header {
  flex: 1;
  justify-content: center;
}

/* Custom styling classes */
.tabs.primary-tabs .tab-header.active {
  background: #007bff;
  color: white;
}

.tabs.card-tabs .tab-header {
  border-radius: 6px 6px 0 0;
  margin-right: 2px;
}

.tabs.minimal-tabs .tab-header {
  border: none;
  background: transparent;
  padding: 0.5rem;
}

.tabs.minimal-tabs .tab-header.active {
  background: rgba(0, 123, 255, 0.1);
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .tabs-headers {
    border-color: #4a5568;
  }
  
  .tab-header {
    background: #2d3748;
    color: #e2e8f0;
    border-color: #4a5568;
  }
  
  .tab-header:hover {
    background: #4a5568;
  }
  
  .tab-header.active {
    background: #1a202c;
    color: #ffffff;
  }
  
  .tab-content {
    background: #1a202c;
    color: #e2e8f0;
    border-color: #4a5568;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .tab-header {
    border-width: 2px;
  }
  
  .tab-header.active {
    border-color: #000000;
    border-width: 3px;
  }
}

/* Animation for smooth transitions */
.tab-content {
  animation: fadeIn 0.2s ease-in-out;
}

@keyframes fadeIn {
  from {
    opacity: 0;
    transform: translateY(-10px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

/* Responsive design */
@media (max-width: 768px) {
  .tabs-left,
  .tabs-right {
    flex-direction: column;
  }
  
  .tabs-left .tabs-headers,
  .tabs-right .tabs-headers {
    flex-direction: row;
    border-right: none;
    border-bottom: 2px solid #e2e8f0;
  }
  
  .tab-header {
    padding: 0.375rem 0.75rem;
    font-size: 0.875rem;
  }
  
  .tabs-scrollable .tabs-headers {
    overflow-x: auto;
    overflow-y: hidden;
  }
}
```

## Best Practices

### 1. Use Appropriate Tab Positioning

```typescript
// ✅ Good - horizontal tabs for main navigation
const mainNavigation = horizontalTabs('main-nav')
  .tab(createTab('dashboard', 'Dashboard'))
  .tab(createTab('projects', 'Projects'))
  .tab(createTab('settings', 'Settings'))
  .build()

// ✅ Good - vertical tabs for sidebar navigation
const sidebarNavigation = verticalTabs('sidebar')
  .tab(createIconTab('files', 'Files', '📁'))
  .tab(createIconTab('search', 'Search', '🔍'))
  .tab(createIconTab('git', 'Git', '🌿'))
  .build()
```

### 2. Provide Clear Visual Hierarchy

```typescript
// ✅ Good - meaningful icons and badges
const documentTabs = tabs({
  id: 'documents',
  tabs: [
    createIconTab('readme', 'README.md', '📝', { 
      badge: '*', // Unsaved indicator
      closeable: true 
    }),
    createIconTab('config', 'config.json', '⚙️', { 
      closeable: true 
    }),
    createBadgeTab('errors', 'Errors', '3', { 
      content: 'Found 3 errors that need attention' 
    })
  ]
})
```

### 3. Handle Content Efficiently

```typescript
// ✅ Good - lazy loading for performance
const performantTabs = tabs({
  id: 'lazy-tabs',
  tabs: [
    createTab('tab1', 'Quick Load'), // No initial content
    createTab('tab2', 'Heavy Data'),
    createTab('tab3', 'Large Report')
  ]
})

// Load content only when tab becomes active
performantTabs.setTabContent('tab2', await loadHeavyData())
performantTabs.setTabContent('tab3', await generateReport())
```

### 4. Implement Proper Keyboard Navigation

```typescript
// ✅ Good - keyboard accessibility
const accessibleTabs = tabs({
  id: 'accessible',
  tabs: [
    createTab('tab1', 'First Tab'),
    createTab('tab2', 'Second Tab'),
    createTab('tab3', 'Third Tab')
  ]
})

// Handle keyboard navigation
const handleKeyDown = (event: KeyboardEvent) => {
  if (event.ctrlKey) {
    switch (event.key) {
      case 'Tab':
        event.preventDefault()
        accessibleTabs.nextTab()
        break
      case 'ArrowLeft':
        accessibleTabs.prevTab()
        break
      case 'ArrowRight':
        accessibleTabs.nextTab()
        break
    }
  }
}
```

### 5. Manage Tab State Properly

```typescript
// ✅ Good - proper state management
const stateManagementTabs = tabs({
  id: 'state-tabs',
  tabs: [
    createCloseableTab('doc1', 'Document 1'),
    createCloseableTab('doc2', 'Document 2')
  ]
})

// Always check if tabs exist before operations
const safeCloseTab = (tabId: string) => {
  const tab = stateManagementTabs.getTab(tabId)
  if (tab && tab.closeable) {
    const removed = stateManagementTabs.removeTab(tabId)
    if (removed) {
      console.log(`Closed: ${removed.label}`)
    }
  }
}

// Handle empty tab state gracefully
const activeTab = stateManagementTabs.getActiveTab()
if (activeTab) {
  console.log(`Current tab: ${activeTab.label}`)
} else {
  console.log('No tabs available')
}
```

## Related Widgets

- **[Panel](./panel)** - Container panels for tab content
- **[Accordion](./accordion)** - Collapsible content sections
- **[Menu](./menu)** - Navigation menus and dropdowns
- **[Modal](./modal)** - Overlay dialogs and popups

## Examples

- **[Basic Tabs](../../examples/basic/tabs-basic)** - Simple tab implementations
- **[Editor Tabs](../../examples/advanced/editor-tabs)** - Code editor with file tabs
- **[Dashboard Tabs](../../examples/apps/dashboard-tabs)** - Multi-view dashboard
- **[Mobile Tabs](../../examples/responsive/mobile-tabs)** - Responsive tab layouts

The Tabs widget provides comprehensive tab navigation functionality with support for multiple orientations, dynamic content management, and extensive customization options for building sophisticated multi-view interfaces.