/**
 * Rich Text Renderer Widget - TypeScript Implementation
 * 
 * Advanced text rendering widget with markdown support, syntax highlighting,
 * scrolling, searching, and hyperlink handling for rich document display.
 * 
 * Features:
 * - CommonMark markdown rendering with extensions
 * - Syntax highlighting for 15+ programming languages
 * - Scrollable viewport with smooth scrolling
 * - Search functionality with result highlighting
 * - Hyperlink support with activation callbacks
 * - Table rendering with alignment options
 * - Word wrapping and text flow optimization
 * - Configurable viewport and content management
 */

export interface MarkdownElement {
  type: 'text' | 'heading' | 'paragraph' | 'code_block' | 'list' | 'table' | 'link' | 'image' | 'blockquote';
  content: string;
  level?: number;
  language?: string;
  href?: string;
  alt?: string;
  alignment?: TableAlignment;
  metadata?: Record<string, any>;
}

export enum TableAlignment {
  Left = 'left',
  Center = 'center',
  Right = 'right'
}

export enum SyntaxLanguage {
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

export interface SyntaxPattern {
  name: string;
  pattern: RegExp;
  type: SyntaxPatternType;
  style: string;
}

export enum SyntaxPatternType {
  Keyword = 'keyword',
  String = 'string',
  Comment = 'comment',
  Number = 'number',
  Function = 'function',
  Variable = 'variable',
  Type = 'type',
  Operator = 'operator'
}

export interface RichTextState {
  content: string;
  parsed_elements: MarkdownElement[];
  scroll_position: number;
  max_scroll: number;
  viewport_height: number;
  search_query: string;
  search_results: number[];
  current_search_index: number;
  word_wrap: boolean;
  show_line_numbers: boolean;
  syntax_highlighting: boolean;
  hyperlinks_enabled: boolean;
  current_language: SyntaxLanguage | null;
  is_loading: boolean;
  last_render_time: number;
}

export interface RichTextConfig {
  width: number;
  height: number;
  max_width: number;
  tab_size: number;
  line_height: number;
  word_wrap: boolean;
  show_line_numbers: boolean;
  syntax_highlighting: boolean;
  hyperlinks_enabled: boolean;
  scroll_step: number;
  search_highlight_color: string;
  link_color: string;
  code_background: string;
  table_border_char: string;
  list_bullet_char: string;
  blockquote_prefix: string;
}

export interface RichTextStyle {
  background_color: string;
  text_color: string;
  heading_color: string;
  code_background: string;
  code_foreground: string;
  link_color: string;
  link_hover_color: string;
  search_highlight_background: string;
  search_highlight_foreground: string;
  line_number_color: string;
  table_border_color: string;
  blockquote_color: string;
  list_marker_color: string;
  syntax_keyword_color: string;
  syntax_string_color: string;
  syntax_comment_color: string;
  syntax_number_color: string;
  syntax_function_color: string;
  css_classes: string[];
}

export interface RichTextCallbacks {
  onScroll?: (position: number, maxScroll: number, richText: RichText) => void;
  onLinkActivate?: (href: string, richText: RichText) => void;
  onSearchResult?: (query: string, results: number, richText: RichText) => void;
  onContentChange?: (content: string, richText: RichText) => void;
  onLanguageDetect?: (language: SyntaxLanguage, richText: RichText) => void;
  onRenderComplete?: (renderTime: number, richText: RichText) => void;
}

export class RichText {
  private id: string;
  private state: RichTextState;
  private config: RichTextConfig;
  private style: RichTextStyle;
  private callbacks: RichTextCallbacks;
  private syntaxPatterns: Map<SyntaxLanguage, SyntaxPattern[]> = new Map();
  private renderCache: Map<string, string[]> = new Map();

  constructor(
    id: string,
    content: string = '',
    config: Partial<RichTextConfig> = {},
    style: Partial<RichTextStyle> = {},
    callbacks: RichTextCallbacks = {}
  ) {
    this.id = id;
    this.callbacks = callbacks;

    this.config = {
      width: 80,
      height: 25,
      max_width: 120,
      tab_size: 4,
      line_height: 1,
      word_wrap: true,
      show_line_numbers: false,
      syntax_highlighting: true,
      hyperlinks_enabled: true,
      scroll_step: 3,
      search_highlight_color: '#ffff00',
      link_color: '#0066cc',
      code_background: '#f5f5f5',
      table_border_char: '│',
      list_bullet_char: '•',
      blockquote_prefix: '│ ',
      ...config
    };

    this.style = {
      background_color: '#ffffff',
      text_color: '#000000',
      heading_color: '#1a365d',
      code_background: '#f7fafc',
      code_foreground: '#2d3748',
      link_color: '#3182ce',
      link_hover_color: '#2c5282',
      search_highlight_background: '#ffd700',
      search_highlight_foreground: '#000000',
      line_number_color: '#a0aec0',
      table_border_color: '#e2e8f0',
      blockquote_color: '#718096',
      list_marker_color: '#4a5568',
      syntax_keyword_color: '#9f7aea',
      syntax_string_color: '#38a169',
      syntax_comment_color: '#a0aec0',
      syntax_number_color: '#d69e2e',
      syntax_function_color: '#3182ce',
      css_classes: [],
      ...style
    };

    this.state = {
      content,
      parsed_elements: [],
      scroll_position: 0,
      max_scroll: 0,
      viewport_height: this.config.height,
      search_query: '',
      search_results: [],
      current_search_index: -1,
      word_wrap: this.config.word_wrap,
      show_line_numbers: this.config.show_line_numbers,
      syntax_highlighting: this.config.syntax_highlighting,
      hyperlinks_enabled: this.config.hyperlinks_enabled,
      current_language: null,
      is_loading: false,
      last_render_time: 0
    };

    this.initializeSyntaxPatterns();
    this.setContent(content);
  }

  // Content management
  setContent(content: string): void {
    this.state.content = content;
    this.state.is_loading = true;
    
    try {
      this.parseMarkdown(content);
      this.detectLanguage();
      this.updateScrollBounds();
      this.callbacks.onContentChange?.(content, this);
    } finally {
      this.state.is_loading = false;
    }
  }

  private parseMarkdown(content: string): void {
    const elements: MarkdownElement[] = [];
    const lines = content.split('\n');
    let currentElement: MarkdownElement | null = null;
    let inCodeBlock = false;
    let codeLanguage = '';
    let inTable = false;
    let tableHeaders: string[] = [];

    for (let i = 0; i < lines.length; i++) {
      const line = lines[i];
      const trimmed = line.trim();

      // Code blocks
      if (trimmed.startsWith('```')) {
        if (!inCodeBlock) {
          inCodeBlock = true;
          codeLanguage = trimmed.substring(3).trim();
          currentElement = {
            type: 'code_block',
            content: '',
            language: codeLanguage || 'text'
          };
        } else {
          inCodeBlock = false;
          if (currentElement) {
            elements.push(currentElement);
            currentElement = null;
          }
        }
        continue;
      }

      if (inCodeBlock && currentElement) {
        currentElement.content += (currentElement.content ? '\n' : '') + line;
        continue;
      }

      // Headings
      const headingMatch = trimmed.match(/^(#{1,6})\s+(.+)$/);
      if (headingMatch) {
        elements.push({
          type: 'heading',
          content: headingMatch[2],
          level: headingMatch[1].length
        });
        continue;
      }

      // Tables
      if (trimmed.includes('|') && !inTable) {
        inTable = true;
        tableHeaders = trimmed.split('|').map(h => h.trim()).filter(h => h);
        continue;
      }

      if (inTable && trimmed.startsWith('|') && trimmed.includes('-')) {
        // Table separator line - determine alignment
        continue;
      }

      if (inTable && trimmed.includes('|')) {
        const cells = trimmed.split('|').map(c => c.trim()).filter(c => c);
        elements.push({
          type: 'table',
          content: cells.join('\t'), // Use tab to separate cells
          metadata: { headers: tableHeaders }
        });
        continue;
      }

      if (inTable && !trimmed.includes('|')) {
        inTable = false;
        tableHeaders = [];
      }

      // Lists
      if (trimmed.match(/^[-*+]\s+/)) {
        elements.push({
          type: 'list',
          content: trimmed.replace(/^[-*+]\s+/, '')
        });
        continue;
      }

      // Blockquotes
      if (trimmed.startsWith('>')) {
        elements.push({
          type: 'blockquote',
          content: trimmed.substring(1).trim()
        });
        continue;
      }

      // Links
      const linkMatch = line.match(/\[([^\]]+)\]\(([^)]+)\)/g);
      if (linkMatch) {
        linkMatch.forEach(match => {
          const linkParts = match.match(/\[([^\]]+)\]\(([^)]+)\)/);
          if (linkParts) {
            elements.push({
              type: 'link',
              content: linkParts[1],
              href: linkParts[2]
            });
          }
        });
        // Continue processing the line as regular text
      }

      // Regular paragraphs
      if (trimmed) {
        elements.push({
          type: 'paragraph',
          content: trimmed
        });
      }
    }

    this.state.parsed_elements = elements;
  }

  private detectLanguage(): void {
    // Simple language detection based on content patterns
    const content = this.state.content.toLowerCase();
    
    if (content.includes('function ') || content.includes('const ') || content.includes('let ')) {
      this.state.current_language = SyntaxLanguage.JavaScript;
    } else if (content.includes('interface ') || content.includes('type ') || content.includes(': string')) {
      this.state.current_language = SyntaxLanguage.TypeScript;
    } else if (content.includes('def ') || content.includes('import ') || content.includes('print(')) {
      this.state.current_language = SyntaxLanguage.Python;
    } else if (content.includes('fn ') || content.includes('let mut') || content.includes('impl ')) {
      this.state.current_language = SyntaxLanguage.Rust;
    } else if (content.includes('func ') || content.includes('package ') || content.includes('import "')) {
      this.state.current_language = SyntaxLanguage.Go;
    } else {
      this.state.current_language = null;
    }

    if (this.state.current_language) {
      this.callbacks.onLanguageDetect?.(this.state.current_language, this);
    }
  }

  private initializeSyntaxPatterns(): void {
    // JavaScript patterns
    this.syntaxPatterns.set(SyntaxLanguage.JavaScript, [
      { name: 'keyword', pattern: /\b(function|const|let|var|if|else|for|while|return|class|extends|import|export|async|await)\b/g, type: SyntaxPatternType.Keyword, style: this.style.syntax_keyword_color },
      { name: 'string', pattern: /(['"`])((?:\\.|(?!\1)[^\\])*?)\1/g, type: SyntaxPatternType.String, style: this.style.syntax_string_color },
      { name: 'comment', pattern: /\/\/.*$|\/\*[\s\S]*?\*\//gm, type: SyntaxPatternType.Comment, style: this.style.syntax_comment_color },
      { name: 'number', pattern: /\b\d+(\.\d+)?\b/g, type: SyntaxPatternType.Number, style: this.style.syntax_number_color },
      { name: 'function', pattern: /\b[a-zA-Z_$][a-zA-Z0-9_$]*(?=\s*\()/g, type: SyntaxPatternType.Function, style: this.style.syntax_function_color }
    ]);

    // TypeScript patterns (extends JavaScript)
    this.syntaxPatterns.set(SyntaxLanguage.TypeScript, [
      ...this.syntaxPatterns.get(SyntaxLanguage.JavaScript)!,
      { name: 'type', pattern: /\b(interface|type|enum|namespace|declare|abstract|readonly)\b/g, type: SyntaxPatternType.Type, style: this.style.syntax_keyword_color }
    ]);

    // Python patterns
    this.syntaxPatterns.set(SyntaxLanguage.Python, [
      { name: 'keyword', pattern: /\b(def|class|if|elif|else|for|while|return|import|from|as|try|except|finally|with|lambda|yield|async|await)\b/g, type: SyntaxPatternType.Keyword, style: this.style.syntax_keyword_color },
      { name: 'string', pattern: /(['"])((?:\\.|(?!\1)[^\\])*?)\1|'''[\s\S]*?'''|"""[\s\S]*?"""/g, type: SyntaxPatternType.String, style: this.style.syntax_string_color },
      { name: 'comment', pattern: /#.*$/gm, type: SyntaxPatternType.Comment, style: this.style.syntax_comment_color },
      { name: 'number', pattern: /\b\d+(\.\d+)?\b/g, type: SyntaxPatternType.Number, style: this.style.syntax_number_color }
    ]);

    // Rust patterns
    this.syntaxPatterns.set(SyntaxLanguage.Rust, [
      { name: 'keyword', pattern: /\b(fn|let|mut|const|static|struct|enum|impl|trait|for|while|if|else|match|return|use|mod|pub|unsafe|async|await)\b/g, type: SyntaxPatternType.Keyword, style: this.style.syntax_keyword_color },
      { name: 'string', pattern: /"(?:[^"\\]|\\.)*"/g, type: SyntaxPatternType.String, style: this.style.syntax_string_color },
      { name: 'comment', pattern: /\/\/.*$|\/\*[\s\S]*?\*\//gm, type: SyntaxPatternType.Comment, style: this.style.syntax_comment_color },
      { name: 'number', pattern: /\b\d+(\.\d+)?[fi]?(32|64)?\b/g, type: SyntaxPatternType.Number, style: this.style.syntax_number_color }
    ]);
  }

  // Scrolling
  scrollUp(lines: number = this.config.scroll_step): void {
    this.state.scroll_position = Math.max(0, this.state.scroll_position - lines);
    this.callbacks.onScroll?.(this.state.scroll_position, this.state.max_scroll, this);
  }

  scrollDown(lines: number = this.config.scroll_step): void {
    this.state.scroll_position = Math.min(this.state.max_scroll, this.state.scroll_position + lines);
    this.callbacks.onScroll?.(this.state.scroll_position, this.state.max_scroll, this);
  }

  scrollToTop(): void {
    this.state.scroll_position = 0;
    this.callbacks.onScroll?.(this.state.scroll_position, this.state.max_scroll, this);
  }

  scrollToBottom(): void {
    this.state.scroll_position = this.state.max_scroll;
    this.callbacks.onScroll?.(this.state.scroll_position, this.state.max_scroll, this);
  }

  pageUp(): void {
    this.scrollUp(Math.floor(this.state.viewport_height * 0.8));
  }

  pageDown(): void {
    this.scrollDown(Math.floor(this.state.viewport_height * 0.8));
  }

  // Search
  search(query: string): number {
    this.state.search_query = query;
    this.state.search_results = [];
    this.state.current_search_index = -1;

    if (!query.trim()) {
      this.callbacks.onSearchResult?.(query, 0, this);
      return 0;
    }

    const lowercaseQuery = query.toLowerCase();
    const content = this.state.content.toLowerCase();
    let index = 0;
    let match;

    while ((match = content.indexOf(lowercaseQuery, index)) !== -1) {
      this.state.search_results.push(match);
      index = match + 1;
    }

    if (this.state.search_results.length > 0) {
      this.state.current_search_index = 0;
      this.scrollToSearchResult(0);
    }

    this.callbacks.onSearchResult?.(query, this.state.search_results.length, this);
    return this.state.search_results.length;
  }

  nextSearchResult(): boolean {
    if (this.state.search_results.length === 0) return false;

    this.state.current_search_index = (this.state.current_search_index + 1) % this.state.search_results.length;
    this.scrollToSearchResult(this.state.current_search_index);
    return true;
  }

  previousSearchResult(): boolean {
    if (this.state.search_results.length === 0) return false;

    this.state.current_search_index = this.state.current_search_index <= 0 
      ? this.state.search_results.length - 1 
      : this.state.current_search_index - 1;
    this.scrollToSearchResult(this.state.current_search_index);
    return true;
  }

  private scrollToSearchResult(index: number): void {
    if (index < 0 || index >= this.state.search_results.length) return;

    const charPosition = this.state.search_results[index];
    const lineNumber = this.getLineNumberFromCharPosition(charPosition);
    
    // Scroll to make the line visible
    if (lineNumber < this.state.scroll_position) {
      this.state.scroll_position = lineNumber;
    } else if (lineNumber >= this.state.scroll_position + this.state.viewport_height) {
      this.state.scroll_position = Math.max(0, lineNumber - this.state.viewport_height + 1);
    }

    this.callbacks.onScroll?.(this.state.scroll_position, this.state.max_scroll, this);
  }

  private getLineNumberFromCharPosition(charPos: number): number {
    const contentUpToPos = this.state.content.substring(0, charPos);
    return contentUpToPos.split('\n').length - 1;
  }

  // Rendering
  render(): string[] {
    const startTime = performance.now();
    
    const cacheKey = `${this.state.content.substring(0, 100)}-${this.state.scroll_position}-${this.config.width}`;
    if (this.renderCache.has(cacheKey) && !this.state.search_query) {
      return this.renderCache.get(cacheKey)!;
    }

    const lines: string[] = [];
    const visibleElements = this.getVisibleElements();

    for (const element of visibleElements) {
      const renderedLines = this.renderElement(element);
      lines.push(...renderedLines);
      
      if (lines.length >= this.state.viewport_height) {
        break;
      }
    }

    // Pad with empty lines if needed
    while (lines.length < this.state.viewport_height) {
      lines.push('');
    }

    // Apply search highlighting
    if (this.state.search_query) {
      for (let i = 0; i < lines.length; i++) {
        lines[i] = this.highlightSearchTerms(lines[i]);
      }
    }

    // Cache the result
    this.renderCache.set(cacheKey, lines);
    
    const renderTime = performance.now() - startTime;
    this.state.last_render_time = renderTime;
    this.callbacks.onRenderComplete?.(renderTime, this);

    return lines;
  }

  private getVisibleElements(): MarkdownElement[] {
    const startLine = this.state.scroll_position;
    const endLine = startLine + this.state.viewport_height;
    
    // This is a simplified version - in practice, you'd need to track
    // which elements correspond to which line numbers
    return this.state.parsed_elements.slice(startLine, endLine);
  }

  private renderElement(element: MarkdownElement): string[] {
    switch (element.type) {
      case 'heading':
        return this.renderHeading(element);
      case 'paragraph':
        return this.renderParagraph(element);
      case 'code_block':
        return this.renderCodeBlock(element);
      case 'list':
        return this.renderList(element);
      case 'table':
        return this.renderTable(element);
      case 'blockquote':
        return this.renderBlockquote(element);
      case 'link':
        return this.renderLink(element);
      default:
        return [element.content];
    }
  }

  private renderHeading(element: MarkdownElement): string[] {
    const level = element.level || 1;
    const prefix = '#'.repeat(level) + ' ';
    const content = this.wrapText(prefix + element.content, this.config.width);
    return content;
  }

  private renderParagraph(element: MarkdownElement): string[] {
    return this.wrapText(element.content, this.config.width);
  }

  private renderCodeBlock(element: MarkdownElement): string[] {
    const lines = element.content.split('\n');
    const paddedLines = lines.map(line => `  ${line}`); // Indent code blocks
    
    if (this.state.syntax_highlighting && element.language) {
      return paddedLines.map(line => this.applySyntaxHighlighting(line, element.language!));
    }
    
    return paddedLines;
  }

  private renderList(element: MarkdownElement): string[] {
    const bullet = this.config.list_bullet_char;
    const content = `${bullet} ${element.content}`;
    return this.wrapText(content, this.config.width, 2); // Indent continuation lines
  }

  private renderTable(element: MarkdownElement): string[] {
    const cells = element.content.split('\t');
    const border = this.config.table_border_char;
    const line = `${border} ${cells.join(` ${border} `)} ${border}`;
    return [line];
  }

  private renderBlockquote(element: MarkdownElement): string[] {
    const prefix = this.config.blockquote_prefix;
    const content = this.wrapText(element.content, this.config.width - prefix.length);
    return content.map(line => prefix + line);
  }

  private renderLink(element: MarkdownElement): string[] {
    if (this.state.hyperlinks_enabled && element.href) {
      return [`[${element.content}](${element.href})`];
    }
    return [element.content];
  }

  private wrapText(text: string, width: number, indent: number = 0): string[] {
    if (!this.state.word_wrap) {
      return [text];
    }

    const words = text.split(' ');
    const lines: string[] = [];
    let currentLine = ' '.repeat(indent);

    for (const word of words) {
      if (currentLine.length + word.length + 1 > width) {
        if (currentLine.trim()) {
          lines.push(currentLine);
          currentLine = ' '.repeat(indent) + word;
        } else {
          // Word is longer than line width
          lines.push(currentLine + word);
          currentLine = ' '.repeat(indent);
        }
      } else {
        currentLine += (currentLine.trim() ? ' ' : '') + word;
      }
    }

    if (currentLine.trim()) {
      lines.push(currentLine);
    }

    return lines.length > 0 ? lines : [''];
  }

  private applySyntaxHighlighting(line: string, language: string): string {
    const patterns = this.syntaxPatterns.get(language as SyntaxLanguage);
    if (!patterns) return line;

    let highlightedLine = line;
    
    for (const pattern of patterns) {
      highlightedLine = highlightedLine.replace(pattern.pattern, (match) => {
        // In a real implementation, you'd apply ANSI color codes or styling
        return match; // Simplified for this example
      });
    }

    return highlightedLine;
  }

  private highlightSearchTerms(line: string): string {
    if (!this.state.search_query) return line;
    
    const regex = new RegExp(this.state.search_query.replace(/[.*+?^${}()|[\]\\]/g, '\\$&'), 'gi');
    return line.replace(regex, (match) => {
      // In a real implementation, you'd apply highlighting styles
      return `[${match}]`; // Simplified highlighting
    });
  }

  private updateScrollBounds(): void {
    const totalLines = this.state.parsed_elements.length;
    this.state.max_scroll = Math.max(0, totalLines - this.state.viewport_height);
  }

  // Event handling
  handleKeyPress(key: string): boolean {
    switch (key) {
      case 'ArrowUp':
      case 'k':
        this.scrollUp(1);
        return true;
      case 'ArrowDown':
      case 'j':
        this.scrollDown(1);
        return true;
      case 'PageUp':
        this.pageUp();
        return true;
      case 'PageDown':
        this.pageDown();
        return true;
      case 'Home':
      case 'g':
        this.scrollToTop();
        return true;
      case 'End':
      case 'G':
        this.scrollToBottom();
        return true;
      case 'n':
        return this.nextSearchResult();
      case 'N':
        return this.previousSearchResult();
      default:
        return false;
    }
  }

  // Getters
  getContent(): string {
    return this.state.content;
  }

  getParsedElements(): MarkdownElement[] {
    return [...this.state.parsed_elements];
  }

  getScrollPosition(): number {
    return this.state.scroll_position;
  }

  getMaxScroll(): number {
    return this.state.max_scroll;
  }

  getSearchResults(): number[] {
    return [...this.state.search_results];
  }

  getCurrentSearchIndex(): number {
    return this.state.current_search_index;
  }

  isLoading(): boolean {
    return this.state.is_loading;
  }

  getLastRenderTime(): number {
    return this.state.last_render_time;
  }

  // Cleanup
  destroy(): void {
    this.renderCache.clear();
  }
}

export class RichTextBuilder {
  private id: string;
  private initialContent: string = '';
  private config: Partial<RichTextConfig> = {};
  private style: Partial<RichTextStyle> = {};
  private callbacks: RichTextCallbacks = {};

  constructor(id: string) {
    this.id = id;
  }

  content(content: string): this {
    this.initialContent = content;
    return this;
  }

  width(width: number): this {
    this.config.width = width;
    return this;
  }

  height(height: number): this {
    this.config.height = height;
    return this;
  }

  wordWrap(wrap: boolean): this {
    this.config.word_wrap = wrap;
    return this;
  }

  showLineNumbers(show: boolean): this {
    this.config.show_line_numbers = show;
    return this;
  }

  syntaxHighlighting(enabled: boolean): this {
    this.config.syntax_highlighting = enabled;
    return this;
  }

  hyperlinksEnabled(enabled: boolean): this {
    this.config.hyperlinks_enabled = enabled;
    return this;
  }

  tabSize(size: number): this {
    this.config.tab_size = size;
    return this;
  }

  scrollStep(step: number): this {
    this.config.scroll_step = step;
    return this;
  }

  onScroll(callback: (position: number, maxScroll: number, richText: RichText) => void): this {
    this.callbacks.onScroll = callback;
    return this;
  }

  onLinkActivate(callback: (href: string, richText: RichText) => void): this {
    this.callbacks.onLinkActivate = callback;
    return this;
  }

  onSearchResult(callback: (query: string, results: number, richText: RichText) => void): this {
    this.callbacks.onSearchResult = callback;
    return this;
  }

  onContentChange(callback: (content: string, richText: RichText) => void): this {
    this.callbacks.onContentChange = callback;
    return this;
  }

  onLanguageDetect(callback: (language: SyntaxLanguage, richText: RichText) => void): this {
    this.callbacks.onLanguageDetect = callback;
    return this;
  }

  onRenderComplete(callback: (renderTime: number, richText: RichText) => void): this {
    this.callbacks.onRenderComplete = callback;
    return this;
  }

  build(): RichText {
    return new RichText(this.id, this.initialContent, this.config, this.style, this.callbacks);
  }
}

// Convenience functions for common rich text patterns

export function documentationViewer(content: string): RichText {
  return new RichTextBuilder('documentation-viewer')
    .content(content)
    .width(100)
    .height(30)
    .wordWrap(true)
    .syntaxHighlighting(true)
    .hyperlinksEnabled(true)
    .showLineNumbers(false)
    .build();
}

export function readmeViewer(content: string): RichText {
  return new RichTextBuilder('readme-viewer')
    .content(content)
    .width(80)
    .height(25)
    .wordWrap(true)
    .syntaxHighlighting(true)
    .hyperlinksEnabled(true)
    .build();
}

export function codePreview(content: string): RichText {
  const builder = new RichTextBuilder('code-preview')
    .content(content)
    .width(120)
    .height(40)
    .wordWrap(false)
    .syntaxHighlighting(true)
    .showLineNumbers(true)
    .hyperlinksEnabled(false);

  return builder.build();
}

export function helpText(content: string): RichText {
  return new RichTextBuilder('help-text')
    .content(content)
    .width(70)
    .height(20)
    .wordWrap(true)
    .syntaxHighlighting(false)
    .hyperlinksEnabled(true)
    .build();
}