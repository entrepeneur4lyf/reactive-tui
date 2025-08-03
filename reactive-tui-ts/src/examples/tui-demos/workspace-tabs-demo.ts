#!/usr/bin/env bun
/**
 * Workspace & Tabs TUI Demo
 * 
 * Shows tabbed interface with multiple workspaces
 */

import { createApp, div, text, Component } from '../../packages/tui-bun/src';
import type { KeyEvent } from '../../packages/tui-bun/src/events';
import type { Element } from '../../packages/tui-bun/src/generated-types';

interface Tab {
  id: string;
  title: string;
  icon: string;
  content: () => Element;
}

interface Workspace {
  id: string;
  name: string;
  tabs: Tab[];
  activeTab: number;
}

// File Manager Tab
class FileManagerTab implements Component {
  private files = [
    { name: 'index.ts', type: 'file', size: '2.4 KB' },
    { name: 'package.json', type: 'file', size: '1.2 KB' },
    { name: 'src/', type: 'dir', size: '-' },
    { name: 'tests/', type: 'dir', size: '-' },
    { name: 'README.md', type: 'file', size: '4.1 KB' }
  ];
  
  private selectedIndex = 0;
  
  handleKeyPress(key: KeyEvent): boolean {
    switch (key.data.key) {
      case 'ArrowUp':
        this.selectedIndex = Math.max(0, this.selectedIndex - 1);
        return true;
      case 'ArrowDown':
        this.selectedIndex = Math.min(this.files.length - 1, this.selectedIndex + 1);
        return true;
      case 'Enter':
        const file = this.files[this.selectedIndex];
        console.log(`Opening ${file.name}`);
        return true;
    }
    return false;
  }
  
  render(): Element {
    return div({ class: 'p-4' })
      .children([
        text('File Explorer', { class: 'text-xl font-bold mb-4' }),
        
        // File list
        div({ class: 'bg-gray-800 rounded border border-gray-700' })
          .children(this.files.map((file, index) => 
            div({ 
              class: `p-2 flex justify-between ${
                index === this.selectedIndex ? 'bg-blue-700' : 'hover:bg-gray-700'
              }`
            })
            .children([
              div({ class: 'flex items-center' })
                .children([
                  text(file.type === 'dir' ? '📁 ' : '📄 '),
                  text(file.name)
                ]),
              text(file.size, { class: 'text-gray-400' })
            ])
          ))
      ])
      .build();
  }
}

// Terminal Tab
class TerminalTab implements Component {
  private output: string[] = [
    '$ npm install',
    'added 152 packages in 3.2s',
    '$ npm run build',
    'Building project...',
    'Build completed successfully!',
    '$ '
  ];
  
  render(): Element {
    return div({ class: 'p-4 bg-black font-mono' })
      .children([
        text('Terminal', { class: 'text-xl font-bold mb-4 text-green-400' }),
        
        div({ class: 'text-green-400' })
          .children(this.output.map(line => 
            div().child(text(line))
          ))
      ])
      .build();
  }
}

// Editor Tab
class EditorTab implements Component {
  private code = [
    'function fibonacci(n: number): number {',
    '  if (n <= 1) return n;',
    '  return fibonacci(n - 1) + fibonacci(n - 2);',
    '}',
    '',
    'console.log(fibonacci(10)); // 55'
  ];
  
  render(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Code Editor', { class: 'text-xl font-bold mb-4' }),
        
        div({ class: 'bg-gray-900 rounded border border-gray-700 p-4 font-mono' })
          .children(this.code.map((line, index) => 
            div({ class: 'flex' })
              .children([
                text(`${index + 1}`.padStart(3), { class: 'text-gray-500 mr-4' }),
                text(line, { class: 'text-gray-300' })
              ])
          ))
      ])
      .build();
  }
}

// Main Workspace Manager
class WorkspaceManager implements Component {
  private workspaces: Workspace[] = [
    {
      id: 'dev',
      name: 'Development',
      tabs: [
        { id: 'files', title: 'Files', icon: '📁', content: () => new FileManagerTab().render() },
        { id: 'editor', title: 'Editor', icon: '📝', content: () => new EditorTab().render() },
        { id: 'terminal', title: 'Terminal', icon: '💻', content: () => new TerminalTab().render() }
      ],
      activeTab: 0
    },
    {
      id: 'debug',
      name: 'Debug',
      tabs: [
        { id: 'console', title: 'Console', icon: '🐛', content: () => this.createDebugContent() },
        { id: 'network', title: 'Network', icon: '🌐', content: () => this.createNetworkContent() }
      ],
      activeTab: 0
    }
  ];
  
  private activeWorkspace = 0;
  
  createDebugContent(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Debug Console', { class: 'text-xl font-bold mb-4' }),
        div({ class: 'bg-red-900 border border-red-700 rounded p-3 mb-2' })
          .child(text('Error: Cannot find module "missing-dep"')),
        div({ class: 'bg-yellow-900 border border-yellow-700 rounded p-3 mb-2' })
          .child(text('Warning: Deprecated API usage detected')),
        div({ class: 'bg-blue-900 border border-blue-700 rounded p-3' })
          .child(text('Info: Server started on port 3000'))
      ])
      .build();
  }
  
  createNetworkContent(): Element {
    return div({ class: 'p-4' })
      .children([
        text('Network Monitor', { class: 'text-xl font-bold mb-4' }),
        div({ class: 'space-y-2' })
          .children([
            'GET /api/users - 200 OK (45ms)',
            'POST /api/login - 201 Created (123ms)',
            'GET /api/profile - 401 Unauthorized (12ms)',
            'PUT /api/settings - 204 No Content (67ms)'
          ].map(req => 
            div({ class: 'bg-gray-800 rounded p-2 font-mono text-sm' })
              .child(text(req))
          ))
      ])
      .build();
  }
  
  handleKeyPress(key: KeyEvent): boolean {
    const workspace = this.workspaces[this.activeWorkspace];
    
    switch (key.data.key) {
      case 'Tab':
        if (key.data.ctrl) {
          // Switch workspace with Ctrl+Tab
          this.activeWorkspace = (this.activeWorkspace + 1) % this.workspaces.length;
        } else {
          // Switch tab within workspace
          workspace.activeTab = (workspace.activeTab + 1) % workspace.tabs.length;
        }
        return true;
        
      case '1':
      case '2':
      case '3':
        const tabIndex = parseInt(key.data.key) - 1;
        if (tabIndex < workspace.tabs.length) {
          workspace.activeTab = tabIndex;
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
    const workspace = this.workspaces[this.activeWorkspace];
    const activeTab = workspace.tabs[workspace.activeTab];
    
    return div({ class: 'flex flex-col h-full bg-gray-900' })
      .children([
        // Workspace selector
        div({ class: 'bg-gray-800 p-2 flex gap-4 border-b border-gray-700' })
          .children(this.workspaces.map((ws, index) => 
            div({ 
              class: `px-3 py-1 rounded cursor-pointer ${
                index === this.activeWorkspace 
                  ? 'bg-blue-700 text-white' 
                  : 'bg-gray-700 text-gray-300'
              }`
            })
            .child(text(ws.name))
          )),
        
        // Tab bar
        div({ class: 'bg-gray-800 px-4 flex gap-2 border-b border-gray-700' })
          .children(workspace.tabs.map((tab, index) => 
            div({ 
              class: `px-4 py-2 flex items-center gap-2 border-b-2 ${
                index === workspace.activeTab 
                  ? 'border-blue-500 text-white bg-gray-700' 
                  : 'border-transparent text-gray-400 hover:text-white'
              }`
            })
            .children([
              text(tab.icon),
              text(tab.title)
            ])
          )),
        
        // Tab content
        div({ class: 'flex-1 overflow-auto' })
          .child(activeTab.content()),
        
        // Status bar
        div({ class: 'bg-gray-800 p-2 border-t border-gray-700 flex justify-between text-sm text-gray-400' })
          .children([
            text(`Workspace: ${workspace.name} | Tab: ${activeTab.title}`),
            text('[Tab] Switch Tab | [Ctrl+Tab] Switch Workspace | [1-3] Quick Tab | [Q] Quit')
          ])
      ])
      .build();
  }
}

// Run the app
async function main() {
  const app = createApp({
    component: () => new WorkspaceManager().render()
  });

  await app.run();
}

main().catch(console.error);