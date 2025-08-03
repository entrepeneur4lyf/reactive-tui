/**
 * Inline Grid Demo - Renders in place without clearing screen
 * 
 * This demonstrates the inline router mode where the demo renders
 * without taking over the full terminal, perfect for embedding
 * in other applications or running alongside other terminal output.
 */

import { div, panel, progress, inlineRouter } from '../../packages/tui-bun/src/index';

// Create inline router (doesn't clear screen)
const router = inlineRouter({
    onNavigate: (event) => {
        if (event.from) {
            console.log(`   → Switched from "${event.from}" to "${event.to}"`);
        }
    }
});

// Register inline demo routes

// Simple keyboard handler for inline mode
let currentRouteIndex = 0;
const routes = ['/grid', '/widgets', '/responsive'];

function setupInlineNavigation() {
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');
    
    process.stdin.on('data', (key: string) => {
        switch (key) {
            case '\u0003': // Ctrl+C
            case 'q':
            case 'Q':
                console.log('\nInline demo ended');
                process.exit(0);
                break;
                
            case '\u001b[C': // Right arrow
            case ' ': // Space
            case '\r': // Enter
                currentRouteIndex = (currentRouteIndex + 1) % routes.length;
                router.navigate(routes[currentRouteIndex]);
                renderInline();
                break;
                
            case '\u001b[D': // Left arrow
                currentRouteIndex = (currentRouteIndex - 1 + routes.length) % routes.length;
                router.navigate(routes[currentRouteIndex]);
                renderInline();
                break;
                
            case 'h':
            case 'H':
                showInlineHelp();
                break;
        }
    });
}

function renderInline() {
    const component = router.getCurrentComponent();
    const route = router.getCurrentRoute();
    
    if (component && route) {
        console.log(`\n${route.title} - ${route.description}`);
        console.log('─'.repeat(50));
        
        // Render component inline (simplified)
        renderComponentInline(component);
        
        console.log('─'.repeat(50));
        console.log('Navigation: ← → arrows | H help | Q quit');
    }
}

function renderComponentInline(element: any) {
    // Simplified inline rendering - just show the structure
    console.log(`   ${element.tag}${element.class ? ` (${element.class})` : ''}`);
    
    if (element.content) {
        element.content.split('\n').forEach((line: string) => {
            console.log(`   │ ${line}`);
        });
    }
    
    if (element.children) {
        element.children.forEach((child: any, index: number) => {
            const isLast = index === element.children.length - 1;
            const prefix = isLast ? '   └─ ' : '   ├─ ';
            console.log(`${prefix}${child.tag}${child.title ? ` "${child.title}"` : ''}`);
            
            if (child.content) {
                child.content.split('\n').forEach((line: string, lineIndex: number) => {
                    if (lineIndex < 2) { // Limit lines for inline display
                        const linePrefix = isLast ? '      ' : '   │  ';
                        console.log(`${linePrefix}${line}`);
                    }
                });
            }
        });
    }
}

function showInlineHelp() {
    console.log('\nInline Demo Help');
    console.log('================');
    console.log('← →    Navigate between demos');
    console.log('Space  Next demo');
    console.log('Enter  Next demo');
    console.log('H      Show this help');
    console.log('Q      Quit demo');
    console.log('');
    console.log('Available demos:');
    routes.forEach((path, index) => {
        const route = router.getRoute(path);
        const marker = index === currentRouteIndex ? '→' : ' ';
        console.log(`${marker} ${index + 1}. ${route?.title} - ${route?.description}`);
    });
    console.log('');
}

// Startup
console.log('Inline Grid/Layout Demo');
console.log('=======================');
console.log('');
console.log('This demo runs inline without clearing your terminal.');
console.log('Perfect for embedding in CLI tools or running alongside other output.');
console.log('');
console.log('Features:');
console.log('• Inline rendering (no screen takeover)');
console.log('• Responsive layouts');
console.log('• Widget integration');
console.log('• Embeddable design');
console.log('');

// Setup and start
setupInlineNavigation();
router.navigate('/grid').then(() => {
    renderInline();
});