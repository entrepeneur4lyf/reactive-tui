use anyhow::Result;
use reactive_tui::prelude::*;
use reactive_tui_guide::state::{GuideState, Screen};
use reactive_tui_guide::registry::{WidgetRegistry, WidgetCategory};

#[derive(Debug, Clone)]
struct SimpleGuide {
    app_state: GuideState,
    registry: WidgetRegistry,
}

impl Component for SimpleGuide {
    fn render(&self) -> Element {
        Element::with_tag("div")
            .class("guide-app")
            .content("🚀 Reactive TUI Interactive Widget Guide\n\nWelcome to the comprehensive widget demonstration!\n\nThis guide showcases all 29+ widgets available in the Reactive TUI framework.\n\n[Coming Soon: Full interactive experience]")
            .build();
        match self.app_state.screen.get() {
            Screen::Splash => Self::splash(),
            Screen::Home => self.home(),
            Screen::Category { index } => Self::category_view(&self.app_state, index, &self.registry),
            Screen::WidgetDemo { name } => self.widget_demo(&name),
        }
    }
}



impl SimpleGuide {
    fn with_state(app_state: GuideState) -> Self {
        Self { app_state, registry: WidgetRegistry::new_basic() }
    }

    fn header() -> Element {
        Element::with_tag("div")
            .class("home-header")
            .content("Reactive TUI – Widget Guide")
            .build()
    }

    fn footer() -> Element {
        Element::with_tag("div")
            .class("home-footer")
            .content("Keys: Enter – Open • ESC – Back/Quit • ↑/↓ – Navigate • ←/→ – Switch • Tab – Toggle Mode")
            .build()
    }

    fn splash() -> Element {
        Element::with_tag("div")
            .class("splash-container")
            .child(
                Element::with_tag("div")
                    .class("splash-content")
                    .child(Element::with_tag("div").class("logo-section").content("Reactive TUI").build())
                    .child(
                        Element::with_tag("div")
                            .class("title-section")
                            .child(Element::with_tag("text").content("Interactive Widget Guide").build())
                            .child(Element::with_tag("text").class("version-info").content("v0.1.0").build())
                            .build()
                    )
                    .child(
                        Element::with_tag("div")
                            .class("continue-prompt")
                            .content("Press ENTER to continue")
                            .build()
                    )
                    .build()
            )
            .build()
    }

    fn home(&self) -> Element {
        Element::with_tag("div")
            .class("home-container")
            .child(Self::header())
            .child(
                Element::with_tag("div")
                    .class("home-main")
                    .child(
                        Element::with_tag("div")
                            .class("overview-section")
                            .content("Welcome! Use ↑/↓ to browse categories, Enter to open. Tab toggles Doc/Interactive; ←/→ choose widget; Enter opens demo.")
                            .build()
                    )
                    .build()
            )
            .child(Self::footer())
            .build()
    }

    fn layout_with_sidebar(title: &str, sidebar: Element, content: Element) -> Element {
        Element::with_tag("div")
            .class("guide-layout")
            .child(Self::header())
            .child(
                Element::with_tag("div")
                    .class("guide-body")
                    .child(
                        Element::with_tag("div")
                            .class("guide-sidebar")
                            .child(sidebar)
                            .build()
                    )
                    .child(
                        Element::with_tag("div")
                            .class("guide-content")
                            .child(Element::with_tag("div").class("content-title").content(title).build())
                            .child(content)
                            .build()
                    )
                    .build()
            )
            .child(Self::footer())
            .build()
    }

    fn category_view(state: &GuideState, index: usize, registry: &WidgetRegistry) -> Element {
        let (title, cat) = match index {
            0 => ("Layout & Navigation", WidgetCategory::LayoutNavigation),
            1 => ("Form Controls", WidgetCategory::FormControls),
            2 => ("Data Display", WidgetCategory::DataDisplay),
            3 => ("Content & Feedback", WidgetCategory::ContentFeedback),
            _ => ("Development", WidgetCategory::Development),
        };

        let items = registry.list_by_category(&cat);
        let mut list_container = Element::with_tag("div").class("categories-section");
        for (i, item) in items.iter().enumerate() {
            let class = if i == state.selected_widget.get() { "category-item selected" } else { "category-item" };
            list_container = list_container.child(
                Element::with_tag("div")
                    .class(class)
                    .content(item.name)
                    .build()
            );
        }
        // Sidebar with categories and back option
        let mut sidebar = Element::with_tag("div")
            .class("category-sidebar")
            .child(Element::with_tag("div").class("sidebar-title").content("Categories").build());
        for (i, name) in state.categories.iter().enumerate() {
            let item_class = if i == state.selected_category.get() { "sidebar-item selected" } else { "sidebar-item" };
            sidebar = sidebar.child(Element::with_tag("div").class(item_class).content(*name).build());
        }
        let sidebar = sidebar.build();

        SimpleGuide::layout_with_sidebar(
            &format!("Category: {}", title),
            sidebar,
            list_container.build(),
        )

    }

    fn widget_demo(&self, name: &str) -> Element {
        use reactive_tui::widgets::Button;
        let content = match name {
            "Button" => {
                let btn = Button::builder("demo-btn", "Click Me").build();
                Element::with_tag("div")
                    .class("demo-panel")
                    .child(btn.to_element())
                    .build()
            }
            "RichText" => {
                let md = "# RichText Demo\n\n- Markdown\n- Syntax highlighting";
                let rt = reactive_tui::widgets::rich_text::documentation_viewer(md.to_string());
                rt.to_element()
            }
            _ => Element::with_tag("div").content("Demo coming soon").build(),
        };
        SimpleGuide::layout_with_sidebar(
            &format!("Demo: {}", name),
            Element::with_tag("div").class("category-sidebar").content("← Back").build(),
            content,
        )
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging for development
    #[cfg(debug_assertions)]
    env_logger::init();

    // Create and run a simple TUI app
    let state = reactive_tui_guide::state::GuideState::new();

    let mut app = TuiApp::builder()
        .component(SimpleGuide::with_state(state.clone()))
        .stylesheet("styles/guide.css")
        .with_title("Reactive TUI Interactive Widget Guide")
        .frame_rate(60)
        .build()?;

    // Basic navigation: Enter to continue; ESC to go back; Ctrl+C/Q handled by defaults to quit
    app.bind_key_to_action(reactive_tui::events::KeyCombination::enter(), "continue").await;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::escape(), "go_back").await;
    use reactive_tui::compat::KeyCode;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::new(KeyCode::Up), "nav_prev").await;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::new(KeyCode::Down), "nav_next").await;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::new(KeyCode::Left), "nav_widget_prev").await;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::new(KeyCode::Right), "nav_widget_next").await;
    app.bind_key_to_action(reactive_tui::events::KeyCombination::new(KeyCode::Tab), "toggle_mode").await;

    // Register actions that update shared state
    let state_for_continue = state.clone();
    app.register_action("continue", move |_action| {
        // If on Splash, go Home; if on Home, open selected category
        match state_for_continue.screen.get() {
            Screen::Splash => state_for_continue.screen.set(Screen::Home),
            Screen::Home => {
                state_for_continue.selected_widget.set(0);
                let idx = state_for_continue.selected_category.get();
                state_for_continue.screen.set(Screen::Category { index: idx });
            }
            Screen::Category { index } => {
                // Open demo for selected widget in current category
                let reg = reactive_tui_guide::registry::WidgetRegistry::new_basic();
                let cat = match index {
                    0 => reactive_tui_guide::registry::WidgetCategory::LayoutNavigation,
                    1 => reactive_tui_guide::registry::WidgetCategory::FormControls,
                    2 => reactive_tui_guide::registry::WidgetCategory::DataDisplay,
                    3 => reactive_tui_guide::registry::WidgetCategory::ContentFeedback,
                    _ => reactive_tui_guide::registry::WidgetCategory::Development,
                };
                let items = reg.list_by_category(&cat);
                if !items.is_empty() {
                    let sel = state_for_continue.selected_widget.get() % items.len();
                    let name = items[sel].name.to_string();
                    state_for_continue.screen.set(Screen::WidgetDemo { name });
                }
            }
            Screen::WidgetDemo { .. } => {}
        }
        reactive_tui::events::ActionResult::Handled
    });

    // Widget navigation within a category (Left/Right)
    let state_for_wprev = state.clone();
    app.register_action("nav_widget_prev", move |_action| {
        if let Screen::Category { index: _ } = state_for_wprev.screen.get() {
            let cur = state_for_wprev.selected_widget.get();
            let new_idx = if cur == 0 { 0 } else { cur - 1 };
            state_for_wprev.selected_widget.set(new_idx);
        }
        reactive_tui::events::ActionResult::Handled
    });

    let state_for_wnext = state.clone();
    app.register_action("nav_widget_next", move |_action| {
        if let Screen::Category { index } = state_for_wnext.screen.get() {
            // Wrap only if we know length; for now allow increment and rely on modulo on open
            let next = state_for_wnext.selected_widget.get() + 1;
            state_for_wnext.selected_widget.set(next);
        }
        reactive_tui::events::ActionResult::Handled
    });

    // Toggle documentation/interactive mode
    let state_for_toggle = state.clone();
    app.register_action("toggle_mode", move |_action| {
        use reactive_tui_guide::state::GuideMode;
        let current = state_for_toggle.guide_mode.get();
        let new_mode = match current {
            GuideMode::Documentation => GuideMode::Interactive,
            GuideMode::Interactive => GuideMode::Documentation,
        };
        state_for_toggle.guide_mode.set(new_mode);
        reactive_tui::events::ActionResult::Handled
    });

    // Navigation actions for categories (home screen)
    let state_for_prev = state.clone();
    app.register_action("nav_prev", move |_action| {
        let current = state_for_prev.selected_category.get();
        let next = if current == 0 { state_for_prev.categories.len() - 1 } else { current - 1 };
        state_for_prev.selected_category.set(next);
        reactive_tui::events::ActionResult::Handled
    });

    let state_for_next = state.clone();
    app.register_action("nav_next", move |_action| {
        let current = state_for_next.selected_category.get();
        let next = (current + 1) % state_for_next.categories.len();
        state_for_next.selected_category.set(next);
        reactive_tui::events::ActionResult::Handled
    });

    // Back navigation action (ESC)
    let state_for_back = state.clone();
    app.register_action("go_back", move |_action| {
        match state_for_back.screen.get() {
            Screen::WidgetDemo { .. } => {
                // Return to current category view
                let idx = state_for_back.selected_category.get();
                state_for_back.screen.set(Screen::Category { index: idx });
            }
            Screen::Category { .. } => {
                state_for_back.screen.set(Screen::Home);
            }
            Screen::Home => {
                state_for_back.screen.set(Screen::Splash);
            }
            Screen::Splash => {
                // Let default Quit bindings handle actual exit (Ctrl+C or Q)
            }
        }
        reactive_tui::events::ActionResult::Handled
    });

    app.run().await?;

    Ok(())
}
