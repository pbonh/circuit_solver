---
title: "Spec: Python Frontend"
type: spec
capability: python-frontend
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
created: 2026-05-21
---

## Source

This spec is a living-documentation mirror of
[openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/python-frontend/spec.md](../../openspec/changes/circuit-solver-2026-05-21-v1-spec/specs/python-frontend/spec.md).

See the source file for the authoritative Gherkin scenarios, glossary,
and acceptance criteria.

## Implementation Evidence

<!-- scientia-ingest-evidence-keyed -->
- **Scenario `zero-copy-numpy-result-arrays`** — task `t_c7037c7a` (key `2026-05-21-v1-spec:task-58:bd85ae4288f3caedb4e14dbea0dbf41d78773a5e7054119ba3e44ce015b9caed`) merged at `e029b4899adf11ed0e8a5f794346b994046f4c97` by `scientia-integrator`. Verification: `cargo test -p circuit-solver-py --no-default-features → 105 passed / 0 failed / 0 ignored; result.rs integration binary → 24 passed / 0 failed / 0 ignored`. Residual risk: Cosmetic Cargo.toml comment/default-features inconsistency; ABI-matching numpy venv required for integration tests; from_channels adapter dormant pending Simulator.run wiring. Changed files: 5.
- **Scenario `zero-copy-numpy-result-arrays`** — task `t_3aa46a43` (key `python-frontend:ADR-0006:zero-copy-numpy-result-arrays:eb66eec1b22d6ab2a01e3f9a208fc719d0e46bff884fc06941adf2591941bcb3`) merged at `a588b07c1b6d61d9a3a9a0ee2e86fc02c8d8d882` by `scientia-integrator`. Verification: `cargo test -p circuit-solver-py --no-default-features → 105 passed, 0 failed; cargo clippy -p circuit-solver-py --no-default-features → clean`. Residual risk: Prior merge e029b489 (from t_c7037c7a) is not an ancestor of current main; this replacement merge a588b07 closes the gap. Atomic CAS update-ref used to prevent main race. Changed files: 6.
