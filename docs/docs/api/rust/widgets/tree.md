# Tree Widget

Hierarchical tree component with lazy loading, drag-and-drop, selection modes, and virtual scrolling for large datasets.

## Overview

The Tree widget provides hierarchical data display with support for expand/collapse, multiple selection modes, drag-and-drop reordering, and efficient rendering of large tree structures.

```rust
use reactive_tui::widgets::{Tree, TreeBuilder, TreeNode, SelectionMode};

let file_tree = Tree::builder("file_explorer")
    .root_nodes(create_file_tree_nodes())
    .selection_mode(SelectionMode::Single)
    .show_lines(true)
    .show_icons(true)
    .lazy_loading(true)
    .build();
```

## TreeBuilder

```rust
impl TreeBuilder {
    pub fn new(id: &str) -> Self
    pub fn root_nodes(mut self, nodes: Vec<TreeNode>) -> Self
    pub fn selection_mode(mut self, mode: SelectionMode) -> Self
    pub fn show_lines(mut self, show: bool) -> Self
    pub fn show_icons(mut self, show: bool) -> Self
    pub fn show_checkboxes(mut self, show: bool) -> Self
    pub fn lazy_loading(mut self, enabled: bool) -> Self
    pub fn virtual_scrolling(mut self, enabled: bool) -> Self
    pub fn draggable(mut self, draggable: bool) -> Self
    pub fn max_height(mut self, height: u16) -> Self
    pub fn indent_size(mut self, size: u16) -> Self
    pub fn on_select<F>(mut self, callback: F) -> Self
    pub fn on_expand<F>(mut self, callback: F) -> Self
    pub fn on_node_load<F>(mut self, callback: F) -> Self
    pub fn build(self) -> Tree
}
```

## TreeNode

```rust
pub struct TreeNode {
    pub id: String,
    pub label: String,
    pub children: Vec<TreeNode>,
    pub expanded: bool,
    pub selected: bool,
    pub disabled: bool,
    pub icon: Option<String>,
    pub icon_expanded: Option<String>,
    pub tooltip: Option<String>,
    pub data: Option<serde_json::Value>,
    pub lazy: bool,
    pub leaf: bool,
}

impl TreeNode {
    pub fn new(id: &str, label: &str) -> Self
    pub fn children(mut self, children: Vec<TreeNode>) -> Self
    pub fn expanded(mut self, expanded: bool) -> Self
    pub fn selected(mut self, selected: bool) -> Self
    pub fn disabled(mut self, disabled: bool) -> Self
    pub fn icon(mut self, icon: &str) -> Self
    pub fn icon_expanded(mut self, icon_expanded: &str) -> Self
    pub fn tooltip(mut self, tooltip: &str) -> Self
    pub fn data(mut self, data: serde_json::Value) -> Self
    pub fn lazy(mut self, lazy: bool) -> Self
    pub fn leaf(mut self, leaf: bool) -> Self
}
```

## Selection Modes

```rust
pub enum SelectionMode {
    None,       // No selection
    Single,     // Single node selection
    Multiple,   // Multiple node selection
    Checkbox,   // Checkbox-based selection
}
```

## Examples

### File Explorer Tree

```rust
use reactive_tui::widgets::{Tree, TreeNode};

fn create_file_tree() -> Tree {
    Tree::builder("file_explorer")
        .root_nodes(vec![
            TreeNode::new("src", "src")
                .icon("📁")
                .icon_expanded("📂")
                .children(vec![
                    TreeNode::new("main.rs", "main.rs")
                        .icon("🦀")
                        .leaf(true),
                    TreeNode::new("lib.rs", "lib.rs")
                        .icon("🦀")
                        .leaf(true),
                    TreeNode::new("widgets", "widgets")
                        .icon("📁")
                        .icon_expanded("📂")
                        .lazy(true), // Load children on demand
                ]),
            TreeNode::new("tests", "tests")
                .icon("📁")
                .icon_expanded("📂")
                .children(vec![
                    TreeNode::new("integration.rs", "integration.rs")
                        .icon("🧪")
                        .leaf(true),
                ]),
            TreeNode::new("Cargo.toml", "Cargo.toml")
                .icon("📄")
                .leaf(true),
            TreeNode::new("README.md", "README.md")
                .icon("📖")
                .leaf(true),
        ])
        .selection_mode(SelectionMode::Single)
        .show_lines(true)
        .show_icons(true)
        .on_select(|node_id| {
            println!("Selected: {}", node_id);
            open_file(node_id);
            Ok(())
        })
        .on_expand(|node_id| {
            println!("Expanded: {}", node_id);
            load_directory_contents(node_id);
            Ok(())
        })
        .build()
}
```

### Organizational Hierarchy

```rust
let org_tree = Tree::builder("organization")
    .root_nodes(vec![
        TreeNode::new("ceo", "CEO - Alice Johnson")
            .icon("👑")
            .expanded(true)
            .children(vec![
                TreeNode::new("cto", "CTO - Bob Smith")
                    .icon("💻")
                    .children(vec![
                        TreeNode::new("dev_lead", "Dev Lead - Charlie Brown")
                            .icon("👨‍💻")
                            .children(vec![
                                TreeNode::new("dev1", "Developer - Diana Prince").icon("👩‍💻").leaf(true),
                                TreeNode::new("dev2", "Developer - Eve Adams").icon("👩‍💻").leaf(true),
                            ]),
                        TreeNode::new("qa_lead", "QA Lead - Frank Miller")
                            .icon("🧪")
                            .children(vec![
                                TreeNode::new("qa1", "QA Engineer - Grace Lee").icon("👩‍🔬").leaf(true),
                            ]),
                    ]),
                TreeNode::new("cfo", "CFO - Henry Davis")
                    .icon("💰")
                    .leaf(true),
            ]),
    ])
    .selection_mode(SelectionMode::Single)
    .show_lines(true)
    .show_icons(true)
    .on_select(|node_id| {
        show_employee_details(node_id);
        Ok(())
    })
    .build();
```

### Lazy Loading Tree

```rust
use reactive_tui::{widgets::Tree, reactive::Reactive};

let lazy_tree = Tree::builder("lazy_tree")
    .root_nodes(vec![
        TreeNode::new("root1", "Large Dataset 1")
            .icon("📊")
            .lazy(true),
        TreeNode::new("root2", "Large Dataset 2")
            .icon("📊")
            .lazy(true),
    ])
    .lazy_loading(true)
    .virtual_scrolling(true)
    .on_node_load(|node_id| {
        // Load children asynchronously
        tokio::spawn(async move {
            let children = fetch_tree_children(node_id).await?;
            update_tree_node(node_id, children);
        });
        Ok(())
    })
    .build();

async fn fetch_tree_children(node_id: &str) -> Result<Vec<TreeNode>> {
    // Simulate API call
    let children = database.fetch_children(node_id).await?;
    
    children.into_iter()
        .map(|child| {
            TreeNode::new(&child.id, &child.name)
                .icon(&child.icon)
                .lazy(!child.is_leaf)
                .leaf(child.is_leaf)
        })
        .collect()
}
```

### Checkbox Tree for Permissions

```rust
let permissions_tree = Tree::builder("permissions")
    .root_nodes(vec![
        TreeNode::new("admin", "Administrative")
            .icon("⚙️")
            .expanded(true)
            .children(vec![
                TreeNode::new("user_mgmt", "User Management")
                    .icon("👥")
                    .children(vec![
                        TreeNode::new("create_user", "Create Users").leaf(true),
                        TreeNode::new("delete_user", "Delete Users").leaf(true),
                        TreeNode::new("modify_user", "Modify Users").leaf(true),
                    ]),
                TreeNode::new("system", "System Settings")
                    .icon("🔧")
                    .children(vec![
                        TreeNode::new("backup", "Backup System").leaf(true),
                        TreeNode::new("restore", "Restore System").leaf(true),
                    ]),
            ]),
        TreeNode::new("content", "Content Management")
            .icon("📝")
            .children(vec![
                TreeNode::new("create_content", "Create Content").leaf(true),
                TreeNode::new("edit_content", "Edit Content").leaf(true),
                TreeNode::new("delete_content", "Delete Content").leaf(true),
                TreeNode::new("publish_content", "Publish Content").leaf(true),
            ]),
    ])
    .selection_mode(SelectionMode::Checkbox)
    .show_checkboxes(true)
    .show_lines(true)
    .on_select(|selected_nodes| {
        println!("Selected permissions: {:?}", selected_nodes);
        update_user_permissions(selected_nodes);
        Ok(())
    })
    .build();
```

### Menu Tree

```rust
let menu_tree = Tree::builder("application_menu")
    .root_nodes(vec![
        TreeNode::new("file", "File")
            .icon("📁")
            .children(vec![
                TreeNode::new("new", "New").icon("📄").leaf(true),
                TreeNode::new("open", "Open").icon("📂").leaf(true),
                TreeNode::new("save", "Save").icon("💾").leaf(true),
                TreeNode::new("recent", "Recent Files")
                    .icon("🕒")
                    .lazy(true), // Load recent files on expand
            ]),
        TreeNode::new("edit", "Edit")
            .icon("✏️")
            .children(vec![
                TreeNode::new("undo", "Undo").icon("↶").leaf(true),
                TreeNode::new("redo", "Redo").icon("↷").leaf(true),
                TreeNode::new("cut", "Cut").icon("✂️").leaf(true),
                TreeNode::new("copy", "Copy").icon("📋").leaf(true),
                TreeNode::new("paste", "Paste").icon("📋").leaf(true),
            ]),
        TreeNode::new("view", "View")
            .icon("👁️")
            .children(vec![
                TreeNode::new("zoom_in", "Zoom In").icon("🔍").leaf(true),
                TreeNode::new("zoom_out", "Zoom Out").icon("🔍").leaf(true),
                TreeNode::new("fullscreen", "Full Screen").icon("🖥️").leaf(true),
            ]),
    ])
    .selection_mode(SelectionMode::Single)
    .show_icons(true)
    .on_select(|node_id| {
        execute_menu_command(node_id);
        Ok(())
    })
    .build();
```

### Draggable Tree for Reordering

```rust
let draggable_tree = Tree::builder("task_tree")
    .root_nodes(create_task_nodes())
    .selection_mode(SelectionMode::Single)
    .draggable(true)
    .show_lines(true)
    .on_drag_drop(|source_node, target_node, position| {
        // Handle drag and drop reordering
        match position {
            DropPosition::Before => {
                move_node_before(source_node, target_node);
            },
            DropPosition::After => {
                move_node_after(source_node, target_node);
            },
            DropPosition::Inside => {
                move_node_inside(source_node, target_node);
            },
        }
        
        update_task_hierarchy();
        Ok(())
    })
    .build();
```

## State Management

```rust
use reactive_tui::{widgets::Tree, reactive::Reactive};

struct TreeState {
    expanded_nodes: Vec<String>,
    selected_nodes: Vec<String>,
    loaded_nodes: Vec<String>,
}

let tree_state = Reactive::new(TreeState {
    expanded_nodes: vec!["root".to_string()],
    selected_nodes: vec![],
    loaded_nodes: vec![],
});

let stateful_tree = Tree::builder("stateful_tree")
    .root_nodes(get_initial_nodes())
    .on_expand({
        let state = tree_state.clone();
        move |node_id| {
            let mut current_state = state.get();
            if !current_state.expanded_nodes.contains(&node_id.to_string()) {
                current_state.expanded_nodes.push(node_id.to_string());
            }
            state.set(current_state);
            Ok(())
        }
    })
    .on_collapse({
        let state = tree_state.clone();
        move |node_id| {
            let mut current_state = state.get();
            current_state.expanded_nodes.retain(|id| id != node_id);
            state.set(current_state);
            Ok(())
        }
    })
    .on_select({
        let state = tree_state.clone();
        move |selected| {
            let mut current_state = state.get();
            current_state.selected_nodes = selected.iter().map(|s| s.to_string()).collect();
            state.set(current_state);
            Ok(())
        }
    })
    .build();
```

## CSS Styling

```css
.tree {
    font-family: monospace;
    font-size: 14px;
    line-height: 1.4;
}

.tree-node {
    display: flex;
    align-items: center;
    padding: 2px 0;
    cursor: pointer;
    transition: background-color 0.1s ease;
}

.tree-node:hover {
    background-color: #f3f4f6;
}

.tree-node.selected {
    background-color: #dbeafe;
    color: #1d4ed8;
}

.tree-node.disabled {
    opacity: 0.5;
    cursor: not-allowed;
}

.tree-indent {
    width: 20px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.tree-line {
    border-left: 1px solid #d1d5db;
    height: 100%;
}

.tree-line-horizontal {
    border-bottom: 1px solid #d1d5db;
    width: 10px;
}

.tree-expander {
    width: 16px;
    height: 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    cursor: pointer;
    border-radius: 2px;
    transition: background-color 0.1s ease;
    margin-right: 4px;
}

.tree-expander:hover {
    background-color: #e5e7eb;
}

.tree-expander-icon {
    font-size: 10px;
    transition: transform 0.1s ease;
}

.tree-expander-icon.expanded {
    transform: rotate(90deg);
}

.tree-checkbox {
    margin-right: 8px;
}

.tree-icon {
    margin-right: 8px;
    font-size: 16px;
    width: 16px;
    text-align: center;
}

.tree-label {
    flex: 1;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
}

.tree-loading {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 0;
    color: #6b7280;
    font-style: italic;
}

.tree-loading-spinner {
    width: 12px;
    height: 12px;
    border: 2px solid #e5e7eb;
    border-top: 2px solid #3b82f6;
    border-radius: 50%;
    animation: tree-spin 1s linear infinite;
}

@keyframes tree-spin {
    0% { transform: rotate(0deg); }
    100% { transform: rotate(360deg); }
}

.tree-empty {
    padding: 20px;
    text-align: center;
    color: #9ca3af;
    font-style: italic;
}

.tree-drag-preview {
    opacity: 0.7;
    background-color: #f3f4f6;
    border: 1px dashed #3b82f6;
    border-radius: 4px;
    padding: 4px 8px;
}

.tree-drop-indicator {
    height: 2px;
    background-color: #3b82f6;
    border-radius: 1px;
    animation: tree-pulse 1s ease-in-out infinite;
}

@keyframes tree-pulse {
    0%, 100% { opacity: 0.5; }
    50% { opacity: 1; }
}
```

## Keyboard Navigation

```rust
// Built-in keyboard shortcuts
// Arrow Up/Down: Navigate nodes
// Arrow Right: Expand node
// Arrow Left: Collapse node
// Space: Toggle selection/checkbox
// Enter: Select node
// Home: Go to first node
// End: Go to last node

let keyboard_tree = Tree::builder("keyboard_tree")
    .keyboard_navigation(true)
    .focus_visible(true)
    .shortcuts(vec![
        TreeShortcut::new("Ctrl+A", "select_all"),
        TreeShortcut::new("Delete", "delete_selected"),
        TreeShortcut::new("F2", "rename_node"),
    ])
    .build();
```

## Integration Examples

### Project Explorer

```rust
use reactive_tui::widgets::{Tree, Splitter, TextEditor};

let project_explorer = Splitter::builder("project_layout")
    .orientation(SplitterOrientation::Horizontal)
    .left_pane(
        Tree::builder("project_tree")
            .root_nodes(load_project_structure())
            .selection_mode(SelectionMode::Single)
            .show_icons(true)
            .on_select(|file_path| {
                load_file_in_editor(file_path);
                Ok(())
            })
            .build()
    )
    .right_pane(
        TextEditor::builder("code_editor")
            .syntax_highlighting(true)
            .line_numbers(true)
            .build()
    )
    .split_ratio(0.3)
    .build();
```

### Database Schema Tree

```rust
let schema_tree = Tree::builder("database_schema")
    .root_nodes(vec![
        TreeNode::new("tables", "Tables")
            .icon("🗃️")
            .lazy(true),
        TreeNode::new("views", "Views")
            .icon("👁️")
            .lazy(true),
        TreeNode::new("procedures", "Stored Procedures")
            .icon("⚙️")
            .lazy(true),
        TreeNode::new("functions", "Functions")
            .icon("🔧")
            .lazy(true),
    ])
    .lazy_loading(true)
    .on_node_load(|node_id| {
        load_database_objects(node_id);
        Ok(())
    })
    .on_select(|node_id| {
        show_object_details(node_id);
        Ok(())
    })
    .build();
```

The Tree widget provides comprehensive hierarchical data display with lazy loading, selection modes, drag-and-drop, and extensive customization options for terminal applications.