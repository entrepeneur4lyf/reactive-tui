#!/usr/bin/env bun
/**
 * Bar Widget Demo - TypeScript
 * 
 * Demonstrates various bar types: header, footer, status, navigation, toolbar
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import {
  headerBar, footerBar, statusBar, navigationBar, toolbar,
  barItem, clickableBarItem, iconBarItem, statusIndicator
} from '../../packages/tui-bun/src/widgets/bar';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class BarDemo implements Component {
  private currentDemo = 0;
  private activeTab = 0;
  private notifications = 3;
  private isOnline = true;
  
  private demos = [
    { name: 'Header Bar', render: () => this.headerBarDemo() },
    { name: 'Footer Bar', render: () => this.footerBarDemo() },
    { name: 'Status Bar', render: () => this.statusBarDemo() },
    { name: 'Navigation Bar', render: () => this.navigationBarDemo() },
    { name: 'Toolbar', render: () => this.toolbarDemo() },
    { name: 'Full Application', render: () => this.fullAppDemo() }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        this.currentDemo = (this.currentDemo + 1) % this.demos.length;
        return true;
      
      case '1':
      case '2':
      case '3':
      case '4':
        const tab = parseInt(key.data.key) - 1;
        if (tab < 4) {
          this.activeTab = tab;
          return true;
        }
        break;
      
      case 'n':
        this.notifications = Math.max(0, this.notifications - 1);
        return true;
      
      case 'N':
        this.notifications++;
        return true;
      
      case 'o':
        this.isOnline = !this.isOnline;
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  headerBarDemo(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        headerBar('demo-header')
          .left('🚀 My Application')
          .item(barItem('File', 'left', { clickable: true }))
          .item(barItem('Edit', 'left', { clickable: true }))
          .item(barItem('View', 'left', { clickable: true }))
          .item(barItem('Help', 'left', { clickable: true }))
          .right(`🔔${this.notifications > 0 ? ` ${this.notifications}` : ''} 👤`)
          .build(),
        
        div({ class: 'flex-1 p-8 text-white' })
          .children([
            text('Header Bar Demo', { class: 'text-2xl mb-4' }),
            text('The header bar provides main application navigation and branding.'),
            div({ class: 'mt-4 space-y-2' })
              .children([
                text('• Logo and title on the left'),
                text('• Main navigation items'),
                text('• Action items on the right'),
                text('• Notification badges'),
                text('Press [N] to add notifications')
              ])
          ])
      ])
      .build();
  }

  footerBarDemo(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        div({ class: 'flex-1 p-8 text-white' })
          .children([
            text('Footer Bar Demo', { class: 'text-2xl mb-4' }),
            text('The footer bar shows status information and quick actions.')
          ]),
        
        footerBar('demo-footer')
          .item(statusIndicator(this.isOnline ? 'Connected' : 'Offline', this.isOnline ? '🟢' : '🔴'))
          .item(barItem('Ready', 'left'))
          .item(barItem('Line 1, Col 1', 'left'))
          .right('UTF-8 | TypeScript | 2 spaces')
          .build()
      ])
      .build();
  }

  statusBarDemo(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        div({ class: 'flex-1 p-8 text-white' })
          .children([
            text('Status Bar Demo', { class: 'text-2xl mb-4' }),
            text('Multiple status bars for different contexts.'),
            text('Press [O] to toggle online status', { class: 'mt-4 text-gray-400' })
          ]),
        
        statusBar('demo-status')
          .item(statusIndicator(this.isOnline ? '● Online' : '● Offline', this.isOnline ? '🟢' : '🔴'))
          .item(barItem(`${this.notifications} notifications`, 'left'))
          .right(new Date().toLocaleTimeString())
          .build()
      ])
      .build();
  }

  navigationBarDemo(): Element {
    const tabs = ['Dashboard', 'Projects', 'Tasks', 'Settings'];
    
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        navigationBar('demo-nav')
          .item(clickableBarItem(tabs[0], 'left', undefined, { clickable: true }))
          .item(clickableBarItem(tabs[1], 'left', undefined, { clickable: true }))
          .item(clickableBarItem(tabs[2], 'left', undefined, { clickable: true }))
          .item(clickableBarItem(tabs[3], 'left', undefined, { clickable: true }))
          .build(),
        
        div({ class: 'flex-1 p-8 text-white' })
          .children([
            text(`${tabs[this.activeTab]} Content`, { class: 'text-2xl mb-4' }),
            text('Navigation bar with tab-style items.'),
            text('Press [1-4] to switch tabs', { class: 'mt-4 text-gray-400' })
          ])
      ])
      .build();
  }

  toolbarDemo(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        toolbar('demo-toolbar')
          .item(iconBarItem('Open', '📁', 'left', { clickable: true }))
          .item(iconBarItem('Save', '💾', 'left', { clickable: true }))
          .item(iconBarItem('Cut', '✂️', 'left', { clickable: true }))
          .item(iconBarItem('Copy', '📋', 'left', { clickable: true }))
          .item(iconBarItem('Paste', '📌', 'left', { clickable: true }))
          .item(barItem('|', 'left', { clickable: false }))
          .item(iconBarItem('Undo', '↩️', 'left', { clickable: true }))
          .item(iconBarItem('Redo', '↪️', 'left', { clickable: true }))
          .build(),
        
        div({ class: 'flex-1 p-8 text-white' })
          .children([
            text('Toolbar Demo', { class: 'text-2xl mb-4' }),
            text('Icon-based toolbar for quick actions.'),
            div({ class: 'mt-4 space-y-2' })
              .children([
                text('• Icon buttons with tooltips'),
                text('• Separators for grouping'),
                text('• Hover effects'),
                text('• Keyboard shortcuts (not shown)')
              ])
          ])
      ])
      .build();
  }

  fullAppDemo(): Element {
    const header = headerBar('full-app-header')
      .left('🎯 Full Application Demo')
      .item(barItem('File', 'left', { clickable: true }))
      .item(barItem('Edit', 'left', { clickable: true }))
      .item(barItem('View', 'left', { clickable: true }))
      .item(barItem('Tools', 'left', { clickable: true }))
      .item(barItem('Window', 'left', { clickable: true }))
      .item(barItem('Help', 'left', { clickable: true }))
      .right(`🔍 🔔${this.notifications > 0 ? ` ${this.notifications}` : ''} Admin`)
      .build();

    const footer = footerBar('full-app-footer')
      .item(statusIndicator(this.isOnline ? 'Connected' : 'Offline', this.isOnline ? '🟢' : '🔴'))
      .item(barItem('main', 'left'))
      .item(barItem('⚡ 125ms', 'left'))
      .right('Ln 42, Col 17 | Spaces: 2 | UTF-8 | TypeScript')
      .build();

    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        header,

        div({ class: 'flex-1 flex' })
          .children([
            // Sidebar
            div({ class: 'w-64 bg-gray-800 p-4' })
              .children([
                text('Sidebar', { class: 'text-lg mb-4 text-white' }),
                div({ class: 'space-y-2' })
                  .children(['Files', 'Search', 'Git', 'Debug', 'Extensions'].map(item =>
                    div({ class: 'p-2 hover:bg-gray-700 rounded text-gray-300' })
                      .child(text(item))
                  ))
              ]),

            // Main content
            div({ class: 'flex-1 p-8 text-white' })
              .children([
                text('Full Application Layout', { class: 'text-2xl mb-4' }),
                text('Complete application with all bar types:'),
                div({ class: 'mt-4 space-y-2' })
                  .children([
                    text('• Header bar with navigation'),
                    text('• Toolbar for actions'),
                    text('• Status bar at bottom'),
                    text('• Sidebar navigation'),
                    text('• Responsive full-screen layout')
                  ])
              ])
          ]),

        footer
      ])
      .build();
  }

  render(): Element {
    const demo = this.demos[this.currentDemo];
    
    return div({ class: 'h-full' })
      .children([
        demo.render(),
        
        // Demo selector overlay
        div({ class: 'absolute bottom-0 right-0 bg-black/80 text-white p-2 text-sm' })
          .child(text(`[Tab] Next Demo (${this.currentDemo + 1}/${this.demos.length}: ${demo.name}) | [Q] Quit`))
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new BarDemo().render()
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}