
use crate::device::NgosDeviceId;
use crate::tensor::{NgosDType, NgosTensorLayout, NgosTensorOwnership, NgosTensorView};
use serde::{Deserialize, Serialize};

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgosMemoryKind {
    Host = 0,
    Device = 1,
    Shared = 2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct MemoryAllocationSpec {
    pub owner: NgosTensorOwnership,
    pub device: NgosDeviceId,
    pub kind: NgosMemoryKind,
    pub byte_len: usize,
}

#[derive(Debug, Clone)]
pub struct HostBuffer {
    data: Vec<u8>,
    dtype: NgosDType,
}

impl HostBuffer {
    pub fn new(byte_len: usize, dtype: NgosDType) -> Self {
        Self {
            data: vec![0; byte_len],
            dtype,
        }
    }

    pub fn as_tensor_view(&mut self, elements: usize) -> NgosTensorView {
        let mut shape = [1usize; crate::tensor::NGOS_MAX_DIMS];
        shape[0] = elements;
        NgosTensorView {
            data: self.data.as_mut_ptr(),
            byte_len: self.data.len(),
            dtype: self.dtype,
            layout: NgosTensorLayout::Contiguous,
            ownership: NgosTensorOwnership::RuntimeOwned,
            device: NgosDeviceId::cpu(),
            ndim: 1,
            shape,
        }
    }
}

