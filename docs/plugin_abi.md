
```markdown
# Plugin ABI

ABI version: `1`.

Required exported symbols:

```c
int ngos_plugin_init(void);
int ngos_plugin_destroy(void);
int ngos_plugin_descriptor(NgosPluginDescriptor* out);
int ngos_plugin_health_check(NgosPluginHealth* out);
int ngos_plugin_execute(const NgosTensorView* input, NgosTensorView* output);
```

Status code `0` means success. Non-zero status codes are defined in `ngos_spec/src/error.rs`.
```

