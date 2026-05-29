# Development Log

Append-only audit trail of pipeline state transitions managed by the
`scientia` orchestrator and its phase skills.

Format:

```
- YYYY-MM-DDTHH:MM:SSZ — <skill> — <event> — <tenant>/<change-id> — <details>
```

Events include: `bootstrap-complete`, `manifest-bound`, `proposal-drafted`,
`spec-authored`, `design-drafted`, `adr-accepted`, `tasks-listed`,
`verified`, `emitted`, `evidence-appended`, `synthesized`, `archived`,
`gate-override`, `gate-blocked`.

<!-- entries appended by scientia skills -->
- 2026-05-21T20:28:53Z — scientia-wiki-init — bootstrap-complete — bundle 0.1.0
- 2026-05-21T20:30:01Z — scientia-wiki-lint — completed — — critical=0 warning=11 suggestion=1355
- 2026-05-28T00:00:00Z — scientia — wiki-reshaped — default/2026-05-28-multidomain-solver-architecture — Pre-seed KG remediation. (1) Page ids set to repo-relative path slug (e.g. `concepts/backward-euler`) to match the wiki's path-style wikilinks; link resolution 0.7%→99.87% (12415/12431). (2) 207 `derived-summary` source pages re-tagged `kind: publication` (academic-textbook corpus); 2 meta `journal` pages untouched. (3) Confidence Policy A (operator-confirmed) lifted base by band 0.45→0.70 / 0.65→0.85 / 0.85→0.95 (14/418/734 claims) to reflect trustworthy-textbook provenance; `effective`/`source_count`/`contradicted`/`inputs_hash` recomputed for all 1166 claims (mean effective 0.913, 0 contradicts edges). (4) index.md Confidence column regenerated from effective (184 cells realigned), `updated` 2026-05-28. 1581 files changed.
