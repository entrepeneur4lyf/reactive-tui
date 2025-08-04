# Viewport Widget

The Viewport widget provides a comprehensive scrollable viewport with virtual scrolling, lazy loading, efficient rendering for large datasets (10k+ items), smooth scrolling, keyboard navigation, and intelligent content caching with LRU eviction.

## Basic Usage

```typescript
import { Viewport, ViewportBuilder, ViewportItem } from 'reactive-tui';

// Basic viewport with content
const items: ViewportItem[] = [
  { id: 'item1', content: 'First item', selectable: true },
  { id: 'item2', content: 'Second item', selectable: true },
  { id: 'item3', content: 'Third item', selectable: true }
];

const viewport = new Viewport('my-viewport', items, {
  width: 80,
  height: 25,
  virtual_scrolling: true,
  selection_mode: SelectionMode.Single
});

// Using the builder pattern
const builderViewport = new ViewportBuilder('builder-viewport')
  .width(100)
  .height(30)
  .content(items)
  .virtualScrolling(true)
  .selectionMode(SelectionMode.Multiple)
  .showScrollbar(true)
  .build();
```

## Configuration

### ViewportConfig Interface

```typescript
interface ViewportConfig {
  width: number;                    // Display width (default: 80)
  height: number;                   // Display height (default: 25)
  scrollable: boolean;              // Enable scrolling (default: true)
  virtual_scrolling: boolean;       // Enable virtual scrolling (default: true)
  lazy_loading: boolean;            // Enable lazy loading (default: false)
  show_scrollbar: boolean;          // Show scrollbar (default: true)
  scrollbar_position: ScrollbarPosition; // Scrollbar position (default: Right)
  scroll_mode: ScrollMode;          // Scroll animation mode (default: Smooth)
  selection_mode: SelectionMode;    // Selection behavior (default: Single)
  item_height: number;              // Height per item (default: 1)
  overscan_count: number;           // Extra items to render (default: 5)
  cache_size: number;               // Cache size limit (default: 1000)
  scroll_sensitivity: number;       // Mouse wheel sensitivity (default: 3)
  momentum_decay: number;           // Scroll momentum decay (default: 0.95)
  smooth_scroll_duration: number;   // Smooth scroll duration (default: 200)
  search_highlight_color: string;   // Search highlight color (default: '#ffff00')
  selection_color: string;          // Selection color (default: '#0066cc')
  highlight_color: string;          // Highlight color (default: '#333333')
}
```

### ViewportItem Interface

```typescript
interface ViewportItem {
  id: ContentId;                    // Unique identifier
  content: string;                  // Display content
  height?: number;                  // Custom height
  selectable?: boolean;             // Can be selected
  metadata?: Record<string, any>;   // Additional data
}
```

### Enums

```typescript
enum ScrollMode {
  Instant = 'instant',             // Immediate scrolling
  Smooth = 'smooth',              // Animated scrolling
  Auto = 'auto'                   // Context-dependent
}

enum SelectionMode {
  None = 'none',                  // No selection
  Single = 'single',              // Single selection
  Multiple = 'multiple'           // Multiple selection
}

enum ScrollbarPosition {
  Right = 'right',                // Right side scrollbar
  Left = 'left',                  // Left side scrollbar
  Hidden = 'hidden'               // No scrollbar
}

enum LazyLoadState {
  NotLoaded = 'not_loaded',       // Not yet loaded
  Loading = 'loading',            // Currently loading
  Loaded = 'loaded',              // Successfully loaded
  Error = 'error'                 // Load failed
}
```

## Core Features

### Virtual Scrolling

```typescript
// Large dataset with virtual scrolling
const largeDataset: ViewportItem[] = Array.from({ length: 10000 }, (_, i) => ({
  id: `item-${i}`,
  content: `Item ${i + 1}: Lorem ipsum dolor sit amet`,
  selectable: true
}));

const virtualViewport = new ViewportBuilder('virtual')
  .content(largeDataset)
  .width(100)
  .height(30)
  .virtualScrolling(true)        // Only render visible items
  .itemHeight(1)                 // Height per item
  .cacheSize(500)               // Cache 500 items
  .build();

// Performance monitoring
console.log('Cache stats:', virtualViewport.getCacheStats());
// { hits: 450, misses: 50, evictions: 0, size: 500, max_size: 500 }
```

### Lazy Loading

```typescript
// Async lazy loading with callbacks
const lazyViewport = new ViewportBuilder('lazy')
  .width(80)
  .height(25)
  .lazyLoading(true)
  .onLazyLoad(async (startLine, count, viewport) => {
    console.log(`Loading ${count} items starting at line ${startLine}`);
    
    // Simulate API call
    await new Promise(resolve => setTimeout(resolve, 100));
    
    // Return loaded items
    return Array.from({ length: count }, (_, i) => ({
      id: `lazy-${startLine + i}`,
      content: `Lazy loaded item ${startLine + i}`,
      selectable: true,
      metadata: { loadTime: Date.now() }
    }));
  })
  .build();

// Load initial placeholder items
const placeholders: ViewportItem[] = Array.from({ length: 1000 }, (_, i) => ({
  id: `placeholder-${i}`,
  content: 'Loading...',
  selectable: false
}));

lazyViewport.setItems(placeholders);
```

### Selection Management

```typescript
const selectableViewport = new ViewportBuilder('selectable')
  .contentFromStrings([
    'First selectable item',
    'Second selectable item', 
    'Third selectable item',
    'Fourth selectable item'
  ])
  .selectionMode(SelectionMode.Multiple)
  .onSelectionChange((selectedIds, viewport) => {
    console.log('Selection changed:', selectedIds);
    console.log('Selected count:', selectedIds.length);
  })
  .build();

// Selection operations
selectableViewport.selectItem('line-0');          // Select first item
selectableViewport.selectItem('line-2');          // Add third item
selectableViewport.toggleSelection('line-1');     // Toggle second item
selectableViewport.selectAll();                   // Select all items
selectableViewport.clearSelection();              // Clear selection

// Get selection state
const selected = selectableViewport.getSelectedItems();
console.log('Currently selected:', selected);
```

### Scrolling Operations

```typescript
const scrollViewport = new ViewportBuilder('scroll')
  .contentFromStrings(Array.from({ length: 100 }, (_, i) => `Line ${i + 1}`))
  .width(80)
  .height(25)
  .onScroll((position, viewport) => {
    const currentLine = viewport.getCurrentLine();
    const totalLines = viewport.getTotalLines();
    console.log(`Scrolled to line ${currentLine}/${totalLines}`);
  })
  .build();

// Scrolling methods
scrollViewport.scrollTo(50);           // Scroll to position 50
scrollViewport.scrollToLine(25);       // Scroll to line 25
scrollViewport.scrollUp(5);            // Scroll up 5 lines
scrollViewport.scrollDown(3);          // Scroll down 3 lines
scrollViewport.pageUp();               // Page up
scrollViewport.pageDown();             // Page down
scrollViewport.scrollToTop();          // Go to beginning
scrollViewport.scrollToBottom();       // Go to end

// Get scroll information
const currentLine = scrollViewport.getCurrentLine();
const totalLines = scrollViewport.getTotalLines();
console.log(`Position: ${currentLine}/${totalLines}`);
```

### Search Functionality

```typescript
const searchViewport = new ViewportBuilder('search')
  .contentFromStrings([
    'function calculateTotal(items) {',
    '  return items.reduce((sum, item) => sum + item.price, 0);',
    '}',
    '',
    'function formatCurrency(amount) {',
    '  return `$${amount.toFixed(2)}`;',
    '}',
    '',
    'const total = calculateTotal(shoppingCart);',
    'console.log(formatCurrency(total));'
  ])
  .build();

// Built-in search
const resultCount = searchViewport.search('function');
console.log(`Found ${resultCount} matches`);

// Navigate search results
searchViewport.nextSearchResult();     // Go to next match
searchViewport.previousSearchResult(); // Go to previous match

// Custom search callback
const customSearchViewport = new ViewportBuilder('custom-search')
  .content(items)
  .onSearch((query, items, viewport) => {
    // Custom search logic - return array of line indices
    const results: number[] = [];
    const regex = new RegExp(query, 'gi');
    
    items.forEach((item, index) => {
      if (regex.test(item.content) || regex.test(item.metadata?.tags || '')) {
        results.push(index);
      }
    });
    
    return results;
  })
  .build();
```

## Advanced Features

### Content Management

```typescript
const managedViewport = new Viewport('managed', []);

// Dynamic content operations
managedViewport.addItem({
  id: 'new-item',
  content: 'Dynamically added item',
  selectable: true,
  metadata: { timestamp: Date.now() }
});

managedViewport.addItem({
  id: 'inserted-item', 
  content: 'Inserted at position 0',
  selectable: true
}, 0); // Insert at beginning

// Remove item
const removed = managedViewport.removeItem('new-item');
console.log('Removed item:', removed);

// Get specific item
const item = managedViewport.getItem('inserted-item');
console.log('Retrieved item:', item);

// Bulk update
const newItems: ViewportItem[] = [
  { id: 'bulk1', content: 'Bulk item 1', selectable: true },
  { id: 'bulk2', content: 'Bulk item 2', selectable: true },
  { id: 'bulk3', content: 'Bulk item 3', selectable: true }
];
managedViewport.setItems(newItems);
```

### Event Handling

```typescript
const eventViewport = new ViewportBuilder('events')
  .contentFromStrings(['Item 1', 'Item 2', 'Item 3'])
  .onScroll((position, viewport) => {
    console.log(`Scrolled to position: ${position}`);
  })
  .onSelectionChange((selectedIds, viewport) => {
    console.log(`Selection: ${selectedIds.join(', ')}`);
  })
  .onItemActivate((id, item, viewport) => {
    console.log(`Activated item: ${id} - ${item.content}`);
  })
  .build();

// Keyboard event handling
eventViewport.handleKeyPress('ArrowDown'); // Scroll down
eventViewport.handleKeyPress('j');         // Vim-style down
eventViewport.handleKeyPress('k');         // Vim-style up
eventViewport.handleKeyPress('PageDown');  // Page down
eventViewport.handleKeyPress('g');         // Go to top
eventViewport.handleKeyPress('G');         // Go to bottom
eventViewport.handleKeyPress('Enter');     // Activate highlighted item
eventViewport.handleKeyPress(' ');         // Toggle selection
eventViewport.handleKeyPress('a');         // Select all (multi-select mode)
eventViewport.handleKeyPress('Escape');    // Clear selection

// Mouse wheel handling
eventViewport.handleMouseWheel(3);         // Scroll down
eventViewport.handleMouseWheel(-2);        // Scroll up
```

### Performance Monitoring

```typescript
// Cache performance monitoring
const performanceViewport = new ViewportBuilder('performance')
  .content(largeDataset)
  .cacheSize(1000)
  .virtualScrolling(true)
  .build();

// Monitor cache performance
setInterval(() => {
  const stats = performanceViewport.getCacheStats();
  const hitRate = (stats.hits / (stats.hits + stats.misses) * 100).toFixed(1);
  console.log(`Cache: ${stats.size}/${stats.max_size} items, ${hitRate}% hit rate`);
}, 5000);

// Memory usage monitoring
const visibleItems = performanceViewport.getVisibleItems();
console.log(`Rendering ${visibleItems.length} visible items out of ${performanceViewport.getTotalLines()}`);
```

## Builder Pattern

```typescript
// Comprehensive builder configuration
const advancedViewport = new ViewportBuilder('advanced')
  .width(120)
  .height(40)
  .contentFromStrings(Array.from({ length: 5000 }, (_, i) => `Line ${i + 1}`))
  .scrollable(true)
  .virtualScrolling(true)
  .selectionMode(SelectionMode.Multiple)
  .showScrollbar(true)
  .scrollbarPosition(ScrollbarPosition.Right)
  .itemHeight(1)
  .cacheSize(500)
  .lazyLoading(false)
  .onScroll((pos, viewport) => console.log(`Scroll: ${pos}`))
  .onSelectionChange((ids, viewport) => console.log(`Selected: ${ids.length}`))
  .onItemActivate((id, item, viewport) => console.log(`Activated: ${id}`))
  .build();
```

## Convenience Functions

```typescript
// Pre-configured viewports for common use cases

// File viewer
const fileContent = [
  'import { Component } from "react";',
  '',
  'class MyComponent extends Component {',
  '  render() {',
  '    return <div>Hello World</div>;',
  '  }',
  '}'
];
const fileViewport = fileViewer(fileContent);

// Log viewer
const logEntries = [
  '[2024-01-01 10:00:00] INFO: Application started',
  '[2024-01-01 10:00:01] DEBUG: Loading configuration',
  '[2024-01-01 10:00:02] INFO: Database connected',
  '[2024-01-01 10:00:03] WARN: Cache miss for key "user:123"',
  '[2024-01-01 10:00:04] ERROR: Failed to process request'
];
const logViewport = logViewer(logEntries);

// Data table viewport
const dataItems: ViewportItem[] = [
  { id: 'row1', content: 'John Doe    | 30 | Engineer', selectable: true },
  { id: 'row2', content: 'Jane Smith  | 25 | Designer', selectable: true },
  { id: 'row3', content: 'Bob Johnson | 35 | Manager',  selectable: true }
];
const tableViewport = dataTableViewport(dataItems);
```

## Real-World Examples

### File Explorer Viewport

```typescript
import { Viewport, ViewportBuilder, ViewportItem, SelectionMode } from 'reactive-tui';

interface FileItem {
  name: string;
  type: 'file' | 'directory';
  size?: number;
  modified: Date;
  permissions: string;
}

class FileExplorerViewport {
  private viewport: Viewport;
  private currentPath: string = '/';
  private files: FileItem[] = [];
  
  constructor() {
    this.viewport = new ViewportBuilder('file-explorer')
      .width(100)
      .height(30)
      .virtualScrolling(true)
      .selectionMode(SelectionMode.Multiple)
      .showScrollbar(true)
      .onItemActivate((id, item, viewport) => this.handleItemActivation(id, item))
      .onSelectionChange((ids, viewport) => this.handleSelectionChange(ids))
      .build();
  }
  
  loadDirectory(path: string) {
    this.currentPath = path;
    // Simulate file system API call
    this.files = this.getDirectoryContents(path);
    
    const items: ViewportItem[] = this.files.map((file, index) => ({
      id: `file-${index}`,
      content: this.formatFileItem(file),
      selectable: true,
      metadata: { file, path: `${path}/${file.name}` }
    }));
    
    this.viewport.setItems(items);
  }
  
  private formatFileItem(file: FileItem): string {
    const icon = file.type === 'directory' ? '📁' : '📄';
    const size = file.size ? this.formatFileSize(file.size) : '';
    const modified = file.modified.toLocaleDateString();
    
    return `${icon} ${file.name.padEnd(30)} ${size.padEnd(10)} ${modified} ${file.permissions}`;
  }
  
  private formatFileSize(bytes: number): string {
    const units = ['B', 'KB', 'MB', 'GB'];
    let size = bytes;
    let unitIndex = 0;
    
    while (size >= 1024 && unitIndex < units.length - 1) {
      size /= 1024;
      unitIndex++;
    }
    
    return `${size.toFixed(1)} ${units[unitIndex]}`;
  }
  
  private handleItemActivation(id: string, item: ViewportItem) {
    const file = item.metadata?.file as FileItem;
    if (!file) return;
    
    if (file.type === 'directory') {
      // Navigate into directory
      this.loadDirectory(`${this.currentPath}/${file.name}`);
    } else {
      // Open file
      console.log(`Opening file: ${file.name}`);
    }
  }
  
  private handleSelectionChange(selectedIds: string[]) {
    const selectedFiles = selectedIds
      .map(id => this.viewport.getItem(id)?.metadata?.file as FileItem)
      .filter(Boolean);
    
    console.log(`Selected ${selectedFiles.length} items:`);
    selectedFiles.forEach(file => console.log(`  ${file.name}`));
  }
  
  private getDirectoryContents(path: string): FileItem[] {
    // Mock file system data
    return [
      {
        name: 'documents',
        type: 'directory',
        modified: new Date('2024-01-15T10:30:00'),
        permissions: 'drwxr-xr-x'
      },
      {
        name: 'photos',
        type: 'directory', 
        modified: new Date('2024-01-14T15:45:00'),
        permissions: 'drwxr-xr-x'
      },
      {
        name: 'readme.txt',
        type: 'file',
        size: 1024,
        modified: new Date('2024-01-10T09:15:00'),
        permissions: '-rw-r--r--'
      },
      {
        name: 'config.json',
        type: 'file',
        size: 512,
        modified: new Date('2024-01-12T14:20:00'),
        permissions: '-rw-r--r--'
      }
    ];
  }
  
  // Navigation methods
  navigateUp() {
    const parentPath = this.currentPath.split('/').slice(0, -1).join('/') || '/';
    this.loadDirectory(parentPath);
  }
  
  refresh() {
    this.loadDirectory(this.currentPath);
  }
  
  searchFiles(query: string) {
    return this.viewport.search(query);
  }
  
  getSelectedFiles(): FileItem[] {
    return this.viewport.getSelectedItems()
      .map(id => this.viewport.getItem(id)?.metadata?.file as FileItem)
      .filter(Boolean);
  }
  
  getCurrentPath(): string {
    return this.currentPath;
  }
  
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'Backspace':
        this.navigateUp();
        return true;
      case 'F5':
        this.refresh();
        return true;
      default:
        return this.viewport.handleKeyPress(key);
    }
  }
  
  render(): string[] {
    // In a real implementation, this would render the viewport
    return [`Current path: ${this.currentPath}`, ...this.viewport.render()];
  }
}

// Usage
const fileExplorer = new FileExplorerViewport();
fileExplorer.loadDirectory('/home/user');
fileExplorer.searchFiles('config');
```

### Log Analysis Viewport

```typescript
interface LogEntry {
  timestamp: Date;
  level: 'DEBUG' | 'INFO' | 'WARN' | 'ERROR';
  message: string;
  source: string;
  metadata?: Record<string, any>;
}

class LogAnalysisViewport {
  private viewport: Viewport;
  private logs: LogEntry[] = [];
  private filters: {
    level?: string;
    source?: string;
    dateRange?: { start: Date; end: Date };
  } = {};
  
  constructor() {
    this.viewport = new ViewportBuilder('log-analysis')
      .width(120)
      .height(40)
      .virtualScrolling(true)
      .selectionMode(SelectionMode.Single)
      .showScrollbar(true)
      .lazyLoading(true)
      .onLazyLoad((startLine, count, viewport) => this.loadLogChunk(startLine, count))
      .onItemActivate((id, item, viewport) => this.showLogDetails(id, item))
      .onSearch((query, items, viewport) => this.customLogSearch(query, items))
      .build();
  }
  
  loadLogs(logs: LogEntry[]) {
    this.logs = logs;
    this.applyFilters();
  }
  
  private async loadLogChunk(startLine: number, count: number): Promise<ViewportItem[]> {
    // Simulate loading logs from a large file or API
    await new Promise(resolve => setTimeout(resolve, 50));
    
    const chunk = this.logs.slice(startLine, startLine + count);
    return chunk.map((log, index) => ({
      id: `log-${startLine + index}`,
      content: this.formatLogEntry(log),
      selectable: true,
      metadata: { log, lineNumber: startLine + index }
    }));
  }
  
  private formatLogEntry(log: LogEntry): string {
    const timestamp = log.timestamp.toISOString().slice(11, 23); // HH:MM:SS.sss
    const level = log.level.padEnd(5);
    const source = log.source.padEnd(15);
    const message = log.message.length > 60 ? log.message.slice(0, 57) + '...' : log.message;
    
    return `${timestamp} [${level}] ${source} ${message}`;
  }
  
  private customLogSearch(query: string, items: ViewportItem[]): number[] {
    const results: number[] = [];
    const searchTerm = query.toLowerCase();
    
    items.forEach((item, index) => {
      const log = item.metadata?.log as LogEntry;
      if (!log) return;
      
      const searchableText = [
        log.level.toLowerCase(),
        log.message.toLowerCase(),
        log.source.toLowerCase(),
        JSON.stringify(log.metadata || {})
      ].join(' ');
      
      if (searchableText.includes(searchTerm)) {
        results.push(index);
      }
    });
    
    return results;
  }
  
  private showLogDetails(id: string, item: ViewportItem) {
    const log = item.metadata?.log as LogEntry;
    if (!log) return;
    
    console.log('=== Log Entry Details ===');
    console.log(`Timestamp: ${log.timestamp.toISOString()}`);
    console.log(`Level: ${log.level}`);
    console.log(`Source: ${log.source}`);
    console.log(`Message: ${log.message}`);
    if (log.metadata) {
      console.log(`Metadata: ${JSON.stringify(log.metadata, null, 2)}`);
    }
    console.log('========================');
  }
  
  applyFilters() {
    let filteredLogs = [...this.logs];
    
    if (this.filters.level) {
      filteredLogs = filteredLogs.filter(log => log.level === this.filters.level);
    }
    
    if (this.filters.source) {
      filteredLogs = filteredLogs.filter(log => log.source.includes(this.filters.source!));
    }
    
    if (this.filters.dateRange) {
      filteredLogs = filteredLogs.filter(log => 
        log.timestamp >= this.filters.dateRange!.start &&
        log.timestamp <= this.filters.dateRange!.end
      );
    }
    
    const items: ViewportItem[] = filteredLogs.map((log, index) => ({
      id: `log-${index}`,
      content: this.formatLogEntry(log),
      selectable: true,
      metadata: { log, lineNumber: index }
    }));
    
    this.viewport.setItems(items);
  }
  
  filterByLevel(level: string | undefined) {
    this.filters.level = level;
    this.applyFilters();
  }
  
  filterBySource(source: string | undefined) {
    this.filters.source = source;
    this.applyFilters();
  }
  
  filterByDateRange(start?: Date, end?: Date) {
    if (start && end) {
      this.filters.dateRange = { start, end };
    } else {
      this.filters.dateRange = undefined;
    }
    this.applyFilters();
  }
  
  clearFilters() {
    this.filters = {};
    this.applyFilters();
  }
  
  searchLogs(query: string): number {
    return this.viewport.search(query);
  }
  
  exportSelectedLogs(): LogEntry[] {
    return this.viewport.getSelectedItems()
      .map(id => this.viewport.getItem(id)?.metadata?.log as LogEntry)
      .filter(Boolean);
  }
  
  getLogStats() {
    const levels = this.logs.reduce((acc, log) => {
      acc[log.level] = (acc[log.level] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);
    
    const sources = this.logs.reduce((acc, log) => {
      acc[log.source] = (acc[log.source] || 0) + 1;
      return acc;
    }, {} as Record<string, number>);
    
    return {
      totalLogs: this.logs.length,
      levels,
      sources,
      timeRange: {
        start: Math.min(...this.logs.map(l => l.timestamp.getTime())),
        end: Math.max(...this.logs.map(l => l.timestamp.getTime()))
      }
    };
  }
  
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'f':
        // Toggle filters menu
        console.log('Opening filters menu...');
        return true;
      case 'e':
        // Export selected logs
        const selected = this.exportSelectedLogs();
        console.log(`Exporting ${selected.length} logs`);
        return true;
      case 's':
        // Show statistics
        console.log('Log Statistics:', this.getLogStats());
        return true;
      default:
        return this.viewport.handleKeyPress(key);
    }
  }
}

// Usage
const logAnalyzer = new LogAnalysisViewport();

// Sample log data
const sampleLogs: LogEntry[] = [
  {
    timestamp: new Date('2024-01-01T10:00:00Z'),
    level: 'INFO',
    message: 'Application started successfully',
    source: 'main',
    metadata: { version: '1.0.0' }
  },
  {
    timestamp: new Date('2024-01-01T10:00:01Z'),
    level: 'DEBUG',
    message: 'Loading configuration from config.json',
    source: 'config'
  },
  {
    timestamp: new Date('2024-01-01T10:00:02Z'),
    level: 'ERROR',
    message: 'Failed to connect to database',
    source: 'database',
    metadata: { error: 'Connection timeout', retries: 3 }
  }
];

logAnalyzer.loadLogs(sampleLogs);
logAnalyzer.filterByLevel('ERROR');
logAnalyzer.searchLogs('database');
```

### Data Grid Viewport

```typescript
interface DataGridColumn {
  key: string;
  title: string;
  width: number;
  sortable?: boolean;
  formatter?: (value: any) => string;
}

interface DataGridRow {
  id: string;
  data: Record<string, any>;
}

class DataGridViewport {
  private viewport: Viewport;
  private columns: DataGridColumn[] = [];
  private rows: DataGridRow[] = [];
  private sortColumn?: string;
  private sortDirection: 'asc' | 'desc' = 'asc';
  
  constructor(columns: DataGridColumn[]) {
    this.columns = columns;
    
    this.viewport = new ViewportBuilder('data-grid')
      .width(this.calculateTotalWidth())
      .height(25)
      .virtualScrolling(true)
      .selectionMode(SelectionMode.Multiple)
      .showScrollbar(true)
      .onItemActivate((id, item, viewport) => this.handleRowActivation(id, item))
      .onSelectionChange((ids, viewport) => this.handleSelectionChange(ids))
      .build();
  }
  
  private calculateTotalWidth(): number {
    return this.columns.reduce((sum, col) => sum + col.width, 0) + 2; // +2 for borders
  }
  
  setData(rows: DataGridRow[]) {
    this.rows = rows;
    this.refreshDisplay();
  }
  
  private refreshDisplay() {
    // Create header row
    const headerItem: ViewportItem = {
      id: 'header',
      content: this.formatHeaderRow(),
      selectable: false,
      metadata: { isHeader: true }
    };
    
    // Create data rows
    const dataItems: ViewportItem[] = this.rows.map(row => ({
      id: row.id,
      content: this.formatDataRow(row),
      selectable: true,
      metadata: { row, isData: true }
    }));
    
    this.viewport.setItems([headerItem, ...dataItems]);
  }
  
  private formatHeaderRow(): string {
    return '│' + this.columns.map(col => {
      let title = col.title;
      if (this.sortColumn === col.key) {
        title += this.sortDirection === 'asc' ? ' ↑' : ' ↓';
      }
      return title.padEnd(col.width - 1).slice(0, col.width - 1);
    }).join('│') + '│';
  }
  
  private formatDataRow(row: DataGridRow): string {
    return '│' + this.columns.map(col => {
      let value = row.data[col.key];
      
      if (col.formatter) {
        value = col.formatter(value);
      } else if (value === null || value === undefined) {
        value = '';
      } else {
        value = String(value);
      }
      
      return value.padEnd(col.width - 1).slice(0, col.width - 1);
    }).join('│') + '│';
  }
  
  sortBy(columnKey: string) {
    const column = this.columns.find(col => col.key === columnKey);
    if (!column || !column.sortable) return;
    
    if (this.sortColumn === columnKey) {
      this.sortDirection = this.sortDirection === 'asc' ? 'desc' : 'asc';
    } else {
      this.sortColumn = columnKey;
      this.sortDirection = 'asc';
    }
    
    this.rows.sort((a, b) => {
      const aVal = a.data[columnKey];
      const bVal = b.data[columnKey];
      
      let comparison = 0;
      if (aVal < bVal) comparison = -1;
      else if (aVal > bVal) comparison = 1;
      
      return this.sortDirection === 'asc' ? comparison : -comparison;
    });
    
    this.refreshDisplay();
  }
  
  private handleRowActivation(id: string, item: ViewportItem) {
    if (item.metadata?.isHeader) {
      // Header clicked - sort by column
      const clickPosition = 0; // Would calculate from mouse position
      const columnIndex = this.getColumnAtPosition(clickPosition);
      if (columnIndex >= 0) {
        this.sortBy(this.columns[columnIndex].key);
      }
    } else if (item.metadata?.isData) {
      // Data row activated
      const row = item.metadata.row as DataGridRow;
      console.log('Row activated:', row);
    }
  }
  
  private getColumnAtPosition(position: number): number {
    let currentPos = 1; // Start after first border
    for (let i = 0; i < this.columns.length; i++) {
      if (position >= currentPos && position < currentPos + this.columns[i].width) {
        return i;
      }
      currentPos += this.columns[i].width + 1; // +1 for border
    }
    return -1;
  }
  
  private handleSelectionChange(selectedIds: string[]) {
    const selectedRows = selectedIds
      .map(id => this.viewport.getItem(id)?.metadata?.row as DataGridRow)
      .filter(Boolean);
    
    console.log(`Selected ${selectedRows.length} rows`);
  }
  
  getSelectedRows(): DataGridRow[] {
    return this.viewport.getSelectedItems()
      .map(id => this.viewport.getItem(id)?.metadata?.row as DataGridRow)
      .filter(Boolean);
  }
  
  searchGrid(query: string): number {
    return this.viewport.search(query);
  }
  
  handleKeyPress(key: string): boolean {
    return this.viewport.handleKeyPress(key);
  }
  
  render(): string[] {
    return this.viewport.render();
  }
}

// Usage
const columns: DataGridColumn[] = [
  { key: 'id', title: 'ID', width: 8, sortable: true },
  { key: 'name', title: 'Name', width: 20, sortable: true },
  { key: 'email', title: 'Email', width: 25, sortable: true },
  { key: 'age', title: 'Age', width: 6, sortable: true },
  { 
    key: 'salary', 
    title: 'Salary', 
    width: 12, 
    sortable: true,
    formatter: (value) => `$${value.toLocaleString()}`
  }
];

const dataGrid = new DataGridViewport(columns);

const rows: DataGridRow[] = [
  { id: '1', data: { id: 1, name: 'John Doe', email: 'john@example.com', age: 30, salary: 75000 } },
  { id: '2', data: { id: 2, name: 'Jane Smith', email: 'jane@example.com', age: 25, salary: 68000 } },
  { id: '3', data: { id: 3, name: 'Bob Johnson', email: 'bob@example.com', age: 35, salary: 82000 } }
];

dataGrid.setData(rows);
dataGrid.sortBy('name');
dataGrid.searchGrid('john');
```

## Performance Considerations

```typescript
// Performance monitoring and optimization
const performanceViewport = new ViewportBuilder('performance')
  .content(largeDataset)
  .virtualScrolling(true)      // Only render visible items
  .cacheSize(1000)             // Reasonable cache size
  .itemHeight(1)               // Consistent item height
  .build();

// Monitor performance
setInterval(() => {
  const stats = performanceViewport.getCacheStats();
  const hitRate = stats.hits / (stats.hits + stats.misses);
  
  if (hitRate < 0.8) {
    console.warn('Low cache hit rate:', hitRate);
  }
  
  if (stats.evictions > 100) {
    console.warn('High eviction rate:', stats.evictions);
  }
}, 10000);

// Optimize for large datasets
const optimizedViewport = new ViewportBuilder('optimized')
  .content(veryLargeDataset)
  .virtualScrolling(true)      // Essential for large datasets
  .cacheSize(2000)             // Larger cache
  .lazyLoading(true)           // Load on demand
  .itemHeight(1)               // Fixed height for better performance
  .build();
```

## Best Practices

1. **Virtual Scrolling**
   - Always enable for datasets > 1000 items
   - Use consistent item heights when possible
   - Monitor cache hit rates

2. **Lazy Loading**
   - Implement for very large or remote datasets
   - Provide loading indicators
   - Handle errors gracefully

3. **Selection Management**
   - Use appropriate selection modes
   - Provide clear visual feedback
   - Handle selection changes efficiently

4. **Performance Optimization**
   - Set reasonable cache sizes
   - Monitor memory usage
   - Use virtual scrolling for large datasets

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const container = new ElementBuilderImpl('div')
  .class('viewport-container')
  .child(
    new ViewportBuilder('integrated')
      .contentFromStrings(['Line 1', 'Line 2', 'Line 3'])
      .width(80)
      .height(25)
      .build()
  )
  .build();
```

The Viewport widget provides comprehensive scrollable content display with virtual scrolling, lazy loading, selection management, and efficient rendering for creating high-performance data display interfaces in terminal applications.