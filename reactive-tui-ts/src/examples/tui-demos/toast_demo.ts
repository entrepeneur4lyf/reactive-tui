#!/usr/bin/env bun
/**
 * Toast Widget Demo - TypeScript
 * 
 * Demonstrates toast notifications with various styles and positions
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  ToastPosition,
  ToastVariant,
  infoToast,
  successToast,
  warningToast,
  errorToast
} from '../../packages/tui-bun/src/widgets/toast';
import { button } from '../../packages/tui-bun/src/widgets/button';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class ToastDemo implements Component {
  private toasts: Array<{
    id: string;
    element: Element;
    timestamp: number;
  }> = [];
  
  private nextId = 1;
  private autoToastEnabled = false;
  private lastAutoToast = 0;

  constructor() {
    // Start with a welcome toast
    this.addToast(
      infoToast({
        message: 'Welcome to the Toast Demo! 🎉',
        position: ToastPosition.TopCenter
      })
    );
  }

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case '1':
        this.addToast(
          successToast({
            message: 'Success! Operation completed.',
            position: ToastPosition.TopRight
          })
        );
        return true;
      
      case '2':
        this.addToast(
          errorToast({
            message: 'Error: Failed to save file.',
            position: ToastPosition.TopRight
          })
        );
        return true;
      
      case '3':
        this.addToast(
          warningToast({
            message: 'Warning: Low disk space.',
            position: ToastPosition.TopRight
          })
        );
        return true;
      
      case '4':
        this.addToast(
          infoToast({
            message: 'Info: New update available.',
            position: ToastPosition.TopRight
          })
        );
        return true;
      
      case '5':
        // Long toast
        this.addToast(
          infoToast({
            message: 'This is a very long toast message that demonstrates how the toast widget handles lengthy content with proper wrapping and maintains readability.',
            position: ToastPosition.BottomCenter
          })
        );
        return true;
      
      case 'a':
        this.autoToastEnabled = !this.autoToastEnabled;
        this.addToast(
          infoToast({
            message: `Auto toast ${this.autoToastEnabled ? 'enabled' : 'disabled'}`,
            position: ToastPosition.TopCenter
          })
        );
        return true;
      
      case 'c':
        this.toasts = [];
        return true;
      
      case 'ArrowUp':
        this.addToast(
          infoToast({
            message: 'Top position',
            position: ToastPosition.TopCenter
          })
        );
        return true;
      
      case 'ArrowDown':
        this.addToast(
          infoToast({
            message: 'Bottom position',
            position: ToastPosition.BottomCenter
          })
        );
        return true;
      
      case 'ArrowLeft':
        this.addToast(
          infoToast({
            message: 'Left position',
            position: ToastPosition.TopLeft
          })
        );
        return true;
      
      case 'ArrowRight':
        this.addToast(
          infoToast({
            message: 'Right position',
            position: ToastPosition.TopRight
          })
        );
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  addToast(toastElement: any) {
    const id = `toast-${this.nextId++}`;
    this.toasts.push({
      id,
      element: toastElement.id(id).build(),
      timestamp: Date.now()
    });
    
    // Auto-remove after 5 seconds
    setTimeout(() => {
      this.toasts = this.toasts.filter(t => t.id !== id);
    }, 5000);
  }

  update() {
    // Clean up old toasts
    const now = Date.now();
    this.toasts = this.toasts.filter(t => now - t.timestamp < 5000);
    
    // Auto toast feature
    if (this.autoToastEnabled && now - this.lastAutoToast > 3000) {
      const messages = [
        'Background task completed',
        'File saved successfully',
        'Connection established',
        'Sync completed',
        'Update downloaded'
      ];
      const variants = [ToastVariant.Success, ToastVariant.Error, ToastVariant.Warning, ToastVariant.Info];
      const positions = [
        ToastPosition.TopLeft, 
        ToastPosition.TopCenter, 
        ToastPosition.TopRight, 
        ToastPosition.BottomLeft, 
        ToastPosition.BottomCenter, 
        ToastPosition.BottomRight
      ];
      
      const message = messages[Math.floor(Math.random() * messages.length)];
      const variant = variants[Math.floor(Math.random() * variants.length)];
      const position = positions[Math.floor(Math.random() * positions.length)];
      
      let toastBuilder;
      switch (variant) {
        case ToastVariant.Success:
          toastBuilder = successToast({ message, position });
          break;
        case ToastVariant.Error:
          toastBuilder = errorToast({ message, position });
          break;
        case ToastVariant.Warning:
          toastBuilder = warningToast({ message, position });
          break;
        default:
          toastBuilder = infoToast({ message, position });
      }
      
      this.addToast(toastBuilder);
      this.lastAutoToast = now;
    }
  }

  render(): Element {
    this.update();
    
    return div({ class: 'relative h-full bg-gray-900 text-white' })
      .children([
        // Main content
        div({ class: 'flex flex-col h-full' })
          .children([
            // Header
            div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
              .children([
                text('🍞 Toast Notification Demo', { class: 'text-2xl font-bold mb-2' }),
                text(this.autoToastEnabled ? '🔄 Auto Toast: ON' : 'Auto Toast: OFF', 
                  { class: 'text-gray-400' })
              ]),
            
            // Content area
            div({ class: 'flex-1 p-8 overflow-auto' })
              .children([
                // Instructions
                div({ class: 'mb-8' })
                  .children([
                    text('Toast Types', { class: 'text-xl font-bold mb-4' }),
                    div({ class: 'grid grid-cols-2 gap-4 mb-6' })
                      .children([
                        button({
                          id: 'success-btn',
                          text: '[1] Success Toast',
                          color: 'success',
                          onClick: () => this.addToast(successToast({ 
                            message: 'Success!', 
                            position: ToastPosition.TopRight 
                          }))
                        }),
                        button({
                          id: 'error-btn',
                          text: '[2] Error Toast',
                          color: 'error',
                          onClick: () => this.addToast(errorToast({ 
                            message: 'Error!', 
                            position: ToastPosition.TopRight 
                          }))
                        }),
                        button({
                          id: 'warning-btn',
                          text: '[3] Warning Toast',
                          color: 'warning',
                          onClick: () => this.addToast(warningToast({ 
                            message: 'Warning!', 
                            position: ToastPosition.TopRight 
                          }))
                        }),
                        button({
                          id: 'info-btn',
                          text: '[4] Info Toast',
                          color: 'info',
                          onClick: () => this.addToast(infoToast({ 
                            message: 'Info!', 
                            position: ToastPosition.TopRight 
                          }))
                        })
                      ])
                  ]),
                
                // Position demo
                div({ class: 'mb-8' })
                  .children([
                    text('Toast Positions', { class: 'text-xl font-bold mb-4' }),
                    text('Use arrow keys to show toasts in different positions:', 
                      { class: 'text-gray-400 mb-4' }),
                    
                    // Visual position guide
                    div({ class: 'relative bg-gray-800 rounded h-64 p-4' })
                      .children([
                        // Corners and edges
                        div({ class: 'absolute top-4 left-4 text-sm text-gray-500' })
                          .child(text('↖ Top Left')),
                        div({ class: 'absolute top-4 left-1/2 -translate-x-1/2 text-sm text-gray-500' })
                          .child(text('↑ Top Center')),
                        div({ class: 'absolute top-4 right-4 text-sm text-gray-500' })
                          .child(text('Top Right ↗')),
                        
                        div({ class: 'absolute bottom-4 left-4 text-sm text-gray-500' })
                          .child(text('↙ Bottom Left')),
                        div({ class: 'absolute bottom-4 left-1/2 -translate-x-1/2 text-sm text-gray-500' })
                          .child(text('↓ Bottom Center')),
                        div({ class: 'absolute bottom-4 right-4 text-sm text-gray-500' })
                          .child(text('Bottom Right ↘'))
                      ])
                  ]),
                
                // Features
                div({ class: 'mb-8' })
                  .children([
                    text('Features', { class: 'text-xl font-bold mb-4' }),
                    div({ class: 'space-y-2 text-gray-300' })
                      .children([
                        text('• Multiple toast types with icons'),
                        text('• 6 different positions'),
                        text('• Auto-dismiss after 5 seconds'),
                        text('• Stack multiple toasts'),
                        text('• Smooth animations'),
                        text('• Responsive to terminal size'),
                        text('• [5] Long message demo'),
                        text('• [A] Toggle auto toast'),
                        text('• [C] Clear all toasts')
                      ])
                  ]),
                
                // Active toasts counter
                div({ class: 'bg-gray-800 rounded p-4' })
                  .children([
                    text(`Active Toasts: ${this.toasts.length}`, { class: 'text-lg' })
                  ])
              ]),
            
            // Footer
            div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
              .children([
                text('[1-4] Toast Types | [↑↓←→] Positions | [5] Long | [A] Auto | [C] Clear | [Q] Quit', 
                  { class: 'text-center text-sm text-gray-400' })
              ])
          ]),
        
        // Toast container - overlay
        div({ class: 'absolute inset-0 pointer-events-none' })
          .children(this.toasts.map(t => t.element))
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new ToastDemo().render(),
    // Uses full terminal by default
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}