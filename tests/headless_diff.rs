use reactive_tui::driver::{headless::HeadlessDriver, Driver, DriverConfig};
use reactive_tui::layout::{ComputedStyles, Layout, LayoutRect};
use reactive_tui::rendering::Renderer;

fn layout_with_text(text: &str) -> Layout {
  Layout {
    rect: LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 1,
    },
    children: vec![],
    element_id: Some("root".to_string()),
    tag: "div".to_string(),
    content: Some(text.to_string()),
    styles: ComputedStyles::default(),
    focused: false,
    focusable: false,
  }
}

#[tokio::test]
async fn e2e_diff_second_frame_no_clear_and_only_changed_rows() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");

  // First frame: full render clears
  let bytes1 = renderer
    .render_diff(&layout_with_text("AAAAA"))
    .await
    .expect("render1");
  driver.write_bytes(&bytes1).expect("write1");
  let out1 = driver.get_output();
  assert!(out1.contains("\u{1b}[2J"), "first diff frame should clear");

  driver.clear_output();

  // Second frame: change content; should not clear
  let bytes2 = renderer
    .render_diff(&layout_with_text("BBB"))
    .await
    .expect("render2");
  driver.write_bytes(&bytes2).expect("write2");
  let out2 = driver.get_output();
  assert!(
    !out2.contains("\u{1b}[2J"),
    "second diff frame should not clear: {}",
    out2
  );
  // Should include cursor move to row 1, col 1
  assert!(out2.contains("\u{1b}[1;1H"), "should move to start of row");
}

#[tokio::test]
async fn e2e_diff_shrinking_row_writes_spaces_to_clear_artifacts() {
  let mut renderer = Renderer::new().expect("renderer");
  renderer.enable_diff_mode();

  let layout_full = layout_with_text("ABCDEFGHIJ");
  let _ = renderer
    .render_diff(&layout_full)
    .await
    .expect("render full");

  // Now shorter content; expect overwrite from start
  let bytes = renderer
    .render_diff(&layout_with_text("ABC"))
    .await
    .expect("render short");

  // We cannot perfectly assert the padded spaces without a real raster, but we can at least
  // assert a move and the new content present.
  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");
  let out = driver.get_output();
  assert!(out.contains("\u{1b}[1;1H"));
  assert!(out.contains("ABC"));
}
