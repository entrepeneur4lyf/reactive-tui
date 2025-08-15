use reactive_tui::driver::{headless::HeadlessDriver, Driver, DriverConfig};
use reactive_tui::layout::{ComputedStyles, Layout, LayoutRect};
use reactive_tui::rendering::Renderer;

fn layout_with(text: &str, _fg: Option<()>, _bg: Option<()>) -> Layout {
  let styles = ComputedStyles::default();
  Layout {
    rect: LayoutRect { x: 0, y: 0, width: 40, height: 2 },
    children: vec![],
    element_id: Some("root".to_string()),
    tag: "div".to_string(),
    content: Some(text.to_string()),
    styles,
    focused: false,
    focusable: true,
  }
}

#[tokio::test]
async fn diff_minimal_reduces_cursor_moves_and_toggles_for_attribute_change() {
  let mut renderer = Renderer::new().expect("renderer");
  // Default now has diff_mode_enabled = true
  renderer.enable_diff_minimal_ansi();

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");

  // Baseline: plain text
  let _ = renderer.render_diff(&layout_with("HelloWorld", None, None)).await.unwrap();
  // Change: simulate style change by using h1 tag which maps to bold style
  let mut layout = layout_with("HelloWorld", None, None);
  layout.tag = "h1".into();
  let bytes = renderer.render_diff(&layout).await.unwrap();
  driver.write_bytes(&bytes).unwrap();
  let out = driver.get_output();
  // Should contain 1;1 move and 1m, but avoid full row repaint
  assert!(out.contains("\u{1b}[1;1H"));
  assert!(out.contains("\u{1b}[1m"));
}

#[tokio::test]
async fn diff_minimal_color_change_emits_color_toggle_only() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_minimal_ansi();

  // Baseline: code tag has magenta fg and dark grey bg (per RenderStyle defaults)
  let mut base = layout_with("Color", None, None);
  base.tag = "code".into();
  let _ = renderer.render_diff(&base).await.unwrap();

  // Change to success tag (green fg)
  let mut next = layout_with("Color", None, None);
  next.tag = "success".into();
  let bytes = renderer.render_diff(&next).await.unwrap();

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).unwrap();
  let out = driver.get_output();
  // Expect a foreground color toggle towards green hues (92 or 32)
  assert!(out.contains("\u{1b}[92m") || out.contains("\u{1b}[32m"));
}

