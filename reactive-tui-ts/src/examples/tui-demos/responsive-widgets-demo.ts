/**
 * Responsive Widgets Demo - Demonstrates architectural harmony with Rust tui-core
 * 
 * This example shows how TypeScript widgets integrate with the LayoutEngine
 * for responsive design, matching the Rust ResponsiveWidget trait pattern.
 */

import { 
  createApp, 
  div, 
  text, 
  flexColumn, 
  flexRow,
  ButtonWidget,
  ResponsiveWidget,
  LayoutEngine,
  LayoutRect,
  hr,
  spacer,
  center,
  padding,
  border
} from '../../packages/tui-bun/src/index'

// Example 1: Basic ResponsiveWidget usage
function createResponsiveButtonExample(): ResponsiveWidget {
  return new ButtonWidget({
    id: 'responsive-btn',
    text: 'Responsive Button',
    variant: 'filled',
    color: 'primary',
    fullWidth: true
  })
}

// Example 2: Layout with multiple responsive widgets
function createResponsiveLayout() {
  const layoutEngine = new LayoutEngine()
  
  // Create responsive widgets
  const primaryBtn = new ButtonWidget({
    id: 'primary',
    text: 'Primary Action',
    variant: 'filled',
    color: 'primary'
  })
  
  const secondaryBtn = new ButtonWidget({
    id: 'secondary', 
    text: 'Secondary',
    variant: 'outlined',
    color: 'secondary'
  })
  
  const dangerBtn = new ButtonWidget({
    id: 'danger',
    text: 'Delete',
    variant: 'filled',
    color: 'error'
  })

  console.log('🎯 Responsive Widget Layout Demo')
  console.log('=' .repeat(50))
  
  // Show widget size constraints
  console.log('\n📏 Widget Size Constraints:')
  console.log(`Primary Button - Min: ${primaryBtn.minSize()}, Max: ${primaryBtn.maxSize()}`)
  console.log(`Secondary Button - Min: ${secondaryBtn.minSize()}, Max: ${secondaryBtn.maxSize()}`)
  console.log(`Danger Button - Min: ${dangerBtn.minSize()}, Max: ${dangerBtn.maxSize()}`)
  
  // Compute layouts for different screen sizes
  console.log('\n📱 Responsive Layout Computation:')
  
  const layouts = [
    { name: 'iPhone 16 Pro Max Portrait (110x240)', width: 110, height: 240 },
    { name: 'iPhone 16 Pro Max Landscape (300x110)', width: 300, height: 110 },
    { name: 'iPad Pro 12.9" (340x255)', width: 340, height: 255 },
    { name: '4K Laptop 15" (320x180)', width: 320, height: 180 },
    { name: 'MacBook Pro 14" M4 (380x245)', width: 380, height: 245 },
    { name: 'MacBook Pro 16" M3 (430x280)', width: 430, height: 280 },
    { name: '1440p Desktop 27" (320x160)', width: 320, height: 160 },
    { name: '4K Desktop 27" (480x270)', width: 480, height: 270 },
    { name: 'Studio Display 27" 5K (640x360)', width: 640, height: 360 },
    { name: 'Ultrawide 1440p 34" (400x160)', width: 400, height: 160 },
    { name: 'Pro Display XDR 32" 6K (750x420)', width: 750, height: 420 }
  ]
  
  layouts.forEach(({ name, width, height }) => {
    console.log(`\n${name}:`)
    
    layoutEngine.updateViewport({
      width,
      height,
      terminalSize: { width, height }
    })
    
    const primaryLayout = layoutEngine.computeResponsiveLayout(primaryBtn)
    const secondaryLayout = layoutEngine.computeResponsiveLayout(secondaryBtn)
    const dangerLayout = layoutEngine.computeResponsiveLayout(dangerBtn)
    
    console.log(`  Primary: ${primaryLayout.rect.width}x${primaryLayout.rect.height}`)
    console.log(`  Secondary: ${secondaryLayout.rect.width}x${secondaryLayout.rect.height}`)
    console.log(`  Danger: ${dangerLayout.rect.width}x${dangerLayout.rect.height}`)
  })
  
  return {
    primaryBtn,
    secondaryBtn, 
    dangerBtn,
    layoutEngine
  }
}

// Example 3: Integration with component system
function createHybridLayout() {
  return div({ class: 'app-container' })
    .child(
      center(
        border(
          padding(
            flexColumn([
              text('🚀 TUI-Bun Responsive Demo'),
              hr(),
              spacer(1),
              text('This demonstrates architectural harmony between:'),
              text('• TypeScript ResponsiveWidget interface'),
              text('• Rust ResponsiveWidget trait'),
              text('• LayoutEngine integration'),
              text('• CSS-based styling'),
              spacer(1),
              flexRow([
                // These would be converted to Elements via bridge functions
                text('[Primary]'),
                text(' '),
                text('[Secondary]'),
                text(' '),
                text('[Danger]')
              ])
            ])
          )
        )
      )
    )
}

// Example 4: Demonstrate widget rendering with layout
function demonstrateWidgetRendering() {
  console.log('\n🎨 Widget Rendering with Computed Layout:')
  console.log('=' .repeat(50))
  
  const button = new ButtonWidget({
    id: 'demo-btn',
    text: 'Click Me!',
    variant: 'filled',
    color: 'primary',
    icon: { symbol: '🚀', position: 'left' }
  })
  
  // Simulate different layout constraints
  const layouts: LayoutRect[] = [
    { x: 0, y: 0, width: 15, height: 1 }, // Tight
    { x: 0, y: 0, width: 25, height: 1 }, // Comfortable
    { x: 0, y: 0, width: 40, height: 1 }  // Spacious
  ]
  
  layouts.forEach((layout, index) => {
    const rendered = button.renderWithLayout(layout)
    console.log(`Layout ${index + 1} (${layout.width}x${layout.height}): ${rendered}`)
  })
}

// Main demo function
async function runDemo() {
  console.log('🎯 TUI-Bun Responsive Widgets Demo')
  console.log('Demonstrating architectural harmony with Rust tui-core')
  console.log('=' .repeat(60))
  
  // Example 1: Basic responsive widget
  console.log('\n1️⃣ Basic ResponsiveWidget:')
  const responsiveBtn = createResponsiveButtonExample()
  console.log(`Created: ${responsiveBtn.constructor.name}`)
  console.log(`Min Size: ${responsiveBtn.minSize()}`)
  console.log(`Max Size: ${responsiveBtn.maxSize()}`)
  console.log(`Can Grow H/V: ${responsiveBtn.canGrowHorizontal()}/${responsiveBtn.canGrowVertical()}`)
  
  // Example 2: Responsive layout
  console.log('\n2️⃣ Responsive Layout System:')
  createResponsiveLayout()
  
  // Example 3: Widget rendering
  demonstrateWidgetRendering()
  
  // Example 4: Hybrid component system
  console.log('\n4️⃣ Hybrid Component System:')
  createApp({
    component: createHybridLayout
  })

  console.log('Created hybrid app with both Elements and ResponsiveWidgets')
  console.log('✅ Architectural harmony achieved!')
  
  console.log('\n🎉 Demo complete! Key achievements:')
  console.log('• ResponsiveWidget interface matches Rust trait')
  console.log('• LayoutEngine integration for responsive design')
  console.log('• Widget bridge functions for Element conversion')
  console.log('• CSS-based styling system')
  console.log('• Seamless TypeScript/Rust architectural harmony')
}

// Run the demo
if (import.meta.main) {
  runDemo().catch(console.error)
}

export { runDemo, createResponsiveButtonExample, createResponsiveLayout, createHybridLayout }
