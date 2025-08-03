/**
 * Example applications and components for Reactive TUI TypeScript
 */

import {
  JsTuiApp,
  TuiUtils
} from 'reactive-tui'
import { TypeScriptTuiUtils, TuiAppBuilder, ToastUtils } from './typescript-examples'
import { generateCompleteTheme } from './utils'

/**
 * Simple Hello World example
 */
export function createHelloWorldExample(): JsTuiApp {
  const app = new TuiAppBuilder('Hello World Example')
    .loadCSS(`
      .container {
        background: #0d1117;
        color: #f0f6fc;
        padding: 2rem;
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        min-height: 100vh;
      }

      .title {
        font-size: 2rem;
        font-weight: bold;
        margin-bottom: 1rem;
        color: #58a6ff;
      }

      .subtitle {
        color: #8b949e;
        text-align: center;
      }
    `)

  const container = TuiUtils.div()
  container.addClass('container')

  const title = TuiUtils.div()
  title.addClass('title')
  title.setContent('🚀 Hello, Reactive TUI!')

  const subtitle = TuiUtils.div()
  subtitle.addClass('subtitle')
  subtitle.setContent('Your first TypeScript terminal application')

  container.addChild(title)
  container.addChild(subtitle)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Interactive button example
 */
export function createButtonExample(): JsTuiApp {
  const app = new TuiAppBuilder('Button Example')
    .loadCSS(generateCompleteTheme())

  const container = TuiUtils.div()
  container.addClass('app-container flex flex-col items-center justify-center')

  const title = TuiUtils.div()
  title.setContent('🎮 Interactive Buttons')
  title.addClass('text-2xl font-bold mb-6 text-accent')

  const buttonGroup = TuiUtils.div()
  buttonGroup.addClass('flex gap-4')

  // Create different button types
  const primaryBtn = TypeScriptTuiUtils.createButton('Primary', 'primary')
  const secondaryBtn = TypeScriptTuiUtils.createButton('Secondary', 'secondary')
  const dangerBtn = TypeScriptTuiUtils.createButton('Danger', 'danger')

  buttonGroup.addChild(primaryBtn)
  buttonGroup.addChild(secondaryBtn)
  buttonGroup.addChild(dangerBtn)

  container.addChild(title)
  container.addChild(buttonGroup)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Card layout example
 */
export function createCardExample(): JsTuiApp {
  const app = new TuiAppBuilder('Card Layout Example')
    .loadCSS(generateCompleteTheme())

  const container = TuiUtils.div()
  container.addClass('app-container p-8')

  const header = TuiUtils.div()
  header.setContent('📋 Card Layout Demo')
  header.addClass('text-2xl font-bold mb-6 text-center')

  const cardGrid = TuiUtils.div()
  cardGrid.addClass('grid grid-cols-3 gap-6')

  // Create sample cards
  const cards = [
    { title: '🚀 Performance', content: 'Lightning-fast rendering with Rust-powered backend' },
    { title: '🎨 Styling', content: 'Modern CSS with responsive design and theming' },
    { title: '🔧 TypeScript', content: 'Full type safety with IntelliSense support' },
    { title: '📱 Responsive', content: 'Adaptive layouts for different terminal sizes' },
    { title: '🧩 Components', content: 'Rich widget library with 25+ components' },
    { title: '⚡ Hot Reload', content: 'Live development with instant CSS updates' }
  ]

  cards.forEach(({ title, content }) => {
    const card = TypeScriptTuiUtils.createCard(title, content)
    cardGrid.addChild(card)
  })

  container.addChild(header)
  container.addChild(cardGrid)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Status dashboard example
 */
export function createDashboardExample(): JsTuiApp {
  const app = new TuiAppBuilder('Status Dashboard')
    .loadCSS(generateCompleteTheme() + `
      .dashboard-header {
        background: var(--color-bg-secondary);
        border-bottom: 1px solid var(--color-border);
        padding: 1rem 2rem;
      }

      .dashboard-main {
        padding: 2rem;
      }

      .metrics-grid {
        display: grid;
        grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
        gap: 1.5rem;
        margin-bottom: 2rem;
      }

      .metric-card {
        background: var(--color-bg-secondary);
        border: 1px solid var(--color-border);
        border-radius: 8px;
        padding: 1.5rem;
        text-align: center;
      }

      .metric-value {
        font-size: 2rem;
        font-weight: bold;
        color: var(--color-accent);
        margin-bottom: 0.5rem;
      }

      .metric-label {
        color: var(--color-text-secondary);
        font-size: 0.875rem;
      }
    `)

  const container = TuiUtils.div()
  container.addClass('app-container')

  // Header
  const header = TuiUtils.div()
  header.addClass('dashboard-header')
  header.setContent('📊 System Dashboard - Real-time Monitoring')

  // Main content
  const main = TuiUtils.div()
  main.addClass('dashboard-main')

  // Metrics grid
  const metricsGrid = TuiUtils.div()
  metricsGrid.addClass('metrics-grid')

  const metrics = [
    { label: 'CPU Usage', value: '45%', status: 'success' },
    { label: 'Memory', value: '67%', status: 'warning' },
    { label: 'Disk I/O', value: '23%', status: 'success' },
    { label: 'Network', value: '89%', status: 'error' },
    { label: 'Uptime', value: '99.9%', status: 'success' },
    { label: 'Load Avg', value: '1.2', status: 'info' }
  ]

  metrics.forEach(({ label, value, status }) => {
    const metric = TypeScriptTuiUtils.createStatusIndicator(label, value, status as any)
    metric.addClass('metric-card')
    metricsGrid.addChild(metric)
  })

  // Activity log
  const logSection = TuiUtils.div()
  logSection.addClass('mt-6')

  const logTitle = TuiUtils.div()
  logTitle.setContent('📝 Recent Activity')
  logTitle.addClass('text-lg font-bold mb-4')

  const logContainer = TuiUtils.div()
  logContainer.addClass('card')

  const logEntries = [
    '✅ System backup completed successfully',
    '⚠️  High memory usage detected on server-02',
    '🔄 Database maintenance scheduled for 2:00 AM',
    '📈 Performance metrics updated',
    '🔒 Security scan completed - no issues found'
  ]

  logEntries.forEach(entry => {
    const logEntry = TuiUtils.div()
    logEntry.setContent(entry)
    logEntry.addClass('p-2 border-b border-gray-700 last:border-b-0')
    logContainer.addChild(logEntry)
  })

  logSection.addChild(logTitle)
  logSection.addChild(logContainer)

  main.addChild(metricsGrid)
  main.addChild(logSection)

  container.addChild(header)
  container.addChild(main)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Toast notification example
 */
export function createToastExample(): JsTuiApp {
  const app = new TuiAppBuilder('Toast Notifications')
    .loadCSS(generateCompleteTheme())

  const container = TuiUtils.div()
  container.addClass('app-container flex flex-col items-center justify-center')

  const title = TuiUtils.div()
  title.setContent('🍞 Toast Notifications Demo')
  title.addClass('text-2xl font-bold mb-6')

  const description = TuiUtils.div()
  description.setContent('Click buttons to see different types of toast notifications')
  description.addClass('text-secondary mb-8 text-center')

  const buttonGrid = TuiUtils.div()
  buttonGrid.addClass('grid grid-cols-2 gap-4')

  // Initialize toast system
  ToastUtils.initialize()

  // Create toast trigger buttons
  const infoBtn = TypeScriptTuiUtils.createButton('ℹ️ Info Toast', 'secondary')
  infoBtn.setAttribute('data-toast-type', 'info')

  const successBtn = TypeScriptTuiUtils.createButton('✅ Success Toast', 'primary')
  successBtn.setAttribute('data-toast-type', 'success')

  const warningBtn = TypeScriptTuiUtils.createButton('⚠️ Warning Toast', 'secondary')
  warningBtn.setAttribute('data-toast-type', 'warning')

  const errorBtn = TypeScriptTuiUtils.createButton('❌ Error Toast', 'danger')
  errorBtn.setAttribute('data-toast-type', 'error')

  buttonGrid.addChild(infoBtn)
  buttonGrid.addChild(successBtn)
  buttonGrid.addChild(warningBtn)
  buttonGrid.addChild(errorBtn)

  const instructions = TuiUtils.div()
  instructions.setContent('💡 In a real application, these buttons would trigger toast notifications')
  instructions.addClass('text-muted text-center mt-6 text-sm')

  container.addChild(title)
  container.addChild(description)
  container.addChild(buttonGrid)
  container.addChild(instructions)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Theme showcase example
 */
export function createThemeExample(): JsTuiApp {
  const app = new TuiAppBuilder('Theme Showcase')
    .loadCSS(generateCompleteTheme())

  const container = TuiUtils.div()
  container.addClass('app-container p-8')

  const header = TuiUtils.div()
  header.setContent('🎨 Theme System Demo')
  header.addClass('text-2xl font-bold mb-6 text-center')

  const themeGrid = TuiUtils.div()
  themeGrid.addClass('grid grid-cols-3 gap-6')

  // Show different theme options
  const themes = [
    { name: 'Dark Theme', description: 'Modern dark interface with blue accents' },
    { name: 'Light Theme', description: 'Clean light interface for bright environments' },
    { name: 'Terminal Theme', description: 'Classic terminal colors and styling' }
  ]

  themes.forEach(({ name, description }) => {
    const themeCard = TypeScriptTuiUtils.createCard(name, description, 'theme-card')
    themeGrid.addChild(themeCard)
  })

  const colorPalette = TuiUtils.div()
  colorPalette.addClass('mt-8')

  const paletteTitle = TuiUtils.div()
  paletteTitle.setContent('🌈 Color Palette')
  paletteTitle.addClass('text-lg font-bold mb-4')

  const colorGrid = TuiUtils.div()
  colorGrid.addClass('grid grid-cols-6 gap-2')

  const colors = [
    { name: 'Primary', class: 'bg-accent' },
    { name: 'Success', class: 'bg-success' },
    { name: 'Warning', class: 'bg-warning' },
    { name: 'Error', class: 'bg-error' },
    { name: 'Secondary', class: 'bg-secondary' },
    { name: 'Muted', class: 'bg-muted' }
  ]

  colors.forEach(({ name, class: className }) => {
    const colorSwatch = TuiUtils.div()
    colorSwatch.addClass(`${className} p-4 rounded text-center text-sm`)
    colorSwatch.setContent(name)
    colorGrid.addChild(colorSwatch)
  })

  colorPalette.addChild(paletteTitle)
  colorPalette.addChild(colorGrid)

  container.addChild(header)
  container.addChild(themeGrid)
  container.addChild(colorPalette)

  app.setRoot(container)
  return app.getApp()
}

/**
 * Export all examples for easy access
 */
export const EXAMPLES = {
  helloWorld: createHelloWorldExample,
  buttons: createButtonExample,
  cards: createCardExample,
  dashboard: createDashboardExample,
  toasts: createToastExample,
  themes: createThemeExample
}

/**
 * Run a specific example by name
 */
export function runExample(name: keyof typeof EXAMPLES): void {
  const example = EXAMPLES[name]
  if (!example) {
    console.error(`Example "${name}" not found. Available examples:`, Object.keys(EXAMPLES))
    return
  }

  const app = example()
  const status = app.start()
  console.log(`Started ${name} example: ${status}`)
}
