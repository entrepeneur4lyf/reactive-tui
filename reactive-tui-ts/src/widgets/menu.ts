/**
 * Menu Widget - Hierarchical navigation with JSON/YAML configuration support
 * Features comprehensive menu functionality with keyboard navigation, submenus, and theming
 */

import type { ColorTheme } from '../themes/colors';
import { getColorTheme, getSemanticColor, getSemanticBackground } from '../themes/colors';

export type MenuOrientation = 'vertical' | 'horizontal';
export type MenuItemState = 'normal' | 'hovered' | 'selected' | 'disabled';

export interface MenuItemType {
    type: 'action' | 'submenu' | 'separator' | 'header' | 'toggle' | 'radio';
    action?: string;
    items?: MenuItem[];
    state?: boolean;
    group?: string;
    selected?: boolean;
}

export interface MenuItem {
    id: string;
    label: string;
    itemType: MenuItemType;
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
    visible?: boolean;
    cssClasses?: string[];
    tooltip?: string;
    data?: Record<string, string>;
}

export interface MenuStyle {
    background?: string;
    foreground?: string;
    hoverBackground?: string;
    hoverForeground?: string;
    selectedBackground?: string;
    selectedForeground?: string;
    disabledForeground?: string;
    borderColor?: string;
    separatorColor?: string;
    padding?: number;
    itemHeight?: number;
    showBorders?: boolean;
    showIcons?: boolean;
    showShortcuts?: boolean;
    submenuIndent?: number;
}

export interface MenuState {
    selectedIndex: number;
    navigationStack: MenuItem[][];
    currentItems: MenuItem[];
    expandedPaths: string[];
    radioSelections: Record<string, string>;
    toggleStates: Record<string, boolean>;
}

export interface MenuConfig {
    id: string;
    items?: MenuItem[];
    orientation?: MenuOrientation;
    style?: MenuStyle;
    cssClasses?: string[];
    visible?: boolean;
    focusable?: boolean;
    colorTheme?: string;
    
    // JSON/YAML support
    configData?: string;
    configFormat?: 'json' | 'yaml';
    
    // Event handlers
    onItemSelect?: (item: MenuItem, event?: any) => void;
    onItemHover?: (item: MenuItem, event?: any) => void;
    onSubmenuOpen?: (item: MenuItem, event?: any) => void;
    onSubmenuClose?: (item: MenuItem, event?: any) => void;
    onKeyPress?: (key: string, event?: any) => void;
}

function getDefaultMenuStyle(): MenuStyle {
    return {
        background: undefined,
        foreground: undefined,
        hoverBackground: '#4682B4',
        hoverForeground: '#FFFFFF',
        selectedBackground: '#6495ED',
        selectedForeground: '#FFFFFF',
        disabledForeground: '#808080',
        borderColor: '#C0C0C0',
        separatorColor: '#A9A9A9',
        padding: 1,
        itemHeight: 1,
        showBorders: true,
        showIcons: true,
        showShortcuts: true,
        submenuIndent: 2,
    };
}

function getDefaultMenuState(): MenuState {
    return {
        selectedIndex: 0,
        navigationStack: [],
        currentItems: [],
        expandedPaths: [],
        radioSelections: {},
        toggleStates: {},
    };
}

export function menu(config: MenuConfig): any {
    const {
        id,
        items = [],
        orientation = 'vertical',
        style = {},
        cssClasses = [],
        visible = true,
        focusable = true,
        colorTheme,
        configData,
        configFormat,
        onItemSelect,
        onItemHover,
        onSubmenuOpen,
        onSubmenuClose,
        onKeyPress
    } = config;

    // Parse configuration data if provided
    let parsedItems = items;
    if (configData && configFormat) {
        try {
            if (configFormat === 'json') {
                parsedItems = JSON.parse(configData) as MenuItem[];
            } else if (configFormat === 'yaml') {
                // Note: In a real implementation, you'd use a YAML parser like js-yaml
                console.warn('YAML parsing requires js-yaml library - using JSON fallback');
                parsedItems = JSON.parse(configData) as MenuItem[];
            }
        } catch (error) {
            console.error(`Failed to parse menu ${configFormat}:`, error);
        }
    }

    // Merge default style with provided style
    const mergedStyle = { ...getDefaultMenuStyle(), ...style };
    
    // Initialize state
    const menuState = getDefaultMenuState();
    menuState.currentItems = parsedItems;

    // Get theme
    const theme = getColorTheme(colorTheme);

    // Build CSS classes
    const classes = [
        'menu',
        `menu-${orientation}`,
        visible ? 'menu-visible' : 'menu-hidden',
        focusable ? 'menu-focusable' : '',
        ...cssClasses
    ].filter(Boolean);

    // Apply theme-based styling
    const themeStyle = getMenuThemeStyle(theme, mergedStyle);

    return {
        tag: 'div',
        id,
        classes,
        focusable,
        style: themeStyle,
        children: renderMenuItems(parsedItems, menuState, mergedStyle, theme, {
            onItemSelect,
            onItemHover,
            onSubmenuOpen,
            onSubmenuClose,
            onKeyPress
        }),
        
        // Menu API methods
        api: {
            addItem: (item: MenuItem) => {
                parsedItems.push(item);
                menuState.currentItems.push(item);
            },
            
            removeItem: (itemId: string) => {
                const index = parsedItems.findIndex(item => item.id === itemId);
                if (index !== -1) {
                    parsedItems.splice(index, 1);
                    menuState.currentItems.splice(index, 1);
                    return true;
                }
                return false;
            },
            
            getItem: (itemId: string) => {
                return parsedItems.find(item => item.id === itemId);
            },
            
            nextItem: () => {
                const visibleItems = menuState.currentItems
                    .map((item, index) => ({ item, index }))
                    .filter(({ item }) => item.visible !== false && item.enabled !== false);
                
                if (visibleItems.length > 0) {
                    const currentPos = visibleItems.findIndex(({ index }) => index === menuState.selectedIndex);
                    const nextPos = (currentPos + 1) % visibleItems.length;
                    menuState.selectedIndex = visibleItems[nextPos].index;
                }
            },
            
            previousItem: () => {
                const visibleItems = menuState.currentItems
                    .map((item, index) => ({ item, index }))
                    .filter(({ item }) => item.visible !== false && item.enabled !== false);
                
                if (visibleItems.length > 0) {
                    const currentPos = visibleItems.findIndex(({ index }) => index === menuState.selectedIndex);
                    const prevPos = currentPos === 0 ? visibleItems.length - 1 : currentPos - 1;
                    menuState.selectedIndex = visibleItems[prevPos].index;
                }
            },
            
            selectItem: () => {
                const selectedItem = menuState.currentItems[menuState.selectedIndex];
                if (selectedItem && selectedItem.enabled !== false) {
                    handleItemSelect(selectedItem, menuState, onItemSelect, onSubmenuOpen);
                }
            },
            
            enterSubmenu: () => {
                const selectedItem = menuState.currentItems[menuState.selectedIndex];
                if (selectedItem?.itemType.type === 'submenu' && selectedItem.itemType.items) {
                    menuState.navigationStack.push(menuState.currentItems);
                    menuState.currentItems = selectedItem.itemType.items;
                    menuState.selectedIndex = 0;
                    onSubmenuOpen?.(selectedItem);
                }
            },
            
            exitSubmenu: () => {
                if (menuState.navigationStack.length > 0) {
                    const previousItems = menuState.navigationStack.pop();
                    if (previousItems) {
                        menuState.currentItems = previousItems;
                        menuState.selectedIndex = 0;
                        onSubmenuClose?.(menuState.currentItems[0]);
                    }
                }
            },
            
            loadFromJson: (jsonData: string) => {
                try {
                    const newItems = JSON.parse(jsonData) as MenuItem[];
                    parsedItems.splice(0, parsedItems.length, ...newItems);
                    menuState.currentItems = parsedItems;
                    menuState.selectedIndex = 0;
                    return true;
                } catch (error) {
                    console.error('Failed to load menu from JSON:', error);
                    return false;
                }
            },
            
            getState: () => ({ ...menuState }),
            
            setToggleState: (itemId: string, state: boolean) => {
                menuState.toggleStates[itemId] = state;
            },
            
            setRadioSelection: (group: string, itemId: string) => {
                menuState.radioSelections[group] = itemId;
            }
        }
    };
}

function getMenuThemeStyle(theme: ColorTheme, style: MenuStyle): Record<string, string> {
    return {
        backgroundColor: style.background || getSemanticBackground(theme, 'panelBackground'),
        color: style.foreground || getSemanticColor(theme, 'panelContent'),
        borderColor: style.borderColor || getSemanticColor(theme, 'panelBorder'),
        padding: `${style.padding}px`,
        ...(style.showBorders && {
            border: '1px solid',
            borderRadius: '4px'
        })
    };
}

function renderMenuItems(
    items: MenuItem[],
    state: MenuState,
    style: MenuStyle,
    theme: ColorTheme,
    handlers: {
        onItemSelect?: (item: MenuItem, event?: any) => void;
        onItemHover?: (item: MenuItem, event?: any) => void;
        onSubmenuOpen?: (item: MenuItem, event?: any) => void;
        onSubmenuClose?: (item: MenuItem, event?: any) => void;
        onKeyPress?: (key: string, event?: any) => void;
    }
): any[] {
    return items
        .filter(item => item.visible !== false)
        .map((item, index) => renderMenuItem(item, index, state, style, theme, handlers));
}

function renderMenuItem(
    item: MenuItem,
    index: number,
    state: MenuState,
    style: MenuStyle,
    theme: ColorTheme,
    handlers: any
): any {
    const isSelected = index === state.selectedIndex;
    const isDisabled = item.enabled === false;
    
    // Determine item state
    let itemState: MenuItemState = 'normal';
    if (isDisabled) itemState = 'disabled';
    else if (isSelected) itemState = 'selected';

    // Build item classes
    const itemClasses = [
        'menu-item',
        `menu-item-${item.itemType.type}`,
        `menu-item-${itemState}`,
        ...(item.cssClasses || [])
    ];

    // Get item style based on state
    const itemStyle = getMenuItemStyle(itemState, style, theme);

    // Handle different item types
    switch (item.itemType.type) {
        case 'separator':
            return {
                tag: 'div',
                id: `${item.id}-separator`,
                classes: ['menu-separator'],
                style: {
                    borderTop: `1px solid ${style.separatorColor}`,
                    margin: '2px 0',
                    height: '1px'
                }
            };

        case 'header':
            return {
                tag: 'div',
                id: item.id,
                classes: ['menu-header', ...itemClasses],
                style: {
                    ...itemStyle,
                    fontWeight: 'bold',
                    padding: `${style.padding}px`
                },
                content: item.label
            };

        case 'toggle':
            const toggleState = state.toggleStates[item.id] ?? item.itemType.state ?? false;
            return {
                tag: 'div',
                id: item.id,
                classes: itemClasses,
                style: itemStyle,
                content: `${style.showIcons ? (toggleState ? '☑' : '☐') : ''} ${item.label}${
                    style.showShortcuts && item.shortcut ? ` (${item.shortcut})` : ''
                }`,
                onClick: () => {
                    if (!isDisabled) {
                        const newState = !toggleState;
                        state.toggleStates[item.id] = newState;
                        handlers.onItemSelect?.(item);
                    }
                }
            };

        case 'radio':
            const isRadioSelected = state.radioSelections[item.itemType.group || ''] === item.id;
            return {
                tag: 'div',
                id: item.id,
                classes: itemClasses,
                style: itemStyle,
                content: `${style.showIcons ? (isRadioSelected ? '●' : '○') : ''} ${item.label}${
                    style.showShortcuts && item.shortcut ? ` (${item.shortcut})` : ''
                }`,
                onClick: () => {
                    if (!isDisabled && item.itemType.group) {
                        state.radioSelections[item.itemType.group] = item.id;
                        handlers.onItemSelect?.(item);
                    }
                }
            };

        case 'submenu':
            return {
                tag: 'div',
                id: item.id,
                classes: itemClasses,
                style: itemStyle,
                content: `${style.showIcons && item.icon ? item.icon + ' ' : ''}${item.label}${
                    style.showShortcuts && item.shortcut ? ` (${item.shortcut})` : ''
                } ${style.showIcons ? '▶' : ''}`,
                onClick: () => {
                    if (!isDisabled) {
                        handleItemSelect(item, state, handlers.onItemSelect, handlers.onSubmenuOpen);
                    }
                },
                onMouseEnter: () => handlers.onItemHover?.(item)
            };

        default: // action
            return {
                tag: 'div',
                id: item.id,
                classes: itemClasses,
                style: itemStyle,
                content: `${style.showIcons && item.icon ? item.icon + ' ' : ''}${item.label}${
                    style.showShortcuts && item.shortcut ? ` (${item.shortcut})` : ''
                }`,
                onClick: () => {
                    if (!isDisabled) {
                        handlers.onItemSelect?.(item);
                    }
                },
                onMouseEnter: () => handlers.onItemHover?.(item),
                title: item.tooltip
            };
    }
}

function getMenuItemStyle(state: MenuItemState, style: MenuStyle, theme: ColorTheme): Record<string, string> {
    const baseStyle = {
        padding: `${style.padding}px`,
        height: `${style.itemHeight}em`,
        cursor: 'pointer',
        userSelect: 'none',
        display: 'flex',
        alignItems: 'center'
    };

    switch (state) {
        case 'hovered':
            return {
                ...baseStyle,
                backgroundColor: style.hoverBackground || getSemanticBackground(theme, 'buttonHover'),
                color: style.hoverForeground || getSemanticColor(theme, 'buttonText')
            };
        case 'selected':
            return {
                ...baseStyle,
                backgroundColor: style.selectedBackground || getSemanticBackground(theme, 'buttonBackground'),
                color: style.selectedForeground || getSemanticColor(theme, 'buttonText')
            };
        case 'disabled':
            return {
                ...baseStyle,
                color: style.disabledForeground || getSemanticColor(theme, 'panelContent'),
                cursor: 'not-allowed',
                opacity: '0.6'
            };
        default:
            return baseStyle;
    }
}

function handleItemSelect(
    item: MenuItem,
    state: MenuState,
    onItemSelect?: (item: MenuItem, event?: any) => void,
    onSubmenuOpen?: (item: MenuItem, event?: any) => void
) {
    switch (item.itemType.type) {
        case 'submenu':
            if (item.itemType.items) {
                state.navigationStack.push(state.currentItems);
                state.currentItems = item.itemType.items;
                state.selectedIndex = 0;
                onSubmenuOpen?.(item);
            }
            break;
        case 'toggle':
            const currentState = state.toggleStates[item.id] ?? item.itemType.state ?? false;
            state.toggleStates[item.id] = !currentState;
            onItemSelect?.(item);
            break;
        case 'radio':
            if (item.itemType.group) {
                state.radioSelections[item.itemType.group] = item.id;
            }
            onItemSelect?.(item);
            break;
        default:
            onItemSelect?.(item);
    }
}

// Convenience functions
export function contextMenu(config: Omit<MenuConfig, 'orientation'> & { items: MenuItem[] }): any {
    return menu({ ...config, orientation: 'vertical' });
}

export function menuBar(config: Omit<MenuConfig, 'orientation'> & { items: MenuItem[] }): any {
    return menu({ ...config, orientation: 'horizontal' });
}

export function dropdownMenu(config: Omit<MenuConfig, 'orientation'> & { items: MenuItem[] }): any {
    return menu({ 
        ...config, 
        orientation: 'vertical',
        style: {
            ...config.style,
            showBorders: true,
            padding: 1
        }
    });
}

// Helper functions for creating menu items
export function menuItem(config: {
    id: string;
    label: string;
    action?: string;
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
    tooltip?: string;
}): MenuItem {
    return {
        id: config.id,
        label: config.label,
        itemType: { type: 'action', action: config.action },
        icon: config.icon,
        shortcut: config.shortcut,
        enabled: config.enabled,
        tooltip: config.tooltip
    };
}

export function submenuItem(config: {
    id: string;
    label: string;
    items: MenuItem[];
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
}): MenuItem {
    return {
        id: config.id,
        label: config.label,
        itemType: { type: 'submenu', items: config.items },
        icon: config.icon,
        shortcut: config.shortcut,
        enabled: config.enabled
    };
}

export function separatorItem(id: string = 'separator'): MenuItem {
    return {
        id,
        label: '',
        itemType: { type: 'separator' }
    };
}

export function headerItem(config: { id: string; label: string }): MenuItem {
    return {
        id: config.id,
        label: config.label,
        itemType: { type: 'header' }
    };
}

export function toggleItem(config: {
    id: string;
    label: string;
    state?: boolean;
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
}): MenuItem {
    return {
        id: config.id,
        label: config.label,
        itemType: { type: 'toggle', state: config.state },
        icon: config.icon,
        shortcut: config.shortcut,
        enabled: config.enabled
    };
}

export function radioItem(config: {
    id: string;
    label: string;
    group: string;
    selected?: boolean;
    icon?: string;
    shortcut?: string;
    enabled?: boolean;
}): MenuItem {
    return {
        id: config.id,
        label: config.label,
        itemType: { type: 'radio', group: config.group, selected: config.selected },
        icon: config.icon,
        shortcut: config.shortcut,
        enabled: config.enabled
    };
}