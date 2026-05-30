//! Compile-time macro/codegen seam for closed-enum device models (ADR-0007).
//!
//! This module provides the [`define_device_families!`] declarative macro
//! that generates the full closed-enum device-model type system from a
//! compact family declaration. Adding a new device family requires only
//! adding a block to the macro invocation — the macro generates the enum
//! variants, the dispatch methods, the linearization structs, and the
//! terminal count constants automatically.
//!
//! # ADR-0007 rationale
//!
//! ADR-0005 chose a closed `enum DeviceModel` for zero-cost
//! monomorphized dispatch, accepting that each new model is a
//! breaking recompile. ADR-0007 refines this by introducing a
//! compile-time macro/codegen seam that generates family variants
//! into the closed enums, preserving the ADR-0005 invariant (static
//! dispatch, no runtime registration) while scaling the model
//! library without hand-writing each variant's boilerplate.
//!
//! # Macro overview
//!
//! [`define_device_families!`] takes a list of family specifications
//! and generates the complete type surface:
//!
//! - `DeviceFamily` enum (one variant per family)
//! - `DeviceModel` enum (one variant per family, carrying its
//!   parameter struct inline)
//! - `OperatingPoint` enum (one variant per family, carrying
//!   terminal voltages)
//! - `LinearizedModel` enum (one variant per family, carrying
//!   the family's linearization struct)
//! - Per-family linearization structs (specified by name)
//! - Per-family terminal count constants (specified by name)
//! - Exhaustive `match` dispatch on `DeviceModel::family()`,
//!   `DeviceModel::name()`, `OperatingPoint::terminal_count()`,
//!   and `LinearizedModel::terminal_count()`
//!
//! # Example
//!
//! ```rust,ignore
//! define_device_families! {
//!     /// Two-terminal junction diode
//!     Diode => {
//!         terminals: 2,
//!         terminals_const: DIODE_TERMINALS,
//!         params: DiodeParams,
//!         linearization: DiodeLinearization,
//!         linearize_fn: linearize_diode,
//!     },
//!     /// Bipolar junction transistor
//!     BJT => {
//!         terminals: 3,
//!         terminals_const: BJT_TERMINALS,
//!         params: BJTParams,
//!         linearization: BJTLinearization,
//!         linearize_fn: linearize_bjt,
//!     },
//!     /// MOS field-effect transistor
//!     MOSFET => {
//!         terminals: 4,
//!         terminals_const: MOSFET_TERMINALS,
//!         params: MOSFETParams,
//!         linearization: MOSFETLinearization,
//!         linearize_fn: linearize_mosfet,
//!     },
//! }
//! ```
//!
//! # Adding a new family
//!
//! To add a new device family (e.g. JFET):
//!
//! 1. Define the parameter struct (`JFETParams`).
//! 2. Add a block to the `define_device_families!` invocation:
//!
//! ```rust,ignore
//! JFET => {
//!     terminals: 3,
//!     terminals_const: JFET_TERMINALS,
//!     params: JFETParams,
//!     linearization: JFETLinearization,
//!     linearize_fn: linearize_jfet,
//! },
//! ```
//!
//! 3. Implement the per-family `linearize_fn` function.
//! 4. Implement the `Linearization::zero()` constructor for the new
//!    linearization struct.
//!
//! The macro regenerates all enum variants and match arms, so the
//! compile-time exhaustiveness guarantee (ADR-0005) holds
//! automatically — any downstream `match` that misses the new
//! variant will fail to compile.
//!
//! # Exhaustiveness guarantee
//!
//! Adding a family to the `define_device_families!` invocation is a
//! compile-time breaking change: the generated `match` arms in every
//! dispatch method expand to cover the new variant, and any
//! downstream code that matches on the generated enums will fail to
//! compile until it handles the new variant. This is exactly the
//! property ADR-0005 exists to guarantee — the macro automates the
//! boilerplate but does not relax the exhaustiveness constraint.
//!
//! # No runtime registration
//!
//! There is no API to register device models at runtime. All variants
//! are compile-time members of the closed enums. This is a deliberate
//! design choice per ADR-0005 / ADR-0007 — the codegen seam refines
//! the closed-enum pattern without reversing its core decision.
//!
//! # Stability
//!
//! Per [ADR-0010] the public API surface is **unstable** at v1.0.0.

// ---------------------------------------------------------------------------
// define_device_families! — the core codegen macro
// ---------------------------------------------------------------------------

/// Generate the complete closed-enum device model type system from
/// inline family specifications.
///
/// This is the core of the ADR-0007 codegen seam. Given a list of
/// family specifications, it generates:
///
/// 1. Per-family terminal count constants.
/// 2. Per-family linearization structs with `jacobian` and
///    `companion_current` fields, plus a `zero()` constructor.
/// 3. The `DeviceFamily` discriminator enum.
/// 4. The `DeviceModel` closed enum with exhaustive `family()` and
///    `name()` dispatch.
/// 5. The `OperatingPoint` enum with `terminal_count()`.
/// 6. The `LinearizedModel` enum with `terminal_count()`.
/// 7. The `DeviceModel::linearize()` dispatch entry point with
///    family-mismatch error handling.
/// 8. Per-family linearize function stubs (returning zero).
///
/// # Syntax
///
/// ```rust,ignore
/// define_device_families! {
///     /// Doc comment for Diode
///     Diode => {
///         terminals: 2,
///         terminals_const: DIODE_TERMINALS,
///         params: DiodeParams,
///         linearization: DiodeLinearization,
///         linearize_fn: linearize_diode,
///     },
///     ...
/// }
/// ```
///
/// Each family block specifies:
///
/// - `terminals: N` — number of terminals (used for array sizes).
/// - `terminals_const: CONST_NAME` — name for the `pub const` terminal
///   count constant generated by the macro.
/// - `params: ParamsType` — the parameter struct type carried by the
///   `DeviceModel` variant.
/// - `linearization: LinType` — name for the linearization struct
///   generated by the macro.
/// - `linearize_fn: fn_name` — name for the per-family linearize
///   function stub generated by the macro. Implementers replace the
///   stub body with the actual device equation.
///
/// # ADR-0005 invariant
///
/// Generated variants are ordinary enum members dispatched by static
/// monomorphization. There is **no** runtime model registration API
/// — the codegen seam refines ADR-0005 without reversing its core
/// decision.
#[macro_export]
macro_rules! define_device_families {
    (
        $(
            $(#[$family_meta:meta])*
            $family:ident => {
                terminals: $terminals:literal,
                terminals_const: $terminals_const:ident,
                params: $params:ty,
                linearization: $linearization:ident,
                linearize_fn: $linearize_fn:ident $(,)?
            }
        ),* $(,)?
    ) => {
        // =================================================================
        // Terminal count constants
        // =================================================================
        $(
            /// Number of terminals on a
            #[doc = ::core::stringify!($family)]
            /// device.
            pub const $terminals_const: usize = $terminals;
        )*

        // =================================================================
        // Per-family linearization structs
        // =================================================================
        $(
            #[doc = ::core::concat!(
                ::core::stringify!($family),
                " linearization: ",
                ::core::stringify!($terminals),
                "×",
                ::core::stringify!($terminals),
                " Jacobian + ",
                ::core::stringify!($terminals),
                "-vector of companion currents."
            )]
            #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq, ::core::marker::Copy)]
            pub struct $linearization {
                /// Jacobian matrix, terminal-local.
                /// `jacobian[i][j]` is `∂I_i / ∂V_j` evaluated at the
                /// operating point.
                pub jacobian: [[f64; $terminals]; $terminals],
                /// Companion current vector, terminal-local.
                /// `companion_current[k]` is subtracted from the MNA
                /// right-hand-side at the node attached to terminal `k`.
                pub companion_current: [f64; $terminals],
            }

            impl $linearization {
                /// All-zero linearization placeholder. Device stamps
                /// that have not yet been implemented can return this
                /// to produce a valid (but zero-contribution) stamp.
                #[must_use]
                pub const fn zero() -> Self {
                    Self {
                        jacobian: [[0.0; $terminals]; $terminals],
                        companion_current: [0.0; $terminals],
                    }
                }
            }
        )*

        // =================================================================
        // DeviceFamily discriminator enum
        // =================================================================
        /// Family discriminator independent of the parameter payload.
        ///
        /// Used by callers that need to classify a device by family
        /// without destructuring the inner parameter struct — for
        /// instance, the netlist-graph topology checker (ADR-0009).
        ///
        /// Generated by [`define_device_families!`] (ADR-0007 codegen
        /// seam).
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq, ::core::cmp::Eq, ::std::hash::Hash)]
        pub enum DeviceFamily {
            $(
                $(#[$family_meta])*
                $family,
            )*
        }

        // =================================================================
        // DeviceModel closed enum
        // =================================================================
        /// Closed-enum device model dispatched on by the numeric-solver
        /// inside the Newton-Raphson stamp loop (ADR-0005, ADR-0007).
        ///
        /// Each variant carries its parameter payload by value (not
        /// by `Box` or reference), so the enum's footprint equals the
        /// discriminant plus the largest variant. Adding a new variant
        /// is a deliberate compile-time breaking change — every `match`
        /// site must be updated, which is exactly the property the
        /// closed enum exists to guarantee.
        ///
        /// This enum is **generated** by [`define_device_families!`]
        /// (ADR-0007 codegen seam). Variants are ordinary enum members
        /// dispatched by static monomorphization with no runtime
        /// registration.
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq)]
        pub enum DeviceModel {
            $(
                $(#[$family_meta])*
                $family($params),
            )*
        }

        impl DeviceModel {
            /// Family discriminator for this model.
            ///
            /// Cheap: a single `match` on the enum tag with no payload
            /// access.
            #[must_use]
            pub fn family(&self) -> DeviceFamily {
                match self {
                    $(
                        Self::$family(_) => DeviceFamily::$family,
                    )*
                }
            }

            /// Borrow this model's identifier as resolved from the
            /// netlist's `.MODEL` card.
            #[must_use]
            pub fn name(&self) -> &circuit_solver_types::ModelName {
                match self {
                    $(
                        Self::$family(p) => &p.name,
                    )*
                }
            }
        }

        // =================================================================
        // OperatingPoint enum
        // =================================================================
        /// Per-iteration terminal voltages handed to
        /// `DeviceModel::linearize`.
        ///
        /// One variant per [`DeviceModel`] family. Voltages are
        /// terminal-local absolute node voltages in SPICE canonical
        /// ordering.
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq)]
        pub enum OperatingPoint {
            $(
                $(#[$family_meta])*
                $family([f64; $terminals_const]),
            )*
        }

        impl OperatingPoint {
            /// Number of terminals carried by this operating point.
            #[must_use]
            pub fn terminal_count(&self) -> usize {
                match self {
                    $(
                        Self::$family(_) => $terminals_const,
                    )*
                }
            }
        }

        // =================================================================
        // LinearizedModel enum
        // =================================================================
        /// Family-tagged linearization returned by
        /// `DeviceModel::linearize`.
        ///
        /// Each variant carries its linearization inline per ADR-0005:
        /// no `Box`, no `dyn`. The enum is `Copy + Clone + PartialEq`.
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::marker::Copy, ::core::cmp::PartialEq)]
        pub enum LinearizedModel {
            $(
                $(#[$family_meta])*
                $family($linearization),
            )*
        }

        impl LinearizedModel {
            /// Number of terminals contributed by this linearization's
            /// stamp.
            #[must_use]
            pub fn terminal_count(&self) -> usize {
                match self {
                    $(
                        Self::$family(_) => $terminals_const,
                    )*
                }
            }
        }

        // =================================================================
        // Dispatch entry point on DeviceModel
        // =================================================================
        /// Mismatched-family error from `DeviceModel::linearize`.
        ///
        /// Returned when the supplied [`OperatingPoint`] variant does
        /// not match the [`DeviceModel`] variant being linearized.
        #[derive(::core::fmt::Debug, ::core::clone::Clone, ::core::cmp::PartialEq, ::core::cmp::Eq)]
        pub struct OperatingPointFamilyMismatch {
            /// The family carried by the DeviceModel.
            pub expected: &'static str,
            /// The family carried by the OperatingPoint.
            pub actual: &'static str,
        }

        impl ::core::fmt::Display for OperatingPointFamilyMismatch {
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(
                    f,
                    "device-model / operating-point family mismatch: device is {} but operating point is {}",
                    self.expected, self.actual
                )
            }
        }

        impl ::std::error::Error for OperatingPointFamilyMismatch {}

        impl DeviceModel {
            /// Compute the [`LinearizedModel`] for this device at the
            /// given [`OperatingPoint`].
            ///
            /// # Dispatch (ADR-0005 / ADR-0007)
            ///
            /// The implementation is a single `match` on `self`, with
            /// one arm per [`DeviceModel`] family. Each arm delegates
            /// to a per-family helper. The match is exhaustive — the
            /// Rust compiler enforces that adding a family variant
            /// breaks every site that lacks the new arm.
            ///
            /// # Errors
            ///
            /// Returns [`OperatingPointFamilyMismatch`] if `op`'s
            /// family does not match `self`'s family.
            pub fn linearize(
                &self,
                op: &OperatingPoint,
            ) -> ::core::result::Result<LinearizedModel, OperatingPointFamilyMismatch> {
                match (self, op) {
                    $(
                        (Self::$family(p), OperatingPoint::$family(v)) => {
                            ::core::result::Result::Ok(LinearizedModel::$family(
                                $linearize_fn(p, v)
                            ))
                        }
                    )*
                    // Mismatched-family arms: exhaustive per variant.
                    $(
                        (Self::$family(_), op) => {
                            ::core::result::Result::Err(OperatingPointFamilyMismatch {
                                expected: ::core::stringify!($family),
                                actual: _op_family_name(op),
                            })
                        }
                    )*
                }
            }
        }

        /// Internal helper: extract the family name from an
        /// `OperatingPoint` for error messages.
        fn _op_family_name(op: &OperatingPoint) -> &'static str {
            match op {
                $(
                    OperatingPoint::$family(_) => ::core::stringify!($family),
                )*
            }
        }

        // =================================================================
        // Per-family linearize function stubs
        // =================================================================
        $(
            #[doc = ::core::concat!(
                "Linearize a ",
                ::core::stringify!($family),
                " at the given terminal voltages.\n\n",
                "Per-family helper dispatched by `DeviceModel::linearize`.\n",
                "This stub returns an all-zero linearization. Implementers\n",
                "must replace the body with the actual device equation."
            )]
            #[must_use]
            pub fn $linearize_fn(
                _params: &$params,
                _terminal_voltages: &[f64; $terminals_const],
            ) -> $linearization {
                $linearization::zero()
            }
        )*
    };
}

// ---------------------------------------------------------------------------
// Tests — validate the codegen seam end-to-end
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    // Minimal params structs for the test families.
    #[derive(Debug, Clone, PartialEq)]
    struct TestDiodeParams {
        name: circuit_solver_types::ModelName,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct TestBJTParams {
        name: circuit_solver_types::ModelName,
    }

    // Generate the full type system for two test families.
    define_device_families! {
        /// Test diode family
        TestDiode => {
            terminals: 2,
            terminals_const: TESTDIODE_TERMINALS,
            params: TestDiodeParams,
            linearization: TestDiodeLinearization,
            linearize_fn: test_linearize_diode,
        },
        /// Test BJT family
        TestBJT => {
            terminals: 3,
            terminals_const: TESTBJT_TERMINALS,
            params: TestBJTParams,
            linearization: TestBJTLinearization,
            linearize_fn: test_linearize_bjt,
        },
    }

    #[test]
    fn codegen_generates_terminal_constants() {
        assert_eq!(TESTDIODE_TERMINALS, 2);
        assert_eq!(TESTBJT_TERMINALS, 3);
    }

    #[test]
    fn codegen_linearization_zero_is_all_zeros() {
        let d = TestDiodeLinearization::zero();
        assert_eq!(d.jacobian, [[0.0; 2]; 2]);
        assert_eq!(d.companion_current, [0.0; 2]);

        let b = TestBJTLinearization::zero();
        assert_eq!(b.jacobian, [[0.0; 3]; 3]);
        assert_eq!(b.companion_current, [0.0; 3]);
    }

    #[test]
    fn codegen_device_model_family_dispatch() {
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d1"),
        });
        assert_eq!(d.family(), DeviceFamily::TestDiode);

        let b = DeviceModel::TestBJT(TestBJTParams {
            name: circuit_solver_types::ModelName::new("q1"),
        });
        assert_eq!(b.family(), DeviceFamily::TestBJT);
    }

    #[test]
    fn codegen_device_model_name_dispatch() {
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d_test"),
        });
        assert_eq!(d.name().as_str(), "d_test");
    }

    #[test]
    fn codegen_operating_point_terminal_count() {
        let d = OperatingPoint::TestDiode([0.7, 0.0]);
        assert_eq!(d.terminal_count(), 2);

        let b = OperatingPoint::TestBJT([1.0, 0.65, 0.0]);
        assert_eq!(b.terminal_count(), 3);
    }

    #[test]
    fn codegen_linearized_model_terminal_count() {
        let d = LinearizedModel::TestDiode(TestDiodeLinearization::zero());
        assert_eq!(d.terminal_count(), 2);

        let b = LinearizedModel::TestBJT(TestBJTLinearization::zero());
        assert_eq!(b.terminal_count(), 3);
    }

    #[test]
    fn codegen_linearize_dispatch_matches_family() {
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d1"),
        });
        let op = OperatingPoint::TestDiode([0.7, 0.0]);
        let lin = d.linearize(&op).expect("matching family should succeed");
        assert!(matches!(lin, LinearizedModel::TestDiode(_)));
    }

    #[test]
    fn codegen_linearize_rejects_mismatched_family() {
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d1"),
        });
        let op = OperatingPoint::TestBJT([1.0, 0.65, 0.0]);
        let err = d.linearize(&op).expect_err("mismatched family should fail");
        assert_eq!(err.expected, "TestDiode");
        assert_eq!(err.actual, "TestBJT");
    }

    #[test]
    fn codegen_linearize_default_stub_returns_zero() {
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d1"),
        });
        let op = OperatingPoint::TestDiode([0.7, 0.0]);
        let lin = d.linearize(&op).expect("should succeed");
        match lin {
            LinearizedModel::TestDiode(dl) => {
                assert_eq!(dl.jacobian, [[0.0; 2]; 2]);
                assert_eq!(dl.companion_current, [0.0; 2]);
            }
            _ => panic!("expected TestDiode variant"),
        }
    }

    #[test]
    fn codegen_linearization_is_copy() {
        fn assert_copy<T: Copy>() {}
        assert_copy::<TestDiodeLinearization>();
        assert_copy::<TestBJTLinearization>();
        assert_copy::<LinearizedModel>();
        assert_copy::<OperatingPoint>();
    }

    #[test]
    fn codegen_closed_enum_match_is_exhaustive() {
        // ADR-0005 compliance: an unannotated match covers all variants.
        // The Rust compiler enforces this; this test pins the intent.
        fn classify(m: &DeviceModel) -> DeviceFamily {
            match m {
                DeviceModel::TestDiode(_) => DeviceFamily::TestDiode,
                DeviceModel::TestBJT(_) => DeviceFamily::TestBJT,
            }
        }
        let d = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d"),
        });
        assert_eq!(classify(&d), DeviceFamily::TestDiode);

        let b = DeviceModel::TestBJT(TestBJTParams {
            name: circuit_solver_types::ModelName::new("q"),
        });
        assert_eq!(classify(&b), DeviceFamily::TestBJT);
    }

    #[test]
    fn codegen_device_model_clone_independently() {
        let original = DeviceModel::TestDiode(TestDiodeParams {
            name: circuit_solver_types::ModelName::new("d_orig"),
        });
        let cloned = original.clone();
        assert_eq!(original, cloned);
    }

    #[test]
    fn codegen_family_mismatch_error_display() {
        let err = OperatingPointFamilyMismatch {
            expected: "TestDiode",
            actual: "TestBJT",
        };
        let msg = format!("{err}");
        assert!(msg.contains("TestDiode"));
        assert!(msg.contains("TestBJT"));
    }

    #[test]
    fn codegen_no_dyn_no_box_layout_witness() {
        // ADR-0005 negative consequence: the enum inlines its largest
        // variant. Verify that DeviceModel is Sized and inlines the
        // TestBJT variant (3-f64 array in the operating point means
        // TestBJT is larger than TestDiode).
        fn assert_sized<T: Sized>() {}
        assert_sized::<DeviceModel>();

        assert!(
            ::core::mem::size_of::<DeviceModel>()
                >= ::core::mem::size_of::<TestBJTParams>(),
            "DeviceModel must inline its largest variant per ADR-0005"
        );
    }
}
