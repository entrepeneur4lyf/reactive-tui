#!/usr/bin/env bun
/**
 * Spinner Widget Demo - TypeScript
 * 
 * Demonstrates various spinner types and loading indicators
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  spinnerWidget, createLoadingSpinner, createProcessingSpinner,
  createSavingSpinner, createCustomSpinner, createEmojiSpinner,
  createMinimalSpinner, createProgressSpinner, createBinarySpinner,
  SpinnerType, SPINNER_TYPES
} from '../../packages/tui-bun/src/widgets/spinner';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class SpinnerDemo implements Component {
  private activeSpinners: Set<string> = new Set(['loading', 'processing', 'saving']);
  private customFrame = 0;
  private downloadProgress = 0;
  private animationSpeed = 80;
  
  private operations = [
    { id: 'fetch', name: 'Fetching data...', active: false, type: SpinnerType.Dots },
    { id: 'compile', name: 'Compiling...', active: false, type: SpinnerType.Bars },
    { id: 'optimize', name: 'Optimizing...', active: false, type: SpinnerType.Circle },
    { id: 'deploy', name: 'Deploying...', active: false, type: SpinnerType.Pulse }
  ];
  
  private animationFrame?: NodeJS.Timeout;

  constructor() {
    this.startAnimation();
  }

  startAnimation() {
    const animate = () => {
      this.customFrame = (this.customFrame + 1) % 8;
      this.downloadProgress = (this.downloadProgress + 2) % 101;
      
      // Randomly toggle operations
      if (Math.random() < 0.05) {
        const op = this.operations[Math.floor(Math.random() * this.operations.length)];
        op.active = !op.active;
      }
      
      this.animationFrame = setTimeout(animate, this.animationSpeed);
    };
    animate();
  }

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case '1':
      case '2':
      case '3':
      case '4':
        const index = parseInt(key.data.key) - 1;
        if (index < this.operations.length) {
          this.operations[index].active = !this.operations[index].active;
        }
        return true;
      
      case '+':
        this.animationSpeed = Math.max(20, this.animationSpeed - 20);
        return true;
      
      case '-':
        this.animationSpeed = Math.min(200, this.animationSpeed + 20);
        return true;
      
      case 'q':
      case 'Q':
        if (this.animationFrame) clearTimeout(this.animationFrame);
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
            text('⏳ Spinner Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text(`Animation Speed: ${this.animationSpeed}ms`, { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Basic Spinners
            div({ class: 'mb-8' })
              .children([
                text('Basic Spinners', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-3 gap-6' })
                  .children([
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        createLoadingSpinner({ 
                          active: this.activeSpinners.has('loading'),
                          text: 'Loading...'
                        })
                      ]),
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        createProcessingSpinner({ 
                          active: this.activeSpinners.has('processing'),
                          text: 'Processing...'
                        })
                      ]),
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        createSavingSpinner({ 
                          active: this.activeSpinners.has('saving'),
                          text: 'Saving...'
                        })
                      ])
                  ])
              ]),
            
            // Spinner Types
            div({ class: 'mb-8' })
              .children([
                text('All Spinner Types', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-4 gap-4' })
                  .children(Object.entries(SPINNER_TYPES).map(([name, frames]) => 
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        spinnerWidget({
                          type: name as SpinnerType,
                          active: true,
                          text: name,
                          color: 'primary'
                        })
                      ])
                  ))
              ]),
            
            // Colored Spinners
            div({ class: 'mb-8' })
              .children([
                text('Colored Spinners', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-6' })
                  .children([
                    spinnerWidget({
                      type: SpinnerType.Dots,
                      active: true,
                      text: 'Primary',
                      color: 'primary'
                    }),
                    spinnerWidget({
                      type: SpinnerType.Dots,
                      active: true,
                      text: 'Success',
                      color: 'success'
                    }),
                    spinnerWidget({
                      type: SpinnerType.Dots,
                      active: true,
                      text: 'Warning',
                      color: 'warning'
                    }),
                    spinnerWidget({
                      type: SpinnerType.Dots,
                      active: true,
                      text: 'Error',
                      color: 'error'
                    })
                  ])
              ]),
            
            // Emoji Spinners
            div({ class: 'mb-8' })
              .children([
                text('Fun Emoji Spinners', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-6' })
                  .children([
                    createEmojiSpinner({
                      frames: ['🌍', '🌎', '🌏'],
                      text: 'Earth',
                      active: true
                    }),
                    createEmojiSpinner({
                      frames: ['🌑', '🌒', '🌓', '🌔', '🌕', '🌖', '🌗', '🌘'],
                      text: 'Moon',
                      active: true
                    }),
                    createEmojiSpinner({
                      frames: ['🕐', '🕑', '🕒', '🕓', '🕔', '🕕', '🕖', '🕗', '🕘', '🕙', '🕚', '🕛'],
                      text: 'Clock',
                      active: true
                    })
                  ])
              ]),
            
            // Progress Spinner
            div({ class: 'mb-8' })
              .children([
                text('Progress Spinner', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'bg-gray-800 p-4 rounded' })
                  .children([
                    createProgressSpinner({
                      progress: this.downloadProgress,
                      text: `Downloading... ${this.downloadProgress}%`,
                      active: true
                    })
                  ])
              ]),
            
            // Binary Spinner
            div({ class: 'mb-8' })
              .children([
                text('Binary/Matrix Spinner', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'bg-gray-800 p-4 rounded' })
                  .children([
                    createBinarySpinner({
                      active: true,
                      text: 'Hacking the mainframe...'
                    })
                  ])
              ]),
            
            // Custom Spinner
            div({ class: 'mb-8' })
              .children([
                text('Custom Animation', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'bg-gray-800 p-4 rounded' })
                  .children([
                    createCustomSpinner({
                      frames: ['╔', '╗', '╝', '╚', '║', '═', '╬', '╣'],
                      text: 'Building...',
                      active: true,
                      frameIndex: this.customFrame
                    })
                  ])
              ]),
            
            // Operations Status
            div({ class: 'mb-8' })
              .children([
                text('Operations Status', { class: 'text-xl font-bold mb-4' }),
                text('Press [1-4] to toggle operations', { class: 'text-sm text-gray-400 mb-2' }),
                div({ class: 'space-y-2' })
                  .children(this.operations.map((op, index) => 
                    div({ class: 'bg-gray-800 p-3 rounded flex items-center justify-between' })
                      .children([
                        div({ class: 'flex items-center gap-4' })
                          .children([
                            text(`[${index + 1}]`, { class: 'text-gray-500' }),
                            spinnerWidget({
                              type: op.type,
                              active: op.active,
                              text: op.name
                            })
                          ]),
                        text(op.active ? 'Running' : 'Idle', 
                          { class: op.active ? 'text-green-400' : 'text-gray-500' })
                      ])
                  ))
              ]),
            
            // Minimal Spinners
            div({ class: 'mb-8' })
              .children([
                text('Minimal Spinners', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-4' })
                  .children([
                    createMinimalSpinner({ active: true }),
                    createMinimalSpinner({ active: true, size: 'sm' }),
                    createMinimalSpinner({ active: true, size: 'lg' })
                  ])
              ])
          ]),
        
        // Footer
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            text('[1-4] Toggle Operations | [+/-] Speed | [Q] Quit', 
              { class: 'text-center text-sm text-gray-400' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new SpinnerDemo().render(),
    fullscreen: true
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}