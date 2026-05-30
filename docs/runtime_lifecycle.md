
```markdown
# Runtime Lifecycle

Lifecycle:

1. Initialize runtime.
2. Discover built-in devices.
3. Load plugin dynamic libraries.
4. Validate ABI descriptor.
5. Initialize plugin.
6. Run health checks.
7. Dispatch tensor execution.
8. Destroy plugin during unload/drop.

For high-risk third-party plugins, certification should be executed in a child process before production loading.
```

