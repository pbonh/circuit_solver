---
title: "The Rust Programming Language — Chapter 15: Smart Pointers"
type: summary
tags: [rust, smart-pointers, advanced, ownership]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Key Points

- Smart pointers are types that act like references but also own data and add capabilities (heap allocation, reference counting, interior mutability, etc.). They typically implement the `Deref` and `Drop` traits.
- `Box<T>` is the simplest smart pointer: a single owner of a heap-allocated `T`. Used for transferring ownership of large data, recursive types with unknown compile-time size, and trait objects (`Box<dyn Trait>`).
- The `Deref` trait makes a smart pointer behave like a reference: `*box_t` calls `box_t.deref()` and works transparently with auto-deref coercion at call boundaries.
- `Drop` runs custom cleanup code automatically when a value goes out of scope. Manual early drop is `std::mem::drop(v)`, not `v.drop()`.
- `Rc<T>` (reference-counted) enables multiple owners of immutable data within a single thread. `Rc::clone(&rc)` increments the count cheaply (no deep copy). Data is dropped when the count reaches zero.
- `RefCell<T>` provides interior mutability: borrows are checked at runtime instead of compile time. `.borrow()` and `.borrow_mut()` return guards that enforce aliasing-XOR-mutability dynamically; violations panic.
- Common pattern: `Rc<RefCell<T>>` — multiple owners of mutable data. The chapter walks through this with a graph-like structure.
- `Cell<T>` is a sibling of `RefCell<T>` that uses `Copy` semantics (get/set) rather than borrowing — useful for small `Copy` payloads.
- Reference cycles via `Rc<RefCell<T>>` leak memory because the reference count never reaches zero. The solution is `Weak<T>`, a non-owning, observation-only reference whose `upgrade()` returns `Option<Rc<T>>`.
- Comparison table at chapter end: `Box<T>` = single owner, compile-time borrow check; `Rc<T>` = multiple owners, immutable, compile-time check; `RefCell<T>` = single owner, mutable, runtime check.
- All three are single-threaded primitives. Threaded equivalents are `Arc<T>` and `Mutex<T>` / `RwLock<T>` (covered in chapter 16).

## Relevant Concepts

- [[concepts/smart-pointers]] — the family of owning pointer types.
- [[concepts/box-type]] — `Box<T>`, simplest heap pointer.
- [[concepts/rc-type]] — single-thread reference counting.
- [[concepts/refcell-type]] — single-thread interior mutability.
- [[concepts/weak-references]] — `Weak<T>` cycle-breakers.
- [[concepts/deref-trait]] — `Deref` and auto-deref coercion.
- [[concepts/drop-trait]] — automatic cleanup.
- [[concepts/interior-mutability]] — mutating via `&T` through runtime check.

## Source Metadata

- Source type: book chapter
- Book title: The Rust Programming Language
- Chapter: 15 — Smart Pointers
- File path: `raw/rust_book/_txt/16-chapter-15-smart-pointers.txt`
- Authors: Steve Klabnik and Carol Nichols
