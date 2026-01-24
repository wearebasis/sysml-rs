# Debugging

This workspace supports CLI debugging with Rust's `rust-gdb` and `rust-lldb` wrappers.

## Prereqs

- `rust-gdb` or `rust-lldb` on PATH (installed with the Rust toolchain).
- Build artifacts in `target/` (use `cargo build` or `cargo test --no-run`).

## Debug an example binary

```bash
# Build an example
cargo build -p sysml-core --example vehicle_model

# GDB (Linux)
rust-gdb --args target/debug/examples/vehicle_model

# LLDB (macOS)
rust-lldb -- target/debug/examples/vehicle_model
```

Common debugger commands:

```
break main
run
bt
frame 0
print some_var
continue
```

## Debug a test

```bash
# Build tests without running
cargo test -p sysml-core --test <test_name> --no-run

# Run the test binary under a debugger
rust-gdb --args target/debug/deps/<test_binary> --nocapture
```

Tip: set `RUST_BACKTRACE=1` to get a backtrace on panic.

## Release-like debugging

Use the workspace's `release-debug` profile for optimized builds with symbols:

```bash
cargo build --profile release-debug -p sysml-core --example vehicle_model
rust-gdb --args target/release-debug/examples/vehicle_model
```

## Logging

If a crate uses `tracing` or `log`, enable verbose output:

```bash
RUST_LOG=debug cargo run -p sysml-core --example vehicle_model
```
