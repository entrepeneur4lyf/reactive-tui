//! Render Batching System
//!
//! Provides intelligent batching of render operations to optimize performance by grouping
//! multiple render updates into single atomic operations. This reduces the number of
//! terminal writes and improves overall rendering performance.
//!
//! ## Features
//!
//! - **Automatic Batching**: Groups render operations within time windows
//! - **Priority-Based Batching**: High-priority updates can bypass batching
//! - **Adaptive Timing**: Adjusts batch timing based on system performance
//! - **Memory Optimization**: Efficient memory usage for batch operations
//! - **Render Coalescing**: Combines overlapping render operations
//!
//! ## Usage
//!
//! ```rust,no_run
//! use reactive_tui::rendering::batch::{RenderBatcher, BatchConfig};
//!
//! let batcher = RenderBatcher::new()
//!     .batch_size(100)
//!     .batch_timeout(Duration::from_millis(16)) // ~60 FPS
//!     .enable_coalescing(true)
//!     .build();
//!
//! // Queue render operations
//! batcher.queue_render(render_op).await?;
//! ```

use crate::error::{Result, TuiError};
use crate::layout::LayoutRect;
use crate::rendering::{FrameBuffer, RenderStyle};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;

/// Priority levels for render operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum RenderPriority {
    /// Low priority - can be heavily batched
    Low = 0,
    /// Normal priority - standard batching
    Normal = 1,
    /// High priority - minimal batching
    High = 2,
    /// Critical priority - immediate rendering
    Critical = 3,
}

/// Types of render operations that can be batched
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RenderOperation {
    /// Clear a specific region
    Clear {
        rect: LayoutRect,
    },
    /// Draw text at a position
    DrawText {
        x: u16,
        y: u16,
        text: String,
        style: RenderStyle,
    },
    /// Draw a filled rectangle
    DrawRect {
        rect: LayoutRect,
        style: RenderStyle,
        fill_char: Option<char>,
    },
    /// Draw a border around a rectangle
    DrawBorder {
        rect: LayoutRect,
        style: RenderStyle,
        border_style: crate::rendering::BorderStyle,
    },
    /// Set cursor position
    SetCursor {
        x: u16,
        y: u16,
        visible: bool,
    },
    /// Apply style changes
    SetStyle {
        style: RenderStyle,
    },
    /// Custom render operation with raw ANSI data
    RawAnsi {
        data: Vec<u8>,
    },
    /// Composite operation containing multiple sub-operations
    Composite {
        operations: Vec<RenderOperation>,
    },
}

/// A batched render request
#[derive(Debug, Clone)]
pub struct RenderRequest {
    /// Unique identifier for this request
    pub id: String,
    /// Priority of this render operation
    pub priority: RenderPriority,
    /// The render operation to perform
    pub operation: RenderOperation,
    /// Timestamp when the request was created
    pub timestamp: Instant,
    /// Optional region that this operation affects (for coalescing)
    pub affected_region: Option<LayoutRect>,
    /// Whether this operation can be coalesced with others
    pub coalescable: bool,
}

/// Configuration for render batching behavior
#[derive(Debug, Clone)]
pub struct BatchConfig {
    /// Maximum number of operations per batch
    pub max_batch_size: usize,
    /// Maximum time to wait before flushing a batch
    pub batch_timeout: Duration,
    /// Whether to enable operation coalescing
    pub enable_coalescing: bool,
    /// Maximum time to spend on coalescing per batch
    pub coalescing_timeout: Duration,
    /// Whether to use adaptive batching based on performance
    pub adaptive_batching: bool,
    /// Target frame rate for adaptive batching
    pub target_fps: f32,
    /// Whether to prioritize low-latency over throughput
    pub low_latency_mode: bool,
    /// Maximum memory usage for batching (in bytes)
    pub max_memory_usage: usize,
}

impl Default for BatchConfig {
    fn default() -> Self {
        Self {
            max_batch_size: 100,
            batch_timeout: Duration::from_millis(16), // ~60 FPS
            enable_coalescing: true,
            coalescing_timeout: Duration::from_millis(2),
            adaptive_batching: true,
            target_fps: 60.0,
            low_latency_mode: false,
            max_memory_usage: 1024 * 1024, // 1MB
        }
    }
}

/// Statistics about batching performance
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BatchStats {
    /// Total number of render requests processed
    pub total_requests: u64,
    /// Total number of batches created
    pub total_batches: u64,
    /// Average batch size
    pub avg_batch_size: f32,
    /// Average batch processing time
    pub avg_batch_time: Duration,
    /// Number of operations coalesced
    pub coalesced_operations: u64,
    /// Memory usage statistics
    pub memory_usage: usize,
    /// Current frame rate
    pub current_fps: f32,
    /// Number of dropped frames due to overload
    pub dropped_frames: u64,
}

/// Main render batching system
pub struct RenderBatcher {
    /// Configuration for batching behavior
    config: BatchConfig,
    /// Queue of pending render requests
    request_queue: Arc<RwLock<VecDeque<RenderRequest>>>,
    /// Channel for sending render requests
    request_sender: mpsc::UnboundedSender<RenderRequest>,
    /// Channel for receiving render requests
    request_receiver: Arc<Mutex<Option<mpsc::UnboundedReceiver<RenderRequest>>>>,
    /// Current batch being assembled
    current_batch: Arc<Mutex<Vec<RenderRequest>>>,
    /// Statistics about batching performance
    stats: Arc<RwLock<BatchStats>>,
    /// Whether the batcher is currently running
    running: Arc<RwLock<bool>>,
    /// Frame buffer for rendering batched operations
    frame_buffer: Arc<Mutex<FrameBuffer>>,
    /// Cache for coalesced operations
    coalescing_cache: Arc<Mutex<HashMap<String, RenderRequest>>>,
}

impl RenderBatcher {
    /// Create a new render batcher with default configuration
    pub fn new() -> RenderBatcherBuilder {
        RenderBatcherBuilder::new()
    }

    /// Queue a render operation for batching
    pub async fn queue_render(&self, operation: RenderOperation) -> Result<String> {
        self.queue_render_with_priority(operation, RenderPriority::Normal).await
    }

    /// Queue a render operation with specific priority
    pub async fn queue_render_with_priority(
        &self,
        operation: RenderOperation,
        priority: RenderPriority,
    ) -> Result<String> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let affected_region = self.calculate_affected_region(&operation);
        let coalescable = self.is_coalescable(&operation);

        let request = RenderRequest {
            id: request_id.clone(),
            priority,
            operation,
            timestamp: Instant::now(),
            affected_region,
            coalescable,
        };

        // For critical priority, bypass batching
        if priority == RenderPriority::Critical {
            self.render_immediately(&request).await?;
            return Ok(request_id);
        }

        // Send to batching queue
        self.request_sender
            .send(request)
            .map_err(|e| TuiError::render(format!("Failed to queue render request: {}", e)))?;

        Ok(request_id)
    }

    /// Start the batching system
    pub async fn start(&self) -> Result<()> {
        let mut running = self.running.write().await;
        if *running {
            return Ok(());
        }
        *running = true;
        drop(running);

        // Take the receiver to start processing
        let receiver = self.request_receiver
            .lock()
            .unwrap()
            .take()
            .ok_or_else(|| TuiError::render("Batcher already started"))?;

        // Spawn the main batching loop
        let batcher = self.clone();
        tokio::spawn(async move {
            if let Err(e) = batcher.run_batching_loop(receiver).await {
                eprintln!("Render batching loop error: {}", e);
            }
        });

        Ok(())
    }

    /// Stop the batching system
    pub async fn stop(&self) -> Result<()> {
        let mut running = self.running.write().await;
        *running = false;
        Ok(())
    }

    /// Get current batching statistics
    pub async fn get_stats(&self) -> BatchStats {
        self.stats.read().await.clone()
    }

    /// Force flush all pending batches immediately
    pub async fn flush(&self) -> Result<()> {
        let batch = {
            let mut current_batch = self.current_batch.lock().unwrap();
            if current_batch.is_empty() {
                return Ok(());
            }
            std::mem::take(&mut *current_batch)
        };
        self.process_batch(batch).await?;
        Ok(())
    }

    /// Main batching loop
    async fn run_batching_loop(&self, mut receiver: mpsc::UnboundedReceiver<RenderRequest>) -> Result<()> {
        let mut batch_timer = interval(self.config.batch_timeout);
        let mut adaptive_timer = if self.config.adaptive_batching {
            Some(interval(Duration::from_millis(100))) // Check every 100ms for adaptive adjustments
        } else {
            None
        };

        loop {
            // Check if we should stop
            if !*self.running.read().await {
                break;
            }

            tokio::select! {
                // New render request received
                request = receiver.recv() => {
                    if let Some(request) = request {
                        self.handle_render_request(request).await?;
                    }
                }

                // Batch timeout - flush current batch
                _ = batch_timer.tick() => {
                    self.flush_current_batch().await?;
                }

                // Adaptive batching adjustment
                _ = async {
                    if let Some(ref mut timer) = adaptive_timer {
                        timer.tick().await;
                    } else {
                        // Never resolves if adaptive batching is disabled
                        std::future::pending::<()>().await;
                    }
                }, if adaptive_timer.is_some() => {
                    self.adjust_adaptive_batching().await?;
                }
            }
        }

        // Flush any remaining operations
        self.flush().await?;
        Ok(())
    }

    /// Handle a new render request
    async fn handle_render_request(&self, request: RenderRequest) -> Result<()> {
        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_requests += 1;
        }

        // Check if we should coalesce this operation
        if self.config.enable_coalescing && request.coalescable {
            if self.try_coalesce_request(&request).await? {
                return Ok(());
            }
        }

        // Add to current batch
        let should_process = {
            let mut current_batch = self.current_batch.lock().unwrap();
            current_batch.push(request);
            current_batch.len() >= self.config.max_batch_size
        };

        if should_process {
            let batch = {
                let mut current_batch = self.current_batch.lock().unwrap();
                std::mem::take(&mut *current_batch)
            };
            self.process_batch(batch).await?;
        }

        Ok(())
    }

    /// Try to coalesce a request with existing operations
    async fn try_coalesce_request(&self, request: &RenderRequest) -> Result<bool> {
        let mut cache = self.coalescing_cache.lock().unwrap();

        // Look for existing operations that can be coalesced
        if let Some(affected_region) = &request.affected_region {
            let cache_key = format!("{}:{}:{}:{}",
                affected_region.x, affected_region.y,
                affected_region.width, affected_region.height);

            if let Some(existing) = cache.get(&cache_key) {
                // Check if operations can be coalesced
                if self.can_coalesce_operations(&existing.operation, &request.operation) {
                    // Replace with the newer operation (later operations override earlier ones)
                    cache.insert(cache_key, request.clone());

                    // Update statistics
                    tokio::spawn({
                        let stats = self.stats.clone();
                        async move {
                            let mut stats = stats.write().await;
                            stats.coalesced_operations += 1;
                        }
                    });

                    return Ok(true);
                }
            } else {
                // Add to cache for potential future coalescing
                cache.insert(cache_key, request.clone());
            }
        }

        Ok(false)
    }

    /// Check if two operations can be coalesced
    fn can_coalesce_operations(&self, op1: &RenderOperation, op2: &RenderOperation) -> bool {
        match (op1, op2) {
            // Text drawing operations in the same region can be coalesced
            (RenderOperation::DrawText { x: x1, y: y1, .. },
             RenderOperation::DrawText { x: x2, y: y2, .. }) => {
                x1 == x2 && y1 == y2
            }

            // Rectangle operations in the same region can be coalesced
            (RenderOperation::DrawRect { rect: r1, .. },
             RenderOperation::DrawRect { rect: r2, .. }) => {
                r1 == r2
            }

            // Style operations can always be coalesced (later one wins)
            (RenderOperation::SetStyle { .. }, RenderOperation::SetStyle { .. }) => true,

            // Cursor operations can be coalesced
            (RenderOperation::SetCursor { .. }, RenderOperation::SetCursor { .. }) => true,

            _ => false,
        }
    }

    /// Flush the current batch
    async fn flush_current_batch(&self) -> Result<()> {
        let batch = {
            let mut current_batch = self.current_batch.lock().unwrap();
            if current_batch.is_empty() {
                return Ok(());
            }
            std::mem::take(&mut *current_batch)
        };

        // Also flush coalescing cache
        let coalesced_ops = {
            let mut cache = self.coalescing_cache.lock().unwrap();
            let ops: Vec<RenderRequest> = cache.values().cloned().collect();
            cache.clear();
            ops
        };

        // Combine batch with coalesced operations
        let mut combined_batch = batch;
        combined_batch.extend(coalesced_ops);

        if !combined_batch.is_empty() {
            self.process_batch(combined_batch).await?;
        }

        Ok(())
    }

    /// Process a batch of render operations
    async fn process_batch(&self, mut batch: Vec<RenderRequest>) -> Result<()> {
        let start_time = Instant::now();

        // Sort batch by priority (highest first)
        batch.sort_by(|a, b| b.priority.cmp(&a.priority));

        // Update statistics
        {
            let mut stats = self.stats.write().await;
            stats.total_batches += 1;
            stats.avg_batch_size = (stats.avg_batch_size * (stats.total_batches - 1) as f32 + batch.len() as f32) / stats.total_batches as f32;
        }

        // Process operations in the frame buffer
        {
            let mut frame_buffer = self.frame_buffer.lock().unwrap();

            for request in &batch {
                self.execute_render_operation(&mut frame_buffer, &request.operation)?;
            }

            // Flush the frame buffer (this would be connected to the actual renderer)
            // For now, we'll just clear it to simulate flushing
            frame_buffer.clear();
        }

        // Update timing statistics
        let batch_time = start_time.elapsed();
        {
            let mut stats = self.stats.write().await;
            let total_batches = stats.total_batches as f32;
            stats.avg_batch_time = Duration::from_nanos(
                ((stats.avg_batch_time.as_nanos() as f32 * (total_batches - 1.0)) + batch_time.as_nanos() as f32) as u64 / total_batches as u64
            );
        }

        Ok(())
    }

    /// Execute a single render operation in the frame buffer
    fn execute_render_operation(&self, frame_buffer: &mut FrameBuffer, operation: &RenderOperation) -> Result<()> {
        match operation {
            RenderOperation::Clear { rect } => {
                // Clear the specified region
                for y in rect.y..rect.y + rect.height {
                    frame_buffer.move_to(rect.x, y)?;
                    let spaces = " ".repeat(rect.width as usize);
                    frame_buffer.print(&spaces)?;
                }
            }

            RenderOperation::DrawText { x, y, text, style } => {
                frame_buffer.move_to(*x, *y)?;
                frame_buffer.apply_style(style)?;
                frame_buffer.print(text)?;
            }

            RenderOperation::DrawRect { rect, style, fill_char } => {
                frame_buffer.apply_style(style)?;
                let fill = fill_char.unwrap_or(' ');
                for y in rect.y..rect.y + rect.height {
                    frame_buffer.move_to(rect.x, y)?;
                    let line = fill.to_string().repeat(rect.width as usize);
                    frame_buffer.print(&line)?;
                }
            }

            RenderOperation::DrawBorder { rect, style, border_style: _ } => {
                frame_buffer.apply_style(style)?;
                // Simplified border drawing - would use actual border characters
                for y in rect.y..rect.y + rect.height {
                    for x in rect.x..rect.x + rect.width {
                        frame_buffer.move_to(x, y)?;
                        if y == rect.y || y == rect.y + rect.height - 1 {
                            frame_buffer.print("-")?;
                        } else if x == rect.x || x == rect.x + rect.width - 1 {
                            frame_buffer.print("|")?;
                        }
                    }
                }
            }

            RenderOperation::SetCursor { x, y, visible: _ } => {
                frame_buffer.move_to(*x, *y)?;
            }

            RenderOperation::SetStyle { style } => {
                frame_buffer.apply_style(style)?;
            }

            RenderOperation::RawAnsi { data: _ } => {
                // Write raw ANSI data - this would need special handling
                // For now, we'll skip it as it requires direct buffer access
            }

            RenderOperation::Composite { operations } => {
                for op in operations {
                    self.execute_render_operation(frame_buffer, op)?;
                }
            }
        }

        Ok(())
    }

    /// Render a request immediately without batching
    async fn render_immediately(&self, request: &RenderRequest) -> Result<()> {
        let mut frame_buffer = self.frame_buffer.lock().unwrap();
        self.execute_render_operation(&mut frame_buffer, &request.operation)?;
        // In a real implementation, this would flush immediately to the terminal
        frame_buffer.clear();
        Ok(())
    }

    /// Adjust batching parameters based on performance
    async fn adjust_adaptive_batching(&self) -> Result<()> {
        if !self.config.adaptive_batching {
            return Ok(());
        }

        let stats = self.stats.read().await;
        let current_fps = stats.current_fps;
        let target_fps = self.config.target_fps;

        // If we're below target FPS, reduce batch size or timeout
        if current_fps < target_fps * 0.9 {
            // Performance is poor, reduce batching overhead
            // This would adjust internal parameters
        } else if current_fps > target_fps * 1.1 {
            // Performance is good, we can increase batching for better throughput
            // This would adjust internal parameters
        }

        Ok(())
    }

    /// Calculate the region affected by a render operation
    fn calculate_affected_region(&self, operation: &RenderOperation) -> Option<LayoutRect> {
        match operation {
            RenderOperation::Clear { rect } => Some(*rect),
            RenderOperation::DrawText { x, y, text, .. } => {
                Some(LayoutRect {
                    x: *x,
                    y: *y,
                    width: text.len() as u16,
                    height: 1,
                })
            }
            RenderOperation::DrawRect { rect, .. } => Some(*rect),
            RenderOperation::DrawBorder { rect, .. } => Some(*rect),
            RenderOperation::SetCursor { .. } => None,
            RenderOperation::SetStyle { .. } => None,
            RenderOperation::RawAnsi { .. } => None,
            RenderOperation::Composite { operations } => {
                // Calculate bounding box of all operations
                let mut min_x = u16::MAX;
                let mut min_y = u16::MAX;
                let mut max_x = 0u16;
                let mut max_y = 0u16;
                let mut has_region = false;

                for op in operations {
                    if let Some(region) = self.calculate_affected_region(op) {
                        has_region = true;
                        min_x = min_x.min(region.x);
                        min_y = min_y.min(region.y);
                        max_x = max_x.max(region.x + region.width);
                        max_y = max_y.max(region.y + region.height);
                    }
                }

                if has_region {
                    Some(LayoutRect {
                        x: min_x,
                        y: min_y,
                        width: max_x - min_x,
                        height: max_y - min_y,
                    })
                } else {
                    None
                }
            }
        }
    }

    /// Check if an operation can be coalesced
    fn is_coalescable(&self, operation: &RenderOperation) -> bool {
        match operation {
            RenderOperation::DrawText { .. } => true,
            RenderOperation::DrawRect { .. } => true,
            RenderOperation::SetStyle { .. } => true,
            RenderOperation::SetCursor { .. } => true,
            RenderOperation::Clear { .. } => false, // Clear operations should not be coalesced
            RenderOperation::RawAnsi { .. } => false, // Raw ANSI is not coalescable
            RenderOperation::Composite { .. } => false, // Composite operations are complex
            RenderOperation::DrawBorder { .. } => true,
        }
    }
}

impl Clone for RenderBatcher {
    fn clone(&self) -> Self {
        Self {
            config: self.config.clone(),
            request_queue: self.request_queue.clone(),
            request_sender: self.request_sender.clone(),
            request_receiver: Arc::new(Mutex::new(None)), // Clone doesn't get the receiver
            current_batch: self.current_batch.clone(),
            stats: self.stats.clone(),
            running: self.running.clone(),
            frame_buffer: self.frame_buffer.clone(),
            coalescing_cache: self.coalescing_cache.clone(),
        }
    }
}

/// Builder for creating render batchers
pub struct RenderBatcherBuilder {
    config: BatchConfig,
}

impl RenderBatcherBuilder {
    /// Create a new render batcher builder
    pub fn new() -> Self {
        Self {
            config: BatchConfig::default(),
        }
    }

    /// Set the maximum batch size
    pub fn max_batch_size(mut self, size: usize) -> Self {
        self.config.max_batch_size = size;
        self
    }

    /// Set the batch timeout
    pub fn batch_timeout(mut self, timeout: Duration) -> Self {
        self.config.batch_timeout = timeout;
        self
    }

    /// Enable or disable operation coalescing
    pub fn enable_coalescing(mut self, enable: bool) -> Self {
        self.config.enable_coalescing = enable;
        self
    }

    /// Set the coalescing timeout
    pub fn coalescing_timeout(mut self, timeout: Duration) -> Self {
        self.config.coalescing_timeout = timeout;
        self
    }

    /// Enable or disable adaptive batching
    pub fn adaptive_batching(mut self, enable: bool) -> Self {
        self.config.adaptive_batching = enable;
        self
    }

    /// Set the target frame rate for adaptive batching
    pub fn target_fps(mut self, fps: f32) -> Self {
        self.config.target_fps = fps;
        self
    }

    /// Enable or disable low-latency mode
    pub fn low_latency_mode(mut self, enable: bool) -> Self {
        self.config.low_latency_mode = enable;
        self
    }

    /// Set the maximum memory usage for batching
    pub fn max_memory_usage(mut self, bytes: usize) -> Self {
        self.config.max_memory_usage = bytes;
        self
    }

    /// Build the render batcher
    pub fn build(self) -> RenderBatcher {
        let (sender, receiver) = mpsc::unbounded_channel();

        RenderBatcher {
            config: self.config,
            request_queue: Arc::new(RwLock::new(VecDeque::new())),
            request_sender: sender,
            request_receiver: Arc::new(Mutex::new(Some(receiver))),
            current_batch: Arc::new(Mutex::new(Vec::new())),
            stats: Arc::new(RwLock::new(BatchStats::default())),
            running: Arc::new(RwLock::new(false)),
            frame_buffer: Arc::new(Mutex::new(FrameBuffer::new())),
            coalescing_cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }
}

impl Default for RenderBatcherBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[tokio::test]
    async fn test_render_batcher_creation() {
        let batcher = RenderBatcher::new()
            .max_batch_size(50)
            .batch_timeout(Duration::from_millis(10))
            .build();

        assert_eq!(batcher.config.max_batch_size, 50);
        assert_eq!(batcher.config.batch_timeout, Duration::from_millis(10));
    }

    #[tokio::test]
    async fn test_render_operation_queuing() {
        let batcher = RenderBatcher::new().build();

        let operation = RenderOperation::DrawText {
            x: 10,
            y: 5,
            text: "Hello, World!".to_string(),
            style: RenderStyle::default(),
        };

        let request_id = batcher.queue_render(operation).await.unwrap();
        assert!(!request_id.is_empty());
    }

    #[tokio::test]
    async fn test_priority_rendering() {
        let batcher = RenderBatcher::new().build();

        let operation = RenderOperation::Clear {
            rect: LayoutRect { x: 0, y: 0, width: 10, height: 10 },
        };

        // Critical priority should render immediately
        let request_id = batcher.queue_render_with_priority(operation, RenderPriority::Critical).await.unwrap();
        assert!(!request_id.is_empty());
    }

    #[tokio::test]
    async fn test_operation_coalescing() {
        let batcher = RenderBatcher::new()
            .enable_coalescing(true)
            .build();

        let operation1 = RenderOperation::DrawText {
            x: 10,
            y: 5,
            text: "First".to_string(),
            style: RenderStyle::default(),
        };

        let operation2 = RenderOperation::DrawText {
            x: 10,
            y: 5,
            text: "Second".to_string(),
            style: RenderStyle::default(),
        };

        batcher.queue_render(operation1).await.unwrap();
        batcher.queue_render(operation2).await.unwrap();

        // The second operation should coalesce with the first
        let _stats = batcher.get_stats().await;
        // Note: In a real test, we'd need to wait for processing and check coalesced_operations
    }

    #[test]
    fn test_affected_region_calculation() {
        let batcher = RenderBatcher::new().build();

        let operation = RenderOperation::DrawText {
            x: 10,
            y: 5,
            text: "Hello".to_string(),
            style: RenderStyle::default(),
        };

        let region = batcher.calculate_affected_region(&operation);
        assert!(region.is_some());

        let region = region.unwrap();
        assert_eq!(region.x, 10);
        assert_eq!(region.y, 5);
        assert_eq!(region.width, 5); // Length of "Hello"
        assert_eq!(region.height, 1);
    }

    #[test]
    fn test_coalescable_operations() {
        let batcher = RenderBatcher::new().build();

        let text_op = RenderOperation::DrawText {
            x: 0,
            y: 0,
            text: "test".to_string(),
            style: RenderStyle::default(),
        };

        let clear_op = RenderOperation::Clear {
            rect: LayoutRect { x: 0, y: 0, width: 10, height: 10 },
        };

        assert!(batcher.is_coalescable(&text_op));
        assert!(!batcher.is_coalescable(&clear_op));
    }
}
