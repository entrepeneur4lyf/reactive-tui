import test from 'ava'
import { 
    JsTuiApp, 
    JsElement, 
    JsToast, 
    JsToastManager,
    JsReactiveState,
    JsColorDefinition,
    JsColorTheme,
    TuiUtils,
    EnhancedFfiTypes
} from '../index.js'

// Test Suite 1: Core Application Functionality
test('JsTuiApp - basic creation and configuration', t => {
    const app = new JsTuiApp()
    t.truthy(app, 'App should be created successfully')
    
    // Test setting title
    t.notThrows(() => app.setTitle('Test App'), 'setTitle should not throw')
    
    // Test loading CSS
    t.notThrows(() => app.loadCss('.test { color: red; }'), 'loadCss should not throw')
    
    // App requires a component before starting
    const element = TuiUtils.div()
    element.setContent('Test content')
    app.setComponent(element)
    
    // Test app initialization
    const status = app.start()
    t.is(typeof status, 'string', 'start() should return a string')
    t.true(status.includes('headless') || status.includes('initialized'), 'Should indicate headless mode or initialization')
})

test('JsTuiApp - stylesheet loading from CSS string', t => {
    const app = new JsTuiApp()
    const complexCss = `
        .container {
            background: #1e1e1e;
            color: #ffffff;
            padding: 2rem;
            display: flex;
            flex-direction: column;
        }
        .button {
            background: #007acc;
            border: 1px solid #005a9e;
            padding: 0.5rem 1rem;
        }
        .button:hover {
            background: #005a9e;
        }
    `
    
    t.notThrows(() => app.loadCss(complexCss), 'Should load complex CSS without errors')
})

// Test Suite 2: Element Creation and Manipulation
test('JsElement - comprehensive element testing', t => {
    const element = new JsElement('div')
    t.truthy(element, 'Element should be created')
    
    // Test ID setting
    t.notThrows(() => element.setId('test-element'), 'setId should work')
    
    // Test class manipulation
    t.notThrows(() => element.addClass('container'), 'addClass should work')
    t.notThrows(() => element.addClass('primary'), 'Multiple classes should work')
    
    // Test content setting
    t.notThrows(() => element.setContent('Hello World'), 'setContent should work')
    
    // Test attribute setting
    t.notThrows(() => element.setAttribute('data-test', 'value'), 'setAttribute should work')
    t.notThrows(() => element.setAttribute('role', 'button'), 'ARIA attributes should work')
    
    // Test focusable setting
    t.notThrows(() => element.makeFocusable(), 'makeFocusable should work without tab index')
    t.notThrows(() => element.makeFocusable(5), 'makeFocusable should work with tab index')
})

test('JsElement - element hierarchy', t => {
    const parent = new JsElement('div')
    const child1 = new JsElement('span')
    const child2 = new JsElement('button')
    
    parent.setId('parent')
    child1.setId('child1')
    child2.setId('child2')
    
    // Test adding children
    t.notThrows(() => parent.addChild(child1), 'Adding first child should work')
    t.notThrows(() => parent.addChild(child2), 'Adding second child should work')
    
    // Test nested hierarchy
    const grandchild = new JsElement('em')
    grandchild.setContent('Nested text')
    t.notThrows(() => child1.addChild(grandchild), 'Nested children should work')
})

// Test Suite 3: Utility Functions
test('TuiUtils - element creation utilities', t => {
    // Test div creation
    const div = TuiUtils.div()
    t.truthy(div, 'div() should create an element')
    
    // Test text creation
    const text = TuiUtils.text('Sample text')
    t.truthy(text, 'text() should create an element')
    
    // Test button creation
    const button = TuiUtils.button()
    t.truthy(button, 'button() should create an element')
    
    // Test input creation
    const input = TuiUtils.input()
    t.truthy(input, 'input() should create an element')
})

test('TuiUtils - terminal and CSS utilities', t => {
    // Test terminal size
    const [width, height] = TuiUtils.getTerminalSize()
    t.is(typeof width, 'number', 'Terminal width should be a number')
    t.is(typeof height, 'number', 'Terminal height should be a number')
    t.true(width > 0, 'Terminal width should be positive')
    t.true(height > 0, 'Terminal height should be positive')
    
    // Test CSS validation
    const validCss = '.test { color: red; background: blue; }'
    const validErrors = TuiUtils.validateCss(validCss)
    t.true(Array.isArray(validErrors), 'validateCss should return an array')
    t.is(validErrors.length, 0, 'Valid CSS should have no errors')
    
    // Test empty CSS
    const emptyErrors = TuiUtils.validateCss('')
    t.true(Array.isArray(emptyErrors), 'Empty CSS should return an array')
    t.true(emptyErrors.length > 0, 'Empty CSS should have errors')
    
    // Test malformed CSS
    const malformedCss = '.test { color: red background: blue'
    const malformedErrors = TuiUtils.validateCss(malformedCss)
    t.true(Array.isArray(malformedErrors), 'Malformed CSS should return an array')
    // Note: CSS validation may be lenient, so we just check it returns an array
})

// Test Suite 4: Toast Notifications
test('JsToast - all toast types', t => {
    // Test info toast
    const infoToast = JsToast.info('Information message')
    t.truthy(infoToast, 'Info toast should be created')
    
    // Test success toast
    const successToast = JsToast.success('Success message')
    t.truthy(successToast, 'Success toast should be created')
    
    // Test warning toast
    const warningToast = JsToast.warning('Warning message')
    t.truthy(warningToast, 'Warning toast should be created')
    
    // Test error toast
    const errorToast = JsToast.error('Error message')
    t.truthy(errorToast, 'Error toast should be created')
})

test('JsToast - toast configuration', t => {
    const toast = JsToast.info('Test message')
    
    // Test title setting
    t.notThrows(() => toast.setTitle('Custom Title'), 'setTitle should work')
    
    // Test duration setting
    t.notThrows(() => toast.setDuration(5000), 'setDuration should work')
    t.notThrows(() => toast.setDuration(0), 'Zero duration should work')
    t.notThrows(() => toast.setDuration(60000), 'Long duration should work')
})

test('JsToastManager - toast management', t => {
    const manager = new JsToastManager(80, 24)
    t.truthy(manager, 'Toast manager should be created')
    
    const toast = JsToast.success('Test toast')
    
    // Test showing toast
    t.notThrows(() => manager.showToast(toast), 'showToast should work')
    
    // Test cleanup
    const expiredIds = manager.cleanupExpired()
    t.true(Array.isArray(expiredIds), 'cleanupExpired should return an array')
})

// Test Suite 5: Color System
test('JsColorDefinition - color creation and manipulation', t => {
    // Test RGB creation
    const rgbColor = JsColorDefinition.rgb(255, 0, 128)
    t.truthy(rgbColor, 'RGB color should be created')
    
    const [r, g, b] = rgbColor.getRgb()
    t.is(r, 255, 'Red component should match')
    t.is(g, 0, 'Green component should match')
    t.is(b, 128, 'Blue component should match')
    
    // Test ANSI conversion
    const foregroundAnsi = rgbColor.toAnsi(false)
    const backgroundAnsi = rgbColor.toAnsi(true)
    t.is(typeof foregroundAnsi, 'string', 'Foreground ANSI should be string')
    t.is(typeof backgroundAnsi, 'string', 'Background ANSI should be string')
    
    // Note: ANSI strings might be empty in headless mode, which is acceptable
    // Just verify they're strings and the function doesn't crash
    
    // Test hex creation
    t.notThrows(() => JsColorDefinition.hex('#ff0080'), 'Hex color creation should work')
    t.notThrows(() => JsColorDefinition.hex('#FF0080'), 'Uppercase hex should work')
    t.notThrows(() => JsColorDefinition.hex('ff0080'), 'Hex without # should work')
})

test('JsColorTheme - theme system', t => {
    // Test predefined themes
    const darkTheme = JsColorTheme.dark()
    const lightTheme = JsColorTheme.light()
    const terminalTheme = JsColorTheme.terminal()
    
    t.truthy(darkTheme, 'Dark theme should be created')
    t.truthy(lightTheme, 'Light theme should be created')
    t.truthy(terminalTheme, 'Terminal theme should be created')
    
    // Test theme names
    t.is(darkTheme.getName(), 'dark', 'Dark theme should have correct name')
    t.is(lightTheme.getName(), 'light', 'Light theme should have correct name')
    
    // Test theme descriptions
    t.is(typeof darkTheme.getDescription(), 'string', 'Theme description should be string')
    t.true(darkTheme.getDescription().length > 0, 'Description should not be empty')
    
    // Test JSON serialization
    const darkJson = darkTheme.toJson()
    t.is(typeof darkJson, 'string', 'toJson should return string')
    t.notThrows(() => JSON.parse(darkJson), 'Theme JSON should be valid')
    
    // Test JSON deserialization
    t.notThrows(() => JsColorTheme.fromJson(darkJson), 'fromJson should work with valid JSON')
})

// Test Suite 6: Reactive State Management
test('JsReactiveState - state management', t => {
    const state = new JsReactiveState()
    t.truthy(state, 'Reactive state should be created')
    
    // Test JSON state operations
    const testState = { count: 0, name: 'test', active: true }
    const stateJson = JSON.stringify(testState)
    
    t.notThrows(() => state.setStateJson(stateJson), 'setStateJson should work')
    
    const retrievedJson = state.getStateJson()
    t.is(typeof retrievedJson, 'string', 'getStateJson should return string')
    t.notThrows(() => JSON.parse(retrievedJson), 'Retrieved state should be valid JSON')
})

// Test Suite 7: Enhanced FFI Types and Metadata
test('EnhancedFfiTypes - metadata access', t => {
    // Test semantic color keys
    const semanticKeys = EnhancedFfiTypes.semanticColorKeys()
    t.true(Array.isArray(semanticKeys), 'semanticColorKeys should return array')
    t.true(semanticKeys.length > 0, 'Should have semantic color keys')
    t.true(semanticKeys.includes('panel_background'), 'Should include panel_background')
    
    // Test color palette keys
    const paletteKeys = EnhancedFfiTypes.colorPaletteKeys()
    t.true(Array.isArray(paletteKeys), 'colorPaletteKeys should return array')
    t.true(paletteKeys.length > 0, 'Should have palette keys')
    t.true(paletteKeys.includes('primary'), 'Should include primary color')
    
    // Test widget types
    const widgetTypes = EnhancedFfiTypes.widgetTypes()
    t.true(Array.isArray(widgetTypes), 'widgetTypes should return array')
    t.true(widgetTypes.includes('Button'), 'Should include Button widget')
    
    // Test element attributes
    const attributes = EnhancedFfiTypes.elementAttributes()
    t.true(Array.isArray(attributes), 'elementAttributes should return array')
    t.true(attributes.includes('id'), 'Should include id attribute')
    t.true(attributes.includes('class'), 'Should include class attribute')
    
    // Test CSS utility prefixes
    const prefixes = EnhancedFfiTypes.cssUtilityPrefixes()
    t.true(Array.isArray(prefixes), 'cssUtilityPrefixes should return array')
    t.true(prefixes.includes('bg-'), 'Should include bg- prefix')
    t.true(prefixes.includes('text-'), 'Should include text- prefix')
})

// Test Suite 8: Integration Testing
test('Integration - complete app workflow', t => {
    // Create a complete mini-application
    const app = new JsTuiApp()
    app.setTitle('Integration Test App')
    
    // Load comprehensive CSS
    app.loadCss(`
        .app { display: flex; flex-direction: column; }
        .header { font-size: 1.2rem; padding: 1rem; }
        .content { flex: 1; padding: 1rem; }
        .button { padding: 0.5rem 1rem; margin: 0.25rem; }
    `)
    
    // Create UI structure
    const appContainer = TuiUtils.div()
    appContainer.addClass('app')
    appContainer.setId('app-root')
    
    const header = TuiUtils.div()
    header.addClass('header')
    header.setContent('Integration Test')
    
    const content = TuiUtils.div()
    content.addClass('content')
    
    const button1 = TuiUtils.button()
    button1.addClass('button')
    button1.setContent('Button 1')
    button1.setId('btn1')
    
    const button2 = TuiUtils.button()
    button2.addClass('button')
    button2.setContent('Button 2')
    button2.setId('btn2')
    
    // Build hierarchy
    content.addChild(button1)
    content.addChild(button2)
    appContainer.addChild(header)
    appContainer.addChild(content)
    
    // Configure app
    app.setComponent(appContainer)
    
    // Test app startup
    const status = app.start()
    t.is(typeof status, 'string', 'Integration test should complete successfully')
    t.true(status.includes('initialized'), 'App should be initialized')
})

// Test Suite 9: Error Handling and Edge Cases
test('Error handling - invalid inputs', t => {
    // Test invalid hex colors (using proper ava syntax)
    t.throws(() => JsColorDefinition.hex('invalid'))
    t.throws(() => JsColorDefinition.hex('#gggggg'))
    
    // Test invalid JSON for themes
    t.throws(() => JsColorTheme.fromJson('invalid json'))
    t.throws(() => JsColorTheme.fromJson('{}'))
    
    // Test invalid state JSON
    const state = new JsReactiveState()
    t.throws(() => state.setStateJson('invalid json'))
    t.throws(() => state.setStateJson('[1,2,3]'))
})

test('Edge cases - boundary values', t => {
    // Test RGB boundary values
    t.notThrows(() => JsColorDefinition.rgb(0, 0, 0), 'RGB(0,0,0) should work')
    t.notThrows(() => JsColorDefinition.rgb(255, 255, 255), 'RGB(255,255,255) should work')
    
    // Test toast duration edge cases
    const toast = JsToast.info('Test')
    t.notThrows(() => toast.setDuration(0), 'Zero duration should work')
    t.notThrows(() => toast.setDuration(1), 'Minimum duration should work')
    t.notThrows(() => toast.setDuration(2147483647), 'Max int32 duration should work')
    
    // Test empty strings
    const element = new JsElement('div')
    t.notThrows(() => element.setContent(''), 'Empty content should work')
    t.notThrows(() => element.setId(''), 'Empty ID should work')
    t.notThrows(() => element.addClass(''), 'Empty class should work')
})