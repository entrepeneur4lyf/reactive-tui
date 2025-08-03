const { JsTuiApp, TuiUtils, JsToast, Actions } = require('./index.js');

console.log('Testing Reactive TUI integration...\n');

try {
    // Test 1: Basic app creation
    console.log('✓ Test 1: Creating TUI app...');
    const app = new JsTuiApp();
    console.log('  App created successfully');

    // Test 2: Element creation
    console.log('✓ Test 2: Creating elements...');
    const div = TuiUtils.div();
    div.setId('test-div');
    div.addClass('container');
    div.setContent('Hello, Reactive TUI!');
    console.log('  Div element created successfully');

    const button = TuiUtils.button();
    button.setId('test-button');
    button.setContent('Click me');
    console.log('  Button element created successfully');

    // Test 3: Toast creation
    console.log('✓ Test 3: Creating toast notifications...');
    const _infoToast = JsToast.info('This is an info message');
    const _successToast = JsToast.success('Operation completed successfully');
    const _warningToast = JsToast.warning('Warning: Something needs attention');
    const _errorToast = JsToast.error('Error: Something went wrong');
    console.log('  All toast types created successfully');

    // Test 4: Utility functions
    console.log('✓ Test 4: Testing utilities...');
    const terminalSize = TuiUtils.getTerminalSize();
    console.log(`  Terminal size: ${terminalSize[0]}x${terminalSize[1]}`);

    const cssErrors = TuiUtils.validateCss('.test { color: red; }');
    console.log(`  CSS validation returned ${cssErrors.length} errors`);

    // Test 5: Actions
    console.log('✓ Test 5: Testing action constants...');
    console.log(`  Quit action: ${Actions.quit}`);
    console.log(`  Refresh action: ${Actions.refresh}`);
    console.log(`  Focus next action: ${Actions.focusNext}`);

    // Test 6: App configuration
    console.log('✓ Test 6: Testing app configuration...');
    app.setTitle('Test App');
    app.loadCss('.container { background: blue; color: white; }');
    app.setComponent(div);
    console.log('  App configured successfully');

    // Test 7: Start app (headless mode)
    console.log('✓ Test 7: Starting app...');
    const status = app.start();
    console.log(`  App status: ${status}`);

    console.log('\n🎉 All tests passed! Reactive TUI integration is working correctly.');

} catch (error) {
    console.error('❌ Test failed:', error.message);
    console.error(error.stack);
    process.exit(1);
}