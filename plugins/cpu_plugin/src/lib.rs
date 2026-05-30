
use plugin_sdk::{
    copy_tensor_bytes, NgosPluginDescriptor, NgosPluginHealth, NgosTensorView, NGOS_SUCCESS,
};

#[no_mangle]
pub extern "C" fn ngos_plugin_init() -> i32 {
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_destroy() -> i32 {
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_descriptor(out: *mut NgosPluginDescriptor) -> i32 {
    if out.is_null() {
        return plugin_sdk::NGOS_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        *out = NgosPluginDescriptor::new("ngos_cpu", "NGOS", 0);
    }
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_health_check(out: *mut NgosPluginHealth) -> i32 {
    if out.is_null() {
        return plugin_sdk::NGOS_ERROR_INVALID_ARGUMENT;
    }
    unsafe {
        *out = NgosPluginHealth::healthy();
    }
    NGOS_SUCCESS
}

#[no_mangle]
pub extern "C" fn ngos_plugin_execute(
    input: *const NgosTensorView,
    output: *mut NgosTensorView,
) -> i32 {
    if input.is_null() || output.is_null() {
        return plugin_sdk::NGOS_ERROR_INVALID_ARGUMENT;
    }
    unsafe { copy_tensor_bytes(&*input, &mut *output) }
}

