---
title: "Associated Types"
type: concept
tags: [rust, traits, advanced, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/rust_book/_txt/20-chapter-19-advanced-features.txt"]
confidence: high
---

## Definition

Associated types are type placeholders declared inside a trait that an implementor fills in. They differ from generic parameters in that a given type can implement the trait *once*, fixing the associated type. The canonical example is `Iterator::Item`.

## How It Works

```rust
trait Iterator {
    type Item;
    fn next(&mut self) -> Option<Self::Item>;
}

impl Iterator for Counter {
    type Item = u32;
    fn next(&mut self) -> Option<u32> { /* ... */ }
}
```

By making `Item` an associated type rather than a generic parameter, calling code does not need to specify the type (`iter.next()` works without annotation). Compare with `trait Iterator<Item>` which would require `impl Iterator<u32> for Counter` and disambiguation everywhere.

## Key Parameters

- One implementation per (Trait, Type) pair
- Defaults: `type Item = Self;`
- Generic Associated Types (GATs): `type Item<'a> where Self: 'a;`
- Bounds: `type Item: Display;`

## When To Use

- When each implementor naturally has one related type
- When method signatures should not be cluttered with generic params
- Iterators, futures, allocators — the common stdlib pattern

## Risks & Pitfalls

- Cannot have multiple `impl Trait for Type` with different associated-type choices
- GAT syntax adds complexity and only became stable relatively recently
- Confusing for newcomers who reach for generic parameters first

## Related Concepts

- [[concepts/traits]]
- [[concepts/generics]]
- [[concepts/iterators]]

## Sources

- [[summaries/rust-book-20-chapter-19-advanced-features]]
