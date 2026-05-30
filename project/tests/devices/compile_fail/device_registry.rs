// compile_fail: device-modeling must not export a DeviceRegistry type.
// Another name for a runtime model store — prohibited by ADR-0005.
use device_modeling::DeviceRegistry;

fn main() {}
