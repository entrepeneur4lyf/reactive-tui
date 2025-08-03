/**
 * JSON Theme Loader
 * 
 * Loads color themes from JSON files and translates them into ANSI escape codes
 * for terminal rendering. Supports various color formats and validation.
 */

import { readFileSync, existsSync } from 'fs';
import { resolve, dirname } from 'path';
import type { ColorTheme, ColorPalette, ColorDefinition, ColorMode } from './colors';
import { rgb, hex, colorToAnsi } from './colors';

export interface JSONColorDefinition {
    r?: number;
    g?: number;
    b?: number;
    hex?: string;
    rgb?: [number, number, number];
    ansi?: number;
    name?: string; // Named color reference
}

export interface JSONColorPalette {
    // Primary colors
    primary?: JSONColorDefinition;
    primaryDark?: JSONColorDefinition;
    primaryLight?: JSONColorDefinition;
    
    // Secondary colors
    secondary?: JSONColorDefinition;
    secondaryDark?: JSONColorDefinition;
    secondaryLight?: JSONColorDefinition;
    
    // Neutral colors
    background?: JSONColorDefinition;
    backgroundAlt?: JSONColorDefinition;
    surface?: JSONColorDefinition;
    surfaceAlt?: JSONColorDefinition;
    
    // Text colors
    text?: JSONColorDefinition;
    textSecondary?: JSONColorDefinition;
    textMuted?: JSONColorDefinition;
    textInverse?: JSONColorDefinition;
    
    // Border colors
    border?: JSONColorDefinition;
    borderFocus?: JSONColorDefinition;
    borderHover?: JSONColorDefinition;
    
    // Status colors
    success?: JSONColorDefinition;
    warning?: JSONColorDefinition;
    error?: JSONColorDefinition;
    info?: JSONColorDefinition;
    
    // Interactive colors
    hover?: JSONColorDefinition;
    active?: JSONColorDefinition;
    disabled?: JSONColorDefinition;
    
    // Shadow colors
    shadow?: JSONColorDefinition;
    shadowLight?: JSONColorDefinition;
    
    // Custom colors (extensible)
    [key: string]: JSONColorDefinition | undefined;
}

export interface JSONColorTheme {
    name: string;
    description: string;
    version?: string;
    author?: string;
    mode?: ColorMode;
    palette: JSONColorPalette;
    semantic?: {
        [key: string]: string; // Maps to palette keys
    };
    extends?: string; // Base theme to extend
    imports?: string[]; // Additional themes to import colors from
}

export interface JSONThemeCollection {
    version?: string;
    themes: JSONColorTheme[];
    namedColors?: Record<string, JSONColorDefinition>;
}

// Named color registry for cross-references
const namedColors = new Map<string, ColorDefinition>();

// Theme cache to avoid reloading
const themeCache = new Map<string, ColorTheme>();

/**
 * Convert JSON color definition to ColorDefinition
 */
export function parseJSONColor(jsonColor: JSONColorDefinition, namedColorMap?: Map<string, ColorDefinition>): ColorDefinition {
    // Handle named color references
    if (jsonColor.name) {
        const namedColor = namedColorMap?.get(jsonColor.name) || namedColors.get(jsonColor.name);
        if (namedColor) {
            return namedColor;
        }
        throw new Error(`Named color '${jsonColor.name}' not found`);
    }
    
    // Handle hex format
    if (jsonColor.hex) {
        return hex(jsonColor.hex);
    }
    
    // Handle RGB array format
    if (jsonColor.rgb && Array.isArray(jsonColor.rgb) && jsonColor.rgb.length === 3) {
        return rgb(jsonColor.rgb[0], jsonColor.rgb[1], jsonColor.rgb[2]);
    }
    
    // Handle RGB object format
    if (typeof jsonColor.r === 'number' && typeof jsonColor.g === 'number' && typeof jsonColor.b === 'number') {
        return rgb(jsonColor.r, jsonColor.g, jsonColor.b);
    }
    
    // Handle ANSI color codes (0-255)
    if (typeof jsonColor.ansi === 'number') {
        return ansiToRgb(jsonColor.ansi);
    }
    
    throw new Error(`Invalid color definition: ${JSON.stringify(jsonColor)}`);
}

/**
 * Convert ANSI color code (0-255) to RGB
 */
function ansiToRgb(ansi: number): ColorDefinition {
    if (ansi < 0 || ansi > 255) {
        throw new Error(`ANSI color code must be between 0-255, got ${ansi}`);
    }
    
    // Standard colors (0-15)
    if (ansi < 16) {
        const standardColors: [number, number, number][] = [
            [0, 0, 0],       // Black
            [128, 0, 0],     // Dark Red
            [0, 128, 0],     // Dark Green
            [128, 128, 0],   // Dark Yellow
            [0, 0, 128],     // Dark Blue
            [128, 0, 128],   // Dark Magenta
            [0, 128, 128],   // Dark Cyan
            [192, 192, 192], // Light Gray
            [128, 128, 128], // Dark Gray
            [255, 0, 0],     // Red
            [0, 255, 0],     // Green
            [255, 255, 0],   // Yellow
            [0, 0, 255],     // Blue
            [255, 0, 255],   // Magenta
            [0, 255, 255],   // Cyan
            [255, 255, 255], // White
        ];
        const [r, g, b] = standardColors[ansi];
        return rgb(r, g, b);
    }
    
    // 216-color cube (16-231)
    if (ansi < 232) {
        const colorIndex = ansi - 16;
        const r = Math.floor(colorIndex / 36);
        const g = Math.floor((colorIndex % 36) / 6);
        const b = colorIndex % 6;
        
        const toRgbValue = (val: number) => val === 0 ? 0 : 55 + val * 40;
        return rgb(toRgbValue(r), toRgbValue(g), toRgbValue(b));
    }
    
    // Grayscale (232-255)
    const gray = (ansi - 232) * 10 + 8;
    return rgb(gray, gray, gray);
}

/**
 * Convert JSON palette to ColorPalette
 */
function parseJSONPalette(jsonPalette: JSONColorPalette, namedColorMap?: Map<string, ColorDefinition>): ColorPalette {
    const palette: Partial<ColorPalette> = {};
    
    for (const [key, jsonColor] of Object.entries(jsonPalette)) {
        if (jsonColor) {
            try {
                palette[key as keyof ColorPalette] = parseJSONColor(jsonColor, namedColorMap);
            } catch (error) {
                console.warn(`Failed to parse color '${key}':`, error);
            }
        }
    }
    
    // Ensure required colors have defaults
    const defaults: ColorPalette = {
        primary: rgb(99, 102, 241),
        primaryDark: rgb(79, 70, 229),
        primaryLight: rgb(129, 140, 248),
        secondary: rgb(16, 185, 129),
        secondaryDark: rgb(5, 150, 105),
        secondaryLight: rgb(52, 211, 153),
        background: rgb(17, 24, 39),
        backgroundAlt: rgb(31, 41, 55),
        surface: rgb(55, 65, 81),
        surfaceAlt: rgb(75, 85, 99),
        text: rgb(249, 250, 251),
        textSecondary: rgb(209, 213, 219),
        textMuted: rgb(156, 163, 175),
        textInverse: rgb(17, 24, 39),
        border: rgb(75, 85, 99),
        borderFocus: rgb(99, 102, 241),
        borderHover: rgb(107, 114, 128),
        success: rgb(34, 197, 94),
        warning: rgb(251, 191, 36),
        error: rgb(239, 68, 68),
        info: rgb(59, 130, 246),
        hover: rgb(67, 56, 202),
        active: rgb(55, 48, 163),
        disabled: rgb(107, 114, 128),
        shadow: rgb(0, 0, 0),
        shadowLight: rgb(31, 41, 55),
    };
    
    return { ...defaults, ...palette } as ColorPalette;
}

/**
 * Load theme from JSON file with validation
 */
export function loadThemeFromFile(filePath: string): ColorTheme {
    const absolutePath = resolve(filePath);
    
    if (!existsSync(absolutePath)) {
        throw new Error(`Theme file not found: ${absolutePath}`);
    }
    
    // Check cache first
    if (themeCache.has(absolutePath)) {
        return themeCache.get(absolutePath)!;
    }
    
    try {
        const fileContent = readFileSync(absolutePath, 'utf-8');
        let jsonData: any;
        
        try {
            jsonData = JSON.parse(fileContent);
        } catch (parseError) {
            throw new Error(`Invalid JSON syntax in theme file: ${parseError}`);
        }
        
        let theme: ColorTheme;
        
        if ('themes' in jsonData) {
            // Handle theme collection - validate and load first theme
            const collectionErrors = validateJSONThemeCollection(jsonData);
            if (collectionErrors.length > 0) {
                throw new Error(`Theme collection validation failed:\n${collectionErrors.join('\n')}`);
            }
            
            if (jsonData.themes.length === 0) {
                throw new Error('Theme collection is empty');
            }
            
            // Register named colors if provided
            if (jsonData.namedColors) {
                registerNamedColors(jsonData.namedColors);
            }
            
            theme = parseJSONTheme(jsonData.themes[0], dirname(absolutePath));
        } else {
            // Handle single theme - validate first
            const themeErrors = validateJSONTheme(jsonData);
            if (themeErrors.length > 0) {
                throw new Error(`Theme validation failed:\n${themeErrors.join('\n')}`);
            }
            
            theme = parseJSONTheme(jsonData, dirname(absolutePath));
        }
        
        // Cache the theme
        themeCache.set(absolutePath, theme);
        return theme;
        
    } catch (error) {
        if (error instanceof Error) {
            throw error; // Re-throw validation errors as-is
        }
        throw new Error(`Failed to load theme from ${filePath}: ${error}`);
    }
}

/**
 * Load multiple themes from JSON collection file with validation
 */
export function loadThemeCollectionFromFile(filePath: string): ColorTheme[] {
    const absolutePath = resolve(filePath);
    
    if (!existsSync(absolutePath)) {
        throw new Error(`Theme collection file not found: ${absolutePath}`);
    }
    
    try {
        const fileContent = readFileSync(absolutePath, 'utf-8');
        let jsonData: any;
        
        try {
            jsonData = JSON.parse(fileContent);
        } catch (parseError) {
            throw new Error(`Invalid JSON syntax in collection file: ${parseError}`);
        }
        
        // Validate collection structure
        const collectionErrors = validateJSONThemeCollection(jsonData);
        if (collectionErrors.length > 0) {
            throw new Error(`Collection validation failed:\n${collectionErrors.join('\n')}`);
        }
        
        if (!jsonData.themes || !Array.isArray(jsonData.themes)) {
            throw new Error('Invalid theme collection format');
        }
        
        // Register named colors if provided
        if (jsonData.namedColors) {
            registerNamedColors(jsonData.namedColors);
        }
        
        const themes: ColorTheme[] = [];
        const baseDir = dirname(absolutePath);
        const parseErrors: string[] = [];
        
        for (const [index, jsonTheme] of jsonData.themes.entries()) {
            try {
                const theme = parseJSONTheme(jsonTheme, baseDir);
                themes.push(theme);
            } catch (error) {
                const errorMsg = `Theme ${index + 1} (${jsonTheme?.name || 'unnamed'}): ${error}`;
                parseErrors.push(errorMsg);
                console.warn(errorMsg);
            }
        }
        
        if (themes.length === 0) {
            throw new Error(`No valid themes found in collection. Errors:\n${parseErrors.join('\n')}`);
        }
        
        if (parseErrors.length > 0) {
            console.warn(`Successfully loaded ${themes.length}/${jsonData.themes.length} themes. ${parseErrors.length} themes had errors.`);
        }
        
        return themes;
        
    } catch (error) {
        if (error instanceof Error) {
            throw error; // Re-throw validation errors as-is
        }
        throw new Error(`Failed to load theme collection from ${filePath}: ${error}`);
    }
}

/**
 * Register named colors for cross-references
 */
function registerNamedColors(namedColorDefs: Record<string, JSONColorDefinition>) {
    for (const [name, jsonColor] of Object.entries(namedColorDefs)) {
        try {
            const color = parseJSONColor(jsonColor);
            namedColors.set(name, color);
        } catch (error) {
            console.warn(`Failed to register named color '${name}':`, error);
        }
    }
}

/**
 * Parse JSON theme to ColorTheme
 */
function parseJSONTheme(jsonTheme: JSONColorTheme, baseDir: string): ColorTheme {
    const namedColorMap = new Map(namedColors);
    
    // Handle theme extension
    let basePalette: Partial<ColorPalette> = {};
    if (jsonTheme.extends) {
        try {
            const baseThemePath = resolve(baseDir, jsonTheme.extends);
            const baseTheme = loadThemeFromFile(baseThemePath);
            basePalette = baseTheme.palette;
        } catch (error) {
            console.warn(`Failed to extend theme '${jsonTheme.extends}':`, error);
        }
    }
    
    // Handle imports
    if (jsonTheme.imports) {
        for (const importPath of jsonTheme.imports) {
            try {
                const importThemePath = resolve(baseDir, importPath);
                const importedTheme = loadThemeFromFile(importThemePath);
                // Merge imported colors into named color map
                for (const [key, color] of Object.entries(importedTheme.palette)) {
                    namedColorMap.set(`${importedTheme.name}.${key}`, color);
                }
            } catch (error) {
                console.warn(`Failed to import theme '${importPath}':`, error);
            }
        }
    }
    
    // Parse current theme palette
    const currentPalette = parseJSONPalette(jsonTheme.palette, namedColorMap);
    
    // Merge with base palette (base colors first, then current theme overrides)
    const palette = { ...basePalette, ...currentPalette } as ColorPalette;
    
    // Default semantic mappings
    const defaultSemantic = {
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
    };
    
    return {
        name: jsonTheme.name,
        description: jsonTheme.description,
        mode: jsonTheme.mode || 'rgb',
        palette,
        semantic: { ...defaultSemantic, ...jsonTheme.semantic }
    } as ColorTheme;
}

/**
 * Save theme to JSON file
 */
export function saveThemeToFile(theme: ColorTheme, filePath: string): void {
    const jsonTheme: JSONColorTheme = {
        name: theme.name,
        description: theme.description,
        mode: theme.mode,
        palette: {},
        semantic: theme.semantic
    };
    
    // Convert palette to JSON format
    for (const [key, color] of Object.entries(theme.palette)) {
        jsonTheme.palette[key] = {
            r: color.r,
            g: color.g,
            b: color.b
        };
    }
    
    const json = JSON.stringify(jsonTheme, null, 2);
    const fs = require('fs');
    fs.writeFileSync(resolve(filePath), json, 'utf-8');
}

/**
 * Convert theme to ANSI escape codes for terminal output
 */
export function themeToAnsiCodes(theme: ColorTheme): Record<string, string> {
    const ansiCodes: Record<string, string> = {};
    
    // Convert palette colors to ANSI
    for (const [key, color] of Object.entries(theme.palette)) {
        ansiCodes[key] = colorToAnsi(color);
        ansiCodes[`${key}_bg`] = colorToAnsi(color, true);
    }
    
    // Add semantic mappings
    for (const [semantic, paletteKey] of Object.entries(theme.semantic)) {
        const color = theme.palette[paletteKey as keyof typeof theme.palette];
        if (color) {
            ansiCodes[semantic] = colorToAnsi(color);
            ansiCodes[`${semantic}_bg`] = colorToAnsi(color, true);
        }
    }
    
    // Add reset code
    ansiCodes.reset = '\x1B[0m';
    
    return ansiCodes;
}

/**
 * Get theme preview with ANSI colors
 */
export function getThemePreview(theme: ColorTheme): string {
    const ansi = themeToAnsiCodes(theme);
    const lines: string[] = [];
    
    lines.push(`Theme: ${ansi.primary}${theme.name}${ansi.reset} - ${theme.description}`);
    lines.push(`Mode: ${theme.mode}`);
    lines.push('');
    
    // Color swatches
    const colors = ['primary', 'secondary', 'success', 'warning', 'error', 'info'];
    for (const color of colors) {
        if (ansi[color]) {
            lines.push(`${ansi[`${color}_bg`]}  ${ansi.reset} ${ansi[color]}${color}${ansi.reset}`);
        }
    }
    
    lines.push('');
    lines.push(`${ansi.panelBackground_bg}${ansi.panelBorder}┌─ Sample Panel ─┐${ansi.reset}`);
    lines.push(`${ansi.panelBackground_bg}${ansi.panelBorder}│ ${ansi.panelTitle}Panel content${ansi.panelBorder} │${ansi.reset}`);
    lines.push(`${ansi.panelBackground_bg}${ansi.panelBorder}└─────────────────┘${ansi.reset}`);
    
    return lines.join('\n');
}

/**
 * Validate JSON color definition
 */
export function validateJSONColor(jsonColor: any, colorName: string): string[] {
    const errors: string[] = [];
    
    if (!jsonColor || typeof jsonColor !== 'object') {
        errors.push(`Color '${colorName}' must be an object`);
        return errors;
    }
    
    let hasValidFormat = false;
    
    // Check for hex format
    if (jsonColor.hex) {
        if (typeof jsonColor.hex !== 'string') {
            errors.push(`Color '${colorName}': hex must be a string`);
        } else if (!/^#?[0-9A-Fa-f]{6}$/.test(jsonColor.hex)) {
            errors.push(`Color '${colorName}': invalid hex format '${jsonColor.hex}' - must be #RRGGBB or RRGGBB`);
        } else {
            hasValidFormat = true;
        }
    }
    
    // Check for RGB array format
    if (jsonColor.rgb) {
        if (!Array.isArray(jsonColor.rgb)) {
            errors.push(`Color '${colorName}': rgb must be an array`);
        } else if (jsonColor.rgb.length !== 3) {
            errors.push(`Color '${colorName}': rgb array must have exactly 3 values`);
        } else {
            const [r, g, b] = jsonColor.rgb;
            if (!Number.isInteger(r) || r < 0 || r > 255) {
                errors.push(`Color '${colorName}': invalid red value ${r} - must be 0-255`);
            }
            if (!Number.isInteger(g) || g < 0 || g > 255) {
                errors.push(`Color '${colorName}': invalid green value ${g} - must be 0-255`);
            }
            if (!Number.isInteger(b) || b < 0 || b > 255) {
                errors.push(`Color '${colorName}': invalid blue value ${b} - must be 0-255`);
            }
            if (errors.filter(e => e.includes(colorName)).length === 0) {
                hasValidFormat = true;
            }
        }
    }
    
    // Check for RGB object format
    if (typeof jsonColor.r === 'number' || typeof jsonColor.g === 'number' || typeof jsonColor.b === 'number') {
        if (typeof jsonColor.r !== 'number' || typeof jsonColor.g !== 'number' || typeof jsonColor.b !== 'number') {
            errors.push(`Color '${colorName}': r, g, b must all be numbers`);
        } else {
            if (!Number.isInteger(jsonColor.r) || jsonColor.r < 0 || jsonColor.r > 255) {
                errors.push(`Color '${colorName}': invalid red value ${jsonColor.r} - must be 0-255`);
            }
            if (!Number.isInteger(jsonColor.g) || jsonColor.g < 0 || jsonColor.g > 255) {
                errors.push(`Color '${colorName}': invalid green value ${jsonColor.g} - must be 0-255`);
            }
            if (!Number.isInteger(jsonColor.b) || jsonColor.b < 0 || jsonColor.b > 255) {
                errors.push(`Color '${colorName}': invalid blue value ${jsonColor.b} - must be 0-255`);
            }
            if (errors.filter(e => e.includes(colorName)).length === 0) {
                hasValidFormat = true;
            }
        }
    }
    
    // Check for ANSI format
    if (typeof jsonColor.ansi === 'number') {
        if (!Number.isInteger(jsonColor.ansi) || jsonColor.ansi < 0 || jsonColor.ansi > 255) {
            errors.push(`Color '${colorName}': invalid ANSI code ${jsonColor.ansi} - must be 0-255`);
        } else {
            hasValidFormat = true;
        }
    }
    
    // Check for named color reference
    if (jsonColor.name) {
        if (typeof jsonColor.name !== 'string') {
            errors.push(`Color '${colorName}': name must be a string`);
        } else if (jsonColor.name.trim().length === 0) {
            errors.push(`Color '${colorName}': name cannot be empty`);
        } else {
            hasValidFormat = true;
        }
    }
    
    if (!hasValidFormat) {
        errors.push(`Color '${colorName}': must have at least one valid format (hex, rgb array, r/g/b object, ansi, or name)`);
    }
    
    // Check for conflicting formats
    const formatCount = [
        jsonColor.hex ? 1 : 0,
        jsonColor.rgb ? 1 : 0,
        (jsonColor.r !== undefined || jsonColor.g !== undefined || jsonColor.b !== undefined) ? 1 : 0,
        jsonColor.ansi !== undefined ? 1 : 0,
        jsonColor.name ? 1 : 0
    ].reduce((a, b) => a + b, 0);
    
    if (formatCount > 1) {
        errors.push(`Color '${colorName}': multiple color formats specified - use only one format per color`);
    }
    
    return errors;
}

/**
 * Validate JSON theme palette
 */
export function validateJSONPalette(palette: any): string[] {
    const errors: string[] = [];
    
    if (!palette || typeof palette !== 'object') {
        errors.push('Palette must be an object');
        return errors;
    }
    
    // Required colors
    const requiredColors = [
        'primary', 'background', 'text', 'border'
    ];
    
    for (const required of requiredColors) {
        if (!palette[required]) {
            errors.push(`Missing required color: ${required}`);
        }
    }
    
    // Validate each color definition
    for (const [colorName, colorDef] of Object.entries(palette)) {
        const colorErrors = validateJSONColor(colorDef, colorName);
        errors.push(...colorErrors);
    }
    
    return errors;
}

/**
 * Validate semantic mappings
 */
export function validateSemanticMappings(semantic: any, palette: any): string[] {
    const errors: string[] = [];
    
    if (!semantic || typeof semantic !== 'object') {
        return errors; // Semantic mappings are optional
    }
    
    const requiredSemantic = [
        'panelBackground', 'panelBorder', 'panelTitle', 'panelContent'
    ];
    
    for (const required of requiredSemantic) {
        if (!semantic[required]) {
            errors.push(`Missing required semantic mapping: ${required}`);
        }
    }
    
    // Validate that semantic mappings reference valid palette colors
    for (const [semanticName, paletteKey] of Object.entries(semantic)) {
        if (typeof paletteKey !== 'string') {
            errors.push(`Semantic mapping '${semanticName}' must be a string`);
            continue;
        }
        
        if (!palette || !palette[paletteKey]) {
            errors.push(`Semantic mapping '${semanticName}' references unknown palette color '${paletteKey}'`);
        }
    }
    
    return errors;
}

/**
 * Validate JSON theme structure with comprehensive checks
 */
export function validateJSONTheme(jsonTheme: any): string[] {
    const errors: string[] = [];
    
    // Basic structure validation
    if (!jsonTheme || typeof jsonTheme !== 'object') {
        errors.push('Theme must be an object');
        return errors;
    }
    
    // Name validation
    if (!jsonTheme.name) {
        errors.push('Theme must have a name');
    } else if (typeof jsonTheme.name !== 'string') {
        errors.push('Theme name must be a string');
    } else if (jsonTheme.name.trim().length === 0) {
        errors.push('Theme name cannot be empty');
    } else if (!/^[a-zA-Z0-9_-]+$/.test(jsonTheme.name)) {
        errors.push('Theme name can only contain letters, numbers, underscores, and hyphens');
    }
    
    // Description validation
    if (!jsonTheme.description) {
        errors.push('Theme must have a description');
    } else if (typeof jsonTheme.description !== 'string') {
        errors.push('Theme description must be a string');
    } else if (jsonTheme.description.trim().length === 0) {
        errors.push('Theme description cannot be empty');
    }
    
    // Version validation (optional)
    if (jsonTheme.version && typeof jsonTheme.version !== 'string') {
        errors.push('Theme version must be a string');
    }
    
    // Author validation (optional)
    if (jsonTheme.author && typeof jsonTheme.author !== 'string') {
        errors.push('Theme author must be a string');
    }
    
    // Mode validation
    if (jsonTheme.mode && !['rgb', 'ansi', 'auto'].includes(jsonTheme.mode)) {
        errors.push('Invalid color mode - must be rgb, ansi, or auto');
    }
    
    // Palette validation
    const paletteErrors = validateJSONPalette(jsonTheme.palette);
    errors.push(...paletteErrors);
    
    // Semantic mappings validation
    const semanticErrors = validateSemanticMappings(jsonTheme.semantic, jsonTheme.palette);
    errors.push(...semanticErrors);
    
    // Extends validation (optional)
    if (jsonTheme.extends) {
        if (typeof jsonTheme.extends !== 'string') {
            errors.push('Theme extends must be a string path');
        } else if (jsonTheme.extends.trim().length === 0) {
            errors.push('Theme extends path cannot be empty');
        }
    }
    
    // Imports validation (optional)
    if (jsonTheme.imports) {
        if (!Array.isArray(jsonTheme.imports)) {
            errors.push('Theme imports must be an array');
        } else {
            jsonTheme.imports.forEach((importPath: any, index: number) => {
                if (typeof importPath !== 'string') {
                    errors.push(`Theme import at index ${index} must be a string`);
                } else if (importPath.trim().length === 0) {
                    errors.push(`Theme import at index ${index} cannot be empty`);
                }
            });
        }
    }
    
    return errors;
}

/**
 * Validate JSON theme collection
 */
export function validateJSONThemeCollection(collection: any): string[] {
    const errors: string[] = [];
    
    if (!collection || typeof collection !== 'object') {
        errors.push('Theme collection must be an object');
        return errors;
    }
    
    // Version validation (optional)
    if (collection.version && typeof collection.version !== 'string') {
        errors.push('Collection version must be a string');
    }
    
    // Themes validation
    if (!collection.themes) {
        errors.push('Collection must have a themes array');
    } else if (!Array.isArray(collection.themes)) {
        errors.push('Collection themes must be an array');
    } else if (collection.themes.length === 0) {
        errors.push('Collection must contain at least one theme');
    } else {
        collection.themes.forEach((theme: any, index: number) => {
            const themeErrors = validateJSONTheme(theme);
            themeErrors.forEach(error => {
                errors.push(`Theme ${index + 1}: ${error}`);
            });
        });
        
        // Check for duplicate theme names
        const themeNames = collection.themes
            .filter((t: any) => t && t.name)
            .map((t: any) => t.name);
        const duplicates = themeNames.filter((name: string, index: number) => 
            themeNames.indexOf(name) !== index
        );
        
        if (duplicates.length > 0) {
            errors.push(`Duplicate theme names found: ${[...new Set(duplicates)].join(', ')}`);
        }
    }
    
    // Named colors validation (optional)
    if (collection.namedColors) {
        if (typeof collection.namedColors !== 'object') {
            errors.push('Named colors must be an object');
        } else {
            for (const [colorName, colorDef] of Object.entries(collection.namedColors)) {
                const colorErrors = validateJSONColor(colorDef, `named.${colorName}`);
                errors.push(...colorErrors);
            }
        }
    }
    
    return errors;
}

/**
 * Validate and sanitize theme name
 */
export function sanitizeThemeName(name: string): string {
    if (typeof name !== 'string') {
        throw new Error('Theme name must be a string');
    }
    
    // Remove invalid characters and normalize
    const sanitized = name
        .trim()
        .toLowerCase()
        .replace(/[^a-z0-9_-]/g, '-')
        .replace(/-+/g, '-')
        .replace(/^-|-$/g, '');
    
    if (sanitized.length === 0) {
        throw new Error('Theme name cannot be empty after sanitization');
    }
    
    return sanitized;
}

/**
 * Clear theme cache
 */
export function clearThemeCache(): void {
    themeCache.clear();
    namedColors.clear();
}