# Scientia-managed project

This project is driven by the Scientia R&D pipeline (learn -> plan -> implement).
`scientia-manifest.json` is the run contract; validate it before each phase.

## Layout

- `scientia-manifest.json` — run manifest (project, mode, wiki, openspec, hermes, outcomes).
- `wiki/` — LLM Wiki root. Run `wiki-init` here to install the toolkit. Learning briefs land in
  `wiki/pages/analyses/`; raw sources and outcome sources in `wiki/sources/`.
- `openspec/changes/<change>/` — OpenSpec change (proposal, specs, decisions, architecture,
  design, tasks.md).
- `plan.json`, `status.json`, `handoff.json` — Hermes Ralph implementation artifacts.

## Run the pipeline

```bash
scientia validate-manifest scientia-manifest.json   # 0. validate the contract
# 1. Learn:     activate scientia-learn  (wiki-init/ingest/query -> learning brief)
# 2. Plan:      activate scientia-plan   (opsx-workflow + c4-modeling + adr-authoring)
# 3. Implement:
scientia tasks-to-plan openspec/changes/<change>/tasks.md --manifest scientia-manifest.json --output plan.json
scientia hermes-setup scientia-manifest.json --execute   # create the project board + worker profile
#     then activate scientia-implement (ralph-plan/loop/report)
# 4. Close the loop:
scientia outcome-source --title "<outcome>" --status status.json \
  --handoff handoff.json --change <change> --plan plan.json \
  --output wiki/sources/<change>-outcome.md
```

Re-run `scientia-init` any time to restore missing skeleton files; it never overwrites content.
