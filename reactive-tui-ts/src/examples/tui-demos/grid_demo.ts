#!/usr/bin/env bun
/**
 * Grid Layout Demo - TypeScript
 * 
 * Demonstrates the Grid widget for creating flexible grid-based layouts
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { grid } from '../../packages/tui-bun/src/widgets/grid';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class GridDemo implements Component {
  private currentDemo = 0;
  private demos = [
    { name: '2x2 Grid', rows: 2, cols: 2 },
    { name: '3x3 Grid', rows: 3, cols: 3 },
    { name: '1x4 Vertical', rows: 4, cols: 1 },
    { name: '4x1 Horizontal', rows: 1, cols: 4 },
    { name: '2x3 Grid', rows: 2, cols: 3 }
  ];

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'ArrowRight':
      case 'Tab':
        this.currentDemo = (this.currentDemo + 1) % this.demos.length;
        return true;
      
      case 'ArrowLeft':
        this.currentDemo = (this.currentDemo - 1 + this.demos.length) % this.demos.length;
        return true;
      
      case 'q':
      case 'Q':
        process.exit(0);
    }
    
    return false;
  }

  createGridCell(row: number, col: number, totalRows: number, totalCols: number): Element {
    const colors = [
      'bg-red-700', 'bg-blue-700', 'bg-green-700', 'bg-yellow-700',
      'bg-purple-700', 'bg-pink-700', 'bg-indigo-700', 'bg-gray-700',
      'bg-orange-700', 'bg-teal-700', 'bg-cyan-700', 'bg-lime-700'
    ];
    
    const index = row * totalCols + col;
    const color = colors[index % colors.length];
    
    return div({ class: `${color} p-4 rounded border border-gray-600 flex items-center justify-center` })
      .children([
        div({ class: 'text-center' })
          .children([
            text(`Cell ${index + 1}`, { class: 'text-lg font-bold' }),
            text(`(${row + 1}, ${col + 1})`, { class: 'text-sm text-gray-300' })
          ])
      ])
      .build();
  }

  render(): Element {
    const demo = this.demos[this.currentDemo];
    
    // Create grid cells
    const cells: Element[] = [];
    for (let row = 0; row < demo.rows; row++) {
      for (let col = 0; col < demo.cols; col++) {
        cells.push(this.createGridCell(row, col, demo.rows, demo.cols));
      }
    }
    
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('🔲 Grid Layout Demo', { class: 'text-xl font-bold mb-2' }),
            text(`Current: ${demo.name} (${demo.rows}×${demo.cols})`, { class: 'text-gray-400' })
          ]),
        
        // Grid container
        div({ class: 'flex-1 p-4' })
          .child(
            grid({
              rows: demo.rows,
              columns: demo.cols,
              gap: 2,
              class: 'h-full'
            }).children(cells).build()
          ),
        
        // Footer
        div({ class: 'bg-gray-800 p-2 border-t border-gray-700 text-center text-sm text-gray-400' })
          .child(text('[←→/Tab] Change Grid | [Q] Quit'))
      ])
      .build();
  }
}

// Standalone demo function
export function runGridDemo() {
  console.log('🔲 Grid Layout Demo\n');
  
  const demos = [
    { name: '2x2 Grid', rows: 2, cols: 2 },
    { name: '3x3 Grid', rows: 3, cols: 3 },
    { name: '1x4 Vertical Layout', rows: 4, cols: 1 },
    { name: '4x1 Horizontal Layout', rows: 1, cols: 4 }
  ];
  
  demos.forEach(demo => {
    console.log(`\n${demo.name}:`);
    console.log('─'.repeat(40));
    
    for (let row = 0; row < demo.rows; row++) {
      let rowStr = '│';
      for (let col = 0; col < demo.cols; col++) {
        const cellNum = row * demo.cols + col + 1;
        rowStr += ` Cell ${cellNum} `.padEnd(10) + '│';
      }
      console.log(rowStr);
      if (row < demo.rows - 1) {
        console.log('├' + '─'.repeat(10) + '┼'.repeat(demo.cols - 1) + '─'.repeat(10) + '┤');
      }
    }
    console.log('─'.repeat(40));
  });
}

// Run the app or demo
async function main() {
  if (process.argv.includes('--cli')) {
    runGridDemo();
  } else {
    const app = createApp({
      component: () => new GridDemo().render()
    });
    
    await app.run();
  }
}

if (import.meta.main) {
  main().catch(console.error);
}