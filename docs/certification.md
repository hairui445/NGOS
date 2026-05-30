
```markdown
# Certification

`ngos-cert` validates:

- dynamic library load
- required symbol presence
- ABI version
- plugin initialization
- health check
- f32 tensor dispatch correctness

Example:

```bash
cargo run -p ngos-cert -- target/debug/libngos_cpu_plugin.so
```
```

