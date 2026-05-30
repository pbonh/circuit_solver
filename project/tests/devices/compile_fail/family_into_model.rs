// compile_fail: DeviceFamily must not have an into_model() method.
// The family→variant mapping must be one-way (variant→family) only.
// An into_model() would allow constructing a DeviceModel from a
// runtime discriminant, violating ADR-0005.
use device_modeling::DeviceFamily;

fn main() {
    let _model = DeviceFamily::Diode.into_model();
}
