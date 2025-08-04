# Grid Widget

Advanced CSS Grid layout system with flexible column/row configurations, gap controls, item positioning, and responsive design capabilities.

## Overview

The Grid widget provides a complete CSS Grid implementation with support for complex layouts, spanning items, precise positioning, flow control, and comprehensive styling options for building sophisticated terminal interfaces.

```typescript
import { grid, GridColumns, GridFlow } from 'reactive-tui-ts'

const dashboardGrid = grid({
  id: 'dashboard-layout',
  columns: GridColumns.Three,
  rows: GridColumns.Two,
  gap: 2,
  items: [
    { content: 'Header', colSpan: 3, backgroundColor: '#1e3a8a' },
    { content: 'Sidebar', rowSpan: 2, backgroundColor: '#166534' },
    { content: 'Main Content', colSpan: 2, backgroundColor: '#374151' },
    { content: 'Footer', colSpan: 3, backgroundColor: '#991b1b' }
  ]
})
```

## Types

### GridColumns

```typescript
enum GridColumns {
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
```

### GridFlow

```typescript
enum GridFlow {
  Row = 'row',
  Column = 'column',
  Dense = 'dense'
}
```

### GridAlign

```typescript
enum GridAlign {
  Start = 'start',
  End = 'end',
  Center = 'center',
  Stretch = 'stretch'
}
```

### GridItemConfig

```typescript
interface GridItemConfig {
  id?: string
  content: string
  column?: number
  row?: number
  colSpan?: number
  rowSpan?: number
  backgroundColor?: string
  borderColor?: string
  textColor?: string
  classes?: string[]
}
```

### GridConfig

```typescript
interface GridConfig {
  id?: string
  columns: GridColumns | string
  rows?: GridColumns | string
  gap?: number
  columnGap?: number
  rowGap?: number
  items: GridItemConfig[]
  flow?: GridFlow
  alignItems?: GridAlign
  justifyItems?: GridAlign
  classes?: string[]
}
```

## Basic Usage

### Simple Grid Layout

```typescript
import { grid, GridColumns } from 'reactive-tui-ts'

const basicGrid = grid({
  id: 'basic-grid',
  columns: GridColumns.Three,
  gap: 1,
  items: [
    { content: 'Item 1' },
    { content: 'Item 2' },
    { content: 'Item 3' },
    { content: 'Item 4' },
    { content: 'Item 5' },
    { content: 'Item 6' }
  ]
})

// Creates a 3-column grid with 6 items arranged in 2 rows
```

### Grid with Rows and Columns

```typescript
const structuredGrid = grid({
  id: 'structured-grid',
  columns: GridColumns.Four,
  rows: GridColumns.Three,
  gap: 2,
  items: [
    { content: 'A1' },
    { content: 'A2' },
    { content: 'A3' },
    { content: 'A4' },
    { content: 'B1' },
    { content: 'B2' },
    { content: 'B3' },
    { content: 'B4' },
    { content: 'C1' },
    { content: 'C2' },
    { content: 'C3' },
    { content: 'C4' }
  ]
})

// Creates a 4x3 grid with explicit row definition
```

### Auto-Sizing Grid

```typescript
import { autoGrid } from 'reactive-tui-ts'

const autoSizeGrid = autoGrid(GridColumns.Two, [
  { content: 'Dynamic Item 1' },
  { content: 'Dynamic Item 2' },
  { content: 'Dynamic Item 3 with longer content' },
  { content: 'Dynamic Item 4' }
])

// Creates a 2-column grid that automatically sizes based on content
```

## Advanced Grid Features

### Item Spanning

```typescript
const spanningGrid = grid({
  id: 'spanning-grid',
  columns: GridColumns.Four,
  rows: GridColumns.Three,
  gap: 1,
  items: [
    // Header spanning all columns
    {
      content: 'Header Section',
      colSpan: 4,
      backgroundColor: '#1e3a8a',
      textColor: '#ffffff'
    },
    // Sidebar spanning 2 rows
    {
      content: 'Sidebar\nNavigation\nMenu',
      rowSpan: 2,
      backgroundColor: '#166534',
      textColor: '#ffffff'
    },
    // Main content spanning 2 columns
    {
      content: 'Main Content Area',
      colSpan: 2,
      backgroundColor: '#374151',
      textColor: '#ffffff'
    },
    // Small widget
    {
      content: 'Widget',
      backgroundColor: '#7c2d92',
      textColor: '#ffffff'
    },
    // Footer spanning remaining columns
    {
      content: 'Footer',
      colSpan: 3,
      backgroundColor: '#991b1b',
      textColor: '#ffffff'
    }
  ]
})
```

### Precise Item Positioning

```typescript
const positionedGrid = grid({
  id: 'positioned-grid',
  columns: GridColumns.Four,
  rows: GridColumns.Four,
  gap: 1,
  items: [
    // Place item at specific grid position
    {
      content: 'Top Left',
      column: 1,
      row: 1,
      backgroundColor: '#1e3a8a'
    },
    // Place item at different position
    {
      content: 'Center',
      column: 2,
      row: 2,
      colSpan: 2,
      rowSpan: 2,
      backgroundColor: '#166534'
    },
    // Bottom right corner
    {
      content: 'Bottom Right',
      column: 4,
      row: 4,
      backgroundColor: '#991b1b'
    }
  ]
})
```

### Grid Flow Control

```typescript
import { GridFlow } from 'reactive-tui-ts'

// Row-first flow (default)
const rowFlowGrid = grid({
  id: 'row-flow',
  columns: GridColumns.Three,
  flow: GridFlow.Row,
  items: [
    { content: '1' }, { content: '2' }, { content: '3' },
    { content: '4' }, { content: '5' }, { content: '6' }
  ]
})
// Layout: 1 2 3
//         4 5 6

// Column-first flow
const columnFlowGrid = grid({
  id: 'column-flow',
  columns: GridColumns.Three,
  flow: GridFlow.Column,
  items: [
    { content: '1' }, { content: '2' }, { content: '3' },
    { content: '4' }, { content: '5' }, { content: '6' }
  ]
})
// Layout: 1 3 5
//         2 4 6

// Dense packing
const denseFlowGrid = grid({
  id: 'dense-flow',
  columns: GridColumns.Three,
  flow: GridFlow.Dense,
  items: [
    { content: 'A', colSpan: 2 },
    { content: 'B' },
    { content: 'C' },
    { content: 'D' }
  ]
})
// Dense flow fills gaps automatically
```

### Gap and Spacing

```typescript
// Uniform gap
const uniformGapGrid = grid({
  id: 'uniform-gap',
  columns: GridColumns.Three,
  gap: 3,
  items: [
    { content: 'Item 1' },
    { content: 'Item 2' },
    { content: 'Item 3' }
  ]
})

// Different column and row gaps
const customGapGrid = grid({
  id: 'custom-gap',
  columns: GridColumns.Three,
  columnGap: 2,
  rowGap: 4,
  items: [
    { content: 'A' }, { content: 'B' }, { content: 'C' },
    { content: 'D' }, { content: 'E' }, { content: 'F' }
  ]
})
```

## Builder Pattern

### GridBuilder Usage

```typescript
import { GridBuilder, GridColumns, GridFlow, GridAlign } from 'reactive-tui-ts'

const complexGrid = GridBuilder.create()
  .id('complex-layout')
  .columns(GridColumns.Six)
  .rows(GridColumns.Four)
  .gap(2)
  .flow(GridFlow.Row)
  .alignItems(GridAlign.Center)
  .justifyItems(GridAlign.Stretch)
  .addItem({
    content: 'Header',
    colSpan: 6,
    backgroundColor: '#1e3a8a',
    textColor: '#ffffff'
  })
  .addItem({
    content: 'Navigation',
    rowSpan: 2,
    backgroundColor: '#166534',
    textColor: '#ffffff'
  })
  .addItem({
    content: 'Main Content',
    colSpan: 4,
    backgroundColor: '#374151',
    textColor: '#ffffff'
  })
  .addItem({
    content: 'Sidebar',
    rowSpan: 2,
    backgroundColor: '#7c2d92',
    textColor: '#ffffff'
  })
  .addItem({
    content: 'Footer',
    colSpan: 6,
    backgroundColor: '#991b1b',
    textColor: '#ffffff'
  })
  .classes(['dashboard-grid', 'main-layout'])
  .build()
```

### Fluent Grid Creation

```typescript
import { createGrid } from 'reactive-tui-ts'

const fluentGrid = createGrid()
  .columns(GridColumns.Four)
  .gap(1)
  .addItem({ content: 'Item 1', backgroundColor: '#3b82f6' })
  .addItem({ content: 'Item 2', backgroundColor: '#10b981' })
  .addItem({ content: 'Item 3', backgroundColor: '#f59e0b' })
  .addItem({ content: 'Item 4', backgroundColor: '#ef4444' })
  .build()
```

## Pre-built Grid Patterns

### Simple Grid Function

```typescript
import { simpleGrid } from 'reactive-tui-ts'

const quickGrid = simpleGrid(
  GridColumns.Three,
  GridColumns.Two,
  [
    { content: 'A' },
    { content: 'B' },
    { content: 'C' },
    { content: 'D' },
    { content: 'E' },
    { content: 'F' }
  ]
)
```

### Colored Grid Template

```typescript
import { createColoredGrid, GRID_COLORS } from 'reactive-tui-ts'

// Pre-built colorful grid
const colorfulGrid = createColoredGrid(GridColumns.Three)

// Custom colored items
const customColorGrid = grid({
  id: 'custom-colors',
  columns: GridColumns.Two,
  gap: 1,
  items: [
    { content: 'Blue Panel', ...GRID_COLORS.blue },
    { content: 'Green Panel', ...GRID_COLORS.green },
    { content: 'Red Panel', ...GRID_COLORS.red },
    { content: 'Yellow Panel', ...GRID_COLORS.yellow }
  ]
})
```

### Color Palette

```typescript
// Available pre-defined colors
const GRID_COLORS = {
  blue: { backgroundColor: '#1e3a8a', borderColor: '#3b82f6', textColor: '#dbeafe' },
  green: { backgroundColor: '#166534', borderColor: '#10b981', textColor: '#d1fae5' },
  red: { backgroundColor: '#991b1b', borderColor: '#ef4444', textColor: '#fecaca' },
  yellow: { backgroundColor: '#a16207', borderColor: '#f59e0b', textColor: '#fef3c7' },
  purple: { backgroundColor: '#7c2d92', borderColor: '#a855f7', textColor: '#e9d5ff' },
  gray: { backgroundColor: '#374151', borderColor: '#6b7280', textColor: '#f3f4f6' },
  cyan: { backgroundColor: '#155e75', borderColor: '#06b6d4', textColor: '#cffafe' },
  pink: { backgroundColor: '#be185d', borderColor: '#ec4899', textColor: '#fce7f3' }
}
```

## Real-World Examples

### Application Dashboard Layout

```typescript
import { GridBuilder, GridColumns, GridFlow, GRID_COLORS } from 'reactive-tui-ts'

class ApplicationDashboard {
  private mainGrid: any
  private metricsData: Record<string, any> = {}

  constructor() {
    this.setupDashboardGrid()
    this.initializeMetrics()
  }

  private setupDashboardGrid() {
    this.mainGrid = GridBuilder.create()
      .id('app-dashboard')
      .columns(GridColumns.Twelve)
      .rows(GridColumns.Eight)
      .gap(1)
      .flow(GridFlow.Row)
      
      // Header spanning full width
      .addItem({
        id: 'header',
        content: this.getHeaderContent(),
        colSpan: 12,
        ...GRID_COLORS.blue,
        classes: ['header-panel']
      })
      
      // Navigation sidebar
      .addItem({
        id: 'navigation',
        content: this.getNavigationContent(),
        rowSpan: 6,
        colSpan: 2,
        ...GRID_COLORS.gray,
        classes: ['nav-panel']
      })
      
      // Main metrics grid (3x2)
      .addItem({
        id: 'cpu-metric',
        content: this.getCPUMetricContent(),
        ...GRID_COLORS.green,
        classes: ['metric-panel']
      })
      .addItem({
        id: 'memory-metric',
        content: this.getMemoryMetricContent(),
        ...GRID_COLORS.yellow,
        classes: ['metric-panel']
      })
      .addItem({
        id: 'disk-metric',
        content: this.getDiskMetricContent(),
        ...GRID_COLORS.cyan,
        classes: ['metric-panel']
      })
      .addItem({
        id: 'network-metric',
        content: this.getNetworkMetricContent(),
        ...GRID_COLORS.purple,
        classes: ['metric-panel']
      })
      
      // Large chart area
      .addItem({
        id: 'main-chart',
        content: this.getChartContent(),
        colSpan: 4,
        rowSpan: 2,
        ...GRID_COLORS.gray,
        classes: ['chart-panel']
      })
      
      // Activity feed
      .addItem({
        id: 'activity-feed',
        content: this.getActivityContent(),
        colSpan: 2,
        rowSpan: 4,
        backgroundColor: '#f8fafc',
        borderColor: '#e2e8f0',
        textColor: '#1f2937',
        classes: ['activity-panel']
      })
      
      // System logs
      .addItem({
        id: 'system-logs',
        content: this.getLogsContent(),
        colSpan: 8,
        rowSpan: 2,
        backgroundColor: '#1f2937',
        borderColor: '#374151',
        textColor: '#f9fafb',
        classes: ['logs-panel']
      })
      
      // Status bar
      .addItem({
        id: 'status-bar',
        content: this.getStatusContent(),
        colSpan: 12,
        ...GRID_COLORS.blue,
        classes: ['status-panel']
      })
      
      .classes(['dashboard-layout', 'main-grid'])
      .build()
  }

  private initializeMetrics() {
    this.metricsData = {
      cpu: { usage: 45, temperature: 67, cores: 8 },
      memory: { used: 8.2, total: 16, cached: 2.1 },
      disk: { used: 425, total: 500, io: { read: 45, write: 23 } },
      network: { in: 12.5, out: 3.2, connections: 247 }
    }
  }

  private getHeaderContent(): string {
    const now = new Date().toLocaleString()
    return `🖥️  System Dashboard - ${now} | Status: Online | Uptime: 247h 35m`
  }

  private getNavigationContent(): string {
    return `📊 Dashboard
🔧 Settings
📈 Analytics  
🔍 Monitoring
⚠️  Alerts
📋 Reports
👤 Profile
🚪 Logout`
  }

  private getCPUMetricContent(): string {
    const { usage, temperature, cores } = this.metricsData.cpu
    return `⚡ CPU Usage
${usage}%
Temp: ${temperature}°C
Cores: ${cores}`
  }

  private getMemoryMetricContent(): string {
    const { used, total, cached } = this.metricsData.memory
    const percent = Math.round((used / total) * 100)
    return `🧠 Memory
${used}GB / ${total}GB
${percent}%
Cache: ${cached}GB`
  }

  private getDiskMetricContent(): string {
    const { used, total, io } = this.metricsData.disk
    const percent = Math.round((used / total) * 100)
    return `💾 Disk Usage
${used}GB / ${total}GB
${percent}%
I/O: ${io.read}/${io.write} MB/s`
  }

  private getNetworkMetricContent(): string {
    const { in: inTraffic, out, connections } = this.metricsData.network
    return `🌐 Network
↓ ${inTraffic} Mbps
↑ ${out} Mbps
Connections: ${connections}`
  }

  private getChartContent(): string {
    return `📈 Performance Chart

CPU:  ████████████░░░░░░░░░░  45%
MEM:  ████████████████░░░░░░  67%
DISK: ██████████████████████  89%
NET:  ████████░░░░░░░░░░░░░░  32%

Trends:
• CPU usage stable over 24h
• Memory trending up +5%
• Disk I/O peak at 14:30
• Network traffic normal`
  }

  private getActivityContent(): string {
    return `🔔 Recent Activity

2 min ago
User login: admin

5 min ago  
Service restart: nginx

15 min ago
Backup completed

1 hour ago
Update installed

2 hours ago
High CPU alert cleared

4 hours ago
Scheduled maintenance`
  }

  private getLogsContent(): string {
    return `📄 System Logs

[15:42:33] INFO: Service nginx restarted successfully
[15:41:28] WARN: High memory usage detected (87%)
[15:40:15] INFO: Backup process started
[15:38:42] ERROR: Connection timeout to external API
[15:37:19] INFO: User session expired: user_123
[15:35:06] DEBUG: Cache invalidated for key: metrics_cpu
[15:34:44] INFO: Scheduled task completed: disk_cleanup`
  }

  private getStatusContent(): string {
    const uptime = '247h 35m'
    const version = 'v2.1.0'
    return `Status: ✅ All Systems Operational | Uptime: ${uptime} | Version: ${version} | Load: 1.2, 1.5, 1.8`
  }

  updateMetrics(newMetrics: Partial<typeof this.metricsData>) {
    this.metricsData = { ...this.metricsData, ...newMetrics }
    
    // Update specific grid items
    if (newMetrics.cpu) {
      this.updateGridItem('cpu-metric', this.getCPUMetricContent())
    }
    if (newMetrics.memory) {
      this.updateGridItem('memory-metric', this.getMemoryMetricContent())
    }
    if (newMetrics.disk) {
      this.updateGridItem('disk-metric', this.getDiskMetricContent())
    }
    if (newMetrics.network) {
      this.updateGridItem('network-metric', this.getNetworkMetricContent())
    }
  }

  private updateGridItem(itemId: string, newContent: string) {
    // In real implementation, would update the grid item
    console.log(`Updated ${itemId}:`, newContent)
  }

  addAlert(message: string, severity: 'info' | 'warning' | 'error' = 'info') {
    const timestamp = new Date().toLocaleTimeString()
    const icons = { info: 'ℹ️', warning: '⚠️', error: '❌' }
    const alertLine = `${timestamp} ${icons[severity]} ${message}`
    
    console.log('New alert:', alertLine)
    // Would prepend to activity feed content
  }

  exportLayout(): any {
    return {
      id: 'app-dashboard',
      columns: 12,
      rows: 8,
      items: [
        { id: 'header', position: { col: 1, row: 1, colSpan: 12 } },
        { id: 'navigation', position: { col: 1, row: 2, colSpan: 2, rowSpan: 6 } },
        { id: 'cpu-metric', position: { col: 3, row: 2 } },
        { id: 'memory-metric', position: { col: 4, row: 2 } },
        { id: 'disk-metric', position: { col: 5, row: 2 } },
        { id: 'network-metric', position: { col: 6, row: 2 } },
        { id: 'main-chart', position: { col: 3, row: 3, colSpan: 4, rowSpan: 2 } },
        { id: 'activity-feed', position: { col: 7, row: 2, colSpan: 2, rowSpan: 4 } },
        { id: 'system-logs', position: { col: 3, row: 5, colSpan: 8, rowSpan: 2 } },
        { id: 'status-bar', position: { col: 1, row: 8, colSpan: 12 } }
      ]
    }
  }

  render(): string {
    return `Application Dashboard (12x8 Grid)
    
Layout Structure:
┌────────────────────────────────────────────────────────────┐
│ Header (12 cols)                                           │
├──┬─────┬─────┬─────┬─────┬─────────────────┬──────────────┤
│N │ CPU │ MEM │DISK │ NET │                 │              │
│a ├─────┴─────┴─────┴─────┤   Activity      │              │
│v │                       │   Feed          │              │
│i │     Main Chart        │   (2x4)         │              │  
│g │     (4x2)             │                 │              │
│a ├───────────────────────┴─────────────────┤              │
│t │              System Logs                │              │
│i │              (8x2)                      │              │
│o ├─────────────────────────────────────────┴──────────────┤
│n │ Status Bar (12 cols)                                   │
└──┴────────────────────────────────────────────────────────┘

Grid uses:
• 12-column layout for flexibility
• Color-coded panels for visual hierarchy
• Responsive spanning for different content types
• Mixed content types (metrics, charts, logs, navigation)`
  }
}

// Usage
const dashboard = new ApplicationDashboard()

// Simulate metric updates
setInterval(() => {
  dashboard.updateMetrics({
    cpu: { usage: 40 + Math.floor(Math.random() * 20), temperature: 65 + Math.floor(Math.random() * 10), cores: 8 },
    memory: { used: 7 + Math.random() * 3, total: 16, cached: 2 + Math.random() }
  })
}, 5000)

// Add some alerts
dashboard.addAlert('System backup completed successfully', 'info')
dashboard.addAlert('High memory usage detected', 'warning')

console.log(dashboard.render())
```

### Responsive Card Layout

```typescript
import { GridBuilder, GridColumns, GridAlign, GRID_COLORS } from 'reactive-tui-ts'

class ResponsiveCardLayout {
  private cardGrid: any
  private cards: Array<{ id: string; title: string; content: string; category: string }> = []

  constructor() {
    this.initializeCards()
    this.setupCardGrid()
  }

  private initializeCards() {
    this.cards = [
      {
        id: 'welcome',
        title: 'Welcome',
        content: 'Welcome to the application!\nGet started with our tutorials.',
        category: 'info'
      },
      {
        id: 'stats',
        title: 'Statistics',
        content: 'Users: 1,247\nActive: 89\nGrowth: +12%',
        category: 'metrics'
      },
      {
        id: 'notifications',
        title: 'Notifications',
        content: '3 new messages\n1 system alert\n5 updates available',
        category: 'alerts'
      },
      {
        id: 'quick-actions',
        title: 'Quick Actions',
        content: '• Create New Project\n• Import Data\n• Export Report\n• Backup System',
        category: 'actions'
      },
      {
        id: 'recent-activity',
        title: 'Recent Activity',
        content: 'Project Alpha updated\nUser John logged in\nBackup completed',
        category: 'activity'
      },
      {
        id: 'system-health',
        title: 'System Health',
        content: 'CPU: 34% ✓\nMemory: 67% ⚠️\nDisk: 89% ❌\nNetwork: OK ✓',
        category: 'health'
      },
      {
        id: 'weather',
        title: 'Weather',
        content: '🌤️ Partly Cloudy\n22°C / 72°F\nHumidity: 65%\nWind: 12 km/h',
        category: 'external'
      },
      {
        id: 'calendar',
        title: 'Upcoming Events',
        content: 'Today:\n• Team Meeting (2:00 PM)\n• Code Review (4:30 PM)\n\nTomorrow:\n• Client Call (9:00 AM)',
        category: 'schedule'
      }
    ]
  }

  private setupCardGrid() {
    const builder = GridBuilder.create()
      .id('responsive-cards')
      .columns(GridColumns.Four)
      .gap(2)
      .alignItems(GridAlign.Stretch)
      .justifyItems(GridAlign.Stretch)

    // Add cards with category-based colors
    this.cards.forEach(card => {
      const colors = this.getCategoryColors(card.category)
      
      builder.addItem({
        id: card.id,
        content: this.formatCardContent(card),
        ...colors,
        classes: ['card', `card-${card.category}`]
      })
    })

    this.cardGrid = builder.build()
  }

  private getCategoryColors(category: string) {
    const categoryColorMap: Record<string, keyof typeof GRID_COLORS> = {
      info: 'blue',
      metrics: 'green',
      alerts: 'yellow',
      actions: 'purple',
      activity: 'cyan',
      health: 'red',
      external: 'gray',
      schedule: 'pink'
    }

    return GRID_COLORS[categoryColorMap[category] || 'gray']
  }

  private formatCardContent(card: { title: string; content: string }): string {
    return `${card.title}
────────────
${card.content}`
  }

  addCard(card: { id: string; title: string; content: string; category: string }) {
    this.cards.push(card)
    this.refreshGrid()
  }

  removeCard(cardId: string) {
    this.cards = this.cards.filter(card => card.id !== cardId)
    this.refreshGrid()
  }

  updateCard(cardId: string, updates: Partial<{ title: string; content: string; category: string }>) {
    const cardIndex = this.cards.findIndex(card => card.id === cardId)
    if (cardIndex !== -1) {
      this.cards[cardIndex] = { ...this.cards[cardIndex], ...updates }
      this.refreshGrid()
    }
  }

  private refreshGrid() {
    this.setupCardGrid()
  }

  setLayout(columns: GridColumns) {
    const builder = GridBuilder.create()
      .id('responsive-cards')
      .columns(columns)
      .gap(2)
      .alignItems(GridAlign.Stretch)

    this.cards.forEach(card => {
      const colors = this.getCategoryColors(card.category)
      builder.addItem({
        id: card.id,
        content: this.formatCardContent(card),
        ...colors,
        classes: ['card', `card-${card.category}`]
      })
    })

    this.cardGrid = builder.build()
  }

  // Responsive layout methods
  setMobileLayout() {
    this.setLayout(GridColumns.One)
  }

  setTabletLayout() {
    this.setLayout(GridColumns.Two)
  }

  setDesktopLayout() {
    this.setLayout(GridColumns.Four)
  }

  setWideLayout() {
    this.setLayout(GridColumns.Six)
  }

  // Filter cards by category
  filterByCategory(category: string) {
    const filteredCards = this.cards.filter(card => card.category === category)
    
    const builder = GridBuilder.create()
      .id('filtered-cards')
      .columns(GridColumns.Three)
      .gap(2)

    filteredCards.forEach(card => {
      const colors = this.getCategoryColors(card.category)
      builder.addItem({
        id: card.id,
        content: this.formatCardContent(card),
        ...colors,
        classes: ['card', `card-${card.category}`]
      })
    })

    return builder.build()
  }

  // Create featured layout with hero card
  createFeaturedLayout(heroCardId: string) {
    const heroCard = this.cards.find(card => card.id === heroCardId)
    const otherCards = this.cards.filter(card => card.id !== heroCardId)

    if (!heroCard) return this.cardGrid

    const builder = GridBuilder.create()
      .id('featured-layout')
      .columns(GridColumns.Four)
      .gap(2)

    // Hero card spans 2x2
    const heroColors = this.getCategoryColors(heroCard.category)
    builder.addItem({
      id: heroCard.id,
      content: this.formatCardContent(heroCard),
      colSpan: 2,
      rowSpan: 2,
      ...heroColors,
      classes: ['card', 'hero-card', `card-${heroCard.category}`]
    })

    // Other cards in regular size
    otherCards.slice(0, 6).forEach(card => {
      const colors = this.getCategoryColors(card.category)
      builder.addItem({
        id: card.id,
        content: this.formatCardContent(card),
        ...colors,
        classes: ['card', `card-${card.category}`]
      })
    })

    return builder.build()
  }

  getCardStatistics() {
    const categories = [...new Set(this.cards.map(card => card.category))]
    const stats = categories.map(category => ({
      category,
      count: this.cards.filter(card => card.category === category).length,
      color: this.getCategoryColors(category)
    }))

    return {
      totalCards: this.cards.length,
      categories: stats,
      averageContentLength: Math.round(
        this.cards.reduce((sum, card) => sum + card.content.length, 0) / this.cards.length
      )
    }
  }

  render(): string {
    const stats = this.getCardStatistics()
    
    return `Responsive Card Layout

Grid Configuration:
• Columns: 4 (responsive)
• Gap: 2 units
• Alignment: Stretch
• Total Cards: ${stats.totalCards}

Category Distribution:
${stats.categories.map(cat => `• ${cat.category}: ${cat.count} cards`).join('\n')}

Layout Features:
• Responsive column adjustment
• Category-based color coding
• Hero card support with spanning
• Filtering and search capabilities
• Mobile/tablet/desktop layouts

Current Layout: 4-column desktop grid
Cards arranged by category with visual hierarchy`
  }
}

// Usage
const cardLayout = new ResponsiveCardLayout()

// Add dynamic content
cardLayout.addCard({
  id: 'news',
  title: 'Latest News',
  content: '• New feature released\n• Security update available\n• Community event next week',
  category: 'info'
})

// Update existing card
cardLayout.updateCard('stats', {
  content: 'Users: 1,298 (+51)\nActive: 94 (+5)\nGrowth: +15% (+3%)'
})

// Responsive layouts
console.log('Desktop Layout:')
cardLayout.setDesktopLayout()

console.log('Mobile Layout:')
cardLayout.setMobileLayout()

console.log('Featured Layout:')
const featuredGrid = cardLayout.createFeaturedLayout('system-health')

console.log(cardLayout.render())
```

## CSS Styling

```css
/* Grid container */
.grid {
  display: grid;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

/* Column classes */
.grid-cols-1 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
.grid-cols-2 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
.grid-cols-3 { grid-template-columns: repeat(3, minmax(0, 1fr)); }
.grid-cols-4 { grid-template-columns: repeat(4, minmax(0, 1fr)); }
.grid-cols-5 { grid-template-columns: repeat(5, minmax(0, 1fr)); }
.grid-cols-6 { grid-template-columns: repeat(6, minmax(0, 1fr)); }
.grid-cols-7 { grid-template-columns: repeat(7, minmax(0, 1fr)); }
.grid-cols-8 { grid-template-columns: repeat(8, minmax(0, 1fr)); }
.grid-cols-9 { grid-template-columns: repeat(9, minmax(0, 1fr)); }
.grid-cols-10 { grid-template-columns: repeat(10, minmax(0, 1fr)); }
.grid-cols-11 { grid-template-columns: repeat(11, minmax(0, 1fr)); }
.grid-cols-12 { grid-template-columns: repeat(12, minmax(0, 1fr)); }

/* Row classes */
.grid-rows-1 { grid-template-rows: repeat(1, minmax(0, 1fr)); }
.grid-rows-2 { grid-template-rows: repeat(2, minmax(0, 1fr)); }
.grid-rows-3 { grid-template-rows: repeat(3, minmax(0, 1fr)); }
.grid-rows-4 { grid-template-rows: repeat(4, minmax(0, 1fr)); }
.grid-rows-5 { grid-template-rows: repeat(5, minmax(0, 1fr)); }
.grid-rows-6 { grid-template-rows: repeat(6, minmax(0, 1fr)); }

/* Gap classes */
.gap-0 { gap: 0; }
.gap-1 { gap: 0.25rem; }
.gap-2 { gap: 0.5rem; }
.gap-3 { gap: 0.75rem; }
.gap-4 { gap: 1rem; }
.gap-5 { gap: 1.25rem; }
.gap-6 { gap: 1.5rem; }

.gap-x-0 { column-gap: 0; }
.gap-x-1 { column-gap: 0.25rem; }
.gap-x-2 { column-gap: 0.5rem; }
.gap-x-3 { column-gap: 0.75rem; }
.gap-x-4 { column-gap: 1rem; }

.gap-y-0 { row-gap: 0; }
.gap-y-1 { row-gap: 0.25rem; }
.gap-y-2 { row-gap: 0.5rem; }
.gap-y-3 { row-gap: 0.75rem; }
.gap-y-4 { row-gap: 1rem; }

/* Flow classes */
.grid-flow-row { grid-auto-flow: row; }
.grid-flow-col { grid-auto-flow: column; }
.grid-flow-row-dense { grid-auto-flow: row dense; }
.grid-flow-col-dense { grid-auto-flow: column dense; }

/* Column span classes */
.col-span-1 { grid-column: span 1 / span 1; }
.col-span-2 { grid-column: span 2 / span 2; }
.col-span-3 { grid-column: span 3 / span 3; }
.col-span-4 { grid-column: span 4 / span 4; }
.col-span-5 { grid-column: span 5 / span 5; }
.col-span-6 { grid-column: span 6 / span 6; }
.col-span-7 { grid-column: span 7 / span 7; }
.col-span-8 { grid-column: span 8 / span 8; }
.col-span-9 { grid-column: span 9 / span 9; }
.col-span-10 { grid-column: span 10 / span 10; }
.col-span-11 { grid-column: span 11 / span 11; }
.col-span-12 { grid-column: span 12 / span 12; }

/* Row span classes */
.row-span-1 { grid-row: span 1 / span 1; }
.row-span-2 { grid-row: span 2 / span 2; }
.row-span-3 { grid-row: span 3 / span 3; }
.row-span-4 { grid-row: span 4 / span 4; }
.row-span-5 { grid-row: span 5 / span 5; }
.row-span-6 { grid-row: span 6 / span 6; }

/* Column start classes */
.col-start-1 { grid-column-start: 1; }
.col-start-2 { grid-column-start: 2; }
.col-start-3 { grid-column-start: 3; }
.col-start-4 { grid-column-start: 4; }
.col-start-5 { grid-column-start: 5; }
.col-start-6 { grid-column-start: 6; }
.col-start-7 { grid-column-start: 7; }
.col-start-8 { grid-column-start: 8; }
.col-start-9 { grid-column-start: 9; }
.col-start-10 { grid-column-start: 10; }
.col-start-11 { grid-column-start: 11; }
.col-start-12 { grid-column-start: 12; }

/* Row start classes */
.row-start-1 { grid-row-start: 1; }
.row-start-2 { grid-row-start: 2; }
.row-start-3 { grid-row-start: 3; }
.row-start-4 { grid-row-start: 4; }
.row-start-5 { grid-row-start: 5; }
.row-start-6 { grid-row-start: 6; }

/* Grid item styling */
.grid > * {
  padding: 0.5rem;
  border: 1px solid #e5e7eb;
  border-radius: 4px;
  background: #ffffff;
  color: #1f2937;
  overflow: hidden;
  min-height: 2rem;
  display: flex;
  align-items: center;
  justify-content: center;
  text-align: center;
  white-space: pre-line;
  line-height: 1.4;
}

/* Dashboard-specific styles */
.dashboard-layout .header-panel {
  border-radius: 6px 6px 0 0;
  font-weight: bold;
  font-size: 1.1rem;
}

.dashboard-layout .nav-panel {
  text-align: left;
  padding: 1rem;
  line-height: 1.6;
}

.dashboard-layout .metric-panel {
  font-weight: 600;
  text-align: center;
}

.dashboard-layout .chart-panel {
  text-align: left;
  font-family: monospace;
  font-size: 0.9rem;
}

.dashboard-layout .activity-panel {
  text-align: left;
  font-size: 0.85rem;
  overflow-y: auto;
}

.dashboard-layout .logs-panel {
  text-align: left;
  font-family: monospace;
  font-size: 0.8rem;
}

.dashboard-layout .status-panel {
  border-radius: 0 0 6px 6px;
  font-weight: 500;
}

/* Card layout styles */
.card {
  transition: transform 0.2s ease, box-shadow 0.2s ease;
  cursor: pointer;
}

.card:hover {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
}

.hero-card {
  font-size: 1.1rem;
  font-weight: 600;
}

/* Responsive design */
@media (max-width: 768px) {
  .grid-cols-12 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
  .grid-cols-6 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .grid-cols-4 { grid-template-columns: repeat(2, minmax(0, 1fr)); }
  .grid-cols-3 { grid-template-columns: repeat(1, minmax(0, 1fr)); }
  
  .col-span-12,
  .col-span-6,
  .col-span-4,
  .col-span-3,
  .col-span-2 {
    grid-column: span 1 / span 1;
  }
}

@media (max-width: 480px) {
  .grid {
    gap: 0.5rem;
  }
  
  .grid > * {
    padding: 0.375rem;
    font-size: 0.9rem;
  }
}

/* Animation support */
.grid > * {
  animation: fadeInGrid 0.3s ease-out;
}

@keyframes fadeInGrid {
  from {
    opacity: 0;
    transform: scale(0.95);
  }
  to {
    opacity: 1;
    transform: scale(1);
  }
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .grid > * {
    background: #1f2937;
    color: #f9fafb;
    border-color: #374151;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .grid > * {
    border-width: 2px;
    border-color: #000000;
  }
}

/* Print styles */
@media print {
  .grid {
    display: block;
  }
  
  .grid > * {
    display: block;
    margin-bottom: 1rem;
    break-inside: avoid;
  }
}
```

## Best Practices

### 1. Use Appropriate Grid Sizes

```typescript
// ✅ Good - 12-column grid for complex layouts
const dashboardGrid = grid({
  columns: GridColumns.Twelve,
  items: [...]
})

// ✅ Good - 3-column grid for card layouts
const cardGrid = grid({
  columns: GridColumns.Three,
  items: [...]
})
```

### 2. Implement Proper Spanning

```typescript
// ✅ Good - logical spanning for headers and content
const layoutGrid = grid({
  columns: GridColumns.Four,
  items: [
    { content: 'Header', colSpan: 4 },    // Full width header
    { content: 'Sidebar', rowSpan: 2 },   // Tall sidebar
    { content: 'Main', colSpan: 3 }       // Wide main content
  ]
})
```

### 3. Use Consistent Gap Spacing

```typescript
// ✅ Good - consistent gap throughout application
const consistentGrid = grid({
  columns: GridColumns.Three,
  gap: 2,          // Uniform gap
  items: [...]
})

// ✅ Good - custom gaps for specific needs
const customGapGrid = grid({
  columns: GridColumns.Four,
  columnGap: 3,    // More horizontal space
  rowGap: 1,       // Less vertical space
  items: [...]
})
```

### 4. Implement Responsive Layouts

```typescript
// ✅ Good - responsive grid that adapts to screen size
class ResponsiveGrid {
  createGrid(screenSize: 'mobile' | 'tablet' | 'desktop') {
    const columns = {
      mobile: GridColumns.One,
      tablet: GridColumns.Two,
      desktop: GridColumns.Four
    }
    
    return grid({
      columns: columns[screenSize],
      gap: screenSize === 'mobile' ? 1 : 2,
      items: this.getItems()
    })
  }
}
```

### 5. Use Semantic Item Organization

```typescript
// ✅ Good - organized grid items with meaningful IDs
const semanticGrid = grid({
  columns: GridColumns.Six,
  items: [
    { id: 'header', content: 'Page Header', colSpan: 6 },
    { id: 'nav', content: 'Navigation', rowSpan: 3 },
    { id: 'main', content: 'Main Content', colSpan: 4 },
    { id: 'aside', content: 'Sidebar', rowSpan: 2 },
    { id: 'footer', content: 'Footer', colSpan: 6 }
  ]
})
```

## Related Widgets

- **[Panel](./panel)** - Container panels as grid items
- **[Tabs](./tabs)** - Tab-based content within grid areas
- **[Accordion](./accordion)** - Collapsible content in grid layouts
- **[Modal](./modal)** - Overlay dialogs over grid layouts

## Examples

- **[Basic Grid](../../examples/basic/grid-basic)** - Simple grid implementations
- **[Dashboard Grid](../../examples/advanced/dashboard-grid)** - Complex dashboard layouts
- **[Card Layout](../../examples/patterns/card-grid)** - Responsive card arrangements
- **[Form Grid](../../examples/apps/form-grid)** - Form layouts with grid positioning

The Grid widget provides comprehensive CSS Grid functionality with flexible column/row configurations, advanced positioning, and responsive design capabilities for building sophisticated layout systems.