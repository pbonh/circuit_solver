---
title: Rc Type
type: claim
id: claim-rc-type
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
  base: 0.85
---

## Definition

`std::rc::Rc<T>` is a single-threaded, reference-counted, shared-ownership smart pointer. Cloning an `Rc` does not copy the inner data — only the reference count is incremented. The value is dropped when the last `Rc` referencing it goes out of scope.

## How It Works

`Rc::new(value)` allocates a header (containing strong and weak counts) plus the value on the heap and returns the first strong reference. `Rc::clone(&rc)` (or `rc.clone()`) bumps the strong count. Dropping decrements; reaching zero deallocates. The data inside an `Rc<T>` is *immutable* by default (`Rc<T>` gives only `&T` access); combine with `RefCell<T>` for shared mutable state. Thread-safe equivalent is `Arc<T>`.

## Key Parameters

- Strong count and weak count
- `Rc::clone` (cheap reference-count bump)
- `Rc::strong_count`, `Rc::weak_count`
- Single-thread only — does not implement `Send` or `Sync`

## When To Use

- Many independent owners need immutable access to the same data
- Graph/tree structures where children may be referenced from several parents
- Avoiding deep cloning of expensive data in a single thread

## Risks & Pitfalls

- Cycles leak memory — break with `Weak<T>`
- Single-threaded; switching to `Arc<T>` requires a conscious rewrite
- Cloning `Rc` is not free in tight loops (one atomic-free integer bump, but still cache traffic)
- Sharing through `Rc<RefCell<T>>` can deadlock-equivalent (panic) at runtime

## Related Concepts

- [[concepts/smart-pointers]]
- [[concepts/refcell-type]]
- [[concepts/weak-references]]
- [[concepts/arc-type]]
- [[concepts/interior-mutability]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
