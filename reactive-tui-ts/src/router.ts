/**
 * TUI Router System
 * 
 * Provides navigation, history management, and route-based rendering
 * for terminal user interface applications.
 */

import type { Element } from './types';

export interface Route {
    path: string;
    title: string;
    description?: string;
    component: () => Element;
    beforeEnter?: () => boolean | Promise<boolean>;
    afterEnter?: () => void;
}

export interface NavigationEvent {
    from: string;
    to: string;
    timestamp: number;
}

export interface RouterOptions {
    fullScreen?: boolean;
    clearOnNavigate?: boolean;
    enableHistory?: boolean;
    maxHistorySize?: number;
    onNavigate?: (event: NavigationEvent) => void;
    onError?: (error: Error, route?: string) => void;
}

export class TUIRouter {
    private routes: Map<string, Route> = new Map();
    private currentRoute: string = '';
    private history: string[] = [];
    private options: Required<RouterOptions>;
    
    constructor(options: RouterOptions = {}) {
        this.options = {
            fullScreen: true,
            clearOnNavigate: true,
            enableHistory: true,
            maxHistorySize: 50,
            onNavigate: () => {},
            onError: (error) => console.error('Router Error:', error),
            ...options
        };
    }
    
    /**
     * Register a route
     */
    route(path: string, route: Omit<Route, 'path'>): this {
        this.routes.set(path, { path, ...route });
        return this;
    }
    
    /**
     * Register multiple routes
     */
    registerRoutes(routes: Record<string, Omit<Route, 'path'>>): this {
        Object.entries(routes).forEach(([path, route]) => {
            this.route(path, route);
        });
        return this;
    }
    
    /**
     * Navigate to a route
     */
    async navigate(path: string, addToHistory: boolean = true): Promise<boolean> {
        const route = this.routes.get(path);
        if (!route) {
            this.options.onError(new Error(`Route not found: ${path}`), path);
            return false;
        }
        
        try {
            // Run beforeEnter guard
            if (route.beforeEnter) {
                const canEnter = await route.beforeEnter();
                if (!canEnter) {
                    return false;
                }
            }
            
            const previousRoute = this.currentRoute;
            
            // Add to history
            if (addToHistory && this.options.enableHistory) {
                this.addToHistory(this.currentRoute);
            }
            
            // Clear screen if full screen mode
            if (this.options.fullScreen && this.options.clearOnNavigate) {
                console.clear();
                if (this.options.fullScreen) {
                    // Move cursor to top-left and hide cursor
                    process.stdout.write('\x1B[H\x1B[?25l');
                }
            }
            
            // Update current route
            this.currentRoute = path;
            
            // Trigger navigation event
            this.options.onNavigate({
                from: previousRoute,
                to: path,
                timestamp: Date.now()
            });
            
            // Run afterEnter hook
            if (route.afterEnter) {
                route.afterEnter();
            }
            
            return true;
        } catch (error) {
            this.options.onError(error as Error, path);
            return false;
        }
    }
    
    /**
     * Go back in history
     */
    back(): Promise<boolean> {
        if (!this.options.enableHistory || this.history.length === 0) {
            return Promise.resolve(false);
        }
        
        const previousRoute = this.history.pop()!;
        return this.navigate(previousRoute, false);
    }
    
    /**
     * Go forward (if we tracked forward history)
     */
    forward(): Promise<boolean> {
        // For now, just refresh current route
        return this.refresh();
    }
    
    /**
     * Refresh current route
     */
    refresh(): Promise<boolean> {
        return this.navigate(this.currentRoute, false);
    }
    
    /**
     * Get current route component
     */
    getCurrentComponent(): Element | null {
        const route = this.routes.get(this.currentRoute);
        return route ? route.component() : null;
    }
    
    /**
     * Get current route info
     */
    getCurrentRoute(): Route | null {
        return this.routes.get(this.currentRoute) || null;
    }
    
    /**
     * Get all registered routes
     */
    getAllRoutes(): Route[] {
        return Array.from(this.routes.values());
    }
    
    /**
     * Check if route exists
     */
    hasRoute(path: string): boolean {
        return this.routes.has(path);
    }
    
    /**
     * Get route by path
     */
    getRoute(path: string): Route | undefined {
        return this.routes.get(path);
    }
    
    /**
     * Get navigation history
     */
    getHistory(): string[] {
        return [...this.history];
    }
    
    /**
     * Clear navigation history
     */
    clearHistory(): void {
        this.history = [];
    }
    
    /**
     * Set router options
     */
    setOptions(options: Partial<RouterOptions>): void {
        this.options = { ...this.options, ...options };
    }
    
    /**
     * Create navigation breadcrumb
     */
    getBreadcrumb(): string {
        const current = this.getCurrentRoute();
        if (!current) return '';
        
        const routeNames = this.history.slice(-3).map(path => {
            const route = this.routes.get(path);
            return route?.title || path;
        });
        
        if (current.title) {
            routeNames.push(current.title);
        }
        
        return routeNames.join(' → ');
    }
    
    /**
     * Create route navigation info
     */
    getNavigationInfo(): { current: number; total: number; canGoBack: boolean } {
        const routes = this.getAllRoutes();
        const currentIndex = routes.findIndex(r => r.path === this.currentRoute);
        
        return {
            current: currentIndex + 1,
            total: routes.length,
            canGoBack: this.history.length > 0
        };
    }
    
    /**
     * Navigate to next route in order
     */
    next(): Promise<boolean> {
        const routes = this.getAllRoutes();
        const currentIndex = routes.findIndex(r => r.path === this.currentRoute);
        const nextIndex = (currentIndex + 1) % routes.length;
        return this.navigate(routes[nextIndex].path);
    }
    
    /**
     * Navigate to previous route in order
     */
    previous(): Promise<boolean> {
        const routes = this.getAllRoutes();
        const currentIndex = routes.findIndex(r => r.path === this.currentRoute);
        const prevIndex = (currentIndex - 1 + routes.length) % routes.length;
        return this.navigate(routes[prevIndex].path);
    }
    
    /**
     * Add route to history
     */
    private addToHistory(route: string): void {
        if (!route) return;
        
        // Remove duplicate if it exists
        const index = this.history.indexOf(route);
        if (index !== -1) {
            this.history.splice(index, 1);
        }
        
        // Add to end
        this.history.push(route);
        
        // Trim history if too long
        if (this.history.length > this.options.maxHistorySize) {
            this.history = this.history.slice(-this.options.maxHistorySize);
        }
    }
    
    /**
     * Setup keyboard navigation
     */
    setupKeyboardNavigation(): void {
        process.stdin.setRawMode(true);
        process.stdin.resume();
        process.stdin.setEncoding('utf8');
        
        process.stdin.on('data', (key: string) => {
            this.handleKeyPress(key);
        });
        
        // Cleanup on exit
        process.on('SIGINT', () => {
            if (this.options.fullScreen) {
                process.stdout.write('\x1B[?25h'); // Show cursor
                console.clear();
            }
            process.exit(0);
        });
        
        // Handle terminal resize
        process.stdout.on('resize', () => {
            this.refresh();
        });
    }
    
    /**
     * Handle keyboard input
     */
    private handleKeyPress(key: string): void {
        switch (key) {
            case '\u0003': // Ctrl+C
            case 'q':
            case 'Q':
                if (this.options.fullScreen) {
                    process.stdout.write('\x1B[?25h'); // Show cursor
                    console.clear();
                }
                process.exit(0);
                break;
                
            case '\u001b[C': // Right arrow
            case ' ': // Space
            case '\r': // Enter
                this.next();
                break;
                
            case '\u001b[D': // Left arrow
                this.previous();
                break;
                
            case '\u001b[A': // Up arrow
            case 'b':
            case 'B':
                this.back();
                break;
                
            case '\u001b[B': // Down arrow
            case 'r':
            case 'R':
                this.refresh();
                break;
                
            case 'h':
            case 'H':
                this.showHelp();
                break;
                
            default:
                // Number keys for direct navigation
                const keyCode = key.charCodeAt(0);
                if (keyCode >= 49 && keyCode <= 57) { // 1-9
                    const routeIndex = keyCode - 49;
                    const routes = this.getAllRoutes();
                    if (routeIndex < routes.length) {
                        this.navigate(routes[routeIndex].path);
                    }
                }
                break;
        }
    }
    
    /**
     * Show navigation help
     */
    private showHelp(): void {
        if (this.options.fullScreen) {
            console.clear();
        }
        
        console.log('Navigation Help');
        console.log('===============');
        console.log('← →    Previous/Next route');
        console.log('↑ B    Back in history'); 
        console.log('↓ R    Refresh current route');
        console.log('Space  Next route');
        console.log('Enter  Next route');
        console.log('1-9    Direct route access');
        console.log('H      Show this help');
        console.log('Q      Quit application');
        console.log('');
        console.log('Routes:');
        this.getAllRoutes().forEach((route, index) => {
            const marker = route.path === this.currentRoute ? '→' : ' ';
            console.log(`${marker} ${index + 1}. ${route.title}`);
            if (route.description) {
                console.log(`    ${route.description}`);
            }
        });
        console.log('');
        console.log('Press any key to continue...');
        
        // Wait for key press
        process.stdin.once('data', () => {
            this.refresh();
        });
    }
}

/**
 * Create a new router instance
 */
export function createRouter(options?: RouterOptions): TUIRouter {
    return new TUIRouter(options);
}

/**
 * Router middleware for full-screen terminal apps
 */
export function fullScreenRouter(options?: Omit<RouterOptions, 'fullScreen'>): TUIRouter {
    return new TUIRouter({
        ...options,
        fullScreen: true,
        clearOnNavigate: true
    });
}

/**
 * Router middleware for inline terminal apps  
 */
export function inlineRouter(options?: Omit<RouterOptions, 'fullScreen'>): TUIRouter {
    return new TUIRouter({
        ...options,
        fullScreen: false,
        clearOnNavigate: false
    });
}