# HotReload Widget

The HotReload widget provides live CSS updates during development without restarting the application. It watches CSS files for changes, validates them, and applies updates instantly with error recovery, debounced reloading, and comprehensive event handling for seamless development workflow.

## Basic Usage

```typescript
import { HotReloadManager, HotReloadBuilder, devHotReload } from 'reactive-tui';

// Basic hot reload setup
const hotReload = new HotReloadManager({
  watchPaths: ['./styles'],
  cssEngine: cssEngine
});

await hotReload.start();

// Using the builder pattern
const builderHotReload = new HotReloadBuilder()
  .watchPaths(['./styles', './css'])
  .extensions(['.css', '.scss'])
  .debounce(150)
  .verbose(true)
  .cssEngine(cssEngine)
  .build();

// Convenience function for development
const devReload = devHotReload(cssEngine);
await devReload.start();
```

## Configuration

### HotReloadConfig Interface

```typescript
interface HotReloadConfig {
  watchPaths: string[];              // Paths to watch for CSS files
  extensions?: string[];             // File extensions to watch (default: ['.css', '.scss'])
  debounceDuration?: number;         // Debounce duration in milliseconds (default: 100)
  validateBeforeApply?: boolean;     // Enable validation before applying CSS (default: true)
  maxRetries?: number;               // Maximum number of reload retries (default: 3)
  verbose?: boolean;                 // Enable verbose logging (default: false)
  cssEngine: CssEngine;             // CSS engine instance (required)
}
```

### HotReloadStats Interface

```typescript
interface HotReloadStats {
  reloadCount: number;              // Total number of successful reloads
  lastReloadPath: string | null;    // Path of last reloaded file
  isReloading: boolean;             // Current reload status
  errors: string[];                 // Array of error messages
  watchedPaths: string[];           // Currently watched directory paths
  backupCount: number;              // Number of backup CSS files stored
}
```

### Event Types

```typescript
enum HotReloadEventType {
  CssChanged = 'css-changed',       // CSS file changed
  ReloadSuccess = 'reload-success', // Reload completed successfully
  ReloadError = 'reload-error',     // Reload failed
  WatcherError = 'watcher-error'    // File watcher error
}

interface HotReloadEvent {
  type: HotReloadEventType;
  path: string;                     // File path
  content?: string;                 // File content (for CssChanged)
  error?: string;                   // Error message (for errors)
}
```

## Core Features

### File System Watching

```typescript
// Watch multiple directories
const hotReload = new HotReloadManager({
  watchPaths: [
    './src/styles',
    './components/styles',
    './themes'
  ],
  extensions: ['.css', '.scss', '.less'],
  cssEngine: cssEngine
});

// Start watching
await hotReload.start();

// The system automatically watches for changes recursively
// and triggers reloads when CSS files are modified

// Stop watching when done
hotReload.stop();
```

### Debounced Reloading

```typescript
// Configure debouncing to handle rapid file changes
const debouncedReload = new HotReloadBuilder()
  .watchPaths(['./styles'])
  .debounce(200)                    // Wait 200ms after last change
  .cssEngine(cssEngine)
  .build();

await debouncedReload.start();

// Multiple rapid changes to the same file will be batched
// into a single reload operation after the debounce period
```

### CSS Validation and Error Recovery

```typescript
// Enable CSS validation before applying changes
const validatingReload = new HotReloadBuilder()
  .watchPaths(['./styles'])
  .validate(true)                   // Enable validation
  .maxRetries(5)                    // Retry up to 5 times on failure
  .cssEngine(cssEngine)
  .build();

// Listen for validation errors
validatingReload.on('reload-error', (event) => {
  console.error('CSS validation failed:', event.error);
  console.log('File:', event.path);
});

// Automatic rollback on persistent failures
await validatingReload.start();
```

### Event Handling

```typescript
const eventReload = new HotReloadManager({
  watchPaths: ['./styles'],
  verbose: true,
  cssEngine: cssEngine
});

// Subscribe to all hot reload events
eventReload.on('css-changed', (event) => {
  console.log(`CSS file changed: ${event.path}`);
  console.log(`Content length: ${event.content?.length} characters`);
});

eventReload.on('reload-success', (event) => {
  console.log(`✅ Successfully reloaded: ${event.path}`);
  
  // Update UI to show reload success
  showSuccessNotification(`CSS updated from ${event.path}`);
});

eventReload.on('reload-error', (event) => {
  console.error(`❌ Failed to reload: ${event.path}`);
  console.error(`Error: ${event.error}`);
  
  // Show error notification to developer
  showErrorNotification(`CSS reload failed: ${event.error}`);
});

eventReload.on('watcher-error', (event) => {
  console.error(`File watcher error for ${event.path}: ${event.error}`);
});

await eventReload.start();
```

### Reactive State Management

```typescript
// Access reactive state for UI updates
const hotReload = new HotReloadManager({
  watchPaths: ['./styles'],
  cssEngine: cssEngine
});

// Watch reactive state changes
hotReload.reloadCount.value; // Number of successful reloads
hotReload.lastReloadPath.value; // Last reloaded file path
hotReload.isReloading.value; // Current reload status
hotReload.errors.value; // Array of current errors

// Use in UI components
function DevStatusBar() {
  const reloadCount = hotReload.reloadCount.value;
  const isReloading = hotReload.isReloading.value;
  const errors = hotReload.errors.value;
  
  return `
Hot Reload: ${reloadCount} reloads
Status: ${isReloading ? 'Reloading...' : 'Watching'}
Errors: ${errors.length}
  `;
}
```

## Builder Pattern

```typescript
// Comprehensive hot reload configuration
const advancedHotReload = new HotReloadBuilder()
  .watchPaths(['./src/styles', './themes', './components'])
  .extensions(['.css', '.scss', '.less', '.stylus'])
  .debounce(150)                    // 150ms debounce
  .validate(true)                   // Enable CSS validation
  .maxRetries(3)                    // Retry up to 3 times
  .verbose(true)                    // Enable detailed logging
  .cssEngine(cssEngine)
  .build();

// Start with event listeners
advancedHotReload.on('reload-success', (event) => {
  console.log(`🔥 Hot reloaded: ${event.path}`);
});

await advancedHotReload.start();
```

## Real-World Examples

### Development Server Integration

```typescript
import { HotReloadManager, HotReloadBuilder } from 'reactive-tui';
import { WebSocketServer } from 'ws';

class DevelopmentServer {
  private hotReload: HotReloadManager;
  private wsServer: WebSocketServer;
  private clients: Set<WebSocket> = new Set();
  
  constructor(private cssEngine: any) {
    this.setupHotReload();
    this.setupWebSocket();
  }
  
  private setupHotReload() {
    this.hotReload = new HotReloadBuilder()
      .watchPaths([
        './src/styles',
        './components/**/*.css',
        './themes'
      ])
      .extensions(['.css', '.scss', '.less'])
      .debounce(100)
      .validate(true)
      .verbose(true)
      .cssEngine(this.cssEngine)
      .build();
    
    // Forward hot reload events to connected clients
    this.hotReload.on('reload-success', (event) => {
      this.broadcastToClients({
        type: 'css-reload-success',
        path: event.path,
        timestamp: Date.now()
      });
    });
    
    this.hotReload.on('reload-error', (event) => {
      this.broadcastToClients({
        type: 'css-reload-error',
        path: event.path,
        error: event.error,
        timestamp: Date.now()
      });
    });
    
    this.hotReload.on('css-changed', (event) => {
      this.broadcastToClients({
        type: 'css-changed',
        path: event.path,
        contentLength: event.content?.length || 0,
        timestamp: Date.now()
      });
    });
  }
  
  private setupWebSocket() {
    this.wsServer = new WebSocketServer({ port: 8080 });
    
    this.wsServer.on('connection', (ws) => {
      this.clients.add(ws);
      console.log('Dev client connected');
      
      // Send current hot reload stats
      ws.send(JSON.stringify({
        type: 'hot-reload-stats',
        stats: this.hotReload.getStats()
      }));
      
      ws.on('close', () => {
        this.clients.delete(ws);
      });
      
      ws.on('message', (data) => {
        const message = JSON.parse(data.toString());
        this.handleClientMessage(message, ws);
      });
    });
  }
  
  private handleClientMessage(message: any, ws: WebSocket) {
    switch (message.type) {
      case 'get-stats':
        ws.send(JSON.stringify({
          type: 'hot-reload-stats',
          stats: this.hotReload.getStats()
        }));
        break;
        
      case 'clear-errors':
        this.hotReload.clearErrors();
        this.broadcastToClients({
          type: 'errors-cleared',
          timestamp: Date.now()
        });
        break;
        
      case 'force-reload':
        // Force reload all CSS files
        this.forceReloadAll();
        break;
    }
  }
  
  private broadcastToClients(message: any) {
    const messageStr = JSON.stringify(message);
    this.clients.forEach(client => {
      if (client.readyState === WebSocket.OPEN) {
        client.send(messageStr);
      }
    });
  }
  
  private async forceReloadAll() {
    const stats = this.hotReload.getStats();
    
    for (const watchedPath of stats.watchedPaths) {
      // Trigger reload for all CSS files in watched paths
      // This would require additional file system scanning
      console.log(`Force reloading CSS files in: ${watchedPath}`);
    }
  }
  
  async start() {
    await this.hotReload.start();
    console.log('🔥 Development server started with hot reload');
    console.log('📡 WebSocket server listening on port 8080');
  }
  
  async stop() {
    this.hotReload.stop();
    this.wsServer.close();
    console.log('🛑 Development server stopped');
  }
  
  getStats() {
    return this.hotReload.getStats();
  }
}

// Usage
const cssEngine = new CSSEngine(); // Your CSS engine instance
const devServer = new DevelopmentServer(cssEngine);

await devServer.start();

// Monitor development stats
setInterval(() => {
  const stats = devServer.getStats();
  console.log(`Dev Stats: ${stats.reloadCount} reloads, ${stats.errors.length} errors`);
}, 10000);
```

### Theme Development Workflow

```typescript
class ThemeDevelopmentWorkflow {
  private hotReload: HotReloadManager;
  private themeCache: Map<string, any> = new Map();
  private activeTheme: string = 'default';
  
  constructor(private cssEngine: any) {
    this.hotReload = new HotReloadBuilder()
      .watchPaths(['./themes'])
      .extensions(['.css', '.scss'])
      .debounce(200)
      .validate(true)
      .verbose(true)
      .cssEngine(cssEngine)
      .build();
    
    this.setupThemeHandling();
  }
  
  private setupThemeHandling() {
    this.hotReload.on('css-changed', (event) => {
      const themeName = this.extractThemeName(event.path);
      if (themeName) {
        console.log(`🎨 Theme '${themeName}' is being updated...`);
        this.invalidateThemeCache(themeName);
      }
    });
    
    this.hotReload.on('reload-success', (event) => {
      const themeName = this.extractThemeName(event.path);
      if (themeName) {
        console.log(`✅ Theme '${themeName}' updated successfully`);
        
        if (themeName === this.activeTheme) {
          this.notifyThemeUpdate(themeName);
        }
        
        // Update theme preview if available
        this.updateThemePreview(themeName);
      }
    });
    
    this.hotReload.on('reload-error', (event) => {
      const themeName = this.extractThemeName(event.path);
      if (themeName) {
        console.error(`❌ Theme '${themeName}' update failed: ${event.error}`);
        
        // Restore from cache if available
        this.restoreThemeFromCache(themeName);
      }
    });
  }
  
  private extractThemeName(filePath: string): string | null {
    // Extract theme name from file path
    // e.g., './themes/dark.css' -> 'dark'
    const match = filePath.match(/themes[\/\\]([^\/\\]+)\.(css|scss)$/);
    return match ? match[1] : null;
  }
  
  private invalidateThemeCache(themeName: string) {
    this.themeCache.delete(themeName);
  }
  
  private notifyThemeUpdate(themeName: string) {
    // Notify the application that the active theme has been updated
    console.log(`🔄 Active theme '${themeName}' has been hot reloaded`);
    
    // Trigger UI refresh
    this.refreshUI();
  }
  
  private updateThemePreview(themeName: string) {
    // Update theme preview in development tools
    console.log(`🖼️ Updating preview for theme '${themeName}'`);
  }
  
  private restoreThemeFromCache(themeName: string) {
    const cached = this.themeCache.get(themeName);
    if (cached) {
      console.log(`🔄 Restoring theme '${themeName}' from cache`);
      // Apply cached theme
    }
  }
  
  private refreshUI() {
    // Trigger UI refresh to apply new theme
    console.log('🔄 Refreshing UI with updated theme');
  }
  
  async start() {
    await this.hotReload.start();
    console.log('🎨 Theme development workflow started');
  }
  
  switchTheme(themeName: string) {
    this.activeTheme = themeName;
    console.log(`🎨 Switched to theme: ${themeName}`);
  }
  
  getCurrentTheme(): string {
    return this.activeTheme;
  }
  
  getAvailableThemes(): string[] {
    // Return list of available themes from watched directories
    const stats = this.hotReload.getStats();
    const themes: string[] = [];
    
    // This would scan the themes directory for available themes
    console.log('Available themes:', themes);
    return themes;
  }
  
  async previewTheme(themeName: string) {
    console.log(`👀 Previewing theme: ${themeName}`);
    // Temporarily apply theme for preview
  }
  
  getThemeStats() {
    const hotReloadStats = this.hotReload.getStats();
    
    return {
      activeTheme: this.activeTheme,
      cachedThemes: Array.from(this.themeCache.keys()),
      hotReloadStats
    };
  }
  
  stop() {
    this.hotReload.stop();
    console.log('🛑 Theme development workflow stopped');
  }
}

// Usage
const themeWorkflow = new ThemeDevelopmentWorkflow(cssEngine);

await themeWorkflow.start();

// Switch between themes during development
themeWorkflow.switchTheme('dark');
themeWorkflow.switchTheme('light');
themeWorkflow.switchTheme('high-contrast');

// Preview themes
await themeWorkflow.previewTheme('experimental');

// Monitor theme development
setInterval(() => {
  const stats = themeWorkflow.getThemeStats();
  console.log('Theme Stats:', stats);
}, 5000);
```

### Component Style Development

```typescript
class ComponentStyleDeveloper {
  private hotReload: HotReloadManager;
  private componentStyles: Map<string, string> = new Map();
  private mountedComponents: Set<string> = new Set();
  
  constructor(private cssEngine: any) {
    this.hotReload = new HotReloadBuilder()
      .watchPaths(['./src/components'])
      .extensions(['.css', '.scss', '.module.css'])
      .debounce(100)
      .validate(true)
      .verbose(true)
      .cssEngine(cssEngine)
      .build();
    
    this.setupComponentHandling();
  }
  
  private setupComponentHandling() {
    this.hotReload.on('css-changed', (event) => {
      const componentName = this.extractComponentName(event.path);
      if (componentName) {
        console.log(`🧩 Component '${componentName}' styles changing...`);
        
        // Store the new content temporarily
        if (event.content) {
          this.componentStyles.set(componentName, event.content);
        }
      }
    });
    
    this.hotReload.on('reload-success', (event) => {
      const componentName = this.extractComponentName(event.path);
      if (componentName) {
        console.log(`✅ Component '${componentName}' styles updated`);
        
        // Check if component is currently mounted
        if (this.mountedComponents.has(componentName)) {
          this.hotReloadComponent(componentName);
        }
        
        // Update style isolation if using CSS modules
        this.updateStyleIsolation(componentName);
      }
    });
    
    this.hotReload.on('reload-error', (event) => {
      const componentName = this.extractComponentName(event.path);
      if (componentName) {
        console.error(`❌ Component '${componentName}' style update failed: ${event.error}`);
        
        // Show inline error in component
        this.showComponentError(componentName, event.error || 'Unknown error');
      }
    });
  }
  
  private extractComponentName(filePath: string): string | null {
    // Extract component name from file path
    // e.g., './src/components/Button/Button.css' -> 'Button'
    const match = filePath.match(/components[\/\\]([^\/\\]+)[\/\\]/);
    return match ? match[1] : null;
  }
  
  private hotReloadComponent(componentName: string) {
    console.log(`🔥 Hot reloading component '${componentName}'`);
    
    // Trigger component re-render with new styles
    this.triggerComponentRerender(componentName);
    
    // Update any component previews
    this.updateComponentPreview(componentName);
  }
  
  private updateStyleIsolation(componentName: string) {
    const styles = this.componentStyles.get(componentName);
    if (styles) {
      // Process CSS modules or scoped styles
      console.log(`🔒 Updating style isolation for '${componentName}'`);
    }
  }
  
  private triggerComponentRerender(componentName: string) {
    // Trigger component re-render
    console.log(`🔄 Re-rendering component instances of '${componentName}'`);
  }
  
  private updateComponentPreview(componentName: string) {
    // Update component in style guide or preview
    console.log(`👀 Updating preview for component '${componentName}'`);
  }
  
  private showComponentError(componentName: string, error: string) {
    // Show error overlay on component
    console.error(`❌ Showing error overlay for '${componentName}': ${error}`);
  }
  
  // Public API for component lifecycle
  mountComponent(componentName: string) {
    this.mountedComponents.add(componentName);
    console.log(`🏗️ Component '${componentName}' mounted`);
  }
  
  unmountComponent(componentName: string) {
    this.mountedComponents.delete(componentName);
    console.log(`🗑️ Component '${componentName}' unmounted`);
  }
  
  getMountedComponents(): string[] {
    return Array.from(this.mountedComponents);
  }
  
  getComponentStyles(componentName: string): string | null {
    return this.componentStyles.get(componentName) || null;
  }
  
  async start() {
    await this.hotReload.start();
    console.log('🧩 Component style developer started');
  }
  
  stop() {
    this.hotReload.stop();
    console.log('🛑 Component style developer stopped');
  }
  
  getStats() {
    return {
      mountedComponents: Array.from(this.mountedComponents),
      trackedComponents: Array.from(this.componentStyles.keys()),
      hotReloadStats: this.hotReload.getStats()
    };
  }
}

// Usage
const componentDev = new ComponentStyleDeveloper(cssEngine);

await componentDev.start();

// Register component lifecycle
componentDev.mountComponent('Button');
componentDev.mountComponent('Input');
componentDev.mountComponent('Modal');

// Monitor component development
setInterval(() => {
  const stats = componentDev.getStats();
  console.log('Component Dev Stats:', stats);
}, 10000);

// Cleanup when components unmount
componentDev.unmountComponent('Modal');
```

### CSS Debugging Assistant

```typescript
class CSSDebuggingAssistant {
  private hotReload: HotReloadManager;
  private debugHistory: Array<{
    timestamp: Date;
    path: string;
    action: string;
    content?: string;
    error?: string;
  }> = [];
  private maxHistorySize: number = 100;
  
  constructor(private cssEngine: any) {
    this.hotReload = new HotReloadBuilder()
      .watchPaths(['./styles', './src', './components'])
      .extensions(['.css', '.scss', '.less'])
      .debounce(50)                 // Quick feedback for debugging
      .validate(true)
      .maxRetries(1)               // Fail fast for debugging
      .verbose(true)
      .cssEngine(cssEngine)
      .build();
    
    this.setupDebugging();
  }
  
  private setupDebugging() {
    this.hotReload.on('css-changed', (event) => {
      this.addToHistory({
        timestamp: new Date(),
        path: event.path,
        action: 'changed',
        content: event.content
      });
      
      // Analyze CSS for common issues
      if (event.content) {
        this.analyzeCSSIssues(event.path, event.content);
      }
    });
    
    this.hotReload.on('reload-success', (event) => {
      this.addToHistory({
        timestamp: new Date(),
        path: event.path,
        action: 'reload-success'
      });
      
      console.log(`✅ CSS successfully applied from ${event.path}`);
    });
    
    this.hotReload.on('reload-error', (event) => {
      this.addToHistory({
        timestamp: new Date(),
        path: event.path,
        action: 'reload-error',
        error: event.error
      });
      
      // Provide debugging suggestions
      this.provideFix suggestions(event.path, event.error || '');
    });
  }
  
  private addToHistory(entry: any) {
    this.debugHistory.unshift(entry);
    
    // Limit history size
    if (this.debugHistory.length > this.maxHistorySize) {
      this.debugHistory = this.debugHistory.slice(0, this.maxHistorySize);
    }
  }
  
  private analyzeCSSIssues(filePath: string, content: string) {
    const issues: string[] = [];
    
    // Check for common CSS issues
    if (content.includes('important')) {
      issues.push('⚠️ Contains !important declarations');
    }
    
    if (content.match(/color:\s*#[0-9a-fA-F]{3,6}/g)) {
      const colors = content.match(/#[0-9a-fA-F]{3,6}/g);
      if (colors && colors.length > 10) {
        issues.push('⚠️ Many hardcoded colors detected');
      }
    }
    
    if (content.includes('position: absolute') && !content.includes('position: relative')) {
      issues.push('⚠️ Absolute positioning without relative parent');
    }
    
    if (content.match(/\d+px/g)) {
      const pxValues = content.match(/\d+px/g);
      if (pxValues && pxValues.length > 20) {
        issues.push('⚠️ Many hardcoded pixel values - consider using relative units');
      }
    }
    
    if (issues.length > 0) {
      console.log(`🔍 CSS Analysis for ${filePath}:`);
      issues.forEach(issue => console.log(`  ${issue}`));
    }
  }
  
  private provideFixSuggestions(filePath: string, error: string) {
    console.log(`🔧 Fix suggestions for ${filePath}:`);
    
    if (error.includes('syntax')) {
      console.log('  • Check for missing semicolons or braces');
      console.log('  • Validate CSS syntax with a linter');
    }
    
    if (error.includes('property')) {
      console.log('  • Verify CSS property names are correct');
      console.log('  • Check for typos in property names');
    }
    
    if (error.includes('selector')) {
      console.log('  • Ensure selectors are valid');
      console.log('  • Check for proper nesting in SCSS/LESS');
    }
    
    // Suggest recent working versions
    const recentSuccess = this.debugHistory.find(
      entry => entry.path === filePath && entry.action === 'reload-success'
    );
    
    if (recentSuccess) {
      console.log(`  • Last successful reload: ${recentSuccess.timestamp.toLocaleString()}`);
    }
  }
  
  // Debugging utilities
  getHistory(count?: number): any[] {
    return count ? this.debugHistory.slice(0, count) : this.debugHistory;
  }
  
  getErrorHistory(): any[] {
    return this.debugHistory.filter(entry => entry.action === 'reload-error');
  }
  
  getRecentChanges(minutes: number = 10): any[] {
    const cutoff = new Date(Date.now() - minutes * 60 * 1000);
    return this.debugHistory.filter(entry => entry.timestamp > cutoff);
  }
  
  analyzeErrorPatterns(): any {
    const errors = this.getErrorHistory();
    const patterns: Record<string, number> = {};
    
    errors.forEach(error => {
      if (error.error) {
        const key = error.error.split(':')[0]; // Get error type
        patterns[key] = (patterns[key] || 0) + 1;
      }
    });
    
    return {
      totalErrors: errors.length,
      patterns,
      mostCommonError: Object.entries(patterns)
        .sort(([,a], [,b]) => b - a)[0]?.[0] || null
    };
  }
  
  generateReport(): string {
    const stats = this.hotReload.getStats();
    const errorAnalysis = this.analyzeErrorPatterns();
    const recentChanges = this.getRecentChanges(30);
    
    return `
# CSS Hot Reload Debug Report

## Statistics
- Total Reloads: ${stats.reloadCount}
- Current Errors: ${stats.errors.length}
- Watched Paths: ${stats.watchedPaths.length}
- Is Reloading: ${stats.isReloading}

## Error Analysis
- Total Errors: ${errorAnalysis.totalErrors}
- Most Common Error: ${errorAnalysis.mostCommonError || 'None'}
- Error Patterns: ${JSON.stringify(errorAnalysis.patterns, null, 2)}

## Recent Activity (30 minutes)
${recentChanges.map(change => 
  `- ${change.timestamp.toLocaleString()}: ${change.action} (${change.path})`
).join('\n')}

## Current Watched Paths
${stats.watchedPaths.map(path => `- ${path}`).join('\n')}
    `.trim();
  }
  
  clearHistory() {
    this.debugHistory = [];
    console.log('🗑️ Debug history cleared');
  }
  
  async start() {
    await this.hotReload.start();
    console.log('🔍 CSS Debugging Assistant started');
  }
  
  stop() {
    this.hotReload.stop();
    console.log('🛑 CSS Debugging Assistant stopped');
  }
}

// Usage
const debugAssistant = new CSSDebuggingAssistant(cssEngine);

await debugAssistant.start();

// Generate debug reports
setInterval(() => {
  console.log(debugAssistant.generateReport());
}, 60000); // Every minute

// Analyze error patterns
setTimeout(() => {
  const patterns = debugAssistant.analyzeErrorPatterns();
  console.log('Error Patterns:', patterns);
}, 30000);
```

## Performance Considerations

```typescript
// Performance optimized hot reload
const optimizedHotReload = new HotReloadBuilder()
  .watchPaths(['./styles'])
  .debounce(200)                    // Higher debounce for performance
  .validate(false)                  // Disable validation for speed
  .maxRetries(1)                    // Fail fast
  .cssEngine(cssEngine)
  .build();

// Monitor performance
optimizedHotReload.on('reload-success', (event) => {
  const stats = optimizedHotReload.getStats();
  
  if (stats.reloadCount % 10 === 0) {
    console.log(`Performance check: ${stats.reloadCount} reloads completed`);
  }
});

// Cleanup resources periodically
setInterval(() => {
  optimizedHotReload.clearErrors();
  optimizedHotReload.clearBackups();
}, 300000); // Every 5 minutes
```

## Best Practices

1. **Development Only**
   - Hot reload automatically disables in production
   - Zero performance overhead in production builds
   - Use environment variables to control activation

2. **File Organization**
   - Organize CSS files in logical directory structures
   - Use consistent naming conventions
   - Keep related styles in the same directories

3. **Error Handling**
   - Always listen for error events
   - Provide visual feedback for reload status
   - Implement fallback mechanisms for critical errors

4. **Performance**
   - Use appropriate debounce durations
   - Limit the number of watched directories
   - Clean up resources regularly

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const devContainer = new ElementBuilderImpl('div')
  .class('development-container')
  .child(
    new HotReloadBuilder('dev-hot-reload')
      .watchPaths(['./app-styles'])
      .verbose(true)
      .cssEngine(cssEngine)
      .build()
  )
  .build();
```

The HotReload widget provides comprehensive CSS hot reloading capabilities with file system watching, debounced updates, validation, error recovery, and extensive development workflow integration for seamless CSS development in terminal applications.