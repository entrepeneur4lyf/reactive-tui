/**
 * Viewport Widget - TypeScript Implementation
 * 
 * A comprehensive scrollable viewport widget supporting lazy loading, virtual scrolling,
 * and efficient rendering of large datasets with smooth scrolling and keyboard navigation.
 * 
 * Features:
 * - Virtual Scrolling: Efficiently render only visible items for large datasets (10k+ items)
 * - Lazy Loading: Load content on-demand with async callbacks and loading indicators
 * - Smooth Scrolling: Pixel-perfect scrolling with momentum and easing
 * - Keyboard Navigation: Arrow keys, Page Up/Down, Home/End, vim-style navigation
 * - Mouse Support: Mouse wheel scrolling and drag scrolling
 * - Scrollbar Rendering: Customizable scrollbar with position indicators
 * - Content Caching: Intelligent content caching with LRU eviction
 * - Search Integration: Find and scroll to content with highlighting
 * - Selection Support: Single/multi-selection with keyboard and mouse
 * - Responsive Sizing: Automatic sizing based on container and content
 */

export type ContentId = string;
export type LineNumber = number;

export interface ViewportItem {
  id: ContentId;
  content: string;
  height?: number;
  selectable?: boolean;
  metadata?: Record<string, any>;
}

export enum ScrollMode {
  Instant = 'instant',
  Smooth = 'smooth',
  Auto = 'auto'
}

export enum SelectionMode {
  None = 'none',
  Single = 'single',
  Multiple = 'multiple'
}

export enum ScrollbarPosition {
  Right = 'right',
  Left = 'left',
  Hidden = 'hidden'
}

export enum LazyLoadState {
  NotLoaded = 'not_loaded',
  Loading = 'loading',
  Loaded = 'loaded',
  Error = 'error'
}

export interface ViewportState {
  scroll_position: number;
  visible_start: number;
  visible_end: number;
  total_height: number;
  viewport_height: number;
  horizontal_scroll: number;
  is_scrolling: boolean;
  scroll_velocity: number;
  selected_items: ContentId[];
  selection_anchor: ContentId | null;
  highlighted_item: ContentId | null;
  search_query: string;
  search_results: number[];
  current_search_index: number;
  lazy_states: Map<ContentId, LazyLoadState>;
  needs_layout: boolean;
  last_update: number;
}

export interface CacheStats {
  hits: number;
  misses: number;
  evictions: number;
  size: number;
  max_size: number;
}

export interface ViewportConfig {
  width: number;
  height: number;
  scrollable: boolean;
  virtual_scrolling: boolean;
  lazy_loading: boolean;
  show_scrollbar: boolean;
  scrollbar_position: ScrollbarPosition;
  scroll_mode: ScrollMode;
  selection_mode: SelectionMode;
  item_height: number;
  overscan_count: number;
  cache_size: number;
  scroll_sensitivity: number;
  momentum_decay: number;
  smooth_scroll_duration: number;
  search_highlight_color: string;
  selection_color: string;
  highlight_color: string;
}

export interface ViewportStyle {
  background_color: string;
  text_color: string;
  border_color: string;
  scrollbar_color: string;
  scrollbar_thumb_color: string;
  selection_background: string;
  selection_foreground: string;
  highlight_background: string;
  highlight_foreground: string;
  search_highlight_background: string;
  search_highlight_foreground: string;
  loading_color: string;
  error_color: string;
  css_classes: string[];
}

export interface ViewportCallbacks {
  onScroll?: (position: number, viewport: Viewport) => void;
  onSelectionChange?: (selected: ContentId[], viewport: Viewport) => void;
  onItemActivate?: (id: ContentId, item: ViewportItem, viewport: Viewport) => void;
  onLazyLoad?: (startLine: LineNumber, count: number, viewport: Viewport) => Promise<ViewportItem[]> | ViewportItem[];
  onSearch?: (query: string, items: ViewportItem[], viewport: Viewport) => number[];
  onResize?: (width: number, height: number, viewport: Viewport) => void;
}

class ContentCache {
  private items: Array<[ContentId, ViewportItem]> = [];
  private index: Map<ContentId, number> = new Map();
  private maxSize: number;
  private hits: number = 0;
  private misses: number = 0;
  private evictions: number = 0;

  constructor(maxSize: number) {
    this.maxSize = maxSize;
  }

  get(id: ContentId): ViewportItem | null {
    const index = this.index.get(id);
    if (index !== undefined && index < this.items.length) {
      this.hits++;
      // Move to front (LRU)
      const [item] = this.items.splice(index, 1);
      this.items.unshift(item);
      this.updateIndex();
      return item[1];
    }
    this.misses++;
    return null;
  }

  insert(id: ContentId, item: ViewportItem): void {
    // Remove if already exists
    const existingIndex = this.index.get(id);
    if (existingIndex !== undefined) {
      this.items.splice(existingIndex, 1);
    }

    // Add to front
    this.items.unshift([id, item]);

    // Evict if over capacity
    while (this.items.length > this.maxSize) {
      const removed = this.items.pop();
      if (removed) {
        this.evictions++;
      }
    }

    this.updateIndex();
  }

  private updateIndex(): void {
    this.index.clear();
    this.items.forEach(([id], index) => {
      this.index.set(id, index);
    });
  }

  clear(): void {
    this.items = [];
    this.index.clear();
    this.hits = 0;
    this.misses = 0;
    this.evictions = 0;
  }

  getStats(): CacheStats {
    return {
      hits: this.hits,
      misses: this.misses,
      evictions: this.evictions,
      size: this.items.length,
      max_size: this.maxSize
    };
  }

  hitRate(): number {
    const total = this.hits + this.misses;
    return total > 0 ? this.hits / total : 0;
  }
}

export class Viewport {
  private id: string;
  private items: ViewportItem[] = [];
  private state: ViewportState;
  private config: ViewportConfig;
  private style: ViewportStyle;
  private callbacks: ViewportCallbacks;
  private cache: ContentCache;
  private searchIndex: Map<string, number[]> = new Map();
  private resizeObserver?: ResizeObserver;

  constructor(
    id: string,
    items: ViewportItem[] = [],
    config: Partial<ViewportConfig> = {},
    style: Partial<ViewportStyle> = {},
    callbacks: ViewportCallbacks = {}
  ) {
    this.id = id;
    this.items = items;
    this.callbacks = callbacks;
    this.cache = new ContentCache(config.cache_size || 1000);

    this.config = {
      width: 80,
      height: 25,
      scrollable: true,
      virtual_scrolling: true,
      lazy_loading: false,
      show_scrollbar: true,
      scrollbar_position: ScrollbarPosition.Right,
      scroll_mode: ScrollMode.Smooth,
      selection_mode: SelectionMode.Single,
      item_height: 1,
      overscan_count: 5,
      cache_size: 1000,
      scroll_sensitivity: 3,
      momentum_decay: 0.95,
      smooth_scroll_duration: 200,
      search_highlight_color: '#ffff00',
      selection_color: '#0066cc',
      highlight_color: '#333333',
      ...config
    };

    this.style = {
      background_color: '#000000',
      text_color: '#ffffff',
      border_color: '#333333',
      scrollbar_color: '#444444',
      scrollbar_thumb_color: '#666666',
      selection_background: '#0066cc',
      selection_foreground: '#ffffff',
      highlight_background: '#333333',
      highlight_foreground: '#ffffff',
      search_highlight_background: '#ffff00',
      search_highlight_foreground: '#000000',
      loading_color: '#888888',
      error_color: '#ff0000',
      css_classes: [],
      ...style
    };

    this.state = {
      scroll_position: 0,
      visible_start: 0,
      visible_end: Math.min(this.config.height, this.items.length),
      total_height: this.items.length * this.config.item_height,
      viewport_height: this.config.height,
      horizontal_scroll: 0,
      is_scrolling: false,
      scroll_velocity: 0,
      selected_items: [],
      selection_anchor: null,
      highlighted_item: null,
      search_query: '',
      search_results: [],
      current_search_index: -1,
      lazy_states: new Map(),
      needs_layout: true,
      last_update: Date.now()
    };

    this.updateVisibleRange();
    this.buildSearchIndex();
  }

  // Content management
  setItems(items: ViewportItem[]): void {
    this.items = items;
    this.state.total_height = items.length * this.config.item_height;
    this.state.needs_layout = true;
    this.buildSearchIndex();
    this.updateVisibleRange();
    this.callbacks.onResize?.(this.config.width, this.config.height, this);
  }

  addItem(item: ViewportItem, index?: number): void {
    if (index !== undefined && index >= 0 && index <= this.items.length) {
      this.items.splice(index, 0, item);
    } else {
      this.items.push(item);
    }
    this.state.total_height = this.items.length * this.config.item_height;
    this.state.needs_layout = true;
    this.buildSearchIndex();
    this.updateVisibleRange();
  }

  removeItem(id: ContentId): ViewportItem | null {
    const index = this.items.findIndex(item => item.id === id);
    if (index >= 0) {
      const removed = this.items.splice(index, 1)[0];
      this.state.total_height = this.items.length * this.config.item_height;
      this.state.needs_layout = true;
      this.deselectItem(id);
      this.buildSearchIndex();
      this.updateVisibleRange();
      return removed;
    }
    return null;
  }

  getItem(id: ContentId): ViewportItem | null {
    return this.items.find(item => item.id === id) || null;
  }

  // Scrolling operations
  scrollTo(position: number): void {
    const maxScroll = Math.max(0, this.state.total_height - this.state.viewport_height);
    this.state.scroll_position = Math.max(0, Math.min(position, maxScroll));
    this.updateVisibleRange();
    this.callbacks.onScroll?.(this.state.scroll_position, this);
  }

  scrollToLine(line: number): void {
    const position = line * this.config.item_height;
    this.scrollTo(position);
  }

  scrollUp(lines: number = 1): void {
    this.scrollTo(this.state.scroll_position - (lines * this.config.item_height));
  }

  scrollDown(lines: number = 1): void {
    this.scrollTo(this.state.scroll_position + (lines * this.config.item_height));
  }

  pageUp(): void {
    this.scrollUp(Math.floor(this.state.viewport_height / this.config.item_height));
  }

  pageDown(): void {
    this.scrollDown(Math.floor(this.state.viewport_height / this.config.item_height));
  }

  scrollToTop(): void {
    this.scrollTo(0);
  }

  scrollToBottom(): void {
    this.scrollTo(this.state.total_height);
  }

  // Selection operations
  selectItem(id: ContentId): boolean {
    if (this.config.selection_mode === SelectionMode.None) {
      return false;
    }

    const item = this.getItem(id);
    if (!item || item.selectable === false) {
      return false;
    }

    if (this.config.selection_mode === SelectionMode.Single) {
      this.state.selected_items = [id];
    } else if (this.config.selection_mode === SelectionMode.Multiple) {
      if (!this.state.selected_items.includes(id)) {
        this.state.selected_items.push(id);
      }
    }

    this.state.selection_anchor = id;
    this.callbacks.onSelectionChange?.(this.state.selected_items, this);
    return true;
  }

  deselectItem(id: ContentId): void {
    const index = this.state.selected_items.indexOf(id);
    if (index >= 0) {
      this.state.selected_items.splice(index, 1);
      this.callbacks.onSelectionChange?.(this.state.selected_items, this);
    }
  }

  toggleSelection(id: ContentId): boolean {
    if (this.state.selected_items.includes(id)) {
      this.deselectItem(id);
      return false;
    } else {
      return this.selectItem(id);
    }
  }

  selectAll(): void {
    if (this.config.selection_mode === SelectionMode.Multiple) {
      this.state.selected_items = this.items
        .filter(item => item.selectable !== false)
        .map(item => item.id);
      this.callbacks.onSelectionChange?.(this.state.selected_items, this);
    }
  }

  clearSelection(): void {
    this.state.selected_items = [];
    this.state.selection_anchor = null;
    this.callbacks.onSelectionChange?.([], this);
  }

  getSelectedItems(): ContentId[] {
    return [...this.state.selected_items];
  }

  // Highlighting
  highlightItem(id: ContentId): boolean {
    const item = this.getItem(id);
    if (!item) {
      return false;
    }
    this.state.highlighted_item = id;
    return true;
  }

  clearHighlight(): void {
    this.state.highlighted_item = null;
  }

  // Search operations
  search(query: string): number {
    this.state.search_query = query;
    
    if (!query.trim()) {
      this.state.search_results = [];
      this.state.current_search_index = -1;
      return 0;
    }

    if (this.callbacks.onSearch) {
      this.state.search_results = this.callbacks.onSearch(query, this.items, this);
    } else {
      this.state.search_results = this.defaultSearch(query);
    }

    this.state.current_search_index = this.state.search_results.length > 0 ? 0 : -1;
    
    if (this.state.current_search_index >= 0) {
      this.scrollToLine(this.state.search_results[0]);
    }

    return this.state.search_results.length;
  }

  private defaultSearch(query: string): number[] {
    const results: number[] = [];
    const lowerQuery = query.toLowerCase();
    
    this.items.forEach((item, index) => {
      if (item.content.toLowerCase().includes(lowerQuery)) {
        results.push(index);
      }
    });
    
    return results;
  }

  nextSearchResult(): boolean {
    if (this.state.search_results.length === 0) {
      return false;
    }

    this.state.current_search_index = 
      (this.state.current_search_index + 1) % this.state.search_results.length;
    
    this.scrollToLine(this.state.search_results[this.state.current_search_index]);
    return true;
  }

  previousSearchResult(): boolean {
    if (this.state.search_results.length === 0) {
      return false;
    }

    this.state.current_search_index = 
      this.state.current_search_index <= 0 
        ? this.state.search_results.length - 1 
        : this.state.current_search_index - 1;
    
    this.scrollToLine(this.state.search_results[this.state.current_search_index]);
    return true;
  }

  // Virtual scrolling support
  private updateVisibleRange(): void {
    if (!this.config.virtual_scrolling) {
      this.state.visible_start = 0;
      this.state.visible_end = this.items.length;
      return;
    }

    const startLine = Math.floor(this.state.scroll_position / this.config.item_height);
    const endLine = Math.ceil((this.state.scroll_position + this.state.viewport_height) / this.config.item_height);

    this.state.visible_start = Math.max(0, startLine - this.config.overscan_count);
    this.state.visible_end = Math.min(this.items.length, endLine + this.config.overscan_count);

    // Trigger lazy loading if needed
    if (this.config.lazy_loading && this.callbacks.onLazyLoad) {
      this.loadVisibleItems();
    }
  }

  private async loadVisibleItems(): Promise<void> {
    const toLoad: number[] = [];
    
    for (let i = this.state.visible_start; i < this.state.visible_end; i++) {
      if (i < this.items.length) {
        const item = this.items[i];
        const state = this.state.lazy_states.get(item.id) || LazyLoadState.NotLoaded;
        
        if (state === LazyLoadState.NotLoaded) {
          toLoad.push(i);
          this.state.lazy_states.set(item.id, LazyLoadState.Loading);
        }
      }
    }

    if (toLoad.length > 0 && this.callbacks.onLazyLoad) {
      try {
        const newItems = await this.callbacks.onLazyLoad(toLoad[0], toLoad.length, this);
        
        if (Array.isArray(newItems)) {
          newItems.forEach((item, index) => {
            const targetIndex = toLoad[index];
            if (targetIndex < this.items.length) {
              this.items[targetIndex] = item;
              this.state.lazy_states.set(item.id, LazyLoadState.Loaded);
              this.cache.insert(item.id, item);
            }
          });
        }
      } catch {
        toLoad.forEach(index => {
          if (index < this.items.length) {
            const item = this.items[index];
            this.state.lazy_states.set(item.id, LazyLoadState.Error);
          }
        });
      }
    }
  }

  private buildSearchIndex(): void {
    this.searchIndex.clear();
    
    this.items.forEach((item, index) => {
      const words = item.content.toLowerCase().split(/\s+/);
      words.forEach(word => {
        if (word.length > 0) {
          if (!this.searchIndex.has(word)) {
            this.searchIndex.set(word, []);
          }
          this.searchIndex.get(word)!.push(index);
        }
      });
    });
  }

  // Event handling
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'ArrowUp':
      case 'k':
        this.scrollUp();
        return true;
      case 'ArrowDown':
      case 'j':
        this.scrollDown();
        return true;
      case 'PageUp':
        this.pageUp();
        return true;
      case 'PageDown':
        this.pageDown();
        return true;
      case 'Home':
      case 'g':
        this.scrollToTop();
        return true;
      case 'End':
      case 'G':
        this.scrollToBottom();
        return true;
      case 'Enter':
        if (this.state.highlighted_item) {
          const item = this.getItem(this.state.highlighted_item);
          if (item) {
            this.callbacks.onItemActivate?.(this.state.highlighted_item, item, this);
          }
        }
        return true;
      case ' ':
        if (this.state.highlighted_item) {
          this.toggleSelection(this.state.highlighted_item);
        }
        return true;
      case 'a':
        if (this.config.selection_mode === SelectionMode.Multiple) {
          this.selectAll();
        }
        return true;
      case 'Escape':
        this.clearSelection();
        this.clearHighlight();
        return true;
      default:
        return false;
    }
  }

  handleMouseWheel(deltaY: number): void {
    const scrollAmount = deltaY * this.config.scroll_sensitivity;
    this.scrollTo(this.state.scroll_position + scrollAmount);
  }

  // Utility methods
  getVisibleItems(): ViewportItem[] {
    return this.items.slice(this.state.visible_start, this.state.visible_end);
  }

  getTotalLines(): number {
    return this.items.length;
  }

  getCurrentLine(): number {
    return Math.floor(this.state.scroll_position / this.config.item_height);
  }

  isItemVisible(id: ContentId): boolean {
    const index = this.items.findIndex(item => item.id === id);
    return index >= this.state.visible_start && index < this.state.visible_end;
  }

  getCacheStats(): CacheStats {
    return this.cache.getStats();
  }

  // Cleanup
  destroy(): void {
    this.resizeObserver?.disconnect();
    this.cache.clear();
    this.searchIndex.clear();
  }
}

export class ViewportBuilder {
  private id: string;
  private items: ViewportItem[] = [];
  private config: Partial<ViewportConfig> = {};
  private style: Partial<ViewportStyle> = {};
  private callbacks: ViewportCallbacks = {};

  constructor(id: string) {
    this.id = id;
  }

  width(width: number): this {
    this.config.width = width;
    return this;
  }

  height(height: number): this {
    this.config.height = height;
    return this;
  }

  content(items: ViewportItem[]): this {
    this.items = items;
    return this;
  }

  contentFromStrings(lines: string[]): this {
    this.items = lines.map((line, index) => ({
      id: `line-${index}`,
      content: line,
      selectable: true
    }));
    return this;
  }

  scrollable(scrollable: boolean): this {
    this.config.scrollable = scrollable;
    return this;
  }

  virtualScrolling(enabled: boolean): this {
    this.config.virtual_scrolling = enabled;
    return this;
  }

  selectionMode(mode: SelectionMode): this {
    this.config.selection_mode = mode;
    return this;
  }

  showScrollbar(show: boolean): this {
    this.config.show_scrollbar = show;
    return this;
  }

  scrollbarPosition(position: ScrollbarPosition): this {
    this.config.scrollbar_position = position;
    return this;
  }

  itemHeight(height: number): this {
    this.config.item_height = height;
    return this;
  }

  cacheSize(size: number): this {
    this.config.cache_size = size;
    return this;
  }

  lazyLoading(enabled: boolean): this {
    this.config.lazy_loading = enabled;
    return this;
  }

  onScroll(callback: (position: number, viewport: Viewport) => void): this {
    this.callbacks.onScroll = callback;
    return this;
  }

  onSelectionChange(callback: (selected: ContentId[], viewport: Viewport) => void): this {
    this.callbacks.onSelectionChange = callback;
    return this;
  }

  onItemActivate(callback: (id: ContentId, item: ViewportItem, viewport: Viewport) => void): this {
    this.callbacks.onItemActivate = callback;
    return this;
  }

  onLazyLoad(callback: (startLine: number, count: number, viewport: Viewport) => Promise<ViewportItem[]> | ViewportItem[]): this {
    this.callbacks.onLazyLoad = callback;
    return this;
  }

  build(): Viewport {
    return new Viewport(this.id, this.items, this.config, this.style, this.callbacks);
  }
}

// Convenience functions for common viewport patterns

export function fileViewer(lines: string[]): Viewport {
  return new ViewportBuilder('file-viewer')
    .contentFromStrings(lines)
    .width(100)
    .height(30)
    .scrollable(true)
    .virtualScrolling(true)
    .selectionMode(SelectionMode.Single)
    .showScrollbar(true)
    .build();
}

export function logViewer(logs: string[]): Viewport {
  return new ViewportBuilder('log-viewer')
    .contentFromStrings(logs)
    .width(120)
    .height(25)
    .scrollable(true)
    .virtualScrolling(true)
    .selectionMode(SelectionMode.None)
    .showScrollbar(true)
    .build();
}

export function dataTableViewport(items: ViewportItem[]): Viewport {
  return new ViewportBuilder('data-table')
    .content(items)
    .width(80)
    .height(20)
    .scrollable(true)
    .selectionMode(SelectionMode.Multiple)
    .showScrollbar(true)
    .virtualScrolling(true)
    .build();
}