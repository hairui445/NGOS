
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgosDeviceKind {
    Cpu = 0,
    Cuda = 1,
    Ascend = 2,
    Unknown = 255,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct NgosDeviceId {
    pub kind: NgosDeviceKind,
    pub index: u32,
}

impl NgosDeviceId {
    pub const fn cpu() -> Self {
        Self {
            kind: NgosDeviceKind::Cpu,
            index: 0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DeviceInfo {
    pub id: NgosDeviceId,
    pub name: String,
    pub available: bool,
    pub memory_bytes: Option<u64>,
}

