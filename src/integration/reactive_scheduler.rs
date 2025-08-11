//! Reactive update scheduler for efficient component updates

use super::{ComponentId, ReactiveId};
use crate::error::Result;
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};

/// Priority levels for component updates
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum UpdatePriority {
  /// Low priority updates (background changes)
  Low = 0,
  /// Normal priority updates (user interactions)
  Normal = 1,
  /// High priority updates (critical UI changes)
  High = 2,
  /// Immediate updates (error states, urgent notifications)
  Immediate = 3,
}

impl Default for UpdatePriority {
  fn default() -> Self {
    Self::Normal
  }
}

/// Request for a component update
#[derive(Debug, Clone)]
pub struct UpdateRequest {
  pub component_id: ComponentId,
  pub reactive_id: ReactiveId,
  pub timestamp: Instant,
  pub priority: UpdatePriority,
}

impl UpdateRequest {
  /// Create a new update request
  pub fn new(component_id: ComponentId, reactive_id: ReactiveId, priority: UpdatePriority) -> Self {
    Self {
      component_id,
      reactive_id,
      timestamp: Instant::now(),
      priority,
    }
  }

  /// Check if this update request is stale
  pub fn is_stale(&self, max_age: Duration) -> bool {
    self.timestamp.elapsed() > max_age
  }
}

/// Statistics for update scheduling
#[derive(Debug, Default)]
pub struct SchedulerStats {
  pub total_updates_scheduled: u64,
  pub total_updates_processed: u64,
  pub total_updates_batched: u64,
  pub total_updates_dropped: u64,
  pub average_batch_size: f64,
  pub last_batch_time: Option<Instant>,
}

/// Scheduler for reactive component updates
pub struct ReactiveUpdateScheduler {
  /// Pending updates organized by priority
  update_queues: HashMap<UpdatePriority, VecDeque<UpdateRequest>>,
  /// Component update deduplication
  pending_components: HashMap<ComponentId, UpdateRequest>,
  /// Batching configuration
  batch_size: usize,
  batch_timeout: Duration,
  /// Statistics
  stats: SchedulerStats,
  /// Last batch processing time
  last_batch: Option<Instant>,
}

impl ReactiveUpdateScheduler {
  /// Create a new reactive update scheduler
  pub fn new() -> Self {
    Self {
      update_queues: HashMap::new(),
      pending_components: HashMap::new(),
      batch_size: 10,
      batch_timeout: Duration::from_millis(16), // ~60fps
      stats: SchedulerStats::default(),
      last_batch: None,
    }
  }

  /// Create scheduler with custom batching configuration
  pub fn with_batching(batch_size: usize, batch_timeout: Duration) -> Self {
    Self {
      update_queues: HashMap::new(),
      pending_components: HashMap::new(),
      batch_size,
      batch_timeout,
      stats: SchedulerStats::default(),
      last_batch: None,
    }
  }

  /// Schedule a component update
  pub async fn schedule_update(&mut self, request: UpdateRequest) -> Result<()> {
    let component_id = request.component_id.clone();
    let priority = request.priority;

    // Check for duplicate updates for the same component
    if let Some(existing) = self.pending_components.get(&component_id).cloned() {
      // If new request has higher priority, replace existing
      if request.priority > existing.priority {
        self
          .pending_components
          .insert(component_id.clone(), request.clone());

        // Remove old request from queue
        if let Some(queue) = self.update_queues.get_mut(&existing.priority) {
          queue.retain(|req| req.component_id != component_id);
        }

        // Add new request to appropriate queue
        self
          .update_queues
          .entry(priority)
          .or_default()
          .push_back(request);
      }
      // Otherwise, ignore duplicate lower-priority request
      return Ok(());
    }

    // Add new request
    self
      .pending_components
      .insert(component_id, request.clone());
    self
      .update_queues
      .entry(priority)
      .or_default()
      .push_back(request);

    self.stats.total_updates_scheduled += 1;
    Ok(())
  }

  /// Get pending updates, respecting priority and batching
  pub async fn get_pending_updates(&mut self) -> Result<Vec<UpdateRequest>> {
    let now = Instant::now();

    // Check if we should process a batch
    let should_batch = self.should_process_batch(now);

    if !should_batch {
      return Ok(Vec::new());
    }

    let mut batch = Vec::new();
    let mut processed_components = std::collections::HashSet::new();

    // Process updates by priority (highest first)
    let priorities = [
      UpdatePriority::Immediate,
      UpdatePriority::High,
      UpdatePriority::Normal,
      UpdatePriority::Low,
    ];

    for priority in priorities {
      if batch.len() >= self.batch_size {
        break;
      }

      if let Some(queue) = self.update_queues.get_mut(&priority) {
        while let Some(request) = queue.pop_front() {
          if batch.len() >= self.batch_size {
            // Put request back for next batch
            queue.push_front(request);
            break;
          }

          // Skip if component already processed in this batch
          if processed_components.contains(&request.component_id) {
            continue;
          }

          // Remove stale requests
          if request.is_stale(Duration::from_secs(1)) {
            self.pending_components.remove(&request.component_id);
            self.stats.total_updates_dropped += 1;
            continue;
          }

          batch.push(request.clone());
          processed_components.insert(request.component_id.clone());
          self.pending_components.remove(&request.component_id);
        }
      }
    }

    // Update statistics
    if !batch.is_empty() {
      self.stats.total_updates_processed += batch.len() as u64;
      self.stats.total_updates_batched += 1;
      self.stats.average_batch_size = (self.stats.average_batch_size
        * (self.stats.total_updates_batched - 1) as f64
        + batch.len() as f64)
        / self.stats.total_updates_batched as f64;
      self.stats.last_batch_time = Some(now);
      self.last_batch = Some(now);
    }

    Ok(batch)
  }

  /// Check if we should process a batch now
  fn should_process_batch(&self, now: Instant) -> bool {
    // Always process immediate priority updates
    if let Some(queue) = self.update_queues.get(&UpdatePriority::Immediate) {
      if !queue.is_empty() {
        return true;
      }
    }

    // Check if batch timeout has elapsed
    if let Some(last_batch) = self.last_batch {
      if now.duration_since(last_batch) >= self.batch_timeout {
        return true;
      }
    } else if !self.pending_components.is_empty() {
      // First batch
      return true;
    }

    // Check if batch size is reached
    self.pending_components.len() >= self.batch_size
  }

  /// Force process all pending updates
  pub async fn flush_all_updates(&mut self) -> Result<Vec<UpdateRequest>> {
    let mut all_updates = Vec::new();

    // Process all priorities
    let priorities = [
      UpdatePriority::Immediate,
      UpdatePriority::High,
      UpdatePriority::Normal,
      UpdatePriority::Low,
    ];

    for priority in priorities {
      if let Some(queue) = self.update_queues.get_mut(&priority) {
        while let Some(request) = queue.pop_front() {
          all_updates.push(request.clone());
          self.pending_components.remove(&request.component_id);
        }
      }
    }

    // Update statistics
    if !all_updates.is_empty() {
      self.stats.total_updates_processed += all_updates.len() as u64;
      self.stats.total_updates_batched += 1;
      self.last_batch = Some(Instant::now());
    }

    Ok(all_updates)
  }

  /// Get scheduler statistics
  pub fn get_stats(&self) -> &SchedulerStats {
    &self.stats
  }

  /// Clear all pending updates
  pub fn clear_all(&mut self) {
    self.update_queues.clear();
    self.pending_components.clear();
  }

  /// Get number of pending updates
  pub fn pending_count(&self) -> usize {
    self.pending_components.len()
  }

  /// Check if there are any pending updates
  pub fn has_pending_updates(&self) -> bool {
    !self.pending_components.is_empty()
  }

  /// Set batch configuration
  pub fn set_batch_config(&mut self, batch_size: usize, batch_timeout: Duration) {
    self.batch_size = batch_size;
    self.batch_timeout = batch_timeout;
  }

  /// Get current batch configuration
  pub fn get_batch_config(&self) -> (usize, Duration) {
    (self.batch_size, self.batch_timeout)
  }
}

impl Default for ReactiveUpdateScheduler {
  fn default() -> Self {
    Self::new()
  }
}
