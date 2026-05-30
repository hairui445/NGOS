
pub use ngos_spec::{
    c_string_from_fixed, error_to_status, fixed_bytes, status_to_result, HostBuffer,
    MemoryAllocationSpec, NgosDType, NgosDeviceId, NgosDeviceKind, NgosError, NgosMemoryKind,
    NgosPluginDescriptor, NgosPluginHealth, NgosResult, NgosTensorLayout, NgosTensorOwnership,
    NgosTensorView, NGOS_ABI_VERSION, NGOS_ERROR_BUFFER_TOO_SMALL, NGOS_ERROR_INTERNAL,
    NGOS_ERROR_INVALID_ARGUMENT, NGOS_ERROR_PLUGIN, NGOS_ERROR_UNSUPPORTED, NGOS_SUCCESS,
};

pub fn copy_tensor_bytes(input: &NgosTensorView, output: &mut NgosTensorView) -> i32 {
    if let Err(error) = input.validate().and_then(|_| output.validate()) {
        return error_to_status(&error);
    }
    if input.dtype != output.dtype || input.element_count().ok() != output.element_count().ok() {
        return NGOS_ERROR_INVALID_ARGUMENT;
    }
    if output.byte_len < input.byte_len {
        return NGOS_ERROR_BUFFER_TOO_SMALL;
    }
    unsafe {
        std::ptr::copy_nonoverlapping(input.data.cast_const(), output.data, input.byte_len);
    }
    NGOS_SUCCESS
}

