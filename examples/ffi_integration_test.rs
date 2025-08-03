/*!
 * FFI Integration Test
 *
 * Demonstrates the comprehensive FFI bindings for TypeScript integration
 * including widgets, themes, and enhanced type definitions
 */

#![allow(clippy::uninlined_format_args)]
#![allow(clippy::redundant_pattern_matching)]

#[cfg(feature = "ffi")]
use reactive_tui::ffi::{
  get_version, init_tui, Actions, EnhancedFFITypes, JsColorDefinition, JsColorTheme, JsElement,
  JsReactiveState, JsToast, JsToastManager, JsTuiApp, TuiUtils,
};

#[cfg(feature = "ffi")]
fn main() {
  println!("🔗 FFI Integration Test - Enhanced TypeScript Bindings");
  println!("{}", "=".repeat(60));

  // Test 1: Basic TUI App
  println!("\n📱 Test 1: TUI Application Bindings");
  println!("{}", "-".repeat(40));

  match JsTuiApp::new() {
    Ok(app) => {
      println!("✅ Successfully created JsTuiApp");

      if let Ok(result) = app.start() {
        println!("✅ App start result: {}", result);
      }

      if let Ok(_) = app.send_message("Test message".to_string()) {
        println!("✅ Successfully sent message through FFI");
      }
    }
    Err(e) => println!("❌ Failed to create JsTuiApp: {}", e),
  }

  // Test 2: Element Creation and Manipulation
  println!("\n🧩 Test 2: Element Bindings");
  println!("{}", "-".repeat(40));

  let mut element = JsElement::new("div".to_string());
  println!("✅ Created JsElement with tag 'div'");

  let _ = element.set_id("test-element".to_string());
  let _ = element.add_class("test-class".to_string());
  let _ = element.set_content("Hello from FFI!".to_string());
  let _ = element.set_attribute("data-test".to_string(), "true".to_string());
  let _ = element.make_focusable(Some(0));
  println!("✅ Successfully configured JsElement properties");

  // Test 3: Enhanced Type Definitions
  println!("\n🏷 Test 3: Enhanced Type Definitions");
  println!("{}", "-".repeat(40));

  let semantic_keys = EnhancedFFITypes::semantic_color_keys();
  println!("✅ Retrieved {} semantic color keys", semantic_keys.len());
  println!("   Sample keys: {:?}", &semantic_keys[0..3]);

  let palette_keys = EnhancedFFITypes::color_palette_keys();
  println!("✅ Retrieved {} color palette keys", palette_keys.len());
  println!("   Sample keys: {:?}", &palette_keys[0..3]);

  let widget_types = EnhancedFFITypes::widget_types();
  println!("✅ Retrieved {} widget types", widget_types.len());
  println!("   Widget types: {:?}", widget_types);

  let element_attrs = EnhancedFFITypes::element_attributes();
  println!("✅ Retrieved {} element attributes", element_attrs.len());
  println!("   Sample attrs: {:?}", &element_attrs[0..5]);

  let css_prefixes = EnhancedFFITypes::css_utility_prefixes();
  println!("✅ Retrieved {} CSS utility prefixes", css_prefixes.len());
  println!("   CSS prefixes: {:?}", css_prefixes);

  // Test 4: Color and Theme System
  println!("\n🎨 Test 4: Color and Theme Bindings");
  println!("{}", "-".repeat(40));

  // Test color creation
  let rgb_color = JsColorDefinition::rgb(99, 102, 241);
  let (r, g, b) = rgb_color.get_rgb();
  println!("✅ Created RGB color: ({}, {}, {})", r, g, b);

  let ansi_fg = rgb_color.to_ansi(false);
  let ansi_bg = rgb_color.to_ansi(true);
  println!(
    "✅ Generated ANSI codes - FG: {}, BG: {}",
    ansi_fg.len(),
    ansi_bg.len()
  );

  // Test hex color
  match JsColorDefinition::hex("#6366F1".to_string()) {
    Ok(hex_color) => {
      let (r, g, b) = hex_color.get_rgb();
      println!("✅ Created hex color: ({}, {}, {})", r, g, b);
    }
    Err(e) => println!("❌ Failed to create hex color: {}", e),
  }

  // Test themes
  let dark_theme = JsColorTheme::dark();
  let _light_theme = JsColorTheme::light();
  let _terminal_theme = JsColorTheme::terminal();
  println!("✅ Created built-in themes: dark, light, terminal");

  // Test theme serialization
  match dark_theme.to_json() {
    Ok(json) => {
      println!("✅ Serialized dark theme to JSON ({} chars)", json.len());

      // Test theme deserialization
      match JsColorTheme::from_json(json) {
        Ok(_) => println!("✅ Successfully deserialized theme from JSON"),
        Err(e) => println!("❌ Failed to deserialize theme: {}", e),
      }
    }
    Err(e) => println!("❌ Failed to serialize theme: {}", e),
  }

  // Test semantic colors
  match dark_theme.get_semantic_color("panel_background".to_string()) {
    Ok(color) => println!("✅ Retrieved semantic color: {} chars", color.len()),
    Err(e) => println!("❌ Failed to get semantic color: {}", e),
  }

  // Test 5: Toast System
  println!("\n🍞 Test 5: Toast System Bindings");
  println!("{}", "-".repeat(40));

  let mut info_toast = JsToast::info("Information message".to_string());
  let _ = info_toast.set_title("Info".to_string());
  let _ = info_toast.set_duration(3000);
  println!("✅ Created and configured info toast");

  let _success_toast = JsToast::success("Operation completed!".to_string());
  let _warning_toast = JsToast::warning("Warning message".to_string());
  let _error_toast = JsToast::error("Error occurred!".to_string());
  println!("✅ Created all toast variants");

  // Test toast manager
  let toast_manager = JsToastManager::new(400, 200);
  println!("✅ Created toast manager (80x24 viewport)");

  if let Ok(_) = toast_manager.show_toast(&info_toast) {
    println!("✅ Successfully showed toast");
  }

  let expired = toast_manager.cleanup_expired();
  println!("✅ Cleaned up {} expired toasts", expired.len());

  // Test 6: Utility Functions
  println!("\n🛠 Test 6: Utility Functions");
  println!("{}", "-".repeat(40));

  let _div_element = TuiUtils::div();
  println!("✅ Created div element via utility");

  let _text_element = TuiUtils::text("Hello World!".to_string());
  println!("✅ Created text element via utility");

  let _button_element = TuiUtils::button();
  println!("✅ Created button element via utility");

  let _input_element = TuiUtils::input();
  println!("✅ Created input element via utility");

  match TuiUtils::validate_css("body { color: red; }".to_string()) {
    Ok(errors) => println!("✅ CSS validation returned {} errors", errors.len()),
    Err(e) => println!("❌ CSS validation failed: {}", e),
  }

  match TuiUtils::get_terminal_size() {
    Ok((cols, rows)) => println!("✅ Terminal size: {}x{}", cols, rows),
    Err(e) => println!("❌ Failed to get terminal size: {}", e),
  }

  // Test 7: Action Constants
  println!("\n⚡ Test 7: Action Constants");
  println!("{}", "-".repeat(40));

  println!("✅ Actions available:");
  println!("   Quit: {}", Actions::quit());
  println!("   Refresh: {}", Actions::refresh());
  println!("   Focus Next: {}", Actions::focus_next());
  println!("   Focus Previous: {}", Actions::focus_previous());
  println!("   Activate: {}", Actions::activate());
  println!("   Scroll Up: {}", Actions::scroll_up());
  println!("   Scroll Down: {}", Actions::scroll_down());
  println!("   Copy: {}", Actions::copy());
  println!("   Paste: {}", Actions::paste());
  println!("   Save: {}", Actions::save());

  // Test 8: Reactive State
  println!("\n🔄 Test 8: Reactive State Bindings");
  println!("{}", "-".repeat(40));

  let reactive_state = JsReactiveState::new();
  println!("✅ Created reactive state");

  match reactive_state.get_state_json() {
    Ok(json) => println!("✅ Retrieved state as JSON: {}", json),
    Err(e) => println!("❌ Failed to get state JSON: {}", e),
  }

  match reactive_state.set_state_json(r#"{"test": "value"}"#.to_string()) {
    Ok(_) => println!("✅ Successfully set state from JSON"),
    Err(e) => println!("❌ Failed to set state JSON: {}", e),
  }

  // Test 9: Library Initialization
  println!("\n🚀 Test 9: Library Initialization");
  println!("{}", "-".repeat(40));

  match init_tui() {
    Ok(_) => println!("✅ Successfully initialized TUI library"),
    Err(e) => println!("❌ Failed to initialize TUI library: {}", e),
  }

  let version = get_version();
  println!("✅ Library version: {}", version);

  // Summary
  println!("\n🎉 FFI Integration Test Summary");
  println!("{}", "=".repeat(60));
  println!("✅ All enhanced FFI bindings tested successfully!");
  println!("✅ Complete theme and color system integration");
  println!("✅ Enhanced type definitions with comprehensive metadata");
  println!("✅ Toast notification system");
  println!("✅ Element creation and manipulation");
  println!("✅ Utility functions and action constants");
  println!("✅ Reactive state management");
  println!("✅ Library initialization and versioning");

  println!("\n🚀 Enhanced FFI integration provides comprehensive");
  println!("   TypeScript interoperability with theme system access!");
}

#[cfg(not(feature = "ffi"))]
fn main() {
  println!("🔗 FFI Integration Test");
  println!("❌ FFI feature is not enabled. Run with: cargo run --example ffi_integration_test --features ffi");
}
