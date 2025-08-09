# Signals, Suspend/Resume, and Resizes (Unix)

Reactive TUI provides deterministic signal handling on Unix platforms using signal-hook.

What you get out of the box when using DriverManager::start():
- SIGWINCH: emits DriverEvent::Resize(cols, rows) via the driver event channel
- SIGTSTP (Ctrl+Z): best-effort clean up (disable raw mode) and re-raise for default suspend
- SIGCONT (resume): best-effort restore of raw mode and an immediate Resize emit
- Clean lifecycle: signal thread starts with the event loop and joins on shutdown

Windows: signal-driven suspend/resume is not applicable. The Windows driver focuses on console modes and virtual terminal sequences.

## Quick start

```rust
use reactive_tui::driver::{DriverManager, DriverConfig, DriverType, DriverEvent};

fn main() -> anyhow::Result<()> {
    // Create a Unix driver explicitly (or omit driver_type to auto-detect)
    let mut manager = DriverManager::with_config(DriverConfig {
        driver_type: Some(DriverType::Unix),
        ..Default::default()
    })?;

    // Start terminal application mode and event loop
    let mut rx = manager.start()?; // installs signal-handling thread on Unix

    // Event loop: will receive Resize on SIGWINCH and after SIGCONT
    while let Some(ev) = rx.blocking_recv() {
        match ev {
            DriverEvent::Resize(cols, rows) => {
                // Re-layout UI here
                println!("resized: {}x{}", cols, rows);
            }
            DriverEvent::Key(k) => { /* ... */ }
            DriverEvent::Mouse(m) => { /* ... */ }
            _ => {}
        }
    }

    manager.stop()?;
    Ok(())
}
```

## Suspend/Resume helpers

Drivers expose default suspend/resume helpers:

```rust
use reactive_tui::driver::Driver;

// Before sending the process a SIGTSTP yourself, you can call:
manager.driver_mut().suspend()?;   // best-effort: leaves app mode if supported
// After resuming (SIGCONT), call:
manager.driver_mut().resume()?;    // best-effort: re-enters app mode if supported
```

Note: The Unix signal thread also reacts to SIGTSTP/SIGCONT to toggle raw mode and emit a Resize on resume, so explicit suspend/resume calls are optional in many cases.

## Terminal title and OSC sequences

If you call set_title(), the Unix driver will emit the normalized sequence:

```text
ESC ] 2 ; <title> ESC \
```

That is, the bytes: "\x1b]2;&lt;title&gt;\x1b\\". This is consistent across Unix and Windows (when VT sequences are available; otherwise Windows uses SetConsoleTitleW).

## Limitations and recommendations

- Avoid doing heavy work inside signal handlers; Reactive TUI processes signals on a dedicated thread and forwards events through the channel.
- If your application changes terminal modes directly, re-apply them after resume in response to the Resize event.
- The framework performs best-effort raw mode toggling around suspend/resume; the authoritative full setup/teardown still happens in start_application_mode/stop_application_mode.

