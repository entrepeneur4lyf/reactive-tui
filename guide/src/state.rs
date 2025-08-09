use std::sync::Arc;
use reactive_tui::reactive::Reactive;

#[derive(Debug, Clone, PartialEq)]
pub enum GuideMode {
    Documentation,
    Interactive,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Screen {
    Splash,
    Home,
    Category { index: usize },
    WidgetDemo { name: String },
}

#[derive(Debug, Clone)]
pub struct GuideState {
    pub screen: Arc<Reactive<Screen>>,
    pub guide_mode: Arc<Reactive<GuideMode>>,
    pub selected_category: Arc<Reactive<usize>>,
    pub selected_widget: Arc<Reactive<usize>>,
    pub categories: Vec<&'static str>,
}

impl GuideState {
    pub fn new() -> Self {
        Self {
            screen: Arc::new(Reactive::new(Screen::Splash)),
            guide_mode: Arc::new(Reactive::new(GuideMode::Documentation)),
            selected_category: Arc::new(Reactive::new(0)),
            selected_widget: Arc::new(Reactive::new(0)),
            categories: vec![
                "Layout & Navigation",
                "Form Controls",
                "Data Display",
                "Content & Feedback",
                "Development",
            ],
        }
    }
}

