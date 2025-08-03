/**
 * Accordion Widget Demo - TypeScript Implementation
 * 
 * Demonstrates the comprehensive Accordion widget with expandable/collapsible sections,
 * keyboard navigation, custom styling, and animation support.
 */

import { 
    createApp, div, text, Component, flexColumn, flexRow, header, main, footer, section
} from '../../packages/tui-bun/src';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class AccordionDemo implements Component {
  private expandedSections: Set<string> = new Set(['welcome']);
  private focusedSection = 'welcome';
  private sections = [
    {
      id: 'welcome',
      title: '👋 Welcome to Our Application',
      content: 'This is the welcome section with basic information about our application. Click to expand/collapse sections and navigate with arrow keys.'
    },
    {
      id: 'features',
      title: '⭐ Key Features',
      content: '• Advanced TUI components\n• Reactive state management\n• Cross-platform support\n• Beautiful animations'
    },
    {
      id: 'support',
      title: '🆘 Getting Support',
      content: 'Need help? Contact our support team at support@company.com\nOr visit our documentation at docs.company.com'
    },
    {
      id: 'settings',
      title: '⚙️  General Settings',
      content: 'Application language: English\nTheme: Dark Mode\nNotifications: Enabled'
    },
    {
      id: 'faq',
      title: '❓ Frequently Asked Questions',
      content: 'Q: How do I install the application?\nA: You can install using npm or download from releases.\n\nQ: Where can I get help?\nA: Check our documentation or contact support.'
    }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'ArrowDown':
        this.focusNext();
        return true;
      
      case 'ArrowUp':
        this.focusPrevious();
        return true;
      
      case 'Enter':
      case ' ':
        this.toggleSection(this.focusedSection);
        return true;
      
      case 'e':
        // Expand all
        this.sections.forEach(section => this.expandedSections.add(section.id));
        return true;
      
      case 'c':
        // Collapse all
        this.expandedSections.clear();
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }
  
  private focusNext() {
    const currentIndex = this.sections.findIndex(s => s.id === this.focusedSection);
    const nextIndex = (currentIndex + 1) % this.sections.length;
    this.focusedSection = this.sections[nextIndex].id;
  }
  
  private focusPrevious() {
    const currentIndex = this.sections.findIndex(s => s.id === this.focusedSection);
    const prevIndex = (currentIndex - 1 + this.sections.length) % this.sections.length;
    this.focusedSection = this.sections[prevIndex].id;
  }
  
  private toggleSection(sectionId: string) {
    if (this.expandedSections.has(sectionId)) {
      this.expandedSections.delete(sectionId);
    } else {
      this.expandedSections.add(sectionId);
    }
  }
  
  private renderSection(section: typeof this.sections[0]): Element {
    const isExpanded = this.expandedSections.has(section.id);
    const isFocused = this.focusedSection === section.id;
    
    return div({ class: `border rounded mb-2 ${isFocused ? 'border-blue-500' : 'border-gray-600'}` })
      .children([
        // Section header
        div({ 
          class: `p-3 cursor-pointer ${isFocused ? 'bg-blue-900' : 'bg-gray-800'} ${isExpanded ? 'rounded-t' : 'rounded'}` 
        })
          .child(
            flexRow([
              text(isExpanded ? '🔼' : '🔽', { class: 'mr-2' }),
              text(section.title, { class: 'flex-1 font-semibold' }),
              text(isFocused ? '●' : '', { class: 'text-blue-400' })
            ], { class: 'items-center' })
          ),
        
        // Section content (only if expanded)
        ...(isExpanded ? [
          div({ class: 'p-4 bg-gray-900 rounded-b border-t border-gray-600' })
            .child(text(section.content, { class: 'text-gray-300 whitespace-pre-line' }))
        ] : [])
      ])
      .build();
  }

  render(): Element {
    return flexColumn([
      // Header
      header({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
        .children([
          text('🪗 Accordion Widget Demo', { class: 'text-2xl font-bold mb-2' }),
          text('Interactive accordion with keyboard navigation', { class: 'text-gray-400' })
        ]),
      
      // Main content
      main({ class: 'flex-1 p-6 overflow-auto' })
        .children([
          // Statistics
          div({ class: 'mb-6 p-4 bg-gray-800 rounded' })
            .children([
              text('📊 Status', { class: 'text-lg font-bold mb-2' }),
              flexRow([
                text(`Expanded: ${this.expandedSections.size}/${this.sections.length}`, { class: 'text-green-400' }),
                text(`Focused: ${this.sections.find(s => s.id === this.focusedSection)?.title || 'None'}`, { class: 'text-blue-400' })
              ], { class: 'gap-6' })
            ]),
          
          // Accordion sections
          div({ class: 'space-y-1' })
            .children(this.sections.map(section => this.renderSection(section)))
        ]),
      
      // Controls footer
      footer({ class: 'bg-gray-800 p-3 border-t border-gray-700' })
        .child(
          flexRow([
            text('[↑↓] Navigate | [Enter/Space] Toggle | [E] Expand All | [C] Collapse All | [Q] Quit', 
                 { class: 'text-sm text-gray-400' })
          ], { class: 'justify-center' })
        )
    ], { class: 'h-full bg-gray-900 text-white' })
    .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new AccordionDemo().render(),
    fullscreen: true
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}

export { AccordionDemo };