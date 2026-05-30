
```markdown
# NGOS Test Strategy

Rust unit tests live beside their modules and are executed with:

```bash
cargo test --workspace
```

Integration validation:

```bash
cargo build --workspace
cargo run -p ngos-cert -- target/debug/libngos_cpu_plugin.so
cargo run -p ngos-bench -- --iterations 1000 --elements 128
```

Long-running 72-hour stability testing is intentionally a release-gate procedure, not a short unit test. Use `ngos-bench` with a supervisor and memory monitor for that gate.
```
