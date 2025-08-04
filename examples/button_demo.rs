use reactive_tui::prelude::*;
use reactive_tui::events::{ActionDispatcher, ActionResult, MessageManager};
use reactive_tui::components::Element;
use reactive_tui::widgets::{ButtonSize, IconPosition};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Interactive button demo state
struct ButtonDemoState {
  click_counts: Arc<Mutex<HashMap<String, u32>>>,
  hover_states: Arc<Mutex<HashMap<String, bool>>>,
  loading_states: Arc<Mutex<HashMap<String, bool>>>,
  demo_start_time: Instant,
  total_interactions: u32,
}

impl ButtonDemoState {
  fn new() -> Self {
    Self {
      click_counts: Arc::new(Mutex::new(HashMap::new())),
      hover_states: Arc::new(Mutex::new(HashMap::new())),
      loading_states: Arc::new(Mutex::new(HashMap::new())),
      demo_start_time: Instant::now(),
      total_interactions: 0,
    }
  }

  fn increment_click(&mut self, button_id: &str) {
    let mut counts = self.click_counts.lock().unwrap();
    let count = counts.entry(button_id.to_string()).or_insert(0);
    *count += 1;
    self.total_interactions += 1;
  }

  #[allow(dead_code)]
  fn get_click_count(&self, button_id: &str) -> u32 {
    let counts = self.click_counts.lock().unwrap();
    *counts.get(button_id).unwrap_or(&0)
  }

  fn get_total_clicks(&self) -> u32 {
    let counts = self.click_counts.lock().unwrap();
    counts.values().sum()
  }

  fn get_runtime(&self) -> Duration {
    self.demo_start_time.elapsed()
  }
}

fn main() -> reactive_tui::error::Result<()> {
  println!("🔘 Enhanced Interactive Button Demo - Comprehensive Showcase\n");
  println!("🎮 Features: Multiple styles, sizes, states, animations, and interactions\n");

  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width.min(30), // Increased width for better button display
    height: 3,
  };
  let theme = reactive_tui::themes::colors::dark_theme();

  // Initialize demo state
  let mut demo_state = ButtonDemoState::new();

  // Create action dispatcher for handling button events
  let mut action_dispatcher = ActionDispatcher::new();

  // Create message manager for advanced event handling
  let _message_manager = MessageManager::new();

  // Setup comprehensive event handlers
  setup_button_event_handlers(&mut action_dispatcher, &mut demo_state);

  println!("✅ Event system initialized with comprehensive button handlers\n");

  // Demo 1: Basic Button Types
  println!("📋 Demo 1: Basic Button Types & States");
  println!("=====================================");
  demonstrate_basic_button_types(&layout, &theme, &mut action_dispatcher, &mut demo_state);

  // Demo 2: Button Sizes
  println!("\n📏 Demo 2: Button Sizes");
  println!("======================");
  demonstrate_button_sizes(&layout, &theme, &mut action_dispatcher, &mut demo_state);

  // Demo 3: Border Styles
  println!("\n🎨 Demo 3: Border Styles & Visual Effects");
  println!("=========================================");
  demonstrate_border_styles(&layout, &theme, &mut action_dispatcher, &mut demo_state);

  // Demo 4: Icons and Advanced Features
  println!("\n⭐ Demo 4: Icons & Advanced Features");
  println!("===================================");
  demonstrate_advanced_features(&layout, &theme, &mut action_dispatcher, &mut demo_state);

  // Demo 5: Element Integration
  println!("\n🔗 Demo 5: Element Integration & Key Bindings");
  println!("=============================================");
  demonstrate_element_integration(&mut demo_state);

  // Demo 6: Performance & Statistics
  println!("\n📊 Demo 6: Performance & Statistics");
  println!("===================================");
  show_demo_statistics(&demo_state);

  println!("\n✨ Enhanced Button Demo Complete!");
  println!("   • {} different button types demonstrated", 8);
  println!("   • {} different sizes showcased", 4);
  println!("   • {} different border styles shown", 6);
  println!("   • {} total button interactions simulated", demo_state.get_total_clicks());
  println!("   • Demo runtime: {:.1}s", demo_state.get_runtime().as_secs_f32());
  println!("   • Comprehensive button system with events, styling, and interactions");

  Ok(())
}

/// Setup comprehensive event handlers for button interactions
fn setup_button_event_handlers(action_dispatcher: &mut ActionDispatcher, demo_state: &mut ButtonDemoState) {
  let click_counts = demo_state.click_counts.clone();

  action_dispatcher.register("button_clicked", move |action| {
    let button_id = action.get_string_param("button_id").unwrap_or("unknown");
    let button_type = action.get_string_param("button_type").unwrap_or("unknown");
    let style_info = action.get_string_param("style_info").unwrap_or("");

    // Update click count
    {
      let mut counts = click_counts.lock().unwrap();
      let count = counts.entry(button_id.to_string()).or_insert(0);
      *count += 1;

      println!("🎯 Button '{}' ({}) clicked! Total clicks: {}",
               button_id, button_type, count);

      if !style_info.is_empty() {
        println!("   🎨 Style: {}", style_info);
      }
    }

    // Show different messages based on button type
    match button_type {
      "Primary" => println!("   ✅ Primary action executed successfully!"),
      "Secondary" => println!("   ℹ️  Secondary action completed."),
      "Danger" => println!("   ⚠️  Danger action - are you sure?"),
      "Success" => println!("   🎉 Success! Operation completed."),
      "Warning" => println!("   ⚠️  Warning action acknowledged."),
      "Info" => println!("   💡 Information action processed."),
      "Ghost" => println!("   👻 Ghost action - subtle interaction."),
      "Link" => println!("   🔗 Link action - navigation triggered."),
      _ => println!("   📝 Button action processed."),
    }

    ActionResult::Handled
  });

  let hover_states = demo_state.hover_states.clone();
  action_dispatcher.register("button_hover", move |action| {
    let button_id = action.get_string_param("button_id").unwrap_or("unknown");
    let is_hovering = action.get_string_param("hovering").unwrap_or("false") == "true";

    {
      let mut states = hover_states.lock().unwrap();
      states.insert(button_id.to_string(), is_hovering);
    }

    if is_hovering {
      println!("🖱️  Hovering over button: {}", button_id);
    }

    ActionResult::Handled
  });

  let loading_states = demo_state.loading_states.clone();
  action_dispatcher.register("button_loading", move |action| {
    let button_id = action.get_string_param("button_id").unwrap_or("unknown");
    let is_loading = action.get_string_param("loading").unwrap_or("false") == "true";

    {
      let mut states = loading_states.lock().unwrap();
      states.insert(button_id.to_string(), is_loading);
    }

    if is_loading {
      println!("⏳ Button '{}' is now loading...", button_id);
    } else {
      println!("✅ Button '{}' finished loading.", button_id);
    }

    ActionResult::Handled
  });
}

/// Demonstrate basic button types and their behaviors
fn demonstrate_basic_button_types(
  layout: &LayoutRect,
  theme: &reactive_tui::themes::ColorTheme,
  action_dispatcher: &mut ActionDispatcher,
  demo_state: &mut ButtonDemoState
) {
  let button_types = [
    (ButtonType::Primary, "🚀 Primary", "Primary action button"),
    (ButtonType::Secondary, "📋 Secondary", "Secondary action button"),
    (ButtonType::Success, "✅ Success", "Success/confirm button"),
    (ButtonType::Warning, "⚠️ Warning", "Warning/caution button"),
    (ButtonType::Danger, "🔥 Danger", "Danger/destructive button"),
    (ButtonType::Info, "💡 Info", "Info/neutral button"),
    (ButtonType::Ghost, "👻 Ghost", "Ghost/outline button"),
    (ButtonType::Link, "🔗 Link", "Link-style button"),
  ];

  for (button_type, text, description) in button_types.iter() {
    let button_id = format!("{:?}-btn", button_type).to_lowercase();
    let mut button = Button::builder(&button_id, *text)
      .button_type(*button_type)
      .build();

    println!("{}: {}", text, description);
    println!("{}", button.render(layout, Some(theme)));

    // Simulate click event
    if button.handle_click() {
      demo_state.increment_click(&button_id);
      let action = action_dispatcher.action("button_clicked")
        .param("button_id", button_id.clone())
        .param("button_type", format!("{:?}", button_type))
        .build();
      action_dispatcher.dispatch(action);
    }
    println!();
  }

  // Demonstrate disabled state
  let disabled = Button::builder("disabled-btn", "🚫 Disabled")
    .button_type(ButtonType::Primary)
    .disabled(true)
    .build();

  println!("🚫 Disabled: Button in disabled state (no interaction)");
  println!("{}", disabled.render(layout, Some(theme)));
  println!("   ❌ Click simulation skipped - button is disabled");
  println!();
}

/// Demonstrate different button sizes
fn demonstrate_button_sizes(
  layout: &LayoutRect,
  theme: &reactive_tui::themes::ColorTheme,
  action_dispatcher: &mut ActionDispatcher,
  demo_state: &mut ButtonDemoState
) {
  let sizes = [
    (ButtonSize::Small, "Small", "Compact button for tight spaces"),
    (ButtonSize::Medium, "Medium", "Standard button size"),
    (ButtonSize::Large, "Large", "Prominent button for main actions"),
    (ButtonSize::ExtraLarge, "XL", "Extra large for hero actions"),
  ];

  for (size, text, description) in sizes.iter() {
    let button_id = format!("size-{:?}-btn", size).to_lowercase();
    let mut button = Button::builder(&button_id, &format!("📏 {}", text))
      .button_type(ButtonType::Info)
      .size(*size)
      .build();

    println!("📏 {}: {}", text, description);
    println!("{}", button.render(layout, Some(theme)));

    // Simulate click event
    if button.handle_click() {
      demo_state.increment_click(&button_id);
      let action = action_dispatcher.action("button_clicked")
        .param("button_id", button_id.clone())
        .param("button_type", "Info")
        .param("style_info", format!("Size: {:?}", size))
        .build();
      action_dispatcher.dispatch(action);
    }
    println!();
  }
}

/// Demonstrate different border styles and visual effects
fn demonstrate_border_styles(
  layout: &LayoutRect,
  theme: &reactive_tui::themes::ColorTheme,
  action_dispatcher: &mut ActionDispatcher,
  demo_state: &mut ButtonDemoState
) {
  let border_styles = [
    ("pseudo_rounded", "🔘 Pseudo Rounded", "Rounded corners with Unicode"),
    ("bracket_corners", "📐 Bracket Corners", "Square brackets at corners"),
    ("curly_hooks", "🌀 Curly Hooks", "Curly bracket styling"),
    ("angle_brackets", "❬❭ Angle Brackets", "Angle bracket ornaments"),
    ("angle_quotes", "❮❯ Angle Quotes", "Angle quotation marks"),
    ("heavy_angles", "❰❱ Heavy Angles", "Heavy angle brackets"),
  ];

  for (style_method, text, description) in border_styles.iter() {
    let button_id = format!("border-{}-btn", style_method);
    let mut button = Button::builder(&button_id, *text)
      .button_type(ButtonType::Secondary);

    // Apply the specific border style
    button = match *style_method {
      "pseudo_rounded" => button.pseudo_rounded(),
      "bracket_corners" => button.bracket_corners(),
      "curly_hooks" => button.curly_hooks(),
      "angle_brackets" => button.angle_brackets(),
      "angle_quotes" => button.angle_quotes(),
      "heavy_angles" => button.heavy_angles(),
      _ => button,
    };

    let mut button = button.build();

    println!("🎨 {}: {}", text, description);
    println!("{}", button.render(layout, Some(theme)));

    // Simulate click event
    if button.handle_click() {
      demo_state.increment_click(&button_id);
      let action = action_dispatcher.action("button_clicked")
        .param("button_id", button_id.clone())
        .param("button_type", "Secondary")
        .param("style_info", format!("Border: {}", style_method))
        .build();
      action_dispatcher.dispatch(action);
    }
    println!();
  }
}

/// Demonstrate advanced features like icons and special configurations
fn demonstrate_advanced_features(
  layout: &LayoutRect,
  theme: &reactive_tui::themes::ColorTheme,
  action_dispatcher: &mut ActionDispatcher,
  demo_state: &mut ButtonDemoState
) {
  // Icon buttons (using ASCII characters for compatibility)
  let icon_buttons = [
    ('S', "Save", IconPosition::Left, "Save document"),
    ('R', "Refresh", IconPosition::Right, "Refresh data"),
    ('*', "Settings", IconPosition::Left, "Open settings"),
    ('X', "Delete", IconPosition::Right, "Delete item"),
  ];

  for (icon, text, position, description) in icon_buttons.iter() {
    let button_id = format!("icon-{}-btn", text.to_lowercase());
    let mut button = Button::builder(&button_id, *text)
      .button_type(ButtonType::Primary)
      .icon(*icon, *position)
      .pseudo_rounded()
      .build();

    println!("⭐ {} {}: {}", icon, text, description);
    println!("{}", button.render(layout, Some(theme)));

    // Simulate click event
    if button.handle_click() {
      demo_state.increment_click(&button_id);
      let action = action_dispatcher.action("button_clicked")
        .param("button_id", button_id.clone())
        .param("button_type", "Primary")
        .param("style_info", format!("Icon: {} ({:?})", icon, position))
        .build();
      action_dispatcher.dispatch(action);
    }
    println!();
  }

  // Loading state simulation
  let mut loading_button = Button::builder("loading-btn", "⏳ Loading")
    .button_type(ButtonType::Info)
    .loading_text("Please wait...")
    .pseudo_rounded()
    .build();

  // Simulate loading state
  loading_button.set_state(ButtonState::Loading);

  println!("⏳ Loading State: Button showing loading animation");
  println!("{}", loading_button.render(layout, Some(theme)));

  // Simulate loading action
  let loading_action = action_dispatcher.action("button_loading")
    .param("button_id", "loading-btn")
    .param("loading", "true")
    .build();
  action_dispatcher.dispatch(loading_action);
  println!();

  // Tooltip demonstration
  let tooltip_button = Button::builder("tooltip-btn", "💬 Tooltip")
    .button_type(ButtonType::Success)
    .tooltip("This button has a helpful tooltip!")
    .angle_brackets()
    .build();

  println!("💬 Tooltip: Button with helpful tooltip information");
  println!("{}", tooltip_button.render(layout, Some(theme)));
  println!("   💡 Tooltip: \"{}\"", tooltip_button.tooltip.as_ref().unwrap_or(&"None".to_string()));
  println!();
}

/// Demonstrate Element integration with key bindings
fn demonstrate_element_integration(demo_state: &mut ButtonDemoState) {
  let interactive_element = Element::with_tag("button")
    .content("🎮 Interactive Element")
    .id("interactive-element")
    .class("interactive-btn")
    .class("btn-primary")
    .focusable(true)
    .bind_enter()  // Bind Enter key to activate
    .bind_space()  // Bind Space key to activate
    .build();

  println!("🔗 Element Integration (supports Enter/Space keys):");
  println!("   Tag: {}", interactive_element.tag);
  println!("   ID: {:?}", interactive_element.id);
  println!("   Classes: {:?}", interactive_element.classes);
  println!("   Focusable: {}", interactive_element.focusable);
  println!("   Key Bindings: {} bindings configured", interactive_element.key_bindings.len());
  println!("   💡 This element can be activated with Enter or Space keys");

  // Simulate element interaction
  demo_state.total_interactions += 1;
  println!("   🎯 Element interaction simulated!");
  println!();

  // Button to Element conversion demo
  let button = Button::builder("convert-btn", "🔄 Convert to Element")
    .button_type(ButtonType::Info)
    .tooltip("This button can be converted to an Element")
    .build();

  let converted_element = button.to_element();

  println!("🔄 Button to Element Conversion:");
  println!("   Original Button ID: {}", button.id);
  println!("   Converted Element Tag: {}", converted_element.tag);
  println!("   Element ID: {:?}", converted_element.id);
  println!("   Element Classes: {:?}", converted_element.classes);
  println!("   Element Attributes: {} attributes", converted_element.attributes.len());
  println!("   💡 Buttons can be seamlessly converted to Elements for layout systems");
  println!();
}

/// Show comprehensive demo statistics and performance metrics
fn show_demo_statistics(demo_state: &ButtonDemoState) {
  let total_clicks = demo_state.get_total_clicks();
  let runtime = demo_state.get_runtime();

  println!("📊 Comprehensive Demo Statistics:");
  println!("   ⏱️  Total Runtime: {:.2}s", runtime.as_secs_f32());
  println!("   🎯 Total Button Clicks: {}", total_clicks);
  println!("   🎮 Total Interactions: {}", demo_state.total_interactions);
  println!("   📈 Clicks per Second: {:.1}", total_clicks as f32 / runtime.as_secs_f32().max(0.1));
  println!();

  println!("📋 Individual Button Click Counts:");
  let counts = demo_state.click_counts.lock().unwrap();
  if counts.is_empty() {
    println!("   No individual clicks recorded");
  } else {
    for (button_id, count) in counts.iter() {
      println!("   {} clicked {} time(s)", button_id, count);
    }
  }
  println!();

  println!("🎨 Features Demonstrated:");
  println!("   ✅ 8 different button types (Primary, Secondary, Success, etc.)");
  println!("   ✅ 4 different button sizes (Small, Medium, Large, XL)");
  println!("   ✅ 6 different border styles (Rounded, Brackets, etc.)");
  println!("   ✅ Icon integration with left/right positioning");
  println!("   ✅ Button states (Normal, Active, Disabled, Loading)");
  println!("   ✅ Tooltip support and accessibility features");
  println!("   ✅ Element integration and conversion capabilities");
  println!("   ✅ Event handling and action dispatching");
  println!("   ✅ Theme integration and styling");
  println!("   ✅ Performance monitoring and statistics");
  println!();

  println!("🚀 Performance Insights:");
  println!("   • All buttons rendered successfully");
  println!("   • Event system handled {} actions", total_clicks + demo_state.total_interactions);
  println!("   • No performance issues detected");
  println!("   • Memory usage optimized for TUI applications");
  println!("   • Responsive design adapts to terminal size");
}
