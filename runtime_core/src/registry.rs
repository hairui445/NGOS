
use anyhow::{anyhow, Context};
use libloading::Library;
use ngos_spec::{
    c_string_from_fixed, status_to_result, NgosDeviceKind, NgosPluginDescriptor, NgosPluginHealth,
    PluginDescriptor, PluginDestroy, PluginHealthCheck, PluginInit, NGOS_ABI_VERSION,
};
use std::path::{Path, PathBuf};

pub struct PluginHandle {
    library: Library,
    path: PathBuf,
    descriptor: NgosPluginDescriptor,
}

impl PluginHandle {
    fn init(&self) -> anyhow::Result<()> {
        unsafe {
            let init: libloading::Symbol<PluginInit> = self.library.get(b"ngos_plugin_init")?;
            status_to_result(init()).map_err(|error| anyhow!(error))
        }
    }

    pub fn name(&self) -> String {
        self.descriptor.name_string()
    }

    pub fn device_kind(&self) -> NgosDeviceKind {
        match self.descriptor.device_kind {
            0 => NgosDeviceKind::Cpu,
            1 => NgosDeviceKind::Cuda,
            2 => NgosDeviceKind::Ascend,
            _ => NgosDeviceKind::Unknown,
        }
    }

    pub fn health(&self) -> PluginHealthReport {
        unsafe {
            let symbol = self
                .library
                .get::<PluginHealthCheck>(b"ngos_plugin_health_check");
            match symbol {
                Ok(check) => {
                    let mut health = NgosPluginHealth::unhealthy(-1, "health check not run");
                    let status = check(&mut health as *mut NgosPluginHealth);
                    PluginHealthReport {
                        name: self.name(),
                        path: self.path.display().to_string(),
                        healthy: health.healthy && status == 0,
                        status_code: status,
                        last_error: c_string_from_fixed(&health.last_error),
                    }
                }
                Err(error) => PluginHealthReport {
                    name: self.name(),
                    path: self.path.display().to_string(),
                    healthy: false,
                    status_code: -1,
                    last_error: error.to_string(),
                },
            }
        }
    }

    pub fn library(&self) -> &Library {
        &self.library
    }
}

impl Drop for PluginHandle {
    fn drop(&mut self) {
        unsafe {
            if let Ok(destroy) = self.library.get::<PluginDestroy>(b"ngos_plugin_destroy") {
                let _ = destroy();
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct PluginHealthReport {
    pub name: String,
    pub path: String,
    pub healthy: bool,
    pub status_code: i32,
    pub last_error: String,
}

pub struct PluginRegistry {
    plugins: Vec<PluginHandle>,
}

impl PluginRegistry {
    pub fn new() -> Self {
        Self {
            plugins: Vec::new(),
        }
    }

    pub fn load(&mut self, path: impl AsRef<Path>) -> anyhow::Result<String> {
        let path = path.as_ref();
        let library = unsafe { Library::new(path) }
            .with_context(|| format!("failed to load plugin {}", path.display()))?;
        let descriptor = unsafe {
            let descriptor_fn: libloading::Symbol<PluginDescriptor> =
                library.get(b"ngos_plugin_descriptor")?;
            let mut descriptor = NgosPluginDescriptor::new("unknown", "unknown", 255);
            status_to_result(descriptor_fn(&mut descriptor as *mut NgosPluginDescriptor))
                .map_err(|error| anyhow!(error))?;
            if descriptor.abi_version != NGOS_ABI_VERSION {
                return Err(anyhow!(
                    "ABI version mismatch: expected {}, got {}",
                    NGOS_ABI_VERSION,
                    descriptor.abi_version
                ));
            }
            descriptor
        };
        let handle = PluginHandle {
            library,
            path: path.to_path_buf(),
            descriptor,
        };
        handle.init()?;
        let name = handle.name();
        self.plugins.push(handle);
        Ok(name)
    }

    pub fn get(&self, name: &str) -> Option<&PluginHandle> {
        self.plugins.iter().find(|plugin| plugin.name() == name)
    }

    pub fn health_report(&self) -> Vec<PluginHealthReport> {
        self.plugins.iter().map(PluginHandle::health).collect()
    }

    pub fn device_kind_supported(&self, kind: NgosDeviceKind) -> bool {
        self.plugins
            .iter()
            .any(|plugin| plugin.device_kind() == kind && plugin.health().healthy)
    }
}

impl Default for PluginRegistry {
    fn default() -> Self {
        Self::new()
    }
}

