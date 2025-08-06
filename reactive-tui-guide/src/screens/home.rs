//! Home screen component

use reactive_tui::prelude::*;
use reactive_tui::components::{div, text, section, header};
use crate::app_state::WidgetCategory;

/// Home screen component with framework overview and widget categories
#[derive(Debug, Clone)]
pub struct HomeScreen;

impl HomeScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Component for HomeScreen {
    fn render(&self) -> Element {
        div()
            .class("flex")
            .class("flex-col")
            .class("h-full")
            .class("p-6")
            .class("bg-gray-50")
            .child_builder(
                header()
                    .class("mb-8")
                    .child_builder(
                        text("Welcome to Reactive TUI")
                            .class("text-4xl")
                            .class("font-bold")
                            .class("text-blue-900")
                            .class("mb-2")
                    )
                    .child_builder(
                        text("Explore 29+ interactive widgets and comprehensive documentation")
                            .class("text-xl")
                            .class("text-gray-600")
                    )
            )
            .child_builder(
                section()
                    .class("grid")
                    .class("grid-cols-2")
                    .class("gap-8")
                    .class("mb-8")
                    .child_builder(
                        div()
                            .class("bg-white")
                            .class("p-6")
                            .class("rounded-lg")
                            .class("shadow-md")
                            .class("border")
                            .class("border-gray-200")
                            .child_builder(
                                text("🚀 Framework Features")
                                    .class("text-2xl")
                                    .class("font-semibold")
                                    .class("text-blue-800")
                                    .class("mb-4")
                            )
                            .child_builder(
                                div()
                                    .class("space-y-2")
                                    .child_builder(text("• CSS-styled terminal interfaces"))
                                    .child_builder(text("• Reactive state management"))
                                    .child_builder(text("• 60 FPS adaptive rendering"))
                                    .child_builder(text("• TypeScript + Rust architecture"))
                                    .child_builder(text("• Full accessibility support"))
                            )
                    )
                    .child_builder(
                        div()
                            .class("bg-white")
                            .class("p-6")
                            .class("rounded-lg")
                            .class("shadow-md")
                            .class("border")
                            .class("border-gray-200")
                            .child_builder(
                                text("📊 Performance Metrics")
                                    .class("text-2xl")
                                    .class("font-semibold")
                                    .class("text-green-800")
                                    .class("mb-4")
                            )
                            .child_builder(
                                div()
                                    .class("space-y-2")
                                    .child_builder(text("• Target: 60 FPS rendering"))
                                    .child_builder(text("• Memory: <50MB typical usage"))
                                    .child_builder(text("• Response: <100ms interactions"))
                                    .child_builder(text("• Flicker-free updates"))
                                    .child_builder(text("• Adaptive framerate"))
                            )
                    )
            )
            .child_builder(
                section()
                    .class("mb-8")
                    .child_builder(
                        text("Widget Categories")
                            .class("text-3xl")
                            .class("font-bold")
                            .class("text-gray-800")
                            .class("mb-6")
                    )
                    .child_builder(
                        div()
                            .class("grid")
                            .class("grid-cols-2")
                            .class("gap-6")
                            .child_builder(self.create_category_card(
                                WidgetCategory::Layout,
                                "🏗️",
                                "Layout & Navigation",
                                "Grid, Bar, Tabs, Modal, Accordion, Panel",
                                "6 widgets"
                            ))
                            .child_builder(self.create_category_card(
                                WidgetCategory::FormControls,
                                "📝",
                                "Form Controls",
                                "Button, Input, Checkbox, Select, Slider, Switch",
                                "8 widgets"
                            ))
                            .child_builder(self.create_category_card(
                                WidgetCategory::DataDisplay,
                                "📊",
                                "Data Display",
                                "DataTable, Tree, Progress, Spinner, Viewport",
                                "6 widgets"
                            ))
                            .child_builder(self.create_category_card(
                                WidgetCategory::ContentFeedback,
                                "🔔",
                                "Content & Feedback",
                                "RichText, Toast, Animation, Menu, Image",
                                "9 widgets"
                            ))
                    )
            )
            .child_builder(
                section()
                    .class("bg-blue-100")
                    .class("p-6")
                    .class("rounded-lg")
                    .class("border")
                    .class("border-blue-200")
                    .child_builder(
                        text("🎯 Getting Started")
                            .class("text-2xl")
                            .class("font-semibold")
                            .class("text-blue-800")
                            .class("mb-4")
                    )
                    .child_builder(
                        div()
                            .class("grid")
                            .class("grid-cols-3")
                            .class("gap-4")
                            .child_builder(
                                text("1. Navigate with ↑/↓ arrows")
                                    .class("text-blue-700")
                            )
                            .child_builder(
                                text("2. Press ENTER to explore widgets")
                                    .class("text-blue-700")
                            )
                            .child_builder(
                                text("3. Toggle TAB for interactive demos")
                                    .class("text-blue-700")
                            )
                    )
            )
            .build()
    }

    fn create_category_card(
        &self,
        _category: WidgetCategory,
        icon: &str,
        title: &str,
        description: &str,
        count: &str,
    ) -> Element {
        div()
            .class("bg-white")
            .class("p-6")
            .class("rounded-lg")
            .class("shadow-md")
            .class("border")
            .class("border-gray-200")
            .class("hover:shadow-lg")
            .class("hover:border-blue-300")
            .class("transition-all")
            .class("cursor-pointer")
            .child_builder(
                div()
                    .class("flex")
                    .class("items-center")
                    .class("mb-3")
                    .child_builder(
                        text(icon)
                            .class("text-3xl")
                            .class("mr-3")
                    )
                    .child_builder(
                        div()
                            .class("flex-1")
                            .child_builder(
                                text(title)
                                    .class("text-xl")
                                    .class("font-semibold")
                                    .class("text-gray-800")
                            )
                            .child_builder(
                                text(count)
                                    .class("text-sm")
                                    .class("text-gray-500")
                            )
                    )
            )
            .child_builder(
                text(description)
                    .class("text-gray-600")
                    .class("text-sm")
                    .class("leading-relaxed")
            )
            .build()
    }
}
