//! Analog-digital boundary signal exchanger.
//!
//! Per **ADR-0007** ("Zero-Order Hold Default at Analog-Digital
//! Boundary"), the analog and digital simulator kernels exchange named
//! boundary signal values at every synchronization point T. Because
//! the analog solver's adaptive timesteps rarely land exactly on a
//! digital event time, an interpolation scheme is required.
//!
//! This module implements both the **zero-order hold (ZOH)** default
//! mandated by ADR-0007 (tasks.md item #45) and the **linear
//! interpolation opt-in** (tasks.md item #46):
//!
//! - **ZOH** ([`BoundarySignalExchanger::zero_order_hold`] +
//!   [`BoundarySignalExchanger::exchange`]): at the synchronization
//!   point T, each exchanged value is the *most recent accepted value
//!   at or before T* from the providing side. No interpolation is
//!   performed; the value is held constant from its last sample until
//!   T. This is charge-conserving by construction and requires no
//!   historical solution vectors.
//! - **Linear** ([`BoundarySignalExchanger::linear`] +
//!   [`BoundarySignalExchanger::exchange_linear`]): the caller passes
//!   the synchronization-point time T plus
//!   [`AnalogSampleHistoryProvider`] / [`DigitalSampleHistoryProvider`]
//!   handles, which return the two most recent accepted `(time, value)`
//!   samples for each named boundary signal. The exchanger interpolates
//!   linearly at T:
//!
//!   ```text
//!   v(T) = v0 + (v1 - v0) * (T - t0) / (t1 - t0)
//!   ```
//!
//!   where `(t0, v0)` is the older sample and `(t1, v1)` is the newer.
//!   When only one sample is available the linear path degrades to
//!   ZOH-equivalent (the lone sample value). When two samples share a
//!   time (`t0 == t1`) the newer value `v1` is returned to avoid
//!   division by zero. When no sample is available the signal is
//!   surfaced via [`BoundaryExchangePacket::missing_sources`], matching
//!   the ZOH path.
//!
//! Per ADR-0007 the linear opt-in accepts a charge-conservation
//! tradeoff in exchange for smoother boundary waveforms; users must
//! opt in explicitly via [`BoundaryInterpolationMode::Linear`].
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
//! ZOH and Linear paths both satisfy the scenario; they differ only
//! in *how* the value at T is computed. Tasks.md #45 covers ZOH;
//! tasks.md #46 covers Linear.
//!
//! # Composition with the Mixed-Signal Scheduler
//!
//! The exchanger is intentionally a standalone component. The
//! [`MixedSignalScheduler`](crate::MixedSignalScheduler) at task #42
//! holds a [`BoundarySignals`] configuration but does not yet wire it
//! through an exchanger in its `run()` loop (sibling tasks #47, #48
//! will). The exchanger is therefore exposed as a directly callable
//! component so:
//!
//! - the sibling Icarus Verilog (#47) and Verilator (#48) adapter
//!   integrations can drive it at each synchronization point, and
//! - the linear-interpolation extension (#46) sits next to the ZOH
//!   path under one type, switched by [`BoundaryInterpolationMode`].

use circuit_solver_types::SignalName;
use core::fmt;

use crate::BoundarySignals;

// ---------------------------------------------------------------------------
// Interpolation mode
// ---------------------------------------------------------------------------

/// Per-request boundary-interpolation policy named by ADR-0007.
///
/// The default ([`Self::ZeroOrderHold`]) is the focus of tasks.md item
/// #45. The [`Self::Linear`] variant is the focus of tasks.md item
/// #46 — it interpolates linearly between the two most recent accepted
/// samples at the synchronization point T.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum BoundaryInterpolationMode {
    /// Hold the last accepted analog (or digital) value constant from
    /// its sample time until the synchronization point T. The default
    /// per ADR-0007. Charge-conserving; requires no historical state.
    #[default]
    ZeroOrderHold,
    /// Linearly interpolate between the two most recent accepted
    /// samples at the synchronization point T. Per ADR-0007 (Option
    /// C, opt-in) the caller accepts a charge-conservation tradeoff
    /// in exchange for smoother boundary waveforms. Implemented in
    /// tasks.md item #46.
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
// Provider traits — ZOH (most-recent value only)
// ---------------------------------------------------------------------------

/// Returns the last accepted analog value for a named boundary signal.
///
/// The ZOH exchanger calls this at every synchronization point to
/// obtain the analog value that should be delivered to the digital
/// simulator per the ZOH discipline ("the analog value at the last
/// accepted timestep is held constant until the digital event time",
/// ADR-0007 Y-statement).
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
// Provider traits — Linear (two most recent (time, value) samples)
// ---------------------------------------------------------------------------

/// A single accepted boundary sample: an integer-nanosecond time stamp
/// paired with the accepted value.
///
/// Per ADR-0007 the *linear* opt-in path requires the solver to retain
/// the two most recent solution vectors; the sample history providers
/// expose only the projection of those vectors onto a single named
/// boundary signal, which is all the exchanger needs.
///
/// `time_ns` is `i64` to match the time stamps already used by the
/// digital simulator adapters (see [`crate::mixed_signal`]).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BoundarySample {
    /// Time at which this sample was accepted, in integer
    /// nanoseconds. Matches the digital simulator's event time
    /// resolution.
    pub time_ns: i64,
    /// The accepted value at that time.
    pub value: f64,
}

impl BoundarySample {
    /// Construct a sample.
    #[must_use]
    pub fn new(time_ns: i64, value: f64) -> Self {
        Self { time_ns, value }
    }
}

/// Returns the two most recent accepted analog samples for a named
/// boundary signal, for the *linear* opt-in path of ADR-0007.
///
/// Implementations are expected to maintain a sliding window of two
/// solution vectors near accepted boundary times; per ADR-0007:
/// *"When the user selects linear interpolation, the numeric solver
/// retains the two most recent solution vectors and interpolates at
/// the event time."*
///
/// Return contract:
///
/// - `(Some(older), Some(newer))` with `older.time_ns <= newer.time_ns`
///   — both samples available; linear interpolation occurs.
/// - `(None, Some(newer))` or `(Some(only), None)` — only one sample
///   available yet; the exchanger degrades to ZOH-equivalent and
///   returns that one value.
/// - `(None, None)` — no sample available; the exchanger surfaces the
///   source signal in [`BoundaryExchangePacket::missing_sources`].
pub trait AnalogSampleHistoryProvider {
    /// Two most recent accepted analog samples for `signal`, ordered
    /// `(older, newer)`. See the trait docs for the encoding of
    /// "fewer than two" cases.
    fn analog_sample_history(
        &self,
        signal: &SignalName,
    ) -> (Option<BoundarySample>, Option<BoundarySample>);
}

/// Returns the two most recent accepted digital samples for a named
/// boundary signal.
///
/// Digital signals are event-driven and quantized; "linear
/// interpolation" on a digital signal is unusual but consistent with
/// the ADR-0007 contract that *every* boundary signal can opt in to
/// the linear scheme. Adapters that want to keep digital outputs
/// piecewise-constant should return `(None, Some(latest))` so the
/// exchanger degrades to ZOH-equivalent for that signal.
pub trait DigitalSampleHistoryProvider {
    /// Two most recent accepted digital samples for `signal`, ordered
    /// `(older, newer)`. See [`AnalogSampleHistoryProvider`] for the
    /// encoding of "fewer than two" cases.
    fn digital_sample_history(
        &self,
        signal: &SignalName,
    ) -> (Option<BoundarySample>, Option<BoundarySample>);
}

// ---------------------------------------------------------------------------
// Linear interpolation kernel
// ---------------------------------------------------------------------------

/// Linearly interpolate (or extrapolate) at time `t` given the line
/// through `(t0, v0)` and `(t1, v1)`.
///
/// When `t0 == t1` (degenerate; division would be by zero) returns
/// `v1` (the newer of the two by the [`AnalogSampleHistoryProvider`]
/// contract). When `t == t1` returns exactly `v1`; when `t == t0`
/// returns exactly `v0` (no rounding drift).
///
/// Extrapolation (`t < t0` or `t > t1`) is permitted: per ADR-0007 the
/// solver advances *past* the digital event time using its own
/// adaptive step, then asks "what was the value at T". In that
/// configuration `t0 <= T <= t1`, but mathematically the formula is
/// the same and we do not clamp.
#[must_use]
pub fn linear_interpolate(t0: i64, v0: f64, t1: i64, v1: f64, t: i64) -> f64 {
    if t0 == t1 {
        return v1;
    }
    if t == t0 {
        return v0;
    }
    if t == t1 {
        return v1;
    }
    // f64 has 53 bits of mantissa; for `i64` time stamps in plausible
    // simulation ranges (nanoseconds over hours) the difference fits
    // without loss. Cast via i128 to be explicit about not panicking
    // on `t1 - t0` overflow at extreme stamps, then to f64 for the
    // ratio. `cast_precision_loss` is acknowledged: nanoseconds over
    // a 30-year span fit in 53 bits.
    #[allow(clippy::cast_precision_loss)]
    let dt_total = i128::from(t1 - t0) as f64;
    #[allow(clippy::cast_precision_loss)]
    let dt = i128::from(t - t0) as f64;
    v0 + (v1 - v0) * (dt / dt_total)
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
///   solver's `vout` at the ZOH-resolved or linear-interpolated time.
/// - `digital_to_analog[i] == (analog_input_name, digital_value)`
///   where `analog_input_name` is the *destination* name (the analog
///   solver's input). The value carries the digital simulator's
///   `dout` at the resolved time.
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
    /// The exchanger was asked to use its ZOH entry point
    /// ([`BoundarySignalExchanger::exchange`]) while constructed in
    /// [`BoundaryInterpolationMode::Linear`], or its linear entry
    /// point ([`BoundarySignalExchanger::exchange_linear`]) while
    /// constructed in [`BoundaryInterpolationMode::ZeroOrderHold`].
    /// The two paths require different provider traits and cannot be
    /// silently coerced.
    WrongModeForEntryPoint {
        /// The mode the exchanger was constructed in.
        configured: BoundaryInterpolationMode,
        /// The mode the entry point requires.
        required: BoundaryInterpolationMode,
    },
}

impl fmt::Display for BoundaryExchangerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongModeForEntryPoint {
                configured,
                required,
            } => write!(
                f,
                "boundary exchanger is configured in mode {configured} but the \
                 invoked entry point requires mode {required}; construct the \
                 exchanger with the matching mode or call the matching method"
            ),
        }
    }
}

impl std::error::Error for BoundaryExchangerError {}

// ---------------------------------------------------------------------------
// BoundarySignalExchanger
// ---------------------------------------------------------------------------

/// Exchanges named boundary signal values between the analog solver
/// and the digital simulator at every synchronization point, using
/// either the **zero-order hold** discipline (default, ADR-0007
/// tasks.md #45) or the **linear interpolation** opt-in (ADR-0007
/// tasks.md #46).
///
/// # Lifecycle
///
/// 1. Construct via [`BoundarySignalExchanger::zero_order_hold`] or
///    [`BoundarySignalExchanger::linear`] with the configured
///    [`BoundarySignals`] pairs (or via
///    [`BoundarySignalExchanger::with_mode`] when the mode is
///    data-driven).
/// 2. At each synchronization point T, call the entry point that
///    matches the configured mode:
///    - ZOH → [`BoundarySignalExchanger::exchange`] with handles to
///      [`AnalogValueProvider`] and [`DigitalValueProvider`].
///    - Linear → [`BoundarySignalExchanger::exchange_linear`] with
///      `time_ns: i64` plus
///      [`AnalogSampleHistoryProvider`] / [`DigitalSampleHistoryProvider`]
///      handles. The two-sample window per signal lives on the
///      providers (see ADR-0007: *"the numeric solver retains the two
///      most recent solution vectors"*).
///
///    The returned [`BoundaryExchangePacket`] is delivered to the
///    destination kernels by the caller (typically the scheduler or
///    an adapter).
///
/// # Why ZOH and not interpolation by default
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
/// The opt-in linear path accepts a charge-conservation tradeoff in
/// exchange for smoother boundary waveforms; users who need that path
/// must opt in explicitly.
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

    /// Construct a linear-interpolation exchanger over the given
    /// boundary signal configuration. Per ADR-0007 (Option C, opt-in)
    /// the caller accepts a charge-conservation tradeoff in exchange
    /// for smoother boundary waveforms.
    #[must_use]
    pub fn linear(boundaries: BoundarySignals) -> Self {
        Self {
            boundaries,
            mode: BoundaryInterpolationMode::Linear,
        }
    }

    /// Construct an exchanger in a caller-supplied mode. Both
    /// [`BoundaryInterpolationMode::ZeroOrderHold`] and
    /// [`BoundaryInterpolationMode::Linear`] are supported; the
    /// `Result` return is retained for forward compatibility with
    /// future ADR-0007 follow-up modes.
    ///
    /// # Errors
    ///
    /// Currently infallible — both v1 modes are supported. The
    /// `Result` return is preserved so a future mode addition that
    /// must validate the boundary configuration at construction time
    /// can do so without breaking call sites.
    #[allow(clippy::unnecessary_wraps)]
    pub fn with_mode(
        boundaries: BoundarySignals,
        mode: BoundaryInterpolationMode,
    ) -> Result<Self, BoundaryExchangerError> {
        Ok(Self { boundaries, mode })
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

    /// Exchange values at synchronization point T using **zero-order
    /// hold** semantics.
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
    ///
    /// # Panics
    ///
    /// Debug builds assert that the exchanger was constructed in
    /// [`BoundaryInterpolationMode::ZeroOrderHold`]; calling this
    /// method on a linear-mode exchanger is a programmer error (use
    /// [`Self::exchange_linear`] instead). Release builds tolerate the
    /// mismatch and emit ZOH semantics regardless, since the two paths
    /// agree when only one sample is available.
    pub fn exchange<A, D>(&self, analog: &A, digital: &D) -> BoundaryExchangePacket
    where
        A: AnalogValueProvider + ?Sized,
        D: DigitalValueProvider + ?Sized,
    {
        debug_assert_eq!(
            self.mode,
            BoundaryInterpolationMode::ZeroOrderHold,
            "BoundarySignalExchanger::exchange is the ZOH path; call \
             exchange_linear when the exchanger is in Linear mode"
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

    /// Exchange values at synchronization point `time_ns` using
    /// **linear interpolation** semantics (ADR-0007 opt-in, tasks.md
    /// item #46).
    ///
    /// For each configured boundary pair, the corresponding sample
    /// history provider returns the two most recent accepted samples
    /// `(older, newer)` (see [`AnalogSampleHistoryProvider`] for the
    /// encoding of fewer-than-two cases). The exchanger then:
    ///
    /// - if **two samples** are available, interpolates linearly at
    ///   `time_ns` (see [`linear_interpolate`]);
    /// - if **one sample** is available, degrades to ZOH-equivalent
    ///   and emits that lone value;
    /// - if **no sample** is available, surfaces the source signal in
    ///   [`BoundaryExchangePacket::missing_sources`].
    ///
    /// As with [`Self::exchange`], missing source values are *not*
    /// defaulted; the caller decides how to recover.
    ///
    /// # Errors
    ///
    /// Returns [`BoundaryExchangerError::WrongModeForEntryPoint`] when
    /// the exchanger was constructed in
    /// [`BoundaryInterpolationMode::ZeroOrderHold`]. The two entry
    /// points use different provider traits and cannot be silently
    /// coerced; a ZOH-mode caller should use [`Self::exchange`]
    /// instead.
    pub fn exchange_linear<A, D>(
        &self,
        time_ns: i64,
        analog: &A,
        digital: &D,
    ) -> Result<BoundaryExchangePacket, BoundaryExchangerError>
    where
        A: AnalogSampleHistoryProvider + ?Sized,
        D: DigitalSampleHistoryProvider + ?Sized,
    {
        if self.mode != BoundaryInterpolationMode::Linear {
            return Err(BoundaryExchangerError::WrongModeForEntryPoint {
                configured: self.mode,
                required: BoundaryInterpolationMode::Linear,
            });
        }

        let mut analog_to_digital = Vec::with_capacity(self.boundaries.analog_to_digital.len());
        let mut digital_to_analog = Vec::with_capacity(self.boundaries.digital_to_analog.len());
        let mut missing_sources = Vec::new();

        for (analog_src, digital_dest) in &self.boundaries.analog_to_digital {
            match analog.analog_sample_history(analog_src) {
                (Some(older), Some(newer)) => {
                    let v = linear_interpolate(
                        older.time_ns,
                        older.value,
                        newer.time_ns,
                        newer.value,
                        time_ns,
                    );
                    analog_to_digital.push((digital_dest.clone(), v));
                }
                (None, Some(only)) | (Some(only), None) => {
                    // Degrade to ZOH-equivalent: a single sample
                    // cannot define a line.
                    analog_to_digital.push((digital_dest.clone(), only.value));
                }
                (None, None) => missing_sources.push(analog_src.clone()),
            }
        }

        for (digital_src, analog_dest) in &self.boundaries.digital_to_analog {
            match digital.digital_sample_history(digital_src) {
                (Some(older), Some(newer)) => {
                    let v = linear_interpolate(
                        older.time_ns,
                        older.value,
                        newer.time_ns,
                        newer.value,
                        time_ns,
                    );
                    digital_to_analog.push((analog_dest.clone(), v));
                }
                (None, Some(only)) | (Some(only), None) => {
                    digital_to_analog.push((analog_dest.clone(), only.value));
                }
                (None, None) => missing_sources.push(digital_src.clone()),
            }
        }

        Ok(BoundaryExchangePacket {
            analog_to_digital,
            digital_to_analog,
            missing_sources,
        })
    }
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // --- ZOH test doubles (carried forward from tasks.md item #45) ---

    /// Trivial in-memory provider implementing both ZOH halves; tests
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

    // --- Linear test doubles (tasks.md item #46) ---

    /// Two-sample history table: `signal` →
    /// `(Option<older>, Option<newer>)`. Mirrors the
    /// [`AnalogSampleHistoryProvider`] / [`DigitalSampleHistoryProvider`]
    /// contract directly.
    struct HistoryProvider {
        analog: HashMap<String, (Option<BoundarySample>, Option<BoundarySample>)>,
        digital: HashMap<String, (Option<BoundarySample>, Option<BoundarySample>)>,
    }

    impl HistoryProvider {
        fn new() -> Self {
            Self {
                analog: HashMap::new(),
                digital: HashMap::new(),
            }
        }
        fn with_analog_history(mut self, name: &str, older: (i64, f64), newer: (i64, f64)) -> Self {
            self.analog.insert(
                name.to_string(),
                (
                    Some(BoundarySample::new(older.0, older.1)),
                    Some(BoundarySample::new(newer.0, newer.1)),
                ),
            );
            self
        }
        fn with_digital_history(
            mut self,
            name: &str,
            older: (i64, f64),
            newer: (i64, f64),
        ) -> Self {
            self.digital.insert(
                name.to_string(),
                (
                    Some(BoundarySample::new(older.0, older.1)),
                    Some(BoundarySample::new(newer.0, newer.1)),
                ),
            );
            self
        }
        fn with_only_newer_analog(mut self, name: &str, newer: (i64, f64)) -> Self {
            self.analog.insert(
                name.to_string(),
                (None, Some(BoundarySample::new(newer.0, newer.1))),
            );
            self
        }
        fn with_only_newer_digital(mut self, name: &str, newer: (i64, f64)) -> Self {
            self.digital.insert(
                name.to_string(),
                (None, Some(BoundarySample::new(newer.0, newer.1))),
            );
            self
        }
    }

    impl AnalogSampleHistoryProvider for HistoryProvider {
        fn analog_sample_history(
            &self,
            signal: &SignalName,
        ) -> (Option<BoundarySample>, Option<BoundarySample>) {
            self.analog
                .get(signal.as_str())
                .copied()
                .unwrap_or((None, None))
        }
    }

    impl DigitalSampleHistoryProvider for HistoryProvider {
        fn digital_sample_history(
            &self,
            signal: &SignalName,
        ) -> (Option<BoundarySample>, Option<BoundarySample>) {
            self.digital
                .get(signal.as_str())
                .copied()
                .unwrap_or((None, None))
        }
    }

    // --- linear_interpolate unit tests ---

    /// Midpoint of `(0,0)→(10,10)` at `t=5` is `5.0`. The textbook
    /// case.
    #[test]
    fn linear_interp_midpoint() {
        assert!((linear_interpolate(0, 0.0, 10, 10.0, 5) - 5.0).abs() < 1e-12);
    }

    /// At an endpoint, exact endpoint value is returned without
    /// floating-point drift (we short-circuit `t == t0` and `t == t1`).
    #[test]
    #[allow(clippy::float_cmp)] // testing exactness of the short-circuit
    fn linear_interp_endpoints_exact() {
        assert_eq!(linear_interpolate(100, 1.5, 200, 3.5, 100), 1.5);
        assert_eq!(linear_interpolate(100, 1.5, 200, 3.5, 200), 3.5);
    }

    /// `t0 == t1` degenerate case returns `v1` (the newer sample by
    /// the provider contract). No NaN, no panic.
    #[test]
    #[allow(clippy::float_cmp)] // testing exactness of the degenerate-case short-circuit
    fn linear_interp_degenerate_equal_times_returns_newer() {
        assert_eq!(linear_interpolate(42, 1.0, 42, 9.9, 42), 9.9);
        // And for any query time:
        assert_eq!(linear_interpolate(42, 1.0, 42, 9.9, 0), 9.9);
        assert_eq!(linear_interpolate(42, 1.0, 42, 9.9, 100), 9.9);
    }

    /// Extrapolation past `t1` follows the same line — the docs
    /// promise this and adapters may rely on it when the analog
    /// solver advances past T.
    #[test]
    fn linear_interp_extrapolates_past_t1() {
        // line through (0, 0) → (10, 1) → at t=20 should be 2.0
        assert!((linear_interpolate(0, 0.0, 10, 1.0, 20) - 2.0).abs() < 1e-12);
    }

    /// Extrapolation before `t0` likewise.
    #[test]
    fn linear_interp_extrapolates_before_t0() {
        // line through (10, 1) → (20, 2) → at t=0 should be 0.0
        assert!((linear_interpolate(10, 1.0, 20, 2.0, 0) - 0.0).abs() < 1e-12);
    }

    // --- ZOH constructor / mode tests (preserved) ---

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

    /// `linear` constructor pins the mode to Linear (tasks.md #46).
    #[test]
    fn linear_constructor_pins_mode() {
        let ex = BoundarySignalExchanger::linear(BoundarySignals::default());
        assert_eq!(ex.mode(), BoundaryInterpolationMode::Linear);
    }

    /// `with_mode(Linear)` now succeeds (tasks.md #46 lifts the #45
    /// rejection). Both v1 modes round-trip through `with_mode`.
    #[test]
    fn with_mode_supports_both_v1_modes() {
        let zoh = BoundarySignalExchanger::with_mode(
            BoundarySignals::default(),
            BoundaryInterpolationMode::ZeroOrderHold,
        )
        .expect("ZOH must succeed");
        assert_eq!(zoh.mode(), BoundaryInterpolationMode::ZeroOrderHold);

        let lin = BoundarySignalExchanger::with_mode(
            BoundarySignals::default(),
            BoundaryInterpolationMode::Linear,
        )
        .expect("Linear must succeed at tasks.md #46");
        assert_eq!(lin.mode(), BoundaryInterpolationMode::Linear);
    }

    /// **Core ADR-0007 invariant (ZOH path)**: the exchanger reads
    /// each side's most recent value and emits it under the
    /// configured destination name. No interpolation.
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
    /// destination independently (ZOH path).
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

    // --- Linear path tests (tasks.md item #46) ---

    /// **Core ADR-0007 linear invariant**: two samples bracket the
    /// synchronization point T; the exchanger emits the linearly
    /// interpolated value at T.
    ///
    /// Set-up:
    ///   analog vout has samples (t=0, 0.0 V) and (t=10 ns, 10.0 V).
    ///   The scheduler reaches T=5 ns.
    /// Expected: din receives 5.0 V (midpoint).
    #[test]
    fn linear_exchange_interpolates_at_t_between_two_samples() {
        let history = HistoryProvider::new()
            .with_analog_history("vout", (0, 0.0), (10, 10.0))
            .with_digital_history("dout", (0, 0.0), (10, 1.0));
        let ex = BoundarySignalExchanger::linear(vout_din_dout_vin_boundaries());

        let packet = ex.exchange_linear(5, &history, &history).unwrap();
        assert!(packet.is_complete());
        let din = packet
            .analog_to_digital_value(&SignalName::new("din"))
            .unwrap();
        assert!(
            (din - 5.0).abs() < 1e-12,
            "linear midpoint should be 5.0, got {din}"
        );
        let vin = packet
            .digital_to_analog_value(&SignalName::new("vin"))
            .unwrap();
        assert!(
            (vin - 0.5).abs() < 1e-12,
            "linear midpoint should be 0.5, got {vin}"
        );
    }

    /// Linear at the exact newer-sample time returns the newer value
    /// (no floating-point drift).
    #[test]
    fn linear_exchange_at_t1_returns_v1_exactly() {
        let history = HistoryProvider::new().with_analog_history("vout", (0, 0.0), (10, 10.0));
        let ex = BoundarySignalExchanger::linear(BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![],
        });
        let pkt = ex.exchange_linear(10, &history, &history).unwrap();
        assert_eq!(
            pkt.analog_to_digital_value(&SignalName::new("din")),
            Some(10.0)
        );
    }

    /// Linear with only one accepted sample available degrades to
    /// ZOH-equivalent (the lone sample's value). This matches the
    /// behaviour at t=0 before the analog solver has produced two
    /// solution vectors.
    #[test]
    fn linear_with_single_sample_degrades_to_zoh() {
        let history = HistoryProvider::new()
            .with_only_newer_analog("vout", (7, 2.5))
            .with_only_newer_digital("dout", (7, 0.0));
        let ex = BoundarySignalExchanger::linear(vout_din_dout_vin_boundaries());

        let pkt = ex.exchange_linear(42, &history, &history).unwrap();
        assert!(pkt.is_complete());
        assert_eq!(
            pkt.analog_to_digital_value(&SignalName::new("din")),
            Some(2.5),
            "single-sample fallback must return that sample's value verbatim"
        );
        assert_eq!(
            pkt.digital_to_analog_value(&SignalName::new("vin")),
            Some(0.0)
        );
    }

    /// Linear with no accepted sample available surfaces the source
    /// name in `missing_sources`. Matches the ZOH "do not invent a
    /// default" invariant.
    #[test]
    fn linear_with_no_samples_surfaces_missing_source() {
        let history = HistoryProvider::new(); // empty
        let ex = BoundarySignalExchanger::linear(vout_din_dout_vin_boundaries());
        let pkt = ex.exchange_linear(0, &history, &history).unwrap();
        assert!(!pkt.is_complete());
        assert_eq!(
            pkt.missing_sources,
            vec![SignalName::new("vout"), SignalName::new("dout")]
        );
        assert!(pkt.analog_to_digital.is_empty());
        assert!(pkt.digital_to_analog.is_empty());
    }

    /// Linear extrapolates past the newer sample (T > t1). The
    /// scheduler may advance the analog solver past the event time
    /// and then ask "what was the value at T"; the line is the same.
    #[test]
    fn linear_extrapolates_when_t_is_past_t1() {
        // Line through (0,0) → (10,1); at T=20 expect 2.0.
        let history = HistoryProvider::new().with_analog_history("vout", (0, 0.0), (10, 1.0));
        let ex = BoundarySignalExchanger::linear(BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![],
        });
        let pkt = ex.exchange_linear(20, &history, &history).unwrap();
        let v = pkt
            .analog_to_digital_value(&SignalName::new("din"))
            .unwrap();
        assert!((v - 2.0).abs() < 1e-12, "expected 2.0 by extrapolation");
    }

    /// Linear mode rejects the ZOH entry point with a structured
    /// error — the two entry points use different provider traits
    /// and silently coercing them would defeat the opt-in.
    ///
    /// (We cannot trigger the ZOH-on-linear-exchanger `debug_assert`
    /// path with a normal call because `exchange` takes ZOH provider
    /// traits; the symmetric `exchange_linear` mismatch is the one
    /// we can validate at runtime.)
    #[test]
    fn linear_exchange_rejects_zoh_mode_exchanger() {
        let history = HistoryProvider::new();
        let ex = BoundarySignalExchanger::zero_order_hold(BoundarySignals::default());
        let err = ex
            .exchange_linear(0, &history, &history)
            .expect_err("ZOH exchanger must refuse the linear entry point");
        match err {
            BoundaryExchangerError::WrongModeForEntryPoint {
                configured,
                required,
            } => {
                assert_eq!(configured, BoundaryInterpolationMode::ZeroOrderHold);
                assert_eq!(required, BoundaryInterpolationMode::Linear);
            }
        }
        // Error message names both modes.
        let err = BoundaryExchangerError::WrongModeForEntryPoint {
            configured: BoundaryInterpolationMode::ZeroOrderHold,
            required: BoundaryInterpolationMode::Linear,
        };
        let msg = format!("{err}");
        assert!(msg.contains("zero_order_hold"));
        assert!(msg.contains("linear"));
    }

    /// **Charge-conservation tradeoff witness**: with two samples
    /// that straddle a fast edge, ZOH and Linear produce *different*
    /// values at the synchronization point T. The test does not
    /// assert which is "correct" — both are; the point is that the
    /// opt-in matters.
    #[test]
    fn linear_differs_from_zoh_on_fast_edge() {
        // Fast edge: 0 V at t=0, 3.3 V at t=10 ns. At T=5 ns:
        //   ZOH  : last accepted ≤ T is (0, 0.0) → 0.0 V
        //   Lin  : 0 + (3.3 - 0)*5/10 → 1.65 V
        let zoh_provider = MapProvider::new().with_analog("vout", 0.0); // pre-edge
        let zoh = BoundarySignalExchanger::zero_order_hold(BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![],
        });
        let zoh_pkt = zoh.exchange(&zoh_provider, &MapProvider::new());

        let lin_history = HistoryProvider::new().with_analog_history("vout", (0, 0.0), (10, 3.3));
        let lin = BoundarySignalExchanger::linear(BoundarySignals {
            analog_to_digital: vec![(SignalName::new("vout"), SignalName::new("din"))],
            digital_to_analog: vec![],
        });
        let lin_pkt = lin.exchange_linear(5, &lin_history, &lin_history).unwrap();

        let zoh_v = zoh_pkt
            .analog_to_digital_value(&SignalName::new("din"))
            .unwrap();
        let lin_v = lin_pkt
            .analog_to_digital_value(&SignalName::new("din"))
            .unwrap();
        assert!(
            zoh_v.abs() < 1e-12,
            "ZOH must latch pre-edge 0.0 V, got {zoh_v}"
        );
        assert!((lin_v - 1.65).abs() < 1e-12);
        assert!(
            (zoh_v - lin_v).abs() > 1e-3,
            "ZOH and Linear must disagree on fast edges (got {zoh_v} vs {lin_v})"
        );
    }

    /// Multi-pair configurations route each source to its configured
    /// destination independently, on the linear path too.
    #[test]
    fn linear_multiple_pairs_route_independently() {
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
        // Each pair's line is parametrised so the midpoint is unique.
        let history = HistoryProvider::new()
            .with_analog_history("vout_a", (0, 0.0), (10, 2.0))
            .with_analog_history("vout_b", (0, 1.0), (10, 5.0))
            .with_digital_history("dout_x", (0, 0.0), (10, 1.0))
            .with_digital_history("dout_y", (0, 2.0), (10, 4.0));
        let ex = BoundarySignalExchanger::linear(boundaries);
        let pkt = ex.exchange_linear(5, &history, &history).unwrap();
        assert!(pkt.is_complete());
        assert!(
            (pkt.analog_to_digital_value(&SignalName::new("din_a"))
                .unwrap()
                - 1.0)
                .abs()
                < 1e-12
        );
        assert!(
            (pkt.analog_to_digital_value(&SignalName::new("din_b"))
                .unwrap()
                - 3.0)
                .abs()
                < 1e-12
        );
        assert!(
            (pkt.digital_to_analog_value(&SignalName::new("vin_x"))
                .unwrap()
                - 0.5)
                .abs()
                < 1e-12
        );
        assert!(
            (pkt.digital_to_analog_value(&SignalName::new("vin_y"))
                .unwrap()
                - 3.0)
                .abs()
                < 1e-12
        );
    }
}
