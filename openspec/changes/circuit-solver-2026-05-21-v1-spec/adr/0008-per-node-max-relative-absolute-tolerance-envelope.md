---
title: "ADR-0008: Per-Node max(Relative, Absolute) Tolerance Envelope for Golden-Reference Conformance"
adr_id: ADR-0008
status: accepted
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
supersedes: []
superseded_by: null
asr:
  - "The conformance harness must compare solver results against ngspice golden references using a tolerance formulation that avoids false failures on near-zero nodes and false passes on large-signal nodes."
tags: [conformance, golden-reference, tolerance, dc-analysis, ac-analysis, transient-analysis, noise-analysis]
created: 2025-07-18
---

# ADR-0008: Per-Node max(Relative, Absolute) Tolerance Envelope for Golden-Reference Conformance

## Y-Statement

**In the context of** the conformance harness that compares circuit-solver results against ngspice golden references,
**facing** the conflicting requirements of catching real errors on large-signal nodes while avoiding false failures on near-zero nodes where the reference tool's own numerical noise exceeds a pure-relative bound,
**we decided for** a per-node tolerance formulation `max(rel_pct × |v_ref|, abs_threshold)` — pass if the absolute difference between solver and reference is below the larger of the relative component and the absolute floor,
**and against** pure-relative tolerance, pure-absolute tolerance, and mixed additive (relative + absolute) formulations,
**to achieve** a single deterministic pass/fail criterion per node that handles both large-signal and small-signal regimes without per-node manual tuning,
**accepting** that the absolute floor must be chosen per analysis type and may need adjustment for circuits with unusual dynamic ranges.

## Architecturally Significant Requirement

The tolerance envelope formulation is architecturally significant because it determines whether the v1 release passes or fails conformance against the golden reference. The proposal identifies this as an open question: "What is the exact tolerance envelope for golden-reference conformance — absolute vs. relative, per-node vs. global, and how do we handle nodes that ngspice labels differently after ground suppression?" The [[concepts/golden-reference]] pitfalls warn: "An overly tight envelope causes false failures when the reference tool's own numerical noise exceeds the bound. An overly loose envelope masks real bugs."

## Options Considered

### Option A — Pure-relative tolerance
Pass if |v_solver − v_ref| < rel_pct × |v_ref|.

- **Pros:** Simple; scales naturally with signal magnitude; no absolute threshold to tune.
- **Cons:** Fails on near-zero nodes where |v_ref| ≈ 0 (division by near-zero; any absolute difference fails a 1 % relative check on a 1 µV node); does not protect against small absolute errors on large-signal nodes that are negligible in relative terms but significant in circuit terms (e.g., a 1 mV offset on a 100 V supply rail is 0.001 % relative, but may indicate a convergence issue).

### Option B — Pure-absolute tolerance
Pass if |v_solver − v_ref| < abs_threshold.

- **Pros:** No issues with near-zero nodes; simple to reason about.
- **Cons:** A 1 mV absolute threshold is far too loose for a 1 µV signal and far too tight for a 100 V signal; cannot serve both regimes simultaneously; requires different thresholds for different circuit scales.

### Option C — Additive (relative + absolute) tolerance
Pass if |v_solver − v_ref| < (rel_pct × |v_ref| + abs_threshold).

- **Pros:** Handles both regimes; always non-zero tolerance.
- **Cons:** Over-generous: at large signals the additive term dominates and the effective relative tolerance is much looser than intended; at small signals the relative term dominates and the effective absolute tolerance is much looser than intended; the combined bound is harder to reason about and explain to users.

### Option D — max(relative, absolute) tolerance (chosen)
Pass if |v_solver − v_ref| < max(rel_pct × |v_ref|, abs_threshold).

- **Pros:** At large signals the relative term dominates, providing proportional accuracy; at near-zero signals the absolute floor dominates, preventing false failures; the formulation is symmetric and easy to explain: "1 % relative or 1 mV absolute, whichever is greater"; matches the [[concepts/golden-reference]] pitfall recommendation: "Tolerance bounds that mix relative and absolute terms must be defined unambiguously."
- **Cons:** The absolute floor must be chosen per analysis type and per quantity (voltage, current, magnitude, phase, spectral density); defaults must be documented and may need adjustment for specific PDKs or circuit types; the formulation is slightly more generous than additive at the crossover point.
- **Default thresholds by analysis type:**
  - DC: 1 % relative or 1 mV absolute per node voltage / per branch current
  - AC magnitude: 0.1 dB relative or 0.01 dB absolute per output/input pair
  - AC phase: 1° relative or 0.1° absolute per output/input pair
  - Transient: 1 % relative or 1 mV absolute per time point per node
  - Noise spectral density: 2 % relative or 1 nV/√Hz absolute per frequency point
  - Mixed-signal analog: same as transient; digital: event-trace equivalence at cycle boundaries

## Consequences

- **Positive:** A single, deterministic formulation covers all signal regimes without manual per-node tuning; conformance results are reproducible and explainable.
- **Positive:** Per-node checking means a single outlier node does not cause a global failure; the conformance report can list the worst-case nodes and their margins.
- **Positive:** The formulation applies uniformly to DC, AC, transient, and noise analyses with analysis-specific default thresholds.
- **Negative:** The absolute thresholds are engineering judgments; they may need to be relaxed for specific PDKs (e.g., ASAP7 FinFET models with strong numerical conditioning) or tightened for precision analog circuits.
- **Negative:** Ground-suppressed nodes in ngspice golden files must be re-mapped to circuit-solver's node numbering before comparison; this is a data-processing concern, not a formulation concern.
- **Follow-up:** The conformance harness must read per-analysis tolerance configuration from a YAML or TOML file; defaults are embedded but overridable. The report must list per-node pass/fail with the applicable threshold.

## Supersession

This ADR does not supersede any prior ADR. It resolves the open question from the proposal regarding the tolerance envelope formulation.
