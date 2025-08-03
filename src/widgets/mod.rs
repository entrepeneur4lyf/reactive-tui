//! # Widget Library
//!
//! Comprehensive collection of interactive UI components for terminal applications.
//!
//! This module provides 25+ pre-built widgets that integrate seamlessly with the TUI
//! framework's reactive state management, CSS styling, and event systems. All widgets
//! implement responsive design principles and support theming, accessibility, and
//! keyboard navigation.
//!
//! ## Widget Categories
//!
//! ### Layout Widgets
//! - [`Grid`](grid): Advanced grid layouts with column/row definitions
//! - [`Bar`](bar): Header/footer bars with flexible positioning  
//! - [`Tabs`](tabs): Tab navigation with multiple orientations
//! - [`Modal`](modal): Overlay dialogs with backdrop
//! - [`Accordion`](accordion): Expandable/collapsible sections
//!
//! ### Form Controls
//! - [`Input`](input): Text input with validation
//! - [`Button`](button): Interactive buttons with states
//! - [`Checkbox`](checkbox): Single and grouped checkboxes
//! - [`Switch`](switch): Toggle switches with labels
//! - [`Radio`](radio): Radio button groups
//! - [`Select`](select): Dropdown selection with search
//! - [`Autocomplete`](autocomplete): Type-ahead search input
//! - [`Slider`](slider): Range sliders with ticks
//!
//! ### Data Display
//! - [`DataTable`](datatable): Sortable, filterable tables with pagination
//! - [`Tree`](tree): Hierarchical tree with lazy loading
//! - [`ScrollableList`](scrollable_list): Virtual scrolling lists
//! - [`Progress`](progress): Progress bars with animations
//! - [`Spinner`](spinner): Loading indicators (30+ types)
//!
//! ### Content Widgets
//! - [`RichText`](rich_text): Markdown rendering with syntax highlighting
//! - [`Textarea`](textarea): Multi-line text editing with vim-like features
//! - [`Viewport`](viewport): Scrollable areas with virtual rendering
//!
//! ### Feedback Widgets
//! - [`Toast`](toast): Notification toasts with positioning
//! - [`FormValidator`](form_validation): Real-time form validation
//!
//! ### Advanced Features
//! - [`Animation`](animation): Property animations with easing
//! - [`Theme`](../themes): JSON-based theming system
//! - [`Plugin`](../plugin): Extensible widget architecture
//!
//! ## Usage Patterns
//!
//! ### Basic Widget Creation
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! // Create a button with builder pattern
//! let button = Button::builder("my_button", "Click Me")
//!     .button_type(ButtonType::Primary)
//!     .size(ButtonSize::Medium)
//!     .build();
//!
//! // Convert to element for layout
//! let element = button.to_element();
//! ```
//!
//! ### Responsive Design
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! // Widget automatically adapts to container size
//! let user_data = vec![
//!     vec!["Alice".to_string(), "25".to_string(), "alice@example.com".to_string()],
//!     vec!["Bob".to_string(), "30".to_string(), "bob@example.com".to_string()],
//! ];
//!
//! let table = DataTableBuilder::new("user_table")
//!     .column(Column::new("name", "Name").width(100).sortable(true))
//!     .column(Column::new("age", "Age").width(60).sortable(true))
//!     .column(Column::new("email", "Email").width(200).sortable(true))
//!     .data(user_data)
//!     .sortable(true)
//!     .filterable(true)
//!     .build();
//!
//! // Responsive behavior handled automatically via ResponsiveWidget trait
//! let element = table.to_element();
//! ```
//!
//! ### Widget Composition
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! // Combine multiple widgets in layouts
//! let input = Input::builder("user_name")
//!     .placeholder("Enter name")
//!     .required(true)
//!     .build();
//!
//! let button = Button::builder("submit_btn", "Submit")
//!     .button_type(ButtonType::Success)
//!     .build();
//!
//! let form = Element::with_tag("form")
//!     .class("user-form")
//!     .child(Element::with_tag("input").id("user_name").build())
//!     .child(button.to_element())
//!     .build();
//! ```

use crate::components::Element;
use crate::layout::LayoutRect;
use crate::themes::ColorTheme;

// Widget Factory Pattern
pub mod factory;
pub use factory::*;

/// # Responsive Widget Trait
///
/// Core trait for widgets that support responsive layout and dynamic sizing.
///
/// `ResponsiveWidget` enables widgets to participate in the CSS-based layout system
/// by providing size constraints, growth behavior, and rendering capabilities. All
/// built-in widgets implement this trait to ensure consistent behavior across the
/// widget library.
///
/// ## Implementation Requirements
///
/// Widgets must be able to:
/// - Convert themselves to DOM-like elements for CSS styling
/// - Render with computed layouts from the layout engine
/// - Declare sizing constraints and growth preferences
/// - Adapt to different terminal sizes responsively
///
/// ## Examples
///
/// ### Basic Implementation
///
/// ```rust,no_run
/// use reactive_tui::prelude::*;
/// use reactive_tui::widgets::ResponsiveWidget;
///
/// struct CustomWidget {
///     content: String,
///     min_width: u16,
/// }
///
/// impl ResponsiveWidget for CustomWidget {
///     fn to_element(&self) -> Element {
///         Element::with_tag("div")
///             .class("custom-widget")
///             .content(&self.content)
///             .build()
///     }
///
///     fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
///         // Render widget within the computed layout bounds
///         format!("Custom: {} at {}x{}", self.content, layout.width, layout.height)
///     }
///
///     fn min_size(&self) -> (u16, u16) {
///         (self.min_width, 1)
///     }
/// }
/// ```
///
/// ### Size-Constrained Widget
///
/// ```rust,no_run
/// use reactive_tui::prelude::*;
/// use reactive_tui::widgets::ResponsiveWidget;
///
/// struct FixedSizeWidget;
///
/// impl ResponsiveWidget for FixedSizeWidget {
///     fn to_element(&self) -> Element {
///         Element::with_tag("div")
///             .class("fixed-widget")
///             .build()
///     }
///
///     fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
///         "Fixed size content".to_string()
///     }
///
///     fn min_size(&self) -> (u16, u16) {
///         (20, 5) // Minimum 20x5 characters
///     }
///
///     fn max_size(&self) -> (Option<u16>, Option<u16>) {
///         (Some(40), Some(10)) // Maximum 40x10 characters
///     }
///
///     fn can_grow_horizontal(&self) -> bool {
///         false // Fixed horizontal size
///     }
/// }
/// ```
pub trait ResponsiveWidget {
  /// Converts the widget to a DOM-like element for CSS styling and layout computation.
  ///
  /// The returned element should contain all necessary CSS classes, attributes, and
  /// child elements that represent the widget's structure. The layout engine uses
  /// this element to compute CSS styles and positioning.
  ///
  /// # Returns
  ///
  /// An [`Element`] representing the widget's DOM structure
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::prelude::*;
  /// use reactive_tui::widgets::ResponsiveWidget;
  ///
  /// struct MyButton {
  ///     label: String,
  /// }
  ///
  /// impl ResponsiveWidget for MyButton {
  ///     fn to_element(&self) -> Element {
  ///         Element::with_tag("button")
  ///             .class("btn")
  ///             .class("btn-primary")
  ///             .attr("role", "button")
  ///             .content(&self.label)
  ///             .build()
  ///     }
  ///     # fn render_with_layout(&self, layout: &reactive_tui::layout::LayoutRect, theme: Option<&reactive_tui::themes::ColorTheme>) -> String { String::new() }
  /// }
  /// ```
  fn to_element(&self) -> Element;

  /// Renders the widget with a computed layout from the layout engine.
  ///
  /// This method is called after the CSS engine and layout engine have computed
  /// the final size, position, and styling for the widget. The implementation
  /// should render the widget's visual representation within the provided bounds.
  ///
  /// # Arguments
  ///
  /// * `layout` - Computed layout rectangle with position and dimensions
  /// * `theme` - Optional theme for color and styling information
  ///
  /// # Returns
  ///
  /// A string containing the rendered widget content (typically ANSI-escaped text)
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::prelude::*;
  /// use reactive_tui::widgets::ResponsiveWidget;
  /// use reactive_tui::themes::*;
  ///
  /// struct MyWidget {
  ///     content: String,
  /// }
  ///
  /// impl ResponsiveWidget for MyWidget {
  ///     fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String {
  ///         let default_color = rgb(0, 122, 255);
  ///         let bg_color = theme
  ///             .map(|t| &t.palette.primary)
  ///             .unwrap_or(&default_color);
  ///         
  ///         format!("{}Content in {}x{} area{}",
  ///             color_to_ansi(*bg_color, true),
  ///             layout.width, layout.height,
  ///             RESET_COLOR)
  ///     }
  ///     # fn to_element(&self) -> Element { Element::with_tag("div").build() }
  /// }
  /// ```
  fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String;

  /// Returns the widget's preferred minimum size in terminal characters.
  ///
  /// Used by the layout engine for responsive calculations and constraint solving.
  /// The widget should never be rendered smaller than this size.
  ///
  /// # Returns
  ///
  /// A tuple of `(width, height)` in terminal characters
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::widgets::ResponsiveWidget;
  ///
  /// struct MyWidget;
  ///
  /// impl ResponsiveWidget for MyWidget {
  ///     fn min_size(&self) -> (u16, u16) {
  ///         (10, 3) // Minimum 10 characters wide, 3 lines tall
  ///     }
  ///     # fn to_element(&self) -> reactive_tui::components::Element { reactive_tui::components::Element::with_tag("div").build() }
  ///     # fn render_with_layout(&self, layout: &reactive_tui::layout::LayoutRect, theme: Option<&reactive_tui::themes::ColorTheme>) -> String { String::new() }
  /// }
  /// ```
  fn min_size(&self) -> (u16, u16) {
    (1, 1) // Default minimum size
  }

  /// Returns the widget's preferred maximum size in terminal characters.
  ///
  /// `None` means no maximum limit (can grow infinitely). Used by the layout
  /// engine to prevent widgets from growing beyond reasonable bounds.
  ///
  /// # Returns
  ///
  /// A tuple of `(Option<width>, Option<height>)` in terminal characters
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::widgets::ResponsiveWidget;
  ///
  /// struct MyWidget;
  ///
  /// impl ResponsiveWidget for MyWidget {
  ///     fn max_size(&self) -> (Option<u16>, Option<u16>) {
  ///         (Some(80), None) // Maximum 80 characters wide, unlimited height
  ///     }
  ///     # fn to_element(&self) -> reactive_tui::components::Element { reactive_tui::components::Element::with_tag("div").build() }
  ///     # fn render_with_layout(&self, layout: &reactive_tui::layout::LayoutRect, theme: Option<&reactive_tui::themes::ColorTheme>) -> String { String::new() }
  /// }
  /// ```
  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    (None, None) // Default: no maximum size
  }

  /// Indicates whether the widget can grow horizontally beyond its minimum size.
  ///
  /// Used by flexbox and grid layouts to determine how to distribute extra space.
  /// Widgets that return `false` will maintain their minimum or intrinsic width.
  ///
  /// # Returns
  ///
  /// `true` if the widget can grow horizontally, `false` otherwise
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::widgets::ResponsiveWidget;
  ///
  /// struct FixedWidthWidget;
  ///
  /// impl ResponsiveWidget for FixedWidthWidget {
  ///     fn can_grow_horizontal(&self) -> bool {
  ///         false // Always maintain exact width
  ///     }
  ///     # fn to_element(&self) -> reactive_tui::components::Element { reactive_tui::components::Element::with_tag("div").build() }
  ///     # fn render_with_layout(&self, layout: &reactive_tui::layout::LayoutRect, theme: Option<&reactive_tui::themes::ColorTheme>) -> String { String::new() }
  /// }
  /// ```
  fn can_grow_horizontal(&self) -> bool {
    true
  }

  /// Indicates whether the widget can grow vertically beyond its minimum size.
  ///
  /// Used by flexbox and grid layouts to determine how to distribute extra space.
  /// Widgets that return `false` will maintain their minimum or intrinsic height.
  ///
  /// # Returns
  ///
  /// `true` if the widget can grow vertically, `false` otherwise
  ///
  /// # Examples
  ///
  /// ```rust,no_run
  /// use reactive_tui::widgets::ResponsiveWidget;
  ///
  /// struct FixedHeightWidget;
  ///
  /// impl ResponsiveWidget for FixedHeightWidget {
  ///     fn can_grow_vertical(&self) -> bool {
  ///         false // Always maintain exact height
  ///     }
  ///     # fn to_element(&self) -> reactive_tui::components::Element { reactive_tui::components::Element::with_tag("div").build() }
  ///     # fn render_with_layout(&self, layout: &reactive_tui::layout::LayoutRect, theme: Option<&reactive_tui::themes::ColorTheme>) -> String { String::new() }
  /// }
  /// ```
  fn can_grow_vertical(&self) -> bool {
    true
  }
}

pub mod accordion;
pub mod animation;
pub mod autocomplete;
pub mod bar;
pub mod button;
pub mod checkbox;
pub mod datatable;
pub mod form_validation;
pub mod input;
pub mod menu;
pub mod modal;
pub mod overlay;
pub mod progress;
pub mod radio;
pub mod rich_text;
pub mod scrollable_list;
pub mod select;
pub mod slider;
pub mod spinner;
pub mod switch;
pub mod tabs;
pub mod textarea;
pub mod toast;
pub mod tree;
pub mod viewport;

pub use accordion::{
  compact_accordion, faq_accordion, settings_accordion, Accordion, AccordionAnimation,
  AccordionBuilder, AccordionConfig, AccordionSection, AccordionSectionStyle, AccordionState,
  AnimationEasing, AnimationState, SectionId,
};
pub use animation::{
  bounce, fade_in, fade_out, pulse, slide_in_left, AnimatedProperty, AnimatedValue, Animation,
  AnimationBuilder, AnimationCallbacks, AnimationConfig, AnimationId, AnimationManager,
  AnimationRuntimeState, AnimationState as AnimationPlayState, AnimationTimeline,
  EasingFunction as TweenEasing, LoopMode, TimelineId,
};
pub use autocomplete::{
  command_autocomplete, country_autocomplete, language_autocomplete, user_autocomplete,
  Autocomplete, AutocompleteBuilder, AutocompleteCallbacks, AutocompleteConfig, AutocompleteState,
  AutocompleteStyle, AutocompleteSuggestion, FilterMode, SelectionMode, SuggestionId,
};
pub use bar::{
  footer_bar, header_bar, navigation_bar, status_bar, toolbar, Bar, BarBorderStyle, BarBuilder,
  BarItem, BarPosition, BarSize, BarStyle, BarType,
};
pub use button::{
  Button, ButtonBorderStyle, ButtonBuilder, ButtonSize, ButtonState, ButtonStyle, ButtonType,
  IconPosition,
};
pub use checkbox::{
  checkbox, checkbox_group, custom_checkbox, horizontal_checkbox_group, simple_checkbox,
  simple_checkbox_group, Checkbox, CheckboxAnimationConfig, CheckboxAnimationState,
  CheckboxBuilder, CheckboxGroup, CheckboxGroupBuilder, CheckboxGroupOrientation,
  CheckboxGroupState, CheckboxLabelPosition, CheckboxOption, CheckboxState, CheckboxStyle,
};
pub use datatable::{
  Column, ColumnAlignment, ColumnId, DataTable, DataTableBuilder, DataTableConfig, DataTableState,
  PaginationState, RowFilter, RowId, SortOrder, SortState,
};
pub use form_validation::{
  contact_form, login_form, user_registration_form, FieldId, FieldType, FormField,
  FormValidationCallbacks, FormValidationConfig, FormValidationState, FormValidationStyle,
  FormValidator, FormValidatorBuilder, ValidationMessage, ValidationResult, ValidationRule,
  ValidationSeverity, ValidationTiming,
};
pub use input::{Input, InputBuilder, InputState, InputStyle, InputType, ValidationState};
pub use menu::{
  context_menu, dropdown_menu, menu_bar, Menu, MenuBuilder, MenuItem, MenuItemState, MenuItemType,
  MenuOrientation, MenuState, MenuStyle,
};
pub use modal::{
  alert_modal, confirm_modal, custom_modal, fullscreen_modal, prompt_modal, Modal, ModalBackdrop,
  ModalBuilder, ModalButton, ModalPosition, ModalSize, ModalStyle, ModalType,
};
pub use overlay::{OverlayManager, OverlayPosition, OverlayStyle};
pub use progress::{
  spinners, EasingFunction, ProgressAnimation, ProgressBar, ProgressBarBuilder, ProgressColors,
  ProgressManager, ProgressMessage, ProgressState, ProgressStyle,
};
pub use radio::{
  radio_group, RadioGroup, RadioGroupBuilder, RadioOption, RadioOrientation, RadioState, RadioStyle,
};
pub use rich_text::{
  code_preview, documentation_viewer, help_text, readme_viewer, MarkdownElement, RichText,
  RichTextBuilder, RichTextCallbacks, RichTextConfig, RichTextState, RichTextStyle, SyntaxLanguage,
  SyntaxPattern, SyntaxPatternType, TableAlignment,
};
pub use scrollable_list::{
  file_browser_list, menu_list, task_list, ListItem, ScrollableList, ScrollableListBuilder,
  ScrollableListCallbacks, ScrollableListConfig, ScrollableListState, ScrollableListStyle,
  SelectionMode as ListSelectionMode,
};
pub use select::{
  DropdownPosition, Select, SelectBuilder, SelectMode, SelectOption, SelectState, SelectStyle,
};
pub use slider::{
  Slider, SliderBuilder, SliderMode, SliderOrientation, SliderState, SliderStyle, SliderTicks,
};
pub use spinner::{
  loading_spinner, processing_spinner, saving_spinner, spinner, Spinner, SpinnerAnimationState,
  SpinnerBuilder, SpinnerDefinition, SpinnerLabelPosition, SpinnerState, SpinnerStyle, SpinnerType,
};
pub use switch::{switch, LabelPosition, Switch, SwitchBuilder, SwitchState, SwitchStyle};
pub use tabs::{
  bottom_tabs, card_tabs, horizontal_tabs, minimal_tabs, vertical_tabs, Tab, TabBorderStyle,
  TabOrientation, TabPosition, TabSize, TabStyle, Tabs, TabsBuilder,
};
pub use textarea::{
  CursorPosition, EditOperation, History, Search, Selection, Textarea, TextareaBuilder,
  TextareaState, TextareaStyle, YankBuffer,
};
pub use toast::{Toast, ToastBuilder, ToastManager, ToastVariant};
pub use tree::{
  LazyLoader, NodeId, Tree, TreeBuilder, TreeConfig, TreeIndentChars, TreeNode, TreeNodeStyle,
  TreeNodeType, TreeState,
};
pub use viewport::{
  data_table_viewport, file_viewer, log_viewer, ContentId, LazyLoadState, ScrollMode,
  ScrollbarPosition, SelectionMode as ViewportSelectionMode, Viewport, ViewportBuilder,
  ViewportCallbacks, ViewportConfig, ViewportItem, ViewportState, ViewportStyle,
};

// Re-export basic CLI components from components module
pub use crate::components::{
  border, center, code, container, div, flex_column, flex_row, footer, header, hr, left, line,
  list, main, padding, pre, right, section, separator, spacer, span, text,
};

// ResponsiveWidget trait is defined above and automatically available
