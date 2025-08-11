use reactive_tui::compat::Color;
use reactive_tui::prelude::*;

#[test]
fn snapshot_background_fill_basic() {
  // Build a simple element tree and compute a minimal layout
  let element = Element::with_tag("div")
    .class("bg-test")
    .content("ABC\nDEF\nGHI")
    .build();

  let _layout_engine = LayoutEngine::with_dimensions(80, 25);
  let mut renderer = Renderer::new().expect("renderer");

  // Manually build a LayoutRect for the element and inject styles
  let layout = Layout {
    rect: LayoutRect {
      x: 2,
      y: 1,
      width: 10,
      height: 3,
    },
    children: vec![],
    element_id: None,
    tag: element.tag.clone(),
    content: element.content.clone(),
    styles: ComputedStyles::default(),
    focused: false,
    focusable: false,
  };

  // Inject a background style through tag style cache
  let mut style = RenderStyle::default();
  style.background = Some(Color::Rgb { r: 0, g: 128, b: 0 });
  renderer.set_style_for_tag(layout.tag.clone(), style);

  // Render and capture bytes
  let rt = tokio::runtime::Runtime::new().unwrap();
  let bytes = rt
    .block_on(async { renderer.render(&layout).await })
    .expect("render");
  let s = String::from_utf8_lossy(&bytes);

  assert!(s.contains("\x1b["), "should contain ANSI sequences");
  assert!(s.contains("ABC"));
  assert!(s.contains("DEF"));
  assert!(s.contains("GHI"));
}

#[test]
fn snapshot_background_fill_component_styles() {
  let mut renderer = Renderer::new().expect("renderer");
  let layout = Layout {
    rect: LayoutRect {
      x: 0,
      y: 0,
      width: 6,
      height: 2,
    },
    children: vec![],
    element_id: None,
    tag: "box".to_string(),
    content: Some("Hi".to_string()),
    styles: ComputedStyles::default(),
    focused: false,
    focusable: false,
  };

  // Simulate style cache with background for tag "box"
  let mut style = RenderStyle::default();
  style.background = Some(Color::Rgb {
    r: 10,
    g: 10,
    b: 10,
  });
  renderer.set_style_for_tag("box".to_string(), style);

  let rt = tokio::runtime::Runtime::new().unwrap();
  let bytes = rt
    .block_on(async { renderer.render(&layout).await })
    .expect("render");
  let s = String::from_utf8_lossy(&bytes);
  assert!(s.contains("Hi"));
}
