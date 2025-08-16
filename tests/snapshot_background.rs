use reactive_tui::compat::Color;
use reactive_tui::layout::OverflowBehavior;
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
  style.background = Some(Color::Rgb { r: 0, g: 128, b: 0 }.into());
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
  }.into());
  renderer.set_style_for_tag("box".to_string(), style);

  let rt = tokio::runtime::Runtime::new().unwrap();
  let bytes = rt
    .block_on(async { renderer.render(&layout).await })
    .expect("render");
  let s = String::from_utf8_lossy(&bytes);
  assert!(s.contains("Hi"));
}

#[test]
fn snapshot_overflow_hidden_clips_children() {
  use reactive_tui::layout::Overflow;

  let mut renderer = Renderer::new().expect("renderer");

  // Parent layout with small rect, simulate overflow hidden
  let parent = Layout {
    rect: LayoutRect {
      x: 0,
      y: 0,
      width: 5,
      height: 1,
    },
    children: vec![Layout {
      rect: LayoutRect {
        x: 0,
        y: 0,
        width: 20,
        height: 1,
      },
      children: vec![],
      element_id: None,
      tag: "child".to_string(),
      content: Some("HELLOWORLD".to_string()),
      styles: ComputedStyles {
        overflow: OverflowBehavior {
          x: Overflow::Visible,
          y: Overflow::Visible,
        },
        ..ComputedStyles::default()
      },
      focused: false,
      focusable: false,
    }],
    element_id: None,
    tag: "parent".to_string(),
    content: None,
    styles: ComputedStyles {
      overflow: OverflowBehavior {
        x: Overflow::Hidden,
        y: Overflow::Hidden,
      },
      ..ComputedStyles::default()
    },
    focused: false,
    focusable: false,
  };

  let rt = tokio::runtime::Runtime::new().unwrap();
  let bytes = rt
    .block_on(async { renderer.render(&parent).await })
    .expect("render");
  let s = String::from_utf8_lossy(&bytes);

  // Expect child text beyond width 5 is clipped
  assert!(s.contains("HELLO"));
  assert!(!s.contains("WORLD"));
}
