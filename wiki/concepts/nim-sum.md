---
title: Nim-Sum
type: claim
id: concepts/nim-sum
tags:
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/05-algorithms.txt
confidence:
  base: 0.95
  source_count: 1
  contradicted: false
  effective: 0.95
  inputs_hash: 8331cbe4e16ebf56
---

## Definition

The nim-sum a ⊕ b of two non-negative integers is the bit-wise XOR of their binary representations. Equivalently, a ⊕ b = mex{k : k ≠ a' ⊕ b for some a' < a OR k ≠ a ⊕ b' for some b' < b}.

## How It Works

Nim with piles n_1, …, n_k has Grundy value n_1 ⊕ n_2 ⊕ … ⊕ n_k. Player 1 wins iff this XOR is nonzero. Strategy: find a pile whose top bit equals the top bit of the XOR; reduce that pile to make the XOR zero.

Nim-multiplication (Lenstra 1978) defines a product n ∘ m turning (ℕ, ⊕, ∘) into a field of characteristic 2. Applications: rectangular coin-turning games.

## Key Parameters

- O(log max_i n_i) bit-operations per XOR.
- The nim field is used in combinatorial game theory and certain cryptographic protocols.

## When To Use

- Combining independent impartial games via Sprague-Grundy.
- Coin-turning, take-away, and many simple combinatorial games.

## Risks & Pitfalls

- Nim-sum is XOR, not arithmetic addition.
- For partizan games (different moves for each player), Grundy theory does not apply.

## Related Concepts

- [[concepts/grundy-value]]

## Sources

- [[summaries/guide-to-graph-algorithms-05-algorithms]]
