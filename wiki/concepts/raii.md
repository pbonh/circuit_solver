---
title: "RAII (Resource Acquisition Is Initialization)"
type: concept
tags: [rust, c-plus-plus, memory-management, idiom, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: medium
---

## Definition

Resource Acquisition Is Initialization (RAII) is the idiom — popularised in C++ and adopted as the default discipline in Rust — that binds the lifetime of an owned resource (heap memory, file handle, mutex guard, network connection) to the lifetime of an object on the stack. When that object goes out of scope its destructor (in Rust, its `Drop` impl) releases the resource. The Rust Book Chapter 15 describes Rust's implementation: "Drop ... lets you customize what happens when a value is about to go out of scope. You can provide an implementation for the Drop trait on any type, and that code can be used to release resources like files or network connections."

## How It Works

In Rust, RAII is implemented through the [[concepts/drop-trait]]. Per the Rust Book: "Smart pointers are usually implemented using structs. Unlike an ordinary struct, smart pointers implement the Deref and Drop traits. ... The Drop trait allows you to customize the code that's run when an instance of the smart pointer goes out of scope." Standard smart pointers (`Box<T>`, `Rc<T>`, `Arc<T>`) deallocate on drop; `MutexGuard` releases the lock; `File` closes the descriptor.

## Key Parameters

- Owner type's `Drop::drop` body — invoked exactly once when the value is dropped.
- Scope-bounded ownership — ownership rules guarantee at most one owner per resource.
- Move semantics — transferring ownership transfers the destruction obligation.

## When To Use

- Any time a resource must be released deterministically: heap allocations, lock guards, file/socket handles, GPU buffers, transactions.
- Wrapping FFI resources (raw pointers, OS handles) in Rust structs that implement Drop.

## Risks & Pitfalls

- `std::mem::forget` and `Box::leak` bypass Drop — useful but easy to misuse.
- Cycles among `Rc<T>` prevent Drop from running — break with `Weak<T>`.
- `drop(...)` cannot be called recursively or panic-safely from another Drop.

## Related Concepts

- [[concepts/drop-trait]] — the Rust mechanism implementing RAII.
- [[concepts/ownership]]
- [[concepts/smart-pointers]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
