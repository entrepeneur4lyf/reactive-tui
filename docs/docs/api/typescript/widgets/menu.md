# Menu Widget

Hierarchical navigation menus with comprehensive keyboard navigation, submenus, state management, and JSON/YAML configuration support for building complex application interfaces.

## Overview

The Menu widget provides sophisticated navigation functionality with support for action items, submenus, toggles, radio buttons, separators, and headers. Features include keyboard navigation, state persistence, theme integration, and dynamic configuration loading.

```typescript
import { menu, menuItem, submenuItem } from 'reactive-tui-ts'

const applicationMenu = menu({
  id: 'main-menu',
  orientation: 'vertical',
  items: [
    menuItem({ id: 'new', label: 'New File', action: 'file:new', icon: '📄', shortcut: 'Ctrl+N' }),
    menuItem({ id: 'open', label: 'Open', action: 'file:open', icon: '📂', shortcut: 'Ctrl+O' }),
    submenuItem({
      id: 'recent', 
      label: 'Recent Files', 
      icon: '📋',
      items: [
        menuItem({ id: 'recent1', label: 'document.txt', action: 'file:recent:1' }),
        menuItem({ id: 'recent2', label: 'script.js', action: 'file:recent:2' })
      ]
    })
  ],
  colorTheme: 'professional',
  onItemSelect: (item) => console.log('Selected:', item.label)
})
```

## Types

### MenuConfig

```typescript
interface MenuConfig {
  id: string
  items?: MenuItem[]
  orientation?: MenuOrientation
  style?: MenuStyle
  cssClasses?: string[]
  visible?: boolean
  focusable?: boolean
  colorTheme?: string
  
  // JSON/YAML support
  configData?: string
  configFormat?: 'json' | 'yaml'
  
  // Event handlers
  onItemSelect?: (item: MenuItem, event?: any) => void
  onItemHover?: (item: MenuItem, event?: any) => void
  onSubmenuOpen?: (item: MenuItem, event?: any) => void
  onSubmenuClose?: (item: MenuItem, event?: any) => void
  onKeyPress?: (key: string, event?: any) => void
}
```

### MenuItem

```typescript
interface MenuItem {
  id: string
  label: string
  itemType: MenuItemType
  icon?: string
  shortcut?: string
  enabled?: boolean
  visible?: boolean
  cssClasses?: string[]
  tooltip?: string
  data?: Record<string, string>
}
```

### MenuItemType

```typescript
interface MenuItemType {
  type: 'action' | 'submenu' | 'separator' | 'header' | 'toggle' | 'radio'
  action?: string
  items?: MenuItem[]
  state?: boolean
  group?: string
  selected?: boolean
}
```

### MenuStyle

```typescript
interface MenuStyle {
  background?: string
  foreground?: string
  hoverBackground?: string
  hoverForeground?: string
  selectedBackground?: string
  selectedForeground?: string
  disabledForeground?: string
  borderColor?: string
  separatorColor?: string
  padding?: number
  itemHeight?: number
  showBorders?: boolean
  showIcons?: boolean
  showShortcuts?: boolean
  submenuIndent?: number
}
```

### MenuState

```typescript
interface MenuState {
  selectedIndex: number
  navigationStack: MenuItem[][]
  currentItems: MenuItem[]
  expandedPaths: string[]
  radioSelections: Record<string, string>
  toggleStates: Record<string, boolean>
}
```

## Basic Usage

### Simple Action Menu

```typescript
import { menu, menuItem, separatorItem } from 'reactive-tui-ts'

const fileMenu = menu({
  id: 'file-menu',
  items: [
    menuItem({
      id: 'new',
      label: 'New',
      action: 'file:new',
      icon: '📄',
      shortcut: 'Ctrl+N',
      tooltip: 'Create a new file'
    }),
    menuItem({
      id: 'open',
      label: 'Open',
      action: 'file:open',
      icon: '📂',
      shortcut: 'Ctrl+O'
    }),
    menuItem({
      id: 'save',
      label: 'Save',
      action: 'file:save',
      icon: '💾',
      shortcut: 'Ctrl+S'
    }),
    separatorItem('sep1'),
    menuItem({
      id: 'exit',
      label: 'Exit',
      action: 'app:exit',
      icon: '🚪',
      shortcut: 'Alt+F4'
    })
  ],
  onItemSelect: (item) => {
    console.log(`Action triggered: ${item.itemType.action}`)
  }
})

// Build the menu element
const element = fileMenu.build()
console.log('Menu element:', element)
```

### Menu with Submenus

```typescript
import { menu, menuItem, submenuItem, headerItem } from 'reactive-tui-ts'

const editMenu = menu({
  id: 'edit-menu',
  items: [
    headerItem({ id: 'basic-header', label: 'Basic Operations' }),
    menuItem({ id: 'undo', label: 'Undo', action: 'edit:undo', shortcut: 'Ctrl+Z' }),
    menuItem({ id: 'redo', label: 'Redo', action: 'edit:redo', shortcut: 'Ctrl+Y' }),
    separatorItem('sep1'),
    
    headerItem({ id: 'clipboard-header', label: 'Clipboard' }),
    menuItem({ id: 'cut', label: 'Cut', action: 'edit:cut', shortcut: 'Ctrl+X' }),
    menuItem({ id: 'copy', label: 'Copy', action: 'edit:copy', shortcut: 'Ctrl+C' }),
    menuItem({ id: 'paste', label: 'Paste', action: 'edit:paste', shortcut: 'Ctrl+V' }),
    
    submenuItem({
      id: 'paste-special',
      label: 'Paste Special',
      icon: '📋',
      items: [
        menuItem({ id: 'paste-text', label: 'Paste as Text', action: 'edit:paste:text' }),
        menuItem({ id: 'paste-formatted', label: 'Paste Formatted', action: 'edit:paste:formatted' }),
        menuItem({ id: 'paste-unformatted', label: 'Paste Unformatted', action: 'edit:paste:unformatted' })
      ]
    }),
    
    separatorItem('sep2'),
    
    submenuItem({
      id: 'find-replace',
      label: 'Find & Replace',
      icon: '🔍',
      items: [
        menuItem({ id: 'find', label: 'Find', action: 'edit:find', shortcut: 'Ctrl+F' }),
        menuItem({ id: 'find-next', label: 'Find Next', action: 'edit:find-next', shortcut: 'F3' }),
        menuItem({ id: 'replace', label: 'Replace', action: 'edit:replace', shortcut: 'Ctrl+H' }),
        menuItem({ id: 'replace-all', label: 'Replace All', action: 'edit:replace-all', shortcut: 'Ctrl+Shift+H' })
      ]
    })
  ],
  colorTheme: 'professional',
  onItemSelect: (item) => console.log('Edit action:', item.itemType.action),
  onSubmenuOpen: (item) => console.log('Opened submenu:', item.label),
  onSubmenuClose: (item) => console.log('Closed submenu:', item.label)
})
```

## Menu Item Types

### Toggle Items

```typescript
import { menu, toggleItem, separatorItem } from 'reactive-tui-ts'

const viewMenu = menu({
  id: 'view-menu',
  items: [
    toggleItem({
      id: 'toolbar',
      label: 'Show Toolbar',
      state: true,
      icon: '🔧',
      shortcut: 'F9'
    }),
    toggleItem({
      id: 'sidebar',
      label: 'Show Sidebar',
      state: false,
      icon: '📋',
      shortcut: 'F12'
    }),
    toggleItem({
      id: 'status-bar',
      label: 'Show Status Bar',
      state: true,
      icon: '📊'
    }),
    separatorItem('sep1'),
    toggleItem({
      id: 'fullscreen',
      label: 'Full Screen',
      state: false,
      icon: '🖥️',
      shortcut: 'F11'
    })
  ],
  onItemSelect: (item) => {
    console.log(`Toggle ${item.label}: ${item.itemType.state}`)
  }
})

// Access menu API to programmatically toggle items
const menuApi = viewMenu.api
menuApi.setToggleState('sidebar', true)
console.log('Menu state:', menuApi.getState())
```

### Radio Button Items

```typescript
import { menu, radioItem, headerItem, separatorItem } from 'reactive-tui-ts'

const formatMenu = menu({
  id: 'format-menu',
  items: [
    headerItem({ id: 'alignment-header', label: 'Text Alignment' }),
    radioItem({
      id: 'align-left',
      label: 'Align Left',
      group: 'alignment',
      selected: true,
      icon: '⬅️',
      shortcut: 'Ctrl+L'
    }),
    radioItem({
      id: 'align-center',
      label: 'Align Center',
      group: 'alignment',
      icon: '↔️',
      shortcut: 'Ctrl+E'
    }),
    radioItem({
      id: 'align-right',
      label: 'Align Right',
      group: 'alignment',
      icon: '➡️',
      shortcut: 'Ctrl+R'
    }),
    
    separatorItem('sep1'),
    
    headerItem({ id: 'size-header', label: 'Font Size' }),
    radioItem({
      id: 'size-small',
      label: 'Small (12px)',
      group: 'font-size',
      icon: '🔤'
    }),
    radioItem({
      id: 'size-medium',
      label: 'Medium (14px)',
      group: 'font-size',
      selected: true,
      icon: '🔤'
    }),
    radioItem({
      id: 'size-large',
      label: 'Large (16px)',
      group: 'font-size',
      icon: '🔤'
    })
  ],
  onItemSelect: (item) => {
    if (item.itemType.type === 'radio') {
      console.log(`Selected ${item.itemType.group}: ${item.label}`)
    }
  }
})

// Access radio selections
const menuApi = formatMenu.api
menuApi.setRadioSelection('alignment', 'align-center')
const state = menuApi.getState()
console.log('Alignment selection:', state.radioSelections['alignment'])
```

## Menu Orientations

### Vertical Context Menu

```typescript
import { contextMenu, menuItem, submenuItem } from 'reactive-tui-ts'

const rightClickMenu = contextMenu({
  id: 'context-menu',
  items: [
    menuItem({ id: 'cut', label: 'Cut', action: 'edit:cut', icon: '✂️', shortcut: 'Ctrl+X' }),
    menuItem({ id: 'copy', label: 'Copy', action: 'edit:copy', icon: '📋', shortcut: 'Ctrl+C' }),
    menuItem({ id: 'paste', label: 'Paste', action: 'edit:paste', icon: '📄', shortcut: 'Ctrl+V' }),
    separatorItem('sep1'),
    
    submenuItem({
      id: 'transform',
      label: 'Transform',
      icon: '🔄',
      items: [
        menuItem({ id: 'uppercase', label: 'UPPERCASE', action: 'transform:upper' }),
        menuItem({ id: 'lowercase', label: 'lowercase', action: 'transform:lower' }),
        menuItem({ id: 'capitalize', label: 'Capitalize', action: 'transform:capitalize' })
      ]
    }),
    
    separatorItem('sep2'),
    menuItem({ id: 'select-all', label: 'Select All', action: 'edit:select-all', shortcut: 'Ctrl+A' }),
    menuItem({ id: 'properties', label: 'Properties', action: 'item:properties', icon: '⚙️' })
  ],
  style: {
    showBorders: true,
    padding: 1,
    itemHeight: 1.2
  },
  colorTheme: 'light'
})
```

### Horizontal Menu Bar

```typescript
import { menuBar, menuItem, submenuItem } from 'reactive-tui-ts'

const applicationMenuBar = menuBar({
  id: 'menu-bar',
  items: [
    submenuItem({
      id: 'file',
      label: 'File',
      items: [
        menuItem({ id: 'new', label: 'New', action: 'file:new', shortcut: 'Ctrl+N' }),
        menuItem({ id: 'open', label: 'Open', action: 'file:open', shortcut: 'Ctrl+O' }),
        menuItem({ id: 'save', label: 'Save', action: 'file:save', shortcut: 'Ctrl+S' }),
        separatorItem('sep1'),
        menuItem({ id: 'exit', label: 'Exit', action: 'app:exit' })
      ]
    }),
    submenuItem({
      id: 'edit',
      label: 'Edit',
      items: [
        menuItem({ id: 'undo', label: 'Undo', action: 'edit:undo', shortcut: 'Ctrl+Z' }),
        menuItem({ id: 'redo', label: 'Redo', action: 'edit:redo', shortcut: 'Ctrl+Y' }),
        separatorItem('sep1'),
        menuItem({ id: 'cut', label: 'Cut', action: 'edit:cut', shortcut: 'Ctrl+X' }),
        menuItem({ id: 'copy', label: 'Copy', action: 'edit:copy', shortcut: 'Ctrl+C' }),
        menuItem({ id: 'paste', label: 'Paste', action: 'edit:paste', shortcut: 'Ctrl+V' })
      ]
    }),
    submenuItem({
      id: 'view',
      label: 'View',
      items: [
        toggleItem({ id: 'toolbar', label: 'Toolbar', state: true }),
        toggleItem({ id: 'status-bar', label: 'Status Bar', state: true }),
        separatorItem('sep1'),
        radioItem({ id: 'zoom-50', label: '50%', group: 'zoom' }),
        radioItem({ id: 'zoom-100', label: '100%', group: 'zoom', selected: true }),
        radioItem({ id: 'zoom-150', label: '150%', group: 'zoom' })
      ]
    }),
    submenuItem({
      id: 'help',
      label: 'Help',
      items: [
        menuItem({ id: 'docs', label: 'Documentation', action: 'help:docs', shortcut: 'F1' }),
        menuItem({ id: 'shortcuts', label: 'Keyboard Shortcuts', action: 'help:shortcuts' }),
        separatorItem('sep1'),
        menuItem({ id: 'about', label: 'About', action: 'help:about' })
      ]
    })
  ],
  style: {
    showBorders: false,
    padding: 2,
    itemHeight: 1
  },
  colorTheme: 'professional'
})
```

### Dropdown Menu

```typescript
import { dropdownMenu, menuItem, separatorItem } from 'reactive-tui-ts'

const sortDropdown = dropdownMenu({
  id: 'sort-dropdown',
  items: [
    headerItem({ id: 'sort-header', label: 'Sort Options' }),
    menuItem({ id: 'sort-name', label: 'Sort by Name', action: 'sort:name', icon: '📝' }),
    menuItem({ id: 'sort-date', label: 'Sort by Date', action: 'sort:date', icon: '📅' }),
    menuItem({ id: 'sort-size', label: 'Sort by Size', action: 'sort:size', icon: '📊' }),
    menuItem({ id: 'sort-type', label: 'Sort by Type', action: 'sort:type', icon: '📁' }),
    separatorItem('sep1'),
    radioItem({ id: 'order-asc', label: 'Ascending', group: 'order', selected: true, icon: '⬆️' }),
    radioItem({ id: 'order-desc', label: 'Descending', group: 'order', icon: '⬇️' })
  ],
  colorTheme: 'light',
  onItemSelect: (item) => {
    if (item.itemType.action?.startsWith('sort:')) {
      console.log('Sorting by:', item.label)
    }
  }
})
```

## Dynamic Configuration

### JSON Configuration Loading

```typescript
import { menu } from 'reactive-tui-ts'

const menuJsonConfig = `[
  {
    "id": "file-new",
    "label": "New Document",
    "itemType": { "type": "action", "action": "file:new" },
    "icon": "📄",
    "shortcut": "Ctrl+N"
  },
  {
    "id": "file-open",
    "label": "Open Document",
    "itemType": { "type": "action", "action": "file:open" },
    "icon": "📂",
    "shortcut": "Ctrl+O"
  },
  {
    "id": "recent-files",
    "label": "Recent Files",
    "itemType": {
      "type": "submenu",
      "items": [
        {
          "id": "recent-1",
          "label": "document1.txt",
          "itemType": { "type": "action", "action": "file:recent:1" }
        },
        {
          "id": "recent-2", 
          "label": "script.js",
          "itemType": { "type": "action", "action": "file:recent:2" }
        }
      ]
    },
    "icon": "📋"
  }
]`

const dynamicMenu = menu({
  id: 'dynamic-menu',
  configData: menuJsonConfig,
  configFormat: 'json',
  colorTheme: 'professional',
  onItemSelect: (item) => {
    console.log('Dynamic menu action:', item.itemType.action)
  }
})

// Runtime configuration updates
const menuApi = dynamicMenu.api
menuApi.loadFromJson(`[
  {
    "id": "updated-item",
    "label": "Updated Menu Item",
    "itemType": { "type": "action", "action": "updated:action" }
  }
]`)
```

### Runtime Menu Management

```typescript
import { menu, menuItem, submenuItem } from 'reactive-tui-ts'

const manageableMenu = menu({
  id: 'manageable-menu',
  items: [
    menuItem({ id: 'initial', label: 'Initial Item', action: 'initial:action' })
  ],
  colorTheme: 'dark'
})

const menuApi = manageableMenu.api

// Add new items dynamically
menuApi.addItem(menuItem({
  id: 'dynamic-1',
  label: 'Dynamic Item 1',
  action: 'dynamic:1',
  icon: '⚡'
}))

menuApi.addItem(submenuItem({
  id: 'dynamic-submenu',
  label: 'Dynamic Submenu',
  icon: '📁',
  items: [
    menuItem({ id: 'sub-1', label: 'Sub Item 1', action: 'sub:1' }),
    menuItem({ id: 'sub-2', label: 'Sub Item 2', action: 'sub:2' })
  ]
}))

// Navigation and selection
menuApi.nextItem()        // Move to next item
menuApi.previousItem()    // Move to previous item
menuApi.selectItem()      // Select current item
menuApi.enterSubmenu()    // Enter submenu if available
menuApi.exitSubmenu()     // Exit current submenu

// Item management
const foundItem = menuApi.getItem('dynamic-1')
console.log('Found item:', foundItem)

const removed = menuApi.removeItem('initial')
console.log('Item removed:', removed)

// State inspection
const currentState = menuApi.getState()
console.log('Menu state:', currentState)
```

## Menu Styling and Themes

### Custom Styling

```typescript
import { menu, menuItem } from 'reactive-tui-ts'

const styledMenu = menu({
  id: 'styled-menu',
  items: [
    menuItem({ id: 'item1', label: 'Styled Item 1', action: 'action:1' }),
    menuItem({ id: 'item2', label: 'Styled Item 2', action: 'action:2' }),
    menuItem({ id: 'item3', label: 'Styled Item 3', action: 'action:3' })
  ],
  style: {
    background: '#2d3748',
    foreground: '#f7fafc',
    hoverBackground: '#4a5568',
    hoverForeground: '#ffffff',
    selectedBackground: '#3182ce',
    selectedForeground: '#ffffff',
    disabledForeground: '#a0aec0',
    borderColor: '#718096',
    separatorColor: '#4a5568',
    padding: 2,
    itemHeight: 1.5,
    showBorders: true,
    showIcons: true,
    showShortcuts: true,
    submenuIndent: 3
  },
  cssClasses: ['custom-menu', 'dark-theme']
})
```

### Theme Integration

```typescript
// Different color themes for various contexts
const professionalMenu = menu({
  id: 'professional',
  items: [...],
  colorTheme: 'professional',
  style: { showBorders: true, padding: 2 }
})

const lightMenu = menu({
  id: 'light',
  items: [...],
  colorTheme: 'light',
  style: { showBorders: false, padding: 1 }
})

const darkMenu = menu({
  id: 'dark',
  items: [...],
  colorTheme: 'dark',
  style: { showBorders: true, padding: 2 }
})

const highContrastMenu = menu({
  id: 'high-contrast',
  items: [...],
  colorTheme: 'high-contrast',
  style: { showBorders: true, padding: 3 }
})
```

## Keyboard Navigation

### Navigation Keys

```typescript
import { menu, menuItem, submenuItem } from 'reactive-tui-ts'

const keyboardMenu = menu({
  id: 'keyboard-menu',
  items: [
    menuItem({ id: 'item1', label: 'First Item', action: 'action:1' }),
    menuItem({ id: 'item2', label: 'Second Item', action: 'action:2' }),
    submenuItem({
      id: 'submenu',
      label: 'Submenu',
      items: [
        menuItem({ id: 'sub1', label: 'Sub Item 1', action: 'sub:1' }),
        menuItem({ id: 'sub2', label: 'Sub Item 2', action: 'sub:2' })
      ]
    }),
    menuItem({ id: 'item3', label: 'Third Item', action: 'action:3' })
  ],
  focusable: true,
  onKeyPress: (key, event) => {
    console.log('Key pressed in menu:', key)
    
    const menuApi = keyboardMenu.api
    
    switch (key) {
      case 'ArrowDown':
        menuApi.nextItem()
        break
      case 'ArrowUp':
        menuApi.previousItem()
        break
      case 'ArrowRight':
      case 'Enter':
        menuApi.selectItem()
        break
      case 'ArrowLeft':
      case 'Escape':
        menuApi.exitSubmenu()
        break
      default:
        // Handle shortcut keys
        const currentState = menuApi.getState()
        const matchingItem = currentState.currentItems.find(item => 
          item.shortcut?.toLowerCase().includes(key.toLowerCase())
        )
        if (matchingItem) {
          menuApi.selectItem()
        }
    }
  }
})

// Keyboard navigation methods available:
// - nextItem(): Move selection down
// - previousItem(): Move selection up  
// - selectItem(): Activate current item
// - enterSubmenu(): Enter submenu if available
// - exitSubmenu(): Exit current submenu level
```

## Real-World Examples

### Code Editor Menu System

```typescript
import { menu, menuBar, menuItem, submenuItem, toggleItem, radioItem, separatorItem, headerItem } from 'reactive-tui-ts'

class CodeEditorMenus {
  private menus: Map<string, any> = new Map()
  private editorState = {
    hasUndo: true,
    hasRedo: false,
    hasSelection: true,
    isWordWrap: false,
    showLineNumbers: true,
    theme: 'dark',
    fontSize: 14
  }

  constructor() {
    this.setupMenus()
  }

  private setupMenus() {
    // Main menu bar
    this.menus.set('main', menuBar({
      id: 'main-menu-bar',
      items: [
        submenuItem({ id: 'file', label: 'File', items: this.createFileMenu() }),
        submenuItem({ id: 'edit', label: 'Edit', items: this.createEditMenu() }),
        submenuItem({ id: 'view', label: 'View', items: this.createViewMenu() }),
        submenuItem({ id: 'selection', label: 'Selection', items: this.createSelectionMenu() }),
        submenuItem({ id: 'go', label: 'Go', items: this.createGoMenu() }),
        submenuItem({ id: 'run', label: 'Run', items: this.createRunMenu() }),
        submenuItem({ id: 'terminal', label: 'Terminal', items: this.createTerminalMenu() }),
        submenuItem({ id: 'help', label: 'Help', items: this.createHelpMenu() })
      ],
      style: {
        showBorders: false,
        padding: 1,
        itemHeight: 1
      },
      colorTheme: 'professional',
      onItemSelect: (item) => this.handleMenuAction(item)
    }))

    // Right-click context menu
    this.menus.set('context', contextMenu({
      id: 'editor-context',
      items: [
        menuItem({ 
          id: 'cut', 
          label: 'Cut', 
          action: 'edit:cut', 
          icon: '✂️', 
          shortcut: 'Ctrl+X',
          enabled: this.editorState.hasSelection 
        }),
        menuItem({ 
          id: 'copy', 
          label: 'Copy', 
          action: 'edit:copy', 
          icon: '📋', 
          shortcut: 'Ctrl+C',
          enabled: this.editorState.hasSelection 
        }),
        menuItem({ id: 'paste', label: 'Paste', action: 'edit:paste', icon: '📄', shortcut: 'Ctrl+V' }),
        separatorItem('sep1'),
        
        submenuItem({
          id: 'refactor',
          label: 'Refactor',
          icon: '🔄',
          items: [
            menuItem({ id: 'rename', label: 'Rename Symbol', action: 'refactor:rename', shortcut: 'F2' }),
            menuItem({ id: 'extract-method', label: 'Extract Method', action: 'refactor:extract-method' }),
            menuItem({ id: 'extract-variable', label: 'Extract Variable', action: 'refactor:extract-variable' }),
            separatorItem('refactor-sep'),
            menuItem({ id: 'format', label: 'Format Document', action: 'format:document', shortcut: 'Shift+Alt+F' })
          ]
        }),
        
        separatorItem('sep2'),
        menuItem({ id: 'go-to-definition', label: 'Go to Definition', action: 'go:definition', shortcut: 'F12' }),
        menuItem({ id: 'go-to-references', label: 'Go to References', action: 'go:references', shortcut: 'Shift+F12' }),
        menuItem({ id: 'peek-definition', label: 'Peek Definition', action: 'peek:definition', shortcut: 'Alt+F12' })
      ],
      colorTheme: 'dark'
    }))
  }

  private createFileMenu(): MenuItem[] {
    return [
      menuItem({ id: 'new-file', label: 'New File', action: 'file:new', icon: '📄', shortcut: 'Ctrl+N' }),
      menuItem({ id: 'new-window', label: 'New Window', action: 'file:new-window', shortcut: 'Ctrl+Shift+N' }),
      separatorItem('file-sep1'),
      
      menuItem({ id: 'open-file', label: 'Open File...', action: 'file:open', icon: '📂', shortcut: 'Ctrl+O' }),
      menuItem({ id: 'open-folder', label: 'Open Folder...', action: 'file:open-folder', shortcut: 'Ctrl+K Ctrl+O' }),
      menuItem({ id: 'open-workspace', label: 'Open Workspace...', action: 'file:open-workspace' }),
      
      submenuItem({
        id: 'open-recent',
        label: 'Open Recent',
        icon: '📋',
        items: [
          menuItem({ id: 'recent-1', label: 'main.ts', action: 'file:recent:main.ts' }),
          menuItem({ id: 'recent-2', label: 'components/Button.tsx', action: 'file:recent:Button.tsx' }),
          menuItem({ id: 'recent-3', label: 'utils/helpers.js', action: 'file:recent:helpers.js' }),
          separatorItem('recent-sep'),
          menuItem({ id: 'clear-recent', label: 'Clear Recently Opened', action: 'file:clear-recent' })
        ]
      }),
      
      separatorItem('file-sep2'),
      menuItem({ id: 'save', label: 'Save', action: 'file:save', icon: '💾', shortcut: 'Ctrl+S' }),
      menuItem({ id: 'save-as', label: 'Save As...', action: 'file:save-as', shortcut: 'Ctrl+Shift+S' }),
      menuItem({ id: 'save-all', label: 'Save All', action: 'file:save-all', shortcut: 'Ctrl+K S' }),
      
      separatorItem('file-sep3'),
      menuItem({ id: 'close-editor', label: 'Close Editor', action: 'file:close', shortcut: 'Ctrl+W' }),
      menuItem({ id: 'close-folder', label: 'Close Folder', action: 'file:close-folder', shortcut: 'Ctrl+K F' }),
      menuItem({ id: 'close-window', label: 'Close Window', action: 'file:close-window', shortcut: 'Alt+F4' }),
      
      separatorItem('file-sep4'),
      menuItem({ id: 'exit', label: 'Exit', action: 'app:exit', icon: '🚪' })
    ]
  }

  private createEditMenu(): MenuItem[] {
    return [
      menuItem({ 
        id: 'undo', 
        label: 'Undo', 
        action: 'edit:undo', 
        shortcut: 'Ctrl+Z',
        enabled: this.editorState.hasUndo 
      }),
      menuItem({ 
        id: 'redo', 
        label: 'Redo', 
        action: 'edit:redo', 
        shortcut: 'Ctrl+Y',
        enabled: this.editorState.hasRedo 
      }),
      separatorItem('edit-sep1'),
      
      menuItem({ 
        id: 'cut', 
        label: 'Cut', 
        action: 'edit:cut', 
        shortcut: 'Ctrl+X',
        enabled: this.editorState.hasSelection 
      }),
      menuItem({ 
        id: 'copy', 
        label: 'Copy', 
        action: 'edit:copy', 
        shortcut: 'Ctrl+C',
        enabled: this.editorState.hasSelection 
      }),
      menuItem({ id: 'paste', label: 'Paste', action: 'edit:paste', shortcut: 'Ctrl+V' }),
      
      separatorItem('edit-sep2'),
      menuItem({ id: 'find', label: 'Find', action: 'edit:find', icon: '🔍', shortcut: 'Ctrl+F' }),
      menuItem({ id: 'replace', label: 'Replace', action: 'edit:replace', shortcut: 'Ctrl+H' }),
      menuItem({ id: 'find-in-files', label: 'Find in Files', action: 'edit:find-files', shortcut: 'Ctrl+Shift+F' }),
      menuItem({ id: 'replace-in-files', label: 'Replace in Files', action: 'edit:replace-files', shortcut: 'Ctrl+Shift+H' }),
      
      separatorItem('edit-sep3'),
      menuItem({ id: 'select-all', label: 'Select All', action: 'edit:select-all', shortcut: 'Ctrl+A' }),
      menuItem({ id: 'expand-selection', label: 'Expand Selection', action: 'edit:expand-selection', shortcut: 'Shift+Alt+Right' }),
      menuItem({ id: 'shrink-selection', label: 'Shrink Selection', action: 'edit:shrink-selection', shortcut: 'Shift+Alt+Left' })
    ]
  }

  private createViewMenu(): MenuItem[] {
    return [
      toggleItem({ 
        id: 'command-palette', 
        label: 'Command Palette', 
        state: false, 
        shortcut: 'Ctrl+Shift+P' 
      }),
      separatorItem('view-sep1'),
      
      headerItem({ id: 'appearance-header', label: 'Appearance' }),
      toggleItem({ 
        id: 'full-screen', 
        label: 'Full Screen', 
        state: false, 
        shortcut: 'F11' 
      }),
      toggleItem({ 
        id: 'zen-mode', 
        label: 'Zen Mode', 
        state: false, 
        shortcut: 'Ctrl+K Z' 
      }),
      toggleItem({ 
        id: 'centered-layout', 
        label: 'Centered Layout', 
        state: false 
      }),
      
      separatorItem('view-sep2'),
      headerItem({ id: 'editor-layout-header', label: 'Editor Layout' }),
      radioItem({ 
        id: 'split-up', 
        label: 'Split Up', 
        group: 'editor-layout', 
        shortcut: 'Ctrl+K Ctrl+\\' 
      }),
      radioItem({ 
        id: 'split-down', 
        label: 'Split Down', 
        group: 'editor-layout' 
      }),
      radioItem({ 
        id: 'split-left', 
        label: 'Split Left', 
        group: 'editor-layout' 
      }),
      radioItem({ 
        id: 'split-right', 
        label: 'Split Right', 
        group: 'editor-layout', 
        selected: true 
      }),
      
      separatorItem('view-sep3'),
      headerItem({ id: 'editor-features-header', label: 'Editor Features' }),
      toggleItem({ 
        id: 'word-wrap', 
        label: 'Word Wrap', 
        state: this.editorState.isWordWrap, 
        shortcut: 'Alt+Z' 
      }),
      toggleItem({ 
        id: 'line-numbers', 
        label: 'Line Numbers', 
        state: this.editorState.showLineNumbers 
      }),
      toggleItem({ 
        id: 'minimap', 
        label: 'Minimap', 
        state: true, 
        shortcut: 'Ctrl+Shift+M' 
      }),
      toggleItem({ 
        id: 'breadcrumbs', 
        label: 'Breadcrumbs', 
        state: true 
      }),
      
      separatorItem('view-sep4'),
      headerItem({ id: 'zoom-header', label: 'Zoom' }),
      menuItem({ id: 'zoom-in', label: 'Zoom In', action: 'view:zoom-in', shortcut: 'Ctrl+=' }),
      menuItem({ id: 'zoom-out', label: 'Zoom Out', action: 'view:zoom-out', shortcut: 'Ctrl+-' }),
      menuItem({ id: 'reset-zoom', label: 'Reset Zoom', action: 'view:reset-zoom', shortcut: 'Ctrl+0' })
    ]
  }

  private createSelectionMenu(): MenuItem[] {
    return [
      menuItem({ id: 'select-all', label: 'Select All', action: 'selection:all', shortcut: 'Ctrl+A' }),
      menuItem({ id: 'expand-selection', label: 'Expand Selection', action: 'selection:expand', shortcut: 'Shift+Alt+Right' }),
      menuItem({ id: 'shrink-selection', label: 'Shrink Selection', action: 'selection:shrink', shortcut: 'Shift+Alt+Left' }),
      separatorItem('selection-sep1'),
      
      menuItem({ id: 'select-line', label: 'Select Line', action: 'selection:line', shortcut: 'Ctrl+L' }),
      menuItem({ id: 'select-word', label: 'Select Word', action: 'selection:word', shortcut: 'Ctrl+D' }),
      menuItem({ id: 'select-all-occurrences', label: 'Select All Occurrences', action: 'selection:all-occurrences', shortcut: 'Ctrl+Shift+L' }),
      
      separatorItem('selection-sep2'),
      menuItem({ id: 'add-cursor-above', label: 'Add Cursor Above', action: 'selection:cursor-above', shortcut: 'Ctrl+Alt+Up' }),
      menuItem({ id: 'add-cursor-below', label: 'Add Cursor Below', action: 'selection:cursor-below', shortcut: 'Ctrl+Alt+Down' }),
      menuItem({ id: 'add-cursors-to-line-ends', label: 'Add Cursors to Line Ends', action: 'selection:cursors-line-ends', shortcut: 'Shift+Alt+I' })
    ]
  }

  private createGoMenu(): MenuItem[] {
    return [
      menuItem({ id: 'go-to-file', label: 'Go to File...', action: 'go:file', shortcut: 'Ctrl+P' }),
      menuItem({ id: 'go-to-symbol-workspace', label: 'Go to Symbol in Workspace...', action: 'go:symbol-workspace', shortcut: 'Ctrl+T' }),
      menuItem({ id: 'go-to-symbol-editor', label: 'Go to Symbol in Editor...', action: 'go:symbol-editor', shortcut: 'Ctrl+Shift+O' }),
      menuItem({ id: 'go-to-line', label: 'Go to Line/Column...', action: 'go:line', shortcut: 'Ctrl+G' }),
      
      separatorItem('go-sep1'),
      menuItem({ id: 'go-back', label: 'Go Back', action: 'go:back', shortcut: 'Alt+Left' }),
      menuItem({ id: 'go-forward', label: 'Go Forward', action: 'go:forward', shortcut: 'Alt+Right' }),
      menuItem({ id: 'go-last-edit', label: 'Go to Last Edit Location', action: 'go:last-edit', shortcut: 'Ctrl+K Ctrl+Q' }),
      
      separatorItem('go-sep2'),
      menuItem({ id: 'go-definition', label: 'Go to Definition', action: 'go:definition', shortcut: 'F12' }),
      menuItem({ id: 'go-declaration', label: 'Go to Declaration', action: 'go:declaration' }),
      menuItem({ id: 'go-type-definition', label: 'Go to Type Definition', action: 'go:type-definition' }),
      menuItem({ id: 'go-implementation', label: 'Go to Implementation', action: 'go:implementation', shortcut: 'Ctrl+F12' }),
      menuItem({ id: 'go-references', label: 'Go to References', action: 'go:references', shortcut: 'Shift+F12' })
    ]
  }

  private createRunMenu(): MenuItem[] {
    return [
      menuItem({ id: 'run-file', label: 'Run File', action: 'run:file', icon: '▶️', shortcut: 'Ctrl+F5' }),
      menuItem({ id: 'run-selection', label: 'Run Selection', action: 'run:selection', shortcut: 'F8' }),
      separatorItem('run-sep1'),
      
      menuItem({ id: 'debug-file', label: 'Debug File', action: 'debug:file', icon: '🐛', shortcut: 'F5' }),
      menuItem({ id: 'toggle-breakpoint', label: 'Toggle Breakpoint', action: 'debug:toggle-breakpoint', shortcut: 'F9' }),
      menuItem({ id: 'step-over', label: 'Step Over', action: 'debug:step-over', shortcut: 'F10' }),
      menuItem({ id: 'step-into', label: 'Step Into', action: 'debug:step-into', shortcut: 'F11' }),
      menuItem({ id: 'step-out', label: 'Step Out', action: 'debug:step-out', shortcut: 'Shift+F11' }),
      
      separatorItem('run-sep2'),
      menuItem({ id: 'start-debugging', label: 'Start Debugging', action: 'debug:start', shortcut: 'F5' }),
      menuItem({ id: 'stop-debugging', label: 'Stop Debugging', action: 'debug:stop', shortcut: 'Shift+F5' }),
      menuItem({ id: 'restart-debugging', label: 'Restart Debugging', action: 'debug:restart', shortcut: 'Ctrl+Shift+F5' })
    ]
  }

  private createTerminalMenu(): MenuItem[] {
    return [
      menuItem({ id: 'new-terminal', label: 'New Terminal', action: 'terminal:new', icon: '💻', shortcut: 'Ctrl+Shift+`' }),
      menuItem({ id: 'split-terminal', label: 'Split Terminal', action: 'terminal:split', shortcut: 'Ctrl+Shift+5' }),
      separatorItem('terminal-sep1'),
      
      menuItem({ id: 'run-task', label: 'Run Task...', action: 'terminal:run-task', shortcut: 'Ctrl+Shift+P' }),
      menuItem({ id: 'run-build-task', label: 'Run Build Task...', action: 'terminal:build-task', shortcut: 'Ctrl+Shift+B' }),
      menuItem({ id: 'run-test-task', label: 'Run Test Task...', action: 'terminal:test-task' }),
      
      separatorItem('terminal-sep2'),
      toggleItem({ id: 'show-terminal', label: 'Show Terminal', state: true, shortcut: 'Ctrl+`' }),
      menuItem({ id: 'focus-terminal', label: 'Focus Terminal', action: 'terminal:focus', shortcut: 'Ctrl+Shift+`' }),
      menuItem({ id: 'kill-terminal', label: 'Kill Terminal', action: 'terminal:kill' })
    ]
  }

  private createHelpMenu(): MenuItem[] {
    return [
      menuItem({ id: 'welcome', label: 'Welcome', action: 'help:welcome' }),
      menuItem({ id: 'documentation', label: 'Documentation', action: 'help:docs', shortcut: 'F1' }),
      menuItem({ id: 'keyboard-shortcuts', label: 'Keyboard Shortcuts Reference', action: 'help:shortcuts', shortcut: 'Ctrl+K Ctrl+R' }),
      separatorItem('help-sep1'),
      
      menuItem({ id: 'view-license', label: 'View License', action: 'help:license' }),
      menuItem({ id: 'privacy-statement', label: 'Privacy Statement', action: 'help:privacy' }),
      separatorItem('help-sep2'),
      
      menuItem({ id: 'check-for-updates', label: 'Check for Updates...', action: 'help:updates' }),
      menuItem({ id: 'release-notes', label: 'Release Notes', action: 'help:release-notes' }),
      separatorItem('help-sep3'),
      
      menuItem({ id: 'report-issue', label: 'Report Issue', action: 'help:report-issue' }),
      menuItem({ id: 'join-community', label: 'Join our Community', action: 'help:community' }),
      separatorItem('help-sep4'),
      
      menuItem({ id: 'about', label: 'About', action: 'help:about' })
    ]
  }

  private handleMenuAction(item: MenuItem) {
    const action = item.itemType.action
    console.log(`Menu action triggered: ${action}`)
    
    // Handle state updates for toggle items
    if (item.itemType.type === 'toggle') {
      switch (item.id) {
        case 'word-wrap':
          this.editorState.isWordWrap = !this.editorState.isWordWrap
          break
        case 'line-numbers':
          this.editorState.showLineNumbers = !this.editorState.showLineNumbers
          break
      }
    }
    
    // Handle radio button selections
    if (item.itemType.type === 'radio') {
      if (item.itemType.group === 'editor-layout') {
        console.log(`Editor layout changed to: ${item.label}`)
      }
    }
    
    // Execute action-specific logic
    switch (action) {
      case 'file:save':
        console.log('Saving file...')
        break
      case 'edit:undo':
        if (this.editorState.hasUndo) {
          console.log('Performing undo...')
          this.editorState.hasRedo = true
        }
        break
      case 'edit:redo':
        if (this.editorState.hasRedo) {
          console.log('Performing redo...')
        }
        break
      case 'view:zoom-in':
        this.editorState.fontSize = Math.min(this.editorState.fontSize + 2, 32)
        console.log(`Font size: ${this.editorState.fontSize}px`)
        break
      case 'view:zoom-out':
        this.editorState.fontSize = Math.max(this.editorState.fontSize - 2, 8)
        console.log(`Font size: ${this.editorState.fontSize}px`)
        break
      case 'view:reset-zoom':
        this.editorState.fontSize = 14
        console.log('Font size reset to 14px')
        break
      default:
        console.log(`Action not implemented: ${action}`)
    }
  }

  getMenu(menuId: string): any {
    return this.menus.get(menuId)
  }

  getAllMenus(): Map<string, any> {
    return this.menus
  }

  updateEditorState(updates: Partial<typeof this.editorState>) {
    Object.assign(this.editorState, updates)
    
    // Update menu items that depend on editor state
    this.refreshMenuStates()
  }

  private refreshMenuStates() {
    // In a real implementation, this would update the menu items
    // based on the current editor state
    console.log('Refreshing menu states based on editor state:', this.editorState)
  }

  showContextMenu(x: number, y: number) {
    const contextMenu = this.menus.get('context')
    if (contextMenu) {
      // Position and show context menu
      console.log(`Showing context menu at (${x}, ${y})`)
      return contextMenu
    }
  }

  hideContextMenu() {
    console.log('Hiding context menu')
  }
}

// Usage
const editorMenus = new CodeEditorMenus()

// Get the main menu bar
const mainMenuBar = editorMenus.getMenu('main')
console.log('Main menu bar created')

// Simulate editor state changes
editorMenus.updateEditorState({
  hasSelection: true,
  isWordWrap: true,
  theme: 'light'
})

// Show context menu on right-click
const contextMenu = editorMenus.showContextMenu(150, 200)

// Handle keyboard shortcuts
document.addEventListener('keydown', (event) => {
  const key = event.key
  const ctrl = event.ctrlKey
  const shift = event.shiftKey
  const alt = event.altKey
  
  // Example shortcut handling
  if (ctrl && key === 's') {
    event.preventDefault()
    editorMenus.handleMenuAction(menuItem({ 
      id: 'save', 
      label: 'Save', 
      action: 'file:save' 
    }))
  }
  
  if (ctrl && key === 'z') {
    event.preventDefault()
    editorMenus.handleMenuAction(menuItem({ 
      id: 'undo', 
      label: 'Undo', 
      action: 'edit:undo' 
    }))
  }
})

console.log('Code editor menu system initialized')
```

### System Tray Menu

```typescript
import { contextMenu, menuItem, submenuItem, toggleItem, separatorItem, headerItem } from 'reactive-tui-ts'

class SystemTrayMenu {
  private isOnline: boolean = true
  private notifications: boolean = true
  private autoStart: boolean = true
  private syncEnabled: boolean = true

  createTrayMenu() {
    return contextMenu({
      id: 'system-tray',
      items: [
        headerItem({ id: 'status-header', label: 'Application Status' }),
        menuItem({ 
          id: 'status', 
          label: this.isOnline ? '🟢 Online' : '🔴 Offline', 
          action: 'status:toggle' 
        }),
        menuItem({ 
          id: 'sync-status', 
          label: this.syncEnabled ? '🔄 Sync Active' : '⏸️ Sync Paused', 
          action: 'sync:toggle' 
        }),
        
        separatorItem('tray-sep1'),
        
        menuItem({ id: 'open-app', label: 'Open Application', action: 'app:open', icon: '🚀' }),
        menuItem({ id: 'dashboard', label: 'Dashboard', action: 'app:dashboard', icon: '📊' }),
        
        separatorItem('tray-sep2'),
        
        submenuItem({
          id: 'settings',
          label: 'Settings',
          icon: '⚙️',
          items: [
            toggleItem({ 
              id: 'notifications', 
              label: 'Enable Notifications', 
              state: this.notifications 
            }),
            toggleItem({ 
              id: 'auto-start', 
              label: 'Start with System', 
              state: this.autoStart 
            }),
            toggleItem({ 
              id: 'auto-sync', 
              label: 'Auto Sync', 
              state: this.syncEnabled 
            }),
            separatorItem('settings-sep'),
            menuItem({ id: 'preferences', label: 'Preferences...', action: 'app:preferences' }),
            menuItem({ id: 'account', label: 'Account Settings...', action: 'app:account' })
          ]
        }),
        
        submenuItem({
          id: 'recent-activity',
          label: 'Recent Activity',
          icon: '📋',
          items: [
            menuItem({ id: 'activity-1', label: 'Document synced (2 min ago)', action: 'activity:1' }),
            menuItem({ id: 'activity-2', label: 'Backup completed (15 min ago)', action: 'activity:2' }),
            menuItem({ id: 'activity-3', label: 'Settings updated (1 hour ago)', action: 'activity:3' }),
            separatorItem('activity-sep'),
            menuItem({ id: 'view-all-activity', label: 'View All Activity...', action: 'activity:view-all' })
          ]
        }),
        
        separatorItem('tray-sep3'),
        
        menuItem({ id: 'help', label: 'Help & Support', action: 'help:support', icon: '❓' }),
        menuItem({ id: 'about', label: 'About', action: 'help:about', icon: 'ℹ️' }),
        
        separatorItem('tray-sep4'),
        
        menuItem({ id: 'quit', label: 'Quit Application', action: 'app:quit', icon: '🚪' })
      ],
      style: {
        showBorders: true,
        padding: 1,
        itemHeight: 1.2,
        showIcons: true
      },
      colorTheme: 'light',
      onItemSelect: (item) => this.handleTrayAction(item)
    })
  }

  private handleTrayAction(item: MenuItem) {
    const action = item.itemType.action
    
    switch (action) {
      case 'status:toggle':
        this.isOnline = !this.isOnline
        console.log(`Status changed to: ${this.isOnline ? 'Online' : 'Offline'}`)
        break
      
      case 'sync:toggle':
        this.syncEnabled = !this.syncEnabled
        console.log(`Sync ${this.syncEnabled ? 'enabled' : 'disabled'}`)
        break
      
      case 'app:open':
        console.log('Opening main application window')
        break
      
      case 'app:quit':
        console.log('Quitting application')
        break
      
      default:
        console.log(`Tray action: ${action}`)
    }
    
    // Handle toggle states
    if (item.itemType.type === 'toggle') {
      switch (item.id) {
        case 'notifications':
          this.notifications = !this.notifications
          console.log(`Notifications ${this.notifications ? 'enabled' : 'disabled'}`)
          break
        case 'auto-start':
          this.autoStart = !this.autoStart
          console.log(`Auto-start ${this.autoStart ? 'enabled' : 'disabled'}`)
          break
        case 'auto-sync':
          this.syncEnabled = !this.syncEnabled
          console.log(`Auto-sync ${this.syncEnabled ? 'enabled' : 'disabled'}`)
          break
      }
    }
  }
}

// Usage
const trayMenu = new SystemTrayMenu()
const menu = trayMenu.createTrayMenu()

// Simulate tray icon click
console.log('System tray menu created:', menu.id)
```

## CSS Styling

```css
/* Menu base styles */
.menu {
  position: relative;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  user-select: none;
  outline: none;
}

.menu-vertical {
  display: flex;
  flex-direction: column;
  min-width: 200px;
}

.menu-horizontal {
  display: flex;
  flex-direction: row;
  align-items: center;
}

.menu-focusable:focus {
  outline: 2px solid #4f46e5;
  outline-offset: 1px;
}

/* Menu items */
.menu-item {
  display: flex;
  align-items: center;
  padding: 0.5rem 1rem;
  cursor: pointer;
  transition: all 0.15s ease;
  border: none;
  background: transparent;
  text-align: left;
  width: 100%;
  min-height: 2rem;
}

.menu-item:hover:not(.menu-item-disabled) {
  background-color: var(--menu-hover-background, #4682B4);
  color: var(--menu-hover-foreground, #FFFFFF);
}

.menu-item-selected {
  background-color: var(--menu-selected-background, #6495ED);
  color: var(--menu-selected-foreground, #FFFFFF);
}

.menu-item-disabled {
  opacity: 0.6;
  cursor: not-allowed;
  color: var(--menu-disabled-foreground, #808080);
}

/* Item types */
.menu-item-action:after {
  content: '';
  margin-left: auto;
}

.menu-item-submenu:after {
  content: '▶';
  margin-left: auto;
  font-size: 0.75em;
  opacity: 0.7;
}

.menu-item-toggle:before {
  content: '☐';
  margin-right: 0.5rem;
  font-size: 0.875em;
}

.menu-item-toggle[data-checked="true"]:before {
  content: '☑';
}

.menu-item-radio:before {
  content: '○';
  margin-right: 0.5rem;
  font-size: 0.875em;
}

.menu-item-radio[data-selected="true"]:before {
  content: '●';
}

/* Headers and separators */
.menu-header {
  font-weight: bold;
  font-size: 0.875rem;
  color: var(--menu-header-color, #6b7280);
  padding: 0.25rem 1rem;
  text-transform: uppercase;
  letter-spacing: 0.025em;
}

.menu-separator {
  height: 1px;
  background-color: var(--menu-separator-color, #e5e7eb);
  margin: 0.25rem 0;
  border: none;
}

/* Shortcuts */
.menu-shortcut {
  margin-left: auto;
  font-size: 0.75rem;
  opacity: 0.7;
  padding-left: 1rem;
}

/* Icons */
.menu-icon {
  margin-right: 0.75rem;
  font-size: 1rem;
  width: 1rem;
  text-align: center;
}

/* Borders and backgrounds */
.menu[data-show-borders="true"] {
  border: 1px solid var(--menu-border-color, #d1d5db);
  border-radius: 0.375rem;
  background: var(--menu-background, #ffffff);
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1);
}

/* Theme support */
.menu[data-color-theme="light"] {
  background: #ffffff;
  color: #1f2937;
  border-color: #e5e7eb;
}

.menu[data-color-theme="dark"] {
  background: #1f2937;
  color: #f9fafb;
  border-color: #4b5563;
}

.menu[data-color-theme="dark"] .menu-separator {
  background-color: #4b5563;
}

.menu[data-color-theme="professional"] {
  background: #f8fafc;
  color: #334155;
  border-color: #cbd5e1;
}

.menu[data-color-theme="high-contrast"] {
  background: #000000;
  color: #ffffff;
  border-color: #ffffff;
  border-width: 2px;
}

/* Responsive design */
@media (max-width: 768px) {
  .menu-item {
    padding: 0.75rem 1rem;
    min-height: 2.5rem;
  }
  
  .menu-shortcut {
    display: none;
  }
  
  .menu {
    min-width: 180px;
  }
}

/* Animation support */
.menu-item {
  transition: background-color 0.15s ease, color 0.15s ease;
}

/* Accessibility */
.menu:focus-within .menu-item-selected {
  outline: 2px solid #4f46e5;
  outline-offset: -2px;
}

@media (prefers-reduced-motion: reduce) {
  .menu-item {
    transition: none;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .menu {
    border-width: 2px;
    border-color: currentColor;
  }
  
  .menu-item:hover:not(.menu-item-disabled) {
    outline: 2px solid currentColor;
    outline-offset: -2px;
  }
}

/* Print styles */
@media print {
  .menu {
    border: 1px solid #000000;
    background: #ffffff;
    color: #000000;
    box-shadow: none;
  }
}
```

## Best Practices

### 1. Logical Menu Organization

```typescript
// ✅ Good - logical grouping with headers and separators
const wellOrganizedMenu = menu({
  id: 'organized',
  items: [
    headerItem({ id: 'file-ops', label: 'File Operations' }),
    menuItem({ id: 'new', label: 'New', action: 'file:new' }),
    menuItem({ id: 'open', label: 'Open', action: 'file:open' }),
    menuItem({ id: 'save', label: 'Save', action: 'file:save' }),
    
    separatorItem('sep1'),
    
    headerItem({ id: 'edit-ops', label: 'Edit Operations' }),
    menuItem({ id: 'undo', label: 'Undo', action: 'edit:undo' }),
    menuItem({ id: 'redo', label: 'Redo', action: 'edit:redo' })
  ]
})

// ❌ Poor - no organization, mixed functionality
const poorMenu = menu({
  id: 'poor',
  items: [
    menuItem({ id: 'new', label: 'New', action: 'file:new' }),
    menuItem({ id: 'undo', label: 'Undo', action: 'edit:undo' }),
    menuItem({ id: 'about', label: 'About', action: 'help:about' }),
    menuItem({ id: 'save', label: 'Save', action: 'file:save' })
  ]
})
```

### 2. Consistent Shortcut Usage

```typescript
// ✅ Good - standard shortcuts
const standardShortcuts = menu({
  id: 'standard',
  items: [
    menuItem({ id: 'new', label: 'New', shortcut: 'Ctrl+N' }),
    menuItem({ id: 'open', label: 'Open', shortcut: 'Ctrl+O' }),  
    menuItem({ id: 'save', label: 'Save', shortcut: 'Ctrl+S' }),
    menuItem({ id: 'copy', label: 'Copy', shortcut: 'Ctrl+C' }),
    menuItem({ id: 'paste', label: 'Paste', shortcut: 'Ctrl+V' })
  ]
})

// ❌ Poor - non-standard, confusing shortcuts  
const confusingShortcuts = menu({
  id: 'confusing',
  items: [
    menuItem({ id: 'new', label: 'New', shortcut: 'Ctrl+Q' }),
    menuItem({ id: 'open', label: 'Open', shortcut: 'Alt+F7' }),
    menuItem({ id: 'save', label: 'Save', shortcut: 'Shift+Ctrl+Alt+S' })
  ]
})
```

### 3. Appropriate State Management

```typescript
// ✅ Good - proper toggle and radio usage
const stateMenu = menu({
  id: 'state-demo',
  items: [
    // Toggle for independent boolean states
    toggleItem({ id: 'word-wrap', label: 'Word Wrap', state: false }),
    toggleItem({ id: 'line-numbers', label: 'Line Numbers', state: true }),
    
    separatorItem('sep1'),
    
    // Radio group for mutually exclusive options
    radioItem({ id: 'theme-light', label: 'Light Theme', group: 'theme', selected: true }),
    radioItem({ id: 'theme-dark', label: 'Dark Theme', group: 'theme' }),
    radioItem({ id: 'theme-auto', label: 'Auto Theme', group: 'theme' })
  ]
})

// ❌ Poor - using toggles for mutually exclusive options
const poorStateMenu = menu({
  id: 'poor-state',
  items: [
    toggleItem({ id: 'theme-light', label: 'Light Theme', state: true }),
    toggleItem({ id: 'theme-dark', label: 'Dark Theme', state: false }),
    toggleItem({ id: 'theme-auto', label: 'Auto Theme', state: false })
  ]
})
```

### 4. Meaningful Icons and Labels

```typescript
// ✅ Good - clear, descriptive labels with appropriate icons
const clearMenu = menu({
  id: 'clear',
  items: [
    menuItem({ id: 'new-file', label: 'New File', icon: '📄', action: 'file:new' }),
    menuItem({ id: 'open-folder', label: 'Open Folder', icon: '📂', action: 'folder:open' }),
    menuItem({ id: 'save-all', label: 'Save All Files', icon: '💾', action: 'file:save-all' }),
    separatorItem('sep1'),
    menuItem({ id: 'find-files', label: 'Find in Files', icon: '🔍', action: 'search:files' })
  ]
})

// ❌ Poor - vague labels, inappropriate icons
const vagueMenu = menu({
  id: 'vague',
  items: [
    menuItem({ id: 'do-thing', label: 'Do Thing', icon: '❓', action: 'thing:do' }),
    menuItem({ id: 'open-stuff', label: 'Open Stuff', icon: '🎉', action: 'stuff:open' }),
    menuItem({ id: 'action', label: 'Action', icon: '🔥', action: 'generic:action' })
  ]
})
```

### 5. Proper Submenu Structure

```typescript
// ✅ Good - logical submenu hierarchy
const hierarchicalMenu = menu({
  id: 'hierarchical',
  items: [
    submenuItem({
      id: 'file',
      label: 'File',
      items: [
        menuItem({ id: 'new', label: 'New', action: 'file:new' }),
        submenuItem({
          id: 'new-from-template',
          label: 'New from Template',
          items: [
            menuItem({ id: 'blank', label: 'Blank Document', action: 'template:blank' }),
            menuItem({ id: 'article', label: 'Article Template', action: 'template:article' }),
            menuItem({ id: 'report', label: 'Report Template', action: 'template:report' })
          ]
        }),
        separatorItem('file-sep'),
        menuItem({ id: 'open', label: 'Open', action: 'file:open' })
      ]
    })
  ]
})

// ❌ Poor - too deep nesting, unclear hierarchy
const deepMenu = menu({
  id: 'deep',
  items: [
    submenuItem({
      id: 'level1',
      label: 'Level 1',
      items: [
        submenuItem({
          id: 'level2',
          label: 'Level 2',
          items: [
            submenuItem({
              id: 'level3',
              label: 'Level 3',
              items: [
                menuItem({ id: 'deep-action', label: 'Deep Action', action: 'deep:action' })
              ]
            })
          ]
        })
      ]
    })
  ]
})
```

## Related Widgets

- **[Button](./button)** - Interactive buttons for menu triggers
- **[Tabs](./tabs)** - Tab-based navigation alternative
- **[Accordion](./accordion)** - Expandable menu-like structures
- **[Modal](./modal)** - Dialog-based menus and overlays

## Examples

- **[Basic Menus](../../examples/basic/menu-basic)** - Simple menu implementations
- **[Context Menus](../../examples/advanced/menu-context)** - Right-click and contextual menus
- **[Menu Bars](../../examples/apps/menu-bar)** - Application menu bars
- **[Dynamic Menus](../../examples/advanced/menu-dynamic)** - JSON-configurable menus

The Menu widget provides comprehensive navigation functionality with support for complex hierarchies, state management, keyboard navigation, and dynamic configuration, making it essential for building sophisticated application interfaces.