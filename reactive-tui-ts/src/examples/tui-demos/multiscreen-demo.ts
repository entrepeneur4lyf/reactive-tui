#!/usr/bin/env bun
/**
 * Multi-Screen TUI Demo
 * 
 * A fully functional multi-screen terminal UI application
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { button } from '../../packages/tui-bun/src/widgets/button';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

// Screen data
interface Screen {
  id: string;
  title: string;
  content: string;
  actions?: string[];
}

// Screen Manager Component
class ScreenManager implements Component {
  private screens: Screen[] = [
    {
      id: 'home',
      title: '🏠 Home',
      content: 'Welcome to the Multi-Screen TUI Demo!\n\nThis demo showcases:\n• Multiple screens with navigation\n• Keyboard shortcuts\n• History tracking\n• Sidebar navigation\n• Fully interactive interface',
      actions: ['Press Tab or → for next screen', 'Press B to toggle sidebar']
    },
    {
      id: 'features',
      title: '✨ Features',
      content: 'Key Features:\n\n→ Screen navigation with history\n→ Direct screen access (1-4 keys)\n→ Collapsible sidebar\n→ Keyboard shortcuts\n→ Responsive layout\n→ Beautiful styling',
      actions: ['Navigate between screens', 'Try keyboard shortcuts']
    },
    {
      id: 'settings',
      title: '⚙️ Settings',
      content: 'Application Settings:\n\n[ ] Enable animations\n[✓] Show sidebar on startup\n[ ] Dark mode (always on)\n[✓] Keyboard navigation\n[ ] Sound effects',
      actions: ['Toggle with spacebar', 'Save with Enter']
    },
    {
      id: 'about',
      title: 'ℹ️ About',
      content: 'Multi-Screen TUI Demo v1.0\n\nBuilt with:\n• TypeScript + Bun\n• TUI Framework\n• CSS-styled components\n• Love and care ❤️',
      actions: ['Visit github.com/yourusername', 'Report issues']
    }
  ];

  private currentIndex = 0;
  private history: number[] = [];
  private maxHistory = 10;

  navigateTo(index: number) {
    if (index >= 0 && index < this.screens.length && index !== this.currentIndex) {
      this.history.push(this.currentIndex);
      if (this.history.length > this.maxHistory) {
        this.history.shift();
      }
      this.currentIndex = index;
    }
  }

  goBack() {
    if (this.history.length > 0) {
      this.currentIndex = this.history.pop()!;
    }
  }

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
      case 'ArrowRight':
        this.navigateTo((this.currentIndex + 1) % this.screens.length);
        return true;
      
      case 'ArrowLeft':
        this.navigateTo((this.currentIndex - 1 + this.screens.length) % this.screens.length);
        return true;
      
      case 'Escape':
        this.goBack();
        return true;
      
      case '1':
      case '2':
      case '3':
      case '4':
        const index = parseInt(key.data.key) - 1;
        if (index < this.screens.length) {
          this.navigateTo(index);
          return true;
        }
        break;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  render(): Element {
    const screen = this.screens[this.currentIndex];
    
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        // Header bar
        div({ class: 'bg-blue-800 text-white p-2 flex justify-between' })
          .children([
            text(screen.title),
            text(`Screen ${this.currentIndex + 1}/${this.screens.length}`)
          ]),
        
        // Main content
        div({ class: 'flex-1 p-4' })
          .children([
            // Content box
            div({ class: 'bg-gray-800 border border-gray-700 rounded p-4 mb-4' })
              .child(text(screen.content)),
            
            // Actions
            ...(screen.actions ? [
              div({ class: 'mt-4' })
                .children([
                  text('Actions:', { class: 'text-yellow-400 mb-2' }),
                  ...screen.actions.map(action => 
                    div({ class: 'text-gray-400 ml-4' })
                      .child(text(action))
                  )
                ])
            ] : [])
          ]),
        
        // Footer
        div({ class: 'bg-gray-800 text-gray-400 p-2 border-t border-gray-700' })
          .child(text('[Tab/→] Next | [←] Previous | [Esc] Back | [1-4] Jump | [Q] Quit'))
      ])
      .build();
  }
}

// Multi-screen app with sidebar
class MultiScreenApp implements Component {
  private screenManager = new ScreenManager();
  private showSidebar = true;

  handleKeyPress(key: KeyEvent): boolean {
    // Toggle sidebar with 'b'
    if (key.data.key === 'b' || key.data.key === 'B') {
      this.showSidebar = !this.showSidebar;
      return true;
    }
    
    // Pass other keys to screen manager
    return this.screenManager.handleKeyPress(key);
  }

  render(): Element {
    return div({ class: 'flex h-full' })
      .children([
        // Sidebar
        ...(this.showSidebar ? [
          div({ class: 'w-64 bg-gray-800 border-r border-gray-700 p-4' })
            .children([
              text('Navigation', { class: 'text-white text-xl mb-4' }),
              
              // Menu items
              ...this.screenManager['screens'].map((screen, index) => 
                button({
                  id: `nav-${screen.id}`,
                  text: `${index + 1}. ${screen.title}`,
                  fullWidth: true,
                  variant: index === this.screenManager['currentIndex'] ? 'filled' : 'ghost',
                  color: index === this.screenManager['currentIndex'] ? 'primary' : 'neutral',
                  cssClasses: ['text-left', 'p-2', 'mb-2']
                })
              ),
              
              // Sidebar footer
              div({ class: 'mt-8 pt-4 border-t border-gray-700' })
                .child(text('[B] Toggle Sidebar', { class: 'text-gray-500 text-sm' }))
            ])
        ] : []),
        
        // Main content
        div({ class: 'flex-1' })
          .child(this.screenManager.render())
      ])
      .build();
  }
}

// Run the application
async function main() {
  const app = createApp({
    component: () => new MultiScreenApp().render()
  });

  await app.run();
}

main().catch(console.error);