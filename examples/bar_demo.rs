use reactive_tui::prelude::*;
use reactive_tui::themes::utility_css::UtilityProcessor;

fn main() -> Result<()> {
  println!("🧭 Bar Components Demo\n");

  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width,
    height: 3,
  };
  let theme = reactive_tui::themes::colors::dark_theme();
  let utility = UtilityProcessor::new();

  // Define bar styling themes
  let header_classes = vec![
    "bg-blue-900".to_string(),
    "text-blue-100".to_string(),
    "font-bold".to_string(),
  ];
  let footer_classes = vec!["bg-gray-800".to_string(), "text-gray-300".to_string()];
  let status_classes = vec!["bg-green-800".to_string(), "text-green-100".to_string()];
  let nav_classes = vec!["bg-purple-800".to_string(), "text-purple-100".to_string()];
  let toolbar_classes = vec!["bg-gray-900".to_string(), "text-gray-100".to_string()];

  println!("Header Style: {}", utility.process_classes(&header_classes));
  println!("Footer Style: {}", utility.process_classes(&footer_classes));
  println!("Status Style: {}", utility.process_classes(&status_classes));
  println!("Nav Style: {}", utility.process_classes(&nav_classes));
  println!(
    "Toolbar Style: {}\n",
    utility.process_classes(&toolbar_classes)
  );

  // Header bar with blue theme
  let header = header_bar("demo-header")
    .left("My Application")
    .center("Dashboard")
    .right("v1.0.0")
    .class("bg-blue-900")
    .class("text-blue-100")
    .class("font-bold")
    .build();

  println!("Header Bar (Blue Theme):");
  println!("{}\n", header.render(&layout, Some(&theme)));

  // Footer bar with gray theme
  let footer = footer_bar("demo-footer")
    .left("© 2024 My Company")
    .center("Press F1 for Help | ESC to Exit")
    .right("Ready")
    .class("bg-gray-800")
    .class("text-gray-300")
    .build();

  println!("Footer Bar (Gray Theme):");
  println!("{}\n", footer.render(&layout, Some(&theme)));

  // Status bar with success theme
  let status = status_bar("demo-status")
    .left("✓ Connected to Database")
    .center("Processing: 75% [████████  ] 45/60")
    .right("🕒 12:34 PM")
    .class("bg-green-800")
    .class("text-green-100")
    .build();

  println!("Status Bar (Success Theme):");
  println!("{}\n", status.render(&layout, Some(&theme)));

  // Navigation bar with purple theme
  let nav = navigation_bar("demo-nav")
    .left("🏠 Home")
    .center("📁 Files")
    .right("⚙️ Settings")
    .class("bg-purple-800")
    .class("text-purple-100")
    .build();

  println!("Navigation Bar (Purple Theme):");
  println!("{}\n", nav.render(&layout, Some(&theme)));

  // Toolbar with custom styling
  let toolbar = toolbar("demo-toolbar")
    .left("📄 New")
    .center("✏️ Edit")
    .right("💾 Save")
    .class("bg-gray-900")
    .class("text-gray-100")
    .build();

  println!("Toolbar (Dark Theme):");
  println!("{}", toolbar.render(&layout, Some(&theme)));

  println!("\n🎨 Theme Integration Demo Complete - All bar components with utility CSS styling");

  Ok(())
}
