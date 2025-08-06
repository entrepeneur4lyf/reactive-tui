//! Performance monitoring system for the Interactive Widget Guide

use std::time::{Duration, Instant};
use std::collections::VecDeque;

/// Performance monitor for tracking framerate and other metrics
pub struct PerformanceMonitor {
    frame_times: VecDeque<Duration>,
    last_frame_time: Instant,
    target_fps: f64,
    max_samples: usize,
}

impl PerformanceMonitor {
    pub fn new() -> Self {
        Self {
            frame_times: VecDeque::new(),
            last_frame_time: Instant::now(),
            target_fps: 60.0,
            max_samples: 60, // Keep last 60 frame times for averaging
        }
    }

    /// Record a frame render time
    pub fn record_frame(&mut self) {
        let now = Instant::now();
        let frame_time = now.duration_since(self.last_frame_time);
        
        self.frame_times.push_back(frame_time);
        
        // Keep only the last max_samples frame times
        if self.frame_times.len() > self.max_samples {
            self.frame_times.pop_front();
        }
        
        self.last_frame_time = now;
    }

    /// Get the current framerate
    pub fn get_framerate(&self) -> f64 {
        if self.frame_times.is_empty() {
            return 0.0;
        }

        let total_time: Duration = self.frame_times.iter().sum();
        let average_frame_time = total_time / self.frame_times.len() as u32;
        
        if average_frame_time.as_secs_f64() > 0.0 {
            1.0 / average_frame_time.as_secs_f64()
        } else {
            0.0
        }
    }

    /// Get the target framerate
    pub fn get_target_fps(&self) -> f64 {
        self.target_fps
    }

    /// Set the target framerate
    pub fn set_target_fps(&mut self, fps: f64) {
        self.target_fps = fps;
    }

    /// Get performance statistics
    pub fn get_stats(&self) -> PerformanceStats {
        let current_fps = self.get_framerate();
        let frame_time_ms = if current_fps > 0.0 {
            1000.0 / current_fps
        } else {
            0.0
        };

        PerformanceStats {
            current_fps,
            target_fps: self.target_fps,
            frame_time_ms,
            frame_count: self.frame_times.len(),
            performance_ratio: if self.target_fps > 0.0 {
                current_fps / self.target_fps
            } else {
                0.0
            },
        }
    }

    /// Check if performance is meeting targets
    pub fn is_performance_good(&self) -> bool {
        let current_fps = self.get_framerate();
        current_fps >= self.target_fps * 0.9 // Within 90% of target
    }

    /// Get a formatted framerate string for display
    pub fn get_framerate_display(&self) -> String {
        let fps = self.get_framerate();
        if fps >= self.target_fps * 0.9 {
            format!("🟢 {:.1} FPS", fps)
        } else if fps >= self.target_fps * 0.7 {
            format!("🟡 {:.1} FPS", fps)
        } else {
            format!("🔴 {:.1} FPS", fps)
        }
    }
}

impl Default for PerformanceMonitor {
    fn default() -> Self {
        Self::new()
    }
}

/// Performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub current_fps: f64,
    pub target_fps: f64,
    pub frame_time_ms: f64,
    pub frame_count: usize,
    pub performance_ratio: f64,
}

impl PerformanceStats {
    /// Get a performance grade (A-F)
    pub fn get_grade(&self) -> char {
        if self.performance_ratio >= 0.95 {
            'A'
        } else if self.performance_ratio >= 0.85 {
            'B'
        } else if self.performance_ratio >= 0.75 {
            'C'
        } else if self.performance_ratio >= 0.65 {
            'D'
        } else {
            'F'
        }
    }

    /// Get a color indicator for performance
    pub fn get_color_indicator(&self) -> &'static str {
        match self.get_grade() {
            'A' | 'B' => "green",
            'C' => "yellow",
            'D' | 'F' => "red",
            _ => "gray",
        }
    }

    /// Get a detailed performance report
    pub fn get_report(&self) -> String {
        format!(
            "FPS: {:.1}/{:.1} | Frame Time: {:.1}ms | Grade: {} | Samples: {}",
            self.current_fps,
            self.target_fps,
            self.frame_time_ms,
            self.get_grade(),
            self.frame_count
        )
    }
}
