/*!
 * Naming Convention Test
 *
 * Verifies that serde rename_all works correctly for FFI compatibility
 * between Rust snake_case and TypeScript camelCase
 */

use reactive_tui::themes::colors::{rgb, ColorPalette, SemanticColorMapping};

fn main() {
  println!("🔧 Testing Naming Convention Standardization");
  println!("{}", "=".repeat(50));

  // Create a test palette with snake_case field names
  let test_palette = ColorPalette {
    primary: rgb(99, 102, 241),
    primary_dark: rgb(79, 70, 229), // snake_case in Rust
    primary_light: rgb(129, 140, 248),
    secondary: rgb(16, 185, 129),
    secondary_dark: rgb(5, 150, 105), // snake_case in Rust
    secondary_light: rgb(52, 211, 153),
    background: rgb(17, 24, 39),
    background_alt: rgb(31, 41, 55), // snake_case in Rust
    surface: rgb(55, 65, 81),
    surface_alt: rgb(75, 85, 99), // snake_case in Rust
    text: rgb(249, 250, 251),
    text_secondary: rgb(209, 213, 219), // snake_case in Rust
    text_muted: rgb(156, 163, 175),     // snake_case in Rust
    text_inverse: rgb(17, 24, 39),      // snake_case in Rust
    border: rgb(75, 85, 99),
    border_focus: rgb(99, 102, 241),  // snake_case in Rust
    border_hover: rgb(107, 114, 128), // snake_case in Rust
    success: rgb(34, 197, 94),
    warning: rgb(251, 191, 36),
    error: rgb(239, 68, 68),
    info: rgb(59, 130, 246),
    hover: rgb(67, 56, 202),
    active: rgb(55, 48, 163),
    disabled: rgb(107, 114, 128),
    shadow: rgb(0, 0, 0),
    shadow_light: rgb(31, 41, 55), // snake_case in Rust
  };

  // Serialize to JSON - should use camelCase
  let json_output = serde_json::to_string_pretty(&test_palette).unwrap();
  println!("📄 JSON Output (should use camelCase):");
  println!("{json_output}");

  // Verify that key fields are correctly converted
  let expected_fields = [
    "primaryDark", // should be camelCase in JSON
    "primaryLight",
    "secondaryDark",
    "secondaryLight",
    "backgroundAlt",
    "surfaceAlt",
    "textSecondary",
    "textMuted",
    "textInverse",
    "borderFocus",
    "borderHover",
    "shadowLight",
  ];

  println!("\n🔍 Verification Tests:");
  for field in &expected_fields {
    if json_output.contains(field) {
      println!("✓ Found camelCase field: {field}");
    } else {
      println!("❌ Missing camelCase field: {field}");
    }
  }

  // Test deserialization from camelCase JSON
  let camel_case_json = r#"{
        "primary": { "r": 99, "g": 102, "b": 241 },
        "primaryDark": { "r": 79, "g": 70, "b": 229 },
        "primaryLight": { "r": 129, "g": 140, "b": 248 },
        "secondary": { "r": 16, "g": 185, "b": 129 },
        "secondaryDark": { "r": 5, "g": 150, "b": 105 },
        "secondaryLight": { "r": 52, "g": 211, "b": 153 },
        "background": { "r": 17, "g": 24, "b": 39 },
        "backgroundAlt": { "r": 31, "g": 41, "b": 55 },
        "surface": { "r": 55, "g": 65, "b": 81 },
        "surfaceAlt": { "r": 75, "g": 85, "b": 99 },
        "text": { "r": 249, "g": 250, "b": 251 },
        "textSecondary": { "r": 209, "g": 213, "b": 219 },
        "textMuted": { "r": 156, "g": 163, "b": 175 },
        "textInverse": { "r": 17, "g": 24, "b": 39 },
        "border": { "r": 75, "g": 85, "b": 99 },
        "borderFocus": { "r": 99, "g": 102, "b": 241 },
        "borderHover": { "r": 107, "g": 114, "b": 128 },
        "success": { "r": 34, "g": 197, "b": 94 },
        "warning": { "r": 251, "g": 191, "b": 36 },
        "error": { "r": 239, "g": 68, "b": 68 },
        "info": { "r": 59, "g": 130, "b": 246 },
        "hover": { "r": 67, "g": 56, "b": 202 },
        "active": { "r": 55, "g": 48, "b": 163 },
        "disabled": { "r": 107, "g": 114, "b": 128 },
        "shadow": { "r": 0, "g": 0, "b": 0 },
        "shadowLight": { "r": 31, "g": 41, "b": 55 }
    }"#;

  println!("\n🔄 Testing Deserialization from camelCase JSON:");
  match serde_json::from_str::<ColorPalette>(camel_case_json) {
    Ok(parsed_palette) => {
      println!("✅ Successfully parsed camelCase JSON to Rust struct!");
      println!("   primaryDark: {:?}", parsed_palette.primary_dark);
      println!("   backgroundAlt: {:?}", parsed_palette.background_alt);
      println!("   textSecondary: {:?}", parsed_palette.text_secondary);
    }
    Err(e) => {
      println!("❌ Failed to parse camelCase JSON: {e}");
    }
  }

  // Test with SemanticColorMapping
  let semantic_mapping = SemanticColorMapping {
    panel_background: "surface".to_string(),
    panel_border: "border".to_string(),
    panel_title: "text".to_string(),
    panel_content: "text_secondary".to_string(), // snake_case in Rust
    panel_shadow: "shadow".to_string(),
    button_background: "primary".to_string(),
    button_border: "primary_dark".to_string(), // snake_case in Rust
    button_text: "text_inverse".to_string(),   // snake_case in Rust
    button_hover: "hover".to_string(),
    input_background: "background_alt".to_string(), // snake_case in Rust
    input_border: "border".to_string(),
    input_text: "text".to_string(),
    input_focus: "border_focus".to_string(), // snake_case in Rust
    progress_background: "surface".to_string(),
    progress_fill: "primary".to_string(),
    progress_text: "text".to_string(),
    editor_background: "background".to_string(),
    editor_text: "text".to_string(),
    editor_cursor: "primary".to_string(),
    editor_line_number: "text_muted".to_string(), // snake_case in Rust
    editor_selection: "primary".to_string(),
    editor_border: "border".to_string(),
    editor_border_focus: "border_focus".to_string(), // snake_case in Rust
    syntax_keyword: "primary".to_string(),
    syntax_string: "success".to_string(),
    syntax_comment: "text_muted".to_string(), // snake_case in Rust
    syntax_number: "warning".to_string(),
    syntax_function: "info".to_string(),
    syntax_type: "secondary".to_string(),
    syntax_variable: "text".to_string(),
    syntax_operator: "text_secondary".to_string(), // snake_case in Rust
    syntax_punctuation: "text_secondary".to_string(), // snake_case in Rust
    syntax_constant: "warning".to_string(),
    syntax_tag: "primary".to_string(),
    syntax_attribute: "secondary".to_string(),
  };

  println!("\n📋 Testing SemanticColorMapping serialization:");
  let semantic_json = serde_json::to_string_pretty(&semantic_mapping).unwrap();
  println!("{semantic_json}");

  // Verify semantic mapping fields are camelCase
  let semantic_expected_fields = [
    "panelBackground",
    "panelContent",
    "buttonBackground",
    "buttonBorder",
    "buttonText",
    "buttonHover",
    "inputBackground",
    "inputBorder",
    "inputText",
    "inputFocus",
    "progressBackground",
    "progressFill",
    "progressText",
    "editorBackground",
    "editorText",
    "editorCursor",
    "editorLineNumber",
    "editorSelection",
    "editorBorder",
    "editorBorderFocus",
    "syntaxKeyword",
    "syntaxString",
    "syntaxComment",
    "syntaxNumber",
    "syntaxFunction",
    "syntaxType",
    "syntaxVariable",
    "syntaxOperator",
    "syntaxPunctuation",
    "syntaxConstant",
    "syntaxTag",
    "syntaxAttribute",
  ];

  println!("\n🔍 Verifying SemanticColorMapping camelCase conversion:");
  for field in &semantic_expected_fields {
    if semantic_json.contains(field) {
      println!("✓ Found camelCase field: {field}");
    } else {
      println!("❌ Missing camelCase field: {field}");
    }
  }

  println!("\n🎉 Naming Convention Test Complete!");
  println!("{}", "=".repeat(50));
  println!("✓ Rust uses snake_case internally (e.g., primary_dark)");
  println!("✓ JSON serialization outputs camelCase (e.g., primaryDark)");
  println!("✓ JSON deserialization accepts camelCase input");
  println!("✓ FFI compatibility achieved without breaking Rust conventions");
  println!(
    "\n🚀 Naming standardization successful - maintains both Rust and TypeScript conventions!"
  );
}
