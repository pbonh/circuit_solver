---
title: "A Guide to Graph Algorithms"
type: source
slug: guide-to-graph-algorithms
created: 2026-06-16
updated: 2026-06-16
summary: Graduate-level survey of graph theory and algorithms — fundamentals, clique/matching/domination algorithms, MSO logic, treewidth, modular decomposition, graph minors, and parameterized complexity.
source_file: Books/GuideToGraphAlgorithms
tags: [graph-algorithms, treewidth, graph-theory, clique, matching, parameterized-complexity, planar-graphs]
status: active
---

# A Guide to Graph Algorithms

- **Source file:** `sources/Books/GuideToGraphAlgorithms/`
- **Author / origin:** [University summer school notes, USTC; first draft 2016-2017, updated thereafter]
- **Date:** ~2018-2020

## Summary

A graduate-level research introduction to graph algorithms, organized around foundational concepts, classical algorithmic highlights, formal problem formulations, and recent structural trends. Uses treewidth as a unifying thread for the "Recent Trends" section.

### Part 1: Graphs

Core graph theory: isomorphism, adjacency list/matrix representation, neighborhoods, connectivity (connected components via Rem's algorithm — path-compression union-find), separators, trees, bipartite graphs, line graphs, cliques, independent sets. Standard graph-theoretic notation.

### Part 2: Algorithms

Key algorithmic results:
- **Finding and counting small induced subgraphs**: motif counting via color-coding, algebraic methods
- **Bron & Kerbosch algorithm**: maximal clique enumeration; timebound analysis (O(3^{n/3}) cliques in n-vertex graph)
- **Total order / hypergraphs**: problem reductions; using total ordering to speed up combinatorial algorithms; hypergraph generalizations
- **NP-completeness**: reductions; graph problems that are NP-hard; splitgraph equivalence covers
- **Lovász Local Lemma (LLL)**: probabilistic method for combinatorial existence proofs; Moser-Tardos constructive algorithm (makes LLL algorithmic); applications to dominating sets and graph coloring
- **Szemerédi's Regularity Lemma**: dense graphs can be partitioned into a bounded number of quasi-random bipartite pairs; construction of regular partitions
- **Clique separators**: perfect elimination orderings; chordal graphs; separation into smaller clique-separated pieces; polynomial dynamic programming
- **Parameterized algorithms**: bounded search tree; vertex cover (FPT); feedback vertex set; edge dominating set; fixed-parameter tractable algorithms
- **Matching**: Blossom algorithm (Edmonds) for maximum matching in general graphs; Minty's algorithm for independent set in claw-free graphs
- **Graph games**: Nim, Grundy values, De Bruijn sequences, Chomp

### Part 3: Problem Formulations

**Graph algebras**: algebraic operations on graphs (disjoint union, join, product); closed under specific graph operations defines graph classes. **Monadic Second-Order Logic (MSO)**: express graph properties in MSO; Courcelle's theorem: every MSO-expressible property is solvable in linear time on graphs of bounded treewidth.

### Part 4: Recent Trends

**Treewidth**: the central structural parameter. A tree-decomposition of width k partitions the graph into bags connected in a tree, where each bag has ≤ k+1 vertices and every edge and triangle is covered. Treewidth measures "how tree-like" a graph is. Key results:
- Chordal graphs: perfect elimination orderings, clique-trees, treewidth characterization
- Treewidth and brambles: dual obstruction (a bramble of order k ↔ treewidth ≥ k-1)
- Dynamic programming on tree-decompositions: Steiner tree, vertex cover, coloring in linear time FPT in treewidth
- Treewidth of planar graphs: planar graph treewidth = O(sqrt(n))
- Modular decomposition: canonical P4-free partition of graphs; linear-time algorithm; useful for cograph recognition and tree-decompositions of cographs
- Rankwidth (Oum-Seymour): less restrictive than treewidth; better for dense graphs; recognizes distance-hereditary graphs; χ-bounded classes
- Well-quasi-orders: Kruskal's theorem for trees; graph minor theorem (Robertson-Seymour); implies finite obstruction sets for every minor-closed graph family
- Graph minors: Robertson-Seymour theorem; minor-monotone properties; implications for circuit layout (genus, crossing number)

## Key takeaways

- Treewidth is the key parameter enabling efficient exact algorithms on "almost-tree-like" graphs — VLSI netlist graphs often have bounded treewidth due to hierarchical structure
- Bron-Kerbosch maximal clique enumeration runs in O(3^{n/3}) — practical for moderate-sized netlists
- Courcelle's theorem: MSO-expressible problems (reachability, acyclicity, matching) can be solved in linear time on bounded-treewidth graphs
- Parameterized algorithms (FPT) make NP-hard problems tractable when a parameter (treewidth, vertex cover size) is small — VLSI graphs often have small separators
- The graph minor theorem guarantees finite obstruction sets for every minor-closed property — including planarity (Kuratowski), genus, and VLSI layout constraints

## Pages updated from this source

- [[graph-algorithms]] - extended with treewidth, parameterized complexity, Lovász Local Lemma
- [[treewidth-and-graph-structure]] - concept created
