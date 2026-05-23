//! Analog-digital boundary signal exchanger.
//!
//! Per **ADR-0007** ("Zero-Order Hold Default at Analog-Digital
//! Boundary"), the analog and digital simulator kernels exchange named
//! boundary signal values at every synchronization point T. Because
//! the analog solver's adaptive timesteps rarely land exactly on a
//! digital event time, an interpolation scheme is required.
//!
//! This module implements the **zero-order hold (ZOH)** default
//! mandated by ADR-0007: at the synchronization point T, each
//! exchanged value is the *most recent accepted value at or before T*
//! from the providing side. No interpolation is performed; the value
//! is held constant from its last sample until T. This is
//! charge-conserving by construction (constant voltage/current over
//! the hold interval implies zero injected charge) and avoids the
//! need to retain historical solution vectors.
//!
//! # Scope (tasks.md item #45)
//!
//! This task implements the **default ZOH path** and the
//! [`BoundarySignalExchanger`] component. The opt-in **linear
//! interpolation** mode named in ADR-0007 is reserved for the sibling
//! tasks.md item #46. The [`BoundaryInterpolationMode`] enum exposes
//! the `Linear` variant so item #46 can extend behaviour without a
//! breaking change to this module's public types, but constructing an
//! exchanger in `Linear` mode is rejected here with a clear error;
//! the linear path lives in #46.
//!
//! # The scenario this module satisfies
//!
//! `mixed-signal-cosim#analog-digital-boundary-signal-exchange`:
//!
//! ```gherkin
//! Given SimulationEngineer has configured boundary signals:
//!   analog output "vout" driving digital input "din"
//!   and digital output "dout" driving analog input "vin"
//! When the Scheduler reaches a synchronization point at time T
//! Then the analog solver provides the value of "vout" at time T to
//!   the digital simulator as "din"
//! And the digital simulator provides the value of "dout" at time T to
//!   the analog solver as "vin"
//! And both simulators proceed from time T with the exchanged
//!   boundary values
//! ```
//!
//! # Composition with the Mixed-Signal Scheduler
//!
//! The exchanger is intentionally a standalone component. The
//! [`MixedSignalScheduler`](crate::MixedSignalScheduler) at task #42
//! holds a [`BoundarySignals`] configuration
//! but does not yet wire it through an exchanger in its `run()` loop
//! (sibling tasks #47, #48 will). The exchanger is therefore exposed
//! as a directly callable component so:
//!
//! - the sibling Icarus Verilog (#47) and Verilator (#48) adapter
//!   integrations can drive it at each synchronization point, and
//! - the linear-interpolation extension (#46) can swap or augment
//!   its interpolation strategy without touching the scheduler.

use circuit_solver_types::SignalName;
use core::fmt;

use crate::BoundarySignals;

// ---------------------------------------------------------------------------
// Interpolation mode
// ---------------------------------------------------------------------------

/// Per-request boundary-interpolation policy named by ADR-0007.
///
/// The default ([`Self::ZeroOrderHold`]) is the focus of tasks.md item
/// #45. The [`Self::Linear`] variant is exposed for forward
/// compatibility with tasks.md item #46 — its semantics
/// (retain-two-vectors + linear interpolation at the event time) are
/// defined in ADR-0007 but are out of scope for this task.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoundaryInterpolationMode {
    /// Hold the last accepted analog (or digital) value constant from
    /// its sample time until the synchronization point T. The default
    /// per ADR-0007. Charge-conserving; requires no historical state.
    #[default]
    ZeroOrderHold,
    /// Linearly interpolate between the two most recent samples at the
    /// synchronization point T. Reserved for tasks.md item #46; not
    /// supported by the exchanger constructed at item #45.
    Linear,
}

impl fmt::Display for BoundaryInterpolationMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ZeroOrderHold => "zero_order_hold",
            Self::Linear => "linear",
        };
        f.write_str(s)
    }
}

// ---------------------------------------------------------------------------
// Provider traits
// ---------------------------------------------------------------------------

/// Returns the last accepted analog value for a named boundary signal.
///
/// The exchanger calls this at every synchronization point to obtain
/// the analog value that should be delivered to the digital simulator
/// per the ZOH discipline ("the analog value at the last accepted
/// timestep is held constant until the digital event time", ADR-0007
/// Y-statement).
///
/// Implementations live in the numeric-solver crate (which knows the
/// solution vector) and in the test doubles below.
pub trait AnalogValueProvider {
    /// Most recent accepted analog value for `signal`, or `None` if no
    /// sample has been accepted yet (e.g., at t=0 before the first
    /// solve). On the ZOH path, "most recent" means strictly the last
    /// accepted sample; no interpolation occurs.
    fn last_analog_value(&self, signal: &SignalName) -> Option<f64>;
}

/// Returns the last digital output value for a named boundary signal.
///
/// In an event-driven digital simulator a signal holds a discrete
/// value between events; ZOH for the digital→analog direction is the
/// natural representation. The returned `f64` is the *logical level*
/// scaled to the analog domain (the adapter chooses the encoding;
/// e.g., 0.0 V for logic-0 and 3.3 V for logic-1).
pub trait DigitalValueProvider {
    /// Most recent digital value for `signal`, or `None` if the
    /// simulator has not yet produced one.
    fn last_digital_value(&self, signal: &SignalName) -> Option<f64>;
}

// ---------------------------------------------------------------------------
// Exchange packet
// ---------------------------------------------------------------------------

/// The set of values exchanged at a single synchronization point T.
///
/// Each direction is a parallel `Vec<(SignalName, f64)>` matching the
/// configured [`BoundarySignals`] pairs:
///
/// - `analog_to_digital[i] == (digital_input_name, analog_value)`
///   where `digital_input_name` is the *destination* name (the digital
///   simulator's input). The value carries the analog
///   solver's `vout` at the ZOH-resolved time.
/// - `digital_to_analog[i] == (analog_input_name, digital_value)`
///   where `analog_input_name` is the *destination* name (the analog
///   solver's input). The value carries the digital simulator's
///   `dout` at the ZOH-resolved time.
///
/// Entries for signals with no available source value are omitted and
/// recorded as `missing_sources` so the caller can decide whether to
/// proceed with a default, raise a diagnostic, or abort.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundaryExchangePacket {
    /// Analog → digital values, keyed by the *destination* (digital
    /// input) name.
    pub analog_to_digital: Vec<(SignalName, f64)>,
    /// Digital → analog values, keyed by the *destination* (analog
    /// input) name.
    pub digital_to_analog: Vec<(SignalName, f64)>,
    /// Names of *source* signals for which no value was available at
    /// the synchronization point. The exchanger does not invent a
    /// default; it surfaces the gap so the scheduler / adapter can
    /// decide.
    pub missing_sources: Vec<SignalName>,
}

impl BoundaryExchangePacket {
    /// True iff every configured pair produced a value.
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.missing_sources.is_empty()
    }

    /// Look up the value delivered to a named digital input.
    #[must_use]
    pub fn analog_to_digital_value(&self, dest: &SignalName) -> Option<f64> {
        self.analog_to_digital
            .iter()
            .find(|(name, _)| name == dest)
            .map(|(_, v)| *v)
    }

    /// Look up the value delivered to a named analog input.
    #[must_use]
    pub fn digital_to_analog_value(&self, dest: &SignalName) -> Option<f64> {
        self.digital_to_analog
            .iter()
            .find(|(name, _)| name == dest)
            .map(|(_, v)| *v)
    }
}

// ---------------------------------------------------------------------------
// Exchanger errors
// ---------------------------------------------------------------------------

/// Errors raised by [`BoundarySignalExchanger`] construction or use.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BoundaryExchangerError {
    /// The requested interpolation mode is not supported by this task's
    /// exchanger. `Linear` is reserved for tasks.md item #46.
    UnsupportedMode(BoundaryInterpolationMode),
}

impl fmt::Display for BoundaryExchangerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::UnsupportedMode(mode) => write!(
                f,
                "boundary interpolation mode {mode} is not supported by the \
                 zero-order-hold exchanger (tasks.md item #45); linear \
                 interpolation is implemented by item #46"
            ),
        }
    }
}

impl std::error::Error for BoundaryExchangerError {}

// ---------------------------------------------------------------------------
// BoundarySignalExchanger
// ---------------------------------------------------------------------------

/// Exchanges named boundary signal values between the analog solver
/// and the digital simulator at every synchronization point, using the
/// **zero-order hold** discipline by default per ADR-0007.
///
/// # Lifecycle
///
/// 1. Construct via [`BoundarySignalExchanger::zero_order_hold`] with
///    the configured [`BoundarySignals`] pairs.
/// 2. At each synchronization point T, call
///    [`BoundarySignalExchanger::exchange`] with handles to the analog
///    and digital value providers. The returned
///    [`BoundaryExchangePacket`] is delivered to the destination
///    kernels by the caller (typically the scheduler or an adapter).
///
/// # Why ZOH and not interpolation
///
/// ADR-0007 commits the v1 default to ZOH because:
///
/// - It is **charge-conserving by construction**: holding a voltage or
///   current constant over the hold interval injects zero additional
///   charge at the boundary.
/// - It is **simple and stateless**: only the most recent sample is
///   needed; no historical solution vectors must be retained.
/// - It **matches SPICE convention** for event-driven stimuli,
///   reducing surprise for users migrating from ngspice or similar.
///
/// The opt-in linear path (tasks.md item #46) accepts a charge-
/// conservation tradeoff in exchange for smoother boundary waveforms;
/// users who need that path must opt in explicitly.
#[derive(Debug, Clone, PartialEq)]
pub struct BoundarySignalExchanger {
    boundaries: BoundarySignals,
    mode: BoundaryInterpolationMode,
}

impl BoundarySignalExchanger {
    /// Construct a ZOH exchanger over the given boundary signal
    /// configuration. The default mode per ADR-0007.
    #[must_use]
    pub fn zero_order_hold(boundaries: BoundarySignals) -> Self {
        Self {
            boundaries,
            mode: BoundaryInterpolationMode::ZeroOrderHold,
        }
    }

    /// Construct an exchanger in a caller-supplied mode. Rejects
    /// [`BoundaryInterpolationMode::Linear`] at item #45 with
    /// [`BoundaryExchangerError::UnsupportedMode`]; tasks.md item #46
    /// will replace this guard with a real linear-interpolation path.
    ///
    /// # Errors
    ///
    /// Returns [`BoundaryExchangerError::UnsupportedMode`] when
    /// `mode == BoundaryInterpolationMode::Linear`. ZOH always succeeds.
    pub fn with_mode(
        boundaries: BoundarySignals,
        mode: BoundaryInterpolationMode,
    ) -> Result<Self, BoundaryExchangerError> {
        match mode {
            BoundaryInterpolationMode::ZeroOrderHold => Ok(Self { boundaries, mode }),
            BoundaryInterpolationMode::Linear => Err(BoundaryExchangerError::UnsupportedMode(mode)),
        }
    }

    /// Borrow the configured boundary signal pairs.
    #[must_use]
    pub fn boundaries(&self) -> &BoundarySignals {
        &self.boundaries
    }

    /// The interpolation mode this exchanger was constructed with.
    #[must_use]
    pub fn mode(&self) -> BoundaryInterpolationMode {
        self.mode
    }

    /// Exchange values at synchronization point T.
    ///
    /// For each `(analog_source, digital_dest)` pair in
    /// `boundaries.analog_to_digital`, read the analog provider's
    /// most-recent value for `analog_source` and emit it as the value
    /// delivered to `digital_dest`. Symmetrically for the
    /// `digital_to_analog` direction.
    ///
    /// Missing source values are *not* defaulted; they are recorded
    /// in [`BoundaryExchangePacket::missing_sources`]. This preserves
    /// the spec's invariant that *both* simulators proceed from time T
    /// with the exchanged values: a downstream caller observing a
    /// non-empty `missing_sources` is the one that decides how to
    /// recover (typical choices: stall, retry, abort with a
    /// diagnostic).
    ///
    /// **ZOH semantics:** the providers are called with no time
    /// argument because ZOH does not interpolate — the value returned
    /// is the most recent accepted sample, held constant until T. The
    /// caller is responsible for ensuring the providers have advanced
    /// past T's previous boundary before invoking `exchange`.
    pub fn exchange<A, D>(&self, analog: &A, digital: &D) -> BoundaryExchangePacket
    where
        A: AnalogValueProvider + ?Sized,
        D: DigitalValueProvider + ?Sized,
    {
        debug_assert_eq!(
            self.mode,
            BoundaryInterpolationMode::ZeroOrderHold,
            "BoundarySignalExchanger::exchange is the ZOH path only at \
             tasks.md item #45; item #46 will fork on `mode`"
        );

        let mut analog_to_digital = Vec::with_capacity(self.boundaries.analog_to_digital.len());
        let mut digital_to_analog = Vec::with_capacity(self.boundaries.digital_to_analog.len());
        let mut missing_sources = Vec::new();

        for (analog_src, digital_dest) in &self.boundaries.analog_to_digital {
            match analog.last_analog_value(analog_src) {
                Some(v) => analog_to_digital.push((digital_dest.clone(), v)),
                None => missing_sources.push(analog_src.clone()),
            }
        }

        for (digital_src, analog_dest) in &self.boundaries.digital_to_analog {
            match digital.last_digital_value(digital_src) {
                Some(v) => digital_to_analog.push((analog_dest.clone(), v)),
                None => missing_sources.push(digital_src.clone()),
            }
        }

        BoundaryExchangePacket {
            analog_to_digital,
            digital_to_analog,
            missing_sources,
        }
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    /// Trivial in-memory provider implementing both halves; tests
    /// configure it with a per-signal "latest value" map to simulate
    /// the ZOH invariant (most-recent accepted value, no time
    /// interpolation).
    struct MapProvider {
        analog: HashMap<String, f64>,
        digital: HashMap<String, f64>,
    }

    impl MapProvider {
        fn new() -> Self {
            Self {
                analog: HashMap::new(),
                digital: HashMap::new(),
            }
        }
        fn with_analog(mut self, name: &str, value: f64) -> Self {
            self.analog.insert(name.to_string(), value);
            self
        }
        fn with_digital(mut self, name: &str, value: f64) -> Self {
            self.digital.insert(name.to_string(), value);
            self
        }
    }

    impl AnalogValueProvider for MapProvider {
        fn last_analog_value(&self, signal: &SignalName) -> Option<f64> {
            self.analog.get(signal.as_str()).copied()
        }
    }

    impl DigitalValueProvider for MapProvider {
        fn last_digital_value(&self, signal: &SignalName) -> Option<f64> {
            self.digital.get(signal.as_str()).copied()
        }
    }

    fn vout_din_dout_vin_boundaries() -> BoundarySignals {
        BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![(SignalName::new("dout"), SignalName::new("vin"))],
        }
    }

    /// Default mode is ZOH per ADR-0007.
    #[test]
    fn default_mode_is_zero_order_hold() {
        assert_eq!(
            BoundaryInterpolationMode::default(),
            BoundaryInterpolationMode::ZeroOrderHold
        );
    }

    /// `zero_order_hold` constructor pins the mode to ZOH regardless
    /// of `Default`.
    #[test]
    fn zoh_constructor_pins_mode() {
        let ex = BoundarySignalExchanger::zero_order_hold(BoundarySignals::default());
        assert_eq!(ex.mode(), BoundaryInterpolationMode::ZeroOrderHold);
    }

    /// `with_mode(Linear)` is rejected at item #45 — that path is item
    /// #46.
    #[test]
    fn linear_mode_is_rejected_at_item_45() {
        let err = BoundarySignalExchanger::with_mode(
            BoundarySignals::default(),
            BoundaryInterpolationMode::Linear,
        )
        .expect_err("linear must be rejected at item #45");
        assert_eq!(
            err,
            BoundaryExchangerError::UnsupportedMode(BoundaryInterpolationMode::Linear)
        );
        // Error message is human-readable and names item #46 as the
        // venue.
        let msg = format!("{err}");
        assert!(msg.contains("linear"));
        assert!(msg.contains("#46"));
    }

    /// `with_mode(ZeroOrderHold)` succeeds and matches the
    /// `zero_order_hold` shortcut.
    #[test]
    fn with_mode_zoh_succeeds() {
        let ex = BoundarySignalExchanger::with_mode(
            BoundarySignals::default(),
            BoundaryInterpolationMode::ZeroOrderHold,
        )
        .unwrap();
        assert_eq!(ex.mode(), BoundaryInterpolationMode::ZeroOrderHold);
    }

    /// **Core ADR-0007 invariant**: the exchanger reads each side's
    /// most recent value and emits it under the configured
    /// destination name. No interpolation.
    #[test]
    fn zoh_exchange_routes_values_to_destination_names() {
        let provider = MapProvider::new()
            .with_analog("vout", 3.3)
            .with_digital("dout", 1.0);
        let ex = BoundarySignalExchanger::zero_order_hold(vout_din_dout_vin_boundaries());

        let packet = ex.exchange(&provider, &provider);

        assert!(packet.is_complete(), "all four signals are present");
        assert_eq!(
            packet.analog_to_digital_value(&SignalName::new("din")),
            Some(3.3),
            "analog vout (3.3 V) must be delivered to digital input din"
        );
        assert_eq!(
            packet.digital_to_analog_value(&SignalName::new("vin")),
            Some(1.0),
            "digital dout (1.0) must be delivered to analog input vin"
        );
    }

    /// ZOH means "hold last value constant" — calling `exchange` twice
    /// with no provider update returns the same packet both times.
    /// (Charge conservation guarantee: nothing changes between
    /// boundaries unless a side accepts a new sample.)
    #[test]
    fn zoh_holds_value_constant_across_repeated_exchanges() {
        let provider = MapProvider::new()
            .with_analog("vout", 1.65)
            .with_digital("dout", 0.0);
        let ex = BoundarySignalExchanger::zero_order_hold(vout_din_dout_vin_boundaries());

        let p1 = ex.exchange(&provider, &provider);
        let p2 = ex.exchange(&provider, &provider);
        assert_eq!(
            p1, p2,
            "ZOH must return identical packets when no side has updated"
        );
        assert_eq!(
            p1.analog_to_digital_value(&SignalName::new("din")),
            Some(1.65)
        );
        assert_eq!(
            p1.digital_to_analog_value(&SignalName::new("vin")),
            Some(0.0)
        );
    }

    /// When a side updates its most-recent value, the next exchange
    /// reflects the new value — but only because the *provider*
    /// changed; the exchanger itself does not interpolate.
    #[test]
    fn zoh_reflects_provider_updates_at_next_exchange() {
        let p1 = MapProvider::new()
            .with_analog("vout", 0.0)
            .with_digital("dout", 0.0);
        let p2 = MapProvider::new()
            .with_analog("vout", 3.3)
            .with_digital("dout", 1.0);
        let ex = BoundarySignalExchanger::zero_order_hold(vout_din_dout_vin_boundaries());

        let pkt_t0 = ex.exchange(&p1, &p1);
        let pkt_t1 = ex.exchange(&p2, &p2);

        assert_eq!(
            pkt_t0.analog_to_digital_value(&SignalName::new("din")),
            Some(0.0)
        );
        assert_eq!(
            pkt_t1.analog_to_digital_value(&SignalName::new("din")),
            Some(3.3)
        );
    }

    /// Missing source values are surfaced, not silently defaulted.
    /// The exchanger preserves the boundary's "*both* simulators
    /// proceed with the exchanged values" invariant by letting the
    /// caller decide how to recover.
    #[test]
    fn missing_sources_are_surfaced_not_defaulted() {
        // Analog has vout but digital has no dout yet.
        let provider = MapProvider::new().with_analog("vout", 3.3);
        let ex = BoundarySignalExchanger::zero_order_hold(vout_din_dout_vin_boundaries());

        let packet = ex.exchange(&provider, &provider);

        assert!(!packet.is_complete());
        assert_eq!(packet.missing_sources, vec![SignalName::new("dout")]);
        // Available side still delivered:
        assert_eq!(
            packet.analog_to_digital_value(&SignalName::new("din")),
            Some(3.3)
        );
        // Unavailable side has no entry:
        assert_eq!(
            packet.digital_to_analog_value(&SignalName::new("vin")),
            None
        );
    }

    /// Empty boundary configuration produces a complete (empty) packet
    /// — useful when the scenario does not exchange anything but the
    /// scheduler still calls exchange unconditionally.
    #[test]
    fn empty_boundaries_yield_empty_complete_packet() {
        let provider = MapProvider::new();
        let ex = BoundarySignalExchanger::zero_order_hold(BoundarySignals::default());
        let packet = ex.exchange(&provider, &provider);
        assert!(packet.is_complete());
        assert!(packet.analog_to_digital.is_empty());
        assert!(packet.digital_to_analog.is_empty());
    }

    /// Mode display strings are stable so log lines emitted by
    /// adapters do not drift between releases.
    #[test]
    fn mode_display_strings_are_stable() {
        assert_eq!(
            format!("{}", BoundaryInterpolationMode::ZeroOrderHold),
            "zero_order_hold"
        );
        assert_eq!(format!("{}", BoundaryInterpolationMode::Linear), "linear");
    }

    /// The exchanger borrows its configured pairs back to the caller
    /// (useful in scheduler diagnostics).
    #[test]
    fn boundaries_accessor_returns_configured_pairs() {
        let b = vout_din_dout_vin_boundaries();
        let ex = BoundarySignalExchanger::zero_order_hold(b.clone());
        assert_eq!(ex.boundaries(), &b);
    }

    /// Multi-pair configurations route each source to its configured
    /// destination independently.
    #[test]
    fn multiple_pairs_route_independently() {
        let boundaries = BoundarySignals {
            analog_to_digital: vec![
                (SignalName::new("vout_a"), SignalName::new("din_a")),
                (SignalName::new("vout_b"), SignalName::new("din_b")),
            ],
            digital_to_analog: vec![
                (SignalName::new("dout_x"), SignalName::new("vin_x")),
                (SignalName::new("dout_y"), SignalName::new("vin_y")),
            ],
        };
        let provider = MapProvider::new()
            .with_analog("vout_a", 1.0)
            .with_analog("vout_b", 2.0)
            .with_digital("dout_x", 3.0)
            .with_digital("dout_y", 4.0);
        let ex = BoundarySignalExchanger::zero_order_hold(boundaries);
        let pkt = ex.exchange(&provider, &provider);
        assert!(pkt.is_complete());
        assert_eq!(
            pkt.analog_to_digital_value(&SignalName::new("din_a")),
            Some(1.0)
        );
        assert_eq!(
            pkt.analog_to_digital_value(&SignalName::new("din_b")),
            Some(2.0)
        );
        assert_eq!(
            pkt.digital_to_analog_value(&SignalName::new("vin_x")),
            Some(3.0)
        );
        assert_eq!(
            pkt.digital_to_analog_value(&SignalName::new("vin_y")),
            Some(4.0)
        );
    }
}
