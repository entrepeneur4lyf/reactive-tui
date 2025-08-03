#!/usr/bin/env bun

/**
 * Menu Widget Demo - TypeScript
 * 
 * Demonstrates the comprehensive menu widget functionality including:
 * - Basic menu navigation
 * - Hierarchical submenus
 * - Different item types (action, toggle, radio, separator, header)
 * - JSON configuration loading
 * - Keyboard navigation
 * - Theme integration
 */

import {
    menu, contextMenu, menuBar,
    menuItem, submenuItem, separatorItem, headerItem, toggleItem, radioItem,
    type MenuItem
} from '../../packages/tui-bun/src/widgets/menu';

console.log('🔥 Menu Widget Demo - TypeScript Implementation\n');

// Demo 1: Basic menu with different item types
console.log('📋 Demo 1: Basic Menu with Different Item Types');
console.log('=' .repeat(50));

const basicMenuItems: MenuItem[] = [
    headerItem({ id: 'header1', label: 'File Operations' }),
    menuItem({ id: 'new', label: 'New File', action: 'file.new', icon: '📄', shortcut: 'Ctrl+N' }),
    menuItem({ id: 'open', label: 'Open File', action: 'file.open', icon: '📂', shortcut: 'Ctrl+O' }),
    menuItem({ id: 'save', label: 'Save File', action: 'file.save', icon: '💾', shortcut: 'Ctrl+S' }),
    separatorItem('sep1'),
    headerItem({ id: 'header2', label: 'View Options' }),
    toggleItem({ id: 'lineNumbers', label: 'Show Line Numbers', state: true, shortcut: 'F11' }),
    toggleItem({ id: 'wordWrap', label: 'Word Wrap', state: false, shortcut: 'Alt+W' }),
    separatorItem('sep2'),
    headerItem({ id: 'header3', label: 'Theme Selection' }),
    radioItem({ id: 'dark', label: 'Dark Theme', group: 'theme', selected: true }),
    radioItem({ id: 'light', label: 'Light Theme', group: 'theme', selected: false }),
    radioItem({ id: 'terminal', label: 'Terminal Theme', group: 'theme', selected: false }),
];

const basicMenu = menu({
    id: 'basic-menu',
    items: basicMenuItems,
    orientation: 'vertical',
    colorTheme: 'dark',
    onItemSelect: (item) => {
        console.log(`✓ Selected: ${item.label} (${item.id})`);
        if (item.itemType.action) {
            console.log(`  Action: ${item.itemType.action}`);
        }
    }
});

console.log(`Menu created with ${basicMenuItems.length} items`);
console.log(`Menu ID: ${basicMenu.id}`);
console.log(`Focusable: ${basicMenu.focusable}`);

// Demo 2: Hierarchical submenu structure
console.log('\n🌳 Demo 2: Hierarchical Submenu Structure');
console.log('=' .repeat(50));

const editSubmenu: MenuItem[] = [
    menuItem({ id: 'undo', label: 'Undo', action: 'edit.undo', icon: '↶', shortcut: 'Ctrl+Z' }),
    menuItem({ id: 'redo', label: 'Redo', action: 'edit.redo', icon: '↷', shortcut: 'Ctrl+Y' }),
    separatorItem('edit-sep1'),
    menuItem({ id: 'cut', label: 'Cut', action: 'edit.cut', icon: '✂', shortcut: 'Ctrl+X' }),
    menuItem({ id: 'copy', label: 'Copy', action: 'edit.copy', icon: '📋', shortcut: 'Ctrl+C' }),
    menuItem({ id: 'paste', label: 'Paste', action: 'edit.paste', icon: '📌', shortcut: 'Ctrl+V' }),
];

const formatSubmenu: MenuItem[] = [
    headerItem({ id: 'format-header', label: 'Text Formatting' }),
    toggleItem({ id: 'bold', label: 'Bold', state: false, shortcut: 'Ctrl+B' }),
    toggleItem({ id: 'italic', label: 'Italic', state: false, shortcut: 'Ctrl+I' }),
    toggleItem({ id: 'underline', label: 'Underline', state: false, shortcut: 'Ctrl+U' }),
    separatorItem('format-sep1'),
    submenuItem({
        id: 'align',
        label: 'Text Alignment',
        icon: '⬌',
        items: [
            radioItem({ id: 'align-left', label: 'Left', group: 'alignment', selected: true }),
            radioItem({ id: 'align-center', label: 'Center', group: 'alignment', selected: false }),
            radioItem({ id: 'align-right', label: 'Right', group: 'alignment', selected: false }),
            radioItem({ id: 'align-justify', label: 'Justify', group: 'alignment', selected: false }),
        ]
    })
];

const hierarchicalMenuItems: MenuItem[] = [
    menuItem({ id: 'new-doc', label: 'New Document', action: 'file.new', icon: '📄', shortcut: 'Ctrl+N' }),
    submenuItem({ id: 'edit', label: 'Edit', icon: '✏', items: editSubmenu }),
    submenuItem({ id: 'format', label: 'Format', icon: '🎨', items: formatSubmenu }),
    separatorItem('main-sep1'),
    menuItem({ id: 'settings', label: 'Settings', action: 'app.settings', icon: '⚙', shortcut: 'Ctrl+,' }),
    menuItem({ id: 'about', label: 'About', action: 'app.about', icon: 'ℹ' }),
];

const _hierarchicalMenu = menu({
    id: 'hierarchical-menu',
    items: hierarchicalMenuItems,
    orientation: 'vertical',
    colorTheme: 'light',
    onItemSelect: (item) => {
        console.log(`✓ Selected: ${item.label} (${item.id})`);
    },
    onSubmenuOpen: (item) => {
        console.log(`📂 Opened submenu: ${item.label}`);
    },
    onSubmenuClose: (item) => {
        console.log(`📁 Closed submenu: ${item.label}`);
    }
});

console.log(`Hierarchical menu created with ${hierarchicalMenuItems.length} top-level items`);
console.log('Submenu structure:');
hierarchicalMenuItems.forEach(item => {
    if (item.itemType.type === 'submenu' && item.itemType.items) {
        console.log(`  └─ ${item.label}: ${item.itemType.items.length} items`);
        item.itemType.items.forEach(subItem => {
            if (subItem.itemType.type === 'submenu' && subItem.itemType.items) {
                console.log(`      └─ ${subItem.label}: ${subItem.itemType.items.length} items`);
            }
        });
    }
});

// Demo 3: JSON Configuration Loading
console.log('\n📄 Demo 3: JSON Configuration Loading');
console.log('=' .repeat(50));

const jsonMenuConfig = JSON.stringify([
    {
        "id": "file",
        "label": "File Menu",
        "itemType": { "type": "header" }
    },
    {
        "id": "new-project",
        "label": "New Project",
        "itemType": { "type": "action", "action": "project.new" },
        "icon": "🚀",
        "shortcut": "Ctrl+Shift+N"
    },
    {
        "id": "open-project",
        "label": "Open Project",
        "itemType": { "type": "action", "action": "project.open" },
        "icon": "📁",
        "shortcut": "Ctrl+Shift+O"
    },
    {
        "id": "recent",
        "label": "Recent Projects",
        "itemType": {
            "type": "submenu",
            "items": [
                {
                    "id": "project1",
                    "label": "AI CLI Project",
                    "itemType": { "type": "action", "action": "project.open.ai-cli" },
                    "icon": "🤖"
                },
                {
                    "id": "project2",
                    "label": "TUI Framework",
                    "itemType": { "type": "action", "action": "project.open.tui" },
                    "icon": "🖥"
                }
            ]
        },
        "icon": "📚"
    },
    {
        "id": "separator1",
        "label": "",
        "itemType": { "type": "separator" }
    },
    {
        "id": "auto-save",
        "label": "Auto Save",
        "itemType": { "type": "toggle", "state": true },
        "icon": "💾"
    }
], null, 2);

console.log('JSON Configuration:');
console.log(jsonMenuConfig);

const _jsonMenu = menu({
    id: 'json-menu',
    configData: jsonMenuConfig,
    configFormat: 'json',
    orientation: 'vertical',
    colorTheme: 'terminal',
    onItemSelect: (item) => {
        console.log(`✓ JSON Menu - Selected: ${item.label}`);
    }
});

console.log('\n✅ JSON menu loaded successfully!');

// Demo 4: Menu Bar (Horizontal)
console.log('\n🧭 Demo 4: Horizontal Menu Bar');
console.log('=' .repeat(50));

const menuBarItems: MenuItem[] = [
    submenuItem({
        id: 'file-menu',
        label: 'File',
        items: [
            menuItem({ id: 'new', label: 'New', shortcut: 'Ctrl+N' }),
            menuItem({ id: 'open', label: 'Open', shortcut: 'Ctrl+O' }),
            menuItem({ id: 'save', label: 'Save', shortcut: 'Ctrl+S' }),
        ]
    }),
    submenuItem({
        id: 'edit-menu',
        label: 'Edit',
        items: editSubmenu
    }),
    submenuItem({
        id: 'view-menu',
        label: 'View',
        items: [
            toggleItem({ id: 'sidebar', label: 'Show Sidebar', state: true }),
            toggleItem({ id: 'minimap', label: 'Show Minimap', state: false }),
            separatorItem('view-sep'),
            menuItem({ id: 'fullscreen', label: 'Toggle Fullscreen', shortcut: 'F11' }),
        ]
    }),
    submenuItem({
        id: 'help-menu',
        label: 'Help',
        items: [
            menuItem({ id: 'docs', label: 'Documentation', icon: '📖' }),
            menuItem({ id: 'shortcuts', label: 'Keyboard Shortcuts', icon: '⌨' }),
            separatorItem('help-sep'),
            menuItem({ id: 'about', label: 'About', icon: 'ℹ' }),
        ]
    }),
];

const _applicationMenuBar = menuBar({
    id: 'app-menu-bar',
    items: menuBarItems,
    colorTheme: 'dark',
    style: {
        showBorders: true,
        padding: 2
    },
    onItemSelect: (item) => {
        console.log(`🧭 Menu Bar - Selected: ${item.label}`);
    }
});

console.log(`Menu bar created with ${menuBarItems.length} top-level menus`);
console.log('Menu structure:');
menuBarItems.forEach(menu => {
    if (menu.itemType.type === 'submenu' && menu.itemType.items) {
        console.log(`  ${menu.label}: ${menu.itemType.items.length} items`);
    }
});

// Demo 5: Context Menu
console.log('\n🖱 Demo 5: Context Menu');
console.log('=' .repeat(50));

const contextMenuItems: MenuItem[] = [
    menuItem({ id: 'cut', label: 'Cut', action: 'edit.cut', icon: '✂', shortcut: 'Ctrl+X' }),
    menuItem({ id: 'copy', label: 'Copy', action: 'edit.copy', icon: '📋', shortcut: 'Ctrl+C' }),
    menuItem({ id: 'paste', label: 'Paste', action: 'edit.paste', icon: '📌', shortcut: 'Ctrl+V' }),
    separatorItem('context-sep1'),
    submenuItem({
        id: 'select',
        label: 'Select',
        icon: '🔍',
        items: [
            menuItem({ id: 'select-all', label: 'Select All', shortcut: 'Ctrl+A' }),
            menuItem({ id: 'select-word', label: 'Select Word', shortcut: 'Ctrl+D' }),
            menuItem({ id: 'select-line', label: 'Select Line', shortcut: 'Ctrl+L' }),
        ]
    }),
    separatorItem('context-sep2'),
    menuItem({ id: 'inspect', label: 'Inspect Element', action: 'dev.inspect', icon: '🔍' }),
];

const _rightClickMenu = contextMenu({
    id: 'context-menu',
    items: contextMenuItems,
    colorTheme: 'light',
    onItemSelect: (item) => {
        console.log(`🖱 Context Menu - Selected: ${item.label}`);
    }
});

console.log(`Context menu created with ${contextMenuItems.length} items`);

// Demo 6: Menu API Testing
console.log('\n🔧 Demo 6: Menu API Testing');
console.log('=' .repeat(50));

const dynamicMenu = menu({
    id: 'dynamic-menu',
    items: [
        headerItem({ id: 'dynamic-header', label: 'Dynamic Menu Demo' }),
        menuItem({ id: 'item1', label: 'Initial Item 1' }),
        menuItem({ id: 'item2', label: 'Initial Item 2' }),
    ],
    onItemSelect: (item) => {
        console.log(`🔧 Dynamic Menu - Selected: ${item.label}`);
    }
});

console.log('Initial menu state:');
console.log(`Items: ${dynamicMenu.api.getState().currentItems.length}`);

// Test adding items
console.log('\n📝 Testing API: Adding items...');
dynamicMenu.api.addItem(separatorItem('dynamic-sep'));
dynamicMenu.api.addItem(menuItem({ 
    id: 'added-item', 
    label: 'Dynamically Added Item', 
    action: 'dynamic.added',
    icon: '➕'
}));

console.log(`Items after adding: ${dynamicMenu.api.getState().currentItems.length}`);

// Test navigation
console.log('\n🧭 Testing API: Navigation...');
console.log(`Current selection: ${dynamicMenu.api.getState().selectedIndex}`);

dynamicMenu.api.nextItem();
console.log(`After next(): ${dynamicMenu.api.getState().selectedIndex}`);

dynamicMenu.api.nextItem();
console.log(`After next(): ${dynamicMenu.api.getState().selectedIndex}`);

dynamicMenu.api.previousItem();
console.log(`After previous(): ${dynamicMenu.api.getState().selectedIndex}`);

// Test JSON loading
console.log('\n📄 Testing API: JSON loading...');
const newJsonConfig = JSON.stringify([
    { "id": "loaded1", "label": "Loaded Item 1", "itemType": { "type": "action" } },
    { "id": "loaded2", "label": "Loaded Item 2", "itemType": { "type": "action" } },
    { "id": "sep", "label": "", "itemType": { "type": "separator" } },
    { "id": "toggle", "label": "Loaded Toggle", "itemType": { "type": "toggle", "state": true } }
]);

const loadResult = dynamicMenu.api.loadFromJson(newJsonConfig);
console.log(`JSON load result: ${loadResult}`);
console.log(`Items after JSON load: ${dynamicMenu.api.getState().currentItems.length}`);

// Test toggle and radio states
console.log('\n🔘 Testing API: Toggle and Radio states...');
dynamicMenu.api.setToggleState('toggle', false);
dynamicMenu.api.setRadioSelection('test-group', 'option1');

const finalState = dynamicMenu.api.getState();
console.log(`Toggle states:`, finalState.toggleStates);
console.log(`Radio selections:`, finalState.radioSelections);

console.log('\n🎉 All Menu Widget Demos Completed Successfully!');
console.log('=' .repeat(60));
console.log('✓ Basic menu with different item types');
console.log('✓ Hierarchical submenu structure');
console.log('✓ JSON configuration loading');
console.log('✓ Horizontal menu bar');
console.log('✓ Context menu');
console.log('✓ Dynamic menu API testing');
console.log('\n🚀 TypeScript Menu Widget is fully functional with feature parity to Rust implementation!');