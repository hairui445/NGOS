
```markdown
# NGOS Architecture

NGOS is a Runtime Compatibility Middleware.

```text
Framework
  -> NGOS Runtime Core
  -> Vendor Plugin ABI
  -> Hardware Runtime
```

The MVP separates:

- `ngos_spec`: stable tensor, memory, device, and ABI definitions.
- `plugin_sdk`: safe helpers for plugin authors.
- `runtime_core`: plugin registry, dispatch, health, metrics, logging.
- `plugins/cpu_plugin`: real CPU copy/execute plugin.
- `plugins/cuda_plugin`: CUDA capability detector and ABI placeholder that returns Unsupported without CUDA.
- `certification`: ABI, health, and tensor dispatch validation.
- `benchmark`: runtime overhead measurement.
- `bindings/python`: PyO3 Python SDK.
- `control_plane`: small Go health service for service orchestration integration.
```

