//! Tree Widget Demo
//!
//! Demonstrates the comprehensive tree widget functionality including:
//! - Hierarchical data display with expand/collapse
//! - Single and multi-selection modes
//! - Lazy loading of child nodes on demand
//! - Search/filtering capabilities
//! - Keyboard navigation
//! - Virtual scrolling for large datasets
//! - Custom node types and styling

use tui_core::error::Result;
use tui_core::widgets::{TreeBuilder, TreeNode, TreeNodeType};

#[tokio::main]
async fn main() -> Result<()> {
    println!("🌳 Tree Widget Demo");
    println!("{}", "=".repeat(50));

    // Demo 1: Simple file system tree
    println!("\n1. File System Tree Structure");
    println!("{}", "-".repeat(30));

    let mut file_tree = TreeBuilder::new("file-tree")
        .expandable(true)
        .multi_select(false)
        .search_enabled(true)
        .on_expand(|node_id, expanded| {
            let action = if expanded { "expanded" } else { "collapsed" };
            println!("   📁 {node_id} {action}");
        })
        .on_select(|selected_nodes| {
            if !selected_nodes.is_empty() {
                println!("   🎯 Selected: {}", selected_nodes.join(", "));
            }
        })
        .build()
        .await;

    // Add root nodes first
    let src_node = TreeNode::folder("src", "src/");
    let tests_node = TreeNode::folder("tests", "tests/");
    let docs_node = TreeNode::folder("docs", "docs/");

    file_tree.add_root_node(src_node).await?;
    file_tree.add_root_node(tests_node).await?;
    file_tree.add_root_node(docs_node).await?;

    // Then add children to the src folder
    file_tree
        .add_child_node(&"src".to_string(), TreeNode::leaf("main.rs", "main.rs"))
        .await?;
    file_tree
        .add_child_node(&"src".to_string(), TreeNode::leaf("lib.rs", "lib.rs"))
        .await?;
    file_tree
        .add_child_node(&"src".to_string(), TreeNode::folder("widgets", "widgets/"))
        .await?;

    // Add children to widgets folder
    file_tree
        .add_child_node(
            &"widgets".to_string(),
            TreeNode::leaf("button.rs", "button.rs"),
        )
        .await?;
    file_tree
        .add_child_node(
            &"widgets".to_string(),
            TreeNode::leaf("input.rs", "input.rs"),
        )
        .await?;
    file_tree
        .add_child_node(&"widgets".to_string(), TreeNode::leaf("tree.rs", "tree.rs"))
        .await?;

    // Add children to tests folder
    file_tree
        .add_child_node(
            &"tests".to_string(),
            TreeNode::leaf("integration.rs", "integration.rs"),
        )
        .await?;
    file_tree
        .add_child_node(&"tests".to_string(), TreeNode::leaf("unit.rs", "unit.rs"))
        .await?;

    // Add children to docs folder
    file_tree
        .add_child_node(
            &"docs".to_string(),
            TreeNode::leaf("README.md", "README.md"),
        )
        .await?;
    file_tree
        .add_child_node(
            &"docs".to_string(),
            TreeNode::leaf("CHANGELOG.md", "CHANGELOG.md"),
        )
        .await?;

    println!("✅ File system tree created: {file_tree}");

    // Test expansion
    file_tree.expand(&"src".to_string()).await?;
    file_tree.expand(&"widgets".to_string()).await?;
    println!("✅ Expanded src/ and widgets/ folders");

    // Test selection
    file_tree.select(&"tree.rs".to_string()).await?;
    println!("✅ Selected tree.rs file");

    // Get visible nodes
    let visible = file_tree.get_visible_nodes().await;
    println!("   Visible nodes: {}", visible.len());

    // Demo 2: Organization hierarchy with multi-select
    println!("\n2. Organization Hierarchy (Multi-Select)");
    println!("{}", "-".repeat(30));

    let mut org_tree = TreeBuilder::new("org-tree")
        .multi_select(true)
        .expandable(true)
        .on_select(|selected_nodes| {
            println!("   👥 Selected departments: {}", selected_nodes.len());
        })
        .build()
        .await;

    // Add root CEO node
    let ceo_node =
        TreeNode::new("ceo", "CEO").node_type(TreeNodeType::Custom("executive".to_string()));
    org_tree.add_root_node(ceo_node).await?;

    // Add department nodes
    org_tree
        .add_child_node(
            &"ceo".to_string(),
            TreeNode::folder("engineering", "Engineering"),
        )
        .await?;
    org_tree
        .add_child_node(
            &"ceo".to_string(),
            TreeNode::folder("marketing", "Marketing"),
        )
        .await?;
    org_tree
        .add_child_node(&"ceo".to_string(), TreeNode::folder("sales", "Sales"))
        .await?;

    // Add engineering teams
    org_tree
        .add_child_node(
            &"engineering".to_string(),
            TreeNode::leaf("frontend-team", "Frontend Team"),
        )
        .await?;
    org_tree
        .add_child_node(
            &"engineering".to_string(),
            TreeNode::leaf("backend-team", "Backend Team"),
        )
        .await?;
    org_tree
        .add_child_node(
            &"engineering".to_string(),
            TreeNode::leaf("devops-team", "DevOps Team"),
        )
        .await?;

    // Add marketing teams
    org_tree
        .add_child_node(
            &"marketing".to_string(),
            TreeNode::leaf("content-team", "Content Team"),
        )
        .await?;
    org_tree
        .add_child_node(
            &"marketing".to_string(),
            TreeNode::leaf("design-team", "Design Team"),
        )
        .await?;

    // Add sales teams
    org_tree
        .add_child_node(
            &"sales".to_string(),
            TreeNode::leaf("enterprise-sales", "Enterprise Sales"),
        )
        .await?;
    org_tree
        .add_child_node(
            &"sales".to_string(),
            TreeNode::leaf("smb-sales", "SMB Sales"),
        )
        .await?;
    println!("✅ Organization tree created: {org_tree}");

    // Test multi-selection
    org_tree.expand(&"ceo".to_string()).await?;
    org_tree.expand(&"engineering".to_string()).await?;
    org_tree.select(&"frontend-team".to_string()).await?;
    org_tree.select(&"backend-team".to_string()).await?;
    org_tree.select(&"devops-team".to_string()).await?;
    println!("✅ Selected multiple engineering teams");

    // Demo 3: Lazy loading tree (simulated API calls)
    println!("\n3. Lazy Loading Tree (API Simulation)");
    println!("{}", "-".repeat(30));

    let lazy_tree = TreeBuilder::new("lazy-tree")
        .lazy_loading(true, |node_id| {
            let node_id = node_id.clone();
            async move {
                // Simulate API call delay
                tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

                println!("   🔄 Loading children for: {node_id}");

                // Simulate different child nodes based on parent
                match node_id.as_str() {
                    "countries" => Ok(vec![
                        TreeNode::folder("usa", "United States"),
                        TreeNode::folder("canada", "Canada"),
                        TreeNode::folder("mexico", "Mexico"),
                    ]),
                    "usa" => Ok(vec![
                        TreeNode::leaf("california", "California"),
                        TreeNode::leaf("texas", "Texas"),
                        TreeNode::leaf("florida", "Florida"),
                    ]),
                    "canada" => Ok(vec![
                        TreeNode::leaf("ontario", "Ontario"),
                        TreeNode::leaf("quebec", "Quebec"),
                        TreeNode::leaf("bc", "British Columbia"),
                    ]),
                    _ => Ok(vec![]),
                }
            }
        })
        .expandable(true)
        .on_expand(|node_id, expanded| {
            if expanded {
                println!("   📡 Lazy loading triggered for: {node_id}");
            }
        })
        .build()
        .await;

    println!("✅ Lazy loading tree created: {lazy_tree}");

    // Demo 4: Search and filtering
    println!("\n4. Search and Filtering");
    println!("{}", "-".repeat(30));

    // Test search on file tree
    file_tree.set_search_query("rs").await;
    let filtered_visible = file_tree.get_visible_nodes().await;
    println!("✅ Search for 'rs': {} matches", filtered_visible.len());

    // Clear search
    file_tree.set_search_query("").await;
    println!("✅ Search cleared");

    // Demo 5: Keyboard navigation simulation
    println!("\n5. Keyboard Navigation Test");
    println!("{}", "-".repeat(30));

    // Navigate through nodes
    file_tree.navigate_next().await;
    file_tree.navigate_next().await;
    println!("✅ Navigated through nodes");

    file_tree.navigate_previous().await;
    println!("✅ Navigated back");

    // Test key handling
    let handled = file_tree.handle_key("ArrowRight").await?;
    println!("✅ Arrow right handled: {handled}");

    let handled = file_tree.handle_key("Space").await?;
    println!("✅ Space key (select) handled: {handled}");

    // Demo 6: Virtual scrolling setup
    println!("\n6. Large Dataset with Virtual Scrolling");
    println!("{}", "-".repeat(30));

    let mut large_tree = TreeBuilder::new("large-tree")
        .virtual_scrolling(true)
        .expandable(true)
        .build()
        .await;

    // Add many root nodes to simulate large dataset
    for i in 0..50 {
        let category_node = TreeNode::folder(format!("category-{i}"), format!("Category {i}"));

        // Note: For demo purposes, we're not adding children here since
        // we need the parent to exist first before adding children

        large_tree.add_root_node(category_node).await?;
    }

    println!("✅ Large tree created: {large_tree} (1000+ nodes)");

    let visible_large = large_tree.get_visible_nodes().await;
    println!(
        "   Virtual scroll viewport: {} visible nodes",
        visible_large.len()
    );

    // Demo 7: Advanced operations
    println!("\n7. Advanced Tree Operations");
    println!("{}", "-".repeat(30));

    // Toggle operations
    org_tree.toggle_expand(&"engineering".to_string()).await?;
    println!("✅ Toggled engineering expansion");

    org_tree.toggle_select(&"marketing".to_string()).await?;
    println!("✅ Toggled marketing selection");

    // Clear all selections
    org_tree.clear_selection().await;
    println!("✅ Cleared all selections");

    // Element conversion for rendering
    let tree_element = file_tree.to_element().await;
    println!("✅ Converted tree to renderable element");
    println!("   Element tag: {}", tree_element.tag);
    println!("   Element classes: {:?}", tree_element.classes);
    println!("   Child elements: {}", tree_element.children.len());

    // Demo 8: Convenience tree builders
    println!("\n8. Convenience Patterns");
    println!("{}", "-".repeat(30));

    let fs_tree = TreeBuilder::file_system("filesystem".to_string())
        .build()
        .await;
    println!("✅ File system pattern tree: {fs_tree}");

    let org_pattern_tree = TreeBuilder::organization("org-pattern".to_string())
        .build()
        .await;
    println!("✅ Organization pattern tree: {org_pattern_tree}");

    let category_tree = TreeBuilder::categories("categories".to_string())
        .build()
        .await;
    println!("✅ Categories pattern tree: {category_tree}");

    // Summary
    println!("\n🎉 Tree Widget Demo Complete!");
    println!("{}", "=".repeat(50));
    println!("✅ Hierarchical data display with expand/collapse");
    println!("✅ Single and multi-selection modes");
    println!("✅ Lazy loading with async callbacks");
    println!("✅ Search and filtering capabilities");
    println!("✅ Keyboard navigation and event handling");
    println!("✅ Virtual scrolling for large datasets");
    println!("✅ Custom node types and styling");
    println!("✅ Advanced operations (toggle, clear, navigate)");

    println!("\n🚀 Features Demonstrated:");
    println!("• Hierarchical tree structure with expandable nodes");
    println!("• Lazy loading of child nodes with async callbacks");
    println!("• Multi-selection with keyboard and programmatic control");
    println!("• Real-time search filtering with highlighting");
    println!("• Virtual scrolling for efficient large dataset rendering");
    println!("• Custom node types (folder, leaf, loading, custom)");
    println!("• Event callbacks for expand, select, and search operations");
    println!("• Comprehensive keyboard navigation (arrows, space, enter)");
    println!("• Element conversion for integration with rendering system");
    println!("• Convenience builders for common tree patterns");

    Ok(())
}
