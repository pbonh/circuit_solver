//! Branch identifier newtype.
//!
//! In Modified Nodal Analysis (MNA), a *branch* is a current-carrying edge
//! that augments the nodal system: voltage sources, inductors, and any
//! element whose stamp introduces a new current unknown contribute one
//! `BranchId` each. Pure conductive (admittance) elements such as
//! resistors and capacitors do **not** introduce a branch — their
//! contribution stays in the conductance matrix.
//!
//! `BranchId` is the analog dual of [`NodeId`](crate::NodeId): it is an
//! opaque, stable index assigned during Pass 1 structure flattening
//! (`tasks.md` items #3 and #6, and ADR-0003). The numeric-solver
//! context consumes it when assembling the augmented MNA system.
//!
//! # Stability
//!
//! Per [ADR-0010](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! this newtype is part of the v1 unstable surface.

use core::fmt;

/// An MNA branch identifier.
///
/// One `BranchId` is allocated per current-carrying augmentation row
/// of the MNA system. Branch indices are assigned in deterministic
/// order during flattening, independently of [`NodeId`](crate::NodeId)
/// indices.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct BranchId(u32);

impl BranchId {
    /// Wrap a raw u32 as a `BranchId`.
    #[must_use]
    pub const fn new(idx: u32) -> Self {
        Self(idx)
    }

    /// Unwrap to a raw u32.
    #[must_use]
    pub const fn index(self) -> u32 {
        self.0
    }
}

impl fmt::Display for BranchId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "branch:{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn branch_display_is_prefixed() {
        assert_eq!(format!("{}", BranchId::new(0)), "branch:0");
        assert_eq!(format!("{}", BranchId::new(42)), "branch:42");
    }

    #[test]
    fn branch_round_trips_through_index() {
        let b = BranchId::new(7);
        assert_eq!(b.index(), 7);
    }

    #[test]
    fn branches_order_by_index() {
        let mut ids = [BranchId::new(3), BranchId::new(1), BranchId::new(2)];
        ids.sort();
        assert_eq!(ids, [BranchId::new(1), BranchId::new(2), BranchId::new(3)]);
    }
}
