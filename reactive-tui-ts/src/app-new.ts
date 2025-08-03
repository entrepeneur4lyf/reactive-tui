/**
 * Application wrapper for Reactive TUI TypeScript examples
 * 
 * This module provides a simplified interface for creating TUI applications
 * using the reactive-tui npm package with TypeScript.
 */

import { JsTuiApp, JsElement } from 'reactive-tui'

/**
 * Simplified TUI App interface for TypeScript examples
 */
export interface TuiApp {
  start(): string
  setTitle(title: string): void
  loadCSS(css: string): void
  setComponent(element: JsElement): void
}

/**
 * Implementation using the reactive-tui npm package
 */
class TuiAppImpl implements TuiApp {
  private app: JsTuiApp
  
  constructor() {
    this.app = new JsTuiApp()
  }
  
  start(): string {
    return this.app.start()
  }
  
  setTitle(title: string): void {
    this.app.setTitle(title)
  }
  
  loadCSS(css: string): void {
    this.app.loadCss(css)
  }
  
  setComponent(element: JsElement): void {
    this.app.setComponent(element)
  }
}

/**
 * Create a new TUI application using reactive-tui
 */
export function createApp(): TuiApp {
  return new TuiAppImpl()
}
