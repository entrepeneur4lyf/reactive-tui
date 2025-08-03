/**
 * Advanced Layout Engine for CSS-styled TUI
 * 
 * Handles flexible positioning, responsive design, and CSS-like layouts
 */

import type { Element } from './types'

export interface LayoutRect {
  x: number
  y: number
  width: number
  height: number
}

export interface LayoutNode {
  element: Element
  rect: LayoutRect
  children: LayoutNode[]
  computedStyle: ComputedStyle
}

export interface ComputedStyle {
  display: 'block' | 'inline' | 'flex' | 'grid' | 'none'
  position: 'static' | 'relative' | 'absolute' | 'fixed'
  flexDirection: 'row' | 'column'
  justifyContent: 'flex-start' | 'center' | 'flex-end' | 'space-between' | 'space-around'
  alignItems: 'flex-start' | 'center' | 'flex-end' | 'stretch'
  width: number | 'auto' | string
  height: number | 'auto' | string
  margin: { top: number, right: number, bottom: number, left: number }
  padding: { top: number, right: number, bottom: number, left: number }
  minWidth: number
  minHeight: number
  maxWidth: number
  maxHeight: number
}

export interface ViewportInfo {
  width: number
  height: number
  terminalSize: { width: number, height: number }
}

export class LayoutEngine {
  private viewport: ViewportInfo

  constructor(viewport?: ViewportInfo) {
    // Default to modern high-resolution display capabilities
    // 4K laptops: 320x180, MacBook Pro M3: 430x280, 4K Desktop: 480x270
    // Even 5-year-old laptops commonly have 4K screens now
    // Use a default that reflects actual modern hardware capabilities
    this.viewport = viewport || {
      width: 400,
      height: 200,
      terminalSize: { width: 400, height: 200 }
    }
  }

  updateViewport(viewport: ViewportInfo): void {
    this.viewport = viewport
  }

  /**
   * Compute layout for ResponsiveWidget - integrates with widget size constraints
   */
  computeResponsiveLayout(widget: import('./types').ResponsiveWidget, parentRect?: LayoutRect): LayoutNode {
    const element = widget.toElement()
    const [minWidth, minHeight] = widget.minSize()
    const [maxWidth, maxHeight] = widget.maxSize()

    // Compute available space
    const availableWidth = parentRect?.width || this.viewport.width
    const availableHeight = parentRect?.height || this.viewport.height

    // Calculate optimal size based on constraints
    let width = Math.max(minWidth, Math.min(maxWidth || availableWidth, availableWidth))
    let height = Math.max(minHeight, Math.min(maxHeight || availableHeight, availableHeight))

    // Apply growth constraints
    if (!widget.canGrowHorizontal()) {
      width = minWidth
    }
    if (!widget.canGrowVertical()) {
      height = minHeight
    }

    const rect: LayoutRect = {
      x: parentRect?.x || 0,
      y: parentRect?.y || 0,
      width,
      height
    }

    const computedStyle = this.computeStyle(element)

    return {
      element,
      rect,
      children: [],
      computedStyle
    }
  }
  
  computeLayout(element: Element): LayoutNode {
    const computedStyle = this.computeStyle(element)
    const rect = this.computeRect(element, computedStyle, this.viewport)
    
    const layoutNode: LayoutNode = {
      element,
      rect,
      children: [],
      computedStyle
    }
    
    // Recursively compute layout for children
    if (element.children && element.children.length > 0) {
      layoutNode.children = this.layoutChildren(element.children, layoutNode)
    }
    
    return layoutNode
  }
  
  private computeStyle(element: Element): ComputedStyle {
    const classes = element.classes || []
    
    // Default style
    const style: ComputedStyle = {
      display: 'block',
      position: 'static',
      flexDirection: 'column',
      justifyContent: 'flex-start',
      alignItems: 'flex-start',
      width: 'auto',
      height: 'auto',
      margin: { top: 0, right: 0, bottom: 0, left: 0 },
      padding: { top: 0, right: 0, bottom: 0, left: 0 },
      minWidth: 0,
      minHeight: 0,
      maxWidth: this.viewport.width,
      maxHeight: this.viewport.height
    }
    
    // Apply CSS class-based styling
    for (const className of classes) {
      this.applyCSSClass(className, style)
    }
    
    // Element-specific defaults
    if (element.tag === 'button') {
      style.display = 'inline'
      style.padding = { top: 0, right: 1, bottom: 0, left: 1 }
    } else if (element.tag === 'input') {
      style.display = 'inline'
      style.width = 22 // [content] with brackets
      style.height = 1
    } else if (element.tag === 'progress') {
      style.width = 35 // label + bar + percentage
      style.height = 1
    }
    
    return style
  }
  
  private applyCSSClass(className: string, style: ComputedStyle): void {
    // Tailwind-inspired utility classes for terminal UI
    
    // === DISPLAY ===
    if (className === 'block') style.display = 'block'
    else if (className === 'inline') style.display = 'inline'
    else if (className === 'flex') style.display = 'flex'
    else if (className === 'grid') style.display = 'grid'
    else if (className === 'hidden') style.display = 'none'
    
    // === FLEX DIRECTION ===
    else if (className === 'flex-row') { style.display = 'flex'; style.flexDirection = 'row' }
    else if (className === 'flex-col') { style.display = 'flex'; style.flexDirection = 'column' }
    
    // === JUSTIFY CONTENT (main axis) ===
    else if (className === 'justify-start') style.justifyContent = 'flex-start'
    else if (className === 'justify-center') style.justifyContent = 'center'
    else if (className === 'justify-end') style.justifyContent = 'flex-end'
    else if (className === 'justify-between') style.justifyContent = 'space-between'
    else if (className === 'justify-around') style.justifyContent = 'space-around'
    
    // === ALIGN ITEMS (cross axis) ===
    else if (className === 'items-start') style.alignItems = 'flex-start'
    else if (className === 'items-center') style.alignItems = 'center'
    else if (className === 'items-end') style.alignItems = 'flex-end'
    else if (className === 'items-stretch') style.alignItems = 'stretch'
    
    // === POSITION ===
    else if (className === 'static') style.position = 'static'
    else if (className === 'relative') style.position = 'relative'
    else if (className === 'absolute') style.position = 'absolute'
    else if (className === 'fixed') style.position = 'fixed'
    
    // === WIDTH (w-*) ===
    else if (className === 'w-auto') style.width = 'auto'
    else if (className === 'w-full') style.width = this.viewport.width
    else if (className === 'w-1/2') style.width = Math.floor(this.viewport.width / 2)
    else if (className === 'w-1/3') style.width = Math.floor(this.viewport.width / 3)
    else if (className === 'w-2/3') style.width = Math.floor(this.viewport.width * 2 / 3)
    else if (className === 'w-1/4') style.width = Math.floor(this.viewport.width / 4)
    else if (className === 'w-3/4') style.width = Math.floor(this.viewport.width * 3 / 4)
    // Numeric widths
    else if (className.startsWith('w-') && !isNaN(parseInt(className.slice(2)))) {
      style.width = parseInt(className.slice(2))
    }
    
    // === HEIGHT (h-*) ===
    else if (className === 'h-auto') style.height = 'auto'
    else if (className === 'h-full') style.height = this.viewport.height
    else if (className === 'h-screen') style.height = this.viewport.height
    else if (className === 'h-1/2') style.height = Math.floor(this.viewport.height / 2)
    else if (className === 'h-1/3') style.height = Math.floor(this.viewport.height / 3)
    else if (className === 'h-2/3') style.height = Math.floor(this.viewport.height * 2 / 3)
    else if (className === 'h-1/4') style.height = Math.floor(this.viewport.height / 4)
    else if (className === 'h-3/4') style.height = Math.floor(this.viewport.height * 3 / 4)
    // Numeric heights
    else if (className.startsWith('h-') && !isNaN(parseInt(className.slice(2)))) {
      style.height = parseInt(className.slice(2))
    }
    
    // === MARGIN (m-*, mx-*, my-*, mt-*, mr-*, mb-*, ml-*) ===
    else if (className === 'm-0') style.margin = { top: 0, right: 0, bottom: 0, left: 0 }
    else if (className === 'm-1') style.margin = { top: 1, right: 1, bottom: 1, left: 1 }
    else if (className === 'm-2') style.margin = { top: 2, right: 2, bottom: 2, left: 2 }
    else if (className === 'm-3') style.margin = { top: 3, right: 3, bottom: 3, left: 3 }
    else if (className === 'm-4') style.margin = { top: 4, right: 4, bottom: 4, left: 4 }
    // Margin X (horizontal)
    else if (className === 'mx-1') { style.margin.left = 1; style.margin.right = 1 }
    else if (className === 'mx-2') { style.margin.left = 2; style.margin.right = 2 }
    else if (className === 'mx-auto') {
      const autoMargin = Math.floor((this.viewport.width - (typeof style.width === 'number' ? style.width : 10)) / 2)
      style.margin.left = autoMargin; style.margin.right = autoMargin
    }
    // Margin Y (vertical)
    else if (className === 'my-1') { style.margin.top = 1; style.margin.bottom = 1 }
    else if (className === 'my-2') { style.margin.top = 2; style.margin.bottom = 2 }
    // Individual margins
    else if (className === 'mt-1') style.margin.top = 1
    else if (className === 'mt-2') style.margin.top = 2
    else if (className === 'mr-1') style.margin.right = 1
    else if (className === 'mr-2') style.margin.right = 2
    else if (className === 'mb-1') style.margin.bottom = 1
    else if (className === 'mb-2') style.margin.bottom = 2
    else if (className === 'ml-1') style.margin.left = 1
    else if (className === 'ml-2') style.margin.left = 2
    
    // === PADDING (p-*, px-*, py-*, pt-*, pr-*, pb-*, pl-*) ===
    else if (className === 'p-0') style.padding = { top: 0, right: 0, bottom: 0, left: 0 }
    else if (className === 'p-1') style.padding = { top: 1, right: 1, bottom: 1, left: 1 }
    else if (className === 'p-2') style.padding = { top: 2, right: 2, bottom: 2, left: 2 }
    else if (className === 'p-3') style.padding = { top: 3, right: 3, bottom: 3, left: 3 }
    else if (className === 'p-4') style.padding = { top: 4, right: 4, bottom: 4, left: 4 }
    // Padding X (horizontal)
    else if (className === 'px-1') { style.padding.left = 1; style.padding.right = 1 }
    else if (className === 'px-2') { style.padding.left = 2; style.padding.right = 2 }
    // Padding Y (vertical)
    else if (className === 'py-1') { style.padding.top = 1; style.padding.bottom = 1 }
    else if (className === 'py-2') { style.padding.top = 2; style.padding.bottom = 2 }
    // Individual paddings
    else if (className === 'pt-1') style.padding.top = 1
    else if (className === 'pt-2') style.padding.top = 2
    else if (className === 'pr-1') style.padding.right = 1
    else if (className === 'pr-2') style.padding.right = 2
    else if (className === 'pb-1') style.padding.bottom = 1
    else if (className === 'pb-2') style.padding.bottom = 2
    else if (className === 'pl-1') style.padding.left = 1
    else if (className === 'pl-2') style.padding.left = 2
    
    // === COMMON PATTERNS ===
    else if (className === 'container') {
      style.width = this.viewport.width
      style.margin.left = 0
      style.margin.right = 0
    }
    else if (className === 'center') {
      style.display = 'flex'
      style.justifyContent = 'center'
      style.alignItems = 'center'
    }
    
    // === GRID LAYOUT CLASSES (matching Rust backend) ===
    // Grid container columns
    else if (className === 'grid-cols-1') { style.display = 'grid'; (style as any).gridCols = 1 }
    else if (className === 'grid-cols-2') { style.display = 'grid'; (style as any).gridCols = 2 }
    else if (className === 'grid-cols-3') { style.display = 'grid'; (style as any).gridCols = 3 }
    else if (className === 'grid-cols-4') { style.display = 'grid'; (style as any).gridCols = 4 }
    else if (className === 'grid-cols-5') { style.display = 'grid'; (style as any).gridCols = 5 }
    else if (className === 'grid-cols-6') { style.display = 'grid'; (style as any).gridCols = 6 }
    else if (className === 'grid-cols-7') { style.display = 'grid'; (style as any).gridCols = 7 }
    else if (className === 'grid-cols-8') { style.display = 'grid'; (style as any).gridCols = 8 }
    else if (className === 'grid-cols-9') { style.display = 'grid'; (style as any).gridCols = 9 }
    else if (className === 'grid-cols-10') { style.display = 'grid'; (style as any).gridCols = 10 }
    else if (className === 'grid-cols-11') { style.display = 'grid'; (style as any).gridCols = 11 }
    else if (className === 'grid-cols-12') { style.display = 'grid'; (style as any).gridCols = 12 }
    else if (className === 'grid-cols-auto') { style.display = 'grid'; (style as any).gridCols = 'auto' }
    
    // Grid container rows
    else if (className === 'grid-rows-1') { style.display = 'grid'; (style as any).gridRows = 1 }
    else if (className === 'grid-rows-2') { style.display = 'grid'; (style as any).gridRows = 2 }
    else if (className === 'grid-rows-3') { style.display = 'grid'; (style as any).gridRows = 3 }
    else if (className === 'grid-rows-4') { style.display = 'grid'; (style as any).gridRows = 4 }
    else if (className === 'grid-rows-5') { style.display = 'grid'; (style as any).gridRows = 5 }
    else if (className === 'grid-rows-6') { style.display = 'grid'; (style as any).gridRows = 6 }
    else if (className === 'grid-rows-auto') { style.display = 'grid'; (style as any).gridRows = 'auto' }
    
    // Grid gaps
    else if (className === 'gap-0') { (style as any).gridGap = { x: 0, y: 0 } }
    else if (className === 'gap-1') { (style as any).gridGap = { x: 1, y: 1 } }
    else if (className === 'gap-2') { (style as any).gridGap = { x: 2, y: 2 } }
    else if (className === 'gap-3') { (style as any).gridGap = { x: 3, y: 3 } }
    else if (className === 'gap-4') { (style as any).gridGap = { x: 4, y: 4 } }
    else if (className === 'gap-5') { (style as any).gridGap = { x: 5, y: 5 } }
    else if (className === 'gap-6') { (style as any).gridGap = { x: 6, y: 6 } }
    else if (className === 'gap-8') { (style as any).gridGap = { x: 8, y: 8 } }
    
    // Grid X gaps
    else if (className === 'gap-x-0') { (style as any).gridGap = { x: 0, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-1') { (style as any).gridGap = { x: 1, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-2') { (style as any).gridGap = { x: 2, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-3') { (style as any).gridGap = { x: 3, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-4') { (style as any).gridGap = { x: 4, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-5') { (style as any).gridGap = { x: 5, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-6') { (style as any).gridGap = { x: 6, y: (style as any).gridGap?.y || 0 } }
    else if (className === 'gap-x-8') { (style as any).gridGap = { x: 8, y: (style as any).gridGap?.y || 0 } }
    
    // Grid Y gaps
    else if (className === 'gap-y-0') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 0 } }
    else if (className === 'gap-y-1') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 1 } }
    else if (className === 'gap-y-2') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 2 } }
    else if (className === 'gap-y-3') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 3 } }
    else if (className === 'gap-y-4') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 4 } }
    else if (className === 'gap-y-5') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 5 } }
    else if (className === 'gap-y-6') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 6 } }
    else if (className === 'gap-y-8') { (style as any).gridGap = { x: (style as any).gridGap?.x || 0, y: 8 } }
    
    // Grid flow
    else if (className === 'grid-flow-row') { (style as any).gridFlow = 'row' }
    else if (className === 'grid-flow-col') { (style as any).gridFlow = 'column' }
    else if (className === 'grid-flow-row-dense') { (style as any).gridFlow = 'row-dense' }
    else if (className === 'grid-flow-col-dense') { (style as any).gridFlow = 'column-dense' }
    
    // Grid item column spans
    else if (className === 'col-span-1') { (style as any).gridColSpan = 1 }
    else if (className === 'col-span-2') { (style as any).gridColSpan = 2 }
    else if (className === 'col-span-3') { (style as any).gridColSpan = 3 }
    else if (className === 'col-span-4') { (style as any).gridColSpan = 4 }
    else if (className === 'col-span-5') { (style as any).gridColSpan = 5 }
    else if (className === 'col-span-6') { (style as any).gridColSpan = 6 }
    else if (className === 'col-span-7') { (style as any).gridColSpan = 7 }
    else if (className === 'col-span-8') { (style as any).gridColSpan = 8 }
    else if (className === 'col-span-9') { (style as any).gridColSpan = 9 }
    else if (className === 'col-span-10') { (style as any).gridColSpan = 10 }
    else if (className === 'col-span-11') { (style as any).gridColSpan = 11 }
    else if (className === 'col-span-12') { (style as any).gridColSpan = 12 }
    else if (className === 'col-span-full') { (style as any).gridColSpan = 'full' }
    
    // Grid item row spans
    else if (className === 'row-span-1') { (style as any).gridRowSpan = 1 }
    else if (className === 'row-span-2') { (style as any).gridRowSpan = 2 }
    else if (className === 'row-span-3') { (style as any).gridRowSpan = 3 }
    else if (className === 'row-span-4') { (style as any).gridRowSpan = 4 }
    else if (className === 'row-span-5') { (style as any).gridRowSpan = 5 }
    else if (className === 'row-span-6') { (style as any).gridRowSpan = 6 }
    else if (className === 'row-span-full') { (style as any).gridRowSpan = 'full' }
    
    // Grid item column positioning
    else if (className === 'col-start-1') { (style as any).gridColStart = 1 }
    else if (className === 'col-start-2') { (style as any).gridColStart = 2 }
    else if (className === 'col-start-3') { (style as any).gridColStart = 3 }
    else if (className === 'col-start-4') { (style as any).gridColStart = 4 }
    else if (className === 'col-start-5') { (style as any).gridColStart = 5 }
    else if (className === 'col-start-6') { (style as any).gridColStart = 6 }
    else if (className === 'col-start-7') { (style as any).gridColStart = 7 }
    else if (className === 'col-start-8') { (style as any).gridColStart = 8 }
    else if (className === 'col-start-9') { (style as any).gridColStart = 9 }
    else if (className === 'col-start-10') { (style as any).gridColStart = 10 }
    else if (className === 'col-start-11') { (style as any).gridColStart = 11 }
    else if (className === 'col-start-12') { (style as any).gridColStart = 12 }
    else if (className === 'col-start-auto') { (style as any).gridColStart = 'auto' }
    
    // Grid item row positioning  
    else if (className === 'row-start-1') { (style as any).gridRowStart = 1 }
    else if (className === 'row-start-2') { (style as any).gridRowStart = 2 }
    else if (className === 'row-start-3') { (style as any).gridRowStart = 3 }
    else if (className === 'row-start-4') { (style as any).gridRowStart = 4 }
    else if (className === 'row-start-5') { (style as any).gridRowStart = 5 }
    else if (className === 'row-start-6') { (style as any).gridRowStart = 6 }
    else if (className === 'row-start-7') { (style as any).gridRowStart = 7 }
    else if (className === 'row-start-auto') { (style as any).gridRowStart = 'auto' }

    // === SEMANTIC LAYOUT HELPERS ===
    else if (className === 'header') {
      style.width = this.viewport.width
      style.height = 3
      style.position = 'relative'
    }
    else if (className === 'footer') {
      style.width = this.viewport.width
      style.height = 3
      style.position = 'absolute'
    }
    else if (className === 'sidebar') {
      style.width = 25
      style.height = this.viewport.height - 6
    }
    else if (className === 'main-content') {
      style.width = this.viewport.width - 25
      style.height = this.viewport.height - 6
    }
  }
  
  private computeRect(element: Element, style: ComputedStyle, viewport: ViewportInfo): LayoutRect {
    let width = typeof style.width === 'number' ? style.width : 
                style.width === 'auto' ? this.calculateAutoWidth(element, style) :
                this.parseSize(style.width, viewport.width)
    
    let height = typeof style.height === 'number' ? style.height :
                 style.height === 'auto' ? this.calculateAutoHeight(element, style) :
                 this.parseSize(style.height, viewport.height)
    
    // Apply constraints
    width = Math.max(style.minWidth, Math.min(width, style.maxWidth))
    height = Math.max(style.minHeight, Math.min(height, style.maxHeight))
    
    return {
      x: 0, // Will be positioned later
      y: 0,
      width,
      height
    }
  }
  
  private layoutChildren(children: Element[], parent: LayoutNode): LayoutNode[] {
    const layoutNodes: LayoutNode[] = []
    const parentStyle = parent.computedStyle
    
    let currentX = parent.rect.x + parentStyle.padding.left
    let currentY = parent.rect.y + parentStyle.padding.top
    
    for (const child of children) {
      const childNode = this.computeLayout(child)
      
      // Position child based on parent's layout
      if (parentStyle.display === 'flex') {
        if (parentStyle.flexDirection === 'row') {
          childNode.rect.x = currentX
          childNode.rect.y = currentY
          currentX += childNode.rect.width + childNode.computedStyle.margin.left + childNode.computedStyle.margin.right
        } else {
          childNode.rect.x = currentX
          childNode.rect.y = currentY
          currentY += childNode.rect.height + childNode.computedStyle.margin.top + childNode.computedStyle.margin.bottom
        }
      } else {
        // Block layout
        childNode.rect.x = currentX
        childNode.rect.y = currentY
        currentY += childNode.rect.height + childNode.computedStyle.margin.top + childNode.computedStyle.margin.bottom
      }
      
      // Apply positioning
      if (childNode.computedStyle.position === 'absolute') {
        // Absolute positioning - remove from flow
        // Position will be set by CSS classes or attributes
      } else if (childNode.computedStyle.position === 'fixed') {
        // Fixed positioning relative to viewport
        childNode.rect.x = 0
        childNode.rect.y = 0
      }
      
      layoutNodes.push(childNode)
    }
    
    // Apply flex justification and alignment
    if (parentStyle.display === 'flex') {
      this.applyFlexAlignment(layoutNodes, parent)
    }
    
    return layoutNodes
  }
  
  private applyFlexAlignment(children: LayoutNode[], parent: LayoutNode): void {
    const parentStyle = parent.computedStyle
    const availableWidth = parent.rect.width - parentStyle.padding.left - parentStyle.padding.right
    const availableHeight = parent.rect.height - parentStyle.padding.top - parentStyle.padding.bottom
    
    if (parentStyle.flexDirection === 'row') {
      // Horizontal alignment
      const totalChildWidth = children.reduce((sum, child) => sum + child.rect.width, 0)
      const remainingSpace = availableWidth - totalChildWidth
      
      switch (parentStyle.justifyContent) {
        case 'center':
          const offset = remainingSpace / 2
          children.forEach(child => child.rect.x += offset)
          break
        case 'flex-end':
          children.forEach(child => child.rect.x += remainingSpace)
          break
        case 'space-between':
          if (children.length > 1) {
            const spacing = remainingSpace / (children.length - 1)
            children.forEach((child, i) => child.rect.x += i * spacing)
          }
          break
        case 'space-around':
          const spacingAround = remainingSpace / children.length
          children.forEach((child, i) => child.rect.x += spacingAround * (i + 0.5))
          break
      }
    } else {
      // Vertical alignment
      const totalChildHeight = children.reduce((sum, child) => sum + child.rect.height, 0)
      const remainingSpace = availableHeight - totalChildHeight
      
      switch (parentStyle.justifyContent) {
        case 'center':
          const offset = remainingSpace / 2
          children.forEach(child => child.rect.y += offset)
          break
        case 'flex-end':
          children.forEach(child => child.rect.y += remainingSpace)
          break
        case 'space-between':
          if (children.length > 1) {
            const spacing = remainingSpace / (children.length - 1)
            children.forEach((child, i) => child.rect.y += i * spacing)
          }
          break
      }
    }
  }
  
  private calculateAutoWidth(element: Element, style: ComputedStyle): number {
    if (element.content) {
      return element.content.length + style.padding.left + style.padding.right
    }
    return 10 // Default width
  }
  
  private calculateAutoHeight(element: Element, style: ComputedStyle): number {
    return 1 + style.padding.top + style.padding.bottom
  }
  
  private parseSize(size: string, containerSize: number): number {
    if (size.endsWith('%')) {
      const percentage = parseInt(size.slice(0, -1))
      return Math.floor(containerSize * percentage / 100)
    }
    return parseInt(size) || containerSize
  }
  
  // Helper method to find element at position (for click/focus)
  findElementAt(layoutNode: LayoutNode, x: number, y: number): LayoutNode | null {
    // Check if point is within this node's bounds
    if (x >= layoutNode.rect.x && x < layoutNode.rect.x + layoutNode.rect.width &&
        y >= layoutNode.rect.y && y < layoutNode.rect.y + layoutNode.rect.height) {
      
      // Check children first (top-to-bottom)
      for (const child of layoutNode.children) {
        const found = this.findElementAt(child, x, y)
        if (found) return found
      }
      
      // Return this node if no child contains the point
      return layoutNode
    }
    
    return null
  }
  
  // Get all focusable elements in layout order
  getFocusableElements(layoutNode: LayoutNode): LayoutNode[] {
    const focusable: LayoutNode[] = []
    
    if (this.isFocusable(layoutNode.element)) {
      focusable.push(layoutNode)
    }
    
    for (const child of layoutNode.children) {
      focusable.push(...this.getFocusableElements(child))
    }
    
    // Sort by position (top-to-bottom, left-to-right)
    return focusable.sort((a, b) => {
      if (a.rect.y !== b.rect.y) return a.rect.y - b.rect.y
      return a.rect.x - b.rect.x
    })
  }
  
  private isFocusable(element: Element): boolean {
    return ['button', 'input', 'checkbox', 'radio'].includes(element.tag)
  }
}