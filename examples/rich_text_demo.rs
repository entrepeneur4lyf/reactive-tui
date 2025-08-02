//! Rich Text Demo - Markdown Rendering with Syntax Highlighting
//!
//! This demo showcases the RichText widget with:
//! - Full CommonMark markdown parsing and rendering
//! - Syntax highlighting for 15+ programming languages
//! - Scrolling through large documents
//! - Search functionality with result highlighting
//! - Interactive navigation and content manipulation

use std::time::{Duration, Instant};
use tui_core::{
    error::Result,
    layout::{Layout, LayoutEngine, LayoutRect},
    rendering::Renderer,
    widgets::rich_text::{RichText, RichTextBuilder},
    components::{div, text},
};

/// Sample markdown content for demonstration
const SAMPLE_MARKDOWN: &str = r##"
# Rich Text Renderer Demo

Welcome to the **comprehensive** rich text rendering demonstration! This widget supports full CommonMark markdown with advanced features.

## Core Features

### Markdown Support
- **Bold text** and *italic text*
- `inline code` and code blocks
- Headers at multiple levels
- Lists (ordered and unordered)
- Blockquotes and horizontal rules
- Tables with alignment support

### Syntax Highlighting

The renderer supports syntax highlighting for 15+ languages:

```rust
// Rust example with syntax highlighting
fn main() {
    // Get terminal size dynamically
    let (term_width, term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    let mut rich_text = RichTextBuilder::new("demo")
        .content("# Hello World")
        .syntax_highlighting(true)
        .scrollable(true)
        .build();
    
    println!("Rendered content: {}", rich_text.render_to_string());
}
```

```javascript
// JavaScript example
function createWidget(config) {
    const widget = new RichText(config);
    widget.addEventListener('scroll', handleScroll);
    return widget;
}

const myWidget = createWidget({
    content: "# MyApp",
    syntaxHighlighting: true
});
```

```python
# Python example
def process_markdown(content):
    """Process markdown content with syntax highlighting."""
    renderer = RichTextRenderer()
    return renderer.render(content, highlight=True)

if __name__ == "__main__":
    result = process_markdown("# Hello Python")
    print(result)
```

### Advanced Features

#### Performance Optimizations
1. **Lazy Rendering**: Only visible content is processed
2. **Caching**: Parsed elements are cached for efficiency
3. **Virtual Scrolling**: Handles documents with thousands of lines
4. **Search Indexing**: Fast text search with result highlighting

#### Interactive Capabilities
- Scrolling with keyboard and mouse wheel
- Text search with navigation between results
- Focus management and accessibility support
- Custom styling and theme integration

### Tables Support

| Feature | Status | Language Support |
|---------|---------|------------------|
| Markdown Parsing | ✅ Complete | All |
| Syntax Highlighting | ✅ Complete | 15+ Languages |
| Search | ✅ Complete | All |
| Scrolling | ✅ Complete | All |
| Tables | ✅ Complete | All |

### Code Examples by Language

#### Go Example
```go
package main

import "fmt"

func main() {
    content := "# GoDemo"
    renderer := NewRichTextRenderer()
    result := renderer.Render(content)
    fmt.Println(result)
}
```

#### C++ Example
```cpp
#include <iostream>
#include <string>

class RichTextRenderer {
public:
    std::string render(const std::string& markdown) {
        // Process markdown content
        return process(markdown);
    }
    
private:
    std::string process(const std::string& content) {
        return "Processed: " + content;
    }
};

int main() {
    RichTextRenderer renderer;
    std::cout << renderer.render("# CppDemo") << std::endl;
    return 0;
}
```

### Lists and Quotes

#### Unordered Lists
- Feature-rich markdown rendering
- Multiple programming language support
- Real-time search capabilities
- Efficient scrolling implementation
- Customizable styling options

#### Ordered Lists
1. Initialize the RichText widget
2. Set the markdown content
3. Configure syntax highlighting
4. Enable scrolling and search
5. Render the final output

#### Blockquotes
> "The best way to learn is by doing. This demo provides hands-on experience with rich text rendering in terminal applications."
> 
> — Rich Text Widget Documentation

---

## Performance Metrics

The widget is optimized for performance:

- **Rendering Speed**: < 1ms per frame for typical documents
- **Memory Usage**: O(viewport_size), not O(document_size)
- **Search Performance**: O(n) with result caching
- **Scroll Lag**: Zero thanks to virtual rendering

## Conclusion

This rich text renderer provides comprehensive markdown support with syntax highlighting, making it perfect for documentation viewers, code editors, and content management systems in terminal applications.

Try scrolling through this document, searching for terms, and exploring the interactive features!
"##;

/// Rich text demo application
struct RichTextDemo {
    renderer: Renderer,
    layout_engine: LayoutEngine,
    rich_text: RichText,
    frame_count: u64,
    start_time: Instant,
    search_query: String,
    demo_mode: DemoMode,
}

/// Different demonstration modes
#[derive(Debug, Clone, Copy, PartialEq)]
enum DemoMode {
    Viewing,
    Searching,
    Scrolling,
    LanguageSwitching,
}

impl RichTextDemo {
    /// Create new rich text demo
    async fn new() -> Result<Self> {
        let renderer = Renderer::with_adaptive_fps().await?;

        let layout_engine = LayoutEngine::new();
        
        let rich_text = RichTextBuilder::new("rich-text-demo")
            .content(SAMPLE_MARKDOWN)
            .syntax_highlighting(true)
            .scrollable(true)
            .searchable(true)
            .word_wrap(true)
            .line_numbers(true)
            .on_scroll(|_position| {
                // Handle scroll events (in real app)
            })
            .on_search(|_query, _results| {
                // Handle search events (in real app)
            })
            .build();

        Ok(Self {
            renderer,
            layout_engine,
            rich_text,
            frame_count: 0,
            start_time: Instant::now(),
            search_query: String::new(),
            demo_mode: DemoMode::Viewing,
        })
    }

    /// Create demo layout
    fn create_demo_layout(&mut self) -> Result<Layout> {
        // Refresh terminal dimensions
        self.layout_engine.refresh_dimensions()?;
        let (term_width, term_height) = self.layout_engine.dimensions();
        let state = self.rich_text.state.get();

        // Create the root element tree using responsive layout classes
        let root_element = div()
            .id("rich_text_demo")
            .class("flex")
            .class("flex-col")
            .class("h-full")
            .child_builder(
                div()
                    .class("p-2")
                    .child_builder(text(format!(
                        "Rich Text Demo - Markdown Rendering with Syntax Highlighting\n\
                        ================================================================\n\
                        \n\
                        Document: {} lines | Scroll: {}/{}% | Mode: {:?}\n\
                        Search: '{}' ({} results) | Frame: {}\n\
                        Runtime: {:.1}s | Languages: Rust, JS, Python, Go, C++\n\
                        \n\
                        Controls:\n\
                        - Up/Down: Scroll  - PgUp/PgDn: Page scroll  - Home/End: Jump to edges\n\
                        - /: Search mode  - n/N: Next/prev result  - Esc: Clear search\n\
                        - Tab: Switch demo modes  - Space: Pause auto-demo\n\
                        \n\
                        Widget Features:\n\
                        - Full CommonMark support with headers, lists, code blocks, tables\n\
                        - Syntax highlighting for 15+ programming languages\n\
                        - Efficient scrolling with virtual rendering optimization\n\
                        - Search functionality with result highlighting\n\
                        - Responsive word wrapping and line number display\n\
                        - Theme integration and custom styling support\n\
                        \n\
                        Document Content:",
                        self.rich_text.line_count(),
                        state.scroll_y,
                        if self.rich_text.line_count() > 0 {
                            (state.scroll_y * 100) / self.rich_text.line_count().max(1)
                        } else {
                            0
                        },
                        self.demo_mode,
                        self.search_query,
                        state.search_results.len(),
                        self.frame_count,
                        self.start_time.elapsed().as_secs_f32()
                    )))
            )
            .child_builder(
                div()
                    .class("flex-1")
                    .class("flex")
                    .class("flex-row")
                    .class("p-2")
                    .child_builder(
                        // Main rich text area
                        div()
                            .id("rich_text_widget")
                            .class("flex-1")
                            .focusable(true)
                            .child_builder(text({
                                let layout_rect = LayoutRect {
                                    x: 0,
                                    y: 0,
                                    width: term_width.saturating_sub(22),
                                    height: term_height.saturating_sub(20),
                                };
                                self.rich_text.render(&layout_rect, None)
                            }))
                    )
                    .child_builder(
                        // Statistics panel
                        div()
                            .id("stats_panel")
                            .class("w-1/6")
                            .class("p-1")
                            .child_builder(text(format!(
                                "Statistics:\n\
                                +==============+\n\
                                | Lines: {:>5} |\n\
                                | Scroll: {:>4} |\n\
                                | Search: {:>4} |\n\
                                |              |\n\
                                | Features:    |\n\
                                | - Markdown   |\n\
                                | - Syntax HL  |\n\
                                | - Search     |\n\
                                | - Scroll     |\n\
                                | - Tables     |\n\
                                |              |\n\
                                | Mode:        |\n\
                                | {:>12} |\n\
                                +==============+",
                                self.rich_text.line_count(),
                                state.scroll_y,
                                state.search_results.len(),
                                format!("{:?}", self.demo_mode)
                            )))
                    )
            )
            .build();

        // Use LayoutEngine to compute the responsive layout
        self.layout_engine.compute_layout(&root_element)
    }

    /// Simulate user interactions for demo
    fn simulate_interaction(&mut self) {
        match self.demo_mode {
            DemoMode::Viewing => {
                // Slow scrolling through content
                if self.frame_count % 90 == 0 {
                    self.rich_text.scroll_down(1);
                }

                // Switch to searching after viewing for a while
                if self.frame_count % 600 == 0 {
                    self.demo_mode = DemoMode::Searching;
                }
            }
            DemoMode::Searching => {
                // Perform searches
                if self.frame_count % 180 == 0 {
                    let search_terms = ["rust", "javascript", "python", "markdown", "syntax"];
                    let term = search_terms[(self.frame_count / 180) as usize % search_terms.len()];
                    self.search_query = term.to_string();
                    self.rich_text.search(term);
                }

                // Navigate search results
                if self.frame_count % 60 == 0 {
                    self.rich_text.next_search_result();
                }

                // Switch to scrolling mode
                if self.frame_count % 900 == 0 {
                    self.rich_text.clear_search();
                    self.search_query.clear();
                    self.demo_mode = DemoMode::Scrolling;
                }
            }
            DemoMode::Scrolling => {
                // Fast scrolling demonstration
                if self.frame_count % 15 == 0 {
                    if (self.frame_count / 15) % 2 == 0 {
                        self.rich_text.scroll_down(3);
                    } else {
                        self.rich_text.scroll_up(2);
                    }
                }

                // Switch to language switching mode
                if self.frame_count % 450 == 0 {
                    self.demo_mode = DemoMode::LanguageSwitching;
                }
            }
            DemoMode::LanguageSwitching => {
                // Demonstrate different syntax highlighting
                if self.frame_count % 240 == 0 {
                    let code_examples = [
                        ("rust", "fn main() { println!(\"Hello Rust!\"); }"),
                        (
                            "javascript",
                            "function hello() { console.log('Hello JS!'); }",
                        ),
                        ("python", "def hello():\n    print('Hello Python!')"),
                        ("go", "func main() { fmt.Println(\"Hello Go!\") }"),
                    ];

                    let (lang, code) =
                        &code_examples[(self.frame_count / 240) as usize % code_examples.len()];
                    let content = format!(
                        "# {} Example\n\n```{}\n{}\n```",
                        lang.to_uppercase(),
                        lang,
                        code
                    );
                    self.rich_text.set_content(content);
                }

                // Return to original content and viewing mode
                if self.frame_count % 1200 == 0 {
                    self.rich_text.set_content(SAMPLE_MARKDOWN);
                    self.demo_mode = DemoMode::Viewing;
                }
            }
        }

        // Add new content dynamically
        if self.frame_count % 600 == 0 && self.frame_count > 0 {
            let additional_content = format!(
                "\n\n## Dynamic Content Update #{}\n\nThis content was added at frame {} (runtime: {:.1}s).\n\n```rust\n// Dynamic code example\nlet timestamp = std::time::Instant::now();\nprintln!(\"Added at: {{:?}}\", timestamp);\n```",
                self.frame_count / 600,
                self.frame_count,
                self.start_time.elapsed().as_secs_f32()
            );

            let current_content = self.rich_text.get_content().to_string();
            self.rich_text
                .set_content(current_content + &additional_content);
        }
    }

    /// Update demo state
    fn update(&mut self) {
        self.frame_count += 1;
        self.simulate_interaction();
    }

    /// Run the demo
    async fn run_demo(&mut self) -> Result<()> {
        println!("📖 Starting Rich Text Demo...");
        println!(
            "📄 Document loaded: {} lines with syntax highlighting",
            self.rich_text.line_count()
        );
        println!("🎮 Demonstrating markdown rendering, search, and scrolling...\\n");

        for _ in 0..1200 {
            // Run for 20 seconds at 60fps
            let frame_start = Instant::now();

            // Update state
            self.update();

            // Render frame
            let layout = self.create_demo_layout()?;
            self.renderer.render(&layout).await?;

            // Progress reporting
            if self.frame_count % 60 == 0 {
                let state = self.rich_text.state.get();
                println!(
                    "Frame {}: Mode: {:?} | Scroll: {}/{} | Search: '{}' ({} results)",
                    self.frame_count,
                    self.demo_mode,
                    state.scroll_y,
                    self.rich_text.line_count(),
                    self.search_query,
                    state.search_results.len()
                );
            }

            // Frame timing (60fps)
            let elapsed = frame_start.elapsed();
            let target_duration = Duration::from_millis(16);
            if elapsed < target_duration {
                tokio::time::sleep(target_duration - elapsed).await;
            }
        }

        let final_state = self.rich_text.state.get();
        println!("\\n🏁 Rich Text Demo Complete!");
        println!("📊 Final Statistics:");
        println!("• Document Lines: {}", self.rich_text.line_count());
        println!("• Final Scroll Position: {}", final_state.scroll_y);
        println!("• Search Results: {}", final_state.search_results.len());
        println!("• Runtime: {:.1}s", self.start_time.elapsed().as_secs_f32());
        println!(
            "• Features Demonstrated: Markdown parsing, syntax highlighting, search, scrolling"
        );

        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let mut demo = RichTextDemo::new().await?;
    demo.run_demo().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rich_text_demo_creation() {
        let demo = RichTextDemo::new().await;
        assert!(demo.is_ok());

        let demo = demo.unwrap();
        assert!(demo.rich_text.line_count() > 0);
        assert_eq!(demo.demo_mode, DemoMode::Viewing);
    }

    #[test]
    fn test_sample_markdown_parsing() {
        let mut rich_text = RichTextBuilder::new("test")
            .content(SAMPLE_MARKDOWN)
            .build();

        assert!(rich_text.line_count() > 50);
        assert!(rich_text
            .get_content()
            .contains("# Rich Text Renderer Demo"));
        assert!(rich_text.get_content().contains("```rust"));
    }

    #[test]
    fn test_demo_modes() {
        assert_eq!(DemoMode::Viewing as u8, 0);
        assert_ne!(DemoMode::Searching, DemoMode::Scrolling);
    }
}
