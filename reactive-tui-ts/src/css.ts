/**
 * CSS utilities and hot reload functionality
 */

export interface CSSRule {
  selector: string
  declarations: Record<string, string>
}

export interface Stylesheet {
  rules: CSSRule[]
  source?: string
}

export class CSSWatcher {
  private watchers: Map<string, Function> = new Map()
  
  async watch(filePath: string, callback: (css: string) => void): Promise<void> {
    // In a real implementation, this would use file system watchers
    console.log(`🎨 Watching CSS file: ${filePath}`)
    
    // For demo purposes, simulate hot reload
    setTimeout(() => {
      console.log('🔥 CSS hot reload triggered!')
      callback('/* Hot reloaded CSS */')
    }, 5000)
  }
  
  unwatch(filePath: string): void {
    this.watchers.delete(filePath)
  }
}

export function parseCSS(css: string): Stylesheet {
  // Simple CSS parser for demo
  // In production, this would use a proper CSS parser
  const rules: CSSRule[] = []
  
  // Very basic regex-based parsing (not production ready!)
  const ruleRegex = /([^{]+)\{([^}]+)\}/g
  let match
  
  while ((match = ruleRegex.exec(css)) !== null) {
    const selector = match[1].trim()
    const declarationsText = match[2].trim()
    
    const declarations: Record<string, string> = {}
    const declRegex = /([^:]+):([^;]+)/g
    let declMatch
    
    while ((declMatch = declRegex.exec(declarationsText)) !== null) {
      const property = declMatch[1].trim()
      const value = declMatch[2].trim()
      declarations[property] = value
    }
    
    rules.push({ selector, declarations })
  }
  
  return { rules, source: css }
}

export const cssWatcher = new CSSWatcher()