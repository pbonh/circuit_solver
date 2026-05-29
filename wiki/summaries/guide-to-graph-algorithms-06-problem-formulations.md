---
title: Guide to Graph Algorithms — Problem Formulations (Chapter 3)
type: source
id: source-guide-to-graph-algorithms-06-problem-formulations
kind: derived-summary
tags:
- graph
- algorithm
- foundational
- well-established
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
---

## Key Points

- Graph algebras describe many graph classes via "gluing" (identifying vertices, e.g. chordal graphs) or "bridging" (adding edges between subsets, e.g. trees, cographs, distance-hereditary graphs) operations.
- Monadic second-order logic (MSO) is a logical language that can express graph properties; for example, connectedness is expressed by quantifying over a partition {A, B} of V and demanding that some edge crosses.
- A sentence consists of logical symbols (parentheses, connectives ⇒, ∧, ∨, ¬, variables) and parameters (quantifiers ∀, ∃, equality, constants, and predicate symbols of arbitrary arity).
- "Monadic" means quantification ranges over elements and subsets but not over arbitrary relations; sentences are well-formed (parenthesized into atomic parts).
- MS1 allows quantification over vertices and vertex subsets; MS2 additionally allows quantification over edges and edge subsets. Hamiltonian cycle is expressible in MS2.
- Courcelle's theorems: MS1 sentences can be evaluated in O(n^3) time on graphs of bounded rankwidth (FPT in rankwidth); MS2 sentences can be evaluated in linear time on graphs of bounded treewidth (FPT in treewidth).
- The language MS2 is strictly more powerful than MS1; thus the class of graphs on which MS2 problems can be efficiently solved is contained in the class on which MS1 problems can be.

## Relevant Concepts

- [[concepts/monadic-second-order-logic]] — the formal language MS1 / MS2 used to specify graph problems
- [[concepts/graph-algebra]] — gluing and bridging operations that generate graph classes
- [[concepts/courcelle-theorem]] — FPT meta-theorem: MS2 in linear time on bounded treewidth
- [[concepts/treewidth]] — parameter that controls MS2 evaluation cost
- [[concepts/rankwidth]] — parameter that controls MS1 evaluation cost
- [[concepts/distance-hereditary-graph]] — class with bounded rankwidth
- [[concepts/cograph]] — example of bridging-defined class
- [[concepts/chordal-graph]] — example of gluing-defined class

## Source Metadata

- Source type: book chapter
- Book title: A Guide to Graph Algorithms
- Chapter 3: Problem Formulations
- File path: raw/GuideToGraphAlgorithms/_txt/06-problem-formulations.txt
- Authors: Ton Kloks, Mingyu Xiao
