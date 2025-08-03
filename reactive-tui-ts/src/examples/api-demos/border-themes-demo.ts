/**
 * Border Themes Demo
 * 
 * Showcases all available Unicode border themes with the widget system.
 * Demonstrates how to use different border styles for various UI elements.
 */

import { 
    div, panel, 
    getBorderTheme, getThemeNames, createThemePreview,
    inlineRouter 
} from '../../packages/tui-bun/src/index';

// Create inline router for theme browsing
const router = inlineRouter({
    onNavigate: (event) => {
        if (event.from) {
            console.log(`   → Switched to "${event.to}" theme`);
        }
    }
});

// Get all available theme names
const themeNames = getThemeNames();

// Register route for each theme
themeNames.forEach(themeName => {
    const theme = getBorderTheme(themeName);
    router.route(`/${themeName}`, {
        title: theme.name,
        description: theme.description,
        component: () => createThemeDemo(themeName)
    });
});

function createThemeDemo(themeName: string): any {
    const theme = getBorderTheme(themeName);
    
    return div({ class: 'grid grid-cols-2 gap-2', id: `theme-${themeName}` }).child(
            panel({
                id: 'theme-info',
                content: `Name: ${theme.name}\nDescription: ${theme.description}\nWeight: ${theme.weight}\nStyle: ${theme.style}`,
                borderTheme: themeName
            }).build()
        )
        .child(
            panel({
                id: 'sample-panel',
                content: `This panel uses the\n"${theme.name}" border theme.\n\nCharacters:\n${theme.chars.topLeft}${theme.chars.horizontal}${theme.chars.topRight}\n${theme.chars.vertical} ${theme.chars.vertical}\n${theme.chars.bottomLeft}${theme.chars.horizontal}${theme.chars.bottomRight}`,
                borderTheme: themeName
            }).build()
        )
        .child(
            panel({
                id: 'grid-demo',
                content: 'Multiple panels\nwith consistent\nborder theme',
                borderTheme: themeName
            }).build()
        )
        .child(
            panel({
                id: 'content-demo',
                content: `Unicode borders provide:\n• Professional appearance\n• Cross-platform compatibility\n• Various visual weights\n• Different decorative styles`,
                borderTheme: themeName
            }).build()
        );
}

// Theme comparison view
router.route('/comparison', {
    description: 'Side-by-side comparison of all themes',
    component: () => createComparisonDemo()
});

function createComparisonDemo(): any {
    const lightThemes = ['ascii', 'light', 'rounded', 'dashed-light', 'dotted'];
    const heavyThemes = ['heavy', 'double', 'dashed-heavy', 'block-light', 'block-solid'];
    
    return div({ class: 'grid grid-cols-2 gap-3', id: 'comparison-demo' }).child(
            div({ class: 'theme-group' })
                .child(
                    panel({
                        id: 'light-themes-header',
                        content: 'Subtle, minimal borders',
                        borderTheme: 'light'
                    }).build()
                )
                .children(
                    lightThemes.map((themeName, index) => 
                        panel({
                            id: `light-${index}`,
                            title: themeName,
                            content: `${getBorderTheme(themeName).description}`,
                            borderTheme: themeName
                        }).build()
                    )
                )
        )
        .child(
            div({ class: 'theme-group' })
                .child(
                    panel({
                        id: 'heavy-themes-header',
                        content: 'Bold, prominent borders',
                        borderTheme: 'heavy'
                    }).build()
                )
                .children(
                    heavyThemes.map((themeName, index) => 
                        panel({
                            id: `heavy-${index}`,
                            title: themeName,
                            content: `${getBorderTheme(themeName).description}`,
                            borderTheme: themeName
                        }).build()
                    )
                )
        );
}

// Navigation state
let currentThemeIndex = 0;
const allRoutes = [...themeNames.map(name => `/${name}`), '/comparison'];

function setupNavigation() {
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');
    
    process.stdin.on('data', (key: string) => {
        switch (key) {
            case '\u0003': // Ctrl+C
            case 'q':
            case 'Q':
                console.log('\nBorder themes demo ended');
                process.exit(0);
                break;
                
            case '\u001b[C': // Right arrow
            case ' ': // Space
            case '\r': // Enter
                currentThemeIndex = (currentThemeIndex + 1) % allRoutes.length;
                router.navigate(allRoutes[currentThemeIndex]);
                renderDemo();
                break;
                
            case '\u001b[D': // Left arrow
                currentThemeIndex = (currentThemeIndex - 1 + allRoutes.length) % allRoutes.length;
                router.navigate(allRoutes[currentThemeIndex]);
                renderDemo();
                break;
                
            case 'c':
            case 'C':
                // Jump to comparison view
                currentThemeIndex = allRoutes.length - 1;
                router.navigate('/comparison');
                renderDemo();
                break;
                
            case 'p':
            case 'P':
                // Show preview of all themes
                showThemePreview();
                break;
                
            case 'h':
            case 'H':
                showHelp();
                break;
        }
    });
}

function renderDemo() {
    const component = router.getCurrentComponent();
    const route = router.getCurrentRoute();
    
    if (component && route) {
        console.log(`\n${route.title} - ${route.description}`);
        console.log('─'.repeat(60));
        
        // Simplified rendering for demo
        renderComponentStructure(component);
        
        console.log('─'.repeat(60));
        console.log('Navigation: ← → arrows | C comparison | P preview | H help | Q quit');
    }
}

function renderComponentStructure(element: any, indent: string = '') {
    const title = element.attributes?.['data-title'] || element.tag;
    const theme = element.attributes?.['data-border-theme'];
    const themeInfo = theme ? ` (${theme})` : '';
    
    console.log(`${indent}${title}${themeInfo}`);
    
    if (element.content) {
        element.content.split('\n').forEach((line: string, index: number) => {
            if (index < 3) { // Limit lines for demo
                console.log(`${indent}  ${line}`);
            }
        });
    }
    
    if (element.children) {
        element.children.forEach((child: any) => {
            renderComponentStructure(child, indent + '  ');
        });
    }
}

function showThemePreview() {
    console.log('\nAll Available Border Themes');
    console.log('===========================');
    console.log(createThemePreview());
    console.log('Press any key to continue...');
    
    process.stdin.once('data', () => {
        renderDemo();
    });
}

function showHelp() {
    console.log('\nBorder Themes Demo Help');
    console.log('======================');
    console.log('← →    Navigate between themes');
    console.log('Space  Next theme');
    console.log('Enter  Next theme');
    console.log('C      Show comparison view');
    console.log('P      Show theme preview');
    console.log('H      Show this help');
    console.log('Q      Quit demo');
    console.log('');
    console.log('Available themes:');
    themeNames.forEach((name, index) => {
        const marker = index === currentThemeIndex ? '→' : ' ';
        const theme = getBorderTheme(name);
        console.log(`${marker} ${index + 1}. ${name} - ${theme.description}`);
    });
    console.log('');
    console.log('Press any key to continue...');
    
    process.stdin.once('data', () => {
        renderDemo();
    });
}

// Startup
console.log('Border Themes Demo');
console.log('==================');
console.log('');
console.log('Explore all available Unicode border themes for TUI widgets.');
console.log('Each theme provides different visual styles and weights.');
console.log('');
console.log(`Available themes: ${themeNames.length}`);
console.log('• ASCII compatible themes');
console.log('• Light, medium, and heavy weights');
console.log('• Solid, dashed, dotted, and block styles');
console.log('• Modern rounded corners');
console.log('');

// Setup and start
setupNavigation();
router.navigate(`/${themeNames[0]}`).then(() => {
    renderDemo();
});