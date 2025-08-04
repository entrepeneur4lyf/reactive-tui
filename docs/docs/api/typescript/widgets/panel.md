# Panel Widget

Modern, clean container panels with comprehensive border themes, color themes, and advanced styling options for organizing content.

## Overview

The Panel widget provides sophisticated container functionality with Unicode border themes, semantic color systems, and flexible styling options for creating dashboard panels, cards, menus, and content containers.

```typescript
import { panel, PanelStyles } from 'reactive-tui-ts'

const dashboardPanel = panel({
  id: 'dashboard-panel',
  title: 'System Overview',
  content: 'CPU: 45%\nMemory: 2.1GB\nDisk: 67%',
  borderStyle: 'unicode',
  colorTheme: 'dark',
  padding: 2,
  textAlign: 'left'
})
```

## Types

### PanelConfig

```typescript
interface PanelConfig {
  id: string
  title?: string
  content?: string
  width?: number
  height?: number
  backgroundColor?: string
  borderStyle?: 'none' | 'minimal' | 'shadow' | 'outlined' | 'clean' | 'unicode' | 'fancy'
  borderTheme?: string
  colorTheme?: string
  padding?: number
  textAlign?: 'left' | 'center' | 'right'
  titleStyle?: 'normal' | 'bold' | 'accent'
}
```

### PanelStyle

```typescript
interface PanelStyle {
  backgroundColor: string
  borderColor: string
  shadowColor: string
  titleColor: string
  contentColor: string
  borderTheme: BorderTheme
  colorTheme: ColorTheme
}
```

### BorderTheme

```typescript
interface BorderTheme {
  name: string
  description: string
  chars: BorderChars
  weight: 'light' | 'medium' | 'heavy'
  style: 'solid' | 'dashed' | 'dotted' | 'double' | 'rounded' | 'block'
}

interface BorderChars {
  horizontal: string
  vertical: string
  topLeft: string
  topRight: string
  bottomLeft: string
  bottomRight: string
  cross: string
  tDown: string
  tUp: string
  tLeft: string
  tRight: string
}
```

## Basic Usage

### Simple Panel

```typescript
import { panel } from 'reactive-tui-ts'

const basicPanel = panel({
  id: 'basic-panel',
  title: 'Welcome',
  content: 'This is a basic panel with default styling.',
  borderStyle: 'clean',
  padding: 1
})

// Build the panel element
const element = basicPanel.build()
console.log('Panel element:', element)
```

### Styled Panels

```typescript
// Modern panel with Unicode borders
const modernPanel = panel({
  id: 'modern-panel',
  title: 'Modern Dashboard',
  content: 'Status: Online\nUptime: 24h 15m\nConnections: 1,247',
  borderStyle: 'unicode',
  colorTheme: 'dark',
  textAlign: 'left',
  titleStyle: 'bold',
  padding: 2
})

// Shadowed card panel
const cardPanel = panel({
  id: 'card-panel',
  title: 'User Profile',
  content: 'Name: John Doe\nRole: Developer\nLast login: 2 hours ago',
  borderStyle: 'shadow',
  colorTheme: 'light',
  textAlign: 'center',
  padding: 3
})

// Minimal panel for menus
const menuPanel = panel({
  id: 'menu-panel',
  title: 'Options',
  content: '1. Settings\n2. Help\n3. Exit',
  borderStyle: 'minimal',
  textAlign: 'left',
  padding: 1
})
```

## Border Styles and Themes

### Available Border Styles

```typescript
// Clean ASCII borders (default)
const cleanPanel = panel({
  id: 'clean',
  title: 'Clean Style',
  content: 'Uses ASCII characters for compatibility',
  borderStyle: 'clean'
})

// Unicode rounded corners
const unicodePanel = panel({
  id: 'unicode',
  title: 'Unicode Style',
  content: 'Modern Unicode box-drawing characters',
  borderStyle: 'unicode'
})

// Heavy outlined borders
const outlinedPanel = panel({
  id: 'outlined',
  title: 'Outlined Style',
  content: 'Heavy border for emphasis',
  borderStyle: 'outlined'
})

// Double-line fancy borders
const fancyPanel = panel({
  id: 'fancy',
  title: 'Fancy Style',
  content: 'Double-line Unicode characters',
  borderStyle: 'fancy'
})

// Shadow effect borders
const shadowPanel = panel({
  id: 'shadow',
  title: 'Shadow Style',
  content: 'Block characters creating shadow effect',
  borderStyle: 'shadow'
})

// Minimal borders
const minimalPanel = panel({
  id: 'minimal',
  title: 'Minimal Style',
  content: 'Light borders for subtle separation',
  borderStyle: 'minimal'
})

// No borders
const noBorderPanel = panel({
  id: 'no-border',
  title: 'No Border',
  content: 'Content without borders',
  borderStyle: 'none'
})
```

### Custom Border Themes

```typescript
// Specific border theme override
const customBorderPanel = panel({
  id: 'custom-border',
  title: 'Custom Border Theme',
  content: 'Using specific border theme',
  borderStyle: 'unicode',
  borderTheme: 'double', // Override with double-line theme
  colorTheme: 'dark'
})

// Available border themes:
const borderThemes = [
  'ascii',      // Classic ASCII (+, -, |)
  'light',      // Light Unicode (─, │, ┌, ┐)
  'heavy',      // Heavy Unicode (━, ┃, ┏, ┓)
  'double',     // Double-line (═, ║, ╔, ╗)
  'rounded',    // Rounded corners (╭, ╮, ╰, ╯)
  'dashed-light',   // Dashed light (┄, ┆)
  'dashed-heavy',   // Dashed heavy (┅, ┇)
  'dotted',     // Dotted (┈, ┊)
  'block-light',    // Light shade blocks (░)
  'block-solid'     // Solid blocks (█)
]

// Example using different border themes
const themedPanels = borderThemes.map((theme, index) => 
  panel({
    id: `themed-${index}`,
    title: `${theme} Theme`,
    content: `Border theme: ${theme}`,
    borderTheme: theme,
    borderStyle: 'unicode'
  })
)
```

## Color Themes

### Theme-Based Styling

```typescript
// Light theme panel
const lightPanel = panel({
  id: 'light-panel',
  title: 'Light Theme',
  content: 'Bright and clean appearance',
  colorTheme: 'light',
  borderStyle: 'clean'
})

// Dark theme panel
const darkPanel = panel({
  id: 'dark-panel',
  title: 'Dark Theme',
  content: 'Dark background with light text',
  colorTheme: 'dark',
  borderStyle: 'unicode'
})

// High contrast theme
const contrastPanel = panel({
  id: 'contrast-panel',
  title: 'High Contrast',
  content: 'Enhanced accessibility colors',
  colorTheme: 'high-contrast',
  borderStyle: 'outlined'
})

// Custom background override
const customColorPanel = panel({
  id: 'custom-color',
  title: 'Custom Colors',
  content: 'Override with specific background',
  colorTheme: 'dark',
  backgroundColor: '#1e3a8a', // Custom blue background
  borderStyle: 'fancy'
})
```

### Semantic Color Usage

```typescript
// Panel automatically uses semantic colors based on theme:
// - panelBackground: Main panel background
// - panelBorder: Border colors
// - panelTitle: Title text color
// - panelContent: Content text color
// - panelShadow: Shadow effects

const semanticPanel = panel({
  id: 'semantic',
  title: 'Semantic Colors',
  content: 'Colors automatically chosen from theme palette',
  colorTheme: 'professional',
  borderStyle: 'shadow'
})
```

## Content and Layout

### Text Alignment

```typescript
// Left-aligned content
const leftAlignedPanel = panel({
  id: 'left-aligned',
  title: 'Left Aligned',
  content: 'Content aligned to the left\nSecond line\nThird line',
  textAlign: 'left',
  padding: 2
})

// Center-aligned content (default)
const centerAlignedPanel = panel({
  id: 'center-aligned',
  title: 'Center Aligned',
  content: 'Centered content\nSecond line\nThird line',
  textAlign: 'center',
  padding: 2
})

// Right-aligned content
const rightAlignedPanel = panel({
  id: 'right-aligned',
  title: 'Right Aligned',
  content: 'Right-aligned content\nSecond line\nThird line',
  textAlign: 'right',
  padding: 2
})
```

### Title Styling

```typescript
// Normal title (default)
const normalTitlePanel = panel({
  id: 'normal-title',
  title: 'Normal Title',
  content: 'Standard title styling',
  titleStyle: 'normal'
})

// Bold title
const boldTitlePanel = panel({
  id: 'bold-title',
  title: 'Bold Title',
  content: 'Emphasized title text',
  titleStyle: 'bold'
})

// Accent title
const accentTitlePanel = panel({
  id: 'accent-title',
  title: 'Accent Title',
  content: 'Special accent styling for title',
  titleStyle: 'accent'
})
```

### Sizing and Padding

```typescript
// Custom dimensions
const sizedPanel = panel({
  id: 'sized-panel',
  title: 'Custom Size',
  content: 'Panel with specific width and height',
  width: 40,
  height: 10,
  padding: 3,
  borderStyle: 'unicode'
})

// Different padding levels
const paddingPanels = [1, 2, 3, 4].map(pad => 
  panel({
    id: `padding-${pad}`,
    title: `Padding: ${pad}`,
    content: `Panel with ${pad} unit(s) of padding`,
    padding: pad,
    borderStyle: 'clean'
  })
)
```

## Pre-built Panel Types

### Dashboard Panel

```typescript
import { dashboardPanel } from 'reactive-tui-ts'

const dashboard = dashboardPanel({
  id: 'system-dashboard',
  title: 'System Status',
  content: 'CPU: 34%\nRAM: 67%\nDisk: 89%\nNetwork: Active',
  colorTheme: 'professional',
  padding: 2
})

// Uses 'clean' border style automatically
```

### Card Panel

```typescript
import { cardPanel } from 'reactive-tui-ts'

const userCard = cardPanel({
  id: 'user-card',
  title: 'User Information',
  content: 'Username: johndoe\nEmail: john@example.com\nStatus: Online',
  colorTheme: 'light',
  padding: 3
})

// Uses 'shadow' border style automatically
```

### Menu Panel

```typescript
import { menuPanel } from 'reactive-tui-ts'

const navigationMenu = menuPanel({
  id: 'nav-menu',
  title: 'Main Menu',
  content: '→ Dashboard\n→ Settings\n→ Profile\n→ Logout',
  textAlign: 'left',
  padding: 1
})

// Uses 'minimal' border style automatically
```

## Panel Composition

### Child Panel Support

```typescript
const parentPanel = panel({
  id: 'parent-panel',
  title: 'Parent Panel',
  content: 'This panel contains child elements',
  borderStyle: 'unicode',
  padding: 2
})

// Add child elements
const childPanel = panel({
  id: 'child-panel',
  title: 'Child Panel',
  content: 'Nested panel content',
  borderStyle: 'minimal',
  padding: 1
})

// Compose panels
const composedPanel = parentPanel.child(childPanel.build())
```

## Pre-built Style Functions

### PanelStyles Usage

```typescript
import { PanelStyles } from 'reactive-tui-ts'

// Get specific panel style configurations
const minimalStyle = PanelStyles.minimal()
const shadowStyle = PanelStyles.shadow()
const unicodeStyle = PanelStyles.unicode()
const fancyStyle = PanelStyles.fancy()
const outlinedStyle = PanelStyles.outlined()
const cleanStyle = PanelStyles.clean()
const noneStyle = PanelStyles.none()

// Apply style to panel
const styledPanel = panel({
  id: 'styled-panel',
  title: 'Styled Panel',
  content: 'Using pre-built style',
  borderStyle: 'shadow',
  colorTheme: 'dark'
})
```

## Real-World Examples

### System Monitor Dashboard

```typescript
import { panel, dashboardPanel, cardPanel } from 'reactive-tui-ts'

class SystemMonitor {
  private cpuPanel: any
  private memoryPanel: any
  private diskPanel: any
  private networkPanel: any
  private alertsPanel: any

  constructor() {
    this.setupPanels()
  }

  private setupPanels() {
    // CPU Usage Panel
    this.cpuPanel = dashboardPanel({
      id: 'cpu-monitor',
      title: 'CPU Usage',
      content: this.getCpuContent(),
      colorTheme: 'professional',
      padding: 2,
      textAlign: 'left'
    })

    // Memory Usage Panel
    this.memoryPanel = dashboardPanel({
      id: 'memory-monitor',
      title: 'Memory Usage',
      content: this.getMemoryContent(),
      colorTheme: 'professional',
      padding: 2,
      textAlign: 'left'
    })

    // Disk Usage Panel
    this.diskPanel = dashboardPanel({
      id: 'disk-monitor',
      title: 'Disk Usage',
      content: this.getDiskContent(),
      colorTheme: 'professional',
      padding: 2,
      textAlign: 'left'
    })

    // Network Panel with Unicode styling
    this.networkPanel = panel({
      id: 'network-monitor',
      title: 'Network Status',
      content: this.getNetworkContent(),
      borderStyle: 'unicode',
      colorTheme: 'dark',
      padding: 2,
      textAlign: 'left',
      titleStyle: 'bold'
    })

    // Alerts Panel with emphasis
    this.alertsPanel = panel({
      id: 'alerts-panel',
      title: 'System Alerts',
      content: this.getAlertsContent(),
      borderStyle: 'outlined',
      colorTheme: 'high-contrast',
      padding: 2,
      textAlign: 'left',
      titleStyle: 'accent'
    })
  }

  private getCpuContent(): string {
    return `Current Usage: 34%
Average Load: 1.2, 1.5, 1.8
Cores: 8 (4 physical)
Temperature: 67°C
Processes: 247 active`
  }

  private getMemoryContent(): string {
    return `Used: 8.2GB / 16GB (51%)
Available: 7.8GB
Cached: 2.1GB
Swap Used: 256MB / 2GB
Page Faults: 1,247/sec`
  }

  private getDiskContent(): string {
    return `Root (/): 89% (425GB / 500GB)
Home: 67% (201GB / 300GB)
Temp: 12% (2.4GB / 20GB)
I/O Read: 45MB/s
I/O Write: 23MB/s`
  }

  private getNetworkContent(): string {
    return `Interface: eth0 (1Gbps)
Status: Connected ✓
IP: 192.168.1.100
Traffic In: 12.5Mbps
Traffic Out: 3.2Mbps
Packets: 15,247 in, 8,934 out`
  }

  private getAlertsContent(): string {
    const alerts = [
      '⚠️  High disk usage on /',
      '⚠️  Memory usage above 80%',
      'ℹ️  System update available',
      '✓  All services running'
    ]
    return alerts.join('\n')
  }

  updateMetrics(metrics: {
    cpu?: number
    memory?: { used: number; total: number }
    disk?: { used: number; total: number }
    network?: { in: number; out: number }
    alerts?: string[]
  }) {
    if (metrics.cpu !== undefined) {
      const content = `Current Usage: ${metrics.cpu}%
Average Load: 1.2, 1.5, 1.8
Cores: 8 (4 physical)
Temperature: ${67 + Math.floor((metrics.cpu - 34) * 0.5)}°C
Processes: ${247 + Math.floor(metrics.cpu * 2)} active`
      
      // Update panel content (in real implementation)
      console.log('Updated CPU panel:', content)
    }

    if (metrics.memory) {
      const percentage = Math.round((metrics.memory.used / metrics.memory.total) * 100)
      const content = `Used: ${metrics.memory.used.toFixed(1)}GB / ${metrics.memory.total}GB (${percentage}%)
Available: ${(metrics.memory.total - metrics.memory.used).toFixed(1)}GB
Cached: 2.1GB
Swap Used: 256MB / 2GB
Page Faults: ${1000 + Math.floor(percentage * 10)}/sec`
      
      console.log('Updated Memory panel:', content)
    }

    if (metrics.alerts) {
      const alertIcons = ['⚠️', 'ℹ️', '✓', '❌']
      const content = metrics.alerts.map((alert, index) => 
        `${alertIcons[index % alertIcons.length]}  ${alert}`
      ).join('\n')
      
      console.log('Updated Alerts panel:', content)
    }
  }

  createDashboardLayout() {
    return {
      topRow: [
        this.cpuPanel.build(),
        this.memoryPanel.build(),
        this.diskPanel.build()
      ],
      bottomRow: [
        this.networkPanel.build(),
        this.alertsPanel.build()
      ]
    }
  }

  render(): string {
    const layout = this.createDashboardLayout()
    
    return `System Monitor Dashboard
    
Top Row:
${layout.topRow.map(panel => `[${panel.id}] ${panel.attributes['data-title']}`).join(' | ')}

Bottom Row:
${layout.bottomRow.map(panel => `[${panel.id}] ${panel.attributes['data-title']}`).join(' | ')}

All panels use appropriate themes and border styles for visual hierarchy.`
  }
}

// Usage
const monitor = new SystemMonitor()

// Simulate real-time updates
setInterval(() => {
  monitor.updateMetrics({
    cpu: 30 + Math.floor(Math.random() * 40),
    memory: { used: 6 + Math.random() * 4, total: 16 },
    network: { in: 10 + Math.random() * 10, out: 2 + Math.random() * 5 },
    alerts: [
      'High CPU usage detected',
      'Memory usage normal',
      'Network connectivity stable',
      'All systems operational'
    ]
  })
}, 5000)

console.log(monitor.render())
```

### Application Settings Interface

```typescript
import { panel, cardPanel, menuPanel } from 'reactive-tui-ts'

class SettingsInterface {
  private categoryPanels: Map<string, any> = new Map()
  private currentCategory: string = 'general'

  constructor() {
    this.setupSettingsCategories()
  }

  private setupSettingsCategories() {
    // General Settings Panel
    this.categoryPanels.set('general', cardPanel({
      id: 'general-settings',
      title: 'General Settings',
      content: this.getGeneralSettingsContent(),
      colorTheme: 'light',
      padding: 3,
      textAlign: 'left'
    }))

    // Appearance Settings Panel
    this.categoryPanels.set('appearance', panel({
      id: 'appearance-settings',
      title: 'Appearance & Theme',
      content: this.getAppearanceSettingsContent(),
      borderStyle: 'unicode',
      colorTheme: 'dark',
      padding: 3,
      textAlign: 'left',
      titleStyle: 'bold'
    }))

    // Privacy Settings Panel
    this.categoryPanels.set('privacy', panel({
      id: 'privacy-settings',  
      title: 'Privacy & Security',
      content: this.getPrivacySettingsContent(),
      borderStyle: 'outlined',
      colorTheme: 'high-contrast',
      padding: 3,
      textAlign: 'left',
      titleStyle: 'accent'
    }))

    // Advanced Settings Panel
    this.categoryPanels.set('advanced', panel({
      id: 'advanced-settings',
      title: 'Advanced Options',
      content: this.getAdvancedSettingsContent(),
      borderStyle: 'fancy',
      colorTheme: 'professional',
      padding: 3,
      textAlign: 'left'
    }))

    // About Panel
    this.categoryPanels.set('about', panel({
      id: 'about-panel',
      title: 'About Application',
      content: this.getAboutContent(),
      borderStyle: 'shadow',
      colorTheme: 'light',
      padding: 3,
      textAlign: 'center'
    }))
  }

  private getGeneralSettingsContent(): string {
    return `Application Language: English
Startup Behavior: Launch on boot
Default Save Location: ~/Documents
Auto-save Interval: 5 minutes
Check for Updates: Weekly

Recent Activity:
• Settings saved 2 hours ago
• Language changed yesterday  
• Update check: 1 day ago`
  }

  private getAppearanceSettingsContent(): string {
    return `Current Theme: Dark Mode 🌙
Font Family: Fira Code
Font Size: 14px
Color Scheme: Professional Blue
Window Opacity: 95%

Border Styles:
• Panel Borders: Unicode
• Button Borders: Clean  
• Input Borders: Outlined
• Menu Borders: Minimal

Animation: Enabled
Transitions: Smooth (300ms)`
  }

  private getPrivacySettingsContent(): string {
    return `Data Collection: Minimal 🔒
Analytics: Disabled
Crash Reports: Anonymous only
Location Services: Disabled
Telemetry: Essential only

Security Features:
• Auto-lock: After 30 minutes
• Secure Storage: Enabled
• Encryption: AES-256
• Two-Factor Auth: Available

Privacy Level: Enhanced`
  }

  private getAdvancedSettingsContent(): string {
    return `Debug Mode: Disabled
Logging Level: Info
Performance Mode: Balanced
Memory Management: Auto
Cache Size: 256MB

Developer Options:
• Show Debug Info: No
• Enable Experimental: No
• API Rate Limiting: Standard
• Background Sync: Enabled

System Integration:
• Shell Integration: Enabled
• Protocol Handlers: Registered
• File Associations: Active`
  }

  private getAboutContent(): string {
    return `Reactive TUI Application
Version 2.1.0

© 2024 Reactive TUI Team
Licensed under MIT License

Credits:
• Core Team: 12 contributors
• Community: 500+ contributors  
• Libraries: 23 dependencies
• Platform: Cross-platform

Support:
📧 support@reactive-tui.com
🌐 https://reactive-tui.com
📖 Documentation available online`
  }

  private createNavigationMenu(): any {
    const menuContent = [
      this.currentCategory === 'general' ? '→ General Settings' : '  General Settings',
      this.currentCategory === 'appearance' ? '→ Appearance & Theme' : '  Appearance & Theme',
      this.currentCategory === 'privacy' ? '→ Privacy & Security' : '  Privacy & Security',
      this.currentCategory === 'advanced' ? '→ Advanced Options' : '  Advanced Options',
      '',
      this.currentCategory === 'about' ? '→ About' : '  About'
    ].join('\n')

    return menuPanel({
      id: 'settings-navigation',
      title: 'Settings Categories',
      content: menuContent,
      padding: 2,
      textAlign: 'left'
    })
  }

  switchCategory(category: string) {
    if (this.categoryPanels.has(category)) {
      this.currentCategory = category
      console.log(`Switched to ${category} settings`)
    }
  }

  getCurrentPanel(): any {
    return this.categoryPanels.get(this.currentCategory)
  }

  updateSetting(category: string, setting: string, value: any) {
    console.log(`Updated ${category}.${setting} = ${value}`)
    
    // Refresh panel content based on category
    const panel = this.categoryPanels.get(category)
    if (panel) {
      // In real implementation, would regenerate content
      console.log(`Refreshed ${category} panel content`)
    }
  }

  exportSettings(): Record<string, any> {
    return {
      general: {
        language: 'English',
        startupBehavior: 'launch-on-boot',
        saveLocation: '~/Documents',
        autoSaveInterval: 5,
        updateCheck: 'weekly'
      },
      appearance: {
        theme: 'dark',
        fontFamily: 'Fira Code',
        fontSize: 14,
        colorScheme: 'professional-blue',
        opacity: 95,
        animations: true
      },
      privacy: {
        dataCollection: 'minimal',
        analytics: false,
        crashReports: 'anonymous',
        locationServices: false,
        telemetry: 'essential'
      },
      advanced: {
        debugMode: false,
        loggingLevel: 'info',
        performanceMode: 'balanced',
        memoryManagement: 'auto',
        cacheSize: 256
      }
    }
  }

  importSettings(settings: Record<string, any>) {
    Object.entries(settings).forEach(([category, categorySettings]) => {
      Object.entries(categorySettings).forEach(([setting, value]) => {
        this.updateSetting(category, setting, value)
      })
    })
    
    console.log('Settings imported successfully')
  }

  render(): string {
    const navigationMenu = this.createNavigationMenu()
    const currentPanel = this.getCurrentPanel()
    
    return `Settings Interface Layout:

Navigation Menu:
${navigationMenu.build().attributes['data-title']}
Current Category: ${this.currentCategory}

Active Panel:
${currentPanel.build().attributes['data-title']}
Border Style: ${currentPanel.build().attributes['data-border-style']}
Color Theme: ${currentPanel.build().attributes['data-color-theme']}

The interface uses different panel styles for visual hierarchy:
- Navigation: Minimal borders for clean separation
- General: Card style with shadow for primary content
- Appearance: Unicode borders with dark theme
- Privacy: Outlined borders with high-contrast theme  
- Advanced: Fancy double-line borders
- About: Shadow borders with centered content`
  }
}

// Usage
const settingsInterface = new SettingsInterface()

// Navigate through settings
settingsInterface.switchCategory('appearance')
settingsInterface.switchCategory('privacy')

// Update settings
settingsInterface.updateSetting('appearance', 'theme', 'light')
settingsInterface.updateSetting('privacy', 'analytics', true)

// Export/import configuration
const config = settingsInterface.exportSettings()
console.log('Exported settings:', config)

settingsInterface.importSettings(config)

console.log(settingsInterface.render())
```

## CSS Styling

```css
/* Panel base styles */
.panel {
  display: inline-block;
  position: relative;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  box-sizing: border-box;
  overflow: hidden;
}

/* Border style classes */
.panel-clean {
  border: 1px solid #e2e8f0;
  background: #ffffff;
}

.panel-minimal {
  border: 1px solid #f1f5f9;
  background: #fefefe;
}

.panel-shadow {
  border: 1px solid #d1d5db;
  background: #ffffff;
  box-shadow: 2px 2px 4px rgba(0, 0, 0, 0.1);
}

.panel-outlined {
  border: 2px solid #374151;
  background: #f9fafb;
}

.panel-unicode {
  border: 1px solid #6b7280;
  background: #ffffff;
  border-radius: 4px;
}

.panel-fancy {
  border: 2px double #4f46e5;
  background: #fefbff;
}

.panel-none {
  border: none;
  background: transparent;
}

/* Text alignment */
.text-left {
  text-align: left;
}

.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

/* Panel content */
.panel-title {
  font-weight: bold;
  padding: 0.5rem 1rem;
  border-bottom: 1px solid #e5e7eb;
  background: #f8fafc;
  margin: 0;
}

.panel-title.title-bold {
  font-weight: 700;
}

.panel-title.title-accent {
  color: #4f46e5;
  font-weight: 600;
}

.panel-content {
  padding: 1rem;
  line-height: 1.5;
  white-space: pre-line;
}

/* Theme-specific styles */
.panel[data-color-theme="light"] {
  background: #ffffff;
  color: #1f2937;
}

.panel[data-color-theme="light"] .panel-title {
  background: #f8fafc;
  color: #374151;
  border-bottom-color: #e5e7eb;
}

.panel[data-color-theme="dark"] {
  background: #1f2937;
  color: #f9fafb;
  border-color: #4b5563;
}

.panel[data-color-theme="dark"] .panel-title {
  background: #374151;
  color: #f3f4f6;
  border-bottom-color: #6b7280;
}

.panel[data-color-theme="high-contrast"] {
  background: #000000;
  color: #ffffff;
  border-color: #ffffff;
  border-width: 2px;
}

.panel[data-color-theme="high-contrast"] .panel-title {
  background: #ffffff;
  color: #000000;
  border-bottom-color: #ffffff;
}

.panel[data-color-theme="professional"] {
  background: #f8fafc;
  color: #334155;
  border-color: #cbd5e1;
}

.panel[data-color-theme="professional"] .panel-title {
  background: #e2e8f0;
  color: #1e293b;
  border-bottom-color: #cbd5e1;
}

/* Border character display (for Unicode themes) */
.panel-unicode::before {
  content: '';
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  pointer-events: none;
  font-family: 'Fira Code', monospace;
}

/* Responsive design */
@media (max-width: 768px) {
  .panel {
    min-width: 200px;
    max-width: 100%;
  }
  
  .panel-content {
    padding: 0.75rem;
    font-size: 0.9rem;
  }
  
  .panel-title {
    padding: 0.375rem 0.75rem;
    font-size: 0.95rem;
  }
}

/* Animation support */
.panel {
  transition: all 0.2s ease;
}

.panel:hover {
  transform: translateY(-1px);
}

.panel-shadow:hover {
  box-shadow: 2px 4px 8px rgba(0, 0, 0, 0.15);
}

/* Accessibility */
.panel:focus-within {
  outline: 2px solid #4f46e5;
  outline-offset: 2px;
}

@media (prefers-reduced-motion: reduce) {
  .panel {
    transition: none;
  }
  
  .panel:hover {
    transform: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .panel {
    border-width: 2px;
    border-color: currentColor;
  }
  
  .panel-title {
    border-bottom-width: 2px;
  }
}

/* Print styles */
@media print {
  .panel {
    border: 1px solid #000000;
    background: #ffffff;
    color: #000000;
    box-shadow: none;
  }
  
  .panel-title {
    background: #f5f5f5;
    color: #000000;
  }
}
```

## Best Practices

### 1. Choose Appropriate Border Styles

```typescript
// ✅ Good - clean borders for general content
const contentPanel = panel({
  id: 'content',
  title: 'Article Content',
  content: 'Main article text...',
  borderStyle: 'clean'
})

// ✅ Good - unicode borders for modern interfaces
const modernPanel = panel({
  id: 'modern',
  title: 'Modern Interface',
  content: 'Updated design...',
  borderStyle: 'unicode'
})

// ✅ Good - outlined borders for emphasis
const importantPanel = panel({
  id: 'important',
  title: 'Important Notice',
  content: 'Critical information...',
  borderStyle: 'outlined'
})
```

### 2. Use Semantic Color Themes

```typescript
// ✅ Good - appropriate themes for context
const dashboardPanel = panel({
  id: 'dashboard',
  title: 'System Status',
  content: 'Status information...',
  colorTheme: 'professional',
  borderStyle: 'clean'
})

const alertPanel = panel({
  id: 'alert',
  title: 'Alert Message',  
  content: 'Warning information...',
  colorTheme: 'high-contrast',
  borderStyle: 'outlined'
})
```

### 3. Consistent Visual Hierarchy

```typescript
// ✅ Good - consistent styling within application
const primaryPanel = panel({
  id: 'primary',
  title: 'Main Content',
  borderStyle: 'unicode',
  colorTheme: 'light',
  titleStyle: 'bold'
})

const secondaryPanel = panel({
  id: 'secondary', 
  title: 'Supporting Info',
  borderStyle: 'minimal',
  colorTheme: 'light',
  titleStyle: 'normal'
})
```

### 4. Proper Text Alignment

```typescript
// ✅ Good - left-align for readable content
const textPanel = panel({
  id: 'text-content',
  title: 'Article',
  content: 'Long form text content that should be left-aligned for readability...',
  textAlign: 'left'
})

// ✅ Good - center-align for status displays
const statusPanel = panel({
  id: 'status',
  title: 'System Status',
  content: 'ONLINE',
  textAlign: 'center'
})
```

### 5. Responsive Padding

```typescript
// ✅ Good - appropriate padding for content type
const compactPanel = panel({
  id: 'compact',
  title: 'Menu',
  content: 'Option 1\nOption 2\nOption 3',
  padding: 1,
  borderStyle: 'minimal'
})

const spaciousPanel = panel({
  id: 'spacious',
  title: 'Content Area',
  content: 'Detailed content with breathing room...',
  padding: 3,
  borderStyle: 'clean'
})
```

## Related Widgets

- **[Tabs](./tabs)** - Tab-based content organization
- **[Accordion](./accordion)** - Expandable content sections
- **[Modal](./modal)** - Overlay panels and dialogs
- **[Grid](./grid)** - Layout containers for panel arrangement

## Examples

- **[Basic Panels](../../examples/basic/panel-basic)** - Simple panel implementations
- **[Themed Panels](../../examples/advanced/panel-themes)** - Border and color theme usage
- **[Dashboard Layout](../../examples/apps/dashboard-panels)** - System monitoring interface
- **[Settings Interface](../../examples/advanced/settings-panels)** - Application configuration UI

The Panel widget provides comprehensive container functionality with sophisticated theming, Unicode border support, and flexible styling options for creating organized, visually appealing interfaces.