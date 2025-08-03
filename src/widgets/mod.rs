//! Widget library for advanced UI components
//!
//! This module provides a rich set of pre-built widgets that integrate
//! seamlessly with the TUI framework's reactive state management,
//! CSS styling, and event systems.

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
