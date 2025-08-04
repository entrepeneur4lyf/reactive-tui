# RichText Widget

The RichText widget provides advanced text rendering with CommonMark markdown support, syntax highlighting for 15+ programming languages, scrolling, search functionality, and hyperlink handling for rich document display in terminal applications.

## Basic Usage

```typescript
import { RichText, RichTextBuilder } from 'reactive-tui';

// Basic rich text display
const richText = new RichText('doc-viewer', `
# Documentation

This is a **markdown** document with:
- Code blocks with syntax highlighting
- [Links](https://example.com)
- Tables and lists

\`\`\`javascript
function hello() {
  console.log('Hello, World!');
}
\`\`\`
`);

// Using the builder pattern
const viewer = new RichTextBuilder('readme-viewer')
  .content(markdownContent)
  .width(100)
  .height(30)
  .wordWrap(true)
  .syntaxHighlighting(true)
  .hyperlinksEnabled(true)
  .build();
```

## Configuration

### RichTextConfig Interface

```typescript
interface RichTextConfig {
  width: number;                    // Display width (default: 80)
  height: number;                   // Display height (default: 25)
  max_width: number;               // Maximum width (default: 120)
  tab_size: number;                // Tab character size (default: 4)
  line_height: number;             // Line height multiplier (default: 1)
  word_wrap: boolean;              // Enable word wrapping (default: true)
  show_line_numbers: boolean;      // Show line numbers (default: false)
  syntax_highlighting: boolean;    // Enable syntax highlighting (default: true)
  hyperlinks_enabled: boolean;     // Enable hyperlink support (default: true)
  scroll_step: number;             // Lines per scroll step (default: 3)
  search_highlight_color: string;  // Search highlight color (default: '#ffff00')
  link_color: string;              // Link color (default: '#0066cc')
  code_background: string;         // Code background color (default: '#f5f5f5')
  table_border_char: string;       // Table border character (default: '│')
  list_bullet_char: string;        // List bullet character (default: '•')
  blockquote_prefix: string;       // Blockquote prefix (default: '│ ')
}
```

### RichTextState Interface

```typescript
interface RichTextState {
  content: string;                 // Raw markdown content
  parsed_elements: MarkdownElement[]; // Parsed markdown elements
  scroll_position: number;         // Current scroll position
  max_scroll: number;             // Maximum scroll position
  viewport_height: number;        // Visible area height
  search_query: string;           // Current search query
  search_results: number[];       // Search result positions
  current_search_index: number;   // Current search result index
  word_wrap: boolean;             // Word wrap state
  show_line_numbers: boolean;     // Line numbers visibility
  syntax_highlighting: boolean;   // Syntax highlighting state
  hyperlinks_enabled: boolean;    // Hyperlinks enabled state
  current_language: SyntaxLanguage | null; // Detected language
  is_loading: boolean;            // Loading state
  last_render_time: number;       // Last render performance
}
```

### Markdown Elements

```typescript
interface MarkdownElement {
  type: 'text' | 'heading' | 'paragraph' | 'code_block' | 'list' | 'table' | 'link' | 'image' | 'blockquote';
  content: string;                // Element content
  level?: number;                 // Heading level (1-6)
  language?: string;              // Code block language
  href?: string;                  // Link URL
  alt?: string;                   // Image alt text
  alignment?: TableAlignment;     // Table alignment
  metadata?: Record<string, any>; // Additional metadata
}
```

### Supported Languages

```typescript
enum SyntaxLanguage {
  JavaScript = 'javascript',
  TypeScript = 'typescript',
  Python = 'python',
  Rust = 'rust',
  Go = 'go',
  Java = 'java',
  CSharp = 'csharp',
  CPlusPlus = 'cpp',
  C = 'c',
  HTML = 'html',
  CSS = 'css',
  JSON = 'json',
  XML = 'xml',
  SQL = 'sql',
  Bash = 'bash',
  Markdown = 'markdown'
}
```

## Core Features

### Markdown Rendering

```typescript
// Automatic markdown parsing and rendering
const richText = new RichText('markdown-viewer', `
# Main Heading
## Sub Heading

This is a paragraph with **bold** and *italic* text.

### Lists
- Item 1
- Item 2
  - Nested item
- Item 3

### Code Blocks
\`\`\`javascript
const greeting = 'Hello, World!';
console.log(greeting);
\`\`\`

### Tables
| Name | Age | City |
|------|-----|------|
| John | 30  | NYC  |
| Jane | 25  | LA   |

### Blockquotes
> This is a blockquote
> with multiple lines

### Links
Visit [OpenAI](https://openai.com) for more info.
`);

// Access parsed elements
const elements = richText.getParsedElements();
console.log('Parsed elements:', elements.length);
```

### Syntax Highlighting

```typescript
// Enable syntax highlighting with language detection
const codeViewer = new RichTextBuilder('code-viewer')
  .content(`
\`\`\`rust
fn main() {
    let greeting = "Hello, Rust!";
    println!("{}", greeting);
}
\`\`\`

\`\`\`python
def hello():
    print("Hello, Python!")

if __name__ == "__main__":
    hello()
\`\`\`
`)
  .syntaxHighlighting(true)
  .showLineNumbers(true)
  .build();

// Language detection callback
const detectionViewer = new RichTextBuilder('detection')
  .content(sourceCode)
  .onLanguageDetect((language, richText) => {
    console.log(`Detected language: ${language}`);
  })
  .build();
```

### Scrolling and Navigation

```typescript
const scrollableViewer = new RichText('scrollable', longContent);

// Scroll methods
scrollableViewer.scrollDown(5);     // Scroll down 5 lines
scrollableViewer.scrollUp(3);       // Scroll up 3 lines
scrollableViewer.pageDown();        // Page down (80% of viewport)
scrollableViewer.pageUp();          // Page up (80% of viewport)
scrollableViewer.scrollToTop();     // Go to beginning
scrollableViewer.scrollToBottom();  // Go to end

// Get scroll information
const position = scrollableViewer.getScrollPosition();
const maxScroll = scrollableViewer.getMaxScroll();
console.log(`Position: ${position}/${maxScroll}`);

// Scroll callback
const callbackViewer = new RichTextBuilder('callback')
  .content(content)
  .onScroll((position, maxScroll, richText) => {
    const percentage = Math.round((position / maxScroll) * 100);
    console.log(`Scrolled to ${percentage}%`);
  })
  .build();
```

### Search Functionality

```typescript
const searchableViewer = new RichText('searchable', documentContent);

// Perform search
const resultCount = searchableViewer.search('function');
console.log(`Found ${resultCount} results`);

// Navigate search results
searchableViewer.nextSearchResult();     // Go to next result
searchableViewer.previousSearchResult(); // Go to previous result

// Get search information
const results = searchableViewer.getSearchResults();
const currentIndex = searchableViewer.getCurrentSearchIndex();
console.log(`Result ${currentIndex + 1} of ${results.length}`);

// Search callback
const searchViewer = new RichTextBuilder('search')
  .content(content)
  .onSearchResult((query, results, richText) => {
    console.log(`Search "${query}" found ${results} matches`);
  })
  .build();
```

### Hyperlink Support

```typescript
const hyperlinkViewer = new RichTextBuilder('hyperlinks')
  .content(`
# Links Example

Visit [GitHub](https://github.com) for repositories.
Check [Stack Overflow](https://stackoverflow.com) for help.
Documentation at [MDN](https://developer.mozilla.org).
`)
  .hyperlinksEnabled(true)
  .onLinkActivate((href, richText) => {
    console.log(`Link activated: ${href}`);
    // Handle link activation (open browser, navigate, etc.)
  })
  .build();
```

## Advanced Configuration

### Custom Styling

```typescript
interface RichTextStyle {
  background_color: string;           // Background color
  text_color: string;                // Default text color
  heading_color: string;             // Heading color
  code_background: string;           // Code block background
  code_foreground: string;           // Code block text color
  link_color: string;                // Link color
  link_hover_color: string;          // Link hover color
  search_highlight_background: string; // Search highlight background
  search_highlight_foreground: string; // Search highlight text
  line_number_color: string;         // Line number color
  table_border_color: string;        // Table border color
  blockquote_color: string;          // Blockquote text color
  list_marker_color: string;         // List bullet color
  syntax_keyword_color: string;      // Syntax: keywords
  syntax_string_color: string;       // Syntax: strings
  syntax_comment_color: string;      // Syntax: comments
  syntax_number_color: string;       // Syntax: numbers
  syntax_function_color: string;     // Syntax: functions
  css_classes: string[];             // Additional CSS classes
}

// Custom styled viewer
const styledViewer = new RichText('styled', content, {
  width: 100,
  height: 40,
  syntax_highlighting: true
}, {
  background_color: '#1e1e1e',
  text_color: '#d4d4d4',
  heading_color: '#569cd6',
  code_background: '#2d2d30',
  code_foreground: '#cccccc',
  link_color: '#4fc1ff',
  syntax_keyword_color: '#c586c0',
  syntax_string_color: '#ce9178',
  syntax_comment_color: '#6a9955',
  syntax_number_color: '#b5cea8',
  syntax_function_color: '#dcdcaa'
});
```

### Event Handling

```typescript
const eventViewer = new RichTextBuilder('events')
  .content(content)
  .onScroll((position, maxScroll, richText) => {
    console.log(`Scroll: ${position}/${maxScroll}`);
  })
  .onLinkActivate((href, richText) => {
    console.log(`Link clicked: ${href}`);
  })
  .onSearchResult((query, results, richText) => {
    console.log(`Search results: ${results} for "${query}"`);
  })
  .onContentChange((content, richText) => {
    console.log('Content updated');
  })
  .onLanguageDetect((language, richText) => {
    console.log(`Language detected: ${language}`);
  })
  .onRenderComplete((renderTime, richText) => {
    console.log(`Rendered in ${renderTime.toFixed(2)}ms`);
  })
  .build();

// Keyboard navigation
eventViewer.handleKeyPress('ArrowDown'); // Scroll down
eventViewer.handleKeyPress('k');         // Vim-style up
eventViewer.handleKeyPress('j');         // Vim-style down
eventViewer.handleKeyPress('PageUp');    // Page up
eventViewer.handleKeyPress('g');         // Go to top
eventViewer.handleKeyPress('G');         // Go to bottom
eventViewer.handleKeyPress('n');         // Next search result
eventViewer.handleKeyPress('N');         // Previous search result
```

## Builder Pattern

```typescript
// Comprehensive builder configuration
const advancedViewer = new RichTextBuilder('advanced')
  .content(markdownContent)
  .width(120)
  .height(50)
  .wordWrap(true)
  .showLineNumbers(true)
  .syntaxHighlighting(true)
  .hyperlinksEnabled(true)
  .tabSize(2)
  .scrollStep(5)
  .onScroll((pos, max, rt) => console.log(`${pos}/${max}`))
  .onLinkActivate((href, rt) => openUrl(href))
  .onSearchResult((query, count, rt) => updateStatus(query, count))
  .build();
```

## Convenience Functions

```typescript
// Pre-configured viewers for common use cases

// Documentation viewer
const docViewer = documentationViewer(`
# API Documentation

## Getting Started
...
`);

// README viewer
const readmeViewer = readmeViewer(readmeContent);

// Code preview with line numbers
const codeViewer = codePreview(`
function fibonacci(n) {
  if (n <= 1) return n;
  return fibonacci(n - 1) + fibonacci(n - 2);
}
`);

// Help text viewer
const helpViewer = helpText(`
# Help

Use the following keyboard shortcuts:
- Arrow keys: Navigate
- Page Up/Down: Page navigation
- g/G: Go to top/bottom
`);
```

## Real-World Examples

### Documentation Browser

```typescript
import { RichText, RichTextBuilder, SyntaxLanguage } from 'reactive-tui';

class DocumentationBrowser {
  private viewer: RichText;
  private currentDocument: string = '';
  private searchHistory: string[] = [];
  
  constructor() {
    this.viewer = new RichTextBuilder('doc-browser')
      .width(120)
      .height(40)
      .wordWrap(true)
      .syntaxHighlighting(true)
      .hyperlinksEnabled(true)
      .showLineNumbers(false)
      .onLinkActivate((href, richText) => this.handleLinkClick(href))
      .onSearchResult((query, results, richText) => this.updateSearchStatus(query, results))
      .onLanguageDetect((language, richText) => this.updateLanguageIndicator(language))
      .build();
  }
  
  loadDocument(title: string, content: string) {
    this.currentDocument = title;
    this.viewer.setContent(content);
    console.log(`Loaded document: ${title}`);
  }
  
  searchDocument(query: string) {
    if (query && !this.searchHistory.includes(query)) {
      this.searchHistory.push(query);
    }
    
    const results = this.viewer.search(query);
    return results;
  }
  
  private handleLinkClick(href: string) {
    if (href.startsWith('#')) {
      // Internal anchor link
      this.scrollToAnchor(href.substring(1));
    } else if (href.startsWith('http')) {
      // External link
      console.log(`Opening external link: ${href}`);
    } else {
      // Relative documentation link
      this.loadRelativeDocument(href);
    }
  }
  
  private scrollToAnchor(anchor: string) {
    // Find heading with matching anchor
    const elements = this.viewer.getParsedElements();
    const headingIndex = elements.findIndex(el => 
      el.type === 'heading' && 
      el.content.toLowerCase().replace(/\s+/g, '-') === anchor.toLowerCase()
    );
    
    if (headingIndex !== -1) {
      // Scroll to heading (simplified)
      this.viewer.scrollToTop();
      this.viewer.scrollDown(headingIndex);
    }
  }
  
  private loadRelativeDocument(path: string) {
    // Load related documentation
    console.log(`Loading related document: ${path}`);
  }
  
  private updateSearchStatus(query: string, results: number) {
    if (results > 0) {
      console.log(`Found ${results} matches for "${query}"`);
    } else {
      console.log(`No matches found for "${query}"`);
    }
  }
  
  private updateLanguageIndicator(language: SyntaxLanguage) {
    console.log(`Syntax highlighting: ${language}`);
  }
  
  // Navigation methods
  nextSearchResult() {
    return this.viewer.nextSearchResult();
  }
  
  previousSearchResult() {
    return this.viewer.previousSearchResult();
  }
  
  scrollToTop() {
    this.viewer.scrollToTop();
  }
  
  scrollToBottom() {
    this.viewer.scrollToBottom();
  }
  
  // Information getters
  getCurrentDocument(): string {
    return this.currentDocument;
  }
  
  getSearchHistory(): string[] {
    return [...this.searchHistory];
  }
  
  getScrollPosition(): { current: number; max: number } {
    return {
      current: this.viewer.getScrollPosition(),
      max: this.viewer.getMaxScroll()
    };
  }
  
  render(): string[] {
    return this.viewer.render();
  }
}

// Usage
const docBrowser = new DocumentationBrowser();

docBrowser.loadDocument('API Reference', `
# API Reference

## Authentication
All API endpoints require authentication via API key.

\`\`\`javascript
const api = new APIClient({
  apiKey: 'your-api-key-here'
});
\`\`\`

## Endpoints

### GET /users
Retrieve user information.

**Parameters:**
- \`id\` (string): User ID
- \`include\` (string[]): Fields to include

**Response:**
\`\`\`json
{
  "id": "user-123",
  "name": "John Doe",
  "email": "john@example.com"
}
\`\`\`

### POST /users
Create a new user.

## Error Handling
The API returns standard HTTP status codes:
- \`200\`: Success
- \`400\`: Bad Request
- \`401\`: Unauthorized
- \`404\`: Not Found
- \`500\`: Internal Server Error
`);

// Search functionality
docBrowser.searchDocument('API');
docBrowser.nextSearchResult();

// Render current view
const displayLines = docBrowser.render();
console.log('Documentation view:', displayLines);
```

### Code Review System

```typescript
class CodeReviewViewer {
  private viewer: RichText;
  private currentFile: string = '';
  private comments: Map<number, string[]> = new Map();
  
  constructor() {
    this.viewer = new RichTextBuilder('code-review')
      .width(140)
      .height(50)
      .wordWrap(false)
      .syntaxHighlighting(true)
      .showLineNumbers(true)
      .hyperlinksEnabled(false)
      .tabSize(2)
      .onLanguageDetect((language, richText) => this.onLanguageDetected(language))
      .onRenderComplete((renderTime, richText) => this.onRenderComplete(renderTime))
      .build();
  }
  
  loadCodeFile(filename: string, content: string) {
    this.currentFile = filename;
    
    // Add file header
    const header = `# Code Review: ${filename}\n\n`;
    const codeBlock = '```' + this.detectLanguageFromFilename(filename) + '\n' + content + '\n```';
    
    this.viewer.setContent(header + codeBlock);
  }
  
  private detectLanguageFromFilename(filename: string): string {
    const ext = filename.split('.').pop()?.toLowerCase();
    const langMap: Record<string, string> = {
      'js': 'javascript',
      'ts': 'typescript',
      'py': 'python',
      'rs': 'rust',
      'go': 'go',
      'java': 'java',
      'cpp': 'cpp',
      'c': 'c',
      'html': 'html',
      'css': 'css',
      'json': 'json',
      'xml': 'xml',
      'sql': 'sql',
      'sh': 'bash',
      'md': 'markdown'
    };
    return langMap[ext || ''] || 'text';
  }
  
  addComment(lineNumber: number, comment: string) {
    if (!this.comments.has(lineNumber)) {
      this.comments.set(lineNumber, []);
    }
    this.comments.get(lineNumber)!.push(comment);
    this.updateViewWithComments();
  }
  
  private updateViewWithComments() {
    const content = this.viewer.getContent();
    const lines = content.split('\n');
    
    // Insert comments at appropriate lines
    let updatedContent = '';
    for (let i = 0; i < lines.length; i++) {
      updatedContent += lines[i] + '\n';
      
      if (this.comments.has(i + 1)) {
        const lineComments = this.comments.get(i + 1)!;
        for (const comment of lineComments) {
          updatedContent += `> **Comment:** ${comment}\n`;
        }
        updatedContent += '\n';
      }
    }
    
    this.viewer.setContent(updatedContent);
  }
  
  searchCode(pattern: string) {
    return this.viewer.search(pattern);
  }
  
  jumpToLine(lineNumber: number) {
    // Scroll to specific line (simplified)
    this.viewer.scrollToTop();
    this.viewer.scrollDown(lineNumber - 1);
  }
  
  private onLanguageDetected(language: SyntaxLanguage) {
    console.log(`Code language detected: ${language}`);
  }
  
  private onRenderComplete(renderTime: number) {
    if (renderTime > 50) {
      console.log(`Slow render: ${renderTime.toFixed(2)}ms`);
    }
  }
  
  // Review navigation
  nextComment() {
    const currentLine = this.viewer.getScrollPosition();
    const commentLines = Array.from(this.comments.keys()).sort((a, b) => a - b);
    const nextLine = commentLines.find(line => line > currentLine);
    
    if (nextLine) {
      this.jumpToLine(nextLine);
      return true;
    }
    return false;
  }
  
  previousComment() {
    const currentLine = this.viewer.getScrollPosition();
    const commentLines = Array.from(this.comments.keys()).sort((a, b) => b - a);
    const prevLine = commentLines.find(line => line < currentLine);
    
    if (prevLine) {
      this.jumpToLine(prevLine);
      return true;
    }
    return false;
  }
  
  getComments(): Map<number, string[]> {
    return new Map(this.comments);
  }
  
  getCurrentFile(): string {
    return this.currentFile;
  }
  
  render(): string[] {
    return this.viewer.render();
  }
}

// Usage
const reviewViewer = new CodeReviewViewer();

reviewViewer.loadCodeFile('auth.ts', `
export interface User {
  id: string;
  email: string;
  name: string;
  role: UserRole;
}

export class AuthService {
  private users: Map<string, User> = new Map();
  
  async authenticate(email: string, password: string): Promise<User | null> {
    // TODO: Implement proper password hashing
    const user = this.findUserByEmail(email);
    if (user && this.verifyPassword(password)) {
      return user;
    }
    return null;
  }
  
  private verifyPassword(password: string): boolean {
    // Placeholder implementation
    return password.length > 6;
  }
}
`);

// Add review comments
reviewViewer.addComment(12, 'Security issue: Passwords should be hashed using bcrypt or similar');
reviewViewer.addComment(20, 'This password validation is too weak');
reviewViewer.addComment(8, 'Consider adding email validation');

// Search for security issues
reviewViewer.searchCode('password');
reviewViewer.nextSearchResult();

// Navigate comments
reviewViewer.nextComment();
```

### Interactive Help System

```typescript
class InteractiveHelperSystem {
  private viewer: RichText;
  private helpSections: Map<string, string> = new Map();
  private currentSection: string = 'overview';
  
  constructor() {
    this.setupHelpSections();
    
    this.viewer = new RichTextBuilder('help-system')
      .width(100)
      .height(30)
      .wordWrap(true)
      .syntaxHighlighting(true)
      .hyperlinksEnabled(true)
      .onLinkActivate((href, richText) => this.navigateToSection(href))
      .onSearchResult((query, results, richText) => this.highlightSearchResults(query, results))
      .build();
    
    this.showSection('overview');
  }
  
  private setupHelpSections() {
    this.helpSections.set('overview', `
# Help System

Welcome to the interactive help system. Navigate using the links below:

## Quick Start
- [Getting Started](#getting-started)
- [Basic Usage](#basic-usage)
- [Advanced Features](#advanced-features)

## Reference
- [Keyboard Shortcuts](#shortcuts)
- [Configuration](#configuration)
- [Troubleshooting](#troubleshooting)

## Examples
- [Simple Example](#simple-example)
- [Complex Example](#complex-example)

---
*Use Ctrl+F to search, or click any link to navigate.*
`);
    
    this.helpSections.set('getting-started', `
# Getting Started

## Installation
\`\`\`bash
npm install reactive-tui
\`\`\`

## First Steps
1. Import the library
2. Create your first widget
3. Add it to your application

\`\`\`typescript
import { RichText } from 'reactive-tui';

const viewer = new RichText('my-viewer', '# Hello, World!');
\`\`\`

[← Back to Overview](#overview) | [Next: Basic Usage →](#basic-usage)
`);
    
    this.helpSections.set('shortcuts', `
# Keyboard Shortcuts

## Navigation
- **↑/↓** or **k/j**: Scroll up/down
- **Page Up/Down**: Page navigation
- **Home/End** or **g/G**: Go to start/end

## Search
- **Ctrl+F**: Start search
- **n**: Next search result
- **N**: Previous search result
- **Esc**: Clear search

## Other
- **Ctrl+C**: Copy selection
- **F1**: Show this help
- **Esc**: Close dialogs

[← Back to Overview](#overview)
`);
    
    this.helpSections.set('troubleshooting', `
# Troubleshooting

## Common Issues

### Performance Problems
- **Symptom**: Slow rendering with large documents
- **Solution**: Enable render caching and limit viewport size

### Search Not Working
- **Symptom**: Search returns no results
- **Solution**: Check for special characters in search query

### Syntax Highlighting Missing
- **Symptom**: Code blocks show no highlighting  
- **Solution**: Verify language is supported and highlighting is enabled

\`\`\`typescript
// Enable syntax highlighting
const viewer = new RichTextBuilder('code')
  .syntaxHighlighting(true)
  .build();
\`\`\`

## Getting Help
- Check the documentation
- Search existing issues
- Create a new issue with examples

[← Back to Overview](#overview)
`);
  }
  
  showSection(sectionId: string) {
    const content = this.helpSections.get(sectionId);
    if (content) {
      this.currentSection = sectionId;
      this.viewer.setContent(content);
      this.viewer.scrollToTop();
    }
  }
  
  private navigateToSection(href: string) {
    // Handle internal navigation
    if (href.startsWith('#')) {
      const sectionId = href.substring(1);
      this.showSection(sectionId);
    } else {
      console.log(`External link: ${href}`);
    }
  }
  
  private highlightSearchResults(query: string, results: number) {
    if (results > 0) {
      console.log(`Help search: "${query}" found ${results} matches`);
    } else {
      console.log(`No help found for: "${query}"`);
    }
  }
  
  searchHelp(query: string): number {
    return this.viewer.search(query);
  }
  
  showQuickHelp(topic: string) {
    const quickHelp: Record<string, string> = {
      'search': 'Use Ctrl+F to search, n/N for next/previous result',
      'navigation': 'Use arrow keys or j/k to scroll, Page Up/Down for pages',
      'links': 'Click links or press Enter on highlighted links to navigate',
      'syntax': 'Code blocks automatically get syntax highlighting'
    };
    
    const help = quickHelp[topic];
    if (help) {
      console.log(`Quick Help - ${topic}: ${help}`);
    }
  }
  
  getCurrentSection(): string {
    return this.currentSection;
  }
  
  getAvailableSections(): string[] {
    return Array.from(this.helpSections.keys());
  }
  
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'F1':
        if (this.currentSection !== 'overview') {
          this.showSection('overview');
        }
        return true;
      case 'Escape':
        if (this.viewer.getSearchResults().length > 0) {
          this.viewer.search(''); // Clear search
        }
        return true;
      default:
        return this.viewer.handleKeyPress(key);
    }
  }
  
  render(): string[] {
    return this.viewer.render();
  }
}

// Usage
const helpSystem = new InteractiveHelperSystem();

// Show specific help section
helpSystem.showSection('shortcuts');

// Search help content
helpSystem.searchHelp('syntax highlighting');

// Quick help for specific topics
helpSystem.showQuickHelp('navigation');

// Handle keyboard input
helpSystem.handleKeyPress('F1'); // Show overview
helpSystem.handleKeyPress('n');  // Next search result
```

## Performance Considerations

```typescript
// Performance monitoring
const performanceViewer = new RichTextBuilder('performance')
  .content(largeDocument)
  .onRenderComplete((renderTime, richText) => {
    if (renderTime > 100) {
      console.warn(`Slow render: ${renderTime.toFixed(2)}ms`);
    }
  })
  .build();

// Optimize for large documents
const optimizedViewer = new RichText('optimized', content, {
  width: 100,        // Reasonable width
  height: 30,        // Limit viewport size
  word_wrap: true    // Enable word wrapping
});

// Use render caching for repeated renders
const cachedViewer = new RichText('cached', content);
const lines1 = cachedViewer.render(); // First render - slow
const lines2 = cachedViewer.render(); // Cached render - fast
```

## Best Practices

1. **Content Management**
   - Use appropriate viewport sizes for performance
   - Enable word wrapping for readability
   - Cache content when possible

2. **Search Optimization**
   - Provide search feedback to users
   - Clear search when not needed
   - Use reasonable search result limits

3. **Language Detection**
   - Specify language explicitly in code blocks
   - Enable syntax highlighting for code content
   - Handle unsupported languages gracefully

4. **Accessibility**
   - Provide keyboard navigation
   - Use semantic markup in markdown
   - Ensure sufficient color contrast

## Integration with Element Builder

```typescript
import { ElementBuilderImpl } from '../components';

const container = new ElementBuilderImpl('div')
  .class('document-container')
  .child(
    new RichTextBuilder('integrated-viewer')
      .content(documentContent)
      .width(100)
      .height(40)
      .build()
  )
  .build();
```

The RichText widget provides comprehensive markdown rendering with syntax highlighting, search functionality, and rich document display capabilities for creating sophisticated text-based interfaces in terminal applications.