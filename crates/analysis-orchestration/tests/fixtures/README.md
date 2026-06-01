# Conformance test fixtures

This directory holds golden-reference rawfiles consumed by the
per-analysis conformance tests in this crate (tasks.md items #63–#67).
The on-disk format is the ngspice ASCII rawfile, parsed by
[`conformance_harness::load_ngspice_ascii`].

## sky130_rc_discharge_transient.raw

Golden reference for the
[`transient-time-domain#transient-conformance-against-ngspice`][spec]
scenario (tasks.md item #65), exercised by the test
`scenario_transient_conformance_against_ngspice.rs`.

[spec]: ../../../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/transient-time-domain/spec.md

### Test bench

A single-pole RC discharge representative of a Sky130 metal-layer
load:

- `R = 10 kΩ`, `C = 1 pF`, time constant `τ = R · C = 10 ns`.
- Initial state: `V(n_cap) = 1 V` seeded via UIC at `t = 0`.
- Stop: `t = 30 ns` (3 τ; analytic residual ≈ 49.8 mV — comfortably
  outside the 1 mV ADR-0008 absolute floor at every reported point).

The R and C values lie in the range a Sky130 PDK extracted parasitic
network produces on a metal-2 segment driving a small-cell load
capacitance; the test bench is *the topology that scenario applies
to*, intentionally restricted to PDK-relevant passive values rather
than instantiating a BSIM3v3/BSIM4 device card. The MOSFET-driven
Sky130 PDK testbench is documented as a v1 scope deferral in the
companion scenario test — see that file's header for rationale.

### Provenance of the golden values

The values column for `v(n_cap)` is computed *analytically* from the
closed-form RC discharge solution

```
v_C(t) = V0 · exp(−t / τ)
```

at each of the 7 fixed time points `t ∈ {0, 5, 10, …, 30} ns`.

ngspice is **not** installed on the host that produced this fixture,
and the project does not yet ship an automated golden-regeneration
pipeline. The analytic-RC closed form *is* the ground truth ngspice
itself would integrate toward (an RC discharge is a single linear
ODE; ngspice's trapezoidal-method integration of it is convergent to
this analytic curve at the LTE tolerances ADR-0008 sets). Treating
the analytic curve as the golden value is therefore physically
faithful for **this passive bench** — it would not be faithful for a
nonlinear MOSFET bench, which is the reason such a bench is deferred
as documented in the consumer test's header.

Precision: 6 decimal places in `%.6e` ngspice rawfile convention.
The corresponding rounding error at `t = 3 ns` is `< 5 · 10⁻⁸` V,
five orders of magnitude below the 1 mV ADR-0008 absolute floor —
the fixture does not introduce a measurable contribution to the
conformance margin.

### Variables

| col | name      | unit    | role        |
|-----|-----------|---------|-------------|
| 0   | `time`    | seconds | sweep axis  |
| 1   | `v(n_cap)`| volts   | observed    |

The bench has a single state-bearing node (`n_cap`) — both `R1` and
`C1` connect it directly to circuit ground, so MNA introduces no
voltage-source branch-current state. The single observed waveform
spans the full ADR-0008 envelope range: at `t = 0` the relative term
governs (envelope = 10 mV at 1 V reference); at `t = 25–30 ns` the
absolute floor governs (envelope = 1 mV at < 100 mV reference). Both
branches of the `max(rel, abs)` envelope are therefore exercised by
this single variable across the sweep.

### Updating the fixture

If a future tasks.md item adds a Sky130 BSIM3v3/BSIM4 testbench, do
**not** edit this fixture — emit a new one alongside (e.g.
`sky130_inverter_transient.raw`) and gate its consumer test on the
presence of the device-modeling integration. The current fixture's
contract is exactly the passive-RC discharge documented above.
