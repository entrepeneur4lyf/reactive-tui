/**
 * TypeScript Examples and Utilities for Reactive TUI
 * 
 * This module provides TypeScript-specific utilities, examples, and helper functions
 * that demonstrate best practices for using Reactive TUI with full type safety.
 */

import { 
  JsTuiApp, 
  JsElement, 
  TuiUtils, 
  JsToast, 
  JsToastManager,
  JsColorTheme,
  JsColorDefinition 
} from 'reactive-tui'

/**
 * TypeScript utility functions for common TUI operations
 */
export class TypeScriptTuiUtils {
  /**
   * Create a type-safe button with proper event handling
   */
  static createButton(
    text: string, 
    variant: 'primary' | 'secondary' | 'danger' = 'primary',
    onClick?: () => void
  ): JsElement {
    const button = TuiUtils.button()
    button.addClass(`btn btn-${variant}`)
    button.setContent(text)
    button.makeFocusable()
    
    if (onClick) {
      button.setAttribute('data-click-handler', 'true')
      // Note: Actual click handling would be implemented in the application layer
    }
    
    return button
  }

  /**
   * Create a card component with TypeScript typing
   */
  static createCard(title: string, content: string, className?: string): JsElement {
    const card = TuiUtils.div()
    card.addClass('card')
    if (className) {
      card.addClass(className)
    }

    const cardTitle = TuiUtils.div()
    cardTitle.addClass('card-title')
    cardTitle.setContent(title)

    const cardContent = TuiUtils.div()
    cardContent.addClass('card-content')
    cardContent.setContent(content)

    card.addChild(cardTitle)
    card.addChild(cardContent)

    return card
  }

  /**
   * Create a status indicator with type safety
   */
  static createStatusIndicator(
    label: string, 
    value: string | number, 
    status: 'success' | 'warning' | 'error' | 'info' = 'info'
  ): JsElement {
    const container = TuiUtils.div()
    container.addClass(`status-indicator status-${status}`)

    const labelElement = TuiUtils.div()
    labelElement.addClass('status-label')
    labelElement.setContent(label)

    const valueElement = TuiUtils.div()
    valueElement.addClass('status-value')
    valueElement.setContent(String(value))

    container.addChild(labelElement)
    container.addChild(valueElement)

    return container
  }

  /**
   * Create a responsive grid container
   */
  static createGrid(columns: number, gap: string = '1rem'): JsElement {
    const grid = TuiUtils.div()
    grid.addClass('grid-container')
    grid.setAttribute('data-columns', String(columns))
    grid.setAttribute('data-gap', gap)
    
    return grid
  }

  /**
   * Create a flex container with TypeScript options
   */
  static createFlex(
    direction: 'row' | 'column' = 'row',
    justify: 'start' | 'center' | 'end' | 'space-between' | 'space-around' = 'start',
    align: 'start' | 'center' | 'end' | 'stretch' = 'start'
  ): JsElement {
    const flex = TuiUtils.div()
    flex.addClass('flex-container')
    flex.setAttribute('data-direction', direction)
    flex.setAttribute('data-justify', justify)
    flex.setAttribute('data-align', align)
    
    return flex
  }
}

/**
 * TypeScript-safe theme utilities
 */
export class ThemeUtils {
  /**
   * Get a theme with type safety
   */
  static getTheme(name: 'dark' | 'light' | 'terminal'): JsColorTheme {
    switch (name) {
      case 'dark':
        return JsColorTheme.dark()
      case 'light':
        return JsColorTheme.light()
      case 'terminal':
        return JsColorTheme.terminal()
      default:
        return JsColorTheme.dark()
    }
  }

  /**
   * Create a custom color with validation
   */
  static createColor(color: string | [number, number, number]): JsColorDefinition {
    if (typeof color === 'string') {
      if (color.startsWith('#')) {
        return JsColorDefinition.hex(color)
      }
      throw new Error('String colors must be hex format (e.g., "#ff0000")')
    } else {
      const [r, g, b] = color
      if (r < 0 || r > 255 || g < 0 || g > 255 || b < 0 || b > 255) {
        throw new Error('RGB values must be between 0 and 255')
      }
      return JsColorDefinition.rgb(r, g, b)
    }
  }

  /**
   * Generate CSS variables from theme
   */
  static generateCSSVariables(theme: JsColorTheme): string {
    const themeName = theme.getName()
    return `
      :root {
        --theme-name: "${themeName}";
        --color-primary: ${theme.getSemanticColor('primary')};
        --color-secondary: ${theme.getSemanticColor('secondary')};
        --color-success: ${theme.getSemanticColor('success')};
        --color-warning: ${theme.getSemanticColor('warning')};
        --color-error: ${theme.getSemanticColor('error')};
        --bg-primary: ${theme.getSemanticBackground('primary')};
        --bg-secondary: ${theme.getSemanticBackground('secondary')};
      }
    `
  }
}

/**
 * Toast notification utilities with TypeScript
 */
export class ToastUtils {
  private static manager: JsToastManager | null = null

  /**
   * Initialize the toast manager
   */
  static initialize(): void {
    const [width, height] = TuiUtils.getTerminalSize()
    this.manager = new JsToastManager(width, height)
  }

  /**
   * Show a typed toast notification
   */
  static show(
    type: 'info' | 'success' | 'warning' | 'error',
    message: string,
    title?: string,
    duration: number = 3000
  ): void {
    if (!this.manager) {
      this.initialize()
    }

    let toast: JsToast
    switch (type) {
      case 'info':
        toast = JsToast.info(message)
        break
      case 'success':
        toast = JsToast.success(message)
        break
      case 'warning':
        toast = JsToast.warning(message)
        break
      case 'error':
        toast = JsToast.error(message)
        break
    }

    if (title) {
      toast.setTitle(title)
    }
    toast.setDuration(duration)

    this.manager!.showToast(toast)
  }

  /**
   * Convenience methods for different toast types
   */
  static info(message: string, title?: string): void {
    this.show('info', message, title)
  }

  static success(message: string, title?: string): void {
    this.show('success', message, title)
  }

  static warning(message: string, title?: string): void {
    this.show('warning', message, title)
  }

  static error(message: string, title?: string): void {
    this.show('error', message, title)
  }
}

/**
 * Application builder with TypeScript patterns
 */
export class TuiAppBuilder {
  private app: JsTuiApp
  private rootElement: JsElement | null = null

  constructor(title?: string) {
    this.app = new JsTuiApp()
    if (title) {
      this.app.setTitle(title)
    }
  }

  /**
   * Set the application title
   */
  setTitle(title: string): this {
    this.app.setTitle(title)
    return this
  }

  /**
   * Load CSS with validation
   */
  loadCSS(css: string): this {
    const errors = TuiUtils.validateCss(css)
    if (errors.length > 0) {
      console.warn('CSS validation warnings:', errors)
    }
    this.app.loadCss(css)
    return this
  }

  /**
   * Set the root component
   */
  setRoot(element: JsElement): this {
    this.rootElement = element
    this.app.setComponent(element)
    return this
  }

  /**
   * Add a child to the root element
   */
  addChild(element: JsElement): this {
    if (!this.rootElement) {
      this.rootElement = TuiUtils.div()
      this.rootElement.addClass('app-root')
      this.app.setComponent(this.rootElement)
    }
    this.rootElement.addChild(element)
    return this
  }

  /**
   * Start the application
   */
  start(): string {
    if (!this.rootElement) {
      throw new Error('No root element set. Use setRoot() or addChild() before starting.')
    }
    return this.app.start()
  }

  /**
   * Get the underlying JsTuiApp instance
   */
  getApp(): JsTuiApp {
    return this.app
  }
}

/**
 * Validation utilities for TypeScript
 */
export class ValidationUtils {
  /**
   * Validate terminal size requirements
   */
  static validateTerminalSize(minWidth: number = 80, minHeight: number = 24): boolean {
    const [width, height] = TuiUtils.getTerminalSize()
    return width >= minWidth && height >= minHeight
  }

  /**
   * Validate CSS with detailed error reporting
   */
  static validateCSS(css: string): { isValid: boolean; errors: string[] } {
    const errors = TuiUtils.validateCss(css)
    return {
      isValid: errors.length === 0,
      errors
    }
  }

  /**
   * Validate color format
   */
  static isValidHexColor(color: string): boolean {
    return /^#([A-Fa-f0-9]{6}|[A-Fa-f0-9]{3})$/.test(color)
  }

  /**
   * Validate RGB values
   */
  static isValidRGB(r: number, g: number, b: number): boolean {
    return [r, g, b].every(value => 
      Number.isInteger(value) && value >= 0 && value <= 255
    )
  }
}

/**
 * Performance utilities for TypeScript applications
 */
export class PerformanceUtils {
  private static timers: Map<string, number> = new Map()

  /**
   * Start a performance timer
   */
  static startTimer(name: string): void {
    this.timers.set(name, Date.now())
  }

  /**
   * End a performance timer and return duration
   */
  static endTimer(name: string): number {
    const startTime = this.timers.get(name)
    if (!startTime) {
      throw new Error(`Timer "${name}" was not started`)
    }
    const duration = Date.now() - startTime
    this.timers.delete(name)
    return duration
  }

  /**
   * Measure function execution time
   */
  static measure<T>(name: string, fn: () => T): { result: T; duration: number } {
    this.startTimer(name)
    const result = fn()
    const duration = this.endTimer(name)
    return { result, duration }
  }

  /**
   * Get terminal performance info
   */
  static getTerminalInfo(): { width: number; height: number; area: number } {
    const [width, height] = TuiUtils.getTerminalSize()
    return { width, height, area: width * height }
  }
}
