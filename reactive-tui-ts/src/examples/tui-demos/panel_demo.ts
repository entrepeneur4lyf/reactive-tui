#!/usr/bin/env bun
/**
 * Panel Widget Demo - TypeScript
 * 
 * Demonstrates various panel layouts and configurations
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { panel } from '../../packages/tui-bun/src/widgets/panel';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class PanelDemo implements Component {
  private selectedPanel = 0;
  private panels = ['basic', 'styled', 'nested', 'grid'];
  private collapsedPanels: Set<string> = new Set();

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        this.selectedPanel = (this.selectedPanel + 1) % this.panels.length;
        return true;
      
      case '1':
      case '2':
      case '3':
      case '4':
        const index = parseInt(key.data.key) - 1;
        if (index < this.panels.length) {
          this.selectedPanel = index;
        }
        return true;
      
      case 'c':
        const currentPanel = this.panels[this.selectedPanel];
        if (this.collapsedPanels.has(currentPanel)) {
          this.collapsedPanels.delete(currentPanel);
        } else {
          this.collapsedPanels.add(currentPanel);
        }
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('📦 Panel Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text('Flexible container layouts with panels', { class: 'text-gray-400' })
          ]),
        
        // Main content
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Basic Panel
            div({ class: 'mb-8' })
              .children([
                text('Basic Panel', { class: 'text-xl font-bold mb-4' }),
                panel({
                  id: 'basic-panel',
                  title: 'Basic Panel',
                  children: [
                    text('This is a basic panel with default styling.'),
                    text('Panels are versatile containers for organizing content.', 
                      { class: 'mt-2 text-gray-300' })
                  ]
                })
              ]),
            
            // Styled Panels
            div({ class: 'mb-8' })
              .children([
                text('Styled Panels', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-3 gap-4' })
                  .children([
                    panel({
                      id: 'primary-panel',
                      title: 'Primary Panel',
                      color: 'primary',
                      children: [
                        text('A primary colored panel for main content.')
                      ]
                    }),
                    panel({
                      id: 'success-panel',
                      title: 'Success Panel',
                      color: 'success',
                      children: [
                        text('✅ Operation completed successfully!')
                      ]
                    }),
                    panel({
                      id: 'warning-panel',
                      title: 'Warning Panel',
                      color: 'warning',
                      children: [
                        text('⚠️ Please review before proceeding.')
                      ]
                    })
                  ])
              ]),
            
            // Panel with Actions
            div({ class: 'mb-8' })
              .children([
                text('Panel with Header Actions', { class: 'text-xl font-bold mb-4' }),
                panel({
                  id: 'action-panel',
                  title: 'User Profile',
                  headerActions: [
                    text('⚙️', { class: 'cursor-pointer hover:text-blue-400' }),
                    text('✏️', { class: 'cursor-pointer hover:text-blue-400' }),
                    text('🗑️', { class: 'cursor-pointer hover:text-red-400' })
                  ],
                  children: [
                    div({ class: 'flex items-center gap-4' })
                      .children([
                        div({ class: 'w-16 h-16 bg-gray-600 rounded-full flex items-center justify-center' })
                          .child(text('👤', { class: 'text-2xl' })),
                        div()
                          .children([
                            text('John Doe', { class: 'font-bold' }),
                            text('john.doe@example.com', { class: 'text-sm text-gray-400' }),
                            text('Administrator', { class: 'text-sm text-green-400' })
                          ])
                      ])
                  ]
                })
              ]),
            
            // Collapsible Panels
            div({ class: 'mb-8' })
              .children([
                text('Collapsible Panels', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-4' })
                  .children([
                    panel({
                      id: 'collapsible-1',
                      title: 'System Information',
                      collapsible: true,
                      collapsed: this.collapsedPanels.has('system'),
                      onToggle: () => {
                        if (this.collapsedPanels.has('system')) {
                          this.collapsedPanels.delete('system');
                        } else {
                          this.collapsedPanels.add('system');
                        }
                      },
                      children: [
                        text('CPU: Intel Core i7-9700K'),
                        text('RAM: 16GB DDR4'),
                        text('Storage: 512GB NVMe SSD'),
                        text('GPU: NVIDIA GTX 1660')
                      ]
                    }),
                    panel({
                      id: 'collapsible-2',
                      title: 'Network Status',
                      collapsible: true,
                      collapsed: this.collapsedPanels.has('network'),
                      onToggle: () => {
                        if (this.collapsedPanels.has('network')) {
                          this.collapsedPanels.delete('network');
                        } else {
                          this.collapsedPanels.add('network');
                        }
                      },
                      children: [
                        text('Status: Connected'),
                        text('IP: 192.168.1.100'),
                        text('Upload: 45.2 Mbps'),
                        text('Download: 125.8 Mbps')
                      ]
                    })
                  ])
              ]),
            
            // Nested Panels
            div({ class: 'mb-8' })
              .children([
                text('Nested Panels', { class: 'text-xl font-bold mb-4' }),
                panel({
                  id: 'outer-panel',
                  title: 'Dashboard',
                  children: [
                    div({ class: 'grid grid-cols-2 gap-4' })
                      .children([
                        panel({
                          id: 'stats-panel',
                          title: 'Statistics',
                          size: 'sm',
                          children: [
                            text('Users: 1,234'),
                            text('Revenue: $45,678'),
                            text('Growth: +12.5%')
                          ]
                        }),
                        panel({
                          id: 'activity-panel',
                          title: 'Recent Activity',
                          size: 'sm',
                          children: [
                            text('• User login: 2 min ago'),
                            text('• New order: 5 min ago'),
                            text('• System update: 1 hour ago')
                          ]
                        })
                      ])
                  ]
                })
              ]),
            
            // Panel Sizes
            div({ class: 'mb-8' })
              .children([
                text('Panel Sizes', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-4' })
                  .children([
                    panel({
                      id: 'sm-panel',
                      title: 'Small Panel',
                      size: 'sm',
                      children: [text('Compact panel with less padding')]
                    }),
                    panel({
                      id: 'md-panel',
                      title: 'Medium Panel (Default)',
                      size: 'md',
                      children: [text('Standard panel with normal padding')]
                    }),
                    panel({
                      id: 'lg-panel',
                      title: 'Large Panel',
                      size: 'lg',
                      children: [text('Spacious panel with extra padding')]
                    })
                  ])
              ]),
            
            // Panel without Title
            div({ class: 'mb-8' })
              .children([
                text('Panel without Title', { class: 'text-xl font-bold mb-4' }),
                panel({
                  id: 'no-title-panel',
                  children: [
                    text('This panel has no title bar.'),
                    text('Useful for simple content grouping.', { class: 'mt-2' })
                  ]
                })
              ]),
            
            // Custom Styled Panel
            div({ class: 'mb-8' })
              .children([
                text('Custom Styled Panel', { class: 'text-xl font-bold mb-4' }),
                panel({
                  id: 'custom-panel',
                  title: '🌟 Featured Content',
                  cssClasses: ['bg-gradient-to-r', 'from-purple-800', 'to-pink-800'],
                  children: [
                    text('This panel uses custom CSS classes for unique styling.'),
                    div({ class: 'mt-4 p-4 bg-black bg-opacity-30 rounded' })
                      .children([
                        text('✨ Gradient background'),
                        text('🎨 Custom colors'),
                        text('💫 Stand out from regular panels')
                      ])
                  ]
                })
              ])
          ]),
        
        // Footer
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            text('[1-4] Select Panel Type | [C] Toggle Collapse | [Tab] Navigate | [Q] Quit', 
              { class: 'text-center text-sm text-gray-400' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new PanelDemo().render(),
    // Uses full terminal by default
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}