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
- 2026-05-28T00:20:00Z — scientia — adrs-promoted — default/2026-05-28-multidomain-solver-architecture — Promoted change ADRs 0006/0007 into the canonical wiki at `wiki/decisions/` (KG schema: type=claim, recomputed confidence effective=1.045 for both). Flipped [[decisions/0004-optimistic-mixed-signal-synchronization-via-shared-scheduler]] to `superseded` (status + `superseded_by: decisions/0006-...`; body preserved per write-once discipline), and stamped `supersedes`/`refines` frontmatter on 0006/0007. index.md Decisions table refreshed (rows for 0006/0007; 0004 status→superseded) and count 5→7. recompute_all clean (0 changed); link resolution 99.87%.
- 2026-05-28T00:10:00Z — scientia — pipeline-complete — default/2026-05-28-multidomain-solver-architecture — All six gates passed. proposal-drafted (55 KG citations, 0 dangling) → grill (11 entries: 3 open, 3 counter-claim, 5 hidden-assumption; all addressed) → specs (6 capabilities, 20 scenarios) → design (3 C4 diagrams; pause_and_ask halt resolved via question-for-operator Q1/Q2/Q3) → adrs-accepted (0006 native digital engine supersedes 0004; 0007 codegen seam refines 0005; both recommended-accept at inherited 0.95, operator-confirmed) → tasks-listed (25 tasks, all traces-spec). Wiki ADR-0004 left `accepted` (flip to `superseded` at implementation).
- 2026-05-28T00:00:00Z — scientia — wiki-reshaped — default/2026-05-28-multidomain-solver-architecture — Pre-seed KG remediation. (1) Page ids set to repo-relative path slug (e.g. `concepts/backward-euler`) to match the wiki's path-style wikilinks; link resolution 0.7%→99.87% (12415/12431). (2) 207 `derived-summary` source pages re-tagged `kind: publication` (academic-textbook corpus); 2 meta `journal` pages untouched. (3) Confidence Policy A (operator-confirmed) lifted base by band 0.45→0.70 / 0.65→0.85 / 0.85→0.95 (14/418/734 claims) to reflect trustworthy-textbook provenance; `effective`/`source_count`/`contradicted`/`inputs_hash` recomputed for all 1166 claims (mean effective 0.913, 0 contradicts edges). (4) index.md Confidence column regenerated from effective (184 cells realigned), `updated` 2026-05-28. 1581 files changed.
