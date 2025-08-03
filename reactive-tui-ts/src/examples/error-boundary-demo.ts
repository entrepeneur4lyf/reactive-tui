#!/usr/bin/env bun
/**
 * 🛡️ Error Boundary System Demo
 * 
 * Comprehensive demonstration of the TUI framework's error handling capabilities
 * including plugin failures, recovery strategies, and health monitoring.
 */

import { createApp, div, text, flexRow, flexColumn } from '../packages/tui-bun/src'
import { 
    Plugin, 
    WidgetPlugin, 
    PluginContext,
    PluginEvent,
    PluginResponse,
    PluginCapability,
    PluginMetadata,
    WidgetConfig
} from '../packages/tui-bun/src/plugin'
import { 
    createEnhancedPluginManager,
    withErrorBoundary,
    safePluginOperation
} from '../packages/tui-bun/src/enhanced-plugin-manager'
import { 
    ErrorCategory,
    ErrorSeverity,
    createErrorBoundaryManager
} from '../packages/tui-bun/src/error-boundary'
import { 
    createErrorReporter,
    ErrorAnalyticsEngine,
    AlertManager
} from '../packages/tui-bun/src/error-reporter'
import { panel, dashboardPanel } from '../packages/tui-bun/src/widgets/panel'
import { barItem, headerBar, statusBar } from '../packages/tui-bun/src/widgets/bar'
import type { Element } from '../packages/tui-bun/src/generated-types'

/**
 * 💥 Faulty Widget Plugin (for testing error boundaries)
 */
class FaultyWidgetPlugin extends WidgetPlugin {
    readonly id = 'faulty-widget'
    readonly widgetType = 'faulty-widget'
    
    readonly metadata: PluginMetadata = {
        name: 'Faulty Widget Plugin',
        version: '1.0.0',
        author: 'Error Boundary Test',
        description: 'A deliberately faulty plugin for testing error boundaries',
        dependencies: [],
        capabilities: [PluginCapability.WidgetProvider],
        tags: ['test', 'error', 'demo']
    }

    private failureMode: 'initialization' | 'rendering' | 'random' | 'none' = 'none'
    private failureCount = 0

    get configSchema() {
        return {
            type: 'object',
            properties: {
                failureMode: { type: 'string', enum: ['initialization', 'rendering', 'random', 'none'] },
                failureRate: { type: 'number', minimum: 0, maximum: 1 },
                message: { type: 'string' }
            },
            required: ['message']
        }
    }

    async initialize(context: PluginContext): Promise<void> {
        console.log('💥 Faulty Widget Plugin initialized')
        
        if (this.failureMode === 'initialization') {
            throw new Error('Simulated initialization failure')
        }
    }

    async cleanup(): Promise<void> {
        console.log('💥 Faulty Widget Plugin cleaned up')
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom' && event.eventType === 'set-failure-mode') {
            this.failureMode = event.data?.mode || 'none'
            console.log(`💥 Failure mode set to: ${this.failureMode}`)
            return { type: 'continue' }
        }
        return null
    }

    createInstance(config: WidgetConfig): WidgetPlugin {
        return new FaultyWidgetInstance(config, this.failureMode)
    }

    render(): Element {
        return div().child(text('Faulty Widget Plugin')).build()
    }
}

class FaultyWidgetInstance extends WidgetPlugin {
    readonly id: string
    readonly widgetType = 'faulty-widget'
    readonly metadata: PluginMetadata
    private failureMode: string
    public config: WidgetConfig
    
    constructor(config: WidgetConfig, failureMode: string) {
        super(config)
        this.config = config
        this.id = config.id
        this.failureMode = failureMode
        this.metadata = new FaultyWidgetPlugin().metadata
    }

    get configSchema() { return new FaultyWidgetPlugin().configSchema }
    async initialize(): Promise<void> {}
    async cleanup(): Promise<void> {}
    createInstance(config: WidgetConfig): WidgetPlugin { 
        return new FaultyWidgetInstance(config, this.failureMode) 
    }

    render(): Element {
        const { message = 'Faulty Widget', failureRate = 0.3 } = this.config.properties
        
        // Simulate random failures
        if (this.failureMode === 'rendering') {
            throw new Error(`Simulated rendering failure: ${message}`)
        }
        
        if (this.failureMode === 'random' && Math.random() < failureRate) {
            throw new Error(`Random failure occurred: ${message}`)
        }
        
        return div()
            .class('faulty-widget')
            .children([
                text(`💥 ${message}`),
                text(`Mode: ${this.failureMode}`),
                text(`Status: Working (for now...)`)
            ])
            .build()
    }
}

/**
 * 🎯 Stress Test Plugin (generates controlled errors)
 */
class StressTestPlugin extends Plugin {
    readonly id = 'stress-test'
    
    readonly metadata: PluginMetadata = {
        name: 'Stress Test Plugin',
        version: '1.0.0',
        author: 'Error Boundary Test',
        description: 'Generates controlled errors for stress testing',
        dependencies: [],
        capabilities: [PluginCapability.EventInterceptor],
        tags: ['test', 'stress', 'demo']
    }

    private isStressing = false
    private stressInterval?: NodeJS.Timeout

    async initialize(context: PluginContext): Promise<void> {
        console.log('🎯 Stress Test Plugin initialized')
    }

    async cleanup(): Promise<void> {
        this.stopStressTesting()
        console.log('🎯 Stress Test Plugin cleaned up')
    }

    handleEvent(event: PluginEvent): PluginResponse | null {
        if (event.type === 'custom') {
            switch (event.eventType) {
                case 'start-stress-test':
                    this.startStressTesting()
                    return { type: 'continue' }
                case 'stop-stress-test':
                    this.stopStressTesting()
                    return { type: 'continue' }
            }
        }
        return null
    }

    private startStressTesting(): void {
        if (this.isStressing) return
        
        this.isStressing = true
        console.log('🎯 Starting stress test...')
        
        this.stressInterval = setInterval(() => {
            // Generate random errors
            const errorTypes = [
                () => { throw new Error('Simulated network timeout') },
                () => { throw new Error('Simulated memory allocation failure') },
                () => { throw new Error('Simulated configuration error') },
                () => { throw new Error('Simulated dependency failure') }
            ]
            
            const randomError = errorTypes[Math.floor(Math.random() * errorTypes.length)]
            
            try {
                randomError()
            } catch (error) {
                // These errors will be caught by the error boundary system
                console.warn('🎯 Stress test generated error:', error)
            }
        }, 2000) // Generate error every 2 seconds
    }

    private stopStressTesting(): void {
        if (!this.isStressing) return
        
        this.isStressing = false
        if (this.stressInterval) {
            clearInterval(this.stressInterval)
            this.stressInterval = undefined
        }
        console.log('🎯 Stress test stopped')
    }
}

/**
 * 🚀 Main Error Boundary Demo Application
 */
async function main() {
    console.log('╔══════════════════════════════════════════╗')
    console.log('║   🛡️ Error Boundary System Demo 🛡️      ║')
    console.log('╚══════════════════════════════════════════╝')
    console.log()

    // Create error boundary manager with comprehensive config
    const errorBoundaryManager = createErrorBoundaryManager({
        id: 'demo-error-boundary',
        enableLogging: true,
        enableReporting: true,
        autoRecovery: true,
        maxErrorHistory: 100
    })

    // Add error reporters
    const consoleReporter = createErrorReporter('console')
    errorBoundaryManager.addErrorReporter(consoleReporter)

    // Create analytics engine
    const analytics = new ErrorAnalyticsEngine()
    
    // Create alert manager
    const alertManager = new AlertManager(
        { 
            enabled: true,
            enableAlerts: true,
            alertThresholds: { errorRate: 5, criticalErrors: 2, pluginFailures: 2 }
        } as any,
        analytics
    )

    // Add console alert
    alertManager.addAlert({
        type: 'console',
        target: '',
        enabled: true,
        throttle: 60000 // 1 minute
    })

    // Create enhanced plugin manager
    const pluginManager = createEnhancedPluginManager({
        id: 'demo-error-boundary',
        enableLogging: true,
        autoRecovery: true
    })

    // Register test plugins
    await pluginManager.register(new FaultyWidgetPlugin())
    await pluginManager.register(new StressTestPlugin())

    // Application state
    let currentFailureMode = 'none'
    let stressTestActive = false

    // Create the app with error boundaries
    const app = createApp({
        component: () => {
            return flexColumn([])
                .child(
                    headerBar('header')
                        .item(barItem('🛡️ Error Boundary Demo', 'left'))
                        .item(barItem(`Mode: ${currentFailureMode}`, 'center'))
                        .item(barItem('Press Q to quit', 'right'))
                        .build()
                )
                .child(
                    flexRow([])
                        .class('flex-1')
                        .children([
                            // Test Widgets Panel
                            dashboardPanel({
                                id: 'test-widgets',
                                title: 'Test Widgets'
                            })
                            .child(
                                flexColumn([])
                                    .class('p-2')
                                    .children([
                                        // Working Widget
                                        div()
                                            .class('mb-2')
                                            .child(
                                                withErrorBoundary(() => {
                                                    return div()
                                                        .class('working-widget')
                                                        .children([
                                                            text('✅ Working Widget'),
                                                            text('This widget works perfectly'),
                                                            text('No errors here!')
                                                        ])
                                                        .build()
                                                })()
                                            ),
                                        
                                        // Faulty Widget (with error boundary)
                                        div()
                                            .class('mb-2')
                                            .child(
                                                withErrorBoundary(() => {
                                                    const config: WidgetConfig = {
                                                        id: 'faulty-test',
                                                        widgetType: 'faulty-widget',
                                                        properties: {
                                                            message: 'Test Faulty Widget',
                                                            failureMode: currentFailureMode,
                                                            failureRate: 0.5
                                                        },
                                                        cssClasses: ['test-widget'],
                                                        eventHandlers: {}
                                                    }
                                                    const widget = pluginManager.createWidget('faulty-widget', config)
                                                    return widget.render()
                                                }, (error) => 
                                                    div()
                                                        .class('error-fallback')
                                                        .children([
                                                            text('🚨 Widget Error Detected'),
                                                            text(`Error: ${error.message}`),
                                                            text('Fallback UI activated'),
                                                            text('Check health dashboard for details')
                                                        ])
                                                        .build()
                                                )()
                                            )
                                    ])
                            ),
                            
                            // System Health Panel
                            panel({
                                id: 'system-health',
                                title: 'System Health & Analytics'
                            })
                            .child(
                                flexColumn([])
                                    .class('p-2')
                                    .children([
                                        // Health Dashboard
                                        div()
                                            .class('mb-4')
                                            .child(pluginManager.createHealthDashboard()),
                                        
                                        // Error Analytics
                                        div()
                                            .class('mb-4')
                                            .children([
                                                text('📊 Error Analytics'),
                                                text('─'.repeat(20)),
                                                ...(() => {
                                                    const analyticsData = analytics.getAnalytics()
                                                    return [
                                                        text(`Total Errors: ${analyticsData.totalErrors}`),
                                                        text(`Error Rate: ${analyticsData.errorRate}/min`),
                                                        text(`Top Error: ${analyticsData.topErrors[0]?.message || 'None'}`),
                                                    ]
                                                })()
                                            ]),
                                        
                                        // Controls
                                        div()
                                            .children([
                                                text('🎮 Test Controls'),
                                                text('─'.repeat(20)),
                                                text('[1] Set Normal Mode'),
                                                text('[2] Set Rendering Failure Mode'),
                                                text('[3] Set Random Failure Mode'),
                                                text('[4] Toggle Stress Test'),
                                                text('[5] Clear Error History'),
                                                text('[Q] Quit Demo')
                                            ])
                                    ])
                            )
                        ])
                )
                .child(
                    statusBar('status')
                        .item(barItem('Error Boundary System Active', 'left'))
                        .item(barItem(`Stress Test: ${stressTestActive ? 'ON' : 'OFF'}`, 'center'))
                        .item(barItem('[1-5] Controls  [Q] Quit', 'right'))
                        .build()
                )
                .build()
        }
    })

    // Simulate some user interactions for demo
    console.log('\n🎮 Demo Controls:')
    console.log('1 - Normal mode (no failures)')
    console.log('2 - Rendering failure mode')
    console.log('3 - Random failure mode')
    console.log('4 - Toggle stress test')
    console.log('5 - Clear error history')
    console.log('Q - Quit demo')
    console.log('\nPress keys to test error boundaries...\n')

    // Start periodic health checks
    setInterval(() => {
        alertManager.checkAlerts()
    }, 10000) // Check every 10 seconds

    // Run the app
    await app.run()
}

// Run the demo
main().catch(console.error)
