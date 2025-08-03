/**
 * Theme Showcase Demo
 * 
 * Demonstrates the complete theming system with border themes, color themes,
 * and custom theme creation. Shows how to mix and match themes for different
 * visual styles and how to create professional looking TUI applications.
 */

import { 
    div, panel, 
    getBorderTheme, getThemeNames as getBorderThemeNames,
    colorThemes, themeBuilder, createCustomTheme,
    rgb, createVariant,
    inlineRouter,
    type ColorTheme
} from '../../packages/tui-bun/src/index';

// Create inline router for theme browsing
const router = inlineRouter({
    onNavigate: (event) => {
        if (event.from) {
            console.log(`   → Switched to "${event.to}" combination`);
        }
    }
});

// Get all available theme names
const _borderThemeNames = getBorderThemeNames();
const _colorThemeNames = Object.keys(colorThemes);

// Create some custom themes for demonstration
const customColorThemes: Record<string, ColorTheme> = {
    ocean: createCustomTheme('ocean', 'Ocean inspired blue theme', {
        primary: rgb(59, 130, 246),
        secondary: rgb(16, 185, 129),
        background: rgb(15, 23, 42),
        surface: rgb(30, 41, 59),
        text: rgb(248, 250, 252),
        border: rgb(71, 85, 105)
    }),
    
    sunset: createCustomTheme('sunset', 'Warm sunset colors', {
        primary: rgb(251, 146, 60),
        secondary: rgb(239, 68, 68),
        background: rgb(41, 37, 36),
        surface: rgb(68, 64, 60),
        text: rgb(254, 243, 199),
        border: rgb(120, 113, 108)
    }),
    
    forest: createCustomTheme('forest', 'Natural forest greens', {
        primary: rgb(34, 197, 94),
        secondary: rgb(132, 204, 22),
        background: rgb(20, 83, 45),
        surface: rgb(22, 101, 52),
        text: rgb(240, 253, 244),
        border: rgb(74, 222, 128)
    })
};

// All available color themes (built-in + custom)
const allColorThemes = { ...colorThemes, ...customColorThemes };

// Create demo combinations
const demoCombinations = [
    { colorTheme: 'dark', borderTheme: 'light', },
    { colorTheme: 'light', borderTheme: 'rounded', },
    { colorTheme: 'terminal', borderTheme: 'heavy', },
    { colorTheme: 'ocean', borderTheme: 'double', },
    { colorTheme: 'sunset', borderTheme: 'dashed-heavy', },
    { colorTheme: 'forest', borderTheme: 'rounded', },
    { colorTheme: 'dark', borderTheme: 'block-light', },
    { colorTheme: 'light', borderTheme: 'dotted', }
];

// Register routes for each combination
demoCombinations.forEach((combo, index) => {
    router.route(`/combo-${index}`, {
        title: combo.title,
        description: `${combo.colorTheme} colors with ${combo.borderTheme} borders`,
        component: () => createThemeComboDemo(combo.colorTheme, combo.borderTheme, combo.title)
    });
});

function createThemeComboDemo(colorThemeName: string, borderThemeName: string, title: string): any {
    const colorTheme = allColorThemes[colorThemeName];
    const borderTheme = getBorderTheme(borderThemeName);
    
    return div({ class: 'theme-showcase', id: `combo-${colorThemeName}-${borderThemeName}` }).child(
            // Header panel with theme info
            panel({
                id: 'theme-header',
                title: `${title} Theme Combination`,
                content: `Color Theme: ${colorTheme.name} - ${colorTheme.description}\\nBorder Theme: ${borderTheme.name} - ${borderTheme.description}\\nMode: ${colorTheme.mode}`,
                colorTheme: colorThemeName,
                borderTheme: borderThemeName,
                borderStyle: 'fancy'
            }).build()
        )
        .child(
            // Color palette display
            panel({
                id: 'color-palette',
                content: `Primary: rgb(${colorTheme.palette.primary.r}, ${colorTheme.palette.primary.g}, ${colorTheme.palette.primary.b})\\nSecondary: rgb(${colorTheme.palette.secondary.r}, ${colorTheme.palette.secondary.g}, ${colorTheme.palette.secondary.b})\\nBackground: rgb(${colorTheme.palette.background.r}, ${colorTheme.palette.background.g}, ${colorTheme.palette.background.b})\\nText: rgb(${colorTheme.palette.text.r}, ${colorTheme.palette.text.g}, ${colorTheme.palette.text.b})`,
                colorTheme: colorThemeName,
                borderTheme: borderThemeName,
                borderStyle: 'shadow'
            }).build()
        )
        .child(
            // Border character preview
            panel({
                id: 'border-preview',
                content: `${borderTheme.chars.topLeft}${borderTheme.chars.horizontal.repeat(8)}${borderTheme.chars.topRight}\\n${borderTheme.chars.vertical}  Sample  ${borderTheme.chars.vertical}\\n${borderTheme.chars.bottomLeft}${borderTheme.chars.horizontal.repeat(8)}${borderTheme.chars.bottomRight}\\n\\nStyle: ${borderTheme.style}\\nWeight: ${borderTheme.weight}`,
                colorTheme: colorThemeName,
                borderTheme: borderThemeName,
                borderStyle: 'unicode'
            }).build()
        )
        .child(
            // Semantic colors demo
            panel({
                id: 'semantic-demo',
                content: 'This panel demonstrates how\\nthe semantic color mappings\\nwork in practice.\\n\\n• Panel backgrounds\\n• Border colors\\n• Text hierarchy\\n• Interactive states',
                colorTheme: colorThemeName,
                borderTheme: borderThemeName,
                borderStyle: 'outlined'
            }).build()
        );
}

// Theme builder demo
router.route('/builder', {
    description: 'Custom theme creation examples',
    component: () => createThemeBuilderDemo()
});

function createThemeBuilderDemo(): any {
    // Create a dynamic theme using ThemeBuilder
    const dynamicTheme = themeBuilder('dark')
        .name('dynamic')
        .description('Dynamically created theme')
        .color('primary', rgb(147, 51, 234))  // Purple
        .color('secondary', rgb(236, 72, 153))  // Pink
        .color('background', createVariant(rgb(17, 24, 39), -0.2))  // Darker
        .semanticMapping('panelBorder', 'primary')
        .build();

    // Add to available themes temporarily
    (allColorThemes as any).dynamic = dynamicTheme;

    return div({ class: 'builder-demo', id: 'theme-builder' }).child(
            panel({
                id: 'builder-explanation',
                content: 'This theme was created using the\\nThemeBuilder class:\\n\\nthemeBuilder("dark")\\n  .name("dynamic")\\n  .color("primary", rgb(147, 51, 234))\\n  .color("secondary", rgb(236, 72, 153))\\n  .build()',
                colorTheme: 'dynamic',
                borderTheme: 'heavy',
                borderStyle: 'fancy'
            }).build()
        )
        .child(
            panel({
                id: 'custom-colors',
                content: `Original: rgb(17, 24, 39)\\nDarker (-20%): rgb(${Math.round(17 * 0.8)}, ${Math.round(24 * 0.8)}, ${Math.round(39 * 0.8)})\\nLighter (+30%): rgb(${Math.round(17 + (255-17) * 0.3)}, ${Math.round(24 + (255-24) * 0.3)}, ${Math.round(39 + (255-39) * 0.3)})`,
                colorTheme: 'dynamic',
                borderTheme: 'rounded',
                borderStyle: 'shadow'
            }).build()
        )
        .child(
            panel({
                id: 'hex-demo',
                content: 'Colors can be defined using:\\n\\n• rgb(r, g, b) - Direct RGB\\n• hex("#FF5733") - Hex strings\\n• createVariant(color, factor) - Variations\\n\\nAll are validated and clamped.',
                colorTheme: 'dynamic',
                borderTheme: 'double',
                borderStyle: 'unicode'
            }).build()
        );
}

// Theme comparison view
router.route('/comparison', {
    description: 'Side-by-side comparison of all theme combinations',
    component: () => createComparisonDemo()
});

function createComparisonDemo(): any {
    const compareColorThemes = ['dark', 'light', 'ocean', 'sunset'];
    const compareBorderThemes = ['light', 'heavy', 'rounded', 'double'];
    
    return div({ class: 'comparison-matrix', id: 'theme-comparison' }).children(
            compareColorThemes.map((colorTheme, colorIndex) => 
                div({ class: 'color-theme-row', id: `row-${colorIndex}` })
                    .child(
                        panel({
                            id: `header-${colorIndex}`,
                            title: colorTheme.toUpperCase(),
                            content: allColorThemes[colorTheme].description,
                            colorTheme: colorTheme,
                            borderTheme: 'ascii',
                            borderStyle: 'minimal'
                        }).build()
                    )
                    .children(
                        compareBorderThemes.map((borderTheme, borderIndex) => 
                            panel({
                                id: `cell-${colorIndex}-${borderIndex}`,
                                title: borderTheme,
                                content: `${colorTheme}\\n+\\n${borderTheme}`,
                                colorTheme: colorTheme,
                                borderTheme: borderTheme,
                                borderStyle: 'shadow'
                            }).build()
                        )
                    )
            )
        );
}

// Navigation state
let currentRouteIndex = 0;
const allRoutes = [
    ...demoCombinations.map((_, index) => `/combo-${index}`),
    '/builder',
    '/comparison'
];

function setupNavigation() {
    process.stdin.setRawMode(true);
    process.stdin.resume();
    process.stdin.setEncoding('utf8');
    
    process.stdin.on('data', (key: string) => {
        switch (key) {
            case '\u0003': // Ctrl+C
            case 'q':
            case 'Q':
                console.log('\nTheme showcase demo ended');
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
                // Jump to comparison view
                const compIndex = allRoutes.indexOf('/comparison');
                if (compIndex !== -1) {
                    currentRouteIndex = compIndex;
                    router.navigate('/comparison');
                    renderDemo();
                }
                break;
                
            case 'b':
            case 'B':
                // Jump to builder demo
                const builderIndex = allRoutes.indexOf('/builder');
                if (builderIndex !== -1) {
                    currentRouteIndex = builderIndex;
                    router.navigate('/builder');
                    renderDemo();
                }
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
        console.log(`╔═══ ${route.title} ═══╗`);
        console.log(`║ ${route.description}`);
        console.log('╚' + '═'.repeat(route.title.length + 8) + '╝');
        console.log('');
        
        // Render component structure with style information
        renderStyledComponent(component);
        
        console.log('');
        console.log('─'.repeat(80));
        console.log('← → arrows | B builder | C comparison | H help | Q quit');
        console.log(`Route ${currentRouteIndex + 1}/${allRoutes.length}`);
    }
}

function renderStyledComponent(element: any, indent: string = '') {
    const title = element.attributes?.['data-title'] || element.tag;
    const colorTheme = element.attributes?.['data-color-theme'];
    const borderTheme = element.attributes?.['data-border-theme'];
    const style = element.style;
    
    // Create visual representation
    let display = `${indent}${title}`;
    if (colorTheme || borderTheme) {
        display += ` (${colorTheme || 'default'}/${borderTheme || 'default'})`;
    }
    
    console.log(display);
    
    if (element.content && style) {
        // Apply actual colors for preview
        const lines = element.content.split('\n');
        lines.slice(0, 3).forEach((line: string) => {
            const styledLine = style.backgroundColor + style.contentColor + line + '\x1B[0m';
            console.log(`${indent}  ${styledLine}`);
        });
        if (lines.length > 3) {
            console.log(`${indent}  ... (${lines.length - 3} more lines)`);
        }
    }
    
    if (element.children) {
        element.children.forEach((child: any) => {
            renderStyledComponent(child, indent + '  ');
        });
    }
}

function showHelp() {
    console.clear();
    console.log('Theme Showcase Demo Help');
    console.log('========================');
    console.log('');
    console.log('Navigation:');
    console.log('← →    Navigate between theme combinations');
    console.log('Space  Next combination');
    console.log('Enter  Next combination');
    console.log('B      Theme builder demo');
    console.log('C      Theme comparison matrix');
    console.log('H      Show this help');
    console.log('Q      Quit demo');
    console.log('');
    console.log('Features Demonstrated:');
    console.log('• Color theme system with semantic mappings');
    console.log('• Border theme system with Unicode characters');
    console.log('• Custom theme creation with ThemeBuilder');
    console.log('• Theme combination and mixing');
    console.log('• Professional styling for TUI applications');
    console.log('');
    console.log(`Current: ${demoCombinations[Math.min(currentRouteIndex, demoCombinations.length - 1)]?.title || 'Special View'}`);
    console.log('');
    console.log('Press any key to continue...');
    
    process.stdin.once('data', () => {
        renderDemo();
    });
}

// Startup
console.log('Theme Showcase Demo');
console.log('===================');
console.log('');
console.log('Complete theming system demonstration for TUI applications.');
console.log('Combines color themes and border themes for professional styling.');
console.log('');
console.log(`Available combinations: ${demoCombinations.length}`);
console.log('• Built-in color themes: dark, light, terminal');
console.log('• Custom color themes: ocean, sunset, forest');
console.log('• Border themes: light, heavy, double, rounded, dashed, dotted, block');
console.log('• Theme builder for custom creation');
console.log('• Semantic color mappings for consistency');
console.log('');

// Setup and start
setupNavigation();
router.navigate('/combo-0').then(() => {
    renderDemo();
});