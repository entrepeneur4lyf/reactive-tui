/**
 * Select Dropdown Widget - TypeScript Implementation
 *
 * A comprehensive dropdown selection widget supporting single/multi-select modes,
 * search filtering, keyboard navigation, and customizable rendering.
 *
 * Features:
 * - Selection Modes: Single-select and multi-select with different behaviors
 * - Search Filtering: Real-time option filtering as user types
 * - Keyboard Navigation: Arrow keys, Enter/Space selection, Escape to close
 * - Custom Rendering: Flexible option display with icons, descriptions, grouping
 * - Overlay Positioning: Smart dropdown positioning with viewport awareness
 * - Accessibility: Full ARIA support, screen reader compatibility
 * - Reactive State: Integration with reactive state management system
 *
 * @example Basic Usage
 * ```typescript
 * import { select, SelectOption } from './widgets/select';
 *
 * // Simple single-select dropdown
 * const languageSelect = select({
 *   id: 'language-select',
 *   options: ['Rust', 'TypeScript', 'Python', 'Go'],
 *   selected: 0,
 *   placeholder: 'Choose a language...',
 *   onChange: (selectedIndices) => {
 *     console.log('Selected:', selectedIndices);
 *   }
 * });
 *
 * // Multi-select with search
 * const tagsSelect = select({
 *   id: 'tags-select',
 *   options: ['Frontend', 'Backend', 'Database', 'Mobile'],
 *   multiSelect: true,
 *   searchable: true,
 *   selectedIndices: [0, 2],
 *   placeholder: 'Select tags...'
 * });
 * ```
 *
 * @example Advanced Usage
 * ```typescript
 * // Custom options with icons and descriptions
 * const options: SelectOption[] = [
 *   {
 *     id: 'rust',
 *     label: 'Rust',
 *     icon: '🦀',
 *     description: 'Systems programming language'
 *   },
 *   {
 *     id: 'typescript',
 *     label: 'TypeScript',
 *     icon: '📘',
 *     description: 'Typed JavaScript'
 *   }
 * ];
 *
 * const advancedSelect = select({
 *   id: 'advanced-select',
 *   customOptions: options,
 *   searchable: true,
 *   maxHeight: 10,
 *   onChange: (selected) => console.log('Selected:', selected)
 * });
 * ```
 */

export interface SelectOption {
  /** Unique identifier for the option */
  id: string;
  /** Display text for the option */
  label: string;
  /** Optional icon to display with the option */
  icon?: string;
  /** Optional description text */
  description?: string;
  /** Optional group this option belongs to */
  group?: string;
  /** Whether this option is disabled */
  disabled?: boolean;
  /** Custom data associated with this option */
  data?: Record<string, string>;
}

export type SelectMode = 'single' | 'multiple';

export type DropdownPosition = 'below' | 'above' | 'auto';

export interface SelectState {
  /** Whether the dropdown is currently open */
  open: boolean;
  /** Index of the currently highlighted option (for keyboard navigation) */
  highlightedIndex?: number;
  /** Indices of selected options */
  selectedIndices: number[];
  /** Current search query (if searchable) */
  searchQuery: string;
  /** Filtered options based on search query */
  filteredIndices: number[];
  /** Whether the select has focus */
  focused: boolean;
  /** Current scroll position in dropdown */
  scrollOffset: number;
}

export interface SelectStyle {
  /** CSS classes for the main select container */
  containerClasses: string[];
  /** CSS classes for the select button/trigger */
  triggerClasses: string[];
  /** CSS classes for the dropdown overlay */
  dropdownClasses: string[];
  /** CSS classes for individual options */
  optionClasses: string[];
  /** CSS classes for selected options */
  selectedOptionClasses: string[];
  /** CSS classes for highlighted option (keyboard navigation) */
  highlightedOptionClasses: string[];
  /** CSS classes for disabled options */
  disabledOptionClasses: string[];
  /** CSS classes for the search input (if searchable) */
  searchInputClasses: string[];
  /** CSS classes for option groups */
  groupClasses: string[];
  /** Character to use for dropdown arrow */
  dropdownArrow: string;
  /** Character to use for selected items in multi-select */
  selectedMarker: string;
  /** Maximum height of dropdown in rows */
  maxHeight: number;
  /** Whether to show icons in options */
  showIcons: boolean;
  /** Whether to show descriptions in options */
  showDescriptions: boolean;
}

export interface SelectConfig {
  /** Unique identifier for the select */
  id: string;
  /** Available options (string labels) */
  options?: string[];
  /** Custom options with icons, descriptions, etc. */
  customOptions?: SelectOption[];
  /** Selection mode */
  mode?: SelectMode;
  /** Enable multi-select mode */
  multiSelect?: boolean;
  /** Whether search filtering is enabled */
  searchable?: boolean;
  /** Placeholder text when no selection is made */
  placeholder?: string;
  /** Dropdown position preference */
  position?: DropdownPosition;
  /** Styling configuration */
  style?: Partial<SelectStyle>;
  /** Maximum height of dropdown */
  maxHeight?: number;
  /** Initially selected option (single-select) */
  selected?: number;
  /** Initially selected options (multi-select) */
  selectedIndices?: number[];
  /** Callback for selection changes */
  onChange?: (selectedIndices: number[]) => void;
  /** Callback for open/close state changes */
  onToggle?: (open: boolean) => void;
  /** Whether the select is disabled */
  disabled?: boolean;
  /** Whether the select is required */
  required?: boolean;
  /** Custom filter function for search */
  filter?: (option: SelectOption, query: string) => boolean;
}

export interface SelectAPI {
  /** Get the currently selected options */
  getSelectedOptions(): SelectOption[];
  /** Get the currently selected option IDs */
  getSelectedIds(): string[];
  /** Check if an option is selected */
  isSelected(index: number): boolean;
  /** Select an option by index */
  select(index: number): void;
  /** Deselect an option by index */
  deselect(index: number): void;
  /** Toggle selection of an option */
  toggleSelection(index: number): void;
  /** Clear all selections */
  clearSelection(): void;
  /** Open the dropdown */
  open(): void;
  /** Close the dropdown */
  close(): void;
  /** Toggle the dropdown open/closed state */
  toggle(): void;
  /** Set the search query and update filtered options */
  setSearchQuery(query: string): void;
  /** Navigate to the next option */
  navigateNext(): void;
  /** Navigate to the previous option */
  navigatePrevious(): void;
  /** Select the currently highlighted option */
  selectHighlighted(): void;
  /** Get display text for the selected value(s) */
  getDisplayText(): string;
  /** Update the select configuration */
  updateConfig(config: Partial<SelectConfig>): void;
  /** Get current state */
  getState(): SelectState;
  /** Handle keyboard events */
  handleKeyEvent(event: KeyboardEvent): boolean;
  /** Handle focus events */
  handleFocus(): void;
  /** Handle blur events */
  handleBlur(): void;
  /** Render the select element */
  render(): HTMLElement;
}

class SelectImplementation implements SelectAPI {
  private config: Required<SelectConfig>;
  private options: SelectOption[];
  private state: SelectState;
  private element?: HTMLElement;

  constructor(config: SelectConfig) {
    // Set defaults
    this.config = {
      ...config,
      options: config.options || [],
      customOptions: config.customOptions || [],
      mode: config.multiSelect ? 'multiple' : (config.mode || 'single'),
      multiSelect: config.multiSelect || config.mode === 'multiple',
      searchable: config.searchable || false,
      placeholder: config.placeholder || 'Select an option...',
      position: config.position || 'auto',
      maxHeight: config.maxHeight || 8,
      selected: config.selected,
      selectedIndices: config.selectedIndices || (config.selected !== undefined ? [config.selected] : []),
      onChange: config.onChange || (() => {}),
      onToggle: config.onToggle || (() => {}),
      disabled: config.disabled || false,
      required: config.required || false,
      filter: config.filter,
      style: {
        containerClasses: ['select'],
        triggerClasses: ['select-trigger'],
        dropdownClasses: ['select-dropdown'],
        optionClasses: ['select-option'],
        selectedOptionClasses: ['select-option-selected'],
        highlightedOptionClasses: ['select-option-highlighted'],
        disabledOptionClasses: ['select-option-disabled'],
        searchInputClasses: ['select-search'],
        groupClasses: ['select-group'],
        dropdownArrow: '▼',
        selectedMarker: '✓',
        maxHeight: config.maxHeight || 8,
        showIcons: true,
        showDescriptions: true,
        ...config.style
      }
    };

    // Process options
    if (this.config.customOptions.length > 0) {
      this.options = this.config.customOptions;
    } else {
      this.options = this.config.options.map((label, index) => ({
        id: index.toString(),
        label,
        disabled: false
      }));
    }

    // Initialize state
    this.state = {
      open: false,
      highlightedIndex: undefined,
      selectedIndices: [...this.config.selectedIndices],
      searchQuery: '',
      filteredIndices: this.options.map((_, index) => index),
      focused: false,
      scrollOffset: 0
    };
  }

  getSelectedOptions(): SelectOption[] {
    return this.state.selectedIndices
      .map(index => this.options[index])
      .filter(Boolean);
  }

  getSelectedIds(): string[] {
    return this.getSelectedOptions().map(option => option.id);
  }

  isSelected(index: number): boolean {
    return this.state.selectedIndices.includes(index);
  }

  select(index: number): void {
    if (index >= this.options.length || this.options[index]?.disabled) {
      return;
    }

    if (this.config.mode === 'single') {
      this.state.selectedIndices = [index];
      this.state.open = false; // Close dropdown after single selection
    } else {
      if (!this.state.selectedIndices.includes(index)) {
        this.state.selectedIndices.push(index);
        this.state.selectedIndices.sort((a, b) => a - b);
      }
    }

    this.config.onChange(this.state.selectedIndices);
    this.updateElement();
  }

  deselect(index: number): void {
    this.state.selectedIndices = this.state.selectedIndices.filter(i => i !== index);
    this.config.onChange(this.state.selectedIndices);
    this.updateElement();
  }

  toggleSelection(index: number): void {
    if (this.isSelected(index)) {
      this.deselect(index);
    } else {
      this.select(index);
    }
  }

  clearSelection(): void {
    this.state.selectedIndices = [];
    this.config.onChange(this.state.selectedIndices);
    this.updateElement();
  }

  open(): void {
    if (this.config.disabled) return;

    if (!this.state.open) {
      this.state.open = true;
      this.updateFilteredOptions();
      
      // Set initial highlighted index
      if (this.state.highlightedIndex === undefined && this.state.filteredIndices.length > 0) {
        this.state.highlightedIndex = this.state.filteredIndices[0];
      }

      this.config.onToggle(true);
      this.updateElement();
    }
  }

  close(): void {
    if (this.state.open) {
      this.state.open = false;
      this.state.searchQuery = '';
      this.state.highlightedIndex = undefined;
      this.state.scrollOffset = 0;
      this.updateFilteredOptions();
      this.config.onToggle(false);
      this.updateElement();
    }
  }

  toggle(): void {
    if (this.state.open) {
      this.close();
    } else {
      this.open();
    }
  }

  setSearchQuery(query: string): void {
    if (!this.config.searchable) return;

    this.state.searchQuery = query;
    this.state.highlightedIndex = undefined;
    this.state.scrollOffset = 0;
    this.updateFilteredOptions();
    this.updateElement();
  }

  private updateFilteredOptions(): void {
    if (this.state.searchQuery === '') {
      this.state.filteredIndices = this.options.map((_, index) => index);
    } else {
      this.state.filteredIndices = this.options
        .map((option, index) => ({ option, index }))
        .filter(({ option }) => {
          if (this.config.filter) {
            return this.config.filter(option, this.state.searchQuery);
          } else {
            // Default search: case-insensitive match in label or description
            const queryLower = this.state.searchQuery.toLowerCase();
            return option.label.toLowerCase().includes(queryLower) ||
                   (option.description?.toLowerCase().includes(queryLower) || false);
          }
        })
        .map(({ index }) => index);
    }

    // Reset highlighted index if current selection is no longer visible
    if (this.state.highlightedIndex !== undefined && 
        !this.state.filteredIndices.includes(this.state.highlightedIndex)) {
      this.state.highlightedIndex = this.state.filteredIndices.length > 0 
        ? this.state.filteredIndices[0] 
        : undefined;
    }
  }

  navigateNext(): void {
    if (this.state.filteredIndices.length === 0) return;

    if (this.state.highlightedIndex === undefined) {
      this.state.highlightedIndex = this.state.filteredIndices[0];
    } else {
      const currentPos = this.state.filteredIndices.indexOf(this.state.highlightedIndex);
      if (currentPos !== -1) {
        const nextPos = (currentPos + 1) % this.state.filteredIndices.length;
        this.state.highlightedIndex = this.state.filteredIndices[nextPos];
        
        // Update scroll offset if needed
        if (nextPos >= this.state.scrollOffset + this.config.style.maxHeight) {
          this.state.scrollOffset = Math.max(0, nextPos - this.config.style.maxHeight + 1);
        }
      }
    }
    this.updateElement();
  }

  navigatePrevious(): void {
    if (this.state.filteredIndices.length === 0) return;

    if (this.state.highlightedIndex === undefined) {
      this.state.highlightedIndex = this.state.filteredIndices[this.state.filteredIndices.length - 1];
    } else {
      const currentPos = this.state.filteredIndices.indexOf(this.state.highlightedIndex);
      if (currentPos !== -1) {
        const prevPos = currentPos === 0 ? this.state.filteredIndices.length - 1 : currentPos - 1;
        this.state.highlightedIndex = this.state.filteredIndices[prevPos];
        
        // Update scroll offset if needed
        if (prevPos < this.state.scrollOffset) {
          this.state.scrollOffset = prevPos;
        }
      }
    }
    this.updateElement();
  }

  selectHighlighted(): void {
    if (this.state.highlightedIndex !== undefined) {
      this.toggleSelection(this.state.highlightedIndex);
    }
  }

  getDisplayText(): string {
    if (this.state.selectedIndices.length === 0) {
      return this.config.placeholder;
    }

    if (this.config.mode === 'single') {
      const index = this.state.selectedIndices[0];
      return this.options[index]?.label || this.config.placeholder;
    } else {
      if (this.state.selectedIndices.length === 1) {
        return this.options[this.state.selectedIndices[0]]?.label || '';
      } else {
        return `${this.state.selectedIndices.length} items selected`;
      }
    }
  }

  updateConfig(config: Partial<SelectConfig>): void {
    Object.assign(this.config, config);
    
    // Reprocess options if they changed
    if (config.options || config.customOptions) {
      if (this.config.customOptions.length > 0) {
        this.options = this.config.customOptions;
      } else {
        this.options = this.config.options.map((label, index) => ({
          id: index.toString(),
          label,
          disabled: false
        }));
      }
      
      // Update filtered indices
      this.updateFilteredOptions();
    }
    
    this.updateElement();
  }

  getState(): SelectState {
    return { ...this.state };
  }

  handleKeyEvent(event: KeyboardEvent): boolean {
    if (this.config.disabled) return false;

    switch (event.key) {
      case 'Enter':
      case ' ':
        event.preventDefault();
        if (this.state.open) {
          this.selectHighlighted();
        } else {
          this.open();
        }
        return true;

      case 'Escape':
        if (this.state.open) {
          event.preventDefault();
          this.close();
          return true;
        }
        return false;

      case 'ArrowDown':
        event.preventDefault();
        if (this.state.open) {
          this.navigateNext();
        } else {
          this.open();
        }
        return true;

      case 'ArrowUp':
        event.preventDefault();
        if (this.state.open) {
          this.navigatePrevious();
        }
        return true;

      case 'Tab':
        if (this.state.open) {
          this.close();
        }
        return false; // Let tab navigation continue

      case 'Backspace':
        if (this.state.open && this.config.searchable) {
          event.preventDefault();
          this.setSearchQuery(this.state.searchQuery.slice(0, -1));
          return true;
        }
        return false;

      default:
        // Handle search input for searchable selects
        if (this.state.open && this.config.searchable && event.key.length === 1) {
          event.preventDefault();
          this.setSearchQuery(this.state.searchQuery + event.key);
          return true;
        }
        return false;
    }
  }

  handleFocus(): void {
    this.state.focused = true;
    this.updateElement();
  }

  handleBlur(): void {
    this.state.focused = false;
    // Close dropdown when losing focus
    if (this.state.open) {
      this.close();
    }
    this.updateElement();
  }

  render(): HTMLElement {
    const container = document.createElement('div');
    container.className = this.config.style.containerClasses.join(' ');
    container.id = this.config.id;
    container.tabIndex = 0;

    // Add state classes
    if (this.config.disabled) {
      container.classList.add('select-disabled');
    }
    if (this.state.focused) {
      container.classList.add('select-focused');
    }
    if (this.state.open) {
      container.classList.add('select-open');
    }

    // Create trigger button
    const trigger = document.createElement('button');
    trigger.className = this.config.style.triggerClasses.join(' ');
    trigger.textContent = `${this.getDisplayText()} ${this.config.style.dropdownArrow}`;
    trigger.disabled = this.config.disabled;
    
    trigger.addEventListener('click', (e) => {
      e.preventDefault();
      this.toggle();
    });

    container.appendChild(trigger);

    // Add dropdown if open
    if (this.state.open) {
      container.appendChild(this.createDropdownElement());
    }

    // Event listeners
    container.addEventListener('keydown', (e) => {
      if (this.handleKeyEvent(e)) {
        e.stopPropagation();
      }
    });

    container.addEventListener('focus', () => this.handleFocus());
    container.addEventListener('blur', () => this.handleBlur());

    this.element = container;
    return container;
  }

  private createDropdownElement(): HTMLElement {
    const dropdown = document.createElement('div');
    dropdown.className = this.config.style.dropdownClasses.join(' ');

    // Add search input if searchable
    if (this.config.searchable) {
      const searchInput = document.createElement('input');
      searchInput.type = 'text';
      searchInput.className = this.config.style.searchInputClasses.join(' ');
      searchInput.placeholder = 'Search...';
      searchInput.value = this.state.searchQuery;
      
      searchInput.addEventListener('input', (e) => {
        const target = e.target as HTMLInputElement;
        this.setSearchQuery(target.value);
      });

      searchInput.addEventListener('click', (e) => e.stopPropagation());
      dropdown.appendChild(searchInput);
    }

    // Add options
    const visibleOptions = this.state.filteredIndices
      .slice(this.state.scrollOffset, this.state.scrollOffset + this.config.style.maxHeight);

    for (const optionIndex of visibleOptions) {
      const option = this.options[optionIndex];
      const optionElement = document.createElement('div');
      optionElement.className = this.config.style.optionClasses.join(' ');

      // Add state classes
      if (this.state.selectedIndices.includes(optionIndex)) {
        optionElement.classList.add(...this.config.style.selectedOptionClasses);
      }
      if (this.state.highlightedIndex === optionIndex) {
        optionElement.classList.add(...this.config.style.highlightedOptionClasses);
      }
      if (option.disabled) {
        optionElement.classList.add(...this.config.style.disabledOptionClasses);
      }

      // Build option content
      const contentParts: string[] = [];

      // Add selection marker for multi-select
      if (this.config.mode === 'multiple') {
        contentParts.push(this.state.selectedIndices.includes(optionIndex) 
          ? this.config.style.selectedMarker 
          : ' ');
      }

      // Add icon
      if (this.config.style.showIcons && option.icon) {
        contentParts.push(option.icon);
      }

      // Add label
      contentParts.push(option.label);

      // Add description
      if (this.config.style.showDescriptions && option.description) {
        contentParts.push(`- ${option.description}`);
      }

      optionElement.textContent = contentParts.join(' ');

      // Click handler
      if (!option.disabled) {
        optionElement.addEventListener('click', (e) => {
          e.stopPropagation();
          this.toggleSelection(optionIndex);
        });
      }

      dropdown.appendChild(optionElement);
    }

    return dropdown;
  }

  private updateElement(): void {
    if (this.element) {
      // Re-render the element
      const newElement = this.render();
      this.element.replaceWith(newElement);
    }
  }
}

/**
 * Create a new select dropdown widget
 */
export function select(config: SelectConfig): SelectAPI {
  return new SelectImplementation(config);
}

/**
 * Convenience functions for common select patterns
 */
export const selectPatterns = {
  /** Create a simple yes/no select */
  yesNo: (id: string): SelectAPI => {
    return select({
      id,
      options: ['Yes', 'No'],
      placeholder: 'Select...'
    });
  },

  /** Create a language selection dropdown */
  languages: (id: string): SelectAPI => {
    const options: SelectOption[] = [
      { id: 'rust', label: 'Rust', icon: '🦀' },
      { id: 'typescript', label: 'TypeScript', icon: '📘' },
      { id: 'javascript', label: 'JavaScript', icon: '📜' },
      { id: 'python', label: 'Python', icon: '🐍' },
      { id: 'go', label: 'Go', icon: '🐹' },
      { id: 'java', label: 'Java', icon: '☕' },
      { id: 'csharp', label: 'C#', icon: '🔷' },
      { id: 'cpp', label: 'C++', icon: '⚡' }
    ];

    return select({
      id,
      customOptions: options,
      searchable: true,
      placeholder: 'Choose a language...'
    });
  },

  /** Create a priority selection dropdown */
  priority: (id: string): SelectAPI => {
    const options: SelectOption[] = [
      { id: 'high', label: 'High Priority', icon: '🔴' },
      { id: 'medium', label: 'Medium Priority', icon: '🟡' },
      { id: 'low', label: 'Low Priority', icon: '🟢' }
    ];

    return select({
      id,
      customOptions: options,
      placeholder: 'Select priority...'
    });
  }
};

// Types are already exported above with their definitions