---
title: "Example Flashcards"
type: flashcards
tags: [flashcards, meta]
created: 2026-04-08
updated: 2026-04-08
sources: ["wiki/index.md"]
confidence: high
---

# Example Flashcards

Demonstrates the three card formats consumed by the [obsidian-spaced-repetition](https://github.com/st3v3nmw/obsidian-spaced-repetition) plugin. The `flashcards` tag in the frontmatter above is what the plugin scans for — every file under `wiki/flashcards/` must carry it.

Each card is separated from the next by a blank line. The `?`/`??` lives on its own line between question and answer.

## Source

[[wiki/index.md]]

## Cards

What three card formats does this wiki support?
?
**Basic** (`Q\n?\nA`) for one-way recall, **reversed** (`Q\n??\nA`) for symmetric term ↔ definition pairs, and **cloze** (`==hidden==`) for fill-in-the-blank inside a sentence.

Obsidian Spaced Repetition plugin
??
A community plugin that scans markdown files tagged `#flashcards` and schedules them for spaced-repetition review.

The Flashcards workflow lives in ==`AGENTS.md`== — slash commands and the wiki-maintainer skill delegate to it rather than restating the rules.
