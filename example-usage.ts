/**
 * Example TypeScript usage of Reactive TUI
 * 
 * This demonstrates how to use the reactive-tui package in a TypeScript project
 * with full type safety and IntelliSense support.
 */

import { 
    JsTuiApp, 
    JsToast, 
    JsToastManager,
    JsColorTheme,
    TuiUtils
} from './index.js';

async function createTuiApplication(): Promise<void> {
    // Initialize the TUI application
    const app = new JsTuiApp();
    app.setTitle('My TypeScript TUI App');

    // Load CSS styling with type safety
    app.loadCss(`
        .app-container {
            background: #1e1e1e;
            color: #ffffff;
            padding: 2rem;
            display: flex;
            flex-direction: column;
            gap: 1rem;
        }
        
        .header {
            font-size: 1.5rem;
            font-weight: bold;
            text-align: center;
            border-bottom: 1px solid #333;
            padding-bottom: 1rem;
        }
        
        .button-group {
            display: flex;
            gap: 1rem;
            justify-content: center;
        }
        
        .btn {
            background: #007acc;
            color: white;
            padding: 0.5rem 1rem;
            border: 1px solid #005a9e;
            border-radius: 4px;
        }
        
        .btn:hover {
            background: #005a9e;
        }
    `);

    // Create UI elements with full TypeScript support
    const container = TuiUtils.div();
    container.setId('app-container');
    container.addClass('app-container');

    // Header
    const header = TuiUtils.div();
    header.addClass('header');
    header.setContent('🚀 Reactive TUI TypeScript Demo');

    // Button group
    const buttonGroup = TuiUtils.div();
    buttonGroup.addClass('button-group');

    // Create buttons with proper typing
    const buttons = [
        { id: 'save', text: '💾 Save', action: 'save' },
        { id: 'copy', text: '📋 Copy', action: 'copy' },
        { id: 'refresh', text: '🔄 Refresh', action: 'refresh' },
        { id: 'quit', text: '❌ Quit', action: 'quit' }
    ];

    buttons.forEach(({ id, text, action }) => {
        const button = TuiUtils.button();
        button.setId(id);
        button.addClass('btn');
        button.setContent(text);
        button.setAttribute('data-action', action);
        button.makeFocusable();
        buttonGroup.addChild(button);
    });

    // Assemble the UI
    container.addChild(header);
    container.addChild(buttonGroup);

    // Set up toast notifications with type safety
    const [termWidth, termHeight] = TuiUtils.getTerminalSize();
    const toastManager = new JsToastManager(termWidth, termHeight);

    // Demo different toast types
    const demoToasts = [
        JsToast.info('Application initialized successfully'),
        JsToast.success('TypeScript integration is working!'),
        JsToast.warning('This is a demo application'),
    ];

    demoToasts.forEach(toast => {
        toast.setDuration(5000);
        toastManager.showToast(toast);
    });

    // Set the root component and start the app
    app.setComponent(container);
    
    // Get application status with proper typing
    const status: string = app.start();
    console.log(`Application status: ${status}`);

    // Demonstrate theme usage
    const darkTheme = JsColorTheme.dark();
    const lightTheme = JsColorTheme.light();
    
    console.log(`\nTheme Demo:`);
    console.log(`Dark theme: ${darkTheme.getName()}`);
    console.log(`Light theme: ${lightTheme.getName()}`);

    // Demonstrate utility functions with proper return types
    const cssErrors: string[] = TuiUtils.validateCss('.test { color: red; }');
    console.log(`CSS validation: ${cssErrors.length} errors found`);

    console.log('\n✨ TypeScript TUI application created successfully!');
    console.log('🔧 Full type safety and IntelliSense support enabled');
}

// Export for use in other modules
export { createTuiApplication };

// Run the demo if this file is executed directly
if (require.main === module) {
    createTuiApplication().catch(console.error);
}