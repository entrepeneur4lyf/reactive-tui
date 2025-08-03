/**
 * JSON Theme Loading Demo
 * 
 * Demonstrates how to load themes from JSON files and use them with the TUI framework.
 * Shows various color formats (hex, rgb, ansi) and theme features like inheritance and named colors.
 */

import { resolve } from 'path';
import { 
    div, panel,
    loadThemeFromFile, loadThemeCollectionFromFile, 
    registerTheme, getRegisteredThemeNames, getThemePreview,
    themeToAnsiCodes, getColorThemeEnhanced,
    inlineRouter
} from '../../packages/tui-bun/src/index';

// Theme directory path
const THEMES_DIR = resolve(__dirname, '../themes');

// Load JSON themes on startup
async function loadJSONThemes() {
    console.log('Loading JSON themes...');
    
    try {
        // Load individual theme files
        const draculaTheme = loadThemeFromFile(resolve(THEMES_DIR, 'dracula.json'));
        const gruvboxTheme = loadThemeFromFile(resolve(THEMES_DIR, 'gruvbox-dark.json'));
        const monokaiTheme = loadThemeFromFile(resolve(THEMES_DIR, 'monokai.json'));
        
        // Register themes
        registerTheme(draculaTheme);
        registerTheme(gruvboxTheme);
        registerTheme(monokaiTheme);
        
        console.log(`✓ Loaded individual themes: ${draculaTheme.name}, ${gruvboxTheme.name}, ${monokaiTheme.name}`);
        
        // Load theme collection
        const collectionThemes = loadThemeCollectionFromFile(resolve(THEMES_DIR, 'collection.json'));
        
        // Register collection themes
        for (const theme of collectionThemes) {
            registerTheme(theme);
        }
        
        console.log(`✓ Loaded collection themes: ${collectionThemes.map(t => t.name).join(', ')}`);
        
        return [draculaTheme, gruvboxTheme, monokaiTheme, ...collectionThemes];
        
    } catch (error) {
        console.error('Failed to load JSON themes:', error);
        return [];
    }
}

// Create router for theme browsing
const router = inlineRouter({
    onNavigate: (event) => {
        if (event.from) {
            console.log(`   → Switched to "${event.to}" theme`);
        }
    }
});

// Initialize demo
async function initDemo() {
    const loadedThemes = await loadJSONThemes();
    
    if (loadedThemes.length === 0) {
        console.log('No themes loaded. Please check theme files.');
        process.exit(1);
    }
    
    // Get all registered theme names (built-in + JSON loaded)
    const allThemeNames = getRegisteredThemeNames();
    
    console.log(`\nAvailable themes: ${allThemeNames.join(', ')}\n`);
    
    // Register routes for each theme
    allThemeNames.forEach(themeName => {
        router.route(`/${themeName}`, {
            title: themeName,
            description: `Demo of ${themeName} theme`,
            component: () => createThemeDemo(themeName)
        });
    });
    
    // Special route for theme comparison
    router.route('/comparison', {
        description: 'Side-by-side comparison of JSON themes',
        component: () => createComparisonDemo(loadedThemes)
    });
    
    // Special route for ANSI codes demo
    router.route('/ansi-codes', {
        description: 'Generated ANSI escape codes for themes',
        component: () => createAnsiCodesDemo()
    });
    
    return allThemeNames;
}

function createThemeDemo(themeName: string): any {
    const theme = getColorThemeEnhanced(themeName);
    const ansiCodes = themeToAnsiCodes(theme);
    
    return div({ class: 'json-theme-demo', id: `theme-${themeName}` }).child(
            // Theme header with live preview
            panel({
                id: 'theme-header',
                title: `${theme.name} Theme`,
                content: `${theme.description}\\nMode: ${theme.mode}\\nSource: ${themeName.includes('-') ? 'JSON file' : 'Built-in'}`,
                colorTheme: themeName,
                borderTheme: 'rounded',
                borderStyle: 'fancy'
            }).build()
        )
        .child(
            // Color swatches
            panel({
                id: 'color-swatches',
                content: generateColorSwatches(theme, ansiCodes),
                colorTheme: themeName,
                borderTheme: 'light',
                borderStyle: 'shadow'
            }).build()
        )
        .child(
            // Widget examples
            panel({
                id: 'widget-examples',
                content: 'Button: [Press Me]\\nInput: [Type here...]\\nProgress: ████████░░ 80%\\nStatus: ✓ Success ⚠ Warning ✗ Error',
                colorTheme: themeName,
                borderTheme: 'heavy',
                borderStyle: 'unicode'
            }).build()
        )
        .child(
            // Technical details
            panel({
                id: 'technical-details',
                content: `Semantic mappings:\\n• Panel BG: ${theme.semantic.panelBackground}\\n• Border: ${theme.semantic.panelBorder}\\n• Text: ${theme.semantic.panelTitle}\\n• Button: ${theme.semantic.buttonBackground}`,
                colorTheme: themeName,
                borderTheme: 'dotted',
                borderStyle: 'minimal'
            }).build()
        );
}

function generateColorSwatches(theme: any, ansiCodes: Record<string, string>): string {
    const swatches: string[] = [];
    const colors = ['primary', 'secondary', 'success', 'warning', 'error', 'info'];
    
    for (const color of colors) {
        const rgb = theme.palette[color];
        if (rgb && ansiCodes[color]) {
            const colorBlock = ansiCodes[`${color}_bg`] + '  ' + ansiCodes.reset;
            swatches.push(`${colorBlock} ${color}: rgb(${rgb.r},${rgb.g},${rgb.b})`);
        }
    }
    
    return swatches.join('\\n');
}

function createComparisonDemo(themes: any[]): any {
    return div({ class: 'comparison-demo', id: 'theme-comparison' }).children(
            themes.slice(0, 4).map((theme, index) => 
                panel({
                    id: `comparison-${index}`,
                    title: theme.name,
                    content: `${theme.description}\\n\\nMode: ${theme.mode}\\nColors: ${Object.keys(theme.palette).length}`,
                    colorTheme: theme.name,
                    borderTheme: index % 2 === 0 ? 'light' : 'heavy',
                    borderStyle: 'shadow'
                }).build()
            )
        );
}

function createAnsiCodesDemo(): any {
    const currentTheme = getColorThemeEnhanced('dracula'); // Use Dracula as example
    const ansiCodes = themeToAnsiCodes(currentTheme);
    
    // Generate ANSI code examples
    const examples: string[] = [];
    examples.push('ANSI Escape Codes Generated:');
    examples.push('');
    
    const colorKeys = ['primary', 'secondary', 'success', 'warning', 'error'];
    for (const key of colorKeys) {
        if (ansiCodes[key]) {
            const code = ansiCodes[key].replace(/\u001B/g, '\\x1B');
            examples.push(`${key}: ${code}`);
        }
    }
    
    examples.push('');
    examples.push('Live preview:');
    for (const key of colorKeys) {
        if (ansiCodes[key]) {
            examples.push(`${ansiCodes[key]}${key} colored text${ansiCodes.reset}`);
        }
    }
    
    return div({ class: 'ansi-demo', id: 'ansi-codes' }).child(
            panel({
                id: 'ansi-explanation',
                content: examples.join('\\n'),
                colorTheme: 'dracula',
                borderTheme: 'double',
                borderStyle: 'fancy'
            }).build()
        );
}

// Navigation state
let currentRouteIndex = 0;
let allRoutes: string[] = [];

function setupNavigation() {
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');
    
    process.stdin.on('data', (key: string) => {
        switch (key) {
            case '\u0003': // Ctrl+C
            case 'q':
            case 'Q':
                console.log('\\nJSON theme demo ended');
                process.exit(0);
                break;
                
            case '\u001b[C': // Right arrow
            case ' ': // Space
            case '\r': // Enter
                currentRouteIndex = (currentRouteIndex + 1) % allRoutes.length;
                router.navigate(allRoutes[currentRouteIndex]);
                renderDemo();
                break;
                
            case '\u001b[D': // Left arrow
                currentRouteIndex = (currentRouteIndex - 1 + allRoutes.length) % allRoutes.length;
                router.navigate(allRoutes[currentRouteIndex]);
                renderDemo();
                break;
                
            case 'c':
            case 'C':
                // Jump to comparison
                const compIndex = allRoutes.indexOf('/comparison');
                if (compIndex !== -1) {
                    currentRouteIndex = compIndex;
                    router.navigate('/comparison');
                    renderDemo();
                }
                break;
                
            case 'a':
            case 'A':
                // Jump to ANSI codes
                const ansiIndex = allRoutes.indexOf('/ansi-codes');
                if (ansiIndex !== -1) {
                    currentRouteIndex = ansiIndex;
                    router.navigate('/ansi-codes');
                    renderDemo();
                }
                break;
                
            case 'p':
            case 'P':
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
        console.clear();
        console.log('JSON Theme Loading Demo');
        console.log('======================');
        console.log(`Current: ${route.title} - ${route.description}`);
        console.log('');
        
        // Render with actual styling
        renderStyledComponent(component);
        
        console.log('');
        console.log('─'.repeat(80));
        console.log('← → navigate | C comparison | A ansi codes | P preview | H help | Q quit');
        console.log(`Theme ${currentRouteIndex + 1}/${allRoutes.length}`);
    }
}

function renderStyledComponent(element: any, indent: string = '') {
    const title = element.attributes?.['data-title'] || element.tag;
    const colorTheme = element.attributes?.['data-color-theme'];
    
    console.log(`${indent}${title}${colorTheme ? ` (${colorTheme})` : ''}`);
    
    if (element.content && element.style) {
        const lines = element.content.split('\\n');
        lines.slice(0, 4).forEach((line: string) => {
            // Apply theme colors for real preview
            const styledLine = element.style.backgroundColor + element.style.contentColor + line + '\\x1B[0m';
            console.log(`${indent}  ${styledLine}`);
        });
    }
    
    if (element.children) {
        element.children.forEach((child: any) => {
            renderStyledComponent(child, indent + '  ');
        });
    }
}

function showThemePreview() {
    const allThemes = getRegisteredThemeNames();
    console.clear();
    console.log('All Theme Previews');
    console.log('==================');
    console.log('');
    
    for (const themeName of allThemes.slice(0, 3)) { // Show first 3 for space
        const theme = getColorThemeEnhanced(themeName);
        console.log(getThemePreview(theme));
        console.log('');
    }
    
    console.log('Press any key to continue...');
    process.stdin.once('data', () => {
        renderDemo();
    });
}

function showHelp() {
    console.clear();
    console.log('JSON Theme Demo Help');
    console.log('===================');
    console.log('');
    console.log('This demo shows how to:');
    console.log('• Load themes from JSON files');
    console.log('• Use various color formats (hex, rgb, ansi)');
    console.log('• Named color references and theme inheritance');
    console.log('• Generate ANSI escape codes automatically');
    console.log('• Register themes dynamically');
    console.log('');
    console.log('Navigation:');
    console.log('← →  Navigate between themes');
    console.log('C    Theme comparison view');
    console.log('A    ANSI codes demonstration');
    console.log('P    Theme previews');
    console.log('H    This help');
    console.log('Q    Quit demo');
    console.log('');
    console.log('JSON theme files location: themes/');
    console.log('');
    console.log('Press any key to continue...');
    
    process.stdin.once('data', () => {
        renderDemo();
    });
}

// Main execution
async function main() {
    console.log('JSON Theme Loading Demo');
    console.log('=======================');
    console.log('');
    console.log('This demo loads themes from JSON files and demonstrates:');
    console.log('• Multiple color formats (hex, rgb, ansi codes)');
    console.log('• Named color references and theme collections');
    console.log('• Automatic ANSI escape code generation');
    console.log('• Dynamic theme registration and loading');
    console.log('');
    
    try {
        const themeNames = await initDemo();
        
        // Build route list
        allRoutes = themeNames.map(name => `/${name}`);
        allRoutes.push('/comparison', '/ansi-codes');
        
        setupNavigation();
        
        // Start with first theme
        await router.navigate(allRoutes[0]);
        renderDemo();
        
    } catch (error) {
        console.error('Failed to initialize demo:', error);
        process.exit(1);
    }
}

// Run the demo
main().catch(console.error);