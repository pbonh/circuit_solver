---
title: "Deref Trait"
type: concept
tags: [rust, smart-pointers, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/16-chapter-15-smart-pointers.txt"]
confidence: high
---

## Definition

`std::ops::Deref` lets a type act like a reference: implementing it makes `*value` produce the wrapped inner value and enables *deref coercion* at function boundaries (`&String` → `&str`, `&Box<T>` → `&T`, etc.). The mutable counterpart is `DerefMut`.

## How It Works

```rust
impl<T> Deref for MyBox<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target { &self.0 }
}
```

The compiler inserts as many `deref()` calls as needed to make types match at call sites — so passing `&String` to a function expecting `&str` works. Method-call resolution walks the deref chain similarly.

## Key Parameters

- Associated type `Target`
- Pair with `DerefMut` for mutable deref
- Coercion rules: `&T` → `&U` if `T: Deref<Target = U>`

## When To Use

- Custom smart pointers (`Box`, `Rc`, `Arc`)
- Wrapper types that should be usable as the inner type
- Newtype wrappers that want frictionless API ergonomics

## Risks & Pitfalls

- Auto-deref can produce surprising overload resolution
- Leaking inner API through `Deref` defeats the encapsulation that motivated a newtype
- `Deref` is technically intended for smart pointers; using it for arbitrary type conversion is an anti-pattern
- Excessive deref chains slow down inference

## Related Concepts

- [[concepts/smart-pointers]]
- [[concepts/box-type]]
- [[concepts/newtype-pattern]]
- [[concepts/traits]]

## Sources

- [[summaries/rust-book-16-chapter-15-smart-pointers]]
