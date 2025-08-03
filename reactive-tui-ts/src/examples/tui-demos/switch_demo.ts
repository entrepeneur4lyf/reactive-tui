#!/usr/bin/env bun
/**
 * Switch Widget Demo - TypeScript
 * 
 * Demonstrates toggle switches with various styles and configurations
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  switchToggle, createSwitch, createCustomSwitch, createFormSwitch,
  createCompactSwitch, createUnicodeSwitch, LabelPosition
} from '../../packages/tui-bun/src/widgets/switch';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface Settings {
  notifications: boolean;
  darkMode: boolean;
  autoSave: boolean;
  telemetry: boolean;
  updates: boolean;
  analytics: boolean;
  bluetooth: boolean;
  wifi: boolean;
  airplane: boolean;
  location: boolean;
  doNotDisturb: boolean;
  nightShift: boolean;
}

class SwitchDemo implements Component {
  private settings: Settings = {
    notifications: true,
    darkMode: true,
    autoSave: false,
    telemetry: false,
    updates: true,
    analytics: false,
    bluetooth: true,
    wifi: true,
    airplane: false,
    location: true,
    doNotDisturb: false,
    nightShift: false
  };
  
  private focusedSwitch = 0;
  private switchNames = Object.keys(this.settings) as (keyof Settings)[];
  private lastToggled = '';

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
        this.focusedSwitch = (this.focusedSwitch + 1) % this.switchNames.length;
        return true;
      
      case 'ArrowUp':
        if (this.focusedSwitch > 0) {
          this.focusedSwitch--;
        }
        return true;
      
      case 'ArrowDown':
        if (this.focusedSwitch < this.switchNames.length - 1) {
          this.focusedSwitch++;
        }
        return true;
      
      case ' ':
      case 'Enter':
        const switchName = this.switchNames[this.focusedSwitch];
        this.settings[switchName] = !this.settings[switchName];
        this.lastToggled = switchName;
        this.handleSpecialToggles(switchName);
        return true;
      
      case 'a':
        // Toggle all on/off
        const allOn = Object.values(this.settings).every(v => v);
        Object.keys(this.settings).forEach(key => {
          (this.settings as any)[key] = !allOn;
        });
        return true;
      
      case 'r':
        // Reset to defaults
        this.settings = {
          notifications: true,
          darkMode: true,
          autoSave: false,
          telemetry: false,
          updates: true,
          analytics: false,
          bluetooth: true,
          wifi: true,
          airplane: false,
          location: true,
          doNotDisturb: false,
          nightShift: false
        };
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  handleSpecialToggles(switchName: keyof Settings) {
    // Handle airplane mode
    if (switchName === 'airplane' && this.settings.airplane) {
      this.settings.wifi = false;
      this.settings.bluetooth = false;
    } else if (switchName === 'airplane' && !this.settings.airplane) {
      this.settings.wifi = true;
      this.settings.bluetooth = true;
    }
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('🎚️ Switch Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text('Toggle switches for settings and preferences', { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Basic Switches
            div({ class: 'mb-8' })
              .children([
                text('App Settings', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    div({ class: 'flex items-center justify-between p-2 bg-gray-800 rounded' })
                      .children([
                        div()
                          .children([
                            text('🔔 Notifications', { class: 'font-medium' }),
                            text('Receive push notifications', { class: 'text-sm text-gray-400' })
                          ]),
                        createSwitch(
                          'notifications',
                          this.settings.notifications
                        )
                      ]),
                    div({ class: 'flex items-center justify-between p-2 bg-gray-800 rounded' })
                      .children([
                        div()
                          .children([
                            text('🌙 Dark Mode', { class: 'font-medium' }),
                            text('Use dark theme throughout the app', { class: 'text-sm text-gray-400' })
                          ]),
                        createSwitch(
                          'darkMode',
                          this.settings.darkMode
                        )
                      ]),
                    div({ class: 'flex items-center justify-between p-2 bg-gray-800 rounded' })
                      .children([
                        div()
                          .children([
                            text('💾 Auto-save', { class: 'font-medium' }),
                            text('Automatically save your work', { class: 'text-sm text-gray-400' })
                          ]),
                        createSwitch(
                          'autoSave',
                          this.settings.autoSave
                        )
                      ])
                  ])
              ]),
            
            // Privacy Settings
            div({ class: 'mb-8' })
              .children([
                text('Privacy Settings', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'bg-gray-800 p-4 rounded' })
                  .children([
                    div({ class: 'space-y-3' })
                      .children([
                        div({ class: 'flex items-center justify-between' })
                          .children([
                            div()
                              .children([
                                text('📊 Send Telemetry', { class: 'font-medium' }),
                                text('Help improve the app by sending usage data', { class: 'text-sm text-gray-400' })
                              ]),
                            createCustomSwitch({
                              id: 'telemetry',
                              enabled: this.settings.telemetry,
                              labels: { on: 'ON', off: 'OFF' },
                              width: 10
                            })
                          ]),
                        div({ class: 'flex items-center justify-between' })
                          .children([
                            div()
                              .children([
                                text('📈 Analytics', { class: 'font-medium' }),
                                text('Enable analytics tracking', { class: 'text-sm text-gray-400' })
                              ]),
                            createCustomSwitch({
                              id: 'analytics',
                              enabled: this.settings.analytics,
                              labels: { on: 'ON', off: 'OFF' },
                              width: 10
                            })
                          ])
                      ])
                  ])
              ]),
            
            // System Settings
            div({ class: 'mb-8' })
              .children([
                text('System Settings', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'grid grid-cols-2 gap-4' })
                  .children([
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        text('Connectivity', { class: 'font-bold mb-3' }),
                        div({ class: 'space-y-2' })
                          .children([
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('📶 WiFi'),
                                createCompactSwitch(
                                  'wifi',
                                  this.settings.wifi
                                )
                              ]),
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('🔵 Bluetooth'),
                                createCompactSwitch(
                                  'bluetooth',
                                  this.settings.bluetooth
                                )
                              ]),
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('✈️ Airplane Mode'),
                                createCompactSwitch(
                                  'airplane',
                                  this.settings.airplane
                                )
                              ])
                          ])
                      ]),
                    div({ class: 'bg-gray-800 p-4 rounded' })
                      .children([
                        text('Features', { class: 'font-bold mb-3' }),
                        div({ class: 'space-y-2' })
                          .children([
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('📍 Location'),
                                createCompactSwitch(
                                  'location',
                                  this.settings.location
                                )
                              ]),
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('🔕 Do Not Disturb'),
                                createCompactSwitch(
                                  'doNotDisturb',
                                  this.settings.doNotDisturb
                                )
                              ]),
                            div({ class: 'flex items-center justify-between' })
                              .children([
                                text('🌅 Night Shift'),
                                createCompactSwitch(
                                  'nightShift',
                                  this.settings.nightShift
                                )
                              ])
                          ])
                      ])
                  ])
              ]),
            
            // Switch Variants
            div({ class: 'mb-8' })
              .children([
                text('Switch Variants', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex flex-wrap gap-6' })
                  .children([
                    div({ class: 'text-center' })
                      .children([
                        createUnicodeSwitch({
                          id: 'emoji-switch',
                          enabled: true,
                          style: 'emoji'
                        }),
                        text('Emoji', { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        createUnicodeSwitch({
                          id: 'symbols-switch',
                          enabled: true,
                          style: 'symbols'
                        }),
                        text('Symbols', { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        createUnicodeSwitch({
                          id: 'geometric-switch',
                          enabled: true,
                          style: 'geometric'
                        }),
                        text('Geometric', { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        createSwitch(
                          'default-switch',
                          true
                        ),
                        text('Default', { class: 'text-sm mt-2' })
                      ])
                  ])
              ]),
            
            // Switch Sizes
            div({ class: 'mb-8' })
              .children([
                text('Switch Sizes', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex items-center gap-8' })
                  .children([
                    div({ class: 'text-center' })
                      .children([
                        createCustomSwitch({
                          id: 'size-sm',
                          enabled: true,
                          width: 6,
                          labels: { on: 'ON', off: 'OFF' }
                        }),
                        text('Small', { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        createCustomSwitch({
                          id: 'size-md',
                          enabled: true,
                          width: 8,
                          labels: { on: 'ON', off: 'OFF' }
                        }),
                        text('Medium', { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        createCustomSwitch({
                          id: 'size-lg',
                          enabled: true,
                          width: 12,
                          labels: { on: 'ON', off: 'OFF' }
                        }),
                        text('Large', { class: 'text-sm mt-2' })
                      ])
                  ])
              ]),
            
            // Form Switch
            div({ class: 'mb-8' })
              .children([
                text('Form Style Switches', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    createFormSwitch({
                      id: 'form-enable',
                      label: 'Enable Feature',
                      enabled: true,
                      description: 'Turn on this awesome feature'
                    }),
                    createFormSwitch({
                      id: 'form-backup',
                      label: 'Automatic Backup',
                      enabled: false,
                      description: 'Backup data every hour'
                    }),
                    createFormSwitch({
                      id: 'form-sync',
                      label: 'Cloud Sync',
                      enabled: true,
                      description: 'Sync across all devices'
                    })
                  ])
              ]),
            
            // Disabled State
            div({ class: 'mb-8' })
              .children([
                text('Disabled States', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-6' })
                  .children([
                    div()
                      .children([
                        switchToggle({
                          id: 'disabled-off',
                          enabled: false,
                          interactive: false,
                          showLabels: true,
                          onLabel: 'Disabled',
                          offLabel: 'Disabled'
                        }),
                        text('Disabled (Off)', { class: 'text-sm text-gray-400 mt-1' })
                      ]),
                    div()
                      .children([
                        switchToggle({
                          id: 'disabled-on',
                          enabled: true,
                          interactive: false,
                          showLabels: true,
                          onLabel: 'Disabled',
                          offLabel: 'Disabled'
                        }),
                        text('Disabled (On)', { class: 'text-sm text-gray-400 mt-1' })
                      ])
                  ])
              ]),
            
            // Auto-update toggle
            div({ class: 'mb-8' })
              .children([
                text('Feature Toggle', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'bg-blue-900 p-4 rounded border border-blue-700' })
                  .children([
                    div({ class: 'flex items-center justify-between' })
                      .children([
                        div()
                          .children([
                            text('🔄 Automatic Updates', { class: 'text-lg font-bold' }),
                            text('Download and install updates automatically', { class: 'text-sm text-gray-300' })
                          ]),
                        createCustomSwitch({
                          id: 'updates',
                          enabled: this.settings.updates,
                          labels: { on: 'ENABLED', off: 'DISABLED' },
                          width: 12,
                          handles: { on: '✓', off: '✗' }
                        })
                      ])
                  ])
              ])
          ]),
        
        // Footer with status
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            div({ class: 'flex justify-between items-center mb-2' })
              .children([
                text('Active Settings:', { class: 'text-sm font-bold' }),
                text(this.lastToggled ? `Last toggled: ${this.lastToggled}` : '', 
                  { class: 'text-sm text-gray-400' })
              ]),
            div({ class: 'text-xs text-gray-400 grid grid-cols-4 gap-2' })
              .children(
                Object.entries(this.settings)
                  .filter(([_, value]) => value)
                  .map(([key]) => text(`✓ ${key}`))
              ),
            text('[Tab/↑↓] Navigate | [Space/Enter] Toggle | [A] Toggle All | [R] Reset | [Q] Quit', 
              { class: 'text-center text-sm text-gray-500 mt-2' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new SwitchDemo().render(),
    // Uses full terminal by default
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}