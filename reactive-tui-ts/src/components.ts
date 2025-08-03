/**
 * Component builders for the TUI framework
 */

import type { Element } from './generated-types'
import type { ElementBuilder, Component } from './types'

// Re-export key types
export type { Element, Component } from './types'

// Import and re-export all widgets
// Handle naming conflicts with explicit exports for conflicting types
export * from './widgets/accordion'
export * from './widgets/autocomplete'
export * from './widgets/bar'
export * from './widgets/button'
export * from './widgets/checkbox'
export * from './widgets/datatable'
export * from './widgets/form_validation'
export * from './widgets/grid'
export * from './widgets/hot_reload'
export * from './widgets/input'
export * from './widgets/menu'
export * from './widgets/modal'
export * from './widgets/panel'
export * from './widgets/progress'
export * from './widgets/radio'
export * from './widgets/rich_text'
export * from './widgets/select'
export * from './widgets/slider'
export * from './widgets/spinner'
export * from './widgets/switch'
export * from './widgets/tabs'
export * from './widgets/toast'
export * from './widgets/tree'

export { AnimationState as AnimationWidgetState } from './widgets/animation'
export { SelectionMode as AutocompleteSelectionMode } from './widgets/autocomplete'
export { SelectionMode as ScrollableListSelectionMode } from './widgets/scrollable_list'

// Widget bridge functions - convert widgets to Elements for responsive layout
// These match the Rust tui-core architecture

/** Create a button widget as an Element */
export function buttonWidget(id: string, text: string): Element {
  const { button } = require('./widgets/button')
  const buttonInstance = button({ id, text })
  return convertWidgetToElement(buttonInstance)
}

/** Create a primary button widget as an Element */
export function primaryButton(id: string, text: string): Element {
  const { button } = require('./widgets/button')
  const buttonInstance = button({ id, text, color: 'primary', variant: 'filled' })
  return convertWidgetToElement(buttonInstance)
}

/** Create a secondary button widget as an Element */
export function secondaryButton(id: string, text: string): Element {
  const { button } = require('./widgets/button')
  const buttonInstance = button({ id, text, color: 'secondary', variant: 'outlined' })
  return convertWidgetToElement(buttonInstance)
}

/** Create a danger button widget as an Element */
export function dangerButton(id: string, text: string): Element {
  const { button } = require('./widgets/button')
  const buttonInstance = button({ id, text, color: 'error', variant: 'filled' })
  return convertWidgetToElement(buttonInstance)
}

// Helper function to convert widget objects to Elements
function convertWidgetToElement(widget: any): Element {
  return {
    tag: widget.tag || 'div',
    id: widget.id,
    classes: widget.classes || [],
    attributes: widget.attributes || {},
    content: widget.content || '',
    children: widget.children || [],
    key_bindings: [],
    focusable: !widget.attributes?.disabled,
    focused: false,
    modal: false,
    tab_index: widget.attributes?.tabindex ? parseInt(widget.attributes.tabindex) : undefined
  }
}

export class ElementBuilderImpl implements ElementBuilder {
  private element: Element

  constructor(tag: string) {
    this.element = {
      tag,
      classes: [],
      attributes: {},
      content: null,
      children: [],
      id: null,
      focusable: false,
      focused: false,
      tab_index: null,
      key_bindings: [],
      modal: false
    }
  }

  class(className: string): ElementBuilder {
    this.element.classes.push(className)
    return this
  }

  classes(classes: string[]): ElementBuilder {
    this.element.classes.push(...classes)
    return this
  }

  id(id: string): ElementBuilder {
    this.element.id = id
    return this
  }

  attr(key: string, value: string): ElementBuilder {
    this.element.attributes[key] = value
    return this
  }

  content(content: string): ElementBuilder {
    this.element.content = content
    return this
  }

  child(child: Element | ElementBuilder): ElementBuilder {
    const element = 'build' in child ? child.build() : child
    this.element.children.push(element)
    return this
  }

  children(children: (Element | ElementBuilder)[]): ElementBuilder {
    for (const child of children) {
      this.child(child)
    }
    return this
  }

  focusable(focusable: boolean): ElementBuilder {
    this.element.focusable = focusable
    return this
  }

  tab_index(index: number): ElementBuilder {
    this.element.tab_index = index
    this.element.focusable = true // Auto-enable focusable for tab index
    return this
  }

  bind_key(key: import('./generated-types').KeyCombination, action: import('./generated-types').ElementAction): ElementBuilder {
    this.element.key_bindings.push({ key, action })
    return this
  }

  bind_char(c: string, action: import('./generated-types').ElementAction): ElementBuilder {
    return this.bind_key({ code: c, modifiers: 0 }, action)
  }

  bind_enter(): ElementBuilder {
    return this.bind_key({ code: 'Enter', modifiers: 0 }, 'Activate')
  }

  bind_space(): ElementBuilder {
    return this.bind_key({ code: ' ', modifiers: 0 }, 'Activate')
  }

  modal(is_modal: boolean): ElementBuilder {
    this.element.modal = is_modal
    if (is_modal) {
      // Add default ESC binding to dismiss modal
      this.element.key_bindings.push({
        key: { code: 'Escape', modifiers: 0 },
        action: { Custom: 'dismiss' }
      })
    }
    return this
  }

  build(): Element {
    return { ...this.element }
  }
}

// Component builder functions
export function div(props?: { class?: string; classes?: string[]; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('div')
  if (props?.class) builder.class(props.class)
  if (props?.classes) builder.classes(props.classes)
  if (props?.id) builder.id(props.id)
  return builder
}

export function text(content: string, props?: { class?: string; classes?: string[]; id?: string }): ElementBuilder
export function text(props: { content: string; class?: string; classes?: string[]; id?: string }): ElementBuilder
export function text(contentOrProps: string | { content: string; class?: string; classes?: string[]; id?: string }, props?: { class?: string; classes?: string[]; id?: string }): ElementBuilder {
  let content: string
  let actualProps: { class?: string; classes?: string[]; id?: string } | undefined

  if (typeof contentOrProps === 'string') {
    content = contentOrProps
    actualProps = props
  } else {
    content = contentOrProps.content
    actualProps = contentOrProps
  }

  const builder = new ElementBuilderImpl('text').content(content)
  if (actualProps?.class) builder.class(actualProps.class)
  if (actualProps?.classes) builder.classes(actualProps.classes)
  if (actualProps?.id) builder.id(actualProps.id)
  return builder
}

export function span(content?: string, props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('span')
  if (content) builder.content(content)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function section(props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('section')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function header(props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('header')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function footer(props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('footer')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function main(props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('main')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

// Modal dialog convenience function matching Rust backend
export function modal_dialog(title: string, props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('modal')
    .class('modal')
    .attr('title', title)
    .modal(true)
    .focusable(true)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

// Higher-order components
export function container(children: (Element | ElementBuilder)[], props?: { class?: string; id?: string }): ElementBuilder {
  return div(props).children(children)
}

export function flexRow(children: (Element | ElementBuilder)[], props?: { class?: string; id?: string }): ElementBuilder {
  const className = props?.class ? `flex-row ${props.class}` : 'flex-row'
  return div({ ...props, class: className }).children(children)
}

export function flexColumn(children: (Element | ElementBuilder)[], props?: { class?: string; id?: string }): ElementBuilder {
  const className = props?.class ? `flex-column ${props.class}` : 'flex-column'
  return div({ ...props, class: className }).children(children)
}

// CLI/TUI-specific components

export function line(props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('br')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function hr(props?: { class?: string; id?: string; char?: string; width?: number }): ElementBuilder {
  const builder = new ElementBuilderImpl('hr')
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  if (props?.char) builder.attr('char', props.char)
  if (props?.width !== undefined) builder.attr('width', props.width.toString())
  return builder
}

export function separator(props?: { class?: string; id?: string; char?: string; width?: number }): ElementBuilder {
  return hr(props)
}

export function spacer(height: number = 1, props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('spacer')
  builder.attr('height', height.toString())
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function code(content: string, props?: { class?: string; id?: string; language?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('code').content(content)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  if (props?.language) builder.attr('language', props.language)
  return builder
}

export function pre(content: string, props?: { class?: string; id?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('pre').content(content)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function list(items: string[], props?: { class?: string; id?: string; type?: 'bullet' | 'numbered'; marker?: string }): ElementBuilder {
  const builder = new ElementBuilderImpl('list')
  builder.attr('items', JSON.stringify(items))
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  if (props?.type) builder.attr('type', props.type)
  if (props?.marker) builder.attr('marker', props.marker)
  return builder
}

// Layout and styling helpers

export function border(child: Element | ElementBuilder, props?: { class?: string; id?: string; style?: 'single' | 'double' | 'rounded' | 'thick' }): ElementBuilder {
  const element = 'build' in child ? child.build() : child
  const builder = new ElementBuilderImpl('border').child(element)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  if (props?.style) builder.attr('style', props.style)
  return builder
}

export function padding(child: Element | ElementBuilder, amount: number = 1, props?: { class?: string; id?: string }): ElementBuilder {
  const element = 'build' in child ? child.build() : child
  const builder = new ElementBuilderImpl('padding').child(element)
  builder.attr('amount', amount.toString())
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function center(child: Element | ElementBuilder, props?: { class?: string; id?: string }): ElementBuilder {
  const element = 'build' in child ? child.build() : child
  const builder = new ElementBuilderImpl('center').child(element)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function left(child: Element | ElementBuilder, props?: { class?: string; id?: string }): ElementBuilder {
  const element = 'build' in child ? child.build() : child
  const builder = new ElementBuilderImpl('left').child(element)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}

export function right(child: Element | ElementBuilder, props?: { class?: string; id?: string }): ElementBuilder {
  const element = 'build' in child ? child.build() : child
  const builder = new ElementBuilderImpl('right').child(element)
  if (props?.class) builder.class(props.class)
  if (props?.id) builder.id(props.id)
  return builder
}
