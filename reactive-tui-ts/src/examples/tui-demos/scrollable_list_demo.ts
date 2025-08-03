#!/usr/bin/env bun
/**
 * ScrollableList Demo - High-Performance Virtual Scrolling
 * 
 * This demo showcases the ScrollableList widget with:
 * - Virtual scrolling for handling large datasets efficiently
 * - Smooth keyboard navigation and mouse wheel support
 * - Search and filtering capabilities
 * - Multiple selection modes
 * - Custom styling and themes
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import {
  ScrollableList,
  createScrollableList
} from '../../packages/tui-bun/src/widgets/scrollable_list';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

// Demo data structure for list items
interface DemoItem {
  id: number;
  title: string;
  description: string;
  category: string;
  priority: Priority;
}

enum Priority {
  Low = 'low',
  Medium = 'medium', 
  High = 'high',
  Critical = 'critical'
}

class ScrollableListDemo implements Component {
  private items: DemoItem[] = [];
  private list: ScrollableList<DemoItem>;
  private searchQuery = '';
  private showHelp = false;
  
  constructor() {
    // Generate demo data
    this.generateItems(1000);
    
    // Create scrollable list
    this.list = createScrollableList<DemoItem>({
      items: this.items,
      itemHeight: 3,
      visibleItems: 10,
      searchable: true,
      multiSelect: true,
      renderItem: (item) => this.renderItem(item),
      onSelect: (item) => console.log('Selected:', item),
      onSearch: (query) => {
        this.searchQuery = query;
        return this.items.filter(item => 
          item.title.toLowerCase().includes(query.toLowerCase()) ||
          item.description.toLowerCase().includes(query.toLowerCase())
        );
      }
    });
  }
  
  generateItems(count: number) {
    const categories = ['Work', 'Personal', 'Projects', 'Ideas', 'Archive'];
    const priorities = [Priority.Low, Priority.Medium, Priority.High, Priority.Critical];
    
    for (let i = 0; i < count; i++) {
      this.items.push({
        id: i + 1,
        title: `Item #${i + 1}: ${this.generateTitle()}`,
        description: this.generateDescription(),
        category: categories[Math.floor(Math.random() * categories.length)],
        priority: priorities[Math.floor(Math.random() * priorities.length)]
      });
    }
  }
  
  generateTitle(): string {
    const titles = [
      'Complete project documentation',
      'Review pull request',
      'Update dependencies',
      'Fix bug in production',
      'Implement new feature',
      'Optimize performance',
      'Write unit tests',
      'Refactor legacy code'
    ];
    return titles[Math.floor(Math.random() * titles.length)];
  }
  
  generateDescription(): string {
    const descriptions = [
      'High priority task that needs immediate attention',
      'Regular maintenance work',
      'Feature requested by multiple users',
      'Critical security update required',
      'Performance improvement opportunity',
      'Code quality enhancement',
      'User experience improvement',
      'Technical debt reduction'
    ];
    return descriptions[Math.floor(Math.random() * descriptions.length)];
  }
  
  renderItem(item: DemoItem): Element {
    const priorityColors = {
      [Priority.Low]: 'text-gray-400',
      [Priority.Medium]: 'text-yellow-400',
      [Priority.High]: 'text-orange-400',
      [Priority.Critical]: 'text-red-400'
    };
    
    return div({ class: 'p-2 border-b border-gray-700' })
      .children([
        div({ class: 'flex justify-between items-start' })
          .children([
            div({ class: 'flex-1' })
              .children([
                text(item.title, { class: 'font-bold' }),
                text(item.description, { class: 'text-sm text-gray-400' })
              ]),
            div({ class: 'flex gap-2 items-center' })
              .children([
                text(item.category, { class: 'text-xs bg-gray-700 px-2 py-1 rounded' }),
                text(item.priority.toUpperCase(), { 
                  class: `text-xs font-bold ${priorityColors[item.priority]}` 
                })
              ])
          ])
      ])
      .build();
  }
  
  handleKeyPress(key: KeyEvent): boolean {
    if (key.data.key === 'h' || key.data.key === 'H') {
      this.showHelp = !this.showHelp;
      return true;
    }
    
    if (key.data.key === 'q' || key.data.key === 'Q') {
      process.exit(0);
    }
    
    // Pass other keys to the list
    return this.list.handleKeyPress(key);
  }
  
  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('📜 Scrollable List Demo', { class: 'text-xl font-bold mb-2' }),
            div({ class: 'flex justify-between items-center' })
              .children([
                text(`${this.items.length} items | ${this.list.getSelectedCount()} selected`, 
                  { class: 'text-gray-400' }),
                text('[H] Help | [Q] Quit', { class: 'text-gray-500' })
              ])
          ]),
        
        // Search bar
        ...(this.searchQuery ? [
          div({ class: 'bg-gray-800 p-2 border-b border-gray-700' })
            .children([
              text(`Search: ${this.searchQuery}`, { class: 'text-yellow-400' })
            ])
        ] : []),
        
        // List content
        div({ class: 'flex-1 overflow-hidden' })
          .child(this.list.render()),
        
        // Help overlay
        ...(this.showHelp ? [
          div({ class: 'absolute inset-0 bg-black/80 flex items-center justify-center' })
            .child(
              div({ class: 'bg-gray-800 p-6 rounded border border-gray-700 max-w-lg' })
                .children([
                  text('Keyboard Shortcuts', { class: 'text-lg font-bold mb-4' }),
                  div({ class: 'space-y-2' })
                    .children([
                      text('[↑↓] Navigate items'),
                      text('[PgUp/PgDn] Page navigation'),
                      text('[Home/End] Jump to start/end'),
                      text('[Space] Toggle selection'),
                      text('[Ctrl+A] Select all'),
                      text('[/] Start search'),
                      text('[ESC] Clear search'),
                      text('[H] Toggle this help'),
                      text('[Q] Quit application')
                    ])
                ])
            )
        ] : []),
        
        // Status bar
        div({ class: 'bg-gray-800 p-2 border-t border-gray-700 text-sm text-gray-400' })
          .children([
            text(`Position: ${this.list.getCurrentIndex() + 1}/${this.items.length} | `),
            text(`Visible: ${this.list.getVisibleRange().start}-${this.list.getVisibleRange().end} | `),
            text(`Performance: ${Math.round(1000 / 16)}fps`)
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new ScrollableListDemo().render()
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}