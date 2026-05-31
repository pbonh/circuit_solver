// compile_fail: device-modeling must not export a ModelFactory type.
// A factory would permit runtime model construction from dynamic data,
// violating ADR-0005.
use device_modeling::ModelFactory;

fn main() {}
