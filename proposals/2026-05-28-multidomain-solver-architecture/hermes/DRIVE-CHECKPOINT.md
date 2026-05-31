# Drive Checkpoint — 2026-05-29 ~19:30

Durable resume record for driving the `circuit-solver-beta` kanban to completion.
Written because the orchestrating shell session became corrupted (git output mangled).

## Snapshot at checkpoint
- Board `circuit-solver-beta`: **done=24, ready=2, running=2, todo=45, archived=5** (~76 original cards + dynamically-spawned resolve cards).
- Trunk = `beta` now contains the v1-spec multi-crate `crates/` workspace (operator decision; landed via #11 merge `9f7a442`).
- All agent work is on local git branches `2026-05-28-multidomain-solver-architecture/task-N` (committed, **NOT pushed to any remote** — local-only).

## What is DURABLE (survives session death)
- Kanban board (SQLite under `~/.hermes`), emit-ledger (`hermes/emit-ledger.json`), all `task-N` branches, `beta` trunk, the amended `proposals/.../design.md`, and the memory files.

## What is AT RISK (must be restarted)
- The **board-scoped dispatch daemon** (was PID 2557762). If gone, restart:
  ```
  hermes kanban --board circuit-solver-beta daemon --force --interval 60 --verbose \
    --pidfile /tmp/hermes-csb-daemon.pid > /tmp/hermes-csb-daemon.log 2>&1 &
  ```
  (Plain `daemon` is deprecated → needs `--force`. Do NOT use `hermes gateway run` — its dispatcher is all-board and also runs the unrelated `circuit-solver` board + cron.)
- The **dashboard on :8787** (preflight only): `hermes dashboard --port 8787 --host 127.0.0.1 --no-open --skip-build &`

## PARKED
- **#25** (Icarus golden-trace harness), resolve/integrate card `t_52dd08e3`: merge conflict (`kernel.rs`, `lib.rs` + 7 scientia-migration wiki frontmatter conflicts) into `beta` not auto-resolving. **Operator decision: park, finish the other tasks, return to #25 last.** Resolve manually in a clean shell (the wiki conflicts are the same frontmatter clashes seen on #11's merge).

## Driving rules (see memory `hermes-driving-discipline`)
- Verify only via `git show <branch>:<path>` — **never** `.worktrees/<id>/...` (recycled across tasks).
- Re-derive card IDs each time from `scientia.hermes.ledger.load(cid).entries`.
- Self-block "review-required" (green tests) → `hermes kanban unblock <id>` (agent completes on re-dispatch).
- Integrate merge-conflict → reassign integrate card to `conflict-resolver` + unblock; let the agent resolve from real markers (do NOT hand-merge).
- Reopen a done card: `hermes kanban edit <id> --reopen --status ready --assignee <profile>`.
- Do NOT `complete` cards to assert unverified test success (harness blocks it; integrity).

## Done (24): epic, #11, #13, #20, #22, and their review/integrate chains, etc.
## Dropped: #1 (archived — redundant single-crate netlist; `crates/netlist-graph` already exists).

## UPDATE ~20:50 — INTEGRATION STALL root cause + fix
**Symptom:** pipeline stalled (dispatch Spawned=0) at done=27; `beta` stuck at `9f7a442` (task-11 merge) despite many integrate cards marked "done".
**Root cause:** integrate work was NOT landing on `beta`. Two compounding issues:
1. The **main worktree had `beta` checked out**, so integrator agents (in their own worktrees) couldn't update the shared `beta` ref — they merged into stranded `integrate-*` branches (e.g. #20's merge `209a45d` on `integrate-t_7ea79482`) and marked the card done without `beta` advancing. Evidence: `equivalence.rs` (#20) is on task-20/task-25 branches but NOT on `beta`.
2. The **daemon had cached the beta-pinned state**, so even read-only dispatch returned Spawned=0.
**Fix applied:** `git checkout --detach` in the main worktree (frees `beta` ref) + killed & restarted the daemon. A fresh `dispatch` then Spawned=1 (conflict-resolver on #3). 
**STILL TO VALIDATE:** that `beta` HEAD actually advances past `9f7a442` once an integrate lands. Watcher now tracks `git rev-parse beta`.
**LIKELY FOLLOWUP:** the post-task-11 "done" integrates are stranded on `integrate-*` branches; they may need re-running (reopen integrate cards) so their merges actually land on the freed `beta`. The done=27 is illusory at the integration level until verified.
**To return main worktree to beta later:** `git checkout beta` (after integrations settle; uncommitted design.md/checkpoint changes are in the working tree).

## CORRECTION ~00:50 (05-30) — the "integration stall" diagnosis was WRONG
`beta` was NOT pinned/stalled. It is advancing with real merges (task-6/12/21/22/25 all on `beta`; HEAD `585530b`). The done=27 "stall" was a lagged/transient read during a slow narrow-frontier stretch; over ~6h the daemon drove done 27 -> **42/76**. The `git checkout --detach` was based on that bad read but is harmless (frees the beta ref). Restore later with `git checkout beta`. Ignore the §"INTEGRATION STALL" claim above — superseded by this.
Current frontier reopened: unblocked 5 cards (#8/#9/#10 green review-required self-blocks; #4 Cargo conflict→conflict-resolver; #15 integrate has REAL compile errors in ac_noise.rs [missing AcAnalysisResult::new/TransferFunction::new/CrateAcRequest]→conflict-resolver, watch for re-escalation).
