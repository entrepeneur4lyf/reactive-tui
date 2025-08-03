#!/usr/bin/env bun
/**
 * Radio Widget Demo - TypeScript
 * 
 * Demonstrates radio button groups and configurations
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  radioGroup, createRadioGroup, createHorizontalRadioGroup, createFormRadioGroup,
  RadioOption, RadioOrientation
} from '../../packages/tui-bun/src/widgets/radio';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface Settings {
  theme: string;
  language: string;
  fontSize: string;
  autoSave: string;
  notification: string;
  layout: string;
}

class RadioDemo implements Component {
  private settings: Settings = {
    theme: 'dark',
    language: 'english',
    fontSize: 'medium',
    autoSave: 'enabled',
    notification: 'all',
    layout: 'grid'
  };
  
  private focusedGroup = 0;
  private groupNames = ['theme', 'language', 'fontSize', 'autoSave', 'notification', 'layout'];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        this.focusedGroup = (this.focusedGroup + 1) % this.groupNames.length;
        return true;
      
      case 'ArrowUp':
      case 'ArrowDown':
        // Handle navigation within focused group
        const groupName = this.groupNames[this.focusedGroup] as keyof Settings;
        const options = this.getOptionsForGroup(groupName);
        const currentIndex = options.findIndex(opt => opt.value === this.settings[groupName]);
        
        if (key.data.key === 'ArrowUp' && currentIndex > 0) {
          this.settings[groupName] = options[currentIndex - 1].value;
          return true;
        } else if (key.data.key === 'ArrowDown' && currentIndex < options.length - 1) {
          this.settings[groupName] = options[currentIndex + 1].value;
          return true;
        }
        break;
      
      case ' ':
      case 'Enter':
        // Select next option in focused group
        const group = this.groupNames[this.focusedGroup] as keyof Settings;
        const opts = this.getOptionsForGroup(group);
        const idx = opts.findIndex(opt => opt.value === this.settings[group]);
        this.settings[group] = opts[(idx + 1) % opts.length].value;
        return true;
      
      case 'r':
        // Reset all to defaults
        this.settings = {
          theme: 'dark',
          language: 'english',
          fontSize: 'medium',
          autoSave: 'enabled',
          notification: 'all',
          layout: 'grid'
        };
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  getOptionsForGroup(group: keyof Settings): RadioOption[] {
    switch (group) {
      case 'theme':
        return [
          { value: 'light', label: '☀️ Light', description: 'Bright theme for day time' },
          { value: 'dark', label: '🌙 Dark', description: 'Dark theme for night time' },
          { value: 'auto', label: '🔄 Auto', description: 'Follow system preference' }
        ];
      
      case 'language':
        return [
          { value: 'english', label: '🇺🇸 English' },
          { value: 'spanish', label: '🇪🇸 Español' },
          { value: 'french', label: '🇫🇷 Français' },
          { value: 'german', label: '🇩🇪 Deutsch' },
          { value: 'japanese', label: '🇯🇵 日本語' }
        ];
      
      case 'fontSize':
        return [
          { value: 'small', label: 'Small (12px)' },
          { value: 'medium', label: 'Medium (14px)' },
          { value: 'large', label: 'Large (16px)' },
          { value: 'xlarge', label: 'Extra Large (18px)' }
        ];
      
      case 'autoSave':
        return [
          { value: 'enabled', label: 'Enabled' },
          { value: 'disabled', label: 'Disabled' }
        ];
      
      case 'notification':
        return [
          { value: 'all', label: 'All Notifications' },
          { value: 'important', label: 'Important Only' },
          { value: 'none', label: 'None' }
        ];
      
      case 'layout':
        return [
          { value: 'grid', label: 'Grid View' },
          { value: 'list', label: 'List View' },
          { value: 'compact', label: 'Compact View' }
        ];
      
      default:
        return [];
    }
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('🔘 Radio Button Demo', { class: 'text-2xl font-bold mb-2' }),
            text('Settings panel with radio button groups', { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Basic Radio Group
            div({ class: 'mb-8' })
              .children([
                text('Theme Selection', { class: 'text-xl font-bold mb-4' }),
                radioGroup({
                  id: 'theme',
                  options: this.getOptionsForGroup('theme'),
                  selected: this.settings.theme,
                  interactive: true,
                  orientation: RadioOrientation.Vertical
                })
              ]),
            
            // Horizontal Radio Group
            div({ class: 'mb-8' })
              .children([
                text('Language Preference', { class: 'text-xl font-bold mb-4' }),
                createHorizontalRadioGroup(
                  'language',
                  this.getOptionsForGroup('language'),
                  this.settings.language
                )
              ]),
            
            // Themed Radio Groups
            div({ class: 'mb-8' })
              .children([
                text('Font Size', { class: 'text-xl font-bold mb-4' }),
                createFormRadioGroup({
                  id: 'fontSize',
                  options: this.getOptionsForGroup('fontSize'),
                  selected: this.settings.fontSize,
                  title: 'Font Size Selection'
                })
              ]),
            
            // Inline Radio Group
            div({ class: 'mb-8' })
              .children([
                text('Auto-save Documents', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex items-center gap-4' })
                  .children([
                    text('Auto-save:', { class: 'text-gray-400' }),
                    createHorizontalRadioGroup(
                      'autoSave',
                      this.getOptionsForGroup('autoSave'),
                      this.settings.autoSave
                    )
                  ])
              ]),
            
            // Radio Group with Descriptions
            div({ class: 'mb-8' })
              .children([
                text('Notification Settings', { class: 'text-xl font-bold mb-4' }),
                createFormRadioGroup({
                  id: 'notification',
                  options: [
                    { 
                      value: 'all', 
                      label: 'All Notifications',
                      description: 'Receive all app notifications and alerts'
                    },
                    { 
                      value: 'important', 
                      label: 'Important Only',
                      description: 'Only critical updates and security alerts'
                    },
                    { 
                      value: 'none', 
                      label: 'None',
                      description: 'Disable all notifications'
                    }
                  ],
                  selected: this.settings.notification,
                  title: 'Notification Settings'
                })
              ]),
            
            // Different Sizes
            div({ class: 'mb-8' })
              .children([
                text('Radio Button Sizes', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-4' })
                  .children([
                    div()
                      .children([
                        text('Small Size', { class: 'text-sm text-gray-400 mb-2' }),
                        createHorizontalRadioGroup(
                          'size-demo-sm',
                          [
                            { value: 'opt1', label: 'Option 1' },
                            { value: 'opt2', label: 'Option 2' }
                          ],
                          'opt1'
                        )
                      ]),
                    div()
                      .children([
                        text('Medium Size (Default)', { class: 'text-sm text-gray-400 mb-2' }),
                        createHorizontalRadioGroup(
                          'size-demo-md',
                          [
                            { value: 'opt1', label: 'Option 1' },
                            { value: 'opt2', label: 'Option 2' }
                          ],
                          'opt1'
                        )
                      ]),
                    div()
                      .children([
                        text('Large Size', { class: 'text-sm text-gray-400 mb-2' }),
                        createHorizontalRadioGroup(
                          'size-demo-lg',
                          [
                            { value: 'opt1', label: 'Option 1' },
                            { value: 'opt2', label: 'Option 2' }
                          ],
                          'opt1'
                        )
                      ])
                  ])
              ]),
            
            // Disabled State
            div({ class: 'mb-8' })
              .children([
                text('Disabled Radio Group', { class: 'text-xl font-bold mb-4' }),
                radioGroup({
                  id: 'disabled-demo',
                  options: [
                    { value: 'opt1', label: 'Option 1' },
                    { value: 'opt2', label: 'Option 2 (Selected)' },
                    { value: 'opt3', label: 'Option 3' }
                  ],
                  selected: 'opt2',
                  interactive: false
                })
              ]),
            
            // Custom Styled Radio Group
            div({ class: 'mb-8' })
              .children([
                text('Layout Options', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-3 gap-4' })
                  .children(this.getOptionsForGroup('layout').map(option => 
                    div({ 
                      class: `p-4 border-2 rounded cursor-pointer transition-all ${
                        this.settings.layout === option.value 
                          ? 'border-blue-500 bg-blue-900' 
                          : 'border-gray-700 hover:border-gray-500'
                      }`
                    })
                      .children([
                        text(
                          `${this.settings.layout === option.value ? '●' : '○'} ${option.label}`,
                          { class: 'font-medium' }
                        ),
                        div({ class: 'mt-2 text-sm text-gray-400' })
                          .child(text(
                            option.value === 'grid' ? '▦ ▦ ▦\n▦ ▦ ▦' :
                            option.value === 'list' ? '═══\n═══\n═══' :
                            '▪ ▪ ▪'
                          ))
                      ])
                  ))
              ])
          ]),
        
        // Settings Summary
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            text('Current Settings:', { class: 'text-sm font-bold mb-2' }),
            div({ class: 'grid grid-cols-3 gap-4 text-sm text-gray-400' })
              .children([
                text(`Theme: ${this.settings.theme}`),
                text(`Language: ${this.settings.language}`),
                text(`Font: ${this.settings.fontSize}`),
                text(`Auto-save: ${this.settings.autoSave}`),
                text(`Notifications: ${this.settings.notification}`),
                text(`Layout: ${this.settings.layout}`)
              ]),
            text('[Tab] Navigate Groups | [↑↓] Select Option | [Space] Quick Select | [R] Reset | [Q] Quit', 
              { class: 'text-center text-sm text-gray-500 mt-2' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new RadioDemo().render(),
    // Uses full terminal by default
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}