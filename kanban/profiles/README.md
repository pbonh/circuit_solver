# Expected Hermes Profiles

The slash commands in this wiki assume the following Hermes profiles
exist on every machine that runs `/wiki-kanban-emit` or
`/wiki-kanban-ingest`. The exact names are configurable in
`../board.yaml`; the *roles* are not.

| Role | Purpose | Required skills |
| --- | --- | --- |
| orchestrator | Decomposes specs into tasks via `kanban_create` / `kanban_link`. Never executes implementation work. | `kanban-orchestrator` |
| worker | Implements one task per spawn. Reads `kanban_show()`, edits files in `project/`, completes with structured handoff per `prompts/_handoff-schema.md`. | `kanban-worker` |
| reviewer | Reads completed work, votes in P3 quorum, opens review tasks. | `kanban-worker`, plus any code-review skills (e.g. `github-code-review`) |

Verify with:

    hermes kanban assignees
    hermes -p <profile> skills list

Missing skill on a profile? Restore via:

    hermes -p <profile> skills reset <skill> --restore
