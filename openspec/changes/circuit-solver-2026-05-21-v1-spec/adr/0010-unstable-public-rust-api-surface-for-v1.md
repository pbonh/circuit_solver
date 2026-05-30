---
title: "ADR-0010: Unstable Public Rust API Surface for v1"
adr_id: ADR-0010
status: accepted
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
supersedes: []
superseded_by: null
asr:
  - "The public Rust API for CircuitGraph, AnalysisRequest, and AnalysisResult must not be semver-frozen at v1.0.0; the release signals feature completeness, not API stability."
tags: [application-frontend, api-stability, semver, rust, pyo3]
created: 2025-07-18
---

# ADR-0010: Unstable Public Rust API Surface for v1

## Y-Statement

**In the context of** the circuit-solver v1 release which introduces the first public Rust API surface across `CircuitGraph`, `AnalysisRequest`, and `AnalysisResult`,
**facing** the need to iterate on these APIs based on user feedback without being constrained by semver stability guarantees,
**we decided for** declaring the public Rust API unstable at v1.0.0 — the version signals feature completeness for the declared scope, not semver-frozen API stability,
**and against** committing to semver stability from v1.0.0 or hiding all types behind a private crate,
**to achieve** the freedom to refine the API based on real-world usage in the Python frontend and downstream integrations,
**accepting** that downstream Rust consumers cannot rely on stable types across minor versions and must pin to exact versions.

## Architecturally Significant Requirement

The proposal declares two breaking changes. The first — "The public Rust API surface for `CircuitGraph`, `AnalysisRequest`, and `AnalysisResult` is not yet stabilized; v1.0.0 signals feature complete for declared scope, not semver frozen" — is an ASR because it sets expectations for all consumers of the crate's public types. Without an ADR, the breaking change is acknowledged in prose but not captured as a decision with alternatives and consequences.

## Options Considered

### Option A — Commit to semver stability from v1.0.0
Treat v1.0.0 as a semver-stable release: any breaking change to public types requires a major version bump.

- **Pros:** Downstream Rust consumers get the standard semver guarantee; predictable upgrade paths.
- **Cons:** Premature commitment: the first release's API shapes are informed by design and spec but not by real-world usage; locking in suboptimal type names, trait bounds, or ownership patterns creates technical debt that can only be cleared with a v2 bump.

### Option B — Hide all public types behind a private crate
Make the Rust crate's public surface minimal (only the PyO3 extension entry point); all types are crate-private.

- **Pros:** No API surface to stabilize; maximum freedom to refactor.
- **Cons:** Prevents any direct Rust consumption of the simulator; contradicts the workspace architecture where `circuit-solver-types` is a shared crate; forces all interaction through Python, even for Rust-native users.

### Option C — Declare API unstable at v1.0.0 (chosen)
Publish the Rust types publicly but document that v1.0.0 does not carry semver stability guarantees. The crate's `Cargo.toml` sets `version = "1.0.0"` but documentation and README explicitly state "API is unstable until a future stabilization announcement."

- **Pros:** API is visible and usable for experimentation; freedom to refine based on feedback; honest about the stability level.
- **Cons:** Downstream Rust consumers must pin exact versions; the 1.0.0 version number creates a convention clash with semver expectations (1.0.0 normally implies stability); requires clear documentation to avoid surprise.

## Consequences

- **Positive:** The team can iterate on `CircuitGraph`, `AnalysisRequest`, and `AnalysisResult` types based on PyO3 integration experience and user feedback without a v2 bump.
- **Positive:** The Python API (which is the primary user-facing interface per ADR-0001) is not affected; Python consumers see a stable module interface regardless of Rust type changes.
- **Negative:** Any Rust crate that depends on `circuit-solver-types` or the workspace crates directly must use `version = "=1.0.0"` or a caret pin that acknowledges the instability risk.
- **Negative:** The "1.0.0 but unstable" signal is unconventional and must be prominently documented to avoid user confusion.
- **Follow-up:** When the API shapes have been validated through at least one minor release cycle of Python-frontend usage, a future ADR should declare API stability and bump to v2.0.0 with semver guarantees.

## Supersession

This ADR does not supersede any prior ADR. It records the proposal's first breaking change as a formal decision. When the API is stabilized, a superseding ADR will be written.
