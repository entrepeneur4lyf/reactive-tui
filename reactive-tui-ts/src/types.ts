/**
 * TypeScript type definitions for the TUI framework
 */

// Import auto-generated types from Rust (copied locally to avoid rootDir issues)
export type { Element, ElementKeyBinding, KeyCombination, ElementAction } from './generated-types'
import type { KeyCombination, ElementAction } from './generated-types'

export interface ElementBuilder {
  class(className: string): ElementBuilder
  classes(classes: string[]): ElementBuilder
  id(id: string): ElementBuilder
  attr(key: string, value: string): ElementBuilder
  content(content: string): ElementBuilder
  child(child: import('./generated-types').Element | ElementBuilder): ElementBuilder
  children(children: (import('./generated-types').Element | ElementBuilder)[]): ElementBuilder
  focusable(focusable: boolean): ElementBuilder
  tab_index(index: number): ElementBuilder
  bind_key(key: KeyCombination, action: ElementAction): ElementBuilder
  bind_char(c: string, action: ElementAction): ElementBuilder
  bind_enter(): ElementBuilder
  bind_space(): ElementBuilder
  modal(is_modal: boolean): ElementBuilder
  build(): import('./generated-types').Element
}

export interface Component {
  render(): import('./generated-types').Element
  onMount?(): Promise<void> | void
  onUnmount?(): Promise<void> | void
  onUpdate?(): Promise<void> | void
}

export type ComponentFunction = () => import('./generated-types').Element | ElementBuilder

export interface AppConfig {
  stylesheet?: string
  component: ComponentFunction
  width?: number
  height?: number
  targetFps?: number
}

export interface CSSStyleDeclaration {
  [property: string]: string | number
}

// Responsive Widget Interface - matches Rust ResponsiveWidget trait
export interface ResponsiveWidget {
  /** Convert the widget to an Element for layout computation */
  toElement(): import('./generated-types').Element

  /** Render the widget with a computed layout */
  renderWithLayout(layout: { x: number; y: number; width: number; height: number }, theme?: any): string

  /** Get the widget's preferred minimum size (width, height) */
  minSize(): [number, number]

  /** Get the widget's preferred maximum size (width, height) - null means no maximum */
  maxSize(): [number | null, number | null]

  /** Whether the widget can grow horizontally */
  canGrowHorizontal(): boolean

  /** Whether the widget can grow vertically */
  canGrowVertical(): boolean
}

// Widget size constraints for responsive design
export interface WidgetSizeConstraints {
  minWidth: number
  minHeight: number
  maxWidth?: number
  maxHeight?: number
  canGrowHorizontal: boolean
  canGrowVertical: boolean
}

// Event types are defined in events.ts
