/**
 * Border Theme System
 * 
 * Provides selectable border themes using the comprehensive Unicode
 * border character system from tui-core. Themes range from classic
 * ASCII to modern Unicode designs with various weights and styles.
 */

export interface BorderChars {
    horizontal: string;
    vertical: string;
    topLeft: string;
    topRight: string;
    bottomLeft: string;
    bottomRight: string;
    cross: string;
    tDown: string;
    tUp: string;
    tLeft: string;
    tRight: string;
}

export interface BorderTheme {
    name: string;
    description: string;
    chars: BorderChars;
    weight: 'light' | 'medium' | 'heavy';
    style: 'solid' | 'dashed' | 'dotted' | 'double' | 'rounded' | 'block';
}

// Classic ASCII theme for maximum compatibility
const asciiTheme: BorderTheme = {
    name: 'ascii',
    description: 'Classic ASCII characters for maximum compatibility',
    chars: {
        horizontal: '-',
        vertical: '|',
        topLeft: '+',
        topRight: '+',
        bottomLeft: '+',
        bottomRight: '+',
        cross: '+',
        tDown: '+',
        tUp: '+',
        tLeft: '+',
        tRight: '+',
    },
    weight: 'light',
    style: 'solid'
};

// Light Unicode box-drawing characters
const lightTheme: BorderTheme = {
    name: 'light',
    description: 'Light Unicode box-drawing characters',
    chars: {
        horizontal: '─',     // U+2500
        vertical: '│',       // U+2502
        topLeft: '┌',        // U+250C
        topRight: '┐',       // U+2510
        bottomLeft: '└',     // U+2514
        bottomRight: '┘',    // U+2518
        cross: '┼',          // U+253C
        tDown: '┬',          // U+252C
        tUp: '┴',            // U+2534
        tLeft: '┤',          // U+2524
        tRight: '├',         // U+251C
    },
    weight: 'light',
    style: 'solid'
};

// Heavy Unicode box-drawing characters
const heavyTheme: BorderTheme = {
    name: 'heavy',
    description: 'Heavy Unicode box-drawing characters',
    chars: {
        horizontal: '━',     // U+2501
        vertical: '┃',       // U+2503
        topLeft: '┏',        // U+250F
        topRight: '┓',       // U+2513
        bottomLeft: '┗',     // U+2517
        bottomRight: '┛',    // U+251B
        cross: '╋',          // U+254B
        tDown: '┳',          // U+2533
        tUp: '┻',            // U+253B
        tLeft: '┫',          // U+252B
        tRight: '┣',         // U+2523
    },
    weight: 'heavy',
    style: 'solid'
};

// Double-line Unicode characters
const doubleTheme: BorderTheme = {
    name: 'double',
    description: 'Double-line Unicode box-drawing characters',
    chars: {
        horizontal: '═',     // U+2550
        vertical: '║',       // U+2551
        topLeft: '╔',        // U+2554
        topRight: '╗',       // U+2557
        bottomLeft: '╚',     // U+255A
        bottomRight: '╝',    // U+255D
        cross: '╬',          // U+256C
        tDown: '╦',          // U+2566
        tUp: '╩',            // U+2569
        tLeft: '╣',          // U+2563
        tRight: '╠',         // U+2560
    },
    weight: 'heavy',
    style: 'double'
};

// Rounded corners for modern look
const roundedTheme: BorderTheme = {
    name: 'rounded',
    description: 'Light borders with rounded corners',
    chars: {
        horizontal: '─',     // U+2500
        vertical: '│',       // U+2502
        topLeft: '╭',        // U+256D
        topRight: '╮',       // U+256E
        bottomLeft: '╰',     // U+2570
        bottomRight: '╯',    // U+256F
        cross: '┼',          // U+253C
        tDown: '┬',          // U+252C
        tUp: '┴',            // U+2534
        tLeft: '┤',          // U+2524
        tRight: '├',         // U+251C
    },
    weight: 'light',
    style: 'rounded'
};

// Dashed light theme
const dashedLightTheme: BorderTheme = {
    name: 'dashed-light',
    description: 'Light dashed Unicode characters',
    chars: {
        horizontal: '┄',     // U+2504 (triple dash)
        vertical: '┆',       // U+2506 (triple dash)
        topLeft: '┌',        // U+250C
        topRight: '┐',       // U+2510
        bottomLeft: '└',     // U+2514
        bottomRight: '┘',    // U+2518
        cross: '┼',          // U+253C
        tDown: '┬',          // U+252C
        tUp: '┴',            // U+2534
        tLeft: '┤',          // U+2524
        tRight: '├',         // U+251C
    },
    weight: 'light',
    style: 'dashed'
};

// Dashed heavy theme
const dashedHeavyTheme: BorderTheme = {
    name: 'dashed-heavy',
    description: 'Heavy dashed Unicode characters',
    chars: {
        horizontal: '┅',     // U+2505 (heavy triple dash)
        vertical: '┇',       // U+2507 (heavy triple dash)
        topLeft: '┏',        // U+250F
        topRight: '┓',       // U+2513
        bottomLeft: '┗',     // U+2517
        bottomRight: '┛',    // U+251B
        cross: '╋',          // U+254B
        tDown: '┳',          // U+2533
        tUp: '┻',            // U+253B
        tLeft: '┫',          // U+252B
        tRight: '┣',         // U+2523
    },
    weight: 'heavy',
    style: 'dashed'
};

// Dotted theme using quadruple dash
const dottedTheme: BorderTheme = {
    name: 'dotted',
    description: 'Dotted Unicode characters',
    chars: {
        horizontal: '┈',     // U+2508 (quadruple dash)
        vertical: '┊',       // U+250A (quadruple dash)
        topLeft: '┌',        // U+250C
        topRight: '┐',       // U+2510
        bottomLeft: '└',     // U+2514
        bottomRight: '┘',    // U+2518
        cross: '┼',          // U+253C
        tDown: '┬',          // U+252C
        tUp: '┴',            // U+2534
        tLeft: '┤',          // U+2524
        tRight: '├',         // U+251C
    },
    weight: 'light',
    style: 'dotted'
};

// Block-based theme using shade characters
const blockLightTheme: BorderTheme = {
    name: 'block-light',
    description: 'Light shade block characters',
    chars: {
        horizontal: '░',     // U+2591 (light shade)
        vertical: '░',       // U+2591 (light shade)
        topLeft: '▘',        // U+2598 (quadrant upper left)
        topRight: '▝',       // U+259D (quadrant upper right)
        bottomLeft: '▖',     // U+2596 (quadrant lower left)
        bottomRight: '▗',    // U+2597 (quadrant lower right)
        cross: '░',          // U+2591 (light shade)
        tDown: '▀',          // U+2580 (upper half block)
        tUp: '▄',            // U+2584 (lower half block)
        tLeft: '▐',          // U+2590 (right half block)
        tRight: '▌',         // U+258C (left half block)
    },
    weight: 'medium',
    style: 'block'
};

// Block-based theme using solid blocks
const blockSolidTheme: BorderTheme = {
    name: 'block-solid',
    description: 'Solid block characters',
    chars: {
        horizontal: '█',     // U+2588 (full block)
        vertical: '█',       // U+2588 (full block)
        topLeft: '█',        // U+2588 (full block)
        topRight: '█',       // U+2588 (full block)
        bottomLeft: '█',     // U+2588 (full block)
        bottomRight: '█',    // U+2588 (full block)
        cross: '█',          // U+2588 (full block)
        tDown: '█',          // U+2588 (full block)
        tUp: '█',            // U+2588 (full block)
        tLeft: '█',          // U+2588 (full block)
        tRight: '█',         // U+2588 (full block)
    },
    weight: 'heavy',
    style: 'block'
};

// Export all available themes
export const borderThemes: Record<string, BorderTheme> = {
    ascii: asciiTheme,
    light: lightTheme,
    heavy: heavyTheme,
    double: doubleTheme,
    rounded: roundedTheme,
    'dashed-light': dashedLightTheme,
    'dashed-heavy': dashedHeavyTheme,
    dotted: dottedTheme,
    'block-light': blockLightTheme,
    'block-solid': blockSolidTheme,
};

export const defaultTheme = lightTheme;

/**
 * Get a border theme by name
 */
export function getBorderTheme(name?: string): BorderTheme {
    if (!name) return defaultTheme;
    return borderThemes[name] || defaultTheme;
}

/**
 * Get all available theme names
 */
export function getThemeNames(): string[] {
    return Object.keys(borderThemes);
}

/**
 * Get themes by category
 */
export function getThemesByStyle(style: BorderTheme['style']): BorderTheme[] {
    return Object.values(borderThemes).filter(theme => theme.style === style);
}

export function getThemesByWeight(weight: BorderTheme['weight']): BorderTheme[] {
    return Object.values(borderThemes).filter(theme => theme.weight === weight);
}

/**
 * Render a border using the specified theme
 */
export function renderBorder(
    theme: BorderTheme,
    width: number,
    height: number
): string[] {
    if (width < 2 || height < 2) {
        return [];
    }

    const lines: string[] = [];
    const chars = theme.chars;

    // Top border
    lines.push(
        chars.topLeft + 
        chars.horizontal.repeat(width - 2) + 
        chars.topRight
    );

    // Middle rows
    for (let i = 1; i < height - 1; i++) {
        lines.push(
            chars.vertical + 
            ' '.repeat(width - 2) + 
            chars.vertical
        );
    }

    // Bottom border
    if (height > 1) {
        lines.push(
            chars.bottomLeft + 
            chars.horizontal.repeat(width - 2) + 
            chars.bottomRight
        );
    }

    return lines;
}

/**
 * Create a preview of all available themes
 */
export function createThemePreview(): string {
    const previews: string[] = [];
    
    Object.values(borderThemes).forEach(theme => {
        const border = renderBorder(theme, 12, 4);
        previews.push(`${theme.name} (${theme.description}):`);
        border.forEach(line => previews.push(`  ${line}`));
        previews.push('');
    });
    
    return previews.join('\n');
}