# 🎓 Reactive TUI Layout Tutorials

A comprehensive series of 5 tutorials that teach you how to create professional TUI layouts using the Reactive TUI Rust crate. Progress from simple terminal applications to complex IDE-style interfaces.

## 📚 Tutorial Series Overview

### [Tutorial 1: Simple Terminal Layout](./01-simple-terminal/)
**🎯 Beginner Level**
- Basic Element creation and styling
- CSS classes and color theming
- Single-column layouts
- Border and padding styling

**What you'll build:** A styled terminal application with colored text, borders, and basic layout.

```bash
cd tutorials/01-simple-terminal
cargo run
```

### [Tutorial 2: Header/Footer Layout](./02-header-footer/)
**🎯 Beginner-Intermediate Level**
- Bar widget usage (header_bar, footer_bar)
- Three-section vertical layouts
- Professional color schemes
- Status information display

**What you'll build:** A dashboard application with header navigation, main content area, and status footer.

```bash
cd tutorials/02-header-footer
cargo run
```

### [Tutorial 3: Sidebar Layout](./03-sidebar-layout/)
**🎯 Intermediate Level**
- Horizontal grid layouts
- Sidebar navigation patterns
- File manager styling
- Content area management

**What you'll build:** A file manager interface with navigation sidebar and main content grid.

```bash
cd tutorials/03-sidebar-layout
cargo run
```

### [Tutorial 4: Dashboard Grid Layout](./04-dashboard-grid/)
**🎯 Intermediate-Advanced Level**
- Complex multi-column grids
- Dashboard widget design
- Statistics and metrics display
- Data visualization patterns

**What you'll build:** A comprehensive analytics dashboard with multiple widget types and grid layouts.

```bash
cd tutorials/04-dashboard-grid
cargo run
```

### [Tutorial 5: Complex IDE Layout](./05-complex-ide/)
**🎯 Advanced Level**
- Multi-panel IDE interfaces
- Nested grid systems
- Toolbar and menu design
- Professional development environment patterns

**What you'll build:** A full IDE interface with file explorer, editor tabs, terminal panel, and status bar.

```bash
cd tutorials/05-complex-ide
cargo run
```

## 🚀 Getting Started

### Prerequisites
- Rust 1.70+ installed
- Basic familiarity with Rust syntax
- Terminal/command line access

### Quick Start
1. Clone the repository
2. Navigate to any tutorial folder
3. Run `cargo run` to see the example
4. Study the code and README for explanations

### Learning Path
**Recommended order:** Complete tutorials 1-5 in sequence for the best learning experience.

## 🎨 Key Concepts Covered

### Layout Systems
- **Flexbox layouts** - Single direction layouts (row/column)
- **CSS Grid** - Multi-dimensional grid systems
- **Nested layouts** - Complex grid-in-grid patterns
- **Responsive design** - Terminal-aware responsive layouts

### Styling & Theming
- **CSS Classes** - Utility-first styling approach
- **Color themes** - Professional color schemes
- **Border styling** - Various border types and colors
- **Typography** - Font weights, sizes, and families

### Widget Patterns
- **Bar widgets** - Headers, footers, toolbars
- **Navigation** - Sidebars, menus, tabs
- **Content areas** - Main content, panels, editors
- **Status displays** - Metrics, alerts, system status

### Professional Patterns
- **Dashboard layouts** - Analytics and metrics displays
- **File managers** - Explorer-style interfaces
- **IDE interfaces** - Development environment layouts
- **Terminal applications** - Command-line tool interfaces

## 📖 Code Structure

Each tutorial follows a consistent structure:
```
tutorial-name/
├── README.md          # Tutorial explanation and concepts
├── main.rs           # Complete working example
└── Cargo.toml        # Dependencies and configuration
```

## 🛠️ Technologies Used

- **Reactive TUI** - Core TUI framework
- **Crossterm** - Terminal manipulation
- **CSS Grid & Flexbox** - Layout systems
- **Utility CSS** - Tailwind-inspired styling

## 🎯 Learning Outcomes

After completing all tutorials, you'll be able to:

✅ Create professional TUI layouts with CSS Grid and Flexbox  
✅ Design responsive interfaces that adapt to terminal size  
✅ Implement common UI patterns (dashboards, file managers, IDEs)  
✅ Use advanced styling with colors, borders, and typography  
✅ Build complex multi-panel interfaces  
✅ Apply professional design principles to terminal applications  

## 🔗 Additional Resources

- [Reactive TUI Documentation](../README.md)
- [CSS Grid Guide](https://css-tricks.com/snippets/css/complete-guide-grid/)
- [Flexbox Guide](https://css-tricks.com/snippets/css/a-guide-to-flexbox/)
- [Terminal UI Design Principles](https://github.com/rothgar/awesome-tuis)

## 🤝 Contributing

Found an issue or want to improve a tutorial? Contributions are welcome!

1. Fork the repository
2. Create a feature branch
3. Make your improvements
4. Submit a pull request

## 📄 License

These tutorials are part of the Reactive TUI project and follow the same license terms.

---

**Happy coding! 🚀 Build amazing terminal interfaces with Reactive TUI!**
