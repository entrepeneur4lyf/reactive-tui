use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum WidgetCategory {
    LayoutNavigation,
    FormControls,
    DataDisplay,
    ContentFeedback,
    Development,
}

#[derive(Debug, Clone)]
pub struct WidgetMeta {
    pub name: &'static str,
    pub description: &'static str,
    pub category: WidgetCategory,
}

#[derive(Debug, Clone)]
pub struct WidgetRegistry {
    pub widgets: Vec<WidgetMeta>,
    pub by_category: HashMap<WidgetCategory, Vec<usize>>, // indices into widgets vec
}

impl WidgetRegistry {
    pub fn new_basic() -> Self {
        let widgets = vec![
            WidgetMeta { name: "Button", description: "Interactive buttons with states", category: WidgetCategory::FormControls },
            WidgetMeta { name: "Input", description: "Text input with validation", category: WidgetCategory::FormControls },
            WidgetMeta { name: "Progress", description: "Progress bars with animations", category: WidgetCategory::DataDisplay },
            WidgetMeta { name: "Tabs", description: "Tab navigation", category: WidgetCategory::LayoutNavigation },
            WidgetMeta { name: "RichText", description: "Markdown and syntax highlighting", category: WidgetCategory::ContentFeedback },
        ];
        let mut by_category: HashMap<WidgetCategory, Vec<usize>> = HashMap::new();
        for (i, meta) in widgets.iter().enumerate() {
            by_category.entry(meta.category.clone()).or_default().push(i);
        }
        Self { widgets, by_category }
    }

    pub fn list_by_category(&self, cat: &WidgetCategory) -> Vec<&WidgetMeta> {
        self.by_category
            .get(cat)
            .map(|idxs| idxs.iter().map(|&i| &self.widgets[i]).collect())
            .unwrap_or_default()
    }
}

