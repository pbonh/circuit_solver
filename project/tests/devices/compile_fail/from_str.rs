// compile_fail: DeviceModel must NOT implement FromStr.
// Adding FromStr would allow constructing a DeviceModel from an arbitrary
// runtime string, violating ADR-0005 (closed-enum, compile-time-only
// extensibility).
use device_modeling::DeviceModel;

fn main() {
    let _: DeviceModel = "Diode".parse().unwrap();
}
