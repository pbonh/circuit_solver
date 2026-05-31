---
title: "Per-Node max(Relative, Absolute) Tolerance Envelope for Golden-Reference Conformance"
type: decision
tags: [decision, circuit-solver, conformance, golden-reference, tolerance, dc-analysis, ac-analysis, transient-analysis, noise-analysis]
created: 2025-07-18
sources:
  - "openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0008-per-node-max-relative-absolute-tolerance-envelope.md"
confidence: high
---

"In the context of the conformance harness that compares circuit-solver results against ngspice golden references, facing the conflicting requirements of catching real errors on large-signal nodes while avoiding false failures on near-zero nodes, we decided for a per-node tolerance formulation max(rel_pct × |v_ref|, abs_threshold), and against pure-relative, pure-absolute, and additive formulations, to achieve a single deterministic pass/fail criterion per node that handles both large-signal and small-signal regimes, accepting that the absolute floor must be chosen per analysis type and may need adjustment for unusual dynamic ranges."

## Status

accepted

## Architecturally Significant Requirement

The tolerance envelope formulation determines whether the v1 release passes or fails conformance. The proposal identifies this as an open question. The [[concepts/golden-reference]] pitfalls warn: "An overly tight envelope causes false failures; an overly loose envelope masks real bugs."

## Options Considered

- **Pure-relative** — fails on near-zero nodes.
- **Pure-absolute** — cannot serve both large and small signal regimes.
- **Additive (rel + abs)** — over-generous at both extremes.
- **max(relative, absolute) (chosen)** — proportional accuracy on large signals; absolute floor prevents false failures on small signals; easy to explain.

## Default Thresholds

| Analysis | Relative | Absolute |
|---|---|---|
| DC | 1 % | 1 mV |
| AC magnitude | 0.1 dB | 0.01 dB |
| AC phase | 1° | 0.1° |
| Transient | 1 % | 1 mV |
| Noise | 2 % | 1 nV/√Hz |
| Mixed-signal analog | same as transient | same as transient |

## Consequences

- Single deterministic formulation covers all regimes; conformance results are reproducible.
- Per-node checking; report lists worst-case nodes and margins.
- Absolute thresholds are engineering judgments; may need PDK-specific adjustment.

## Source

- OpenSpec ADR: `openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0008-per-node-max-relative-absolute-tolerance-envelope.md`
