---
title: "ADR-0007: Zero-Order Hold Default at Analog-Digital Boundary"
adr_id: ADR-0007
status: accepted
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
supersedes: []
superseded_by: null
asr:
  - "Analog-digital boundary signal exchange must define a deterministic interpolation scheme when the analog timestep does not land exactly on a digital event time."
tags: [mixed-signal, boundary-interpolation, scheduler, analysis-orchestration]
created: 2025-07-18
---

# ADR-0007: Zero-Order Hold Default at Analog-Digital Boundary

## Y-Statement

**In the context of** mixed-signal co-simulation where the analog solver's adaptive timesteps rarely align with digital event times,
**facing** the need for a deterministic, charge-conserving signal exchange scheme at the analog-digital boundary,
**we decided for** zero-order hold (ZOH) as the default interpolation method — the analog value at the last accepted timestep is held constant until the digital event time — with linear interpolation available as a per-request option,
**and against** always using linear interpolation or requiring exact timestep alignment,
**to achieve** charge conservation at the boundary and simplicity of implementation, avoiding the need to interpolate solver state between accepted time points,
**accepting** that ZOH introduces stairstep artifacts on fast digital edges and may require smaller analog timesteps near edges for acceptable accuracy.

## Architecturally Significant Requirement

The analog-digital boundary interpolation scheme is architecturally significant because it determines the accuracy and charge conservation properties of every mixed-signal simulation. The spec/mixed-signal-cosim scenario "Analog-digital boundary signal exchange" requires that both simulators proceed from the synchronization point with exchanged boundary values, but does not specify how values are determined when the analog solve does not produce a sample exactly at the event time. The [[concepts/mixed-level-simulation]] pitfall warns: "Discrete/analog timestep coordination can introduce subtle artifacts at boundary crossings."

## Options Considered

### Option A — Linear interpolation at boundary
Interpolate analog values linearly between the last two accepted time points to estimate the value at the digital event time.

- **Pros:** More accurate signal representation for fast edges; reduces stairstep artifacts; smoother boundary waveforms.
- **Cons:** Requires access to the two most recent analog solution vectors, which may not both be in memory if checkpoints were sparse; does not guarantee charge conservation (linear interpolation of companion-model outputs can inject or remove charge); complicates the sub-view extraction in the numeric solver.

### Option B — Force analog timestep alignment
Require the adaptive timestepper to place a step exactly at every predicted digital event time.

- **Pros:** No interpolation needed; exact sample at the boundary; simplest reasoning about correctness.
- **Cons:** Constrains adaptive timestepping, potentially producing many very small steps that degrade performance; conflicts with ADR-0004's optimistic time-advance strategy where the analog solver runs ahead with native adaptive steps; LTE estimator may reject forced steps.

### Option C — Zero-order hold with linear opt-in (chosen)
Default to ZOH: hold the last accepted analog value constant until the event time. Offer linear interpolation as a per-request option via `AnalysisRequest` parameters.

- **Pros:** ZOH is charge-conserving by construction (constant voltage/current implies zero injected charge over the hold interval); no need to access historical solution vectors; matches SPICE convention for event-driven stimuli; simple to implement and reason about.
- **Cons:** Stairstep artifacts on fast edges; user must opt into linear interpolation and accept its charge-conservation tradeoff; ZOH may require tighter timestepping near digital edges to achieve the same accuracy as linear interpolation.
- **Opt-in linear:** When the user selects linear interpolation, the numeric solver retains the two most recent solution vectors and interpolates at the event time; the `BoundarySignalExchanger` component applies the chosen scheme.

## Consequences

- **Positive:** ZOH preserves charge conservation at the boundary, which is critical for transient accuracy in mixed-signal circuits with significant capacitive coupling at the analog-digital interface.
- **Positive:** Default behavior matches SPICE convention, reducing surprise for users migrating from ngspice or other SPICE derivatives.
- **Positive:** The opt-in linear interpolation path is isolated in the `BoundarySignalExchanger` component and does not complicate the default solver path.
- **Negative:** Stairstep artifacts on fast digital edges may cause visible waveform discrepancies in conformance testing; the conformance harness must account for the interpolation scheme when comparing against ngspice golden references.
- **Negative:** Linear interpolation opt-in requires the solver to retain an extra solution vector in memory near boundaries, slightly increasing per-checkpoint memory.
- **Follow-up:** The `MixedSignalScheduler` must pass the interpolation mode to the `BoundarySignalExchanger` at each synchronization point. The `AnalysisRequest` Python API must expose a `boundary_interpolation` parameter (`"zero_order_hold"` | `"linear"`).

## Supersession

This ADR does not supersede any prior ADR. It refines ADR-0004 (optimistic mixed-signal synchronization) by specifying the boundary interpolation scheme that ADR-0004 left open.
