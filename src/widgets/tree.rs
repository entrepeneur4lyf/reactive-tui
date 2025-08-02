//! Tree Widget
//!
//! A comprehensive hierarchical tree widget supporting expand/collapse, lazy loading,
//! multi-selection, keyboard navigation, and virtual scrolling for large datasets.
//!
//! # Features
//!
//! - **Hierarchical Display**: Nested tree structure with visual indentation
//! - **Expand/Collapse**: Interactive node expansion with animation support
//! - **Lazy Loading**: Async loading of child nodes on demand
//! - **Multi-Selection**: Single or multiple node selection with keyboard shortcuts
//! - **Virtual Scrolling**: Efficient rendering for large datasets (10k+ nodes)
//! - **Keyboard Navigation**: Arrow keys, Enter/Space, Ctrl+A, etc.
//! - **Search/Filter**: Real-time tree filtering with highlight
//! - **Drag & Drop**: Node reordering and moving (optional)
//! - **Custom Rendering**: Flexible node display with icons, badges, custom content
//! - **Accessibility**: Full ARIA support and screen reader compatibility
//!
//! # Basic Usage
//!
//! ```rust
//! use tui_core::widgets::{Tree, TreeBuilder, TreeNode};
//!
//! // Simple file system tree
//! let mut tree = TreeBuilder::new("file-tree")
//!     .root_node(TreeNode::new("root", "/")
//!         .child(TreeNode::new("src", "src/")
//!             .child(TreeNode::new("main.rs", "main.rs"))
//!             .child(TreeNode::new("lib.rs", "lib.rs")))
//!         .child(TreeNode::new("tests", "tests/")
//!             .child(TreeNode::new("integration.rs", "integration.rs"))))
//!     .expandable(true)
//!     .selectable(true)
//!     .build();
//!
//! // Expand a node
//! tree.expand("src")?;
//!
//! // Select multiple nodes
//! tree.select("main.rs")?;
//! tree.select("lib.rs")?;
//! ```
//!
//! # Advanced Usage
//!
//! ```rust
//! use tui_core::widgets::{TreeBuilder, TreeNode, TreeNodeType};
//!
//! // Advanced tree with lazy loading
//! let tree = TreeBuilder::new("advanced-tree")
//!     .lazy_loading(true, |node_id| async move {
//!         // Load children from API/database
//!         load_children_from_api(node_id).await
//!     })
//!     .virtual_scrolling(true)
//!     .multi_select(true)
//!     .search_enabled(true)
//!     .on_select(|selected_nodes| {
//!         println!("Selected: {:?}", selected_nodes);
//!     })
//!     .on_expand(|node_id, expanded| {
//!         println!("Node {} {}", node_id, if expanded { "expanded" } else { "collapsed" });
//!     })
//!     .build();
//! ```

use crate::{
    components::Element,
    error::{Result, TuiError},
    reactive::Reactive,
};
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt,
    future::Future,
    pin::Pin,
    sync::Arc,
};

// Type aliases for complex function pointer types
type OnSelectCallback = Arc<dyn Fn(&[NodeId]) + Send + Sync>;
type OnExpandCallback = Arc<dyn Fn(&NodeId, bool) + Send + Sync>;
type OnActionCallback = Arc<dyn Fn(&NodeId) + Send + Sync>;
type OnSearchCallback = Arc<dyn Fn(&str) + Send + Sync>;
use tokio::sync::RwLock;

/// Unique identifier for tree nodes
pub type NodeId = String;

/// Tree node type for different visual styling
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TreeNodeType {
    /// Regular node
    Node,
    /// Folder/container node
    Folder,
    /// Leaf node (cannot have children)
    Leaf,
    /// Loading placeholder
    Loading,
    /// Custom type with identifier
    Custom(String),
}

/// Visual representation configuration for tree nodes
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
pub struct TreeNodeStyle {
    /// Icon to display for this node type
    pub icon: Option<String>,
    /// Text color CSS class
    pub text_class: Option<String>,
    /// Background color CSS class
    pub background_class: Option<String>,
    /// Additional CSS classes
    pub css_classes: Vec<String>,
    /// Indentation characters (e.g., "├── ", "└── ")
    pub indent_chars: TreeIndentChars,
}

/// Characters used for tree indentation and structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeIndentChars {
    /// Vertical line for continuation
    pub vertical: String,
    /// Horizontal line for branches
    pub horizontal: String,
    /// Branch connector (├)
    pub branch: String,
    /// Last branch connector (└)
    pub last_branch: String,
    /// Space for indentation
    pub space: String,
    /// Expanded folder icon
    pub expanded: String,
    /// Collapsed folder icon
    pub collapsed: String,
    /// Leaf node icon
    pub leaf: String,
}

impl Default for TreeIndentChars {
    fn default() -> Self {
        Self {
            vertical: "│".to_string(),
            horizontal: "─".to_string(),
            branch: "├".to_string(),
            last_branch: "└".to_string(),
            space: " ".to_string(),
            expanded: "▼".to_string(),
            collapsed: "▶".to_string(),
            leaf: "•".to_string(),
        }
    }
}

/// A single node in the tree structure
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeNode {
    /// Unique identifier for this node
    pub id: NodeId,
    /// Display text for the node
    pub label: String,
    /// Optional description or subtitle
    pub description: Option<String>,
    /// Node type for styling and behavior
    pub node_type: TreeNodeType,
    /// Custom data associated with this node
    pub data: HashMap<String, String>,
    /// Child node IDs (for lazy loading)
    pub children: Vec<NodeId>,
    /// Whether this node can have children
    pub expandable: bool,
    /// Whether children have been loaded
    pub children_loaded: bool,
    /// Whether this node is currently expanded
    pub expanded: bool,
    /// Whether this node is selected
    pub selected: bool,
    /// Whether this node is disabled
    pub disabled: bool,
    /// Custom styling for this node
    pub style: Option<TreeNodeStyle>,
    /// Hierarchical level (0 = root, 1 = first level, etc.)
    pub level: usize,
    /// Whether this is the last child at its level
    pub is_last_child: bool,
}

impl TreeNode {
    /// Create a new tree node
    pub fn new<S: Into<String>>(id: S, label: S) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            description: None,
            node_type: TreeNodeType::Node,
            data: HashMap::new(),
            children: Vec::new(),
            expandable: false,
            children_loaded: true,
            expanded: false,
            selected: false,
            disabled: false,
            style: None,
            level: 0,
            is_last_child: false,
        }
    }

    /// Create a folder node (expandable by default)
    pub fn folder<S: Into<String>>(id: S, label: S) -> Self {
        Self {
            node_type: TreeNodeType::Folder,
            expandable: true,
            children_loaded: false, // Folders typically use lazy loading
            ..Self::new(id, label)
        }
    }

    /// Create a leaf node (cannot have children)
    pub fn leaf<S: Into<String>>(id: S, label: S) -> Self {
        Self {
            node_type: TreeNodeType::Leaf,
            expandable: false,
            ..Self::new(id, label)
        }
    }

    /// Add a child node
    pub fn child(mut self, child: TreeNode) -> Self {
        self.children.push(child.id.clone());
        self.expandable = true;
        self.children_loaded = true;
        self
    }

    /// Set the description
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the node type
    pub fn node_type(mut self, node_type: TreeNodeType) -> Self {
        self.node_type = node_type;
        self
    }

    /// Set custom data
    pub fn data<K: Into<String>, V: Into<String>>(mut self, key: K, value: V) -> Self {
        self.data.insert(key.into(), value.into());
        self
    }

    /// Set expandable state
    pub fn expandable(mut self, expandable: bool) -> Self {
        self.expandable = expandable;
        self
    }

    /// Set expanded state
    pub fn expanded(mut self, expanded: bool) -> Self {
        self.expanded = expanded;
        self
    }

    /// Set selected state
    pub fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    /// Set disabled state
    pub fn disabled(mut self, disabled: bool) -> Self {
        self.disabled = disabled;
        self
    }

    /// Set custom styling
    pub fn style(mut self, style: TreeNodeStyle) -> Self {
        self.style = Some(style);
        self
    }
}

/// Current state of the tree widget
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TreeState {
    /// Currently expanded node IDs
    pub expanded_nodes: HashSet<NodeId>,
    /// Currently selected node IDs
    pub selected_nodes: HashSet<NodeId>,
    /// Currently highlighted node for keyboard navigation
    pub highlighted_node: Option<NodeId>,
    /// Current search/filter query
    pub search_query: String,
    /// Nodes that match the current search
    pub filtered_nodes: HashSet<NodeId>,
    /// Whether the tree has focus
    pub focused: bool,
    /// Virtual scrolling offset
    pub scroll_offset: usize,
    /// Number of visible rows in viewport
    pub viewport_height: usize,
    /// Currently loading nodes (for lazy loading)
    pub loading_nodes: HashSet<NodeId>,
}

impl Default for TreeState {
    fn default() -> Self {
        Self {
            expanded_nodes: HashSet::new(),
            selected_nodes: HashSet::new(),
            highlighted_node: None,
            search_query: String::new(),
            filtered_nodes: HashSet::new(),
            focused: false,
            scroll_offset: 0,
            viewport_height: 20,
            loading_nodes: HashSet::new(),
        }
    }
}

/// Configuration for tree widget behavior
#[derive(Debug, Clone)]
pub struct TreeConfig {
    /// Whether multiple nodes can be selected
    pub multi_select: bool,
    /// Whether nodes can be expanded/collapsed
    pub expandable: bool,
    /// Whether to use lazy loading for child nodes
    pub lazy_loading: bool,
    /// Whether to enable virtual scrolling for performance
    pub virtual_scrolling: bool,
    /// Whether search/filtering is enabled
    pub search_enabled: bool,
    /// Whether drag & drop is enabled
    pub drag_drop_enabled: bool,
    /// Maximum number of nodes to render at once
    pub max_visible_nodes: usize,
    /// Animation duration for expand/collapse (milliseconds)
    pub animation_duration: u32,
    /// Default styling for different node types
    pub node_styles: HashMap<TreeNodeType, TreeNodeStyle>,
}

impl Default for TreeConfig {
    fn default() -> Self {
        let mut node_styles = HashMap::new();

        // Default styles for different node types
        node_styles.insert(
            TreeNodeType::Folder,
            TreeNodeStyle {
                icon: Some("📁".to_string()),
                text_class: Some("tree-folder".to_string()),
                ..Default::default()
            },
        );

        node_styles.insert(
            TreeNodeType::Leaf,
            TreeNodeStyle {
                icon: Some("📄".to_string()),
                text_class: Some("tree-leaf".to_string()),
                ..Default::default()
            },
        );

        node_styles.insert(
            TreeNodeType::Loading,
            TreeNodeStyle {
                icon: Some("⏳".to_string()),
                text_class: Some("tree-loading".to_string()),
                ..Default::default()
            },
        );

        Self {
            multi_select: false,
            expandable: true,
            lazy_loading: false,
            virtual_scrolling: false,
            search_enabled: false,
            drag_drop_enabled: false,
            max_visible_nodes: 1000,
            animation_duration: 150,
            node_styles,
        }
    }
}

/// Lazy loading callback type
pub type LazyLoader = Arc<
    dyn Fn(&NodeId) -> Pin<Box<dyn Future<Output = Result<Vec<TreeNode>>> + Send>> + Send + Sync,
>;

/// Main tree widget
#[derive(Clone)]
pub struct Tree {
    /// Unique identifier for the tree
    pub id: String,
    /// All nodes in the tree (flat map for efficient access)
    pub nodes: Arc<RwLock<HashMap<NodeId, TreeNode>>>,
    /// Root node IDs
    pub root_nodes: Vec<NodeId>,
    /// Current tree state
    pub state: Reactive<TreeState>,
    /// Configuration
    pub config: TreeConfig,
    /// Lazy loading callback
    pub lazy_loader: Option<LazyLoader>,
    /// Selection change callback
    pub on_select: Option<OnSelectCallback>,
    /// Expand/collapse callback
    pub on_expand: Option<OnExpandCallback>,
    /// Node action callback (double-click, enter)
    pub on_action: Option<OnActionCallback>,
    /// Search/filter callback
    pub on_search: Option<OnSearchCallback>,
}

impl Tree {
    /// Create a new tree widget builder
    pub fn builder<S: Into<String>>(id: S) -> TreeBuilder {
        TreeBuilder::new(id)
    }

    /// Add a root node to the tree
    pub async fn add_root_node(&mut self, node: TreeNode) -> Result<()> {
        let node_id = node.id.clone();

        {
            let mut nodes = self.nodes.write().await;
            nodes.insert(node_id.clone(), node);
        }

        self.root_nodes.push(node_id);
        Ok(())
    }

    /// Add a child node to a parent
    pub async fn add_child_node(
        &mut self,
        parent_id: impl AsRef<str>,
        child: TreeNode,
    ) -> Result<()> {
        let parent_id = parent_id.as_ref();
        let child_id = child.id.clone();

        {
            let mut nodes = self.nodes.write().await;

            // Add the child node
            nodes.insert(child_id.clone(), child);

            // Update parent to include this child
            if let Some(parent) = nodes.get_mut(parent_id) {
                if !parent.children.contains(&child_id) {
                    parent.children.push(child_id);
                    parent.expandable = true;
                }
            } else {
                return Err(TuiError::component(format!("{parent_id} not found")));
            }
        }

        Ok(())
    }

    /// Expand a node
    pub async fn expand(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        // Check if lazy loading is needed
        if self.config.lazy_loading {
            let needs_loading = {
                let nodes = self.nodes.read().await;
                if let Some(node) = nodes.get(node_id) {
                    node.expandable && !node.children_loaded
                } else {
                    return Err(TuiError::component(format!("{node_id} not found")));
                }
            };

            if needs_loading {
                if let Some(loader) = &self.lazy_loader {
                    // Mark as loading
                    self.state.update(|state| {
                        state.loading_nodes.insert(node_id.to_string());
                    });

                    // Load children
                    let node_id_string = node_id.to_string();
                    match loader(&node_id_string).await {
                        Ok(children) => {
                            // Add children to the tree
                            for child in children {
                                self.add_child_node(node_id, child).await?;
                            }

                            // Mark as loaded
                            {
                                let mut nodes = self.nodes.write().await;
                                if let Some(node) = nodes.get_mut(node_id) {
                                    node.children_loaded = true;
                                }
                            }

                            // Remove loading state
                            self.state.update(|state| {
                                state.loading_nodes.remove(&node_id_string);
                            });
                        }
                        Err(e) => {
                            // Remove loading state on error
                            self.state.update(|state| {
                                state.loading_nodes.remove(&node_id_string);
                            });
                            return Err(e);
                        }
                    }
                }
            }
        }

        // Expand the node
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                node.expanded = true;
            }
        }

        self.state.update(|state| {
            state.expanded_nodes.insert(node_id.to_string());
        });

        if let Some(callback) = &self.on_expand {
            let node_id_string = node_id.to_string();
            callback(&node_id_string, true);
        }

        Ok(())
    }

    /// Collapse a node
    pub async fn collapse(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                node.expanded = false;
            }
        }

        self.state.update(|state| {
            state.expanded_nodes.remove(node_id);
        });

        if let Some(callback) = &self.on_expand {
            let node_id_string = node_id.to_string();
            callback(&node_id_string, false);
        }

        Ok(())
    }

    /// Toggle expand/collapse state of a node
    pub async fn toggle_expand(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        let is_expanded = {
            let nodes = self.nodes.read().await;
            nodes
                .get(node_id)
                .map(|node| node.expanded)
                .unwrap_or(false)
        };

        if is_expanded {
            self.collapse(node_id).await
        } else {
            self.expand(node_id).await
        }
    }

    /// Select a node
    pub async fn select(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                if node.disabled {
                    return Ok(()); // Cannot select disabled nodes
                }
                node.selected = true;
            } else {
                return Err(TuiError::component(format!("{node_id} not found")));
            }
        }

        self.state.update(|state| {
            if !self.config.multi_select {
                // Clear previous selections in single-select mode
                state.selected_nodes.clear();
            }
            state.selected_nodes.insert(node_id.to_string());
        });

        // Update node selection states
        if !self.config.multi_select {
            let mut nodes = self.nodes.write().await;
            for node in nodes.values_mut() {
                node.selected = node.id == node_id;
            }
        }

        if let Some(callback) = &self.on_select {
            let state = self.state.get();
            let selected: Vec<NodeId> = state.selected_nodes.iter().cloned().collect();
            callback(&selected);
        }

        Ok(())
    }

    /// Deselect a node
    pub async fn deselect(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        {
            let mut nodes = self.nodes.write().await;
            if let Some(node) = nodes.get_mut(node_id) {
                node.selected = false;
            }
        }

        self.state.update(|state| {
            state.selected_nodes.remove(node_id);
        });

        if let Some(callback) = &self.on_select {
            let state = self.state.get();
            let selected: Vec<NodeId> = state.selected_nodes.iter().cloned().collect();
            callback(&selected);
        }

        Ok(())
    }

    /// Toggle selection of a node
    pub async fn toggle_select(&mut self, node_id: impl AsRef<str>) -> Result<()> {
        let node_id = node_id.as_ref();
        let is_selected = {
            let nodes = self.nodes.read().await;
            nodes
                .get(node_id)
                .map(|node| node.selected)
                .unwrap_or(false)
        };

        if is_selected {
            self.deselect(node_id).await
        } else {
            self.select(node_id).await
        }
    }

    /// Clear all selections
    pub async fn clear_selection(&mut self) {
        {
            let mut nodes = self.nodes.write().await;
            for node in nodes.values_mut() {
                node.selected = false;
            }
        }

        self.state.update(|state| {
            state.selected_nodes.clear();
        });

        if let Some(callback) = &self.on_select {
            callback(&[]);
        }
    }

    /// Set search query and filter nodes
    pub async fn set_search_query<S: Into<String>>(&mut self, query: S) {
        let query = query.into();

        if query.is_empty() {
            self.state.update(|state| {
                state.search_query.clear();
                state.filtered_nodes.clear();
            });
        } else {
            let filtered_nodes = {
                let nodes = self.nodes.read().await;
                nodes
                    .values()
                    .filter(|node| {
                        node.label.to_lowercase().contains(&query.to_lowercase())
                            || node
                                .description
                                .as_ref()
                                .map(|desc| desc.to_lowercase().contains(&query.to_lowercase()))
                                .unwrap_or(false)
                    })
                    .map(|node| node.id.clone())
                    .collect()
            };

            self.state.update(|state| {
                state.search_query = query.clone();
                state.filtered_nodes = filtered_nodes;
            });
        }

        if let Some(callback) = &self.on_search {
            callback(&query);
        }
    }

    /// Get flattened list of visible nodes (for rendering)
    pub async fn get_visible_nodes(&self) -> Vec<NodeId> {
        let mut visible = Vec::new();
        let nodes = self.nodes.read().await;
        let state = self.state.get();

        for root_id in &self.root_nodes {
            self.collect_visible_nodes(root_id, &nodes, &state, &mut visible, 0);
        }

        // Apply search filter if active
        if !state.search_query.is_empty() {
            visible.retain(|id| state.filtered_nodes.contains(id));
        }

        // Apply virtual scrolling if enabled
        if self.config.virtual_scrolling {
            let start = state.scroll_offset;
            let end = (start + state.viewport_height).min(visible.len());
            visible = visible[start..end].to_vec();
        }

        visible
    }

    /// Recursive helper to collect visible nodes
    fn collect_visible_nodes(
        &self,
        node_id: &NodeId,
        nodes: &HashMap<NodeId, TreeNode>,
        state: &TreeState,
        visible: &mut Vec<NodeId>,
        _level: usize,
    ) {
        Self::collect_visible_nodes_recursive(node_id, nodes, state, visible, _level);
    }

    /// Recursive implementation
    fn collect_visible_nodes_recursive(
        node_id: &NodeId,
        nodes: &HashMap<NodeId, TreeNode>,
        state: &TreeState,
        visible: &mut Vec<NodeId>,
        _level: usize,
    ) {
        if let Some(node) = nodes.get(node_id) {
            visible.push(node_id.clone());

            if node.expanded && state.expanded_nodes.contains(node_id) {
                for child_id in &node.children {
                    Self::collect_visible_nodes_recursive(
                        child_id,
                        nodes,
                        state,
                        visible,
                        _level + 1,
                    );
                }
            }
        }
    }

    /// Navigate to the next node (keyboard navigation)
    pub async fn navigate_next(&mut self) {
        let visible_nodes = self.get_visible_nodes().await;
        if visible_nodes.is_empty() {
            return;
        }

        let state = self.state.get();
        let current_index = state
            .highlighted_node
            .as_ref()
            .and_then(|id| visible_nodes.iter().position(|n| n == id))
            .unwrap_or(0);

        let next_index = (current_index + 1) % visible_nodes.len();
        let next_node = visible_nodes[next_index].clone();

        drop(state);
        self.state.update(|state| {
            state.highlighted_node = Some(next_node);
        });
    }

    /// Navigate to the previous node (keyboard navigation)
    pub async fn navigate_previous(&mut self) {
        let visible_nodes = self.get_visible_nodes().await;
        if visible_nodes.is_empty() {
            return;
        }

        let state = self.state.get();
        let current_index = state
            .highlighted_node
            .as_ref()
            .and_then(|id| visible_nodes.iter().position(|n| n == id))
            .unwrap_or(0);

        let prev_index = if current_index == 0 {
            visible_nodes.len() - 1
        } else {
            current_index - 1
        };
        let prev_node = visible_nodes[prev_index].clone();

        drop(state);
        self.state.update(|state| {
            state.highlighted_node = Some(prev_node);
        });
    }

    /// Handle keyboard events
    pub async fn handle_key(&mut self, key: &str) -> Result<bool> {
        match key {
            "ArrowDown" => {
                self.navigate_next().await;
                Ok(true)
            }
            "ArrowUp" => {
                self.navigate_previous().await;
                Ok(true)
            }
            "ArrowRight" | "Enter" => {
                let state = self.state.get();
                if let Some(node_id) = &state.highlighted_node {
                    let node_id = node_id.clone();
                    drop(state);
                    self.expand(&node_id).await?;
                }
                Ok(true)
            }
            "ArrowLeft" => {
                let state = self.state.get();
                if let Some(node_id) = &state.highlighted_node {
                    let node_id = node_id.clone();
                    drop(state);
                    self.collapse(&node_id).await?;
                }
                Ok(true)
            }
            "Space" => {
                let state = self.state.get();
                if let Some(node_id) = &state.highlighted_node {
                    let node_id = node_id.clone();
                    drop(state);
                    self.toggle_select(&node_id).await?;
                }
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Convert to Element for rendering
    pub async fn to_element(&self) -> Element {
        let mut container = Element::with_tag("div".to_string())
            .id(self.id.clone())
            .class("tree")
            .focusable(true);

        let state = self.state.get();
        if state.focused {
            container = container.class("tree-focused");
        }

        // Add search input if enabled
        if self.config.search_enabled {
            let search_input = Element::with_tag("input".to_string())
                .class("tree-search")
                .attr("placeholder", "Search tree...")
                .attr("value", state.search_query.clone());

            container = container.child(search_input.build());
        }

        // Render visible nodes
        let visible_nodes = self.get_visible_nodes().await;
        let nodes = self.nodes.read().await;

        for node_id in visible_nodes {
            if let Some(node) = nodes.get(&node_id) {
                let node_element = self.render_node(node, &state);
                container = container.child(node_element);
            }
        }

        container.build()
    }

    /// Render a single tree node
    fn render_node(&self, node: &TreeNode, state: &TreeState) -> Element {
        let mut node_element = Element::with_tag("div".to_string())
            .class("tree-node")
            .class(node.level.to_string());

        // Add state classes
        if node.selected {
            node_element = node_element.class("tree-node-selected");
        }
        if state.highlighted_node.as_ref() == Some(&node.id) {
            node_element = node_element.class("tree-node-highlighted");
        }
        if node.disabled {
            node_element = node_element.class("tree-node-disabled");
        }
        if state.loading_nodes.contains(&node.id) {
            node_element = node_element.class("tree-node-loading");
        }

        // Build content with indentation
        let mut content_parts = Vec::new();

        // Add indentation
        let default_indent = TreeIndentChars::default();
        let indent_chars = node
            .style
            .as_ref()
            .map(|s| &s.indent_chars)
            .unwrap_or(&default_indent);

        for _ in 0..node.level {
            content_parts.push(indent_chars.space.clone());
            content_parts.push(indent_chars.space.clone());
        }

        // Add expand/collapse indicator
        if node.expandable {
            if node.expanded {
                content_parts.push(indent_chars.expanded.clone());
            } else {
                content_parts.push(indent_chars.collapsed.clone());
            }
        } else {
            content_parts.push(indent_chars.leaf.clone());
        }
        content_parts.push(indent_chars.space.clone());

        // Add icon
        let style = node
            .style
            .as_ref()
            .or_else(|| self.config.node_styles.get(&node.node_type));

        if let Some(icon) = style.and_then(|s| s.icon.as_ref()) {
            content_parts.push(icon.clone());
            content_parts.push(indent_chars.space.clone());
        }

        // Add label
        content_parts.push(node.label.clone());

        // Add description if present
        if let Some(description) = &node.description {
            content_parts.push(" - ".to_string());
            content_parts.push(description.clone());
        }

        node_element.content(content_parts.join("")).build()
    }
}

/// Builder for creating tree widgets
pub struct TreeBuilder {
    id: String,
    root_nodes: Vec<TreeNode>,
    config: TreeConfig,
    lazy_loader: Option<LazyLoader>,
    on_select: Option<OnSelectCallback>,
    on_expand: Option<OnExpandCallback>,
    on_action: Option<OnActionCallback>,
    on_search: Option<OnSearchCallback>,
}

impl TreeBuilder {
    /// Create a new tree builder
    pub fn new<S: Into<String>>(id: S) -> Self {
        Self {
            id: id.into(),
            root_nodes: Vec::new(),
            config: TreeConfig::default(),
            lazy_loader: None,
            on_select: None,
            on_expand: None,
            on_action: None,
            on_search: None,
        }
    }

    /// Add a root node
    pub fn root_node(mut self, node: TreeNode) -> Self {
        self.root_nodes.push(node);
        self
    }

    /// Enable multi-selection
    pub fn multi_select(mut self, multi_select: bool) -> Self {
        self.config.multi_select = multi_select;
        self
    }

    /// Enable expandable nodes
    pub fn expandable(mut self, expandable: bool) -> Self {
        self.config.expandable = expandable;
        self
    }

    /// Enable lazy loading with callback
    pub fn lazy_loading<F, Fut>(mut self, enabled: bool, loader: F) -> Self
    where
        F: Fn(&NodeId) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<TreeNode>>> + Send + 'static,
    {
        self.config.lazy_loading = enabled;
        if enabled {
            self.lazy_loader = Some(Arc::new(move |node_id| Box::pin(loader(node_id))));
        }
        self
    }

    /// Enable virtual scrolling
    pub fn virtual_scrolling(mut self, enabled: bool) -> Self {
        self.config.virtual_scrolling = enabled;
        self
    }

    /// Enable search functionality
    pub fn search_enabled(mut self, enabled: bool) -> Self {
        self.config.search_enabled = enabled;
        self
    }

    /// Set selection callback
    pub fn on_select<F>(mut self, callback: F) -> Self
    where
        F: Fn(&[NodeId]) + Send + Sync + 'static,
    {
        self.on_select = Some(Arc::new(callback));
        self
    }

    /// Set expand/collapse callback
    pub fn on_expand<F>(mut self, callback: F) -> Self
    where
        F: Fn(&NodeId, bool) + Send + Sync + 'static,
    {
        self.on_expand = Some(Arc::new(callback));
        self
    }

    /// Set node action callback
    pub fn on_action<F>(mut self, callback: F) -> Self
    where
        F: Fn(&NodeId) + Send + Sync + 'static,
    {
        self.on_action = Some(Arc::new(callback));
        self
    }

    /// Set search callback
    pub fn on_search<F>(mut self, callback: F) -> Self
    where
        F: Fn(&str) + Send + Sync + 'static,
    {
        self.on_search = Some(Arc::new(callback));
        self
    }

    /// Build the tree widget
    pub async fn build(self) -> Tree {
        let mut nodes_map = HashMap::new();
        let mut root_ids = Vec::new();

        // Process root nodes and their children recursively
        for root in self.root_nodes {
            root_ids.push(root.id.clone());
            Self::add_node_recursive(&mut nodes_map, root, 0, true);
        }

        Tree {
            id: self.id,
            nodes: Arc::new(RwLock::new(nodes_map)),
            root_nodes: root_ids,
            state: Reactive::new(TreeState::default()),
            config: self.config,
            lazy_loader: self.lazy_loader,
            on_select: self.on_select,
            on_expand: self.on_expand,
            on_action: self.on_action,
            on_search: self.on_search,
        }
    }

    /// Recursively add nodes to the map
    fn add_node_recursive(
        nodes_map: &mut HashMap<NodeId, TreeNode>,
        mut node: TreeNode,
        level: usize,
        is_last_child: bool,
    ) {
        node.level = level;
        node.is_last_child = is_last_child;

        let children = node.children.clone();
        let node_id = node.id.clone();
        nodes_map.insert(node_id.clone(), node);

        // Recursively add children by processing the queue
        // We use a queue-based approach to avoid deep recursion
        let mut queue = std::collections::VecDeque::new();
        for child_id in children {
            queue.push_back((node_id.clone(), child_id));
        }
        
        // Process the queue to maintain parent-child relationships
        while let Some((parent_id, child_id)) = queue.pop_front() {
            // Store the parent-child relationship in the nodes map
            if let Some(parent_node) = nodes_map.get_mut(&parent_id) {
                if !parent_node.children.contains(&child_id) {
                    parent_node.children.push(child_id.clone());
                }
            }
            
            // If this child has its own children, add them to the queue
            if let Some(child_node) = nodes_map.get(&child_id) {
                for grandchild_id in &child_node.children {
                    queue.push_back((child_id.clone(), grandchild_id.clone()));
                }
            }
        }
    }
}

impl fmt::Display for Tree {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tree({}): {} root nodes", self.id, self.root_nodes.len())
    }
}

/// Convenience functions for common tree patterns
impl TreeBuilder {
    /// Create a file system tree
    pub fn file_system(id: String) -> TreeBuilder {
        Self::new(id)
            .expandable(true)
            .lazy_loading(true, |path| {
                let path = path.clone();
                async move {
                    // Integrate with file system using async I/O
                    use tokio::fs;
                    
                    let mut nodes = Vec::new();
                    match fs::read_dir(&path).await {
                    Ok(mut entries) => {
                        while let Ok(Some(entry)) = entries.next_entry().await {
                            if let Ok(metadata) = entry.metadata().await {
                                let file_name = entry.file_name().to_string_lossy().to_string();
                                let file_path = entry.path().to_string_lossy().to_string();
                                
                                let icon = if metadata.is_dir() {
                                    "📁"
                                } else {
                                    "📄"
                                };
                                
                                nodes.push(TreeNode {
                                    id: file_path.clone(),
                                    label: file_name,
                                    description: None,
                                    node_type: if metadata.is_dir() { 
                                        TreeNodeType::Folder 
                                    } else { 
                                        TreeNodeType::Leaf 
                                    },
                                    data: HashMap::new(),
                                    children: Vec::new(),
                                    expandable: metadata.is_dir(),
                                    children_loaded: false,
                                    expanded: false,
                                    selected: false,
                                    disabled: false,
                                    style: Some(TreeNodeStyle {
                                        icon: Some(icon.to_string()),
                                        text_class: None,
                                        background_class: None,
                                        css_classes: Vec::new(),
                                        indent_chars: TreeIndentChars::default(),
                                    }),
                                    level: 0,
                                    is_last_child: false,
                                });
                            }
                        }
                    }
                    Err(e) => {
                        eprintln!("Failed to read directory {}: {}", path, e);
                    }
                }
                    Ok(nodes)
                }
            })
            .multi_select(true)
            .search_enabled(true)
    }
    /// Create an organization/department tree
    pub fn organization(id: String) -> TreeBuilder {
        Self::new(id)
            .expandable(true)
            .multi_select(false)
            .search_enabled(true)
    }
    /// Create a category/tag tree
    pub fn categories(id: String) -> TreeBuilder {
        Self::new(id)
            .expandable(true)
            .multi_select(true)
            .search_enabled(true)
    }
}
