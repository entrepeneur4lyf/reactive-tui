//! Update coordinator for managing reactive bindings between components and reactive values

use super::{ComponentId, ReactiveId};
use std::collections::{HashMap, HashSet};

/// Binding between a reactive value and a component
#[derive(Debug, Clone)]
pub struct ReactiveBinding {
  pub reactive_id: ReactiveId,
  pub component_id: ComponentId,
  pub field_path: Option<String>, // For nested reactive state
  pub binding_type: BindingType,
}

/// Type of reactive binding
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BindingType {
  /// Component reads from reactive value
  Read,
  /// Component writes to reactive value
  Write,
  /// Component both reads and writes
  ReadWrite,
  /// Component computes derived value
  Computed,
}

impl ReactiveBinding {
  /// Create a new reactive binding
  pub fn new(reactive_id: ReactiveId, component_id: ComponentId) -> Self {
    Self {
      reactive_id,
      component_id,
      field_path: None,
      binding_type: BindingType::Read,
    }
  }

  /// Set the field path for nested state
  pub fn with_field_path(mut self, field_path: String) -> Self {
    self.field_path = Some(field_path);
    self
  }

  /// Set the binding type
  pub fn with_type(mut self, binding_type: BindingType) -> Self {
    self.binding_type = binding_type;
    self
  }
}

/// Coordinates updates between reactive values and components
pub struct UpdateCoordinator {
  /// Map from reactive ID to components that depend on it
  reactive_to_components: HashMap<ReactiveId, HashSet<ComponentId>>,
  /// Map from component ID to reactive values it depends on
  component_to_reactives: HashMap<ComponentId, HashSet<ReactiveId>>,
  /// Detailed binding information
  bindings: HashMap<(ReactiveId, ComponentId), ReactiveBinding>,
  /// Computed value dependencies
  computed_dependencies: HashMap<ReactiveId, HashSet<ReactiveId>>,
}

impl UpdateCoordinator {
  /// Create a new update coordinator
  pub fn new() -> Self {
    Self {
      reactive_to_components: HashMap::new(),
      component_to_reactives: HashMap::new(),
      bindings: HashMap::new(),
      computed_dependencies: HashMap::new(),
    }
  }

  /// Bind a reactive value to a component
  pub fn bind_reactive_to_component(&mut self, reactive_id: ReactiveId, component_id: ComponentId) {
    // Add to reactive -> components mapping
    self
      .reactive_to_components
      .entry(reactive_id.clone())
      .or_default()
      .insert(component_id.clone());

    // Add to component -> reactives mapping
    self
      .component_to_reactives
      .entry(component_id.clone())
      .or_default()
      .insert(reactive_id.clone());

    // Store basic binding
    let binding = ReactiveBinding::new(reactive_id.clone(), component_id.clone());
    self.bindings.insert((reactive_id, component_id), binding);
  }

  /// Add a detailed reactive binding
  pub fn add_binding(&mut self, binding: ReactiveBinding) {
    let reactive_id = binding.reactive_id.clone();
    let component_id = binding.component_id.clone();

    // Update mappings
    self
      .reactive_to_components
      .entry(reactive_id.clone())
      .or_default()
      .insert(component_id.clone());

    self
      .component_to_reactives
      .entry(component_id.clone())
      .or_default()
      .insert(reactive_id.clone());

    // Store detailed binding
    self.bindings.insert((reactive_id, component_id), binding);
  }

  /// Remove binding between reactive value and component
  pub fn unbind_reactive_from_component(
    &mut self,
    reactive_id: &ReactiveId,
    component_id: &ComponentId,
  ) {
    // Remove from reactive -> components mapping
    if let Some(components) = self.reactive_to_components.get_mut(reactive_id) {
      components.remove(component_id);
      if components.is_empty() {
        self.reactive_to_components.remove(reactive_id);
      }
    }

    // Remove from component -> reactives mapping
    if let Some(reactives) = self.component_to_reactives.get_mut(component_id) {
      reactives.remove(reactive_id);
      if reactives.is_empty() {
        self.component_to_reactives.remove(component_id);
      }
    }

    // Remove binding
    self
      .bindings
      .remove(&(reactive_id.clone(), component_id.clone()));
  }

  /// Get all components that depend on a reactive value
  pub fn get_components_for_reactive(&self, reactive_id: &ReactiveId) -> Vec<ComponentId> {
    self
      .reactive_to_components
      .get(reactive_id)
      .map(|components| components.iter().cloned().collect())
      .unwrap_or_default()
  }

  /// Get all reactive values that a component depends on
  pub fn get_reactives_for_component(&self, component_id: &ComponentId) -> Vec<ReactiveId> {
    self
      .component_to_reactives
      .get(component_id)
      .map(|reactives| reactives.iter().cloned().collect())
      .unwrap_or_default()
  }

  /// Get binding details between reactive value and component
  pub fn get_binding(
    &self,
    reactive_id: &ReactiveId,
    component_id: &ComponentId,
  ) -> Option<&ReactiveBinding> {
    self
      .bindings
      .get(&(reactive_id.clone(), component_id.clone()))
  }

  /// Clean up all bindings for a component
  pub fn cleanup_component_bindings(&mut self, component_id: &ComponentId) {
    // Get all reactive values this component depends on
    let reactive_ids = self.get_reactives_for_component(component_id);

    // Remove bindings
    for reactive_id in reactive_ids {
      self.unbind_reactive_from_component(&reactive_id, component_id);
    }
  }

  /// Clean up all bindings for a reactive value
  pub fn cleanup_reactive_bindings(&mut self, reactive_id: &ReactiveId) {
    // Get all components that depend on this reactive value
    let component_ids = self.get_components_for_reactive(reactive_id);

    // Remove bindings
    for component_id in component_ids {
      self.unbind_reactive_from_component(reactive_id, &component_id);
    }
  }

  /// Add computed value dependency
  pub fn add_computed_dependency(&mut self, computed_id: ReactiveId, dependency_id: ReactiveId) {
    self
      .computed_dependencies
      .entry(computed_id)
      .or_default()
      .insert(dependency_id);
  }

  /// Get all reactive values that need to be updated when a value changes (including computed)
  pub fn get_all_affected_reactives(&self, reactive_id: &ReactiveId) -> Vec<ReactiveId> {
    let mut affected = Vec::new();
    let mut visited = HashSet::new();

    self.collect_affected_reactives(reactive_id, &mut affected, &mut visited);

    affected
  }

  /// Recursively collect affected reactive values
  fn collect_affected_reactives(
    &self,
    reactive_id: &ReactiveId,
    affected: &mut Vec<ReactiveId>,
    visited: &mut HashSet<ReactiveId>,
  ) {
    if visited.contains(reactive_id) {
      return; // Prevent infinite loops
    }

    visited.insert(reactive_id.clone());
    affected.push(reactive_id.clone());

    // Find computed values that depend on this reactive value
    for (computed_id, dependencies) in &self.computed_dependencies {
      if dependencies.contains(reactive_id) {
        self.collect_affected_reactives(computed_id, affected, visited);
      }
    }
  }

  /// Get statistics about bindings
  pub fn get_binding_stats(&self) -> BindingStats {
    BindingStats {
      total_bindings: self.bindings.len(),
      total_reactive_values: self.reactive_to_components.len(),
      total_components: self.component_to_reactives.len(),
      total_computed_dependencies: self.computed_dependencies.len(),
      average_bindings_per_component: if self.component_to_reactives.is_empty() {
        0.0
      } else {
        self.bindings.len() as f64 / self.component_to_reactives.len() as f64
      },
      average_components_per_reactive: if self.reactive_to_components.is_empty() {
        0.0
      } else {
        self
          .reactive_to_components
          .values()
          .map(|components| components.len())
          .sum::<usize>() as f64
          / self.reactive_to_components.len() as f64
      },
    }
  }

  /// Check if a component has any reactive bindings
  pub fn has_bindings(&self, component_id: &ComponentId) -> bool {
    self.component_to_reactives.contains_key(component_id)
  }

  /// Check if a reactive value has any component bindings
  pub fn has_component_bindings(&self, reactive_id: &ReactiveId) -> bool {
    self.reactive_to_components.contains_key(reactive_id)
  }

  /// Get all reactive IDs
  pub fn get_all_reactive_ids(&self) -> Vec<ReactiveId> {
    self.reactive_to_components.keys().cloned().collect()
  }

  /// Get all component IDs
  pub fn get_all_component_ids(&self) -> Vec<ComponentId> {
    self.component_to_reactives.keys().cloned().collect()
  }
}

/// Statistics about reactive bindings
#[derive(Debug)]
pub struct BindingStats {
  pub total_bindings: usize,
  pub total_reactive_values: usize,
  pub total_components: usize,
  pub total_computed_dependencies: usize,
  pub average_bindings_per_component: f64,
  pub average_components_per_reactive: f64,
}

impl Default for UpdateCoordinator {
  fn default() -> Self {
    Self::new()
  }
}
