//! Focus management system for cursor navigation

use crate::components::Element;

#[derive(Debug, Clone)]
pub struct FocusableElement {
    pub id: String,
    pub tab_index: i32,
    pub element_path: Vec<usize>, // Path to element in the tree
}

#[derive(Debug)]
pub struct FocusManager {
    focusable_elements: Vec<FocusableElement>,
    current_focus_index: Option<usize>,
    focus_order: Vec<usize>, // Ordered by tab_index
}

impl FocusManager {
    pub fn new() -> Self {
        Self {
            focusable_elements: Vec::new(),
            current_focus_index: None,
            focus_order: Vec::new(),
        }
    }

    /// Build focus list from element tree
    pub fn build_focus_list(&mut self, root: &Element) {
        self.focusable_elements.clear();
        self.focus_order.clear();
        self.current_focus_index = None;

        self.collect_focusable_elements(root, &mut Vec::new());
        self.sort_focus_order();

        // Set initial focus to first element
        if !self.focusable_elements.is_empty() {
            self.current_focus_index = Some(0);
        }
    }

    fn collect_focusable_elements(&mut self, element: &Element, path: &mut Vec<usize>) {
        if element.focusable {
            let id = element
                .id
                .clone()
                .unwrap_or_else(|| format!("element_{}", self.focusable_elements.len()));

            let tab_index = element.tab_index.unwrap_or(0);

            self.focusable_elements.push(FocusableElement {
                id,
                tab_index,
                element_path: path.clone(),
            });
        }

        for (i, child) in element.children.iter().enumerate() {
            path.push(i);
            self.collect_focusable_elements(child, path);
            path.pop();
        }
    }

    fn sort_focus_order(&mut self) {
        // Create indices and sort by tab_index
        let mut indexed_elements: Vec<(usize, i32)> = self
            .focusable_elements
            .iter()
            .enumerate()
            .map(|(i, elem)| (i, elem.tab_index))
            .collect();

        indexed_elements.sort_by_key(|(_, tab_index)| *tab_index);

        self.focus_order = indexed_elements.into_iter().map(|(i, _)| i).collect();
    }

    /// Move focus to next element
    pub fn focus_next(&mut self) -> Option<&FocusableElement> {
        if self.focus_order.is_empty() {
            return None;
        }

        let current_order_index = self
            .current_focus_index
            .and_then(|focus_idx| {
                self.focus_order
                    .iter()
                    .position(|&order_idx| order_idx == focus_idx)
            })
            .unwrap_or(0);

        let next_order_index = (current_order_index + 1) % self.focus_order.len();
        let next_focus_index = self.focus_order[next_order_index];

        self.current_focus_index = Some(next_focus_index);
        self.focusable_elements.get(next_focus_index)
    }

    /// Move focus to previous element
    pub fn focus_previous(&mut self) -> Option<&FocusableElement> {
        if self.focus_order.is_empty() {
            return None;
        }

        let current_order_index = self
            .current_focus_index
            .and_then(|focus_idx| {
                self.focus_order
                    .iter()
                    .position(|&order_idx| order_idx == focus_idx)
            })
            .unwrap_or(0);

        let prev_order_index = if current_order_index == 0 {
            self.focus_order.len() - 1
        } else {
            current_order_index - 1
        };

        let prev_focus_index = self.focus_order[prev_order_index];

        self.current_focus_index = Some(prev_focus_index);
        self.focusable_elements.get(prev_focus_index)
    }

    /// Get currently focused element
    pub fn get_focused_element(&self) -> Option<&FocusableElement> {
        self.current_focus_index
            .and_then(|idx| self.focusable_elements.get(idx))
    }

    /// Focus element by ID
    pub fn focus_by_id(&mut self, id: &str) -> Option<&FocusableElement> {
        if let Some(index) = self
            .focusable_elements
            .iter()
            .position(|elem| elem.id == id)
        {
            self.current_focus_index = Some(index);
            self.focusable_elements.get(index)
        } else {
            None
        }
    }

    /// Apply focus state to element tree
    pub fn apply_focus_to_tree(&self, root: &mut Element) {
        if let Some(focused_element) = self.get_focused_element() {
            Self::clear_focus_from_tree(root);
            Self::set_focus_at_path(root, &focused_element.element_path);
        }
    }

    fn clear_focus_from_tree(element: &mut Element) {
        element.focused = false;
        for child in &mut element.children {
            Self::clear_focus_from_tree(child);
        }
    }

    fn set_focus_at_path(element: &mut Element, path: &[usize]) {
        if path.is_empty() {
            element.focused = true;
            return;
        }

        if let Some(&child_index) = path.first() {
            if let Some(child) = element.children.get_mut(child_index) {
                Self::set_focus_at_path(child, &path[1..]);
            }
        }
    }

    /// Get focus navigation info for display
    pub fn get_focus_info(&self) -> FocusInfo {
        FocusInfo {
            total_focusable: self.focusable_elements.len(),
            current_index: self.current_focus_index.map(|idx| {
                self.focus_order
                    .iter()
                    .position(|&order_idx| order_idx == idx)
                    .unwrap_or(0)
            }),
            current_element: self.get_focused_element().map(|elem| elem.id.clone()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FocusInfo {
    pub total_focusable: usize,
    pub current_index: Option<usize>,
    pub current_element: Option<String>,
}

impl Default for FocusManager {
    fn default() -> Self {
        Self::new()
    }
}
