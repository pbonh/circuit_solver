---
status: archived
task_id: t_6c4bab8b
tenant: circuit-solver
change_id: 2026-05-21-v1-spec
spec_scenario: ac-small-signal#ac-conformance-against-ngspice
tasks_md_item: 64
assignee: scientia-integrator
role: integrate
parent_tasks: [t_2e93b02a, t_5c76ecce]
merge_commit: 65898aaf71cd6cd2adf6d993739a08dc514b2601
archived_at: 2026-05-23T04:17:40Z
---

# Integrate: AC conformance test — Sky130 PDK, 0.1 dB / 1° phase

Rebased `impl/ac-conformance-sky130-t_9cf1d756` onto main, resolved
comment-block additive collision in `crates/analysis-orchestration/Cargo.toml`
(via prior fixup parent t_5c76ecce), passed all preflights, merged as
`65898aaf71cd6cd2adf6d993739a08dc514b2601`, and evidence-ingested into
`wiki/specs/ac-small-signal.md`.
