// compile_fail: DeviceModel must not have a new_from_string() constructor.
// Any string→DeviceModel constructor is runtime registration by another name.
use device_modeling::DeviceModel;

fn main() {
    let _m = DeviceModel::new_from_string("MOSFET");
}
