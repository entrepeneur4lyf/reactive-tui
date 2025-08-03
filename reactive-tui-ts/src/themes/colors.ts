/**
 * Color Theme System
 * 
 * Provides a comprehensive color theming system with predefined themes
 * and interfaces for creating custom themes. Supports both RGB and
 * ANSI color modes for maximum terminal compatibility.
 */

export interface ColorDefinition {
    r: number;
    g: number;
    b: number;
}

export interface ColorPalette {
    // Primary colors
    primary: ColorDefinition;
    primaryDark: ColorDefinition;
    primaryLight: ColorDefinition;
    
    // Secondary colors
    secondary: ColorDefinition;
    secondaryDark: ColorDefinition;
    secondaryLight: ColorDefinition;
    
    // Neutral colors
    background: ColorDefinition;
    backgroundAlt: ColorDefinition;
    surface: ColorDefinition;
    surfaceAlt: ColorDefinition;
    
    // Text colors
    text: ColorDefinition;
    textSecondary: ColorDefinition;
    textMuted: ColorDefinition;
    textInverse: ColorDefinition;
    
    // Border colors
    border: ColorDefinition;
    borderFocus: ColorDefinition;
    borderHover: ColorDefinition;
    
    // Status colors
    success: ColorDefinition;
    warning: ColorDefinition;
    error: ColorDefinition;
    info: ColorDefinition;
    
    // Interactive colors
    hover: ColorDefinition;
    active: ColorDefinition;
    disabled: ColorDefinition;
    
    // Shadow colors
    shadow: ColorDefinition;
    shadowLight: ColorDefinition;
}

export type ColorMode = 'rgb' | 'ansi' | 'auto';

export interface ColorTheme {
    name: string;
    description: string;
    palette: ColorPalette;
    mode: ColorMode;
    
    // Semantic color mappings
    semantic: {
        panelBackground: keyof ColorPalette;
        panelBorder: keyof ColorPalette;
        panelTitle: keyof ColorPalette;
        panelContent: keyof ColorPalette;
        panelShadow: keyof ColorPalette;
        
        buttonBackground: keyof ColorPalette;
        buttonBorder: keyof ColorPalette;
        buttonText: keyof ColorPalette;
        buttonHover: keyof ColorPalette;
        
        inputBackground: keyof ColorPalette;
        inputBorder: keyof ColorPalette;
        inputText: keyof ColorPalette;
        inputFocus: keyof ColorPalette;
        
        progressBackground: keyof ColorPalette;
        progressFill: keyof ColorPalette;
        progressText: keyof ColorPalette;
    };
}

// Helper function to create color definitions
export function rgb(r: number, g: number, b: number): ColorDefinition {
    return { r: Math.max(0, Math.min(255, r)), g: Math.max(0, Math.min(255, g)), b: Math.max(0, Math.min(255, b)) };
}

// Convert hex to RGB
export function hex(hexColor: string): ColorDefinition {
    const hex = hexColor.replace('#', '');
    const r = parseInt(hex.substring(0, 2), 16);
    const g = parseInt(hex.substring(2, 4), 16);
    const b = parseInt(hex.substring(4, 6), 16);
    return rgb(r, g, b);
}

// Predefined color palettes
const darkPalette: ColorPalette = {
    primary: rgb(99, 102, 241),      // Indigo
    primaryDark: rgb(79, 70, 229),   // Dark indigo
    primaryLight: rgb(129, 140, 248), // Light indigo
    
    secondary: rgb(16, 185, 129),    // Emerald
    secondaryDark: rgb(5, 150, 105), // Dark emerald
    secondaryLight: rgb(52, 211, 153), // Light emerald
    
    background: rgb(17, 24, 39),     // Dark slate
    backgroundAlt: rgb(31, 41, 55),  // Slate
    surface: rgb(55, 65, 81),        // Light slate
    surfaceAlt: rgb(75, 85, 99),     // Lighter slate
    
    text: rgb(249, 250, 251),        // White
    textSecondary: rgb(209, 213, 219), // Light gray
    textMuted: rgb(156, 163, 175),   // Gray
    textInverse: rgb(17, 24, 39),    // Dark for light backgrounds
    
    border: rgb(75, 85, 99),         // Light slate
    borderFocus: rgb(99, 102, 241),  // Primary
    borderHover: rgb(107, 114, 128), // Lighter slate
    
    success: rgb(34, 197, 94),       // Green
    warning: rgb(251, 191, 36),      // Amber
    error: rgb(239, 68, 68),         // Red
    info: rgb(59, 130, 246),         // Blue
    
    hover: rgb(67, 56, 202),         // Dark primary
    active: rgb(55, 48, 163),        // Darker primary
    disabled: rgb(107, 114, 128),    // Gray
    
    shadow: rgb(0, 0, 0),            // Black
    shadowLight: rgb(31, 41, 55),    // Dark slate
};

const lightPalette: ColorPalette = {
    primary: rgb(99, 102, 241),      // Indigo
    primaryDark: rgb(79, 70, 229),   // Dark indigo
    primaryLight: rgb(165, 180, 252), // Very light indigo
    
    secondary: rgb(16, 185, 129),    // Emerald
    secondaryDark: rgb(5, 150, 105), // Dark emerald
    secondaryLight: rgb(110, 231, 183), // Very light emerald
    
    background: rgb(255, 255, 255),  // White
    backgroundAlt: rgb(249, 250, 251), // Very light gray
    surface: rgb(243, 244, 246),     // Light gray
    surfaceAlt: rgb(229, 231, 235),  // Gray
    
    text: rgb(17, 24, 39),           // Dark slate
    textSecondary: rgb(55, 65, 81),  // Slate
    textMuted: rgb(107, 114, 128),   // Light slate
    textInverse: rgb(249, 250, 251), // Light for dark backgrounds
    
    border: rgb(209, 213, 219),      // Light gray
    borderFocus: rgb(99, 102, 241),  // Primary
    borderHover: rgb(156, 163, 175), // Gray
    
    success: rgb(34, 197, 94),       // Green
    warning: rgb(245, 158, 11),      // Amber
    error: rgb(239, 68, 68),         // Red
    info: rgb(59, 130, 246),         // Blue
    
    hover: rgb(129, 140, 248),       // Light primary
    active: rgb(109, 40, 217),       // Purple
    disabled: rgb(156, 163, 175),    // Gray
    
    shadow: rgb(0, 0, 0),            // Black
    shadowLight: rgb(107, 114, 128), // Gray
};

const terminalPalette: ColorPalette = {
    primary: rgb(0, 255, 0),         // Bright green
    primaryDark: rgb(0, 128, 0),     // Green
    primaryLight: rgb(144, 238, 144), // Light green
    
    secondary: rgb(255, 255, 0),     // Yellow
    secondaryDark: rgb(255, 165, 0), // Orange
    secondaryLight: rgb(255, 255, 224), // Light yellow
    
    background: rgb(0, 0, 0),        // Black
    backgroundAlt: rgb(32, 32, 32),  // Dark gray
    surface: rgb(64, 64, 64),        // Gray
    surfaceAlt: rgb(96, 96, 96),     // Light gray
    
    text: rgb(255, 255, 255),        // White
    textSecondary: rgb(192, 192, 192), // Light gray
    textMuted: rgb(128, 128, 128),   // Gray
    textInverse: rgb(0, 0, 0),       // Black
    
    border: rgb(128, 128, 128),      // Gray
    borderFocus: rgb(0, 255, 0),     // Bright green
    borderHover: rgb(192, 192, 192), // Light gray
    
    success: rgb(0, 255, 0),         // Bright green
    warning: rgb(255, 255, 0),       // Yellow
    error: rgb(255, 0, 0),           // Red
    info: rgb(0, 255, 255),          // Cyan
    
    hover: rgb(0, 128, 0),           // Green
    active: rgb(0, 64, 0),           // Dark green
    disabled: rgb(64, 64, 64),       // Dark gray
    
    shadow: rgb(0, 0, 0),            // Black
    shadowLight: rgb(32, 32, 32),    // Dark gray
};

// Predefined themes
export const colorThemes: Record<string, ColorTheme> = {
    dark: {
        name: 'dark',
        description: 'Modern dark theme with professional colors',
        palette: darkPalette,
        mode: 'rgb',
        semantic: {
            panelBackground: 'surface',
            panelBorder: 'border',
            panelTitle: 'text',
            panelContent: 'textSecondary',
            panelShadow: 'shadow',
            
            buttonBackground: 'primary',
            buttonBorder: 'primaryDark',
            buttonText: 'textInverse',
            buttonHover: 'hover',
            
            inputBackground: 'backgroundAlt',
            inputBorder: 'border',
            inputText: 'text',
            inputFocus: 'borderFocus',
            
            progressBackground: 'surface',
            progressFill: 'primary',
            progressText: 'text',
        }
    },
    
    light: {
        name: 'light',
        description: 'Clean light theme for bright environments',
        palette: lightPalette,
        mode: 'rgb',
        semantic: {
            panelBackground: 'surface',
            panelBorder: 'border',
            panelTitle: 'text',
            panelContent: 'textSecondary',
            panelShadow: 'shadowLight',
            
            buttonBackground: 'primary',
            buttonBorder: 'primaryDark',
            buttonText: 'textInverse',
            buttonHover: 'hover',
            
            inputBackground: 'background',
            inputBorder: 'border',
            inputText: 'text',
            inputFocus: 'borderFocus',
            
            progressBackground: 'surface',
            progressFill: 'primary',
            progressText: 'text',
        }
    },
    
    terminal: {
        name: 'terminal',
        description: 'Classic terminal colors for retro feel',
        palette: terminalPalette,
        mode: 'ansi',
        semantic: {
            panelBackground: 'surface',
            panelBorder: 'border',
            panelTitle: 'text',
            panelContent: 'textSecondary',
            panelShadow: 'shadow',
            
            buttonBackground: 'primary',
            buttonBorder: 'primaryDark',
            buttonText: 'textInverse',
            buttonHover: 'hover',
            
            inputBackground: 'backgroundAlt',
            inputBorder: 'border',
            inputText: 'text',
            inputFocus: 'borderFocus',
            
            progressBackground: 'surface',
            progressFill: 'primary',
            progressText: 'text',
        }
    }
};

export const defaultTheme = colorThemes.dark;

/**
 * Get a color theme by name
 */
export function getColorTheme(name?: string): ColorTheme {
    if (!name) return defaultTheme;
    return colorThemes[name] || defaultTheme;
}

/**
 * Create a custom color theme
 */
export function createCustomTheme(
    name: string,
    description: string,
    palette: Partial<ColorPalette>,
    semantics?: Partial<ColorTheme['semantic']>,
    mode: ColorMode = 'rgb'
): ColorTheme {
    return {
        name,
        description,
        palette: { ...defaultTheme.palette, ...palette },
        mode,
        semantic: { ...defaultTheme.semantic, ...semantics }
    };
}

/**
 * Convert color to ANSI escape sequence
 */
export function colorToAnsi(color: ColorDefinition, background: boolean = false): string {
    const base = background ? 48 : 38;
    return `\x1B[${base};2;${color.r};${color.g};${color.b}m`;
}

/**
 * Get semantic color from theme
 */
export function getSemanticColor(theme: ColorTheme, semantic: keyof ColorTheme['semantic']): string {
    const colorKey = theme.semantic[semantic];
    const color = theme.palette[colorKey];
    return colorToAnsi(color);
}

/**
 * Get semantic background color from theme
 */
export function getSemanticBackground(theme: ColorTheme, semantic: keyof ColorTheme['semantic']): string {
    const colorKey = theme.semantic[semantic];
    const color = theme.palette[colorKey];
    return colorToAnsi(color, true);
}

/**
 * Create a color variant (lighter/darker)
 */
export function createVariant(color: ColorDefinition, factor: number): ColorDefinition {
    if (factor > 0) {
        // Lighten
        return {
            r: Math.min(255, color.r + (255 - color.r) * factor),
            g: Math.min(255, color.g + (255 - color.g) * factor),
            b: Math.min(255, color.b + (255 - color.b) * factor),
        };
    } else {
        // Darken
        const darkFactor = Math.abs(factor);
        return {
            r: Math.max(0, color.r * (1 - darkFactor)),
            g: Math.max(0, color.g * (1 - darkFactor)),
            b: Math.max(0, color.b * (1 - darkFactor)),
        };
    }
}

/**
 * Reset all colors
 */
export const resetColor = '\x1B[0m';

/**
 * Theme builder class for creating custom themes
 */
export class ThemeBuilder {
    private theme: Partial<ColorTheme> = {};
    private palette: Partial<ColorPalette> = {};
    private semantic: Partial<ColorTheme['semantic']> = {};

    constructor(baseName?: string) {
        if (baseName) {
            const base = getColorTheme(baseName);
            this.theme = { ...base };
            this.palette = { ...base.palette };
            this.semantic = { ...base.semantic };
        }
    }

    name(name: string): this {
        this.theme.name = name;
        return this;
    }

    description(description: string): this {
        this.theme.description = description;
        return this;
    }

    mode(mode: ColorMode): this {
        this.theme.mode = mode;
        return this;
    }

    color(key: keyof ColorPalette, color: ColorDefinition): this {
        this.palette[key] = color;
        return this;
    }

    colors(colors: Partial<ColorPalette>): this {
        Object.assign(this.palette, colors);
        return this;
    }

    semanticMapping(key: keyof ColorTheme['semantic'], paletteKey: keyof ColorPalette): this {
        this.semantic[key] = paletteKey;
        return this;
    }

    semanticMappings(mappings: Partial<ColorTheme['semantic']>): this {
        Object.assign(this.semantic, mappings);
        return this;
    }

    build(): ColorTheme {
        return {
            name: this.theme.name || 'custom',
            description: this.theme.description || 'Custom theme',
            mode: this.theme.mode || 'rgb',
            palette: { ...defaultTheme.palette, ...this.palette },
            semantic: { ...defaultTheme.semantic, ...this.semantic }
        };
    }
}

/**
 * Helper function to create theme builder
 */
export function themeBuilder(baseName?: string): ThemeBuilder {
    return new ThemeBuilder(baseName);
}

/**
 * Theme registry for dynamic theme loading
 */
const themeRegistry = new Map<string, ColorTheme>();

// Register built-in themes
themeRegistry.set('dark', colorThemes.dark);
themeRegistry.set('light', colorThemes.light);
themeRegistry.set('terminal', colorThemes.terminal);

/**
 * Register a theme in the global registry
 */
export function registerTheme(theme: ColorTheme): void {
    themeRegistry.set(theme.name, theme);
}

/**
 * Get registered theme names
 */
export function getRegisteredThemeNames(): string[] {
    return Array.from(themeRegistry.keys());
}

/**
 * Get theme from registry (includes dynamically loaded themes)
 */
export function getRegisteredTheme(name: string): ColorTheme | undefined {
    return themeRegistry.get(name);
}

/**
 * Enhanced getColorTheme that checks registry first
 */
export function getColorThemeEnhanced(name?: string): ColorTheme {
    if (!name) return defaultTheme;
    
    // Check registry first (includes JSON loaded themes)
    const registeredTheme = themeRegistry.get(name);
    if (registeredTheme) return registeredTheme;
    
    // Fall back to built-in themes
    return colorThemes[name] || defaultTheme;
}