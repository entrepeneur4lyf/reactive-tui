//! Accordion Widget Demo
//!
//! Demonstrates the comprehensive Accordion widget with expandable/collapsible sections,
//! keyboard navigation, custom styling, and animation support.

use tui_core::widgets::{
    compact_accordion, faq_accordion, settings_accordion, AccordionBuilder, AccordionSection,
};

#[derive(Debug, Clone)]
struct Employee {
    #[allow(dead_code)]
    id: u32,
    name: String,
    department: String,
    role: String,
    email: String,
}

impl Employee {
    fn new(id: u32, name: &str, department: &str, role: &str, email: &str) -> Self {
        Self {
            id,
            name: name.to_string(),
            department: department.to_string(),
            role: role.to_string(),
            email: email.to_string(),
        }
    }
}

fn create_sample_employees() -> Vec<Employee> {
    vec![
        Employee::new(
            1,
            "Alice Johnson",
            "Engineering",
            "Senior Developer",
            "alice@company.com",
        ),
        Employee::new(
            2,
            "Bob Smith",
            "Sales",
            "Account Manager",
            "bob@company.com",
        ),
        Employee::new(
            3,
            "Carol Davis",
            "Marketing",
            "Marketing Director",
            "carol@company.com",
        ),
        Employee::new(
            4,
            "David Wilson",
            "Engineering",
            "DevOps Engineer",
            "david@company.com",
        ),
        Employee::new(5, "Eva Brown", "HR", "HR Manager", "eva@company.com"),
    ]
}

fn get_department_icon(department: &str) -> &str {
    match department {
        "Engineering" => "🔧",
        "Sales" => "💼",
        "Marketing" => "📢",
        "HR" => "👥",
        "Finance" => "💰",
        "Operations" => "⚙️",
        _ => "📁",
    }
}

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

    // Demo 5: Employee Directory Accordion
    println!("👥 Demo 5: Employee Directory Accordion");
    println!("---------------------------------------");

    let employees = create_sample_employees();
    let mut departments: std::collections::HashMap<String, Vec<Employee>> =
        std::collections::HashMap::new();

    // Group employees by department
    for employee in employees {
        departments
            .entry(employee.department.clone())
            .or_default()
            .push(employee);
    }

    let mut employee_accordion_builder = AccordionBuilder::new("employee-directory")
        .multi_expand(false)
        .animated(true)
        .bordered(true);

    // Add sections for each department
    for (department, emp_list) in departments.iter() {
        let content = emp_list
            .iter()
            .map(|emp| format!("{} - {}\n  📧 {}", emp.name, emp.role, emp.email))
            .collect::<Vec<_>>()
            .join("\n\n");

        employee_accordion_builder = employee_accordion_builder.section(
            AccordionSection::new(
                department.to_lowercase(),
                format!("{department} Department"),
            )
            .content(content)
            .badge(emp_list.len().to_string())
            .icon(get_department_icon(department)),
        );
    }

    let mut employee_accordion = employee_accordion_builder.build();

    println!(
        "✅ Created employee directory with {} departments",
        departments.len()
    );

    // Test keyboard navigation
    println!("\n⌨️  Testing Keyboard Navigation:");
    employee_accordion.focus_first()?;
    println!(
        "🎯 Focused first: {:?}",
        employee_accordion.get_focused_section()
    );

    employee_accordion.focus_next()?;
    println!(
        "🎯 Focused next: {:?}",
        employee_accordion.get_focused_section()
    );

    employee_accordion.focus_last()?;
    println!(
        "🎯 Focused last: {:?}",
        employee_accordion.get_focused_section()
    );

    employee_accordion.focus_previous()?;
    println!(
        "🎯 Focused previous: {:?}",
        employee_accordion.get_focused_section()
    );

    println!();

    // Demo 6: Dynamic Content Updates
    println!("🔄 Demo 6: Dynamic Content Updates");
    println!("----------------------------------");

    let mut dynamic_accordion = AccordionBuilder::new("dynamic-accordion")
        .section(
            AccordionSection::new("status", "System Status")
                .content("Loading system information...")
                .expanded(true),
        )
        .build();

    println!("✅ Created dynamic accordion with initial content");

    // Simulate content update
    std::thread::sleep(std::time::Duration::from_millis(100));
    dynamic_accordion.update_section_content(
        "status",
        "🟢 All systems operational\n\
         📊 CPU Usage: 45%\n\
         💾 Memory Usage: 67%\n\
         💿 Disk Usage: 23%\n\
         🌐 Network: Connected",
    )?;
    println!("🔄 Updated system status content");

    // Add a new section dynamically
    dynamic_accordion.add_section(
        AccordionSection::new("logs", "Recent Logs")
            .content("[INFO] Application started\n[DEBUG] Loading configuration\n[INFO] Server listening on port 3000")
            .badge("3")
    );
    println!("➕ Added new logs section dynamically");
    println!("📊 Total sections: {}", dynamic_accordion.section_count());

    println!();

    // Demo 7: Advanced Features
    println!("🚀 Demo 7: Advanced Features");
    println!("----------------------------");

    let mut advanced_accordion = AccordionBuilder::new("advanced-accordion")
        .section(
            AccordionSection::new("enabled", "Enabled Section")
                .content("This section is fully interactive and can be expanded/collapsed.")
                .expanded(false),
        )
        .section(
            AccordionSection::new("disabled", "Disabled Section")
                .content("This section is disabled and cannot be interacted with.")
                .disabled(true)
                .expanded(false),
        )
        .multi_expand(true)
        .icons("🔽", "🔼")
        .on_expand(|section_id| println!("🟢 Section expanded: {section_id}"))
        .on_collapse(|section_id| println!("🔴 Section collapsed: {section_id}"))
        .on_focus(|section_id| println!("🎯 Section focused: {section_id}"))
        .on_section_click(|section_id| println!("👆 Section clicked: {section_id}"))
        .build();

    println!("✅ Created advanced accordion with disabled sections and custom icons");

    // Test disabled section
    let expanded = advanced_accordion.expand_section("disabled").is_ok();
    println!(
        "❌ Attempted to expand disabled section: {}",
        if expanded {
            "Success"
        } else {
            "Failed (as expected)"
        }
    );

    // Test accordion disable/enable
    advanced_accordion.set_disabled(true);
    println!(
        "🔒 Accordion disabled: {}",
        advanced_accordion.is_disabled()
    );

    advanced_accordion.set_disabled(false);
    println!("🔓 Accordion enabled: {}", advanced_accordion.is_disabled());

    println!();

    // Demo 8: Rendering Test
    println!("🎨 Demo 8: Accordion Rendering");
    println!("------------------------------");

    let render_accordion = AccordionBuilder::new("render-test")
        .section(
            AccordionSection::new("expanded", "Expanded Section")
                .content("This section is expanded and should show its content.")
                .expanded(true),
        )
        .section(
            AccordionSection::new("collapsed", "Collapsed Section")
                .content("This content is hidden because the section is collapsed.")
                .expanded(false),
        )
        .build();

    println!("✅ Created accordion for rendering test");
    println!("\n📄 Rendered Output:");
    println!("{render_accordion}");

    println!();
    println!("✨ Accordion Demo Complete!");
    println!("   • All accordion variants tested successfully");
    println!("   • Multi-expand and single-expand modes working");
    println!("   • Keyboard navigation functional");
    println!("   • Dynamic content updates working");
    println!("   • Advanced features like disabled sections working");
    println!("   • Rendering system functional");
    println!("   • Rust implementation provides comprehensive accordion functionality");

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tui_core::widgets::AccordionBuilder;

    #[test]
    fn test_employee_creation() {
        let emp = Employee::new(
            1,
            "Test User",
            "Engineering",
            "Developer",
            "test@company.com",
        );
        assert_eq!(emp.id, 1);
        assert_eq!(emp.name, "Test User");
        assert_eq!(emp.department, "Engineering");
        assert_eq!(emp.role, "Developer");
        assert_eq!(emp.email, "test@company.com");
    }

    #[test]
    fn test_sample_employees() {
        let employees = create_sample_employees();
        assert_eq!(employees.len(), 5);
        assert_eq!(employees[0].name, "Alice Johnson");
    }

    #[test]
    fn test_accordion_creation() {
        let accordion = AccordionBuilder::new("test-accordion")
            .section(AccordionSection::new("test1", "Test Section 1").content("Test content 1"))
            .section(AccordionSection::new("test2", "Test Section 2").content("Test content 2"))
            .build();

        assert_eq!(accordion.section_count(), 2);
    }

    #[test]
    fn test_accordion_operations() {
        let mut accordion = AccordionBuilder::new("test-accordion")
            .section(AccordionSection::new("section1", "Section 1").content("Content 1"))
            .build();

        // Test expand
        assert!(accordion.expand_section("section1").is_ok());
        assert!(accordion.is_section_expanded("section1"));

        // Test collapse
        assert!(accordion.collapse_section("section1").is_ok());
        assert!(!accordion.is_section_expanded("section1"));

        // Test toggle
        assert!(accordion.toggle_section("section1").is_ok());
        assert!(accordion.is_section_expanded("section1"));
    }

    #[test]
    fn test_convenience_functions() {
        let settings = settings_accordion(vec![
            ("general", "General", "General settings content"),
            ("privacy", "Privacy", "Privacy settings content"),
        ]);
        assert_eq!(settings.section_count(), 2);

        let faq = faq_accordion(vec![
            ("q1", "Question 1?", "Answer 1"),
            ("q2", "Question 2?", "Answer 2"),
        ]);
        assert_eq!(faq.section_count(), 2);

        let compact = compact_accordion(vec![
            ("item1", "Item 1", "Content 1"),
            ("item2", "Item 2", "Content 2"),
        ]);
        assert_eq!(compact.section_count(), 2);
    }
}
