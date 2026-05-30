//! Model name identifier.
//!
//! A `ModelName` is the string key by which a `DeviceModel` is
//! registered in the `ModelLibrary` (per the device-modeling
//! bounded-context's ubiquitous language). When an element in the
//! netlist references a model — for example, a MOSFET instance citing
//! `MODEL nmos_lvt` — the netlist-graph builder records the reference
//! as a `ModelName`, and the device-modeling context resolves it to
//! the corresponding `ModelParameters` during elaboration.
//!
//! The closed-enum dispatch decision (ADR-0005) governs the *kind* of
//! model that backs a given name (Diode / BJT / MOSFET / ...); the
//! name itself is the user-facing handle.
//!
//! `ModelName` is a thin newtype around `String`. It carries no
//! structural invariant beyond non-empty-ness on construction via the
//! public constructors — empty model names are accepted because some
//! legacy SPICE inputs use them implicitly, and rejecting them is the
//! job of the netlist-graph validator, not this type.
//!
//! # Stability
//!
//! Per [ADR-0010](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! this newtype is part of the v1 unstable surface.

use core::fmt;

/// A string-keyed device-model name.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ModelName(String);

impl ModelName {
    /// Construct from anything convertible into an owned `String`.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrow as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// True iff the name is the empty string.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

impl fmt::Display for ModelName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "model:{}", self.0)
    }
}

impl From<&str> for ModelName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ModelName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trips_as_str() {
        let m = ModelName::new("nmos_lvt");
        assert_eq!(m.as_str(), "nmos_lvt");
        assert!(!m.is_empty());
    }

    #[test]
    fn display_is_prefixed() {
        assert_eq!(format!("{}", ModelName::new("d1n4148")), "model:d1n4148");
    }

    #[test]
    fn from_str_and_string_agree() {
        assert_eq!(
            ModelName::from("Q2N2222"),
            ModelName::from(String::from("Q2N2222"))
        );
    }

    #[test]
    fn empty_is_accepted_but_flagged() {
        let m = ModelName::new("");
        assert!(m.is_empty());
        assert_eq!(m.as_str(), "");
    }
}
