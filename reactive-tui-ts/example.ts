#!/usr/bin/env bun

/**
 * Reactive TUI TypeScript Example
 *
 * This example demonstrates the basic usage of Reactive TUI with TypeScript,
 * showcasing type safety, modern CSS styling, and interactive components.
 *
 * Run with: bun run example.ts
 */

import {
  JsTuiApp,
  TuiUtils,
  JsElement,
  JsToast,
  JsToastManager,
  JsColorTheme
} from 'reactive-tui'

/**
 * Main application class demonstrating TypeScript best practices
 */
class ReactiveUIDemo {
  private app: JsTuiApp
  private toastManager: JsToastManager
  private currentTheme: JsColorTheme

  constructor() {
    // Initialize the TUI application with type safety
    this.app = new JsTuiApp()
    this.app.setTitle('🚀 Reactive TUI TypeScript Demo')

    // Set up toast notifications
    const [termWidth, termHeight] = TuiUtils.getTerminalSize()
    this.toastManager = new JsToastManager(termWidth, termHeight)

    // Initialize with dark theme
    this.currentTheme = JsColorTheme.dark()

    this.setupStyling()
    this.createUI()
  }

  /**
   * Set up CSS styling with modern design patterns
   */
  private setupStyling(): void {
    this.app.loadCss(`
      /* Modern CSS Reset and Base Styles */
      * {
        box-sizing: border-box;
      }

      /* Main Application Container */
      .app-container {
        background: #0d1117;
        color: #f0f6fc;
        padding: 2rem;
        min-height: 100vh;
        display: flex;
        flex-direction: column;
        font-family: 'SF Mono', 'Monaco', 'Inconsolata', monospace;
      }

      /* Header Section */
      .header {
        text-align: center;
        margin-bottom: 2rem;
        padding-bottom: 1rem;
        border-bottom: 1px solid #21262d;
      }

      .title {
        font-size: 2rem;
        font-weight: bold;
        background: linear-gradient(45deg, #58a6ff, #f85149);
        background-clip: text;
        color: transparent;
        margin-bottom: 0.5rem;
      }

      .subtitle {
        color: #8b949e;
        font-size: 1rem;
      }

      /* Main Content Grid */
      .main-content {
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 2rem;
        flex: 1;
      }

      /* Card Components */
      .card {
        background: #161b22;
        border: 1px solid #30363d;
        border-radius: 8px;
        padding: 1.5rem;
        transition: all 0.2s ease;
      }

      .card:hover {
        border-color: #58a6ff;
        box-shadow: 0 0 0 1px #58a6ff;
      }

      .card-title {
        font-size: 1.25rem;
        font-weight: 600;
        color: #f0f6fc;
        margin-bottom: 1rem;
        display: flex;
        align-items: center;
        gap: 0.5rem;
      }

      .card-content {
        color: #8b949e;
        line-height: 1.6;
      }

      /* Button Styles */
      .button-group {
        display: flex;
        gap: 1rem;
        margin-top: 1.5rem;
        flex-wrap: wrap;
      }

      .btn {
        background: #238636;
        color: #ffffff;
        border: 1px solid #2ea043;
        padding: 0.75rem 1.5rem;
        border-radius: 6px;
        font-weight: 500;
        cursor: pointer;
        transition: all 0.2s ease;
        text-align: center;
        min-width: 120px;
      }

      .btn:hover {
        background: #2ea043;
        border-color: #46954a;
      }

      .btn:focus {
        outline: 2px solid #58a6ff;
        outline-offset: 2px;
      }

      .btn-secondary {
        background: #21262d;
        color: #f0f6fc;
        border-color: #30363d;
      }

      .btn-secondary:hover {
        background: #30363d;
        border-color: #484f58;
      }

      .btn-danger {
        background: #da3633;
        border-color: #f85149;
      }

      .btn-danger:hover {
        background: #f85149;
        border-color: #ff7b72;
      }

      /* Status Indicators */
      .status-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
        gap: 1rem;
        margin-top: 1rem;
      }

      .status-item {
        background: #0d1117;
        border: 1px solid #21262d;
        border-radius: 6px;
        padding: 1rem;
        text-align: center;
      }

      .status-value {
        font-size: 1.5rem;
        font-weight: bold;
        color: #58a6ff;
      }

      .status-label {
        color: #8b949e;
        font-size: 0.875rem;
        margin-top: 0.25rem;
      }

      /* Footer */
      .footer {
        margin-top: 2rem;
        padding-top: 1rem;
        border-top: 1px solid #21262d;
        text-align: center;
        color: #6e7681;
        font-size: 0.875rem;
      }

      /* Responsive Design */
      @media (max-width: 80) {
        .main-content {
          grid-template-columns: 1fr;
        }

        .button-group {
          flex-direction: column;
        }

        .status-grid {
          grid-template-columns: 1fr;
        }
      }
    `)
  }

  /**
   * Create the main UI structure with TypeScript type safety
   */
  private createUI(): void {
    // Main container
    const container = TuiUtils.div()
    container.setId('app-root')
    container.addClass('app-container')

    // Header section
    const header = this.createHeader()
    container.addChild(header)

    // Main content
    const mainContent = this.createMainContent()
    container.addChild(mainContent)

    // Footer
    const footer = this.createFooter()
    container.addChild(footer)

    // Set the root component
    this.app.setComponent(container)
  }

  /**
   * Create the header section
   */
  private createHeader(): JsElement {
    const header = TuiUtils.div()
    header.addClass('header')

    const title = TuiUtils.div()
    title.addClass('title')
    title.setContent('🚀 Reactive TUI')

    const subtitle = TuiUtils.div()
    subtitle.addClass('subtitle')
    subtitle.setContent('CSS-styled Terminal User Interfaces with TypeScript')

    header.addChild(title)
    header.addChild(subtitle)

    return header
  }

  /**
   * Create the main content area
   */
  private createMainContent(): JsElement {
    const mainContent = TuiUtils.div()
    mainContent.addClass('main-content')

    // Features card
    const featuresCard = this.createFeaturesCard()
    mainContent.addChild(featuresCard)

    // Demo card
    const demoCard = this.createDemoCard()
    mainContent.addChild(demoCard)

    return mainContent
  }

  /**
   * Create the features showcase card
   */
  private createFeaturesCard(): JsElement {
    const card = TuiUtils.div()
    card.addClass('card')

    const title = TuiUtils.div()
    title.addClass('card-title')
    title.setContent('✨ Features')

    const content = TuiUtils.div()
    content.addClass('card-content')
    content.setContent(`
• Full TypeScript support with IntelliSense
• CSS-first design with modern layouts
• Rich widget library (25+ components)
• Responsive terminal interfaces
• Theme system with custom colors
• Hot reload development workflow
• Cross-platform compatibility
• Zero dependencies, native performance
    `.trim())

    const statusGrid = this.createStatusGrid()

    card.addChild(title)
    card.addChild(content)
    card.addChild(statusGrid)

    return card
  }

  /**
   * Create the interactive demo card
   */
  private createDemoCard(): JsElement {
    const card = TuiUtils.div()
    card.addClass('card')

    const title = TuiUtils.div()
    title.addClass('card-title')
    title.setContent('🎮 Interactive Demo')

    const content = TuiUtils.div()
    content.addClass('card-content')
    content.setContent('Try out the interactive features below. Each button demonstrates different capabilities of the framework with full type safety.')

    const buttonGroup = this.createButtonGroup()

    card.addChild(title)
    card.addChild(content)
    card.addChild(buttonGroup)

    return card
  }

  /**
   * Create status indicators
   */
  private createStatusGrid(): JsElement {
    const statusGrid = TuiUtils.div()
    statusGrid.addClass('status-grid')

    const statuses = [
      { label: 'TypeScript', value: '100%' },
      { label: 'Performance', value: 'A+' },
      { label: 'Widgets', value: '25+' },
      { label: 'Themes', value: '∞' }
    ]

    statuses.forEach(status => {
      const item = TuiUtils.div()
      item.addClass('status-item')

      const value = TuiUtils.div()
      value.addClass('status-value')
      value.setContent(status.value)

      const label = TuiUtils.div()
      label.addClass('status-label')
      label.setContent(status.label)

      item.addChild(value)
      item.addChild(label)
      statusGrid.addChild(item)
    })

    return statusGrid
  }

  /**
   * Create interactive button group
   */
  private createButtonGroup(): JsElement {
    const buttonGroup = TuiUtils.div()
    buttonGroup.addClass('button-group')

    // Success toast button
    const successBtn = TuiUtils.button()
    successBtn.addClass('btn')
    successBtn.setContent('✅ Success Toast')
    successBtn.makeFocusable(1)
    successBtn.setAttribute('data-action', 'success-toast')

    // Info toast button
    const infoBtn = TuiUtils.button()
    infoBtn.addClass('btn btn-secondary')
    infoBtn.setContent('ℹ️ Info Toast')
    infoBtn.makeFocusable(2)
    infoBtn.setAttribute('data-action', 'info-toast')

    // Warning toast button
    const warningBtn = TuiUtils.button()
    warningBtn.addClass('btn btn-danger')
    warningBtn.setContent('⚠️ Warning Toast')
    warningBtn.makeFocusable(3)
    warningBtn.setAttribute('data-action', 'warning-toast')

    // Theme toggle button
    const themeBtn = TuiUtils.button()
    themeBtn.addClass('btn btn-secondary')
    themeBtn.setContent('🎨 Toggle Theme')
    themeBtn.makeFocusable(4)
    themeBtn.setAttribute('data-action', 'toggle-theme')

    buttonGroup.addChild(successBtn)
    buttonGroup.addChild(infoBtn)
    buttonGroup.addChild(warningBtn)
    buttonGroup.addChild(themeBtn)

    return buttonGroup
  }

  /**
   * Create footer section
   */
  private createFooter(): JsElement {
    const footer = TuiUtils.div()
    footer.addClass('footer')

    const [width, height] = TuiUtils.getTerminalSize()
    footer.setContent(`Terminal: ${width}×${height} • Theme: ${this.currentTheme.getName()} • Made with ❤️ and TypeScript`)

    return footer
  }

  /**
   * Start the application
   */
  public start(): void {
    // Show welcome toast
    const welcomeToast = JsToast.info('Welcome to Reactive TUI TypeScript Demo!')
    welcomeToast.setTitle('🚀 Welcome')
    welcomeToast.setDuration(3000)
    this.toastManager.showToast(welcomeToast)

    // Start the application
    const status: string = this.app.start()
    console.log(`\n🎉 Application Status: ${status}`)
    console.log('📝 This demo showcases TypeScript integration with Reactive TUI')
    console.log('🔧 Full type safety, IntelliSense, and modern development workflow')
    console.log('🎨 CSS-first design with responsive layouts and theming')
    console.log('\n💡 Press Ctrl+C to exit\n')
  }
}

// Initialize and start the demo application
async function main(): Promise<void> {
  try {
    console.log('🚀 Starting Reactive TUI TypeScript Demo...\n')

    const demo = new ReactiveUIDemo()
    demo.start()

  } catch (error) {
    console.error('❌ Error starting demo:', error)
    process.exit(1)
  }
}

// Run the demo if this file is executed directly
if (import.meta.main) {
  await main()
}

export { ReactiveUIDemo }
