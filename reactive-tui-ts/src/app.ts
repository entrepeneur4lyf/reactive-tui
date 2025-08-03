/**
 * Main application API for Bun/TypeScript
 */

import type { ComponentFunction, AppConfig } from './types'

export interface TuiApp {
  run(): Promise<void>
  stop(): Promise<void>
  updateStylesheet(css: string): Promise<void>
  setComponent(component: ComponentFunction): void
}

class TuiAppImpl implements TuiApp {
  private config: AppConfig
  private rustApp: any // Will be the Rust TuiApp instance via FFI
  
  constructor(config: AppConfig) {
    this.config = config
  }
  
  async run(): Promise<void> {
    // Initialize Rust app via FFI
    // This will be implemented once we have the Rust FFI bindings
    console.log('🚀 Starting CSS-styled TUI app...')
    console.log('📄 Stylesheet:', this.config.stylesheet)
    
    // For now, create a mock implementation
    await this.mockRun()
  }
  
  async stop(): Promise<void> {
    console.log('⏹️  Stopping TUI app...')
  }
  
  async updateStylesheet(_css: string): Promise<void> {
    console.log('🎨 Hot reloading CSS...')
  }
  
  setComponent(component: ComponentFunction): void {
    this.config.component = component
  }

  /**
   * Render a ResponsiveWidget with proper layout computation
   * This demonstrates the integration between widgets and the LayoutEngine
   */
  renderResponsiveWidget(widget: import('./types').ResponsiveWidget, x: number, y: number, availableWidth: number, availableHeight: number): void {
    const { LayoutEngine } = require('./layout')
    const layoutEngine = new LayoutEngine({
      width: availableWidth,
      height: availableHeight,
      terminalSize: { width: process.stdout.columns || 400, height: process.stdout.rows || 200 }
    })

    // Compute responsive layout for the widget
    const layoutNode = layoutEngine.computeResponsiveLayout(widget, {
      x, y, width: availableWidth, height: availableHeight
    })

    // Render the widget with computed layout
    const renderedContent = widget.renderWithLayout(layoutNode.rect)

    // Position cursor and write content
    process.stdout.write(`\x1B[${y + 1};${x + 1}H${renderedContent}`)
  }
  
  // Proper terminal rendering implementation
  private async mockRun(): Promise<void> {
    const componentResult = this.config.component()
    const component = 'build' in componentResult ? componentResult.build() : componentResult
    
    // Set up terminal for TUI rendering
    process.stdout.write('\x1B[2J\x1B[0f') // Clear screen and move cursor to top
    process.stdout.write('\x1B[?25l') // Hide cursor
    
    // Enable raw mode for input handling
    if (process.stdin.setRawMode) {
      process.stdin.setRawMode(true)
    }
    process.stdin.resume()
    
    // Render the component tree to terminal
    this.focusableElements = [] // Reset focusable elements
    this.renderComponentToTerminal(component, 0, 0)
    
    // Minimal status line - just essential info
    process.stdout.write(`\x1B[24;1H\x1B[2m\x1B[90m↑↓ navigate • enter: next • q: quit\x1B[0m`)
    
    // Handle input with navigation
    process.stdin.on('data', (key) => {
      const keyStr = key.toString()
      
      if (keyStr === '\u0003' || keyStr === 'q') { // Ctrl+C or 'q'
        this.cleanup()
        process.exit(0)
      } else if (keyStr === '\u001b[A' || keyStr === 'k') { // Up arrow or 'k'
        this.moveFocus(-1)
      } else if (keyStr === '\u001b[B' || keyStr === 'j') { // Down arrow or 'j'
        this.moveFocus(1)
      } else if (keyStr === '\u001b[C' || keyStr === 'l') { // Right arrow or 'l'
        this.moveFocus(1)
      } else if (keyStr === '\u001b[D' || keyStr === 'h') { // Left arrow or 'h'
        this.moveFocus(-1)
      } else if (keyStr === '\r' || keyStr === ' ') { // Enter or Space
        this.activateFocused()
      } else if (keyStr === '\t') { // Tab
        this.moveFocus(1)
      } else if (keyStr === '\u001b[Z') { // Shift+Tab
        this.moveFocus(-1)
      }
      
      if (this.needsRerender) {
        this.rerenderInterface()
      }
    })
    
    process.on('SIGINT', () => {
      this.cleanup()
      process.exit(0)
    })
    
    // Keep the process alive until user exits
    await new Promise(() => {})
  }
  
  private cleanup(): void {
    process.stdout.write('\x1B[?25h') // Show cursor
    process.stdout.write('\x1B[2J\x1B[0f') // Clear screen
    if (process.stdin.setRawMode) {
      process.stdin.setRawMode(false)
    }
    console.log('\n👋 CSS-styled TUI demo finished!')
  }
  
  private renderComponentToTerminal(element: any, x: number, y: number): void {
    // Move cursor to position
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    // Apply CSS-based styling based on classes with position
    this.applyTerminalStyling(element.classes || [], x, y)
    
    // Render based on element type
    switch (element.tag) {
      case 'text':
        if (element.content) {
          process.stdout.write(element.content)
        }
        break
        
      case 'div':
        this.renderContainer(element, x, y)
        break
        
      case 'button':
        this.renderButton(element, x, y)
        break
        
      case 'input':
        this.renderInput(element, x, y)
        break
        
      case 'progress':
        this.renderProgress(element, x, y)
        break
        
      case 'checkbox':
        this.renderCheckbox(element, x, y)
        break
        
      case 'radio':
        this.renderRadio(element, x, y)
        break
        
      case 'spinner':
        this.renderSpinner(element, x, y)
        break
        
      case 'table':
        this.renderTable(element, x, y)
        break
        
      case 'modal':
        this.renderModal(element, x, y)
        break
        
      case 'slider':
        this.renderSlider(element, x, y)
        break
        
      case 'grid':
        this.renderGrid(element, x, y)
        break
        
      case 'panel':
        this.renderPanel(element, x, y)
        break
        
      case 'bar':
        this.renderBar(element, x, y)
        break
        
      case 'toast':
        this.renderToast(element, x, y)
        break
        
      default:
        // Render children for unknown elements
        this.renderChildren(element, x, y)
        break
    }
    
    // Reset styling
    process.stdout.write('\x1B[0m')
  }
  
  private renderLayers: Array<{element: any, x: number, y: number, layer: number}> = []
  private focusableElements: Array<{element: any, x: number, y: number, id: string}> = []
  private focusedIndex: number = 0
  private needsRerender: boolean = false
  
  private flushLayeredRendering(): void {
    // Sort by layer (z-index) - lower layers render first
    this.renderLayers.sort((a, b) => a.layer - b.layer)
    
    // Render shadow layers first
    for (const item of this.renderLayers) {
      if (item.element.classes?.includes('shadow-layer')) {
        this.renderShadowEffect(item.element, item.x + 1, item.y + 1)
      }
    }
    
    // Then render main elements
    for (const item of this.renderLayers) {
      if (!item.element.classes?.includes('shadow-layer')) {
        this.renderElementDirect(item.element, item.x, item.y)
      }
    }
    
    // Clear layers for next frame
    this.renderLayers = []
  }
  
  private renderElementDirect(element: any, x: number, y: number): void {
    // Move cursor to position
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    // Apply CSS-based styling based on classes with position
    this.applyTerminalStyling(element.classes || [], x, y)
    
    // Render based on element type
    switch (element.tag) {
      case 'text':
        if (element.content) {
          process.stdout.write(element.content)
        }
        break
        
      case 'div':
        this.renderContainer(element, x, y)
        break
        
      case 'button':
        this.renderButton(element, x, y)
        break
        
      default:
        // Render children for unknown elements
        this.renderChildren(element, x, y)
        break
    }
    
    // Reset styling
    process.stdout.write('\x1B[0m')
  }
  
  private renderShadowEffect(element: any, x: number, y: number): void {
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    this.applyTerminalStyling(['shadow-layer'])
    
    // Render shadow version of content
    if (element.content) {
      process.stdout.write(element.content.replace(/./g, '▓'))
    } else if (element.tag === 'button') {
      const content = element.children?.[0]?.content || 'Button'
      process.stdout.write(` ${'▓'.repeat(content.length)} `)
    }
    
    process.stdout.write('\x1B[0m')
  }
  
  private getZIndex(classes: string[]): number {
    if (classes.includes('overlay')) return 100
    if (classes.includes('modal')) return 50
    if (classes.includes('tooltip')) return 30
    if (classes.includes('floating')) return 20
    if (classes.includes('elevated')) return 10
    if (classes.includes('shadow-layer')) return -1
    return 0
  }
  
  private renderContainer(element: any, x: number, y: number): void {
    const classes = element.classes || []
    
    // Handle overlay positioning
    if (classes.includes('overlay') && classes.includes('floating')) {
      if (classes.includes('corner-indicator')) {
        // Position in top-right corner
        const terminalWidth = process.stdout.columns || 400
        x = terminalWidth - 12 // Right side with some margin
        y = 1 // Top
      }
    }
    
    // Check if this is a grid container
    if (classes.some((cls: string) => cls.startsWith('grid-cols-'))) {
      this.renderGrid(element, x, y)
      return
    }
    
    let currentY = y
    
    if (classes.includes('header')) {
      // Header with border
      process.stdout.write(`\x1B[${currentY + 1};${x + 1}H`)
      process.stdout.write('─'.repeat(80)) // Top border
      currentY++
      
      // Render header children
      for (const child of element.children || []) {
        this.renderComponentToTerminal(child, x + 2, currentY)
        currentY++
      }
    } else if (classes.includes('main')) {
      // Main content area - render sidebar and content side by side
      const sidebarChild = element.children?.find((c: any) => c.classes?.includes('sidebar'))
      const contentChild = element.children?.find((c: any) => c.classes?.includes('content'))
      
      if (sidebarChild) {
        this.renderContainer(sidebarChild, x, currentY + 2)
      }
      if (contentChild) {
        this.renderContainer(contentChild, x + 25, currentY + 2)
      }
    } else if (classes.includes('sidebar')) {
      // Sidebar navigation - vertical layout
      for (const child of element.children || []) {
        if (child.tag === 'text' && child.classes?.includes('sidebar-title')) {
          process.stdout.write(`\x1B[${currentY + 1};${x + 1}H`)
          this.applyTerminalStyling(child.classes, x, currentY)
          process.stdout.write(child.content)
          process.stdout.write('\x1B[0m')
          currentY += 2
        } else if (child.tag === 'button') {
          this.renderButton(child, x + 2, currentY)
          currentY += 2
        }
      }
    } else if (classes.includes('content')) {
      // Content area
      for (const child of element.children || []) {
        if (child.classes?.includes('features')) {
          this.renderFeatures(child, x, currentY + 2)
          currentY += 6
        } else {
          this.renderComponentToTerminal(child, x, currentY)
          currentY++
        }
      }
    } else if (classes.includes('footer')) {
      // Footer with border
      currentY = 20 // Fixed position near bottom
      process.stdout.write(`\x1B[${currentY};${x + 1}H`)
      process.stdout.write('─'.repeat(80)) // Bottom border
      currentY++
      for (const child of element.children || []) {
        this.renderComponentToTerminal(child, x + 2, currentY)
      }
    } else {
      // Default container - render children vertically
      for (const child of element.children || []) {
        this.renderComponentToTerminal(child, x, currentY)
        if (child.tag === 'text') currentY++
      }
    }
  }
  
  private renderFeatures(element: any, x: number, y: number): void {
    const cards = element.children || []
    for (let i = 0; i < cards.length; i++) {
      const card = cards[i]
      if (card.classes?.includes('feature-card')) {
        const cardX = x + (i * 26)
        this.renderFeatureCard(card, cardX, y)
      }
    }
  }
  
  private renderButton(element: any, x: number, y: number): void {
    // Register as focusable
    this.addFocusableElement(element, x, y)
    
    const classes = element.classes || []
    const isActive = classes.includes('active')
    const isFocused = this.isFocused(element)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    // Apply button styling with focus indicator
    if (isFocused) {
      process.stdout.write('\x1B[103m\x1B[30m\x1B[1m') // Bright yellow focus indicator
    } else if (isActive) {
      process.stdout.write('\x1B[44m\x1B[97m') // Blue background, white text
    } else {
      process.stdout.write('\x1B[47m\x1B[30m') // Light background, dark text
    }
    
    // Render button content
    const content = element.children?.[0]?.content || 'Button'
    process.stdout.write(` ${content} `)
    
    process.stdout.write('\x1B[0m') // Reset
  }
  
  private renderFeatureCard(element: any, x: number, y: number): void {
    // Card border
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    process.stdout.write('┌' + '─'.repeat(22) + '┐')
    
    let cardY = y + 1
    for (const child of element.children || []) {
      process.stdout.write(`\x1B[${cardY + 1};${x + 1}H`)
      process.stdout.write('│')
      
      if (child.classes?.includes('feature-title')) {
        process.stdout.write('\x1B[96m') // Cyan for title
      } else {
        process.stdout.write('\x1B[37m') // White for description
      }
      
      const content = child.content || ''
      const truncated = content.length > 20 ? content.substring(0, 20) : content.padEnd(20)
      process.stdout.write(` ${truncated} `)
      process.stdout.write('\x1B[0m│')
      cardY++
    }
    
    // Bottom border
    process.stdout.write(`\x1B[${cardY + 1};${x + 1}H`)
    process.stdout.write('└' + '─'.repeat(22) + '┘')
  }
  
  private renderChildren(element: any, x: number, y: number): void {
    let currentY = y
    for (const child of element.children || []) {
      this.renderComponentToTerminal(child, x, currentY)
      if (child.tag === 'text') currentY++
    }
  }
  
  private applyTerminalStyling(classes: string[], x: number = 0, y: number = 0): void {
    // Advanced CSS styling features with gradients, shadows, transparency
    if (classes.includes('linear-gradient-purple')) {
      // Dynamic linear gradient based on position
      const gradientPos = Math.floor((x + y) / 4) % 5
      const colors = [
        '\x1B[48;2;128;0;255m', // Purple
        '\x1B[48;2;148;20;235m', // Purple-Pink
        '\x1B[48;2;168;40;215m', // Pink-Purple
        '\x1B[48;2;188;60;195m', // Light Purple
        '\x1B[48;2;208;80;175m'  // Pink
      ]
      process.stdout.write('\x1B[38;2;255;255;255m' + colors[gradientPos] + '\x1B[1m')
    } else if (classes.includes('radial-gradient-blue')) {
      // Radial gradient effect from center
      const distance = Math.sqrt((x - 40) ** 2 + (y - 12) ** 2)
      const intensity = Math.max(0, 255 - distance * 8)
      process.stdout.write(`\x1B[38;2;255;255;255m\x1B[48;2;0;${Math.floor(intensity/4)};${Math.floor(intensity)}m`)
    } else if (classes.includes('glass-morphism')) {
      // Advanced glass morphism with blur simulation
      process.stdout.write('\x1B[38;2;230;240;255m\x1B[48;2;40;50;70m\x1B[2m') // Semi-transparent blue
      // Add subtle border effect
      if (x % 20 === 0 || y % 8 === 0) {
        process.stdout.write('\x1B[38;2;180;200;255m') // Lighter border
      }
    } else if (classes.includes('drop-shadow')) {
      // Enhanced drop shadow with opacity
      const shadowIntensity = Math.max(20, 80 - (x + y) * 2)
      process.stdout.write(`\x1B[38;2;${shadowIntensity};${shadowIntensity};${shadowIntensity}m\x1B[48;2;0;0;0m\x1B[2m`)
    } else if (classes.includes('box-shadow-inset')) {
      // Inset shadow effect
      const isEdge = x < 2 || y < 1 || x > 76 || y > 22
      if (isEdge) {
        process.stdout.write('\x1B[38;2;80;80;80m\x1B[48;2;10;10;15m\x1B[2m') // Dark inset
      } else {
        process.stdout.write('\x1B[38;2;255;255;255m\x1B[48;2;30;35;45m') // Normal content
      }
    } else if (classes.includes('text-shadow-glow')) {
      // Text with glow shadow effect
      process.stdout.write('\x1B[38;2;255;255;255m\x1B[48;2;0;50;100m\x1B[1m\x1B[3m') // Bold italic with blue glow
    } else if (classes.includes('backdrop-blur')) {
      // Backdrop blur simulation with dithering
      const dither = (x + y) % 3
      const blurColors = [
        '\x1B[48;2;25;30;40m',
        '\x1B[48;2;35;40;50m',
        '\x1B[48;2;45;50;60m'
      ]
      process.stdout.write('\x1B[38;2;200;210;230m' + blurColors[dither] + '\x1B[2m')
    } else if (classes.includes('neon-rainbow')) {
      // Animated rainbow neon effect
      const hue = (x + y + Date.now() / 100) % 360
      const [r, g, b] = this.hslToRgb(hue / 360, 1, 0.5)
      process.stdout.write(`\x1B[38;2;${r};${g};${b}m\x1B[48;2;0;0;0m\x1B[1m\x1B[5m`)
    } else if (classes.includes('gradient-header')) {
      process.stdout.write('\x1B[38;2;255;100;150m\x1B[48;2;50;20;80m\x1B[1m')
    } else if (classes.includes('gradient-card')) {
      process.stdout.write('\x1B[38;2;255;255;255m\x1B[48;2;30;60;120m')
    } else if (classes.includes('glass-effect')) {
      process.stdout.write('\x1B[38;2;200;220;255m\x1B[48;2;20;30;50m\x1B[2m')
    } else if (classes.includes('neon-glow')) {
      process.stdout.write('\x1B[38;2;0;255;255m\x1B[48;2;0;30;30m\x1B[1m\x1B[5m')
    } else if (classes.includes('title')) {
      process.stdout.write('\x1B[1m\x1B[96m\x1B[48;2;20;20;40m')
    } else if (classes.includes('content-title')) {
      process.stdout.write('\x1B[1m\x1B[97m\x1B[48;2;40;40;60m')
    } else if (classes.includes('description')) {
      process.stdout.write('\x1B[37m\x1B[48;2;15;15;25m')
    } else if (classes.includes('sidebar-title')) {
      process.stdout.write('\x1B[1m\x1B[33m\x1B[48;2;60;40;0m')
    } else if (classes.includes('shadow-layer')) {
      process.stdout.write('\x1B[38;2;100;100;100m\x1B[48;2;0;0;0m\x1B[2m')
    } else {
      process.stdout.write('\x1B[97m\x1B[48;2;10;15;25m')
    }
  }
  
  private hslToRgb(h: number, s: number, l: number): [number, number, number] {
    const c = (1 - Math.abs(2 * l - 1)) * s
    const x = c * (1 - Math.abs((h * 6) % 2 - 1))
    const m = l - c / 2
    let r = 0, g = 0, b = 0
    
    if (h < 1/6) { r = c; g = x; b = 0 }
    else if (h < 2/6) { r = x; g = c; b = 0 }
    else if (h < 3/6) { r = 0; g = c; b = x }
    else if (h < 4/6) { r = 0; g = x; b = c }
    else if (h < 5/6) { r = x; g = 0; b = c }
    else { r = c; g = 0; b = x }
    
    return [
      Math.round((r + m) * 255),
      Math.round((g + m) * 255),
      Math.round((b + m) * 255)
    ]
  }
  
  // Advanced Component Renderers
  
  private renderInput(element: any, x: number, y: number): void {
    // Register as focusable
    this.addFocusableElement(element, x, y)
    
    const type = element.attributes?.type || 'text'
    const placeholder = element.attributes?.placeholder || ''
    const value = element.attributes?.value || ''
    const isFocused = this.isFocused(element)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    // Input field with border and focus indicator
    if (isFocused) {
      process.stdout.write('\x1B[48;2;60;60;100m\x1B[97m[') // Blue focus background
    } else {
      process.stdout.write('\x1B[48;2;40;40;50m\x1B[97m[')
    }
    
    if (type === 'password') {
      process.stdout.write('*'.repeat(value.length))
      process.stdout.write(' '.repeat(Math.max(0, 20 - value.length)))
    } else {
      if (value) {
        process.stdout.write(value.padEnd(20))
      } else {
        process.stdout.write('\x1B[2m' + placeholder.padEnd(20) + '\x1B[0m')
        if (isFocused) {
          process.stdout.write('\x1B[48;2;60;60;100m')
        } else {
          process.stdout.write('\x1B[48;2;40;40;50m')
        }
      }
    }
    
    process.stdout.write(']\x1B[0m')
    
    // Show cursor for focused input
    if (isFocused) {
      process.stdout.write(`\x1B[${y + 1};${x + value.length + 2}H\x1B[97m|\x1B[0m`)
    }
  }
  
  private renderProgress(element: any, x: number, y: number): void {
    const value = parseInt(element.attributes?.value || '0')
    const max = parseInt(element.attributes?.max || '100')
    const label = element.attributes?.label || ''
    const percentage = Math.min(100, Math.max(0, (value / max) * 100))
    const barWidth = 30
    const filled = Math.floor((percentage / 100) * barWidth)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    if (label) {
      process.stdout.write('\x1B[97m' + label + ' ')
    }
    
    // Progress bar with gradient fill
    process.stdout.write('\x1B[90m[')
    for (let i = 0; i < barWidth; i++) {
      if (i < filled) {
        const intensity = Math.floor(255 * (i / barWidth))
        process.stdout.write(`\x1B[48;2;${intensity};${255-intensity};0m█`)
      } else {
        process.stdout.write('\x1B[48;2;20;20;20m░')
      }
    }
    process.stdout.write('\x1B[0m\x1B[90m] ')
    process.stdout.write('\x1B[97m' + Math.round(percentage) + '%\x1B[0m')
  }
  
  private renderCheckbox(element: any, x: number, y: number): void {
    // Register as focusable
    this.addFocusableElement(element, x, y)
    
    const checked = element.attributes?.checked === 'true'
    const label = element.attributes?.label || ''
    const isFocused = this.isFocused(element)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    if (isFocused) {
      process.stdout.write('\x1B[103m\x1B[30m') // Yellow focus background
    }
    
    if (checked) {
      process.stdout.write('\x1B[92m[✓]\x1B[0m ')
    } else {
      process.stdout.write('\x1B[90m[ ]\x1B[0m ')
    }
    
    if (isFocused) {
      process.stdout.write('\x1B[103m\x1B[30m') // Continue focus background for label
    }
    
    if (label) {
      process.stdout.write('\x1B[97m' + label + '\x1B[0m')
    }
  }
  
  private renderRadio(element: any, x: number, y: number): void {
    // Register as focusable
    this.addFocusableElement(element, x, y)
    
    const checked = element.attributes?.checked === 'true'
    const label = element.attributes?.label || ''
    const isFocused = this.isFocused(element)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    if (isFocused) {
      process.stdout.write('\x1B[103m\x1B[30m') // Yellow focus background
    }
    
    if (checked) {
      process.stdout.write('\x1B[96m(●)\x1B[0m ')
    } else {
      process.stdout.write('\x1B[90m( )\x1B[0m ')
    }
    
    if (isFocused) {
      process.stdout.write('\x1B[103m\x1B[30m') // Continue focus background for label
    }
    
    if (label) {
      process.stdout.write('\x1B[97m' + label + '\x1B[0m')
    }
  }
  
  private renderSpinner(element: any, x: number, y: number): void {
    const type = element.attributes?.type || 'dots'
    const frame = Math.floor(Date.now() / 200) % 4
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    switch (type) {
      case 'dots':
        const dots = ['⠋', '⠙', '⠹', '⠸', '⠼', '⠴', '⠦', '⠧', '⠇', '⠏']
        process.stdout.write('\x1B[96m' + dots[frame % dots.length] + '\x1B[0m Loading...')
        break
      case 'bars':
        const bars = ['▁', '▃', '▄', '▅', '▆', '▇', '█', '▇', '▆', '▅', '▄', '▃']
        process.stdout.write('\x1B[93m' + bars[frame % bars.length] + '\x1B[0m Loading...')
        break
      case 'circle':
        const circles = ['◐', '◓', '◑', '◒']
        process.stdout.write('\x1B[95m' + circles[frame] + '\x1B[0m Loading...')
        break
    }
  }
  
  private renderTable(element: any, x: number, y: number): void {
    const headers = JSON.parse(element.attributes?.headers || '[]')
    const rows = JSON.parse(element.attributes?.rows || '[]')
    
    let currentY = y
    
    if (headers.length > 0) {
      // Render headers
      process.stdout.write(`\x1B[${currentY + 1};${x + 1}H`)
      process.stdout.write('\x1B[1m\x1B[97m')
      
      for (let i = 0; i < headers.length; i++) {
        process.stdout.write('│ ' + headers[i].padEnd(12) + ' ')
      }
      process.stdout.write('│\x1B[0m')
      currentY++
      
      // Header separator
      process.stdout.write(`\x1B[${currentY + 1};${x + 1}H`)
      process.stdout.write('\x1B[90m')
      for (let i = 0; i < headers.length; i++) {
        process.stdout.write('├' + '─'.repeat(14))
      }
      process.stdout.write('┤\x1B[0m')
      currentY++
    }
    
    // Render rows
    for (const row of rows) {
      process.stdout.write(`\x1B[${currentY + 1};${x + 1}H`)
      process.stdout.write('\x1B[97m')
      
      for (let i = 0; i < row.length; i++) {
        process.stdout.write('│ ' + String(row[i]).padEnd(12) + ' ')
      }
      process.stdout.write('│\x1B[0m')
      currentY++
    }
  }
  
  private renderModal(element: any, x: number, y: number): void {
    const title = element.attributes?.title || 'Dialog'
    const visible = element.attributes?.visible !== 'false'
    
    if (!visible) return
    
    const modalWidth = 50
    const modalHeight = 10
    const modalX = x + 15
    const modalY = y + 5
    
    // Modal background overlay
    for (let i = 0; i < modalHeight; i++) {
      process.stdout.write(`\x1B[${modalY + i + 1};${modalX + 1}H`)
      process.stdout.write('\x1B[48;2;0;0;0m\x1B[2m' + ' '.repeat(modalWidth) + '\x1B[0m')
    }
    
    // Modal border
    process.stdout.write(`\x1B[${modalY + 1};${modalX + 1}H`)
    process.stdout.write('\x1B[97m┌' + '─'.repeat(modalWidth - 2) + '┐')
    
    // Modal title
    process.stdout.write(`\x1B[${modalY + 2};${modalX + 1}H`)
    process.stdout.write('\x1B[97m│ \x1B[1m' + title.padEnd(modalWidth - 4) + '\x1B[0m\x1B[97m │')
    
    // Modal content area
    for (let i = 2; i < modalHeight - 2; i++) {
      process.stdout.write(`\x1B[${modalY + i + 1};${modalX + 1}H`)
      process.stdout.write('\x1B[97m│' + ' '.repeat(modalWidth - 2) + '│')
    }
    
    // Modal bottom border
    process.stdout.write(`\x1B[${modalY + modalHeight};${modalX + 1}H`)
    process.stdout.write('\x1B[97m└' + '─'.repeat(modalWidth - 2) + '┘\x1B[0m')
    
    // Render modal children
    for (const child of element.children || []) {
      this.renderComponentToTerminal(child, modalX + 2, modalY + 3)
    }
  }
  
  // Widget-specific renderers
  
  private renderSlider(element: any, x: number, y: number): void {
    const mode = element.attributes?.['data-mode'] || 'single'
    const orientation = element.attributes?.['data-orientation'] || 'horizontal'
    const min = parseInt(element.attributes?.min || '0')
    const max = parseInt(element.attributes?.max || '100')
    const value = parseInt(element.attributes?.value || '0')
    const rangeEnd = parseInt(element.attributes?.['range-end'] || value.toString())
    const label = element.attributes?.label || ''
    
    // Register as focusable
    this.addFocusableElement(element, x, y)
    const isFocused = this.isFocused(element)
    
    process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    
    if (label) {
      process.stdout.write('\x1B[97m' + label + '\x1B[0m')
      y++
      process.stdout.write(`\x1B[${y + 1};${x + 1}H`)
    }
    
    if (orientation === 'horizontal') {
      const sliderWidth = 30
      const position = Math.floor(((value - min) / (max - min)) * sliderWidth)
      const rangeEndPos = mode === 'range' ? Math.floor(((rangeEnd - min) / (max - min)) * sliderWidth) : position
      
      // Slider track
      process.stdout.write('\x1B[90m[')
      for (let i = 0; i < sliderWidth; i++) {
        if (mode === 'range') {
          if (i >= Math.min(position, rangeEndPos) && i <= Math.max(position, rangeEndPos)) {
            process.stdout.write('\x1B[96m█') // Cyan for range
          } else {
            process.stdout.write('\x1B[90m─')
          }
        } else {
          if (i <= position) {
            process.stdout.write('\x1B[94m█') // Blue for filled
          } else {
            process.stdout.write('\x1B[90m─')
          }
        }
      }
      process.stdout.write('\x1B[90m]')
      
      // Focus indicator
      if (isFocused) {
        process.stdout.write(`\x1B[${y + 1};${x + 1 + position + 1}H\x1B[103m\x1B[30m●\x1B[0m`)
        if (mode === 'range') {
          process.stdout.write(`\x1B[${y + 1};${x + 1 + rangeEndPos + 1}H\x1B[103m\x1B[30m●\x1B[0m`)
        }
      }
      
    } else {
      // Vertical slider (simplified)
      const sliderHeight = 10
      const position = Math.floor(((value - min) / (max - min)) * sliderHeight)
      
      for (let i = sliderHeight; i >= 0; i--) {
        process.stdout.write(`\x1B[${y + sliderHeight - i + 1};${x + 1}H`)
        if (i <= position) {
          process.stdout.write('\x1B[94m█')
        } else {
          process.stdout.write('\x1B[90m│')
        }
      }
    }
    
    process.stdout.write('\x1B[0m')
  }
  
  private renderGrid(element: any, x: number, y: number): void {
    // Extract grid configuration from CSS classes
    const gridCols = element.classes?.find((cls: string) => cls.startsWith('grid-cols-'))
    const numColumns = gridCols ? parseInt(gridCols.replace('grid-cols-', '')) : 2
    const gap = 1 // Default gap
    
    const items = element.children || []
    
    // Get actual terminal dimensions for responsive layout
    const terminalWidth = process.stdout.columns || 400
    const terminalHeight = process.stdout.rows || 200
    
    // Use full terminal width and height for responsive layout
    const availableWidth = terminalWidth - 2 // Minimal margin
    const availableHeight = terminalHeight - 4 - Math.max(0, y - 2) // From header to status bar
    
    // Calculate item dimensions based on available space
    const itemWidth = Math.floor((availableWidth - (gap * (numColumns - 1))) / numColumns)
    
    // Calculate rows needed and adjust item height accordingly
    const totalRows = Math.ceil(items.length / numColumns)
    const itemHeight = Math.max(3, Math.floor((availableHeight - (gap * (totalRows - 1))) / totalRows))
    
    let currentRow = 0
    let currentCol = 0
    
    for (const item of items) {
      // Extract spans from CSS classes
      const colSpanClass = item.classes?.find((cls: string) => cls.startsWith('col-span-'))
      const rowSpanClass = item.classes?.find((cls: string) => cls.startsWith('row-span-'))
      const colSpan = colSpanClass ? parseInt(colSpanClass.replace('col-span-', '')) : 1
      const rowSpan = rowSpanClass ? parseInt(rowSpanClass.replace('row-span-', '')) : 1
      
      const backgroundColor = item.attributes?.['data-bg-color']
      const content = item.content || ''
      
      // Calculate responsive position with minimal margins
      const itemX = x + 1 + (currentCol * (itemWidth + gap))
      const itemY = y + (currentRow * (itemHeight + gap))
      
      // Apply background color if specified
      let bgColor = '\x1B[48;2;30;30;40m' // Default dark background
      if (backgroundColor) {
        // Parse hex color to RGB
        const hex = backgroundColor.replace('#', '')
        if (hex.length === 6) {
          const r = parseInt(hex.substr(0, 2), 16)
          const g = parseInt(hex.substr(2, 2), 16)
          const b = parseInt(hex.substr(4, 2), 16)
          bgColor = `\x1B[48;2;${r};${g};${b}m`
        }
      }
      
      // Calculate actual dimensions with spanning
      const actualWidth = Math.min(itemWidth * colSpan + gap * (colSpan - 1), availableWidth - (itemX - x - 2))
      const actualHeight = itemHeight * rowSpan + gap * (rowSpan - 1)
      
      // Modern minimal design with subtle shadows
      const shadowColor = '\x1B[38;2;20;20;25m'
      
      // Drop shadow effect (offset by 1)
      for (let row = 1; row <= actualHeight; row++) {
        process.stdout.write(`\x1B[${itemY + row + 1};${itemX + 2}H`)
        process.stdout.write(shadowColor + '▓'.repeat(actualWidth) + '\x1B[0m')
      }
      
      // Main panel with clean background
      for (let row = 0; row < actualHeight; row++) {
        process.stdout.write(`\x1B[${itemY + row + 1};${itemX + 1}H`)
        process.stdout.write(bgColor + ' '.repeat(actualWidth) + '\x1B[0m')
      }
      
      // Render content with modern typography
      const lines = content.split('\n')
      const contentStartY = itemY + Math.floor((actualHeight - lines.length) / 2)
      
      for (let i = 0; i < Math.min(lines.length, actualHeight - 2); i++) {
        const line = lines[i].substring(0, actualWidth - 4)
        const centeredLine = line.padStart(Math.floor((actualWidth - 4 + line.length) / 2)).padEnd(actualWidth - 4)
        process.stdout.write(`\x1B[${contentStartY + i + 1};${itemX + 3}H`)
        
        // Clean modern typography - white text, no bold
        process.stdout.write(bgColor + '\x1B[255m' + centeredLine + '\x1B[0m')
      }
      
      // Move to next position with proper column wrapping
      currentCol += colSpan
      if (currentCol >= numColumns) {
        currentCol = 0
        currentRow += rowSpan
      }
    }
  }
  
  private renderPanel(element: any, x: number, y: number): void {
    const title = element.attributes?.['data-title'] || ''
    const content = element.attributes?.['data-content'] || element.content || ''
    const borderStyle = element.attributes?.['data-border-style'] || 'clean'
    const padding = parseInt(element.attributes?.['data-padding'] || '2')
    const _titleStyle = element.attributes?.['data-title-style'] || 'normal'
    
    // Get terminal dimensions
    const terminalWidth = process.stdout.columns || 400
    const terminalHeight = process.stdout.rows || 200
    
    // Calculate panel dimensions - use more of terminal space
    const panelWidth = Math.floor(terminalWidth * 0.48) // Larger responsive width
    const contentLines = content.split('\n')
    const maxHeight = terminalHeight - y - 4 // Leave space for status bar
    const panelHeight = Math.min(maxHeight, Math.max(10, contentLines.length + (title ? 4 : 2) + (padding * 2)))
    
    // Panel styles - brighter for black backgrounds
    const styles = {
      clean: {
        bg: '\x1B[48;2;70;80;95m',  // Lighter background
        shadow: '\x1B[38;2;30;35;45m',
        title: '\x1B[38;2;255;255;255m\x1B[1m',
        content: '\x1B[38;2;240;245;255m'
      },
      shadow: {
        bg: '\x1B[48;2;65;75;90m',  // Lighter background
        shadow: '\x1B[38;2;25;30;40m',
        title: '\x1B[38;2;255;255;255m\x1B[1m',
        content: '\x1B[38;2;230;235;245m'
      },
      minimal: {
        bg: '\x1B[48;2;60;70;85m',  // Lighter background
        shadow: '\x1B[38;2;20;25;35m',
        title: '\x1B[38;2;255;255;255m\x1B[1m',
        content: '\x1B[38;2;220;225;235m'
      }
    }
    
    const style = styles[borderStyle as keyof typeof styles] || styles.clean
    
    // Render shadow first (for depth)
    if (borderStyle === 'shadow') {
      for (let row = 1; row <= panelHeight; row++) {
        process.stdout.write(`\x1B[${y + row + 1};${x + 2}H`)
        process.stdout.write(style.shadow + '▓'.repeat(panelWidth) + '\x1B[0m')
      }
    }
    
    // Render main panel background
    for (let row = 0; row < panelHeight; row++) {
      process.stdout.write(`\x1B[${y + row + 1};${x + 1}H`)
      process.stdout.write(style.bg + ' '.repeat(panelWidth) + '\x1B[0m')
    }
    
    // Render title if present
    let contentStartY = y + padding
    if (title) {
      const titleY = y + 1
      const titleText = title.length > panelWidth - 4 ? title.substring(0, panelWidth - 4) : title
      const centeredTitle = titleText.padStart(Math.floor((panelWidth - 4 + titleText.length) / 2)).padEnd(panelWidth - 4)
      
      process.stdout.write(`\x1B[${titleY + 1};${x + 3}H`)
      process.stdout.write(style.bg + style.title + centeredTitle + '\x1B[0m')
      contentStartY += 2
    }
    
    // Render content
    for (let i = 0; i < Math.min(contentLines.length, panelHeight - (title ? 4 : 2) - padding); i++) {
      const line = contentLines[i].substring(0, panelWidth - 4)
      const centeredLine = line.padStart(Math.floor((panelWidth - 4 + line.length) / 2)).padEnd(panelWidth - 4)
      
      process.stdout.write(`\x1B[${contentStartY + i + 1};${x + 3}H`)
      process.stdout.write(style.bg + style.content + centeredLine + '\x1B[0m')
    }
  }
  
  private renderBar(element: any, x: number, y: number): void {
    const position = element.attributes?.['data-position'] || 'top'
    const height = parseInt(element.attributes?.['data-height'] || '1')
    const backgroundColor = element.attributes?.['data-background'] || ''
    const borderStyle = element.attributes?.['data-border-style'] || 'none'
    const borderColor = element.attributes?.['data-border-color'] || ''
    const items = JSON.parse(element.attributes?.['data-items'] || '[]')
    
    const terminalWidth = process.stdout.columns || 400

    // Calculate bar position
    let barY = y
    if (position === 'bottom' || position === 'fixed-bottom') {
      barY = (process.stdout.rows || 200) - height - 1
    }
    
    // Background color
    let bgColor = '\x1B[48;2;45;55;70m' // Default bar background
    if (backgroundColor) {
      if (backgroundColor.startsWith('#')) {
        const hex = backgroundColor.slice(1)
        const r = parseInt(hex.substr(0, 2), 16)
        const g = parseInt(hex.substr(2, 2), 16)
        const b = parseInt(hex.substr(4, 2), 16)
        bgColor = `\x1B[48;2;${r};${g};${b}m`
      }
    }
    
    // Render bar background
    for (let row = 0; row < height; row++) {
      process.stdout.write(`\x1B[${barY + row + 1};1H`)
      process.stdout.write(bgColor + ' '.repeat(terminalWidth) + '\x1B[0m')
    }
    
    // Render border if specified
    if (borderStyle === 'solid') {
      const borderColorCode = borderColor ? 
        `\x1B[38;2;${parseInt(borderColor.slice(1,3), 16)};${parseInt(borderColor.slice(3,5), 16)};${parseInt(borderColor.slice(5,7), 16)}m` :
        '\x1B[38;2;100;110;125m'
      
      if (position === 'top' || position === 'fixed-top') {
        process.stdout.write(`\x1B[${barY + height + 1};1H`)
        process.stdout.write(borderColorCode + '─'.repeat(terminalWidth) + '\x1B[0m')
      } else {
        process.stdout.write(`\x1B[${barY};1H`)
        process.stdout.write(borderColorCode + '─'.repeat(terminalWidth) + '\x1B[0m')
      }
    }
    
    // Render items
    const leftItems = items.filter((item: any) => item.position === 'left')
    const centerItems = items.filter((item: any) => item.position === 'center') 
    const rightItems = items.filter((item: any) => item.position === 'right')
    
    let currentX = 2 // Start with padding
    
    // Render left items
    for (const item of leftItems) {
      const content = item.content || ''
      const itemStyle = item.style || {}
      
      let textColor = '\x1B[97m' // Default white
      if (itemStyle.color) {
        if (itemStyle.color.startsWith('#')) {
          const hex = itemStyle.color.slice(1)
          const r = parseInt(hex.substr(0, 2), 16)
          const g = parseInt(hex.substr(2, 2), 16)
          const b = parseInt(hex.substr(4, 2), 16)
          textColor = `\x1B[38;2;${r};${g};${b}m`
        }
      }
      
      const decorations = [
        itemStyle.bold ? '\x1B[1m' : '',
        itemStyle.italic ? '\x1B[3m' : '',
        itemStyle.underline ? '\x1B[4m' : ''
      ].join('')
      
      process.stdout.write(`\x1B[${barY + 1};${currentX + 1}H`)
      process.stdout.write(bgColor + textColor + decorations + content + '\x1B[0m')
      
      currentX += content.length + 2 // Add spacing
    }
    
    // Render right items (from right side)
    let rightX = terminalWidth - 2
    for (let i = rightItems.length - 1; i >= 0; i--) {
      const item = rightItems[i]
      const content = item.content || ''
      const itemStyle = item.style || {}
      
      let textColor = '\x1B[97m'
      if (itemStyle.color) {
        if (itemStyle.color.startsWith('#')) {
          const hex = itemStyle.color.slice(1)
          const r = parseInt(hex.substr(0, 2), 16)
          const g = parseInt(hex.substr(2, 2), 16)
          const b = parseInt(hex.substr(4, 2), 16)
          textColor = `\x1B[38;2;${r};${g};${b}m`
        }
      }
      
      const decorations = [
        itemStyle.bold ? '\x1B[1m' : '',
        itemStyle.italic ? '\x1B[3m' : '',
        itemStyle.underline ? '\x1B[4m' : ''
      ].join('')
      
      rightX -= content.length
      process.stdout.write(`\x1B[${barY + 1};${rightX + 1}H`)
      process.stdout.write(bgColor + textColor + decorations + content + '\x1B[0m')
      rightX -= 2 // Add spacing
    }
    
    // Render center items
    if (centerItems.length > 0) {
      const centerContent = centerItems.map(item => item.content || '').join(' ')
      const centerX = Math.floor((terminalWidth - centerContent.length) / 2)
      
      process.stdout.write(`\x1B[${barY + 1};${centerX + 1}H`)
      process.stdout.write(bgColor + '\x1B[97m' + centerContent + '\x1B[0m')
    }
  }
  
  private renderToast(element: any, x: number, y: number): void {
    const variant = element.attributes?.['data-variant'] || 'info'
    const position = element.attributes?.['data-position'] || 'top-right'
    const message = element.attributes?.message || ''
    const dismissible = element.attributes?.dismissible !== 'false'
    
    // Determine color based on variant
    let bgColor = '\x1B[48;2;59;130;246m' // Blue for info
    let textColor = '\x1B[97m'
    
    switch (variant) {
      case 'success':
        bgColor = '\x1B[48;2;34;197;94m' // Green
        break
      case 'warning':
        bgColor = '\x1B[48;2;245;158;11m' // Orange
        textColor = '\x1B[30m' // Dark text for better contrast
        break
      case 'error':
        bgColor = '\x1B[48;2;239;68;68m' // Red
        break
    }
    
    const toastWidth = Math.min(50, message.length + 4)
    const toastHeight = 3
    
    // Adjust position based on position attribute
    let toastX = x
    let toastY = y
    
    if (position.includes('right')) {
      toastX = 80 - toastWidth - 2
    } else if (position.includes('center')) {
      toastX = Math.floor((80 - toastWidth) / 2)
    }
    
    if (position.includes('bottom')) {
      toastY = 20 - toastHeight - 2
    }
    
    // Render toast background
    for (let row = 0; row < toastHeight; row++) {
      process.stdout.write(`\x1B[${toastY + row + 1};${toastX + 1}H`)
      process.stdout.write(bgColor + ' '.repeat(toastWidth) + '\x1B[0m')
    }
    
    // Render toast content
    process.stdout.write(`\x1B[${toastY + 2};${toastX + 3}H`)
    process.stdout.write(bgColor + textColor + message.substring(0, toastWidth - 4) + '\x1B[0m')
    
    // Render close button if dismissible
    if (dismissible) {
      process.stdout.write(`\x1B[${toastY + 1};${toastX + toastWidth - 2}H`)
      process.stdout.write(bgColor + textColor + '×\x1B[0m')
    }
  }
  
  // Focus Management System
  
  private isFocusable(element: any): boolean {
    return ['button', 'input', 'checkbox', 'radio', 'slider'].includes(element.tag)
  }
  
  private addFocusableElement(element: any, x: number, y: number): void {
    if (this.isFocusable(element)) {
      const id = element.id || `${element.tag}-${this.focusableElements.length}`
      this.focusableElements.push({ element, x, y, id })
    }
  }
  
  private moveFocus(direction: number): void {
    if (this.focusableElements.length === 0) return
    
    this.focusedIndex = (this.focusedIndex + direction + this.focusableElements.length) % this.focusableElements.length
    this.needsRerender = true
  }
  
  private activateFocused(): void {
    if (this.focusableElements.length === 0) return
    
    const focused = this.focusableElements[this.focusedIndex]
    const element = focused.element
    
    if (element.tag === 'button') {
      // Button click feedback
      process.stdout.write('\x1B[?25l') // Hide cursor
      process.stdout.write(`\x1B[${focused.y + 1};${focused.x + 1}H`)
      process.stdout.write('\x1B[103m\x1B[30m CLICKED! \x1B[0m')
      
      setTimeout(() => {
        this.needsRerender = true
        this.rerenderInterface()
      }, 200)
      
    } else if (element.tag === 'checkbox') {
      // Toggle checkbox
      const currentState = element.attributes?.checked === 'true'
      element.attributes = element.attributes || {}
      element.attributes.checked = (!currentState).toString()
      this.needsRerender = true
      
    } else if (element.tag === 'radio') {
      // Select radio button (unselect others in same group)
      const name = element.attributes?.name
      if (name) {
        for (const focusable of this.focusableElements) {
          if (focusable.element.tag === 'radio' && focusable.element.attributes?.name === name) {
            focusable.element.attributes.checked = 'false'
          }
        }
      }
      element.attributes = element.attributes || {}
      element.attributes.checked = 'true'
      this.needsRerender = true
      
    } else if (element.tag === 'slider') {
      // Slider value adjustment (placeholder - real implementation would handle arrow keys)
      const _min = parseInt(element.attributes?.min || '0')
      const _max = parseInt(element.attributes?.max || '100')
      const _currentValue = parseInt(element.attributes?.value || '0')
      const _step = parseFloat(element.attributes?.step || '1')
      
      // For now, just show feedback that slider is focused
      process.stdout.write('\x1B[?25l') // Hide cursor
      process.stdout.write(`\x1B[${focused.y + 1};${focused.x + 1}H`)
      process.stdout.write('\x1B[103m\x1B[30m SLIDER FOCUSED \x1B[0m')
      
      setTimeout(() => {
        this.needsRerender = true
        this.rerenderInterface()
      }, 200)
    }
  }
  
  private rerenderInterface(): void {
    // Clear screen and re-render
    process.stdout.write('\x1B[2J\x1B[0f')
    
    const component = this.config.component()
    const builtComponent = 'build' in component ? component.build() : component
    
    this.renderComponentToTerminal(builtComponent, 0, 0)
    
    // Minimal status line - just essential info
    process.stdout.write(`\x1B[24;1H\x1B[2m\x1B[90m↑↓ navigate • enter: next • q: quit\x1B[0m`)
    
    this.needsRerender = false
  }
  
  private isFocused(element: any): boolean {
    if (this.focusableElements.length === 0) return false
    const focused = this.focusableElements[this.focusedIndex]
    return focused && focused.element === element
  }
}

/**
 * Create a new TUI application
 * 
 * @example
 * ```typescript
 * const app = createApp({
 *   stylesheet: './styles.css',
 *   component: () => div().child(text('Hello World!'))
 * })
 * 
 * await app.run()
 * ```
 */
export function createApp(config: AppConfig): TuiApp {
  return new TuiAppImpl(config)
}

/**
 * Quick start function for simple apps
 * 
 * @example
 * ```typescript
 * await runApp('./styles.css', () => 
 *   div({ class: 'app' }).child(text('Quick start!'))
 * )
 * ```
 */
export async function runApp(
  stylesheet: string | undefined, 
  component: ComponentFunction
): Promise<void> {
  const app = createApp({ stylesheet, component })
  await app.run()
}