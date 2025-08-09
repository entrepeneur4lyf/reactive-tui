# Observability (optional)

Reactive TUI ships with an optional tracing feature for diagnostics without impacting default runtime performance.

Enable the feature:

```bash
# library users
cargo add reactive-tui --features tracing

# or when running tests locally
cargo test --features tracing
```

Initialize a subscriber in your binary:

```rust
#[cfg(feature = "tracing")]
fn init_tracing() {
    use tracing_subscriber::EnvFilter;
    let _ = tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()) // RUST_LOG env support
        .try_init();
}
```

What’s instrumented currently:
- Driver construction (Unix): trace begin/end
- Event loop thread start (Unix): debug
- Signal thread start (Unix): debug

Design goals:
- Zero overhead when the feature is disabled (no deps pulled in, no logging cost)
- Consistent story across platforms as we expand coverage

Planned additions:
- Light spans around layout and render hot paths
- Error-level logs at key fallback scenarios

Tip: Set RUST_LOG=reactive_tui=debug to focus on framework logs.

