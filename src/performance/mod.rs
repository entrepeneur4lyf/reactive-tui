use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Comprehensive performance monitoring system
#[derive(Debug)]
pub struct PerformanceMonitor {
    metrics: Arc<Mutex<PerformanceMetrics>>,
    frame_times: VecDeque<Duration>,
    render_times: VecDeque<Duration>,
    memory_usage: VecDeque<usize>,
    event_processing_times: HashMap<String, VecDeque<Duration>>,
    reactive_update_times: VecDeque<Duration>,
    component_update_times: HashMap<String, VecDeque<Duration>>,
    css_processing_times: VecDeque<Duration>,
    layout_computation_times: VecDeque<Duration>,
    start_time: Instant,
    last_report: Instant,
    report_interval: Duration,
}

/// Comprehensive performance metrics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    // Frame performance
    pub avg_frame_time: Duration,
    pub max_frame_time: Duration,
    pub min_frame_time: Duration,
    pub current_fps: f64,
    pub target_fps: f64,
    pub dropped_frames: u64,
    pub total_frames: u64,
    pub frame_stability: f64, // 0.0-1.0, higher is more stable

    // Render performance
    pub avg_render_time: Duration,
    pub max_render_time: Duration,
    pub render_efficiency: f64, // render_time / frame_time

    // Memory performance
    pub current_memory_usage: usize,
    pub peak_memory_usage: usize,
    pub avg_memory_usage: usize,
    pub memory_growth_rate: f64, // bytes per second

    // System performance
    pub reactive_updates_per_second: f64,
    pub event_processing_avg: HashMap<String, Duration>,
    pub component_update_frequency: HashMap<String, u64>,
    pub css_processing_avg: Duration,
    pub layout_computation_avg: Duration,

    // Overall health
    pub performance_score: f64, // 0.0-100.0, higher is better
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub recommendations: Vec<String>,
}

/// Performance bottleneck identification
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub category: BottleneckCategory,
    pub severity: BottleneckSeverity,
    pub description: String,
    pub impact_score: f64, // 0.0-10.0
    pub suggested_fix: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckCategory {
    Rendering,
    Layout,
    CSS,
    Reactive,
    Events,
    Memory,
    Components,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckSeverity {
    Low,
    Medium,
    High,
    Critical,
}

/// Performance report for monitoring and debugging
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceReport {
    #[serde(with = "instant_serde")]
    pub timestamp: std::time::Instant,
    pub uptime: Duration,
    pub metrics: PerformanceMetrics,
    pub system_info: SystemInfo,
    pub recommendations: Vec<String>,
}

// Custom serialization for Instant
mod instant_serde {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

    pub fn serialize<S>(_instant: &Instant, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        // Convert to SystemTime for serialization
        let system_time = SystemTime::now();
        let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap_or(Duration::ZERO);
        duration_since_epoch.serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Instant, D::Error>
    where
        D: Deserializer<'de>,
    {
        // For deserialization, just return current instant
        // This is not perfect but Instant doesn't have a stable epoch
        let _duration = Duration::deserialize(deserializer)?;
        Ok(Instant::now())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemInfo {
    pub terminal_type: String,
    pub color_support: String,
    pub estimated_cpu_usage: f64,
    pub estimated_memory_usage: usize,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            avg_frame_time: Duration::from_millis(16), // 60 FPS default
            max_frame_time: Duration::ZERO,
            min_frame_time: Duration::from_secs(1),
            current_fps: 60.0,
            target_fps: 60.0,
            dropped_frames: 0,
            total_frames: 0,
            frame_stability: 1.0,
            avg_render_time: Duration::ZERO,
            max_render_time: Duration::ZERO,
            render_efficiency: 1.0,
            current_memory_usage: 0,
            peak_memory_usage: 0,
            avg_memory_usage: 0,
            memory_growth_rate: 0.0,
            reactive_updates_per_second: 0.0,
            event_processing_avg: HashMap::new(),
            component_update_frequency: HashMap::new(),
            css_processing_avg: Duration::ZERO,
            layout_computation_avg: Duration::ZERO,
            performance_score: 100.0,
            bottlenecks: Vec::new(),
            recommendations: Vec::new(),
        }
    }
}

impl PerformanceMonitor {
    /// Create a new performance monitor
    pub fn new() -> Self {
        let now = Instant::now();
        Self {
            metrics: Arc::new(Mutex::new(PerformanceMetrics::default())),
            frame_times: VecDeque::with_capacity(120), // 2 seconds at 60fps
            render_times: VecDeque::with_capacity(120),
            memory_usage: VecDeque::with_capacity(60), // 1 second at 60fps
            event_processing_times: HashMap::new(),
            reactive_update_times: VecDeque::with_capacity(120),
            component_update_times: HashMap::new(),
            css_processing_times: VecDeque::with_capacity(120),
            layout_computation_times: VecDeque::with_capacity(120),
            start_time: now,
            last_report: now,
            report_interval: Duration::from_secs(1),
        }
    }

    /// Record frame timing
    pub fn record_frame_time(&mut self, frame_time: Duration, render_time: Duration) {
        // Update sliding windows
        if self.frame_times.len() >= 120 {
            self.frame_times.pop_front();
            self.render_times.pop_front();
        }

        self.frame_times.push_back(frame_time);
        self.render_times.push_back(render_time);

        // Update metrics
        let mut metrics = self.metrics.lock().unwrap();
        metrics.total_frames += 1;

        // Check for dropped frames (>16.67ms for 60fps)
        if frame_time > Duration::from_millis(17) {
            metrics.dropped_frames += 1;
        }

        // Update frame time statistics
        if frame_time > metrics.max_frame_time {
            metrics.max_frame_time = frame_time;
        }
        if frame_time < metrics.min_frame_time {
            metrics.min_frame_time = frame_time;
        }

        // Calculate current FPS
        if !self.frame_times.is_empty() {
            let avg_frame_time: Duration = self.frame_times.iter().sum::<Duration>() / self.frame_times.len() as u32;
            metrics.avg_frame_time = avg_frame_time;
            metrics.current_fps = 1.0 / avg_frame_time.as_secs_f64();
        }

        // Calculate render efficiency
        if !self.render_times.is_empty() {
            let avg_render_time: Duration = self.render_times.iter().sum::<Duration>() / self.render_times.len() as u32;
            metrics.avg_render_time = avg_render_time;
            metrics.render_efficiency = avg_render_time.as_secs_f64() / frame_time.as_secs_f64();
        }

        // Calculate frame stability (lower variance = higher stability)
        if self.frame_times.len() > 10 {
            let avg = metrics.avg_frame_time.as_secs_f64();
            let variance: f64 = self.frame_times.iter()
                .map(|t| (t.as_secs_f64() - avg).powi(2))
                .sum::<f64>() / self.frame_times.len() as f64;
            let std_dev = variance.sqrt();
            metrics.frame_stability = (1.0 - (std_dev / avg)).max(0.0);
        }
    }

    /// Record memory usage
    pub fn record_memory_usage(&mut self, usage: usize) {
        if self.memory_usage.len() >= 60 {
            self.memory_usage.pop_front();
        }
        self.memory_usage.push_back(usage);

        let mut metrics = self.metrics.lock().unwrap();
        metrics.current_memory_usage = usage;

        if usage > metrics.peak_memory_usage {
            metrics.peak_memory_usage = usage;
        }

        if !self.memory_usage.is_empty() {
            metrics.avg_memory_usage = self.memory_usage.iter().sum::<usize>() / self.memory_usage.len();
        }

        // Calculate memory growth rate
        if self.memory_usage.len() > 30 {
            let recent_avg = self.memory_usage.iter().rev().take(10).sum::<usize>() / 10;
            let older_avg = self.memory_usage.iter().take(10).sum::<usize>() / 10;
            let time_diff = Duration::from_secs_f64(20.0 / 60.0); // ~20 frames at 60fps
            metrics.memory_growth_rate = (recent_avg as f64 - older_avg as f64) / time_diff.as_secs_f64();
        }
    }

    /// Record event processing time
    pub fn record_event_processing(&mut self, event_type: &str, processing_time: Duration) {
        let times = self.event_processing_times.entry(event_type.to_string()).or_insert_with(|| VecDeque::with_capacity(30));

        if times.len() >= 30 {
            times.pop_front();
        }
        times.push_back(processing_time);

        // Update metrics
        let mut metrics = self.metrics.lock().unwrap();
        let avg_time = times.iter().sum::<Duration>() / times.len() as u32;
        metrics.event_processing_avg.insert(event_type.to_string(), avg_time);
    }

    /// Record reactive update time
    pub fn record_reactive_update(&mut self, update_time: Duration) {
        if self.reactive_update_times.len() >= 120 {
            self.reactive_update_times.pop_front();
        }
        self.reactive_update_times.push_back(update_time);

        // Calculate updates per second
        let mut metrics = self.metrics.lock().unwrap();
        let recent_updates = self.reactive_update_times.len().min(60); // Last second
        metrics.reactive_updates_per_second = recent_updates as f64;
    }

    /// Record component update
    pub fn record_component_update(&mut self, component_id: &str, update_time: Duration) {
        let times = self.component_update_times.entry(component_id.to_string()).or_insert_with(|| VecDeque::with_capacity(30));

        if times.len() >= 30 {
            times.pop_front();
        }
        times.push_back(update_time);

        // Update frequency counter
        let mut metrics = self.metrics.lock().unwrap();
        *metrics.component_update_frequency.entry(component_id.to_string()).or_insert(0) += 1;
    }

    /// Record CSS processing time
    pub fn record_css_processing(&mut self, processing_time: Duration) {
        if self.css_processing_times.len() >= 120 {
            self.css_processing_times.pop_front();
        }
        self.css_processing_times.push_back(processing_time);

        let mut metrics = self.metrics.lock().unwrap();
        if !self.css_processing_times.is_empty() {
            metrics.css_processing_avg = self.css_processing_times.iter().sum::<Duration>() / self.css_processing_times.len() as u32;
        }
    }

    /// Record layout computation time
    pub fn record_layout_computation(&mut self, computation_time: Duration) {
        if self.layout_computation_times.len() >= 120 {
            self.layout_computation_times.pop_front();
        }
        self.layout_computation_times.push_back(computation_time);

        let mut metrics = self.metrics.lock().unwrap();
        if !self.layout_computation_times.is_empty() {
            metrics.layout_computation_avg = self.layout_computation_times.iter().sum::<Duration>() / self.layout_computation_times.len() as u32;
        }
    }

    /// Get current performance metrics
    pub fn get_metrics(&self) -> PerformanceMetrics {
        let mut metrics = self.metrics.lock().unwrap();

        // Update performance score
        metrics.performance_score = self.calculate_performance_score(&metrics);

        // Identify bottlenecks
        metrics.bottlenecks = self.identify_bottlenecks(&metrics);

        // Generate recommendations
        metrics.recommendations = self.generate_recommendations(&metrics);

        metrics.clone()
    }

    /// Generate comprehensive performance report
    pub fn generate_report(&mut self) -> PerformanceReport {
        let metrics = self.get_metrics();
        let uptime = self.start_time.elapsed();

        PerformanceReport {
            timestamp: Instant::now(),
            uptime,
            metrics,
            system_info: self.get_system_info(),
            recommendations: self.generate_recommendations(&self.metrics.lock().unwrap()),
        }
    }

    /// Calculate overall performance score (0-100)
    fn calculate_performance_score(&self, metrics: &PerformanceMetrics) -> f64 {
        let mut score = 100.0;

        // Frame rate score (40% weight)
        let fps_score = if metrics.current_fps >= metrics.target_fps * 0.95 {
            100.0
        } else if metrics.current_fps >= metrics.target_fps * 0.8 {
            80.0
        } else if metrics.current_fps >= metrics.target_fps * 0.6 {
            60.0
        } else {
            40.0
        };
        score = score * 0.4 + fps_score * 0.4;

        // Frame stability score (20% weight)
        let stability_score = metrics.frame_stability * 100.0;
        score = score * 0.8 + stability_score * 0.2;

        // Memory efficiency score (20% weight)
        let memory_score = if metrics.memory_growth_rate <= 0.0 {
            100.0
        } else if metrics.memory_growth_rate < 1024.0 { // < 1KB/s growth
            80.0
        } else if metrics.memory_growth_rate < 10240.0 { // < 10KB/s growth
            60.0
        } else {
            40.0
        };
        score = score * 0.8 + memory_score * 0.2;

        // Render efficiency score (20% weight)
        let render_score = if metrics.render_efficiency <= 0.5 {
            100.0
        } else if metrics.render_efficiency <= 0.7 {
            80.0
        } else if metrics.render_efficiency <= 0.9 {
            60.0
        } else {
            40.0
        };
        score = score * 0.8 + render_score * 0.2;

        // Safely normalize score to 0.0-100.0 range, handling NaN
        match score {
            s if s.is_nan() => 0.0,
            s if s < 0.0 => 0.0,
            s if s > 100.0 => 100.0,
            s => s,
        }
    }

    /// Identify performance bottlenecks
    fn identify_bottlenecks(&self, metrics: &PerformanceMetrics) -> Vec<PerformanceBottleneck> {
        let mut bottlenecks = Vec::new();

        // Check frame rate issues
        if metrics.current_fps < metrics.target_fps * 0.8 {
            bottlenecks.push(PerformanceBottleneck {
                category: BottleneckCategory::Rendering,
                severity: if metrics.current_fps < metrics.target_fps * 0.5 {
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::High
                },
                description: format!("Low frame rate: {:.1} FPS (target: {:.1})", metrics.current_fps, metrics.target_fps),
                impact_score: (metrics.target_fps - metrics.current_fps) / metrics.target_fps * 10.0,
                suggested_fix: "Consider reducing visual complexity or optimizing render pipeline".to_string(),
            });
        }

        // Check memory growth
        if metrics.memory_growth_rate > 10240.0 { // > 10KB/s
            bottlenecks.push(PerformanceBottleneck {
                category: BottleneckCategory::Memory,
                severity: if metrics.memory_growth_rate > 102400.0 { // > 100KB/s
                    BottleneckSeverity::Critical
                } else {
                    BottleneckSeverity::High
                },
                description: format!("Memory leak detected: {:.1} KB/s growth", metrics.memory_growth_rate / 1024.0),
                impact_score: (metrics.memory_growth_rate / 10240.0).min(10.0),
                suggested_fix: "Check for unreleased resources or circular references".to_string(),
            });
        }

        // Check render efficiency
        if metrics.render_efficiency > 0.8 {
            bottlenecks.push(PerformanceBottleneck {
                category: BottleneckCategory::Rendering,
                severity: BottleneckSeverity::Medium,
                description: format!("High render time: {:.1}% of frame time", metrics.render_efficiency * 100.0),
                impact_score: metrics.render_efficiency * 5.0,
                suggested_fix: "Optimize rendering operations or reduce visual complexity".to_string(),
            });
        }

        // Check CSS processing time
        if metrics.css_processing_avg > Duration::from_millis(2) {
            bottlenecks.push(PerformanceBottleneck {
                category: BottleneckCategory::CSS,
                severity: BottleneckSeverity::Medium,
                description: format!("Slow CSS processing: {:.1}ms", metrics.css_processing_avg.as_secs_f64() * 1000.0),
                impact_score: (metrics.css_processing_avg.as_millis() as f64 / 2.0).min(10.0),
                suggested_fix: "Optimize CSS selectors or reduce style complexity".to_string(),
            });
        }

        // Check layout computation time
        if metrics.layout_computation_avg > Duration::from_millis(3) {
            bottlenecks.push(PerformanceBottleneck {
                category: BottleneckCategory::Layout,
                severity: BottleneckSeverity::Medium,
                description: format!("Slow layout computation: {:.1}ms", metrics.layout_computation_avg.as_secs_f64() * 1000.0),
                impact_score: (metrics.layout_computation_avg.as_millis() as f64 / 3.0).min(10.0),
                suggested_fix: "Simplify layout structure or cache layout results".to_string(),
            });
        }

        bottlenecks
    }

    /// Generate performance recommendations
    fn generate_recommendations(&self, metrics: &PerformanceMetrics) -> Vec<String> {
        let mut recommendations = Vec::new();

        if metrics.current_fps < metrics.target_fps * 0.9 {
            recommendations.push("Consider reducing target FPS or optimizing render pipeline".to_string());
        }

        if metrics.frame_stability < 0.8 {
            recommendations.push("Frame timing is unstable - check for blocking operations".to_string());
        }

        if metrics.memory_growth_rate > 1024.0 {
            recommendations.push("Monitor memory usage - potential leak detected".to_string());
        }

        if metrics.reactive_updates_per_second > 30.0 {
            recommendations.push("High reactive update frequency - consider batching updates".to_string());
        }

        if metrics.render_efficiency > 0.7 {
            recommendations.push("Rendering takes significant portion of frame time - optimize render operations".to_string());
        }

        if recommendations.is_empty() {
            recommendations.push("Performance is optimal".to_string());
        }

        recommendations
    }

    /// Get system information
    fn get_system_info(&self) -> SystemInfo {
        SystemInfo {
            terminal_type: std::env::var("TERM").unwrap_or_else(|_| "unknown".to_string()),
            color_support: if std::env::var("COLORTERM").is_ok() {
                "truecolor".to_string()
            } else {
                "basic".to_string()
            },
            estimated_cpu_usage: self.estimate_cpu_usage(),
            estimated_memory_usage: self.metrics.lock().unwrap().current_memory_usage,
        }
    }

    /// Estimate CPU usage based on performance metrics
    fn estimate_cpu_usage(&self) -> f64 {
        let metrics = self.metrics.lock().unwrap();

        // Rough estimation based on render efficiency and frame stability
        let base_usage = metrics.render_efficiency * 50.0; // Rendering load
        let instability_penalty = (1.0 - metrics.frame_stability) * 20.0; // Instability suggests high CPU usage

        (base_usage + instability_penalty).min(100.0)
    }

    /// Check if it's time to generate a report
    pub fn should_generate_report(&self) -> bool {
        self.last_report.elapsed() >= self.report_interval
    }

    /// Reset report timer
    pub fn reset_report_timer(&mut self) {
        self.last_report = Instant::now();
    }

    /// Set report interval
    pub fn set_report_interval(&mut self, interval: Duration) {
        self.report_interval = interval;
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}
