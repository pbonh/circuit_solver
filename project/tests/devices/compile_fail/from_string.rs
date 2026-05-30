// compile_fail: DeviceModel must NOT implement From<String>.
// Same rationale as from_str.rs — no string→variant conversion at runtime.
use device_modeling::DeviceModel;

fn main() {
    let _: DeviceModel = DeviceModel::from(String::from("BJT"));
}
