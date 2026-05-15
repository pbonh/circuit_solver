# Project Implementation Workspace

This directory holds the actual software being built. The `wiki/` tree
documents *about* this project (vision, contexts, ADRs, specs, glossary);
this tree *is* the project.

<!--
CUSTOMIZE via `/wiki-project-init`. Replace this block with a one-paragraph
description of the implementation:
  - language: <primary language(s)>
  - build:    <build / package-manager command, e.g. `cargo build`>
  - test:     <single command `/wiki-lint` and Hermes workers use to verify, e.g. `cargo test`>
  - entry:    <path or command to start the program>
Remove this HTML comment when done.
-->

## Conventions

- All Hermes Kanban implementation work lands here. The `kanban/board.yaml`
  in the wiki root points workers at this directory (either as a `dir:`
  workspace or as the parent of git worktrees).
- Do **not** put wiki content, ADRs, or specs here. Those belong in
  `../wiki/`.
- Changed-file paths in `## Implementation Evidence` sections (written
  back by `/wiki-kanban-ingest`) must resolve to files under this
  directory. `/wiki-lint` enforces this.
