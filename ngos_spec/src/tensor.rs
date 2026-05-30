
use crate::device::NgosDeviceId;
use crate::error::{NgosError, NgosResult};
use serde::{Deserialize, Serialize};

pub const NGOS_MAX_DIMS: usize = 8;

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgosDType {
    F32 = 0,
    F64 = 1,
    I32 = 2,
    I64 = 3,
    U8 = 4,
}

impl NgosDType {
    pub const fn byte_width(self) -> usize {
        match self {
            Self::F32 | Self::I32 => 4,
            Self::F64 | Self::I64 => 8,
            Self::U8 => 1,
        }
    }
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgosTensorLayout {
    Contiguous = 0,
    Strided = 1,
}

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum NgosTensorOwnership {
    Borrowed = 0,
    RuntimeOwned = 1,
    PluginOwned = 2,
}

#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct NgosTensorView {
    pub data: *mut u8,
    pub byte_len: usize,
    pub dtype: NgosDType,
    pub layout: NgosTensorLayout,
    pub ownership: NgosTensorOwnership,
    pub device: NgosDeviceId,
    pub ndim: usize,
    pub shape: [usize; NGOS_MAX_DIMS],
}

unsafe impl Send for NgosTensorView {}
unsafe impl Sync for NgosTensorView {}

impl NgosTensorView {
    pub fn from_f32_slice_mut(values: &mut [f32]) -> Self {
        let mut shape = [1; NGOS_MAX_DIMS];
        shape[0] = values.len();
        Self {
            data: values.as_mut_ptr().cast::<u8>(),
            byte_len: values.len() * std::mem::size_of::<f32>(),
            dtype: NgosDType::F32,
            layout: NgosTensorLayout::Contiguous,
            ownership: NgosTensorOwnership::Borrowed,
            device: NgosDeviceId::cpu(),
            ndim: 1,
            shape,
        }
    }

    pub fn element_count(&self) -> NgosResult<usize> {
        if self.ndim == 0 || self.ndim > NGOS_MAX_DIMS {
            return Err(NgosError::InvalidArgument("invalid tensor rank".into()));
        }
        self.shape[..self.ndim]
            .iter()
            .try_fold(1usize, |acc, item| acc.checked_mul(*item))
            .ok_or_else(|| NgosError::InvalidArgument("shape overflow".into()))
    }

    pub fn expected_bytes(&self) -> NgosResult<usize> {
        self.element_count()?
            .checked_mul(self.dtype.byte_width())
            .ok_or_else(|| NgosError::InvalidArgument("tensor byte size overflow".into()))
    }

    pub fn validate(&self) -> NgosResult<()> {
        if self.data.is_null() {
            return Err(NgosError::InvalidArgument(
                "tensor data pointer is null".into(),
            ));
        }
        let expected = self.expected_bytes()?;
        if self.byte_len < expected {
            return Err(NgosError::BufferTooSmall);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_f32_shape() {
        let mut data = vec![1.0f32; 4];
        let view = NgosTensorView::from_f32_slice_mut(&mut data);
        assert_eq!(view.expected_bytes().unwrap(), 16);
        assert!(view.validate().is_ok());
    }
}

