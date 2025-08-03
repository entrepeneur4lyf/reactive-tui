#!/usr/bin/env bun
/**
 * Modal Demo - TypeScript
 * 
 * Demonstrates various modal types and configurations
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  modalWidget, alertModal, confirmModal, promptModal, customModal,
  createModalButton, createPrimaryButton, createSecondaryButton
} from '../../packages/tui-bun/src/widgets/modal';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class ModalDemo implements Component {
  private showModal = false;
  private currentModal = 0;
  private modalResult = '';
  
  private modals = [
    {
      name: 'Alert Modal',
      description: 'Simple alert with OK button',
      create: () => alertModal({
        title: '⚠️ Alert',
        message: 'This is an important alert message!',
        onClose: () => {
          this.modalResult = 'Alert closed';
          this.showModal = false;
        }
      })
    },
    {
      name: 'Confirm Modal',
      description: 'Confirmation dialog with Yes/No',
      create: () => confirmModal({
        title: '❓ Confirm Action',
        message: 'Are you sure you want to proceed?',
        onConfirm: () => {
          this.modalResult = 'Confirmed!';
          this.showModal = false;
        },
        onCancel: () => {
          this.modalResult = 'Cancelled';
          this.showModal = false;
        }
      })
    },
    {
      name: 'Prompt Modal',
      description: 'Input dialog for user text',
      create: () => promptModal({
        title: '📝 Enter Text',
        message: 'Please enter your name:',
        placeholder: 'John Doe',
        onSubmit: (value) => {
          this.modalResult = `You entered: ${value}`;
          this.showModal = false;
        },
        onCancel: () => {
          this.modalResult = 'Input cancelled';
          this.showModal = false;
        }
      })
    },
    {
      name: 'Custom Modal',
      description: 'Fully customized modal',
      create: () => customModal({
        title: '🎨 Custom Modal',
        size: 'large',
        position: 'center',
        content: div({ class: 'p-4' })
          .children([
            text('This is a custom modal with:', { class: 'mb-4' }),
            div({ class: 'space-y-2' })
              .children([
                text('• Custom content layout'),
                text('• Multiple buttons'),
                text('• Custom styling'),
                text('• Advanced features')
              ])
          ]).build(),
        buttons: [
          createPrimaryButton('Save', () => {
            this.modalResult = 'Saved!';
            this.showModal = false;
          }),
          createSecondaryButton('Cancel', () => {
            this.modalResult = 'Cancelled';
            this.showModal = false;
          })
        ]
      })
    }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    if (this.showModal) {
      if (key.data.key === 'Escape') {
        this.showModal = false;
        this.modalResult = 'Modal closed with ESC';
        return true;
      }
      return false;
    }
    
    switch (key.data.key) {
      case 'ArrowRight':
      case 'Tab':
        this.currentModal = (this.currentModal + 1) % this.modals.length;
        return true;
      
      case 'ArrowLeft':
        this.currentModal = (this.currentModal - 1 + this.modals.length) % this.modals.length;
        return true;
      
      case 'Enter':
      case ' ':
        this.showModal = true;
        this.modalResult = '';
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  render(): Element {
    const modal = this.modals[this.currentModal];
    
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('🪟 Modal Demo', { class: 'text-xl font-bold mb-2' }),
            text('Interactive modal components showcase', { class: 'text-gray-400' })
          ]),
        
        // Main content
        div({ class: 'flex-1 p-8' })
          .children([
            // Modal selector
            div({ class: 'mb-8' })
              .children([
                text('Select Modal Type:', { class: 'text-lg mb-4' }),
                div({ class: 'grid grid-cols-2 gap-4' })
                  .children(this.modals.map((m, index) => 
                    div({ 
                      class: `p-4 rounded border ${
                        index === this.currentModal 
                          ? 'bg-blue-800 border-blue-600' 
                          : 'bg-gray-800 border-gray-700'
                      }`
                    })
                    .children([
                      text(m.name, { class: 'font-bold mb-1' }),
                      text(m.description, { class: 'text-sm text-gray-400' })
                    ])
                  ))
              ]),
            
            // Show modal button
            div({ class: 'text-center mb-8' })
              .children([
                div({ class: 'inline-block bg-blue-700 px-6 py-3 rounded' })
                  .child(text('Press [Enter] or [Space] to show modal'))
              ]),
            
            // Result display
            ...(this.modalResult ? [
              div({ class: 'bg-gray-800 rounded p-4 border border-gray-700' })
                .children([
                  text('Last Result:', { class: 'text-gray-400 mb-1' }),
                  text(this.modalResult, { class: 'text-lg' })
                ])
            ] : [])
          ]),
        
        // Modal overlay
        ...(this.showModal ? [modal.create()] : []),
        
        // Footer
        div({ class: 'bg-gray-800 p-2 border-t border-gray-700 text-center text-sm text-gray-400' })
          .child(text('[←→/Tab] Select Modal | [Enter/Space] Show | [ESC] Close | [Q] Quit'))
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new ModalDemo().render()
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}