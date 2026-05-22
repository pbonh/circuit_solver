//! Opaque identifier newtypes used across the workspace.
//!
//! This module hosts the identifier newtypes that name *located* things
//! in the netlist — circuit nodes, elements, and boundary signals:
//!
//! - `NodeId` — an analog circuit node,
//! - `ElementId` — an analog circuit element,
//! - `SignalName` — a string-keyed boundary signal name shared between
//!   the analog solver and the digital simulator (e.g. `"vout"` driving
//!   `"din"`).
//!
//! The MNA `BranchId` lives in the sibling [`crate::branch`] module
//! because branches are an MNA-system construct rather than a netlist
//! locator; `ModelName` lives in [`crate::model`] for the same reason
//! (it names a *kind* of device, not a located instance).

use core::fmt;

/// An analog circuit node, identified by a stable index assigned during
/// flattening (tasks.md item #6). Node 0 is conventionally ground.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(u32);

impl NodeId {
    /// The reserved ground node identifier.
    pub const GROUND: Self = Self(0);

    /// Wrap a raw u32 as a `NodeId`.
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

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Self::GROUND {
            write!(f, "node:GND")
        } else {
            write!(f, "node:{}", self.0)
        }
    }
}

/// An analog circuit element identifier (resistor, capacitor, device, ...).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementId(u32);

impl ElementId {
    /// Wrap a raw u32 as an `ElementId`.
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

impl fmt::Display for ElementId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "elem:{}", self.0)
    }
}

/// A named boundary signal exchanged between the analog solver and the
/// digital simulator (e.g., `"vout"` -> `"din"` per the
/// `analog-digital-boundary-signal-exchange` scenario).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SignalName(String);

impl SignalName {
    /// Construct from an owned String. The signal name carries no
    /// internal structure here; the bounded-context's ubiquitous
    /// language treats it as an opaque identifier exchanged through
    /// the scheduler.
    #[must_use]
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }

    /// Borrow as a `&str`.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl fmt::Display for SignalName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for SignalName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SignalName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_ground_displays_distinctly() {
        assert_eq!(format!("{}", NodeId::GROUND), "node:GND");
        assert_eq!(format!("{}", NodeId::new(1)), "node:1");
    }

    #[test]
    fn signal_name_roundtrips() {
        let s = SignalName::new("vout");
        assert_eq!(s.as_str(), "vout");
        assert_eq!(SignalName::from("vout"), s);
    }
}
