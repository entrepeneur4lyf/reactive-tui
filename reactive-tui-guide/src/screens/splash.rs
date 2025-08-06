//! Splash screen component

use reactive_tui::prelude::*;
use reactive_tui::components::{div, text, section};
use reactive_tui::widgets::image::logo;

/// Splash screen component with branding and framework information
#[derive(Debug, Clone)]
pub struct SplashScreen;

impl SplashScreen {
    pub fn new() -> Self {
        Self
    }
}

impl Component for SplashScreen {
    fn render(&self) -> Element {
        div()
            .class("flex")
            .class("flex-col")
            .class("h-full")
            .class("items-center")
            .class("justify-center")
            .class("bg-gradient-to-b")
            .class("from-blue-900")
            .class("to-purple-900")
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/6")
                    .class("mb-4")
                    .child(logo("reactive-tui-logo", "assets/reactive-tui-logo.png"))
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("flex-col")
                    .class("items-center")
                    .class("h-1/6")
                    .class("mb-6")
                    .child_builder(
                        text("REACTIVE TUI")
                            .class("text-5xl")
                            .class("font-bold")
                            .class("text-white")
                            .class("mb-2")
                    )
                    .child_builder(
                        text("Interactive Widget Guide")
                            .class("text-2xl")
                            .class("text-blue-200")
                            .class("font-semibold")
                    )
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/12")
                    .class("mb-8")
                    .child_builder(
                        text("A revolutionary CSS-styled terminal UI framework")
                            .class("text-lg")
                            .class("text-gray-300")
                            .class("italic")
                    )
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("flex-col")
                    .class("items-center")
                    .class("h-1/4")
                    .class("justify-center")
                    .class("space-y-2")
                    .child_builder(
                        text("✨ CSS-Styled Terminal Interfaces")
                            .class("text-green-300")
                            .class("text-lg")
                    )
                    .child_builder(
                        text("🧩 29+ Production-Ready Widgets")
                            .class("text-blue-300")
                            .class("text-lg")
                    )
                    .child_builder(
                        text("⚡ High Performance Rust + TypeScript")
                            .class("text-yellow-300")
                            .class("text-lg")
                    )
                    .child_builder(
                        text("🔗 Full TypeScript SDK Integration")
                            .class("text-purple-300")
                            .class("text-lg")
                    )
                    .child_builder(
                        text("🎯 60 FPS Adaptive Rendering")
                            .class("text-red-300")
                            .class("text-lg")
                    )
            )
            .child_builder(
                section()
                    .class("flex")
                    .class("items-center")
                    .class("justify-center")
                    .class("h-1/12")
                    .class("mt-8")
                    .child_builder(
                        text("Press ENTER to continue or Q to quit")
                            .class("text-xl")
                            .class("font-bold")
                            .class("text-white")
                            .class("bg-blue-600")
                            .class("px-6")
                            .class("py-2")
                            .class("rounded")
                            .class("border")
                            .class("border-blue-400")
                            .class("animate-pulse")
                    )
            )
            .build()
    }
}
