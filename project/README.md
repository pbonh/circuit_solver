# Project Implementation Workspace

This directory holds the actual software being built. The `wiki/` tree
documents *about* this project (vision, contexts, ADRs, specs, glossary);
this tree *is* the project.

This project is implemented in Rust and Python. Build with `cargo build`, verify with `cargo test`, and run the entry point via `python -m circuit_solver`.

## Conventions

- All Hermes Kanban implementation work lands here. The `kanban/board.yaml`
  in the wiki root points workers at this directory (either as a `dir:`
  workspace or as the parent of git worktrees).
- Do **not** put wiki content, ADRs, or specs here. Those belong in
  `../wiki/`.
- Changed-file paths in `## Implementation Evidence` sections (written
  back by `/wiki-kanban-ingest`) must resolve to files under this
  directory. `/wiki-lint` enforces this.
