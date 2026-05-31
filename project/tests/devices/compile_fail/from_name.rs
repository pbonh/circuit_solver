// compile_fail: DeviceModel must not have a from_name() constructor.
// A string-keyed constructor would bypass the closed-enum invariant.
use device_modeling::DeviceModel;

fn main() {
    let _m = DeviceModel::from_name("Diode");
}
