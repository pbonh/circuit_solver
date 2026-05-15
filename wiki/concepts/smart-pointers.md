---
title: "Smart Pointers"
type: concept
tags: [rust, ownership, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Definition

A smart pointer in Rust is a type that behaves like a pointer (auto-deref to inner data) but owns the pointed-to value and adds extra capability — heap allocation (`Box<T>`), reference counting (`Rc<T>`, `Arc<T>`), interior mutability (`RefCell<T>`, `Cell<T>`, `Mutex<T>`), or weak references (`Weak<T>`). Smart pointers typically implement `Deref` and `Drop`.

## How It Works

`Deref` gives smart pointers method-call ergonomics like references; `Drop` automates cleanup at scope end. Each smart pointer enforces a specific ownership/sharing policy: `Box<T>` is unique ownership on the heap; `Rc<T>` shares ownership via reference counting; `RefCell<T>` provides runtime-checked aliasing for interior mutability. Combinations (`Rc<RefCell<T>>`, `Arc<Mutex<T>>`) compose these capabilities.

## Key Parameters

- Ownership semantics: unique vs shared (`Rc`, `Arc`)
- Mutability: compile-time vs runtime check
- Thread safety: `Rc`/`RefCell` single-thread; `Arc`/`Mutex` multi-thread
- Allocation cost: `Box` once; `Rc`/`Arc` add count overhead

## When To Use

- `Box<T>`: heap-allocate large data, recursive types, trait objects
- `Rc<T>`: many readers, single-thread, immutable data
- `RefCell<T>`: interior mutability through `&T`, single-thread
- `Arc<T>` / `Mutex<T>`: same patterns across threads
- `Weak<T>`: cycle-breaking back-references

## Risks & Pitfalls

- `Rc<RefCell<T>>` cycles leak memory — break with `Weak<T>`
- `RefCell::borrow_mut` panics if a borrow is already alive — runtime fragility
- Overusing `Rc`/`Arc` instead of references hides ownership intent
- Mixing `Rc` and `Arc` requires conversion; they are not interchangeable

## Related Concepts

- [[concepts/box-type]]
- [[concepts/rc-type]]
- [[concepts/refcell-type]]
- [[concepts/weak-references]]
- [[concepts/deref-trait]]
- [[concepts/drop-trait]]
- [[concepts/interior-mutability]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
