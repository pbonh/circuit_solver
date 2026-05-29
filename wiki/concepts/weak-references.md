---
title: Weak References
type: claim
id: concepts/weak-references
tags:
- rust
- smart-pointers
- advanced
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/rust_book/_txt/16-chapter-15-smart-pointers.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

`Weak<T>` is a non-owning companion to `Rc<T>` (or `Arc<T>` for the threaded form). It tracks the existence of a value without keeping it alive. Weak references are the standard way to break reference cycles that would otherwise leak memory.

## How It Works

`Rc::downgrade(&rc)` produces a `Weak<T>`. `Weak::upgrade()` returns `Option<Rc<T>>`: `Some(rc)` if the value is still alive, `None` if the last strong reference has been dropped. The header carries both a strong and a weak count; only strong references decide when to drop the data, but the header itself outlives until all weaks are gone too.

## Key Parameters

- `Weak::new` (creates a never-upgradable weak)
- `Rc::downgrade(&rc)` from an existing strong reference
- `Weak::upgrade()` returning `Option<Rc<T>>`
- Strong vs weak counts in the heap header

## When To Use

- Parent-pointer in a tree where children own siblings
- Observer relationships that must not extend the observed lifetime
- Caches that should not prevent the underlying value from being freed
- Breaking cycles in graph structures

## Risks & Pitfalls

- `upgrade()` may return `None` unexpectedly if you forgot a strong owner
- Weak references still keep the heap header alive — memory overhead persists
- Easy to design oneself into ABA-style errors when juggling strong/weak counts

## Related Concepts

- [[concepts/rc-type]]
- [[concepts/arc-type]]
- [[concepts/smart-pointers]]
- [[concepts/refcell-type]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
