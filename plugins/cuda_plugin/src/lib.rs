
use plugin_sdk::{
    NgosPluginDescriptor, NgosPluginHealth, NgosTensorView, NGOS_ERROR_INVALID_ARGUMENT,
    NGOS_ERROR_UNSUPPORTED, NGOS_SUCCESS,
};

fn cuda_available() -> bool {
    ["libcuda.so.1", "libcuda.so", "nvcuda.dll", "libcuda.dylib"]
        .iter()
        .any(|name| unsafe { libloading::Library::new(name).is_ok() })
}

#[no_mangle]
pub extern "C" fn ngos_plugin_init() -> i32 {
    if cuda_available() {
        NGOS_SUCCESS
    } else {
        NGOS_ERROR_UNSUPPORTED
    }
}

#[no_mangle]
pub extern "C" fn ngos_plugin_destroy() -> i32 {
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_descriptor(out: *mut NgosPluginDescriptor) -> i32 {
    if out.is_null() {
        return NGOS_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        *out = NgosPluginDescriptor::new("ngos_cuda", "NGOS", 1);
    }
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_health_check(out: *mut NgosPluginHealth) -> i32 {
    if out.is_null() {
        return NGOS_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        *out = if cuda_available() {
            NgosPluginHealth::healthy()
        } else {
            NgosPluginHealth::unhealthy(NGOS_ERROR_UNSUPPORTED, "CUDA driver library not found")
        };
    }
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_execute(
    _input: *const NgosTensorView,
    _output: *mut NgosTensorView,
) -> i32 {
    NGOS_ERROR_UNSUPPORTED
}

