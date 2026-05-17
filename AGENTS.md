# Circuit Simulation Knowledge Base — Schema

## Purpose

This is an LLM-maintained knowledge base on a unified view of analog, digital, and mixed-signal circuit simulation. Graph representations model the circuit netlist, mathematical solvers consume the netlist and produce a unified structure for simulation results. The LLM writes and maintains all files under `wiki/`. The human curates raw sources and directs queries. The human never edits wiki files directly.

## Directory Layout

- `raw/` — Immutable source documents (transcripts, articles, notes). Never modify these.
- `wiki/index.md` — Master catalog. Every wiki page must appear here.
- `wiki/log.md` — Append-only activity log.
- `wiki/summaries/` — One summary page per raw source document.
- `wiki/concepts/` — Concept, strategy, and framework pages.
- `wiki/entities/` — Entity pages (people, tools, organizations, products — whatever "things" exist in your domain).
- `wiki/syntheses/` — Comparison tables, decision frameworks, cross-cutting analyses.
- `wiki/specs/` — Gherkin spec artifacts synthesized from user goals (one `.feature`-style page per spec). Populated by the Spec workflow.
- `wiki/journal/` — Research or session journal entries.
- `wiki/flashcards/` — One flashcard file per source wiki page, formatted for the [obsidian-spaced-repetition](https://github.com/st3v3nmw/obsidian-spaced-repetition) plugin. Populated by the Flashcards workflow.
- `wiki/presentations/` — Marp slide decks synthesized from wiki content. Populated by the Presentation workflow.
- `wiki/books/` — Long-form mdBook renderings of source PDFs. Each book is its own directory (`wiki/books/<slug>/`) with `book.toml`, `src/SUMMARY.md`, chapter files, and `src/images/`. Populated by the PDF → mdBook workflow.
- `wiki/vision/` — One [[concepts/domain-vision-statement]] per R&D effort. Declares what is differentiating and bounds the scope of the rest of the pipeline. Populated by the Strategy workflow.
- `wiki/contexts/` — One page per [[concepts/bounded-context]] — the models that participate in the R&D effort and the slice of the wiki each is pinned to. Populated by the Strategy workflow.
- `wiki/context-maps/` — One [[concepts/context-map]] per R&D effort, documenting translation rules and [[concepts/false-cognate]]s between bounded contexts. Populated by the Strategy workflow.
- `wiki/grills/` — One [[concepts/grill]] page per R&D topic, recording the decision tree, depth-first Q&A log, decisions made, and parked open questions surfaced *before* specs are written. Populated by the Grill workflow.
- `wiki/architecture/` — One architecture page per R&D topic, containing the Mermaid C4 diagrams that answer a stated `## Purpose` and the `## Decisions Surfaced` list that `/wiki-adr` consumes. Populated by the Architecture workflow.
- `wiki/decisions/` — Architectural Decision Records, file-named `NNNN-kebab-title.md` (monotonically increasing across the wiki). Populated by the ADR workflow. **Write-once** — never edit an `accepted` ADR; supersede it with a new one and set the predecessor's status to `superseded by NNNN`.

## File Naming

- All lowercase, hyphens for word separation: `concept-name.md`
- No spaces, no special characters, no uppercase
- Name should match the page title slug

## Page Format

Every wiki page uses this frontmatter and structure:

```yaml
---
title: "Page Title"
type: concept | entity | summary | synthesis | spec | flashcards | presentation | vision | context | context-map | grill | architecture | decision
tags: [tag1, tag2, tag3]
created: YYYY-MM-DD
updated: YYYY-MM-DD
sources: ["raw/filename.txt"]
confidence: high | medium | low
---
```

### Required Sections by Page Type

**Summary pages** (`wiki/summaries/`):
- `## Key Points` — Bulleted list of main claims/ideas
- `## Relevant Concepts` — Links to concept pages this source touches
- `## Source Metadata` — Type of source, author/speaker, date, URL or identifier

**Concept pages** (`wiki/concepts/`):
- `## Definition` — One-paragraph plain-English definition
- `## How It Works` — Mechanics, process, or structure of the concept
- `## Key Parameters` — Important variables, dimensions, or factors
- `## When To Use` — Situations and contexts where this concept applies
- `## Risks & Pitfalls` — Known failure modes, common mistakes, limitations
- `## Related Concepts` — Wiki links to related pages
- `## Sources` — Which raw sources inform this page

**Entity pages** (`wiki/entities/`):
- `## Overview` — What this entity is
- `## Characteristics` — Key properties, attributes, structure
- `## Common Strategies` — Links to concept pages for strategies or methods associated with this entity
- `## Related Entities` — Links to related entity pages

**Synthesis pages** (`wiki/syntheses/`):
- `## Comparison` — Table or structured comparison
- `## Analysis` — Cross-cutting insights
- `## Recommendations` — When to prefer which approach
- `## Pages Compared` — Links to all pages involved

**Spec pages** (`wiki/specs/`):
- `## Goal` — One-paragraph restatement of the user's intent
- `## Scope` — *Conditional, emitted only when the goal needed impact-mapping.* Table of `Actor | Impact | Deliverable` rows
- `## User Stories` — One block per story: `**Story:** As a <actor>, I want <capability>, so that <outcome>.` followed by `**Acceptance criteria:**` as a bulleted list of binary pass/fail statements
- `## Scenarios` — Gherkin feature(s) inside ` ```gherkin ` fenced code blocks. Quality rules:
  - Business-readable: never reference UI buttons, database tables, or CSS selectors. Steps describe outcomes ("the invoice is marked paid"), not interactions ("she clicks the Pay button")
  - Single `When` per scenario — keep behaviors decoupled
  - Realistic data: named personas and concrete numbers, not generic placeholders
  - Use `Scenario Outline` + `Examples` tables when multiple cases share the same structure
- `## Glossary` — `Term — definition` lines for the ubiquitous-language terms used in the scenarios; link any term that already exists as `concepts/<term>`
- `## Sources` — Wiki links to every concept or entity page cited in the scenarios or glossary

**Flashcard pages** (`wiki/flashcards/`):
- `tags:` frontmatter must include `flashcards` — the obsidian-spaced-repetition plugin uses this tag to discover cards
- `## Source` — Single wiki link back to the page these cards were derived from
- `## Cards` — The cards themselves, separated by blank lines, using the multi-line formats:
  - Basic: `Question` on one line, `?` on its own line, `Answer` on the next line
  - Reversed (term ↔ definition pairs that should drill both ways): `Question` / `??` / `Answer`
  - Cloze (fill-in-the-blank inside a sentence): wrap the hidden span in `==double-equals==`

**Presentation pages** (`wiki/presentations/`):
- Frontmatter must include `marp: true` plus `theme: default` and `paginate: true` (Marp configuration), in addition to the standard wiki frontmatter
- Slides are separated by `---` on its own line (standard Marp convention)
- `## Outline` slide near the top listing the sections of the deck
- `## References` slide at the end listing every cited wiki page as a wiki link

**Book directories** (`wiki/books/<slug>/`):
- A complete, runnable [mdBook](https://rust-lang.github.io/mdBook/) project, not a single markdown file.
- Required files: `book.toml` (mdBook config), `src/SUMMARY.md` (chapter list), one markdown file per chapter under `src/`, optional `src/images/` for figures.
- `src/SUMMARY.md` follows mdBook syntax strictly: chapters as `- [Title](path.md)`, two-space indent for sub-chapters, `# Part Title` for parts, `---` on its own line for separators. mdBook will refuse to build if this file is malformed.
- Each chapter file starts with a single `# Chapter Title` H1 matching its `SUMMARY.md` entry; sub-sections use `##`/`###`.
- Discoverability comes from the `## Books` row in `wiki/index.md`, which links directly to `[[books/<slug>/src/SUMMARY|<Title>]]`. The PDF → mdBook workflow does **not** produce a paired `wiki/summaries/<slug>.md`; if a summary or concept extraction is wanted, the user invokes the Ingest workflow separately on the source PDF or the rendered book.

**Vision pages** (`wiki/vision/`):
- `## Value Proposition` — One paragraph naming what this R&D effort delivers that is differentiating. Plain English, no jargon.
- `## In Scope` — Bulleted list of capabilities, audiences, or domains this effort covers.
- `## Out of Scope` — Bulleted list of what is explicitly excluded, especially things a reader might assume are in.
- `## Differentiation` — How this effort differs from adjacent or competing approaches. Cite specific alternatives where relevant.
- `## Revisions` — Append-only dated log of scope changes. Each entry: `YYYY-MM-DD — what changed and why`.

**Context pages** (`wiki/contexts/`):
- `## Model` — The model that lives inside this [[concepts/bounded-context]]: its core entities, key invariants, and the language it uses.
- `## Boundary` — Where this context starts and stops. Name the adjacent contexts and the artifacts that cross the boundary (events, payloads, translated terms).
- `## Ubiquitous Language` — Inline glossary (`Term — definition`) for every term that has a specific meaning inside this context. Inline the definitions; do not just link out. This is the per-context dictionary that prevents drift.
- `## Relationships` — Links into the relevant `wiki/context-maps/` page(s) showing how this context relates to its neighbours.

**Context-map pages** (`wiki/context-maps/`):
- `## Contexts` — Bulleted list of links to every `wiki/contexts/<slug>.md` page that participates in this map.
- `## Translations` — Markdown table: `Term in A | Context A | Term in B | Context B | Notes`. One row per cross-boundary term that needs explicit translation.
- `## False Cognates` — Bulleted list of terms that *look* identical across contexts but mean different things. Each entry calls out both meanings and why conflating them would be a bug ([[concepts/false-cognate]]).
- `## Integration Patterns` — Which integration pattern governs each pair of contexts: `shared-kernel`, `customer-supplier`, `conformist`, `anticorruption-layer`, `published-language`, `open-host-service`, `separate-ways`. Link to concept pages where they exist.

**Grill pages** (`wiki/grills/`):
- `## Decision Tree` — Numbered 3–7 top-level decisions with indented sub-questions surfaced during interrogation. Top-level entries keyed by short title rewrite in place on re-run; sub-questions append under their parent.
- `## Q&A Log` — Numbered `Q<n>` / `A<n>` pairs, exact wording preserved, oldest-first. **Append-only**, global numbering. Resume picks up at `Q<next>`.
- `## Decisions Made` — One bullet per resolved decision: `**<title>** — <picked option>. <one-line rationale>`. Keyed by `<title>` (same title rewrites, new titles append).
- `## Open Questions` — Parked items, verbatim. **Append-only across runs**; never re-asked in the same run.
- `## Cross-Links` — Backlinks to strategy / contexts / concepts; forward placeholders to architecture / ADR / spec.
- Optional `## Status` — `in progress (resume with /wiki-grill <topic>)` mid-grill; `done` (or omitted) on clean exit.

**Architecture pages** (`wiki/architecture/`):
- `## Purpose` — One-sentence question this diagram set answers. **Hard gate**: page is not written until this is non-empty. *"general system architecture"* fails the gate; *"How does a rate request flow through cache, carrier adapters, and aggregator when one carrier times out?"* passes.
- `## System Context` / `## Container Diagram` / `## Component Diagram` / `## Dynamic Diagram` / `## Deployment Diagram` — Mermaid blocks for whichever C4 levels were chosen (`C4Context` / `C4Container` / `C4Component` / `C4Deployment` for the static levels; `sequenceDiagram` or `flowchart` for Dynamic). Default to Context + Container only. Refuse all five without a stated question for each level.
- `## Assumptions` — Bulleted list of assumptions the diagrams bake in (e.g. *"carrier APIs return within 2s under nominal load"*). Append-only across runs.
- `## Open Questions` — Anything the diagrams could not resolve; carries forward to `/wiki-adr` or `/wiki-refine`. Append-only.
- `## Decisions Surfaced` — Bulleted list, one bullet per architectural decision the diagrams surface: `**<short decision title>** — <one-line summary>`. **Load-bearing for `/wiki-adr`** — every accepted ADR must cite a bullet here. Same title rewrites; new titles append. After `/wiki-adr` runs, the matching bullet is upserted with `→ ADR-NNNN`.
- `## Cross-Links` — Backlinks to vision / grill / cited contexts / concepts; forward placeholders to `[[wiki/decisions/...]]` and `[[wiki/specs/<topic-slug>]]`.

**Decision pages** (`wiki/decisions/`):
- File-named `NNNN-kebab-title.md`, with `NNNN` monotonically increasing across the wiki.
- Frontmatter `type: decision`. Tags should include `decision` plus a domain tag.
- Optional top-of-file [[concepts/y-statement]] one-liner as an executive summary: *"In the context of `<X>`, facing `<Y>`, we decided for `<Z>` to achieve `<Q>`, accepting `<W>`."*
- `## Status` — One of `proposed | accepted | deprecated | superseded by NNNN`. The status field is load-bearing: downstream workflows (Spec, Kanban Emit, Lint) gate on it. When superseding, fill in the successor id.
- `## Context` — Must cite the triggering [[concepts/architecturally-significant-requirement]] (ASR) — the requirement whose effect on structure or quality attributes made this decision necessary. ADRs without an ASR citation are the AKM log-bloat failure mode; they should not be promoted.
- `## Decision` — The chosen direction, stated as a commitment. One paragraph.
- `## Consequences` — Positive, negative, and neutral consequences of the decision. Be honest about the costs.
- Optional (MADR-style, when alternatives deserve preserved analysis):
  - `## Decision Drivers` — The constraints, quality attributes, or stakeholder asks that shaped the trade-off.
  - `## Considered Options` — Bulleted list of alternatives with pros/cons each.
- `## Related Decisions` — Links to ADRs that this one supersedes, is superseded by, or depends on.

ADR discipline (per [[concepts/architectural-decision-record]] and [[concepts/decision-log]]):
- **Write-once.** Once `## Status` is `accepted`, the body of the ADR does not get edited — fix the wording before acceptance. To change an accepted decision, open a new ADR that supersedes it.
- **Numbered, never deleted.** Even superseded and deprecated ADRs remain in `wiki/decisions/`; the log is the history.
- **One decision per ADR.** Do not bundle multiple commitments in one record.

## Linking Conventions

- Use Obsidian-style wiki links: `[[concepts/concept-name]]`
- Always use relative paths from wiki root
- Every page must link to at least one other page (no orphans)
- When mentioning a concept that has a page, always link it

## Tagging Taxonomy

- **Signal Domain**: `analog`, `digital`, `mixed-signal`, `rf`
- **Representation**: `netlist`, `graph`, `sparse-matrix`, `device-model`
- **Analysis**: `dc`, `ac`, `transient`, `noise`, `harmonic-balance`
- **Scope**: `foundational`, `advanced`, `experimental`
- **Status**: `well-established`, `emerging`, `speculative`

## Confidence Levels

- **high** — Well-established idea, multiple corroborating sources, demonstrated with concrete examples
- **medium** — Supported by sources but limited examples or single-source
- **low** — Single mention, anecdotal, or speculative

## Pipeline manifest

`wiki/.pipeline.yaml` declares the ordered artifacts that make up the R&D pipeline and which prerequisites each phase requires. Every slash command calls `scripts/check-prereqs.sh <artifact> [--slug <slug>] [--branch-head <sha>]` before doing any work, and aborts on the first missing prerequisite. The manifest is the single source of truth — prompts do not hardcode dependency lists.

### Schema (v1)

```yaml
version: 1

defaults:
  trunk_branch: main           # branch the kanban round-trip gates on
  enforcement: strict          # strict | warn | off

artifacts:
  <name>:
    produces: <path-or-uri>    # file path with {slug}/NNNN placeholders, OR a URI (kanban://...), OR a fragment (file#section)
    requires: [<name>, "git:<check-name>", ...]
    command: /wiki-<slash>     # surfaced in error hints
    optional_in_v1: false      # true means "not-produced is not a failure"

git_checks:
  <check-name>:
    description: <one-line>
    test: |
      <bash snippet exiting 0 on success; supports {{trunk_branch}}, {{spec_path}}, {{branch_head}}>
```

### Default pipeline (shipped in `template/wiki/.pipeline.yaml`)

```
strategy ──> grill ──> architecture ──> adr ──> spec ──> kanban_emit ──> kanban_ingest ──> refine
                                                          ↑   ↑              ↑   ↑
                                                          │   git:spec-on-trunk │
                                                          │                     git:worker-branch-merged
                                                          board_bound ─── project_init
```

- `grill` and `architecture` are `optional_in_v1: true` — they warn rather than block when missing. v2 tightens them to required.
- `git:spec-on-trunk` blocks `/wiki-kanban-emit` while the spec is on a feature branch (idempotency keys would hash a moving target).
- `git:worker-branch-merged` blocks `/wiki-kanban-ingest` while the worker's branch is unmerged (the wiki would record evidence pointing at non-trunk code).
- `project_init` and `board_bound` are infrastructure artifacts. `project_init` produces `project/README.md` (filled in by `/wiki-project-init`). `board_bound` requires `project_init` and produces `kanban/board.yaml` (filled in by `/wiki-kanban-board`). Both gate the kanban round-trip — emit and ingest will refuse to run until the board is bound.

### Enforcement modes

- `strict` — missing prereqs exit non-zero; slash commands abort. The default.
- `warn` — missing prereqs print to stderr but exit 0; slash commands proceed. Use during migration.
- `off` — the helper exits 0 immediately. No checks run.

### Back-compat

Absence of `wiki/.pipeline.yaml` is **not an error**. The helper exits 0 and slash commands run as if there were no enforcement. Wikis predating Phase 0 continue working unchanged.

### Adding a new artifact

1. Add an entry under `artifacts:` with `produces`, `requires`, and `command`.
2. Prepend a `scripts/check-prereqs.sh <new-artifact>` call to the slash command's prompt.

That's it — no other files reference the artifact list directly.

See `docs/rd-pipeline/pipeline-manifest.md` in the `pi-llm-wiki` source for the full reference (exit codes, JSON output, failure modes, idempotency rules).

## Implementation Workspace

The implementation of this project lives in `project/` (sibling of `wiki/`). It is implemented in Rust and Python: build with `cargo build`, verify with `cargo test`, and run the entry point via `python -m circuit_solver`. Every changed-file path written by `/wiki-kanban-ingest` into a spec's `## Implementation Evidence` section must resolve under that directory. `/wiki-lint` enforces this.

`/wiki-project-init` scaffolds the directory and walks the customization marker; the manifest artifact `project_init` flips green once the marker is filled in.



## Kanban Board

This wiki is bound to a Hermes Kanban board via `kanban/board.yaml`. Tasks are emitted by `/wiki-kanban-emit` and round-tripped by `/wiki-kanban-ingest`; both pass `--board <slug>` from `board.yaml` and resolve assignees via its `profiles:` section. See `kanban/README.md` for the file layout.

`/wiki-kanban-board <slug>` binds the board and writes the YAML; the manifest artifact `board_bound` requires `project_init` and gates `kanban_emit` and `kanban_ingest`.



## Workflows

### Ingest

When the user says "ingest [source]" or adds a file to `raw/`:

1. Read the raw source completely
2. Create `wiki/summaries/<source-slug>.md` with full summary
3. Identify all concepts, entities, and strategies mentioned
4. For each concept/entity: create the page if it doesn't exist, or update it with new information if it does
5. Add cross-links in both directions between all touched pages
6. **Verify zero dangling links — acceptance gate.** Before declaring the ingest complete, scan every page you created or updated and list every `[[...]]` reference. For each link, confirm the target file exists on disk. For each dangling link, either (a) create the missing page now with the full schema for its type (a stub with frontmatter + required sections is fine if the source only mentions the concept in passing — set `confidence: low` and note the source mention), or (b) remove the link from the citing page when the source genuinely does not support a standalone page. Re-run the scan until it returns zero dangling links. A summary that cites pages which do not exist is a workflow failure, not a partial success. Every `[[wiki-link]]` you write must resolve to a real file by the end of the run.
7. Update `wiki/index.md` — add new entries, update summaries of changed pages
8. Append to `wiki/log.md` with timestamp, source name, pages created/updated
9. Flag any contradictions with existing wiki content

### Query

When the user asks a question:

1. Read `wiki/index.md` to find relevant pages
2. Read those pages
3. Synthesize an answer citing specific pages with wiki links
4. If the answer reveals new insight worth preserving:
   - Create a synthesis page in `wiki/syntheses/`
   - Update index and log

### Lint

When the user says "lint" or "health check":

1. Read all wiki pages
2. Check for: orphan pages (no inbound links), stale claims, contradictions between pages, missing cross-links, incomplete sections, low-confidence pages that could be strengthened
3. **Pipeline-manifest checks** (run when `wiki/.pipeline.yaml` exists; skip cleanly when absent):
   - **Manifest compliance.** For every spec, run `scripts/check-prereqs.sh spec --slug <slug> --quiet --json`. Treat exit 1 as a lint warning (`spec missing upstream <missing>`).
   - **Mermaid validation.** For every page under `wiki/architecture/`, parse each fenced ` ```mermaid ` block. If `mmdc` is on PATH, run `mmdc --parseOnly` on each block. Skip with a warning when `mmdc` is not installed.
   - **`## Purpose` non-empty** on every architecture page (placeholder text like *"general system architecture"* fails the check).
   - **ADR ↔ architecture backlinks.** Every accepted ADR should be referenced from at least one architecture page's `## Decisions Surfaced` bullet with `→ ADR-NNNN`. Missing back-references are flagged.
   - **Spec ↔ ADR backlinks.** Every spec's `frontmatter.adr_ids` entry must resolve to an existing ADR file with `## Status: accepted`.
   - **Kanban ↔ spec backlinks.** When `hermes` is on PATH, any task in `running` or `blocked` whose `@wiki-spec` tag points at a spec lacking a `## Kanban Tasks` section listing that task id is flagged.
   - **Surfaced decisions without ADRs.** Bullets under `## Decisions Surfaced` lacking a `→ ADR-NNNN` tail on architecture pages whose `last_updated` is older than 7 days are flagged.
   - **Triage cross-link integrity.** Every `→ triage:<id>` annotation should resolve to a Hermes triage task; promoted entries become `→ spec:<slug>`.
4. R&D-pipeline schema checks (always run):
   - Every page under `wiki/specs/` cites at least one `accepted` ADR (warn, do not block — this matches the soft gate in the Spec workflow).
   - No two `accepted` ADRs contradict each other on the same decision. Heuristic: same decision title, or overlapping `## Decision` statements about the same artifact.
   - Every page under `wiki/contexts/` is referenced by at least one `wiki/context-maps/` page — orphan contexts cannot drift safely.
   - Every ADR in `wiki/decisions/` cites at least one [[concepts/architecturally-significant-requirement]] (ASR) in `## Context`. ADRs without an ASR are the AKM log-bloat failure mode.
   - Every ADR with status `superseded by NNNN` has a `## Related Decisions` link to the successor file, and the successor file exists.
   - Every ADR with status `deprecated` has no inbound references from *active* kanban tasks. (Skip this check with a warning if `hermes` is not on `PATH` — do not fail the lint.)
4. Fix what can be fixed automatically
5. Report issues that need human judgment
6. Suggest new sources or topics to investigate
7. Update log

Lint does **not** flag pages that lack a corresponding flashcard or presentation file — those workflows are explicit and on-demand.

### Flashcards

When the user says "flashcards [page]" — argument is a wiki page path such as `concepts/braising` or `entities/cast-iron-skillet`:

1. Read the source page in full
2. Derive cards from the page's content. For **concept pages**: pull from `## Definition`, `## How It Works`, `## Key Parameters`, `## When To Use`, and `## Risks & Pitfalls`. For **entity pages**: pull from `## Overview` and `## Characteristics`. For **summary pages**: pull from `## Key Points`.
3. Choose the right format for each card:
   - **Basic** (`Q` / `?` / `A`) — one-way recall (the default; use this for most cards)
   - **Reversed** (`Q` / `??` / `A`) — symmetric term ↔ definition pairs where the user should drill both directions
   - **Cloze** (`==hidden==`) — fill-in-the-blank inside a sentence; good for highlighting key terms in their natural context
4. Write `wiki/flashcards/<page-slug>.md` with frontmatter (`title`, `type: flashcards`, `tags: [flashcards, ...source-page-tags]`, `created`, `updated`, `sources: ["wiki/<path>"]`), a `## Source` section with a single wiki link to the source page, and a `## Cards` section containing the cards separated by blank lines
5. Add a cross-link back from the source page (a `## Flashcards` link line) so the source and cards stay discoverable from each other
6. Update `wiki/index.md` — add an entry under the Flashcards table
7. Append a dated entry to `wiki/log.md` recording the source page and cards file

The `flashcards` tag on the frontmatter is **required** — without it the obsidian-spaced-repetition plugin will not find the cards. Prefer rewriting an existing flashcards file over creating a new one if the source page is re-ingested.

### Presentation

When the user says "present [topic]" or "presentation [topic]" — argument is a topic or question, not a page path:

1. Read `wiki/index.md` to identify pages relevant to the topic (same retrieval as Query)
2. Read those pages in full
3. Generate a Marp slide deck with these slides, separated by `---` on its own line:
   - **Title slide** — `# <Topic>`, with a subtitle line summarizing the deck
   - **Outline slide** — `## Outline` followed by a bulleted list of sections
   - **Content slides** — one section per concept or entity, with the key claims pulled from the source pages. Keep each slide short (3–6 bullets or a single diagram/example)
   - **References slide** — `## References` followed by a wiki-link line for every page cited
4. Write `wiki/presentations/<topic-slug>.md` with Marp frontmatter (`marp: true`, `theme: default`, `paginate: true`) merged with the standard wiki frontmatter (`title`, `type: presentation`, `tags`, `created`, `updated`, `sources` listing every cited wiki page)
5. Update `wiki/index.md` — add an entry under the Presentations table
6. Append a dated entry to `wiki/log.md` recording the topic, deck path, and pages cited

If the wiki lacks enough material to build a coherent deck on the requested topic, say so and suggest sources that would fill the gap — do not invent claims. The Presentation workflow does **not** create a synthesis page; it only produces the deck.

### Spec

When the user says "spec [goal]" or runs `/wiki-spec <goal>` — argument is a user goal, feature request, or capability description in plain English.

The Spec workflow runs an automated specification-by-example workshop against the wiki's existing knowledge: it consumes domain concepts the same way Query does, but emits formal Gherkin scenarios + acceptance criteria + a ubiquitous-language glossary rather than prose. Output is a single `wiki/specs/<slug>.md` page; the Gherkin lives inside ` ```gherkin ` fenced blocks within that page (copy-paste into a target repo's test suite as needed).

1. Read `wiki/index.md` to find concept/entity pages relevant to the goal's domain.
2. Read those pages in full.
3. **ADR check (soft gate).** Scan `wiki/decisions/` for an `accepted` ADR governing this domain — heuristic: an ADR whose `## Context` cites the same bounded-context page or concept pages the goal touches. If none exists, warn the user: *"No accepted ADR found for this domain. Architectural commitments embedded inline in this spec will be hard to track across re-emissions. Consider drafting a [[concepts/y-statement]] ADR via `/wiki-adr` first."* Offer to hand control to `/wiki-adr`. If the user confirms they want to proceed without an ADR, continue; record the decision-to-skip in the spec page's `## Sources` section.
4. **Vagueness check.** If the goal lacks a clear actor + outcome, emit only the `## Scope` impact map (a table of `Actor | Impact | Deliverable` rows derived from the goal) and ask the user to confirm scope before continuing. If actor + outcome are already clear, skip the impact map.
5. Derive user stories from the (possibly scope-narrowed) goal. Each story is `As a <actor>, I want <capability>, so that <outcome>` paired with binary pass/fail acceptance criteria.
6. Translate each acceptance criterion into Gherkin scenarios under the Spec quality rules above (business-readable, single `When`, realistic data, `Scenario Outline` for parameterized cases). Use the exact `Given/When/Then` template — no UI mechanics, no DB tables, no CSS selectors.
7. Extract a `## Glossary` fragment recording every ubiquitous-language term that appears in the scenarios. Link terms that already exist as `concepts/<term>`; flag any new term that warrants its own concept page.
8. Write `wiki/specs/<slug>.md` with full frontmatter (`type: spec`, tags including `spec` and `gherkin`, plus domain tags) and the required sections (`## Goal`, optional `## Scope`, `## User Stories`, `## Scenarios`, `## Glossary`, `## Sources`).
9. **Verify zero dangling links — acceptance gate.** Same rule as Ingest: scan every `[[...]]` reference on the new spec page, confirm each target file exists, and for each dangling link either create a stub concept page (full schema, `confidence: low`, brief content grounded in how the spec uses the term) or remove the link. Re-scan until zero dangling links remain. The Spec workflow grows the concept graph the same way Ingest does.
10. Update `wiki/index.md` — add a row under the `## Specs` table, and bump the `Specs` count under Statistics.
11. Append a dated entry to `wiki/log.md` recording the goal, slug, story count, scenario count, and pages cited.

The Spec workflow does **not** create a paired synthesis page — specs are the artifact. If the wiki lacks enough material to ground the scenarios in real domain concepts, say so and suggest sources that would fill the gap — do not invent business rules.

### PDF → mdBook

When the user says "pdfbook [path]" or runs `/pdf-to-mdbook <path>` — argument is a path to a PDF, typically under `raw/` but any readable path is allowed.

The goal is to convert a single PDF (scanned, structured, textbook, or research paper) into a faithful, runnable mdBook under `wiki/books/<slug>/`. The chapter/section hierarchy must match the real source. **Do not invent chapters or content the PDF does not contain.**

**Scope: conversion only.** This workflow does NOT ingest the book's content into the wiki graph. It does not write `wiki/summaries/<slug>.md`, does not create concept or entity pages, and does not produce `Relevant Concepts` cross-links. Its only wiki-graph touchpoints are a row in the `## Books` table of `wiki/index.md` and a `wiki/log.md` entry — both of which merely record that the book exists. If the user wants the book ingested into the knowledge graph (concepts extracted, entities cross-linked, syntheses drawn), they will invoke the Ingest workflow separately afterwards on the source PDF or the rendered book. Treat conversion and ingestion as independent — running PDF → mdBook never implies running Ingest.

**Required tooling on PATH** (fail fast with `brew install poppler ocrmypdf tesseract mdbook` if any are missing):

- `pdfinfo`, `pdftotext`, `pdftoppm`, `pdfimages` (poppler)
- `ocrmypdf` and `tesseract`
- `mdbook`
- `python3` with `pypdf` (used only to read the embedded outline; fall back to vision if absent)

Optional, used if present: `marker` or `docling` for higher-fidelity extraction on textbooks and papers. Never add these as hard dependencies.

Steps:

1. **Resolve inputs.** Compute `slug` from the PDF filename (lowercase, hyphenated, no extension). Choose `wiki/books/<slug>/` as the output dir. If it already exists, treat this as a re-run and overwrite chapter files rather than duplicating.

2. **Classify the PDF.** Sample text from the first ~5 pages with `pdftotext -layout -f 1 -l 5 <pdf> -` and count alphabetic characters. If the sample is sparse (≲ 100 alpha chars per page on average), treat the PDF as scanned.

3. **OCR when needed (automatic, no confirmation).** For scanned PDFs, run `ocrmypdf --skip-text --output-type pdf --rotate-pages --deskew <input.pdf> <work>/ocr.pdf`. This is expected to be slow on 500+ page books — let it run. Use the OCRed PDF as the working source for all subsequent steps.

4. **Extract reference text.** Run `pdftotext -layout <work>/source.pdf <work>/text-layout.txt` and `pdftotext <work>/source.pdf <work>/text-flow.txt`. Keep both as *reference material* for the vision pass — they help disambiguate hard-to-read characters, technical identifiers, citations, and long URLs. They are **not** the primary source of chapter content. `pdftotext` consistently fails on two-column research papers (it interleaves columns), on scanned PDFs after OCR (artefact noise), on figure-heavy textbook pages (vector-figure labels bleed into prose), and on math-heavy content (operators and superscripts are lost). Treat the extracted text as advisory only.

5. **Extract images.** Run `pdfimages -all <work>/source.pdf <work>/img/fig` and copy the result into `wiki/books/<slug>/src/images/`. Keep a manifest mapping image file → originating page so figure references in chapters can point to the right file.

6. **Recover structure — this is the critical step.** Spend tokens generously here.
   1. Try the embedded outline first:
      ```bash
      python3 -c "import pypdf,sys,json; r=pypdf.PdfReader(sys.argv[1]); \
      def w(it,d=0,o=[]):
       for x in it:
        if isinstance(x,list): w(x,d+1,o)
        else: o.append({'depth':d,'title':getattr(x,'title','')})
       return o\nprint(json.dumps(w(r.outline)))" <pdf>
      ```
      (Run it as a heredoc or short script — the example is schematic.) Reject the outline if it is empty, if titles look like filenames (e.g. `00.pdf`, `chapter01.pdf`), or if the depth-1 entries clearly do not correspond to chapters.
   2. **Vision-based structure extraction** when the outline is missing or unreliable. Render representative pages with `pdftoppm -png -r 150 <work>/source.pdf <work>/pages/p` and read those PNGs as images. Cover at minimum: the table of contents pages (when present), the first page of every plausible chapter (detect by skimming page thumbnails at lower DPI like `-r 75` first to find chapter-opening pages), and any pages whose text extraction looks scrambled. The expectation is that the vision pass produces a structured list of `{depth, title, start_page, end_page}` entries grounded in what is actually on the page images — not invented.
   3. Cross-check: every chapter's `start_page` must contain text matching the proposed title; if it does not, re-examine the image and correct the entry before continuing.
   4. **Image-limit safeguard.** Most vision APIs cap images per conversation at ~30. For books longer than ~25 pages you cannot read every page as an image in one turn — and you also cannot read every chapter-opening + figure page in one turn for a typical textbook. For long books, switch to the **`pi-subagents` worker dispatch** reconstruction strategy in step 7b: the parent caps itself at ~12 images and each chapter is reconstructed by a fresh `worker` child (`subagent` tool, `context: "fresh"`). Do not try to fit a long book into the parent conversation.

7. **Assemble chapter files.** Choose the reconstruction strategy based on page count and source quality.

   **Strategy A — Pure vision-per-page (recommended for ≤ ~25 pages).**
   Render every page as an image and have the agent read each rendered page to produce the chapter's markdown. `pdftotext` output is consulted only as a cross-check for tricky identifiers, citations, and long URLs.
   - Render the chapter's page range at 150 DPI (200 DPI for math-heavy or figure-heavy pages):
     ```bash
     pdftoppm -png -r 150 -f <start> -l <end> <work>/source.pdf <work>/ch<NN>/p
     ```
   - Read the rendered PNGs in order (in batches of 3–5 pages per read when the chapter is long). For each page, produce clean markdown that reflects what is actually on the page — nothing more, nothing less:
     - Honour the visual reading order (top-to-bottom within each column for two-column layouts; reconstruct the prose column-by-column, not interleaved).
     - Drop running headers, running footers, page numbers, and copyright/permissions notices.
     - Drop in-figure labels and arrows; do not splice them into surrounding prose.
     - Preserve math as inline TeX (`$...$`) or display TeX (`$$...$$`). Do not approximate equations as ASCII; if a symbol is genuinely unreadable, mark it `$\text{?}$` and note it in the log entry instead of guessing.
     - Preserve code listings as fenced blocks. Use a language hint when the language is clear; fall back to plain fences otherwise.
     - Preserve tables as GitHub-flavoured markdown when feasible; fall back to fenced plain-text for tables that do not survive the conversion.
     - Preserve citation markers in their original form.
     - Stitch sentences across page boundaries: undo end-of-line hyphenation, merge paragraphs that continue from one page to the next, and do not introduce a paragraph break where the source had none.
   - Figures: for raster figures already extracted (step 5), reference them at the position they appeared. For vector figures, **crop just the figure region** with `pdftoppm -x -y -W -H` (pixel coords at the rendering DPI; a US Letter page at 150 DPI is 1275×1650). Determine the bounding box by reading the rendered page image and estimating the four bounds (figure + caption, excluding headers, body prose, adjacent figures). Verify the crop visually; tighten if body text bleeds in, widen if the caption or right edge is clipped. Rename the output to a stable filename (`pdftoppm` appends a page-number suffix like `-08.png`).
   - Cross-check identifiers, long URLs, DOIs, ISBNs, and citation keys against the corresponding region in `text-layout.txt`/`text-flow.txt`; vision OCR can miss a digit or a hyphen in long alphanumeric strings.
   - Start each chapter file with `# <Chapter Title>` matching `SUMMARY.md` exactly. Sub-sections become `##`/`###` honouring the printed hierarchy.

   **Strategy B — Subagent-dispatched hybrid (required for > ~25 pages because of the ~30 image-per-conversation limit).**
   Pure vision-per-page in a single conversation is impossible for a long book — and "render every chapter opening + every figure page" also blows past the cap on most textbooks. Split the work: the parent does structure + assembly, and one subagent per chapter does prose reconstruction with a fresh image budget. **Hard rule: the parent must never render or read more than ~12 page images for the whole job.** If you approach that cap, stop and dispatch the rest.

   1. **Structure pass in the parent (≤12 images, hard cap):**
      - Try the pypdf outline first (step 6.1).
      - If the outline is unusable, render only TOC pages plus 2–3 sample chapter-opening pages for cross-check. Read them, then stop rendering in the parent.
      - Commit the recovered structure to `<work>/chapters.json` as a list of `{index, title, start_page, end_page}` entries. Both `SUMMARY.md` and the subagent dispatch in step 3 come from this file.
   2. **Bulk prose extraction (no vision):**
      - Run `pdftotext -layout <work>/source.pdf <work>/text-layout.txt` (or flow mode for two-column papers).
      - Use header/footer patterns observed in the structure pass to strip running elements (e.g. `grep -v` for repeated header lines, drop trailing page-number-only lines).
      - Slice the cleaned text by chapter range into `<work>/text/ch-<NN>.txt` using the page map.
   3. **Per-chapter `worker` dispatch via the `subagent` tool (`pi-subagents`).** This is the actual workaround for the image limit — each chapter is reconstructed by a fresh `worker` child whose conversation starts empty and gets its own ~30-image budget. For each chapter in `chapters.json`, call the `subagent` tool with the builtin `worker` agent. **You MUST pass `context: "fresh"` explicitly:** `worker` defaults to `context: "fork"` (forked from the parent session), which would inherit the parent's images and defeat the entire point. Dispatch independent chapters in a single `tasks: [...]` call so they run in parallel (pi-subagents default `concurrency: 4`, `maxTasks: 8` per call — for books with > 8 chapters, send multiple `subagent` calls of ≤8 tasks each, or raise `parallel.maxTasks` in `~/.pi/agent/extensions/subagent/config.json`).

      Call shape:
      ```ts
      subagent({
        tasks: [
          { agent: "worker", task: "<chapter 1 self-contained prompt>" },
          { agent: "worker", task: "<chapter 2 self-contained prompt>" },
          ...
        ],
        context: "fresh"
      })
      ```

      Each `task` string must be a self-contained prompt (the worker starts fresh and inherits `AGENTS.md`/`CLAUDE.md`, but no other parent context). Include:
      - PDF path, slug, chapter index, title, `start_page`, `end_page`.
      - Path to the chapter's `pdftotext` slice (`<work>/text/ch-<NN>.txt`).
      - Path to the `pdfimages` output dir and the figure→page manifest.
      - Target chapter-file path (`wiki/books/<slug>/src/<NN>-<chapter-slug>.md`) and the exact required first line (`# <Chapter Title>` matching `SUMMARY.md`).
      - **Hard rule for the worker:** render and read at most ~20 page images. Spend that budget on (a) the chapter opening, (b) any page identified as figure/table/equation/math-heavy from the slice, and (c) any page where the slice is garbled. Do not render every page of the chapter.
      - Stitching: start the chapter file with the vision-reconstructed opening page, append the bulk slice prose, swap in vision-reconstructed markdown for figure/table/math-heavy pages, cross-check math/code/tables/long identifiers between vision and bulk text, correct OCR artefacts (`Ð`→em-dash, `ł`→left-quote, etc.) using patterns observed in the vision reads.
      - The worker writes the chapter file directly with `Write`/`Edit` and returns a short status message (pages rendered, figures inserted, unresolved issues). It must not write `SUMMARY.md`, `book.toml`, or touch other chapters.
   4. **Re-dispatch on worker failure.** If a worker reports it hit its own image cap or could not complete the chapter, re-dispatch that one chapter with a narrower page range (split the chapter in half into two tasks) rather than retrying in the parent.
   5. **Figures:** follow the same vector-figure cropping rules as Strategy A. Under Strategy B, cropping happens inside the subagent that owns the figure's chapter.

8. **Write `book.toml`** at `wiki/books/<slug>/book.toml`:
   ```toml
   [book]
   title = "<PDF Title>"
   authors = ["<from PDF metadata when available>"]
   language = "en"
   src = "src"

   [output.html]
   mathjax-support = true
   ```

9. **Write `src/SUMMARY.md`** in strict mdBook syntax. Example:
   ```markdown
   # Summary

   [Introduction](00-introduction.md)

   # Part I — Fundamentals

   - [Chapter 1: Title](01-chapter-one.md)
     - [1.1 Subsection](01-chapter-one.md#section-anchor)
   - [Chapter 2: Title](02-chapter-two.md)
   ```
   Use two-space indents for sub-chapters. Use `[Title]()` (empty link) only for unwritten draft chapters; the workflow should not normally emit drafts.

10. **Validate the build.** Run `mdbook build wiki/books/<slug>/`. If it fails, fix `SUMMARY.md` or chapter files until it succeeds. A successful build is part of the acceptance bar.

11. **Update `wiki/index.md`.** Add an entry under the `## Books` table (create the table if missing). Columns: `Page | Title | Pages | OCR | Created`. The `Page` cell links directly to the rendered book — `[[books/<slug>/src/SUMMARY|<Title>]]`. Do **not** link to a `summaries/<slug>` page; this workflow does not create one.

12. **Append `wiki/log.md`** with a dated entry recording the PDF path, slug, total chapters, page count, OCR status, structure-recovery method, and `mdbook build` result.

Do **not** create `wiki/summaries/<slug>.md`. Do **not** create concept or entity pages. Do **not** produce `## Relevant Concepts` links from the book's content. Those are the Ingest workflow's responsibilities; the user invokes that separately if desired.

### Strategy

When the user says "strategy [topic]" or runs `/wiki-strategy <topic>` — argument is the topic of the R&D effort (a domain, a feature area, a product theme).

The Strategy workflow runs the [[concepts/domain-driven-design]] strategic-design phase against the wiki's existing knowledge: it distills the [[concepts/core-domain]], names the [[concepts/bounded-context]]s that will participate, and draws the initial [[concepts/context-map]] so downstream specs and ADRs have a model to anchor in. Output is one page in `wiki/vision/`, one or more pages in `wiki/contexts/`, and one page in `wiki/context-maps/`.

Strategy is **re-runnable.** Later runs *update* the existing pages rather than duplicating, the same way Ingest updates concept pages. Use the topic slug as the stable identifier.

1. Read `wiki/index.md` to find concept/entity/summary pages relevant to the topic. Read those pages in full.
2. **Distillation pass.** Identify which concepts are core (load-bearing for the R&D effort) vs. supporting (necessary but not differentiating). Pull from [[concepts/distillation]] and [[concepts/strategic-design]] if they exist in the wiki.
3. **Vision.** Write `wiki/vision/<topic-slug>.md` with `type: vision` frontmatter and the required sections (`## Value Proposition`, `## In Scope`, `## Out of Scope`, `## Differentiation`, `## Revisions`). The Value Proposition must be one paragraph stating what is differentiating; In/Out of Scope must be bulleted lists.
4. **Bounded contexts.** Identify each model that will participate in the R&D effort — e.g. for a wiki-to-kanban pipeline: "wiki content context", "spec emission context", "kanban emission context", "profile host context". For each, write `wiki/contexts/<context-slug>.md` with `type: context` frontmatter and the required sections (`## Model`, `## Boundary`, `## Ubiquitous Language` inline glossary, `## Relationships`).
5. **Context map.** Write `wiki/context-maps/<topic-slug>.md` with `type: context-map` frontmatter and the required sections (`## Contexts`, `## Translations` table, `## False Cognates`, `## Integration Patterns`). The translations table is load-bearing — it is what prevents the same word from drifting between contexts.
6. Cross-link: every bounded-context page references the context map under `## Relationships`; the context map references every bounded-context page under `## Contexts`; the vision page references both.
7. **Verify zero dangling links — acceptance gate.** Same rule as Ingest: scan every `[[...]]` reference and confirm each target file exists. Create stub concept pages (`confidence: low`) or remove the link until zero remain.
8. Update `wiki/index.md` — add rows under the Vision / Contexts / Context Maps tables, and bump the corresponding Statistics counts.
9. Append a dated entry to `wiki/log.md` recording the topic, slug, contexts identified, false cognates flagged.

If the wiki lacks enough material to identify distinct bounded contexts (e.g. only two or three concept pages on the topic), say so and suggest sources or further `/wiki-ingest` runs that would fill the gap — do not invent contexts. A single bounded context is fine and common; in that case the context map is a one-row map of the context against the outside world.

### Grill

When the user says "grill [topic]" or runs `/wiki-grill <topic>` — argument is the topic slug (the same slug used by Strategy).

The Grill workflow surfaces and resolves open design questions **before** specs are written. It is modeled on Matt Pocock's `grill-me` skill: identify a decision tree, then interrogate depth-first, asking **one question per turn** with concrete options. Park what cannot be answered; record what is decided. The output is the wiki's record of why later ADRs and specs ended up where they did.

**Preflight gate.** Run `scripts/check-prereqs.sh grill --slug <topic-slug>`. On exit 1 (e.g. `missing: strategy — run /wiki-strategy first`), abort without writing anything. Exit 0 → proceed. Manifest absent → exit 0 (back-compat).

1. **Context load.** Read `wiki/vision/<topic-slug>.md` in full, then every `wiki/contexts/*.md` it cites, then every `wiki/concepts/*.md` reachable via `[[wikilinks]]` from there.
2. **Resume check.** If `wiki/grills/<topic-slug>.md` exists, treat its `## Decision Tree`, `## Q&A Log`, `## Decisions Made`, and `## Open Questions` as authoritative. Resume at the next unanswered question.
3. **Decision-tree elicitation (first run only).** Identify 3–7 top-level decisions grounded in the vision/contexts, print them numbered under a candidate `## Decision Tree`, and ask the user **which to grill first**. Do not proceed until the user picks.
4. **Depth-first interrogation.** Ask **exactly one question per turn**. Each question has 2–4 concrete options numbered, plus an `other → free text` escape. Branch into follow-ups until the subtree is exhausted; then return to the next top-level decision the user selects. **One question per turn is a hard rule.**
5. **Park open questions.** *"I don't know yet"*, *"park this"*, *"skip"* → copy the question verbatim into `## Open Questions` and move on. Never blocks; never re-asked in the same run.
6. **Write `wiki/grills/<topic-slug>.md`** with `type: grill` frontmatter and the required sections: `## Decision Tree`, `## Q&A Log` (append-only, global `Q<n>` numbering), `## Decisions Made` (one bullet per decision, keyed by short title — same title rewrites, new titles append), `## Open Questions` (append-only across runs), `## Cross-Links` (backlinks to vision/contexts, forward placeholders to architecture/ADR/spec).
7. **Cross-link back.** Append (or update in place) a `## Grill Notes` section on `wiki/vision/<topic-slug>.md` linking the grill.
8. **Abandonment handling.** On mid-session exit, set `## Status: in progress (resume with /wiki-grill <topic>)`. On clean exit, set `## Status: done` or omit.
9. **Zero-dangling-links acceptance gate.** Forward placeholders to not-yet-written architecture / ADR / spec pages get stubs (`confidence: low`) or are removed.
10. Update `wiki/index.md` (`## Grills` table — create if absent) and append to `wiki/log.md`.

Downstream: `/wiki-architecture` reads the grill to know which questions the diagrams should answer; `/wiki-adr` typically cites a specific `## Decisions Made` bullet as rationale. If the vision page is too thin to derive 3 top-level decisions, say so and suggest `/wiki-strategy` — do not invent decisions.

### Architecture

When the user says "architecture [topic]" or runs `/wiki-architecture <topic>` — argument is the topic slug (the same slug used by Strategy and Grill).

The Architecture workflow draws [C4](https://c4model.com/) diagrams (in Mermaid) for the topic and surfaces the architectural decisions the diagrams imply. Output is a single page in `wiki/architecture/` plus a `## Architecture` cross-link on every referenced bounded-context page.

**Preflight gate.** Run `scripts/check-prereqs.sh architecture --slug <topic-slug>`. On exit 1, abort without writing. `grill` is required transitively but is `optional_in_v1: true` — a missing grill page warns rather than blocks; a missing vision page hard-fails.

1. **Context load.** Read the vision page in full; read the grill page if it exists (its `## Decisions Made` and `## Open Questions` shape the questions the diagrams must answer); read cited bounded-context and concept pages.
2. **Purpose gate (hard).** Ask the user *"What question does this diagram set answer?"* — refuse to draw anything until the answer is a single, non-empty sentence. The sentence becomes the page's `## Purpose` verbatim. Diagrams without a stated question are decoration.
3. **Level selection.** Ask which C4 levels are needed: Context, Container, Component, Dynamic, Deployment. Default to Context + Container on *"minimum"*. **Refuse to draw all five without a stated question for each level.**
4. **Format — Mermaid C4.** All diagrams as fenced ` ```mermaid ` blocks using `C4Context` / `C4Container` / `C4Component` / `C4Deployment` syntax; use `sequenceDiagram` or `flowchart` for the Dynamic level. ASCII fallback only on explicit opt-out.
5. **Write `wiki/architecture/<topic-slug>.md`** with `type: architecture` frontmatter and the required sections (in order): `## Purpose`, `## System Context` / `## Container Diagram` / `## Component Diagram` / `## Dynamic Diagram` / `## Deployment Diagram` (only the chosen levels), `## Assumptions`, `## Open Questions`, `## Decisions Surfaced`, `## Cross-Links`.
6. **`## Decisions Surfaced` is load-bearing.** Each bullet: `**<short decision title>** — <one-line summary>`. `/wiki-adr` consumes this list: every accepted ADR must cite a bullet here. After `/wiki-adr` runs, the corresponding bullet is upserted with `→ ADR-NNNN`.
7. **Approval gate.** Print *"Architecture pending approval. Run `/wiki-adr <decision title>` for each surfaced decision."* **Do not invoke `/wiki-adr` automatically** — the user picks which decisions warrant a written record.
8. **Cross-link back.** Append (or update in place) a `## Architecture` section on every referenced `wiki/contexts/<context>.md` linking the architecture page.
9. **Idempotency.** Mermaid blocks are keyed by parent heading and rewritten in place on re-run. `## Decisions Surfaced` bullets are keyed by short title (same title rewrites, new titles append). `## Assumptions` and `## Open Questions` are append-only.
10. **Zero-dangling-links acceptance gate.** Forward placeholders to ADR / spec pages that don't exist yet get stubs or are removed.
11. Update `wiki/index.md` (`## Architecture` table — create if absent) and append to `wiki/log.md`.

After a clean run, `/wiki-adr <surfaced decision title>` is the next step.

### ADR

When the user says "adr [decision title]" or runs `/wiki-adr <decision title>` — argument is a short title for the architectural decision (e.g. "Use the wiki page slug as the cross-system identifier").

The ADR workflow opens a new [[concepts/architectural-decision-record]] in `wiki/decisions/`. ADRs pin architectural commitments before they are encoded in code, carry their rationale forward, and survive [[concepts/breakthrough]]s via the [[concepts/decision-log]] discipline of write-once, supersede-don't-edit.

1. **Number the ADR.** Scan `wiki/decisions/NNNN-*.md` and pick the next four-digit number. Numbers are monotonic across the wiki and never reused, even for deleted drafts.
2. **Identify the triggering [[concepts/architecturally-significant-requirement]].** Ask the user *what requirement made this decision necessary?* — the structural or quality-attribute requirement whose effect on the system makes this load-bearing. Refuse to proceed if the user cannot name one; an ADR without an ASR is the AKM log-bloat failure mode. The ASR goes in the `## Context` section of the ADR.
3. **Choose a template:**
   - **[[concepts/nygard-adr]]** (default) — five sections (Status, Context, Decision, Consequences, Related Decisions). Use this unless the user asks otherwise.
   - **[[concepts/madr]]** — when the user's input mentions multiple realistic alternatives whose trade-off analysis deserves preservation. Adds `## Decision Drivers` and `## Considered Options` sections.
   - **[[concepts/y-statement]]** — when the user asks for a one-liner. Format: *"In the context of `<X>`, facing `<Y>`, we decided for `<Z>` to achieve `<Q>`, accepting `<W>`."* Y-Statements can stand alone or sit at the top of a Nygard ADR as an executive summary.
4. **Write `wiki/decisions/NNNN-<kebab-title>.md`** with `type: decision` frontmatter, `## Status: proposed`, the ASR citation in `## Context`, and the rest of the required sections for the chosen template. Tags must include `decision`.
5. **Cross-link.** The ADR's `## Context` and `## Consequences` should link every wiki page the decision governs (concepts, bounded contexts, prior ADRs). Update those pages' `## Related Concepts` / `## Related Decisions` sections to link back.
6. **Refuse to edit an `accepted` ADR.** If the user is trying to change a decision that has already been accepted, instead open a new ADR that *supersedes* the old one: open `NNNN+1-<kebab-title>.md`, set its `## Status: proposed`, cite the predecessor in `## Related Decisions`, and (separately, with the user's permission) edit the predecessor's `## Status` to `superseded by NNNN+1` and add a link to the successor. The predecessor's body stays intact.
7. **Verify zero dangling links — acceptance gate.** Same rule as Ingest.
8. Update `wiki/index.md` Decisions table and bump the Decisions Statistics count.
9. Append a dated entry to `wiki/log.md` recording the ADR number, title, status, triggering ASR, and pages cross-linked.

The status field is load-bearing for downstream workflows: `/wiki-spec` warns when no `accepted` ADR governs the domain, and `/wiki-kanban-emit` picks the [[concepts/collaboration-pattern]] based on ADR status (`proposed` → P5 human-in-the-loop; `accepted` → P2 pipeline; `deprecated`/`superseded` without successor → refuses to emit).

### Kanban Emit

When the user says "kanban-emit [spec-slug]" or runs `/wiki-kanban-emit <spec-slug>` — argument is the slug of a `wiki/specs/<slug>.md` page that already exists.

The Kanban Emit workflow decomposes a spec page into one or more [[concepts/task-specification]] rows on a [[concepts/durable-task-board]] hosted by [[entities/hermes-agent]]. The wiki is the source of truth; the board is the execution substrate. **Discipline: this workflow creates rows and steps back — it does not claim, run, or shell out to do worker tasks.** This is the [[concepts/orchestrator-pattern]] / [[concepts/three-plane-architecture]] discipline; the pi-llm-wiki extension is the *orchestrator*, never a *worker*.

**Preflight gate (manifest).** Run `scripts/check-prereqs.sh kanban_emit --slug <spec-slug>` from the wiki root. The pipeline manifest requires `spec` (produced) and the `git:spec-on-trunk` check (latest commit touching `wiki/specs/<slug>.md` is an ancestor of `origin/<trunk>`). Exit 1 → print the `missing` + `hint` and abort. Exit 3 → spec is on a feature branch not yet merged; print the rendered `git merge-base` command and refuse to emit — kanban identity hashes the spec body, and emitting while the spec moves produces a moving idempotency key. Exit 0 → proceed. Manifest absent → back-compat, helper exits 0.

**Preflight gate (soft Hermes dependency).** Invoke `hermes kanban assignees` (or the host's equivalent profile-list command). If `hermes` is not on `PATH`, or the command fails, abort with:

> *"Hermes is not installed or no profiles are configured. See https://github.com/NousResearch/hermes for installation. Kanban emission requires Hermes; the rest of this wiki — ingest, query, spec, ADR, strategy — works without it."*

Do not attempt to proceed without Hermes.

**ADR gate (soft).** Read the spec page; if it cites no `accepted` ADR governing its domain, warn the user and offer to hand control to `/wiki-adr` to draft a [[concepts/y-statement]] ADR first. Proceed on confirmation; record the choice to proceed-without-ADR in the spec page's `## Sources` section.

1. **Read the spec.** Open `wiki/specs/<spec-slug>.md` and read it in full, including frontmatter, every user story, every Gherkin scenario, every glossary entry, and every cited source.
2. **Read governing context.** For every ADR id cited on the spec page, open `wiki/decisions/<adr-id>-*.md` and confirm its `## Status` is `accepted`. For the relevant `wiki/contexts/<context>.md` page, read the inline `## Ubiquitous Language` glossary — this is what gets inlined into each task body (per the [[concepts/false-cognate]] guard at the agent-fleet scale).
3. **Compute the idempotency keys.** Per [[concepts/idempotency-key]] and the [[syntheses/rd-pipeline-ddd-adr-kanban]] identity rule, the **parent** key is the triple `<spec-slug>:<adr-id>:<sha256(spec-body)>` over the spec body excluding frontmatter and the auto-generated `## Kanban Tasks` section. Each **per-scenario child** uses the extended key `<spec-slug>:<adr-id>:<scenario-slug>:<sha256(scenario-body)>`. Re-emission semantics: same triple → update existing task; edited scenario → new sha256 → updates only that child; new ADR id (supersession) → new parent + new children; new slug (context split) → entirely new rows.
4. **Enumerate Hermes profiles.** The output of `hermes kanban assignees` is the *exhaustive* list of legal `assignee` values; unknown assignee names silently fail to spawn (per [[entities/kanban-orchestrator-skill]]). Map logical roles to real profile names — `implementer`, `reviewer`, `integrator` — using a wiki-side mapping if one exists (under `wiki/contexts/` or a dedicated config page), or by asking the user when no mapping is recorded. Fail loudly on missing mappings; do not pick a "default" profile.
5. **Pick the [[concepts/collaboration-pattern]] from ADR status:**
   - `accepted` → **P2 pipeline** (`implementer → reviewer → integrator`) by default.
   - `proposed` → **P5 human-in-the-loop**: implementer + reviewer + a final human-gate task before integration.
   - Multiple independent feature files → wrap the pipeline in a **P1 fan-out** parent.
   - When reviewer agreement matters more than throughput → **P3 voting/quorum** (two reviewers + aggregator).
   - `deprecated` or `superseded` without a successor in `accepted` state → **refuse to emit**; tell the user the governing ADR is no longer current.
6. **Decompose into parent + per-scenario children + aggregator.** Emit:
   - One **parent task** per spec (key from step 3) — its body is the full spec context.
   - One **child task per Gherkin scenario** with `parents=[parent_id]` — its body carries the scenario-specific goal, acceptance criteria, the fenced ` ```gherkin ` block, the inlined glossary excerpt, the wiki backlink, the ADR ids, the `@wiki-spec` tag, and the [[concepts/structured-handoff]] schema inlined verbatim under `## Required Handoff`.
   - One **aggregator task** with `parents=[<every child id>]` whose job is to synthesize the `## Implementation Evidence` block the wiki ingests via `/wiki-kanban-ingest`.

   Children run in parallel under P1 fan-out semantics. Re-emission keys are per-scenario, so editing one scenario only re-emits that one child.

   Each task body must include: `## Goal` (verbatim from the spec, scoped to this story or scenario); approach (the story narrative, when the spec has an opinion; otherwise leave open per [[concepts/task-specification]]); acceptance criteria verbatim (preserves the [[concepts/executable-specification]] contract end-to-end); the Gherkin scenario block fenced inside ` ```gherkin `; the relevant glossary excerpt inlined from the bounded-context page (not just a link — inline, to prevent [[concepts/false-cognate]] drift across workers); a wiki backlink to `wiki/specs/<spec-slug>.md`; the ADR id(s) cited by the spec; a `@wiki-spec` traceability tag in the task title or metadata; and the `## Required Handoff` schema inlined from `prompts/_handoff-schema.md`.

7. **Per-task skill pinning + tenant.** Every `kanban_create` call MUST pass `--skill wiki-maintainer` and `--skill kanban-worker`, and `--tenant <bounded-context-slug>` (derived from the spec's `frontmatter.context`; fall back to `default` only when the spec has no context tag). The skill pinning closes the loop with this wiki's workflows; the tenant separates boards across bounded contexts so e.g. `hermes kanban list --tenant payments` returns only payments-context work.
8. **Express ordering.** Use `parents=[...]` on each `kanban_create` so the dispatcher gates `todo → ready` promotion. The reviewer task's `parents` is the implementer; the integrator's `parents` is the reviewer; the human-gate's `parents` is whatever it gates; per-scenario children list the parent task; the aggregator lists every child.
9. **Workspace selection.** For tasks that mutate the wiki itself, use `workspace=dir:<absolute path to wiki repo>`. For tasks that touch a code repository, prefer `worktree`. Reject relative paths at dispatch — the confused-deputy guard in [[concepts/task-specification]].
10. **Record task ids on the spec page.** Append a `## Kanban Tasks` section to `wiki/specs/<spec-slug>.md` listing: parent key, every child key, aggregator key, ADR ids, collaboration pattern, profile mapping, tenant, skills, and each task's id + role + assignee.
11. **Verify zero dangling links — acceptance gate.** Scan the updated spec page for any new `[[...]]` references and resolve per the standard rule.
12. Append a dated entry to `wiki/log.md` recording the spec slug, parent + child + aggregator idempotency keys, ADR ids, pattern, task ids, tenant, and assignees. Do **not** poll the board for completion — fire and forget (per the orchestrator-pattern rule).

If the spec is updated mid-execution, prefer pushing the diff onto the task thread via `kanban_comment`; do not silently edit a `running` task's body (per [[entities/kanban-orchestrator-skill]]).

### Kanban Ingest

When the user says "kanban-ingest [task-id|run-id]" or runs `/wiki-kanban-ingest <id>` — argument is a Hermes task id or run id whose status is `done` or `failed`.

The Kanban Ingest workflow round-trips a completed Hermes run back into the originating wiki page as a [[concepts/living-documentation]] receipt. This is the *documentary* half of the refinement loop; the *structural* half (new ADRs, re-emission) is the Refine workflow.

**Preflight gate (Hermes).** Same Hermes check as Kanban Emit. Abort with the same message if `hermes` is unavailable.

**Preflight gate (manifest + git discipline).** After fetching the task's run metadata, pull `metadata.branch_head` from the [[concepts/structured-handoff]] and run `scripts/check-prereqs.sh kanban_ingest --branch-head <branch_head>`. The manifest's `git:worker-branch-merged` check runs `git fetch --quiet origin <trunk> && git merge-base --is-ancestor <branch_head> origin/<trunk>`. Exit 3 → the worker's branch is not yet on trunk; abort with the rendered command and the message *"Run <id>'s branch <branch_head> is not on <trunk>; merge before ingesting."* Do not write any `## Implementation Evidence` section while the branch is unmerged — the wiki would be recording evidence pointing at code that isn't yet trunk. Manifest absent → back-compat, helper exits 0.

1. **Fetch attempt history.** Use `hermes kanban runs <id> --json` to retrieve **every** run for the task (the durable `task_runs` table), oldest-first. Capture per run: `summary`, `verification`, `changed_files`, `residual_risk`, `metadata.branch_head`, and the [[concepts/structured-handoff]] payload.
2. **Validate the handoff schema.** Every run's `metadata` MUST contain the keys declared in `prompts/_handoff-schema.md`: `changed_files`, `verification`, `dependencies`, `blocked_reason`, `retry_notes`, `residual_risk`, `branch_head`, `wiki_spec`, `wiki_adr_ids`. Missing keys → abort with `missing keys: [...]`. Do not partially ingest a run with a broken handoff.
3. **Sanitize metadata.** [[concepts/structured-handoff]] explicitly forbids tokens, OAuth material, raw logs, and unrelated transcripts in the data that crosses the boundary. Lint the fetched metadata: copy summaries and pointers (file paths, commit hashes, run ids, URLs), refuse to copy secrets or raw logs even if they appear. If the metadata is dirty, surface the issue to the user and stop — do not silently scrub-and-copy without acknowledgment.
4. **Locate the originating wiki page.** Read the task body for the `@wiki-spec` traceability tag or the wiki backlink. The target is normally `wiki/specs/<spec-slug>.md`; for refinement-driven re-runs it may be a concept page.
5. **Append `## Implementation Evidence`** to the target page. For multi-attempt tasks, emit one `### Attempt N: <outcome>` subsection per non-final run (with `retry_notes` and `blocked_reason` quoted from the handoff), then the final-run block. Required content for the final block: run id, completion timestamp, the assignee profile, the `verification` command(s) and their result, `changed_files` (linked to the repo when possible), `residual_risk`, `branch_head`, and a one-paragraph summary derived from the structured-handoff. If the section already exists from a prior re-ingest, append a new dated subheading rather than overwriting.
6. **Cross-link.** Add the new section to the page's frontmatter `updated` field and update any `## Sources` row that points at the spec or concept.
7. **Verify zero dangling links — acceptance gate.** Same rule as Ingest.
8. Update `wiki/index.md` if any new pages were referenced (rare — usually only the target's `updated` date moves). Append a dated entry to `wiki/log.md` recording the run id, target page, verification outcome, attempt count, and any residual risk.

If the run failed, still ingest — `## Implementation Evidence` documents failures honestly so the next attempt has context. Mark the section with `Result: failed` and quote the failure reason from the handoff.

### Triage

When the user says "triage [note]" or runs `/wiki-triage <note> [--from <page>]` — argument is a one-line question or open issue parked off a wiki page, typically a `## Open Questions` bullet from a grill or architecture page.

The Triage workflow drops the note onto Hermes' [triage column](https://hermes-agent.nousresearch.com/docs/user-guide/features/kanban#columns) so it can be specified later via [P9 collaboration](https://hermes-agent.nousresearch.com/docs/user-guide/features/kanban#collaboration-patterns). The wiki is the source of truth for *answered* questions; triage is the durable inbox for *unanswered* ones.

**Preflight gate (Hermes).** `hermes kanban assignees` must succeed. Abort with the same install-hint message as `/wiki-kanban-emit` if Hermes is unavailable.

1. **Parse args.** Split into `<note>` and an optional `--from <page>` tail. Reject empty notes.
2. **Compose the task body** with the note under `## Question`, an optional `## Source` linking the originating page (`[[<page>]]`) and quoting its `## Open Questions` bullet, and a `@wiki-source: <page>` traceability tag.
3. **Create the triage task** via `hermes kanban create --triage --skill wiki-maintainer --skill kanban-worker --tenant <bounded-context-slug>`. Tenant derives from the source page's `frontmatter.context` if present; falls back to `triage`.
4. **Cross-link back.** If `--from <page>` was given, append `→ triage:<task-id>` to the source page's matching `## Open Questions` bullet (idempotent).
5. Append a dated entry to `wiki/log.md` recording the task id, the source page, the tenant, and the note.

After a clean run, the hint is: *"Question parked as triage task `<task-id>`. Run `/wiki-triage-promote <task-id>` when you're ready to expand it into a real spec."*

### Triage Promote

When the user says "triage-promote [task-id]" or runs `/wiki-triage-promote <task-id>` — argument is the id of a Hermes triage-column task previously created by `/wiki-triage`.

The Triage Promote workflow expands a triage one-liner into a real spec via Hermes' [`kanban specify`](https://hermes-agent.nousresearch.com/docs/user-guide/features/kanban#cli-reference) command (P9 collaboration pattern), then hands the structured output to `/wiki-spec`.

**Preflight gate (Hermes).** Same as `/wiki-triage`.

1. **Fetch the triage task** via `hermes kanban show <id> --json`; read its body, `@wiki-source` tag, tenant, and column. Refuse if not in `triage` — this command is column-scoped.
2. **Specify** via `hermes kanban specify <id>`. Capture the specifier's expanded body (problem statement, acceptance criteria, glossary).
3. **Derive a spec slug** from the title (kebab-case); confirm with the user.
4. **Hand off to `/wiki-spec`** with the specifier's output as the goal. All normal `/wiki-spec` gates apply — including the accepted-ADR preflight; the specifier output may not name an ADR, in which case `/wiki-spec` will offer to hand off to `/wiki-adr` first.
5. **Cross-link back.** Comment the triage task with the resulting spec path; move the triage task to `done` (or `cancelled` if the user abandons). On the originating wiki page, replace the bullet's `→ triage:<id>` tail with `→ spec:<slug>`.
6. Append a dated entry to `wiki/log.md` recording the triage id, resulting spec slug, and originating page.

If the specifier output cannot be turned into a coherent goal, stop; the triage task stays put. Do not invent a goal.

### Refine

When the user says "refine [run-id | breakthrough note]" or runs `/wiki-refine <argument>` — argument is either a Hermes run id whose completion surfaced new architectural information, or a free-form "breakthrough" note describing something that invalidates prior decisions.

The Refine workflow is the *structural* counterpart to Kanban Ingest. It closes the R∞ refinement loop: when a closed-out task or a breakthrough surfaces a new implicit concept, a new [[concepts/false-cognate]], a moved context boundary, or an invalidated decision, Refine updates the upstream model (concept pages, context maps, ADRs) and re-emits the affected kanban tasks under a fresh idempotency key. This is the named realization of [[concepts/evolving-order]] and [[concepts/breakthrough]] inside the multi-agent substrate.

1. **Classify the trigger.** Read the run id's structured handoff (or the user's breakthrough note). Decide:
   - **Documentary only** — the run surfaced no new structural information; what was learned is a concept-page-level fact. Hand off to `/wiki-kanban-ingest` instead and stop.
   - **Structural** — at least one of: an architectural commitment was invalidated; a new false cognate surfaced; a bounded-context boundary moved; a new concept emerged that deserves its own page. Continue with steps 2–6.
2. **Update upstream model.**
   - If a concept-page-level fact emerged, open or update the relevant `wiki/concepts/<term>.md` page (or create a new one if the term is genuinely new).
   - If a bounded-context boundary moved, update the relevant `wiki/contexts/<context>.md` page's `## Boundary` and add a `## Revisions`-style note dated entry.
   - If a new false cognate surfaced, update the `wiki/context-maps/<topic>.md` `## False Cognates` list and `## Translations` table.
3. **Open a superseding ADR if a decision was invalidated.** Hand off to `/wiki-adr` to open a new ADR that supersedes the prior one — do not edit the accepted predecessor. The new ADR's `## Context` must cite the originating run id (or breakthrough note) and the predecessor ADR's `## Status` must be flipped to `superseded by NNNN` with a link to the successor.
4. **Re-emit affected kanban tasks.** For every active kanban task whose spec page cites the now-superseded ADR, hand off to `/wiki-kanban-emit` with the originating spec slug. The fresh `<spec-slug>:<adr-id>:<sha256>` triple — with the *new* ADR id — produces a new task row rather than updating the old one. The old row should be closed with a `kanban_comment` pointing forward to the successor task.
5. **Verify zero dangling links — acceptance gate.** Same rule as Ingest, applied to every page touched.
6. Update `wiki/index.md` (touching the rows for every changed page) and append a dated entry to `wiki/log.md` recording the trigger (run id or note), the model changes, the new ADR id, the superseded ADR id, and the re-emitted task ids.

Refinement is the workflow that keeps the wiki and the board honest with each other when reality contradicts a prior model. Use it sparingly — most run outcomes are documentary, not structural — but use it without hesitation when an ADR is invalidated; an unrecorded supersession is worse than an explicit one.

Quality bar (acceptance):
- `mdbook build wiki/books/<slug>/` exits 0.
- Chapter list in `SUMMARY.md` matches the actual structure of the PDF (verify by spot-checking 2–3 chapter openings against the source).
- No invented chapters, sections, paragraphs, or figure captions.
- Chapter prose matches the printed page in reading order; no column interleaving, no in-figure labels spliced into the prose, no copyright/permissions notices bleeding into chapter 1.
- Math preserved as TeX, code as fenced blocks with a language hint where possible, tables as GFM where feasible, figures present where they appear in the source with captions quoted from the page.
- Front matter (preface, TOC, copyright) is either included as `[Front Matter](00-front-matter.md)` style entries before the first numbered part, or omitted deliberately and noted in the log.
- No `wiki/summaries/<slug>.md` was written, no concept/entity pages were created, and the only `wiki/` touchpoints outside `wiki/books/<slug>/` are the `Books` table row in `index.md` and the dated `log.md` entry.

## Rules

- Never modify files in `raw/`
- Always update `index.md` and `log.md` after any wiki change
- Prefer updating existing pages over creating duplicates
- When in doubt about a claim, set confidence to "low" and note the uncertainty
- Keep pages focused — one concept per page, split if a page gets too long
- Use plain English — define jargon on first use in each page
- All dates in ISO 8601 format: YYYY-MM-DD
- When a source provides specific examples, include them with concrete details
