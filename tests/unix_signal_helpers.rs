use reactive_tui::driver::{DriverEvent, DriverManager};

#[test]
#[ignore = "requires TTY"]
fn resume_emits_resize() {
  let mut manager = DriverManager::with_config(reactive_tui::driver::DriverConfig {
    driver_type: Some(reactive_tui::driver::DriverType::Unix),
    ..Default::default()
  })
  .expect("driver manager");

  let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
  // Start app mode to ensure terminal is setup
  manager.driver_mut().start_application_mode().unwrap();
  manager.driver_mut().start_event_loop(tx).unwrap();

  // Simulate resume path by calling resume() directly
  manager.driver_mut().resume().unwrap();

  // Drain a few events to look for Resize (non-flaky)
  let mut found = false;
  for _ in 0..10 {
    if let Some(ev) = rx.blocking_recv() {
      if let DriverEvent::Resize(_, _) = ev {
        found = true;
        break;
      }
    } else {
      break;
    }
  }

  assert!(found, "expected at least one Resize event after resume");

  manager.driver_mut().stop_event_loop().unwrap();
  manager.driver_mut().stop_application_mode().unwrap();
}

#[test]
#[ignore = "requires TTY/signals"]
fn sigwinch_emits_resize() {
  let mut manager = DriverManager::with_config(reactive_tui::driver::DriverConfig {
    driver_type: Some(reactive_tui::driver::DriverType::Unix),
    ..Default::default()
  })
  .expect("driver manager");

  let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
  manager.driver_mut().start_application_mode().unwrap();
  manager.driver_mut().start_event_loop(tx).unwrap();

  unsafe { libc::raise(libc::SIGWINCH) };

  // Drain a few events to look for Resize
  let mut found = false;
  for _ in 0..20 {
    if let Some(ev) = rx.blocking_recv() {
      if let DriverEvent::Resize(_, _) = ev {
        found = true;
        break;
      }
    } else {
      break;
    }
  }
  assert!(found, "expected Resize event after SIGWINCH");

  manager.driver_mut().stop_event_loop().unwrap();
  manager.driver_mut().stop_application_mode().unwrap();
}
