/*!
 * Bar Component for Header/Footer Bars
 *
 * A comprehensive bar widget that provides flexible content positioning,
 * styling integration, and responsive design for headers, footers, and
 * navigation bars in terminal user interfaces.
 */

use crate::{
    layout::LayoutRect,
    themes::{color_to_ansi, hex, ColorDefinition, ColorTheme, UtilityProcessor},
};
use serde::{Deserialize, Serialize};
use std::fmt::Write;

/// Bar widget types for different use cases
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BarType {
    /// Header bar at top of interface
    #[default]
    Header,
    /// Footer bar at bottom of interface
    Footer,
    /// Navigation bar for menus
    Navigation,
    /// Status bar for information display
    Status,
    /// Tool bar for actions
    Toolbar,
    /// Tab bar for tab navigation
    Tabbar,
    /// Custom bar type
    Custom,
}

/// Content positioning within the bar
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BarPosition {
    /// Left-aligned content
    #[default]
    Left,
    /// Center-aligned content
    Center,
    /// Right-aligned content
    Right,
    /// Justify content across full width
    Justify,
}

/// Bar border styles
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BarBorderStyle {
    /// No border
    None,
    /// Single line border
    #[default]
    Single,
    /// Double line border
    Double,
    /// Thick border
    Thick,
    /// Custom Unicode border
    Custom(char),
}

/// Bar size variants
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum BarSize {
    /// Compact bar (1 line height)
    Compact,
    /// Normal bar (2 lines height)
    #[default]
    Normal,
    /// Large bar (3 lines height)
    Large,
    /// Custom height
    Custom(u16),
}

/// Bar content item with positioning
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarItem {
    /// Item content text
    pub content: String,
    /// Position within the bar
    pub position: BarPosition,
    /// CSS classes for styling
    pub css_classes: Vec<String>,
    /// Optional icon
    pub icon: Option<char>,
    /// Item identifier
    pub id: Option<String>,
    /// Whether item is clickable
    pub clickable: bool,
    /// Item weight for justify positioning
    pub weight: f32,
}

/// Bar styling configuration
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BarStyle {
    /// Background color
    pub background: Option<ColorDefinition>,
    /// Text color
    pub foreground: Option<ColorDefinition>,
    /// Border color
    pub border_color: Option<ColorDefinition>,
    /// Border style
    pub border_style: BarBorderStyle,
    /// Padding around content
    pub padding: u16,
    /// Item separator character
    pub separator: Option<char>,
    /// Whether to fill full width
    pub fill_width: bool,
}

/// CSS-styled bar widget
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bar {
    /// Unique identifier
    pub id: String,
    /// Bar type
    pub bar_type: BarType,
    /// Size variant
    pub size: BarSize,
    /// Bar items
    pub items: Vec<BarItem>,
    /// Styling configuration
    pub style: BarStyle,
    /// CSS classes for styling
    pub css_classes: Vec<String>,
    /// Whether bar is visible
    pub visible: bool,
    /// Whether bar is sticky (remains at top/bottom)
    pub sticky: bool,
}

/// Builder for Bar component
pub struct BarBuilder {
    bar: Bar,
}

impl Default for BarStyle {
    fn default() -> Self {
        Self {
            background: None,
            foreground: None,
            border_color: None,
            border_style: BarBorderStyle::Single,
            padding: 1,
            separator: Some('│'),
            fill_width: true,
        }
    }
}

impl BarItem {
    /// Create a new bar item
    pub fn new(content: impl Into<String>, position: BarPosition) -> Self {
        Self {
            content: content.into(),
            position,
            css_classes: Vec::new(),
            icon: None,
            id: None,
            clickable: false,
            weight: 1.0,
        }
    }

    /// Add CSS class to item
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.css_classes.push(class.into());
        self
    }

    /// Set item icon
    pub fn icon(mut self, icon: char) -> Self {
        self.icon = Some(icon);
        self
    }

    /// Set item ID
    pub fn id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Make item clickable
    pub fn clickable(mut self) -> Self {
        self.clickable = true;
        self
    }

    /// Set item weight for justify positioning
    pub fn weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
}

impl Bar {
    /// Create a new bar
    pub fn new(id: impl Into<String>, bar_type: BarType) -> Self {
        Self {
            id: id.into(),
            bar_type,
            size: BarSize::default(),
            items: Vec::new(),
            style: BarStyle::default(),
            css_classes: Vec::new(),
            visible: true,
            sticky: false,
        }
    }

    /// Create a builder for the bar
    pub fn builder(id: impl Into<String>, bar_type: BarType) -> BarBuilder {
        BarBuilder {
            bar: Self::new(id, bar_type),
        }
    }

    /// Add an item to the bar
    pub fn add_item(&mut self, item: BarItem) {
        self.items.push(item);
    }

    /// Remove item by ID
    pub fn remove_item(&mut self, id: &str) -> Option<BarItem> {
        if let Some(pos) = self
            .items
            .iter()
            .position(|item| item.id.as_deref() == Some(id))
        {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    /// Get item by ID
    pub fn get_item(&self, id: &str) -> Option<&BarItem> {
        self.items
            .iter()
            .find(|item| item.id.as_deref() == Some(id))
    }

    /// Get mutable item by ID
    pub fn get_item_mut(&mut self, id: &str) -> Option<&mut BarItem> {
        self.items
            .iter_mut()
            .find(|item| item.id.as_deref() == Some(id))
    }

    /// Set bar visibility
    pub fn set_visible(&mut self, visible: bool) {
        self.visible = visible;
    }

    /// Set bar sticky behavior
    pub fn set_sticky(&mut self, sticky: bool) {
        self.sticky = sticky;
    }

    /// Get bar height
    pub fn get_height(&self) -> u16 {
        match self.size {
            BarSize::Compact => 3, // 1 content line + 2 borders
            BarSize::Normal => 4,  // 2 content lines + 2 borders
            BarSize::Large => 5,   // 3 content lines + 2 borders
            BarSize::Custom(height) => height,
        }
    }

    /// Render the bar with basic theme support
    pub fn render(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
        if !self.visible {
            return String::new();
        }

        let height = self.get_height().min(layout.height);
        let width = layout.width;

        let mut output = String::new();

        // Apply theme colors
        let (bg_color, text_color, border_color) = self.get_theme_colors(theme);

        // Top border
        if let Some(border) = self.render_border_line(width, true, &border_color) {
            writeln!(output, "{border}").unwrap();
        }

        // Content lines
        let content_height = height.saturating_sub(self.border_height());
        for line in 0..content_height {
            let line_content = self.render_content_line(width, line, &bg_color, &text_color);
            writeln!(output, "{line_content}").unwrap();
        }

        // Bottom border
        if let Some(border) = self.render_border_line(width, false, &border_color) {
            writeln!(output, "{border}").unwrap();
        }

        output
    }

    /// Render the bar with utility CSS classes
    pub fn render_with_utilities(
        &self,
        layout: &LayoutRect,
        utility_processor: &UtilityProcessor,
    ) -> String {
        if !self.visible {
            return String::new();
        }

        // Process utility classes to get ANSI codes
        let utility_styles = utility_processor.process_classes(&self.css_classes);

        let height = self.get_height().min(layout.height);
        let width = layout.width;

        let mut output = String::new();

        // Apply styles - either from utility classes or fallback to bar style
        let bg_color = if !utility_styles.is_empty() {
            utility_styles.clone()
        } else {
            color_to_ansi(
                self.style.background.unwrap_or(ColorDefinition {
                    r: 240,
                    g: 240,
                    b: 240,
                }),
                false,
            )
        };

        let text_color = color_to_ansi(
            self.style
                .foreground
                .unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }),
            true,
        );

        let border_color = color_to_ansi(
            self.style.border_color.unwrap_or(ColorDefinition {
                r: 128,
                g: 128,
                b: 128,
            }),
            true,
        );

        // Top border
        if let Some(border) = self.render_border_line(width, true, &border_color) {
            writeln!(output, "{bg_color}{border}\x1b[0m").unwrap();
        }

        // Content lines
        let content_height = height.saturating_sub(self.border_height());
        for line in 0..content_height {
            let line_content = self.render_content_line(width, line, &bg_color, &text_color);
            writeln!(output, "{line_content}\x1b[0m").unwrap();
        }

        // Bottom border
        if let Some(border) = self.render_border_line(width, false, &border_color) {
            writeln!(output, "{bg_color}{border}\x1b[0m").unwrap();
        }

        output
    }

    /// Get theme colors with fallbacks
    fn get_theme_colors(&self, theme: Option<&ColorTheme>) -> (String, String, String) {
        let bg_color = if let Some(theme) = theme {
            // Use theme colors based on bar type
            match self.bar_type {
                BarType::Header => color_to_ansi(theme.palette.primary, false),
                BarType::Footer => color_to_ansi(theme.palette.secondary, false),
                BarType::Status => color_to_ansi(theme.palette.success, false),
                _ => color_to_ansi(theme.palette.background, false),
            }
        } else {
            color_to_ansi(
                self.style.background.unwrap_or(ColorDefinition {
                    r: 240,
                    g: 240,
                    b: 240,
                }),
                false,
            )
        };

        let text_color = color_to_ansi(
            self.style
                .foreground
                .unwrap_or(ColorDefinition { r: 0, g: 0, b: 0 }),
            true,
        );

        let border_color = color_to_ansi(
            self.style.border_color.unwrap_or(ColorDefinition {
                r: 128,
                g: 128,
                b: 128,
            }),
            true,
        );

        (bg_color, text_color, border_color)
    }

    /// Get border height
    fn border_height(&self) -> u16 {
        match self.style.border_style {
            BarBorderStyle::None => 0,
            _ => 2, // Top and bottom borders
        }
    }

    /// Render border line
    fn render_border_line(&self, width: u16, is_top: bool, border_color: &str) -> Option<String> {
        let border_char = match self.style.border_style {
            BarBorderStyle::None => return None,
            BarBorderStyle::Single => '─',
            BarBorderStyle::Double => '═',
            BarBorderStyle::Thick => '━',
            BarBorderStyle::Custom(ch) => ch,
        };

        let (left_corner, right_corner) = match self.style.border_style {
            BarBorderStyle::Single => {
                if is_top {
                    ('┌', '┐')
                } else {
                    ('└', '┘')
                }
            }
            BarBorderStyle::Double => {
                if is_top {
                    ('╔', '╗')
                } else {
                    ('╚', '╝')
                }
            }
            BarBorderStyle::Thick => {
                if is_top {
                    ('┏', '┓')
                } else {
                    ('┗', '┛')
                }
            }
            _ => {
                if is_top {
                    ('┌', '┐')
                } else {
                    ('└', '┘')
                }
            }
        };

        let mut line = String::new();
        write!(line, "{border_color}{left_corner}").unwrap();
        for _ in 0..width.saturating_sub(2) {
            line.push(border_char);
        }
        write!(line, "{right_corner}").unwrap();

        Some(line)
    }

    /// Render content line
    fn render_content_line(
        &self,
        width: u16,
        line_index: u16,
        bg_color: &str,
        text_color: &str,
    ) -> String {
        let content_width = width.saturating_sub(2 * self.style.padding);
        let mut line = String::new();

        // Start with background and border
        write!(line, "{bg_color}{text_color}").unwrap();

        if !matches!(self.style.border_style, BarBorderStyle::None) {
            line.push('│');
        }

        // Add padding
        for _ in 0..self.style.padding {
            line.push(' ');
        }

        // Position items on the line
        let positioned_content = self.position_content(content_width, line_index);
        line.push_str(&positioned_content);

        // Add trailing padding
        let used_width = positioned_content.chars().count();
        let remaining_width = content_width.saturating_sub(used_width as u16);
        for _ in 0..remaining_width {
            line.push(' ');
        }

        for _ in 0..self.style.padding {
            line.push(' ');
        }

        if !matches!(self.style.border_style, BarBorderStyle::None) {
            line.push('│');
        }

        line
    }

    /// Position content within the bar
    fn position_content(&self, width: u16, _line_index: u16) -> String {
        if self.items.is_empty() {
            return " ".repeat(width as usize);
        }

        // Group items by position
        let left_items: Vec<&BarItem> = self
            .items
            .iter()
            .filter(|item| item.position == BarPosition::Left)
            .collect();
        let center_items: Vec<&BarItem> = self
            .items
            .iter()
            .filter(|item| item.position == BarPosition::Center)
            .collect();
        let right_items: Vec<&BarItem> = self
            .items
            .iter()
            .filter(|item| item.position == BarPosition::Right)
            .collect();
        let justify_items: Vec<&BarItem> = self
            .items
            .iter()
            .filter(|item| item.position == BarPosition::Justify)
            .collect();

        let result = if !justify_items.is_empty() {
            // Justify positioning - distribute items across width
            self.render_justified_items(&justify_items, width)
        } else {
            // Standard left/center/right positioning
            let left_content = self.render_positioned_items(&left_items);
            let center_content = self.render_positioned_items(&center_items);
            let right_content = self.render_positioned_items(&right_items);

            let left_len = left_content.chars().count();
            let center_len = center_content.chars().count();
            let right_len = right_content.chars().count();

            // Calculate spacing
            let total_content = left_len + center_len + right_len;
            let available_space = width as usize;

            if total_content >= available_space {
                // Content overflows, prioritize left content
                format!("{left_content}{center_content}{right_content}")
                    .chars()
                    .take(available_space)
                    .collect()
            } else {
                // Position content with proper spacing
                let remaining_space = available_space - total_content;

                if center_len > 0 {
                    // Three-section layout
                    let left_padding = if left_len == 0 {
                        (remaining_space / 2).saturating_sub(center_len / 2)
                    } else {
                        remaining_space / 3
                    };
                    let right_padding = remaining_space - left_padding;

                    format!(
                        "{}{}{}{}{}",
                        left_content,
                        " ".repeat(left_padding),
                        center_content,
                        " ".repeat(right_padding.saturating_sub(right_len)),
                        right_content
                    )
                } else {
                    // Two-section layout
                    let middle_padding = remaining_space;
                    format!(
                        "{}{}{}",
                        left_content,
                        " ".repeat(middle_padding),
                        right_content
                    )
                }
            }
        };

        // Ensure exact width
        let final_result = if result.chars().count() > width as usize {
            result.chars().take(width as usize).collect()
        } else if result.chars().count() < width as usize {
            let padding_needed = width as usize - result.chars().count();
            format!("{}{}", result, " ".repeat(padding_needed))
        } else {
            result
        };

        final_result
    }

    /// Render items in justified layout
    fn render_justified_items(&self, items: &[&BarItem], width: u16) -> String {
        if items.is_empty() {
            return " ".repeat(width as usize);
        }

        // Calculate total weight and content length
        let total_weight: f32 = items.iter().map(|item| item.weight).sum();
        let total_content_length: usize = items
            .iter()
            .map(|item| self.render_item_content(item).chars().count())
            .sum();

        let available_space = width as usize;
        if total_content_length >= available_space {
            // Not enough space, just concatenate
            return items
                .iter()
                .map(|item| self.render_item_content(item))
                .collect::<Vec<_>>()
                .join("")
                .chars()
                .take(available_space)
                .collect();
        }

        let remaining_space = available_space - total_content_length;
        let mut result = String::new();

        for (i, item) in items.iter().enumerate() {
            result.push_str(&self.render_item_content(item));

            // Add proportional spacing (except after last item)
            if i < items.len() - 1 {
                let item_space = ((item.weight / total_weight) * remaining_space as f32) as usize;
                result.push_str(&" ".repeat(item_space));
            }
        }

        result
    }

    /// Render positioned items
    fn render_positioned_items(&self, items: &[&BarItem]) -> String {
        items
            .iter()
            .map(|item| self.render_item_content(item))
            .collect::<Vec<_>>()
            .join(
                &self
                    .style
                    .separator
                    .map(|c| format!(" {c} "))
                    .unwrap_or_else(|| " ".to_string()),
            )
    }

    /// Render individual item content
    fn render_item_content(&self, item: &BarItem) -> String {
        let mut content = String::new();

        if let Some(icon) = item.icon {
            content.push(icon);
            content.push(' ');
        }

        content.push_str(&item.content);
        content
    }
}

impl BarBuilder {
    /// Set bar size
    pub fn size(mut self, size: BarSize) -> Self {
        self.bar.size = size;
        self
    }

    /// Add an item to the bar
    pub fn item(mut self, item: BarItem) -> Self {
        self.bar.items.push(item);
        self
    }

    /// Add a left-aligned item
    pub fn left(mut self, content: impl Into<String>) -> Self {
        self.bar
            .items
            .push(BarItem::new(content, BarPosition::Left));
        self
    }

    /// Add a center-aligned item
    pub fn center(mut self, content: impl Into<String>) -> Self {
        self.bar
            .items
            .push(BarItem::new(content, BarPosition::Center));
        self
    }

    /// Add a right-aligned item
    pub fn right(mut self, content: impl Into<String>) -> Self {
        self.bar
            .items
            .push(BarItem::new(content, BarPosition::Right));
        self
    }

    /// Set bar style
    pub fn style(mut self, style: BarStyle) -> Self {
        self.bar.style = style;
        self
    }

    /// Add CSS class
    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.bar.css_classes.push(class.into());
        self
    }

    /// Set background color
    pub fn background(mut self, color: ColorDefinition) -> Self {
        self.bar.style.background = Some(color);
        self
    }

    /// Set text color
    pub fn foreground(mut self, color: ColorDefinition) -> Self {
        self.bar.style.foreground = Some(color);
        self
    }

    /// Set border style
    pub fn border(mut self, style: BarBorderStyle) -> Self {
        self.bar.style.border_style = style;
        self
    }

    /// Set padding
    pub fn padding(mut self, padding: u16) -> Self {
        self.bar.style.padding = padding;
        self
    }

    /// Set sticky behavior
    pub fn sticky(mut self) -> Self {
        self.bar.sticky = true;
        self
    }

    /// Make bar invisible
    pub fn hidden(mut self) -> Self {
        self.bar.visible = false;
        self
    }

    /// Build the bar
    pub fn build(self) -> Bar {
        self.bar
    }
}

/// Convenience functions for common bar types
/// Create a header bar
pub fn header_bar(id: impl Into<String>) -> BarBuilder {
    Bar::builder(id, BarType::Header)
        .background(hex("#3B82F6").unwrap_or(ColorDefinition {
            r: 59,
            g: 130,
            b: 246,
        }))
        .foreground(hex("#FFFFFF").unwrap_or(ColorDefinition {
            r: 255,
            g: 255,
            b: 255,
        }))
}

/// Create a footer bar
pub fn footer_bar(id: impl Into<String>) -> BarBuilder {
    Bar::builder(id, BarType::Footer)
        .background(hex("#6B7280").unwrap_or(ColorDefinition {
            r: 107,
            g: 114,
            b: 128,
        }))
        .foreground(hex("#FFFFFF").unwrap_or(ColorDefinition {
            r: 255,
            g: 255,
            b: 255,
        }))
}

/// Create a status bar
pub fn status_bar(id: impl Into<String>) -> BarBuilder {
    Bar::builder(id, BarType::Status)
        .size(BarSize::Compact)
        .background(hex("#22C55E").unwrap_or(ColorDefinition {
            r: 34,
            g: 197,
            b: 94,
        }))
        .foreground(hex("#FFFFFF").unwrap_or(ColorDefinition {
            r: 255,
            g: 255,
            b: 255,
        }))
}

/// Create a navigation bar
pub fn navigation_bar(id: impl Into<String>) -> BarBuilder {
    Bar::builder(id, BarType::Navigation)
        .background(hex("#1F2937").unwrap_or(ColorDefinition {
            r: 31,
            g: 41,
            b: 55,
        }))
        .foreground(hex("#F9FAFB").unwrap_or(ColorDefinition {
            r: 249,
            g: 250,
            b: 251,
        }))
}

/// Create a toolbar
pub fn toolbar(id: impl Into<String>) -> BarBuilder {
    Bar::builder(id, BarType::Toolbar)
        .background(hex("#F3F4F6").unwrap_or(ColorDefinition {
            r: 243,
            g: 244,
            b: 246,
        }))
        .foreground(hex("#374151").unwrap_or(ColorDefinition {
            r: 55,
            g: 65,
            b: 81,
        }))
        .border(BarBorderStyle::Single)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::themes::UtilityProcessor;

    #[test]
    fn test_bar_creation() {
        let bar = Bar::new("test-bar", BarType::Header);
        assert_eq!(bar.id, "test-bar");
        assert_eq!(bar.bar_type, BarType::Header);
        assert!(bar.visible);
        assert!(!bar.sticky);
    }

    #[test]
    fn test_bar_builder() {
        let bar = Bar::builder("header", BarType::Header)
            .left("App Name")
            .center("Page Title")
            .right("User Info")
            .size(BarSize::Large)
            .sticky()
            .build();

        assert_eq!(bar.items.len(), 3);
        assert_eq!(bar.size, BarSize::Large);
        assert!(bar.sticky);
    }

    #[test]
    fn test_bar_items() {
        let mut bar = Bar::new("test", BarType::Header);

        let item = BarItem::new("Test Item", BarPosition::Left)
            .icon('🏠')
            .id("home")
            .clickable();

        bar.add_item(item);
        assert_eq!(bar.items.len(), 1);

        let retrieved = bar.get_item("home").unwrap();
        assert_eq!(retrieved.content, "Test Item");
        assert_eq!(retrieved.icon, Some('🏠'));
        assert!(retrieved.clickable);
    }

    #[test]
    fn test_bar_rendering() {
        let bar = Bar::builder("test", BarType::Header)
            .left("Left")
            .center("Center")
            .right("Right")
            .build();

        let layout = LayoutRect {
            x: 0,
            y: 0,
            width: 50,
            height: 3,
        };
        let rendered = bar.render(&layout, None);

        assert!(!rendered.is_empty());
        assert!(rendered.contains("Left"));
        assert!(rendered.contains("Center"));
        assert!(rendered.contains("Right"));
    }

    #[test]
    fn test_convenience_functions() {
        let header = header_bar("main-header").build();
        assert_eq!(header.bar_type, BarType::Header);

        let footer = footer_bar("main-footer").build();
        assert_eq!(footer.bar_type, BarType::Footer);

        let status = status_bar("status").build();
        assert_eq!(status.bar_type, BarType::Status);
        assert_eq!(status.size, BarSize::Compact);
    }

    #[test]
    fn test_justified_positioning() {
        let bar = Bar::builder("test", BarType::Header)
            .item(BarItem::new("Item 1", BarPosition::Justify).weight(1.0))
            .item(BarItem::new("Item 2", BarPosition::Justify).weight(2.0))
            .item(BarItem::new("Item 3", BarPosition::Justify).weight(1.0))
            .build();

        let layout = LayoutRect {
            x: 0,
            y: 0,
            width: 40,
            height: 3,
        };
        let rendered = bar.render(&layout, None);

        assert!(!rendered.is_empty());
        assert!(rendered.contains("Item 1"));
        assert!(rendered.contains("Item 2"));
        assert!(rendered.contains("Item 3"));
    }

    #[test]
    fn test_bar_height() {
        let compact = Bar::builder("compact", BarType::Header)
            .size(BarSize::Compact)
            .build();
        assert_eq!(compact.get_height(), 3); // 1 content + 2 borders

        let normal = Bar::builder("normal", BarType::Header)
            .size(BarSize::Normal)
            .build();
        assert_eq!(normal.get_height(), 4); // 2 content + 2 borders

        let large = Bar::builder("large", BarType::Header)
            .size(BarSize::Large)
            .build();
        assert_eq!(large.get_height(), 5); // 3 content + 2 borders

        let custom = Bar::builder("custom", BarType::Header)
            .size(BarSize::Custom(5))
            .build();
        assert_eq!(custom.get_height(), 5);
    }

    #[test]
    fn test_utility_css_rendering() {
        let bar = Bar::builder("test", BarType::Header)
            .left("Test")
            .class("bg-blue-500")
            .class("text-white")
            .build();

        let layout = LayoutRect {
            x: 0,
            y: 0,
            width: 30,
            height: 3,
        };
        let utility_processor = UtilityProcessor::new();
        let rendered = bar.render_with_utilities(&layout, &utility_processor);

        assert!(!rendered.is_empty());
        assert!(rendered.contains("Test"));
    }
}
