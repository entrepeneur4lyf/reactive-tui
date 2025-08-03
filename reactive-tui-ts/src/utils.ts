/**
 * Utility functions for Reactive TUI TypeScript applications
 */

export * from './typescript-examples'

/**
 * Common CSS patterns for terminal interfaces
 */
export const CSS_PATTERNS = {
  /**
   * Modern dark theme base styles
   */
  DARK_THEME: `
    :root {
      --color-bg-primary: #0d1117;
      --color-bg-secondary: #161b22;
      --color-bg-tertiary: #21262d;
      --color-text-primary: #f0f6fc;
      --color-text-secondary: #8b949e;
      --color-text-muted: #6e7681;
      --color-border: #30363d;
      --color-accent: #58a6ff;
      --color-success: #238636;
      --color-warning: #d29922;
      --color-error: #f85149;
    }

    * {
      box-sizing: border-box;
    }

    .app-container {
      background: var(--color-bg-primary);
      color: var(--color-text-primary);
      min-height: 100vh;
      font-family: 'SF Mono', 'Monaco', 'Inconsolata', monospace;
    }
  `,

  /**
   * Responsive grid system
   */
  GRID_SYSTEM: `
    .grid {
      display: grid;
      gap: 1rem;
    }

    .grid-cols-1 { grid-template-columns: 1fr; }
    .grid-cols-2 { grid-template-columns: repeat(2, 1fr); }
    .grid-cols-3 { grid-template-columns: repeat(3, 1fr); }
    .grid-cols-4 { grid-template-columns: repeat(4, 1fr); }

    @media (max-width: 80) {
      .grid-cols-2,
      .grid-cols-3,
      .grid-cols-4 {
        grid-template-columns: 1fr;
      }
    }
  `,

  /**
   * Flexbox utilities
   */
  FLEX_UTILITIES: `
    .flex { display: flex; }
    .flex-col { flex-direction: column; }
    .flex-row { flex-direction: row; }
    .justify-start { justify-content: flex-start; }
    .justify-center { justify-content: center; }
    .justify-end { justify-content: flex-end; }
    .justify-between { justify-content: space-between; }
    .items-start { align-items: flex-start; }
    .items-center { align-items: center; }
    .items-end { align-items: flex-end; }
    .items-stretch { align-items: stretch; }
  `,

  /**
   * Component styles
   */
  COMPONENTS: `
    .card {
      background: var(--color-bg-secondary);
      border: 1px solid var(--color-border);
      border-radius: 8px;
      padding: 1.5rem;
      transition: all 0.2s ease;
    }

    .card:hover {
      border-color: var(--color-accent);
    }

    .btn {
      padding: 0.75rem 1.5rem;
      border-radius: 6px;
      font-weight: 500;
      cursor: pointer;
      transition: all 0.2s ease;
      text-align: center;
      border: 1px solid transparent;
    }

    .btn-primary {
      background: var(--color-success);
      color: white;
      border-color: var(--color-success);
    }

    .btn-secondary {
      background: var(--color-bg-tertiary);
      color: var(--color-text-primary);
      border-color: var(--color-border);
    }

    .btn-danger {
      background: var(--color-error);
      color: white;
      border-color: var(--color-error);
    }
  `,

  /**
   * Spacing utilities
   */
  SPACING: `
    .p-1 { padding: 0.25rem; }
    .p-2 { padding: 0.5rem; }
    .p-3 { padding: 0.75rem; }
    .p-4 { padding: 1rem; }
    .p-6 { padding: 1.5rem; }
    .p-8 { padding: 2rem; }

    .m-1 { margin: 0.25rem; }
    .m-2 { margin: 0.5rem; }
    .m-3 { margin: 0.75rem; }
    .m-4 { margin: 1rem; }
    .m-6 { margin: 1.5rem; }
    .m-8 { margin: 2rem; }

    .gap-1 { gap: 0.25rem; }
    .gap-2 { gap: 0.5rem; }
    .gap-3 { gap: 0.75rem; }
    .gap-4 { gap: 1rem; }
    .gap-6 { gap: 1.5rem; }
    .gap-8 { gap: 2rem; }
  `
}

/**
 * Common layout patterns
 */
export const LAYOUT_PATTERNS = {
  /**
   * Dashboard layout with header, sidebar, and main content
   */
  DASHBOARD: `
    .dashboard {
      display: grid;
      grid-template-areas: 
        "header header"
        "sidebar main";
      grid-template-rows: auto 1fr;
      grid-template-columns: 250px 1fr;
      height: 100vh;
    }

    .dashboard-header { grid-area: header; }
    .dashboard-sidebar { grid-area: sidebar; }
    .dashboard-main { grid-area: main; }

    @media (max-width: 100) {
      .dashboard {
        grid-template-areas: 
          "header"
          "main";
        grid-template-columns: 1fr;
      }
      .dashboard-sidebar { display: none; }
    }
  `,

  /**
   * Centered content layout
   */
  CENTERED: `
    .centered {
      display: flex;
      justify-content: center;
      align-items: center;
      min-height: 100vh;
    }

    .centered-content {
      max-width: 600px;
      width: 100%;
      padding: 2rem;
    }
  `,

  /**
   * Two-column layout
   */
  TWO_COLUMN: `
    .two-column {
      display: grid;
      grid-template-columns: 1fr 1fr;
      gap: 2rem;
    }

    @media (max-width: 80) {
      .two-column {
        grid-template-columns: 1fr;
      }
    }
  `
}

/**
 * Animation utilities
 */
export const ANIMATIONS = {
  /**
   * Fade in animation
   */
  FADE_IN: `
    @keyframes fadeIn {
      from { opacity: 0; }
      to { opacity: 1; }
    }

    .fade-in {
      animation: fadeIn 0.3s ease-in-out;
    }
  `,

  /**
   * Slide in animation
   */
  SLIDE_IN: `
    @keyframes slideIn {
      from { transform: translateY(-10px); opacity: 0; }
      to { transform: translateY(0); opacity: 1; }
    }

    .slide-in {
      animation: slideIn 0.3s ease-out;
    }
  `,

  /**
   * Pulse animation for loading states
   */
  PULSE: `
    @keyframes pulse {
      0%, 100% { opacity: 1; }
      50% { opacity: 0.5; }
    }

    .pulse {
      animation: pulse 2s infinite;
    }
  `
}

/**
 * Utility function to combine CSS patterns
 */
export function combineCSS(...patterns: string[]): string {
  return patterns.join('\n\n')
}

/**
 * Generate a complete CSS theme
 */
export function generateCompleteTheme(): string {
  return combineCSS(
    CSS_PATTERNS.DARK_THEME,
    CSS_PATTERNS.GRID_SYSTEM,
    CSS_PATTERNS.FLEX_UTILITIES,
    CSS_PATTERNS.COMPONENTS,
    CSS_PATTERNS.SPACING,
    LAYOUT_PATTERNS.DASHBOARD,
    LAYOUT_PATTERNS.CENTERED,
    LAYOUT_PATTERNS.TWO_COLUMN,
    ANIMATIONS.FADE_IN,
    ANIMATIONS.SLIDE_IN,
    ANIMATIONS.PULSE
  )
}

/**
 * Terminal size utilities
 */
export const TERMINAL_UTILS = {
  /**
   * Check if terminal is wide enough for desktop layout
   */
  isWideScreen(): boolean {
    const [width] = require('reactive-tui').TuiUtils.getTerminalSize()
    return width >= 120
  },

  /**
   * Check if terminal is narrow (mobile-like)
   */
  isNarrowScreen(): boolean {
    const [width] = require('reactive-tui').TuiUtils.getTerminalSize()
    return width < 80
  },

  /**
   * Get responsive breakpoint
   */
  getBreakpoint(): 'small' | 'medium' | 'large' {
    const [width] = require('reactive-tui').TuiUtils.getTerminalSize()
    if (width < 80) return 'small'
    if (width < 120) return 'medium'
    return 'large'
  }
}

/**
 * Common keyboard shortcuts
 */
export const KEYBOARD_SHORTCUTS = {
  QUIT: 'q',
  ESCAPE: '\u001b',
  ENTER: '\r',
  TAB: '\t',
  ARROW_UP: '\u001b[A',
  ARROW_DOWN: '\u001b[B',
  ARROW_LEFT: '\u001b[D',
  ARROW_RIGHT: '\u001b[C',
  CTRL_C: '\u0003',
  CTRL_D: '\u0004'
}

/**
 * Color palette for consistent theming
 */
export const COLOR_PALETTE = {
  // GitHub-inspired colors
  GRAY: {
    50: '#f6f8fa',
    100: '#eaeef2',
    200: '#d0d7de',
    300: '#afb8c1',
    400: '#8c959f',
    500: '#6e7681',
    600: '#57606a',
    700: '#424a53',
    800: '#32383f',
    900: '#24292f'
  },
  BLUE: {
    50: '#dbeafe',
    100: '#c3ddfd',
    200: '#a4cafe',
    300: '#79a6f7',
    400: '#58a6ff',
    500: '#388bfd',
    600: '#1f6feb',
    700: '#1158c7',
    800: '#0d419d',
    900: '#0c2d6b'
  },
  GREEN: {
    50: '#dcfce7',
    100: '#bbf7d0',
    200: '#86efac',
    300: '#4ade80',
    400: '#22c55e',
    500: '#238636',
    600: '#16a34a',
    700: '#15803d',
    800: '#166534',
    900: '#14532d'
  },
  RED: {
    50: '#fef2f2',
    100: '#fee2e2',
    200: '#fecaca',
    300: '#fca5a5',
    400: '#f87171',
    500: '#f85149',
    600: '#dc2626',
    700: '#b91c1c',
    800: '#991b1b',
    900: '#7f1d1d'
  },
  YELLOW: {
    50: '#fefce8',
    100: '#fef3c7',
    200: '#fde68a',
    300: '#fcd34d',
    400: '#d29922',
    500: '#eab308',
    600: '#ca8a04',
    700: '#a16207',
    800: '#854d0e',
    900: '#713f12'
  }
}
