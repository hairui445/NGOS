
```markdown
# Memory Specification

Memory ownership is explicit:

- `Borrowed`: caller owns allocation and release.
- `RuntimeOwned`: runtime owns allocation and release.
- `PluginOwned`: plugin owns allocation and release.

Cross-device copies must be represented as explicit dispatch operations. Zero-copy sharing must only be used when both runtimes agree on lifetime and synchronization boundaries.
```

