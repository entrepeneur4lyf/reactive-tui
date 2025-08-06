//! Widget registry system for organizing and displaying widget metadata

use crate::app_state::{WidgetType, WidgetCategory};
use std::collections::HashMap;

/// Widget metadata for documentation and demonstration
#[derive(Debug, Clone)]
pub struct WidgetMeta {
    pub name: String,
    pub description: String,
    pub category: WidgetCategory,
    pub documentation: String,
    pub example_code: String,
    pub parameters: Vec<ParameterDefinition>,
}

/// Parameter definition for interactive widget configuration
#[derive(Debug, Clone)]
pub struct ParameterDefinition {
    pub name: String,
    pub param_type: ParameterType,
    pub description: String,
    pub default_value: serde_json::Value,
    pub required: bool,
}

/// Parameter types for widget configuration
#[derive(Debug, Clone)]
pub enum ParameterType {
    String,
    Number,
    Boolean,
    Enum(Vec<String>),
    Color,
}

/// Widget registry for managing all available widgets
pub struct WidgetRegistry {
    widgets: HashMap<WidgetType, WidgetMeta>,
}

impl WidgetRegistry {
    pub fn new() -> Self {
        let mut registry = Self {
            widgets: HashMap::new(),
        };
        registry.register_all_widgets();
        registry
    }

    /// Register all available widgets with their metadata
    fn register_all_widgets(&mut self) {
        // Layout & Navigation widgets
        self.register_widget(WidgetType::Accordion, WidgetMeta {
            name: "Accordion".to_string(),
            description: "Expandable/collapsible sections for organizing content".to_string(),
            category: WidgetCategory::Layout,
            documentation: include_str!("../docs/widgets/accordion.md").to_string(),
            example_code: r#"use reactive_tui::widgets::*;

let accordion = Accordion::builder("settings-accordion")
    .section("General", general_content)
    .section("Advanced", advanced_content)
    .animation(AccordionAnimation::Slide)
    .build();"#.to_string(),
            parameters: vec![
                ParameterDefinition {
                    name: "animation".to_string(),
                    param_type: ParameterType::Enum(vec!["Slide".to_string(), "Fade".to_string()]),
                    description: "Animation type for expand/collapse".to_string(),
                    default_value: serde_json::Value::String("Slide".to_string()),
                    required: false,
                },
            ],
        });

        self.register_widget(WidgetType::Bar, WidgetMeta {
            name: "Bar".to_string(),
            description: "Header/footer bars with flexible positioning".to_string(),
            category: WidgetCategory::Layout,
            documentation: include_str!("../docs/widgets/bar.md").to_string(),
            example_code: r#"use reactive_tui::widgets::*;

let header = Bar::builder("header")
    .position(BarPosition::Top)
    .left_content("Logo")
    .center_content("Title")
    .right_content("Status")
    .build();"#.to_string(),
            parameters: vec![
                ParameterDefinition {
                    name: "position".to_string(),
                    param_type: ParameterType::Enum(vec!["Top".to_string(), "Bottom".to_string()]),
                    description: "Bar position".to_string(),
                    default_value: serde_json::Value::String("Top".to_string()),
                    required: false,
                },
            ],
        });

        self.register_widget(WidgetType::Button, WidgetMeta {
            name: "Button".to_string(),
            description: "Interactive buttons with multiple variants and states".to_string(),
            category: WidgetCategory::FormControls,
            documentation: include_str!("../docs/widgets/button.md").to_string(),
            example_code: r#"use reactive_tui::widgets::*;

let button = Button::builder("save-btn", "Save File")
    .button_type(ButtonType::Primary)
    .size(ButtonSize::Large)
    .icon("💾", IconPosition::Left)
    .build();"#.to_string(),
            parameters: vec![
                ParameterDefinition {
                    name: "text".to_string(),
                    param_type: ParameterType::String,
                    description: "Button text".to_string(),
                    default_value: serde_json::Value::String("Click Me".to_string()),
                    required: true,
                },
                ParameterDefinition {
                    name: "variant".to_string(),
                    param_type: ParameterType::Enum(vec![
                        "Primary".to_string(), "Secondary".to_string(), 
                        "Success".to_string(), "Danger".to_string()
                    ]),
                    description: "Button variant".to_string(),
                    default_value: serde_json::Value::String("Primary".to_string()),
                    required: false,
                },
                ParameterDefinition {
                    name: "disabled".to_string(),
                    param_type: ParameterType::Boolean,
                    description: "Whether the button is disabled".to_string(),
                    default_value: serde_json::Value::Bool(false),
                    required: false,
                },
            ],
        });

        self.register_widget(WidgetType::Input, WidgetMeta {
            name: "Input".to_string(),
            description: "Text input fields with validation and formatting".to_string(),
            category: WidgetCategory::FormControls,
            documentation: include_str!("../docs/widgets/input.md").to_string(),
            example_code: r#"use reactive_tui::widgets::*;

let input = Input::builder("username")
    .placeholder("Enter username")
    .required(true)
    .max_length(20)
    .build();"#.to_string(),
            parameters: vec![
                ParameterDefinition {
                    name: "placeholder".to_string(),
                    param_type: ParameterType::String,
                    description: "Placeholder text".to_string(),
                    default_value: serde_json::Value::String("Enter text...".to_string()),
                    required: false,
                },
                ParameterDefinition {
                    name: "required".to_string(),
                    param_type: ParameterType::Boolean,
                    description: "Whether the input is required".to_string(),
                    default_value: serde_json::Value::Bool(false),
                    required: false,
                },
            ],
        });

        self.register_widget(WidgetType::Progress, WidgetMeta {
            name: "Progress".to_string(),
            description: "Progress bars and indicators with animations".to_string(),
            category: WidgetCategory::DataDisplay,
            documentation: include_str!("../docs/widgets/progress.md").to_string(),
            example_code: r#"use reactive_tui::widgets::*;

let progress = Progress::builder("download")
    .value(65)
    .max(100)
    .label("Downloading...")
    .show_percentage(true)
    .build();"#.to_string(),
            parameters: vec![
                ParameterDefinition {
                    name: "value".to_string(),
                    param_type: ParameterType::Number,
                    description: "Current progress value".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(50)),
                    required: true,
                },
                ParameterDefinition {
                    name: "max".to_string(),
                    param_type: ParameterType::Number,
                    description: "Maximum progress value".to_string(),
                    default_value: serde_json::Value::Number(serde_json::Number::from(100)),
                    required: false,
                },
            ],
        });

        // Add more widgets as needed...
        // For now, we'll register basic metadata for all widget types
        for widget_type in WidgetType::all() {
            if !self.widgets.contains_key(&widget_type) {
                self.register_basic_widget(widget_type);
            }
        }
    }

    fn register_basic_widget(&mut self, widget_type: WidgetType) {
        let meta = WidgetMeta {
            name: widget_type.display_name().to_string(),
            description: format!("{} widget - comprehensive functionality", widget_type.display_name()),
            category: widget_type.category(),
            documentation: format!("# {}\n\nComprehensive {} widget with full feature set.", 
                widget_type.display_name(), widget_type.display_name()),
            example_code: format!("// {} example\nlet widget = {}::new();", 
                widget_type.display_name(), widget_type.display_name()),
            parameters: vec![],
        };
        self.widgets.insert(widget_type, meta);
    }

    fn register_widget(&mut self, widget_type: WidgetType, meta: WidgetMeta) {
        self.widgets.insert(widget_type, meta);
    }

    pub fn get_widget(&self, widget_type: WidgetType) -> Option<&WidgetMeta> {
        self.widgets.get(&widget_type)
    }

    pub fn get_widgets_by_category(&self, category: WidgetCategory) -> Vec<WidgetType> {
        self.widgets
            .iter()
            .filter(|(_, meta)| meta.category == category)
            .map(|(widget_type, _)| *widget_type)
            .collect()
    }

    pub fn get_all_widgets(&self) -> Vec<WidgetType> {
        self.widgets.keys().copied().collect()
    }

    pub fn get_categories(&self) -> Vec<WidgetCategory> {
        WidgetCategory::all()
    }
}
