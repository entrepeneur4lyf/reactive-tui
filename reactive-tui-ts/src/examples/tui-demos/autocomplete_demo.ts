/**
 * Autocomplete Widget Demo - TypeScript Implementation
 * 
 * Demonstrates the comprehensive Autocomplete widget with:
 * - Search-as-you-type with real-time filtering
 * - Multiple filter modes (contains, starts_with, fuzzy)
 * - Keyboard navigation and selection
 * - Single and multi-select modes
 * - Async suggestion loading
 * - Custom scoring and highlighting
 * - Performance optimizations with debouncing
 */

import {
  Autocomplete, AutocompleteBuilder, AutocompleteSuggestion,
  countryAutocomplete, languageAutocomplete, userAutocomplete,
  FilterMode, SelectionMode
} from '../../packages/tui-bun/src/widgets/autocomplete';

interface DemoStats {
  totalSuggestions: number;
  filteredCount: number;
  selectedCount: number;
  queryLength: number;
  processingTime: number;
}

class AutocompleteDemo {
  private demos: { name: string; autocomplete: Autocomplete; description: string }[] = [];

  constructor() {
    this.setupDemos();
  }

  private setupDemos(): void {
    // Demo 1: Country Autocomplete (StartsWith filter)
    this.demos.push({
      name: "Country Search",
      autocomplete: countryAutocomplete(),
      description: "Search countries with starts-with filtering and descriptions"
    });

    // Demo 2: Programming Language Autocomplete (Contains filter)
    this.demos.push({
      name: "Language Search", 
      autocomplete: languageAutocomplete(),
      description: "Search programming languages with contains filtering and categories"
    });

    // Demo 3: User Autocomplete (Fuzzy filter + Multi-select)
    const users = [
      { id: '1', name: 'John Doe', email: 'john@example.com', role: 'Developer' },
      { id: '2', name: 'Jane Smith', email: 'jane@example.com', role: 'Designer' },
      { id: '3', name: 'Bob Johnson', email: 'bob@example.com', role: 'Manager' },
      { id: '4', name: 'Alice Brown', email: 'alice@example.com', role: 'Developer' },
      { id: '5', name: 'Charlie Wilson', email: 'charlie@example.com', role: 'Tester' },
      { id: '6', name: 'Diana Lee', email: 'diana@example.com', role: 'DevOps' }
    ];
    
    this.demos.push({
      name: "User Search",
      autocomplete: userAutocomplete(users),
      description: "Search users with fuzzy matching and multi-select support"
    });

    // Demo 4: Command Autocomplete with async loading
    this.demos.push({
      name: "Command Search",
      autocomplete: this.createAsyncCommandAutocomplete(),
      description: "Search commands with async loading and custom filtering"
    });

    // Demo 5: Advanced Custom Autocomplete
    this.demos.push({
      name: "Advanced Custom",
      autocomplete: this.createAdvancedAutocomplete(),
      description: "Advanced configuration with custom callbacks and scoring"
    });
  }

  private createAsyncCommandAutocomplete(): Autocomplete {
    const commands = [
      { name: 'git add', description: 'Add files to staging area', category: 'Git' },
      { name: 'git commit', description: 'Commit staged changes', category: 'Git' },
      { name: 'git push', description: 'Push changes to remote', category: 'Git' },
      { name: 'git pull', description: 'Pull changes from remote', category: 'Git' },
      { name: 'npm install', description: 'Install dependencies', category: 'NPM' },
      { name: 'npm run', description: 'Run package script', category: 'NPM' },
      { name: 'npm test', description: 'Run tests', category: 'NPM' },
      { name: 'docker build', description: 'Build Docker image', category: 'Docker' },
      { name: 'docker run', description: 'Run Docker container', category: 'Docker' },
      { name: 'kubectl get', description: 'Get Kubernetes resources', category: 'Kubernetes' },
      { name: 'kubectl apply', description: 'Apply Kubernetes configuration', category: 'Kubernetes' }
    ];

    return new AutocompleteBuilder('async-commands')
      .maxSuggestions(8)
      .minQueryLength(2)
      .debounceMs(200)
      .filterMode(FilterMode.Contains)
      .showDescriptions(true)
      .showCategories(true)
      .onLoadSuggestions(async (query: string) => {
        // Simulate API delay
        await this.sleep(100 + Math.random() * 200);
        
        return commands
          .filter(cmd => 
            cmd.name.toLowerCase().includes(query.toLowerCase()) ||
            cmd.description.toLowerCase().includes(query.toLowerCase())
          )
          .map((cmd, index) => ({
            id: `async-cmd-${index}`,
            text: cmd.name,
            value: cmd.name,
            description: cmd.description,
            category: cmd.category
          }));
      })
      .onQueryChange((query, autocomplete) => {
        console.log(`  🔍 Query changed: "${query}" (loading: ${autocomplete.isLoading()})`);
      })
      .build();
  }

  private createAdvancedAutocomplete(): Autocomplete {
    const programmingTopics: AutocompleteSuggestion[] = [
      { id: '1', text: 'Data Structures', value: 'data-structures', description: 'Arrays, Lists, Trees, Graphs', category: 'Computer Science', score: 0.9 },
      { id: '2', text: 'Algorithms', value: 'algorithms', description: 'Sorting, Searching, Dynamic Programming', category: 'Computer Science', score: 0.95 },
      { id: '3', text: 'Machine Learning', value: 'ml', description: 'Neural Networks, Deep Learning, AI', category: 'AI/ML', score: 0.85 },
      { id: '4', text: 'Web Development', value: 'web-dev', description: 'HTML, CSS, JavaScript, Frameworks', category: 'Frontend', score: 0.8 },
      { id: '5', text: 'Database Design', value: 'db-design', description: 'SQL, NoSQL, Normalization', category: 'Backend', score: 0.75 },
      { id: '6', text: 'System Design', value: 'sys-design', description: 'Scalability, Architecture, Patterns', category: 'Architecture', score: 0.9 },
      { id: '7', text: 'DevOps', value: 'devops', description: 'CI/CD, Docker, Kubernetes, Monitoring', category: 'Operations', score: 0.7 },
      { id: '8', text: 'Security', value: 'security', description: 'Cryptography, Authentication, Best Practices', category: 'Security', score: 0.85 }
    ];

    return new AutocompleteBuilder('advanced-topics')
      .suggestions(programmingTopics)
      .maxSuggestions(6)
      .minQueryLength(1)
      .debounceMs(100)
      .filterMode(FilterMode.Custom)
      .selectionMode(SelectionMode.Multiple)
      .fuzzyThreshold(0.5)
      .showDescriptions(true)
      .showCategories(true)
      .highlightMatches(true)
      .onCustomFilter((query, suggestions) => {
        // Custom scoring algorithm
        return suggestions
          .map(suggestion => {
            const text = suggestion.text.toLowerCase();
            const desc = (suggestion.description || '').toLowerCase();
            const cat = (suggestion.category || '').toLowerCase();
            const q = query.toLowerCase();
            
            let score = 0;
            
            // Exact matches get highest score
            if (text === q) score += 10;
            else if (text.startsWith(q)) score += 8;
            else if (text.includes(q)) score += 5;
            
            // Description matches
            if (desc.includes(q)) score += 3;
            
            // Category matches
            if (cat.includes(q)) score += 2;
            
            // Use original score as base
            score += (suggestion.score || 0) * 2;
            
            return score > 0 ? { ...suggestion, score } : null;
          })
          .filter((s): s is AutocompleteSuggestion => s !== null)
          .sort((a, b) => (b.score || 0) - (a.score || 0));
      })
      .onSelectionChange((selected, _autocomplete) => {
        console.log(`  ✅ Selection changed: ${selected.length} items selected`);
      })
      .build();
  }

  public async runDemo(): Promise<void> {
    console.log("🔍 TUI Bun - Autocomplete Widget Demo");
    console.log("=====================================");
    console.log();
    console.log("This demo showcases the TypeScript Autocomplete widget with:");
    console.log("- Search-as-you-type with multiple filter modes");
    console.log("- Keyboard navigation and selection");
    console.log("- Single and multi-select capabilities");
    console.log("- Async suggestion loading with debouncing");
    console.log("- Custom scoring and filtering algorithms");
    console.log();

    for (let i = 0; i < this.demos.length; i++) {
      const demo = this.demos[i];
      console.log(`${i + 1}. ${demo.name}`);
      console.log(`   ${demo.description}`);
      console.log();
      
      await this.runSingleDemo(demo.autocomplete, demo.name);
      console.log();
    }

    await this.runInteractiveDemo();
    
    console.log("✨ All autocomplete demos completed successfully!");
    console.log();
    this.printSummary();
  }

  private async runSingleDemo(autocomplete: Autocomplete, demoName: string): Promise<void> {
    const queries = this.getTestQueries(demoName);
    
    for (const query of queries) {
      const startTime = performance.now();
      
      autocomplete.setQuery(query);
      
      // Wait for debouncing and async loading
      await this.sleep(250);
      
      const endTime = performance.now();
      const stats = this.getAutocompleteStats(autocomplete, endTime - startTime);
      
      console.log(`  Query: "${query}"`);
      console.log(`    → ${stats.filteredCount} results in ${stats.processingTime.toFixed(1)}ms`);
      
      if (stats.filteredCount > 0) {
        const suggestions = autocomplete.getFilteredSuggestions().slice(0, 3);
        suggestions.forEach((suggestion, index) => {
          const marker = index === 0 ? '▶' : ' ';
          console.log(`    ${marker} ${suggestion.text} ${suggestion.description ? `(${suggestion.description})` : ''}`);
        });
        
        if (autocomplete.getFilteredSuggestions().length > 3) {
          console.log(`    ... and ${autocomplete.getFilteredSuggestions().length - 3} more`);
        }
      }
      
      // Test selection for some queries
      if (stats.filteredCount > 0 && query.length > 2) {
        autocomplete.selectHighlighted();
        const selected = autocomplete.getSelectedSuggestions();
        if (selected.length > 0) {
          console.log(`    ✓ Selected: ${selected[selected.length - 1]}`);
        }
      }
    }
  }

  private getTestQueries(demoName: string): string[] {
    switch (demoName) {
      case "Country Search":
        return ["", "U", "Un", "United", "Fra", "Ger"];
      case "Language Search":
        return ["", "J", "Java", "Script", "Py", "Rust"];
      case "User Search":
        return ["", "J", "Jo", "John", "dev", "@example"];
      case "Command Search":
        return ["", "g", "git", "npm", "docker run"];
      case "Advanced Custom":
        return ["", "A", "Alg", "Data", "ML", "Web"];
      default:
        return ["", "t", "te", "test"];
    }
  }

  private async runInteractiveDemo(): Promise<void> {
    console.log("🎮 Interactive Demo: Keyboard Navigation");
    console.log("-----------------------------------------");
    
    const interactive = languageAutocomplete();
    
    // Setup callbacks to show navigation
    let _navigationLog: string[] = [];
    
    interactive.setQuery("Script");
    await this.sleep(100);
    
    console.log(`Query: "Script" - ${interactive.getFilteredSuggestions().length} results found`);
    
    // Simulate keyboard navigation
    const navigationSequence = [
      { key: 'ArrowDown', action: 'Next' },
      { key: 'ArrowDown', action: 'Next' },
      { key: 'ArrowUp', action: 'Previous' },
      { key: 'Enter', action: 'Select' }
    ];
    
    for (const nav of navigationSequence) {
      const handled = interactive.handleKeyPress(nav.key);
      if (handled) {
        const highlighted = interactive.getHighlightedSuggestion();
        const selected = interactive.getSelectedSuggestions();
        
        console.log(`  ${nav.action}: ${highlighted ? highlighted.text : 'None'} ${selected.length > 0 ? '(Selected)' : ''}`);
      }
      
      await this.sleep(50);
    }
  }

  private getAutocompleteStats(autocomplete: Autocomplete, processingTime: number): DemoStats {
    return {
      totalSuggestions: autocomplete.getTotalSuggestions(),
      filteredCount: autocomplete.getFilteredSuggestions().length,
      selectedCount: autocomplete.getSelectedSuggestions().length,
      queryLength: autocomplete.getQuery().length,
      processingTime
    };
  }

  private sleep(ms: number): Promise<void> {
    return new Promise(resolve => setTimeout(resolve, ms));
  }

  private printSummary(): void {
    console.log("The TypeScript Autocomplete widget demonstrates:");
    console.log("  ✓ Real-time search-as-you-type functionality");
    console.log("  ✓ Multiple filter modes: starts-with, contains, fuzzy, custom");
    console.log("  ✓ Keyboard navigation with arrow keys, Enter, Escape");
    console.log("  ✓ Single and multi-selection modes");
    console.log("  ✓ Debounced input processing for performance");
    console.log("  ✓ Async suggestion loading with loading states");
    console.log("  ✓ Custom scoring and filtering algorithms");
    console.log("  ✓ Configurable appearance and behavior");
    console.log();
    console.log("🎯 Key Performance Features:");
    console.log("  - Debounced queries: 100-200ms delay prevents excessive processing");
    console.log("  - Fuzzy matching: Intelligent scoring with word boundary bonuses");
    console.log("  - Async loading: Non-blocking suggestion fetching");
    console.log("  - Memory efficient: Only processes visible suggestions");
    console.log("  - Customizable: Extensive configuration and callback options");
    console.log();
    console.log("🔧 Configuration Examples:");
    console.log("  - Country search: StartsWith filter, 8 max results, descriptions");
    console.log("  - Language search: Contains filter, categories, auto-select first");
    console.log("  - User search: Fuzzy matching (0.4 threshold), multi-select");
    console.log("  - Command search: Async loading, custom filtering, debounced");
    console.log("  - Advanced: Custom scoring algorithm, multiple selection modes");
  }
}

// Demo execution with error handling
async function runAutocompleteDemo(): Promise<void> {
  try {
    const demo = new AutocompleteDemo();
    await demo.runDemo();
  } catch (error) {
    console.error("❌ Demo failed:", error);
    process.exit(1);
  }
}

// Export demo for module usage
export { AutocompleteDemo, runAutocompleteDemo };

// Run demo if executed directly
if (require.main === module) {
  runAutocompleteDemo();
}