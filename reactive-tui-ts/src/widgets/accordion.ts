/**
 * Accordion Widget - TypeScript Implementation
 * 
 * A comprehensive accordion widget supporting expandable/collapsible sections,
 * with keyboard navigation, custom styling, and animation support.
 * 
 * Features:
 * - Multiple Sections: Support for multiple collapsible sections in a single accordion
 * - Expand/Collapse: Individual section expand/collapse with smooth animations
 * - Multi-Expand Mode: Allow multiple sections to be open simultaneously or single-expand mode
 * - Keyboard Navigation: Arrow keys, Enter/Space, Home/End navigation support
 * - Custom Headers: Customizable section headers with icons, badges, and styling
 * - Rich Content: Support for any content type in accordion sections
 * - Animation Support: Smooth expand/collapse animations with easing
 * - Accessibility: Full ARIA support and screen reader compatibility
 * - Event Callbacks: onExpand, onCollapse, onChange event handling
 * - Themeable: CSS utility classes and custom styling support
 * 
 * Basic Usage:
 * ```typescript
 * const accordion = createAccordion({
 *   id: 'settings-accordion',
 *   sections: [
 *     {
 *       id: 'general',
 *       title: 'General Settings',
 *       content: 'General application settings go here...',
 *       expanded: true
 *     },
 *     {
 *       id: 'privacy', 
 *       title: 'Privacy & Security',
 *       content: 'Privacy and security options...',
 *       icon: '🔒'
 *     }
 *   ],
 *   multiExpand: true,
 *   animated: true
 * });
 * ```
 */

import type { ElementBuilder } from '../types';
import { ElementBuilderImpl } from '../components';

/// Unique identifier for accordion sections
export type SectionId = string;

/// Animation easing functions
export enum AnimationEasing {
    Linear = 'linear',
    EaseIn = 'ease-in',
    EaseOut = 'ease-out', 
    EaseInOut = 'ease-in-out',
    EaseInBack = 'ease-in-back',
    EaseOutBack = 'ease-out-back',
    EaseInOutBack = 'ease-in-out-back'
}

/// Animation state for individual sections
export enum AnimationState {
    Collapsed = 'collapsed',
    Expanding = 'expanding',
    Expanded = 'expanded',
    Collapsing = 'collapsing'
}

/// Animation configuration for accordion sections
export interface AccordionAnimation {
    /// Enable animation
    enabled: boolean;
    /// Animation duration in milliseconds
    duration: number;
    /// Animation easing function
    easing: AnimationEasing;
    /// Stagger delay between multiple sections (ms)
    staggerDelay: number;
}

/// Styling configuration for accordion sections
export interface AccordionSectionStyle {
    /// Header background color
    headerBackground?: string;
    /// Header text color
    headerTextColor?: string;
    /// Content background color
    contentBackground?: string;
    /// Content text color
    contentTextColor?: string;
    /// Border color
    borderColor?: string;
    /// Disabled section opacity
    disabledOpacity: number;
    /// Custom padding for content
    contentPadding: number;
    /// Header height
    headerHeight: number;
}

/// Individual accordion section
export interface AccordionSection {
    /// Unique section identifier
    id: SectionId;
    /// Section header title
    title: string;
    /// Optional section description/subtitle
    description?: string;
    /// Section content (can be text or HTML)
    content: string;
    /// Whether section is expanded
    expanded: boolean;
    /// Whether section is disabled
    disabled: boolean;
    /// Optional header icon
    icon?: string;
    /// Optional badge text
    badge?: string;
    /// Custom CSS classes for the section
    cssClasses: string[];
    /// Section-specific styling
    style: AccordionSectionStyle;
}

/// Accordion state management
export interface AccordionState {
    /// Currently expanded sections
    expandedSections: SectionId[];
    /// Currently focused section
    focusedSection?: SectionId;
    /// Animation states for sections
    animationStates: Map<SectionId, AnimationState>;
    /// Whether accordion is disabled
    disabled: boolean;
}

/// Accordion configuration
export interface AccordionConfig {
    /// Allow multiple sections to be expanded simultaneously
    multiExpand: boolean;
    /// Enable animations
    animated: boolean;
    /// Animation configuration
    animation: AccordionAnimation;
    /// Show section borders
    bordered: boolean;
    /// Rounded corners for sections
    rounded: boolean;
    /// Compact layout (reduced spacing)
    compact: boolean;
    /// Collapsible sections (can all be collapsed)
    collapsible: boolean;
    /// Show expand/collapse icons
    showIcons: boolean;
    /// Icon to show when section is collapsed
    collapsedIcon: string;
    /// Icon to show when section is expanded
    expandedIcon: string;
}

/// Event callbacks for accordion interactions
export interface AccordionCallbacks {
    /// Called when a section is expanded
    onExpand?: (sectionId: SectionId) => void;
    /// Called when a section is collapsed
    onCollapse?: (sectionId: SectionId) => void;
    /// Called when sections change (with all expanded section IDs)
    onChange?: (expandedSections: SectionId[]) => void;
    /// Called when focus changes
    onFocus?: (sectionId: SectionId) => void;
    /// Called on section click/activation
    onSectionClick?: (sectionId: SectionId) => void;
}

/// Main accordion configuration interface
export interface AccordionOptions {
    id: string;
    sections: Partial<AccordionSection>[];
    config?: Partial<AccordionConfig>;
    callbacks?: AccordionCallbacks;
    cssClasses?: string[];
}

/// Default values
const DEFAULT_ANIMATION: AccordionAnimation = {
    enabled: true,
    duration: 300,
    easing: AnimationEasing.EaseInOut,
    staggerDelay: 50
};

const DEFAULT_SECTION_STYLE: AccordionSectionStyle = {
    disabledOpacity: 0.6,
    contentPadding: 2,
    headerHeight: 3
};

const DEFAULT_CONFIG: AccordionConfig = {
    multiExpand: false,
    animated: true,
    animation: DEFAULT_ANIMATION,
    bordered: true,
    rounded: false,
    compact: false,
    collapsible: true,
    showIcons: true,
    collapsedIcon: '▶',  // Right arrow
    expandedIcon: '▼'    // Down arrow
};

/**
 * Accordion class for managing expandable/collapsible sections
 */
export class Accordion {
    private id: string;
    private sections: AccordionSection[];
    private state: AccordionState;
    private config: AccordionConfig;
    private callbacks: AccordionCallbacks;
    private cssClasses: string[];

    constructor(options: AccordionOptions) {
        this.id = options.id;
        this.config = { ...DEFAULT_CONFIG, ...options.config };
        this.callbacks = options.callbacks || {};
        this.cssClasses = options.cssClasses || [];
        
        // Initialize sections with defaults
        this.sections = options.sections.map((section, index) => ({
            id: section.id || `section-${index}`,
            title: section.title || `Section ${index + 1}`,
            description: section.description,
            content: section.content || '',
            expanded: section.expanded || false,
            disabled: section.disabled || false,
            icon: section.icon,
            badge: section.badge,
            cssClasses: section.cssClasses || [],
            style: { ...DEFAULT_SECTION_STYLE, ...section.style }
        }));

        // Initialize state
        this.state = {
            expandedSections: this.sections.filter(s => s.expanded).map(s => s.id),
            focusedSection: undefined,
            animationStates: new Map(),
            disabled: false
        };
    }

    /// Add a new section to the accordion
    addSection(section: Partial<AccordionSection>): void {
        const newSection: AccordionSection = {
            id: section.id || `section-${this.sections.length}`,
            title: section.title || `Section ${this.sections.length + 1}`,
            description: section.description,
            content: section.content || '',
            expanded: section.expanded || false,
            disabled: section.disabled || false,
            icon: section.icon,
            badge: section.badge,
            cssClasses: section.cssClasses || [],
            style: { ...DEFAULT_SECTION_STYLE, ...section.style }
        };
        
        this.sections.push(newSection);
        this.refreshState();
    }

    /// Remove a section from the accordion
    removeSection(sectionId: string): AccordionSection | null {
        const index = this.sections.findIndex(s => s.id === sectionId);
        if (index !== -1) {
            const removed = this.sections.splice(index, 1)[0];
            this.refreshState();
            return removed;
        }
        return null;
    }

    /// Expand a specific section
    expandSection(sectionId: string): boolean {
        const section = this.sections.find(s => s.id === sectionId);
        if (!section || section.disabled) {
            return false;
        }

        // If not multi-expand mode, collapse all other sections
        if (!this.config.multiExpand) {
            this.state.expandedSections = [];
            for (const s of this.sections) {
                if (s.id !== sectionId) {
                    this.state.animationStates.set(s.id, AnimationState.Collapsed);
                }
            }
        }

        // Add section to expanded list if not already there
        if (!this.state.expandedSections.includes(sectionId)) {
            this.state.expandedSections.push(sectionId);
            
            // Set animation state
            if (this.config.animated) {
                this.state.animationStates.set(sectionId, AnimationState.Expanding);
            } else {
                this.state.animationStates.set(sectionId, AnimationState.Expanded);
            }
        }

        // Update section expanded state
        section.expanded = true;

        // Trigger callback
        if (this.callbacks.onExpand) {
            this.callbacks.onExpand(sectionId);
        }

        this.triggerChangeCallback();
        return true;
    }

    /// Collapse a specific section
    collapseSection(sectionId: string): boolean {
        const section = this.sections.find(s => s.id === sectionId);
        if (!section) {
            return false;
        }

        // Remove section from expanded list
        this.state.expandedSections = this.state.expandedSections.filter(id => id !== sectionId);
        
        // Set animation state
        if (this.config.animated) {
            this.state.animationStates.set(sectionId, AnimationState.Collapsing);
        } else {
            this.state.animationStates.set(sectionId, AnimationState.Collapsed);
        }

        // Update section expanded state
        section.expanded = false;

        // Trigger callback
        if (this.callbacks.onCollapse) {
            this.callbacks.onCollapse(sectionId);
        }

        this.triggerChangeCallback();
        return true;
    }

    /// Toggle a section's expanded state
    toggleSection(sectionId: string): boolean {
        const isExpanded = this.isSectionExpanded(sectionId);
        return isExpanded ? this.collapseSection(sectionId) : this.expandSection(sectionId);
    }

    /// Check if a section is expanded
    isSectionExpanded(sectionId: string): boolean {
        return this.state.expandedSections.includes(sectionId);
    }

    /// Expand all sections (if multi-expand is enabled)
    expandAll(): boolean {
        if (!this.config.multiExpand) {
            return false;
        }

        const sectionsToExpand = this.sections.filter(s => !s.disabled);
        for (const section of sectionsToExpand) {
            this.expandSection(section.id);
        }
        return true;
    }

    /// Collapse all sections
    collapseAll(): boolean {
        const sectionsToCollapse = this.sections.filter(s => this.isSectionExpanded(s.id));
        for (const section of sectionsToCollapse) {
            this.collapseSection(section.id);
        }
        return true;
    }

    /// Focus a specific section
    focusSection(sectionId: string): boolean {
        if (!this.sections.find(s => s.id === sectionId)) {
            return false;
        }

        this.state.focusedSection = sectionId;

        // Trigger callback
        if (this.callbacks.onFocus) {
            this.callbacks.onFocus(sectionId);
        }

        return true;
    }

    /// Get the currently focused section
    getFocusedSection(): string | undefined {
        return this.state.focusedSection;
    }

    /// Navigate to next section
    focusNext(): boolean {
        const sectionIds = this.sections.map(s => s.id);
        if (sectionIds.length === 0) {
            return false;
        }

        const currentIndex = this.state.focusedSection 
            ? sectionIds.indexOf(this.state.focusedSection) 
            : -1;
        
        const nextIndex = (currentIndex + 1) % sectionIds.length;
        return this.focusSection(sectionIds[nextIndex]);
    }

    /// Navigate to previous section
    focusPrevious(): boolean {
        const sectionIds = this.sections.map(s => s.id);
        if (sectionIds.length === 0) {
            return false;
        }

        const currentIndex = this.state.focusedSection 
            ? sectionIds.indexOf(this.state.focusedSection) 
            : 0;
        
        const prevIndex = currentIndex === 0 ? sectionIds.length - 1 : currentIndex - 1;
        return this.focusSection(sectionIds[prevIndex]);
    }

    /// Focus first section
    focusFirst(): boolean {
        if (this.sections.length > 0) {
            return this.focusSection(this.sections[0].id);
        }
        return false;
    }

    /// Focus last section
    focusLast(): boolean {
        if (this.sections.length > 0) {
            return this.focusSection(this.sections[this.sections.length - 1].id);
        }
        return false;
    }

    /// Get all expanded section IDs
    getExpandedSections(): string[] {
        return [...this.state.expandedSections];
    }

    /// Get section by ID
    getSection(sectionId: string): AccordionSection | undefined {
        return this.sections.find(s => s.id === sectionId);
    }

    /// Update section content
    updateSectionContent(sectionId: string, content: string): boolean {
        const section = this.getSection(sectionId);
        if (section) {
            section.content = content;
            return true;
        }
        return false;
    }

    /// Enable/disable the entire accordion
    setDisabled(disabled: boolean): void {
        this.state.disabled = disabled;
    }

    /// Check if accordion is disabled
    isDisabled(): boolean {
        return this.state.disabled;
    }

    /// Get the number of sections
    sectionCount(): number {
        return this.sections.length;
    }

    /// Private helper to refresh internal state
    private refreshState(): void {
        const validSectionIds = this.sections.map(s => s.id);
        
        // Remove expanded sections that no longer exist
        this.state.expandedSections = this.state.expandedSections.filter(id => 
            validSectionIds.includes(id)
        );
        
        // Remove animation states for sections that no longer exist
        for (const id of this.state.animationStates.keys()) {
            if (!validSectionIds.includes(id)) {
                this.state.animationStates.delete(id);
            }
        }
        
        // Reset focus if focused section no longer exists
        if (this.state.focusedSection && !validSectionIds.includes(this.state.focusedSection)) {
            this.state.focusedSection = undefined;
        }
    }

    /// Private helper to trigger change callback
    private triggerChangeCallback(): void {
        if (this.callbacks.onChange) {
            this.callbacks.onChange(this.getExpandedSections());
        }
    }

    /// Render the accordion to HTML string
    render(): string {
        const classes = ['accordion'];
        if (this.config.bordered) classes.push('accordion-bordered');
        if (this.config.rounded) classes.push('accordion-rounded');
        if (this.config.compact) classes.push('accordion-compact');
        if (this.state.disabled) classes.push('accordion-disabled');
        classes.push(...this.cssClasses);

        let html = `<div id="${this.id}" class="${classes.join(' ')}" role="tablist">`;

        for (let i = 0; i < this.sections.length; i++) {
            const section = this.sections[i];
            const isExpanded = this.state.expandedSections.includes(section.id);
            const isFocused = this.state.focusedSection === section.id;
            const animationState = this.state.animationStates.get(section.id) || 
                (isExpanded ? AnimationState.Expanded : AnimationState.Collapsed);

            html += this.renderSection(section, isExpanded, isFocused, animationState);
            
            // Add spacing between sections (except last)
            if (i < this.sections.length - 1 && !this.config.compact) {
                html += '\n';
            }
        }

        html += '</div>';
        return html;
    }

    /// Render individual section
    private renderSection(
        section: AccordionSection, 
        isExpanded: boolean, 
        isFocused: boolean, 
        animationState: AnimationState
    ): string {
        const sectionClasses = ['accordion-section'];
        if (isExpanded) sectionClasses.push('accordion-section-expanded');
        if (isFocused) sectionClasses.push('accordion-section-focused');
        if (section.disabled) sectionClasses.push('accordion-section-disabled');
        sectionClasses.push(...section.cssClasses);

        let html = `<div class="${sectionClasses.join(' ')}" data-section-id="${section.id}">`;
        
        // Render section header
        html += this.renderSectionHeader(section, isExpanded, isFocused);
        
        // Render section content if expanded or animating
        if ([AnimationState.Expanded, AnimationState.Expanding, AnimationState.Collapsing].includes(animationState)) {
            html += this.renderSectionContent(section, animationState);
        }
        
        html += '</div>';
        return html;
    }

    /// Render section header
    private renderSectionHeader(section: AccordionSection, isExpanded: boolean, isFocused: boolean): string {
        const icon = this.config.showIcons ? 
            (isExpanded ? this.config.expandedIcon : this.config.collapsedIcon) : '';
        
        const focusIndicator = isFocused ? '► ' : '  ';
        const disabledIndicator = section.disabled ? ' (disabled)' : '';
        
        let html = `<div class="accordion-header" role="tab" aria-expanded="${isExpanded}">`;
        html += focusIndicator;
        
        if (icon) {
            html += `<span class="accordion-icon">${icon}</span> `;
        }
        
        if (section.icon) {
            html += `<span class="section-icon">${section.icon}</span> `;
        }
        
        html += `<span class="section-title">${section.title}</span>`;
        
        if (section.badge) {
            html += ` <span class="section-badge">[${section.badge}]</span>`;
        }
        
        html += disabledIndicator;
        html += '</div>';
        
        if (section.description) {
            html += `<div class="section-description">    ${section.description}</div>`;
        }
        
        return html;
    }

    /// Render section content
    private renderSectionContent(section: AccordionSection, animationState: AnimationState): string {
        const padding = ' '.repeat(this.config.compact ? 2 : section.style.contentPadding);
        
        let html = '<div class="accordion-content" role="tabpanel">';
        
        // Animation indicator
        if (animationState === AnimationState.Expanding) {
            html += `<div class="animation-indicator">${padding}[Expanding...]</div>`;
        } else if (animationState === AnimationState.Collapsing) {
            html += `<div class="animation-indicator">${padding}[Collapsing...]</div>`;
        }
        
        // Content lines
        const lines = section.content.split('\n');
        for (const line of lines) {
            html += `<div class="content-line">${padding}${line}</div>`;
        }
        
        // Add bottom padding for expanded sections
        if (animationState === AnimationState.Expanded && !this.config.compact) {
            html += '<div class="content-spacing"></div>';
        }
        
        html += '</div>';
        return html;
    }

    /// Convert to ElementBuilder for integration with layout system
    toElement(): ElementBuilder {
        const builder = new ElementBuilderImpl('div');
        builder.id(this.id);
        builder.classes(this.cssClasses);
        builder.content(this.render());
        builder.attr('role', 'tablist');
        builder.focusable(false);
        return builder;
    }
}

/**
 * Create an accordion with the specified configuration
 */
export function createAccordion(options: AccordionOptions): Accordion {
    return new Accordion(options);
}

/**
 * Create a simple settings-style accordion
 */
export function createSettingsAccordion(sections: Array<{id: string, title: string, content: string}>): Accordion {
    return createAccordion({
        id: 'settings-accordion',
        sections: sections.map(s => ({ ...s, expanded: false })),
        config: {
            multiExpand: true,
            bordered: true,
            animated: true
        }
    });
}

/**
 * Create a FAQ-style accordion (single expand)
 */
export function createFaqAccordion(faqs: Array<{id: string, question: string, answer: string}>): Accordion {
    return createAccordion({
        id: 'faq-accordion',
        sections: faqs.map(faq => ({
            id: faq.id,
            title: faq.question,
            content: faq.answer,
            expanded: false
        })),
        config: {
            multiExpand: false,
            bordered: true,
            animated: true,
            collapsedIcon: '❓',
            expandedIcon: '✅'
        }
    });
}

/**
 * Create a compact accordion with minimal styling
 */
export function createCompactAccordion(sections: Array<{id: string, title: string, content: string}>): Accordion {
    return createAccordion({
        id: 'compact-accordion',
        sections: sections.map(s => ({ ...s, expanded: false })),
        config: {
            compact: true,
            bordered: false,
            animated: false,
            collapsedIcon: '▸',
            expandedIcon: '▾'
        }
    });
}

/**
 * Create an accordion element using the component builder pattern
 */
export function accordionElement(options: AccordionOptions): ElementBuilder {
    const accordion = createAccordion(options);
    return accordion.toElement();
}

// Types are already exported above with their definitions