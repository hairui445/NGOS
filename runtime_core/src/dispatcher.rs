
use crate::PluginRegistry;
use anyhow::{anyhow, Context};
use ngos_spec::{status_to_result, NgosTensorView, PluginExecute};

pub fn dispatch_f32_copy(
    registry: &PluginRegistry,
    plugin_name: &str,
    input: &[f32],
) -> anyhow::Result<Vec<f32>> {
    let plugin = registry
        .get(plugin_name)
        .ok_or_else(|| anyhow!("plugin '{plugin_name}' is not loaded"))?;
    let mut input_buffer = input.to_vec();
    let mut output_buffer = vec![0.0f32; input.len()];
    let input_view = NgosTensorView::from_f32_slice_mut(&mut input_buffer);
    let mut output_view = NgosTensorView::from_f32_slice_mut(&mut output_buffer);
    unsafe {
        let execute: libloading::Symbol<PluginExecute> = plugin
            .library()
            .get(b"ngos_plugin_execute")
            .context("plugin missing ngos_plugin_execute")?;
        status_to_result(execute(
            &input_view as *const NgosTensorView,
            &mut output_view as *mut NgosTensorView,
        ))
        .map_err(|error| anyhow!(error))?;
    }
    Ok(output_buffer)
}

