//! Widget demonstration screen component

use reactive_tui::prelude::*;
use reactive_tui::components::{div, text, section};
use reactive_tui::widgets::*;
use crate::app_state::{WidgetType, GuideMode};

/// Widget demonstration screen with documentation and interactive modes
#[derive(Debug, Clone)]
pub struct WidgetDemoScreen {
    widget_type: WidgetType,
    mode: GuideMode,
}

impl WidgetDemoScreen {
    pub fn new(widget_type: WidgetType, mode: GuideMode) -> Self {
        Self { widget_type, mode }
    }
}

impl Component for WidgetDemoScreen {
    fn render(&self) -> Element {
        div()
            .class("flex")
            .class("flex-col")
            .class("h-full")
            .class("bg-gray-50")
            .child_builder(
                // Header with widget name and mode toggle
                section()
                    .class("bg-white")
                    .class("border-b")
                    .class("border-gray-200")
                    .class("p-4")
                    .child_builder(
                        div()
                            .class("flex")
                            .class("items-center")
                            .class("justify-between")
                            .child_builder(
                                div()
                                    .class("flex")
                                    .class("items-center")
                                    .child_builder(
                                        text(self.widget_type.display_name())
                                            .class("text-3xl")
                                            .class("font-bold")
                                            .class("text-gray-800")
                                            .class("mr-4")
                                    )
                                    .child_builder(
                                        text(format!("Category: {}", self.widget_type.category().display_name()))
                                            .class("text-sm")
                                            .class("text-gray-500")
                                            .class("bg-gray-100")
                                            .class("px-3")
                                            .class("py-1")
                                            .class("rounded-full")
                                    )
                            )
                            .child_builder(
                                div()
                                    .class("flex")
                                    .class("items-center")
                                    .class("space-x-2")
                                    .child_builder(
                                        text(match self.mode {
                                            GuideMode::Documentation => "📖 Documentation",
                                            GuideMode::Interactive => "🎮 Interactive Demo",
                                        })
                                        .class("text-lg")
                                        .class("font-semibold")
                                        .class(match self.mode {
                                            GuideMode::Documentation => "text-blue-600",
                                            GuideMode::Interactive => "text-green-600",
                                        })
                                    )
                                    .child_builder(
                                        text("(Press TAB to toggle)")
                                            .class("text-sm")
                                            .class("text-gray-400")
                                    )
                            )
                    )
            )
            .child_builder(
                // Main content area
                section()
                    .class("flex-1")
                    .class("p-6")
                    .child_builder(match self.mode {
                        GuideMode::Documentation => self.render_documentation(),
                        GuideMode::Interactive => self.render_interactive_demo(),
                    })
            )
            .build()
    }

    fn render_documentation(&self) -> Element {
        div()
            .class("bg-white")
            .class("rounded-lg")
            .class("shadow-md")
            .class("p-6")
            .class("h-full")
            .class("overflow-auto")
            .child_builder(
                div()
                    .class("prose")
                    .class("max-w-none")
                    .child_builder(
                        text(format!("# {} Widget Documentation", self.widget_type.display_name()))
                            .class("text-2xl")
                            .class("font-bold")
                            .class("text-gray-800")
                            .class("mb-6")
                    )
                    .child_builder(
                        text(self.get_widget_description())
                            .class("text-lg")
                            .class("text-gray-600")
                            .class("mb-8")
                            .class("leading-relaxed")
                    )
                    .child_builder(
                        div()
                            .class("bg-gray-100")
                            .class("p-4")
                            .class("rounded-lg")
                            .class("mb-6")
                            .child_builder(
                                text("## Basic Usage")
                                    .class("text-xl")
                                    .class("font-semibold")
                                    .class("text-gray-800")
                                    .class("mb-4")
                            )
                            .child_builder(
                                text(self.get_example_code())
                                    .class("font-mono")
                                    .class("text-sm")
                                    .class("bg-gray-800")
                                    .class("text-green-400")
                                    .class("p-4")
                                    .class("rounded")
                                    .class("whitespace-pre")
                            )
                    )
                    .child_builder(
                        div()
                            .class("grid")
                            .class("grid-cols-2")
                            .class("gap-6")
                            .child_builder(
                                div()
                                    .class("bg-blue-50")
                                    .class("p-4")
                                    .class("rounded-lg")
                                    .child_builder(
                                        text("✨ Key Features")
                                            .class("text-lg")
                                            .class("font-semibold")
                                            .class("text-blue-800")
                                            .class("mb-3")
                                    )
                                    .child_builder(
                                        div()
                                            .class("space-y-2")
                                            .child_builder(text("• Responsive design"))
                                            .child_builder(text("• CSS styling support"))
                                            .child_builder(text("• Keyboard navigation"))
                                            .child_builder(text("• Event handling"))
                                            .child_builder(text("• Accessibility features"))
                                    )
                            )
                            .child_builder(
                                div()
                                    .class("bg-green-50")
                                    .class("p-4")
                                    .class("rounded-lg")
                                    .child_builder(
                                        text("⚙️ Configuration")
                                            .class("text-lg")
                                            .class("font-semibold")
                                            .class("text-green-800")
                                            .class("mb-3")
                                    )
                                    .child_builder(
                                        div()
                                            .class("space-y-2")
                                            .child_builder(text("• Builder pattern"))
                                            .child_builder(text("• Factory functions"))
                                            .child_builder(text("• Custom styling"))
                                            .child_builder(text("• Event callbacks"))
                                            .child_builder(text("• State management"))
                                    )
                            )
                    )
            )
            .build()
    }

    fn render_interactive_demo(&self) -> Element {
        div()
            .class("grid")
            .class("grid-cols-2")
            .class("gap-6")
            .class("h-full")
            .child_builder(
                // Widget demo area
                div()
                    .class("bg-white")
                    .class("rounded-lg")
                    .class("shadow-md")
                    .class("p-6")
                    .child_builder(
                        text("🎮 Live Widget Demo")
                            .class("text-xl")
                            .class("font-semibold")
                            .class("text-gray-800")
                            .class("mb-4")
                    )
                    .child_builder(
                        div()
                            .class("border")
                            .class("border-gray-200")
                            .class("rounded-lg")
                            .class("p-8")
                            .class("bg-gray-50")
                            .class("min-h-64")
                            .class("flex")
                            .class("items-center")
                            .class("justify-center")
                            .child_builder(self.render_widget_instance())
                    )
            )
            .child_builder(
                // Controls and configuration
                div()
                    .class("bg-white")
                    .class("rounded-lg")
                    .class("shadow-md")
                    .class("p-6")
                    .child_builder(
                        text("⚙️ Widget Configuration")
                            .class("text-xl")
                            .class("font-semibold")
                            .class("text-gray-800")
                            .class("mb-4")
                    )
                    .child_builder(
                        div()
                            .class("space-y-4")
                            .child_builder(self.render_widget_controls())
                    )
            )
            .build()
    }

    fn render_widget_instance(&self) -> Element {
        match self.widget_type {
            WidgetType::Button => {
                Button::builder("demo-button", "Click Me!")
                    .button_type(ButtonType::Primary)
                    .size(ButtonSize::Large)
                    .build()
                    .to_element()
            }
            WidgetType::Input => {
                Input::builder("demo-input")
                    .placeholder("Enter text here...")
                    .build()
                    .to_element()
            }
            WidgetType::Progress => {
                Progress::builder("demo-progress")
                    .value(65)
                    .max(100)
                    .label("Demo Progress")
                    .build()
                    .to_element()
            }
            _ => {
                text(format!("{} Widget Demo", self.widget_type.display_name()))
                    .class("text-2xl")
                    .class("text-gray-600")
                    .class("text-center")
                    .build()
            }
        }
    }

    fn render_widget_controls(&self) -> Element {
        div()
            .class("space-y-4")
            .child_builder(
                text("Interactive controls will be available here")
                    .class("text-gray-600")
                    .class("italic")
            )
            .child_builder(
                text("• Modify widget properties")
                    .class("text-sm")
                    .class("text-gray-500")
            )
            .child_builder(
                text("• See real-time updates")
                    .class("text-sm")
                    .class("text-gray-500")
            )
            .child_builder(
                text("• Copy generated code")
                    .class("text-sm")
                    .class("text-gray-500")
            )
            .build()
    }

    fn get_widget_description(&self) -> String {
        match self.widget_type {
            WidgetType::Button => "Interactive buttons with multiple variants, states, and styling options. Supports icons, different sizes, and comprehensive event handling.".to_string(),
            WidgetType::Input => "Text input fields with validation, formatting, and accessibility features. Supports various input types and real-time validation.".to_string(),
            WidgetType::Progress => "Progress bars and indicators with animations and multiple display styles. Perfect for showing task completion and loading states.".to_string(),
            _ => format!("{} widget with comprehensive functionality and styling options.", self.widget_type.display_name()),
        }
    }

    fn get_example_code(&self) -> String {
        match self.widget_type {
            WidgetType::Button => r#"use reactive_tui::widgets::*;

let button = Button::builder("my-button", "Click Me!")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .icon("🚀", IconPosition::Left)
    .on_click("handle_click")
    .build();"#.to_string(),
            WidgetType::Input => r#"use reactive_tui::widgets::*;

let input = Input::builder("username")
    .placeholder("Enter username")
    .required(true)
    .max_length(20)
    .validation_pattern(r"^[a-zA-Z0-9_]+$")
    .build();"#.to_string(),
            WidgetType::Progress => r#"use reactive_tui::widgets::*;

let progress = Progress::builder("download")
    .value(65)
    .max(100)
    .label("Downloading...")
    .show_percentage(true)
    .build();"#.to_string(),
            _ => format!("// {} example\nlet widget = {}::new();", self.widget_type.display_name(), self.widget_type.display_name()),
        }
    }
}
