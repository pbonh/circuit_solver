---
adr: 0008
title: Multi-Crate Cargo Workspace Layout
status: accepted
created: 2026-05-30
---

# ADR 0008: Multi-Crate Cargo Workspace Layout

## Status

accepted

## Y-Statement

In the context of a Rust solver decomposed into six bounded-context containers,
facing the risk of accidental cross-boundary coupling in a single flat crate,
we decided for a Cargo workspace with one crate per bounded-context container
under `crates/<name>/` and against a single flat crate (all containers as Rust
modules), to achieve compiler-enforced dependency boundaries, independent
compilation units, and finer-grained incremental rebuilds, accepting more
`Cargo.toml` manifests to maintain and explicit re-exports at each crate
boundary.

## Context

The v1 architecture and the multidomain-solver design both decompose the solver
into six bounded-context containers (frontend, netlist, orchestration, numeric,
devices, digital). In a single flat crate this decomposition is advisory only —
Rust modules do not enforce that `orch` cannot reach into `devices` directly.
A Cargo workspace maps each container to its own crate: the compiler rejects
any access that crosses a crate boundary without an explicit `[dependency]`
declaration. The Rust module/crate visibility model is well-grounded in the KG
([[concepts/ownership]] effective 0.95, [[concepts/zero-cost-abstractions]]
effective 0.95, [[concepts/static-dispatch]] effective 0.95); all from
*The Rust Programming Language*. The six wiki bounded-context entities each
carry effective 1.045. No contradicts edges exist for this choice.

## Decision

Adopt a **Cargo workspace** at the repository root with seven members:

- `circuit-solver` (PyO3 binding, workspace root) — depends only on
  `circuit-solver-frontend`
- `circuit-solver-frontend` (`crates/frontend/`) — depends on
  `circuit-solver-netlist`, `circuit-solver-orchestration`
- `circuit-solver-orchestration` (`crates/orchestration/`) — depends on
  `circuit-solver-netlist`, `circuit-solver-numeric`, `circuit-solver-digital`
- `circuit-solver-numeric` (`crates/numeric/`) — depends on
  `circuit-solver-devices`, `circuit-solver-netlist`
- `circuit-solver-netlist` (`crates/netlist/`) — no domain deps
- `circuit-solver-devices` (`crates/devices/`) — no domain deps
- `circuit-solver-digital` (`crates/digital/`) — no domain deps

Inter-crate access is via explicit Cargo path-deps only; module re-exports
across undeclared boundaries are a build error. The PyO3 binding crate is the
only crate Python loads and it depends on no domain crate except `frontend`,
keeping the ABI surface minimal.

## Consequences

Positive: the Rust compiler enforces bounded-context boundaries — an accidental
use of a type from an undeclared peer is a build error, not a code-review
finding. The three leaf crates (`netlist`, `devices`, `digital`) compile
independently and impose no transitive recompile when unrelated peers change.
Negative: each crate boundary requires explicit re-exports and a `Cargo.toml`
manifest; the `netlist` crate is a shared type-exporter (`FlattenedView`
consumed by both `orchestration` and `numeric`) whose API changes have
downstream cascade cost.

## Sources

- `design.md` (§ Cargo Workspace; § Component Map; multi-crate trade-off)
- `specs/workspace/spec.md`
- KG: [[concepts/ownership]] (0.95), [[concepts/zero-cost-abstractions]] (0.95),
  [[concepts/static-dispatch]] (0.95) — *The Rust Programming Language*
- Bounded contexts: all six `wiki/contexts/*` entities (effective 1.045)
- Inherited confidence (min rollup): 0.95 — recommended-accept; operator-confirmed
