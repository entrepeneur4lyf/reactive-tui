#!/usr/bin/env bun

/**
 * Interactive Layout Demo
 * 
 * Showcases the Tailwind-inspired layout system with navigation
 */

import {
  createApp, div, text, button, input, progress, checkbox,
  radio
} from '../../packages/tui-bun/src'

// Interactive Layout Demo with Tailwind-style classes
function LayoutDemo() {
  return div({ class: 'container h-screen flex flex-col' }).children([
      // Header - full width, centered content
      div({ class: 'header w-full h-3 flex justify-center items-center' })
        .child(text('🎨 Tailwind-Style Layout Demo', { class: 'title linear-gradient-purple' })),
      
      // Main content - flexible layout
      div({ class: 'flex flex-row h-full w-full' })
        .children([
          // Sidebar - fixed width with padding
          div({ class: 'w-1/4 h-full p-2 flex flex-col' })
            .children([
              text('Navigation', { class: 'sidebar-title neon-glow mb-2' }),
              
              // Navigation buttons - vertical stack with spacing
              div({ class: 'flex flex-col' })
                .children([
                  button({ id: 'dashboard-btn' })
                    .class('mb-1 p-1 w-full justify-start')
                    .child(text('🏠 Dashboard')),
                  button({ id: 'analytics-btn' })
                    .class('mb-1 p-1 w-full justify-start')
                    .child(text('📊 Analytics')),
                  button({ id: 'settings-btn' })
                    .class('mb-1 p-1 w-full justify-start')
                    .child(text('⚙️ Settings')),
                  button({ id: 'profile-btn' })
                    .class('mb-1 p-1 w-full justify-start')
                    .child(text('👤 Profile')),
                ]),
              
              // Settings section
              div({ class: 'mt-4 flex flex-col' })
                .children([
                  text('Preferences', { class: 'mb-1' }),
                  checkbox({ class: 'mb-1', checked: true, label: 'Dark mode' }),
                  checkbox({ class: 'mb-1', checked: false, label: 'Notifications' }),
                  checkbox({ class: 'mb-1', checked: true, label: 'Auto-save' }),
                ])
            ]),
          
          // Main content area - flexible width
          div({ class: 'w-3/4 h-full p-2 flex flex-col' })
            .children([
              // Content header
              div({ class: 'flex justify-between items-center mb-2' })
                .children([
                  text('Interactive Components', { class: 'content-title radial-gradient-blue' }),
                  text('Press Tab to navigate', { class: 'description' })
                ]),
              
              // Form section - two columns
              div({ class: 'flex flex-row w-full mb-4' })
                .children([
                  // Left column
                  div({ class: 'w-1/2 pr-2' })
                    .children([
                      text('User Information', { class: 'mb-1' }),
                      input({ id: 'name-input', type: 'text', placeholder: 'Enter name...' })
                        .class('mb-1 w-full'),
                      input({ id: 'email-input', type: 'text', placeholder: 'Enter email...' })
                        .class('mb-1 w-full'),
                      
                      text('Theme Selection', { class: 'mb-1 mt-2' }),
                      radio({ class: 'mb-1', checked: true, name: 'theme', value: 'dark', label: 'Dark Theme' }),
                      radio({ class: 'mb-1', checked: false, name: 'theme', value: 'light', label: 'Light Theme' }),
                      radio({ class: 'mb-1', checked: false, name: 'theme', value: 'auto', label: 'Auto Theme' }),
                    ]),
                  
                  // Right column
                  div({ class: 'w-1/2 pl-2' })
                    .children([
                      text('Progress Indicators', { class: 'mb-1' }),
                      progress({ class: 'mb-1', value: 85, max: 100, label: 'Upload' }),
                      progress({ class: 'mb-1', value: 42, max: 100, label: 'Processing' }),
                      progress({ class: 'mb-1', value: 100, max: 100, label: 'Complete' }),
                      
                      // Action buttons - horizontal layout
                      div({ class: 'flex flex-row mt-3 justify-between' })
                        .children([
                          button({ id: 'save-btn' })
                            .class('gradient-card px-2')
                            .child(text('Save')),
                          button({ id: 'cancel-btn' })
                            .class('glass-morphism px-2')
                            .child(text('Cancel')),
                          button({ id: 'apply-btn' })
                            .class('neon-rainbow px-2')
                            .child(text('Apply')),
                        ])
                    ])
                ]),
              
              // Status cards - responsive grid
              div({ class: 'flex flex-row justify-between w-full' })
                .children([
                  div({ class: 'w-1/3 p-1 mr-1' })
                    .children([
                      div({ class: 'gradient-card p-2 h-8 flex flex-col justify-center' })
                        .children([
                          text('Active Users', { class: 'feature-title' }),
                          text('1,234', { class: 'feature-desc' })
                        ])
                    ]),
                  
                  div({ class: 'w-1/3 p-1 mx-1' })
                    .children([
                      div({ class: 'glass-morphism p-2 h-8 flex flex-col justify-center' })
                        .children([
                          text('Revenue', { class: 'feature-title neon-glow' }),
                          text('$12,345', { class: 'feature-desc' })
                        ])
                    ]),
                  
                  div({ class: 'w-1/3 p-1 ml-1' })
                    .children([
                      div({ class: 'neon-rainbow p-2 h-8 flex flex-col justify-center' })
                        .children([
                          text('Growth', { class: 'feature-title' }),
                          text('+23%', { class: 'feature-desc' })
                        ])
                    ])
                ])
            ])
        ]),
      
      // Footer - full width, centered
      div({ class: 'footer w-full h-3 flex justify-center items-center' })
        .child(text('🚀 Tailwind-Style Terminal UI • Flexbox • Grid • Responsive', { class: 'text-shadow-glow' }))
    ])
}

// Run the interactive layout demo
const app = createApp({
  stylesheet: './examples/styles.css',
  component: LayoutDemo
})

console.log('🎨 Starting interactive Tailwind-style layout demo...')
console.log('Use arrow keys, Tab, or hjkl to navigate • Enter/Space to interact • q to quit')
await app.run()