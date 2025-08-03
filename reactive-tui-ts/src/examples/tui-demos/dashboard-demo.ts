#!/usr/bin/env bun
/**
 * Full Dashboard Demo
 * 
 * A complete multi-screen dashboard with real-time updates
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

// Progress bar component
class ProgressBar implements Component {
  constructor(
    private value: number,
    private max: number = 100,
    private color: string = 'bg-blue-500'
  ) {}

  render(): Element {
    const percentage = Math.min(100, (this.value / this.max) * 100);
    const width = Math.floor(percentage / 5); // 20 chars max width
    
    return div({ class: 'flex items-center gap-2' })
      .children([
        div({ class: 'w-20 bg-gray-700 rounded overflow-hidden' })
          .child(
            div({ class: `${this.color} h-2` })
              .child(text('█'.repeat(width)))
          ),
        text(`${percentage.toFixed(0)}%`, { class: 'text-gray-400 text-sm' })
      ])
      .build();
  }
}

// Metric card component
class MetricCard implements Component {
  constructor(
    private title: string,
    private value: string | number,
    private change?: string,
    private icon?: string
  ) {}

  render(): Element {
    const isPositive = this.change?.startsWith('+');
    const changeColor = isPositive ? 'text-green-400' : 'text-red-400';
    
    return div({ class: 'bg-gray-800 rounded p-4 border border-gray-700' })
      .children([
        div({ class: 'flex justify-between items-start mb-2' })
          .children([
            text(this.title, { class: 'text-gray-400 text-sm' }),
            ...(this.icon ? [text(this.icon)] : [])
          ]),
        text(String(this.value), { class: 'text-2xl font-bold text-white mb-1' }),
        ...(this.change ? [
          text(this.change, { class: `text-sm ${changeColor}` })
        ] : [])
      ])
      .build();
  }
}

// Main Dashboard
class Dashboard implements Component {
  private currentScreen = 0;
  private screens = ['overview', 'analytics', 'reports', 'settings'];
  
  // Sample data - in real app this would update
  private metrics = {
    users: 1234,
    revenue: '$48,293',
    conversion: '3.4%',
    activeNow: 89
  };
  
  private cpuUsage = 45;
  private memoryUsage = 67;
  private diskUsage = 82;
  
  private recentActivity = [
    { time: '2m ago', action: 'User login', user: 'john.doe' },
    { time: '5m ago', action: 'Payment received', user: 'jane.smith' },
    { time: '12m ago', action: 'New signup', user: 'bob.wilson' },
    { time: '23m ago', action: 'Report generated', user: 'admin' },
    { time: '1h ago', action: 'System backup', user: 'system' }
  ];

  private interval?: NodeJS.Timeout;

  constructor() {
    // Simulate real-time updates
    this.interval = setInterval(() => {
      this.cpuUsage = Math.floor(Math.random() * 100);
      this.memoryUsage = Math.floor(Math.random() * 100);
      this.metrics.activeNow = Math.floor(80 + Math.random() * 40);
    }, 2000);
  }

  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'Tab':
      case 'ArrowRight':
        this.currentScreen = (this.currentScreen + 1) % this.screens.length;
        return true;
      
      case 'ArrowLeft':
        this.currentScreen = (this.currentScreen - 1 + this.screens.length) % this.screens.length;
        return true;
      
      case 'q':
      case 'Q':
        if (this.interval) clearInterval(this.interval);
        process.exit(0);
      
      case '1':
      case '2':
      case '3':
      case '4':
        const index = parseInt(key.data.key) - 1;
        if (index < this.screens.length) {
          this.currentScreen = index;
          return true;
        }
        break;
    }
    
    return false;
  }

  renderOverview(): Element {
    return div({ class: 'p-4' })
      .children([
        // Metrics row
        div({ class: 'grid grid-cols-4 gap-4 mb-6' })
          .children([
            new MetricCard('Total Users', this.metrics.users, '+12.5%', '👥').render(),
            new MetricCard('Revenue', this.metrics.revenue, '+8.2%', '💰').render(),
            new MetricCard('Conversion', this.metrics.conversion, '-2.1%', '📈').render(),
            new MetricCard('Active Now', this.metrics.activeNow, '', '🟢').render()
          ]),
        
        // System status
        div({ class: 'bg-gray-800 rounded p-4 border border-gray-700 mb-6' })
          .children([
            text('System Status', { class: 'text-lg font-bold mb-4' }),
            
            div({ class: 'space-y-3' })
              .children([
                div({ class: 'flex justify-between items-center' })
                  .children([
                    text('CPU Usage'),
                    new ProgressBar(this.cpuUsage, 100, 'bg-blue-500').render()
                  ]),
                
                div({ class: 'flex justify-between items-center' })
                  .children([
                    text('Memory'),
                    new ProgressBar(this.memoryUsage, 100, 'bg-green-500').render()
                  ]),
                
                div({ class: 'flex justify-between items-center' })
                  .children([
                    text('Disk Space'),
                    new ProgressBar(this.diskUsage, 100, 'bg-red-500').render()
                  ])
              ])
          ]),
        
        // Recent activity
        div({ class: 'bg-gray-800 rounded p-4 border border-gray-700' })
          .children([
            text('Recent Activity', { class: 'text-lg font-bold mb-4' }),
            
            div({ class: 'space-y-2' })
              .children(this.recentActivity.map(activity => 
                div({ class: 'flex justify-between text-sm' })
                  .children([
                    div({ class: 'flex gap-4' })
                      .children([
                        text(activity.time, { class: 'text-gray-500' }),
                        text(activity.action),
                        text(activity.user, { class: 'text-blue-400' })
                      ]),
                  ])
              ))
          ])
      ])
      .build();
  }

  renderAnalytics(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Analytics Dashboard', { class: 'text-2xl font-bold mb-6' }),
        
        // Chart placeholder
        div({ class: 'bg-gray-800 rounded p-8 border border-gray-700 mb-6' })
          .children([
            text('📊 Traffic Overview', { class: 'text-lg font-bold mb-4' }),
            div({ class: 'h-32 flex items-end gap-2' })
              .children([40, 65, 45, 70, 55, 80, 60, 75, 50, 85, 70, 90].map(height => 
                div({ class: `bg-blue-500 w-8` })
                  .child(text('█'.repeat(Math.floor(height / 10))))
              ))
          ]),
        
        // Stats grid
        div({ class: 'grid grid-cols-3 gap-4' })
          .children([
            new MetricCard('Page Views', '142,394', '+23%', '👁️').render(),
            new MetricCard('Bounce Rate', '32.4%', '-5%', '📉').render(),
            new MetricCard('Avg Duration', '3m 42s', '+18s', '⏱️').render()
          ])
      ])
      .build();
  }

  renderReports(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Reports', { class: 'text-2xl font-bold mb-6' }),
        
        div({ class: 'space-y-4' })
          .children([
            'Monthly Revenue Report - March 2024',
            'User Engagement Analysis - Q1 2024',
            'System Performance Report - Week 12',
            'Marketing Campaign Results - Spring 2024',
            'Customer Satisfaction Survey - March 2024'
          ].map((report, index) => 
            div({ class: 'bg-gray-800 rounded p-4 border border-gray-700 flex justify-between items-center' })
              .children([
                text(report),
                text(index < 2 ? '📄 Download' : '⏳ Processing', { class: 'text-blue-400' })
              ])
          ))
      ])
      .build();
  }

  renderSettings(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Settings', { class: 'text-2xl font-bold mb-6' }),
        
        div({ class: 'space-y-4' })
          .children([
            div({ class: 'bg-gray-800 rounded p-4 border border-gray-700' })
              .children([
                text('Notifications', { class: 'font-bold mb-2' }),
                div({ class: 'space-y-2' })
                  .children([
                    text('[✓] Email notifications'),
                    text('[✓] Desktop alerts'),
                    text('[ ] SMS alerts'),
                    text('[✓] Weekly reports')
                  ])
              ]),
            
            div({ class: 'bg-gray-800 rounded p-4 border border-gray-700' })
              .children([
                text('Data & Privacy', { class: 'font-bold mb-2' }),
                div({ class: 'space-y-2' })
                  .children([
                    text('[✓] Analytics tracking'),
                    text('[ ] Share data with partners'),
                    text('[✓] Auto-backup enabled')
                  ])
              ])
          ])
      ])
      .build();
  }

  render(): Element {
    const screens = {
      overview: this.renderOverview(),
      analytics: this.renderAnalytics(),
      reports: this.renderReports(),
      settings: this.renderSettings()
    };
    
    return div({ class: 'flex flex-col h-full bg-gray-900 text-white' })
      .children([
        // Header
        div({ class: 'bg-gray-800 p-4 border-b border-gray-700' })
          .children([
            div({ class: 'flex justify-between items-center mb-2' })
              .children([
                text('🎯 Admin Dashboard', { class: 'text-xl font-bold' }),
                text(new Date().toLocaleTimeString(), { class: 'text-gray-400' })
              ]),
            
            // Navigation tabs
            div({ class: 'flex gap-4' })
              .children(this.screens.map((screen, index) => 
                div({ 
                  class: `px-4 py-2 rounded cursor-pointer capitalize ${
                    index === this.currentScreen 
                      ? 'bg-blue-700 text-white' 
                      : 'bg-gray-700 text-gray-300'
                  }`
                })
                .child(text(`${index + 1}. ${screen}`))
              ))
          ]),
        
        // Content area
        div({ class: 'flex-1 overflow-auto' })
          .child(screens[this.screens[this.currentScreen] as keyof typeof screens]),
        
        // Footer
        div({ class: 'bg-gray-800 p-2 border-t border-gray-700 text-center text-sm text-gray-400' })
          .child(text('[Tab/→] Next Screen | [←] Previous | [1-4] Jump | [Q] Quit'))
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new Dashboard().render()
  });

  await app.run();
}

main().catch(console.error);