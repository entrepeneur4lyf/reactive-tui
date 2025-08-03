#!/usr/bin/env bun
/**
 * Widget Factory Pattern Demo
 * 
 * Demonstrates the new widget factory pattern with type safety,
 * configuration validation, instance caching, and performance monitoring.
 */

import { 
  createApp, div, text, flexColumn, flexRow, header, main, footer
} from '../../packages/tui-bun/src';
import { 
  WidgetFactory, 
  createWidget,
  createWidgetCreator,
  widgetRegistry
} from '../../packages/tui-bun/src/widget-factory';
import { 
  ButtonWidget, 
  primaryButton, 
  secondaryButton, 
  dangerButton,
  iconButton,
  loadingButton,
  buttonGroup
} from '../../packages/tui-bun/src/widgets/factory-button';
import type { ButtonConfig } from '../../packages/tui-bun/src/widgets/factory-button';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface ComponentState {
  currentDemo: number;
  loadingStates: Record<string, boolean>;
  widgetStats: any;
}

class WidgetFactoryDemo {
  private state: ComponentState = {
    currentDemo: 0,
    loadingStates: {},
    widgetStats: {}
  };

  private demos = [
    {
      name: 'Basic Factory Usage',
      description: 'Creating widgets with the factory pattern',
      render: () => this.renderBasicDemo()
    },
    {
      name: 'Configuration Validation',
      description: 'Demonstrating config validation and error handling',
      render: () => this.renderValidationDemo()
    },
    {
      name: 'Performance & Caching',
      description: 'Widget instance caching and performance monitoring',
      render: () => this.renderPerformanceDemo()
    },
    {
      name: 'Batch Operations',
      description: 'Creating and managing multiple widgets efficiently',
      render: () => this.renderBatchDemo()
    },
    {
      name: 'Migration Example',
      description: 'Before/after comparison with old pattern',
      render: () => this.renderMigrationDemo()
    }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'ArrowRight':
      case 'Tab':
        this.state.currentDemo = (this.state.currentDemo + 1) % this.demos.length;
        return true;
      
      case 'ArrowLeft':
        this.state.currentDemo = (this.state.currentDemo - 1 + this.demos.length) % this.demos.length;
        return true;
      
      case 'r':
      case 'R':
        this.refreshStats();
        return true;
      
      case 'c':
      case 'C':
        this.clearCache();
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  private refreshStats(): void {
    this.state.widgetStats = WidgetFactory.getStats();
  }

  private clearCache(): void {
    WidgetFactory.clearCache();
    this.refreshStats();
  }

  render(): Element {
    const demo = this.demos[this.state.currentDemo];
    
    return flexColumn([
      // Header
      header({ class: 'bg-blue-800 text-white p-4 border-b border-blue-700' })
        .children([
          text('🏭 Widget Factory Pattern Demo', { class: 'text-2xl font-bold mb-2' }),
          flexRow([
            text(demo.name, { class: 'text-lg' }),
            text(`${this.state.currentDemo + 1}/${this.demos.length}`, { class: 'text-blue-300' })
          ], { class: 'justify-between items-center' }),
          text(demo.description, { class: 'text-blue-200 text-sm mt-1' })
        ]),
      
      // Main content
      main({ class: 'flex-1 p-6 overflow-auto' })
        .child(demo.render()),
      
      // Stats footer
      footer({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
        .children([
          this.renderStats(),
          text('[←→] Navigate | [R] Refresh Stats | [C] Clear Cache | [Q] Quit', 
               { class: 'text-gray-400 text-sm mt-2' })
        ])
    ], { class: 'h-full bg-gray-900 text-white' })
    .build();
  }

  private renderStats(): Element {
    const stats = WidgetFactory.getStats();
    
    return flexRow([
      text(`Types: ${stats.registeredTypes.length}`, { class: 'text-green-400' }),
      text(`Cached: ${stats.cache.totalInstances}`, { class: 'text-blue-400' }),
      text(`Memory: ${stats.cache.memoryUsage}`, { class: 'text-yellow-400' })
    ], { class: 'gap-6' })
    .build();
  }

  private renderBasicDemo(): Element {
    return flexColumn([
      text('Basic Widget Factory Usage', { class: 'text-xl font-bold mb-4' }),
      
      // Factory creation examples
      div({ class: 'mb-6' })
        .children([
          text('1. Using Factory.create():', { class: 'font-semibold mb-2' }),
          div({ class: 'bg-gray-800 p-4 rounded mb-4' })
            .children([
              text('WidgetFactory.create("button", config)', { class: 'font-mono text-green-300' }),
              this.createFactoryButton('factory-btn-1', 'Factory Created Button')
            ]),
          
          text('2. Using convenience functions:', { class: 'font-semibold mb-2' }),
          flexRow([
            primaryButton('primary-demo', 'Primary').render(),
            secondaryButton('secondary-demo', 'Secondary').render(),
            dangerButton('danger-demo', 'Danger').render()
          ], { class: 'gap-2 mb-4' }),
          
          text('3. Using generic createWidget():', { class: 'font-semibold mb-2' }),
          div({ class: 'bg-gray-800 p-4 rounded' })
            .children([
              text('createWidget("button", config)', { class: 'font-mono text-green-300' }),
              this.createGenericButton('generic-btn', 'Generic Button')
            ])
        ]),
      
      // Feature demonstration
      div({ class: 'mb-6' })
        .children([
          text('Widget Features:', { class: 'text-lg font-bold mb-3' }),
          flexRow([
            iconButton('icon-btn', '⚙️').render(),
            loadingButton('loading-btn', 'Loading', undefined, true).render(),
            this.createDisabledButton('disabled-btn', 'Disabled')
          ], { class: 'gap-2' })
        ])
    ])
    .build();
  }

  private renderValidationDemo(): Element {
    return flexColumn([
      text('Configuration Validation', { class: 'text-xl font-bold mb-4' }),
      
      div({ class: 'mb-6' })
        .children([
          text('✅ Valid Configuration:', { class: 'text-green-400 font-semibold mb-2' }),
          div({ class: 'bg-gray-800 p-4 rounded mb-4' })
            .children([
              text('{ id: "valid", type: "button", text: "Valid Button" }', 
                   { class: 'font-mono text-green-300 mb-2' }),
              this.createValidButton('valid-btn', 'Valid Button')
            ]),
          
          text('❌ Validation Errors (handled gracefully):', { class: 'text-red-400 font-semibold mb-2' }),
          div({ class: 'bg-gray-800 p-4 rounded mb-4' })
            .children([
              text('Missing required fields', { class: 'text-red-300 mb-1' }),
              text('Invalid enum values', { class: 'text-red-300 mb-1' }),
              text('Type mismatches', { class: 'text-red-300 mb-1' }),
              text('Custom validation failures', { class: 'text-red-300' })
            ]),
          
          text('🔧 Configuration with defaults:', { class: 'text-blue-400 font-semibold mb-2' }),
          div({ class: 'bg-gray-800 p-4 rounded' })
            .children([
              text('Minimal config gets defaults applied automatically', 
                   { class: 'text-blue-300 mb-2' }),
              this.createMinimalButton('minimal-btn', 'Minimal Config')
            ])
        ])
    ])
    .build();
  }

  private renderPerformanceDemo(): Element {
    return flexColumn([
      text('Performance & Caching', { class: 'text-xl font-bold mb-4' }),
      
      div({ class: 'mb-6' })
        .children([
          text('Cache Performance:', { class: 'font-semibold mb-2' }),
          
          // Cache statistics
          div({ class: 'bg-gray-800 p-4 rounded mb-4' })
            .children([
              text('Widget Cache Statistics:', { class: 'text-blue-300 mb-2' }),
              ...this.renderCacheStats()
            ]),
          
          // Performance test
          div({ class: 'mb-4' })
            .children([
              text('Performance Test:', { class: 'font-semibold mb-2' }),
              primaryButton('perf-test', 'Run Performance Test').render(),
              text('Creates 1000 widgets and measures time', { class: 'text-gray-400 text-sm mt-1' })
            ]),
          
          // Cache demonstration
          flexRow([
            primaryButton('cache-demo-1', 'Cached Instance').render(),
            secondaryButton('cache-demo-2', 'New Instance').render(),
            dangerButton('clear-cache', 'Clear Cache').render()
          ], { class: 'gap-2' })
        ])
    ])
    .build();
  }

  private renderBatchDemo(): Element {
    return flexColumn([
      text('Batch Operations', { class: 'text-xl font-bold mb-4' }),
      
      div({ class: 'mb-6' })
        .children([
          text('Creating Multiple Widgets:', { class: 'font-semibold mb-2' }),
          
          // Batch creation example
          div({ class: 'bg-gray-800 p-4 rounded mb-4' })
            .children([
              text('WidgetFactory.createBatch(widgets)', { class: 'font-mono text-green-300 mb-3' }),
              
              // Create button group with batch
              this.createBatchButtons(),
              
              text('All buttons created in single batch operation', 
                   { class: 'text-gray-400 text-sm mt-2' })
            ]),
          
          // Error handling in batch
          div({ class: 'mb-4' })
            .children([
              text('Error Handling:', { class: 'font-semibold mb-2' }),
              text('• Individual failures don\'t stop batch', { class: 'text-gray-300' }),
              text('• Continue on error option available', { class: 'text-gray-300' }),
              text('• Detailed error reporting for debugging', { class: 'text-gray-300' })
            ])
        ])
    ])
    .build();
  }

  private renderMigrationDemo(): Element {
    return flexColumn([
      text('Migration Example', { class: 'text-xl font-bold mb-4' }),
      
      flexRow([
        // Before (old pattern)
        div({ class: 'flex-1 mr-3' })
          .children([
            text('❌ Before (Old Pattern):', { class: 'text-red-400 font-semibold mb-2' }),
            div({ class: 'bg-gray-800 p-4 rounded text-sm' })
              .children([
                text('// Manual widget creation', { class: 'text-gray-400 mb-1' }),
                text('const button = new ButtonWidget({', { class: 'font-mono' }),
                text('  id: "old-button",', { class: 'font-mono ml-2' }),
                text('  text: "Click me",', { class: 'font-mono ml-2' }),
                text('  variant: "filled",', { class: 'font-mono ml-2' }),
                text('  color: "primary"', { class: 'font-mono ml-2' }),
                text('});', { class: 'font-mono' }),
                text('', { class: 'mb-2' }),
                text('// No validation', { class: 'text-red-300' }),
                text('// No caching', { class: 'text-red-300' }),
                text('// No error handling', { class: 'text-red-300' }),
                text('// Boilerplate code', { class: 'text-red-300' })
              ])
          ]),
        
        // After (factory pattern)
        div({ class: 'flex-1 ml-3' })
          .children([
            text('✅ After (Factory Pattern):', { class: 'text-green-400 font-semibold mb-2' }),
            div({ class: 'bg-gray-800 p-4 rounded text-sm' })
              .children([
                text('// Factory creation', { class: 'text-gray-400 mb-1' }),
                text('const button = createWidget("button", {', { class: 'font-mono' }),
                text('  id: "new-button",', { class: 'font-mono ml-2' }),
                text('  type: "button",', { class: 'font-mono ml-2' }),
                text('  text: "Click me"', { class: 'font-mono ml-2' }),
                text('});', { class: 'font-mono' }),
                text('', { class: 'mb-2' }),
                text('// Or convenience function:', { class: 'text-gray-400' }),
                text('primaryButton("btn", "Click me")', { class: 'font-mono' }),
                text('', { class: 'mb-2' }),
                text('✓ Automatic validation', { class: 'text-green-300' }),
                text('✓ Instance caching', { class: 'text-green-300' }),
                text('✓ Error boundaries', { class: 'text-green-300' }),
                text('✓ Performance monitoring', { class: 'text-green-300' })
              ])
          ])
      ], { class: 'mb-6' }),
      
      // Live comparison
      div()
        .children([
          text('Live Comparison:', { class: 'font-semibold mb-3' }),
          flexRow([
            div({ class: 'flex-1 text-center' })
              .children([
                text('Factory Pattern', { class: 'font-semibold mb-2' }),
                primaryButton('factory-example', 'Factory Button').render()
              ]),
            div({ class: 'flex-1 text-center' })
              .children([
                text('Convenience Functions', { class: 'font-semibold mb-2' }),
                flexRow([
                  primaryButton('conv-1', 'Primary').render(),
                  secondaryButton('conv-2', 'Secondary').render()
                ], { class: 'gap-2 justify-center' })
              ])
          ])
        ])
    ])
    .build();
  }

  // Helper methods for creating widgets
  private createFactoryButton(id: string, text: string): Element {
    const config: ButtonConfig = {
      id,
      type: 'button',
      text,
      variant: 'filled',
      color: 'primary'
    };
    
    const widget = WidgetFactory.create('button', config);
    return widget.render();
  }

  private createGenericButton(id: string, text: string): Element {
    const widget = createWidget('button', {
      id,
      type: 'button',
      text,
      variant: 'outlined',
      color: 'secondary'
    });
    return widget.render();
  }

  private createValidButton(id: string, text: string): Element {
    try {
      const widget = WidgetFactory.create('button', {
        id,
        type: 'button',
        text,
        variant: 'filled',
        color: 'success'
      });
      return widget.render();
    } catch (error) {
      return text(`Error: ${error}`, { class: 'text-red-400' }).build();
    }
  }

  private createMinimalButton(id: string, text: string): Element {
    // Only provide required fields - defaults will be applied
    const widget = WidgetFactory.create('button', {
      id,
      type: 'button',
      text
    });
    return widget.render();
  }

  private createDisabledButton(id: string, text: string): Element {
    const widget = WidgetFactory.create('button', {
      id,
      type: 'button',
      text,
      disabled: true
    });
    return widget.render();
  }

  private createBatchButtons(): Element {
    const buttonConfigs = [
      { type: 'button', config: { id: 'batch-1', type: 'button' as const, text: 'Batch 1', color: 'primary' as const } },
      { type: 'button', config: { id: 'batch-2', type: 'button' as const, text: 'Batch 2', color: 'secondary' as const } },
      { type: 'button', config: { id: 'batch-3', type: 'button' as const, text: 'Batch 3', color: 'success' as const } },
      { type: 'button', config: { id: 'batch-4', type: 'button' as const, text: 'Batch 4', color: 'warning' as const } }
    ];

    try {
      const widgets = WidgetFactory.createBatch(buttonConfigs);
      const elements = widgets.map(w => w.render());
      
      return flexRow(elements, { class: 'gap-2' }).build();
    } catch (error) {
      return text(`Batch creation failed: ${error}`, { class: 'text-red-400' }).build();
    }
  }

  private renderCacheStats(): Element[] {
    const stats = WidgetFactory.getStats();
    
    return [
      text(`Total instances: ${stats.cache.totalInstances}`, { class: 'text-white' }),
      text(`Memory usage: ${stats.cache.memoryUsage}`, { class: 'text-white' }),
      text('Type distribution:', { class: 'text-gray-300 mt-2' }),
      ...Object.entries(stats.cache.typeDistribution).map(([type, count]) =>
        text(`  ${type}: ${count}`, { class: 'text-gray-400 ml-2' })
      )
    ].map(t => t.build());
  }
}

// Run the demo
async function runDemo() {
  const demo = new WidgetFactoryDemo();

  const app = createApp({
    component: () => demo.render(),
    fullscreen: true
  });

  // Simulate some key presses for demo
  demo.refreshStats();

  await app.run();
}

if (import.meta.main) {
  runDemo().catch(console.error);
}

export { WidgetFactoryDemo };