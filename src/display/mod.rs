//! Display capabilities detection and adaptive refresh rate management
//!
//! This module provides intelligent FPS adaptation based on terminal capabilities,
//! system performance, and user preferences. Unlike GUI applications that can query
//! monitor refresh rates directly, terminal applications must use heuristics and
//! runtime performance monitoring.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::time::{Duration, Instant};

/// Display capability information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayCapabilities {
  /// Detected maximum viable FPS for this terminal/system
  pub max_fps: u32,
  /// Recommended FPS for optimal performance/quality balance
  pub recommended_fps: u32,
  /// Terminal type and capabilities
  pub terminal_info: TerminalInfo,
  /// System performance characteristics
  pub performance_profile: PerformanceProfile,
  /// VSync-like capabilities (terminal responsiveness)
  pub sync_capabilities: SyncCapabilities,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TerminalInfo {
  /// Terminal program name (if detectable)
  pub program: Option<String>,
  /// Terminal supports high refresh rates
  pub supports_high_refresh: bool,
  /// Terminal has hardware acceleration
  pub has_gpu_acceleration: bool,
  /// Connection type (local, SSH, etc.)
  pub connection_type: ConnectionType,
  /// Color depth capabilities
  pub color_depth: ColorDepth,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConnectionType {
  Local,
  SSH,
  Tmux,
  Screen,
  Web,
  Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ColorDepth {
  Monochrome,
  Color16,
  Color256,
  TrueColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceProfile {
  /// Average render time in microseconds
  pub avg_render_time_us: f32,
  /// System can handle high FPS without dropping frames
  pub high_fps_capable: bool,
  /// CPU usage characteristics
  pub cpu_efficiency: f32,
  /// Memory usage efficiency
  pub memory_efficiency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyncCapabilities {
  /// Terminal can handle rapid updates smoothly
  pub smooth_updates: bool,
  /// Effective maximum update rate (measured)
  pub max_update_rate: u32,
  /// Input lag characteristics
  pub input_latency_ms: f32,
}

/// Adaptive FPS manager that automatically adjusts refresh rate
pub struct AdaptiveFpsManager {
  /// Current target FPS
  target_fps: u32,
  /// Detected display capabilities
  capabilities: DisplayCapabilities,
  /// Performance monitoring
  performance_monitor: PerformanceMonitor,
  /// Adaptive settings
  config: AdaptiveConfig,
}

#[derive(Debug, Clone)]
pub struct AdaptiveConfig {
  /// Allow automatic FPS adjustment
  pub auto_adapt: bool,
  /// Minimum FPS to maintain
  pub min_fps: u32,
  /// Maximum FPS to attempt
  pub max_fps: u32,
  /// Performance vs quality preference (0.0 = performance, 1.0 = quality)
  pub quality_preference: f32,
  /// Enable power-saving mode
  pub power_save: bool,
}

impl Default for AdaptiveConfig {
  fn default() -> Self {
    Self {
      auto_adapt: true,
      min_fps: 30,
      max_fps: 144,
      quality_preference: 0.7, // Prefer quality but not at extreme cost
      power_save: false,
    }
  }
}

/// Real-time performance monitoring
struct PerformanceMonitor {
  frame_times: VecDeque<Duration>,
  render_times: VecDeque<Duration>,
  dropped_frames: u64,
  total_frames: u64,
  last_adjustment: Instant,
  adjustment_cooldown: Duration,
}

impl PerformanceMonitor {
  fn new() -> Self {
    Self {
      frame_times: VecDeque::with_capacity(120), // 2 seconds at 60fps
      render_times: VecDeque::with_capacity(120),
      dropped_frames: 0,
      total_frames: 0,
      last_adjustment: Instant::now(),
      adjustment_cooldown: Duration::from_secs(2),
    }
  }

  fn record_frame(&mut self, frame_time: Duration, render_time: Duration, dropped: bool) {
    // Keep sliding window of recent performance
    if self.frame_times.len() >= 120 {
      self.frame_times.pop_front();
      self.render_times.pop_front();
    }

    self.frame_times.push_back(frame_time);
    self.render_times.push_back(render_time);
    self.total_frames += 1;

    if dropped {
      self.dropped_frames += 1;
    }
  }

  fn get_current_performance(&self) -> PerformanceMetrics {
    if self.frame_times.is_empty() {
      return PerformanceMetrics::default();
    }

    let avg_frame_time =
      self.frame_times.iter().sum::<Duration>().as_secs_f32() / self.frame_times.len() as f32;
    let avg_render_time =
      self.render_times.iter().sum::<Duration>().as_secs_f32() / self.render_times.len() as f32;

    let current_fps = 1.0 / avg_frame_time;
    let drop_rate = self.dropped_frames as f32 / self.total_frames as f32;

    PerformanceMetrics {
      current_fps,
      avg_render_time_ms: avg_render_time * 1000.0,
      drop_rate_percent: drop_rate * 100.0,
      is_stable: drop_rate < 0.05, // Less than 5% drops is stable
    }
  }

  fn can_adjust(&self) -> bool {
    self.last_adjustment.elapsed() > self.adjustment_cooldown
  }
}

#[derive(Debug, Default)]
pub struct PerformanceMetrics {
  pub current_fps: f32,
  pub avg_render_time_ms: f32,
  pub drop_rate_percent: f32,
  pub is_stable: bool,
}

impl AdaptiveFpsManager {
  /// Create a new adaptive FPS manager with automatic capability detection
  pub async fn new() -> crate::error::Result<Self> {
    let capabilities = Self::detect_display_capabilities().await?;
    let recommended_fps = Self::calculate_recommended_fps(&capabilities);

    Ok(Self {
      target_fps: recommended_fps,
      capabilities,
      performance_monitor: PerformanceMonitor::new(),
      config: AdaptiveConfig::default(),
    })
  }

  /// Create with custom configuration
  pub async fn with_config(config: AdaptiveConfig) -> crate::error::Result<Self> {
    let capabilities = Self::detect_display_capabilities().await?;
    let recommended_fps =
      Self::calculate_recommended_fps(&capabilities).clamp(config.min_fps, config.max_fps);

    Ok(Self {
      target_fps: recommended_fps,
      capabilities,
      performance_monitor: PerformanceMonitor::new(),
      config,
    })
  }

  /// Detect terminal and system display capabilities
  async fn detect_display_capabilities() -> crate::error::Result<DisplayCapabilities> {
    let terminal_info = Self::detect_terminal_info();
    let performance_profile = Self::benchmark_performance().await?;
    let sync_capabilities = Self::test_sync_capabilities().await?;

    // Calculate maximum viable FPS based on all factors
    let max_fps = Self::calculate_max_fps(&terminal_info, &performance_profile, &sync_capabilities);
    let recommended_fps = Self::calculate_recommended_from_max(max_fps, &terminal_info);

    Ok(DisplayCapabilities {
      max_fps,
      recommended_fps,
      terminal_info,
      performance_profile,
      sync_capabilities,
    })
  }

  /// Detect terminal program and capabilities
  fn detect_terminal_info() -> TerminalInfo {
    let program = std::env::var("TERM_PROGRAM")
      .ok()
      .or_else(|| std::env::var("TERMINAL_EMULATOR").ok());

    let connection_type = Self::detect_connection_type();
    let supports_high_refresh = Self::terminal_supports_high_refresh(&program, &connection_type);
    let has_gpu_acceleration = Self::detect_gpu_acceleration(&program);
    let color_depth = Self::detect_color_depth();

    TerminalInfo {
      program,
      supports_high_refresh,
      has_gpu_acceleration,
      connection_type,
      color_depth,
    }
  }

  fn detect_connection_type() -> ConnectionType {
    if std::env::var("SSH_CLIENT").is_ok() || std::env::var("SSH_TTY").is_ok() {
      ConnectionType::SSH
    } else if std::env::var("TMUX").is_ok() {
      ConnectionType::Tmux
    } else if std::env::var("STY").is_ok() {
      ConnectionType::Screen
    } else if std::env::var("TERM").is_ok_and(|t| t.contains("web")) {
      ConnectionType::Web
    } else {
      ConnectionType::Local
    }
  }

  fn terminal_supports_high_refresh(program: &Option<String>, connection: &ConnectionType) -> bool {
    match connection {
      ConnectionType::SSH => return false, // Network latency limits high refresh
      ConnectionType::Web => return false, // Browser limitations
      _ => {}
    }

    if let Some(prog) = program {
      match prog.as_str() {
        "iTerm.app" => true,        // Excellent performance
        "WezTerm" => true,          // GPU accelerated
        "Alacritty" => true,        // High performance
        "kitty" => true,            // GPU accelerated
        "Windows Terminal" => true, // Modern Windows terminal
        "Hyper" => false,           // Electron-based, slower
        "Terminal.app" => false,    // Basic macOS terminal
        _ => true,                  // Assume modern terminals can handle it
      }
    } else {
      true // Unknown terminal, optimistically assume capable
    }
  }

  fn detect_gpu_acceleration(program: &Option<String>) -> bool {
    if let Some(prog) = program {
      matches!(prog.as_str(), "WezTerm" | "Alacritty" | "kitty")
    } else {
      false
    }
  }

  fn detect_color_depth() -> ColorDepth {
    if let Ok(colorterm) = std::env::var("COLORTERM") {
      if colorterm.contains("truecolor") || colorterm.contains("24bit") {
        return ColorDepth::TrueColor;
      }
    }

    if let Ok(term) = std::env::var("TERM") {
      if term.contains("256") {
        return ColorDepth::Color256;
      } else if term.contains("color") {
        return ColorDepth::Color16;
      }
    }

    ColorDepth::Color16 // Conservative default
  }

  /// Benchmark system performance capabilities
  async fn benchmark_performance() -> crate::error::Result<PerformanceProfile> {
    // Quick performance test - render simple layout multiple times
    let mut renderer = crate::rendering::Renderer::new()?;
    let test_layout = Self::create_benchmark_layout();

    let mut render_times = Vec::new();
    let benchmark_frames = 30;

    for _ in 0..benchmark_frames {
      // Use offscreen render to avoid emitting terminal clear during benchmarks
      // This prevents visible flicker at startup when adaptive detection runs.
      let _ = renderer.render_offscreen(&test_layout).await?;

      let start = Instant::now();
      let _bytes = renderer.render_offscreen(&test_layout).await?;
      render_times.push(start.elapsed());
    }

    let avg_render_time_us = render_times
      .iter()
      .map(|d| d.as_micros() as f32)
      .sum::<f32>()
      / render_times.len() as f32;

    // Determine capabilities based on render performance
    let high_fps_capable = avg_render_time_us < 2000.0; // Sub-2ms = high FPS capable
    let cpu_efficiency = (2000.0 / avg_render_time_us.max(100.0)).min(10.0) / 10.0;
    let memory_efficiency = 0.8; // Estimate - our double buffering is quite efficient

    Ok(PerformanceProfile {
      avg_render_time_us,
      high_fps_capable,
      cpu_efficiency,
      memory_efficiency,
    })
  }

  fn create_benchmark_layout() -> crate::layout::Layout {
    use crate::layout::{ComputedStyles, Layout, LayoutRect};

    Layout {
      rect: LayoutRect {
        x: 0,
        y: 0,
        width: 80,
        height: 24,
      },
      tag: "benchmark".to_string(),
      content: Some("Performance Test".to_string()),
      children: (0..10)
        .map(|i| Layout {
          rect: LayoutRect {
            x: i * 8,
            y: 2,
            width: 7,
            height: 1,
          },
          tag: format!("item-{i}"),
          content: Some(format!("Test{i}")),
          children: vec![],
          focused: false,
          element_id: Some(format!("item-{i}")),
          focusable: false,
          styles: ComputedStyles::default(),
        })
        .collect(),
      focused: false,
      element_id: Some("benchmark".to_string()),
      focusable: false,
      styles: ComputedStyles::default(),
    }
  }

  /// Test terminal synchronization capabilities
  async fn test_sync_capabilities() -> crate::error::Result<SyncCapabilities> {
    // Test rapid updates to see how terminal handles them
    let mut renderer = crate::rendering::Renderer::new()?;
    let test_layout = Self::create_benchmark_layout();

    let test_start = Instant::now();
    let rapid_updates = 60; // Test 60 rapid updates

    for _ in 0..rapid_updates {
      let _bytes = renderer.render_offscreen(&test_layout).await?;
    }

    let elapsed = test_start.elapsed();
    let effective_rate = rapid_updates as f32 / elapsed.as_secs_f32();

    Ok(SyncCapabilities {
      smooth_updates: effective_rate > 30.0, // Can handle >30fps smoothly
      max_update_rate: effective_rate as u32,
      input_latency_ms: 16.0, // Estimate - difficult to measure in terminal
    })
  }

  fn calculate_max_fps(
    terminal: &TerminalInfo,
    performance: &PerformanceProfile,
    sync: &SyncCapabilities,
  ) -> u32 {
    let mut max_fps = 240u32; // Start optimistically

    // Reduce based on connection type
    match terminal.connection_type {
      ConnectionType::SSH => max_fps = max_fps.min(60),
      ConnectionType::Web => max_fps = max_fps.min(60),
      ConnectionType::Tmux | ConnectionType::Screen => max_fps = max_fps.min(90),
      _ => {}
    }

    // Reduce based on terminal capabilities
    if !terminal.supports_high_refresh {
      max_fps = max_fps.min(60);
    }

    // Reduce based on performance
    if !performance.high_fps_capable {
      max_fps = max_fps.min(60);
    } else if performance.avg_render_time_us > 1000.0 {
      max_fps = max_fps.min(90);
    }

    // Limit by terminal sync capabilities
    max_fps = max_fps.min(sync.max_update_rate * 2); // 2x headroom

    max_fps.max(30) // Never go below 30fps
  }

  fn calculate_recommended_from_max(max_fps: u32, terminal: &TerminalInfo) -> u32 {
    match max_fps {
      240.. => {
        if terminal.has_gpu_acceleration {
          144
        } else {
          120
        }
      }
      120..240 => 90,
      60..120 => 60,
      _ => 30,
    }
  }

  fn calculate_recommended_fps(capabilities: &DisplayCapabilities) -> u32 {
    capabilities.recommended_fps
  }

  /// Get current target FPS
  pub fn get_target_fps(&self) -> u32 {
    self.target_fps
  }

  /// Get target frame duration
  pub fn get_frame_duration(&self) -> Duration {
    Duration::from_nanos(1_000_000_000 / self.target_fps as u64)
  }

  /// Record frame performance and potentially adjust FPS
  pub fn record_frame_performance(
    &mut self,
    frame_time: Duration,
    render_time: Duration,
    dropped: bool,
  ) {
    self
      .performance_monitor
      .record_frame(frame_time, render_time, dropped);

    if self.config.auto_adapt && self.performance_monitor.can_adjust() {
      self.adaptive_adjustment();
    }
  }

  /// Perform adaptive FPS adjustment based on performance
  fn adaptive_adjustment(&mut self) {
    let metrics = self.performance_monitor.get_current_performance();

    // Adjustment logic
    if !metrics.is_stable && metrics.drop_rate_percent > 10.0 {
      // Too many dropped frames - reduce FPS
      let new_fps = (self.target_fps as f32 * 0.8) as u32;
      self.target_fps = new_fps.clamp(self.config.min_fps, self.target_fps);
    } else if metrics.is_stable
      && metrics.avg_render_time_ms < 5.0
      && self.target_fps < self.config.max_fps
    {
      // Very stable and fast - try increasing FPS
      let new_fps = (self.target_fps as f32 * 1.2) as u32;
      self.target_fps = new_fps.min(self.config.max_fps);
    }

    self.performance_monitor.last_adjustment = Instant::now();
  }

  /// Manually set target FPS (disables auto-adaptation)
  pub fn set_target_fps(&mut self, fps: u32) {
    self.target_fps = fps.clamp(self.config.min_fps, self.config.max_fps);
    self.config.auto_adapt = false;
  }

  /// Get display capabilities
  pub fn get_capabilities(&self) -> &DisplayCapabilities {
    &self.capabilities
  }

  /// Get current performance metrics
  pub fn get_performance_metrics(&self) -> PerformanceMetrics {
    self.performance_monitor.get_current_performance()
  }

  /// Enable/disable auto-adaptation
  pub fn set_auto_adapt(&mut self, enabled: bool) {
    self.config.auto_adapt = enabled;
  }

  /// Get recommendation summary for user
  pub fn get_recommendation_summary(&self) -> String {
    format!(
      "Display Analysis:\n  \
            Terminal: {} ({})\n  \
            Connection: {:?}\n  \
            Max FPS: {}\n  \
            Recommended: {} FPS\n  \
            Current Target: {} FPS\n  \
            GPU Acceleration: {}\n  \
            High Refresh Support: {}",
      self
        .capabilities
        .terminal_info
        .program
        .as_deref()
        .unwrap_or("Unknown"),
      match self.capabilities.terminal_info.color_depth {
        ColorDepth::TrueColor => "TrueColor",
        ColorDepth::Color256 => "256 Color",
        ColorDepth::Color16 => "16 Color",
        ColorDepth::Monochrome => "Monochrome",
      },
      self.capabilities.terminal_info.connection_type,
      self.capabilities.max_fps,
      self.capabilities.recommended_fps,
      self.target_fps,
      if self.capabilities.terminal_info.has_gpu_acceleration {
        "Yes"
      } else {
        "No"
      },
      if self.capabilities.terminal_info.supports_high_refresh {
        "Yes"
      } else {
        "No"
      }
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[tokio::test]
  async fn test_adaptive_fps_manager_creation() {
    // Create a mock manager without actually rendering to the terminal
    let capabilities = DisplayCapabilities {
      max_fps: 120,
      recommended_fps: 60,
      terminal_info: TerminalInfo {
        program: None,
        supports_high_refresh: false,
        has_gpu_acceleration: false,
        connection_type: ConnectionType::Local,
        color_depth: ColorDepth::TrueColor,
      },
      performance_profile: PerformanceProfile {
        avg_render_time_us: 1000.0,
        high_fps_capable: true,
        cpu_efficiency: 0.8,
        memory_efficiency: 0.9,
      },
      sync_capabilities: SyncCapabilities {
        smooth_updates: true,
        max_update_rate: 60,
        input_latency_ms: 16.0,
      },
    };

    let manager = AdaptiveFpsManager {
      target_fps: 60,
      capabilities,
      performance_monitor: PerformanceMonitor::new(),
      config: AdaptiveConfig::default(),
    };

    assert!(manager.get_target_fps() >= 30);
    assert!(manager.get_target_fps() <= 240);
  }

  #[test]
  fn test_terminal_detection() {
    let info = AdaptiveFpsManager::detect_terminal_info();
    // Should not panic and should provide reasonable defaults
    assert!(matches!(
      info.connection_type,
      ConnectionType::Local | ConnectionType::SSH | ConnectionType::Unknown
    ));
  }

  #[test]
  fn test_frame_duration_calculation() {
    let config = AdaptiveConfig::default();
    let manager = AdaptiveFpsManager {
      target_fps: 60,
      capabilities: DisplayCapabilities {
        max_fps: 120,
        recommended_fps: 60,
        terminal_info: TerminalInfo {
          program: None,
          supports_high_refresh: true,
          has_gpu_acceleration: false,
          connection_type: ConnectionType::Local,
          color_depth: ColorDepth::TrueColor,
        },
        performance_profile: PerformanceProfile {
          avg_render_time_us: 1000.0,
          high_fps_capable: true,
          cpu_efficiency: 0.8,
          memory_efficiency: 0.9,
        },
        sync_capabilities: SyncCapabilities {
          smooth_updates: true,
          max_update_rate: 120,
          input_latency_ms: 16.0,
        },
      },
      performance_monitor: PerformanceMonitor::new(),
      config,
    };

    let frame_duration = manager.get_frame_duration();
    let expected = Duration::from_nanos(1_000_000_000 / 60);
    assert_eq!(frame_duration, expected);
  }
}
