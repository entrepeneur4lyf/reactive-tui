/**
 * Comprehensive Error Boundary System for TUI Plugin Framework
 * 
 * Provides robust error handling, recovery, and monitoring for plugin failures
 * with graceful degradation and detailed error reporting.
 */

import type { Element, Component } from './types'
import { div, text } from './components'

// Error severity levels
export enum ErrorSeverity {
    LOW = 'low',
    MEDIUM = 'medium', 
    HIGH = 'high',
    CRITICAL = 'critical'
}

// Error categories for better classification
export enum ErrorCategory {
    PLUGIN_INITIALIZATION = 'plugin_initialization',
    WIDGET_CREATION = 'widget_creation',
    WIDGET_RENDERING = 'widget_rendering',
    DATA_PROVIDER = 'data_provider',
    THEME_PROVIDER = 'theme_provider',
    EVENT_HANDLING = 'event_handling',
    CONFIGURATION = 'configuration',
    DEPENDENCY = 'dependency',
    NETWORK = 'network',
    UNKNOWN = 'unknown'
}

// Comprehensive error information
export interface ErrorInfo {
    id: string
    timestamp: number
    message: string
    stack?: string
    category: ErrorCategory
    severity: ErrorSeverity
    pluginId?: string
    widgetType?: string
    context?: Record<string, any>
    recoverable: boolean
    retryCount: number
    maxRetries: number
}

// Error recovery strategies
export enum RecoveryStrategy {
    RETRY = 'retry',
    FALLBACK = 'fallback',
    DISABLE = 'disable',
    RESTART = 'restart',
    IGNORE = 'ignore'
}

// Recovery action configuration
export interface RecoveryAction {
    strategy: RecoveryStrategy
    delay?: number
    maxAttempts?: number
    fallbackComponent?: () => Element
    onRecovery?: (error: ErrorInfo) => void
    onFailure?: (error: ErrorInfo) => void
}

// Error boundary configuration
export interface ErrorBoundaryConfig {
    id: string
    fallbackComponent?: (error: ErrorInfo) => Element
    recoveryActions?: Map<ErrorCategory, RecoveryAction>
    enableLogging?: boolean
    enableReporting?: boolean
    reportingEndpoint?: string
    maxErrorHistory?: number
    autoRecovery?: boolean
}

// Error reporter interface
export interface ErrorReporter {
    report(error: ErrorInfo): Promise<void>
    reportBatch(errors: ErrorInfo[]): Promise<void>
}

// Plugin health status
export interface PluginHealth {
    pluginId: string
    status: 'healthy' | 'degraded' | 'failed' | 'disabled'
    errorCount: number
    lastError?: ErrorInfo
    uptime: number
    lastHealthCheck: number
}

/**
 * Central Error Boundary Manager
 * Handles all error boundary operations and plugin health monitoring
 */
export class ErrorBoundaryManager {
    private static instance: ErrorBoundaryManager
    private errorHistory: ErrorInfo[] = []
    private pluginHealth = new Map<string, PluginHealth>()
    private errorReporters: ErrorReporter[] = []
    private config: ErrorBoundaryConfig
    private healthCheckInterval?: NodeJS.Timeout

    constructor(config: ErrorBoundaryConfig) {
        this.config = {
            enableLogging: true,
            enableReporting: false,
            maxErrorHistory: 1000,
            autoRecovery: true,
            ...config
        }
        
        if (this.config.autoRecovery) {
            this.startHealthMonitoring()
        }
    }

    static getInstance(config?: ErrorBoundaryConfig): ErrorBoundaryManager {
        if (!ErrorBoundaryManager.instance) {
            if (!config) {
                throw new Error('ErrorBoundaryManager must be initialized with config')
            }
            ErrorBoundaryManager.instance = new ErrorBoundaryManager(config)
        }
        return ErrorBoundaryManager.instance
    }

    /**
     * Handle an error with comprehensive logging and recovery
     */
    async handleError(
        error: Error,
        category: ErrorCategory,
        severity: ErrorSeverity,
        context?: Record<string, any>
    ): Promise<ErrorInfo> {
        const errorInfo: ErrorInfo = {
            id: this.generateErrorId(),
            timestamp: Date.now(),
            message: error.message,
            stack: error.stack,
            category,
            severity,
            pluginId: context?.pluginId,
            widgetType: context?.widgetType,
            context,
            recoverable: this.isRecoverable(category, severity),
            retryCount: 0,
            maxRetries: this.getMaxRetries(category, severity)
        }

        // Add to error history
        this.addToHistory(errorInfo)

        // Update plugin health
        if (errorInfo.pluginId) {
            this.updatePluginHealth(errorInfo.pluginId, errorInfo)
        }

        // Log error
        if (this.config.enableLogging) {
            this.logError(errorInfo)
        }

        // Report error
        if (this.config.enableReporting) {
            await this.reportError(errorInfo)
        }

        // Attempt recovery if enabled
        if (this.config.autoRecovery && errorInfo.recoverable) {
            await this.attemptRecovery(errorInfo)
        }

        return errorInfo
    }

    /**
     * Create an error boundary component for widgets
     */
    createErrorBoundary(
        componentFactory: () => Element,
        config?: Partial<ErrorBoundaryConfig>
    ): () => Element {
        const boundaryConfig = { ...this.config, ...config }
        
        return () => {
            try {
                return componentFactory()
            } catch (error) {
                // Handle error synchronously for UI rendering
                const errorInfo: ErrorInfo = {
                    id: this.generateErrorId(),
                    timestamp: Date.now(),
                    message: (error as Error).message,
                    stack: (error as Error).stack,
                    category: ErrorCategory.WIDGET_RENDERING,
                    severity: ErrorSeverity.MEDIUM,
                    context: { boundary: boundaryConfig.id },
                    recoverable: true,
                    retryCount: 0,
                    maxRetries: 2
                }

                // Log error asynchronously
                this.handleError(
                    error as Error,
                    ErrorCategory.WIDGET_RENDERING,
                    ErrorSeverity.MEDIUM,
                    { boundary: boundaryConfig.id }
                ).catch(console.error)

                // Return fallback component
                if (boundaryConfig.fallbackComponent) {
                    try {
                        return boundaryConfig.fallbackComponent(errorInfo)
                    } catch (fallbackError) {
                        console.error('Fallback component failed:', fallbackError)
                        return this.createDefaultErrorComponent(errorInfo)
                    }
                }

                return this.createDefaultErrorComponent(errorInfo)
            }
        }
    }

    /**
     * Wrap plugin operations with error handling
     */
    wrapPluginOperation<T>(
        operation: () => T,
        pluginId: string,
        category: ErrorCategory,
        fallback?: T
    ): T {
        try {
            return operation()
        } catch (error) {
            this.handleError(
                error as Error,
                category,
                ErrorSeverity.HIGH,
                { pluginId }
            )

            if (fallback !== undefined) {
                return fallback
            }

            throw error
        }
    }

    /**
     * Get plugin health status
     */
    getPluginHealth(pluginId: string): PluginHealth | undefined {
        return this.pluginHealth.get(pluginId)
    }

    /**
     * Get all plugin health statuses
     */
    getAllPluginHealth(): Map<string, PluginHealth> {
        return new Map(this.pluginHealth)
    }

    /**
     * Get error history with optional filtering
     */
    getErrorHistory(filter?: {
        category?: ErrorCategory
        severity?: ErrorSeverity
        pluginId?: string
        since?: number
    }): ErrorInfo[] {
        let errors = [...this.errorHistory]

        if (filter) {
            errors = errors.filter(error => {
                if (filter.category && error.category !== filter.category) return false
                if (filter.severity && error.severity !== filter.severity) return false
                if (filter.pluginId && error.pluginId !== filter.pluginId) return false
                if (filter.since && error.timestamp < filter.since) return false
                return true
            })
        }

        return errors.sort((a, b) => b.timestamp - a.timestamp)
    }

    /**
     * Clear error history
     */
    clearErrorHistory(): void {
        this.errorHistory = []
    }

    /**
     * Add error reporter
     */
    addErrorReporter(reporter: ErrorReporter): void {
        this.errorReporters.push(reporter)
    }

    /**
     * Remove error reporter
     */
    removeErrorReporter(reporter: ErrorReporter): void {
        const index = this.errorReporters.indexOf(reporter)
        if (index > -1) {
            this.errorReporters.splice(index, 1)
        }
    }

    // Private methods
    private generateErrorId(): string {
        return `error_${Date.now()}_${Math.random().toString(36).substr(2, 9)}`
    }

    private isRecoverable(category: ErrorCategory, severity: ErrorSeverity): boolean {
        // Critical errors are generally not recoverable
        if (severity === ErrorSeverity.CRITICAL) return false
        
        // Some categories are more recoverable than others
        const recoverableCategories = [
            ErrorCategory.WIDGET_RENDERING,
            ErrorCategory.DATA_PROVIDER,
            ErrorCategory.THEME_PROVIDER,
            ErrorCategory.NETWORK
        ]
        
        return recoverableCategories.includes(category)
    }

    private getMaxRetries(category: ErrorCategory, severity: ErrorSeverity): number {
        if (severity === ErrorSeverity.CRITICAL) return 0
        if (severity === ErrorSeverity.HIGH) return 1
        if (category === ErrorCategory.NETWORK) return 3
        return 2
    }

    private addToHistory(error: ErrorInfo): void {
        this.errorHistory.push(error)
        
        // Trim history if it exceeds max size
        if (this.errorHistory.length > (this.config.maxErrorHistory || 1000)) {
            this.errorHistory = this.errorHistory.slice(-this.config.maxErrorHistory!)
        }
    }

    private updatePluginHealth(pluginId: string, error: ErrorInfo): void {
        const health = this.pluginHealth.get(pluginId) || {
            pluginId,
            status: 'healthy',
            errorCount: 0,
            uptime: Date.now(),
            lastHealthCheck: Date.now()
        }

        health.errorCount++
        health.lastError = error
        health.lastHealthCheck = Date.now()

        // Update status based on error count and severity
        if (error.severity === ErrorSeverity.CRITICAL) {
            health.status = 'failed'
        } else if (health.errorCount > 5) {
            health.status = 'degraded'
        } else if (health.errorCount > 10) {
            health.status = 'failed'
        }

        this.pluginHealth.set(pluginId, health)
    }

    private logError(error: ErrorInfo): void {
        const logLevel = this.getLogLevel(error.severity)
        const message = `[${error.category}] ${error.message}`
        const context = {
            id: error.id,
            pluginId: error.pluginId,
            widgetType: error.widgetType,
            context: error.context
        }

        console[logLevel](message, context)
        if (error.stack) {
            console.debug('Stack trace:', error.stack)
        }
    }

    private getLogLevel(severity: ErrorSeverity): 'error' | 'warn' | 'info' | 'debug' {
        switch (severity) {
            case ErrorSeverity.CRITICAL: return 'error'
            case ErrorSeverity.HIGH: return 'error'
            case ErrorSeverity.MEDIUM: return 'warn'
            case ErrorSeverity.LOW: return 'info'
            default: return 'debug'
        }
    }

    private async reportError(error: ErrorInfo): Promise<void> {
        const reportPromises = this.errorReporters.map(reporter => 
            reporter.report(error).catch(err => 
                console.warn('Error reporter failed:', err)
            )
        )
        
        await Promise.allSettled(reportPromises)
    }

    private async attemptRecovery(error: ErrorInfo): Promise<void> {
        const recoveryAction = this.config.recoveryActions?.get(error.category)
        if (!recoveryAction) return

        try {
            switch (recoveryAction.strategy) {
                case RecoveryStrategy.RETRY:
                    if (error.retryCount < error.maxRetries) {
                        error.retryCount++
                        // Retry logic would be implemented by the caller
                        recoveryAction.onRecovery?.(error)
                    }
                    break
                
                case RecoveryStrategy.FALLBACK:
                    recoveryAction.onRecovery?.(error)
                    break
                
                case RecoveryStrategy.DISABLE:
                    if (error.pluginId) {
                        const health = this.pluginHealth.get(error.pluginId)
                        if (health) {
                            health.status = 'disabled'
                        }
                    }
                    break
                
                case RecoveryStrategy.RESTART:
                    // Plugin restart logic would be implemented by the plugin manager
                    recoveryAction.onRecovery?.(error)
                    break
            }
        } catch (recoveryError) {
            console.error('Recovery attempt failed:', recoveryError)
            recoveryAction.onFailure?.(error)
        }
    }

    private createDefaultErrorComponent(error: ErrorInfo): Element {
        return div()
            .class('error-boundary')
            .children([
                div()
                    .class('error-icon')
                    .child(text('⚠️')),
                div()
                    .class('error-message')
                    .child(text(`Error: ${error.message}`)),
                div()
                    .class('error-details')
                    .child(text(`Category: ${error.category} | Severity: ${error.severity}`))
            ])
            .build()
    }

    private startHealthMonitoring(): void {
        this.healthCheckInterval = setInterval(() => {
            this.performHealthCheck()
        }, 30000) // Check every 30 seconds
    }

    private performHealthCheck(): void {
        const now = Date.now()
        
        for (const [pluginId, health] of this.pluginHealth) {
            // Reset error count if plugin has been stable
            if (now - health.lastHealthCheck > 300000 && health.errorCount > 0) { // 5 minutes
                health.errorCount = Math.max(0, health.errorCount - 1)
                
                // Improve status if error count decreased
                if (health.status === 'degraded' && health.errorCount <= 5) {
                    health.status = 'healthy'
                }
            }
            
            health.lastHealthCheck = now
        }
    }

    /**
     * Cleanup resources
     */
    destroy(): void {
        if (this.healthCheckInterval) {
            clearInterval(this.healthCheckInterval)
        }
        this.errorHistory = []
        this.pluginHealth.clear()
        this.errorReporters = []
    }
}

// Default error boundary configuration
export const defaultErrorBoundaryConfig: ErrorBoundaryConfig = {
    id: 'default-error-boundary',
    enableLogging: true,
    enableReporting: false,
    maxErrorHistory: 1000,
    autoRecovery: true,
    recoveryActions: new Map([
        [ErrorCategory.WIDGET_RENDERING, {
            strategy: RecoveryStrategy.FALLBACK,
            maxAttempts: 1
        }],
        [ErrorCategory.DATA_PROVIDER, {
            strategy: RecoveryStrategy.RETRY,
            delay: 1000,
            maxAttempts: 3
        }],
        [ErrorCategory.NETWORK, {
            strategy: RecoveryStrategy.RETRY,
            delay: 2000,
            maxAttempts: 5
        }]
    ])
}

// Convenience function to create error boundary manager
export function createErrorBoundaryManager(config?: Partial<ErrorBoundaryConfig>): ErrorBoundaryManager {
    return ErrorBoundaryManager.getInstance({
        ...defaultErrorBoundaryConfig,
        ...config
    })
}
