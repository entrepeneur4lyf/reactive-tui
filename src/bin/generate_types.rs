//! Binary to generate TypeScript type definitions

use std::fs;
use std::path::Path;
use tui_core::components::element::ElementKeyBinding;
use tui_core::components::Element;
use tui_core::events::{ElementAction, KeyCombination};

#[cfg(feature = "typescript")]
use ts_rs::TS;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "typescript")]
    {
        println!("Generating TypeScript types...");

        // Create bindings directory if it doesn't exist
        let bindings_dir = Path::new("bindings");
        if !bindings_dir.exists() {
            fs::create_dir_all(bindings_dir)?;
        }

        // Generate individual type files
        Element::export_all_to(bindings_dir.join("Element.ts"))?;
        ElementKeyBinding::export_all_to(bindings_dir.join("ElementKeyBinding.ts"))?;
        KeyCombination::export_all_to(bindings_dir.join("KeyCombination.ts"))?;
        ElementAction::export_all_to(bindings_dir.join("ElementAction.ts"))?;

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

        fs::write(bindings_dir.join("types.ts"), combined_types)?;

        println!("TypeScript types generated successfully in bindings/");
    }

    #[cfg(not(feature = "typescript"))]
    {
        eprintln!("TypeScript feature not enabled. Run with: cargo run --bin generate_types --features typescript");
        std::process::exit(1);
    }

    Ok(())
}
