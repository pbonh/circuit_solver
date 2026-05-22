//! Element kinds, terminal lists, and instance parameters carried by a
//! `Branch` in the `CircuitGraph`.
//!
//! Per the `netlist-graph` bounded context's ubiquitous language:
//!
//! - An **Element** is a circuit component with terminals, an optional
//!   `ModelName` reference (resolved by the device-modeling context),
//!   and instance parameters.
//! - A **Terminal** is one pin of an element mapped to a `NodeId`.
//!
//! The closed-enum [`ElementKind`] enumerates the element categories
//! recognized by the netlist-graph context. The Gherkin scenario this
//! crate enables exercises only `Resistor` and `VoltageSource`; the
//! remaining variants are placeholders that downstream tasks (#7..#15)
//! refine via the `device-modeling` crate's `DeviceModel` enum
//! (ADR-0005). They are listed here so the builder's API can already
//! accept future kinds without an API break — consistent with
//! ADR-0010's unstable-v1 surface stance.
//!
//! # Stability
//!
//! Per [ADR-0010](../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md)
//! this type is part of the v1 unstable surface.

use circuit_solver_types::{ElementId, ModelName, NodeId};
use core::fmt;

/// The element-name string the user supplied in the netlist (e.g.
/// `"R1"`, `"V1"`, `"M3"`). Names are case-sensitive and unique within
/// a single `CircuitBuilder` scope.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ElementName(String);

impl ElementName {
    /// Construct from anything convertible into an owned String.
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

impl fmt::Display for ElementName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<&str> for ElementName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for ElementName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

/// The closed enumeration of element kinds the `netlist-graph` context
/// recognizes. Each variant carries the structural attributes the
/// builder needs to record at netlist-construction time; per-variant
/// device-physics parameters (Shockley `Is`, `Beta_F`, ...) live in
/// the `device-modeling` crate's `DeviceModel` enum and are resolved
/// from the optional `ModelName` reference during elaboration.
#[derive(Debug, Clone, PartialEq)]
#[non_exhaustive]
pub enum ElementKind {
    /// A linear resistor of `resistance_ohms` ohms between its two
    /// terminals.
    Resistor {
        /// Resistance value in ohms (Ω).
        resistance_ohms: f64,
    },
    /// A linear capacitor of `capacitance_farads` farads between its
    /// two terminals.
    Capacitor {
        /// Capacitance value in farads (F).
        capacitance_farads: f64,
    },
    /// A linear inductor of `inductance_henries` henries between its
    /// two terminals.
    Inductor {
        /// Inductance value in henries (H).
        inductance_henries: f64,
    },
    /// An ideal independent voltage source of `voltage_volts` volts
    /// from terminal 0 (positive) to terminal 1 (negative).
    VoltageSource {
        /// DC value in volts (V).
        voltage_volts: f64,
    },
    /// An ideal independent current source of `current_amperes`
    /// amperes flowing from terminal 0 into terminal 1.
    CurrentSource {
        /// DC value in amperes (A).
        current_amperes: f64,
    },
    /// A semiconductor device whose physics is resolved through a
    /// `ModelName` (carried by the enclosing [`Element`]) against the
    /// device-modeling context. The terminal count is determined by
    /// the resolved [`circuit_solver_types::ModelName`] (2 for a diode,
    /// 3 for a BJT, 4 for a MOSFET).
    Semiconductor,
    /// An instance of a previously-registered subcircuit; flattened
    /// away during `CircuitBuilder::expand_subcircuits`. After
    /// expansion the resulting `CircuitGraph` contains no
    /// `SubcircuitInstance` variants.
    SubcircuitInstance {
        /// Name of the subcircuit definition this instance points at.
        definition: SubcircuitName,
    },
}

impl ElementKind {
    /// True iff this kind expects exactly two terminals (the common
    /// case for resistors, capacitors, inductors, two-terminal voltage
    /// and current sources).
    #[must_use]
    pub const fn is_two_terminal(&self) -> bool {
        matches!(
            self,
            Self::Resistor { .. }
                | Self::Capacitor { .. }
                | Self::Inductor { .. }
                | Self::VoltageSource { .. }
                | Self::CurrentSource { .. }
        )
    }

    /// Short tag used in `Display` and error messages.
    #[must_use]
    pub const fn tag(&self) -> &'static str {
        match self {
            Self::Resistor { .. } => "R",
            Self::Capacitor { .. } => "C",
            Self::Inductor { .. } => "L",
            Self::VoltageSource { .. } => "V",
            Self::CurrentSource { .. } => "I",
            Self::Semiconductor => "DEV",
            Self::SubcircuitInstance { .. } => "X",
        }
    }
}

impl fmt::Display for ElementKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.tag())
    }
}

/// A located instance of a circuit element in the `CircuitGraph`. An
/// `Element` carries its kind, the ordered list of [`NodeId`]
/// terminals (assigned by the builder during `build()`), and an
/// optional `ModelName` reference. Terminal *order is meaningful*:
/// terminal 0 is the positive pin of a voltage source, terminal 0 is
/// the drain of a MOSFET, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Element {
    id: ElementId,
    name: ElementName,
    kind: ElementKind,
    terminals: Vec<NodeId>,
    model: Option<ModelName>,
}

impl Element {
    /// Construct an `Element`. Crate-private; user code obtains
    /// `Element`s only through `CircuitGraph` queries.
    pub(crate) fn new(
        id: ElementId,
        name: ElementName,
        kind: ElementKind,
        terminals: Vec<NodeId>,
        model: Option<ModelName>,
    ) -> Self {
        Self {
            id,
            name,
            kind,
            terminals,
            model,
        }
    }

    /// The stable element identifier assigned during `build()`.
    #[must_use]
    pub const fn id(&self) -> ElementId {
        self.id
    }

    /// The user-supplied element name (e.g. `"R1"`).
    #[must_use]
    pub fn name(&self) -> &ElementName {
        &self.name
    }

    /// The element kind discriminator and its structural attributes.
    #[must_use]
    pub fn kind(&self) -> &ElementKind {
        &self.kind
    }

    /// The ordered list of `NodeId` terminals.
    #[must_use]
    pub fn terminals(&self) -> &[NodeId] {
        &self.terminals
    }

    /// The optional model-name reference resolved by device-modeling.
    #[must_use]
    pub fn model(&self) -> Option<&ModelName> {
        self.model.as_ref()
    }
}

/// Identifier for a registered subcircuit definition.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SubcircuitName(String);

impl SubcircuitName {
    /// Construct from anything convertible into an owned String.
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

impl fmt::Display for SubcircuitName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "subckt:{}", self.0)
    }
}

impl From<&str> for SubcircuitName {
    fn from(s: &str) -> Self {
        Self::new(s)
    }
}

impl From<String> for SubcircuitName {
    fn from(s: String) -> Self {
        Self::new(s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_terminal_kinds_are_classified() {
        assert!(ElementKind::Resistor {
            resistance_ohms: 1.0
        }
        .is_two_terminal());
        assert!(ElementKind::VoltageSource { voltage_volts: 5.0 }.is_two_terminal());
        assert!(!ElementKind::Semiconductor.is_two_terminal());
        assert!(!ElementKind::SubcircuitInstance {
            definition: SubcircuitName::new("INV"),
        }
        .is_two_terminal());
    }

    #[test]
    fn tags_are_spice_letters() {
        assert_eq!(
            ElementKind::Resistor {
                resistance_ohms: 1.0
            }
            .tag(),
            "R"
        );
        assert_eq!(ElementKind::VoltageSource { voltage_volts: 5.0 }.tag(), "V");
        assert_eq!(ElementKind::Semiconductor.tag(), "DEV");
    }

    #[test]
    fn element_name_round_trips() {
        let n = ElementName::from("R1");
        assert_eq!(n.as_str(), "R1");
        assert_eq!(format!("{n}"), "R1");
    }

    #[test]
    fn subcircuit_name_display_is_prefixed() {
        assert_eq!(format!("{}", SubcircuitName::new("INV")), "subckt:INV");
    }
}
