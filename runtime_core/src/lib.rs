
mod dispatcher;
mod metrics;
mod registry;

pub use dispatcher::*;
pub use metrics::*;
pub use registry::*;

use ngos_spec::{DeviceInfo, NgosDeviceId, NgosDeviceKind};

pub fn init_logging() {
    let _ = tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .json()
        .try_init();
}

pub struct Runtime {
    registry: PluginRegistry,
    metrics: RuntimeMetrics,
}

impl Runtime {
    pub fn new() -> Self {
        init_logging();
        Self {
            registry: PluginRegistry::new(),
            metrics: RuntimeMetrics::default(),
        }
    }

    pub fn discover_devices(&self) -> Vec<DeviceInfo> {
        vec![DeviceInfo {
            id: NgosDeviceId::cpu(),
            name: "CPU Runtime".to_string(),
            available: true,
            memory_bytes: None,
        }]
    }

    pub fn load_plugin(&mut self, path: impl AsRef<std::path::Path>) -> anyhow::Result<String> {
        let handle = self.registry.load(path)?;
        Ok(handle)
    }

    pub fn health_report(&self) -> Vec<PluginHealthReport> {
        self.registry.health_report()
    }

    pub fn execute_f32_copy(
        &mut self,
        plugin_name: &str,
        input: &[f32],
    ) -> anyhow::Result<Vec<f32>> {
        let output = dispatch_f32_copy(&self.registry, plugin_name, input)?;
        self.metrics.dispatch_count += 1;
        self.metrics.bytes_copied += input.len() as u64 * std::mem::size_of::<f32>() as u64;
        Ok(output)
    }

    pub fn metrics(&self) -> RuntimeMetrics {
        self.metrics
    }

    pub fn device_kind_supported(&self, kind: NgosDeviceKind) -> bool {
        self.discover_devices()
            .iter()
            .any(|device| device.id.kind == kind && device.available)
            || self.registry.device_kind_supported(kind)
    }
}

impl Default for Runtime {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn runtime_discovers_cpu() {
        let runtime = Runtime::new();
        assert!(runtime.device_kind_supported(NgosDeviceKind::Cpu));
    }
}

