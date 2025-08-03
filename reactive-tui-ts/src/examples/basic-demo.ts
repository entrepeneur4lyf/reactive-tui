#!/usr/bin/env bun

/**
 * Basic CSS-styled TUI demo
 * 
 * This example showcases the revolutionary CSS-styled TUI framework
 * with familiar web development patterns applied to terminal interfaces.
 */

import { createApp, div, text, button } from '../packages/tui-bun/src'

// Create a simple demo showcasing advanced visual effects
function App() {
  return div({ class: 'app' }).children([
      // Header with gradient
      div({ class: 'header' })
        .child(text('🚀 CSS-Styled TUI Demo', { class: 'title linear-gradient-purple' })),
      
      // Main content
      div({ class: 'main' })
        .children([
          // Sidebar
          div({ class: 'sidebar' })
            .children([
              text('Navigation', { class: 'sidebar-title neon-glow' }),
              button({ 
                id: 'home-btn',
                cssClasses: ['nav-button', 'active'] 
              })
                .child(text('🏠 Home')),
              button({ 
                id: 'dashboard-btn',
                cssClasses: ['nav-button'] 
              })
                .child(text('📊 Dashboard')),
              button({ 
                id: 'settings-btn',
                cssClasses: ['nav-button'] 
              })
                .child(text('⚙️ Settings')),
            ]),
          
          // Content area
          div({ class: 'content' })
            .children([
              text('Welcome to Advanced TUI Effects!', { class: 'content-title radial-gradient-blue' }),
              text('Experience CSS styling in your terminal', { class: 'description glass-morphism' }),
              
              // Feature cards
              div({ class: 'features' })
                .children([
                  div({ class: 'feature-card gradient-card' })
                    .children([
                      text('🎨 Gradients', { class: 'feature-title' }),
                      text('24-bit RGB colors', { class: 'feature-desc' })
                    ]),
                  
                  div({ class: 'feature-card glass-morphism' })
                    .children([
                      text('✨ Glass Effects', { class: 'feature-title neon-glow' }),
                      text('Blur & transparency', { class: 'feature-desc' })
                    ]),
                  
                  div({ class: 'feature-card neon-rainbow' })
                    .children([
                      text('🌈 Neon Glow', { class: 'feature-title' }),
                      text('Animated rainbow', { class: 'feature-desc' })
                    ])
                ])
            ])
        ]),
      
      // Footer
      div({ class: 'footer' })
        .child(text('🎭 Advanced CSS Effects in Terminal', { class: 'text-shadow-glow' }))
    ])
}

// Run the application
const app = createApp({
  stylesheet: './examples/styles.css',
  component: App
})

console.log('🎉 Starting revolutionary CSS-styled TUI demo...')
await app.run()