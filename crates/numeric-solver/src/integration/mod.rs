//! Implicit-integration companion models for reactive elements.
//!
//! This module owns the *time-discretization* of reactive (memory-bearing)
//! elements in the netlist — `Capacitor` and `Inductor` from
//! `netlist_graph::ElementKind` —
//! and produces the per-timestep [`CompanionStamp`]
//! (a Norton-equivalent conductance + history current source) that
//! the Pass-2 MNA assembler folds into the MNA matrix at the *new*
//! timestep alongside the algebraic stamps from resistors, sources,
//! and the nonlinear-device `LinearizedModel` family produced by the
//! `device-modeling` crate.
//!
//! # Why companion models live here, not in `device-modeling`
//!
//! The `device-modeling` crate handles *nonlinear* devices
//! (`Diode`, `BJT`, `MOSFET`) — each is described by an algebraic
//! constitutive law and a Newton-Raphson Jacobian, and the
//! linearization is *memoryless* (it depends only on the current
//! iterate's terminal voltages). Reactive elements are different:
//! they are *linear with memory* — the stamp at timestep `n+1` depends
//! on the state at timestep `n`, and the discretization
//! (Backward Euler vs. Trapezoidal vs. Gear-2 BDF) is an
//! analysis-orchestration concern, not a device-physics concern. The
//! same capacitor element produces a different stamp under each
//! integration method.
//!
//! Per `design.md` (lines 111–112, ADR-0003), the transient sub-view
//! is where "companion-model stamps for transient" attach to the
//! flattened structure. Sibling tasks #30 (Trapezoidal) and #31
//! (Gear-2 BDF) will add their own per-method module alongside
//! [`backward_euler`] here, all returning the same
//! [`CompanionStamp`] shape so the MNA assembler can fold any method's
//! output identically.
//!
//! # Tasks-md slicing
//!
//! This task (tasks.md **#29**) lands:
//!
//! 1. The shared companion-model types ([`CompanionStamp`],
//!    [`CapacitorHistory`], [`InductorHistory`]) used by every
//!    implicit method.
//! 2. The Backward Euler bodies for capacitor and inductor
//!    ([`backward_euler::capacitor_companion`],
//!    [`backward_euler::inductor_companion`]) plus their history
//!    advancers.
//!
//! Per task #29's "depends on #8" header, the dispatch surface
//! conventions established by `device-modeling::stamp` (closed-enum
//! per-family helpers, terminal-local coordinates,
//! `let _ = …` placeholder hygiene, exhaustive `match`) are mirrored
//! here. There is *no* enum dispatch on `ElementKind` in this task,
//! though — the MNA assembler (tasks.md **#14**) decides per element
//! which of [`backward_euler::capacitor_companion`] /
//! [`backward_euler::inductor_companion`] / sibling methods to call.
//! Adding integration methods does **not** add `ElementKind` arms;
//! adding reactive elements (e.g. mutual inductors in a future
//! change) is what would.
//!
//! Sibling tasks within the same change deliver their analogues
//! under the *same* module shape:
//!
//! - tasks.md **#30** — `trapezoidal::{capacitor_companion, inductor_companion}`,
//! - tasks.md **#31** — `gear2_bdf::{capacitor_companion, inductor_companion}`,
//! - tasks.md **#32** — adaptive LTE estimator that depends on the
//!   per-method companion outputs.
//!
//! # Stability
//!
//! Per [ADR-0010](../../../wiki/decisions/0010-unstable-public-rust-api-surface-for-v1.md)
//! the public API surface is **unstable** at v1.0.0. Consumers must
//! pin to exact versions until a future stabilization ADR.

pub mod backward_euler;
pub mod companion;

pub use backward_euler::{
    advance_capacitor_history, advance_inductor_history, capacitor_companion, inductor_companion,
    CompanionInputError,
};
pub use companion::{CapacitorHistory, CompanionStamp, InductorHistory};
