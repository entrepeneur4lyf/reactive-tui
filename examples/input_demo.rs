use tui_core::prelude::*;
use tui_core::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
    // Get terminal size dynamically with modern fallback
    let (term_width, _term_height) = crossterm::terminal::size()
        .unwrap_or((400, 200));

    println!("🔤 Input Widget Demo\n");

    let layout = LayoutRect {
        x: 0,
        y: 0,
        width: term_width,
        height: 3,
    };
    let theme = tui_core::themes::colors::dark_theme();
    let utility = UtilityProcessor::new();

    // Define styled input classes
    let base_classes = vec![
        "bg-gray-800".to_string(),
        "border-gray-600".to_string(),
        "text-gray-100".to_string(),
    ];
    let focus_classes = vec![
        "focus:border-blue-500".to_string(),
        "focus:ring-2".to_string(),
    ];
    let error_classes = vec!["border-red-500".to_string(), "text-red-400".to_string()];
    let success_classes = vec!["border-green-500".to_string(), "text-green-400".to_string()];

    println!(
        "Base Input Style: {}",
        utility.process_classes(&base_classes)
    );

    // Text input with base styling
    let text_input = Input::builder("demo-text")
        .input_type(InputType::Text)
        .placeholder("Enter your name")
        .value("John Doe")
        .css_classes(base_classes.clone())
        .build();

    println!("Text Input (Base Theme):");
    println!("{}\n", text_input.render(&layout, Some(&theme)));

    // Email input with validation styling
    let email_input = Input::builder("demo-email")
        .input_type(InputType::Email)
        .placeholder("user@example.com")
        .value("john@example.com")
        .required(true)
        .css_classes([base_classes.clone(), success_classes].concat())
        .build();

    println!("Email Input (Valid - Success Theme):");
    println!("{}\n", email_input.render(&layout, Some(&theme)));

    // Password input with focus styling
    let password_input = Input::builder("demo-password")
        .input_type(InputType::Password)
        .placeholder("Enter password")
        .value("secret123")
        .min_length(8)
        .css_classes([base_classes.clone(), focus_classes].concat())
        .build();

    println!("Password Input (Focused):");
    println!("{}\n", password_input.render(&layout, Some(&theme)));

    // Number input with error styling
    let number_input = Input::builder("demo-number")
        .input_type(InputType::Number)
        .placeholder("Enter a valid number")
        .value("invalid")
        .css_classes([base_classes.clone(), error_classes].concat())
        .build();

    println!("Number Input (Invalid - Error Theme):");
    println!("{}\n", number_input.render(&layout, Some(&theme)));

    // Disabled input with special styling
    let disabled_input = Input::builder("demo-disabled")
        .input_type(InputType::Text)
        .placeholder("This field is disabled")
        .value("Cannot edit")
        .disabled(true)
        .css_classes(vec![
            "bg-gray-900".to_string(),
            "border-gray-700".to_string(),
            "text-gray-500".to_string(),
        ])
        .build();

    println!("Disabled Input:");
    println!("{}\n", disabled_input.render(&layout, Some(&theme)));

    // Large input with custom styling
    let large_layout = LayoutRect {
        x: 0,
        y: 0,
        width: 60,
        height: 4,
    };
    let large_input = Input::builder("demo-large")
        .input_type(InputType::Text)
        .placeholder("Large input field with custom styling")
        .value("This is a larger input field")
        .css_classes(vec![
            "bg-indigo-900".to_string(),
            "border-indigo-500".to_string(),
            "text-indigo-100".to_string(),
            "font-bold".to_string(),
        ])
        .build();

    println!("Large Input (Custom Indigo Theme):");
    println!("{}", large_input.render(&large_layout, Some(&theme)));

    println!("\n🎨 Theme Integration Demo Complete - All input types with utility CSS styling");

    Ok(())
}
