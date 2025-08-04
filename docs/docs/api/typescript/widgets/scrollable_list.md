# ScrollableList Widget

Comprehensive vertical scrolling list with efficient rendering, keyboard navigation, multi-select support, and search functionality for handling large datasets with smooth performance.

## Overview

The ScrollableList widget provides advanced list functionality with smooth scrolling, keyboard navigation (including vim-style), selection modes, search integration, and virtual rendering for large datasets. Perfect for file browsers, menus, task lists, and data exploration interfaces.

```typescript
import { ScrollableList, ScrollableListBuilder, SelectionMode } from 'reactive-tui-ts'

const fileList = new ScrollableListBuilder('file-browser')
  .items([
    { id: '1', text: 'package.json', subtitle: '1.2KB', icon: '📄' },
    { id: '2', text: 'src/', subtitle: 'directory', icon: '📁' },
    { id: '3', text: 'README.md', subtitle: '3.4KB', icon: '📄' }
  ])
  .height(10)
  .selectionMode(SelectionMode.Single)
  .showScrollbar(true)
  .searchEnabled(true)
  .onItemActivate((itemId, item) => console.log('Opened:', item.text))
  .build()
```

## Types

### ListItem

```typescript
interface ListItem {
  id: string
  text: string
  subtitle?: string
  icon?: string
  metadata?: Record<string, any>
  disabled?: boolean
  css_classes?: string[]
}
```

### ScrollableListConfig

```typescript
interface ScrollableListConfig {
  height: number
  selection_mode: SelectionMode
  show_scrollbar: boolean
  show_icons: boolean
  show_subtitles: boolean
  search_enabled: boolean
  vim_navigation: boolean
  auto_scroll: boolean
  smooth_scrolling: boolean
  scroll_step: number
  item_height: number
  padding: number
  border_width: number
}
```

### SelectionMode

```typescript
enum SelectionMode {
  Single = 'single',
  Multiple = 'multiple',
  None = 'none'
}
```

### ScrollableListState

```typescript
interface ScrollableListState {
  scroll_position: number
  highlighted_index: number | null
  selected_items: string[]
  is_focused: boolean
  search_query: string
  filtered_indices: number[]
  total_items: number
  visible_items: number
  search_active: boolean
}
```

### ScrollableListCallbacks

```typescript
interface ScrollableListCallbacks {
  onSelectionChange?: (selectedItems: string[], list: ScrollableList) => void
  onItemActivate?: (itemId: string, item: ListItem, list: ScrollableList) => void
  onHighlightChange?: (itemId: string | null, list: ScrollableList) => void
  onScroll?: (position: number, maxScroll: number, list: ScrollableList) => void
  onSearchChange?: (query: string, results: number, list: ScrollableList) => void
  onFocusChange?: (focused: boolean, list: ScrollableList) => void
}
```

## Basic Usage

### Simple List

```typescript
import { ScrollableList, ScrollableListBuilder, SelectionMode } from 'reactive-tui-ts'

const simpleList = new ScrollableListBuilder('simple-list')
  .items([
    { id: '1', text: 'First Item' },
    { id: '2', text: 'Second Item' },
    { id: '3', text: 'Third Item' },
    { id: '4', text: 'Fourth Item' },
    { id: '5', text: 'Fifth Item' }
  ])
  .height(3)
  .selectionMode(SelectionMode.Single)
  .onSelectionChange((selected) => {
    console.log('Selected items:', selected)
  })
  .build()

// Navigate and select
simpleList.selectNext()           // Move down
simpleList.selectPrevious()       // Move up
simpleList.selectFirst()          // Jump to first
simpleList.selectLast()           // Jump to last

// Get current state
console.log('Highlighted:', simpleList.getHighlightedItem())
console.log('Selected:', simpleList.getSelectedItems())
```

### List with Icons and Subtitles

```typescript
const richList = new ScrollableListBuilder('rich-list')
  .items([
    { 
      id: 'home', 
      text: 'Home Directory', 
      subtitle: '/home/user', 
      icon: '🏠' 
    },
    { 
      id: 'documents', 
      text: 'Documents', 
      subtitle: '45 files', 
      icon: '📁' 
    },
    { 
      id: 'downloads', 
      text: 'Downloads', 
      subtitle: '12 files', 
      icon: '📥' 
    },
    { 
      id: 'pictures', 
      text: 'Pictures', 
      subtitle: '128 files', 
      icon: '🖼️' 
    }
  ])
  .height(5)
  .showIcons(true)
  .showSubtitles(true)
  .showScrollbar(true)
  .onItemActivate((itemId, item) => {
    console.log(`Navigating to: ${item.text}`)
  })
  .build()

// Render the list
const renderedLines = richList.render()
renderedLines.forEach(line => console.log(line))
```

## Selection Modes

### Single Selection

```typescript
const singleSelectList = new ScrollableListBuilder('single-select')
  .items([
    { id: '1', text: 'Option A' },
    { id: '2', text: 'Option B' },
    { id: '3', text: 'Option C' }
  ])
  .selectionMode(SelectionMode.Single)
  .height(5)
  .onSelectionChange((selected, list) => {
    console.log('Single selection:', selected[0] || 'none')
  })
  .build()

// Select items programmatically
singleSelectList.selectItem('2')
console.log('Selected:', singleSelectList.getSelectedItems())
// Output: ['2']
```

### Multiple Selection

```typescript
const multiSelectList = new ScrollableListBuilder('multi-select')
  .items([
    { id: 'file1', text: 'document.txt', icon: '📄' },
    { id: 'file2', text: 'image.png', icon: '🖼️' },
    { id: 'file3', text: 'video.mp4', icon: '🎥' },
    { id: 'file4', text: 'audio.mp3', icon: '🎵' }
  ])
  .selectionMode(SelectionMode.Multiple)
  .height(6)
  .showIcons(true)
  .onSelectionChange((selected, list) => {
    console.log(`Selected ${selected.length} items:`, selected)
  })
  .build()

// Select multiple items
multiSelectList.selectItem('file1')
multiSelectList.selectItem('file3')
multiSelectList.selectItem('file4')

console.log('Multi-selection:', multiSelectList.getSelectedItems())
// Output: ['file1', 'file3', 'file4']

// Deselect an item
multiSelectList.deselectItem('file3')
console.log('After deselection:', multiSelectList.getSelectedItems())
// Output: ['file1', 'file4']
```

### No Selection Mode

```typescript
const readOnlyList = new ScrollableListBuilder('read-only')
  .items([
    { id: '1', text: 'Information Item 1' },
    { id: '2', text: 'Information Item 2' },
    { id: '3', text: 'Information Item 3' }
  ])
  .selectionMode(SelectionMode.None)
  .height(4)
  .onHighlightChange((itemId, list) => {
    const item = list.getHighlightedItem()
    if (item) {
      console.log('Viewing info for:', item.text)
    }
  })
  .build()

// Can highlight but not select
readOnlyList.selectNext()  // Highlights next item
console.log('Selected count:', readOnlyList.getSelectedCount())
// Output: 0 (no selections in None mode)
```

## Navigation and Scrolling

### Keyboard Navigation

```typescript
const navigableList = new ScrollableListBuilder('navigable')
  .items(Array.from({ length: 20 }, (_, i) => ({
    id: `item-${i}`,
    text: `Item ${i + 1}`,
    subtitle: `Description for item ${i + 1}`
  })))
  .height(8)
  .vimNavigation(true)
  .showScrollbar(true)
  .build()

// Navigation methods
navigableList.selectNext()        // Arrow Down or 'j'
navigableList.selectPrevious()    // Arrow Up or 'k'
navigableList.selectFirst()       // Home or 'g'
navigableList.selectLast()        // End or 'G'
navigableList.pageUp()            // Page Up
navigableList.pageDown()          // Page Down

// Scroll methods
navigableList.scrollUp(2)         // Scroll up 2 lines
navigableList.scrollDown(3)       // Scroll down 3 lines
navigableList.scrollToTop()       // Scroll to beginning
navigableList.scrollToBottom()    // Scroll to end

// Auto-scroll to highlighted item
navigableList.scrollToItem(15)    // Scroll to show item at index 15
```

### Smooth Scrolling

```typescript
const smoothScrollList = new ScrollableListBuilder('smooth-scroll')
  .items(Array.from({ length: 50 }, (_, i) => ({
    id: `smooth-${i}`,
    text: `Smooth Item ${i + 1}`
  })))
  .height(10)
  .smoothScrolling(true)
  .scrollStep(1)                   // Small steps for smoothness
  .onScroll((position, maxScroll, list) => {
    const percentage = maxScroll > 0 ? Math.round((position / maxScroll) * 100) : 0
    console.log(`Scroll position: ${position}/${maxScroll} (${percentage}%)`)
  })
  .build()

// Smooth scrolling with small steps
smoothScrollList.scrollDown(1)
```

## Search Functionality

### Basic Search

```typescript
const searchableList = new ScrollableListBuilder('searchable')
  .items([
    { id: '1', text: 'Apple', subtitle: 'Fruit' },
    { id: '2', text: 'Banana', subtitle: 'Fruit' },
    { id: '3', text: 'Carrot', subtitle: 'Vegetable' },
    { id: '4', text: 'Date', subtitle: 'Fruit' },
    { id: '5', text: 'Eggplant', subtitle: 'Vegetable' }
  ])
  .height(6)
  .searchEnabled(true)
  .showSubtitles(true)
  .onSearchChange((query, resultCount, list) => {
    console.log(`Search "${query}" found ${resultCount} results`)
  })
  .build()

// Perform search
searchableList.setSearchQuery('app')
console.log('Filtered items:', searchableList.getFilteredItems())
// Shows: Apple (matches in text)

// Search in subtitles too
searchableList.setSearchQuery('fruit')
console.log('Fruit results:', searchableList.getFilteredCount())
// Shows: 3 (Apple, Banana, Date match in subtitle)

// Clear search
searchableList.clearSearch()
console.log('All items restored:', searchableList.getTotalItems())
```

### Search with Highlighting

```typescript
const highlightSearchList = new ScrollableListBuilder('highlight-search')
  .items([
    { id: '1', text: 'JavaScript Developer' },
    { id: '2', text: 'Java Backend Engineer' },
    { id: '3', text: 'TypeScript Specialist' },
    { id: '4', text: 'Python Programmer' },
    { id: '5', text: 'Go Developer' }
  ])
  .height(7)
  .searchEnabled(true)
  .build()

// Search highlights matches in rendered output
searchableList.setSearchQuery('java')
const renderedLines = highlightSearchList.render()
// Will show: [Java]Script Developer, [Java] Backend Engineer
// (with actual highlighting in terminal output)
```

## Large Dataset Handling

### Virtual Rendering

```typescript
const largeList = new ScrollableListBuilder('large-list')
  .items(Array.from({ length: 10000 }, (_, i) => ({
    id: `large-${i}`,
    text: `Large Dataset Item ${i + 1}`,
    subtitle: `ID: ${i + 1}`,
    metadata: { index: i, category: Math.floor(i / 100) }
  })))
  .height(15)
  .showScrollbar(true)
  .showSubtitles(true)
  .searchEnabled(true)
  .onScroll((position, maxScroll, list) => {
    // Only renders visible items for performance
    console.log(`Showing items ${position + 1}-${position + 15} of ${list.getTotalItems()}`)
  })
  .build()

// Efficient navigation through large dataset
largeList.selectLast()           // Jump to item 10,000
largeList.selectFirst()          // Jump back to item 1
largeList.pageDown()             // Fast page navigation

// Search still works efficiently
largeList.setSearchQuery('500')  // Finds items with "500" in text
console.log('Search results:', largeList.getFilteredCount())
```

### Dynamic Item Management

```typescript
const dynamicList = new ScrollableListBuilder('dynamic')
  .items([
    { id: 'initial', text: 'Initial Item' }
  ])
  .height(8)
  .selectionMode(SelectionMode.Multiple)
  .onSelectionChange((selected) => {
    console.log('Selection updated:', selected)
  })
  .build()

// Add items dynamically
dynamicList.addItem({ 
  id: 'dynamic-1', 
  text: 'Dynamically Added Item', 
  icon: '✨' 
})

dynamicList.addItem({ 
  id: 'dynamic-2', 
  text: 'Another Dynamic Item', 
  subtitle: 'Added at runtime',
  icon: '🔄' 
})

// Replace all items
const newItems = Array.from({ length: 5 }, (_, i) => ({
  id: `new-${i}`,
  text: `New Item Set ${i + 1}`,
  icon: '🆕'
}))
dynamicList.setItems(newItems)

// Remove specific items
dynamicList.removeItem('new-2')
console.log('Remaining items:', dynamicList.getTotalItems())
```

## Keyboard Event Handling

### Standard Navigation

```typescript
const keyboardList = new ScrollableListBuilder('keyboard')
  .items([
    { id: '1', text: 'First Option' },
    { id: '2', text: 'Second Option' },
    { id: '3', text: 'Third Option' },
    { id: '4', text: 'Fourth Option' }
  ])
  .height(4)
  .vimNavigation(true)
  .build()

// Handle keyboard events
document.addEventListener('keydown', (event) => {
  if (keyboardList.isFocused()) {
    const handled = keyboardList.handleKeyPress(event.key)
    
    if (handled) {
      event.preventDefault()
      
      // Re-render list to show updates
      const lines = keyboardList.render()
      console.log('Updated list:', lines)
    }
  }
})

// Supported keys:
// - ArrowUp/ArrowDown or k/j: Navigate up/down
// - PageUp/PageDown: Page navigation
// - Home/End or g/G: Jump to first/last
// - Enter/Space: Activate highlighted item
// - Escape: Clear selection or search
```

### Custom Key Handling

```typescript
class CustomKeyList {
  private list: ScrollableList

  constructor() {
    this.list = new ScrollableListBuilder('custom-keys')
      .items([
        { id: '1', text: 'Item 1', metadata: { action: 'action1' } },
        { id: '2', text: 'Item 2', metadata: { action: 'action2' } },
        { id: '3', text: 'Item 3', metadata: { action: 'action3' } }
      ])
      .height(5)
      .onItemActivate((itemId, item) => {
        console.log(`Executing action: ${item.metadata?.action}`)
      })
      .build()
  }

  handleCustomKeys(key: string): boolean {
    // Handle custom key combinations
    switch (key) {
      case 'Enter':
        // Custom enter behavior
        const highlighted = this.list.getHighlightedItem()
        if (highlighted) {
          console.log(`Custom action for: ${highlighted.text}`)
          return true
        }
        break
      
      case 'Delete':
        // Delete highlighted item
        const toDelete = this.list.getHighlightedItem()
        if (toDelete) {
          this.list.removeItem(toDelete.id)
          console.log(`Deleted: ${toDelete.text}`)
          return true
        }
        break
      
      case 'a':
        // Select all items (if multi-select)
        this.list.getItems().forEach(item => {
          this.list.selectItem(item.id)
        })
        console.log('Selected all items')
        return true
      
      default:
        // Fall back to standard navigation
        return this.list.handleKeyPress(key)
    }
    
    return false
  }

  getList(): ScrollableList {
    return this.list
  }
}

// Usage
const customList = new CustomKeyList()
const list = customList.getList()

document.addEventListener('keydown', (event) => {
  if (list.isFocused()) {
    const handled = customList.handleCustomKeys(event.key)
    if (handled) {
      event.preventDefault()
    }
  }
})
```

## Pre-built List Types

### File Browser List

```typescript
import { fileBrowserList } from 'reactive-tui-ts'

const files = [
  { name: 'package.json', type: 'file' as const, size: 1024 },
  { name: 'src', type: 'directory' as const },
  { name: 'README.md', type: 'file' as const, size: 2048 },
  { name: 'node_modules', type: 'directory' as const },
  { name: 'index.ts', type: 'file' as const, size: 512 }
]

const fileBrowser = fileBrowserList(files)

// Pre-configured with file icons, sizes, search enabled
fileBrowser.onItemActivate((itemId, item) => {
  const metadata = item.metadata
  if (metadata?.type === 'directory') {
    console.log(`Entering directory: ${item.text}`)
  } else {
    console.log(`Opening file: ${item.text} (${metadata?.size} bytes)`)
  }
})
```

### Menu List

```typescript
import { menuList } from 'reactive-tui-ts'

const menuItems = [
  { label: 'New File', action: 'file:new', shortcut: 'Ctrl+N' },
  { label: 'Open File', action: 'file:open', shortcut: 'Ctrl+O' },
  { label: 'Save File', action: 'file:save', shortcut: 'Ctrl+S' },
  { label: 'Exit', action: 'app:exit', shortcut: 'Alt+F4', disabled: false }
]

const menu = menuList(menuItems)

// Pre-configured for menu usage (no search, shows shortcuts)
menu.onItemActivate((itemId, item) => {
  console.log(`Menu action: ${itemId}`)
  // Execute the action based on itemId
})
```

### Task List

```typescript
import { taskList } from 'reactive-tui-ts'

const tasks = [
  { id: 'task1', title: 'Implement user login', status: 'pending' as const, priority: 'high' as const },
  { id: 'task2', title: 'Write unit tests', status: 'completed' as const, priority: 'medium' as const },
  { id: 'task3', title: 'Update documentation', status: 'failed' as const, priority: 'low' as const },
  { id: 'task4', title: 'Deploy to staging', status: 'pending' as const, priority: 'high' as const }
]

const taskManager = taskList(tasks)

// Pre-configured with status icons, multi-select, search enabled
taskManager.onSelectionChange((selected) => {
  console.log(`Selected ${selected.length} tasks for batch operation`)
})

taskManager.onItemActivate((itemId, item) => {
  console.log(`Viewing details for task: ${item.text}`)
})
```

## Real-World Examples

### Project File Explorer

```typescript
import { ScrollableList, ScrollableListBuilder, SelectionMode } from 'reactive-tui-ts'

class ProjectFileExplorer {
  private fileList: ScrollableList
  private currentPath: string = '/'
  private fileSystem: Map<string, FileNode[]> = new Map()

  constructor() {
    this.setupFileSystem()
    this.setupFileList()
  }

  private setupFileSystem() {
    // Mock file system structure
    this.fileSystem.set('/', [
      { name: 'src', type: 'directory', size: 0, children: ['src/'] },
      { name: 'docs', type: 'directory', size: 0, children: ['docs/'] },
      { name: 'package.json', type: 'file', size: 1024, extension: 'json' },
      { name: 'README.md', type: 'file', size: 2048, extension: 'md' },
      { name: 'tsconfig.json', type: 'file', size: 512, extension: 'json' }
    ])

    this.fileSystem.set('src/', [
      { name: '..', type: 'directory', size: 0, parent: true },
      { name: 'components', type: 'directory', size: 0, children: ['src/components/'] },
      { name: 'utils', type: 'directory', size: 0, children: ['src/utils/'] },
      { name: 'index.ts', type: 'file', size: 1536, extension: 'ts' },
      { name: 'app.ts', type: 'file', size: 2048, extension: 'ts' }
    ])

    this.fileSystem.set('src/components/', [
      { name: '..', type: 'directory', size: 0, parent: true },
      { name: 'Button.tsx', type: 'file', size: 1024, extension: 'tsx' },
      { name: 'Input.tsx', type: 'file', size: 896, extension: 'tsx' },
      { name: 'Modal.tsx', type: 'file', size: 1536, extension: 'tsx' }
    ])

    this.fileSystem.set('docs/', [
      { name: '..', type: 'directory', size: 0, parent: true },
      { name: 'API.md', type: 'file', size: 4096, extension: 'md' },
      { name: 'GUIDE.md', type: 'file', size: 3072, extension: 'md' }
    ])
  }

  private setupFileList() {
    this.fileList = new ScrollableListBuilder('file-explorer')
      .height(15)
      .selectionMode(SelectionMode.Single)
      .showScrollbar(true)
      .showIcons(true)
      .showSubtitles(true)
      .searchEnabled(true)
      .vimNavigation(true)
      .onItemActivate((itemId, item) => this.handleFileActivate(item))
      .onSelectionChange((selected) => this.handleSelectionChange(selected))
      .onSearchChange((query, results) => {
        console.log(`File search "${query}" found ${results} results`)
      })
      .build()

    this.loadDirectory('/')
  }

  private loadDirectory(path: string) {
    const files = this.fileSystem.get(path) || []
    
    const items = files.map((file, index) => ({
      id: `${path}${file.name}`,
      text: file.name,
      subtitle: this.getFileSubtitle(file),
      icon: this.getFileIcon(file),
      metadata: {
        type: file.type,
        size: file.size,
        extension: file.extension,
        path: path,
        parent: file.parent || false
      }
    }))

    this.fileList.setItems(items)
    this.currentPath = path
    console.log(`Loaded directory: ${path}`)
  }

  private getFileIcon(file: FileNode): string {
    if (file.type === 'directory') {
      return file.parent ? '⬆️' : '📁'
    }

    const iconMap: Record<string, string> = {
      'ts': '🔷',
      'tsx': '⚛️',
      'js': '🟨',
      'jsx': '⚛️',
      'json': '📋',
      'md': '📝',
      'html': '🌐',
      'css': '🎨',
      'png': '🖼️',
      'jpg': '🖼️',
      'gif': '🖼️',
      'pdf': '📄',
      'zip': '📦'
    }

    return iconMap[file.extension || ''] || '📄'
  }

  private getFileSubtitle(file: FileNode): string {
    if (file.type === 'directory') {
      return file.parent ? 'parent directory' : 'directory'
    }

    const sizeStr = this.formatFileSize(file.size)
    const ext = file.extension ? `.${file.extension}` : ''
    
    return `${sizeStr}${ext ? ' • ' + ext : ''}`
  }

  private formatFileSize(bytes: number): string {
    const units = ['B', 'KB', 'MB', 'GB']
    let size = bytes
    let unitIndex = 0

    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024
      unitIndex++
    }

    return `${size.toFixed(1)} ${units[unitIndex]}`
  }

  private handleFileActivate(item: any) {
    const metadata = item.metadata
    
    if (metadata.type === 'directory') {
      if (metadata.parent) {
        // Navigate to parent directory
        const parentPath = this.getParentPath(this.currentPath)
        this.loadDirectory(parentPath)
      } else {
        // Navigate into directory
        const newPath = `${this.currentPath}${item.text}/`
        this.loadDirectory(newPath)
      }
    } else {
      // Open file
      console.log(`Opening file: ${item.text}`)
      console.log(`Path: ${metadata.path}${item.text}`)
      console.log(`Size: ${metadata.size} bytes`)
      console.log(`Type: ${metadata.extension || 'unknown'}`)
    }
  }

  private handleSelectionChange(selected: string[]) {
    if (selected.length > 0) {
      const item = this.fileList.getItem(selected[0])
      if (item) {
        console.log(`Selected: ${item.text}`)
      }
    }
  }

  private getParentPath(path: string): string {
    if (path === '/') return '/'
    
    const parts = path.split('/').filter(Boolean)
    parts.pop() // Remove last directory
    
    return parts.length === 0 ? '/' : `/${parts.join('/')}/`
  }

  // Public API
  getCurrentPath(): string {
    return this.currentPath
  }

  searchFiles(query: string) {
    this.fileList.setSearchQuery(query)
  }

  refresh() {
    this.loadDirectory(this.currentPath)
  }

  navigateUp() {
    const parentPath = this.getParentPath(this.currentPath)
    this.loadDirectory(parentPath)
  }

  getFileList(): ScrollableList {
    return this.fileList
  }

  handleKeyPress(key: string): boolean {
    // Custom key handling
    switch (key) {
      case 'Backspace':
        // Navigate to parent directory
        this.navigateUp()
        return true
      
      case 'F5':
        // Refresh current directory
        this.refresh()
        return true
      
      case '/':
        // Start search
        console.log('Search mode activated')
        return true
        
      default:
        return this.fileList.handleKeyPress(key)
    }
  }
}

interface FileNode {
  name: string
  type: 'file' | 'directory'
  size: number
  extension?: string
  children?: string[]
  parent?: boolean
}

// Usage
const fileExplorer = new ProjectFileExplorer()
const fileList = fileExplorer.getFileList()

// Handle keyboard events
document.addEventListener('keydown', (event) => {
  if (fileList.isFocused()) {
    const handled = fileExplorer.handleKeyPress(event.key)
    if (handled) {
      event.preventDefault()
      
      // Update display
      const lines = fileList.render()
      console.log('File Explorer:')
      console.log(`Current Path: ${fileExplorer.getCurrentPath()}`)
      lines.forEach(line => console.log(line))
    }
  }
})

// Focus the file list
fileList.setFocused(true)

// Demo navigation
fileExplorer.searchFiles('ts')        // Search TypeScript files
fileExplorer.searchFiles('')          // Clear search
```

### Task Management Dashboard

```typescript
import { ScrollableList, ScrollableListBuilder, SelectionMode } from 'reactive-tui-ts'

class TaskManagementDashboard {
  private taskList: ScrollableList
  private tasks: Map<string, Task> = new Map()
  private filterMode: TaskFilter = 'all'

  constructor() {
    this.setupSampleTasks()
    this.setupTaskList()
  }

  private setupSampleTasks() {
    const sampleTasks: Task[] = [
      {
        id: 'task-1',
        title: 'Implement user authentication',
        description: 'Add login and registration functionality',
        status: 'in-progress',
        priority: 'high',
        assignee: 'john@example.com',
        dueDate: '2024-02-15',
        tags: ['backend', 'security']
      },
      {
        id: 'task-2', 
        title: 'Design landing page',
        description: 'Create mockups and wireframes for homepage',
        status: 'completed',
        priority: 'medium',
        assignee: 'jane@example.com',
        dueDate: '2024-02-10',
        tags: ['frontend', 'design']
      },
      {
        id: 'task-3',
        title: 'Write API documentation',
        description: 'Document all REST endpoints with examples',
        status: 'pending',
        priority: 'low',
        assignee: 'bob@example.com',
        dueDate: '2024-02-20',
        tags: ['documentation', 'api']
      },
      {
        id: 'task-4',
        title: 'Setup CI/CD pipeline',
        description: 'Configure automated testing and deployment',
        status: 'in-progress',
        priority: 'high',
        assignee: 'alice@example.com',
        dueDate: '2024-02-12',
        tags: ['devops', 'automation']
      },
      {
        id: 'task-5',
        title: 'Fix mobile responsive issues',
        description: 'Address layout problems on mobile devices',
        status: 'pending',
        priority: 'medium',
        assignee: 'charlie@example.com',
        dueDate: '2024-02-18',
        tags: ['frontend', 'mobile', 'bug']
      }
    ]

    sampleTasks.forEach(task => this.tasks.set(task.id, task))
  }

  private setupTaskList() {
    this.taskList = new ScrollableListBuilder('task-dashboard')
      .height(12)
      .selectionMode(SelectionMode.Multiple)
      .showScrollbar(true)
      .showIcons(true)
      .showSubtitles(true)
      .searchEnabled(true)
      .vimNavigation(true)
      .onItemActivate((itemId, item) => this.viewTaskDetails(itemId))
      .onSelectionChange((selected) => this.handleBulkSelection(selected))
      .onSearchChange((query, results) => {
        console.log(`Task search "${query}" found ${results} results`)
      })
      .build()

    this.updateTaskList()
  }

  private updateTaskList() {
    const filteredTasks = this.getFilteredTasks()
    
    const items = filteredTasks.map(task => ({
      id: task.id,
      text: task.title,
      subtitle: this.getTaskSubtitle(task),
      icon: this.getTaskIcon(task),
      disabled: task.status === 'completed',
      metadata: task
    }))

    this.taskList.setItems(items)
  }

  private getFilteredTasks(): Task[] {
    const allTasks = Array.from(this.tasks.values())
    
    switch (this.filterMode) {
      case 'pending':
        return allTasks.filter(task => task.status === 'pending')
      case 'in-progress':
        return allTasks.filter(task => task.status === 'in-progress')
      case 'completed':
        return allTasks.filter(task => task.status === 'completed')
      case 'high-priority':
        return allTasks.filter(task => task.priority === 'high')
      case 'overdue':
        return allTasks.filter(task => this.isOverdue(task))
      default:
        return allTasks
    }
  }

  private getTaskIcon(task: Task): string {
    const statusIcons = {
      'pending': '⏳',
      'in-progress': '🔄',
      'completed': '✅'
    }

    const priorityModifiers = {
      'high': '🔥',
      'medium': '',
      'low': '❄️'
    }

    const baseIcon = statusIcons[task.status]
    const modifier = priorityModifiers[task.priority]
    
    return modifier ? `${baseIcon}${modifier}` : baseIcon
  }

  private getTaskSubtitle(task: Task): string {
    const parts = []
    
    // Priority and status
    parts.push(`${task.priority} priority`)
    parts.push(task.status.replace('-', ' '))
    
    // Assignee
    if (task.assignee) {
      const name = task.assignee.split('@')[0]
      parts.push(`@${name}`)
    }
    
    // Due date with overdue indication
    if (task.dueDate) {
      const dueDateStr = this.formatDueDate(task.dueDate)
      parts.push(this.isOverdue(task) ? `⚠️ ${dueDateStr}` : dueDateStr)
    }
    
    // Tags
    if (task.tags.length > 0) {
      parts.push(`#${task.tags.join(' #')}`)
    }

    return parts.join(' • ')
  }

  private formatDueDate(dateStr: string): string {
    const date = new Date(dateStr)
    const now = new Date()
    const diffDays = Math.ceil((date.getTime() - now.getTime()) / (1000 * 60 * 60 * 24))
    
    if (diffDays === 0) return 'due today'
    if (diffDays === 1) return 'due tomorrow'
    if (diffDays === -1) return 'due yesterday'
    if (diffDays > 0) return `due in ${diffDays} days`
    return `${Math.abs(diffDays)} days overdue`
  }

  private isOverdue(task: Task): boolean {
    if (!task.dueDate) return false
    return new Date(task.dueDate) < new Date()
  }

  private viewTaskDetails(taskId: string) {
    const task = this.tasks.get(taskId)
    if (task) {
      console.log('\n=== Task Details ===')
      console.log(`Title: ${task.title}`)
      console.log(`Description: ${task.description}`)
      console.log(`Status: ${task.status}`)
      console.log(`Priority: ${task.priority}`)
      console.log(`Assignee: ${task.assignee}`)
      console.log(`Due Date: ${task.dueDate}`)
      console.log(`Tags: ${task.tags.join(', ')}`)
      console.log('==================\n')
    }
  }

  private handleBulkSelection(selected: string[]) {
    if (selected.length === 0) {
      console.log('No tasks selected')
      return
    }

    if (selected.length === 1) {
      const task = this.tasks.get(selected[0])
      if (task) {
        console.log(`Selected: ${task.title}`)
      }
    } else {
      console.log(`Selected ${selected.length} tasks for bulk operations:`)
      selected.forEach(taskId => {
        const task = this.tasks.get(taskId)
        if (task) {
          console.log(`  - ${task.title}`)
        }
      })
    }
  }

  // Public API methods
  setFilter(filter: TaskFilter) {
    this.filterMode = filter
    this.updateTaskList()
    console.log(`Filter changed to: ${filter}`)
  }

  addTask(task: Omit<Task, 'id'>) {
    const id = `task-${Date.now()}`
    const newTask: Task = { ...task, id }
    this.tasks.set(id, newTask)
    this.updateTaskList()
    console.log(`Added task: ${task.title}`)
  }

  updateTaskStatus(taskId: string, status: TaskStatus) {
    const task = this.tasks.get(taskId)
    if (task) {
      task.status = status
      this.updateTaskList()
      console.log(`Updated task ${task.title} status to: ${status}`)
    }
  }

  deleteSelectedTasks() {
    const selected = this.taskList.getSelectedItems()
    if (selected.length === 0) {
      console.log('No tasks selected for deletion')
      return
    }

    selected.forEach(taskId => {
      const task = this.tasks.get(taskId)
      if (task) {
        this.tasks.delete(taskId)
        console.log(`Deleted task: ${task.title}`)
      }
    })

    this.updateTaskList()
    console.log(`Deleted ${selected.length} tasks`)
  }

  bulkUpdatePriority(priority: TaskPriority) {
    const selected = this.taskList.getSelectedItems()
    if (selected.length === 0) {
      console.log('No tasks selected for priority update')
      return
    }

    selected.forEach(taskId => {
      const task = this.tasks.get(taskId)
      if (task) {
        task.priority = priority
      }
    })

    this.updateTaskList()
    console.log(`Updated priority to ${priority} for ${selected.length} tasks`)
  }

  searchTasks(query: string) {
    this.taskList.setSearchQuery(query)
  }

  getTaskList(): ScrollableList {
    return this.taskList
  }

  handleKeyPress(key: string): boolean {
    // Custom keyboard shortcuts
    switch (key) {
      case 'n':
        // New task shortcut
        console.log('New task shortcut pressed')
        return true
      
      case 'd':
        // Delete selected tasks
        this.deleteSelectedTasks()
        return true
      
      case '1':
        this.setFilter('pending')
        return true
      
      case '2':
        this.setFilter('in-progress')
        return true
      
      case '3':
        this.setFilter('completed')
        return true
      
      case '0':
        this.setFilter('all')
        return true
      
      case 'h':
        this.setFilter('high-priority')
        return true
      
      case 'o':
        this.setFilter('overdue')
        return true
      
      default:
        return this.taskList.handleKeyPress(key)
    }
  }

  getStats() {
    const allTasks = Array.from(this.tasks.values())
    return {
      total: allTasks.length,
      pending: allTasks.filter(t => t.status === 'pending').length,
      inProgress: allTasks.filter(t => t.status === 'in-progress').length,
      completed: allTasks.filter(t => t.status === 'completed').length,
      overdue: allTasks.filter(t => this.isOverdue(t)).length,
      highPriority: allTasks.filter(t => t.priority === 'high').length
    }
  }
}

// Types
interface Task {
  id: string
  title: string
  description: string
  status: TaskStatus
  priority: TaskPriority
  assignee: string
  dueDate: string
  tags: string[]
}

type TaskStatus = 'pending' | 'in-progress' | 'completed'
type TaskPriority = 'low' | 'medium' | 'high'
type TaskFilter = 'all' | 'pending' | 'in-progress' | 'completed' | 'high-priority' | 'overdue'

// Usage
const taskDashboard = new TaskManagementDashboard()
const taskList = taskDashboard.getTaskList()

// Show initial stats
console.log('Task Dashboard Stats:', taskDashboard.getStats())

// Handle keyboard events
document.addEventListener('keydown', (event) => {
  if (taskList.isFocused()) {
    const handled = taskDashboard.handleKeyPress(event.key)
    if (handled) {
      event.preventDefault()
      
      // Update display
      const lines = taskList.render()
      console.log('\n=== Task Dashboard ===')
      console.log(`Filter: ${taskDashboard.filterMode || 'all'}`)
      console.log(`Stats: ${JSON.stringify(taskDashboard.getStats())}`)
      lines.forEach(line => console.log(line))
      console.log('=====================\n')
    }
  }
})

// Demo operations
taskList.setFocused(true)

// Add a new task
taskDashboard.addTask({
  title: 'Optimize database queries',
  description: 'Improve query performance and add indexes',
  status: 'pending',
  priority: 'high',
  assignee: 'dev@example.com',
  dueDate: '2024-02-14',
  tags: ['backend', 'performance', 'database']
})

// Search for specific tasks
taskDashboard.searchTasks('authentication')

// Filter by status
taskDashboard.setFilter('high-priority')
```

## CSS Styling

```css
/* ScrollableList container */
.scrollable-list {
  position: relative;
  display: inline-block;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  border: 1px solid #4b5563;
  background-color: #1f2937;
  color: #f9fafb;
  border-radius: 0.375rem;
  overflow: hidden;
}

/* List items */
.scrollable-list-item {
  display: flex;
  align-items: center;
  padding: 0.5rem 0.75rem;
  line-height: 1.5;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 1px solid rgba(75, 85, 99, 0.3);
}

.scrollable-list-item:last-child {
  border-bottom: none;
}

.scrollable-list-item:hover {
  background-color: #374151;
}

.scrollable-list-item.highlighted {
  background-color: #4b5563;
  color: #ffffff;
}

.scrollable-list-item.selected {
  background-color: #3b82f6;
  color: #ffffff;
}

.scrollable-list-item.selected.highlighted {
  background-color: #2563eb;
}

.scrollable-list-item.disabled {
  opacity: 0.6;
  cursor: not-allowed;
  color: #9ca3af;
}

/* Selection indicators */
.item-selection-indicator {
  margin-right: 0.5rem;
  font-size: 0.875rem;
  width: 1rem;
  text-align: center;
}

/* Icons */
.item-icon {
  margin-right: 0.5rem;
  font-size: 1rem;
  width: 1.25rem;
  text-align: center;
  color: #60a5fa;
}

/* Content */
.item-content {
  flex: 1;
  min-width: 0;
}

.item-text {
  font-weight: 500;
  font-size: 0.875rem;
  margin-bottom: 0.125rem;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.item-subtitle {
  font-size: 0.75rem;
  color: #9ca3af;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.scrollable-list-item.highlighted .item-subtitle {
  color: rgba(255, 255, 255, 0.8);
}

.scrollable-list-item.selected .item-subtitle {
  color: rgba(255, 255, 255, 0.9);
}

/* Search highlighting */
.search-highlight {
  background-color: #fbbf24;
  color: #92400e;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-weight: 600;
}

.scrollable-list-item.highlighted .search-highlight,
.scrollable-list-item.selected .search-highlight {
  background-color: rgba(255, 255, 255, 0.3);
  color: #ffffff;
}

/* Scrollbar */
.scrollable-list-scrollbar {
  position: absolute;
  top: 0;
  right: 0;
  width: 1rem;
  height: 100%;
  background-color: rgba(75, 85, 99, 0.3);
}

.scrollbar-thumb {
  position: absolute;
  right: 0;
  width: 1rem;
  background-color: #6b7280;
  border-radius: 0.25rem;
  transition: background-color 0.15s ease;
}

.scrollbar-thumb:hover {
  background-color: #9ca3af;
}

/* Borders */
.scrollable-list.bordered {
  border: 2px solid #4b5563;
}

.scrollable-list.bordered.focused {
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

/* Empty state */
.scrollable-list-empty {
  padding: 2rem 1rem;
  text-align: center;
  color: #9ca3af;
  font-size: 0.875rem;
}

.scrollable-list-empty::before {
  content: '📄';
  display: block;
  font-size: 2rem;
  margin-bottom: 0.5rem;
  opacity: 0.5;
}

/* Loading state */
.scrollable-list-loading {
  padding: 1rem;
  text-align: center;
  color: #9ca3af;
}

.scrollable-list-loading::before {
  content: '';
  display: inline-block;
  width: 1rem;
  height: 1rem;
  margin-right: 0.5rem;
  border: 2px solid #4b5563;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  vertical-align: middle;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Search active state */
.scrollable-list.search-active {
  border-color: #f59e0b;
}

.scrollable-list.search-active::before {
  content: '🔍 Search active';
  position: absolute;
  top: -1.5rem;
  left: 0;
  font-size: 0.75rem;
  color: #f59e0b;
  background-color: #1f2937;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
}

/* Dark theme variations */
.scrollable-list.theme-dark {
  background-color: #111827;
  border-color: #374151;
  color: #f3f4f6;
}

.scrollable-list.theme-dark .scrollable-list-item:hover {
  background-color: #1f2937;
}

.scrollable-list.theme-dark .scrollable-list-item.highlighted {
  background-color: #374151;
}

/* Light theme */
.scrollable-list.theme-light {
  background-color: #ffffff;
  border-color: #e5e7eb;
  color: #1f2937;
}

.scrollable-list.theme-light .scrollable-list-item:hover {
  background-color: #f9fafb;
}

.scrollable-list.theme-light .scrollable-list-item.highlighted {
  background-color: #e5e7eb;
  color: #1f2937;
}

.scrollable-list.theme-light .scrollable-list-item.selected {
  background-color: #3b82f6;
  color: #ffffff;
}

.scrollable-list.theme-light .item-subtitle {
  color: #6b7280;
}

/* Responsive design */
@media (max-width: 768px) {
  .scrollable-list-item {
    padding: 0.75rem;
  }
  
  .item-subtitle {
    display: none;
  }
  
  .scrollable-list-scrollbar {
    width: 0.75rem;
  }
}

/* Accessibility */
.scrollable-list:focus-within {
  outline: none; /* Custom focus handling */
}

.scrollable-list-item:focus {
  outline: 2px solid #3b82f6;
  outline-offset: -2px;
}

@media (prefers-reduced-motion: reduce) {
  .scrollable-list-item,
  .scrollbar-thumb {
    transition: none;
  }
  
  @keyframes spin {
    to {
      transform: none;
    }
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .scrollable-list {
    border-width: 2px;
    border-color: currentColor;
  }
  
  .scrollable-list-item.highlighted,
  .scrollable-list-item.selected {
    outline: 2px solid currentColor;
    outline-offset: -2px;
  }
}

/* Print styles */
@media print {
  .scrollable-list {
    border: 1px solid #000000;
    background: #ffffff;
    color: #000000;
  }
  
  .scrollable-list-scrollbar {
    display: none;
  }
  
  .scrollable-list-item.selected {
    background-color: #e5e7eb;
    color: #000000;
  }
}
```

## Best Practices

### 1. Appropriate Height Settings

```typescript
// ✅ Good - reasonable heights for different use cases
const compactList = new ScrollableListBuilder('compact')
  .height(5)          // Small for dropdowns/menus
  .items(menuItems)
  .build()

const standardList = new ScrollableListBuilder('standard')
  .height(12)         // Standard for main content
  .items(fileItems)
  .build()

const fullHeightList = new ScrollableListBuilder('full-height')
  .height(25)         // Larger for primary interfaces
  .items(largeDataset)
  .build()

// ❌ Poor - inappropriate heights
const tinyList = new ScrollableListBuilder('tiny')
  .height(1)          // Too small, poor usability
  .build()

const hugeList = new ScrollableListBuilder('huge')
  .height(100)        // Too large, overwhelming
  .build()
```

### 2. Efficient Large Dataset Handling

```typescript
// ✅ Good - virtual rendering for large datasets
const efficientLargeList = new ScrollableListBuilder('efficient')
  .items(largeDataset)  // 10,000+ items
  .height(15)           // Reasonable viewport
  .showScrollbar(true)  // Visual scroll indicator
  .searchEnabled(true)  // Quick filtering
  .build()

// ❌ Poor - loading everything at once
const inefficientList = new ScrollableListBuilder('inefficient')
  .items(massiveDataset)  // 100,000+ items
  .height(50)             // Trying to show too much
  .showScrollbar(false)   // No scroll indication
  .build()
```

### 3. Meaningful Item Structure

```typescript
// ✅ Good - descriptive items with useful metadata
const informativeList = new ScrollableListBuilder('informative')
  .items([
    {
      id: 'task-123',
      text: 'Implement user authentication',
      subtitle: 'High priority • Due Feb 15 • @john',
      icon: '🔒',
      metadata: { priority: 'high', assignee: 'john', dueDate: '2024-02-15' }
    }
  ])
  .showIcons(true)
  .showSubtitles(true)
  .build()

// ❌ Poor - minimal information, no context
const uninformativeList = new ScrollableListBuilder('uninformative')
  .items([
    { id: '1', text: 'Task 1' },
    { id: '2', text: 'Task 2' }
  ])
  .showIcons(false)
  .showSubtitles(false)
  .build()
```

### 4. Appropriate Selection Modes

```typescript
// ✅ Good - correct selection modes for use cases
const fileOperationsList = new ScrollableListBuilder('file-ops')
  .selectionMode(SelectionMode.Multiple)  // Multiple files for batch operations
  .items(fileItems)
  .build()

const navigationMenu = new ScrollableListBuilder('nav-menu')
  .selectionMode(SelectionMode.Single)    // Single selection for navigation
  .items(menuItems)
  .build()

const informationDisplay = new ScrollableListBuilder('info-display')
  .selectionMode(SelectionMode.None)      // Read-only information
  .items(infoItems)
  .build()

// ❌ Poor - wrong selection modes
const confusingBatchList = new ScrollableListBuilder('confusing')
  .selectionMode(SelectionMode.Single)    // Should be Multiple for batch operations
  .items(batchOperationItems)
  .build()
```

### 5. Smart Search Integration

```typescript
// ✅ Good - search enabled for large, searchable datasets
const searchableDataList = new ScrollableListBuilder('searchable-data')
  .items(largeUserList)
  .searchEnabled(true)
  .height(15)
  .onSearchChange((query, resultCount) => {
    console.log(`Found ${resultCount} users matching "${query}"`)
  })
  .build()

// ✅ Good - search disabled for small, static lists
const staticMenuList = new ScrollableListBuilder('static-menu')
  .items(shortMenuItems)  // 5-10 items
  .searchEnabled(false)   // No need for search
  .height(6)
  .build()

// ❌ Poor - search enabled for inappropriate content
const searchlessMenu = new ScrollableListBuilder('searchless')
  .items([
    { id: '1', text: 'Yes' },
    { id: '2', text: 'No' }
  ])
  .searchEnabled(true)    // Unnecessary for 2 items
  .build()
```

## Related Widgets

- **[Menu](./menu)** - Hierarchical navigation menus
- **[Select](./select)** - Dropdown selection lists
- **[Tree](./tree)** - Hierarchical tree structures
- **[Autocomplete](./autocomplete)** - Search-as-you-type lists

## Examples

- **[Basic Lists](../../examples/basic/scrollable-list-basic)** - Simple list implementations
- **[File Browser](../../examples/apps/file-browser)** - Interactive file system navigation
- **[Task Dashboard](../../examples/apps/task-dashboard)** - Task management interface
- **[Data Explorer](../../examples/advanced/data-explorer)** - Large dataset navigation

The ScrollableList widget provides comprehensive list functionality with efficient rendering, advanced navigation, search capabilities, and flexible selection modes, making it essential for data-heavy applications and interactive interfaces.