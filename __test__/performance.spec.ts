import test from 'ava'
import { 
    JsTuiApp, 
    JsToast, 
    JsToastManager,
    TuiUtils,
    JsColorDefinition 
} from '../index.js'

// Performance Test Suite
test('Performance - app creation speed', t => {
    const startTime = process.hrtime.bigint()
    
    // Create multiple apps quickly
    const apps = []
    for (let i = 0; i < 100; i++) {
        apps.push(new JsTuiApp())
    }
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.is(apps.length, 100, 'Should create 100 apps')
    t.true(durationMs < 1000, `App creation should be fast (${durationMs.toFixed(2)}ms for 100 apps)`)
})

test('Performance - element creation and manipulation', t => {
    const startTime = process.hrtime.bigint()
    
    // Create a complex DOM-like structure
    const root = TuiUtils.div()
    root.setId('performance-root')
    root.addClass('performance-test')
    
    for (let i = 0; i < 1000; i++) {
        const element = TuiUtils.div()
        element.setId(`element-${i}`)
        element.addClass(`class-${i % 10}`)
        element.setContent(`Content ${i}`)
        element.setAttribute('data-index', i.toString())
        
        if (i % 10 === 0) {
            element.makeFocusable(i)
        }
        
        root.addChild(element)
    }
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.true(durationMs < 5000, `Element creation should be fast (${durationMs.toFixed(2)}ms for 1000 elements)`)
})

test('Performance - CSS loading and validation', t => {
    const app = new JsTuiApp()
    
    // Generate large CSS content
    let largeCss = ''
    for (let i = 0; i < 1000; i++) {
        largeCss += `
        .class-${i} {
            background: #${(i * 1000).toString(16).padStart(6, '0').slice(-6)};
            color: #${((1000 - i) * 1000).toString(16).padStart(6, '0').slice(-6)};
            padding: ${i % 10}px;
            margin: ${i % 5}px;
            border: 1px solid #ccc;
        }
        `
    }
    
    const startTime = process.hrtime.bigint()
    
    t.notThrows(() => app.loadCss(largeCss), 'Should handle large CSS without errors')
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.true(durationMs < 10000, `CSS loading should be reasonable (${durationMs.toFixed(2)}ms for large CSS)`)
})

test('Performance - color operations', t => {
    const startTime = process.hrtime.bigint()
    
    const colors = []
    for (let i = 0; i < 10000; i++) {
        const r = i % 256
        const g = (i * 2) % 256
        const b = (i * 3) % 256
        
        const color = JsColorDefinition.rgb(r, g, b)
        colors.push(color)
        
        // Test RGB access
        const [rOut, , ] = color.getRgb()
        t.is(rOut, r, `RGB values should match for iteration ${i}`)
        
        // Test ANSI conversion
        color.toAnsi(false)
        color.toAnsi(true)
    }
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.is(colors.length, 10000, 'Should create 10000 colors')
    t.true(durationMs < 15000, `Color operations should be fast (${durationMs.toFixed(2)}ms for 10000 colors)`)
})

test('Performance - toast management stress test', t => {
    const manager = new JsToastManager(200, 50)
    
    const startTime = process.hrtime.bigint()
    
    // Create and manage many toasts
    for (let i = 0; i < 1000; i++) {
        const toastType = ['info', 'success', 'warning', 'error'][i % 4] as keyof typeof JsToast
        const toast = (JsToast as any)[toastType](`Toast message ${i}`)
        toast.setTitle(`Title ${i}`)
        toast.setDuration(1000 + (i % 5000))
        
        manager.showToast(toast)
        
        // Periodically clean up
        if (i % 100 === 0) {
            manager.cleanupExpired()
        }
    }
    
    // Final cleanup
    const expiredIds = manager.cleanupExpired()
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.true(Array.isArray(expiredIds), 'Cleanup should return array')
    t.true(durationMs < 10000, `Toast management should be fast (${durationMs.toFixed(2)}ms for 1000 toasts)`)
})

test('Performance - utility function calls', t => {
    const startTime = process.hrtime.bigint()
    
    // Test repeated utility calls with reduced iterations for realistic testing
    for (let i = 0; i < 1000; i++) {
        // Terminal size calls - these might be expensive in headless mode
        if (i % 10 === 0) {
            const [width, height] = TuiUtils.getTerminalSize()
            t.is(typeof width, 'number', 'Width should be number')
            t.is(typeof height, 'number', 'Height should be number')
        }
        
        // Element creation calls - should be fast
        TuiUtils.div()
        TuiUtils.button()
        TuiUtils.input()
        TuiUtils.text(`Text ${i}`)
        
        // CSS validation (every 50th iteration)
        if (i % 50 === 0) {
            TuiUtils.validateCss(`.test-${i} { color: red; }`)
        }
    }
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    // More realistic performance expectation
    t.true(durationMs < 10000, `Utility calls should be reasonable (${durationMs.toFixed(2)}ms for 1000 iterations)`)
    
    // Report actual performance for monitoring
    console.log(`Performance: ${durationMs.toFixed(2)}ms for 1000 utility operations`)
})

test('Memory - cleanup and garbage collection hints', t => {
    // This test checks that objects can be created and released properly
    const initialMemory = process.memoryUsage().heapUsed
    
    // Create many objects that should be GC-able
    for (let i = 0; i < 1000; i++) {
        const app = new JsTuiApp()
        app.setTitle(`App ${i}`)
        
        const elements = []
        for (let j = 0; j < 100; j++) {
            const element = TuiUtils.div()
            element.setId(`element-${i}-${j}`)
            elements.push(element)
        }
        
        // Let the objects go out of scope
    }
    
    // Force garbage collection if available
    if (global.gc) {
        global.gc()
    }
    
    const finalMemory = process.memoryUsage().heapUsed
    const memoryIncrease = finalMemory - initialMemory
    
    // Memory should not grow excessively (allowing for reasonable overhead)
    const maxReasonableIncrease = 100 * 1024 * 1024 // 100MB
    t.true(memoryIncrease < maxReasonableIncrease, 
        `Memory increase should be reasonable (${(memoryIncrease / 1024 / 1024).toFixed(2)}MB)`)
})

test('Performance - concurrent operations', async t => {
    // Test multiple operations happening concurrently
    const operations = []
    
    const startTime = process.hrtime.bigint()
    
    // Start multiple async-like operations
    for (let i = 0; i < 100; i++) {
        operations.push(new Promise<void>((resolve) => {
            // Simulate concurrent app usage
            const app = new JsTuiApp()
            app.setTitle(`Concurrent App ${i}`)
            app.loadCss(`.concurrent-${i} { background: blue; }`)
            
            const root = TuiUtils.div()
            root.setId(`concurrent-root-${i}`)
            
            for (let j = 0; j < 10; j++) {
                const child = TuiUtils.button()
                child.setContent(`Button ${j}`)
                root.addChild(child)
            }
            
            app.setComponent(root)
            app.start()
            
            // Use setImmediate to simulate async behavior
            setImmediate(resolve)
        }))
    }
    
    // Wait for all operations to complete
    await Promise.all(operations)
    
    const endTime = process.hrtime.bigint()
    const durationMs = Number(endTime - startTime) / 1_000_000
    
    t.true(durationMs < 30000, `Concurrent operations should complete reasonably fast (${durationMs.toFixed(2)}ms)`)
    t.is(operations.length, 100, 'All operations should complete')
})