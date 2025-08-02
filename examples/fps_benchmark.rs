//! FPS Benchmark Test
//!
//! This benchmark tests the maximum refresh rate capabilities of the double-buffered
//! TUI rendering system. It measures performance at various target frame rates to
//! determine the practical limits of flicker-free rendering.

use std::time::{Duration, Instant};
use tui_core::{
    layout::{ComputedStyles, Layout, LayoutRect},
    rendering::Renderer,
};

/// FPS benchmark configuration
#[derive(Debug, Clone)]
struct BenchmarkConfig {
    target_fps: u32,
    duration_seconds: u32,
    complexity_level: ComplexityLevel,
}

#[derive(Debug, Clone)]
enum ComplexityLevel {
    Simple,  // Basic layout with minimal elements
    Medium,  // Moderate number of elements with some animation
    Complex, // Heavy layout with many animated elements
    Extreme, // Maximum stress test with hundreds of elements
}

/// FPS benchmark results
#[derive(Debug)]
struct BenchmarkResults {
    target_fps: u32,
    actual_avg_fps: f32,
    actual_max_fps: f32,
    actual_min_fps: f32,
    frame_count: u64,
    dropped_frames: u64,
    avg_render_time_us: f32,
    max_render_time_us: u64,
    buffer_efficiency: f32,
}

/// High-performance FPS benchmark
struct FpsBenchmark {
    renderer: Renderer,
    config: BenchmarkConfig,
    start_time: Instant,
    frame_times: Vec<Duration>,
    render_times: Vec<Duration>,
    frame_count: u64,
}

impl FpsBenchmark {
    fn new(config: BenchmarkConfig) -> tui_core::error::Result<Self> {
        Ok(Self {
            renderer: Renderer::new()?,
            config,
            start_time: Instant::now(),
            frame_times: Vec::with_capacity(10000),
            render_times: Vec::with_capacity(10000),
            frame_count: 0,
        })
    }

    /// Create layout based on complexity level
    fn create_benchmark_layout(&self) -> Layout {
        match self.config.complexity_level {
            ComplexityLevel::Simple => self.create_simple_layout(),
            ComplexityLevel::Medium => self.create_medium_layout(),
            ComplexityLevel::Complex => self.create_complex_layout(),
            ComplexityLevel::Extreme => self.create_extreme_layout(),
        }
    }

    /// Simple layout with minimal elements
    fn create_simple_layout(&self) -> Layout {
        Layout {
            rect: LayoutRect {
                x: 0,
                y: 0,
                width: 80,
                height: 24,
            },
            tag: "simple".to_string(),
            content: Some(format!(
                "FPS Benchmark - Frame: {} | Target: {}fps",
                self.frame_count, self.config.target_fps
            )),
            children: vec![Layout {
                rect: LayoutRect {
                    x: 10,
                    y: 5,
                    width: 60,
                    height: 10,
                },
                tag: "content".to_string(),
                content: Some("Simple rendering test with minimal complexity".to_string()),
                children: vec![],
                focused: false,
                element_id: Some("content".to_string()),
                focusable: false,
                styles: ComputedStyles::default(),
            }],
            focused: false,
            element_id: Some("simple".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        }
    }

    /// Medium complexity layout
    fn create_medium_layout(&self) -> Layout {
        let mut main_layout = Layout {
            rect: LayoutRect {
                x: 0,
                y: 0,
                width: 80,
                height: 24,
            },
            tag: "medium".to_string(),
            content: Some(format!(
                "FPS Benchmark - Frame: {} | Target: {}fps",
                self.frame_count, self.config.target_fps
            )),
            children: vec![],
            focused: false,
            element_id: Some("medium".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        };

        // Add 20 animated elements
        for i in 0..20 {
            let x = ((self.frame_count as f32 * 0.1 + i as f32).sin() * 30.0 + 40.0) as u16;
            let child = Layout {
                rect: LayoutRect {
                    x: x.min(70),
                    y: 2 + i,
                    width: 8,
                    height: 1,
                },
                tag: format!("item-{i}"),
                content: Some(format!("Item{i}")),
                children: vec![],
                focused: i as u64 == (self.frame_count / 5) % 20,
                element_id: Some(format!("item-{i}")),
                focusable: true,
                styles: ComputedStyles::default(),
            };
            main_layout.children.push(child);
        }

        main_layout
    }

    /// Complex layout with many elements
    fn create_complex_layout(&self) -> Layout {
        let mut main_layout = Layout {
            rect: LayoutRect {
                x: 0,
                y: 0,
                width: 120,
                height: 40,
            },
            tag: "complex".to_string(),
            content: Some(format!(
                "FPS Benchmark - Frame: {} | Target: {}fps | Buffer: {} bytes",
                self.frame_count,
                self.config.target_fps,
                self.renderer.get_buffer_size()
            )),
            children: vec![],
            focused: false,
            element_id: Some("complex".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        };

        // Add 100 elements with various animations
        for i in 0..100 {
            let row = i / 10;
            let col = i % 10;
            let phase = self.frame_count as f32 * 0.05 + i as f32 * 0.1;
            let x = (col * 11) as u16 + ((phase.sin() * 3.0) as i16).clamp(-3, 3) as u16;
            let y = (row * 3 + 2) as u16 + ((phase.cos() * 1.0) as i16).clamp(-1, 1) as u16;

            let child = Layout {
                rect: LayoutRect {
                    x,
                    y,
                    width: 10,
                    height: 2,
                },
                tag: format!("complex-{i}"),
                content: Some(format!("C{i:02}")),
                children: vec![],
                focused: i as u64 == (self.frame_count / 3) % 100,
                element_id: Some(format!("complex-{i}")),
                focusable: true,
                styles: ComputedStyles::default(),
            };
            main_layout.children.push(child);
        }

        main_layout
    }

    /// Extreme stress test layout
    fn create_extreme_layout(&self) -> Layout {
        let mut main_layout = Layout {
            rect: LayoutRect {
                x: 0,
                y: 0,
                width: 200,
                height: 60,
            },
            tag: "extreme".to_string(),
            content: Some(format!(
                "EXTREME FPS TEST - Frame: {} | Target: {}fps | Elements: 500",
                self.frame_count, self.config.target_fps
            )),
            children: vec![],
            focused: false,
            element_id: Some("extreme".to_string()),
            focusable: false,
            styles: ComputedStyles::default(),
        };

        // Add 500 animated elements for maximum stress
        for i in 0..500 {
            let row = i / 25;
            let col = i % 25;
            let phase1 = self.frame_count as f32 * 0.08 + i as f32 * 0.02;
            let phase2 = self.frame_count as f32 * 0.05 + i as f32 * 0.03;

            let x = (col * 8) as u16 + ((phase1.sin() * 4.0) as i16).clamp(-3, 3) as u16;
            let y = (row * 2 + 3) as u16 + ((phase2.cos() * 2.0) as i16).clamp(-1, 1) as u16;

            let child = Layout {
                rect: LayoutRect {
                    x: x.min(190),
                    y: y.min(55),
                    width: 7,
                    height: 1,
                },
                tag: format!("extreme-{i}"),
                content: Some(format!("{i:03}")),
                children: vec![],
                focused: i as u64 == (self.frame_count / 2) % 500,
                element_id: Some(format!("extreme-{i}")),
                focusable: false,
                styles: ComputedStyles::default(),
            };
            main_layout.children.push(child);
        }

        main_layout
    }

    /// Run benchmark at target FPS
    async fn run_benchmark(&mut self) -> BenchmarkResults {
        let target_frame_time = Duration::from_nanos(1_000_000_000 / self.config.target_fps as u64);
        let total_frames = self.config.target_fps * self.config.duration_seconds;

        println!("🚀 Running FPS benchmark:");
        println!("   Target FPS: {}", self.config.target_fps);
        println!("   Duration: {}s", self.config.duration_seconds);
        println!("   Complexity: {:?}", self.config.complexity_level);
        println!("   Expected frames: {total_frames}");

        let mut last_frame_time = Instant::now();
        let mut dropped_frames = 0u64;
        let mut min_fps = f32::INFINITY;
        let mut max_fps = 0.0f32;

        for frame in 0..total_frames {
            let frame_start = Instant::now();
            self.frame_count = frame as u64;

            // Render frame
            let render_start = Instant::now();
            let layout = self.create_benchmark_layout();
            if let Err(e) = self.renderer.render(&layout).await {
                eprintln!("Render error at frame {frame}: {e}");
                continue;
            }
            let render_time = render_start.elapsed();
            self.render_times.push(render_time);

            // Calculate actual FPS
            let frame_time = frame_start.duration_since(last_frame_time);
            self.frame_times.push(frame_time);
            last_frame_time = frame_start;

            let current_fps = 1.0 / frame_time.as_secs_f32();
            min_fps = min_fps.min(current_fps);
            max_fps = max_fps.max(current_fps);

            // Progress reporting
            if frame % (self.config.target_fps / 4) == 0 {
                print!(
                    "\rFrame: {}/{} | FPS: {:.1} | Render: {:.2}ms | Buffer: {}B",
                    frame,
                    total_frames,
                    current_fps,
                    render_time.as_secs_f32() * 1000.0,
                    self.renderer.get_buffer_size()
                );
                std::io::Write::flush(&mut std::io::stdout()).ok();
            }

            // Frame timing control
            let elapsed = frame_start.elapsed();
            if elapsed < target_frame_time {
                let sleep_time = target_frame_time - elapsed;
                std::thread::sleep(sleep_time);
            } else {
                dropped_frames += 1;
            }
        }

        println!(); // New line after progress

        // Calculate results
        let total_time = self.start_time.elapsed().as_secs_f32();
        let avg_fps = self.frame_count as f32 / total_time;
        let avg_render_time_us = self
            .render_times
            .iter()
            .map(|d| d.as_micros() as f32)
            .sum::<f32>()
            / self.render_times.len() as f32;
        let max_render_time_us = self
            .render_times
            .iter()
            .map(|d| d.as_micros())
            .max()
            .unwrap_or(0) as u64;

        let buffer_efficiency = if self.frame_count > 0 {
            let avg_buffer_size = self.renderer.get_buffer_size() as f32;
            // Efficiency = operations per buffer byte (higher is better)
            self.frame_count as f32 / (avg_buffer_size + 1.0)
        } else {
            0.0
        };

        BenchmarkResults {
            target_fps: self.config.target_fps,
            actual_avg_fps: avg_fps,
            actual_max_fps: max_fps,
            actual_min_fps: min_fps,
            frame_count: self.frame_count,
            dropped_frames,
            avg_render_time_us,
            max_render_time_us,
            buffer_efficiency,
        }
    }
}

/// Run comprehensive FPS benchmarks
async fn run_fps_benchmarks() -> tui_core::error::Result<()> {
    println!("🎯 TUI FPS Benchmark Suite - Double Buffered Rendering");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
    println!("Testing maximum refresh rate capabilities of the flicker-free system\n");

    // Test configurations - from conservative to extreme
    let test_configs = [
        // Standard refresh rates
        BenchmarkConfig {
            target_fps: 60,
            duration_seconds: 3,
            complexity_level: ComplexityLevel::Simple,
        },
        BenchmarkConfig {
            target_fps: 120,
            duration_seconds: 3,
            complexity_level: ComplexityLevel::Simple,
        },
        BenchmarkConfig {
            target_fps: 144,
            duration_seconds: 3,
            complexity_level: ComplexityLevel::Simple,
        },
        BenchmarkConfig {
            target_fps: 240,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Simple,
        },
        // High refresh with complexity
        BenchmarkConfig {
            target_fps: 60,
            duration_seconds: 3,
            complexity_level: ComplexityLevel::Medium,
        },
        BenchmarkConfig {
            target_fps: 120,
            duration_seconds: 3,
            complexity_level: ComplexityLevel::Medium,
        },
        BenchmarkConfig {
            target_fps: 144,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Medium,
        },
        // Stress tests
        BenchmarkConfig {
            target_fps: 60,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Complex,
        },
        BenchmarkConfig {
            target_fps: 120,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Complex,
        },
        // Extreme stress
        BenchmarkConfig {
            target_fps: 60,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Extreme,
        },
    ];

    let mut all_results = Vec::new();

    for (i, config) in test_configs.iter().enumerate() {
        println!(
            "📊 Test {}/{}: {:?} @ {}fps",
            i + 1,
            test_configs.len(),
            config.complexity_level,
            config.target_fps
        );

        let mut benchmark = FpsBenchmark::new(config.clone())?;
        let results = benchmark.run_benchmark().await;

        // Display results
        println!("   ✓ Results:");
        println!(
            "     Average FPS: {:.1} ({:.1}% of target)",
            results.actual_avg_fps,
            (results.actual_avg_fps / results.target_fps as f32) * 100.0
        );
        println!(
            "     FPS Range: {:.1} - {:.1}",
            results.actual_min_fps, results.actual_max_fps
        );
        println!(
            "     Render Time: {:.2}ms avg, {:.2}ms max",
            results.avg_render_time_us / 1000.0,
            results.max_render_time_us as f32 / 1000.0
        );
        println!(
            "     Dropped Frames: {} ({:.1}%)",
            results.dropped_frames,
            (results.dropped_frames as f32 / results.frame_count as f32) * 100.0
        );
        println!("     Buffer Efficiency: {:.2}", results.buffer_efficiency);
        println!();

        all_results.push(results);
    }

    // Summary analysis
    println!("🏆 BENCHMARK SUMMARY");
    println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

    let best_simple = all_results
        .iter()
        .filter(|r| r.target_fps <= 300)
        .max_by(|a, b| a.actual_avg_fps.partial_cmp(&b.actual_avg_fps).unwrap());

    if let Some(best) = best_simple {
        println!(
            "🥇 Peak Performance: {:.1} FPS achieved (target: {})",
            best.actual_avg_fps, best.target_fps
        );
    }

    let most_stable = all_results
        .iter()
        .min_by(|a, b| a.dropped_frames.cmp(&b.dropped_frames));

    if let Some(stable) = most_stable {
        println!(
            "🎯 Most Stable: {} dropped frames at {} FPS",
            stable.dropped_frames, stable.target_fps
        );
    }

    println!("\n💡 Recommendations:");
    println!("   • For smooth gaming/interactive: 120-144 FPS");
    println!("   • For general applications: 60-90 FPS");
    println!("   • For complex layouts: 30-60 FPS");
    println!("   • The double buffering eliminates flicker at ANY refresh rate!");

    Ok(())
}

#[tokio::main]
async fn main() -> tui_core::error::Result<()> {
    run_fps_benchmarks().await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_benchmark_config() {
        let config = BenchmarkConfig {
            target_fps: 120,
            duration_seconds: 2,
            complexity_level: ComplexityLevel::Medium,
        };
        assert_eq!(config.target_fps, 120);
    }

    #[tokio::test]
    async fn test_simple_benchmark() {
        let config = BenchmarkConfig {
            target_fps: 30,
            duration_seconds: 1,
            complexity_level: ComplexityLevel::Simple,
        };

        let mut benchmark = FpsBenchmark::new(config).unwrap();
        let results = benchmark.run_benchmark().await;

        assert!(results.actual_avg_fps > 0.0);
        assert!(results.frame_count > 0);
    }
}
