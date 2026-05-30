
```markdown
# Benchmark

`ngos-bench` reports JSON with:

- iterations
- tensor element count
- elapsed time
- bytes processed
- detected devices

Example:

```bash
cargo run -p ngos-bench -- --iterations 10000 --elements 1024
```
```

