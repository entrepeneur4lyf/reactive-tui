# Mouse Widget (TypeScript)

Advanced mouse interaction widget with gesture recognition, cross-platform support, and comprehensive event handling.

## Overview

The TypeScript Mouse widget provides sophisticated mouse interaction capabilities for terminal applications, with seamless integration between browser and Node.js environments. It supports various mouse events, gesture recognition, hover states, and multi-button interactions with proper cross-platform event handling.

## Features

- **Cross-Platform Support**: Works in both Node.js and browser environments
- **Click Detection**: Single, double, and triple-click detection with configurable timing
- **Hover States**: Enter, leave, and move tracking with hover callbacks
- **Drag & Drop**: Full drag and drop support with drag start/end detection
- **Gesture Recognition**: Swipe detection and custom gesture patterns
- **Multi-Button Support**: Left, right, middle button support with separate handlers
- **Mouse Wheel**: Scroll up/down detection with momentum tracking
- **Coordinate Mapping**: Precise pixel coordinate mapping within widget bounds
- **Event Cleanup**: Proper event listener management and cleanup

## Basic Usage

### Simple Click Handler

```typescript
import { mouse } from 'tui-bun/widgets';

const clickArea = mouse('click-area', (builder) =>
  builder
    .onClick('handle_click')
    .cursorStyle('pointer')
);
```

### Factory Functions

The Mouse widget provides several convenience factory functions:

```typescript
import { 
  clickArea, 
  dragDropArea, 
  draggable, 
  droppable, 
  dropTarget,
  hoverArea, 
  gestureArea, 
  interactiveButton 
} from 'tui-bun/widgets';

// Simple click area
const button = clickArea('btn', 'button_clicked');

// Drag and drop zone (both start and end)
const dropZone = dragDropArea('drop-zone', 'drag_start', 'drag_end');

// Draggable element
const draggableItem = draggable('item-1', 'start_drag');

// Droppable area
const dropArea = droppable('drop-area', 'handle_drop');

// Drop target with hover feedback
const target = dropTarget('target-zone', 'item_dropped');

// Hover detection area
const tooltip = hoverArea('tooltip-trigger', 'show_tooltip', 'hide_tooltip');

// Gesture recognition area
const swipeArea = gestureArea('swipe-zone', 'handle_gesture');

// Interactive button with hover
const smartButton = interactiveButton('smart-btn', 'click_action', 'hover_feedback');
```

## Configuration

### MouseConfig Interface

```typescript
interface MouseConfig extends BaseWidgetConfig {
  type: 'mouse';
  cursorStyle?: CursorStyle;
  enableHover?: boolean;
  enableDragDrop?: boolean;
  enableGestures?: boolean;
  doubleClickThreshold?: number;
  tripleClickThreshold?: number;
  dragThreshold?: number;
  gestureThreshold?: number;
  onClick?: string;
  onDoubleClick?: string;
  onTripleClick?: string;
  onRightClick?: string;
  onMiddleClick?: string;
  onHoverEnter?: string;
  onHoverLeave?: string;
  onHoverMove?: string;
  onDragStart?: string;
  onDragMove?: string;
  onDragEnd?: string;
  onScroll?: string;
  onGesture?: string;
}
```

### Mouse States

```typescript
type MouseInteractionState = 'normal' | 'hover' | 'pressed' | 'dragging' | 'released';
```

### Cursor Styles

```typescript
type CursorStyle = 'default' | 'pointer' | 'text' | 'crosshair' | 'move' | 'not-allowed' | 'grab' | 'grabbing';
```

### Mouse Position

```typescript
interface MousePosition {
  x: number;              // Absolute screen coordinate
  y: number;              // Absolute screen coordinate
  relativeX: number;      // Relative to widget bounds
  relativeY: number;      // Relative to widget bounds
}
```

### Mouse Gestures

```typescript
interface MouseGesture {
  type: 'click' | 'double-click' | 'triple-click' | 'drag' | 'swipe' | 'scroll';
  button?: MouseButtonType;
  count?: number;
  start?: { x: number; y: number };
  end?: { x: number; y: number };
  direction?: SwipeDirection | ScrollDirection;
  distance?: number;
  amount?: number;
}
```

## Advanced Usage

### Multi-Click Detection

```typescript
const multiClickArea = mouse('multi-click', (builder) =>
  builder
    .onClick('single_click')
    .onDoubleClick('double_click')
    .onTripleClick('triple_click')
    .doubleClickThreshold(300)
    .class('multi-click-zone')
);
```

### Drag and Drop with Custom Thresholds

```typescript
const dragArea = mouse('drag-zone', (builder) =>
  builder
    .enableDragDrop(true)
    .dragThreshold(5)
    .onDragStart('begin_drag')
    .onDragEnd('complete_drag')
    .cursorStyle('grab')
    .class('draggable-area')
);
```

### Comprehensive Event Handling

```typescript
const fullFeaturedArea = mouse('full-area', (builder) =>
  builder
    .onClick('primary_click')
    .onRightClick('context_menu')
    .onDoubleClick('edit_mode')
    .enableHover(true)
    .onHoverEnter('show_preview')
    .onHoverLeave('hide_preview')
    .enableDragDrop(true)
    .onDragStart('start_move')
    .onDragEnd('finish_move')
    .onScroll('handle_scroll')
    .cursorStyle('pointer')
    .class('interactive-panel')
);
```

### Gesture Recognition

```typescript
const gestureArea = mouse('gesture-area', (builder) =>
  builder
    .enableGestures(true)
    .gestureThreshold(10)
    .onGesture('process_gesture')
    .class('gesture-zone')
);
```

## Cross-Platform Support

### Browser Environment

```typescript
// Automatic browser event handling
const browserArea = mouse('browser-area', (builder) =>
  builder
    .onClick('web_click')
    .onScroll('web_scroll')
    .enableHover(true)
);

// Browser-specific features are automatically available:
// - Standard MouseEvent handling
// - Wheel events for scrolling
// - Context menu prevention
// - Drag event integration
```

### Node.js Environment

```typescript
// Node.js terminal integration
const terminalArea = mouse('terminal-area', (builder) =>
  builder
    .onClick('terminal_click')
    .enableHover(true)
    .class('terminal-interactive')
);

// Node.js features:
// - Terminal mouse sequence handling
// - Stdin event integration
// - Cross-platform terminal support
```

### Environment Detection

```typescript
// The widget automatically detects the environment:
if (typeof window !== 'undefined') {
  // Browser-specific initialization
  console.log('Running in browser');
} else if (typeof process !== 'undefined') {
  // Node.js-specific initialization
  console.log('Running in Node.js');
}
```

## Widget Class Usage

### Direct Widget Creation

```typescript
import { MouseWidget, MouseConfig } from 'tui-bun/widgets';

const config: MouseConfig = {
  id: 'my-mouse',
  type: 'mouse',
  cursorStyle: 'pointer',
  enableHover: true,
  enableDragDrop: false,
  onClick: 'handle_click'
};

const widget = new MouseWidget(config);
```

### Widget Methods

```typescript
// Get current mouse state
const state = widget.getState();
console.log(`Current state: ${state}`);

// Get last mouse position
const position = widget.getLastPosition();
if (position) {
  console.log(`Mouse at: (${position.x}, ${position.y})`);
}

// Check hover status
if (widget.isHovering()) {
  const duration = widget.getHoverDuration();
  console.log(`Hovering for: ${duration}ms`);
}

// Set widget bounds
widget.setBounds({ x: 0, y: 0, width: 200, height: 100 });

// Get widget bounds
const bounds = widget.getBounds();
```

## Event Handling

### Browser Events

```typescript
// Browser mouse events are handled automatically
const browserWidget = mouse('browser', (builder) =>
  builder
    .onClick('browser_click')
    .onScroll('browser_scroll')
);

// Events triggered:
// - mousedown, mouseup
// - mousemove, mouseenter, mouseleave
// - wheel
// - contextmenu
// - dragstart, dragend
```

### Custom Event Processing

```typescript
// Access the built widget for custom event handling
const widget = mouse('custom', (builder) =>
  builder.onClick('custom_click')
).build();

// Handle custom mouse events
widget.handleMouseEvent(new MouseEvent('click', {
  clientX: 100,
  clientY: 50,
  button: 0
}));
```

### Event Callbacks

```typescript
// Event callbacks receive structured data
const callbackWidget = mouse('callback', (builder) =>
  builder
    .onClick('click_handler')          // { button, position, count }
    .onHoverEnter('hover_handler')     // { position, relativePosition }
    .onDragStart('drag_handler')       // { startPosition }
    .onScroll('scroll_handler')        // { direction, amount }
);
```

## State Management

### State Transitions

```typescript
// State flow: normal → hover → pressed → dragging → released → normal/hover
const widget = new MouseWidget(config);

// Monitor state changes
widget.onStateChange = (newState: MouseInteractionState) => {
  console.log(`State changed to: ${newState}`);
};
```

### State Queries

```typescript
const widget = mouse('state-demo', (builder) => builder.onClick('click'));

// Check current state
switch (widget.getState()) {
  case 'normal':
    console.log('Widget is idle');
    break;
  case 'hover':
    console.log('Mouse is hovering');
    break;
  case 'pressed':
    console.log('Mouse button is pressed');
    break;
  case 'dragging':
    console.log('Drag operation in progress');
    break;
  case 'released':
    console.log('Mouse button just released');
    break;
}
```

## Coordinate System

### Position Calculation

```typescript
// Position objects contain both absolute and relative coordinates
interface MousePosition {
  x: number;         // Absolute screen coordinate
  y: number;         // Absolute screen coordinate  
  relativeX: number; // Relative to widget bounds (0-based)
  relativeY: number; // Relative to widget bounds (0-based)
}
```

### Bounds Management

```typescript
const widget = mouse('bounds-demo', (builder) => builder.onClick('click'));

// Set bounds (typically done by layout system)
widget.setBounds({
  x: 50, y: 30,      // Widget position
  width: 200,        // Widget width
  height: 100        // Widget height
});

// Events outside bounds are automatically ignored
// Relative coordinates are calculated automatically
```

## Performance Optimization

### Event Throttling

```typescript
// Built-in optimizations:
// - Click detection uses efficient timestamp comparison
// - Drag operations use distance thresholds
// - Hover events are managed with minimal overhead
// - Gesture tracking optimizes path storage

const optimizedArea = mouse('optimized', (builder) =>
  builder
    .dragThreshold(5)      // Avoid micro-movements
    .gestureThreshold(10)  // Require significant gesture
    .doubleClickThreshold(300) // Reasonable timing window
);
```

### Memory Management

```typescript
const widget = mouse('cleanup-demo', (builder) => builder.onClick('click'));

// Proper cleanup
widget.destroy(); // Removes event listeners, clears state

// Automatic cleanup:
// - Event listeners are removed on destroy
// - Click tracking is reset after timeout
// - Drag state is cleaned after operations
// - Gesture paths are cleared after detection
```

## Error Handling

### Event Processing Errors

```typescript
try {
  const widget = mouse('error-demo', (builder) =>
    builder
      .onClick('safe_handler')
      .onError('error_handler')
  );
  
  // Event processing
  await widget.handleMouseEvent(event);
} catch (error) {
  console.error('Mouse event error:', error);
}
```

### Bounds Validation

```typescript
// Widget automatically validates coordinates
// Out-of-bounds events are safely ignored
const widget = mouse('bounds-safe', (builder) => builder.onClick('click'));

// Safe even without bounds set
widget.setBounds(null); // Events will be ignored until bounds are set
```

## Integration Examples

### React Integration

```typescript
import React, { useEffect, useRef } from 'react';
import { mouse } from 'tui-bun/widgets';

interface MouseAreaProps {
  onMouseClick: (data: any) => void;
  onMouseHover: (data: any) => void;
}

const MouseArea: React.FC<MouseAreaProps> = ({ onMouseClick, onMouseHover }) => {
  const containerRef = useRef<HTMLDivElement>(null);
  
  useEffect(() => {
    const mouseWidget = mouse('react-mouse', (builder) =>
      builder
        .onClick('react_click')
        .onHoverEnter('react_hover')
        .cursorStyle('pointer')
    );
    
    // Setup event listeners
    const handleClick = (event: CustomEvent) => onMouseClick(event.detail);
    const handleHover = (event: CustomEvent) => onMouseHover(event.detail);
    
    window.addEventListener('mouse:react_click', handleClick);
    window.addEventListener('mouse:react_hover', handleHover);
    
    return () => {
      window.removeEventListener('mouse:react_click', handleClick);
      window.removeEventListener('mouse:react_hover', handleHover);
      mouseWidget.destroy();
    };
  }, [onMouseClick, onMouseHover]);
  
  return (
    <div 
      ref={containerRef}
      className="mouse-interaction-area"
      style={{ width: '200px', height: '100px', border: '1px solid #ccc' }}
    />
  );
};
```

### Vue Integration

```vue
<template>
  <div 
    ref="mouseArea"
    class="mouse-area"
    @mousedown="handleMouseDown"
    @mouseup="handleMouseUp"
    @mousemove="handleMouseMove"
  >
    <slot />
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue';
import { mouse } from 'tui-bun/widgets';

interface Props {
  onInteraction?: (type: string, data: any) => void;
}

const props = defineProps<Props>();
const mouseArea = ref<HTMLElement>();
let mouseWidget: any;

onMounted(() => {
  mouseWidget = mouse('vue-mouse', (builder) =>
    builder
      .onClick('vue_click')
      .onHoverEnter('vue_hover')
      .enableDragDrop(true)
      .onDragStart('vue_drag')
  );
  
  // Set bounds based on element
  if (mouseArea.value) {
    const rect = mouseArea.value.getBoundingClientRect();
    mouseWidget.setBounds({
      x: rect.left,
      y: rect.top,
      width: rect.width,
      height: rect.height
    });
  }
});

onUnmounted(() => {
  if (mouseWidget) {
    mouseWidget.destroy();
  }
});

const handleMouseDown = (event: MouseEvent) => {
  mouseWidget?.handleMouseEvent(event);
  props.onInteraction?.('mousedown', { x: event.clientX, y: event.clientY });
};

const handleMouseUp = (event: MouseEvent) => {
  mouseWidget?.handleMouseEvent(event);
  props.onInteraction?.('mouseup', { x: event.clientX, y: event.clientY });
};

const handleMouseMove = (event: MouseEvent) => {
  mouseWidget?.handleMouseEvent(event);
};
</script>

<style scoped>
.mouse-area {
  position: relative;
  cursor: pointer;
}
</style>
```

### Node.js Terminal Integration

```typescript
// Terminal mouse handling in Node.js
import { mouse } from 'tui-bun/widgets';
import { stdin, stdout } from 'process';

const terminalMouse = mouse('terminal', (builder) =>
  builder
    .onClick('terminal_click')
    .onScroll('terminal_scroll')
    .enableHover(true)
);

// Enable mouse reporting in terminal
stdout.write('\x1b[?1000h'); // Enable mouse tracking
stdout.write('\x1b[?1002h'); // Enable mouse movement tracking
stdout.write('\x1b[?1015h'); // Enable UTF-8 mouse mode
stdout.write('\x1b[?1006h'); // Enable SGR mouse mode

// Handle stdin mouse sequences
stdin.setRawMode(true);
stdin.on('data', (data) => {
  const sequence = data.toString();
  
  // Parse mouse sequences (simplified)
  if (sequence.includes('\x1b[<')) {
    // SGR mouse sequence
    const match = sequence.match(/\x1b\[<(\d+);(\d+);(\d+)([mM])/);
    if (match) {
      const [, button, x, y, type] = match;
      const isPress = type === 'M';
      
      // Convert to MouseEvent-like object
      const event = {
        button: parseInt(button),
        clientX: parseInt(x),
        clientY: parseInt(y),
        type: isPress ? 'mousedown' : 'mouseup'
      };
      
      terminalMouse.handleMouseEvent(event as MouseEvent);
    }
  }
});

// Cleanup on exit
process.on('exit', () => {
  stdout.write('\x1b[?1000l'); // Disable mouse tracking
  terminalMouse.destroy();
});
```

### Framework-Agnostic Usage

```typescript
// Pure TypeScript usage without frameworks
import { mouse, clickArea } from 'tui-bun/widgets';

class InteractivePanel {
  private mouseWidget: any;
  private element: HTMLElement;
  
  constructor(containerId: string) {
    this.element = document.getElementById(containerId)!;
    this.initializeMouse();
  }
  
  private initializeMouse() {
    this.mouseWidget = mouse('panel-mouse', (builder) =>
      builder
        .onClick('panel_click')
        .onRightClick('panel_context')
        .enableHover(true)
        .onHoverEnter('panel_hover_enter')
        .onHoverLeave('panel_hover_leave')
        .enableDragDrop(true)
        .onDragStart('panel_drag_start')
        .onDragEnd('panel_drag_end')
        .cursorStyle('pointer')
    );
    
    // Set bounds
    const rect = this.element.getBoundingClientRect();
    this.mouseWidget.setBounds({
      x: rect.left,
      y: rect.top,
      width: rect.width,
      height: rect.height
    });
    
    // Attach native event handlers
    this.element.addEventListener('mousedown', this.handleMouseDown.bind(this));
    this.element.addEventListener('mouseup', this.handleMouseUp.bind(this));
    this.element.addEventListener('mousemove', this.handleMouseMove.bind(this));
    this.element.addEventListener('contextmenu', this.handleContextMenu.bind(this));
  }
  
  private handleMouseDown(event: MouseEvent) {
    this.mouseWidget.handleMouseEvent(event);
  }
  
  private handleMouseUp(event: MouseEvent) {
    this.mouseWidget.handleMouseEvent(event);
  }
  
  private handleMouseMove(event: MouseEvent) {
    this.mouseWidget.handleMouseEvent(event);
  }
  
  private handleContextMenu(event: MouseEvent) {
    event.preventDefault();
    this.mouseWidget.handleMouseEvent(event);
  }
  
  public destroy() {
    this.mouseWidget.destroy();
    // Remove event listeners...
  }
}

// Usage
const panel = new InteractivePanel('my-panel');
```

## Testing

### Unit Tests

```typescript
import { MouseWidget, MouseConfig } from 'tui-bun/widgets';

describe('MouseWidget', () => {
  let widget: MouseWidget;
  let config: MouseConfig;
  
  beforeEach(() => {
    config = {
      id: 'test-mouse',
      type: 'mouse',
      enableHover: true,
      onClick: 'test_click'
    };
    widget = new MouseWidget(config);
  });
  
  afterEach(() => {
    widget.destroy();
  });
  
  test('should initialize with correct state', () => {
    expect(widget.getState()).toBe('normal');
    expect(widget.isHovering()).toBe(false);
  });
  
  test('should handle bounds correctly', () => {
    const bounds = { x: 0, y: 0, width: 100, height: 50 };
    widget.setBounds(bounds);
    
    expect(widget.getBounds()).toEqual(bounds);
  });
  
  test('should track hover state', () => {
    widget.setBounds({ x: 0, y: 0, width: 100, height: 50 });
    
    const enterEvent = new MouseEvent('mouseenter', {
      clientX: 25, clientY: 25
    });
    
    widget.handleMouseEvent(enterEvent);
    expect(widget.isHovering()).toBe(true);
    expect(widget.getState()).toBe('hover');
  });
  
  test('should handle click detection', () => {
    widget.setBounds({ x: 0, y: 0, width: 100, height: 50 });
    
    const clickEvent = new MouseEvent('mousedown', {
      clientX: 25, clientY: 25, button: 0
    });
    
    widget.handleMouseEvent(clickEvent);
    expect(widget.getState()).toBe('pressed');
  });
});
```

### Integration Tests

```typescript
import { mouse, clickArea } from 'tui-bun/widgets';

describe('Mouse Factory Functions', () => {
  test('clickArea should create proper configuration', () => {
    const area = clickArea('test-click', 'handler');
    
    expect(area.id).toBe('test-click');
    expect(area.attributes['data-cursor']).toBe('pointer');
  });
  
  test('mouse builder should configure correctly', () => {
    const area = mouse('test', (builder) =>
      builder
        .enableHover(true)
        .onClick('click_handler')
        .cursorStyle('crosshair')
    );
    
    expect(area.attributes['data-enable-hover']).toBe('true');
    expect(area.attributes['data-cursor']).toBe('crosshair');
  });
});
```

## Related

- [Button Widget](button.md) - For simple click interactions
- [Base Widget](base-widget.md) - Base widget functionality
- [Widget Factory](../core/widget-factory.md) - Widget creation patterns
- [Layout System](../../../layout.md) - For positioning and bounds
- [TypeScript Integration](../overview.md) - TypeScript-specific features
- [Cross-Platform Support](../overview.md) - Platform compatibility