use reactive_tui::driver::{headless::HeadlessDriver, Driver, DriverConfig};
use reactive_tui::layout::{ComputedStyles, Layout, LayoutRect};
use reactive_tui::rendering::Renderer;

fn layout_with(text: &str, focused: bool) -> Layout {
  Layout {
    rect: LayoutRect {
      x: 0,
      y: 0,
      width: 40,
      height: 2,
    },
    children: vec![],
    element_id: Some("root".to_string()),
    tag: "div".to_string(),
    content: Some(text.to_string()),
    styles: ComputedStyles::default(),
    focused,
    focusable: true,
  }
}

#[tokio::test]
async fn e2e_diff_style_change_updates_row() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");

  // First frame: unfocused
  let bytes1 = renderer
    .render_diff(&layout_with("Hello Style", false))
    .await
    .expect("render1");
  driver.write_bytes(&bytes1).expect("write1");
  driver.clear_output();

  // Second frame: same text, focused -> style changes
  let bytes2 = renderer
    .render_diff(&layout_with("Hello Style", true))
    .await
    .expect("render2");
  driver.write_bytes(&bytes2).expect("write2");
  let out2 = driver.get_output();
  // Should not clear and should move to row 1
  assert!(!out2.contains("\u{1b}[2J"));
  assert!(out2.contains("\u{1b}[1;1H"));
}

#[tokio::test]
async fn e2e_diff_multi_line_only_updates_changed_row() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  // Two-line content
  let first = layout_with("Line1\nLine2", false);
  let _ = renderer.render_diff(&first).await.expect("render1");

  // Change only second line
  let second = layout_with("Line1\nChanged", false);
  let bytes = renderer.render_diff(&second).await.expect("render2");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");
  let out = driver.get_output();

  // Should include move to second row and the changed content
  assert!(
    out.contains("\u{1b}[2;1H"),
    "expected cursor move to row 2: {}",
    out
  );
  assert!(out.contains("Changed"));
}

#[tokio::test]
async fn e2e_diff_resize_forces_full_repaint() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  // Initial frame establishes baseline without asserting
  let _ = renderer
    .render_diff(&layout_with("Hello", false))
    .await
    .expect("render1");

  // Now simulate resize
  renderer.on_resize(100, 40);

  // Next diff render should perform a full repaint (clear)
  let bytes = renderer
    .render_diff(&layout_with("Hello", false))
    .await
    .expect("render2");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");
  let out = driver.get_output();
  assert!(
    out.contains("\u{1b}[2J"),
    "resize should trigger full repaint: {}",
    out
  );
}

#[tokio::test]
async fn e2e_diff_in_row_style_change_minimal_update() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  // First render baseline
  let _ = renderer
    .render_diff(&layout_with("AAA BBB", false))
    .await
    .expect("render1");

  // Simulate in-row style change by toggling focus (style likely changes but text same)
  let bytes = renderer
    .render_diff(&layout_with("AAA BBB", true))
    .await
    .expect("render2");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");
  let out = driver.get_output();
  // Should at least include move to first row; we can't assert exact ANSI yet without full grid emission
  assert!(out.contains("\u{1b}[1;1H"));
}
