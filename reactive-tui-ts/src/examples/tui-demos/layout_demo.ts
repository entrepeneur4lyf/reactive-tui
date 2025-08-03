#!/usr/bin/env bun
/**
 * Layout Demo - TypeScript
 * 
 * Demonstrates various layout patterns using flexbox and grid
 */

import { 
  createApp, div, text, Component, flexColumn, flexRow, header, footer, main, section
} from '../../packages/tui-bun/src';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class LayoutDemo implements Component {
  private currentLayout = 0;
  private layouts = [
    {
      name: 'Vertical Layout',
      description: 'Header, content, footer',
      render: () => this.verticalLayout()
    },
    {
      name: 'Horizontal Layout',
      description: 'Sidebar, main, aside',
      render: () => this.horizontalLayout()
    },
    {
      name: 'Holy Grail Layout',
      description: 'Classic web layout pattern',
      render: () => this.holyGrailLayout()
    },
    {
      name: 'Dashboard Layout',
      description: 'Complex dashboard structure',
      render: () => this.dashboardLayout()
    },
    {
      name: 'Split Pane Layout',
      description: 'Two-column split view',
      render: () => this.splitPaneLayout()
    }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'ArrowRight':
      case 'Tab':
        this.currentLayout = (this.currentLayout + 1) % this.layouts.length;
        return true;
      
      case 'ArrowLeft':
        this.currentLayout = (this.currentLayout - 1 + this.layouts.length) % this.layouts.length;
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  verticalLayout(): Element {
    return flexColumn([
      // Header
      header({ class: 'bg-blue-800 text-white p-4' })
        .child(text('Header (fixed height)')),
      
      // Content
      main({ class: 'flex-1 bg-gray-700 p-4 overflow-auto' })
        .children([
          text('Main Content Area', { class: 'text-xl mb-4' }),
          text('This area expands to fill available space'),
          section({ class: 'mt-4 p-4 bg-gray-800 rounded' })
            .child(text('Nested content block'))
        ]),
      
      // Footer
      footer({ class: 'bg-gray-800 text-gray-400 p-2' })
        .child(text('Footer (fixed height)'))
    ], { class: 'h-full' })
    .build();
  }

  horizontalLayout(): Element {
    return flexRow([
      // Sidebar
      section({ class: 'w-48 bg-gray-800 p-4' })
        .children([
          text('Sidebar', { class: 'text-lg mb-4' }),
          flexColumn([
            text('• Menu Item 1'),
            text('• Menu Item 2'),
            text('• Menu Item 3')
          ], { class: 'space-y-2' })
        ]),
      
      // Main content
      main({ class: 'flex-1 bg-gray-700 p-4' })
        .children([
          text('Main Content', { class: 'text-xl mb-4' }),
          text('Flexible width content area')
        ]),
      
      // Aside
      section({ class: 'w-64 bg-gray-800 p-4' })
        .children([
          text('Aside', { class: 'text-lg mb-4' }),
          text('Additional information')
        ])
    ], { class: 'h-full' })
    .build();
  }

  holyGrailLayout(): Element {
    return flexColumn([
      // Header
      header({ class: 'bg-blue-800 text-white p-4' })
        .child(text('Header')),
      
      // Middle section
      flexRow([
        // Left sidebar
        section({ class: 'w-48 bg-gray-800 p-4' })
          .child(text('Left Sidebar')),
        
        // Main content
        main({ class: 'flex-1 bg-gray-700 p-4' })
          .children([
            text('Main Content', { class: 'text-xl mb-4' }),
            text('The holy grail layout is a classic web design pattern')
          ]),
        
        // Right sidebar
        section({ class: 'w-48 bg-gray-800 p-4' })
          .child(text('Right Sidebar'))
      ], { class: 'flex-1' }),
      
      // Footer
      footer({ class: 'bg-gray-800 text-gray-400 p-2' })
        .child(text('Footer'))
    ], { class: 'h-full' })
    .build();
  }

  dashboardLayout(): Element {
    return div({ class: 'flex flex-col h-full' })
      .children([
        // Top bar
        div({ class: 'bg-gray-800 p-2 flex justify-between' })
          .children([
            text('Dashboard'),
            text('User: admin')
          ]),
        
        // Main area
        div({ class: 'flex-1 flex' })
          .children([
            // Navigation
            div({ class: 'w-16 bg-gray-900 p-2' })
              .children([
                div({ class: 'space-y-4 text-center' })
                  .children(['📊', '📈', '⚙️', '👤'].map(icon => 
                    div({ class: 'p-2 hover:bg-gray-800 rounded' })
                      .child(text(icon))
                  ))
              ]),
            
            // Content area
            div({ class: 'flex-1 p-4 bg-gray-700' })
              .children([
                // Stats row
                div({ class: 'grid grid-cols-4 gap-4 mb-4' })
                  .children([1, 2, 3, 4].map(i => 
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        text(`Metric ${i}`, { class: 'text-sm text-gray-400' }),
                        text(`${i * 123}`, { class: 'text-2xl' })
                      ])
                  )),
                
                // Main content
                div({ class: 'bg-gray-800 rounded p-4' })
                  .child(text('Main dashboard content'))
              ])
          ])
      ])
      .build();
  }

  splitPaneLayout(): Element {
    return div({ class: 'flex h-full' })
      .children([
        // Left pane
        div({ class: 'flex-1 bg-gray-800 p-4 border-r border-gray-700' })
          .children([
            text('Left Pane', { class: 'text-lg mb-4' }),
            div({ class: 'bg-gray-900 rounded p-4' })
              .child(text('Content for left side'))
          ]),
        
        // Right pane
        div({ class: 'flex-1 bg-gray-800 p-4' })
          .children([
            text('Right Pane', { class: 'text-lg mb-4' }),
            div({ class: 'bg-gray-900 rounded p-4' })
              .child(text('Content for right side'))
          ])
      ])
      .build();
  }

  render(): Element {
    const layout = this.layouts[this.currentLayout];
    
    return flexColumn([
      // Demo header
      header({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
        .children([
          text('📐 Layout Demo', { class: 'text-xl font-bold mb-2' }),
          flexRow([
            text(`${layout.name}: ${layout.description}`, { class: 'text-gray-400' }),
            text(`${this.currentLayout + 1}/${this.layouts.length}`, { class: 'text-gray-500' })
          ], { class: 'justify-between items-center' })
        ]),
      
      // Layout content
      main({ class: 'flex-1 overflow-hidden' })
        .child(layout.render()),
      
      // Controls
      footer({ class: 'bg-gray-800 p-2 border-t border-gray-700 text-center text-sm text-gray-400' })
        .child(text('[←→/Tab] Change Layout | [Q] Quit'))
    ], { class: 'h-full bg-gray-900 text-white' })
    .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new LayoutDemo().render()
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}