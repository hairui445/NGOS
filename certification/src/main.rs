
use anyhow::{anyhow, Context};
use ngos_runtime::Runtime;
use serde_json::json;

fn main() -> anyhow::Result<()> {
    let mut args = std::env::args().skip(1);
    let plugin_path = args
        .next()
        .ok_or_else(|| anyhow!("usage: ngos-cert <plugin.so|plugin.dll|plugin.dylib>"))?;
    let mut runtime = Runtime::new();
    let name = runtime
        .load_plugin(&plugin_path)
        .with_context(|| format!("plugin failed ABI/init certification: {plugin_path}"))?;
    let health = runtime.health_report();
    let output = runtime
        .execute_f32_copy(&name, &[1.0, 2.0, 3.0, 4.0])
        .context("plugin failed tensor dispatch certification")?;
    if output != [1.0, 2.0, 3.0, 4.0] {
        return Err(anyhow!("plugin copied incorrect tensor data"));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "plugin": name,
            "abi": "ok",
            "health": health.iter().map(|item| json!({
                "name": item.name,
                "healthy": item.healthy,
                "status_code": item.status_code,
                "last_error": item.last_error,
            })).collect::<Vec<_>>(),
            "tensor_dispatch": "ok",
            "result": "certified"
        }))?
    );
    Ok(())
}

