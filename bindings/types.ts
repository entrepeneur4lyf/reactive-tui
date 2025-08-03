// Auto-generated TypeScript types from Rust structs
// These are internal development types, not for public API

export type Element = { tag: string, classes: Array<string>, attributes: { [key in string]?: string }, content: string | null, children: Array<Element>, id: string | null, focusable: boolean, focused: boolean, disabled: boolean, tab_index: number | null, key_bindings: Array<ElementKeyBinding>, modal: boolean, };

export type ElementKeyBinding = { key: KeyCombination, action: ElementAction, };

export type KeyCombination = { code: string, modifiers: number, };

export type ElementAction = "Activate" | "Focus" | "Toggle" | { "Custom": string };
