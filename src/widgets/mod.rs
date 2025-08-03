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
//! let button = ButtonBuilder::new()
//!     .content("Click Me")
//!     .button_type(ButtonType::Primary)
//!     .size(ButtonSize::Medium)
//!     .build()?;
//!
//! // Convert to element for layout
//! let element = button.to_element();
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Responsive Design
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! // Widget automatically adapts to container size
//! let table = DataTableBuilder::new()
//!     .columns(vec!["Name", "Age", "Email"])
//!     .data(user_data)
//!     .sortable(true)
//!     .filterable(true)
//!     .build()?;
//!
//! // Responsive behavior handled automatically
//! let (min_width, min_height) = table.min_size();
//! let (max_width, max_height) = table.max_size();
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```
//!
//! ### Widget Composition
//!
//! ```rust,no_run
//! use reactive_tui::prelude::*;
//! use reactive_tui::widgets::*;
//!
//! // Combine multiple widgets in layouts
//! let form = ElementBuilder::new("form")
//!     .class("user-form")
//!     .child(
//!         InputBuilder::new()
//!             .placeholder("Enter name")
//!             .validation_required()
//!             .build()?
//!             .to_element()
//!     )
//!     .child(
//!         ButtonBuilder::new()
//!             .content("Submit")
//!             .button_type(ButtonType::Success)
//!             .build()?
//!             .to_element()
//!     )
//!     .build();
//! # Ok::<(), reactive_tui::error::TuiError>(())
//! ```

use crate::components::Element;
use crate::layout::LayoutRect;
use crate::themes::ColorTheme;

// Widget Factory Pattern
pub mod factory;
pub use factory::*;

/// Trait for widgets that can be converted to Elements for responsive layout
pub trait ResponsiveWidget {
  /// Convert the widget to an Element for layout computation
  /// The Element should contain CSS classes and attributes for styling
  fn to_element(&self) -> Element;

  /// Render the widget with a computed layout from the LayoutEngine
  /// This is called after the LayoutEngine computes the final size and position
  fn render_with_layout(&self, layout: &LayoutRect, theme: Option<&ColorTheme>) -> String;

  /// Get the widget's preferred minimum size (width, height)
  /// Used by the LayoutEngine for responsive calculations
  fn min_size(&self) -> (u16, u16) {
    (1, 1) // Default minimum size
  }

  /// Get the widget's preferred maximum size (width, height)
  /// None means no maximum (can grow infinitely)
  fn max_size(&self) -> (Option<u16>, Option<u16>) {
    (None, None) // Default: no maximum size
  }

  /// Whether the widget can grow horizontally
  fn can_grow_horizontal(&self) -> bool {
    true
  }

  /// Whether the widget can grow vertically
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
