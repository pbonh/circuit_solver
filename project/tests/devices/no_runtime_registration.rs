//! Integration test: no runtime model-registration API exists.
//!
//! Traces spec `device-modeling#runtime-registration-rejected` and ADR-0005.
//!
//! The closed-enum `DeviceModel` decision means:
//!
//! - `DeviceModel` is a Rust `enum` — no `Box`, no `dyn`, no string-keyed
//!   registry.
//! - Constructing a `DeviceModel` requires knowing the variant at compile
//!   time; there is no `FromStr`, `From<String>`, or factory function that
//!   maps an arbitrary runtime string to a `DeviceModel` variant.
//! - The crate exposes no `register`, `insert`, or `add` API that could
//!   extend the model set at runtime.
//!
//! This test file encodes those invariants via three mechanisms:
//!
//! 1. **Compile-time trait witnesses** (const blocks) proving `DeviceModel`
//!    is `Sized` and `Clone` — no `dyn` or heap indirection.
//! 2. **Runtime assertions** confirming inline layout and closed-enum
//!    exhaustiveness.
//! 3. **Compile-fail tests** (via `trybuild`) proving that `FromStr`,
//!    `From<String>`, registry types, and string-keyed constructors do NOT
//!    exist on `DeviceModel`. If someone adds one of these APIs, the
//!    corresponding compile-fail test will start compiling, and `trybuild`
//!    will report the failure — surfacing the ADR-0005 violation.

use device_modeling::{
    DeviceFamily, DeviceModel, DiodeParams, BJTParams, MOSFETParams, MosLevel1Params,
};
use circuit_solver_types::ModelName;

// ---------------------------------------------------------------------------
// 1. DeviceModel is Sized, Clone, and owns its payload inline (no dyn/Box)
// ---------------------------------------------------------------------------

/// Compile-time witness: `DeviceModel` is `Sized` (i.e. not `dyn`).
/// If someone turned `DeviceModel` into a trait object, this would fail.
const _: () = {
    #[allow(dead_code)]
    fn assert_sized<T: Sized>() {}
    fn _check() {
        assert_sized::<DeviceModel>();
    }
};

/// Compile-time witness: `DeviceModel` is `Clone` (owned value semantics).
const _: () = {
    #[allow(dead_code)]
    fn assert_clone<T: Clone>() {}
    fn _check() {
        assert_clone::<DeviceModel>();
    }
};

/// Runtime witness: the enum size is bounded by its largest variant —
/// no heap indirection has been smuggled in via `Box` inside the enum.
#[test]
fn enum_inlines_largest_variant_no_heap_indirection() {
    assert!(
        std::mem::size_of::<DeviceModel>() >= std::mem::size_of::<MOSFETParams>(),
        "DeviceModel must inline its largest variant per ADR-0005 (no Box/dyn)"
    );
}

// ---------------------------------------------------------------------------
// 2. Exhaustiveness: match on DeviceModel covers all variants
// ---------------------------------------------------------------------------

#[test]
fn closed_enum_match_is_exhaustive_no_wildcard() {
    // An unannotated match (no `_` arm) proves the enum is closed.
    // Adding a new variant without updating this test breaks compilation.
    fn classify(m: &DeviceModel) -> DeviceFamily {
        match m {
            DeviceModel::Diode(_) => DeviceFamily::Diode,
            DeviceModel::BJT(_) => DeviceFamily::BJT,
            DeviceModel::MOSFET(_) => DeviceFamily::MOSFET,
        }
    }

    let diode = DeviceModel::Diode(DiodeParams {
        name: ModelName::new("d1n4148"),
        ..Default::default()
    });
    assert_eq!(classify(&diode), DeviceFamily::Diode);

    let bjt = DeviceModel::BJT(BJTParams {
        name: ModelName::new("q2n2222"),
        ..Default::default()
    });
    assert_eq!(classify(&bjt), DeviceFamily::BJT);

    let mosfet = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()));
    assert_eq!(classify(&mosfet), DeviceFamily::MOSFET);
}

// ---------------------------------------------------------------------------
// 3. Compile-fail tests via trybuild
// ---------------------------------------------------------------------------

/// Each `.rs` file in `compile_fail/` MUST fail to compile. If it starts
/// compiling, that means a runtime registration API was added, violating
/// ADR-0005 and spec `device-modeling#runtime-registration-rejected`.
#[test]
fn no_runtime_registration_apis_exist() {
    let t = trybuild::TestCases::new();
    t.compile_fail("tests/devices/compile_fail/*.rs");
}
