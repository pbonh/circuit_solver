# {{WIKI_NAME}} - LLM Wiki

This repository is an **LLM Wiki**: a persistent, interlinked markdown knowledge
base that an LLM agent builds and maintains. You (the agent) own everything under
`pages/`, plus `index.md` and `log.md`. The user owns `sources/`.

Created {{DATE}}.

## Read this first

The full, authoritative conventions are in **`.wiki/schema/SCHEMA.md`**. Read it
before editing. In short:

- Pages live in `pages/<type>/` with YAML frontmatter (`title`, `type`, `slug`,
  `updated`, `summary`, ...). `slug` must equal the filename.
- Cross-reference with Obsidian wikilinks: `[[slug]]`.
- `index.md` is generated; never hand-edit it.
- `log.md` is append-only with `## [YYYY-MM-DD] op | title` entries.

## The toolkit

Use `.wiki/bin/wikikit` for all bookkeeping (every command takes `--json`):

```bash
python3 .wiki/bin/wikikit search   .  "your query"   # find relevant pages
python3 .wiki/bin/wikikit index rebuild .             # regenerate index.md
python3 .wiki/bin/wikikit log add  .  --op ingest --title "..." --pages a,b
python3 .wiki/bin/wikikit validate .                  # enforce the schema
python3 .wiki/bin/wikikit lint     .                  # orphans, broken links, gaps
python3 .wiki/bin/wikikit stats    .                  # overview counts
```

(`.` = this wiki's root; adjust if you run from elsewhere.)

## Workflows

- **Ingest a source** -> use the `wiki-ingest` skill.
- **Answer a question** -> use the `wiki-query` skill.
- **Health-check** -> use the `wiki-lint` skill.

After any change to pages: rebuild the index, append a log entry, and run
`validate`. Co-evolve this file and `.wiki/schema/SCHEMA.md` as conventions for
this wiki's domain become clear.
