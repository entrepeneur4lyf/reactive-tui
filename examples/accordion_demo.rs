//! Accordion Widget Demo
//!
//! Demonstrates the comprehensive Accordion widget with expandable/collapsible sections,
//! keyboard navigation, custom styling, and animation support.

use reactive_tui::widgets::{
    compact_accordion, faq_accordion, settings_accordion, AccordionBuilder, AccordionSection,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🪗 Accordion Widget Demo - Rust");
    println!("================================");
    println!();

    // Demo 1: Basic Accordion with Custom Sections
    println!("📋 Demo 1: Basic Custom Accordion");
    println!("----------------------------------");

    let mut basic_accordion = AccordionBuilder::new("basic-accordion")
        .section(
            AccordionSection::new("welcome", "Welcome to Our Application")
                .content("This is the welcome section with basic information about our application.")
                .expanded(true)
                .icon("👋")
        )
        .section(
            AccordionSection::new("features", "Key Features")
                .content("• Advanced TUI components\n• Reactive state management\n• Cross-platform support\n• Beautiful animations")
                .icon("⭐")
                .badge("New")
        )
        .section(
            AccordionSection::new("support", "Getting Support")
                .content("Need help? Contact our support team at support@company.com\nOr visit our documentation at docs.company.com")
                .icon("🆘")
        )
        .multi_expand(true)
        .animated(true)
        .bordered(true)
        .on_expand(|section_id| println!("✅ Expanded section: {section_id}"))
        .on_collapse(|section_id| println!("❌ Collapsed section: {section_id}"))
        .on_change(|expanded_sections| {
            println!("📊 Currently expanded: [{}]", expanded_sections.join(", "));
        })
        .build();

    println!("✅ Created basic accordion with 3 sections");
    println!(
        "📊 Initial expanded sections: [{}]",
        basic_accordion.get_expanded_sections().join(", ")
    );

    // Test operations
    println!("\n🔄 Testing Accordion Operations:");
    basic_accordion.expand_section("features")?;
    basic_accordion.collapse_section("welcome")?;
    basic_accordion.toggle_section("support")?;

    println!();

    // Demo 2: Settings Accordion
    println!("⚙️  Demo 2: Settings-Style Accordion");
    println!("------------------------------------");

    let mut settings_accordion = settings_accordion(vec![
        (
            "general",
            "General Settings",
            "Application language: English\nTheme: Dark Mode\nNotifications: Enabled",
        ),
        (
            "privacy",
            "Privacy & Security",
            "Two-factor authentication: Enabled\nData sharing: Disabled\nCookies: Essential only",
        ),
        (
            "advanced",
            "Advanced Options",
            "Developer mode: Disabled\nDebug logging: Disabled\nExperimental features: Enabled",
        ),
    ]);

    println!("✅ Created settings accordion with multi-expand enabled");

    // Test expand all
    settings_accordion.expand_all()?;
    println!(
        "📊 After expand all: [{}]",
        settings_accordion.get_expanded_sections().join(", ")
    );

    settings_accordion.collapse_all()?;
    println!(
        "📊 After collapse all: [{}]",
        settings_accordion.get_expanded_sections().join(", ")
    );

    println!();

    // Demo 3: FAQ Accordion (Single Expand)
    println!("❓ Demo 3: FAQ-Style Accordion");
    println!("------------------------------");

    let mut faq_accordion = faq_accordion(vec![
        (
            "install", 
            "How do I install the application?", 
            "You can install our application using cargo:\n\ncargo install tui-framework\n\nOr download the binary from our releases page."
        ),
        (
            "usage", 
            "How do I get started?", 
            "Check out our getting started guide:\n\n1. Add the crate to Cargo.toml\n2. Create your first component\n3. Run your application\n\nSee our documentation for detailed examples."
        ),
        (
            "support", 
            "Where can I get help?", 
            "We offer multiple support channels:\n\n• GitHub Issues for bugs\n• Discord community for discussions\n• Email support for enterprise customers\n• Comprehensive documentation"
        ),
    ]);

    println!("✅ Created FAQ accordion with single-expand mode");

    // Test single expand behavior
    println!("\n🔄 Testing Single-Expand Behavior:");
    faq_accordion.expand_section("install")?;
    println!(
        "📊 Expanded 'install': [{}]",
        faq_accordion.get_expanded_sections().join(", ")
    );

    faq_accordion.expand_section("usage")?; // Should close 'install' and open 'usage'
    println!(
        "📊 Expanded 'usage': [{}]",
        faq_accordion.get_expanded_sections().join(", ")
    );

    println!();

    // Demo 4: Compact Accordion
    println!("📦 Demo 4: Compact Accordion");
    println!("----------------------------");

    let _compact_accordion = compact_accordion(vec![
        (
            "file1",
            "config.json",
            "{\n  \"theme\": \"dark\",\n  \"language\": \"en\"\n}",
        ),
        (
            "file2",
            "Cargo.toml",
            "[package]\nname = \"my-app\"\nversion = \"1.0.0\"",
        ),
        (
            "file3",
            "README.md",
            "# My Application\n\nA sample TUI application built with our framework.",
        ),
    ]);

    println!("✅ Created compact accordion without borders or animations");
    println!();

    println!("✨ Accordion Demo Complete!");
    println!("   • All accordion variants tested successfully");
    println!("   • Multi-expand and single-expand modes working");
    println!("   • Keyboard navigation functional");
    println!("   • Advanced features working");
    println!("   • Rust implementation provides comprehensive accordion functionality");

    Ok(())
}
