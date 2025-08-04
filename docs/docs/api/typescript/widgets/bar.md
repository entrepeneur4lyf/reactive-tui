# Bar Widget

Flexible header, footer, navigation, and toolbar bars with content positioning, icons, actions, and comprehensive styling options.

## Overview

The Bar widget provides a complete bar system for headers, footers, navigation menus, toolbars, and status bars with flexible content positioning, clickable items, icons, and advanced layout control.

```typescript
import { bar, headerBar, BarPosition } from 'reactive-tui-ts'

const applicationHeader = headerBar('app-header')
  .left('📝 My Application')
  .center('Document: untitled.txt')
  .right('👤 John Doe')
  .sticky()
  .build()
```

## Types

### BarType

```typescript
type BarType = 'header' | 'footer' | 'navigation' | 'status' | 'toolbar' | 'tabbar' | 'custom'
```

### BarPosition

```typescript
type BarPosition = 'left' | 'center' | 'right' | 'justify'
```

### BarBorderStyle

```typescript
type BarBorderStyle = 'none' | 'single' | 'double' | 'thick' | 'custom'
```

### BarSize

```typescript
type BarSize = 'compact' | 'normal' | 'large' | 'custom'
```

### BarItem

```typescript
interface BarItem {
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
```

### BarStyle

```typescript
interface BarStyle {
  background?: string
  foreground?: string
  borderColor?: string
  borderStyle?: BarBorderStyle
  padding?: number
  separator?: string
  fillWidth?: boolean
}
```

### BarConfig

```typescript
interface BarConfig {
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
```

## Basic Usage

### Simple Bar

```typescript
import { bar } from 'reactive-tui-ts'

const basicBar = bar({
  id: 'basic-bar',
  barType: 'header',
  items: [
    { content: 'App Name', position: 'left' },
    { content: 'Page Title', position: 'center' },
    { content: 'User', position: 'right' }
  ]
})

// Access bar methods
const height = basicBar.getHeight()
const rendered = basicBar.render(80)
console.log('Bar height:', height)
console.log('Rendered content:', rendered)
```

### Bar with Icons

```typescript
const iconBar = bar({
  id: 'icon-bar',
  barType: 'toolbar',
  items: [
    {
      content: 'New File',
      position: 'left',
      icon: '📄',
      clickable: true,
      action: 'new-file'
    },
    {
      content: 'Save',
      position: 'left',
      icon: '💾',
      clickable: true,
      action: 'save-file',
      hotkey: 'Ctrl+S'
    },
    {
      content: 'Status: Ready',
      position: 'right',
      icon: '✅'
    }
  ]
})
```

### Weighted Positioning

```typescript
const weightedBar = bar({
  id: 'weighted-bar',
  items: [
    {
      content: 'Section 1',
      position: 'justify',
      weight: 2  // Takes 2x space
    },
    {
      content: 'Section 2',
      position: 'justify',
      weight: 1  // Takes 1x space
    },
    {
      content: 'Section 3',
      position: 'justify',
      weight: 3  // Takes 3x space
    }
  ]
})
// Content distributed proportionally across bar width
```

## Bar Types and Pre-built Styles

### Header Bar

```typescript
import { headerBar } from 'reactive-tui-ts'

const header = headerBar('main-header')
  .left('🏠 Home')
  .center('Dashboard')
  .right('Settings ⚙️')
  .sticky()
  .build()

// Pre-configured with blue background and white text
```

### Footer Bar

```typescript
import { footerBar } from 'reactive-tui-ts'

const footer = footerBar('main-footer')
  .left('© 2024 Company Name')
  .center('Privacy | Terms | Support')
  .right('Version 1.2.3')
  .build()

// Pre-configured with gray background
```

### Status Bar

```typescript
import { statusBar } from 'reactive-tui-ts'

const status = statusBar('app-status')
  .left('Ready')
  .center('No errors')
  .right('Line 42, Col 15')
  .build()

// Compact size with green background
```

### Navigation Bar

```typescript
import { navigationBar } from 'reactive-tui-ts'

const nav = navigationBar('main-nav')
  .left('File')
  .left('Edit')
  .left('View')
  .left('Tools')
  .right('Help')
  .build()

// Dark theme with light text
```

### Toolbar

```typescript
import { toolbar } from 'reactive-tui-ts'

const tools = toolbar('main-toolbar')
  .left('New')
  .left('Open')
  .left('Save')
  .right('Zoom: 100%')
  .build()

// Light background with border
```

## Builder Pattern

### Fluent Bar Construction

```typescript
import { barBuilder } from 'reactive-tui-ts'

const complexBar = barBuilder('complex-bar', 'header')
  .size('large')
  .background('#1e40af')
  .foreground('#ffffff')
  .border('double')
  .padding(2)
  .left('📱 Mobile App', { icon: '📱', clickable: true })
  .center('Current Page', { weight: 2 })
  .right('Online', { icon: '🟢' })
  .right('👤 Admin', { clickable: true, action: 'profile' })
  .cssClass('main-header')
  .cssClass('sticky-header')
  .sticky()
  .build()
```

### Dynamic Bar Management

```typescript
const dynamicBar = barBuilder('dynamic-bar', 'toolbar')
  .build()

// Add items dynamically
dynamicBar.addItem({
  id: 'dynamic-item-1',
  content: 'Dynamic Content',
  position: 'left',
  icon: '⚡'
})

// Update existing item
dynamicBar.updateItem('dynamic-item-1', {
  content: 'Updated Content',
  icon: '✨'
})

// Remove item
const removed = dynamicBar.removeItem('dynamic-item-1')
console.log('Removed item:', removed)

// Get item by ID
const item = dynamicBar.getItem('dynamic-item-1')
console.log('Found item:', item)
```

## Bar Item Helpers

### Basic Bar Items

```typescript
import { barItem, clickableBarItem, iconBarItem, weightedBarItem } from 'reactive-tui-ts'

// Simple text item
const textItem = barItem('Simple Text', 'left')

// Clickable item with action
const buttonItem = clickableBarItem('Click Me', 'center', 'button-action')

// Item with icon
const iconItem = iconBarItem('Status', '✅', 'right')

// Weighted item for justify positioning
const weightedItem = weightedBarItem('Wide Section', 'justify', 3)
```

### Specialized Items

```typescript
import { menuBarItem, statusIndicator } from 'reactive-tui-ts'

// Menu item with hotkey
const fileMenu = menuBarItem('File', 'file-menu', 'Alt+F')

// Status indicator with color
const errorStatus = statusIndicator('3 Errors', '❌', 'red')
const successStatus = statusIndicator('All Good', '✅', 'green')
```

## Bar Sizes and Styling

### Size Variations

```typescript
// Compact bar (3 units high)
const compactBar = barBuilder('compact', 'status')
  .size('compact')
  .left('Ready')
  .build()

// Normal bar (4 units high)
const normalBar = barBuilder('normal', 'header')
  .size('normal')
  .left('Application')
  .build()

// Large bar (5 units high)
const largeBar = barBuilder('large', 'header')
  .size('large')
  .left('🏢 Enterprise App')
  .build()
```

### Custom Styling

```typescript
const styledBar = barBuilder('styled-bar', 'custom')
  .background('#2563eb')
  .foreground('#f8fafc')
  .border('thick')
  .padding(3)
  .left('Custom Styled Bar')
  .build()

// Custom style object
const customBar = bar({
  id: 'custom-style',
  style: {
    background: '#dc2626',
    foreground: '#fef2f2',
    borderStyle: 'double',
    borderColor: '#f87171',
    padding: 2,
    separator: ' | ',
    fillWidth: true
  },
  items: [
    { content: 'Alert', position: 'left', icon: '⚠️' },
    { content: 'System Error', position: 'center' },
    { content: 'Fix Now', position: 'right', clickable: true }
  ]
})
```

## Visibility and Behavior

### Sticky and Hidden Bars

```typescript
const behaviorBar = barBuilder('behavior-bar', 'header')
  .left('Sticky Header')
  .sticky()  // Stays at top when scrolling
  .build()

// Toggle visibility
behaviorBar.setVisible(false)  // Hide bar
behaviorBar.setVisible(true)   // Show bar

// Toggle sticky behavior
behaviorBar.setSticky(true)    // Make sticky
behaviorBar.setSticky(false)   // Remove sticky
```

### Responsive Width

```typescript
const responsiveBar = bar({
  id: 'responsive',
  items: [
    { content: 'Responsive', position: 'left' },
    { content: 'Center Content', position: 'center' },
    { content: 'Right', position: 'right' }
  ]
})

// Render at different widths
const narrow = responsiveBar.render(40)
const wide = responsiveBar.render(120)

console.log('Narrow render:', narrow)
console.log('Wide render:', wide)
```

## Real-World Examples

### Complete Application Interface

```typescript
import { 
  headerBar, 
  navigationBar, 
  toolbar, 
  statusBar, 
  footerBar,
  menuBarItem,
  iconBarItem,
  statusIndicator
} from 'reactive-tui-ts'

class ApplicationInterface {
  private headerBar: any
  private menuBar: any
  private toolbarBar: any
  private statusBar: any
  private footerBar: any
  
  private currentFile: string = 'untitled.txt'
  private isModified: boolean = false
  private lineNumber: number = 1
  private columnNumber: number = 1
  private zoom: number = 100

  constructor() {
    this.setupInterface()
  }

  private setupInterface() {
    // Application header
    this.headerBar = headerBar('app-header')
      .left('📝 CodeEditor Pro')
      .center(this.getDocumentTitle())
      .right('👤 Developer')
      .sticky()
      .build()

    // Menu bar
    this.menuBar = navigationBar('app-menu')
      .item(menuBarItem('File', 'file-menu', 'Alt+F'))
      .item(menuBarItem('Edit', 'edit-menu', 'Alt+E'))
      .item(menuBarItem('View', 'view-menu', 'Alt+V'))
      .item(menuBarItem('Selection', 'selection-menu', 'Alt+S'))
      .item(menuBarItem('Go', 'go-menu', 'Alt+G'))
      .item(menuBarItem('Run', 'run-menu', 'Alt+R'))
      .item(menuBarItem('Terminal', 'terminal-menu', 'Alt+T'))
      .item(menuBarItem('Help', 'help-menu', 'Alt+H'))
      .right('🔍 Search')
      .build()

    // Toolbar with common actions
    this.toolbarBar = toolbar('app-toolbar')
      .item(iconBarItem('New', '📄', 'left', { 
        clickable: true, 
        action: 'new-file',
        hotkey: 'Ctrl+N',
        id: 'new-file'
      }))
      .item(iconBarItem('Open', '📂', 'left', { 
        clickable: true, 
        action: 'open-file',
        hotkey: 'Ctrl+O',
        id: 'open-file'
      }))
      .item(iconBarItem('Save', '💾', 'left', { 
        clickable: true, 
        action: 'save-file',
        hotkey: 'Ctrl+S',
        id: 'save-file'
      }))
      .item(iconBarItem('Undo', '↶', 'left', { 
        clickable: true, 
        action: 'undo',
        hotkey: 'Ctrl+Z',
        id: 'undo'
      }))
      .item(iconBarItem('Redo', '↷', 'left', { 
        clickable: true, 
        action: 'redo',
        hotkey: 'Ctrl+Y',
        id: 'redo'
      }))
      .item(iconBarItem('Find', '🔍', 'left', { 
        clickable: true, 
        action: 'find',
        hotkey: 'Ctrl+F',
        id: 'find'
      }))
      .item(iconBarItem('Replace', '🔄', 'left', { 
        clickable: true, 
        action: 'replace',
        hotkey: 'Ctrl+H',
        id: 'replace'
      }))
      .right(this.getToolbarStatus())
      .build()

    // Status bar with file info
    this.statusBar = statusBar('app-status')
      .item(statusIndicator(this.getReadyStatus(), this.getStatusIcon()))
      .item(statusIndicator('TypeScript', '🔷'))
      .center(this.getProblemsStatus())
      .item(iconBarItem(this.getPositionInfo(), '📍', 'right'))
      .item(iconBarItem('UTF-8', '📝', 'right'))
      .item(iconBarItem(`${this.zoom}%`, '🔍', 'right', {
        clickable: true,
        action: 'zoom-menu'
      }))
      .build()

    // Footer bar
    this.footerBar = footerBar('app-footer')
      .left('© 2024 CodeEditor Pro')
      .center('F1=Help F2=Rename F3=Find F4=Replace F5=Debug F12=Go to Definition')
      .right('v2.1.0')
      .build()
  }

  private getDocumentTitle(): string {
    const modified = this.isModified ? ' •' : ''
    return `${this.currentFile}${modified}`
  }

  private getToolbarStatus(): string {
    const status = this.isModified ? 'Modified' : 'Ready'
    return `${status} | TypeScript`
  }

  private getReadyStatus(): string {
    return this.isModified ? 'Modified' : 'Ready'
  }

  private getStatusIcon(): string {
    return this.isModified ? '🟡' : '🟢'
  }

  private getProblemsStatus(): string {
    // Simulate problem detection
    const errors = 0
    const warnings = 2
    
    if (errors > 0) {
      return `${errors} error${errors === 1 ? '' : 's'}, ${warnings} warning${warnings === 1 ? '' : 's'}`
    } else if (warnings > 0) {
      return `${warnings} warning${warnings === 1 ? '' : 's'}`
    } else {
      return 'No problems'
    }
  }

  private getPositionInfo(): string {
    return `Ln ${this.lineNumber}, Col ${this.columnNumber}`
  }

  // File operations
  newFile() {
    this.currentFile = 'untitled.txt'
    this.isModified = false
    this.updateInterface()
    console.log('Created new file')
  }

  openFile(filename: string) {
    this.currentFile = filename
    this.isModified = false
    this.updateInterface()
    console.log(`Opened file: ${filename}`)
  }

  saveFile() {
    if (this.isModified) {
      this.isModified = false
      this.updateInterface()
      console.log(`Saved file: ${this.currentFile}`)
    }
  }

  modifyDocument() {
    this.isModified = true
    this.updateInterface()
  }

  setCursorPosition(line: number, column: number) {
    this.lineNumber = line
    this.columnNumber = column
    this.updateStatusBar()
  }

  setZoom(zoomLevel: number) {
    this.zoom = zoomLevel
    this.updateStatusBar()
  }

  private updateInterface() {
    // Update header with document title
    this.headerBar.updateItem('center', {
      content: this.getDocumentTitle()
    })

    // Update toolbar status
    this.toolbarBar.updateItem('right', {
      content: this.getToolbarStatus()
    })

    this.updateStatusBar()
  }

  private updateStatusBar() {
    // Update status indicator
    this.statusBar.updateItem('ready-status', {
      content: this.getReadyStatus(),
      icon: this.getStatusIcon()
    })

    // Update position info
    this.statusBar.updateItem('position', {
      content: this.getPositionInfo()
    })

    // Update zoom level
    this.statusBar.updateItem('zoom', {
      content: `${this.zoom}%`
    })
  }

  executeAction(action: string) {
    console.log(`Executing action: ${action}`)
    
    switch (action) {
      case 'new-file':
        this.newFile()
        break
      case 'open-file':
        this.openFile('example.ts')
        break
      case 'save-file':
        this.saveFile()
        break
      case 'undo':
        console.log('Undo operation')
        break
      case 'redo':
        console.log('Redo operation')
        break
      case 'find':
        console.log('Open find dialog')
        break
      case 'replace':
        console.log('Open replace dialog')
        break
      default:
        console.log(`Unknown action: ${action}`)
    }
  }

  getInterfaceLayout() {
    return {
      header: this.headerBar.render(100),
      menu: this.menuBar.render(100),
      toolbar: this.toolbarBar.render(100),
      status: this.statusBar.render(100),
      footer: this.footerBar.render(100)
    }
  }

  render(): string {
    const layout = this.getInterfaceLayout()
    
    return `CodeEditor Pro Interface Layout

Header Bar:
${layout.header.left} | ${layout.header.center} | ${layout.header.right}

Menu Bar:
${layout.menu.left} | ${layout.menu.right}

Toolbar:
${layout.toolbar.left} | ${layout.toolbar.right}

Status Bar:
${layout.status.left} | ${layout.status.center} | ${layout.status.right}

Footer Bar:
${layout.footer.left} | ${layout.footer.center} | ${layout.footer.right}

Current State:
• File: ${this.currentFile}
• Modified: ${this.isModified}
• Position: Line ${this.lineNumber}, Column ${this.columnNumber}
• Zoom: ${this.zoom}%`
  }
}

// Usage
const app = new ApplicationInterface()

// Simulate user interactions
app.openFile('main.ts')
app.modifyDocument()
app.setCursorPosition(42, 15)
app.setZoom(125)

// Execute actions
app.executeAction('save-file')
app.executeAction('find')

console.log(app.render())
```

### Dashboard with Multiple Bar Types

```typescript
import { 
  headerBar, 
  navigationBar, 
  statusBar, 
  barBuilder,
  statusIndicator,
  iconBarItem
} from 'reactive-tui-ts'

class DashboardInterface {
  private titleBar: any
  private breadcrumbBar: any
  private actionBar: any
  private metricsBar: any
  private alertsBar: any

  private currentPage: string = 'Dashboard'
  private breadcrumbs: string[] = ['Home', 'Dashboard']
  private notifications: number = 3
  private alerts: number = 1

  constructor() {
    this.setupDashboard()
  }

  private setupDashboard() {
    // Main title bar
    this.titleBar = headerBar('dashboard-title')
      .left('🏢 Enterprise Dashboard')
      .center(this.currentPage)
      .right(`🔔 ${this.notifications} | 👤 Admin`)
      .sticky()
      .build()

    // Breadcrumb navigation
    this.breadcrumbBar = barBuilder('breadcrumb', 'navigation')
      .size('compact')
      .background('#f3f4f6')
      .foreground('#374151')
      .left(this.getBreadcrumbString())
      .right('🕐 ' + new Date().toLocaleTimeString())
      .build()

    // Action bar with quick actions
    this.actionBar = barBuilder('actions', 'toolbar')
      .item(iconBarItem('Refresh', '🔄', 'left', { 
        clickable: true, 
        action: 'refresh',
        id: 'refresh-btn'
      }))
      .item(iconBarItem('Export', '📊', 'left', { 
        clickable: true, 
        action: 'export',
        id: 'export-btn'
      }))
      .item(iconBarItem('Settings', '⚙️', 'left', { 
        clickable: true, 
        action: 'settings',
        id: 'settings-btn'
      }))
      .center('Quick Actions: Refresh Data | Export Reports | Configure')
      .item(iconBarItem('Filter', '🔍', 'right', { 
        clickable: true, 
        action: 'filter',
        id: 'filter-btn'
      }))
      .build()

    // System metrics bar
    this.metricsBar = barBuilder('metrics', 'status')
      .background('#22c55e')
      .foreground('#ffffff')
      .item(statusIndicator('CPU: 45%', '⚡'))
      .item(statusIndicator('Memory: 67%', '🧠'))
      .item(statusIndicator('Disk: 89%', '💾'))
      .item(statusIndicator('Network: OK', '🌐'))
      .center('System Status: All Services Running')
      .item(iconBarItem('Uptime: 15d 4h', '🕒', 'right'))
      .build()

    // Alerts and notifications bar
    this.alertsBar = barBuilder('alerts', 'custom')
      .background(this.alerts > 0 ? '#ef4444' : '#22c55e')
      .foreground('#ffffff')
      .left(this.getAlertsString())
      .center(this.getAlertsMessage())
      .right('Last Check: ' + new Date().toLocaleTimeString())
      .build()
  }

  private getBreadcrumbString(): string {
    return this.breadcrumbs.join(' > ')
  }

  private getAlertsString(): string {
    if (this.alerts > 0) {
      return `⚠️ ${this.alerts} Alert${this.alerts === 1 ? '' : 's'}`
    }
    return '✅ No Alerts'
  }

  private getAlertsMessage(): string {
    if (this.alerts > 0) {
      return 'High disk usage detected on server-01 | Click for details'
    }
    return 'All systems operating normally'
  }

  navigateTo(page: string, breadcrumbs: string[]) {
    this.currentPage = page
    this.breadcrumbs = breadcrumbs
    
    // Update title bar
    this.titleBar.updateItem('center', { content: this.currentPage })
    
    // Update breadcrumb bar
    this.breadcrumbBar.updateItem('left', { 
      content: this.getBreadcrumbString() 
    })
    
    console.log(`Navigated to: ${page}`)
  }

  updateNotifications(count: number) {
    this.notifications = count
    this.titleBar.updateItem('right', {
      content: `🔔 ${this.notifications} | 👤 Admin`
    })
  }

  updateAlerts(count: number) {
    this.alerts = count
    
    // Update background color based on alert status
    const backgroundColor = this.alerts > 0 ? '#ef4444' : '#22c55e'
    this.alertsBar.build().attributes['data-style'] = JSON.stringify({
      background: backgroundColor,
      foreground: '#ffffff'
    })
    
    // Update content
    this.alertsBar.updateItem('left', {
      content: this.getAlertsString()
    })
    this.alertsBar.updateItem('center', {
      content: this.getAlertsMessage()
    })
  }

  updateMetrics(metrics: {
    cpu?: number
    memory?: number
    disk?: number
    network?: string
  }) {
    if (metrics.cpu !== undefined) {
      this.metricsBar.updateItem('cpu-metric', {
        content: `CPU: ${metrics.cpu}%`,
        icon: '⚡'
      })
    }
    
    if (metrics.memory !== undefined) {
      this.metricsBar.updateItem('memory-metric', {
        content: `Memory: ${metrics.memory}%`,
        icon: '🧠'
      })
    }
    
    if (metrics.disk !== undefined) {
      this.metricsBar.updateItem('disk-metric', {
        content: `Disk: ${metrics.disk}%`,
        icon: '💾'
      })
    }
    
    if (metrics.network) {
      this.metricsBar.updateItem('network-metric', {
        content: `Network: ${metrics.network}`,
        icon: '🌐'
      })
    }
  }

  executeAction(action: string) {
    console.log(`Dashboard action: ${action}`)
    
    switch (action) {
      case 'refresh':
        this.refreshData()
        break
      case 'export':
        this.exportReports()
        break
      case 'settings':
        this.openSettings()
        break
      case 'filter':
        this.openFilters()
        break
      default:
        console.log(`Unknown action: ${action}`)
    }
  }

  private refreshData() {
    console.log('Refreshing dashboard data...')
    // Simulate data refresh
    setTimeout(() => {
      this.updateMetrics({
        cpu: 30 + Math.floor(Math.random() * 40),
        memory: 50 + Math.floor(Math.random() * 30),
        disk: 80 + Math.floor(Math.random() * 15),
        network: 'OK'
      })
      console.log('Data refreshed')
    }, 1000)
  }

  private exportReports() {
    console.log('Exporting dashboard reports...')
  }

  private openSettings() {
    this.navigateTo('Settings', ['Home', 'Dashboard', 'Settings'])
  }

  private openFilters() {
    console.log('Opening filter panel...')
  }

  render(): string {
    return `Enterprise Dashboard Interface

Title Bar: ${this.titleBar.render(100).left} | ${this.titleBar.render(100).center} | ${this.titleBar.render(100).right}

Breadcrumb: ${this.breadcrumbBar.render(100).left} | ${this.breadcrumbBar.render(100).right}

Actions: ${this.actionBar.render(100).left} | ${this.actionBar.render(100).center} | ${this.actionBar.render(100).right}

Metrics: ${this.metricsBar.render(100).left} | ${this.metricsBar.render(100).center} | ${this.metricsBar.render(100).right}

Alerts: ${this.alertsBar.render(100).left} | ${this.alertsBar.render(100).center} | ${this.alertsBar.render(100).right}

Current State:
• Page: ${this.currentPage}
• Breadcrumbs: ${this.breadcrumbs.join(' > ')}
• Notifications: ${this.notifications}
• Alerts: ${this.alerts}
• Time: ${new Date().toLocaleTimeString()}`
  }
}

// Usage
const dashboard = new DashboardInterface()

// Simulate navigation
dashboard.navigateTo('Analytics', ['Home', 'Dashboard', 'Analytics'])
dashboard.navigateTo('Reports', ['Home', 'Dashboard', 'Reports'])

// Update counters
dashboard.updateNotifications(5)
dashboard.updateAlerts(2)

// Update system metrics
dashboard.updateMetrics({
  cpu: 67,
  memory: 82,
  disk: 94,
  network: 'High Traffic'
})

// Execute actions
dashboard.executeAction('refresh')
dashboard.executeAction('export')

console.log(dashboard.render())
```

## CSS Styling

```css
/* Bar base styles */
.bar {
  display: flex;
  align-items: center;
  width: 100%;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  box-sizing: border-box;
  position: relative;
}

/* Bar types */
.bar-header {
  background: #3b82f6;
  color: #ffffff;
  border-bottom: 1px solid #2563eb;
}

.bar-footer {
  background: #6b7280;
  color: #ffffff;
  border-top: 1px solid #4b5563;
}

.bar-navigation {
  background: #1f2937;
  color: #f9fafb;
  border-bottom: 1px solid #374151;
}

.bar-status {
  background: #22c55e;
  color: #ffffff;
  border-top: 1px solid #16a34a;
}

.bar-toolbar {
  background: #f3f4f6;
  color: #374151;
  border: 1px solid #d1d5db;
}

.bar-tabbar {
  background: #f8fafc;
  color: #1e293b;
  border-bottom: 1px solid #e2e8f0;
}

.bar-custom {
  background: #ffffff;
  color: #000000;
  border: 1px solid #e5e7eb;
}

/* Bar sizes */
.bar-compact {
  height: 2rem;
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
}

.bar-normal {
  height: 2.5rem;
  padding: 0.5rem 1rem;
  font-size: 1rem;
}

.bar-large {
  height: 3rem;
  padding: 0.75rem 1.25rem;
  font-size: 1.125rem;
}

/* Bar behavior */
.bar-sticky {
  position: sticky;
  top: 0;
  z-index: 100;
}

.bar-hidden {
  display: none;
}

/* Bar content positioning */
.bar-content {
  display: flex;
  justify-content: space-between;
  align-items: center;
  width: 100%;
}

.bar-left {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.bar-center {
  display: flex;
  align-items: center;
  justify-content: center;
  flex: 1;
  margin: 0 1rem;
}

.bar-right {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-left: auto;
}

.bar-justify {
  display: flex;
  justify-content: space-between;
  width: 100%;
}

/* Bar items */
.bar-item {
  display: flex;
  align-items: center;
  gap: 0.25rem;
  padding: 0.25rem 0.5rem;
  border-radius: 3px;
  transition: all 0.2s ease;
}

.bar-item.clickable {
  cursor: pointer;
}

.bar-item.clickable:hover {
  background: rgba(255, 255, 255, 0.1);
  transform: translateY(-1px);
}

.bar-item.clickable:active {
  transform: translateY(0);
}

/* Bar item icons */
.bar-item-icon {
  font-size: 1rem;
  margin-right: 0.25rem;
}

/* Border styles */
.bar[data-border-style="single"] {
  border-width: 1px;
  border-style: solid;
}

.bar[data-border-style="double"] {
  border-width: 2px;
  border-style: double;
}

.bar[data-border-style="thick"] {
  border-width: 3px;
  border-style: solid;
}

/* Status indicators */
.status-red {
  color: #ef4444;
  background: rgba(239, 68, 68, 0.1);
}

.status-green {
  color: #22c55e;
  background: rgba(34, 197, 94, 0.1);
}

.status-yellow {
  color: #f59e0b;
  background: rgba(245, 158, 11, 0.1);
}

.status-blue {
  color: #3b82f6;
  background: rgba(59, 130, 246, 0.1);
}

/* Responsive design */
@media (max-width: 768px) {
  .bar {
    padding: 0.375rem 0.75rem;
    font-size: 0.9rem;
  }
  
  .bar-center {
    margin: 0 0.5rem;
  }
  
  .bar-item {
    padding: 0.125rem 0.375rem;
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .bar-custom {
    background: #1f2937;
    color: #f9fafb;
    border-color: #374151;
  }
  
  .bar-toolbar {
    background: #374151;
    color: #e5e7eb;
    border-color: #4b5563;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .bar {
    border-width: 2px;
  }
  
  .bar-item.clickable:hover {
    background: rgba(255, 255, 255, 0.3);
    border: 1px solid currentColor;
  }
}

/* Animation support */
@keyframes slideDown {
  from {
    transform: translateY(-100%);
    opacity: 0;
  }
  to {
    transform: translateY(0);
    opacity: 1;
  }
}

.bar-header {
  animation: slideDown 0.3s ease-out;
}

/* Print styles */
@media print {
  .bar-sticky {
    position: static;
  }
  
  .bar {
    border: 1px solid #000000;
    background: #ffffff !important;
    color: #000000 !important;
  }
}
```

## Best Practices

### 1. Use Appropriate Bar Types

```typescript
// ✅ Good - header for main application title
const header = headerBar('app-header')
  .left('📱 Mobile App')
  .build()

// ✅ Good - status bar for system information
const status = statusBar('system-status')
  .left('Ready')
  .right('CPU: 45%')
  .build()

// ✅ Good - toolbar for actions
const toolbar = toolbar('actions')
  .left('New')
  .left('Save')
  .build()
```

### 2. Implement Proper Content Positioning

```typescript
// ✅ Good - logical content placement
const wellPositionedBar = bar({
  id: 'positioned',
  items: [
    { content: 'App Name', position: 'left' },      // Brand/identity
    { content: 'Current Page', position: 'center' }, // Context
    { content: 'User Menu', position: 'right' }      // User actions
  ]
})
```

### 3. Use Icons and Actions Effectively

```typescript
// ✅ Good - meaningful icons with actions
const actionBar = toolbar('actions')
  .item(iconBarItem('New', '📄', 'left', { 
    clickable: true, 
    action: 'new-file',
    hotkey: 'Ctrl+N'
  }))
  .item(iconBarItem('Save', '💾', 'left', { 
    clickable: true, 
    action: 'save-file',
    hotkey: 'Ctrl+S'
  }))
  .build()
```

### 4. Implement Responsive Behavior

```typescript
// ✅ Good - responsive content that adapts to width
const responsiveBar = bar({
  id: 'responsive',
  items: [
    { content: 'Essential', position: 'left' },
    { content: 'Optional Info', position: 'center' },
    { content: 'Critical', position: 'right' }
  ]
})

// Handle narrow screens by hiding non-essential content
if (screenWidth < 60) {
  responsiveBar.removeItem('optional-info')
}
```

### 5. Maintain Consistent Styling

```typescript
// ✅ Good - consistent styling across application
const consistentHeader = headerBar('header')
  .background('#1e40af')
  .foreground('#ffffff')
  .build()

const consistentFooter = footerBar('footer')
  .background('#1e40af')
  .foreground('#ffffff')
  .build()
```

## Related Widgets

- **[Grid](./grid)** - Layout system for organizing bar content
- **[Panel](./panel)** - Container panels that bars can control
- **[Tabs](./tabs)** - Tab navigation that bars can contain
- **[Menu](./menu)** - Dropdown menus triggered from bars

## Examples

- **[Basic Bars](../../examples/basic/bar-basic)** - Simple bar implementations
- **[Application Interface](../../examples/advanced/app-bars)** - Complete application bar system
- **[Dashboard Bars](../../examples/apps/dashboard-bars)** - Multi-bar dashboard interface
- **[Responsive Bars](../../examples/responsive/mobile-bars)** - Mobile-friendly bar layouts

The Bar widget provides comprehensive header, footer, navigation, and toolbar functionality with flexible positioning, interactive elements, and advanced styling options for building complete application interfaces.