use reactive_tui::prelude::*;
use reactive_tui::themes::utility_css::UtilityProcessor;
use reactive_tui::widgets::*;

fn main() -> Result<()> {
  // Get terminal size dynamically
  let (term_width, _term_height) = crossterm::terminal::size().unwrap_or((400, 200));

  println!("🪟 Modal/Dialog System Demo\n");

  let layout = LayoutRect {
    x: 0,
    y: 0,
    width: term_width,
    height: 20,
  };
  let theme = reactive_tui::themes::colors::dark_theme();
  let utility = UtilityProcessor::new();

  // Utility classes for modal styling
  let modal_classes = vec![
    "bg-gray-800".to_string(),
    "border-gray-600".to_string(),
    "text-gray-100".to_string(),
  ];
  let primary_classes = vec!["bg-blue-600".to_string(), "text-white".to_string()];
  let danger_classes = vec!["bg-red-600".to_string(), "text-white".to_string()];

  println!(
    "CSS Utility Classes: {}",
    utility.process_classes(&modal_classes)
  );

  // Alert modal with theme integration
  let alert = alert_modal(
    "demo-alert",
    "System Notification",
    "Database backup completed successfully. All data has been secured.",
  )
  .classes(vec![
    "bg-emerald-50".to_string(),
    "border-emerald-500".to_string(),
    "text-emerald-900".to_string(),
  ]);

  println!("Alert Modal (Success Theme):");
  println!("{}\n", alert.render(&layout, Some(&theme)));

  // Confirmation modal with danger theme
  let confirm = confirm_modal(
    "demo-confirm",
    "Delete Confirmation",
    "Are you sure you want to permanently delete this user account? This action cannot be undone.",
  )
  .classes(vec![
    "bg-red-50".to_string(),
    "border-red-500".to_string(),
    "text-red-900".to_string(),
  ]);

  println!("Confirmation Modal (Danger Theme):");
  println!("{}\n", confirm.render(&layout, Some(&theme)));

  // Prompt modal with info theme
  let prompt = prompt_modal(
    "demo-prompt",
    "Database Connection",
    "Enter the database connection string:",
    "postgresql://localhost:5432/mydb",
  )
  .classes(vec![
    "bg-blue-50".to_string(),
    "border-blue-500".to_string(),
    "text-blue-900".to_string(),
  ]);

  println!("Prompt Modal (Info Theme):");
  println!("{}\n", prompt.render(&layout, Some(&theme)));

  // Custom modal with styled buttons
  let custom_buttons = vec![
    ModalButton::new("save", "Save Changes")
      .variant("primary")
      .classes(primary_classes.clone())
      .closes_modal(),
    ModalButton::new("save_as", "Save As New")
      .variant("secondary")
      .classes(vec!["bg-gray-600".to_string(), "text-gray-100".to_string()])
      .action("save_as_dialog"),
    ModalButton::new("discard", "Discard Changes")
      .variant("danger")
      .classes(danger_classes)
      .closes_modal(),
    ModalButton::new("cancel", "Cancel")
      .variant("secondary")
      .classes(vec!["bg-gray-500".to_string(), "text-gray-100".to_string()])
      .closes_modal(),
  ];
  let custom = custom_modal(
    "demo-custom",
    "Unsaved Document Changes",
    "You have unsaved changes in your document. Choose how to proceed:",
    custom_buttons,
  )
  .classes(vec![
    "bg-yellow-50".to_string(),
    "border-yellow-500".to_string(),
    "text-yellow-900".to_string(),
  ]);

  println!("Custom Modal (Warning Theme):");
  println!("{}\n", custom.render(&layout, Some(&theme)));

  // Full-screen modal with dark theme
  let fullscreen = fullscreen_modal("settings", 
        "APPLICATION SETTINGS\n\nGeneral Settings:\n├─ Language: [English ▼]\n├─ Theme: [Dark ▼]\n└─ Auto-save: [☑]\n\nSecurity Settings:\n├─ Session Timeout: [30 min]\n└─ 2FA: [☑]")
        .classes(vec!["bg-gray-900".to_string(), "border-gray-700".to_string(), "text-gray-100".to_string()]);

  println!("Full-screen Modal (Dark Theme):");
  println!("{}", fullscreen.render(&layout, Some(&theme)));

  println!("\n🎨 Theme Integration Demo Complete - Utility CSS classes applied to all modals");

  Ok(())
}
