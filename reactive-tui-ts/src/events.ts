/**
 * Event handling system for TUI applications
 */

export interface TuiEvent {
  type: string
  target?: string
  data: any
  timestamp: number
}

export interface KeyEvent extends TuiEvent {
  type: 'key'
  data: {
    key: string
    ctrl: boolean
    shift: boolean
    alt: boolean
  }
}

export interface MouseEvent extends TuiEvent {
  type: 'mouse'
  data: {
    button: 'left' | 'right' | 'middle'
    x: number
    y: number
    action: 'press' | 'release' | 'move'
  }
}

export interface ResizeEvent extends TuiEvent {
  type: 'resize'
  data: {
    width: number
    height: number
  }
}

export type EventHandler = (event: TuiEvent) => boolean | void

export class EventEmitter {
  private listeners: Map<string, EventHandler[]> = new Map()
  
  on(eventType: string, handler: EventHandler): void {
    if (!this.listeners.has(eventType)) {
      this.listeners.set(eventType, [])
    }
    this.listeners.get(eventType)!.push(handler)
  }
  
  off(eventType: string, handler: EventHandler): void {
    const handlers = this.listeners.get(eventType)
    if (handlers) {
      const index = handlers.indexOf(handler)
      if (index > -1) {
        handlers.splice(index, 1)
      }
    }
  }
  
  emit(event: TuiEvent): boolean {
    const handlers = this.listeners.get(event.type)
    if (!handlers || handlers.length === 0) {
      return false
    }
    
    for (const handler of handlers) {
      try {
        const result = handler(event)
        if (result === true) {
          // Handler consumed the event, stop propagation
          return true
        }
      } catch (error) {
        console.error(`Error in event handler for ${event.type}:`, error)
      }
    }
    
    return false
  }
  
  once(eventType: string, handler: EventHandler): void {
    const onceHandler: EventHandler = (event) => {
      handler(event)
      this.off(eventType, onceHandler)
      return false
    }
    this.on(eventType, onceHandler)
  }
}

// Global event emitter instance
export const globalEvents = new EventEmitter()

// Convenience functions
export function onKey(handler: (event: KeyEvent) => boolean | void): void {
  globalEvents.on('key', handler as EventHandler)
}

export function onMouse(handler: (event: MouseEvent) => boolean | void): void {
  globalEvents.on('mouse', handler as EventHandler)
}

export function onResize(handler: (event: ResizeEvent) => boolean | void): void {
  globalEvents.on('resize', handler as EventHandler)
}