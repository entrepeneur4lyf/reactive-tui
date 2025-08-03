# Contributing to Reactive TUI

Thank you for your interest in contributing to Reactive TUI! This document provides guidelines and information for contributors.

## 🚀 Getting Started

### Prerequisites

- **Rust**: >= 1.70.0
- **Node.js**: >= 16.0.0
- **Cargo**: Latest stable
- **Git**: For version control

### Development Setup

1. **Fork the repository**
   ```bash
   # Fork on GitHub, then clone your fork
   git clone https://github.com/YOUR_USERNAME/reactive-tui.git
   cd reactive-tui
   ```

2. **Install dependencies**
   ```bash
   npm install
   ```

3. **Build the project**
   ```bash
   # Development build with debug symbols
   npm run build:debug
   
   # Production build
   npm run build
   ```

4. **Run tests**
   ```bash
   # Run JavaScript/TypeScript tests
   npm test
   
   # Run Rust tests
   cargo test
   
   # Run all tests
   npm test && cargo test
   ```

## 🛠️ Development Workflow

### Branch Naming

- `feature/feature-name` - New features
- `fix/bug-description` - Bug fixes
- `docs/update-description` - Documentation updates
- `refactor/component-name` - Code refactoring
- `test/test-description` - Test additions/improvements

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

```
type(scope): description

feat(widgets): add new DataTable sorting feature
fix(css): resolve flexbox alignment issue
docs(readme): update installation instructions
test(toast): add comprehensive toast notification tests
```

**Types:**
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation only
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding/updating tests
- `chore`: Build process or auxiliary tool changes

## 📝 Code Guidelines

### Rust Code Standards

- **Format with rustfmt**: `cargo fmt`
- **Lint with clippy**: `cargo clippy -- -D warnings`
- **Documentation**: All public APIs must have documentation comments
- **Error handling**: Use `Result<T, TuiError>` for fallible operations
- **Testing**: Write unit tests for new functionality

```rust
/// Creates a new button widget with specified text
/// 
/// # Arguments
/// * `text` - The button label text
/// 
/// # Returns
/// A configured Button widget
/// 
/// # Example
/// ```rust
/// let button = Button::new("Click me")?;
/// button.set_style("padding: 8px; background: blue;")?;
/// ```
pub fn new(text: &str) -> Result<Self, TuiError> {
    // Implementation
}
```

### JavaScript/TypeScript Standards

- **Format with Prettier**: `npm run format`
- **Lint with oxlint**: `npm run lint`
- **Type safety**: Use TypeScript for all new code
- **Testing**: Write tests using AVA framework

### CSS Standards

- **Utility-first**: Follow Tailwind CSS patterns where possible
- **Responsive**: Consider terminal size variations
- **Performance**: Minimize CSS complexity for terminal rendering

## 🧪 Testing Guidelines

### Writing Tests

1. **Unit tests** for individual components
2. **Integration tests** for component interactions
3. **Performance tests** for critical paths
4. **Visual tests** for rendering accuracy

### Test Structure

```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_widget_creation() {
        let widget = Widget::new().expect("Widget creation failed");
        assert_eq!(widget.get_type(), WidgetType::Container);
    }
    
    #[test]
    fn test_widget_styling() {
        let mut widget = Widget::new().unwrap();
        widget.set_style("color: red;").unwrap();
        
        let computed_style = widget.get_computed_style();
        assert_eq!(computed_style.color, Color::Red);
    }
}
```

### Running Specific Tests

```bash
# Run specific Rust test
cargo test test_name

# Run specific JavaScript test
npm test -- --match="*specific test*"

# Run tests with coverage
cargo test --coverage
```

## 📚 Documentation

### Code Documentation

- **Rust**: Use `///` doc comments for public APIs
- **TypeScript**: Use JSDoc comments
- **Examples**: Include usage examples in documentation

### README Updates

- Keep API examples current
- Update feature lists when adding functionality
- Maintain accuracy of performance benchmarks

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Environment**: OS, Node.js version, terminal type
2. **Steps to reproduce**: Minimal example
3. **Expected behavior**: What should happen
4. **Actual behavior**: What actually happens
5. **Code example**: Minimal failing case

### Bug Report Template

```markdown
**Environment:**
- OS: [e.g., Ubuntu 22.04]
- Node.js: [e.g., 18.17.0]
- Terminal: [e.g., GNOME Terminal]
- Reactive TUI: [e.g., 1.0.0]

**Bug Description:**
Brief description of the issue

**Steps to Reproduce:**
1. Create app with...
2. Add widget...
3. Run app...

**Expected Result:**
Widget should display correctly

**Actual Result:**
Widget is not visible

**Code Example:**
```javascript
const app = new JsTuiApp();
// Minimal failing example
```
```

## ✨ Feature Requests

For new features, please:

1. **Check existing issues** to avoid duplicates
2. **Describe the use case** and motivation
3. **Provide examples** of desired API
4. **Consider breaking changes** and migration paths

### Feature Request Template

```markdown
**Feature Description:**
Brief summary of the requested feature

**Use Case:**
Why is this feature needed? What problem does it solve?

**Proposed API:**
```javascript
// Example of how the feature might work
const newWidget = TuiUtils.newFeature(options);
```

**Additional Context:**
Any other relevant information
```

## 🔄 Pull Request Process

### Before Submitting

1. **Create an issue** for discussion (for non-trivial changes)
2. **Fork and branch** from main
3. **Write tests** for new functionality
4. **Update documentation** as needed
5. **Run all tests** and ensure they pass
6. **Format code** with rustfmt and prettier

### Pull Request Template

```markdown
**Description:**
Brief description of changes

**Type of Change:**
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

**Testing:**
- [ ] Unit tests added/updated
- [ ] Integration tests added/updated
- [ ] All tests pass locally

**Checklist:**
- [ ] Code follows style guidelines
- [ ] Self-review completed
- [ ] Documentation updated
- [ ] No breaking changes (or clearly documented)
```

### Review Process

1. **Automated checks** must pass (CI/CD)
2. **Code review** by maintainers
3. **Testing** in multiple environments
4. **Approval** and merge by maintainers

## 🏗️ Architecture Guidelines

### Widget Development

When creating new widgets:

1. **Inherit from base traits**: `Widget`, `Renderable`, `Focusable`
2. **Follow naming conventions**: `PascalCase` for types, `snake_case` for functions
3. **Implement required traits**: `Debug`, `Clone` where appropriate
4. **Add to widget factory**: Update `TuiUtils` if applicable

### Performance Considerations

- **Minimize allocations** in hot paths
- **Use efficient data structures** (Vec, HashMap where appropriate)
- **Profile critical sections** with benchmarks
- **Consider terminal rendering costs**

### FFI Guidelines

When adding JavaScript bindings:

1. **Use NAPI macros** for consistency
2. **Handle errors gracefully** with proper error types
3. **Document TypeScript types** in `index.d.ts`
4. **Test FFI boundaries** thoroughly

## 📋 Code Review Checklist

### For Reviewers

- [ ] Code follows established patterns
- [ ] Tests are comprehensive and pass
- [ ] Documentation is clear and complete
- [ ] Performance impact is acceptable
- [ ] Breaking changes are justified and documented
- [ ] Error handling is robust
- [ ] Memory safety is maintained (Rust)
- [ ] TypeScript types are accurate

### For Contributors

- [ ] Feature/fix is working as expected
- [ ] Code is self-documenting with clear variable names
- [ ] Edge cases are handled
- [ ] Tests cover the changes
- [ ] Documentation is updated
- [ ] Backward compatibility is maintained
- [ ] Performance is not degraded

## 🤝 Community Guidelines

### Code of Conduct

- **Be respectful** and inclusive
- **Provide constructive feedback**
- **Help newcomers** get started
- **Focus on the code**, not the person
- **Assume good intentions**

### Communication

- **GitHub Issues**: Bug reports and feature requests
- **GitHub Discussions**: General questions and community chat
- **Pull Request Reviews**: Code-specific feedback
- **X (@entrepeneur4lyf)**: Project updates and announcements

## 📄 Contributor License Agreement (CLA)

**All contributors must sign the Contributor License Agreement before their contributions can be accepted.**

Please review and agree to the [Contributor License Agreement (CLA)](CLA.md) before submitting your first contribution.

### How to Sign the CLA

Include the following in your pull request description:

```
I hereby agree to the terms of the Contributor License Agreement (CLA) available at:
https://github.com/entrepeneur4lyf/reactive-tui/blob/main/CLA.md

Signed: [Your Full Name] <your.email@example.com>
Date: [YYYY-MM-DD]
```

### Why We Require a CLA

The CLA ensures that:
- You have the legal right to contribute your code
- The project can be distributed under its dual license model
- Both contributors and users are legally protected
- The project can adapt to future licensing needs

### Corporate Contributors

If you're contributing on behalf of your employer, please ensure your company has also signed our Corporate CLA. Contact the maintainers for the Corporate CLA document.

## 📄 License

By contributing to Reactive TUI and signing the CLA, you agree that your contributions will be licensed under the project's dual license (Apache 2.0 / Commercial).

---

**Thank you for contributing to Reactive TUI! 🎉**

Together we're building the future of terminal user interfaces.