---
title: Guide to Graph Algorithms — Graphs (Chapter 1)
type: source
id: summaries/guide-to-graph-algorithms-04-graphs
kind: publication
tags:
- graph
- foundational
- well-established
- netlist
created: 2026-05-15
updated: 2026-05-15
sources:
- raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
---

## Key Points

- A graph G = (V, E) is an ordered pair of finite sets, where V is a nonempty set of vertices and E is a set of unordered pairs of vertices called edges. All graphs in the book are finite.
- Two graphs are isomorphic if there is an edge-preserving bijection between their vertex sets; "being isomorphic" is an equivalence relation.
- Graphs can be represented by an adjacency matrix (symmetric 0/1 matrix) or by adjacency lists, and concepts like neighborhood N(x), closed neighborhood N[x], degree d(x), and regularity are defined in terms of edges.
- Connectedness is defined via partitions (a graph is connected if every bipartition of V has a crossing edge); subgraphs and induced subgraphs G[W] are introduced, and the special case of spanning subgraphs / spanning trees is discussed.
- Paths and cycles are defined as ordered vertex sequences; distance d(x,y) is the minimum number of edges on a path between x and y, and the path Pn and cycle Cn notations are established.
- The complement Ḡ flips edges and nonedges; cliques (sets of pairwise adjacent vertices, ω(G) is max clique size) and independent sets (α(G) is max independent set size) are dual under complementation: ω(G) = α(Ḡ).
- A component of a graph is a maximal vertex set inducing a connected subgraph; the chapter presents Rem's algorithm (Algorithm 1) for component computation in O(n^2) time using a representative function δ updated as edges are added.
- A separator S ⊂ V is a vertex set whose removal disconnects G; minimal a|b-separators and the special case of cutvertices are formalized.
- Trees are connected acyclic graphs; equivalent characterizations include "every minimal separator has cardinality 1" and "every connected induced subgraph with at least two vertices has a vertex of degree one (leaf/pendant)". Trees admit elimination orders that successively prune leaves. Forests are graphs whose components are trees.
- A graph is bipartite if it has a 2-coloring (vertex partition {A, B} with all edges crossing); equivalently χ(G) ≤ 2 or all cycles in G are even (Theorem 1.19). The chromatic number χ(G) is the minimum number of colors needed to properly color the vertices.
- Linegraphs L(G) have edges of G as vertices and adjacency by shared endpoint; linegraphs are claw-free. For any fixed H, "H-free" graphs are those without H as an induced subgraph.
- Notational conventions: n = |V|, m = |E|, ω = ω(G), α = α(G); sets are sometimes abused to denote the induced subgraphs they generate.

## Relevant Concepts

- [[concepts/graph]] — the basic structure (V, E) underlying the chapter
- [[concepts/adjacency-matrix]] — symmetric 0/1 matrix representation of a graph
- [[concepts/neighborhood]] — open and closed neighborhoods of vertices
- [[concepts/connectedness]] — defined via partitions; basis for components
- [[concepts/induced-subgraph]] — G[W] for W ⊆ V
- [[concepts/path]] — ordered vertex sequence with consecutive adjacencies
- [[concepts/cycle]] — closed path of length at least 3
- [[concepts/complement]] — Ḡ, swaps edges and nonedges
- [[concepts/component]] — maximal connected vertex set
- [[concepts/rems-algorithm]] — incremental component-labeling algorithm (Algorithm 1)
- [[concepts/separator]] — vertex set whose removal disconnects the graph
- [[concepts/minimal-separator]] — separator with no proper subset that separates the same pair
- [[concepts/tree]] — connected acyclic graph
- [[concepts/forest]] — graph whose components are trees
- [[concepts/spanning-tree]] — spanning subgraph that is a tree
- [[concepts/bipartite-graph]] — 2-colorable graph, no odd cycles
- [[concepts/clique]] — pairwise-adjacent vertex set
- [[concepts/independent-set]] — pairwise-nonadjacent vertex set
- [[concepts/chromatic-number]] — χ(G), minimum proper coloring
- [[concepts/linegraph]] — graph with edges of G as vertices

## Source Metadata

- Source type: book chapter
- Book title: A Guide to Graph Algorithms
- Chapter 1: Graphs
- File path: raw/GuideToGraphAlgorithms/_txt/04-graphs.txt
- Authors: Ton Kloks, Mingyu Xiao
