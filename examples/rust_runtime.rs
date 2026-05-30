
use ngos_runtime::Runtime;

fn main() {
    let runtime = Runtime::new();
    for device in runtime.discover_devices() {
        println!("{} available={}", device.name, device.available);
    }
}

