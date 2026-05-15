---
title: "RefCell Type"
type: concept
tags: [rust, smart-pointers, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Definition

`std::cell::RefCell<T>` provides single-threaded interior mutability: you may mutate the inner `T` through a shared `&RefCell<T>` reference. The borrow rules normally enforced at compile time are deferred to runtime checks that panic on violation.

## How It Works

`.borrow()` returns a `Ref<T>` (acts like `&T`) that increments a borrow counter; `.borrow_mut()` returns a `RefMut<T>` (acts like `&mut T`) and verifies no other borrow is live. Dropping the guard decrements the count. Borrowing-rule violations — say, calling `borrow_mut()` while a `Ref<T>` is still alive — panic at runtime. `RefCell` does not implement `Sync`, so it cannot cross thread boundaries; the threaded equivalents are `Mutex<T>` and `RwLock<T>`.

## Key Parameters

- `.borrow()` / `.borrow_mut()`
- Runtime borrow counters
- Panic on overlapping borrows
- Single-thread only

## When To Use

- Interior mutability for fields inside otherwise-immutable structures
- Mock objects in tests where the trait under test takes `&self`
- Self-referential data via `Rc<RefCell<T>>` (with care)
- Caching computed values lazily

## Risks & Pitfalls

- Borrow violations panic at runtime — fragile if access patterns are complex
- Easy to leak borrows accidentally with long-lived guards
- `Rc<RefCell<T>>` cycles still leak; couple with `Weak<T>`
- Mixing `RefCell<T>` and threading is a compile error (good!) but tempting

## Related Concepts

- [[concepts/smart-pointers]]
- [[concepts/rc-type]]
- [[concepts/interior-mutability]]
- [[concepts/mutex]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
