use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn has_node() -> bool {
  Command::new("node")
    .arg("--version")
    .output()
    .map(|o| o.status.success())
    .unwrap_or(false)
}

fn build_napi_cdylib() {
  // Build only the library with the ffi feature to produce the cdylib addon
  // This avoids linking examples/binaries that would fail without Node at link time.
  let status = Command::new("cargo")
    .args(["build", "--lib", "--features", "ffi"]) // debug profile is fine
    .status()
    .expect("failed to spawn cargo build --lib --features ffi");
  assert!(status.success(), "cargo build --lib --features ffi failed");
}

fn lib_name_and_ext() -> (&'static str, &'static str) {
  if cfg!(target_os = "windows") {
    ("reactive_tui", "dll")
  } else if cfg!(target_os = "macos") {
    ("libreactive_tui", "dylib")
  } else {
    ("libreactive_tui", "so")
  }
}

fn find_built_cdylib() -> PathBuf {
  let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
  let (name, ext) = lib_name_and_ext();
  let mut p = PathBuf::from(target_dir);
  p.push("debug");
  p.push(format!("{name}.{ext}"));
  assert!(p.exists(), "Expected cdylib at {:?}", p);
  p
}

fn create_node_copy(src: &Path) -> PathBuf {
  // Node expects a .node extension; copy to a temp dir with correct extension
  let tmp_dir = env::temp_dir().join("reactive_tui_napi_test");
  let _ = fs::create_dir_all(&tmp_dir);
  let target = tmp_dir.join("reactive_tui.node");
  let _ = fs::remove_file(&target);
  fs::copy(src, &target).expect("failed to copy cdylib to .node path");
  target
}

fn write_js_smoke(addon_path: &Path) -> PathBuf {
  let js = format!(
    r#"const addon = require({path});
const keys = Object.keys(addon).sort();
console.log('EXPORTS:', keys.join(','));

function assert(cond, msg) {{ if (!cond) {{ throw new Error(msg); }} }}

assert(typeof addon.get_version === 'function', 'get_version missing');
assert(typeof addon.init_tui === 'function', 'init_tui missing');
assert(typeof addon.JsTuiApp === 'function', 'JsTuiApp missing');
assert(typeof addon.JsElement === 'function', 'JsElement missing');
assert(typeof addon.TuiUtils === 'function', 'TuiUtils missing');
assert(typeof addon.Actions === 'function', 'Actions missing');

const ver = addon.get_version();
console.log('VERSION:', ver);

const el = new addon.JsElement('div');
el.addClass('x');
el.setId('id1');
el.setContent('hello');

const app = new addon.JsTuiApp();
app.setTitle('title');
app.setComponent(el);

console.log('SMOKE_OK');
process.exit(0);
"#,
    path = serde_json::to_string(addon_path.to_str().unwrap()).unwrap()
  );
  let js_path = env::temp_dir().join("reactive_tui_napi_test_smoke.js");
  fs::write(&js_path, js).expect("failed to write JS smoke file");
  js_path
}

#[test]
fn napi_node_smoke_test() {
  if env::var("RUN_NAPI_TESTS").ok().as_deref() != Some("1") {
    eprintln!("skipping napi_node_smoke_test (set RUN_NAPI_TESTS=1 to run)");
    return;
  }

  if !has_node() {
    eprintln!("skipping napi_node_smoke_test: node not found in PATH");
    return;
  }

  // Build the addon cdylib with ffi feature
  build_napi_cdylib();

  // Locate and prepare .node addon
  let cdylib = find_built_cdylib();
  let addon = create_node_copy(&cdylib);
  let js = write_js_smoke(&addon);

  // Run Node to require the addon and call a simple exported function
  let output = Command::new("node")
    .arg(&js)
    .output()
    .expect("failed to spawn node smoke test");

  let stdout = String::from_utf8_lossy(&output.stdout);
  println!("node stdout:\n{}", stdout);
  println!("node stderr:\n{}", String::from_utf8_lossy(&output.stderr));

  assert!(output.status.success(), "node exited with error: {}\nstdout:\n{}\nstderr:\n{}",
    output.status,
    stdout,
    String::from_utf8_lossy(&output.stderr),
  );

  assert!(stdout.contains("SMOKE_OK"), "unexpected node output: {}", stdout);
}

