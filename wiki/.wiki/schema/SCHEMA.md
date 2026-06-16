# LLM Wiki Schema

This document defines how an LLM Wiki is structured and the conventions every
page must follow. It is the contract that the `wikikit` toolkit validates and
that the ingest / query / lint skills rely on. A copy lives at
`.wiki/schema/SCHEMA.md` inside every initialized wiki and is refreshed by
`wikikit init --upgrade`.

## 1. Layers

The wiki has three layers (per the LLM Wiki pattern):

1. **Raw sources** (`sources/`) - immutable inputs you curate. The LLM reads
   them but never edits them. Your source of truth.
2. **The wiki** (`pages/`, `index.md`, `log.md`) - LLM-generated, interlinked
   markdown. The LLM owns this layer entirely.
3. **The schema** (`AGENTS.md` + this file) - the conventions and workflows that
   make the LLM a disciplined maintainer rather than a generic chatbot.

## 2. Directory layout

```
<wiki>/
├── AGENTS.md              # per-wiki instructions (auto-read by the agent)
├── index.md              # generated catalog of all pages (wikikit index rebuild)
├── log.md                # append-only chronological operation log
├── sources/              # raw, immutable source files (user-owned)
├── pages/
│   ├── overview.md        # the single overview/synthesis page (type: overview)
│   ├── entities/          # type: entity
│   ├── concepts/          # type: concept
│   ├── topics/            # type: topic
│   ├── analyses/          # type: analysis (filed-back query answers)
│   └── sources/           # type: source (one summary page per raw source)
└── .wiki/                # toolkit (not part of the knowledge content)
    ├── bin/wikikit        # the maintenance CLI
    ├── schema/            # this file + frontmatter.schema.json
    └── templates/         # page templates
```

The `.wiki/` directory is tooling, not knowledge. It begins with a dot so
Obsidian and most tools ignore it.

## 3. Page types

| `type`     | Directory          | Purpose                                              |
|------------|--------------------|------------------------------------------------------|
| `overview` | `pages/overview.md`| Top-level synthesis / entry point. Exactly one.      |
| `entity`   | `pages/entities/`  | A person, org, place, product, or named thing.       |
| `concept`  | `pages/concepts/`  | An idea, theory, method, or term.                    |
| `topic`    | `pages/topics/`    | A broader theme that ties entities/concepts together.|
| `source`   | `pages/sources/`   | A summary of one raw source file.                    |
| `analysis` | `pages/analyses/`  | A filed-back answer to a query (comparison, finding). |

## 4. Frontmatter

Every page begins with YAML frontmatter. The machine-readable contract is
`frontmatter.schema.json`; `wikikit validate` enforces it.

```yaml
---
title: Ada Lovelace            # required
type: entity                   # required; one of the types above
slug: ada-lovelace             # required; kebab-case; MUST equal the filename
created: 2026-06-15            # YYYY-MM-DD
updated: 2026-06-15            # required; bump on every edit
summary: First computer programmer.   # one line; shown in index.md
tags: [history, computing]
sources: [memoir-1843]         # provenance: source page slugs backing this page
status: active                 # active | stale | draft
---
```

For `type: source` pages also set `source_file:` to the path (relative to
`sources/`) of the raw file being summarized. `wikikit lint` reports raw sources
that have no covering source page.

Rules enforced by `wikikit validate`:

- `slug` must equal the filename without `.md`.
- `type` must be one of the valid types.
- required fields (`title`, `type`, `slug`, `updated`) must be present.
- dates must be `YYYY-MM-DD`; `status` must be a valid value.
- missing `summary` is a warning (index entries lose their description).

## 5. Cross-references (links)

Use **Obsidian wikilinks**: `[[slug]]` or `[[slug|display text]]`. Always link by
slug, never by file path. Examples:

```markdown
[[ada-lovelace]] collaborated with [[charles-babbage]] on the
[[analytical-engine|Analytical Engine]].
```

`wikikit lint` resolves every `[[slug]]` against the pages on disk:

- a link to a non-existent slug is a **broken link**.
- a page with zero inbound links (except `overview`) is an **orphan**.

Cross-referencing is the whole point: a fact is only useful when it is connected.

## 6. index.md

Content-oriented catalog, **generated** by `wikikit index rebuild` from page
frontmatter. Grouped by type, each entry is `- [[slug]] - summary`. Never edit it
by hand; rebuild it after adding or renaming pages. `wikikit index check`
reports drift between `index.md` and the pages on disk.

## 7. log.md

Chronological, append-only. Each entry starts with a grep-able prefix so the log
is parseable with plain unix tools:

```
## [2026-06-15] ingest | Ada Lovelace's 1843 notes
- summary: added entity + concept pages, updated overview
- sources: 1
- pages: ada-lovelace, analytical-engine, overview
```

`grep "^## \[" log.md | tail -5` shows recent activity. Append entries with
`wikikit log add`; read them back with `wikikit log tail`.

## 8. Operations (workflows)

- **Ingest** - read a raw source, write a `source` page, create/update the
  `entity` / `concept` / `topic` pages it touches (with `[[links]]`), rebuild the
  index, append an `ingest` log entry, then `validate`.
- **Query** - `wikikit search` for relevant pages, read them, synthesize a cited
  answer. Good answers are filed back as an `analysis` page so explorations
  compound.
- **Lint** - `wikikit lint` + `validate` to surface contradictions, orphans,
  broken links, stale claims, and gaps; fix them; append a `lint` log entry.

## 9. The toolkit (`wikikit`)

`.wiki/bin/wikikit` provides observable, testable operations. All commands accept
`--json`.

| Command | What it does |
|---------|--------------|
| `init <dir> [--upgrade]` | scaffold a wiki / refresh the toolkit |
| `search <wiki> "q" [-k N]` | BM25 keyword search over pages |
| `index rebuild\|check <wiki>` | regenerate / verify `index.md` |
| `log add\|tail <wiki> ...` | append / read log entries |
| `validate <wiki>` | enforce this schema; exit 1 on errors |
| `lint <wiki> [--strict]` | health checks; exit 1 on issues with `--strict` |
| `stats <wiki>` | counts by type, links, orphans, activity |
| `selftest` | end-to-end smoke test in a throwaway wiki |
