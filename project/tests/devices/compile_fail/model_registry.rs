// compile_fail: device-modeling must not export a ModelRegistry type.
// A registry would permit runtime model insertion, violating ADR-0005.
use device_modeling::ModelRegistry;

fn main() {}
