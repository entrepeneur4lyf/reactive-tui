#!/usr/bin/env bun
/**
 * Button Widget Demo - TypeScript
 * 
 * Demonstrates all button variants, sizes, states, and configurations
 * Uses harmonized component architecture with ElementBuilder pattern
 */

import { 
  createApp, div, text, Component, flexColumn, flexRow, container, header, main, separator
} from '../../packages/tui-bun/src';
import { 
  primaryButton, secondaryButton, dangerButton
} from '../../packages/tui-bun/src/components';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class ButtonDemo implements Component {
  private clickCounts: Record<string, number> = {};
  private loadingStates: Record<string, boolean> = {};
  private toggleStates: Record<string, boolean> = {};
  private lastClicked = '';

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'l':
        // Toggle loading state on primary button
        this.loadingStates['primary'] = !this.loadingStates['primary'];
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  handleClick(id: string) {
    this.clickCounts[id] = (this.clickCounts[id] || 0) + 1;
    this.lastClicked = id;
    
    // Simulate async operation for some buttons
    if (id === 'async-button') {
      this.loadingStates[id] = true;
      setTimeout(() => {
        this.loadingStates[id] = false;
      }, 2000);
    }
  }

  render(): Element {
    return flexColumn([
      // Header section using semantic components
      header({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
        .child(text('🔘 Button Widget Demo', { class: 'text-2xl font-bold mb-2' }))
        .child(text('Comprehensive showcase of button variants and states', { class: 'text-gray-400' })),
        
        // Main content with scrollable area
        main({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Button Variants
            div({ class: 'mb-8' })
              .children([
                text('Button Variants', { class: 'text-xl font-bold mb-4' }),
                flexRow([
                  primaryButton('primary', 'Primary Button'),
                  secondaryButton('secondary', 'Secondary'),
                  dangerButton('danger', 'Danger')
                ], { class: 'gap-4' })
              ]),
            
            // Button States Demo
            div({ class: 'mb-8' })
              .children([
                text('Button State Demonstration', { class: 'text-xl font-bold mb-4' }),
                flexRow([
                  text(`Last clicked: ${this.lastClicked || 'None'}`, { class: 'text-gray-400' }),
                  text(`Clicks: ${this.clickCounts[this.lastClicked] || 0}`, { class: 'text-gray-500' })
                ], { class: 'justify-between items-center' })
              ])
          ]),
        
        // Status footer
        footer({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .child(
            flexRow([
              text('Click buttons to interact with them', { class: 'text-gray-400' }),
              text('[L] Toggle Loading | [Q] Quit', { class: 'text-gray-500' })
            ], { class: 'justify-between items-center' })
          )
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new ButtonDemo().render(),
    fullscreen: true
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}