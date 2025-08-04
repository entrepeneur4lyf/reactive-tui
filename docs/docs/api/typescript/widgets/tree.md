---
sidebar_position: 4
---

# Tree Widget

A comprehensive hierarchical tree widget supporting expand/collapse, lazy loading, multi-selection, keyboard navigation, and virtual scrolling for large datasets.

## Overview

The Tree widget provides a powerful hierarchical data display system with comprehensive functionality for displaying nested structures like file systems, organizational charts, and navigation menus.

```typescript
import { tree, TreeBuilder, createTreeNode, TreeNodeType } from 'reactive-tui-ts'

const fileTree = tree('file-explorer', {
  multiSelect: true,
  expandable: true,
  lazyLoading: true,
  searchEnabled: true
})
```

## Types

### NodeId

```typescript
export type NodeId = string
```

### TreeNodeType

```typescript
export enum TreeNodeType {
  Node = 'node',
  Folder = 'folder',
  Leaf = 'leaf',
  Loading = 'loading',
  Custom = 'custom'
}
```

## Configuration

### TreeConfig

```typescript
interface TreeConfig {
  multiSelect: boolean
  expandable: boolean
  lazyLoading: boolean
  virtualScrolling: boolean
  searchEnabled: boolean
  dragDropEnabled: boolean
  maxVisibleNodes: number
  animationDuration: number
  nodeStyles: Map<TreeNodeType, TreeNodeStyle>
}
```

### TreeNode

```typescript
interface TreeNode {
  id: NodeId
  label: string
  description?: string
  nodeType: TreeNodeType
  data: Record<string, string>
  children: NodeId[]
  expandable: boolean
  childrenLoaded: boolean
  expanded: boolean
  selected: boolean
  disabled: boolean
  style?: TreeNodeStyle
  level: number
  isLastChild: boolean
}
```

### TreeNodeStyle

```typescript
interface TreeNodeStyle {
  icon?: string
  textClass?: string
  backgroundClass?: string
  cssClasses: string[]
  indentChars: TreeIndentChars
}
```

### TreeIndentChars

```typescript
interface TreeIndentChars {
  vertical: string
  horizontal: string
  branch: string
  lastBranch: string
  space: string
  expanded: string
  collapsed: string
  leaf: string
}
```

### Default Indent Characters

```typescript
export const defaultIndentChars: TreeIndentChars = {
  vertical: '│',
  horizontal: '─',
  branch: '├',
  lastBranch: '└',
  space: ' ',
  expanded: '▼',
  collapsed: '▶',
  leaf: '•'
}
```

### TreeState

```typescript
interface TreeState {
  expandedNodes: Set<NodeId>
  selectedNodes: Set<NodeId>
  highlightedNode?: NodeId
  searchQuery: string
  filteredNodes: Set<NodeId>
  focused: boolean
  scrollOffset: number
  viewportHeight: number
  loadingNodes: Set<NodeId>
}
```

### LazyLoader

```typescript
export type LazyLoader = (nodeId: NodeId) => Promise<TreeNode[]>
```

### TreeCallbacks

```typescript
interface TreeCallbacks {
  onSelect?: (selectedNodes: NodeId[]) => void
  onExpand?: (nodeId: NodeId, expanded: boolean) => void
  onAction?: (nodeId: NodeId) => void
  onSearch?: (query: string) => void
}
```

## Basic Usage

### Creating a Tree

```typescript
import { tree, createTreeNode, createFolderNode, createLeafNode, TreeNodeType } from 'reactive-tui-ts'

// Create tree instance
const fileTree = tree('file-explorer', {
  multiSelect: true,
  expandable: true,
  lazyLoading: false,
  searchEnabled: true
})

// Create and add nodes
const rootNode = createFolderNode('root', 'Project')
fileTree.addRootNode(rootNode)

// Create child nodes
const srcNode = createFolderNode('src', 'src')
const docsNode = createFolderNode('docs', 'docs')
const indexNode = createLeafNode('index', 'index.ts')
const appNode = createLeafNode('app', 'app.ts')

// Add child nodes
fileTree.addChildNode('root', srcNode)
fileTree.addChildNode('root', docsNode)
fileTree.addChildNode('src', indexNode)
fileTree.addChildNode('src', appNode)

// Expand root node
await fileTree.expand('root')
```

### Tree with Callbacks

```typescript
const interactiveTree = tree('interactive-tree', {
  multiSelect: false,
  expandable: true,
  searchEnabled: true
})

// Set up callbacks
interactiveTree.callbacks = {
  onSelect: (selectedNodes) => {
    console.log('Selected nodes:', selectedNodes)
    handleNodeSelection(selectedNodes)
  },
  onExpand: (nodeId, expanded) => {
    console.log(`Node ${nodeId} ${expanded ? 'expanded' : 'collapsed'}`)
    if (expanded) {
      loadChildrenIfNeeded(nodeId)
    }
  },
  onAction: (nodeId) => {
    console.log('Action on node:', nodeId)
    openNodeDetails(nodeId)
  },
  onSearch: (query) => {
    console.log('Search query:', query)
    highlightSearchResults(query)
  }
}
```

## TreeBuilder Pattern

### Using TreeBuilder

```typescript
import { TreeBuilder, createFolderNode, createLeafNode } from 'reactive-tui-ts'

const navigationTree = new TreeBuilder('navigation-tree')
  .multiSelect(false)
  .expandable(true)
  .searchEnabled(true)
  .rootNode(createFolderNode('home', 'Home'))
  .rootNode(createFolderNode('settings', 'Settings'))
  .onSelect((selectedNodes) => {
    console.log('Navigation selection:', selectedNodes)
    navigateTo(selectedNodes[0])
  })
  .onExpand((nodeId, expanded) => {
    console.log(`${nodeId} ${expanded ? 'expanded' : 'collapsed'}`)
  })
  .build()

// Add children after building
navigationTree.addChildNode('home', createLeafNode('dashboard', 'Dashboard'))
navigationTree.addChildNode('home', createLeafNode('profile', 'Profile'))
navigationTree.addChildNode('settings', createLeafNode('general', 'General'))
navigationTree.addChildNode('settings', createLeafNode('security', 'Security'))
```

### Lazy Loading Tree

```typescript
class LazyFileSystemTree {
  private tree: Tree

  constructor() {
    this.tree = tree('file-system', {
      multiSelect: true,
      expandable: true,
      lazyLoading: true,
      searchEnabled: true
    })

    // Set up lazy loader
    this.tree.lazyLoader = async (nodeId: NodeId) => {
      return await this.loadDirectoryContents(nodeId)
    }

    // Set up callbacks
    this.tree.callbacks = {
      onSelect: (selectedNodes) => {
        console.log('Selected files:', selectedNodes)
      },
      onExpand: (nodeId, expanded) => {
        if (expanded) {
          console.log('Loading contents for:', nodeId)
        }
      },
      onAction: (nodeId) => {
        const node = this.tree.nodes.get(nodeId)
        if (node && node.nodeType === TreeNodeType.Leaf) {
          this.openFile(node)
        }
      }
    }
  }

  private async loadDirectoryContents(path: string): Promise<TreeNode[]> {
    // Simulate API call
    const files = await this.fetchDirectoryContents(path)
    
    return files.map(file => {
      if (file.isDirectory) {
        return createFolderNode(file.id, file.name)
      } else {
        return createLeafNode(file.id, file.name)
      }
    })
  }

  private async fetchDirectoryContents(path: string) {
    // Simulate API call delay
    await new Promise(resolve => setTimeout(resolve, 200))
    
    return [
      { id: `${path}/file1.txt`, name: 'file1.txt', isDirectory: false },
      { id: `${path}/file2.txt`, name: 'file2.txt', isDirectory: false },
      { id: `${path}/subfolder`, name: 'subfolder', isDirectory: true }
    ]
  }

  private openFile(node: TreeNode) {
    console.log('Opening file:', node.label)
    // Implement file opening logic
  }

  getTree(): Tree {
    return this.tree
  }
}
```

## Node Creation Functions

### Helper Functions

```typescript
import { createTreeNode, createFolderNode, createLeafNode, TreeNodeType } from 'reactive-tui-ts'

// Generic node (default type: Node)
const genericNode = createTreeNode('node1', 'Generic Node')
// Returns: { id: 'node1', label: 'Generic Node', nodeType: TreeNodeType.Node, ... }

// Folder node (expandable by default)
const folderNode = createFolderNode('folder1', 'My Folder')
// Returns: { id: 'folder1', label: 'My Folder', nodeType: TreeNodeType.Folder, expandable: true, ... }

// Leaf node (cannot have children)
const leafNode = createLeafNode('file1', 'file1.txt')
// Returns: { id: 'file1', label: 'file1.txt', nodeType: TreeNodeType.Leaf, expandable: false, ... }
```

### Custom Node Creation

```typescript
// Create custom nodes with specific properties
const customNode: TreeNode = {
  id: 'custom1',
  label: 'Custom Node',
  description: 'A custom node with extra data',
  nodeType: TreeNodeType.Custom,
  data: { 
    category: 'important',
    priority: 'high'
  },
  children: [],
  expandable: true,
  childrenLoaded: false,
  expanded: false,
  selected: false,
  disabled: false,
  level: 0,
  isLastChild: false,
  style: {
    icon: '⭐',
    textClass: 'custom-text',
    cssClasses: ['custom-node'],
    indentChars: defaultIndentChars
  }
}
```

## Tree Manipulation

### Selection Operations

```typescript
// Select a single node
tree.select('node1')

// Deselect a node
tree.deselect('node1')

// Toggle selection
tree.toggleSelection('node1')

// Clear all selections
tree.clearSelection()

// Get selected nodes
const selectedIds = tree.getSelectedIds()
const selectedNodes = tree.getSelectedNodes()
```

### Expansion Operations

```typescript
// Expand a node (supports lazy loading)
await tree.expand('folder1')

// Collapse a node
tree.collapse('folder1')

// Toggle expansion
await tree.toggleExpand('folder1')
```

### Search Operations

```typescript
// Set search query (filters visible nodes)
tree.setSearchQuery('important')

// Clear search
tree.setSearchQuery('')

// Get filtered results
const visibleNodes = tree.getVisibleNodes()
```

### Keyboard Navigation

```typescript
// Navigate to next/previous node
tree.navigateNext()
tree.navigatePrevious()

// Handle keyboard events
const handleKeyboard = async (event: KeyboardEvent) => {
  const handled = await tree.handleKeyEvent(event)
  if (handled) {
    event.preventDefault()
  }
}

// Supported keys:
// - ArrowDown: Navigate to next node
// - ArrowUp: Navigate to previous node
// - ArrowRight/Enter: Expand highlighted node
// - ArrowLeft: Collapse highlighted node
// - Space: Toggle selection of highlighted node
```

### Node Rendering

```typescript
// Render a single node to string
const nodeText = tree.renderNode(node)

// Get current tree state
const state = tree.getState()
console.log('Expanded nodes:', state.expandedNodes)
console.log('Selected nodes:', state.selectedNodes)
console.log('Search query:', state.searchQuery)

// Update tree configuration
tree.updateConfig({
  multiSelect: true,
  virtualScrolling: true,
  maxVisibleNodes: 500
})
```

## Lazy Loading

### Setting up Lazy Loading

```typescript
const lazyTree = tree('lazy-tree', {
  multiSelect: true,
  expandable: true,
  lazyLoading: true,
  virtualScrolling: true
})

// Set lazy loader function
lazyTree.lazyLoader = async (nodeId: NodeId): Promise<TreeNode[]> => {
  // Simulate API call
  console.log(`Loading children for ${nodeId}...`)
  
  try {
    const response = await fetch(`/api/tree-nodes/${nodeId}/children`)
    const data = await response.json()
    
    return data.map((item: any) => {
      if (item.isDirectory) {
        return createFolderNode(item.id, item.name)
      } else {
        return createLeafNode(item.id, item.name)
      }
    })
  } catch (error) {
    console.error('Failed to load children:', error)
    return []
  }
}

// Add root nodes that will lazy load their children
const rootFolder = createFolderNode('root', 'Root Directory')
lazyTree.addRootNode(rootFolder)
```

### Lazy Loading with Caching

```typescript
class CachedTreeLoader {
  private cache = new Map<NodeId, TreeNode[]>()
  private tree: Tree

  constructor() {
    this.tree = tree('cached-tree', {
      multiSelect: false,
      expandable: true,
      lazyLoading: true
    })

    this.tree.lazyLoader = this.loadWithCache.bind(this)
  }

  private async loadWithCache(nodeId: NodeId): Promise<TreeNode[]> {
    // Check cache first
    if (this.cache.has(nodeId)) {
      console.log(`Cache hit for ${nodeId}`)
      return this.cache.get(nodeId)!
    }

    // Load from API
    console.log(`Loading ${nodeId} from API...`)
    const children = await this.fetchChildren(nodeId)
    
    // Cache the results
    this.cache.set(nodeId, children)
    
    return children
  }

  private async fetchChildren(nodeId: NodeId): Promise<TreeNode[]> {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 300))
    
    // Return mock data
    return [
      createFolderNode(`${nodeId}-folder1`, 'Subfolder 1'),
      createFolderNode(`${nodeId}-folder2`, 'Subfolder 2'),
      createLeafNode(`${nodeId}-file1`, 'file1.txt'),
      createLeafNode(`${nodeId}-file2`, 'file2.txt')
    ]
  }

  getTree(): Tree {
    return this.tree
  }
}
```

## Event Handling

### Setting up Callbacks

```typescript
const interactiveTree = tree('interactive-tree', {
  multiSelect: true,
  expandable: true,
  searchEnabled: true
})

// Set up event callbacks
interactiveTree.callbacks = {
  onSelect: (selectedNodes: NodeId[]) => {
    console.log('Selection changed:', selectedNodes)
    updateToolbar(selectedNodes.length)
    
    // Get actual node objects
    const nodes = selectedNodes
      .map(id => interactiveTree.nodes.get(id))
      .filter(node => node !== undefined)
    
    displaySelectedItems(nodes)
  },
  
  onExpand: (nodeId: NodeId, expanded: boolean) => {
    const node = interactiveTree.nodes.get(nodeId)
    if (node) {
      console.log(`${node.label} ${expanded ? 'expanded' : 'collapsed'}`)
      
      if (expanded && node.nodeType === TreeNodeType.Folder) {
        // Auto-load children if needed
        if (node.children.length === 0 && !node.childrenLoaded) {
          loadFolderContents(nodeId)
        }
      }
    }
  },
  
  onAction: (nodeId: NodeId) => {
    const node = interactiveTree.nodes.get(nodeId)
    if (node) {
      console.log('Action triggered on:', node.label)
      
      if (node.nodeType === TreeNodeType.Leaf) {
        openFile(node)
      } else if (node.nodeType === TreeNodeType.Folder) {
        showFolderProperties(node)
      }
    }
  },
  
  onSearch: (query: string) => {
    console.log('Search query updated:', query)
    
    // Update search UI
    updateSearchHighlight(query)
    
    // Get filtered results
    const visibleNodes = interactiveTree.getVisibleNodes()
    updateSearchResults(visibleNodes.length)
  }
}
```

## Complete File Explorer Example

```typescript
import { tree, createFolderNode, createLeafNode, TreeNodeType, Tree, TreeNode, NodeId } from 'reactive-tui-ts'

class FileExplorer {
  private tree: Tree
  private selectedFiles: NodeId[] = []

  constructor() {
    this.tree = tree('file-explorer', {
      multiSelect: true,
      expandable: true,
      lazyLoading: true,
      searchEnabled: true,
      virtualScrolling: true,
      maxVisibleNodes: 1000
    })

    this.setupTree()
  }

  private setupTree() {
    // Set up lazy loader
    this.tree.lazyLoader = async (nodeId: NodeId) => {
      return await this.loadDirectoryContents(nodeId)
    }

    // Set up callbacks
    this.tree.callbacks = {
      onSelect: (selectedNodes) => {
        this.selectedFiles = selectedNodes
        this.updateSelectionDisplay()
      },
      
      onExpand: (nodeId, expanded) => {
        const node = this.tree.nodes.get(nodeId)
        if (node && expanded) {
          console.log(`Loading contents of: ${node.label}`)
        }
      },
      
      onAction: (nodeId) => {
        const node = this.tree.nodes.get(nodeId)
        if (node) {
          if (node.nodeType === TreeNodeType.Leaf) {
            this.openFile(node)
          } else {
            this.showFolderProperties(node)
          }
        }
      },
      
      onSearch: (query) => {
        this.updateSearchResults(query)
      }
    }

    // Initialize with root directories
    this.initializeRootNodes()
  }

  private initializeRootNodes() {
    const rootDirs = [
      createFolderNode('home', 'Home'),
      createFolderNode('documents', 'Documents'),
      createFolderNode('downloads', 'Downloads'),
      createFolderNode('projects', 'Projects')
    ]

    rootDirs.forEach(dir => {
      this.tree.addRootNode(dir)
    })
  }

  private async loadDirectoryContents(nodeId: NodeId): Promise<TreeNode[]> {
    // Simulate API call to load directory contents
    await new Promise(resolve => setTimeout(resolve, 200))

    try {
      const contents = await this.fetchDirectoryContents(nodeId)
      const nodes: TreeNode[] = []

      for (const item of contents) {
        if (item.isDirectory) {
          const folderNode = createFolderNode(item.id, item.name)
          folderNode.data = {
            path: item.path,
            size: item.size?.toString() || '0',
            modified: item.modified || new Date().toISOString()
          }
          nodes.push(folderNode)
        } else {
          const fileNode = createLeafNode(item.id, item.name)
          fileNode.data = {
            path: item.path,
            size: item.size?.toString() || '0',
            type: item.type || 'file',
            modified: item.modified || new Date().toISOString()
          }
          nodes.push(fileNode)
        }
      }

      // Sort: directories first, then files alphabetically
      return nodes.sort((a, b) => {
        if (a.nodeType === TreeNodeType.Folder && b.nodeType === TreeNodeType.Leaf) return -1
        if (a.nodeType === TreeNodeType.Leaf && b.nodeType === TreeNodeType.Folder) return 1
        return a.label.localeCompare(b.label)
      })
    } catch (error) {
      console.error(`Failed to load directory ${nodeId}:`, error)
      return []
    }
  }

  private async fetchDirectoryContents(path: string) {
    // Mock API response
    const mockContents = [
      { id: `${path}/src`, name: 'src', isDirectory: true, path: `${path}/src` },
      { id: `${path}/docs`, name: 'docs', isDirectory: true, path: `${path}/docs` },
      { id: `${path}/package.json`, name: 'package.json', isDirectory: false, path: `${path}/package.json`, type: 'json', size: 1024 },
      { id: `${path}/README.md`, name: 'README.md', isDirectory: false, path: `${path}/README.md`, type: 'markdown', size: 2048 },
      { id: `${path}/tsconfig.json`, name: 'tsconfig.json', isDirectory: false, path: `${path}/tsconfig.json`, type: 'json', size: 512 }
    ]

    return mockContents
  }

  private openFile(node: TreeNode) {
    console.log(`Opening file: ${node.label}`)
    console.log('File data:', node.data)
    
    // Implement file opening logic based on file type
    const fileType = node.data.type || 'unknown'
    switch (fileType) {
      case 'json':
        this.openJsonFile(node)
        break
      case 'markdown':
        this.openMarkdownFile(node)
        break
      default:
        this.openTextFile(node)
    }
  }

  private openJsonFile(node: TreeNode) {
    console.log(`Opening JSON file: ${node.label}`)
    // Implement JSON file viewer
  }

  private openMarkdownFile(node: TreeNode) {
    console.log(`Opening Markdown file: ${node.label}`)
    // Implement Markdown viewer
  }

  private openTextFile(node: TreeNode) {
    console.log(`Opening text file: ${node.label}`)
    // Implement text editor
  }

  private showFolderProperties(node: TreeNode) {
    console.log(`Folder properties: ${node.label}`)
    console.log('Folder data:', node.data)
    // Implement folder properties dialog
  }

  private updateSelectionDisplay() {
    console.log(`Selected ${this.selectedFiles.length} items:`)
    
    this.selectedFiles.forEach(nodeId => {
      const node = this.tree.nodes.get(nodeId)
      if (node) {
        console.log(`- ${node.label} (${node.nodeType})`)
      }
    })
  }

  private updateSearchResults(query: string) {
    if (query) {
      const visibleNodes = this.tree.getVisibleNodes()
      console.log(`Search "${query}" found ${visibleNodes.length} results`)
    } else {
      console.log('Search cleared')
    }
  }

  // Public API methods
  public async expandPath(path: string) {
    const pathParts = path.split('/')
    let currentPath = ''
    
    for (const part of pathParts) {
      if (part) {
        currentPath = currentPath ? `${currentPath}/${part}` : part
        await this.tree.expand(currentPath)
      }
    }
  }

  public selectFile(nodeId: NodeId) {
    this.tree.select(nodeId)
  }

  public searchFiles(query: string) {
    this.tree.setSearchQuery(query)
  }

  public getSelectedFiles(): TreeNode[] {
    return this.tree.getSelectedNodes()
  }

  public getTree(): Tree {
    return this.tree
  }
}

// Usage example
const fileExplorer = new FileExplorer()

// Expand a specific path
await fileExplorer.expandPath('projects/my-app/src')

// Search for files
fileExplorer.searchFiles('.ts')

// Get the tree instance for rendering
const treeWidget = fileExplorer.getTree()
```

## Tree Patterns

### Navigation Tree Pattern

```typescript
import { treePatterns, TreeBuilder, createFolderNode, createLeafNode } from 'reactive-tui-ts'

// Use the built-in navigation pattern
const navigationTree = treePatterns.organization('nav-tree')
  .multiSelect(false)
  .searchEnabled(true)
  .onSelect((selectedNodes) => {
    if (selectedNodes.length > 0) {
      const nodeId = selectedNodes[0]
      navigateToRoute(nodeId)
    }
  })
  .build()

// Add navigation items
const dashboardNode = createLeafNode('dashboard', 'Dashboard')
dashboardNode.data = { route: '/dashboard', icon: '📊' }

const usersFolder = createFolderNode('users', 'Users')
usersFolder.data = { icon: '👥' }

navigationTree.addRootNode(dashboardNode)
navigationTree.addRootNode(usersFolder)

// Add user management subitems
navigationTree.addChildNode('users', createLeafNode('users-list', 'All Users'))
navigationTree.addChildNode('users', createLeafNode('users-add', 'Add User'))
navigationTree.addChildNode('users', createLeafNode('users-roles', 'User Roles'))

function navigateToRoute(nodeId: string) {
  const node = navigationTree.nodes.get(nodeId)
  if (node && node.data.route) {
    console.log(`Navigating to: ${node.data.route}`)
    // Implement navigation logic
  }
}
```

### File System Pattern

```typescript
// Use the built-in file system pattern
const fileSystemTree = treePatterns.fileSystem('file-system')
  .lazyLoading(true, async (nodeId) => {
    // Load directory contents
    return await loadDirectoryContents(nodeId)
  })
  .onSelect((selectedNodes) => {
    console.log('Selected files:', selectedNodes)
  })
  .build()

// Initialize with root directories
fileSystemTree.addRootNode(createFolderNode('/', 'Root'))
```

### Category Tree Pattern

```typescript
// Use the built-in categories pattern
const categoryTree = treePatterns.categories('categories')
  .multiSelect(true)
  .searchEnabled(true)
  .onSelect((selectedNodes) => {
    console.log('Selected categories:', selectedNodes)
    filterContentByCategories(selectedNodes)
  })
  .build()

// Add category hierarchy
const techCategory = createFolderNode('technology', 'Technology')
const webDevCategory = createFolderNode('web-dev', 'Web Development')
const frontendCategory = createLeafNode('frontend', 'Frontend')
const backendCategory = createLeafNode('backend', 'Backend')

categoryTree.addRootNode(techCategory)
categoryTree.addChildNode('technology', webDevCategory)
categoryTree.addChildNode('web-dev', frontendCategory)
categoryTree.addChildNode('web-dev', backendCategory)

function filterContentByCategories(categories: NodeId[]) {
  console.log('Filtering content by categories:', categories)
  // Implement content filtering logic
}
```

## CSS Styling

```css
/* Tree container */
.tree {
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  line-height: 1.5;
  user-select: none;
  background: #ffffff;
  border: 1px solid #e1e5e9;
  border-radius: 6px;
  overflow: hidden;
}

/* Tree nodes */
.tree-node {
  display: flex;
  align-items: center;
  padding: 0.375rem 0.75rem;
  cursor: pointer;
  transition: all 0.15s ease;
  border-bottom: 1px solid transparent;
}

.tree-node:hover {
  background-color: #f6f8fa;
  border-color: #e1e5e9;
}

.tree-node.selected {
  background-color: #0969da;
  color: white;
  font-weight: 500;
}

.tree-node.highlighted {
  background-color: #fff8c5;
  border-color: #d4ac0d;
}

.tree-node.disabled {
  opacity: 0.5;
  cursor: not-allowed;
  background-color: #f8f9fa;
}

.tree-node.loading {
  opacity: 0.7;
  cursor: wait;
}

/* Node content */
.tree-node-content {
  display: flex;
  align-items: center;
  flex: 1;
  min-width: 0;
}

/* Expand/collapse indicators */
.tree-expand-icon {
  margin-right: 0.5rem;
  font-size: 0.75rem;
  color: #656d76;
  width: 12px;
  text-align: center;
  transition: transform 0.15s ease;
}

.tree-expand-icon.expanded {
  transform: rotate(0deg);
}

.tree-expand-icon.collapsed {
  transform: rotate(-90deg);
}

/* Node icons */
.tree-node-icon {
  margin-right: 0.5rem;
  font-size: 1rem;
  width: 16px;
  text-align: center;
}

/* Node labels */
.tree-node-label {
  flex: 1;
  min-width: 0;
  overflow: hidden;
  text-overflow: ellipsis;
  white-space: nowrap;
}

.tree-node-description {
  margin-left: 0.5rem;
  font-size: 0.875rem;
  color: #656d76;
  font-style: italic;
}

/* Indentation lines */
.tree-indent {
  color: #d0d7de;
  font-size: 0.875rem;
  font-family: monospace;
  white-space: pre;
}

/* Node type styling */
.tree-node.tree-folder {
  font-weight: 500;
}

.tree-node.tree-folder .tree-node-icon {
  color: #8250df;
}

.tree-node.tree-leaf .tree-node-icon {
  color: #656d76;
}

.tree-node.tree-loading .tree-node-icon {
  animation: spin 1s linear infinite;
}

@keyframes spin {
  from { transform: rotate(0deg); }
  to { transform: rotate(360deg); }
}

/* Search highlighting */
.tree-node.search-match {
  background-color: #fff3cd;
  border-color: #ffeaa7;
}

.tree-node.search-match .tree-node-label {
  font-weight: 600;
}

/* Custom node classes */
.tree-node.custom-node {
  border-left: 3px solid #0969da;
}

.tree-node.important {
  border-left: 3px solid #d73a49;
}

/* Virtual scrolling viewport */
.tree-viewport {
  height: 400px;
  overflow-y: auto;
  scrollbar-width: thin;
  scrollbar-color: #d0d7de #f6f8fa;
}

.tree-viewport::-webkit-scrollbar {
  width: 8px;
}

.tree-viewport::-webkit-scrollbar-track {
  background: #f6f8fa;
}

.tree-viewport::-webkit-scrollbar-thumb {
  background-color: #d0d7de;
  border-radius: 4px;
}

.tree-viewport::-webkit-scrollbar-thumb:hover {
  background-color: #b1b7c1;
}

/* Loading states */
.tree-loading-overlay {
  position: absolute;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(255, 255, 255, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.875rem;
  color: #656d76;
}

/* Empty state */
.tree-empty {
  padding: 2rem;
  text-align: center;
  color: #656d76;
  font-style: italic;
}

/* Focus styles for accessibility */
.tree-node:focus {
  outline: 2px solid #0969da;
  outline-offset: -2px;
}

.tree:focus-within {
  border-color: #0969da;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .tree {
    background: #0d1117;
    border-color: #30363d;
    color: #e6edf3;
  }
  
  .tree-node:hover {
    background-color: #161b22;
  }
  
  .tree-node.selected {
    background-color: #1f6feb;
  }
  
  .tree-expand-icon,
  .tree-node-description {
    color: #8b949e;
  }
  
  .tree-indent {
    color: #30363d;
  }
}
```

## Best Practices

### 1. Use Appropriate Node Types

```typescript
// ✅ Good - semantic node types with proper structure
const fileSystemTree = tree('filesystem', {
  multiSelect: true,
  expandable: true,
  lazyLoading: true
})

// Create nodes with appropriate types
const documentFolder = createFolderNode('docs', 'Documents')
const readmeFile = createLeafNode('readme', 'README.md')
readmeFile.data = { type: 'markdown', size: '2048' }

fileSystemTree.addRootNode(documentFolder)
fileSystemTree.addChildNode('docs', readmeFile)
```

### 2. Implement Lazy Loading for Performance

```typescript
// ✅ Good - lazy loading with proper error handling
const lazyTree = tree('large-tree', {
  multiSelect: false,
  expandable: true,
  lazyLoading: true,
  virtualScrolling: true,
  maxVisibleNodes: 1000
})

lazyTree.lazyLoader = async (nodeId: NodeId) => {
  try {
    const children = await fetchChildNodes(nodeId)
    return children.map(child => 
      child.isDirectory 
        ? createFolderNode(child.id, child.name)
        : createLeafNode(child.id, child.name)
    )
  } catch (error) {
    console.error(`Failed to load children for ${nodeId}:`, error)
    return []
  }
}
```

### 3. Provide Clear User Feedback

```typescript
// ✅ Good - comprehensive feedback system
const interactiveTree = tree('interactive-tree', {
  multiSelect: true,
  expandable: true,
  searchEnabled: true
})

interactiveTree.callbacks = {
  onSelect: (selectedNodes) => {
    // Update UI to show selection count
    updateSelectionCounter(selectedNodes.length)
    
    // Enable/disable bulk actions
    toggleBulkActions(selectedNodes.length > 0)
  },
  
  onExpand: (nodeId, expanded) => {
    const node = interactiveTree.nodes.get(nodeId)
    if (node && expanded) {
      // Show loading indicator for lazy loading
      showLoadingIndicator(nodeId)
    }
  },
  
  onSearch: (query) => {
    // Update search results counter
    const visibleNodes = interactiveTree.getVisibleNodes()
    updateSearchResults(query, visibleNodes.length)
  }
}
```

### 4. Handle Large Datasets Efficiently

```typescript
// ✅ Good - virtual scrolling for large datasets
const largeDataTree = tree('large-data', {
  multiSelect: true,
  expandable: true,
  virtualScrolling: true,
  maxVisibleNodes: 500, // Limit visible nodes for performance
  searchEnabled: true
})

// Use proper node hierarchy to minimize memory usage
const createNodeHierarchy = (data: any[]) => {
  // Only create nodes that are initially visible
  return data.slice(0, 100).map(item => 
    item.isDirectory 
      ? createFolderNode(item.id, item.name)
      : createLeafNode(item.id, item.name)
  )
}
```

### 5. Implement Proper Error Handling

```typescript
// ✅ Good - robust error handling
class RobustTreeManager {
  private tree: Tree

  constructor() {
    this.tree = tree('robust-tree', {
      multiSelect: true,
      expandable: true,
      lazyLoading: true
    })

    this.tree.lazyLoader = this.safeLoadChildren.bind(this)
  }

  private async safeLoadChildren(nodeId: NodeId): Promise<TreeNode[]> {
    try {
      const children = await this.loadChildren(nodeId)
      return children
    } catch (error) {
      console.error(`Failed to load children for ${nodeId}:`, error)
      
      // Show error node
      const errorNode = createLeafNode(`${nodeId}-error`, '⚠️ Failed to load')
      errorNode.disabled = true
      errorNode.data = { error: error.message }
      
      return [errorNode]
    }
  }

  private async loadChildren(nodeId: NodeId): Promise<TreeNode[]> {
    // Actual loading implementation
    const response = await fetch(`/api/tree/${nodeId}/children`)
    if (!response.ok) {
      throw new Error(`HTTP ${response.status}: ${response.statusText}`)
    }
    
    const data = await response.json()
    return data.map((item: any) => 
      item.isDirectory 
        ? createFolderNode(item.id, item.name)
        : createLeafNode(item.id, item.name)
    )
  }
}
```

## Related Widgets

- **[DataTable](./datatable)** - Tabular data display with sorting and filtering
- **[ScrollableList](./scrollable-list)** - Simple list display
- **[Menu](./menu)** - Navigation menus and dropdowns
- **[Accordion](./accordion)** - Collapsible content sections
- **[Progress](./progress)** - Loading indicators for lazy loading

## Examples

- **[File Explorer](../../examples/advanced/file-manager)** - Complete file browser with lazy loading
- **[Navigation Tree](../../examples/basic/navigation)** - Application navigation sidebar
- **[Code Explorer](../../examples/advanced/code-explorer)** - Source code browser
- **[Category Browser](../../examples/basic/categories)** - Hierarchical category selection

The Tree widget provides comprehensive hierarchical data display functionality with excellent performance characteristics for large datasets, making it ideal for file systems, navigation structures, and organizational hierarchies.
