# Link Widget (TypeScript)

Interactive hyperlink component with comprehensive URL handling and cross-platform click events.

## Overview

The TypeScript Link widget provides flexible, interactive hyperlink functionality for terminal applications with seamless integration across Node.js and browser environments. It supports various URL types, click actions, and visual states with proper accessibility features.

## Features

- **URL Handling**: Local file paths, web URLs, email addresses, and custom protocols
- **Cross-Platform**: Works in both Node.js and browser environments
- **Click Actions**: Open in browser, execute commands, trigger application events
- **State Management**: Normal, hover, active, focused, visited, disabled states
- **Accessibility**: Keyboard navigation and screen reader support
- **Security**: URL validation and safe opening mechanisms

## Basic Usage

### Simple Web Link

```typescript
import { link } from 'tui-bun/widgets';

const docsLink = link("docs-link", (builder) =>
  builder
    .text("Documentation")
    .url("https://docs.example.com")
    .target('browser')
);
```

### Factory Functions

The Link widget provides several convenience factory functions:

```typescript
import { webLink, emailLink, fileLink, appLink, commandLink } from 'tui-bun/widgets';

// Web link
const homepage = webLink("homepage", "https://example.com", "Visit Homepage");

// Email link
const contact = emailLink("contact", "support@example.com", "Contact Support");

// File link
const config = fileLink("config", "./config.json", "📄 Configuration");

// Application link
const settings = appLink("settings", "open_settings", "Settings");

// Command link (use with caution)
const build = commandLink("build", "npm run build", "Build Project");
```

## Configuration

### LinkConfig Interface

```typescript
interface LinkConfig extends BaseWidgetConfig {
  type: 'link';
  text: string;
  url: string;
  target?: LinkTarget;
  decoration?: LinkDecoration;
  altText?: string;
  tooltip?: string;
  onClick?: string;
  showExternalIndicator?: boolean;
  visited?: boolean;
}
```

### Link Targets

```typescript
type LinkTarget = 'browser' | 'application' | 'command' | 'system' | 'custom';
```

### Link States

```typescript
type LinkState = 'normal' | 'hover' | 'active' | 'focused' | 'visited' | 'disabled';
```

### Link Decorations

```typescript
type LinkDecoration = 'none' | 'underline' | 'dotted' | 'dashed' | 'double' | 'overline' | 'strikethrough';
```

## Advanced Usage

### Custom Link Configuration

```typescript
const customLink = link("custom", (builder) =>
  builder
    .text("Advanced Link")
    .url("https://example.com/advanced")
    .target('browser')
    .decoration('dotted')
    .altText("Link to advanced documentation")
    .tooltip("Click to open advanced guide")
    .showExternalIndicator(true)
    .class("custom-link")
    .class("highlight")
);
```

### Email Link with Subject

```typescript
const emailWithSubject = link("support", (builder) =>
  builder
    .text("Get Support")
    .url("mailto:support@example.com?subject=Help%20Request&body=Please%20describe%20your%20issue")
    .target('browser')
    .class("support-link")
);
```

### Application Navigation

```typescript
const navLink = link("profile", (builder) =>
  builder
    .text("My Profile")
    .url("profile")
    .target('application')
    .onClick("navigate_to_profile")
    .showExternalIndicator(false)
    .class("nav-link")
);
```

### Conditional Links

```typescript
const createLink = (user: User) => {
  if (user.isAdmin()) {
    return appLink("admin", "admin_panel", "Admin Panel");
  } else {
    return link("disabled-admin", (builder) =>
      builder
        .text("Admin Panel")
        .url("#")
        .disabled(true)
        .tooltip("Admin access required")
        .decoration('none')
    );
  }
};
```

## Widget Class Usage

### Direct Widget Creation

```typescript
import { LinkWidget, LinkConfig } from 'tui-bun/widgets';

const config: LinkConfig = {
  id: 'my-link',
  type: 'link',
  text: 'Click Me',
  url: 'https://example.com',
  target: 'browser',
  decoration: 'underline',
  showExternalIndicator: true
};

const widget = new LinkWidget(config);
```

### Widget Methods

```typescript
// Get/set state
const currentState = widget.getState();
widget.setState('hover');

// Activate link
try {
  await widget.activate();
  console.log('Link opened successfully');
} catch (error) {
  console.error('Failed to open link:', error);
}

// Validate URL
if (widget.validateUrl()) {
  console.log('URL is valid');
}

// Check URL type
if (widget.isExternalUrl()) {
  console.log('This is an external URL');
}
```

## URL Types and Handling

### Automatic URL Type Detection

```typescript
// Web URLs
"https://example.com"        // Opens in browser
"http://example.com"         // Opens in browser
"ftp://files.example.com"    // Opens in browser

// Email addresses
"user@example.com"           // Opens in email client
"mailto:user@example.com"    // Opens in email client

// File paths
"./config.json"              // Opens with system default
"../README.md"               // Opens with system default
"/etc/hosts"                 // Opens with system default
"file:///path/to/file"       // Opens with system default

// Commands (Node.js only, use with LinkTarget 'command')
"ls -la"                     // Executes command
"code ."                     // Opens VS Code
```

### Cross-Platform URL Opening

#### Node.js Environment

```typescript
// Platform-specific commands are handled automatically
// Windows: cmd /c start
// macOS: open
// Linux: xdg-open

const fileLink = link("local-file", (builder) =>
  builder
    .text("Open Config")
    .url("./config.json")
    .target('system')
);
```

#### Browser Environment

```typescript
// Uses window.open() for URLs
const browserLink = link("external", (builder) =>
  builder
    .text("External Site")
    .url("https://example.com")
    .target('browser')
);
```

## State Management

### Programmatic State Control

```typescript
const widget = new LinkWidget(config);

// Set state
widget.setState('hover');

// Get current state
switch (widget.getState()) {
  case 'normal':
    console.log('Link is normal');
    break;
  case 'visited':
    console.log('Link has been visited');
    break;
  case 'disabled':
    console.log('Link is disabled');
    break;
}

// Automatic state transitions
await widget.activate(); // Sets to 'visited' after successful activation
```

### State-Based Styling

```typescript
const styledLink = link("styled", (builder) =>
  builder
    .text("Interactive Link")
    .url("https://example.com")
    .class("interactive")
    .class("color-primary") // Normal state
);

// CSS classes are automatically applied:
// .link-normal, .link-hover, .link-active, etc.
```

## Async Operations

### Promise-Based Activation

```typescript
const asyncLink = async () => {
  const link = webLink("async", "https://api.example.com", "API Docs");
  
  try {
    await link.activate();
    console.log('Link opened successfully');
  } catch (error) {
    console.error('Failed to open link:', error.message);
  }
};
```

### Error Handling

```typescript
const handleLinkClick = async (linkWidget: LinkWidget) => {
  if (!linkWidget.validateUrl()) {
    console.error('Invalid URL format');
    return;
  }

  try {
    await linkWidget.activate();
  } catch (error) {
    if (error.message.includes('Command execution not allowed')) {
      console.error('Security error: Command execution blocked');
    } else if (error.message.includes('Invalid URL')) {
      console.error('URL validation failed');
    } else {
      console.error('Unexpected error:', error.message);
    }
  }
};
```

## Security Considerations

### URL Validation

```typescript
// Built-in validation
const isValid = widget.validateUrl();

// Custom validation
const validateCustomUrl = (url: string): boolean => {
  // Block potentially dangerous URLs
  const blockedDomains = ['malicious.com', 'spam.net'];
  const hostname = new URL(url).hostname;
  return !blockedDomains.includes(hostname);
};
```

### Command Execution Safety

```typescript
// Safe - won't execute commands
const safeLink = link("safe", (builder) =>
  builder
    .text("Safe Link")
    .url("rm -rf /")  // This won't execute
    .target('browser')  // Will try to open as URL
);

// Dangerous - will execute commands in Node.js
const dangerousLink = link("dangerous", (builder) =>
  builder
    .text("Dangerous Link")
    .url("rm -rf /")
    .target('command')  // This WILL execute!
);
```

### Content Security Policy

```typescript
// Sanitize URLs before use
const sanitizeUrl = (url: string): string => {
  try {
    const parsed = new URL(url);
    // Only allow http, https, mailto protocols
    if (!['http:', 'https:', 'mailto:'].includes(parsed.protocol)) {
      throw new Error('Invalid protocol');
    }
    return parsed.toString();
  } catch {
    return 'about:blank';
  }
};

const secureLink = link("secure", (builder) =>
  builder
    .text("Secure Link")
    .url(sanitizeUrl(userProvidedUrl))
    .target('browser')
);
```

## Accessibility

### Alternative Text and Labels

```typescript
const accessibleLink = link("accessible", (builder) =>
  builder
    .text("Download")
    .url("https://example.com/download.zip")
    .altText("Download the latest version of our software (5.2MB ZIP file)")
    .tooltip("Click to start download")
    .class("download-link")
);
```

### Keyboard Navigation

```typescript
// Links are automatically focusable and support:
// Tab: Navigate to link
// Enter: Activate link
// Space: Activate link

const keyboardLink = link("keyboard", (builder) =>
  builder
    .text("Keyboard Accessible")
    .url("https://example.com")
    .class("focusable")
    .attr("tabindex", "0")
);
```

### Screen Reader Support

```typescript
const screenReaderLink = link("sr", (builder) =>
  builder
    .text("Learn More")
    .url("https://example.com/learn")
    .altText("Learn more about our accessibility features and compliance standards")
    .attr("aria-label", "Learn more about accessibility")
    .attr("role", "link")
);
```

## Visual Styling

### Decoration Styles

```typescript
// Different underline styles
const underlineLink = link("underline", (b) => b.text("Underlined").url("#").decoration('underline'));
const dottedLink = link("dotted", (b) => b.text("Dotted").url("#").decoration('dotted'));
const dashedLink = link("dashed", (b) => b.text("Dashed").url("#").decoration('dashed'));
const doubleLink = link("double", (b) => b.text("Double").url("#").decoration('double'));
```

### State-Based Styling

```typescript
// ANSI escape sequences are applied automatically
// Normal: Default text
// Hover: \x1b[1m (bold)
// Active: \x1b[2m (dim)
// Focused: \x1b[7m (reverse)
// Visited: \x1b[2m (dim)
// Disabled: \x1b[2;37m (dim gray)
```

### External Link Indicators

```typescript
// Automatic external indicators
const externalLink = webLink("external", "https://example.com", "External Site");
// Renders as: "External Site ↗"

// Disable indicators
const noIndicatorLink = link("no-indicator", (builder) =>
  builder
    .text("No Indicator")
    .url("https://example.com")
    .showExternalIndicator(false)
);
```

## Examples

### Navigation Menu

```typescript
import { flexRow } from 'tui-bun';

const navMenu = flexRow()
  .child(appLink("home", "home", "Home"))
  .child(appLink("about", "about", "About"))
  .child(appLink("contact", "contact", "Contact"))
  .child(webLink("blog", "https://blog.example.com", "Blog"))
  .class("nav-menu");
```

### Footer Links

```typescript
import { flexColumn, flexRow, text } from 'tui-bun';

const footer = flexColumn()
  .child(
    flexRow()
      .child(webLink("privacy", "https://example.com/privacy", "Privacy Policy"))
      .child(webLink("terms", "https://example.com/terms", "Terms of Service"))
      .child(emailLink("support", "support@example.com", "Support"))
      .class("footer-links")
  )
  .child(text("© 2025 Example Corp"))
  .class("footer");
```

### Link List with Icons

```typescript
const linkList = flexColumn()
  .child(webLink("docs", "https://docs.example.com", "📚 Documentation"))
  .child(webLink("api", "https://api.example.com", "🔧 API Reference"))
  .child(fileLink("config", "./config.json", "⚙️ Configuration"))
  .child(appLink("settings", "settings", "🎛️ Settings"))
  .class("link-list");
```

### Dynamic Link Creation

```typescript
interface LinkData {
  id: string;
  text: string;
  url: string;
  type: 'web' | 'email' | 'file' | 'app';
}

const createDynamicLink = (data: LinkData) => {
  switch (data.type) {
    case 'web':
      return webLink(data.id, data.url, data.text);
    case 'email':
      return emailLink(data.id, data.url, data.text);
    case 'file':
      return fileLink(data.id, data.url, data.text);
    case 'app':
      return appLink(data.id, data.url, data.text);
    default:
      return link(data.id, (builder) =>
        builder.text(data.text).url(data.url).target('browser')
      );
  }
};
```

### React Integration

```typescript
import React from 'react';
import { link } from 'tui-bun/widgets';

interface LinkComponentProps {
  href: string;
  children: React.ReactNode;
  external?: boolean;
  onClick?: () => void;
}

const LinkComponent: React.FC<LinkComponentProps> = ({ 
  href, 
  children, 
  external = false,
  onClick 
}) => {
  const linkWidget = link(`link-${Date.now()}`, (builder) => {
    let b = builder
      .text(children?.toString() || '')
      .url(href)
      .target(external ? 'browser' : 'application');
    
    if (onClick) {
      b = b.onClick('custom-handler');
    }
    
    return b;
  });

  return <span dangerouslySetInnerHTML={{ __html: linkWidget.render() }} />;
};
```

## Related

- [Button Widget](button.md) - For clickable actions without URLs
- [Base Widget](base-widget.md) - Base widget functionality
- [Widget Factory](../core/widget-factory.md) - Widget creation patterns
- [Layout System](../../../layout.md) - For link positioning
- [TypeScript Integration](../overview.md) - TypeScript-specific features