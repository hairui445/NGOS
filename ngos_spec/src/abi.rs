
use crate::tensor::NgosTensorView;

pub const NGOS_ABI_VERSION: u32 = 1;

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NgosPluginDescriptor {
    pub abi_version: u32,
    pub name: [u8; 64],
    pub vendor: [u8; 64],
    pub device_kind: u32,
}

impl NgosPluginDescriptor {
    pub fn new(name: &str, vendor: &str, device_kind: u32) -> Self {
        Self {
            abi_version: NGOS_ABI_VERSION,
            name: fixed_bytes::<64>(name),
            vendor: fixed_bytes::<64>(vendor),
            device_kind,
        }
    }

    pub fn name_string(&self) -> String {
        c_string_from_fixed(&self.name)
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NgosPluginHealth {
    pub healthy: bool,
    pub status_code: i32,
    pub last_error: [u8; 128],
}

impl NgosPluginHealth {
    pub fn healthy() -> Self {
        Self {
            healthy: true,
            status_code: 0,
            last_error: [0; 128],
        }
    }

    pub fn unhealthy(status_code: i32, error: &str) -> Self {
        Self {
            healthy: false,
            status_code,
            last_error: fixed_bytes::<128>(error),
        }
    }
}

pub type PluginInit = unsafe extern "C" fn() -> i32;
pub type PluginDestroy = unsafe extern "C" fn() -> i32;
pub type PluginDescriptor = unsafe extern "C" fn(*mut NgosPluginDescriptor) -> i32;
pub type PluginHealthCheck = unsafe extern "C" fn(*mut NgosPluginHealth) -> i32;
pub type PluginExecute = unsafe extern "C" fn(*const NgosTensorView, *mut NgosTensorView) -> i32;

pub fn fixed_bytes<const N: usize>(text: &str) -> [u8; N] {
    let mut out = [0u8; N];
    let bytes = text.as_bytes();
    let len = bytes.len().min(N.saturating_sub(1));
    out[..len].copy_from_slice(&bytes[..len]);
    out
}

pub fn c_string_from_fixed(bytes: &[u8]) -> String {
    let len = bytes.iter().position(|b| *b == 0).unwrap_or(bytes.len());
    String::from_utf8_lossy(&bytes[..len]).to_string()
}

