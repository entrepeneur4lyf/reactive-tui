# Image Widget (TypeScript)

Advanced image rendering widget with high-quality terminal display and graceful fallback systems.

## Overview

The TypeScript Image widget provides comprehensive image display capabilities for terminal applications, with seamless integration between browser and Node.js environments. It supports multiple image formats and provides intelligent fallback options for different terminal capabilities.

## Features

- **Multi-Format Support**: PNG, JPEG, GIF, WebP, and BMP image formats
- **Cross-Environment**: Works in both Node.js and browser environments
- **Intelligent Fallbacks**: ASCII art, Unicode blocks, or placeholder text
- **Responsive Scaling**: Automatic image scaling with aspect ratio preservation
- **Async Loading**: Promise-based image loading with proper error handling
- **Terminal Detection**: Automatic capability detection with graceful degradation

## Basic Usage

### Simple Image Display

```typescript
import { image } from 'tui-bun/widgets';

const logo = image("company-logo", (builder) =>
  builder
    .sourceFile("assets/logo.png")
    .width(200)
    .height(100)
    .scaling('fit')
    .fallback('ascii')
);
```

### Factory Functions

The Image widget provides several convenience factory functions:

```typescript
import { logo, icon, diagram, embeddedImage, imageFromUrl } from 'tui-bun/widgets';

// Logo with automatic sizing
const companyLogo = logo("header-logo", "assets/logo.png");

// Fixed-size icon
const settingsIcon = icon("settings-icon", "icons/settings.png", 32);

// Diagram with ASCII fallback
const architectureDiagram = diagram("architecture", "docs/architecture.png");

// Embedded image data
const avatarImage = embeddedImage("avatar", imageBytes, "png");

// Image from URL
const webImage = imageFromUrl("web-img", "https://example.com/image.jpg");
```

## Configuration

### ImageConfig Interface

```typescript
interface ImageConfig extends BaseWidgetConfig {
  type: 'image';
  source: ImageSource;
  width?: number;
  height?: number;
  scaling?: ImageScalingMode;
  alignment?: ImageAlignment;
  fallback?: ImageFallbackMode;
  altText?: string;
}
```

### Image Sources

```typescript
interface ImageSource {
  type: 'file' | 'data' | 'url';
  path?: string;        // For file type
  data?: Uint8Array;    // For data type
  format?: string;      // Format hint for data type
  url?: string;         // For url type
}
```

### Scaling Modes

```typescript
type ImageScalingMode = 'fit' | 'fill' | 'stretch' | 'original';
```

### Fallback Modes

```typescript
type ImageFallbackMode = 'ascii' | 'unicode' | 'placeholder' | 'hide';
```

### Terminal Capabilities

```typescript
type TerminalCapability = 'sixel' | 'basic_color' | 'monochrome';
```

## Advanced Usage

### Custom Configuration

```typescript
const customImage = image("custom", (builder) =>
  builder
    .sourceFile("photo.jpg")
    .width(150)
    .height(100)
    .scaling('fill')
    .alignment('center')
    .fallback('unicode')
    .altText("Profile photo")
    .class("profile-image")
);
```

### Loading from Binary Data

```typescript
const imageData: Uint8Array = await loadImageBytes();
const dataImage = image("data-image", (builder) =>
  builder
    .sourceData(imageData, "jpeg")
    .scaling('fit')
    .fallback('placeholder')
);
```

### Async Image Loading

```typescript
const widget = new ImageWidget(config);
try {
  await widget.loadImage();
  console.log('Image loaded successfully');
} catch (error) {
  console.error('Failed to load image:', error);
}
```

### URL Images

```typescript
const urlImage = image("remote", (builder) =>
  builder
    .sourceUrl("https://example.com/image.png")
    .scaling('fit')
    .fallback('placeholder')
    .altText("Remote image")
);
```

## Cross-Platform Compatibility

### Node.js Environment

```typescript
// File system access
const fileImage = image("local", (builder) =>
  builder
    .sourceFile("./assets/logo.png")
    .scaling('fit')
);

// HTTP requests for URL images
const remoteImage = image("remote", (builder) =>
  builder
    .sourceUrl("https://example.com/image.jpg")
    .scaling('fit')
);
```

### Browser Environment

```typescript
// File input handling
const handleFileInput = async (file: File) => {
  const arrayBuffer = await file.arrayBuffer();
  const imageData = new Uint8Array(arrayBuffer);
  
  const uploadedImage = image("uploaded", (builder) =>
    builder
      .sourceData(imageData, file.type.split('/')[1])
      .scaling('fit')
  );
};

// Canvas integration
const canvasImage = image("canvas", (builder) =>
  builder
    .sourceUrl(canvas.toDataURL())
    .scaling('original')
);
```

## Widget Class Usage

### Direct Widget Creation

```typescript
import { ImageWidget, ImageConfig } from 'tui-bun/widgets';

const config: ImageConfig = {
  id: 'my-image',
  type: 'image',
  source: { type: 'file', path: 'image.png' },
  width: 100,
  height: 50,
  scaling: 'fit',
  fallback: 'ascii',
  altText: 'My image'
};

const widget = new ImageWidget(config);
```

### Widget Methods

```typescript
// Load image asynchronously
await widget.loadImage();

// Set widget bounds
widget.setBounds({ x: 0, y: 0, width: 100, height: 50 });

// Get bounds
const bounds = widget.getBounds();

// Render with layout
const rendered = widget.renderWithLayout(layout, theme);

// Convert to element
const element = widget.toElement();
```

## URL Handling

### File URLs

```typescript
// Local file paths
"./assets/image.png"
"../images/logo.jpg"
"/absolute/path/to/image.gif"

// File protocol URLs
"file:///path/to/image.png"
```

### Web URLs

```typescript
// HTTP/HTTPS URLs
"https://example.com/image.jpg"
"http://example.com/image.png"

// CDN URLs
"https://cdn.example.com/assets/logo.svg"
```

### Data URLs

```typescript
// Data URLs (base64 encoded)
"data:image/png;base64,iVBORw0KGgoAAAANSUhEUgAA..."
"data:image/jpeg;base64,/9j/4AAQSkZJRgABAQEAYABgAAD..."
```

## Image Processing

### Scaling and Sizing

```typescript
// Maintain aspect ratio within bounds
.scaling('fit')

// Fill bounds exactly (may crop)
.scaling('fill')

// Stretch to exact dimensions (may distort)
.scaling('stretch')

// Use original size
.scaling('original')
```

### Alignment Options

```typescript
// Align to start
.alignment('start')

// Center alignment (default)
.alignment('center')

// Align to end
.alignment('end')
```

## Fallback Rendering

### ASCII Art Conversion

```typescript
const asciiImage = image("ascii", (builder) =>
  builder
    .sourceFile("image.png")
    .fallback('ascii')
    .altText("ASCII art representation")
);
```

### Unicode Block Characters

```typescript
const blockImage = image("blocks", (builder) =>
  builder
    .sourceFile("image.png")
    .fallback('unicode')
    .altText("Unicode block representation")
);
```

### Placeholder Text

```typescript
const placeholderImage = image("placeholder", (builder) =>
  builder
    .sourceFile("missing.png")
    .fallback('placeholder')
    .altText("Image could not be displayed")
);
```

## Error Handling

### Loading Errors

```typescript
try {
  await widget.loadImage();
} catch (error) {
  if (error.message.includes('Failed to load')) {
    console.error('Network or file system error:', error);
  } else if (error.message.includes('Invalid format')) {
    console.error('Unsupported image format:', error);
  }
}
```

### Validation Errors

```typescript
// Check if widget configuration is valid
if (!widget.validate()) {
  console.error('Invalid widget configuration');
}

// Validate specific aspects
if (!widget.validateSpecific()) {
  console.error('Image source validation failed');
}
```

## Accessibility

### Alternative Text

```typescript
const accessibleImage = image("chart", (builder) =>
  builder
    .sourceFile("sales-chart.png")
    .altText("Q4 sales performance showing 15% growth over Q3")
    .class("chart-image")
);
```

### Screen Reader Support

```typescript
// Provide descriptive alt text
.altText("Bar chart showing monthly revenue from January to December")

// Use semantic classes
.class("data-visualization")
.class("financial-chart")
```

## Performance Optimization

### Lazy Loading

```typescript
// Images are loaded only when needed
const lazyImage = image("lazy", (builder) =>
  builder
    .sourceFile("large-image.png")
    .scaling('fit')
);

// Load when widget becomes visible
await lazyImage.loadImage();
```

### Caching

```typescript
// Rendered output is cached automatically
const cachedWidget = new ImageWidget(config);
await cachedWidget.loadImage();

// Subsequent renders use cached data
const output1 = cachedWidget.renderWithLayout(layout);
const output2 = cachedWidget.renderWithLayout(layout); // Uses cache
```

## Examples

### Image Gallery

```typescript
import { flexRow, image } from 'tui-bun';

const gallery = flexRow()
  .child(image("img1", (b) => b.sourceFile("1.jpg").width(100)))
  .child(image("img2", (b) => b.sourceFile("2.jpg").width(100)))
  .child(image("img3", (b) => b.sourceFile("3.jpg").width(100)))
  .class("gallery");
```

### Hero Section

```typescript
import { flexColumn, image, text } from 'tui-bun';

const hero = flexColumn()
  .child(
    image("hero-bg", (builder) =>
      builder
        .sourceFile("hero-background.jpg")
        .scaling('fill')
        .fallback('hide')
        .class("hero-background")
    )
  )
  .child(text("Welcome to Our App").class("hero-title"))
  .class("hero-section");
```

### Responsive Logo

```typescript
const responsiveLogo = image("logo", (builder) =>
  builder
    .sourceFile("logo.png")
    .scaling('fit')
    .fallback('ascii')
    .altText("Company Logo")
    .class("responsive-logo")
    .class("header-brand")
);
```

### Profile Avatar

```typescript
const profileAvatar = (imageData: Uint8Array) =>
  image("avatar", (builder) =>
    builder
      .sourceData(imageData, "png")
      .width(64)
      .height(64)
      .scaling('fill')
      .fallback('placeholder')
      .altText("User profile picture")
      .class("avatar")
      .class("rounded")
  );
```

## Integration with React/Vue

### React Component

```typescript
import React from 'react';
import { image } from 'tui-bun/widgets';

interface ImageComponentProps {
  src: string;
  alt: string;
  width?: number;
  height?: number;
}

const ImageComponent: React.FC<ImageComponentProps> = ({ src, alt, width, height }) => {
  const imageWidget = image(`img-${Date.now()}`, (builder) => {
    let b = builder.sourceFile(src).altText(alt).scaling('fit');
    if (width) b = b.width(width);
    if (height) b = b.height(height);
    return b;
  });

  return <div dangerouslySetInnerHTML={{ __html: imageWidget.render() }} />;
};
```

### Vue Component

```vue
<template>
  <div v-html="renderedImage"></div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import { image } from 'tui-bun/widgets';

interface Props {
  src: string;
  alt: string;
  width?: number;
  height?: number;
}

const props = defineProps<Props>();

const renderedImage = computed(() => {
  const imageWidget = image(`img-${Date.now()}`, (builder) => {
    let b = builder.sourceFile(props.src).altText(props.alt).scaling('fit');
    if (props.width) b = b.width(props.width);
    if (props.height) b = b.height(props.height);
    return b;
  });
  
  return imageWidget.render();
});
</script>
```

## Related

- [Layout System](../../../layout.md) - For responsive image placement
- [Base Widget](base-widget.md) - Base widget functionality
- [Widget Factory](../core/widget-factory.md) - Widget creation patterns
- [TypeScript Integration](../overview.md) - TypeScript-specific features