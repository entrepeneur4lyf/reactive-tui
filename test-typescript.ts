import { JsTuiApp, JsToast, TuiUtils } from './index.js';

console.log('Testing TypeScript integration...\n');

// Test TypeScript type checking and IntelliSense
try {
    // Test 1: App creation with proper typing
    console.log('✓ Test 1: TypeScript app creation...');
    const app = new JsTuiApp();
    console.log('  TUI app created with TypeScript types');

    // Test 2: Element creation with type safety
    console.log('✓ Test 2: TypeScript element creation...');
    const container = TuiUtils.div();
    container.setId('container');
    container.addClass('main-container');
    container.setContent('TypeScript TUI App');

    const button = TuiUtils.button();
    button.setId('submit-btn');
    button.setContent('Submit');
    button.makeFocusable(0);

    // Test element hierarchy
    container.addChild(button);

    // Test 3: Toast creation with type safety
    console.log('✓ Test 3: TypeScript toast creation...');
    const successToast = JsToast.success('TypeScript integration successful!');
    successToast.setTitle('Success');
    successToast.setDuration(3000);

    // Test 4: Utilities with proper return types
    console.log('✓ Test 4: TypeScript utility functions...');
    const [width, height] = TuiUtils.getTerminalSize();
    console.log(`  Terminal size (typed): ${width}x${height}`);

    const cssErrors: string[] = TuiUtils.validateCss(`
        .container {
            background: #1e1e1e;
            color: #ffffff;
            padding: 1rem;
        }
        .button {
            background: #007acc;
            border: none;
            padding: 0.5rem 1rem;
        }
    `);
    console.log(`  CSS validation (typed): ${cssErrors.length} errors`);

    // Test 5: App configuration with TypeScript
    console.log('✓ Test 5: TypeScript app configuration...');
    app.setTitle('TypeScript TUI Demo');
    app.loadCss(`
        .main-container {
            display: flex;
            flex-direction: column;
            align-items: center;
            padding: 2rem;
        }
    `);
    app.setComponent(container);

    // Test 6: Start app and verify return type
    console.log('✓ Test 6: TypeScript app execution...');
    const status: string = app.start();
    console.log(`  App status (typed): ${status}`);

    console.log('\n🎉 TypeScript integration test passed!');
    console.log('✅ All types are properly defined and working');

} catch (error) {
    console.error('❌ TypeScript test failed:', error);
    if (error instanceof Error) {
        console.error(error.message);
        console.error(error.stack);
    }
    process.exit(1);
}