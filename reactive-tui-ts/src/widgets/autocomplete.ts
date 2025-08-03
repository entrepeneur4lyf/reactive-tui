/**
 * Autocomplete Widget - TypeScript Implementation
 * 
 * A comprehensive search-as-you-type input with filtered suggestions, keyboard navigation,
 * and multi-select support for enhanced user interactions.
 * 
 * Features:
 * - Search-as-you-type with real-time filtering
 * - Multiple filter modes (contains, starts_with, fuzzy, custom)
 * - Keyboard navigation with arrow keys, Enter, Escape
 * - Single and multi-select modes
 * - Debounced input processing for performance
 * - Async suggestion loading with loading states
 * - Customizable suggestion rendering
 * - Highlight matching text in suggestions
 * - Maximum results limiting
 * - Custom scoring and sorting algorithms
 */

export type SuggestionId = string;

export interface AutocompleteSuggestion {
  id: SuggestionId;
  text: string;
  value: string;
  description?: string;
  category?: string;
  score?: number;
  metadata?: Record<string, any>;
}

export enum FilterMode {
  Contains = 'contains',
  StartsWith = 'starts_with',
  Fuzzy = 'fuzzy',
  Custom = 'custom'
}

export enum SelectionMode {
  Single = 'single',
  Multiple = 'multiple'
}

export interface AutocompleteState {
  query: string;
  suggestions: AutocompleteSuggestion[];
  filtered_suggestions: AutocompleteSuggestion[];
  selected_suggestions: SuggestionId[];
  highlighted_index: number;
  is_open: boolean;
  is_loading: boolean;
  is_focused: boolean;
  last_query_time: number;
  total_suggestions: number;
  visible_count: number;
}

export interface AutocompleteConfig {
  max_suggestions: number;
  min_query_length: number;
  debounce_ms: number;
  case_sensitive: boolean;
  filter_mode: FilterMode;
  selection_mode: SelectionMode;
  show_descriptions: boolean;
  show_categories: boolean;
  highlight_matches: boolean;
  auto_select_first: boolean;
  close_on_select: boolean;
  allow_custom_values: boolean;
  fuzzy_threshold: number;
  max_visible_suggestions: number;
}

export interface AutocompleteStyle {
  input_background: string;
  input_foreground: string;
  input_border: string;
  suggestions_background: string;
  suggestions_foreground: string;
  suggestions_border: string;
  highlighted_background: string;
  highlighted_foreground: string;
  selected_background: string;
  selected_foreground: string;
  match_highlight_background: string;
  match_highlight_foreground: string;
  description_color: string;
  category_color: string;
  loading_color: string;
  css_classes: string[];
}

export interface AutocompleteCallbacks {
  onQueryChange?: (query: string, autocomplete: Autocomplete) => void;
  onSuggestionSelect?: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void;
  onSuggestionActivate?: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void;
  onSelectionChange?: (selected: SuggestionId[], autocomplete: Autocomplete) => void;
  onOpen?: (autocomplete: Autocomplete) => void;
  onClose?: (autocomplete: Autocomplete) => void;
  onLoadSuggestions?: (query: string, autocomplete: Autocomplete) => Promise<AutocompleteSuggestion[]> | AutocompleteSuggestion[];
  onCustomFilter?: (query: string, suggestions: AutocompleteSuggestion[], autocomplete: Autocomplete) => AutocompleteSuggestion[];
}

export class Autocomplete {
  private id: string;
  private suggestions: AutocompleteSuggestion[] = [];
  private state: AutocompleteState;
  private config: AutocompleteConfig;
  private style: AutocompleteStyle;
  private callbacks: AutocompleteCallbacks;
  private debounceTimer?: NodeJS.Timeout;
  private loadingPromise?: Promise<void>;

  constructor(
    id: string,
    suggestions: AutocompleteSuggestion[] = [],
    config: Partial<AutocompleteConfig> = {},
    style: Partial<AutocompleteStyle> = {},
    callbacks: AutocompleteCallbacks = {}
  ) {
    this.id = id;
    this.suggestions = suggestions;
    this.callbacks = callbacks;

    this.config = {
      max_suggestions: 10,
      min_query_length: 1,
      debounce_ms: 150,
      case_sensitive: false,
      filter_mode: FilterMode.Contains,
      selection_mode: SelectionMode.Single,
      show_descriptions: true,
      show_categories: true,
      highlight_matches: true,
      auto_select_first: true,
      close_on_select: true,
      allow_custom_values: false,
      fuzzy_threshold: 0.6,
      max_visible_suggestions: 8,
      ...config
    };

    this.style = {
      input_background: '#ffffff',
      input_foreground: '#000000',
      input_border: '#cccccc',
      suggestions_background: '#ffffff',
      suggestions_foreground: '#000000',
      suggestions_border: '#cccccc',
      highlighted_background: '#0066cc',
      highlighted_foreground: '#ffffff',
      selected_background: '#e6f3ff',
      selected_foreground: '#0066cc',
      match_highlight_background: '#ffff00',
      match_highlight_foreground: '#000000',
      description_color: '#666666',
      category_color: '#888888',
      loading_color: '#999999',
      css_classes: [],
      ...style
    };

    this.state = {
      query: '',
      suggestions: [...this.suggestions],
      filtered_suggestions: [],
      selected_suggestions: [],
      highlighted_index: -1,
      is_open: false,
      is_loading: false,
      is_focused: false,
      last_query_time: 0,
      total_suggestions: this.suggestions.length,
      visible_count: 0
    };

    this.filterSuggestions();
  }

  // Query management
  setQuery(query: string): void {
    if (this.state.query === query) return;

    this.state.query = query;
    this.state.last_query_time = Date.now();

    // Clear existing debounce timer
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }

    // Debounce the filtering
    this.debounceTimer = setTimeout(() => {
      this.processQuery(query);
    }, this.config.debounce_ms);

    this.callbacks.onQueryChange?.(query, this);
  }

  private async processQuery(query: string): Promise<void> {
    // Check minimum query length
    if (query.length < this.config.min_query_length) {
      this.state.filtered_suggestions = [];
      this.state.is_open = false;
      this.state.highlighted_index = -1;
      return;
    }

    // Load suggestions if callback provided
    if (this.callbacks.onLoadSuggestions && !this.state.is_loading) {
      this.state.is_loading = true;
      try {
        const newSuggestions = await this.callbacks.onLoadSuggestions(query, this);
        if (Array.isArray(newSuggestions)) {
          this.suggestions = newSuggestions;
          this.state.suggestions = [...newSuggestions];
          this.state.total_suggestions = newSuggestions.length;
        }
      } catch (error) {
        console.error('Error loading suggestions:', error);
      } finally {
        this.state.is_loading = false;
      }
    }

    this.filterSuggestions();
    
    if (this.state.filtered_suggestions.length > 0) {
      this.state.is_open = true;
      if (this.config.auto_select_first) {
        this.state.highlighted_index = 0;
      }
      this.callbacks.onOpen?.(this);
    } else {
      this.state.is_open = false;
      this.state.highlighted_index = -1;
    }
  }

  private filterSuggestions(): void {
    if (!this.state.query || this.state.query.length < this.config.min_query_length) {
      this.state.filtered_suggestions = [];
      this.state.visible_count = 0;
      return;
    }

    let filtered: AutocompleteSuggestion[];

    if (this.callbacks.onCustomFilter && this.config.filter_mode === FilterMode.Custom) {
      filtered = this.callbacks.onCustomFilter(this.state.query, this.state.suggestions, this);
    } else {
      filtered = this.applyBuiltinFilter(this.state.query, this.state.suggestions);
    }

    // Limit results
    if (filtered.length > this.config.max_suggestions) {
      filtered = filtered.slice(0, this.config.max_suggestions);
    }

    this.state.filtered_suggestions = filtered;
    this.state.visible_count = Math.min(filtered.length, this.config.max_visible_suggestions);
  }

  private applyBuiltinFilter(query: string, suggestions: AutocompleteSuggestion[]): AutocompleteSuggestion[] {
    const searchQuery = this.config.case_sensitive ? query : query.toLowerCase();
    
    return suggestions
      .map(suggestion => {
        const text = this.config.case_sensitive ? suggestion.text : suggestion.text.toLowerCase();
        const value = this.config.case_sensitive ? suggestion.value : suggestion.value.toLowerCase();
        
        let score = 0;
        let matches = false;

        switch (this.config.filter_mode) {
          case FilterMode.StartsWith:
            if (text.startsWith(searchQuery) || value.startsWith(searchQuery)) {
              matches = true;
              score = text.startsWith(searchQuery) ? 1.0 : 0.8;
            }
            break;

          case FilterMode.Contains:
            if (text.includes(searchQuery) || value.includes(searchQuery)) {
              matches = true;
              const textIndex = text.indexOf(searchQuery);
              const valueIndex = value.indexOf(searchQuery);
              
              // Higher score for earlier matches
              if (textIndex === 0 || valueIndex === 0) {
                score = 1.0;
              } else if (textIndex > 0 || valueIndex > 0) {
                score = 0.8 - (Math.min(textIndex, valueIndex) / text.length) * 0.3;
              }
            }
            break;

          case FilterMode.Fuzzy:
            const fuzzyScore = this.calculateFuzzyScore(searchQuery, text);
            if (fuzzyScore >= this.config.fuzzy_threshold) {
              matches = true;
              score = fuzzyScore;
            }
            break;
        }

        return matches ? { ...suggestion, score } : null;
      })
      .filter(suggestion => suggestion !== null)
      .sort((a, b) => ((a as AutocompleteSuggestion).score || 0) - ((b as AutocompleteSuggestion).score || 0)) as AutocompleteSuggestion[];
  }

  private calculateFuzzyScore(query: string, text: string): number {
    if (query.length === 0) return 1;
    if (text.length === 0) return 0;
    
    let score = 0;
    let queryIndex = 0;
    let lastMatchIndex = -1;
    
    for (let i = 0; i < text.length && queryIndex < query.length; i++) {
      if (text[i] === query[queryIndex]) {
        score += 1;
        
        // Bonus for consecutive matches
        if (i === lastMatchIndex + 1) {
          score += 0.5;
        }
        
        // Bonus for matches at word boundaries
        if (i === 0 || text[i - 1] === ' ' || text[i - 1] === '_' || text[i - 1] === '-') {
          score += 0.3;
        }
        
        lastMatchIndex = i;
        queryIndex++;
      }
    }
    
    // Normalize score
    return queryIndex === query.length ? score / (query.length + text.length * 0.1) : 0;
  }

  // Navigation
  highlightNext(): boolean {
    if (this.state.filtered_suggestions.length === 0) return false;
    
    this.state.highlighted_index = 
      (this.state.highlighted_index + 1) % this.state.filtered_suggestions.length;
    return true;
  }

  highlightPrevious(): boolean {
    if (this.state.filtered_suggestions.length === 0) return false;
    
    this.state.highlighted_index = 
      this.state.highlighted_index <= 0 
        ? this.state.filtered_suggestions.length - 1 
        : this.state.highlighted_index - 1;
    return true;
  }

  highlightFirst(): boolean {
    if (this.state.filtered_suggestions.length === 0) return false;
    this.state.highlighted_index = 0;
    return true;
  }

  highlightLast(): boolean {
    if (this.state.filtered_suggestions.length === 0) return false;
    this.state.highlighted_index = this.state.filtered_suggestions.length - 1;
    return true;
  }

  // Selection
  selectHighlighted(): boolean {
    if (this.state.highlighted_index < 0 || 
        this.state.highlighted_index >= this.state.filtered_suggestions.length) {
      return false;
    }

    const suggestion = this.state.filtered_suggestions[this.state.highlighted_index];
    return this.selectSuggestion(suggestion.id);
  }

  selectSuggestion(id: SuggestionId): boolean {
    const suggestion = this.state.filtered_suggestions.find(s => s.id === id);
    if (!suggestion) return false;

    if (this.config.selection_mode === SelectionMode.Single) {
      this.state.selected_suggestions = [id];
      if (this.config.close_on_select) {
        this.close();
      }
    } else {
      const index = this.state.selected_suggestions.indexOf(id);
      if (index >= 0) {
        this.state.selected_suggestions.splice(index, 1);
      } else {
        this.state.selected_suggestions.push(id);
      }
    }

    this.callbacks.onSuggestionSelect?.(suggestion, this);
    this.callbacks.onSelectionChange?.(this.state.selected_suggestions, this);
    return true;
  }

  deselectSuggestion(id: SuggestionId): void {
    const index = this.state.selected_suggestions.indexOf(id);
    if (index >= 0) {
      this.state.selected_suggestions.splice(index, 1);
      this.callbacks.onSelectionChange?.(this.state.selected_suggestions, this);
    }
  }

  clearSelection(): void {
    this.state.selected_suggestions = [];
    this.callbacks.onSelectionChange?.([], this);
  }

  activateHighlighted(): boolean {
    if (this.state.highlighted_index < 0 || 
        this.state.highlighted_index >= this.state.filtered_suggestions.length) {
      return false;
    }

    const suggestion = this.state.filtered_suggestions[this.state.highlighted_index];
    this.callbacks.onSuggestionActivate?.(suggestion, this);
    return true;
  }

  // State management
  open(): void {
    if (!this.state.is_open) {
      this.state.is_open = true;
      this.callbacks.onOpen?.(this);
    }
  }

  close(): void {
    if (this.state.is_open) {
      this.state.is_open = false;
      this.state.highlighted_index = -1;
      this.callbacks.onClose?.(this);
    }
  }

  focus(): void {
    this.state.is_focused = true;
  }

  blur(): void {
    this.state.is_focused = false;
    // Optionally close on blur
    this.close();
  }

  // Data management
  setSuggestions(suggestions: AutocompleteSuggestion[]): void {
    this.suggestions = suggestions;
    this.state.suggestions = [...suggestions];
    this.state.total_suggestions = suggestions.length;
    this.filterSuggestions();
  }

  addSuggestion(suggestion: AutocompleteSuggestion): void {
    this.suggestions.push(suggestion);
    this.state.suggestions.push(suggestion);
    this.state.total_suggestions++;
    this.filterSuggestions();
  }

  removeSuggestion(id: SuggestionId): boolean {
    const index = this.suggestions.findIndex(s => s.id === id);
    if (index === -1) return false;

    this.suggestions.splice(index, 1);
    this.state.suggestions.splice(index, 1);
    this.state.total_suggestions--;
    
    // Remove from selections if present
    this.deselectSuggestion(id);
    
    this.filterSuggestions();
    return true;
  }

  // Event handling
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'ArrowDown':
        return this.highlightNext();
      case 'ArrowUp':
        return this.highlightPrevious();
      case 'Home':
        return this.highlightFirst();
      case 'End':
        return this.highlightLast();
      case 'Enter':
        if (this.state.is_open) {
          return this.selectHighlighted();
        }
        return false;
      case 'Escape':
        if (this.state.is_open) {
          this.close();
          return true;
        }
        return false;
      case 'Tab':
        if (this.state.is_open && this.state.highlighted_index >= 0) {
          return this.selectHighlighted();
        }
        return false;
      default:
        return false;
    }
  }

  // Getters
  getQuery(): string {
    return this.state.query;
  }

  getFilteredSuggestions(): AutocompleteSuggestion[] {
    return [...this.state.filtered_suggestions];
  }

  getSelectedSuggestions(): SuggestionId[] {
    return [...this.state.selected_suggestions];
  }

  getHighlightedIndex(): number {
    return this.state.highlighted_index;
  }

  getHighlightedSuggestion(): AutocompleteSuggestion | null {
    if (this.state.highlighted_index < 0 || 
        this.state.highlighted_index >= this.state.filtered_suggestions.length) {
      return null;
    }
    return this.state.filtered_suggestions[this.state.highlighted_index];
  }

  isOpen(): boolean {
    return this.state.is_open;
  }

  isLoading(): boolean {
    return this.state.is_loading;
  }

  isFocused(): boolean {
    return this.state.is_focused;
  }

  getTotalSuggestions(): number {
    return this.state.total_suggestions;
  }

  getVisibleCount(): number {
    return this.state.visible_count;
  }

  // Cleanup
  destroy(): void {
    if (this.debounceTimer) {
      clearTimeout(this.debounceTimer);
    }
  }
}

export class AutocompleteBuilder {
  private id: string;
  private initialSuggestions: AutocompleteSuggestion[] = [];
  private config: Partial<AutocompleteConfig> = {};
  private style: Partial<AutocompleteStyle> = {};
  private callbacks: AutocompleteCallbacks = {};

  constructor(id: string) {
    this.id = id;
  }

  suggestions(suggestions: AutocompleteSuggestion[]): this {
    this.initialSuggestions = suggestions;
    return this;
  }

  maxSuggestions(max: number): this {
    this.config.max_suggestions = max;
    return this;
  }

  minQueryLength(min: number): this {
    this.config.min_query_length = min;
    return this;
  }

  debounceMs(ms: number): this {
    this.config.debounce_ms = ms;
    return this;
  }

  caseSensitive(sensitive: boolean): this {
    this.config.case_sensitive = sensitive;
    return this;
  }

  filterMode(mode: FilterMode): this {
    this.config.filter_mode = mode;
    return this;
  }

  selectionMode(mode: SelectionMode): this {
    this.config.selection_mode = mode;
    return this;
  }

  showDescriptions(show: boolean): this {
    this.config.show_descriptions = show;
    return this;
  }

  showCategories(show: boolean): this {
    this.config.show_categories = show;
    return this;
  }

  highlightMatches(highlight: boolean): this {
    this.config.highlight_matches = highlight;
    return this;
  }

  autoSelectFirst(auto: boolean): this {
    this.config.auto_select_first = auto;
    return this;
  }

  closeOnSelect(close: boolean): this {
    this.config.close_on_select = close;
    return this;
  }

  allowCustomValues(allow: boolean): this {
    this.config.allow_custom_values = allow;
    return this;
  }

  fuzzyThreshold(threshold: number): this {
    this.config.fuzzy_threshold = threshold;
    return this;
  }

  maxVisibleSuggestions(max: number): this {
    this.config.max_visible_suggestions = max;
    return this;
  }

  onQueryChange(callback: (query: string, autocomplete: Autocomplete) => void): this {
    this.callbacks.onQueryChange = callback;
    return this;
  }

  onSuggestionSelect(callback: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void): this {
    this.callbacks.onSuggestionSelect = callback;
    return this;
  }

  onSuggestionActivate(callback: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void): this {
    this.callbacks.onSuggestionActivate = callback;
    return this;
  }

  onSelectionChange(callback: (selected: SuggestionId[], autocomplete: Autocomplete) => void): this {
    this.callbacks.onSelectionChange = callback;
    return this;
  }

  onOpen(callback: (autocomplete: Autocomplete) => void): this {
    this.callbacks.onOpen = callback;
    return this;
  }

  onClose(callback: (autocomplete: Autocomplete) => void): this {
    this.callbacks.onClose = callback;
    return this;
  }

  onLoadSuggestions(callback: (query: string, autocomplete: Autocomplete) => Promise<AutocompleteSuggestion[]> | AutocompleteSuggestion[]): this {
    this.callbacks.onLoadSuggestions = callback;
    return this;
  }

  onCustomFilter(callback: (query: string, suggestions: AutocompleteSuggestion[], autocomplete: Autocomplete) => AutocompleteSuggestion[]): this {
    this.callbacks.onCustomFilter = callback;
    return this;
  }

  build(): Autocomplete {
    return new Autocomplete(this.id, this.initialSuggestions, this.config, this.style, this.callbacks);
  }
}

// Convenience functions for common autocomplete patterns

export function countryAutocomplete(): Autocomplete {
  const countries: AutocompleteSuggestion[] = [
    { id: 'us', text: 'United States', value: 'US', description: 'North America' },
    { id: 'ca', text: 'Canada', value: 'CA', description: 'North America' },
    { id: 'uk', text: 'United Kingdom', value: 'GB', description: 'Europe' },
    { id: 'fr', text: 'France', value: 'FR', description: 'Europe' },
    { id: 'de', text: 'Germany', value: 'DE', description: 'Europe' },
    { id: 'jp', text: 'Japan', value: 'JP', description: 'Asia' },
    { id: 'au', text: 'Australia', value: 'AU', description: 'Oceania' },
    { id: 'br', text: 'Brazil', value: 'BR', description: 'South America' }
  ];

  return new AutocompleteBuilder('country-autocomplete')
    .suggestions(countries)
    .maxSuggestions(8)
    .filterMode(FilterMode.StartsWith)
    .showDescriptions(true)
    .autoSelectFirst(true)
    .build();
}

export function languageAutocomplete(): Autocomplete {
  const languages: AutocompleteSuggestion[] = [
    { id: 'js', text: 'JavaScript', value: 'javascript', category: 'Web' },
    { id: 'ts', text: 'TypeScript', value: 'typescript', category: 'Web' },
    { id: 'py', text: 'Python', value: 'python', category: 'General' },
    { id: 'rs', text: 'Rust', value: 'rust', category: 'Systems' },
    { id: 'go', text: 'Go', value: 'go', category: 'Systems' },
    { id: 'java', text: 'Java', value: 'java', category: 'Enterprise' },
    { id: 'cpp', text: 'C++', value: 'cpp', category: 'Systems' },
    { id: 'rb', text: 'Ruby', value: 'ruby', category: 'Web' }
  ];

  return new AutocompleteBuilder('language-autocomplete')
    .suggestions(languages)
    .maxSuggestions(6)
    .filterMode(FilterMode.Contains)
    .showCategories(true)
    .build();
}

export function userAutocomplete(users: Array<{id: string, name: string, email: string, role?: string}>): Autocomplete {
  const suggestions: AutocompleteSuggestion[] = users.map(user => ({
    id: user.id,
    text: user.name,
    value: user.email,
    description: user.role ? `${user.email} (${user.role})` : user.email,
    category: user.role || 'User'
  }));

  return new AutocompleteBuilder('user-autocomplete')
    .suggestions(suggestions)
    .maxSuggestions(10)
    .filterMode(FilterMode.Fuzzy)
    .fuzzyThreshold(0.4)
    .showDescriptions(true)
    .selectionMode(SelectionMode.Multiple)
    .build();
}

export function commandAutocomplete(commands: Array<{name: string, description: string, category?: string}>): Autocomplete {
  const suggestions: AutocompleteSuggestion[] = commands.map((cmd, index) => ({
    id: `cmd-${index}`,
    text: cmd.name,
    value: cmd.name,
    description: cmd.description,
    category: cmd.category || 'Command'
  }));

  return new AutocompleteBuilder('command-autocomplete')
    .suggestions(suggestions)
    .maxSuggestions(12)
    .filterMode(FilterMode.StartsWith)
    .showDescriptions(true)
    .showCategories(true)
    .closeOnSelect(true)
    .build();
}