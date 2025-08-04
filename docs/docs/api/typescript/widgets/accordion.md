# Accordion Widget

Expandable and collapsible sections with animation support, keyboard navigation, and comprehensive state management for building organized content interfaces.

## Overview

The Accordion widget provides a complete expandable/collapsible section system with smooth animations, multi-expand modes, keyboard navigation, and rich customization options for creating FAQ sections, settings panels, and organized content displays.

```typescript
import { createAccordion } from 'reactive-tui-ts'

const settingsAccordion = createAccordion({
  id: 'settings-accordion',
  sections: [
    {
      id: 'general',
      title: 'General Settings',
      content: 'Configure general application preferences and behavior.',
      icon: '⚙️',
      expanded: true
    },
    {
      id: 'privacy',
      title: 'Privacy & Security', 
      content: 'Manage your privacy settings and security options.',
      icon: '🔒',
      badge: 'Important'
    }
  ],
  config: {
    multiExpand: true,
    animated: true
  }
})
```

## Types

### AnimationEasing

```typescript
enum AnimationEasing {
  Linear = 'linear',
  EaseIn = 'ease-in',
  EaseOut = 'ease-out',
  EaseInOut = 'ease-in-out',
  EaseInBack = 'ease-in-back',
  EaseOutBack = 'ease-out-back',
  EaseInOutBack = 'ease-in-out-back'
}
```

### AnimationState

```typescript
enum AnimationState {
  Collapsed = 'collapsed',
  Expanding = 'expanding',
  Expanded = 'expanded',
  Collapsing = 'collapsing'
}
```

### AccordionSection

```typescript
interface AccordionSection {
  id: string
  title: string
  description?: string
  content: string
  expanded: boolean
  disabled: boolean
  icon?: string
  badge?: string
  cssClasses: string[]
  style: AccordionSectionStyle
}
```

### AccordionConfig

```typescript
interface AccordionConfig {
  multiExpand: boolean
  animated: boolean
  animation: AccordionAnimation
  bordered: boolean
  rounded: boolean
  compact: boolean
  collapsible: boolean
  showIcons: boolean
  collapsedIcon: string
  expandedIcon: string
}
```

### AccordionAnimation

```typescript
interface AccordionAnimation {
  enabled: boolean
  duration: number
  easing: AnimationEasing
  staggerDelay: number
}
```

### AccordionCallbacks

```typescript
interface AccordionCallbacks {
  onExpand?: (sectionId: string) => void
  onCollapse?: (sectionId: string) => void
  onChange?: (expandedSections: string[]) => void
  onFocus?: (sectionId: string) => void
  onSectionClick?: (sectionId: string) => void
}
```

## Basic Usage

### Simple Accordion

```typescript
import { createAccordion, Accordion } from 'reactive-tui-ts'

const basicAccordion = createAccordion({
  id: 'basic-accordion',
  sections: [
    {
      id: 'section1',
      title: 'First Section',
      content: 'This is the content of the first section.\nIt can span multiple lines.'
    },
    {
      id: 'section2', 
      title: 'Second Section',
      content: 'Content for the second section goes here.'
    },
    {
      id: 'section3',
      title: 'Third Section',
      content: 'The third section contains this information.'
    }
  ]
})

// Expand a section
basicAccordion.expandSection('section1')

// Collapse a section
basicAccordion.collapseSection('section1')

// Toggle a section
basicAccordion.toggleSection('section2')

// Get expanded sections
const expanded = basicAccordion.getExpandedSections()
console.log('Expanded sections:', expanded)
```

### Multi-Expand Accordion

```typescript
const multiExpandAccordion = createAccordion({
  id: 'multi-expand',
  sections: [
    { id: 'intro', title: 'Introduction', content: 'Welcome to our service' },
    { id: 'features', title: 'Features', content: 'List of available features' },
    { id: 'pricing', title: 'Pricing', content: 'Pricing information' }
  ],
  config: {
    multiExpand: true,     // Allow multiple sections open
    animated: true,
    bordered: true
  }
})

// Expand multiple sections at once
multiExpandAccordion.expandAll()

// Collapse all sections
multiExpandAccordion.collapseAll()

// Check if specific section is expanded
const isExpanded = multiExpandAccordion.isSectionExpanded('features')
console.log('Features section expanded:', isExpanded)
```

### Single-Expand Accordion (FAQ Style)

```typescript
const faqAccordion = createAccordion({
  id: 'faq',
  sections: [
    {
      id: 'q1',
      title: 'How do I get started?',
      content: 'Getting started is easy! Simply create an account and follow our setup guide.'
    },
    {
      id: 'q2',
      title: 'What are the system requirements?',
      content: 'Our system works on all modern browsers and requires JavaScript to be enabled.'
    },
    {
      id: 'q3',
      title: 'How much does it cost?',
      content: 'We offer flexible pricing plans starting from $9.99/month.'
    }
  ],
  config: {
    multiExpand: false,    // Only one section can be open at a time
    animated: true,
    collapsedIcon: '❓',
    expandedIcon: '✅'
  }
})
```

## Advanced Configuration

### Animated Accordion with Custom Easing

```typescript
import { AnimationEasing } from 'reactive-tui-ts'

const animatedAccordion = createAccordion({
  id: 'animated-accordion',
  sections: [
    { id: 'anim1', title: 'Smooth Animation', content: 'Watch this expand smoothly' },
    { id: 'anim2', title: 'Custom Easing', content: 'This uses bounce animation' }
  ],
  config: {
    animated: true,
    animation: {
      enabled: true,
      duration: 500,
      easing: AnimationEasing.EaseOutBack,
      staggerDelay: 100
    },
    showIcons: true,
    collapsedIcon: '▶',
    expandedIcon: '▼'
  }
})
```

### Styled Accordion with Icons and Badges

```typescript
const styledAccordion = createAccordion({
  id: 'styled-accordion',
  sections: [
    {
      id: 'account',
      title: 'Account Settings',
      description: 'Manage your account preferences',
      content: 'Update your profile, change password, and manage notifications.',
      icon: '👤',
      expanded: true
    },
    {
      id: 'billing',
      title: 'Billing Information',
      description: 'Payment methods and invoices',
      content: 'View billing history, update payment methods, and download invoices.',
      icon: '💳',
      badge: 'Updated'
    },
    {
      id: 'security',
      title: 'Security Settings',
      description: 'Two-factor authentication and security logs',
      content: 'Enable 2FA, view login history, and manage security preferences.',
      icon: '🔐',
      badge: '2',
      disabled: false
    }
  ],
  config: {
    multiExpand: true,
    bordered: true,
    rounded: true,
    animated: true
  },
  cssClasses: ['premium-accordion', 'settings-panel']
})
```

### Compact Accordion

```typescript
const compactAccordion = createAccordion({
  id: 'compact-accordion',
  sections: [
    { id: 'quick1', title: 'Quick Item 1', content: 'Brief content here' },
    { id: 'quick2', title: 'Quick Item 2', content: 'Another brief item' },
    { id: 'quick3', title: 'Quick Item 3', content: 'Last quick item' }
  ],
  config: {
    compact: true,         // Minimal spacing
    bordered: false,       // No borders
    animated: false,       // No animations for speed
    showIcons: true,
    collapsedIcon: '▸',
    expandedIcon: '▾'
  }
})
```

## Section Management

### Dynamic Section Operations

```typescript
const dynamicAccordion = createAccordion({
  id: 'dynamic-accordion',
  sections: [
    { id: 'static1', title: 'Static Section', content: 'This section stays' }
  ]
})

// Add new sections dynamically
dynamicAccordion.addSection({
  id: 'dynamic1',
  title: 'Dynamically Added Section',
  content: 'This section was added at runtime',
  icon: '✨',
  expanded: false
})

dynamicAccordion.addSection({
  id: 'dynamic2',
  title: 'Another Dynamic Section',
  content: 'Yet another section added dynamically',
  badge: 'New'
})

// Remove a section
const removedSection = dynamicAccordion.removeSection('dynamic1')
if (removedSection) {
  console.log(`Removed section: ${removedSection.title}`)
}

// Update section content
dynamicAccordion.updateSectionContent('dynamic2', 'Updated content goes here')

// Get section details
const section = dynamicAccordion.getSection('static1')
console.log('Section:', section?.title)

// Get section count
console.log('Total sections:', dynamicAccordion.sectionCount())
```

### Section State Management

```typescript
const stateAccordion = createAccordion({
  id: 'state-management',
  sections: [
    { id: 'state1', title: 'Section 1', content: 'Content 1' },
    { id: 'state2', title: 'Section 2', content: 'Content 2' },
    { id: 'state3', title: 'Section 3', content: 'Content 3' }
  ]
})

// Check accordion state
console.log('Is disabled:', stateAccordion.isDisabled())
console.log('Expanded sections:', stateAccordion.getExpandedSections())

// Disable entire accordion
stateAccordion.setDisabled(true)

// Enable accordion
stateAccordion.setDisabled(false)

// Get specific section
const targetSection = stateAccordion.getSection('state2')
if (targetSection) {
  console.log(`Section "${targetSection.title}" is ${targetSection.expanded ? 'expanded' : 'collapsed'}`)
}
```

## Keyboard Navigation

### Focus Management

```typescript
const navAccordion = createAccordion({
  id: 'navigation-accordion',
  sections: [
    { id: 'nav1', title: 'First Item', content: 'First content' },
    { id: 'nav2', title: 'Second Item', content: 'Second content' },
    { id: 'nav3', title: 'Third Item', content: 'Third content' },
    { id: 'nav4', title: 'Fourth Item', content: 'Fourth content' }
  ]
})

// Focus navigation
navAccordion.focusFirst()          // Focus first section
navAccordion.focusLast()           // Focus last section
navAccordion.focusNext()           // Move focus to next section
navAccordion.focusPrevious()       // Move focus to previous section

// Direct focus
navAccordion.focusSection('nav3')  // Focus specific section

// Get current focus
const focusedSection = navAccordion.getFocusedSection()
console.log('Currently focused:', focusedSection)

// Keyboard navigation would typically be handled like this:
const handleKeyDown = (event: KeyboardEvent) => {
  switch (event.key) {
    case 'ArrowDown':
      navAccordion.focusNext()
      break
    case 'ArrowUp':
      navAccordion.focusPrevious()
      break
    case 'Home':
      navAccordion.focusFirst()
      break
    case 'End':
      navAccordion.focusLast()
      break
    case 'Enter':
    case ' ':
      const focused = navAccordion.getFocusedSection()
      if (focused) {
        navAccordion.toggleSection(focused)
      }
      break
  }
}
```

## Event Handling

### Comprehensive Event Callbacks

```typescript
const eventAccordion = createAccordion({
  id: 'event-accordion',
  sections: [
    { id: 'event1', title: 'Event Section 1', content: 'Content with events' },
    { id: 'event2', title: 'Event Section 2', content: 'More event content' }
  ],
  callbacks: {
    onExpand: (sectionId) => {
      console.log(`Section expanded: ${sectionId}`)
      
      // Load content dynamically when expanded
      if (sectionId === 'event2') {
        eventAccordion.updateSectionContent(sectionId, 'Dynamically loaded content!')
      }
    },
    
    onCollapse: (sectionId) => {
      console.log(`Section collapsed: ${sectionId}`)
      
      // Save state or cleanup when collapsed
      const section = eventAccordion.getSection(sectionId)
      if (section) {
        console.log(`Saved state for: ${section.title}`)
      }
    },
    
    onChange: (expandedSections) => {
      console.log(`Accordion state changed. Expanded: [${expandedSections.join(', ')}]`)
      
      // Update URL or save state
      const state = { expanded: expandedSections }
      localStorage.setItem('accordionState', JSON.stringify(state))
    },
    
    onFocus: (sectionId) => {
      console.log(`Section focused: ${sectionId}`)
      
      // Update accessibility indicators
      const section = eventAccordion.getSection(sectionId)
      if (section) {
        console.log(`Focused on: ${section.title}`)
      }
    },
    
    onSectionClick: (sectionId) => {
      console.log(`Section clicked: ${sectionId}`)
      
      // Custom behavior on click
      const section = eventAccordion.getSection(sectionId)
      if (section?.disabled) {
        console.log('Section is disabled, showing tooltip')
      }
    }
  }
})

// Trigger events
eventAccordion.expandSection('event1')  // Will trigger onExpand and onChange
eventAccordion.focusSection('event2')   // Will trigger onFocus
```

## Pre-built Accordion Types

### Settings Accordion

```typescript
import { createSettingsAccordion } from 'reactive-tui-ts'

const settingsAccordion = createSettingsAccordion([
  {
    id: 'appearance',
    title: 'Appearance',
    content: 'Theme: Dark\nFont Size: 14px\nColor Scheme: Blue'
  },
  {
    id: 'notifications',
    title: 'Notifications',
    content: 'Email: Enabled\nPush: Disabled\nSound: Enabled'
  },
  {
    id: 'privacy',
    title: 'Privacy',
    content: 'Profile Visibility: Friends Only\nData Sharing: Disabled'
  }
])

// Settings accordion has multiExpand: true by default
settingsAccordion.expandAll()
```

### FAQ Accordion

```typescript
import { createFaqAccordion } from 'reactive-tui-ts'

const faqAccordion = createFaqAccordion([
  {
    id: 'faq1',
    question: 'How do I reset my password?',
    answer: 'Click on "Forgot Password" on the login page and follow the instructions sent to your email.'
  },
  {
    id: 'faq2', 
    question: 'Can I change my username?',
    answer: 'Yes, you can change your username in Account Settings. Note that this can only be done once every 30 days.'
  },
  {
    id: 'faq3',
    question: 'How do I delete my account?',
    answer: 'Account deletion can be requested from the Privacy Settings page. This action is permanent and cannot be undone.'
  }
])

// FAQ accordion has multiExpand: false and special icons
```

### Compact Accordion

```typescript
import { createCompactAccordion } from 'reactive-tui-ts'

const compactAccordion = createCompactAccordion([
  {
    id: 'compact1',
    title: 'Quick Access 1',
    content: 'Minimal content display'
  },
  {
    id: 'compact2',
    title: 'Quick Access 2',
    content: 'Another minimal item'
  }
])

// Compact accordion has no borders, no animations, minimal spacing
```

## Real-World Examples

### Documentation Browser

```typescript
import { createAccordion, AnimationEasing } from 'reactive-tui-ts'

class DocumentationBrowser {
  private accordion: any
  private searchTerm: string = ''

  constructor() {
    this.setupDocumentation()
  }

  private setupDocumentation() {
    this.accordion = createAccordion({
      id: 'docs-browser',
      sections: [
        {
          id: 'getting-started',
          title: 'Getting Started',
          description: 'Basic setup and installation',
          icon: '🚀',
          content: this.getGettingStartedContent(),
          expanded: true
        },
        {
          id: 'api-reference',
          title: 'API Reference',
          description: 'Complete API documentation',
          icon: '📚',
          content: 'Loading API documentation...',
          badge: 'Updated'
        },
        {
          id: 'examples',
          title: 'Code Examples',
          description: 'Working examples and tutorials',
          icon: '💡',
          content: 'Browse our collection of examples'
        },
        {
          id: 'troubleshooting',
          title: 'Troubleshooting',
          description: 'Common issues and solutions',
          icon: '🔧',
          content: 'Find solutions to common problems'
        },
        {
          id: 'changelog',
          title: 'Changelog',
          description: 'Version history and updates',
          icon: '📝',
          content: 'View recent changes and updates'
        }
      ],
      config: {
        multiExpand: true,
        animated: true,
        animation: {
          enabled: true,
          duration: 350,
          easing: AnimationEasing.EaseOutBack,
          staggerDelay: 75
        },
        bordered: true,
        showIcons: true
      },
      callbacks: {
        onExpand: (sectionId) => {
          this.loadSectionContent(sectionId)
        },
        onChange: (expandedSections) => {
          this.updateUrl(expandedSections)
        },
        onFocus: (sectionId) => {
          this.highlightSection(sectionId)
        }
      },
      cssClasses: ['documentation-browser', 'help-system']
    })
  }

  private getGettingStartedContent(): string {
    return `Welcome to our documentation!

Installation:
1. npm install reactive-tui-ts
2. Import the components you need
3. Start building your interface

Quick Start:
- Check out the examples section
- Review the API reference
- Join our community Discord`
  }

  private async loadSectionContent(sectionId: string) {
    console.log(`Loading content for section: ${sectionId}`)
    
    // Simulate loading content
    setTimeout(() => {
      let content = ''
      
      switch (sectionId) {
        case 'api-reference':
          content = this.generateApiReference()
          break
        case 'examples':
          content = this.generateExamples()
          break
        case 'troubleshooting':
          content = this.generateTroubleshooting()
          break
        case 'changelog':
          content = this.generateChangelog()
          break
        default:
          return
      }
      
      this.accordion.updateSectionContent(sectionId, content)
    }, 500)
  }

  private generateApiReference(): string {
    return `API Reference

Core Components:
- createAccordion(options)
- createTabs(config)
- createModal(settings)

Widget Properties:
- id: string (required)
- sections: AccordionSection[]
- config: AccordionConfig
- callbacks: AccordionCallbacks

Methods:
- expandSection(id): boolean
- collapseSection(id): boolean
- toggleSection(id): boolean
- addSection(section): void
- removeSection(id): AccordionSection | null`
  }

  private generateExamples(): string {
    return `Code Examples

Basic Accordion:
const accordion = createAccordion({
  id: 'example',
  sections: [...]
})

Settings Panel:
const settings = createSettingsAccordion([
  { id: 'general', title: 'General', content: '...' }
])

FAQ Section:
const faq = createFaqAccordion([
  { id: 'q1', question: '...', answer: '...' }
])`
  }

  private generateTroubleshooting(): string {
    return `Common Issues

Problem: Accordion not expanding
Solution: Check if multiExpand is set correctly

Problem: Animations not working
Solution: Ensure animated: true in config

Problem: Keyboard navigation issues
Solution: Verify focus management is enabled

Problem: Content not updating
Solution: Use updateSectionContent() method`
  }

  private generateChangelog(): string {
    return `Recent Changes

v2.1.0 - Latest
- Added animation easing options
- Improved keyboard navigation
- Fixed focus management bugs
- Added new pre-built accordion types

v2.0.0
- Major API restructure
- Added TypeScript support
- New event callback system
- Performance improvements

v1.5.2
- Bug fixes and stability improvements
- Updated documentation`
  }

  private updateUrl(expandedSections: string[]) {
    const params = new URLSearchParams()
    if (expandedSections.length > 0) {
      params.set('sections', expandedSections.join(','))
    }
    
    const newUrl = `${window.location.pathname}?${params.toString()}`
    window.history.replaceState({}, '', newUrl)
  }

  private highlightSection(sectionId: string) {
    console.log(`Highlighting section: ${sectionId}`)
    // Visual highlighting logic would go here
  }

  search(term: string) {
    this.searchTerm = term.toLowerCase()
    
    // Filter sections based on search term
    for (let i = 0; i < this.accordion.sectionCount(); i++) {
      const section = this.accordion.sections[i]
      const matches = section.title.toLowerCase().includes(this.searchTerm) ||
                     section.content.toLowerCase().includes(this.searchTerm)
      
      if (matches && this.searchTerm.length > 0) {
        // Expand matching sections
        this.accordion.expandSection(section.id)
      }
    }
  }

  expandAllSections() {
    this.accordion.expandAll()
  }

  collapseAllSections() {
    this.accordion.collapseAll()
  }

  exportConfiguration() {
    return {
      expanded: this.accordion.getExpandedSections(),
      focused: this.accordion.getFocusedSection(),
      searchTerm: this.searchTerm
    }
  }

  importConfiguration(config: any) {
    // Restore expanded sections
    this.accordion.collapseAll()
    config.expanded?.forEach((sectionId: string) => {
      this.accordion.expandSection(sectionId)
    })
    
    // Restore focus
    if (config.focused) {
      this.accordion.focusSection(config.focused)
    }
    
    // Restore search
    if (config.searchTerm) {
      this.search(config.searchTerm)
    }
  }

  render() {
    return this.accordion.render()
  }
}

// Usage
const docsBrowser = new DocumentationBrowser()

// Search functionality
docsBrowser.search('API')

// Export/import state
const state = docsBrowser.exportConfiguration()
localStorage.setItem('docsState', JSON.stringify(state))

// Later...
const savedState = JSON.parse(localStorage.getItem('docsState') || '{}')
docsBrowser.importConfiguration(savedState)

console.log(docsBrowser.render())
```

### Settings Management Panel

```typescript
import { createAccordion, AnimationEasing } from 'reactive-tui-ts'

class ApplicationSettings {
  private settingsAccordion: any
  private settings: Record<string, any> = {}

  constructor() {
    this.loadSettings()
    this.setupSettingsAccordion()
  }

  private loadSettings() {
    // Load settings from storage or API
    this.settings = {
      appearance: {
        theme: 'dark',
        fontSize: 14,
        colorScheme: 'blue'
      },
      notifications: {
        email: true,
        push: false,
        sound: true,
        frequency: 'daily'
      },
      privacy: {
        profileVisibility: 'friends',
        dataSharing: false,
        analytics: true
      },
      advanced: {
        debugMode: false,
        experimentalFeatures: false,
        autoSave: true,
        backupFrequency: 'weekly'
      }
    }
  }

  private setupSettingsAccordion() {
    this.settingsAccordion = createAccordion({
      id: 'app-settings',
      sections: [
        {
          id: 'appearance',
          title: 'Appearance & Display',
          description: 'Customize the look and feel',
          icon: '🎨',
          content: this.generateAppearanceContent(),
          expanded: true
        },
        {
          id: 'notifications',
          title: 'Notifications',
          description: 'Manage notification preferences',
          icon: '🔔',
          content: this.generateNotificationsContent(),
          badge: this.getNotificationsBadge()
        },
        {
          id: 'privacy',
          title: 'Privacy & Security',
          description: 'Control your privacy settings',
          icon: '🔒',
          content: this.generatePrivacyContent(),
          badge: this.settings.privacy.dataSharing ? undefined : 'Secure'
        },
        {
          id: 'advanced',
          title: 'Advanced Settings',
          description: 'Developer and advanced options',
          icon: '⚙️',
          content: this.generateAdvancedContent(),
          badge: this.settings.advanced.debugMode ? 'Debug' : undefined
        }
      ],
      config: {
        multiExpand: true,
        animated: true,
        animation: {
          enabled: true,
          duration: 300,
          easing: AnimationEasing.EaseInOut,
          staggerDelay: 50
        },
        bordered: true,
        rounded: true
      },
      callbacks: {
        onExpand: (sectionId) => {
          console.log(`Opened settings section: ${sectionId}`)
          this.trackSettingsAccess(sectionId)
        },
        onChange: (expandedSections) => {
          this.saveUIState({ expandedSections })
        }
      },
      cssClasses: ['settings-panel', 'application-config']
    })
  }

  private generateAppearanceContent(): string {
    const { theme, fontSize, colorScheme } = this.settings.appearance
    
    return `Current Settings:
• Theme: ${theme} ${theme === 'dark' ? '🌙' : '☀️'}
• Font Size: ${fontSize}px
• Color Scheme: ${colorScheme}

Available Themes:
- Light Theme (Default)
- Dark Theme (Current)
- Auto (System)

Font Sizes: 12px, 14px, 16px, 18px
Color Schemes: Blue, Green, Purple, Orange`
  }

  private generateNotificationsContent(): string {
    const { email, push, sound, frequency } = this.settings.notifications
    
    return `Notification Types:
• Email Notifications: ${email ? 'Enabled ✅' : 'Disabled ❌'}
• Push Notifications: ${push ? 'Enabled ✅' : 'Disabled ❌'}
• Sound Alerts: ${sound ? 'Enabled 🔊' : 'Disabled 🔇'}

Frequency: ${frequency}
Options: Real-time, Hourly, Daily, Weekly

Do Not Disturb:
- Quiet hours: 10 PM - 7 AM
- Weekend mode: Available`
  }

  private generatePrivacyContent(): string {
    const { profileVisibility, dataSharing, analytics } = this.settings.privacy
    
    return `Privacy Controls:
• Profile Visibility: ${profileVisibility}
• Data Sharing: ${dataSharing ? 'Enabled' : 'Disabled'} ${dataSharing ? '⚠️' : '🔒'}
• Analytics: ${analytics ? 'Enabled' : 'Disabled'}

Security Features:
- Two-Factor Authentication: Recommended
- Session Management: Active
- Login History: Available
- Data Export: Available

Privacy Level: ${dataSharing ? 'Standard' : 'Enhanced'}`
  }

  private generateAdvancedContent(): string {
    const { debugMode, experimentalFeatures, autoSave, backupFrequency } = this.settings.advanced
    
    return `Developer Options:
• Debug Mode: ${debugMode ? 'Enabled 🐛' : 'Disabled'}
• Experimental Features: ${experimentalFeatures ? 'Enabled ⚗️' : 'Disabled'}

Data Management:
• Auto-save: ${autoSave ? 'Enabled 💾' : 'Disabled'}
• Backup Frequency: ${backupFrequency}
• Cache Management: Available

Performance:
- Memory Usage: 45MB
- CPU Usage: Low
- Network: Optimal

${debugMode ? 'WARNING: Debug mode may affect performance' : ''}`
  }

  private getNotificationsBadge(): string {
    const enabledCount = Object.values(this.settings.notifications).filter(v => v === true).length
    return enabledCount > 0 ? enabledCount.toString() : undefined
  }

  updateSetting(section: string, key: string, value: any) {
    if (this.settings[section]) {
      this.settings[section][key] = value
      this.saveSettings()
      this.refreshSectionContent(section)
      
      console.log(`Updated ${section}.${key} = ${value}`)
    }
  }

  private refreshSectionContent(sectionId: string) {
    let content = ''
    
    switch (sectionId) {
      case 'appearance':
        content = this.generateAppearanceContent()
        break
      case 'notifications':
        content = this.generateNotificationsContent()
        // Update badge
        this.settingsAccordion.updateSection(sectionId, {
          badge: this.getNotificationsBadge()
        })
        break
      case 'privacy':
        content = this.generatePrivacyContent()
        break
      case 'advanced':
        content = this.generateAdvancedContent()
        break
    }
    
    if (content) {
      this.settingsAccordion.updateSectionContent(sectionId, content)
    }
  }

  private saveSettings() {
    localStorage.setItem('appSettings', JSON.stringify(this.settings))
  }

  private saveUIState(state: any) {
    localStorage.setItem('settingsUIState', JSON.stringify(state))
  }

  private trackSettingsAccess(sectionId: string) {
    // Analytics tracking
    console.log(`Settings section accessed: ${sectionId}`)
  }

  // Quick setting toggles
  toggleDarkMode() {
    const newTheme = this.settings.appearance.theme === 'dark' ? 'light' : 'dark'
    this.updateSetting('appearance', 'theme', newTheme)
  }

  toggleNotifications() {
    const newValue = !this.settings.notifications.email
    this.updateSetting('notifications', 'email', newValue)
    this.updateSetting('notifications', 'push', newValue)
  }

  toggleDebugMode() {
    const newValue = !this.settings.advanced.debugMode
    this.updateSetting('advanced', 'debugMode', newValue)
    
    // Update badge
    const badge = newValue ? 'Debug' : undefined
    this.settingsAccordion.updateSection('advanced', { badge })
  }

  resetSection(sectionId: string) {
    const defaults = {
      appearance: { theme: 'light', fontSize: 14, colorScheme: 'blue' },
      notifications: { email: true, push: true, sound: true, frequency: 'daily' },
      privacy: { profileVisibility: 'public', dataSharing: true, analytics: true },
      advanced: { debugMode: false, experimentalFeatures: false, autoSave: true, backupFrequency: 'weekly' }
    }
    
    if (defaults[sectionId]) {
      this.settings[sectionId] = { ...defaults[sectionId] }
      this.saveSettings()
      this.refreshSectionContent(sectionId)
      console.log(`Reset ${sectionId} to defaults`)
    }
  }

  exportSettings() {
    return {
      settings: this.settings,
      uiState: {
        expandedSections: this.settingsAccordion.getExpandedSections(),
        focused: this.settingsAccordion.getFocusedSection()
      },
      timestamp: new Date().toISOString()
    }
  }

  importSettings(data: any) {
    if (data.settings) {
      this.settings = { ...data.settings }
      this.saveSettings()
      
      // Refresh all sections
      ['appearance', 'notifications', 'privacy', 'advanced'].forEach(section => {
        this.refreshSectionContent(section)
      })
    }
    
    if (data.uiState) {
      // Restore UI state
      data.uiState.expandedSections?.forEach((sectionId: string) => {
        this.settingsAccordion.expandSection(sectionId)
      })
      
      if (data.uiState.focused) {
        this.settingsAccordion.focusSection(data.uiState.focused)
      }
    }
  }

  render() {
    return this.settingsAccordion.render()
  }
}

// Usage
const appSettings = new ApplicationSettings()

// Quick toggles
appSettings.toggleDarkMode()
appSettings.toggleNotifications()

// Update specific settings
appSettings.updateSetting('appearance', 'fontSize', 16)
appSettings.updateSetting('privacy', 'dataSharing', false)

// Reset a section
appSettings.resetSection('notifications')

// Export/import
const exported = appSettings.exportSettings()
console.log('Exported settings:', exported)

// Later...
appSettings.importSettings(exported)

console.log(appSettings.render())
```

## CSS Styling

```css
/* Accordion container */
.accordion {
  font-family: 'Fira Code', 'JetBrains Mono', monospace;
  border-radius: 4px;
  overflow: hidden;
}

.accordion-bordered {
  border: 1px solid #e2e8f0;
}

.accordion-rounded {
  border-radius: 8px;
}

.accordion-compact {
  margin: 0;
}

.accordion-disabled {
  opacity: 0.6;
  pointer-events: none;
}

/* Accordion sections */
.accordion-section {
  border-bottom: 1px solid #e2e8f0;
  transition: all 0.2s ease;
}

.accordion-section:last-child {
  border-bottom: none;
}

.accordion-section-expanded {
  background: #f8f9fa;
}

.accordion-section-focused {
  outline: 2px solid #007bff;
  outline-offset: -2px;
}

.accordion-section-disabled {
  opacity: 0.6;
  background: #f1f3f4;
}

/* Section headers */
.accordion-header {
  padding: 1rem;
  background: #ffffff;
  cursor: pointer;
  user-select: none;
  display: flex;
  align-items: center;
  gap: 0.5rem;
  transition: background-color 0.15s ease;
}

.accordion-header:hover {
  background: #f8f9fa;
}

.accordion-section-expanded .accordion-header {
  background: #e3f2fd;
  border-bottom: 1px solid #bbdefb;
}

.accordion-section-disabled .accordion-header {
  cursor: not-allowed;
  background: #f1f3f4;
}

.accordion-section-disabled .accordion-header:hover {
  background: #f1f3f4;
}

/* Header elements */
.accordion-icon {
  font-size: 0.875rem;
  color: #6c757d;
  min-width: 1rem;
  text-align: center;
  transition: transform 0.2s ease;
}

.accordion-section-expanded .accordion-icon {
  transform: rotate(90deg);
}

.section-icon {
  font-size: 1rem;
}

.section-title {
  font-weight: 600;
  color: #212529;
  flex: 1;
}

.section-badge {
  background: #dc3545;
  color: white;
  padding: 0.125rem 0.5rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: bold;
}

.section-description {
  padding: 0 1rem 0.5rem 1rem;
  font-size: 0.875rem;
  color: #6c757d;
  font-style: italic;
}

/* Section content */
.accordion-content {
  background: #ffffff;
  border-top: 1px solid #e9ecef;
  transition: all 0.3s ease;
}

.accordion-section-expanded .accordion-content {
  animation: expandContent 0.3s ease-out;
}

.accordion-section-collapsed .accordion-content {
  animation: collapseContent 0.3s ease-in;
}

@keyframes expandContent {
  from {
    max-height: 0;
    opacity: 0;
  }
  to {
    max-height: 500px;
    opacity: 1;
  }
}

@keyframes collapseContent {
  from {
    max-height: 500px;
    opacity: 1;
  }
  to {
    max-height: 0;
    opacity: 0;
  }
}

.content-line {
  padding: 0.25rem 1rem;
  line-height: 1.5;
  color: #495057;
}

.content-spacing {
  height: 0.5rem;
}

.animation-indicator {
  padding: 0.5rem 1rem;
  font-style: italic;
  color: #6c757d;
  font-size: 0.875rem;
}

/* Compact styling */
.accordion-compact .accordion-header {
  padding: 0.5rem;
}

.accordion-compact .content-line {
  padding: 0.125rem 0.5rem;
}

.accordion-compact .section-description {
  padding: 0 0.5rem 0.25rem 0.5rem;
}

/* Custom accordion types */
.settings-panel {
  background: #f8f9fa;
  border: 1px solid #dee2e6;
}

.settings-panel .section-title {
  color: #495057;
}

.documentation-browser {
  max-width: 800px;
  margin: 0 auto;
}

.documentation-browser .section-badge {
  background: #28a745;
}

/* Animation easing classes */
.accordion[data-animation-easing="ease-in-back"] .accordion-content {
  transition-timing-function: cubic-bezier(0.6, -0.28, 0.735, 0.045);
}

.accordion[data-animation-easing="ease-out-back"] .accordion-content {
  transition-timing-function: cubic-bezier(0.175, 0.885, 0.32, 1.275);
}

.accordion[data-animation-easing="ease-in-out-back"] .accordion-content {
  transition-timing-function: cubic-bezier(0.68, -0.55, 0.265, 1.55);
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .accordion {
    background: #1a202c;
    border-color: #4a5568;
  }
  
  .accordion-header {
    background: #2d3748;
    color: #e2e8f0;
  }
  
  .accordion-header:hover {
    background: #4a5568;
  }
  
  .accordion-section-expanded .accordion-header {
    background: #2b6cb0;
    border-bottom-color: #3182ce;
  }
  
  .accordion-content {
    background: #1a202c;
    border-top-color: #4a5568;
  }
  
  .content-line {
    color: #e2e8f0;
  }
  
  .section-title {
    color: #f7fafc;
  }
  
  .section-description {
    color: #a0aec0;
  }
  
  .accordion-icon {
    color: #a0aec0;
  }
}

/* High contrast mode */
@media (prefers-contrast: high) {
  .accordion-header {
    border: 2px solid #000000;
  }
  
  .accordion-section-focused {
    outline: 3px solid #000000;
  }
  
  .section-title {
    font-weight: bold;
    color: #000000;
  }
}

/* Reduced motion */
@media (prefers-reduced-motion: reduce) {
  .accordion-content,
  .accordion-icon {
    transition: none;
  }
  
  .accordion-section-expanded .accordion-content,
  .accordion-section-collapsed .accordion-content {
    animation: none;
  }
}

/* Responsive design */
@media (max-width: 768px) {
  .accordion-header {
    padding: 0.75rem;
    font-size: 0.9rem;
  }
  
  .content-line {
    padding: 0.25rem 0.75rem;
    font-size: 0.875rem;
  }
  
  .section-description {
    padding: 0 0.75rem 0.5rem 0.75rem;
    font-size: 0.8rem;
  }
  
  .accordion-compact .accordion-header {
    padding: 0.5rem;
  }
}
```

## Best Practices

### 1. Use Appropriate Expansion Modes

```typescript
// ✅ Good - single expand for FAQ/exclusive content
const faqAccordion = createFaqAccordion([...])

// ✅ Good - multi-expand for settings/independent sections
const settingsAccordion = createSettingsAccordion([...])
```

### 2. Provide Clear Section Titles and Descriptions

```typescript
// ✅ Good - descriptive titles with helpful descriptions
const accordion = createAccordion({
  sections: [
    {
      id: 'billing',
      title: 'Billing & Payments',
      description: 'Manage your subscription and payment methods',
      icon: '💳',
      content: '...'
    }
  ]
})
```

### 3. Handle Loading States and Dynamic Content

```typescript
// ✅ Good - lazy loading with proper feedback
const accordion = createAccordion({
  sections: [{ id: 'dynamic', title: 'Dynamic Content', content: 'Loading...' }],
  callbacks: {
    onExpand: async (sectionId) => {
      if (sectionId === 'dynamic') {
        const content = await loadDynamicContent()
        accordion.updateSectionContent(sectionId, content)
      }
    }
  }
})
```

### 4. Implement Proper Keyboard Navigation

```typescript
// ✅ Good - comprehensive keyboard support
const handleKeyboard = (event: KeyboardEvent) => {
  switch (event.key) {
    case 'ArrowDown': accordion.focusNext(); break
    case 'ArrowUp': accordion.focusPrevious(); break
    case 'Home': accordion.focusFirst(); break
    case 'End': accordion.focusLast(); break
    case 'Enter':
    case ' ':
      const focused = accordion.getFocusedSection()
      if (focused) accordion.toggleSection(focused)
      break
  }
}
```

### 5. Save and Restore State

```typescript
// ✅ Good - persistent accordion state
const accordion = createAccordion({
  // ... configuration
  callbacks: {
    onChange: (expandedSections) => {
      localStorage.setItem('accordionState', JSON.stringify(expandedSections))
    }
  }
})

// Restore state on load
const savedState = JSON.parse(localStorage.getItem('accordionState') || '[]')
savedState.forEach(sectionId => accordion.expandSection(sectionId))
```

## Related Widgets

- **[Tabs](./tabs)** - Tab-based navigation and content switching
- **[Panel](./panel)** - Container panels for organizing content
- **[Modal](./modal)** - Overlay dialogs and popups
- **[Tree](./tree)** - Hierarchical data display with expansion

## Examples

- **[Basic Accordion](../../examples/basic/accordion-basic)** - Simple expandable sections
- **[Settings Panel](../../examples/advanced/settings-accordion)** - Application settings interface
- **[FAQ System](../../examples/apps/faq-accordion)** - Frequently asked questions
- **[Documentation Browser](../../examples/advanced/docs-accordion)** - Interactive documentation

The Accordion widget provides comprehensive expandable/collapsible functionality with animation support, keyboard navigation, and extensive customization options for building organized content interfaces.