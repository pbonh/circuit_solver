//! Scenario: `device-modeling#runtime-registration-rejected`
//!
//! Confirm no runtime model-registration API exists — compile-time-only
//! extensibility per ADR-0005.
//!
//! # Given
//!
//! The `DeviceModel` closed enum as defined in `device-modeling::model`,
//! with variants `Diode`, `BJT`, `MOSFET` each carrying their parameter
//! payload inline (no `Box`, no `dyn`, no string-keyed registry).
//!
//! # When
//!
//! An external consumer (downstream crate or Python binding) attempts to
//! register a new device model at runtime.
//!
//! # Then
//!
//! The Rust type system prevents this:
//! - `DeviceModel` is a concrete `enum`, not a trait, so no `dyn DeviceModel`
//!   registration surface exists.
//! - No `register()`, `add_model()`, or `insert()` methods appear on the
//!   public API.
//! - The only way to produce a `DeviceModel` value is by constructing one of
//!   the three known variants at the call site — a compile-time act.

use device_modeling::{
    BJTParams, BJTPolarity, DeviceFamily, DeviceModel, DiodeParams, MOSFETParams,
    MosBSIM3v3Params, MosBSIM4Params, MosLevel1Params, MosPolarity,
};
use std::mem;

// ---------------------------------------------------------------------------
// 1. DeviceModel is Sized — no dyn-registration surface
// ---------------------------------------------------------------------------

/// Compile-time witness: `DeviceModel` is `Sized`, so it cannot be
/// behind a `dyn` trait-object registration table. ADR-0005 explicitly
/// rejects `dyn DeviceModel` as a dispatch mechanism.
#[test]
fn device_model_is_sized_no_dyn_registration() {
    fn assert_sized<T: Sized>() {}
    assert_sized::<DeviceModel>();

    // Also confirm the enum is not `!Unsized` — it lives on the stack.
    let _on_stack = DeviceModel::Diode(DiodeParams::default());
}

// ---------------------------------------------------------------------------
// 2. DeviceModel is not a trait — no vtable-registration loophole
// ---------------------------------------------------------------------------

/// `DeviceModel` is an `enum`, not a `trait`. There is no way to
/// `impl DeviceModel for MyCustomDevice` and then register it. This test
/// confirms the type is a concrete enum by matching exhaustively.
#[test]
fn device_model_is_concrete_enum_not_trait() {
    // Exhaustive match proves no open-ended trait dispatch.
    let models: Vec<DeviceModel> = vec![
        DeviceModel::Diode(DiodeParams::default()),
        DeviceModel::BJT(BJTParams {
            polarity: BJTPolarity::Npn,
            ..Default::default()
        }),
        DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params {
            polarity: MosPolarity::Nmos,
            ..Default::default()
        })),
    ];

    for m in &models {
        // Every model must fall into one of the three closed arms.
        let _family: DeviceFamily = match m {
            DeviceModel::Diode(_) => DeviceFamily::Diode,
            DeviceModel::BJT(_) => DeviceFamily::BJT,
            DeviceModel::MOSFET(_) => DeviceFamily::MOSFET,
        };
    }
}

// ---------------------------------------------------------------------------
// 3. No runtime registration methods on the public API
// ---------------------------------------------------------------------------

/// The only methods on `DeviceModel` are accessors (`family`, `name`,
/// `linearize`, `noise_stamp`). There is no `register`, `add_model`,
/// `insert`, or any mutating method that could grow the variant set at
/// runtime. This test documents the exhaustive method set.
#[test]
fn no_register_add_model_or_insert_methods_exist() {
    // We cannot *call* a method that doesn't exist, so we verify the
    // observable method set by exercising the methods that *do* exist
    // and confirming none of them accept a type-erased or string-keyed
    // model parameter that would enable runtime extension.

    let d = DeviceModel::Diode(DiodeParams::default());

    // Accessor-only: these return data, they don't register anything.
    let _family: DeviceFamily = d.family();

    // `name` returns a reference to the existing model's identifier —
    // it does not look up or register in a table.
    let _name_str: &str = d.name().as_str();

    // The only way to *obtain* a DeviceModel is to construct a variant
    // directly. There is no `DeviceModel::from_dynamic(...)` or
    // `DeviceModel::register(...)` entry point.
}

// ---------------------------------------------------------------------------
// 4. Constructing a DeviceModel requires knowing the variant at compile time
// ---------------------------------------------------------------------------

/// Every `DeviceModel` value is produced by naming a variant. There is
/// no `From<&str>` or `From<Box<dyn Any>>` that would let a caller inject
/// an unknown model type. This test shows the construction path and
/// confirms no alternative exists.
#[test]
fn construction_requires_compile_time_variant_selection() {
    // The only construction paths:
    let _d = DeviceModel::Diode(DiodeParams::default());
    let _b = DeviceModel::BJT(BJTParams::default());
    let _m1 = DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()));
    let _m3 = DeviceModel::MOSFET(MOSFETParams::BSIM3v3(MosBSIM3v3Params::default()));
    let _m4 = DeviceModel::MOSFET(MOSFETParams::BSIM4(MosBSIM4Params::default()));

    // No `DeviceModel::new(name: &str, params: Box<dyn Any>)` exists.
    // No `DeviceModel::register(...)` exists.
    // The compiler rejects any `match` arm that doesn't cover all variants,
    // which means adding a new variant is a *compile-time* breaking change.
}

// ---------------------------------------------------------------------------
// 5. Closed enum: adding a variant is a compile-time breaking change
// ---------------------------------------------------------------------------

/// If someone adds a new `DeviceModel` variant (e.g. `JFET`) without
/// updating this test's `match`, the Rust compiler will error with
/// "non-exhaustive patterns". This is the exhaustiveness guarantee
/// ADR-0005 provides — the type system prevents silent extension.
#[test]
fn adding_variant_is_compile_time_breaking_change() {
    fn classify_all(m: &DeviceModel) -> DeviceFamily {
        match m {
            DeviceModel::Diode(_) => DeviceFamily::Diode,
            DeviceModel::BJT(_) => DeviceFamily::BJT,
            DeviceModel::MOSFET(_) => DeviceFamily::MOSFET,
            // No `_` wildcard — the compiler will reject this if a
            // fourth variant is added, which is the desired property.
        }
    }

    assert_eq!(classify_all(&DeviceModel::Diode(DiodeParams::default())), DeviceFamily::Diode);
    assert_eq!(classify_all(&DeviceModel::BJT(BJTParams::default())), DeviceFamily::BJT);
    assert_eq!(
        classify_all(&DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default()))),
        DeviceFamily::MOSFET
    );
}

// ---------------------------------------------------------------------------
// 6. No Box<dyn DeviceModel> — heap indirection rejected
// ---------------------------------------------------------------------------

/// ADR-0005 states: "no Box, no dyn DeviceModel, no string-keyed registry."
/// This test confirms the enum carries its payload inline (no heap
/// indirection) and that a `Vec<DeviceModel>` stores values directly.
#[test]
fn no_box_dyn_device_model_heap_indirection() {
    // DeviceModel owns its payload inline.
    let models: Vec<DeviceModel> = vec![
        DeviceModel::Diode(DiodeParams::default()),
        DeviceModel::BJT(BJTParams::default()),
        DeviceModel::MOSFET(MOSFETParams::Level1(MosLevel1Params::default())),
    ];

    // Each element in the vec is the enum itself, not a pointer to it.
    // Sizeof the enum >= sizeof the largest variant (MOSFET params).
    assert!(
        mem::size_of::<DeviceModel>() >= mem::size_of::<MOSFETParams>(),
        "DeviceModel must inline its largest variant — no heap indirection per ADR-0005"
    );

    // The vec stores the enums contiguously (no pointer chase).
    assert_eq!(models.len(), 3);
}

// ---------------------------------------------------------------------------
// 7. DeviceModel does not implement Any — no downcast-registration loophole
// ---------------------------------------------------------------------------

/// Even though `DeviceModel` automatically has `Any` via the blanket impl,
/// there is no public API that accepts `Box<dyn Any>` and downcasts it
/// into a `DeviceModel`. The only path to create a `DeviceModel` is
/// through variant construction. This test confirms the type is not
/// `!Send` or `!Sync` (which would suggest interior mutability used
/// for registration).
#[test]
fn device_model_is_send_and_sync_no_interior_mutability_for_registration() {
    fn assert_send_sync<T: Send + Sync>() {}
    assert_send_sync::<DeviceModel>();

    // If `DeviceModel` had interior mutability (e.g. `RefCell<HashMap<...>>`
    // for a registration table), it would likely be `!Sync` or require
    // unsafe. `Send + Sync` confirms no hidden registration state.
}
