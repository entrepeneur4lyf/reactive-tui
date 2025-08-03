/**
 * Error Reporting System for TUI Plugin Framework
 * 
 * Provides comprehensive error reporting, analytics, and monitoring
 * with support for multiple reporting backends and real-time alerts.
 */

import type { ErrorInfo, ErrorReporter } from './error-boundary'

// Error reporting configuration
export interface ErrorReportingConfig {
    enabled: boolean
    endpoint?: string
    apiKey?: string
    batchSize: number
    flushInterval: number
    retryAttempts: number
    enableAnalytics: boolean
    enableAlerts: boolean
    alertThresholds: {
        errorRate: number      // errors per minute
        criticalErrors: number // critical errors per hour
        pluginFailures: number // plugin failures per hour
    }
}

// Error analytics data
export interface ErrorAnalytics {
    totalErrors: number
    errorsByCategory: Record<string, number>
    errorsBySeverity: Record<string, number>
    errorsByPlugin: Record<string, number>
    errorRate: number // errors per minute
    topErrors: Array<{
        message: string
        count: number
        lastOccurrence: number
    }>
    trends: {
        hourly: number[]
        daily: number[]
    }
}

// Alert configuration
export interface AlertConfig {
    type: 'email' | 'webhook' | 'console' | 'custom'
    target: string
    enabled: boolean
    throttle: number // minimum time between alerts in ms
}

/**
 * Console Error Reporter
 * Simple console-based error reporting for development
 */
export class ConsoleErrorReporter implements ErrorReporter {
    private config: ErrorReportingConfig

    constructor(config: Partial<ErrorReportingConfig> = {}) {
        this.config = {
            enabled: true,
            batchSize: 1,
            flushInterval: 0,
            retryAttempts: 0,
            enableAnalytics: false,
            enableAlerts: false,
            alertThresholds: {
                errorRate: 10,
                criticalErrors: 5,
                pluginFailures: 3
            },
            ...config
        }
    }

    async report(error: ErrorInfo): Promise<void> {
        if (!this.config.enabled) return

        const timestamp = new Date(error.timestamp).toISOString()
        const severity = this.getSeverityIcon(error.severity)
        
        console.group(`${severity} Error Report [${error.id}]`)
        console.log(`🕐 Time: ${timestamp}`)
        console.log(`📂 Category: ${error.category}`)
        console.log(`🔥 Severity: ${error.severity}`)
        console.log(`💬 Message: ${error.message}`)
        
        if (error.pluginId) {
            console.log(`🔌 Plugin: ${error.pluginId}`)
        }
        
        if (error.widgetType) {
            console.log(`🎛️ Widget: ${error.widgetType}`)
        }
        
        if (error.context) {
            console.log(`📋 Context:`, error.context)
        }
        
        if (error.stack) {
            console.log(`📚 Stack Trace:`)
            console.log(error.stack)
        }
        
        console.groupEnd()
    }

    async reportBatch(errors: ErrorInfo[]): Promise<void> {
        for (const error of errors) {
            await this.report(error)
        }
    }

    private getSeverityIcon(severity: string): string {
        switch (severity) {
            case 'critical': return '🚨'
            case 'high': return '❌'
            case 'medium': return '⚠️'
            case 'low': return 'ℹ️'
            default: return '🔍'
        }
    }
}

/**
 * HTTP Error Reporter
 * Reports errors to a remote HTTP endpoint
 */
export class HttpErrorReporter implements ErrorReporter {
    private config: ErrorReportingConfig
    private pendingErrors: ErrorInfo[] = []
    private flushTimer?: NodeJS.Timeout

    constructor(config: ErrorReportingConfig) {
        this.config = config
        
        if (this.config.flushInterval > 0) {
            this.startBatchTimer()
        }
    }

    async report(error: ErrorInfo): Promise<void> {
        if (!this.config.enabled || !this.config.endpoint) return

        if (this.config.batchSize > 1) {
            this.pendingErrors.push(error)
            
            if (this.pendingErrors.length >= this.config.batchSize) {
                await this.flush()
            }
        } else {
            await this.sendErrors([error])
        }
    }

    async reportBatch(errors: ErrorInfo[]): Promise<void> {
        if (!this.config.enabled || !this.config.endpoint) return
        
        await this.sendErrors(errors)
    }

    async flush(): Promise<void> {
        if (this.pendingErrors.length === 0) return
        
        const errors = [...this.pendingErrors]
        this.pendingErrors = []
        
        await this.sendErrors(errors)
    }

    private async sendErrors(errors: ErrorInfo[]): Promise<void> {
        const payload = {
            timestamp: Date.now(),
            errors,
            metadata: {
                userAgent: typeof navigator !== 'undefined' ? navigator.userAgent : 'Node.js',
                framework: 'tui-bun',
                version: '1.0.0'
            }
        }

        let attempt = 0
        while (attempt <= this.config.retryAttempts) {
            try {
                const response = await fetch(this.config.endpoint!, {
                    method: 'POST',
                    headers: {
                        'Content-Type': 'application/json',
                        ...(this.config.apiKey && { 'Authorization': `Bearer ${this.config.apiKey}` })
                    },
                    body: JSON.stringify(payload)
                })

                if (response.ok) {
                    console.debug(`Reported ${errors.length} errors to ${this.config.endpoint}`)
                    return
                } else {
                    throw new Error(`HTTP ${response.status}: ${response.statusText}`)
                }
            } catch (error) {
                attempt++
                if (attempt > this.config.retryAttempts) {
                    console.error('Failed to report errors after retries:', error)
                    return
                }
                
                // Exponential backoff
                await new Promise(resolve => setTimeout(resolve, Math.pow(2, attempt) * 1000))
            }
        }
    }

    private startBatchTimer(): void {
        this.flushTimer = setInterval(() => {
            this.flush().catch(console.error)
        }, this.config.flushInterval)
    }

    destroy(): void {
        if (this.flushTimer) {
            clearInterval(this.flushTimer)
        }
        this.flush().catch(console.error)
    }
}

/**
 * Error Analytics Engine
 * Provides comprehensive error analytics and insights
 */
export class ErrorAnalyticsEngine {
    private errors: ErrorInfo[] = []
    private hourlyBuckets: number[] = new Array(24).fill(0)
    private dailyBuckets: number[] = new Array(7).fill(0)
    private lastAnalyticsUpdate = Date.now()

    addError(error: ErrorInfo): void {
        this.errors.push(error)
        this.updateTimeBuckets(error.timestamp)
        
        // Keep only last 10000 errors for memory management
        if (this.errors.length > 10000) {
            this.errors = this.errors.slice(-10000)
        }
    }

    getAnalytics(): ErrorAnalytics {
        const now = Date.now()
        const oneMinuteAgo = now - 60000
        const recentErrors = this.errors.filter(e => e.timestamp > oneMinuteAgo)
        
        return {
            totalErrors: this.errors.length,
            errorsByCategory: this.groupBy(this.errors, 'category'),
            errorsBySeverity: this.groupBy(this.errors, 'severity'),
            errorsByPlugin: this.groupBy(
                this.errors.filter(e => e.pluginId), 
                'pluginId'
            ),
            errorRate: recentErrors.length,
            topErrors: this.getTopErrors(),
            trends: {
                hourly: [...this.hourlyBuckets],
                daily: [...this.dailyBuckets]
            }
        }
    }

    private groupBy(errors: ErrorInfo[], key: keyof ErrorInfo): Record<string, number> {
        return errors.reduce((acc, error) => {
            const value = error[key] as string
            if (value) {
                acc[value] = (acc[value] || 0) + 1
            }
            return acc
        }, {} as Record<string, number>)
    }

    private getTopErrors(): Array<{ message: string; count: number; lastOccurrence: number }> {
        const errorCounts = new Map<string, { count: number; lastOccurrence: number }>()
        
        for (const error of this.errors) {
            const existing = errorCounts.get(error.message)
            if (existing) {
                existing.count++
                existing.lastOccurrence = Math.max(existing.lastOccurrence, error.timestamp)
            } else {
                errorCounts.set(error.message, {
                    count: 1,
                    lastOccurrence: error.timestamp
                })
            }
        }
        
        return Array.from(errorCounts.entries())
            .map(([message, data]) => ({ message, ...data }))
            .sort((a, b) => b.count - a.count)
            .slice(0, 10)
    }

    private updateTimeBuckets(timestamp: number): void {
        const date = new Date(timestamp)
        const hour = date.getHours()
        const day = date.getDay()
        
        this.hourlyBuckets[hour]++
        this.dailyBuckets[day]++
    }

    reset(): void {
        this.errors = []
        this.hourlyBuckets.fill(0)
        this.dailyBuckets.fill(0)
    }
}

/**
 * Alert Manager
 * Handles error-based alerts and notifications
 */
export class AlertManager {
    private config: ErrorReportingConfig
    private alerts: AlertConfig[] = []
    private lastAlertTimes = new Map<string, number>()
    private analytics: ErrorAnalyticsEngine

    constructor(config: ErrorReportingConfig, analytics: ErrorAnalyticsEngine) {
        this.config = config
        this.analytics = analytics
    }

    addAlert(alert: AlertConfig): void {
        this.alerts.push(alert)
    }

    removeAlert(alert: AlertConfig): void {
        const index = this.alerts.indexOf(alert)
        if (index > -1) {
            this.alerts.splice(index, 1)
        }
    }

    async checkAlerts(): Promise<void> {
        if (!this.config.enableAlerts) return

        const analytics = this.analytics.getAnalytics()
        const now = Date.now()

        // Check error rate threshold
        if (analytics.errorRate > this.config.alertThresholds.errorRate) {
            await this.triggerAlert('error-rate', {
                type: 'Error Rate Alert',
                message: `High error rate detected: ${analytics.errorRate} errors/minute`,
                threshold: this.config.alertThresholds.errorRate,
                current: analytics.errorRate
            })
        }

        // Check critical errors
        const criticalErrors = analytics.errorsBySeverity['critical'] || 0
        if (criticalErrors > this.config.alertThresholds.criticalErrors) {
            await this.triggerAlert('critical-errors', {
                type: 'Critical Error Alert',
                message: `Multiple critical errors detected: ${criticalErrors} in the last hour`,
                threshold: this.config.alertThresholds.criticalErrors,
                current: criticalErrors
            })
        }
    }

    private async triggerAlert(alertType: string, data: any): Promise<void> {
        const lastAlert = this.lastAlertTimes.get(alertType) || 0
        const now = Date.now()
        
        // Check throttling
        if (now - lastAlert < 300000) { // 5 minutes throttle
            return
        }
        
        this.lastAlertTimes.set(alertType, now)
        
        for (const alert of this.alerts.filter(a => a.enabled)) {
            try {
                await this.sendAlert(alert, data)
            } catch (error) {
                console.error('Failed to send alert:', error)
            }
        }
    }

    private async sendAlert(alert: AlertConfig, data: any): Promise<void> {
        switch (alert.type) {
            case 'console':
                console.warn('🚨 ALERT:', data)
                break
                
            case 'webhook':
                await fetch(alert.target, {
                    method: 'POST',
                    headers: { 'Content-Type': 'application/json' },
                    body: JSON.stringify(data)
                })
                break
                
            // Add more alert types as needed
        }
    }
}

// Default error reporting configuration
export const defaultErrorReportingConfig: ErrorReportingConfig = {
    enabled: true,
    batchSize: 10,
    flushInterval: 30000, // 30 seconds
    retryAttempts: 3,
    enableAnalytics: true,
    enableAlerts: true,
    alertThresholds: {
        errorRate: 10,
        criticalErrors: 5,
        pluginFailures: 3
    }
}

// Convenience function to create error reporter
export function createErrorReporter(
    type: 'console' | 'http' = 'console',
    config?: Partial<ErrorReportingConfig>
): ErrorReporter {
    const fullConfig = { ...defaultErrorReportingConfig, ...config }
    
    switch (type) {
        case 'http':
            return new HttpErrorReporter(fullConfig)
        case 'console':
        default:
            return new ConsoleErrorReporter(fullConfig)
    }
}
