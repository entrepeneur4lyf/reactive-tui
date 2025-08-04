# Autocomplete Widget

Advanced search-as-you-type input with filtered suggestions, keyboard navigation, multi-select support, fuzzy matching, and async data loading for enhanced user interactions.

## Overview

The Autocomplete widget provides comprehensive search functionality with real-time filtering, multiple filter modes, debounced input processing, keyboard navigation, and customizable suggestion rendering. Perfect for user searches, command palettes, and data selection interfaces.

```typescript
import { Autocomplete, AutocompleteBuilder, FilterMode, SelectionMode } from 'reactive-tui-ts'

const fileSearchAutocomplete = new AutocompleteBuilder('file-search')
  .suggestions([
    { id: '1', text: 'package.json', value: 'package.json', description: 'Node.js package configuration' },
    { id: '2', text: 'tsconfig.json', value: 'tsconfig.json', description: 'TypeScript configuration' },
    { id: '3', text: 'src/index.ts', value: 'src/index.ts', description: 'Main entry point' }
  ])
  .filterMode(FilterMode.Fuzzy)
  .maxSuggestions(10)
  .highlightMatches(true)
  .onSuggestionSelect((suggestion) => console.log('Selected:', suggestion.text))
  .build()
```

## Types

### AutocompleteSuggestion

```typescript
interface AutocompleteSuggestion {
  id: string
  text: string
  value: string
  description?: string
  category?: string
  score?: number
  metadata?: Record<string, any>
}
```

### AutocompleteConfig

```typescript
interface AutocompleteConfig {
  max_suggestions: number
  min_query_length: number
  debounce_ms: number
  case_sensitive: boolean
  filter_mode: FilterMode
  selection_mode: SelectionMode
  show_descriptions: boolean
  show_categories: boolean
  highlight_matches: boolean
  auto_select_first: boolean
  close_on_select: boolean
  allow_custom_values: boolean
  fuzzy_threshold: number
  max_visible_suggestions: number
}
```

### FilterMode

```typescript
enum FilterMode {
  Contains = 'contains',
  StartsWith = 'starts_with', 
  Fuzzy = 'fuzzy',
  Custom = 'custom'
}
```

### SelectionMode

```typescript
enum SelectionMode {
  Single = 'single',
  Multiple = 'multiple'
}
```

### AutocompleteCallbacks

```typescript
interface AutocompleteCallbacks {
  onQueryChange?: (query: string, autocomplete: Autocomplete) => void
  onSuggestionSelect?: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void
  onSuggestionActivate?: (suggestion: AutocompleteSuggestion, autocomplete: Autocomplete) => void
  onSelectionChange?: (selected: string[], autocomplete: Autocomplete) => void
  onOpen?: (autocomplete: Autocomplete) => void
  onClose?: (autocomplete: Autocomplete) => void
  onLoadSuggestions?: (query: string, autocomplete: Autocomplete) => Promise<AutocompleteSuggestion[]> | AutocompleteSuggestion[]
  onCustomFilter?: (query: string, suggestions: AutocompleteSuggestion[], autocomplete: Autocomplete) => AutocompleteSuggestion[]
}
```

## Basic Usage

### Simple Autocomplete

```typescript
import { Autocomplete, AutocompleteBuilder, FilterMode } from 'reactive-tui-ts'

const cityAutocomplete = new AutocompleteBuilder('city-search')
  .suggestions([
    { id: 'nyc', text: 'New York', value: 'new-york', description: 'New York, USA' },
    { id: 'la', text: 'Los Angeles', value: 'los-angeles', description: 'California, USA' },
    { id: 'chi', text: 'Chicago', value: 'chicago', description: 'Illinois, USA' },
    { id: 'hou', text: 'Houston', value: 'houston', description: 'Texas, USA' },
    { id: 'pho', text: 'Phoenix', value: 'phoenix', description: 'Arizona, USA' }
  ])
  .filterMode(FilterMode.StartsWith)
  .maxSuggestions(5)
  .minQueryLength(2)
  .showDescriptions(true)
  .autoSelectFirst(true)
  .build()

// Set query and handle results
cityAutocomplete.setQuery('new')
console.log('Filtered suggestions:', cityAutocomplete.getFilteredSuggestions())

// Handle keyboard navigation
cityAutocomplete.handleKeyPress('ArrowDown')  // Highlight next
cityAutocomplete.handleKeyPress('Enter')       // Select highlighted
```

### Multi-Select Autocomplete

```typescript
import { SelectionMode } from 'reactive-tui-ts'

const skillsAutocomplete = new AutocompleteBuilder('skills-select')
  .suggestions([
    { id: 'js', text: 'JavaScript', value: 'javascript', category: 'Programming' },
    { id: 'py', text: 'Python', value: 'python', category: 'Programming' },
    { id: 'rs', text: 'Rust', value: 'rust', category: 'Programming' },
    { id: 'react', text: 'React', value: 'react', category: 'Framework' },
    { id: 'vue', text: 'Vue.js', value: 'vue', category: 'Framework' },
    { id: 'node', text: 'Node.js', value: 'nodejs', category: 'Runtime' }
  ])
  .selectionMode(SelectionMode.Multiple)
  .filterMode(FilterMode.Contains)
  .showCategories(true)
  .closeOnSelect(false)
  .onSelectionChange((selected, autocomplete) => {
    console.log('Selected skills:', selected)
  })
  .build()

// Select multiple items
skillsAutocomplete.selectSuggestion('js')
skillsAutocomplete.selectSuggestion('py')
skillsAutocomplete.selectSuggestion('react')

console.log('Selected:', skillsAutocomplete.getSelectedSuggestions())
// Output: ['js', 'py', 'react']
```

## Filter Modes

### StartsWith Filter

```typescript
const startsWithAutocomplete = new AutocompleteBuilder('starts-with')
  .suggestions([
    { id: '1', text: 'Apple', value: 'apple' },
    { id: '2', text: 'Application', value: 'application' },
    { id: '3', text: 'Pineapple', value: 'pineapple' },
    { id: '4', text: 'Orange', value: 'orange' }
  ])
  .filterMode(FilterMode.StartsWith)
  .build()

// Query 'app' will match 'Apple' and 'Application' but not 'Pineapple'
startsWithAutocomplete.setQuery('app')
```

### Contains Filter

```typescript
const containsAutocomplete = new AutocompleteBuilder('contains')
  .suggestions([
    { id: '1', text: 'JavaScript', value: 'javascript' },
    { id: '2', text: 'TypeScript', value: 'typescript' },
    { id: '3', text: 'CoffeeScript', value: 'coffeescript' },
    { id: '4', text: 'ActionScript', value: 'actionscript' }
  ])
  .filterMode(FilterMode.Contains)
  .build()

// Query 'script' will match all items containing 'script'
containsAutocomplete.setQuery('script')
```

### Fuzzy Filter

```typescript
const fuzzyAutocomplete = new AutocompleteBuilder('fuzzy')
  .suggestions([
    { id: '1', text: 'JavaScript Framework', value: 'js-framework' },
    { id: '2', text: 'Java Application', value: 'java-app' },
    { id: '3', text: 'React Component', value: 'react-component' },
    { id: '4', text: 'Vue.js Router', value: 'vue-router' }
  ])
  .filterMode(FilterMode.Fuzzy)
  .fuzzyThreshold(0.6)
  .build()

// Query 'jsfr' can match 'JavaScript Framework' with fuzzy matching
fuzzyAutocomplete.setQuery('jsfr')
```

### Custom Filter

```typescript
const customFilterAutocomplete = new AutocompleteBuilder('custom-filter')
  .suggestions([
    { id: '1', text: 'Alice Johnson', value: 'alice', metadata: { department: 'Engineering', level: 'Senior' } },
    { id: '2', text: 'Bob Smith', value: 'bob', metadata: { department: 'Marketing', level: 'Junior' } },
    { id: '3', text: 'Carol Davis', value: 'carol', metadata: { department: 'Engineering', level: 'Lead' } }
  ])
  .filterMode(FilterMode.Custom)
  .onCustomFilter((query, suggestions, autocomplete) => {
    const searchTerm = query.toLowerCase()
    
    return suggestions
      .filter(suggestion => {
        // Search in text, value, and metadata
        const textMatch = suggestion.text.toLowerCase().includes(searchTerm)
        const departmentMatch = suggestion.metadata?.department?.toLowerCase().includes(searchTerm)
        const levelMatch = suggestion.metadata?.level?.toLowerCase().includes(searchTerm)
        
        return textMatch || departmentMatch || levelMatch
      })
      .sort((a, b) => {
        // Prioritize exact name matches
        const aNameMatch = a.text.toLowerCase().startsWith(searchTerm)
        const bNameMatch = b.text.toLowerCase().startsWith(searchTerm)
        
        if (aNameMatch && !bNameMatch) return -1
        if (!aNameMatch && bNameMatch) return 1
        return 0
      })
  })
  .build()

// Query 'eng' matches users in Engineering department
customFilterAutocomplete.setQuery('eng')
```

## Async Data Loading

### Dynamic Suggestion Loading

```typescript
const searchApiAutocomplete = new AutocompleteBuilder('search-api')
  .minQueryLength(3)
  .debounceMs(300)
  .maxSuggestions(8)
  .onLoadSuggestions(async (query, autocomplete) => {
    try {
      // Simulate API call
      const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`)
      const data = await response.json()
      
      return data.results.map((item: any, index: number) => ({
        id: `result-${index}`,
        text: item.title,
        value: item.id,
        description: item.description,
        category: item.category,
        metadata: item
      }))
    } catch (error) {
      console.error('Failed to load suggestions:', error)
      return []
    }
  })
  .onQueryChange((query, autocomplete) => {
    if (autocomplete.isLoading()) {
      console.log('Loading suggestions for:', query)
    }
  })
  .build()

// Set query triggers async loading
searchApiAutocomplete.setQuery('javascript')
```

### Cached Async Loading

```typescript
class CachedAutocomplete {
  private cache = new Map<string, AutocompleteSuggestion[]>()
  private autocomplete: Autocomplete

  constructor() {
    this.autocomplete = new AutocompleteBuilder('cached-search')
      .debounceMs(200)
      .onLoadSuggestions(async (query) => {
        // Check cache first
        if (this.cache.has(query)) {
          console.log('Cache hit for:', query)
          return this.cache.get(query)!
        }

        // Load from API
        console.log('Loading from API:', query)
        const suggestions = await this.loadFromApi(query)
        
        // Cache results
        this.cache.set(query, suggestions)
        return suggestions
      })
      .build()
  }

  private async loadFromApi(query: string): Promise<AutocompleteSuggestion[]> {
    // Simulate API delay
    await new Promise(resolve => setTimeout(resolve, 500))
    
    // Mock data based on query
    const mockResults = [
      { id: '1', text: `${query} Result 1`, value: `${query}-1`, description: 'First result' },
      { id: '2', text: `${query} Result 2`, value: `${query}-2`, description: 'Second result' },
      { id: '3', text: `${query} Result 3`, value: `${query}-3`, description: 'Third result' }
    ]

    return mockResults
  }

  getAutocomplete(): Autocomplete {
    return this.autocomplete
  }

  clearCache(): void {
    this.cache.clear()
  }
}

// Usage
const cachedSearch = new CachedAutocomplete()
const autocomplete = cachedSearch.getAutocomplete()

autocomplete.setQuery('react')  // Loads from API
autocomplete.setQuery('vue')    // Loads from API  
autocomplete.setQuery('react')  // Uses cache
```

## Keyboard Navigation

### Navigation Methods

```typescript
const navigableAutocomplete = new AutocompleteBuilder('navigable')
  .suggestions([
    { id: '1', text: 'First Option', value: 'first' },
    { id: '2', text: 'Second Option', value: 'second' },
    { id: '3', text: 'Third Option', value: 'third' },
    { id: '4', text: 'Fourth Option', value: 'fourth' },
    { id: '5', text: 'Fifth Option', value: 'fifth' }
  ])
  .autoSelectFirst(true)
  .build()

// Open suggestions
navigableAutocomplete.setQuery('option')

// Navigation methods
autocomplete.highlightNext()        // Move to next suggestion
autocomplete.highlightPrevious()    // Move to previous suggestion
autocomplete.highlightFirst()       // Jump to first suggestion
autocomplete.highlightLast()        // Jump to last suggestion

// Selection methods
autocomplete.selectHighlighted()    // Select currently highlighted
autocomplete.activateHighlighted()  // Activate (trigger callback) without selecting

// State queries
console.log('Highlighted index:', autocomplete.getHighlightedIndex())
console.log('Highlighted suggestion:', autocomplete.getHighlightedSuggestion())
```

### Keyboard Event Handling

```typescript
const keyboardAutocomplete = new AutocompleteBuilder('keyboard')
  .suggestions([
    { id: '1', text: 'Home', value: 'home' },
    { id: '2', text: 'About', value: 'about' },
    { id: '3', text: 'Services', value: 'services' },
    { id: '4', text: 'Contact', value: 'contact' }
  ])
  .build()

// Handle keyboard events
document.addEventListener('keydown', (event) => {
  if (keyboardAutocomplete.isFocused()) {
    const handled = keyboardAutocomplete.handleKeyPress(event.key)
    
    if (handled) {
      event.preventDefault()
    }
  }
})

// Supported keys:
// - ArrowDown: Highlight next suggestion
// - ArrowUp: Highlight previous suggestion  
// - Home: Highlight first suggestion
// - End: Highlight last suggestion
// - Enter: Select highlighted suggestion
// - Tab: Select highlighted suggestion
// - Escape: Close suggestions
```

## Styling and Theming

### Custom Styling

```typescript
const styledAutocomplete = new AutocompleteBuilder('styled')
  .suggestions([
    { id: '1', text: 'Option 1', value: '1' },
    { id: '2', text: 'Option 2', value: '2' }
  ])
  .build()

// Get autocomplete style configuration
const customStyle = {
  input_background: '#2d3748',
  input_foreground: '#f7fafc',
  input_border: '#4a5568',
  suggestions_background: '#2d3748',
  suggestions_foreground: '#f7fafc',
  suggestions_border: '#4a5568',
  highlighted_background: '#4299e1',
  highlighted_foreground: '#ffffff',
  selected_background: '#63b3ed',
  selected_foreground: '#1a202c',
  match_highlight_background: '#ed8936',
  match_highlight_foreground: '#1a202c',
  description_color: '#a0aec0',
  category_color: '#718096',
  loading_color: '#9ca3af',
  css_classes: ['dark-theme', 'custom-autocomplete']
}

// Apply styling (implementation would use these values)
console.log('Custom style configuration:', customStyle)
```

### Match Highlighting

```typescript
const highlightAutocomplete = new AutocompleteBuilder('highlight')
  .suggestions([
    { id: '1', text: 'JavaScript Developer', value: 'js-dev', description: 'Frontend JavaScript development' },
    { id: '2', text: 'Java Backend Engineer', value: 'java-be', description: 'Server-side Java development' },
    { id: '3', text: 'TypeScript Specialist', value: 'ts-spec', description: 'Advanced TypeScript programming' }
  ])
  .highlightMatches(true)
  .filterMode(FilterMode.Contains)
  .build()

// When user types 'java', matches are highlighted in results:
// "JavaScript Developer" -> "<mark>Java</mark>Script Developer"
// "Java Backend Engineer" -> "<mark>Java</mark> Backend Engineer"  
highlightAutocomplete.setQuery('java')
```

## Pre-built Autocomplete Types

### Country Autocomplete

```typescript
import { countryAutocomplete } from 'reactive-tui-ts'

const countries = countryAutocomplete()

// Pre-configured with countries, starts-with filter, descriptions
countries.setQuery('unit')
// Matches: United States, United Kingdom
```

### Programming Language Autocomplete

```typescript
import { languageAutocomplete } from 'reactive-tui-ts'

const languages = languageAutocomplete()

// Pre-configured with programming languages, contains filter, categories
languages.setQuery('script')
// Matches: JavaScript, TypeScript
```

### User Autocomplete

```typescript
import { userAutocomplete } from 'reactive-tui-ts'

const users = [
  { id: '1', name: 'John Doe', email: 'john@example.com', role: 'Developer' },
  { id: '2', name: 'Jane Smith', email: 'jane@example.com', role: 'Designer' },
  { id: '3', name: 'Bob Johnson', email: 'bob@example.com', role: 'Manager' }
]

const userSearch = userAutocomplete(users)

// Pre-configured with fuzzy search, multi-select, descriptions
userSearch.setQuery('jo')
// Matches: John Doe, Bob Johnson (fuzzy match)
```

### Command Autocomplete

```typescript
import { commandAutocomplete } from 'reactive-tui-ts'

const commands = [
  { name: 'git commit', description: 'Record changes to the repository', category: 'Git' },
  { name: 'git push', description: 'Update remote refs along with objects', category: 'Git' },
  { name: 'npm install', description: 'Install package dependencies', category: 'NPM' },
  { name: 'npm run', description: 'Run package scripts', category: 'NPM' }
]

const commandPalette = commandAutocomplete(commands)

// Pre-configured for command palette usage
commandPalette.setQuery('git')
// Shows git commands with descriptions and categories
```

## Real-World Examples

### Code Editor Search

```typescript
import { AutocompleteBuilder, FilterMode, SelectionMode } from 'reactive-tui-ts'

class CodeEditorSearch {
  private fileAutocomplete: Autocomplete
  private symbolAutocomplete: Autocomplete
  private workspaceSearchCache = new Map<string, AutocompleteSuggestion[]>()

  constructor() {
    this.setupFileSearch()
    this.setupSymbolSearch()
  }

  private setupFileSearch() {
    this.fileAutocomplete = new AutocompleteBuilder('file-search')
      .minQueryLength(2)
      .debounceMs(200)
      .maxSuggestions(15)
      .filterMode(FilterMode.Fuzzy)
      .fuzzyThreshold(0.4)
      .highlightMatches(true)
      .onLoadSuggestions(async (query) => this.loadFiles(query))
      .onSuggestionSelect((suggestion) => this.openFile(suggestion))
      .build()
  }

  private setupSymbolSearch() {
    this.symbolAutocomplete = new AutocompleteBuilder('symbol-search')
      .minQueryLength(1)
      .debounceMs(150)
      .maxSuggestions(20)
      .filterMode(FilterMode.Contains)
      .showCategories(true)
      .showDescriptions(true)
      .onLoadSuggestions(async (query) => this.loadSymbols(query))
      .onSuggestionSelect((suggestion) => this.goToSymbol(suggestion))
      .build()
  }

  private async loadFiles(query: string): Promise<AutocompleteSuggestion[]> {
    try {
      // Simulate workspace file search
      const response = await fetch(`/api/workspace/files?q=${encodeURIComponent(query)}`)
      const files = await response.json()

      return files.map((file: any) => ({
        id: file.path,
        text: file.name,
        value: file.path,
        description: file.relativePath,
        category: this.getFileCategory(file.extension),
        metadata: {
          size: file.size,
          modified: file.lastModified,
          extension: file.extension
        }
      }))
    } catch (error) {
      console.error('Failed to load files:', error)
      return []
    }
  }

  private async loadSymbols(query: string): Promise<AutocompleteSuggestion[]> {
    const cacheKey = `symbols:${query}`
    
    if (this.workspaceSearchCache.has(cacheKey)) {
      return this.workspaceSearchCache.get(cacheKey)!
    }

    try {
      // Load symbols from language server
      const response = await fetch(`/api/workspace/symbols?q=${encodeURIComponent(query)}`)
      const symbols = await response.json()

      const suggestions = symbols.map((symbol: any) => ({
        id: `${symbol.file}:${symbol.line}:${symbol.column}`,
        text: symbol.name,
        value: symbol.name,
        description: `${symbol.containerName ? symbol.containerName + '.' : ''}${symbol.name}`,
        category: symbol.kind, // 'Function', 'Class', 'Variable', etc.
        metadata: {
          file: symbol.file,
          line: symbol.line,
          column: symbol.column,
          kind: symbol.kind
        }
      }))

      this.workspaceSearchCache.set(cacheKey, suggestions)
      return suggestions
    } catch (error) {
      console.error('Failed to load symbols:', error)
      return []
    }
  }

  private getFileCategory(extension: string): string {
    const categories: Record<string, string> = {
      'ts': 'TypeScript',
      'js': 'JavaScript', 
      'tsx': 'React',
      'jsx': 'React',
      'vue': 'Vue',
      'py': 'Python',
      'rs': 'Rust',
      'go': 'Go',
      'java': 'Java',
      'cpp': 'C++',
      'c': 'C',
      'cs': 'C#',
      'php': 'PHP',
      'rb': 'Ruby',
      'md': 'Markdown',
      'json': 'JSON',
      'yaml': 'YAML',
      'yml': 'YAML',
      'xml': 'XML',
      'html': 'HTML',
      'css': 'CSS',
      'scss': 'SCSS',
      'less': 'LESS'
    }
    
    return categories[extension] || 'File'
  }

  private openFile(suggestion: AutocompleteSuggestion) {
    console.log(`Opening file: ${suggestion.value}`)
    
    // Simulate opening file in editor
    const metadata = suggestion.metadata
    if (metadata) {
      console.log(`File size: ${metadata.size} bytes`)
      console.log(`Last modified: ${metadata.modified}`)
    }
  }

  private goToSymbol(suggestion: AutocompleteSuggestion) {
    const metadata = suggestion.metadata
    if (metadata) {
      console.log(`Going to ${metadata.kind}: ${suggestion.text}`)
      console.log(`Location: ${metadata.file}:${metadata.line}:${metadata.column}`)
    }
  }

  // Public API methods
  searchFiles(query: string) {
    this.fileAutocomplete.setQuery(query)
    return this.fileAutocomplete
  }

  searchSymbols(query: string) {
    this.symbolAutocomplete.setQuery(query)
    return this.symbolAutocomplete
  }

  handleKeyPress(searchType: 'files' | 'symbols', key: string): boolean {
    const autocomplete = searchType === 'files' ? this.fileAutocomplete : this.symbolAutocomplete
    return autocomplete.handleKeyPress(key)
  }

  clearCache() {
    this.workspaceSearchCache.clear()
  }
}

// Usage
const editorSearch = new CodeEditorSearch()

// File search
const fileSearch = editorSearch.searchFiles('comp')
fileSearch.onOpen(() => console.log('File search opened'))

// Symbol search  
const symbolSearch = editorSearch.searchSymbols('function')
symbolSearch.onOpen(() => console.log('Symbol search opened'))

// Keyboard handling
document.addEventListener('keydown', (event) => {
  if (event.ctrlKey && event.key === 'p') {
    event.preventDefault()
    editorSearch.searchFiles('')
  }
  
  if (event.ctrlKey && event.shiftKey && event.key === 'O') {
    event.preventDefault()
    editorSearch.searchSymbols('')
  }
})
```

### Multi-Select Tag Input

```typescript
import { AutocompleteBuilder, SelectionMode, FilterMode } from 'reactive-tui-ts'

class TagInputWidget {
  private autocomplete: Autocomplete
  private selectedTags: Set<string> = new Set()
  private availableTags: AutocompleteSuggestion[]

  constructor(availableTags: string[]) {
    this.availableTags = availableTags.map((tag, index) => ({
      id: `tag-${index}`,
      text: tag,
      value: tag,
      category: this.getCategoryForTag(tag)
    }))

    this.setupAutocomplete()
  }

  private setupAutocomplete() {
    this.autocomplete = new AutocompleteBuilder('tag-input')
      .suggestions(this.availableTags)
      .selectionMode(SelectionMode.Multiple)
      .filterMode(FilterMode.StartsWith)
      .maxSuggestions(8)
      .minQueryLength(1)
      .closeOnSelect(false)
      .autoSelectFirst(false)
      .showCategories(true)
      .onSuggestionSelect((suggestion) => this.addTag(suggestion.value))
      .onSelectionChange((selected) => this.updateSelectedTags(selected))
      .build()
  }

  private getCategoryForTag(tag: string): string {
    if (tag.startsWith('#')) return 'Hashtag'
    if (tag.includes('@')) return 'Mention'
    if (['urgent', 'important', 'low', 'high'].includes(tag.toLowerCase())) return 'Priority'
    if (['bug', 'feature', 'enhancement', 'task'].includes(tag.toLowerCase())) return 'Type'
    return 'General'
  }

  private addTag(tag: string) {
    if (!this.selectedTags.has(tag)) {
      this.selectedTags.add(tag)
      this.updateUI()
      console.log(`Added tag: ${tag}`)
    }
  }

  private removeTag(tag: string) {
    if (this.selectedTags.has(tag)) {
      this.selectedTags.delete(tag)
      
      // Update autocomplete selection
      const suggestion = this.availableTags.find(s => s.value === tag)
      if (suggestion) {
        this.autocomplete.deselectSuggestion(suggestion.id)
      }
      
      this.updateUI()
      console.log(`Removed tag: ${tag}`)
    }
  }

  private updateSelectedTags(selectedIds: string[]) {
    const newTags = selectedIds
      .map(id => this.availableTags.find(tag => tag.id === id))
      .filter(tag => tag !== undefined)
      .map(tag => tag!.value)

    this.selectedTags.clear()
    newTags.forEach(tag => this.selectedTags.add(tag))
    this.updateUI()
  }

  private updateUI() {
    console.log('Current tags:', Array.from(this.selectedTags))
    
    // Filter out already selected tags from suggestions
    const unselectedTags = this.availableTags.filter(tag => 
      !this.selectedTags.has(tag.value)
    )
    
    this.autocomplete.setSuggestions(unselectedTags)
  }

  // Public API
  addCustomTag(tag: string) {
    // Add custom tag not in predefined list
    const customTag: AutocompleteSuggestion = {
      id: `custom-${Date.now()}`,
      text: tag,
      value: tag,
      category: 'Custom'
    }
    
    this.availableTags.push(customTag)
    this.addTag(tag)
  }

  getSelectedTags(): string[] {
    return Array.from(this.selectedTags)
  }

  setSelectedTags(tags: string[]) {
    this.selectedTags.clear()
    tags.forEach(tag => this.selectedTags.add(tag))
    
    // Update autocomplete selections
    const selectedIds = tags
      .map(tag => this.availableTags.find(t => t.value === tag)?.id)
      .filter(id => id !== undefined) as string[]
    
    selectedIds.forEach(id => this.autocomplete.selectSuggestion(id))
    this.updateUI()
  }

  clearTags() {
    this.selectedTags.clear()
    this.autocomplete.clearSelection()
    this.updateUI()
  }

  searchTags(query: string) {
    this.autocomplete.setQuery(query)
  }

  handleKeyPress(key: string): boolean {
    if (key === 'Backspace' && this.autocomplete.getQuery() === '') {
      // Remove last tag on backspace when input is empty
      const tags = Array.from(this.selectedTags)
      if (tags.length > 0) {
        this.removeTag(tags[tags.length - 1])
        return true
      }
    }
    
    if (key === 'Enter' && this.autocomplete.getQuery().trim()) {
      // Add custom tag on Enter if no suggestion is highlighted
      const highlighted = this.autocomplete.getHighlightedSuggestion()
      if (!highlighted) {
        this.addCustomTag(this.autocomplete.getQuery().trim())
        this.autocomplete.setQuery('')
        return true
      }
    }
    
    return this.autocomplete.handleKeyPress(key)
  }

  getAutocomplete(): Autocomplete {
    return this.autocomplete
  }
}

// Usage
const predefinedTags = [
  'javascript', 'typescript', 'react', 'vue', 'angular',
  'nodejs', 'python', 'rust', 'go', 'java',
  'bug', 'feature', 'enhancement', 'task',
  'urgent', 'important', 'low', 'high',
  '#frontend', '#backend', '#fullstack',
  '@team-lead', '@designer', '@developer'
]

const tagInput = new TagInputWidget(predefinedTags)

// Set initial tags
tagInput.setSelectedTags(['javascript', 'react', 'bug', 'urgent'])

// Search for tags
tagInput.searchTags('java')

// Handle keyboard input
document.addEventListener('keydown', (event) => {
  const handled = tagInput.handleKeyPress(event.key)
  if (handled) {
    event.preventDefault()
  }
})

// Get current tags
console.log('Selected tags:', tagInput.getSelectedTags())
```

### Search Results with Categories

```typescript
import { AutocompleteBuilder, FilterMode } from 'reactive-tui-ts'

class UniversalSearch {
  private autocomplete: Autocomplete
  private searchProviders: Map<string, SearchProvider> = new Map()

  constructor() {
    this.setupSearchProviders()
    this.setupAutocomplete()
  }

  private setupSearchProviders() {
    // File search provider
    this.searchProviders.set('files', {
      name: 'Files',
      search: async (query: string) => {
        const files = await this.mockFileSearch(query)
        return files.map(file => ({
          id: `file:${file.path}`,
          text: file.name,
          value: file.path,
          description: file.path,
          category: 'Files',
          metadata: { type: 'file', ...file }
        }))
      }
    })

    // Symbol search provider
    this.searchProviders.set('symbols', {
      name: 'Symbols',
      search: async (query: string) => {
        const symbols = await this.mockSymbolSearch(query)
        return symbols.map(symbol => ({
          id: `symbol:${symbol.name}:${symbol.file}`,
          text: symbol.name,
          value: symbol.name,
          description: `${symbol.kind} in ${symbol.file}`,
          category: 'Symbols',
          metadata: { type: 'symbol', ...symbol }
        }))
      }
    })

    // Command search provider
    this.searchProviders.set('commands', {
      name: 'Commands',
      search: async (query: string) => {
        const commands = await this.mockCommandSearch(query)
        return commands.map(cmd => ({
          id: `command:${cmd.id}`,
          text: cmd.name,
          value: cmd.id,
          description: cmd.description,
          category: 'Commands',
          metadata: { type: 'command', ...cmd }
        }))
      }
    })

    // Recent items provider
    this.searchProviders.set('recent', {
      name: 'Recent',
      search: async (query: string) => {
        const recent = await this.mockRecentSearch(query)
        return recent.map(item => ({
          id: `recent:${item.id}`,
          text: item.title,
          value: item.id,
          description: `Opened ${item.lastAccessed}`,
          category: 'Recent',
          metadata: { type: 'recent', ...item }
        }))
      }
    })
  }

  private setupAutocomplete() {
    this.autocomplete = new AutocompleteBuilder('universal-search')
      .minQueryLength(2)
      .debounceMs(300)
      .maxSuggestions(20)
      .filterMode(FilterMode.Custom)
      .showCategories(true)
      .showDescriptions(true)
      .highlightMatches(true)
      .onLoadSuggestions(async (query) => this.searchAll(query))
      .onCustomFilter((query, suggestions) => {
        // Custom sorting: prioritize exact matches, then category order
        return suggestions.sort((a, b) => {
          const aExact = a.text.toLowerCase() === query.toLowerCase()
          const bExact = b.text.toLowerCase() === query.toLowerCase()
          
          if (aExact && !bExact) return -1
          if (!aExact && bExact) return 1
          
          const categoryOrder = ['Recent', 'Files', 'Symbols', 'Commands']
          const aCategoryIndex = categoryOrder.indexOf(a.category || '')
          const bCategoryIndex = categoryOrder.indexOf(b.category || '')
          
          return aCategoryIndex - bCategoryIndex
        })
      })
      .onSuggestionSelect((suggestion) => this.handleSelection(suggestion))
      .build()
  }

  private async searchAll(query: string): Promise<AutocompleteSuggestion[]> {
    const searchPromises = Array.from(this.searchProviders.values()).map(provider =>
      provider.search(query).catch(error => {
        console.error(`Search provider ${provider.name} failed:`, error)
        return []
      })
    )

    const results = await Promise.all(searchPromises)
    return results.flat()
  }

  private async mockFileSearch(query: string): Promise<any[]> {
    // Simulate file search
    await new Promise(resolve => setTimeout(resolve, 100))
    
    const mockFiles = [
      { name: 'package.json', path: '/project/package.json', size: 1024 },
      { name: 'index.ts', path: '/project/src/index.ts', size: 2048 },
      { name: 'components.tsx', path: '/project/src/components.tsx', size: 4096 },
      { name: 'utils.js', path: '/project/src/utils.js', size: 1536 }
    ]

    return mockFiles.filter(file => 
      file.name.toLowerCase().includes(query.toLowerCase()) ||
      file.path.toLowerCase().includes(query.toLowerCase())
    )
  }

  private async mockSymbolSearch(query: string): Promise<any[]> {
    await new Promise(resolve => setTimeout(resolve, 150))
    
    const mockSymbols = [
      { name: 'useState', kind: 'Function', file: 'hooks.ts', line: 10 },
      { name: 'Component', kind: 'Class', file: 'component.tsx', line: 25 },
      { name: 'handleClick', kind: 'Method', file: 'button.tsx', line: 45 },
      { name: 'API_URL', kind: 'Constant', file: 'config.ts', line: 5 }
    ]

    return mockSymbols.filter(symbol =>
      symbol.name.toLowerCase().includes(query.toLowerCase())
    )
  }

  private async mockCommandSearch(query: string): Promise<any[]> {
    await new Promise(resolve => setTimeout(resolve, 50))
    
    const mockCommands = [
      { id: 'file.new', name: 'New File', description: 'Create a new file' },
      { id: 'file.open', name: 'Open File', description: 'Open an existing file' },
      { id: 'edit.find', name: 'Find', description: 'Search in current file' },
      { id: 'view.toggle', name: 'Toggle Sidebar', description: 'Show/hide sidebar' }
    ]

    return mockCommands.filter(cmd =>
      cmd.name.toLowerCase().includes(query.toLowerCase()) ||
      cmd.description.toLowerCase().includes(query.toLowerCase())
    )
  }

  private async mockRecentSearch(query: string): Promise<any[]> {
    await new Promise(resolve => setTimeout(resolve, 25))
    
    const mockRecent = [
      { id: 'recent1', title: 'main.ts', lastAccessed: '2 minutes ago' },
      { id: 'recent2', title: 'README.md', lastAccessed: '5 minutes ago' },
      { id: 'recent3', title: 'package.json', lastAccessed: '10 minutes ago' }
    ]

    return mockRecent.filter(item =>
      item.title.toLowerCase().includes(query.toLowerCase())
    )
  }

  private handleSelection(suggestion: AutocompleteSuggestion) {
    const metadata = suggestion.metadata
    
    switch (metadata?.type) {
      case 'file':
        console.log(`Opening file: ${suggestion.value}`)
        break
      
      case 'symbol':
        console.log(`Navigating to symbol: ${suggestion.text} in ${metadata.file}:${metadata.line}`)
        break
      
      case 'command':
        console.log(`Executing command: ${suggestion.value}`)
        break
      
      case 'recent':
        console.log(`Opening recent item: ${suggestion.text}`)
        break
      
      default:
        console.log(`Selected: ${suggestion.text}`)
    }
  }

  // Public API
  search(query: string) {
    this.autocomplete.setQuery(query)
    return this.autocomplete
  }

  handleKeyPress(key: string): boolean {
    return this.autocomplete.handleKeyPress(key)
  }

  close() {
    this.autocomplete.close()
  }

  isOpen(): boolean {
    return this.autocomplete.isOpen()
  }
}

interface SearchProvider {
  name: string
  search: (query: string) => Promise<AutocompleteSuggestion[]>
}

// Usage
const universalSearch = new UniversalSearch()

// Open search with Ctrl+Shift+P
document.addEventListener('keydown', (event) => {
  if (event.ctrlKey && event.shiftKey && event.key === 'P') {
    event.preventDefault()
    const searchBox = universalSearch.search('')
    searchBox.focus()
    console.log('Universal search opened')
  }
})

// Perform search
const searchBox = universalSearch.search('comp')
console.log('Search initiated for: comp')
```

## CSS Styling

```css
/* Autocomplete container */
.autocomplete {
  position: relative;
  display: inline-block;
  width: 100%;
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
}

/* Input field */
.autocomplete-input {
  width: 100%;
  padding: 0.75rem;
  border: 2px solid #e2e8f0;
  border-radius: 0.375rem;
  font-size: 1rem;
  line-height: 1.5;
  background-color: #ffffff;
  color: #1f2937;
  transition: all 0.15s ease;
}

.autocomplete-input:focus {
  outline: none;
  border-color: #3b82f6;
  box-shadow: 0 0 0 3px rgba(59, 130, 246, 0.1);
}

.autocomplete-input.is-loading {
  background-image: url('data:image/svg+xml;base64,PHN2ZyB3aWR0aD0iMjAiIGhlaWdodD0iMjAiIHZpZXdCb3g9IjAgMCAyMCAyMCIgZmlsbD0ibm9uZSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIj4KPHBhdGggZD0iTTEwIDNWN0w2IDVMMTAgM1oiIGZpbGw9IiM5Q0EzQUYiLz4KPHBhdGggZD0iTTEwIDEzVjE3TDE0IDE1TDEwIDEzWiIgZmlsbD0iIzlDQTNBRiIvPgo8L3N2Zz4K');
  background-repeat: no-repeat;
  background-position: right 0.75rem center;
  background-size: 1rem;
}

/* Suggestions dropdown */
.autocomplete-suggestions {
  position: absolute;
  top: 100%;
  left: 0;
  right: 0;
  z-index: 1000;
  max-height: 300px;
  overflow-y: auto;
  background: #ffffff;
  border: 1px solid #d1d5db;
  border-radius: 0.375rem;
  box-shadow: 0 4px 6px -1px rgba(0, 0, 0, 0.1), 0 2px 4px -1px rgba(0, 0, 0, 0.06);
  margin-top: 0.25rem;
}

.autocomplete-suggestions.hidden {
  display: none;
}

/* Individual suggestions */
.autocomplete-suggestion {
  display: flex;
  align-items: center;
  padding: 0.75rem;
  cursor: pointer;
  border-bottom: 1px solid #f3f4f6;
  transition: background-color 0.15s ease;
}

.autocomplete-suggestion:last-child {
  border-bottom: none;
}

.autocomplete-suggestion:hover {
  background-color: #f8fafc;
}

.autocomplete-suggestion.highlighted {
  background-color: #3b82f6;
  color: #ffffff;
}

.autocomplete-suggestion.selected {
  background-color: #dbeafe;
  color: #1e40af;
}

.autocomplete-suggestion.selected.highlighted {
  background-color: #2563eb;
  color: #ffffff;
}

/* Suggestion content */
.suggestion-main {
  flex: 1;
  min-width: 0;
}

.suggestion-text {
  font-weight: 500;
  font-size: 0.875rem;
  line-height: 1.25;
  margin-bottom: 0.125rem;
}

.suggestion-description {
  font-size: 0.75rem;
  color: #6b7280;
  line-height: 1.25;
}

.autocomplete-suggestion.highlighted .suggestion-description {
  color: rgba(255, 255, 255, 0.8);
}

.suggestion-category {
  font-size: 0.75rem;
  font-weight: 600;
  color: #9ca3af;
  text-transform: uppercase;
  letter-spacing: 0.025em;
  margin-left: 0.75rem;
  flex-shrink: 0;
}

.autocomplete-suggestion.highlighted .suggestion-category {
  color: rgba(255, 255, 255, 0.7);
}

/* Match highlighting */
.suggestion-match {
  background-color: #fbbf24;
  color: #92400e;
  padding: 0.125rem 0.25rem;
  border-radius: 0.25rem;
  font-weight: 600;
}

.autocomplete-suggestion.highlighted .suggestion-match {
  background-color: rgba(255, 255, 255, 0.3);
  color: #ffffff;
}

/* Multi-select indicators */
.suggestion-checkbox {
  margin-right: 0.75rem;
  width: 1rem;
  height: 1rem;
  border: 2px solid #d1d5db;
  border-radius: 0.25rem;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 0.75rem;
}

.suggestion-checkbox.checked {
  background-color: #3b82f6;
  border-color: #3b82f6;
  color: #ffffff;
}

.suggestion-checkbox.checked::after {
  content: '✓';
}

/* Loading state */
.autocomplete-loading {
  padding: 1rem;
  text-align: center;
  color: #6b7280;
  font-size: 0.875rem;
}

.autocomplete-loading::before {
  content: '';
  display: inline-block;
  width: 1rem;
  height: 1rem;
  margin-right: 0.5rem;
  border: 2px solid #e5e7eb;
  border-top-color: #3b82f6;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  vertical-align: middle;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}

/* Empty state */
.autocomplete-empty {
  padding: 1rem;
  text-align: center;
  color: #9ca3af;
  font-size: 0.875rem;
}

/* Dark theme */
.autocomplete.dark {
  --bg-primary: #1f2937;
  --bg-secondary: #374151;
  --text-primary: #f9fafb;
  --text-secondary: #d1d5db;
  --border-color: #4b5563;
  --highlight-bg: #3b82f6;
  --highlight-text: #ffffff;
}

.autocomplete.dark .autocomplete-input {
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border-color: var(--border-color);
}

.autocomplete.dark .autocomplete-suggestions {
  background-color: var(--bg-primary);
  border-color: var(--border-color);
}

.autocomplete.dark .autocomplete-suggestion {
  border-bottom-color: var(--border-color);
}

.autocomplete.dark .autocomplete-suggestion:hover {
  background-color: var(--bg-secondary);
}

.autocomplete.dark .suggestion-description {
  color: var(--text-secondary);
}

.autocomplete.dark .suggestion-category {
  color: var(--text-secondary);
}

/* Responsive design */
@media (max-width: 768px) {
  .autocomplete-input {
    padding: 1rem;
    font-size: 1rem;
  }
  
  .autocomplete-suggestion {
    padding: 1rem;
  }
  
  .suggestion-category {
    display: none;
  }
  
  .autocomplete-suggestions {
    max-height: 200px;
  }
}

/* Accessibility */
.autocomplete-input:focus-visible {
  outline: 2px solid #3b82f6;
  outline-offset: 2px;
}

.autocomplete-suggestion:focus {
  outline: 2px solid #3b82f6;  
  outline-offset: -2px;
}

@media (prefers-reduced-motion: reduce) {
  .autocomplete-input,
  .autocomplete-suggestion {
    transition: none;
  }
  
  @keyframes spin {
    to {
      transform: none;
    }
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .autocomplete-input {
    border-width: 2px;
    border-color: currentColor;
  }
  
  .autocomplete-suggestion.highlighted {
    outline: 2px solid currentColor;
    outline-offset: -2px;
  }
}

/* Print styles */
@media print {
  .autocomplete-suggestions {
    display: none;
  }
}
```

## Best Practices

### 1. Appropriate Debouncing

```typescript
// ✅ Good - reasonable debounce times for different use cases
const fileSearchAutocomplete = new AutocompleteBuilder('file-search')
  .debounceMs(200)  // Quick for local file search
  .build()

const apiSearchAutocomplete = new AutocompleteBuilder('api-search')
  .debounceMs(500)  // Longer for API calls
  .build()

const instantSearchAutocomplete = new AutocompleteBuilder('instant-search')
  .debounceMs(50)   // Very quick for instant results
  .build()

// ❌ Poor - too aggressive debouncing
const sluggishAutocomplete = new AutocompleteBuilder('sluggish')
  .debounceMs(2000)  // Too slow, poor user experience
  .build()
```

### 2. Smart Filter Selection

```typescript
// ✅ Good - appropriate filters for different data types
const exactMatchAutocomplete = new AutocompleteBuilder('exact')
  .filterMode(FilterMode.StartsWith)  // Good for prefixes
  .build()

const flexibleSearchAutocomplete = new AutocompleteBuilder('flexible')
  .filterMode(FilterMode.Fuzzy)       // Good for typo tolerance
  .fuzzyThreshold(0.6)
  .build()

const containsSearchAutocomplete = new AutocompleteBuilder('contains')
  .filterMode(FilterMode.Contains)    // Good for general search
  .build()

// ❌ Poor - wrong filter for use case
const inefficientAutocomplete = new AutocompleteBuilder('inefficient')
  .filterMode(FilterMode.Fuzzy)       // Overkill for simple prefix matching
  .fuzzyThreshold(0.1)                // Too permissive
  .build()
```

### 3. Reasonable Result Limits

```typescript
// ✅ Good - appropriate limits for context
const mobileAutocomplete = new AutocompleteBuilder('mobile')
  .maxSuggestions(5)           // Fewer for mobile
  .maxVisibleSuggestions(3)    // Limited screen space
  .build()

const desktopAutocomplete = new AutocompleteBuilder('desktop')
  .maxSuggestions(15)          // More for desktop
  .maxVisibleSuggestions(8)    // More screen space
  .build()

// ❌ Poor - too many results overwhelm user
const overwhelmingAutocomplete = new AutocompleteBuilder('overwhelming')
  .maxSuggestions(100)         // Too many options
  .maxVisibleSuggestions(50)   // Scrolling fatigue
  .build()
```

### 4. Proper Error Handling

```typescript
// ✅ Good - comprehensive error handling
const robustAutocomplete = new AutocompleteBuilder('robust')
  .onLoadSuggestions(async (query) => {
    try {
      const response = await fetch(`/api/search?q=${encodeURIComponent(query)}`)
      
      if (!response.ok) {
        throw new Error(`Search failed: ${response.statusText}`)
      }
      
      const data = await response.json()
      return data.results || []
    } catch (error) {
      console.error('Search error:', error)
      // Return fallback suggestions or empty array
      return []
    }
  })
  .build()

// ❌ Poor - no error handling
const fragileAutocomplete = new AutocompleteBuilder('fragile')
  .onLoadSuggestions(async (query) => {
    const response = await fetch(`/api/search?q=${query}`)
    return response.json()  // Can throw, no error handling
  })
  .build()
```

### 5. Accessibility Considerations

```typescript
// ✅ Good - accessible autocomplete
const accessibleAutocomplete = new AutocompleteBuilder('accessible')
  .suggestions([
    { 
      id: '1', 
      text: 'Option 1', 
      value: '1',
      description: 'First option - press Enter to select'
    }
  ])
  .autoSelectFirst(true)       // Clear initial selection
  .showDescriptions(true)      // Helpful context
  .build()

// Handle ARIA attributes and screen readers
const element = document.getElementById('autocomplete-input')
if (element) {
  element.setAttribute('role', 'combobox')
  element.setAttribute('aria-expanded', 'false')
  element.setAttribute('aria-autocomplete', 'list')
}

// ❌ Poor - not accessible
const inaccessibleAutocomplete = new AutocompleteBuilder('inaccessible')
  .suggestions(suggestions)
  .autoSelectFirst(false)      // No clear initial state
  .showDescriptions(false)     // No context for screen readers
  .build()
```

## Related Widgets

- **[Input](./input)** - Basic text input for simple text entry
- **[Select](./select)** - Dropdown selection for predefined options
- **[Menu](./menu)** - Navigation menus with keyboard support
- **[Modal](./modal)** - Modal dialogs for search interfaces

## Examples

- **[Basic Autocomplete](../../examples/basic/autocomplete-basic)** - Simple autocomplete implementations
- **[Async Search](../../examples/advanced/autocomplete-async)** - API-driven search with loading states
- **[Multi-Select Tags](../../examples/advanced/autocomplete-tags)** - Tag input with multi-selection
- **[Command Palette](../../examples/apps/command-palette)** - Universal search interface

The Autocomplete widget provides comprehensive search-as-you-type functionality with advanced filtering, keyboard navigation, multi-select support, and async data loading, making it essential for modern interactive applications.