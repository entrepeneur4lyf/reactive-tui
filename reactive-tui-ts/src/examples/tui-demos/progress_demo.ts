#!/usr/bin/env bun
/**
 * Progress Widget Demo - TypeScript
 * 
 * Demonstrates linear, circular, and custom progress indicators
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import { 
  linearProgress, circularProgress, arcProgress,
  createLinearProgress, createCircularProgress,
  ProgressBuilder, ProgressStyle
} from '../../packages/tui-bun/src/widgets/progress';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

class ProgressDemo implements Component {
  private downloads: { [key: string]: number } = {
    file1: 0,
    file2: 0,
    file3: 0,
    batch: 0
  };
  
  private tasks = [
    { name: 'Initializing', progress: 100 },
    { name: 'Loading Assets', progress: 75 },
    { name: 'Compiling Code', progress: 45 },
    { name: 'Running Tests', progress: 20 },
    { name: 'Building', progress: 0 }
  ];
  
  private cpuUsage = 45;
  private memoryUsage = 67;
  private diskUsage = 82;
  private networkUsage = 23;
  
  private animationFrame?: NodeJS.Timeout;
  private paused = false;

  constructor() {
    this.startAnimation();
  }

  startAnimation() {
    const animate = () => {
      if (!this.paused) {
        // Simulate downloads
        Object.keys(this.downloads).forEach(key => {
          if (this.downloads[key] < 100) {
            this.downloads[key] = Math.min(100, this.downloads[key] + Math.random() * 3);
          }
        });
        
        // Simulate system usage
        this.cpuUsage = 40 + Math.random() * 40;
        this.memoryUsage = 50 + Math.random() * 30;
        this.networkUsage = Math.random() * 100;
        
        // Update tasks
        this.tasks.forEach(task => {
          if (task.progress < 100) {
            task.progress = Math.min(100, task.progress + Math.random() * 2);
          }
        });
      }
      
      this.animationFrame = setTimeout(animate, 100);
    };
    animate();
  }

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case ' ':
        this.paused = !this.paused;
        return true;
      
      case 'r':
        // Reset all progress
        Object.keys(this.downloads).forEach(key => {
          this.downloads[key] = 0;
        });
        this.tasks.forEach(task => {
          task.progress = Math.random() * 30;
        });
        return true;
      
      case 'q':
      case 'Q':
        if (this.animationFrame) clearTimeout(this.animationFrame);
        process.exit(0);
    }
    
    return false;
  }

  render(): Element {
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            text('📊 Progress Widget Demo', { class: 'text-2xl font-bold mb-2' }),
            text(this.paused ? '⏸️ Paused' : '▶️ Running', { class: 'text-gray-400' })
          ]),
        
        // Main content - scrollable
        div({ class: 'flex-1 overflow-auto p-8' })
          .children([
            // Linear Progress Examples
            div({ class: 'mb-8' })
              .children([
                text('Linear Progress', { class: 'text-xl font-bold mb-4' }),
                
                // Basic linear progress
                div({ class: 'mb-4' })
                  .children([
                    text('Basic Progress', { class: 'text-sm text-gray-400 mb-1' }),
                    linearProgress({
                      value: 75,
                      max: 100,
                      showPercentage: true
                    })
                  ]),
                
                // Colored progress bars
                div({ class: 'space-y-3' })
                  .children([
                    div()
                      .children([
                        text('Primary', { class: 'text-sm text-gray-400 mb-1' }),
                        createLinearProgress({
                          value: 60,
                          color: 'primary',
                          showPercentage: true
                        })
                      ]),
                    div()
                      .children([
                        text('Success', { class: 'text-sm text-gray-400 mb-1' }),
                        createLinearProgress({
                          value: 85,
                          color: 'success',
                          showPercentage: true
                        })
                      ]),
                    div()
                      .children([
                        text('Warning', { class: 'text-sm text-gray-400 mb-1' }),
                        createLinearProgress({
                          value: 45,
                          color: 'warning',
                          showPercentage: true
                        })
                      ]),
                    div()
                      .children([
                        text('Error', { class: 'text-sm text-gray-400 mb-1' }),
                        createLinearProgress({
                          value: 25,
                          color: 'error',
                          showPercentage: true
                        })
                      ])
                  ])
              ]),
            
            // Download Progress
            div({ class: 'mb-8' })
              .children([
                text('Download Progress', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    div()
                      .children([
                        div({ class: 'flex justify-between text-sm mb-1' })
                          .children([
                            text('document.pdf'),
                            text(`${Math.round(this.downloads.file1)}%`, { class: 'text-gray-400' })
                          ]),
                        linearProgress({
                          value: this.downloads.file1,
                          style: ProgressStyle.Striped,
                          animated: this.downloads.file1 < 100
                        })
                      ]),
                    div()
                      .children([
                        div({ class: 'flex justify-between text-sm mb-1' })
                          .children([
                            text('video.mp4'),
                            text(`${Math.round(this.downloads.file2)}%`, { class: 'text-gray-400' })
                          ]),
                        linearProgress({
                          value: this.downloads.file2,
                          style: ProgressStyle.Striped,
                          animated: this.downloads.file2 < 100
                        })
                      ]),
                    div()
                      .children([
                        div({ class: 'flex justify-between text-sm mb-1' })
                          .children([
                            text('archive.zip'),
                            text(`${Math.round(this.downloads.file3)}%`, { class: 'text-gray-400' })
                          ]),
                        linearProgress({
                          value: this.downloads.file3,
                          style: ProgressStyle.Striped,
                          animated: this.downloads.file3 < 100
                        })
                      ])
                  ])
              ]),
            
            // Task Progress
            div({ class: 'mb-8' })
              .children([
                text('Task Progress', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children(this.tasks.map(task => 
                    div()
                      .children([
                        div({ class: 'flex justify-between text-sm mb-1' })
                          .children([
                            text(task.name),
                            text(task.progress === 100 ? '✓ Complete' : `${Math.round(task.progress)}%`, 
                              { class: task.progress === 100 ? 'text-green-400' : 'text-gray-400' })
                          ]),
                        linearProgress({
                          value: task.progress,
                          color: task.progress === 100 ? 'success' : 'primary',
                          height: 'sm'
                        })
                      ])
                  ))
              ]),
            
            // Circular Progress
            div({ class: 'mb-8' })
              .children([
                text('Circular Progress', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-8' })
                  .children([
                    div({ class: 'text-center' })
                      .children([
                        circularProgress({
                          value: this.cpuUsage,
                          size: 'lg',
                          color: this.cpuUsage > 80 ? 'error' : 'primary',
                          label: 'CPU'
                        }),
                        text(`${Math.round(this.cpuUsage)}%`, { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        circularProgress({
                          value: this.memoryUsage,
                          size: 'lg',
                          color: this.memoryUsage > 80 ? 'warning' : 'primary',
                          label: 'RAM'
                        }),
                        text(`${Math.round(this.memoryUsage)}%`, { class: 'text-sm mt-2' })
                      ]),
                    div({ class: 'text-center' })
                      .children([
                        circularProgress({
                          value: this.diskUsage,
                          size: 'lg',
                          color: this.diskUsage > 80 ? 'error' : 'primary',
                          label: 'Disk'
                        }),
                        text(`${Math.round(this.diskUsage)}%`, { class: 'text-sm mt-2' })
                      ])
                  ])
              ]),
            
            // Arc Progress (Gauge)
            div({ class: 'mb-8' })
              .children([
                text('Arc Progress (Gauges)', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'flex gap-8' })
                  .children([
                    div({ class: 'text-center' })
                      .children([
                        arcProgress({
                          value: this.networkUsage,
                          min: 0,
                          max: 100,
                          startAngle: 180,
                          endAngle: 360,
                          label: 'Network',
                          unit: 'Mbps'
                        }),
                        text(`${Math.round(this.networkUsage)} Mbps`, { class: 'text-sm mt-2' })
                      ])
                  ])
              ]),
            
            // Progress Sizes
            div({ class: 'mb-8' })
              .children([
                text('Progress Sizes', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    linearProgress({ value: 70, height: 'xs', label: 'Extra Small' }),
                    linearProgress({ value: 70, height: 'sm', label: 'Small' }),
                    linearProgress({ value: 70, height: 'md', label: 'Medium' }),
                    linearProgress({ value: 70, height: 'lg', label: 'Large' }),
                    linearProgress({ value: 70, height: 'xl', label: 'Extra Large' })
                  ])
              ]),
            
            // Indeterminate Progress
            div({ class: 'mb-8' })
              .children([
                text('Indeterminate Progress', { class: 'text-xl font-bold mb-4' }),
                div({ class: 'space-y-3' })
                  .children([
                    linearProgress({ indeterminate: true, label: 'Loading...' }),
                    linearProgress({ 
                      indeterminate: true, 
                      style: ProgressStyle.Pulse,
                      label: 'Processing...' 
                    })
                  ])
              ])
          ]),
        
        // Footer
        div({ class: 'bg-gray-800 p-4 border-t border-gray-700' })
          .children([
            text('[Space] Pause/Resume | [R] Reset | [Q] Quit', { class: 'text-center text-sm text-gray-400' })
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new ProgressDemo().render(),
    fullscreen: true
  });

  await app.run();
}

if (import.meta.main) {
  main().catch(console.error);
}