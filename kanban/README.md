# Hermes Kanban Staging

This directory is the bridge between the wiki and a Hermes Kanban board.
Everything here is either declarative configuration (checked in) or
volatile workspace state (gitignored).

## Files

- `board.yaml` — single source of truth read by `/wiki-kanban-emit`,
  `/wiki-kanban-ingest`, `/wiki-kanban-board`, and `/wiki-lint`. Declares
  the board slug, profile-to-role mapping, and default workspace shape.
- `profiles/` — README documenting which Hermes profiles this board
  expects to find on the operator's machine. Pure documentation; no
  runtime effect.
- `handoffs/<task-id>.<run-id>.json` — sanitized structured-handoff
  metadata from completed Hermes runs, written by `/wiki-kanban-ingest`.
  One file per attempt (mirroring the per-attempt `### Attempt N`
  subsections in the spec page's `## Implementation Evidence`).
  Append-only; never edited by hand. Feeds `/wiki-refine`.
- `.worktrees/` *(gitignored)* — root for `--workspace worktree` runs.
- `logs/` *(gitignored)* — pointers / copies of `~/.hermes/kanban/logs/`
  entries the operator wants to keep with the project.

## Initialization

Run `/wiki-kanban-board <slug>` once per wiki to bind a Hermes board to
this directory. It creates the board, validates that the declared
profiles exist via `hermes kanban assignees`, and writes `board.yaml`.

`/wiki-project-init` must run first — it scaffolds `project/` and the
rest of this directory, and the manifest gate `board_bound` requires
`project_init` to be satisfied.
