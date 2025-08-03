//! Binary to generate TypeScript type definitions

#[cfg(feature = "typescript")]
use ts_rs::TS;

fn main() {
  #[cfg(feature = "typescript")]
  {
    use reactive_tui::components::element::ElementKeyBinding;
    use reactive_tui::components::Element;
    use reactive_tui::events::{ElementAction, KeyCombination};
    use std::fs;
    use std::path::Path;

    println!("Generating TypeScript types...");

    // Create bindings directory if it doesn't exist
    let bindings_dir = Path::new("bindings");
    if !bindings_dir.exists() {
      fs::create_dir_all(bindings_dir).expect("Failed to create bindings directory");
    }

    // Generate individual type files
    Element::export_all_to(bindings_dir.join("Element.ts"))
      .expect("Failed to export Element types");
    ElementKeyBinding::export_all_to(bindings_dir.join("ElementKeyBinding.ts"))
      .expect("Failed to export ElementKeyBinding types");
    KeyCombination::export_all_to(bindings_dir.join("KeyCombination.ts"))
      .expect("Failed to export KeyCombination types");
    ElementAction::export_all_to(bindings_dir.join("ElementAction.ts"))
      .expect("Failed to export ElementAction types");

    // Create a combined types file
    let mut combined_types = String::new();
    combined_types.push_str("// Auto-generated TypeScript types from Rust structs\n\n");

    combined_types.push_str(&Element::decl());
    combined_types.push_str("\n\n");
    combined_types.push_str(&ElementKeyBinding::decl());
    combined_types.push_str("\n\n");
    combined_types.push_str(&KeyCombination::decl());
    combined_types.push_str("\n\n");
    combined_types.push_str(&ElementAction::decl());
    combined_types.push('\n');

    fs::write(bindings_dir.join("types.ts"), combined_types)
      .expect("Failed to write combined types");

    println!("TypeScript types generated successfully in bindings/");
  }

  #[cfg(not(feature = "typescript"))]
  {
    eprintln!("TypeScript feature not enabled. Run with: cargo run --bin generate_types --features typescript");
    std::process::exit(1);
  }
}
