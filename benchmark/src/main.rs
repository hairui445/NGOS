
use ngos_runtime::Runtime;
use serde_json::json;
use std::time::Instant;

fn parse_arg(name: &str, default: usize) -> usize {
    let args: Vec<String> = std::env::args().collect();
    args.windows(2)
        .find(|pair| pair[0] == name)
        .and_then(|pair| pair[1].parse::<usize>().ok())
        .unwrap_or(default)
}

fn main() -> anyhow::Result<()> {
    let iterations = parse_arg("--iterations", 10_000);
    let elements = parse_arg("--elements", 1_024);
    let input = vec![1.0f32; elements];
    let start = Instant::now();
    let mut checksum = 0.0f32;
    for _ in 0..iterations {
        let output = input.clone();
        checksum += output[0];
    }
    let elapsed = start.elapsed();
    let bytes = iterations as u128 * elements as u128 * std::mem::size_of::<f32>() as u128;
    let runtime = Runtime::new();
    println!(
        "{}",
        serde_json::to_string_pretty(&json!({
            "iterations": iterations,
            "elements": elements,
            "elapsed_ms": elapsed.as_secs_f64() * 1000.0,
            "bytes_processed": bytes,
            "dispatch_path": "cpu_memory_copy_baseline",
            "devices": runtime.discover_devices(),
            "checksum": checksum
        }))?
    );
    Ok(())
}

