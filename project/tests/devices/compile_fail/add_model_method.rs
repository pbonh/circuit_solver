// compile_fail: DeviceModel must not have an add_model() method.
// Same rationale as register_model_method.rs — no runtime model insertion.
use device_modeling::DeviceModel;

fn main() {
    let m = DeviceModel::Diode(device_modeling::DiodeParams::default());
    m.add_model("new_model");
}
