use reactive_tui::driver::{headless::HeadlessDriver, Driver, DriverConfig};
use reactive_tui::layout::{ComputedStyles, Layout, LayoutRect, Overflow, OverflowBehavior};
use reactive_tui::rendering::Renderer;

fn make_layout(
  rect: LayoutRect,
  tag: &str,
  content: Option<&str>,
  styles: ComputedStyles,
  children: Vec<Layout>,
) -> Layout {
  Layout {
    rect,
    tag: tag.to_string(),
    content: content.map(|s| s.to_string()),
    children,
    focused: false,
    element_id: Some(tag.to_string()),
    focusable: false,
    styles,
  }
}

#[tokio::test]
async fn e2e_first_render_clears_and_hides_cursor() {
  let mut renderer = Renderer::new().expect("renderer");

  let root = make_layout(
    LayoutRect {
      x: 0,
      y: 0,
      width: 10,
      height: 2,
    },
    "root",
    Some("hello"),
    ComputedStyles::default(),
    vec![],
  );

  let bytes = renderer.render(&root).await.expect("render bytes");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");

  let out = driver.get_output();
  // Expect clear screen (CSI 2J) and cursor hide (CSI ?25l)
  assert!(
    out.contains("\u{1b}[2J"),
    "output did not contain clear: {}",
    out
  );
  assert!(
    out.contains("\u{1b}[?25l"),
    "output did not contain hide: {}",
    out
  );
}

#[tokio::test]
async fn e2e_offscreen_render_has_no_clear_hide_show() {
  let mut renderer = Renderer::new().expect("renderer");

  let root = make_layout(
    LayoutRect {
      x: 0,
      y: 0,
      width: 10,
      height: 2,
    },
    "root",
    Some("hello"),
    ComputedStyles::default(),
    vec![],
  );

  let bytes = renderer
    .render_offscreen(&root)
    .await
    .expect("render bytes");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");

  let out = driver.get_output();
  assert!(!out.contains("\u{1b}[2J"), "offscreen should not clear");
  assert!(
    !out.contains("\u{1b}[?25l"),
    "offscreen should not hide cursor"
  );
  assert!(
    !out.contains("\u{1b}[?25h"),
    "offscreen should not show cursor"
  );
}

#[tokio::test]
async fn e2e_overflow_hidden_clips_child_content() {
  let mut renderer = Renderer::new().expect("renderer");

  // Parent with width 5 and overflow hidden
  let parent_styles = ComputedStyles {
    overflow: OverflowBehavior {
      x: Overflow::Hidden,
      y: Overflow::Hidden,
    },
    ..ComputedStyles::default()
  };
  let child_styles = ComputedStyles {
    overflow: OverflowBehavior {
      x: Overflow::Visible,
      y: Overflow::Visible,
    },
    ..ComputedStyles::default()
  };

  let child = make_layout(
    LayoutRect {
      x: 0,
      y: 0,
      width: 20,
      height: 1,
    },
    "child",
    Some("HELLOWORLD"),
    child_styles,
    vec![],
  );

  let root = make_layout(
    LayoutRect {
      x: 0,
      y: 0,
      width: 5,
      height: 1,
    },
    "root",
    None,
    parent_styles,
    vec![child],
  );

  let bytes = renderer
    .render_offscreen(&root)
    .await
    .expect("render bytes");

  let mut driver = HeadlessDriver::new(DriverConfig::default()).expect("headless");
  driver.start_application_mode().expect("start");
  driver.write_bytes(&bytes).expect("write");

  let out = driver.get_output();
  assert!(
    out.contains("HELLO"),
    "clipped content should include HELLO: {}",
    out
  );
  assert!(
    !out.contains("WORLD"),
    "clipped content should exclude WORLD: {}",
    out
  );
}
