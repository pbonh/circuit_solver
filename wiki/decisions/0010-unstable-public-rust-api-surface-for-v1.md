---
title: "Unstable Public Rust API Surface for v1"
type: decision
tags: [decision, circuit-solver, application-frontend, api-stability, semver, rust, pyo3]
created: 2025-07-18
sources:
  - "openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md"
confidence: high
---

"In the context of the circuit-solver v1 release which introduces the first public Rust API surface, facing the need to iterate on these APIs based on user feedback without semver constraints, we decided for declaring the public Rust API unstable at v1.0.0 — feature complete, not semver frozen — and against committing to semver stability from v1 or hiding all types behind a private crate, to achieve freedom to refine the API based on real-world usage, accepting that downstream Rust consumers must pin exact versions."

## Status

accepted

## Architecturally Significant Requirement

The proposal's first breaking change declares that the Rust API is not semver-frozen at v1.0.0. This ADR captures that declaration as a formal decision with alternatives and consequences.

## Consequences

- Freedom to iterate on API shapes without v2 bumps.
- Python API is not affected (primary user-facing interface per ADR-0001).
- Rust consumers must pin exact crate versions.
- "1.0.0 but unstable" is unconventional; must be prominently documented.
- Future ADR should declare API stability after validation.

## Source

- OpenSpec ADR: `openspec/changes/circuit-solver-2026-05-21-v1-spec/adr/0010-unstable-public-rust-api-surface-for-v1.md`
