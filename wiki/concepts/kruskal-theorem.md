---
title: "Kruskal's Tree Theorem"
type: concept
tags: [algorithm, foundational, well-established]
created: 2026-05-15
updated: 2026-05-15
sources: ["raw/GuideToGraphAlgorithms/_txt/07-recent-trends.txt"]
confidence: high
---

## Definition

Kruskal's tree theorem (1960): the set of rooted trees with vertex labels from a finite set is well-quasi-ordered under the embedding relation T_i ≤ T_j: there is an injective vertex map preserving root, labels, and common-ancestor structure.

(Not to be confused with Kruskal's MST algorithm.)

## How It Works

Nash-Williams' proof generalizes the Higman bad-sequence argument: pick a minimal bad sequence, find a "child-subsequence" via the pigeonhole on root labels and recursive structure, contradiction.

Extensions:
- The set of labels can itself be a well-quasi-order (then the embedding additionally requires label-≤).
- Gap embeddings (Theorem 4.101): edge labels from a totally ordered set allow gap-≤ comparisons; the result still gives wqo.

Applications: k-cograph wqo, threshold-width wqo, immersions in digraphs (Liu-Muzi).

## Key Parameters

- Number of labels.
- Edge / vertex / gap labeling structure.

## When To Use

- Proving finite obstruction sets for tree-structured graph classes.
- Forward Ramsey / Colcombet-style decompositions in proofs of Bonamy-Pilipczuk.

## Risks & Pitfalls

- Non-constructive: gives no bound on the embedding indices.
- Conditions on label structure must be checked carefully.

## Related Concepts

- [[concepts/well-quasi-order]]
- [[concepts/higmans-lemma]]
- [[concepts/graph-minor-theorem]]
- [[concepts/k-cograph]]
- [[concepts/threshold-width]]

## Sources

- [[summaries/guide-to-graph-algorithms-07-recent-trends]]
