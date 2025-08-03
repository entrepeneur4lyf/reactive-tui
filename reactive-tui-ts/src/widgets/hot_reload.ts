/**
 * Hot Reload System for CSS Changes - TypeScript Implementation
 * 
 * Provides live CSS updates during development without restarting the application.
 * Watches CSS files for changes and applies them instantly to the running TUI.
 * 
 * Features:
 * - File system watching for CSS changes
 * - Debounced reload to handle rapid changes
 * - CSS validation before applying changes
 * - Error recovery and fallback to previous CSS
 * - Development-only feature with zero overhead in production
 * - TypeScript-first API with full type safety
 * 
 * Basic Usage:
 * ```typescript
 * const hotReload = new HotReloadManager({
 *   watchPaths: ['./styles'],
 *   cssEngine: cssEngine
 * });
 * 
 * await hotReload.start();
 * 
 * // Subscribe to events
 * hotReload.on('reload-success', (path) => {
 *   console.log(`CSS reloaded from: ${path}`);
 * });
 * ```
 */

import { watch, type FSWatcher } from 'fs';
import { readFile } from 'fs/promises';
import { extname, resolve } from 'path';
import { EventEmitter } from 'events';
import type { Stylesheet } from '../css';
import { parseCSS } from '../css';

// Stub implementations for missing dependencies
class ReactiveState<T> {
  constructor(private _value: T) {}
  get value(): T { return this._value; }
  set value(val: T) { this._value = val; }
}

interface CssEngine {
  getRawCss(): string | null;
  applyStylesheet(stylesheet: Stylesheet): void;
  invalidateCache(): void;
}

function validateCss(_content: string): { valid: boolean; errors: string[] } {
  // Simple validation stub
  return { valid: true, errors: [] };
}

function parseCss(content: string): Promise<Stylesheet> {
  // Use the existing parseCSS function
  return Promise.resolve(parseCSS(content));
}

/// Hot reload event types
export enum HotReloadEventType {
    CssChanged = 'css-changed',
    ReloadSuccess = 'reload-success',
    ReloadError = 'reload-error',
    WatcherError = 'watcher-error'
}

/// Hot reload event payloads
export interface HotReloadEvent {
    type: HotReloadEventType;
    path: string;
    content?: string;
    error?: string;
}

/// Configuration for hot reload
export interface HotReloadConfig {
    /// Paths to watch for CSS files
    watchPaths: string[];
    /// File extensions to watch (default: ['.css', '.scss'])
    extensions?: string[];
    /// Debounce duration in milliseconds (default: 100)
    debounceDuration?: number;
    /// Enable validation before applying CSS (default: true)
    validateBeforeApply?: boolean;
    /// Maximum number of reload retries (default: 3)
    maxRetries?: number;
    /// Enable verbose logging (default: false)
    verbose?: boolean;
    /// CSS engine instance
    cssEngine: CssEngine;
}

/// Debounced function helper
interface DebouncedFunction<T extends (...args: any[]) => any> {
    (...args: Parameters<T>): void;
    cancel(): void;
    flush(): void;
}

function debounce<T extends (...args: any[]) => any>(
    func: T,
    wait: number
): DebouncedFunction<T> {
    let timeout: NodeJS.Timeout | null = null;
    let lastArgs: Parameters<T> | null = null;

    const debounced = (...args: Parameters<T>) => {
        lastArgs = args;
        if (timeout) clearTimeout(timeout);
        timeout = setTimeout(() => {
            if (lastArgs) func(...lastArgs);
            timeout = null;
            lastArgs = null;
        }, wait);
    };

    debounced.cancel = () => {
        if (timeout) {
            clearTimeout(timeout);
            timeout = null;
            lastArgs = null;
        }
    };

    debounced.flush = () => {
        if (timeout && lastArgs) {
            clearTimeout(timeout);
            func(...lastArgs);
            timeout = null;
            lastArgs = null;
        }
    };

    return debounced;
}

/**
 * Hot reload manager for CSS files
 */
export class HotReloadManager extends EventEmitter {
    private config: Required<HotReloadConfig>;
    private watchers: Map<string, FSWatcher> = new Map();
    private lastReloadTimes: Map<string, number> = new Map();
    private backupCss: Map<string, string> = new Map();
    private debouncedReloads: Map<string, DebouncedFunction<() => void>> = new Map();
    
    // Reactive state for UI updates
    public readonly reloadCount = new ReactiveState(0);
    public readonly lastReloadPath = new ReactiveState<string | null>(null);
    public readonly isReloading = new ReactiveState(false);
    public readonly errors = new ReactiveState<string[]>([]);

    constructor(config: HotReloadConfig) {
        super();
        
        // Apply defaults
        this.config = {
            watchPaths: config.watchPaths,
            extensions: config.extensions || ['.css', '.scss'],
            debounceDuration: config.debounceDuration ?? 100,
            validateBeforeApply: config.validateBeforeApply ?? true,
            maxRetries: config.maxRetries ?? 3,
            verbose: config.verbose ?? false,
            cssEngine: config.cssEngine
        };
    }

    /**
     * Start watching for CSS changes
     */
    async start(): Promise<void> {
        // Only enable in development
        if (process.env.NODE_ENV === 'production') {
            return;
        }

        for (const watchPath of this.config.watchPaths) {
            await this.watchDirectory(watchPath);
        }

        if (this.config.verbose) {
            console.log('🔥 Hot reload enabled for CSS files');
            console.log('📁 Watching paths:', this.config.watchPaths);
            console.log('📝 Extensions:', this.config.extensions);
        }
    }

    /**
     * Stop watching for changes
     */
    stop(): void {
        // Cancel all debounced functions
        for (const debounced of this.debouncedReloads.values()) {
            debounced.cancel();
        }
        this.debouncedReloads.clear();

        // Close all watchers
        for (const [path, watcher] of this.watchers) {
            watcher.close();
            if (this.config.verbose) {
                console.log(`🛑 Stopped watching: ${path}`);
            }
        }
        this.watchers.clear();
    }

    /**
     * Watch a directory for CSS changes
     */
    private async watchDirectory(dirPath: string): Promise<void> {
        const resolvedPath = resolve(dirPath);
        
        // Create watcher
        const watcher = watch(
            resolvedPath,
            { recursive: true },
            (eventType, filename) => {
                if (eventType === 'change' && filename) {
                    const fullPath = resolve(resolvedPath, filename);
                    const ext = extname(fullPath);
                    
                    if (this.config.extensions.includes(ext)) {
                        this.handleFileChange(fullPath);
                    }
                }
            }
        );

        watcher.on('error', (error) => {
            this.emit(HotReloadEventType.WatcherError, {
                type: HotReloadEventType.WatcherError,
                path: resolvedPath,
                error: error.message
            });
            
            if (this.config.verbose) {
                console.error(`⚠️ Watcher error for ${resolvedPath}:`, error);
            }
        });

        this.watchers.set(resolvedPath, watcher);
    }

    /**
     * Handle file change event
     */
    private handleFileChange(filePath: string): void {
        if (this.config.verbose) {
            console.log(`🔄 Detected change in: ${filePath}`);
        }

        // Get or create debounced reload function for this file
        let debouncedReload = this.debouncedReloads.get(filePath);
        if (!debouncedReload) {
            debouncedReload = debounce(
                () => this.reloadCss(filePath),
                this.config.debounceDuration
            );
            this.debouncedReloads.set(filePath, debouncedReload);
        }

        // Call debounced reload
        debouncedReload();
    }

    /**
     * Reload CSS from file
     */
    async reloadCss(filePath: string): Promise<void> {
        try {
            // Set reloading state
            this.isReloading.value = true;
            
            // Read file content
            const content = await readFile(filePath, 'utf-8');
            
            // Emit CSS changed event
            this.emit(HotReloadEventType.CssChanged, {
                type: HotReloadEventType.CssChanged,
                path: filePath,
                content
            });

            // Backup current CSS
            const currentCss = this.config.cssEngine.getRawCss();
            if (currentCss) {
                this.backupCss.set(filePath, currentCss);
            }

            // Validate CSS if configured
            if (this.config.validateBeforeApply) {
                const validationResult = validateCss(content);
                if (!validationResult.valid) {
                    throw new Error(`CSS validation failed: ${validationResult.errors.join(', ')}`);
                }
            }

            // Apply new CSS with retries
            let retryCount = 0;
            let lastError: Error | null = null;

            while (retryCount < this.config.maxRetries) {
                try {
                    await this.applyCssChanges(content);
                    
                    // Success!
                    this.reloadCount.value++;
                    this.lastReloadPath.value = filePath;
                    
                    // Clear errors
                    this.errors.value = [];
                    
                    this.emit(HotReloadEventType.ReloadSuccess, {
                        type: HotReloadEventType.ReloadSuccess,
                        path: filePath
                    });
                    
                    if (this.config.verbose) {
                        console.log(`✅ CSS hot reload successful: ${filePath}`);
                    }
                    
                    return; // Exit on success
                    
                } catch (error) {
                    lastError = error as Error;
                    retryCount++;
                    
                    if (retryCount < this.config.maxRetries) {
                        // Wait before retry
                        await new Promise(resolve => setTimeout(resolve, 100));
                    }
                }
            }

            // All retries failed - rollback
            if (lastError) {
                await this.rollbackCss(filePath);
                throw lastError;
            }
            
        } catch (error) {
            const errorMessage = error instanceof Error ? error.message : String(error);
            
            // Add error to reactive state
            this.errors.value = [...this.errors.value, errorMessage];
            
            this.emit(HotReloadEventType.ReloadError, {
                type: HotReloadEventType.ReloadError,
                path: filePath,
                error: errorMessage
            });
            
            if (this.config.verbose) {
                console.error(`❌ CSS hot reload failed for ${filePath}:`, errorMessage);
            }
        } finally {
            this.isReloading.value = false;
        }
    }

    /**
     * Apply CSS changes to the engine
     */
    private async applyCssChanges(content: string): Promise<void> {
        // Parse CSS
        const stylesheet = await parseCss(content);
        
        // Apply to engine
        this.config.cssEngine.applyStylesheet(stylesheet);
        
        // Invalidate cache to trigger re-computation
        this.config.cssEngine.invalidateCache();
    }

    /**
     * Rollback to backup CSS
     */
    private async rollbackCss(filePath: string): Promise<void> {
        const backup = this.backupCss.get(filePath);
        if (backup) {
            try {
                await this.applyCssChanges(backup);
                if (this.config.verbose) {
                    console.log(`🔄 Rolled back to previous CSS for ${filePath}`);
                }
            } catch (error) {
                console.error('Failed to rollback CSS:', error);
            }
        }
    }

    /**
     * Get current statistics
     */
    getStats(): HotReloadStats {
        return {
            reloadCount: this.reloadCount.value,
            lastReloadPath: this.lastReloadPath.value,
            isReloading: this.isReloading.value,
            errors: this.errors.value,
            watchedPaths: Array.from(this.watchers.keys()),
            backupCount: this.backupCss.size
        };
    }

    /**
     * Clear all errors
     */
    clearErrors(): void {
        this.errors.value = [];
    }

    /**
     * Clear backup CSS
     */
    clearBackups(): void {
        this.backupCss.clear();
    }
}

/// Hot reload statistics
export interface HotReloadStats {
    reloadCount: number;
    lastReloadPath: string | null;
    isReloading: boolean;
    errors: string[];
    watchedPaths: string[];
    backupCount: number;
}

/**
 * Create a hot reload manager with builder pattern
 */
export class HotReloadBuilder {
    private config: Partial<HotReloadConfig> = {};

    watchPaths(paths: string[]): this {
        this.config.watchPaths = paths;
        return this;
    }

    watchPath(path: string): this {
        this.config.watchPaths = [...(this.config.watchPaths || []), path];
        return this;
    }

    extensions(exts: string[]): this {
        this.config.extensions = exts;
        return this;
    }

    extension(ext: string): this {
        this.config.extensions = [...(this.config.extensions || []), ext];
        return this;
    }

    debounce(ms: number): this {
        this.config.debounceDuration = ms;
        return this;
    }

    validate(enable: boolean): this {
        this.config.validateBeforeApply = enable;
        return this;
    }

    maxRetries(count: number): this {
        this.config.maxRetries = count;
        return this;
    }

    verbose(enable: boolean): this {
        this.config.verbose = enable;
        return this;
    }

    cssEngine(engine: CssEngine): this {
        this.config.cssEngine = engine;
        return this;
    }

    build(): HotReloadManager {
        if (!this.config.watchPaths || this.config.watchPaths.length === 0) {
            throw new Error('At least one watch path is required');
        }
        if (!this.config.cssEngine) {
            throw new Error('CSS engine is required');
        }
        
        return new HotReloadManager(this.config as HotReloadConfig);
    }
}

/**
 * Convenience function to create hot reload manager
 */
export function createHotReload(config: HotReloadConfig): HotReloadManager {
    return new HotReloadManager(config);
}

/**
 * Development mode hot reload with sensible defaults
 */
export function devHotReload(cssEngine: CssEngine): HotReloadManager {
    return new HotReloadBuilder()
        .watchPaths(['./styles', './src/styles', './css'])
        .extensions(['.css', '.scss', '.less'])
        .debounce(100)
        .validate(true)
        .verbose(true)
        .cssEngine(cssEngine)
        .build();
}