/**
 * Panel Widget - Modern, clean panels with comprehensive border and color themes
 * Provides sophisticated styling with Unicode borders, semantic colors, and typography
 */

import type { BorderTheme } from '../themes/borders';
import { getBorderTheme } from '../themes/borders';
import type { ColorTheme } from '../themes/colors';
import { getColorTheme, getSemanticColor, getSemanticBackground } from '../themes/colors';

export interface PanelConfig {
    id: string
    title?: string
    content?: string
    width?: number
    height?: number
    backgroundColor?: string  // Override color if needed
    borderStyle?: 'none' | 'minimal' | 'shadow' | 'outlined' | 'clean' | 'unicode' | 'fancy'
    borderTheme?: string  // Theme name from border themes
    colorTheme?: string   // Theme name from color themes
    padding?: number
    textAlign?: 'left' | 'center' | 'right'
    titleStyle?: 'normal' | 'bold' | 'accent'
}

export interface PanelStyle {
    backgroundColor: string
    borderColor: string
    shadowColor: string
    titleColor: string
    contentColor: string
    borderTheme: BorderTheme
    colorTheme: ColorTheme
}

/**
 * Get panel style based on color theme and border theme
 */
function getPanelStyle(
    borderStyle: string,
    colorTheme: ColorTheme,
    borderTheme?: string
): PanelStyle {
    // Map border styles to appropriate border themes if not specified
    const borderThemeMap: Record<string, string> = {
        'minimal': 'light',
        'shadow': 'block-light',
        'clean': 'ascii',
        'unicode': 'rounded',
        'fancy': 'double',
        'outlined': 'heavy',
        'none': 'ascii'
    };

    const selectedBorderTheme = getBorderTheme(borderTheme || borderThemeMap[borderStyle] || 'light');

    return {
        backgroundColor: getSemanticBackground(colorTheme, 'panelBackground'),
        borderColor: getSemanticColor(colorTheme, 'panelBorder'),
        shadowColor: getSemanticColor(colorTheme, 'panelShadow'),
        titleColor: getSemanticColor(colorTheme, 'panelTitle'),
        contentColor: getSemanticColor(colorTheme, 'panelContent'),
        borderTheme: selectedBorderTheme,
        colorTheme
    };
}

export const PanelStyles = {
    // Legacy styles for backward compatibility - now use theme system
    minimal: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('minimal', colorTheme),
    shadow: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('shadow', colorTheme),
    clean: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('clean', colorTheme),
    unicode: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('unicode', colorTheme),
    fancy: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('fancy', colorTheme),
    outlined: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('outlined', colorTheme),
    none: (colorTheme: ColorTheme = getColorTheme()) => getPanelStyle('none', colorTheme)
} as const

export function panel(config: PanelConfig): any {
    const {
        id,
        title,
        content = '',
        backgroundColor,
        borderStyle = 'clean',
        borderTheme,
        colorTheme,
        padding = 2,
        textAlign = 'center',
        titleStyle = 'normal'
    } = config

    // Get the appropriate color theme
    const theme = getColorTheme(colorTheme);
    const panelStyle = getPanelStyle(borderStyle, theme, borderTheme);

    // Build element with panel data and theme information
    const element = {
        tag: 'panel',
        id,
        classes: ['panel', `panel-${borderStyle}`, `text-${textAlign}`],
        attributes: {
            'data-title': title || '',
            'data-content': content,
            'data-background': backgroundColor || panelStyle.backgroundColor,
            'data-border-style': borderStyle,
            'data-border-theme': borderTheme || panelStyle.borderTheme.name,
            'data-color-theme': colorTheme || theme.name,
            'data-padding': padding.toString(),
            'data-title-style': titleStyle,
            // Store computed styles for rendering
            'data-computed-background': panelStyle.backgroundColor,
            'data-computed-border': panelStyle.borderColor,
            'data-computed-title': panelStyle.titleColor,
            'data-computed-content': panelStyle.contentColor,
            'data-computed-shadow': panelStyle.shadowColor
        },
        content: content,
        children: [],
        style: panelStyle  // Include style for direct access
    }

    return {
        ...element,
        child: (childElement: any) => ({
            ...element,
            children: [...element.children, childElement]
        }),
        build: () => element
    }
}

// Helper functions for common panel types
export function dashboardPanel(config: Omit<PanelConfig, 'borderStyle'>) {
    return panel({ ...config, borderStyle: 'clean' })
}

export function cardPanel(config: Omit<PanelConfig, 'borderStyle'>) {
    return panel({ ...config, borderStyle: 'shadow' })  
}

export function menuPanel(config: Omit<PanelConfig, 'borderStyle'>) {
    return panel({ ...config, borderStyle: 'minimal' })
}