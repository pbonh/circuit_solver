// compile_fail: DeviceModel must not have a register_model() method.
// Adding a registration method to DeviceModel itself would allow runtime
// model insertion, violating ADR-0005.
use device_modeling::DeviceModel;

fn main() {
    let m = DeviceModel::Diode(device_modeling::DiodeParams::default());
    m.register_model("new_model");
}
